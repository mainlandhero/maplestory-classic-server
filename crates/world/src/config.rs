//! What one channel server needs to know before it can listen.

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// One channel of one world.
///
/// A channel is a process, not a thread: the login server hands the client an address and
/// the client connects to it, so every channel needs its own listener and its own
/// advertised address. `world_id` and `channel_id` are here so the log says which channel
/// a line belongs to, and so a migration minted for channel 1 is not claimed by channel 2.
///
/// # Cloning one SHARES its NPC dialogue rather than copying it
///
/// `serve` builds exactly one of these, wraps it in an `Arc`, and hands a clone of the
/// **`Arc`** to every connection - so all of them share one allocation, and [`NpcStringTable`]
/// is swapped in place through it. That is the only reason `!npcreload` can reach a session
/// which connected before it ran.
///
/// A `Config::clone` is a different thing and only tests do it, but the obvious deep copy
/// would be a trap: a session holding the copy would go on serving the old dialogue with
/// nothing in any log to say so. [`NpcStringTable`] is therefore `Clone` by sharing its lock,
/// so the two halves of a cloned `Config` reload together.
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

    /// **The world hub (`maplecw-chat`) this channel dials**, or `None` to run the channel on
    /// its own - parties and party chat then stay per-channel, exactly as before 2026-09-14.
    /// `--link ADDR`, default `127.0.0.1:8483`; `--link none` clears it. `crate::link`.
    pub link: Option<std::net::SocketAddr>,

    /// **Which host a Change Channel answer names** - `--advertise`, the same flag and the
    /// same rule as the login server's, because both write a channel address into a packet
    /// the client dials. `channels` above keeps the ports; this decides the host per
    /// connection. `net::advertise`.
    pub advertise: std::sync::Arc<net::advertise::Advertiser>,

    /// Whether the channel answers packets at all. **On, and it should stay on.**
    ///
    /// With it clear, `Session::handle` and `Session::tick` return nothing for *every*
    /// packet: the migration hello goes unanswered and the client sits on "Connecting..."
    /// looking exactly like a server that is not running.
    ///
    /// | in | out |
    /// |---|---|
    /// | `0x007D` migration hello | `SetField` with a full character record on the map |
    /// | `0x00D1` transfer field | `SetField` for the portal's target map and arrival portal |
    /// | `0x00DC` field entered | every NPC on the map, then a `UserAvatarModified` attempt |
    ///
    /// **It was `false` until 2026-09-14, reached by `--set-field-probe`**, on the reasoning
    /// that "it is the whole game path and nothing on it authenticates anybody". Nothing on
    /// it authenticates anybody still (`CLAUDE.md`), but the default bought no safety: the
    /// shipped `tools/installer/start-server.ps1` passed the flag unconditionally, and so did
    /// all sixty test sites and `tools/test-server.ps1`. **Sixty-one callers, every one of
    /// them turning it on.** What the default actually did was cost the owner a manual launch on
    /// 2026-08-20, when a launcher line without `-SetFieldProbe` left the character on the
    /// select screen and read as a server bug for a whole run.
    ///
    /// A default nobody chooses is not a safety measure, it is a trap with a docstring. The
    /// off case is now `--silent-channel`, which has to be asked for by name, and its one
    /// real use is eliminating the channel as a variable.
    pub answer_packets: bool,

    /// **What this channel does when a claiming connection's address is not the one the
    /// migration was minted for** - `--migration-peer-policy`. `Require` (the default since
    /// 2026-09-05) refuses it; `Record` logs it and lets it through, which is what every run
    /// before that did.
    ///
    /// The owner: *"is there a way to enforce that the initial character enter has to be from an
    /// authenticated session from our launcher?"* For a client on another machine the address
    /// is the one fact the channel connection and the login connection share, so requiring it
    /// is the whole of the off-box enforcement; on this machine the OS attestation of the
    /// owning process does the stronger job (`claim_for_character`). Addresses are compared
    /// after normalisation - `::ffff:a.b.c.d` is `a.b.c.d`, and the two loopbacks are one -
    /// which is what removed the dual-stack false refusal that kept this at `Record`.
    pub peer_policy: store::migration::PeerPolicy,

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

    /// **A test lever, not a game rule:** the `moveAction` byte a summoned pet is given in
    /// `0x0277`, for one run. `--pet-move-action N`. `None` sends the normal `0`.
    ///
    /// The client stores that byte at `pet+0x2d8` and `FUN_141ebe350` decodes it - bit 0 the
    /// facing, `(v >> 1) - 1` an index into a fifteen-entry stance table - and `FUN_141ec7e90`
    /// turns the result into a bool for `CPet::SetStance`: **0 for every ordinary value, 1
    /// only when the table gives 8, which is `moveAction` 30 or 31**. Stance 0 is the land
    /// arm, and on 2026-09-14 that arm was read all the way down: it positions the pet's layer
    /// but **never calls the layer's `put_z` (`vtbl+0x198`)**, because the field z that
    /// `FUN_141eca710` computes is passed to `FUN_141ecaa40` in `edx`, which never reads it.
    /// Stance 1 - the flying arm, `FUN_141ec22f0` - does call `put_z`, and the Husky has a
    /// two-frame `fly` animation. So `30` here is the one-byte experiment that says whether
    /// the missing z is why a summoned pet is invisible on the field while the Character Info
    /// window draws the same pet perfectly. `research/pet-draw-chain-2026-09-14.md` §12.
    pub pet_move_action: Option<u8>,

    /// **Whether a summoned pet is shown to OTHER players in the map.** Default `true` - the
    /// social point of a pet is that other people see it.
    ///
    /// The owner, 2026-09-13, asked for it: *"broadcast player pet movement so other people can
    /// see pets moving even if it is not their own."* 2026-09-15: *"pets need to be animated
    /// across all clients, this is at the core of the social aspect of the game."*
    ///
    /// The first attempt **crashed** the observer on the pet's first step, because
    /// [`net::pet::pet_move_broadcast`] forwarded the client's `0x0202` path verbatim and that
    /// path has no leading key `u32`, so the applier read the pet's X coordinate as the element
    /// count, appended nothing, and dereferenced the empty list tail
    /// (`research/pet-remote-crash-2026-09-15.md`, faulting `rax = 0` at `0x141d59bf3` in the
    /// dump). The move builder now inserts the `0x0202` tick as that key, so the observer reads
    /// the real x/y/count, and drops any (never-observed) zero-element path defensively.
    ///
    /// Left as a flag so a run can turn broadcasting **off** (`--broadcast-pets false` is not a
    /// thing; the launch default is on and `Config::default` is on) if the remote pet ever
    /// misbehaves again - a pet whose move desyncs is a visual glitch, but the owner-local mode
    /// is the safe fallback that never touches another client.
    pub broadcast_pets: bool,

    /// **How the other players' copy of a character is redressed** when its look changes -
    /// an equip on or off, a hair or face coupon, the pet's hat.
    ///
    /// `false` (the default): one `0x02AE` per observer - the user pool's own in-place
    /// redress, which decodes the look into the pooled user and rebuilds its avatar
    /// (`net::lookupdate`; `research/remote-redress-2026-09-18.md`). No leave, no enter, no
    /// blink, the pet copy untouched. The owner, 2026-09-18: *"Please find another suitable way
    /// without leave-and-enter."* Never on a screen yet; it rides the chair relay's router.
    ///
    /// `true` (`--look-reenter`): `0x0225` then `0x0224` then the pet for that one character,
    /// what a fresh sighting gets. Works on any client; the observer's copy blinks and its pet
    /// respawns. The fallback if `0x02AE` is refuted on screen.
    ///
    /// (`0x0138 UserAvatarModified`, tried 14:07 the same day, is retired: its apply walks the
    /// user's summoned map, never the player - measured inert, then read.)
    pub look_change_reenter: bool,

    /// **List the hair and the face in another player's Character Info ITEM tab** (default
    /// `true`; `--no-look-items` turns it off). A hair or face entry is an equip slot under
    /// the look id, and its icon is one `tools/backport_install.py` renders into the hybrid
    /// Hair and Face archives (`look_icons`) - on a client without that install the entry
    /// asks the widget for an icon that is not there. The owner, 2026-09-18. `session/charinfo.rs`.
    pub charinfo_look_items: bool,

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

    /// **Every map's spawn points**: map -> the indices of its portals named **`sp`** that lead
    /// nowhere and run no script. The name is the rule, measured on `gm-handbook/portals.txt`:
    /// every one of its 426 maps has at least one `sp`, and the other target-0 portals are
    /// special-purpose - `tp` (72, the Mystic Door's town points), `st00` (51, a stage's arrival),
    /// `h001`, `start00`, Kerning City's `pc00` and `cab00` - not places to put a player down.
    ///
    /// The owner, 2026-09-26: a character comes back in at the spawn point **nearest** to where they
    /// left, and a teleport lands on a **random** one. [`Self::nearest_spawn_point`],
    /// [`Self::random_spawn_point`].
    pub spawn_points: HashMap<u32, Vec<u8>>,

    /// `(map, portal index)` -> where that portal stands, in map pixels.
    ///
    /// The owner, 2026-09-14: *"The first client also sees the client joining start from the origin
    /// of the map and then snap to their real position."* After a warp `last_position` was
    /// deliberately `None`, and the `0x0224` announced to the field fell back to `(0, 0)` until
    /// the newcomer's first step. The arrival portal is a fact the server already knows - it
    /// put it in the SetField - so the announcement stands there instead, and a drop placed
    /// before the first step lands there too. Empty when `portals.txt` predates the columns;
    /// every reader falls back to what it did before.
    pub portal_positions: HashMap<(u32, u8), (i16, i16)>,

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
    /// Every map's reactors - the breakable boxes - keyed by map id, from
    /// `gm-handbook/reactors.txt`. Server-sent like NPCs and mobs: the client's field loader
    /// walks the WZ `reactor` node only to preload `Reactor/%07d.img`. The owner, 2026-09-13:
    /// Pio's quest items come out of Wooden Boxes nobody was placing. 19 on the six Amherst
    /// maps. `crate::session::reactor`.
    pub reactors: HashMap<u32, Vec<ReactorSpawn>>,
    /// What a reactor gives when it breaks, keyed by `Reactor.wz` id - `data/reactor-drops.txt`,
    /// the same shape as `drops.txt`.
    pub reactor_drops: crate::droptables::DropTables,
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
    /// Every mob template's full stat row, for building a **forced stat** block.
    ///
    /// `mob_attack` above is the same `PADamage` in a flatter shape and predates this. Both
    /// are kept because they answer different questions: that one is "what does the SERVER
    /// think this hit for", this one is "what should the CLIENT be told the mob is".
    pub mob_templates: HashMap<u32, MobTemplate>,
    /// Every mob's MP, MP-costing attacks and skills, and the skill levels they name -
    /// `gm-handbook/mobskills.txt`. `crate::mobskills`.
    pub mob_skills: crate::mobskills::MobSkillTable,
    /// What each quest requires: mobs to kill and items to hold. `gm-handbook/questreq.txt`.
    ///
    /// Generated from the client's own `Quest.wz`, so this is the client's data rather than
    /// a guess - unlike the drop chances or the level gains.
    pub quest_reqs: net::quest::QuestRequirementTable,
    /// **Which items carry `info/quest`, and which quest wants each one.**
    ///
    /// The owner, from a screenshot of a full ETC tab: a quest item must only be offered to a
    /// player who has the quest ACTIVE. Built at startup from
    /// [`Config::shops`]`.item_data` (the flag) and `gm-handbook/questreq.txt` (the
    /// mapping), so there is one parse of `itemdata.txt` in the process and the drop rule
    /// and the "may not be sold" rule cannot disagree about what a quest item is.
    ///
    /// **Empty is legal and gates nothing** - `gm-handbook/` is generated and gitignored.
    /// `QuestItems::is_armed` says which it is, and the startup banner prints it, because a
    /// guard that quietly disappears is worse than one that refuses. See
    /// [`crate::questitems`].
    pub quest_items: crate::questitems::QuestItems,
    /// Every equip's template values, keyed by item id, from `gm-handbook/equips.txt`.
    ///
    /// The character record carries an item's stats and upgrade slots per *instance*, and a
    /// real server fills them from the template when the item is created. An empty table is
    /// not fatal - items are still sent, just bare - so a missing file degrades to exactly
    /// the behaviour confirmed on screen on 2026-08-19.
    pub equips: HashMap<u32, EquipTemplate>,
    /// **The client's own 208 scrolls**, keyed by item id, from `gm-handbook/scrolls.txt`.
    ///
    /// Wanted by the Treasure Scroll, which guarantees a real scroll the player is carrying -
    /// so it has to know what that scroll would have granted. Empty is legal and degrades to
    /// "you are not carrying a scroll I can use", which is a refusal the player is told about
    /// rather than a silent no-op.
    pub scrolls: HashMap<u32, ScrollTemplate>,
    /// What each summoning sack lets out, keyed by item id, from
    /// `gm-handbook/summonsacks.txt`.
    ///
    /// Empty is legal and degrades to the behaviour before `0x0111` was handled: the sack is
    /// refused, the latch is cleared, and the log says the table is missing rather than
    /// leaving a silent no-op that looks identical to the unhandled opcode.
    pub summon_sacks: HashMap<u32, SummonSack>,
    /// What each chair adds to the idle tick, keyed by item id, from `gm-handbook/chairs.txt`.
    ///
    /// Empty is legal and degrades to exactly the behaviour before chairs existed: the tick
    /// stays at its flat base. `gm-handbook/` is generated and gitignored, so a clean checkout
    /// has no table until `python tools/dump_chairs.py` runs. `world::chairs`.
    pub chairs: HashMap<u32, crate::chairs::Chair>,
    /// Every crafting recipe, keyed by the client's own recipe key, from
    /// `gm-handbook/craftrecipes.txt`.
    ///
    /// Empty is legal and is answered with "This function is currently unavailable" rather
    /// than with a craft that silently does nothing - `world::crafting`. `gm-handbook/` is
    /// generated and gitignored, so a clean checkout has no table until
    /// `python tools/dump_craftrecipe.py` runs.
    pub recipes: crate::crafting::Recipes,
    /// Every NPC template's name, spoken dialogue and idle chatter, keyed by template id.
    ///
    /// **Behind a lock, so `!npcreload` reaches connections that are already open.** Every
    /// session on a channel shares one `Arc<Config>`, taken at accept time - replacing the
    /// server's `Arc` would do nothing for anybody already playing. See [`NpcStringTable`].
    pub npc_strings: NpcStringTable,
    /// Every pet's chat commands, from `gm-handbook/petcommands.txt`. `crate::petcommands`.
    pub pet_commands: crate::petcommands::PetCommands,

    /// Where [`NpcStringTable::base`] was read from. **Reported, not re-read.**
    ///
    /// It is carried so `!npcreload` can name the file in its answer. The generated base is
    /// deliberately *not* re-read - see [`reload_npc_dialogue`] for why that is a guarantee
    /// rather than a shortcut.
    pub npc_strings_path: PathBuf,

    /// The hand-authored dialogue overlay `!npcreload` re-reads. `data/npc-dialogue.txt`.
    ///
    /// **Authored and committed, like `data/shops.txt` and `data/quest-scripts.txt`.** It is
    /// deliberately not in `gm-handbook/`: that directory is generated from the client's WZ
    /// and `CLAUDE.md` says never hand-edit it, so writing dialogue there would be destroyed
    /// by the next `tools/dump_npcstrings.py` run.
    pub npc_dialogue_path: PathBuf,
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

    /// Maps whose Map.wz image declares a top-level `clock` node - the ones that build a
    /// wall-clock widget for `0x01BC` to set. From `gm-handbook/clocks.txt`.
    ///
    /// Empty means no clock is ever sent, which is the safe direction: the client's
    /// set-time path throws for a widget the map did not build. `net::clock`.
    pub clocks: std::collections::HashSet<u32>,

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

    /// **Every hair id this client can draw**, from `gm-handbook/beauty.txt`'s `[hair]`
    /// rows: each base id and its colour range (`0-7` or `0-8`). The salon
    /// (`crate::salon`) keeps a player's colour digit across a style change only when the
    /// new base has that colour; an id outside this set draws bald. Empty when the file is
    /// not there, and `hair_exists` then says no - the fail-safe direction, colour 0.
    pub hair_ids: HashSet<u32>,
    /// Every face id this client draws: `gm-handbook/beauty.txt`'s `[face]` rows, each
    /// style at all its eye colours (`styleId + 100*c`). Same shape and same reason as
    /// [`Self::hair_ids`].
    pub face_ids: HashSet<u32>,
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
        let (links, index, _) = Self::load_portals_with_positions(path);
        (links, index)
    }

    /// [`Self::load_portals`] plus where each portal stands - `(map, index) -> (x, y)`, from
    /// the `x, y` columns `tools/dump_portals.py` writes. A `portals.txt` without them still
    /// loads; the position table is then empty and arrivals fall back to the origin.
    #[allow(clippy::type_complexity)]
    pub fn load_portals_with_positions(
        path: &std::path::Path,
    ) -> (HashMap<(u32, String), (u32, String)>, HashMap<(u32, String), u8>, HashMap<(u32, u8), (i16, i16)>) {
        let (mut links, mut index, mut positions) = (HashMap::new(), HashMap::new(), HashMap::new());
        let Ok(text) = std::fs::read_to_string(path) else { return (links, index, positions) };
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
                if let (Some(Ok(x)), Some(Ok(y))) = (f.get(6).map(|v| v.parse::<i16>()), f.get(7).map(|v| v.parse::<i16>())) {
                    positions.insert((map, idx), (x, y));
                }
            }
            if target != 0 {
                links.insert((map, f[2].to_string()), (target, f[4].to_string()));
                continue;
            }
            // **A script portal has target 0 and still leads somewhere.** The owner, 2026-09-09:
            // *"Ellinia should also have a portal to go to Ellinia Station right here, but
            // this portal does not exist where I expect it."* It does - `Map.wz` puts its
            // destination in a `script` name rather than in `tm`, and the dumper used to drop
            // that field, so 41 portals arrived here looking exactly like spawn points.
            //
            // Folded into `links` rather than kept in a second table on purpose: every
            // existing caller - the transfer handler, the taxi's reachability walk, the
            // return-scroll check - then treats them as the ordinary portals they are.
            // `crate::scriptportals` resolves only the ones whose destination is DERIVED from
            // the data; the rest stay dead and are listed there by name.
            if let Some(script) = f.get(5).filter(|s| !s.is_empty()) {
                if let Some((to_map, to_portal)) = crate::scriptportals::destination(script) {
                    links.insert((map, f[2].to_string()), (to_map, to_portal.to_string()));
                }
            }
        }
        (links, index, positions)
    }

    /// The spawn points of every map in `portals.txt` - see [`Self::spawn_points`]. A missing file
    /// is an empty table, and every caller then falls back to portal 0 as before.
    pub fn load_spawn_points(path: &std::path::Path) -> HashMap<u32, Vec<u8>> {
        let mut out: HashMap<u32, Vec<u8>> = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // map, index, name, target map, target portal, script, x, y
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            if f.len() < 5 {
                continue;
            }
            let (Ok(map), Ok(idx), Ok(target)) = (f[0].parse::<u32>(), f[1].parse::<u8>(), f[3].parse::<u32>()) else {
                continue;
            };
            if f[2] == "sp" && target == 0 && f.get(5).is_none_or(|s| s.is_empty()) {
                out.entry(map).or_default().push(idx);
            }
        }
        out
    }

    /// **The spawn point on `map` nearest `(x, y)`**, by straight-line distance - where a
    /// character who logged off, changed channel or went into the Cash Shop there comes back in.
    /// `None` when the map has no spawn point with a known position.
    pub fn nearest_spawn_point(&self, map: u32, (x, y): (i16, i16)) -> Option<u8> {
        self.spawn_points.get(&map)?.iter().copied().filter_map(|idx| {
            let (px, py) = *self.portal_positions.get(&(map, idx))?;
            let (dx, dy) = (i64::from(px) - i64::from(x), i64::from(py) - i64::from(y));
            Some((dx * dx + dy * dy, idx))
        }).min().map(|(_, idx)| idx)
    }

    /// **A random spawn point on `map`**, for a teleport - the owner, 2026-09-26: *"if a player is
    /// teleported into a map, the server will choose a random spawn point. Such as when Nella
    /// teleports the player back to Kerning City."* `0`, the map's default, when it has none.
    pub fn random_spawn_point(&self, map: u32, roll: u64) -> u8 {
        match self.spawn_points.get(&map) {
            Some(all) if !all.is_empty() => all[(roll % all.len() as u64) as usize],
            _ => 0,
        }
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
    /// Load `gm-handbook/beauty.txt`'s `[hair]` section into every drawable id:
    /// `baseId, gender, colours, name` rows, `colours` being `0-7` or `0-8`; the colour is
    /// the id's last digit.
    pub fn load_hair_ids(path: &std::path::Path) -> HashSet<u32> {
        Self::load_beauty_ids(path, "[hair]", 1)
    }

    /// The `[face]` section the same way: `styleId, gender, eyeColours, name`, the eye
    /// colour being the hundreds digit (`20003` at colour 1 is `20103`).
    pub fn load_face_ids(path: &std::path::Path) -> HashSet<u32> {
        Self::load_beauty_ids(path, "[face]", 100)
    }

    fn load_beauty_ids(path: &std::path::Path, section: &str, stride: u32) -> HashSet<u32> {
        let mut out = HashSet::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        let mut inside = false;
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                inside = line.starts_with(section);
                continue;
            }
            if !inside || line.is_empty() || line.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split(',').map(str::trim).collect();
            if cols.len() < 3 {
                continue;
            }
            let (Ok(base), Some((lo, hi))) = (cols[0].parse::<u32>(), cols[2].split_once('-')) else { continue };
            let (Ok(lo), Ok(hi)) = (lo.parse::<u32>(), hi.parse::<u32>()) else { continue };
            for c in lo..=hi {
                out.insert(base + c * stride);
            }
        }
        out
    }

    /// Whether this client draws hair id `id`. `false` on an empty table.
    pub fn hair_exists(&self, id: u32) -> bool {
        self.hair_ids.contains(&id)
    }

    /// Whether this client draws face id `id`. `false` on an empty table.
    pub fn face_exists(&self, id: u32) -> bool {
        self.face_ids.contains(&id)
    }

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

    /// Load `gm-handbook/clocks.txt` - `map, x, y, width, height`, one row per map whose
    /// Map.wz image has a top-level `clock` node.
    ///
    /// Only the map id is kept. The placement is the client's business - it builds the
    /// widget itself - and the server's whole job is to know WHERE one exists, because
    /// `0x01BC` on a map without a widget throws in the client. `net::clock`.
    pub fn load_clocks(path: &std::path::Path) -> std::collections::HashSet<u32> {
        let mut out = std::collections::HashSet::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(Ok(id)) = line.split(',').next().map(|f| f.trim().parse::<u32>()) {
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
            // Column 25, `cash`, added 2026-09-11 after the six requirement columns. A file
            // from before then has the name there, which does not parse, and means "not cash".
            let cash = f.get(25).and_then(|x| x.parse::<u32>().ok()).unwrap_or(0) != 0;
            // Column 19, `reqLevel` - read for item variance (`crate::variance`), which takes
            // its range from it. A file without it reads as level 0: no variance.
            let req_level = f.get(19).and_then(|x| x.parse::<u16>().ok()).unwrap_or(0);
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
                    cash,
                    req_level,
                },
            );
        }
        out
    }

    /// Which bag an item goes in - `store::InventoryType::for_item`, except that a **cash
    /// equip goes to the Deco tab**, which is where the client will look for it.
    ///
    /// The owner, 2026-09-11, after opening the Übel set: *"It says I also got the outfit, but I
    /// see nothing in my decoration inventory."* The four equips had gone into the Equip tab
    /// by their leading digit. See [`EquipTemplate::cash`].
    pub fn tab_for(&self, item_id: u32) -> Option<store::InventoryType> {
        let tab = store::InventoryType::for_item(item_id)?;
        if tab == store::InventoryType::Equip && self.equips.get(&item_id).is_some_and(|t| t.cash) {
            return Some(store::InventoryType::Deco);
        }
        Some(tab)
    }

    /// Every real scroll, from `tools/dump_scrolls.py`'s `scrolls.txt`.
    ///
    /// Same skip-the-row-rather-than-half-read rule as [`Config::load_equips`], and the same
    /// reason: a partially parsed scroll would grant a wrong number of stat points with
    /// nothing downstream able to tell.
    ///
    /// **The column order is NOT the same as `equips.txt`'s** - this file puts `incWAT` at 9
    /// and `incSpeed`/`incJump` at 15/16, where the equip file has them at 10 and 8/9. The
    /// indices below are read off that file's own header line and nothing else; copying the
    /// equip loader's order would compile, run, and silently grant Speed where the player was
    /// promised Accuracy.
    pub fn load_scrolls(path: &std::path::Path) -> HashMap<u32, ScrollTemplate> {
        let mut out = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            // 19 numeric fields then a name, which is not numeric - hence the slice.
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
                ScrollTemplate {
                    success: u(1),
                    cursed: u(2),
                    increments: net::opcode::EquipStatSet {
                        inc_str: u(3),
                        inc_dex: u(4),
                        inc_int: u(5),
                        inc_luk: u(6),
                        inc_mhp: u(7),
                        inc_mmp: u(8),
                        inc_wat: u(9),
                        inc_mad: u(10),
                        inc_pdd: u(11),
                        inc_mdd: u(12),
                        inc_acc: u(13),
                        inc_eva: u(14),
                        inc_speed: u(15),
                        inc_jump: u(16),
                        inc_crt: u(17),
                        inc_crd: u(18),
                        // **Not a column, and it must not be invented.** This client's WZ has
                        // no `incPAD` on any scroll either - `tools/dump_scrolls.py`'s census
                        // lists the fields it found and `incPAD` is not among them. Bit 8 is
                        // zero here for the same reason it is zero in `fresh_stats`.
                        inc_pad: 0,
                    },
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

/// One reactor placement: a row of `gm-handbook/reactors.txt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactorSpawn {
    /// Per-map, from [`REACTOR_OBJECT_ID_BASE`] up, in file order. Its own range so it can
    /// never collide with a mob's spawn-point id or a summon's.
    pub object_id: u32,
    /// The `Reactor.wz` image id: 1 is the Wooden Box.
    pub template_id: u32,
    pub x: i16,
    pub y: i16,
    /// Seconds from breaking to standing again - the WZ `reactorTime`.
    pub respawn_s: u32,
    pub flip: bool,
    /// The state a hit on state `break_at - 1` produces: the broken one. The Wooden Box has
    /// events on states 0..3 and none on 4, so 4. Counted from `Reactor.wz` by the dumper.
    pub break_at: u8,
    pub name: String,
}

/// Where reactor object ids start on every map. Mobs' spawn points are 2000.., summons have
/// their own base; a distinct range means a reactor id can never be mistaken for either.
pub const REACTOR_OBJECT_ID_BASE: u32 = 6000;

impl Config {
    /// `gm-handbook/reactors.txt`: `map, index, reactorId, x, y, reactorTime, f, breakAt, name`.
    /// A missing file is an empty table - no boxes anywhere - and the server says so at
    /// start; see `world_server.rs`.
    pub fn load_reactors(path: &std::path::Path) -> HashMap<u32, Vec<ReactorSpawn>> {
        let mut out: HashMap<u32, Vec<ReactorSpawn>> = HashMap::new();
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.splitn(9, ',').map(str::trim).collect();
            if f.len() < 8 {
                continue;
            }
            let n = |i: usize| f[i].parse::<i64>().ok();
            let (Some(map), Some(template), Some(x), Some(y), Some(time), Some(flip), Some(break_at)) =
                (n(0), n(2), n(3), n(4), n(5), n(6), n(7))
            else {
                continue;
            };
            let list = out.entry(map as u32).or_default();
            list.push(ReactorSpawn {
                object_id: REACTOR_OBJECT_ID_BASE + list.len() as u32,
                template_id: template as u32,
                x: x as i16,
                y: y as i16,
                respawn_s: u32::try_from(time).unwrap_or(120).max(1),
                flip: flip != 0,
                break_at: u8::try_from(break_at).unwrap_or(1).max(1),
                name: f.get(8).map(|s| s.to_string()).unwrap_or_default(),
            });
        }
        out
    }
}

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
/// **The cap is not in the WZ, and that is measured rather than assumed - across every map,
/// not just this one.** Map 40's own `info` node has no capacity field, but that alone is a
/// filter and not an enumeration: map 40 could simply be a map carrying no override. So the
/// question was widened rather than re-asked. All **426** field images were read and the
/// whole key space of their `info` nodes collected - **57 distinct keys**, 426/426 readable,
/// none lacking an `info` node - and there is **no** capacity, cap, quota or share field of
/// any name on any map. `mobRate` is present 426/426 (`1.0000002` on map 40) but that is a
/// respawn *rate*, not a cap, and a byte scan of the client image finds it **0 times** in
/// either encoding, so the client does not even read it. So the cap is **server policy**, and
/// it has to come from us. The full enumeration, with its controls, is in
/// [`choose_spawns`] and `research/mob-spawn-selection.md`.
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

