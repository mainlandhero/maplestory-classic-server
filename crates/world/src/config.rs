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

    /// **One address per channel, in channel order** - the same list the login server
    /// advertises, and for the same reason: a Change Channel request has to be answered with
    /// the address of the channel being entered, and a channel with no address is one nobody
    /// can enter.
    ///
    /// It is here rather than only on the login server because `0x00D2` arrives on the
    /// *channel* connection, not the login one. Empty is legal and means Change Channel is
    /// refused with a message rather than ignored - see `Session::on_change_channel`.
    pub channels: Vec<std::net::SocketAddrV4>,

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
    /// authenticates anybody.
    ///
    /// That default is also a trap, and it has cost one of the owner's manual launches: without
    /// `--set-field-probe`, `Session::handle` returns nothing for *every* packet, the
    /// migration hello goes unanswered, and the client sits on "Connecting..." looking like
    /// a server that is not running. `tools/test-server.ps1` takes `-SetFieldProbe` and
    /// `STATUS.md`'s test plan says so in the command line itself.
    pub set_field_probe: bool,

    /// Override every character's inventory slot counts, for one run.
    ///
    /// **A test lever, not a game rule.** The real value is per-character and persisted
    /// (`characters.slots_*`); this replaces it in the record the moment before it goes out,
    /// so a client launch can put a number on screen that could not have come from anywhere
    /// else.
    ///
    /// That matters because the default - **30**, six rows of five, corrected by the owner on
    /// 2026-08-19 from the 24 this comment used to claim - is also the number this game
    /// family's client would plausibly have arrived at on its own, so a run at it cannot
    /// tell "the server sized the bag" from "the server changed nothing". A run at **125**,
    /// the maximum, can: the bag either grows a scrollbar or it does not. That is the run
    /// that settled it.
    ///
    /// 30 is also the floor. Below it the value has no use and is clamped - see
    /// `net::opcode::MIN_INVENTORY_SLOTS`.
    pub inventory_slots: Option<u16>,

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
    /// `(map, objectId)` -> the WZ's `mobTime` for that spawn point, in **seconds**.
    ///
    /// Kept beside `mobs` rather than on `net::mob::FieldMob`, because it is not a wire
    /// field: the client never respawns anything, it renders what it is sent. See
    /// [`respawn_delay_ms`] for what the three cases mean.
    pub mob_respawn_s: HashMap<(u32, u32), i32>,
    /// How many mobs to send per field, whatever the capacity says. `None` is no limit.
    ///
    /// **A blast-radius control, not game behaviour.** The mob body killed the client on
    /// 2026-08-19 and the fault could equally have come from the body being wrong or from
    /// thirty objects arriving at once - the White Map crash the same day *was* an
    /// allocation failure, so "too many" is not a silly hypothesis. `--mob-limit 1` makes
    /// those two answers distinguishable in one run.
    pub mob_limit: Option<usize>,
    /// How many rows a shop counter may send, whatever the shop holds. `None` is no limit.
    ///
    /// **The same blast-radius control as [`Config::mob_limit`], and for the same reason.**
    /// On 2026-08-20 Lucy's counter went out with twelve rows - six buy, six sell - and the
    /// client threw a C++ exception **ten milliseconds later**, stopped sending anything at
    /// all, and faulted three and a half seconds after that inside a refcount release. A
    /// crash like that can come from one row's contents or from twelve rows arriving at
    /// once, and on screen those are the same picture.
    ///
    /// `--shop-rows 1` sends a single **buy** row. The buy direction has a straight-line
    /// trace from the row bytes to the request bytes behind it; the sell direction has
    /// never been seen on a wire in either direction, so the cap keeps the better-evidenced
    /// half. It can never take the list to zero: a zero-row shop is a different client arm
    /// that builds a dialog box instead of a counter.
    pub shop_rows: Option<usize>,
    // **`send_shop` was removed on 2026-08-28, and the field it gated is worth remembering.**
    //
    // It existed to keep `0x0560` OpenShop OFF, because that packet killed this client twice.
    // Not a protocol bug and never fixable from the server: the Shop2 UI's constructor loads
    // `UI/UIWindow2.img/Shop2/backgrnd`, that image is **not in this client's WZ**, the
    // resource call fails, `_com_issue_errorex` throws and the unwinder faults - *before* a
    // single row byte is read, which is why one correctly-formed row killed it exactly as
    // twelve did.
    //
    // The flag's own doc ended with the answer: *"the WZ ships `UIShop.img/Shop`, the
    // classic-layout counter; which opcode builds that is the open question."* It is
    // **`0x055D`** (`research/classic-shop-opcode.md`), its body is decoded
    // (`research/classic-shop-rows.md`) and `net::classicshop` builds it against that file's
    // own golden vector. So shops are on, unconditionally, and there is nothing left to gate.
    //
    // `--shop` is now a no-op that says so rather than an error, because it is in the owner's
    // shell history.
    /// What each mob drops when it dies, plus the global event table. `data/drops.txt`.
    ///
    /// Empty is legal and means mobs drop nothing - see `crate::droptables::DropTables::load`
    /// for why a missing file degrades rather than refusing to start.
    pub drops: crate::droptables::DropTables,
    /// How much experience each level costs, and what a level awards. `data/exp-curve.txt`.
    pub exp_curve: crate::expcurve::ExpCurve,
    /// Template id -> experience for killing one, from the client's own `mobtemplates.txt`.
    ///
    /// Unlike the curve and the drop tables, this **is** the client's data - `tools/dump_mobs.py`
    /// reads it out of `Mob.wz` - so it is generated, gitignored, and not a guess.
    pub mob_exp: HashMap<u32, u32>,

    /// `templateId -> PADamage`, from the same `mobtemplates.txt` as [`Config::mob_exp`].
    ///
    /// What a mob hits for, before the player's defence. Empty means the server keeps
    /// whatever damage the client reported, which is the behaviour that shipped before this
    /// existed - and which had every snail hitting for 1.
    pub mob_attack: HashMap<u32, u32>,
    /// What each quest requires: mobs to kill and items to hold. `gm-handbook/questreq.txt`.
    ///
    /// Generated from the client's own `Quest.wz`, so this is the client's data rather than
    /// a guess - unlike the drop chances or the level gains.
    pub quest_reqs: net::quest::QuestRequirementTable,
    /// Every equip's template values, keyed by item id, from `gm-handbook/equips.txt`.
    ///
    /// The character record carries an item's stats and upgrade slots per *instance*, and a
    /// real server fills them from the template when the item is created. An empty table is
    /// not fatal - items are still sent, just bare - so a missing file degrades to exactly
    /// the behaviour confirmed on screen on 2026-08-19.
    pub equips: HashMap<u32, EquipTemplate>,
    /// Every NPC template's name, spoken dialogue and idle chatter, keyed by template id.
    pub npc_strings: HashMap<u32, NpcStrings>,
    /// Every quest the client ships, keyed by quest id, from `gm-handbook/questlines.txt`.
    pub quests: HashMap<u32, Quest>,
    /// Every NPC shop, from `data/shops.txt` - **authored, not generated**.
    ///
    /// The odd one out in this struct: every other table here is extracted from the client's
    /// own WZ, and shop contents are provably not in it (STATUS.md goal F checked three ways
    /// with a control each). See [`crate::shops`].
    ///
    /// **Wired since 2026-08-20.** `0x0560` opens the counter and `0x0104` is answered;
    /// clicking a shopkeeper sends this table's rows. The join from a shop's NPC *name* onto
    /// the template id a click carries is [`Self::shop_by_template`], and it was the last
    /// thing between a decoded packet and a shop on screen.
    pub shops: crate::shops::ShopTable,
    /// `npcTemplateId -> index into shops.shops`, built by
    /// [`crate::shops::resolve_npc_templates`] at startup.
    ///
    /// **This is the join that was missing.** `data/shops.txt` names the NPC the way the
    /// live UI does and the client's click carries a template id; without this map a fully
    /// decoded shop packet has nobody to send it to, which is exactly what the owner saw when
    /// clicking Lucy produced placeholder dialogue.
    pub shop_by_template: HashMap<u32, usize>,
    /// Turn NPC idle chatter off. It is the server's only unsolicited path, so a flag to
    /// silence it makes "is this packet the problem" answerable in one run.
    pub chatter_off: bool,
    /// Whether to actually send them. **Default `true` since 2026-08-19.**
    ///
    /// It was `false` for one day, because the mob body faulted the client at
    /// `0x141c810b0` on the first `0x03C6`. That fault has a cause and a fix - `move_action`
    /// was `0`, the one value that takes a callback into an interface `encodeInit` has not
    /// built yet, and it is `2` now (`research/mob-spawn.md` §11).
    ///
    /// **The opt-in was retired because it cost a run.** The owner spent a launch standing on
    /// map 40 seeing no snails: the server had the six of them loaded and sent none, and
    /// said so only in `world.log.err`, which nobody reads during a run. A default that
    /// silently does nothing is worse than a crash - a crash at least reports itself.
    ///
    /// Turn it off with `--no-mobs` when mobs are the variable being eliminated.
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

    /// Where a character who dies on each map comes back, from
    /// `gm-handbook/returnmaps.txt`'s **`reviveMap`** column.
    ///
    /// **Do not walk `returnMap` yourself.** The generator already did, and the walk has two
    /// traps a caller re-deriving it will fall into. `town == 1` is coarser than "town
    /// square" - shop interiors carry it, and **94 of this client's 115 town fields point
    /// their own `returnMap` somewhere else** - so a resolver that stops on the flag revives
    /// the player inside Southperry Armor Store. And 33 fields form `returnMap` self-loops.
    /// The shipped rule is one **unconditional** hop, then walk while not a town, cap 8,
    /// returning the last real field on a loop; over all 426 maps that gives 421 in one hop,
    /// 5 in two, and none with no destination. `research/return-maps.md`.
    ///
    /// Empty means the file was not generated - [`Config::revive_field`] then leaves the
    /// character where they fell rather than guessing.
    pub revive_maps: HashMap<u32, u32>,

    /// Every map's floor, from `gm-handbook/footholds.txt`, so a drop lands somewhere a
    /// player can actually reach it. See [`crate::footholds`].
    ///
    /// An empty table means every drop keeps the position the caller already had, which is
    /// exactly the behaviour that shipped before this existed - so a missing file degrades to
    /// something known rather than to something untested.
    pub footholds: crate::footholds::Footholds,

    /// What each consumable restores, from `gm-handbook/consumables.txt`.
    ///
    /// Empty means every potion is refused with a notice rather than silently doing nothing
    /// - see [`crate::consumables`] for why that direction was chosen.
    pub consumables: crate::consumables::Consumables,

    /// The cash shop's sale list, keyed by **SN**, from `gm-handbook/commodity.txt`.
    ///
    /// Empty means every purchase is refused with "sold out" rather than priced from a guess.
    /// The client draws its catalogue from its own copy of the same file either way, so an
    /// empty table here looks like a fully stocked shop where nothing can be bought - which is
    /// why [`crate::commodity::CommodityTable::banner`] says so unconditionally at start-up.
    pub commodity: crate::commodity::CommodityTable,

    /// What each job may learn and how far, from `gm-handbook/skills.txt`.
    ///
    /// Empty means only the three beginner skills are grantable - exactly the behaviour that
    /// shipped before the table existed, so a missing file degrades to something known.
    pub skills: crate::skilltable::SkillTable,
    /// What a cast COSTS and what it does, per skill level, from the same generated file.
    ///
    /// Separate from [`Self::skills`] because they answer different questions: that one is
    /// "may this job learn it, and how far", this one is "what does level `n` cost". The
    /// attack path needs the second and nothing else.
    ///
    /// Empty means an attack skill costs no MP - which is exactly the behaviour that produced
    /// The owner's *"the Red Potion recovered my MP"* on 2026-08-28, so a missing file degrades to
    /// a known bug rather than an unknown one. `banner()` says so at start-up.
    pub firstjob: crate::firstjob::CombatTable,

    /// `mapId -> name`, from `gm-handbook/maps.txt`.
    ///
    /// **Only ever used to say something on screen.** Nothing routes on it, so a missing
    /// file costs a GM acknowledgement that reads "map 40" instead of "map 40, Ant Tunnel
    /// Park" and nothing else. That is why it is a plain map with no failure path: this is
    /// the one table where degrading quietly is the right behaviour.
    pub map_names: HashMap<u32, String>,

    /// `itemId -> name`, from `gm-handbook/items.txt`.
    ///
    /// The reverse of the map `crate::shops::load_item_names` builds. That one is
    /// `name -> ids` because `data/shops.txt` is authored with names and has to resolve
    /// them; this one is for printing an id back to a person.
    pub item_names: HashMap<u32, String>,
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
    /// **A field image is what this checks and it is not a promise the map is safe.**
    /// `!map 900000000` crashed the client on 2026-08-19 and was briefly denylisted here -
    /// then the owner logged in with that map stored and it loaded fine, so the denylist was
    /// blocking a working map. The crash is in the mid-session *transition*, not the
    /// destination; see the retraction above `MobTemplate`.
    pub fn map_exists(&self, map: u32) -> bool {
        self.fields.is_empty() || self.fields.contains(&map)
    }

    /// Load one map id per line, ignoring blanks and `#` comments.
    /// Load an `id, name` table - `gm-handbook/maps.txt`, `gm-handbook/items.txt`.
    ///
    /// The name may itself contain commas, so the split is on the **first** one only. Two
    /// of the map names in this client do.
    pub fn load_id_names(path: &std::path::Path) -> HashMap<u32, String> {
        let mut out = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((id, name)) = line.split_once(',') else { continue };
            let Ok(id) = id.trim().parse::<u32>() else { continue };
            let name = name.trim();
            if !name.is_empty() {
                out.insert(id, name.to_string());
            }
        }
        out
    }

    /// Load the `reviveMap` column of `gm-handbook/returnmaps.txt`.
    ///
    /// Columns are `map, returnMap, forcedReturn, town, reviveMap, hops, name` and the file
    /// is **TAB separated**, because one map is called `The Resting Spot, Pig Park` and a
    /// comma split would cut it in the wrong place.
    ///
    /// `forcedReturn` is deliberately **not** read here. It is an *eject* target, not a
    /// respawn one - its 72 real entries are ship cabins mid-flight, timed subway depots and
    /// PQ stages, and it disagrees with `returnMap` on 28 of the 44 where both are real.
    /// `Dead Mine I` returns to El Nath, the town, and force-ejects to the field outside the
    /// mine. Using it for death would put a dead player back at the dungeon door.
    pub fn load_revive_maps(path: &std::path::Path) -> HashMap<u32, u32> {
        let mut out = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut cols = line.split('\t');
            let (Some(map), Some(_ret), Some(_forced), Some(_town), Some(revive)) =
                (cols.next(), cols.next(), cols.next(), cols.next(), cols.next())
            else {
                continue;
            };
            if let (Ok(map), Ok(revive)) = (map.trim().parse::<u32>(), revive.trim().parse::<u32>())
            {
                out.insert(map, revive);
            }
        }
        out
    }

    /// Where a character who died on `field` comes back, or `None` if the table has no row.
    ///
    /// `None` means the caller should leave them where they fell. That is deliberately not
    /// the same as picking a default: sending a character to a map this client has no field
    /// image for strands them, and `research/map1-exists.md` records a branch that does not
    /// return.
    pub fn revive_field(&self, field: u32) -> Option<u32> {
        self.revive_maps.get(&field).copied()
    }

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
            // **At least the 19 this needs, and ignore whatever follows.** The generator grew
            // six requirement columns and a NAME on 2026-08-28, so the file is 26 fields wide
            // now. An `!= 19` here skipped every row of the new file, and the symptom would
            // have been the 2026-08-19 bug returning exactly: a shirt with no defence and no
            // upgrade slots, from a parser that silently agreed with itself.
            //
            // The name is last and is not numeric, which is why the parse below is scoped to
            // the leading fields rather than applied to all of them.
            if f.len() < 19 {
                continue;
            }
            let n: Vec<Option<u32>> = f[..19].iter().map(|x| x.parse::<u32>().ok()).collect();
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
    pub fn load_mobs(
        path: &std::path::Path,
        templates: &HashMap<u32, MobTemplate>,
    ) -> LoadedMobs {
        let mut out: HashMap<u32, Vec<net::mob::FieldMob>> = HashMap::new();
        let mut respawn: HashMap<(u32, u32), i32> = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return (out, respawn) };
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
            // A fresh mob is at its template's full HP. DEFAULT_MOB_HP is only reached
            // when Mob.wz has nothing to say about the template, and it is not zero for the
            // reason in its own docs.
            let hp = templates
                .get(&(template as u32))
                .map(|t| u64::from(t.max_hp))
                .unwrap_or(DEFAULT_MOB_HP);
            // Column 8 is the WZ's `mobTime`. Older dumps have eight columns and no such
            // value; those read as 0, which is "the field's ordinary rate" and is the right
            // reading for a spawn point with no mobTime node at all.
            respawn.insert((map as u32, object_id), n(8).unwrap_or(0) as i32);
            list.push(net::mob::FieldMob::new(
                object_id,
                template as u32,
                x as i16,
                cy as i16,
                fh as i16,
                hp,
            ));
        }
        (out, respawn)
    }
}

