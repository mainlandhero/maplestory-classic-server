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
    // The canonical client this server patches launchers up to. See `auth::clientpatch`: it
    // should be a PREPARED client folder - GameGuard stubbed, the Nexon gate byte patched -
    // because that is the state a launcher's own folder is in when it checks itself.
    let mut client_dir: Option<String> = None;
    let mut launcher: Option<String> = None;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--db" if i + 1 < args.len() => {
                db_path = args[i + 1].clone();
                i += 2;
            }
            "--launcher" if i + 1 < args.len() => {
                launcher = Some(args[i + 1].clone());
                i += 2;
            }
            "--client-dir" if i + 1 < args.len() => {
                client_dir = Some(args[i + 1].clone());
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
                println!("usage: maplecw-auth [--db <path>] [--bind <addr>] [--port <port>] [--client-dir <path>] [--launcher <exe>]");
                println!("  --launcher    publish this launcher executable: a launcher whose own");
                println!("                file hashes differently replaces itself from here");
                println!("  --client-dir  publish this client folder: launchers check their own");
                println!("                copy against it and download only the files that differ.");
                println!("                Point it at a PREPARED client (stub installed, Nexon");
                println!("                gate patched) - see auth::clientpatch.");
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

    let mut service = AuthService::new(Arc::new(store));
    // Scanned once, here, rather than per request: the real client is 405 files and 773 MB,
    // about half a second warm, and doing that for every launcher at once would put a stall in
    // front of every player. A restart is how a new client version is published.
    if let Some(dir) = &client_dir {
        match auth::clientpatch::ClientPatchSource::open(std::path::Path::new(dir)) {
            Ok(source) => {
                println!("{}", source.describe());
                // For the live server's Discord status (`world::discordstatus`), which the hub
                // reads from the same folder: the client patch this server publishes.
                if let Err(e) = std::fs::write("client-patch-version.txt", source.manifest().short_id()) {
                    eprintln!("client-patch-version.txt not written: {e}");
                }
                service = service.with_client_patches(Arc::new(source));
            }
            Err(e) => {
                // Refused rather than started without it. A server that silently publishes no
                // client, when it was told to publish one, turns every launcher's version
                // check into a 503 - and with blocking enabled that is every player locked out
                // by a typo in a path.
                eprintln!("--client-dir {dir}: {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    } else {
        println!("CLIENT PATCHES: OFF - no --client-dir, so /client/manifest answers 503 and");
        println!("  a launcher that requires a confirmed version will refuse to start the game.");
    }
    // The launcher, the same way: hashed once here, a restart publishes a new one. Refused on
    // a bad path for the same reason as the client - a server told to publish a launcher and
    // silently publishing none would tell every launcher it is current.
    if let Some(exe) = &launcher {
        match auth::launcherpatch::LauncherSource::open(std::path::Path::new(exe)) {
            Ok(source) => {
                println!("{}", source.describe());
                service = service.with_launcher_patches(Arc::new(source));
            }
            Err(e) => {
                eprintln!("--launcher {exe}: {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    } else {
        println!("LAUNCHER PATCHES: OFF - no --launcher, so /launcher/manifest answers 503 and");
        println!("  every launcher keeps the executable it has.");
    }
    let service = Arc::new(service);
    if let Err(e) = http::serve_on(service, &bind, port, tls) {
        eprintln!("server error: {e}");
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}