/// A tiny splitmix64, so the spawn choice can be random without a dependency and still be
/// reproducible from a seed.
///
/// Deliberately not a good general-purpose PRNG and deliberately not `rand`: this picks which
/// of forty spawn points are used, a decision with no security property and one caller. What
/// it does need is to be **seedable**, so a test can pin an exact selection and so two runs of
/// the same map do not lay the mobs out identically.
pub(crate) fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Choose which spawn points hold a live mob: a **uniform random sample** of the whole set.
///
/// The owner, 2026-09-01: *"I'm starting to doubt myself that the mob cap is not split by ratio,
/// but rather randomly picked out across all of the mob spawns. The ratio of the mobs on the
/// map is less important than what we previously thought."*
///
/// This replaces a largest-remainder (Hamilton) apportionment that **enforced** each type's
/// share of the map. What it does not replace is the older fix underneath it: taking the
/// first N points in WZ order is still wrong, and still ruled out by a test, because `life`
/// entries run left to right and the generated table is grouped by type - so the first N are
/// both bunched at the low-x end and often missing whole types.
///
/// # Nothing in this client can decide which spawn points hold a mob
///
/// That sentence is the result, and it means this is a design decision rather than a
/// discoverable fact. Three independent measurements, none of which cost a client run:
///
/// **[L]** Enumerated rather than searched for: all **426** of this client's field images
/// were read and the *whole key space* of their `info` nodes collected - **57 distinct
/// keys**, 426/426 readable, none without an `info` node. The only spawn-related one is
/// `mobRate` (426/426, a float, `1.0000002` on map 40), and a rate is not a cap. There is no
/// capacity, quota, cap or share field of any name on any map. The same pass collected every
/// key on all **9928** `life` entries of `type == "m"`: `id`, `type`, `x`, `y`, `cy`, `fh`,
/// `rx0`, `rx1` on all of them, then `f`, `mobTime`, `hide`, `forcedZMass`, `forcedZPage`,
/// `limitedname`, `useDay`, `useNight`. No weight, no priority, no group.
///
/// **[L]** A second instrument, structurally different - a byte scan of
/// `client-patched\MapleStory.exe` for each key name. `mobRate` is **0 ASCII, 0 UTF-16LE**:
/// the client never looks up the one rate key its own data carries. `mobTime` is 0/0 too.
/// Positive controls hit (`fieldType` 2/2, `foothold` 4/4, `returnMap` 1/3, `town` 9/5) and
/// negative controls do not (`notAKeyAtAll`, `capacityZZ`, `returnMapZZ` all 0/0), so the
/// scan discriminates. *Stated blind spot, so it can be quoted: a name assembled at runtime
/// or stored compressed would not appear - this scan proves presence, never absence, which
/// is why the WZ enumeration above and not the scan is what carries the negative.*
///
/// **[L]** And underneath both, the client cannot spawn a mob at all: its field loader walks
/// `life` only to preload `Mob/%07d.img` art, and a populated mob is only ever built from a
/// packet. `research/npc-spawn.md` §2, `research/mob-spawn.md`.
///
/// The capacity-shaped names that *do* appear in the image - `maxMobCount`, `mobCount` - are
/// accounted for and are not this: they sit inside the skill-attack property tables beside
/// `hitLimitEveryMob`, `minAttackableCount` and `mobCountDamR`, which is how many mobs one
/// swing may hit. Reported rather than passed over, because a candidate that comes back
/// non-zero and is waved away is how a clean confident zero gets written down.
///
/// # The ratio is preserved in expectation, not enforced
///
/// Uniform sampling does **not** throw the ratio away, and that is arithmetic rather than
/// opinion. Drawing `cap` of `total` points uniformly without replacement makes each type's
/// count **hypergeometric**, so its expectation is *exactly* its share of the map:
/// `E = cap * points_of_type / total`. [D], from the sampling scheme, and asserted against
/// this code rather than merely asserted - see `each_types_expected_share_is_its_share`.
///
/// Map 1006, "Hunting Ground Middle of the Forest II" - 45 points, five types, cap 33 - is
/// the worked example the
/// old doc used, kept so the two can be read side by side:
///
/// | type | points | share | E\[count\] | SD | middle 90% | old rule |
/// |---|---|---|---|---|---|---|
/// | Snail | 10 | 22.2% | 7.33 | 1.25 | 5..9 | 7 |
/// | Blue Snail | 16 | 35.6% | 11.73 | 1.44 | 9..14 | 12 |
/// | Shroom | 7 | 15.6% | 5.13 | 1.09 | 3..7 | 5 |
/// | Red Snail | 6 | 13.3% | 4.40 | 1.02 | 3..6 | **5** |
/// | Orange Mushroom | 6 | 13.3% | 4.40 | 1.02 | 3..6 | **4** |
///
/// So a type drifts about **one slot** either side of its share, and `P(any type missing
/// entirely) <= 2.4e-4` - about one draw in 4092. The spread is small *because* the cap is
/// 75%: only 12 of 45 points are left out, and the finite-population correction
/// `(total - cap) / (total - 1) = 3/11` divides the variance by 3.7. A looser cap would
/// drift further.
///
/// The last column is the argument the other way, and it is the reason enforcing was never
/// free. Red Snail and Orange Mushroom hold the **same** 6 points of 45 - the same share -
/// and largest-remainder handed them 5 and 4, on every spawn, for the life of the process,
/// because the tie was broken by template id. Enforcing the ratio is what made two equal
/// types permanently unequal. Sampling gives both 4.40.
///
/// # The draw happens once per map, per channel
///
/// `Fields::seed` books every chosen point once and `Fields::hurt` re-books *the point that
/// died*, so a respawn returns to its own point and the chosen set is frozen until the
/// process restarts. That is deliberate rather than incidental - `research/mob-spawn-selection.md`
/// §4 - and it means the spread above is the spread of **one draw that lasts a whole
/// session**, not something that averages out over an evening.
///
/// Returns the chosen mobs in spawn order, so the wire order does not depend on the draw.
pub fn choose_spawns(
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

    // Partial Fisher-Yates across **all** the points, with no grouping by template: only the
    // first `cap` positions have to be settled, and drawing `j` uniformly from `i..total`
    // makes every `cap`-subset equally likely.
    //
    // That uniformity is the whole property. Nothing here knows what a template is, and that
    // is exactly why each type's expected count comes out at its share of the map - the draw
    // cannot favour or disfavour a type it cannot see.
    //
    // `% (total - i)` is modulo-biased by about `total / 2^64`, which is 2e-18 at total = 45,
    // against the 1e-4 probabilities in the table above. Named rather than ignored, and far
    // too small to be worth rejection sampling.
    let mut rng = seed;
    let mut pool: Vec<usize> = (0..total).collect();
    for i in 0..cap {
        let j = i + (splitmix64(&mut rng) % (total - i) as u64) as usize;
        pool.swap(i, j);
    }
    pool.truncate(cap);
    // Sorted so the packets still go out in map order, which is what the client expects and
    // what makes two logs of the same field comparable.
    pool.sort_unstable();
    pool.into_iter().map(|i| &mobs[i]).collect()
}

