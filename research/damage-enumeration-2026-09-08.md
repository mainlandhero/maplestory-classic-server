# The writer damages a handful of objects per session, not one and not dozens - and the pool we have been counting is under a third of the pool

Every previous pass looked at **the one object that killed the run**. This one enumerates. A new
tool, `tools/damagescan.py`, sweeps a full-memory dump with four independent detectors and was
run over all **37 dumps in `dumps/`** - 16 distinct processes, 11 s to 7015 s of process life.
`tools/damagescan_all.py` drives it and keeps the per-dump text and JSON under
`research/damagescan/`.

Tags: **[L]** measured off a dump, **[D]** derived, **[I]** inferred.

---

## 0. The instrument finding that comes first, because it changes a denominator

**`tools/poolchain.py` follows one chunk list per bucket. There are 819 chunk lists.** [L]

`poolchain.py` and `crates/grap-stub/src/poolsentry.rs` both start from the chunk-list head at
`0x143AD68A0 + 0x88` and follow it. That is an enumeration of **one list**, not of the pool. The
`0x40` victim `0x3b69a4a8` from `research/the-writer-damages-live-objects-2026-09-08.md` - the
vtable-plus-2 object - **is not on that list**, and the first version of this tool duly reported
that it did not exist. Its positive control failed, which is the only reason this was found.

So the tool stopped following the list and enumerated the **shape**: the big allocator writes
`count*(slot+8)+8` as an 8-byte header at `chunkbase-8`, and the carve loop then writes the
bucket's slot size into `count` headers at an exact stride. Requiring all but two of those to be
exactly right is 6 to 62 identical qwords at a fixed stride, which does not happen by accident.
In the 372984 dump:

| | |
|---|---|
| chunks the chunk-list walk reaches | 7 251 |
| chunks the shape scan finds | **24 046** |
| of the walk's chunks, found by the scan | **7 251 of 7 251 - the positive control** |
| accepted chunks that overlap another | **0** |
| extra chunks whose `[base]` link lands on another accepted chunk of the **same bucket** | **15 980** |
| extra chunks whose link is 0 - a list tail | **815** |
| extra chunks whose link goes nowhere valid | **0** |

Across all 37 dumps: 889 170 chunks accepted, 266 708 of them on the known context's lists,
**592 273** of the remaining 622 462 linking to another accepted chunk of the same bucket,
**30 186** list tails and **3** that link nowhere - three in six hundred thousand [L].

Every extra chunk is on a well-formed chunk list of its own bucket. **815 tails among the extras,
plus the four the known context owns, is 819 chunk lists** - the known context's four hold 7 251
chunks between them and the other 815 average ~20 each. At four lists per pool context that is on
the order of two hundred pool instances, but I have not counted contexts, only lists. The
scan-to-list ratio is 3.1-3.4x in every one of the 37 dumps [L].

Two consequences, and the second is the one that matters:

* Every "N slots enumerated, 0 damaged" line this project has printed - `poolchain.py`'s
  174 528, the sentry's 1 153 752 - is a count over **one context**. The real slot count in the
  372984 dump is 393 408, not 174 528.
* **The live sentry is watching one context's headers.** It is not blind because it is looking
  at the wrong thing; it is blind because it is looking at 30% of the thing. 14 of the 59
  confirmed damaged objects found below sit in chunks the sentry's walk never reaches [L].

I did not chase what the other 815 lists belong to. The link test says they are real; the
decompilation of who constructs them is not done and is the obvious next static pass.

## 1. The four detectors, their controls, and what each cannot see

Each detector has a **positive control** it must find in its own dump before its zeros count.
All three hand-found victims are wired into `--verify` and all three pass.

