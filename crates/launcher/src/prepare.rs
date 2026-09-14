//! What the **Start Game** button does, in order.
//!
//! Ported from `tools/setup-client.ps1` and the launch block near the end of
//! `tools/test-server.ps1`. The order is not arbitrary: everything that can refuse happens
//! before anything is written, so a refusal leaves the client directory exactly as it was.
//!
//! 1. refuse to touch the original install;
//! 2. check the client executable is there;
//! 2b. check the folder is complete and self-contained - reported, never refused;
//! 3. stub GameGuard (back up once, install the stub, disable `grap\`);
//! 4. make sure the crash-dump directory exists;
//! 5. **archive** the previous `maplecw-hook.log` - never delete it;
//! 6. write the hook's four marker files;
//! 7. write **or delete** the client-credential marker;
//! 8. `ShellExecuteW` the client with `-NXLDEBUG <ip> <port>`.

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
    /// What was typed or configured: a dotted IPv4 address **or a host name**. See
    /// [`Plan::resolved`] - the client is only ever handed the former.
    pub ip: String,
    pub port: u16,
}

impl Plan {
    /// The same plan with `ip` turned into a dotted IPv4 literal, resolving a host name if
    /// that is what was typed.
    ///
    /// The owner, 2026-09-06: *"I would like to give my clients a CNAME and have them DNS resolve
    /// the server IP using that."* The launcher's own connections already take a name -
    /// `to_socket_addrs` resolves it for the sign-in and for the reachability probe, and the
    /// pinned TLS verifier ignores the server name - but the same string also went straight
    /// into `-NXLDEBUG <host> <port>`, and **whether the game client resolves a name is not
    /// known.** Its Winsock imports are rebuilt at runtime by the packer, so `gethostbyname`'s
    /// one static import has no traceable caller, and the `NXLDEBUG` string has no static
    /// reference either: both cross-references came back zero on 2026-09-06, and both zeros
    /// are properties of the packing, not findings. Rather than spend a client launch on it,
    /// the launcher resolves the name itself and the client is always handed a literal - the
    /// one input it has ever been measured accepting.
    ///
    /// IPv4 only. The client's address tables and the login server's advertised hosts are
    /// dotted quads throughout, and a v6 literal on this command line would be a new
    /// experiment; it is refused rather than tried. The first IPv4 answer is taken - a name
    /// with several addresses is a round-robin the sign-in may land on differently, which
    /// matters only if those addresses are different machines.
    pub fn resolved(&self) -> Result<Plan, String> {
        let host = self.ip.trim();
        if host.parse::<std::net::Ipv4Addr>().is_ok() {
            return Ok(Plan { ip: host.to_string(), port: self.port });
        }
        if host.parse::<std::net::IpAddr>().is_ok() {
            return Err(format!(
                "{host} is an IPv6 address, and the client is only known to take IPv4 on its \
                 command line"
            ));
        }
        let addrs = std::net::ToSocketAddrs::to_socket_addrs(&(host, self.port))
            .map_err(|e| format!("{host:?} is not an address and did not resolve as a name: {e}"))?;
        let v4 = addrs
            .filter_map(|a| match a {
                std::net::SocketAddr::V4(v4) => Some(*v4.ip()),
                std::net::SocketAddr::V6(_) => None,
            })
            .next()
            .ok_or_else(|| format!("{host:?} resolved, but to no IPv4 address"))?;
        Ok(Plan { ip: v4.to_string(), port: self.port })
    }
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
///
/// `client_token` is this launch's credential, or `None`. It is the SAME value on every
/// `Start Game` of one sign-in - see `crate::http::AuthReply::Ok::client_token`. **`None` deletes any marker
/// a previous launch left**, which is not tidiness: see [`client::write_identity_marker`].
pub fn prepare(
    layout: &Layout,
    client_token: Option<&str>,
    log: &mut dyn FnMut(Level, String),
) -> Result<(), String> {
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

    // 2b. Is the folder complete, and does it stand on its own?
    //
    // Read-only, and before anything is written, so a folder that cannot work says so while
    // the directory is still untouched. **It reports and does not refuse** - see
    // `crate::integrity`: a false positive here would block a launch on a machine where the
    // client runs, and the client is about to fail loudly by itself anyway. What this adds is
    // a file name next to that failure, in the log the person on the other machine already
    // knows how to send.
    for (level, line) in crate::integrity::check(client_dir).lines() {
        log(level, line);
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
        // `None` is also what comes back when another client still has the log open, which
        // is the ordinary case for a second launch rather than a fault - see
        // `client::archive_previous_log`.
        None => log(
            Level::Info,
            "no previous hook log to archive (or a running client still has it)".into(),
        ),
    }

    // 6. The markers.
    //
    // `layout.guardpage` is the config file's half of the heap-quarantine kill switch; the
    // other half is a marker file beside the client, which `write_markers` checks for itself.
    // Either alone turns it off, and the step it pushes says which - in words a player can
    // read back over chat, because on a machine nobody can see that log line is the only
    // evidence of which way it went.
    for step in client::write_markers(
        client_dir,
        client::DEFAULT_PROBE,
        client::DEFAULT_SESSION,
        &dumps,
        layout.guardpage,
    )? {
        let level = if step.starts_with("WARNING") { Level::Warn } else { Level::Good };
        log(level, step);
    }

    // 7. The credential the client itself will carry, or the removal of the last one. Kept out
    //    of `write_markers` because the other four are fixed strings that say how to debug and
    //    this one is a secret whose absence has to delete a file.
    for step in client::write_identity_marker(client_dir, client_token)? {
        log(Level::Good, step);
    }
    if client_token.is_none() {
        // Not silent. This is the difference between "the client presents a credential" and
        // "the server works out whose socket this is", and on a machine where nobody else is
        // signed in the two look identical - which is exactly how it would go unnoticed.
        log(
            Level::Warn,
            "this launch has NO client credential: the client will send an empty identity in \
             0x0073 and the server will attribute the connection the way it always has, by the \
             process that owns it. Sign in against a server that issues one to change that."
                .into(),
        );
    }

    Ok(())
}

/// Prepare, then start the client, then **register the launch**.
///
/// `launch_id` is the handle the sign-in produced. Registering the client's process id is what
/// makes two launchers on one machine two distinguishable claims: the client sends nothing
/// per-launch (measured - see `store::peerowner`'s module docs) and the address is shared, so
/// the owning process is the only thing left.
///
/// # Registration never fails the launch
///
/// It happens **after** the client is started, and every outcome is a log line rather than an
/// `Err`. The client is already on screen by then, so returning an error would say "the launch
/// failed" about something that plainly did not - and the cost of an unregistered launch is
/// real but narrow: on a machine where nobody else is signed in it changes nothing at all,
/// because the server's sole-claim rule still finds the right account.
///
/// What it must not do is be silent. `crate::http::LaunchReply::message` spells out the cost
/// for each outcome, and `CLAUDE.md`'s standing complaint is exactly about refusals that get
/// captured into a log string and then ignored - so the unhappy ones are logged at `Warn`.
pub fn prepare_and_launch(
    layout: &Layout,
    plan: &Plan,
    launch_id: Option<&crate::http::LaunchId>,
    client_token: Option<&crate::http::ClientToken>,
    log: &mut dyn FnMut(Level, String),
) -> Result<(), String> {
    prepare(layout, client_token.map(|t| t.as_str()), log)?;

    // The address the client will be handed: a literal, resolved here if a name was typed.
    // Before the probe, so the probe and the client agree on which address was checked.
    let typed = plan;
    let resolved = typed.resolved().map_err(|e| format!("{e}\n\n(The client was NOT launched.)"))?;
    let plan = &resolved;
    if plan.ip != typed.ip.trim() {
        log(
            Level::Info,
            format!("{} resolves to {} - the client is given the address, not the name", typed.ip.trim(), plan.ip),
        );
    }

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

    // **The Visual C++ runtime, checked here because nothing else can check it.**
    //
    // `grap64.dll` imports `VCRUNTIME140.dll`, and `MapleStory.exe` statically imports
    // `grap64.dll` - so on a machine without the redistributable the CLIENT dies at startup
    // with a missing-DLL dialog naming a file nobody has heard of. `install.ps1` used to
    // refuse to install for this reason; a client payload no longer ships one, so the check
    // has to live where the launch does.
    //
    // The launcher itself is built with a static CRT for the client payload
    // (`tools/make-installer.ps1 -ClientOnly`), which is what lets this code run at all on
    // such a machine: a dynamically linked launcher would have failed to start before
    // reaching any check of its own.
    if let Err(why) = crate::stub::runtime_present() {
        return Err(format!("{why}\n\n(The client was NOT launched.)"));
    }

    // **The outbound block, scoped to the address this launch actually resolved.**
    //
    // The owner, 2026-09-07: *"Could it enforce a firewall rule so that it can only connect to a
    // specific IP that is given by the launcher?"* Here rather than in `install.ps1` for the
    // reason they named in the same breath: a CNAME is resolved on **every** launch, so a rule
    // computed from `plan.ip` follows the name, where the installer's is pinned to whatever
    // the name meant on the day it ran.
    //
    // Placed after the reachability probe on purpose. The probe is what proves this address is
    // the server; writing a rule around an address that answers nothing would be a firewall
    // change bought with no information, and on a typo it would block the real server.
    //
    // A failure is a WARNING and the launch continues - see `firewall::apply`.
    if layout.firewall {
        match plan.ip.parse::<std::net::Ipv4Addr>() {
            Ok(addr) => match crate::firewall::apply(&layout.client_exe(), &[addr]) {
                Ok(line) => log(Level::Info, line),
                Err(why) => log(Level::Warn, why),
            },
            // `resolved()` guarantees a literal IPv4, so this is unreachable rather than
            // expected - and it says so instead of silently skipping the rule.
            Err(e) => log(
                Level::Warn,
                format!("firewall NOT applied: {:?} is not an IPv4 literal after resolving ({e})", plan.ip),
            ),
        }
    } else {
        log(
            Level::Warn,
            "firewall rule skipped (firewall = \"off\") - this client can reach the internet"
                .into(),
        );
    }

    let args = launch_args(plan);
    log(
        Level::Info,
        format!(
            "launching {} {}",
            layout.client_exe().display(),
            launch::quote_args(&args)
        ),
    );
    let launched = launch::launch_for_pid(&layout.client_exe(), &args, &layout.client_dir)?;
    log(Level::Good, "client started (Windows will ask for elevation)".into());
    log(
        if launched.pid.is_some() { Level::Info } else { Level::Warn },
        launched.describe(),
    );

    register_launch(layout, plan, launch_id, launched.pid, log);

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

/// Tell the server which process this launch started, and say what happened either way.
///
/// Split out so the three "could not register" paths are one place rather than three, and so
/// each of them is a sentence that names the consequence instead of a shrug. Returns nothing:
/// there is no outcome here that should stop a client which is already running.
fn register_launch(
    layout: &Layout,
    plan: &Plan,
    launch_id: Option<&crate::http::LaunchId>,
    pid: Option<u32>,
    log: &mut dyn FnMut(Level, String),
) {
    let Some(launch_id) = launch_id else {
        // No sign-in handle at all. Start Game is gated on a successful sign-in, so this is
        // reachable only from a caller that did not pass one - a programming error, not a
        // user one, and it says so rather than looking like a server problem.
        log(
            Level::Warn,
            "no launch handle was carried from the sign-in, so this launch cannot be \
             registered. The client still starts and will be served correctly unless somebody \
             else is signed in on this machine."
                .into(),
        );
        return;
    };
    let Some(pid) = pid else {
        // Already reported by `Launched::describe`, which says what it costs. Not repeated.
        return;
    };
    let Some(pin) = layout.auth_fingerprint.as_ref() else {
        // Unreachable after a sign-in, which needs the same pin - but the sentence exists
        // because a layout can change between the two.
        log(
            Level::Warn,
            "this launch cannot be registered: no certificate fingerprint is pinned, so nothing \
             is sent to the sign-in service. The client still starts."
                .into(),
        );
        return;
    };
    let reply = crate::http::bind_launch(&plan.ip, layout.auth_port, pin, launch_id, pid);
    let level = match reply {
        crate::http::LaunchReply::Bound { .. } => Level::Good,
        _ => Level::Warn,
    };
    log(level, reply.message());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::{resolve_from, Source};
    use crate::testutil::TempDir;
    use std::path::PathBuf;

    /// **The client is handed a literal, whatever was typed.** A dotted address passes
    /// through trimmed; `localhost` - the one name every machine resolves without a network -
    /// becomes `127.0.0.1`, IPv4 chosen over the `::1` Windows lists first; a v6 literal is
    /// refused rather than tried; and a name in the reserved `.invalid` domain, which no
    /// resolver may answer, is an error that names the host.
    #[test]
    fn a_plan_resolves_a_name_to_a_dotted_address_before_the_client_sees_it() {
        let literal = Plan { ip: " 10.1.2.3 ".into(), port: 8484 };
        assert_eq!(literal.resolved().unwrap(), Plan { ip: "10.1.2.3".into(), port: 8484 });

        let local = Plan { ip: "localhost".into(), port: 8484 }.resolved().unwrap();
        assert_eq!(local, Plan { ip: "127.0.0.1".into(), port: 8484 });
        assert_eq!(launch_args(&local)[1], "127.0.0.1", "and that is what -NXLDEBUG carries");

        let v6 = Plan { ip: "::1".into(), port: 8484 }.resolved().unwrap_err();
        assert!(v6.contains("IPv6"), "{v6}");

        let bad = Plan { ip: "no-such-host.invalid".into(), port: 8484 }.resolved().unwrap_err();
        assert!(bad.contains("no-such-host.invalid"), "{bad}");
    }

    /// 26 characters of uppercase base32, the shape `store::claims` mints.
    const TEST_TOKEN: &str = "MFRGGZDFMZTWQ2LKNNWG23TP2A";

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
        prepare(&layout, Some(TEST_TOKEN), &mut |l, s| lines.push((l, s))).expect("prepare");

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
        // The credential the client will carry, written verbatim and with nothing round it.
        assert_eq!(
            std::fs::read(client.join(client::HOOK_IDENTITY_MARKER)).unwrap(),
            TEST_TOKEN.as_bytes()
        );
        // Nothing was flagged as an error.
        assert!(!lines.iter().any(|(l, _)| *l == Level::Error), "{lines:?}");
        // And the token itself never reaches the log pane.
        assert!(!lines.iter().any(|(_, s)| s.contains(TEST_TOKEN)), "{lines:?}");
    }

    /// **The arm that has to be right.** A launch with no token must leave no marker behind,
    /// because a token from a previous launch is refused by the login server's anti-downgrade
    /// rule rather than ignored - so it would take this launch from "attributed by its owning
    /// process" down to "served as the fallback account".
    #[test]
    fn a_launch_with_no_token_deletes_the_previous_launchs_marker() {
        let t = TempDir::new("prepstale");
        let layout = installed(&t);
        let marker = layout.client_dir.join(client::HOOK_IDENTITY_MARKER);

        let mut lines: Vec<(Level, String)> = Vec::new();
        prepare(&layout, Some(TEST_TOKEN), &mut |l, s| lines.push((l, s))).expect("first");
        assert!(marker.is_file(), "the first launch should have written one");

        lines.clear();
        prepare(&layout, None, &mut |l, s| lines.push((l, s))).expect("second");
        assert!(
            !marker.exists(),
            "a stale credential was left behind; this launch would be REFUSED, not ignored"
        );
        // And it is said out loud, at Warn: on a one-player machine the two states look
        // identical on screen, which is exactly how this would go unnoticed.
        assert!(
            lines.iter().any(|(l, s)| *l == Level::Warn && s.contains("NO client credential")),
            "{lines:?}"
        );
        assert!(lines.iter().any(|(_, s)| s.contains("removed a previous")), "{lines:?}");
    }

    /// A first launch with no token must not fail just because there is nothing to remove.
    #[test]
    fn no_token_and_no_previous_marker_is_not_an_error() {
        let t = TempDir::new("prepnomarker");
        let layout = installed(&t);
        prepare(&layout, None, &mut |_, _| {}).expect("prepare");
        assert!(!layout.client_dir.join(client::HOOK_IDENTITY_MARKER).exists());
    }

    /// An empty string is not a credential, and must be treated as `None` all the way down -
    /// otherwise a server that answered with `"client_token":""` would leave an empty file,
    /// which reads to the hook as a marker that exists and to the person as one that works.
    #[test]
    fn an_empty_token_writes_nothing() {
        let t = TempDir::new("prepempty");
        let layout = installed(&t);
        prepare(&layout, Some("   "), &mut |_, _| {}).expect("prepare");
        assert!(!layout.client_dir.join(client::HOOK_IDENTITY_MARKER).exists());
    }

    /// **The whole point of this change, end to end: a player's Start Game arms the
    /// quarantine, and one file beside the client takes it away again.**
    ///
    /// `prepare` is the only path a player has - `tools/test-server.ps1` is the owner's - so this
    /// asserts on the marker the hook actually reads rather than on any constant, and it
    /// asserts on the log pane too, because on a machine nobody can see that line is the only
    /// evidence of which way the launch went.
    #[test]
    fn a_players_launch_arms_the_guard_page_and_the_off_file_disarms_it() {
        let t = TempDir::new("prepgp");
        let layout = installed(&t);
        let marker = layout.client_dir.join(client::HOOK_SESSION_MARKER);

        let mut lines: Vec<(Level, String)> = Vec::new();
        prepare(&layout, Some(TEST_TOKEN), &mut |l, s| lines.push((l, s))).expect("on");
        assert!(
            std::fs::read_to_string(&marker).unwrap().contains(client::GUARDPAGE_PREFIX),
            "an ordinary launch must arm it; before 2026-09-08 no player's client ever did"
        );
        assert!(
            lines
                .iter()
                .any(|(l, s)| *l == Level::Good && s.contains("heap quarantine ON")),
            "{lines:?}"
        );

        // The kill switch, with no rebuild and no config file.
        std::fs::write(layout.client_dir.join(client::HOOK_GUARDPAGE_OFF_MARKER), "").unwrap();
        lines.clear();
        prepare(&layout, Some(TEST_TOKEN), &mut |l, s| lines.push((l, s))).expect("off");
        assert_eq!(
            std::fs::read_to_string(&marker).unwrap(),
            "mode=2,create=on",
            "off means NO guardpage= term at all - the client the world had before this feature"
        );
        assert!(
            lines
                .iter()
                .any(|(l, s)| *l == Level::Warn && s.contains("is OFF for this launch")),
            "and the log pane says so at Warn: {lines:?}"
        );
    }

    /// The config file's half of the same switch, for a whole install.
    #[test]
    fn guardpage_off_in_the_config_reaches_the_session_marker() {
        let t = TempDir::new("prepgpcfg");
        let mut layout = installed(&t);
        layout.guardpage = false;
        prepare(&layout, None, &mut |_, _| {}).expect("prepare");
        assert_eq!(
            std::fs::read_to_string(layout.client_dir.join(client::HOOK_SESSION_MARKER)).unwrap(),
            "mode=2,create=on"
        );
    }

    #[test]
    fn preparing_twice_leaves_the_real_dll_backed_up() {
        let t = TempDir::new("preptwice");
        let layout = installed(&t);
        let mut sink = |_l: Level, _s: String| {};
        prepare(&layout, Some(TEST_TOKEN), &mut sink).expect("first");
        prepare(&layout, Some(TEST_TOKEN), &mut sink).expect("second");
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
        prepare(&layout, Some(TEST_TOKEN), &mut |l, s| lines.push((l, s))).expect("prepare");

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
        let err = prepare(&layout, Some(TEST_TOKEN), &mut |l, s| lines.push((l, s))).unwrap_err();
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

        let err = prepare(&layout, Some(TEST_TOKEN), &mut |_, _| {}).unwrap_err();
        assert!(err.contains("MapleStory.exe"), "{err}");
        assert!(
            !layout.client_dir.join(client::GRAP_BACKUP).exists(),
            "the backup was taken before the client was checked"
        );
    }
}
