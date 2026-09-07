# The writer runs on a 180-second clock

Four catches in one idle session, and the intervals are the whole finding:

```text
23:59:43.551   FINDING #1   slot 0x38608a88
00:02:43.553   FINDING #2   slot 0x2ced6b08     +180.002 s
00:05:43.668   FINDING #3   slot 0x38606e40     +180.115 s
00:08:43.688   FINDING #4   slot 0x388cf950     +180.020 s
```

`[L]`, from `client-patched\maplecw-hook.log`, process 238480, sentry armed 23:53:38 with
`dumps=4`. All four dumps written. Adding the previous evening's pair - 22:34:34.376 and
22:37:34.414, **+180.038 s** - the family now has **six catches across two sessions and four
consecutive intervals, every one of them 180.0 s to within 0.12 s.** `[L]`

Tags: **[L]** read out of a file, **[D]** derived, **[I]** inferred.

---

## 1. What this settles

**The writer is on a timer, not on traffic.** Every previous file could only say the damage
"accumulates with session age" and that no packet correlated. A period of 180.00 s held to one
part in 1500 over four intervals is not traffic, not allocation churn and not chance.

**It kills the census lead of `heap-corruption-2026-09-06.md` §4, and that is a retraction of
my own reading.** Two catches 180 s apart, each landing a few hundred ms before a `0x013D`
census send, looked like a census correlation. The census is on a **30-second** grid, so 180 is
a multiple of it and a census will *always* appear beside a catch. This session shows the same
alignment - the census lands 0.66 to 0.72 s after each catch, four for four - and it means
nothing: **five out of every six censuses produce no catch at all.** `[D]` The alignment is two
grids sharing a wall clock, not cause.

## 2. What the four catches were sitting in, and it is not the point

| | slot contents (UTF-16 where printable) |
|---|---|
| #1 | three self-pointers, then `01 01 f2 37` - live, not on the free list |
| #2 | `.rdata` pointers `0x1434210d8` and `0x143422e28` with two `1`s between them, then `"eet"` - an object with a vtable, not a string |
| #3 | self-pointers, then `".img"` |
| #4 | a pointer, then `"qLUK"` - the tail of `reqLUK`, an item-requirement property name |

Neighbours across the four include `"weapon"` as a clean 6-character BSTR, `"9.img"`,
`"ingT2"`, `"eers"`. `[L]` So the `0x20` class holds WZ property and path strings plus small
vtable'd objects, and **the four damaged slots have nothing in common with each other**: one
live, one free, one a string, one an object. The writer does not care what it hits, which is
the same conclusion §5.2 of the previous file reached from two samples and is now four.

Two were `va%16=8` (heap failure type 9 if freed) and two `va%16=0` (type 8), which is why the
family's failure types have never been consistent.

## 3. The burst is bounded, with one named blind spot

After 00:08:43 the sentry ran for **7 minutes 50 seconds more** and reported nothing. Its
heartbeats are in the log every 60 s with `carve PASS` on all four buckets throughout, so it
was walking the whole time. `[L]` The client then died on close at 00:16:33 with the family's
`0xC0000374`, in the second pooled free at `0x14019bbf3` reached through `operator delete` -
the same route `heapfix-did-not-hold.md` documents. `[L]`

So: **four events on a 180 s clock, then nothing.** The previous session was two events, then
death. `[D]`

**The blind spot, stated because the instrument has one.** `PoolWalk::mark_reported` inserts
the *header address* into a `reported` set, so a fifth event that damaged one of the four
already-reported slots again would be **silent**. There is no cap on findings, only on dumps,
so a new address would have been reported. The honest claim is therefore *"no new slot was
damaged after 00:08:43"*, not *"nothing happened"*. `[L]` for the code, `[D]` for the
distinction.

## 4. It is silent everywhere else

At all four moments, in the same logs: `[L]`

* **no packet** - the only traffic is mob movement and the client's own periodic reports, and
  those have 30 s and 120 s cadences that do not match;
* **no exception** - the hook counted **2** C++ throws all session, both in the first 1.4 s of
  startup;
* **no socket event** - one `SOCKET` line at connect and one at close, nothing between;
* **every thread parked in `ntdll`** at the moment of confirmation, as before.

## 5. Where this points, and what is not yet claimed

`0x2bf20` is 180000, and it appears **42 times in `.text`** (`tools/pe_packing`-style literal
scan of `client-patched\MapleStory.exe`, mapped through the section table). `[L]` They cluster:
seventeen in `0x140c92f62..0x140c94fc1`, three at `0x140c9b8b0..0x140c9ba50`, thirteen in
`0x14180d2a9..0x141811bd4`, and singletons including **`0x142e0fa4e`**.

That last one is read: it is the usage-report ticker `FUN_142e0f9e0`, called every frame from
the game-stage tick, and the branch the 180000 comparison selects calls `FUN_142d207f0`, which
**destroys a global `std::map` and frees its nodes into this pool**. `[L]`

**This is a lead, not the answer, and one measurement already argues against it**: that
routine's gate reads `[ctx+0x39ec]`, which was **0** in both dumps of the previous session,
making its comparison true on *every frame* rather than once every 180 s. `[L]` A per-frame
branch cannot produce a 180.00 s period. So either the field is non-zero during play and the
dumps caught it in an unusual state, or the timer is one of the other 41 sites. **Not
established either way, and 42 sites is a list to enumerate, not a candidate to assume.**

## 6. What to do with a period

A predictable event is a much easier target than a random one, and it changes what the next
instrument has to be able to do.

* **The guard-page build** (`heap-corruption-2026-09-06.md` §3.2) was already the only thing
  that can name the writing instruction, and the period makes it cheap: a run needs to survive
  about **six minutes** to catch two events rather than an unknown wait.
* **A cheaper half-step exists now.** The sentry walks every 100 ms, so a catch is up to 106 ms
  stale and every thread has parked by then. With the period known, the sentry could drop to a
  **5-10 ms** interval for a few seconds either side of the predicted time and sample threads on
  the tick it fires. That is a change to `poolsentry.rs`, no allocator surgery, and it either
  catches a running thread or proves the write is a single instruction between two walks.
* **The phase origin is not known.** First catches sit 192.9 s and 367.2 s after hook install
  in the two sessions, which differ by 174.3 s - not a multiple of 180. So the clock does not
  start at process start, and predicting the *first* catch needs another session. Predicting
  the *next* one, once one has happened, is exact.
