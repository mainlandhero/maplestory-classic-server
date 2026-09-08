# The guard-page quarantine: naming the writer on the surface the window watch cannot reach

The owner, 2026-09-08: *"Build the guard-page allocator and also the write watch."*

> **PARTLY SUPERSEDED, 2026-09-08 evening. Read this for the design and the argument; do NOT
> take its numbers.** Four things here are now wrong, and each is corrected in place below:
> the default class, the reserve size, the slot count, and "not yet on a client". The current
> state is in `research/the-writer-damages-live-objects-2026-09-08.md` (the overnight death, and
> why a clean pool proves nothing) and in `STATUS.md`'s entries for the 12:01 and 14:47 runs.
> **It has run on a client since**: armed, `control PASS`, 1 h 57 m, and at the time of writing a
> second run is past 1 h 20 m with zero damage.

`crates/grap-stub/src/guardpage.rs`, armed by `-GuardPage`. This file is the design and the
argument for it. Tags: **[L]** read off this client's data or listing, **[D]** derived,
**[I]** inferred.

---

## 0. Why, in one paragraph

Five write-watch runs (`research/the-180-second-clock-2026-09-07.md`) show one writer with one
habit: it holds a pointer into pool memory that has been freed and handed back out, and writes a
small increment through it on a timer. That reading is the only one consistent with the same
damage landing on a **freed `0x20` slot** (the header, `body−4`, → the `0xC0000374` pooled
free), a **live `0x40` map node's own pointer** (run 2, §8), and a **pointer to a `0x40` node**
(run 5, §11). The write watch protects the `0x20` chunk pages read-only around a predicted
firing and catches the first. It cannot catch the other two: wrong class, and run 5 died at 3.5
minutes before any window opened. What catches an *arbitrary* stale write, on any clock, is
memory that is **never handed out twice**.

## 1. The mechanism

While armed for a chosen size class (**default since 2026-09-08 evening: `0x20+0x40`, a SET;
this file was written when it was one class, `0x40`**):

* **Allocation** of that class is served from its own **one-slot page** inside a private 2 GB
  reservation, not from the pool.
* **Free** of that slot **decommits the page and retires the address** — the bump cursor only
  ever advances.
* A genuine allocation that outlives its free is invisible: the client stops touching a slot it
  freed. A **stale** pointer is not — the next access through it hits a decommitted page and
  faults. The handler logs the faulting **RIP**, the target address, the return address that
  **allocated** the slot and the one that **freed** it, then recommits the page so the client
  runs on to the next one.

The reserve is address space only; committed memory is one 4 KB page per *live* slot, and a
decommitted page costs nothing but its address.

> **The numbers in this paragraph were 2 GB and 512 K slots and are now 32 GB and 8 388 608.**
> The 12:01 run measured the churn this had to survive - a 627 172-allocation burst in the first
> minute and then 1 560/s - and the old cursor was spent at six minutes, four minutes before
> anything could age out. The binding constraint was never memory; live slots held at ~26 000,
> about 104 MB. It was the 40-byte-a-slot metadata array, 320 MB if committed eagerly, which is
> why the reserve could not grow until that array was made lazy.

**A retired address comes back after 600 s** (`REUSE_AFTER_MS`), and §7 is why: without that the
reserve is a budget of *total* allocations rather than a working set, and it does not last a
night. The delay is three firings of the writer's own 180 s clock, so a stale pointer taken at
the moment of a free still faults for three periods after it. Beyond the delay, a page comes
back and the exposure returns to what the client's own pool has after a few milliseconds.

## 2. Why this is safe on the client's hottest subsystem

`heap-corruption-2026-09-06.md` §3.2 proposed replacing the allocator. This does much less, and
the two decisions are the reason:

* **Alloc is the only inline hook**, through [`crate::identity::install_detour`], which refuses
  to patch unless `FUN_14019b780`'s first fifteen bytes are the three position-independent
  `mov [rsp+disp],reg` this was built against (`research/heap-wild-write.md` §1). A mismatch
  stands the module down and reverts.
