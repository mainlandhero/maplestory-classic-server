//! Watch — and optionally rewrite — the two bytes that decide the login prompt.
//!
//! # What this is for
//!
//! `FUN_1415d9210` raises the "Having trouble logging in?" dialog from two fields on the
//! account/session object `DAT_143aa84a0`:
//!
//! ```text
//! if ((*(u8 *)(obj + 0x2270) & 4) == 0) return;   // no flag -> no dialog at all
//! switch (*(u8 *)(obj + 0x227c)) {
//!     case 0:    -> loginTroubleAskSupport    <- ours
//!     case 1:    -> 0x2100000D
//!     case 2:    -> incorrectFormOfID
//!     case 0x11: -> errorUnableToConnect
//!     case 0x1B: -> temporaryBlockedIPAddr
//!     case 0x1C: -> 0x2100000C
//!     case 0xFF: -> selectiveShutdownYouth
//!     default:   -> loginTroubleAskSupport    <- also ours
//! }
//! ```
//!
//! The prompt is the **zero/default case** of a status byte on the same object that holds
//! `textAccount` (`+0x22f8`) and the world/channel ids — i.e. exactly what an unpopulated
//! session looks like. See `docs/session.md`.
//!
//! # Why a polling thread rather than a hook
//!
//! The dialog appears seconds after the login screen, and in a gate-only run the client
//! dispatches exactly **one** inbound packet. Sampling on dispatch would give a single
//! reading, taken before the interesting moment. Polling produces a timeline instead, which
//! also shows *when* the fields change and therefore what changes them.
//!
//! # Modes
//!
//! Driven by the marker file [`SESSION_MARKER`]:
//!
//! * empty or `watch` — log the two bytes and every change. Changes nothing.
//! * `suppress` — additionally clear bit 2 of `+0x2270`, so `FUN_1415d9210` returns before
//!   raising anything. This is the one-line test of the whole theory: if the prompt stops
//!   appearing, the chain is proven.
//! * `status=<hex>` — additionally write that byte to `+0x227c`.
//!
//! `suppress` and `status=` are **client-side patches**. They make the client stop
//! reporting an invalid session; they do not make the session valid. Do not report a run
//! using them as "the session works".

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::hook::log;

/// Marker file selecting the mode. Absent means this module does nothing at all.
pub const SESSION_MARKER: &str = "maplecw-hook.session";

/// `DAT_143aa84a0` — the global holding a pointer to the account/session object.
const SESSION_PTR_RVA: usize = 0x143AA84A0 - 0x140000000;

/// Flag byte. Bit 2 gates the dialog entirely.
const FLAGS_OFF: usize = 0x2270;
/// Status byte selecting which dialog. 0 and any unmapped value mean "trouble logging in".
const STATUS_OFF: usize = 0x227C;
/// World id (`FUN_142cb9230`) and channel id (`FUN_142cb9260`).
///
/// The login result compares the pair it carries against these two, and calls
/// `FUN_141b2c7c0` - which searches the world list and *advances the client* - only when
/// they differ. Sending a matching pair is therefore how to answer `0x0080` without the
/// client jumping straight to character select, which is not the real flow.
const WORLD_OFF: usize = 0x2258;
const CHANNEL_OFF: usize = 0x2260;
/// The bit of [`FLAGS_OFF`] that `FUN_1415d9210` tests.
const DIALOG_FLAG: u8 = 4;

const MEM_COMMIT: u32 = 0x1000;
/// Protections that allow a read. `PAGE_NOACCESS` (0x01) and `PAGE_GUARD` (0x100) do not.
const READABLE: u32 = 0x02 | 0x04 | 0x08 | 0x20 | 0x40 | 0x80;
const PAGE_GUARD: u32 = 0x100;

static PATCHED: AtomicBool = AtomicBool::new(false);

/// The connection object, captured from the dispatcher's first argument.
///
/// Recorded so the socket handle inside it can be watched - see [`CONN_SOCKET_OFF`].
pub(crate) static CONN: AtomicUsize = AtomicUsize::new(0);

