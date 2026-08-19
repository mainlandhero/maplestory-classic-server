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
    /// Every map's mobs, keyed by map id, from `gm-handbook/mobs.txt`.
    ///
    /// Server-sent for the same reason NPCs are: the client's field loader walks the WZ
    /// `life` node only to preload `Mob/%07d.img` art. 9928 spawns across 289 maps.
    pub mobs: HashMap<u32, Vec<net::mob::FieldMob>>,
    /// Every equip's template values, keyed by item id, from `gm-handbook/equips.txt`.
    ///
    /// The character record carries an item's stats and upgrade slots per *instance*, and a
    /// real server fills them from the template when the item is created. An empty table is
    /// not fatal - items are still sent, just bare - so a missing file degrades to exactly
    /// the behaviour confirmed on screen on 2026-08-19.
    pub equips: HashMap<u32, EquipTemplate>,
    /// Whether to actually send them. **Default `false`, and that is a measurement.**
    ///
    /// The run of 2026-08-19 faulted the client at `0x141c810b0` on the **first** `0x03C6`,
    /// after two `0x044F` NPCs had dispatched cleanly. The mob body is wrong, and it is the
    /// only one of the four builds that is - so the flag exists to get the other three back
    /// in front of a client without waiting for the mob layout to be fixed.
    ///
    /// Turn it on with `--mobs`, and only when the mob body is the variant under test.
    pub send_mobs: bool,

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
    ///
    /// **Having a field image is necessary and not sufficient**, which was measured the
    /// expensive way: see [`MAPS_THAT_KILL_THE_CLIENT`].
    pub fn map_exists(&self, map: u32) -> bool {
        if MAPS_THAT_KILL_THE_CLIENT.contains(&map) {
            return false;
        }
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

    /// Every equip's template values, from `tools/dump_equips.py`'s `equips.txt`.
    ///
    /// Column order is the file's header and is fixed by the generator; a row with the wrong
    /// number of columns is skipped rather than partially read, because a silently
    /// half-filled template would put a wrong number into a packet field with no length
    /// prefix behind it.
    pub fn load_equips(path: &std::path::Path) -> HashMap<u32, EquipTemplate> {
        let mut out = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            if f.len() != 19 {
                continue;
            }
            let n: Vec<Option<u32>> = f.iter().map(|x| x.parse::<u32>().ok()).collect();
            if n.iter().any(Option::is_none) {
                continue;
            }
            let v: Vec<u32> = n.into_iter().map(Option::unwrap).collect();
            let u = |i: usize| u16::try_from(v[i]).unwrap_or(u16::MAX);
            out.insert(
                v[0],
                EquipTemplate {
                    tuc: u(1),
                    inc_str: u(2),
                    inc_dex: u(3),
                    inc_int: u(4),
                    inc_luk: u(5),
                    inc_mhp: u(6),
                    inc_mmp: u(7),
                    inc_speed: u(8),
                    inc_jump: u(9),
                    inc_wat: u(10),
                    inc_mad: u(11),
                    inc_pdd: u(12),
                    inc_mdd: u(13),
                    inc_acc: u(14),
                    inc_eva: u(15),
                    inc_crt: u(16),
                    inc_crd: u(17),
                    trade_block: v[18] != 0,
                },
            );
        }
        out
    }

    /// Every map's mobs, from `tools/dump_portals.py`'s `mobs.txt` - the same `life` walk
    /// that produced the NPCs, filtered to `type == "m"`.
    ///
    /// **Object ids start at 2000, not 1000**, so that a map's mobs and its NPCs never
    /// collide even if the two pools turn out to share an id space. They are separate pools
    /// in the client - the mob singleton is `[0x143ABFE00]`, the NPC one is not - but that
    /// is one assumption this does not need to make, and a collision would show as a
    /// silently dropped mob with nothing in any log.
    ///
    /// [`net::mob::FieldMob::new`] then steps any id that is zero or a multiple of 178 past
    /// itself: both are values the client's own decoder treats specially, and a multiple of
    /// 178 takes a branch through a vtable slot on what looks like an exception object.
    pub fn load_mobs(path: &std::path::Path) -> HashMap<u32, Vec<net::mob::FieldMob>> {
        let mut out: HashMap<u32, Vec<net::mob::FieldMob>> = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            if f.len() < 5 {
                continue;
            }
            let n = |i: usize| f[i].parse::<i64>().ok();
            let (Some(map), Some(template), Some(x), Some(cy), Some(fh)) =
                (n(0), n(1), n(2), n(3), n(4))
            else {
                continue;
            };
            let list = out.entry(map as u32).or_default();
            let object_id = 2000 + list.len() as u32;
            list.push(net::mob::FieldMob::new(
                object_id,
                template as u32,
                x as i16,
                cy as i16,
                fh as i16,
                DEFAULT_MOB_HP,
            ));
        }
        out
    }
}

