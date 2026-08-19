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
  --inventory-slots N  give every inventory N slots instead of the character's own,
                   so a client run can read the number off the screen (1..=100).
                   Go UNDER the 30 default: the window is 5x6 with a scrollbar,
                   so a bigger number looks the same as a fixed viewport
  --no-mobs        do NOT send monsters. On by default since 2026-08-19; this is for
                   eliminating mobs as a variable, not for ordinary use
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
    let mut npcs_path = PathBuf::from("gm-handbook/npcs.txt");
    let mut fields_path = PathBuf::from("gm-handbook/fields.txt");
    let mut mobs_path = PathBuf::from("gm-handbook/mobs.txt");
    let mut equips_path = PathBuf::from("gm-handbook/equips.txt");
    let mut mob_templates_path = PathBuf::from("gm-handbook/mobtemplates.txt");
    let mut npc_strings_path = PathBuf::from("gm-handbook/npcstrings.txt");
    let mut quests_path = PathBuf::from("gm-handbook/questlines.txt");
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
            "--npcs" => value().map(|v| npcs_path = PathBuf::from(v)),
            "--fields" => value().map(|v| fields_path = PathBuf::from(v)),
            "--mobs-file" => value().map(|v| mobs_path = PathBuf::from(v)),
            "--equips" => value().map(|v| equips_path = PathBuf::from(v)),
            "--mob-templates" => value().map(|v| mob_templates_path = PathBuf::from(v)),
            "--npc-strings" => value().map(|v| npc_strings_path = PathBuf::from(v)),
            "--quests" => value().map(|v| quests_path = PathBuf::from(v)),
            "--inventory-slots" => value().and_then(|v| {
                v.parse::<u16>()
                    .map_err(|e| format!("--inventory-slots {v}: {e}"))
                    .and_then(|n| {
                        if n == 0 || n > net::opcode::MAX_INVENTORY_SLOTS {
                            Err(format!(
                                "--inventory-slots {n}: must be 1..={}",
                                net::opcode::MAX_INVENTORY_SLOTS
                            ))
                        } else {
                            config.inventory_slots = Some(n);
                            Ok(())
                        }
                    })
            }),
            "--mob-limit" => value().and_then(|v| {
                v.parse()
                    .map(|n: usize| config.mob_limit = Some(n))
                    .map_err(|e| format!("--mob-limit {v}: {e}"))
            }),
            // Kept, and a no-op, because it is in the owner's shell history and in three
            // documents. An argument that used to mean something and now errors is a
            // failed launch for a reason nobody would guess.
            "--mobs" => {
                config.send_mobs = true;
                Ok(())
            }
            "--no-mobs" => {
                config.send_mobs = false;
                Ok(())
            }
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
    (config.portals, config.portal_index) = world::config::Config::load_portals(&portals_path);
    if config.portals.is_empty() {
        eprintln!(
            "maplecw-world: no portals loaded from {} - portal walks will re-send the              current map. Regenerate with: python tools/dump_portals.py",
            portals_path.display()
        );
    }

    config.fields = world::config::Config::load_fields(&fields_path);
    if config.fields.is_empty() {
        eprintln!(
            "maplecw-world: no field list from {} - map ids will NOT be validated, so a              bad /map can strand a character. Regenerate with: python tools/dump_portals.py",
            fields_path.display()
        );
    }

    config.npcs = world::config::Config::load_npcs(&npcs_path);
    if config.npcs.is_empty() {
        eprintln!(
            "maplecw-world: no NPCs loaded from {} - maps will be empty. Regenerate with:              python tools/dump_portals.py",
            npcs_path.display()
        );
    }

    let mob_templates = world::config::load_mob_templates(&mob_templates_path);
    if mob_templates.is_empty() {
        eprintln!(
            "maplecw-world: no mob templates from {} - every mob would spawn on a fallback              HP rather than its own. Regenerate with: python tools/dump_mobs.py",
            mob_templates_path.display()
        );
    }
    config.mobs = world::config::Config::load_mobs(&mobs_path, &mob_templates);
    if config.mobs.is_empty() {
        eprintln!(
            "maplecw-world: no mobs loaded from {} - maps will have no monsters.              Regenerate with: python tools/dump_portals.py",
            mobs_path.display()
        );
    }
    // On stdout, not stderr. The old warning went to eprintln and therefore to
    // world.log.err, which is not the file anyone opens while a client is running - so a
    // server that had six snails loaded and sent none looked exactly like a server that
    // had none, for a whole launch.
    if !config.send_mobs {
        println!(
            "maplecw-world: MOBS ARE OFF (--no-mobs). {} maps have mobs loaded and none of              them will be sent. Maps will look empty; that is this flag, not a bug.",
            config.mobs.len()
        );
    }

    config.equips = world::config::Config::load_equips(&equips_path);
    if config.equips.is_empty() {
        eprintln!(
            "maplecw-world: no equip templates from {} - worn items will have no stats and              no upgrade slots. Regenerate with: python tools/dump_equips.py",
            equips_path.display()
        );
    }

    config.npc_strings = world::config::load_npc_strings(&npc_strings_path);
    if config.npc_strings.is_empty() {
        eprintln!(
            "maplecw-world: no NPC text from {} - NPCs will fall back to placeholder              dialogue. Regenerate with: python tools/dump_npcstrings.py",
            npc_strings_path.display()
        );
    }

    config.quests = world::config::load_quests(&quests_path);
    if config.quests.is_empty() {
        eprintln!(
            "maplecw-world: no quest text from {} - NPCs will fall back to their generic              line. Regenerate with: python tools/dump_quests.py",
            quests_path.display()
        );
    }

    if let Err(e) = world::serve(config) {
        eprintln!("maplecw-world: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
