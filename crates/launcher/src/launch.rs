//! Starting `MapleStory.exe`, and saying something useful when that fails.
//!
//! **`ShellExecuteExW`, not `std::process::Command`.** The client carries an elevation
//! manifest, so `CreateProcess` - which is what `Command` uses - fails outright with
//! `ERROR_ELEVATION_REQUIRED` (740). `tools/test-server.ps1` uses `Start-Process` for the
//! same reason and says so in a comment above the call. ShellExecute is the API that consults
//! the manifest and raises the UAC prompt.
//!
//! The `Ex` form rather than plain `ShellExecuteW` for exactly one reason: `SEE_MASK_NOCLOSEPROCESS`
//! hands back the client's process handle, and the pid is the only thing that tells two
//! clients on one machine apart - see [`launch_for_pid`]. There is deliberately **one** launch
//! path, not two: a second one would be the way the client is started on the machine nobody
//! tests, which is the same trade `crate::http` refuses for sign-in.
//!
//! Declared with a raw `extern "system"` block the way `crates/grap-stub` does its FFI, so
//! this costs no dependency.

use std::ffi::c_void;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

const SW_SHOWNORMAL: i32 = 1;

#[link(name = "user32")]
extern "system" {
    fn MessageBoxW(hwnd: *mut c_void, text: *const u16, caption: *const u16, kind: u32) -> i32;
}

