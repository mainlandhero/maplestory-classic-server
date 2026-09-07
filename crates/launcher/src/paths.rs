//! Where everything is, worked out from where the executable is.
//!
//! **Nothing in this crate may hard-code a path on the owner's machine.** The launcher is meant to
//! go into an installer and run on a test machine that has no repo, no `client-patched\` and
//! no `target\`. A path that is right here and wrong there fails *silently* - the client
//! simply never starts, or worse, starts against the wrong database - so the resolved layout
//! is carried in [`Layout`] and shown in the UI, wrong guess and all.
//!
//! Three sources, in this order:
//!
//! 1. **`maplecw-launcher.toml` beside the executable.** Applied per key, so a config that
//!    only sets `server_ip` still gets its paths from 2 or 3.
//! 2. **The installed layout** - `client\` beside the exe. That is the whole test: the
//!    launcher carries its own GameGuard stub and signs in over the network, so neither a
//!    `grap64.dll` nor a database has to be there.
//!    This is what the installer lays down.
//! 3. **The dev layout** - walk up from the exe (which lives in `target\debug\` or
//!    `target\release\`) until a directory containing `client-patched\MapleStory.exe` turns
//!    up. That directory is the repo root, and the stub is
//!    `target\<profile>\grap64.dll`, which is where `cargo build -p grap-stub` puts it.
//!
//! If none of them matches, the layout is still *filled in* with the installed shape and
//! marked [`Source::Fallback`], because a launcher that shows four wrong paths and says so is
//! more useful than one that shows an error and nothing else.

use std::path::{Path, PathBuf};

use crate::config::{self, LauncherConfig};

/// Where the client's own executable lives inside the client directory.
pub const CLIENT_EXE_NAME: &str = "MapleStory.exe";
/// The directory name the installer uses beside the launcher.
pub const INSTALLED_CLIENT_DIR: &str = "client";
/// The directory name a repo checkout uses. `client-patched\` exists so the original client
/// is never touched - a standing constraint, see `CLAUDE.md`.
pub const DEV_CLIENT_DIR: &str = "client-patched";
pub const STUB_FILE_NAME: &str = "grap64.dll";

/// **The dev-layout stub is always the `release` one, whatever profile the launcher itself
/// was built in.**
///
/// This is not a default; it is the repo's convention and getting it wrong is expensive.
/// `tools/test-server.ps1` runs `cargo build --release -p grap-stub` and then
/// `tools/setup-client.ps1`, whose `-StubPath` default is `target\release\grap64.dll`. So
/// `target\release\grap64.dll` is the hook every other tool in this repo installs.
///
/// Deriving the profile from the launcher's own directory looks tidier and is a trap: on
/// 2026-08-28 this checkout held a `target\debug\grap64.dll` from six days earlier beside a
/// current `target\release\grap64.dll`, so `cargo run -p launcher` would have installed the
/// stale hook - and a stale hook does not fail, it answers. `CLAUDE.md`'s note about
/// rebuilding grap-stub not updating the client is the same failure one directory over.
///
/// If the release stub is missing, [`Layout::problems`] says so and names the build command.
/// It does **not** fall back to the debug one: silently installing a different binary from
/// the one every other tool installs is the failure this constant exists to prevent.
pub const STUB_PROFILE: &str = "release";

pub const DEFAULT_SERVER_IP: &str = "127.0.0.1";
/// `tools/test-server.ps1`'s `-Port` default. The GAME port, handed to the client.
pub const DEFAULT_PORT: u16 = 8484;

/// Where `crates/auth` listens. `auth::DEFAULT_PORT`.
///
/// A second number rather than a derived one, because the two services are separate
/// processes: a machine can reach the game port and not the sign-in port, or the reverse, and
/// folding them together would make "wrong port" and "server down" the same report.
pub const DEFAULT_AUTH_PORT: u16 = 8080;

/// How far up from the executable the dev-layout walk goes. `target\debug\` is two, and a
/// few more cover `target\<triple>\debug\` and an examples subdirectory.
const MAX_WALK_UP: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// `client\` was found beside the executable.
    Installed,
    /// A repo root with `client-patched\MapleStory.exe` was found above the executable.
    Dev,
    /// Neither. The paths below are guesses, shaped like an install.
    Fallback,
}

