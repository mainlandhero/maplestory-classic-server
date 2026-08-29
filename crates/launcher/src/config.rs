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
pub const KNOWN_KEYS: &[&str] =
    &["client_dir", "stub_path", "server_ip", "port", "auth_port"];

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LauncherConfig {
    pub client_dir: Option<String>,
    pub stub_path: Option<String>,
    pub server_ip: Option<String>,
    pub port: Option<u16>,
    pub auth_port: Option<u16>,
    /// Lines that could not be used, with a reason. Surfaced in the UI; never fatal.
    pub problems: Vec<String>,
}

impl LauncherConfig {
    /// Did the file actually set anything?
    pub fn is_empty(&self) -> bool {
        self.client_dir.is_none()
            && self.stub_path.is_none()
            && self.server_ip.is_none()
            && self.port.is_none()
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
            // Accepted and IGNORED rather than rejected. Every installer written before
            // 2026-08-29 writes this key, and answering an old config file with "unknown
            // key" would read as the file being wrong when it is merely out of date.
            "db_path" => cfg.problems.push(format!(
                "line {line_no}: db_path is obsolete and was ignored - sign-in goes to the \
                 server over the network now, so this machine needs no database"
            )),
            "stub_path" => cfg.stub_path = Some(value),
            "server_ip" => cfg.server_ip = Some(value),
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

    #[test]
    fn empty_text_yields_an_empty_config_with_no_complaints() {
        let cfg = parse("");
        assert!(cfg.is_empty());
        assert!(cfg.problems.is_empty());
    }
}
