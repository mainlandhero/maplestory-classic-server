//! `maplecw-world` - run one channel of the game world.
//!
//! ```text
//! maplecw-world [--bind ADDR] [--db PATH] [--world-id N] [--channel N]
//! ```
//!
//! One process per channel. The login server hands the client an address and the client
//! connects to it, so a channel that is not listening on its own port cannot be entered.

use std::path::PathBuf;
use std::process::ExitCode;

use world::Config;

const USAGE: &str = "\
maplecw-world - one channel of the MapleCW game world

  --bind ADDR      what this channel listens on   (default 127.0.0.1:8485)
  --db PATH        the SQLite file                (default maplecw.db)
  --world-id N     which world                    (default 0)
  --channel N      which channel                  (default 0)
  -h, --help       this

The database is shared with the login server: that is how a migration minted at
character select is claimed here. See crates/world/src/lib.rs.";

fn main() -> ExitCode {
    let mut config = Config::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or_else(|| format!("{arg} needs a value"));
        let outcome: Result<(), String> = match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            "--bind" => value().and_then(|v| {
                v.parse().map(|b| config.bind = b).map_err(|e| format!("--bind {v}: {e}"))
            }),
            "--db" => value().map(|v| config.db_path = PathBuf::from(v)),
            "--world-id" => value().and_then(|v| {
                v.parse().map(|n| config.world_id = n).map_err(|e| format!("--world-id {v}: {e}"))
            }),
            "--channel" => value().and_then(|v| {
                v.parse().map(|n| config.channel_id = n).map_err(|e| format!("--channel {v}: {e}"))
            }),
            other => Err(format!("unknown argument {other}")),
        };
        if let Err(e) = outcome {
            eprintln!("{e}\n\n{USAGE}");
            return ExitCode::FAILURE;
        }
    }

    if let Err(e) = world::serve(config) {
        eprintln!("maplecw-world: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
