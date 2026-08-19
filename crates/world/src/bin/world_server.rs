//! `maplecw-world` - run one channel of the game world.
//!
//! ```text
//! maplecw-world [--bind ADDR] [--db PATH] [--world-id N] [--channel N]
//!               [--set-field-probe]
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
  --set-field-probe   answer the migration hello with the fixed head of a SetField
                      and nothing after it. OFF by default. It cannot put a character
                      in a map; it exists so a run can tell an ignored packet apart
                      from one that never arrived, which the handler's two silent
                      early returns otherwise make identical. Arm a watch on
                      142097f80 or the run measures nothing.
  -h, --help       this

The database is shared with the login server: that is how a migration minted at
character select is claimed here. See crates/world/src/lib.rs.";

fn main() -> ExitCode {
    let mut config = Config::default();
    let mut portals_path = PathBuf::from("gm-handbook/portals.txt");
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
            "--set-field-probe" => {
                config.set_field_probe = true;
                Ok(())
            }
            "--portals" => value().map(|v| portals_path = PathBuf::from(v)),
            other => Err(format!("unknown argument {other}")),
        };
        if let Err(e) = outcome {
            eprintln!("{e}\n\n{USAGE}");
            return ExitCode::FAILURE;
        }
    }

    // Portals come from the client's own Map.wz via tools/dump_portals.py. A missing file
    // is not fatal: the server still answers a transfer request, it just cannot resolve a
    // destination and says so per request rather than failing to start.
    config.portals = world::config::Config::load_portals(&portals_path);
    if config.portals.is_empty() {
        eprintln!(
            "maplecw-world: no portals loaded from {} - portal walks will re-send the              current map. Regenerate with: python tools/dump_portals.py",
            portals_path.display()
        );
    }

    if let Err(e) = world::serve(config) {
        eprintln!("maplecw-world: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
