//! **The launcher replaces its own executable from the server.**
//!
//! The owner, 2026-09-16: *"The launcher that we have should have the ability to patch itself
//! should we need to. Currently it doesn't seem able to do that."* It could not: the version
//! gate (`crate::clientpatch`) covers the client folder, and the launcher lives beside it.
//!
//! # What happens at Start Game, before anything else
//!
//! 1. `GET /launcher/manifest` - one entry, the size and SHA-256 of the launcher the server
//!    ships (`auth::launcherpatch`). A 503 means the server publishes none; that is logged
//!    and the launch goes on, because an old server must not lock every player out of a
//!    launcher that was working a minute ago. Any other failure to reach the server is an
//!    error, the same as the client gate's: the next request would be to the same host.
//! 2. This executable is hashed. Equal - nothing to do, one line in the log.
//! 3. Different - `GET /launcher/file`, verified against the manifest before a byte is
//!    written, then **swapped in place**: the running executable is renamed aside (Windows
//!    allows renaming a running program; it forbids overwriting or deleting it), the new one
//!    is renamed onto its path, and the new one is started with `--updated-from <old>`. The
//!    window then closes. The new launcher deletes the old file once this process has gone.
//!
//! If the second rename fails the first is undone, so the worst case is the launcher that
//! was already there. Nothing is written directly onto the executable's path.
//!
//! # Why the new launcher starts at the sign-in screen
//!
//! The sign-in and its claim belong to the process that made them, and the password is never
//! on disk. Sign in again, press Start Game again; the server address and the client folder
//! are remembered. Stated rather than hidden.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use patchset::{Entry, Manifest};
use tlspin::Fingerprint;

use crate::prepare::Level;

/// Mirrors `auth::launcherpatch`.
const MANIFEST_PATH: &str = "/launcher/manifest";
const FILE_PATH: &str = "/launcher/file";

/// The argument the freshly installed launcher is started with: the path of the executable
/// it replaced, which it deletes once the old process has exited.
pub const UPDATED_FROM_FLAG: &str = "--updated-from";

/// What the check found.
#[derive(Debug)]
pub enum Outcome {
    /// This executable matches the server's, byte for byte.
    UpToDate,
    /// The server publishes no launcher (started without `--launcher`). Not an error.
    NotPublished,
    /// The new launcher is installed and starting; **this process must exit.**
    Replaced { version: String, bytes: u64 },
}

/// **Check this launcher against the server and replace it if it is behind.**
pub fn check_and_update(
    host: &str,
    port: u16,
    pin: &Fingerprint,
    log: &mut dyn FnMut(Level, String),
) -> Result<Outcome, String> {
    let exe = std::env::current_exe().map_err(|e| format!("could not find this launcher's own path: {e}"))?;
    let want = match fetch_manifest(host, port, pin)? {
        Some(m) => m,
        None => {
            log(
                Level::Warn,
                format!(
                    "{host}:{port} publishes no launcher (maplecw-auth was started without --launcher), so this \
                     launcher cannot check itself. Going on with the one you have"
                ),
            );
            return Ok(Outcome::NotPublished);
        }
    };
    let (size, sha256) = patchset::hash_file(&exe).map_err(|e| format!("could not read this launcher to check it ({}): {e}", exe.display()))?;
    let version = short(&want.sha256);
    if !needs_update(size, &sha256, &want) {
        log(Level::Info, format!("launcher version {version} confirmed with the server"));
        return Ok(Outcome::UpToDate);
    }
    log(
        Level::Info,
        format!(
            "this launcher is version {} and the server publishes {version} - updating it ({:.1} MB)",
            short(&sha256),
            want.size as f64 / (1024.0 * 1024.0)
        ),
    );
    let bytes = fetch_file(host, port, pin)?;
    verify(&bytes, &want)?;
    let old = swap_in(&exe, &bytes)?;
    // Start the new one before this one goes; it inherits nothing but the flag.
    std::process::Command::new(&exe)
        .arg(UPDATED_FROM_FLAG)
        .arg(&old)
        .spawn()
        .map_err(|e| {
            format!(
                "the new launcher is installed at {} but could not be started: {e}. Start it by hand; \
                 the previous one is beside it as {}",
                exe.display(),
                old.display()
            )
        })?;
    log(
        Level::Good,
        format!("launcher updated to version {version}. The new launcher is opening - this window closes now; sign in again there"),
    );
    Ok(Outcome::Replaced { version, bytes: want.size })
}