/// **A compatibility shim - delete this.** The name from when this enforced each type's
/// share of the map, kept only because `crates/world/src/fields.rs` still calls it and that
/// file belongs to another agent in this session. Point `Fields::seed` at [`choose_spawns`]
/// and remove this function; it adds nothing but a second name for one behaviour, and a
/// second name is how a stale one survives.
pub fn share_balanced(
    mobs: &[net::mob::FieldMob],
    cap: usize,
    seed: u64,
) -> Vec<&net::mob::FieldMob> {
    choose_spawns(mobs, cap, seed)
}

/// What one summoning sack lets out.
///
/// From `0210.img/<id>/mob`, a **sibling of `info`** rather than a child of it - the brief
/// that decoded this said "in the consumable's info block" and it is not there.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SummonSack {
    /// The mob templates, **one entry per mob to spawn**.
    ///
    /// A template listed twice in the WZ means spawn two of it, so this is a `Vec` with
    /// repeats rather than a set - `2100007` lists `700005` twice and is the only sack in
    /// this client that does. Collapsing it to a set would halve that summon silently.
    pub mobs: Vec<u32>,
}

/// Every summoning sack, from `tools/dump_summon_sacks.py`'s `summonsacks.txt`.
///
/// The `mobs` column is `templateId:prob` joined by `;`. **`prob` is deliberately discarded**:
/// it is 100 on all nine entries across all eight sacks, so nothing in this client exercises a
/// probability roll, and a server that stored it would be carrying a field it could never
/// demonstrate it read correctly.
pub fn load_summon_sacks(path: &std::path::Path) -> HashMap<u32, SummonSack> {
    let mut out = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else { return out };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // `itemId, slotMax, price, mobCount, mobs, name` - and the name may contain commas,
        // so the mobs column is taken by index rather than by splitting the whole line.
        let f: Vec<&str> = line.split(',').map(str::trim).collect();
        if f.len() < 5 {
            continue;
        }
        let Ok(item_id) = f[0].parse::<u32>() else { continue };
        let mobs: Vec<u32> = f[4]
            .split(';')
            .filter_map(|entry| entry.split(':').next()?.trim().parse::<u32>().ok())
            .collect();
        if mobs.is_empty() {
            continue;
        }
        out.insert(item_id, SummonSack { mobs });
    }
    out
}

/// One of the client's own scrolls, as `Item.wz`'s `0204.img` has it.
///
/// **Field names are the WZ's own**, exactly as [`EquipTemplate`]'s are, and for the same
/// reason: `tools/dump_scrolls.py` enumerated every scalar `info` property across all 208
/// scroll nodes, and the set it found is what is here. There is no `incPAD` among them.
///
/// [`ScrollTemplate::category`] is the one derived thing, and its derivation is checked
/// against the client's own names rather than asserted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ScrollTemplate {
    /// Percent. Three values across all 208: 100, 60 and 10.
    ///
    /// **The Treasure Scroll ignores this** - it is a guarantee - but it is loaded because a
    /// menu that does not say what the scroll normally is would be hiding the whole point of
    /// the item.
    pub success: u16,
    /// Percent chance of destroying the item on failure. 156 of 208 are 0.
    ///
    /// Also ignored by the Treasure Scroll: a guaranteed success never reaches the failure
    /// arm, so nothing can be destroyed. That is a consequence of the rule, not a special
    /// case, and `crate::scrolls` states it where it applies.
    pub cursed: u16,
    /// What the scroll adds on success, as the same 17-field set the wire uses.
    pub increments: net::opcode::EquipStatSet,
}

impl ScrollTemplate {
    /// Which equip category a scroll id is for, as the equip id's own leading three digits.
    ///
    /// ```text
    ///   2040000 Hat …          -> 100   1002xxx Hat
    ///   2040300 Earring …      -> 103   1032xxx Earring
    ///   2043200 One-Handed BW  -> 132   1322999 Wizet Secret Agent Suitcase
    ///   2044700 Claw …         -> 147   1472xxx Claw
    ///   2048000 Pet Equip …    -> 180   180xxxx Pet equip
    /// ```
    ///
    /// **This is derived, so it has a positive control.** Grouping all 208 scrolls by this
    /// value and printing one name per group gives 24 groups whose names are *Hat*, *Earring*,
    /// *Topwear*, *Overall Armor*, *Bottomwear*, *Shoes*, *Gloves*, *Shield*, *Cape*,
    /// *One-Handed Sword*, … *Claw*, *Pet Equip* - the client's own words, agreeing with the
    /// derived category in 24 of 24 cases. The test below re-runs that check against the real
    /// file when it is present.
    ///
    /// It was also checked against a case that would have caught an id-prefix rule going
    /// wrong: the owner's suitcase is `1322999`, and its tooltip says *One-Handed Blunt Weapon*.
    /// `1322999 / 10000 = 132`, and `2043200`'s name is *One-Handed Blunt Weapon Attack
    /// Scroll*. The screen and the derivation agree.
    pub fn category(scroll_item_id: u32) -> u32 {
        100 + (scroll_item_id % 10_000) / 100
    }

    /// The equip's own category, the other half of the same comparison.
    pub fn equip_category(equip_item_id: u32) -> u32 {
        equip_item_id / 10_000
    }