/// A NUL-terminated UTF-16 string, which is what every `W` entry point wants.
pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Turn an argument list into the single command-line string ShellExecute takes.
///
/// Only quoting is needed - none of the arguments this launcher passes contains a quote or a
/// backslash-before-quote, so the full CommandLineToArgvW escaping dance would be dead code
/// dressed up as rigour. An argument that would need it is refused loudly by the caller
/// instead (see [`quote_args`]'s tests).
pub fn quote_args(args: &[String]) -> String {
    args.iter()
        .map(|a| {
            if a.is_empty() || a.contains(' ') || a.contains('\t') {
                format!("\"{a}\"")
            } else {
                a.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Map a `ShellExecuteEx` error code to something a person can act on.
///
/// On failure `ShellExecuteExW` leaves the same `HINSTANCE`-shaped code in `hInstApp` that
/// plain `ShellExecuteW` would have returned, so this table is unchanged by the move to the
/// `Ex` form.
///
/// The one that matters most in practice is **5**: that is what comes back when the UAC
/// prompt is dismissed, and "access denied" on its own sends people looking at file
/// permissions.
pub fn describe_shell_error(code: isize) -> String {
    let detail = match code {
        0 => "the operating system is out of memory or resources",
        2 => "the file was not found (SE_ERR_FNF) - check the client path above",
        3 => "the path was not found (SE_ERR_PNF) - check the client path above",
        5 => {
            "access denied (SE_ERR_ACCESSDENIED). The usual cause is the UAC prompt being \
             dismissed: the client has an elevation manifest, so Windows asks every launch"
        }
        8 => "not enough memory to start the client (SE_ERR_OOM)",
        11 => "the .exe is not a valid Win32 image (ERROR_BAD_FORMAT)",
        26 => "a sharing violation (SE_ERR_SHARE) - is the client already running?",
        27 => "the file association is incomplete or invalid (SE_ERR_ASSOCINCOMPLETE)",
        28 => "the DDE transaction timed out (SE_ERR_DDETIMEOUT)",
        29 => "the DDE transaction failed (SE_ERR_DDEFAIL)",
        30 => "the DDE transaction could not be completed, DDE was busy (SE_ERR_DDEBUSY)",
        31 => "no application is associated with this file (SE_ERR_NOASSOC)",
        32 => "a required DLL was not found (SE_ERR_DLLNOTFOUND)",
        _ => return format!("ShellExecuteEx failed with code {code}"),
    };
    format!("ShellExecuteEx failed with code {code}: {detail}")
}

// ------------------------------------------------------- starting the client, with its pid

/// `SHELLEXECUTEINFOW`. Laid out by hand, like `OpenFileNameW` below and for the same reason.
///
/// **`cbSize` is `size_of` rather than a literal**, exactly as the file picker's is: Windows
/// switches struct version on it and a wrong value is a failure that looks like a user
/// cancelling. The union at the end is `hIcon`/`hMonitor`, both handles, so one pointer-sized
/// field covers it.
#[repr(C)]
struct ShellExecuteInfoW {
    cb_size: u32,
    mask: u32,
    hwnd: *mut c_void,
    verb: *const u16,
    file: *const u16,
    parameters: *const u16,
    directory: *const u16,
    show: i32,
    inst_app: *mut c_void,
    id_list: *mut c_void,
    class: *const u16,
    hkey_class: *mut c_void,
    hot_key: u32,
    icon_or_monitor: *mut c_void,
    process: *mut c_void,
}

/// Return the process handle instead of closing it. Without this `hProcess` is always null and
/// there is no pid to register.
const SEE_MASK_NOCLOSEPROCESS: u32 = 0x0000_0040;
/// Wait for the shell operation to finish rather than returning while it is still starting.
/// This runs on a worker thread with no message pump, which is the case the flag documents.
const SEE_MASK_NOASYNC: u32 = 0x0000_0100;

#[link(name = "shell32")]
extern "system" {
    fn ShellExecuteExW(info: *mut ShellExecuteInfoW) -> i32;
}

#[link(name = "kernel32")]
extern "system" {
    fn GetProcessId(process: *mut c_void) -> u32;
    fn CloseHandle(handle: *mut c_void) -> i32;
}

/// What a launch produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launched {
    /// The client's process id, if Windows handed one back.
    ///
    /// **`None` is a real possibility and is not an error.** `ShellExecuteEx` documents
    /// `hProcess` as being set only when the operation actually started a process, and the
    /// client carries an elevation manifest, so the launch goes through the consent UI. That
    /// path has **not** been measured here - measuring it costs one of the owner's manual client
    /// launches, which `CLAUDE.md` says to spend only when nothing cheaper answers the
    /// question - so the code treats a null handle as ordinary and the launcher says out loud
    /// that the launch could not be registered. See [`Launched::describe`].
    pub pid: Option<u32>,
}

impl Launched {
    /// The line to print. **The `None` case says what it costs**, because "no pid" is silent
    /// on screen and only bites when a second person is signed in.
    pub fn describe(&self) -> String {
        match self.pid {
            Some(pid) => format!("client started as process {pid}"),
            None => "client started, but Windows did not report its process id - most likely \
                     the elevation prompt started it out of process. This launch cannot be \
                     registered, so if somebody else is signed in on this machine the game \
                     will be served as the server's fallback account rather than yours"
                .to_string(),
        }
    }
}

/// Start the client and report the process id, so the launch can be registered with the
/// server.
///
/// The pid is the only thing that tells two clients on one machine apart: the client sends
/// nothing per-launch (measured), and the address is shared. `store::peerowner` recovers the
/// same number on the server side from the accepted socket, so the two ends meet without the
/// client having to carry anything.
///
/// `ShellExecuteExW` rather than `ShellExecuteW` purely for `hProcess`. Everything else - the
/// elevation manifest, the working directory, the arguments - is identical, which is
/// deliberate: this is not a second way of launching the client, it is the same one with the
/// handle kept.
pub fn launch_for_pid(exe: &Path, args: &[String], working_dir: &Path) -> Result<Launched, String> {
    if !exe.is_file() {
        return Err(format!("no client executable at {}", exe.display()));
    }
    let op = to_wide("open");
    let file = to_wide(&exe.to_string_lossy());
    let params = to_wide(&quote_args(args));
    let dir = to_wide(&working_dir.to_string_lossy());

    let mut info = ShellExecuteInfoW {
        cb_size: std::mem::size_of::<ShellExecuteInfoW>() as u32,
        mask: SEE_MASK_NOCLOSEPROCESS | SEE_MASK_NOASYNC,
        hwnd: std::ptr::null_mut(),
        verb: op.as_ptr(),
        file: file.as_ptr(),
        parameters: params.as_ptr(),
        directory: dir.as_ptr(),
        show: SW_SHOWNORMAL,
        inst_app: std::ptr::null_mut(),
        id_list: std::ptr::null_mut(),
        class: std::ptr::null(),
        hkey_class: std::ptr::null_mut(),
        hot_key: 0,
        icon_or_monitor: std::ptr::null_mut(),
        process: std::ptr::null_mut(),
    };

    // SAFETY: every string buffer above is NUL-terminated and outlives the call, and `info`
    // is a live exclusive borrow for its duration.
    let ok = unsafe { ShellExecuteExW(&mut info) };
    if ok == 0 {
        // On failure `hInstApp` carries the same code `ShellExecuteW` would have returned, so
        // the error messages stay identical between the two paths.
        return Err(describe_shell_error(info.inst_app as isize));
    }

    if info.process.is_null() {
        return Ok(Launched { pid: None });
    }
    // SAFETY: a non-null process handle we own because of SEE_MASK_NOCLOSEPROCESS.
    let pid = unsafe { GetProcessId(info.process) };
    // The handle is ours to close and we do not wait on the client - the launcher's job ends
    // here, and leaking a handle per launch would keep a zombie entry alive for the life of
    // the launcher window.
    unsafe { CloseHandle(info.process) };
    // GetProcessId returns 0 on failure, and 0 is the System Idle Process, so it can never be
    // a real client. Report it as "no pid" rather than registering a number that means
    // "unset" - the same reasoning `store::migration::random_seed` uses for never minting 0.
    Ok(Launched { pid: (pid != 0).then_some(pid) })
}

const MB_ICONERROR: u32 = 0x10;
const MB_ICONINFORMATION: u32 = 0x40;

/// A last-resort message box, for a failure that happens before there is a window to print
/// into. In a release build the launcher is a `windows` subsystem binary with no console, so
/// without this a startup failure is completely silent.
pub fn message_box(caption: &str, text: &str) {
    show_box(caption, text, MB_ICONERROR);
}

/// The same, for output rather than failure - `--print-paths` in a release build has nowhere
/// else to go.
pub fn info_box(caption: &str, text: &str) {
    show_box(caption, text, MB_ICONINFORMATION);
}

const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
const STD_ERROR_HANDLE: u32 = -12i32 as u32;

/// Can `println!` be seen?
///
/// A `windows`-subsystem process started from a console inherits that console's standard
/// handles, so printing works and popping a modal box would be worse than useless - it would
/// block a script. Started from Explorer or a shortcut there are no handles at all.
///
/// This is not only about where the text goes. **`println!` panics** when the handle is
/// missing - "failed printing to stdout" - and in a panic hook that is a double panic, which
/// aborts the process and prints nothing anywhere. So both callers check first.
pub fn stdout_is_usable() -> bool {
    std_handle_usable(STD_OUTPUT_HANDLE)
}

/// The same for `eprintln!`. Separate because a console can hand a process one and not the
/// other - a redirect of only one stream is ordinary.
pub fn stderr_is_usable() -> bool {
    std_handle_usable(STD_ERROR_HANDLE)
}

fn std_handle_usable(which: u32) -> bool {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetStdHandle(which: u32) -> *mut c_void;
    }
    // SAFETY: one integer argument; the returned handle is only compared, never used.
    let handle = unsafe { GetStdHandle(which) };
    !handle.is_null() && handle as isize != -1
}

fn show_box(caption: &str, text: &str, kind: u32) {
    let text = to_wide(text);
    let caption = to_wide(caption);
    // SAFETY: two NUL-terminated UTF-16 buffers that outlive the call.
    unsafe {
        MessageBoxW(std::ptr::null_mut(), text.as_ptr(), caption.as_ptr(), kind);
    }
}

// ---------------------------------------------------------------- the file picker

#[link(name = "comdlg32")]
extern "system" {
    fn GetOpenFileNameW(lpofn: *mut OpenFileNameW) -> i32;
}

/// `OPENFILENAMEW`. Laid out by hand for the same reason every other Win32 struct here is:
/// this workspace declares its FFI rather than taking a dependency for it.
///
/// **The size field is load-bearing and is `size_of` rather than a literal.** Windows
/// switches struct version on it, and a wrong value is `CDERR_STRUCTSIZE` - which
/// `GetOpenFileNameW` reports by returning 0, exactly as a user pressing Cancel does. A
/// hard-coded number here would make "the dialog is broken" and "you cancelled" the same
/// answer, which is the class of mistake `CLAUDE.md` is full of.
#[repr(C)]
struct OpenFileNameW {
    l_struct_size: u32,
    hwnd_owner: *mut c_void,
    h_instance: *mut c_void,
    lpstr_filter: *const u16,
    lpstr_custom_filter: *mut u16,
    n_max_cust_filter: u32,
    n_filter_index: u32,
    lpstr_file: *mut u16,
    n_max_file: u32,
    lpstr_file_title: *mut u16,
    n_max_file_title: u32,
    lpstr_initial_dir: *const u16,
    lpstr_title: *const u16,
    flags: u32,
    n_file_offset: u16,
    n_file_extension: u16,
    lpstr_def_ext: *const u16,
    l_cust_data: isize,
    lpfn_hook: *mut c_void,
    lp_template_name: *const u16,
    pv_reserved: *mut c_void,
    dw_reserved: u32,
    flags_ex: u32,
}

const OFN_FILEMUSTEXIST: u32 = 0x0000_1000;
const OFN_PATHMUSTEXIST: u32 = 0x0000_0800;
const OFN_NOCHANGEDIR: u32 = 0x0000_0008;

/// Ask the player to point at `MapleStory.exe`.
///
/// `None` means they cancelled **or** the dialog failed - the two are genuinely
/// indistinguishable through this API without `CommDlgExtendedError`, and neither is worth
/// interrupting anybody over: the field stays as it was and they can type a path instead.
///
/// `OFN_NOCHANGEDIR` matters more than it looks. Without it the common dialog changes the
/// **process** working directory, and this process later launches the client with a working
/// directory of its own - a stale cwd is the kind of thing that produces a failure three
/// steps from its cause.
pub fn pick_client_exe(start_in: Option<&Path>) -> Option<PathBuf> {
    // "Label\0pattern\0...\0\0" - a double NUL ends the list.
    let filter: Vec<u16> = "MapleStory.exe\0MapleStory.exe\0Executables\0*.exe\0All files\0*.*\0\0"
        .encode_utf16()
        .collect();
    let title: Vec<u16> = "Where is MapleStory.exe?\0".encode_utf16().collect();
    let initial: Option<Vec<u16>> = start_in.map(|p| {
        let mut v: Vec<u16> = p.as_os_str().encode_wide().collect();
        v.push(0);
        v
    });

    let mut buf = vec![0u16; 1024];
    let mut ofn = OpenFileNameW {
        l_struct_size: std::mem::size_of::<OpenFileNameW>() as u32,
        hwnd_owner: std::ptr::null_mut(),
        h_instance: std::ptr::null_mut(),
        lpstr_filter: filter.as_ptr(),
        lpstr_custom_filter: std::ptr::null_mut(),
        n_max_cust_filter: 0,
        n_filter_index: 1,
        lpstr_file: buf.as_mut_ptr(),
        n_max_file: buf.len() as u32,
        lpstr_file_title: std::ptr::null_mut(),
        n_max_file_title: 0,
        lpstr_initial_dir: initial.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
        lpstr_title: title.as_ptr(),
        flags: OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_NOCHANGEDIR,
        n_file_offset: 0,
        n_file_extension: 0,
        lpstr_def_ext: std::ptr::null(),
        l_cust_data: 0,
        lpfn_hook: std::ptr::null_mut(),
        lp_template_name: std::ptr::null(),
        pv_reserved: std::ptr::null_mut(),
        dw_reserved: 0,
        flags_ex: 0,
    };

    // SAFETY: every pointer above outlives the call, and `buf` is the documented size.
    let ok = unsafe { GetOpenFileNameW(&mut ofn) };
    if ok == 0 {
        return None;
    }
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    Some(PathBuf::from(std::ffi::OsString::from_wide(&buf[..end])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_strings_are_nul_terminated() {
        let w = to_wide("ab");
        assert_eq!(w, vec![b'a' as u16, b'b' as u16, 0]);
        assert_eq!(to_wide(""), vec![0]);
    }

    #[test]
    fn wide_strings_survive_a_non_ascii_path() {
        // A test machine's user name is not guaranteed to be ASCII, and this is the only
        // place a path leaves Rust.
        let w = to_wide("C:\\Users\\Zoë\\game");
        assert_eq!(*w.last().unwrap(), 0);
        let back = String::from_utf16(&w[..w.len() - 1]).unwrap();
        assert_eq!(back, "C:\\Users\\Zoë\\game");
    }

    #[test]
    fn the_launch_arguments_are_the_ones_test_server_ps1_passes() {
        let args = vec!["-NXLDEBUG".to_string(), "127.0.0.1".into(), "8484".into()];
        assert_eq!(quote_args(&args), "-NXLDEBUG 127.0.0.1 8484");
    }

    #[test]
    fn arguments_with_spaces_are_quoted() {
        let args = vec!["-NXLDEBUG".to_string(), "a b".into(), "".into()];
        assert_eq!(quote_args(&args), "-NXLDEBUG \"a b\" \"\"");
    }

    #[test]
    fn the_uac_case_says_uac() {
        let msg = describe_shell_error(5);
        assert!(msg.contains("UAC"), "{msg}");
        assert!(msg.contains('5'), "{msg}");
    }

    #[test]
    fn each_documented_code_maps_to_its_own_message() {
        let mut seen = std::collections::HashSet::new();
        for code in [0isize, 2, 3, 5, 8, 11, 26, 27, 28, 29, 30, 31, 32] {
            let msg = describe_shell_error(code);
            assert!(msg.contains(&code.to_string()), "{code}: {msg}");
            assert!(seen.insert(msg.clone()), "{code} duplicated a message: {msg}");
        }
    }

    #[test]
    fn an_unknown_code_still_reports_the_number() {
        let msg = describe_shell_error(1234);
        assert!(msg.contains("1234"), "{msg}");
    }

    #[test]
    fn launching_something_that_is_not_there_fails_before_any_ffi() {
        let err = launch_for_pid(
            Path::new("Z:\\definitely\\not\\here\\MapleStory.exe"),
            &[],
            Path::new("Z:\\definitely\\not\\here"),
        )
        .unwrap_err();
        assert!(err.contains("no client executable"), "{err}");
    }

    /// **The positive control for the `SHELLEXECUTEINFOW` layout.**
    ///
    /// `CLAUDE.md`: *"A constant that came from reading a header is a claim, not a fact"*, and
    /// every offset in that struct came from a header. A wrong `cbSize` or a misplaced field
    /// does not raise - `ShellExecuteExW` either fails in a way that looks like a cancelled
    /// prompt, or succeeds and writes `hProcess` somewhere else, and the pid comes back as
    /// garbage or zero. So this starts a real, harmless process and checks the number.
    ///
    /// **What it does NOT cover, stated rather than implied:** the client carries an elevation
    /// manifest and goes through the consent UI, and `cmd.exe` does not. Whether `hProcess`
    /// comes back for an elevated launch is unmeasured here - measuring it costs one of the owner's
    /// manual client launches. `launch_for_pid` treats a null handle as ordinary and
    /// `Launched::describe` says what that costs, which is the honest shape for an untested
    /// branch.
    #[test]
    fn shell_execute_ex_reports_the_pid_of_a_process_it_started() {
        let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".into());
        let cmd = PathBuf::from(&system_root).join("System32").join("cmd.exe");
        if !cmd.is_file() {
            // Not a silent skip: if this ever fires the control is not running and the layout
            // is unverified, which is the state this test exists to prevent.
            panic!("no {} to launch - the layout check cannot run", cmd.display());
        }
        let launched = launch_for_pid(
            &cmd,
            &["/c".to_string(), "exit".into()],
            Path::new(&system_root),
        )
        .expect("cmd.exe /c exit must start");
        let pid = launched.pid.expect("hProcess must come back with SEE_MASK_NOCLOSEPROCESS");
        assert_ne!(pid, 0, "0 is the System Idle Process, never a started one");
        assert_ne!(pid, std::process::id(), "that is this process, not the one just started");
    }

    /// The no-pid case has to say what it costs. It is silent on screen otherwise, and only
    /// bites when a second person is signed in on the same machine.
    #[test]
    fn a_launch_with_no_pid_says_what_that_costs() {
        let msg = Launched { pid: None }.describe();
        assert!(msg.contains("fallback"), "{msg}");
        assert!(msg.to_lowercase().contains("cannot be registered"), "{msg}");
        assert!(Launched { pid: Some(42) }.describe().contains("42"));
    }
}

