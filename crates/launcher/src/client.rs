//! Preparing the client directory: the GameGuard stub, the hook's marker files, and
//! archiving the previous run's hook log.
//!
//! This is a port of `tools/setup-client.ps1` and the launch block near the end of
//! `tools/test-server.ps1`. Three of the rules below are load-bearing and are each stated
//! where they are enforced rather than in a comment somewhere above the caller:
//!
//! * **The original install is never modified.** `client-patched\` exists for exactly this
//!   reason and it is a standing constraint in `CLAUDE.md`.
//! * **`grap64.dll.orig` is written once, and only if it does not already exist.** Get this
//!   wrong on the second run and the backup becomes a copy of the stub, which means the real
//!   GameGuard DLL is gone for good and no `-Restore` can bring it back.
//! * **The previous hook log is archived, not deleted.** `CLAUDE.md` has a whole section on
//!   this: a client run costs the owner a manual launch, and four conclusions have already died
//!   with a log that got overwritten at the next launch.

use std::path::{Path, PathBuf};

/// The install this launcher must never write to. `tools/setup-client.ps1` refuses the same
/// path, and for the same reason.
pub const ORIGINAL_INSTALL: &str = r"C:\Nexon\Library\maplestorycw\appdata";

pub const GRAP_DLL: &str = "grap64.dll";
pub const GRAP_BACKUP: &str = "grap64.dll.orig";
pub const GRAP_DIR: &str = "grap";
pub const GRAP_DIR_DISABLED: &str = "grap.disabled";

pub const HOOK_ENABLE_MARKER: &str = "maplecw-hook.enable";
pub const HOOK_PROBE_MARKER: &str = "maplecw-hook.probe";
pub const HOOK_SESSION_MARKER: &str = "maplecw-hook.session";
pub const HOOK_DUMPDIR_MARKER: &str = "maplecw-hook.dumpdir";
/// **The one-time credential the client will carry**, in plain text, beside the client.
///
/// Read by `grap_stub::identity`, which writes it into the client's own session object so the
/// client sends it in `0x0073`. Written only when a sign-in produced a fresh token, and
/// **deleted otherwise** - see [`write_identity_marker`].
///
/// Anything running as this user can read this file. That is the same power as being this
/// launch, so on a single-user machine it does not widen anything - but it is a secret on
/// disk and it is named as one here rather than left to be discovered.
pub const HOOK_IDENTITY_MARKER: &str = "maplecw-hook.identity";
pub const HOOK_LOG: &str = "maplecw-hook.log";

/// `tools/test-server.ps1`'s `-Probe` default.
///
/// `1415db360:ret` and `141b2a280:rdx=0` are **not optional**: without the first the client
/// `__fastfail`s about 37 seconds in, because its own reachability check overruns a stack
/// buffer when nothing is reachable; without the second the "trouble logging in" dialog
/// blocks the per-frame tick that enables the Login button.
pub const DEFAULT_PROBE: &str = "watch@1415db360:ret,141b2a280:rdx=0,141b36f60,142ef3e44:hits=8";

/// `tools/test-server.ps1`'s `-Session` default.
///
/// `mode=2` also routes `0x000B` to the classic handler, which is load-bearing; `create=on`
/// sets the protected flag gating "Create a character", re-armed on every login result
/// because the client's handshake zeroes it.
pub const DEFAULT_SESSION: &str = "mode=2,create=on";

/// A stub smaller than this is not a DLL. Checked before anything is displaced, the same way
/// `setup-client.ps1` does it.
pub const MIN_STUB_BYTES: u64 = 1024;

/// One line of progress for the UI's log pane.
pub type Steps = Vec<String>;

// ---------------------------------------------------------------------------------------
// The original-install guard
// ---------------------------------------------------------------------------------------

