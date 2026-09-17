//! A quarantine allocator for **one or more** pool size classes, so a write through a **stale
//! pointer into freed-and-reused memory** faults *at the writing instruction* instead of days
//! later in someone else's free.
//!
//! # The disease this is built for
//!
//! Five write-watch runs (`research/the-180-second-clock-2026-09-07.md`) point at one writer
//! with one habit: it holds a pointer into pool memory that has since been freed and handed
//! back out, and writes a small increment through it on a timer. The damage has now been seen
//! on **three different size classes**:
//!
//! | when | victim | damage |
//! |---|---|---|
//! | 2026-09-07 runs 2 and 5 | a live `0x40` map node | a pointer's low dword `+2` |
//! | 2026-09-08 overnight | a live `0x20` red-black tree node | a pointer's high dword to `-1` |
//! | 2026-09-08 the 1 h 57 m run | a live `0x40` object | a vtable pointer `+2` |
//!
//! `[I]`, and it is why this module now takes a *set* of classes: **the class is probably
//! incidental.** The writer holds a stale ADDRESS; whichever bucket's chunk is later carved
//! over that address is the victim. Quarantining one class is whack-a-mole, and the run of
//! 2026-09-08 12:01 proved it - `0x20` was quarantined for 1 h 57 m and the client died on a
//! `0x40` slot.
//!
//! # What it does
//!
//! While armed, an allocation of a watched class is served from its **own page** in a large
//! private reservation, and its free **decommits that page and holds the address back** for
//! [`REUSE_AFTER_MS`]. A genuine allocation that outlives its free is invisible - the client
//! stops touching a slot it freed. A *stale* pointer is not: the next access through it hits a
//! decommitted page and faults, and the handler names the instruction, the address, the return
//! address that allocated the slot, the one that freed it, and how long ago each happened, then
//! recommits the page and continues so the client survives to the next one.
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
//!   [`QUARANTINE_HEADER`], so the client's own free takes its `HeapFree` arm rather than the
//!   pool arm - keeping our slots entirely off the pool's free-list and live-counter - and we
//!   intercept by swapping the cached `HeapFree` pointer at [`HEAPFREE_SLOT_RVA`], exactly as
//!   `crate::freeguard` swaps PCOM's.
//!
//! Our slots never touch the pool's chunk chain, free list or counters, so `poolsentry` and
//! `tools/poolchain.py` are unaffected and keep walking the real pool.
//!
//! # The header stamp is class-independent, and that was checked, not assumed
//!
//! `research/heapfix-did-not-hold.md` §1 disassembles all three standalone frees. Every one of
//! them ladders **on the header value alone** and never on the requested size:
//!
//! ```text
//! 14019bb63  mov  rax,[rdx-8]        ; the header
//! 14019bb7f  cmp  rax,0x20 / ja      ; -> 14019bb90
//! 14019bb90  cmp  rax,0x40 / ja      ; -> 14019bbc4
//! 14019bbc4  cmp  rax,0x80 / mov ecx,-1 / cmovbe ecx,3
//! 14019bbd7  test ecx,ecx / jns      ; not taken for a header > 0x80
//! 14019bbdb  ...HeapFree(heap, 0, ptr-8)
//! ```
//!
//! So one stamp `> 0x80` diverts **every** class we serve, and the third free
//! (`0x14019ba40`, ladder `0x28/0x38/0x58/0x98`) is covered by the same value as long as it is
//! also `> 0x98`. `0x100` is. `[L]` from the listing; the test
//! [`the_quarantine_header_takes_the_heapfree_arm_on_both_ladders`] pins both ladders.
//!
//! What is **not** proved for a new class is that its frees all reach the *cached pointer we
//! swapped*: the 2026-09-08 run showed 1.02 M `0x20` frees coming back through
//! [`heapfree_shim`] `[L]`, but 53 of the 56 ladder sites are inlined into ordinary functions
//! and none of those was traced. `[I]` So this module now logs the **first free of each class**
//! and the heartbeat shouts if a class is served and never freed - that is the liveness control
//! for a newly-added class, and it answers in seconds rather than in a morning.
//!
//! # Off by default, a named set of classes, and bounded
//!
//! Armed only by `guardpage=<classes>` in the session marker: `0x20`, `0x20+0x40`, or `all`.
//! The reserve is address space only; committed memory is one page per *live* slot, and a
//! decommitted page costs nothing but its address. If the cursor runs out, allocation falls
//! back to the client's own and says so **per class**.
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
const MEM_RELEASE: u32 = 0x8000;

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
// The size classes
// ---------------------------------------------------------------------------------------

/// The pool's four buckets, in the order the allocator's own ladder tests them. A *bucket* is
/// an index `0..4` into this; a *class* is the byte size.
pub(crate) const CLASSES: [usize; 4] = [0x10, 0x20, 0x40, 0x80];
pub(crate) const NCLASS: usize = CLASSES.len();

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

/// The bucket index of a class size, or `None` if it is not one of the four.
pub(crate) fn bucket_of(class: usize) -> Option<usize> {
    CLASSES.iter().position(|&c| c == class)
}

/// The bucket a request of `size` lands in.
pub(crate) fn size_bucket(size: usize) -> Option<usize> {
    size_class(size).and_then(bucket_of)
}

/// Parse the `guardpage=` token into a bucket mask.
///
/// Accepted: one class (`0x20`), several joined by `+` or `|` (`0x20+0x40`), or the word
/// `all`. **Not** commas - the session marker is itself comma-separated, so a comma here would
/// split the token in [`crate::session::marker_token`] and silently arm half of what was asked
/// for. A bare number without `0x` is read as **decimal**, which is what the single-class
/// parser did before this and is kept so an old command line means what it always meant
/// (`32` is `0x20`).
///
/// Returns the mask, or a message naming exactly what was rejected - never a silent zero.
pub(crate) fn parse_classes(tok: &str) -> Result<u32, String> {
    let t = tok.trim().to_ascii_lowercase();
    if t.is_empty() {
        return Err("the token is empty".into());
    }
    if t == "all" {
        return Ok((1 << NCLASS) - 1);
    }
    let mut mask = 0u32;
    for part in t.split(['+', '|']) {
        let p = part.trim();
        if p.is_empty() {
            return Err(format!("{tok:?} has an empty term - write it as 0x20+0x40"));
        }
        let v = match p.strip_prefix("0x") {
            Some(h) => usize::from_str_radix(h, 16).ok(),
            None => p.parse::<usize>().ok(),
        };
        match v.and_then(bucket_of) {
            Some(b) => mask |= 1 << b,
            None => {
                return Err(format!(
                    "{p:?} is not one of 0x10/0x20/0x40/0x80 (or `all`, or a `+`-joined list \
                     of them)"
                ))
            }
        }
    }
    Ok(mask)
}

/// The classes a mask names, for a log line.
pub(crate) fn mask_text(mask: u32) -> String {
    let mut s = String::new();
    for (b, &c) in CLASSES.iter().enumerate() {
        if mask & (1 << b) != 0 {
            if !s.is_empty() {
                s.push('+');
            }
            s.push_str(&format!("{c:#x}"));
        }
    }
    if s.is_empty() {
        s.push_str("(none)");
    }
    s
}

// ---------------------------------------------------------------------------------------
// Sizing, and the arithmetic that justifies it
// ---------------------------------------------------------------------------------------

/// **The largest reserve this module will take, in one-page slots: 8 M, i.e. 32 GiB of address
/// space.** Reserved, never committed as a whole; a decommitted page costs only its slot here.
///
/// # Why this number, from the 2026-09-08 12:01 run
///
/// That run quarantined `0x20` with `MAX_SLOTS = 1 048 576` and the cursor was **spent at six
/// minutes**, four minutes before anything could age out. From its heartbeats `[L]`:
///
/// ```text
///    60 s     627 172 served      <- a startup burst, ~10 000/s in the first minute
///   120 s     720 807             +93 635
///   300 s   1 001 708             +93 621     -> 1 560/s sustained
///   360 s   1 048 576 = cap       ***** 47 058 FELL BACK *****
///   600 s   1 050 852             419 588 fell back; recycling begins
/// ```
///
/// Two quantities, and confusing them is what made the first sizing wrong:
///
/// * **steady state** needs one retirement window of churn: `1 560 x 600 = 936 000` slots.
///   That *fits* in the old 1 M. The old size was not wrong for the steady state.
/// * **the first window** starts at zero with nothing to recycle, so it needs the burst too:
///   `627 172 + 1 560 x 540 = 1 469 572`. That does **not** fit in 1 M, and the difference,
///   ~421 K, is the 419 588 that fell back. `[D]`, and it agrees with the measurement to 0.06 %.
///
/// So the binding constraint is the burst, and the fix is a bigger cursor rather than a
/// shorter window. 8 M slots is **5.7 x** one class's measured first window, which leaves room
/// for two classes at that rate with better than 2 x headroom, or all four if the other three
/// together are no worse than `0x20`. Everything above the cursor is address space and lazily
/// committed metadata, so the headroom is nearly free - see [`META_BLOCK_BYTES`].
///
/// # Doubled to 16 M on 2026-09-16, from the 16:46 run - and the reserve is SHARED
///
/// That run armed `0x20+0x40` with 8 M and spent the whole reserve at **~254 s** `[L]`: the
/// `0x20` counter stopped at 7 508 881 and `0x40`'s at 879 727, and those two sum to exactly
/// 8 388 608. The 8 M sizing above modelled each class against its own share, and the arming
/// line said "2.8x for 2 classes"; the truth was one cursor for both and a churn twenty times
/// the 12:01 run's - see [`MEASURED_1646_STEADY_PER_S_0X20`]. The client fell back to its own
/// pool at four and a half minutes, the sentry caught the known header damage two minutes
/// later in a slot the guard was no longer serving, and it died at nine.
///
/// The owner: *"Okay, let's recycle sooner."* The window is the lever ([`REUSE_AFTER_MS`], now 200
/// s), and this doubles the cursor with it because at 200 s the measured pair needs 7.5 M
/// slots in flight and 8 M is a fit, not headroom. 16 M is 64 GiB of address space, a 64 MB
/// ring committed at arm, and metadata that is still committed only as far as the cursor
/// reaches - which, once recycling holds, is about the working set and not the reserve.
pub(crate) const MAX_SLOTS: usize = 16 * 1024 * 1024;