/// How many players it takes for a field to run at full spawn capacity.
///
/// **Adopted as policy by the owner on 2026-08-19**, from the same unofficial fan site as the
/// percentages: *"the mob cap is 75% of the map capacity unless there are more than 6
/// players on the map."* The site labels the two columns "Solo" and "6+ players", so the
/// threshold here is **six or more**. If strictly-more-than-six was meant, this is the one
/// number to change.
pub const CROWD_THRESHOLD: usize = 6;

/// Percent of a field's spawn points that hold a live mob below [`CROWD_THRESHOLD`].
pub const SPAWN_PERCENT_SOLO: usize = 75;

/// And at or above it. Every spawn point is filled.
pub const SPAWN_PERCENT_CROWDED: usize = 100;

/// How many of a map's spawn points may hold a live mob at once.
///
/// **A spawn point is not a mob.** Map 40, "Snail Hunting Ground I", has **40 mob spawn
/// points** in its WZ `life` node - checked, it is 42 life entries, 40 of type `m` plus
/// Robin and Sam - and a real server keeps **30** alive on it for a solo player. Sending one
/// mob per spawn point over-populates every map.
///
/// **The cap is not in the WZ, and that is measured rather than assumed.** Map 40's whole
/// `info` node is `AmbientBGM(v)`, `MR*`/`VR*` bounds, `bgm`, `cloud`, `fieldLimit`,
/// `fieldLimit2`, `fieldLimit_tw`, `fieldScript`, `fieldType`, `fly`, `forcedReturn`,
/// `hideMinimap`, `mapDesc`, `mapMark`, **`mobRate`**, `moveLimit`, `noMapCmd`,
/// `onFirstUserEnter`, `onUserEnter`, `partyStandAlone`, `personalShop`, `quarterView`,
/// `returnMap`, `standAlone`, `swim`, `town` and `version`. There is **no** capacity field
/// of any name. `mobRate` is there (`1.0` for map 40) but that is a respawn *rate*, not a
/// cap. So the cap is **server policy**, and it has to come from us.
///
/// **The rule is [I] and adopted deliberately.** The owner took it from an unofficial fan site,
/// flagged it as such, and then chose to accept it blanket: 75% below six players, 100% at
/// six or more, with nothing in between. Nothing in this client corroborates it. The single
/// datapoint is **40 spawn points -> 30**, which both floor and ceiling of `3n/4` reproduce,
/// so the **rounding is unsettled**; this floors. A small map is where the two would differ
/// (6 -> 4 flooring, 5 rounding up).
///
/// `players` is the number on the *field*, not on the channel. Today it is always 1: this
/// server has no field-occupancy tracking at all, so the crowded branch is written and
/// untaken. It is a parameter rather than a constant so that adding occupancy is a change
/// at the call site and not here.
pub fn spawn_capacity(spawn_points: usize, players: usize) -> usize {
    let percent = if players >= CROWD_THRESHOLD {
        SPAWN_PERCENT_CROWDED
    } else {
        SPAWN_PERCENT_SOLO
    };
    spawn_points * percent / 100
}

