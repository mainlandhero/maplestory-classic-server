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
//! # What it established
//!
//! * **Themida does not checksum this part of `.text`** — the inline patch survives and
//!   the client runs on.
//! * **Every opcode arrives as the one we sent**, which confirms framing, header rule, IV
//!   chain and AES key from *inside* the client rather than by inference.
//! * **Elapsed time does discriminate**, once pointed at a range that has handlers. Across
//!   `0x0000`-`0x0019` the spread is a flat 102-148 us — but `0x00A1` came back at 9122 us
//!   and `0x0010` at 355 us against that same baseline. A flat range means no handlers,
//!   not a blunt instrument.
//! * The client accepted **26 packets and no more** at 0.2 s and 3.0 s spacing alike. That
//!   was `FUN_1415e7090`'s startup loop leaking a `0x5b4` buffer per packet, and it ends
//!   the moment `conn+0x150` is set — see [`CONN_DONE_FLAG`]. It was a leak, not a rate
//!   limit, and answering `0x0032` disposes of it.
//! * `conn+0x150` and the patch state are sampled either side of every dispatch, so the
//!   log says which opcode moved them rather than leaving it to be inferred.
//!
//! # Scope
//!
//! Opt-in: enabled only by the [`HOOK_MARKER`] file (or [`HOOK_ENV`]). A debugging aid for
//! `client-patched/`, which is firewalled outbound and GameGuard-stubbed, pointed at our
//! own loopback server.

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

/// `conn + 0x150` — the byte that releases the client from its startup loop.
///
/// `FUN_1415e7090` hashes `Data.wz`, sends opcode `0xA1` carrying that hash, then loops
/// on recv/decrypt/dispatch until *this byte* is non-zero. Nothing else ends the loop, so
/// watching it across a dispatch says precisely whether the opcode we just sent was the
/// one the client was waiting for. See `docs/transport.md`.
pub(crate) const CONN_DONE_FLAG: usize = 0x150;

/// `conn + 0x14c` — the `Data.wz` hash the client computed and sent.
const CONN_DATAWZ_HASH: usize = 0x14C;

/// `DAT_143ace3c8` — the patch state machine: 0 = idle, 1 = mid-transfer, 2 = settled.
///
/// Worth logging separately from the flag: a reply that reaches `FUN_1415e5c20` but
/// declares a non-zero length moves this to 1 *without* setting the flag, so state alone
/// distinguishes "wrong opcode" from "right opcode, wrong body".
const PATCH_STATE_RVA: usize = 0x143ACE3C8 - 0x140000000;

static INSTALLED: AtomicBool = AtomicBool::new(false);
static TRIGGERED: AtomicBool = AtomicBool::new(false);
static TRAMPOLINE: AtomicU64 = AtomicU64::new(0);
static CALLS: AtomicU64 = AtomicU64::new(0);
static BASE: AtomicU64 = AtomicU64::new(0);

/// Module base, once `install()` has resolved it. Zero before that.
pub(crate) fn base() -> usize {
    BASE.load(Ordering::SeqCst) as usize
}

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;

/// `MEMORY_BASIC_INFORMATION`, only as much of it as the readability check needs.
#[repr(C)]
struct MemoryBasicInformation {
    base_address: *mut c_void,
    allocation_base: *mut c_void,
    allocation_protect: u32,
    _alignment: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    kind: u32,
    _alignment2: u32,
}

const MEM_COMMIT: u32 = 0x1000;
const PAGE_NOACCESS: u32 = 0x01;
const PAGE_GUARD: u32 = 0x100;

extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
    fn VirtualQuery(addr: *const c_void, buf: *mut MemoryBasicInformation, len: usize) -> usize;
    fn VirtualAlloc(addr: *mut c_void, size: usize, typ: u32, protect: u32) -> *mut c_void;
    fn QueryPerformanceCounter(v: *mut i64) -> i32;
    fn QueryPerformanceFrequency(v: *mut i64) -> i32;
}

/// Where to log. Falls back to `maplecw-hook.log` beside the client rather than going
/// silent.
///
/// Logging used to be gated on `HOOK_ENV` too, which made a run that produced *no* file
/// ambiguous: our code never ran, or the variable never arrived? Logging unconditionally
/// separates those. Installing is still gated.
pub(crate) fn log_path() -> String {
    std::env::var(HOOK_ENV).unwrap_or_else(|_| "maplecw-hook.log".to_string())
}

