//! `maplecw-world` - run one channel of the game world.
//!
//! ```text
//! maplecw-world [--bind ADDR] [--db PATH] [--world-id N] [--channel N]
//!               [--silent-channel]
//! ```
//!
//! One process per channel. The login server hands the client an address and the client
//! connects to it, so a channel that is not listening on its own port cannot be entered.

use std::path::PathBuf;
use std::process::ExitCode;

// Every `println!` in this file is a startup banner line, and once `--log-file` is installed
// those belong in the same file as everything after them - `world::server::say` writes
// there, or to stdout when no file is installed. Shadowing the macro is one line; routing
// fifty-six call sites by hand is fifty-six chances to miss one. `eprintln!` is untouched:
// an error still goes to stderr, which the launch scripts keep as `<log>.err`.
macro_rules! println {
    ($($t:tt)*) => { world::server::say(&format!($($t)*)) };
}

use world::Config;

const USAGE: &str = "\
maplecw-world - one channel of the MapleCW game world

  --bind ADDR      what this channel listens on   (default 127.0.0.1:8485)
  --db PATH        the SQLite file                (default maplecw.db)
  --world-id N     which world                    (default 0)
  --channel N      which channel                  (default 0)
  --channels A,B,. one address per channel, in channel order - host:port, or a bare port
                   when --advertise decides the host. Needed only to answer Change
                   Channel (0x00D2); without it the request is refused with a message
                   rather than ignored.
  --advertise MODE which HOST a Change Channel answer names: auto (default), list, or
                   one IPv4 address. --help prints the full description.
  --link ADDR      the world hub (maplecw-chat) to dial for cross-channel parties and
                   chat (default 127.0.0.1:8483). 'none' runs this channel on its own.
  --migration-peer-policy require|record
                   what to do when the connection claiming a migration does not come
                   from the address the migration was minted for. require (default)
                   refuses it - the off-box half of only-the-launcher-client-enters;
                   record logs it and lets it through, as every run before 2026-09-05 did.
  --pet-move-action N  the moveAction byte a summoned pet gets in 0x0277, instead of
                   0. A test lever: 30 sends the client down the stance-1 arm, the
                   only pet arm that gives the pet's layer a z. See Config::pet_move_action
  --broadcast-pets show a summoned pet to OTHER players in the map. OFF by default:
                   it crashed a second client on 2026-09-15 (the remote pet has no
                   visual and its first move null-derefs). Owner-local otherwise.
                   research/pet-remote-crash-2026-09-15.md
  --inventory-slots N  give every inventory N slots instead of the character's own,
                   so a client run can read the number off the screen (1..=100).
                   Go UNDER the 30 default: the window is 5x6 with a scrollbar,
                   so a bigger number looks the same as a fixed viewport
  --no-mobs        do NOT send monsters. On by default since 2026-08-19; this is for
                   eliminating mobs as a variable, not for ordinary use
  --set-field-probe   ACCEPTED AND IGNORED since 2026-09-14 - the channel answers by
                      default now. It is left in the parser because the flag is
                      typed into STATUS.md, the fixture notes and the launch lines,
                      and an unknown argument fails the whole paste.
  --log-file PATH   write the log to PATH instead of stdout, and ROLL it at 50 MB:
                   PATH.1 is the newest roll, PATH.5 the oldest, the sixth is
                   deleted. The launch scripts pass world-chN.log here. Without
                   it the log is stdout, as before, and nothing can roll it -
                   whoever redirected stdout owns that file
  --log-chatter    log every 0x02FF mob move, 0x03E4 ack and inbound 0x0070
                   report as its own line. Measured 2026-09-14: that is 97% of a
                   busy channel log. Off, they are COUNTED and the counts printed
                   once a minute; on, they are logged as every run before did
  --silent-channel  answer NOTHING: Session::handle and Session::tick return empty
                      for every packet, the migration hello included, so a client
                      sits on Connecting... forever. This was the DEFAULT until
                      2026-09-14, when it turned out all 61 callers passed
                      --set-field-probe to escape it and the only thing the default
                      had ever done was cost a manual launch. Pass this to
                      eliminate the channel as a variable, and for nothing else.
  --footholds PATH  map floor geometry, from tools/dump_portals.py
  --consumables PATH  what potions restore, from tools/dump_itemdata.py
  --skills PATH    what each job may learn and how far, from
                   tools/dump_skills.py. Missing means only the three beginner
                   skills can be raised - the behaviour before the table existed
  --commodity PATH  the cash shop's sale list keyed by SN, from
                   tools/dump_commodity.py. Missing means every purchase is
                   refused as sold out - the client draws its catalogue from
                   its own copy either way, so the shop still looks stocked
  --shops PATH     the authored NPC shop file      (default data/shops.txt)
  --npc-dialogue PATH  authored NPC dialogue laid over the generated npcstrings
                   (default data/npc-dialogue.txt). Rows are
                   `templateId TAB d<n> TAB text` and they REPLACE the client's
                   line. Re-read in game by !npcreload, with no restart and
                   without disconnecting anybody - see world::config
  --quest-scripts PATH  authored openings for the 12 quests whose bodies the
                      client does NOT ship (default data/quest-scripts.txt).
                      An overlay: it only fills nodes the WZ left empty, and a
                      shipped row always wins. See research/quest-scripts.md.
  --item-names PATH  id -> name, from tools/dump_names.py
  --item-data PATH   id -> price/quest/tradeBlock/slotMax, from tools/dump_itemdata.py
  -h, --help       this