/// What [`Config::load_mobs`] returns: every map's spawn points, and each one's `mobTime`.
///
/// A named pair rather than a tuple because the two halves are keyed differently - one by
/// map, one by `(map, objectId)` - and a caller that mixes them up gets a compiling program
/// that respawns nothing.
pub type LoadedMobs = (HashMap<u32, Vec<net::mob::FieldMob>>, HashMap<(u32, u32), i32>);

/// The ordinary field respawn rate, for a spawn point whose WZ node has no `mobTime`.
///
/// **Policy, `[I]`.** 9485 of this client's 9928 spawn points have no `mobTime` at all, so
/// this number decides how almost every map feels. Seven seconds is this game family's
/// long-standing field rate. It is one constant in one place precisely because it is a guess.
pub const DEFAULT_RESPAWN_MS: u64 = 7_000;

/// The WZ value that means **never respawn this spawn point**.
pub const MOB_TIME_NEVER: i32 = -1;

/// How long after a mob dies its spawn point refills, from the WZ's `mobTime`.
///
/// Three cases, and conflating the first two empties a map after one pass:
///
/// | `mobTime` | meaning |
/// |---|---|
/// | `> 0` | that many **seconds**, from the WZ |
/// | `0` | no `mobTime` node - the field's ordinary rate, [`DEFAULT_RESPAWN_MS`] |
/// | `-1` | never. One spawn point in this client says so |
pub fn respawn_delay_ms(mob_time_s: i32) -> Option<u64> {
    match mob_time_s {
        MOB_TIME_NEVER => None,
        0 => Some(DEFAULT_RESPAWN_MS),
        s if s > 0 => Some(s as u64 * 1_000),
        _ => Some(DEFAULT_RESPAWN_MS),
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
    // **A map with spawn points must not have zero mobs.** 75% of one point is 0.75, which
    // truncates to none - so a one-point map would sit empty forever and a four-point map
    // would hold three. Found by a test on a one-point map, and it would have been almost
    // invisible in play: a small map that is simply never populated looks like a small map
    // with nothing on it.
    (spawn_points * percent / 100).max(1)
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
/// A tiny splitmix64, so the spawn choice can be random without a dependency and still be
/// reproducible from a seed.
///
/// Deliberately not a good general-purpose PRNG and deliberately not `rand`: this picks which
/// of forty spawn points are used, a decision with no security property and one caller. What
/// it does need is to be **seedable**, so a test can pin an exact selection and so two runs of
/// the same map do not lay the mobs out identically.
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn share_balanced(
    mobs: &[net::mob::FieldMob],
    cap: usize,
    seed: u64,
) -> Vec<&net::mob::FieldMob> {
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

    // **Which points, not just how many of each.** The owner, 2026-08-22, on Right Around Lith
    // Harbor: *"the mobs that spawn are completely concentrated on the left side of the map
    // on fresh spawn. The spawn points that gets activated should be randomly chosen even on
    // fresh spawn."*
    //
    // The quota arithmetic above was written to fix a *different* half of this, and its test
    // says so: taking the first N in WZ order returned almost all of one TYPE. That is fixed
    // and stays fixed. But inside each group this still did `idx.iter().take(n)` - the first
    // n in WZ order - and `life` entries are laid out left to right, so every fresh spawn put
    // its mobs at the low-x end of the map. Balanced by type, bunched by position.
    //
    // Shuffled per group rather than globally, so the per-template quota is untouched: this
    // decides *which* of a template's points are used, never how many.
    let mut rng = seed;
    let mut keep: Vec<usize> = Vec::with_capacity(cap);
    for (g, (_, idx)) in groups.iter().enumerate() {
        let take = quota[g].1;
        if take >= idx.len() {
            keep.extend(idx.iter().copied());
            continue;
        }
        // Partial Fisher-Yates: only the first `take` positions have to be settled.
        let mut pool: Vec<usize> = idx.clone();
        for i in 0..take {
            let j = i + (splitmix64(&mut rng) % (pool.len() - i) as u64) as usize;
            pool.swap(i, j);
        }
        keep.extend(pool.into_iter().take(take));
    }
    // Sorted so the packets still go out in map order, which is what the client expects and
    // what makes two logs of the same field comparable.
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

/// One quest, from `Quest.wz/QuestData`.
///
/// The client ships all 322 with their full dialogue, and the ids are **the same namespace
/// the protocol uses**: NPC template 1 starts exactly one quest, 1000, and the `0x0151` a
/// real client sent on 2026-08-19 carried quest 1000 with npc template 1.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Quest {
    pub name: String,
    /// `Check.0.npc` - who starts it.
    pub start_npc: Option<u32>,
    /// `Check.1.npc` - who finishes it. Often a different NPC on a different map.
    pub end_npc: Option<u32>,
    /// `Act.<state>.nextQuest` - the quest this one chains into.
    pub next_quest: Option<u32>,
    /// `Act.0.item.N` - what accepting the quest hands over, as `(item id, count)`.
    ///
    /// Quest 1001's is `(4031000, 1)`: Sera's mirror. The owner, 2026-08-20: *"when I talked to
    /// Sera, after their dialogue, they did not give me Sera's Mirror to be able to complete the
    /// quest."* They did not because nothing read this.
    ///
    /// A **negative** count in the WZ means "take it away" - quest 1001's `Act.1.item.0.count`
    /// is `-1`, the mirror going back when Heena is done with it. Only the giving direction is
    /// wired; see [`Quest::start_items`]'s note.
    pub start_items: Vec<(u32, i32)>,
    /// `Act.1.item.N` - what **completing** the quest does to the bag.
    ///
    /// A positive count is a reward and a negative one is the quest taking its item back.
    /// Quest 1001's is `(4031000, -1)`: Heena keeps the mirror.
    pub complete_items: Vec<(u32, i32)>,
    /// `Act.1.exp` - experience for turning it in. Quest 1001's is 2.
    pub complete_exp: u64,
    /// The conversation, keyed by the `Say` path with the line index removed.
    ///
    /// `"0"` is the opening conversation and `"1"` the completion one; `"0.yes"`,
    /// `"0.no"`, `"1.stop.npc"` and so on are the branches. Each value is that node's
    /// numbered lines **in index order**, which is not the same as string order once a
    /// conversation reaches ten lines.
    pub say: HashMap<String, Vec<String>>,

    /// `Act.<state>.hp` - **an AUTHORED key, not a WZ one.**
    ///
    /// Set the character's HP to this the moment the quest reaches that state. Roger's quest
    /// takes you down to 25/50 so the apple has something to heal, and enumerating the whole
    /// `Act` key space across all 322 quests finds **17 shapes and no `hp` among them** - so
    /// this behaviour lived in `q1002s`, the script the client does not ship, and it is ours
    /// to author. `data/quest-scripts.txt`.
    ///
    /// Absolute, matching `Act.1.exp`'s convention. Capped at the character's maximum by the
    /// caller.
    pub set_hp: HashMap<u8, u32>,

    /// `Check.<state>.consumeitem` - **also AUTHORED.**
    ///
    /// The owner, 2026-08-21: *"Once the user consumes the apple, the quest would be completed."*
    ///
    /// This is the reading of a shape the WZ *does* ship and nobody could settle: quest
    /// 1002's `Check.1.item.0` has an **`id` and no `count`**, while 214 other quests carry
    /// one. `research/quest-scripts.md` called "no count means must-not-hold" **[I]** and
    /// left it open. The owner's sentence settles it - the quest finishes when the item is gone -
    /// and this key is that rule made explicit rather than inferred from an absence at
    /// runtime.
    pub complete_on_consume: Option<u32>,
}

/// One `Act.<state>.item.<n>` while it is still being read.
///
/// The id and the count are separate rows in `questlines.txt` and nothing promises which
/// arrives first, so both sides are optional until the file is exhausted.
type HalfItem = (Option<u32>, Option<i32>);

/// The `<n>` out of `<state>.item.<n>.id`.
fn act_item_index(dotted: &str) -> Option<usize> {
    dotted.split('.').nth(2)?.parse().ok()
}

/// The `<state>` out of `<state>.item.<n>.id`, for the two states this server reads.
///
/// `0` is what accepting does and `1` is what completing does. Anything else is a state the
/// client has and this server does not model, and is ignored rather than guessed at.
fn act_state(dotted: &str) -> Option<u8> {
    let mut parts = dotted.split('.');
    let state: u8 = parts.next()?.parse().ok()?;
    if parts.next()? != "item" || !(state == 0 || state == 1) {
        return None;
    }
    Some(state)
}

/// Every quest, from `tools/dump_quests.py`'s `questlines.txt`.
///
/// TSV of `questId, node, dotted.path, value` - one row per scalar leaf, which is lossless
/// and needs no JSON parser for a query that is a flat lookup either way.
pub fn load_quests(path: &std::path::Path) -> HashMap<u32, Quest> {
    let mut out = HashMap::new();
    if let Ok(text) = std::fs::read_to_string(path) {
        read_quest_rows(&text, &mut out, Overlay::No);
    }
    out
}

/// Apply an **authored** overlay - `data/quest-scripts.txt` - onto quests already loaded.
///
/// # Why an overlay exists at all
///
/// Twelve of this client's 322 quests open with a *script* rather than a `Say` tree, and
/// **the script bodies are not in the client**. That is a verified negative, not a failed
/// search: all 205 `.wz` archives under `client-patched/Data` were opened and all 10021
/// images decoded, and `q1002s` occurs exactly twice in the whole tree - both times as a
/// *name* inside `1002.img`. `Data/Etc/Script/Script.wz` is a 63-byte header with zero
/// entries. `research/quest-scripts.md` has the full enumeration with every negative named.
///
/// So those quests' opening lines, their start items and their rewards have to be authored,
/// and this is where they are merged in.
///
/// # The client always wins
///
/// An overlay row only ever **fills a hole**. A `Say` node the client ships is never
/// replaced, `Act` items are skipped entirely if the WZ already gave that state any, and
/// `Act.1.exp` is only taken when the loaded value is still 0. If an overlay row ever
/// contradicts a shipped one the shipped one stands and the overlay has a bug - which is the
/// same direction this project takes everywhere else: the client's own data outranks ours.
///
/// Returns how many quests the file touched, so the caller can say so on **stdout**. A
/// missing file is 0 and is not an error - the same handling as `data/shops.txt`.
pub fn overlay_quests(quests: &mut HashMap<u32, Quest>, path: &std::path::Path) -> usize {
    let Ok(text) = std::fs::read_to_string(path) else { return 0 };
    read_quest_rows(&text, quests, Overlay::Yes)
}

/// Whether [`read_quest_rows`] is building the table or filling holes in one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Overlay {
    No,
    Yes,
}

/// The TSV reader behind both [`load_quests`] and [`overlay_quests`].
///
/// **Factored rather than duplicated on purpose.** The `rsplit_once` / `BTreeMap<usize, _>`
/// stitching below is what makes a ten-line `Say` node come out in index order instead of
/// string order - re-implementing it for the overlay is how `0.10` ends up before `0.2`, and
/// nothing would catch it.
fn read_quest_rows(text: &str, out: &mut HashMap<u32, Quest>, mode: Overlay) -> usize {
    use std::collections::BTreeMap;
    let mut lines: HashMap<u32, HashMap<String, BTreeMap<usize, String>>> = HashMap::new();
    // quest -> item index -> a half-built (id, count), stitched after the read because the two
    // halves are separate rows and the file does not promise an order.
    let mut act_items: HashMap<(u32, u8), BTreeMap<usize, HalfItem>> = HashMap::new();
    let mut touched: std::collections::HashSet<u32> = std::collections::HashSet::new();

    for row in text.lines() {
        if row.trim().is_empty() || row.starts_with('#') {
            continue;
        }
        let mut f = row.splitn(4, '\t');
        let (Some(id), Some(node), Some(dotted), Some(value)) =
            (f.next(), f.next(), f.next(), f.next())
        else {
            continue;
        };
        let Ok(qid) = id.trim().parse::<u32>() else { continue };
        touched.insert(qid);
        let quest = out.entry(qid).or_default();
        // An overlay fills holes; it never overwrites what the client shipped.
        let fill = mode == Overlay::No;
        match node {
            "QuestInfo" if dotted == "name" && (fill || quest.name.is_empty()) => {
                quest.name = value.to_string()
            }
            "Check" if dotted == "0.npc" && (fill || quest.start_npc.is_none()) => {
                quest.start_npc = value.parse().ok()
            }
            "Check" if dotted == "1.npc" && (fill || quest.end_npc.is_none()) => {
                quest.end_npc = value.parse().ok()
            }
            "Act"
                if dotted.ends_with(".nextQuest") && (fill || quest.next_quest.is_none()) =>
            {
                quest.next_quest = value.parse().ok()
            }
            // `Act.0.item.<n>.id` and `.count` arrive as separate rows in either order, so
            // both sides are stitched together after the file is read - see below.
            "Act" if dotted.ends_with(".id") && act_state(dotted).is_some() => {
                if let (Some(st), Some(n), Some(id)) =
                    (act_state(dotted), act_item_index(dotted), value.parse::<u32>().ok())
                {
                    act_items.entry((qid, st)).or_default().entry(n).or_default().0 = Some(id);
                }
            }
            "Act" if dotted.ends_with(".count") && act_state(dotted).is_some() => {
                if let (Some(st), Some(n), Some(c)) =
                    (act_state(dotted), act_item_index(dotted), value.parse::<i32>().ok())
                {
                    act_items.entry((qid, st)).or_default().entry(n).or_default().1 = Some(c);
                }
            }
            "Act" if dotted == "1.exp" && (fill || quest.complete_exp == 0) => {
                quest.complete_exp = value.parse().unwrap_or(0);
            }
            // Authored keys. Neither exists in the WZ - see the fields they set.
            "Act" if dotted.ends_with(".hp") => {
                if let (Some(state), Some(hp)) = (
                    dotted.split('.').next().and_then(|s| s.parse::<u8>().ok()),
                    value.parse::<u32>().ok(),
                ) {
                    quest.set_hp.entry(state).or_insert(hp);
                }
            }
            "Check" if dotted.ends_with(".consumeitem") => {
                if quest.complete_on_consume.is_none() {
                    quest.complete_on_consume = value.parse().ok();
                }
            }
            "Say" => {
                // The last path segment is the line index; everything before it is the
                // node. Splitting on the index rather than assuming a depth is what lets
                // "0.2" and "1.stop.npc.0" both work.
                let (key, index) = match dotted.rsplit_once('.') {
                    Some((head, tail)) => match tail.parse::<usize>() {
                        Ok(i) => (head.to_string(), i),
                        Err(_) => (dotted.to_string(), 0),
                    },
                    None => match dotted.parse::<usize>() {
                        Ok(i) => (String::new(), i),
                        Err(_) => (dotted.to_string(), 0),
                    },
                };
                lines
                    .entry(qid)
                    .or_default()
                    .entry(key)
                    .or_default()
                    .insert(index, value.to_string());
            }
            _ => {}
        }
    }

    for ((qid, state), indexed) in act_items {
        let quest = out.entry(qid).or_default();
        // Decided ONCE per state, before the loop: asking `is_empty()` inside it would let
        // the first overlay item through and then reject its siblings, which is a half-given
        // reward and worse than either whole answer.
        let take = match state {
            0 => mode == Overlay::No || quest.start_items.is_empty(),
            _ => mode == Overlay::No || quest.complete_items.is_empty(),
        };
        if !take {
            continue;
        }
        // BTreeMap: index order, so a two-item reward is handed over the same way twice.
        for (_, (id, count)) in indexed {
            if let (Some(id), Some(count)) = (id, count) {
                match state {
                    0 => quest.start_items.push((id, count)),
                    _ => quest.complete_items.push((id, count)),
                }
            }
        }
    }

    for (qid, nodes) in lines {
        let quest = out.entry(qid).or_default();
        for (key, indexed) in nodes {
            // BTreeMap keyed on the parsed index, so line 10 follows line 9 rather than
            // line 1 - which a string sort would get wrong and nothing would catch.
            let node: Vec<String> = indexed.into_values().collect();
            match mode {
                Overlay::No => {
                    quest.say.insert(key, node);
                }
                // A shipped node is never replaced. Quest 1002's `Say."0"` is `{}` in the
                // WZ, so there is nothing to lose - but 1002 is not the only script quest.
                Overlay::Yes => {
                    quest.say.entry(key).or_insert(node);
                }
            }
        }
    }
    touched.len()
}

/// One NPC template's text, from `String.wz/Npc.img`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NpcStrings {
    pub name: String,
    /// What the NPC says when talked to, in order - the WZ's `d0`, `d1`, ...
    ///
    /// **The text is sent raw, `#p8#` and all.** That token is a name substitution and
    /// whether the client expands it is not established; sending it unexpanded makes the
    /// screen answer the question, since "Hello! I'm Robin." and "Hello! I'm #p8#." are
    /// different on sight and neither is a guess.
    pub dialogue: Vec<String>,
    /// The idle-chatter lines, in order - the WZ's `n*`, then `f*`, `w*`, `h*`.
    ///
    /// **Four of the twelve prefixes in that data are classified and eight are not**
    /// (`c` appears 130 times and `s` 109); see `tools/dump_npcstrings.py`. Robin has only
    /// the four, which is why their ten lines match an outside list exactly and cannot
    /// discriminate the rest.
    pub chatter: Vec<String>,
    /// Just the `info/speak` group - the WZ's `n*` lines.
    ///
    /// **This is the only group a chat balloon can reach without also changing the NPC's
    /// animation.** `0x0453` with `nAction = -1` indexes `info/speak`; the other three
    /// groups hang off the `finger`, `wink` and `heart` animation nodes and need their own
    /// action value, which is not established. So [`Self::chatter`] is the full list for
    /// reference and this is the one the server can actually send.
    pub info: Vec<String>,
}

