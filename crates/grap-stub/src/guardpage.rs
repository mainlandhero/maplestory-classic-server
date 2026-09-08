//! A quarantine allocator for one pool size class, so a write through a **stale pointer into
//! freed-and-reused memory** faults *at the writing instruction* instead of days later in
//! someone else's free.
//!
//! # The disease this is built for
//!
//! Five write-watch runs (`research/the-180-second-clock-2026-09-07.md`) point at one writer
//! with one habit: it holds a pointer into pool memory that has since been freed and handed
//! back out, and writes a small increment through it on a timer. The evidence is that the same
//! damage lands on three different occupants of the same addresses:
//!
//! * a **freed `0x20` slot**, where the increment hits the size header (`body − 4`) and the
//!   next free of that slot dies `0xC0000374` — the pooled-free family;
//! * a **live `0x40` map node**, where it hits a pointer's low dword and a later read of the
//!   `+2` pointer dies `0xC0000005` — runs 2 and 5.
//!
//! The write watch (`crate::writewatch`) catches the first: it protects the `0x20` chunk pages
//! read-only around a predicted firing. It cannot catch the second — wrong class, a live node,
//! and run 5 died before any window opened. What catches an arbitrary stale write, on any
//! clock, is memory that is **never handed out twice**: quarantine.
//!
//! # What it does
//!
//! While armed, an allocation of the watched class is served from its **own page** in a large
//! private reservation, and its free **decommits that page and never reuses the address**. A
//! genuine allocation that outlives its free is invisible — the client stops touching a slot it
//! freed. A *stale* pointer is not: the next write through it hits a decommitted page and
//! faults, and the handler names the instruction, the address, the return address that
//! allocated the slot, and the one that freed it, then recommits the page and continues so the
//! client survives to the next one.
//!
//! # Why this is not a rewrite of the client's allocator
//!
//! `heap-corruption-2026-09-06.md` §3.2 proposed replacing the allocator. This does far less,
//! and the two decisions that make it safe on the client's hottest subsystem:
//!
//! * **Alloc is the only inline hook**, and its stolen bytes are three position-independent
//!   `mov [rsp+disp],reg` (15 bytes, `research/heap-wild-write.md` §1), reused through
//!   [`crate::identity::install_detour`], which refuses to patch unless the prologue matches.
//! * **Free is a pointer swap, not a code patch.** A quarantined slot's header is stamped
//!   `> 0x80`, so the client's own free (`FUN_14019bb50` / `FUN_14019b4e0`) takes its
//!   `HeapFree` arm rather than the pool arm — keeping our slots entirely off the pool's
//!   free-list and live-counter — and we intercept by swapping the cached `HeapFree` pointer
//!   at [`HEAPFREE_SLOT_RVA`], exactly as `crate::freeguard` swaps PCOM's.
//!
//! Our slots never touch the pool's chunk chain, free list or counters, so `poolsentry` and
//! `tools/poolchain.py` are unaffected and keep walking the real pool.
//!
//! # Off by default, one bucket, and bounded
//!
//! Armed only by `guardpage=<hex slot size>` in the session marker (`0x40` is the default the
//! flag writes). Default bucket 2 (`0x40`) because runs 2 and 5 have no other coverage and it
//! is the lowest-traffic class (18 758 allocations in a 50-minute run against bucket 1's
//! 56 446). The reserve is address space only (2 GB reserved = 512 K one-page slots); committed
//! memory is one page per *live* slot; a decommitted page costs nothing but its address. If the
//! cursor runs out, allocation falls back to the client's own and says so.
//!
//! # The control that has to pass first
//!
//! `CLAUDE.md`: an instrument that has never fired is exactly the kind this file distrusts.
//! [`self_test`] reserves one page of its own, decommits it, writes to it, and requires the
//! handler to have caught that write at that address and recommitted it so the write lands. It
//! runs before the hooks go in; a failure stands the module down rather than arming a watch
//! that cannot see its own subject.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

use crate::hook::log;

// ---------------------------------------------------------------------------------------
// The client, from research
// ---------------------------------------------------------------------------------------