impl Source {
    pub fn label(self) -> &'static str {
        match self {
            Source::Installed => "installed layout (client\\ beside the launcher)",
            Source::Dev => "dev layout (repo root found above the launcher)",
            Source::Fallback => "NOT FOUND - guessing the installed layout",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    /// The directory holding `maplecw-launcher.exe`.
    pub exe_dir: PathBuf,
    /// Which of the three sources supplied the base paths.
    pub source: Source,
    /// The config file, if one was read.
    pub config_file: Option<PathBuf>,
    /// Keys the config file overrode, for the UI.
    pub config_applied: Vec<String>,
    /// Complaints from the config parser.
    pub config_problems: Vec<String>,

    pub client_dir: PathBuf,
    /// Where `client_dir` came from, in words: the layout, the config file, or the folder the
    /// player chose last time (`crate::remembered`). Shown beside the path, because a
    /// remembered choice that has gone stale looks exactly like a wrong guess until the line
    /// says which it is.
    pub client_dir_from: String,
    /// The GameGuard stub to install, when one is on disk. A launcher that carries its own
    /// (the normal case) never reads it - see `crate::stub`.
    pub stub_path: PathBuf,
    /// Where run output goes: `previous-runs\` and `dumps\` are created under this. The repo
    /// root in a dev layout, so archived hook logs land beside the archived `world.log`s
    /// exactly as `tools/test-server.ps1` puts them.
    pub data_root: PathBuf,

    pub server_ip: String,
    pub port: u16,
    /// The account name to pre-fill in the sign-in box, from a successful Start Game or from
    /// `identity` in the config file. **Never a password**; see `crate::remembered`.
    pub identity: Option<String>,
    /// Write the outbound block at Start Game. **Defaults to true** and only `firewall = "off"`
    /// in the config file turns it off: forgetting a key must not be a way to lose the rule
    /// that keeps a modified client off the internet. See `crate::firewall`.
    pub firewall: bool,
    /// Where the sign-in service listens - `crates/auth`, default 8080.
    ///
    /// Separate from `port`, which is the GAME port the client is handed. They are two
    /// different services and a machine can legitimately reach one and not the other; folding
    /// them into one number would make "wrong port" and "server down" the same report.
    pub auth_port: u16,
    /// The sign-in service's certificate fingerprint, and the only certificate the launcher
    /// will speak to. `None` means sign-in is REFUSED, not attempted in the clear - the
    /// password does not leave this machine until the operator has said which server it is
    /// for. From `auth_fingerprint` in the config file, else `auth-cert-fingerprint.txt`
    /// beside the launcher, else the same file at the data root (the repo root in a dev
    /// checkout, which is where the dev sign-in service writes it). `crates/tlspin`.
    pub auth_fingerprint: Option<tlspin::Fingerprint>,
    /// Where the pin came from, for the report - or why there is none.
    pub auth_fingerprint_from: String,
}

impl Layout {
    pub fn client_exe(&self) -> PathBuf {
        self.client_dir.join(CLIENT_EXE_NAME)
    }

    /// The dev checkout this launcher is running out of, if it is running out of one.
    ///
    /// Used to name the exact command in an error rather than describing it - "run
    /// `tools\test-server.ps1`" is much less useful than the full quoted path, because an
    /// elevated window opens in `system32` and a relative path there is a command that fails.
    ///
    /// **It checks that the script is actually there** rather than trusting
    /// [`Source::Dev`]. A config file can point a dev-layout launcher at an installed client,
    /// and a message naming a `tools\` directory that does not exist is worse than one that
    /// says nothing: it sends someone to a path to find out it is wrong.
    pub fn repo_root(&self) -> Option<PathBuf> {
        if self.source != Source::Dev {
            return None;
        }
        let root = self.data_root.clone();
        root.join("tools").join("test-server.ps1").is_file().then_some(root)
    }

    pub fn previous_runs_dir(&self) -> PathBuf {
        self.data_root.join("previous-runs")
    }

    pub fn dumps_dir(&self) -> PathBuf {
        self.data_root.join("dumps")
    }

    /// A plain-text dump of everything that was resolved, for `--print-paths`.
    ///
    /// Worth having as more than a convenience: it is the only way to check path resolution
    /// on a machine without opening a window, which is exactly the check an installer on a
    /// test machine needs and exactly the check a GUI cannot be scripted into giving.
    pub fn report(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("source    {}\n", self.source.label()));
        out.push_str(&format!("launcher  {}\n", self.exe_dir.display()));
        match &self.config_file {
            Some(p) => {
                out.push_str(&format!("config    {}\n", p.display()));
                let applied = if self.config_applied.is_empty() {
                    "(nothing)".to_string()
                } else {
                    self.config_applied.join(", ")
                };
                out.push_str(&format!("  set     {applied}\n"));
            }
            None => out.push_str(&format!(
                "config    none ({} beside the launcher would be read)\n",
                crate::config::CONFIG_FILE_NAME
            )),
        }
        out.push_str(&format!("client    {}\n", self.client_dir.display()));
        out.push_str(&format!("  from    {}\n", self.client_dir_from));
        out.push_str(&format!("exe       {}\n", self.client_exe().display()));
        out.push_str(&format!("stub      {}\n", self.stub_path.display()));
        out.push_str(&format!("output    {}\n", self.data_root.display()));
        out.push_str(&format!("archives  {}\n", self.previous_runs_dir().display()));
        out.push_str(&format!("dumps     {}\n", self.dumps_dir().display()));
        out.push_str(&format!("game      {}:{}\n", self.server_ip, self.port));
        out.push_str(&format!("sign-in   {}:{} (TLS)\n", self.server_ip, self.auth_port));
        match &self.auth_fingerprint {
            Some(fp) => out.push_str(&format!("pin       {fp}\n  from    {}\n", self.auth_fingerprint_from)),
            None => out.push_str(&format!("pin       {}\n", self.auth_fingerprint_from)),
        }
        for problem in &self.config_problems {
            out.push_str(&format!("CONFIG    {problem}\n"));
        }
        let problems = self.problems();
        if problems.is_empty() {
            out.push_str("MISSING   nothing\n");
        } else {
            for problem in problems {
                out.push_str(&format!("MISSING   {problem}\n"));
            }
        }
        out
    }