    /// Can this scroll be used on that equip?
    pub fn fits(scroll_item_id: u32, equip_item_id: u32) -> bool {
        Self::category(scroll_item_id) == Self::equip_category(equip_item_id)
    }
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
    /// `info/cash`: a cash equip, which lives in the **Deco** tab (6), not the Equip tab.
    ///
    /// The client decides the tab itself: `FUN_1403E8AF0` returns 6 for a `1xxxxxx` id whose
    /// ItemInfo `+0x18` is non-zero (`research/cash-shop-buy-done.md` 5.2.1, **[L]** for the
    /// branch; **[I]** that `+0x18` is `cash`, on the strength of every backported cash equip
    /// carrying `cash = 1` and no other candidate). Every request the client builds about
    /// the item names that tab, so the server has to agree or every later move misses.
    pub cash: bool,
    /// `info/reqLevel`. Item variance's range is this over ten (`crate::variance`).
    pub req_level: u16,
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
    /// [`Quest::start_items`] and [`Quest::complete_items`] with their rules: `prop` and
    /// `gender`. The hand-out reads THESE through [`choose_rewards`]; the pairs above are the
    /// whole list, kept for the readers that count it.
    pub start_rewards: Vec<RewardItem>,
    pub complete_rewards: Vec<RewardItem>,
    /// `Act.1.exp` - experience for turning it in. Quest 1001's is 2.
    pub complete_exp: u64,
    /// `Act.1.skill.<n>` - `(skillId, exp)`, what turning it in teaches.
    ///
    /// **18 rows in this client, all six crafting professions, three quests each.** The
    /// starter quest of each profession grants `(9200x000, 1)` and its two follow-ups
    /// `(.., 20)` and `(.., 50)`: the id is the profession, and `exp` is **mastery**, not
    /// skill points. `store::crafting` keeps the level and the mastery; `world::crafting`
    /// has the curve. Quest 80008's is `(92000000, 1)` - Silas Irons making a Blacksmith.
    ///
    /// A non-profession skill here would be a shape this server has never seen; it is
    /// carried rather than dropped so the handler can say so out loud.
    pub complete_skills: Vec<(u32, u32)>,
    /// The conversation, keyed by the `Say` path with the line index removed.
    ///
    /// `"0"` is the opening conversation and `"1"` the completion one; `"0.yes"`,
    /// `"0.no"`, `"1.stop.npc"` and so on are the branches. Each value is that node's
    /// numbered lines **in index order**, which is not the same as string order once a
    /// conversation reaches ten lines.
    pub say: HashMap<String, Vec<String>>,
    /// The WZ line indices behind each [`Quest::say`] node, in the same order. Most nodes
    /// are `0, 1, 2 ...`; a quiz's wrong-answer node is not - Rain's `1.stop.0` has lines
    /// `0`, `1` and `3` (the missing `2` is the right answer), and the index is the menu
    /// choice it answers. Without this the vector's positions would say `0, 1, 2`.
    pub say_indices: HashMap<String, Vec<usize>>,

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

    /// `Check.0.citizenshipTown` / `Check.0.citizenshipGrade` - **starting** this quest needs an
    /// ACTIVE citizenship of that town at that grade or higher. 86 quests, all of them
    /// `506001..506045` / `506101..506141`. The client refuses with reason `0x50`
    /// (`FUN_14070FE30`, `research/citizenship-2026-09-27.md` §5.1); the server refuses too.
    /// `(town, grade)`.
    pub citizenship_check: Option<(u8, u8)>,
    /// `Act.1.citizenshipContr` - what turning it in banks, and where.
    pub citizenship_contr: Option<CitizenshipContr>,
    /// `Act.1.money` - mesos for turning it in, at the Quest rate. 255 quests carry one, every
    /// value positive. **Nothing paid it until 2026-09-28** (the owner: *"make the server pay for
    /// quest mesos at the 10x rate too for all quests"*) - the quest window showed the amount
    /// and the purse never moved. `Session::pay_quest_mesos`. The one `Act.0.money` in this
    /// client (quest 10303, `-1000`, a cost to START) is not read.
    pub complete_money: u32,
}

/// `Act.1.citizenshipContr`: `town`, and either a flat `amount` (the weeklies, the story arcs)
/// or an `amountFormula` in `citizenshipGrade` (every daily: `"100 + ( ( citizenshipGrade - 1 )
/// x 50 )"`). **[L]** The client evaluates the same string to draw the reward
/// (`FUN_1402C96E0`); `world::citizenship::contribution_for` computes the number it draws.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CitizenshipContr {
    pub town: u8,
    pub amount: Option<u32>,
    pub formula: Option<String>,
}

/// One `Act.<state>.item.<n>` while it is still being read.
///
/// The id and the count are separate rows in `questlines.txt` and nothing promises which
/// arrives first, so both sides are optional until the file is exhausted.
/// One `Act.<state>.item.<n>` while its rows are being stitched together: `.id`, `.count`,
/// and the two the random-reward rule reads, `.prop` and `.gender`.
#[derive(Debug, Clone, Default)]
struct HalfItem {
    id: Option<u32>,
    count: Option<i32>,
    prop: u32,
    gender: Option<u8>,
}

/// One item a quest hands over or takes, with the rule that decides whether it does.
///
/// The owner, 2026-09-13: *"When I finished 'Please bring this letter to Lucas', Maria gave me one of
/// every single Headband item when it's suppose to be choose 1 randomly from the pool."* Quest
/// 1008's `Act.1.item.1..7` are seven headbands, each with **`prop 1`** - the WZ's mark for
/// "one of these, at random, weighted by prop". `Act.1.item.0` (the letter back, `count -1`)
/// carries no `prop` and is unconditional. 39 quests use the mark. **[L]** for the data;
/// the weighted-pick reading of `prop` is the one every server has used and the only one that
/// makes seven `prop 1` hats a reward rather than a wardrobe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardItem {
    pub id: u32,
    /// Positive gives, negative takes.
    pub count: i32,
    /// `0`: unconditional. `> 0`: one item is chosen from all the `prop > 0` items of the same
    /// state, each with weight `prop`.
    pub prop: u32,
    /// `Some(0)` male only, `Some(1)` female only, `None` (or 2) anyone.
    pub gender: Option<u8>,
}

impl RewardItem {
    /// Whether this character may receive it at all.
    pub fn fits_gender(&self, gender: u8) -> bool {
        !matches!(self.gender, Some(g) if g < 2 && g != gender)
    }
}

/// Which of `rewards` a turn-in actually hands over: every unconditional one, and ONE of the
/// `prop`-marked ones, drawn with weight `prop`, after the gender filter. `roll` is the
/// caller's random number; the same number always picks the same item.
pub fn choose_rewards(rewards: &[RewardItem], gender: u8, roll: u64) -> Vec<RewardItem> {
    let eligible: Vec<&RewardItem> = rewards.iter().filter(|r| r.fits_gender(gender)).collect();
    let mut out: Vec<RewardItem> = eligible.iter().filter(|r| r.prop == 0).map(|r| (*r).clone()).collect();
    let pool: Vec<&RewardItem> = eligible.iter().copied().filter(|r| r.prop > 0).collect();
    let total: u64 = pool.iter().map(|r| u64::from(r.prop)).sum();
    if total > 0 {
        let mut pick = roll % total;
        for r in pool {
            if pick < u64::from(r.prop) {
                out.push(r.clone());
                break;
            }
            pick -= u64::from(r.prop);
        }
    }
    out
}

/// The `<n>` out of `<state>.item.<n>.id`.
fn act_item_index(dotted: &str) -> Option<usize> {
    dotted.split('.').nth(2)?.parse().ok()
}