/// Every NPC's text, from `tools/dump_npcstrings.py`'s `npcstrings.txt`.
///
/// TSV, because the lines contain commas, apostrophes and quotes.
pub fn load_npc_strings(path: &std::path::Path) -> HashMap<u32, NpcStrings> {
    let mut out: HashMap<u32, NpcStrings> = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else { return out };
    for line in text.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let mut f = line.splitn(3, '\t');
        let (Some(id), Some(key), Some(value)) = (f.next(), f.next(), f.next()) else {
            continue;
        };
        let Ok(template) = id.trim().parse::<u32>() else { continue };
        let entry = out.entry(template).or_default();
        // The generator writes rows in key order within a template, so pushing keeps `d0`
        // before `d1` and `idle0` before `idle1` without re-parsing the index. A row out of
        // order would only reorder lines, never lose one.
        match key {
            "name" => entry.name = value.to_string(),
            k if k.starts_with("idle") => entry.chatter.push(value.to_string()),
            k if k.starts_with("info") => entry.info.push(value.to_string()),
            k if k.starts_with('d') => entry.dialogue.push(value.to_string()),
            _ => {}
        }
    }
    out
}

/// A mob template's stats, as `Mob.wz` has them. Field names are the WZ's own.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MobTemplate {
    /// What a freshly spawned mob's HP should be.
    ///
    /// The client draws the health bar as `hp * 100 / maxHP` through an `IDIV` at
    /// `141c50502` with **no zero guard**. The server used to send a flat `100` to every
    /// mob, which for a snail (`maxHP` 30) is 333% of its health.
    pub max_hp: u32,
    pub max_mp: u32,
    pub level: u32,
    pub exp: u32,
    /// `PADamage` - what the mob hits for before the player's defence.
    ///
    /// Column 6 of `mobtemplates.txt`, straight out of this client's own `Mob.wz`. The
    /// snail (template 2) is **3**. Loaded since 2026-08-21, when it turned out the client
    /// was reporting **1** for every snail hit and nothing server-side had a number to
    /// disagree with it.
    pub pa_damage: u32,
}

