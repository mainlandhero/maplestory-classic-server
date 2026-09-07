# Naming the writer: a write watch on a predicted firing

The owner, 2026-09-07: *"build whatever it takes to fix this crash, as long as you are confident
that we are not causing this crash."*

**I am not confident, and this file says exactly how far the evidence goes.** It also builds
the instrument, because the same instrument settles both questions at once: it names the
instruction that damages the header, and the module that instruction lives in is the answer to
*"is this ours"*. Nothing else in this project can produce that.

Tags: **[L]** read out of a file, **[D]** derived, **[I]** inferred.

---

## 1. What is actually established about "is it ours"

| claim | tag |
|---|---|
| Our hook has **no 180-second cadence of any kind**. Its periodic work is at 100 ms, 200 ms, 5 s, 30 s, 60 s and 90 s | `[L]` |
| Every write this hook makes into client memory is a **fixed-address byte patch** (`int3`, a 12-byte absolute jmp, a session field at a known offset), one refcount at a block `identity.rs` allocated itself, or the sentry repair - which writes **zero**, not `1` | `[L]` |
| The 180 s period is generated **inside the client**: a template that re-arms `LAST := now` on its own firing branch, read in the listing at `0x140c93930` | `[L]` |
| The client does not refuse anything we send while this happens: `0x009E` fires 34 times across the archive and **zero** times in the three catch sessions | `[L]` |
| Therefore **our code is not the instruction that writes the damage** | `[D]` |
| **But our environment has not been cleared.** 75 of 75 archived client runs carried this hook, so no comparison exists; and this client demonstrably *does* have a memory-corruption path that only our network environment reaches - the reachability check that overruns its own stack buffer, which is why `1415db360:ret` is not optional | `[L]`, `research/is-the-corruption-ours-2026-09-06.md` §1, §3.1 |

So the honest position is: **we are not writing it, and we have never ruled out putting the
client somewhere it writes it.** Those are different claims and only the first is measured.

The enumeration behind row 2 is worth one line each, because "the hook writes to the client"
is true and vague, and vague is what lets a suspicion survive:

* `hook.rs`, `identity.rs`, `instance.rs`, `probe.rs`, `netwatch.rs` write **code bytes at
  addresses fixed at build time** - trampolines and `0xCC`. None is an address computed from
  client data.
* `session.rs` writes three named fields of the session object at fixed offsets.
* `identity.rs::lay_out_string` writes a refcount of `1` into a pool block - **at `body + 0`,
  once per session, into a block it allocated and sized to the `0x40` class**. The family is
  four bytes at `body − 4`, in the `0x20` class, every 180 s. Different offset, different
  class, different cardinality, and eleven days younger than the first death. `[L]`
* `poolsentry.rs::repair_header` writes `0` into a header high dword that was already
  non-zero, and reads it back. It cannot manufacture a `1`.

## 2. The instrument, and why it is not the build that was written down

`heap-corruption-2026-09-06.md` §3.2 proposed replacing bucket 1's allocator so every freed
slot became its own decommitted page. That was the right idea before 2026-09-07 and it is the
wrong build now, for three reasons that are measurements rather than preferences:

1. **Two of the four catches were LIVE slots.** `the-180-second-clock-2026-09-07.md` §2, and
   §5.2 of the earlier file: *"the writer holds a pointer of its own and writes through it
   regardless of the slot's state."* `[L]` A decommit-on-free scheme is blind to half the
   events by construction.
2. **The damage is at `body − 4`, inside the pool header**, and the pool must keep writing
   that header to work. Page protection is page-granular, so no layout separates the
   allocator's own carve from the writer while the slot is live. `[D]`
3. It replaces the client's hottest allocator with an inline hook, in a client whose packer
   is built to notice exactly that.

What makes something cheaper possible is the finding of 2026-09-07: **the period is exact and
it re-arms on firing**, so after one catch the next write is predictable to a few hundred
milliseconds. Protecting the size class for a whole session is unusable. Protecting it for
1.2 s every three minutes is a stutter.

`crates/grap-stub/src/writewatch.rs`:

* the sentry hands over bucket 1's chunk bases `write_lead` (500 ms) before a predicted firing;
* every page **fully contained** inside a chunk span goes to `PAGE_READONLY` - partial pages at
  the ends are dropped, so no unrelated allocation is put behind a fault;
* reads are untouched, so the sentry's walk and the client's string reads carry on unchanged;
* a **write** faults. The vectored handler classifies the address against the chunk table -
  slot header, slot body, or chunk bookkeeping - records RIP, the exact address, the slot and
  the offset inside the header, makes the page writable and returns
  `EXCEPTION_CONTINUE_EXECUTION`, so **the instruction re-executes and the write lands**;
* pages go blind once touched, so the armer re-protects every 5 ms;
* after 1.2 s every run goes back to `PAGE_READWRITE`.

**It writes nothing to the client.** `VirtualProtect` is a permission change, not an edit. The
only module that writes a client byte is still the repair, still behind `repair=on`.

