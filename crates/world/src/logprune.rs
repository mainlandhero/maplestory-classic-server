//! **`previous-runs\` keeps a week.** The owner, 2026-09-24: *"For the server log rotation, if
//! the logs are older than 7 days in previous-runs, please have the server automatically get
//! rid of them."*
//!
//! Both launchers - `tools/test-server.ps1` and the installed `start-server.ps1` - MOVE the
//! last run's logs into a `previous-runs\` beside them instead of deleting them, and nothing
//! ever emptied it. The world server knows that directory without being told: it is started
//! with `--log-file <dir>\world-chN.log` by both, and `previous-runs\` is `<dir>`'s. So it
//! prunes there when its log opens, and once a day after that for a server left running.
//!
//! # What goes, and what does not
//!
//! * A **file** directly in `previous-runs\` whose **last write** is more than
//!   [`KEEP_DAYS`] old. The last write, not the name: the stamp in an archived name is the
//!   log's own last write too, and the file time is what cannot be misparsed.
//! * **Nothing else**: not a subdirectory, not a file anywhere but that one folder, and
//!   never `research/fixtures/` - which is where `CLAUDE.md` says a run that settles a
//!   question is copied, precisely because `previous-runs\` was always meant to be a
//!   rolling buffer. This makes it one, at a week.
//! * Every file removed is named in the log, with its age, so a deletion is never silent.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// How long an archived run is kept.
pub const KEEP_DAYS: u64 = 7;

/// How often a running server prunes again.
pub const EVERY: Duration = Duration::from_secs(24 * 60 * 60);

/// The archive folder a `--log-file` path implies: `previous-runs` beside it.
pub fn archive_dir_for(log_file: &Path) -> PathBuf {
    let dir = log_file.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    dir.join("previous-runs")
}

/// **Remove every file directly in `dir` last written more than `keep` before `now`.**
/// Returns what was removed with its age; a folder that does not exist is nothing to do.
/// A file that cannot be read or removed is skipped and reported, never fatal.
pub fn prune(dir: &Path, keep: Duration, now: SystemTime) -> (Vec<(PathBuf, Duration)>, Vec<String>) {
    let mut gone = Vec::new();
    let mut problems = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return (gone, problems) };
    for entry in entries.flatten() {
        let path = entry.path();
        // `symlink_metadata`, so a link is judged as itself and never followed out of the folder.
        let Ok(meta) = std::fs::symlink_metadata(&path) else { continue };
        if !meta.is_file() {
            continue;
        }
        let Ok(written) = meta.modified() else {
            problems.push(format!("{}: no modified time", path.display()));
            continue;
        };
        // A file from the future (a clock set back) has age zero and is kept.
        let age = now.duration_since(written).unwrap_or(Duration::ZERO);
        if age <= keep {
            continue;
        }
        match std::fs::remove_file(&path) {
            Ok(()) => gone.push((path, age)),
            Err(e) => problems.push(format!("{}: {e}", path.display())),
        }
    }
    gone.sort();
    (gone, problems)
}

/// [`prune`] with [`KEEP_DAYS`] against the wall clock, and the result as log lines.
pub fn prune_archive(dir: &Path) -> Vec<String> {
    let keep = Duration::from_secs(KEEP_DAYS * 24 * 60 * 60);
    let (gone, problems) = prune(dir, keep, SystemTime::now());
    let mut lines = Vec::new();
    for (path, age) in &gone {
        lines.push(format!(
            "log prune: removed {} ({} days old; previous-runs keeps {KEEP_DAYS})",
            path.display(),
            age.as_secs() / 86_400
        ));
    }
    for p in problems {
        lines.push(format!("log prune: could not remove {p}"));
    }
    if gone.is_empty() && lines.is_empty() {
        lines.push(format!("log prune: nothing in {} is older than {KEEP_DAYS} days", dir.display()));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: Duration = Duration::from_secs(86_400);

    fn scratch(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("maplecw-logprune-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn file_aged(dir: &Path, name: &str, now: SystemTime, age: Duration) -> PathBuf {
        let p = dir.join(name);
        let f = std::fs::File::create(&p).unwrap();
        f.set_modified(now - age).unwrap();
        p
    }

    /// Older than seven days goes; seven days and younger stays, and so does a file from the
    /// future; a subdirectory is never touched, even an old one; and the control - the
    /// same folder with nothing old - removes nothing.
    #[test]
    fn only_files_older_than_a_week_go() {
        let dir = scratch("week");
        let now = SystemTime::now();
        let keep = DAY * KEEP_DAYS as u32;
        let old = file_aged(&dir, "world-ch0-20260901-120000.log", now, DAY * 8);
        let edge = file_aged(&dir, "world-ch0-20260917-120000.log", now, keep - Duration::from_secs(60));
        let young = file_aged(&dir, "login-20260923-120000.log", now, DAY);
        let future = file_aged(&dir, "auth-future.log", now, Duration::ZERO);
        let sub = dir.join("keep-me");
        std::fs::create_dir_all(&sub).unwrap();
        let inner = file_aged(&sub, "inner.log", now, DAY * 30);

        let (gone, problems) = prune(&dir, keep, now);
        assert!(problems.is_empty(), "{problems:?}");
        assert_eq!(gone.iter().map(|(p, _)| p.clone()).collect::<Vec<_>>(), vec![old.clone()]);
        assert!(!old.exists());
        for kept in [&edge, &young, &future, &inner] {
            assert!(kept.exists(), "{} must stay", kept.display());
        }
        // Run again: nothing left to do.
        assert!(prune(&dir, keep, now).0.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A folder that does not exist is nothing to do, and the archive folder is the one
    /// beside the log the server was told to write.
    #[test]
    fn the_archive_is_beside_the_log_and_a_missing_one_is_fine() {
        assert_eq!(
            archive_dir_for(Path::new(r"C:\MapleCW\world-ch0.log")),
            Path::new(r"C:\MapleCW").join("previous-runs")
        );
        assert_eq!(archive_dir_for(Path::new("world-ch1.log")), Path::new(".").join("previous-runs"));
        let (gone, problems) = prune(Path::new("this/does/not/exist"), DAY, SystemTime::now());
        assert!(gone.is_empty() && problems.is_empty());
    }
}
