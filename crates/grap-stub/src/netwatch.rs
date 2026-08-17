//! Log every `connect()` the client attempts, with the address it asked for.
//!
//! # The question this answers
//!
//! After login the client closes our connection. Two explanations fit equally well:
//!
//! * it is **migrating** to a channel server - normally a different port - and cannot,
//!   because we send 256 zero bytes and any address field reads `0.0.0.0:0`;
//! * it is simply **timing out** after ~8s of silence.
//!
//! A socket poll cannot separate these. `tools/watch-sockets.ps1` saw one socket and no
//! second connect, but a client that sanity-checks an address before dialling never
//! creates a socket at all - so "no socket" is exactly what *both* explanations look like.
//!
//! Hooking `connect` sees the attempt itself, including one to `0.0.0.0:0` that never
//! becomes a socket. It also proves the negative: if `connect` is never called again, the
//! client is not trying to go anywhere.
//!
//! # Why this is easy where the client is not
//!
//! `ws2_32.dll` is a stock system DLL - not packed, not virtualised, exports resolvable by
//! name. None of the Themida problems that made the client's own decision logic unreadable
//! apply here.
//!
//! # How
//!
//! An `int3` on each export, caught by a vectored handler that logs `rdx` (the `sockaddr *`
//! in both signatures), then restores the byte, single-steps, and re-arms - the same
//! technique as `probe::watch`. Its own handler and its own state, so it cannot disturb
//! the opcode probe.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use crate::hook::log;

/// Presence of this file enables the connect watch.
pub const NETWATCH_MARKER: &str = "maplecw-hook.netwatch";

/// `connect(SOCKET, const sockaddr *, int)` and
/// `WSAConnect(SOCKET, const sockaddr *, int, ...)` - `rdx` is the address in both.
const EXPORTS: [&[u8]; 2] = [b"connect\0", b"WSAConnect\0"];

const MAX_TARGETS: usize = EXPORTS.len();
static TARGETS: [AtomicU64; MAX_TARGETS] = [AtomicU64::new(0), AtomicU64::new(0)];
static ORIG: [AtomicU64; MAX_TARGETS] = [AtomicU64::new(0), AtomicU64::new(0)];
static REARM: AtomicUsize = AtomicUsize::new(usize::MAX);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static HITS: AtomicUsize = AtomicUsize::new(0);

/// Enough to see where it is going without flooding a log if something reconnects in a loop.
const MAX_HITS: usize = 64;

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
const EXCEPTION_BREAKPOINT: u32 = 0x8000_0003;
const EXCEPTION_SINGLE_STEP: u32 = 0x8000_0004;
const CTX_EFLAGS: usize = 0x44;
const CTX_RDX: usize = 0x88;
const CTX_RSP: usize = 0x98;
const CTX_RIP: usize = 0xF8;
const TRAP_FLAG: u32 = 0x100;

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

extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: *const c_void) -> *mut c_void;
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

pub fn enabled() -> bool {
    std::path::Path::new(NETWATCH_MARKER).exists()
}

unsafe fn write_byte(addr: usize, value: u8) -> bool {
    let mut old = 0u32;
    if VirtualProtect(addr as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return false;
    }
    *(addr as *mut u8) = value;
    VirtualProtect(addr as *mut c_void, 1, old, &mut old);
    true
}

/// Decode a `sockaddr` for logging. Only AF_INET is spelled out; anything else is reported
/// by family rather than guessed at.
unsafe fn describe(addr: usize) -> String {
    if !crate::session::can_read(addr, 8) {
        return "<unreadable sockaddr>".to_string();
    }
    let family = *(addr as *const u16);
    if family != 2 {
        return format!("family={family} (not AF_INET)");
    }
    // Both are network byte order in the struct.
    let port = u16::from_be(*((addr + 2) as *const u16));
    let ip = u32::from_be(*((addr + 4) as *const u32));
    let [a, b, c, d] = ip.to_be_bytes();
    format!("{a}.{b}.{c}.{d}:{port}")
}

pub unsafe fn install() {
    if !enabled() || INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    let ws2 = GetModuleHandleA(c"ws2_32.dll".as_ptr().cast());
    if ws2.is_null() {
        log("netwatch: ws2_32.dll not loaded yet - connect watch not installed");
        return;
    }
    AddVectoredExceptionHandler(1, veh as *const c_void);

    let mut armed = 0;
    for (i, name) in EXPORTS.iter().enumerate() {
        let addr = GetProcAddress(ws2, name.as_ptr()) as usize;
        if addr == 0 {
            continue;
        }
        let orig = *(addr as *const u8);
        if !write_byte(addr, 0xCC) {
            log(&format!("netwatch: could not arm {addr:#x}"));
            continue;
        }
        TARGETS[i].store(addr as u64, Ordering::SeqCst);
        ORIG[i].store(orig as u64, Ordering::SeqCst);
        armed += 1;
        let label = std::str::from_utf8(&name[..name.len() - 1]).unwrap_or("?");
        log(&format!("netwatch: watching ws2_32!{label} at {addr:#x}"));
    }
    if armed == 0 {
        log("netwatch: nothing armed");
    }
}

unsafe extern "system" fn veh(info: *mut ExceptionPointers) -> i32 {
    if info.is_null() {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let code = (*(*info).record).code;
    let at = (*(*info).record).address as usize;
    let ctx = (*info).context.cast::<u8>();

    if code == EXCEPTION_SINGLE_STEP {
        let i = REARM.swap(usize::MAX, Ordering::SeqCst);
        if i < MAX_TARGETS {
            let addr = TARGETS[i].load(Ordering::SeqCst) as usize;
            if addr != 0 {
                write_byte(addr, 0xCC);
            }
            *(ctx.add(CTX_EFLAGS).cast::<u32>()) &= !TRAP_FLAG;
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        return EXCEPTION_CONTINUE_SEARCH;
    }

    if code != EXCEPTION_BREAKPOINT {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    for i in 0..MAX_TARGETS {
        let addr = TARGETS[i].load(Ordering::SeqCst) as usize;
        if addr == 0 || at != addr {
            continue;
        }
        let n = HITS.fetch_add(1, Ordering::SeqCst) + 1;
        if n <= MAX_HITS {
            let sockaddr = *(ctx.add(CTX_RDX).cast::<u64>()) as usize;
            let rsp = *(ctx.add(CTX_RSP).cast::<u64>()) as usize;
            let from = if crate::session::can_read(rsp, 8) {
                format!(" called-from={:#x}", *(rsp as *const u64))
            } else {
                String::new()
            };
            log(&format!(
                "***** CONNECT #{n}: -> {}{from} *****",
                describe(sockaddr)
            ));
        }

        write_byte(addr, ORIG[i].load(Ordering::SeqCst) as u8);
        *(ctx.add(CTX_RIP).cast::<u64>()) = addr as u64;
        if n >= MAX_HITS {
            TARGETS[i].store(0, Ordering::SeqCst);
            log("netwatch: hit limit reached, disarmed");
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        *(ctx.add(CTX_EFLAGS).cast::<u32>()) |= TRAP_FLAG;
        REARM.store(i, Ordering::SeqCst);
        return EXCEPTION_CONTINUE_EXECUTION;
    }
    EXCEPTION_CONTINUE_SEARCH
}
