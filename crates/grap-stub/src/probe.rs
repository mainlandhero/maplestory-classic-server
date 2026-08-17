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
//! * **A handler tries to end the process.** `ExitProcess`, `TerminateProcess`,
//!   `RtlExitUserProcess` and `NtTerminateProcess` are all detoured to return without
//!   doing anything — detouring only the kernel32 pair was not enough, the client simply
//!   left by a lower door. `NtTerminateThread` is deliberately left alone.
//!
//! # Three modes, and which oracle each uses
//!
//! * `<from>-<to>` — oracle is `conn+0x150`. Only ever finds the one handler that releases
//!   the startup loop. A hit is self-announcing: the run that finds it is the run where the
//!   login screen appears. This found `0x0032`.
//! * `<from>-<to>@<VA>` — oracle is an `int3` on that function. Works for any handler.
//! * `watch@<VA>` — no walking at all: report **every** time that function runs, with the
//!   dispatching opcode and its first four integer arguments (`rcx`, `rdx`, `r8`, `r9`).
//!   Cheaper than guessing at bodies; it confirmed `FUN_141b307b0` is entered on `0x0010`,
//!   and the arguments answer the questions after that — "which *value* was it called
//!   with", e.g. which result code reaches `FUN_141b267c0`, where a readable switch turns
//!   that number into the dialog on screen.
//!
//!   Note it can only arm once the hook sees a dispatch, so a call that happens before the
//!   first inbound packet is dispatched will not be seen.
//! * `watch@<VA>:rdx=<hex>` — watch, and **rewrite** the second integer argument on entry.
//!   For when the caller is virtualised and therefore unreadable: "who decided this" has no
//!   answer, but "what would the client do if this value were X" still does. The log
//!   records the original value first, so the run says what the client actually computed
//!   as well as what we substituted. This patches the client — describe results
//!   accordingly.
//!
//! `#N` starts the walk on the Nth dispatched packet.
//!
//! # Four ways this lies, all of them silently
//!
//! Every one of these produced a fully instrumented, confident miss:
//!
//! * **Aimed at a callee, or at a virtual method.** A walk target must have no direct
//!   callers *and* appear in no vtable. Check with `tools/handler_root.py`, never by eye.
//! * **Run at the wrong moment.** The walk executes inside whatever loop the client is in,
//!   so hunting a login-stage handler before the login screen exists cannot work at any
//!   address. That is what `#N` is for.
//! * **Snapshot taken after the dispatch.** It consumes the opcode and moves the cursor
//!   4 -> 6, so replays start past the opcode and dispatch "opcode 0" every time.
//! * **Two oracles live at once.** With a target armed, `conn+0x150` is *expected* to be
//!   set already, so consulting it too reports a hit on the first opcode tested.
//!
//! Before believing a negative, read the walk's own counters: how many calls advanced the
//! cursor, and how many distinct values the dispatcher returned.
//!
//! # Prefer reading the stage switch
//!
//! The virtualised dispatcher only *routes*. A stage's `OnPacket` is ordinary code —
//! `FUN_141b25f30` is a plain `switch` naming every login-stage opcode at once. Walking is
//! for when no readable switch covers the handler in question.
//!
//! # Scope
//!
//! Opt-in via [`PROBE_MARKER`], for `client-patched/` only - a firewalled, GameGuard-stubbed
//! copy pointed at our own loopback server.

use std::ffi::{c_void, CStr};
use std::fs::OpenOptions;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use crate::hook::{log, CONN_DONE_FLAG};

