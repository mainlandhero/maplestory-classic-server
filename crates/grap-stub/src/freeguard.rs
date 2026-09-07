//! Refuse the one free that killed the client on a map change: a **pool chunk** handed to the
//! NT heap.
//!
//! # The crash this is for
//!
//! `research/second-crash-family-2026-09-07.md`. Process 288744 died `0xC0000374` 1.3 s after
//! a portal transfer. From the dump's own heap-failure record, innermost first:
//!
//! ```text
//! #2  ntdll!RtlFreeHeap+0x51
//! #3  PCOM.dll+0xe113          <- frees the bad pointer
//! ...
//! #11 oleaut32!VariantClear+0x193
//! ...
//! #17 oleaut32!VariantClear+0x193
//! #18 ResMan.dll+0xe637
//! #20 MapleStory.exe fn 0x142097f80  (the SetField handler, opcode 0x01A0)
//! #21 MapleStory.exe fn 0x141820080  (CField::OnPacket)
//! ```
//!
//! Type 8, *block not busy*, address `0x6b4c2e0` — **interior to a live 16 KB block**. Read
//! out of the dump, that address is `[0x508][next][0x0000000100000020]`: the size qword of a
//! **bucket-1 pool chunk**, its list link, and its first slot header carrying the family's
//! damage. `[L]`
//!
//! So the WZ property teardown followed a stale pointer into the client's own pool and asked
//! Windows to free memory Windows never issued.
//!
//! # What this module does, and why refusing is *correct* rather than a hack
//!
//! It interposes on PCOM's free. If the pointer is a pool chunk it does not pass it on.
//!
//! That is not a workaround for a bad pointer, it is the right answer to one: **a pool chunk
//! is not PCOM's to free.** The pool still owns it, still has it on its own structures, and
//! will free it itself. Passing it to `RtlFreeHeap` can only ever corrupt the process heap or
//! kill it. Refusing leaks nothing PCOM allocated, because PCOM never allocated it.
//!
//! # Where it hooks, and the reason that took a check rather than an assumption
//!
//! The obvious interception is an IAT hook on PCOM's `HeapFree` import. **It would not have
//! worked**, and nothing in the log would have said so. The call site is:
//!
//! ```text
//! PCOM+0xe0fb  48 8b 1d 7e da 0c 00   mov  rbx, [rip+0xcda7e]  ; -> PCOM+0xdbb80
//! PCOM+0xe102  ff 15 80 da 0c 00      call [rip+0xcda80]       ; -> PCOM+0xdbb88
//! PCOM+0xe108  4c 8d 47 f8            lea  r8, [rdi-8]         ; the pointer, at data-8
//! PCOM+0xe10c  33 d2                  xor  edx, edx
//! PCOM+0xe10e  48 8b c8               mov  rcx, rax            ; the heap handle
//! PCOM+0xe111  ff d3                  call rbx                 ; <- the fatal free
//! ```
//!
//! `call rbx`, out of a **cached function pointer in `.data`** at `PCOM+0xdbb80` — not the
//! import thunk at `PCOM+0xae4d0`. Patching the IAT would have installed cleanly, logged
//! success, and intercepted nothing. `[L]` for the bytes, decoded out of the shipped DLL.
//!
//! **`call rbx` ends at `PCOM+0xe113`, which is the return address the dump recorded**, so the
//! slot this module replaces is provably the pointer that performed the fatal free rather than
//! a plausible-looking neighbour. `[L]`
//!
//! `lea r8, [rdi-8]` is the other half worth reading: PCOM's data pointers sit **8 bytes past
//! their block**, so `rdi` held `0x6b4c2e8` — the pool chunk's base — and PCOM freed the
//! chunk's size qword as if it were its own block header. `[D]` That 8-byte prefix convention
//! is also the one `research/heap-corruption-2026-09-06.md` §5.3 needs to explain the damage
//! offset: a `body+4` string pointer used by code that assumes a cookie at `p-8` writes into
//! `body-4`, which is the pool header's high dword. Same convention, two different stale
//! pointers. `[I]`, and `crate::writewatch` is what can settle it.
//!
//! One qword is therefore the whole hook, and it covers **all seven** `mov reg,[PCOM+0xdbb80]`
//! sites in `.text`, not just this one. `[L]` No code is patched, nothing is relocated, and
//! there is no short branch to move — which is what makes this safer than an inline hook on
//! `PCOM+0xe08a`, whose first thirteen bytes end in a `jns +3` that relocation would break.
//!
//! `0xdbb80` is past `.data`'s `SizeOfRawData`, so it is zero-filled at load and written at
//! runtime. `[L]` The guard therefore **waits for it to be non-zero and checks what it holds**
//! against `kernel32!HeapFree` and `ntdll!RtlFreeHeap` before touching it, and stands down
//! with a log line if it is neither — the same read-before-write rule `heapfix.rs` follows.
//!
//! # What it is not
//!
//! * **Not a fix.** The stale pointer is still stale and the 180-second writer is still
//!   running. This removes one lethal *consequence*. `crate::writewatch` is the instrument
//!   aimed at the cause.
//! * **Not a substitute for the pool repair**, and not the same surface: the repair keeps the
//!   client's own pooled free alive, this keeps the NT heap's free alive.
//! * **Not on by default**, and it must be **off** during a `writewatch` measurement run —
//!   it is another patch to the client, and the open half of *"is any of this ours"* is not
//!   helped by adding one while measuring.
//!
//! # Modes
//!
//! `freeguard=observe` in the session marker counts and logs but passes everything through, so
//! the false-positive rate can be measured without changing client behaviour.
//! `freeguard=on` refuses. Absent, this module reads nothing and starts no thread.
//!
//! # The liveness control
//!
//! A guard that is never called reports zero refusals, which reads exactly like a guard that
//! is working. So the installer thread keeps a **pass-through count** and prints it. Zero
//! passes means the shim is not on the path, and that is a different result from "the bad free
//! did not happen" — the distinction this repo has paid for more than once.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

