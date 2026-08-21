//! Write our own crash dump, because Windows Error Reporting will not write one for this
//! client.
//!
//! # Why this exists, measured rather than assumed
//!
//! Every heap-corruption death in this project has been undiagnosable, because
//! `STATUS_HEAP_CORRUPTION` is raised at the *next* allocator walk rather than where the
//! damage happened. A full memory dump is the one instrument that fixes that, and for six
//! sightings none was ever captured.
//!
//! The standing explanation was that WER LocalDumps was misconfigured. It is not. On
//! 2026-08-21 the whole chain was tested end to end **without spending a client run**, by
//! building a decoy that does nothing but dereference null, naming it `MapleStory.exe` -
//! LocalDumps keys match on the executable's base name - and running it:
//!
//! ```text
//! HKLM\...\Windows Error Reporting  Disabled = 0        (key last written 12:21:01)
//! HKLM\...\LocalDumps\MapleStory.exe  -> ...\MapleCW\dumps, DumpType 2, DumpCount 2
//! decoy exit code 0xC0000005 at 15:26 -> dumps\MapleStory.exe.1092140.dmp, 9 406 954 bytes
//! ```
//!
//! So the registry is armed, the folder is writable, and the name matches. And yet the
//! real client raised `0xC0000005` at **13:49:56 that same day** - 88 minutes *after* WER
//! was switched on - and produced nothing at all. Same exception code, same executable
//! name, same machine, same hour. The configuration is not the variable; the client is.
//! It ships its own crash reporting (`CrashReportClient.exe` sits beside it, and it
//! uploads its own error log and call stack in `0x008F`/`0x0090`), and a process that
//! handles its own faults never reaches `WerFault`.
//!
//! That is `CLAUDE.md`'s rule about instruments, arrived at from the other side: the
//! instrument looked armed, *was* armed, and still could not see this particular subject.
//! Widening the configuration would not have helped. Changing the question did.
//!
//! # What this does instead
//!
//! [`crate::probe::veh`] is a vectored exception handler, and it **already sees the
//! fault** - it is what writes the `CLIENT FAULT` line naming `0xc0000005 at 0x140ce89d6`.
//! It just logged and returned. So the dump is written from there, by us, with no
//! dependence on WER at all.
//!
//! A vectored handler runs **first-chance**, before any of the client's own SEH frames, so
//! the dump is taken at the instruction that faulted rather than after the client has
//! unwound and swallowed it. For the heap corruption that is strictly better than anything
//! WER could have given us.
//!
//! # Two things to read carefully in the log
//!
//! Writing a dump from inside a process whose heap may be corrupt can itself fault, so
//! this logs **before** it starts and **after** it finishes, with the byte count:
//!
//! ```text
//! ***** CRASH DUMP: writing <path> ... (this can take a few seconds) *****
//! ***** CRASH DUMP: wrote <path>, 412 379 648 bytes *****
//! ```
//!
//! A first line with no second line means the dump attempt died partway - which is itself
//! evidence, and is not the same thing as never having tried. Those two were
//! indistinguishable before, and telling them apart is the whole point.
//!
//! Because this is first-chance, a dump can in principle be written for an access
//! violation the client would have gone on to handle. Only one `CLIENT FAULT` line has
//! ever been seen in a run, and it was fatal, so that is a small risk - but a dump is not
//! by itself proof that the process died. Read `client-exit.log` for that.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Set for the duration of a write, so a fault raised *by* `MiniDumpWriteDump` cannot send
/// us round again. A recursive dump attempt on a corrupt heap would never terminate.
static WRITING: AtomicBool = AtomicBool::new(false);

/// Dumps written this run. Full-memory dumps of this client are ~400 MB, so a repeating
/// fault must not be allowed to fill the disk.
static WRITTEN: AtomicU32 = AtomicU32::new(0);

/// How many dumps one run may write. Two, matching the `DumpCount` the LocalDumps key
/// uses, so the convention is the same whichever instrument produced the file.
const MAX_DUMPS: u32 = 2;

/// A file holding one line: where dumps should go. The launcher writes the repo's
/// `dumps\` into it, so our dumps land in the one folder the test plan tells the owner to look
/// in.
///
/// **A marker file, not an environment variable**, and that is not a style choice. The
/// client is started with `ShellExecute` because it carries an elevation manifest, and
/// `ShellExecute` does not pass `$env:` through to the child - which is why the launcher
/// already sets `maplecw-hook.enable`, `.probe` and `.session` the same way, and why
/// `MAPLECW_HOOK_LOG` is set and then never actually arrives. Reaching for the obvious
/// env var here would have put every dump somewhere other than where the owner was told to
/// look, and looked exactly like no dump at all.
pub const DUMP_DIR_MARKER: &str = "maplecw-hook.dumpdir";

