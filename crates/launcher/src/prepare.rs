//! What the **Start Game** button does, in order.
//!
//! Ported from `tools/setup-client.ps1` and the launch block near the end of
//! `tools/test-server.ps1`. The order is not arbitrary: everything that can refuse happens
//! before anything is written, so a refusal leaves the client directory exactly as it was.
//!
//! 1. refuse to touch the original install;
//! 2. check the client executable is there;
//! 3. stub GameGuard (back up once, install the stub, disable `grap\`);
//! 4. make sure the crash-dump directory exists;
//! 5. **archive** the previous `maplecw-hook.log` - never delete it;
//! 6. write the hook's four marker files;
//! 7. `ShellExecuteW` the client with `-NXLDEBUG <ip> <port>`.

use std::path::Path;

use crate::client;
use crate::launch;
use crate::paths::Layout;
use crate::servers;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    Info,
    Good,
    Warn,
    Error,
}

/// The two things the user chooses on the launch line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    pub ip: String,
    pub port: u16,
}

/// The client's command line.
///
/// **Three arguments, and no more.** `docs/launcher.md` records the measurement: launch
/// arguments from the third onward do land in the client's config session array at `+0x90`,
/// and nothing puts them on the wire - six distinguishable tokens produced a byte-identical
/// outbound `0x0073`. Passing them *also* broke the run with a "trouble connecting" dialog
/// from a path we do not suppress. So a session token here would be inert at best and fatal
/// at worst.
pub fn launch_args(plan: &Plan) -> Vec<String> {
    vec!["-NXLDEBUG".to_string(), plan.ip.clone(), plan.port.to_string()]
}

/// Everything except starting the client. Split out so it can be tested against a temp
/// directory without launching anything.
pub fn prepare(layout: &Layout, log: &mut dyn FnMut(Level, String)) -> Result<(), String> {
    let client_dir = &layout.client_dir;

    log(Level::Info, format!("paths from: {}", layout.source.label()));
    if let Some(cfg) = &layout.config_file {
        log(Level::Info, format!("config: {}", cfg.display()));
    }
    log(Level::Info, format!("client:  {}", client_dir.display()));
    log(Level::Info, format!("stub:    {}", layout.stub_path.display()));
    log(Level::Info, format!("output:  {}", layout.data_root.display()));

    // 1. The standing constraint: `client-patched\` exists so the original client is never
    //    touched. Checked before a single byte is written.
    client::refuse_original_install(client_dir, Path::new(client::ORIGINAL_INSTALL))?;

    // 2. A missing client executable would otherwise only show up at step 7, after the
    //    directory had already been modified.
    let exe = layout.client_exe();
    if !exe.is_file() {
        return Err(format!("no {} at {}", crate::paths::CLIENT_EXE_NAME, exe.display()));
    }

    // 3. GameGuard. The stub is a file on disk when there is one, and otherwise the copy
    //    compiled into this executable - which is what lets a single .exe patch a stock
    //    client on a machine that has nothing else on it. `crate::stub`.
    let stub = crate::stub::resolve(&layout.stub_path, client_dir)?;
    log(Level::Info, stub.describe());
    for step in client::stub_gameguard(client_dir, stub.path())? {
        log(Level::Good, step);
    }

    // 4. Where a crash dump goes. The hook writes its own, because Windows Error Reporting
    //    never sees this client's faults - it ships its own crash reporting and never reaches
    //    WerFault. Measured, not assumed; see `CLAUDE.md`.
    let dumps = layout.dumps_dir();
    std::fs::create_dir_all(&dumps)
        .map_err(|e| format!("could not create {}: {e}", dumps.display()))?;

    // 5. Archive the previous run. NEVER delete: a client run costs a manual launch, and the
    //    hook log is the only record of what the CLIENT did with a packet.
    let hook_log = client_dir.join(client::HOOK_LOG);
    match client::archive_previous_log(&hook_log, &layout.data_root)? {
        Some(archived) => log(
            Level::Good,
            format!("previous hook log archived -> {}", archived.display()),
        ),
        None => log(Level::Info, "no previous hook log to archive".into()),
    }

    // 6. The markers.
    for step in client::write_markers(
        client_dir,
        client::DEFAULT_PROBE,
        client::DEFAULT_SESSION,
        &dumps,
    )? {
        let level = if step.starts_with("WARNING") { Level::Warn } else { Level::Good };
        log(level, step);
    }

    Ok(())
}

