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
//! # TLS to a pinned certificate, or nothing
//!
//! This said *"THE PASSWORD CROSSES THE WIRE IN PLAIN TEXT"* until 2026-09-05, as the price of
//! being installable. The owner: *"We should not be sending passwords in plain text."* It is now
//! TLS 1.3, and the launcher accepts exactly one certificate: the one whose fingerprint it was
//! told (`crates/tlspin`). No fingerprint means the password is **not sent** - `sign_in`
//! refuses with a sentence naming the config key - rather than sent in the clear or sent to
//! whoever answers. A certificate that does not match fails the handshake before a byte of
//! the request leaves this machine, and the error says so in those words.
//!
//! What this does not change: the *game* socket still carries no credentials.
//!
//! # Why it is hand-rolled
//!
//! One POST to one endpoint with a small flat JSON reply. A dependency would bring a runtime,
//! a TLS stack and a hundred crates for that, and this workspace declares its Win32 by hand
//! for the same reason. `tiny_http` is the server side of the same trade.

use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use rustls::pki_types::ServerName;
use tlspin::Fingerprint;

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

/// **The credential the CLIENT carries**, as opposed to [`LaunchId`], which only this program
/// ever sends.
///
/// The launcher writes it into `maplecw-hook.identity` beside the client; the hook puts it in
/// the client's own session object at `session+0x1b8`; the client encodes it into `0x0073`
/// with its own code and its own cipher. That makes it the first value in this project that
/// says which launch a connection belongs to **on the wire** rather than by inference about
/// the socket. `store::claims` rule 1b.
///
/// One-time and short-lived: the login server spends it on first presentation and stores only
/// its SHA-256. `Debug` redacts for the same reason `LaunchId` does - a `{:?}` on a struct is
/// how a secret reaches a log without anybody deciding to put it there.
#[derive(Clone, PartialEq, Eq)]
pub struct ClientToken(String);

impl ClientToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    /// True when the service issued none - an older server, or one that could not stake a
    /// claim. The launcher must then write **no** marker at all, and delete any left over from
    /// a previous launch: a dead token is worse than none, because the server's anti-downgrade
    /// rule refuses a connection that presents one instead of falling back to the weaker rules.
    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }
}

impl std::fmt::Debug for ClientToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.is_empty() {
            "ClientToken(<none>)"
        } else {
            "ClientToken(<redacted>)"
        })
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
        /// The token the **client** will carry in `0x0073`. Empty on a server that predates
        /// it, and empty when the claim could not be staked. See [`ClientToken`].
        ///
        /// **The launcher keeps this and hands the same value to every `Start Game`.** That
        /// was always true and used to be the bug: the server spent it on first presentation,
        /// so the second `Start Game` was refused while this screen said the session was good
        /// for twelve hours. Since 2026-09-08 the server honours it until the claim expires.
        client_token: ClientToken,
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
pub fn login(host: &str, port: u16, pin: &Fingerprint, identity: &str, password: &str) -> AuthReply {
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

    let response = match send(host, port, pin, request.as_bytes()) {
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
pub fn bind_launch(
    host: &str,
    port: u16,
    pin: &Fingerprint,
    launch_id: &LaunchId,
    pid: u32,
) -> LaunchReply {
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
    match send(host, port, pin, request.as_bytes()) {
        Ok(r) => parse_launch(&r),
        Err(e) => LaunchReply::Failed(e),
    }
}

/// What `POST /register` said. The Register screen shows [`RegisterReply::message`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterReply {
    Ok { account_id: i64, username: String },
    /// Unknown, expired or already used - one answer for all three, from the service.
    InvalidCode,
    UsernameTaken,
    /// The service refused the input itself - name shape, email shape, password policy - and
    /// this is the sentence it wants shown.
    Refused(String),
    TooManyAttempts,
    /// Unreachable, a non-200 that was not a refusal, or an unreadable body.
    Failed(String),
}

impl RegisterReply {
    pub fn message(&self) -> String {
        match self {
            RegisterReply::Ok { username, .. } => format!(
                "account {username} created. Sign in with it - or with the email you gave - and \
                 press Start Game."
            ),
            RegisterReply::InvalidCode => "that registration code is not valid: unknown, expired, \
                 or already used. Ask the administrator for another one (in game: \
                 !registrationcode)."
                .into(),
            RegisterReply::UsernameTaken => "that username is taken - choose another.".into(),
            RegisterReply::Refused(why) => why.clone(),
            RegisterReply::TooManyAttempts => "too many wrong codes from this address recently. \
                 Wait fifteen minutes and try again."
                .into(),
            RegisterReply::Failed(why) => why.clone(),
        }
    }
}

/// What `POST /recover` said. The Forgot-password screen shows [`RecoverReply::message`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoverReply {
    Ok { username: String },
    /// Unknown, expired, used, minted for another account, or the identity names no account.
    /// The service deliberately does not say which.
    InvalidCode,
    Refused(String),
    TooManyAttempts,
    Failed(String),
}

