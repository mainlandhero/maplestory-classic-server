//! An inline hook on the client's packet dispatcher, to find which inbound opcodes it
//! actually handles.
//!
//! # Why this exists
//!
//! The dispatcher `FUN_1415d60e0` is a 22-byte stub that tail-jumps to `0x144ADD569` in
//! the `.themida` section — a section with **no file bytes**, materialised at runtime, and
//! virtualised once it is there (`pushfq`, `mov rax, rax` filler, constants laundered
//! through the stack). So the inbound opcode table cannot be read statically, and dumping
//! the region gives VM bytecode rather than a jump table. See `docs/transport.md`.
//!
//! But the *stub* is ordinary code in `.text`, and it is the choke point every decrypted
//! packet passes through. Hooking it costs nothing in analysis and answers the one
//! question we cannot answer from outside: **was this opcode handled?**
//!
//! # How the answer falls out
//!
//! An unhandled opcode falls through the dispatch and returns almost immediately. A
//! handled one runs a handler. Logging `opcode` and elapsed ticks per call therefore
//! separates the two, and a single sweep produces the set of handled opcodes — which is
//! exactly what six sweeps of guessing from outside failed to produce.
//!
//! # Scope
//!
//! Opt-in via [`HOOK_ENV`]; unset means this module never touches anything. It is a
//! debugging aid for `client-patched/`, which is firewalled outbound and GameGuard-stubbed,
//! and it is useless anywhere else: it only reads, and only from a client we are pointing
//! at our own loopback server.
//!
//! # Caveat
//!
//! Themida may verify `.text`. If it does, patching here will trip anti-tamper and the
//! client will die on startup — which is itself a clear, quick result. The hook logs
//! every step so a failure says where it happened.

use std::ffi::c_void;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Set to a writable path to install the dispatcher hook and log to that file.
///
/// Kept as an override, but the marker file below is the primary switch: the client
/// requires elevation, so it must be launched via ShellExecute, and that does not
/// reliably carry `$env:` changes into the child.
pub const HOOK_ENV: &str = "MAPLECW_HOOK_LOG";

/// Presence of this file, relative to the client's working directory, enables the hook.
///
/// A file rather than an environment variable because the client runs elevated: launching
/// it needs ShellExecute, which does not propagate environment changes, and switching to
/// CreateProcess to fix that fails outright with "requires elevation". A marker file is
/// immune to both.
pub const HOOK_MARKER: &str = "maplecw-hook.enable";

/// `FUN_1415d60e0(conn, view)` — the dispatcher stub. The client has no ASLR slide
/// (observed base `0x140000000`), but we resolve the module base anyway rather than
/// assuming it.
const DISPATCH_RVA: usize = 0x1415D60E0 - 0x140000000;

/// Bytes of the stub prologue we relocate into the trampoline.
///
/// ```text
/// 48 89 54 24 10        mov [rsp+0x10], rdx
/// 48 89 4c 24 08        mov [rsp+0x08], rcx
/// 48 81 ec 88 00 00 00  sub rsp, 0x88
/// ```
/// 17 bytes, all position-independent, and comfortably more than the 12 a 64-bit
/// absolute jump needs. The `e9` tail-jump that follows is left in place, so the
/// trampoline re-enters at +17 and proceeds into the VM as normal.
const STOLEN: usize = 17;

static INSTALLED: AtomicBool = AtomicBool::new(false);
static TRIGGERED: AtomicBool = AtomicBool::new(false);
static TRAMPOLINE: AtomicU64 = AtomicU64::new(0);
static CALLS: AtomicU64 = AtomicU64::new(0);

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;

extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
    fn VirtualAlloc(addr: *mut c_void, size: usize, typ: u32, protect: u32) -> *mut c_void;
    fn QueryPerformanceCounter(v: *mut i64) -> i32;
    fn QueryPerformanceFrequency(v: *mut i64) -> i32;
}

/// Where to log. Falls back to a file beside the client rather than going silent.
///
/// Both `DllMain` and the export stubs previously logged only when `HOOK_ENV` was set,
/// so a run that produced *no* files was ambiguous: it could mean our code never ran, or
/// merely that the variable did not reach the client. Logging unconditionally removes
/// that ambiguity. Installing the hook is still gated on the variable.
fn log_path() -> String {
    std::env::var(HOOK_ENV).unwrap_or_else(|_| "maplecw-hook.log".to_string())
}

fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path()) {
        let _ = writeln!(f, "{msg}");
    }
}