* **Free is a pointer swap, not a code patch.** A quarantined slot's header is stamped `0x100`
  (`> 0x80`), so the client's own free ladders past the four buckets and takes its `HeapFree`
  arm — keeping our slots entirely off the pool's free list and live counter. We intercept by
  swapping the cached `HeapFree` pointer at `0x143ad5530`, exactly as `crate::freeguard` swaps
  PCOM's, and pass every non-quarantine pointer straight through. `[L]` for the ladder and the
  slot (`heapfix-did-not-hold.md` §1).

So our slots never appear on the pool's chunk chain, free list or counters: `poolsentry` and
`tools/poolchain.py` keep walking the real pool unchanged, and the write watch keeps its `0x20`
pages. The two instruments cover disjoint memory and cooperate in one launch — the intended
pairing: `-SentryWriteWatch` on `0x20`, `-GuardPage` on `0x40`.

## 3. The control that runs before it arms

`CLAUDE.md`: an instrument that has never fired is the kind this project distrusts. `self_test`
reserves one page of its own, decommits it, writes to it, and requires the handler to have
caught that write **at that address** and recommitted it so the store lands. It runs before the
hooks go in; a failure logs `control FAIL` and arms nothing. This is the guard-page sibling of
`writewatch`'s self-test and `minidump`'s decoy.

The pure core — the size-class ladder, the reserve range check, the `body−8` free pointer
arithmetic, the header value that selects the `HeapFree` arm — is unit-tested (5 tests) against
the numbers the client's own code uses.

## 4. What a catch will and will not say

**Will:** the instruction that touches freed memory (RIP + module), the exact address, and —
from the per-page metadata kept outside the guard page so it survives the decommit — the call
sites that allocated and freed the slot. For the `0x40` map-node surface that is the answer runs
2 and 5 could not give: *who writes the +2, and into whose object.*

**Will not:** it catches the access that *uses* freed memory. If the writer's target field is
itself in *live* memory and only the pointer it holds is stale, the guard catches the
downstream dereference into the freed page (as run 5's fatal read was downstream of the +2
write), which still names the instruction and the object. And it only quarantines one class per
run; `0x40` is the default because it is uncovered and low-traffic, `-GuardBucket` picks another.

## 5. Limits, stated

* **It writes to the client** — an allocator inline hook and a `HeapFree` swap — so it is off
  by default and, unlike the read-only write watch, is a patch whose absence keeps the client
  as it was. It reverts cleanly if the prologue check or self-test fails.
* **A kernel write into a decommitted page** returns an error to the client rather than
  faulting into our handler. `0x40` slots are small objects and map nodes, not I/O buffers, so
  this is unlikely, but it is the one path where the quarantine changes behaviour instead of
  observing it.
* ~~**Not yet on a client.** Compile and 97 unit tests only. The launch is the test.~~
  **It has run since (2026-09-08 12:01):** armed, `control PASS`, and the client ran 1 h 57 m -
  the longest session this project has had. An inline hook on the pool allocator, called from
  thirty threads thousands of times a second, does not destabilise the client. That question is
  closed. What was still first-launch code afterwards is multi-class serving, the lazy metadata
  commit, the reserve ladder and the first-free control.

## 6. The review before the first launch (2026-09-08)

The owner: *"Can you review what was written and make sure you agree?"* Read adversarially against
the listing rather than re-described. Four things were checked off the binary and three defects
were found in the code; all three are fixed in the same commit as this section.

### 6.1 Confirmed off the listing **[L]**

| claim the build rests on | what the listing says |
|---|---|
| both pool frees load `HeapFree` from `0x143ad5530` | `FUN_14019b4e0` at `0x14019b577`, `FUN_14019bb50` at `0x14019bbdb` - both `mov rbx,[rip+..]` of that slot. The slot has **60** readers in `.text`, so the shim sits on every large free in the client; every one but ours is a pass-through |
| a `0x100` header takes the `HeapFree` arm | `cmp rax,0x20 / ja; cmp rax,0x40 / ja; cmp rax,0x80; ecx=-1; cmovbe ecx,3; test ecx,ecx; jns pooled` - `0x100` leaves `ecx=-1` and falls to `call rbx` |
| `HeapFree` receives `body-8` | `call GetProcessHeap; lea r8,[rdi-8]; xor edx,edx; mov rcx,rax; call rbx` at `0x14019bbe2..bbf1`. Nothing else in the free touches a counter before that arm |
| the 15 stolen bytes are the whole prologue and the trampoline's `mov rax,imm; jmp rax` clobbers nothing live | `mov [rsp+8],rbx / [rsp+18],rbp / [rsp+20],rsi` is 15 bytes; the next instructions are `push rdi/r14/r15; sub rsp,0x20; mov rax,rdx` - `rax` is written before it is read, and `r8` is `xor`ed before it is read |
| the size ladder | `cmp rdx,0x20 / ja; cmp rax,0x10; seta` → classes `0..=0x10`, `..=0x20`, `..=0x40`, `..=0x80` (`rsi=0x80`), else the large path. Matches `size_class` |

