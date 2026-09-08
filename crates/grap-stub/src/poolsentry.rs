//! A live watch on the client's pool allocator, so the `0x0000000100000020` write is caught
//! **while it is fresh** instead of tens of thousands of allocations later.
//!
//! Opt-in via the [`SENTRY_MARKER`] file. Absent, this module does not read one byte of the
//! client's memory and does not start a thread.
//!
//! # Why a live sentry and not more post-mortem work
//!
//! Seven `0xC0000374` dumps have been walked. The damage is always the identical qword
//! `0x0000000100000020` in the size header of a `0x20`-class pool slot — **14 of 14** across
//! the family, 9 damaged in 537 696 bucket-1 slots against **0 damaged in 589 240 slots
//! outside bucket 1** (`research/heap-wild-write.md`, `research/heap-third-dump.md`,
//! `research/heap-corruption-2026-08-27.md`, `research/heapfix-did-not-hold.md`).
//!
//! `research/heap-wild-write.md` §6 then closes the post-mortem route in one sentence: the
//! header is written **once, at carve, and never again**, so a slot may have been damaged
//! tens of thousands of allocations before whatever occupies it in the dump. It calls the
//! contents "a dead end **by construction**". Nine damaged slots have produced nine unrelated
//! strings.
//!
//! A detector that notices within ~100 ms makes the contents **~100 ms old**, and that is not
//! a nicety — it is the discriminator between the two surviving hypotheses:
//!
//! * a **freshly-allocated** occupant supports the refcount/flag reading — *a 4-byte field
//!   written at `p−4` by code handed a raw pool body where it expected a Ztl/BSTR payload
//!   pointer*, which is where `0x1401be120` freeing two pointers out of one object with two
//!   different conventions points;
//! * a **long-lived** occupant, or a live **predecessor**, supports the 4-byte overrun at
//!   `+0x24` of an object sized to the `0x28` stride rather than the `0x20` slot.
//!
//! Two more things a post-mortem cannot give and this can:
//!
//! * **whether the predecessor slot was live at the moment of damage.**
//!   `research/heap-corruption-2026-08-27.md` §6.7 lists that as unanswerable from a dump.
//! * **an unbiased sample.** Every rate in the project comes from five *deaths*, and a death
//!   requires a damaged slot to have been freed. `heap-third-dump.md` §6.2 and
//!   `heap-corruption-2026-08-27.md` §6.3 have both asked for a sample that does not require
//!   one. The heartbeat below is that sample, taken every 60 s, at no launch cost.
//!
//! # What this is NOT
//!
//! It is not a fix and it does not touch a byte of the client. It reads. `-HeapFix` is a
//! different thing, it is off, and `research/heapfix-did-not-hold.md` §6 says leave it off —
//! it patches one of three entry points and dump 319924 is a third death at the unpatched
//! `0x14019bb50`. Nothing here revives it.
//!
//! # The expected hit rate, and why the log is quiet
//!
//! `research/heap-crash-pattern.md` §7 fits the five dumps' damaged-slot counts at **one slot
//! per ~298 s of in-field time** (CV 0.19, against 0.34 on process time). At
//! [`DEFAULT_INTERVAL`] that is roughly **3 000 clean walks per catch**. So a walk logs
//! nothing; only the arm line, a heartbeat every [`DEFAULT_HEARTBEAT`], and a confirmed
//! finding reach the log. A ten-minute run that catches nothing is a *result* — the heartbeat
//! is what tells it apart from a detector that never ran, and it prints the instrument's own
//! controls (see below) so a silent walk cannot pass for a working one.
//!
//! # How it tolerates a live allocator
//!
//! Everything here runs against structures other threads are mutating, in a process a fault
//! would kill. Five properties, in the order they matter:
//!
//! 1. **No lock is taken, ever.** The per-bucket spinlock at `ctx + i*16 + 0x28` is *read*
//!    for the record and never acquired. Holding it across a millisecond-long walk would
//!    stall every allocation in the client's hottest size class and change the timing of the
//!    very thing being measured; and the acquire protocol (owner TEB + recursion count) is
//!    not decoded, so getting it wrong deadlocks the client. That is the worst outcome
//!    available and it is not worth the tidiness.
//! 2. **The header read cannot tear.** It is an 8-byte-aligned qword written by a single
//!    aligned 8-byte store (`mov [rax-8], rdi` in the carve, `heap-wild-write.md` §5), and an
//!    aligned 8-byte access on x86-64 is atomic. This is the same argument §5 uses to prove
//!    only four bytes were replaced, used in the other direction.
//! 3. **Every dereference is either inside memory this module has validated with
//!    `VirtualQuery`, or preceded by one.** A chunk is admitted only after
//!    `can_read(base − 8, chunk_bytes + 8)` *and* the allocator's own size identity
//!    `u64[base − 8] == per_chunk * (slot + 8) + 8`. The chunk-list link `[base]` and all its
//!    slot headers then live inside that validated span. A garbage link is caught by the next
//!    chunk's guard, not by a fault. Admitted chunks are re-validated on a rolling
//!    [`REVALIDATE_PERIOD`]-tick cycle so a decommit cannot sit undetected.
//! 4. **A race is a delay, never a fault and never a false positive.** The chunk list is
//!    push-front; a chunk whose link is not yet stored, or whose head has moved under the
//!    walk, ends that tick's discovery early and is picked up on the next one, 100 ms later.
//!    A candidate must survive [`CONFIRM_READS`] re-reads spaced [`CONFIRM_SPACING`] apart
//!    before it is reported, which no carve in flight can do — the carve loop writes 32
//!    headers in nanoseconds.
//! 5. **Nothing is allocated, and no API is called, while a thread is suspended.** The stack
//!    capture suspends **one** thread, calls `GetThreadContext`, and resumes it before doing
//!    anything else at all. Suspending a set of threads and then calling `VirtualQuery` or
//!    `format!` is how a tool of this shape deadlocks a process: the suspended thread may
//!    hold the address-space lock or the CRT heap lock.
//!
//! # False positives
//!
//! **No legitimately non-zero high dword has ever been observed on a slot header in this
//! pool**, and there are two independent reasons, one measured and one structural:
//!
//! * measured — across six dumps, **1 153 752 enumerated slots carried 10 non-`slot`
//!   headers, every one of them `0x0000000100000020`**, and each was corroborated as
//!   pathological by the failure record of the process that died holding it
//!   (`heapfix-did-not-hold.md` §3);
//! * structural — the carve writes the plain size with a full 8-byte store and **explicitly
//!   zeroes the high dword**, and nothing in the allocator ever rewrites the field
//!   (`heap-wild-write.md` §5.1, §5.2). There is therefore no legitimate *transient* either,
//!   which is what a snapshot instrument could otherwise have missed.
//!
//! The one moment a `0x20` cell legitimately holds something else is **before its carve loop
//! has reached it**, and that is handled twice over: the chunk is not on the chunk list yet,
//! and the confirm re-reads would not agree.
//!
//! This detector is deliberately **more** sensitive than `tools/poolchain.py`, which tests
//! only the high dword: anything that is not exactly the bucket's slot size is a candidate,
//! classified as `KNOWN FAMILY` or `NOVEL`. A NOVEL hit is the single most valuable outcome
//! available here — 14 of 14 identical is the entire basis for "the writer is selective", and
//! that claim should be able to come back false.
//!
//! It also walks **all four buckets** on a slower cycle rather than only bucket 1. 0 of
//! 693 272 slots outside the `0x20` class have ever been damaged, so a hit there would say
//! the writer targets an *address* rather than an object type — and `CLAUDE.md`'s rule is to
//! enumerate before filtering, not to search the list you already believe.
//!
//! # A fire in the first two minutes is more likely to be the instrument than the client
//!
//! `research/heap-crash-pattern.md` §4: **3 831 client-seconds of exposure in the 0–100 s
//! band and not one heap death**, earliest at 192 s. And §7's rate puts the expected damaged
//! count at 100 s of in-field play at about **0.34**. So a confirmed finding inside
//! [`EARLY_SUSPECT`] of arming is tagged `EARLY` and reported as *suspect*: it is still
//! snapshotted and still dumped, because suppressing it would be the silent-negative failure
//! this repo keeps paying for, but nobody should read it as a catch until the snapshot is
//! examined. It is a claim that can come back false in either direction.

use std::collections::HashSet;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{Duration, Instant};

use crate::hook::log;

/// Presence of this file, relative to the client's working directory, arms the sentry.
///
/// A marker file rather than an environment variable for the reason the rest of this crate
/// uses one: the client is launched with `ShellExecute` because it carries an elevation
/// manifest, and `ShellExecute` does not carry `$env:` into the child.
pub const SENTRY_MARKER: &str = "maplecw-hook.sentry";

/// `0x143AD68A0` — the pool context, from `lea rbp,[rip+0x393b366]` at `0x14019b533` inside
/// the client's pooled `free`. `research/heap-wild-write.md` §1.
pub(crate) const POOL_CTX_RVA: usize = 0x143AD68A0 - 0x140000000;

/// `ctx + i*4 + 0x04` — slots carved, an allocator counter this module uses as a control.
const OFF_CARVED: usize = 0x04;
/// `ctx + i*4 + 0x14` — objects live / allocations served.
const OFF_SERVED: usize = 0x14;
const COUNTER_STRIDE: usize = 4;

/// `ctx + i*16 + 0x28` — the per-bucket spinlock's owner word; `+0x30` is its recursion
/// count. **Sixteen** bytes per bucket, not eight.
///
/// `tools/poolchain.py` had `i*8` here until this pass. The listing says `i*16`
/// (`research/heapfix-did-not-hold.md` §1: *"rbx = ctx + i*16; take [rbx+0x28] spinlock"*),
/// and the surrounding layout settles it without needing the listing at all: the counters
/// end at `ctx+0x24` (`i*4 + 0x14` for `i = 3`, plus 4) and the free-list heads start at
/// `ctx+0x68`, which is **exactly 4 × 16 bytes** of lock. With `i*8` the range `0x48..0x68`
/// would be unaccounted for, and bucket 1's "lock" would be bucket 0's recursion count.
///
/// It was harmless in the dumps, where every lock word read zero. **This module walks live
/// memory, where they are not zero**, which is why it was worth fixing first.
const OFF_LOCK: usize = 0x28;
const LOCK_STRIDE: usize = 16;

/// `ctx + i*8 + 0x68` — free-list head. The links live **in the slot bodies**
/// (`mov qword ptr [rcx], rax` in the carve), so `next = u64[body]`.
const OFF_FREE_HEAD: usize = 0x68;
/// `ctx + i*8 + 0x88` — chunk-list head, a `body0` pointer; `[body0 − 0x10]` links to the
/// previous chunk.
const OFF_CHUNK_HEAD: usize = 0x88;
const PTR_STRIDE: usize = 8;

/// Bytes of pool context this module reads, so one `VirtualQuery` covers the lot.
const CTX_SPAN: usize = OFF_CHUNK_HEAD + 4 * PTR_STRIDE;

/// One size class. Read off the switch at `0x14019b7f0`; `research/heap-wild-write.md` §1.
#[derive(Clone, Copy)]
pub(crate) struct Bucket {
    pub slot: usize,
    pub per_chunk: usize,
}

impl Bucket {
    /// Slot size plus its 8-byte header.
    pub(crate) const fn stride(&self) -> usize {
        self.slot + 8
    }
    /// The chunk-size identity the big allocator writes at `chunkbase − 8`.
    pub(crate) const fn chunk_bytes(&self) -> usize {
        self.per_chunk * self.stride() + 8
    }
}

pub(crate) const BUCKETS: [Bucket; 4] = [
    Bucket {
        slot: 0x10,
        per_chunk: 64,
    },
    Bucket {
        slot: 0x20,
        per_chunk: 32,
    },
    Bucket {
        slot: 0x40,
        per_chunk: 16,
    },
    Bucket {
        slot: 0x80,
        per_chunk: 8,
    },
];

/// The `0x20` class — every damaged slot in fourteen observations.
pub(crate) const DAMAGE_CLASS: usize = 1;

/// The value fourteen of fourteen damaged headers have carried.
pub(crate) const KNOWN_DAMAGE: u64 = 0x0000_0001_0000_0020;

/// How often bucket 1 is walked. `research/heap-wild-write.md` §10 proposed this number; the
/// damage interval is 250–500 s, so the walk is free against it.
const DEFAULT_INTERVAL: Duration = Duration::from_millis(100);
/// Every Nth tick walks all four buckets rather than only the `0x20` class.
const DEFAULT_FULL_EVERY: u32 = 10;
/// How often the instrument reports that it is alive, and its own controls with it.
const DEFAULT_HEARTBEAT: Duration = Duration::from_secs(60);
/// Full-memory dumps of this client are ~1.3 GB and writing one freezes it for seconds.
const DEFAULT_MAX_DUMPS: u32 = 1;
/// A confirmed finding sooner than this after arming is tagged as suspect. See the module
/// docs: the archive has zero heap deaths under 192 s and an expected damaged count of ~0.34
/// at 100 s of in-field play.
const EARLY_SUSPECT: Duration = Duration::from_secs(120);

/// How many times a candidate must re-read the same non-`slot` value before it is believed.
const CONFIRM_READS: u32 = 3;
/// Spacing between those reads. A carve loop writes 32 headers in nanoseconds, so anything
/// still wrong 2 ms later is not a carve in flight.
const CONFIRM_SPACING: Duration = Duration::from_millis(2);

/// Re-run the `VirtualQuery` guard on each admitted chunk once every this many ticks, one
/// slice of the list per tick. At the default interval that closes the decommit window to
/// ~10 s for a cost of ~1 % of the chunks per walk.
const REVALIDATE_PERIOD: usize = 100;

/// Refuse to chase a chunk list longer than this. The client has never held more than ~2 700
/// chunks in a bucket; this exists so a cyclic or corrupt list cannot spin the thread.
const MAX_CHUNKS: usize = 65_536;

/// x64 `CONTEXT` field offsets, used by [`thread_snapshot`].
///
/// Module-level rather than local to that function **so a test can disagree with them**.
/// `CLAUDE.md`: a constant that came from reading a header is a claim, not a fact — the
/// `MINIDUMP_EXCEPTION_INFORMATION` test next door asserted 24/8 for weeks while Windows
/// wanted 16/4, because it pinned what the code already did. See
/// `the_context_offsets_name_rip_and_rsp`, which captures a real context and checks these two
/// offsets name something they could only name if they are right.
const CTX_FLAGS_OFF: usize = 0x30;
const CTX_RSP_OFF: usize = 0x98;
const CTX_RIP_OFF: usize = 0xF8;

static ARMED: AtomicBool = AtomicBool::new(false);
static DUMPS_WRITTEN: AtomicU32 = AtomicU32::new(0);
static FINDINGS: AtomicU32 = AtomicU32::new(0);
/// Headers put back by [`repair_header`]. Printed in the heartbeat beside the finding count,
/// because "how many deaths did this prevent" is not answerable and "how many headers did it
/// restore" is.
static REPAIRS: AtomicU32 = AtomicU32::new(0);

// ---------------------------------------------------------------------------------------
// Guarded reads
// ---------------------------------------------------------------------------------------

/// Read a qword from memory a caller has already validated with `VirtualQuery`.
///
/// # Safety
///
/// `at .. at+8` must lie inside a span the caller checked with [`crate::session::can_read`]
/// and has not released. Every call site in this file reads inside a chunk admitted by
/// [`BucketWalk::discover`].
unsafe fn read_u64_unchecked(at: usize) -> u64 {
    std::ptr::read_volatile(at as *const u64)
}

// ---------------------------------------------------------------------------------------
// The pool context
// ---------------------------------------------------------------------------------------

/// A reader for the allocator's own bookkeeping. Holds an address, not a copy: every read is
/// a fresh volatile read of live memory.
pub(crate) struct Pool {
    ctx: usize,
}