/// Compare two directory paths without requiring either to exist.
///
/// `canonicalize` is tried first and is authoritative when it works, but it **fails on a path
/// that is not there** - and on a test machine with no Nexon install that is exactly the
/// case, which would quietly turn the guard off. So a normalised textual comparison is the
/// fallback rather than the other way round.
fn normalise(p: &Path) -> String {
    let text = p.to_string_lossy().replace('/', "\\");
    let text = text.strip_prefix(r"\\?\").unwrap_or(&text).to_string();
    let trimmed = text.trim_end_matches('\\');
    // A bare drive letter must keep its separator: `C:` and `C:\` mean different things to
    // Windows, and trimming turns the second into the first.
    if trimmed.len() == 2 && trimmed.ends_with(':') {
        format!("{trimmed}\\").to_ascii_lowercase()
    } else {
        trimmed.to_ascii_lowercase()
    }
}

fn canonical_or_raw(p: &Path) -> String {
    match std::fs::canonicalize(p) {
        Ok(c) => normalise(&c),
        Err(_) => normalise(p),
    }
}

/// Is `child` the same directory as `parent`, or inside it?
pub fn is_within(child: &Path, parent: &Path) -> bool {
    let c = canonical_or_raw(child);
    let p = canonical_or_raw(parent);
    if c == p {
        return true;
    }
    let p_prefix = if p.ends_with('\\') { p } else { format!("{p}\\") };
    c.starts_with(&p_prefix)
}

/// Refuse to touch the original install.
///
/// Wider than `setup-client.ps1`, which compares for equality only: this also refuses a
/// client directory *inside* the original install, and an original install *inside* the
/// client directory. Neither can happen by accident on a correct setup, and both would
/// modify files the project has promised never to modify.
pub fn refuse_original_install(client_dir: &Path, original: &Path) -> Result<(), String> {
    if is_within(client_dir, original) || is_within(original, client_dir) {
        return Err(format!(
            "refusing to touch the original install. {} overlaps {} - point the launcher at \
             the copy (client-patched\\ in a checkout, client\\ beside an installed launcher)",
            client_dir.display(),
            original.display()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// The GameGuard stub
// ---------------------------------------------------------------------------------------

/// Install the no-op `grap64.dll`, idempotently.
///
/// Running this twice must leave `grap64.dll.orig` holding the **real** DLL, not the stub.
/// That is the one thing in this file that cannot be undone if it goes wrong.
pub fn stub_gameguard(client_dir: &Path, stub: &Path) -> Result<Steps, String> {
    let mut steps = Steps::new();

    if !client_dir.is_dir() {
        return Err(format!("client directory not found: {}", client_dir.display()));
    }
    let stub_len = match std::fs::metadata(stub) {
        Ok(m) => m.len(),
        Err(_) => {
            return Err(format!(
                "GameGuard stub not found at {}. Build it first: cargo build --release -p grap-stub",
                stub.display()
            ))
        }
    };
    if stub_len < MIN_STUB_BYTES {
        return Err(format!(
            "the stub at {} looks empty ({stub_len} bytes) - refusing to displace {GRAP_DLL} with it",
            stub.display()
        ));
    }

    let dll = client_dir.join(GRAP_DLL);
    let backup = client_dir.join(GRAP_BACKUP);

    // THE ORDER HERE IS THE POINT. Back up only when there is no backup; otherwise the
    // second run copies the stub over the real DLL's only copy.
    if backup.exists() {
        steps.push(format!("{GRAP_BACKUP} already exists - left alone (this is the real DLL)"));
    } else {
        if !dll.is_file() {
            return Err(format!(
                "no {GRAP_DLL} in {} - is that a client directory?",
                client_dir.display()
            ));
        }
        std::fs::copy(&dll, &backup)
            .map_err(|e| format!("could not back up {GRAP_DLL} to {GRAP_BACKUP}: {e}"))?;
        steps.push(format!("backed up the real {GRAP_DLL} -> {GRAP_BACKUP}"));
    }

    // **Skip the copy when the stub is already installed**, and that is what makes a SECOND
    // client possible on one machine.
    //
    // The owner, 2026-09-01: *"when I tried to start a second client, the grap.dll stub failed
    // because it was being used by another process"*. A running client has this DLL mapped,
    // and Windows refuses to write a mapped image - so the copy raised a sharing violation,
    // `prepare` returned `Err`, and the second launcher gave up **before it ever started a
    // client**. On screen that is indistinguishable from "the client refuses to run twice",
    // which is the conclusion it nearly bought.
    //
    // The backup step above has been idempotent since it was written, for the same class of
    // reason - "back up only when there is no backup". This is that rule applied to the
    // install: if the bytes on disk are already the bytes we would write, there is nothing to
    // do and a locked file is not an error. If they DIFFER we still try, and still fail
    // loudly, because then the client genuinely is running the wrong DLL.
    //
    // Compared by content rather than by length: a stale stub from an older build is exactly
    // the case that must NOT be skipped, and it is the case a length check would miss.
    let already_installed = match (std::fs::read(&dll), std::fs::read(stub)) {
        (Ok(on_disk), Ok(want)) => on_disk == want,
        _ => false,
    };
    if already_installed {
        steps.push(format!(
            "{GRAP_DLL} is already this exact stub ({stub_len} bytes) - left alone, so a \
             client already running does not block a second launch"
        ));
    } else {
        std::fs::copy(stub, &dll)
            .map_err(|e| format!("could not install the stub over {GRAP_DLL}: {e}"))?;
        steps.push(format!("installed the stub as {GRAP_DLL} ({stub_len} bytes)"));
    }

    let grap = client_dir.join(GRAP_DIR);
    let disabled = client_dir.join(GRAP_DIR_DISABLED);
    if grap.is_dir() && !disabled.exists() {
        std::fs::rename(&grap, &disabled)
            .map_err(|e| format!("could not rename {GRAP_DIR}\\ to {GRAP_DIR_DISABLED}: {e}"))?;
        steps.push(format!(
            "renamed {GRAP_DIR}\\ -> {GRAP_DIR_DISABLED} (NGService.exe and BlackCat64.sys cannot run)"
        ));
    } else if disabled.exists() {
        steps.push(format!("{GRAP_DIR_DISABLED} already in place"));
    } else {
        steps.push(format!("no {GRAP_DIR}\\ folder in this client - nothing to disable"));
    }

    steps.push("GameGuard neutralised: no service created, no driver loaded".into());
    Ok(steps)
}

// ---------------------------------------------------------------------------------------
// The hook's marker files
// ---------------------------------------------------------------------------------------

/// Write the four files that switch the hook on.
///
/// **Files, not environment variables**, and that is not a style choice: the client is
/// started with `ShellExecuteW` because it carries an elevation manifest, and ShellExecute
/// does not carry the environment into the child. `crates/grap-stub` reads all four relative
/// to its working directory, which is why the client is launched with the client directory as
/// its working directory.
///
/// Contents are written as UTF-8 with no BOM and no trailing newline. The hook trims what it
/// reads, so a newline would be harmless, but a BOM would not - `read_to_string` keeps it and
/// `trim` does not remove it, so `"\u{feff}watch@..."` would fail every prefix test in
/// `probe.rs` silently.
pub fn write_markers(
    client_dir: &Path,
    probe: &str,
    session: &str,
    dump_dir: &Path,
) -> Result<Steps, String> {
    let mut steps = Steps::new();

    let write = |name: &str, body: &str| -> Result<(), String> {
        let path = client_dir.join(name);
        std::fs::write(&path, body.as_bytes())
            .map_err(|e| format!("could not write {}: {e}", path.display()))
    };

    write(HOOK_ENABLE_MARKER, "")?;
    write(HOOK_PROBE_MARKER, probe)?;
    write(HOOK_SESSION_MARKER, session)?;

    let dump_text = dump_dir.to_string_lossy().to_string();
    write(HOOK_DUMPDIR_MARKER, &dump_text)?;
    // **The sentry marker is deliberately NOT touched here.** An earlier version of this
    // removed it, on the reasoning that the launcher is the shipping path. That was wrong
    // and it broke the ordinary diagnostic combination: `test-server.ps1 -ServersOnly` arms
    // the sentry and the LAUNCHER starts the client, so removing it here made the two
    // impossible to use together. The hook deletes the marker itself once it has read it,
    // which closes the stale-marker hole without closing that path.
    if !dump_text.is_ascii() {
        steps.push(format!(
            "WARNING: the dump directory is not ASCII ({dump_text}) - the hook reads it as UTF-8, \
             which is fine, but nothing else in this project has been tested with one"
        ));
    }

    steps.push(format!("hook enabled ({HOOK_ENABLE_MARKER})"));
    steps.push(format!("client patches: {probe}"));
    steps.push(format!("session patches: {session}"));
    steps.push(format!("crash dumps -> {dump_text}"));
    Ok(steps)
}

/// Put this launch's client token where the hook can find it - or **remove the last one**.
///
/// # A stale token is worse than no token, and that is the whole reason this is a function
///
/// The login server's anti-downgrade rule gives a connection that presents a credential *that
/// claim or none*: it deliberately does **not** fall through to the weaker rules, because a
/// guard that can be skipped by presenting junk is not a guard. So a marker left over from a
/// previous launch does not merely fail to help - it takes this launch from "resolved by the
/// process that owns the socket" down to "served as the server's fallback account".
///
/// Which means the `None` arm is not an omission to tidy up later. It is the arm that has to
/// be right. Every path that does not have a fresh token deletes the file:
///
/// * signing in against a server that predates the client token;
/// * a sign-in that could not stake a claim;
/// * `Start Game` reached without one, which the caller also warns about.
///
/// Returns the lines for the log pane. The token itself is never one of them.
pub fn write_identity_marker(client_dir: &Path, token: Option<&str>) -> Result<Steps, String> {
    let path = client_dir.join(HOOK_IDENTITY_MARKER);
    let mut steps = Steps::new();
    match token {
        Some(token) if !token.trim().is_empty() => {
            // No BOM, no trailing newline, for the same reason the other markers have neither:
            // `read_to_string` keeps a BOM and `trim` does not remove it. The hook trims ASCII
            // whitespace, so a stray newline is harmless and a BOM would not be.
            std::fs::write(&path, token.trim().as_bytes())
                .map_err(|e| format!("could not write {}: {e}", path.display()))?;
            steps.push(format!(
                "client credential written ({HOOK_IDENTITY_MARKER}, {} characters). The client \
                 will carry it in 0x0073 and the server will spend it once. IT IS PLAIN TEXT \
                 ON DISK: anything running as you can read it, which is the same power as \
                 being this launch",
                token.trim().chars().count()
            ));
        }
        _ => {
            // `remove_file` on a path that is not there is `NotFound`, which is success here.
            match std::fs::remove_file(&path) {
                Ok(()) => steps.push(format!(
                    "removed a previous {HOOK_IDENTITY_MARKER}: this launch has no client \
                     token, and a token from an earlier launch would be REFUSED rather than \
                     ignored - the connection would be served as the server's fallback account \
                     instead of being attributed by its owning process"
                )),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    return Err(format!(
                        "could not remove the stale {}: {e}. Refusing to launch with it in \
                         place - the client would present a dead credential and be served as \
                         the fallback account",
                        path.display()
                    ))
                }
            }
        }
    }
    Ok(steps)
}

// ---------------------------------------------------------------------------------------
// Archiving the previous run
// ---------------------------------------------------------------------------------------

/// Move the previous `maplecw-hook.log` into `<data_root>\previous-runs\`, timestamped.
///
/// **Never deletes.** If the timestamped name is already taken, a counter is appended rather
/// than overwriting - `tools/test-server.ps1` uses `Move-Item -Force` here, and this is the
/// one place this port deliberately differs, because a `-Force` that lands on an existing
/// archive destroys the older run.
///
/// Returns the archive path, or `None` if there was no previous log.
pub fn archive_previous_log(log_path: &Path, into: &Path) -> Result<Option<PathBuf>, String> {
    if !log_path.is_file() {
        return Ok(None);
    }
    let dir = into.join("previous-runs");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("could not create {}: {e}", dir.display()))?;

    let stamp = last_write_stamp(log_path);
    let stem = log_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "log".into());
    let ext = log_path
        .extension()
        .map(|s| format!(".{}", s.to_string_lossy()))
        .unwrap_or_default();

    let mut target = dir.join(format!("{stem}-{stamp}{ext}"));
    let mut n = 2;
    while target.exists() {
        target = dir.join(format!("{stem}-{stamp}-{n}{ext}"));
        n += 1;
        if n > 100 {
            return Err(format!(
                "could not find a free name for the previous {} in {}",
                log_path.display(),
                dir.display()
            ));
        }
    }

    // `rename` fails across volumes; copy-then-remove is the fallback, and it is still never
    // a delete-without-a-copy.
    if std::fs::rename(log_path, &target).is_err() {
        std::fs::copy(log_path, &target)
            .map_err(|e| format!("could not archive {}: {e}", log_path.display()))?;
        if std::fs::remove_file(log_path).is_err() {
            // **Another client is still writing this log.** Take the copy back out and leave
            // the original alone.
            //
            // Two reasons, and the second is the one that matters. A second launcher must not
            // fail here - that would block a second client for a reason that has nothing to
            // do with the client. And archiving a log that is still being appended to
            // produces exactly the near-duplicate `CLAUDE.md` describes under "a fixture is
            // copied while the run is still being written": eleven such pairs already exist
            // in this repo, they hash differently from their own run, and file-level
            // deduplication counted them as two observations.
            //
            // So: no half-archive, no error, and the running client keeps its log.
            let _ = std::fs::remove_file(&target);
            return Ok(None);
        }
    }
    Ok(Some(target))
}

/// `yyyyMMdd-HHmmss` in **local** time, matching the names `tools/test-server.ps1` writes, so
/// the archived hook logs sort beside the archived `world.log`s from the same run.
fn last_write_stamp(path: &Path) -> String {
    let modified = std::fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs());
    let Some(unix) = modified else {
        return "unknown".to_string();
    };
    match local_civil_from_unix(unix) {
        Some((y, mo, d, h, mi, s)) => format_stamp(y, mo, d, h, mi, s),
        // No local time available: the raw seconds still sort correctly and still never
        // collide, which is all the name has to do.
        None => format!("unix{unix}"),
    }
}

fn format_stamp(y: u16, mo: u16, d: u16, h: u16, mi: u16, s: u16) -> String {
    format!("{y:04}{mo:02}{d:02}-{h:02}{mi:02}{s:02}")
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct FileTime {
    low: u32,
    high: u32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct Win32SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

extern "system" {
    fn FileTimeToLocalFileTime(ft: *const FileTime, out: *mut FileTime) -> i32;
    fn FileTimeToSystemTime(ft: *const FileTime, out: *mut Win32SystemTime) -> i32;
}

/// Unix seconds -> local `(year, month, day, hour, minute, second)`.
///
/// Done through Win32 rather than by hand because "local" here means whatever the machine's
/// time zone and DST rules say, and reimplementing that is exactly the kind of confident
/// wrong answer this project keeps paying for.
fn local_civil_from_unix(unix: u64) -> Option<(u16, u16, u16, u16, u16, u16)> {
    // FILETIME counts 100 ns ticks from 1601-01-01; 11644473600 seconds separate the epochs.
    let ticks = unix.checked_add(11_644_473_600)?.checked_mul(10_000_000)?;
    let utc = FileTime {
        low: ticks as u32,
        high: (ticks >> 32) as u32,
    };
    let mut local = FileTime::default();
    let mut st = Win32SystemTime::default();
    // SAFETY: both calls take a pointer to one initialised struct and write one struct we own.
    unsafe {
        if FileTimeToLocalFileTime(&utc, &mut local) == 0 {
            return None;
        }
        if FileTimeToSystemTime(&local, &mut st) == 0 {
            return None;
        }
    }
    Some((st.year, st.month, st.day, st.hour, st.minute, st.second))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::TempDir;

    fn fake_client(t: &TempDir) -> PathBuf {
        t.file("client/MapleStory.exe", "client");
        t.file("client/grap64.dll", "THE REAL GAMEGUARD DLL");
        t.dir("client/grap");
        t.path().join("client")
    }

    fn fake_stub(t: &TempDir) -> PathBuf {
        t.file("target/release/grap64.dll", &"S".repeat(4096))
    }

    // -- the backup rule ---------------------------------------------------------------

    #[test]
    fn the_first_run_backs_up_the_real_dll_and_installs_the_stub() {
        let t = TempDir::new("stub1");
        let client = fake_client(&t);
        let stub = fake_stub(&t);

        let steps = stub_gameguard(&client, &stub).expect("stub");
        assert!(steps.iter().any(|s| s.contains("backed up")), "{steps:?}");

        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
        assert_eq!(std::fs::read_to_string(client.join(GRAP_DLL)).unwrap().len(), 4096);
        assert!(client.join(GRAP_DIR_DISABLED).is_dir());
        assert!(!client.join(GRAP_DIR).exists());
    }

    /// **A second launcher must not try to write a DLL the first client has open.**
    ///
    /// The owner, 2026-09-01: *"when I tried to start a second client, the grap.dll stub failed
    /// because it was being used by another process"*. A running client has `grap64.dll`
    /// mapped and Windows refuses to write a mapped image, so the copy raised a sharing
    /// violation, `prepare` returned `Err`, and the second launcher stopped **before it ever
    /// started a client**. That reads on screen as "the client will not run twice", which is
    /// a completely different and much more expensive conclusion.
    ///
    /// The check cannot be "does the file exist" - a stale stub from an older build has to be
    /// replaced. It is a **content** comparison, and this test asserts both directions,
    /// because a skip that always skips is the same bug wearing the opposite sign.
    #[test]
    fn an_identical_stub_is_not_rewritten_so_a_running_client_cannot_block_a_second_launch() {
        let t = TempDir::new("stubidem");
        let client = fake_client(&t);
        let stub = fake_stub(&t);

        let first = stub_gameguard(&client, &stub).expect("first");
        assert!(first.iter().any(|s| s.contains("installed the stub")), "{first:?}");

        // Second launch, nothing changed on disk: the copy must be SKIPPED.
        let second = stub_gameguard(&client, &stub).expect("second");
        assert!(
            second.iter().any(|s| s.contains("left alone")),
            "the second run must not rewrite an identical stub: {second:?}"
        );
        assert!(
            !second.iter().any(|s| s.contains("installed the stub")),
            "and must not claim it installed one: {second:?}"
        );
        assert_eq!(std::fs::read(client.join(GRAP_DLL)).unwrap().len(), 4096);

        // **The other direction.** A DIFFERENT stub - a rebuild - must still be installed,
        // or this "fix" would quietly pin every client to whatever was there first.
        let newer = t.file("target/release/grap64.dll", &"N".repeat(5000));
        let third = stub_gameguard(&client, &newer).expect("third");
        assert!(
            third.iter().any(|s| s.contains("installed the stub")),
            "a rebuilt stub must still replace the old one: {third:?}"
        );
        let on_disk = std::fs::read(client.join(GRAP_DLL)).unwrap();
        assert_eq!(on_disk.len(), 5000);
        assert!(on_disk.iter().all(|&b| b == b'N'), "the new bytes, not the old ones");

        // And through all of that the real DLL is still backed up, untouched.
        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
    }

    #[test]
    fn a_second_run_must_not_overwrite_the_backup_with_the_stub() {
        // This is the one mistake in this file that cannot be undone: get it wrong and the
        // real grap64.dll is gone from the machine.
        let t = TempDir::new("stub2");
        let client = fake_client(&t);
        let stub = fake_stub(&t);

        stub_gameguard(&client, &stub).expect("first");
        let steps = stub_gameguard(&client, &stub).expect("second");

        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL",
            "the second run replaced the backup with the stub"
        );
        assert!(steps.iter().any(|s| s.contains("already exists")), "{steps:?}");
        // ...and it is still idempotent about the folder, too.
        assert!(client.join(GRAP_DIR_DISABLED).is_dir());
    }

    #[test]
    fn a_third_run_is_still_safe() {
        let t = TempDir::new("stub3");
        let client = fake_client(&t);
        let stub = fake_stub(&t);
        for _ in 0..3 {
            stub_gameguard(&client, &stub).expect("run");
        }
        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
    }

    #[test]
    fn a_tiny_stub_is_refused_before_anything_is_displaced() {
        let t = TempDir::new("tinystub");
        let client = fake_client(&t);
        let stub = t.file("target/release/grap64.dll", "nope");

        let err = stub_gameguard(&client, &stub).unwrap_err();
        assert!(err.contains("looks empty"), "{err}");
        // Nothing moved.
        assert!(!client.join(GRAP_BACKUP).exists());
        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_DLL)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
        assert!(client.join(GRAP_DIR).is_dir());
    }

    #[test]
    fn a_missing_stub_is_refused_with_the_build_command() {
        let t = TempDir::new("nostub");
        let client = fake_client(&t);
        let err = stub_gameguard(&client, &t.path().join("nowhere.dll")).unwrap_err();
        assert!(err.contains("cargo build --release -p grap-stub"), "{err}");
    }

    #[test]
    fn a_directory_with_no_grap64_is_refused_rather_than_backed_up_as_nothing() {
        let t = TempDir::new("notclient");
        let client = t.dir("client");
        let stub = fake_stub(&t);
        let err = stub_gameguard(&client, &stub).unwrap_err();
        assert!(err.contains("is that a client directory"), "{err}");
        assert!(!client.join(GRAP_BACKUP).exists());
    }

    // -- the original-install guard ----------------------------------------------------

    #[test]
    fn the_original_install_is_refused_even_though_it_does_not_exist_here() {
        // The guard has to work on a machine with no Nexon install, which is exactly where
        // `canonicalize` cannot help.
        let original = Path::new(ORIGINAL_INSTALL);
        let err = refuse_original_install(original, original).unwrap_err();
        assert!(err.contains("refusing"), "{err}");
    }

    #[test]
    fn case_and_slashes_do_not_get_past_the_guard() {
        let original = Path::new(ORIGINAL_INSTALL);
        for candidate in [
            r"c:\nexon\library\maplestorycw\appdata",
            r"C:/Nexon/Library/maplestorycw/appdata",
            r"C:\Nexon\Library\maplestorycw\appdata\",
        ] {
            assert!(
                refuse_original_install(Path::new(candidate), original).is_err(),
                "{candidate} got through"
            );
        }
    }

    #[test]
    fn a_subdirectory_of_the_original_install_is_refused_too() {
        let original = Path::new(ORIGINAL_INSTALL);
        let inside = Path::new(r"C:\Nexon\Library\maplestorycw\appdata\Data");
        assert!(refuse_original_install(inside, original).is_err());
        // ...and so is a client dir that CONTAINS the original install.
        let outside = Path::new(r"C:\Nexon\Library\maplestorycw");
        assert!(refuse_original_install(outside, original).is_err());
    }

    #[test]
    fn an_ordinary_client_copy_passes_the_guard() {
        let original = Path::new(ORIGINAL_INSTALL);
        for ok in [
            r"C:\MapleCW\client-patched",
            r"D:\MapleCW\client",
            r"C:\Nexon\Library\maplestorycw-copy\appdata",
        ] {
            assert!(
                refuse_original_install(Path::new(ok), original).is_ok(),
                "{ok} was refused"
            );
        }
    }

    #[test]
    fn a_bare_drive_letter_is_not_treated_as_a_prefix_of_everything() {
        // `C:` normalised to `C` would make `is_within` true for every path on the drive.
        assert!(!is_within(Path::new(r"D:\game"), Path::new(r"C:\")));
        assert!(is_within(Path::new(r"C:\game"), Path::new(r"C:\")));
    }

    // -- the marker files ---------------------------------------------------------------

    #[test]
    fn the_four_markers_are_written_with_no_bom_and_no_newline() {
        let t = TempDir::new("markers");
        let client = t.dir("client");
        let dumps = t.dir("dumps");

        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps).expect("markers");
        assert!(steps.iter().any(|s| s.contains(DEFAULT_PROBE)), "{steps:?}");

        let enable = std::fs::read(client.join(HOOK_ENABLE_MARKER)).unwrap();
        assert!(enable.is_empty(), "the enable marker should be empty, got {enable:?}");

        for (name, expected) in [
            (HOOK_PROBE_MARKER, DEFAULT_PROBE.to_string()),
            (HOOK_SESSION_MARKER, DEFAULT_SESSION.to_string()),
            (HOOK_DUMPDIR_MARKER, dumps.to_string_lossy().to_string()),
        ] {
            let raw = std::fs::read(client.join(name)).unwrap();
            assert_ne!(&raw[..raw.len().min(3)], b"\xef\xbb\xbf", "{name} has a BOM");
            assert_eq!(String::from_utf8(raw).unwrap(), expected, "{name}");
        }
    }

    /// The credential is written verbatim: no BOM, no newline, no quoting. `grap_stub::identity`
    /// trims ASCII whitespace, so a newline would survive - but a BOM would not be trimmed and
    /// would be sent as three bytes of the token.
    #[test]
    fn the_credential_marker_is_written_verbatim() {
        let t = TempDir::new("identity");
        let client = t.dir("client");
        let token = "MFRGGZDFMZTWQ2LKNNWG23TP2A";

        let steps = write_identity_marker(&client, Some(token)).expect("write");
        let raw = std::fs::read(client.join(HOOK_IDENTITY_MARKER)).unwrap();
        assert_ne!(&raw[..raw.len().min(3)], b"\xef\xbb\xbf", "a BOM would be sent as data");
        assert_eq!(raw, token.as_bytes());
        // The count reaches the log pane so it can be compared against the hook log and
        // login.log; the token itself must not.
        assert!(steps.iter().any(|s| s.contains("26 characters")), "{steps:?}");
        assert!(!steps.iter().any(|s| s.contains(token)), "the token reached the log: {steps:?}");
        assert!(steps.iter().any(|s| s.contains("PLAIN TEXT")), "{steps:?}");
    }

    /// **The arm that has to be right.** A stale credential is refused by the login server,
    /// not ignored, so leaving one behind costs the next launch its account.
    #[test]
    fn no_credential_removes_the_previous_one_and_says_so() {
        let t = TempDir::new("identitynone");
        let client = t.dir("client");
        let path = client.join(HOOK_IDENTITY_MARKER);

        write_identity_marker(&client, Some("OLDTOKEN")).expect("write");
        assert!(path.is_file());

        let steps = write_identity_marker(&client, None).expect("remove");
        assert!(!path.exists(), "the stale credential survived");
        assert!(steps.iter().any(|s| s.contains("REFUSED")), "{steps:?}");

        // Twice is fine: nothing to remove is success, not NotFound.
        let steps = write_identity_marker(&client, None).expect("remove again");
        assert!(steps.is_empty(), "a no-op should say nothing: {steps:?}");
    }

    /// An empty or whitespace-only token is not a credential and must take the delete arm -
    /// otherwise a server answering `"client_token":""` leaves a file that looks like one.
    #[test]
    fn a_blank_credential_is_treated_as_none() {
        let t = TempDir::new("identityblank");
        let client = t.dir("client");
        write_identity_marker(&client, Some("STALE")).expect("write");
        write_identity_marker(&client, Some("  \r\n")).expect("blank");
        assert!(!client.join(HOOK_IDENTITY_MARKER).exists());
    }

    #[test]
    fn the_default_probe_keeps_the_two_patches_that_keep_the_client_alive() {
        // Not decoration. Without 1415db360:ret the client __fastfails at ~37s; without
        // 141b2a280:rdx=0 the Login button never becomes clickable.
        assert!(DEFAULT_PROBE.contains("1415db360:ret"));
        assert!(DEFAULT_PROBE.contains("141b2a280:rdx=0"));
        assert!(DEFAULT_PROBE.starts_with("watch@"));
        assert!(DEFAULT_SESSION.contains("create=on"));
    }

    // -- archiving ----------------------------------------------------------------------

    #[test]
    fn a_previous_hook_log_is_moved_into_previous_runs_and_not_deleted() {
        let t = TempDir::new("archive");
        let root = t.dir("root");
        let log = t.file("client/maplecw-hook.log", "WATCH the previous run");

        let archived = archive_previous_log(&log, &root).expect("archive").expect("some");
        assert!(!log.exists(), "the live log should have moved");
        assert_eq!(std::fs::read_to_string(&archived).unwrap(), "WATCH the previous run");
        assert!(archived.starts_with(root.join("previous-runs")), "{}", archived.display());
        let name = archived.file_name().unwrap().to_string_lossy().to_string();
        assert!(name.starts_with("maplecw-hook-"), "{name}");
        assert!(name.ends_with(".log"), "{name}");
    }

    #[test]
    fn no_previous_log_is_not_an_error() {
        let t = TempDir::new("noarchive");
        let root = t.dir("root");
        let missing = t.path().join("client").join("maplecw-hook.log");
        assert_eq!(archive_previous_log(&missing, &root).unwrap(), None);
    }

    #[test]
    fn a_name_collision_adds_a_counter_rather_than_overwriting_the_older_run() {
        // `tools/test-server.ps1` uses Move-Item -Force here. A run's output is the most
        // expensive data this project produces; two runs in the same second must not cost
        // one of them.
        let t = TempDir::new("collide");
        let root = t.dir("root");
        let first = t.file("client/maplecw-hook.log", "run one");
        let a = archive_previous_log(&first, &root).unwrap().unwrap();

        let second = t.file("client/maplecw-hook.log", "run two");
        // Force the same stamp by copying the first archive's timestamp onto the new file is
        // not portable, so instead assert the general property: whatever names come out, the
        // first archive still holds run one.
        let b = archive_previous_log(&second, &root).unwrap().unwrap();
        assert_eq!(std::fs::read_to_string(&a).unwrap(), "run one");
        assert_eq!(std::fs::read_to_string(&b).unwrap(), "run two");
        assert_ne!(a, b);
    }

    #[test]
    fn the_stamp_is_zero_padded_and_sorts() {
        assert_eq!(format_stamp(2026, 8, 9, 7, 5, 3), "20260809-070503");
        assert_eq!(format_stamp(2026, 12, 31, 23, 59, 59), "20261231-235959");
        assert!(format_stamp(2026, 8, 9, 7, 5, 3) < format_stamp(2026, 8, 9, 7, 5, 4));
    }

    #[test]
    fn local_time_conversion_answers_for_a_known_instant() {
        // 2000-01-01T00:00:00Z. The local answer depends on this machine's time zone, so the
        // assertion is on the shape rather than the value - what is being checked is that the
        // Win32 pair returns at all, because the fallback name would be silently different.
        let got = local_civil_from_unix(946_684_800).expect("FileTimeToSystemTime failed");
        assert!(got.0 == 1999 || got.0 == 2000, "year {}", got.0);
        assert!((1..=12).contains(&got.1), "month {}", got.1);
        assert!((1..=31).contains(&got.2), "day {}", got.2);
        assert!(got.3 < 24 && got.4 < 60 && got.5 < 60, "{got:?}");
    }
}
