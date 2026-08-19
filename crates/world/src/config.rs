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

    /// Play the game: answer the migration hello, portal walks, and field entry.
    ///
    /// **The name is a fossil and the doc that went with it was badly stale.** It was written
    /// when this flag sent the fixed head of a `SetField` and nothing after, and said "it
    /// cannot put a character in a map - `characterData` is `0` and the branch that does
    /// carry a character calls an 18525-byte record decoder nobody has read yet". All of that
    /// has been untrue since 2026-08-19: the record decoder is read, a character stands on
    /// map 1, and this flag now drives everything the channel does.
    ///
    /// With it on the server answers:
    ///
    /// | in | out |
    /// |---|---|
    /// | `0x007D` migration hello | `SetField` with a full character record on the map |
    /// | `0x00D1` transfer field | `SetField` for the portal's target map and arrival portal |
    /// | `0x00DC` field entered | every NPC on the map, then a `UserAvatarModified` attempt |
    ///
    /// **Still off by default**, because it is the whole game path and nothing on it
    /// authenticates anybody. See `STATUS.md` NEXT GOALS.
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
    /// `(map, portal name)` -> `(target map, target portal name)`.
    pub portals: HashMap<(u32, String), (u32, String)>,

    /// `(map, portal name)` -> that portal's **index** on its own map.
    ///
    /// Separate from [`Self::portals`] because arrival needs the reverse direction: the
    /// source portal names its destination portal (`tn`), and the stat block wants that
    /// portal's index. Spawn points are in here too - they lead nowhere but are perfectly
    /// valid arrival points, and `sp` is what an ordinary login uses.
    pub portal_index: HashMap<(u32, String), u8>,

    /// Every NPC standing on every map, keyed by map id.
    ///
    /// Also generated from the client's `Map.wz` by `tools/dump_portals.py`, out of each
    /// field's `life` node. The client **cannot** spawn these itself - its field loader walks
    /// `life` only to preload art - so they are the server's to send, after every `SetField`.
    pub npcs: HashMap<u32, Vec<net::opcode::FieldNpc>>,

    /// Every map id that has a field image in `Map.wz`.
    ///
    /// The authoritative "does this map exist" list, and **not** the same as `String.wz`'s
    /// name table: a survey of this client found **12 ids named but absent** and **6 present
    /// but unnamed**. Sending a character to an id with no field image strands it, and one
    /// with no name entry can take the client into a branch that does not return
    /// (`research/map1-exists.md`).
    ///
    /// Empty means "unknown", not "nothing exists" - see [`Config::map_exists`].
    pub fields: std::collections::HashSet<u32>,
}

impl Config {
    /// Load `map, portal, target map, target portal` rows, ignoring blanks and `#` comments.
    ///
    /// A missing file is **not** an error: the server runs without portals and logs each
    /// unresolved request. A malformed line is skipped rather than aborting startup, because
    /// this file is regenerated from game data and one bad row should not stop a test run.
    #[allow(clippy::type_complexity)]
    pub fn load_portals(
        path: &std::path::Path,
    ) -> (HashMap<(u32, String), (u32, String)>, HashMap<(u32, String), u8>) {
        let (mut links, mut index) = (HashMap::new(), HashMap::new());
        let Ok(text) = std::fs::read_to_string(path) else { return (links, index) };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // map, index, name, target map, target portal
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            if f.len() < 5 {
                continue;
            }
            let (Ok(map), Ok(idx), Ok(target)) =
                (f[0].parse::<u32>(), f[1].parse::<u32>(), f[3].parse::<u32>())
            else {
                continue;
            };
            // The stat block's portal field is one byte, so an index past 255 cannot be
            // expressed. Skip rather than truncate - a wrong portal is worse than the spawn.
            if let Ok(idx) = u8::try_from(idx) {
                index.insert((map, f[2].to_string()), idx);
            }
            if target != 0 {
                links.insert((map, f[2].to_string()), (target, f[4].to_string()));
            }
        }
        (links, index)
    }

    /// Is this a map the client can actually load?
    ///
    /// **An empty table answers `true` for everything**, deliberately. The table is generated
    /// game data and a missing file must not turn every warp into a refusal - that would fail
    /// closed on a tool problem rather than a real one. When it is loaded it is exact.
    pub fn map_exists(&self, map: u32) -> bool {
        self.fields.is_empty() || self.fields.contains(&map)
    }

    /// Load one map id per line, ignoring blanks and `#` comments.
    pub fn load_fields(path: &std::path::Path) -> std::collections::HashSet<u32> {
        let mut out = std::collections::HashSet::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Ok(id) = line.parse::<u32>() {
                out.insert(id);
            }
        }
        out
    }

    /// Load `map, template, x, cy, fh, rx0, rx1, f` rows into per-map NPC lists.
    ///
    /// **Object ids are assigned here**, sequentially within each map. They only have to be
    /// unique on the field: the client's pool keys on the id, and a repeat makes its handler
    /// return after four bytes and silently drop the NPC - which would show as one NPC where
    /// two should stand, with nothing in any log.
    pub fn load_npcs(path: &std::path::Path) -> HashMap<u32, Vec<net::opcode::FieldNpc>> {
        let mut out: HashMap<u32, Vec<net::opcode::FieldNpc>> = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            if f.len() < 8 {
                continue;
            }
            let n = |i: usize| f[i].parse::<i64>().ok();
            let (Some(map), Some(template), Some(x), Some(cy), Some(fh), Some(rx0), Some(rx1),
                 Some(fl)) = (n(0), n(1), n(2), n(3), n(4), n(5), n(6), n(7))
            else {
                continue;
            };
            let list = out.entry(map as u32).or_default();
            let object_id = 1000 + list.len() as u32;
            list.push(net::opcode::FieldNpc {
                object_id,
                template_id: template as u32,
                x: x as i16,
                cy: cy as i16,
                fh: fh as u16,
                rx0: rx0 as i16,
                rx1: rx1 as i16,
                f: fl as u8,
            });
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
            portal_index: HashMap::new(),
            npcs: HashMap::new(),
            fields: std::collections::HashSet::new(),
        }
    }
}