/// The reserve the 16:46 run of 2026-09-16 had, and spent at ~254 s. `[L]`
#[cfg(test)] // the 12:01 and 16:46 runs are what the sizing TESTS argue with
pub(crate) const MAX_SLOTS_UNTIL_0916: usize = 8 * 1024 * 1024;

/// Reserve sizes to try, in slots, largest first. A 32 GiB reservation in a 64-bit process
/// with 128 TB of user address space should never fail - but "should never fail" is how this
/// project has lost runs before, and standing the whole module down because address space was
/// fragmented would be a worse outcome than arming with a smaller cursor and saying so. Every
/// entry is a power of two: the retirement ring indexes with a mask.
const RESERVE_LADDER: [usize; 6] = [
    16 * 1024 * 1024,
    8 * 1024 * 1024,
    4 * 1024 * 1024,
    2 * 1024 * 1024,
    1024 * 1024,
    512 * 1024,
];

/// Where in a guard page the slot sits. The header the client reads at `body − 8` lands at
/// `page + 8`; the body at `page + 0x10` is 16-aligned, as the pool's are. The largest class
/// we serve is `0x80`, so a slot occupies `page..page+0x90` and one page is ample.
const SLOT_BODY_OFF: usize = 0x10;
const SLOT_HEADER_OFF: usize = SLOT_BODY_OFF - 8;

/// **How long a freed slot's address stays retired before it may be handed out again.**
///
/// The original design never reused an address, which is the strongest possible guarantee and
/// cannot last a night: the cursor only advances, so the reserve is a budget of total
/// allocations. So a page is reused only after this long.
///
/// **600 s until 2026-09-16, then 200 s.** The number comes from the writer: it fires on an
/// exact **180 s** clock (`research/the-180-second-clock-2026-09-07.md`). 600 s covered three
/// firings of any pointer taken at the instant of a free, and on 2026-09-08 it was kept there
/// deliberately - the 12:01 run had shown the *cursor* was the constraint, and the cursor is
/// the cheap thing to grow.
///
/// The 16:46 run of 2026-09-16 showed the other case. `0x20` churned at **34 173/s** `[L]`,
/// twenty-two times the 1 560/s the reserve was sized from, so one 600 s window needed 20.5 M
/// slots in flight for the shipped pair and the 8 M reserve was spent at ~254 s - before a
/// single slot could age out, because nothing ages out before the window. Recycling that
/// starts after the reserve is gone protects nothing. The owner: *"Okay, let's recycle sooner."*
///
/// **200 s is one full firing of the writer's clock plus 20 s of slack.** The claim it keeps:
/// a pointer taken at the free is written at most once per 180 s, so any write through it
/// within the first period lands on a decommitted page and is caught. The claim it gives up:
/// the second and third firings. Nothing measured says the writer fires through the same
/// pointer more than once - the catches on record are single writes, `+0x24` and `+0x90` past
/// an array allocated on the tick that writes it - so what is given up is margin, not a
/// measured exposure. [`MAX_SLOTS`] doubled in the same change, so the pair's 200 s window
/// (7.5 M slots) has better than 2x.
///
/// A stale write inside the window lands on a decommitted page and is caught; one after it
/// lands on a live quarantined slot, which is the same exposure the client's own pool has after
/// a few milliseconds.
const REUSE_AFTER_MS: u64 = 200_000;

/// The window every run before 2026-09-16 had. Kept for the 12:01 model's test, which has to
/// reproduce *that* run's fallback with *that* run's window.
#[cfg(test)] // the 12:01 and 16:46 runs are what the sizing TESTS argue with
pub(crate) const REUSE_AFTER_MS_UNTIL_0916: u64 = 600_000;

/// The header value stamped on a quarantined slot: **greater than `0x98`**, so every one of the
/// client's three free ladders (`0x10/0x20/0x40/0x80` twice, `0x28/0x38/0x58/0x98` once) falls
/// through to the `HeapFree` arm we intercept instead of pushing our slot onto a pool free
/// list. `research/heapfix-did-not-hold.md` §1. It does **not** depend on which class the slot
/// was requested as - the ladders read the header and nothing else - which is what makes one
/// stamp correct for all four classes at once.
const QUARANTINE_HEADER: u64 = 0x100;

/// The page containing `addr`.
pub(crate) fn page_of(addr: usize) -> usize {
    addr & !(PAGE_BYTES - 1)
}

// ---- the measurement this sizing has to argue with -------------------------------------
//
// All read off the sentry heartbeat of the 2026-09-08 12:01 run,
// `client-patched/maplecw-hook.log`, the first time the guard page ever ran on a client. It
// quarantined `0x20` only, armed at 12:01:21, and the client lived to 13:58:14 - 1 h 57 m, the
// longest session this project has had.

/// Slots served in the first 60 s: the startup burst, ~10 000/s. `[L]`
#[cfg(test)] // the 12:01 and 16:46 runs are what the sizing TESTS argue with
pub(crate) const MEASURED_BURST_60S: usize = 627_172;
/// Sustained rate afterwards: +93 635, +93 637, +93 643, +93 621 per 60 s. `[L]`
#[cfg(test)] // the 12:01 and 16:46 runs are what the sizing TESTS argue with
pub(crate) const MEASURED_STEADY_PER_S: usize = 1_560;
/// The cursor that run had, and which was spent between its 300 s and 360 s heartbeats. `[L]`
#[cfg(test)] // the 12:01 and 16:46 runs are what the sizing TESTS argue with
pub(crate) const OLD_MAX_SLOTS: usize = 1_048_576;
/// Allocations that had fallen back by the 600 s heartbeat, when recycling began. `[L]`
#[cfg(test)] // the 12:01 and 16:46 runs are what the sizing TESTS argue with
pub(crate) const MEASURED_FALLBACK_AT_600S: usize = 419_588;
/// Live `0x20` slots at every heartbeat after the first three minutes: ~26 000. `[L]`
/// One committed page each, so ~104 MB - not the ~230 MB predicted before the launch.
#[cfg(test)] // the 12:01 and 16:46 runs are what the sizing TESTS argue with
pub(crate) const MEASURED_LIVE_0X20: usize = 26_000;

// ---- the 16:46 run of 2026-09-16, which spent the 8 M reserve in four and a half minutes --
//
// `D:\MapleCW\previous-runs\maplecw-hook-20260916-165530.log`, `0x20+0x40` armed at 16:46:16,
// client dead of `0xC0000374` at 16:55:28. All `[L]`, read off the 60 s heartbeats. The two
// classes share ONE cursor: their served counters both stopped between the 240 s and 300 s
// heartbeats, and 7 508 881 + 879 727 = 8 388 608 exactly.

/// `0x20` served in the first 60 s. `[L]`
pub(crate) const MEASURED_1646_BURST_60S_0X20: usize = 877_768;
/// `0x20` sustained: 60 s -> 240 s served 877 768 -> 7 028 973, i.e. 6 151 205 in 180 s. `[L]`
pub(crate) const MEASURED_1646_STEADY_PER_S_0X20: usize = 34_173;
/// `0x40` served in the first 60 s. `[L]`
pub(crate) const MEASURED_1646_BURST_60S_0X40: usize = 221_237;
/// `0x40` sustained: 221 237 -> 832 568 over the same 180 s. `[L]`
pub(crate) const MEASURED_1646_STEADY_PER_S_0X40: usize = 3_396;
/// Live `0x20` slots at every heartbeat after the first minute: ~54 000, so ~210 MB of
/// committed pages for that class. `[L]`
pub(crate) const MEASURED_1646_LIVE_0X20: usize = 54_000;

/// What an armed set needs from the reserve: `(first window, steady state)`, summed over its
/// classes, because the cursor is shared. `0x20` and `0x40` at their own rates from the 16:46
/// run; `0x10` and `0x80` are unmeasured and take `0x20`'s - the conservative reading, and the
/// one that keeps `all` printing the not-enough-headroom shout until someone measures them.
pub(crate) fn armed_need(mask: u32) -> (usize, usize) {
    let mut first = 0usize;
    let mut steady = 0usize;
    for (bucket, &class) in CLASSES.iter().enumerate() {
        if mask & (1 << bucket) == 0 {
            continue;
        }
        let (burst, rate) = match class {
            0x40 => (MEASURED_1646_BURST_60S_0X40, MEASURED_1646_STEADY_PER_S_0X40),
            _ => (MEASURED_1646_BURST_60S_0X20, MEASURED_1646_STEADY_PER_S_0X20),
        };
        first += first_window_slots(burst, rate, REUSE_AFTER_MS);
        steady += steady_slots(rate, REUSE_AFTER_MS);
    }
    (first, steady)
}

/// Slots one class needs to be covered from a standing start: its first-minute burst plus the
/// rest of one retirement window at the sustained rate. Pure, so the sizing has to argue with
/// the measurement rather than with a comment.
pub(crate) fn first_window_slots(burst_60s: usize, steady_per_s: usize, reuse_ms: u64) -> usize {
    let window_s = (reuse_ms / 1000) as usize;
    burst_60s + steady_per_s * window_s.saturating_sub(60)
}