The database is shared with the login server: that is how a migration minted at
character select is claimed here. See crates/world/src/lib.rs.";

fn main() -> ExitCode {
    let mut config = Config::default();
    // The hub is dialled by default: a channel started without one still works, it just
    // says so in its log and keeps parties and party chat to itself.
    config.link = Some(std::net::SocketAddr::from(([127, 0, 0, 1], world::link::DEFAULT_HUB_PORT)));
    let mut portals_path = PathBuf::from("gm-handbook/portals.txt");
    let mut npcs_path = PathBuf::from("gm-handbook/npcs.txt");
    let mut fields_path = PathBuf::from("gm-handbook/fields.txt");
    let mut footholds_path = PathBuf::from("gm-handbook/footholds.txt");
    let mut consumables_path = PathBuf::from("gm-handbook/consumables.txt");
    let mut commodity_path = PathBuf::from("gm-handbook/commodity.txt");
    let mut skills_path = PathBuf::from("gm-handbook/skills.txt");
    let mut mobs_path = PathBuf::from("gm-handbook/mobs.txt");
    let mut equips_path = PathBuf::from("gm-handbook/equips.txt");
    let mut scrolls_path = PathBuf::from("gm-handbook/scrolls.txt");
    let mut sacks_path = PathBuf::from("gm-handbook/summonsacks.txt");
    let mut chairs_path = PathBuf::from("gm-handbook/chairs.txt");
    let mut mob_templates_path = PathBuf::from("gm-handbook/mobtemplates.txt");
    let mut npc_strings_path = PathBuf::from("gm-handbook/npcstrings.txt");
    let mut pet_commands_path = PathBuf::from("gm-handbook/petcommands.txt");
    // Authored source like data/shops.txt: hand-written, committed, and NOT in gm-handbook/,
    // which is generated and would be overwritten by the next dump_npcstrings.py run.
    let mut npc_dialogue_path = PathBuf::from("data/npc-dialogue.txt");
    let mut quests_path = PathBuf::from("gm-handbook/questlines.txt");
    // Authored source like data/shops.txt: the script bodies are not in the client at all.
    let mut quest_scripts_path = PathBuf::from("data/quest-scripts.txt");
    // Authored source, not generated data - the only path here that is not gm-handbook/.
    let mut shops_path = PathBuf::from("data/shops.txt");
    let mut item_names_path = PathBuf::from("gm-handbook/items.txt");
    let mut item_data_path = PathBuf::from("gm-handbook/itemdata.txt");
    // Authored source like data/shops.txt: it cannot be regenerated from the client.
    let mut drops_path = PathBuf::from("data/drops.txt");
    let mut reactors_path = PathBuf::from("gm-handbook/reactors.txt");
    let mut reactor_drops_path = PathBuf::from("data/reactor-drops.txt");
    let mut exp_curve_path = PathBuf::from("data/exp-curve.txt");
    let mut quest_reqs_path = PathBuf::from("gm-handbook/questreq.txt");
    // `--log-file` is honoured before anything else is parsed or printed, so the very first
    // banner line lands in the file. Everything else keeps its order.
    {
        let raw: Vec<String> = std::env::args().skip(1).collect();
        if let Some(i) = raw.iter().position(|a| a == "--log-file") {
            match raw.get(i + 1) {
                Some(path) => {
                    if let Err(e) = world::server::install_log_file(std::path::Path::new(path)) {
                        eprintln!("maplecw-world: cannot open --log-file {path}: {e}");
                        return ExitCode::FAILURE;
                    }
                }
                None => {
                    eprintln!("--log-file needs a path\n\n{USAGE}");
                    return ExitCode::FAILURE;
                }
            }
        }
    }
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
            "--db" => value().map(|v| config.db_path = PathBuf::from(v)),
            "--link" => value().and_then(|v| {
                if v.eq_ignore_ascii_case("none") {
                    config.link = None;
                    Ok(())
                } else {
                    v.parse().map(|a| config.link = Some(a)).map_err(|e| format!("--link {v}: {e}"))
                }
            }),
            "--world-id" => value().and_then(|v| {
                v.parse().map(|n| config.world_id = n).map_err(|e| format!("--world-id {v}: {e}"))
            }),
            "--channel" => value().and_then(|v| {
                v.parse().map(|n| config.channel_id = n).map_err(|e| format!("--channel {v}: {e}"))
            }),
            "--channels" => value().and_then(|v| {
                net::advertise::parse_channels(&v)
                    .map(|c| config.channels = c)
                    .map_err(|e| format!("--channels {v}: {e}"))
            }),
            "--advertise" => value().and_then(|v| {
                net::advertise::Mode::parse(&v).map(|mode| {
                    config.advertise = std::sync::Arc::new(net::advertise::Advertiser::new(mode))
                })
            }),
            "--migration-peer-policy" => value().and_then(|v| match v.trim().to_ascii_lowercase().as_str() {
                "require" => {
                    config.peer_policy = store::migration::PeerPolicy::Require;
                    Ok(())
                }
                "record" => {
                    config.peer_policy = store::migration::PeerPolicy::Record;
                    Ok(())
                }
                other => Err(format!("--migration-peer-policy {other:?}: expected require or record")),
            }),
            // Accepted and ignored. It was how the channel was switched ON until
            // 2026-09-14, and it is still typed into `STATUS.md`, every fixture note and
            // The owner's launch lines - so it keeps working rather than failing an elevated
            // paste with "unknown argument".
            "--set-field-probe" => Ok(()),
            // Consumed above, before the loop; accepted here so it is not "unknown".
            "--log-file" => value().map(|_| ()),
            "--log-chatter" => {
                world::server::LOG_CHATTER.store(true, std::sync::atomic::Ordering::Relaxed);
                Ok(())
            }
            "--silent-channel" => {
                config.answer_packets = false;
                Ok(())
            }
            // Off by default since it crashed a second client on 2026-09-15
            // (research/pet-remote-crash-2026-09-15.md). Turn it on only to re-investigate the
            // remote-pet path with a second client and a dump armed.
            "--broadcast-pets" => {
                config.broadcast_pets = true;
                Ok(())
            }
            "--portals" => value().map(|v| portals_path = PathBuf::from(v)),
            "--npcs" => value().map(|v| npcs_path = PathBuf::from(v)),
            "--fields" => value().map(|v| fields_path = PathBuf::from(v)),
            "--footholds" => value().map(|v| footholds_path = PathBuf::from(v)),
            "--consumables" => value().map(|v| consumables_path = PathBuf::from(v)),
            "--commodity" => value().map(|v| commodity_path = PathBuf::from(v)),
            "--skills" => value().map(|v| skills_path = PathBuf::from(v)),
            "--mobs-file" => value().map(|v| mobs_path = PathBuf::from(v)),
            "--equips" => value().map(|v| equips_path = PathBuf::from(v)),
            "--scrolls" => value().map(|v| scrolls_path = PathBuf::from(v)),
            "--summon-sacks" => value().map(|v| sacks_path = PathBuf::from(v)),
            "--chairs" => value().map(|v| chairs_path = PathBuf::from(v)),
            "--mob-templates" => value().map(|v| mob_templates_path = PathBuf::from(v)),
            "--npc-strings" => value().map(|v| npc_strings_path = PathBuf::from(v)),
            "--pet-commands" => value().map(|v| pet_commands_path = PathBuf::from(v)),
            "--npc-dialogue" => value().map(|v| npc_dialogue_path = PathBuf::from(v)),
            "--quests" => value().map(|v| quests_path = PathBuf::from(v)),
            "--quest-scripts" => value().map(|v| quest_scripts_path = PathBuf::from(v)),
            "--shops" => value().map(|v| shops_path = PathBuf::from(v)),
            "--item-names" => value().map(|v| item_names_path = PathBuf::from(v)),
            "--item-data" => value().map(|v| item_data_path = PathBuf::from(v)),
            "--drops" => value().map(|v| drops_path = PathBuf::from(v)),
            "--reactors-file" => value().map(|v| reactors_path = PathBuf::from(v)),
            "--reactor-drops" => value().map(|v| reactor_drops_path = PathBuf::from(v)),
            "--exp-curve" => value().map(|v| exp_curve_path = PathBuf::from(v)),
            "--quest-reqs" => value().map(|v| quest_reqs_path = PathBuf::from(v)),
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
            "--pet-move-action" => value().and_then(|v| {
                v.parse::<u8>()
                    .map(|n| {
                        config.pet_move_action = Some(n);
                    })
                    .map_err(|e| format!("--pet-move-action {v}: {e}"))
            }),
            "--mob-limit" => value().and_then(|v| {
                v.parse()
                    .map(|n: usize| config.mob_limit = Some(n))
                    .map_err(|e| format!("--mob-limit {v}: {e}"))
            }),
            // **A no-op since 2026-08-28, and kept for the reason `--session-tokens` is.**
            // Shops are ON by default now. The flag existed because `0x0560` killed the
            // client; the cause was that this client has two shop windows and that was the
            // one whose art it does not ship. `net::classicshop` sends `0x055D` instead.
            // An argument that used to mean something and now errors is a failed launch for
            // a reason nobody would guess, so this accepts and says so.
            "--shop" => {
                println!("--shop: no longer needed - NPC shops are on by default. The old                           0x0560 window is gone; this client's counter is the classic 0x055D.");
                Ok(())
            }
            // The shop counter's blast-radius control. It was known NOT to be the variable
            // for Shop2 - one correctly-formed row killed the client exactly as twelve did,
            // because it died before reading any row byte - but for the CLASSIC counter it is
            // a live instrument again: that window really does parse rows, and
            // `research/classic-shop-rows.md` names five separate gates that can drop one.
            "--shop-rows" => value().and_then(|v| {
                v.parse()
                    .map(|n: usize| config.shop_rows = Some(n))
                    .map_err(|e| format!("--shop-rows {v}: {e}"))
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
    (config.portals, config.portal_index, config.portal_positions) =
        world::config::Config::load_portals_with_positions(&portals_path);
    if !config.portal_index.is_empty() && config.portal_positions.is_empty() {
        eprintln!(
            "maplecw-world: {} has no x, y columns, so an arriving character is announced at the map ORIGIN until their first step. Regenerate with: python tools/dump_portals.py",
            portals_path.display()
        );
    }
    if config.portals.is_empty() {
        eprintln!(
            "maplecw-world: no portals loaded from {} - portal walks will re-send the current map. Regenerate with: python tools/dump_portals.py",
            portals_path.display()
        );
    }

    // Names for the GM acknowledgements. A missing file costs an id printed bare - see
    // Config::map_names - so neither of these gets a startup warning.
    config.map_names = world::config::Config::load_id_names(&PathBuf::from("gm-handbook/maps.txt"));
    config.item_names =
        world::config::Config::load_id_names(&PathBuf::from("gm-handbook/items.txt"));
    config.fields = world::config::Config::load_fields(&fields_path);
    // Which maps draw a wall clock. A missing file is safe - no clock is sent anywhere,
    // which is exactly the state before 2026-09-10 - so it warns and carries on. Sending
    // to a map that is NOT in the list is the unsafe direction (the client throws), and an
    // empty list cannot do that.
    let clocks_path = PathBuf::from("gm-handbook/clocks.txt");
    config.clocks = world::config::Config::load_clocks(&clocks_path);
    if config.clocks.is_empty() {
        eprintln!(
            "maplecw-world: no clock table from {} - field clocks will stay at 00:00. Regenerate with: python tools/dump_portals.py",
            clocks_path.display()
        );
    } else {
        println!("maplecw-world: clocks: {} maps declare a field clock", config.clocks.len());
    }
    // Where a dead character comes back. A missing file is not fatal - revive then leaves
    // the player where they fell, which is wrong but safe - so it warns rather than exits.
    let revive_path = PathBuf::from("gm-handbook/returnmaps.txt");
    config.revive_maps = world::config::Config::load_revive_maps(&revive_path);
    if config.revive_maps.is_empty() {
        eprintln!(
            "maplecw-world: no revive map table from {} - a dead character will be revived WHERE THEY FELL. Regenerate with: python tools/dump_returnmaps.py",
            revive_path.display()
        );
    } else {
        println!("maplecw-world: revive: {} maps have a respawn destination", config.revive_maps.len());
    }
    if config.fields.is_empty() {
        eprintln!(
            "maplecw-world: no field list from {} - map ids will NOT be validated, so a bad /map can strand a character. Regenerate with: python tools/dump_portals.py",
            fields_path.display()
        );
    }

    // The floor. A missing file degrades to the behaviour that shipped before it existed -
    // drops land at the height the mob died at - and the banner says which of the two states
    // we are in, on STDOUT, in both cases. `map_exists` was fail-open on an empty table with
    // its only warning on stderr, where nothing reads it, and that silently removed the
    // `!map` guard for a day. A banner that is quiet when things are fine cannot be told
    // apart from one that is not being printed, so this one is unconditional.
    config.footholds = world::footholds::Footholds::load(&footholds_path);
    println!("{}", config.footholds.banner());
    for line in config.footholds.problems() {
        println!("maplecw-world: footholds: {line}");
    }

    // What a potion does. Loud in both directions on stdout, same reasoning as the floor.
    config.consumables = world::consumables::Consumables::load(&consumables_path);
    println!("{}", config.consumables.banner());

    // What the cash shop sells, and for how much. Loud in both directions for the same reason
    // the floor is: an empty table here is invisible on screen, because the client draws the
    // catalogue from its own copy of the very same file.
    config.commodity = world::commodity::CommodityTable::load(&commodity_path);
    println!("{}", config.commodity.banner());

    // What each job may learn. Loud in both directions: an empty table is invisible on screen
    // until someone clicks + on a skill and is told it is not theirs.
    config.skills = world::skilltable::SkillTable::load(&skills_path);
    println!("{}", config.skills.banner());
    // The per-level cast numbers, from the same file. Loaded separately because the two
    // tables answer different questions - see Config::firstjob.
    config.firstjob = world::firstjob::CombatTable::load(&skills_path);
    println!("{}", config.firstjob.banner());

    config.npcs = world::config::Config::load_npcs(&npcs_path);
    if config.npcs.is_empty() {
        eprintln!(
            "maplecw-world: no NPCs loaded from {} - maps will be empty. Regenerate with: python tools/dump_portals.py",
            npcs_path.display()
        );
    }

    let mob_templates = world::config::load_mob_templates(&mob_templates_path);
    if mob_templates.is_empty() {
        eprintln!(
            "maplecw-world: no mob templates from {} - every mob would spawn on a fallback HP rather than its own. Regenerate with: python tools/dump_mobs.py",
            mob_templates_path.display()
        );
    }
    // Kept whole, so a forced-stat block can be built from the mob's own WZ row rather than
    // from zeros. See Config::mob_templates.
    config.mob_templates = mob_templates.clone();
    let (mob_fields, mob_respawn) = world::config::Config::load_mobs(&mobs_path, &mob_templates);
    config.mobs = mob_fields;
    config.mob_respawn_s = mob_respawn;
    if config.mobs.is_empty() {
        eprintln!(
            "maplecw-world: no mobs loaded from {} - maps will have no monsters. Regenerate with: python tools/dump_portals.py",
            mobs_path.display()
        );
    }
    // On stdout, not stderr. The old warning went to eprintln and therefore to
    // world.log.err, which is not the file anyone opens while a client is running - so a
    // server that had six snails loaded and sent none looked exactly like a server that
    // had none, for a whole launch.
    if !config.send_mobs {
        println!(
            "maplecw-world: MOBS ARE OFF (--no-mobs). {} maps have mobs loaded and none of them will be sent. Maps will look empty; that is this flag, not a bug.",
            config.mobs.len()
        );
    }

    config.equips = world::config::Config::load_equips(&equips_path);
    if config.equips.is_empty() {
        eprintln!(
            "maplecw-world: no equip templates from {} - worn items will have no stats and no upgrade slots. Regenerate with: python tools/dump_equips.py",
            equips_path.display()
        );
    }

    // The client's own 208 scrolls, which the Treasure Scroll guarantees one of. Empty is
    // legal and is said out loud for the same reason the equip table's emptiness is: a
    // Treasure Scroll that finds no scroll to guarantee refuses, and a refusal nobody can
    // explain looks exactly like a broken item.
    config.scrolls = world::config::Config::load_scrolls(&scrolls_path);
    if config.scrolls.is_empty() {
        eprintln!(
            "maplecw-world: no scroll table from {} - the Treasure Scroll will find nothing to guarantee. Regenerate with: python tools/dump_scrolls.py",
            scrolls_path.display()
        );
    }

    // What each summoning sack lets out. Empty is legal and is said out loud: a sack that
    // finds no row refuses, and a refusal nobody can explain looks exactly like the unhandled
    // opcode this replaced.
    config.summon_sacks = world::config::load_summon_sacks(&sacks_path);
    if config.summon_sacks.is_empty() {
        eprintln!(
            "maplecw-world: no summoning-sack table from {} - sacks will refuse. Regenerate with: python tools/dump_summon_sacks.py",
            sacks_path.display()
        );
    }

    // What a chair adds to the idle tick. Empty is legal: the tick stays at its flat base,
    // which is what it did before chairs existed. Said out loud anyway, because "sitting
    // changes nothing" reads on screen as the feature being broken rather than ungenerated.
    config.chairs = world::chairs::load_chairs(&chairs_path);
    if config.chairs.is_empty() {
        eprintln!(
            "maplecw-world: no chair table from {} - sitting will not change idle recovery. Regenerate with: python tools/dump_chairs.py",
            chairs_path.display()
        );
    }

    // **The generated base, read once.** It is not re-read by `!npcreload`, on purpose:
    // `shop_by_template` below is derived from these NPC *names* and sits behind no lock, so
    // a name that changed mid-session would silently re-point a shop. See
    // `world::config::NpcStringTable`.
    config.pet_commands = world::petcommands::PetCommands::load(&pet_commands_path);
    println!(
        "pet commands: {} entries across {} pets from {}",
        config.pet_commands.entries(),
        config.pet_commands.pets(),
        pet_commands_path.display()
    );
    config.npc_strings = world::config::load_npc_strings(&npc_strings_path).into();
    config.npc_strings_path = npc_strings_path.clone();
    config.npc_dialogue_path = npc_dialogue_path.clone();
    if config.npc_strings.is_empty() {
        eprintln!(
            "maplecw-world: no NPC text from {} - NPCs will fall back to placeholder dialogue. Regenerate with: python tools/dump_npcstrings.py",
            npc_strings_path.display()
        );
    }
    // The authored overlay, applied through **the same function `!npcreload` calls**, so the
    // start-up banner and the in-game acknowledgement are the same sentence and cannot drift.
    // Loud on stdout in both directions: an overlay that read nothing is invisible on screen,
    // because an un-amended NPC still says its generated line.
    let npc_reload = world::config::reload_npc_dialogue(&config);
    for line in &npc_reload.refused {
        println!("maplecw-world: npc dialogue: refused {line}");
    }
    println!("maplecw-world: npc dialogue: {}", npc_reload.summary());

    config.quests = world::config::load_quests(&quests_path);
    if config.quests.is_empty() {
        eprintln!(
            "maplecw-world: no quest text from {} - NPCs will fall back to their generic line. Regenerate with: python tools/dump_quests.py",
            quests_path.display()
        );
    }
    // The authored overlay, on top of what the client ships. Roger's quest 1002 opens with
    // `startscript q1002s` and has no `Say."0"`; the body is not in the client's data - all
    // 205 archives and 10021 images were enumerated to establish that, and the name occurs
    // exactly twice, both times as a name. research/quest-scripts.md.
    //
    // Loud on STDOUT, not stderr. `map_exists` was fail-open on an empty field table with
    // its only warning on stderr, where nothing reads it, and that silently removed the
    // `!map` guard for a day.
    let scripted = world::config::overlay_quests(&mut config.quests, &quest_scripts_path);
    if scripted == 0 {
        println!(
            "maplecw-world: quests: no script overlay from {} - the 12 script quests (Roger's 1002 among them) will fall through to the NPC's idle line",
            quest_scripts_path.display()
        );
    } else {
        println!(
            "maplecw-world: quests: {} loaded, {} of them carrying an authored script overlay from {}",
            config.quests.len(),
            scripted,
            quest_scripts_path.display()
        );
    }

    // What mobs drop. Missing is legal and means nothing drops - see DropTables::load for
    // why this degrades rather than refusing to start.
    config.drops = world::droptables::DropTables::load(&drops_path);
    // The breakable boxes and what they give. The owner, 2026-09-13: Pio's quest items come out of
    // Wooden Boxes nobody was placing. Both tables missing is legal and means no boxes.
    config.reactors = world::config::Config::load_reactors(&reactors_path);
    config.reactor_drops = world::droptables::DropTables::load(&reactor_drops_path);
    if config.reactors.is_empty() {
        eprintln!(
            "maplecw-world: no reactors loaded from {} - no breakable boxes on any map. Regenerate with: python tools/dump_portals.py",
            reactors_path.display()
        );
    } else {
        println!(
            "maplecw-world: {} reactor placement(s) across {} map(s) from {}; drops from {}",
            config.reactors.values().map(Vec::len).sum::<usize>(),
            config.reactors.len(),
            reactors_path.display(),
            reactor_drops_path.display()
        );
    }
    config.exp_curve = world::expcurve::ExpCurve::load(&exp_curve_path);
    config.quest_reqs = match std::fs::read_to_string(&quest_reqs_path) {
        Ok(text) => net::quest::QuestRequirementTable::parse(&text),
        Err(e) => {
            println!("maplecw-world: quest requirements: {}: {e} - no quest will progress",
                     quest_reqs_path.display());
            net::quest::QuestRequirementTable::default()
        }
    };
    config.mob_exp = mob_templates.iter().map(|(id, t)| (*id, t.exp)).collect();
    config.mob_attack = mob_templates.iter().map(|(id, t)| (*id, t.pa_damage)).collect();
    for line in &config.exp_curve.problems {
        println!("maplecw-world: exp curve: {line}");
    }
    println!(
        "maplecw-world: exp curve: {} levels; {} mob templates carry an EXP value",
        config.exp_curve.levels(),
        config.mob_exp.values().filter(|e| **e > 0).count()
    );
    // **Say whether the !map guard exists.** `map_exists` is fail-open on an empty table -
    // deliberately, so a tool problem does not turn every warp into a refusal - which means a
    // missing gm-handbook/fields.txt silently removes the guard entirely. That directory is
    // generated and gitignored, so it CAN be missing, and on 2026-08-20 the owner typed `!map 45`
    // and the client died. Nothing in this banner said whether the check was live.
    if config.fields.is_empty() {
        println!(
            "maplecw-world: fields: NONE LOADED from {}. !map will accept ANY id, including              ones with no field image, and the client dies on those. Regenerate with              tools/dump_portals.py.",
            fields_path.display()
        );
    } else {
        println!("maplecw-world: fields: {} maps have a field image", config.fields.len());
    }
    for line in &config.drops.problems {
        println!("maplecw-world: drops: {line}");
    }
    println!(
        "maplecw-world: drops: {} rows over {} mobs, {} global (event) rows",
        config.drops.total_entries(),
        config.drops.mobs_with_drops(),
        config.drops.global().len()
    );

    config.shops =
        world::ShopTable::load(&shops_path, &item_names_path, &item_data_path);
    // The NPC name -> template join. A shop whose name matches nothing can never open, and
    // that is invisible on screen - it looks exactly like an NPC with no shop - so every
    // failure is printed rather than counted.
    //
    // **From the generated BASE, not the live table.** The join is on NPC *names* and this
    // map is a plain `HashMap` behind no lock, read by `session::shop` for the life of the
    // process - so it must be derived from the one half of the table a reload cannot touch.
    // `data/npc-dialogue.txt` refuses `name` rows for the same reason.
    let base_names = config.npc_strings.base();
    let (by_template, shop_problems) =
        world::shops::resolve_npc_templates(&config.shops, &base_names);
    config.shop_by_template = by_template;
    for line in &shop_problems {
        println!("maplecw-world: shops: {line}");
    }
    if config.shop_by_template.is_empty() && !config.shops.shops.is_empty() {
        eprintln!("maplecw-world: NO shop resolves to an NPC template, so no shop can ever open. Regenerate the NPC strings with: python tools/dump_npcstrings.py");
    }
    // LOUDLY, and on stdout. A shop row that did not resolve is an item an NPC will not
    // sell, and the only symptom at the counter is that it is not in the list - which is
    // indistinguishable from it never having been transcribed. The mob default that
    // "silently did nothing" cost a whole client launch for exactly this reason, and its
    // warning was in world.log.err, which nobody opens during a run.
    for problem in &config.shops.problems {
        println!("maplecw-world: shops: {problem}");
    }
    if config.shops.is_empty() {
        eprintln!(
            "maplecw-world: no shops loaded from {} - every NPC shop would be empty. The file is authored source, not generated; if it is missing it was deleted, not un-regenerated",
            shops_path.display()
        );
    } else {
        println!(
            "maplecw-world: {} NPC shops, {} item rows, {} unresolved",
            config.shops.shops.len(),
            config.shops.item_count(),
            config.shops.problems.len()
        );
    }

    // **Which items may only drop for a player who has the quest.** Built after the shop
    // table because the `info/quest` flag arrives with it - one parse of itemdata.txt, so
    // "may not be sold" and "may not drop" cannot disagree about what a quest item is.
    //
    // Both inputs live in gm-handbook/, which is generated and gitignored, so an empty table
    // is a real possibility and it gates NOTHING - the pre-feature behaviour. That is the
    // `!map` guard's failure mode, so this says which one it is out loud rather than leaving
    // it to be inferred from a bag filling up again.
    config.quest_items =
        world::questitems::QuestItems::load(&config.shops.item_data, &quest_reqs_path);
    if config.quest_items.is_armed() {
        println!(
            "maplecw-world: quest items: {} carry info/quest, {} are wanted by a quest, {} by \
             none (those never drop). Only a player with the quest IN PROGRESS is offered one",
            config.quest_items.flagged_count(),
            config.quest_items.mapped_count(),
            config.quest_items.orphan_count(),
        );
    } else {
        eprintln!(
            "maplecw-world: quest items: NOT ARMED - {} flagged items and {} quest rows from {} \
             and {}. Quest items will drop for EVERYBODY, as they did before this filter \
             existed. Regenerate with: python tools/dump_itemdata.py and python tools/dump_quests.py",
            config.quest_items.flagged_count(),
            config.quest_items.mapped_count(),
            item_data_path.display(),
            quest_reqs_path.display(),
        );
    }

    report_binding_readiness();

    // A bare-port channel under --advertise list would answer Change Channel with 0.0.0.0.
    if let Err(e) = config.advertise.validate(&config.channels) {
        eprintln!("{e}\n\n{USAGE}");
        return ExitCode::FAILURE;
    }
    if let Err(e) = world::serve(config) {
        eprintln!("maplecw-world: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// **Say whether this channel could satisfy a bound migration, before anybody turns binding
/// on.**
///
/// A channel server cannot present a session token - the client measurably does not carry one
/// back. What it can do is ask the operating system which process owns the connection and look
/// that process up among the launcher's registered sign-ins
/// (`store::Store::attest_channel_connection`). Two instruments carry that, and **both were
/// written from headers**: the TCP-table row layout in `store::peerowner`, and
/// `GetProcessTimes` in `store::migration::launchtime`. `CLAUDE.md`: *"a constant that came
/// from reading a header is a claim, not a fact"*, and *"an instrument that has never produced
/// a positive is exactly the shape this file keeps warning about"*.
///
/// So both are exercised here, at startup, on this machine, against a socket of exactly the
/// shape the accept path will hand them. This is the control that is *close to the subject* -
/// the distinction the WER episode turned on - and it costs one loopback connection and one
/// `cmd.exe /c exit` per launch, no client run at all.
///
/// # Why the failure text is this long
///
/// Because `--bind-migrations` is the one flag on this project that can produce a **total
/// outage that looks like a crash**: a bound migration presented with nothing is refused, the
/// character never enters the world, and the client sits there. The banner is where the owner finds
/// out that the pid path is dead *before* they flip it, rather than afterwards from a frozen
/// client.
fn report_binding_readiness() {
    println!("maplecw-world: migration binding readiness -");
    match store::peerowner::self_test() {
        Ok(pid) => println!(
            "  OK   this machine's TCP table attributes a loopback connection to the process \
             that opened it (proved against this process, {pid}). A same-machine client the \
             launcher registered CAN satisfy a migration bound to its sign-in."
        ),
        Err(why) => println!(
            "  DEAD the peer-owner lookup does not work here: {why}\n       \
             Nothing can be attested, so with --bind-migrations ON at the login server EVERY \
             character select would be refused and the client would never leave 'Connecting'. \
             Leave it OFF."
        ),
    }
    match store::migration::launchtime::self_test() {
        Ok(what) => println!("  OK   {what} (recorded per attestation, so a recycled process id is readable afterwards)"),
        Err(why) => println!(
            "  WARN process start times are not readable here: {why}\n       \
             Not fatal and not a refusal - it only means an attestation line will not carry \
             the client process's age, which is the datum a recycled-pid investigation needs."
        ),
    }
    println!(
        "  NOTE binding is decided at the LOGIN server (--bind-migrations), not here. This \
         channel logs one ATTESTED / NOT ATTESTED line per migration hello either way, so a \
         run with binding still OFF measures whether turning it on would work."
    );
    println!(
        "  NOTE if no such line appears beside 'MIGRATION HELLO', then \
         session::claim_for_character has NOT been wired to Store::attest_channel_connection \
         and this channel still presents nothing - built, not wired."
    );
}
