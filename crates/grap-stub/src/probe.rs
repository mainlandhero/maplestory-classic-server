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
//!   Armed from `install()`, so it catches calls made before any packet is dispatched -
//!   including the client's own startup decisions.
//! * `watch@<VA>:rdx=<hex>` — watch, and **rewrite** the second integer argument on entry.
//!   For when the caller is virtualised and therefore unreadable: "who decided this" has no
//!   answer, but "what would the client do if this value were X" still does. The log
//!   records the original value first, so the run says what the client actually computed
//!   as well as what we substituted. This patches the client — describe results
//!   accordingly.
//!
//! A watch target takes any number of `:key=value` options: `rdx=`, `peek=<off>`,
//! `args=<n>`, and `hits=<n>`, the per-target log cap. Raise the cap on anything where the
//! *last* call is the interesting one - a thread-exit function in a client that recycles
//! threads would otherwise spend the default 32 early and disarm before the moment in
//! question, which reads exactly like a function that never ran. Every hit records the
//! calling thread id.
//!
//! `args=<n>` dumps integer arguments **5..=n** off the stack, since arguments 1-4 arrive
//! in RCX/RDX/R8/R9 and are logged already. It exists because a function can be entirely
//! readable and still have its deciding argument out of reach: `FUN_141d31b20`, the melee
//! target collector, returns before examining a single mob when its argument 17 is at
//! least its argument 4, and neither the register dump nor `stack_trace` - which filters
//! to values that look like code addresses, and so discards every small integer - could
//! see it.
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
/// How many functions may be watched at once.
///
/// One was not enough. Two questions needed answering in the same run and could not share
/// the slot: `watch@141b2a280:rdx=0` has to stay armed for the whole session or the login
/// dialog blocks the button that gets us to character select, and every *other* question
/// happens after that point. Rather than choose, watch several.
///
/// **Four was not enough either, for the same reason. Raised to six on 2026-08-20.** Two of
/// the four are permanently spoken for - `1415db360:ret` and `141b2a280:rdx=0` keep the
/// client alive - and a third should always be `140304100:hits=200`, the positive control
/// that separates "the client never called this" from "the hook never armed". That left
/// **one** slot for the actual question, so the mob-targeting run, which needs a cause and
/// its effect read side by side, could only be armed by dropping the control.
///
/// Trading away the positive control to fit a measurement is the exact trade this project
/// has lost before. A slot costs an `int3` and a single-step re-arm per hit, which is
/// cheap; a run that cannot tell a silent negative from an unarmed watch costs a launch.
const WATCH_SLOTS: usize = 6;
/// The most stack arguments `:args=` will dump. Sized to cover the deepest argument list
/// anyone has needed to read here - `FUN_141d31b20` takes seventeen - with room, but not so
/// much that a typo walks somebody else's frame.
const WATCH_ARGS_MAX: u32 = 32;
/// The most memory one `dump=` may copy. The EXP curve is 968 bytes; this is room for a few
/// tables of that size and not enough for a typo to write a megabyte of hex into the log.
const DUMP_MAX_BYTES: u32 = 8192;

#[allow(clippy::declare_interior_mutable_const)]
const WATCH_ZERO: AtomicU64 = AtomicU64::new(0);
#[allow(clippy::declare_interior_mutable_const)]
const WATCH_NONE: AtomicU64 = AtomicU64::new(u64::MAX);
#[allow(clippy::declare_interior_mutable_const)]
const WATCH_ZERO32: AtomicU32 = AtomicU32::new(0);
#[allow(clippy::declare_interior_mutable_const)]
const WATCH_CAP32: AtomicU32 = AtomicU32::new(WATCH_MAX_HITS);