/// The replacement dispatcher. Times the original and records the opcode.
unsafe extern "system" fn hooked_dispatch(conn: *mut c_void, view: *mut c_void) -> u64 {
    // The view's data pointer sits at +0x10 and covers header *and* payload, so the
    // decrypted body — and therefore the opcode — starts 4 bytes in. See FUN_1406e88d0.
    let opcode = if view.is_null() {
        0xFFFFu16
    } else {
        let data = *(view.cast::<u8>().add(0x10).cast::<*const u8>());
        if data.is_null() {
            0xFFFF
        } else {
            *(data.add(4).cast::<u16>())
        }
    };

    let mut start = 0i64;
    QueryPerformanceCounter(&mut start);

    let tramp: extern "system" fn(*mut c_void, *mut c_void) -> u64 =
        std::mem::transmute(TRAMPOLINE.load(Ordering::SeqCst) as usize);
    let ret = tramp(conn, view);

    let mut end = 0i64;
    QueryPerformanceCounter(&mut end);
    let mut freq = 1i64;
    QueryPerformanceFrequency(&mut freq);
    let micros = (end - start) as f64 * 1_000_000.0 / freq as f64;

    let n = CALLS.fetch_add(1, Ordering::Relaxed);
    // One line per packet: a handled opcode does real work and shows a visibly larger
    // elapsed time than one that falls straight through the dispatch.
    log(&format!("{n:5} opcode=0x{opcode:04X} elapsed_us={micros:.1} ret={ret}"));
    ret
}

/// Install once, from a background thread, on the first call from anywhere.
///
/// Called from every stubbed export as well as `DllMain`, because a Rust `cdylib`'s
/// `DllMain` is not dependably invoked - the first attempt logged nothing at all even
/// though the client loaded the DLL and ran normally. The exports are certain to run:
/// ordinal #9 is statically imported by `MapleStory.exe`.
pub fn install_once() {
    if TRIGGERED.swap(true, Ordering::SeqCst) {
        return;
    }
    let by_env = std::env::var(HOOK_ENV).is_ok();
    let by_marker = std::path::Path::new(HOOK_MARKER).exists();
    let enabled = by_env || by_marker;
    log(&format!(
        "install_once: our code IS running. env={by_env} marker={by_marker} -> {}",
        if enabled { "installing" } else { "standing down" }
    ));
    if !enabled {
        return;
    }
    std::thread::spawn(|| {
        // Let the client finish unpacking .text before patching it.
        std::thread::sleep(std::time::Duration::from_secs(5));
        unsafe { install() };
    });
}

/// Install the hook. Safe to call twice; the second call is a no-op.
pub unsafe fn install() {
    if std::env::var(HOOK_ENV).is_err() {
        return;
    }
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }

    let base = GetModuleHandleA(std::ptr::null()) as usize;
    if base == 0 {
        log("install: GetModuleHandleA failed");
        return;
    }
    let target = base + DISPATCH_RVA;
    log(&format!("install: base={base:#x} target={target:#x}"));

    // Trampoline: the stolen prologue, then an absolute jump back to target+STOLEN.
    let tramp = VirtualAlloc(
        std::ptr::null_mut(),
        64,
        MEM_COMMIT_RESERVE,
        PAGE_EXECUTE_READWRITE,
    );
    if tramp.is_null() {
        log("install: VirtualAlloc failed");
        return;
    }
    let t = tramp.cast::<u8>();
    std::ptr::copy_nonoverlapping(target as *const u8, t, STOLEN);
    write_abs_jmp(t.add(STOLEN), target + STOLEN);
    TRAMPOLINE.store(tramp as u64, Ordering::SeqCst);

    // Patch the stub to jump to us.
    let mut old = 0u32;
    if VirtualProtect(target as *mut c_void, STOLEN, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log("install: VirtualProtect failed (Themida may be guarding .text)");
        return;
    }
    // Bind to a typed fn pointer first: casting a function *item* straight to an
    // integer is a lint trap and can pick up the wrong address.
    let detour: unsafe extern "system" fn(*mut c_void, *mut c_void) -> u64 = hooked_dispatch;
    write_abs_jmp(target as *mut u8, detour as usize);
    // Pad the remainder of the stolen range with NOPs so a disassembler stays sane.
    for i in 12..STOLEN {
        *(target as *mut u8).add(i) = 0x90;
    }
    VirtualProtect(target as *mut c_void, STOLEN, old, &mut old);

    log("install: hook active");
}

/// `mov rax, imm64; jmp rax` — 12 bytes, and unlike a relative `e9` it works at any
/// distance, which matters because the trampoline is a fresh allocation far from `.text`.
unsafe fn write_abs_jmp(at: *mut u8, dest: usize) {
    *at = 0x48;
    *at.add(1) = 0xB8;
    std::ptr::copy_nonoverlapping(dest.to_le_bytes().as_ptr(), at.add(2), 8);
    *at.add(10) = 0xFF;
    *at.add(11) = 0xE0;
}