impl Pool {
    pub(crate) fn at(ctx: usize) -> Self {
        Pool { ctx }
    }

    /// Is the whole context block readable? One syscall, checked once per tick.
    unsafe fn readable(&self) -> bool {
        self.ctx != 0 && crate::session::can_read(self.ctx, CTX_SPAN)
    }

    unsafe fn carved(&self, i: usize) -> u32 {
        std::ptr::read_volatile((self.ctx + i * COUNTER_STRIDE + OFF_CARVED) as *const u32)
    }
    unsafe fn served(&self, i: usize) -> u32 {
        std::ptr::read_volatile((self.ctx + i * COUNTER_STRIDE + OFF_SERVED) as *const u32)
    }
    unsafe fn lock_owner(&self, i: usize) -> u64 {
        std::ptr::read_volatile((self.ctx + i * LOCK_STRIDE + OFF_LOCK) as *const u64)
    }
    unsafe fn lock_depth(&self, i: usize) -> u32 {
        std::ptr::read_volatile((self.ctx + i * LOCK_STRIDE + OFF_LOCK + 8) as *const u32)
    }
    unsafe fn free_head(&self, i: usize) -> usize {
        std::ptr::read_volatile((self.ctx + i * PTR_STRIDE + OFF_FREE_HEAD) as *const u64) as usize
    }
    unsafe fn chunk_head(&self, i: usize) -> usize {
        std::ptr::read_volatile((self.ctx + i * PTR_STRIDE + OFF_CHUNK_HEAD) as *const u64) as usize
    }
}

// ---------------------------------------------------------------------------------------
// The walk
// ---------------------------------------------------------------------------------------

/// One damaged slot header, as read.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Finding {
    pub bucket: usize,
    pub chunk: usize,
    /// Position in the chunk list at the moment of discovery, newest chunk = 0.
    pub chunk_age: usize,
    pub index: usize,
    pub header_va: usize,
    pub value: u64,
}

impl Finding {
    pub(crate) fn body(&self) -> usize {
        self.header_va + 8
    }
    /// Which arm of ntdll's `test r13b, 0xf` this slot would take if it were freed.
    /// `research/heap-wild-write.md` §4: the pool's 40-byte stride alternates 0 and 8 mod 16,
    /// so the failure type is decided by the parity of the slot index.
    pub(crate) fn predicted_failure_type(&self) -> u32 {
        if self.header_va % 16 == 0 {
            8
        } else {
            9
        }
    }
    pub(crate) fn classify(&self) -> &'static str {
        let slot = BUCKETS[self.bucket].slot as u64;
        if self.value == KNOWN_DAMAGE && slot == 0x20 {
            "KNOWN FAMILY - 0x0000000100000020, the value in 14 of 14 observations"
        } else if self.value & 0xFFFF_FFFF == slot {
            "NOVEL - same size class, but a high dword nobody has seen before"
        } else {
            "NOVEL - the low dword is not the slot size either. Suspect this module first."
        }
    }
}

/// Why a discovery walk stopped.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum WalkEnd {
    /// Reached the end of the chunk list. The normal outcome.
    ListEnd,
    /// A chunk pointer did not survive the `VirtualQuery` guard. Benign: the list is
    /// re-walked from the head next tick.
    Unreadable(usize),
    /// A chunk failed the allocator's own size identity.
    Identity(usize),
    /// The list was longer than [`MAX_CHUNKS`] — cyclic or corrupt.
    Overflow,
    /// The pool context itself was not readable.
    NoContext,
}

/// Per-bucket state that persists across ticks.
pub(crate) struct BucketWalk {
    pub bucket: usize,
    /// Chunk bases admitted by the guard, in chunk-list order (newest first).
    bases: Vec<usize>,
    known: HashSet<usize>,
    /// Chunks that failed the size identity after admission; skipped, and named once.
    suspect: HashSet<usize>,
    /// Header addresses already reported, so one damaged slot fires once.
    reported: HashSet<usize>,
    /// Rotating cursor for [`REVALIDATE_PERIOD`].
    revalidate_phase: usize,
}

impl BucketWalk {
    pub(crate) fn new(bucket: usize) -> Self {
        BucketWalk {
            bucket,
            bases: Vec::new(),
            known: HashSet::new(),
            suspect: HashSet::new(),
            reported: HashSet::new(),
            revalidate_phase: 0,
        }
    }

    pub(crate) fn chunks(&self) -> usize {
        self.bases.len()
    }

    /// Walk the chunk list from the head, admitting chunks that are new.
    ///
    /// The whole list is walked every time rather than stopping at the first known chunk.
    /// Stopping early was the first design and it has a trap: a chunk that fails the guard
    /// once truncates the list *permanently*, because the next walk stops at the first known
    /// chunk before ever reaching it. Walking through is a few thousand qword reads inside
    /// already-validated memory and it cannot go stale.
    ///
    /// # Safety
    ///
    /// `pool` must address the client's pool context, or a synthetic one of the same layout.
    pub(crate) unsafe fn discover(&mut self, pool: &Pool) -> WalkEnd {
        if !pool.readable() {
            return WalkEnd::NoContext;
        }
        let b = BUCKETS[self.bucket];
        let mut cur = pool.chunk_head(self.bucket);
        let mut steps = 0usize;
        while cur != 0 {
            steps += 1;
            if steps > MAX_CHUNKS {
                return WalkEnd::Overflow;
            }
            let Some(base) = cur.checked_sub(0x10) else {
                return WalkEnd::Unreadable(cur);
            };
            if !self.known.contains(&base) {
                // A chunk is admitted only if its whole span, INCLUDING the size header one
                // qword below it, is committed and readable, and the allocator's own identity
                // holds. Everything after this point reads inside that span.
                if base < 8 || !crate::session::can_read(base - 8, b.chunk_bytes() + 8) {
                    return WalkEnd::Unreadable(base);
                }
                if read_u64_unchecked(base - 8) != b.chunk_bytes() as u64 {
                    return WalkEnd::Identity(base);
                }
                self.bases.push(base);
                self.known.insert(base);
            }
            // Safe: `base` is inside a validated span, and `[base]` is its first qword.
            cur = read_u64_unchecked(base) as usize;
        }
        WalkEnd::ListEnd
    }

    /// Read every slot header in every admitted chunk. Returns anything that is not exactly
    /// the bucket's slot size and has not already been reported.
    ///
    /// # Safety
    ///
    /// Chunks must have been admitted by [`Self::discover`] against the same pool.
    pub(crate) unsafe fn scan(&mut self) -> (Vec<Finding>, usize, Vec<usize>) {
        let b = BUCKETS[self.bucket];
        let mut out = Vec::new();
        let mut slots = 0usize;
        let mut newly_suspect = Vec::new();

        // Rolling re-validation, one slice per tick, so a decommitted chunk cannot sit in the
        // set indefinitely. This is the residual TOCTOU of the admission guard, bounded.
        let phase = self.revalidate_phase;
        self.revalidate_phase = (self.revalidate_phase + 1) % REVALIDATE_PERIOD;

        for (i, &base) in self.bases.iter().enumerate() {
            if self.suspect.contains(&base) {
                continue;
            }
            if i % REVALIDATE_PERIOD == phase
                && !crate::session::can_read(base - 8, b.chunk_bytes() + 8)
            {
                newly_suspect.push(base);
                continue;
            }
            // Cheap per-tick re-check of the identity: one read out of thirty-three, and a
            // chunk whose size header changed is itself news.
            if read_u64_unchecked(base - 8) != b.chunk_bytes() as u64 {
                newly_suspect.push(base);
                continue;
            }
            for k in 0..b.per_chunk {
                let va = base + 8 + k * b.stride();
                let h = read_u64_unchecked(va);
                slots += 1;
                if h != b.slot as u64 && !self.reported.contains(&va) {
                    out.push(Finding {
                        bucket: self.bucket,
                        chunk: base,
                        chunk_age: i,
                        index: k,
                        header_va: va,
                        value: h,
                    });
                }
            }
        }
        for base in &newly_suspect {
            self.suspect.insert(*base);
        }
        (out, slots, newly_suspect)
    }

    /// A candidate is believed only after it re-reads the same wrong value
    /// [`CONFIRM_READS`] times. Returns the confirmed value.
    ///
    /// # Safety
    ///
    /// `f.header_va` must lie in a chunk admitted by [`Self::discover`].
    pub(crate) unsafe fn confirm(&self, f: &Finding) -> Option<u64> {
        let slot = BUCKETS[self.bucket].slot as u64;
        let mut seen = f.value;
        for _ in 0..CONFIRM_READS {
            std::thread::sleep(CONFIRM_SPACING);
            let again = read_u64_unchecked(f.header_va);
            if again != seen || again == slot {
                return None;
            }
            seen = again;
        }
        Some(seen)
    }

    pub(crate) fn mark_reported(&mut self, f: &Finding) {
        self.reported.insert(f.header_va);
    }

    /// **A slot that has been repaired is a slot that can be damaged again - and it is.**
    ///
    /// Run of 2026-09-07 19:40-20:50: twelve catches, twelve repairs, and the death dump shows
    /// **three of those twelve headers damaged again** - one of them reading
    /// `0x0000000200000020`, hit twice more after the repair. The writer re-uses its stale
    /// pointers. `reported` suppressed every re-hit (21 firings in 63 minutes, 12 caught, 9
    /// silent), so the re-hit on catch #12's slot went unrepaired and its free killed the
    /// client. Called after a successful repair, so the next damage to the same address is a
    /// new finding, a new repair, and a new window for the write watch.
    pub(crate) fn forget_reported(&mut self, header_va: usize) {
        self.reported.remove(&header_va);
    }

    /// The allocator's own control: chunks walked × slots per chunk must equal the `slots
    /// carved` counter. A short walk shows up here rather than as a clean, confident zero.
    ///
    /// # Safety
    ///
    /// `pool` must be readable; the caller checks that each tick.
    pub(crate) unsafe fn carve_identity(&self, pool: &Pool) -> (usize, u32, bool) {
        let walked = self.bases.len() * BUCKETS[self.bucket].per_chunk;
        let carved = pool.carved(self.bucket);
        (walked, carved, walked as u64 == carved as u64)
    }

    /// Sorted chunk bases, for containment tests. Built on demand — only the fire path needs
    /// it, and the hot walk must not pay for it.
    fn sorted_bases(&self) -> Vec<usize> {
        let mut v = self.bases.clone();
        v.sort_unstable();
        v
    }
}

/// Which slot of which admitted chunk a free-list body pointer names, if any.
///
/// This is what makes a live free-list walk safe: every step is checked against the chunk set
/// **before** it is dereferenced, so the walk can only ever read slot bodies inside memory
/// this module has validated. A popped node whose body has since been overwritten by its new
/// owner points somewhere that fails this test, and the walk stops.
pub(crate) fn slot_of_body(sorted: &[usize], b: &Bucket, body: usize) -> Option<(usize, usize)> {
    let i = sorted.partition_point(|&x| x <= body);
    if i == 0 {
        return None;
    }
    let base = sorted[i - 1];
    // body = base + 8 (chunk header) + k*stride + 8 (slot header)
    let off = body.checked_sub(base + 16)?;
    if off % b.stride() != 0 {
        return None;
    }
    let k = off / b.stride();
    if k >= b.per_chunk {
        return None;
    }
    Some((base, k))
}

/// The result of walking a bucket's free list against the allocator's two counters.
pub(crate) struct FreeList {
    pub bodies: HashSet<usize>,
    pub len: usize,
    /// `carved − freelist == served`, the identity `heap-corruption-2026-08-27.md` §4 uses.
    pub identity_holds: bool,
    pub carved: u32,
    pub served: u32,
    /// The walk stopped on a pointer that is not a slot of any admitted chunk. Expected
    /// occasionally on live memory — the list mutates while it is read — and reported rather
    /// than hidden, because a short walk would otherwise silently answer "not free".
    pub stopped_early: bool,
}

/// Walk one bucket's free list, dereferencing nothing that is not a validated slot body.
///
/// # Safety
///
/// `pool` must be readable and `walk` must hold chunks admitted against it.
pub(crate) unsafe fn walk_free_list(pool: &Pool, walk: &BucketWalk) -> FreeList {
    let b = BUCKETS[walk.bucket];
    let sorted = walk.sorted_bases();
    let carved = pool.carved(walk.bucket);
    let served = pool.served(walk.bucket);
    let mut bodies = HashSet::new();
    let mut cur = pool.free_head(walk.bucket);
    let mut stopped_early = false;
    // A free list cannot be longer than the number of slots that exist.
    let cap = carved as usize + 1;
    while cur != 0 {
        if slot_of_body(&sorted, &b, cur).is_none() || bodies.contains(&cur) || bodies.len() > cap {
            stopped_early = true;
            break;
        }
        bodies.insert(cur);
        cur = read_u64_unchecked(cur) as usize;
    }
    let len = bodies.len();
    FreeList {
        identity_holds: !stopped_early && carved as i64 - len as i64 == served as i64,
        bodies,
        len,
        carved,
        served,
        stopped_early,
    }
}

// ---------------------------------------------------------------------------------------
// The snapshot
// ---------------------------------------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 3);
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 && i % 8 == 0 {
            s.push(' ');
        }
        s.push_str(&format!("{byte:02x} "));
    }
    s.trim_end().to_string()
}

/// Header and body of one slot, plus whether it is on the free list right now.
///
/// # Safety
///
/// `header_va` must lie inside an admitted chunk.
unsafe fn describe_slot(
    header_va: usize,
    b: &Bucket,
    free: &FreeList,
    label: &str,
) -> String {
    let body = header_va + 8;
    let h = read_u64_unchecked(header_va);
    let raw = std::slice::from_raw_parts(body as *const u8, b.slot);
    let on_free = free.bodies.contains(&body);
    // A slot's body that looks like a UTF-16 BSTR is worth naming, because the 0x20 class is
    // Ztl task memory with BSTR layout: a 4-byte byte-count at body+0 and the pointer handed
    // around is body+4. That is why the damaged dword, header+4, is exactly body-4.
    let mut note = String::new();
    if b.slot >= 8 {
        let prefix = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]) as usize;
        if prefix >= 2 && prefix % 2 == 0 && prefix + 4 <= b.slot {
            let mut text = String::new();
            for i in 0..(prefix / 2) {
                let c = u16::from_le_bytes([raw[4 + i * 2], raw[5 + i * 2]]);
                if let Some(ch) = char::from_u32(u32::from(c)) {
                    if ch.is_ascii_graphic() || ch == ' ' {
                        text.push(ch);
                        continue;
                    }
                }
                text.clear();
                break;
            }
            if !text.is_empty() {
                note = format!("  looks like a BSTR of {} chars: {text:?}", prefix / 2);
            }
        }
    }
    format!(
        "      {label:<12} {header_va:#x}  hdr {h:#018x}  free-list={}{note}\n\
         \x20                   body {}",
        if on_free { "YES" } else { "no" },
        hex(raw)
    )
}