static WATCH: [AtomicU64; WATCH_SLOTS] = [WATCH_ZERO; WATCH_SLOTS];
static WATCH_BYTE: [AtomicU32; WATCH_SLOTS] = [WATCH_ZERO32; WATCH_SLOTS];
static WATCH_HITS: [AtomicU32; WATCH_SLOTS] = [WATCH_ZERO32; WATCH_SLOTS];
/// Per-target hit cap, `WATCH_MAX_HITS` unless the spec says `:hits=<n>`.
///
/// The default exists so a per-frame accessor cannot fill the disk, but it is exactly
/// wrong for a target where the *last* call is the interesting one. A thread-exit function
/// in a client that recycles threads would spend the cap early and disarm itself before
/// the exit being investigated - a silent negative of the kind this project has already
/// paid for more than once.
static WATCH_LIMIT: [AtomicU32; WATCH_SLOTS] = [WATCH_CAP32; WATCH_SLOTS];
/// Per-target: return straight away instead of running the function.
///
/// For a body that cannot be read - `FUN_1415db360` hands twenty hardcoded server IPs to a
/// Themida-virtualised routine that overruns its own 512-byte stack buffer when none of
/// them answer, which is every run here because the client is firewalled. The body cannot
/// be fixed, so the call is skipped.
static WATCH_RET: [AtomicU32; WATCH_SLOTS] = [WATCH_ZERO32; WATCH_SLOTS];
/// `u64::MAX` means "do not force"; anything else is written to RDX on every watch hit.
static FORCE_RDX: [AtomicU64; WATCH_SLOTS] = [WATCH_NONE; WATCH_SLOTS];
/// `u64::MAX` means none; anything else is an offset to read from `rcx` and log.
static PEEK_OFF: [AtomicU64; WATCH_SLOTS] = [WATCH_NONE; WATCH_SLOTS];
/// Highest integer argument to dump off the stack, or 0 for none. See [`stack_arg_offset`].
///
/// Arguments 1-4 arrive in registers and are already logged, so this only ever means
/// "and also slots 5 through N".
static WATCH_ARGS: [AtomicU32; WATCH_SLOTS] = [WATCH_ZERO32; WATCH_SLOTS];
/// `dump=<VA>/<len>`: absolute address to dump on this watch's FIRST hit, or 0 for none.
///
/// **This exists because some of the client's data is not in the file at all.** The EXP
/// curve - 121 `u64`s at `0x143AC2400` - lives in the zero-initialised tail of `.data`, so
/// it has no bytes on disk and no static read can ever produce it; `tools/rtti.py` now
/// raises rather than returning the `.pdata` bytes that happen to sit at that file offset.
/// A running client has the real table, and one watch hit is enough to copy it out.
static DUMP_AT: [AtomicU64; WATCH_SLOTS] = [WATCH_ZERO; WATCH_SLOTS];
/// How many bytes [`DUMP_AT`] should copy. Capped by `DUMP_MAX_BYTES`.
static DUMP_LEN: [AtomicU32; WATCH_SLOTS] = [WATCH_ZERO32; WATCH_SLOTS];
/// Set once, however many slots are armed - the handler must not be registered twice.
static VEH_REGISTERED: AtomicBool = AtomicBool::new(false);
/// Faults reported to the log, capped so a repeating one cannot fill the disk.
static FAULT_LOGS: AtomicU32 = AtomicU32::new(0);
/// C++ throws seen, and how many were logged.
///
/// These are routine here - the packet decoders raise one on underflow - so logging every
/// one buries the line that matters. They are logged only inside a window near the
/// deadline; see [`THROW_LOG_AFTER_MS`].
static THROWS: AtomicU32 = AtomicU32::new(0);
static THROW_LOGS: AtomicU32 = AtomicU32::new(0);
/// `GetTickCount64` when the watches were armed, so the window can be measured.
static ARMED_AT_MS: AtomicU64 = AtomicU64::new(0);
/// Set between restoring the original byte and re-planting it one instruction later.
static WATCH_REARM: AtomicU64 = AtomicU64::new(0);
static CURRENT_OPCODE: AtomicU32 = AtomicU32::new(0);

/// x64 `CONTEXT` is 1232 bytes and must be 16-byte aligned.
#[repr(C, align(16))]
struct Context([u8; 1232]);

static mut SAVED: Context = Context([0; 1232]);

extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: *const c_void) -> *mut c_void;
    fn GetCurrentThreadId() -> u32;
    fn GetTickCount64() -> u64;
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
/// A C++ throw (`0xE06D7363`) is only logged once the process is this old.
///
/// The client `__fastfail`s on a fixed ~36.9s deadline from launch, and four of the five
/// sites that can raise it are ordinary CRT fatal paths - `abort` at reason 7, the
/// invalid-parameter handler at 5, `__report_gsfailure` at 2. An uncaught C++ exception
/// reaches `abort` through `terminate`, so the throw that kills the client happens shortly
/// before the deadline. Logging only this window keeps the routine decoder throws out of
/// the way while catching the one that matters.
const THROW_LOG_AFTER_MS: u64 = 25_000;
/// How many throws to log inside that window.
const THROW_LOG_MAX: u32 = 32;
/// How many throws to log **regardless of the clock**, so a short-lived run is not silent.
///
/// A run that dies before [`THROW_LOG_AFTER_MS`] used to record no throws at all, which
/// reads as "the client did not throw" and is not the same statement. Eight is enough to
/// catch the one that follows a packet we just sent - the case that matters here - without
/// reinstating the flood the window exists to prevent.
const THROW_LOG_EARLY_MAX: u32 = 8;
/// A C++ exception, as raised by `_CxxThrowException`.
const CPP_EXCEPTION: u32 = 0xE06D_7363;

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
    arm_watch();
}

/// Arm the watch, if the marker asks for one. Idempotent.
///
/// **Called from `install()`, not from the first dispatch.** It used to arm only when the
/// hook saw a packet, which made every run a race against the harness: if the gate packet
/// reached the client before the DLL's five-second install delay elapsed, the watch armed
/// seconds late and whatever it was meant to catch had already happened. That is exactly how
/// the login dialog came back twice - the suppression was arming after the client had
/// already decided to show it - and both times it looked like a change in the client rather
/// than a change in timing.
///
/// It also removes a real limitation: a call made *before* the first inbound packet used to
/// be invisible to watch mode by construction.
pub unsafe fn arm_watch() {
    let text = std::fs::read_to_string(PROBE_MARKER).unwrap_or_default();
    let Some(rest) = text.trim().strip_prefix("watch@") else {
        return;
    };
    if WATCH_ARMED.swap(true, Ordering::SeqCst) {
        return;
    }
    ARMED_AT_MS.store(GetTickCount64(), Ordering::SeqCst);
    for (slot, spec) in rest.split(',').filter(|s| !s.trim().is_empty()).enumerate() {
        if slot >= WATCH_SLOTS {
            log(&format!(
                "probe: {text:?} names more than {WATCH_SLOTS} targets - ignoring the rest"
            ));
            break;
        }
        arm_one(slot, spec.trim(), &text);
    }
}

