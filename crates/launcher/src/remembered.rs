//! What the launcher remembers between runs: the game folder the player chose.
//!
//! The owner, 2026-09-05: *"different clients may change their MapleStory.exe client location,
//! does our launcher save whatever the user set it to upon subsequent starts? Setting it every
//! time is going to be very frustrating for users."* It did not. Browse and a typed path only
//! changed the in-memory layout, and the next start went back to the resolver's guess.
//!
//! # Where it lives, and why not in the user's profile
//!
//! `maplecw-launcher.remembered.toml`, **beside the executable**, next to the operator's
//! `maplecw-launcher.toml`. Not in `%LOCALAPPDATA%`, which is where a per-user preference
//! would normally go: the launcher runs elevated (`build.rs` embeds `requireAdministrator`),
//! and under an over-the-shoulder UAC prompt the elevated process runs as the *administrator*
//! account, so its `%LOCALAPPDATA%` is the administrator's profile, not the player's. A
//! preference saved there would vanish the moment a different account approved the prompt.
//! Beside the executable is the same place for everybody who runs this copy, which is what
//! "the folder I picked last time" means on a shared machine anyway.
//!
//! # Why a separate file rather than rewriting `maplecw-launcher.toml`
//!
//! That file is the operator's. It carries the server address and the certificate pin, and it
//! carries comments; rewriting it from a parsed copy would drop the comments and put the
//! launcher's hand on settings it has no business changing. A second file with one key keeps
//! "what the installer said" and "what the player chose" apart, and deleting it is the whole
//! of "forget my choice".
//!
//! # Format
//!
//! The same literal `key = "value"` shape as [`crate::config`], read by the same parser, so a
//! Windows path with `\U` in it is a path and not an escape. One key, `client_dir`.

use std::io;
use std::path::{Path, PathBuf};

use crate::config;

/// The file name, beside the executable.
pub const FILE_NAME: &str = "maplecw-launcher.remembered.toml";

/// What a read found.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Remembered {
    /// The game folder, exactly as saved. Resolved against the executable's directory by
    /// [`crate::paths`], like every other configured path.
    pub client_dir: Option<String>,
    /// The server the last successful Start Game reached. An address or a name, as typed.
    pub server_ip: Option<String>,
    /// The login port that went with it.
    pub port: Option<u16>,
    /// The account name or email that was in the box. **Never the password** - see
    /// [`render`].
    pub identity: Option<String>,
    /// Lines that could not be used, with a reason. Surfaced, never fatal.
    pub problems: Vec<String>,
}

/// Read the remembered file out of `dir`, if there is one. `None` when there is not.
pub fn load(dir: &Path) -> Option<(PathBuf, Remembered)> {
    let path = dir.join(FILE_NAME);
    let text = std::fs::read_to_string(&path).ok()?;
    Some((path, parse(&text)))
}

/// Parse the text. Pure; the tests drive this.
///
/// Anything other than `client_dir` is reported rather than obeyed: this file is written by
/// the launcher, so an unexpected key in it means somebody edited it by hand, and the right
/// answer to that is a line saying so, not a silent setting.
pub fn parse(text: &str) -> Remembered {
    let cfg = config::parse(text);
    let mut problems = cfg.problems;
    // Still rejected: these are an OPERATOR's choices, not the player's, and none of them is
    // editable in the window - so a value here came from somebody hand-editing the file, and
    // the right answer to that is a line saying where it belongs.
    for (key, present) in [
        ("stub_path", cfg.stub_path.is_some()),
        ("auth_port", cfg.auth_port.is_some()),
        ("auth_fingerprint", cfg.auth_fingerprint.is_some()),
    ] {
        if present {
            problems.push(format!(
                "`{key}` is not something the launcher remembers; set it in {} instead",
                config::CONFIG_FILE_NAME
            ));
        }
    }
    Remembered {
        client_dir: cfg.client_dir,
        server_ip: cfg.server_ip,
        port: cfg.port,
        identity: cfg.identity,
        problems,
    }
}

