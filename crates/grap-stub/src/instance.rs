//! **How this client decides it is already running, and how to stop it deciding that.**
//!
//! The owner, 2026-09-01: *"A second client does not open after the launcher changes. I have
//! tried to minimize the client to open a second instance of the client, but when I tried,
//! the previously minimized MapleStory.exe became un-minimized."*
//!
//! That last sentence is the whole reason this module exists, and it is worth more than it
//! looks. A process that merely refuses to start does not touch anybody else's window. This
//! one **found the first instance's window and restored it**, which is the signature of a
//! deliberate single-instance guard - `FindWindow`, then `ShowWindow`/`SetForegroundWindow`,
//! then exit. A bare named-mutex guard cannot un-minimize anything.
//!
//! Labels are the project's: **[L]** read off this client or a capture, **[D]** derived,
//! **[I]** inferred.
//!
//! # Why this has to be a runtime hook, and why it can be
//!
//! **Static analysis cannot name the guard.** Themida strips the import name tables: parsing
//! the client's import directory gives 34 DLLs but exactly **one named import per DLL** -
//! `kernel32!GetModuleHandleA` and `USER32!SendMessageA` - and everything else is resolved by
//! the packer at runtime. **[L]** A whole-image search for `CreateMutexW` returns zero, and so
//! does the control (`CreateFileW`, `GetProcAddress`, `WSAStartup`), which is what says the
//! search is blind rather than the API absent. `CLAUDE.md` calls that class of clean confident
//! nothing the most expensive kind.
//!
//! **But `grap64.dll` is a STATIC import of `MapleStory.exe`.** **[L]** from the same import
//! directory walk. So the Windows loader maps this DLL and calls `DllMain` **before the
//! executable's entry point** - before Themida's stub, and long before any guard the client's
//! own code runs. We are first, in every process, including the second one.
//!
//! # What it does
//!
//! Gated on the marker file [`MULTICLIENT_MARKER`]; absent, this module does not read a byte
//! or patch an instruction.
//!
//! * Hooks `FindWindowW`, `FindWindowA`, `CreateMutexW` and `CreateMutexA`, and **logs every
//!   call's arguments**. One launch names the guard exactly instead of guessing at it.
//! * And **neutralises** both shapes in the same build, because a client run costs the owner a
//!   manual launch and these two answers are not in tension:
//!   - `FindWindow*` returns `NULL` - the guard does not find the other window.
//!   - `CreateMutex*` calls the real one and then clears the thread's last error, so the
//!     caller's `GetLastError() == ERROR_ALREADY_EXISTS` test comes back false. No renaming,
//!     no second handle, nothing to leak.
//!
//! So one run has four distinguishable outcomes, and every one of them is information:
//!
//! ```text
//!   a second client opens, and the log names which API fired  -> that was the guard
//!   a second client opens and NEITHER fired                   -> the launcher was the
//!                                                                only thing stopping it
//!   no second client, but the log shows calls                 -> the guard is one of these
//!                                                                and suppressing it is not
//!                                                                enough; the log says what
//!                                                                it asked for
//!   no second client and NO log from the second process       -> our DllMain did not run
//!                                                                there, and nothing
//!                                                                in-process can help
//! ```
//!
//! # The safety rule this module will not break
//!
//! A 64-bit absolute jump needs 12 bytes, so hooking means overwriting a function's first 12.
//! Overwriting **part** of an instruction corrupts the function for every caller in the
//! process - including the first client, which is running fine.
//!
//! There is no length disassembler here, so [`safe_prologue_len`] measures the prologue
//! against a **whitelist of known-length opcodes** and refuses to patch unless whole
//! instructions cover at least 12 bytes. If a target does not match, it is **logged with its
//! first 16 bytes and skipped**. That failure is useful and harmless: the bytes in the log are
//! exactly what a later pass needs, and nothing was written.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::hook::log;

/// Presence of this file enables the instance-guard hooks. A file rather than an environment
/// variable for the reason `crate::hook::HOOK_MARKER` gives: the client is started with
/// `ShellExecuteW` because it carries an elevation manifest, and ShellExecute does not carry
/// the environment into the child.
pub const MULTICLIENT_MARKER: &str = "maplecw-hook.multiclient";

/// **A veto that outranks the marker**, so this whole module can be taken out of a run
/// without rebuilding anything.
///
/// The launcher writes [`MULTICLIENT_MARKER`] on every Start Game, so deleting it by hand
/// does not survive to the next launch - there was no way to ask "does the client behave
/// differently without these hooks?" without editing code. That question came up the moment
/// a rendering fault appeared in a run that also had two `user32` functions code-patched, and
/// it is exactly the A/B this file's own rules demand before calling anything a regression.
///
/// `tools/test-server.ps1 -NoInstanceHooks` writes it, and every other launch of that script
/// deletes it - so it cannot be left switched on by accident, which a hand-made file could.
pub const VETO_MARKER: &str = "maplecw-hook.nomulticlient";

/// Whether the marker is present, and the veto is not. Checked once, at `DllMain`.
pub fn enabled() -> bool {
    if std::path::Path::new(VETO_MARKER).exists() {
        log(&format!(
            "instance: {VETO_MARKER} is present - NOTHING is hooked and no code is patched \
             this run. This is the control: the client behaves as it did before this module \
             existed, and a second one will be stopped by its own guard"
        ));
        return false;
    }
    std::path::Path::new(MULTICLIENT_MARKER).exists()
}

static ARMED: AtomicBool = AtomicBool::new(false);
/// **Whether this process should have the guard suppressed for it.**
///
/// False in the first client, true in every later one - see [`arm`]. The hooks are installed
/// either way, because their log is the measurement; only the *answers* they give change.
static SUPPRESS: AtomicBool = AtomicBool::new(false);
static FIND_W: AtomicU64 = AtomicU64::new(0);
static FIND_A: AtomicU64 = AtomicU64::new(0);
static MUTEX_W: AtomicU64 = AtomicU64::new(0);
static MUTEX_A: AtomicU64 = AtomicU64::new(0);

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
/// For the forwarder-slot rewrite, which touches data and must not make it executable.
const PAGE_READWRITE: u32 = 0x04;
const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
/// `ERROR_ALREADY_EXISTS`, the value a single-instance mutex guard tests for.
const ERROR_ALREADY_EXISTS: u32 = 183;

