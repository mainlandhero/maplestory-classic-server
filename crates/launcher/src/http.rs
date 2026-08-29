//! **A ~100-line HTTP client, because a client machine has no database to read.**
//!
//! The owner, 2026-08-29: *"everyone installs this differently, on a client machine you won't have
//! access to the project or the database."*
//!
//! That sentence retires the launcher's original sign-in. It opened `maplecw.db` directly,
//! which works on the one machine that has it and cannot work anywhere else: an installed
//! client has no repo, no server binaries and no database. The only thing it can reach is the
//! server, so sign-in goes over the wire to `crates/auth`.
//!
//! # One path, not two
//!
//! There is deliberately **no** "use the local file when there is one" shortcut. Two auth
//! paths would mean the machine that gets tested is not the machine that ships - the dev box
//! would exercise the file and every other machine the socket, and the socket would be the
//! one nobody tried. `tools/test-server.ps1` starts `maplecw-auth` alongside the other two
//! servers so the dev flow runs exactly what an installed one does.
//!
//! # THE PASSWORD CROSSES THE WIRE IN PLAIN TEXT
//!
//! Stated here rather than buried. This is plain HTTP: anything on the path between the
//! client machine and the server can read the password. That is a real downgrade from reading
//! a local file, and it is the price of being installable at all.
//!
//! It is acceptable for **a test server on a network you control**, which is the only thing
//! this project is. `docs/deployment.md` records TLS as the work that has to happen before it
//! is anything else, and the auth service still defaults to a loopback bind so going wider is
//! a decision somebody makes rather than one that happens.
//!
//! # Why it is hand-rolled
//!
//! One POST to one endpoint with a small flat JSON reply. A dependency would bring a runtime,
//! a TLS stack and a hundred crates for that, and this workspace declares its Win32 by hand
//! for the same reason. `tiny_http` is the server side of the same trade.

use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// How long to wait for the whole exchange.
///
/// Generous, because argon2id is **deliberately slow** on the server side - that is the point
/// of it - and a timeout tuned for an idle socket would report a working server as
/// unreachable. It is also why the sign-in runs off the UI thread.
const TIMEOUT: Duration = Duration::from_secs(20);

/// **A one-purpose handle, kept out of `Debug`.**
///
/// The service hands this back at sign-in so the launcher can say "the client I just started
/// is process N, and it belongs to my claim". It is not the session token and cannot stand in
/// for one - `store::StakedClaim` has the whole reasoning - but it is still a random secret,
/// and `{:?}` on a struct is how a secret reaches a log without anybody deciding to put it
/// there. So `Debug` redacts and [`LaunchId::as_str`] is the only way out.
#[derive(Clone, PartialEq, Eq)]
pub struct LaunchId(String);

impl LaunchId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    /// True when the service could not stake a claim and sent an empty handle. The launcher
    /// must say so rather than posting it and reading the refusal back.
    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }
}

impl std::fmt::Debug for LaunchId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.is_empty() { "LaunchId(<none>)" } else { "LaunchId(<redacted>)" })
    }
}

/// What the auth service said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthReply {
    Ok {
        account_id: i64,
        username: String,
        /// The handle for [`bind_launch`]. Empty when the service signed the person in but
        /// could not stake a claim - which it reports rather than failing the login, because
        /// the password really was right.
        launch_id: LaunchId,
    },
    InvalidCredentials,
    Disabled,
    /// Anything else: unreachable, a non-200, a body that did not parse.
    Failed(String),
}

/// What `POST /launch` said.
///
/// **None of these is fatal**, and that is the point of having three of them. The client can be
/// started either way; what changes is whether the login server can tell this launch from
/// somebody else's on the same machine. The launcher prints the sentence and carries on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchReply {
    /// The process is bound to this launch's claim.
    Bound { pid: u32 },
    /// The handle named no live claim - stale, expired, or Login was pressed again.
    UnknownLaunch,
    /// Unreachable, a non-200, or a body that did not parse.
    Failed(String),
}

impl LaunchReply {
    /// The line the log pane shows. The two unhappy ones have to say what they cost, because
    /// on a single-player machine nothing goes wrong and the failure only appears months later
    /// when somebody else signs in.
    pub fn message(&self) -> String {
        match self {
            LaunchReply::Bound { pid } => format!(
                "this launch is registered as process {pid} - the server will serve it as your \
                 account even if somebody else is signed in on this machine"
            ),
            LaunchReply::UnknownLaunch => "the server did not recognise this launch (the \
                 sign-in may have expired, or Login was pressed again). The client still \
                 starts; if somebody else is signed in on this machine the game will be \
                 served as the server's fallback account rather than yours - sign in again"
                .to_string(),
            LaunchReply::Failed(why) => format!(
                "this launch could not be registered: {why}. The client still starts; if \
                 somebody else is signed in on this machine the game will be served as the \
                 server's fallback account rather than yours"
            ),
        }
    }
}