/// Wall-clock `HH:MM:SS.mmm`, to match the timestamps `handshake_probe.py` writes.
///
/// Without this the two logs cannot be lined up, and that is not a cosmetic problem: the
/// client resets the connection at a moment when the hook log also shows socket teardowns,
/// and there was no way to tell whether those were the same event or unrelated telemetry
/// sockets closing on their own schedule. Computed from `SystemTime` rather than pulled
/// from a formatting crate, because this runs inside another process's address space and
/// the dependency list here is deliberately empty.
fn stamp() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    // Local time, via the same offset the OS reports, so the two logs agree.
    let secs = now.as_secs() as i64 - i64::from(utc_offset_secs());
    let s = secs.rem_euclid(86_400);
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        s / 3600,
        (s % 3600) / 60,
        s % 60,
        now.subsec_millis()
    )
}

/// `GetTimeZoneInformation`'s `Bias`, in seconds. UTC = local + bias, so subtracting it
/// turns a UTC timestamp into a local one.
fn utc_offset_secs() -> i32 {
    #[repr(C)]
    struct TimeZoneInformation {
        bias: i32,
        rest: [u8; 168],
    }
    extern "system" {
        fn GetTimeZoneInformation(info: *mut TimeZoneInformation) -> u32;
    }
    const TIME_ZONE_ID_INVALID: u32 = u32::MAX;
    const TIME_ZONE_ID_DAYLIGHT: u32 = 2;
    let mut tz = TimeZoneInformation {
        bias: 0,
        rest: [0; 168],
    };
    // SAFETY: the struct is the documented size and is fully initialised above.
    let id = unsafe { GetTimeZoneInformation(&mut tz) };
    if id == TIME_ZONE_ID_INVALID {
        return 0;
    }
    // DaylightBias is the last i32 of the trailing block; add it when DST is in effect.
    let daylight_bias = if id == TIME_ZONE_ID_DAYLIGHT {
        i32::from_le_bytes([tz.rest[164], tz.rest[165], tz.rest[166], tz.rest[167]])
    } else {
        0
    };
    (tz.bias + daylight_bias) * 60
}

/// **One `write` per line, of a string built first.**
///
/// `writeln!` formats straight into the file handle, which issues a write per fragment - so
/// two threads appending at once interleave *within* a line. That is not theoretical; the
/// run of 2026-09-02 produced
///
/// ```text
///   00:24:58.24900:24:58.249  probe: will FORCE rdx=0x0 ...session: watching DAT_143aa84a0
/// ```
///
/// two timestamps and two messages spliced together, at the exact moment the client hung.
/// It became visible then because hooking `CreateMutex` briefly put this function on every
/// thread in the process at startup - but nothing about it was new, and any future hook that
/// logs from more than one thread would have hit it.
///
/// A single append write of a small buffer does not interleave with another process's, which
/// matters here for a second reason: two clients share this file.
pub(crate) fn log(msg: &str) {
    let line = format!("{} {msg}\r\n", stamp());
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path()) {
        let _ = f.write_all(line.as_bytes());
    }
}