/// Presence of this file enables the probe. Contents select the mode:
///
/// * `` (empty)              — walk the default range, oracle `conn+0x150`
/// * `0000-1000`             — walk that range, same oracle
/// * `0000-1000@141b25f30`   — walk, oracle is an `int3` on that function
/// * `0000-1000@141b25f30#2` — as above, starting on the 2nd dispatched packet
/// * `watch@141b307b0`       — do not walk; report every entry with its arguments
/// * `watch@141b2a280:rdx=0` — as above, and rewrite RDX on entry
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
static CAPTURED: AtomicBool = AtomicBool::new(false);
static WATCH_ARMED: AtomicBool = AtomicBool::new(false);
static IN_CALL: AtomicBool = AtomicBool::new(false);
static FAULTED: AtomicBool = AtomicBool::new(false);
static CURRENT: AtomicU32 = AtomicU32::new(0);
static FAULTS: AtomicU32 = AtomicU32::new(0);
static RESTORE_CTX: AtomicU64 = AtomicU64::new(0);
static CONSUMED: AtomicU32 = AtomicU32::new(0);
static RET_CHANGES: AtomicU32 = AtomicU32::new(0);
static LAST_RET: AtomicU64 = AtomicU64::new(0);
static TARGET: AtomicU64 = AtomicU64::new(0);
static TARGET_BYTE: AtomicU32 = AtomicU32::new(0);
static HIT: AtomicBool = AtomicBool::new(false);
static WATCH: AtomicU64 = AtomicU64::new(0);
static WATCH_BYTE: AtomicU32 = AtomicU32::new(0);
static WATCH_HITS: AtomicU32 = AtomicU32::new(0);
/// `u64::MAX` means "do not force"; anything else is written to RDX on every watch hit.
static FORCE_RDX: AtomicU64 = AtomicU64::new(u64::MAX);
/// `u64::MAX` means none; anything else is an offset to read from `rcx` and log.
static PEEK_OFF: AtomicU64 = AtomicU64::new(u64::MAX);
/// Set between restoring the original byte and re-planting it one instruction later.
static WATCH_REARM: AtomicU64 = AtomicU64::new(0);
static CURRENT_OPCODE: AtomicU32 = AtomicU32::new(0);

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
const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
const EXCEPTION_BREAKPOINT: u32 = 0x8000_0003;
const EXCEPTION_SINGLE_STEP: u32 = 0x8000_0004;
/// Offsets into x64 CONTEXT.
const CTX_EFLAGS: usize = 0x44;
const CTX_RCX: usize = 0x80;
const CTX_RDX: usize = 0x88;
const CTX_RSP: usize = 0x98;
const CTX_R8: usize = 0xB8;
const CTX_R9: usize = 0xC0;
const CTX_RIP: usize = 0xF8;
/// EFLAGS.TF - single-step after the next instruction.
const TRAP_FLAG: u32 = 0x100;
/// Stop logging after this many hits, so a function on a per-frame path cannot fill the
/// disk while someone reads a dialog.
const WATCH_MAX_HITS: u32 = 32;

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

/// Which dispatched packet the walk should start on.
fn trigger() -> u64 {
    // `...#N` starts the walk on the Nth dispatched packet instead of the first.
    //
    // Timing turns out to matter as much as the address. The walk runs inside
    // FUN_1415e7090's startup loop, which is *before* the client has a login screen - so
    // a per-stage OnPacket handler does not exist yet and nothing can route to it. Waiting
    // for a later packet lets the client get where it is going first.
    let text = std::fs::read_to_string(PROBE_MARKER).unwrap_or_default();
    text.trim()
        .rsplit_once('#')
        .and_then(|(_, n)| n.trim().parse::<u64>().ok())
        .unwrap_or(1)
        .max(1)
}

/// Read `from-to[@targetVA]` out of the marker file, defaulting when it is empty.
///
/// The optional target generalises the walk. Without it the oracle is `conn+0x150`, which
/// only ever finds the one handler that releases the startup loop. With it, the oracle
/// becomes "which opcode called *this function*", which is how any other handler gets
/// identified when no readable stage switch covers it.
///
/// Content that is present but unparseable is reported rather than quietly replaced by
/// the default. A caller-side bug once wrote a file path in here and the silent fallback
/// made the walk look like it had honoured the range it was given.
fn range() -> (u32, u32, usize) {
    let raw = std::fs::read_to_string(PROBE_MARKER).unwrap_or_default();
    let text = raw.trim().split('#').next().unwrap_or("").trim().to_string();
    if text.is_empty() {
        return (DEFAULT_FROM, DEFAULT_TO, 0);
    }
    let (span, target) = match text.split_once('@') {
        Some((s, t)) => (
            s,
            usize::from_str_radix(t.trim().trim_start_matches("0x"), 16).unwrap_or(0),
        ),
        None => (text.as_str(), 0),
    };
    if let Some((a, b)) = span.split_once('-') {
        if let (Ok(a), Ok(b)) = (
            u32::from_str_radix(a.trim().trim_start_matches("0x"), 16),
            u32::from_str_radix(b.trim().trim_start_matches("0x"), 16),
        ) {
            return (a, b, target);
        }
    }
    log(&format!(
        "probe: marker says {text:?}, which is not <from>-<to>[@targetVA] in hex - \
         falling back to 0x{DEFAULT_FROM:04X}-0x{DEFAULT_TO:04X}"
    ));
    (DEFAULT_FROM, DEFAULT_TO, 0)
}

