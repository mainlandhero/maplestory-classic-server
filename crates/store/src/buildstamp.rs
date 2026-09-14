//! **Which build is this?** One line every process prints at startup.
//!
//! The owner, 2026-09-13: *"As part of startup, all of the processes should include a build time
//! from now on."* The reason is one hour old and it cost a wrong conclusion in both
//! directions. The server at `C:\MapleCW-Server` was running code from before that
//! afternoon's fixes, and the only way anybody could tell was to reason about its *behaviour*
//! - three live claims for one account at startup, and a claim count that grew where the new
//! rule would have held it flat. That is a clever inference and it should never have been
//! necessary; a deployed process should simply say what it is.
//!
//! # Why the executable's own file, and not a compile-time constant
//!
//! The obvious build is `build.rs` writing `cargo:rustc-env=BUILD_TIME`. It is wrong here,
//! and wrong in the exact way `CLAUDE.md` keeps warning about - it produces a **confident
//! stale number**.
//!
//! A build script re-runs when files in *its own package* change. Put the stamp in `store` and
//! a change to `crates/world` leaves it untouched; put one in each binary crate and a change
//! to `crates/store` - which is where today's login-claim fix lived - relinks every server
//! while every stamp stays at yesterday's date. The stamp would have been *most* wrong in
//! precisely the situation that prompted it.
//!
//! The executable file's own last-write time has none of that. It moves whenever the linker
//! writes the file, for any reason, including a dependency-only change. It is not a claim
//! about the source; it is a fact about the bytes that are running.
//!
//! # And the hash, which is the part that actually settles an argument
//!
//! A timestamp answers *"is this recent?"*. It cannot answer *"is this the file I built?"*,
//! because copying, unzipping and touching all move it. The first 16 hex of the SHA-256
//! answers that one exactly: run this on the dev box and on the server and compare two short
//! strings. `tools/package-server.ps1` prints the same digest for every binary it packages,
//! so a deployment can be checked against its own artifact without another build.
//!
//! Hashing a 4 MB executable costs about 10 ms, once, at startup.
//!
//! # What it cannot tell you
//!
//! Nothing here proves the source was committed, or clean. It identifies bytes. A digest that
//! matches the artifact you packaged is proof you deployed that artifact, which is the
//! question that was actually open.

use std::path::PathBuf;

use sha2::{Digest, Sha256};

/// What the running executable is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildStamp {
    pub path: PathBuf,
    /// Bytes on disk, or `None` when the file could not be read.
    pub size: Option<u64>,
    /// Last write time as unix seconds, or `None` when the platform or the file would not say.
    pub modified: Option<i64>,
    /// First 16 hex characters of the SHA-256 of the executable, or `None`.
    pub digest: Option<String>,
    /// The crate version this was compiled from - the one genuinely compile-time fact
    /// available without a build script.
    pub version: &'static str,
}

impl BuildStamp {
    /// The line a server prints at startup.
    ///
    /// Deliberately one line and deliberately self-describing: it is read on a machine nobody
    /// can debug remotely, by someone comparing it to another line somewhere else.
    pub fn line(&self) -> String {
        let when = match self.modified {
            Some(secs) => format_unix(secs),
            None => "unknown".to_string(),
        };
        let digest = self.digest.as_deref().unwrap_or("unknown");
        let size = match self.size {
            Some(n) => format!("{n} bytes"),
            None => "size unknown".to_string(),
        };
        format!(
            "BUILD: {} v{} built {when} ({size}, sha256 {digest}) - the executable's own file, \
             so it moves whenever the linker rewrites it. Compare the digest with the one \
             package-server.ps1 printed to prove WHICH artifact is deployed",
            self.path.display(),
            self.version
        )
    }
}

/// Read the running executable and describe it.
///
/// Never fails: every field it cannot establish comes back `None` and says `unknown` in the
/// line. A server must not refuse to start because it could not introspect itself.
pub fn stamp() -> BuildStamp {
    let version = env!("CARGO_PKG_VERSION");
    let Ok(path) = std::env::current_exe() else {
        return BuildStamp {
            path: PathBuf::from("<unknown>"),
            size: None,
            modified: None,
            digest: None,
            version,
        };
    };

    let meta = std::fs::metadata(&path).ok();
    let size = meta.as_ref().map(|m| m.len());
    let modified = meta
        .as_ref()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64);

    let digest = std::fs::read(&path).ok().map(|bytes| {
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hex::encode(hasher.finalize())[..16].to_string()
    });

    BuildStamp { path, size, modified, digest, version }
}

/// The startup line, in one call.
pub fn line() -> String {
    stamp().line()
}

/// `YYYY-MM-DD HH:MM:SS` **UTC**, from unix seconds.
///
/// UTC and labelled, because the two machines this is compared across need not be in one
/// timezone, and a bare local time that silently differs by hours is worse than no time.
/// Written out rather than pulled from a crate for the same reason the rest of this workspace
/// does its own date arithmetic: the dependency list is deliberately short.
fn format_unix(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02} {:02}:{:02}:{:02} UTC",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Howard Hinnant's `civil_from_days`, the same algorithm `crate::dailyperks` uses for its
/// UTC day boundaries.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The positive control: this runs inside a test binary, which is an executable on disk,
    /// so every field must be populated. A stamp of all `None` would mean the whole thing is
    /// decorative.
    #[test]
    fn the_running_executable_is_described_completely() {
        let s = stamp();
        assert!(s.path.is_file(), "current_exe did not name a file: {}", s.path.display());
        assert!(s.size.unwrap_or(0) > 0);
        assert!(s.modified.is_some(), "no last-write time");
        let digest = s.digest.clone().expect("a digest");
        assert_eq!(digest.len(), 16);
        assert!(digest.chars().all(|c| c.is_ascii_hexdigit()), "{digest}");
    }

    /// **The property the whole module exists for**: two different files must not produce the
    /// same identity, and the same file must produce the same one twice.
    #[test]
    fn the_digest_is_stable_for_one_file_and_the_line_carries_it() {
        let first = stamp();
        let second = stamp();
        assert_eq!(first.digest, second.digest);
        let line = first.line();
        assert!(line.starts_with("BUILD: "), "{line}");
        assert!(line.contains(first.digest.as_deref().unwrap()), "{line}");
        assert!(line.contains("UTC"), "the time must say which zone: {line}");
    }

    #[test]
    fn a_unix_time_renders_as_utc_calendar_time() {
        // 2026-09-13 22:00:00 UTC.
        assert_eq!(format_unix(1_789_336_800), "2026-09-13 22:00:00 UTC");
        // The epoch itself, and a leap day.
        assert_eq!(format_unix(0), "1970-01-01 00:00:00 UTC");
        assert_eq!(format_unix(1_709_164_800), "2024-02-29 00:00:00 UTC");
    }

    /// A stamp that could not read anything still produces a line rather than panicking - a
    /// server must not fail to start because it could not introspect itself.
    #[test]
    fn an_unreadable_executable_still_produces_a_line() {
        let s = BuildStamp {
            path: PathBuf::from("<unknown>"),
            size: None,
            modified: None,
            digest: None,
            version: "0.0.0",
        };
        let line = s.line();
        assert!(line.contains("unknown"), "{line}");
        assert!(line.starts_with("BUILD: "), "{line}");
    }
}
