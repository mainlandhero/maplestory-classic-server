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

/// Whether the marker is present. Checked once, at `DllMain`.
pub fn enabled() -> bool {
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
                _ => return None,
            }
        }
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
    log(&format!(
        "instance: FindWindowW(class={}, title={}) -> {}",
        show(wide(class)),
        show(wide(title)),
        if suppress { "forced NULL".into() } else { format!("{found:?} (passed through)") }
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
    log(&format!(
        "instance: FindWindowA(class={}, title={}) -> {}",
        show(ansi(class)),
        show(ansi(title)),
        if suppress { "forced NULL".into() } else { format!("{found:?} (passed through)") }
    ));
    found
}

unsafe extern "system" fn create_mutex_w(
    attrs: *mut c_void,
    owner: i32,
    name: *const u16,
) -> *mut c_void {
    let real: unsafe extern "system" fn(*mut c_void, i32, *const u16) -> *mut c_void =
        std::mem::transmute(MUTEX_W.load(Ordering::SeqCst) as usize);
    let h = real(attrs, owner, name);
    let err = GetLastError();
    let existed = err == ERROR_ALREADY_EXISTS;
    let suppress = SUPPRESS.load(Ordering::SeqCst);
    if existed && suppress {
        // The handle is still the right one - a second opener gets the same object. All the
        // caller must not see is the *error code* that tells it somebody was here first.
        SetLastError(0);
    } else {
        // Restore whatever the real call set, since GetLastError() above does not disturb it
        // but the logging between here and the return could.
        SetLastError(err);
    }
    log(&format!(
        "instance: CreateMutexW(name={}) already_existed={existed}{}",
        show(wide(name)),
        if existed && suppress { " -> last error CLEARED" } else if existed { " -> left alone (first instance)" } else { "" }
    ));
    h
}

unsafe extern "system" fn create_mutex_a(
    attrs: *mut c_void,
    owner: i32,
    name: *const u8,
) -> *mut c_void {
    let real: unsafe extern "system" fn(*mut c_void, i32, *const u8) -> *mut c_void =
        std::mem::transmute(MUTEX_A.load(Ordering::SeqCst) as usize);
    let h = real(attrs, owner, name);
    let err = GetLastError();
    let existed = err == ERROR_ALREADY_EXISTS;
    let suppress = SUPPRESS.load(Ordering::SeqCst);
    if existed && suppress {
        // The handle is still the right one - a second opener gets the same object. All the
        // caller must not see is the *error code* that tells it somebody was here first.
        SetLastError(0);
    } else {
        // Restore whatever the real call set, since GetLastError() above does not disturb it
        // but the logging between here and the return could.
        SetLastError(err);
    }
    log(&format!(
        "instance: CreateMutexA(name={}) already_existed={existed}{}",
        show(ansi(name)),
        if existed && suppress { " -> last error CLEARED" } else if existed { " -> left alone (first instance)" } else { "" }
    ));
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
    if let Some(t) = hook_with_trampoline("kernel32.dll", "CreateMutexW", mw as usize) {
        MUTEX_W.store(t as u64, Ordering::SeqCst);
    }
    if let Some(t) = hook_with_trampoline("kernel32.dll", "CreateMutexA", ma as usize) {
        MUTEX_A.store(t as u64, Ordering::SeqCst);
    }
}

/// [`hook`], for a detour that needs to call the original.
///
/// Split out only to make the ordering hazard explicit at the call site: the trampoline has to
/// be published to its atomic before the target is patched, because the very next call from
/// anywhere in the process goes through the detour.
unsafe fn hook_with_trampoline(module: &str, name: &str, detour: usize) -> Option<usize> {
    hook(module, name, detour)
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
