//! Put a session credential in the client's own session object, so the **client** carries it.
//!
//! The owner, 2026-08-29: *"if the client itself has a way to carry an identity, I would like to
//! use that way more ... using the client to pass a session should be what we aim for instead
//! of inference."*
//!
//! Everything the login server does today to decide whose socket it is - the owning process,
//! the source address, "there was only one claim" - is inference about a credential-less
//! connection. This module is the other half of making it a credential: the launcher mints a
//! one-time token, writes it beside the client, and this fills in the one field the client
//! already puts on the wire.
//!
//! # What is filled, and why nothing is forged
//!
//! `research/client-session-args.md` establishes the shape. `0x0073`'s builder
//! `FUN_141b21ea0` reads the identity through `FUN_142c50400(session, &out)` and hands the
//! result straight to the string encoder:
//!
//! ```text
//! 141b23f51  lea  rdx, [rsp+0x68]
//! 141b23f56  mov  rcx, rsi              ; the session object
//! 141b23f59  call 0x142c50400           ; GetSessionIdentity(this, &tmp) -> &tmp
//! 141b23f5f  mov  rdx, rax
//! 141b23f69  call 0x1406edc80           ; w_str: u16 length, then that many bytes
//! ```
//!
//! The field is `session+0x1b8`, it has a working setter `FUN_142c503c0`, and **that setter
//! has no callers in this build**. So it is not that the client cannot carry a session; it is
//! that nothing populates the field. All 72 captured `0x0073` bodies carry a `0000` length
//! prefix.
//!
//! **We send nothing.** The client builds, encrypts and sends `0x0073` with its own code, so
//! the login socket's stateful cipher is never touched. That is the whole reason writing this
//! field is cheap where injecting a packet is not.
//!
//! # Why the write happens *here* and not on a timer
//!
//! An inline hook on `FUN_142c50400` itself. That is the one place where the ordering is
//! **structural rather than a clock margin**:
//!
//! * *after the object exists* - the client just handed it to us in RCX;
//! * *before the field is read* - the detour runs before the target's first instruction, and
//!   the target's first use of the object is `mov rbp,[rcx+0x1b8]` at `+0x18`.
//!
//! The alternative, writing from the packet dispatcher, would have rested on the 3.1 s gap
//! between the last inbound packet and `0x0073` in one archived run. That is a measurement of
//! one session, not a guarantee, and `CLAUDE.md` has a whole section on treating one of those
//! as the other.
//!
//! # The token is a local secret in PLAIN TEXT, and that is deliberate
//!
//! [`IDENTITY_MARKER`] sits beside `MapleStory.exe` holding the token as readable text.
//! Anything running as the owner can read it. That is the same power as *being* the launch, so it
//! does not widen the trust boundary on a single-user machine - but it is written down here
//! rather than glossed.
//!
//! `CLAUDE.md`'s standing constraint is *"never store passwords in plain text"*. This is not a
//! password: it is a **one-time, short-lived launch token** that the server spends on first
//! presentation. The server stores only its SHA-256. The distinction is the whole reason this
//! file may exist at all, so it is stated where somebody will read it.
//!
//! **Do not overstate it, and an earlier draft of this paragraph did.** It said the token
//! "grants no access to an account without also being the process that opened the socket".
//! That is wrong: a **fresh** token is accepted from any process, deliberately, and the
//! credential smoke check depends on that - it proves a connection the server has just
//! failed to attribute is served correctly on the strength of the token alone. The process
//! identity gates a **replay** of an already-spent token, nothing more. So the honest
//! statement is: whoever can read this file for the ~1.5 s it exists can be that launch,
//! once.
//!
//! [`install`] **deletes the marker the moment it has read it**, so the file exists for about
//! a second and a half of client startup rather than until the next launch. That is worth
//! having for its own sake, and it also closes a trap: the login server refuses a connection
//! presenting a credential it does not recognise rather than falling back to the weaker rules,
//! so a marker left behind by one run would quietly downgrade the *next* one.
//!
//! # Every failure is a no-op
//!
//! No marker, an unreadable marker, a token with a byte the client would mangle, a null
//! object, a field that is already set, a refused allocation, a prologue that does not match
//! the one this module was written against - **every one of them logs and changes nothing**.
//! The client then does exactly what it does today: it sends an empty identity. That is a
//! working baseline and it stays the failure mode.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use crate::hook::log;

/// The token, as plain text, beside the client. Absent means this module does nothing at all.
///
/// A file rather than an environment variable for the reason every other marker here is one:
/// the client needs elevation, so it is started with `ShellExecuteW`, which does not carry the
/// environment into the child.
pub const IDENTITY_MARKER: &str = "maplecw-hook.identity";

/// `FUN_142c50400` - `GetSessionIdentity(this, ZXString8 *out)`. The only reader of the field,
/// and it is called only from `FUN_141b21ea0` (2 call sites, 0 tail jumps, 0 pointers -
/// `tools/callers.py`).
const GET_IDENTITY_RVA: usize = 0x1_42C5_0400 - 0x1_4000_0000;

/// `FUN_142c503c0` - `SetSessionIdentity(this, ZXString8 *src)`. **Zero callers in this
/// build.** It assigns into `this+0x1b8` and then releases the caller's temporary, so it takes
/// ownership of one reference.
const SET_IDENTITY_RVA: usize = 0x1_42C5_03C0 - 0x1_4000_0000;

/// `FUN_14019b600(pool, size)` - the client's block allocator. It stores the allocation size
/// at `block-8`, which `FUN_14019f2c0` reads back when freeing, so a block from any other
/// allocator would be freed through the wrong path. Using this one is not optional.
const ALLOC_RVA: usize = 0x1_4019_B600 - 0x1_4000_0000;

/// `&DAT_143ad6a30` - the pool `FUN_14019b600` is called with everywhere strings are made.
///
/// Read off two independent call sites rather than one: `142c50450 lea rcx,[rip+0xe865d9]`
/// inside the identity getter, and `14019a344 lea rcx,[rip+0x393c6e5]` inside the string
/// assign helper. Both resolve to `0x143AD6A30`.
const POOL_RVA: usize = 0x1_43AD_6A30 - 0x1_4000_0000;

/// The identity, on the session object.
const IDENTITY_OFF: usize = 0x1B8;