/// `FUN_14019b780(ctx, size) -> body` — the pool allocator. `research/equip-crash.md`,
/// `research/heap-wild-write.md` §1.
const ALLOC_RVA: usize = 0x14019b780 - 0x140000000;
/// Its first fifteen bytes: `mov [rsp+8],rbx; mov [rsp+0x18],rbp; mov [rsp+0x20],rsi`. All
/// position-independent, a clean boundary at +15. `[L]`, the listing.
const ALLOC_PROLOGUE: [u8; 15] = [
    0x48, 0x89, 0x5c, 0x24, 0x08, 0x48, 0x89, 0x6c, 0x24, 0x18, 0x48, 0x89, 0x74, 0x24, 0x20,
];
/// The pool context. Both frees ladder on the header and, for a slot whose header is not one
/// of `0x10/0x20/0x40/0x80`, take the `HeapFree` arm. `heapfix-did-not-hold.md` §1.
const POOL_CTX_RVA: usize = 0x143AD68A0 - 0x140000000;
/// The cached `HeapFree` pointer both frees load with `mov rbx,[rip+..]` /
/// `call qword [rip+..]`. `research/heapfix-did-not-hold.md` §1: `-> 0x143ad5530`. Swapping it
/// is `crate::freeguard`'s technique; in the run-5 dump it held `kernel32!HeapFree`.
const HEAPFREE_SLOT_RVA: usize = 0x143AD5530 - 0x140000000;
/// The pool's own code: the hardcoded-context free `FUN_14019b4e0`, the allocator
/// `FUN_14019b780`, the free `FUN_14019bb50`, and the helpers between them. A return address
/// in here is the pool calling itself, never the caller worth naming - see [`caller_ra`].
const POOL_CODE_LO_RVA: usize = 0x14019b4e0 - 0x140000000;
const POOL_CODE_HI_RVA: usize = 0x14019bc60 - 0x140000000;

// ---------------------------------------------------------------------------------------
// Windows
// ---------------------------------------------------------------------------------------

const PAGE_BYTES: usize = 0x1000;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_NOACCESS: u32 = 0x01;
const MEM_RESERVE: u32 = 0x2000;
const MEM_COMMIT: u32 = 0x1000;
const MEM_DECOMMIT: u32 = 0x4000;

const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
const EXCEPTION_ACCESS_VIOLATION: u32 = 0xC000_0005;

const REC_CODE: usize = 0x00;
const REC_PARAMS: usize = 0x18;
const REC_INFO: usize = 0x20;
const CTX_RIP: usize = 0xF8;

#[repr(C)]
struct ExceptionPointers {
    record: *mut u8,
    context: *mut c_void,
}

extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: *const c_void) -> *mut c_void;
    fn VirtualAlloc(addr: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
    fn VirtualFree(addr: *mut c_void, size: usize, kind: u32) -> i32;
    fn GetTickCount64() -> u64;
    fn GetCurrentThreadId() -> u32;
}

// ---------------------------------------------------------------------------------------
// Sizing and the pure core
// ---------------------------------------------------------------------------------------

/// 2 GB of address space: 512 K one-page slots. Reserved, not committed; a decommitted page
/// costs only its slot in this range.
const RESERVE_BYTES: usize = 2 * 1024 * 1024 * 1024;
const MAX_SLOTS: usize = RESERVE_BYTES / PAGE_BYTES;

/// Where in a guard page the slot sits. The header the client reads at `body − 8` lands at
/// `page + 8`; the body at `page + 0x10` is 16-aligned, as the pool's are.
const SLOT_BODY_OFF: usize = 0x10;
const SLOT_HEADER_OFF: usize = SLOT_BODY_OFF - 8;

/// The header value stamped on a quarantined slot: **greater than `0x80`**, so the client's
/// free ladders past `0x10/0x20/0x40/0x80` and takes the `HeapFree` arm we intercept, instead
/// of pushing our slot onto the pool's free list. `research/heapfix-did-not-hold.md` §1.
const QUARANTINE_HEADER: u64 = 0x100;

/// The pool's size-class ladder, from the allocator's own thresholds. Rounds a requested size
/// up to `0x10/0x20/0x40/0x80`, or `None` for anything larger (the big-block path, never
/// quarantined).
pub(crate) fn size_class(size: usize) -> Option<usize> {
    match size {
        0..=0x10 => Some(0x10),
        0x11..=0x20 => Some(0x20),
        0x21..=0x40 => Some(0x40),
        0x41..=0x80 => Some(0x80),
        _ => None,
    }
}

/// The page containing `addr`.
pub(crate) fn page_of(addr: usize) -> usize {
    addr & !(PAGE_BYTES - 1)
}

// ---------------------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------------------

const MODE_OFF: u32 = 0;
const MODE_ARMED: u32 = 1;

static INSTALLED: AtomicBool = AtomicBool::new(false);
static MODE: AtomicU32 = AtomicU32::new(MODE_OFF);
/// The size class we quarantine (`0x40` by default).
static WATCHED: AtomicUsize = AtomicUsize::new(0);
/// Base of the reservation, and a bump cursor in pages. The cursor only ever advances: a freed
/// page's address is retired, which is the whole point.
static RESERVE_BASE: AtomicUsize = AtomicUsize::new(0);
static CURSOR: AtomicUsize = AtomicUsize::new(0);
/// The original allocator, reached for every non-quarantined size.
static ALLOC_TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
/// The real `HeapFree`, reached for every pointer that is not one of ours.
static REAL_HEAPFREE: AtomicUsize = AtomicUsize::new(0);
static HEAPFREE_SLOT: AtomicUsize = AtomicUsize::new(0);