/// The replacement dispatcher. Times the original and records the opcode.
unsafe extern "system" fn hooked_dispatch(conn: *mut c_void, view: *mut c_void) -> u64 {
    // Hand the connection object to the session monitor, which polls the socket handle
    // inside it. This is the only place we are given that pointer.
    if !conn.is_null() {
        crate::session::CONN.store(conn as usize, std::sync::atomic::Ordering::SeqCst);
    }

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

    // Sample the two things that say whether this opcode was the one the client wanted,
    // before and after, so a change can be attributed to exactly this packet.
    let read_flag = || -> u32 {
        if conn.is_null() {
            u32::MAX
        } else {
            *(conn.cast::<u8>().add(CONN_DONE_FLAG)) as u32
        }
    };
    let read_state = || -> u32 {
        let base = BASE.load(Ordering::SeqCst) as usize;
        if base == 0 {
            u32::MAX
        } else {
            *((base + PATCH_STATE_RVA) as *const u32)
        }
    };
    let flag_before = read_flag();
    let state_before = read_state();

    // Snapshot before the dispatcher consumes the opcode and moves the read cursor.
    // `CALLS` has not been incremented yet, so this packet is number CALLS + 1.
    let dispatch = CALLS.load(Ordering::Relaxed) + 1;
    if crate::probe::enabled() {
        crate::probe::note_opcode(opcode);
        crate::probe::capture(view, dispatch);
    }

    let mut start = 0i64;
    QueryPerformanceCounter(&mut start);

    let tramp: extern "system" fn(*mut c_void, *mut c_void) -> u64 =
        std::mem::transmute(TRAMPOLINE.load(Ordering::SeqCst) as usize);
    let ret = tramp(conn, view);

    // Apply the launch-mode patch here, on the way out of the handler that made it
    // necessary, rather than on a timer.
    //
    // Ordering is the whole difficulty. Patch too early and `FUN_141b21ea0` takes a
    // different branch and never sends `0x0073`/`0x0080` at all; patch too late and the
    // per-frame tick has already auto-logged-in, because in mode 5 it does not wait for
    // the Login button. Doing it inside the dispatch, after the world list has set
    // `stage+0x108` but before control returns to the frame loop, is the one window where
    // both are true.
    crate::session::patch_mode_after_dispatch(opcode);
    crate::session::enable_character_creation_after_dispatch(opcode);

    let mut end = 0i64;
    QueryPerformanceCounter(&mut end);

    let flag_after = read_flag();
    let state_after = read_state();
    let mut freq = 1i64;
    QueryPerformanceFrequency(&mut freq);
    let micros = (end - start) as f64 * 1_000_000.0 / freq as f64;

    let n = CALLS.fetch_add(1, Ordering::Relaxed);
    log(&format!(
        "{n:5} opcode=0x{opcode:04X} elapsed_us={micros:.1} ret={ret} \
         flag={flag_before}->{flag_after} state={state_before}->{state_after}"
    ));

    // The whole point of the exercise, called out so it cannot be missed in a log of
    // dozens of near-identical lines.
    if flag_after != flag_before && flag_after != u32::MAX {
        let hash = if conn.is_null() {
            0
        } else {
            *(conn.cast::<u8>().add(CONN_DATAWZ_HASH).cast::<u32>())
        };
        log(&format!(
            "***** OPCODE 0x{opcode:04X} RELEASED THE STARTUP LOOP \
             (conn+0x150 {flag_before}->{flag_after}, Data.wz hash now 0x{hash:08X}) *****"
        ));
    } else if state_after != state_before && state_after != u32::MAX {
        log(&format!(
            "***** OPCODE 0x{opcode:04X} REACHED THE Data.wz HANDLER \
             (state {state_before}->{state_after}) but did not set the flag - \
             right opcode, wrong body *****"
        ));
    }

    // With one real packet in hand we have everything the walk needs: a live connection,
    // a well-formed view, and a buffer we own. Run it here rather than from a thread, so
    // the dispatcher is re-entered on the thread that normally calls it.
    //
    // Deliberately not gated on the flag still being clear. The useful sequence is to
    // answer 0x0032 first - which sets it - and *then* walk for some other handler, so
    // gating here skipped the walk entirely. Whether a set flag invalidates the run
    // depends on which oracle is in use, and only the probe knows that, so it decides.
    if crate::probe::enabled() {
        crate::probe::run(conn, view, tramp, dispatch);
    }
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
        wait_for_text_then_install();
    });
}

/// How long to keep waiting for `.text` to settle before installing anyway.
///
/// This used to be a flat `sleep(5)`, and on 2026-08-19 that lost a race it had been
/// winning by luck: the client raised its "having trouble logging in" dialog **21 ms
/// before** the watch that suppresses it was armed, so the suppression never happened and
/// the run was wasted. Across three runs the margin was +1.36 s, +0.74 s, then -0.02 s -
/// it had been shrinking, and nothing in the design kept it positive.
const INSTALL_DEADLINE: std::time::Duration = std::time::Duration::from_secs(5);

/// How often to check whether the client has finished unpacking.
const INSTALL_POLL: std::time::Duration = std::time::Duration::from_millis(25);

