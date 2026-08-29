//! `maplecw-login` - run the login server.
//!
//! ```text
//! maplecw-login [--bind ADDR] [--db PATH] [--account NAME] [--display-name NAME]
//!               [--world NAME] [--world-id N] [--channels N]
//! ```
//!
//! Defaults are loopback and `maplecw.db`, which is what every run so far has used. The
//! account must already exist; create one with `maplecw-useradd`.

use std::path::PathBuf;
use std::process::ExitCode;

use login::{Config, World};

const USAGE: &str = "\
maplecw-login - the MapleCW login server

  --bind ADDR           what to listen on           (default 127.0.0.1:8484)
  --db PATH             the SQLite file             (default maplecw.db)
  --account NAME        which account every connection is served as (default maplecw)
  --display-name NAME   what the login screen shows (default: the account name)
  --world NAME          world name                  (default Scania)
  --world-id N          world id                    (default 0)
  --channels A,B,...    one address per channel, in channel order (default
                        127.0.0.1:8485). These are what the client is told to connect to
                        when it enters the world, so they must be reachable from the
                        *client* machine, not from the server. IPv4 only: the migration
                        packet carries four octets. Run one maplecw-world per address.
  --channel N           which channel a player entering the world is sent to (default 0)
  --list                print the stored characters and exit, without listening
  --delete NAME         delete one character on --account, then exit
  --bind-migrations     bind each migration to the launcher sign-in that authorised it.
                        DO NOT PASS THIS YET. A bound migration can only be claimed by a
                        connection presenting a matching session token, and the channel
                        server presents nothing - so with this on, every migration is
                        refused and no character can enter the world. It needs the hook to
                        send the token on the channel socket first.
  -h, --help            this

The game socket carries no credentials, so --account is not a login: it decides whose
characters every connection sees. See docs/launcher.md.";

fn main() -> ExitCode {
    let mut config = Config::default();
    let mut display_name: Option<String> = None;
    let mut list_only = false;
    let mut delete_name: Option<String> = None;

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
            "--list" => {
                list_only = true;
                Ok(())
            }
            "--bind-migrations" => {
                config.bind_migrations = true;
                Ok(())
            }
            "--delete" => value().map(|v| delete_name = Some(v)),
            "--db" => value().map(|v| config.db_path = PathBuf::from(v)),
            "--account" => value().map(|v| config.account = v),
            "--display-name" => value().map(|v| display_name = Some(v)),
            "--world" => value().map(|v| config.world.name = v),
            "--world-id" => value().and_then(|v| {
                v.parse().map(|n| config.world.id = n).map_err(|e| format!("--world-id {v}: {e}"))
            }),
            "--channels" => value().and_then(|v| {
                v.split(',')
                    .map(|a| a.trim().parse())
                    .collect::<Result<Vec<_>, _>>()
                    .map(|c| config.world.channels = c)
                    .map_err(|e| format!("--channels {v}: {e} (IPv4 host:port, comma separated)"))
            }),
            "--channel" => value().and_then(|v| {
                v.parse()
                    .map(|n| config.world.channel_id = n)
                    .map_err(|e| format!("--channel {v}: {e}"))
            }),
            other => Err(format!("unknown argument {other}")),
        };
        if let Err(e) = outcome {
            eprintln!("{e}\n\n{USAGE}");
            return ExitCode::FAILURE;
        }
    }

    // The login screen shows the account name unless something better is configured.
    config.display_name = display_name.unwrap_or_else(|| config.account.clone());
    let World { id, ref channels, channel_id, .. } = config.world;
    if channels.is_empty() {
        eprintln!("--channels: at least one channel address is required

{USAGE}");
        return ExitCode::FAILURE;
    }
    if channel_id as usize >= channels.len() {
        eprintln!(
            "--channel {channel_id}: there are only {} channel(s)

{USAGE}",
            channels.len()
        );
        return ExitCode::FAILURE;
    }
    debug_assert!(id <= u32::from(u8::MAX));

    let outcome = match (list_only, delete_name.as_deref()) {
        (_, Some(name)) => login::delete(&config, name),
        (true, None) => login::list(&config),
        (false, None) => login::serve(config),
    };
    if let Err(e) = outcome {
        eprintln!("maplecw-login: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
