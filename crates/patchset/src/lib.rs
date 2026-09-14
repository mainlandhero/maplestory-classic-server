//! **Is this client the one the server expects, and if not, which files bring it up to date?**
//!
//! The owner, 2026-09-14: *"we need to implement an integrity check that reaches out to the server
//! to make sure that this is the correct version that the client should be running on. Older
//! clients will be forced to patch to be up to date, and we need to ensure that the server
//! delivers those patches to the client so the client don't have to re-download everything
//! from a package every time."*
//!
//! One manifest describes a client folder: one line per file, with its size and its SHA-256.
//! The launcher builds one for the folder on disk, asks the server for its own, and the
//! difference is the download list. A 562 MB payload becomes the handful of archives that
//! actually moved.
//!
//! # The manifest is a text table, not JSON
//!
//! ```text
//! # maplecw client manifest v1
//! <64 hex sha256>\t<size in bytes>\t<relative/path/with/forward/slashes>
//! ```
//!
//! Sorted by path, LF line endings, no trailing spaces. Same shape as everything in
//! `gm-handbook/`, and for the same reasons: it diffs in git, it is readable in a log when
//! somebody pastes it back from a machine nobody can debug remotely, and parsing it needs no
//! dependency in a crate that is compiled into the binary players download.
//!
//! The manifest's **identity** is the SHA-256 of its own exact bytes ([`Manifest::id`]). Two
//! clients agree when their ids agree; that one short string is what a launcher and a server
//! compare first, and what a log line can carry.
//!
//! # THE TRAP: the launcher modifies the client, so a naive manifest never converges
//!
//! `crates/launcher` writes to the client folder on every Start Game - it installs the
//! GameGuard stub over `grap64.dll`, renames `grap\` to `grap.disabled\`, and flips one byte
//! in `MapleStory.exe` (the Nexon Launcher gate). A manifest taken from a *pristine* payload
//! would therefore disagree with every prepared client about `grap64.dll` and
//! `MapleStory.exe` - so the patcher would re-download 76 MB, the launcher would re-patch it,
//! and the next launch would do it again. Forever.
//!
//! Two rules close that, and both matter:
//!
//! * **The canonical client the server serves is a PREPARED one** - a folder a launcher has
//!   already stubbed and gate-patched, exactly like `client-patched\` on the dev box. The
//!   integrity check then runs *after* the launcher's own patch steps, so both sides have had
//!   the same things done to them and the bytes match.
//! * **[`is_volatile`] excludes what differs per machine no matter what** - logs, hook
//!   markers, crash dumps, screenshots the client drops beside itself, and the
//!   `grap64.dll.orig` / `grap.disabled\` backups the stubbing leaves behind. None of those is
//!   part of "which version is this", and including them would make every client permanently
//!   out of date against every other.
//!
//! # Extra local files are left alone
//!
//! [`Manifest::plan`] only ever says *fetch this*. It never says *delete that*. A client
//! folder legitimately holds files the manifest has never heard of, and a patcher that
//! deleted the unknown would eventually delete somebody's screenshots. Missing and differing
//! files are the whole job.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

/// The header every manifest starts with. A file without it is not one.
pub const HEADER: &str = "# maplecw client manifest v1";

/// One file in a client folder.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    /// Relative to the client folder, forward slashes, never absolute and never containing
    /// `..` - see [`is_safe_relative`], which every parser and every server read goes through.
    pub path: String,
    pub size: u64,
    /// Lowercase hex SHA-256, 64 characters.
    pub sha256: String,
}

/// Every file in one client folder.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Manifest {
    /// Sorted by `path`, so the rendering is stable and two scans of one folder produce the
    /// same id.
    pub entries: Vec<Entry>,
}

/// What a launcher must do to match a server.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plan {
    /// Files to download, in manifest order. Missing locally, or present with the wrong
    /// contents.
    pub fetch: Vec<Entry>,
}