/// What the launcher will write. Separate from [`save`] so the round trip is testable
/// without a disk.
///
/// # There is no password here, and there never will be
///
/// `CLAUDE.md`'s first standing constraint is that passwords are never stored in plain text.
/// This file is plain text beside the executable, so the account NAME is remembered and the
/// password is not: the player types eight characters instead of forty, and nothing on disk
/// would let somebody else sign in. If a future key ever looks like a credential, it belongs
/// in the sign-in service behind argon2id, not here.
///
/// # Values are written only when they are worth remembering
///
/// An empty box means "I did not set this", and writing `server_ip = ""` would come back on
/// the next start as a configured empty address, which reads as a launcher that forgot in a
/// way nobody can see. Absent keys fall through to the config file, which is the behaviour
/// somebody re-running `install.ps1` expects.
pub fn render(
    client_dir: Option<&Path>,
    server_ip: Option<&str>,
    port: Option<u16>,
    identity: Option<&str>,
) -> String {
    let mut out = format!(
        "# Written by maplecw-launcher after a successful Start Game. It is read on the next\n\
         # start, AHEAD of {}. Delete this file to go back to the defaults.\n\
         # Paths are literal - single backslashes, no escaping.\n\
         # Your password is NOT here and never will be.\n",
        config::CONFIG_FILE_NAME
    );
    if let Some(v) = client_dir {
        out.push_str(&format!("client_dir = \"{}\"\n", v.display()));
    }
    if let Some(v) = server_ip.map(str::trim).filter(|v| !v.is_empty()) {
        out.push_str(&format!("server_ip = \"{v}\"\n"));
    }
    if let Some(v) = port {
        out.push_str(&format!("port = \"{v}\"\n"));
    }
    if let Some(v) = identity.map(str::trim).filter(|v| !v.is_empty()) {
        out.push_str(&format!("identity = \"{v}\"\n"));
    }
    out
}

/// Kept so a Browse with nothing else known still writes a usable file.
pub fn render_client_dir(client_dir: &Path) -> String {
    render(Some(client_dir), None, None, None)
}

/// Remember `client_dir` in `dir`. Returns the file written.
///
/// Written to a sibling and renamed into place, so a crash mid-write leaves the previous
/// choice rather than half a line - a half line would parse as "no `=`" and read on the next
/// start as the launcher having forgotten, which is the failure this module exists to remove.
pub fn save_client_dir(dir: &Path, client_dir: &Path) -> io::Result<PathBuf> {
    write_file(dir, render_client_dir(client_dir))
}

/// Remember everything the window lets a player set, after a launch that worked.
///
/// The owner, 2026-09-07: *"if the Start Game is successful, it should automatically save all of
/// the settings in a file so it will remember."* Until this, only the game folder survived a
/// restart, so a player on a machine whose config file named the wrong server retyped the
/// address every single time.
///
/// **Success is the trigger on purpose.** Settings that got as far as launching the client are
/// known to work; settings that failed are exactly the ones nobody wants back next time.
pub fn save(
    dir: &Path,
    client_dir: &Path,
    server_ip: &str,
    port: u16,
    identity: &str,
) -> io::Result<PathBuf> {
    write_file(
        dir,
        render(Some(client_dir), Some(server_ip), Some(port), Some(identity)),
    )
}