    /// Everything that is missing *right now*, in the words the UI should use.
    ///
    /// Deliberately computed on demand rather than at resolve time: the stub is built by
    /// `cargo build -p grap-stub` and the database by `maplecw-useradd`, so both can appear
    /// while the launcher is open, and a stale "missing" would send the owner hunting a problem
    /// they have already fixed.
    pub fn problems(&self) -> Vec<String> {
        let mut out = Vec::new();
        if !self.client_exe().is_file() {
            out.push(format!("no {} at {}", CLIENT_EXE_NAME, self.client_exe().display()));
        }
        // **A MISSING DATABASE IS NOT A PROBLEM.** The owner, 2026-08-29: *"everyone installs this
        // differently, on a client machine you won't have access to the project or the
        // database."* Sign-in goes to the server's auth service over the network now, so a
        // client machine legitimately has no `maplecw.db` at all - and reporting one as a
        // fault would send somebody hunting for a file that is not meant to be there.
        //
        // The path is still resolved and still shown, because a machine that DOES have one
        // (the dev box) benefits from seeing which, and because `--print-paths` is the only
        // way to check resolution without a window.
        if !self.stub_path.is_file() && crate::stub::EMBEDDED.is_none() {
            // Only when there is no built-in copy either. A launcher that carries its own
            // stub does not need one on disk, which is the whole point of embedding it - and
            // complaining about a file it will never read would be a false alarm.
            out.push(format!(
                "no GameGuard stub at {} and none compiled in - build it with `cargo build \
                 --release -p grap-stub`",
                self.stub_path.display()
            ));
        }
        out
    }
}

/// Resolve using the real executable's directory.
///
/// Falls back to the current directory if `current_exe()` fails, which it does not on
/// Windows, but a launcher that panics before drawing a window explains nothing.
pub fn resolve() -> Layout {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    resolve_from(&exe_dir)
}

/// The whole of the resolution, driven by a directory rather than by the process, so the
/// precedence order is testable with temp dirs.
pub fn resolve_from(exe_dir: &Path) -> Layout {
    let (base_source, client_dir, stub_path, data_root) = base_layout(exe_dir);

    let mut layout = Layout {
        exe_dir: exe_dir.to_path_buf(),
        source: base_source,
        config_file: None,
        config_applied: Vec::new(),
        config_problems: Vec::new(),
        client_dir,
        client_dir_from: format!("the {}", base_source.label()),
        stub_path,
        data_root,
        server_ip: DEFAULT_SERVER_IP.to_string(),
        port: DEFAULT_PORT,
        identity: None,
        firewall: true,
        auth_port: DEFAULT_AUTH_PORT,
        auth_fingerprint: None,
        auth_fingerprint_from: String::new(),
    };

    let mut pinned_in_config: Option<String> = None;
    if let Some((path, cfg)) = config::load(exe_dir) {
        layout.config_file = Some(path);
        layout.config_problems = cfg.problems.clone();
        // A config file that is present and sets nothing is worth saying out loud: it is the
        // shape of a typo'd key or a file saved in the wrong place, and its symptom is
        // "the launcher ignored my settings" with nothing anywhere to explain it.
        if cfg.is_empty() && cfg.problems.is_empty() {
            layout.config_problems.push(format!(
                "{} is present but sets nothing",
                config::CONFIG_FILE_NAME
            ));
        }
        pinned_in_config = cfg.auth_fingerprint.clone();
        apply_config(&mut layout, &cfg, exe_dir);
    }
    resolve_fingerprint(&mut layout, pinned_in_config.as_deref(), exe_dir);
    apply_remembered(&mut layout, exe_dir);

    layout
}

/// Overlay the folder the player chose last time, on top of everything else.
///
/// It wins over the config file on purpose: the config's `client_dir` is what the installer
/// guessed, and a Browse is a person correcting that guess. Correcting it once and having the
/// next start undo it is the complaint that made this exist (the owner, 2026-09-05: *"setting it
/// every time is going to be very frustrating for users"*).
///
/// A remembered folder that no longer holds `MapleStory.exe` is **kept, and flagged** by
/// [`Layout::problems`] like any other missing client, rather than silently dropped for the
/// default. Dropping it would launch a different client from the one the player chose with
/// nothing on screen to say so; keeping it puts the stale path in the box, one Browse from
/// fixed, with a line naming the file it came from.
fn apply_remembered(layout: &mut Layout, exe_dir: &Path) {
    let Some((path, remembered)) = crate::remembered::load(exe_dir) else {
        return;
    };
    for problem in &remembered.problems {
        layout
            .config_problems
            .push(format!("{}: {problem}", crate::remembered::FILE_NAME));
    }
    let mut used_something = false;
    if let Some(v) = &remembered.client_dir {
        layout.client_dir = absolutise(exe_dir, v);
        layout.client_dir_from =
            format!("the folder chosen last time, remembered in {}", path.display());
        used_something = true;
    }
    // **The rest of what a successful Start Game saved.** These outrank the config file for
    // the same reason `client_dir` does: the file is written only after a launch that worked,
    // so it records settings that are known to reach a server, and re-running `install.ps1`
    // is a deliberate reconfiguration that removes it.
    if let Some(v) = &remembered.server_ip {
        layout.server_ip = v.clone();
        used_something = true;
    }
    if let Some(v) = remembered.port {
        layout.port = v;
        used_something = true;
    }
    if let Some(v) = &remembered.identity {
        layout.identity = Some(v.clone());
        used_something = true;
    }
    if used_something {
        layout
            .config_applied
            .push(format!("remembered from the last successful Start Game ({})", path.display()));
    } else if remembered.problems.is_empty() {
        layout.config_problems.push(format!(
            "{} is present but remembers nothing",
            crate::remembered::FILE_NAME
        ));
    }
}