/// Resolve one watch target.
///
/// Either a hex VA in the client's own image, or `<module>!<export>`. The symbolic form is
/// not a convenience: `ntdll` is relocated on every boot, so there is no VA to write down
/// for `RtlExitUserProcess`, and that is exactly the function worth watching when the
/// client ends its own process.
unsafe fn resolve_target(target: &str) -> Option<usize> {
    if let Some((module, export)) = target.split_once('!') {
        let m = GetModuleHandleA(format!("{module}\0").as_ptr().cast());
        if m.is_null() {
            log(&format!("probe: module {module:?} is not loaded"));
            return None;
        }
        let f = GetProcAddress(m, format!("{export}\0").as_ptr().cast());
        if f.is_null() {
            log(&format!("probe: {module}!{export} not found"));
            return None;
        }
        log(&format!("probe: {module}!{export} resolves to {:#x}", f as usize));
        return Some(f as usize);
    }
    usize::from_str_radix(target.trim_start_matches("0x"), 16).ok()
}

/// One watch spec, parsed.
///
/// Split out of `arm_one` so it can be tested without a client to plant breakpoints in.
/// A spec that parses wrong is not a compile error and not a crash - it is a run that
/// looks normal and measures the wrong thing, and every run costs a manual launch.
#[derive(Debug)]
struct WatchSpec<'a> {
    target: &'a str,
    force: Option<&'a str>,
    peek: Option<&'a str>,
    hits: Option<&'a str>,
    /// Highest integer argument to dump off the stack - see [`stack_args`].
    args: Option<&'a str>,
    /// `<VA>/<len>`: dump raw memory at an absolute address on this watch's first hit.
    dump: Option<&'a str>,
    /// Return from the function immediately instead of running it.
    ret: bool,
}

/// `<target>` followed by any number of `:key=value` options. Neither a hex VA nor
/// `<module>!<export>` contains a colon, so this splits cleanly.
///
///   * `rdx=<hex>` rewrites the second integer argument on entry - for asking "what would
///     the client do if this value were X", the only question left when the *caller* is
///     virtualised and unreadable;
///   * `peek=<hex>` logs the byte and dword at `rcx + off` - for reading the field a tiny
///     accessor exists to return, which is usually the actual question;
///   * `hits=<dec>` raises or lowers this target's log cap;
///   * `args=<dec>` also dumps integer arguments 5..=N off the stack, for a function whose
///     interesting argument is past the four the ABI puts in registers.
///
/// An unrecognised option is an error rather than something to skip, for the same reason
/// `-Session` rejects an unknown token: a run that looks fine and measures nothing costs
/// more than a run that refuses to start.
fn parse_watch_spec(spec: &str) -> Result<WatchSpec<'_>, String> {
    let mut fields = spec.split(':');
    let target = fields.next().unwrap_or("").trim();
    if target.is_empty() {
        return Err(format!("watch spec {spec:?} names no target"));
    }
    let mut parsed = WatchSpec {
        target,
        force: None,
        peek: None,
        hits: None,
        args: None,
        dump: None,
        ret: false,
    };
    for opt in fields {
        let opt = opt.trim();
        if let Some(v) = opt.strip_prefix("rdx=") {
            parsed.force = Some(v.trim());
        } else if let Some(v) = opt.strip_prefix("peek=") {
            parsed.peek = Some(v.trim());
        } else if let Some(v) = opt.strip_prefix("hits=") {
            parsed.hits = Some(v.trim());
        } else if let Some(v) = opt.strip_prefix("args=") {
            parsed.args = Some(v.trim());
        } else if let Some(v) = opt.strip_prefix("dump=") {
            parsed.dump = Some(v.trim());
        } else if opt == "ret" {
            parsed.ret = true;
        } else {
            return Err(format!(
                "watch spec {spec:?} has an unknown option {opt:?} - refusing to arm"
            ));
        }
    }
    Ok(parsed)
}