### 2.1 What the handler may not do, and why it is written that way

It runs on a client thread that may hold the pool's spinlock or the CRT heap lock. It
allocates nothing, takes no lock, opens no file and calls no logging - it writes into fixed
static rings with atomics and returns. `hook::log` formats a `String` and opens a file;
calling it from there is how this module would hang the client at the exact moment it finally
had the answer. The **dump** is for the same reason deferred: the handler asks, and the sentry
thread - which holds nothing - writes it milliseconds later.

### 2.2 The instrument's own control, and it runs in `cargo test`

Three facts this module rests on come out of `winnt.h` and nowhere else:
`EXCEPTION_RECORD.NumberParameters` at `0x18`, `ExceptionInformation` at `0x20`, and
`ExceptionInformation[0] == 1` for a write. `minidump.rs` is this repo's standing lesson that a
layout read from a header can be wrong with no error at all - every `MINIDUMP_*` struct is
inside `<pshpack4.h>`, and the obvious 24/8 reading returned `ERROR_NOACCESS` on every call for
a day, beside a passing test that asserted the same wrong number.

If any of the three were wrong here the failure would be **silence**, which reads exactly like
"the writer did not fire". So there are two controls, both in `cargo test`, both on the real
operating system:

* `the_handler_reads_a_real_exception_record_correctly` - allocate a page, protect it, store to
  a known offset, and require the handler to report **that exact address**, with kind `1`, and
  the store to have landed. `arm_for` runs the same check once before its first window and
  **stands the module down** if it fails, rather than arming something that can only report
  nothing.
* `it_catches_the_family_write_names_it_and_lets_it_through` - eleven chunks laid out as a
  16 KB segment holds them, the window armed, then **the family's own write**: four bytes of
  `1` at `header + 4`. It asserts the header afterwards reads `0x0000000100000020`, that it was
  classified as slot 3's header at offset 4 with a non-zero RIP, that a body write beside it
  was classified as a body and not reported, and that the pages are ordinary writable memory
  again when the window closes.

The second is the decoy-named-`MapleStory.exe` standard: the instrument is shown catching the
exact thing it is being pointed at, before it is pointed at the client.

### 2.3 The knobs, and the one that exists because of a number nobody has

`write=on,writelead=500,writewindow=1200,writeburst=5` in the sentry marker;
`tools\test-server.ps1 -SentryWriteWatch` writes it and implies `-PoolSentry` (the flag alone
would arm nothing and look identical to a run that armed and saw nothing).

A window's cost is bounded by `pages x sweeps`, not by the client's write rate, because a page
stays writable until the next sweep. **That bound has never been measured against a live
client.** So a window that handles 20 000 faults stops re-protecting and says so in the
heartbeat. Backing off is itself a result - it would say the `0x20` class is written far harder
than assumed - and it is a much better outcome than finding the number by freezing the owner's
client for a minute.

## 3. What the run says, and what it cannot say

`client-patched\maplecw-hook.log`, in order of value:

| line | reading |
|---|---|
| `THE WRITER: a store to slot header X at +4 ... from RIP R` | **the answer.** `R` is the instruction. The module it sits in also settles §1's open half |
| `POOL WRITE WATCH saw a write into a watched page` | the liveness control: pages really are protected and faults really do reach us |
| `window #N open ... control PASS` | armed, self-test passed |
| `control FAIL` | the module stood itself down instead of reporting silence |
| windows opened, **zero** write faults | a property of the instrument, **not** evidence the client did not write. Report it that way |
| no window at all | no catch happened, so there was never a phase to predict from |

Three things it cannot do, stated now rather than after the run:

1. **A late firing falls outside the window.** Six of seven measured intervals sit within
   0.07 s of 180.000; one was **1.019 s late**, and the phase stayed shifted afterwards. The
   window covers −500 ms to +700 ms, so a firing like that one is missed. It costs one cycle:
   the sentry still catches the damage, which re-anchors the phase. `[D]`
2. **It names an instruction, not an object.** If the store turns out to be inside a shared
   `memcpy`, RIP is a CRT address and the caller is the next question - answered by
   `probe.rs`'s existing `watch@<rip>`, which already prints registers and a stack.
3. **It does not stop the writer.** Naming it is what makes stopping it possible: this hook
   already neutralises one client function by address (`1415db360:ret`), so a named store is a
   candidate for the same treatment - and *that* is the fix, not this.

## 4. What this does not change

The repair still ships and still earns its place: the live pool in the field crash was clean,
0 damaged of 193 632. The second lethal surface - the WZ property/VARIANT teardown freeing a
pool chunk as an NT-heap block during a map change - is untouched by this module and by the
repair, and `research/second-crash-family-2026-09-07.md` is unchanged. A mitigation for that
surface is deliberately **not** built here: its mechanism from corrupted header to interior
free target is `[I]`, not traced, and building a blind guard for a once-observed path before
the writer is named is the wrong order.