/// Prepare, then start the client.
pub fn prepare_and_launch(
    layout: &Layout,
    plan: &Plan,
    log: &mut dyn FnMut(Level, String),
) -> Result<(), String> {
    prepare(layout, log)?;

    // BEFORE the client, and after everything else: a client launched at a dead port sits on
    // "Connecting..." forever and reads as a broken client. `tools/test-server.ps1` carries
    // the same guard for the same reason. One TCP connect is cheap; a manual launch is not.
    //
    // The probe does put one connection in `login.log` that no client made - a `connection
    // from #N` with a close and nothing between. That is a real cost in a project whose logs
    // are its evidence, so it happens exactly once, here, and is announced.
    log(
        Level::Info,
        format!("checking for a server on {}:{}", plan.ip, plan.port),
    );
    servers::check(&plan.ip, plan.port, layout.repo_root().as_deref()).map_err(|e| {
        // Not a warning that scrolls past. Returning Err stops the launch, which is the whole
        // point: the client is not started, so nothing has been spent.
        format!("{e}\n\n(The client was NOT launched.)")
    })?;
    log(Level::Good, "a server is answering".into());

    let args = launch_args(plan);
    log(
        Level::Info,
        format!(
            "launching {} {}",
            layout.client_exe().display(),
            launch::quote_args(&args)
        ),
    );
    launch::launch(&layout.client_exe(), &args, &layout.client_dir)?;
    log(Level::Good, "client started (Windows will ask for elevation)".into());
    log(
        Level::Info,
        format!(
            "if it dies, look in {} and {}",
            layout.client_dir.join(client::HOOK_LOG).display(),
            layout.dumps_dir().display()
        ),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::{resolve_from, Source};
    use crate::testutil::TempDir;
    use std::path::PathBuf;

    /// An installed layout with a fake client in it.
    fn installed(t: &TempDir) -> Layout {
        t.file("app/client/MapleStory.exe", "client");
        t.file("app/client/grap64.dll", "THE REAL GAMEGUARD DLL");
        t.dir("app/client/grap");
        t.file("app/maplecw.db", "db");
        t.file("app/grap64.dll", &"S".repeat(4096));
        resolve_from(&t.path().join("app"))
    }

    #[test]
    fn the_client_command_line_is_exactly_three_arguments() {
        let plan = Plan { ip: "127.0.0.1".into(), port: 8484 };
        assert_eq!(launch_args(&plan), vec!["-NXLDEBUG", "127.0.0.1", "8484"]);
    }

    #[test]
    fn no_session_token_is_ever_appended() {
        // docs/launcher.md: tokens reach the client's config array and never reach the wire,
        // AND passing them broke the run with a "trouble connecting" dialog.
        let plan = Plan { ip: "10.0.0.2".into(), port: 9999 };
        assert_eq!(launch_args(&plan).len(), 3);
    }

    #[test]
    fn preparing_an_installed_layout_does_every_step() {
        let t = TempDir::new("prep");
        let layout = installed(&t);
        assert_eq!(layout.source, Source::Installed);

        let mut lines: Vec<(Level, String)> = Vec::new();
        prepare(&layout, &mut |l, s| lines.push((l, s))).expect("prepare");

        let client = &layout.client_dir;
        // GameGuard.
        assert_eq!(
            std::fs::read_to_string(client.join(client::GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
        assert_eq!(std::fs::read(client.join(client::GRAP_DLL)).unwrap().len(), 4096);
        assert!(client.join(client::GRAP_DIR_DISABLED).is_dir());
        // Markers.
        for name in [
            client::HOOK_ENABLE_MARKER,
            client::HOOK_PROBE_MARKER,
            client::HOOK_SESSION_MARKER,
            client::HOOK_DUMPDIR_MARKER,
        ] {
            assert!(client.join(name).is_file(), "{name} was not written");
        }
        assert_eq!(
            std::fs::read_to_string(client.join(client::HOOK_DUMPDIR_MARKER)).unwrap(),
            layout.dumps_dir().to_string_lossy()
        );
        // Dump directory.
        assert!(layout.dumps_dir().is_dir());
        // Nothing was flagged as an error.
        assert!(!lines.iter().any(|(l, _)| *l == Level::Error), "{lines:?}");
    }

    #[test]
    fn preparing_twice_leaves_the_real_dll_backed_up() {
        let t = TempDir::new("preptwice");
        let layout = installed(&t);
        let mut sink = |_l: Level, _s: String| {};
        prepare(&layout, &mut sink).expect("first");
        prepare(&layout, &mut sink).expect("second");
        assert_eq!(
            std::fs::read_to_string(layout.client_dir.join(client::GRAP_BACKUP)).unwrap(),
            "THE REAL GAMEGUARD DLL"
        );
    }

    #[test]
    fn a_hook_log_from_the_previous_run_is_archived_not_deleted() {
        let t = TempDir::new("preparchive");
        let layout = installed(&t);
        let hook_log = layout.client_dir.join(client::HOOK_LOG);
        std::fs::write(&hook_log, "CLIENT FAULT 0xC0000005").unwrap();

        let mut lines: Vec<(Level, String)> = Vec::new();
        prepare(&layout, &mut |l, s| lines.push((l, s))).expect("prepare");

        assert!(!hook_log.exists(), "the live hook log should have been moved");
        let archived: Vec<PathBuf> = std::fs::read_dir(layout.previous_runs_dir())
            .expect("previous-runs")
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        assert_eq!(archived.len(), 1, "{archived:?}");
        assert_eq!(
            std::fs::read_to_string(&archived[0]).unwrap(),
            "CLIENT FAULT 0xC0000005"
        );
        assert!(
            lines.iter().any(|(_, s)| s.contains("archived")),
            "the archive should be reported: {lines:?}"
        );
    }

    #[test]
    fn the_original_install_is_refused_before_anything_is_written() {
        let t = TempDir::new("prepguard");
        let mut layout = installed(&t);
        layout.client_dir = PathBuf::from(client::ORIGINAL_INSTALL);

        let mut lines: Vec<(Level, String)> = Vec::new();
        let err = prepare(&layout, &mut |l, s| lines.push((l, s))).unwrap_err();
        assert!(err.contains("refusing to touch the original install"), "{err}");
        // The dump directory is created at step 4, after the guard: nothing should exist.
        assert!(!layout.dumps_dir().exists(), "prepare wrote something before refusing");
    }

    #[test]
    fn a_missing_client_executable_stops_before_the_backup() {
        let t = TempDir::new("prepnoexe");
        t.file("app/client/grap64.dll", "THE REAL GAMEGUARD DLL");
        t.file("app/grap64.dll", &"S".repeat(4096));
        let layout = resolve_from(&t.path().join("app"));

        let err = prepare(&layout, &mut |_, _| {}).unwrap_err();
        assert!(err.contains("MapleStory.exe"), "{err}");
        assert!(
            !layout.client_dir.join(client::GRAP_BACKUP).exists(),
            "the backup was taken before the client was checked"
        );
    }
}