/// Choose which spawn points hold a live mob, keeping each type's **share** of the map.
///
/// The owner, 2026-08-19: *"on maps with multiple mobs, there's a concept of shares, the map will
/// try to maintain the balance ratio between the mobs under the cap."*
///
/// **Taking the first N spawn points is wrong**, and that is what this replaces. The
/// generated table is in WZ `life` index order, so on a mixed map the first N can be almost
/// all one type. The Field South of Ellinia has 45 spawns across five types - Snail 10,
/// Blue Snail 16, Shroom 7, Red Snail 6, Orange Mushroom 6 - and a cap of 33 has to keep
/// roughly 22 / 36 / 16 / 13 / 13 percent, not whatever the first 33 rows happen to be.
///
/// The apportionment is **largest-remainder** (Hamilton): each type gets
/// `floor(count * cap / total)` slots, then the leftover slots go to the types with the
/// largest remainders, ties broken by template id so the result is deterministic. That is
/// the standard way to hand out whole seats in proportion and it cannot overshoot the cap.
///
/// **[I], and only the shape of it.** That the engine balances by share is the owner's, from the
/// same unofficial fan site as the capacity scalar; nothing in this client corroborates it,
/// and the *exact* rounding the real engine uses is unknown. What this does guarantee is
/// that the result is capped, proportional and stable between runs.
///
/// Returns the chosen mobs in spawn order, so the wire order does not depend on the
/// grouping.
pub fn share_balanced(mobs: &[net::mob::FieldMob], cap: usize) -> Vec<&net::mob::FieldMob> {
    let total = mobs.len();
    if cap == 0 || total == 0 {
        return Vec::new();
    }
    if cap >= total {
        return mobs.iter().collect();
    }

    // Group spawn points by template, keeping WZ order inside each group.
    let mut groups: Vec<(u32, Vec<usize>)> = Vec::new();
    for (i, mob) in mobs.iter().enumerate() {
        match groups.iter_mut().find(|(t, _)| *t == mob.template_id) {
            Some((_, idx)) => idx.push(i),
            None => groups.push((mob.template_id, vec![i])),
        }
    }

    // floor(count * cap / total) each, then hand out what is left by largest remainder.
    let mut quota: Vec<(u32, usize, usize)> = groups
        .iter()
        .map(|(t, idx)| {
            let numerator = idx.len() * cap;
            (*t, numerator / total, numerator % total)
        })
        .collect();
    let mut leftover = cap - quota.iter().map(|(_, base, _)| base).sum::<usize>();
    let mut order: Vec<usize> = (0..quota.len()).collect();
    order.sort_by(|&a, &b| {
        quota[b].2.cmp(&quota[a].2).then(quota[a].0.cmp(&quota[b].0))
    });
    for &g in &order {
        if leftover == 0 {
            break;
        }
        quota[g].1 += 1;
        leftover -= 1;
    }

    let mut keep: Vec<usize> = Vec::with_capacity(cap);
    for (g, (_, idx)) in groups.iter().enumerate() {
        keep.extend(idx.iter().take(quota[g].1).copied());
    }
    keep.sort_unstable();
    keep.into_iter().map(|i| &mobs[i]).collect()
}

/// One equip's template values, as `Character.wz` has them.
///
/// **Field names are the WZ's own**, which is why there is no `inc_pad`: enumerating every
/// scalar `info` property across all 1760 equip images found **`incWAT` on 202 items and
/// `incPAD` on none**. A struct written from the game family's usual names would have had an
/// always-zero attack field and no weapon would ever have had any. `tools/dump_equips.py`
/// carries the full census.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EquipTemplate {
    /// Upgrade slots. Printed by the tooltip as "Remaining Enhancements" straight from the
    /// packet - the client does **not** fall back to this value, so a zero on the wire shows
    /// as zero even though the template says 7.
    pub tuc: u16,
    pub inc_str: u16,
    pub inc_dex: u16,
    pub inc_int: u16,
    pub inc_luk: u16,
    pub inc_mhp: u16,
    pub inc_mmp: u16,
    pub inc_speed: u16,
    pub inc_jump: u16,
    /// Weapon attack. **`incWAT`, not `incPAD`** - see the struct docs.
    pub inc_wat: u16,
    pub inc_mad: u16,
    pub inc_pdd: u16,
    pub inc_mdd: u16,
    pub inc_acc: u16,
    pub inc_eva: u16,
    pub inc_crt: u16,
    pub inc_crd: u16,
    /// **Only 7 of 1760 equips carry this**, which is the measured form of the owner's "that
    /// should only apply to some items, and not the starter items".
    pub trade_block: bool,
}