/// Same thing by environment variable, for anything that runs the hook in-process (the
/// tests do). Checked after the marker.
pub const DUMP_DIR_ENV: &str = "MAPLECW_DUMP_DIR";

// `MINIDUMP_TYPE` bits. Full memory is the one that matters: without it the heap is not in
// the file, and the heap is the entire reason this exists.
const MINIDUMP_WITH_FULL_MEMORY: u32 = 0x0000_0002;
const MINIDUMP_WITH_HANDLE_DATA: u32 = 0x0000_0004;
const MINIDUMP_WITH_UNLOADED_MODULES: u32 = 0x0000_0020;
const MINIDUMP_WITH_FULL_MEMORY_INFO: u32 = 0x0000_0800;
const MINIDUMP_WITH_THREAD_INFO: u32 = 0x0000_1000;

const GENERIC_WRITE: u32 = 0x4000_0000;
const CREATE_ALWAYS: u32 = 2;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_HANDLE_VALUE: *mut c_void = usize::MAX as *mut c_void;

/// `MINIDUMP_EXCEPTION_INFORMATION`, **packed to 4 bytes**.
///
/// `minidumpapiset.h` wraps every `MINIDUMP_*` structure in `<pshpack4.h>`/`<poppack.h>`,
/// so on x64 this is **16 bytes with the pointer at offset 4** - not the 24 bytes with the
/// pointer at offset 8 that natural alignment produces. Dropping `packed(4)` puts
/// `exception_pointers` where dbghelp expects padding, and it then reads four bytes of
/// zero plus half a pointer as an address.
///
/// That is not a subtle failure but it is a silent one: `MiniDumpWriteDump` returns
/// `FALSE` with `ERROR_NOACCESS` **whatever the contents are** - synthetic pointers or a
/// live handler's, stack or heap, pseudo handle or a real one from `OpenProcess`, from the
/// faulting thread or another, at any dump type. All of those were measured, all failed
/// identically, and the constant result is what finally pointed at the layout rather than
/// the data. Packing it and changing nothing else turned the same call into a 22 MB dump.
#[repr(C, packed(4))]
struct MinidumpExceptionInformation {
    thread_id: u32,
    exception_pointers: *mut c_void,
    /// `FALSE`: the `EXCEPTION_POINTERS` are in *our* address space, which they are,
    /// because we are the faulting process. Passing `TRUE` here makes dbghelp read the
    /// pointer out of the target and produces a dump with no exception record at all.
    client_pointers: i32,
}

type MiniDumpWriteDumpFn = unsafe extern "system" fn(
    process: *mut c_void,
    process_id: u32,
    file: *mut c_void,
    dump_type: u32,
    exception_param: *const MinidumpExceptionInformation,
    user_stream_param: *const c_void,
    callback_param: *const c_void,
) -> i32;

extern "system" {
    fn LoadLibraryA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn GetCurrentProcess() -> *mut c_void;
    fn GetCurrentProcessId() -> u32;
    fn GetCurrentThreadId() -> u32;
    fn CreateFileW(
        name: *const u16,
        access: u32,
        share: u32,
        security: *mut c_void,
        disposition: u32,
        flags: u32,
        template: *mut c_void,
    ) -> *mut c_void;
    fn CloseHandle(handle: *mut c_void) -> i32;
    fn GetLastError() -> u32;
}

/// Where to put the dump.
///
/// [`DUMP_DIR_MARKER`] if the launcher wrote one, then [`DUMP_DIR_ENV`], then beside the
/// hook log, then the working directory. Falling back rather than giving up is
/// deliberate: a dump in an unexpected place is recoverable, a dump that was never written
/// is not.
fn dump_dir() -> String {
    if let Ok(dir) = std::fs::read_to_string(DUMP_DIR_MARKER) {
        let dir = dir.trim().to_string();
        if !dir.is_empty() {
            return dir;
        }
    }
    if let Ok(dir) = std::env::var(DUMP_DIR_ENV) {
        if !dir.trim().is_empty() {
            return dir;
        }
    }
    // The hook log's directory. `log_path` is often the bare file name, in which case
    // there is no parent and the working directory is already the right answer.
    let log = crate::hook::log_path();
    match std::path::Path::new(&log).parent() {
        Some(p) if !p.as_os_str().is_empty() => p.to_string_lossy().into_owned(),
        _ => ".".to_string(),
    }
}