unsafe fn arm_one(slot: usize, spec: &str, text: &str) {
    let (target_txt, force, peek, hits, args, dump, want_ret) = match parse_watch_spec(spec) {
        Ok(p) => (p.target, p.force, p.peek, p.hits, p.args, p.dump, p.ret),
        Err(why) => {
            log(&format!("probe: {why}"));
            return;
        }
    };
    let Some(va) = resolve_target(target_txt) else {
        log(&format!(
            "probe: watch spec {spec:?} in {text:?} is not <hex VA> or <module>!<export>"
        ));
        return;
    };
    if want_ret {
        WATCH_RET[slot].store(1, Ordering::SeqCst);
        log(&format!(
            "probe: entries to {va:#x} will RETURN IMMEDIATELY - THIS IS A CLIENT PATCH"
        ));
    }
    if let Some(h) = hits {
        let Ok(v) = h.trim().parse::<u32>() else {
            log(&format!("probe: {spec:?} has an unparseable :hits= count"));
            return;
        };
        WATCH_LIMIT[slot].store(v, Ordering::SeqCst);
        log(&format!("probe: watch on {va:#x} will log up to {v} hits"));
    }
    if let Some(a) = args {
        let Ok(v) = a.trim().parse::<u32>() else {
            log(&format!("probe: {spec:?} has an unparseable :args= count"));
            return;
        };
        // Refuse rather than clamp, both ways. `args=4` would silently print the caller's
        // shadow slots and read like arguments; `args=400` would walk three kilobytes of
        // somebody else's frame on every hit. Either produces a log that looks like a
        // measurement, which is worse than a run that will not start.
        if !(5..=WATCH_ARGS_MAX).contains(&v) {
            log(&format!(
                "probe: {spec:?} asks for :args={v} - only 5..={WATCH_ARGS_MAX} means \
                 anything (1-4 are in rcx/rdx/r8/r9 and are already logged). Not arming."
            ));
            return;
        }
        WATCH_ARGS[slot].store(v, Ordering::SeqCst);
        log(&format!(
            "probe: will dump stack arguments 5..={v} on every entry to {va:#x}"
        ));
    }
    if let Some(d) = dump {
        // `<VA>/<len>`, both in hex and decimal respectively - the address is an address and
        // the length is a count, so they are written the way each is normally written.
        let (va_txt, len_txt) = match d.split_once('/') {
            Some(pair) => pair,
            None => {
                log(&format!("probe: {spec:?} dump= needs <VA>/<len>"));
                return;
            }
        };
        let at = u64::from_str_radix(va_txt.trim().trim_start_matches("0x"), 16);
        let len = len_txt.trim().parse::<u32>();
        match (at, len) {
            (Ok(a), Ok(n)) if a != 0 && n > 0 && n <= DUMP_MAX_BYTES => {
                DUMP_AT[slot].store(a, Ordering::SeqCst);
                DUMP_LEN[slot].store(n, Ordering::SeqCst);
                log(&format!(
                    "probe: will dump {n} bytes at {a:#x} on the FIRST entry to {va:#x}"
                ));
            }
            _ => {
                log(&format!(
                    "probe: {spec:?} dump= wants <hex VA>/<len 1..={DUMP_MAX_BYTES}>"
                ));
                return;
            }
        }
    }
    if let Some(p) = peek {
        match u64::from_str_radix(p.trim().trim_start_matches("0x"), 16) {
            Ok(v) => {
                PEEK_OFF[slot].store(v, Ordering::SeqCst);
                log(&format!("probe: will log [rcx+{v:#x}] on every entry to {va:#x}"));
            }
            Err(_) => {
                log(&format!("probe: {spec:?} has an unparseable :peek= offset"));
                return;
            }
        }
    }
    if let Some(f) = force {
        let Ok(v) = u64::from_str_radix(f.trim().trim_start_matches("0x"), 16) else {
            log(&format!("probe: {spec:?} has an unparseable :rdx= value"));
            return;
        };
        FORCE_RDX[slot].store(v, Ordering::SeqCst);
        log(&format!("probe: will FORCE rdx={v:#x} on every entry to {va:#x}"));
    }
    if !VEH_REGISTERED.swap(true, Ordering::SeqCst) {
        AddVectoredExceptionHandler(1, veh as *const c_void);
    }
    let mut old = 0u32;
    if VirtualProtect(va as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log(&format!("probe: could not arm watch at {va:#x}"));
        return;
    }
    WATCH_BYTE[slot].store(*(va as *mut u8) as u32, Ordering::SeqCst);
    *(va as *mut u8) = 0xCC;
    VirtualProtect(va as *mut c_void, 1, old, &mut old);
    // Read the byte back. A planted int3 that did not take reports nothing and looks
    // exactly like a function that never runs - the silent negative this repo keeps
    // getting caught by.
    if *(va as *mut u8) != 0xCC {
        log(&format!("probe: int3 at {va:#x} DID NOT TAKE - this watch is worthless"));
        return;
    }
    WATCH[slot].store(va as u64, Ordering::SeqCst);
    let cap = WATCH_LIMIT[slot].load(Ordering::SeqCst);
    log(&format!(
        "probe: watching {va:#x} (slot {slot}), int3 verified - will report every entry \
         with tid and rcx/rdx/r8/r9 (first {cap})"
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
/// Where integer argument `n` sits, relative to RSP **at an entry breakpoint**.
///
/// Win64 passes arguments 1-4 in RCX/RDX/R8/R9 and the rest on the stack. At the point of
/// the `call` the caller has already reserved 32 bytes of shadow space for those four, so
/// argument 5 is at `[rsp + 0x20]` and argument *n* at `[rsp + 0x20 + 8*(n-5)]`. The `call`
/// then pushes the return address, which moves everything down eight bytes - so inside the
/// callee, before its prologue runs, that collapses to **`[rsp + 8*n]`**.
///
/// A free function with a test rather than a comment on a magic number, because the
/// arithmetic is the whole measurement: read the wrong slot and the log prints a confident
/// number for the wrong argument, which is the failure mode `CLAUDE.md` describes
/// everywhere else. Off by one slot here would have reported argument 16 as argument 17.
///
/// Only meaningful for `n >= 5`; arguments 1-4 are in registers and never on the stack at
/// entry, so the shadow slots this would name for them hold whatever the caller left there.
fn stack_arg_offset(n: u32) -> usize {
    8 * n as usize
}

/// Dump integer arguments 5..=`highest` off the stack at an entry breakpoint.
///
/// Deliberately a **range, not a single index**. The question that motivated this - "does
/// `FUN_141d31b20` take the early-out at `141d31c96`, where argument 17 is compared against
/// argument 4" - reaches us through a decompiler's argument numbering, and a decompiler
/// numbering that disagrees with the ABI by one would be invisible in a single-slot read.
/// Printing every slot up to the one asked for means the answer is in the log whichever
/// index is right. Enumerate, then filter.
///
/// Rendered as i32 as well as raw, because these are read back with `movsxd ... dword` -
/// the high half is not part of the value.
unsafe fn stack_args(rsp: usize, highest: u32) -> String {
    if highest < 5 {
        return String::new();
    }
    let mut out = String::new();
    for n in 5..=highest {
        let at = rsp + stack_arg_offset(n);
        if crate::session::can_read(at, 8) {
            let v = *(at as *const u64);
            out.push_str(&format!(" a{n}={v:#x}(i32 {})", v as u32 as i32));
        } else {
            out.push_str(&format!(" a{n}=<unreadable>"));
        }
    }
    format!("\n      stack args (rsp+{:#x}..):{out}", stack_arg_offset(5))
}

pub(crate) unsafe fn stack_trace(rsp: usize) -> String {
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
    let hit_slot = (0..WATCH_SLOTS).find(|&i| {
        let va = WATCH[i].load(Ordering::SeqCst) as usize;
        va != 0 && va == at
    });
    if let (EXCEPTION_BREAKPOINT, Some(slot)) = (code, hit_slot) {
        let watch = at;
        let ctx = (*info).context.cast::<u8>();
        let n = WATCH_HITS[slot].fetch_add(1, Ordering::SeqCst) + 1;
        let cap = WATCH_LIMIT[slot].load(Ordering::SeqCst);
        if n <= cap {
            let op = CURRENT_OPCODE.load(Ordering::SeqCst);
            let rcx = *(ctx.add(CTX_RCX).cast::<u64>());
            let rdx = *(ctx.add(CTX_RDX).cast::<u64>());
            let r8 = *(ctx.add(CTX_R8).cast::<u64>());
            let r9 = *(ctx.add(CTX_R9).cast::<u64>());
            // The thread id matters as much as the arguments on any exit path:
            // "which thread ended, and in what order" is the whole question when
            // the surviving explanation is that the last thread simply ran out.
            let tid = GetCurrentThreadId();
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
            let peek = match PEEK_OFF[slot].load(Ordering::SeqCst) {
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
                "***** WATCH #{n}: {watch:#x} ENTERED on tid {tid} while dispatching \
                 opcode 0x{op:04X} rcx={rcx:#x}{}{}{peek} rdx={rdx:#x} \
                 (as i32 {}){}{} r8={r8:#x} r9={r9:#x} (as i32 {}){ret}{}{} *****",
                deref(rcx),
                deref_wstr(rcx),
                rdx as u32 as i32,
                deref(rdx),
                deref_wstr(rdx),
                r9 as u32 as i32,
                stack_args(rsp, WATCH_ARGS[slot].load(Ordering::SeqCst)),
                stack_trace(rsp),
            ));
            if n == cap {
                log("probe: watch hit limit reached, further calls will not be logged");
            }
        }

        // `dump=` - copy raw memory out, once, on this watch's first hit.
        //
        // On the FIRST hit rather than every hit because the target is a table, not a
        // variable: repeating it would bury the log and tell us nothing new. And on a hit
        // rather than at arming time because the interesting tables live in the
        // zero-initialised tail of `.data` - they contain nothing at all until the client
        // has run far enough to fill them in.
        if n == 1 {
            let at = DUMP_AT[slot].load(Ordering::SeqCst) as usize;
            let len = DUMP_LEN[slot].load(Ordering::SeqCst) as usize;
            if at != 0 && len != 0 {
                if crate::session::can_read(at, len) {
                    let bytes = std::slice::from_raw_parts(at as *const u8, len);
                    let mut hex = String::with_capacity(len * 2);
                    for b in bytes {
                        hex.push_str(&format!("{b:02x}"));
                    }
                    log(&format!("***** DUMP {len} bytes at {at:#x}: {hex} *****"));
                } else {
                    // Say which, rather than printing nothing: an address that is not mapped
                    // is a different fact from a table that is all zeroes, and only one of
                    // them means "look somewhere else".
                    log(&format!("***** DUMP {at:#x}+{len} is NOT READABLE - nothing copied *****"));
                }
            }
        }

        // Rewrite the argument if asked, after logging so the log records what the client
        // actually computed, not what we substituted.
        let force = FORCE_RDX[slot].load(Ordering::SeqCst);
        if force != u64::MAX {
            *(ctx.add(CTX_RDX).cast::<u64>()) = force;
            if n <= cap {
                log(&format!("      forced rdx -> {force:#x}"));
            }
        }

        // `:ret` - skip the function entirely. At the entry breakpoint the return address
        // is on top of the stack, so returning is RIP = [RSP], RSP += 8. Nothing needs to
        // be restored or re-armed: execution never resumes at the target, so the int3 can
        // stay planted for the next call.
        if WATCH_RET[slot].load(Ordering::SeqCst) == 1 {
            let rsp = *(ctx.add(CTX_RSP).cast::<u64>());
            if crate::session::can_read(rsp as usize, 8) {
                *(ctx.add(CTX_RIP).cast::<u64>()) = *(rsp as *const u64);
                *(ctx.add(CTX_RSP).cast::<u64>()) = rsp + 8;
                return EXCEPTION_CONTINUE_EXECUTION;
            }
            log("probe: :ret asked for, but the return address is unreadable - running it");
        }

        // Restore the byte and resume *at* the target so the real first instruction runs.
        let mut old = 0u32;
        if VirtualProtect(watch as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old) != 0 {
            *(watch as *mut u8) = WATCH_BYTE[slot].load(Ordering::SeqCst) as u8;
            VirtualProtect(watch as *mut c_void, 1, old, &mut old);
        }
        *(ctx.add(CTX_RIP).cast::<u64>()) = watch as u64;

        // Past the cap, stop re-arming entirely instead of trapping forever in silence.
        // Some useful targets are per-frame accessors - `FUN_141b2a160` is eight bytes and
        // runs every frame - and an exception plus a single-step on each call would slow
        // the client to the point where the test itself is what breaks.
        if n >= cap {
            WATCH[slot].store(0, Ordering::SeqCst);
            log(&format!("probe: watch on {watch:#x} disarmed after the hit limit"));
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
        // **A benign write fault the pool write watch armed is not a client crash.**
        //
        // `writewatch` protects bucket 1's pages read-only for under a second around a
        // predicted firing of the 180 s clock, so ordinary client writes into that memory
        // arrive here as `0xC000_0005`. Its own handler returns
        // `EXCEPTION_CONTINUE_EXECUTION`, and a first-chance handler registered later is
        // called first, so in practice this branch is never reached for one of them - but
        // vectored handler order is a property of registration, not a guarantee written
        // down anywhere, and getting it wrong here would write a 1.3 GB dump per write.
        if crate::writewatch::suppresses(info.cast::<c_void>()) {
            return EXCEPTION_CONTINUE_SEARCH;
        }
        // Not our walk, so this exception belongs to the client - but say so before
        // handing it back.
        //
        // A watch answers "did this function run". It cannot answer "the process died and
        // called nothing", which is exactly what happened when `RtlExitUserProcess` stayed
        // silent through a client exit with its int3 verified planted. A process that
        // leaves without calling an exit function has usually been killed by an exception,
        // and a vectored handler sees every one of those first.
        //
        // Only faults, and only a few: C++ throws (0xE06D7363) and the debugger traps
        // above are routine here, and logging them would bury the one line that matters.
        if code == CPP_EXCEPTION {
            let seen = THROWS.fetch_add(1, Ordering::SeqCst) + 1;
            let armed = ARMED_AT_MS.load(Ordering::SeqCst);
            let age = if armed == 0 {
                0
            } else {
                GetTickCount64().saturating_sub(armed)
            };
            // **Log the first few unconditionally, whatever the clock says.**
            //
            // The window alone is not enough, and that cost a diagnosis on 2026-08-20. The
            // shop crash killed the client 23.2 s after the watches armed; the window opens
            // at 25 s, so it opened 1.8 s AFTER the process was gone and the run recorded
            // **zero throws**. Read naively that says "the client did not throw" - the exact
            // opposite of the truth, and the previous run had caught the throw 10 ms after
            // the same packet. A silent negative produced by the instrument's own schedule.
            //
            // So: the first `THROW_LOG_EARLY_MAX` are always logged, which covers any run
            // that dies young, and the deadline window still catches the one that reaches
            // `terminate` in a run that lives long enough to `__fastfail`.
            let early = seen <= THROW_LOG_EARLY_MAX;
            if early || age >= THROW_LOG_AFTER_MS {
                let n = THROW_LOGS.fetch_add(1, Ordering::SeqCst) + 1;
                if early || n <= THROW_LOG_MAX {
                    let rsp = *((*info).context.cast::<u8>().add(CTX_RSP).cast::<u64>()) as usize;
                    log(&format!(
                        "***** C++ THROW #{seen} at {at:#x}{} on tid {} at +{age}ms{} *****",
                        crate::netwatch::module_of(at),
                        GetCurrentThreadId(),
                        stack_trace(rsp),
                    ));
                }
            }
            return EXCEPTION_CONTINUE_SEARCH;
        }
        // **`0xC000_0374` was missing from this list, and that cost a diagnosis.**
        // `STATUS_HEAP_CORRUPTION` is what killed the client on map 1013 and on map
        // 20001075, and because the code was not here the run produced *no* CLIENT FAULT
        // line at all - which was then read as "the client did not fault". That was a
        // property of this filter, not evidence about the client. `CLAUDE.md`: prove a
        // search can find a positive control before believing that it found nothing.
        //
        // It gets the stack the C++-throw branch above already builds, because a heap
        // corruption is raised by the allocator at the *next* walk rather than at the
        // moment of the damage: `at` will be inside ntdll and says nothing about who did
        // it. Only the `<-TEXT` frames name a client function. `at=` is exact; the frames
        // are a heuristic scan of the stack, so read them as leads.
        if matches!(
            code,
            0xC000_0005 | 0xC000_001D | 0xC000_0025 | 0xC000_008C | 0xC000_008E
                | 0xC000_0094 | 0xC000_00FD | 0xC000_0096 | 0xC000_0374
        ) {
            let n = FAULT_LOGS.fetch_add(1, Ordering::SeqCst) + 1;
            if n <= 8 {
                let rsp = *((*info).context.cast::<u8>().add(CTX_RSP).cast::<u64>()) as usize;
                log(&format!(
                    "***** CLIENT FAULT #{n}: code={code:#010x} at {at:#x}{} - the client \
                     raised this, we did not. An unhandled one ends the process without \
                     any call to ExitProcess. {} C++ throw(s) seen before this, {} logged. \
                     *****{}",
                    crate::netwatch::module_of(at),
                    // Counted even when not logged, so that "no THROW lines in this file"
                    // can be told apart from "the client did not throw". Those two were
                    // indistinguishable on 2026-08-20 and the difference was the diagnosis.
                    THROWS.load(Ordering::SeqCst),
                    THROW_LOGS.load(Ordering::SeqCst),
                    stack_trace(rsp)
                ));
            }
            // And take a full-memory dump, which is the only way any of this becomes
            // diagnosable.
            //
            // We are already standing on the exception - this handler is what wrote the
            // line above - and Windows Error Reporting demonstrably will not write one for
            // this client even with LocalDumps correctly armed. See `crate::minidump` for
            // the decoy experiment that established that without spending a client run.
            //
            // First-chance, so the dump is taken at the faulting instruction rather than
            // after the client has unwound. For `0xC000_0374` that is the difference
            // between a usable heap and none: the allocator raises it at the next walk, so
            // by the time anything else could look, the evidence is what is *in* the heap
            // rather than where the code is.
            crate::minidump::write_crash_dump(info.cast::<c_void>(), code);
        }
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
        "probe: finished 0x{from:04X}..0x{to:04X} with no hit ({} faults, \
         {consumed}/{total} calls advanced the cursor, {} distinct returns)",
        FAULTS.load(Ordering::Relaxed),
        RET_CHANGES.load(Ordering::Relaxed)
    ));
    if consumed == 0 {
        log("probe: WARNING - the cursor never moved on any call, so the dispatcher was \
             not reading our packet. A broken walk, not an empty range.");
    }
}


#[cfg(test)]
mod tests {
    use super::parse_watch_spec;

    #[test]
    fn a_bare_target_carries_no_options() {
        let s = parse_watch_spec("141b2a280").expect("a bare VA is a valid spec");
        assert_eq!(s.target, "141b2a280");
        assert!(s.force.is_none() && s.peek.is_none() && s.hits.is_none());
    }

    #[test]
    fn a_module_export_target_survives_the_colon_split() {
        // The whole reason the split is on ':' and not on '!': ntdll is relocated every
        // boot, so the thread-exit watches have no VA to write down and must be named.
        let s = parse_watch_spec("ntdll!RtlExitUserThread:hits=200").expect("valid");
        assert_eq!(s.target, "ntdll!RtlExitUserThread");
        assert_eq!(s.hits, Some("200"));
    }

    #[test]
    fn options_are_read_in_any_order() {
        let a = parse_watch_spec("141b36a10:peek=1c0:rdx=0:hits=5").expect("valid");
        let b = parse_watch_spec("141b36a10:hits=5:rdx=0:peek=1c0").expect("valid");
        for s in [a, b] {
            assert_eq!(s.target, "141b36a10");
            assert_eq!(s.peek, Some("1c0"));
            assert_eq!(s.force, Some("0"));
            assert_eq!(s.hits, Some("5"));
        }
    }

    #[test]
    fn the_specs_test_charselect_actually_passes_all_parse() {
        for spec in [
            "141b2a280:rdx=0",
            "ntdll!RtlExitUserThread:hits=200",
            "ntdll!NtTerminateThread:hits=200",
        ] {
            assert!(parse_watch_spec(spec).is_ok(), "{spec} must parse");
        }
    }

    #[test]
    fn ret_is_a_bare_flag_and_mixes_with_the_others() {
        let s = parse_watch_spec("1415db360:ret").expect("valid");
        assert_eq!(s.target, "1415db360");
        assert!(s.ret);
        let s = parse_watch_spec("1415db360:ret:hits=40").expect("valid");
        assert!(s.ret);
        assert_eq!(s.hits, Some("40"));
        assert!(!parse_watch_spec("1415db360").expect("valid").ret);
    }

    /// Every probe string `tools/test-server.ps1` can install, parsed the way `arm_watch`
    /// parses it.
    ///
    /// Two failures this catches, both of which cost a whole manual launch and both of
    /// which look like a normal run:
    ///
    /// * a spec that does not parse is logged and skipped, so the watch is simply absent
    ///   and reports a clean zero;
    /// * a **fifth** target is dropped with "ignoring the rest" - and in every one of these
    ///   strings the last target is `140304100`, the positive control. Losing it turns
    ///   "the client never called this" and "the hook never armed" back into the same
    ///   observation, which is the distinction the control exists to make.
    #[test]
    fn every_launcher_probe_string_arms_every_slot_it_names() {
        for text in [
            // the bare default, no -SetFieldProbe
            "watch@1415db360:ret,141b2a280:rdx=0,141b36f60,142ef3e44:hits=8",
            // -SetFieldProbe, default pair: mob spawn
            "watch@1415db360:ret,141b2a280:rdx=0,141c532ab:peek=24:hits=20,140304100:hits=200:dump=143AC2400/968",
            // -SetFieldProbe -InventorySlots N
            "watch@1415db360:ret,141b2a280:rdx=0,140305e48:peek=24:hits=20,140304100:hits=200:dump=143AC2400/968",
            // -SetFieldProbe -UserState
            "watch@1415db360:ret,141b2a280:rdx=0,140f810e0:hits=60,140304100:hits=200:dump=143AC2400/968",
            // -SetFieldProbe -MobTargets: five targets, which is why WATCH_SLOTS is six.
            // Kept on ONE line deliberately. Written with a `\` continuation it silently
            // retained the leading whitespace of the next line and produced a target that
            // parses and can never resolve - caught only by the whitespace assertion below.
            "watch@1415db360:ret,141b2a280:rdx=0,141d32675:peek=0xa88:hits=40,141d3267c:peek=0x42c:hits=40,140304100:hits=200:dump=143AC2400/968",
        ] {
            let rest = text.strip_prefix("watch@").expect("every string is a watch");
            let specs: Vec<&str> = rest.split(',').filter(|s| !s.trim().is_empty()).collect();
            assert!(
                specs.len() <= super::WATCH_SLOTS,
                "{text:?} names {} targets; only {} are armed and the rest are \
                 silently dropped",
                specs.len(),
                super::WATCH_SLOTS
            );
            for spec in specs {
                // No whitespace anywhere. `parse_watch_spec` only trims, so a spec carrying
                // an embedded space still "parses" and then fails to resolve at launch -
                // and the one string here written with a Rust `\` line continuation is
                // exactly where that would creep in.
                assert!(
                    !spec.contains(char::is_whitespace),
                    "{spec:?} in {text:?} carries whitespace; it will not resolve"
                );
                assert!(parse_watch_spec(spec).is_ok(), "{spec:?} in {text:?} must parse");
            }
        }
    }

    /// The negative control for the test above: a string with the faults it looks for must
    /// actually trip it.
    ///
    /// Without this, `every_launcher_probe_string_arms_all_four_slots` could pass because
    /// the strings are fine or because the check is incapable of failing, and those two
    /// look identical from the outside. That is the exact shape of the test summary in
    /// `CLAUDE.md` that was structurally unable to report a failure.
    #[test]
    fn the_four_slot_check_can_fail() {
        // One target per slot, plus one. Built from WATCH_SLOTS rather than written out,
        // so raising the slot count cannot quietly turn this control into a no-op - which
        // is exactly what happened when it went from four to six.
        let mut over_full = String::from("watch@1415db360:ret");
        for _ in 0..super::WATCH_SLOTS {
            over_full.push_str(",140304100:hits=1");
        }
        let rest = over_full.strip_prefix("watch@").expect("a watch");
        let specs: Vec<&str> = rest.split(',').filter(|s| !s.trim().is_empty()).collect();
        assert!(
            specs.len() > super::WATCH_SLOTS,
            "the over-full string must be over full, or the check above proves nothing"
        );
        // And an unparseable member is caught rather than skipped.
        assert!(parse_watch_spec("141d31b20:args").is_err());
        assert!(parse_watch_spec("141d31b20:rip=0").is_err());
    }

    #[test]
    fn dump_parses_and_wants_an_address_and_a_length() {
        let s = parse_watch_spec("140304100:hits=200:dump=143AC2400/968").expect("valid");
        assert_eq!(s.dump, Some("143AC2400/968"));
        assert_eq!(s.hits, Some("200"));
        // Order must not matter, and it must coexist with the other options.
        let s = parse_watch_spec("140304100:dump=143AC2400/968:peek=1c0").expect("valid");
        assert_eq!(s.dump, Some("143AC2400/968"));
        assert_eq!(s.peek, Some("1c0"));
        assert!(parse_watch_spec("140304100").expect("valid").dump.is_none());
        // A bare `dump` with no value is not an option and must be refused, like any other
        // malformed spec - a watch that arms and measures nothing costs a whole launch.
        assert!(parse_watch_spec("140304100:dump").is_err());
    }

    #[test]
    fn a_stack_argument_lands_where_the_abi_puts_it() {
        // The whole value of `:args=` is this arithmetic, so it is pinned rather than
        // commented. At an entry breakpoint the return address is at [rsp], the four
        // shadow slots follow it, and argument 5 is the first real stack argument.
        assert_eq!(super::stack_arg_offset(5), 0x28);
        assert_eq!(super::stack_arg_offset(6), 0x30);
        // The one this was built for: FUN_141d31b20's argument 17, compared against
        // argument 4 at 141d31c96 to decide whether any mob is examined at all.
        assert_eq!(super::stack_arg_offset(17), 0x88);
        // Every slot is eight bytes and none is skipped.
        for n in 5..super::WATCH_ARGS_MAX {
            assert_eq!(
                super::stack_arg_offset(n + 1) - super::stack_arg_offset(n),
                8,
                "argument {n} to {} must be one slot apart",
                n + 1
            );
        }
    }

    #[test]
    fn args_parses_and_mixes_with_the_others() {
        let s = parse_watch_spec("141d31b20:args=17:hits=8").expect("valid");
        assert_eq!(s.target, "141d31b20");
        assert_eq!(s.args, Some("17"));
        assert_eq!(s.hits, Some("8"));
        // Order must not matter - a spec is written by hand at a launch prompt.
        let s = parse_watch_spec("141d31b20:hits=8:args=17").expect("valid");
        assert_eq!(s.args, Some("17"));
        assert!(parse_watch_spec("141d31b20").expect("valid").args.is_none());
    }

    #[test]
    fn an_unknown_option_is_refused_rather_than_ignored() {
        // Skipping it would arm the watch and measure something other than what was
        // asked for, which is the failure this project keeps paying for.
        let err = parse_watch_spec("141b2a280:rcx=0").expect_err("rcx= is not an option");
        assert!(err.contains("unknown option"), "{err}");
        assert!(parse_watch_spec("141b2a280:hits").is_err());
    }
}