/// Every thread's instruction pointer and a shallow scan of its stack.
///
/// **One thread is suspended at a time and nothing at all happens between the suspend and
/// the resume** — no allocation, no `VirtualQuery`, no formatting. A suspended thread can be
/// holding the process address-space lock or the CRT heap lock, and a tool of this shape that
/// calls into either while holding a set of threads suspended deadlocks the process it is
/// trying to observe. Fidelity is traded for that: the stack is scanned after the thread is
/// running again, so a frame can be stale. `probe::stack_trace` has that property already and
/// `research/heap-corruption-2026-08-27.md` §5 records what it costs — read the frames as
/// leads, and note the damage is up to one interval old anyway, so the writing thread has
/// very likely moved on.
unsafe fn thread_snapshot() -> String {
    const TH32CS_SNAPTHREAD: u32 = 0x0000_0004;
    const THREAD_ACCESS: u32 = 0x0002 | 0x0008 | 0x0040; // SUSPEND_RESUME|GET_CONTEXT|QUERY_INFO
    const CONTEXT_CONTROL_INTEGER: u32 = 0x0010_0001 | 0x0010_0002;
    const INVALID_HANDLE_VALUE: *mut c_void = usize::MAX as *mut c_void;

    #[repr(C)]
    struct ThreadEntry32 {
        dw_size: u32,
        cnt_usage: u32,
        th32_thread_id: u32,
        th32_owner_process_id: u32,
        tp_base_pri: i32,
        tp_delta_pri: i32,
        dw_flags: u32,
    }
    #[repr(C, align(16))]
    struct Context([u8; 1232]);

    extern "system" {
        fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut c_void;
        fn Thread32First(snap: *mut c_void, entry: *mut ThreadEntry32) -> i32;
        fn Thread32Next(snap: *mut c_void, entry: *mut ThreadEntry32) -> i32;
        fn OpenThread(access: u32, inherit: i32, tid: u32) -> *mut c_void;
        fn SuspendThread(thread: *mut c_void) -> u32;
        fn ResumeThread(thread: *mut c_void) -> u32;
        fn GetThreadContext(thread: *mut c_void, ctx: *mut c_void) -> i32;
        fn CloseHandle(handle: *mut c_void) -> i32;
        fn GetCurrentThreadId() -> u32;
        fn GetCurrentProcessId() -> u32;
    }

    let pid = GetCurrentProcessId();
    let me = GetCurrentThreadId();
    let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if snap == INVALID_HANDLE_VALUE || snap.is_null() {
        return "      (thread snapshot unavailable: CreateToolhelp32Snapshot failed)\n"
            .to_string();
    }

    // Collect (tid, rip, rsp) first, formatting nothing while anything is suspended.
    let mut sampled: Vec<(u32, usize, usize)> = Vec::with_capacity(64);
    let mut entry = ThreadEntry32 {
        dw_size: std::mem::size_of::<ThreadEntry32>() as u32,
        cnt_usage: 0,
        th32_thread_id: 0,
        th32_owner_process_id: 0,
        tp_base_pri: 0,
        tp_delta_pri: 0,
        dw_flags: 0,
    };
    let mut ok = Thread32First(snap, &mut entry);
    while ok != 0 {
        let tid = entry.th32_thread_id;
        if entry.th32_owner_process_id == pid && tid != me {
            let handle = OpenThread(THREAD_ACCESS, 0, tid);
            if !handle.is_null() {
                let mut ctx = Context([0; 1232]);
                ctx.0[CTX_FLAGS_OFF..CTX_FLAGS_OFF + 4]
                    .copy_from_slice(&CONTEXT_CONTROL_INTEGER.to_le_bytes());
                // Suspend, read, resume. Nothing in between, deliberately.
                if SuspendThread(handle) != u32::MAX {
                    let got = GetThreadContext(handle, std::ptr::addr_of_mut!(ctx).cast());
                    ResumeThread(handle);
                    if got != 0 {
                        let rip = usize::from_le_bytes(
                            ctx.0[CTX_RIP_OFF..CTX_RIP_OFF + 8].try_into().unwrap(),
                        );
                        let rsp = usize::from_le_bytes(
                            ctx.0[CTX_RSP_OFF..CTX_RSP_OFF + 8].try_into().unwrap(),
                        );
                        sampled.push((tid, rip, rsp));
                    }
                }
                CloseHandle(handle);
            }
        }
        entry.dw_size = std::mem::size_of::<ThreadEntry32>() as u32;
        ok = Thread32Next(snap, &mut entry);
    }
    CloseHandle(snap);

    let mut out = format!("      {} other threads sampled\n", sampled.len());
    for (tid, rip, rsp) in sampled {
        out.push_str(&format!(
            "      tid {tid:<6} rip={rip:#x}{}{}\n",
            crate::netwatch::module_of(rip),
            crate::probe::stack_trace(rsp)
        ));
    }
    out
}

// ---------------------------------------------------------------------------------------
// The dump
// ---------------------------------------------------------------------------------------

/// Where the dump goes. The same marker and env the crash dump writer uses, so both land in
/// the folder the test plan tells the owner to look in.
fn dump_dir() -> String {
    if let Ok(dir) = std::fs::read_to_string(crate::minidump::DUMP_DIR_MARKER) {
        let dir = dir.trim().to_string();
        if !dir.is_empty() {
            return dir;
        }
    }
    if let Ok(dir) = std::env::var(crate::minidump::DUMP_DIR_ENV) {
        if !dir.trim().is_empty() {
            return dir;
        }
    }
    match std::path::Path::new(&crate::hook::log_path()).parent() {
        Some(p) if !p.as_os_str().is_empty() => p.to_string_lossy().into_owned(),
        _ => ".".to_string(),
    }
}

const MINIDUMP_WITH_FULL_MEMORY: u32 = 0x0000_0002;
const MINIDUMP_WITH_HANDLE_DATA: u32 = 0x0000_0004;
const MINIDUMP_WITH_UNLOADED_MODULES: u32 = 0x0000_0020;
const MINIDUMP_WITH_FULL_MEMORY_INFO: u32 = 0x0000_0800;
const MINIDUMP_WITH_THREAD_INFO: u32 = 0x0000_1000;
const DUMP_FLAGS: u32 = MINIDUMP_WITH_FULL_MEMORY
    | MINIDUMP_WITH_HANDLE_DATA
    | MINIDUMP_WITH_UNLOADED_MODULES
    | MINIDUMP_WITH_FULL_MEMORY_INFO
    | MINIDUMP_WITH_THREAD_INFO;

type MiniDumpWriteDumpFn = unsafe extern "system" fn(
    process: *mut c_void,
    process_id: u32,
    file: *mut c_void,
    dump_type: u32,
    exception_param: *const c_void,
    user_stream_param: *const c_void,
    callback_param: *const c_void,
) -> i32;

/// Write a full-memory dump with **no** exception information.
///
/// This is deliberately its own writer rather than a call into [`crate::minidump`], for two
/// reasons that are both about not spoiling an instrument that already works: that module's
/// dump budget is the *crash* budget, and a sentry that spent it would leave the fatal
/// `0xC0000374` with no dump at all — a strict regression. And there is no exception here, so
/// the `exception_param` is null, which sidesteps the `pshpack4` layout trap
/// `crates/grap-stub/src/minidump.rs` documents at length: with a null pointer there is no
/// struct to lay out wrongly.
///
/// # Safety
///
/// Calls into dbghelp on the live process. Safe to call from any thread; dbghelp suspends the
/// others itself for the duration, which is why this freezes the client for seconds.
/// **`MiniDumpWriteDump` is not reentrant, and this crate now has two callers.**
///
/// Measured, not assumed. With this module's self-test and `minidump.rs`'s running in one
/// process among 51 others, the test binary **deadlocked in 1 run of 4**, hanging both dump
/// tests and freezing three unrelated tests mid-flight — the signature of dbghelp suspending
/// every other thread and then allocating. With **either dump test alone** among the same 51:
/// **0 hangs in 12 runs, both ways.** Two callers is the necessary ingredient.
///
/// That is a production hazard, not just a test one: the sentry writing a dump while the
/// client faults would put [`crate::minidump::write_crash_dump`] into dbghelp on the faulting
/// thread at the same time, and the likely outcome is a hung client with **neither** dump —
/// which would cost the project its most valuable artifact.
///
/// **Both halves are now in place.** `crates/grap-stub/src/minidump.rs` takes this same
/// lock around its own `write_dump(...)`, so the two callers in this crate cannot be inside
/// dbghelp at once.
///
/// The hazard is not theoretical and it is not a test problem. Measured while building this
/// module: with both dump self-tests running among 51 tests, the binary **deadlocked in 1
/// run of 4**, hanging both and freezing three unrelated tests mid-flight. With either alone
/// it was **0 hangs in 12 runs, both ways** - two callers is the necessary ingredient. In
/// production the shape is worse: the sentry dumping at the moment the client faults would
/// put both into dbghelp together, and the likely outcome is a hung client with **neither**
/// dump - losing the crash dump this project spent weeks learning to capture at all.
static DUMP_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Serialise a dbghelp call against the sentry's. See [`DUMP_LOCK`].
///
/// Returns a guard; hold it across the whole `MiniDumpWriteDump`. A poisoned lock is
/// **ignored deliberately** - the previous holder panicking mid-dump is not a reason to
/// refuse to write a crash dump, which is the one artefact that explains why it panicked.
pub(crate) fn dump_lock() -> std::sync::MutexGuard<'static, ()> {
    match DUMP_LOCK.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// `dir` is passed rather than resolved inside, so the self-test never has to touch
/// [`crate::minidump::DUMP_DIR_ENV`]. It did at first, and that is a **process-global**: it
/// redirected `minidump.rs`'s own self-test into this test's scratch directory, which then
/// deleted it, and `minidump::tests::it_actually_writes_a_readable_minidump` failed with
/// "expected exactly one dump, got []". A per-pid directory was not enough — the shared
/// mutable global had to go.
pub(crate) unsafe fn write_pool_dump(dir: &str, tag: &str) -> Result<(String, u64), String> {
    let _serialised = DUMP_LOCK.lock();
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const CREATE_ALWAYS: u32 = 2;
    const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
    const INVALID_HANDLE_VALUE: *mut c_void = usize::MAX as *mut c_void;
    extern "system" {
        fn LoadLibraryA(name: *const u8) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
        fn GetCurrentProcess() -> *mut c_void;
        fn GetCurrentProcessId() -> u32;
        fn CreateFileW(
            name: *const u16,
            access: u32,
            share: u32,
            security: *mut c_void,
            disposition: u32,
            flags: u32,
            template: *mut c_void,
        ) -> *mut c_void;
        fn CloseHandle(handle: *mut c_void) -> i32;
        fn GetLastError() -> u32;
    }

    let dbghelp = LoadLibraryA(c"dbghelp.dll".as_ptr().cast());
    if dbghelp.is_null() {
        return Err("dbghelp.dll would not load".to_string());
    }
    let proc = GetProcAddress(dbghelp, c"MiniDumpWriteDump".as_ptr().cast());
    if proc.is_null() {
        return Err("dbghelp.dll has no MiniDumpWriteDump".to_string());
    }
    let write_dump: MiniDumpWriteDumpFn = std::mem::transmute(proc);

    let pid = GetCurrentProcessId();
    let n = DUMPS_WRITTEN.load(Ordering::SeqCst);
    let path = format!(
        "{}\\maplecw-sentry-{pid}-{tag}-{n}.dmp",
        dir.trim_end_matches(['\\', '/'])
    );
    let wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    let file = CreateFileW(
        wide.as_ptr(),
        GENERIC_WRITE,
        0,
        std::ptr::null_mut(),
        CREATE_ALWAYS,
        FILE_ATTRIBUTE_NORMAL,
        std::ptr::null_mut(),
    );
    if file == INVALID_HANDLE_VALUE || file.is_null() {
        return Err(format!("could not create {path} (error {})", GetLastError()));
    }
    let ok = write_dump(
        GetCurrentProcess(),
        pid,
        file,
        DUMP_FLAGS,
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    );
    let err = if ok == 0 { GetLastError() } else { 0 };
    CloseHandle(file);
    if ok == 0 {
        return Err(format!("MiniDumpWriteDump FAILED for {path} (error {err:#x})"));
    }
    let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
    Ok((path, size))
}

// ---------------------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(crate) struct Config {
    pub interval: Duration,
    pub full_every: u32,
    pub heartbeat: Duration,
    pub max_dumps: u32,
    pub stacks: bool,
    /// Put the damaged header back. **Off by default, and it is the only thing in this module
    /// that writes to the client.** See [`repair_header`].
    pub repair: bool,
    /// Walk this slowly while no firing is due. `None` means never - walk at
    /// [`Config::interval`] the whole time, which is the behaviour every measurement to date
    /// was taken with. See [`nap_for`].
    pub coarse: Option<Duration>,
    /// Arm [`crate::writewatch`] around each predicted firing, so the store that damages the
    /// header faults at the writing instruction. Off by default: it protects the client's
    /// hottest size class read-only for [`Config::write_window`] every three minutes.
    pub write: bool,
    /// How far BEFORE the predicted firing the window opens.
    pub write_lead: Duration,
    /// How long the window stays open.
    pub write_window: Duration,
    /// How often pages that have been written are protected again inside the window.
    pub write_burst: Duration,
}

/// The period six catches across three sessions measured, used to place the write window
/// after the FIRST finding instead of the second.
///
/// This is the file asserting a conclusion of its own, so it is confined to one thing: where
/// to point a watch that costs a stutter if it is wrong. [`nap_for`], which decides whether
/// to walk at all and can therefore miss a catch, still uses only the period this session
/// measured for itself.
const ASSUMED_PERIOD: Duration = Duration::from_secs(180);

/// How long to wait for the write window to close before a repair. The window's own burst is
/// 5 ms, so this is twenty of them, and the corrupt-to-free race it has to fit inside is
/// 720 ms.
const WRITE_STOP_WAIT: Duration = Duration::from_millis(60);

/// How far either side of a predicted firing the sentry stays at the fine interval.
///
/// The period held to ±1.02 s over six consecutive intervals on 2026-09-07, and the one long
/// gap **shifted the phase permanently** rather than snapping back - so the window has to
/// absorb drift, not just jitter. Five seconds is about five times the largest excursion seen.
const FIRE_WINDOW: Duration = Duration::from_secs(5);

/// How long to sleep before the next walk.
///
/// # Why this exists
///
/// The owner, 2026-09-07: *"it lags/freezes the client every time it runs, which is undesirable."*
/// Measured from that session's own heartbeats: an ordinary walk costs **0.63-0.71 ms**, a
/// finding without a dump costs **49 ms**, and a finding *with* one costs **703-895 ms** and
/// freezes the client outright. The dumps and the thread scan are the freezes and both are
/// already switchable (`dumps=0`, `stacks=off`). What is left is 70 368 slot headers read
/// every 100 ms, which is cheap in CPU and **not** cheap in cache - it evicts the client's
/// working set ten times a second, which is the shape of a continuous stutter that does not
/// show up in the walk's own wall-clock timing. That last part is `[I]`, not measured.
///
/// # What makes this safe
///
/// The writer runs on a 180-second clock (`research/the-180-second-clock-2026-09-07.md`), so
/// once two findings have been seen the next one is predictable. Outside a
/// [`FIRE_WINDOW`] of that prediction there is nothing to catch, and the walk can be as lazy
/// as the caller likes. Inside it, the interval is exactly what it always was.
///
/// **The fine interval still has to win a race**, which is why the coarse one is opt-in and
/// why a surprise resets it: the one observed corrupt-to-free gap is 720 ms, so a walk that is
/// 2 s late has already lost. A finding that arrives while coarse means the prediction was
/// wrong, and [`walk_loop`] drops the learned period on the spot and goes back to fine.
fn nap_for(cfg: &Config, last_fire: Option<Instant>, period: Option<Duration>) -> Duration {
    let (Some(coarse), Some(last), Some(p)) = (cfg.coarse, last_fire, period) else {
        return cfg.interval;
    };
    if p.is_zero() {
        return cfg.interval;
    }
    let since = last.elapsed().as_secs_f64();
    let p = p.as_secs_f64();
    let phase = since % p;
    let window = FIRE_WINDOW.as_secs_f64();
    // Near the firing we just passed, or near the one coming up.
    if phase <= window || p - phase <= window {
        cfg.interval
    } else {
        coarse.min(Duration::from_secs_f64((p - phase - window).max(0.001)))
    }
}

/// Which firing of a `period` clock, counted from `last`, has been reached by `at`.
///
/// Returns `0` before the first one. [`run`] arms the write window when this steps up, with
/// `at = now + lead`, so the window opens exactly `lead` before the predicted firing and once
/// per firing rather than once per walk.
fn firing_index(last: Instant, period: Duration, at: Instant) -> u64 {
    let p = period.as_secs_f64();
    if p <= 0.0 {
        return 0;
    }
    let elapsed = at.saturating_duration_since(last).as_secs_f64();
    if elapsed <= 0.0 {
        return 0;
    }
    (elapsed / p).floor() as u64
}

