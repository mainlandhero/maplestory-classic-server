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
/// `PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY`.
/// Deliberately **not** a superset of [`READABLE`]: `PAGE_READONLY` and `PAGE_EXECUTE_READ`
/// are readable and not writable, and a store into one of those raises an access violation
/// rather than failing quietly.
const WRITABLE: u32 = 0x04 | 0x08 | 0x40 | 0x80;
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

/// The whole marker as written, for a module that has to report **why** it did not arm.
///
/// A module that reads a token it cannot find and returns silently is indistinguishable from
/// a module that is not in the build. On 2026-09-08 that cost an overnight run: `-GuardPage`
/// never reached the launcher, `guardpage::install` found no token and returned without a
/// word, and the log looked exactly like a healthy run with no guard page compiled in.
pub(crate) fn marker_raw() -> Option<String> {
    mode()
}

/// One comma-separated option out of the marker, e.g. `mode=2,create=on`.
///
/// The marker used to hold exactly one setting. It now carries more than one, and splitting
/// on commas rather than matching the whole string means an option that is not understood
/// is ignored instead of silently disabling the one beside it.
pub(crate) fn marker_token(prefix: &str) -> Option<String> {
    mode()?
        .split(',')
        .map(str::trim)
        .find_map(|t| t.strip_prefix(prefix).map(str::to_string))
}

pub fn enabled() -> bool {
    std::path::Path::new(SESSION_MARKER).exists()
}

/// Guarded read for other modules — notably the probe, which dereferences argument
/// registers inside a vectored handler where a fault would be fatal.
pub(crate) unsafe fn can_read(addr: usize, len: usize) -> bool {
    readable(addr, len)
}