/// Called by the NEW launcher at startup with the path it was told: delete the executable it
/// replaced. The old process may still be shutting down, so this retries for a few seconds;
/// a file that will not go is reported, not fatal. Returns the line for the log.
pub fn finish_update(old: &Path) -> String {
    for _ in 0..40 {
        match std::fs::remove_file(old) {
            Ok(()) => {
                let note = format!("launcher updated: the previous executable {} was removed", old.display());
                let _ = STARTUP_NOTE.set(note.clone());
                return note;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let note = format!("launcher updated (the previous executable {} was already gone)", old.display());
                let _ = STARTUP_NOTE.set(note.clone());
                return note;
            }
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(100)),
        }
    }
    let note = format!(
        "launcher updated, but the previous executable {} could not be removed (still in use?). It is harmless; delete it by hand",
        old.display()
    );
    let _ = STARTUP_NOTE.set(note.clone());
    note
}

/// What [`finish_update`] found, for the window's first log lines.
pub fn startup_note() -> Option<&'static str> {
    STARTUP_NOTE.get().map(|s| s.as_str())
}

static STARTUP_NOTE: OnceLock<String> = OnceLock::new();

/// The whole decision: size and hash both, because a size match alone is what two builds
/// differing in one byte look like.
pub fn needs_update(own_size: u64, own_sha256: &str, want: &Entry) -> bool {
    own_size != want.size || !own_sha256.eq_ignore_ascii_case(&want.sha256)
}

/// Refuse a download that is not the file the manifest described - before it is written.
pub fn verify(bytes: &[u8], want: &Entry) -> Result<(), String> {
    if bytes.len() as u64 != want.size || patchset::hash_bytes(bytes) != want.sha256.to_ascii_lowercase() {
        return Err(format!(
            "the downloaded launcher did not survive the download: expected {} bytes hashing to {}, got {} bytes hashing to {}. \
             Nothing was installed",
            want.size,
            short(&want.sha256),
            bytes.len(),
            short(&patchset::hash_bytes(bytes))
        ));
    }
    Ok(())
}

/// **Put `bytes` at `exe`'s path without ever overwriting `exe`.** Returns where the previous
/// executable now is (`<exe>.old`).
///
/// Order: write `<exe>.maplecw-part`; move the current executable to `<exe>.old` (a running
/// program can be renamed on Windows); move the part onto the executable's path. If that last
/// move fails the old one is moved back, so the folder never lacks a launcher.
pub fn swap_in(exe: &Path, bytes: &[u8]) -> Result<PathBuf, String> {
    let part = PathBuf::from(format!("{}.maplecw-part", exe.display()));
    let old = PathBuf::from(format!("{}.old", exe.display()));
    std::fs::write(&part, bytes).map_err(|e| format!("could not write {}: {e}", part.display()))?;
    // A leftover from an update that never finished cleaning up.
    let _ = std::fs::remove_file(&old);
    std::fs::rename(exe, &old).map_err(|e| {
        let _ = std::fs::remove_file(&part);
        format!("could not move the current launcher aside ({} -> {}): {e}", exe.display(), old.display())
    })?;
    if let Err(e) = std::fs::rename(&part, exe) {
        let restored = std::fs::rename(&old, exe).is_ok();
        let _ = std::fs::remove_file(&part);
        let where_now = if restored {
            "The previous launcher was put back".to_string()
        } else {
            format!("AND the previous launcher could not be put back - it is at {}", old.display())
        };
        return Err(format!("could not install the new launcher at {}: {e}. {where_now}", exe.display()));
    }
    Ok(old)
}

fn short(sha256: &str) -> String {
    sha256.chars().take(16).collect()
}

/// `None` when the server publishes no launcher (503).
fn fetch_manifest(host: &str, port: u16, pin: &Fingerprint) -> Result<Option<Entry>, String> {
    let (status, body) = crate::http::get_bytes(host, port, pin, MANIFEST_PATH)?;
    if status == 503 {
        return Ok(None);
    }
    if status != 200 {
        return Err(format!(
            "the server at {host}:{port} answered {status} for the launcher manifest, so this launcher \
             could not check itself and the game will not be started."
        ));
    }
    let text = String::from_utf8(body).map_err(|_| "the launcher manifest was not text".to_string())?;
    let m = Manifest::parse(&text).map_err(|e| format!("the server's launcher manifest is unreadable: {e}"))?;
    m.entries.into_iter().next().map(Some).ok_or_else(|| "the server's launcher manifest is empty".to_string())
}