| | what it flags | control (372984) | ratio | blind to |
|---|---|---|---|---|
| **D1 header** | slot header at `body-8` is not the bucket's slot size | 393 408 headers exactly right | - | anything in a payload |
| **D2 high dword** | 8-aligned qword that is not a valid address but whose **low dword is** | see the tier table | 1 in 18 899 at T2 | a damaged field that was not a pointer; damage that lands on a still-valid address; a **low**-dword change |
| **D3 misaligned** | unaligned qword that becomes a vtable pointer when moved ±1,2,3,4,6 - target's first four qwords all in `.text` | 17 288 aligned vtable pointers found by the same test; 680 662 unaligned qwords examined, 118 893 with a committed aligned neighbour rejected | 0 false positives | a low-dword change on any pointer that is **not** a vtable pointer. There is no way to tell a damaged data pointer from a live one by looking at it |
| **D4 twin** | two fields in one allocation with equal low dwords and different high dwords | 34 331 identical-pointer pairs in one object | - | any object that does not hold the same pointer twice - most do not |

**D2's tiers exist because the loose reading is worthless**, and the tool prints all of them so
that can be seen rather than asserted. 296 MB of the low 4 GB is committed private memory, so
roughly one random dword in fourteen lands in it, and UTF-16 strings spray those:

```
372984:  T0 committed                       flagged 84572   control 577254   1 in 7
         T1 8-aligned + PRIVATE             flagged  9821   control 453105   1 in 46
         T2 an enumerated slot body         flagged    18   control 340183   1 in 18899
         T3 + referenced elsewhere          flagged    14   control 131424   1 in 9387
         T4 + the only dirty ref to that slot   flagged 2
```