/// Plant an `int3` on `target` so a call to it traps into our vectored handler.
///
/// A breakpoint rather than an inline hook, for two reasons: it needs one byte, so no
/// assumptions about the target's prologue being relocatable; and the handler can make
/// the function return immediately instead of running, which matters when we are about to
/// invoke it with a body it was never meant to see.
unsafe fn arm_target(target: usize) -> bool {
    let mut old = 0u32;
    if VirtualProtect(target as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log(&format!("probe: could not make {target:#x} writable"));
        return false;
    }
    let p = target as *mut u8;
    TARGET_BYTE.store(*p as u32, Ordering::SeqCst);
    *p = 0xCC;
    VirtualProtect(target as *mut c_void, 1, old, &mut old);
    TARGET.store(target as u64, Ordering::SeqCst);
    log(&format!(
        "probe: armed int3 at {target:#x} (was {:#04x})",
        TARGET_BYTE.load(Ordering::SeqCst)
    ));
    true
}

/// Where the walk records how far it got, so a fatal opcode costs one launch, not the search.
pub const RESUME_FILE: &str = "maplecw-hook.resume";

/// Tag identifying which walk a resume point belongs to.
///
/// Without this, a resume point left over from one range would silently skip the start of
/// the next - the same class of bug as the marker fallback, where stale state makes a run
/// look like it covered ground it never touched.
fn resume_tag(from: u32, to: u32, target: usize) -> String {
    format!("{from:04X}-{to:04X}@{target:X}")
}

/// Read the resume point: the furthest opcode recorded for *this* walk.
///
/// Records are appended, so the file is a history rather than a single value and the last
/// matching line wins.
fn resume_from(from: u32, to: u32, target: usize) -> Option<u32> {
    let text = std::fs::read_to_string(RESUME_FILE).ok()?;
    let want = resume_tag(from, to, target);
    let mut best = None;
    let mut foreign = false;
    for line in text.lines() {
        match line.trim().split_once(':') {
            Some((tag, at)) if tag == want => {
                if let Ok(at) = u32::from_str_radix(at.trim(), 16) {
                    best = Some(best.map_or(at, |b: u32| b.max(at)));
                }
            }
            Some(_) => foreign = true,
            None => {}
        }
    }
    if best.is_none() && foreign {
        log("probe: resume file holds only points from a different walk - starting over");
    }
    best.filter(|at| *at > from && *at < to)
}

/// Append the next opcode to try.
///
/// Appending, not rewriting. `fs::write` truncates first, so a process that dies during
/// the write leaves an empty file - which is exactly what happened, and the next launch
/// restarted from zero and died in the same place. An append can lose the newest record
/// but never the ones before it.
fn record_resume(from: u32, to: u32, target: usize, at: u32) {
    use std::io::Write;
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(RESUME_FILE) {
        let _ = writeln!(f, "{}:{at:04X}", resume_tag(from, to, target));
    }
}

fn clear_resume() {
    let _ = std::fs::remove_file(RESUME_FILE);
}

/// Put the target's original byte back.
///
/// Must happen before the walk returns. The vectored handler only services the trap while
/// `IN_CALL` is set, so an `int3` left behind would fire on the client's own next call to
/// that function with nothing willing to handle it - turning a finished experiment into a
/// crash that looks unrelated.
unsafe fn disarm_target() {
    let target = TARGET.swap(0, Ordering::SeqCst) as usize;
    if target == 0 {
        return;
    }
    let mut old = 0u32;
    if VirtualProtect(target as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log(&format!("probe: could not restore {target:#x} - it still holds an int3"));
        return;
    }
    *(target as *mut u8) = TARGET_BYTE.load(Ordering::SeqCst) as u8;
    VirtualProtect(target as *mut c_void, 1, old, &mut old);
    log(&format!("probe: disarmed int3 at {target:#x}"));
}