/// Every mob template's stats, from `tools/dump_mobs.py`'s `mobtemplates.txt`.
pub fn load_mob_templates(path: &std::path::Path) -> HashMap<u32, MobTemplate> {
    let mut out = HashMap::new();
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
        let n = |i: usize| f.get(i).and_then(|v| v.parse::<i64>().ok());
        let (Some(id), Some(hp), Some(mp), Some(level), Some(exp)) =
            (n(0), n(1), n(2), n(3), n(4))
        else {
            continue;
        };
        // Column 6. Optional on purpose: the `f.len() < 5` guard above admits a row without
        // it, and a mob with no attack column should hit for its own nothing rather than
        // drop out of the table entirely.
        let pa_damage = n(5).unwrap_or(0).max(0) as u32;
        // A template with no HP would make the client divide by zero. All 193 in this
        // client have one, so a row without is a generator fault and is dropped rather
        // than sent.
        if hp <= 0 {
            continue;
        }
        out.insert(
            id as u32,
            MobTemplate {
                max_hp: hp as u32,
                max_mp: mp.max(0) as u32,
                level: level.max(0) as u32,
                exp: exp.max(0) as u32,
                pa_damage,
            },
        );
    }
    out
}

/// **RETRACTED 2026-08-19: there is no such list, and the White Map is not broken.**
///
/// `!map 900000000` crashed the client on 2026-08-19 and this was a denylist with that one
/// entry in it. **The owner then logged in with a character whose stored map was 900000000 and
/// the map loaded fine** - "Hidden Street : White Map", character on screen, HP and MP
/// live, chat working. Their words: *"the fact that I spawned in map 900000000 should
/// disprove the fact the map was broken."* They are right, and the guard was blocking a map
/// that works.
///
/// **What that leaves, which is a better question than the one the denylist answered.** The
/// same map is fine on **login** and killed the client on a **mid-session `!map`** - and
/// `!map 1` and `!map 40` both worked mid-session in that very run. So the fault is in the
/// *transition*, not the destination, and it is selective about which destination.
///
/// The fault itself is still what it was: `0xC0000005` at `0x14019b8cf`, inside the
/// client's own small-block allocator, popping a free-list head with `MOV RCX,[RAX]`
/// straight after the chunk allocator at `0x14019d3c0` returned null. An allocation failure.
///
/// One difference worth someone's time: the White Map's view rectangle is tiny
/// (`VRLeft -389, VRRight 389, VRTop -265, VRBottom 285`) while map 40's is
/// `-299..2909` by `-1165..585`. A field-to-field transition that resizes buffers from the
/// second to the first is a shape that could plausibly ask an allocator for something it
/// refuses. **[I], and nothing supports it yet beyond the two numbers.**
///
/// **The minimap theory is dead.** It was never acted on - see the note that used to be
/// here - and the run killed it: 900000000 has no `miniMap` node and loads fine.
///
/// `!map` is unrestricted again beyond the field-image check. It is a debugging command and
/// a wrong theory that removes working maps from it costs more than the crash does.
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
            inventory_slots: None,
            portals: HashMap::new(),
            portal_index: HashMap::new(),
            npcs: HashMap::new(),
            mobs: HashMap::new(),
            mob_respawn_s: HashMap::new(),
            mob_limit: None,
            shop_rows: None,
            drops: crate::droptables::DropTables::default(),
            exp_curve: crate::expcurve::ExpCurve::default(),
            mob_exp: HashMap::new(),
            mob_attack: HashMap::new(),
            quest_reqs: net::quest::QuestRequirementTable::default(),
            chatter_off: false,
            equips: HashMap::new(),
            npc_strings: HashMap::new(),
            quests: HashMap::new(),
            shops: crate::shops::ShopTable::default(),
            shop_by_template: HashMap::new(),
            channels: Vec::new(),
            map_names: HashMap::new(),
            item_names: HashMap::new(),
            send_mobs: true,
            fields: std::collections::HashSet::new(),
            revive_maps: HashMap::new(),
            footholds: crate::footholds::Footholds::default(),
            consumables: crate::consumables::Consumables::default(),
            commodity: crate::commodity::CommodityTable::default(),
            skills: crate::skilltable::SkillTable::default(),
            firstjob: crate::firstjob::CombatTable::default(),
        }
    }
}

