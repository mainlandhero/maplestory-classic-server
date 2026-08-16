//! Walk the whole inbound opcode space from inside the client, in one launch.
//!
//! # Why
//!
//! The client blocks in `FUN_1415e7090` until a dispatched packet sets `conn+0x150`, and
//! only `FUN_1415e5c20` - the `Data.wz` patch handler - does that. Which opcode reaches
//! it cannot be read statically: the address appears nowhere as data, in the image or in
//! a gigabyte of live memory, so the mapping exists only inside the Themida VM.
//!
//! Finding it over the network costs about two opcodes per launch, because live handlers
//! take the client down. But the dispatcher is an ordinary function pointer once hooked,
//! and the packet it takes is a buffer we already own. So instead of sending thousands of
//! packets we rewrite the opcode in one captured packet and call the dispatcher directly,
//! checking `conn+0x150` after each call. No sockets, no pacing, no packet budget.
//!
//! # Surviving the walk
//!
//! Calling arbitrary handlers with a synthetic body goes wrong in three ways, all handled:
//!
//! * **The decoders throw.** `FUN_1406e8380` and `FUN_1406e82f0` raise a C++ exception on
//!   underflow, which is a Windows SEH exception. A vectored handler catches it and longjmps
//!   back into the loop via `RtlCaptureContext`/`RtlRestoreContext`, so one bad opcode costs
//!   one iteration rather than the run.
//! * **Access violations**, same path.
//! * **A handler tries to end the process.** `ExitProcess` and `TerminateProcess` are
//!   detoured to return without doing anything while the probe is running.
//!
//! A hit is self-announcing: setting `conn+0x150` is exactly what releases the client, so
//! the run that finds the opcode is also the run where the login screen finally appears.
//!
//! # Scope
//!
//! Opt-in via [`PROBE_MARKER`], for `client-patched/` only - a firewalled, GameGuard-stubbed
//! copy pointed at our own loopback server.

use std::ffi::{c_void, CStr};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use crate::hook::{log, CONN_DONE_FLAG};

/// Presence of this file enables the walk. Its contents may be `from-to` in hex
/// (e.g. `0000-0fff`); empty means the default range.
pub const PROBE_MARKER: &str = "maplecw-hook.probe";

const DEFAULT_FROM: u32 = 0x0000;
const DEFAULT_TO: u32 = 0x1000;

/// Offsets inside the packet view, confirmed against `FUN_1406e8c20`/`FUN_1406e9170`.
const VIEW_DATA: usize = 0x10; // -> buffer, which begins with the 4-byte frame header
const VIEW_LEN: usize = 0x18; // total bytes in that buffer
const VIEW_CURSOR: usize = 0x24; // read position, which handlers advance
const VIEW_STRUCT: usize = 0x30; // bytes of the view worth snapshotting
const OPCODE_AT: usize = 4; // the opcode sits just past the header

static RUNNING: AtomicBool = AtomicBool::new(false);
static DONE: AtomicBool = AtomicBool::new(false);
static IN_CALL: AtomicBool = AtomicBool::new(false);
static FAULTED: AtomicBool = AtomicBool::new(false);
static CURRENT: AtomicU32 = AtomicU32::new(0);
static FAULTS: AtomicU32 = AtomicU32::new(0);
static RESTORE_CTX: AtomicU64 = AtomicU64::new(0);

/// x64 `CONTEXT` is 1232 bytes and must be 16-byte aligned.
#[repr(C, align(16))]
struct Context([u8; 1232]);

static mut SAVED: Context = Context([0; 1232]);

extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: *const c_void) -> *mut c_void;
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn RtlCaptureContext(ctx: *mut c_void);
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const EXCEPTION_CONTINUE_SEARCH: i32 = 0;

#[repr(C)]
struct ExceptionPointers {
    record: *mut ExceptionRecord,
    context: *mut c_void,
}

#[repr(C)]
struct ExceptionRecord {
    code: u32,
    flags: u32,
    next: *mut ExceptionRecord,
    address: *mut c_void,
}

/// Read `from-to` out of the marker file, defaulting when it is empty or malformed.
fn range() -> (u32, u32) {
    let text = std::fs::read_to_string(PROBE_MARKER).unwrap_or_default();
    let text = text.trim();
    if let Some((a, b)) = text.split_once('-') {
        if let (Ok(a), Ok(b)) = (
            u32::from_str_radix(a.trim().trim_start_matches("0x"), 16),
            u32::from_str_radix(b.trim().trim_start_matches("0x"), 16),
        ) {
            return (a, b);
        }
    }
    (DEFAULT_FROM, DEFAULT_TO)
}

pub fn enabled() -> bool {
    std::path::Path::new(PROBE_MARKER).exists()
}

