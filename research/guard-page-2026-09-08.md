# The guard-page quarantine: naming the writer on the surface the window watch cannot reach

The owner, 2026-09-08: *"Build the guard-page allocator and also the write watch."*

`crates/grap-stub/src/guardpage.rs`, armed by `-GuardPage`. No client run has exercised it yet;
this file is the design and the argument for it. Tags: **[L]** read off this client's data or
listing, **[D]** derived, **[I]** inferred.

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

While armed for a chosen size class (default `0x40`):

* **Allocation** of that class is served from its own **one-slot page** inside a private 2 GB
  reservation, not from the pool.
* **Free** of that slot **decommits the page and retires the address** — the bump cursor only
  ever advances.
* A genuine allocation that outlives its free is invisible: the client stops touching a slot it
  freed. A **stale** pointer is not — the next access through it hits a decommitted page and
  faults. The handler logs the faulting **RIP**, the target address, the return address that
  **allocated** the slot and the one that **freed** it, then recommits the page so the client
  runs on to the next one.

The 2 GB reserve is 512 K one-page slots. It is address space only; committed memory is one 4 KB
page per *live* slot, and a decommitted page costs nothing but its address. Bucket 2 served
18 758 allocations in a 50-minute run (`dumps/…-358616…`), so the reserve is far more than a
long session needs; if the cursor exhausts it, allocation falls back to the client's own and the
heartbeat says so.

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
* **Not yet on a client.** Compile and 95 unit tests only. The launch is the test.
