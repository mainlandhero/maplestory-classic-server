//! The outbound block that keeps the patched client off the internet, applied by the launcher
//! at Start Game rather than once by an installer.
//!
//! The owner, 2026-09-07: *"Could it enforce a firewall rule so that it can only connect to a
//! specific IP that is given by the launcher? Note that the launcher accepts a CNAME too, so
//! it may have to be converted to be an IP before adding to the firewall."*
//!
//! # Why the client needs blocking at all, and why the GameGuard stub is not enough
//!
//! Stubbing `grap64.dll` neutralises GameGuard and nothing else. `MapleStory.exe`'s own import
//! table, read with `tools/pe_import_dlls.py`, names **`nexon_api_x64.dll`, `libcurl.dll`,
//! `crashreporter.dll`, `machineidlib.dll`, `nxoverlay_x64.dll`** and `ws2_32.dll` - all
//! present beside the executable, all loaded by the client itself at startup, none of them
//! ours. Running a modified client with live internet access is how a machine or an account
//! gets flagged.
//!
//! Whether it *does* reach out has never been measured here, because the rule has been on for
//! every run this project has made. The rule is not evidence that it would; it is the reason
//! nobody has had to find out.
//!
//! # Why the launcher and not the installer
//!
//! `tools/installer/install.ps1` still does this, and for a machine that runs both halves it
//! is the right place. For a client it has two problems, and the second is the one the owner
//! named:
//!
//! * **A name is pinned once.** The installer resolves a CNAME at install time and writes that
//!   address into the rule, so its own README says to re-run it if the name later moves. The
//!   launcher resolves the name on **every** launch, so a rule computed here follows the name
//!   by construction.
//! * **A LAN server opens the whole LAN.** The installer's private-address branch allows all
//!   three RFC1918 ranges, on the reasoning that Nexon is not among them. True, and much wider
//!   than "only the server". This module scopes to the resolved addresses and nothing else,
//!   whatever range they are in.
//!
//! # The shape of the rule, and why it is a block rather than an allow
//!
//! Windows evaluates block rules before allow rules, so an allow beside a broad block would
//! never be reached. The block itself has to be narrowed, and `netsh` has no negation - so the
//! remote set is written as **the gaps around the addresses that are allowed**, which is what
//! [`complement`] computes.
//!
//! Loopback is the one exception: Windows Firewall does not filter it, so a rule for a server
//! on this machine is unnecessary rather than wrong, and this says so instead of writing one
//! that does nothing.

use std::net::Ipv4Addr;

/// The rule's name. Shared with `tools/installer/install.ps1` and `tools/firewall.ps1` on
/// purpose: all three manage the same rule, and a second name would leave a stale block behind
/// that nobody would think to look for.
pub const RULE_NAME: &str = "MapleCW - block patched client outbound";

/// What the client is allowed to reach, and therefore what to block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    /// Every allowed address is loopback. Windows does not filter loopback, so a rule would
    /// block nothing that matters and allow nothing that is not already allowed.
    LoopbackOnly,
    /// Block these remote ranges - the gaps around the allowed addresses.
    Block { remote: String, allowed: Vec<Ipv4Addr> },
    /// Nothing to scope to. Refused rather than guessed at: a rule with no allowance would
    /// block the server too, and a rule with no remote set would block the whole internet
    /// including it.
    Unknown(String),
}

/// The complement of `allowed` over the IPv4 space, as `netsh` wants it.
///
/// Ranges are inclusive and comma-separated. The input is sorted and de-duplicated first, so
/// the caller may pass a name's A records in whatever order the resolver gave them.
///
/// This is the whole of the arithmetic and it is pure, because the failure it can have is
/// silent: a complement that is one address wide in the wrong place produces a rule that looks
/// right in `netsh` output and blocks the server.
pub fn complement(allowed: &[Ipv4Addr]) -> String {
    let mut nums: Vec<u32> = allowed.iter().map(|a| u32::from(*a)).collect();
    nums.sort_unstable();
    nums.dedup();

    let mut parts: Vec<String> = Vec::new();
    let mut cursor: u64 = 0; // u64 so the "one past the last address" case cannot wrap
    for n in nums {
        let n = n as u64;
        if n > cursor {
            parts.push(format!(
                "{}-{}",
                Ipv4Addr::from(cursor as u32),
                Ipv4Addr::from((n - 1) as u32)
            ));
        }
        cursor = n + 1;
    }
    if cursor <= u32::MAX as u64 {
        parts.push(format!(
            "{}-{}",
            Ipv4Addr::from(cursor as u32),
            Ipv4Addr::from(u32::MAX)
        ));
    }
    parts.join(",")
}