/// **Our own mutex, used only to answer "am I the first client?"**
///
/// Deliberately not shared with anything and deliberately session-local (`Local\`), so it
/// cannot collide with the client's own guard or with another Windows session.
///
/// It exists because the alternative was worse. Suppressing `FindWindow` unconditionally
/// would change the behaviour of the **first** client too - the one that is currently working
/// - and the client may well use `FindWindow` for something legitimate. Letting the stub work
/// out its own ordinal means the first process behaves exactly as it does today and only the
/// second is treated. It also needs nothing from the launcher, which keeps the decision in the
/// one place that can actually observe it.
const OUR_MUTEX: &str = "Local\\MapleCW-client-instance";

extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
    fn VirtualAlloc(addr: *mut c_void, size: usize, typ: u32, protect: u32) -> *mut c_void;
    fn GetCurrentProcessId() -> u32;
    fn GetLastError() -> u32;
    fn SetLastError(code: u32);
}

/// The 12 bytes of an absolute jump: `mov rax, imm64` then `jmp rax`.
///
/// Same encoding `crate::hook` uses; repeated rather than shared because that one is private
/// to its own installer and this module must not depend on the dispatcher hook being armed.
const ABS_JMP_LEN: usize = 12;

unsafe fn write_abs_jmp(at: *mut u8, dest: usize) {
    *at = 0x48;
    *at.add(1) = 0xB8;
    std::ptr::copy_nonoverlapping(dest.to_le_bytes().as_ptr(), at.add(2), 8);
    *at.add(10) = 0xFF;
    *at.add(11) = 0xE0;
}

/// **How many bytes of whole instructions start this function, up to `want`** - or `None` if
/// the prologue is not one this module is willing to steal.
///
/// This is deliberately a whitelist and not a disassembler. Every entry is a fixed-length
/// opcode form that appears in the standard MSVC x64 prologues these APIs are compiled with,
/// and anything unrecognised stops the walk immediately. The consequence of being too strict
/// is a skipped hook and a log line; the consequence of being too clever would be half an
/// instruction overwritten in a DLL the whole process shares.
///
/// Returned length is **at least** `want` when `Some`, so the caller may copy exactly that
/// many bytes and know it is not splitting anything.
pub fn safe_prologue_len(code: &[u8], want: usize) -> Option<usize> {
    let mut i = 0usize;
    while i < want {
        let n = insn_len(&code[i..])?;
        i += n;
    }
    Some(i)
}

/// The length of one instruction from a small whitelist, or `None`.
fn insn_len(b: &[u8]) -> Option<usize> {
    let first = *b.first()?;
    Some(match first {
        // push r64 / pop r64  (0x50..0x5F), one byte.
        0x50..=0x5F => 1,
        // A REX prefix: look at what follows.
        0x40..=0x4F => {
            let op = *b.get(1)?;
            match op {
                // push/pop with REX.B - two bytes.
                0x50..=0x5F => 2,
                // 89 /r  mov r/m64, r64   and   8B /r  mov r64, r/m64
                0x89 | 0x8B => 2 + modrm_len(b.get(2..)?)?,
                // 83 /digit ib   arithmetic with a sign-extended byte (sub rsp, 0x28)
                0x83 => 2 + modrm_len(b.get(2..)?)? + 1,
                // 81 /digit id   arithmetic with a dword
                0x81 => 2 + modrm_len(b.get(2..)?)? + 4,
                // 8D /r  lea
                0x8D => 2 + modrm_len(b.get(2..)?)?,
                // 31 /r and 33 /r with REX - `xor rax, rax` and friends.
                0x31 | 0x33 => 2 + modrm_len(b.get(2..)?)?,
                // 85 /r with REX - `test rdx, rdx`.
                0x85 => 2 + modrm_len(b.get(2..)?)?,
                // B8+r with REX. **The width depends on REX.W and getting it wrong splits an
                // instruction**: without W it is still a 4-byte immediate (`mov r9d, imm32`,
                // 6 bytes total), with W it is a full 8-byte one (`movabs r64, imm64`, 10).
                // The real prologue this was added for is the first form.
                0xB8..=0xBF => {
                    if first & 0x08 != 0 {
                        10
                    } else {
                        6
                    }
                }
                _ => return None,
            }
        }
        // B8+r id  mov r32, imm32   /   with REX.W, movabs r64, imm64
        //
        // Added 2026-09-02, again from a measurement. `kernelbase!CreateMutexW` - which is
        // where kernel32 forwards to - begins `49 8b c0 / 41 b9 01 00 1f 00 / 45 33 c0`, and
        // the middle one is `mov r9d, 0x1F0001`. Without it the measurer stopped after three
        // bytes and BOTH mutex exports came back "NOT hooked, nothing written", which is the
        // reason the second client's exit could not be observed.
        //
        // The immediate is data, not an address, so it relocates freely - unlike the `ff 25`
        // form, which `modrm_len` still refuses because its displacement is RIP-relative.
        0xB8..=0xBF => 5,
        // 85 /r  test r/m, r - the instruction after the xor in that same prologue. Opcode
        // plus ModRM, exactly like 0x89 / 0x8B.
        0x85 => 1 + modrm_len(b.get(1..)?)?,
        // 31 /r  xor r/m, r   and   33 /r  xor r, r/m
        //
        // Added 2026-09-02 from a measurement, not from reading a list. `user32!FindWindowA`
        // begins `48 83 ec 38 / 4c 8b ca / 4c 8b c1 / 33 d2 / 33 c9` - the two `xor` are the
        // idiomatic zeroing of edx and ecx before a call, and without them this measurer
        // stopped at 10 bytes and the export was refused. The first four instructions were
        // already understood; the whole prologue is 14 bytes, one over what is needed.
        //
        // Safe for the same reason 0x89/0x8B are: opcode plus ModRM, and `modrm_len` refuses
        // the RIP-relative form, so nothing position-dependent can be copied.
        0x31 | 0x33 => 1 + modrm_len(b.get(1..)?)?,
        // 89 /r and 8B /r without REX.
        0x89 | 0x8B => 1 + modrm_len(b.get(1..)?)?,
        // 83 /digit ib without REX.
        0x83 => 1 + modrm_len(b.get(1..)?)? + 1,
        _ => return None,
    })
}