impl Plan {
    /// Nothing to do: this client already matches.
    pub fn is_current(&self) -> bool {
        self.fetch.is_empty()
    }

    /// Total bytes to download.
    pub fn bytes(&self) -> u64 {
        self.fetch.iter().map(|e| e.size).sum()
    }

    /// One line for a log or a launcher window.
    pub fn describe(&self) -> String {
        if self.is_current() {
            return "client is up to date".to_string();
        }
        let mb = self.bytes() as f64 / (1024.0 * 1024.0);
        format!(
            "{} file(s) to update, {mb:.1} MB: {}",
            self.fetch.len(),
            self.fetch
                .iter()
                .take(6)
                .map(|e| e.path.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// **Is this path one a manifest may carry, and a server may be asked for?**
///
/// The same predicate guards the parser and the file-serving endpoint, deliberately: a
/// manifest is data that arrives over the network, and `../../Windows/System32/...` in one of
/// its lines must not become a write on a player's disk or a read on the server's. Rejects an
/// empty path, an absolute one, a Windows drive letter, a UNC prefix, a backslash (paths are
/// normalised to forward slashes before they get here), any `.` or `..` segment, and a
/// trailing or doubled separator.
pub fn is_safe_relative(path: &str) -> bool {
    if path.is_empty() || path.len() > 1024 {
        return false;
    }
    if path.contains('\\') || path.starts_with('/') || path.contains("//") {
        return false;
    }
    // A drive letter (`C:`) or any other colon: no path in a client folder needs one, and on
    // Windows `C:foo` is a *relative* path on drive C, which `Path::join` will happily follow.
    if path.contains(':') {
        return false;
    }
    if path.ends_with('/') {
        return false;
    }
    path.split('/').all(|seg| !seg.is_empty() && seg != "." && seg != "..")
}

/// **Files that are never part of "which version is this".**
///
/// Everything here differs between two machines running the identical client, so including
/// any of it would leave every client permanently out of date. See the module docs.
///
/// `rel` is a manifest-style relative path: forward slashes, no leading slash.
pub fn is_volatile(rel: &str) -> bool {
    let lower = rel.to_ascii_lowercase();
    let name = lower.rsplit('/').next().unwrap_or(&lower);

    // Whole directories the launcher or the client owns.
    for dir in ["grap.disabled/", "grap/", "dumps/", "previous-runs/"] {
        if lower.starts_with(dir) {
            return true;
        }
    }
    // The hook's markers and its log, all named maplecw-hook.*
    if name.starts_with("maplecw-hook.") {
        return true;
    }
    // **grap64.dll is the LAUNCHER's, not the client payload's.** It is the GameGuard stub,
    // compiled into the launcher binary and written over this file on every Start Game when
    // the bytes differ (`launcher::client::stub_gameguard`). Versioning it here would give one
    // file two owners: the patcher would fetch the canonical copy, the launcher would overwrite
    // it with its own on the next launch, and the launch after that would fetch it again -
    // 476 KB of churn forever, the same shape as the MapleStory.exe trap in the module docs.
    // Its version travels with the launcher, which is the thing that produces it.
    if name == "grap64.dll" {
        return true;
    }
    // The backup the GameGuard stubbing leaves, and any other .orig it keeps.
    if name.ends_with(".orig") || name.ends_with(".before-patch") || name.ends_with(".bak") {
        return true;
    }
    // A half-written patch file. `launcher::clientpatch` writes to this name and renames over
    // the target, so one only exists if a patch was interrupted - and a leftover must not turn
    // up in the next scan as a file the server has never heard of.
    if name.ends_with(".maplecw-part") {
        return true;
    }
    // Logs, crash reports and the screenshots the client drops beside itself (Maple_A_*.jpg).
    if name.ends_with(".log") || name.ends_with(".err") {
        return true;
    }
    if name.starts_with("maple_a_") && name.ends_with(".jpg") {
        return true;
    }
    // The launcher's own files, when a client folder happens to sit beside them.
    if name.starts_with("maplecw-launcher") {
        return true;
    }
    false
}

impl Manifest {
    /// **Walk a client folder and hash everything in it.**
    ///
    /// Recursive, skipping [`is_volatile`] paths. Returns entries sorted by path. A file that
    /// cannot be read is an error rather than a silent omission: a manifest missing a file is
    /// indistinguishable from a client that does not have it, and on the server side that
    /// would quietly stop shipping a patch.
    pub fn scan(root: &Path) -> std::io::Result<Manifest> {
        let mut entries = Vec::new();
        walk(root, root, &mut entries)?;
        entries.sort();
        Ok(Manifest { entries })
    }

    /// The manifest as its text table, ending in a newline.
    pub fn render(&self) -> String {
        let mut out = String::with_capacity(64 + self.entries.len() * 96);
        out.push_str(HEADER);
        out.push('\n');
        for e in &self.entries {
            out.push_str(&e.sha256);
            out.push('\t');
            out.push_str(&e.size.to_string());
            out.push('\t');
            out.push_str(&e.path);
            out.push('\n');
        }
        out
    }

    /// Parse a manifest, refusing anything malformed rather than skipping the bad line.
    ///
    /// A dropped line is a file that silently stops being checked, so every failure here is
    /// reported with the line number and the reason.
    pub fn parse(text: &str) -> Result<Manifest, String> {
        let mut lines = text.lines();
        match lines.next() {
            Some(first) if first.trim_end() == HEADER => {}
            Some(other) => return Err(format!("not a client manifest: first line is {other:?}")),
            None => return Err("empty manifest".to_string()),
        }
        let mut entries = Vec::new();
        for (i, line) in lines.enumerate() {
            let no = i + 2;
            if line.trim().is_empty() {
                continue;
            }
            let mut parts = line.split('\t');
            let (Some(sha), Some(size), Some(path)) = (parts.next(), parts.next(), parts.next())
            else {
                return Err(format!("line {no}: expected sha<TAB>size<TAB>path"));
            };
            if parts.next().is_some() {
                return Err(format!("line {no}: a tab in the path, which is not allowed"));
            }
            if sha.len() != 64 || !sha.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err(format!("line {no}: {sha:?} is not a sha256"));
            }
            let size: u64 =
                size.parse().map_err(|_| format!("line {no}: {size:?} is not a size"))?;
            if !is_safe_relative(path) {
                return Err(format!("line {no}: {path:?} is not a safe relative path"));
            }
            entries.push(Entry {
                path: path.to_string(),
                size,
                sha256: sha.to_ascii_lowercase(),
            });
        }
        entries.sort();
        Ok(Manifest { entries })
    }

    /// **The one short string two sides compare.** SHA-256 of [`Manifest::render`]'s bytes.
    pub fn id(&self) -> String {
        let mut h = Sha256::new();
        h.update(self.render().as_bytes());
        hex::encode(h.finalize())
    }

    /// The first 16 characters of [`Manifest::id`], for a log line.
    pub fn short_id(&self) -> String {
        self.id()[..16].to_string()
    }

    pub fn get(&self, path: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.path == path)
    }

    pub fn total_bytes(&self) -> u64 {
        self.entries.iter().map(|e| e.size).sum()
    }

    /// **What this client must download to become `want`.**
    ///
    /// `self` is what is on disk. Every entry of `want` that is missing here, or here with a
    /// different hash, goes in the plan. Files present locally and absent from `want` are left
    /// alone - see the module docs.
    pub fn plan(&self, want: &Manifest) -> Plan {
        let fetch = want
            .entries
            .iter()
            .filter(|w| match self.get(&w.path) {
                Some(local) => local.sha256 != w.sha256,
                None => true,
            })
            .cloned()
            .collect();
        Plan { fetch }
    }
}

/// Hash one file, streaming, so a 159 MB archive does not become 159 MB of memory.
pub fn hash_file(path: &Path) -> std::io::Result<(u64, String)> {
    use std::io::Read;
    let mut f = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1024 * 1024];
    let mut size = 0u64;
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        size += n as u64;
        hasher.update(&buf[..n]);
    }
    Ok((size, hex::encode(hasher.finalize())))
}