/// Decide the rule for a set of resolved server addresses.
pub fn scope_for(allowed: &[Ipv4Addr]) -> Scope {
    if allowed.is_empty() {
        return Scope::Unknown(
            "no server address resolved, so there is nothing to allow - the rule is NOT applied \
             rather than blocking the server along with everything else"
                .to_string(),
        );
    }
    if allowed.iter().all(|a| a.is_loopback()) {
        return Scope::LoopbackOnly;
    }
    // A mix of loopback and real addresses keeps the real ones: loopback needs no allowance,
    // so including it in the complement would only carve a hole nothing uses.
    let routable: Vec<Ipv4Addr> = allowed.iter().copied().filter(|a| !a.is_loopback()).collect();
    Scope::Block {
        remote: complement(&routable),
        allowed: routable,
    }
}

/// One sentence for the log pane, so a player can see what was done to their machine.
pub fn describe(scope: &Scope, client_exe: &std::path::Path) -> String {
    match scope {
        Scope::LoopbackOnly => format!(
            "no firewall rule needed: the server is on this machine, and Windows does not \
             filter loopback ({})",
            client_exe.display()
        ),
        Scope::Block { allowed, .. } => {
            let list: Vec<String> = allowed.iter().map(|a| a.to_string()).collect();
            format!(
                "firewall: {} may reach {} and nothing else",
                client_exe.display(),
                list.join(", ")
            )
        }
        Scope::Unknown(why) => format!("firewall NOT applied - {why}"),
    }
}