/// Write a full-memory dump for the exception now being raised.
///
/// `info` is the `EXCEPTION_POINTERS` the vectored handler was given, and `code` the
/// exception code, used only to name the file. Returns quietly when the cap is reached or
/// a write is already in progress; every other outcome is logged, including the failures,
/// because "no dump" has to be able to say *why*.
///
/// # Safety
///
/// `info` must be the live `EXCEPTION_POINTERS` for the current exception on the current
/// thread. It is handed straight to `MiniDumpWriteDump`.
pub unsafe fn write_crash_dump(info: *mut c_void, code: u32) {
    if info.is_null() {
        return;
    }
    // A fault inside the dump writer must not start another dump.
    if WRITING.swap(true, Ordering::SeqCst) {
        return;
    }
    let n = WRITTEN.fetch_add(1, Ordering::SeqCst) + 1;
    if n > MAX_DUMPS {
        WRITING.store(false, Ordering::SeqCst);
        return;
    }

    let dbghelp = LoadLibraryA(c"dbghelp.dll".as_ptr().cast());
    if dbghelp.is_null() {
        crate::hook::log(
            "***** CRASH DUMP: dbghelp.dll would not load - no dump. This is the \
             instrument failing, not the client *****",
        );
        WRITING.store(false, Ordering::SeqCst);
        return;
    }
    let proc = GetProcAddress(dbghelp, c"MiniDumpWriteDump".as_ptr().cast());
    if proc.is_null() {
        crate::hook::log(
            "***** CRASH DUMP: dbghelp.dll has no MiniDumpWriteDump - no dump *****",
        );
        WRITING.store(false, Ordering::SeqCst);
        return;
    }
    let write_dump: MiniDumpWriteDumpFn = std::mem::transmute(proc);

    let pid = GetCurrentProcessId();
    let path = format!(
        "{}\\maplecw-crash-{pid}-{code:08x}-{n}.dmp",
        dump_dir().trim_end_matches(['\\', '/'])
    );
    let wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

    let file = CreateFileW(
        wide.as_ptr(),
        GENERIC_WRITE,
        0,
        std::ptr::null_mut(),
        CREATE_ALWAYS,
        FILE_ATTRIBUTE_NORMAL,
        std::ptr::null_mut(),
    );
    if file == INVALID_HANDLE_VALUE || file.is_null() {
        crate::hook::log(&format!(
            "***** CRASH DUMP: could not create {path} (error {}) - no dump *****",
            GetLastError()
        ));
        WRITING.store(false, Ordering::SeqCst);
        return;
    }

    let exc = MinidumpExceptionInformation {
        thread_id: GetCurrentThreadId(),
        exception_pointers: info,
        client_pointers: 0,
    };

    // Logged BEFORE the call. A full-memory dump of this client is a few hundred megabytes
    // and takes seconds, and it is being taken from a process that may have a corrupt heap
    // - so it can die partway. A first line with no second line says exactly that, and is
    // not the same thing as never having tried.
    crate::hook::log(&format!(
        "***** CRASH DUMP: writing {path} ... (full memory, this can take a few seconds) \
         *****"
    ));

    let ok = write_dump(
        GetCurrentProcess(),
        pid,
        file,
        MINIDUMP_WITH_FULL_MEMORY
            | MINIDUMP_WITH_HANDLE_DATA
            | MINIDUMP_WITH_UNLOADED_MODULES
            | MINIDUMP_WITH_FULL_MEMORY_INFO
            | MINIDUMP_WITH_THREAD_INFO,
        std::ptr::addr_of!(exc),
        std::ptr::null(),
        std::ptr::null(),
    );
    let err = if ok == 0 { GetLastError() } else { 0 };
    CloseHandle(file);

    if ok == 0 {
        crate::hook::log(&format!(
            "***** CRASH DUMP: MiniDumpWriteDump FAILED for {path} (error {err:#x}) *****"
        ));
    } else {
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        crate::hook::log(&format!(
            "***** CRASH DUMP: wrote {path}, {size} bytes. Open it with WinDbg; \
             `!analyze -v` and `!heap -p -a <addr>` are the two that matter here *****"
        ));
    }
    WRITING.store(false, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **16 bytes, aligned 4** - the `pshpack4` layout, not the natural one.
    ///
    /// This test was first written asserting 24 and 8, which is what `repr(C)` gives and
    /// what the field list looks like it should give. It passed, and the dump writer did
    /// not work: `MiniDumpWriteDump` returned `ERROR_NOACCESS` on every single call that
    /// carried exception information. A test that pins the number the code already
    /// produces confirms rather than checks, which is the whole failure `CLAUDE.md`
    /// describes - so these two numbers come from the run that measured them, where
    /// `packed(4)` wrote 22 MB and `repr(C)` wrote nothing.
    #[test]
    fn exception_information_is_packed_to_four_bytes() {
        assert_eq!(std::mem::size_of::<MinidumpExceptionInformation>(), 16);
        assert_eq!(std::mem::align_of::<MinidumpExceptionInformation>(), 4);
    }

    /// Full memory is the bit that carries the heap, and the heap is the entire reason
    /// this module exists. A dump without it cannot answer the question it was written for.
    #[test]
    fn the_dump_type_includes_full_memory() {
        let flags = MINIDUMP_WITH_FULL_MEMORY
            | MINIDUMP_WITH_HANDLE_DATA
            | MINIDUMP_WITH_UNLOADED_MODULES
            | MINIDUMP_WITH_FULL_MEMORY_INFO
            | MINIDUMP_WITH_THREAD_INFO;
        assert_eq!(flags & MINIDUMP_WITH_FULL_MEMORY, MINIDUMP_WITH_FULL_MEMORY);
        assert_eq!(flags, 0x1826);
    }

    /// The cap exists so a fault that repeats cannot fill the disk with 400 MB files, and
    /// it matches the `DumpCount` on the LocalDumps key so both instruments behave alike.
    #[test]
    fn the_dump_cap_matches_the_localdumps_convention() {
        assert_eq!(MAX_DUMPS, 2);
    }

    /// With no environment variable and no directory in the log path, the working
    /// directory is the answer - never an empty string, which `CreateFileW` would reject
    /// and which would look exactly like the client refusing to dump.
    #[test]
    fn the_dump_directory_is_never_empty() {
        std::env::remove_var(DUMP_DIR_ENV);
        assert!(!dump_dir().is_empty());
    }

    /// **The instrument itself, driven end to end.**
    ///
    /// This module exists because an instrument that looked armed could not see its
    /// subject, so shipping a *replacement* instrument without exercising it would be the
    /// same mistake with a different name. This runs the real `write_crash_dump` - the
    /// real `LoadLibraryA`, the real `GetProcAddress`, the real struct, the real
    /// `MiniDumpWriteDump` - and checks a file comes out with a minidump header on it.
    ///
    /// The exception pointers are synthetic: a genuine `CONTEXT` captured here, and an
    /// `EXCEPTION_RECORD` we fill in. dbghelp does not care how the record was produced -
    /// it serialises what it is handed - so everything on our side of the call is covered.
    /// What this cannot cover is a *hardware* fault arriving on a thread with a corrupt
    /// heap, which is the client's job to provide.
    #[test]
    fn it_actually_writes_a_readable_minidump() {
        #[repr(C, align(16))]
        struct Ctx([u8; 1232]);
        #[repr(C)]
        struct Record {
            code: u32,
            flags: u32,
            next: *mut c_void,
            address: *mut c_void,
            number_parameters: u32,
            information: [usize; 15],
        }
        #[repr(C)]
        struct Pointers {
            record: *mut Record,
            context: *mut Ctx,
        }
        extern "system" {
            fn RtlCaptureContext(ctx: *mut c_void);
        }

        let dir = std::env::temp_dir().join("maplecw-minidump-selftest");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch dir");
        std::env::set_var(DUMP_DIR_ENV, &dir);
        // The counters are process-wide and other tests must not be able to exhaust them.
        WRITTEN.store(0, Ordering::SeqCst);
        WRITING.store(false, Ordering::SeqCst);

        let mut ctx = Ctx([0; 1232]);
        // CONTEXT_ALL for x64. RtlCaptureContext fills the rest.
        ctx.0[0x30..0x34].copy_from_slice(&0x0010_003fu32.to_le_bytes());
        let mut record = Record {
            code: 0xC000_0005,
            flags: 0,
            next: std::ptr::null_mut(),
            address: it_actually_writes_a_readable_minidump as *mut c_void,
            number_parameters: 0,
            information: [0; 15],
        };
        // SAFETY: `ctx` is the documented size and alignment for an x64 CONTEXT, and the
        // pointers below outlive the call.
        unsafe {
            RtlCaptureContext(std::ptr::addr_of_mut!(ctx).cast());
            let mut pointers = Pointers {
                record: &mut record,
                context: &mut ctx,
            };
            write_crash_dump(std::ptr::addr_of_mut!(pointers).cast(), 0xC000_0005);
        }

        let written: Vec<_> = std::fs::read_dir(&dir)
            .expect("scratch dir")
            .filter_map(Result::ok)
            .map(|e| e.path())
            .collect();
        assert_eq!(
            written.len(),
            1,
            "expected exactly one dump in {dir:?}, got {written:?}"
        );
        let bytes = std::fs::read(&written[0]).expect("read the dump back");
        // A full-memory dump of the test runner is small but never trivial, and the magic
        // is the only cheap proof that dbghelp wrote a dump rather than an empty file.
        assert_eq!(&bytes[..4], b"MDMP", "not a minidump: {:?}", &bytes[..4]);
        assert!(
            bytes.len() > 64 * 1024,
            "dump is {} bytes - too small to contain memory",
            bytes.len()
        );

        std::env::remove_var(DUMP_DIR_ENV);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