static SERVED: AtomicU64 = AtomicU64::new(0);
static FREED: AtomicU64 = AtomicU64::new(0);
static FALLBACK: AtomicU64 = AtomicU64::new(0);
static CATCHES: AtomicU64 = AtomicU64::new(0);
static CATCH_LOGS: AtomicU32 = AtomicU32::new(0);
const MAX_CATCH_LOGS: u32 = 64;

/// The client's executable sections as `[lo, hi)`, from the PE header at the module base,
/// read once at arm. On this build: `.text`, `.themida` and `.boot`. A return address is only
/// believed if it lies in one of these - see [`caller_ra`].
const MAX_EXEC: usize = 8;
const ZERO_USIZE: AtomicUsize = AtomicUsize::new(0);
static EXEC_LO: [AtomicUsize; MAX_EXEC] = [ZERO_USIZE; MAX_EXEC];
static EXEC_HI: [AtomicUsize; MAX_EXEC] = [ZERO_USIZE; MAX_EXEC];
static EXEC_N: AtomicUsize = AtomicUsize::new(0);

/// The executable sections of a PE image whose first bytes are `hdr`, as absolute
/// `[lo, hi)` ranges at `base`. Returns the ranges and how many are filled; a header that does
/// not parse yields zero.
pub(crate) fn parse_exec_ranges(hdr: &[u8], base: usize) -> ([(usize, usize); MAX_EXEC], usize) {
    const IMAGE_SCN_MEM_EXECUTE: u32 = 0x2000_0000;
    let mut out = [(0usize, 0usize); MAX_EXEC];
    let rd32 = |at: usize| -> Option<u32> {
        hdr.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    let rd16 = |at: usize| -> Option<u16> { hdr.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]])) };
    if hdr.get(0..2) != Some(b"MZ") {
        return (out, 0);
    }
    let Some(pe) = rd32(0x3c).map(|v| v as usize) else { return (out, 0) };
    if hdr.get(pe..pe + 4) != Some(b"PE\0\0") {
        return (out, 0);
    }
    let (Some(nsec), Some(optsz)) = (rd16(pe + 6), rd16(pe + 20)) else { return (out, 0) };
    let mut n = 0;
    for i in 0..nsec as usize {
        let sec = pe + 24 + optsz as usize + i * 40;
        let (Some(vsize), Some(va), Some(ch)) = (rd32(sec + 8), rd32(sec + 12), rd32(sec + 36)) else {
            break;
        };
        if ch & IMAGE_SCN_MEM_EXECUTE != 0 && vsize != 0 && n < MAX_EXEC {
            out[n] = (base + va as usize, base + va as usize + vsize as usize);
            n += 1;
        }
    }
    (out, n)
}

/// Is `v` a return address worth recording: inside one of the client's executable sections
/// and not inside the pool's own routines.
pub(crate) fn accept_ra(v: usize, base: usize, exec: &[(usize, usize)]) -> bool {
    if base != 0 && v >= base + POOL_CODE_LO_RVA && v < base + POOL_CODE_HI_RVA {
        return false;
    }
    exec.iter().any(|&(lo, hi)| v >= lo && v < hi)
}

/// The self-test's own page and the address it stores to.
static PROBE_PAGE: AtomicUsize = AtomicUsize::new(0);
static PROBE_SAW: AtomicUsize = AtomicUsize::new(0);
static SELF_TEST_OK: AtomicBool = AtomicBool::new(false);

/// One page's metadata, kept **outside** the guard page so it survives the decommit and can
/// name the allocator and freer when a stale write finally arrives. Indexed by page number
/// within the reserve. Committed once at install (512 K × 32 B = 16 MB).
#[repr(C)]
struct Meta {
    alloc_ra: AtomicU64,
    free_ra: AtomicU64,
    alloc_tick: AtomicU64,
    /// 0 never used, 1 live, 2 quarantined (decommitted), 3 caught (recommitted after a stale
    /// write).
    state: AtomicU32,
    _pad: AtomicU32,
}

static META: AtomicUsize = AtomicUsize::new(0); // *mut Meta, MAX_SLOTS long

unsafe fn meta(i: usize) -> &'static Meta {
    &*((META.load(Ordering::SeqCst) as *const Meta).add(i))
}

/// Is `addr` inside the reservation, and if so which page index?
fn slot_index(base: usize, addr: usize) -> Option<usize> {
    if base == 0 || addr < base || addr >= base + RESERVE_BYTES {
        return None;
    }
    Some((page_of(addr) - base) / PAGE_BYTES)
}