/// Decide which certificate the sign-in service must present, or record that none is known.
///
/// The config key wins, because it is the one an operator wrote on purpose. A file is second:
/// the sign-in service writes `auth-cert-fingerprint.txt` beside its database, so in a dev
/// checkout the launcher finds the dev service's pin at the repo root with nobody copying
/// anything, and an installer can drop the same file beside an installed launcher. A value
/// that is present and does not parse is a problem line, never silently "no pin" - that
/// would turn a typo into a refused sign-in with nothing anywhere to explain it.
fn resolve_fingerprint(layout: &mut Layout, in_config: Option<&str>, exe_dir: &Path) {
    if let Some(text) = in_config {
        match tlspin::Fingerprint::parse(text) {
            Ok(fp) => {
                layout.auth_fingerprint = Some(fp);
                layout.auth_fingerprint_from = "auth_fingerprint in the config file".into();
                layout.config_applied.push("auth_fingerprint".into());
                return;
            }
            Err(e) => layout.config_problems.push(format!("auth_fingerprint: {e}")),
        }
    }
    for candidate in [
        exe_dir.join(tlspin::FINGERPRINT_FILE),
        layout.data_root.join(tlspin::FINGERPRINT_FILE),
    ] {
        match tlspin::read_fingerprint_file(&candidate) {
            None => continue,
            Some(Ok(fp)) => {
                layout.auth_fingerprint = Some(fp);
                layout.auth_fingerprint_from = candidate.display().to_string();
                return;
            }
            Some(Err(e)) => layout.config_problems.push(e),
        }
    }
    // **Last, and only if nothing on disk answered: the pin compiled into this build.**
    //
    // `crates/launcher/build.rs` reads `auth-cert-fingerprint.txt` from the repo root at build
    // time, so an ordinary deployment needs no certificate configuration on the client at all.
    // It comes last on purpose - every on-disk source outranks it, so pointing a launcher at a
    // different server stays one config key away and needs no rebuild.
    //
    // A build with no fingerprint file bakes nothing and falls through to NOT PINNED below,
    // which is the behaviour this had before. There is no branch here that accepts an
    // unknown certificate.
    if let Some(baked) = baked_fingerprint() {
        match tlspin::Fingerprint::parse(baked) {
            Ok(fp) => {
                layout.auth_fingerprint = Some(fp);
                layout.auth_fingerprint_from =
                    "compiled into this launcher at build time (no client configuration needed; \
                     override with auth_fingerprint in the config file)"
                        .into();
                return;
            }
            // Unreachable unless build.rs's shape check and tlspin disagree, which is worth
            // saying out loud rather than falling through as though nothing were baked.
            Err(e) => layout
                .config_problems
                .push(format!("the fingerprint compiled into this launcher does not parse: {e}")),
        }
    }

    layout.auth_fingerprint_from = format!(
        "NOT PINNED - sign-in will be refused. This build has none compiled in either. Set auth_fingerprint in {} or put {} beside the launcher; the sign-in service prints the value at startup",
        crate::config::CONFIG_FILE_NAME,
        tlspin::FINGERPRINT_FILE
    );
}

/// The fingerprint `build.rs` compiled in, if it found one.
///
/// Separate so the precedence tests can read it, and so the `cfg` appears exactly once.
pub fn baked_fingerprint() -> Option<&'static str> {
    #[cfg(has_baked_fingerprint)]
    {
        Some(env!("MAPLECW_BAKED_FINGERPRINT"))
    }
    #[cfg(not(has_baked_fingerprint))]
    {
        None
    }
}

/// Overlay a config file onto an already-resolved layout. Separate so the precedence test
/// can read as "these keys won, the rest fell through".
fn apply_config(layout: &mut Layout, cfg: &LauncherConfig, exe_dir: &Path) {
    if let Some(v) = &cfg.client_dir {
        layout.client_dir = absolutise(exe_dir, v);
        layout.client_dir_from = format!("client_dir in {}", config::CONFIG_FILE_NAME);
        layout.config_applied.push("client_dir".into());
    }
    if let Some(v) = &cfg.stub_path {
        layout.stub_path = absolutise(exe_dir, v);
        layout.config_applied.push("stub_path".into());
    }
    if let Some(v) = &cfg.server_ip {
        layout.server_ip = v.clone();
        layout.config_applied.push("server_ip".into());
    }
    if let Some(v) = cfg.auth_port {
        layout.auth_port = v;
        layout.config_applied.push("auth_port".into());
    }
    if let Some(v) = cfg.port {
        layout.port = v;
        layout.config_applied.push("port".into());
    }
    if let Some(v) = &cfg.identity {
        layout.identity = Some(v.clone());
        layout.config_applied.push("identity".into());
    }
    if let Some(v) = cfg.firewall {
        layout.firewall = v;
        layout.config_applied.push(format!("firewall = {}", if v { "on" } else { "off" }));
    }
}

