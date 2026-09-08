//! Preparing the client directory: the GameGuard stub, the hook's marker files, and
//! archiving the previous run's hook log.
//!
//! This is a port of `tools/setup-client.ps1` and the launch block near the end of
//! `tools/test-server.ps1`. Three of the rules below are load-bearing and are each stated
//! where they are enforced rather than in a comment somewhere above the caller:
//!
//! * **The original install is never modified.** `client-patched\` exists for exactly this
//!   reason and it is a standing constraint in `CLAUDE.md`.
//! * **`grap64.dll.orig` is written once, and only if it does not already exist.** Get this
//!   wrong on the second run and the backup becomes a copy of the stub, which means the real
//!   GameGuard DLL is gone for good and no `-Restore` can bring it back.
//! * **The previous hook log is archived, not deleted.** `CLAUDE.md` has a whole section on
//!   this: a client run costs the owner a manual launch, and four conclusions have already died
//!   with a log that got overwritten at the next launch.

use std::path::{Path, PathBuf};

/// The install this launcher must never write to. `tools/setup-client.ps1` refuses the same
/// path, and for the same reason.
pub const ORIGINAL_INSTALL: &str = r"C:\Nexon\Library\maplestorycw\appdata";

pub const GRAP_DLL: &str = "grap64.dll";
pub const GRAP_BACKUP: &str = "grap64.dll.orig";
pub const GRAP_DIR: &str = "grap";
pub const GRAP_DIR_DISABLED: &str = "grap.disabled";

pub const HOOK_ENABLE_MARKER: &str = "maplecw-hook.enable";
pub const HOOK_PROBE_MARKER: &str = "maplecw-hook.probe";
pub const HOOK_SESSION_MARKER: &str = "maplecw-hook.session";
pub const HOOK_DUMPDIR_MARKER: &str = "maplecw-hook.dumpdir";

/// Lets the stub hook the client's single-instance guard. `grap_stub::instance`.
///
/// **Always written, and safe to write always**, because the marker only turns the *hooks*
/// on - the stub works out for itself whether it is the first client on this desktop and
/// suppresses nothing in that one. So a single-client run behaves exactly as it did before
/// and still produces the log that names the guard.
pub const HOOK_MULTICLIENT_MARKER: &str = "maplecw-hook.multiclient";
/// **The credential the client will carry**, in plain text, beside the client.
///
/// Read by `grap_stub::identity`, which writes it into the client's own session object so the
/// client sends it in `0x0073`. Written only when a sign-in produced a fresh token, and
/// **deleted otherwise** - see [`write_identity_marker`].
///
/// Anything running as this user can read this file. That is the same power as being this
/// launch, so on a single-user machine it does not widen anything - but it is a secret on
/// disk and it is named as one here rather than left to be discovered.
///
/// **This file got more valuable on 2026-09-08 and the sentence above got weaker.** The token
/// used to be spent on its first presentation, so a copy of this file was worth one race
/// against the real client. The owner asked for the token to be honoured for the life of the login
/// claim (`store::claims::Store::present_client_token`), so a copy is now worth being served
/// as that account on the LOGIN socket for up to twelve hours. The hook deletes the file once
/// it has read it, which is what keeps that window short in practice rather than in theory.
pub const HOOK_IDENTITY_MARKER: &str = "maplecw-hook.identity";
pub const HOOK_LOG: &str = "maplecw-hook.log";

/// `tools/test-server.ps1`'s `-Probe` default.
///
/// `1415db360:ret` and `141b2a280:rdx=0` are **not optional**: without the first the client
/// `__fastfail`s about 37 seconds in, because its own reachability check overruns a stack
/// buffer when nothing is reachable; without the second the "trouble logging in" dialog
/// blocks the per-frame tick that enables the Login button.
pub const DEFAULT_PROBE: &str = "watch@1415db360:ret,141b2a280:rdx=0,141b36f60,142ef3e44:hits=8";

/// The size classes the shipped guard page quarantines, spelled the way
/// `grap_stub::guardpage::parse_classes` reads them: `+`-joined, **never** comma-joined,
/// because the session marker is itself comma-separated and a comma here would arm half of
/// what was asked for.
///
/// **`0x20+0x40` and not `all`**, for two measured reasons:
///
/// * these are the two classes the damage has actually been seen on - a `0x20` red-black tree
///   node on the 2026-09-08 overnight run and a `0x40` map node / vtable pointer on the
///   2026-09-07 runs and the 1 h 57 m run. `0x10` and `0x80` have never been a victim.
/// * the reserve is shared across classes and sized from **one** measured class. The 12:01 run
///   measured `0x20` at a 627 172-slot first-minute burst and 1 560/s after it, so one class
///   needs ~1.47 M slots for its first retirement window. The 8 M-slot cursor is 2.8x that for
///   two classes and only **1.4x** for four - below the 1.5x floor
///   `guardpage::render_armed` shouts about, and a spent cursor means allocations fall back to
///   the client's own pool and the class silently stops being covered.
///
/// The other two classes' churn is still unmeasured; the heartbeat's `pool allocations seen by
/// class` counters are what would justify widening this, and they cost no launch of their own.
pub const SHIPPED_GUARDPAGE_CLASSES: &str = "0x20+0x40";

/// The token itself, as it appears in the session marker.
pub const GUARDPAGE_PREFIX: &str = "guardpage=";

/// `tools/test-server.ps1`'s `-Session` default, and **what every launcher launch writes**.
///
/// `mode=2` also routes `0x000B` to the classic handler, which is load-bearing; `create=on`
/// sets the protected flag gating "Create a character", re-armed on every login result
/// because the client's handshake zeroes it.
///
/// # `guardpage=` ships here as of 2026-09-08, and that is a change of blast radius
///
/// The owner: *"work under the assumption that if this works, all of the clients should have it."*
/// The quarantine was previously armed only by `tools/test-server.ps1 -GuardPage`, which writes
/// a session **pin**. Players do not run that script - they run this launcher - so before this
/// change a player's client had no guard page at all, which is exactly why the 2026-09-08 01:33
/// overnight run carried no `guardpage=` token.
///
/// What is measured and what is not, so the risk is sized honestly:
///
/// * `[L]` It has armed on a client, passed its own control, and run 1 h 57 m quarantining
///   `0x20`, against a previous best of 1 h 57 m and a worst of 8 minutes. A second run with
///   `0x20+0x40` was at 1 h 22 m with zero damage, zero catches and zero fall-back when this
///   was written.
/// * `[L]` It has run on **exactly one machine, the owner's.** Never on a player's.
/// * `[D]` It costs 32 GiB of *reserved* address space (never committed as a whole), ~100 MB of
///   live pages per quarantined class, a 32 MB retirement ring, and lazily-committed metadata
///   that grows with the cursor.
/// * `[L]` It patches the client: one inline hook on the pool allocator, and a `HeapFree`
///   pointer swap. Every failure path in `guardpage::arm` reverts to an unpatched client.
///
/// Because of the last two, it has a kill switch that needs no rebuild - see
/// [`HOOK_GUARDPAGE_OFF_MARKER`] and the `guardpage` key in `maplecw-launcher.toml`.
pub const DEFAULT_SESSION: &str = "mode=2,create=on,guardpage=0x20+0x40";

/// A stub smaller than this is not a DLL. Checked before anything is displaced, the same way
/// `setup-client.ps1` does it.
pub const MIN_STUB_BYTES: u64 = 1024;

/// One line of progress for the UI's log pane.
pub type Steps = Vec<String>;

// ---------------------------------------------------------------------------------------
// The original-install guard
// ---------------------------------------------------------------------------------------