fn fetch_file(host: &str, port: u16, pin: &Fingerprint) -> Result<Vec<u8>, String> {
    let (status, body) = crate::http::get_bytes(host, port, pin, FILE_PATH)?;
    if status != 200 {
        return Err(format!("the server answered {status} for the launcher file; the update was not installed."));
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::TempDir;

    fn entry(bytes: &[u8]) -> Entry {
        Entry { path: "maplecw-launcher.exe".into(), size: bytes.len() as u64, sha256: patchset::hash_bytes(bytes) }
    }

    #[test]
    fn the_decision_is_size_and_hash_and_the_download_is_verified_before_it_is_written() {
        let want = entry(b"new launcher");
        assert!(!needs_update(12, &want.sha256, &want));
        assert!(!needs_update(12, &want.sha256.to_ascii_uppercase(), &want), "case does not matter");
        assert!(needs_update(12, &patchset::hash_bytes(b"old launcher"), &want), "same size, different bytes");
        assert!(needs_update(11, &want.sha256, &want));
        assert!(verify(b"new launcher", &want).is_ok());
        assert!(verify(b"new launche", &want).is_err(), "truncated");
        assert!(verify(b"new launchex", &want).is_err(), "same length, wrong bytes");
    }

    /// **End to end over TLS, against the real service**: the launcher's own fetchers read the
    /// one-entry manifest and the exact bytes back, and a server started without `--launcher`
    /// answers "none" rather than an error. `check_and_update` itself is NOT called here - it
    /// would compare and replace the TEST BINARY - so the swap is covered by the test above.
    #[test]
    fn the_manifest_and_the_file_come_back_over_tls_and_a_bare_server_publishes_none() {
        let dir = std::env::temp_dir().join(format!("maplecw-launcher-selfupdate-e2e-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let identity = auth::tls::ensure_identity(&dir).unwrap();
        let pin = identity.fingerprint;
        let published = dir.join("maplecw-launcher.exe");
        std::fs::write(&published, b"MZ the launcher the server ships").unwrap();

        let store = store::Store::open_in_memory().unwrap();
        let source = auth::launcherpatch::LauncherSource::open(&published).unwrap();
        let service = std::sync::Arc::new(
            auth::AuthService::new(std::sync::Arc::new(store)).with_launcher_patches(std::sync::Arc::new(source)),
        );
        let listener = auth::http::listen("127.0.0.1", 0).unwrap();
        let port = listener.local_addr().unwrap().port();
        let tls = identity.server_config().unwrap();
        std::thread::spawn(move || {
            let _ = auth::http::run(listener, service, tls);
        });

        let want = fetch_manifest("127.0.0.1", port, &pin).unwrap().expect("published");
        assert_eq!(want.path, "maplecw-launcher.exe");
        assert_eq!(want.size, 32);
        let bytes = fetch_file("127.0.0.1", port, &pin).unwrap();
        assert!(verify(&bytes, &want).is_ok(), "what came back is what was published");
        assert_eq!(bytes, b"MZ the launcher the server ships");
        // A launcher that IS that file has nothing to do; any other does.
        assert!(!needs_update(32, &patchset::hash_bytes(&bytes), &want));
        assert!(needs_update(32, &patchset::hash_bytes(b"MZ an older launcher, same size!"), &want));

        // The bare server: 503 -> None, not Err.
        let bare = std::sync::Arc::new(auth::AuthService::new(std::sync::Arc::new(store::Store::open_in_memory().unwrap())));
        let listener = auth::http::listen("127.0.0.1", 0).unwrap();
        let bare_port = listener.local_addr().unwrap().port();
        let tls = identity.server_config().unwrap();
        std::thread::spawn(move || {
            let _ = auth::http::run(listener, bare, tls);
        });
        assert!(fetch_manifest("127.0.0.1", bare_port, &pin).unwrap().is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The file mechanics, on a stand-in for the executable: the old one is beside the new
    /// one under `.old`, nothing was ever written onto the executable's path directly, and the
    /// part file is gone.
    #[test]
    fn swap_in_moves_the_old_launcher_aside_and_puts_the_new_one_on_its_path() {
        let t = TempDir::new("selfupdate");
        let exe = t.path().join("maplecw-launcher.exe");
        std::fs::write(&exe, b"old launcher").unwrap();
        // A stale .old from a previous, unfinished update must not block this one.
        std::fs::write(t.path().join("maplecw-launcher.exe.old"), b"older still").unwrap();

        let old = swap_in(&exe, b"new launcher").unwrap();
        assert_eq!(std::fs::read(&exe).unwrap(), b"new launcher");
        assert_eq!(old, t.path().join("maplecw-launcher.exe.old"));
        assert_eq!(std::fs::read(&old).unwrap(), b"old launcher");
        assert!(!t.path().join("maplecw-launcher.exe.maplecw-part").exists());

        // The new launcher's first act: the old one is removed.
        let note = finish_update(&old);
        assert!(!old.exists(), "{note}");
        assert!(note.contains("removed"));
        assert_eq!(startup_note().map(|s| s.contains("removed")), Some(true));
        // And a second call for a path that is already gone is not an error either.
        assert!(finish_update(&old).contains("already gone"));
    }
}
