//! No-op stand-in for `grap64.dll`, the GameGuard **NGS-X** interface layer.
//!
//! The real DLL (`grap-client-pc-release-1.5.0.0`) is a thin shim: it launches
//! `NGService.exe`, which installs a Windows service and the `BlackCat64.sys`
//! **kernel driver**, then talks to the encrypted `grap-core64.aes` engine.
//! Replacing it means none of that happens — no service, no driver, nothing
//! installed system-wide.
//!
//! `MapleStory.exe` statically imports **ordinal #9**, so the file must exist and
//! must export the same ordinals or the process will not start. Every entry point
//! here simply reports success.
//!
//! Every call is logged (see [`LOG_ENV`]) because we do not yet know the client's
//! expected return convention. The log tells us which ordinals are called, in what
//! order, and how far the client gets — which is exactly what we need to tune
//! [`SUCCESS`] if the client rejects the stub.
//!
//! **This is for a local, offline test client only.** It is deliberately useless
//! against a real server: it provides no anti-cheat, and the server it is built for
//! is one we run ourselves on localhost.

#![allow(clippy::missing_safety_doc)]

pub mod heapfix;
pub mod hitnumber;
pub mod hook;
pub mod minidump;
pub mod netwatch;
pub mod probe;
pub mod session;

use std::ffi::c_void;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Value returned from every stubbed entry point.
///
/// `0` is the usual "no error" convention for this style of C API. If the client
/// refuses to start, the log will show which ordinal it gave up after; try `1`
/// next, then whatever the decompiled original returns on its success path.
const SUCCESS: i32 = 0;

/// Set this environment variable to a writable path to record calls.
/// Unset means no logging and no file access at all.
pub const LOG_ENV: &str = "GRAP_STUB_LOG";

static CALLS: AtomicUsize = AtomicUsize::new(0);

fn log(entry: &str) {
    let Ok(path) = std::env::var(LOG_ENV) else {
        return;
    };
    let n = CALLS.fetch_add(1, Ordering::Relaxed);
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "[{n:04}] {entry}");
    }
}

/// Generates a stubbed export: logs the call, returns [`SUCCESS`].
///
/// On x64 Windows the caller cleans up the stack, so ignoring the real parameter
/// list is safe no matter what arity the original had.
macro_rules! stub {
    ($name:ident, $label:literal) => {
        #[no_mangle]
        pub unsafe extern "system" fn $name(
            _a: *mut c_void,
            _b: *mut c_void,
            _c: *mut c_void,
            _d: *mut c_void,
        ) -> i32 {
            log($label);
            // Install the dispatcher hook from here as well as DllMain. A Rust cdylib's
            // DllMain is not reliably invoked, and the first run produced no hook output
            // at all despite the client loading the DLL and running normally - so the
            // export the client statically imports is the dependable trigger.
            hook::install_once();
            SUCCESS
        }
    };
}

stub!(grap_ord_1, "ordinal #1");
stub!(grap_ord_2, "ordinal #2");
stub!(grap_ord_3, "ordinal #3");
stub!(grap_ord_4, "ordinal #4");
stub!(grap_ord_5, "ordinal #5");
stub!(grap_ord_6, "ordinal #6");
stub!(grap_ord_7, "ordinal #7");
stub!(grap_ord_8, "ordinal #8");
// The one MapleStory.exe binds statically; almost certainly GameGuard init.
stub!(grap_ord_9, "ordinal #9  <- statically imported by MapleStory.exe");
stub!(__syscall_Common_Param8, "__syscall_Common_Param8");
stub!(__syscall_Common_Param16, "__syscall_Common_Param16");

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    _module: *mut c_void,
    reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;
    if reason == DLL_PROCESS_ATTACH {
        log("DllMain: PROCESS_ATTACH (GameGuard stub loaded; no service, no driver)");

        // Opt-in dispatcher hook. Installed from a spawned thread rather than inline:
        // DllMain runs under the loader lock, and patching another module's code from
        // there risks deadlock. The delay also lets the client finish unpacking .text
        // before we touch it.
        if std::env::var(hook::HOOK_ENV).is_ok() {
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                unsafe { hook::install() };
            });
        }
    }
    1
}