/// Arm watch mode if the marker says `watch@<VA>`, and report the opcode being dispatched.
///
/// Separate from the walk: the walk asks "which opcode reaches X" by synthesising
/// thousands of packets, while this asks "did X run just now" for a packet we sent for
/// real. After several runs guessing at result codes, knowing whether the handler is
/// entered at all is the cheaper question.
pub unsafe fn note_opcode(opcode: u16) {
    CURRENT_OPCODE.store(opcode as u32, Ordering::SeqCst);
    let text = std::fs::read_to_string(PROBE_MARKER).unwrap_or_default();
    let Some(rest) = text.trim().strip_prefix("watch@") else {
        return;
    };
    if WATCH.load(Ordering::SeqCst) != 0 || WATCH_ARMED.swap(true, Ordering::SeqCst) {
        return;
    }
    // `watch@<VA>` or `watch@<VA>:rdx=<hex>`. The second form rewrites the second integer
    // argument on entry — for asking "what would the client do if this value were X",
    // which is the only question left when the *caller* is virtualised and unreadable.
    // `:peek=<hex off>` logs the byte and dword at `rcx + off` — for reading the field a
    // tiny accessor exists to return, which is usually the actual question.
    let (rest, peek) = match rest.split_once(":peek=") {
        Some((v, p)) => (v, Some(p)),
        None => (rest, None),
    };
    let (va_txt, force) = match rest.split_once(":rdx=") {
        Some((v, f)) => (v, Some(f)),
        None => (rest, None),
    };
    if let Some(p) = peek {
        match u64::from_str_radix(p.trim().trim_start_matches("0x"), 16) {
            Ok(v) => {
                PEEK_OFF.store(v, Ordering::SeqCst);
                log(&format!("probe: will log [rcx+{v:#x}] on every entry"));
            }
            Err(_) => {
                log(&format!("probe: {text:?} has an unparseable :peek= offset"));
                return;
            }
        }
    }
    let Ok(va) = usize::from_str_radix(va_txt.trim().trim_start_matches("0x"), 16) else {
        log(&format!("probe: watch marker {text:?} is not watch@<hex VA>[:rdx=<hex>]"));
        return;
    };
    if let Some(f) = force {
        let Ok(v) = u64::from_str_radix(f.trim().trim_start_matches("0x"), 16) else {
            log(&format!("probe: {text:?} has an unparseable :rdx= value"));
            return;
        };
        FORCE_RDX.store(v, Ordering::SeqCst);
        log(&format!("probe: will FORCE rdx={v:#x} on every entry to {va:#x}"));
    }
    AddVectoredExceptionHandler(1, veh as *const c_void);
    let mut old = 0u32;
    if VirtualProtect(va as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log(&format!("probe: could not arm watch at {va:#x}"));
        return;
    }
    WATCH_BYTE.store(*(va as *mut u8) as u32, Ordering::SeqCst);
    *(va as *mut u8) = 0xCC;
    VirtualProtect(va as *mut c_void, 1, old, &mut old);
    WATCH.store(va as u64, Ordering::SeqCst);
    log(&format!(
        "probe: watching {va:#x} - will report every entry with rcx/rdx/r8/r9 \
         (first {WATCH_MAX_HITS})"
    ));
}

pub fn enabled() -> bool {
    std::path::Path::new(PROBE_MARKER).exists()
}

/// Watch mode observes; it must never fall through into a walk.
///
/// `range()` cannot parse `watch@<VA>` and defaults to 0x0000-0x1000, so without this the
/// marker that means "just tell me if this function runs" would quietly start walking four
/// thousand opcodes instead - the same silent-fallback trap as before.
fn watching() -> bool {
    std::fs::read_to_string(PROBE_MARKER)
        .map(|t| t.trim().starts_with("watch@"))
        .unwrap_or(false)
}

static mut VIEW_SNAP: Vec<u8> = Vec::new();
static mut BUF_SNAP: Vec<u8> = Vec::new();

/// Snapshot the packet **before** the dispatcher touches it.
///
/// This has to happen first, not after. The dispatcher reads the opcode out of the view
/// itself, which advances the read cursor from 4 to 6; snapshotting afterwards captured
/// the *consumed* state, so every replayed call found the cursor already past the opcode
/// and read the zero padding instead. That produced a full clean pass over 4096 opcodes
/// with no faults and no effect - a null result that looked like an answer.
pub unsafe fn capture(view: *mut c_void, dispatch: u64) {
    if watching() || dispatch < trigger() || view.is_null() || CAPTURED.swap(true, Ordering::SeqCst) {
        return;
    }
    let data = *(view.cast::<u8>().add(VIEW_DATA).cast::<*mut u8>());
    let len = *(view.cast::<u8>().add(VIEW_LEN).cast::<u32>()) as usize;
    if data.is_null() || !(OPCODE_AT + 2..=0x10000).contains(&len) {
        return;
    }
    VIEW_SNAP = (0..VIEW_STRUCT).map(|i| *view.cast::<u8>().add(i)).collect();
    BUF_SNAP = (0..len).map(|i| *data.add(i)).collect();
}