### 6.2 Defect 1: the detour could run before its trampoline existed

`identity::install_detour` wrote the live jump, restored the page, read it back and *returned*;
the caller then stored the trampoline. Between the write and that store, `alloc_detour` entered
by any thread loaded a zero and jumped to it. For the identity getter, called once at login,
the gap was academic. For the pool allocator - entered from thirty threads, thousands of times a
second - it was a crash at RIP 0 a few microseconds into arming, on the launch meant to show the
instrument does not disturb the client. And it would have been unattributable: a fault at
address 0 with a return address in `grap64.dll`.

Fixed in `install_detour` for both users: the trampoline is stored into a caller-supplied
`AtomicUsize` **before** the jump is written. While it is written, every other thread is
suspended and checked to be outside `[target, target+15)` (a 12-byte jump over a live prologue
is not atomic; a thread resuming at byte 5 would execute the middle of the immediate); nothing
allocates or logs while they are parked, the rule `poolsentry::thread_snapshot` already keeps.
Residue, stated: a thread created between the snapshot and the suspend is not parked. The
identity self-test now asserts the publish-before-jump ordering; a refused install publishes
nothing.

### 6.3 Defect 2: "freed from" would always have named the pool

`caller_ra` accepted any value in the 128 MB image and took the first one up the stack. From
`heapfree_shim` that is always `0x14019bbf3` - the return into the free that called
`HeapFree` - so every catch line would have read *freed from the free function*. From
`alloc_detour` the pool context `0x143ad68a0`, a `.data` address sitting in the allocator's home
slot, qualified too. Now: only the image's executable sections (`.text`, `.themida`, `.boot`,
read from the mapped PE header at arm) and never `[0x14019b4e0, 0x14019bc60)`, the pool's own
code. Both frees are `push rdi; sub rsp,0x20` frames [L], so the caller worth naming is one
frame up; the scan window covers it. Tested against the client's own section table.

### 6.4 Defect 3: `probe.rs`'s handler would have seen every catch first

The probe's vectored handler is registered when the first `watch@` arms, seconds after this
module's. A first-chance handler registered later runs first. Its non-watch branch logs a
`CLIENT FAULT` and writes a crash dump - per catch, before this module's handler recommitted the
page. `writewatch` already had a `suppresses()` hand-off there for exactly this reason;
`guardpage::suppresses()` now sits beside it. The write watch's own handler, also registered
later, returns `CONTINUE_SEARCH` for any target outside its runs, so it needed nothing.

### 6.5 The fix's own defect: an API call inside the parked window

Asked *"has everything been fixed?"*, the answer was no, and the thing that was not fixed was
in §6.2's fix. It parked the threads and **then** called `VirtualProtect` to make the prologue
writable. `VirtualProtect` takes the process address-space lock, which a thread suspended
mid-`VirtualAlloc` is holding - and the pool allocator is a `VirtualAlloc` caller. That is the
instrument deadlocking the client it exists to observe, at arm time, and it would have looked
like the client hanging on launch with no fault and no log line.

`poolsentry::thread_snapshot` already carries the rule in its doc block - *"nothing at all
happens between the suspend and the resume - no allocation, no `VirtualQuery`, no
formatting"* - written after this project had already paid for it once. It was cited in the
same patch that broke it. Both `VirtualProtect` calls are now outside the parked window: make
the page writable, park, write twelve bytes and the `nop` tail, unpark, restore. Nothing but
stores between the suspend and the resume. The page is executable-writable for a few
microseconds longer, which is the right side of that trade.

The general form, for the next instrument: **a rule quoted in a patch is not a rule the patch
follows.** `CLAUDE.md` says a comment describing a guarantee is not the guarantee; a citation
is not either.

### 6.6 Still true after the review