/// Slots one class needs once it is turning over: one retirement window of churn, no burst.
pub(crate) fn steady_slots(steady_per_s: usize, reuse_ms: u64) -> usize {
    steady_per_s * (reuse_ms / 1000) as usize
}

// ---------------------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------------------

const MODE_OFF: u32 = 0;
const MODE_ARMED: u32 = 1;

static INSTALLED: AtomicBool = AtomicBool::new(false);
static MODE: AtomicU32 = AtomicU32::new(MODE_OFF);
/// Which buckets we quarantine, one bit each. Zero until [`arm`] succeeds.
static WATCHED_MASK: AtomicU32 = AtomicU32::new(0);
/// Base of the reservation, how many slots it actually holds (a power of two from
/// [`RESERVE_LADDER`]), and a bump cursor in pages.
static RESERVE_BASE: AtomicUsize = AtomicUsize::new(0);
static SLOTS: AtomicUsize = AtomicUsize::new(0);
static CURSOR: AtomicUsize = AtomicUsize::new(0);
/// The original allocator, reached for every non-quarantined size.
static ALLOC_TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
/// The real `HeapFree`, reached for every pointer that is not one of ours.
static REAL_HEAPFREE: AtomicUsize = AtomicUsize::new(0);
static HEAPFREE_SLOT: AtomicUsize = AtomicUsize::new(0);

const ZERO_U64: AtomicU64 = AtomicU64::new(0);
/// Per-bucket counters. **Summing across classes is what hides which one is exhausting**, so
/// nothing here is kept as a total; the heartbeat prints them apart.
static SERVED: [AtomicU64; NCLASS] = [ZERO_U64; NCLASS];
static FREED: [AtomicU64; NCLASS] = [ZERO_U64; NCLASS];
static FALLBACK: [AtomicU64; NCLASS] = [ZERO_U64; NCLASS];
static RECYCLED: [AtomicU64; NCLASS] = [ZERO_U64; NCLASS];
/// **Every pool allocation the detour sees, by class, watched or not.** One relaxed increment
/// on a path that already does three atomic loads, and it is the churn measurement nobody has:
/// after the 12:01 run we know `0x20` runs at 1 560/s and we know *nothing at all* about the
/// other three, which is exactly the number needed to size a two- or four-class run. It costs
/// no launch of its own - it rides along on whatever the next run is.
static SEEN: [AtomicU64; NCLASS] = [ZERO_U64; NCLASS];

static CATCHES: AtomicU64 = AtomicU64::new(0);
static CATCH_LOGS: AtomicU32 = AtomicU32::new(0);
const MAX_CATCH_LOGS: u32 = 64;
/// A pointer inside the reserve that was handed to `HeapFree` but never handed out by us.
/// Should be zero forever; a non-zero value means a wild pointer landed in our range.
static ALIEN_FREES: AtomicU64 = AtomicU64::new(0);

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
/// within the reserve.
///
/// **Committed lazily** - see [`ensure_meta`]. At 40 bytes a slot, an 8 M-slot reserve would be
/// 320 MB if it were committed up front, and that array is what stopped the reserve growing
/// after the 12:01 run. Reserved in full, committed in [`META_BLOCK_BYTES`] blocks as the
/// cursor advances, it costs only what the run actually uses: 60 MB for one class's first
/// window, and nothing at all for the 6.5 M slots a two-class run never reaches.
#[repr(C)]
struct Meta {
    alloc_ra: AtomicU64,
    free_ra: AtomicU64,
    alloc_tick: AtomicU64,
    /// When the slot was freed, for [`REUSE_AFTER_MS`]. Zero while live.
    freed_tick: AtomicU64,
    /// 0 never used, 1 live, 2 quarantined (decommitted), 3 caught (recommitted after a stale
    /// write).
    state: AtomicU32,
    /// Which class this slot was handed out as, so the free path can count per class without
    /// re-deriving it from a size it is never told.
    class: AtomicU32,
}

const META_STRIDE: usize = std::mem::size_of::<Meta>();
/// How much `Meta` is committed at a time. 1 MB is 26 214 slots, so at the measured 1 560/s
/// this is one `VirtualAlloc` every ~17 seconds per class - off the hot path in every practical
/// sense, and the block is large enough that the commit ladder never becomes the cost.
const META_BLOCK_BYTES: usize = 1024 * 1024;

static META: AtomicUsize = AtomicUsize::new(0); // *mut Meta, reserved for SLOTS entries
/// Bytes of [`META`] committed so far, from its base. Only ever grows.
static META_COMMITTED: AtomicUsize = AtomicUsize::new(0);

/// Round `need` up to the next commit block, capped at the reservation.
pub(crate) fn commit_target(need: usize, block: usize, total: usize) -> usize {
    if need >= total {
        return total;
    }
    let up = need.div_ceil(block) * block;
    up.min(total)
}

/// Make sure `Meta[i]` is backed before anyone dereferences it.
///
/// Idempotent and lock-free. `MEM_COMMIT` over an already-committed range succeeds, so two
/// threads racing on the same block do redundant work and never corrupt anything; the
/// high-water is published with a CAS so a thread that observes it can dereference safely.
unsafe fn ensure_meta(i: usize) -> bool {
    let need = (i + 1) * META_STRIDE;
    let mut have = META_COMMITTED.load(Ordering::Acquire);
    if need <= have {
        return true;
    }
    let base = META.load(Ordering::SeqCst);
    let total = SLOTS.load(Ordering::SeqCst) * META_STRIDE;
    if base == 0 || need > total {
        return false;
    }
    while need > have {
        let want = commit_target(need, META_BLOCK_BYTES, total);
        if VirtualAlloc(
            (base + have) as *mut c_void,
            want - have,
            MEM_COMMIT,
            PAGE_READWRITE,
        )
        .is_null()
        {
            return false;
        }
        match META_COMMITTED.compare_exchange(have, want, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => return true,
            Err(now) => have = now,
        }
    }
    true
}

/// Is `Meta[i]` backed *right now*? Read-only, for the paths that must not commit: the
/// exception handler (a fault on a reserve page that was never handed out would otherwise make
/// the handler dereference an uncommitted array and fault inside itself) and the free shim.
fn meta_ready(i: usize) -> bool {
    (i + 1) * META_STRIDE <= META_COMMITTED.load(Ordering::Acquire)
}

unsafe fn meta(i: usize) -> &'static Meta {
    &*((META.load(Ordering::SeqCst) as *const Meta).add(i))
}

// ---------------------------------------------------------------------------------------
// The retirement queue
// ---------------------------------------------------------------------------------------
//
// Freed slots in the order they were freed, so the oldest is the first candidate for reuse.
// A `u32` slot index per entry, `SLOTS` of them, reserved and committed in full with the
// reservation. Head and tail are guarded by one spinlock: the critical section is a bounds
// check and one store, shorter than the `lock cmpxchg` the client's own pool takes on every
// allocation, and it is never held across a page operation or a log.
//
// **The ring is committed eagerly and `Meta` is not, and that asymmetry is deliberate.** The
// ring is 4 bytes a slot where `Meta` is 40, so at 8 M slots it is 32 MB against 320 MB - it
// does not scale into the number that blocked the reserve. And its index is `tail & mask`,
// where `tail` counts *every free ever*, not distinct slots: at the measured 1 560/s it sweeps
// the whole array inside 90 minutes whatever the cursor does, so laziness here would defer the
// charge rather than avoid it - while putting a `VirtualAlloc` inside the ring spinlock, on the
// hot free path, in the one module in this crate that has a client run behind it.

static RING: AtomicUsize = AtomicUsize::new(0); // *mut u32, SLOTS long
static RING_HEAD: AtomicUsize = AtomicUsize::new(0); // pop here
static RING_TAIL: AtomicUsize = AtomicUsize::new(0); // push here
static RING_LOCK: AtomicBool = AtomicBool::new(false);

struct RingGuard;
impl Drop for RingGuard {
    fn drop(&mut self) {
        RING_LOCK.store(false, Ordering::Release);
    }
}
fn ring_lock() -> RingGuard {
    while RING_LOCK
        .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        std::hint::spin_loop();
    }
    RingGuard
}

/// Push a freed slot onto the tail of the retirement queue.
unsafe fn retire(i: usize) {
    let ring = RING.load(Ordering::SeqCst);
    let slots = SLOTS.load(Ordering::SeqCst);
    if ring == 0 || slots == 0 {
        return;
    }
    let _g = ring_lock();
    let tail = RING_TAIL.load(Ordering::Relaxed);
    let head = RING_HEAD.load(Ordering::Relaxed);
    // Full is impossible - the queue holds at most `slots` entries and every entry is a
    // distinct slot - but a wrap that would collide is dropped rather than corrupting the
    // queue. A dropped entry is a slot that is simply never reused, the old behaviour.
    if tail.wrapping_sub(head) >= slots {
        return;
    }
    *((ring as *mut u32).add(tail & (slots - 1))) = i as u32;
    RING_TAIL.store(tail.wrapping_add(1), Ordering::Relaxed);
}

