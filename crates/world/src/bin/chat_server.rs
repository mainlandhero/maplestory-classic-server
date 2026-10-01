//! `maplecw-chat` - the world hub the channel processes dial for anything that crosses
//! channels: the party registry, party chat, invites. `crate::link` is the whole of it.
//!
//! ```text
//! maplecw-chat [--bind ADDR] [--discord-webhook-file PATH]
//! maplecw-chat --discord-offline --discord-webhook-file PATH
//! ```
//!
//! `--discord-webhook-file` turns on the live server's Discord status message
//! (`world::discordstatus`). Only the packaged `start-server.ps1` passes it; the test launcher
//! never does.
//!
//! Server-to-server only. Nothing a game client sends reaches this port, and no frame on it
//! carries credentials - the channel socket still carries none either.

use std::process::ExitCode;

const USAGE: &str = "maplecw-chat - the MapleCW world hub (cross-channel parties and chat)

  --bind ADDR      what the hub listens on (default 127.0.0.1:8483)
  --discord-webhook-file PATH
                   post the server's status to the Discord webhook whose URL is the first line
                   of PATH, as ONE message edited every minute. Live server only.
  --discord-offline
                   with --discord-webhook-file: edit that message to 'offline for maintenance'
                   and exit. start-server.ps1 runs this before it stops the servers.
";

fn main() -> ExitCode {
    let mut bind: std::net::SocketAddr = ([127, 0, 0, 1], world::link::DEFAULT_HUB_PORT).into();
    let mut webhook_file: Option<std::path::PathBuf> = None;
    let mut offline = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            "--bind" => match args.next().map(|v| v.parse::<std::net::SocketAddr>()) {
                Some(Ok(a)) => bind = a,
                other => {
                    eprintln!("--bind: {other:?}");
                    return ExitCode::from(2);
                }
            },
            "--discord-offline" => offline = true,
            "--discord-webhook-file" => match args.next() {
                Some(p) => webhook_file = Some(p.into()),
                None => {
                    eprintln!("--discord-webhook-file needs a path");
                    return ExitCode::from(2);
                }
            },
            other => {
                eprintln!("unknown argument {other}
{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    if offline {
        let Some(file) = webhook_file else {
            eprintln!("--discord-offline needs --discord-webhook-file");
            return ExitCode::from(2);
        };
        return match world::discordstatus::Reporter::from_file(&file).and_then(|mut r| r.publish_offline(unix_now())) {
            Ok(what) => {
                println!("discord: {what}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("discord: {e}");
                ExitCode::from(1)
            }
        };
    }
    let listener = match std::net::TcpListener::bind(bind) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("cannot listen on {bind}: {e}");
            return ExitCode::from(1);
        }
    };
    world::server::log(&format!("maplecw-chat: world hub listening on {bind}. Nothing authenticates here; server-to-server only."));
    // A webhook that cannot be read is reported and the hub runs without it: parties and chat
    // must not go down because a status message could not be posted.
    let status = webhook_file.and_then(|f| match world::discordstatus::Reporter::from_file(&f) {
        Ok(r) => {
            world::server::log(&r.describe());
            console::mark_offline_on_close(f);
            Some(r)
        }
        Err(e) => {
            world::server::log(&format!("DISCORD STATUS: OFF - {e}"));
            None
        }
    });
    world::link::run_hub_with_status(listener, status);
    ExitCode::SUCCESS
}

fn unix_now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

/// **The window closing takes the hub with it - mark the message offline first.** A console
/// control handler runs on Ctrl+C, Ctrl+Break, the window's X, logoff and shutdown; Windows gives
/// it a few seconds, which one HTTPS edit fits in. A process killed outright (`Stop-Process`,
/// Task Manager) runs no handler - that is what `--discord-offline` in the stop script is for.
#[cfg(windows)]
mod console {
    use std::path::PathBuf;
    use std::sync::OnceLock;

    static WEBHOOK_FILE: OnceLock<PathBuf> = OnceLock::new();

    type HandlerRoutine = unsafe extern "system" fn(u32) -> i32;

    #[link(name = "kernel32")]
    extern "system" {
        fn SetConsoleCtrlHandler(handler: Option<HandlerRoutine>, add: i32) -> i32;
    }

    unsafe extern "system" fn on_console_event(event: u32) -> i32 {
        if let Some(file) = WEBHOOK_FILE.get() {
            let outcome = world::discordstatus::Reporter::from_file(file).and_then(|mut r| r.publish_offline(super::unix_now()));
            world::server::log(&format!("discord: console event {event}: {}", outcome.unwrap_or_else(|e| e)));
        }
        0 // not handled: the default handler ends the process, as it would have
    }

    pub fn mark_offline_on_close(file: PathBuf) {
        if WEBHOOK_FILE.set(file).is_ok() {
            unsafe {
                SetConsoleCtrlHandler(Some(on_console_event), 1);
            }
        }
    }
}

#[cfg(not(windows))]
mod console {
    pub fn mark_offline_on_close(_file: std::path::PathBuf) {}
}
