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
pub fn spawn(addr: SocketAddr, store: Arc<store::Store>, config: Arc<Config>, mob_names: HashMap<u32, String>) {
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            crate::server::log(&format!("drop-table page: could not listen on {addr}: {e} - the game is unaffected"));
            return;
        }
    };
    let tables = tables_json(&config, &mob_names);
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
        "/" | "/index.html" => respond(&mut stream, "200 OK", "text/html; charset=utf-8", PAGE, "max-age=300", head_only),
        "/tables.json" => respond(&mut stream, "200 OK", "application/json", tables, "max-age=3600", head_only),
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
        2_040_000..=2_049_999 => "scroll",
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

/// `/tables.json`: the shape the page reads - `mobs` (`id`, `name`, `level`, `rows` of
/// `[item, ppm, min, max, fixed]`), `global`, and `items` (`id -> [name, kind, quest]`). A mob
/// with no meso row gets the server's level-scaled default as a `fixed` row: always dropped,
/// not scaled by the rate (`session/combat.rs`). The First Time Together box is the last
/// "mob", `kind` set, one line per prize at `1 / LINES`.
pub fn tables_json(config: &Config, mob_names: &HashMap<u32, String>) -> String {
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
    let row = |s: &mut String, first: &mut bool, item: u32, ppm: u32, lo: u32, hi: u32, fixed: bool| {
        let _ = write!(s, "{}[{item},{ppm},{lo},{hi},{}]", if *first { "" } else { "," }, u8::from(fixed));
        *first = false;
    };
    for (i, (level, t, rows)) in mobs.iter().enumerate() {
        let name = mob_names.get(t).cloned().unwrap_or_else(|| format!("Mob {t}"));
        let _ = write!(s, "{}{{\"id\":{t},\"name\":\"{}\",\"level\":{level},\"rows\":[", if i > 0 { "," } else { "" }, esc(&name));
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
        let _ = write!(
            s,
            "{}\"{id}\":[\"{}\",\"{}\",{}]",
            if i > 0 { "," } else { "" },
            esc(&name),
            category(*id),
            u8::from(config.quest_items.is_quest_item(*id))
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
        let j = tables_json(&config(), &names);
        assert!(j.contains("{\"id\":2,\"name\":\"Snail\",\"level\":0,\"rows\":[[4000001,600000,1,1,0],[0,400000,4,6,0]]}"), "{j}");
        assert!(j.contains("\"id\":7,\"name\":\"Mob 7\",\"level\":6,\"rows\":[[0,1000000,10,13,1],"), "level 6: 10-13 mesos, fixed: {j}");
        assert!(j.contains("\"global\":[[4031065,5000,1,1,0]]"));
        assert!(j.contains("\"kind\":\"First Time Together reward\""));
        assert!(j.contains(&format!("[2043701,{},1,1,1]", 1_000_000 / crate::magicbox::LINES as u32)), "the Wand scroll in the box");
        assert!(j.contains("\"4000004\":[\"Squishy \\\"Liquid\\\"\",\"etc\",0]"), "{j}");
        assert!(!j.contains("<"), "no raw angle bracket can close the page's script");
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
        spawn(addr, Arc::new(store::Store::open_in_memory().unwrap()), Arc::new(config()), HashMap::new());
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
