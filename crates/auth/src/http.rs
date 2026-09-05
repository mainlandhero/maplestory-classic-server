//! Minimal HTTPS/JSON front end.
//!
//! Endpoints:
//!
//! | Method | Path      | Body                                | Purpose |
//! |--------|-----------|-------------------------------------|---------|
//! | POST   | `/login`  | `{"username":..,"password":..}`     | authenticate, issue a token, stake a per-launch claim |
//! | POST   | `/launch` | `{"launch_id":..,"pid":..}`         | bind the client process to that claim |
//! | POST   | `/verify` | `{"token":..}`                      | check a token, leaves it valid |
//! | POST   | `/consume`| `{"token":..}`                      | check and spend a token |
//! | POST   | `/register` | `{"username","email","password","code"}` | create an account against a registration code (`crate::register`) |
//! | POST   | `/recover`  | `{"identity","code","new_password"}`     | set a new password against a recovery code |
//! | GET    | `/health` | —                                   | liveness |
//!
//! # `/launch` is what makes two clients on one machine work
//!
//! `/login` used to be the whole story, and it was enough only because the claim table held
//! one global row - which is exactly the bug: a second sign-in evicted the first and both
//! login connections were served as the second account. The claim is now per launch, and the
//! launcher registers the process id of the client it started so the login server can tell one
//! launch from another. See `store::claims` and `store::peerowner`.
//!
//! # TLS, and why the HTTP is hand-rolled
//!
//! The owner, 2026-09-05: *"We should not be sending passwords in plain text."* Until then this was
//! `tiny_http` over plain HTTP, stated as the price of being installable. It is now TLS 1.3
//! with the service's own certificate (`crate::tls`), pinned by fingerprint on every client
//! (`crates/tlspin`).
//!
//! `tiny_http` went with the change rather than being kept: its TLS feature pins **rustls
//! 0.20**, which is end-of-life and still carries CVE-2024-32650 - an unfixed infinite loop on
//! a malformed `close_notify`, which is a remote denial of service on exactly the port that
//! is about to be forwarded. Five endpoints with flat JSON bodies do not need an HTTP library;
//! they need a request line, a `Content-Length`, and a refusal for anything else. That is
//! [`read_request`], and it is bounded at every step.
//!
//! The dispatch is [`handle`], a pure function from a parsed request to a response, so every
//! endpoint is unit-tested without a socket - and the socket path is tested once, end to end,
//! through TLS with the real pin.

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;

use crate::{AuthService, LoginRequest, RecoverRequest, RegisterRequest};

/// Cap on the request head (request line plus headers). These are a launcher's few lines.
const MAX_HEAD: usize = 8 * 1024;
/// Cap on request bodies; these are tiny JSON documents.
const MAX_BODY: usize = 8 * 1024;
/// A connection that has not finished its request or read its answer in this long is dropped.
/// A thread per connection is fine at this scale only because nothing can hold one forever.
const IO_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Deserialize)]
struct TokenRequest {
    token: String,
}

/// `POST /launch`. The handle from `/login`, and the process the launcher just started.
#[derive(Debug, Deserialize)]
struct LaunchRequest {
    launch_id: String,
    pid: u32,
}

/// One parsed request. The body is the raw text; each endpoint decodes its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub body: String,
    /// The peer's address, recorded on a login claim. `store::claims` rule 3.
    pub peer: Option<String>,
}

/// One answer. Always JSON, always `Connection: close`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    fn json(status: u16, body: String) -> Self {
        Response { status, body }
    }
    fn error(status: u16, message: &str) -> Self {
        Response::json(status, error_json(message))
    }
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        413 => "Payload Too Large",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        _ => "Error",
    }
}

