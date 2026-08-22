//! Three bytes that turn `0xC0000374` into a correct free — **opt-in, and an experiment.**
//!
//! Off unless the launcher writes `heapfix=on` into the session marker. Nothing on disk in
//! `client-patched/` changes; this writes to the mapped image at run time and dies with the
//! process, exactly like the create-character flag in [`crate::session`].
//!
//! # What is being fixed, and what is not
//!
//! Three crash dumps, three processes, and the client's own pool allocator enumerated in all
//! of them by `tools/poolchain.py`:
//!
//! ```text
//! dump 1   596 s   2 damaged slots
//! dump 2   644 s   3 damaged slots
//! dump 3   306 s   1 damaged slot
//! ```
//!
//! **Every damaged slot in all three reads `0x0000000100000020`** — a stray `1` in the high
//! dword of a 32-byte slot's size header — and all six are in the `0x20` size class, against
//! **0 of 360 216** slots enumerated in the `0x10`, `0x40` and `0x80` classes. `[L]`
//!
//! This patch does **not** stop the stray write. It stops the write from being fatal:
//!
//! ```text
//! 14019b500  48 8d 79 f8   lea rdi, [rcx - 8]
//! 14019b504  48 8b 07      mov rax, qword ptr [rdi]   <- reads all 64 bits of a field whose
//! 14019b507  48 85 c0      test rax, rax                 only legal values are 0x10/0x20/
//! 14019b50a  79 03         jns +3                        0x40/0x80
//! 14019b50c  48 f7 d0      not rax
//! 14019b50f  48 83 f8 20   cmp rax, 0x20
//! ```
//!
//! With the high dword non-zero the comparison chain falls past `0x80`, and the block is sent
//! to `HeapFree` instead of back to the pool's free list. `HeapFree` is handed a pointer
//! Windows never issued, and answers by killing the process. The replacement is the same
//! three bytes wide:
//!
//! ```text
//! 8b 07 90      mov eax, dword ptr [rdi]   ; zero-extends; the header's low half is intact
//! ```
//!
//! The damaged slot is then recognised as the `0x20` slot it genuinely still is and returned
//! to the right free list — the *correct* outcome, not a suppression.
//!
//! # The risk, stated
//!
//! It discards the `not rax` path at `0x14019b50c`, which handles a one's-complement-encoded
//! header. **0 of 481 000** enumerated slots across three dumps carry one, but the walk only
//! covers the four buckets of the global pool context at `0x143AD68A0`; a block from a
//! different context freed through the same function would not be in that count. `[I]`
//!
//! # Why it is worth switching on
//!
//! It is a **prediction that can come back false**, which is the only reason to ship it. If
//! the client stops dying with `0xC0000374` while a later dump still shows damaged slots
//! accumulating, the whole chain in `research/heap-wild-write.md` is confirmed end to end.
//! If it dies anyway, something in that chain is wrong and we learn which half.
//!
//! # It verifies before it writes
//!
//! The three bytes are read first and must be exactly `48 8b 07`; anything else and the patch
//! refuses and says what it found. A patch that writes blind into a Themida-packed image is
//! the kind of instrument `CLAUDE.md` spends its length warning about — and here the check is
//! free, because the site is a fixed three bytes with a known value.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::hook::log;

/// `0x14019b504` — the qword load of the pool slot's size header, inside the client's free.
///
/// Found from `FUN_14019b4e0`, whose `lea rbp,[rip+0x393b366]` at `0x14019b533` resolves the
/// pool context to `0x143AD68A0`. Both the RVA and the bytes are checked against the on-disk
/// image, so a different build cannot be patched by accident.
const FREE_HEADER_LOAD_RVA: usize = 0x14019B504 - 0x140000000;

/// `mov rax, qword ptr [rdi]` — what must be there.
const EXPECT: [u8; 3] = [0x48, 0x8B, 0x07];
/// `mov eax, dword ptr [rdi]` then `nop` — what goes there instead.
const REPLACE: [u8; 3] = [0x8B, 0x07, 0x90];

const PAGE_EXECUTE_READWRITE: u32 = 0x40;

extern "system" {
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

static APPLIED: AtomicBool = AtomicBool::new(false);

/// Apply the patch if `heapfix=on` is in the marker. Idempotent; logs once either way.
///
/// Called from the hook's install path, which runs long after Themida has unpacked `.text` —
/// the same window the dispatcher detour is written in, and that one has never been
/// checksummed.
pub unsafe fn install() {
    if APPLIED.swap(true, Ordering::SeqCst) {
        return;
    }
    if crate::session::marker_token("heapfix=").as_deref() != Some("on") {
        return;
    }

    let at = crate::hook::base() + FREE_HEADER_LOAD_RVA;
    if !crate::session::can_read(at, EXPECT.len()) {
        log(&format!(
            "***** HEAPFIX: {at:#x} is not readable - NOT patched. The client's free is where \
             0xC0000374 comes from; without this it behaves as it always has *****"
        ));
        return;
    }

    // Read before write. The three bytes are a fixed, known sequence, so this costs nothing
    // and turns "patched the wrong build" from a silent corruption into a log line.
    let found = std::slice::from_raw_parts(at as *const u8, EXPECT.len()).to_vec();
    if found != EXPECT {
        log(&format!(
            "***** HEAPFIX: refusing to patch {at:#x} - expected {EXPECT:02x?} (mov rax,[rdi]) \
             and found {found:02x?}. Either the RVA is wrong for this build or something else \
             is already there. NOT patched *****"
        ));
        return;
    }

    let mut old = 0u32;
    if VirtualProtect(at as *mut c_void, REPLACE.len(), PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log("***** HEAPFIX: VirtualProtect failed - NOT patched *****");
        return;
    }
    std::ptr::copy_nonoverlapping(REPLACE.as_ptr(), at as *mut u8, REPLACE.len());
    VirtualProtect(at as *mut c_void, REPLACE.len(), old, &mut old);

    // Read it back. A write that silently did not take looks exactly like a patch that did
    // not help, and those two would be indistinguishable in the one place it matters - a run
    // that crashes anyway.
    let after = std::slice::from_raw_parts(at as *const u8, REPLACE.len()).to_vec();
    if after == REPLACE {
        log(&format!(
            "***** HEAPFIX: {at:#x} is now {after:02x?} (mov eax,[rdi]; nop). A damaged pool \
             header goes back to the 0x20 free list instead of to HeapFree. THIS IS A CLIENT \
             PATCH and it is an EXPERIMENT: if the client still dies with 0xC0000374, the \
             chain in research/heap-wild-write.md is wrong somewhere *****"
        ));
    } else {
        log(&format!(
            "***** HEAPFIX: the write did not take - {at:#x} reads {after:02x?}. Treat this \
             run as UNPATCHED *****"
        ));
    }
}
