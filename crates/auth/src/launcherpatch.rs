//! **Publish the launcher itself**, so a launcher can replace its own executable.
//!
//! The owner, 2026-09-16: *"The launcher that we have should have the ability to patch itself
//! should we need to. Currently it doesn't seem able to do that."* It could not: the server
//! published the client folder (`crate::clientpatch`) and nothing else, and the launcher sits
//! beside that folder, outside it.
//!
//! The same shape as the client, cut down to one file: a one-entry `patchset::Manifest` for
//! `maplecw-launcher.exe`, hashed **once at startup** (a restart is how a new launcher is
//! published, exactly as for the client), and an endpoint that streams the bytes. The launcher
//! compares the entry against its own executable and, when they differ, fetches, verifies and
//! swaps itself - `launcher::selfupdate`.
//!
//! # Not the client manifest, on purpose
//!
//! It would be one line to put the launcher into the client manifest. It would also make the
//! launcher patch a file *beside* its client folder through the code path that writes *into*
//! it - and `patchset::is_safe_relative` exists precisely so that path can never write outside
//! the folder. A second, single-purpose endpoint keeps that guarantee whole.
//!
//! # Nothing here authenticates
//!
//! The endpoints are public, like the client's: the manifest is a hash and a size, and the
//! file is the launcher every player already has. Serving them needs no session.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use patchset::{Entry, Manifest};

/// `GET` here for the one-entry manifest.
pub const MANIFEST_PATH: &str = "/launcher/manifest";

/// `GET` here for the executable's bytes.
pub const FILE_PATH: &str = "/launcher/file";

/// The name the launcher is published under, whatever the file on the server was called.
/// The launcher never uses it as a path: it replaces its own `current_exe()`.
pub const PUBLISHED_NAME: &str = "maplecw-launcher.exe";

/// The launcher this server hands out.
pub struct LauncherSource {
    path: PathBuf,
    manifest: Arc<Manifest>,
    rendered: Arc<String>,
}

impl LauncherSource {
    /// Hash `path` once and publish it. `Err` for a missing or unreadable file - refused at
    /// startup rather than served as "nothing to update", for the same reason `--client-dir`
    /// is: a server that silently publishes nothing tells every launcher it is current.
    pub fn open(path: &Path) -> std::io::Result<LauncherSource> {
        if !path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{} is not a file", path.display()),
            ));
        }
        let (size, sha256) = patchset::hash_file(path)?;
        if size == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{} is empty - not a launcher", path.display()),
            ));
        }
        let manifest = Manifest { entries: vec![Entry { path: PUBLISHED_NAME.to_string(), size, sha256 }] };
        let rendered = manifest.render();
        Ok(LauncherSource { path: path.to_path_buf(), manifest: Arc::new(manifest), rendered: Arc::new(rendered) })
    }

    /// The one entry: size and hash of the published launcher.
    pub fn entry(&self) -> &Entry {
        &self.manifest.entries[0]
    }

    /// The manifest as the text a launcher parses.
    pub fn rendered(&self) -> Arc<String> {
        self.rendered.clone()
    }

    /// The file to stream for [`FILE_PATH`].
    pub fn file(&self) -> PathBuf {
        self.path.clone()
    }

    /// The line the server prints at startup.
    pub fn describe(&self) -> String {
        let e = self.entry();
        format!(
            "LAUNCHER PATCHES: serving {} from {} - {:.1} MB, version {}. A launcher whose own \
             executable hashes differently replaces itself from here at Start Game",
            MANIFEST_PATH,
            self.path.display(),
            e.size as f64 / (1024.0 * 1024.0),
            &e.sha256[..16]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(tag: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("maplecw-launcherpatch-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn the_manifest_is_one_entry_named_for_the_launcher_and_the_file_is_the_one_given() {
        let dir = temp("one");
        let exe = dir.join("some-build-of-the-launcher.exe");
        std::fs::write(&exe, b"MZ launcher bytes").unwrap();
        let src = LauncherSource::open(&exe).unwrap();
        let m = Manifest::parse(&src.rendered()).expect("the launcher parses it");
        assert_eq!(m.entries.len(), 1);
        assert_eq!(m.entries[0].path, PUBLISHED_NAME, "published under the canonical name, not the file's");
        assert_eq!(m.entries[0].size, 17);
        assert_eq!(m.entries[0].sha256, patchset::hash_bytes(b"MZ launcher bytes"));
        assert_eq!(src.file(), exe);
        assert!(src.describe().contains(&m.entries[0].sha256[..16]));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_missing_or_empty_launcher_is_refused_rather_than_published_as_nothing() {
        let dir = temp("refused");
        assert!(LauncherSource::open(&dir.join("nope.exe")).is_err());
        let empty = dir.join("empty.exe");
        std::fs::write(&empty, b"").unwrap();
        assert!(LauncherSource::open(&empty).is_err(), "an empty file would tell every launcher to become empty");
        assert!(LauncherSource::open(&dir).is_err(), "a directory is not a launcher");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