/// `POST /login` with an identity and a password.
///
/// The **token is deliberately not returned.** The service stakes the login claim itself, so
/// nothing on this side needs it, and a secret that is never carried cannot be logged by
/// accident. `crates/auth`'s own `login` is where the claim is written.
///
/// The **launch handle is** returned, and it is a different thing: it is what lets this
/// machine say which client process belongs to this sign-in. Without it two launchers on one
/// box are indistinguishable to the server and both fall back.
pub fn login(host: &str, port: u16, identity: &str, password: &str) -> AuthReply {
    let body = format!(
        "{{\"username\":{},\"password\":{}}}",
        json_string(identity),
        json_string(password)
    );
    let request = format!(
        "POST /login HTTP/1.1\r\n\
         Host: {host}:{port}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n{body}",
        body.len()
    );

    let response = match send(host, port, request.as_bytes()) {
        Ok(r) => r,
        Err(e) => return AuthReply::Failed(e),
    };
    parse(&response)
}

/// `POST /launch`: tell the server which process this launch started.
///
/// Called immediately after the client is started, with the pid `launch::launch_for_pid`
/// reported. The server matches it against the process the operating system says owns the
/// game connection, which is the only per-launch fact either end can observe - the client
/// sends nothing that varies between launches, and the address is shared by every client on
/// the machine.
pub fn bind_launch(host: &str, port: u16, launch_id: &LaunchId, pid: u32) -> LaunchReply {
    if launch_id.is_empty() {
        return LaunchReply::Failed(
            "the sign-in did not hand back a launch handle, so the server has no claim to \
             attach this process to"
                .into(),
        );
    }
    let body = format!("{{\"launch_id\":{},\"pid\":{pid}}}", json_string(launch_id.as_str()));
    let request = format!(
        "POST /launch HTTP/1.1\r\n\
         Host: {host}:{port}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n{body}",
        body.len()
    );
    match send(host, port, request.as_bytes()) {
        Ok(r) => parse_launch(&r),
        Err(e) => LaunchReply::Failed(e),
    }
}

/// Turn a raw HTTP response into a [`LaunchReply`]. Status line first, for the same reason
/// [`parse`] checks it first.
pub fn parse_launch(response: &str) -> LaunchReply {
    let Some((head, body)) = response.split_once("\r\n\r\n") else {
        return LaunchReply::Failed(format!("no HTTP body in the answer: {}", trim(response)));
    };
    let status = head.lines().next().unwrap_or("");
    if status.split_whitespace().nth(1).unwrap_or("") != "200" {
        return LaunchReply::Failed(format!("the server answered {status:?}"));
    }
    match field(body, "status").as_deref() {
        Some("bound") => match number(body, "pid") {
            Some(pid) if pid > 0 => LaunchReply::Bound { pid: pid as u32 },
            _ => LaunchReply::Failed(format!("no process id in the answer: {}", trim(body))),
        },
        Some("unknown_launch") => LaunchReply::UnknownLaunch,
        Some("failed") => LaunchReply::Failed(
            field(body, "message").unwrap_or_else(|| "the server refused".into()),
        ),
        _ => LaunchReply::Failed(format!("could not read the answer: {}", trim(body))),
    }
}

