//! **The drop-table page, served over HTTP** - channel 0, port 8481 by default.
//!
//! The owner, 2026-10-01: *"serve a http web service on port 8481 of the drop table, a live drop
//! stat for the last 7 days of all the kills along side the drop table with how many player (and
//! how many mob kills) within tracking period"*, with fuzzy search, the global drops and the
//! party-quest box, and *"Drop rate should be set to default whatever the server's current drop
//! rate is."* Then: *"Make this data cached ... at worst can be 30 or 60 minutes out of date."*
//!
//! Three routes, all `GET`, all read-only:
//!
//! | path | what | built |
//! |---|---|---|
//! | `/` | the page (`dropweb.html`, the same file `tools/drops_page.py` embeds) | once, at compile time |
//! | `/tables.json` | every mob's table, the global table, the PQ box, item and mob names | once, at start |
//! | `/live.json` | the server's drop rate and the seven days of kills (`store::killstats`) | at most every [`LIVE_TTL`] |
//!
//! So a page view costs two map lookups and, once per half hour at most, four aggregate queries
//! over at most 168 hour buckets. Requests are answered one at a time on one thread with short
//! timeouts: nothing here can hold a lock a game connection needs.
//!
//! **Nothing on this port authenticates, and nothing on it needs to**: it publishes the drop
//! tables and counts, takes no input but a path, and writes nothing. Bind it to 127.0.0.1 with
//! `--drops-web` to keep it off the network.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::config::Config;

/// The default port.
pub const DEFAULT_PORT: u16 = 8481;
/// How stale `/live.json` may be. The owner allowed 30 to 60 minutes; this is the low end.
pub const LIVE_TTL: Duration = Duration::from_secs(30 * 60);

/// The page. The template is also the one `tools/drops_page.py` embeds into a stand-alone file,
/// which is published without a document head of its own, so the head is added here.
const PAGE: &str = concat!(
    "<!doctype html>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n",
    include_str!("dropweb.html")
);

/// Bind `addr` and serve on a thread of its own. A port that cannot be bound is logged and the
/// channel carries on - the page is a convenience, never a reason for the game not to start.
pub fn spawn(
    addr: SocketAddr,
    store: Arc<store::Store>,
    config: Arc<Config>,
    mob_names: HashMap<u32, String>,
    descs: HashMap<u32, String>,
) {
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            crate::server::log(&format!("drop-table page: could not listen on {addr}: {e} - the game is unaffected"));
            return;
        }
    };
    let tables = tables_json(&config, &mob_names, &descs);
    crate::server::log(&format!(
        "drop-table page: http://{addr}/ ({} KB of tables; live counts refreshed at most every {} min). Read-only, unauthenticated.",
        tables.len() / 1024,
        LIVE_TTL.as_secs() / 60
    ));
    let live: Mutex<Option<(Instant, String)>> = Mutex::new(None);
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let _ = answer(stream, &tables, &live, &store);
        }
    });
}

fn answer(mut stream: TcpStream, tables: &str, live: &Mutex<Option<(Instant, String)>>, store: &store::Store) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;
    // The request line is all that matters; read until the end of the headers or 8 KB.
    let mut buf = [0u8; 8192];
    let mut got = 0;
    while got < buf.len() {
        let n = stream.read(&mut buf[got..])?;
        if n == 0 {
            break;
        }
        got += n;
        if buf[..got].windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    let head = String::from_utf8_lossy(&buf[..got]);
    let mut parts = head.lines().next().unwrap_or("").split_whitespace();
    let (method, target) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let path = target.split('?').next().unwrap_or("");
    if method != "GET" && method != "HEAD" {
        return respond(&mut stream, "405 Method Not Allowed", "text/plain", "GET only\n", "no-store", method == "HEAD");
    }
    let head_only = method == "HEAD";
    match path {
        // The page and the tables change only on a redeploy, but a redeploy must reach players
        // promptly: the page is revalidated every load, the tables kept five minutes.
        "/" | "/index.html" => respond(&mut stream, "200 OK", "text/html; charset=utf-8", PAGE, "no-cache", head_only),
        "/tables.json" => respond(&mut stream, "200 OK", "application/json", tables, "max-age=300", head_only),
        "/live.json" => {
            let body = live_json_cached(live, store);
            respond(&mut stream, "200 OK", "application/json", &body, "max-age=300", head_only)
        }
        _ => respond(&mut stream, "404 Not Found", "text/plain", "not here\n", "no-store", head_only),
    }
}