/// Is `len` bytes at `addr` committed and **writable**?
///
/// Same shape as [`readable`] and deliberately a separate mask: the sentry's repair mode is
/// the only thing in this crate that writes to memory the client owns, and a store into a
/// `PAGE_READONLY` region would raise an access violation inside the client rather than
/// returning an error. So the protection is checked before the store, not inferred from the
/// fact that the same address was readable a moment earlier.
pub(crate) unsafe fn can_write(addr: usize, len: usize) -> bool {
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
    if mbi.state != MEM_COMMIT || mbi.protect & PAGE_GUARD != 0 || mbi.protect & WRITABLE == 0 {
        return false;
    }
    addr + len <= mbi.base as usize + mbi.region_size
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
    let Some(hex) = marker_token("mode=") else {
        return;
    };
    let Ok(want) = u32::from_str_radix(hex.trim().trim_start_matches("0x"), 16) else {
        log(&format!("session: mode={hex:?} is not mode=<hex>"));
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

/// `FUN_140c9e230`, the setter that stores plaintext `1` into the protected flag guarding
/// the "Create a character" button.
const CREATE_FLAG_ENABLE_RVA: usize = 0x140C9E230 - 0x140000000;
/// The login result. The handshake has certainly finished by the time one is dispatched.
const LOGIN_RESULT_OPCODE: u16 = 0x0010;

static CREATE_ENABLED: AtomicBool = AtomicBool::new(false);

/// Turn the "Create a character" button back on.
///
/// # What this is
///
/// `FUN_141177a10` only calls the create handler when `FUN_140c9e3f0()` is non-zero, and a
/// watch measured that call returning zero on every click - the handler `FUN_141b282d0` was
/// never entered. The flag behind it is a protected byte (value plus rolling checksum) with
/// exactly two setters, identical but for one instruction:
///
/// * `FUN_140c9e8a0` stores plaintext `0`, and the **handshake calls it on success**;
/// * `FUN_140c9e230` stores plaintext `1`, and has no caller in `.text` and no vtable
///   entry, so it is reached only from the Themida VM.
///
/// So the real service enables this from virtualised code we cannot read, driven by
/// something we do not yet send. Calling the enable setter directly says whether that flag
/// is the *only* thing in the way - and if it is, the whole creation flow becomes
/// measurable, including the create request, which is virtualised and can only be measured.
///
/// **This patches the client.** It does not make the session valid, and it is not how a
/// real server would do it. Say so when reporting any result that depends on it.
///
/// # Why after the login result
///
/// The handshake sets the flag to zero, so anything earlier is overwritten. `0x0010` is the
/// first dispatch that is certainly after it, and it is also the packet that builds the
/// screen the button lives on.
pub unsafe fn enable_character_creation_after_dispatch(opcode: u16) {
    if opcode != LOGIN_RESULT_OPCODE || marker_token("create=").as_deref() != Some("on") {
        return;
    }
    // **Every login result, not just the first.** This used to latch on `CREATE_ENABLED`
    // and fire once per launch, which made "Create a character" work on the first login of
    // a launch and silently stop working on every later one.
    //
    // The owner, 2026-08-21: *"when I log in, I have full character slots, if I delete one, I
    // cannot immediately create another to replace it."* The delete was incidental. What
    // that session actually did was **log in twice** - enter the world on connection #1,
    // come back to character select on connection #2 - and the handshake calls
    // `FUN_140c9e8a0`, which stores plaintext **0** into this same flag, on every success.
    // So connection #2 zeroed it and the latch stopped us putting it back.
    //
    // Measured: `login.log` shows two connections and two `0x0010`s; the hook log shows
    // **one** "called FUN_140c9e230" line, at the first. And the symptom is exactly what a
    // cleared flag predicts - the button draws enabled, because that is separate state, and
    // `FUN_141177a10` never calls `FUN_141b282d0`, so **no packet is sent at all**. Not a
    // refusal notice, not an `0x00A8`: silence.
    //
    // The setter stores a plaintext 1 with a rolling checksum and takes no arguments, so
    // calling it again is idempotent. Only the log line is rationed.
    let first = !CREATE_ENABLED.swap(true, Ordering::SeqCst);
    let at = crate::hook::base() + CREATE_FLAG_ENABLE_RVA;
    let enable: extern "system" fn() = std::mem::transmute(at);
    enable();
    if first {
        log(&format!(
            "***** SESSION called FUN_140c9e230 at {at:#x} - the create-character flag should \
             now read 1. THIS IS A CLIENT PATCH: the real service sets it from virtualised \
             code, so this proves the button's gate, not the protocol *****"
        ));
    } else {
        // Kept, and deliberately not silent: a second login is exactly the case that used
        // to break, so a run that reaches character select twice should say so in the log.
        log("***** SESSION re-armed the create-character flag on a later login result - the \
             handshake zeroes it on every success, so this has to run per login, not once \
             per launch *****");
    }
}

/// The character-select UI singleton, `0x143aca790` - the object whose three slots hold
/// the avatars on the select screen. Every reader resolves to this one address
/// (`research/charselect-avatar-fade-race.md` §1); its constructor `FUN_141177490` stores it.
const SELECT_UI_PTR_RVA: usize = 0x143ACA790 - 0x140000000;
/// `FUN_141177e40(selectUi)` - refill the three slots from the decoded character list, then
/// `[vtable+0x90](obj, 0)`. It is what the **mode-5** login-result handler calls at
/// `141b33ea3` after decoding the list, and what the **mode-2** body never calls.
const SELECT_REFRESH_RVA: usize = 0x141177E40 - 0x140000000;
/// Its prologue, read back before the call so a different binary refuses instead of jumping
/// into the wrong function: `mov [rsp+8],rbx; push rdi; sub rsp,0x20; mov rdi,rcx; xor ebx,ebx`.
const SELECT_REFRESH_PROLOGUE: [u8; 15] =
    [0x48, 0x89, 0x5c, 0x24, 0x08, 0x57, 0x48, 0x83, 0xec, 0x20, 0x48, 0x8b, 0xf9, 0x33, 0xdb];
/// The select UI object is `FUN_14019b780(pool, 0x5f0)` bytes (`141b28033 mov edx,0x5f0`).
const SELECT_UI_SIZE: usize = 0x5f0;

/// Refill the character-select slots after a login result - the call the mode-2 handler
/// is missing. Off with the session token `selectfill=off`; absent means on.
///
/// # THIS IS LOAD-BEARING. Do not remove it, gate it, or "simplify" it away.
///
/// **CONFIRMED on screen 2026-09-12 09:23**: four consecutive logins, every one the blank
/// ordering (select UI built 30 ms after the login request, before the list), every one
/// rescued - `SELECTFILL: called` after each `0x0010`, three fills carrying real records
/// (`r8=0x14e0f8 r9=0xfffb`) where the build had filled empties (`r8=0x20 r9=0x140331540`),
/// and the owner: *"it seems consistently fixed now."* Fixture:
/// `research/fixtures/selectfill-rescued-early-build-avatars-drew-hook.log`. Before it,
/// five of five blank logins were measured as this exact ordering, and no server-side
/// timing can reach it (the client dispatches nothing while it builds the empty screen).
/// The whole history - three wrong theories and the measurements that killed each - is
/// `research/charselect-avatar-fade-race.md`; do not re-derive it from the symptom.
///
/// Things that look like cleanups and are not:
/// * Removing this because "the login server re-sends the list": the re-send is not the
///   mechanism (it was measured not to help; mode 2 never refills on any `0x0010`).
/// * Gating it on the `0x0010` being the first, or on the mode: it must run on every
///   `0x0010` on which the object already exists, and it correctly does nothing otherwise.
/// * Adding a `-Probe` watch on `141177e40` "to see it fire": the watch's int3 is the byte
///   this guard reads. It tolerates that now, but the `SELECTFILL:` line already says so.
/// * Moving it before `enable_character_creation_after_dispatch`: harmless, but pointless -
///   keep the three after-dispatch steps together in `hook.rs`, and keep the wiring test
///   below green.
///
/// # Why
///
/// The owner, 2026-09-12: *"I relaunched 3 times, the first 2 launches drew the avatar at
/// character select just fine, but the third launch drew blank avatars."* All three were
/// instrumented (`research/charselect-avatar-fade-race.md` §6-§8). The select UI is built
/// **once**, by the fade-deadline populator, and its build method `FUN_141177790` fills the
/// three slots from whatever character list exists at that instant. On the two good logins
/// the build ran ~490 ms after the login request, after `0x0010` had been decoded. On the
/// blank one it ran **30 ms** after the login request, from inside the still-running `0x0032`
/// dispatch - before the world list had even been dispatched - so it filled from an empty
/// list, and when `0x0010` arrived 370 ms later nothing refilled it: the mode-2 handler
/// `FUN_141b307b0` decodes the list and calls nothing in the fill chain. The placement loop
/// then skips every slot whose character pointer is null, which is a select screen with
/// frames, a statboard, and no avatars.
///
/// No server timing can fix that ordering - the client dispatched nothing for 555 ms while
/// it built the empty screen. The client's own answer is this call: the mode-5 handler
/// makes it after every decode. So this makes it after every `0x0010` dispatch, **only when
/// the object already exists**. When it does not (the good ordering), the build that comes
/// later fills from the decoded list exactly as it did on the two good logins, and this does
/// nothing.
///
/// **This patches the client.** It calls a client function the live handler would not have
/// called. It does not make the session valid; say so when reporting a result that depends
/// on it.
pub unsafe fn refresh_select_after_dispatch(opcode: u16) {
    if opcode != LOGIN_RESULT_OPCODE || marker_token("selectfill=").as_deref() == Some("off") {
        return;
    }
    let base = crate::hook::base();
    let slot = base + SELECT_UI_PTR_RVA;
    if !can_read(slot, 8) {
        log(&format!("***** SELECTFILL: cannot read the select-UI pointer at {slot:#x}; skipped *****"));
        return;
    }
    let obj = std::ptr::read_unaligned(slot as *const usize);
    if obj == 0 {
        // The good ordering: the list decoded before the screen was built, so the build
        // that follows fills from it. Nothing to do, and worth one line, because it is the
        // measurement that tells a good login from a rescued one.
        log("***** SELECTFILL: the select UI is not built yet after this 0x0010 - the build \
             will fill from the decoded list; nothing called *****");
        return;
    }
    if !can_read(obj, SELECT_UI_SIZE) {
        log(&format!("***** SELECTFILL: select UI {obj:#x} is not readable; refusing to call into it *****"));
        return;
    }
    let at = base + SELECT_REFRESH_RVA;
    if !can_read(at, SELECT_REFRESH_PROLOGUE.len()) {
        log(&format!("***** SELECTFILL: {at:#x} is not readable; skipped *****"));
        return;
    }
    let found = std::slice::from_raw_parts(at as *const u8, SELECT_REFRESH_PROLOGUE.len());
    // **A `-Probe` watch on this very function plants `0xCC` over its first byte**, and on
    // 2026-09-12 (09:16) that is exactly what was there: the default probe watched 141177e40
    // to measure the fill, so the instrument defeated the fix on both blank logins - the
    // guard read [cc, 89, 5c, ...] and refused. A planted int3 is not a different binary:
    // the probe's handler restores the byte and continues for any caller, this one included.
    // So byte 0 may be 0xCC, the other fourteen must match, and the log says which it was.
    let watched = found[0] == 0xCC;
    if found[1..] != SELECT_REFRESH_PROLOGUE[1..] || !(watched || found[0] == SELECT_REFRESH_PROLOGUE[0]) {
        log(&format!(
            "***** SELECTFILL: refusing to call {at:#x} - expected {SELECT_REFRESH_PROLOGUE:02x?} \
             (FUN_141177e40's prologue), found {found:02x?}. Different binary? *****"
        ));
        return;
    }
    let refresh: extern "system" fn(usize) = std::mem::transmute(at);
    refresh(obj);
    log(&format!(
        "***** SELECTFILL: called FUN_141177e40({obj:#x}) after 0x0010{} - the select UI existed \
         before the list was decoded, so its three slots were built EMPTY; they are now \
         refilled from the decoded list. THIS IS A CLIENT PATCH: it is the call the mode-5 \
         login handler makes and the mode-2 handler does not (research/charselect-avatar-fade-race.md sec 8) *****",
        if watched { " (through a -Probe int3 planted on it)" } else { "" }
    ));
}

#[cfg(test)]
mod selectfill_tests {
    use super::*;

    /// The RVAs are derived from the VAs rather than typed.
    #[test]
    fn the_rvas_are_the_vas_less_the_image_base() {
        assert_eq!(SELECT_UI_PTR_RVA, 0x3aca790);
        assert_eq!(SELECT_REFRESH_RVA, 0x1177e40);
    }

    /// **Built is not wired** is this project's oldest silent failure (CLAUDE.md). The step
    /// only exists if the dispatch hook calls it, so this reads `hook.rs` and refuses to let
    /// the call be dropped without a test going red. If you are here because you removed
    /// it: read the doc block on `refresh_select_after_dispatch` first.
    #[test]
    fn the_refill_step_is_wired_into_the_dispatch_hook() {
        let hook = include_str!("hook.rs");
        assert!(
            hook.contains("crate::session::refresh_select_after_dispatch(opcode);"),
            "hook.rs no longer calls refresh_select_after_dispatch after the handler - the \
             blank character-select screen comes back on every fast start"
        );
        // And it stays behind the two steps it depends on nothing from but sits beside.
        let order = |needle: &str| hook.find(needle).expect(needle);
        assert!(order("patch_mode_after_dispatch(opcode)") < order("refresh_select_after_dispatch(opcode)"));
    }

    /// The default session marker must not disable it, and the token spelling the kill
    /// switch documents is the one this code reads.
    #[test]
    fn the_shipped_session_does_not_switch_it_off() {
        // launcher::client::DEFAULT_SESSION as of 2026-09-12; its twin test lives there.
        let shipped = "mode=2,create=on,guardpage=0x20+0x40";
        assert!(!shipped.contains("selectfill=off"));
        assert!("mode=2,create=on,guardpage=0x20+0x40,selectfill=off".contains("selectfill=off"));
    }

    /// Map an RVA to a file offset through the PE section table - the on-disk image is not
    /// laid out like the mapped one, so a test that indexes the file by RVA reads garbage.
    fn file_offset(bytes: &[u8], rva: usize) -> usize {
        let u16_at = |o: usize| u16::from_le_bytes([bytes[o], bytes[o + 1]]) as usize;
        let u32_at = |o: usize| u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap()) as usize;
        let pe = u32_at(0x3c);
        let nsec = u16_at(pe + 6);
        let first = pe + 24 + u16_at(pe + 20);
        for i in 0..nsec {
            let s = first + i * 40;
            let (vsize, va, rsize, raw) = (u32_at(s + 8), u32_at(s + 12), u32_at(s + 16), u32_at(s + 20));
            if va <= rva && rva < va + vsize.max(rsize) {
                return raw + (rva - va);
            }
        }
        panic!("rva {rva:#x} is in no section");
    }

    /// **Every constant above, read back out of the client on disk.** `rip`-relative operands
    /// are resolved the way the CPU does it - next instruction plus displacement - so this
    /// test cannot agree with a hand-typed address by accident; it can only agree with the
    /// bytes. Skipped, loudly, when the client is not beside the repo.
    #[test]
    fn the_constants_agree_with_the_client_on_disk() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../client-patched/MapleStory.exe");
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("skipped: {path} is not present");
            return;
        };
        let at = |rva: usize, n: usize| {
            let o = file_offset(&bytes, rva);
            &bytes[o..o + n]
        };
        // The refresh's prologue, exactly what the runtime check compares against.
        assert_eq!(at(SELECT_REFRESH_RVA, SELECT_REFRESH_PROLOGUE.len()), SELECT_REFRESH_PROLOGUE);

        // `48 8b 0d disp32` = mov rcx,[rip+disp32]; target = next instruction + disp.
        let rip_load = |rva: usize| {
            let b = at(rva, 7);
            assert_eq!(&b[..3], &[0x48, 0x8b, 0x0d], "not a mov rcx,[rip+..] at {rva:#x}: {b:02x?}");
            let disp = i32::from_le_bytes(b[3..7].try_into().unwrap()) as isize;
            (rva as isize + 7 + disp) as usize
        };
        // The screen builder's null-gate before avatar placement (research §1)...
        assert_eq!(rip_load(0x141b3e0d9 - 0x140000000), SELECT_UI_PTR_RVA);
        // ...and the mode-5 login handler's load right before it calls the refresh (§6).
        assert_eq!(rip_load(0x141b33e9c - 0x140000000), SELECT_UI_PTR_RVA);

        // `e8 rel32` at 141b33ea3: the mode-5 handler calling FUN_141177e40 with that object.
        let call_rva = 0x141b33ea3 - 0x140000000;
        let b = at(call_rva, 5);
        assert_eq!(b[0], 0xe8, "not a call at {call_rva:#x}: {b:02x?}");
        let rel = i32::from_le_bytes(b[1..5].try_into().unwrap()) as isize;
        assert_eq!((call_rva as isize + 5 + rel) as usize, SELECT_REFRESH_RVA);
    }
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
