//! The optional `maplecw-launcher.toml` that sits beside the launcher executable.
//!
//! **This is deliberately not TOML.** It is a hand-rolled `key = "value"` reader, and the
//! difference matters in exactly one place: a real TOML basic string treats `\U` in
//! `"C:\Users\user\..."` as an escape and rejects the line. Every value this file carries is
//! a Windows path, so quotes here are *literal delimiters* and nothing inside them is
//! interpreted. Keep it that way; the alternative is a config file whose most obvious
//! contents are a parse error.
//!
//! Accepted:
//!
//! ```text
//! # a comment
//! client_dir = "C:\MapleCW\client"     ; also a comment
//! stub_path  = grap64.dll              # unquoted values work, and are trimmed
//! stub_path  = grap64.dll
//! server_ip  = 192.168.1.20
//! port       = 8484
//! [anything]                           # section headers are skipped, not an error
//! ```
//!
//! Relative paths are resolved against the directory holding the executable - see
//! [`crate::paths`] - so a config file is portable between test machines.

use std::path::{Path, PathBuf};

/// The file name looked for beside the executable.
pub const CONFIG_FILE_NAME: &str = "maplecw-launcher.toml";

/// Every key this reader understands. Anything else is reported rather than ignored, because
/// a typo'd key that silently does nothing is the same failure mode as a stale instrument.
pub const KNOWN_KEYS: &[&str] = &[
    "client_dir",
    "stub_path",
    "server_ip",
    "port",
    "auth_port",
    "auth_fingerprint",
    "identity",
    "firewall",
    "guardpage",
];

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LauncherConfig {
    pub client_dir: Option<String>,
    pub stub_path: Option<String>,
    pub server_ip: Option<String>,
    pub port: Option<u16>,
    pub auth_port: Option<u16>,
    /// The sign-in service's certificate fingerprint, as pasted. Parsed and validated in
    /// `crate::paths`, where a bad value becomes a problem line rather than a silent absence.
    pub auth_fingerprint: Option<String>,
    /// The account name or email to put in the sign-in box. **Never a password** - see
    /// `crate::remembered`, which is what writes this in practice.
    pub identity: Option<String>,
    /// `off` stops the launcher writing the outbound block at Start Game. **Absent means ON**:
    /// the rule is what keeps a modified client off the internet, so it is not something to
    /// lose by forgetting a key. See `crate::firewall`.
    pub firewall: Option<bool>,
    /// `off` stops the launcher putting `guardpage=` in the session marker, so the client runs
    /// with the heap quarantine not merely disabled but **not installed**. **Absent means ON**,
    /// the same rule as `firewall` and for the same reason: the mitigation is what keeps the
    /// client alive for hours instead of minutes, and forgetting a key must not be a way to
    /// lose it. See `crate::client::HOOK_GUARDPAGE_OFF_MARKER` for the per-machine switch that
    /// needs no config file at all.
    pub guardpage: Option<bool>,
    /// Lines that could not be used, with a reason. Surfaced in the UI; never fatal.
    pub problems: Vec<String>,
}

impl LauncherConfig {
    /// Did the file actually set anything?
    ///
    /// **The switches count.** `resolve_from` turns a `true` here into "this file is present
    /// but sets nothing", which is the right complaint about a typo and exactly the wrong one
    /// about a file whose entire contents are `guardpage = "off"` - the shape a support
    /// instruction produces, and the one case where the person reading the line is already
    /// dealing with a client that misbehaves. `firewall` and `identity` were missing here for
    /// the same reason and are now counted too.
    pub fn is_empty(&self) -> bool {
        self.client_dir.is_none()
            && self.stub_path.is_none()
            && self.server_ip.is_none()
            && self.port.is_none()
            && self.auth_port.is_none()
            && self.auth_fingerprint.is_none()
            && self.identity.is_none()
            && self.firewall.is_none()
            && self.guardpage.is_none()
    }
}

/// Read `maplecw-launcher.toml` out of `dir`, if it is there.
///
/// Returns the path it read as well as the parse, so the UI can say *which* file a setting
/// came from - a launcher that silently obeys a config file nobody remembers putting there
/// is the same problem as a hard-coded path, one level of indirection further away.
pub fn load(dir: &Path) -> Option<(PathBuf, LauncherConfig)> {
    let path = dir.join(CONFIG_FILE_NAME);
    let text = std::fs::read_to_string(&path).ok()?;
    Some((path, parse(&text)))
}

