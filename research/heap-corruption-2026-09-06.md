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

## 4. What this file does not claim

* That the three `2`s and the `-1` are the same writer as the `1`s. Same width, same offset,
  same size class for the `2`s; the `-1` is in a live node, not a free slot. **[D]** each.
* That `!rates` had anything to do with it. Its reply was the first chat line in thirty
  minutes and the tree it walked was already broken. The 40-second `!rates` test in the plan
  is what separates that from a fatal command, and it has not been run.
* Which of the three idle sources it is. Counts do not say; the sentry's times will.
