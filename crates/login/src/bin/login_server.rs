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
  --no-list-resend      do NOT send the character list a second time when the client's
                        0x007A report shows it arrived before the client was ready (the
                        kill switch for login::session::LIST_RESEND_THRESHOLD_MS)
  --fallback-account NAME  serve a connection this server CANNOT attribute to a launcher
                        sign-in as NAME instead of refusing it. OFF by default: without
                        it, such a connection gets a login failure (notRegisteredID) and
                        sees no characters. For the smoke tests and a one-player dev box
                        only - with it on, ANYTHING that reaches this port is served as NAME
  --account NAME        the account --list and --delete operate on
  --display-name NAME   what the login screen shows for an account with no email
  --world NAME          world name                  (default Scania)
  --world-id N          world id                    (default 0)
  --channels A,B,...    one address per channel, in channel order (default
                        127.0.0.1:8485). These are what the client is told to connect to
                        when it enters the world, so they must be reachable from the
                        *client* machine, not from the server. IPv4 only: the migration
                        packet carries four octets. Run one maplecw-world per address.
  --channel N           which channel a player entering the world is sent to (default 0)
  --advertise MODE      which HOST those channels are advertised as: auto (default), list,
                        or one IPv4 address. --help prints the full description
  --list                print the stored characters and exit, without listening
  --delete NAME         delete one character on --account, then exit
  --bind-migrations MODE  when a migration is bound to the sign-in that authorised it, so
                        that only that launch's client can claim it at the channel:
                          auto    (default) bind when the login connection came from a
                                  process on THIS machine - the channel can check that
                                  through the OS. Off-box connections are bound by their
                                  address instead (the channel requires it to match)
                          always  bind every migration. Refuses every OFF-BOX client until
                                  the hook carries the token on the channel connection
                          never   bind nothing (the behaviour before 2026-09-05)
  -h, --help            this

The game socket carries no credentials. A connection is served as the account whose launcher
sign-in it can be tied to - by the one-time token the client carries, the process that owns
the socket, or the address - and refused when it cannot be tied to any. See docs/launcher.md.";

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
                println!("{USAGE}\n\n{}", net::advertise::USAGE);
                return ExitCode::SUCCESS;
            }
            "--bind" => value().and_then(|v| {
                v.parse().map(|b| config.bind = b).map_err(|e| format!("--bind {v}: {e}"))
            }),
            "--advertise" => value().and_then(|v| {
                net::advertise::Mode::parse(&v).map(|mode| {
                    config.advertise = std::sync::Arc::new(net::advertise::Advertiser::new(mode))
                })
            }),
            "--list" => {
                list_only = true;
                Ok(())
            }
            "--bind-migrations" => value().and_then(|v| {
                login::config::MigrationBinding::parse(&v).map(|m| config.bind_migrations = m)
            }),
            "--delete" => value().map(|v| delete_name = Some(v)),
            "--db" => value().map(|v| config.db_path = PathBuf::from(v)),
            "--no-list-resend" => {
                config.resend_list_on_late_report = false;
                Ok(())
            }
            "--account" => value().map(|v| config.account = Some(v)),
            "--fallback-account" => value().map(|v| config.fallback_account = Some(v)),
            "--display-name" => value().map(|v| display_name = Some(v)),
            "--world" => value().map(|v| config.world.name = v),
            "--world-id" => value().and_then(|v| {
                v.parse().map(|n| config.world.id = n).map_err(|e| format!("--world-id {v}: {e}"))
            }),
            "--channels" => value().and_then(|v| {
                net::advertise::parse_channels(&v)
                    .map(|c| config.world.channels = c)
                    .map_err(|e| format!("--channels {v}: {e}"))
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

    // What the login screen shows for an account with no email: the fallback's name if there
    // is one, else the server's. Every account with an email shows its own masked address.
    config.display_name = display_name
        .unwrap_or_else(|| config.fallback_account.clone().unwrap_or_else(|| "MapleCW".into()));
    // The equip templates behind the select sheet's equipment totals - `login::selectstats`.
    // A missing file is not fatal: stored (scrolled) bonuses still show, unscrolled items'
    // base stats do not, and the banner says which state we are in on every start.
    let equips_path = PathBuf::from("gm-handbook/equips.txt");
    config.equips = world::config::Config::load_equips(&equips_path);
    if config.equips.is_empty() {
        eprintln!(
            "maplecw-login: no equip templates from {} - the select sheet will carry STORED bonuses only; unscrolled items' base stats will be missing. Regenerate with: python tools/dump_equips.py",
            equips_path.display()
        );
    } else {
        println!("maplecw-login: {} equip templates for the select sheet", config.equips.len());
    }
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
    // A bare-port channel under --advertise list would tell the client to dial 0.0.0.0.
    if let Err(e) = config.advertise.validate(channels) {
        eprintln!("{e}\n\n{USAGE}");
        return ExitCode::FAILURE;
    }

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