impl EquipTemplate {
    /// This template as the stat block a **fresh, unscrolled** instance of the item carries.
    ///
    /// **The packet field is the absolute value, not an increment**, and the first reading of
    /// this was the other way round. `FUN_142699710` loads the packet value into `R8D`,
    /// tests *it* for the print guard, and then **subtracts** the `ITEMINFO` template value
    /// to work out the leftover for the ` (%d +%d +%d)` breakdown - so the number the user
    /// reads is the packet's alone, and the short "no parenthetical" path is taken exactly
    /// when packet == template, which only makes sense if the packet carries the total.
    /// `FUN_14038d3c0` corroborates it from a different subsystem: it compares
    /// `item+0x62` against `ITEMINFO.incSTR` directly, same width, no arithmetic on either
    /// side. That is meaningless unless both are absolutes.
    ///
    /// So a fresh Grey T-Shirt goes out with `inc_pdd = 6`, and the owner's tooltip showing **no
    /// stat section at all** is explained: the guard is on the packet field, and zero
    /// suppresses the line however large the template value is.
    ///
    /// **Index 8 is always 0.** The mask's bit order has `incPAD` there, and this client's
    /// `Character.wz` does not contain that property on any of its 1760 equips - weapon
    /// attack lives in `incWAT`, which is bit 16. See `tools/dump_equips.py`.
    pub fn fresh_stats(&self) -> net::opcode::EquipStats {
        // In net::opcode::EQUIP_STAT_WZ_PROPERTIES order.
        let values = [
            self.inc_str, self.inc_dex, self.inc_int, self.inc_luk,
            self.inc_mhp, self.inc_mmp, self.inc_speed, self.inc_jump,
            0, // incPAD - not a property this client's data ever carries
            self.inc_mad, self.inc_pdd, self.inc_mdd, self.inc_acc, self.inc_eva,
            self.inc_crt, self.inc_crd, self.inc_wat,
        ];
        // remaining_enhancements must stay within 0..=tuc: FUN_14038d3c0 compares the two at
        // 0x14038d41c and treats a larger value as outside the range it models.
        let tuc = u8::try_from(self.tuc).unwrap_or(u8::MAX);
        net::opcode::EquipStats::fresh(net::opcode::EquipStatSet::from_wz_template(values), tuc)
    }
}

/// Maps that have a field image, have a name, and **still kill this client**.
///
/// **Measured, one entry at a time, and this is the only honest way to build this list.**
/// The owner typed `!map 900000000` (the White Map) on 2026-08-19 and the client died 0.4 s
/// later: `0xC0000005` at `0x14019b8cf`, which is inside the client's own small-block
/// allocator - `FUN_14019b780` pops a free-list head with `MOV RCX,[RAX]` immediately after
/// the chunk allocator at `0x14019d3c0` returned, so `RAX` was **null and unchecked**. That
/// is an allocation failure, not a packet desync: the `SetField` differed from a working one
/// by four bytes of map id.
///
/// **900000000 passes every check we have.** It is in `fields.txt` (it has a real field
/// image), it is in the `String.wz` name table, and its image parses clean with one spawn
/// portal and a foothold group. So "the field exists and is named" is *necessary and not
/// sufficient*, and there is no property of it we can currently test that predicts the
/// crash.
///
/// **One hypothesis, deliberately NOT acted on.** 900000000 has **no `miniMap` node** while
/// its `hideMinimap` is `0`. Surveying all 426 field images finds **44** maps in that state.
/// Blocking all 44 on that theory would remove a tenth of the maps from a debugging command
/// on the strength of a guess, and the fault address - a generic allocator - does not point
/// at a minimap. **The cheap falsifier costs nothing on the next run:** `!map 61` is also in
/// that set and is an ordinary low id. If it dies the same way, the guard should become the
/// 44; if it loads fine, the theory is dead and this list stays a list.
pub const MAPS_THAT_KILL_THE_CLIENT: &[u32] = &[
    900000000, // the White Map. Measured 2026-08-19.
];

/// The HP a spawned mob starts with until `Mob.wz` is read for the real value.
///
/// **Not zero, deliberately.** Zero is structurally legal and draws a mob at 0% health,
/// which is exactly the shape of the NPC bug - `isEnabled` and `alpha` were zero and every
/// NPC was created, pooled, disabled and fully transparent while the layout was perfect.
/// The client computes the bar as `hp * 100 / maxHp`, and `141c50502` is an `IDIV` with **no
/// zero guard**.
pub const DEFAULT_MOB_HP: u64 = 100;

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
            mobs: HashMap::new(),
            equips: HashMap::new(),
            send_mobs: false,
            fields: std::collections::HashSet::new(),
        }
    }
}