/// Bytes consumed by a ModRM byte plus any SIB and displacement it implies.
fn modrm_len(b: &[u8]) -> Option<usize> {
    let modrm = *b.first()?;
    let md = modrm >> 6;
    let rm = modrm & 0b111;
    // No 16-bit or RIP-relative forms are accepted: mod == 0 with rm == 5 is RIP-relative on
    // x64 and its displacement would move if we relocated the instruction, so it is refused
    // outright rather than copied.
    if md == 0 && rm == 0b101 {
        return None;
    }
    let sib = usize::from(md != 0b11 && rm == 0b100);
    let disp = match md {
        0b00 => 0,
        0b01 => 1,
        0b10 => 4,
        _ => 0,
    };
    Some(1 + sib + disp)
}

/// **Follow an `ff 25` indirect jump to the function it forwards to.**
///
/// Found by measurement on 2026-09-02, and it is the reason a mutex guard would have gone
/// completely unhooked. Both mutex exports in `kernel32` are six-byte thunks:
///
/// ```text
///   kernel32!CreateMutexW   ff 25 0a 09 06 00   jmp qword ptr [rip+0x6090a]
///   kernel32!CreateMutexA   ff 25 6a 08 06 00   jmp qword ptr [rip+0x6086a]
/// ```
///
/// That is the API-set forwarder: the implementation lives in `kernelbase.dll` and
/// `kernel32` is a shim over it. [`safe_prologue_len`] refused them, correctly - `ff 25`
/// carries a RIP-relative displacement and copying it elsewhere would jump into nowhere - so
/// the log said "NOT hooked, nothing written" twice and the run could not have observed a
/// mutex even if the client's guard were one.
///
/// Resolving is better than naming `kernelbase.dll` in the caller, because it does not depend
/// on knowing which module any given export forwards to on any given Windows build: the
/// pointer says. Bounded to a few hops so a cycle cannot spin, and every hop is logged -
/// hooking an address in a module the caller did not name is exactly the kind of thing that
/// must not happen quietly.
unsafe fn follow_thunk(mut addr: usize, what: &str) -> usize {
    for _ in 0..4 {
        let b = std::slice::from_raw_parts(addr as *const u8, 6);
        if b[0] != 0xFF || b[1] != 0x25 {
            return addr;
        }
        let disp = i32::from_le_bytes([b[2], b[3], b[4], b[5]]) as isize;
        // The displacement is from the END of the six-byte instruction.
        let slot = (addr as isize + 6 + disp) as usize;
        let next = *(slot as *const usize);
        if next == 0 {
            return addr;
        }
        log(&format!(
            "instance: {what} at {addr:#x} is an ff-25 forwarder -> {next:#x}, following it"
        ));
        addr = next;
    }
    addr
}

/// Install one hook. Returns the trampoline address, or `None` with a reason already logged.
unsafe fn hook(module: &str, name: &str, detour: usize) -> Option<usize> {
    let module_z = format!("{module}\0");
    let name_z = format!("{name}\0");
    let m = GetModuleHandleA(module_z.as_ptr());
    if m.is_null() {
        log(&format!("instance: {module} is not loaded - {name} NOT hooked"));
        return None;
    }
    let target = GetProcAddress(m, name_z.as_ptr()) as usize;
    if target == 0 {
        log(&format!("instance: {module}!{name} not found - NOT hooked"));
        return None;
    }
    // Both `kernel32` mutex exports are forwarders into `kernelbase`. Hook what actually runs.
    let target = follow_thunk(target, &format!("{module}!{name}"));
    let head = std::slice::from_raw_parts(target as *const u8, 16);
    let hex: Vec<String> = head.iter().map(|b| format!("{b:02x}")).collect();
    let Some(stolen) = safe_prologue_len(head, ABS_JMP_LEN) else {
        // Not a failure to report quietly: these 16 bytes are exactly what a later pass needs.
        log(&format!(
            "instance: {module}!{name} at {target:#x} has a prologue this module will not \
             steal - NOT hooked, nothing written. first 16 bytes: {}",
            hex.join(" ")
        ));
        return None;
    };

    let tramp = VirtualAlloc(std::ptr::null_mut(), 64, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if tramp.is_null() {
        log(&format!("instance: VirtualAlloc failed - {name} NOT hooked"));
        return None;
    }
    let t = tramp.cast::<u8>();
    std::ptr::copy_nonoverlapping(target as *const u8, t, stolen);
    write_abs_jmp(t.add(stolen), target + stolen);

    let mut old = 0u32;
    if VirtualProtect(target as *mut c_void, stolen, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log(&format!("instance: VirtualProtect failed - {name} NOT hooked"));
        return None;
    }
    write_abs_jmp(target as *mut u8, detour);
    // NOP the remainder so a disassembler reading this later stays in step.
    for i in ABS_JMP_LEN..stolen {
        *(target as *mut u8).add(i) = 0x90;
    }
    VirtualProtect(target as *mut c_void, stolen, old, &mut old);
    log(&format!(
        "instance: hooked {module}!{name} at {target:#x}, stole {stolen} bytes ({})",
        hex.join(" ")
    ));
    Some(tramp as usize)
}

/// **A log line from inside a detour is written by ANOTHER thread, later.**
///
/// The regression of 2026-09-02, and the reason it is worth this much machinery. With
/// `CreateMutex` hooked, the FIRST client opened its window and then stopped - never reaching
/// character select. It was not dead: the session monitor kept writing its three-second
/// heartbeat for as long as the owner left it up. **One thread was stuck and the rest of the
/// process was fine**, which is what a lock held by a blocked thread looks like from outside.
///
/// The log named its own cause:
///
/// ```text
///   instance: CreateMutexW(name="Local\DirectSound DllMain mutex (0x000E27F0)")
/// ```
///
/// DirectSound creating a named mutex **from its `DllMain`** - so the loader lock is held -
/// and our detour answering it by opening and writing a file. Anything on that path that
/// needs the loader lock deadlocks against the thread already holding it, and file I/O has
/// plenty of ways to need it.
///
/// The two rules that fall out are the ones this module now follows:
///
/// * **A detour on a system API must not do I/O.** It records into memory with `try_lock`,
///   which cannot block, and a thread that owns nothing does the writing.
/// * **Match the name before doing anything at all.** 77 `CreateMutex` calls went through
///   here and exactly one of them mattered; the other 76 were DirectSound, DirectInput,
///   Internet Explorer's zone cache, and 73 unnamed ones that cannot be an instance guard
///   under any circumstances.
fn note(msg: String) {
    if let Ok(mut q) = PENDING.try_lock() {
        // Bounded, because a detour that is called in a tight loop must not grow a queue
        // faster than the flusher drains it. Losing a line is a cost this can pay; unbounded
        // memory inside somebody else's process is not.
        if q.len() < 256 {
            q.push(msg);
        }
    }
}

static PENDING: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

/// Start the thread that empties [`PENDING`].
///
/// Spawned from `arm`, which runs in `DllMain` - the same thing `crate::lib` already does for
/// the dispatcher hook. A thread created under the loader lock simply waits for it before its
/// own `DLL_THREAD_ATTACH` runs, which is exactly the delay wanted here.
fn start_flusher() {
    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_millis(200));
        let drained: Vec<String> = match PENDING.try_lock() {
            Ok(mut q) => q.drain(..).collect(),
            Err(_) => continue,
        };
        for m in drained {
            log(&m);
        }
    });
}