/// The first 15 bytes of `FUN_142c50400`, which the trampoline relocates.
///
/// ```text
/// 48 89 5c 24 08   mov [rsp+0x08], rbx
/// 48 89 6c 24 10   mov [rsp+0x10], rbp
/// 48 89 74 24 18   mov [rsp+0x18], rsi
///                  <- +15, `push rdi`, where the trampoline jumps back to
/// ```
///
/// All three are position-independent, and 15 is comfortably more than the 12 a 64-bit
/// absolute jump needs. Exactly the shape `hook::install` already relocates out of the packet
/// dispatcher, which is the positive control for this mechanism working at all.
const STOLEN: usize = 15;

/// Those same bytes, checked before anything is patched.
///
/// **A wrong address does not fail loudly.** Patching the middle of some other function would
/// produce a client that dies for reasons nobody could trace back to here, and `CLAUDE.md`
/// records what an unexplained client death costs. So the prologue is compared first and a
/// mismatch stands down.
const EXPECTED_PROLOGUE: [u8; STOLEN] = [
    0x48, 0x89, 0x5C, 0x24, 0x08, 0x48, 0x89, 0x6C, 0x24, 0x10, 0x48, 0x89, 0x74, 0x24, 0x18,
];

/// The absolute jump written over the target is 12 bytes. Fewer stolen bytes than that would
/// leave half an instruction behind. A compile error rather than a test, because there is no
/// input that could make it true at run time.
const _: () = assert!(STOLEN >= 12, "an absolute jump needs 12 bytes");

/// The most identity bytes this module will write.
///
/// **Not the client's limit - deliberately far below it.** What the client's own code and the
/// archive say, established 2026-08-29 because nothing had ever been sent in this field and
/// the token format on both sides depended on the answer:
///
/// * **65535 is the hard ceiling.** `w_str` (`FUN_1406edc80`) computes the length as a `u32`
///   from `[ptr-8]`, writes the prefix as `movzx r8d, cx` - a **u16 truncation** - and then
///   writes the full `u32` count of bytes. Past 65535 the prefix wraps while the body does
///   not, and everything after it in the stream is garbage.
/// * **~3.9 KB is the largest unconditionally quiet frame.** `FUN_1415d3990` compares the
///   frame length against `FUN_14029e2f0(conn+0xc) - 0x64`, and that function returns
///   `0x1000`, `0x2000`, `0x8000` or `0x10000` by connection kind. The smallest bucket puts
///   the threshold at 3996 bytes. Over it the client builds and uploads an oversized-packet
///   report; it does **not** refuse to send.
/// * **6550 bytes is what the client has actually sent** on this socket, in an archived run
///   (`previous-runs/login-20260826-171724.log`, `21:17:15.011 <- 0x0090`, its own error log).
///   So the practical ceiling is nowhere near a token.
///
/// And a constraint that is not about length at all, which [`validate`] enforces: the identity
/// must be **NUL-free and non-empty**, because the getter derives the length it sends with a
/// `strlen` loop rather than from the string header.
///
/// The path from the field to the wire is four calls long - `FUN_14019b600` (allocate),
/// `FUN_142ef7ba0` (copy), and two assert helpers - and then `w_str`'s two leaf writers, which
/// call nothing at all. **Nothing on it transforms the bytes**, and in particular nothing
/// upper-cases them the way the launch-argument parser's `FUN_142c9f2d0` does. Corroborated on
/// the wire: `w_raw` shares `w_str`'s byte writer `FUN_1406ed360`, and the 16-byte machine GUID
/// it writes into this same packet appears verbatim in all 72 captured bodies.
///
/// A launch token is tens of bytes. Anything four figures long is a mistake somewhere else,
/// and refusing it is cheaper than sending it and wondering.
pub const MAX_IDENTITY_BYTES: usize = 1024;

/// Header of the client's `char*` string, byte for byte as `FUN_142c50400` builds one at
/// `142c5045c..142c504f0`. The variable is a `char*` pointing at `base + 0x10`.
///
/// ```text
/// base+0x00  i32 refcount    -1 while under construction, 1 when finished
/// base+0x04  u32 capacity    bytes available, excluding the NUL
/// base+0x08  u32 length      bytes used, excluding the NUL
/// base+0x0c  u32             never written by the client; zeroed here
/// base+0x10  char data[]     length bytes, then a NUL
/// ```
///
/// Freed as `FUN_14019f2c0(ptr - 0x10)`, which is why the header is exactly 16 bytes and why
/// the allocation is `length + 0x11`.
const HDR_REFCOUNT: usize = 0x00;
const HDR_CAPACITY: usize = 0x04;
const HDR_LENGTH: usize = 0x08;
const HDR_SPARE: usize = 0x0C;
const HDR_BYTES: usize = 0x10;

/// One entry does the work; later ones are cheap no-ops.
static FILLED: AtomicBool = AtomicBool::new(false);
/// The token, read and validated once at install time.
///
/// Held here rather than re-read at the moment of use for two reasons, and the second is the
/// one that matters: the getter runs on the client's own thread inside its own call, which is
/// no place to open a file - and **the marker is deleted the moment it has been read**, so
/// there would be nothing to re-read. See [`install`].
static TOKEN: std::sync::OnceLock<String> = std::sync::OnceLock::new();
/// The relocated prologue plus a jump back to `FUN_142c50400+15`. Written by
/// [`install_detour`] itself, before the jump into [`hooked_get_identity`] exists.
static TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
/// How many times the getter has run. Logged so "the field was never read" can be told from
/// "the field was read and was empty" - `CLAUDE.md`, count the same event in two logs.
static ENTRIES: AtomicU64 = AtomicU64::new(0);

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
const MEM_COMMIT: u32 = 0x1000;
const PAGE_GUARD: u32 = 0x100;
const WRITABLE: u32 = 0x04 | 0x08 | 0x40 | 0x80;

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

extern "system" {
    fn VirtualAlloc(addr: *mut c_void, size: usize, typ: u32, protect: u32) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
    fn VirtualQuery(addr: *const c_void, buf: *mut MemoryBasicInformation, len: usize) -> usize;
}