/// `conn + 0x20` - the `SOCKET`, read from `FUN_1415d35f0`, which hands `conn + 0x20` to
/// `FUN_1415e3b60`, and that is the function that calls `closesocket` and then stores `-1`.
///
/// **Why this is worth polling.** With the code watch verified by a canary, the client's
/// connection teardown and session destructor are both known *not* to run, and neither
/// `closesocket` nor `shutdown` is called - yet the connection is reset while the client
/// carries on. Two possibilities remain, and this field separates them:
///
/// * the handle goes to `-1` -> something *did* tear the socket down, through a path that
///   does not go via the two watched functions;
/// * the handle stays a live-looking value -> the client still believes it owns a socket
///   that is already dead, so the reset came from outside the client's own logic.
const CONN_SOCKET_OFF: usize = 0x20;

/// Is this handle still open in our process?
///
/// `GetHandleInformation` touches nothing and works on socket handles, which are ordinary
/// kernel handles. It is the cheapest way to tell "the socket object is gone" from "the
/// socket object is fine and the *connection* is what died" - and those two point at
/// completely different culprits.
unsafe fn handle_is_valid(handle: u64) -> bool {
    extern "system" {
        fn GetHandleInformation(handle: usize, flags: *mut u32) -> i32;
    }
    let mut flags = 0u32;
    GetHandleInformation(handle as usize, &mut flags) != 0
}

fn describe_socket(socket: u64) -> &'static str {
    if socket == u64::MAX {
        "closed and cleared by the client (FUN_1415e3b60 ran)"
    } else {
        "the client holds a socket"
    }
}

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

extern "system" {
    fn VirtualQuery(addr: *const c_void, buf: *mut MemoryBasicInformation, len: usize) -> usize;
}

/// Is `len` bytes at `addr` committed and readable?
///
/// The session pointer is null for the first seconds of startup and garbage if the offset
/// is ever wrong, and a bad read here would crash the client mid-test — an expensive way to
/// learn nothing, given each run costs a manual launch.
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr == 0 {
        return false;
    }
    let mut mbi = MemoryBasicInformation {
        base: std::ptr::null_mut(),
        allocation_base: std::ptr::null_mut(),
        ..Default::default()
    };
    let size = std::mem::size_of::<MemoryBasicInformation>();
    if VirtualQuery(addr as *const c_void, &mut mbi, size) == 0 {
        return false;
    }
    if mbi.state != MEM_COMMIT || mbi.protect & PAGE_GUARD != 0 || mbi.protect & READABLE == 0 {
        return false;
    }
    // Do not let a read run off the end of the region into an unmapped neighbour.
    let end = mbi.base as usize + mbi.region_size;
    addr + len <= end
}

fn mode() -> Option<String> {
    std::fs::read_to_string(SESSION_MARKER)
        .ok()
        .map(|t| t.trim().to_ascii_lowercase())
}

pub fn enabled() -> bool {
    std::path::Path::new(SESSION_MARKER).exists()
}

/// Guarded read for other modules — notably the probe, which dereferences argument
/// registers inside a vectored handler where a fault would be fatal.
pub(crate) unsafe fn can_read(addr: usize, len: usize) -> bool {
    readable(addr, len)
}

/// `DAT_143ac1898` — the global holding the launch/session config object.
const CONFIG_PTR_RVA: usize = 0x143AC1898 - 0x140000000;
/// The launch mode. `-NXLDEBUG` sets 5.
const MODE_OFF: usize = 0x68;
/// The world list. Patching on the way out of this handler is the only correct moment.
const WORLD_LIST_OPCODE: u16 = 0x000B;

static MODE_PATCHED: AtomicBool = AtomicBool::new(false);