/// The read cursor at the moment we snapshotted, for logging.
pub unsafe fn captured_cursor() -> i64 {
    let snap = &*std::ptr::addr_of!(VIEW_SNAP);
    if snap.len() < VIEW_CURSOR + 4 {
        return -1;
    }
    i64::from(u32::from_le_bytes([
        snap[VIEW_CURSOR],
        snap[VIEW_CURSOR + 1],
        snap[VIEW_CURSOR + 2],
        snap[VIEW_CURSOR + 3],
    ]))
}

/// Render `[reg]` as a `u32` when the register is a readable pointer, else nothing.
///
/// Half these functions take a pointer to the interesting value rather than the value:
/// `FUN_141804140(u32 *code, ...)` switches on `*code`, so the raw register is an address
/// and the number that names the dialog is one dereference away. Guarded, because this
/// runs inside a vectored handler where a faulting read would be fatal - and a register
/// holding a small integer rather than a pointer is the normal case, not an error.
unsafe fn deref(reg: u64) -> String {
    let addr = reg as usize;
    if !crate::session::can_read(addr, 4) {
        return String::new();
    }
    format!(" [{:#010x}]", *(addr as *const u32))
}

// Section bounds, read from the PE headers rather than estimated. The first version of
// this guessed a 64 MB image and silently filtered out *every* candidate, because the
// virtualised caller sits at RVA 0x4C05EB2 — about 80 MB in. An estimate that is too small
// does not report "nothing plausible"; it reports nothing at all.
/// `SizeOfImage`.
const IMAGE_SPAN: usize = 0x05DA_B000;
/// `.text`: real, decompilable code.
const TEXT: std::ops::Range<usize> = 0x0000_1000..0x0326_194A;
/// `.themida`: virtualised code, which cannot be decompiled — labelled, not chased.
const THEMIDA: std::ops::Range<usize> = 0x03D8_7000..0x0517_3000;

/// A poor man's backtrace: stack slots that look like return addresses into this module.
///
/// Needed because the immediate caller can be virtualised. `FUN_141b2a280` is invoked from
/// `.themida`, which is a dead end on its own — but the VM frame was itself entered from
/// somewhere, and that address is usually still sitting on the stack. Scanning a couple of
/// hundred bytes up finds it without needing to understand the VM's frame layout.
///
/// Every slot is checked with `VirtualQuery` first, and non-code values are skipped rather
/// than reported, so a stack full of data does not produce noise.
unsafe fn stack_trace(rsp: usize) -> String {
    let base = crate::hook::base();
    if base == 0 {
        return String::new();
    }
    let mut out = String::new();
    let mut found = 0;
    let mut off = 0;
    while off < 0x400 && found < 16 {
        let at = rsp + off;
        off += 8;
        if !crate::session::can_read(at, 8) {
            continue;
        }
        let v = *(at as *const u64) as usize;
        if v <= base || v >= base + IMAGE_SPAN {
            continue;
        }
        let rva = v - base;
        let tag = if TEXT.contains(&rva) {
            "<-TEXT" // the ones worth decompiling
        } else if THEMIDA.contains(&rva) {
            "(vm)"
        } else {
            "(?)"
        };
        out.push_str(&format!(" {v:#x}{tag}"));
        found += 1;
    }
    if out.is_empty() {
        out
    } else {
        format!("\n      stack:{out}")
    }
}

/// Render `*(wchar_t **)reg` as text when it looks like one, else nothing.
///
/// The notice display `FUN_141b4ac80(name, ...)` is handed a pointer to a string object
/// whose first field is the `wchar_t *` — so the *name of the dialog on screen* is two
/// dereferences from `rcx`. That is worth reading directly: it identifies the notice
/// regardless of how the caller chose it, which matters here because the name is evidently
/// not always a literal in the binary.
///
/// Conservative on purpose. Anything that is not a readable pointer to a run of printable
/// ASCII-range UTF-16 ending in NUL is reported as nothing rather than as garbage.
unsafe fn deref_wstr(reg: u64) -> String {
    if !crate::session::can_read(reg as usize, 8) {
        return String::new();
    }
    let p = *(reg as usize as *const usize);
    if !crate::session::can_read(p, 2) {
        return String::new();
    }
    let mut s = String::new();
    for i in 0..64 {
        let at = p + i * 2;
        if !crate::session::can_read(at, 2) {
            return String::new();
        }
        let c = *(at as *const u16);
        if c == 0 {
            break;
        }
        if !(0x20..0x7F).contains(&c) {
            return String::new(); // not the kind of string we are looking for
        }
        s.push(c as u8 as char);
    }
    if s.is_empty() {
        String::new()
    } else {
        format!(" \"{s}\"")
    }
}