fn base_layout(exe_dir: &Path) -> (Source, PathBuf, PathBuf, PathBuf) {
    if let Some(client_dir) = installed_client_dir(exe_dir) {
        return (
            Source::Installed,
            client_dir,
            exe_dir.join(STUB_FILE_NAME),
            exe_dir.to_path_buf(),
        );
    }
    if let Some(root) = repo_root_above(exe_dir) {
        return (
            Source::Dev,
            root.join(DEV_CLIENT_DIR),
            root.join("target").join(STUB_PROFILE).join(STUB_FILE_NAME),
            root,
        );
    }
    (
        Source::Fallback,
        exe_dir.join(INSTALLED_CLIENT_DIR),
        exe_dir.join(STUB_FILE_NAME),
        exe_dir.to_path_buf(),
    )
}

/// The installed layout is recognised by `client\` beside the executable.
///
/// Not by `client\` AND `grap64.dll` both being present: the launcher carries its own stub,
/// so requiring one on disk would demote a real install to [`Source::Fallback`] and hide the
/// reason. [`Layout::problems`] reports a genuinely missing piece instead, which is the same
/// information without the wrong verdict attached.
fn installed_client_dir(exe_dir: &Path) -> Option<PathBuf> {
    let dir = exe_dir.join(INSTALLED_CLIENT_DIR);
    if dir.is_dir() {
        Some(dir)
    } else {
        None
    }
}

/// Walk up looking for a directory holding `client-patched\MapleStory.exe`.
fn repo_root_above(exe_dir: &Path) -> Option<PathBuf> {
    let mut here = Some(exe_dir);
    for _ in 0..=MAX_WALK_UP {
        let dir = here?;
        if dir.join(DEV_CLIENT_DIR).join(CLIENT_EXE_NAME).is_file() {
            return Some(dir.to_path_buf());
        }
        here = dir.parent();
    }
    None
}

/// A relative config value is relative to the executable, not to the working directory.
///
/// The working directory of a launcher started from a Start Menu shortcut is not something
/// anybody can predict, and `System32` is a common answer - the same trap `CLAUDE.md`
/// describes for elevated PowerShell windows.
pub fn absolutise(exe_dir: &Path, value: &str) -> PathBuf {
    let cleaned = collapse_separators(value);
    let p = Path::new(&cleaned);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        exe_dir.join(p)
    }
}