use crate::hook::log;
use crate::poolsentry::BUCKETS;

/// `PCOM+0xdbb80` — the cached free routine, in BSS, referenced by seven `mov` sites.
const PCOM_FREE_SLOT_RVA: usize = 0xdbb80;

/// How long to wait for PCOM to load and fill its cache before giving up.
const WAIT_TIMEOUT: Duration = Duration::from_secs(120);
const WAIT_STEP: Duration = Duration::from_millis(250);
/// How often the liveness line is printed while the guard is installed.
const REPORT_EVERY: Duration = Duration::from_secs(120);
/// Refusals logged in full, so a shape test that misfires cannot fill the disk.
const MAX_REFUSAL_LOGS: u32 = 16;

const MODE_OFF: u32 = 0;
const MODE_OBSERVE: u32 = 1;
const MODE_REFUSE: u32 = 2;

static INSTALLED: AtomicBool = AtomicBool::new(false);
static MODE: AtomicU32 = AtomicU32::new(MODE_OFF);
/// The routine the slot held before we replaced it.
static REAL: AtomicUsize = AtomicUsize::new(0);
static PASSED: AtomicU64 = AtomicU64::new(0);
static MATCHED: AtomicU64 = AtomicU64::new(0);
static REFUSED: AtomicU64 = AtomicU64::new(0);
static REFUSAL_LOGS: AtomicU32 = AtomicU32::new(0);

extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
}

// ---------------------------------------------------------------------------------------
// The test - pure, so the dump's own bytes can be asserted against it
// ---------------------------------------------------------------------------------------

/// Does this look like the head of a pool chunk rather than a heap block PCOM owns?
///
/// The three words are read at `mem+0`, `mem+8` and `mem+0x10`. A chunk laid out by the
/// client's pool reads `[chunk_bytes][chunk link][slot 0's header]`
/// (`poolsentry::BucketWalk::scan`: the size qword sits at `base − 8`, the link at `base`, and
/// slot `k`'s header at `base + 8 + k*stride`), so a pointer to `base − 8` sees exactly that.
///
/// Two independent identities must agree on the **same bucket** before anything is refused:
/// the chunk size, and the slot size in the low dword of the first header. The high dword of
/// that header is ignored on purpose - it is `1` in precisely the case this exists for.
///
/// Returns the bucket index.
pub(crate) fn pool_chunk_bucket(size_word: u64, link: u64, first_header: u64) -> Option<usize> {
    // The link is either the end of the chunk list or a pointer. Cheap, and it is what makes
    // a coincidental pair of size words unlikely to survive.
    let plausible_link =
        link == 0 || (link >= 0x1_0000 && link < 0x7FFF_FFFF_FFFF && link % 8 == 0);
    if !plausible_link {
        return None;
    }
    for (i, b) in BUCKETS.iter().enumerate() {
        if size_word == b.chunk_bytes() as u64 && first_header & 0xFFFF_FFFF == b.slot as u64 {
            return Some(i);
        }
    }
    None
}

// ---------------------------------------------------------------------------------------
// The shim
// ---------------------------------------------------------------------------------------