/// Parse the text of a config file. Pure; every test drives this.
pub fn parse(text: &str) -> LauncherConfig {
    let mut cfg = LauncherConfig::default();

    for (index, raw) in text.trim_start_matches('\u{feff}').lines().enumerate() {
        let line_no = index + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        // A section header. Harmless: every key here is unique across the whole file, so
        // grouping them under `[paths]` or `[server]` changes nothing.
        if line.starts_with('[') {
            continue;
        }
        let Some((key, rest)) = line.split_once('=') else {
            cfg.problems
                .push(format!("line {line_no}: no `=` in {line:?}, ignored"));
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = unquote(rest.trim());
        if value.is_empty() {
            cfg.problems
                .push(format!("line {line_no}: `{key}` has an empty value, ignored"));
            continue;
        }
        match key.as_str() {
            "client_dir" => cfg.client_dir = Some(value),
            "identity" => cfg.identity = Some(value),
            "firewall" => match value.to_ascii_lowercase().as_str() {
                "off" | "false" | "no" | "0" => cfg.firewall = Some(false),
                "on" | "true" | "yes" | "1" => cfg.firewall = Some(true),
                _ => cfg.problems.push(format!(
                    "line {line_no}: `firewall` takes on or off, not {value:?} - leaving the \
                     rule ON, because that is the safe way to misread it"
                )),
            },
            // Same shape and same default as `firewall`, deliberately: two switches with
            // opposite spellings would be two things to remember under pressure, and this one
            // is reached for exactly when something is going wrong on a machine nobody can see.
            "guardpage" => match value.to_ascii_lowercase().as_str() {
                "off" | "false" | "no" | "0" => cfg.guardpage = Some(false),
                "on" | "true" | "yes" | "1" => cfg.guardpage = Some(true),
                _ => cfg.problems.push(format!(
                    "line {line_no}: `guardpage` takes on or off, not {value:?} - leaving the \
                     heap quarantine ON, because that is the safe way to misread it"
                )),
            },
            // Accepted and IGNORED rather than rejected. Every installer written before
            // 2026-08-29 writes this key, and answering an old config file with "unknown
            // key" would read as the file being wrong when it is merely out of date.
            "db_path" => cfg.problems.push(format!(
                "line {line_no}: db_path is obsolete and was ignored - sign-in goes to the \
                 server over the network now, so this machine needs no database"
            )),
            "stub_path" => cfg.stub_path = Some(value),
            "server_ip" => cfg.server_ip = Some(value),
            "auth_fingerprint" => cfg.auth_fingerprint = Some(value),
            "auth_port" => match value.parse::<u16>() {
                Ok(0) => cfg
                    .problems
                    .push(format!("line {line_no}: auth_port 0 is not a port, ignored")),
                Ok(v) => cfg.auth_port = Some(v),
                Err(_) => cfg.problems.push(format!(
                    "line {line_no}: auth_port {value:?} is not a number 1..=65535, ignored"
                )),
            },
            "port" => match value.parse::<u16>() {
                Ok(0) => cfg
                    .problems
                    .push(format!("line {line_no}: port 0 is not a port, ignored")),
                Ok(p) => cfg.port = Some(p),
                Err(_) => cfg.problems.push(format!(
                    "line {line_no}: port {value:?} is not a number 1..=65535, ignored"
                )),
            },
            _ => cfg.problems.push(format!(
                "line {line_no}: unknown key `{key}` (known: {}), ignored",
                KNOWN_KEYS.join(", ")
            )),
        }
    }
    cfg
}

/// Strip one layer of matching quotes, or take the rest of the line and drop a trailing
/// comment.
///
/// Unquoted values lose anything after a ` #` or ` ;`; quoted values keep every byte between
/// the quotes, which is how a path containing a `#` stays intact.
fn unquote(value: &str) -> String {
    let bytes = value.as_bytes();
    if let Some(&first) = bytes.first() {
        if first == b'"' || first == b'\'' {
            let quote = first as char;
            if let Some(end) = value[1..].find(quote) {
                return value[1..=end].to_string();
            }
            // An opening quote with no close: take the rest, minus the quote.
            return value[1..].to_string();
        }
    }
    let mut cut = value.len();
    for marker in [" #", " ;", "\t#", "\t;"] {
        if let Some(at) = value.find(marker) {
            cut = cut.min(at);
        }
    }
    value[..cut].trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoted_windows_path_survives_intact() {
        // The whole reason this is not a TOML parser: `\U` and `\D` are not escapes here.
        let cfg = parse(r#"client_dir = "C:\MapleCW\client-patched""#);
        assert_eq!(
            cfg.client_dir.as_deref(),
            Some(r"C:\MapleCW\client-patched")
        );
        assert!(cfg.problems.is_empty(), "{:?}", cfg.problems);
    }

    #[test]
    fn unquoted_values_are_trimmed_and_lose_trailing_comments() {
        let cfg = parse("stub_path =   grap64.dll   # the stub\nserver_ip = 10.0.0.5");
        assert_eq!(cfg.stub_path.as_deref(), Some("grap64.dll"));
        assert_eq!(cfg.server_ip.as_deref(), Some("10.0.0.5"));
        assert!(cfg.problems.is_empty(), "{:?}", cfg.problems);
    }

    #[test]
    fn a_hash_inside_quotes_is_part_of_the_path() {
        let cfg = parse(r#"stub_path = "C:\odd #name\grap64.dll""#);
        assert_eq!(cfg.stub_path.as_deref(), Some(r"C:\odd #name\grap64.dll"));
    }

    #[test]
    fn comments_blank_lines_sections_and_crlf() {
        let text = "\u{feff}# header\r\n\r\n[paths]\r\nport = 9000\r\n; trailing\r\n";
        let cfg = parse(text);
        assert_eq!(cfg.port, Some(9000));
        assert!(cfg.problems.is_empty(), "{:?}", cfg.problems);
    }

    #[test]
    fn a_bad_port_is_reported_and_not_silently_defaulted() {
        let cfg = parse("port = eight-four-eight-four");
        assert_eq!(cfg.port, None);
        assert_eq!(cfg.problems.len(), 1);
        assert!(cfg.problems[0].contains("port"), "{:?}", cfg.problems);
    }

    #[test]
    fn port_zero_is_refused() {
        let cfg = parse("port = 0");
        assert_eq!(cfg.port, None);
        assert_eq!(cfg.problems.len(), 1);
    }

    #[test]
    fn an_unknown_key_is_reported_rather_than_ignored() {
        let cfg = parse("clientdir = C:\\x");
        assert!(cfg.is_empty());
        assert_eq!(cfg.problems.len(), 1);
        assert!(cfg.problems[0].contains("unknown key"), "{:?}", cfg.problems);
        // The message has to say what WOULD have worked, or a typo costs a support round trip.
        assert!(cfg.problems[0].contains("client_dir"), "{:?}", cfg.problems);
    }

    #[test]
    fn a_line_with_no_equals_is_reported() {
        let cfg = parse("client_dir C:\\x");
        assert_eq!(cfg.problems.len(), 1);
        assert!(cfg.problems[0].contains("no `=`"), "{:?}", cfg.problems);
    }

    #[test]
    fn keys_are_case_insensitive() {
        let cfg = parse("Client_Dir = client\nPORT = 1234");
        assert_eq!(cfg.client_dir.as_deref(), Some("client"));
        assert_eq!(cfg.port, Some(1234));
    }

    /// **Absent means ON, and a value nobody can read means ON.**
    ///
    /// This key is reached for when a player's client is misbehaving and nobody can see the
    /// machine, so the two failure directions are not symmetric: leaving the quarantine on when
    /// somebody meant to turn it off costs one more round trip, and turning it off because a
    /// value was misread costs the mitigation with nothing on screen to say so.
    #[test]
    fn guardpage_defaults_to_on_and_only_a_readable_off_turns_it_off() {
        assert_eq!(parse("").guardpage, None, "absent is ON, decided by the caller");
        for off in ["off", "OFF", "false", "no", "0"] {
            assert_eq!(parse(&format!("guardpage = {off}")).guardpage, Some(false), "{off}");
        }
        for on in ["on", "true", "yes", "1"] {
            assert_eq!(parse(&format!("guardpage = {on}")).guardpage, Some(true), "{on}");
        }
        let cfg = parse("guardpage = maybe");
        assert_eq!(cfg.guardpage, None, "an unreadable value must not turn it off");
        assert_eq!(cfg.problems.len(), 1);
        assert!(cfg.problems[0].contains("guardpage"), "{:?}", cfg.problems);
    }

    /// A key that is not in `KNOWN_KEYS` is reported by name and the message lists what would
    /// have worked - so a typo'd `guard_page` costs one line, not a support round trip.
    /// A file whose whole contents are the kill switch is not an empty file. Otherwise the UI
    /// answers a support instruction with "present but sets nothing".
    #[test]
    fn a_config_that_only_turns_the_guard_page_off_is_not_reported_as_setting_nothing() {
        let cfg = parse("guardpage = off");
        assert!(!cfg.is_empty());
        assert!(cfg.problems.is_empty(), "{:?}", cfg.problems);
        assert!(!parse("firewall = off").is_empty());
    }

    #[test]
    fn guardpage_is_a_known_key() {
        assert!(KNOWN_KEYS.contains(&"guardpage"));
        let cfg = parse("guard_page = off");
        assert_eq!(cfg.guardpage, None);
        assert!(cfg.problems[0].contains("guardpage"), "{:?}", cfg.problems);
    }

    #[test]
    fn empty_text_yields_an_empty_config_with_no_complaints() {
        let cfg = parse("");
        assert!(cfg.is_empty());
        assert!(cfg.problems.is_empty());
    }
}
