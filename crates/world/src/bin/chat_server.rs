//! `maplecw-chat` - the world hub the channel processes dial for anything that crosses
//! channels: the party registry, party chat, invites. `crate::link` is the whole of it.
//!
//! ```text
//! maplecw-chat [--bind ADDR]      default 127.0.0.1:8483
//! ```
//!
//! Server-to-server only. Nothing a game client sends reaches this port, and no frame on it
//! carries credentials - the channel socket still carries none either.

use std::process::ExitCode;

const USAGE: &str = "maplecw-chat - the MapleCW world hub (cross-channel parties and chat)

  --bind ADDR      what the hub listens on (default 127.0.0.1:8483)
";

fn main() -> ExitCode {
    let mut bind: std::net::SocketAddr = ([127, 0, 0, 1], world::link::DEFAULT_HUB_PORT).into();
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
            other => {
                eprintln!("unknown argument {other}
{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    let listener = match std::net::TcpListener::bind(bind) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("cannot listen on {bind}: {e}");
            return ExitCode::from(1);
        }
    };
    world::server::log(&format!("maplecw-chat: world hub listening on {bind}. Nothing authenticates here; server-to-server only."));
    world::link::run_hub(listener);
    ExitCode::SUCCESS
}