/// Stand in for `HeapFree`/`RtlFreeHeap` on PCOM's cached slot.
///
/// The return value is carried through as a full register rather than a `BOOL`, because the
/// slot may hold either `kernel32!HeapFree` (returns `BOOL`) or `ntdll!RtlFreeHeap` (returns
/// `BOOLEAN`, one byte, with the rest of `RAX` undefined). Passing the raw value back keeps
/// `AL` exactly as the real routine left it for either.
unsafe extern "system" fn shim(heap: *mut c_void, flags: u32, mem: *mut c_void) -> u64 {
    let real = REAL.load(Ordering::SeqCst);
    if real == 0 {
        return 0;
    }
    let real: extern "system" fn(*mut c_void, u32, *mut c_void) -> u64 = std::mem::transmute(real);

    // `HeapFree(h, 0, NULL)` is legal and succeeds. Nothing to inspect.
    if mem.is_null() {
        PASSED.fetch_add(1, Ordering::Relaxed);
        return real(heap, flags, mem);
    }

    // Three qwords of the user data, read without a `VirtualQuery`.
    //
    // Deliberate, and the reasoning is that the real routine is about to read `mem − 0x10`
    // and walk the block: a pointer whose first bytes cannot be read is a pointer that was
    // going to take the process down one instruction later. Reading it here moves the fault
    // from inside `ntdll` to inside this shim, where `probe.rs`'s handler names it and writes
    // a dump. A `VirtualQuery` per free would be a syscall on one of the hottest paths in the
    // client, for a check the allocator performs anyway.
    let p = mem as *const u64;
    let bucket = pool_chunk_bucket(
        std::ptr::read_volatile(p),
        std::ptr::read_volatile(p.add(1)),
        std::ptr::read_volatile(p.add(2)),
    );

    let Some(bucket) = bucket else {
        PASSED.fetch_add(1, Ordering::Relaxed);
        return real(heap, flags, mem);
    };

    MATCHED.fetch_add(1, Ordering::SeqCst);
    let refusing = MODE.load(Ordering::SeqCst) == MODE_REFUSE;
    if refusing {
        REFUSED.fetch_add(1, Ordering::SeqCst);
    }

    // Logging opens a file and allocates - out of GRAP64's heap, not PCOM's, so it cannot
    // re-enter the allocator whose lock this thread may be about to take. It is still slow,
    // so it is capped: this event is expected at most once or twice a session, and if that
    // turns out to be wrong the cap is what stops the cure being worse than the disease.
    if REFUSAL_LOGS.fetch_add(1, Ordering::SeqCst) < MAX_REFUSAL_LOGS {
        log(&format!(
            "***** FREE GUARD: PCOM asked the NT heap to free {mem:?}, which is the head of a \
             bucket-{bucket} POOL CHUNK ([{:#x}][{:#x}][{:#x}] = chunk size, list link, slot 0's \
             header). The pool owns this memory; Windows never issued it, and RtlFreeHeap would \
             raise 0xC0000374 - which is exactly how process 288744 died on a map change. {} \
             This does NOT fix the stale pointer that produced it *****",
            std::ptr::read_volatile(p),
            std::ptr::read_volatile(p.add(1)),
            std::ptr::read_volatile(p.add(2)),
            if refusing {
                "REFUSED - the free did not happen and TRUE was returned."
            } else {
                "OBSERVE MODE: passed through anyway, so this run behaves exactly as an \
                 unguarded one. Expect the client to die here."
            },
        ));
    }

    if refusing {
        // TRUE. PCOM's callers only ever test this for success, and a failure return would
        // send them down an error path that has never run in this client.
        return 1;
    }
    real(heap, flags, mem)
}

// ---------------------------------------------------------------------------------------
// Installation
// ---------------------------------------------------------------------------------------

/// Arm the guard if `freeguard=on` or `freeguard=observe` is in the session marker.
///
/// Returns immediately; PCOM may not be loaded yet and its cache is filled at runtime, so the
/// wait happens on a thread of its own.
pub fn install() {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    let mode = match crate::session::marker_token("freeguard=").as_deref() {
        Some("on") => MODE_REFUSE,
        Some("observe") => MODE_OBSERVE,
        Some(other) => {
            log(&format!(
                "***** FREE GUARD: the session marker says freeguard={other:?}, which is neither \
                 `on` nor `observe` - standing down. Nothing is hooked *****"
            ));
            return;
        }
        None => return,
    };
    MODE.store(mode, Ordering::SeqCst);
    std::thread::spawn(move || unsafe { wait_and_patch(mode) });
}

