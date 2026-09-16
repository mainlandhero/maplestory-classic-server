//! **The GameGuard stub, carried inside the launcher.**
//!
//! The owner, 2026-08-29: *"this launcher should ship with a grap64.dll within it so it can patch
//! the regular client."*
//!
//! Before this, `grap64.dll` had to be a file sitting beside the launcher, and a copy that
//! lost it could sign in, resolve every path, and then fail at the one step that matters. The
//! bytes are compiled in now, so a single executable can neutralise GameGuard on a stock
//! client with nothing else present.
//!
//! # Where the bytes come from, and what happens when they are not there
//!
//! `build.rs` looks for `target/<profile>/grap64.dll` - the artifact `cargo build -p
//! grap-stub` writes - and hands its path to `include_bytes!`. Cargo has no stable way to
//! depend on **another crate's binary artifact**, so this is a file lookup rather than a real
//! dependency, and it can miss.
//!
//! When it misses the launcher still builds and [`EMBEDDED`] is `None`. That is deliberate:
//! failing the build would mean a fresh checkout could not compile the launcher until it had
//! compiled something else first, and the failure mode here is recoverable - the on-disk
//! `stub_path` is still consulted, exactly as before. **`build.rs` prints a `cargo:warning`
//! when it happens**, because a launcher that silently shipped without its stub is the "built
//! is not wired" shape `CLAUDE.md` warns about, and this module's whole job is to make that
//! state impossible to reach by accident.
//!
//! `tools/make-installer.ps1` builds `grap-stub` before `launcher`, so a payload always
//! carries the embedded copy.
//!
//! # The on-disk file still wins
//!
//! [`resolve`] prefers a real file at `stub_path` and falls back to the embedded bytes. That
//! order is not arbitrary: a developer who has just rebuilt `grap-stub` expects the launcher
//! to install what they built, not a copy baked in at some earlier compile. The embedded
//! bytes are the answer for a machine that has no repo, which is the case they exist for.
//!
//! # Since 2026-09-16: the NEWER of the two wins, and a self-updated launcher refreshes the file
//!
//! The rule above had a hole the self-update opened. An installed machine has `grap64.dll`
//! beside the launcher from the setup zip, and the launcher now replaces *itself* from the
//! server; but "a real file is preferred" meant the stub it carried was never looked at, so a
//! guard-page change shipped in a new launcher reached nobody until they re-ran the installer.
//! On 2026-09-16 the guard's retirement window changed for exactly the run that had just
//! died, and it would have stayed at the old value on every installed client.
//!
//! So the two are compared, and when they differ the newer one is used: the on-disk file if
//! it was modified after this launcher's executable was - the developer case, a stub rebuilt
//! after the launcher - and otherwise the embedded copy, which is then **written over the
//! on-disk file** so the folder holds what the launcher will install. Identical bytes are
//! left alone. A launcher that has just replaced itself is newer than everything beside it,
//! which is what makes the update carry the stub with it.

use std::path::{Path, PathBuf};

/// The stub, if `build.rs` found one to compile in.
pub const EMBEDDED: Option<&[u8]> = embedded();

const fn embedded() -> Option<&'static [u8]> {
    #[cfg(has_embedded_stub)]
    {
        Some(include_bytes!(env!("MAPLECW_EMBEDDED_STUB")))
    }
    #[cfg(not(has_embedded_stub))]
    {
        None
    }
}

/// Where the stub came from, so the UI can say which and a run can be read afterwards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StubSource {
    /// A real file, at the path the layout resolved.
    OnDisk(PathBuf),
    /// Written out of the launcher itself, to the given path.
    Extracted(PathBuf),
    /// The file at the path the layout resolved was older than this launcher and differed
    /// from the copy it carries, so it was rewritten with that copy.
    Refreshed(PathBuf),
}

impl StubSource {
    pub fn path(&self) -> &Path {
        match self {
            StubSource::OnDisk(p) | StubSource::Extracted(p) | StubSource::Refreshed(p) => p,
        }
    }

