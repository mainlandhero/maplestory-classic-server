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
  --channels A,B,. one address per channel, in channel order. Needed only to answer
                   Change Channel (0x00D2); without it the request is refused with a
                   message rather than ignored.
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
  --footholds PATH  map floor geometry, from tools/dump_portals.py
  --consumables PATH  what potions restore, from tools/dump_itemdata.py
  --shops PATH     the authored NPC shop file      (default data/shops.txt)
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
    let mut portals_path = PathBuf::from("gm-handbook/portals.txt");
    let mut npcs_path = PathBuf::from("gm-handbook/npcs.txt");
    let mut fields_path = PathBuf::from("gm-handbook/fields.txt");
    let mut footholds_path = PathBuf::from("gm-handbook/footholds.txt");
    let mut consumables_path = PathBuf::from("gm-handbook/consumables.txt");
    let mut mobs_path = PathBuf::from("gm-handbook/mobs.txt");
    let mut equips_path = PathBuf::from("gm-handbook/equips.txt");
    let mut mob_templates_path = PathBuf::from("gm-handbook/mobtemplates.txt");
    let mut npc_strings_path = PathBuf::from("gm-handbook/npcstrings.txt");
    let mut quests_path = PathBuf::from("gm-handbook/questlines.txt");
    // Authored source like data/shops.txt: the script bodies are not in the client at all.
    let mut quest_scripts_path = PathBuf::from("data/quest-scripts.txt");
    // Authored source, not generated data - the only path here that is not gm-handbook/.
    let mut shops_path = PathBuf::from("data/shops.txt");
    let mut item_names_path = PathBuf::from("gm-handbook/items.txt");
    let mut item_data_path = PathBuf::from("gm-handbook/itemdata.txt");
    // Authored source like data/shops.txt: it cannot be regenerated from the client.
    let mut drops_path = PathBuf::from("data/drops.txt");
    let mut exp_curve_path = PathBuf::from("data/exp-curve.txt");
    let mut quest_reqs_path = PathBuf::from("gm-handbook/questreq.txt");
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
            "--channels" => value().and_then(|v| {
                v.split(',')
                    .map(|a| a.trim().parse::<std::net::SocketAddrV4>())
                    .collect::<Result<Vec<_>, _>>()
                    .map(|c| config.channels = c)
                    .map_err(|e| format!("--channels {v}: {e} (IPv4 host:port, comma separated)"))
            }),
            "--set-field-probe" => {
                config.set_field_probe = true;
                Ok(())
            }
            "--portals" => value().map(|v| portals_path = PathBuf::from(v)),
            "--npcs" => value().map(|v| npcs_path = PathBuf::from(v)),
            "--fields" => value().map(|v| fields_path = PathBuf::from(v)),
            "--footholds" => value().map(|v| footholds_path = PathBuf::from(v)),
            "--consumables" => value().map(|v| consumables_path = PathBuf::from(v)),
            "--mobs-file" => value().map(|v| mobs_path = PathBuf::from(v)),
            "--equips" => value().map(|v| equips_path = PathBuf::from(v)),
            "--mob-templates" => value().map(|v| mob_templates_path = PathBuf::from(v)),
            "--npc-strings" => value().map(|v| npc_strings_path = PathBuf::from(v)),
            "--quests" => value().map(|v| quests_path = PathBuf::from(v)),
            "--quest-scripts" => value().map(|v| quest_scripts_path = PathBuf::from(v)),
            "--shops" => value().map(|v| shops_path = PathBuf::from(v)),
            "--item-names" => value().map(|v| item_names_path = PathBuf::from(v)),
            "--item-data" => value().map(|v| item_data_path = PathBuf::from(v)),
            "--drops" => value().map(|v| drops_path = PathBuf::from(v)),
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
            "--mob-limit" => value().and_then(|v| {
                v.parse()
                    .map(|n: usize| config.mob_limit = Some(n))
                    .map_err(|e| format!("--mob-limit {v}: {e}"))
            }),
            // Re-enable OpenShop. OFF by default: it kills this client, because the window
            // it builds needs a WZ image this client does not ship. See Config::send_shop
            // and research/npc-shop-crash2.md.
            "--shop" => {
                config.send_shop = true;
                Ok(())
            }
            // The shop counter's blast-radius control, and it is now known NOT to be the
            // variable: one correctly-formed row killed the client exactly as twelve did,
            // because the client dies before reading any row byte. Kept for the next time
            // rows are a suspect.
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
    (config.portals, config.portal_index) = world::config::Config::load_portals(&portals_path);
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

    config.npc_strings = world::config::load_npc_strings(&npc_strings_path);
    if config.npc_strings.is_empty() {
        eprintln!(
            "maplecw-world: no NPC text from {} - NPCs will fall back to placeholder dialogue. Regenerate with: python tools/dump_npcstrings.py",
            npc_strings_path.display()
        );
    }

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
    let (by_template, shop_problems) =
        world::shops::resolve_npc_templates(&config.shops, &config.npc_strings);
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

    if let Err(e) = world::serve(config) {
        eprintln!("maplecw-world: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