/// Is `len` bytes at `addr` committed and writable?
///
/// Kept beside the read check in `session.rs` rather than shared with it, because that one
/// logs under a `session:` prefix and a line about the identity that says `session:` is a line
/// somebody will spend ten minutes on.
unsafe fn can_write(addr: usize, len: usize) -> bool {
    if addr == 0 {
        return false;
    }
    let mut mbi = std::mem::MaybeUninit::<MemoryBasicInformation>::zeroed();
    let n = VirtualQuery(
        addr as *const c_void,
        mbi.as_mut_ptr(),
        std::mem::size_of::<MemoryBasicInformation>(),
    );
    if n == 0 {
        return false;
    }
    let mbi = mbi.assume_init();
    if mbi.state != MEM_COMMIT || mbi.protect & PAGE_GUARD != 0 || mbi.protect & WRITABLE == 0 {
        return false;
    }
    addr.saturating_add(len) <= (mbi.base_address as usize).saturating_add(mbi.region_size)
}

// ------------------------------------------------------------------------------------------
// The token
// ------------------------------------------------------------------------------------------

/// Why a marker was refused. Each one is a sentence in the log rather than a silent skip,
/// because a token that is quietly dropped looks exactly like a hook that never ran.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    /// No marker file, or it could not be read. The ordinary case: this module is opt-in.
    Absent,
    /// The marker held nothing but whitespace.
    Empty,
    /// A byte the client would not carry intact. See [`validate`].
    BadByte { at: usize, byte: u8 },
    TooLong { len: usize },
}

impl TokenError {
    pub fn describe(&self) -> String {
        match self {
            TokenError::Absent => format!("no {IDENTITY_MARKER} beside the client"),
            TokenError::Empty => format!("{IDENTITY_MARKER} is empty"),
            TokenError::BadByte { at, byte } => format!(
                "{IDENTITY_MARKER} byte {at} is {byte:#04x}, which the client would not carry \
                 intact - the identity is a C string end to end (FUN_142c50400 re-derives its \
                 length with strlen, FUN_14019a260 does the same on its copy path), so a NUL \
                 truncates it silently and anything outside printable ASCII has never been \
                 tested through w_str. Only 0x21..=0x7e is accepted"
            ),
            TokenError::TooLong { len } => format!(
                "{IDENTITY_MARKER} holds {len} bytes; this module refuses more than \
                 {MAX_IDENTITY_BYTES}. The client's own ceiling is far higher (65535 before \
                 w_str's u16 length prefix wraps) but a launch token is tens of bytes and a \
                 four-figure one is a mistake somewhere else"
            ),
        }
    }
}

/// The bytes a token may contain: printable ASCII, no space.
///
/// **This is not tidiness.** The identity is a C string on both sides of the client:
/// `FUN_142c50400` derives the length it sends with a `strlen` loop rather than from the
/// string header (`142c50445 inc rbx; cmp byte [rbx+rbp],0; jne`), and it substitutes an empty
/// literal when the field is null *or begins with a NUL*. So an embedded NUL would truncate
/// the credential with no error anywhere, and a leading one would send nothing at all - both
/// of which read on the wire as "the hook did not write".
///
/// Space and the control range are excluded for a duller reason: the server trims ASCII
/// whitespace and NUL off what arrives (`store::claims::normalise_client_token`), so a token
/// with either at its edge would be a different string at each end.
pub fn validate(token: &str) -> Result<&str, TokenError> {
    let t = token.trim();
    if t.is_empty() {
        return Err(TokenError::Empty);
    }
    if t.len() > MAX_IDENTITY_BYTES {
        return Err(TokenError::TooLong { len: t.len() });
    }
    for (at, &byte) in t.as_bytes().iter().enumerate() {
        if !(0x21..=0x7E).contains(&byte) {
            return Err(TokenError::BadByte { at, byte });
        }
    }
    Ok(t)
}

/// Read and validate the marker. `Err` for every reason there is nothing to write.
pub fn token_from(text: Option<String>) -> Result<String, TokenError> {
    let Some(text) = text else {
        return Err(TokenError::Absent);
    };
    validate(&text).map(str::to_string)
}

fn read_marker() -> Result<String, TokenError> {
    token_from(std::fs::read_to_string(IDENTITY_MARKER).ok())
}

// ------------------------------------------------------------------------------------------
// Building the client's string
// ------------------------------------------------------------------------------------------

/// How many bytes a token of `len` needs, header and NUL included.
///
/// `len + 0x11` is the client's own arithmetic, read straight off `142c5045c
/// lea eax,[rdi+0x11]` where `edi` is the string length. Written as a function so the test
/// below can disagree with it.
pub const fn block_size(len: usize) -> usize {
    len + HDR_BYTES + 1
}

/// Lay a finished client string into `block`, and return the `char*` the client uses.
///
/// `block` must be at least [`block_size`] bytes and must have come from `FUN_14019b600` in
/// the live client - the free path reads the allocation size from `block-8`. In the test below
/// it is an ordinary Rust allocation, which is safe precisely because nothing ever frees it.
///
/// # Safety
/// `block` is written for `block_size(token.len())` bytes.
pub unsafe fn lay_out_string(block: *mut u8, token: &[u8]) -> *mut u8 {
    let n = token.len();
    // Order and values copied from the client's own constructor, with one deliberate
    // difference: it stores -1 in the refcount while it works and rewrites it to 1 at the end,
    // and there is nothing to protect against here, so 1 goes in once.
    *(block.add(HDR_REFCOUNT).cast::<i32>()) = 1;
    *(block.add(HDR_CAPACITY).cast::<u32>()) = n as u32;
    *(block.add(HDR_LENGTH).cast::<u32>()) = n as u32;
    // The client never writes this word. Zeroed anyway: leaving allocator residue in a
    // structure the client compares against would be a bug that only appears sometimes.
    *(block.add(HDR_SPARE).cast::<u32>()) = 0;
    let data = block.add(HDR_BYTES);
    std::ptr::copy_nonoverlapping(token.as_ptr(), data, n);
    // The NUL is load-bearing, not politeness: the getter's length comes from a strlen.
    *data.add(n) = 0;
    data
}

// ------------------------------------------------------------------------------------------
// The inline hook
// ------------------------------------------------------------------------------------------