/// Should a write window open now for the firing `lead` ahead, given the one already armed?
///
/// Returns the key to record when it should. Keyed by **anchor and index**: every catch
/// re-anchors the clock, so "index 1" recurs every cycle, and a guard that compared indices
/// alone opened exactly one window in a 42-minute run with twelve catches (2026-09-07 21:57).
fn window_due(
    armed: Option<(Instant, u64)>,
    last: Instant,
    period: Duration,
    at: Instant,
) -> Option<(Instant, u64)> {
    let idx = firing_index(last, period, at);
    if idx >= 1 && armed != Some((last, idx)) {
        Some((last, idx))
    } else {
        None
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interval: DEFAULT_INTERVAL,
            full_every: DEFAULT_FULL_EVERY,
            heartbeat: DEFAULT_HEARTBEAT,
            max_dumps: DEFAULT_MAX_DUMPS,
            stacks: true,
            repair: false,
            coarse: None,
            write: false,
            write_lead: Duration::from_millis(500),
            write_window: Duration::from_millis(1200),
            write_burst: Duration::from_millis(5),
        }
    }
}

/// Parse the marker's contents. Unknown tokens are ignored rather than disabling the ones
/// beside them, the same rule `session::marker_token` follows.
///
/// `off` (or `0`) anywhere in the file disarms, which is the one token that must not be
/// silently misread — a stale marker arming the sentry on an unrelated run would be exactly
/// the kind of quiet confound `CLAUDE.md` catalogues.
pub(crate) fn parse_config(text: &str) -> Option<Config> {
    let text = text.trim().to_ascii_lowercase();
    let mut cfg = Config::default();
    let mut disabled = false;
    let mut coarse_ms: Option<u64> = None;
    for token in text.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        if token == "off" || token == "0" || token == "no" {
            disabled = true;
        } else if let Some(v) = token.strip_prefix("interval=") {
            if let Ok(ms) = v.parse::<u64>() {
                if ms >= 5 {
                    cfg.interval = Duration::from_millis(ms);
                }
            }
        } else if let Some(v) = token.strip_prefix("full=") {
            if let Ok(n) = v.parse::<u32>() {
                cfg.full_every = n.max(1);
            }
        } else if let Some(v) = token.strip_prefix("heartbeat=") {
            if let Ok(s) = v.parse::<u64>() {
                cfg.heartbeat = Duration::from_secs(s.max(1));
            }
        } else if let Some(v) = token.strip_prefix("dumps=") {
            if let Ok(n) = v.parse::<u32>() {
                cfg.max_dumps = n;
            }
        } else if token == "stacks=off" {
            cfg.stacks = false;
        } else if token == "repair=on" {
            cfg.repair = true;
        } else if token == "write=on" {
            cfg.write = true;
        } else if let Some(v) = token.strip_prefix("writelead=") {
            if let Ok(ms) = v.parse::<u64>() {
                if ms <= 5_000 {
                    cfg.write_lead = Duration::from_millis(ms);
                }
            }
        } else if let Some(v) = token.strip_prefix("writewindow=") {
            if let Ok(ms) = v.parse::<u64>() {
                // Capped hard. This window protects the client's hottest size class, and a
                // marker typo asking for a minute of it would be indistinguishable from the
                // client having become unplayable.
                if (50..=5_000).contains(&ms) {
                    cfg.write_window = Duration::from_millis(ms);
                }
            }
        } else if let Some(v) = token.strip_prefix("writeburst=") {
            if let Ok(ms) = v.parse::<u64>() {
                if (1..=500).contains(&ms) {
                    cfg.write_burst = Duration::from_millis(ms);
                }
            }
        } else if let Some(v) = token.strip_prefix("coarse=") {
            // Held aside and validated after the loop: it is compared against `interval`, and
            // comparing here would make the answer depend on which token came first.
            coarse_ms = v.parse::<u64>().ok();
        }
    }
    // A coarse interval shorter than the fine one is a mistake, not an instruction to walk
    // harder, and one long enough to lose the 720 ms corrupt-to-free race is refused rather
    // than honoured quietly. Both are dropped in silence the way every other bad token here is.
    if let Some(ms) = coarse_ms {
        let d = Duration::from_millis(ms);
        if d > cfg.interval && ms <= 10_000 {
            cfg.coarse = Some(d);
        }
    }
    if disabled {
        None
    } else {
        Some(cfg)
    }
}

// ---------------------------------------------------------------------------------------
// Installation and the loop
// ---------------------------------------------------------------------------------------

/// Arm the sentry if [`SENTRY_MARKER`] is present. Idempotent; reads nothing if it is not.
///
/// Called from the hook's install path, after the module base is resolved and after Themida
/// has unpacked `.text` — the pool context is a data address so unpacking does not strictly
/// matter, but the base does.
pub fn install() {
    if ARMED.swap(true, Ordering::SeqCst) {
        return;
    }
    // **Read once, then delete**, exactly as `identity.rs` does with its own marker and for
    // the same reason: a marker that outlives its run silently arms a 100 ms allocator walk
    // inside whatever launches next, and that confound would be invisible in the logs of the
    // run it contaminated.
    //
    // Self-clearing is what lets the LAUNCHER leave the file alone. It cannot: the ordinary
    // diagnostic path is `test-server.ps1 -ServersOnly` writing the marker and the launcher
    // starting the client, so a launcher that deleted it would make that combination
    // impossible - which it briefly did.
    let Ok(text) = std::fs::read_to_string(SENTRY_MARKER) else {
        return;
    };
    let _ = std::fs::remove_file(SENTRY_MARKER);
    let Some(cfg) = parse_config(&text) else {
        log(&format!(
            "***** POOL SENTRY: {SENTRY_MARKER} says {:?} - standing down *****",
            text.trim()
        ));
        return;
    };
    std::thread::spawn(move || unsafe { run(cfg) });
}

/// The sentry loop.
///
/// # Safety
///
/// Reads the client's live pool allocator. Every dereference is guarded; see the module docs.
unsafe fn run(cfg: Config) {
    let base = crate::hook::base();
    if base == 0 {
        log("***** POOL SENTRY: no module base - NOT armed. Nothing is watching the pool *****");
        return;
    }
    let pool = Pool::at(base + POOL_CTX_RVA);
    let mut walks: Vec<BucketWalk> = (0..BUCKETS.len()).map(BucketWalk::new).collect();

    // Give the client a moment to have carved something. Arming against an empty pool would
    // log a baseline of zero and look identical to a broken walk.
    std::thread::sleep(Duration::from_secs(2));

    let armed_at = Instant::now();
    let mut baseline = String::new();
    for i in 0..BUCKETS.len() {
        let end = walks[i].discover(&pool);
        let (found, slots, _) = walks[i].scan();
        let (walked, carved, holds) = walks[i].carve_identity(&pool);
        baseline.push_str(&format!(
            "\n      bucket {i} slot {:#04x}: {} chunks, {slots} slots, {} not equal to the \
             slot size; walk ended {end:?}; carve identity {walked} vs carved {carved} {}",
            BUCKETS[i].slot,
            walks[i].chunks(),
            found.len(),
            if holds { "[PASS]" } else { "[FAIL]" }
        ));
        for f in &found {
            walks[i].mark_reported(&f.clone());
            baseline.push_str(&format!(
                "\n        ALREADY DAMAGED AT ARM: {:#x} hdr {:#018x} - the write happened \
                 before the sentry could see it",
                f.header_va, f.value
            ));
        }
    }
    log(&format!(
        "***** POOL SENTRY ARMED: pool ctx {:#x}, every {} ms (all four buckets every {}), \
         heartbeat {} s, up to {} dump(s). {} No lock is taken either way. The writer runs on \
         a 180-SECOND CLOCK - six catches, four intervals, all 180.0 s within 0.12 s \
         (research/the-180-second-clock-2026-09-07.md) - so after the first catch the next one \
         is predictable, and silence for longer than that is itself the news.{baseline} *****",
        pool.ctx,
        cfg.interval.as_millis(),
        cfg.full_every,
        cfg.heartbeat.as_secs(),
        cfg.max_dumps,
        if cfg.repair {
            "REPAIR IS ON: a confirmed damaged header is put back to the slot size, which is \
             the ONE client byte this module ever writes - it turns a fatal free into a \
             correct one and does NOT stop the writer."
        } else {
            "It READS ONLY - no client byte is written."
        },
    ));
    if cfg.write {
        log(&format!(
            "***** POOL WRITE WATCH ARMED: {} ms before each predicted firing, bucket              {DAMAGE_CLASS}'s pages go PAGE_READONLY for {} ms, re-protected every {} ms.              Reads are untouched. A write into that memory faults, the handler records RIP              and the exact address, makes the page writable and re-executes the instruction,              so the client continues. This is what turns a *when* into a *who*: no window can              open until the first finding gives it a phase, so the first six minutes look              exactly like an ordinary sentry run. It writes nothing to the client -              VirtualProtect is a permission change, not an edit *****",
            cfg.write_lead.as_millis(),
            cfg.write_window.as_millis(),
            cfg.write_burst.as_millis(),
        ));
    }

    let mut tick: usize = 0;
    let mut last_beat = Instant::now();
    let mut walk_us_max = 0u128;
    let mut walk_us_total = 0u128;
    let mut walk_count = 0u128;

    // What [`nap_for`] needs to predict the next firing, learned from the findings themselves
    // rather than assumed: a hard-coded 180 s would be this file asserting its own conclusion.
    let mut last_fire: Option<Instant> = None;
    let mut period: Option<Duration> = None;
    // Which predicted firing the write window has already been opened for - keyed by the
    // anchor it was predicted FROM as well as the index, so one firing arms one window however
    // many walks fall inside the lead, and a new anchor starts afresh.
    //
    // The first version kept only the index. Every catch re-anchors `last_fire`, so the next
    // firing is index 1 from the new anchor - the same index the previous window had - and the
    // guard skipped it. Run of 2026-09-07 21:57-22:39: twelve catches, ONE window. Found by
    // counting windows against catches in the heartbeat.
    let mut write_armed_for: Option<(Instant, u64)> = None;
    // Every header this run has caught, whether or not it has since been repaired. The writer
    // re-hits them (see `BucketWalk::forget_reported`), so the write watch pins their pages on
    // every window rather than hoping the fully-contained rule happens to cover them.
    let mut ever_damaged: Vec<usize> = Vec::new();

    loop {
        let mut nap = nap_for(&cfg, last_fire, period);
        // Never sleep past the moment a write window should open. `nap_for` walks coarse
        // outside its own FIRE_WINDOW and knows nothing about the write watch, so without
        // this a 2 s coarse nap would place the window up to 2 s late - which is the entire
        // budget it has.
        if cfg.write {
            if let (Some(last), Some(p)) = (last_fire, period.or(Some(ASSUMED_PERIOD))) {
                let now = Instant::now();
                let next = firing_index(last, p, now) + 1;
                if let Some(open) = (last + p.mul_f64(next as f64)).checked_sub(cfg.write_lead) {
                    if open > now {
                        nap = nap.min(open.duration_since(now));
                    }
                }
            }
        }
        let was_coarse = nap > cfg.interval;
        std::thread::sleep(nap);
        tick += 1;
        let full = cfg.full_every > 0 && tick as u32 % cfg.full_every == 0;
        let started = Instant::now();

        // Open the write window `write_lead` before the next predicted firing. It needs the
        // chunk bases, which only this thread has, and it must NOT block: the whole design is
        // that the walk keeps running while the pages are protected, because the walk is what
        // notices the damage the fault let through.
        if cfg.write {
            if let (Some(last), Some(p)) = (last_fire, period.or(Some(ASSUMED_PERIOD))) {
                if let Some(key) = window_due(write_armed_for, last, p, started + cfg.write_lead) {
                    write_armed_for = Some(key);
                    let bases = walks[DAMAGE_CLASS].sorted_bases();
                    let chunks = bases.len();
                    match crate::writewatch::arm_for(bases, ever_damaged.clone(), cfg.write_window, cfg.write_burst) {
                        Some(a) => log(&format!(
                            "***** POOL WRITE WATCH: window #{} open for {} ms over {} run(s)                              ({} page(s), {} protected, {} of them PINNED under headers this run already caught) covering {chunks} bucket-{DAMAGE_CLASS}                              chunks, {} ms before the predicted firing ({} period, {:.3} s).                              Reads are untouched; a WRITE into this memory now faults at the                              instruction that made it.{}{} *****",
                            crate::writewatch::armed_windows(),
                            cfg.write_window.as_millis(),
                            a.runs,
                            a.pages,
                            a.protected,
                            a.pinned,
                            cfg.write_lead.as_millis(),
                            if period.is_some() { "measured" } else { "ASSUMED" },
                            p.as_secs_f64(),
                            a.note.map(|n| format!(" {n}.")).unwrap_or_default(),
                            if a.protected == 0 {
                                " NOTHING WAS PROTECTED - this window can only report silence."
                            } else {
                                ""
                            },
                        )),
                        None => log(
                            "***** POOL WRITE WATCH: a window was still open when the next one                              came due - skipped rather than overlapped *****",
                        ),
                    }
                }
            }
            for line in crate::writewatch::drain() {
                log(&line);
            }
            if crate::writewatch::wants_dump() {
                let n = DUMPS_WRITTEN.fetch_add(1, Ordering::SeqCst);
                if n < cfg.max_dumps {
                    let began = Instant::now();
                    match write_pool_dump(&dump_dir(), "writer") {
                        Ok((path, size)) => log(&format!(
                            "***** POOL WRITE WATCH: wrote {path}, {size} bytes in {} ms - the                              pool as it stood within milliseconds of the write *****",
                            began.elapsed().as_millis()
                        )),
                        Err(why) => log(&format!(
                            "***** POOL WRITE WATCH: NO DUMP after a header write - {why}. The                              RIP logged above is still the answer *****"
                        )),
                    }
                }
            }
        }

        for i in 0..BUCKETS.len() {
            if i != DAMAGE_CLASS && !full {
                continue;
            }
            let end = walks[i].discover(&pool);
            if matches!(end, WalkEnd::NoContext) {
                continue;
            }
            let (found, _slots, newly_suspect) = walks[i].scan();
            for base in newly_suspect {
                log(&format!(
                    "***** POOL SENTRY: chunk {base:#x} in bucket {i} stopped satisfying the \
                     allocator's size identity (or its pages went away) and is no longer \
                     scanned. That is itself news - the pool has never been seen to release a \
                     chunk *****"
                ));
            }
            for f in found {
                let Some(value) = walks[i].confirm(&f) else {
                    log(&format!(
                        "***** POOL SENTRY: UNCONFIRMED candidate at {:#x} (read \
                         {:#018x}, did not survive {CONFIRM_READS} re-reads). Recorded rather \
                         than dropped: a detector that silently discards its own near-misses \
                         cannot be told from one that never fires *****",
                        f.header_va, f.value
                    ));
                    walks[i].mark_reported(&f);
                    continue;
                };
                let mut confirmed = f;
                confirmed.value = value;
                walks[i].mark_reported(&confirmed);
                if !ever_damaged.contains(&confirmed.header_va) {
                    ever_damaged.push(confirmed.header_va);
                }

                // Learn the cadence. A finding that arrived while we were walking COARSE means
                // the prediction was wrong, so the period is dropped rather than refined - the
                // fine interval has a 720 ms race to win and guessing again with bad data is
                // how it would be lost.
                let now = Instant::now();
                if was_coarse {
                    if period.is_some() {
                        log(
                            "***** POOL SENTRY: a finding arrived OUTSIDE the predicted window \
                             - the learned period was wrong. Dropping it and returning to the \
                             fine interval *****",
                        );
                    }
                    period = None;
                } else if let Some(prev) = last_fire {
                    let gap = now.duration_since(prev);
                    period = if gap >= Duration::from_secs(20) && gap <= Duration::from_secs(600) {
                        Some(gap)
                    } else {
                        None
                    };
                }
                last_fire = Some(now);
                // **The repair goes FIRST, and the ordering is the whole point of it.**
                // `report` writes a ~1.3 GB dump that freezes the client for about 660 ms,
                // and the one observed corrupt-to-free gap in this family is 720 ms. A repair
                // after the dump would have beaten that by 60 ms; a repair before it beats it
                // by 700. The cost is that the dump then shows a clean header - acceptable,
                // because the FINDING block below records the value as read, and five dumps
                // of the damage already exist.
                if cfg.repair {
                    // The repair stores into a slot header, and `session::can_write` refuses
                    // a PAGE_READONLY page - so an open write window would turn every repair
                    // into a refusal. Close it first and wait for the pages to come back.
                    if cfg.write && !crate::writewatch::stop_and_wait(WRITE_STOP_WAIT) {
                        log(&format!(
                            "***** POOL WRITE WATCH: the window did not close within {} ms, so                              the repair below may be refused for a read-only page. That is the                              instrument standing in the way of the mitigation, not new damage                              *****",
                            WRITE_STOP_WAIT.as_millis()
                        ));
                    }
                    match repair_header(&confirmed) {
                        Ok(now) => {
                            REPAIRS.fetch_add(1, Ordering::SeqCst);
                            // Repaired, so damageable again: let the next hit on this address
                            // be a finding rather than a silence.
                            walks[i].forget_reported(confirmed.header_va);
                            log(&format!(
                                "***** POOL SENTRY REPAIR: {:#x} was {:#018x}, now {now:#018x}. \
                                 The next free of this slot goes back on the pool's own list \
                                 instead of to HeapFree. This does NOT stop the writer, and \
                                 the dump below (if any) shows the REPAIRED header *****",
                                confirmed.header_va, confirmed.value
                            ));
                        }
                        Err(why) => log(&format!(
                            "***** POOL SENTRY REPAIR REFUSED for {:#x}: {why} *****",
                            confirmed.header_va
                        )),
                    }
                }
                report(&pool, &walks[i], &confirmed, armed_at, &cfg);
            }
        }

        let us = started.elapsed().as_micros();
        walk_us_max = walk_us_max.max(us);
        walk_us_total += us;
        walk_count += 1;

        if last_beat.elapsed() >= cfg.heartbeat {
            last_beat = Instant::now();
            let mut line = String::new();
            for i in 0..BUCKETS.len() {
                let (walked, carved, holds) = walks[i].carve_identity(&pool);
                line.push_str(&format!(
                    " | b{i}:{}ch {}slots carve {}",
                    walks[i].chunks(),
                    walked,
                    if holds {
                        "PASS".to_string()
                    } else {
                        format!("FAIL {walked}!={carved}")
                    }
                ));
            }
            let repaired = REPAIRS.load(Ordering::SeqCst);
            let line = if cfg.repair {
                format!("{line} | {repaired} header(s) repaired")
            } else {
                line
            };
            // The write watch's own counters. `faults` is the liveness control: a window
            // that protected pages and then saw zero writes did not watch anything, and that
            // is a different result from "the writer did not fire".
            let line = if cfg.write {
                let (faults, header, body, edge) = crate::writewatch::counters();
                let off = crate::writewatch::backed_off();
                format!(
                    "{line} | write watch: {} window(s), {faults} write fault(s)                      ({header} on a slot HEADER, {body} body, {edge} edge){}",
                    crate::writewatch::armed_windows(),
                    if off > 0 {
                        format!(
                            " | {off} window(s) BACKED OFF at the fault cap and were blind for                              part of their window - the 0x20 class is written harder than                              assumed"
                        )
                    } else {
                        String::new()
                    }
                )
            } else {
                line
            };
            // Say which cadence is in force. A run that quietly walked coarse the whole time
            // would under-count findings and look identical to a quiet client.
            let line = match (cfg.coarse, period) {
                (Some(c), Some(p)) => format!(
                    "{line} | adaptive: {} ms fine within {} s of each firing, {} ms otherwise \
                     (period learned: {:.3} s)",
                    cfg.interval.as_millis(),
                    FIRE_WINDOW.as_secs(),
                    c.as_millis(),
                    p.as_secs_f64()
                ),
                (Some(c), None) => format!(
                    "{line} | adaptive armed ({} ms) but NO period learned yet - still walking \
                     every {} ms",
                    c.as_millis(),
                    cfg.interval.as_millis()
                ),
                _ => line,
            };
            log(&format!(
                "POOL SENTRY alive {}s: {} confirmed finding(s){line} | walk {} us avg, \
                 {walk_us_max} us max over {walk_count} walks",
                armed_at.elapsed().as_secs(),
                FINDINGS.load(Ordering::SeqCst),
                if walk_count == 0 {
                    0
                } else {
                    walk_us_total / walk_count
                },
            ));
            walk_us_max = 0;
            walk_us_total = 0;
            walk_count = 0;
        }
    }
}