/// **Is this a name worth intervening on?**
///
/// `Global\WvsClientMtx` is the client's own single-instance mutex - `WvsClient` is Nexon's
/// codename for it, and it appeared exactly once, in the first client, at 00:24:56.547.
///
/// A substring match rather than an equality test on the full string, because the prefix is
/// a namespace (`Global\` here, `Local\` on some builds) and the point is to recognise the
/// client's own mutex however it is scoped. Everything else - DirectSound, DirectInput,
/// `ZonesCacheCounterMutex`, the obfuscated `CDdf212806D6EmB31yE0c` - is somebody else's
/// business and is passed through untouched and unlogged.
fn is_the_guard(name: &str) -> bool {
    name.contains("WvsClient")
}

/// **Are we already inside one of our own detours on this thread?**
///
/// `log` opens and writes a file. If anything on that path creates a named mutex, the detour
/// calls `log` again and the thread never returns. Nothing observed has done it - Windows file
/// I/O does not appear to create named mutexes - but the cost of being wrong is a hung client
/// with no error, which is exactly what this session spent two launches on already.
///
/// `try_with` rather than `with`: thread-local storage can be unavailable during thread
/// teardown, and a panic inside a `kernelbase` detour would be far worse than a missed log
/// line. Unavailable is treated as "assume re-entrant" and passes the call straight through.
fn in_our_own_code() -> bool {
    INSIDE.try_with(|c| c.replace(true)).unwrap_or(true)
}

/// Leave the detour, so the next call on this thread is logged again.
fn leaving() {
    let _ = INSIDE.try_with(|c| c.set(false));
}

thread_local! {
    static INSIDE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Read a NUL-terminated UTF-16 string for the log. `None` for a null pointer, which is a
/// perfectly ordinary argument to both of these APIs and must not be confused with a failure.
unsafe fn wide(p: *const u16) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let mut n = 0usize;
    while n < 260 && *p.add(n) != 0 {
        n += 1;
    }
    Some(String::from_utf16_lossy(std::slice::from_raw_parts(p, n)))
}

/// Read a NUL-terminated ANSI string for the log.
unsafe fn ansi(p: *const u8) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let mut n = 0usize;
    while n < 260 && *p.add(n) != 0 {
        n += 1;
    }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(p, n)).into_owned())
}

fn show(v: Option<String>) -> String {
    v.map(|s| format!("{s:?}")).unwrap_or_else(|| "NULL".into())
}

unsafe extern "system" fn find_window_w(class: *const u16, title: *const u16) -> *mut c_void {
    let suppress = SUPPRESS.load(Ordering::SeqCst);
    let found = if suppress {
        std::ptr::null_mut()
    } else {
        let real: unsafe extern "system" fn(*const u16, *const u16) -> *mut c_void =
            std::mem::transmute(FIND_W.load(Ordering::SeqCst) as usize);
        real(class, title)
    };
    // `note`, not `log`: a detour must not do I/O. `crate::instance::note` records why -
    // DirectSound calls a hooked API from its own DllMain, and a file write there deadlocks
    // against the loader lock. FindWindow has only ever been seen on the client's own thread,
    // but "only ever been seen" is not a property of the API.
    note(format!(
        "instance: FindWindowW(class={}, title={}) -> {}",
        show(wide(class)),
        show(wide(title)),
        if suppress { "forced NULL".to_string() } else { format!("{found:?} (passed through)") }
    ));
    found
}

unsafe extern "system" fn find_window_a(class: *const u8, title: *const u8) -> *mut c_void {
    let suppress = SUPPRESS.load(Ordering::SeqCst);
    let found = if suppress {
        std::ptr::null_mut()
    } else {
        let real: unsafe extern "system" fn(*const u8, *const u8) -> *mut c_void =
            std::mem::transmute(FIND_A.load(Ordering::SeqCst) as usize);
        real(class, title)
    };
    // `note`, not `log`: a detour must not do I/O. `crate::instance::note` records why -
    // DirectSound calls a hooked API from its own DllMain, and a file write there deadlocks
    // against the loader lock. FindWindow has only ever been seen on the client's own thread,
    // but "only ever been seen" is not a property of the API.
    note(format!(
        "instance: FindWindowA(class={}, title={}) -> {}",
        show(ansi(class)),
        show(ansi(title)),
        if suppress { "forced NULL".to_string() } else { format!("{found:?} (passed through)") }
    ));
    found
}

/// **An UNNAMED mutex is passed straight through, and that is not an optimisation.**
///
/// It is the fix for the regression of 2026-09-02: with these two exports hooked, the FIRST
/// client stopped reaching character select and hung with its window open. 77 `CreateMutex`
/// calls went through here, **73 of them unnamed**, the earliest one millisecond after
/// `DllMain` - which is to say while the loader is still initialising other DLLs, on several
/// threads, each one now opening and writing a file inside a `kernelbase` detour.
///
/// Two things are wrong with that and only one of them is speed:
///
/// * An unnamed mutex is process-local. It **cannot** tell anybody that another instance
///   exists, so it can never be the guard, and there is nothing here to decide about it.
/// * `already_existed` is meaningless for one. `CreateMutex` does not clear the last error on
///   success, so `GetLastError()` returns whatever the thread was carrying - which is why the
///   log has the impossible line `CreateMutexW(name=NULL) already_existed=true`. In a
///   suppressing client that stale reading would have cleared an error the caller was about
///   to act on.
///
/// So: named only. That is four calls instead of seventy-seven, and one of the four is the
/// answer.
unsafe extern "system" fn create_mutex_w(
    attrs: *mut c_void,
    owner: i32,
    name: *const u16,
) -> *mut c_void {
    let real: unsafe extern "system" fn(*mut c_void, i32, *const u16) -> *mut c_void =
        std::mem::transmute(MUTEX_W.load(Ordering::SeqCst) as usize);
    // Named, ours, and not re-entrant - three tests before anything happens, all of them
    // cheap and none of them touching a file. See `note`.
    let Some(text) = wide(name).filter(|n| is_the_guard(n)) else {
        return real(attrs, owner, name);
    };
    if in_our_own_code() {
        return real(attrs, owner, name);
    }
    let h = real(attrs, owner, name);
    let err = GetLastError();
    let existed = err == ERROR_ALREADY_EXISTS;
    let suppress = SUPPRESS.load(Ordering::SeqCst);
    // **`note` records; it does not write.** The pattern being defeated is
    // `CreateMutex(...); if (GetLastError() == ERROR_ALREADY_EXISTS) exit;`, so the value the
    // caller reads on its very next instruction is the only thing that matters - and
    // `SetLastError` is therefore the last thing this function does before returning.
    note(format!(
        "instance: CreateMutexW(name={text:?}) already_existed={existed}{}",
        if existed && suppress { " -> last error CLEARED, this client is a later one" } else if existed { " -> left alone (first instance)" } else { " -> we are the first to create it" }
    ));
    leaving();
    if existed && suppress {
        // The handle is still the right one - a second opener gets the same object. All the
        // caller must not see is the *error code* that tells it somebody was here first.
        SetLastError(0);
    } else {
        SetLastError(err);
    }
    h
}