/// Wait for the client's `.text` to stop changing, then install.
///
/// The fixed sleep was answering the right question - "has Themida finished unpacking the
/// code we are about to patch?" - with a guess. Watching the bytes answers it directly, and
/// installs as soon as it is safe instead of at a hard-coded time.
///
/// The deadline means this can never wait *longer* than the old sleep, so the worst case is
/// today's behaviour and the normal case is much earlier. The elapsed time is logged
/// because the margin against the client's startup is the thing that actually matters and
/// it was invisible before.
fn wait_for_text_then_install() {
    let started = std::time::Instant::now();
    let base = unsafe { GetModuleHandleA(std::ptr::null()) } as usize;
    if base == 0 {
        log("install: no module handle while waiting; falling back to a fixed delay");
        std::thread::sleep(INSTALL_DEADLINE);
        unsafe { install() };
        return;
    }

    // A window at the dispatcher: the first thing we patch, so the first thing that has to
    // be real. Packed pages read as zeros or as filler and keep changing while Themida
    // works; two identical non-zero reads mean it has stopped.
    let probe = (base + DISPATCH_RVA) as *const u8;

    // **Ask before reading.** This used to dereference `probe` the instant the DLL loaded.
    // Themida maps and decrypts sections lazily, so early in startup that page can be
    // uncommitted or PAGE_NOACCESS, and the read then raises an access violation on a
    // thread that has no handler yet - the client dies in under a second with 0xC0000005
    // and no log, because the first line this function writes comes after the poll. It is
    // a race, so it survived several runs before losing one on 2026-08-19.
    let readable = || -> bool {
        let mut info = std::mem::MaybeUninit::<MemoryBasicInformation>::zeroed();
        let n = unsafe {
            VirtualQuery(
                probe as *const c_void,
                info.as_mut_ptr(),
                std::mem::size_of::<MemoryBasicInformation>(),
            )
        };
        if n == 0 {
            return false;
        }
        let info = unsafe { info.assume_init() };
        info.state == MEM_COMMIT
            && info.protect & PAGE_NOACCESS == 0
            && info.protect & PAGE_GUARD == 0
            && (probe as usize).saturating_add(16)
                <= (info.base_address as usize).saturating_add(info.region_size)
    };

    let read_window = || -> Option<[u8; 16]> {
        if !readable() {
            return None;
        }
        let mut w = [0u8; 16];
        for (i, slot) in w.iter_mut().enumerate() {
            *slot = unsafe { std::ptr::read_volatile(probe.add(i)) };
        }
        Some(w)
    };

    let mut previous = read_window();
    let mut stable_for = std::time::Duration::ZERO;
    while started.elapsed() < INSTALL_DEADLINE {
        std::thread::sleep(INSTALL_POLL);
        let current = read_window();
        if current.is_some() && current == previous && current != Some([0u8; 16]) {
            stable_for += INSTALL_POLL;
            // Two consecutive quiet polls, so a single lucky read cannot pass for settled.
            if stable_for >= INSTALL_POLL * 2 {
                break;
            }
        } else {
            stable_for = std::time::Duration::ZERO;
        }
        previous = current;
    }

    log(&format!(
        "install: .text {} after {} ms (deadline {} ms) - the client's login dialog \
         fires around 4.5 s after connect, so this margin is what decides whether the \
         suppression lands",
        if previous.is_some() { "settled" } else { "NEVER BECAME READABLE" },
        started.elapsed().as_millis(),
        INSTALL_DEADLINE.as_millis()
    ));
    unsafe { install() };
}

/// Install the hook. Safe to call twice; the second call is a no-op.
pub unsafe fn install() {
    // No env check here: install_once() already decided, via the env var *or* the marker
    // file. Re-checking only the env var here is what silently swallowed the marker path.
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }

    let base = GetModuleHandleA(std::ptr::null()) as usize;
    if base == 0 {
        log("install: GetModuleHandleA failed");
        return;
    }
    BASE.store(base as u64, Ordering::SeqCst);
    let target = base + DISPATCH_RVA;
    log(&format!("install: base={base:#x} target={target:#x}"));

    // Started here because this is the first point with a verified module base. It polls
    // rather than piggy-backing on dispatches: a gate-only run dispatches one packet, and
    // one sample taken before the login screen would say nothing about the prompt.
    if crate::session::enabled() {
        std::thread::spawn(move || unsafe { crate::session::monitor(base) });
    }
    // Arm watch mode here rather than on the first dispatched packet. Waiting for a packet
    // made every run a race against the harness's gate, and losing that race silently armed
    // the watch seconds too late — which is how the login dialog came back twice.
    crate::probe::arm_watch();
    // Same moment, same reason: ws2_32 is loaded long before this and the client has
    // already made its first connection, so anything seen from here is a *later* one -
    // which is the only kind that matters.
    crate::netwatch::install();

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

    // Opt-in, and after the detour is in so its log line lands in order. Themida has
    // long since unpacked .text by now - the detour above is written to the same
    // section and has never been checksummed.
    crate::heapfix::install();
    // Same window, same reason: Themida has unpacked .text by now. Gated on
    // `hitnumber=off` in the marker; see crates/grap-stub/src/hitnumber.rs.
    crate::hitnumber::install();
    // The session credential the client will carry itself. Gated on
    // `maplecw-hook.identity` holding a usable token; absent, it does not touch a byte.
    //
    // Here rather than later because `0x0073` is built about 8 s after this point in the one
    // correlated capture - but the ordering this actually rests on is structural, not that
    // margin: the detour sits on the entry of the only function that reads the field.
    // See crates/grap-stub/src/identity.rs.
    crate::identity::install();
    // A read-only watch on the pool allocator's slot headers, so the 0x0000000100000020
    // write is caught ~100 ms after it lands instead of tens of thousands of allocations
    // later. Gated on `maplecw-hook.sentry`; absent, it does not start a thread or read a
    // byte. See crates/grap-stub/src/poolsentry.rs.
    crate::poolsentry::install();
    // Refuse a pool chunk handed to the NT heap - the OTHER lethal free, the one that killed
    // 288744 on a map change through PCOM's WZ property teardown. Gated on `freeguard=` in the
    // session marker; absent, it starts no thread and hooks nothing. It waits for PCOM.dll on
    // a thread of its own, because PCOM's cached free pointer is filled at runtime.
    // See crates/grap-stub/src/freeguard.rs.
    crate::freeguard::install();

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