It has not run on a client. The three defects above are the kind that only a run or a review
finds, and the review found them; what a run will add is whether Themida objects to the
allocator's first fifteen bytes changing (the prologue check and the `identity:` line will say)
and whether a `0x40` class served from guard pages changes the client's behaviour in any way the
write watch or the sentry can see.

---

## 7. Overnight: the reserve was a nine-minute budget, and a wrong number said otherwise

The owner, 2026-09-08: *"Our goal is to leave the client running overnight without it exiting."*

That is a different goal from naming the writer, and checking the build against it found the
module could not have lasted the night.

### 7.1 The wrong number

§1 said *"bucket 2 served 18 758 allocations in a 50-minute run, so the reserve is far more than
a long session needs."* **That is a live-object count, not a rate.** The pool's counter at
`ctx + i*4 + 0x14` is `inc`remented at `0x14019b8ca` on allocate and `dec`remented at
`0x14019bc36` on free [L] - it is how many `0x40` objects were alive at the instant of the dump.
Cumulative allocations over those 50 minutes are **unmeasured**, and every map change, mob spawn
and despawn churns this class.

`tools/poolchain.py` prints the field as `allocations served`, and its own header calls it
*"allocations served / objects live"* - an ambiguity read the wrong way, then written into the
design's justification as a fact. `CLAUDE.md`'s rule about units, one more time: **the field's
meaning came from a label, and the label was not checked against the listing.**

### 7.2 What that meant

The cursor only ever advanced, so the reserve was a budget of 512 K total allocations of the
class. At a thousand a second - a rate nobody has measured but which is unremarkable for map
nodes - that is **under nine minutes**, after which every allocation falls back to the client's
own pool and the quarantine covers nothing. Against an eight-hour goal, and with no measurement
of the rate either way, that is not a risk worth taking on a run that costs the owner a night.

The failure mode is the one this project keeps meeting: it is **silent**. A fallen-back run
looks exactly like a protected one, right up to the death it was meant to prevent.

### 7.3 The fix

A retirement queue. Freed slots are pushed in free order; an allocation takes the head **only if
it was freed more than `REUSE_AFTER_MS` (600 s) ago**, and otherwise takes a fresh page from the
cursor. The queue is in free order, so one look at the head decides it. What must now fit in the
reserve is not every allocation of the night but the allocations made **during one 600-second
window** - about 800 a second sustained. The delay is three periods of the 180 s clock.

The trade is stated rather than hidden: an absolute guarantee that lasted nine minutes becomes a
600-second guarantee that lasts as long as the client runs. A stale pointer older than ten
minutes writes into a live quarantined slot, which is the exposure the client's own pool has
after milliseconds.

And the exhaustion is now **loud**: the heartbeat prints
`***** N FELL BACK - the class is NO LONGER COVERED *****` rather than a quiet `N fell back`,
because that counter above zero is the difference between a protected night and an unprotected
one, and nothing else on screen would say which happened.

### 7.4 The overnight run is not the measuring run

`-SentryWriteWatch` **observes**; it protects nothing. Overnight it is ~160 windows of read-only
pool pages and single-stepped writes, up to 20 000 faults each. The survival recipe drops it and
takes `-SentryQuiet` (no dumps, no 68-thread stack scan, a 2 s walk except near a predicted
firing), which keeps the repair. Test plan item T20.

Both death surfaces are then covered by *prevention*, not observation:

| surface | mechanism | evidence it works |
|---|---|---|
| `0x20` pool header | the sentry repairs the header before the free that would be fatal | runs 3 and 4 ended with a clean pool |
| `0x40` map node | the address is not handed back, so the increment lands on a dead page instead of a live node | **none - this is its first launch** |

The `0x40` row is the honest one. Run 2's death was the writer incrementing an *empty map's head
node* through an address the pool had recycled into it; the quarantine breaks that by refusing to
recycle. That is an argument, not a measurement.

### 7.5 What is still unknown

No run has passed 70 minutes. Eight hours is a long extrapolation from a short measurement, and
nothing in this file rules out a cause that first appears at hour three - including the two
close-time faults (`0x14094e190`, `0x141d12df0`) that were recorded and never chased. The
allocation rate of the `0x40` class is still unmeasured; the first night's `recycled` and
`fell back` counters will measure it.