fn respond(stream: &mut TcpStream, status: &str, kind: &str, body: &str, cache: &str, head_only: bool) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nCache-Control: {cache}\r\n\
         X-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    if !head_only {
        stream.write_all(body.as_bytes())?;
    }
    stream.flush()
}

/// `/live.json`, rebuilt only when the cached copy is older than [`LIVE_TTL`].
fn live_json_cached(cache: &Mutex<Option<(Instant, String)>>, store: &store::Store) -> String {
    let mut held = cache.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((at, body)) = held.as_ref() {
        if at.elapsed() < LIVE_TTL {
            return body.clone();
        }
    }
    let body = live_json(store, store::Store::unix_now());
    *held = Some((Instant::now(), body.clone()));
    body
}

/// The server's drop rate and the seven days, as of `now`.
pub fn live_json(store: &store::Store, now: i64) -> String {
    let rate = store.rates().map(|r| r.get(store::rates::RateKind::Drop)).unwrap_or(store::rates::Rate::NORMAL);
    let stats = store.kill_stats(now).unwrap_or_default();
    let mut s = String::with_capacity(64 * 1024);
    let _ = write!(
        s,
        "{{\"rate\":{},\"built\":{now},\"since\":{},\"players\":{},\"kills\":{},\"mobs\":{{",
        f64::from(rate.per_cent()) / 100.0,
        store::killstats::window_start(now),
        stats.players,
        stats.kills
    );
    let mut mobs: Vec<_> = stats.mobs.iter().collect();
    mobs.sort();
    for (i, (t, (k, p))) in mobs.iter().enumerate() {
        let _ = write!(s, "{}\"{t}\":[{k},{p}]", if i > 0 { "," } else { "" });
    }
    s.push_str("},\"drops\":{");
    let mut by_mob: HashMap<u32, Vec<(u32, u64, u64)>> = HashMap::new();
    for (&(t, item), &(d, q)) in &stats.drops {
        by_mob.entry(t).or_default().push((item, d, q));
    }
    let mut keys: Vec<_> = by_mob.keys().copied().collect();
    keys.sort();
    for (i, t) in keys.iter().enumerate() {
        let mut rows = by_mob[t].clone();
        rows.sort();
        let _ = write!(s, "{}\"{t}\":{{", if i > 0 { "," } else { "" });
        for (j, (item, d, q)) in rows.iter().enumerate() {
            let _ = write!(s, "{}\"{item}\":[{d},{q}]", if j > 0 { "," } else { "" });
        }
        s.push('}');
    }
    s.push_str("}}");
    s
}