/// `mov rax, imm64; jmp rax` - 12 bytes, and unlike a relative `e9` it works at any distance.
unsafe fn write_abs_jmp(at: *mut u8, dest: usize) {
    *at = 0x48;
    *at.add(1) = 0xB8;
    std::ptr::copy_nonoverlapping(dest.to_le_bytes().as_ptr(), at.add(2), 8);
    *at.add(10) = 0xFF;
    *at.add(11) = 0xE0;
}

/// How many times [`park_other_threads_outside`] will suspend, look, and let go again.
const PARK_ATTEMPTS: u32 = 200;

/// Suspend every other thread of this process, with none of them stopped inside `[lo, hi)`.
/// Returns the suspended threads' handles for [`release_parked`], or `None` if after
/// [`PARK_ATTEMPTS`] tries some thread was still inside the range (all are running again).
///
/// Handles are opened and the vector sized before anything is suspended; between the first
/// `SuspendThread` and the return nothing allocates. A thread created after the snapshot is
/// not parked - that residue is accepted and noted here rather than hidden.
unsafe fn park_other_threads_outside(lo: usize, hi: usize) -> Option<Vec<*mut c_void>> {
    const TH32CS_SNAPTHREAD: u32 = 0x0000_0004;
    const THREAD_ACCESS: u32 = 0x0002 | 0x0008; // SUSPEND_RESUME | GET_CONTEXT
    const CONTEXT_CONTROL: u32 = 0x0010_0001;
    const CTX_FLAGS_OFF: usize = 0x30;
    const CTX_RIP_OFF: usize = 0xF8;
    const INVALID_HANDLE_VALUE: *mut c_void = usize::MAX as *mut c_void;

    #[repr(C)]
    struct ThreadEntry32 {
        dw_size: u32,
        cnt_usage: u32,
        th32_thread_id: u32,
        th32_owner_process_id: u32,
        tp_base_pri: i32,
        tp_delta_pri: i32,
        dw_flags: u32,
    }
    #[repr(C, align(16))]
    struct Context([u8; 1232]);

    extern "system" {
        fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut c_void;
        fn Thread32First(snap: *mut c_void, entry: *mut ThreadEntry32) -> i32;
        fn Thread32Next(snap: *mut c_void, entry: *mut ThreadEntry32) -> i32;
        fn OpenThread(access: u32, inherit: i32, tid: u32) -> *mut c_void;
        fn SuspendThread(thread: *mut c_void) -> u32;
        fn ResumeThread(thread: *mut c_void) -> u32;
        fn GetThreadContext(thread: *mut c_void, ctx: *mut c_void) -> i32;
        fn CloseHandle(handle: *mut c_void) -> i32;
        fn GetCurrentThreadId() -> u32;
        fn GetCurrentProcessId() -> u32;
        fn Sleep(ms: u32);
    }

    let pid = GetCurrentProcessId();
    let me = GetCurrentThreadId();
    let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if snap == INVALID_HANDLE_VALUE || snap.is_null() {
        return None;
    }
    let mut handles: Vec<*mut c_void> = Vec::with_capacity(128);
    let mut entry = ThreadEntry32 {
        dw_size: std::mem::size_of::<ThreadEntry32>() as u32,
        cnt_usage: 0,
        th32_thread_id: 0,
        th32_owner_process_id: 0,
        tp_base_pri: 0,
        tp_delta_pri: 0,
        dw_flags: 0,
    };
    let mut ok = Thread32First(snap, &mut entry);
    while ok != 0 {
        if entry.th32_owner_process_id == pid && entry.th32_thread_id != me {
            let h = OpenThread(THREAD_ACCESS, 0, entry.th32_thread_id);
            if !h.is_null() {
                handles.push(h);
            }
        }
        entry.dw_size = std::mem::size_of::<ThreadEntry32>() as u32;
        ok = Thread32Next(snap, &mut entry);
    }
    CloseHandle(snap);

    for _ in 0..PARK_ATTEMPTS {
        // Suspend all, then look. Nothing allocates from here to the resume.
        for &h in &handles {
            SuspendThread(h);
        }
        let mut inside = false;
        for &h in &handles {
            let mut ctx = Context([0; 1232]);
            ctx.0[CTX_FLAGS_OFF..CTX_FLAGS_OFF + 4].copy_from_slice(&CONTEXT_CONTROL.to_le_bytes());
            if GetThreadContext(h, std::ptr::addr_of_mut!(ctx).cast()) != 0 {
                let rip = usize::from_le_bytes(ctx.0[CTX_RIP_OFF..CTX_RIP_OFF + 8].try_into().unwrap());
                if rip >= lo && rip < hi {
                    inside = true;
                    break;
                }
            }
        }
        if !inside {
            return Some(handles);
        }
        for &h in &handles {
            ResumeThread(h);
        }
        Sleep(1);
    }
    for h in handles {
        CloseHandle(h);
    }
    None
}

/// Resume and close what [`park_other_threads_outside`] returned.
unsafe fn release_parked(handles: Vec<*mut c_void>) {
    extern "system" {
        fn ResumeThread(thread: *mut c_void) -> u32;
        fn CloseHandle(handle: *mut c_void) -> i32;
    }
    for h in handles {
        ResumeThread(h);
        CloseHandle(h);
    }
}

