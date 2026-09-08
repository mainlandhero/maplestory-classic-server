//! A write watch on the pool's `0x20` chunk pages, armed for half a second either side of a
//! **predicted** firing of the 180-second clock, so the store that damages a slot header
//! faults *at the writing instruction*.
//!
//! # Why this exists
//!
//! `poolsentry` answers *when*. It cannot answer *who*, and its own docs say so
//! (`research/heap-corruption-2026-09-06.md` §5.4): a 100 ms walk finds the damaged slot long
//! after the thread that wrote it has parked, and all 68 threads sampled at both catches were
//! in `ntdll` waits. No tuning of a poller changes that. The only instrument that produces a
//! *who* is one that faults on the write itself.
//!
//! `heap-corruption-2026-09-06.md` §3.2 proposed one: replace bucket 1's allocator so every
//! freed slot is its own decommitted page. This is **not** that build, and the reason is a
//! measurement rather than a preference:
//!
//! * **Two of the four catches on 2026-09-07 were LIVE slots**, not free ones
//!   (`the-180-second-clock-2026-09-07.md` §2, and §5.2 of the file above: *"the writer holds a
//!   pointer of its own and writes through it regardless of the slot's state"*). A
//!   decommit-on-free scheme is blind to half of them by construction.
//! * The damage is at `body − 4`, which is **inside the pool header**, and the pool must keep
//!   writing that header to function. Page protection is page-granular, so no layout separates
//!   "the allocator's own carve" from "the writer" while the slot is live.
//! * It replaces the client's hottest allocator with an inline hook. This module writes
//!   nothing, patches nothing, and does not sit in any client code path.
//!
//! What makes a cheaper instrument possible is the thing 2026-09-07 established: **the period
//! is 180.000 s and it re-arms on the firing branch**, so after one catch the next write is
//! predictable to a few hundred milliseconds. Protecting the whole size class for the whole
//! session would be unusable. Protecting it for 1.2 s every three minutes is a stutter.
//!
//! # How it works
//!
//! 1. `poolsentry` hands over the sorted chunk bases of bucket 1 when a predicted firing is
//!    about `lead` milliseconds away.
//! 2. Every page **fully contained** inside a chunk span goes to `PAGE_READONLY`. Reads are
//!    untouched, so the sentry's own walk and the client's string reads carry on.
//! 3. A write faults. The vectored handler classifies the address against the chunk table:
//!    a slot **header** write is the event; a body write is the noise the class is full of.
//!    Either way the page is made writable again and the instruction re-executes, so the
//!    client makes progress.
//! 4. Because a page stays writable once it has been touched, the armer **re-protects on a
//!    `burst` cadence**. Cost is bounded by the page count, not by the write rate.
//! 5. The window closes, every run goes back to `PAGE_READWRITE`, and the sentry logs what
//!    was recorded. Nothing is logged from the handler - see below.
//!
//! # What the handler may not do
//!
//! It runs on a client thread that may hold the pool's spinlock or the CRT heap lock. So it
//! **allocates nothing, takes no lock, opens no file and calls no logging**. It writes into
//! fixed static rings with atomics and returns. `hook::log` formats a `String` and opens the
//! log file; calling it from here is how this module would deadlock the client instead of
//! diagnosing it. The sentry thread drains the rings.
//!
//! # The instrument's own control
//!
//! `CLAUDE.md`: prove a search can find a positive control before reporting that it found
//! nothing - and `minidump.rs` is the standing example of a constant read out of a header
//! being wrong. The three claims this module rests on are all *header* claims:
//! `EXCEPTION_RECORD.NumberParameters` at `0x18`, `ExceptionInformation` at `0x20`, and
//! `ExceptionInformation[0] == 1` meaning a write.
//!
//! [`self_test`] asserts all three against something that can disagree: it allocates one page
//! of its own, protects it, stores to a known offset, and requires the handler to have
//! reported **that exact address**. It runs once, before the first arming, and a failure
//! disarms the module and says so in the log. A silent watch that reports nothing because it
//! decoded the wrong field is precisely the failure this repo keeps paying for.
//!
//! # Known limits, stated
//!
//! * **A kernel write into a protected page returns an error to the client instead of
//!   faulting.** `0x20` slots are short UTF-16 strings and small nodes, not I/O buffers, and
//!   the window is under a second - but this is the one way this module could change client
//!   behaviour rather than only observe it, and it is why the window is short and opt-in.
//! * **A page already written this burst is blind until the next re-protect.** Bounded by
//!   `burst`, and the re-protect cadence is the knob.
//! * **It names the instruction, not the object.** A store from a shared `memcpy` would give
//!   a CRT address; the follow-up in that case is `probe.rs`'s existing `watch@<rip>`, which
//!   already prints registers and a stack.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use crate::poolsentry::{Bucket, BUCKETS, DAMAGE_CLASS};

// ---------------------------------------------------------------------------------------
// Windows
// ---------------------------------------------------------------------------------------

const PAGE_BYTES: usize = 0x1000;
const PAGE_READONLY: u32 = 0x02;
const PAGE_READWRITE: u32 = 0x04;
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_RELEASE: u32 = 0x8000;

const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
const EXCEPTION_ACCESS_VIOLATION: u32 = 0xC000_0005;
/// The trap after a single-stepped instruction. `probe.rs` uses the same three constants to
/// re-plant its `int3` one instruction later; here they re-protect a page one instruction
/// after the write that opened it.
const EXCEPTION_SINGLE_STEP: u32 = 0x8000_0004;
const CTX_EFLAGS: usize = 0x44;
const TRAP_FLAG: u32 = 0x100;

/// `ExceptionInformation[0]` for a write. `0` is a read, `8` a DEP violation.
const AV_WRITE: usize = 1;

/// Offsets into the x64 `EXCEPTION_RECORD`, and every one of them is a claim read out of
/// `winnt.h` rather than measured. [`self_test`] is what turns them into facts.
const REC_CODE: usize = 0x00;
const REC_PARAMS: usize = 0x18;
const REC_INFO: usize = 0x20;

#[repr(C)]
#[derive(Default)]
struct MemoryBasicInformation {
    base: *mut c_void,
    allocation_base: *mut c_void,
    allocation_protect: u32,
    _align1: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    kind: u32,
    _align2: u32,
}

#[repr(C)]
struct ExceptionPointers {
    record: *mut u8,
    context: *mut c_void,
}

extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: *const c_void) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
    fn VirtualQuery(addr: *const c_void, buf: *mut MemoryBasicInformation, len: usize) -> usize;
    fn VirtualAlloc(addr: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
    fn VirtualFree(addr: *mut c_void, size: usize, kind: u32) -> i32;
    fn GetCurrentThreadId() -> u32;
    fn GetTickCount64() -> u64;
}