/// Named only, and not re-entrant. See [`create_mutex_w`] for both reasons.
unsafe extern "system" fn create_mutex_a(
    attrs: *mut c_void,
    owner: i32,
    name: *const u8,
) -> *mut c_void {
    let real: unsafe extern "system" fn(*mut c_void, i32, *const u8) -> *mut c_void =
        std::mem::transmute(MUTEX_A.load(Ordering::SeqCst) as usize);
    let Some(text) = ansi(name).filter(|n| is_the_guard(n)) else {
        return real(attrs, owner, name);
    };
    if in_our_own_code() {
        return real(attrs, owner, name);
    }
    let h = real(attrs, owner, name);
    let err = GetLastError();
    let existed = err == ERROR_ALREADY_EXISTS;
    let suppress = SUPPRESS.load(Ordering::SeqCst);
    // Record, then set the error last. See `create_mutex_w` for why the order is the function.
    note(format!(
        "instance: CreateMutexA(name={text:?}) already_existed={existed}{}",
        if existed && suppress { " -> last error CLEARED, this client is a later one" } else if existed { " -> left alone (first instance)" } else { " -> we are the first to create it" }
    ));
    leaving();
    if existed && suppress {
        SetLastError(0);
    } else {
        SetLastError(err);
    }
    h
}

/// Arm the hooks. Safe to call more than once; only the first call does anything.
///
/// **Called inline from `DllMain`, not from a thread**, and that is the one thing about this
/// module that is not negotiable: the guard runs during the client's own startup, so a hook
/// installed five seconds later would be installed after the process it was meant to save had
/// already exited. `crate::hook` defers for good reasons that do not apply here - it patches
/// the client's own `.text`, which Themida has not finished unpacking at `DllMain`, whereas
/// `user32` and `kernel32` are mapped and final before any of our code runs.
pub unsafe fn arm() {
    if ARMED.swap(true, Ordering::SeqCst) {
        return;
    }
    // **Work out whether we are the first client, BEFORE hooking CreateMutex** - otherwise
    // this call would go through our own detour and the answer would depend on the order the
    // hooks happened to install in.
    let name: Vec<u16> = OUR_MUTEX.encode_utf16().chain(std::iter::once(0)).collect();
    let create: unsafe extern "system" fn(*mut c_void, i32, *const u16) -> *mut c_void = {
        let k = GetModuleHandleA(b"kernel32.dll\0".as_ptr());
        let f = GetProcAddress(k, b"CreateMutexW\0".as_ptr());
        std::mem::transmute(f)
    };
    let held = create(std::ptr::null_mut(), 0, name.as_ptr());
    let first = GetLastError() != ERROR_ALREADY_EXISTS;
    // The handle is deliberately never closed: it must outlive this function so that the NEXT
    // process finds the mutex still there. It is released when the process exits, which is
    // exactly the lifetime wanted.
    let _ = held;
    SUPPRESS.store(!first, Ordering::SeqCst);

    log(&format!(
        "instance: ARMING in pid {} - marker {MULTICLIENT_MARKER} present, this is the {} \
         client on this desktop, so the guard will be {}",
        GetCurrentProcessId(),
        if first { "FIRST" } else { "SECOND-OR-LATER" },
        if first { "LOGGED ONLY (first client behaves exactly as before)" } else { "SUPPRESSED" }
    ));
    // **Bound to typed fn pointers before being cast.** `crate::hook` records why: casting a
    // function *item* straight to an integer is a lint trap and can pick up the wrong
    // address. A detour installed at the wrong address is a jump into the middle of
    // something, in a DLL the whole process shares.
    start_flusher();

    let fw: unsafe extern "system" fn(*const u16, *const u16) -> *mut c_void = find_window_w;
    let fa: unsafe extern "system" fn(*const u8, *const u8) -> *mut c_void = find_window_a;
    let mw: unsafe extern "system" fn(*mut c_void, i32, *const u16) -> *mut c_void = create_mutex_w;
    let ma: unsafe extern "system" fn(*mut c_void, i32, *const u8) -> *mut c_void = create_mutex_a;

    // **All four need a trampoline now.** The first client passes every call through to the
    // real function, so the detours must be able to reach it - the earlier design, where
    // FindWindow simply returned NULL, would have changed the behaviour of the one client
    // that is currently working.
    if let Some(t) = hook("user32.dll", "FindWindowW", fw as usize) {
        FIND_W.store(t as u64, Ordering::SeqCst);
    }
    if let Some(t) = hook("user32.dll", "FindWindowA", fa as usize) {
        FIND_A.store(t as u64, Ordering::SeqCst);
    }
    // CreateMutex does: it must really create the mutex, and only the error code is changed.
    // The trampoline is stored BEFORE the patch goes in, or the first call through the detour
    // would read a zero and jump to nowhere.
    if let Some(t) = hook_forwarder_slot("kernel32.dll", "CreateMutexW", mw as usize) {
        MUTEX_W.store(t as u64, Ordering::SeqCst);
    }
    if let Some(t) = hook_forwarder_slot("kernel32.dll", "CreateMutexA", ma as usize) {
        MUTEX_A.store(t as u64, Ordering::SeqCst);
    }
}

