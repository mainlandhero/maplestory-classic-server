# The sixth dump: eight damaged slots in an idle session, and the value is not always 1

The owner, 2026-09-06: *"Is there a way that we can track what has been corrupting the client? The
client was sitting in the same map while simulating mob motion. It wasn't even getting hit."*

`dumps\maplecw-crash-179092-c0000005-1.dmp`, written 21:53:26 local (01:53:26 UTC), 1.3 GB,
process alive **3069 s**. Walked with `tools/poolchain.py` (chunk-size identity: 2540 of 2540
bucket-1 chunks pass, 0 fail - the instrument's own control) and `tools/dumpwalk.py`
(header, module base, pid and Memory64 coverage all PASS).

## 1. What the dump shows

### 1.1 Eight damaged slots, all in bucket 1 - and three of them read `2`

```text
bucket 1 (0x20 slots): 81 280 enumerated, 8 with a non-zero high dword
   0x3a224be8  0x0000000200000020
   0x3a2144c8  0x0000000100000020
   0x3848e340  0x0000000100000020
   0x38349e10  0x0000000100000020
   0x369fa030  0x0000000100000020
   0x330a3640  0x0000000200000020
   0x2f32eeb0  0x0000000100000020
   0x2ee8a930  0x0000000200000020
buckets 0/2/3: 53 376 + 40 608 + 17 832 slots, 0 damaged
```

**[L]**. Two things here are new.

* **The count.** Eight, against a previous maximum of three. At 3069 s that is one per ~380 s,
  in line with `heap-crash-pattern.md`'s one per ~298 s of in-field time - and this session is
  the closest thing yet to the *unbiased* sample every earlier file asked for: the death was
  not a `0xC0000374` free of a damaged slot (see §1.3), so the eight were not selected by it.
* **The value.** `heap-wild-write.md` §10 wrote, of the then five-for-five `1`s: *"a single
  different value breaks it and puts a wild write back on the table."* Here are three `2`s
  beside five `1`s. That sentence was about a `1` being a *constant* - "refcount or flag
  initialised to 1". A `2` next to `1`s is not an arbitrary value; it is the next one. The
  reading this supports is a **counter**, not an initialiser: something increments a 32-bit
  field at `slot + 4` (= `body - 4`), and a slot that is hit twice reads `2`. **[D]**

### 1.2 The map node that killed the client carries `-1` in the same position

The death itself was `0xC0000005` at `0x1425e6fd2` (`dumpwalk.py`, exception record): a
red-black-tree `lower_bound` walk - `cmp byte [rax+0x19], 0` is `_Isnil`, `cmp dword
[rax+0x1c], edx` the integer key - dereferencing a node pointer of

```text
0xffffffff2f822a41
```

That is not a random 64-bit value. Its low dword `0x2f822a41` has the shape of the low half
of every heap address in this dump (`0x2ee8a930 .. 0x3fa5a7c8` above), and its high dword is
exactly `0xffffffff` - **`-1`, written as a 32-bit store into the upper half of an 8-byte
pointer field.** `1`, `2` and `-1`, all 32 bits wide, all at `+4` of an 8-byte word: an
increment, a second increment, and a decrement from zero. **[D]**, and stated as a reading,
not a finding: the node was live when it was hit, which the free-list slots were not, so if
it is the same writer it is one that does not care whether its target is free.

The stack, from the dump's unwind (`.pdata`, not the scan): the chat-notice printer
`0x1415eca30` called from the `0x00BB` handler `0x142d95630`, called from the game
dispatcher `0x142cbaa80` - i.e. the client died drawing the `!rates` reply, which was the
first chat notice in thirty minutes. `research/talking-back.md` §1.1 is that handler. The
tree being walked belongs to the chat window's text path and is not otherwise identified.

### 1.3 It is not the `0xC0000374` death, and that is why the sample is honest

Every previous dump of this family died in the pool's free, on a damaged header. This one
died elsewhere, with eight damaged slots still sitting in the pool that the free never
reached. So the count is not conditioned on a fatal free - it is what 51 minutes of this
client accumulates.

## 2. What the client was doing, from `world.log`

Session 01:04:36 - 01:53:29 UTC. The owner: standing on one map, controlling the mobs, never hit.

| traffic | count |
|---|---|
| mob move reports in (`0x02FF`) / control acks out (`0x03E4`) | 34 041 / 34 038 |
| NPC idle chatter out (`0x0453`) | 1 317 |
| scrolling banner out (`0x00AC`, up/down every 5 min - rates were 2x/5x/5x all session) | 21 |
| attacks (`0x00DF`) | 10 |
| hits taken (`0x00E5`), remote damage, field entries/exits | 0 |
| chat notices (`0x00BB`) | 3 |

So the writer runs on idle traffic. `heap-corruption-2026-08-27.md` §7 asked exactly this
question - *"if a deliberately-killed idle run of ~900 s shows two or three damaged slots,
the writer is on a timer and the search narrows enormously"* - and this run answers it in the
affirmative with eight in 3069 s. What it cannot do is pick among the three idle sources:
counts alone do not correlate (21 banners, 1 317 chatter lines, 34 038 acks, 8 slots).
**The moment of each write is the missing measurement**, and that is §3.

## 3. How to track it - the instrument already exists and has run for nineteen seconds

`crates/grap-stub/src/poolsentry.rs` is a live watch on bucket 1: every 100 ms it walks the
chunk chain read-only (no lock taken), confirms a candidate over three spaced re-reads, and
on a catch logs the slot, its fresh contents, its predecessor's, one thread's stack, and
writes one dump. Its module docs are the design; its point is that a catch is **~100 ms
old**, where every dump so far has been tens of thousands of allocations old.

**It has been armed exactly once**, 2026-08-30 22:01:54, in a run that ended at 22:02:11 -
nineteen seconds, no heartbeat, no catch. It has never been used for the thing it was built
for. Two reasons, both plumbing: `test-server.ps1 -PoolSentry` used to sit past an early
`return` (fixed), and **the launcher never arms it** - by design, `crates/launcher/src/client.rs`
leaves the marker alone - and every session since 2026-09-01 has gone through the launcher.

### 3.1 The run to do, and what each outcome means

Arm it. Either `tools\test-server.ps1 -SetFieldProbe -PoolSentry`, or - for a client started
from the launcher, which is how the owner's sessions run now - **create the file
`maplecw-hook.sentry` containing `on` in the client folder before pressing Start Game**. The
hook reads it at attach and deletes it, so it is one launch per marker. Then reproduce today:
one map with mobs, stand still, ten to fifteen minutes, and **close the client yourself** -
the death is not needed and a deliberate end keeps the sample unbiased.

The hook log (`client-patched\maplecw-hook.log`) then says one of three things:

| what the log shows | reading |
|---|---|
| `POOL SENTRY ARMED`, a heartbeat every 60 s, **no catch** in 15 min | the rate is not one per ~6 min while idle after all - today's eight were something else this run did not do. Say what was different |
| catches, each with a time | **the payload.** For each catch, open `world.log` at that time ±100 ms. Three idle sources send at very different rhythms - a banner on the 5-minute mark, chatter every ~1.3 s, acks tens per second - so two or three catches will already say which one they sit beside, or that they sit beside none |
| a catch whose fresh occupant is a **live** object, or whose predecessor is live | the discriminator `heap-corruption-2026-08-27.md` §6.7 said a dump cannot give: overrun from the neighbour vs. a stale pointer into this slot |

A catch also leaves a dump written within a second of the write; `poolchain.py` on it should
show exactly one damaged slot with a `1`, and if the same slot later shows a `2`, the same
stale pointer was used twice - which is the strongest single fact the counter reading could
get.

### 3.2 If the moment does not name the packet: the build that names the writer

`heap-wild-write.md` §10 already states it: *"What would actually name the writer is a write
watch on `body-4` of a slot known to be damaged-prone."* A hardware watchpoint cannot do that
- there are four debug registers and twenty thousand free slots - but the hook can make
*every* freed slot a watchpoint:

* intercept bucket 1's allocation and free (the three entry points `heapfix-did-not-hold.md`
  names, all runtime patches of the kind the hook already makes);