/// Catch the fault and resume in the loop, rather than letting it unwind the client.
unsafe extern "system" fn veh(info: *mut ExceptionPointers) -> i32 {
    if info.is_null() {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let code = (*(*info).record).code;
    let at = (*(*info).record).address as usize;

    // Watch mode: answer "was this function entered, by which opcode, and with what
    // arguments". Deliberately outside the IN_CALL guard, because the whole point is to
    // observe the client's own dispatch of a packet we sent for real.
    //
    // The arguments are the useful half. "Did it run" was enough to confirm an opcode
    // reaches a handler, but the questions that follow are of the form "which *value* was
    // it called with" - e.g. which result code reaches FUN_141b267c0, where the code names
    // the dialog via a switch we can already read. RCX/RDX/R8/R9 are the first four
    // integer arguments under the Win64 ABI, read at entry before the prologue moves them.
    //
    // Repeating, not one-shot. A one-shot disarms on the first call, and the first call is
    // not always the interesting one - FUN_141b267c0 is also called with result 0 on
    // success, which would consume the single observation and report nothing useful.
    let watch = WATCH.load(Ordering::SeqCst) as usize;
    if code == EXCEPTION_BREAKPOINT && watch != 0 && at == watch {
        let ctx = (*info).context.cast::<u8>();
        let n = WATCH_HITS.fetch_add(1, Ordering::SeqCst) + 1;
        if n <= WATCH_MAX_HITS {
            let op = CURRENT_OPCODE.load(Ordering::SeqCst);
            let rcx = *(ctx.add(CTX_RCX).cast::<u64>());
            let rdx = *(ctx.add(CTX_RDX).cast::<u64>());
            let r8 = *(ctx.add(CTX_R8).cast::<u64>());
            let r9 = *(ctx.add(CTX_R9).cast::<u64>());
            // The breakpoint sits on the function's first byte, so the `call` that got
            // here has just pushed the return address and RSP points straight at it. That
            // names the *caller*, which is the whole question once a watch confirms the
            // callee runs - and it costs one read.
            let rsp = *(ctx.add(CTX_RSP).cast::<u64>()) as usize;
            let ret = if crate::session::can_read(rsp, 8) {
                format!(" called-from={:#x}", *(rsp as *const u64))
            } else {
                String::new()
            };
            let peek = match PEEK_OFF.load(Ordering::SeqCst) {
                u64::MAX => String::new(),
                off if crate::session::can_read(rcx as usize + off as usize, 4) => {
                    let at = rcx as usize + off as usize;
                    format!(
                        " [rcx+{off:#x}]=u8:{:#04x}/u32:{:#010x}",
                        *(at as *const u8),
                        *(at as *const u32)
                    )
                }
                off => format!(" [rcx+{off:#x}]=<unreadable>"),
            };
            log(&format!(
                "***** WATCH #{n}: {watch:#x} ENTERED while dispatching opcode 0x{op:04X} \
                 rcx={rcx:#x}{}{}{peek} rdx={rdx:#x} (as i32 {}){}{} r8={r8:#x} r9={r9:#x}\
                 {ret}{} *****",
                deref(rcx),
                deref_wstr(rcx),
                rdx as u32 as i32,
                deref(rdx),
                deref_wstr(rdx),
                stack_trace(rsp),
            ));
            if n == WATCH_MAX_HITS {
                log("probe: watch hit limit reached, further calls will not be logged");
            }
        }

        // Rewrite the argument if asked, after logging so the log records what the client
        // actually computed, not what we substituted.
        let force = FORCE_RDX.load(Ordering::SeqCst);
        if force != u64::MAX {
            *(ctx.add(CTX_RDX).cast::<u64>()) = force;
            if n <= WATCH_MAX_HITS {
                log(&format!("      forced rdx -> {force:#x}"));
            }
        }

        // Restore the byte and resume *at* the target so the real first instruction runs.
        let mut old = 0u32;
        if VirtualProtect(watch as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old) != 0 {
            *(watch as *mut u8) = WATCH_BYTE.load(Ordering::SeqCst) as u8;
            VirtualProtect(watch as *mut c_void, 1, old, &mut old);
        }
        *(ctx.add(CTX_RIP).cast::<u64>()) = watch as u64;

        // Past the cap, stop re-arming entirely instead of trapping forever in silence.
        // Some useful targets are per-frame accessors - `FUN_141b2a160` is eight bytes and
        // runs every frame - and an exception plus a single-step on each call would slow
        // the client to the point where the test itself is what breaks.
        if n >= WATCH_MAX_HITS {
            WATCH.store(0, Ordering::SeqCst);
            log("probe: watch disarmed after the hit limit");
            return EXCEPTION_CONTINUE_EXECUTION;
        }

        // Set the trap flag so we get a single-step exception immediately after that
        // instruction - that is where the int3 goes back in.
        *(ctx.add(CTX_EFLAGS).cast::<u32>()) |= TRAP_FLAG;
        WATCH_REARM.store(watch as u64, Ordering::SeqCst);
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // The step that follows a watch hit: put the breakpoint back.
    if code == EXCEPTION_SINGLE_STEP {
        let rearm = WATCH_REARM.swap(0, Ordering::SeqCst) as usize;
        if rearm != 0 {
            let mut old = 0u32;
            if VirtualProtect(rearm as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old) != 0 {
                *(rearm as *mut u8) = 0xCC;
                VirtualProtect(rearm as *mut c_void, 1, old, &mut old);
            }
            // TF clears itself on delivery, but clear it explicitly: leaving it set would
            // single-step the client through the rest of the function.
            let ctx = (*info).context.cast::<u8>();
            *(ctx.add(CTX_EFLAGS).cast::<u32>()) &= !TRAP_FLAG;
            return EXCEPTION_CONTINUE_EXECUTION;
        }
    }

    if !IN_CALL.load(Ordering::SeqCst) {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // Our own breakpoint on the target function: record the hit and make the call return
    // straight away, so the handler never runs on a body it was not meant to see.
    let target = TARGET.load(Ordering::SeqCst) as usize;
    if code == EXCEPTION_BREAKPOINT && target != 0 && at == target {
        HIT.store(true, Ordering::SeqCst);
        let ctx = (*info).context.cast::<u8>();
        // Simulate `ret`: the return address is on top of the stack at function entry.
        let rsp = *(ctx.add(CTX_RSP).cast::<u64>());
        let ret_addr = *(rsp as *const u64);
        *(ctx.add(CTX_RIP).cast::<u64>()) = ret_addr;
        *(ctx.add(CTX_RSP).cast::<u64>()) = rsp + 8;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // Leave other debugger traps alone; everything else is ours to swallow.
    if code == EXCEPTION_BREAKPOINT || code == 0x8000_0004 {
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
    dispatch: u64,
) {
    if watching() {
        return;
    }
    let want = trigger();
    if dispatch < want {
        log(&format!(
            "probe: dispatch #{dispatch}, waiting for #{want} before walking"
        ));
        return;
    }
    if conn.is_null() || view.is_null() || DONE.swap(true, Ordering::SeqCst) {
        return;
    }
    let (from, to, target) = range();

    let ntdll = GetModuleHandleA(c"ntdll.dll".as_ptr().cast());
    let restore = GetProcAddress(ntdll, c"RtlRestoreContext".as_ptr().cast());
    if restore.is_null() {
        log("probe: RtlRestoreContext not found - aborting");
        return;
    }
    // Refuse a walk whose oracle is already tripped. With no target the oracle is
    // conn+0x150, so if that byte is set before we start - which it is whenever we answer
    // 0x0032 first - the very first opcode would "hit" and the walk would confidently
    // name the wrong number. Aborting loudly beats answering wrongly.
    if target == 0 && *(conn.cast::<u8>().add(CONN_DONE_FLAG)) != 0 {
        log("probe: conn+0x150 is ALREADY set and no target was given, so every opcode \
             would look like a hit. Give a target (<from>-<to>@<VA>) or do not pre-answer \
             0x0032. Aborting.");
        return;
    }

    RESTORE_CTX.store(restore as u64, Ordering::SeqCst);
    AddVectoredExceptionHandler(1, veh as *const c_void);
    // Detouring kernel32 is not enough: the client exited anyway, so it left by one of
    // the lower doors. RtlExitUserProcess is what ExitProcess actually calls, and
    // NtTerminateProcess is the syscall underneath both. Thread exit goes through
    // NtTerminateThread, which is deliberately left alone.
    neutralise(c"ntdll.dll", c"RtlExitUserProcess");
    neutralise(c"ntdll.dll", c"NtTerminateProcess");
    if target != 0 && !arm_target(target) {
        log("probe: target could not be armed - aborting rather than reporting a false miss");
        return;
    }
    neutralise(c"kernel32.dll", c"ExitProcess");
    neutralise(c"kernel32.dll", c"TerminateProcess");

    let data = *(view.cast::<u8>().add(VIEW_DATA).cast::<*mut u8>());
    let view_snapshot = &*std::ptr::addr_of!(VIEW_SNAP);
    let buf_snapshot = &*std::ptr::addr_of!(BUF_SNAP);
    if data.is_null() || view_snapshot.len() < VIEW_STRUCT || buf_snapshot.len() < OPCODE_AT + 2 {
        log("probe: no pre-dispatch snapshot - aborting");
        return;
    }
    let len = buf_snapshot.len();
    let cursor_now = *(view.cast::<u8>().add(VIEW_CURSOR).cast::<u32>());

    log(&format!(
        "probe: walking 0x{from:04X}..0x{to:04X}, packet len={len} \
         cursor before dispatch={} after={cursor_now}, oracle={}",
        captured_cursor(),
        if target != 0 { "int3 on target" } else { "conn+0x150" }
    ));

    RUNNING.store(true, Ordering::SeqCst);
    let start = match resume_from(from, to, target) {
        Some(at) => {
            log(&format!(
                "probe: resuming at 0x{at:04X} - 0x{:04X} was fatal last run, skipping it",
                at - 1
            ));
            at
        }
        None => from,
    };
    CURRENT.store(start, Ordering::SeqCst);

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

        // Record the *next* opcode before running this one. If this call kills the
        // process despite the guards, the next launch resumes past it automatically -
        // which is the difference between one wasted run and restarting the search.
        record_resume(from, to, target, op + 1);
        if op.is_multiple_of(0x10) {
            log(&format!("probe: at 0x{op:04X} (faults so far {})", FAULTS.load(Ordering::Relaxed)));
        }

        FAULTED.store(false, Ordering::SeqCst);
        RtlCaptureContext(std::ptr::addr_of_mut!(SAVED) as *mut c_void);
        // Reached twice: once normally, and again if the VEH longjmps back here.
        if !FAULTED.load(Ordering::SeqCst) {
            IN_CALL.store(true, Ordering::SeqCst);
            let ret = tramp(conn, view);
            IN_CALL.store(false, Ordering::SeqCst);
            // Proof of work. If the dispatcher never really looks at the packet, the
            // cursor never moves and every call returns the same thing - which is exactly
            // what a silently-inert walk looks like, and is otherwise indistinguishable
            // from "swept the whole range, found nothing".
            let cursor_after = *(view.cast::<u8>().add(VIEW_CURSOR).cast::<u32>());
            if cursor_after != OPCODE_AT as u32 {
                CONSUMED.fetch_add(1, Ordering::Relaxed);
            }
            let last = LAST_RET.swap(ret, Ordering::Relaxed);
            if last != ret {
                RET_CHANGES.fetch_add(1, Ordering::Relaxed);
            }
        }

        let op = CURRENT.load(Ordering::SeqCst) - 1;
        if HIT.load(Ordering::SeqCst) {
            log(&format!(
                "***** FOUND IT: inbound opcode 0x{op:04X} reaches {target:#x} \
                 (faults along the way: {}) *****",
                FAULTS.load(Ordering::Relaxed)
            ));
            RUNNING.store(false, Ordering::SeqCst);
            disarm_target();
            clear_resume();
            return;
        }
        // Exactly one oracle is live per walk. When a target is armed, conn+0x150 is
        // *expected* to be set already - we answer 0x0032 to get past the gate before
        // walking - so consulting it here reports a hit on the first opcode every time.
        if target == 0 && *(conn.cast::<u8>().add(CONN_DONE_FLAG)) != 0 {
            log(&format!(
                "***** FOUND IT: inbound opcode 0x{op:04X} sets conn+0x150 \
                 (faults along the way: {}) *****",
                FAULTS.load(Ordering::Relaxed)
            ));
            RUNNING.store(false, Ordering::SeqCst);
            disarm_target();
            clear_resume();
            return;
        }
    }

    RUNNING.store(false, Ordering::SeqCst);
    disarm_target();
    clear_resume();
    let consumed = CONSUMED.load(Ordering::Relaxed);
    let total = to - from;
    log(&format!(
        "probe: finished 0x{from:04X}..0x{to:04X} with no hit          ({} faults, {consumed}/{total} calls advanced the cursor, {} distinct returns)",
        FAULTS.load(Ordering::Relaxed),
        RET_CHANGES.load(Ordering::Relaxed)
    ));
    if consumed == 0 {
        log("probe: WARNING - the cursor never moved on any call, so the dispatcher was \
             not reading our packet. A broken walk, not an empty range.");
    }
}