// ---------------------------------------------------------------------------------------
// Sizing
// ---------------------------------------------------------------------------------------

/// Bucket 1 held 2 574 chunks in the longest measured session. Six times that is a ceiling,
/// not a budget: the table is 128 KB of `.bss` and is never grown.
const MAX_CHUNKS: usize = 16_384;
/// Chunks are contiguous inside a segment, so the page runs coalesce to roughly one per 16 KB
/// segment - about 230 for a full bucket.
const MAX_RUNS: usize = 4_096;
/// Header writes recorded in full. The event is one per three minutes.
const MAX_HITS: usize = 32;
/// Write faults of ANY kind kept - the LAST this many, as a ring. The liveness control, and
/// since run 4 the record of what wrote to a page in the moments before a header write on it:
/// a window that records zero of these did not watch anything, which is a different result
/// from "the writer did not fire".
const MAX_SEEN: usize = 64;
/// Chunks whose spans are within this many bytes of each other are merged into one run. The
/// big allocator puts its own block header between segments; merging across it costs nothing
/// (those bytes fault, get unprotected and are counted as edge writes) and saves the run
/// table from one entry per chunk.
const MERGE_GAP: usize = 0x40;
/// Stop re-protecting inside a window once it has handled this many faults.
///
/// The cost of a window is bounded by `pages x sweeps`, not by the client's write rate - a
/// page that has been written stays writable until the next sweep. That bound has never been
/// measured against a live client, and the failure mode if it is wrong is the one the owner already
/// complained about in a milder form: *"it lags/freezes the client every time it runs"*. So the
/// window backs off and says so, rather than discovering the number the expensive way. A
/// window that trips this is still a result - it says the `0x20` class is written far harder
/// than assumed, which is worth knowing on its own.
const MAX_FAULTS_PER_WINDOW: u64 = 20_000;

// ---------------------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------------------

const ZERO_USIZE: AtomicUsize = AtomicUsize::new(0);
const ZERO_U64: AtomicU64 = AtomicU64::new(0);

/// Sorted chunk bases of the watched bucket. Written only while [`WATCHING`] is false.
static CHUNKS: [AtomicUsize; MAX_CHUNKS] = [ZERO_USIZE; MAX_CHUNKS];
static CHUNK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Page runs currently protected, as `(start, len)`. Same publication rule as [`CHUNKS`].
static RUN_START: [AtomicUsize; MAX_RUNS] = [ZERO_USIZE; MAX_RUNS];
static RUN_LEN: [AtomicUsize; MAX_RUNS] = [ZERO_USIZE; MAX_RUNS];
static RUN_COUNT: AtomicUsize = AtomicUsize::new(0);

/// The handler does nothing at all unless this is set. It is the first thing read and the
/// last thing published, so a partially-filled table is never consulted.
static WATCHING: AtomicBool = AtomicBool::new(false);
/// An armer is running. Guards against two windows overlapping.
static ACTIVE: AtomicBool = AtomicBool::new(false);
/// Asks the armer to close the window early - `poolsentry` sets it before a repair, which
/// needs the pages writable.
static STOP: AtomicBool = AtomicBool::new(false);

static VEH_REGISTERED: AtomicBool = AtomicBool::new(false);
static SELF_TEST_DONE: AtomicBool = AtomicBool::new(false);
static SELF_TEST_OK: AtomicBool = AtomicBool::new(false);
/// The self-test's own page, and the address it stores to. Read by the handler.
static PROBE_PAGE: AtomicUsize = AtomicUsize::new(0);
static PROBE_SAW: AtomicUsize = AtomicUsize::new(0);
static PROBE_KIND: AtomicUsize = AtomicUsize::new(usize::MAX);

/// Counters. All cumulative for the process; the heartbeat prints them.
static WINDOWS: AtomicU32 = AtomicU32::new(0);
static FAULTS: AtomicU64 = AtomicU64::new(0);
static BODY_WRITES: AtomicU64 = AtomicU64::new(0);
static EDGE_WRITES: AtomicU64 = AtomicU64::new(0);
static HEADER_WRITES: AtomicU64 = AtomicU64::new(0);
/// Set by the handler when a header write lands; the sentry thread turns it into a dump.
static DUMP_WANTED: AtomicBool = AtomicBool::new(false);
/// Windows that hit [`MAX_FAULTS_PER_WINDOW`] and stopped re-protecting early.
static BACKED_OFF: AtomicU32 = AtomicU32::new(0);

struct Hit {
    rip: AtomicU64,
    target: AtomicU64,
    slot_header: AtomicU64,
    tid: AtomicU32,
    /// Byte offset inside the 8-byte slot header. The family writes at **4**.
    inside: AtomicU32,
    index: AtomicU32,
    tick: AtomicU64,
}

const HIT_ZERO: Hit = Hit {
    rip: AtomicU64::new(0),
    target: AtomicU64::new(0),
    slot_header: AtomicU64::new(0),
    tid: AtomicU32::new(0),
    inside: AtomicU32::new(0),
    index: AtomicU32::new(0),
    tick: AtomicU64::new(0),
};

static HITS: [Hit; MAX_HITS] = [HIT_ZERO; MAX_HITS];
static HIT_COUNT: AtomicUsize = AtomicUsize::new(0);
static HITS_DRAINED: AtomicUsize = AtomicUsize::new(0);

/// `(rip, target)` for the first few write faults of any kind. The liveness control.
static SEEN_RIP: [AtomicU64; MAX_SEEN] = [ZERO_U64; MAX_SEEN];
static SEEN_TARGET: [AtomicU64; MAX_SEEN] = [ZERO_U64; MAX_SEEN];
static SEEN_TICK: [AtomicU64; MAX_SEEN] = [ZERO_U64; MAX_SEEN];
/// Monotonic; the ring index is `n % MAX_SEEN`.
static SEEN_COUNT: AtomicUsize = AtomicUsize::new(0);
static SEEN_DRAINED: AtomicUsize = AtomicUsize::new(0);

/// The page a just-faulted write opened, to be protected again on the single-step trap that
/// follows the instruction. One slot: two threads faulting in the same microsecond would
/// leave one page open until the next burst sweep, which is the state every page was in
/// before this existed. `probe.rs`'s `WATCH_REARM` makes the same trade for the same reason.
static REPROTECT: AtomicUsize = AtomicUsize::new(0);

// ---------------------------------------------------------------------------------------
// Pure helpers - the parts worth testing without a client
// ---------------------------------------------------------------------------------------