/// The whole of the service's behaviour, from a request to a response. No I/O.
pub fn handle(service: &AuthService, req: &Request) -> Response {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/health") => Response::json(200, r#"{"status":"ok"}"#.into()),

        ("POST", "/login") => match serde_json::from_str::<LoginRequest>(&req.body) {
            // Never echo the body back: it contains the password.
            Err(_) => Response::error(400, "invalid JSON"),
            Ok(login) => {
                let resp = service.login(&login, req.peer.as_deref());
                let code = match resp {
                    crate::LoginResponse::Ok { .. } => 200,
                    _ => 401,
                };
                Response::json(code, serde_json::to_string(&resp).unwrap_or_default())
            }
        },

        // Bind the client process the launcher started to the claim it signed in with.
        //
        // **Never a 5xx or a hang on a refusal.** A launcher that cannot register its launch
        // still starts the client - the claim is simply resolved by address, or by being the
        // only one - so the answer has to be a value it can print rather than a failure it has
        // to interpret. 200 with a `status` the caller reads, the same shape `/login` uses for
        // a wrong password.
        ("POST", "/launch") => match serde_json::from_str::<LaunchRequest>(&req.body) {
            Err(_) => Response::error(400, "invalid JSON"),
            Ok(l) => {
                let resp = service.bind_launch(&l.launch_id, l.pid);
                Response::json(200, serde_json::to_string(&resp).unwrap_or_default())
            }
        },

        // Registration and recovery: single-use codes a GM minted. The status code carries
        // the verdict class and the body carries the sentence; the launcher reads both.
        ("POST", "/register") => match serde_json::from_str::<RegisterRequest>(&req.body) {
            // Never echo the body: it carries a password and a live code.
            Err(_) => Response::error(400, "invalid JSON"),
            Ok(r) => {
                let resp = service.register(&r, req.peer.as_deref());
                Response::json(resp.http_status(), serde_json::to_string(&resp).unwrap_or_default())
            }
        },
        ("POST", "/recover") => match serde_json::from_str::<RecoverRequest>(&req.body) {
            Err(_) => Response::error(400, "invalid JSON"),
            Ok(r) => {
                let resp = service.recover(&r, req.peer.as_deref());
                Response::json(resp.http_status(), serde_json::to_string(&resp).unwrap_or_default())
            }
        },

        ("POST", "/verify") | ("POST", "/consume") => {
            match serde_json::from_str::<TokenRequest>(&req.body) {
                Err(_) => Response::error(400, "invalid JSON"),
                Ok(tr) => {
                    let resp = if req.path == "/consume" {
                        service.consume(&tr.token)
                    } else {
                        service.verify(&tr.token)
                    };
                    let code = match resp {
                        crate::VerifyResponse::Ok { .. } => 200,
                        _ => 401,
                    };
                    Response::json(code, serde_json::to_string(&resp).unwrap_or_default())
                }
            }
        }

        ("GET", "/login") | ("GET", "/launch") | ("GET", "/verify") | ("GET", "/consume")
        | ("GET", "/register") | ("GET", "/recover") => Response::error(405, "POST"),
        _ => Response::error(404, "not found"),
    }
}

/// Bind the listening socket. Separate from [`run`] so a test can bind port 0 and read the
/// port back before serving.
pub fn listen(bind: &str, port: u16) -> io::Result<TcpListener> {
    let addr = format!("{bind}:{port}");
    TcpListener::bind(&addr).map_err(|e| io::Error::other(format!("could not bind {addr}: {e}")))
}

/// Serve on a chosen interface until the process is stopped. `0.0.0.0` is what an installed
/// server box needs, because the launcher on a client machine has no other way to reach
/// anything; the default in the binary stays loopback so going wider is a decision.
pub fn serve_on(
    service: Arc<AuthService>,
    bind: &str,
    port: u16,
    tls: Arc<rustls::ServerConfig>,
) -> io::Result<()> {
    let listener = listen(bind, port)?;
    println!("auth server listening on https://{bind}:{port} (TLS 1.3, pinned certificate)");
    run(listener, service, tls)
}

/// Accept forever. One thread per connection; every connection is bounded by [`IO_TIMEOUT`]
/// and the two size caps, so a stuck or hostile peer costs one thread for fifteen seconds.
pub fn run(
    listener: TcpListener,
    service: Arc<AuthService>,
    tls: Arc<rustls::ServerConfig>,
) -> io::Result<()> {
    for incoming in listener.incoming() {
        let stream = match incoming {
            Ok(s) => s,
            Err(e) => {
                eprintln!("auth: accept failed: {e}");
                continue;
            }
        };
        let service = service.clone();
        let tls = tls.clone();
        std::thread::spawn(move || {
            let peer = stream.peer_addr().ok();
            if let Err(e) = connection(stream, peer, &service, tls) {
                let who = peer.map(|p| p.to_string()).unwrap_or_else(|| "?".into());
                eprintln!("auth: {who}: {e}");
            }
        });
    }
    Ok(())
}

/// One connection: handshake, one request, one response, close.
fn connection(
    tcp: TcpStream,
    peer: Option<SocketAddr>,
    service: &AuthService,
    tls: Arc<rustls::ServerConfig>,
) -> Result<(), String> {
    tcp.set_read_timeout(Some(IO_TIMEOUT)).ok();
    tcp.set_write_timeout(Some(IO_TIMEOUT)).ok();
    tcp.set_nodelay(true).ok();
    let conn = rustls::ServerConnection::new(tls).map_err(|e| format!("TLS setup: {e}"))?;
    let mut stream = rustls::StreamOwned::new(conn, tcp);

    // The handshake happens inside the first read. A plain-HTTP client - an older launcher,
    // or a browser given http:// - fails here, and the sentence should say that rather than
    // "corrupt message".
    let request = match read_request(&mut stream, peer.map(|p| p.ip().to_string())) {
        Ok(Ok(req)) => req,
        Ok(Err(refusal)) => {
            let line = format!("refused: {}", refusal.body);
            write_response(&mut stream, &refusal)?;
            return Err(line);
        }
        Err(e) => {
            let text = e.to_string();
            return Err(if text.contains("corrupt") || text.contains("InvalidMessage") || text.contains("record") {
                format!("{text} - that is not TLS. An older launcher, or something speaking plain HTTP to the TLS port?")
            } else {
                format!("handshake or read failed: {text}")
            });
        }
    };

    let response = handle(service, &request);
    // Log method, path and status only. Bodies carry passwords and tokens.
    println!("{} {} -> {}", request.method, request.path, response.status);
    write_response(&mut stream, &response)
}