T3 is the control the 372984 write-up ran by hand ("`0x301bad30` occurs 13 times, the corrupted
form twice") turned into a predicate. **T4 came from a false positive that T3 let through**:
eleven bucket-1 objects in that dump each hold `0x38950000` at `+0x18` with a different value
above it, which is a packed `{u32; u32}` pair and not eleven stray increments. A value that
repeats is a constant; damage is sporadic. The same rule was then needed for D3 - dump 1007028
has **seven** `0x40` slots holding the identical `0x143270001` at `+0x18`, and `0x143270000` is
not a vtable start but the middle of a long `.text` pointer array, so the four-qword test passes
anywhere inside it. Seven identical hits in a 372 s process cannot be a writer that fires every
180 s.

## 2. How many objects are damaged - the answer is single digits

Confirmed damage per **process**, at the strictest tier of each detector, taking that process's
oldest dump [L]:

```
 pid       alive   D1   D2   D3   D4   distinct objects
 992840      11s    0    1    0    0     1     <- not in the value family; probably noise
 1010344     38s    0    0    0    0     0
 362016     220s    0    4    0    3     4
 1007028    372s    1    1    0    0     2
 212108     375s    2    2    0    2     4
 372984     491s    0    2    0    1     2
 345148     544s    0    1    0    0     1
 374940     630s    1    2    0    1     3
 249464     912s    0    0    0    0     0
 84428     1076s    4    1    0    1     5
 238480    1378s    4    4    0    1     8
 356516    2522s    0    4    0    4     4
 358616    3026s    3    1    0    1     4
 179092    3069s    9    4    0    1    13
 322016    4215s    5    2    0    1     7
 419988    7015s    0    0    1    0     1
```

**0 to 13 distinct objects, 59 in total across the 16 processes** (45 in chunks the chunk list
reaches, 14 only in the shape scan). Not one, and not dozens. For the guard page this settles
the sizing question: **it must catch a handful of writes per session, not one and not a
thousand** - but it fires often enough that a quarantine of a single bucket will be tested
several times per run rather than once.

The two zero rows are real results, not silence: pid 249464's four sentry dumps span 372-912 s
and every detector reports zero with its control intact.

## 3. The offset is NOT consistent, and that is the answer to the overrun question

Offsets of confirmed damage relative to the containing allocation, deduplicated by
`(pid, body, detector@offset)` across all 37 dumps [L]:

```
   +0x10   32     the damaged field is at body+0x10; the dword written is at body+0x14
   -8      29     the slot's own size header; the dword written is at body-4
   +0x18    7
   +0x40    2
   +0x28    2
   +0x50    1
   +0x30    1
   +0x0     1     the 419988 vtable pointer, a LOW-dword change
```

Two populations dominate and they are at **different offsets in different structures**. Reading
that as a fixed-offset overrun requires a constant, and there is none: the written dword sits at
`body-4` in one population and `body+0x14` in the other, which are `chunkbase+0xc` and
`chunkbase+0x24` modulo the bucket-1 stride of `0x28`. **[D] Not one fixed offset relative to
the damaged allocation, so not one overrun from a fixed neighbour.** §5 shows they can still be
one fixed pair of offsets relative to the *writer's* object, which is a different claim and the
one worth chasing.

**What is consistent is the field, not the offset.** Every `+0x10` hit is the same structure. In
**11 of the 16 distinct objects D4 flags** the object is an MSVC `_Tree` node - `_Left` at `+0`,
`_Parent` at `+8`, `_Right` at `+0x10` - and in **7** of those all three links point at the node
itself, which is what `_Myhead` looks like for an **empty** `std::map`/`std::set`. The damaged
field is `_Right`, every time:

```
   pid 212108   body 0x2f3ecc68  +0x0=0x2f3ecc68 +0x8=0x2f3ecc68 +0x10=0xffffffff2f3ecc68
   pid 238480   body 0x2f1f46a0  +0x0=0x2f1f46a0 +0x8=0x2f1f46a0 +0x10=0xffffffff2f1f46a0
   pid 322016   body 0x3a43df10  +0x0=0x3a43df10 +0x8=0x3a43df10 +0x10=0xffffffff3a43df10
   pid 84428    body 0x2ef45d90  +0x0=0x2ef45d90 +0x8=0x2ef45d90 +0x10=0xffffffff2ef45d90
   pid 356516   body 0x2ecbb238  +0x0=0x2ecbb238 +0x8=0x2ecbb238 +0x10=0xffffffff2ecbb238
   pid 356516   body 0x343ecc98  +0x0=0x343ecc98 +0x8=0x343ecc98 +0x10=0xffffffff343ecc98
   pid 356516   body 0x696fd98   +0x0=0x696fd98  +0x8=0x696fd98  +0x10=0xfffffffd0696fd98
```

Seven processes, seven empty-container head nodes, the same field, the same sign. That is a
**type**-shaped cluster, not an address-shaped one.

## 4. The value family, enumerated rather than assumed - and it moves in both directions

High dwords on confirmed damage, deduplicated by `(pid, address, value)` [L]:

| where | values |
|---|---|
| **D1, slot headers** (29 distinct addresses) | final value `0x0000000100000020` x23, `0x0000000200000020` x5, `0x0000000300000020` x1 |
| **D2 tier 4** (31 instances) | in the family: `0xffffffff` x12, `0x1` x4, `0xfffffffe` x1, `0xfffffffd` x1 = **18**. The other **13** are 13 *distinct* arbitrary values at 13 different offsets - no value and no offset repeats - and are read as residual noise |

Two things this corrects or adds:

* `poolsentry.rs`'s doc block records **"14 of 14 identical `0x0000000100000020`"**. Over 37
  dumps it is **23 of 29**; the other six are `+2` and `+3`. The header's high dword is not set
  to one, it is **incremented**, and sometimes more than once.
* And one header was watched doing it. Pid 238480's header at `0x38606e40` reads
  `0x0000000100000020` at 728 s and at 908 s, and **`0x0000000300000020` at 1378 s** [L] - a
  read-modify-write on a header, in the increment direction, seen across three captures of one
  process.
* The headers go **up** (`+1, +2, +3`) and the `_Tree` `_Right` fields go **down**
  (`-1, -2, -3`). Same shape, opposite sign, in the same sessions.

## 5. The measurement this whole pass was worth: one field, decrementing

Pid 356516 wrote four dumps. The **same object, the same field**, across all four [L]:

```
   550 s   0x696fda8   _Right = 0xffffffff0696fd98      -1
   730 s   0x696fda8   _Right = 0xfffffffe0696fd98      -2
   910 s   0x696fda8   _Right = 0xfffffffe0696fd98      -2   (no firing in this interval)
  2522 s   0x696fda8   _Right = 0xfffffffd0696fd98      -3
```

`0x696fd98` is an empty `_Tree` head node in a `0x20` slot. Its `_Right` points at itself; the
low dword is intact and unchanged in all four captures; the high dword decrements. **This is a
32-bit counter at `field+4` being decremented, observed four times on one address in one
process.** [L]

`research/the-writer-damages-live-objects-2026-09-08.md` §3 offered exactly this as its single
inference and said a second observation would settle it. This is the second, third and fourth,
on the same address, and it is the strongest evidence in the project for the **refcount through
a stale pointer** reading:

* it is a read-modify-write, not a store - the value changes by one each time, so the writer
  reads what is there;
* the target does not move - the same address across 1 972 s of one session;
* increments and decrements both occur, which is what an `AddRef`/`Release` pair looks like when
  one of the two arguments is stale.

**[D] And the two populations are exactly `0x10` apart, which unifies them.** Relative to a
bucket-1 slot body `B`, the written dwords are:

```
  decremented   B + 0x14     _Right's high dword, inside the 0x20 payload
  incremented   B + 0x24     because header_{k+1} + 4 == body_k + 0x24 :
                             body_k        = chunkbase + 0x10 + 0x28k
                             body_k + 0x24 = chunkbase + 0x0c + 0x28(k+1) = header_{k+1} + 4
```

So **one object, two 32-bit counters `0x10` apart at `+0x14` and `+0x24`, one decremented and
one incremented**, would produce both populations from one write site - and `+0x24` on an object
the allocator gave a `0x20` slot is a **four-byte overrun onto the next slot's header**, which is
precisely the second hypothesis `crates/grap-stub/src/poolsentry.rs` already names ("a 4-byte
overrun at `+0x24` of an object sized to the `0x28` stride rather than the `0x20` slot"). The two
readings that have been treated as alternatives for weeks are the same write pair seen from two
sides. **[I]** that the object is refcounted and `p` is stale; the arithmetic above is **[D]** and
is checkable, and the guard page can come back false on it.

## 6. Clustering, and accumulation

**Clustering: no fixed spacing, but the set is stable and only ever grows.** [L] Pid 238480's
five dumps carry the identical damaged addresses in the same order, one more each time:

```
  368 s            2f1f46a0 2f3ddce8 339e1a40 37befdf8          38608a90
  548 s   2ced6b10 2f1f46a0 2f3ddce8 339e1a40 37befdf8          38608a90
  728 s   2ced6b10 2f1f46a0 2f3ddce8 339e1a40 37befdf8 38606e48 38608a90
  908 s   2ced6b10 2f1f46a0 2f3ddce8 339e1a40 37befdf8 38606e48 38608a90 388cf958
 1378 s   2ced6b10 2f1f46a0 2f3ddce8 339e1a40 37befdf8 38606e48 38608a90 388cf958
```

Nothing ever heals and nothing ever moves. The gaps between damaged addresses are megabytes and
irregular, with one recurring exception: a gap of exactly **`0x50`** appears in three different
processes (212108, 179092 twice), and in each case it is a damaged node and the node its damaged
`_Right` points at, two bucket-1 slots apart. That is adjacency of two related allocations, not
a stride.

**Accumulation: monotone within a process, weak across processes.** [L] Pearson r = 0.27 and a
rank correlation of ~0.46 between process age and confirmed object count over the 16 processes,
which is far too weak to call a rate. Within a process it is unambiguous - the count never
decreases in any of the six processes with more than one dump.

The cleanest per-firing number comes from pid 238480's header count, which goes **1, 2, 3, 4 at
368, 548, 728 and 908 s - exactly 180 s apart** [L]. That is the writer's clock, seen as an
accumulating count for the first time. It has to be read with care: those dumps were *triggered*
by the sentry finding a damaged header, so the sampling times are not independent of the count.
What is independent is that the **payload** damage in the same four dumps stayed at four objects
throughout - the header population and the `_Tree` population do not advance together.

**The 7 015 s outlier.** Pid 419988 ran for nearly two hours and shows **one** confirmed damaged
object. If damage accumulated at one per 180 s it should show ~39. It does not, and its pool is
also anomalous - 347 664 slots against ~390 000 everywhere else, and 350 free bucket-1 slots
against 14 104. I have no explanation and am recording it rather than smoothing it. It is the
single strongest piece of evidence *against* steady accumulation.

## 7. The whole-image sweep found nothing the pool sweep did not, and here is why

`--full` scans every committed byte for the enumerated high-dword family. On the 372984 dump
that is 346 917 flagged qwords in 1 216.9 MB, against a sampled control of 68 654 clean 32-bit
pointers per 36.2 MB - the family fires at roughly **1 in 39** against ordinary pointers, and 1
in 7 on some dumps. **That number is meaningless and is printed to say so.** Outside the pool
there is no allocation map, so a flagged qword cannot be told from a legal 64-bit datum.

The one whole-image reading that *has* a control is: flagged qwords whose **repaired** value is
an enumerated slot body. That finds a damaged pointer-to-a-pool-object held anywhere - an NT heap
block, a big-allocator buffer, a global, a stack slot - which the payload sweep structurally
cannot see. Across the four dumps swept it produced 8-16 hits each, and on inspection they are
coincidences: values repeated at the identical address in different dumps
(`0x7ff9efb715c0 = 0x206553230` in both 372984 and 419988), page-aligned "pointers" like
`0x137370000`, and system-DLL `.data` constants. The only one that reproduces a payload finding
is `0x3848e308` in 179092, which is a pool payload anyway.

**[D] So: no evidence of damage outside the pool - and the honest form of that sentence is that
outside the pool this instrument has no control and could not have found any.** That is a
blind spot, not a negative.

## 8. What to do with this

1. **The sentry and `poolchain.py` cover 4 chunk lists of 819, ~30% of the chunks.** Either widen them to the
   shape scan, or say in every report that the counts are one context's. `tools/damagescan.py`
   has the enumeration; `carve_scan()` is 40 lines and needs no chunk-list head.
2. **`-GuardBucket 0x20` remains right.** 17 of the 18 in-family D2 hits and all 29 damaged
   headers are bucket-1; the one exception is a bucket-3 slot. Bucket 2 carries only the two
   vtable cases.
3. **The guard page should expect several firings per run, not one.** 0-13 objects per session,
   and 8 of the 16 processes have three or more.
4. **The best single target for a static pass is a function that touches `+0x14` and `+0x24` of
   the same pointer**, decrementing one and incrementing the other, reachable from a 180 s
   timer. §5 derives that pair from the data; it would explain both damage populations from one
   site. That is one Ghidra question and it costs no client run.
5. Nothing here authenticates and nothing here was run against a client. Every number above is
   from files already on disk.

## 9. Reproducing

```
powershell -ExecutionPolicy Bypass -Command "cd C:\MapleCW; python tools\damagescan.py dumps\maplecw-crash-372984-c0000005-1.dmp --verify"
powershell -ExecutionPolicy Bypass -Command "cd C:\MapleCW; python tools\damagescan_all.py"
```

`--verify` exits non-zero unless the three hand-found victims are found by the detector that is
supposed to find them. Per-dump text and JSON land in `research/damagescan/`; a sweep is ~15 s
per dump and `damagescan_all.py` runs them one at a time on purpose, because a client is usually
running on this machine.