/// Hash bytes already in memory - what a launcher checks a freshly downloaded file with.
pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn walk(root: &Path, dir: &Path, out: &mut Vec<Entry>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let Some(rel) = relative_of(root, &path) else { continue };
        if is_volatile(&rel) {
            continue;
        }
        if entry.file_type()?.is_dir() {
            walk(root, &path, out)?;
        } else if entry.file_type()?.is_file() {
            if !is_safe_relative(&rel) {
                continue;
            }
            let (size, sha256) = hash_file(&path)?;
            out.push(Entry { path: rel, size, sha256 });
        }
    }
    Ok(())
}

/// A path under `root`, as manifest-style forward slashes.
fn relative_of(root: &Path, path: &Path) -> Option<String> {
    let rel: PathBuf = path.strip_prefix(root).ok()?.to_path_buf();
    let mut s = String::new();
    for part in rel.components() {
        let std::path::Component::Normal(seg) = part else { return None };
        if !s.is_empty() {
            s.push('/');
        }
        s.push_str(&seg.to_string_lossy());
    }
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Temp(PathBuf);
    impl Temp {
        fn new(tag: &str) -> Temp {
            let p = std::env::temp_dir().join(format!(
                "maplecw-patchset-{tag}-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = std::fs::remove_dir_all(&p);
            std::fs::create_dir_all(&p).unwrap();
            Temp(p)
        }
        fn file(&self, rel: &str, body: &[u8]) {
            let p = self.0.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, body).unwrap();
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn a_scan_finds_every_file_with_its_size_and_hash() {
        let t = Temp::new("scan");
        t.file("MapleStory.exe", b"exe bytes");
        t.file("Data/Item/Pet/Pet_000.wz", b"pet archive");
        let m = Manifest::scan(t.path()).unwrap();
        assert_eq!(
            m.entries.iter().map(|e| e.path.as_str()).collect::<Vec<_>>(),
            vec!["Data/Item/Pet/Pet_000.wz", "MapleStory.exe"],
            "sorted by path, forward slashes, relative"
        );
        let exe = m.get("MapleStory.exe").unwrap();
        assert_eq!(exe.size, 9);
        assert_eq!(exe.sha256, hash_bytes(b"exe bytes"));
    }

    /// **The trap.** Anything the launcher or the client writes per machine must not count.
    #[test]
    fn volatile_files_are_not_part_of_the_version() {
        let t = Temp::new("volatile");
        t.file("MapleStory.exe", b"exe");
        t.file("maplecw-hook.log", b"log");
        t.file("maplecw-hook.session", b"marker");
        t.file("grap64.dll.orig", b"the real gameguard");
        t.file("grap.disabled/x.dll", b"disabled");
        t.file("dumps/crash.dmp", b"dump");
        t.file("Maple_A_260913_040540.jpg", b"screenshot");
        t.file("world.log", b"log");
        t.file("Data/Base/Base.wz", b"real data");

        let m = Manifest::scan(t.path()).unwrap();
        assert_eq!(
            m.entries.iter().map(|e| e.path.as_str()).collect::<Vec<_>>(),
            vec!["Data/Base/Base.wz", "MapleStory.exe"],
            "only the files that define the version: {:?}",
            m.entries
        );
    }

    /// **Two owners for one file is the bug this prevents.** `grap64.dll` is the launcher's
    /// own stub, rewritten on every Start Game; `MapleStory.exe` comes from the payload and the
    /// launcher only flips one idempotent byte in it, so that one IS versioned.
    #[test]
    fn the_gameguard_stub_is_the_launchers_and_the_exe_is_the_payloads() {
        assert!(is_volatile("grap64.dll"), "the launcher writes this one; it has one owner");
        assert!(is_volatile("grap64.dll.orig"), "and its backup is per machine");
        assert!(
            !is_volatile("MapleStory.exe"),
            "the exe is the client's version; the gate patch is idempotent so it does not churn"
        );
    }

    #[test]
    fn a_manifest_round_trips_through_its_text_form() {
        let t = Temp::new("round");
        t.file("a.txt", b"a");
        t.file("d/b.txt", b"b");
        let m = Manifest::scan(t.path()).unwrap();
        let back = Manifest::parse(&m.render()).unwrap();
        assert_eq!(m, back);
        assert_eq!(m.id(), back.id());
        assert!(m.render().starts_with(HEADER));
        assert!(m.render().ends_with('\n'));
    }

    #[test]
    fn the_id_changes_when_any_byte_of_any_file_changes() {
        let t = Temp::new("id");
        t.file("a.txt", b"one");
        let before = Manifest::scan(t.path()).unwrap().id();
        t.file("a.txt", b"two");
        let after = Manifest::scan(t.path()).unwrap().id();
        assert_ne!(before, after);
    }

    /// **The whole point**: only what moved is downloaded.
    #[test]
    fn a_plan_names_only_the_files_that_differ_or_are_missing() {
        let server = Manifest::parse(&format!(
            "{HEADER}\n{}\t3\tsame.wz\n{}\t3\tmoved.wz\n{}\t3\tnew.wz\n",
            hash_bytes(b"aaa"),
            hash_bytes(b"bbb"),
            hash_bytes(b"ccc")
        ))
        .unwrap();
        let local = Manifest::parse(&format!(
            "{HEADER}\n{}\t3\tsame.wz\n{}\t3\tmoved.wz\n{}\t5\textra.wz\n",
            hash_bytes(b"aaa"),
            hash_bytes(b"OLD"),
            hash_bytes(b"local")
        ))
        .unwrap();

        let plan = local.plan(&server);
        assert_eq!(
            plan.fetch.iter().map(|e| e.path.as_str()).collect::<Vec<_>>(),
            vec!["moved.wz", "new.wz"],
            "the unchanged file is not re-downloaded and the extra local file is left alone"
        );
        assert_eq!(plan.bytes(), 6);
        assert!(!plan.is_current());
        assert!(Manifest::parse(&server.render()).unwrap().plan(&server).is_current());
    }

    /// A manifest arrives over the network. A path in it must never escape the client folder.
    #[test]
    fn a_path_that_escapes_the_client_folder_is_refused() {
        for bad in [
            "../outside.dll",
            "a/../../b",
            "/absolute",
            "C:/Windows/System32/x.dll",
            "C:relative-on-c",
            "back\\slash",
            "trailing/",
            "double//slash",
            "",
            ".",
            "..",
            "a/./b",
        ] {
            assert!(!is_safe_relative(bad), "{bad:?} must be refused");
            let text = format!("{HEADER}\n{}\t1\t{bad}\n", hash_bytes(b"x"));
            assert!(Manifest::parse(&text).is_err(), "{bad:?} must not parse");
        }
        for good in ["a.wz", "Data/Item/Pet/Pet_000.wz", "a b/c d.wz"] {
            assert!(is_safe_relative(good), "{good:?} must be allowed");
        }
    }

    #[test]
    fn a_malformed_manifest_is_refused_rather_than_partly_read() {
        assert!(Manifest::parse("").is_err());
        assert!(Manifest::parse("something else\n").is_err());
        assert!(Manifest::parse(&format!("{HEADER}\nnotahash\t1\ta\n")).is_err());
        assert!(Manifest::parse(&format!("{HEADER}\n{}\tnotasize\ta\n", hash_bytes(b"x"))).is_err());
        assert!(Manifest::parse(&format!("{HEADER}\n{}\t1\n", hash_bytes(b"x"))).is_err());
        // Blank lines are fine.
        assert!(Manifest::parse(&format!("{HEADER}\n\n{}\t1\ta\n", hash_bytes(b"x"))).is_ok());
    }
}