impl RecoverReply {
    pub fn message(&self) -> String {
        match self {
            RecoverReply::Ok { username } => {
                format!("the password for {username} was changed. Sign in with the new one.")
            }
            RecoverReply::InvalidCode => "that recovery code is not valid for that account: \
                 unknown, expired, already used, minted for a different account - or no \
                 account has that name or email. Ask the administrator for another one (in \
                 game: !recoverycode <email or username>)."
                .into(),
            RecoverReply::Refused(why) => why.clone(),
            RecoverReply::TooManyAttempts => "too many wrong codes from this address recently. \
                 Wait fifteen minutes and try again."
                .into(),
            RecoverReply::Failed(why) => why.clone(),
        }
    }
}

/// `POST /register`: create an account against a registration code.
///
/// The password goes over the same pinned TLS connection the sign-in uses, and nowhere else.
pub fn register(
    host: &str,
    port: u16,
    pin: &Fingerprint,
    username: &str,
    email: &str,
    password: &str,
    code: &str,
) -> RegisterReply {
    let body = format!(
        "{{\"username\":{},\"email\":{},\"password\":{},\"code\":{}}}",
        json_string(username),
        json_string(email),
        json_string(password),
        json_string(code)
    );
    match send(host, port, pin, post_request(host, port, "/register", &body).as_bytes()) {
        Ok(r) => parse_register(&r),
        Err(e) => RegisterReply::Failed(e),
    }
}

/// `POST /recover`: set a new password against a recovery code minted for that account.
pub fn recover(
    host: &str,
    port: u16,
    pin: &Fingerprint,
    identity: &str,
    code: &str,
    new_password: &str,
) -> RecoverReply {
    let body = format!(
        "{{\"identity\":{},\"code\":{},\"new_password\":{}}}",
        json_string(identity),
        json_string(code),
        json_string(new_password)
    );
    match send(host, port, pin, post_request(host, port, "/recover", &body).as_bytes()) {
        Ok(r) => parse_recover(&r),
        Err(e) => RecoverReply::Failed(e),
    }
}

fn post_request(host: &str, port: u16, path: &str, body: &str) -> String {
    format!(
        "POST {path} HTTP/1.1\r\n\
         Host: {host}:{port}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n{body}",
        body.len()
    )
}

/// Split a response into its status code and body, or say why not.
fn status_and_body(response: &str) -> Result<(&str, &str, &str), String> {
    let Some((head, body)) = response.split_once("\r\n\r\n") else {
        return Err(format!("no HTTP body in the answer: {}", trim(response)));
    };
    let status = head.lines().next().unwrap_or("");
    let code = status.split_whitespace().nth(1).unwrap_or("");
    Ok((status, code, body))
}