/// Relocate `stolen` bytes of `target` into a fresh page and point `target` at `detour`.
///
/// Returns the trampoline address. `None` means nothing was patched: either the prologue was
/// not the one this module was written against, the page could not be made writable, or some
/// other thread would not leave the prologue.
///
/// **The trampoline is stored into `publish` BEFORE the jump goes live**, and that is not a
/// nicety. Found in review on 2026-09-08, before the guard-page allocator's first launch: the
/// original shape wrote the jump, restored the page protection, read the bytes back, returned,
/// and only *then* did the caller store the trampoline. A detour entered in that gap loaded a
/// zero and jumped to it. For the identity getter, called once at login, that gap was
/// academic. For `FUN_14019b780` - the pool allocator, entered from thirty threads thousands
/// of times a second - it was a crash at RIP 0 a few microseconds into arming, on the launch
/// meant to prove the instrument does not disturb the client.
///
/// **Every other thread is suspended while the bytes change**, and none of them is allowed to
/// be stopped inside `[target, target+stolen)`. A 12-byte jump written over a live prologue
/// is not atomic; a thread that had already fetched the first `mov` and resumed at byte 5
/// would execute the middle of our immediate. **Nothing at all happens between the suspend
/// and the resume** - not a log, not an allocation, and in particular not the `VirtualProtect`
/// that makes the page writable, which takes the address-space lock a parked thread may be
/// holding. Only the twelve bytes and the `nop` tail are written there. Same rule, same
/// reason, as `poolsentry::thread_snapshot`.
///
/// # Safety
/// `target` must be the first byte of a function whose first `stolen` bytes are
/// position-independent and at least 12 long.
pub unsafe fn install_detour(
    target: usize,
    detour: usize,
    stolen: usize,
    expect: &[u8],
    publish: &AtomicUsize,
) -> Option<usize> {
    debug_assert!(stolen >= 12 && stolen == expect.len());

    // Check BEFORE writing. A wrong address is the failure that costs a launch and cannot be
    // traced back here, so it has to be refused rather than survived.
    let found = std::slice::from_raw_parts(target as *const u8, stolen);
    if found != expect {
        log(&format!(
            "identity: {target:#x} does not begin with the prologue this module was built \
             against - found {found:02x?}, expected {expect:02x?}. NOT patching; the client \
             will send an empty identity exactly as it does today"
        ));
        return None;
    }

    let tramp = VirtualAlloc(
        std::ptr::null_mut(),
        64,
        MEM_COMMIT_RESERVE,
        PAGE_EXECUTE_READWRITE,
    );
    if tramp.is_null() {
        log("identity: VirtualAlloc for the trampoline failed - NOT patching");
        return None;
    }
    let t = tramp.cast::<u8>();
    std::ptr::copy_nonoverlapping(target as *const u8, t, stolen);
    write_abs_jmp(t.add(stolen), target + stolen);

    // Published first: from here on the detour can be entered at any instant and must find
    // a real trampoline.
    publish.store(tramp as usize, Ordering::SeqCst);

    // **Both `VirtualProtect` calls are outside the parked window, and that is the whole
    // ordering.** Found reviewing the first version of this fix: it parked the threads and
    // then called `VirtualProtect`, which takes the process address-space lock - the same lock
    // a thread suspended mid-`VirtualAlloc` is holding. That is a deadlock of the client by
    // the instrument meant to observe it, and it is exactly the rule
    // `poolsentry::thread_snapshot` already states: **nothing at all happens between the
    // suspend and the resume.** Here that leaves only twelve `mov` bytes and some `nop`s.
    // The page is executable-writable for a few microseconds longer in exchange.
    let mut old = 0u32;
    if VirtualProtect(target as *mut c_void, stolen, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log(&format!(
            "identity: could not make {target:#x} writable - NOT patching"
        ));
        return None;
    }
    let Some(parked) = park_other_threads_outside(target, target + stolen) else {
        VirtualProtect(target as *mut c_void, stolen, old, &mut old);
        log(&format!(
            "identity: some thread would not leave the prologue at {target:#x} after {PARK_ATTEMPTS} \
             attempts - NOT patching"
        ));
        return None;
    };
    write_abs_jmp(target as *mut u8, detour);
    for i in 12..stolen {
        *(target as *mut u8).add(i) = 0x90;
    }
    release_parked(parked);
    VirtualProtect(target as *mut c_void, stolen, old, &mut old);

    // Read the patch back. A jump that did not take reports nothing and looks exactly like a
    // function that never runs - the silent negative this repo keeps getting caught by.
    let back = std::slice::from_raw_parts(target as *const u8, 12);
    let mut want = [0u8; 12];
    write_abs_jmp(want.as_mut_ptr(), detour);
    if back != want {
        // **`None` has to mean "nothing was patched", and on THIS path alone it did not.**
        //
        // Found reviewing the guard page before it shipped to players, 2026-09-08. Every other
        // refusal above returns before a byte of `target` is written, so the caller's undo -
        // `guardpage::arm` reverts the HeapFree swap and releases its reserve - is enough. Here
        // the twelve bytes are already in, and the read-back disagreeing means something
        // changed them underneath us: either it restored the whole prologue (in which case
        // nothing is patched and all is well) or it restored PART of it, which is a client
        // executing the middle of our immediate. The doc block above promised the first
        // reading for both, which is a comment describing a guarantee rather than enforcing it
        // - and this was a test-only instrument when that was written and is not any more.
        //
        // The original bytes are not gone: the trampoline's first `stolen` bytes are a verbatim
        // copy of them, taken before anything was written. So put them back, read them back,
        // and say which of the three outcomes happened. Best-effort by construction - a page
        // that is fighting us may win - but "we tried and here is what is there now" is a very
        // different report from silence.
        let restored = restore_prologue(target, t, stolen, expect);
        log(&format!(
            "identity: the jump at {target:#x} DID NOT TAKE - found {back:02x?}. Something is \
             guarding this page; treat any result from this run as unarmed. {restored}"
        ));
        return None;
    }
    Some(tramp as usize)
}

/// Put the original prologue back from the copy in the trampoline, and say what is there now.
///
/// Only ever called from the read-back failure above, so it is off every path that works. It
/// parks the other threads for the same reason the install does - the bytes it writes are a
/// live function's first instructions - and it never reports success it has not read back.
unsafe fn restore_prologue(target: usize, tramp: *mut u8, stolen: usize, expect: &[u8]) -> String {
    let now = std::slice::from_raw_parts(target as *const u8, stolen);
    if now == expect {
        return "The bytes at that address are the ORIGINAL prologue, so nothing is patched and \
                the client is exactly as it was."
            .to_string();
    }
    let mut old = 0u32;
    if VirtualProtect(target as *mut c_void, stolen, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return "***** AND THE ADDRESS COULD NOT BE MADE WRITABLE TO PUT THE ORIGINAL BYTES \
                BACK - the prologue is neither ours nor the client's. Expect a fault. *****"
            .to_string();
    }
    let parked = park_other_threads_outside(target, target + stolen);
    std::ptr::copy_nonoverlapping(tramp, target as *mut u8, stolen);
    if let Some(handles) = parked {
        release_parked(handles);
    }
    VirtualProtect(target as *mut c_void, stolen, old, &mut old);
    let after = std::slice::from_raw_parts(target as *const u8, stolen);
    if after == expect {
        "The original prologue has been written back from the trampoline's copy and reads \
         correctly, so the client is unpatched."
            .to_string()
    } else {
        format!(
            "***** AND THE ORIGINAL PROLOGUE COULD NOT BE RESTORED - {after:02x?} is at \
             {target:#x} and it is neither our jump nor the client's own bytes. Expect a \
             fault. *****"
        )
    }
}