/// Take the oldest retired slot **if it has been retired for [`REUSE_AFTER_MS`]**, else
/// `None`. The queue is in free order, so the head is the oldest and one look decides it.
unsafe fn take_reusable(now: u64) -> Option<usize> {
    let ring = RING.load(Ordering::SeqCst);
    let slots = SLOTS.load(Ordering::SeqCst);
    if ring == 0 || slots == 0 {
        return None;
    }
    let _g = ring_lock();
    let head = RING_HEAD.load(Ordering::Relaxed);
    if head == RING_TAIL.load(Ordering::Relaxed) {
        return None;
    }
    let i = *((ring as *const u32).add(head & (slots - 1))) as usize;
    if i >= slots || !meta_ready(i) {
        RING_HEAD.store(head.wrapping_add(1), Ordering::Relaxed);
        return None;
    }
    if !reusable_at(meta(i).freed_tick.load(Ordering::SeqCst), now) {
        return None; // the oldest is not old enough, so none of them is
    }
    RING_HEAD.store(head.wrapping_add(1), Ordering::Relaxed);
    Some(i)
}

/// Whether `freed` is old enough to hand out again at `now`. Pure, for the tests.
pub(crate) fn reusable_at(freed_tick: u64, now: u64) -> bool {
    freed_tick != 0 && now.saturating_sub(freed_tick) >= REUSE_AFTER_MS
}

/// Is `addr` inside the reservation, and if so which page index?
fn slot_index(base: usize, slots: usize, addr: usize) -> Option<usize> {
    if base == 0 || slots == 0 || addr < base || addr >= base + slots * PAGE_BYTES {
        return None;
    }
    Some((page_of(addr) - base) / PAGE_BYTES)
}

// ---------------------------------------------------------------------------------------
// The allocator detour
// ---------------------------------------------------------------------------------------

/// `alloc(ctx, size)`. Quarantines a request of any watched class from our own reserve;
/// everything else goes to the client's own allocator through the trampoline.
unsafe extern "system" fn alloc_detour(ctx: usize, size: usize) -> usize {
    let tramp: extern "system" fn(usize, usize) -> usize =
        std::mem::transmute(ALLOC_TRAMPOLINE.load(Ordering::SeqCst));

    if MODE.load(Ordering::SeqCst) != MODE_ARMED || ctx != crate::hook::base() + POOL_CTX_RVA {
        return tramp(ctx, size);
    }
    let Some(b) = size_bucket(size) else {
        return tramp(ctx, size); // the big-block path
    };
    // Count it whether or not we serve it: this is the per-class churn measurement.
    SEEN[b].fetch_add(1, Ordering::Relaxed);
    if WATCHED_MASK.load(Ordering::SeqCst) & (1 << b) == 0 {
        return tramp(ctx, size);
    }

    // Who allocated this slot, for the eventual fault report. Best-effort stack scan, the
    // same one the free path uses; a wrong value only makes a worse log line.
    let ra = caller_ra();
    match quarantine_alloc(ra, b) {
        Some(body) => body,
        None => {
            FALLBACK[b].fetch_add(1, Ordering::SeqCst);
            tramp(ctx, size)
        }
    }
}