/// Turn a raw HTTP response into a [`RegisterReply`]. The body's `status` is the verdict; the
/// HTTP status only has to agree with it for `ok`, for the reason [`parse`] gives.
pub fn parse_register(response: &str) -> RegisterReply {
    let (status, code, body) = match status_and_body(response) {
        Ok(parts) => parts,
        Err(e) => return RegisterReply::Failed(e),
    };
    let message = || field(body, "message");
    match field(body, "status").as_deref() {
        Some("ok") if code == "200" => match number(body, "account_id") {
            Some(account_id) => RegisterReply::Ok {
                account_id,
                username: field(body, "username").unwrap_or_default(),
            },
            None => RegisterReply::Failed(format!("no account id in the answer: {}", trim(body))),
        },
        Some("ok") => RegisterReply::Failed(format!("the service answered {status:?} with an ok body")),
        Some("invalid_code") => RegisterReply::InvalidCode,
        Some("username_taken") => RegisterReply::UsernameTaken,
        Some("invalid_username") | Some("invalid_email") | Some("weak_password") => {
            RegisterReply::Refused(message().unwrap_or_else(|| "the service refused the input".into()))
        }
        Some("too_many_attempts") => RegisterReply::TooManyAttempts,
        Some("failed") => RegisterReply::Failed(message().unwrap_or_else(|| "the service refused".into())),
        _ => RegisterReply::Failed(format!("could not read the answer ({status:?}): {}", trim(body))),
    }
}