/// Compare two directory paths without requiring either to exist.
///
/// `canonicalize` is tried first and is authoritative when it works, but it **fails on a path
/// that is not there** - and on a test machine with no Nexon install that is exactly the
/// case, which would quietly turn the guard off. So a normalised textual comparison is the
/// fallback rather than the other way round.
fn normalise(p: &Path) -> String {
    let text = p.to_string_lossy().replace('/', "\\");
    let text = text.strip_prefix(r"\\?\").unwrap_or(&text).to_string();
    let trimmed = text.trim_end_matches('\\');
    // A bare drive letter must keep its separator: `C:` and `C:\` mean different things to
    // Windows, and trimming turns the second into the first.
    if trimmed.len() == 2 && trimmed.ends_with(':') {
        format!("{trimmed}\\").to_ascii_lowercase()
    } else {
        trimmed.to_ascii_lowercase()
    }
}

fn canonical_or_raw(p: &Path) -> String {
    match std::fs::canonicalize(p) {
        Ok(c) => normalise(&c),
        Err(_) => normalise(p),
    }
}

/// Is `child` the same directory as `parent`, or inside it?
pub fn is_within(child: &Path, parent: &Path) -> bool {
    let c = canonical_or_raw(child);
    let p = canonical_or_raw(parent);
    if c == p {
        return true;
    }
    let p_prefix = if p.ends_with('\\') { p } else { format!("{p}\\") };
    c.starts_with(&p_prefix)
}

/// Refuse to touch the original install.
///
/// Wider than `setup-client.ps1`, which compares for equality only: this also refuses a
/// client directory *inside* the original install, and an original install *inside* the
/// client directory. Neither can happen by accident on a correct setup, and both would
/// modify files the project has promised never to modify.
pub fn refuse_original_install(client_dir: &Path, original: &Path) -> Result<(), String> {
    if is_within(client_dir, original) || is_within(original, client_dir) {
        return Err(format!(
            "refusing to touch the original install. {} overlaps {} - point the launcher at \
             the copy (client-patched\\ in a checkout, client\\ beside an installed launcher)",
            client_dir.display(),
            original.display()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// The GameGuard stub
// ---------------------------------------------------------------------------------------

/// Install the no-op `grap64.dll`, idempotently.
///
/// Running this twice must leave `grap64.dll.orig` holding the **real** DLL, not the stub.
/// That is the one thing in this file that cannot be undone if it goes wrong.
pub fn stub_gameguard(client_dir: &Path, stub: &Path) -> Result<Steps, String> {
    let mut steps = Steps::new();

    if !client_dir.is_dir() {
        return Err(format!("client directory not found: {}", client_dir.display()));
    }
    let stub_len = match std::fs::metadata(stub) {
        Ok(m) => m.len(),
        Err(_) => {
            return Err(format!(
                "GameGuard stub not found at {}. Build it first: cargo build --release -p grap-stub",
                stub.display()
            ))
        }
    };
    if stub_len < MIN_STUB_BYTES {
        return Err(format!(
            "the stub at {} looks empty ({stub_len} bytes) - refusing to displace {GRAP_DLL} with it",
            stub.display()
        ));
    }

    let dll = client_dir.join(GRAP_DLL);
    let backup = client_dir.join(GRAP_BACKUP);

    // THE ORDER HERE IS THE POINT. Back up only when there is no backup; otherwise the
    // second run copies the stub over the real DLL's only copy.
    if backup.exists() {
        steps.push(format!("{GRAP_BACKUP} already exists - left alone (this is the real DLL)"));
    } else {
        if !dll.is_file() {
            return Err(format!(
                "no {GRAP_DLL} in {} - is that a client directory?",
                client_dir.display()
            ));
        }
        std::fs::copy(&dll, &backup)
            .map_err(|e| format!("could not back up {GRAP_DLL} to {GRAP_BACKUP}: {e}"))?;
        steps.push(format!("backed up the real {GRAP_DLL} -> {GRAP_BACKUP}"));
    }

    // **Skip the copy when the stub is already installed**, and that is what makes a SECOND
    // client possible on one machine.
    //
    // The owner, 2026-09-01: *"when I tried to start a second client, the grap.dll stub failed
    // because it was being used by another process"*. A running client has this DLL mapped,
    // and Windows refuses to write a mapped image - so the copy raised a sharing violation,
    // `prepare` returned `Err`, and the second launcher gave up **before it ever started a
    // client**. On screen that is indistinguishable from "the client refuses to run twice",
    // which is the conclusion it nearly bought.
    //
    // The backup step above has been idempotent since it was written, for the same class of
    // reason - "back up only when there is no backup". This is that rule applied to the
    // install: if the bytes on disk are already the bytes we would write, there is nothing to
    // do and a locked file is not an error. If they DIFFER we still try, and still fail
    // loudly, because then the client genuinely is running the wrong DLL.
    //
    // Compared by content rather than by length: a stale stub from an older build is exactly
    // the case that must NOT be skipped, and it is the case a length check would miss.
    let already_installed = match (std::fs::read(&dll), std::fs::read(stub)) {
        (Ok(on_disk), Ok(want)) => on_disk == want,
        _ => false,
    };
    if already_installed {
        steps.push(format!(
            "{GRAP_DLL} is already this exact stub ({stub_len} bytes) - left alone, so a \
             client already running does not block a second launch"
        ));
    } else {
        std::fs::copy(stub, &dll)
            .map_err(|e| format!("could not install the stub over {GRAP_DLL}: {e}"))?;
        steps.push(format!("installed the stub as {GRAP_DLL} ({stub_len} bytes)"));
    }

    let grap = client_dir.join(GRAP_DIR);
    let disabled = client_dir.join(GRAP_DIR_DISABLED);
    if grap.is_dir() && !disabled.exists() {
        std::fs::rename(&grap, &disabled)
            .map_err(|e| format!("could not rename {GRAP_DIR}\\ to {GRAP_DIR_DISABLED}: {e}"))?;
        steps.push(format!(
            "renamed {GRAP_DIR}\\ -> {GRAP_DIR_DISABLED} (NGService.exe and BlackCat64.sys cannot run)"
        ));
    } else if disabled.exists() {
        steps.push(format!("{GRAP_DIR_DISABLED} already in place"));
    } else {
        steps.push(format!("no {GRAP_DIR}\\ folder in this client - nothing to disable"));
    }

    steps.push("GameGuard neutralised: no service created, no driver loaded".into());
    Ok(steps)
}

// ---------------------------------------------------------------------------------------
// The hook's marker files
// ---------------------------------------------------------------------------------------

/// Write the four files that switch the hook on.
///
/// **Files, not environment variables**, and that is not a style choice: the client is
/// started with `ShellExecuteW` because it carries an elevation manifest, and ShellExecute
/// does not carry the environment into the child. `crates/grap-stub` reads all four relative
/// to its working directory, which is why the client is launched with the client directory as
/// its working directory.
///
/// Contents are written as UTF-8 with no BOM and no trailing newline. The hook trims what it
/// reads, so a newline would be harmless, but a BOM would not - `read_to_string` keeps it and
/// `trim` does not remove it, so `"\u{feff}watch@..."` would fail every prefix test in
/// `probe.rs` silently.
/// `grap_stub::poolsentry`'s marker. Written by the launcher unless one is already there.
pub const HOOK_SENTRY_MARKER: &str = "maplecw-hook.sentry";

/// What the launcher arms the pool sentry with on an ordinary launch.
///
/// # Why this ships
///
/// The client corrupts one 32-byte pool slot every 180 seconds and dies of it - the pool's
/// free reads a header whose high dword should be zero, misclassifies the slot, and hands it
/// to `HeapFree`. `repair=on` puts that dword back, which turns the fatal free into an
/// ordinary one. Measured 2026-09-07: 26 minutes, seven catches, **seven repairs, no death**,
/// against the same idle session the night before dying at 23 minutes.
/// `research/the-180-second-clock-2026-09-07.md`.
///
/// It does **not** stop the writer, and it is a race - a write and a free inside one walk
/// interval still dies. It is a mitigation and every report that depends on the client having
/// stayed alive should say it was on.
///
/// # Why these settings and not the diagnostic ones
///
/// The owner, 2026-09-07: *"it lags/freezes the client every time it runs, which is undesirable."*
/// Measured from that run's own heartbeats: an ordinary walk costs 0.63-0.71 ms, a finding
/// without a dump costs 49 ms (the 68-thread stack scan), and a finding **with** a dump costs
/// 703-895 ms with the client frozen throughout. So a shipped client gets `dumps=0` and
/// `stacks=off` - a player has no use for a 1.3 GB minidump - and `coarse=2000`, which walks
/// every 2 s except within five seconds of a predicted firing. The period is learned from the
/// findings, so the first two catches are still at the fine interval.
///
/// # It never overrides a marker that is already there
///
/// `tools/test-server.ps1 -PoolSentry` writes this file to arm a **measurement** run, and the
/// launcher starts the client. Overwriting it would silently turn every such run into a
/// shipping-settings run and quietly discard the dumps it was asked for.
pub const SHIPPED_SENTRY: &str = "dumps=0,stacks=off,coarse=2000,repair=on";

/// **The kill switch for the guard-page quarantine, and it needs no rebuild.**
///
/// # Why a file and not a flag
///
/// The quarantine ships on by default (see [`DEFAULT_SESSION`]) and has run on exactly one
/// machine. If it misbehaves on somebody else's, the fix has to be reachable by a person who
/// has a launcher `.exe` and a chat window - not a Rust toolchain. So:
///
/// * **Absent means ON.** Forgetting the file is not a way to lose the mitigation, the same
///   rule `crate::config::LauncherConfig::firewall` states for the outbound block.
/// * **Present means OFF**, whatever it contains. Contents are ignored as a *setting* and
///   echoed into the log pane as a note, so "off because Pixel's client stuttered, 2026-09-09"
///   can be written inside it and read back later. A file called `.off` that could itself say
///   `on` is a switch with two states and three meanings.
/// * Turning it off writes a session marker with **no `guardpage=` token at all**, which is
///   byte-for-byte what a client ran before this feature existed. `guardpage::install` then
///   logs `NOT ARMED` and hooks nothing.
///
/// One player: put the file in that player's client folder. Everyone: `guardpage = "off"` in
/// `maplecw-launcher.toml`, which is the same shape as `firewall = "off"` and is what an
/// installer can write. Either alone is enough; neither needs a new binary.
pub const HOOK_GUARDPAGE_OFF_MARKER: &str = "maplecw-hook.guardpage.off";

/// A one-launch override for [`HOOK_PROBE_MARKER`], written by `tools/test-server.ps1`.
pub const HOOK_PROBE_PIN: &str = "maplecw-hook.probe.pin";
/// The same for [`HOOK_SESSION_MARKER`].
pub const HOOK_SESSION_PIN: &str = "maplecw-hook.session.pin";

/// Read a pin, delete it, and return its contents in place of this launch's default.
///
/// # Why this exists, and it is not a convenience
///
/// The launcher writes the probe and session markers with [`DEFAULT_PROBE`] and
/// [`DEFAULT_SESSION`] on every launch, and it is the only path that reaches the world - a
/// direct client has not got past character select since 2026-09-05 and three launches went
/// into finding that out (`research/is-the-corruption-ours-2026-09-06.md` §5). So **the
/// launcher's defaults were, in practice, the only patch set the client could ever run**, and
/// two separate measurements were blocked on that:
///
/// * the heap-corruption patch control - 75 of 75 archived runs carried the full hook, so
///   nothing on disk separates "this client corrupts its heap" from "it does so while we are
///   inside it";
/// * a watch on `FUN_140ca61d0`, the 32-byte array allocator the 180-second ticker family
///   calls (`research/the-180-second-clock-2026-09-07.md`), which needs a probe slot the
///   default set does not leave free.
///
/// # It is deleted on read, and that is the guard
///
/// Every marker convention in this project is read-once, for the reason
/// `crates/grap-stub/src/identity.rs` and the sentry block both give: a marker left behind by
/// one run silently turns the *next* one into an instrumented run whose logs nobody would
/// think to distrust. A pin is the highest-consequence marker of the lot - it changes which
/// bytes of the client are patched - so it is deleted before it is used, not after, and the
/// substitution is announced in the launcher's own step log.
///
/// An unreadable or empty pin is ignored and the default stands. A pin that cannot be deleted
/// is **refused**, because leaving one on disk is exactly the failure this is guarding.
fn take_pin(client_dir: &Path, name: &str, default: &str, steps: &mut Steps) -> String {
    let path = client_dir.join(name);
    let Ok(body) = std::fs::read_to_string(&path) else {
        return default.to_string();
    };
    let body = body.trim().to_string();
    if let Err(e) = std::fs::remove_file(&path) {
        steps.push(format!(
            "WARNING: {name} could not be deleted ({e}) - IGNORING it and using the default, \
             because a pin that survives this launch would silently instrument the next one"
        ));
        return default.to_string();
    }
    if body.is_empty() {
        steps.push(format!("{name} was empty - deleted, default kept"));
        return default.to_string();
    }
    steps.push(format!(
        "{name} OVERRIDES this launch's default and has been deleted - one launch only. \
         Default was {default:?}"
    ));
    body
}

/// Remove every `guardpage=` term from a comma-separated session string.
///
/// Returns the string and whether anything was taken out. Pure, and it is the whole of what
/// "off" means: `grap_stub::session::marker_token` splits the marker on commas and
/// `guardpage::install` arms only if it finds the prefix, so a marker with no such term leaves
/// the client with nothing hooked - not a disabled hook, no hook.
///
/// It strips **every** occurrence rather than the first, because a session string assembled
/// from a default plus a pin could carry two and stopping at one would leave the feature on
/// while the log said it was off.
pub fn strip_guardpage(session: &str) -> (String, bool) {
    let kept: Vec<&str> = session
        .split(',')
        .filter(|t| !t.trim().to_ascii_lowercase().starts_with(GUARDPAGE_PREFIX))
        .collect();
    let removed = kept.len() != session.split(',').count();
    (kept.join(","), removed)
}

/// Why the guard page is on or off for this launch, in one sentence a player could read back
/// over chat.
///
/// `config_allows` is the `guardpage` key from `maplecw-launcher.toml` (absent = `true`). The
/// marker beside the client is checked here, so **either** switch alone turns it off and a
/// launcher with no config file still has a working kill switch.
pub fn guardpage_decision(client_dir: &Path, config_allows: bool) -> (bool, String) {
    let marker = client_dir.join(HOOK_GUARDPAGE_OFF_MARKER);
    let note = std::fs::read_to_string(&marker).ok();
    if let Some(note) = note {
        let note = note.trim().to_string();
        let because = if note.is_empty() {
            String::new()
        } else {
            format!(" The file says: {note:?}.")
        };
        return (
            false,
            format!(
                "WARNING: the heap quarantine (guard page) is OFF for this launch because \
                 {} exists.{because} Delete that file to turn it back on. This client runs \
                 exactly as it did before the quarantine existed: nothing is hooked, and \
                 whatever was making it die every few minutes will do so again",
                marker.display()
            ),
        );
    }
    if !config_allows {
        return (
            false,
            format!(
                "WARNING: the heap quarantine (guard page) is OFF for this launch because \
                 guardpage = \"off\" is set in {}. Remove that line, or set it to \"on\", to \
                 turn it back on. This client runs exactly as it did before the quarantine \
                 existed: nothing is hooked",
                crate::config::CONFIG_FILE_NAME
            ),
        );
    }
    (
        true,
        format!(
            "heap quarantine ON ({GUARDPAGE_PREFIX}{SHIPPED_GUARDPAGE_CLASSES}): freed memory \
             of those two size classes is held back for ten minutes so a stale write faults \
             where it is made instead of killing the client minutes later. To turn it OFF \
             without a new launcher, create an empty file called {HOOK_GUARDPAGE_OFF_MARKER} \
             beside MapleStory.exe (or put guardpage = \"off\" in {}) and start the game again"
            ,
            crate::config::CONFIG_FILE_NAME
        ),
    )
}

pub fn write_markers(
    client_dir: &Path,
    probe: &str,
    session: &str,
    dump_dir: &Path,
    guardpage_allowed: bool,
) -> Result<Steps, String> {
    let mut steps = Steps::new();

    let write = |name: &str, body: &str| -> Result<(), String> {
        let path = client_dir.join(name);
        std::fs::write(&path, body.as_bytes())
            .map_err(|e| format!("could not write {}: {e}", path.display()))
    };

    // **A pin overrides this launch's defaults, once.** See [`take_pin`].
    let was_pinned = client_dir.join(HOOK_SESSION_PIN).is_file();
    let probe = take_pin(client_dir, HOOK_PROBE_PIN, probe, &mut steps);
    let session = take_pin(client_dir, HOOK_SESSION_PIN, session, &mut steps);

    // **The kill switch is applied AFTER the pin, and it is the last word on this one token.**
    //
    // The two rules it has to satisfy pull in opposite directions only when both are present.
    // A pin still replaces the default wholesale - that is how a measurement run works and
    // `tools/test-server.ps1` depends on it. But "off" has to mean the marker carries no
    // `guardpage=` term, or it is not a kill switch; a switch that a leftover pin could defeat
    // is exactly the guard-whose-answer-is-ignored `CLAUDE.md` has a section about. So the pin
    // wins on everything else and this wins on `guardpage=`, and when it takes the token out of
    // a PIN it says so at WARNING rather than quietly disagreeing with the person who wrote it.
    let (on, why) = guardpage_decision(client_dir, guardpage_allowed);
    let session = if on {
        session
    } else {
        let (stripped, removed) = strip_guardpage(&session);
        if removed && was_pinned {
            steps.push(
                "WARNING: the off switch also removed the guardpage= term from the SESSION PIN \
                 this launch was given. The rest of the pin stands. Delete the off switch if \
                 this was a measurement run"
                    .to_string(),
            );
        }
        if stripped.trim().is_empty() {
            steps.push(
                "WARNING: with guardpage= removed there is nothing left in the session marker, \
                 so mode=2 and create=on are NOT set either - that is what the pin asked for \
                 minus the guard page, and the client will behave accordingly"
                    .to_string(),
            );
        }
        stripped
    };
    steps.push(why);
    let probe = probe.as_str();
    let session = session.as_str();

    write(HOOK_ENABLE_MARKER, "")?;
    write(HOOK_PROBE_MARKER, probe)?;
    write(HOOK_SESSION_MARKER, session)?;
    write(HOOK_MULTICLIENT_MARKER, "")?;

    let dump_text = dump_dir.to_string_lossy().to_string();
    write(HOOK_DUMPDIR_MARKER, &dump_text)?;

    // **The heap repair ships with the client.** See [`SHIPPED_SENTRY`].
    let sentry = client_dir.join(HOOK_SENTRY_MARKER);
    if sentry.exists() {
        steps.push(format!(
            "{HOOK_SENTRY_MARKER} was already here - left alone. This launch uses ITS settings, \
             not the shipping ones"
        ));
    } else {
        write(HOOK_SENTRY_MARKER, SHIPPED_SENTRY)?;
        steps.push(format!("heap repair armed ({SHIPPED_SENTRY})"));
    }
    // **The sentry marker is never REMOVED or overwritten here**, and that rule is older than
    // the repair. An earlier version deleted it, on the reasoning that the launcher is the
    // shipping path. That was wrong and it broke the ordinary diagnostic combination:
    // `test-server.ps1 -ServersOnly` arms the sentry and the LAUNCHER starts the client, so
    // removing it here made the two impossible to use together. The hook deletes the marker
    // itself once it has read it, which closes the stale-marker hole without closing that
    // path. Writing a shipping default when there is none, above, keeps both.
    if !dump_text.is_ascii() {
        steps.push(format!(
            "WARNING: the dump directory is not ASCII ({dump_text}) - the hook reads it as UTF-8, \
             which is fine, but nothing else in this project has been tested with one"
        ));
    }

    steps.push(format!("hook enabled ({HOOK_ENABLE_MARKER})"));
    steps.push(format!("client patches: {probe}"));
    steps.push(format!("session patches: {session}"));
    steps.push(format!("crash dumps -> {dump_text}"));
    steps.push(format!(
        "instance-guard hooks armed ({HOOK_MULTICLIENT_MARKER}) - the FIRST client only logs; a \n         second one gets the guard suppressed"
    ));
    Ok(steps)
}

/// Put this launch's client token where the hook can find it - or **remove the last one**.
///
/// # A stale token is worse than no token, and that is the whole reason this is a function
///
/// The login server's anti-downgrade rule gives a connection that presents a credential *that
/// claim or none*: it deliberately does **not** fall through to the weaker rules, because a
/// guard that can be skipped by presenting junk is not a guard. So a marker left over from a
/// previous launch does not merely fail to help - it takes this launch from "resolved by the
/// process that owns the socket" down to "served as the server's fallback account".
///
/// Which means the `None` arm is not an omission to tidy up later. It is the arm that has to
/// be right. Every path that does not have a fresh token deletes the file:
///
/// * signing in against a server that predates the client token;
/// * a sign-in that could not stake a claim;
/// * `Start Game` reached without one, which the caller also warns about.
///
/// Returns the lines for the log pane. The token itself is never one of them.
pub fn write_identity_marker(client_dir: &Path, token: Option<&str>) -> Result<Steps, String> {
    let path = client_dir.join(HOOK_IDENTITY_MARKER);
    let mut steps = Steps::new();
    match token {
        Some(token) if !token.trim().is_empty() => {
            // No BOM, no trailing newline, for the same reason the other markers have neither:
            // `read_to_string` keeps a BOM and `trim` does not remove it. The hook trims ASCII
            // whitespace, so a stray newline is harmless and a BOM would not be.
            std::fs::write(&path, token.trim().as_bytes())
                .map_err(|e| format!("could not write {}: {e}", path.display()))?;
            steps.push(format!(
                "client credential written ({HOOK_IDENTITY_MARKER}, {} characters). The client \
                 will carry it in 0x0073 and the server will spend it once. IT IS PLAIN TEXT \
                 ON DISK: anything running as you can read it, which is the same power as \
                 being this launch",
                token.trim().chars().count()
            ));
        }
        _ => {
            // `remove_file` on a path that is not there is `NotFound`, which is success here.
            match std::fs::remove_file(&path) {
                Ok(()) => steps.push(format!(
                    "removed a previous {HOOK_IDENTITY_MARKER}: this launch has no client \
                     token, and a token from an earlier launch would be REFUSED rather than \
                     ignored - the connection would be served as the server's fallback account \
                     instead of being attributed by its owning process"
                )),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    return Err(format!(
                        "could not remove the stale {}: {e}. Refusing to launch with it in \
                         place - the client would present a dead credential and be served as \
                         the fallback account",
                        path.display()
                    ))
                }
            }
        }
    }
    Ok(steps)
}

// ---------------------------------------------------------------------------------------
// Archiving the previous run
// ---------------------------------------------------------------------------------------

/// Move the previous `maplecw-hook.log` into `<data_root>\previous-runs\`, timestamped.
///
/// **Never deletes.** If the timestamped name is already taken, a counter is appended rather
/// than overwriting - `tools/test-server.ps1` uses `Move-Item -Force` here, and this is the
/// one place this port deliberately differs, because a `-Force` that lands on an existing
/// archive destroys the older run.
///
/// Returns the archive path, or `None` if there was no previous log.
pub fn archive_previous_log(log_path: &Path, into: &Path) -> Result<Option<PathBuf>, String> {
    if !log_path.is_file() {
        return Ok(None);
    }
    let dir = into.join("previous-runs");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("could not create {}: {e}", dir.display()))?;

    let stamp = last_write_stamp(log_path);
    let stem = log_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "log".into());
    let ext = log_path
        .extension()
        .map(|s| format!(".{}", s.to_string_lossy()))
        .unwrap_or_default();

    let mut target = dir.join(format!("{stem}-{stamp}{ext}"));
    let mut n = 2;
    while target.exists() {
        target = dir.join(format!("{stem}-{stamp}-{n}{ext}"));
        n += 1;
        if n > 100 {
            return Err(format!(
                "could not find a free name for the previous {} in {}",
                log_path.display(),
                dir.display()
            ));
        }
    }

    // `rename` fails across volumes; copy-then-remove is the fallback, and it is still never
    // a delete-without-a-copy.
    if std::fs::rename(log_path, &target).is_err() {
        std::fs::copy(log_path, &target)
            .map_err(|e| format!("could not archive {}: {e}", log_path.display()))?;
        if std::fs::remove_file(log_path).is_err() {
            // **Another client is still writing this log.** Take the copy back out and leave
            // the original alone.
            //
            // Two reasons, and the second is the one that matters. A second launcher must not
            // fail here - that would block a second client for a reason that has nothing to
            // do with the client. And archiving a log that is still being appended to
            // produces exactly the near-duplicate `CLAUDE.md` describes under "a fixture is
            // copied while the run is still being written": eleven such pairs already exist
            // in this repo, they hash differently from their own run, and file-level
            // deduplication counted them as two observations.
            //
            // So: no half-archive, no error, and the running client keeps its log.
            let _ = std::fs::remove_file(&target);
            return Ok(None);
        }
    }
    Ok(Some(target))
}

/// `yyyyMMdd-HHmmss` in **local** time, matching the names `tools/test-server.ps1` writes, so
/// the archived hook logs sort beside the archived `world.log`s from the same run.
fn last_write_stamp(path: &Path) -> String {
    let modified = std::fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs());
    let Some(unix) = modified else {
        return "unknown".to_string();
    };
    match local_civil_from_unix(unix) {
        Some((y, mo, d, h, mi, s)) => format_stamp(y, mo, d, h, mi, s),
        // No local time available: the raw seconds still sort correctly and still never
        // collide, which is all the name has to do.
        None => format!("unix{unix}"),
    }
}

fn format_stamp(y: u16, mo: u16, d: u16, h: u16, mi: u16, s: u16) -> String {
    format!("{y:04}{mo:02}{d:02}-{h:02}{mi:02}{s:02}")
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct FileTime {
    low: u32,
    high: u32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct Win32SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

extern "system" {
    fn FileTimeToLocalFileTime(ft: *const FileTime, out: *mut FileTime) -> i32;
    fn FileTimeToSystemTime(ft: *const FileTime, out: *mut Win32SystemTime) -> i32;
}

/// Unix seconds -> local `(year, month, day, hour, minute, second)`.
///
/// Done through Win32 rather than by hand because "local" here means whatever the machine's
/// time zone and DST rules say, and reimplementing that is exactly the kind of confident
/// wrong answer this project keeps paying for.
fn local_civil_from_unix(unix: u64) -> Option<(u16, u16, u16, u16, u16, u16)> {
    // FILETIME counts 100 ns ticks from 1601-01-01; 11644473600 seconds separate the epochs.
    let ticks = unix.checked_add(11_644_473_600)?.checked_mul(10_000_000)?;
    let utc = FileTime {
        low: ticks as u32,
        high: (ticks >> 32) as u32,
    };
    let mut local = FileTime::default();
    let mut st = Win32SystemTime::default();
    // SAFETY: both calls take a pointer to one initialised struct and write one struct we own.
    unsafe {
        if FileTimeToLocalFileTime(&utc, &mut local) == 0 {
            return None;
        }
        if FileTimeToSystemTime(&local, &mut st) == 0 {
            return None;
        }
    }
    Some((st.year, st.month, st.day, st.hour, st.minute, st.second))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::TempDir;

    fn fake_client(t: &TempDir) -> PathBuf {
        t.file("client/MapleStory.exe", "client");
        t.file("client/grap64.dll", "THE REAL GAMEGUARD DLL");
        t.dir("client/grap");
        t.path().join("client")
    }

    fn fake_stub(t: &TempDir) -> PathBuf {
        t.file("target/release/grap64.dll", &"S".repeat(4096))
    }

    // -- the backup rule ---------------------------------------------------------------

    #[test]
    fn the_first_run_backs_up_the_real_dll_and_installs_the_stub() {
        let t = TempDir::new("stub1");
        let client = fake_client(&t);
        let stub = fake_stub(&t);

        let steps = stub_gameguard(&client, &stub).expect("stub");
        assert!(steps.iter().any(|s| s.contains("backed up")), "{steps:?}");

        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
        assert_eq!(std::fs::read_to_string(client.join(GRAP_DLL)).unwrap().len(), 4096);
        assert!(client.join(GRAP_DIR_DISABLED).is_dir());
        assert!(!client.join(GRAP_DIR).exists());
    }

    /// **A second launcher must not try to write a DLL the first client has open.**
    ///
    /// The owner, 2026-09-01: *"when I tried to start a second client, the grap.dll stub failed
    /// because it was being used by another process"*. A running client has `grap64.dll`
    /// mapped and Windows refuses to write a mapped image, so the copy raised a sharing
    /// violation, `prepare` returned `Err`, and the second launcher stopped **before it ever
    /// started a client**. That reads on screen as "the client will not run twice", which is
    /// a completely different and much more expensive conclusion.
    ///
    /// The check cannot be "does the file exist" - a stale stub from an older build has to be
    /// replaced. It is a **content** comparison, and this test asserts both directions,
    /// because a skip that always skips is the same bug wearing the opposite sign.
    /// **The four marker names, spelled the way the stub reads them.**
    ///
    /// The launcher does not depend on `grap-stub` - they are two separate binaries that meet
    /// only through files in the client directory - so nothing but this test connects the two
    /// spellings. A typo here does not fail to compile; it produces a marker nobody reads and
    /// a feature that is silently off, which is the failure mode `CLAUDE.md` calls
    /// indistinguishable from the code not existing.
    #[test]
    fn the_marker_names_are_the_ones_the_stub_looks_for() {
        assert_eq!(HOOK_ENABLE_MARKER, "maplecw-hook.enable");
        assert_eq!(HOOK_DUMPDIR_MARKER, "maplecw-hook.dumpdir");
        assert_eq!(HOOK_MULTICLIENT_MARKER, "maplecw-hook.multiclient");
        // All of them are bare file names resolved against the client's working directory,
        // never paths - the stub joins nothing.
        for m in [HOOK_ENABLE_MARKER, HOOK_DUMPDIR_MARKER, HOOK_MULTICLIENT_MARKER] {
            // Byte 92 is the backslash, written as a number so this line cannot be
            // mangled by a heredoc on the way into the file - which is exactly what
            // happened on the first attempt, and CLAUDE.md warns about.
            assert!(
                !m.as_bytes().contains(&b'/') && !m.as_bytes().contains(&92),
                "{m} is a bare file name"
            );
        }
    }

    /// **A launch writes the multiclient marker**, or the second client is never treated.
    #[test]
    fn write_markers_arms_the_instance_guard_hooks() {
        let t = TempDir::new("markers-mc");
        let client = fake_client(&t);
        let dumps = t.path().join("dumps");
        let steps = write_markers(&client, "probe", "session", &dumps, true).expect("markers");
        assert!(
            client.join(HOOK_MULTICLIENT_MARKER).is_file(),
            "the marker file must exist beside the client"
        );
        assert!(
            steps.iter().any(|s| s.contains("instance-guard")),
            "and the launcher must say so on screen: {steps:?}"
        );
        // The control: the markers that were already there still are, so this did not
        // displace them.
        assert!(client.join(HOOK_ENABLE_MARKER).is_file());
        assert!(client.join(HOOK_DUMPDIR_MARKER).is_file());
    }

    #[test]
    fn an_identical_stub_is_not_rewritten_so_a_running_client_cannot_block_a_second_launch() {
        let t = TempDir::new("stubidem");
        let client = fake_client(&t);
        let stub = fake_stub(&t);

        let first = stub_gameguard(&client, &stub).expect("first");
        assert!(first.iter().any(|s| s.contains("installed the stub")), "{first:?}");

        // Second launch, nothing changed on disk: the copy must be SKIPPED.
        let second = stub_gameguard(&client, &stub).expect("second");
        assert!(
            second.iter().any(|s| s.contains("left alone")),
            "the second run must not rewrite an identical stub: {second:?}"
        );
        assert!(
            !second.iter().any(|s| s.contains("installed the stub")),
            "and must not claim it installed one: {second:?}"
        );
        assert_eq!(std::fs::read(client.join(GRAP_DLL)).unwrap().len(), 4096);

        // **The other direction.** A DIFFERENT stub - a rebuild - must still be installed,
        // or this "fix" would quietly pin every client to whatever was there first.
        let newer = t.file("target/release/grap64.dll", &"N".repeat(5000));
        let third = stub_gameguard(&client, &newer).expect("third");
        assert!(
            third.iter().any(|s| s.contains("installed the stub")),
            "a rebuilt stub must still replace the old one: {third:?}"
        );
        let on_disk = std::fs::read(client.join(GRAP_DLL)).unwrap();
        assert_eq!(on_disk.len(), 5000);
        assert!(on_disk.iter().all(|&b| b == b'N'), "the new bytes, not the old ones");

        // And through all of that the real DLL is still backed up, untouched.
        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
    }

    #[test]
    fn a_second_run_must_not_overwrite_the_backup_with_the_stub() {
        // This is the one mistake in this file that cannot be undone: get it wrong and the
        // real grap64.dll is gone from the machine.
        let t = TempDir::new("stub2");
        let client = fake_client(&t);
        let stub = fake_stub(&t);

        stub_gameguard(&client, &stub).expect("first");
        let steps = stub_gameguard(&client, &stub).expect("second");

        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL",
            "the second run replaced the backup with the stub"
        );
        assert!(steps.iter().any(|s| s.contains("already exists")), "{steps:?}");
        // ...and it is still idempotent about the folder, too.
        assert!(client.join(GRAP_DIR_DISABLED).is_dir());
    }

    #[test]
    fn a_third_run_is_still_safe() {
        let t = TempDir::new("stub3");
        let client = fake_client(&t);
        let stub = fake_stub(&t);
        for _ in 0..3 {
            stub_gameguard(&client, &stub).expect("run");
        }
        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
    }

    #[test]
    fn a_tiny_stub_is_refused_before_anything_is_displaced() {
        let t = TempDir::new("tinystub");
        let client = fake_client(&t);
        let stub = t.file("target/release/grap64.dll", "nope");

        let err = stub_gameguard(&client, &stub).unwrap_err();
        assert!(err.contains("looks empty"), "{err}");
        // Nothing moved.
        assert!(!client.join(GRAP_BACKUP).exists());
        assert_eq!(
            std::fs::read_to_string(client.join(GRAP_DLL)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
        assert!(client.join(GRAP_DIR).is_dir());
    }

    #[test]
    fn a_missing_stub_is_refused_with_the_build_command() {
        let t = TempDir::new("nostub");
        let client = fake_client(&t);
        let err = stub_gameguard(&client, &t.path().join("nowhere.dll")).unwrap_err();
        assert!(err.contains("cargo build --release -p grap-stub"), "{err}");
    }

    #[test]
    fn a_directory_with_no_grap64_is_refused_rather_than_backed_up_as_nothing() {
        let t = TempDir::new("notclient");
        let client = t.dir("client");
        let stub = fake_stub(&t);
        let err = stub_gameguard(&client, &stub).unwrap_err();
        assert!(err.contains("is that a client directory"), "{err}");
        assert!(!client.join(GRAP_BACKUP).exists());
    }

    // -- the original-install guard ----------------------------------------------------

    #[test]
    fn the_original_install_is_refused_even_though_it_does_not_exist_here() {
        // The guard has to work on a machine with no Nexon install, which is exactly where
        // `canonicalize` cannot help.
        let original = Path::new(ORIGINAL_INSTALL);
        let err = refuse_original_install(original, original).unwrap_err();
        assert!(err.contains("refusing"), "{err}");
    }

    #[test]
    fn case_and_slashes_do_not_get_past_the_guard() {
        let original = Path::new(ORIGINAL_INSTALL);
        for candidate in [
            r"c:\nexon\library\maplestorycw\appdata",
            r"C:/Nexon/Library/maplestorycw/appdata",
            r"C:\Nexon\Library\maplestorycw\appdata\",
        ] {
            assert!(
                refuse_original_install(Path::new(candidate), original).is_err(),
                "{candidate} got through"
            );
        }
    }

    #[test]
    fn a_subdirectory_of_the_original_install_is_refused_too() {
        let original = Path::new(ORIGINAL_INSTALL);
        let inside = Path::new(r"C:\Nexon\Library\maplestorycw\appdata\Data");
        assert!(refuse_original_install(inside, original).is_err());
        // ...and so is a client dir that CONTAINS the original install.
        let outside = Path::new(r"C:\Nexon\Library\maplestorycw");
        assert!(refuse_original_install(outside, original).is_err());
    }

    #[test]
    fn an_ordinary_client_copy_passes_the_guard() {
        let original = Path::new(ORIGINAL_INSTALL);
        for ok in [
            r"C:\MapleCW\client-patched",
            r"D:\MapleCW\client",
            r"C:\Nexon\Library\maplestorycw-copy\appdata",
        ] {
            assert!(
                refuse_original_install(Path::new(ok), original).is_ok(),
                "{ok} was refused"
            );
        }
    }

    #[test]
    fn a_bare_drive_letter_is_not_treated_as_a_prefix_of_everything() {
        // `C:` normalised to `C` would make `is_within` true for every path on the drive.
        assert!(!is_within(Path::new(r"D:\game"), Path::new(r"C:\")));
        assert!(is_within(Path::new(r"C:\game"), Path::new(r"C:\")));
    }

    // -- the marker files ---------------------------------------------------------------

    /// **A pin replaces the default for exactly one launch and is gone afterwards.**
    ///
    /// The read-once half is the part that matters and it is asserted in both directions: the
    /// first launch takes the pin, and a second `write_markers` against the same directory
    /// gets the default back. A pin that survived would silently instrument the next run, and
    /// this project has already been bitten by a stale marker twice - the multiclient marker
    /// on 2026-09-06 and, before it, a probe marker turning an ordinary run into a watched one.
    #[test]
    fn a_pin_overrides_one_launch_and_then_is_gone() {
        let t = TempDir::new("pin");
        let client = t.dir("client");
        let dumps = t.dir("dumps");
        let mine = "watch@1415db360:ret,141b2a280:rdx=0,140ca61d0:hits=40";

        std::fs::write(client.join(HOOK_PROBE_PIN), mine).unwrap();
        std::fs::write(client.join(HOOK_SESSION_PIN), "mode=2").unwrap();

        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("first");
        assert_eq!(
            std::fs::read_to_string(client.join(HOOK_PROBE_MARKER)).unwrap(),
            mine,
            "the pin must reach the marker the hook actually reads"
        );
        assert_eq!(
            std::fs::read_to_string(client.join(HOOK_SESSION_MARKER)).unwrap(),
            "mode=2"
        );
        assert!(
            !client.join(HOOK_PROBE_PIN).exists() && !client.join(HOOK_SESSION_PIN).exists(),
            "a pin that survives its launch is the whole failure mode this guards"
        );
        assert!(
            steps.iter().any(|s| s.contains("OVERRIDES")),
            "the substitution must be announced, not silent: {steps:?}"
        );

        // The second launch, same directory, no pin: the defaults come back.
        write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("second");
        assert_eq!(
            std::fs::read_to_string(client.join(HOOK_PROBE_MARKER)).unwrap(),
            DEFAULT_PROBE
        );
        assert_eq!(
            std::fs::read_to_string(client.join(HOOK_SESSION_MARKER)).unwrap(),
            DEFAULT_SESSION
        );
    }

    /// **The heap repair ships, and a marker that is already there is left alone.**
    ///
    /// Both halves matter. Without the first, a player's client dies of the 180-second
    /// corruption every few minutes. Without the second, `test-server.ps1 -PoolSentry` would
    /// arm a measurement run and the launcher would silently overwrite it with the shipping
    /// settings, discarding the dumps that run was started to collect.
    #[test]
    fn the_launcher_arms_the_repair_but_never_overwrites_a_marker() {
        let t = TempDir::new("sentry");
        let client = t.dir("client");
        let dumps = t.dir("dumps");

        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("first");
        let written = std::fs::read_to_string(client.join(HOOK_SENTRY_MARKER)).unwrap();
        assert_eq!(written, SHIPPED_SENTRY);
        assert!(written.contains("repair=on"), "the whole point is the repair: {written}");
        assert!(written.contains("dumps=0"), "a player has no use for a 1.3 GB dump: {written}");
        assert!(steps.iter().any(|s| s.contains("heap repair armed")), "{steps:?}");

        // A marker already there is a measurement run being set up. Leave it.
        let mine = "dumps=4,repair=on";
        std::fs::write(client.join(HOOK_SENTRY_MARKER), mine).unwrap();
        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("second");
        assert_eq!(
            std::fs::read_to_string(client.join(HOOK_SENTRY_MARKER)).unwrap(),
            mine,
            "the launcher must not overwrite a sentry marker somebody else wrote"
        );
        assert!(steps.iter().any(|s| s.contains("left alone")), "{steps:?}");
    }

    /// An empty or whitespace-only pin is a mistake, not an instruction to patch nothing.
    #[test]
    fn an_empty_pin_is_deleted_and_the_default_stands() {
        let t = TempDir::new("pin-empty");
        let client = t.dir("client");
        let dumps = t.dir("dumps");
        std::fs::write(client.join(HOOK_PROBE_PIN), "   \r\n  ").unwrap();

        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("markers");
        assert_eq!(
            std::fs::read_to_string(client.join(HOOK_PROBE_MARKER)).unwrap(),
            DEFAULT_PROBE,
            "an empty pin must not disarm the two patches the client cannot live without"
        );
        assert!(!client.join(HOOK_PROBE_PIN).exists());
        assert!(steps.iter().any(|s| s.contains("was empty")), "{steps:?}");
    }

    #[test]
    fn the_four_markers_are_written_with_no_bom_and_no_newline() {
        let t = TempDir::new("markers");
        let client = t.dir("client");
        let dumps = t.dir("dumps");

        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("markers");
        assert!(steps.iter().any(|s| s.contains(DEFAULT_PROBE)), "{steps:?}");

        let enable = std::fs::read(client.join(HOOK_ENABLE_MARKER)).unwrap();
        assert!(enable.is_empty(), "the enable marker should be empty, got {enable:?}");

        for (name, expected) in [
            (HOOK_PROBE_MARKER, DEFAULT_PROBE.to_string()),
            (HOOK_SESSION_MARKER, DEFAULT_SESSION.to_string()),
            (HOOK_DUMPDIR_MARKER, dumps.to_string_lossy().to_string()),
        ] {
            let raw = std::fs::read(client.join(name)).unwrap();
            assert_ne!(&raw[..raw.len().min(3)], b"\xef\xbb\xbf", "{name} has a BOM");
            assert_eq!(String::from_utf8(raw).unwrap(), expected, "{name}");
        }
    }

    // -- the guard page and its kill switch ---------------------------------------------

    /// **The shipped default carries the quarantine, spelled the way the hook parses it.**
    ///
    /// The launcher does not depend on `grap-stub` - they meet only through files in the client
    /// directory - so nothing but this test and its twin in
    /// `grap_stub::guardpage::tests::the_class_set_the_launcher_ships_parses` connects the two
    /// spellings. In particular the classes are joined with `+`: a comma would be split by
    /// `session::marker_token` and would arm `0x20` while the log said `0x20+0x40`.
    #[test]
    fn the_shipped_session_arms_the_guard_page_on_the_two_damaged_classes() {
        assert!(DEFAULT_SESSION.contains("mode=2"));
        assert!(DEFAULT_SESSION.contains("create=on"));
        assert_eq!(
            DEFAULT_SESSION,
            format!("mode=2,create=on,{GUARDPAGE_PREFIX}{SHIPPED_GUARDPAGE_CLASSES}"),
            "a player's client gets the quarantine because THIS string is what the launcher \
             writes; test-server.ps1's -GuardPage pin only ever reached the owner's machine"
        );
        assert!(
            !SHIPPED_GUARDPAGE_CLASSES.contains(','),
            "a comma would be split by the session marker's own separator and arm half of it"
        );
    }

    /// `strip_guardpage` is what "off" means, so it is pinned on its own: every occurrence,
    /// case-insensitively, and nothing else disturbed.
    #[test]
    fn stripping_removes_every_guardpage_term_and_leaves_the_rest_alone() {
        assert_eq!(
            strip_guardpage(DEFAULT_SESSION),
            ("mode=2,create=on".to_string(), true)
        );
        // Nothing to take out: unchanged, and it says so.
        assert_eq!(
            strip_guardpage("mode=2,create=on"),
            ("mode=2,create=on".to_string(), false)
        );
        // Two of them, and one in the middle. Stopping at the first would leave the feature
        // ON while the log pane said OFF - the worst of the three possible outcomes.
        assert_eq!(
            strip_guardpage("guardpage=0x20,mode=2,GUARDPAGE=all,create=on"),
            ("mode=2,create=on".to_string(), true)
        );
        // A key that merely starts the same way is not this key.
        assert_eq!(
            strip_guardpage("guardpages=1,mode=2"),
            ("guardpages=1,mode=2".to_string(), false)
        );
    }

    /// **The switch, in both directions, through the function that actually writes the file.**
    ///
    /// A switch that has never turned anything off is a switch nobody has tested, so the
    /// assertion that matters is the second half: the marker on disk must contain no
    /// `guardpage=` at all. Not `guardpage=off`, not `guardpage=none` - absent, because
    /// `guardpage::install` arms on finding the prefix and logs `NOT ARMED` when it does not,
    /// which is byte-for-byte the client that ran before this feature existed.
    #[test]
    fn the_off_marker_takes_the_guard_page_out_of_the_session_and_says_so() {
        let t = TempDir::new("gpoff");
        let client = t.dir("client");
        let dumps = t.dir("dumps");

        // ON: the shipped default reaches the marker the hook reads.
        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("on");
        let written = std::fs::read_to_string(client.join(HOOK_SESSION_MARKER)).unwrap();
        assert!(written.contains(GUARDPAGE_PREFIX), "{written}");
        assert!(
            steps.iter().any(|s| s.contains("heap quarantine ON")),
            "the log pane must say which way it went: {steps:?}"
        );
        assert!(
            steps.iter().any(|s| s.contains(HOOK_GUARDPAGE_OFF_MARKER)),
            "and must name the file that turns it off, because that is the whole support \
             procedure: {steps:?}"
        );

        // OFF, by the marker file. Contents are a note, not a setting.
        std::fs::write(
            client.join(HOOK_GUARDPAGE_OFF_MARKER),
            "stuttering on Pixel's box 2026-09-09",
        )
        .unwrap();
        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("off");
        let written = std::fs::read_to_string(client.join(HOOK_SESSION_MARKER)).unwrap();
        assert!(
            !written.contains(GUARDPAGE_PREFIX),
            "the off switch did not reach the marker the hook reads: {written:?}"
        );
        assert_eq!(written, "mode=2,create=on", "and nothing else may be lost with it");
        let said = steps.iter().find(|s| s.contains("quarantine (guard page) is OFF")).expect(
            "the log pane must say it is off, in words a player could read back over chat",
        );
        assert!(said.starts_with("WARNING"), "and at WARNING, not buried in green: {said}");
        assert!(said.contains("Pixel"), "the note in the file is echoed: {said}");

        // ...and back ON when the file is gone. A switch that only latches is not a switch.
        std::fs::remove_file(client.join(HOOK_GUARDPAGE_OFF_MARKER)).unwrap();
        write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("on again");
        assert!(std::fs::read_to_string(client.join(HOOK_SESSION_MARKER))
            .unwrap()
            .contains(GUARDPAGE_PREFIX));
    }

    /// The other half of the switch: `guardpage = "off"` in the config file, for a whole
    /// machine or a whole install, with no file to place by hand.
    #[test]
    fn the_config_switch_alone_also_turns_it_off() {
        let t = TempDir::new("gpcfg");
        let client = t.dir("client");
        let dumps = t.dir("dumps");

        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, false).expect("off");
        let written = std::fs::read_to_string(client.join(HOOK_SESSION_MARKER)).unwrap();
        assert_eq!(written, "mode=2,create=on");
        assert!(
            steps
                .iter()
                .any(|s| s.starts_with("WARNING") && s.contains(crate::config::CONFIG_FILE_NAME)),
            "the log must name the file the setting is in, or nobody can undo it: {steps:?}"
        );
        assert!(!client.join(HOOK_GUARDPAGE_OFF_MARKER).exists(), "and it wrote no marker");
    }

    /// **A leftover pin must not defeat the kill switch, and the switch must not silently
    /// discard the rest of a pin.**
    ///
    /// The pin is how a measurement run works and it still overrides the default wholesale.
    /// But `off` has to mean *no `guardpage=` term reaches the hook*, or it is a guard whose
    /// answer can be ignored - the shape `CLAUDE.md` has a whole section about. So the pin wins
    /// on everything else, this wins on that one token, and the disagreement is announced
    /// rather than resolved in silence.
    #[test]
    fn the_off_switch_outranks_a_session_pin_on_the_guardpage_token_only() {
        let t = TempDir::new("gppin");
        let client = t.dir("client");
        let dumps = t.dir("dumps");
        std::fs::write(client.join(HOOK_GUARDPAGE_OFF_MARKER), "").unwrap();
        std::fs::write(client.join(HOOK_SESSION_PIN), "mode=2,create=on,guardpage=all,chat=on")
            .unwrap();

        let steps = write_markers(&client, DEFAULT_PROBE, DEFAULT_SESSION, &dumps, true).expect("pin");
        assert_eq!(
            std::fs::read_to_string(client.join(HOOK_SESSION_MARKER)).unwrap(),
            "mode=2,create=on,chat=on",
            "the pin's other terms must survive; only guardpage= is taken out"
        );
        assert!(
            steps.iter().any(|s| s.starts_with("WARNING") && s.contains("SESSION PIN")),
            "taking a term out of somebody's pin must be said out loud: {steps:?}"
        );
        // The pin is still read-once, exactly as before.
        assert!(!client.join(HOOK_SESSION_PIN).exists());
    }

    /// The credential is written verbatim: no BOM, no newline, no quoting. `grap_stub::identity`
    /// trims ASCII whitespace, so a newline would survive - but a BOM would not be trimmed and
    /// would be sent as three bytes of the token.
    #[test]
    fn the_credential_marker_is_written_verbatim() {
        let t = TempDir::new("identity");
        let client = t.dir("client");
        let token = "MFRGGZDFMZTWQ2LKNNWG23TP2A";

        let steps = write_identity_marker(&client, Some(token)).expect("write");
        let raw = std::fs::read(client.join(HOOK_IDENTITY_MARKER)).unwrap();
        assert_ne!(&raw[..raw.len().min(3)], b"\xef\xbb\xbf", "a BOM would be sent as data");
        assert_eq!(raw, token.as_bytes());
        // The count reaches the log pane so it can be compared against the hook log and
        // login.log; the token itself must not.
        assert!(steps.iter().any(|s| s.contains("26 characters")), "{steps:?}");
        assert!(!steps.iter().any(|s| s.contains(token)), "the token reached the log: {steps:?}");
        assert!(steps.iter().any(|s| s.contains("PLAIN TEXT")), "{steps:?}");
    }

    /// **The arm that has to be right.** A stale credential is refused by the login server,
    /// not ignored, so leaving one behind costs the next launch its account.
    #[test]
    fn no_credential_removes_the_previous_one_and_says_so() {
        let t = TempDir::new("identitynone");
        let client = t.dir("client");
        let path = client.join(HOOK_IDENTITY_MARKER);

        write_identity_marker(&client, Some("OLDTOKEN")).expect("write");
        assert!(path.is_file());

        let steps = write_identity_marker(&client, None).expect("remove");
        assert!(!path.exists(), "the stale credential survived");
        assert!(steps.iter().any(|s| s.contains("REFUSED")), "{steps:?}");

        // Twice is fine: nothing to remove is success, not NotFound.
        let steps = write_identity_marker(&client, None).expect("remove again");
        assert!(steps.is_empty(), "a no-op should say nothing: {steps:?}");
    }

    /// An empty or whitespace-only token is not a credential and must take the delete arm -
    /// otherwise a server answering `"client_token":""` leaves a file that looks like one.
    #[test]
    fn a_blank_credential_is_treated_as_none() {
        let t = TempDir::new("identityblank");
        let client = t.dir("client");
        write_identity_marker(&client, Some("STALE")).expect("write");
        write_identity_marker(&client, Some("  \r\n")).expect("blank");
        assert!(!client.join(HOOK_IDENTITY_MARKER).exists());
    }

    #[test]
    fn the_default_probe_keeps_the_two_patches_that_keep_the_client_alive() {
        // Not decoration. Without 1415db360:ret the client __fastfails at ~37s; without
        // 141b2a280:rdx=0 the Login button never becomes clickable.
        assert!(DEFAULT_PROBE.contains("1415db360:ret"));
        assert!(DEFAULT_PROBE.contains("141b2a280:rdx=0"));
        assert!(DEFAULT_PROBE.starts_with("watch@"));
        assert!(DEFAULT_SESSION.contains("create=on"));
    }

    // -- archiving ----------------------------------------------------------------------

    #[test]
    fn a_previous_hook_log_is_moved_into_previous_runs_and_not_deleted() {
        let t = TempDir::new("archive");
        let root = t.dir("root");
        let log = t.file("client/maplecw-hook.log", "WATCH the previous run");

        let archived = archive_previous_log(&log, &root).expect("archive").expect("some");
        assert!(!log.exists(), "the live log should have moved");
        assert_eq!(std::fs::read_to_string(&archived).unwrap(), "WATCH the previous run");
        assert!(archived.starts_with(root.join("previous-runs")), "{}", archived.display());
        let name = archived.file_name().unwrap().to_string_lossy().to_string();
        assert!(name.starts_with("maplecw-hook-"), "{name}");
        assert!(name.ends_with(".log"), "{name}");
    }

    #[test]
    fn no_previous_log_is_not_an_error() {
        let t = TempDir::new("noarchive");
        let root = t.dir("root");
        let missing = t.path().join("client").join("maplecw-hook.log");
        assert_eq!(archive_previous_log(&missing, &root).unwrap(), None);
    }

    #[test]
    fn a_name_collision_adds_a_counter_rather_than_overwriting_the_older_run() {
        // `tools/test-server.ps1` uses Move-Item -Force here. A run's output is the most
        // expensive data this project produces; two runs in the same second must not cost
        // one of them.
        let t = TempDir::new("collide");
        let root = t.dir("root");
        let first = t.file("client/maplecw-hook.log", "run one");
        let a = archive_previous_log(&first, &root).unwrap().unwrap();

        let second = t.file("client/maplecw-hook.log", "run two");
        // Force the same stamp by copying the first archive's timestamp onto the new file is
        // not portable, so instead assert the general property: whatever names come out, the
        // first archive still holds run one.
        let b = archive_previous_log(&second, &root).unwrap().unwrap();
        assert_eq!(std::fs::read_to_string(&a).unwrap(), "run one");
        assert_eq!(std::fs::read_to_string(&b).unwrap(), "run two");
        assert_ne!(a, b);
    }

    #[test]
    fn the_stamp_is_zero_padded_and_sorts() {
        assert_eq!(format_stamp(2026, 8, 9, 7, 5, 3), "20260809-070503");
        assert_eq!(format_stamp(2026, 12, 31, 23, 59, 59), "20261231-235959");
        assert!(format_stamp(2026, 8, 9, 7, 5, 3) < format_stamp(2026, 8, 9, 7, 5, 4));
    }

    #[test]
    fn local_time_conversion_answers_for_a_known_instant() {
        // 2000-01-01T00:00:00Z. The local answer depends on this machine's time zone, so the
        // assertion is on the shape rather than the value - what is being checked is that the
        // Win32 pair returns at all, because the fallback name would be silently different.
        let got = local_civil_from_unix(946_684_800).expect("FileTimeToSystemTime failed");
        assert!(got.0 == 1999 || got.0 == 2000, "year {}", got.0);
        assert!((1..=12).contains(&got.1), "month {}", got.1);
        assert!((1..=31).contains(&got.2), "day {}", got.2);
        assert!(got.3 < 24 && got.4 < 60 && got.5 < 60, "{got:?}");
    }
}