/// Catch the fault and resume in the loop, rather than letting it unwind the client.
unsafe extern "system" fn veh(info: *mut ExceptionPointers) -> i32 {
    if !IN_CALL.load(Ordering::SeqCst) || info.is_null() {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let code = (*(*info).record).code;
    // Leave debugger traps alone; everything else is ours to swallow.
    if code == 0x8000_0003 || code == 0x8000_0004 {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    IN_CALL.store(false, Ordering::SeqCst);
    FAULTED.store(true, Ordering::SeqCst);
    FAULTS.fetch_add(1, Ordering::Relaxed);

    let restore: extern "system" fn(*mut c_void, *mut c_void) =
        std::mem::transmute(RESTORE_CTX.load(Ordering::SeqCst) as usize);
    restore(
        std::ptr::addr_of_mut!(SAVED) as *mut c_void,
        std::ptr::null_mut(),
    );
    EXCEPTION_CONTINUE_SEARCH // not reached
}

/// Make a function return to its caller immediately: `xor eax,eax; ret`.
///
/// Used on `ExitProcess`/`TerminateProcess` so a handler that decides to quit does not
/// take the walk down with it.
unsafe fn neutralise(module: &CStr, name: &CStr) {
    let m = GetModuleHandleA(module.as_ptr().cast());
    if m.is_null() {
        return;
    }
    let f = GetProcAddress(m, name.as_ptr().cast());
    if f.is_null() {
        return;
    }
    let mut old = 0u32;
    if VirtualProtect(f, 8, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return;
    }
    let p = f.cast::<u8>();
    *p = 0x31; // xor eax, eax
    *p.add(1) = 0xC0;
    *p.add(2) = 0xC3; // ret
    VirtualProtect(f, 8, old, &mut old);
    log(&format!("probe: neutralised {}", name.to_string_lossy()));
}

/// Rewrite the opcode of a captured packet and re-dispatch it, once per opcode.
///
/// `conn` and `view` are the dispatcher's own arguments; `tramp` is the trampoline back
/// into the real dispatcher. Returns once the flag moves or the range is exhausted.
pub unsafe fn run(
    conn: *mut c_void,
    view: *mut c_void,
    tramp: extern "system" fn(*mut c_void, *mut c_void) -> u64,
) {
    if conn.is_null() || view.is_null() || DONE.swap(true, Ordering::SeqCst) {
        return;
    }
    let (from, to) = range();

    let ntdll = GetModuleHandleA(c"ntdll.dll".as_ptr().cast());
    let restore = GetProcAddress(ntdll, c"RtlRestoreContext".as_ptr().cast());
    if restore.is_null() {
        log("probe: RtlRestoreContext not found - aborting");
        return;
    }
    RESTORE_CTX.store(restore as u64, Ordering::SeqCst);
    AddVectoredExceptionHandler(1, veh as *const c_void);
    neutralise(c"kernel32.dll", c"ExitProcess");
    neutralise(c"kernel32.dll", c"TerminateProcess");

    // Snapshot the view and its buffer so every opcode starts from the same packet.
    let data = *(view.cast::<u8>().add(VIEW_DATA).cast::<*mut u8>());
    let len = *(view.cast::<u8>().add(VIEW_LEN).cast::<u32>()) as usize;
    if data.is_null() || !(OPCODE_AT + 2..=0x10000).contains(&len) {
        log(&format!("probe: packet looks wrong (data={data:?} len={len}) - aborting"));
        return;
    }
    let view_snapshot: Vec<u8> = (0..VIEW_STRUCT)
        .map(|i| *view.cast::<u8>().add(i))
        .collect();
    let buf_snapshot: Vec<u8> = (0..len).map(|i| *data.add(i)).collect();
    let cursor = *(view.cast::<u8>().add(VIEW_CURSOR).cast::<u32>());

    log(&format!(
        "probe: walking 0x{from:04X}..0x{to:04X}, packet len={len} cursor={cursor}"
    ));

    RUNNING.store(true, Ordering::SeqCst);
    CURRENT.store(from, Ordering::SeqCst);

    loop {
        let op = CURRENT.load(Ordering::SeqCst);
        if op >= to {
            break;
        }
        CURRENT.store(op + 1, Ordering::SeqCst);

        // Restore the packet, then set the opcode under test.
        for (i, b) in view_snapshot.iter().enumerate() {
            *view.cast::<u8>().add(i) = *b;
        }
        for (i, b) in buf_snapshot.iter().enumerate() {
            *data.add(i) = *b;
        }
        *(data.add(OPCODE_AT).cast::<u16>()) = op as u16;

        // Leave a breadcrumb before each call: if a handler kills the process outright
        // despite the guards, the log still says exactly which opcode did it.
        if op.is_multiple_of(0x40) {
            log(&format!("probe: at 0x{op:04X} (faults so far {})", FAULTS.load(Ordering::Relaxed)));
        }

        FAULTED.store(false, Ordering::SeqCst);
        RtlCaptureContext(std::ptr::addr_of_mut!(SAVED) as *mut c_void);
        // Reached twice: once normally, and again if the VEH longjmps back here.
        if !FAULTED.load(Ordering::SeqCst) {
            IN_CALL.store(true, Ordering::SeqCst);
            tramp(conn, view);
            IN_CALL.store(false, Ordering::SeqCst);
        }

        let flag = *(conn.cast::<u8>().add(CONN_DONE_FLAG));
        if flag != 0 {
            let op = CURRENT.load(Ordering::SeqCst) - 1;
            log(&format!(
                "***** FOUND IT: inbound opcode 0x{op:04X} sets conn+0x150 \
                 (faults along the way: {}) *****",
                FAULTS.load(Ordering::Relaxed)
            ));
            RUNNING.store(false, Ordering::SeqCst);
            return;
        }
    }

    RUNNING.store(false, Ordering::SeqCst);
    log(&format!(
        "probe: finished 0x{from:04X}..0x{to:04X} with no hit ({} faults)",
        FAULTS.load(Ordering::Relaxed)
    ));
}