/// What a row is, for the page's filter chips - the same split `tools/drops_page.py` uses.
fn category(item: u32) -> &'static str {
    match item {
        0 => "mesos",
        1..=1_999_999 => "equip",
        // The Lucky Day Scroll is a scroll to a player, though its id sits outside the 204 block.
        2_040_000..=2_049_999 | crate::scrolls::LUCKY_DAY => "scroll",
        2_000_000..=2_999_999 => "use",
        3_000_000..=3_999_999 => "setup",
        _ => "etc",
    }
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '<' => out.push_str("\\u003c"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

/// `gm-handbook/itemdesc.txt` - `id, desc`, one per line, line breaks as the two characters
/// `\n` (`tools/dump_names.py`). A missing file is an empty map: the tooltips then show the
/// name and stats without the client's text.
pub fn load_descs(path: &std::path::Path) -> HashMap<u32, String> {
    let Ok(text) = std::fs::read_to_string(path) else { return HashMap::new() };
    text.lines()
        .filter_map(|l| {
            let (id, desc) = l.split_once(", ")?;
            Some((id.trim().parse().ok()?, desc.to_string()))
        })
        .collect()
}

/// **The tooltip's lines below the description**, the way the client draws an item: an
/// equip's level requirement, stats and upgrade slots from its own template, and for the
/// four backported scrolls, how THIS server applies them - their client text is the modern
/// game's (Innocence's mentions Hidden Potentials), and the owner's rules differ
/// (`crate::scrolls`).
fn tip_lines(config: &Config, item: u32) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(e) = config.equips.get(&item) {
        if e.req_level > 0 {
            out.push(format!("REQ LEV : {}", e.req_level));
        }
        for (label, v) in [
            ("STR", e.inc_str),
            ("DEX", e.inc_dex),
            ("INT", e.inc_int),
            ("LUK", e.inc_luk),
            ("MaxHP", e.inc_mhp),
            ("MaxMP", e.inc_mmp),
            ("Weapon Attack", e.inc_wat),
            ("Magic Attack", e.inc_mad),
            ("Weapon Def.", e.inc_pdd),
            ("Magic Def.", e.inc_mdd),
            ("Accuracy", e.inc_acc),
            ("Evasion", e.inc_eva),
            ("Speed", e.inc_speed),
            ("Jump", e.inc_jump),
        ] {
            if v > 0 {
                out.push(format!("{label} : +{v}"));
            }
        }
        if e.tuc > 0 {
            out.push(format!("Number of upgrades available : {}", e.tuc));
        }
        if e.trade_block {
            out.push("Untradeable".to_string());
        }
    }
    let rule: Option<&str> = match crate::scrolls::backported(item) {
        Some((crate::scrolls::SecretsMode::Chaos, _)) => Some(
            "Changes one of the item's own stats by -5 to +5, never 0. Uses an upgrade slot whether it \
             succeeds or fails; a failed slot can be restored with a Clean Slate.",
        ),
        Some((crate::scrolls::SecretsMode::CleanSlate, _)) => Some("Restores one upgrade slot lost to a failed scroll."),
        Some((crate::scrolls::SecretsMode::Innocence, _)) => Some(
            "Returns the item to its original stats and all of its upgrade slots. Does not use an upgrade slot.",
        ),
        None if item == crate::scrolls::LUCKY_DAY => Some(
            "The next scroll used on the item succeeds and cannot destroy it - Chaos, Clean Slate and \
             Innocence included. Does not use an upgrade slot.",
        ),
        None => None,
    };
    if let Some(rule) = rule {
        let pct = crate::scrolls::backported(item).map_or(100, |(_, p)| p);
        out.push(format!("Success rate: {pct}%"));
        out.push(format!("On this server: {rule}"));
    }
    out
}

/// **Where each monster can be met**: the maps whose spawn list places it, each with how many
/// spawn points it has there, most first; and the ways it appears without a spawn point.
///
/// The owner, 2026-10-01: *"If those monsters do not spawn yet, can we make sure that they are
/// hidden from the drop table ... can we also list out the maps that they are present on, sorted
/// by number of spawns for that monster on that map?"* 57 of the 170 templates with a drop
/// table are on no map in this client - Ludibrium is not in it at all - and a monster with
/// neither a map nor another way in is left off the page.
///
/// The other ways in are the ones the server itself spawns: the ship invasion
/// (`boat::CRIMSON_BALROG`) and the summoning sacks (`Config::summon_sacks`). A monster another
/// monster summons by skill is not listed - King Slime's Slimes are the only such summon here,
/// and Slimes are on maps anyway.
pub struct Whereabouts {
    /// `template -> [(map, spawn points)]`, most spawn points first, then by map id.
    pub maps: HashMap<u32, Vec<(u32, u32)>>,
    /// `template -> other ways it appears`, as lines the page shows.
    pub also: HashMap<u32, Vec<String>>,
}