unsafe fn wait_and_patch(mode: u32) {
    let waited = match wait_for_slot() {
        Ok(v) => v,
        Err(why) => {
            log(&format!("***** FREE GUARD: {why} - NOT hooked *****"));
            MODE.store(MODE_OFF, Ordering::SeqCst);
            return;
        }
    };
    let (slot, held) = waited;

    // Read before write, exactly as `heapfix.rs` does. A cached pointer that is not one of
    // the two routines it should be is either an encoded pointer or a different build, and
    // in both cases the honest move is to say so rather than patch something unknown.
    let known = free_routines();
    let name = known.iter().find(|(_, va)| *va == held).map(|(n, _)| *n);
    let Some(name) = name else {
        log(&format!(
            "***** FREE GUARD: {slot:#x} holds {held:#x}, which is neither kernel32!HeapFree \
             ({:#x}) nor ntdll!RtlFreeHeap ({:#x}). Either this is a different PCOM build or \
             the pointer is encoded. NOT hooked *****",
            known.first().map(|k| k.1).unwrap_or(0),
            known.get(1).map(|k| k.1).unwrap_or(0),
        ));
        MODE.store(MODE_OFF, Ordering::SeqCst);
        return;
    };

    REAL.store(held, Ordering::SeqCst);
    // Through a typed fn pointer, not a direct cast of the fn item: the item's type is
    // zero-sized and casting it straight to an integer is the lint's way of pointing out that
    // the address you get is not obviously the one you meant.
    let shim_ptr: unsafe extern "system" fn(*mut c_void, u32, *mut c_void) -> u64 = shim;
    let shim_va = shim_ptr as usize;
    // An aligned 8-byte store is atomic on x86-64, so a thread calling through this slot sees
    // either the old routine or the new one and never a torn pointer. `.data` is already
    // writable; no VirtualProtect is needed and none is done.
    std::ptr::write_volatile(slot as *mut u64, shim_va as u64);
    let after = std::ptr::read_volatile(slot as *const u64) as usize;
    if after != shim_va {
        log(&format!(
            "***** FREE GUARD: the store did not take - {slot:#x} reads {after:#x}. Treat this \
             run as UNGUARDED *****"
        ));
        MODE.store(MODE_OFF, Ordering::SeqCst);
        return;
    }

    log(&format!(
        "***** FREE GUARD ARMED ({}): PCOM's cached free at {slot:#x} was {name} ({held:#x}) and \
         now comes here first. All seven `mov reg,[PCOM+{PCOM_FREE_SLOT_RVA:#x}]` sites in \
         PCOM's .text go through it - the IAT at PCOM+0xae4d0 is NOT the call site, the cached \
         pointer is. A pointer whose first three qwords read as a pool chunk is {}. Everything \
         else is passed to the real routine untouched. This does not stop the writer and it is \
         not the pool repair: it is the OTHER free, the one that killed 288744 on a map change \
         *****",
        if mode == MODE_REFUSE { "REFUSE" } else { "OBSERVE" },
        if mode == MODE_REFUSE {
            "not freed"
        } else {
            "logged and freed anyway"
        },
    ));

    // The liveness control. "Zero refusals" and "never called" are the same log line without
    // this, and telling them apart is the whole difference between a working guard and a
    // silent one.
    loop {
        std::thread::sleep(REPORT_EVERY);
        if MODE.load(Ordering::SeqCst) == MODE_OFF {
            return;
        }
        let passed = PASSED.load(Ordering::Relaxed);
        log(&format!(
            "FREE GUARD alive: {passed} free(s) passed through, {} matched the pool-chunk shape, \
             {} refused.{}",
            MATCHED.load(Ordering::SeqCst),
            REFUSED.load(Ordering::SeqCst),
            if passed == 0 {
                " ZERO passes means this shim is NOT on PCOM's free path, so no refusal count \
                 from this run means anything."
            } else {
                ""
            }
        ));
    }
}

/// Wait for PCOM to load and for its cached free pointer to be filled in.
unsafe fn wait_for_slot() -> Result<(usize, usize), String> {
    let mut waited = Duration::ZERO;
    let mut base = 0usize;
    while waited < WAIT_TIMEOUT {
        if base == 0 {
            base = GetModuleHandleA(b"PCOM.dll\0".as_ptr()) as usize;
        }
        if base != 0 {
            let slot = base + PCOM_FREE_SLOT_RVA;
            if crate::session::can_read(slot, 8) {
                let held = std::ptr::read_volatile(slot as *const u64) as usize;
                if held != 0 {
                    return Ok((slot, held));
                }
            }
        }
        std::thread::sleep(WAIT_STEP);
        waited += WAIT_STEP;
    }
    if base == 0 {
        Err(format!(
            "PCOM.dll was not loaded within {}s",
            WAIT_TIMEOUT.as_secs()
        ))
    } else {
        Err(format!(
            "PCOM.dll is at {base:#x} but its cached free at +{PCOM_FREE_SLOT_RVA:#x} was still \
             zero after {}s, so nothing has called PCOM's free yet",
            WAIT_TIMEOUT.as_secs()
        ))
    }
}