/// Merge chunk spans and return the **fully contained** page runs, as `(start, len)`.
///
/// `bases` must be sorted. A chunk occupies `[base − 8, base − 8 + chunk_bytes + 8)`, which is
/// exactly the span `poolsentry`'s admission guard validates with `can_read`, so a page inside
/// a run is a page this module has already been told is committed.
///
/// Partial pages at the ends of a run are dropped rather than protected. Protecting a page
/// that is only half pool memory would put an unrelated allocation behind a fault for the
/// length of the window, and the whole design principle here is that the client must not be
/// able to tell the difference except in timing.
pub(crate) fn page_runs(bases: &[usize], b: &Bucket) -> Vec<(usize, usize)> {
    let span = b.chunk_bytes() + 8;
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for &base in bases {
        if base < 8 {
            continue;
        }
        let start = base - 8;
        let end = start + span;
        match merged.last_mut() {
            Some(last) if start <= last.1 + MERGE_GAP => {
                if end > last.1 {
                    last.1 = end;
                }
            }
            _ => merged.push((start, end)),
        }
    }
    let mut runs = Vec::new();
    for (start, end) in merged {
        let first = ((start + PAGE_BYTES - 1) / PAGE_BYTES) * PAGE_BYTES;
        let last = (end / PAGE_BYTES) * PAGE_BYTES;
        if last > first {
            runs.push((first, last - first));
        }
    }
    runs
}

/// The page of every header in `headers`, as one-page runs - the **pinned** set.
///
/// Run of 2026-09-07: fourteen windows protected only the pages that lay entirely inside pool
/// chunks - 73 of roughly 700 - because a `0x508`-byte chunk is one NT-heap block among many
/// and a whole page rarely belongs to the pool alone. Every store landed elsewhere. The same
/// run showed the writer **re-hits headers it has hit before** (three of twelve repaired slots
/// were damaged again by the end), so the pages of every header the sentry has caught are
/// protected on every window whether or not the fully-contained rule covers them. They are
/// pool pages by definition - a caught header sits in a chunk on the live chain - which keeps
/// the kernel-write caveat where it was.
pub(crate) fn header_pages(headers: &[usize]) -> Vec<(usize, usize)> {
    let mut pages: Vec<usize> = headers.iter().map(|h| (h / PAGE_BYTES) * PAGE_BYTES).collect();
    pages.sort_unstable();
    pages.dedup();
    pages.into_iter().map(|p| (p, PAGE_BYTES)).collect()
}

/// Sort and coalesce runs that touch or overlap, so two sources of pages become one table.
pub(crate) fn merge_runs(mut runs: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    runs.sort_unstable();
    let mut out: Vec<(usize, usize)> = Vec::new();
    for (start, len) in runs {
        match out.last_mut() {
            Some(last) if start <= last.0 + last.1 => {
                let end = (start + len).max(last.0 + last.1);
                last.1 = end - last.0;
            }
            _ => out.push((start, len)),
        }
    }
    out
}

/// The chunk containing `target`, by binary search over sorted bases.
fn find_chunk(bases: &[usize], target: usize, b: &Bucket) -> Option<usize> {
    let span = b.chunk_bytes() + 8;
    // The greatest base whose span could start at or before `target`.
    let mut lo = 0usize;
    let mut hi = bases.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if bases[mid] >= 8 && bases[mid] - 8 <= target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo == 0 {
        return None;
    }
    let base = bases[lo - 1];
    if base >= 8 && target < base - 8 + span {
        Some(base)
    } else {
        None
    }
}

/// Where inside a chunk `target` lands.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Where {
    /// A slot header: `(slot index, byte offset inside the 8-byte header)`. The family writes
    /// `(k, 4)`.
    Header(usize, usize),
    /// A slot body, which is what the class is full of and what the client writes all day.
    Body(usize),
    /// The chunk's own size qword or its list link - not a slot at all.
    Bookkeeping,
}

/// Classify an address inside a chunk. `base` is the chunk base; slot `k`'s header is at
/// `base + 8 + k * stride` and its body at `+8` from there (`poolsentry::BucketWalk::scan`).
pub(crate) fn classify(base: usize, target: usize, b: &Bucket) -> Where {
    if target < base + 8 {
        return Where::Bookkeeping;
    }
    let off = target - (base + 8);
    let slots = b.per_chunk * b.stride();
    if off >= slots {
        return Where::Bookkeeping;
    }
    let k = off / b.stride();
    let r = off % b.stride();
    if r < 8 {
        Where::Header(k, r)
    } else {
        Where::Body(k)
    }
}

// ---------------------------------------------------------------------------------------
// The handler
// ---------------------------------------------------------------------------------------

/// Is `target` inside a run we protected? Linear over the run table, which is a few hundred
/// entries and is only reached for an access violation.
unsafe fn in_runs(target: usize) -> bool {
    let n = RUN_COUNT.load(Ordering::SeqCst).min(MAX_RUNS);
    for i in 0..n {
        let start = RUN_START[i].load(Ordering::SeqCst);
        let len = RUN_LEN[i].load(Ordering::SeqCst);
        if target >= start && target < start + len {
            return true;
        }
    }
    false
}

/// Give the page containing `target` back its write permission so the faulting instruction can
/// re-execute. Failure is not fatal here - the instruction faults again, the handler runs
/// again, and if it never succeeds the client sees a real access violation, which is loud.
unsafe fn unprotect_page(target: usize) -> bool {
    let page = (target / PAGE_BYTES) * PAGE_BYTES;
    let mut old = 0u32;
    VirtualProtect(page as *mut c_void, PAGE_BYTES, PAGE_READWRITE, &mut old) != 0
}

/// Does the pool write watch own this exception?
///
/// `probe.rs` calls it before its `CLIENT FAULT` branch. If our handler ran first it returned
/// `EXCEPTION_CONTINUE_EXECUTION` and `probe` never sees the fault at all - but vectored
/// handler order is *registration* order, and a module that quietly depends on it would write
/// a 1.3 GB dump for every benign write into a watched page. This makes the order irrelevant.
///
/// # Safety
///
/// `info` must be the live `EXCEPTION_POINTERS` for the current exception.
pub(crate) unsafe fn suppresses(info: *mut c_void) -> bool {
    if !WATCHING.load(Ordering::SeqCst) || info.is_null() {
        return false;
    }
    let p = info.cast::<ExceptionPointers>();
    let rec = (*p).record;
    if rec.is_null() {
        return false;
    }
    if *(rec.add(REC_CODE).cast::<u32>()) != EXCEPTION_ACCESS_VIOLATION {
        return false;
    }
    if (*(rec.add(REC_PARAMS).cast::<u32>()) as usize) < 2 {
        return false;
    }
    let kind = *(rec.add(REC_INFO).cast::<usize>());
    let target = *(rec.add(REC_INFO + 8).cast::<usize>());
    kind == AV_WRITE && (in_runs(target) || probe_page_hit(target))
}