/// Put a damaged header back to the value the carve wrote, so the next free of that slot is
/// an ordinary free instead of `0xC0000374`.
///
/// **This is the only write to client memory in this module, and it is off by default**
/// (`repair=on` in the marker). Everything else here reads.
///
/// # What it does and does not fix
///
/// It does not stop the writer. Every death in this family is the same event: the pool's free
/// reads all 64 bits of the size header, a non-zero high dword sends the block past the
/// `0x10/0x20/0x40/0x80` ladder to `HeapFree`, and `HeapFree` is handed a pointer Windows
/// never issued. Restoring the high dword to zero makes the slot the `0x20` slot it still
/// genuinely is, and the free goes back on the pool's own list.
///
/// So it converts a fatal free into a correct one. Three things it cannot do, stated because
/// a mitigation that is described as a fix will be trusted as one:
///
/// * **It is a race.** A write and a free inside one walk interval still dies. The observed
///   gaps are 720 ms once and minutes otherwise, against a 100 ms walk, but the guarantee is
///   statistical and nothing here makes it otherwise.
/// * **It only sees this shape.** The damage that killed the client on 2026-09-06 with
///   `0xC0000005` was a `-1` in the upper half of a *pointer* in a map node, not a pool
///   header (`research/heap-corruption-2026-09-06.md` §1.2). Same writer, most likely; not a
///   header, so not repairable here.
/// * **It changes what the writer sees.** If that dword is a reference count reached through
///   a stale pointer, zeroing it means the next increment starts from zero rather than
///   continuing. The `-1` already in the record says a decrement from zero does not kill the
///   client on its own, but that is an inference and it is written down as one.
///
/// # Why this is safe to write
///
/// Three conditions, all checked, and any one failing means nothing is written:
///
/// 1. the low dword equals this bucket's slot size exactly — so the qword really is a header
///    of the known shape and not some other object;
/// 2. the high dword is non-zero — there is something to repair;
/// 3. `VirtualQuery` says the four bytes are committed and writable — a store into a
///    `PAGE_READONLY` page would raise an access violation inside the client.
///
/// The store is a single aligned 32-bit write, which is atomic on x86-64, so no other thread
/// can observe a torn header. It is read back and the read-back is what the log reports:
/// a repair nobody verified is exactly the kind of claim `CLAUDE.md` is about.
unsafe fn repair_header(f: &Finding) -> Result<u64, String> {
    let slot = BUCKETS[f.bucket].slot as u64;
    if f.value & 0xFFFF_FFFF != slot {
        return Err(format!(
            "low dword {:#x} is not bucket {}'s slot size {slot:#x} - this is not the known \
             shape and the sentry will not guess at it",
            f.value & 0xFFFF_FFFF,
            f.bucket
        ));
    }
    if f.value >> 32 == 0 {
        return Err("high dword is already zero - nothing to repair".to_string());
    }
    let high = f.header_va + 4;
    if !crate::session::can_write(high, 4) {
        return Err(format!("{high:#x} is not committed and writable - NOT repaired"));
    }
    std::ptr::write_volatile(high as *mut u32, 0);
    let after = std::ptr::read_volatile(f.header_va as *const u64);
    if after != slot {
        return Err(format!(
            "the write did not take - {:#x} now reads {after:#018x}, wanted {slot:#018x}. \
             Treat every later reading of this slot as suspect",
            f.header_va
        ));
    }
    Ok(after)
}

/// Everything worth knowing about one confirmed finding, written to the log **before** the
/// dump is attempted.
///
/// The order is deliberate. A full-memory dump of this client is ~1.3 GB, takes seconds, and
/// is being taken from a process whose heap is known to be damaged — it can die partway, and
/// `crates/grap-stub/src/minidump.rs` exists partly to make "tried and died" distinguishable
/// from "never tried". So the snapshot reaches the log whatever happens to the dump, and the
/// dump's own outcome is logged in both directions.
///
/// # Safety
///
/// `f` must name a slot in a chunk `walk` admitted against `pool`.
unsafe fn report(
    pool: &Pool,
    walk: &BucketWalk,
    f: &Finding,
    armed_at: Instant,
    cfg: &Config,
) {
    let n = FINDINGS.fetch_add(1, Ordering::SeqCst) + 1;
    let age = armed_at.elapsed();
    let free = walk_free_list(pool, walk);
    let mut body = snapshot_text(pool, walk, f, n, age, cfg.interval, &free);
    if cfg.stacks {
        body.push_str("      THREADS (shallow stack scan, not an unwind - read as leads):\n");
        body.push_str(&thread_snapshot());
    }
    body.push_str("      *****");
    log(&body);

    // The dump last, and only after everything above is on disk.
    let written = DUMPS_WRITTEN.fetch_add(1, Ordering::SeqCst);
    if written >= cfg.max_dumps {
        log(&format!(
            "***** POOL SENTRY: no dump for finding #{n} - the cap of {} is already spent. \
             The snapshot above is the whole record *****",
            cfg.max_dumps
        ));
        return;
    }
    log(
        "***** POOL SENTRY: writing a full-memory dump ... the client is frozen while this \
         runs (seconds). A line saying 'writing' with no line after it means the dump attempt \
         died partway, which is not the same as never having tried *****",
    );
    let started = Instant::now();
    match write_pool_dump(&dump_dir(), &format!("finding{n}")) {
        Ok((path, size)) => log(&format!(
            "***** POOL SENTRY: wrote {path}, {size} bytes in {} ms. Run \
             `python tools/poolchain.py \"{path}\"` from C:\\MapleCW \
             *****",
            started.elapsed().as_millis()
        )),
        Err(why) => log(&format!(
            "***** POOL SENTRY: NO DUMP for finding #{n} - {why}. The snapshot logged above \
             is the whole record, and it is still a catch *****"
        )),
    }
}

/// The finding written out, with no I/O and no thread suspension, so a test can render it and
/// **read** it.
///
/// Split out of [`report`] deliberately. `CLAUDE.md` records a test plan that was maintained
/// for days in one of its two copies while the copy the owner actually reads went stale, and the
/// habit that catches it is *render the text and read it*, not *check that it compiles*. This
/// block is the entire product of a catch when the dump fails, so it gets the same treatment.
///
/// # Safety
///
/// `f` must name a slot in a chunk `walk` admitted against `pool`.
unsafe fn snapshot_text(
    pool: &Pool,
    walk: &BucketWalk,
    f: &Finding,
    n: u32,
    age: Duration,
    interval: Duration,
    free: &FreeList,
) -> String {
    let b = BUCKETS[f.bucket];
    let early = age < EARLY_SUSPECT;
    let mut body = String::new();
    body.push_str(&format!(
        "***** POOL SENTRY FINDING #{n}{}: bucket {} (slot {:#04x}) header {:#x} reads \
         {:#018x}\n      {}\n      caught {:.1} s after arming, so the slot's CONTENTS ARE AT \
         MOST {} ms OLD (one {} ms walk plus the confirm re-reads) - which is the whole point: \
         research/heap-wild-write.md \u{a7}6 calls a post-mortem's contents a dead end by \
         construction.\n      chunk {:#x}, position {} in \
         the chunk list (newest = 0), slot {}/{}, va%16={} -> if this slot is freed it will \
         raise heap failure type {}\n",
        if early { " [EARLY - SUSPECT]" } else { "" },
        f.bucket,
        b.slot,
        f.header_va,
        f.value,
        f.classify(),
        age.as_secs_f64(),
        interval.as_millis() + u128::from(CONFIRM_READS) * CONFIRM_SPACING.as_millis(),
        interval.as_millis(),
        f.chunk,
        f.chunk_age,
        f.index,
        b.per_chunk,
        f.header_va % 16,
        f.predicted_failure_type(),
    ));
    body.push_str(&format!(
        "      the slot body is {:#x}; the four bytes that changed are at {:#x}, which is \
         body-4. The 0x20 class is Ztl task memory with BSTR layout - a 4-byte byte-count at \
         body+0, and the pointer handed around is body+4 - so body-4 sits exactly on the \
         boundary between the pool's 8-byte convention and Ztl's 4-byte one.\n",
        f.body(),
        f.body() - 4,
    ));

    if early {
        body.push_str(
            "      READ THIS BEFORE BELIEVING IT. The archive has ZERO heap deaths under \
             192 s across 3 831 client-seconds of exposure in the 0-100 s band, and the \
             in-field rate predicts about 0.34 damaged slots by 100 s. A finding this early \
             is more likely to be this instrument than the client. Check the confirm re-reads \
             and the carve identity below before it goes anywhere near a write-up.\n",
        );
    }

    // The three hypotheses, and the state that discriminates them.
    body.push_str("      THE THREE SLOTS THAT MATTER, read now rather than at the next death:\n");
    if f.index > 0 {
        body.push_str(&describe_slot(
            f.header_va - b.stride(),
            &b,
            &free,
            "predecessor",
        ));
        body.push('\n');
    } else {
        body.push_str("      predecessor  (this is slot 0 of its chunk - none)\n");
    }
    body.push_str(&describe_slot(f.header_va, &b, &free, "DAMAGED"));
    body.push('\n');
    if f.index + 1 < b.per_chunk {
        body.push_str(&describe_slot(
            f.header_va + b.stride(),
            &b,
            &free,
            "successor",
        ));
        body.push('\n');
    }

    body.push_str(&format!(
        "      WHAT THIS DISCRIMINATES: the damaged slot on the free list means it was \
         damaged while it belonged to NOBODY (a damaged header can never be pushed - the free \
         reads it first and diverts to HeapFree), which kills 'the occupant underruns its own \
         buffer'. A LIVE predecessor supports the +0x24 overrun; a free one does not settle \
         it. A freshly-written body in the damaged slot supports the refcount/flag reading.\n"
    ));

    body.push_str(&format!(
        "      free list: {} entries, carved {} - freelist {} == served {} : {}{}\n",
        free.len,
        free.carved,
        free.len,
        free.served,
        if free.identity_holds {
            "[PASS]"
        } else {
            "[FAIL - read the next sentence]"
        },
        if free.stopped_early {
            ". The walk STOPPED EARLY on a pointer that is not a slot of any admitted chunk. \
             That is expected sometimes on live memory - the list mutates while it is read - \
             and it means the free-list answers above are 'not seen', not 'not free'"
        } else {
            ""
        }
    ));

    for i in 0..BUCKETS.len() {
        body.push_str(&format!(
            "      bucket {i}: lock owner {:#x} depth {}, carved {}, served {}, free head {:#x}\n",
            pool.lock_owner(i),
            pool.lock_depth(i),
            pool.carved(i),
            pool.served(i),
            pool.free_head(i),
        ));
    }
    let (walked, carved, holds) = walk.carve_identity(pool);
    body.push_str(&format!(
        "      instrument control for this bucket: {} chunks x {} = {walked} slots walked \
         against the allocator's own carved counter {carved} {}\n",
        walk.chunks(),
        b.per_chunk,
        if holds {
            "[PASS - the walk is not short]"
        } else {
            "[FAIL - THE WALK IS SHORT, treat everything above as a lower bound]"
        }
    ));
    body.push_str(&format!(
        "      heapfix marker: {:?} (research/heapfix-did-not-hold.md \u{a7}6: with it ON a \
         damaged slot CAN be pushed to the free list and the free-list argument above is \
         void)\n",
        crate::session::marker_token("heapfix=")
    ));
    body
}