// ---------------------------------------------------------------------------------------
// The allocator detour
// ---------------------------------------------------------------------------------------

/// `alloc(ctx, size)`. Quarantines a request of the watched class from our own reserve;
/// everything else goes to the client's own allocator through the trampoline.
unsafe extern "system" fn alloc_detour(ctx: usize, size: usize) -> usize {
    let tramp: extern "system" fn(usize, usize) -> usize =
        std::mem::transmute(ALLOC_TRAMPOLINE.load(Ordering::SeqCst));

    if MODE.load(Ordering::SeqCst) != MODE_ARMED
        || ctx != crate::hook::base() + POOL_CTX_RVA
        || size_class(size) != Some(WATCHED.load(Ordering::SeqCst))
    {
        return tramp(ctx, size);
    }

    // Who allocated this slot, for the eventual fault report. Best-effort stack scan, the
    // same one the free path uses; a wrong value only makes a worse log line.
    let ra = caller_ra();
    match quarantine_alloc(ra) {
        Some(body) => body,
        None => {
            FALLBACK.fetch_add(1, Ordering::SeqCst);
            tramp(ctx, size)
        }
    }
}

/// Take the next page, commit it, stamp the header, and return the body.
unsafe fn quarantine_alloc(alloc_ra: usize) -> Option<usize> {
    let base = RESERVE_BASE.load(Ordering::SeqCst);
    if base == 0 {
        return None;
    }
    let i = CURSOR.fetch_add(1, Ordering::SeqCst);
    if i >= MAX_SLOTS {
        return None; // reserve exhausted; caller falls back
    }
    let page = base + i * PAGE_BYTES;
    if VirtualAlloc(page as *mut c_void, PAGE_BYTES, MEM_COMMIT, PAGE_READWRITE).is_null() {
        return None;
    }
    let body = page + SLOT_BODY_OFF;
    std::ptr::write_volatile((page + SLOT_HEADER_OFF) as *mut u64, QUARANTINE_HEADER);
    let m = meta(i);
    m.alloc_ra.store(alloc_ra as u64, Ordering::SeqCst);
    m.free_ra.store(0, Ordering::SeqCst);
    m.alloc_tick.store(GetTickCount64(), Ordering::SeqCst);
    m.state.store(1, Ordering::SeqCst);
    SERVED.fetch_add(1, Ordering::SeqCst);
    Some(body)
}

// ---------------------------------------------------------------------------------------
// The free interception (HeapFree pointer swap)
// ---------------------------------------------------------------------------------------

/// Stand in for `HeapFree(heap, flags, mem)`. The client's free passes `mem = body − 8`
/// (`lea r8,[rdi-8]`), which for a quarantined slot is `page + 8`.
unsafe extern "system" fn heapfree_shim(heap: usize, flags: u32, mem: usize) -> i32 {
    let base = RESERVE_BASE.load(Ordering::SeqCst);
    if let Some(i) = slot_index(base, mem) {
        let page = page_of(mem);
        let m = meta(i);
        m.free_ra.store(caller_ra() as u64, Ordering::SeqCst);
        m.state.store(2, Ordering::SeqCst);
        // Decommit: the address is retired for good. A later write here faults in the VEH.
        VirtualFree(page as *mut c_void, PAGE_BYTES, MEM_DECOMMIT);
        FREED.fetch_add(1, Ordering::SeqCst);
        return 1; // BOOL TRUE, as HeapFree returns on success
    }
    let real: extern "system" fn(usize, u32, usize) -> i32 =
        std::mem::transmute(REAL_HEAPFREE.load(Ordering::SeqCst));
    real(heap, flags, mem)
}

/// The caller's return address, best-effort: the first value up the stack from here that
/// [`accept_ra`] believes. Used only for the per-page metadata, where a wrong value is a
/// worse log line, never a fault.
///
/// Two filters, both from the review of 2026-09-08, before the first launch:
///
/// * **Executable sections only, not "the image".** The first version accepted any value in
///   the 128 MB image, and the pool context `0x143ad68a0` - a `.data` address that is the
///   allocator's first argument and sits in its home slot - qualified.
/// * **Skip the pool's own code.** From [`heapfree_shim`] the nearest client address up the
///   stack is always `0x14019bbf3`, the return into the free that called `HeapFree` - so every
///   catch would have read "freed from 0x14019bbf3", which names the pool and not the freer.
///   Both frees are `push rdi; sub rsp,0x20` frames [L], so the caller worth naming is one
///   frame further up; skipping `[0x14019b4e0, 0x14019bc60)` reaches it.
///
/// From [`alloc_detour`], entered by the jump at the allocator's first byte, the first
/// accepted value is the allocator's caller directly.
#[inline(never)]
unsafe fn caller_ra() -> usize {
    let mut rsp: usize;
    std::arch::asm!("mov {}, rsp", out(reg) rsp);
    let base = crate::hook::base();
    let n = EXEC_N.load(Ordering::SeqCst).min(MAX_EXEC);
    let mut exec = [(0usize, 0usize); MAX_EXEC];
    for (i, slot) in exec.iter_mut().enumerate().take(n) {
        *slot = (EXEC_LO[i].load(Ordering::SeqCst), EXEC_HI[i].load(Ordering::SeqCst));
    }
    // 0x180 bytes: this frame, the shim's, the free's 0x28-byte frame and its return slot.
    for off in (0..0x180).step_by(8) {
        let v = std::ptr::read_volatile((rsp + off) as *const usize);
        if accept_ra(v, base, &exec[..n]) {
            return v;
        }
    }
    0
}