* serve each `0x20` allocation from its own page inside a large reserved region, header and
  body at the page start;
* on free, **decommit the page and never reuse it** (address space is the only cost - a
  reserved page holds no memory; at ~1 000 frees a second that is ~14 GB of address space an
  hour, out of 128 TB);
* the next write to any freed slot - the increment at `body-4`, the decrement, anything -
  faults **at the writer**, and the hook's vectored handler already catches faults, names the
  RIP, walks the stack and writes a dump. That is the whole answer in one exception record.

Costs, stated: bucket 1 is the client's hottest size class, so `VirtualAlloc`/`VirtualFree`
per allocation is a real slowdown (microseconds each - fine at thousands a second, not at
hundreds of thousands); live pages cost 4 KB each (57 558 served today → ~230 MB, tolerable);
and an inline hook that misses one of the three entry points sees a block it did not hand out
come back through free, which the shim must recognise by address range and pass through. It
is a day's build and one idle launch, and it is the only route left that produces a *who*
rather than a *when*. Do §3.1 first: if a catch lands within 100 ms of the same packet three
times, the build may not be needed.

## 5. The sentry run, same evening: two catches, the whole chain, and the offset

The owner armed the sentry by the marker file and stood idle on map 10001010 (29 mobs). Process
212108. `client-patched\maplecw-hook.log`: `POOL SENTRY ARMED`, a heartbeat every 60 s with
its own controls passing (`2574 chunks x 32 = 82368` against the allocator's carved counter,
walk 740 µs average), then:

