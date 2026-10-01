//! **A Discord webhook, and nothing more.**
//!
//! The owner, 2026-09-30: *"I want to be able to integrate with discord ... This URL should be
//! configurable and only used on the live server instead of the test server. It should display
//! information about current server status and online members in each channel as well as current
//! up time and patch version."* `world::discordstatus` decides what the message says; this crate
//! only carries it: `POST <webhook>?wait=true` to create the message (the reply carries its id),
//! `PATCH <webhook>/messages/<id>` to edit it in place afterwards.
//!
//! # The URL is a credential
//!
//! Anyone holding it can post as the webhook. So it is never logged - [`Webhook::redacted`] is
//! what log lines print - never in the repository (`discord-webhook.txt` is gitignored), and
//! only a process started with an explicit `--discord-webhook-file` reads it.
//!
//! # HTTP/1.1 by hand, deliberately small
//!
//! One request per connection (`Connection: close`), a JSON body, the whole response read to
//! EOF, `Transfer-Encoding: chunked` decoded. Certificates are checked against Mozilla's roots
//! (`webpki-roots`), TLS 1.3 only.

use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

/// A parsed webhook URL.
#[derive(Clone, PartialEq, Eq)]
pub struct Webhook {
    host: String,
    /// `/api/webhooks/<id>/<token>` - no query, no trailing slash.
    path: String,
}

impl std::fmt::Debug for Webhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.redacted())
    }
}

/// What went wrong, in words a log line can carry. Never contains the token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    BadUrl(String),
    Network(String),
    /// A non-2xx answer: the status and at most 200 characters of the body.
    Status(u16, String),
    /// A 2xx answer that did not carry what was asked for.
    Response(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadUrl(s) => write!(f, "not a Discord webhook URL: {s}"),
            Error::Network(s) => write!(f, "network: {s}"),
            Error::Status(c, b) => write!(f, "HTTP {c}: {b}"),
            Error::Response(s) => write!(f, "unexpected response: {s}"),
        }
    }
}

/// An edit's outcome - `Gone` when the message was deleted in Discord and a new one is needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Edited {
    Ok,
    Gone,
}

impl Webhook {
    /// `https://discord.com/api/webhooks/<id>/<token>` (or `discordapp.com`, `ptb.`/`canary.`).
    /// Anything else is refused: this client talks to Discord and nowhere else.
    pub fn parse(url: &str) -> Result<Webhook, Error> {
        let url = url.trim();
        let rest = url.strip_prefix("https://").ok_or_else(|| Error::BadUrl("must start with https://".into()))?;
        let (host, path) = rest.split_once('/').ok_or_else(|| Error::BadUrl("no path".into()))?;
        let allowed = ["discord.com", "discordapp.com", "ptb.discord.com", "canary.discord.com"];
        if !allowed.contains(&host) {
            return Err(Error::BadUrl(format!("host {host:?} is not Discord")));
        }
        let path = path.split(['?', '#']).next().unwrap_or("").trim_end_matches('/');
        let parts: Vec<&str> = path.split('/').collect();
        let ok = parts.len() == 4
            && parts[0] == "api"
            && parts[1] == "webhooks"
            && !parts[2].is_empty()
            && parts[2].bytes().all(|b| b.is_ascii_digit())
            && !parts[3].is_empty()
            && parts[3].bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
        if !ok {
            return Err(Error::BadUrl("expected /api/webhooks/<id>/<token>".into()));
        }
        Ok(Webhook { host: host.to_string(), path: format!("/{path}") })
    }

    /// For log lines: the host and the webhook's id, the token replaced.
    pub fn redacted(&self) -> String {
        let id = self.path.split('/').nth(3).unwrap_or("?");
        format!("https://{}/api/webhooks/{id}/<token hidden>", self.host)
    }

    /// Create a message; returns its id.
    pub fn post(&self, json: &str) -> Result<String, Error> {
        let (status, body) = request(&self.host, "POST", &format!("{}?wait=true", self.path), json)?;
        if !(200..300).contains(&status) {
            return Err(Error::Status(status, snippet(&body)));
        }
        json_string_field(&body, "id").ok_or_else(|| Error::Response(format!("no message id in {}", snippet(&body))))
    }

    /// Replace message `id`'s content.
    pub fn edit(&self, id: &str, json: &str) -> Result<Edited, Error> {
        if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
            return Ok(Edited::Gone);
        }
        let (status, body) = request(&self.host, "PATCH", &format!("{}/messages/{id}", self.path), json)?;
        match status {
            200..=299 => Ok(Edited::Ok),
            404 => Ok(Edited::Gone),
            _ => Err(Error::Status(status, snippet(&body))),
        }
    }
}

fn snippet(body: &str) -> String {
    body.chars().take(200).collect()
}

/// The first `"key":"value"` in a JSON object, value unescaped for the plain ids Discord sends.
pub fn json_string_field(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let at = json.find(&needle)? + needle.len();
    let rest = json[at..].trim_start().strip_prefix(':')?.trim_start().strip_prefix('"')?;
    Some(rest[..rest.find('"')?].to_string())
}

/// A JSON string literal, quotes included.
pub fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn tls_config() -> Arc<rustls::ClientConfig> {
    let roots = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let config = rustls::ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("TLS 1.3 with ring is a valid configuration")
        .with_root_certificates(roots)
        .with_no_client_auth();
    Arc::new(config)
}