#[cfg(test)]
mod spawn_tests {
    /// A fixed seed for the tests that are about the per-type QUOTA rather than about which
    /// positions get used. Pinning it keeps those assertions exact; the tests that are about
    /// the randomness vary it on purpose.
    const TEST_SEED: u64 = 0x5EED_1234_5EED_1234;

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

    /// **A fresh spawn must use points from all over the map, not the first N.**
    ///
    /// The owner, 2026-08-22, on Right Around Lith Harbor: *"the mobs that spawn are completely
    /// concentrated on the left side of the map on fresh spawn. The spawn points that gets
    /// activated should be randomly chosen even on fresh spawn."*
    ///
    /// The per-type quota below was written for a *different* half of this - it stopped one
    /// TYPE taking every slot - and inside each group the code still did `take(n)` on WZ
    /// order. `life` entries run left to right, so a fresh field put every mob at the low-x
    /// end. Balanced by type, bunched by position.
    ///
    /// This is a distribution test, so it is written not to be flaky: with 20 of 60 points
    /// taken, it asserts only that **both halves of the list are represented** and that the
    /// mean index is not jammed against one end. The old `take(n)` behaviour fails it on the
    /// first assertion for every seed, not probabilistically - it can only ever return
    /// indices 0..20.
    #[test]
    fn a_fresh_spawn_spreads_across_the_map_instead_of_the_first_n() {
        let mobs = field(&[(2, 60)]);
        let cap = 20;

        for seed in [1u64, 2, 3, 99, 0x5EED, u64::MAX] {
            let chosen = share_balanced(&mobs, cap, seed);
            assert_eq!(chosen.len(), cap, "seed {seed}: the cap is still filled exactly");

            let idx: Vec<usize> = chosen
                .iter()
                .map(|c| mobs.iter().position(|m| std::ptr::eq(m, *c)).unwrap())
                .collect();
            let lower = idx.iter().filter(|&&i| i < 30).count();
            let upper = idx.len() - lower;
            assert!(
                lower > 0 && upper > 0,
                "seed {seed}: every chosen point came from one half - {idx:?}"
            );
            // The mean of 20 draws from 0..60 sits near 29.5. Anything under 15 or over 44
            // is a list that is still ordered rather than sampled.
            let mean = idx.iter().sum::<usize>() as f64 / idx.len() as f64;
            assert!(
                (15.0..=44.0).contains(&mean),
                "seed {seed}: mean index {mean:.1} - the choice is not spread"
            );
        }
    }