impl Whereabouts {
    pub fn of(config: &Config) -> Self {
        let mut counts: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
        for (map, points) in &config.mobs {
            for p in points {
                *counts.entry(p.template_id).or_default().entry(*map).or_default() += 1;
            }
        }
        let maps = counts
            .into_iter()
            .map(|(t, per_map)| {
                let mut v: Vec<(u32, u32)> = per_map.into_iter().collect();
                v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                (t, v)
            })
            .collect();
        let mut also: HashMap<u32, Vec<String>> = HashMap::new();
        also.entry(crate::boat::CRIMSON_BALROG)
            .or_default()
            .push("Invades the ship between Ellinia and Orbis".to_string());
        let mut sacks: Vec<(&u32, &crate::config::SummonSack)> = config.summon_sacks.iter().collect();
        sacks.sort_by_key(|(id, _)| **id);
        for (sack, s) in sacks {
            let name = config.item_names.get(sack).cloned().unwrap_or_else(|| format!("Item {sack}"));
            let mut seen = Vec::new();
            for t in &s.mobs {
                if !seen.contains(t) {
                    seen.push(*t);
                    also.entry(*t).or_default().push(format!("Summoned by {name}"));
                }
            }
        }
        Whereabouts { maps, also }
    }

    /// Can a player meet this monster at all?
    pub fn exists(&self, template: u32) -> bool {
        self.maps.contains_key(&template) || self.also.contains_key(&template)
    }
}

