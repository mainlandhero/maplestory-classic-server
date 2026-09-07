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

## 3a. The repair works: 26 minutes, seven catches, seven repairs, no death

First run with `-SentryRepair` (process 249464, 2026-09-07 00:40 to 01:06). `[L]`

```text
#1 00:46:52.671                    #5 00:58:52.744  +179.965
#2 00:49:52.714  +180.043          #6 01:01:53.763  +181.019
#3 00:52:52.715  +180.001          #7 01:04:53.740  +179.977
#4 00:55:52.779  +180.064
POOL SENTRY alive 1561s: 7 confirmed finding(s) ... 7 header(s) repaired
CLIENT FAULT: none.   REPAIR REFUSED: none.   Closed by hand.
```

**Seven repairs, zero refusals, no `0xC0000374`, and the client closed cleanly.** The
comparable session the night before - same map, same idle, same client, no repair - died at 23
minutes on exactly the shutdown free this is meant to survive. `[L]` One session is not proof
that the mitigation always holds, and it is a race by construction, but the prediction it made
came back true on its first outing.

**The one long interval is evidence *for* the mechanism, not against it.** Five of the six gaps
sit within 0.07 s of 180.000; #6 is 1.019 s late and **the phase stays shifted afterwards**
(#7 is 179.977 after it, not 178.96 back onto the old grid). That is exactly what §5a's template
does: the gate re-arms `LAST := now` at the *firing* instant, so a frame the client was late to
run moves every subsequent firing with it. A fixed-origin timer could not do that. `[D]`

## 4a. Is it our scrolling banner? No, and the test is worth keeping

The owner, 2026-09-07: *"Does this have anything to do with our scrolling message? Since that's the
only component of our server that seems to be on a 180 second timer."*

The right question, and it nearly passes. Measured from `world.log` of the run in progress: `[L]`

```text
04:42:52.117   04:44:52.017  +119.900   04:47:52.171  +180.154   04:49:52.059  +119.888
04:52:52.398  +180.339       04:54:52.268 +119.870    04:57:52.029 +179.761
```

So the banner alternates **120 s and 180 s** - a 300-second cycle with two sends in it - and in
*this* session every event, banner and catch alike, lands on `:52`. Two of the four catches sit
within 0.3 to 0.6 s of a banner send. On this session alone it looks like a hit.

**Three things kill it, and the first is sufficient.**

1. **The banner's cycle contains a 120-second leg, and no catch interval has ever been 120 s.**
   Seven measured intervals across three sessions: 180.038, 180.002, 180.115, 180.020, 180.045,
   180.003, 180.066. `[L]` If the banner were the trigger, roughly half of those would be 120.
   None is.

2. **The phase relationship is different in a different session, which is what "unrelated
   clocks" looks like.** In the four-catch session the banners are at `:52.0`–`:52.2` and the
   catches at `:43.5`–`:43.7`: `[L]`

   ```text
   catch 03:59:43.551   nearest banner 03:59:52.154   +8.60 s
   catch 04:02:43.553   nearest banner 04:02:52.058   +8.51 s
   catch 04:05:43.668   nearest banner 04:04:52.120   -51.5 s   (none closer)
   catch 04:08:43.688   nearest banner 04:07:52.090   -51.6 s   (none closer)
   ```

   Two catches with no banner inside fifty seconds, and a fixed 8.5 s offset on the other two.
   Coincidence in one session and an 8.5 s offset in another is two clocks that share a rough
   origin and then drift, not cause and effect.

3. **The period is generated inside the client.** §5a reads the gate: the ticker re-arms its own
   timestamp on the firing branch, which is what makes the interval exactly 180 000 ms. Nothing
   we send participates in that. `[D]`

**Why both sessions still put everything near the same second** is the common origin, not a
link: our banner is computed per session from the session's own anchor, and the client's ticker
seeds its timestamp on its first call, which is a second or two after the same world entry. Two
clocks started by one event look aligned until they are measured across sessions.

**This is the second time a periodic thing of ours has lined up by construction** - the
`0x013D` census was the first, in §4 - and the falsification was the same both times: *find a
session where the phase differs.* Worth doing before the next correlation is believed.

## 5a. The clock has a name: a family of fifteen tickers, and the period is exact by construction

A static enumeration of every `0x2bf20` in `.text` (agent pass, 2026-09-07; its instrument
controls, its one byte-scan false positive at `0x141b5d7a1`, and its own mid-pass correction of
a wrong free entry point are all in its report) found **fifteen near-identical functions in
`0x140c93530..0x140c95095`** sharing one template. `FUN_140c93930`, re-read here rather than
taken on trust: `[L]`

```text
140c93938  cmp   dword [rip+LAST], 0
140c9393f  jne   +0xc
140c93945  mov   [rip+LAST], eax        ; FIRST CALL ONLY: seed
140c9394b  mov   r8d, now
140c93950  mov   edx, 0x2bf20           ; 180 000
140c93955  mov   ecx, dword [rip+LAST]
140c9395b  call  0x1408fcaa0            ; (now - LAST) > 180000 ?
140c93965  je    end
140c9396f  mov   [rip+LAST], eax        ; TRUE BRANCH RE-ARMS  <-- exact period
```

**The re-arm on the firing branch is the whole thing.** A gate that reloads its own timestamp
from the firing instant produces exactly 180 000 ms between firings rather than a drifting
cadence, which is what 180.002 / 180.115 / 180.020 / 180.038 is. `[D]` And the seed is on
*first call*, not process start, which is why §6's phase origins differ between sessions.

**Seven are called directly from the per-frame game-stage tick `FUN_142ce0130`.** Six of the
fifteen call `FUN_140ca61d0(out, n)`, re-read here: `[L]`

```text
140ca61f2  lea rdx, [rdi*4 + 8]        ; size = n*4 + 8
140ca61fa  call 0x14019b780            ; the pooled allocator
140ca6204  add rax, 8
140ca620d  mov qword ptr [rax - 8], rdi ; element count below the returned pointer
```

`callers.py` on it returns exactly the six tickers plus `FUN_141b2c7c0`. `[L]` **Two of them
pass `n = 6`, which is `6*4+8 = 32` bytes - the `0x20` class, the only class that has ever been
damaged**, allocated and freed on every firing. `[D]`

That is the strongest candidate the investigation has: right clock, right period mechanism,
right size class, on the frame tick, allocating and freeing into `0x143AD68A0` itself. It is
**not** established that one of them writes the damage - four of the seven frame-tick tickers
jump straight into `.themida` (rawsize 0, unreadable), and every one has inner feature gates
that can be read but not evaluated statically.

**It also closes out §5 below.** `fieldrefs.py` over the whole image returns exactly four
references to `[ctx+0x39ec]`: three writers, one reader, and the reader is `FUN_142e0f9e0`'s own
comparison. Nothing on its elapsed-true path re-arms the field - that path stamps `[+0x3b18]`
and returns. **A branch that cannot re-arm its own gate cannot produce a 180 s period**, which
is a cleaner reason to drop that lead than the "it read zero in the dumps" one below, and it
agrees with it. `[D]`

## 4b. Is the server sending bad data? No, and here is what the client itself says

The owner, 2026-09-07: *"Isn't the server just sending the client bad data which cause the bad heap
lookup?"*

The honest answer has two halves and they point different ways.

**Not as a packet, and the client agrees.** `0x009E` is the client's own *"I could not handle
this packet"* report, and it carries the offending opcode and body verbatim - a gift this
project has used before. Across the archive it fires **34** times, so the instrument works and
has a positive control. In the three sessions that produced catches it fires **zero** times.
`[L]` The client is not refusing anything we send while this is happening.

And it is a **write**, not a lookup. A misread does not corrupt; something stores four bytes
into a slot header. That store is on a clock the client generates itself (§5a), with no packet,
no exception and no socket event within a second of it (§4), and traffic volume does not track
damage - 36 231 mob-move acks against five damaged slots in the run measured here. `[L]`

**But the other half is open, and it is the half the owner is really pointing at.** This family has
only ever been observed in *our* environment; there is no unhooked or non-MapleCW run anywhere
in the archive (`research/is-the-corruption-ours-2026-09-06.md` §1). And this client demonstrably
*does* have a memory-corruption path that only our environment reaches: the reachability check
that overruns its own stack buffer when nothing is reachable, which is why `1415db360:ret` is
not optional. So *"our setup puts the client somewhere it was never meant to be"* is very much
live. It is just not *"a malformed packet parsed wrongly"*.

### 4b.1 A new lead: the client writes an error record on this clock

`0x008F CLIENT_ELOG` is the client's own error log, uploaded at the **start** of a session and
describing the **previous** one. Decoded with `tools/decode_elog.py`, two of them land on the
180-second clock: `[L]`

| uploaded in | record | its `Time2` against that session's catches |
|---|---|---|
| the 23:27 run | `ELog|10 ... DATETIME 02:34:33 ... Time1 173293072 Time2 173292058` | catch #1 of the sentry session was 22:34:34.376; the dump 725 ms later read `[ctx+0x3b18] = 173292538`, so `Time2` is ~250 ms **after** that catch |
| the run in progress | `ELog|10 ... DATETIME 03:56:42 ... Time1 178222699 Time2 178222162` | the four-catch session's catch-1 dump read `178402192`; `Time2` is **180 030 ms** before it - one period, to 30 ms |

Type 10 carries `State|3`, `AccountId|0` and a trailing `767`, and other sessions show type 15
and 18 records that are unmistakable errors (`throw ZException|-|1330|HR|38`). `[L]`

**Two observations, and this file has already burned two correlations tonight** - the census and
the banner - so it is tagged `[D]` and no further. What makes it worth chasing anyway is that it
is the client *saying something happened*, at a moment we can predict to the second.

### 4b.2 The prediction was run, and it survived - just

The 26-minute repair session was closed and the next login uploaded its record: `[L]`

```text
ELog|10|VERSION|100|DATETIME|2026/09/07 04:43:51|FID|10001010|State|3
       |Time1|181051781|Time2|181051206|NAME|Cobalt|Socket|127.0.0.1:8485|0|767|AccountId|0|
```

**The anchor, measured rather than assumed.** `[ctx+0x4088]` advances by exactly `180180` between
each of that session's four dumps, which are 180 s apart, and `[ctx+0x3b18]` advances `180090`
between dumps 3 and 4 - so both are tick-derived and `0x3b18` is current at those two. `[L]`
Taking `tick 181591326 ↔ 00:52:53` and `181771416 ↔ 00:55:53` (they agree to 0.1 s):

| | wall time | vs the firing before catch #1 (00:43:52.63, extrapolated back from the seven measured catches) |
|---|---|---|
| `Time2` | 00:43:52.85 | **0.22 s after** |
| `Time1` | 00:43:53.42 | 0.79 s after |

Re-anchoring the earlier session the same way puts its `Time2` **0.67 s after** its own
extrapolated firing. `[D]`

So: two sessions, and in both the record lands **within about 0.7 s of a firing of the clock**.
That is as tight as the anchor allows and it is not tight enough to call. `DATETIME` disagrees
with `Time1` by about 2 s in both sessions, consistently, so it has a different origin and is no
use for this.

**And the firing it lands on damaged nothing.** The sentry was armed at 00:40:44 and walking; it
reported no finding at 00:43:52. So if this is the same clock, the timer fires, sometimes writes
an error record, and only sometimes corrupts a slot - which is a different and more interesting
claim than "the error is the corruption". `[D]`

**Where the static chain stops.** The two functions that reference the record's `AccountId` field
name - `FUN_1415dd920` and `FUN_142d17160` - have **zero** direct callers, tail jumps or pointers
between them. `[L]` Per `tools/callers.py`'s own warning that is *indirect*, not *unreachable*:
a register, a vtable slot or the Themida VM. Naming what raises a type-10 record needs something
other than a call scan.

### 4b.3 The allocator watch: armed, consumed, and inconclusive

The launcher took the pin (`probe: watching 0x140ca61d0 (slot 2)`), so that mechanism works.
`[L]` But the session was an enter-and-exit, and the watch logged **two** hits, both at
`01:09:53.942` while dispatching `0x0010 LOGIN_RESULT`, both `rdx=2`, called from
`0x141b2c96c` / `0x141b2c9ac` - inside `FUN_141b2c7c0`, the **channel-list filler**
(`research/channel-select.md` §9.4), not the ticker family, which passes `6`. `[L]`

Nothing was in the world long enough for a 180-second ticker to fire, so §5a's Tier 1 is
**untested, not refuted**. It needs the same pin and a fifteen-minute idle run.

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