/// Our stand-in for `FUN_142c50400`. Fills the field, then runs the real getter.
///
/// Ordinary code on the client's own thread, not a vectored handler: the client is standing at
/// a function entry with nothing of its own half-done, which is the cheapest moment there is
/// to call its allocator - the real getter's own next act is a call to the same allocator with
/// the same pool.
unsafe extern "system" fn hooked_get_identity(
    session: *mut c_void,
    out: *mut c_void,
) -> *mut c_void {
    let n = ENTRIES.fetch_add(1, Ordering::SeqCst) + 1;
    fill(session, n);
    let tramp: extern "system" fn(*mut c_void, *mut c_void) -> *mut c_void =
        std::mem::transmute(TRAMPOLINE.load(Ordering::SeqCst) as usize);
    tramp(session, out)
}

/// Write the identity once. Every refusal is a log line and a return.
unsafe fn fill(session: *mut c_void, entry: u64) {
    if FILLED.swap(true, Ordering::SeqCst) {
        return;
    }
    let Some(token) = TOKEN.get() else {
        // Not reachable: `install` stores the token before it patches anything, and nothing
        // else plants this detour. Logged rather than asserted, because a panic here would
        // unwind through the client's own call frame.
        log("identity: the detour ran with no token stored - nothing written");
        return;
    };

    let obj = session as usize;
    let field = obj + IDENTITY_OFF;
    if obj == 0 || !crate::session::can_read(field, 8) || !can_write(field, 8) {
        log(&format!(
            "identity: session object {obj:#x} has no writable {IDENTITY_OFF:#x} - leaving the \
             identity empty"
        ));
        return;
    }
    let before = *(field as *const usize);
    if before != 0 {
        log(&format!(
            "identity: {obj:#x}+{IDENTITY_OFF:#x} already holds {before:#x} - the client set it \
             itself, which has never been seen. NOT overwriting it"
        ));
        return;
    }

    let base = crate::hook::base();
    let pool = (base + POOL_RVA) as *mut c_void;
    let alloc: extern "system" fn(*mut c_void, usize) -> *mut u8 =
        std::mem::transmute(base + ALLOC_RVA);
    let want = block_size(token.len());
    let block = alloc(pool, want);
    if block.is_null() || !can_write(block as usize, want) {
        log(&format!(
            "identity: the client's allocator refused {want} bytes - leaving the identity empty"
        ));
        return;
    }
    let mut tmp = lay_out_string(block, token.as_bytes());

    // The client's own setter, rather than a store into the field. It does the assign and then
    // releases our temporary, which is exactly one reference handed over, and it is the code
    // the client would have run if anything ever called it.
    let set: extern "system" fn(*mut c_void, *mut *mut u8) = std::mem::transmute(base + SET_IDENTITY_RVA);
    set(session, &mut tmp);

    let after = *(field as *const usize);
    if after != block.add(HDR_BYTES) as usize {
        log(&format!(
            "identity: FUN_142c503c0 left {after:#x} in {obj:#x}+{IDENTITY_OFF:#x}, not the \
             {:#x} we built. The field is whatever the client has now; nothing further is done",
            block.add(HDR_BYTES) as usize
        ));
        return;
    }

    // **A deliberate one-block leak, and it is the safer half of the trade.**
    //
    // The setter leaves the field owning one reference, so the session destructor
    // (`142c444df mov rcx,[rdi+0x1b8]` then `FUN_14019f2c0(rcx-0x10)`) would free our block at
    // shutdown. That free is the one place a header we built could take the client down, and a
    // client that dies on the way out still costs the owner the launch it dies on. Holding a second
    // reference means the destructor's release drops 2 to 1 and returns without freeing
    // anything: after this line the only client code that ever touches the block is a
    // decrement. The cost is `block_size` bytes, once, for the life of the process.
    let refs = &*(block.cast::<std::sync::atomic::AtomicI32>());
    refs.fetch_add(1, Ordering::SeqCst);

    log(&format!(
        "***** IDENTITY wrote {} bytes into {obj:#x}+{IDENTITY_OFF:#x} on getter entry #{entry} \
         (block {:#x}, refcount held at 2 so the session destructor cannot free it). The token \
         itself is NOT logged. login.log should now show `0x0073 IDENTITY: ... identity \
         length={}` - if it shows 0, the client dropped it between here and w_str *****",
        token.len(),
        block as usize,
        token.len()
    ));
}

/// Install, if there is a token to write. Safe to call twice.
pub unsafe fn install() {
    if TRAMPOLINE.load(Ordering::SeqCst) != 0 {
        return;
    }
    let token = match read_marker() {
        Ok(t) => t,
        Err(TokenError::Absent) => return, // the ordinary case; not worth a line every run
        Err(why) => {
            log(&format!(
                "identity: NOT ARMED - {}. The client will send an empty identity, which is \
                 what it does today",
                why.describe()
            ));
            return;
        }
    };

    let base = crate::hook::base();
    if base == 0 {
        log("identity: no module base yet - not arming");
        return;
    }
    let _ = TOKEN.set(token.clone());

    // **Delete the marker now that it has been read.** Three things fall out of one line:
    //
    // * a one-time secret stops existing on disk the moment it is no longer needed;
    // * `fill` does no file I/O on the client's own thread;
    // * and the important one - **a token cannot survive into a later run**. The login
    //   server refuses a connection that presents a credential it does not recognise rather
    //   than falling through to the weaker rules, so a marker left behind by a launcher run
    //   would silently downgrade the *next* ordinary run to the fallback account. That is a
    //   failure nobody would attribute to a leftover file.
    match std::fs::remove_file(IDENTITY_MARKER) {
        Ok(()) => log(&format!(
            "identity: {IDENTITY_MARKER} read and DELETED - the token now exists only in this \
             process, and cannot be picked up by a later run"
        )),
        Err(e) => log(&format!(
            "identity: could not delete {IDENTITY_MARKER} ({e}). This run is unaffected, but \
             the token is still on disk and a LATER run would present it and be REFUSED - \
             delete it by hand before the next launch"
        )),
    }

    let target = base + GET_IDENTITY_RVA;
    let detour: unsafe extern "system" fn(*mut c_void, *mut c_void) -> *mut c_void =
        hooked_get_identity;
    let Some(tramp) =
        install_detour(target, detour as usize, STOLEN, &EXPECTED_PROLOGUE, &TRAMPOLINE)
    else {
        return;
    };
    log(&format!(
        "identity: armed on FUN_142c50400 at {target:#x} (trampoline {tramp:#x}), {} byte token \
         from {IDENTITY_MARKER}. It was a PLAIN-TEXT LOCAL SECRET on disk until the line above \
         deleted it: anything running as this user could have read it, which is the same power \
         as being this launch. It is one-time, and the server keeps only its SHA-256",
        token.len()
    ));
}