/// [`hook`], for a detour that needs to call the original.
///
/// Split out only to make the ordering hazard explicit at the call site: the trampoline has to
/// be published to its atomic before the target is patched, because the very next call from
/// anywhere in the process goes through the detour.
/// **Redirect a forwarder by rewriting the pointer it reads - eight bytes of data, and not
/// one byte of code.**
///
/// Three runs of 2026-09-02, and the correlation is exact:
///
/// ```text
///   run     kernelbase!CreateMutex patched     packets the client dispatched
///   00:16              no                                10
///   00:24              yes                                0
///   00:34              yes                                0
/// ```
///
/// The first client opened its window and stopped. It was not dead - the session monitor kept
/// its three-second heartbeat going - and it never dispatched a single packet. The 00:34 run
/// is what makes this a measurement rather than a guess: it had the logging moved out of the
/// detours entirely, the name filter down to one mutex, and a clean uninterleaved log. **It
/// hung identically.** So the cause is the presence of the patch in `kernelbase`, not
/// anything our detour was doing.
///
/// Which is worth stating plainly: the previous fix was a real bug - a file write inside a
/// detour that DirectSound calls from its `DllMain` is a deadlock waiting to happen - and it
/// was not this bug. Fixing something real is not evidence of having fixed the thing you were
/// chasing.
///
/// # Why the slot is a different and much smaller act
///
/// `kernel32!CreateMutexW` is not a function. It is six bytes:
///
/// ```text
///   ff 25 0a 09 06 00     jmp qword ptr [rip+0x6090a]
/// ```
///
/// and the pointer it reads holds `kernelbase!CreateMutexW`. Writing our own address into
/// that pointer redirects **everyone who goes through `kernel32`** - which is the client,
/// because that is the documented export anybody links against - and leaves everyone who
/// calls `kernelbase` directly completely untouched. DirectSound's `DllMain` mutex, DirectInput,
/// the zone cache: all of them keep running the real function through the real code.
///
/// It also removes the whole instruction-stealing question. There is no prologue to measure,
/// nothing copied to a trampoline, and the "trampoline" is simply the address that was already
/// in the slot.
unsafe fn hook_forwarder_slot(module: &str, name: &str, detour: usize) -> Option<usize> {
    let module_z = format!("{module}\0");
    let name_z = format!("{name}\0");
    let m = GetModuleHandleA(module_z.as_ptr());
    if m.is_null() {
        log(&format!("instance: {module} is not loaded - {name} NOT hooked"));
        return None;
    }
    let thunk = GetProcAddress(m, name_z.as_ptr()) as usize;
    if thunk == 0 {
        log(&format!("instance: {module}!{name} not found - NOT hooked"));
        return None;
    }
    let b = std::slice::from_raw_parts(thunk as *const u8, 6);
    if b[0] != 0xFF || b[1] != 0x25 {
        log(&format!(
            "instance: {module}!{name} at {thunk:#x} is NOT an ff-25 forwarder, so there is no \
             pointer to rewrite - NOT hooked, nothing written. Patching its code instead is \
             what hung the client on 2026-09-02 and is deliberately not attempted. first 6 \
             bytes: {}",
            b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")
        ));
        return None;
    }
    // The displacement is signed and measured from the END of the six-byte instruction.
    let disp = i32::from_le_bytes([b[2], b[3], b[4], b[5]]) as isize;
    let slot = (thunk as isize + 6 + disp) as usize;
    let real = *(slot as *const usize);
    if real == 0 {
        log(&format!("instance: {module}!{name} forwarder slot is null - NOT hooked"));
        return None;
    }

    let mut old = 0u32;
    if VirtualProtect(slot as *mut c_void, 8, PAGE_READWRITE, &mut old) == 0 {
        log(&format!("instance: VirtualProtect on the {name} slot failed - NOT hooked"));
        return None;
    }
    *(slot as *mut usize) = detour;
    VirtualProtect(slot as *mut c_void, 8, old, &mut old);
    log(&format!(
        "instance: redirected {module}!{name} by rewriting its forwarder slot at {slot:#x} \
         ({real:#x} -> {detour:#x}). No code was modified; callers that reach kernelbase by \
         another route are unaffected"
    ));
    Some(real)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The prologue measurer must never return a length that splits an instruction.**
    ///
    /// This is the safety property of the whole module: the returned length is copied to a
    /// trampoline and then overwritten with a jump, in a DLL every thread in the process
    /// shares - including the client that is already running fine.
    #[test]
    fn a_measured_prologue_is_whole_instructions_and_long_enough() {
        // `mov [rsp+0x08], rbx` / `mov [rsp+0x10], rbp` / `push rdi` - a stock MSVC x64
        // prologue, 5 + 5 + 1 = 11, then `sub rsp, 0x40` takes it to 15.
        let p = [
            0x48, 0x89, 0x5C, 0x24, 0x08, // mov [rsp+8], rbx      5
            0x48, 0x89, 0x6C, 0x24, 0x10, // mov [rsp+0x10], rbp   5
            0x57, //                          push rdi             1
            0x48, 0x83, 0xEC, 0x40, //        sub rsp, 0x40        4
        ];
        assert_eq!(safe_prologue_len(&p, 12), Some(15));
        // Every prefix boundary it could have returned is a real instruction boundary.
        assert_eq!(safe_prologue_len(&p, 1), Some(5));
        assert_eq!(safe_prologue_len(&p, 5), Some(5));
        assert_eq!(safe_prologue_len(&p, 6), Some(10));
        assert_eq!(safe_prologue_len(&p, 11), Some(11));
    }

    /// **The two real prologues this machine actually has**, copied out of a hook log rather
    /// than invented.
    ///
    /// `FindWindowW` was hooked on the first try and `FindWindowA` was refused at 10 bytes,
    /// because `33 d2` / `33 c9` - zeroing edx and ecx before a call - were not in the
    /// whitelist. Synthetic prologues had passed for weeks; these two are the measurement.
    #[test]
    fn the_real_user32_prologues_from_this_machine_measure_correctly() {
        // 00:02:31.916  instance: hooked user32.dll!FindWindowW ... stole 14 bytes
        let find_w = [
            0x48, 0x89, 0x5C, 0x24, 0x08, // mov [rsp+8], rbx       5
            0x48, 0x89, 0x7C, 0x24, 0x10, // mov [rsp+0x10], rdi    5
            0x55, //                          push rbp              1
            0x48, 0x8B, 0xEC, //              mov rbp, rsp          3   = 14
            0x48, 0x83, //                    (sub rsp, ..) - NOT included
        ];
        assert_eq!(safe_prologue_len(&find_w, ABS_JMP_LEN), Some(14));

        // 00:02:31.917  instance: user32.dll!FindWindowA ... NOT hooked. This is the one the
        // xor entries were added for; it comes to exactly 14, one byte over what is needed.
        let find_a = [
            0x48, 0x83, 0xEC, 0x38, // sub rsp, 0x38    4
            0x4C, 0x8B, 0xCA, //       mov r9, rdx      3
            0x4C, 0x8B, 0xC1, //       mov r8, rcx      3
            0x33, 0xD2, //             xor edx, edx     2
            0x33, 0xC9, //             xor ecx, ecx     2   = 14
            0xE8, 0x75, //             (call ..) - NOT included
        ];
        // 12, not 14: `ABS_JMP_LEN` is 12 and the fourth instruction ENDS there, so nothing
        // extra is stolen. `FindWindowW` above needs 14 only because 12 falls inside its
        // `mov rbp, rsp`. Both are hookable; the difference is the point of measuring rather
        // than assuming a fixed steal.
        assert_eq!(
            safe_prologue_len(&find_a, ABS_JMP_LEN),
            Some(12),
            "the xor entries are what make this reachable at all"
        );
        // Every boundary is a real one - a measurer that returned 13 here would split
        // `xor ecx, ecx` and corrupt user32 for every thread in the process.
        for (want, expect) in [(1, 4), (5, 7), (8, 10), (11, 12), (12, 12), (13, 14)] {
            assert_eq!(safe_prologue_len(&find_a, want), Some(expect), "want {want}");
        }
        // And the control for the whole change: without the xor entries this stops at 10,
        // which is SHORT of the 12 an absolute jump needs - which is exactly why the real
        // FindWindowA came back "NOT hooked, nothing written".
        assert!(10 < ABS_JMP_LEN, "10 bytes is not enough to write the jump");
    }

    /// **The forwarder slot is the pointer AFTER the six-byte instruction**, and an off-by-six
    /// here rewrites whatever is next to it instead.
    ///
    /// Same arithmetic as `follow_thunk`, in the opposite direction: that one reads the slot,
    /// this one writes it. Getting it wrong does not fail - it corrupts a neighbouring import.
    #[test]
    fn a_forwarder_slot_is_found_at_the_same_place_it_is_read_from() {
        let mut landing = vec![0u8; 16];
        landing[0] = 0x48;
        let real = landing.as_ptr() as usize;

        let mut image = vec![0u8; 64];
        let base = image.as_ptr() as usize;
        let slot = base + 40;
        let disp = (slot as isize) - (base as isize + 6);
        image[0] = 0xFF;
        image[1] = 0x25;
        image[2..6].copy_from_slice(&(disp as i32).to_le_bytes());
        image[40..48].copy_from_slice(&real.to_le_bytes());

        // `follow_thunk` reads the slot, so agreeing with it is the check: a writer that
        // disagreed with the reader would patch an address nobody dispatches through.
        assert_eq!(unsafe { follow_thunk(base, "test") }, real);
        assert_eq!(
            unsafe { *(slot as *const usize) },
            real,
            "and the slot is where the value actually lives"
        );
    }

    /// **Only the client's own mutex is acted on**, and the list of what it must ignore is
    /// not hypothetical - these are the eight named mutexes one client actually created.
    ///
    /// The bug this pins is a hang, not a wrong answer. Touching `DirectSound DllMain mutex`
    /// meant doing work inside a detour while the loader lock was held, and the client stopped
    /// with its window open and its session monitor still ticking.
    #[test]
    fn only_the_clients_own_mutex_is_recognised() {
        assert!(is_the_guard(r"Global\WvsClientMtx"));
        // The namespace is a prefix and builds differ on it, so the match is on the name.
        assert!(is_the_guard(r"Local\WvsClientMtx"));
        assert!(is_the_guard("WvsClientMtx"));

        // Every other named mutex from the 00:24:56 capture. Acting on any of these is what
        // hung the first client.
        for other in [
            r"Local\DirectSound DllMain mutex (0x000E27F0)",
            "DirectSound Administrator shared thread array (lock",
            "DirectInput.{89521361-AA8A-11CF-BFC7-444553540000}",
            "DirectInput.{5944E682-C92E-11CF-BFC7-444553540000}",
            r"Local\ZonesCacheCounterMutex",
            r"Local\ZonesLockedCacheCounterMutex",
            "CDdf212806D6EmB31yE0c",
        ] {
            assert!(!is_the_guard(other), "{other} must be passed through untouched");
        }
    }

    /// The deferred log queue is bounded, drains, and never blocks its caller.
    #[test]
    fn notes_are_queued_and_bounded_rather_than_written() {
        for i in 0..300 {
            note(format!("line {i}"));
        }
        let q = PENDING.lock().unwrap();
        assert_eq!(q.len(), 256, "a detour must not grow a queue without limit");
        assert_eq!(q[0], "line 0", "and it keeps the EARLIEST, which is the interesting end");
    }

    /// **The kernelbase mutex prologue, from the hook log of 2026-09-02.**
    ///
    /// This is the export the second client's fate turns on. `kernel32!CreateMutexW` is an
    /// `ff 25` forwarder; following it lands here, and these are the bytes that were refused:
    ///
    /// ```text
    ///   49 8b c0            mov rax, r8            3
    ///   41 b9 01 00 1f 00   mov r9d, 0x1F0001      6   <- the one that was missing
    ///   45 33 c0            xor r8d, r8d           3   = 12, exactly ABS_JMP_LEN
    /// ```
    ///
    /// Both mutex exports have the identical prologue, which is itself worth noticing: they
    /// are two thin wrappers over the same `CreateMutexEx`.
    #[test]
    fn the_real_kernelbase_mutex_prologue_measures_to_exactly_the_jump_length() {
        let create_mutex = [
            0x49, 0x8B, 0xC0, // mov rax, r8          3
            0x41, 0xB9, 0x01, 0x00, 0x1F, 0x00, // mov r9d, 0x1F0001   6
            0x45, 0x33, 0xC0, // xor r8d, r8d         3   = 12
            0x85, 0xD2, //       test edx, edx
            0x48, 0x8B, //       (mov ..)
        ];
        assert_eq!(
            safe_prologue_len(&create_mutex, ABS_JMP_LEN),
            Some(12),
            "without the B8..BF entry this stops at 3 and the export is refused"
        );
        // Every boundary is real. 9 is the one that matters: a measurer that returned it
        // would cut `mov r9d, imm32` in half, in kernelbase, for every thread in the process.
        for (want, expect) in [(1, 3), (3, 3), (4, 9), (9, 9), (10, 12), (13, 14)] {
            assert_eq!(safe_prologue_len(&create_mutex, want), Some(expect), "want {want}");
        }
    }

    /// **REX.W changes how many bytes `B8+r` carries, and guessing costs a split
    /// instruction.**
    ///
    /// `41 b9 imm32` is six bytes; `48 b8 imm64` is ten. The real prologue above is the first
    /// form, so the second is untested by it - which is exactly why it is asserted here
    /// rather than left to be discovered on a different Windows build.
    #[test]
    fn a_rex_w_immediate_is_eight_bytes_wide_and_a_plain_one_is_four() {
        // 48 b8 = movabs rax, imm64
        let wide = [0x48, 0xB8, 1, 2, 3, 4, 5, 6, 7, 8, 0x90, 0x90, 0x90, 0x90];
        assert_eq!(safe_prologue_len(&wide, 1), Some(10));
        // 41 b9 = mov r9d, imm32
        let narrow = [0x41, 0xB9, 1, 2, 3, 4, 0x41, 0xB9, 5, 6, 7, 8, 0x90, 0x90];
        assert_eq!(safe_prologue_len(&narrow, 1), Some(6));
        assert_eq!(safe_prologue_len(&narrow, 7), Some(12));
        // No REX at all: b8 = mov eax, imm32, five bytes.
        let bare = [0xB8, 1, 2, 3, 4, 0xB8, 5, 6, 7, 8, 0x90, 0x90];
        assert_eq!(safe_prologue_len(&bare, 1), Some(5));
        assert_eq!(safe_prologue_len(&bare, 6), Some(10));
    }

    /// **An `ff 25` forwarder is still refused by the measurer** - following it is the only
    /// correct answer, and `hook` is what does that.
    ///
    /// This is the shape both `kernel32` mutex exports have. Copying six bytes carrying a
    /// RIP-relative displacement to a trampoline would produce a jump to an address that
    /// depends on where the trampoline happened to land.
    #[test]
    fn a_forwarder_thunk_is_never_stolen() {
        let thunk = [
            0xFF, 0x25, 0x0A, 0x09, 0x06, 0x00, // jmp qword ptr [rip+0x6090a]
            0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC,
        ];
        assert_eq!(safe_prologue_len(&thunk, ABS_JMP_LEN), None);
    }

    /// **`follow_thunk` lands on the function, not six bytes past the jump.**
    ///
    /// The displacement is measured from the END of the six-byte instruction, so the
    /// arithmetic is `addr + 6 + disp`. An off-by-six here does not fail loudly - it hooks
    /// whatever happens to sit at the wrong address, inside a DLL the whole process shares.
    #[test]
    fn following_a_forwarder_resolves_through_the_pointer_slot() {
        // A little image: six bytes of thunk, padding, then a pointer slot holding the
        // "real function" address. Boxed so it does not move.
        //
        // The destination is a REAL buffer rather than a made-up address, because
        // `follow_thunk` dereferences what it lands on to ask whether that is a thunk too.
        // A synthetic pointer makes this test crash the whole harness, which is how this was
        // found - and it is also the honest shape: in the client, the slot holds the address
        // of a mapped function in kernelbase.
        let mut landing = vec![0u8; 16];
        landing[0] = 0x48; // mov ... - anything that is not `ff 25`
        landing[1] = 0x89;
        let real = landing.as_ptr() as usize;

        let mut image = vec![0u8; 64];
        let base = image.as_ptr() as usize;
        let slot = base + 32;

        let disp = (slot as isize) - (base as isize + 6);
        image[0] = 0xFF;
        image[1] = 0x25;
        image[2..6].copy_from_slice(&(disp as i32).to_le_bytes());
        image[32..40].copy_from_slice(&real.to_le_bytes());

        assert_eq!(unsafe { follow_thunk(base, "test") }, real);

        // The control: an address that is NOT a thunk is returned untouched, or every
        // ordinary export would be chased through whatever its first bytes happened to be.
        let mut plain = vec![0u8; 16];
        plain[0] = 0x48;
        plain[1] = 0x89;
        let p = plain.as_ptr() as usize;
        assert_eq!(unsafe { follow_thunk(p, "test") }, p);
    }

    /// **An unrecognised prologue is refused, not guessed at.**
    #[test]
    fn an_unknown_opcode_refuses_rather_than_guessing() {
        // 0xE9 is a near jump - a perfectly common first byte for a hot-patched or forwarded
        // export, and exactly the case where stealing 12 bytes would be wrong.
        assert_eq!(safe_prologue_len(&[0xE9, 0, 0, 0, 0, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90], 12), None);
        // A truncated buffer must not read past its end.
        assert_eq!(safe_prologue_len(&[0x48, 0x89], 12), None);
        assert_eq!(safe_prologue_len(&[], 12), None);
    }

    /// **RIP-relative operands are refused**, because relocating one changes what it reads.
    ///
    /// `mod == 00, rm == 101` is RIP-relative on x64 - it is NOT `[rbp]`, which is the x86
    /// habit that makes this the easiest ModRM case to get wrong.
    #[test]
    fn a_rip_relative_instruction_is_never_stolen() {
        // 48 8B 05 <disp32>   mov rax, [rip+disp32]
        let rip = [0x48, 0x8B, 0x05, 0x11, 0x22, 0x33, 0x44, 0x90, 0x90, 0x90, 0x90, 0x90];
        assert_eq!(safe_prologue_len(&rip, 12), None);
        // The control: the same instruction with a register operand IS accepted, so the
        // refusal above is about the addressing mode and not about `48 8B`.
        let reg = [0x48, 0x8B, 0xC4, 0x48, 0x89, 0x5C, 0x24, 0x08, 0x48, 0x89, 0x6C, 0x24, 0x10];
        assert_eq!(safe_prologue_len(&reg, 12), Some(13));
    }

    /// The marker gates everything, and its name is the one the launcher writes.
    #[test]
    fn the_marker_is_a_file_beside_the_client() {
        assert_eq!(MULTICLIENT_MARKER, "maplecw-hook.multiclient");
        assert!(!MULTICLIENT_MARKER.contains('/') && !MULTICLIENT_MARKER.contains('\\'));
    }
}