    /// Two different seeds must not produce the same field, or a "random" spawn is just a
    /// fixed one with extra steps - and two fresh entries in a row would look identical.
    #[test]
    fn different_seeds_choose_different_spawn_points() {
        let mobs = field(&[(2, 40)]);
        let ids = |seed| -> Vec<u32> {
            share_balanced(&mobs, 30, seed).iter().map(|m| m.object_id).collect()
        };
        assert_ne!(ids(1), ids(2));
        assert_eq!(ids(7), ids(7), "and the same seed reproduces exactly, or nothing is testable");
    }

    /// The quota is decided before the shuffle and the shuffle must not disturb it.
    ///
    /// This is the control for the two tests above: randomising *which* points are used must
    /// never change *how many* of each type, which is the property the earlier fix bought.
    #[test]
    fn shuffling_positions_does_not_disturb_the_per_type_quota() {
        let mobs = field(&[(1, 10), (2, 16), (3, 7), (4, 6), (5, 6)]);
        let cap = spawn_capacity(45, 1);
        let want: std::collections::BTreeMap<u32, usize> =
            [(1, 7), (2, 12), (3, 5), (4, 5), (5, 4)].into_iter().collect();
        for seed in [0u64, 1, 12345, u64::MAX / 3] {
            assert_eq!(counts(&share_balanced(&mobs, cap, seed)), want, "seed {seed}");
        }
    }