/// How many times the getter ran, for the two-log count.
pub fn entries() -> u64 {
    ENTRIES.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------------------
    // The string the client will read
    // --------------------------------------------------------------------------------------

    /// The offsets are the ones `FUN_142c50400` writes, at `142c50470`, `142c504f0`,
    /// `142c504b7` and `142c504c1`. Pinned here so a later edit that "tidies" them has to
    /// argue with the listing.
    #[test]
    fn the_header_is_the_one_the_client_builds() {
        assert_eq!((HDR_REFCOUNT, HDR_CAPACITY, HDR_LENGTH, HDR_SPARE, HDR_BYTES),
                   (0x00, 0x04, 0x08, 0x0C, 0x10));
        // `142c5045c lea eax,[rdi+0x11]` - the client's own allocation arithmetic.
        assert_eq!(block_size(0), 0x11);
        assert_eq!(block_size(26), 26 + 0x11);
    }

    /// Build one and read every field back the way the client would.
    #[test]
    fn a_laid_out_string_reads_back_as_the_client_would_read_it() {
        let token = b"MFRGGZDFMZTWQ2LKNNWG23TP2A";
        let mut block = vec![0xAAu8; block_size(token.len())];
        // SAFETY: the buffer is exactly block_size long and outlives the reads below.
        let data = unsafe { lay_out_string(block.as_mut_ptr(), token) };
        let off = unsafe { data.offset_from(block.as_ptr()) };
        assert_eq!(off, HDR_BYTES as isize, "the char* must point at base+0x10");

        let u32_at = |o: usize| u32::from_le_bytes(block[o..o + 4].try_into().unwrap());
        assert_eq!(u32_at(HDR_REFCOUNT), 1, "refcount");
        assert_eq!(u32_at(HDR_CAPACITY), token.len() as u32, "capacity");
        assert_eq!(u32_at(HDR_LENGTH), token.len() as u32, "length");
        assert_eq!(u32_at(HDR_SPARE), 0, "the spare word must not keep allocator residue");
        assert_eq!(&block[HDR_BYTES..HDR_BYTES + token.len()], token);
        assert_eq!(
            block[HDR_BYTES + token.len()],
            0,
            "no NUL: FUN_142c50400 derives the length it sends with a strlen loop, so an \
             unterminated string sends whatever follows it in the heap"
        );
    }

    /// The empty case, because `block_size(0)` is the one place the arithmetic could go
    /// negative and because a zero-length token must still terminate.
    #[test]
    fn a_zero_length_string_still_terminates() {
        let mut block = vec![0xAAu8; block_size(0)];
        let data = unsafe { lay_out_string(block.as_mut_ptr(), b"") };
        assert_eq!(unsafe { *data }, 0);
        assert_eq!(u32::from_le_bytes(block[HDR_LENGTH..HDR_LENGTH + 4].try_into().unwrap()), 0);
    }

    // --------------------------------------------------------------------------------------
    // The token
    // --------------------------------------------------------------------------------------

    #[test]
    fn a_base32_token_is_accepted_and_trimmed() {
        // 26 characters of uppercase base32 - what `store::claims` mints.
        assert_eq!(validate("MFRGGZDFMZTWQ2LKNNWG23TP2A"), Ok("MFRGGZDFMZTWQ2LKNNWG23TP2A"));
        // A marker written with a trailing newline must still work: `Set-Content` adds one.
        assert_eq!(validate("MFRGGZDFMZTWQ2LKNNWG23TP2A\r\n"), Ok("MFRGGZDFMZTWQ2LKNNWG23TP2A"));
    }

    /// **A NUL is the one that would be invisible.** It does not fail anywhere: the string is
    /// laid out, the client sends `strlen` bytes, and the credential is silently short.
    #[test]
    fn a_token_the_client_would_mangle_is_refused() {
        assert_eq!(validate("ABC\0DEF"), Err(TokenError::BadByte { at: 3, byte: 0 }));
        assert_eq!(validate("ABC DEF"), Err(TokenError::BadByte { at: 3, byte: b' ' }));
        assert_eq!(validate("ABC\tDEF"), Err(TokenError::BadByte { at: 3, byte: b'\t' }));
        // Non-ASCII: w_str writes bytes, but nothing on either side has been tested with them.
        assert!(matches!(validate("ABCé"), Err(TokenError::BadByte { .. })));
    }

    #[test]
    fn an_empty_or_missing_marker_is_not_a_credential() {
        assert_eq!(validate(""), Err(TokenError::Empty));
        assert_eq!(validate("   \r\n"), Err(TokenError::Empty));
        assert_eq!(token_from(None), Err(TokenError::Absent));
        assert_eq!(token_from(Some("  ".into())), Err(TokenError::Empty));
    }

    #[test]
    fn an_absurdly_long_token_is_refused_rather_than_sent() {
        let long = "A".repeat(MAX_IDENTITY_BYTES + 1);
        assert_eq!(validate(&long), Err(TokenError::TooLong { len: MAX_IDENTITY_BYTES + 1 }));
        assert!(validate(&"A".repeat(MAX_IDENTITY_BYTES)).is_ok());
    }

    /// Every refusal has to say what it costs. A blank sentence in the hook log is the same as
    /// no sentence, and this module's whole failure mode is silence.
    #[test]
    fn every_refusal_says_something() {
        for e in [
            TokenError::Absent,
            TokenError::Empty,
            TokenError::BadByte { at: 0, byte: 0 },
            TokenError::TooLong { len: 9999 },
        ] {
            let d = e.describe();
            assert!(d.len() > 20, "{e:?} -> {d:?}");
            assert!(d.contains(IDENTITY_MARKER), "{e:?} -> {d:?}");
        }
    }

    // --------------------------------------------------------------------------------------
    // The mechanism itself, driven end to end
    // --------------------------------------------------------------------------------------

    /// The 15 bytes come from `python tools/dump_va.py 0x142C50400 24` against the client in
    /// `client-patched/`, not from hand-assembling the mnemonics. That matters: this constant
    /// is what stops a wrong address from being patched, so deriving it the same way the code
    /// under test would derive it proves nothing.
    #[test]
    fn the_expected_prologue_is_fifteen_position_independent_bytes() {
        assert_eq!(EXPECTED_PROLOGUE.len(), STOLEN);
        assert_eq!(STOLEN, 15);
        assert_eq!(
            EXPECTED_PROLOGUE,
            [0x48, 0x89, 0x5C, 0x24, 0x08, // mov [rsp+0x08], rbx
             0x48, 0x89, 0x6C, 0x24, 0x10, // mov [rsp+0x10], rbp
             0x48, 0x89, 0x74, 0x24, 0x18] // mov [rsp+0x18], rsi
        );
    }

    /// A hand-built function with the same prologue shape as `FUN_142c50400`, so the
    /// mechanism can be exercised on something we own.
    ///
    /// ```text
    /// 48 89 5c 24 08   mov [rsp+0x08], rbx     <- the 15 relocated bytes
    /// 48 89 6c 24 10   mov [rsp+0x10], rbp
    /// 48 89 74 24 18   mov [rsp+0x18], rsi
    /// 48 8d 04 11      lea rax, [rcx+rdx]      <- the body: return arg1 + arg2
    /// c3               ret
    /// ```
    const SUBJECT: [u8; 20] = [
        0x48, 0x89, 0x5C, 0x24, 0x08, 0x48, 0x89, 0x6C, 0x24, 0x10, 0x48, 0x89, 0x74, 0x24, 0x18,
        0x48, 0x8D, 0x04, 0x11, 0xC3,
    ];

    static SELFTEST_TRAMP: AtomicUsize = AtomicUsize::new(0);
    static SELFTEST_HITS: AtomicU64 = AtomicU64::new(0);

    unsafe extern "system" fn selftest_detour(a: u64, b: u64) -> u64 {
        SELFTEST_HITS.fetch_add(1, Ordering::SeqCst);
        let t: extern "system" fn(u64, u64) -> u64 =
            std::mem::transmute(SELFTEST_TRAMP.load(Ordering::SeqCst) as usize);
        t(a, b)
    }

    /// **The instrument, driven end to end on a subject we control.**
    ///
    /// `crates/grap-stub/src/minidump.rs` exists because a dump writer that had never written
    /// a dump was an unverified instrument. This is the same rule: an inline hook that has
    /// never been installed is a claim. The one thing it cannot cover is Themida deciding to
    /// checksum `.text` - and the positive control for that is `hook::install`, which has been
    /// patching the packet dispatcher in the same section for weeks.
    #[test]
    fn the_inline_hook_relocates_a_prologue_and_still_returns_the_right_answer() {
        unsafe {
            let page = VirtualAlloc(
                std::ptr::null_mut(),
                4096,
                MEM_COMMIT_RESERVE,
                PAGE_EXECUTE_READWRITE,
            );
            assert!(!page.is_null(), "VirtualAlloc");
            std::ptr::copy_nonoverlapping(SUBJECT.as_ptr(), page.cast::<u8>(), SUBJECT.len());
            let subject: extern "system" fn(u64, u64) -> u64 = std::mem::transmute(page);

            // Unhooked, so a later equality proves the trampoline and not a constant.
            assert_eq!(subject(7, 5), 12);
            assert_eq!(SELFTEST_HITS.load(Ordering::SeqCst), 0);

            let detour: unsafe extern "system" fn(u64, u64) -> u64 = selftest_detour;
            let tramp = install_detour(
                page as usize,
                detour as usize,
                STOLEN,
                &SUBJECT[..STOLEN],
                &SELFTEST_TRAMP,
            )
            .expect("the detour should install over a prologue that matches");
            // Published by install_detour itself, before the jump went live - the detour
            // never sees a zero trampoline, however early it is entered.
            assert_eq!(SELFTEST_TRAMP.load(Ordering::SeqCst), tramp);

            // Through the detour: the same answer, and the detour demonstrably ran. Both
            // halves matter - a hook that runs but breaks the function is worse than none.
            assert_eq!(subject(7, 5), 12, "the trampoline did not reach the real body");
            assert_eq!(SELFTEST_HITS.load(Ordering::SeqCst), 1, "the detour never ran");
            assert_eq!(subject(1000, 24), 1024);
            assert_eq!(SELFTEST_HITS.load(Ordering::SeqCst), 2);
        }
    }

    /// **A wrong address must be refused, not survived.** This is the guard that stops a
    /// changed client or a mistyped RVA from turning into a fault nobody can trace.
    #[test]
    fn a_prologue_that_does_not_match_is_not_patched() {
        unsafe {
            let page = VirtualAlloc(
                std::ptr::null_mut(),
                4096,
                MEM_COMMIT_RESERVE,
                PAGE_EXECUTE_READWRITE,
            );
            assert!(!page.is_null());
            std::ptr::copy_nonoverlapping(SUBJECT.as_ptr(), page.cast::<u8>(), SUBJECT.len());

            let mut wrong = EXPECTED_PROLOGUE;
            wrong[0] = 0xCC;
            let detour: unsafe extern "system" fn(u64, u64) -> u64 = selftest_detour;
            static UNTOUCHED: AtomicUsize = AtomicUsize::new(0);
            assert!(
                install_detour(page as usize, detour as usize, STOLEN, &wrong, &UNTOUCHED)
                    .is_none(),
                "a mismatched prologue must refuse"
            );
            assert_eq!(UNTOUCHED.load(Ordering::SeqCst), 0, "a refusal publishes nothing");
            // And nothing was written: the subject still runs untouched.
            let subject: extern "system" fn(u64, u64) -> u64 = std::mem::transmute(page);
            assert_eq!(subject(2, 3), 5);
        }
    }
}