/// Leave launch mode 5, once the world list has landed.
///
/// # Why
///
/// Mode 5 (`-NXLDEBUG`) makes the client log itself in. `FUN_14112a720`'s per-frame tick,
/// once `stage+0x108` is set, calls `FUN_141b3ff10` — *the same function the Login button
/// calls* — so the button never gets a turn. With any other mode, `FUN_141b3fd10` is false,
/// the tick does nothing, the button still enables (that happens independently of mode),
/// and clicking it runs `FUN_141b3f050(stage, 4, 600)`: an animated 600 ms transition to
/// CharSelect, which is the real flow.
///
/// # Why here and not on a timer
///
/// Too early and `FUN_141b21ea0` takes a different branch and never sends `0x0073` /
/// `0x0080`. Too late and the tick has already auto-logged-in. Inside the world-list
/// dispatch is the one window where the flag is set and the frame loop has not run.
///
/// Also: `0x000B` is handled by the mode-5 `FUN_141b31ff0` on the way in, which is the
/// variant whose field order we decoded. Patching on the way *out* keeps that true.
///
/// # Do not send a world-list terminator with this
///
/// This fires after the **first** `0x000B`, so any later one is handled by the *classic*
/// `FUN_141b2fac0` — and its terminator branch calls `FUN_141b3f050(stage, 2, 400)`,
/// transitioning to **WorldSelect**. The mode-5 terminator has no transition, so the
/// difference only appears once the mode is patched. It cost a run: the client jumped to a
/// world-select screen that this service does not even use.
///
/// Send the world *entry* only. `stage+0x108` is set by the entry; the terminator merely
/// closes the list, and nothing here needs it closed.
///
/// **This patches the client.** It makes the client follow the normal flow; it does not
/// make the session valid.
pub unsafe fn patch_mode_after_dispatch(opcode: u16) {
    if opcode != WORLD_LIST_OPCODE || MODE_PATCHED.load(Ordering::SeqCst) {
        return;
    }
    let Some(mode) = mode() else { return };
    let Some(hex) = mode.strip_prefix("mode=") else {
        return;
    };
    let Ok(want) = u32::from_str_radix(hex.trim().trim_start_matches("0x"), 16) else {
        log(&format!("session: {mode:?} is not mode=<hex>"));
        MODE_PATCHED.store(true, Ordering::SeqCst);
        return;
    };

    let base = crate::hook::base();
    let ptr_at = base + CONFIG_PTR_RVA;
    if !readable(ptr_at, 8) {
        return;
    }
    let obj = *(ptr_at as *const usize);
    if obj == 0 || !readable(obj + MODE_OFF, 4) || !writable(obj + MODE_OFF) {
        return;
    }
    let was = *((obj + MODE_OFF) as *const u32);
    *((obj + MODE_OFF) as *mut u32) = want;
    MODE_PATCHED.store(true, Ordering::SeqCst);
    log(&format!(
        "***** SESSION mode patched at {obj:#x}+0x68: {was} -> {want} - the tick should no \
         longer auto-login, and Login should transition to CharSelect *****"
    ));
}