/// The `<n>` out of `1.skill.<n>.id` / `1.skill.<n>.exp`, and `None` for anything else.
///
/// **State 1 only.** Every one of this client's 18 skill acts is a turn-in, and a state this
/// server does not model is ignored rather than guessed at - the same rule [`act_state`]
/// applies to items.
fn act_skill_index(dotted: &str) -> Option<usize> {
    let mut parts = dotted.split('.');
    if parts.next()? != "1" || parts.next()? != "skill" {
        return None;
    }
    let n: usize = parts.next()?.parse().ok()?;
    matches!(parts.next()?, "id" | "exp").then_some(n)
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
    // quest -> skill index -> a half-built (skillId, mastery exp). `Act.1.skill.<n>`.
    let mut act_skills: HashMap<u32, BTreeMap<usize, HalfItem>> = HashMap::new();
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
                    act_items.entry((qid, st)).or_default().entry(n).or_default().id = Some(id);
                }
            }
            "Act" if dotted.ends_with(".count") && act_state(dotted).is_some() => {
                if let (Some(st), Some(n), Some(c)) =
                    (act_state(dotted), act_item_index(dotted), value.parse::<i32>().ok())
                {
                    act_items.entry((qid, st)).or_default().entry(n).or_default().count = Some(c);
                }
            }
            "Act" if dotted.ends_with(".prop") && act_state(dotted).is_some() => {
                if let (Some(st), Some(n), Some(p)) =
                    (act_state(dotted), act_item_index(dotted), value.parse::<i64>().ok())
                {
                    act_items.entry((qid, st)).or_default().entry(n).or_default().prop = u32::try_from(p.max(0)).unwrap_or(0);
                }
            }
            "Act" if dotted.ends_with(".gender") && act_state(dotted).is_some() => {
                if let (Some(st), Some(n), Some(g)) =
                    (act_state(dotted), act_item_index(dotted), value.parse::<u8>().ok())
                {
                    act_items.entry((qid, st)).or_default().entry(n).or_default().gender = Some(g);
                }
            }
            // Citizenship - `Check.0.citizenshipTown/Grade` and `Act.1.citizenshipContr.*`.
            // WZ keys, so an overlay never touches them.
            "Check" if fill && dotted == "0.citizenshipTown" => {
                if let Ok(town) = value.parse::<u8>() {
                    let grade = quest.citizenship_check.map_or(1, |(_, g)| g);
                    quest.citizenship_check = Some((town, grade));
                }
            }
            "Check" if fill && dotted == "0.citizenshipGrade" => {
                if let Ok(grade) = value.parse::<u8>() {
                    let town = quest.citizenship_check.map_or(0, |(t, _)| t);
                    quest.citizenship_check = Some((town, grade));
                }
            }
            "Act" if fill && dotted == "1.money" => {
                quest.complete_money = value.parse().unwrap_or(0);
            }
            "Act" if fill && dotted.starts_with("1.citizenshipContr.") => {
                let c = quest.citizenship_contr.get_or_insert_with(CitizenshipContr::default);
                match &dotted["1.citizenshipContr.".len()..] {
                    "town" => c.town = value.parse().unwrap_or(0),
                    "amount" => c.amount = value.parse().ok(),
                    "amountFormula" => c.formula = Some(value.to_string()),
                    _ => {}
                }
            }
            "Act" if dotted == "1.exp" && (fill || quest.complete_exp == 0) => {
                quest.complete_exp = value.parse().unwrap_or(0);
            }
            // `Act.1.skill.<n>.id` and `.exp`, stitched like the items below: the two halves
            // are separate rows and the file does not promise an order.
            "Act" if act_skill_index(dotted).is_some() => {
                if let Some(n) = act_skill_index(dotted) {
                    let half = act_skills.entry(qid).or_default().entry(n).or_default();
                    if dotted.ends_with(".id") {
                        half.id = value.parse().ok();
                    } else if dotted.ends_with(".exp") {
                        half.count = value.parse().ok();
                    }
                }
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
        for (_, half) in indexed {
            if let (Some(id), Some(count)) = (half.id, half.count) {
                let reward = RewardItem { id, count, prop: half.prop, gender: half.gender };
                match state {
                    0 => {
                        quest.start_items.push((id, count));
                        quest.start_rewards.push(reward);
                    }
                    _ => {
                        quest.complete_items.push((id, count));
                        quest.complete_rewards.push(reward);
                    }
                }
            }
        }
    }

    // The skill acts, stitched the same way and with the same overlay rule: an overlay only
    // fills a hole, so a client-shipped grant is never replaced by an authored one.
    for (qid, indexed) in act_skills {
        let quest = out.entry(qid).or_default();
        if !(mode == Overlay::No || quest.complete_skills.is_empty()) {
            continue;
        }
        for (_, half) in indexed {
            // `exp` absent is 0 mastery, not "skip the grant": a quest could teach a
            // profession without paying any mastery, and the id is the part that matters.
            if let Some(id) = half.id {
                quest.complete_skills.push((id, half.count.unwrap_or(0).max(0) as u32));
            }
        }
    }

    for (qid, nodes) in lines {
        let quest = out.entry(qid).or_default();
        for (key, indexed) in nodes {
            // BTreeMap keyed on the parsed index, so line 10 follows line 9 rather than
            // line 1 - which a string sort would get wrong and nothing would catch.
            let indices: Vec<usize> = indexed.keys().copied().collect();
            let node: Vec<String> = indexed.into_values().collect();
            match mode {
                Overlay::No => {
                    quest.say.insert(key.clone(), node);
                    quest.say_indices.insert(key, indices);
                }
                // A shipped node is never replaced. Quest 1002's `Say."0"` is `{}` in the
                // WZ, so there is nothing to lose - but 1002 is not the only script quest.
                Overlay::Yes => {
                    quest.say.entry(key.clone()).or_insert(node);
                    quest.say_indices.entry(key).or_insert(indices);
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
///
/// **A missing or unreadable file is an empty table, silently.** That is the behaviour every
/// other loader in this file has and it is kept - but it is also exactly the shape that makes
/// a naive reload report success after reading nothing, so anything that needs to tell
/// "empty file" from "no file" must call [`read_npc_strings`] instead.
pub fn load_npc_strings(path: &std::path::Path) -> HashMap<u32, NpcStrings> {
    read_npc_strings(path).unwrap_or_default()
}

/// [`load_npc_strings`], but a read error is returned rather than swallowed.
pub fn read_npc_strings(path: &std::path::Path) -> std::io::Result<HashMap<u32, NpcStrings>> {
    let text = std::fs::read_to_string(path)?;
    Ok(parse_npc_strings(&text))
}

fn parse_npc_strings(text: &str) -> HashMap<u32, NpcStrings> {
    let mut out: HashMap<u32, NpcStrings> = HashMap::new();
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

/// The NPC text every live session reads, and the one table in `Config` that can be
/// **replaced while the server is running**.
///
/// # Why a lock and not a new `Arc<Config>`
///
/// The owner, 2026-08-29: *"Restarting the server kicks all of the clients off, but ... I would
/// like to introduce a command to reload all of the NPC server side chats."*
///
/// `crate::server::serve` builds one `Config`, wraps it in an `Arc`, and gives every accepted
/// connection a clone of that `Arc` (`server.rs:210,258`). A session therefore holds a handle
/// to **one allocation**, taken when it connected. Swapping the server's `Arc` for a freshly
/// loaded one would change what the *next* connection sees and nothing at all for anybody
/// already playing - which is the entire feature. So the mutable part has to be inside the
/// allocation they already share.
///
/// # The read never holds the guard
///
/// [`Self::snapshot`] clones an `Arc` and drops the guard immediately, so no lock is held
/// across packet construction or a send. Dialogue opens at human speed; an `Arc` clone per
/// box is free next to the alternative, which is a reader holding a lock while a `Vec<Reply>`
/// is built.
///
/// # `base` is not swappable, and that is a guarantee rather than an oversight
///
/// `Config::shop_by_template` is **derived** from this table's NPC *names* at start-up
/// (`shops::resolve_npc_templates`) and is a plain `HashMap` behind no lock at all, read by
/// `session::shop`. A reload that changed a name would leave that join pointing at the wrong
/// shopkeeper, with nothing on screen to say so - a shop that simply does not open looks
/// exactly like an NPC that never had one.
///
/// Two things make that impossible rather than merely unlikely:
///
/// * `base` is private, set once at construction, and has no setter. The reload rebuilds
///   `live` from it; it never replaces it.
/// * the overlay parser accepts **`d<n>` rows only** and refuses `name` out loud
///   ([`parse_npc_dialogue_overlay`]).
///
/// Regenerating `gm-handbook/npcstrings.txt` therefore still needs a restart, and
/// [`NpcDialogueReload::summary`] says so.
///
/// # Three layers, and each one is doing a different job
///
/// `Arc<RwLock<Arc<HashMap<..>>>>` looks like one `Arc` too many and is not:
///
/// * the **outer `Arc`** is what makes `Clone` share rather than copy. A cloned [`Config`]
///   reloads with the original instead of quietly keeping the old lines
/// * the **`RwLock`** is the swap itself
/// * the **inner `Arc`** is what lets [`Self::snapshot`] hand back the whole table and drop
///   the guard in the same expression, so no lock is ever held while a packet is built
#[derive(Debug, Clone, Default)]
pub struct NpcStringTable {
    /// The generated table exactly as it was read at start-up. Never replaced.
    base: Arc<HashMap<u32, NpcStrings>>,
    /// `base` with the authored overlay applied. Swapped whole by a reload.
    live: Arc<RwLock<Arc<HashMap<u32, NpcStrings>>>>,
}

impl From<HashMap<u32, NpcStrings>> for NpcStringTable {
    fn from(base: HashMap<u32, NpcStrings>) -> Self {
        let base = Arc::new(base);
        NpcStringTable { live: Arc::new(RwLock::new(Arc::clone(&base))), base }
    }
}

impl NpcStringTable {
    /// The table as it stands, as a cheap handle. **The lock is released before this
    /// returns.**
    ///
    /// A poisoned lock is recovered from rather than propagated. A panic elsewhere must not
    /// turn every NPC mute for the rest of the process: the data behind the lock is a
    /// whole-value swap, so it is never observed half-written, and `CLAUDE.md`'s "always
    /// answer" points the same way.
    pub fn snapshot(&self) -> Arc<HashMap<u32, NpcStrings>> {
        match self.live.read() {
            Ok(g) => Arc::clone(&g),
            Err(poisoned) => Arc::clone(&poisoned.into_inner()),
        }
    }

    /// The generated base, before any overlay. Only the start-up shop join wants this.
    pub fn base(&self) -> Arc<HashMap<u32, NpcStrings>> {
        Arc::clone(&self.base)
    }

    /// What this NPC says when talked to - its `d0`, cloned out from under the lock.
    pub fn dialogue_line(&self, template: u32) -> Option<String> {
        self.snapshot().get(&template).and_then(|s| s.dialogue.first()).cloned()
    }

    /// How many `info/speak` lines this NPC has, which is all the idle-chatter tick needs:
    /// the client holds the text and only an index goes on the wire.
    pub fn info_lines(&self, template: u32) -> usize {
        self.snapshot().get(&template).map(|s| s.info.len()).unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.snapshot().len()
    }

    pub fn is_empty(&self) -> bool {
        self.snapshot().is_empty()
    }

    /// Build `base + overlay` into a **new** map and swap it in whole.
    ///
    /// **Never clear-then-fill.** A half-applied table is every NPC mute, and the failure
    /// would arrive between two packets rather than at start-up where it could be seen.
    fn apply_overlay(&self, overlay: &HashMap<u32, Vec<String>>) -> usize {
        let mut next = (*self.base).clone();
        for (template, lines) in overlay {
            next.entry(*template).or_default().dialogue = lines.clone();
        }
        let live = next.len();
        let next = Arc::new(next);
        match self.live.write() {
            Ok(mut g) => *g = next,
            Err(poisoned) => *poisoned.into_inner() = next,
        }
        live
    }
}

/// What one `!npcreload` did, in enough detail to tell it from one that did nothing.
///
/// **`load_npc_strings` returns an empty map on a read error**, so a naive wiring of this
/// command would report success after reading no file at all. Every count here is separate
/// for that reason, and [`Self::refusal`] is what a caller must check before saying "ok".
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NpcDialogueReload {
    /// The generated file the base table came from. Named, not re-read.
    pub base_path: PathBuf,
    /// The authored file this reload read.
    pub overlay_path: PathBuf,
    /// NPC templates in the generated base.
    pub base_templates: usize,
    /// Templates the overlay gave at least one usable `d<n>` line for.
    pub overlay_templates: usize,
    /// Of those, how many replaced dialogue the base already had.
    pub overridden: usize,
    /// And how many are templates the base had no dialogue for at all.
    pub added: usize,
    /// Non-comment overlay rows that were read and not used, each with its reason.
    pub refused: Vec<String>,
    /// Templates live after the swap. **Unchanged from before when [`Self::refusal`] is set.**
    pub live_templates: usize,
    /// Why nothing was swapped. `None` means the table really was replaced.
    pub refusal: Option<String>,
}

impl NpcDialogueReload {
    pub fn applied(&self) -> bool {
        self.refusal.is_none()
    }

    /// One line, for the GM acknowledgement and for the start-up banner.
    ///
    /// It says what happened rather than that something happened: the file, the counts, and
    /// on a refusal, that the old table is still the live one. A command that silently works
    /// and one that silently does nothing look identical on screen.
    ///
    /// **The refused rows are deliberately NOT in here** - [`Self::refusal_line`] carries
    /// them. This string is already about as long as `GM_COMMANDS`, which is the longest
    /// notice this client has been seen to draw; appending an unbounded list of parse errors
    /// to it would take it somewhere unproven, and the rows are more useful on a line of
    /// their own anyway.
    pub fn summary(&self) -> String {
        if let Some(why) = &self.refusal {
            return format!(
                "REFUSED and NOTHING changed - {} NPC templates still live: {why}",
                self.live_templates
            );
        }
        format!(
            "{} NPC templates live. {} from {} (generated; regenerating it still needs a \
             restart, because the shop join is built from its names at start-up). {} \
             overlaid from {}: {} replaced a line the NPC already had, {} added to an NPC \
             that had none. Every connection on THIS channel process sees it at once; \
             another channel is another process and needs its own reload.",
            self.live_templates,
            self.base_templates,
            clip_tail(&self.base_path.display().to_string()),
            self.overlay_templates,
            clip_tail(&self.overlay_path.display().to_string()),
            self.overridden,
            self.added,
        )
    }

    /// The refused rows as one bounded line, or `None` when every row was taken.
    ///
    /// Bounded on purpose: two rows, each clipped, then a count. The whole list goes to the
    /// server console, which has no length to worry about.
    pub fn refusal_line(&self) -> Option<String> {
        if self.refused.is_empty() {
            return None;
        }
        let shown: Vec<String> = self
            .refused
            .iter()
            .take(2)
            .map(|r| r.chars().take(110).collect::<String>())
            .collect();
        let more = self.refused.len().saturating_sub(shown.len());
        Some(format!(
            "{} overlay row(s) REFUSED and NOT applied: {}{}",
            self.refused.len(),
            shown.join(" | "),
            if more > 0 { format!(" | +{more} more, see the server console") } else { String::new() }
        ))
    }
}

/// The **tail** of a path, so a long `--npc-dialogue` cannot make the acknowledgement run
/// away.
///
/// The tail rather than the head, because the informative half of a path is its end. Nothing
/// is clipped at the shipped defaults; this is a bound, not a formatter. The server console
/// always gets the whole path - only the on-screen notice is bounded, and it is bounded
/// because `GM_COMMANDS` at 474 characters is the longest notice this client has been
/// **seen** to draw and anything past that is unproven.
fn clip_tail(path: &str) -> String {
    const KEEP: usize = 60;
    let n = path.chars().count();
    if n <= KEEP {
        return path.to_string();
    }
    format!("...{}", path.chars().skip(n - KEEP).collect::<String>())
}

/// One overlay file, parsed: `template -> the `d` lines in index order`, plus every refusal.
///
/// `rows` counts non-blank, non-comment lines, so "the file had rows and produced nothing"
/// is a state the caller can recognise - which is what a file edited with spaces instead of
/// tabs looks like, and it is the most likely way this file gets broken.
#[derive(Debug, Clone, Default)]
pub struct NpcDialogueOverlay {
    pub lines: HashMap<u32, Vec<String>>,
    pub refused: Vec<String>,
    pub rows: usize,
}

/// Parse `data/npc-dialogue.txt`: `templateId <TAB> d<n> <TAB> text`.
///
/// # Only `d<n>`, and the other three keys are refused out loud
///
/// The generated file carries four kinds of key and only one of them is text this server
/// puts on a wire:
///
/// | key | who holds the text | why the overlay refuses it |
/// |---|---|---|
/// | `d<n>` | **the server** | this is the one that works |
/// | `name` | nobody - it never goes on a wire | `shops::resolve_npc_templates` joins `data/shops.txt` onto template ids **by this name**, at start-up, into a table behind no lock. Renaming here would silently re-point a shop |
/// | `info<n>` | **the client** | `0x0453` carries an *index*, never the text. Authoring it here would change nothing on screen while changing how many indices the chatter tick cycles - and an index past what the client's own `info/speak` group holds has never been sent, so it is not established as safe |
/// | `idle<n>` | **the client** | same, and this group is not even the one a balloon can reach - see [`NpcStrings::info`] |
///
/// A refusal is counted and named. Silently ignoring a row is how an author concludes the
/// reload is broken.
///
/// # Order comes from the index, not from the file
///
/// `d10` sorts before `d2` as a string. `read_quest_rows` documents the same trap and it is
/// the reason this parses the suffix into a `BTreeMap<usize, _>` rather than pushing.
pub fn parse_npc_dialogue_overlay(text: &str) -> NpcDialogueOverlay {
    use std::collections::BTreeMap;
    let mut indexed: HashMap<u32, BTreeMap<usize, String>> = HashMap::new();
    let mut out = NpcDialogueOverlay::default();
    for (n, line) in text.lines().enumerate() {
        let n = n + 1;
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        out.rows += 1;
        let mut f = line.splitn(3, '\t');
        let (Some(id), Some(key), Some(value)) = (f.next(), f.next(), f.next()) else {
            out.refused.push(format!(
                "line {n}: not three TAB-separated fields (templateId, d<n>, text)"
            ));
            continue;
        };
        let Ok(template) = id.trim().parse::<u32>() else {
            out.refused.push(format!("line {n}: {:?} is not an NPC template id", id.trim()));
            continue;
        };
        let key = key.trim();
        let Some(index) = key.strip_prefix('d').and_then(|i| i.parse::<usize>().ok()) else {
            out.refused.push(format!(
                "line {n}: key {key:?} - this file carries spoken dialogue only, as d0, d1, \
                 ...; name/info/idle rows are refused because that text is not the server's \
                 to send"
            ));
            continue;
        };
        if value.trim().is_empty() {
            out.refused.push(format!(
                "line {n}: template {template} {key} is empty - an empty box is worse than \
                 the placeholder"
            ));
            continue;
        }
        if indexed.entry(template).or_default().insert(index, value.to_string()).is_some() {
            out.refused.push(format!(
                "line {n}: template {template} has two {key} rows - the later one won"
            ));
        }
    }
    out.lines = indexed
        .into_iter()
        .map(|(t, lines)| (t, lines.into_values().collect::<Vec<String>>()))
        .collect();
    out
}

/// Re-read the authored overlay and swap the live NPC dialogue table. **No restart.**
///
/// This is the whole of `!npcreload`, and it is also what the world server runs at start-up,
/// so the two cannot drift - the start-up banner and the in-game acknowledgement are the same
/// sentence from the same function.
///
/// # A parse failure leaves the running table exactly as it was
///
/// The new table is built beside the old one and swapped whole ([`NpcStringTable::apply_overlay`]).
/// Three states refuse the swap outright, and each says which:
///
/// * the overlay file cannot be read for a reason other than "it is not there" - a missing
///   file is legal and means "no overrides", exactly as `data/quest-scripts.txt` is
/// * the file has rows and **not one** of them parsed. That is what an edit saved with spaces
///   instead of tabs looks like, and applying it would quietly revert every override the
///   author already had working
/// * the generated base is empty, so there is nothing for an overlay to sit on and the
///   command would report success over a table that cannot say anything
pub fn reload_npc_dialogue(config: &Config) -> NpcDialogueReload {
    let base = config.npc_strings.base();
    let mut r = NpcDialogueReload {
        base_path: config.npc_strings_path.clone(),
        overlay_path: config.npc_dialogue_path.clone(),
        base_templates: base.len(),
        live_templates: config.npc_strings.len(),
        ..Default::default()
    };

    let text = match std::fs::read_to_string(&config.npc_dialogue_path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => {
            r.refusal =
                Some(format!("could not read {}: {e}", clip_tail(&config.npc_dialogue_path.display().to_string())));
            return r;
        }
    };

    if base.is_empty() {
        r.refusal = Some(format!(
            "the generated base {} holds no NPC text at all, so there is nothing for an \
             overlay to amend. Regenerate it with: python tools/dump_npcstrings.py, then \
             restart",
            clip_tail(&config.npc_strings_path.display().to_string())
        ));
        return r;
    }

    let overlay = parse_npc_dialogue_overlay(&text);
    r.refused = overlay.refused;
    if overlay.rows > 0 && overlay.lines.is_empty() {
        r.refusal = Some(format!(
            "{} has {} row(s) and not one is a usable \"<templateId> TAB d<n> TAB text\" row. \
             The separator is a TAB, not spaces",
            clip_tail(&config.npc_dialogue_path.display().to_string()),
            overlay.rows
        ));
        return r;
    }

    r.overlay_templates = overlay.lines.len();
    for template in overlay.lines.keys() {
        // "Overrode a base entry" means the NPC already had something to say. An entry that
        // exists with an empty `dialogue` is an NPC that fell through to the placeholder, and
        // counting it as an override would make a new line look like a replaced one.
        let had = base.get(template).is_some_and(|s| !s.dialogue.is_empty());
        if had {
            r.overridden += 1;
        } else {
            r.added += 1;
        }
    }
    r.live_templates = config.npc_strings.apply_overlay(&overlay.lines);
    r
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
    /// `PDDamage`, `MADamage`, `MDDamage`, `acc` and `eva` - columns 7, 8, 9, 10 and 11.
    ///
    /// Carried only so that a **forced stat** block can be built from the mob's own WZ row.
    /// `net::mobdamage::MobForcedStat` overrides every stat at once, so sending it with the
    /// real values for everything except the one being changed is what keeps the mob behaving
    /// as the client's own data says it should. Zeroing them would make every mob defenceless
    /// and unable to miss, which is a much bigger change than the one intended.
    pub pd_damage: u32,
    pub ma_damage: u32,
    pub md_damage: u32,
    pub accuracy: u32,
    pub evasion: u32,
    /// `summonType`, column 17: which `Effect/Summon.img` entry plays when the mob is
    /// SUMMONED (a sack, a mob skill). `0` for the Balrogs, `1` for 180 of 193. Not used
    /// by ordinary spawns, which arrive with no effect (`net::mob::APPEAR_SPAWNING`).
    pub summon_type: u32,
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
                // Optional for the same reason `pa_damage` is: a short row should lose a
                // column, not the whole mob.
                pd_damage: n(6).unwrap_or(0).max(0) as u32,
                ma_damage: n(7).unwrap_or(0).max(0) as u32,
                md_damage: n(8).unwrap_or(0).max(0) as u32,
                accuracy: n(9).unwrap_or(0).max(0) as u32,
                evasion: n(10).unwrap_or(0).max(0) as u32,
                summon_type: n(16).unwrap_or(0).max(0) as u32,
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
            answer_packets: true,
            peer_policy: store::migration::PeerPolicy::Require,
            inventory_slots: None,
            pet_move_action: None,
            broadcast_pets: true,
            look_change_reenter: false,
            charinfo_look_items: true,
            chairs: HashMap::new(),
            recipes: crate::crafting::Recipes::new(),
            portals: HashMap::new(),
            portal_index: HashMap::new(),
            portal_positions: HashMap::new(),
            spawn_points: HashMap::new(),
            npcs: HashMap::new(),
            mobs: HashMap::new(),
            mob_respawn_s: HashMap::new(),
            reactors: HashMap::new(),
            reactor_drops: crate::droptables::DropTables::default(),
            mob_limit: None,
            shop_rows: None,
            drops: crate::droptables::DropTables::default(),
            exp_curve: crate::expcurve::ExpCurve::default(),
            mob_exp: HashMap::new(),
            mob_attack: HashMap::new(),
            mob_templates: HashMap::new(),
            mob_skills: crate::mobskills::MobSkillTable::default(),
            quest_reqs: net::quest::QuestRequirementTable::default(),
            quest_items: crate::questitems::QuestItems::default(),
            chatter_off: false,
            equips: HashMap::new(),
            scrolls: HashMap::new(),
            summon_sacks: HashMap::new(),
            npc_strings: NpcStringTable::default(),
            pet_commands: crate::petcommands::PetCommands::default(),
            npc_strings_path: PathBuf::from("gm-handbook/npcstrings.txt"),
            npc_dialogue_path: PathBuf::from("data/npc-dialogue.txt"),
            quests: HashMap::new(),
            shops: crate::shops::ShopTable::default(),
            shop_by_template: HashMap::new(),
            channels: Vec::new(),
            link: None,
            advertise: std::sync::Arc::new(net::advertise::Advertiser::default()),
            map_names: HashMap::new(),
            item_names: HashMap::new(),
            hair_ids: HashSet::new(),
            face_ids: HashSet::new(),
            send_mobs: true,
            fields: std::collections::HashSet::new(),
            clocks: std::collections::HashSet::new(),
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
    /// `life` entries run left to right, so any rule that takes a prefix of WZ order puts
    /// every mob at the low-x end of the map. That was true of the original `take(n)` and it
    /// stayed true inside each group of the type-quota version; the uniform draw that
    /// replaced both is the first form where it is structurally impossible.
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
            let chosen = choose_spawns(&mobs, cap, seed);
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
            choose_spawns(&mobs, 30, seed).iter().map(|m| m.object_id).collect()
        };
        assert_ne!(ids(1), ids(2));
        assert_eq!(ids(7), ids(7), "and the same seed reproduces exactly, or nothing is testable");
    }

    /// **The per-type count is no longer fixed, and that is the rule change.**
    ///
    /// This replaces `shuffling_positions_does_not_disturb_the_per_type_quota`, which pinned
    /// the counts to exactly 7/12/5/5/4 for every seed. That assertion was the largest-
    /// remainder rule written down, so it had to go when the rule did - but it is replaced by
    /// a *stronger* claim rather than dropped: the counts must actually vary, and the old
    /// fixed split must not be what every seed returns.
    ///
    /// It fails on the old implementation immediately and non-probabilistically: that one
    /// returns the identical BTreeMap for every seed, so `distinct == 1` on all five types.
    #[test]
    fn the_per_type_count_is_no_longer_pinned() {
        let mobs = field(&[(1, 10), (2, 16), (3, 7), (4, 6), (5, 6)]);
        let cap = spawn_capacity(45, 1);
        let old_fixed: std::collections::BTreeMap<u32, usize> =
            [(1, 7), (2, 12), (3, 5), (4, 5), (5, 4)].into_iter().collect();

        let mut distinct: std::collections::BTreeMap<u32, std::collections::BTreeSet<usize>> =
            std::collections::BTreeMap::new();
        let mut matched_old = 0usize;
        let seeds = 200u64;
        for seed in 0..seeds {
            let c = counts(&choose_spawns(&mobs, cap, seed));
            if c == old_fixed {
                matched_old += 1;
            }
            for (template, n) in c {
                distinct.entry(template).or_default().insert(n);
            }
        }

        for (template, seen) in &distinct {
            assert!(
                seen.len() > 1,
                "template {template} took the same count {seen:?} on all {seeds} seeds - \
                 the draw is still enforcing a quota"
            );
        }
        // The old split is still a *possible* draw - it is close to the expectation, so it
        // had better be - but it must not be the only one. Measured at 1.2% of 4000 seeds.
        assert!(
            matched_old < seeds as usize / 2,
            "the old fixed 7/12/5/5/4 came back on {matched_old} of {seeds} seeds"
        );
    }

    /// Map 40, "Snail Hunting Ground I": 40 spawn points, one type, 30 alive for a solo
    /// player. The single datapoint the capacity rule has.
    #[test]
    fn map_40_keeps_thirty_of_its_forty_spawn_points() {
        assert_eq!(spawn_capacity(40, 1), 30);
        let mobs = field(&[(2, 40)]);
        let chosen = choose_spawns(&mobs, spawn_capacity(mobs.len(), 1), TEST_SEED);
        assert_eq!(chosen.len(), 30);
        assert_eq!(counts(&chosen), [(2, 30)].into_iter().collect());
    }

    /// Map 1006, "Hunting Ground Middle of the Forest II": 45 spawns across five types,
    /// cap 33. Blue Snail 16, Snail 10, Shroom 7, Orange Mushroom 6, Red Snail 6.
    ///
    /// **The ratio is preserved in expectation, not enforced per draw.** This replaces
    /// `a_mixed_map_keeps_each_types_share_rather_than_the_first_n`, which asserted the exact
    /// largest-remainder split 7/12/5/5/4 and then that every type was `< 1.0` slot from its
    /// exact proportion. Both assertions state the deleted rule: a single uniform draw is
    /// routinely more than one slot out - Blue Snail's SD alone is 1.44 - so the second one
    /// is false now, not merely unenforced.
    ///
    /// What replaces them is a claim the old code **fails**, which is the point of writing it
    /// this way rather than loosening the tolerance. Averaged over 4000 draws each type's
    /// count must land within 0.15 of `cap * points / total`. That band is 6 standard errors
    /// (`SD/sqrt(4000) <= 0.0227`), so it is a real bound and not a wide one; the
    /// implementation's worst observed error is **0.029**. The old fixed split misses it on
    /// three of the five types - Red Snail is 5 against an exact 4.4, out by **0.60**.
    #[test]
    fn each_types_expected_share_is_its_share_of_the_map() {
        let mobs = field(&[(1, 10), (2, 16), (3, 7), (4, 6), (5, 6)]);
        assert_eq!(mobs.len(), 45);
        let cap = spawn_capacity(45, 1);
        assert_eq!(cap, 33);

        const DRAWS: u64 = 4000;
        // Six standard errors, from SD <= 1.44 over DRAWS draws. Derived, not tuned.
        const TOLERANCE: f64 = 0.15;

        let mut total: std::collections::BTreeMap<u32, usize> = std::collections::BTreeMap::new();
        for seed in 0..DRAWS {
            let chosen = choose_spawns(&mobs, cap, seed);
            assert_eq!(chosen.len(), cap, "seed {seed}: the cap must be filled exactly");
            for (template, n) in counts(&chosen) {
                *total.entry(template).or_default() += n;
            }
        }

        for (template, points) in [(1u32, 10usize), (2, 16), (3, 7), (4, 6), (5, 6)] {
            let exact = points as f64 * cap as f64 / 45.0;
            let mean = total[&template] as f64 / DRAWS as f64;
            assert!(
                (mean - exact).abs() < TOLERANCE,
                "template {template}: mean {mean:.4} against an exact share of {exact:.4}"
            );
        }

        // Two types with the SAME share must get the same treatment. This is the assertion
        // the old rule could never pass: templates 4 and 5 both hold 6 of 45 points, and
        // largest-remainder gave them 5 and 4 on every single draw, forever, because the tie
        // broke on template id.
        let four = total[&4] as f64 / DRAWS as f64;
        let five = total[&5] as f64 / DRAWS as f64;
        assert!(
            (four - five).abs() < TOLERANCE,
            "equal shares drew unequally: template 4 {four:.4}, template 5 {five:.4}"
        );

        // What the naive prefix does, kept as the thing still being ruled out.
        let naive: Vec<u32> = mobs.iter().take(cap).map(|m| m.template_id).collect();
        assert!(
            !naive.contains(&5),
            "the first 33 in WZ order miss a whole type - that is the bug"
        );
    }

    /// **The drift the owner is told to expect is the drift the code produces.**
    ///
    /// The doc table on [`choose_spawns`] quotes a standard deviation per type, and those
    /// numbers came from a closed form - `Var = cap * p * (1-p) * (total-cap)/(total-1)`,
    /// the hypergeometric with its finite-population correction. A number that came from
    /// doing algebra is a claim, exactly like one read off a header, so it is asserted here
    /// against something that can disagree with it.
    ///
    /// This is also the test that would catch the draw not being uniform. A grouped or
    /// weighted selection can still hit the right *mean* while collapsing the variance -
    /// the old rule had SD exactly 0 - so the mean test above is not sufficient on its own.
    #[test]
    fn the_spread_around_that_share_matches_the_hypergeometric() {
        let mobs = field(&[(1, 10), (2, 16), (3, 7), (4, 6), (5, 6)]);
        let total_points = 45.0f64;
        let cap = spawn_capacity(45, 1);

        const DRAWS: u64 = 4000;
        let mut samples: std::collections::BTreeMap<u32, Vec<f64>> =
            std::collections::BTreeMap::new();
        for seed in 0..DRAWS {
            for (template, n) in counts(&choose_spawns(&mobs, cap, seed)) {
                samples.entry(template).or_default().push(n as f64);
            }
        }

        for (template, points) in [(1u32, 10usize), (2, 16), (3, 7), (4, 6), (5, 6)] {
            let p = points as f64 / total_points;
            let want = (cap as f64 * p * (1.0 - p) * (total_points - cap as f64)
                / (total_points - 1.0))
                .sqrt();

            let xs = &samples[&template];
            let mean = xs.iter().sum::<f64>() / xs.len() as f64;
            let got = (xs.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>()
                / xs.len() as f64)
                .sqrt();

            // 10% of the predicted SD. The measured worst case is 1.4%.
            assert!(
                (got / want - 1.0).abs() < 0.10,
                "template {template}: SD {got:.3} against a predicted {want:.3}"
            );
        }
    }

    /// Spawn order is preserved, so the wire order does not depend on how the grouping ran.
    #[test]
    fn the_chosen_mobs_come_back_in_spawn_order() {
        let mobs = field(&[(1, 4), (2, 4)]);
        let chosen = choose_spawns(&mobs, 6, TEST_SEED);
        let ids: Vec<u32> = chosen.iter().map(|m| m.object_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
    }

    /// The edges, because these come off generated data and a panic here takes the server
    /// down on a field entry.
    #[test]
    fn the_edges_do_not_panic_or_overshoot() {
        assert!(choose_spawns(&[], 10, TEST_SEED).is_empty());
        assert!(choose_spawns(&field(&[(1, 5)]), 0, TEST_SEED).is_empty());

        // A cap at or above the total keeps everything, and never more.
        let mobs = field(&[(1, 3), (2, 2)]);
        assert_eq!(choose_spawns(&mobs, 5, TEST_SEED).len(), 5);
        assert_eq!(choose_spawns(&mobs, 99, TEST_SEED).len(), 5);

        // Every cap from 0 to total is filled exactly, on a ragged mix.
        let ragged = field(&[(7, 1), (3, 13), (9, 4), (1, 2)]);
        for cap in 0..=ragged.len() {
            assert_eq!(choose_spawns(&ragged, cap, TEST_SEED).len(), cap, "cap {cap}");
        }
    }

    /// **A cap of one goes to whichever point was drawn, not to the largest type.**
    ///
    /// Split out of `the_edges_do_not_panic_or_overshoot`, which asserted
    /// `one[0].template_id == 1, "the largest share takes the only slot"`. That was the
    /// apportionment showing through at the smallest possible cap: with 3 points of template
    /// 1 and 2 of template 2, largest-remainder always handed the single slot to template 1.
    ///
    /// Under a uniform draw the slot goes to a point, and the point's type follows from where
    /// it landed - so template 2 must sometimes win, at about its 2/5 share. Asserting only
    /// "one of the two" would have been the weaker rewrite; this pins the frequency, which is
    /// what actually distinguishes a uniform draw from a biased one. Measured 1210/790 over
    /// 2000 seeds against an expected 1200/800.
    #[test]
    fn a_cap_of_one_goes_to_a_random_point_not_the_largest_type() {
        let mobs = field(&[(1, 3), (2, 2)]);
        const DRAWS: u64 = 2000;

        let mut wins: std::collections::BTreeMap<u32, usize> = std::collections::BTreeMap::new();
        for seed in 0..DRAWS {
            let one = choose_spawns(&mobs, 1, seed);
            assert_eq!(one.len(), 1, "seed {seed}");
            *wins.entry(one[0].template_id).or_default() += 1;
        }

        assert_eq!(wins.len(), 2, "one type took every single slot: {wins:?}");
        // 3/5 and 2/5 of the draws, within 5 points. The old rule gives 2000 / 0.
        for (template, share) in [(1u32, 0.6f64), (2, 0.4)] {
            let got = wins[&template] as f64 / DRAWS as f64;
            assert!(
                (got - share).abs() < 0.05,
                "template {template} took {got:.3} of the slots, expected {share:.3}"
            );
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

    /// The real scroll table, and the category derivation **against the client's own names**.
    ///
    /// The derivation is the risky part: `100 + (id % 10000) / 100` is an arithmetic rule
    /// asserted about someone else's id scheme, and this repo's standing rule is that a rule
    /// like that needs something that can disagree with it. The names can. Every scroll whose
    /// name begins with a category word is checked against the category the arithmetic gives,
    /// so a scheme that is not what I think it is fails here rather than in a player's
    /// inventory.
    #[test]
    fn every_real_scroll_loads_and_its_category_agrees_with_its_own_name() {
        let path = std::path::Path::new("../../gm-handbook/scrolls.txt");
        if !path.exists() {
            return; // generated data, gitignored - tools/dump_scrolls.py makes it
        }
        let scrolls = Config::load_scrolls(path);
        assert!(scrolls.len() > 200, "only {} scrolls loaded", scrolls.len());

        // A row that reads at all: the Lesser Hat Accuracy Scroll is 100% and +1 Accuracy.
        let hat = scrolls[&2_040_000];
        assert_eq!(hat.success, 100);
        assert_eq!(hat.cursed, 0, "156 of 208 never destroy the item");
        assert_eq!(hat.increments.inc_acc, 1, "incACC is column 13, not wherever equips.txt has it");
        assert_eq!(hat.increments.inc_pad, 0, "no scroll in this client carries incPAD");

        // The column-order trap, stated as an assertion: a weapon attack scroll must put its
        // number in `inc_wat`, and must NOT have moved it into `inc_mad` or `inc_speed` by
        // reusing the equip loader's indices.
        let sword = scrolls[&2_043_000];
        assert!(sword.increments.inc_wat > 0, "a sword attack scroll grants incWAT");
        assert_eq!(sword.increments.inc_mad, 0);
        assert_eq!(sword.increments.inc_speed, 0);

        // The control on the derivation. `(word in the name, category the id must give)`,
        // taken from the client's own scroll names.
        let expected: &[(&str, u32)] = &[
            ("Hat ", 100),
            ("Earring ", 103),
            ("Topwear ", 104),
            ("Overall Armor ", 105),
            ("Bottomwear ", 106),
            ("Shoes ", 107),
            ("Gloves ", 108),
            ("Shield ", 109),
            ("Cape ", 110),
            ("One-Handed Sword ", 130),
            ("One-Handed Axe ", 131),
            ("One-Handed Blunt Weapon ", 132),
            ("Dagger ", 133),
            ("Wand ", 137),
            ("Staff ", 138),
            ("Two-handed Sword ", 140),
            ("Two-handed Axe ", 141),
            ("Two-handed Blunt Weapon ", 142),
            ("Spear ", 143),
            ("Polearm ", 144),
            ("Bow ", 145),
            ("Crossbow ", 146),
            ("Claw ", 147),
            ("Pet Equip ", 180),
        ];
        let text = std::fs::read_to_string(path).unwrap();
        let mut checked = 0;
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((head, name)) = line.rsplit_once(',') else { continue };
            let Some(id) = head.split(',').next().and_then(|s| s.trim().parse::<u32>().ok())
            else {
                continue;
            };
            let name = name.trim();
            for (word, category) in expected {
                if name.starts_with(word) {
                    assert_eq!(
                        ScrollTemplate::category(id),
                        *category,
                        "{id} is named {name:?}, so its category must be {category}"
                    );
                    checked += 1;
                    break;
                }
            }
        }
        assert!(checked > 190, "only {checked} of ~208 names matched a category word");

        // And the derivation discriminates, rather than agreeing with everything: a hat
        // scroll must NOT fit the owner's one-handed blunt weapon.
        assert!(ScrollTemplate::fits(2_043_200, 1_322_999), "1322999 is a One-Handed BW");
        assert!(!ScrollTemplate::fits(2_040_000, 1_322_999), "a hat scroll is not for a weapon");
        assert!(!ScrollTemplate::fits(2_043_000, 1_322_999), "nor a One-Handed SWORD scroll");
    }

    /// **The eight sacks load, and the one that summons TWO really carries two.**
    ///
    /// `2100007` lists `700005` twice in the client's own data, and it is the only sack that
    /// does - so it is the single row that can tell a `Vec` apart from a set. A loader that
    /// deduplicated would halve that summon and pass on all seven others.
    #[test]
    fn the_summoning_sacks_load_and_the_double_one_is_not_collapsed() {
        let path = std::path::Path::new("../../gm-handbook/summonsacks.txt");
        if !path.exists() {
            return; // generated data, gitignored - tools/dump_summon_sacks.py makes it
        }
        let sacks = load_summon_sacks(path);
        assert_eq!(sacks.len(), 8, "this client has eight sacks: {sacks:?}");
        // The owner's: GM Black Sack: Jr. Balrog Level 80.
        assert_eq!(sacks[&2_100_006].mobs, vec![800_023]);
        assert_eq!(sacks[&2_100_000].mobs, vec![700_004], "the plain Black Sack");
        assert_eq!(
            sacks[&2_100_007].mobs,
            vec![700_005, 700_005],
            "listed twice means summon two"
        );
        // Every template a sack names must exist, or the summon has no HP to give it.
        let templates = load_mob_templates(std::path::Path::new("../../gm-handbook/mobtemplates.txt"));
        if !templates.is_empty() {
            for (item, sack) in &sacks {
                for t in &sack.mobs {
                    assert!(templates.contains_key(t), "sack {item} names unknown mob {t}");
                }
            }
        }
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
        assert_eq!(fields.len(), 27, "19 numeric + 6 requirement + cash + name: {row}");
        assert_eq!(fields[25], "0", "the Sword is not a cash equip: {row}");
        assert_eq!(fields[26], "Sword", "the name is the last column: {row}");
        // The cash column, and what it decides: a backported Signature Style equip is a cash
        // equip and belongs in the Deco tab; the starter sword in the Equip tab.
        let uebel = text.lines().find(|l| l.starts_with("1054562,")).expect("Ubel's Clothes is in the hybrid file");
        let f: Vec<&str> = uebel.split(',').map(str::trim).collect();
        assert_eq!(f[25], "1", "cash: {uebel}");
        let cfg = Config { equips: Config::load_equips(path), ..Config::default() };
        assert_eq!(cfg.tab_for(1054562), Some(store::InventoryType::Deco));
        assert_eq!(cfg.tab_for(1302000), Some(store::InventoryType::Equip));
        assert_eq!(cfg.tab_for(5150000), Some(store::InventoryType::Cash));
        assert_eq!(cfg.tab_for(9999999), None);
        // reqLevel is field 19 and the starter sword needs nothing at all - it is the one
        // weapon in this client with a completely free entry, which is why `!kit` uses it.
        assert_eq!(fields[19], "0", "the Sword has no level requirement: {row}");
        // And the loader reads that column: item variance's range is reqLevel / 10.
        assert_eq!(equips[&1302000].req_level, 0);
        assert_eq!(equips[&1072128].req_level, 28, "Squishy Shoes, the King Slime's drop");
        assert_eq!(equips[&1050000].req_level, 15, "Beige Plain Robe");
    }

    /// **Spawn points are the `sp` portals, and nothing else** - checked on Kerning City's real
    /// row set, which has all the lookalikes: 15 `sp`, six `tp` Mystic Door points, `pc00` and
    /// `cab00` (target 0, no script, not spawn points) and doors. Then the two picks: the
    /// nearest to where someone stood, and a random one for a teleport.
    #[test]
    fn spawn_points_are_the_sp_portals_and_the_picks_use_them() {
        let path = std::path::Path::new("../../gm-handbook/portals.txt");
        if !path.exists() {
            return; // generated data, gitignored
        }
        let (portals, portal_index, portal_positions) = Config::load_portals_with_positions(path);
        let spawn_points = Config::load_spawn_points(path);
        let cfg = Config { portals, portal_index, portal_positions, spawn_points, ..Config::default() };
        let kerning = &cfg.spawn_points[&crate::firsttime::TOWN_MAP];
        assert_eq!(kerning.len(), 15, "Kerning City's fifteen `sp`: {kerning:?}");
        for not_a_spawn in [24u8, 33, 34, 40] {
            assert!(!kerning.contains(&not_a_spawn), "in01 / pc00 / cab00 / tp are not spawn points: {not_a_spawn}");
        }
        assert_eq!(cfg.spawn_points.len(), 426, "every map in the file has at least one");
        // Standing at the in04 door (-761, 115): sp 12 at (-1051, 107) is 290 px away, sp 3 at
        // (-19, 88) is 742.
        assert_eq!(cfg.nearest_spawn_point(crate::firsttime::TOWN_MAP, (-761, 115)), Some(12));
        assert_eq!(cfg.nearest_spawn_point(crate::firsttime::TOWN_MAP, (2102, -286)), Some(5), "on a spawn point: that one");
        assert_eq!(cfg.nearest_spawn_point(999_999_999, (0, 0)), None);
        // A teleport: always a spawn point, and not always the same one.
        let picks: std::collections::BTreeSet<u8> =
            (0..200u64).map(|r| cfg.random_spawn_point(crate::firsttime::TOWN_MAP, r.wrapping_mul(0x9E37_79B9_7F4A_7C15))).collect();
        assert!(picks.iter().all(|p| kerning.contains(p)), "{picks:?}");
        assert!(picks.len() > 5, "a teleport is spread over the town: {picks:?}");
        assert_eq!(cfg.random_spawn_point(999_999_999, 7), 0, "a map with none: the default");
        // The Magician Job Instructor's `job00` is NOT a random landing spot - it is used by name.
        assert!(!cfg.spawn_points[&10002070].contains(&32));
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

    /// **Ellinia Station declares a clock, and every clock map is a real field.** The first
    /// half is the owner's screenshot; the second is the property the gate relies on - a map in
    /// `clocks.txt` that is not in `fields.txt` would be one the server could never send a
    /// player to anyway, but it would mean the two dumps disagree about what a map is.
    #[test]
    fn the_clock_table_lists_ellinia_station_and_only_real_fields() {
        let clocks = std::path::Path::new("../../gm-handbook/clocks.txt");
        let fields = std::path::Path::new("../../gm-handbook/fields.txt");
        if !clocks.exists() || !fields.exists() {
            return; // generated data, gitignored
        }
        let clocks = Config::load_clocks(clocks);
        let fields = Config::load_fields(fields);
        assert!(clocks.contains(&10_002_090), "Ellinia Station has a clock node: {clocks:?}");
        assert!(!clocks.is_empty());
        for map in &clocks {
            assert!(fields.contains(map), "{map} declares a clock but has no field image");
        }
        // The control: a map with no clock node is not in it. Ellinia town itself.
        assert!(!clocks.contains(&10_002_000), "Ellinia has no clock node");
        // And the loader reads the FIRST column, so a placement row parses to its map id.
        let dir = std::env::temp_dir().join(format!("maplecw-clocks-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("clocks.txt");
        std::fs::write(&p, "# header\n10002090, 635, -226, 200, 200\n\n7, 0, 0, 0, 0\n").unwrap();
        let parsed = Config::load_clocks(&p);
        assert_eq!(parsed.len(), 2);
        assert!(parsed.contains(&10_002_090) && parsed.contains(&7));
        let _ = std::fs::remove_dir_all(&dir);
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

/// The hot-swappable NPC dialogue overlay: its parser, and the table it swaps.
///
/// The session-level claim - *a reload reaches a connection that was already open* - is not
/// here. It cannot be: it is a property of two `Session`s sharing one `Arc<Config>`, and it
/// is tested in `session::gm`.
#[cfg(test)]
mod npc_dialogue_tests {
    use super::*;

    fn base(rows: &[(u32, &str)]) -> HashMap<u32, NpcStrings> {
        rows.iter()
            .map(|(id, d0)| {
                let dialogue = if d0.is_empty() { Vec::new() } else { vec![(*d0).to_string()] };
                (*id, NpcStrings { name: format!("NPC {id}"), dialogue, ..Default::default() })
            })
            .collect()
    }

    /// The happy path, and the only thing the parser is allowed to accept.
    #[test]
    fn a_d_row_is_taken_and_its_text_survives_a_tab_free_body() {
        let o = parse_npc_dialogue_overlay("8\td0\tHello, and #p8# too\n");
        assert_eq!(o.rows, 1);
        assert!(o.refused.is_empty(), "{:?}", o.refused);
        assert_eq!(o.lines[&8], vec!["Hello, and #p8# too".to_string()]);
    }

    /// **`d10` must follow `d9`.** A string sort puts it second, and `read_quest_rows` carries
    /// the same warning - the screen would show it as a conversation that jumps.
    #[test]
    fn overlay_lines_come_out_in_index_order_not_string_order() {
        let mut text = String::new();
        for i in 0..12 {
            text.push_str(&format!("8\td{i}\tline {i}\n"));
        }
        let o = parse_npc_dialogue_overlay(&text);
        assert!(o.refused.is_empty(), "{:?}", o.refused);
        let lines = &o.lines[&8];
        assert_eq!(lines.len(), 12);
        assert_eq!(lines[9], "line 9");
        assert_eq!(lines[10], "line 10", "string order would put this second");
    }

    /// **The guarantee `Config::shop_by_template` rests on, enforced rather than commented.**
    ///
    /// `CLAUDE.md`: *"A comment describing a guarantee is not the guarantee."* The shop join is
    /// built from NPC names at start-up into a table behind no lock; if this file could carry a
    /// `name` row, a reload could silently re-point a shop. So the parser refuses it, and the
    /// refusal is counted rather than dropped.
    #[test]
    fn a_name_row_is_refused_so_a_reload_can_never_move_a_shop() {
        let o = parse_npc_dialogue_overlay("21\tname\tNot Lucy\n21\td0\tKept\n");
        assert_eq!(o.rows, 2);
        assert_eq!(o.lines[&21], vec!["Kept".to_string()]);
        assert_eq!(o.refused.len(), 1, "{:?}", o.refused);
        assert!(o.refused[0].contains("line 1"), "{}", o.refused[0]);
        assert!(o.refused[0].contains("name"), "{}", o.refused[0]);
    }

    /// `info`/`idle` text lives in the CLIENT - only an index goes on the wire - so authoring
    /// it here would change nothing on screen while changing how many indices are cycled.
    #[test]
    fn info_and_idle_rows_are_refused_because_that_text_is_not_the_servers_to_send() {
        let o = parse_npc_dialogue_overlay("8\tinfo0\tnope\n8\tidle3\talso nope\n");
        assert!(o.lines.is_empty(), "{:?}", o.lines);
        assert_eq!(o.refused.len(), 2, "{:?}", o.refused);
    }

    /// Every other way a row can be wrong is named, not silently skipped. An author who cannot
    /// see the refusal concludes the command is broken.
    #[test]
    fn each_malformed_row_is_refused_by_name() {
        let o = parse_npc_dialogue_overlay(
            "8 d0 spaces not tabs\nrobin\td0\tnot an id\n8\tdx\tnot an index\n8\td0\t   \n\
             8\td1\tone\n8\td1\ttwo\n",
        );
        assert_eq!(o.rows, 6);
        assert_eq!(o.refused.len(), 5, "{:?}", o.refused);
        // The duplicate is a refusal AND the later row wins, which is stated in the message.
        assert_eq!(o.lines[&8], vec!["two".to_string()]);
    }

    /// Comments and blank lines are not rows, so a file of nothing but the header is not a
    /// file that "had rows and produced nothing".
    #[test]
    fn comments_and_blank_lines_are_not_counted_as_rows() {
        let o = parse_npc_dialogue_overlay("# a header\n\n   \n#\tso is this\n");
        assert_eq!(o.rows, 0);
        assert!(o.refused.is_empty());
        assert!(o.lines.is_empty());
    }

    // ---- the table and the reload -------------------------------------------------------

    fn scratch(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("maplecw-npcreload-{tag}-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn config_with(base_rows: &[(u32, &str)], overlay: &std::path::Path) -> Config {
        Config {
            npc_strings: base(base_rows).into(),
            npc_strings_path: PathBuf::from("gm-handbook/npcstrings.txt"),
            npc_dialogue_path: overlay.to_path_buf(),
            ..Config::default()
        }
    }

    /// The counts the command reports, split the way a person would ask the question:
    /// *did my edit replace something, or add something that was not there?*
    #[test]
    fn the_report_separates_a_replaced_line_from_an_added_one() {
        let dir = scratch("counts");
        let file = dir.join("npc-dialogue.txt");
        // 8 already speaks; 9 has an entry with no `d0` at all; 10 is not in the base.
        let config = config_with(&[(8, "shipped"), (9, "")], &file);
        std::fs::write(&file, "8\td0\tA\n9\td0\tB\n10\td0\tC\n").unwrap();

        let r = reload_npc_dialogue(&config);
        assert!(r.applied(), "{:?}", r.refusal);
        assert_eq!(r.base_templates, 2);
        assert_eq!(r.overlay_templates, 3);
        assert_eq!(r.overridden, 1, "only template 8 had a line to replace");
        assert_eq!(r.added, 2, "9 had an entry but no dialogue, 10 had no entry");
        assert_eq!(r.live_templates, 3);
        assert_eq!(config.npc_strings.dialogue_line(8).as_deref(), Some("A"));
        assert_eq!(config.npc_strings.dialogue_line(10).as_deref(), Some("C"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **A file that fails to parse leaves the running table exactly as it was.**
    ///
    /// The realistic failure is an edit saved with spaces instead of tabs. Applying that would
    /// quietly revert every override the author already had working, between two packets,
    /// with the command reporting success.
    #[test]
    fn a_file_with_rows_and_no_usable_row_is_refused_and_changes_nothing() {
        let dir = scratch("spaces");
        let file = dir.join("npc-dialogue.txt");
        let config = config_with(&[(8, "shipped")], &file);

        std::fs::write(&file, "8\td0\tamended\n").unwrap();
        assert!(reload_npc_dialogue(&config).applied());
        assert_eq!(config.npc_strings.dialogue_line(8).as_deref(), Some("amended"));

        std::fs::write(&file, "8 d0 saved with spaces\n9 d0 and another\n").unwrap();
        let r = reload_npc_dialogue(&config);
        assert!(!r.applied(), "a file with rows and no usable row must refuse");
        assert!(r.refusal.as_ref().unwrap().contains("TAB"), "{:?}", r.refusal);
        assert_eq!(r.live_templates, 1, "the count reported is the SURVIVING table");
        assert_eq!(
            config.npc_strings.dialogue_line(8).as_deref(),
            Some("amended"),
            "the previous overlay must survive a failed reload"
        );
        assert!(r.summary().contains("NOTHING changed"), "{}", r.summary());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A missing overlay is legal - the same handling `data/quest-scripts.txt` has - and it
    /// means "no overrides", so a file that is deleted takes its overrides with it.
    #[test]
    fn a_missing_overlay_file_is_not_a_failure_and_reverts_to_the_generated_base() {
        let dir = scratch("missing");
        let file = dir.join("npc-dialogue.txt");
        let config = config_with(&[(8, "shipped")], &file);

        std::fs::write(&file, "8\td0\tamended\n").unwrap();
        assert!(reload_npc_dialogue(&config).applied());
        assert_eq!(config.npc_strings.dialogue_line(8).as_deref(), Some("amended"));

        std::fs::remove_file(&file).unwrap();
        let r = reload_npc_dialogue(&config);
        assert!(r.applied(), "{:?}", r.refusal);
        assert_eq!(r.overlay_templates, 0);
        assert_eq!(config.npc_strings.dialogue_line(8).as_deref(), Some("shipped"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// An empty generated base is refused rather than reported as a successful reload of
    /// nothing. `load_npc_strings` returns an empty map on a read error, so without this the
    /// command says "ok" over a server whose NPCs cannot say anything.
    #[test]
    fn an_empty_generated_base_is_refused_rather_than_reported_as_success() {
        let dir = scratch("nobase");
        let file = dir.join("npc-dialogue.txt");
        std::fs::write(&file, "8\td0\tamended\n").unwrap();
        let config = config_with(&[], &file);

        let r = reload_npc_dialogue(&config);
        assert!(!r.applied());
        assert!(r.refusal.as_ref().unwrap().contains("dump_npcstrings"), "{:?}", r.refusal);
        assert_eq!(r.live_templates, 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The base is never re-read and never overwritten, which is what keeps
    /// `Config::shop_by_template` honest for the life of the process.
    #[test]
    fn a_reload_leaves_the_generated_base_and_every_name_untouched() {
        let dir = scratch("names");
        let file = dir.join("npc-dialogue.txt");
        let config = config_with(&[(21, "shipped")], &file);
        std::fs::write(&file, "21\tname\tNot Lucy\n21\td0\tamended\n").unwrap();

        let r = reload_npc_dialogue(&config);
        assert!(r.applied(), "{:?}", r.refusal);
        assert_eq!(r.refused.len(), 1, "{:?}", r.refused);
        assert_eq!(config.npc_strings.base()[&21].name, "NPC 21");
        assert_eq!(config.npc_strings.snapshot()[&21].name, "NPC 21", "the live name too");
        assert_eq!(config.npc_strings.dialogue_line(21).as_deref(), Some("amended"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A `Config::clone` must SHARE the table, not copy it. A copy would keep serving the old
    /// dialogue after a reload with nothing in any log to say so.
    #[test]
    fn cloning_a_config_shares_the_swappable_table_instead_of_copying_it() {
        let dir = scratch("clone");
        let file = dir.join("npc-dialogue.txt");
        let config = config_with(&[(8, "shipped")], &file);
        let twin = config.clone();

        std::fs::write(&file, "8\td0\tamended\n").unwrap();
        assert!(reload_npc_dialogue(&config).applied());
        assert_eq!(twin.npc_strings.dialogue_line(8).as_deref(), Some("amended"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The notice has to stay inside the longest one this client has been seen to draw, and
    /// the only unbounded parts of it are the two paths. Nothing is clipped at the defaults.
    #[test]
    fn the_acknowledgement_stays_within_the_longest_notice_this_client_has_drawn() {
        // 474 is the longest notice this client has been SEEN to draw, which is what makes it
        // the bound. It is not `GM_COMMANDS.len()`: this comment used to say the help text
        // "is 474 characters and is shipping" and by 2026-09-09 it was **330**, because the
        // 09-06 prune shortened the string and left the number behind. Measured, not read.
        const PROVEN: usize = 474;
        assert_eq!(clip_tail("data/npc-dialogue.txt"), "data/npc-dialogue.txt", "no clipping at the default");
        let long = format!("C:\\{}\\npc-dialogue.txt", "d".repeat(300));
        assert!(clip_tail(&long).chars().count() <= 63);
        assert!(clip_tail(&long).ends_with("npc-dialogue.txt"), "the tail is the useful half");

        let r = NpcDialogueReload {
            base_path: PathBuf::from("gm-handbook/npcstrings.txt"),
            overlay_path: PathBuf::from(long),
            base_templates: 266,
            overlay_templates: 12,
            overridden: 9,
            added: 3,
            live_templates: 269,
            ..Default::default()
        };
        assert!(r.summary().len() <= PROVEN, "{} chars: {}", r.summary().len(), r.summary());

        // The refusal branch is a different sentence and embeds a path of its own, so it gets
        // its own bound rather than being assumed to inherit this one.
        let dir = scratch("longpath");
        let deep = dir.join("a".repeat(80)).join("b".repeat(80));
        std::fs::create_dir_all(&deep).unwrap();
        let file = deep.join("npc-dialogue.txt");
        std::fs::write(&file, "8 d0 saved with spaces\n").unwrap();
        let config = config_with(&[(8, "shipped")], &file);
        let refused = reload_npc_dialogue(&config);
        assert!(!refused.applied());
        assert!(
            refused.summary().len() <= PROVEN,
            "{} chars: {}",
            refused.summary().len(),
            refused.summary()
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **The files that actually ship, through the code that actually runs.**
    ///
    /// Everything above is synthetic. This one reads the committed `data/npc-dialogue.txt`
    /// against the generated `gm-handbook/npcstrings.txt` and asserts the reload applies with
    /// nothing refused - which is what catches a header comment that accidentally parses as a
    /// row, or a file saved with the tabs expanded.
    #[test]
    fn the_committed_overlay_applies_cleanly_against_the_real_generated_table() {
        let base_path = std::path::Path::new("../../gm-handbook/npcstrings.txt");
        let overlay = std::path::Path::new("../../data/npc-dialogue.txt");
        assert!(overlay.exists(), "data/npc-dialogue.txt is authored source and is committed");
        if !base_path.exists() {
            return; // generated data, gitignored
        }
        let config = Config {
            npc_strings: load_npc_strings(base_path).into(),
            npc_strings_path: base_path.to_path_buf(),
            npc_dialogue_path: overlay.to_path_buf(),
            ..Config::default()
        };
        let before = config.npc_strings.len();
        let r = reload_npc_dialogue(&config);
        assert!(r.applied(), "{:?}", r.refusal);
        assert!(r.refused.is_empty(), "{:?}", r.refused);
        assert!(r.base_templates > 200, "only {} templates", r.base_templates);
        assert!(r.live_templates >= before);
        // Robin is the fixture the rest of this file uses, so their line is the control that
        // the base really was read rather than the overlay having supplied everything.
        assert!(config.npc_strings.dialogue_line(8).is_some(), "Robin still has a d0");
    }

    /// Two reloads of the same file are the same table - the overlay is applied to the base
    /// each time, never to the result of the last one.
    #[test]
    fn a_second_reload_of_the_same_file_does_not_accumulate() {
        let dir = scratch("twice");
        let file = dir.join("npc-dialogue.txt");
        let config = config_with(&[(8, "shipped")], &file);
        std::fs::write(&file, "8\td0\tone\n8\td1\ttwo\n").unwrap();

        let first = reload_npc_dialogue(&config);
        let live_after_first = config.npc_strings.snapshot()[&8].dialogue.clone();
        let second = reload_npc_dialogue(&config);
        assert_eq!(config.npc_strings.snapshot()[&8].dialogue, live_after_first);
        assert_eq!(live_after_first, vec!["one".to_string(), "two".to_string()]);
        assert_eq!(first.live_templates, second.live_templates);
        assert_eq!(first.overridden, second.overridden);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