/// Connect, write, read to EOF.
fn send(host: &str, port: u16, request: &[u8]) -> Result<String, String> {
    let addr = (host, port)
        .to_socket_addrs()
        .map_err(|e| format!("{host}:{port} is not an address this machine can resolve: {e}"))?
        .next()
        .ok_or_else(|| format!("{host}:{port} resolved to nothing"))?;

    let mut stream = TcpStream::connect_timeout(&addr, TIMEOUT).map_err(|e| {
        format!(
            "could not reach the sign-in service at {host}:{port}: {e}\n\
             Is the server running, and is that the right address? On the server box it is \
             started by start-servers.cmd along with the game servers."
        )
    })?;
    stream.set_read_timeout(Some(TIMEOUT)).ok();
    stream.set_write_timeout(Some(TIMEOUT)).ok();
    stream.write_all(request).map_err(|e| format!("could not send the sign-in request: {e}"))?;
    stream.flush().ok();

    let mut buf = Vec::new();
    stream
        .read_to_end(&mut buf)
        .map_err(|e| format!("the sign-in service closed before answering: {e}"))?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

/// Turn a raw HTTP response into an [`AuthReply`].
///
/// The status line is checked before the body: a 500 whose body happens to contain
/// `"status":"ok"` would otherwise read as a successful login.
pub fn parse(response: &str) -> AuthReply {
    let Some((head, body)) = response.split_once("\r\n\r\n") else {
        return AuthReply::Failed(format!("no HTTP body in the answer: {}", trim(response)));
    };
    let status = head.lines().next().unwrap_or("");
    let code = status.split_whitespace().nth(1).unwrap_or("");
    if code != "200" {
        return AuthReply::Failed(format!("the sign-in service answered {status:?}"));
    }

    match field(body, "status").as_deref() {
        Some("ok") => {
            let account_id = field(body, "account_id")
                .and_then(|v| v.parse::<i64>().ok())
                // `account_id` is a JSON number, so `field` will not find it as a string.
                .or_else(|| number(body, "account_id"));
            match account_id {
                Some(account_id) => AuthReply::Ok {
                    account_id,
                    username: field(body, "username").unwrap_or_default(),
                    // Absent on an older server, or empty when the claim could not be staked.
                    // Both come out as an empty handle, which `bind_launch` refuses with a
                    // sentence rather than posting and reading the refusal back.
                    launch_id: LaunchId::new(field(body, "launch_id").unwrap_or_default()),
                },
                None => AuthReply::Failed(format!("no account id in the answer: {}", trim(body))),
            }
        }
        Some("invalid_credentials") => AuthReply::InvalidCredentials,
        Some("disabled") => AuthReply::Disabled,
        _ => AuthReply::Failed(format!("could not read the answer: {}", trim(body))),
    }
}

/// A `"key":"value"` string field. Enough for these four flat replies and no more.
fn field(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\":\"");
    let start = body.find(&needle)? + needle.len();
    let rest = &body[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// A `"key":123` numeric field.
fn number(body: &str, key: &str) -> Option<i64> {
    let needle = format!("\"{key}\":");
    let start = body.find(&needle)? + needle.len();
    let rest = body[start..].trim_start();
    let end = rest.find(|c: char| !c.is_ascii_digit() && c != '-').unwrap_or(rest.len());
    rest[..end].parse().ok()
}

/// JSON-escape a string. Only what these two fields can legitimately contain.
fn json_string(s: &str) -> String {
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

/// Keep an error message short enough to read in the log pane.
fn trim(s: &str) -> String {
    let s = s.trim();
    if s.len() > 200 {
        format!("{}...", &s[..200])
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn http(status: &str, body: &str) -> String {
        format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\n\r\n{body}")
    }

    #[test]
    fn a_successful_login_is_read() {
        let r = parse(&http(
            "200 OK",
            r#"{"status":"ok","account_id":2,"username":"tester","token":"abc","expires_in":900,"launch_id":"L1"}"#,
        ));
        assert_eq!(
            r,
            AuthReply::Ok {
                account_id: 2,
                username: "tester".into(),
                launch_id: LaunchId::new("L1"),
            }
        );
    }

    #[test]
    fn the_two_refusals_are_told_apart() {
        assert_eq!(
            parse(&http("200 OK", r#"{"status":"invalid_credentials"}"#)),
            AuthReply::InvalidCredentials
        );
        assert_eq!(parse(&http("200 OK", r#"{"status":"disabled"}"#)), AuthReply::Disabled);
    }

    /// **The status line is checked first.** A 500 whose body happens to say `"status":"ok"`
    /// must not read as a successful login - that is a body an error page could plausibly
    /// contain, and treating it as a sign-in would let a broken server log anybody in.
    #[test]
    fn a_non_200_is_a_failure_whatever_the_body_says() {
        let r = parse(&http("500 Internal Server Error", r#"{"status":"ok","account_id":1}"#));
        assert!(matches!(r, AuthReply::Failed(_)), "{r:?}");
    }

    #[test]
    fn a_body_that_is_not_json_fails_rather_than_being_guessed_at() {
        assert!(matches!(parse(&http("200 OK", "<html>nope</html>")), AuthReply::Failed(_)));
        assert!(matches!(parse("not http at all"), AuthReply::Failed(_)));
    }

    #[test]
    fn the_token_never_leaves_this_module() {
        // The service stakes the claim itself, so nothing here needs the token - and a secret
        // that is never carried cannot be logged by accident.
        let r = parse(&http(
            "200 OK",
            r#"{"status":"ok","account_id":2,"username":"t","token":"SECRET","expires_in":900,"launch_id":"HANDLE"}"#,
        ));
        assert!(!format!("{r:?}").contains("SECRET"), "{r:?}");
        // The launch handle DOES leave this module - the launcher needs it - but it must not
        // ride out inside a `{:?}`, which is how a secret reaches a log without anybody
        // deciding to put it there.
        assert!(!format!("{r:?}").contains("HANDLE"), "the launch handle leaked into Debug: {r:?}");
        let AuthReply::Ok { launch_id, .. } = &r else { panic!("{r:?}") };
        assert_eq!(launch_id.as_str(), "HANDLE", "and it is still readable on purpose");
    }

    /// A server that predates `/launch` sends no `launch_id`. The login still succeeds - the
    /// password was right - and the handle is empty, which `bind_launch` refuses with a
    /// sentence rather than a round trip.
    #[test]
    fn an_older_server_with_no_launch_id_still_signs_in() {
        let r = parse(&http(
            "200 OK",
            r#"{"status":"ok","account_id":2,"username":"t","token":"abc","expires_in":900}"#,
        ));
        let AuthReply::Ok { launch_id, .. } = &r else { panic!("{r:?}") };
        assert!(launch_id.is_empty());
        assert_eq!(format!("{launch_id:?}"), "LaunchId(<none>)");
    }

    #[test]
    fn a_bound_launch_is_read() {
        assert_eq!(
            parse_launch(&http("200 OK", r#"{"status":"bound","pid":4242}"#)),
            LaunchReply::Bound { pid: 4242 }
        );
    }

    /// The three unhappy answers are told apart, because they need different sentences and
    /// only one of them has a fix the person can carry out.
    #[test]
    fn the_launch_refusals_are_told_apart() {
        assert_eq!(
            parse_launch(&http("200 OK", r#"{"status":"unknown_launch"}"#)),
            LaunchReply::UnknownLaunch
        );
        assert_eq!(
            parse_launch(&http("200 OK", r#"{"status":"failed","message":"the store said no"}"#)),
            LaunchReply::Failed("the store said no".into())
        );
        assert!(matches!(parse_launch(&http("500 Oops", "")), LaunchReply::Failed(_)));
        assert!(matches!(parse_launch("not http at all"), LaunchReply::Failed(_)));
        // A `bound` with no pid is a malformed answer, not a success with pid 0 - 0 is the
        // System Idle Process and can never own a socket.
        assert!(matches!(
            parse_launch(&http("200 OK", r#"{"status":"bound","pid":0}"#)),
            LaunchReply::Failed(_)
        ));
    }

    /// **Every launch outcome says what it costs.** A registration that quietly failed is
    /// invisible on a single-player machine and only bites when somebody else signs in.
    #[test]
    fn every_launch_outcome_says_what_it_costs() {
        for reply in [
            LaunchReply::UnknownLaunch,
            LaunchReply::Failed("nope".into()),
        ] {
            let msg = reply.message();
            assert!(msg.contains("fallback"), "{reply:?}: {msg}");
            assert!(msg.contains("still starts"), "{reply:?}: {msg}");
        }
        assert!(LaunchReply::Bound { pid: 7 }.message().contains('7'));
    }

    /// An empty handle is refused without a round trip, and the message says why rather than
    /// reporting whatever the server would have said about a blank id.
    #[test]
    fn an_empty_launch_handle_is_refused_before_the_network() {
        let r = bind_launch("127.0.0.1", 1, &LaunchId::new(""), 42);
        let LaunchReply::Failed(why) = r else { panic!("expected a refusal") };
        assert!(why.contains("launch handle"), "{why}");
    }

    #[test]
    fn a_password_with_quotes_and_backslashes_is_escaped() {
        assert_eq!(json_string(r#"a"b\c"#), r#""a\"b\\c""#);
        assert_eq!(json_string("line\nbreak"), r#""line\nbreak""#);
    }

    #[test]
    fn an_unreachable_host_says_where_to_look() {
        // A CLOSED port on loopback, not an unroutable address. Both exercise the same
        // connect failure, but TEST-NET-1 hangs until the 20-second timeout expires and this
        // refuses instantly - a slow test is a tax on every run, and this one was paying it.
        // Re-checked, for the reason `servers::tests::a_closed_port` spells out: a
        // just-released ephemeral port can be taken by another test before this looks.
        let port = (0..50)
            .find_map(|_| {
                let l = std::net::TcpListener::bind("127.0.0.1:0").ok()?;
                let p = l.local_addr().ok()?.port();
                drop(l);
                std::net::TcpStream::connect_timeout(
                    &("127.0.0.1", p).to_socket_addrs().ok()?.next()?,
                    Duration::from_millis(100),
                )
                .is_err()
                .then_some(p)
            })
            .expect("a closed port");

        let r = login("127.0.0.1", port, "someone", "something");
        let AuthReply::Failed(msg) = r else { panic!("expected a failure") };
        assert!(msg.contains("could not reach"), "{msg}");
        assert!(msg.contains("start-servers.cmd"), "the message must say how to fix it: {msg}");
    }
}