/// The two routines the cached slot may legitimately hold.
unsafe fn free_routines() -> Vec<(&'static str, usize)> {
    let mut out = Vec::new();
    for (module, name, label) in [
        (
            b"kernel32.dll\0".as_ref(),
            b"HeapFree\0".as_ref(),
            "kernel32!HeapFree",
        ),
        (
            b"ntdll.dll\0".as_ref(),
            b"RtlFreeHeap\0".as_ref(),
            "ntdll!RtlFreeHeap",
        ),
    ] {
        let m = GetModuleHandleA(module.as_ptr());
        if !m.is_null() {
            let va = GetProcAddress(m, name.as_ptr()) as usize;
            if va != 0 {
                out.push((label, va));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The exact three qwords the dump holds at the address that killed process 288744.**
    ///
    /// `research/second-crash-family-2026-09-07.md` §1: the free target `0x6b4c2e0` reads
    /// `[size 0x508][next 0x3360dc68][slot header 0x0000000100000020]`. If this test ever
    /// stops passing, the guard would not have caught the crash it was built for.
    #[test]
    fn it_recognises_the_pointer_that_killed_288744() {
        assert_eq!(
            pool_chunk_bucket(0x508, 0x3360dc68, 0x0000_0001_0000_0020),
            Some(1)
        );
        // And the same chunk before the writer got to it - an undamaged header must match too,
        // because a chunk is not PCOM's to free whether or not it is damaged.
        assert_eq!(pool_chunk_bucket(0x508, 0x3360dc68, 0x20), Some(1));
        // End of the chunk list.
        assert_eq!(pool_chunk_bucket(0x508, 0, 0x20), Some(1));
    }

    #[test]
    fn every_bucket_is_recognised_and_only_with_its_own_slot_size() {
        for (i, b) in BUCKETS.iter().enumerate() {
            assert_eq!(
                pool_chunk_bucket(b.chunk_bytes() as u64, 0x3360dc68, b.slot as u64),
                Some(i),
                "bucket {i}"
            );
            // A chunk size from one bucket with a slot size from another is not a chunk. The
            // two identities have to agree, which is what makes a coincidence unlikely.
            for (j, other) in BUCKETS.iter().enumerate() {
                if i != j {
                    assert_eq!(
                        pool_chunk_bucket(b.chunk_bytes() as u64, 0, other.slot as u64),
                        None,
                        "bucket {i} size with bucket {j} slot"
                    );
                }
            }
        }
    }

    #[test]
    fn ordinary_heap_data_is_passed_through() {
        // A C++ object: vtable pointer, then two members.
        assert_eq!(
            pool_chunk_bucket(0x0000_0001_4342_10d8, 0x0000_0001_4342_2e28, 1),
            None
        );
        // A BSTR: length prefix and UTF-16 text.
        assert_eq!(pool_chunk_bucket(0x000c, 0x0069_0070_006d_0069, 0), None);
        // All zeroes - freshly zeroed memory must not read as bucket anything.
        assert_eq!(pool_chunk_bucket(0, 0, 0), None);
        // The right size word but a link that cannot be a pointer or a terminator.
        assert_eq!(pool_chunk_bucket(0x508, 0xdead_beef_dead_beef, 0x20), None);
        assert_eq!(pool_chunk_bucket(0x508, 3, 0x20), None, "unaligned link");
        assert_eq!(pool_chunk_bucket(0x508, 0x40, 0x20), None, "link too low");
    }

    /// The four chunk sizes are derived, not typed in, so this pins the arithmetic that the
    /// shape test depends on against the sizes `tools/poolchain.py` prints.
    #[test]
    fn the_chunk_sizes_are_the_ones_the_pool_walker_reports() {
        let sizes: Vec<usize> = BUCKETS.iter().map(|b| b.chunk_bytes()).collect();
        assert_eq!(sizes, vec![0x608, 0x508, 0x488, 0x448]);
        // And they are all distinct, or the bucket a match reports would be arbitrary.
        let mut sorted = sizes.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 4);
    }
}