// ---------------------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// A synthetic pool with the client's exact layout, built out of ordinary heap memory.
    ///
    /// This is what makes the sentry testable at all: the walk takes a context *address*
    /// rather than reading the hard-coded RVA, so a test can build a pool, damage it on
    /// purpose, and drive the real `discover` / `scan` / `confirm` / free-list code over it.
    /// Every guard in the module is exercised, including `VirtualQuery`, because this memory
    /// is really committed and really readable.
    struct FakePool {
        ctx: Vec<u64>,
        chunks: Vec<Vec<u8>>,
        bucket: usize,
    }

    impl FakePool {
        /// `n` chunks of bucket `bucket`, chained newest-first exactly as the client does.
        fn new(bucket: usize, n: usize) -> Self {
            let b = BUCKETS[bucket];
            let mut chunks = Vec::new();
            for _ in 0..n {
                // 8 bytes of size header below `base`, then the chunk itself.
                let mut buf = vec![0u8; 8 + b.chunk_bytes()];
                let base = unsafe { buf.as_mut_ptr().add(8) };
                unsafe {
                    // the large allocator's header: [base-8] = per_chunk*(slot+8)+8
                    std::ptr::write_unaligned(base.sub(8).cast::<u64>(), b.chunk_bytes() as u64);
                    // the carve loop: [body-8] = slot, for every slot
                    for k in 0..b.per_chunk {
                        std::ptr::write_unaligned(
                            base.add(8 + k * b.stride()).cast::<u64>(),
                            b.slot as u64,
                        );
                    }
                }
                chunks.push(buf);
            }
            let mut me = FakePool {
                ctx: vec![0u64; CTX_SPAN / 8 + 2],
                chunks,
                bucket,
            };
            me.relink();
            me
        }

        fn base(&self, i: usize) -> usize {
            self.chunks[i].as_ptr() as usize + 8
        }
        fn body0(&self, i: usize) -> usize {
            self.base(i) + 0x10
        }
        fn slot_header(&self, chunk: usize, k: usize) -> usize {
            self.base(chunk) + 8 + k * BUCKETS[self.bucket].stride()
        }
        fn ctx_addr(&self) -> usize {
            self.ctx.as_ptr() as usize
        }

        /// Chain the chunks and set the counters the allocator maintains.
        fn relink(&mut self) {
            let b = BUCKETS[self.bucket];
            let n = self.chunks.len();
            for i in 0..n {
                // push-front order: chunk 0 is the newest, and [base] points at the previous
                let prev = if i + 1 < n { self.body0(i + 1) as u64 } else { 0 };
                let base = self.base(i);
                unsafe { std::ptr::write_unaligned(base as *mut u64, prev) };
            }
            let ctx = self.ctx.as_mut_ptr() as usize;
            unsafe {
                let head = if n > 0 { self.body0(0) as u64 } else { 0 };
                std::ptr::write_unaligned(
                    (ctx + self.bucket * PTR_STRIDE + OFF_CHUNK_HEAD) as *mut u64,
                    head,
                );
                std::ptr::write_unaligned(
                    (ctx + self.bucket * COUNTER_STRIDE + OFF_CARVED) as *mut u32,
                    (n * b.per_chunk) as u32,
                );
                std::ptr::write_unaligned(
                    (ctx + self.bucket * COUNTER_STRIDE + OFF_SERVED) as *mut u32,
                    (n * b.per_chunk) as u32,
                );
            }
        }

        /// Plant the exact damage fourteen dumps have shown.
        fn damage(&mut self, chunk: usize, k: usize, value: u64) -> usize {
            let at = self.slot_header(chunk, k);
            unsafe { std::ptr::write_unaligned(at as *mut u64, value) };
            at
        }

        /// Build a free list over the given (chunk, slot) pairs, linked through the bodies,
        /// and set the served counter so `carved - freelist == served` holds.
        fn free_list(&mut self, slots: &[(usize, usize)]) {
            let ctx = self.ctx.as_mut_ptr() as usize;
            let bodies: Vec<usize> = slots
                .iter()
                .map(|(c, k)| self.slot_header(*c, *k) + 8)
                .collect();
            for i in 0..bodies.len() {
                let next = if i + 1 < bodies.len() { bodies[i + 1] } else { 0 };
                unsafe { std::ptr::write_unaligned(bodies[i] as *mut u64, next as u64) };
            }
            let head = bodies.first().copied().unwrap_or(0);
            unsafe {
                std::ptr::write_unaligned(
                    (ctx + self.bucket * PTR_STRIDE + OFF_FREE_HEAD) as *mut u64,
                    head as u64,
                );
                let carved = std::ptr::read_unaligned(
                    (ctx + self.bucket * COUNTER_STRIDE + OFF_CARVED) as *const u32,
                );
                std::ptr::write_unaligned(
                    (ctx + self.bucket * COUNTER_STRIDE + OFF_SERVED) as *mut u32,
                    carved - bodies.len() as u32,
                );
            }
        }
    }

    /// **The layout claim this module rests on, asserted against something that can
    /// disagree.**
    ///
    /// `tools/poolchain.py` read the per-bucket lock at `i*8 + 0x28`; the listing says
    /// `i*16 + 0x28`. This does not restate the constant — it derives it from two *other*
    /// offsets that were read independently: the counters end at `ctx+0x24` and the free-list
    /// heads begin at `ctx+0x68`, so the lock array is exactly `0x68 - 0x28 = 0x40` bytes for
    /// four buckets. Writing `LOCK_STRIDE = 8` fails this.
    #[test]
    fn the_lock_array_fills_the_gap_between_the_counters_and_the_free_heads() {
        let counters_end = 3 * COUNTER_STRIDE + OFF_SERVED + 4;
        assert_eq!(counters_end, 0x24, "the counters do not end where the listing says");
        assert!(counters_end <= OFF_LOCK);
        assert_eq!(
            BUCKETS.len() * LOCK_STRIDE,
            OFF_FREE_HEAD - OFF_LOCK,
            "four locks of {LOCK_STRIDE} bytes do not fill {:#x}..{:#x}",
            OFF_LOCK,
            OFF_FREE_HEAD
        );
    }

    /// The bucket table against the chunk sizes read off the client's own switch. A wrong
    /// `per_chunk` or `slot` moves every slot header this module reads.
    #[test]
    fn the_bucket_table_reproduces_the_chunk_sizes_from_the_listing() {
        assert_eq!(BUCKETS[0].chunk_bytes(), 0x608);
        assert_eq!(BUCKETS[1].chunk_bytes(), 0x508);
        assert_eq!(BUCKETS[2].chunk_bytes(), 0x488);
        assert_eq!(BUCKETS[3].chunk_bytes(), 0x448);
        assert_eq!(BUCKETS[DAMAGE_CLASS].slot, 0x20);
    }

    /// A clean pool must produce **nothing**. Without this the catch below proves only that
    /// the detector fires, not that it discriminates.
    #[test]
    fn a_clean_pool_produces_no_findings() {
        let pool = FakePool::new(DAMAGE_CLASS, 12);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe {
            assert_eq!(walk.discover(&p), WalkEnd::ListEnd);
            assert_eq!(walk.chunks(), 12);
            let (found, slots, suspect) = walk.scan();
            assert_eq!(slots, 12 * 32);
            assert!(suspect.is_empty());
            assert!(found.is_empty(), "clean pool reported {found:?}");
            let (walked, carved, holds) = walk.carve_identity(&p);
            assert_eq!((walked, carved), (384, 384));
            assert!(holds);
        }
    }

    /// **The planted write, caught end to end.**
    ///
    /// A sentry that has never caught a deliberately damaged pool is worth nothing. This
    /// drives the real `discover`, the real `scan`, the real `confirm` re-reads and the real
    /// classification over the exact value fourteen dumps have shown, and checks every field
    /// of the report that a write-up would quote.
    #[test]
    fn it_catches_a_planted_write_of_the_exact_value_the_dumps_show() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 40);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe {
            assert_eq!(walk.discover(&p), WalkEnd::ListEnd);
            assert!(walk.scan().0.is_empty(), "the pool was not clean to begin with");
        }
        // Chunk 17 is 17th from the head, slot 23 is odd, so the header sits 8 mod 16 and
        // ntdll's `test r13b,0xf` takes the misaligned arm: failure type 9.
        let at = pool.damage(17, 23, KNOWN_DAMAGE);
        unsafe {
            let (found, _, _) = walk.scan();
            assert_eq!(found.len(), 1, "expected exactly one finding, got {found:?}");
            let f = found[0];
            assert_eq!(f.header_va, at);
            assert_eq!(f.value, KNOWN_DAMAGE);
            assert_eq!(f.index, 23);
            assert_eq!(f.chunk_age, 17);
            assert_eq!(f.chunk, pool.base(17));
            assert_eq!(f.body(), at + 8);
            assert_eq!(f.predicted_failure_type(), 9, "slot 23 must be the type-9 arm");
            assert!(f.classify().starts_with("KNOWN FAMILY"));
            assert_eq!(
                walk.confirm(&f),
                Some(KNOWN_DAMAGE),
                "a stable planted write must survive the confirm re-reads"
            );
            // and it fires once
            walk.mark_reported(&f);
            assert!(walk.scan().0.is_empty(), "the same slot fired twice");
            // **But a repaired slot that is damaged AGAIN fires again.** Three of twelve did
            // on 2026-09-07 and the silence on the third killed the client.
            pool.damage(17, 23, 0x20);
            walk.forget_reported(at);
            assert!(walk.scan().0.is_empty(), "repaired and clean: nothing to report");
            pool.damage(17, 23, KNOWN_DAMAGE);
            let again = walk.scan().0;
            assert_eq!(again.len(), 1, "the re-hit is a new finding: {again:?}");
            assert_eq!(again[0].header_va, at);
        }
    }

    /// The even-index arm of the same prediction, so the parity rule is a claim rather than a
    /// coincidence of one example.
    #[test]
    fn an_even_slot_predicts_failure_type_eight() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 3);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe { walk.discover(&p) };
        pool.damage(1, 18, KNOWN_DAMAGE);
        let f = unsafe { walk.scan() }.0[0];
        assert_eq!(f.index, 18);
        assert_eq!(f.header_va % 16, 0);
        assert_eq!(f.predicted_failure_type(), 8);
    }

    /// A value nobody has seen must be caught **and labelled differently**. 14 of 14
    /// identical is the whole basis for "the writer is selective for 0x20 cells", and a
    /// detector that quietly folded a novel value into the known family would make that
    /// claim unfalsifiable.
    #[test]
    fn a_novel_value_is_caught_and_labelled_novel() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 4);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe { walk.discover(&p) };

        pool.damage(0, 5, 0x0000_0007_0000_0020);
        let f = unsafe { walk.scan() }.0[0];
        assert!(f.classify().starts_with("NOVEL - same size class"), "{}", f.classify());
        walk.mark_reported(&f);

        pool.damage(1, 6, 0x0000_0000_0000_0021);
        let f = unsafe { walk.scan() }.0[0];
        assert!(
            f.classify().starts_with("NOVEL - the low dword"),
            "a header whose low dword is not the slot size must say so: {}",
            f.classify()
        );
    }

    /// Damage outside the `0x20` class has never been seen — 0 of 693 272 slots — and would
    /// mean the writer targets an address rather than an object type. The sentry must be able
    /// to see it, or that claim can never come back false.
    #[test]
    fn damage_outside_bucket_one_is_visible() {
        for bucket in [0usize, 2, 3] {
            let mut pool = FakePool::new(bucket, 5);
            let p = Pool::at(pool.ctx_addr());
            let mut walk = BucketWalk::new(bucket);
            unsafe { walk.discover(&p) };
            assert!(unsafe { walk.scan() }.0.is_empty());
            let value = 0x0000_0001_0000_0000 | BUCKETS[bucket].slot as u64;
            pool.damage(2, 1, value);
            let found = unsafe { walk.scan() }.0;
            assert_eq!(found.len(), 1, "bucket {bucket} damage was invisible");
            assert_eq!(found[0].value, value);
        }
    }

    /// **The guard, proven to work rather than assumed.** A chunk-list link into memory that
    /// is reserved but not committed must end the walk, not fault the process. If the
    /// `VirtualQuery` guard were removed this test would crash the test runner rather than
    /// fail — which is exactly the failure mode it exists to prevent inside the owner's client.
    #[test]
    fn an_unmapped_chunk_pointer_ends_the_walk_instead_of_faulting() {
        const MEM_RESERVE: u32 = 0x2000;
        const PAGE_NOACCESS: u32 = 0x01;
        extern "system" {
            fn VirtualAlloc(addr: *mut c_void, size: usize, typ: u32, protect: u32)
                -> *mut c_void;
        }
        let reserved =
            unsafe { VirtualAlloc(std::ptr::null_mut(), 0x10000, MEM_RESERVE, PAGE_NOACCESS) }
                as usize;
        assert!(reserved != 0, "could not reserve a page to point at");

        let pool = FakePool::new(DAMAGE_CLASS, 3);
        // Point the newest chunk's link at reserved-but-uncommitted memory.
        unsafe { std::ptr::write_unaligned(pool.base(0) as *mut u64, (reserved + 0x1000) as u64) };
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        let end = unsafe { walk.discover(&p) };
        assert_eq!(end, WalkEnd::Unreadable(reserved + 0x1000 - 0x10));
        assert_eq!(walk.chunks(), 1, "only the head chunk should have been admitted");
        // and the slots it did admit are still scannable
        assert!(unsafe { walk.scan() }.0.is_empty());

        // **And it must RECOVER.** This is the half that matters, and it is why `discover`
        // re-walks the whole list instead of stopping at the first chunk it already knows: a
        // guard failure would otherwise truncate the list permanently, because every later
        // walk would stop at chunk 0 before ever reaching the repaired link. Restore the link
        // and the remaining chunks must be picked up.
        unsafe { std::ptr::write_unaligned(pool.base(0) as *mut u64, pool.body0(1) as u64) };
        assert_eq!(unsafe { walk.discover(&p) }, WalkEnd::ListEnd);
        assert_eq!(
            walk.chunks(),
            3,
            "the walk did not recover after the bad link was repaired - a transient guard \
             failure must not cost the rest of the list forever"
        );
    }

    /// A chunk whose size header is not the allocator's own identity is refused. This is the
    /// second half of the admission guard and it is what stops a garbage pointer that happens
    /// to land in readable memory from being scanned as 32 slots of noise.
    #[test]
    fn a_chunk_that_fails_the_size_identity_is_refused() {
        let pool = FakePool::new(DAMAGE_CLASS, 3);
        let bad = pool.base(1);
        unsafe { std::ptr::write_unaligned((bad - 8) as *mut u64, 0x1234u64) };
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        assert_eq!(unsafe { walk.discover(&p) }, WalkEnd::Identity(bad));
        assert_eq!(walk.chunks(), 1);
    }

    /// A cyclic chunk list must terminate. The client has never held more than ~2 700 chunks
    /// in a bucket, so this is a corruption case, and the sentry's own thread spinning
    /// forever inside a client the owner is trying to keep alive is not an acceptable answer.
    #[test]
    fn a_cyclic_chunk_list_terminates() {
        let pool = FakePool::new(DAMAGE_CLASS, 2);
        // last chunk points back at the head
        unsafe { std::ptr::write_unaligned(pool.base(1) as *mut u64, pool.body0(0) as u64) };
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        assert_eq!(unsafe { walk.discover(&p) }, WalkEnd::Overflow);
    }

    /// The free-list walk, its containment test, and the allocator's own identity as the
    /// control. `research/heap-corruption-2026-08-27.md` §4 turns on exactly this: a damaged
    /// slot **on** the free list was damaged while it belonged to nobody.
    #[test]
    fn the_free_list_walk_answers_membership_and_closes_its_identity() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 6);
        pool.free_list(&[(0, 1), (2, 5), (4, 31), (5, 0)]);
        let damaged_free = pool.damage(2, 5, KNOWN_DAMAGE);
        let damaged_live = pool.damage(3, 8, KNOWN_DAMAGE);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe {
            walk.discover(&p);
            let free = walk_free_list(&p, &walk);
            assert!(!free.stopped_early, "the walk should have reached the end");
            assert_eq!(free.len, 4);
            assert!(free.identity_holds, "carved - freelist == served must close");
            assert!(free.bodies.contains(&(damaged_free + 8)));
            assert!(!free.bodies.contains(&(damaged_live + 8)));
        }
    }

    /// A free-list link that is not a slot of any admitted chunk must stop the walk rather
    /// than be dereferenced. This is the property that makes walking a *live* free list safe
    /// at all: a node popped under us has had its body overwritten by its new owner.
    #[test]
    fn a_free_list_link_outside_the_pool_stops_the_walk_and_says_so() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 4);
        pool.free_list(&[(0, 1), (1, 2)]);
        let body = pool.slot_header(1, 2) + 8;
        // Point the second node at something that is not a slot body: one byte off.
        unsafe { std::ptr::write_unaligned(body as *mut u64, (pool.slot_header(2, 3) + 9) as u64) };
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe {
            walk.discover(&p);
            let free = walk_free_list(&p, &walk);
            assert!(free.stopped_early, "a bogus link must be refused, not chased");
            assert_eq!(free.len, 2);
            assert!(
                !free.identity_holds,
                "a short walk must NOT report a passing identity - that would be a clean, \
                 confident number from an instrument that did not finish"
            );
        }
    }

    /// The containment test itself, at both edges of a chunk and one past them.
    #[test]
    fn slot_containment_rejects_the_boundaries() {
        let pool = FakePool::new(DAMAGE_CLASS, 2);
        let b = BUCKETS[DAMAGE_CLASS];
        let sorted = {
            let mut v = vec![pool.base(0), pool.base(1)];
            v.sort_unstable();
            v
        };
        let first = pool.slot_header(0, 0) + 8;
        let last = pool.slot_header(0, 31) + 8;
        assert_eq!(slot_of_body(&sorted, &b, first), Some((pool.base(0), 0)));
        assert_eq!(slot_of_body(&sorted, &b, last), Some((pool.base(0), 31)));
        assert_eq!(slot_of_body(&sorted, &b, first + 1), None, "misaligned body accepted");
        assert_eq!(slot_of_body(&sorted, &b, last + b.stride()), None, "past the last slot");
        assert_eq!(slot_of_body(&sorted, &b, pool.base(0)), None, "the chunk header is not a slot");
        assert_eq!(slot_of_body(&sorted, &b, 0x10), None);
    }

    /// A chunk appended after the first walk must be picked up, and the whole list must be
    /// re-walked rather than stopped at the first known chunk — the early-stop design
    /// truncates the list permanently the first time a guard fails.
    #[test]
    fn new_chunks_are_picked_up_on_a_later_tick() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 3);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe { walk.discover(&p) };
        assert_eq!(walk.chunks(), 3);

        // The client pushes new chunks on the front. Rebuild with one more and re-walk.
        let extra = FakePool::new(DAMAGE_CLASS, 1);
        pool.chunks.insert(0, extra.chunks[0].clone());
        pool.relink();
        unsafe { assert_eq!(walk.discover(&p), WalkEnd::ListEnd) };
        assert_eq!(walk.chunks(), 4, "the new head chunk was not admitted");

        let at = pool.damage(0, 2, KNOWN_DAMAGE);
        let found = unsafe { walk.scan() }.0;
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].header_va, at);
    }

    /// A transient wrong value that does not persist must NOT be reported as damage. This is
    /// the carve-in-flight case: the only moment a `0x20` cell legitimately holds something
    /// other than `0x20`.
    #[test]
    fn a_value_that_does_not_persist_is_not_confirmed() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 2);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe { walk.discover(&p) };
        let at = pool.damage(0, 3, KNOWN_DAMAGE);
        let f = unsafe { walk.scan() }.0[0];
        // The carve loop finishes: the header goes back to the slot size.
        unsafe { std::ptr::write_unaligned(at as *mut u64, BUCKETS[DAMAGE_CLASS].slot as u64) };
        assert_eq!(unsafe { walk.confirm(&f) }, None, "a transient was confirmed as damage");
    }

    /// A value that *changes* between re-reads is not confirmed either — that is a live
    /// buffer being written, not a header that was scribbled once and abandoned.
    #[test]
    fn a_value_that_keeps_changing_is_not_confirmed() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 2);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe { walk.discover(&p) };
        let at = pool.damage(0, 4, 0xDEAD_BEEF_0000_0020);
        let f = unsafe { walk.scan() }.0[0];
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stop2 = stop.clone();
        // **The churn has to be RUNNING before `confirm` reads, or this test asserts
        // nothing.** It used to spawn the thread and call `confirm` immediately: under load -
        // two agents compiling in the same `target/` was enough - the writer got no time
        // slice between the scanner's reads, the value looked stable, and the test failed for
        // a reason that had nothing to do with the scanner. A test that can fail on
        // scheduling is not measuring what its name says.
        let writes = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let writes2 = writes.clone();
        let churn = std::thread::spawn(move || {
            let mut i = 1u64;
            while !stop2.load(Ordering::SeqCst) {
                unsafe { std::ptr::write_volatile(at as *mut u64, (i << 32) | 0x20) };
                writes2.fetch_add(1, Ordering::SeqCst);
                i = i.wrapping_add(1);
                std::thread::yield_now();
            }
        });
        while writes.load(Ordering::SeqCst) == 0 {
            std::thread::yield_now();
        }

        let got = unsafe { walk.confirm(&f) };
        let during = writes.load(Ordering::SeqCst);
        stop.store(true, Ordering::SeqCst);
        churn.join().unwrap();

        assert!(during > 0, "the churn never ran, so this run tested nothing");
        assert_eq!(got, None, "a churning value was confirmed as stable damage");
    }

    /// A missing pool context is a stand-down, not a fault.
    #[test]
    fn an_unreadable_context_is_reported_not_dereferenced() {
        let p = Pool::at(0);
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        assert_eq!(unsafe { walk.discover(&p) }, WalkEnd::NoContext);
        assert_eq!(walk.chunks(), 0);
    }

    /// The marker parser. `off` must win, and an unrecognised token must not disable the ones
    /// beside it.
    #[test]
    fn the_exact_marker_test_server_writes_arms_what_it_says_it_arms() {
        // `tools/test-server.ps1` composes this string from -SentryDumps, -SentryRepair and
        // -SentryWriteWatch. The two halves are in different languages and neither can fail
        // loudly: an unknown token here is ignored in silence, exactly as every other bad
        // token is, so a rename on one side would produce a run that armed nothing and looked
        // identical to a quiet client. That is the failure `CLAUDE.md` calls "count the same
        // event in two logs" - so this test IS the second log.
        let c = parse_config("dumps=4,repair=on,write=on").unwrap();
        assert_eq!(c.max_dumps, 4);
        assert!(c.repair, "-SentryRepair");
        assert!(c.write, "-SentryWriteWatch");
        // And the shipping default `crates/launcher/src/client.rs::SHIPPED_SENTRY`, which must
        // NOT turn the write watch on for an ordinary player.
        let shipped = parse_config("dumps=0,stacks=off,coarse=2000,repair=on").unwrap();
        assert!(shipped.repair);
        assert!(!shipped.write, "an ordinary launch must not protect the client's pool");
        assert!(!parse_config("").unwrap().write, "off by default");
        assert!(!parse_config("write").unwrap().write, "bare `write` is not `write=on`");
    }

    #[test]
    fn the_write_window_refuses_a_marker_typo_that_would_freeze_the_client() {
        // The window protects the hottest size class in the client. A marker asking for a
        // minute of that is a typo, and honouring it would be indistinguishable from the
        // client having become unplayable.
        let d = parse_config("").unwrap();
        assert_eq!(parse_config("writewindow=60000").unwrap().write_window, d.write_window);
        assert_eq!(parse_config("writewindow=0").unwrap().write_window, d.write_window);
        assert_eq!(parse_config("writewindow=wat").unwrap().write_window, d.write_window);
        assert_eq!(
            parse_config("writewindow=1500").unwrap().write_window,
            Duration::from_millis(1500)
        );
        assert_eq!(parse_config("writeburst=0").unwrap().write_burst, d.write_burst);
        assert_eq!(parse_config("writelead=99999").unwrap().write_lead, d.write_lead);
    }

    /// **A new catch re-anchors the clock, and the window must open again from it.** The run
    /// of 2026-09-07 21:57 had twelve catches and one window because the guard compared the
    /// firing index alone, and after every catch the next firing is index 1 again.
    #[test]
    fn a_window_opens_every_cycle_when_each_catch_re_anchors_the_clock() {
        // `window_due` is handed `now + lead`, as `run` hands it, so a window opens `lead`
        // before the firing: the moment `now + lead` crosses the firing instant.
        let p = Duration::from_secs(180);
        let lead = Duration::from_millis(500);
        let a = Instant::now() - Duration::from_secs(1000);
        let at = |anchor: Instant, since: Duration| anchor + since + lead;
        // Cycle one: nothing until `lead` before the first firing, then exactly once.
        assert_eq!(window_due(None, a, p, at(a, Duration::from_secs(100))), None);
        let k1 = window_due(None, a, p, at(a, p - lead + Duration::from_millis(1))).expect("opens");
        assert_eq!(k1.1, 1);
        assert_eq!(window_due(Some(k1), a, p, at(a, p - Duration::from_millis(100))), None, "not twice");
        // The catch at the firing re-anchors to `b`. Index 1 again - and it MUST open again.
        let b = a + p;
        assert_eq!(window_due(Some(k1), b, p, at(b, Duration::from_secs(10))), None, "too early");
        let k2 = window_due(Some(k1), b, p, at(b, p - lead + Duration::from_millis(1)))
            .expect("the bug: same index, new anchor, must open");
        assert_eq!(k2, (b, 1));
        assert_ne!(k1, k2);
        // A missed catch (no re-anchor) still steps to index 2 from the old anchor.
        let k3 = window_due(Some(k2), b, p, at(b, 2 * p - lead + Duration::from_millis(1))).expect("index 2");
        assert_eq!(k3.1, 2);
    }

    #[test]
    fn the_write_window_opens_once_per_firing_and_only_after_one_is_known() {
        let last = Instant::now() - Duration::from_secs(600);
        let p = Duration::from_secs(180);
        // Before the first firing, nothing. `run` requires an index of 1 or more.
        assert_eq!(firing_index(last, p, last), 0);
        assert_eq!(firing_index(last, p, last + Duration::from_secs(179)), 0);
        // One step per period, and it does not step twice inside one.
        assert_eq!(firing_index(last, p, last + Duration::from_secs(180)), 1);
        assert_eq!(firing_index(last, p, last + Duration::from_secs(359)), 1);
        assert_eq!(firing_index(last, p, last + Duration::from_secs(360)), 2);
        // A moment BEFORE the anchor cannot produce a window.
        assert_eq!(firing_index(last + Duration::from_secs(10), p, last), 0);
        assert_eq!(firing_index(last, Duration::ZERO, last + p), 0);
    }

    #[test]
    fn the_marker_parses_and_off_wins() {
        assert!(parse_config("").is_some());
        assert!(parse_config("on").is_some());
        assert!(parse_config("off").is_none());
        assert!(parse_config("interval=50,off").is_none());
        let c = parse_config("interval=250,full=4,heartbeat=30,dumps=2,stacks=off,wat").unwrap();
        assert_eq!(c.interval, Duration::from_millis(250));
        assert_eq!(c.full_every, 4);
        assert_eq!(c.heartbeat, Duration::from_secs(30));
        assert_eq!(c.max_dumps, 2);
        assert!(!c.stacks);
        // a silly interval must not be honoured - a 1 ms walk would be a busy loop
        assert_eq!(parse_config("interval=1").unwrap().interval, DEFAULT_INTERVAL);
        // **The coarse interval is compared against the fine one, so it must not depend on
        // token order.** Both spellings have to give the same answer or the flag is a trap.
        assert_eq!(
            parse_config("coarse=1000,interval=200").unwrap().coarse,
            parse_config("interval=200,coarse=1000").unwrap().coarse,
            "token order must not decide whether coarse is honoured"
        );
        assert_eq!(
            parse_config("coarse=1000").unwrap().coarse,
            Some(Duration::from_millis(1000))
        );
        assert!(parse_config("").unwrap().coarse.is_none(), "off by default");
        // shorter than the fine walk is a mistake, not an instruction to walk harder
        assert!(parse_config("interval=500,coarse=200").unwrap().coarse.is_none());
        assert!(parse_config("coarse=100").unwrap().coarse.is_none(), "equal is not greater");
        // and one long enough to lose the 720 ms corrupt-to-free race by a mile is refused
        assert!(parse_config("coarse=60000").unwrap().coarse.is_none());
        assert!(parse_config("coarse=wat").unwrap().coarse.is_none());
    }

    /// **The adaptive walk sleeps long only when nothing is due, and never before it has
    /// learned a period.** The whole point of the flag is to stop the walk evicting the
    /// client's cache ten times a second between firings; the whole risk of it is sleeping
    /// through one, so every case that could do that is pinned here.
    #[test]
    fn the_coarse_interval_only_applies_away_from_a_predicted_firing() {
        let fine = Duration::from_millis(100);
        let coarse = Duration::from_millis(2000);
        let cfg = Config {
            interval: fine,
            coarse: Some(coarse),
            ..Config::default()
        };
        let period = Duration::from_secs(180);

        // Nothing learned yet: always fine, whatever else is set.
        assert_eq!(nap_for(&cfg, None, None), fine);
        assert_eq!(nap_for(&cfg, Some(Instant::now()), None), fine);

        // Just fired: inside the window, so fine.
        assert_eq!(nap_for(&cfg, Some(Instant::now()), Some(period)), fine);

        // Well away from a firing: coarse, and never long enough to overshoot the next one.
        let long_ago = Instant::now() - Duration::from_secs(90);
        let nap = nap_for(&cfg, Some(long_ago), Some(period));
        assert!(nap > fine, "90 s into a 180 s period is nowhere near a firing");
        assert!(nap <= coarse);

        // Approaching the next firing: back to fine before it lands.
        let nearly = Instant::now() - Duration::from_secs(177);
        assert_eq!(nap_for(&cfg, Some(nearly), Some(period)), fine);

        // A coarse nap must never carry past the window's edge, at any phase.
        for secs in 6..175u64 {
            let at = Instant::now() - Duration::from_secs(secs);
            let nap = nap_for(&cfg, Some(at), Some(period));
            let remaining = 180.0 - secs as f64 - FIRE_WINDOW.as_secs_f64();
            assert!(
                nap.as_secs_f64() <= remaining.max(0.0) + 0.05 || nap == fine,
                "at {secs}s the nap {nap:?} would sleep past the window"
            );
        }

        // With the flag off, nothing changes however much is known.
        let off = Config { interval: fine, ..Config::default() };
        assert_eq!(nap_for(&off, Some(long_ago), Some(period)), fine);
        // A zero period must not divide by zero or pin the walk coarse.
        assert_eq!(nap_for(&cfg, Some(long_ago), Some(Duration::ZERO)), fine);
        // **Repair is the one option that writes to the client, so it must be OFF unless the
        // marker says exactly `repair=on`.** A near miss enabling it would be the worst
        // possible failure of this parser.
        assert!(!parse_config("").unwrap().repair, "default must not write");
        assert!(!parse_config("dumps=4").unwrap().repair);
        assert!(!parse_config("repair").unwrap().repair, "bare `repair` is not `repair=on`");
        assert!(!parse_config("repair=off").unwrap().repair);
        assert!(!parse_config("repair=1").unwrap().repair);
        assert!(parse_config("dumps=4,repair=on").unwrap().repair);
        assert!(parse_config("REPAIR=ON").unwrap().repair, "the marker is lower-cased first");
        // and `off` still disarms the whole thing even beside a repair request
        assert!(parse_config("repair=on,off").is_none());
    }

    /// **The repair, driven against a real damaged header**, for the same reason the dump
    /// writer is driven end to end: this is the only code in the crate that writes to memory
    /// the client owns, and a repair that has never repaired anything is exactly the
    /// instrument `CLAUDE.md` warns about.
    ///
    /// It also pins the three refusals, because each one is a case where writing would be
    /// wrong rather than merely useless.
    #[test]
    fn the_repair_restores_a_header_and_refuses_everything_else() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 3);
        let at = pool.damage(1, 7, KNOWN_DAMAGE);
        let f = Finding {
            bucket: DAMAGE_CLASS,
            chunk: 0,
            chunk_age: 0,
            index: 7,
            header_va: at,
            value: KNOWN_DAMAGE,
        };

        // The positive control first: the damage is really there before the repair.
        assert_eq!(unsafe { std::ptr::read_volatile(at as *const u64) }, KNOWN_DAMAGE);
        let now = unsafe { repair_header(&f) }.expect("a known-family header must be repairable");
        assert_eq!(now, 0x20, "the header must read exactly the slot size afterwards");
        assert_eq!(
            unsafe { std::ptr::read_volatile(at as *const u64) },
            0x20,
            "and it must still read that when re-read independently"
        );

        // Refusal 1: nothing to do. Idempotent rather than a second write.
        let clean = Finding { value: 0x20, ..f };
        assert!(unsafe { repair_header(&clean) }.is_err());

        // Refusal 2: the low dword is not this bucket's slot size, so the qword is not a
        // header of the known shape and guessing at it could corrupt a live object.
        let wrong_class = Finding {
            value: 0x0000_0001_0000_0040,
            ..f
        };
        let why = unsafe { repair_header(&wrong_class) }.unwrap_err();
        assert!(why.contains("slot size"), "{why}");
        assert_eq!(
            unsafe { std::ptr::read_volatile(at as *const u64) },
            0x20,
            "a refused repair must not have written anything"
        );

        // Refusal 3: unwritable memory. PAGE_NOACCESS rather than a bogus pointer, so the
        // guard is what refuses - without it this test would fault the runner.
        const MEM_RESERVE: u32 = 0x2000;
        const PAGE_NOACCESS: u32 = 0x01;
        extern "system" {
            fn VirtualAlloc(addr: *mut std::ffi::c_void, size: usize, typ: u32, prot: u32)
                -> *mut std::ffi::c_void;
        }
        let dead = unsafe { VirtualAlloc(std::ptr::null_mut(), 0x1000, MEM_RESERVE, PAGE_NOACCESS) };
        assert!(!dead.is_null());
        let unwritable = Finding {
            header_va: dead as usize,
            ..f
        };
        let why = unsafe { repair_header(&unwritable) }.unwrap_err();
        assert!(why.contains("NOT repaired"), "{why}");
    }

    /// **The dump writer, driven end to end**, for the same reason
    /// `crates/grap-stub/src/minidump.rs` drives its own: a dump writer that has never
    /// written a dump is exactly the instrument `CLAUDE.md` warns about. This calls the real
    /// `LoadLibraryA`, the real `GetProcAddress` and the real `MiniDumpWriteDump`, and checks
    /// a readable minidump comes out.
    ///
    /// **Render the finding and read it.**
    ///
    /// `CLAUDE.md` records a test plan maintained for days in the copy nobody reads while the
    /// copy the owner actually sees went stale, and the habit that catches it is *render the text
    /// and read it*. This block is the entire product of a catch when the dump fails, so it
    /// gets the same treatment: a pool damaged on purpose, the real snapshot rendered, and
    /// the load-bearing claims asserted rather than eyeballed.
    ///
    /// Run `cargo test -p grap-stub --lib the_finding_renders -- --nocapture` to read it.
    #[test]
    fn the_finding_renders_and_says_what_it_discriminates() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 9);
        // Slot 12 is the damaged one and it is on the free list; slot 11, its predecessor, is
        // NOT - so this exercises both halves of the discrimination paragraph.
        pool.free_list(&[(4, 12), (7, 3)]);
        let at = pool.damage(4, 12, KNOWN_DAMAGE);
        // Give the predecessor a BSTR body, which is what the 0x20 class actually holds.
        let pred_body = pool.slot_header(4, 11) + 8;
        unsafe {
            std::ptr::write_unaligned(pred_body as *mut u32, 12u32); // 6 UTF-16 chars
            for (i, ch) in "origin".chars().enumerate() {
                std::ptr::write_unaligned(
                    (pred_body + 4 + i * 2) as *mut u16,
                    ch as u16,
                );
            }
        }
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        let text = unsafe {
            walk.discover(&p);
            let f = walk.scan().0[0];
            assert_eq!(f.header_va, at);
            let free = walk_free_list(&p, &walk);
            snapshot_text(
                &p,
                &walk,
                &f,
                1,
                Duration::from_secs(412),
                Duration::from_millis(100),
                &free,
            )
        };
        println!("\n{text}\n");

        // The claims a write-up would quote, each asserted rather than assumed.
        assert!(text.contains("POOL SENTRY FINDING #1"));
        assert!(!text.contains("[EARLY - SUSPECT]"), "412 s is not early");
        assert!(text.contains("caught 412.0 s after arming"), "the age must read in seconds");
        assert!(text.contains("KNOWN FAMILY"));
        assert!(text.contains("failure type 8"), "slot 12 is even -> the aligned arm");
        // 100 ms of walk interval plus 3 confirm re-reads 2 ms apart. Not "100" — the
        // instrument's own latency has to be stated honestly, including the part it adds.
        assert!(
            text.contains("CONTENTS ARE AT MOST 106 ms OLD (one 100 ms walk"),
            "the staleness bound must include the confirm re-reads: {text}"
        );
        assert!(text.contains(&format!("{:#x}", at)), "the header address must appear");
        assert!(text.contains(&format!("{:#x}", at + 8 - 4)), "body-4 must be named");
        assert!(text.contains("free-list=YES"), "the damaged slot is on the free list");
        assert!(text.contains("free-list=no"), "the predecessor is not");
        assert!(text.contains("\"origin\""), "the BSTR body was not decoded: {text}");
        assert!(text.contains("[PASS - the walk is not short]"));
        assert!(text.contains("WHAT THIS DISCRIMINATES"));
        assert!(text.contains("heapfix marker"));
    }

    /// The same finding at 30 s must carry the EARLY warning. The archive has zero heap
    /// deaths under 192 s, so a catch that early is more likely to be the instrument.
    #[test]
    fn an_early_finding_is_flagged_as_suspect() {
        let mut pool = FakePool::new(DAMAGE_CLASS, 3);
        pool.damage(1, 4, KNOWN_DAMAGE);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        let text = unsafe {
            walk.discover(&p);
            let f = walk.scan().0[0];
            let free = walk_free_list(&p, &walk);
            snapshot_text(
                &p,
                &walk,
                &f,
                1,
                Duration::from_secs(30),
                Duration::from_millis(100),
                &free,
            )
        };
        assert!(text.contains("[EARLY - SUSPECT]"), "30 s must be flagged");
        assert!(text.contains("ZERO heap deaths under 192 s"));
    }

    /// **What one walk costs at the client's real scale**, printed so the number is measured
    /// rather than asserted from a guess.
    ///
    /// 2 273 chunks is bucket 1 of `dumps\maplecw-crash-1224132-c0000374-1.dmp`, the largest
    /// this project has enumerated, and 72 736 slots is what `tools/poolchain.py` counts in
    /// it. The assertion is deliberately loose — this is a timing test and a tight bound
    /// would flake — but it is not decorative: anything accidentally quadratic in the chunk
    /// count blows straight through it.
    ///
    /// It is a **lower bound** on the real cost. These chunks come from one allocation run so
    /// they are near each other; the client's are scattered across a 1.3 GB address space and
    /// will miss cache more. The heartbeat logs the real figure every 60 s of a live run, so
    /// nobody has to trust this one.
    ///
    /// `cargo test -p grap-stub --lib a_full_size_walk -- --nocapture` to read it.
    #[test]
    fn a_full_size_walk_costs_microseconds_not_milliseconds() {
        let pool = FakePool::new(DAMAGE_CLASS, 2273);
        let p = Pool::at(pool.ctx_addr());
        let mut walk = BucketWalk::new(DAMAGE_CLASS);
        unsafe { walk.discover(&p) };
        assert_eq!(walk.chunks(), 2273);

        let mut slots = 0;
        let started = std::time::Instant::now();
        for _ in 0..20 {
            let (found, n, _) = unsafe { walk.scan() };
            assert!(found.is_empty());
            slots = n;
        }
        let per_walk = started.elapsed() / 20;
        assert_eq!(slots, 72_736, "the slot count must match poolchain.py on dump 1224132");
        println!(
            "one bucket-1 walk: {} chunks, {slots} slots, {:?} per walk ({} walks/s)",
            walk.chunks(),
            per_walk,
            (1.0 / per_walk.as_secs_f64()) as u64
        );
        assert!(
            per_walk < Duration::from_millis(50),
            "a walk took {per_walk:?}; at a 100 ms interval that is a real share of a core, \
             and it smells like something quadratic in the chunk count"
        );
    }

    /// Where the child half of the dump self-test writes.
    const SELFTEST_DIR_ENV: &str = "MAPLECW_POOLSENTRY_SELFTEST_DIR";

    /// The half that actually calls dbghelp. `#[ignore]`d so it runs **only** when the parent
    /// below re-execs this binary for it, alone.
    ///
    /// It has to be alone. `MiniDumpWriteDump` suspends every other thread in the process and
    /// then allocates, so two concurrent callers deadlock — measured at 1 hang in 4 full-suite
    /// runs with this module's self-test and `minidump.rs`'s both in the pool, against 0 in 12
    /// with either one alone. Running it in its own process is not tidiness; it is the same
    /// invariant [`DUMP_LOCK`] asks for in production, enforced the one way a module that does
    /// not own `minidump.rs` can enforce it.
    #[test]
    #[ignore = "driven by it_actually_writes_a_readable_minidump in a child process"]
    fn dump_writer_child() {
        let dir = std::env::var(SELFTEST_DIR_ENV).expect("the parent must set the scratch dir");
        let (path, size) =
            unsafe { write_pool_dump(&dir, "selftest") }.expect("the dump must be written");
        println!("child wrote {path} ({size} bytes)");
    }

    /// **The dump writer, driven end to end**, for the same reason
    /// `crates/grap-stub/src/minidump.rs` drives its own: a dump writer that has never written
    /// a dump is exactly the instrument `CLAUDE.md` warns about. The real `LoadLibraryA`, the
    /// real `GetProcAddress`, the real `MiniDumpWriteDump`, the production flags — and a
    /// readable minidump has to come out of it.
    ///
    /// The scratch directory is **per-pid**, deliberately: `minidump.rs`'s self-test uses a
    /// fixed path and flakes when two `cargo test` processes run at once. Setting the shared
    /// `DUMP_DIR_ENV` is worse still — this test did that at first and it redirected *that*
    /// test's dump into this one's scratch directory, which then deleted it.
    #[test]
    fn it_actually_writes_a_readable_minidump() {
        let dir = std::env::temp_dir().join(format!(
            "maplecw-poolsentry-selftest-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch dir");

        let exe = std::env::current_exe().expect("the test binary's own path");
        let out = std::process::Command::new(exe)
            .args([
                "--exact",
                "poolsentry::tests::dump_writer_child",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(SELFTEST_DIR_ENV, &dir)
            .output()
            .expect("re-exec the test binary for the dump child");
        assert!(
            out.status.success(),
            "the dump child failed.\n--- stdout ---\n{}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );

        let dumps: Vec<_> = std::fs::read_dir(&dir)
            .expect("scratch dir")
            .filter_map(Result::ok)
            .map(|e| e.path())
            .collect();
        assert_eq!(dumps.len(), 1, "expected one dump in {dir:?}, got {dumps:?}");
        let bytes = std::fs::read(&dumps[0]).expect("read the dump back");
        assert_eq!(&bytes[..4], b"MDMP", "not a minidump: {:?}", &bytes[..4]);
        // **4 MB, not 64 KB.** 64 KB was the first threshold and it did not discriminate:
        // deleting `MINIDUMP_WITH_FULL_MEMORY` from the flags still produced a dump that
        // passed it, so the assertion said nothing about the one bit that matters. The
        // measured full-memory dump of this test binary is ~23 MB; without full memory it is
        // a few hundred KB. The heap is the entire subject, so a dump without it is not a
        // dump this module can use.
        assert!(
            bytes.len() > 4 * 1024 * 1024,
            "dump is {} bytes - far too small to be a full-memory dump, so the heap is not \
             in it and the file is useless for this investigation",
            bytes.len()
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Full memory is the bit that carries the heap, and the heap is the entire subject. A
    /// dump without it could not answer the question the sentry exists to ask.
    #[test]
    fn the_dump_type_includes_full_memory() {
        assert_eq!(DUMP_FLAGS & MINIDUMP_WITH_FULL_MEMORY, MINIDUMP_WITH_FULL_MEMORY);
        assert_eq!(DUMP_FLAGS, 0x1826, "the sentry's dump type drifted from the crash dump's");
    }

    /// The fallback chain for the dump directory must never produce an empty string —
    /// `CreateFileW` rejects one, and that would look exactly like the sentry refusing to
    /// dump.
    #[test]
    fn the_dump_directory_is_never_empty() {
        assert!(!dump_dir().is_empty());
    }

    /// **The x64 `CONTEXT` offsets, asserted against something that can disagree.**
    ///
    /// `CLAUDE.md`: a constant that came from reading a header is a claim, not a fact — the
    /// `MINIDUMP_EXCEPTION_INFORMATION` test next door passed for weeks while the layout was
    /// wrong. So rather than restating `0xF8` and `0x98`, this captures a real context and
    /// checks the field at `RIP` is an address inside this module and the field at `RSP` is
    /// within a megabyte of a local. Both fail if the offsets are wrong.
    #[test]
    fn the_context_offsets_name_rip_and_rsp() {
        #[repr(C, align(16))]
        struct Ctx([u8; 1232]);
        extern "system" {
            fn RtlCaptureContext(ctx: *mut c_void);
        }
        let mut ctx = Ctx([0; 1232]);
        ctx.0[CTX_FLAGS_OFF..CTX_FLAGS_OFF + 4].copy_from_slice(&0x0010_003fu32.to_le_bytes());
        let local = 0u64;
        unsafe { RtlCaptureContext(std::ptr::addr_of_mut!(ctx).cast()) };
        let rip = usize::from_le_bytes(ctx.0[CTX_RIP_OFF..CTX_RIP_OFF + 8].try_into().unwrap());
        let rsp = usize::from_le_bytes(ctx.0[CTX_RSP_OFF..CTX_RSP_OFF + 8].try_into().unwrap());
        let here = the_context_offsets_name_rip_and_rsp as *const () as usize;
        assert!(
            rip.abs_diff(here) < 0x1000_0000,
            "the field at {CTX_RIP_OFF:#x} is {rip:#x}, nowhere near this function at {here:#x}"
        );
        assert!(
            rsp.abs_diff(std::ptr::addr_of!(local) as usize) < 0x10_0000,
            "the field at {CTX_RSP_OFF:#x} is {rsp:#x}, not near the stack local at {:#x}",
            std::ptr::addr_of!(local) as usize
        );
    }
}
