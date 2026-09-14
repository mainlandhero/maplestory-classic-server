//! **Ask the server which client it expects, and fetch only the files that differ.**
//!
//! The owner, 2026-09-14: *"an integrity check that reaches out to the server to make sure that
//! this is the correct version that the client should be running on. Older clients will be
//! forced to patch to be up to date, and we need to ensure that the server delivers those
//! patches to the client so the client don't have to re-download everything from a package
//! every time."*
//!
//! [`crate::prepare`] calls [`check_and_patch`] on every Start Game, **after** the launcher's
//! own patch steps - the GameGuard stub and the Nexon gate byte - because the canonical client
//! on the server is a prepared one and the two must have had the same things done to them.
//! `patchset`'s module docs are where that trap is written down.
//!
//! # Blocking is the point, and it is a real trade
//!
//! The owner chose *block always*: the game does not start unless this launcher has **positively
//! confirmed** its version with the server. That is stronger than the rest of this codebase's
//! standing rule that a check which cannot read must not lock anybody out, and the difference
//! is deliberate - a client whose WZ disagrees with the server's data is a broken session, not
//! a degraded one.
//!
//! What it costs is stated here so nobody has to rediscover it: **if the patch server is
//! unreachable, or was started without `--client-dir`, nobody can play.** Every refusal below
//! says which of those it is, in words, because those two have different fixes and only one of
//! them is the player's.
//!
//! # A download is verified before it is installed, and installed atomically
//!
//! Each file is hashed after it arrives and refused if it does not match the manifest - a
//! truncated 13 MB archive is exactly the kind of thing that would otherwise be written over a
//! working one and turn into a client that starts and then cannot load a map. It is written to
//! `<name>.part` and renamed into place, so an interrupted patch leaves the old file whole.

use std::path::Path;

use patchset::Manifest;
use tlspin::Fingerprint;

use crate::prepare::Level;

/// Where the launcher asks. Mirrors `auth::clientpatch`, which serves them.
const MANIFEST_PATH: &str = "/client/manifest";
const FILE_PATH: &str = "/client/file";

/// What happened, so the caller can refuse the launch and say why.
#[derive(Debug)]
pub enum Outcome {
    /// The local folder matches the server, byte for byte.
    UpToDate { version: String },
    /// It did not, and these files were fetched and installed. It matches now.
    Patched { version: String, files: usize, bytes: u64 },
}

impl Outcome {
    /// The version this client is confirmed to be running, either way.
    ///
    /// The caller puts it in one last line, so the launcher log always ends with the version
    /// that was actually confirmed - the same reason every server prints its build stamp.
    pub fn version(&self) -> &str {
        match self {
            Outcome::UpToDate { version } | Outcome::Patched { version, .. } => version,
        }
    }
}

/// **Confirm this client against the server, patching it if it is behind.**
///
/// `Err` means the launch must not proceed - see the module docs on blocking. The message is
/// written for a player reading it in the launcher window.
pub fn check_and_patch(
    client_dir: &Path,
    host: &str,
    port: u16,
    pin: &Fingerprint,
    log: &mut dyn FnMut(Level, String),
) -> Result<Outcome, String> {
    log(
        Level::Info,
        format!("checking this client against {host}:{port} before starting the game"),
    );

    let want = fetch_manifest(host, port, pin)?;
    let have = Manifest::scan(client_dir).map_err(|e| {
        format!(
            "could not read the client folder to check it: {e}\n{}",
            client_dir.display()
        )
    })?;

    let version = want.short_id();
    let plan = have.plan(&want);
    if plan.is_current() {
        log(
            Level::Good,
            format!(
                "client version {version} confirmed by the server - {} file(s) checked, nothing \
                 to update",
                have.entries.len()
            ),
        );
        return Ok(Outcome::UpToDate { version });
    }

    log(
        Level::Warn,
        format!(
            "this client is out of date: the server expects version {version} and {}. \
             Downloading only what changed",
            plan.describe()
        ),
    );

    let mut done = 0usize;
    for entry in &plan.fetch {
        let bytes = fetch_file(host, port, pin, &entry.path)?;
        // **Verified before it is installed.** A short or corrupted body must never land on
        // top of a working archive.
        let got = patchset::hash_bytes(&bytes);
        if got != entry.sha256 || bytes.len() as u64 != entry.size {
            return Err(format!(
                "the patch for {} did not survive the download: expected {} bytes hashing to \
                 {}, got {} bytes hashing to {}. Nothing was written. Try again; if it keeps \
                 happening the server's copy and its manifest disagree",
                entry.path,
                entry.size,
                &entry.sha256[..16],
                bytes.len(),
                &got[..16]
            ));
        }
        install(client_dir, &entry.path, &bytes)?;
        done += 1;
        log(
            Level::Good,
            format!(
                "patched {} ({}/{}, {:.1} MB)",
                entry.path,
                done,
                plan.fetch.len(),
                entry.size as f64 / (1024.0 * 1024.0)
            ),
        );
    }

    // **Re-scan and prove it.** The plan said what to fetch; only a second scan says the folder
    // now matches. Without this a file that could not be written - a locked archive, a full
    // disk - would be reported as patched and the player would start a client that is still
    // behind.
    let after = Manifest::scan(client_dir).map_err(|e| format!("re-reading the client: {e}"))?;
    let left = after.plan(&want);
    if !left.is_current() {
        return Err(format!(
            "the client is still out of date after patching: {}. Is the game or a file browser \
             holding one of those files open?",
            left.describe()
        ));
    }

    Ok(Outcome::Patched { version, files: plan.fetch.len(), bytes: plan.bytes() })
}