fn write_file(dir: &Path, body: String) -> io::Result<PathBuf> {
    let path = dir.join(FILE_NAME);
    let tmp = dir.join(format!("{FILE_NAME}.tmp"));
    std::fs::write(&tmp, body)?;
    if let Err(e) = std::fs::rename(&tmp, &path) {
        // `rename` onto an existing file is a replace on every Windows since Vista, but a
        // reader holding the old file open can still make it fail. Clean up and report; the
        // caller turns this into a log line.
        let _ = std::fs::remove_file(&tmp);
        return Err(e);
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::TempDir;

    #[test]
    fn a_saved_folder_reads_back_exactly() {
        let t = TempDir::new("remembered");
        let chosen = Path::new(r"D:\Games\Some #odd\MapleStory Classic");
        let written = save_client_dir(t.path(), chosen).unwrap();
        assert_eq!(written, t.path().join(FILE_NAME));

        let (path, got) = load(t.path()).expect("the file was just written");
        assert_eq!(path, written);
        assert_eq!(got.client_dir.as_deref(), Some(chosen.to_str().unwrap()));
        assert!(got.problems.is_empty(), "{:?}", got.problems);
        // And no temp file left behind.
        assert!(!t.path().join(format!("{FILE_NAME}.tmp")).exists());
    }

    #[test]
    fn a_second_save_replaces_the_first() {
        let t = TempDir::new("remembered-replace");
        save_client_dir(t.path(), Path::new(r"C:\first")).unwrap();
        save_client_dir(t.path(), Path::new(r"C:\second")).unwrap();
        let (_, got) = load(t.path()).unwrap();
        assert_eq!(got.client_dir.as_deref(), Some(r"C:\second"));
    }

    #[test]
    fn no_file_is_none_not_an_error() {
        let t = TempDir::new("remembered-none");
        assert!(load(t.path()).is_none());
    }

    #[test]
    fn the_rendered_text_is_the_literal_shape_the_config_reader_wants() {
        // `\U` must survive: the whole reason the reader is not a TOML parser.
        let text = render_client_dir(Path::new(r"C:\MapleCW\client-patched"));
        let got = parse(&text);
        assert_eq!(
            got.client_dir.as_deref(),
            Some(r"C:\MapleCW\client-patched")
        );
        assert!(got.problems.is_empty(), "{:?}", got.problems);
        // The file explains itself, because the next person to find it will not have read
        // this module.
        assert!(text.contains("Delete this file"));
    }

    /// **What a successful Start Game remembers, and what stays the operator's.**
    ///
    /// This test used to assert that `server_ip` here was rejected. It changed deliberately on
    /// 2026-09-07 - the owner: *"if the Start Game is successful, it should automatically save all
    /// of the settings in a file so it will remember."* The split that remains is between what
    /// the WINDOW lets a player set and what only an operator can: the first three are boxes on
    /// screen, the last three are not, so a value for them here was hand-edited into the wrong
    /// file and gets a line saying so.
    #[test]
    fn the_players_settings_are_remembered_and_the_operators_are_not() {
        let got = parse(
            "client_dir = C:\\x\nserver_ip = 10.0.0.9\nport = 8484\nidentity = cobalt\n",
        );
        assert_eq!(got.client_dir.as_deref(), Some("C:\\x"));
        assert_eq!(got.server_ip.as_deref(), Some("10.0.0.9"));
        assert_eq!(got.port, Some(8484));
        assert_eq!(got.identity.as_deref(), Some("cobalt"));
        assert!(got.problems.is_empty(), "{:?}", got.problems);

        for key in ["stub_path = C:\\y", "auth_port = 8480", "auth_fingerprint = sha256:ab"] {
            let got = parse(&format!("client_dir = C:\\x\n{key}\n"));
            assert_eq!(got.client_dir.as_deref(), Some("C:\\x"), "{key}");
            assert_eq!(got.problems.len(), 1, "{key}: {:?}", got.problems);
            assert!(
                got.problems[0].contains(config::CONFIG_FILE_NAME),
                "{key}: the line must say where it belongs - {:?}",
                got.problems
            );
        }
    }

    /// **The password is not in the file, and an empty box does not become a setting.**
    ///
    /// The first half is `CLAUDE.md`'s standing constraint and this file is plain text beside
    /// the executable. The second is subtler: writing `server_ip = ""` would come back on the
    /// next start as a configured empty address, which looks like a launcher that forgot in a
    /// way nobody can see, and would shadow the config file while doing it.
    #[test]
    fn nothing_secret_is_written_and_empty_boxes_are_left_out() {
        let full = render(
            Some(Path::new(r"C:\MapleCW\client")),
            Some("  10.0.0.9  "),
            Some(8484),
            Some(" cobalt "),
        );
        // Checked over the KEYS, not the prose: the header says the word "password" on
        // purpose, and a substring search over the whole file would pass only while nobody
        // reassured the reader. The first version of this test failed on its own comment.
        let keys: Vec<&str> = full
            .lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .filter_map(|l| l.split('=').next())
            .map(str::trim)
            .filter(|k| !k.is_empty())
            .collect();
        assert_eq!(keys, ["client_dir", "server_ip", "port", "identity"], "{full}");
        for k in &keys {
            let k = k.to_ascii_lowercase();
            assert!(
                !k.contains("pass") && !k.contains("secret") && !k.contains("token"),
                "`{k}` looks like a credential and this file is plain text"
            );
        }
        assert!(full.contains("Your password is NOT here"), "{full}");
        // Trimmed, not stored with the spaces the box had.
        assert!(full.contains("server_ip = \"10.0.0.9\""), "{full}");
        assert!(full.contains("identity = \"cobalt\""), "{full}");

        let sparse = render(Some(Path::new(r"C:\MapleCW\client")), Some("   "), None, Some(""));
        assert!(!sparse.contains("server_ip"), "an empty box must not be written: {sparse}");
        assert!(!sparse.contains("identity"), "{sparse}");
        assert!(!sparse.contains("port"), "{sparse}");
        assert!(sparse.contains("client_dir"), "{sparse}");

        // And the round trip: what render writes, parse reads back unchanged.
        let back = parse(&full);
        assert_eq!(back.server_ip.as_deref(), Some("10.0.0.9"));
        assert_eq!(back.port, Some(8484));
        assert_eq!(back.identity.as_deref(), Some("cobalt"));
        assert!(back.problems.is_empty(), "{:?}", back.problems);
    }

    #[test]
    fn an_empty_or_broken_file_remembers_nothing_and_says_why() {
        assert_eq!(parse(""), Remembered::default());
        let got = parse("client_dir C:\\x");
        assert_eq!(got.client_dir, None);
        assert_eq!(got.problems.len(), 1);
    }
}