/// Poll the session object and report the two bytes that decide the prompt.
pub unsafe fn monitor(base: usize) {
    let Some(mode) = mode() else { return };
    let ptr_at = base + SESSION_PTR_RVA;
    log(&format!(
        "session: watching DAT_143aa84a0 at {ptr_at:#x} (+0x2270 flags, +0x227c status), \
         mode={mode:?}"
    ));

    let mut last: Option<(u8, u8)> = None;
    let mut was_readable: Option<bool> = None;
    let mut last_socket: Option<u64> = None;
    let mut last_alive: Option<bool> = None;
    let mut ticks: u32 = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(200));
        ticks += 1;

        // Report every change to the connection's socket handle, including the first sight
        // of it. `-1` is what `FUN_1415e3b60` writes after closing; anything else means the
        // client still thinks it holds a socket.
        let conn = CONN.load(Ordering::SeqCst);
        if conn != 0 && readable(conn + CONN_SOCKET_OFF, 8) {
            let socket = *((conn + CONN_SOCKET_OFF) as *const u64);
            if last_socket != Some(socket) {
                if socket != u64::MAX {
                    crate::netwatch::arm_handle_watch(socket);
                }
                log(&format!(
                    "***** SOCKET conn={conn:#x} +0x20={socket:#x} - {} *****",
                    describe_socket(socket)
                ));
                last_socket = Some(socket);
            } else if socket != u64::MAX && ticks.is_multiple_of(5) {
                // Once a second, ask the OS whether the handle the client still holds is
                // actually alive. This is the whole question: if the client's field is
                // unchanged but the handle is dead, something closed it behind the client's
                // back; if the handle is alive while the connection is not, nothing closed
                // it at all and the reset came from outside the client.
                let alive = handle_is_valid(socket);
                if last_alive != Some(alive) {
                    log(&format!(
                        "***** SOCKET {socket:#x} is now {} (client still holds it) *****",
                        if alive { "VALID" } else { "AN INVALID HANDLE" }
                    ));
                    last_alive = Some(alive);
                }
            }
        }

        let obj = if readable(ptr_at, 8) { *(ptr_at as *const usize) } else { 0 };
        let ok = obj != 0 && readable(obj + STATUS_OFF, 1);

        // Log every readable<->unreadable transition, not just the first. Logging "not
        // ready" once and then falling silent left it ambiguous whether the fields stayed
        // put or whether the object simply vanished before the interesting moment - which
        // is exactly what happened on the first run of this.
        if was_readable != Some(ok) {
            log(&format!(
                "session: object at {obj:#x} is now {}",
                if ok { "readable" } else { "UNREADABLE - values below are stale" }
            ));
            was_readable = Some(ok);
            last = None; // force a fresh reading when it comes back
        }
        if !ok {
            continue;
        }

        let flags = *((obj + FLAGS_OFF) as *const u8);
        let status = *((obj + STATUS_OFF) as *const u8);
        // Heartbeat every ~3s even when nothing changes, so silence in the log always
        // means "not running" and never "running but unchanged".
        if last != Some((flags, status)) || ticks.is_multiple_of(15) {
            let verdict = if flags & DIALOG_FLAG == 0 {
                "no dialog (flag bit 2 clear)"
            } else {
                match status {
                    1 => "0x2100000D",
                    2 => "incorrectFormOfID",
                    0x11 => "errorUnableToConnect",
                    0x1B => "temporaryBlockedIPAddr",
                    0x1C => "0x2100000C",
                    0xFF => "selectiveShutdownYouth",
                    _ => "loginTroubleAskSupport  <- the prompt",
                }
            };
            let world = if readable(obj + WORLD_OFF, 4) {
                *((obj + WORLD_OFF) as *const u32) as i64
            } else {
                -1
            };
            let channel = if readable(obj + CHANNEL_OFF, 4) {
                *((obj + CHANNEL_OFF) as *const u32) as i64
            } else {
                -1
            };
            log(&format!(
                "***** SESSION obj={obj:#x} +0x2270={flags:#04x} +0x227c={status:#04x} \
                 world={world} channel={channel} -> {verdict} *****"
            ));
            last = Some((flags, status));
        }

        // Patching modes. Once only: rewriting every 200 ms would fight whatever the
        // client does and make the log unreadable.
        if PATCHED.load(Ordering::SeqCst) {
            continue;
        }
        if let Some(hex) = mode.strip_prefix("status=") {
            if let Ok(v) = u8::from_str_radix(hex.trim().trim_start_matches("0x"), 16) {
                if writable(obj + STATUS_OFF) {
                    *((obj + STATUS_OFF) as *mut u8) = v;
                    log(&format!("session: wrote +0x227c = {v:#04x}"));
                    PATCHED.store(true, Ordering::SeqCst);
                }
            } else {
                log(&format!("session: {mode:?} is not status=<hex>"));
                PATCHED.store(true, Ordering::SeqCst);
            }
        } else if mode == "suppress" && flags & DIALOG_FLAG != 0 && writable(obj + FLAGS_OFF) {
            *((obj + FLAGS_OFF) as *mut u8) = flags & !DIALOG_FLAG;
            log(&format!(
                "session: cleared bit 2 of +0x2270 ({flags:#04x} -> {:#04x}) - \
                 FUN_1415d9210 should now return before raising anything",
                flags & !DIALOG_FLAG
            ));
            PATCHED.store(true, Ordering::SeqCst);
        }
    }
}

/// Writable check, kept separate so a read-only page is reported rather than crashing.
unsafe fn writable(addr: usize) -> bool {
    let mut mbi = MemoryBasicInformation {
        base: std::ptr::null_mut(),
        allocation_base: std::ptr::null_mut(),
        ..Default::default()
    };
    let size = std::mem::size_of::<MemoryBasicInformation>();
    if VirtualQuery(addr as *const c_void, &mut mbi, size) == 0 {
        return false;
    }
    const WRITABLE: u32 = 0x04 | 0x08 | 0x40 | 0x80;
    if mbi.state != MEM_COMMIT || mbi.protect & PAGE_GUARD != 0 || mbi.protect & WRITABLE == 0 {
        log(&format!("session: {addr:#x} is not writable (protect={:#x})", mbi.protect));
        return false;
    }
    true
}