    /// Map 40, "Snail Hunting Ground I": 40 spawn points, one type, 30 alive for a solo
    /// player. The single datapoint the capacity rule has.
    #[test]
    fn map_40_keeps_thirty_of_its_forty_spawn_points() {
        assert_eq!(spawn_capacity(40, 1), 30);
        let mobs = field(&[(2, 40)]);
        let chosen = share_balanced(&mobs, spawn_capacity(mobs.len(), 1), TEST_SEED);
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

        let chosen = share_balanced(&mobs, cap, TEST_SEED);
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
        let chosen = share_balanced(&mobs, 6, TEST_SEED);
        let ids: Vec<u32> = chosen.iter().map(|m| m.object_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
    }

    /// The edges, because these come off generated data and a panic here takes the server
    /// down on a field entry.
    #[test]
    fn the_edges_do_not_panic_or_overshoot() {
        assert!(share_balanced(&[], 10, TEST_SEED).is_empty());
        assert!(share_balanced(&field(&[(1, 5)]), 0, TEST_SEED).is_empty());

        // A cap at or above the total keeps everything, and never more.
        let mobs = field(&[(1, 3), (2, 2)]);
        assert_eq!(share_balanced(&mobs, 5, TEST_SEED).len(), 5);
        assert_eq!(share_balanced(&mobs, 99, TEST_SEED).len(), 5);

        // And a cap of one still returns exactly one, from the largest type.
        let one = share_balanced(&mobs, 1, TEST_SEED);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].template_id, 1, "the largest share takes the only slot");