/// Write one patched file atomically: to `.part`, then rename over the original.
fn install(client_dir: &Path, rel: &str, bytes: &[u8]) -> Result<(), String> {
    if !patchset::is_safe_relative(rel) {
        // Belt and braces: the manifest parser already refused this shape. A path from the
        // network that becomes a write on a player's disk gets checked at the write too.
        return Err(format!("refusing to write {rel:?}: not a safe relative path"));
    }
    let mut target = client_dir.to_path_buf();
    for seg in rel.split('/') {
        target.push(seg);
    }
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("could not make {}: {e}", parent.display()))?;
    }
    // Appended, not `with_extension`, which REPLACES: two files differing only in extension
    // would otherwise share one temporary name.
    let part = std::path::PathBuf::from(format!("{}.maplecw-part", target.display()));
    std::fs::write(&part, bytes).map_err(|e| format!("could not write {}: {e}", part.display()))?;
    std::fs::rename(&part, &target).map_err(|e| {
        let _ = std::fs::remove_file(&part);
        format!("could not replace {}: {e}", target.display())
    })?;
    Ok(())
}

/// `GET /client/manifest`.
fn fetch_manifest(host: &str, port: u16, pin: &Fingerprint) -> Result<Manifest, String> {
    let (status, body) = crate::http::get_bytes(host, port, pin, MANIFEST_PATH)?;
    if status == 503 {
        return Err(format!(
            "the server at {host}:{port} publishes no client, so this launcher cannot confirm \
             which version to run and will not start the game.\nOn the server box, start \
             maplecw-auth with --client-dir pointing at the client folder to publish."
        ));
    }
    if status != 200 {
        return Err(format!(
            "the server at {host}:{port} answered {status} for the client manifest, so the \
             version could not be confirmed and the game will not be started."
        ));
    }
    let text = String::from_utf8(body)
        .map_err(|_| "the client manifest was not text - is that really a MapleCW server?".to_string())?;
    Manifest::parse(&text).map_err(|e| format!("the server's client manifest is unreadable: {e}"))
}

/// `GET /client/file?path=...`
fn fetch_file(host: &str, port: u16, pin: &Fingerprint, rel: &str) -> Result<Vec<u8>, String> {
    let path = format!("{FILE_PATH}?path={}", percent_encode(rel));
    let (status, body) = crate::http::get_bytes(host, port, pin, &path)?;
    if status != 200 {
        return Err(format!(
            "the server answered {status} for the patch file {rel}, so this client cannot be \
             brought up to date and the game will not be started."
        ));
    }
    Ok(body)
}

/// Percent-encode everything that is not unreserved, so a path with a space or a `+` survives.
///
/// `/` is encoded too: it is a path *inside* the client folder and the server decodes it back,
/// so leaving it raw would work but encoding it keeps the query one opaque value.
fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::TempDir;

    #[test]
    fn a_path_is_encoded_so_the_server_decodes_the_same_string_back() {
        assert_eq!(percent_encode("a.wz"), "a.wz");
        assert_eq!(percent_encode("Data/Item/Pet_000.wz"), "Data%2FItem%2FPet_000.wz");
        assert_eq!(percent_encode("a b+c.wz"), "a%20b%2Bc.wz");
        // Round-trips through the server's own decoder.
        for s in ["a.wz", "Data/Item/Pet/Pet_000.wz", "a b+c.wz"] {
            let q = format!("path={}", percent_encode(s));
            assert_eq!(auth_path_param(&q).as_deref(), Some(s));
        }
    }

    /// The server's decoder, copied here so the two cannot drift without a test failing.
    /// `crates/auth/src/clientpatch.rs::path_param` is the original.
    fn auth_path_param(query: &str) -> Option<String> {
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

    #[test]
    fn a_patched_file_lands_in_place_and_leaves_no_part_file() {
        let t = TempDir::new("install");
        let dir = t.path();
        std::fs::write(dir.join("old.wz"), b"old").unwrap();
        install(dir, "old.wz", b"new").unwrap();
        assert_eq!(std::fs::read(dir.join("old.wz")).unwrap(), b"new");

        // A file in a subdirectory that does not exist yet.
        install(dir, "Data/Item/Pet_000.wz", b"pet").unwrap();
        assert_eq!(std::fs::read(dir.join("Data/Item/Pet_000.wz")).unwrap(), b"pet");

        let leftovers: Vec<String> = walk_names(dir)
            .into_iter()
            .filter(|n| n.contains("maplecw-part"))
            .collect();
        assert!(leftovers.is_empty(), "a .part file was left behind: {leftovers:?}");
    }

    /// A path from the network is refused at the write as well as at the parse.
    #[test]
    fn install_refuses_a_path_that_escapes_the_client_folder() {
        let t = TempDir::new("escape");
        for bad in ["../outside.wz", "/abs.wz", "C:/Windows/win.ini", "a/../../b.wz"] {
            assert!(install(t.path(), bad, b"x").is_err(), "{bad} must be refused");
        }
    }

    fn walk_names(dir: &Path) -> Vec<String> {
        let mut out = Vec::new();
        if let Ok(rd) = std::fs::read_dir(dir) {
            for e in rd.flatten() {
                let p = e.path();
                out.push(p.file_name().unwrap().to_string_lossy().into_owned());
                if p.is_dir() {
                    out.extend(walk_names(&p));
                }
            }
        }
        out
    }
}