/// Read one HTTP/1.1 request, bounded.
///
/// `Ok(Err(response))` is a request that arrived and was refused - too big, not HTTP - and
/// the refusal is an HTTP response so the client sees a status rather than a dropped socket.
/// `Err` is the connection itself failing.
pub fn read_request<R: Read>(
    r: &mut R,
    peer: Option<String>,
) -> io::Result<Result<Request, Response>> {
    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    let mut chunk = [0u8; 1024];
    let head_end = loop {
        if let Some(pos) = find(&buf, b"\r\n\r\n") {
            break pos;
        }
        if buf.len() > MAX_HEAD {
            return Ok(Err(Response::error(413, "request head too large")));
        }
        let n = r.read(&mut chunk)?;
        if n == 0 {
            return Ok(Err(Response::error(400, "connection closed before the request was complete")));
        }
        buf.extend_from_slice(&chunk[..n]);
    };
    let head = String::from_utf8_lossy(&buf[..head_end]).into_owned();
    let mut lines = head.lines();
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let (Some(method), Some(target), Some(version)) = (parts.next(), parts.next(), parts.next()) else {
        return Ok(Err(Response::error(400, "malformed request line")));
    };
    if !version.starts_with("HTTP/1.") {
        return Ok(Err(Response::error(400, "not HTTP/1.x")));
    }
    let path = target.split('?').next().unwrap_or("").to_string();
    let mut content_length = 0usize;
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            if name.trim().eq_ignore_ascii_case("content-length") {
                content_length = match value.trim().parse() {
                    Ok(n) => n,
                    Err(_) => return Ok(Err(Response::error(400, "bad Content-Length"))),
                };
            }
        }
    }
    if content_length > MAX_BODY {
        return Ok(Err(Response::error(413, "body too large")));
    }
    let mut body = buf[head_end + 4..].to_vec();
    while body.len() < content_length {
        let n = r.read(&mut chunk)?;
        if n == 0 {
            return Ok(Err(Response::error(400, "connection closed before the body was complete")));
        }
        body.extend_from_slice(&chunk[..n]);
    }
    body.truncate(content_length);
    Ok(Ok(Request {
        method: method.to_string(),
        path,
        body: String::from_utf8_lossy(&body).into_owned(),
        peer,
    }))
}

/// Write one response and close the TLS session cleanly, so the client's `read_to_end` ends
/// with EOF rather than an error.
fn write_response<C, T>(stream: &mut rustls::StreamOwned<C, T>, resp: &Response) -> Result<(), String>
where
    C: std::ops::DerefMut + std::ops::Deref<Target = rustls::ConnectionCommon<rustls::server::ServerConnectionData>>,
    T: Read + Write,
{
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        resp.status,
        reason(resp.status),
        resp.body.len()
    );
    stream.write_all(head.as_bytes()).map_err(|e| format!("send: {e}"))?;
    stream.write_all(resp.body.as_bytes()).map_err(|e| format!("send: {e}"))?;
    stream.conn.send_close_notify();
    stream.flush().map_err(|e| format!("close: {e}"))?;
    Ok(())
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