/// `/tables.json`: the shape the page reads - `mobs` (`id`, `name`, `level`, `rows` of
/// `[item, ppm, min, max, fixed]`, `maps` of `[map, name, spawn points]`, `also`), `global`,
/// and `items` (`id -> [name, kind, quest, desc, [tooltip lines]]`). A monster that appears
/// nowhere ([`Whereabouts`]) is left out. A mob
/// with no meso row gets the server's level-scaled default as a `fixed` row: always dropped,
/// not scaled by the rate (`session/combat.rs`). The First Time Together box is the last
/// "mob", `kind` set, one line per prize at `1 / LINES`.
pub fn tables_json(config: &Config, mob_names: &HashMap<u32, String>, descs: &HashMap<u32, String>) -> String {
    let mut items: HashMap<u32, ()> = HashMap::new();
    let mut s = String::with_capacity(256 * 1024);
    s.push_str("{\"mobs\":[");
    let mut mobs: Vec<(u32, u32, &[crate::droptables::DropEntry])> = config
        .drops
        .mobs()
        .into_iter()
        .map(|(t, rows)| (config.mob_templates.get(&t).map_or(0, |m| m.level), t, rows))
        .collect();
    mobs.sort_by_key(|(level, t, _)| (*level, *t));
    let whereabouts = Whereabouts::of(config);
    mobs.retain(|(_, t, _)| whereabouts.exists(*t));
    let row = |s: &mut String, first: &mut bool, item: u32, ppm: u32, lo: u32, hi: u32, fixed: bool| {
        let _ = write!(s, "{}[{item},{ppm},{lo},{hi},{}]", if *first { "" } else { "," }, u8::from(fixed));
        *first = false;
    };
    for (i, (level, t, rows)) in mobs.iter().enumerate() {
        let name = mob_names.get(t).cloned().unwrap_or_else(|| format!("Mob {t}"));
        let _ = write!(s, "{}{{\"id\":{t},\"name\":\"{}\",\"level\":{level},\"maps\":[", if i > 0 { "," } else { "" }, esc(&name));
        for (j, (map, n)) in whereabouts.maps.get(t).map(Vec::as_slice).unwrap_or(&[]).iter().enumerate() {
            let map_name = config.map_names.get(map).cloned().unwrap_or_else(|| format!("Map {map}"));
            let _ = write!(s, "{}[{map},\"{}\",{n}]", if j > 0 { "," } else { "" }, esc(&map_name));
        }
        s.push_str("],\"also\":[");
        for (j, line) in whereabouts.also.get(t).map(Vec::as_slice).unwrap_or(&[]).iter().enumerate() {
            let _ = write!(s, "{}\"{}\"", if j > 0 { "," } else { "" }, esc(line));
        }
        s.push_str("],\"rows\":[");
        let mut first = true;
        if !config.drops.has_meso_row(*t) {
            if let Some((lo, hi)) = crate::droptables::level_meso_range(*level) {
                row(&mut s, &mut first, 0, crate::droptables::PER_MILLION, lo, hi, true);
            }
        }
        for e in rows.iter() {
            row(&mut s, &mut first, e.item_id, e.chance_ppm, e.min_qty, e.max_qty, false);
            items.insert(e.item_id, ());
        }
        s.push_str("]}");
    }
    // The party quest's reward box: one line per prize, equal odds, untouched by the rate.
    let share = crate::droptables::PER_MILLION / crate::magicbox::LINES as u32;
    let _ = write!(
        s,
        ",{{\"id\":\"box\",\"name\":\"Companion's Magic Box\",\"level\":0,\"kind\":\"First Time Together reward\",\"rows\":["
    );
    let mut first = true;
    for n in 0..crate::magicbox::LINES {
        let (item, q) = crate::magicbox::line(n);
        row(&mut s, &mut first, item, share, u32::from(q), u32::from(q), true);
        items.insert(item, ());
    }
    s.push_str("]}],\"global\":[");
    let mut first = true;
    for e in config.drops.global() {
        row(&mut s, &mut first, e.item_id, e.chance_ppm, e.min_qty, e.max_qty, false);
        items.insert(e.item_id, ());
    }
    s.push_str("],\"items\":{");
    let mut ids: Vec<u32> = items.into_keys().filter(|i| *i != 0).collect();
    ids.sort();
    for (i, id) in ids.iter().enumerate() {
        let name = config.item_names.get(id).cloned().unwrap_or_else(|| format!("Item {id}"));
        let lines: Vec<String> = tip_lines(config, *id).iter().map(|l| format!("\"{}\"", esc(l))).collect();
        let _ = write!(
            s,
            "{}\"{id}\":[\"{}\",\"{}\",{},\"{}\",[{}]]",
            if i > 0 { "," } else { "" },
            esc(&name),
            category(*id),
            u8::from(config.quest_items.is_quest_item(*id)),
            esc(descs.get(id).map_or("", String::as_str)),
            lines.join(",")
        );
    }
    s.push_str("}}");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        let drops = crate::droptables::DropTables::parse(
            "2 | 4000001 | 60 | 1 | 1 | 9 | Snail Shell\n2 | 0 | 40 | 4 | 6 | 1 | mesos\n\
             7 | 4000004 | 60 | 1 | 1 | 0 | Squishy \"Liquid\"\n* | 4031065 | 0.5 | 1 | 1 | 0 | Scroll of Secrets\n",
        );
        let mut c = Config { drops, ..Config::default() };
        // Snail (2) on two maps, three points on one; template 7 on one; template 9 nowhere.
        let point = |id, t| net::mob::FieldMob::new(id, t, 0, 0, 1, 10);
        c.mobs.insert(100, vec![point(1, 2), point(2, 2), point(3, 2), point(4, 7)]);
        c.mobs.insert(101, vec![point(1, 2)]);
        c.map_names.insert(100, "Snail Garden".into());
        c.item_names.insert(4_000_001, "Snail Shell".into());
        c.item_names.insert(4_000_004, "Squishy \"Liquid\"".into());
        c.mob_templates.insert(7, crate::config::MobTemplate { level: 6, ..Default::default() });
        c
    }

    /// The tables are the server's own rows, a mob with no meso row gets the level default as a
    /// fixed row, the PQ box and the global row are there, and a quote in a name is escaped.
    #[test]
    fn the_tables_carry_every_source_and_escape_names() {
        let mut names = HashMap::new();
        names.insert(2, "Snail".to_string());
        let mut descs = HashMap::new();
        descs.insert(4_000_001, "A shell from a snail.\\nAn etc item.".to_string());
        let j = tables_json(&config(), &names, &descs);
        assert!(
            j.contains("{\"id\":2,\"name\":\"Snail\",\"level\":0,\"maps\":[[100,\"Snail Garden\",3],[101,\"Map 101\",1]],\"also\":[],\"rows\":[[4000001,600000,1,1,0],[0,400000,4,6,0]]}"),
            "most spawn points first: {j}"
        );
        assert!(j.contains("\"id\":7,\"name\":\"Mob 7\",\"level\":6,\"maps\":[[100,\"Snail Garden\",1]],\"also\":[],\"rows\":[[0,1000000,10,13,1],"), "level 6: 10-13 mesos, fixed: {j}");
        assert!(j.contains("\"global\":[[4031065,5000,1,1,0]]"));
        assert!(j.contains("\"kind\":\"First Time Together reward\""));
        assert!(j.contains(&format!("[2043701,{},1,1,1]", 1_000_000 / crate::magicbox::LINES as u32)), "the Wand scroll in the box");
        assert!(j.contains("\"4000004\":[\"Squishy \\\"Liquid\\\"\",\"etc\",0,\"\",[]]"), "{j}");
        // The client's description, its escape kept for the page to break the line on.
        assert!(j.contains("\"4000001\":[\"Snail Shell\",\"etc\",0,\"A shell from a snail.\\\\nAn etc item.\",[]]"), "{j}");
        assert!(!j.contains("<"), "no raw angle bracket can close the page's script");
    }

    /// **A monster no map spawns is left off the page** - unless the server brings it some other
    /// way: the ship's Crimson Balrog, and anything a summoning sack calls.
    #[test]
    fn a_monster_that_appears_nowhere_is_hidden_and_the_ways_in_are_listed() {
        let mut c = config();
        c.drops = crate::droptables::DropTables::parse(
            "2 | 4000001 | 60 | 1 | 1 | 0 | Snail Shell\n9 | 4000001 | 60 | 1 | 1 | 0 | Nowhere\n\
             700005 | 4000001 | 60 | 1 | 1 | 0 | Balrog\n123 | 4000001 | 60 | 1 | 1 | 0 | Sacked\n",
        );
        c.summon_sacks.insert(2_100_009, crate::config::SummonSack { mobs: vec![123, 123] });
        c.item_names.insert(2_100_009, "Test Sack".into());
        let j = tables_json(&c, &HashMap::new(), &HashMap::new());
        assert!(j.contains("\"id\":2,"), "on a map: shown");
        assert!(!j.contains("\"id\":9,"), "on no map and no other way in: hidden - {j}");
        assert!(j.contains("\"also\":[\"Invades the ship between Ellinia and Orbis\"]"), "{j}");
        assert!(j.contains("\"id\":123,\"name\":\"Mob 123\",\"level\":0,\"maps\":[],\"also\":[\"Summoned by Test Sack\"]"), "once, though the sack lists it twice: {j}");
    }

    /// **The tooltip lines**: an equip shows its requirement, stats and slots the way the client
    /// does; each backported scroll states this server's rule and rate; anything else has none.
    #[test]
    fn tooltips_carry_equip_stats_and_the_backported_rules() {
        let mut c = config();
        c.equips.insert(
            1_002_007,
            crate::config::EquipTemplate { tuc: 7, inc_pdd: 6, inc_dex: 1, req_level: 10, ..Default::default() },
        );
        assert_eq!(
            tip_lines(&c, 1_002_007),
            vec!["REQ LEV : 10", "DEX : +1", "Weapon Def. : +6", "Number of upgrades available : 7"]
        );
        let chaos = tip_lines(&c, 2_049_100);
        assert_eq!(chaos[0], "Success rate: 60%");
        assert!(chaos[1].contains("never 0"), "{chaos:?}");
        assert_eq!(tip_lines(&c, 2_049_003)[0], "Success rate: 20%");
        assert_eq!(tip_lines(&c, 2_049_190)[0], "Success rate: 70%");
        assert!(tip_lines(&c, crate::scrolls::LUCKY_DAY)[1].contains("cannot destroy"));
        assert!(tip_lines(&c, 4_000_001).is_empty());
        assert_eq!(category(crate::scrolls::LUCKY_DAY), "scroll", "a scroll to a player, outside the 204 block");
        assert_eq!(category(2_000_000), "use");
    }

    /// The live counts are the store's, with the server's drop rate.
    #[test]
    fn live_json_has_the_rate_and_the_week() {
        let store = store::Store::open_in_memory().unwrap();
        let now = 1_800_000_000;
        let mut b = store::killstats::KillBatch::default();
        b.note(now, 2, 200, &[(0, 5)]);
        b.note(now, 2, 201, &[]);
        store.flush_kill_stats(&b).unwrap();
        let j = live_json(&store, now);
        assert!(j.starts_with("{\"rate\":1,"), "{j}");
        assert!(j.contains("\"players\":2,\"kills\":2,\"mobs\":{\"2\":[2,2]},\"drops\":{\"2\":{\"0\":[1,5]}}}"), "{j}");
    }

    /// **The cache is the owner's "at worst 30 or 60 minutes out of date"**: kills written after
    /// the first build are not seen until the copy is older than [`LIVE_TTL`], and then they are.
    #[test]
    fn live_counts_are_rebuilt_only_when_the_copy_is_stale() {
        let store = store::Store::open_in_memory().unwrap();
        let cache = Mutex::new(None);
        let first = live_json_cached(&cache, &store);
        assert!(first.contains("\"kills\":0"));
        let mut b = store::killstats::KillBatch::default();
        b.note(store::Store::unix_now(), 2, 200, &[]);
        store.flush_kill_stats(&b).unwrap();
        assert_eq!(live_json_cached(&cache, &store), first, "inside the TTL: the cached copy, no query");
        let stale = Instant::now().checked_sub(LIVE_TTL + Duration::from_secs(1)).expect("a clock that old");
        cache.lock().unwrap().as_mut().unwrap().0 = stale;
        assert!(live_json_cached(&cache, &store).contains("\"kills\":1"), "past it: rebuilt");
    }

    /// Through a real socket: the page, both JSON routes, a 404 and a refused POST.
    #[test]
    fn the_routes_answer_over_http() {
        let probe = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = probe.local_addr().unwrap();
        drop(probe);
        spawn(addr, Arc::new(store::Store::open_in_memory().unwrap()), Arc::new(config()), HashMap::new(), HashMap::new());
        let get = |req: &str| {
            let mut s = TcpStream::connect(addr).unwrap();
            s.write_all(req.as_bytes()).unwrap();
            let mut out = String::new();
            s.read_to_string(&mut out).unwrap();
            out
        };
        let page = get("GET / HTTP/1.1\r\nHost: x\r\n\r\n");
        assert!(page.starts_with("HTTP/1.1 200 OK") && page.contains("<title>"), "{}", &page[..200.min(page.len())]);
        assert!(get("GET /tables.json HTTP/1.1\r\n\r\n").contains("\"global\":[[4031065"));
        assert!(get("GET /live.json?x=1 HTTP/1.1\r\n\r\n").contains("\"kills\":0"));
        assert!(get("GET /etc/passwd HTTP/1.1\r\n\r\n").starts_with("HTTP/1.1 404"));
        assert!(get("POST / HTTP/1.1\r\n\r\n").starts_with("HTTP/1.1 405"));
    }
}