#[cfg(test)]
mod spawn_tests {
    use super::*;

    fn field(templates: &[(u32, usize)]) -> Vec<net::mob::FieldMob> {
        let mut out = Vec::new();
        let mut id = 2000;
        for (template, count) in templates {
            for _ in 0..*count {
                out.push(net::mob::FieldMob::new(id, *template, 0, 0, 1, DEFAULT_MOB_HP));
                id += 1;
            }
        }
        out
    }

    fn counts(chosen: &[&net::mob::FieldMob]) -> std::collections::BTreeMap<u32, usize> {
        let mut m = std::collections::BTreeMap::new();
        for mob in chosen {
            *m.entry(mob.template_id).or_insert(0) += 1;
        }
        m
    }

    /// Map 40, "Snail Hunting Ground I": 40 spawn points, one type, 30 alive for a solo
    /// player. The single datapoint the capacity rule has.
    #[test]
    fn map_40_keeps_thirty_of_its_forty_spawn_points() {
        assert_eq!(spawn_capacity(40, 1), 30);
        let mobs = field(&[(2, 40)]);
        let chosen = share_balanced(&mobs, spawn_capacity(mobs.len(), 1));
        assert_eq!(chosen.len(), 30);
        assert_eq!(counts(&chosen), [(2, 30)].into_iter().collect());
    }

    /// The Field South of Ellinia: 45 spawns across five types. Taking the first N in WZ
    /// order would return almost all of one type, which is the bug this replaces - the
    /// generated table is grouped, so the first 33 rows here are Snail and Blue Snail only.
    #[test]
    fn a_mixed_map_keeps_each_types_share_rather_than_the_first_n() {
        let mobs = field(&[(1, 10), (2, 16), (3, 7), (4, 6), (5, 6)]);
        assert_eq!(mobs.len(), 45);
        let cap = spawn_capacity(45, 1);
        assert_eq!(cap, 33);

        let chosen = share_balanced(&mobs, cap);
        assert_eq!(chosen.len(), cap, "the cap must be filled exactly");

        // Largest remainder from 10/16/7/6/6 at cap 33: bases 7/11/5/4/4 = 31, and the two
        // leftover slots go to the largest remainders (Blue Snail 33, then Red Snail and
        // Orange Mushroom tie at 18 - broken by template id).
        assert_eq!(
            counts(&chosen),
            [(1, 7), (2, 12), (3, 5), (4, 5), (5, 4)].into_iter().collect()
        );

        // Every type survives, and none is over-represented: each share is within one slot
        // of its exact proportion. That is the property, the exact split is the arithmetic.
        for (template, count) in [(1usize, 10usize), (2, 16), (3, 7), (4, 6), (5, 6)] {
            let exact = count as f64 * cap as f64 / 45.0;
            let got = counts(&chosen)[&(template as u32)] as f64;
            assert!(
                (got - exact).abs() < 1.0,
                "template {template}: {got} against an exact {exact}"
            );
        }

        // What the naive version did, kept as the thing being ruled out.
        let naive: Vec<u32> = mobs.iter().take(cap).map(|m| m.template_id).collect();
        assert!(
            !naive.contains(&5),
            "the first 33 in WZ order miss a whole type - that is the bug"
        );
    }

    /// Spawn order is preserved, so the wire order does not depend on how the grouping ran.
    #[test]
    fn the_chosen_mobs_come_back_in_spawn_order() {
        let mobs = field(&[(1, 4), (2, 4)]);
        let chosen = share_balanced(&mobs, 6);
        let ids: Vec<u32> = chosen.iter().map(|m| m.object_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
    }

    /// The edges, because these come off generated data and a panic here takes the server
    /// down on a field entry.
    #[test]
    fn the_edges_do_not_panic_or_overshoot() {
        assert!(share_balanced(&[], 10).is_empty());
        assert!(share_balanced(&field(&[(1, 5)]), 0).is_empty());

        // A cap at or above the total keeps everything, and never more.
        let mobs = field(&[(1, 3), (2, 2)]);
        assert_eq!(share_balanced(&mobs, 5).len(), 5);
        assert_eq!(share_balanced(&mobs, 99).len(), 5);

        // And a cap of one still returns exactly one, from the largest type.
        let one = share_balanced(&mobs, 1);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].template_id, 1, "the largest share takes the only slot");

