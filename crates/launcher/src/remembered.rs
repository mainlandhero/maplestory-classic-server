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
    for (key, present) in [
        ("stub_path", cfg.stub_path.is_some()),
        ("server_ip", cfg.server_ip.is_some()),
        ("port", cfg.port.is_some()),
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
        problems,
    }
}

/// The text that will be written for `client_dir`. Separate from [`save_client_dir`] so the
/// round trip is testable without a disk.
pub fn render_client_dir(client_dir: &Path) -> String {
    format!(
        "# Written by maplecw-launcher when you chose a game folder. It is read on the next\n\
         # start, ahead of {}. Delete this file to go back to the default.\n\
         # Paths are literal - single backslashes, no escaping.\n\
         client_dir = \"{}\"\n",
        config::CONFIG_FILE_NAME,
        client_dir.display()
    )
}

/// Remember `client_dir` in `dir`. Returns the file written.
///
/// Written to a sibling and renamed into place, so a crash mid-write leaves the previous
/// choice rather than half a line - a half line would parse as "no `=`" and read on the next
/// start as the launcher having forgotten, which is the failure this module exists to remove.
pub fn save_client_dir(dir: &Path, client_dir: &Path) -> io::Result<PathBuf> {
    let path = dir.join(FILE_NAME);
    let tmp = dir.join(format!("{FILE_NAME}.tmp"));
    std::fs::write(&tmp, render_client_dir(client_dir))?;
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

    #[test]
    fn a_hand_edited_server_key_is_reported_and_not_obeyed() {
        let got = parse("client_dir = C:\\x\nserver_ip = 10.0.0.9\n");
        assert_eq!(got.client_dir.as_deref(), Some("C:\\x"));
        assert_eq!(got.problems.len(), 1, "{:?}", got.problems);
        assert!(got.problems[0].contains("server_ip"), "{:?}", got.problems);
        assert!(got.problems[0].contains(config::CONFIG_FILE_NAME), "{:?}", got.problems);
    }

    #[test]
    fn an_empty_or_broken_file_remembers_nothing_and_says_why() {
        assert_eq!(parse(""), Remembered::default());
        let got = parse("client_dir C:\\x");
        assert_eq!(got.client_dir, None);
        assert_eq!(got.problems.len(), 1);
    }
}