/// Collapse runs of `\` that Windows would collapse anyway, keeping a leading `\\` so a UNC
/// path survives.
///
/// **Why this is here rather than left alone.** `tools/installer/install.ps1` writes the
/// config with `$path -replace '\\', '\\'`, which in .NET replacement syntax is not an escape
/// but two literal backslashes - measured, `C:\MapleCW\client` comes out as
/// `C:\\MapleCW\\client`. [`crate::config`] is deliberately literal, so it keeps both.
///
/// That still *works*: Win32 collapses interior duplicate separators, and an installed
/// launcher fed that config resolved every path and reported nothing missing. The reason to
/// normalise anyway is the UI's job - the paths are shown so that a **wrong** one is visible,
/// and `C:\\MapleCW\\client` looks broken while being fine, which makes a genuinely broken
/// path harder to pick out of the same list. Cheap, and it costs the config file nothing.
fn collapse_separators(value: &str) -> String {
    let (prefix, rest) = match value.strip_prefix(r"\\") {
        Some(rest) => (r"\\", rest),
        None => ("", value),
    };
    let mut out = String::with_capacity(value.len());
    out.push_str(prefix);
    let mut last_was_sep = false;
    for ch in rest.chars() {
        let is_sep = ch == '\\';
        if is_sep && last_was_sep {
            continue;
        }
        out.push(ch);
        last_was_sep = is_sep;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::TempDir;

    /// `<tmp>/app/` with `client/MapleStory.exe`, `maplecw.db`, `grap64.dll` - what the
    /// installer lays down.
    fn make_installed(t: &TempDir) -> PathBuf {
        t.file("app/client/MapleStory.exe", "client");
        t.file("app/maplecw.db", "db");
        t.file("app/grap64.dll", "stub");
        t.path().join("app")
    }

    /// `<tmp>/repo/` with `client-patched/MapleStory.exe`, and an exe dir at
    /// `repo/target/debug/`.
    fn make_dev(t: &TempDir) -> (PathBuf, PathBuf) {
        t.file("repo/client-patched/MapleStory.exe", "client");
        t.file("repo/maplecw.db", "db");
        // The stub is the RELEASE one even though the launcher below is a debug build.
        t.file("repo/target/release/grap64.dll", "stub");
        let exe_dir = t.dir("repo/target/debug");
        (t.path().join("repo"), exe_dir)
    }

    #[test]
    fn installed_layout_is_found_beside_the_exe() {
        let t = TempDir::new("installed");
        let exe_dir = make_installed(&t);
        let l = resolve_from(&exe_dir);
        assert_eq!(l.source, Source::Installed);
        assert_eq!(l.client_dir, exe_dir.join("client"));
        assert_eq!(l.stub_path, exe_dir.join("grap64.dll"));
        assert_eq!(l.data_root, exe_dir);
        assert!(l.problems().is_empty(), "{:?}", l.problems());
    }

    #[test]
    fn dev_layout_walks_up_to_the_repo_root() {
        let t = TempDir::new("dev");
        let (root, exe_dir) = make_dev(&t);
        let l = resolve_from(&exe_dir);
        assert_eq!(l.source, Source::Dev);
        assert_eq!(l.client_dir, root.join("client-patched"));
        assert_eq!(l.data_root, root);
        assert!(l.problems().is_empty(), "{:?}", l.problems());
    }

    #[test]
    fn a_debug_launcher_still_installs_the_release_stub() {
        // The trap this exists to prevent, seen for real on 2026-08-28: this checkout held a
        // `target\debug\grap64.dll` six days older than `target\release\grap64.dll`. Every
        // other tool in the repo installs the release one; a launcher that installed the
        // debug one because it happened to be a debug build itself would put a stale hook in
        // the client, and a stale hook does not fail - it answers.
        let t = TempDir::new("devstale");
        t.file("repo/client-patched/MapleStory.exe", "client");
        t.file("repo/target/debug/grap64.dll", "STALE debug stub");
        t.file("repo/target/release/grap64.dll", "the current stub");
        let exe_dir = t.dir("repo/target/debug");

        let l = resolve_from(&exe_dir);
        assert_eq!(l.source, Source::Dev);
        assert_eq!(
            l.stub_path,
            t.path().join("repo").join("target").join("release").join("grap64.dll")
        );
        assert_eq!(std::fs::read_to_string(&l.stub_path).unwrap(), "the current stub");
    }

    /// The stale-debug-stub trap, still. `STUB_PROFILE` is release unconditionally, so a
    /// current `target\debug\grap64.dll` must NOT be picked up in a dev layout.
    ///
    /// **What changed on 2026-08-29:** a missing release stub is only a *problem* when this
    /// launcher has none compiled in either. A build that carries its own does not need a
    /// file it will never read, and complaining about one would be a false alarm - see
    /// `crate::stub`. The path it resolves is unchanged and is still asserted.
    #[test]
    fn a_missing_release_stub_is_reported_and_not_replaced_by_the_debug_one() {
        let t = TempDir::new("devnorelease");
        t.file("repo/client-patched/MapleStory.exe", "client");
        t.file("repo/maplecw.db", "db");
        t.file("repo/target/debug/grap64.dll", "STALE debug stub");
        let exe_dir = t.dir("repo/target/debug");

        let l = resolve_from(&exe_dir);
        assert!(
            l.stub_path.ends_with("release/grap64.dll")
                || l.stub_path.ends_with("release\\grap64.dll"),
            "the debug stub must never be chosen: {}",
            l.stub_path.display()
        );

        let problems = l.problems();
        match crate::stub::EMBEDDED {
            // Nothing to complain about: the launcher carries its own.
            Some(_) => assert!(problems.is_empty(), "{problems:?}"),
            None => {
                assert_eq!(problems.len(), 1, "{problems:?}");
                assert!(problems[0].contains("cargo build"), "{problems:?}");
            }
        }
    }

    #[test]
    fn installed_beats_dev_when_both_are_present() {
        // The precedence that matters on the owner's own machine if they ever unpacks an installer
        // inside the checkout: the thing beside the exe wins.
        let t = TempDir::new("both");
        t.file("repo/client-patched/MapleStory.exe", "client");
        t.file("repo/target/debug/client/MapleStory.exe", "installed client");
        let exe_dir = t.dir("repo/target/debug");
        let l = resolve_from(&exe_dir);
        assert_eq!(l.source, Source::Installed);
        assert_eq!(l.client_dir, exe_dir.join("client"));
    }

    #[test]
    fn config_beats_installed_and_dev_key_by_key() {
        let t = TempDir::new("cfgwins");
        let exe_dir = make_installed(&t);
        let elsewhere = t.dir("elsewhere");
        t.file("elsewhere/MapleStory.exe", "other client");
        std::fs::write(
            exe_dir.join(config::CONFIG_FILE_NAME),
            format!(
                "client_dir = \"{}\"\nserver_ip = 192.168.1.20\nport = 9999\n",
                elsewhere.display()
            ),
        )
        .unwrap();

        let l = resolve_from(&exe_dir);
        // The base is still the installed layout...
        assert_eq!(l.source, Source::Installed);
        // ...but the three keys the config named win.
        assert_eq!(l.client_dir, elsewhere);
        assert_eq!(l.server_ip, "192.168.1.20");
        assert_eq!(l.port, 9999);
        // ...and everything it did not name fell through to the installed layout.
        assert_eq!(l.stub_path, exe_dir.join("grap64.dll"));
        assert_eq!(
            l.config_applied,
            vec!["client_dir".to_string(), "server_ip".into(), "port".into()]
        );
        assert!(l.config_file.is_some());
    }

    /// The complaint that made `crate::remembered` exist: a chosen folder has to survive a
    /// restart, and it has to beat the installer's guess in the config file.
    #[test]
    fn a_remembered_folder_beats_the_config_and_the_layout() {
        let t = TempDir::new("remembered-wins");
        let exe_dir = make_installed(&t);
        let configured = t.dir("configured");
        t.file("configured/MapleStory.exe", "configured client");
        std::fs::write(
            exe_dir.join(config::CONFIG_FILE_NAME),
            format!("client_dir = \"{}\"\nserver_ip = 192.168.1.20\n", configured.display()),
        )
        .unwrap();
        let chosen = t.dir("chosen");
        t.file("chosen/MapleStory.exe", "the one the player picked");
        crate::remembered::save_client_dir(&exe_dir, &chosen).unwrap();

        let l = resolve_from(&exe_dir);
        assert_eq!(l.client_dir, chosen);
        assert!(l.client_dir_from.contains("remembered"), "{}", l.client_dir_from);
        assert!(
            l.client_dir_from.contains(crate::remembered::FILE_NAME),
            "the line must name the file, or nobody can find it to delete it: {}",
            l.client_dir_from
        );
        // Only the client folder is remembered. The server address still comes from the
        // config, and the config is still reported as having set client_dir - it did; it was
        // then overridden, and the report says by what.
        assert_eq!(l.server_ip, "192.168.1.20");
        assert!(l.problems().is_empty(), "{:?}", l.problems());
        assert!(l.report().contains("remembered"), "{}", l.report());
    }

    #[test]
    fn without_a_remembered_file_the_config_still_wins_and_says_so() {
        let t = TempDir::new("remembered-absent");
        let exe_dir = make_installed(&t);
        let configured = t.dir("configured");
        t.file("configured/MapleStory.exe", "configured client");
        std::fs::write(
            exe_dir.join(config::CONFIG_FILE_NAME),
            format!("client_dir = \"{}\"\n", configured.display()),
        )
        .unwrap();
        let l = resolve_from(&exe_dir);
        assert_eq!(l.client_dir, configured);
        assert!(l.client_dir_from.contains(config::CONFIG_FILE_NAME), "{}", l.client_dir_from);
    }

    #[test]
    fn with_neither_the_layout_is_named_as_the_source() {
        let t = TempDir::new("remembered-neither");
        let exe_dir = make_installed(&t);
        let l = resolve_from(&exe_dir);
        assert_eq!(l.client_dir, exe_dir.join("client"));
        assert!(l.client_dir_from.contains("installed layout"), "{}", l.client_dir_from);
    }

    /// The player moved the game after choosing it. The stale choice stays in the box and is
    /// flagged, rather than being silently swapped for the default - see `apply_remembered`.
    #[test]
    fn a_stale_remembered_folder_is_kept_and_flagged_not_dropped() {
        let t = TempDir::new("remembered-stale");
        let exe_dir = make_installed(&t);
        let gone = t.path().join("moved-away");
        crate::remembered::save_client_dir(&exe_dir, &gone).unwrap();
        let l = resolve_from(&exe_dir);
        assert_eq!(l.client_dir, gone);
        let problems = l.problems();
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("moved-away"), "{problems:?}");
    }

    #[test]
    fn a_remembered_file_that_remembers_nothing_is_reported() {
        let t = TempDir::new("remembered-empty");
        let exe_dir = make_installed(&t);
        std::fs::write(exe_dir.join(crate::remembered::FILE_NAME), "# nothing here\n").unwrap();
        let l = resolve_from(&exe_dir);
        assert_eq!(l.client_dir, exe_dir.join("client"));
        assert!(
            l.config_problems.iter().any(|p| p.contains("remembers nothing")),
            "{:?}",
            l.config_problems
        );
    }

    #[test]
    fn config_relative_paths_resolve_against_the_exe_not_the_cwd() {
        let t = TempDir::new("cfgrel");
        let exe_dir = make_installed(&t);
        t.file("app/other/MapleStory.exe", "other");
        std::fs::write(
            exe_dir.join(config::CONFIG_FILE_NAME),
            "client_dir = other\ndb_path = data\\live.db\n",
        )
        .unwrap();
        let l = resolve_from(&exe_dir);
        assert_eq!(l.client_dir, exe_dir.join("other"));
    }

    #[test]
    fn a_config_with_only_a_server_ip_leaves_the_paths_alone() {
        let t = TempDir::new("cfgpartial");
        let (root, exe_dir) = make_dev(&t);
        std::fs::write(exe_dir.join(config::CONFIG_FILE_NAME), "server_ip = 10.1.2.3\n").unwrap();
        let l = resolve_from(&exe_dir);
        assert_eq!(l.source, Source::Dev);
        assert_eq!(l.client_dir, root.join("client-patched"));
        assert_eq!(l.server_ip, "10.1.2.3");
        assert_eq!(l.port, DEFAULT_PORT);
    }

    #[test]
    fn config_problems_reach_the_layout() {
        let t = TempDir::new("cfgbad");
        let exe_dir = make_installed(&t);
        std::fs::write(
            exe_dir.join(config::CONFIG_FILE_NAME),
            "port = not-a-number\nnonsense = 1\n",
        )
        .unwrap();
        let l = resolve_from(&exe_dir);
        assert_eq!(l.port, DEFAULT_PORT);
        assert_eq!(l.config_problems.len(), 2, "{:?}", l.config_problems);
    }

    #[test]
    fn the_installers_doubled_backslashes_resolve_to_the_same_place() {
        // `tools/installer/install.ps1` writes `$p -replace '\\', '\\'`, which doubles every
        // separator rather than escaping anything - measured against PowerShell 5.1:
        // `C:\MapleCW\client` comes out `C:\\MapleCW\\client`. The config reader is literal
        // on purpose, so the doubling has to be handled here.
        let t = TempDir::new("cfgdoubled");
        let exe_dir = make_installed(&t);
        let real = t.dir("app/elsewhere");
        t.file("app/elsewhere/MapleStory.exe", "client");
        let doubled = real.to_string_lossy().replace('\\', "\\\\");
        std::fs::write(
            exe_dir.join(config::CONFIG_FILE_NAME),
            format!("client_dir = \"{doubled}\"\n"),
        )
        .unwrap();

        let l = resolve_from(&exe_dir);
        assert_eq!(l.client_dir, real, "doubled separators changed the destination");
        assert!(
            !l.client_dir.to_string_lossy().contains("\\\\"),
            "the shown path still has doubled separators: {}",
            l.client_dir.display()
        );
        assert!(l.client_exe().is_file());
    }

    #[test]
    fn a_unc_path_keeps_its_leading_double_backslash() {
        assert_eq!(collapse_separators(r"\\server\share\dir"), r"\\server\share\dir");
        assert_eq!(collapse_separators(r"\\server\\share"), r"\\server\share");
        assert_eq!(collapse_separators(r"C:\\a\\\b"), r"C:\a\b");
        assert_eq!(collapse_separators("relative/path"), "relative/path");
    }

    #[test]
    fn an_empty_config_file_is_reported_rather_than_silently_ignored() {
        let t = TempDir::new("cfgempty");
        let exe_dir = make_installed(&t);
        std::fs::write(exe_dir.join(config::CONFIG_FILE_NAME), "# nothing here\n").unwrap();
        let l = resolve_from(&exe_dir);
        assert_eq!(l.config_problems.len(), 1, "{:?}", l.config_problems);
        assert!(l.config_problems[0].contains("sets nothing"), "{:?}", l.config_problems);
    }

    #[test]
    fn nothing_found_still_yields_a_usable_layout_that_says_so() {
        let t = TempDir::new("nothing");
        let exe_dir = t.dir("lonely");
        let l = resolve_from(&exe_dir);
        assert_eq!(l.source, Source::Fallback);
        assert_eq!(l.client_dir, exe_dir.join("client"));

        // The client executable is the one thing that is ALWAYS a problem when missing:
        // nothing can be launched without it. The database is not (sign-in is over the
        // network) and the stub is not when one is compiled in.
        let problems = l.problems();
        assert!(problems.iter().any(|p| p.contains("MapleStory.exe")), "{problems:?}");
        let expected = 1 + usize::from(crate::stub::EMBEDDED.is_none());
        assert_eq!(problems.len(), expected, "{problems:?}");
    }

    #[test]
    fn the_walk_up_does_not_escape_past_its_limit() {
        // A repo root far enough above the exe is not found - otherwise a launcher dropped
        // deep inside someone's Downloads could adopt an unrelated checkout.
        let t = TempDir::new("deep");
        t.file("repo/client-patched/MapleStory.exe", "client");
        let deep = t.dir("repo/a/b/c/d/e/f/g/h/i/j");
        let l = resolve_from(&deep);
        assert_eq!(l.source, Source::Fallback);
    }

    #[test]
    /// **A missing database is NOT a problem any more**, and that is the whole point of the
    /// 2026-08-29 change. The owner: *"on a client machine you won't have access to the project or
    /// the database."* Sign-in goes to the server over HTTP, so an installed client
    /// legitimately has no `maplecw.db` - and reporting one would send somebody hunting for a
    /// file that is not meant to exist.
    fn missing_pieces_are_named_individually() {
        let t = TempDir::new("partial");
        t.file("app/client/MapleStory.exe", "client");
        let exe_dir = t.path().join("app");
        let l = resolve_from(&exe_dir);
        assert_eq!(l.source, Source::Installed);

        let problems = l.problems();
        assert!(
            !problems.iter().any(|p| p.contains("database")),
            "a client machine has no database and that is correct: {problems:?}"
        );
        // The stub is still named, but only when there is no built-in copy to fall back on.
        match crate::stub::EMBEDDED {
            Some(_) => assert!(problems.is_empty(), "{problems:?}"),
            None => assert!(problems.iter().any(|p| p.contains("stub")), "{problems:?}"),
        }
    }
}