fn error_json(message: &str) -> String {
    serde_json::json!({ "status": "error", "message": message }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use store::Store;

    fn service() -> AuthService {
        let s = Store::open_in_memory().unwrap();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        AuthService::new(Arc::new(s))
    }

    fn req(method: &str, path: &str, body: &str) -> Request {
        Request { method: method.into(), path: path.into(), body: body.into(), peer: Some("127.0.0.1".into()) }
    }

    // ---------------------------------------------------------------- dispatch, no socket

    #[test]
    fn health_is_200_and_unknown_paths_are_404() {
        let svc = service();
        assert_eq!(handle(&svc, &req("GET", "/health", "")).status, 200);
        assert_eq!(handle(&svc, &req("GET", "/nothing", "")).status, 404);
        assert_eq!(handle(&svc, &req("GET", "/login", "")).status, 405);
    }

    #[test]
    fn a_login_body_that_is_not_json_is_400_and_is_never_echoed() {
        let svc = service();
        let r = handle(&svc, &req("POST", "/login", "not json hunter2hunter2"));
        assert_eq!(r.status, 400);
        assert!(!r.body.contains("hunter2"), "{}", r.body);
    }

    #[test]
    fn a_right_password_is_200_with_a_client_token_and_a_wrong_one_is_401() {
        let svc = service();
        let ok = handle(&svc, &req("POST", "/login", r#"{"username":"player_one","password":"hunter2hunter2"}"#));
        assert_eq!(ok.status, 200, "{}", ok.body);
        assert!(ok.body.contains(r#""status":"ok""#), "{}", ok.body);
        assert!(ok.body.contains("client_token"), "{}", ok.body);
        let bad = handle(&svc, &req("POST", "/login", r#"{"username":"player_one","password":"nope"}"#));
        assert_eq!(bad.status, 401);
    }

    #[test]
    fn an_unknown_launch_is_200_with_a_status_never_a_failure() {
        let svc = service();
        let r = handle(&svc, &req("POST", "/launch", r#"{"launch_id":"stale","pid":4242}"#));
        assert_eq!(r.status, 200);
        assert!(r.body.contains("unknown_launch"), "{}", r.body);
    }

    // ---------------------------------------------------------------- the parser

    #[test]
    fn a_request_is_read_with_its_body_and_query_strings_are_dropped_from_the_path() {
        let raw = b"POST /login?x=1 HTTP/1.1\r\nHost: h\r\ncontent-length: 5\r\n\r\nhello".to_vec();
        let r = read_request(&mut raw.as_slice(), None).unwrap().unwrap();
        assert_eq!(r.method, "POST");
        assert_eq!(r.path, "/login");
        assert_eq!(r.body, "hello");
    }

    #[test]
    fn an_oversized_body_is_refused_with_413_before_it_is_read() {
        let raw = format!("POST /login HTTP/1.1\r\nContent-Length: {}\r\n\r\n", MAX_BODY + 1).into_bytes();
        let r = read_request(&mut raw.as_slice(), None).unwrap().unwrap_err();
        assert_eq!(r.status, 413);
    }

    #[test]
    fn a_head_that_never_ends_is_refused_and_a_non_http_line_is_400() {
        let raw = vec![b'A'; MAX_HEAD + 10];
        assert_eq!(read_request(&mut raw.as_slice(), None).unwrap().unwrap_err().status, 413);
        let raw = b"\x16\x03\x01\x02\x00\r\n\r\n".to_vec();
        assert_eq!(read_request(&mut raw.as_slice(), None).unwrap().unwrap_err().status, 400);
        let raw = b"GET /health\r\n\r\n".to_vec();
        assert_eq!(read_request(&mut raw.as_slice(), None).unwrap().unwrap_err().status, 400);
    }

    // ---------------------------------------------------------------- end to end, over TLS

    /// A rustls client pinned to `pin`, doing one GET and returning the raw response.
    fn https_get(port: u16, pin: &tlspin::Fingerprint, path: &str) -> Result<String, String> {
        let tcp = TcpStream::connect(("127.0.0.1", port)).map_err(|e| e.to_string())?;
        tcp.set_read_timeout(Some(Duration::from_secs(10))).ok();
        let name = rustls::pki_types::ServerName::try_from("127.0.0.1").unwrap();
        let conn = rustls::ClientConnection::new(tlspin::client_config(pin), name).map_err(|e| e.to_string())?;
        let mut s = rustls::StreamOwned::new(conn, tcp);
        s.write_all(format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n").as_bytes())
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        s.read_to_end(&mut out).map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&out).into_owned())
    }

    #[test]
    fn the_real_server_answers_a_pinned_client_over_tls_and_refuses_an_unpinned_one() {
        let dir = std::env::temp_dir().join(format!("maplecw-auth-http-e2e-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let identity = crate::tls::ensure_identity(&dir).unwrap();
        let tls = identity.server_config().unwrap();
        let listener = listen("127.0.0.1", 0).unwrap();
        let port = listener.local_addr().unwrap().port();
        let svc = Arc::new(service());
        std::thread::spawn(move || {
            let _ = run(listener, svc, tls);
        });

        let good = https_get(port, &identity.fingerprint, "/health").unwrap();
        assert!(good.starts_with("HTTP/1.1 200 OK"), "{good}");
        assert!(good.contains(r#"{"status":"ok"}"#), "{good}");

        // The wrong pin fails the handshake: no request is ever sent.
        let wrong = tlspin::Fingerprint::of_der(b"some other certificate");
        let err = https_get(port, &wrong, "/health").unwrap_err();
        assert!(err.contains("fingerprint"), "{err}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