/// Turn a raw HTTP response into a [`RecoverReply`].
pub fn parse_recover(response: &str) -> RecoverReply {
    let (status, code, body) = match status_and_body(response) {
        Ok(parts) => parts,
        Err(e) => return RecoverReply::Failed(e),
    };
    let message = || field(body, "message");
    match field(body, "status").as_deref() {
        Some("ok") if code == "200" => {
            RecoverReply::Ok { username: field(body, "username").unwrap_or_default() }
        }
        Some("ok") => RecoverReply::Failed(format!("the service answered {status:?} with an ok body")),
        Some("invalid_code") => RecoverReply::InvalidCode,
        Some("weak_password") => {
            RecoverReply::Refused(message().unwrap_or_else(|| "the service refused the password".into()))
        }
        Some("too_many_attempts") => RecoverReply::TooManyAttempts,
        Some("failed") => RecoverReply::Failed(message().unwrap_or_else(|| "the service refused".into())),
        _ => RecoverReply::Failed(format!("could not read the answer ({status:?}): {}", trim(body))),
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

/// Connect, handshake against the pinned certificate, write, read to EOF.
///
/// The handshake runs inside the first write, so a certificate that does not match the pin
/// fails there - before the request, and therefore before the password, has been sent.
fn send(host: &str, port: u16, pin: &Fingerprint, request: &[u8]) -> Result<String, String> {
    let addr = (host, port)
        .to_socket_addrs()
        .map_err(|e| format!("{host}:{port} is not an address this machine can resolve: {e}"))?
        .next()
        .ok_or_else(|| format!("{host}:{port} resolved to nothing"))?;

    let stream = TcpStream::connect_timeout(&addr, TIMEOUT).map_err(|e| {
        format!(
            "could not reach the sign-in service at {host}:{port}: {e}\n\
             Is the server running, and is that the right address? On the server box it is \
             started by start-servers.cmd along with the game servers."
        )
    })?;
    stream.set_read_timeout(Some(TIMEOUT)).ok();
    stream.set_write_timeout(Some(TIMEOUT)).ok();
    stream.set_nodelay(true).ok();

    // The name is irrelevant to a pinned verifier, but rustls needs one; an IP literal is a
    // valid ServerName and is what the field almost always holds.
    let server_name = ServerName::try_from(host.to_string())
        .map_err(|e| format!("{host:?} is not a host name or address TLS can be asked for: {e}"))?;
    let conn = rustls::ClientConnection::new(tlspin::client_config(pin), server_name)
        .map_err(|e| format!("could not start TLS: {e}"))?;
    let mut tls = rustls::StreamOwned::new(conn, stream);

    tls.write_all(request).map_err(|e| explain(host, port, e, "send the sign-in request"))?;
    tls.flush().ok();

    let mut buf = Vec::new();
    match tls.read_to_end(&mut buf) {
        Ok(_) => {}
        // A server that closed the socket without a TLS close_notify. Ours sends one; an
        // answer that arrived is still an answer.
        Err(e) if e.kind() == ErrorKind::UnexpectedEof && !buf.is_empty() => {}
        Err(e) => return Err(explain(host, port, e, "read the answer")),
    }
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

/// A TLS or socket error as a sentence that says what to do. The fingerprint case is the one
/// that matters: it is what an operator sees after regenerating the server's certificate, and
/// also what they would see under an active man-in-the-middle, and it must not read as
/// "network problem".
fn explain(host: &str, port: u16, e: std::io::Error, doing: &str) -> String {
    let text = e.to_string();
    if text.contains("fingerprint") {
        format!(
            "the sign-in service at {host}:{port} presented a certificate that does NOT match \
             the pinned fingerprint ({text}). Either this is not the server you think it is, or \
             its certificate was regenerated - it prints its current fingerprint at startup. \
             NOTHING was sent."
        )
    } else if text.contains("corrupt") || text.contains("InvalidMessage") || text.contains("record") {
        format!(
            "could not {doing}: {text}. What answered is not speaking TLS - an older \
             maplecw-auth still on plain HTTP, or the wrong port?"
        )
    } else {
        format!("could not {doing}: {text}")
    }
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
    // 401 is how the service says "wrong password" or "disabled", with the reason in the
    // body. Until 2026-09-05 this returned `Failed` for anything but 200, so through the
    // real service a wrong password read as "the sign-in service answered 401" rather than
    // as a wrong password - found by the first end-to-end test that sent one. Any other
    // code is still a failure whatever the body says: a 500 whose body happened to contain
    // `"status":"ok"` must not read as a successful login.
    if code != "200" && code != "401" {
        return AuthReply::Failed(format!("the sign-in service answered {status:?}"));
    }

    match field(body, "status").as_deref() {
        Some("ok") if code != "200" => {
            AuthReply::Failed(format!("the sign-in service answered {status:?} with an ok body"))
        }
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
                    // Absent on a server that predates the client token, which must read as
                    // "no token" rather than as a failure: that server signs people in
                    // perfectly well and the launch is resolved by the weaker rules, exactly
                    // as it is today.
                    client_token: ClientToken::new(
                        field(body, "client_token").unwrap_or_default(),
                    ),
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

    /// **End to end, over TLS, against the real sign-in service.** The launcher's client, the
    /// service's own certificate, and the pin between them - the one test here that a socket
    /// takes part in. The wrong pin must fail before the password is sent, which is checked
    /// by the fact that the service never sees a request at all: it answers nothing, and the
    /// error names the fingerprint rather than the password.
    #[test]
    fn a_pinned_client_signs_in_over_tls_and_a_wrong_pin_never_sends_the_password() {
        let dir = std::env::temp_dir().join(format!("maplecw-launcher-tls-e2e-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let identity = auth::tls::ensure_identity(&dir).unwrap();
        let tls = identity.server_config().unwrap();
        let store = store::Store::open_in_memory().unwrap();
        store.create_account("tester", "correct horse battery").unwrap();
        let service = std::sync::Arc::new(auth::AuthService::new(std::sync::Arc::new(store)));
        let listener = auth::http::listen("127.0.0.1", 0).unwrap();
        let port = listener.local_addr().unwrap().port();
        std::thread::spawn(move || {
            let _ = auth::http::run(listener, service, tls);
        });
        let pin = identity.fingerprint;

        // Right pin, wrong password: TLS let the request through and the SERVICE said no.
        assert_eq!(
            login("127.0.0.1", port, &pin, "tester", "wrong"),
            AuthReply::InvalidCredentials
        );
        // Right pin, right password.
        match login("127.0.0.1", port, &pin, "tester", "correct horse battery") {
            AuthReply::Ok { username, client_token, .. } => {
                assert_eq!(username, "tester");
                assert!(!client_token.is_empty(), "a claim was staked and a client token issued");
            }
            other => panic!("expected Ok, got {other:?}"),
        }
        // Wrong pin: refused in the handshake, and the sentence says which fingerprint.
        let wrong = Fingerprint::of_der(b"not the service's certificate");
        match login("127.0.0.1", port, &wrong, "tester", "correct horse battery") {
            AuthReply::Failed(why) => {
                assert!(why.contains("does NOT match the pinned"), "{why}");
                assert!(why.contains("NOTHING was sent"), "{why}");
            }
            other => panic!("expected Failed, got {other:?}"),
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **Registration and recovery, end to end over TLS.** The codes are minted in the store
    /// the way a GM's `!registrationcode` / `!recoverycode` mint them, and redeemed by this
    /// client through the real service. The negative half is what matters: a weak password is
    /// refused BEFORE the code is spent, and a spent code registers nobody else.
    #[test]
    fn a_registration_code_creates_an_account_and_a_recovery_code_resets_its_password() {
        let dir = std::env::temp_dir().join(format!("maplecw-launcher-codes-e2e-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let identity = auth::tls::ensure_identity(&dir).unwrap();
        let tls = identity.server_config().unwrap();
        let store = std::sync::Arc::new(store::Store::open_in_memory().unwrap());
        let service = std::sync::Arc::new(auth::AuthService::new(store.clone()));
        let listener = auth::http::listen("127.0.0.1", 0).unwrap();
        let port = listener.local_addr().unwrap().port();
        std::thread::spawn(move || {
            let _ = auth::http::run(listener, service, tls);
        });
        let pin = identity.fingerprint;
        let h = "127.0.0.1";

        let invite = store.create_invite_code(store::INVITE_TTL_SECS).unwrap().code;
        match register(h, port, &pin, "newbie", "newbie@example.test", "lettersonly", &invite) {
            RegisterReply::Refused(why) => assert!(why.contains("digit"), "{why}"),
            other => panic!("a weak password must be refused first, got {other:?}"),
        }
        match register(h, port, &pin, "newbie", "newbie@example.test", "Passw0rd", &invite) {
            RegisterReply::Ok { username, .. } => assert_eq!(username, "newbie"),
            other => panic!("expected Ok, got {other:?}"),
        }
        assert!(matches!(login(h, port, &pin, "newbie@example.test", "Passw0rd"), AuthReply::Ok { .. }));
        assert_eq!(
            register(h, port, &pin, "someone_else", "x@example.test", "Passw0rd", &invite),
            RegisterReply::InvalidCode,
            "spent"
        );

        let recovery = store.create_recovery_code("newbie@example.test", store::RECOVERY_TTL_SECS).unwrap().code;
        assert_eq!(
            recover(h, port, &pin, "newbie", &recovery, "NewPass99"),
            RecoverReply::Ok { username: "newbie".into() }
        );
        assert_eq!(login(h, port, &pin, "newbie", "Passw0rd"), AuthReply::InvalidCredentials, "old password gone");
        assert!(matches!(login(h, port, &pin, "newbie", "NewPass99"), AuthReply::Ok { .. }));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn register_and_recover_answers_are_read_by_body_status_with_ok_requiring_200() {
        assert_eq!(
            parse_register(&http("200 OK", r#"{"status":"ok","account_id":7,"username":"newbie"}"#)),
            RegisterReply::Ok { account_id: 7, username: "newbie".into() }
        );
        assert_eq!(parse_register(&http("409 Conflict", r#"{"status":"username_taken"}"#)), RegisterReply::UsernameTaken);
        assert_eq!(parse_register(&http("401 Unauthorized", r#"{"status":"invalid_code"}"#)), RegisterReply::InvalidCode);
        assert_eq!(
            parse_register(&http("400 Bad Request", r#"{"status":"weak_password","message":"the password must be longer"}"#)),
            RegisterReply::Refused("the password must be longer".into())
        );
        assert_eq!(parse_register(&http("429 Too Many Requests", r#"{"status":"too_many_attempts"}"#)), RegisterReply::TooManyAttempts);
        assert!(matches!(parse_register(&http("500 Internal Server Error", r#"{"status":"ok","account_id":1}"#)), RegisterReply::Failed(_)));
        assert_eq!(
            parse_recover(&http("200 OK", r#"{"status":"ok","username":"newbie"}"#)),
            RecoverReply::Ok { username: "newbie".into() }
        );
        assert_eq!(parse_recover(&http("401 Unauthorized", r#"{"status":"invalid_code"}"#)), RecoverReply::InvalidCode);
    }

    fn http(status: &str, body: &str) -> String {
        format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\n\r\n{body}")
    }

    #[test]
    fn a_successful_login_is_read() {
        let r = parse(&http(
            "200 OK",
            r#"{"status":"ok","account_id":2,"username":"tester","token":"abc","expires_in":900,"launch_id":"L1","client_token":"MFRGGZDFMZTWQ2LKNNWG23TP2A"}"#,
        ));
        assert_eq!(
            r,
            AuthReply::Ok {
                account_id: 2,
                username: "tester".into(),
                launch_id: LaunchId::new("L1"),
                client_token: ClientToken::new("MFRGGZDFMZTWQ2LKNNWG23TP2A"),
            }
        );
    }

    /// The client token is the one secret that leaves this machine again - it goes into a file
    /// beside the client. It still must not ride out in a `{:?}`.
    #[test]
    fn the_client_token_is_readable_on_purpose_and_redacted_in_debug() {
        let r = parse(&http(
            "200 OK",
            r#"{"status":"ok","account_id":2,"username":"t","token":"abc","expires_in":900,"launch_id":"L","client_token":"CLIENTSECRET"}"#,
        ));
        assert!(!format!("{r:?}").contains("CLIENTSECRET"), "{r:?}");
        let AuthReply::Ok { client_token, .. } = &r else { panic!("{r:?}") };
        assert_eq!(client_token.as_str(), "CLIENTSECRET");
        assert!(!client_token.is_empty());
    }

    /// **A server that predates the client token must still sign people in.** The launcher
    /// then writes no identity marker at all, which leaves the launch resolved by the weaker
    /// rules - today's behaviour exactly.
    #[test]
    fn an_older_server_with_no_client_token_still_signs_in() {
        let r = parse(&http(
            "200 OK",
            r#"{"status":"ok","account_id":2,"username":"t","token":"abc","expires_in":900,"launch_id":"L"}"#,
        ));
        let AuthReply::Ok { client_token, .. } = &r else { panic!("{r:?}") };
        assert!(client_token.is_empty());
        assert_eq!(format!("{client_token:?}"), "ClientToken(<none>)");
    }

    #[test]
    fn the_two_refusals_are_told_apart() {
        // 401, because that is what the service sends with these bodies. This fixture said
        // "200 OK" until 2026-09-05 and passed against a `parse` that refused every 401 -
        // a test that pinned what the code did rather than what the service does. The
        // end-to-end test above is what caught it.
        assert_eq!(
            parse(&http("401 Unauthorized", r#"{"status":"invalid_credentials"}"#)),
            AuthReply::InvalidCredentials
        );
        assert_eq!(parse(&http("401 Unauthorized", r#"{"status":"disabled"}"#)), AuthReply::Disabled);
        // The reason `parse` looks at the status line at all: an ok body under a failure
        // status is not a login.
        let AuthReply::Failed(why) = parse(&http("500 Internal Server Error", r#"{"status":"ok","account_id":1}"#)) else {
            panic!("a 500 must never read as a successful login")
        };
        assert!(why.contains("500"), "{why}");
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
        let pin = Fingerprint::of_der(b"any");
        let r = bind_launch("127.0.0.1", 1, &pin, &LaunchId::new(""), 42);
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

        let pin = Fingerprint::of_der(b"any");
        let r = login("127.0.0.1", port, &pin, "someone", "something");
        let AuthReply::Failed(msg) = r else { panic!("expected a failure") };
        assert!(msg.contains("could not reach"), "{msg}");
        assert!(msg.contains("start-servers.cmd"), "the message must say how to fix it: {msg}");
    }
}