        // Every cap from 0 to total is filled exactly, on a ragged mix.
        let ragged = field(&[(7, 1), (3, 13), (9, 4), (1, 2)]);
        for cap in 0..=ragged.len() {
            assert_eq!(share_balanced(&ragged, cap).len(), cap, "cap {cap}");
        }
    }

    /// Map 30's six snails are unaffected in kind but not in number, and the rounding is
    /// the part that is NOT settled: floor gives 4 where ceiling would give 5, and the one
    /// datapoint we have (40 -> 30) cannot tell them apart. Pinned so a change is deliberate.
    #[test]
    fn a_small_map_shows_the_rounding_that_is_still_unsettled() {
        assert_eq!(spawn_capacity(6, 1), 4, "floor(6 * 75 / 100); rounding up would be 5");
        assert_eq!(spawn_capacity(1, 1), 0, "and one spawn point rounds to none");
    }
    /// The four items a created character wears, read back out of the generated table.
    ///
    /// These values are the reason the table exists: a shirt with `incPDD = 6` and `tuc = 7`
    /// is what the client's own `Character.wz` says a Grey T-Shirt is, and the server was
    /// sending zeros for both.
    #[test]
    fn the_starter_equips_come_back_with_the_stats_the_wz_gives_them() {
        let path = std::path::Path::new("../../gm-handbook/equips.txt");
        if !path.exists() {
            return; // generated data, gitignored - tools/dump_equips.py makes it
        }
        let equips = Config::load_equips(path);
        assert!(equips.len() > 1000, "only {} equips loaded", equips.len());

        let shirt = equips[&1040002];
        assert_eq!(shirt.tuc, 7, "Grey T-Shirt has 7 upgrade slots");
        assert_eq!(shirt.inc_pdd, 6, "and 6 weapon defence");
        assert!(!shirt.trade_block, "a starter shirt is not trade-blocked");

        // The sword is the check that matters for the column set: its attack is in
        // `incWAT`, and this client's WZ has no `incPAD` at all. A loader written from the
        // family's usual names would report 0 here and every weapon would be harmless.
        let sword = equips[&1302000];
        assert_eq!(sword.inc_wat, 17, "the starter sword's attack is incWAT, not incPAD");
        assert_eq!(sword.inc_pdd, 0);

        // Only a handful of equips are trade-blocked, which is the measured version of
        // "that should only apply to some items".
        let blocked = equips.values().filter(|e| e.trade_block).count();
        assert!(blocked > 0 && blocked < 20, "{blocked} equips carry tradeBlock");
    }

    /// A map that has a field image and a name can still kill the client, and one does.
    #[test]
    fn the_white_map_is_refused_even_though_it_passes_every_other_check() {
        let path = std::path::Path::new("../../gm-handbook/fields.txt");
        if !path.exists() {
            return; // generated data, gitignored
        }
        let config = Config { fields: Config::load_fields(path), ..Config::default() };

        // It really does pass the checks that exist - this is not a typo being refused.
        assert!(config.fields.contains(&900000000), "the White Map has a field image");
        assert!(!config.map_exists(900000000), "and is refused anyway");

        // Ordinary maps are untouched, including the one the owner warps to most.
        for map in [1u32, 10, 20, 30, 40] {
            assert!(config.map_exists(map), "map {map} must still be reachable");
        }

        // The list stays short on purpose: every entry costs a client run to establish.
        assert!(
            MAPS_THAT_KILL_THE_CLIENT.len() < 5,
            "this list should only ever grow by measurement"
        );
    }

    /// The crowd threshold the owner adopted: 75% below six players on the field, 100% at six or
    /// more, nothing in between. Written and untaken - this server has no field-occupancy
    /// tracking, so `players` is always 1 today.
    #[test]
    fn a_crowded_field_fills_every_spawn_point() {
        for players in 0..CROWD_THRESHOLD {
            assert_eq!(spawn_capacity(40, players), 30, "{players} player(s)");
        }
        for players in [CROWD_THRESHOLD, CROWD_THRESHOLD + 1, 50] {
            assert_eq!(spawn_capacity(40, players), 40, "{players} player(s)");
        }
        // The step is a step, not a ramp: nothing between the two percentages.
        assert_eq!(spawn_capacity(45, 5), 33);
        assert_eq!(spawn_capacity(45, 6), 45);
    }
}