fn probe_page_hit(target: usize) -> bool {
    let page = PROBE_PAGE.load(Ordering::SeqCst);
    page != 0 && target >= page && target < page + PAGE_BYTES
}

/// The watch. Allocates nothing, takes no lock, writes no log - see the module docs.
unsafe extern "system" fn veh(info: *mut ExceptionPointers) -> i32 {
    if info.is_null() {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let rec = (*info).record;
    if rec.is_null() {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let code = *(rec.add(REC_CODE).cast::<u32>());

    // **The step after a write we let through: protect the page again, now.**
    //
    // Run 4 (2026-09-07 22:45-23:35): the store landed inside window #3 on a page that had
    // been PINNED for three minutes, and was not caught. The design until then unprotected a
    // page on its first write and left it open until the next 5 ms sweep - so a body write to
    // the same page a few microseconds earlier held the door for the header write. Every
    // window's pages take body traffic (258 first-writes across eleven windows), and the
    // writer's own routine plausibly writes a body before it writes the header. So the page is
    // now closed again one instruction after it was opened, and EVERY write faults.
    //
    // Checked before WATCHING: the window may have closed between the fault and the step, in
    // which case the page must stay writable and only the trap flag is cleared.
    if code == EXCEPTION_SINGLE_STEP {
        let page = REPROTECT.swap(0, Ordering::SeqCst);
        if page == 0 {
            return EXCEPTION_CONTINUE_SEARCH; // not ours - probe.rs re-plants its int3 on these
        }
        if WATCHING.load(Ordering::SeqCst) {
            let mut old = 0u32;
            VirtualProtect(page as *mut c_void, PAGE_BYTES, PAGE_READONLY, &mut old);
        }
        let ctx = (*info).context.cast::<u8>();
        *(ctx.add(CTX_EFLAGS).cast::<u32>()) &= !TRAP_FLAG;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    if !WATCHING.load(Ordering::SeqCst) || code != EXCEPTION_ACCESS_VIOLATION {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    if (*(rec.add(REC_PARAMS).cast::<u32>()) as usize) < 2 {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let kind = *(rec.add(REC_INFO).cast::<usize>());
    let target = *(rec.add(REC_INFO + 8).cast::<usize>());

    // The self-test's own page, which is not part of the pool and is classified against
    // nothing. Recording the raw parameters is the entire point: they are what proves the
    // two header offsets and the write code.
    if probe_page_hit(target) {
        PROBE_KIND.store(kind, Ordering::SeqCst);
        PROBE_SAW.store(target, Ordering::SeqCst);
        open_for_one_instruction(info, target);
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    if kind != AV_WRITE || !in_runs(target) {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    FAULTS.fetch_add(1, Ordering::SeqCst);
    let rip = *(rec.add(0x10).cast::<u64>());

    let n = SEEN_COUNT.fetch_add(1, Ordering::SeqCst) % MAX_SEEN;
    SEEN_RIP[n].store(rip, Ordering::SeqCst);
    SEEN_TARGET[n].store(target as u64, Ordering::SeqCst);
    SEEN_TICK[n].store(GetTickCount64(), Ordering::SeqCst);

    let b = &BUCKETS[DAMAGE_CLASS];
    let count = CHUNK_COUNT.load(Ordering::SeqCst).min(MAX_CHUNKS);
    let bases = std::slice::from_raw_parts(CHUNKS.as_ptr().cast::<usize>(), count);
    match find_chunk(bases, target, b) {
        Some(base) => match classify(base, target, b) {
            Where::Header(k, r) => {
                HEADER_WRITES.fetch_add(1, Ordering::SeqCst);
                let i = HIT_COUNT.fetch_add(1, Ordering::SeqCst);
                if i < MAX_HITS {
                    HITS[i].rip.store(rip, Ordering::SeqCst);
                    HITS[i].target.store(target as u64, Ordering::SeqCst);
                    HITS[i]
                        .slot_header
                        .store((base + 8 + k * b.stride()) as u64, Ordering::SeqCst);
                    HITS[i].tid.store(GetCurrentThreadId(), Ordering::SeqCst);
                    HITS[i].inside.store(r as u32, Ordering::SeqCst);
                    HITS[i].index.store(k as u32, Ordering::SeqCst);
                    HITS[i].tick.store(GetTickCount64(), Ordering::SeqCst);
                }
                // The dump is written by the sentry thread, not here. A 1.3 GB
                // `MiniDumpWriteDump` from inside a vectored handler on a thread that may
                // hold the pool spinlock is how this module would hang the client at the
                // exact moment it finally had the answer.
                DUMP_WANTED.store(true, Ordering::SeqCst);
            }
            Where::Body(_) => {
                BODY_WRITES.fetch_add(1, Ordering::SeqCst);
            }
            Where::Bookkeeping => {
                EDGE_WRITES.fetch_add(1, Ordering::SeqCst);
            }
        },
        None => {
            EDGE_WRITES.fetch_add(1, Ordering::SeqCst);
        }
    }

    open_for_one_instruction(info, target);
    EXCEPTION_CONTINUE_EXECUTION
}

/// Make the page writable so the faulting store can complete, and arrange for it to be
/// read-only again immediately after: the trap flag on the resumed context delivers a
/// single-step exception after exactly one instruction, and the branch at the top of [`veh`]
/// re-protects the page recorded here.
unsafe fn open_for_one_instruction(info: *mut ExceptionPointers, target: usize) {
    let page = (target / PAGE_BYTES) * PAGE_BYTES;
    if unprotect_page(target) {
        REPROTECT.store(page, Ordering::SeqCst);
        let ctx = (*info).context.cast::<u8>();
        *(ctx.add(CTX_EFLAGS).cast::<u32>()) |= TRAP_FLAG;
    }
}

// ---------------------------------------------------------------------------------------
// Arming
// ---------------------------------------------------------------------------------------

/// Split a page run at region boundaries, keeping only committed `PAGE_READWRITE` pages.
///
/// `VirtualProtect` fails for the **whole** range if it crosses two reserved allocations, so a
/// run that straddles one would silently protect nothing. Splitting here is also the filter
/// that keeps this module off any page that is not ordinary writable heap.
unsafe fn split_committed(runs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let size = std::mem::size_of::<MemoryBasicInformation>();
    for &(start, len) in runs {
        let mut at = start;
        let end = start + len;
        while at < end {
            let mut mbi = MemoryBasicInformation {
                base: std::ptr::null_mut(),
                allocation_base: std::ptr::null_mut(),
                ..Default::default()
            };
            if VirtualQuery(at as *const c_void, &mut mbi, size) == 0 {
                break;
            }
            let region_end = (mbi.base as usize + mbi.region_size).min(end);
            if region_end <= at {
                break;
            }
            if mbi.state == MEM_COMMIT && mbi.protect == PAGE_READWRITE {
                out.push((at, region_end - at));
                if out.len() >= MAX_RUNS {
                    return out;
                }
            }
            at = region_end;
        }
    }
    out
}

unsafe fn protect_runs(new: u32) -> usize {
    let n = RUN_COUNT.load(Ordering::SeqCst).min(MAX_RUNS);
    let mut ok = 0usize;
    for i in 0..n {
        let start = RUN_START[i].load(Ordering::SeqCst);
        let len = RUN_LEN[i].load(Ordering::SeqCst);
        let mut old = 0u32;
        if len > 0 && VirtualProtect(start as *mut c_void, len, new, &mut old) != 0 {
            ok += 1;
        }
    }
    ok
}

/// Prove the three `EXCEPTION_RECORD` claims against a page of our own. See the module docs.
unsafe fn self_test() -> Result<(), String> {
    let page = VirtualAlloc(
        std::ptr::null_mut(),
        PAGE_BYTES,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    ) as usize;
    if page == 0 {
        return Err("VirtualAlloc for the control page failed".into());
    }
    let at = page + 0x40;
    std::ptr::write_volatile(at as *mut u32, 0xAAAA_AAAA);
    PROBE_SAW.store(0, Ordering::SeqCst);
    PROBE_KIND.store(usize::MAX, Ordering::SeqCst);
    PROBE_PAGE.store(page, Ordering::SeqCst);

    let mut old = 0u32;
    if VirtualProtect(page as *mut c_void, PAGE_BYTES, PAGE_READONLY, &mut old) == 0 {
        PROBE_PAGE.store(0, Ordering::SeqCst);
        VirtualFree(page as *mut c_void, 0, MEM_RELEASE);
        return Err("VirtualProtect on the control page failed".into());
    }
    // The handler is gated on WATCHING, so the control has to raise it. Nothing is in the run
    // table yet, so no client page is protected while this runs.
    let was = WATCHING.swap(true, Ordering::SeqCst);
    std::ptr::write_volatile(at as *mut u32, 0x5555_5555);
    WATCHING.store(was, Ordering::SeqCst);

    let saw = PROBE_SAW.load(Ordering::SeqCst);
    let kind = PROBE_KIND.load(Ordering::SeqCst);
    let value = std::ptr::read_volatile(at as *const u32);
    PROBE_PAGE.store(0, Ordering::SeqCst);
    VirtualFree(page as *mut c_void, 0, MEM_RELEASE);

    if saw == 0 {
        return Err(
            "the handler never saw the control write - either the vectored handler is not \
             installed or EXCEPTION_RECORD is not laid out as assumed"
                .into(),
        );
    }
    if saw != at {
        return Err(format!(
            "the handler reported {saw:#x} for a write to {at:#x} - ExceptionInformation[1] is \
             not at record+{REC_INFO:#x}"
        ));
    }
    if kind != AV_WRITE {
        return Err(format!(
            "the handler read ExceptionInformation[0] = {kind}, expected {AV_WRITE} for a write"
        ));
    }
    if value != 0x5555_5555 {
        return Err(format!(
            "the control write did not land ({value:#x}) - EXCEPTION_CONTINUE_EXECUTION did not \
             re-execute the instruction"
        ));
    }
    Ok(())
}

/// Result of one arming, for the sentry to log.
pub(crate) struct Armed {
    pub runs: usize,
    pub pages: usize,
    pub protected: usize,
    /// Pages protected because a header on them was caught earlier this run.
    pub pinned: usize,
    pub note: Option<String>,
}

/// Open a window: protect bucket `DAMAGE_CLASS`'s pages now, re-protect every `burst`, and
/// restore after `window` (or as soon as [`stop_and_wait`] asks).
///
/// Returns `None` if a window is already open. Spawns a thread and returns immediately; the
/// caller's own walk must keep running, which is the whole point - the sentry has to be the
/// thing that notices the damage the fault let through.
pub(crate) fn arm_for(
    bases: Vec<usize>,
    caught_headers: Vec<usize>,
    window: Duration,
    burst: Duration,
) -> Option<Armed> {
    if ACTIVE.swap(true, Ordering::SeqCst) {
        return None;
    }
    STOP.store(false, Ordering::SeqCst);

    let b = &BUCKETS[DAMAGE_CLASS];
    let pinned_runs = header_pages(&caught_headers);
    let pinned = pinned_runs.len();
    let mut runs = page_runs(&bases, b);
    runs.extend(pinned_runs);
    let runs = merge_runs(runs);
    let pages: usize = runs.iter().map(|r| r.1 / PAGE_BYTES).sum();

    let mut note = None;
    unsafe {
        if !VEH_REGISTERED.swap(true, Ordering::SeqCst) {
            // Registered here rather than at hook install, and late on purpose: a first-chance
            // handler added last is called FIRST, so this one sees the benign write faults
            // before `probe.rs` can decide they are a client crash. `suppresses` covers the
            // case where that ordering ever changes.
            AddVectoredExceptionHandler(1, veh as *const c_void);
        }
        if !SELF_TEST_DONE.swap(true, Ordering::SeqCst) {
            match self_test() {
                Ok(()) => {
                    SELF_TEST_OK.store(true, Ordering::SeqCst);
                    note = Some(
                        "control PASS: a write to a page of our own faulted, was reported at \
                         the exact address, and re-executed"
                            .to_string(),
                    );
                }
                Err(e) => {
                    note = Some(format!("control FAIL: {e}. NOT arming - a watch that cannot \
                         see its own control would report silence as a result"));
                }
            }
        }
        if !SELF_TEST_OK.load(Ordering::SeqCst) {
            ACTIVE.store(false, Ordering::SeqCst);
            return Some(Armed {
                runs: 0,
                pages: 0,
                protected: 0,
                pinned: 0,
                note,
            });
        }

        let committed = split_committed(&runs);
        let n = committed.len().min(MAX_RUNS);
        for (i, &(start, len)) in committed.iter().take(n).enumerate() {
            RUN_START[i].store(start, Ordering::SeqCst);
            RUN_LEN[i].store(len, Ordering::SeqCst);
        }
        RUN_COUNT.store(n, Ordering::SeqCst);

        let c = bases.len().min(MAX_CHUNKS);
        for (i, &base) in bases.iter().take(c).enumerate() {
            CHUNKS[i].store(base, Ordering::SeqCst);
        }
        CHUNK_COUNT.store(c, Ordering::SeqCst);
    }

    let armed_runs = RUN_COUNT.load(Ordering::SeqCst);
    WATCHING.store(true, Ordering::SeqCst);
    let protected = unsafe { protect_runs(PAGE_READONLY) };
    WINDOWS.fetch_add(1, Ordering::SeqCst);

    let faults_at_open = FAULTS.load(Ordering::SeqCst);
    std::thread::spawn(move || {
        let started = Instant::now();
        while started.elapsed() < window && !STOP.load(Ordering::SeqCst) {
            std::thread::sleep(burst);
            if STOP.load(Ordering::SeqCst) {
                break;
            }
            if FAULTS.load(Ordering::SeqCst) - faults_at_open >= MAX_FAULTS_PER_WINDOW {
                BACKED_OFF.fetch_add(1, Ordering::SeqCst);
                break;
            }
            unsafe { protect_runs(PAGE_READONLY) };
        }
        // Order matters on the way down: stop claiming faults BEFORE the pages become
        // writable, so a fault that arrives in between is still handled by us rather than
        // reaching `probe.rs` as a client crash.
        WATCHING.store(false, Ordering::SeqCst);
        unsafe { protect_runs(PAGE_READWRITE) };
        RUN_COUNT.store(0, Ordering::SeqCst);
        ACTIVE.store(false, Ordering::SeqCst);
    });

    Some(Armed {
        runs: armed_runs,
        pages,
        protected,
        pinned,
        note,
    })
}

/// Close the window and wait for the pages to be writable again.
///
/// `poolsentry`'s repair writes to a slot header, and `session::can_write` refuses a
/// `PAGE_READONLY` page - so a repair inside an open window would be refused rather than
/// applied, which is exactly the failure the repair exists to prevent. Returns whether the
/// window actually closed in time.
pub(crate) fn stop_and_wait(timeout: Duration) -> bool {
    if !ACTIVE.load(Ordering::SeqCst) {
        return true;
    }
    STOP.store(true, Ordering::SeqCst);
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if !ACTIVE.load(Ordering::SeqCst) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    !ACTIVE.load(Ordering::SeqCst)
}

pub(crate) fn wants_dump() -> bool {
    DUMP_WANTED.swap(false, Ordering::SeqCst)
}

pub(crate) fn armed_windows() -> u32 {
    WINDOWS.load(Ordering::SeqCst)
}

/// How many windows backed off under [`MAX_FAULTS_PER_WINDOW`]. Non-zero means the watch was
/// blind for part of those windows, on purpose, and it must be said rather than left to look
/// like a quiet client.
pub(crate) fn backed_off() -> u32 {
    BACKED_OFF.load(Ordering::SeqCst)
}

/// `(write faults, header writes, body writes, edge writes)` for the heartbeat.
pub(crate) fn counters() -> (u64, u64, u64, u64) {
    (
        FAULTS.load(Ordering::SeqCst),
        HEADER_WRITES.load(Ordering::SeqCst),
        BODY_WRITES.load(Ordering::SeqCst),
        EDGE_WRITES.load(Ordering::SeqCst),
    )
}

/// Everything recorded since the last drain, formatted. Called from the sentry thread, which
/// may allocate and may log.
pub(crate) fn drain() -> Vec<String> {
    let mut out = Vec::new();

    // The ring holds the last MAX_SEEN faults; anything older than that since the previous
    // drain has been overwritten and is counted, not listed.
    let seen = SEEN_COUNT.load(Ordering::SeqCst);
    let mut from = SEEN_DRAINED.load(Ordering::SeqCst);
    if seen.saturating_sub(from) > MAX_SEEN {
        out.push(format!(
            "POOL WRITE WATCH: {} write fault(s) since the last drain, the last {MAX_SEEN} follow",
            seen - from
        ));
        from = seen - MAX_SEEN;
    }
    while from < seen {
        let i = from % MAX_SEEN;
        let rip = SEEN_RIP[i].load(Ordering::SeqCst) as usize;
        let target = SEEN_TARGET[i].load(Ordering::SeqCst) as usize;
        let tick = SEEN_TICK[i].load(Ordering::SeqCst);
        out.push(format!(
            "POOL WRITE WATCH saw a write into a watched page: {target:#x} from {rip:#x}{} at \
             tick {tick}. Liveness: the protection is real and faults reach us; a run of these \
             on one page just before a header write on it is the writer's own footprint",
            unsafe { crate::netwatch::module_of(rip) }
        ));
        from += 1;
    }
    SEEN_DRAINED.store(seen, Ordering::SeqCst);

    let hits = HIT_COUNT.load(Ordering::SeqCst).min(MAX_HITS);
    let mut from = HITS_DRAINED.load(Ordering::SeqCst);
    while from < hits {
        let h = &HITS[from];
        let rip = h.rip.load(Ordering::SeqCst) as usize;
        let target = h.target.load(Ordering::SeqCst) as usize;
        let header = h.slot_header.load(Ordering::SeqCst) as usize;
        let inside = h.inside.load(Ordering::SeqCst);
        out.push(format!(
            "***** POOL WRITE WATCH - THE WRITER: a store to slot header {header:#x} at \
             +{inside} (address {target:#x}, slot index {}) from RIP {rip:#x}{}, tid {}, tick \
             {}.{} This is the instruction that damages the header. Everything before today \
             named a moment; this names a *who* *****",
            h.index.load(Ordering::SeqCst),
            unsafe { crate::netwatch::module_of(rip) },
            h.tid.load(Ordering::SeqCst),
            h.tick.load(Ordering::SeqCst),
            unsafe { bytes_at(rip) },
        ));
        from += 1;
    }
    HITS_DRAINED.store(hits, Ordering::SeqCst);
    out
}

/// The faulting instruction's own bytes. An access violation is a **fault**, so RIP is the
/// instruction that has not yet completed - these bytes are it, not the one after.
unsafe fn bytes_at(rip: usize) -> String {
    if rip == 0 || !crate::session::can_read(rip, 16) {
        return String::new();
    }
    let mut s = String::from(" bytes:");
    for i in 0..16 {
        s.push_str(&format!(" {:02x}", *((rip + i) as *const u8)));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The module's state is process-global by construction - one vectored handler, one run
    /// table - so the two tests that arm it must not overlap.
    static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn register() {
        unsafe {
            if !VEH_REGISTERED.swap(true, Ordering::SeqCst) {
                AddVectoredExceptionHandler(1, veh as *const c_void);
            }
        }
    }

    /// **The instrument's own control, run by `cargo test`.**
    ///
    /// Three offsets in this module come out of `winnt.h` and nothing else:
    /// `NumberParameters` at `0x18`, `ExceptionInformation` at `0x20`, and `[0] == 1` for a
    /// write. `minidump.rs` is this repo's standing lesson that a struct layout read out of a
    /// header can be wrong in a way that produces no error at all - there, every
    /// `MINIDUMP_*` type is inside `<pshpack4.h>` and the "obvious" 24/8 layout returned
    /// `ERROR_NOACCESS` on every call for a day.
    ///
    /// If any of the three were wrong here the failure would be *silence*: the handler would
    /// hand every fault back and a run would report "no write was seen", which reads exactly
    /// like "the writer did not fire".
    #[test]
    fn the_handler_reads_a_real_exception_record_correctly() {
        let _g = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        register();
        unsafe { self_test() }.expect("the control write must fault, be reported at its own              address, and re-execute");
    }

    /// End to end on a fabricated pool, with no client: lay out eleven chunks exactly as a
    /// 16 KB segment holds them, arm the window, perform **the family's own write** - four
    /// bytes of `1` at `header + 4` - and require that it is caught, classified as slot 3's
    /// header at offset 4, named with a RIP, and *still lands*.
    ///
    /// The last part is not a detail. The client has to keep running: an instrument that
    /// caught the write by preventing it would change the very behaviour under study.
    #[test]
    fn it_catches_the_family_write_names_it_and_lets_it_through() {
        let _g = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        register();
        let b = &BUCKETS[DAMAGE_CLASS];
        let stride = b.chunk_bytes() + 8;
        unsafe {
            let region = VirtualAlloc(
                std::ptr::null_mut(),
                0x10000,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            ) as usize;
            assert_ne!(region, 0, "VirtualAlloc");

            // Eleven chunks, tiled. Each gets the size qword the walk's identity check reads
            // and 32 slot headers carrying the plain slot size, as the carve writes them.
            let bases: Vec<usize> = (0..11).map(|i| region + 8 + i * stride).collect();
            for &base in &bases {
                std::ptr::write_volatile((base - 8) as *mut u64, b.chunk_bytes() as u64);
                for k in 0..b.per_chunk {
                    std::ptr::write_volatile((base + 8 + k * b.stride()) as *mut u64, b.slot as u64);
                }
            }

            let headers_before = HEADER_WRITES.load(Ordering::SeqCst);
            let bodies_before = BODY_WRITES.load(Ordering::SeqCst);
            let hits_before = HIT_COUNT.load(Ordering::SeqCst);

            let armed = arm_for(
                bases.clone(),
                Vec::new(),
                Duration::from_millis(600),
                Duration::from_millis(5),
            )
            .expect("no window should already be open");
            assert!(
                armed.protected > 0,
                "nothing was protected, so the rest of this test would pass vacuously: {:?}",
                armed.note
            );

            // A body write first, as the control for the classifier: the class is full of
            // these and they must NOT be reported as the writer.
            let body = bases[2] + 8 + 7 * b.stride() + 8;
            let faults_before = FAULTS.load(Ordering::SeqCst);
            std::ptr::write_volatile(body as *mut u64, 0xdead_beef);
            assert_eq!(std::ptr::read_volatile(body as *const u64), 0xdead_beef);
            assert_eq!(FAULTS.load(Ordering::SeqCst), faults_before + 1);
            // **A second write to the SAME page faults again.** Run 4's store landed on a
            // pinned page inside its window and was missed, because the first write to a page
            // used to leave it open until the next sweep. The single-step re-protect closes it
            // one instruction later, so every write is seen.
            std::ptr::write_volatile((body + 8) as *mut u64, 0xfeed_face);
            assert_eq!(
                FAULTS.load(Ordering::SeqCst),
                faults_before + 2,
                "the page must be read-only again one instruction after the first write"
            );
            assert_eq!(std::ptr::read_volatile((body + 8) as *const u64), 0xfeed_face);

            // Now the family's write, on slot 3 of chunk 5.
            let header = bases[5] + 8 + 3 * b.stride();
            std::ptr::write_volatile((header + 4) as *mut u32, 1);

            assert_eq!(
                std::ptr::read_volatile(header as *const u64),
                crate::poolsentry::KNOWN_DAMAGE,
                "the write must still land - the watch observes, it does not prevent"
            );
            assert_eq!(
                HEADER_WRITES.load(Ordering::SeqCst),
                headers_before + 1,
                "exactly one header write"
            );
            assert!(
                BODY_WRITES.load(Ordering::SeqCst) > bodies_before,
                "the body write must be seen and classified as a body"
            );

            let hit = &HITS[hits_before];
            assert_eq!(hit.slot_header.load(Ordering::SeqCst) as usize, header);
            assert_eq!(hit.target.load(Ordering::SeqCst) as usize, header + 4);
            assert_eq!(hit.inside.load(Ordering::SeqCst), 4, "the family's offset");
            assert_eq!(hit.index.load(Ordering::SeqCst), 3);
            assert_ne!(hit.rip.load(Ordering::SeqCst), 0, "the writing instruction");
            assert!(wants_dump(), "a header write asks the sentry for a dump");

            assert!(stop_and_wait(Duration::from_secs(2)), "the window must close");
            // And the pages must be ordinary writable memory again afterwards, with no
            // fault: a watch that leaves the client's allocator read-only is worse than none.
            let faults = FAULTS.load(Ordering::SeqCst);
            std::ptr::write_volatile(header as *mut u64, b.slot as u64);
            assert_eq!(FAULTS.load(Ordering::SeqCst), faults, "no fault after the window");

            let lines = drain();
            assert!(
                lines.iter().any(|l| l.contains("THE WRITER") && l.contains(&format!("{header:#x}"))),
                "the drain must name the slot it caught: {lines:?}"
            );

            VirtualFree(region as *mut c_void, 0, MEM_RELEASE);
        }
    }

    fn b1() -> Bucket {
        Bucket {
            slot: 0x20,
            per_chunk: 32,
        }
    }

    #[test]
    fn chunk_geometry_matches_the_walk() {
        let b = b1();
        assert_eq!(b.stride(), 0x28);
        assert_eq!(b.chunk_bytes(), 0x508);
    }

    #[test]
    fn a_lone_chunk_yields_no_page_because_no_page_is_fully_inside_it() {
        // 0x508 bytes cannot contain a 0x1000 page whatever its alignment. The run table
        // being empty here is the conservative behaviour, not a bug.
        let b = b1();
        assert!(page_runs(&[0x10_0008], &b).is_empty());
    }

    #[test]
    fn contiguous_chunks_coalesce_into_whole_pages() {
        let b = b1();
        // Eleven chunks laid end to end, as a 16 KB segment holds them.
        let bases: Vec<usize> = (0..11).map(|i| 0x10_0008 + i * 0x508).collect();
        let runs = page_runs(&bases, &b);
        assert_eq!(runs.len(), 1);
        let (start, len) = runs[0];
        assert_eq!(start % PAGE_BYTES, 0);
        assert_eq!(len % PAGE_BYTES, 0);
        // The span is 11 * 0x508 + 8 = 0x3728 + 8 bytes from 0x100000, so three whole pages.
        assert_eq!(start, 0x10_0000);
        assert_eq!(len, 3 * PAGE_BYTES);
    }

    /// **A caught header's page is protected even when no whole page lies inside its chunk.**
    /// The run of 2026-09-07: 73 pages of ~700 covered, every store elsewhere, and the writer
    /// re-hitting slots it had hit before. A lone chunk yields no run on its own (the test
    /// above); with one of its headers in the caught list, its page is a run.
    #[test]
    fn a_caught_headers_page_is_pinned_even_when_the_chunk_alone_yields_nothing() {
        let b = b1();
        let base = 0x10_0008;
        assert!(page_runs(&[base], &b).is_empty(), "the control: no whole page in one chunk");
        let header = base + 8 + 5 * b.stride();
        let pinned = header_pages(&[header, header + b.stride()]);
        assert_eq!(pinned, vec![(0x10_0000, PAGE_BYTES)], "two headers on one page: one run");
        let all = merge_runs([page_runs(&[base], &b), pinned].concat());
        assert_eq!(all, vec![(0x10_0000, PAGE_BYTES)]);
        // And pinned pages coalesce with the ordinary runs around them rather than doubling.
        let bases: Vec<usize> = (0..11).map(|i| 0x20_0008 + i * 0x508).collect();
        let runs = page_runs(&bases, &b);
        let with_pin = merge_runs([runs.clone(), header_pages(&[0x20_0008 + 8])].concat());
        assert_eq!(with_pin, runs, "a header inside an already-covered run adds nothing");
        let with_edge = merge_runs([runs.clone(), header_pages(&[0x20_3fff])].concat());
        assert_eq!(with_edge.len(), 1, "an adjacent page extends the run");
        assert_eq!(with_edge[0].1, runs[0].1 + PAGE_BYTES);
    }

    #[test]
    fn distant_chunks_do_not_merge() {
        let b = b1();
        let bases = vec![0x10_0008, 0x10_0008 + 0x508, 0x20_0008, 0x20_0008 + 0x508];
        let merged = page_runs(&bases, &b);
        // Neither pair spans a whole page on its own, so both drop out - and crucially the
        // two do NOT become one run covering the gap between them.
        assert!(merged.iter().all(|&(s, l)| s + l <= 0x10_2000 || s >= 0x20_0000));
    }

    #[test]
    fn find_chunk_locates_the_owner_and_rejects_the_gaps() {
        let b = b1();
        let bases = vec![0x10_0008, 0x10_0008 + 0x508, 0x20_0008];
        assert_eq!(find_chunk(&bases, 0x10_0008, &b), Some(0x10_0008));
        assert_eq!(find_chunk(&bases, 0x10_0000, &b), Some(0x10_0008)); // the size qword
        assert_eq!(find_chunk(&bases, 0x10_0510, &b), Some(0x10_0510));
        assert_eq!(find_chunk(&bases, 0x0f_ffff, &b), None);
        assert_eq!(find_chunk(&bases, 0x18_0000, &b), None);
        // A chunk's span starts EIGHT bytes before its base, at the size qword the walk's
        // identity check reads - so 0x200000..0x200008 belongs to the 0x200008 chunk and the
        // first address outside it is 0x1fffff. Getting this edge wrong the other way would
        // silently drop every write to a chunk's own bookkeeping into "not ours".
        assert_eq!(find_chunk(&bases, 0x20_0000, &b), Some(0x20_0008));
        assert_eq!(find_chunk(&bases, 0x20_0007, &b), Some(0x20_0008));
        assert_eq!(find_chunk(&bases, 0x1f_ffff, &b), None);
        assert_eq!(classify(0x20_0008, 0x20_0000, &b), Where::Bookkeeping);
        // The last byte of the last slot's body is still inside; one past it is not.
        let last = 0x20_0008 + 8 + (b.per_chunk - 1) * b.stride() + 8 + b.slot - 1;
        assert_eq!(find_chunk(&bases, last, &b), Some(0x20_0008));
        assert_eq!(find_chunk(&bases, 0x20_0000 + b.chunk_bytes() + 8, &b), None);
    }

    #[test]
    fn the_family_signature_classifies_as_a_header_write_at_four() {
        let b = b1();
        let base = 0x3860_8a80;
        // Slot k's header, and the high dword the writer damages.
        for k in [0usize, 1, 17, 31] {
            let header = base + 8 + k * b.stride();
            assert_eq!(classify(base, header, &b), Where::Header(k, 0));
            assert_eq!(classify(base, header + 4, &b), Where::Header(k, 4));
            assert_eq!(classify(base, header + 8, &b), Where::Body(k));
            assert_eq!(classify(base, header + 0x20, &b), Where::Body(k));
        }
    }

    #[test]
    fn a_real_finding_address_lands_where_the_sentry_put_it() {
        // 0x2f3eccb0 was FINDING #1 on 2026-09-06. Any slot header is `base + 8 + k*0x28`,
        // so the address minus 8 must be a multiple of the stride from the base - this pins
        // the two formulas together rather than trusting them separately.
        let b = b1();
        let header = 0x2f3e_ccb0usize;
        let base = header - 8 - 3 * b.stride(); // pick slot 3
        assert_eq!(classify(base, header + 4, &b), Where::Header(3, 4));
    }

    #[test]
    fn the_chunk_bookkeeping_is_not_a_slot() {
        let b = b1();
        let base = 0x10_0008;
        assert_eq!(classify(base, base, &b), Where::Bookkeeping); // the list link
        assert_eq!(classify(base, base + 4, &b), Where::Bookkeeping);
        let past = base + 8 + b.per_chunk * b.stride();
        assert_eq!(classify(base, past, &b), Where::Bookkeeping);
    }

    #[test]
    fn every_slot_header_in_a_chunk_is_reachable_and_distinct() {
        let b = b1();
        let base = 0x10_0008;
        let mut seen = std::collections::HashSet::new();
        for k in 0..b.per_chunk {
            let header = base + 8 + k * b.stride();
            assert_eq!(classify(base, header + 4, &b), Where::Header(k, 4));
            assert!(seen.insert(header));
        }
        assert_eq!(seen.len(), b.per_chunk);
    }
}
