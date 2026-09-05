//! The local authentication server.
//!
//!   maplecw-auth [--db <path>] [--bind <addr>] [--port <port>]
//!
//! `--bind 0.0.0.0` is what an installed server box needs: the launcher on a client machine
//! has no repo, no database and no server binaries, so this service is the only thing it can
//! reach. The default stays loopback - going wider is a decision somebody makes.

use std::sync::Arc;

use auth::{http, AuthService, DEFAULT_PORT};
use store::Store;

fn main() -> std::process::ExitCode {
    let mut db_path = String::from("maplecw.db");
    let mut port = DEFAULT_PORT;
    let mut bind = String::from("127.0.0.1");

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--db" if i + 1 < args.len() => {
                db_path = args[i + 1].clone();
                i += 2;
            }
            "--bind" if i + 1 < args.len() => {
                bind = args[i + 1].clone();
                i += 2;
            }
            "--port" if i + 1 < args.len() => {
                match args[i + 1].parse() {
                    Ok(p) => port = p,
                    Err(_) => {
                        eprintln!("invalid port: {}", args[i + 1]);
                        return std::process::ExitCode::FAILURE;
                    }
                }
                i += 2;
            }
            "-h" | "--help" => {
                println!("usage: maplecw-auth [--db <path>] [--bind <addr>] [--port <port>]");
                return std::process::ExitCode::SUCCESS;
            }
            other => {
                eprintln!("unknown argument: {other}");
                return std::process::ExitCode::FAILURE;
            }
        }
    }

    let store = match Store::open(&db_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not open database {db_path}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    if let Ok(n) = store.purge_expired_sessions() {
        if n > 0 {
            println!("purged {n} expired session(s)");
        }
    }

    match store.list_accounts() {
        Ok(accounts) if accounts.is_empty() => {
            println!("note: no accounts yet - create one with `maplecw-useradd <name>`");
        }
        Ok(accounts) => println!("{} account(s) in {db_path}", accounts.len()),
        Err(e) => eprintln!("could not list accounts: {e}"),
    }

    // The certificate lives beside the database: the one directory an installed box, the dev
    // checkout and a smoke test all agree is "this server's". The fingerprint file lands
    // there too, which is where the dev launcher reads it from.
    let cert_dir = std::path::Path::new(&db_path)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let identity = match auth::tls::ensure_identity(cert_dir) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("could not set up TLS in {}: {e}", cert_dir.display());
            return std::process::ExitCode::FAILURE;
        }
    };
    for line in identity.banner() {
        println!("{line}");
    }
    let tls = match identity.server_config() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let service = Arc::new(AuthService::new(Arc::new(store)));
    if let Err(e) = http::serve_on(service, &bind, port, tls) {
        eprintln!("server error: {e}");
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}