/// Apply the rule with `netsh`, replacing whatever is there.
///
/// # Why delete-then-add rather than `set rule`
///
/// The same three words `install.ps1` uses, for the same reason: `netsh set rule` edits the
/// first match and leaves any duplicate behind, and a leftover rule from an older server
/// address would keep blocking it. Deleting by name removes every rule with that name.
///
/// # Every failure is a warning and the launch continues
///
/// The client is about to start and the player wants to play. A firewall rule that could not
/// be written is worth a loud line - it means this machine's client can reach the internet -
/// but refusing to launch over it would turn a hardening measure into an outage. The line says
/// what is true so somebody can act on it.
///
/// Returns the sentence to log.
pub fn apply(client_exe: &std::path::Path, allowed: &[Ipv4Addr]) -> Result<String, String> {
    let scope = scope_for(allowed);
    let described = describe(&scope, client_exe);
    let remote = match &scope {
        // Nothing to do, and saying so is the whole result.
        Scope::LoopbackOnly => return Ok(described),
        Scope::Unknown(_) => return Err(described),
        Scope::Block { remote, .. } => remote.clone(),
    };

    // Delete first, ignoring "no rule found" - which is the ordinary case on a first run and
    // is reported by netsh as a non-zero exit, not as an error worth surfacing.
    let _ = std::process::Command::new("netsh")
        .args(["advfirewall", "firewall", "delete", "rule", &format!("name={RULE_NAME}")])
        .output();

    let out = std::process::Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "add",
            "rule",
            &format!("name={RULE_NAME}"),
            "dir=out",
            "action=block",
            &format!("program={}", client_exe.display()),
            "enable=yes",
            "profile=any",
            &format!("remoteip={remote}"),
        ])
        .output()
        .map_err(|e| format!("could not run netsh to add the firewall rule: {e}"))?;

    if !out.status.success() {
        return Err(format!(
            "netsh refused the firewall rule ({}): {}{}. The client can reach the internet \
             until this is fixed - `tools\\firewall.ps1 -Add` from an elevated window does the \
             same job.",
            out.status,
            String::from_utf8_lossy(&out.stdout).trim(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(described)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> Ipv4Addr {
        s.parse().unwrap()
    }

    /// **The gaps around one address, which is the case the owner asked for.**
    #[test]
    fn one_address_leaves_exactly_it_reachable() {
        let got = complement(&[ip("192.168.0.122")]);
        assert_eq!(got, "0.0.0.0-192.168.0.121,192.168.0.123-255.255.255.255");
    }

    /// **Several A records for one name.** A CNAME can round-robin, and pinning the first
    /// answer would block the server on the launches that resolved to a different one.
    #[test]
    fn every_resolved_address_is_allowed_and_the_order_does_not_matter() {
        let a = complement(&[ip("10.0.0.5"), ip("10.0.0.7")]);
        let b = complement(&[ip("10.0.0.7"), ip("10.0.0.5")]);
        assert_eq!(a, b, "the resolver's order must not change the rule");
        assert_eq!(a, "0.0.0.0-10.0.0.4,10.0.0.6-10.0.0.6,10.0.0.8-255.255.255.255");
        // and a duplicate answer is not a second gap
        assert_eq!(complement(&[ip("10.0.0.5"), ip("10.0.0.5")]), complement(&[ip("10.0.0.5")]));
    }

    /// **The edges, where an off-by-one would be invisible.** `0.0.0.0` has no range below it
    /// and `255.255.255.255` none above; getting either wrong writes a rule that parses.
    #[test]
    fn the_first_and_last_addresses_do_not_wrap() {
        assert_eq!(complement(&[ip("0.0.0.0")]), "0.0.0.1-255.255.255.255");
        assert_eq!(complement(&[ip("255.255.255.255")]), "0.0.0.0-255.255.255.254");
        assert_eq!(
            complement(&[ip("0.0.0.0"), ip("255.255.255.255")]),
            "0.0.0.1-255.255.255.254"
        );
    }

    /// **The allowed address is never inside the blocked set.** Checked by walking the ranges
    /// rather than by reading them, because that is the failure that matters: a rule that
    /// blocks the server looks identical to a server that is down.
    #[test]
    fn no_allowed_address_falls_inside_a_blocked_range() {
        for case in [
            vec![ip("192.168.0.122")],
            vec![ip("10.0.0.5"), ip("10.0.0.7")],
            vec![ip("1.2.3.4"), ip("8.8.8.8"), ip("203.0.113.9")],
            vec![ip("0.0.0.0")],
            vec![ip("255.255.255.255")],
        ] {
            let remote = complement(&case);
            for a in &case {
                let n = u32::from(*a);
                for part in remote.split(',').filter(|p| !p.is_empty()) {
                    let (lo, hi) = part.split_once('-').expect(part);
                    let lo = u32::from(lo.parse::<Ipv4Addr>().unwrap());
                    let hi = u32::from(hi.parse::<Ipv4Addr>().unwrap());
                    assert!(lo <= hi, "{part} is inverted");
                    assert!(n < lo || n > hi, "{a} is inside blocked range {part}");
                }
            }
        }
    }

    /// **A LAN server is scoped to the server, not to the LAN.** This is the difference from
    /// `install.ps1`, whose private-address branch allows all three RFC1918 ranges.
    #[test]
    fn a_private_server_does_not_open_the_whole_private_space() {
        let Scope::Block { remote, .. } = scope_for(&[ip("192.168.0.122")]) else {
            panic!("a LAN server must still get a rule");
        };
        // Another machine on the same LAN is blocked.
        let other = u32::from(ip("192.168.0.50"));
        let blocked = remote.split(',').any(|part| {
            let (lo, hi) = part.split_once('-').unwrap();
            let lo = u32::from(lo.parse::<Ipv4Addr>().unwrap());
            let hi = u32::from(hi.parse::<Ipv4Addr>().unwrap());
            other >= lo && other <= hi
        });
        assert!(blocked, "192.168.0.50 should be blocked when only .122 is the server");
    }

    #[test]
    fn loopback_needs_no_rule_and_no_address_refuses_to_write_one() {
        assert_eq!(scope_for(&[ip("127.0.0.1")]), Scope::LoopbackOnly);
        assert!(matches!(scope_for(&[]), Scope::Unknown(_)));
        // Loopback beside a real address keeps the real one and does not carve a hole for
        // something Windows never filters anyway.
        let Scope::Block { allowed, .. } = scope_for(&[ip("127.0.0.1"), ip("10.0.0.5")]) else {
            panic!()
        };
        assert_eq!(allowed, vec![ip("10.0.0.5")]);
    }

    /// **The default must be ON, and only an explicit `off` may turn it off.**
    ///
    /// Losing this rule by forgetting a key would leave a modified client on the internet with
    /// nothing on screen to say so, which is the one failure this whole module exists to
    /// prevent. A misspelled value keeps the rule and reports the line.
    #[test]
    fn the_rule_is_on_unless_the_config_says_off() {
        use crate::config;
        assert_eq!(config::parse("").firewall, None, "absent means the default, which is ON");
        for off in ["firewall = off", "firewall = OFF", "firewall = \"no\"", "firewall = 0"] {
            assert_eq!(config::parse(off).firewall, Some(false), "{off}");
        }
        for on in ["firewall = on", "firewall = yes", "firewall = 1"] {
            assert_eq!(config::parse(on).firewall, Some(true), "{on}");
        }
        // A value that is neither: the rule stays on and the line says why.
        let bad = config::parse("firewall = maybe");
        assert_eq!(bad.firewall, None, "an unreadable value must not disable the rule");
        assert_eq!(bad.problems.len(), 1, "{:?}", bad.problems);
        assert!(bad.problems[0].contains("on or off"), "{:?}", bad.problems);
    }

    #[test]
    fn the_description_says_what_was_done() {
        let exe = std::path::Path::new(r"C:\MapleCW\client\MapleStory.exe");
        let text = describe(&scope_for(&[ip("192.168.0.122")]), exe);
        assert!(text.contains("192.168.0.122"), "{text}");
        assert!(text.contains("nothing else"), "{text}");
        assert!(describe(&Scope::LoopbackOnly, exe).contains("loopback"));
        assert!(describe(&scope_for(&[]), exe).contains("NOT applied"));
    }
}
