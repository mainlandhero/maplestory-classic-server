//! Minimal HTTP/JSON front end.
//!
//! Endpoints:
//!
//! | Method | Path      | Body                            | Purpose |
//! |--------|-----------|---------------------------------|---------|
//! | POST   | `/login`  | `{"username":..,"password":..}` | authenticate, issue a token |
//! | POST   | `/verify` | `{"token":..}`                  | check a token, leaves it valid |
//! | POST   | `/consume`| `{"token":..}`                  | check and spend a token |
//! | GET    | `/health` | —                               | liveness |

use std::sync::Arc;

use serde::Deserialize;
use tiny_http::{Header, Method, Request, Response, Server};

use crate::{AuthService, LoginRequest};

/// Cap on request bodies; these are tiny JSON documents.
const MAX_BODY: usize = 8 * 1024;

#[derive(Debug, Deserialize)]
struct TokenRequest {
    token: String,
}

fn json_response(status: u16, body: String) -> Response<std::io::Cursor<Vec<u8>>> {
    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
        .expect("static header is valid");
    Response::from_string(body)
        .with_status_code(status)
        .with_header(header)
}

fn read_body(req: &mut Request) -> Result<String, String> {
    let len = req.body_length().unwrap_or(0);
    if len > MAX_BODY {
        return Err("body too large".into());
    }
    let mut body = String::new();
    std::io::Read::read_to_string(req.as_reader(), &mut body)
        .map_err(|e| format!("could not read body: {e}"))?;
    Ok(body)
}

/// Serve until the process is stopped. Binds to loopback only.
pub fn serve(service: Arc<AuthService>, port: u16) -> std::io::Result<()> {
    let addr = format!("127.0.0.1:{port}");
    let server = Server::http(&addr).map_err(|e| {
        std::io::Error::other(format!("could not bind {addr}: {e}"))
    })?;
    println!("auth server listening on http://{addr} (loopback only)");

    for mut req in server.incoming_requests() {
        let method = req.method().clone();
        let url = req.url().to_string();
        let path = url.split('?').next().unwrap_or("").to_string();

        let response = match (&method, path.as_str()) {
            (Method::Get, "/health") => json_response(200, r#"{"status":"ok"}"#.into()),

            (Method::Post, "/login") => match read_body(&mut req) {
                Err(e) => json_response(400, error_json(&e)),
                Ok(body) => match serde_json::from_str::<LoginRequest>(&body) {
                    // Never echo the body back: it contains the password.
                    Err(_) => json_response(400, error_json("invalid JSON")),
                    Ok(login) => {
                        let resp = service.login(&login);
                        let code = match resp {
                            crate::LoginResponse::Ok { .. } => 200,
                            _ => 401,
                        };
                        json_response(code, serde_json::to_string(&resp).unwrap_or_default())
                    }
                },
            },

            (Method::Post, "/verify") | (Method::Post, "/consume") => {
                match read_body(&mut req) {
                    Err(e) => json_response(400, error_json(&e)),
                    Ok(body) => match serde_json::from_str::<TokenRequest>(&body) {
                        Err(_) => json_response(400, error_json("invalid JSON")),
                        Ok(tr) => {
                            let resp = if path == "/consume" {
                                service.consume(&tr.token)
                            } else {
                                service.verify(&tr.token)
                            };
                            let code = match resp {
                                crate::VerifyResponse::Ok { .. } => 200,
                                _ => 401,
                            };
                            json_response(
                                code,
                                serde_json::to_string(&resp).unwrap_or_default(),
                            )
                        }
                    },
                }
            }

            _ => json_response(404, error_json("not found")),
        };

        // Log method and path only. Bodies carry passwords and tokens.
        println!("{method} {path}");
        if let Err(e) = req.respond(response) {
            eprintln!("auth: failed to send response: {e}");
        }
    }
    Ok(())
}

fn error_json(message: &str) -> String {
    serde_json::json!({ "status": "error", "message": message }).to_string()
}