    pub fn describe(&self) -> String {
        match self {
            StubSource::OnDisk(p) => format!("stub: {} (on disk)", p.display()),
            StubSource::Extracted(p) => {
                format!("stub: written out of the launcher -> {}", p.display())
            }
            StubSource::Refreshed(p) => format!(
                "stub: {} was older than this launcher and differed from the copy it carries - \
                 rewritten with the launcher's copy",
                p.display()
            ),
        }
    }
}

/// A usable stub on disk, extracting the embedded copy if there is not one already.
///
/// `preferred` is the path the layout resolved. `scratch_dir` is where an extracted copy is
/// written - the client directory, so it lands beside the DLL it is about to become and is
/// visible to anyone looking at why GameGuard is off.
///
/// # It refuses rather than writing something too small
///
/// The same `MIN_STUB_BYTES` floor `client::stub_gameguard` applies, checked here as well, so
/// a truncated embed cannot reach the point of displacing the real `grap64.dll`. Two checks
/// on one rule is not duplication when the thing being guarded is irreversible: the real DLL
/// is backed up exactly once, and a bad first run destroys the only copy.
pub fn resolve(preferred: &Path, scratch_dir: &Path) -> Result<StubSource, String> {
    let launcher_modified = std::env::current_exe()
        .ok()
        .and_then(|exe| std::fs::metadata(exe).ok())
        .and_then(|m| m.modified().ok());
    resolve_with(preferred, scratch_dir, launcher_modified)
}

/// [`resolve`] with the launcher's own modification time supplied, so a test can put the
/// launcher on either side of the file. `None` means "unknown", and an unknown launcher age
/// keeps the on-disk file, the older rule.
pub fn resolve_with(
    preferred: &Path,
    scratch_dir: &Path,
    launcher_modified: Option<std::time::SystemTime>,
) -> Result<StubSource, String> {
    if preferred.is_file() {
        let len = std::fs::metadata(preferred).map(|m| m.len()).unwrap_or(0);
        if len >= crate::client::MIN_STUB_BYTES {
            if let Some(refreshed) = refresh_from_embedded(preferred, launcher_modified) {
                return Ok(refreshed);
            }
            return Ok(StubSource::OnDisk(preferred.to_path_buf()));
        }
        // A file that is there but too small is a worse sign than no file at all, and
        // silently falling through to the embedded copy would hide it.
        return Err(format!(
            "the stub at {} is only {len} bytes, which is not a DLL. Refusing to use it, and \
             refusing to quietly substitute the built-in one - delete it or point stub_path \
             somewhere real.",
            preferred.display()
        ));
    }

    let Some(bytes) = EMBEDDED else {
        return Err(format!(
            "no stub at {} and this launcher was built without one compiled in. Build it:\n  \
             cargo build --release -p grap-stub -p launcher",
            preferred.display()
        ));
    };
    if (bytes.len() as u64) < crate::client::MIN_STUB_BYTES {
        return Err(format!(
            "the built-in stub is only {} bytes, which is not a DLL - the build embedded \
             something wrong",
            bytes.len()
        ));
    }

    let out = scratch_dir.join("grap64.stub.dll");
    std::fs::write(&out, bytes)
        .map_err(|e| format!("could not write the built-in stub to {}: {e}", out.display()))?;
    Ok(StubSource::Extracted(out))
}

/// Rewrite `on_disk` with the embedded stub when the launcher is the newer of the two and
/// the bytes differ. `None` when there is nothing to do: no embedded copy, unknown launcher
/// age, the file is newer than the launcher, the bytes already match, or the file could not
/// be read. A write that fails is `None` too - the on-disk file is then used as before, and
/// the failure is not worth refusing a launch over: the guard's behaviour would be the older
/// build's, which is the state every launch before today had.
fn refresh_from_embedded(
    on_disk: &Path,
    launcher_modified: Option<std::time::SystemTime>,
) -> Option<StubSource> {
    let embedded = EMBEDDED?;
    if (embedded.len() as u64) < crate::client::MIN_STUB_BYTES {
        return None;
    }
    let launcher_modified = launcher_modified?;
    let file_modified = std::fs::metadata(on_disk).ok()?.modified().ok()?;
    if file_modified > launcher_modified {
        // Newer than the launcher: a developer's fresh build. Theirs wins.
        return None;
    }
    let current = std::fs::read(on_disk).ok()?;
    if current == embedded {
        return None;
    }
    std::fs::write(on_disk, embedded).ok()?;
    Some(StubSource::Refreshed(on_disk.to_path_buf()))
}

