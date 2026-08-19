//! What one channel server needs to know before it can listen.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;

/// One channel of one world.
///
/// A channel is a process, not a thread: the login server hands the client an address and
/// the client connects to it, so every channel needs its own listener and its own
/// advertised address. `world_id` and `channel_id` are here so the log says which channel
/// a line belongs to, and so a migration minted for channel 1 is not claimed by channel 2.
#[derive(Debug, Clone)]
pub struct Config {
    /// What this channel listens on.
    pub bind: SocketAddr,

    /// The SQLite file. Shared with the login server - that is how the migration handoff
    /// crosses the process boundary.
    pub db_path: PathBuf,

    pub world_id: u32,
    pub channel_id: u32,

    /// Answer the migration hello with the fixed head of a `SetField`, and nothing after.
    ///
    /// **Off by default, and it should stay off except during a run that is measuring
    /// something.** It cannot put a character in a map - `characterData` is `0` and the
    /// branch that does carry a character calls an 18525-byte record decoder nobody has
    /// read yet. What it is for is one question static analysis cannot answer: `SetField`'s
    /// handler has two silent early returns, so "the client did nothing" and "the client
    /// never received it" look identical from here. Send this with a watch armed on
    /// `142097f80` and the two become distinguishable.
    ///
    /// See `research/msexe-stage-setfield.md`.
    pub set_field_probe: bool,

    /// Where every portal leads, keyed by `(map, portal name)`.
    ///
    /// Generated from the client's own `Map.wz` by `tools/dump_portals.py` - the data is the
    /// client's, not ours to invent. Empty if the file is missing, in which case the server
    /// still answers a transfer request but re-sends the current map and says so, rather
    /// than guessing a destination.
    ///
    /// This replaced a hand-typed two-row stub that let a character walk from map 1 to map
    /// 10 and then stranded it: every portal out of map 10 was "not in the table".
    pub portals: HashMap<(u32, String), u32>,
}

impl Config {
    /// Load `map, portal, target map, target portal` rows, ignoring blanks and `#` comments.
    ///
    /// A missing file is **not** an error: the server runs without portals and logs each
    /// unresolved request. A malformed line is skipped rather than aborting startup, because
    /// this file is regenerated from game data and one bad row should not stop a test run.
    pub fn load_portals(path: &std::path::Path) -> HashMap<(u32, String), u32> {
        let mut out = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut f = line.split(',').map(str::trim);
            let (Some(map), Some(name), Some(target)) = (f.next(), f.next(), f.next()) else {
                continue;
            };
            if let (Ok(map), Ok(target)) = (map.parse::<u32>(), target.parse::<u32>()) {
                out.insert((map, name.to_string()), target);
            }
        }
        out
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind: "127.0.0.1:8485".parse().expect("a literal address"),
            db_path: PathBuf::from("maplecw.db"),
            world_id: 0,
            channel_id: 0,
            set_field_probe: false,
            portals: HashMap::new(),
        }
    }
}