/// Take a page - the oldest one retired longer than [`REUSE_AFTER_MS`] ago, or a fresh one
/// from the cursor - commit it, stamp the header, and return the body.
///
/// The reserve is **shared across every watched class**, and that is not a compromise: a page
/// is a page, the body offset is the same for all four, and the header stamp is the same value
/// for all four, so a slot retired by `0x20` is a perfectly good `0x40` ten minutes later. The
/// alternative - one reserve per class - would need each class's burst sized separately, and
/// three of the four rates are still unmeasured.
unsafe fn quarantine_alloc(alloc_ra: usize, bucket: usize) -> Option<usize> {
    let base = RESERVE_BASE.load(Ordering::SeqCst);
    let slots = SLOTS.load(Ordering::SeqCst);
    if base == 0 || slots == 0 {
        return None;
    }
    let now = GetTickCount64();
    // A slot retired long enough ago is preferred over a fresh one, so the cursor advances
    // only while nothing has aged out and the reserve becomes a working set rather than a
    // budget of total allocations.
    let (i, recycled) = match take_reusable(now) {
        Some(i) => (i, true),
        None => {
            let i = CURSOR.fetch_add(1, Ordering::SeqCst);
            if i >= slots {
                CURSOR.store(slots, Ordering::SeqCst); // do not wrap the counter
                return None; // nothing aged out and no fresh slot: caller falls back
            }
            (i, false)
        }
    };
    if !ensure_meta(i) {
        return None; // the metadata array could not grow: fall back rather than deref it
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
    m.alloc_tick.store(now, Ordering::SeqCst);
    m.freed_tick.store(0, Ordering::SeqCst);
    m.class.store(CLASSES[bucket] as u32, Ordering::SeqCst);
    m.state.store(1, Ordering::SeqCst);
    if recycled {
        RECYCLED[bucket].fetch_add(1, Ordering::SeqCst);
    }
    SERVED[bucket].fetch_add(1, Ordering::SeqCst);
    Some(body)
}

// ---------------------------------------------------------------------------------------
// The free interception (HeapFree pointer swap)
// ---------------------------------------------------------------------------------------

/// Stand in for `HeapFree(heap, flags, mem)`. The client's free passes `mem = body − 8`
/// (`lea r8,[rdi-8]`), which for a quarantined slot is `page + 8`.
unsafe extern "system" fn heapfree_shim(heap: usize, flags: u32, mem: usize) -> i32 {
    let base = RESERVE_BASE.load(Ordering::SeqCst);
    let slots = SLOTS.load(Ordering::SeqCst);
    if let Some(i) = slot_index(base, slots, mem) {
        if !meta_ready(i) {
            // A pointer inside our reservation that we never handed out. Answer TRUE rather
            // than passing it to the real HeapFree, which would be handed a non-heap address.
            ALIEN_FREES.fetch_add(1, Ordering::SeqCst);
            return 1;
        }
        let page = page_of(mem);
        let m = meta(i);
        let class = m.class.load(Ordering::SeqCst) as usize;
        m.free_ra.store(caller_ra() as u64, Ordering::SeqCst);
        m.freed_tick.store(GetTickCount64(), Ordering::SeqCst);
        m.state.store(2, Ordering::SeqCst);
        // Decommit: the address is retired. A write here faults in the VEH until the slot has
        // aged out of the retirement queue - see REUSE_AFTER_MS.
        VirtualFree(page as *mut c_void, PAGE_BYTES, MEM_DECOMMIT);
        retire(i);
        if let Some(b) = bucket_of(class) {
            // **The liveness control for a class we have never quarantined before.** 53 of the
            // 56 ladder sites are inlined and none was traced, so "this class's frees reach the
            // cached pointer we swapped" is an inference until a free actually arrives here.
            // One line, the first time each class comes back, settles it in seconds.
            if FREED[b].fetch_add(1, Ordering::SeqCst) == 0 {
                log(&format!(
                    "guard page: the FIRST free of class {class:#x} came back through our \
                     HeapFree shim - that class's free path IS intercepted and its slots are \
                     being decommitted. (This is the control: a class that is served and never \
                     freed would leak and the heartbeat would say so.)"
                ));
            }
        }
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
    let slots = SLOTS.load(Ordering::SeqCst);
    let Some(i) = slot_index(base, slots, target) else {
        return EXCEPTION_CONTINUE_SEARCH; // not ours - the client's, or another watcher's
    };
    // A fault on a reserved page in our range is a use of retired memory: THE catch. Recommit
    // so the access completes and the client runs on to the next one.
    CATCHES.fetch_add(1, Ordering::SeqCst);
    let page = page_of(target);
    let rip = *((*p).context.cast::<u8>().add(CTX_RIP).cast::<u64>()) as usize;
    let is_write = kind == 1;
    let now = GetTickCount64();
    // **Never dereference `Meta` for a slot the cursor has not reached.** The array is
    // committed lazily, so a wild pointer into the untouched tail of the reservation would
    // otherwise make the handler fault inside itself. Attribution is dropped, the catch is not.
    let ready = meta_ready(i);
    if CATCH_LOGS.fetch_add(1, Ordering::SeqCst) < MAX_CATCH_LOGS {
        // Fixed static ring would be safer, but this event is rare (a stale access, at most a
        // few a session) and the log write is out of GRAP64's heap, not the client's - the
        // same trade `crate::freeguard`'s refusal log makes.
        let attribution = if ready {
            let m = meta(i);
            let alloc_ra = m.alloc_ra.load(Ordering::SeqCst) as usize;
            let free_ra = m.free_ra.load(Ordering::SeqCst) as usize;
            let alloc_tick = m.alloc_tick.load(Ordering::SeqCst);
            let freed_tick = m.freed_tick.load(Ordering::SeqCst);
            format!(
                "This slot was handed out as class {:#x}, allocated from {alloc_ra:#x}{} {} ms \
                 ago and freed from {free_ra:#x}{} {} ms ago",
                m.class.load(Ordering::SeqCst),
                crate::netwatch::module_of(alloc_ra),
                now.saturating_sub(alloc_tick),
                crate::netwatch::module_of(free_ra),
                now.saturating_sub(freed_tick),
            )
        } else {
            "This page was NEVER handed out by the quarantine - the pointer is wild in our \
             reservation, not stale in a slot we served"
                .to_string()
        };
        log(&format!(
            "***** GUARD PAGE - STALE {} at {target:#x} (slot page {page:#x}, retired): the \
             instruction is RIP {rip:#x}{}, tid {}. {attribution}. THE WRITER holds a pointer \
             into memory it no longer owns; this is the instruction that uses it. Recommitting \
             the page so the client continues. The 32-bit increment family, caught at the \
             source *****",
            if is_write { "WRITE" } else { "READ" },
            crate::netwatch::module_of(rip),
            GetCurrentThreadId(),
        ));
    }
    if ready {
        meta(i).state.store(3, Ordering::SeqCst);
    }
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
    slot_index(
        RESERVE_BASE.load(Ordering::SeqCst),
        SLOTS.load(Ordering::SeqCst),
        target,
    )
    .is_some()
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
    VirtualFree(page as *mut c_void, 0, MEM_RELEASE);
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

/// Arm the quarantine if `guardpage=<classes>` is in the session marker.
pub fn install() {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    let Some(tok) = crate::session::marker_token("guardpage=") else {
        // **Say so.** This exact silence cost the overnight run of 2026-09-08: the flag never
        // reached the launcher, the marker carried no token, this returned without a word, and
        // the log was indistinguishable from a build with no guard page in it at all. One line
        // per run is the price of never spending a night that way again.
        log(&format!(
            "guard page: NOT ARMED - the session marker carries no `guardpage=` token, so \
             nothing was quarantined this run. The marker reads {:?}. If you passed -GuardPage \
             and are reading this, the flag did not reach the client",
            crate::session::marker_raw().unwrap_or_default()
        ));
        return;
    };
    let mask = match parse_classes(&tok) {
        Ok(m) if m != 0 => m,
        Ok(_) => {
            log("***** GUARD PAGE: guardpage= named no classes - standing down *****");
            return;
        }
        Err(why) => {
            log(&format!("***** GUARD PAGE: guardpage={tok:?} rejected - {why}. Standing down *****"));
            return;
        }
    };
    std::thread::spawn(move || unsafe { arm(mask) });
}

/// Release everything this module reserved. Used on every failure path after the reservation
/// exists, so a stand-down leaves no 32 GB hole behind.
unsafe fn release_all() {
    for cell in [&RESERVE_BASE, &RING, &META] {
        let p = cell.swap(0, Ordering::SeqCst);
        if p != 0 {
            VirtualFree(p as *mut c_void, 0, MEM_RELEASE);
        }
    }
    SLOTS.store(0, Ordering::SeqCst);
    META_COMMITTED.store(0, Ordering::SeqCst);
    WATCHED_MASK.store(0, Ordering::SeqCst);
}

unsafe fn arm(mask: u32) {
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

    // The reservation. Largest first; a smaller cursor is a worse run but a far better one
    // than standing down, and the log says which was taken so a short run can be read for what
    // it is rather than blamed on the writer.
    let mut reserve = 0usize;
    let mut slots = 0usize;
    for &want in RESERVE_LADDER.iter() {
        let p = VirtualAlloc(std::ptr::null_mut(), want * PAGE_BYTES, MEM_RESERVE, PAGE_NOACCESS) as usize;
        if p != 0 {
            reserve = p;
            slots = want;
            break;
        }
    }
    if reserve == 0 {
        log(&format!(
            "***** GUARD PAGE: could not reserve address space for even {} slots - NOT armed *****",
            RESERVE_LADDER[RESERVE_LADDER.len() - 1]
        ));
        return;
    }
    RESERVE_BASE.store(reserve, Ordering::SeqCst);
    SLOTS.store(slots, Ordering::SeqCst);
    if slots != MAX_SLOTS {
        log(&format!(
            "***** GUARD PAGE: only {slots} slots could be reserved, not {MAX_SLOTS}. The \
             cursor is {}x smaller than designed and may be spent before the retirement queue \
             turns over - watch FELL BACK *****",
            MAX_SLOTS / slots
        ));
    }

    // The retirement queue, committed in full (4 bytes a slot; see the note above the ring).
    let ring_bytes = slots * std::mem::size_of::<u32>();
    let ring_ptr = VirtualAlloc(std::ptr::null_mut(), ring_bytes, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE) as usize;
    if ring_ptr == 0 {
        release_all();
        log("***** GUARD PAGE: could not commit the retirement queue - NOT armed *****");
        return;
    }
    RING.store(ring_ptr, Ordering::SeqCst);
    // The metadata: RESERVED in full, COMMITTED a block at a time as the cursor advances. This
    // is the change that let the reserve grow at all - 40 bytes a slot committed up front is
    // 320 MB, and it was the array, not the address space, that pinned MAX_SLOTS at 1 M.
    let meta_bytes = slots * META_STRIDE;
    let meta_ptr = VirtualAlloc(std::ptr::null_mut(), meta_bytes, MEM_RESERVE, PAGE_READWRITE) as usize;
    if meta_ptr == 0 {
        release_all();
        log("***** GUARD PAGE: could not reserve the metadata array - NOT armed *****");
        return;
    }
    META.store(meta_ptr, Ordering::SeqCst);
    META_COMMITTED.store(0, Ordering::SeqCst);
    if !ensure_meta(0) {
        release_all();
        log("***** GUARD PAGE: the first metadata block would not commit - NOT armed *****");
        return;
    }
    WATCHED_MASK.store(mask, Ordering::SeqCst);

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
        release_all();
        log(&format!("***** GUARD PAGE: HeapFree slot {slot:#x} unreadable - NOT armed *****"));
        return;
    }
    let real = std::ptr::read_volatile(slot as *const usize);
    REAL_HEAPFREE.store(real, Ordering::SeqCst);
    HEAPFREE_SLOT.store(slot, Ordering::SeqCst);
    let shim: unsafe extern "system" fn(usize, u32, usize) -> i32 = heapfree_shim;
    std::ptr::write_volatile(slot as *mut usize, shim as usize);
    if std::ptr::read_volatile(slot as *const usize) != shim as usize {
        std::ptr::write_volatile(slot as *mut usize, real);
        release_all();
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
            log(&render_armed(mask, slots, reserve, ring_bytes));
        }
        None => {
            // Undo the swap; leave nothing behind.
            std::ptr::write_volatile(slot as *mut usize, real);
            release_all();
            log("***** GUARD PAGE: the allocator hook was refused (prologue mismatch, or a \
                 thread would not leave it - the identity: line above says which) - HeapFree \
                 swap reverted, NOT armed *****");
        }
    }
}

/// The `GUARD PAGE ARMED` line, pure so the test can render it and read it.
///
/// It is the one line the owner reads at launch to decide whether to leave the run overnight, and
/// it is long on purpose: it has to state the sizing model, the measurement the model came
/// from, and what would make it wrong, because a run that quietly falls back looks exactly
/// like a healthy one.
pub(crate) fn render_armed(mask: u32, slots: usize, reserve: usize, ring_bytes: usize) -> String {
    let (first, steady) = armed_need(mask);
    let need = first.max(steady);
    let nclass = mask.count_ones() as usize;
    // Headroom against the model, to one decimal, and a shout if the reserve is below it -
    // "1x" and "0x" both read as a number rather than as a warning.
    let tenths = slots.saturating_mul(10) / need.max(1);
    let headroom = if tenths >= 15 {
        format!("{}.{}x", tenths / 10, tenths % 10)
    } else {
        format!(
            "***** only {}.{}x, which is NOT enough headroom - expect FELL BACK *****",
            tenths / 10,
            tenths % 10
        )
    };
    format!(
        "***** GUARD PAGE ARMED: size class(es) {} are served one-slot-per-page from a \
         {} GB reserve ({slots} slots) at {reserve:#x} and DECOMMITTED on free, each \
         address held back for {}s - one full firing of the writer's 180 s clock plus \
         20 s - before it can be handed out again. SIZED FROM THE 16:46 RUN OF \
         2026-09-16, which spent an 8388608-slot reserve in 254 s: 0x20 burst to \
         {MEASURED_1646_BURST_60S_0X20} in its first minute then ran at \
         {MEASURED_1646_STEADY_PER_S_0X20}/s (22x the 12:01 run it was sized from), 0x40 \
         at {MEASURED_1646_STEADY_PER_S_0X40}/s, and the two classes share ONE cursor. \
         The {nclass} class(es) armed need {first} slots for their first retirement \
         window and {steady} once they are turning over; against the larger of those \
         this reserve has {headroom} of headroom. 0x10 and 0x80 are unmeasured and \
         modelled at 0x20's rate, so read the 'pool allocations seen by class' counters \
         in the heartbeat - and read FELL BACK first: a fallen-back run looks exactly \
         like a protected one. Metadata is committed lazily, so arming costs {} MB now \
         and grows with the cursor, which stops growing once recycling holds. \
         Committed PAGE memory is bounded by the LIVE set, not by the cursor: 0x20 held \
         ~{MEASURED_1646_LIVE_0X20} live slots all run, which is {} MB, so expect \
         roughly that per class. A stale write or read into a freed slot faults at the \
         instruction that makes it, which the handler logs (RIP, target, who allocated, \
         who freed, how long ago) and recommits so the client continues. control PASS. \
         The pool's own chain, free list and counters are untouched - our slots take \
         the HeapFree arm, and that arm is chosen by the header value ALONE, so one \
         stamp {QUARANTINE_HEADER:#x} covers every class at once *****",
        mask_text(mask),
        slots * PAGE_BYTES / (1024 * 1024 * 1024),
        REUSE_AFTER_MS / 1000,
        (ring_bytes + META_BLOCK_BYTES) / (1024 * 1024),
        MEASURED_1646_LIVE_0X20 * PAGE_BYTES / (1024 * 1024),
    )
}

static VEH_REGISTERED: AtomicBool = AtomicBool::new(false);
unsafe fn register_veh() -> bool {
    if VEH_REGISTERED.swap(true, Ordering::SeqCst) {
        return true;
    }
    !AddVectoredExceptionHandler(1, veh as *const c_void).is_null()
}

/// One watched class's counters, for the heartbeat.
pub(crate) struct ClassCounters {
    pub class: usize,
    pub watched: bool,
    pub seen: u64,
    pub served: u64,
    pub freed: u64,
    pub live: u64,
    pub recycled: u64,
    pub fallback: u64,
}

/// Per-class counters plus the shared ones: `(classes, catches, cursor, slots, alien_frees)`.
///
/// **Nothing is summed across classes.** The 12:01 run printed one `served` for one class and
/// that was enough; with several, a total hides which class is exhausting the shared cursor,
/// which is precisely the number the run exists to produce. `fallback` is still the one to read
/// on a long run: it counts allocations that went to the client's own pool because nothing had
/// aged out and the cursor was spent, and a rising fallback means the class is no longer
/// covered - the failure that looks exactly like a quiet, healthy run.
pub(crate) fn counters() -> ([ClassCounters; NCLASS], u64, usize, usize, u64) {
    let mask = WATCHED_MASK.load(Ordering::SeqCst);
    let one = |b: usize| {
        let served = SERVED[b].load(Ordering::SeqCst);
        let freed = FREED[b].load(Ordering::SeqCst);
        ClassCounters {
            class: CLASSES[b],
            watched: mask & (1 << b) != 0,
            seen: SEEN[b].load(Ordering::Relaxed),
            served,
            freed,
            live: served.saturating_sub(freed),
            recycled: RECYCLED[b].load(Ordering::SeqCst),
            fallback: FALLBACK[b].load(Ordering::SeqCst),
        }
    };
    let slots = SLOTS.load(Ordering::SeqCst);
    (
        [one(0), one(1), one(2), one(3)],
        CATCHES.load(Ordering::SeqCst),
        CURSOR.load(Ordering::SeqCst).min(slots.max(1)),
        slots,
        ALIEN_FREES.load(Ordering::SeqCst),
    )
}

pub(crate) fn armed() -> bool {
    MODE.load(Ordering::SeqCst) == MODE_ARMED
}

/// The guard page's half of the sentry heartbeat.
///
/// Split out of `poolsentry` and made pure so it can be **rendered and read** in a test rather
/// than only at 03:00 on the owner's console - `CLAUDE.md`'s rule for the launcher's own plan, and
/// the same reason applies to a line whose whole job is to be read in the morning.
pub(crate) fn heartbeat_line() -> String {
    let (classes, catches, cursor, slots, alien) = counters();
    render_heartbeat(&classes, catches, cursor, slots, alien)
}

/// Three readings, in the order they matter:
///
/// * **`FELL BACK`** - allocations the quarantine did not serve because nothing had aged out
///   and the shared cursor was spent. Above zero means that class is no longer covered from
///   that moment, and nothing else on screen would say so: an uncovered run looks exactly like
///   a quiet one.
/// * **`NEVER FREED`** - served, but nothing came back through [`heapfree_shim`]. For a class
///   that has never been quarantined before, this is the live control on whether its frees
///   reach the cached pointer we swapped; 53 of the client's 56 free sites are inlined and
///   untraced, so it is an inference until this says otherwise.
/// * **`pool allocations seen`** - every allocation of each class the detour saw, watched or
///   not. This is the churn measurement that sizes the next run. `0x20` is known to run at
///   1 560/s after a 627 K first-minute burst; the other three are not known at all, and a
///   number here costs no launch of its own.
///
/// Nothing is summed across classes: with several classes sharing one cursor, a total would
/// hide which one is exhausting it.
pub(crate) fn render_heartbeat(
    classes: &[ClassCounters],
    catches: u64,
    cursor: usize,
    slots: usize,
    alien: u64,
) -> String {
    let per: Vec<String> = classes
        .iter()
        .filter(|c| c.watched)
        .map(|c| {
            let fell_back = if c.fallback == 0 {
                "0 fell back".to_string()
            } else {
                format!("***** {} FELL BACK - NO LONGER COVERED *****", c.fallback)
            };
            let never_freed = if c.served > 1_000 && c.freed == 0 {
                " ***** NEVER FREED - this class's free path does NOT reach our shim, so its \
                 slots are leaking a page each *****"
            } else {
                ""
            };
            format!(
                "{:#x}: {} served, {} freed, {} live, {} recycled, {fell_back}{never_freed}",
                c.class, c.served, c.freed, c.live, c.recycled
            )
        })
        .collect();
    let seen: Vec<String> = classes
        .iter()
        .map(|c| format!("{:#x} {}", c.class, c.seen))
        .collect();
    let alien = if alien == 0 {
        String::new()
    } else {
        format!(
            " | ***** {alien} free(s) of an address inside the reserve that we never handed \
             out *****"
        )
    };
    format!(
        "guard page: {} | {cursor} of {slots} fresh pages used, {catches} STALE-ACCESS \
         CATCH(es){alien} | pool allocations seen by class: {}",
        per.join(" | "),
        seen.join(", ")
    )
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

    /// **The exact string the launcher ships to every player, parsed by the code that has to
    /// read it.**
    ///
    /// `launcher::client::DEFAULT_SESSION` is `"mode=2,create=on,guardpage=0x20+0x40"` as of
    /// 2026-09-08, and the launcher does not depend on this crate - the two binaries meet only
    /// through files in the client directory, so nothing but this test and its twin
    /// (`launcher::client::tests::the_shipped_session_arms_the_guard_page_on_the_two_damaged_classes`)
    /// connects the two spellings. A mismatch does not fail to compile; it produces a token
    /// this module rejects and a mitigation that is silently off on every player's machine,
    /// which is the failure mode `CLAUDE.md` calls indistinguishable from the code not
    /// existing.
    #[test]
    fn the_class_set_the_launcher_ships_parses() {
        const SHIPPED: &str = "0x20+0x40";
        let mask = parse_classes(SHIPPED).expect("the shipped class set must parse");
        assert_eq!(mask, (1 << 1) | (1 << 2), "0x20 and 0x40, and nothing else");
        assert_eq!(mask_text(mask), SHIPPED, "and it round-trips to the same words");
        assert!(mask.count_ones() == 2);

        // The headroom the shipped set gets, from the model rather than from prose: the pair
        // clears the 1.5x floor `render_armed` shouts below, all four do not - which is why
        // `all` is not what ships.
        let tenths = |m: u32| {
            let (first, steady) = armed_need(m);
            MAX_SLOTS * 10 / first.max(steady)
        };
        assert!(tenths(mask) >= 15, "the pair: {} tenths", tenths(mask));
        assert!(tenths(parse_classes("all").unwrap()) < 15, "all four would print the NOT-enough-headroom shout");

        // A comma is the one separator that must NOT work: the session marker is itself
        // comma-separated, so `guardpage=0x20,0x40` reaches here as `0x20` and arms half of
        // what was asked for, silently, with a log line that says the whole set.
        assert!(parse_classes("0x20,0x40").is_err());
    }

    /// **The reserve must be a working set, not a budget.** A slot is reusable only after
    /// `REUSE_AFTER_MS`, which covers one full period of the writer's 180 s clock with slack -
    /// three periods until 2026-09-16, when the 16:46 run spent the reserve before anything
    /// could age out. Shorter than one period would hand a page back before the writer's next
    /// tick, which is the one exposure the window exists to close.
    #[test]
    fn a_retired_slot_is_reusable_only_after_one_full_firing_of_the_writers_clock() {
        assert!(
            REUSE_AFTER_MS > 180_000,
            "the delay must cover one 180 s firing of any pointer taken at the free, with slack"
        );
        assert!(
            REUSE_AFTER_MS < 2 * 180_000,
            "and it was shortened on purpose - two firings is the old budget problem again"
        );
        let freed = 1_000_000u64;
        assert!(!reusable_at(freed, freed), "just freed");
        assert!(!reusable_at(freed, freed + REUSE_AFTER_MS - 1), "one tick short");
        assert!(reusable_at(freed, freed + REUSE_AFTER_MS), "exactly old enough");
        assert!(reusable_at(freed, freed + REUSE_AFTER_MS * 10));
        // A slot that has never been freed is live and must never be handed out again.
        assert!(!reusable_at(0, u64::MAX), "freed_tick 0 means live");
        // GetTickCount64 is monotonic, but a clock that went backwards must not make a live
        // slot look ancient.
        assert!(!reusable_at(freed, freed - 1_000));
    }

    /// **The 12:01 run's own numbers, and the model that has to reproduce them.**
    ///
    /// The run is the only measurement of pool churn this project has, and it settled which of
    /// two quantities the reserve has to hold. Everything here is arithmetic over `[L]`
    /// constants, so a future sizing change has to argue with the run rather than with prose.
    #[test]
    fn the_model_reproduces_the_measured_fallback_of_the_first_guard_page_run() {
        // What one class needs from a standing start: the burst plus the rest of the window.
        let first = first_window_slots(MEASURED_BURST_60S, MEASURED_STEADY_PER_S, REUSE_AFTER_MS_UNTIL_0916);
        assert_eq!(first, 1_469_572, "627172 + 1560*540");

        // The old cursor could not hold it, and the shortfall IS the measured fallback. The
        // run reported 419 588 fallen back at the 600 s heartbeat; the model says 421 K.
        let shortfall = first - OLD_MAX_SLOTS;
        let err = shortfall.abs_diff(MEASURED_FALLBACK_AT_600S);
        assert!(
            err * 100 / MEASURED_FALLBACK_AT_600S < 2,
            "model {shortfall} vs measured {MEASURED_FALLBACK_AT_600S}"
        );

        // **The burst is what broke it, not the steady state.** Steady-state churn alone would
        // have fitted in the old cursor with room to spare - which is why shortening
        // REUSE_AFTER_MS was the wrong lever and a bigger cursor was the right one.
        let steady = steady_slots(MEASURED_STEADY_PER_S, REUSE_AFTER_MS_UNTIL_0916);
        assert_eq!(steady, 936_000);
        assert!(steady < OLD_MAX_SLOTS, "the steady state always fitted: {steady}");

        // And the cursor really was spent between the 300 s and 360 s heartbeats.
        let spent_at_s = 60 + (OLD_MAX_SLOTS - MEASURED_BURST_60S) / MEASURED_STEADY_PER_S;
        assert!((300..360).contains(&spent_at_s), "model says {spent_at_s}s");
    }

    /// **The 16:46 run of 2026-09-16, and the model that has to reproduce it.** The pair was
    /// armed with 8 M and 600 s; both served counters stopped between the 240 s and 300 s
    /// heartbeats, summing to exactly the reserve. The model has to put the exhaustion there,
    /// and has to say that 600 s could never have recycled in time at that churn.
    #[test]
    fn the_model_reproduces_when_the_1646_run_spent_its_shared_reserve() {
        let burst = MEASURED_1646_BURST_60S_0X20 + MEASURED_1646_BURST_60S_0X40;
        let rate = MEASURED_1646_STEADY_PER_S_0X20 + MEASURED_1646_STEADY_PER_S_0X40;
        let spent_at_s = 60 + (MAX_SLOTS_UNTIL_0916 - burst) / rate;
        assert!((240..300).contains(&spent_at_s), "model says {spent_at_s}s; the heartbeats say 240..300");
        // At 600 s the pair's steady state alone was 2.4x the reserve: no cursor could have
        // been recycled into, because nothing is old enough before the window.
        let steady_600 = steady_slots(rate, REUSE_AFTER_MS_UNTIL_0916);
        assert!(steady_600 > 2 * MAX_SLOTS_UNTIL_0916, "{steady_600} vs {MAX_SLOTS_UNTIL_0916}");
        // 34 173/s is 22x the 12:01 run's 1 560/s - the number the old sizing rested on.
        assert_eq!(MEASURED_1646_STEADY_PER_S_0X20 / MEASURED_STEADY_PER_S, 21);
    }

    /// **The new reserve and window, sized against that run.**
    #[test]
    fn the_reserve_holds_the_shipped_pair_with_headroom_against_the_1646_run() {
        let pair = parse_classes("0x20+0x40").unwrap();
        let (first, steady) = armed_need(pair);
        assert_eq!(steady, (34_173 + 3_396) * 200, "{steady}");
        assert_eq!(first, 877_768 + 221_237 + (34_173 + 3_396) * 140, "{first}");
        // Better than 2x on the larger of the two, from the model rather than from prose.
        assert!(MAX_SLOTS * 10 / first.max(steady) >= 20, "{MAX_SLOTS} vs {first}/{steady}");
        // 8 M would have been a fit, not headroom - which is why the cursor doubled with the
        // window rather than instead of it.
        assert!(MAX_SLOTS_UNTIL_0916 * 10 / first.max(steady) < 15);
        // The ladder is powers of two, because the ring indexes with a mask.
        for w in RESERVE_LADDER {
            assert!(w.is_power_of_two(), "{w} is not a power of two");
        }
        assert_eq!(RESERVE_LADDER[0], MAX_SLOTS, "the ladder starts at the design size");
        for pair in RESERVE_LADDER.windows(2) {
            assert!(pair[0] > pair[1], "the ladder must descend: {pair:?}");
        }
    }

    /// **Committed memory is not the constraint, and the run said so.** The prediction before
    /// the launch was ~230 MB of committed pages for bucket 1; the measurement was ~26 000 live
    /// slots, so ~104 MB. Retired pages are decommitted and cost only address space.
    #[test]
    fn committed_page_memory_is_bounded_by_the_live_set_not_the_cursor() {
        let live_mb = MEASURED_LIVE_0X20 * PAGE_BYTES / (1024 * 1024);
        assert!((100..110).contains(&live_mb), "{live_mb} MB");
        // The cursor is 320x the live set at the design size, and costs nothing but address
        // space plus lazily-committed metadata.
        assert!(MAX_SLOTS > MEASURED_LIVE_0X20 * 300);
        // All four classes' live sets together, scaled from poolchain's 174 528 enumerated
        // slots at the same live fraction the 0x20 measurement showed: still under 300 MB.
        let all_four_live = 174_528 * MEASURED_LIVE_0X20 / 70_848;
        let all_four_mb = all_four_live * PAGE_BYTES / (1024 * 1024);
        assert!(all_four_mb < 300, "{all_four_mb} MB for all four classes");
    }

    /// **The metadata array is the thing that stopped the reserve growing, and it is now
    /// lazy.** Eagerly committed it would be 320 MB at the design size; a run that touches one
    /// class's first window commits 60 MB of it and nothing more.
    #[test]
    fn the_metadata_array_is_committed_only_as_far_as_the_cursor_reaches() {
        assert_eq!(META_STRIDE, 40, "four u64-ish fields and two u32");
        let eager_mb = MAX_SLOTS * META_STRIDE / (1024 * 1024);
        assert_eq!(eager_mb, 640, "what it would cost committed up front at 16 M");

        // The 12:01 run's one-class window at the old 600 s: 60 MB, as the doc block says.
        let first = first_window_slots(MEASURED_BURST_60S, MEASURED_STEADY_PER_S, REUSE_AFTER_MS_UNTIL_0916);
        let used = commit_target(first * META_STRIDE, META_BLOCK_BYTES, MAX_SLOTS * META_STRIDE);
        assert!(used / (1024 * 1024) < 60, "{} MB for one class", used / (1024 * 1024));
        // The 16:46 run's churn, the shipped pair, at 200 s: the cursor reaches the larger of
        // the pair's two windows and then holds there once recycling supplies every fresh
        // request, so this is what a night at that churn commits - under 300 MB, not 640.
        let (pair_first, pair_steady) = armed_need(parse_classes("0x20+0x40").unwrap());
        let used = commit_target(pair_first.max(pair_steady) * META_STRIDE, META_BLOCK_BYTES, MAX_SLOTS * META_STRIDE);
        assert!((280..300).contains(&(used / (1024 * 1024))), "{} MB for the pair at 16:46 churn", used / (1024 * 1024));

        // commit_target rounds up to a block, never past the reservation, and is monotone.
        assert_eq!(commit_target(1, 1024, 8192), 1024);
        assert_eq!(commit_target(1024, 1024, 8192), 1024);
        assert_eq!(commit_target(1025, 1024, 8192), 2048);
        assert_eq!(commit_target(9000, 1024, 8192), 8192, "never past the reservation");
        assert_eq!(commit_target(8192, 1024, 8192), 8192);
        // A block is big enough that the commit ladder is not itself a cost: at the measured
        // rate it fires about three times a minute.
        let slots_per_block = META_BLOCK_BYTES / META_STRIDE;
        assert!(slots_per_block > MEASURED_STEADY_PER_S * 10, "{slots_per_block} slots a block");
        // At the 16:46 run's 34 173/s it fires about once a second while the cursor is still
        // advancing - one VirtualAlloc a second, off the hot path - and not at all once
        // recycling holds the cursor still.
        assert!(slots_per_block * 2 > MEASURED_1646_STEADY_PER_S_0X20, "{slots_per_block} slots a block");

        // The ring is the other array, and it is eager on purpose: 4 bytes a slot.
        let ring_mb = MAX_SLOTS * std::mem::size_of::<u32>() / (1024 * 1024);
        assert_eq!(ring_mb, 64, "16 M slots x 4 bytes, committed at arm");
        assert_eq!(
            ring_mb * META_STRIDE / std::mem::size_of::<u32>(),
            eager_mb,
            "the ring is exactly a tenth of the metadata - 4 bytes a slot against 40, which is \
             why one is committed eagerly and the other is not"
        );
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

        // And the bucket index the counters are kept by.
        assert_eq!(size_bucket(0x11), Some(1));
        assert_eq!(size_bucket(0x41), Some(3));
        assert_eq!(size_bucket(0x81), None);
        assert_eq!(bucket_of(0x40), Some(2));
        assert_eq!(bucket_of(0x30), None, "0x30 is not a class, it is a size");
        assert_eq!(bucket_of(0), None);
    }

    /// **How several classes are named, and what is refused.**
    ///
    /// The separator is `+`, not `,`: the session marker is comma-separated, so a comma here
    /// would be split away by `session::marker_token` and arm half of what was asked for -
    /// silently, which is the failure mode this whole module keeps being rebuilt around.
    #[test]
    fn a_class_list_is_parsed_or_refused_by_name() {
        assert_eq!(parse_classes("0x20"), Ok(0b0010));
        assert_eq!(parse_classes("0x40"), Ok(0b0100));
        assert_eq!(parse_classes(" 0X20 "), Ok(0b0010), "case and space");
        assert_eq!(parse_classes("0x20+0x40"), Ok(0b0110), "the next run's pair");
        assert_eq!(parse_classes("0x40+0x20"), Ok(0b0110), "order does not matter");
        assert_eq!(parse_classes("0x20|0x40"), Ok(0b0110), "| is accepted too");
        assert_eq!(parse_classes("0x10+0x20+0x40+0x80"), Ok(0b1111));
        assert_eq!(parse_classes("all"), Ok(0b1111));
        // The single-class form the test plan already tells the owner to type still means what it
        // always meant, including the bare-decimal spelling the old parser accepted.
        assert_eq!(parse_classes("32"), Ok(0b0010), "decimal 32 is 0x20, as before");
        assert_eq!(parse_classes("64"), Ok(0b0100));
        // Refusals name the offending term rather than returning an empty mask.
        assert!(parse_classes("0x30").is_err());
        assert!(parse_classes("").is_err());
        assert!(parse_classes("0x20+").is_err(), "a trailing + is a typo, not a class");
        assert!(parse_classes("0x20,0x40").is_err(), "a comma cannot survive the marker");
        assert!(parse_classes("on").is_err());
        for e in [parse_classes("0x30"), parse_classes("0x20+")] {
            assert!(e.unwrap_err().len() > 10, "a refusal has to say what was wrong");
        }

        assert_eq!(mask_text(0b0110), "0x20+0x40");
        assert_eq!(mask_text(0b0010), "0x20");
        assert_eq!(mask_text(0b1111), "0x10+0x20+0x40+0x80");
        assert_eq!(mask_text(0), "(none)");
    }

    #[test]
    fn a_slot_index_is_only_inside_the_reserve() {
        let base = 0x1_0000_0000;
        let slots = 1024;
        assert_eq!(slot_index(0, slots, base), None, "no reserve yet");
        assert_eq!(slot_index(base, 0, base), None, "no slots yet");
        assert_eq!(slot_index(base, slots, base - 1), None);
        assert_eq!(slot_index(base, slots, base), Some(0));
        assert_eq!(slot_index(base, slots, base + 0x10), Some(0), "the body of slot 0");
        assert_eq!(slot_index(base, slots, base + PAGE_BYTES), Some(1));
        assert_eq!(slot_index(base, slots, base + PAGE_BYTES + 0x40), Some(1));
        assert_eq!(slot_index(base, slots, base + slots * PAGE_BYTES - 1), Some(slots - 1));
        assert_eq!(slot_index(base, slots, base + slots * PAGE_BYTES), None, "one past the end");
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
        assert_eq!(slot_index(base, 1024, mem), Some(5));
    }

    /// **One stamp has to divert every class, on every one of the client's free ladders.**
    ///
    /// `research/heapfix-did-not-hold.md` §1 lists three standalone frees. Two ladder
    /// `0x10/0x20/0x40/0x80` and one ladders `0x28/0x38/0x58/0x98`, and every one of them
    /// decides on the **header value alone** - never on the size the caller asked for. So the
    /// stamp is correct for a multi-class quarantine if and only if it is above the largest
    /// rung of both ladders, which is the actual check rather than "greater than 0x80".
    #[test]
    fn the_quarantine_header_takes_the_heapfree_arm_on_both_ladders() {
        const POOL_LADDER: [u64; 4] = [0x10, 0x20, 0x40, 0x80];
        const OTHER_LADDER: [u64; 4] = [0x28, 0x38, 0x58, 0x98]; // FUN_14019ba40
        for rung in POOL_LADDER.into_iter().chain(OTHER_LADDER) {
            assert!(
                QUARANTINE_HEADER > rung,
                "{QUARANTINE_HEADER:#x} must be above every rung, including {rung:#x}"
            );
        }
        // Every class we can serve is one of the four the pool ladder names, and the stamp is
        // the SAME value for all of them - that is what makes one reserve serve all four.
        for c in CLASSES {
            assert!(POOL_LADDER.contains(&(c as u64)));
            assert_ne!(QUARANTINE_HEADER, c as u64);
        }
        // The ladder does `test rax,rax; jns; not rax` first, so a stamp with the sign bit set
        // would be complemented into something small. Ours is positive.
        assert_eq!(QUARANTINE_HEADER >> 63, 0, "a negative header would be NOTed into a bucket");
    }

    /// **Render the ARMED line and read it**, for the two configurations that matter: the
    /// single class the test plan already tells the owner to type, and the pair the next run wants.
    #[test]
    fn the_armed_line_states_the_sizing_model_and_its_headroom() {
        let one = render_armed(parse_classes("0x20").unwrap(), MAX_SLOTS, 0x1_8006_0000, MAX_SLOTS * 4);
        eprintln!("{one}\n");
        assert!(one.contains("size class(es) 0x20 are served"), "{one}");
        assert!(one.contains("64 GB reserve (16777216 slots)"), "{one}");
        assert!(one.contains("held back for 200s - one full firing"), "{one}");
        assert!(one.contains("The 1 class(es) armed need 5661988 slots for their first"), "{one}");
        assert!(one.contains("6834600 once they are turning over"), "{one}");
        assert!(one.contains("this reserve has 2.4x of headroom"), "{one}");
        assert!(one.contains("arming costs 65 MB now"), "{one}");
        assert!(one.contains("~54000 live slots all run, which is 210 MB"), "{one}");
        assert!(one.contains("control PASS"), "the launcher's plan greps for this");
        assert!(one.contains("stamp 0x100 covers every class at once"), "{one}");

        let two = render_armed(parse_classes("0x20+0x40").unwrap(), MAX_SLOTS, 0x1_8006_0000, MAX_SLOTS * 4);
        assert!(two.contains("size class(es) 0x20+0x40 are served"), "{two}");
        assert!(two.contains("The 2 class(es) armed need 6358665 slots"), "{two}");
        assert!(two.contains("7513800 once they are turning over"), "{two}");
        assert!(two.contains("this reserve has 2.2x of headroom"), "{two}");

        // All four at 0x20's rate for the unmeasured two is a shortfall, and the line has to
        // SAY so rather than print "0.7x" as if it were a number like any other.
        let four = render_armed(parse_classes("all").unwrap(), MAX_SLOTS, 0x1000, MAX_SLOTS * 4);
        assert!(four.contains("***** only 0.7x, which is NOT enough headroom"), "{four}");

        // And the bottom rung of the ladder, which the run should never see but the line must
        // still describe honestly.
        let small = render_armed(parse_classes("0x20").unwrap(), 512 * 1024, 0x1000, 512 * 1024 * 4);
        assert!(small.contains("(524288 slots)"), "{small}");
        assert!(small.contains("***** only 0.0x, which is NOT enough headroom"), "{small}");
    }

    /// **Render the heartbeat and read it.** The same rule the launcher's test plan lives
    /// under: a line that is only ever seen at 03:00 on someone else's console has to be
    /// produced here, in full, and looked at.
    #[test]
    fn the_heartbeat_says_which_class_ran_out_and_which_never_freed() {
        let c = |class: usize, watched: bool, seen, served, freed, recycled, fallback| ClassCounters {
            class,
            watched,
            seen,
            served,
            freed,
            live: served - freed,
            recycled,
            fallback,
        };

        // The healthy shape: two classes, both turning over, nothing lost.
        let healthy = [
            c(0x10, false, 41_000, 0, 0, 0, 0),
            c(0x20, true, 1_628_991, 1_628_991, 1_603_226, 580_415, 0),
            c(0x40, true, 402_113, 402_113, 396_004, 141_002, 0),
            c(0x80, false, 9_004, 0, 0, 0, 0),
        ];
        let line = render_heartbeat(&healthy, 0, 1_469_572, MAX_SLOTS, 0);
        assert_eq!(
            line,
            "guard page: 0x20: 1628991 served, 1603226 freed, 25765 live, 580415 recycled, \
             0 fell back | 0x40: 402113 served, 396004 freed, 6109 live, 141002 recycled, \
             0 fell back | 1469572 of 16777216 fresh pages used, 0 STALE-ACCESS CATCH(es) | \
             pool allocations seen by class: 0x10 41000, 0x20 1628991, 0x40 402113, 0x80 9004"
        );
        // An unwatched class contributes its churn measurement and nothing else - that is the
        // number that sizes the next run, and it must not read as though it were quarantined.
        assert!(!line.contains("0x10: "), "an unwatched class has no served/freed section");
        assert!(line.contains("0x10 41000"), "but it is still counted");

        // The three failures, each of which is silent without a shout.
        let broken = [
            c(0x10, false, 41_000, 0, 0, 0, 0),
            c(0x20, true, 1_628_991, 1_048_576, 1_022_029, 0, 419_588),
            c(0x40, true, 402_113, 402_113, 0, 0, 0),
            c(0x80, false, 9_004, 0, 0, 0, 0),
        ];
        let line = render_heartbeat(&broken, 3, OLD_MAX_SLOTS, MAX_SLOTS, 7);
        assert!(line.contains("***** 419588 FELL BACK - NO LONGER COVERED *****"), "{line}");
        assert!(line.contains("0x40: 402113 served, 0 freed"), "{line}");
        assert!(line.contains("NEVER FREED - this class's free path does NOT reach our shim"), "{line}");
        assert!(line.contains("3 STALE-ACCESS CATCH(es)"), "{line}");
        assert!(line.contains("7 free(s) of an address inside the reserve"), "{line}");
        // The 12:01 run's exact numbers must render as a FELL BACK, since that is what it was.
        assert!(line.contains("1048576 of 16777216 fresh pages used"), "{line}");

        // A served-but-never-freed class is only shouted about once there is enough of it to
        // mean something: one allocation in the first millisecond of a run is not a leak.
        let early = [
            c(0x10, false, 0, 0, 0, 0, 0),
            c(0x20, true, 40, 40, 0, 0, 0),
            c(0x40, false, 0, 0, 0, 0, 0),
            c(0x80, false, 0, 0, 0, 0, 0),
        ];
        assert!(!render_heartbeat(&early, 0, 40, MAX_SLOTS, 0).contains("NEVER FREED"));
    }

    #[test]
    fn the_reserve_is_whole_pages_and_the_geometry_is_consistent() {
        assert_eq!(MAX_SLOTS * PAGE_BYTES % PAGE_BYTES, 0);
        assert!(MAX_SLOTS.is_power_of_two(), "the ring indexes with a mask");
        assert_eq!(SLOT_HEADER_OFF + 8, SLOT_BODY_OFF, "header sits at body-8");
        // The largest class we serve has to fit in one page beside its header.
        assert!(SLOT_BODY_OFF + CLASSES[NCLASS - 1] < PAGE_BYTES);
        // A slot index must survive the u32 the retirement ring stores it in.
        assert!(MAX_SLOTS <= u32::MAX as usize);
    }
}