// ---------------------------------------------------------------------------------------
// The handler
// ---------------------------------------------------------------------------------------

unsafe extern "system" fn veh(info: *mut ExceptionPointers) -> i32 {
    if info.is_null() {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let p = info.cast::<ExceptionPointers>();
    let rec = (*p).record;
    if rec.is_null() || *(rec.add(REC_CODE).cast::<u32>()) != EXCEPTION_ACCESS_VIOLATION {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    if (*(rec.add(REC_PARAMS).cast::<u32>()) as usize) < 2 {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let kind = *(rec.add(REC_INFO).cast::<usize>());
    let target = *(rec.add(REC_INFO + 8).cast::<usize>());

    // The self-test's own page: record and recommit, nothing else.
    let probe = PROBE_PAGE.load(Ordering::SeqCst);
    if probe != 0 && page_of(target) == probe {
        PROBE_SAW.store(target, Ordering::SeqCst);
        VirtualAlloc(probe as *mut c_void, PAGE_BYTES, MEM_COMMIT, PAGE_READWRITE);
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    let base = RESERVE_BASE.load(Ordering::SeqCst);
    let Some(i) = slot_index(base, target) else {
        return EXCEPTION_CONTINUE_SEARCH; // not ours - the client's, or another watcher's
    };
    // A fault on a reserved page in our range is a use of retired memory: THE catch. Recommit
    // so the access completes and the client runs on to the next one.
    CATCHES.fetch_add(1, Ordering::SeqCst);
    let page = page_of(target);
    let rip = *((*p).context.cast::<u8>().add(CTX_RIP).cast::<u64>()) as usize;
    let m = meta(i);
    let is_write = kind == 1;
    if CATCH_LOGS.fetch_add(1, Ordering::SeqCst) < MAX_CATCH_LOGS {
        // Fixed static ring would be safer, but this event is rare (a stale access, at most a
        // few a session) and the log write is out of GRAP64's heap, not the client's - the
        // same trade `crate::freeguard`'s refusal log makes.
        log(&format!(
            "***** GUARD PAGE - STALE {} at {target:#x} (slot page {page:#x}, retired): the \
             instruction is RIP {rip:#x}{}, tid {}. This slot was allocated from {:#x}{} and \
             freed from {:#x}{}. THE WRITER holds a pointer into memory it no longer owns; this \
             is the instruction that uses it. Recommitting the page so the client continues. \
             The 32-bit increment family, caught at the source *****",
            if is_write { "WRITE" } else { "READ" },
            crate::netwatch::module_of(rip),
            GetCurrentThreadId(),
            m.alloc_ra.load(Ordering::SeqCst),
            crate::netwatch::module_of(m.alloc_ra.load(Ordering::SeqCst) as usize),
            m.free_ra.load(Ordering::SeqCst),
            crate::netwatch::module_of(m.free_ra.load(Ordering::SeqCst) as usize),
        ));
    }
    m.state.store(3, Ordering::SeqCst);
    VirtualAlloc(page as *mut c_void, PAGE_BYTES, MEM_COMMIT, PAGE_READWRITE);
    // Re-stamp the header so a subsequent free of the recommitted slot still diverts here
    // rather than corrupting the pool.
    std::ptr::write_volatile((page + SLOT_HEADER_OFF) as *mut u64, QUARANTINE_HEADER);
    EXCEPTION_CONTINUE_EXECUTION
}

/// Is this exception one of ours - a fault on a retired page in the reserve, or on the
/// self-test's page? `probe.rs` asks before it treats a fault as a client crash, exactly as
/// it asks `writewatch`. Its handler is registered when the first watch arms, seconds after
/// this module's, and a first-chance handler registered later runs FIRST - so without this a
/// guard-page catch would be logged as `CLIENT FAULT` and would write a crash dump before our
/// handler ever saw it.
pub(crate) unsafe fn suppresses(info: *mut c_void) -> bool {
    if info.is_null() {
        return false;
    }
    let p = info.cast::<ExceptionPointers>();
    let rec = (*p).record;
    if rec.is_null() || *(rec.add(REC_CODE).cast::<u32>()) != EXCEPTION_ACCESS_VIOLATION {
        return false;
    }
    if (*(rec.add(REC_PARAMS).cast::<u32>()) as usize) < 2 {
        return false;
    }
    let target = *(rec.add(REC_INFO + 8).cast::<usize>());
    let probe = PROBE_PAGE.load(Ordering::SeqCst);
    if probe != 0 && page_of(target) == probe {
        return true;
    }
    slot_index(RESERVE_BASE.load(Ordering::SeqCst), target).is_some()
}

// ---------------------------------------------------------------------------------------
// Installation
// ---------------------------------------------------------------------------------------

/// Prove the handler catches a write to a retired page of our own, and recommits it. See the
/// module docs.
unsafe fn self_test() -> Result<(), String> {
    let page = VirtualAlloc(std::ptr::null_mut(), PAGE_BYTES, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE)
        as usize;
    if page == 0 {
        return Err("VirtualAlloc for the control page failed".into());
    }
    let at = page + 0x40;
    PROBE_SAW.store(0, Ordering::SeqCst);
    PROBE_PAGE.store(page, Ordering::SeqCst);
    // Retire it, then write - the handler must fault, record, and recommit so the store lands.
    VirtualFree(page as *mut c_void, PAGE_BYTES, MEM_DECOMMIT);
    std::ptr::write_volatile(at as *mut u32, 0x5555_5555);
    let saw = PROBE_SAW.load(Ordering::SeqCst);
    let landed = std::ptr::read_volatile(at as *const u32);
    PROBE_PAGE.store(0, Ordering::SeqCst);
    VirtualFree(page as *mut c_void, 0, 0x8000 /* MEM_RELEASE */);
    if saw != at {
        return Err(format!(
            "the handler did not catch the control write ({saw:#x}) - the VEH is not installed \
             or EXCEPTION_RECORD is not laid out as assumed"
        ));
    }
    if landed != 0x5555_5555 {
        return Err("the control write did not land after recommit".into());
    }
    Ok(())
}

/// Arm the quarantine if `guardpage=<hex slot size>` is in the session marker.
pub fn install() {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    let Some(tok) = crate::session::marker_token("guardpage=") else {
        return;
    };
    let watched = tok
        .trim()
        .strip_prefix("0x")
        .and_then(|h| usize::from_str_radix(h, 16).ok())
        .or_else(|| tok.trim().parse::<usize>().ok());
    let Some(watched) = watched.filter(|w| [0x10, 0x20, 0x40, 0x80].contains(w)) else {
        log(&format!(
            "***** GUARD PAGE: guardpage={tok:?} is not one of 0x10/0x20/0x40/0x80 - standing \
             down *****"
        ));
        return;
    };
    std::thread::spawn(move || unsafe { arm(watched) });
}

unsafe fn arm(watched: usize) {
    let base = crate::hook::base();
    if base == 0 {
        log("***** GUARD PAGE: no module base - NOT armed *****");
        return;
    }

    // Register the handler and prove it works before anything is hooked.
    if !register_veh() {
        log("***** GUARD PAGE: AddVectoredExceptionHandler failed - NOT armed *****");
        return;
    }
    if let Err(e) = self_test() {
        log(&format!("***** GUARD PAGE: control FAIL: {e}. NOT arming *****"));
        return;
    }
    SELF_TEST_OK.store(true, Ordering::SeqCst);

    // The reservation and its metadata.
    let reserve = VirtualAlloc(std::ptr::null_mut(), RESERVE_BYTES, MEM_RESERVE, PAGE_NOACCESS) as usize;
    if reserve == 0 {
        log("***** GUARD PAGE: could not reserve 2 GB of address space - NOT armed *****");
        return;
    }
    let meta_bytes = MAX_SLOTS * std::mem::size_of::<Meta>();
    let meta_ptr = VirtualAlloc(std::ptr::null_mut(), meta_bytes, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE) as usize;
    if meta_ptr == 0 {
        VirtualFree(reserve as *mut c_void, 0, 0x8000);
        log("***** GUARD PAGE: could not commit the metadata array - NOT armed *****");
        return;
    }
    RESERVE_BASE.store(reserve, Ordering::SeqCst);
    META.store(meta_ptr, Ordering::SeqCst);
    WATCHED.store(watched, Ordering::SeqCst);

    // Which addresses count as a caller. Read from the mapped image's own header so a
    // rebuilt client cannot silently move `.text` out from under a constant.
    let hdr = std::slice::from_raw_parts(base as *const u8, PAGE_BYTES);
    let (exec, n) = parse_exec_ranges(hdr, base);
    for i in 0..n {
        EXEC_LO[i].store(exec[i].0, Ordering::SeqCst);
        EXEC_HI[i].store(exec[i].1, Ordering::SeqCst);
    }
    EXEC_N.store(n, Ordering::SeqCst);
    if n == 0 {
        log("***** GUARD PAGE: the PE header at the module base did not parse; caller \
             attribution will read 0 on every catch (the catch itself is unaffected) *****");
    }

    // The free interception first (a pointer swap; nothing depends on ordering with alloc, and
    // doing it before the alloc hook means no quarantined slot can be served before its free
    // path exists).
    let slot = base + HEAPFREE_SLOT_RVA;
    if !crate::session::can_read(slot, 8) {
        log(&format!("***** GUARD PAGE: HeapFree slot {slot:#x} unreadable - NOT armed *****"));
        return;
    }
    let real = std::ptr::read_volatile(slot as *const usize);
    REAL_HEAPFREE.store(real, Ordering::SeqCst);
    HEAPFREE_SLOT.store(slot, Ordering::SeqCst);
    let shim: unsafe extern "system" fn(usize, u32, usize) -> i32 = heapfree_shim;
    std::ptr::write_volatile(slot as *mut usize, shim as usize);
    if std::ptr::read_volatile(slot as *const usize) != shim as usize {
        log("***** GUARD PAGE: the HeapFree swap did not take - NOT arming the allocator *****");
        return;
    }

    // The allocator hook last. If it refuses (prologue mismatch), undo the free swap so the
    // client is left exactly as it was.
    let detour: unsafe extern "system" fn(usize, usize) -> usize = alloc_detour;
    // `install_detour` stores the trampoline into ALLOC_TRAMPOLINE itself, BEFORE the jump
    // into `alloc_detour` exists, and parks every other thread outside the prologue while the
    // bytes change. Storing it here, after the return, left a gap in which the allocator -
    // called from thirty threads - jumped through a zero.
    match crate::identity::install_detour(
        base + ALLOC_RVA,
        detour as usize,
        ALLOC_PROLOGUE.len(),
        &ALLOC_PROLOGUE,
        &ALLOC_TRAMPOLINE,
    ) {
        Some(_tramp) => {
            MODE.store(MODE_ARMED, Ordering::SeqCst);
            log(&format!(
                "***** GUARD PAGE ARMED: size class {watched:#x} is served one-slot-per-page \
                 from a 2 GB reserve at {reserve:#x} and DECOMMITTED on free, never reused. A \
                 stale write or read into a freed slot faults at the instruction that makes it, \
                 which the handler logs (RIP, target, who allocated, who freed) and recommits so \
                 the client continues. control PASS. The pool's own chain, free list and \
                 counters are untouched - our slots take the HeapFree arm. This is the writer's \
                 arbitrary stale-pointer surface, the one the write watch cannot reach *****"
            ));
        }
        None => {
            // Undo the swap; leave nothing behind.
            std::ptr::write_volatile(slot as *mut usize, real);
            log("***** GUARD PAGE: the allocator hook was refused (prologue mismatch, or a \
                 thread would not leave it - the identity: line above says which) - HeapFree \
                 swap reverted, NOT armed *****");
        }
    }
}

static VEH_REGISTERED: AtomicBool = AtomicBool::new(false);
unsafe fn register_veh() -> bool {
    if VEH_REGISTERED.swap(true, Ordering::SeqCst) {
        return true;
    }
    !AddVectoredExceptionHandler(1, veh as *const c_void).is_null()
}

/// `(served, freed, live, catches, fallback)` for the heartbeat.
pub(crate) fn counters() -> (u64, u64, u64, u64, u64) {
    let served = SERVED.load(Ordering::SeqCst);
    let freed = FREED.load(Ordering::SeqCst);
    (served, freed, served.saturating_sub(freed), CATCHES.load(Ordering::SeqCst), FALLBACK.load(Ordering::SeqCst))
}

pub(crate) fn armed() -> bool {
    MODE.load(Ordering::SeqCst) == MODE_ARMED
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The client's own section table, read off `client-patched/MapleStory.exe`: three
    /// executable sections, and the two addresses the first version of `caller_ra` got wrong.
    #[test]
    fn a_return_address_is_believed_only_in_an_executable_section_outside_the_pool() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../client-patched/MapleStory.exe");
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("skipped: {path} is not present");
            return;
        };
        let base = 0x1_4000_0000usize;
        let (exec, n) = parse_exec_ranges(&bytes[..PAGE_BYTES], base);
        let exec = &exec[..n];
        assert_eq!(n, 3, "{exec:x?}");
        assert_eq!(exec[0].0, base + 0x1000, ".text");
        assert!(exec.iter().any(|&(lo, _)| lo == base + 0x3d87000), ".themida: {exec:x?}");
        assert!(exec.iter().any(|&(lo, _)| lo == base + 0x5173000), ".boot: {exec:x?}");

        // The rdx=5 caller in the Themida region that precedes every catch by ~100 ms.
        assert!(accept_ra(0x14491cafd, base, exec));
        // A caller in .text.
        assert!(accept_ra(0x140ca61d0, base, exec));
        // The pool context: in the image, in .data, on the stack in the allocator's home slot.
        assert!(!accept_ra(0x143ad68a0, base, exec), "a data address is not a caller");
        // The return into the free that called HeapFree - the pool naming itself.
        assert!(!accept_ra(0x14019bbf3, base, exec));
        assert!(!accept_ra(0x14019b58e, base, exec), "the other free's return, too");
        assert!(!accept_ra(0x14019b4e0, base, exec));
        assert!(accept_ra(0x14019bc60, base, exec), "the range ends where the free does");
        // Outside every module.
        assert!(!accept_ra(0x7ffe_0000_0000, base, exec));
        assert!(!accept_ra(0, base, exec));
    }

    #[test]
    fn a_header_that_is_not_a_pe_yields_no_ranges() {
        assert_eq!(parse_exec_ranges(&[0u8; 64], 0x1_4000_0000).1, 0);
        assert_eq!(parse_exec_ranges(b"MZ", 0x1_4000_0000).1, 0);
        let mut junk = vec![0u8; 0x200];
        junk[..2].copy_from_slice(b"MZ");
        junk[0x3c..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        assert_eq!(parse_exec_ranges(&junk, 0x1_4000_0000).1, 0, "no PE signature");
    }

    #[test]
    fn the_size_ladder_matches_the_pools_four_classes() {
        assert_eq!(size_class(1), Some(0x10));
        assert_eq!(size_class(0x10), Some(0x10));
        assert_eq!(size_class(0x11), Some(0x20));
        assert_eq!(size_class(0x20), Some(0x20));
        assert_eq!(size_class(0x21), Some(0x40));
        assert_eq!(size_class(0x40), Some(0x40));
        assert_eq!(size_class(0x41), Some(0x80));
        assert_eq!(size_class(0x80), Some(0x80));
        assert_eq!(size_class(0x81), None, "the big-block path is never quarantined");
        assert_eq!(size_class(0), Some(0x10));
    }

    #[test]
    fn a_slot_index_is_only_inside_the_reserve() {
        let base = 0x1_0000_0000;
        assert_eq!(slot_index(0, base), None, "no reserve yet");
        assert_eq!(slot_index(base, base - 1), None);
        assert_eq!(slot_index(base, base), Some(0));
        assert_eq!(slot_index(base, base + 0x10), Some(0), "the body of slot 0");
        assert_eq!(slot_index(base, base + PAGE_BYTES), Some(1));
        assert_eq!(slot_index(base, base + PAGE_BYTES + 0x40), Some(1));
        assert_eq!(slot_index(base, base + RESERVE_BYTES - 1), Some(MAX_SLOTS - 1));
        assert_eq!(slot_index(base, base + RESERVE_BYTES), None, "one past the end");
    }

    #[test]
    fn a_freed_pointer_from_the_client_arrives_as_body_minus_eight() {
        // The client's free does `lea r8,[rdi-8]` with rdi=body, and our body is page+0x10,
        // so the pointer HeapFree receives is page+8 - still inside the same page, so the
        // slot index and the page both resolve.
        let base = 0x2_0000_0000;
        let page = base + 5 * PAGE_BYTES;
        let body = page + SLOT_BODY_OFF;
        let mem = body - 8;
        assert_eq!(page_of(mem), page);
        assert_eq!(slot_index(base, mem), Some(5));
    }

    #[test]
    fn the_quarantine_header_takes_the_heapfree_arm_not_the_pool_arm() {
        // The free ladders on the header value: <=0x80 is a pool bucket, >0x80 falls through
        // to HeapFree. Our stamp must be strictly greater than the largest bucket.
        assert!(QUARANTINE_HEADER > 0x80);
        for bucket in [0x10u64, 0x20, 0x40, 0x80] {
            assert_ne!(QUARANTINE_HEADER, bucket);
        }
    }

    #[test]
    fn the_reserve_is_whole_pages_and_the_geometry_is_consistent() {
        assert_eq!(RESERVE_BYTES % PAGE_BYTES, 0);
        assert_eq!(MAX_SLOTS, RESERVE_BYTES / PAGE_BYTES);
        assert_eq!(SLOT_HEADER_OFF + 8, SLOT_BODY_OFF, "header sits at body-8");
        assert!(SLOT_BODY_OFF < PAGE_BYTES);
    }
}