```text
22:34:34.376  FINDING #1  slot 0x2f3eccb0  header 0x0000000100000020   NOT on the free list
22:34:35.101  dump written 724 ms later: maplecw-sentry-212108-finding1-1.dmp
22:37:34.414  FINDING #2  slot 0x39203c50  header 0x0000000100000020   ON the free list
22:37:35.134  CLIENT FAULT 0xC0000374 - the pool free of 0x39203c50, the slot caught 720 ms before
22:37:35.792  crash dump: maplecw-crash-212108-c0000374-1.dmp
```

**[L]** for all of it. Three things this run settles that no dump could.

### 5.1 The chain, observed once end to end

Catch, then 720 ms later the client frees that exact slot, reads the damaged header, diverts
to `HeapFree`, and dies with the family's `0xC0000374`. `poolchain.py` on the crash dump: two
damaged slots, `0x39203c50` marked as the one that was freed, and `0x2f3eccb0` still sitting
there. Every earlier file inferred this chain from a corpse; this is the first time it was
watched.

### 5.2 The writer does not care whether the slot is free

Finding #1's slot was **live** - not on the free list, body freshly written (`0x2f3eccb8` twice,
its own address, then `"234"` in UTF-16). Finding #2's slot was **free**, and so were both of
its neighbours (`"EDOPHILIA"`, `"322007.img"`, `"inolympics"` - stale UTF-16 fragments, the
predecessor and successor tailed with `dd dd dd dd`). Same value, same offset, one slot
somebody owned and one slot nobody did. That kills "the pool's own push/pop wrote it" and
"the occupant underruns its own buffer" in one stroke: the writer holds a pointer of its own
and writes through it regardless of the slot's state. **[D]**

The 0x20 class is short UTF-16 strings and small nodes - WZ image names among them - which is
the memory that churns while mobs animate, and why an idle map produces it.

### 5.3 The offset is the boundary between two prefix conventions

The sentry names the arithmetic: the pool's header is 8 bytes at `body-8`; the 0x20 class is
laid out BSTR-style with a 4-byte count at `body+0` and **the pointer handed around is
`body+4`**. The four bytes that change are at `body-4` - which is `(body+4) - 8`. Code holding
the `body+4` pointer and writing a 4-byte field at **pointer − 8**, as it would for an object
with an 8-byte cookie before its data, lands exactly on the pool header's high dword. The
values are `1`, `1`, `1` … with an occasional `2` and today's `-1`: a **reference count**
written through a data pointer of the wrong prefix convention. **[D]**, and the strongest
reading the family has had - it explains the offset, the width, the values, both slot states,
and why the string in the slot has never mattered. It has not been seen as an instruction.

### 5.5 SUPERSEDED the next day: it is a 180-second clock, and §4 above is retracted

A four-catch run on 2026-09-07 measured the intervals: **+180.002 s, +180.115 s, +180.020 s**,
and this session's own pair was **+180.038 s**. Four consecutive intervals, all 180.0 s to
within 0.12 s. `[L]` **The writer runs on a timer, not on traffic.**

That retracts the census reading offered in §4. `0x013D` is on a **30-second** grid and 180 is a
multiple of it, so a census lands beside every catch by construction while five out of six
censuses produce nothing at all. The alignment is two grids sharing a wall clock. Full write-up
and what to do with a period: `research/the-180-second-clock-2026-09-07.md`.

## 5.4 Why the sentry cannot name the instruction, and what can

Both catches sampled all 68 threads at the moment of confirmation and every one was parked in
`ntdll` waits. The write is one instruction; a 100 ms walk finds the slot long after the thread
that wrote it has gone back to sleep. No tuning of the sentry changes that. The instrument that
names the writer is the one §3.2 describes - every freed `0x20` slot turned into a guard page,
so the next write through the stale pointer faults *at the writer*, with RIP and stack in the
hook's exception handler. §5.3 says what that fault will show: a 4-byte store at `reg - 8`
where `reg` holds a `body+4` pointer.

Two cheaper things first, both one launch: run the sentry again with `dumps=4` in the marker
(the cap of 1 spent the only dump on finding #1), and note that `0x01ED` - the client's own
usage-report channel, `CLIENT_USAGE_REPORT` - fired 89 ms before catch #2 and not near catch
#1, so it is a coincidence until a third catch says otherwise.

## 4. What this file does not claim

* That the three `2`s and the `-1` are the same writer as the `1`s. Same width, same offset,
  same size class for the `2`s; the `-1` is in a live node, not a free slot. **[D]** each.
* That `!rates` had anything to do with it. Its reply was the first chat line in thirty
  minutes and the tree it walked was already broken. The 40-second `!rates` test in the plan
  is what separates that from a fatal command, and it has not been run.
* Which of the three idle sources it is. Counts do not say; the sentry's times will.