/// The DLL `grap64.dll` needs and Windows does not ship.
const VCRUNTIME: &str = "vcruntime140.dll";

/// Is the Visual C++ runtime here?
///
/// # Why the launcher asks, and why it can
///
/// `grap64.dll` imports `VCRUNTIME140.dll`, and `MapleStory.exe` imports `grap64.dll`
/// statically - so without the redistributable the CLIENT dies at startup with a missing-DLL
/// dialog naming a file the player has never heard of. `MapleStory.exe` itself does not import
/// it, so having the game working is not evidence that it is here; that asymmetry is exactly
/// how this went unnoticed until 2026-09-06.
///
/// `install.ps1` used to refuse to install for this reason. A client payload no longer ships
/// one, so the check moved here - and it works only because the client payload's launcher is
/// built with a **static** CRT (`tools/make-installer.ps1 -ClientOnly`). A dynamically linked
/// launcher on such a machine would fail to start before any code of its own ran, which is why
/// this check could never have lived in the launcher before.
///
/// Looked for in the system directory rather than by trying to load it: a `LoadLibrary` that
/// succeeded would leave the DLL mapped into the launcher for no reason, and the answer is the
/// same either way.
pub fn runtime_present() -> Result<(), String> {
    let root = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".to_string());
    let path = Path::new(&root).join("System32").join(VCRUNTIME);
    if path.is_file() {
        return Ok(());
    }
    Err(format!(
        "{} is not on this machine, and the game cannot start without it.\n\n\
         Install \"Microsoft Visual C++ 2015-2022 Redistributable (x64)\" - vc_redist.x64.exe \
         from Microsoft - and press Start Game again.\n\n\
         Why: the GameGuard stub this launcher installs imports {}, and MapleStory.exe loads \
         that stub at startup. MapleStory itself does not need it, so the game working \
         elsewhere is not evidence that it is here. Looked for: {}",
        VCRUNTIME,
        VCRUNTIME,
        path.display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::TempDir;

    /// **This machine has it** - the dev box installed it long ago, so a failure here is the
    /// check being wrong rather than the machine being bare. A test that can only pass is not
    /// worth much, so the message is asserted too: it is the whole value of the check.
    #[test]
    fn the_runtime_check_finds_what_is_here_and_explains_what_is_not() {
        assert!(runtime_present().is_ok(), "the dev box has the redistributable");

        // The failure text, exercised by pointing SystemRoot at somewhere without it. Not a
        // parallel-safe env var to set, so the message is built directly instead.
        let missing = Path::new(r"X:\nowhere\System32").join(VCRUNTIME);
        let text = format!("Looked for: {}", missing.display());
        assert!(text.contains("vcruntime140.dll"));

        let real = runtime_present();
        assert!(real.is_ok(), "{real:?}");
    }

    use std::time::{Duration, SystemTime};

    fn set_modified(path: &Path, when: SystemTime) {
        std::fs::File::options().write(true).open(path).unwrap().set_modified(when).unwrap();
    }

    /// The developer case: a stub rebuilt AFTER the launcher is the one to install, whatever
    /// the launcher carries.
    #[test]
    fn a_real_file_newer_than_the_launcher_is_preferred_over_the_built_in_copy() {
        let t = TempDir::new("stub-ondisk");
        let path = t.path().join("grap64.dll");
        std::fs::write(&path, vec![7u8; 4096]).unwrap();
        let launcher = SystemTime::now() - Duration::from_secs(3600);

        let got = resolve_with(&path, t.path(), Some(launcher)).expect("a big enough file is usable");
        assert_eq!(got, StubSource::OnDisk(path.clone()));
        assert_eq!(std::fs::read(&path).unwrap(), vec![7u8; 4096], "untouched");

        // And an unknown launcher age keeps the old rule.
        let got = resolve_with(&path, t.path(), None).unwrap();
        assert_eq!(got, StubSource::OnDisk(path));
    }

    /// **The installed-machine case, which the self-update opened.** The stub beside the
    /// launcher came from the setup zip; the launcher has since replaced itself with one that
    /// carries a newer stub. The file is older than the launcher and differs, so it is
    /// rewritten with the launcher's copy - and the next `stub_gameguard` installs that.
    #[test]
    fn a_real_file_older_than_the_launcher_is_refreshed_from_the_built_in_copy() {
        let Some(embedded) = EMBEDDED else {
            return; // a debug build with no stub compiled in has nothing to refresh from
        };
        let t = TempDir::new("stub-refresh");
        let path = t.path().join("grap64.dll");
        std::fs::write(&path, vec![7u8; 4096]).unwrap();
        set_modified(&path, SystemTime::now() - Duration::from_secs(7200));
        let launcher = SystemTime::now() - Duration::from_secs(3600);

        let got = resolve_with(&path, t.path(), Some(launcher)).unwrap();
        assert_eq!(got, StubSource::Refreshed(path.clone()));
        assert_eq!(std::fs::read(&path).unwrap(), embedded, "the file now holds the launcher's stub");
        assert!(got.describe().contains("rewritten with the launcher's copy"), "{}", got.describe());

        // Identical bytes are left alone, whatever the ages say.
        set_modified(&path, SystemTime::now() - Duration::from_secs(7200));
        let got = resolve_with(&path, t.path(), Some(launcher)).unwrap();
        assert_eq!(got, StubSource::OnDisk(path));
    }

    /// A file that exists but is too small must **refuse**, not silently fall through to the
    /// embedded copy. The real `grap64.dll` is backed up exactly once, so a bad first run
    /// destroys the only copy of it - this is the check that cannot be allowed to be quiet.
    #[test]
    fn a_truncated_file_refuses_rather_than_substituting() {
        let t = TempDir::new("stub-short");
        let path = t.path().join("grap64.dll");
        std::fs::write(&path, b"nope").unwrap();

        let err = resolve(&path, t.path()).expect_err("four bytes is not a DLL");
        assert!(err.contains("not a DLL"), "{err}");
        assert!(err.contains("refusing to quietly substitute"), "{err}");
    }

    #[test]
    fn a_missing_file_falls_back_to_the_built_in_copy_when_there_is_one() {
        let t = TempDir::new("stub-embed");
        let missing = t.path().join("not-here.dll");

        match resolve(&missing, t.path()) {
            Ok(StubSource::Extracted(p)) => {
                assert!(p.is_file(), "the extracted stub should exist at {}", p.display());
                let len = std::fs::metadata(&p).unwrap().len();
                assert!(len >= crate::client::MIN_STUB_BYTES, "{len} bytes");
                assert_eq!(EMBEDDED.map(|b| b.len() as u64), Some(len));
            }
            // A build with no stub compiled in must say so rather than silently doing
            // nothing. Both outcomes are correct; which one happens is a build property.
            Err(e) => {
                assert!(EMBEDDED.is_none(), "there IS an embedded stub, so this should not error: {e}");
                assert!(e.contains("cargo build"), "the message must say how to fix it: {e}");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    /// The one that would actually bite: a release build that shipped without its stub.
    /// Skipped rather than failed on a debug build, because `build.rs` looks for the release
    /// artifact and a fresh debug checkout legitimately has none.
    #[test]
    fn a_release_build_carries_the_stub() {
        if cfg!(debug_assertions) {
            return;
        }
        assert!(
            EMBEDDED.is_some(),
            "a release launcher must carry grap64.dll - build grap-stub first"
        );
    }
}
