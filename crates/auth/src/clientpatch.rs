//! **The patch source: what the canonical client is, and which of its files may be sent.**
//!
//! The owner, 2026-09-14: *"we need to ensure that the server delivers those patches to the client
//! so the client don't have to re-download everything from a package every time."*
//!
//! The auth service is where this lives because it is the one server every launcher already
//! reaches: it is TLS with a pinned certificate, its address is already configured on every
//! client, and it is already in `MapleCW-server.zip`. A second listener would be a second
//! port to open, a second pin to distribute and a second thing to be down.
//!
//! # The canonical client is a PREPARED one
//!
//! `--client-dir` should point at a client folder a launcher has already prepared - GameGuard
//! stubbed, the Nexon gate byte patched - exactly like `client-patched\` on the dev box. The
//! launcher checks integrity *after* its own patch steps, so both sides have had the same
//! things done to them and the bytes agree. `patchset`'s module docs spell out why a pristine
//! payload as the canonical copy would make every client re-download `MapleStory.exe` forever.
//!
//! # Nothing outside that folder is reachable
//!
//! A file request names a path from the manifest, and **only a path the manifest already
//! carries is served** - the lookup is against the in-memory manifest, not against the disk.
//! So the answer to `../../maplecw.db` is 404 before any filesystem call, and it would be 404
//! even if `patchset::is_safe_relative` (which also runs, on the parse) had let it through.
//! Two independent gates, because this is the one endpoint that turns a string from the
//! network into a path on the server.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use patchset::Manifest;

/// The canonical client, scanned once at startup.
///
/// Scanned **once**, not per request: hashing the real client folder is 405 files and 773 MB,
/// which measures at about half a second warm. Doing that on every launcher's check would put
/// a multi-second stall in front of every player at once. A server restart is how a new client
/// version is published, which is the same way every other change to this server is published.
pub struct ClientPatchSource {
    root: PathBuf,
    manifest: Arc<Manifest>,
    rendered: Arc<String>,
}

impl ClientPatchSource {
    /// Scan `root` and hold its manifest. Returns the error rather than serving a wrong one.
    pub fn open(root: &Path) -> std::io::Result<ClientPatchSource> {
        let manifest = Manifest::scan(root)?;
        let rendered = manifest.render();
        Ok(ClientPatchSource {
            root: root.to_path_buf(),
            manifest: Arc::new(manifest),
            rendered: Arc::new(rendered),
        })
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// The manifest as the text a launcher parses.
    pub fn rendered(&self) -> Arc<String> {
        self.rendered.clone()
    }

    /// The line the server prints at startup, so a deployment says which client it serves.
    pub fn describe(&self) -> String {
        format!(
            "CLIENT PATCHES: serving {} from {} - {} file(s), {:.1} MB, version {}. A launcher \
             whose own folder hashes to a different version is patched from here before it may \
             start the game",
            crate::clientpatch::MANIFEST_PATH,
            self.root.display(),
            self.manifest.entries.len(),
            self.manifest.total_bytes() as f64 / (1024.0 * 1024.0),
            self.manifest.id()
        )
    }

    /// The absolute path of a file a client may fetch, or `None`.
    ///
    /// **The manifest is the allow-list.** A path that is not an entry is refused before the
    /// disk is touched, so no traversal, no symlink chase and no read of a neighbouring file
    /// is reachable from a request.
    pub fn file(&self, rel: &str) -> Option<PathBuf> {
        if !patchset::is_safe_relative(rel) {
            return None;
        }
        let entry = self.manifest.get(rel)?;
        let mut path = self.root.clone();
        for seg in entry.path.split('/') {
            path.push(seg);
        }
        Some(path)
    }
}

/// `GET` here for the manifest.
pub const MANIFEST_PATH: &str = "/client/manifest";

/// `GET /client/file?path=<relative/path>` for one file.
pub const FILE_PATH: &str = "/client/file";

/// The `path=` value out of a query string, percent-decoded.
///
/// Hand-rolled because it is four lines and this crate does not otherwise need a URL parser.
/// Only `%XX` is decoded; `+` is **not** treated as a space, because a client folder can
/// legitimately contain a `+` in a filename and misreading it would 404 a real file.
pub fn path_param(query: &str) -> Option<String> {
    let raw = query.split('&').find_map(|kv| kv.strip_prefix("path="))?;
    let bytes = raw.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Temp(PathBuf);
    impl Temp {
        fn new(tag: &str) -> Temp {
            let p = std::env::temp_dir()
                .join(format!("maplecw-clientpatch-{tag}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&p);
            std::fs::create_dir_all(&p).unwrap();
            Temp(p)
        }
        fn file(&self, rel: &str, body: &[u8]) {
            let p = self.0.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, body).unwrap();
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn source(t: &Temp) -> ClientPatchSource {
        ClientPatchSource::open(&t.0).unwrap()
    }

    #[test]
    fn the_manifest_describes_the_folder_and_a_listed_file_resolves() {
        let t = Temp::new("ok");
        t.file("MapleStory.exe", b"exe");
        t.file("Data/Base/Base.wz", b"base");
        let s = source(&t);
        assert_eq!(s.manifest().entries.len(), 2);
        assert!(s.rendered().starts_with(patchset::HEADER));
        let p = s.file("Data/Base/Base.wz").expect("a listed file resolves");
        assert_eq!(std::fs::read(p).unwrap(), b"base");
        assert!(s.describe().contains(&s.manifest().id()));
    }

    /// **The gate that matters.** This endpoint turns a string from the network into a path on
    /// the server, so it is refused twice: by the path rules, and by not being in the manifest.
    #[test]
    fn nothing_outside_the_client_folder_can_be_asked_for() {
        let t = Temp::new("escape");
        t.file("MapleStory.exe", b"exe");
        // A neighbour of the client folder - the shape of a real secret on a server box.
        std::fs::write(t.0.parent().unwrap().join("maplecw-secret.db"), b"secrets").unwrap();
        let s = source(&t);
        for bad in [
            "../maplecw-secret.db",
            "..%2Fmaplecw-secret.db",
            "/etc/passwd",
            "C:/Windows/System32/config/SAM",
            "Data/../../maplecw-secret.db",
            "MapleStory.exe/../../maplecw-secret.db",
            "notlisted.wz",
        ] {
            assert!(s.file(bad).is_none(), "{bad:?} must not resolve");
        }
        let _ = std::fs::remove_file(t.0.parent().unwrap().join("maplecw-secret.db"));
    }

    /// A file that exists on disk but is volatile is not in the manifest, so it is not served
    /// either - the same rule from both directions.
    #[test]
    fn a_volatile_file_is_neither_listed_nor_servable() {
        let t = Temp::new("volatile");
        t.file("MapleStory.exe", b"exe");
        t.file("maplecw-hook.log", b"log");
        let s = source(&t);
        assert_eq!(s.manifest().entries.len(), 1);
        assert!(s.file("maplecw-hook.log").is_none());
    }

    #[test]
    fn the_path_parameter_is_read_and_percent_decoded() {
        assert_eq!(path_param("path=a.wz").as_deref(), Some("a.wz"));
        assert_eq!(
            path_param("path=Data%2FItem%2FPet%2FPet_000.wz").as_deref(),
            Some("Data/Item/Pet/Pet_000.wz")
        );
        assert_eq!(path_param("path=a%20b.wz").as_deref(), Some("a b.wz"));
        // `+` stays a plus: a real filename may contain one.
        assert_eq!(path_param("path=a+b.wz").as_deref(), Some("a+b.wz"));
        assert_eq!(path_param("other=1"), None);
    }
}
