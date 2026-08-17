//! Log the client's socket lifecycle: every `connect()` it attempts, and every teardown of
//! a socket it already has.
//!
//! # The questions this answers
//!
//! **Does the client migrate?** After login it closes our connection. Two explanations fit
//! equally well: it is migrating to a channel server and cannot, because we send 256 zero
//! bytes and any address field reads `0.0.0.0:0`; or it is simply timing out.
//!
//! A socket poll cannot separate these. `tools/watch-sockets.ps1` saw one socket and no
//! second connect, but a client that sanity-checks an address before dialling never
//! creates a socket at all - so "no socket" is exactly what *both* explanations look like.
//! Hooking `connect` sees the attempt itself. **Settled: it never calls it again.**
//!
//! **Who closes the connection, and why?** That is the open one. The close is not an idle
//! timeout in any obvious sense - the last thing the client sends is `0x007A`, which
//! `FUN_141b0ef00` emits when its four *loading* phases finish, carrying their durations.
//! So the close arrives on the heels of loading completing, not of a quiet socket, and the
//! two have been confounded because they happen at the same moment.
//!
//! `closesocket`/`shutdown` are the same kind of decisive test as `connect` was: whoever
//! calls them names the moment. And unlike the login-failure path, this call is very
//! unlikely to be virtualised - it is ordinary networking teardown - so `called-from`
//! should land in `.text` and be decompilable. If it does not, the stack scan is there.
//!
//! # Why this is easy where the client is not
//!
//! `ws2_32.dll` is a stock system DLL - not packed, not virtualised, exports resolvable by
//! name. None of the Themida problems that made the client's own decision logic unreadable
//! apply here.
//!
//! # How
//!
//! An `int3` on each export, caught by a vectored handler that logs the arguments that
//! matter for that export, then restores the byte, single-steps, and re-arms - the same
//! technique as `probe::watch`. Its own handler and its own state, so it cannot disturb
//! the opcode probe.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use crate::hook::log;

/// Presence of this file enables the socket watch.
pub const NETWATCH_MARKER: &str = "maplecw-hook.netwatch";

/// What a watched export tells us, which decides how its arguments are read.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// `connect(SOCKET, const sockaddr *, int)`, `WSAConnect(SOCKET, const sockaddr *, ...)`
    /// - `rdx` is the address in both.
    Connect,
    /// `closesocket(SOCKET)`, `shutdown(SOCKET, int how)` - `rcx` is the socket, and for
    /// `shutdown` `rdx` says which directions are being torn down.
    Close,
}

const EXPORTS: [(&[u8], Kind); 4] = [
    (b"connect\0", Kind::Connect),
    (b"WSAConnect\0", Kind::Connect),
    (b"closesocket\0", Kind::Close),
    (b"shutdown\0", Kind::Close),
];

const MAX_TARGETS: usize = EXPORTS.len();
#[allow(clippy::declare_interior_mutable_const)]
const ZERO: AtomicU64 = AtomicU64::new(0);
static TARGETS: [AtomicU64; MAX_TARGETS] = [ZERO; MAX_TARGETS];
static ORIG: [AtomicU64; MAX_TARGETS] = [ZERO; MAX_TARGETS];
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
const CTX_RCX: usize = 0x80;
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
    for (i, (name, _)) in EXPORTS.iter().enumerate() {
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
        return;
    }
    // On a thread, and never inline. The first version ran the self-test here and cost
    // **two seconds** before `hook::install()` got as far as arming the dispatcher - long
    // enough that the client's gate packet was dispatched unhooked, which in turn meant the
    // `watch` probe armed late and the suppressed dialog came back. A diagnostic that
    // changes the thing it is diagnosing is worse than no diagnostic.
    std::thread::spawn(|| unsafe { self_test() });
}

