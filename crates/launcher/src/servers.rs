//! Is there a server to launch at?
//!
//! The launcher does **not** start the servers. The owner, 2026-08-28: *"I'm happy to start the
//! server by double clicking a powershell script, but the client I would also like to start
//! by double clicking the new login client as administrator."* So the servers stay with
//! `tools/test-server.ps1` (or `start-server.ps1` on an installed machine), and this module
//! exists only to stop the one failure that would otherwise cost a launch.
//!
//! # Why a check at all
//!
//! A client launched at a port nothing is listening on does not say so. It reaches
//! "Connecting..." and stays there, which reads as a broken client or a broken patch, and the
//! only way to find out otherwise is to close it and look at a log. `tools/test-server.ps1`
//! already carries this guard - *"Never launch the client against a dead server"* - and it is
//! there because that mistake has been made.
//!
//! A client run costs the owner a manual launch. One TCP connect is a cheap way not to spend one.
//!
//! # Why the probe is not free, and is worth it anyway
//!
//! Connecting and closing puts a connection in `login.log` that no client made. The login
//! server logs `connection from #N` and then a close with nothing in between, which is a line
//! someone reading a capture has to explain. That is a real cost - this project's logs are
//! its evidence - so the probe happens **once**, immediately before the launch, and the log
//! pane says it happened. The alternative is a phantom connection every few seconds from a
//! poll, which would be much worse.

use std::net::{IpAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

/// How long a single connect attempt may block. Loopback answers in microseconds; this is
/// sized for a LAN server on a slow link, not for the local case.
const PROBE_TIMEOUT: Duration = Duration::from_millis(600);

/// Is something answering there right now?
pub fn is_listening(ip: &str, port: u16) -> bool {
    let Ok(addrs) = (ip, port).to_socket_addrs() else {
        return false;
    };
    for addr in addrs {
        if TcpStream::connect_timeout(&addr, PROBE_TIMEOUT).is_ok() {
            return true;
        }
    }
    false
}

/// Does this address name this machine?
///
/// Used only to pick which sentence to show when nothing answers: a local address means the
/// server script has not been run, a remote one means it has not been run **over there**, and
/// those need different things done about them.
pub fn is_local(ip: &str) -> bool {
    if ip.eq_ignore_ascii_case("localhost") {
        return true;
    }
    match ip.parse::<IpAddr>() {
        Ok(addr) => addr.is_loopback(),
        Err(_) => false,
    }
}

/// `Ok(())` when there is a server to launch at, otherwise the sentence to show.
///
/// `repo_hint` is the directory a dev checkout's `tools\` sits under, when the layout found
/// one - it lets the message name the exact command rather than describing it.
pub fn check(ip: &str, port: u16, repo_hint: Option<&std::path::Path>) -> Result<(), String> {
    if is_listening(ip, port) {
        return Ok(());
    }
    if !is_local(ip) {
        return Err(format!(
            "nothing is answering at {ip}:{port}.\n\
             That address is not this machine, so the servers have to be started ON {ip} - or \
             change the server address back to 127.0.0.1."
        ));
    }
    let how = match repo_hint {
        Some(root) => format!(
            "Start them by double-clicking start-servers.cmd, or run:\n  \
             powershell -ExecutionPolicy Bypass -File \"{}\\tools\\test-server.ps1\" -SetFieldProbe",
            root.display()
        ),
        None => "Start them by double-clicking start-servers.cmd beside this launcher.".to_string(),
    };
    Err(format!(
        "nothing is answering on {ip}:{port}, so there is no server to play on.\n\
         Launching now would leave the client on \"Connecting...\" forever, which looks like a \
         broken client and is not - so it has not been launched.\n\n{how}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::path::Path;

    #[test]
    fn loopback_is_this_machine_and_a_lan_address_is_not() {
        assert!(is_local("127.0.0.1"));
        assert!(is_local("localhost"));
        assert!(is_local("LOCALHOST"));
        assert!(is_local("::1"));
        assert!(!is_local("192.168.1.20"));
        assert!(!is_local("10.0.0.5"));
        assert!(!is_local("not-an-address"));
    }

    /// The positive control. Without it, "nothing is answering" cannot be told apart from
    /// "this function is incapable of seeing a server", which is the failure mode
    /// `CLAUDE.md` records over and over - and here it would block every launch.
    #[test]
    fn a_bound_port_is_seen_and_a_closed_one_is_not() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind an ephemeral port");
        let port = listener.local_addr().unwrap().port();
        assert!(is_listening("127.0.0.1", port), "a bound port must be seen");

        drop(listener);
        assert!(!is_listening("127.0.0.1", port), "a closed port must not be seen");
    }

    #[test]
    fn a_live_server_passes_the_check() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        assert!(check("127.0.0.1", port, None).is_ok());
    }

    #[test]
    fn a_dead_local_port_names_the_command_that_fixes_it() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let err = check("127.0.0.1", port, Some(Path::new("C:\\repo"))).unwrap_err();
        assert!(err.contains("Connecting..."), "{err}");
        assert!(err.contains("test-server.ps1"), "{err}");
        assert!(err.contains("C:\\repo"), "the message should name the actual repo: {err}");
    }

    #[test]
    fn without_a_repo_it_points_at_the_double_click_script() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let err = check("127.0.0.1", port, None).unwrap_err();
        assert!(err.contains("start-servers.cmd"), "{err}");
        assert!(!err.contains("test-server.ps1"), "an installed machine has no tools\\: {err}");
    }

    #[test]
    fn a_dead_remote_port_says_to_start_it_over_there() {
        // TEST-NET-1, guaranteed unroutable, so this cannot accidentally reach something.
        let err = check("192.0.2.1", 8484, Some(Path::new("C:\\repo"))).unwrap_err();
        assert!(err.contains("not this machine"), "{err}");
        assert!(
            !err.contains("test-server.ps1"),
            "telling someone to start a local server for a remote address is wrong: {err}"
        );
    }
}