/// One HTTPS request, `Connection: close`: `(status, body)`.
fn request(host: &str, method: &str, path: &str, json: &str) -> Result<(u16, String), Error> {
    let net = |e: std::io::Error| Error::Network(e.to_string());
    let addr = (host, 443)
        .to_socket_addrs()
        .map_err(net)?
        .next()
        .ok_or_else(|| Error::Network(format!("{host} did not resolve")))?;
    let tcp = TcpStream::connect_timeout(&addr, Duration::from_secs(10)).map_err(net)?;
    tcp.set_read_timeout(Some(Duration::from_secs(15))).map_err(net)?;
    tcp.set_write_timeout(Some(Duration::from_secs(15))).map_err(net)?;
    let name = rustls_pki_types::ServerName::try_from(host.to_string()).map_err(|e| Error::BadUrl(e.to_string()))?;
    let conn = rustls::ClientConnection::new(tls_config(), name).map_err(|e| Error::Network(e.to_string()))?;
    let mut tls = rustls::StreamOwned::new(conn, tcp);
    let head = format!(
        "{method} {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: MapleCW-status (local server, 1)\r\n\
         Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        json.len()
    );
    tls.write_all(head.as_bytes()).map_err(net)?;
    tls.write_all(json.as_bytes()).map_err(net)?;
    tls.flush().map_err(net)?;
    let mut raw = Vec::new();
    match tls.read_to_end(&mut raw) {
        Ok(_) => {}
        // A server that closes without close_notify: what arrived is the answer.
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof && !raw.is_empty() => {}
        Err(e) => return Err(net(e)),
    }
    parse_response(&raw)
}

/// Split an HTTP/1.1 response into its status and (de-chunked) body.
pub fn parse_response(raw: &[u8]) -> Result<(u16, String), Error> {
    let split = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| Error::Response("no header terminator".into()))?;
    let head = String::from_utf8_lossy(&raw[..split]).to_string();
    let body = &raw[split + 4..];
    let status: u16 = head
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| Error::Response(format!("bad status line: {}", head.lines().next().unwrap_or(""))))?;
    let chunked = head.lines().any(|l| {
        let l = l.to_ascii_lowercase();
        l.starts_with("transfer-encoding:") && l.contains("chunked")
    });
    let body = if chunked { dechunk(body) } else { body.to_vec() };
    Ok((status, String::from_utf8_lossy(&body).to_string()))
}

fn dechunk(mut b: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let Some(eol) = b.windows(2).position(|w| w == b"\r\n") else { break };
        let size_line = String::from_utf8_lossy(&b[..eol]);
        let Ok(size) = usize::from_str_radix(size_line.split(';').next().unwrap_or("").trim(), 16) else { break };
        b = &b[eol + 2..];
        if size == 0 || b.len() < size {
            out.extend_from_slice(&b[..size.min(b.len())]);
            break;
        }
        out.extend_from_slice(&b[..size]);
        b = b.get(size + 2..).unwrap_or(&[]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_discord_webhook_urls_parse_and_the_token_never_prints() {
        let w = Webhook::parse("https://discord.com/api/webhooks/123456/abc_DEF-9").unwrap();
        assert_eq!(w.redacted(), "https://discord.com/api/webhooks/123456/<token hidden>");
        assert!(!format!("{w:?}").contains("abc_DEF"), "Debug is the redacted form");
        assert!(Webhook::parse("https://discord.com/api/webhooks/123456/abc?wait=true").is_ok());
        for bad in [
            "http://discord.com/api/webhooks/1/a",
            "https://evil.example/api/webhooks/1/a",
            "https://discord.com/api/webhooks/1",
            "https://discord.com/api/webhooks/x/a",
            "https://discord.com/api/channels/1/a",
            "https://discord.com/api/webhooks/1/a b",
        ] {
            assert!(Webhook::parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_plain_and_a_chunked_response_both_parse() {
        let plain = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"id\": \"42\", \"type\": 0}";
        let (s, b) = parse_response(plain).unwrap();
        assert_eq!((s, json_string_field(&b, "id").as_deref()), (200, Some("42")));
        let chunked = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n7\r\n{\"id\":\"\r\n4\r\n99\"}\r\n0\r\n\r\n";
        let (s, b) = parse_response(chunked).unwrap();
        assert_eq!((s, b.as_str()), (200, "{\"id\":\"99\"}"));
        let (s, _) = parse_response(b"HTTP/1.1 404 Not Found\r\n\r\n{}").unwrap();
        assert_eq!(s, 404);
    }

    /// **Against the real Discord, posting nothing**: an edit on a webhook that does not exist.
    /// A 401/404 proves the TLS handshake, the root store and the response parse all work.
    /// Network, so ignored by default: `cargo test -p webhook -- --ignored`.
    #[test]
    #[ignore]
    fn the_real_discord_answers_over_tls() {
        let w = Webhook::parse("https://discord.com/api/webhooks/1/not_a_real_token").unwrap();
        match w.edit("1", "{\"content\":\"x\"}") {
            Err(Error::Status(code, _)) => assert!(code == 401 || code == 404 || code == 403, "{code}"),
            Ok(Edited::Gone) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn json_strings_are_escaped() {
        assert_eq!(json_str("a\"b\\c\nd\u{1}"), "\"a\\\"b\\\\c\\nd\\u0001\"");
        assert_eq!(json_str("the owner • ch 1"), "\"the owner • ch 1\"");
    }
}