/// Prove the hook fires, by calling the thing it watches.
///
/// **This exists because a negative from this hook was once written down as settled.** The
/// `connect` watch has never logged a single `CONNECT` line - including for the connection
/// to our own server, which certainly happened. Two explanations fit: the client reaches
/// the socket through some path these exports do not cover, or the hook simply does not
/// work. Until one is ruled out, "the client never called `connect`" is not evidence of
/// anything, and the migration question it was used to close is still open.
///
/// So make a call we control. A `connect` to a discard address on the loopback interface
/// needs no server and no network: what matters is only that the export was *entered*. If
/// `SELF-TEST ok` appears in the log, later silence is a real negative; if it does not, the
/// hook is broken and nothing it reports means anything.
///
/// The socket is put in **non-blocking** mode first. Blocking cost two seconds here,
/// because this client runs behind a firewall rule that drops its outbound traffic, so the
/// connect sat in SYN retries. Non-blocking returns `WSAEWOULDBLOCK` immediately and the
/// breakpoint has already been hit by then, which is the whole point.
unsafe fn self_test() {
    const AF_INET: i32 = 2;
    const SOCK_STREAM: i32 = 1;
    const FIONBIO: i32 = -0x7FFB_9982; // 0x8004667E
    const INVALID_SOCKET: usize = usize::MAX;
    extern "system" {
        fn socket(af: i32, ty: i32, proto: i32) -> usize;
        fn connect(s: usize, name: *const u8, namelen: i32) -> i32;
        fn closesocket(s: usize) -> i32;
        fn ioctlsocket(s: usize, cmd: i32, argp: *mut u32) -> i32;
    }

    let before = HITS.load(Ordering::SeqCst);
    let s = socket(AF_INET, SOCK_STREAM, 0);
    if s == INVALID_SOCKET {
        log("netwatch: SELF-TEST inconclusive - could not create a socket");
        return;
    }
    let mut nonblocking = 1u32;
    ioctlsocket(s, FIONBIO, &mut nonblocking);
    // sockaddr_in { family=AF_INET, port=9 (discard), addr=127.0.0.1 }, network order.
    let mut sa = [0u8; 16];
    sa[0..2].copy_from_slice(&(AF_INET as u16).to_le_bytes());
    sa[2..4].copy_from_slice(&9u16.to_be_bytes());
    sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
    connect(s, sa.as_ptr(), sa.len() as i32);
    closesocket(s);

    // At least our own `connect` and `closesocket`. The client is running concurrently, so
    // this can over-count - which only ever makes the check pass, and it is a sanity check
    // on the instrument, not a measurement.
    let hits = HITS.load(Ordering::SeqCst) - before;
    if hits >= 2 {
        log(&format!(
            "netwatch: SELF-TEST ok - {hits} calls were caught while making 2 of our own, \
             so a later absence of lines is a real negative"
        ));
    } else {
        log(&format!(
            "netwatch: SELF-TEST FAILED - only {hits} calls caught while making 2 of our \
             own. DO NOT read anything into what this hook does or does not report."
        ));
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
            let (name, kind) = EXPORTS[i];
            let label = std::str::from_utf8(&name[..name.len() - 1]).unwrap_or("?");
            let rsp = *(ctx.add(CTX_RSP).cast::<u64>()) as usize;
            // The breakpoint is on the export's first byte, so the call has just pushed
            // its return address and `[rsp]` is the immediate caller.
            let from = if crate::session::can_read(rsp, 8) {
                format!(" called-from={:#x}", *(rsp as *const u64))
            } else {
                String::new()
            };
            let what = match kind {
                Kind::Connect => {
                    let sockaddr = *(ctx.add(CTX_RDX).cast::<u64>()) as usize;
                    format!("CONNECT -> {}", describe(sockaddr))
                }
                Kind::Close => {
                    let socket = *(ctx.add(CTX_RCX).cast::<u64>());
                    let how = *(ctx.add(CTX_RDX).cast::<u64>()) as u32;
                    if label == "shutdown" {
                        // SD_RECEIVE / SD_SEND / SD_BOTH.
                        let dir = match how {
                            0 => "recv",
                            1 => "send",
                            2 => "both",
                            _ => "?",
                        };
                        format!("SHUTDOWN socket={socket:#x} how={how} ({dir})")
                    } else {
                        format!("CLOSESOCKET socket={socket:#x}")
                    }
                }
            };
            // A close is the thing we are trying to attribute, so spend the stack scan on
            // it; a connect is already answered by the address alone.
            let stack = if kind == Kind::Close {
                let t = crate::probe::stack_trace(rsp);
                if t.is_empty() {
                    String::new()
                } else {
                    format!("\n          stack: {t}")
                }
            } else {
                String::new()
            };
            log(&format!("***** {what} #{n}{from} *****{stack}"));
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
