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
use std::sync::atomic::{AtomicBool, Ordering};

use crate::hook::log;

/// Marker file selecting the mode. Absent means this module does nothing at all.
pub const SESSION_MARKER: &str = "maplecw-hook.session";

/// `DAT_143aa84a0` — the global holding a pointer to the account/session object.
const SESSION_PTR_RVA: usize = 0x143AA84A0 - 0x140000000;

/// Flag byte. Bit 2 gates the dialog entirely.
const FLAGS_OFF: usize = 0x2270;
/// Status byte selecting which dialog. 0 and any unmapped value mean "trouble logging in".
const STATUS_OFF: usize = 0x227C;
/// The bit of [`FLAGS_OFF`] that `FUN_1415d9210` tests.
const DIALOG_FLAG: u8 = 4;

const MEM_COMMIT: u32 = 0x1000;
/// Protections that allow a read. `PAGE_NOACCESS` (0x01) and `PAGE_GUARD` (0x100) do not.
const READABLE: u32 = 0x02 | 0x04 | 0x08 | 0x20 | 0x40 | 0x80;
const PAGE_GUARD: u32 = 0x100;

static PATCHED: AtomicBool = AtomicBool::new(false);

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

/// Poll the session object and report the two bytes that decide the prompt.
pub unsafe fn monitor(base: usize) {
    let Some(mode) = mode() else { return };
    let ptr_at = base + SESSION_PTR_RVA;
    log(&format!(
        "session: watching DAT_143aa84a0 at {ptr_at:#x} (+0x2270 flags, +0x227c status), \
         mode={mode:?}"
    ));

    let mut last: Option<(u8, u8)> = None;
    let mut announced_null = false;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(200));

        if !readable(ptr_at, 8) {
            continue;
        }
        let obj = *(ptr_at as *const usize);
        if !readable(obj + STATUS_OFF, 1) {
            if !announced_null {
                log("session: object not ready yet (null or unreadable), still polling");
                announced_null = true;
            }
            continue;
        }

        let flags = *((obj + FLAGS_OFF) as *const u8);
        let status = *((obj + STATUS_OFF) as *const u8);
        if last != Some((flags, status)) {
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
            log(&format!(
                "***** SESSION obj={obj:#x} +0x2270={flags:#04x} +0x227c={status:#04x} \
                 -> {verdict} *****"
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