        // Every cap from 0 to total is filled exactly, on a ragged mix.
        let ragged = field(&[(7, 1), (3, 13), (9, 4), (1, 2)]);
        for cap in 0..=ragged.len() {
            assert_eq!(share_balanced(&ragged, cap, TEST_SEED).len(), cap, "cap {cap}");
        }
    }

    /// Map 30's six snails are unaffected in kind but not in number, and the rounding is
    /// still NOT settled: floor gives 4 where ceiling would give 5, and the one datapoint we
    /// have (40 -> 30) cannot tell them apart. Pinned so a change is deliberate.
    ///
    /// **The one-point case WAS settled, on 2026-08-20, and it changed.** This used to pin
    /// `spawn_capacity(1, 1) == 0` - a map with a spawn point and no mob on it, forever.
    /// That is not a rounding preference, it is a map that is never populated, and it would
    /// have been nearly invisible in play: a small map with nothing on it looks like a small
    /// map with nothing on it. There is now a floor of one. The 6 -> 4 question is
    /// untouched, because that one really is unsettled.
    #[test]
    fn a_small_map_shows_the_rounding_that_is_still_unsettled() {
        assert_eq!(spawn_capacity(6, 1), 4, "floor(6 * 75 / 100); rounding up would be 5");
        assert_eq!(spawn_capacity(1, 1), 1, "a map with a spawn point is never empty");
        assert_eq!(spawn_capacity(0, 1), 1, "and the floor applies even with nothing to fill");
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

        // **The file grew six requirement columns and a NAME on 2026-08-28, and this loader
        // must ignore them rather than skip the row.** The old `f.len() != 19` skipped every
        // row of the wider file, and the symptom would have been indistinguishable from the
        // 2026-08-19 bug this whole table exists to fix: a shirt with no defence and no
        // upgrade slots. The assertions above only catch it because they read real rows -
        // `equips.len() > 1000` on its own would have caught it too, which is why it is there.
        let text = std::fs::read_to_string(path).unwrap();
        let row = text
            .lines()
            .find(|l| l.starts_with("1302000,"))
            .expect("the starter sword is in the file");
        let fields: Vec<&str> = row.split(',').map(str::trim).collect();
        assert_eq!(fields.len(), 26, "19 numeric + 6 requirement + name: {row}");
        assert_eq!(fields[25], "Sword", "the name is the last column: {row}");
        // reqLevel is field 19 and the starter sword needs nothing at all - it is the one
        // weapon in this client with a completely free entry, which is why `!kit` uses it.
        assert_eq!(fields[19], "0", "the Sword has no level requirement: {row}");
    }

    /// Quest 1000's tree, read back out of the generated table.
    ///
    /// It is the one quest a real client has been observed asking for - the `0x0151` of
    /// 2026-08-19 carried quest id 1000 and npc template 1 - so it is the only row here that
    /// is cross-checked against the wire rather than only against the WZ.
    #[test]
    fn quest_1000_comes_back_with_its_branches_in_order() {
        let path = std::path::Path::new("../../gm-handbook/questlines.txt");
        if !path.exists() {
            return; // generated data, gitignored
        }
        let quests = load_quests(path);
        assert_eq!(quests.len(), 322, "the client ships 322 quests");

        let q = &quests[&1000];
        assert_eq!(q.name, "Borrowing Sera's Mirror");
        assert_eq!(q.start_npc, Some(1), "NPC template 1 starts it - matches the capture");
        assert_eq!(q.end_npc, Some(2), "and template 2 finishes it");
        assert_eq!(q.next_quest, Some(1001));

        // The opening conversation is four lines, and the FIRST one is what the server
        // sends. Getting the order wrong would open the conversation mid-way.
        let opening = &q.say["0"];
        assert_eq!(opening.len(), 4);
        assert!(opening[0].starts_with("You must be the new traveler"), "{}", opening[0]);
        assert!(opening[3].contains("Quest Helper"), "{}", opening[3]);

        // The branches are separate nodes, not more lines of the opening.
        assert!(q.say["0.yes"][0].contains("hill to the east"), "{:?}", q.say["0.yes"]);
        assert!(q.say["0.no"][0].contains("come back when you change your mind"));
        assert!(q.say.contains_key("1.stop.npc"), "{:?}", q.say.keys().collect::<Vec<_>>());

        // The markup is carried raw - whether the client expands it is what the screen
        // will answer.
        assert!(q.say["0.yes"][0].contains("#i4031000#"), "the item icon token survives");

        // Line order is by parsed index, not string order. Find a conversation with ten or
        // more lines and check line 10 follows line 9 - a string sort puts "10" after "1".
        if let Some((qid, lines)) = quests.iter().find_map(|(qid, q)| {
            q.say.get("0").filter(|l| l.len() > 10).map(|l| (qid, l))
        }) {
            assert!(!lines[9].is_empty() && !lines[10].is_empty(), "quest {qid}");
        }
    }

    /// Robin's lines, read back out of the generated table.
    ///
    /// The ten idle lines and their order are the strongest check available: an outside
    /// listing of this NPC's idle chatter matches these ten, in this order, exactly.
    #[test]
    fn robin_has_his_own_dialogue_and_ten_idle_lines_in_order() {
        let path = std::path::Path::new("../../gm-handbook/npcstrings.txt");
        if !path.exists() {
            return; // generated data, gitignored
        }
        let npcs = load_npc_strings(path);
        assert!(npcs.len() > 200, "only {} NPCs", npcs.len());

        let robin = &npcs[&8];
        assert_eq!(robin.name, "Robin");

        // d0 is what they say when talked to, and it carries the raw substitution token.
        // Sending it unexpanded is deliberate - see NpcStrings::dialogue.
        assert_eq!(robin.dialogue.len(), 2);
        assert!(robin.dialogue[0].contains("#p8#"), "{}", robin.dialogue[0]);

        // Ten idle lines, first and last pinned. The order is prefix-major n, f, w, h, so
        // "Yoohoo!" (h0) is last and "Be careful!" (n0) first - a flat alphabetical or
        // per-key sort would put them elsewhere, which is what this pins.
        assert_eq!(robin.chatter.len(), 10);
        assert!(robin.chatter[0].starts_with("Be careful!"), "{}", robin.chatter[0]);
        assert_eq!(robin.chatter[9], "Yoohoo!");
        assert!(robin.chatter[7].contains("Roger and Peter"), "{}", robin.chatter[7]);

        // No line may be empty: an empty balloon is indistinguishable from a broken one.
        assert!(robin.chatter.iter().all(|l| !l.trim().is_empty()));

        // Tabs are the field separator, so no value may contain one.
        assert!(npcs.values().all(|n| {
            !n.name.contains('\t')
                && n.dialogue.iter().chain(&n.chatter).all(|l| !l.contains('\t'))
        }));
    }

    /// A fresh mob is at its template's HP, not at a flat placeholder.
    ///
    /// The client draws the bar as `hp * 100 / maxHP` through an IDIV with no zero guard, so
    /// a snail sent with 100 HP against its template's 30 is at 333%.
    #[test]
    fn a_spawned_mob_carries_its_own_templates_hp() {
        let mobs = std::path::Path::new("../../gm-handbook/mobs.txt");
        let templates = std::path::Path::new("../../gm-handbook/mobtemplates.txt");
        if !mobs.exists() || !templates.exists() {
            return; // generated data, gitignored
        }
        let t = load_mob_templates(templates);
        assert!(t.len() > 100, "only {} templates", t.len());
        assert_eq!(t[&1].max_hp, 30, "the snail");
        assert_eq!(t[&2].max_hp, 45, "map 40's mob");

        // No template may have zero HP: that divide has no guard, and the loader drops such
        // a row rather than letting it reach the wire.
        assert!(t.values().all(|m| m.max_hp > 0));

        let (fields, _) = Config::load_mobs(mobs, &t);
        for (map, want) in [(30u32, 30u64), (40, 45)] {
            let list = &fields[&map];
            assert!(!list.is_empty());
            assert!(
                list.iter().all(|m| m.hp == want),
                "map {map} should spawn at {want} HP, got {:?}",
                list.iter().map(|m| m.hp).take(3).collect::<Vec<_>>()
            );
        }

        // And a template the table does not know still gets a non-zero fallback rather than
        // a division by zero.
        let empty = HashMap::new();
        let (bare, _) = Config::load_mobs(mobs, &empty);
        assert!(bare[&30].iter().all(|m| m.hp == DEFAULT_MOB_HP));
        assert_ne!(DEFAULT_MOB_HP, 0);
    }

    /// The White Map is reachable again, and the retraction is pinned so it is not
    /// re-blocked on the theory the owner's own run killed.
    #[test]
    fn the_white_map_is_reachable_because_the_run_showed_it_loads() {
        let path = std::path::Path::new("../../gm-handbook/fields.txt");
        if !path.exists() {
            return; // generated data, gitignored
        }
        let config = Config { fields: Config::load_fields(path), ..Config::default() };
        assert!(config.fields.contains(&900000000), "the White Map has a field image");
        assert!(
            config.map_exists(900000000),
            "it was denylisted on a theory the run disproved - the owner logged in with this map stored and it loaded"
        );
        for map in [1u32, 10, 20, 30, 40] {
            assert!(config.map_exists(map));
        }
        // And a map with no field image is still refused, which is the check that is real.
        assert!(!config.map_exists(104040000));
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

    /// The overlay fills the holes a script quest leaves and never overwrites the client.
    ///
    /// Quest 1002's `Say."0"` and `Act."0"`/`Act."1"` are all `{}` in this client's WZ -
    /// the opening, the apple and the reward all lived in `q1002s`, which the client does
    /// not ship. `research/quest-scripts.md`.
    #[test]
    fn an_authored_overlay_fills_a_script_quests_holes() {
        let shipped = "1002	QuestInfo	name	Roger's Apple
                       1002	Check	0.npc	3
                       1002	Say	1.stop.item.0	Eat the apple I gave you
";
        let authored = "1002	Say	0.0	You'll die when your HP reaches 0
                        1002	Say	0.1	Shall we?
                        1002	Say	0.yes.0	Open your Item Inventory
                        1002	Act	0.item.0.id	2010000
                        1002	Act	0.item.0.count	1
                        1002	Act	1.exp	3
";

        let mut quests = HashMap::new();
        read_quest_rows(shipped, &mut quests, Overlay::No);
        assert!(!quests[&1002].say.contains_key("0"), "the client ships no opening");

        assert_eq!(read_quest_rows(authored, &mut quests, Overlay::Yes), 1);
        let q = &quests[&1002];
        assert_eq!(q.say["0"], vec!["You'll die when your HP reaches 0", "Shall we?"]);
        assert_eq!(q.say["0.yes"], vec!["Open your Item Inventory"]);
        assert_eq!(q.start_items, vec![(2010000, 1)]);
        assert_eq!(q.complete_exp, 3);

        // Untouched: the name, the NPC and the shipped `stop` line are all still the
        // client's.
        assert_eq!(q.name, "Roger's Apple");
        assert_eq!(q.start_npc, Some(3));
        assert_eq!(q.say["1.stop.item"], vec!["Eat the apple I gave you"]);
    }

    /// A shipped row always wins, in every field the overlay can touch.
    ///
    /// This is the direction that matters: an overlay that quietly replaced a client `Say`
    /// node would put words in an NPC's mouth that the client's own data contradicts, and
    /// nothing on screen would say which source won.
    #[test]
    fn an_overlay_never_overwrites_what_the_client_shipped() {
        let shipped = "1000	QuestInfo	name	Shipped name
                       1000	Say	0.0	The client's own line
                       1000	Act	0.item.0.id	4031000
                       1000	Act	0.item.0.count	1
                       1000	Act	1.exp	2
";
        let authored = "1000	QuestInfo	name	Ours
                        1000	Say	0.0	Our line
                        1000	Act	0.item.0.id	9999999
                        1000	Act	0.item.0.count	7
                        1000	Act	1.exp	500
";

        let mut quests = HashMap::new();
        read_quest_rows(shipped, &mut quests, Overlay::No);
        read_quest_rows(authored, &mut quests, Overlay::Yes);

        let q = &quests[&1000];
        assert_eq!(q.name, "Shipped name");
        assert_eq!(q.say["0"], vec!["The client's own line"]);
        assert_eq!(q.start_items, vec![(4031000, 1)], "not appended to, not replaced");
        assert_eq!(q.complete_exp, 2);
    }

    /// A ten-line node comes out in index order, through the overlay path too.
    ///
    /// The whole reason `read_quest_rows` is factored rather than duplicated: a second
    /// hand-written parser is how `0.10` ends up before `0.2`, and the screen would show it
    /// as a conversation that jumps.
    #[test]
    fn overlay_lines_come_out_in_index_order_not_string_order() {
        let mut rows = String::new();
        for i in 0..12 {
            rows.push_str(&format!("1002	Say	0.{i}	line {i}
"));
        }
        let mut quests = HashMap::new();
        read_quest_rows(&rows, &mut quests, Overlay::Yes);
        let said = &quests[&1002].say["0"];
        assert_eq!(said.len(), 12);
        assert_eq!(said[9], "line 9");
        assert_eq!(said[10], "line 10", "string order would put this second");
    }

    /// A missing overlay file is not an error, and it is not a silent one either.
    #[test]
    fn a_missing_overlay_file_touches_nothing_and_says_zero() {
        let mut quests = HashMap::new();
        read_quest_rows("1000	QuestInfo	name	Kept
", &mut quests, Overlay::No);
        let n = overlay_quests(&mut quests, std::path::Path::new("no/such/overlay.txt"));
        assert_eq!(n, 0);
        assert_eq!(quests[&1000].name, "Kept");
    }
}
