//! One channel connection's protocol state, as a pure state machine.
//!
//! Bodies in, bodies out - no socket, no cipher, no clock, for the same reason
//! `login::session` is built that way: a client launch costs the owner a manual elevated run,
//! so anything that can be a unit test has to be one.
//!
//! # This stage is not decoded yet
//!
//! Everything the client sends on a channel connection is new. The only packet with a name
//! is `0x007D`, and that name was read out of the client rather than captured: the builder
//! is `FUN_1415d10e0`, and it writes the `u32` the migration packet handed over. So the
//! first job here is to log what arrives and pick the seed out of `0x007D` - which is also
//! the check on the whole `0x0011` decode.
//!
//! **Nothing here answers anything yet, deliberately.** An unanswered request freezes the
//! client's UI, so this is not a state to stay in - but answering a packet whose meaning is
//! unknown is worse than not answering, because a wrong reply moves the client into a state
//! nobody has read. Log first, then decode, then answer.

use std::sync::Arc;

use store::{ClaimedMigration, Store};

use crate::config::Config;

/// One packet to send, plus what it is - the label goes in the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    pub opcode: u16,
    pub body: Vec<u8>,
    pub what: String,
}

impl Reply {
    /// Opcode then body - the packet as the framer wants it.
    pub fn packet(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(2 + self.body.len());
        out.extend_from_slice(&self.opcode.to_le_bytes());
        out.extend_from_slice(&self.body);
        out
    }
}

/// The client's hello on a channel connection, built by `FUN_1415d10e0`.
///
/// **It carries the character id, not the migration seed.** The seed was the design's
/// assumption and the capture disproved it - see [`migration_hello_character`] for the
/// measured layout. The bytes in front of the id are still undecoded: `FUN_1415d10e0`
/// writes two `u32` before it, and the tail is the same MAC and machine id `0x0073`
/// carries.
pub const CLIENT_MIGRATION_HELLO: u16 = 0x007D;

/// The client asking to walk through a portal.
///
/// **Decoded from the run that first put a character on map 1**, 2026-08-19 - the owner used the
/// right-hand portal and nothing happened, because we answered nothing. Full working in
/// `research/transfer-field-request.md`; every field below marked *proven* was read both off
/// the builder `FUN_1418283f0` and off the capture.
///
/// ```text
///  0  u32   100          a literal - MOV EDX,0x64 in the builder
///  4  u16                a checksum-protected counter
///  6  u64                a protected value, the 0x9a65 family
/// 14  u8                 0 from the move-path caller
/// 15  u8                 derived from the current stage
/// 16  u32   -1           targetField: -1 means "use the portal"        PROVEN
/// 20  u16 + bytes        portalName, e.g. "out00"                      PROVEN
/// 27  u16                character x                                   PROVEN
/// 29  u16                character y                                   PROVEN
/// 31  u8    0            hard-coded
/// 32  u8                 0 from the caller
/// 33  u8                 0 from the caller
/// ```
///
/// **The body is 31 bytes, not 34, when the portal name is empty** - the client skips both
/// coordinate writes when the name pointer is null, so x and y are not at a fixed offset from
/// the end. Parse the string first and let it tell you where they are.
pub const CLIENT_TRANSFER_FIELD: u16 = 0x00D1;

/// The client announcing it has finished entering a field. **Once per field, every time.**
///
/// **Measured 2026-08-19**, from `research/fixtures/portal-works-npcs-and-avatar-do-not-world.log`:
/// this arrives ~420 ms after every `SetField` - the first migration and every portal walk
/// alike. That is what makes it the per-field marker.
///
/// It replaced `0x0238`, and the reason is worth keeping. `0x0238` and `0x024D` are built
/// back to back by `FUN_142caa4e0`, the world object's field-entry reset, which is why they
/// looked like the field-entry signal. But the capture shows **`0x0238` arrives only on the
/// FIRST field entry** and never again - the three portal transitions in that run produced
/// no `0x0238` at all, only `0x00DC`. So NPCs triggered on `0x0238` could never appear after
/// a portal walk even if everything else were right.
pub const CLIENT_FIELD_ENTERED: u16 = 0x00DC;

/// Sent once, with `0x024D`, on the first field entry only - **not** a per-field marker.
/// Kept named so nobody re-derives it from the capture and reaches for it again.
pub const CLIENT_ENTERED_WORLD_ONCE: u16 = 0x0238;

/// What the client asked for in a [`CLIENT_TRANSFER_FIELD`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferFieldRequest {
    /// `-1` on the wire means "no explicit target - resolve `portal_name` instead".
    pub target_field: Option<u32>,
    pub portal_name: String,
    /// Absent when the portal name is empty; the client omits both coordinates then.
    pub position: Option<(u16, u16)>,
}

/// Parse a [`CLIENT_TRANSFER_FIELD`] body (opcode already stripped).
///
/// Returns `None` only if the body is too short to hold the fixed part. Everything before
/// offset 16 is a client integrity block we neither check nor echo.
pub fn parse_transfer_field(body: &[u8]) -> Option<TransferFieldRequest> {
    const TARGET_AT: usize = 16;
    const NAME_AT: usize = 20;
    let raw = u32::from_le_bytes(body.get(TARGET_AT..TARGET_AT + 4)?.try_into().ok()?);
    let len = u16::from_le_bytes(body.get(NAME_AT..NAME_AT + 2)?.try_into().ok()?) as usize;
    let name_end = NAME_AT + 2 + len;
    let portal_name = String::from_utf8_lossy(body.get(NAME_AT + 2..name_end)?).into_owned();
    let position = if portal_name.is_empty() {
        None
    } else {
        let x = u16::from_le_bytes(body.get(name_end..name_end + 2)?.try_into().ok()?);
        let y = u16::from_le_bytes(body.get(name_end + 2..name_end + 4)?.try_into().ok()?);
        Some((x, y))
    };
    Some(TransferFieldRequest {
        target_field: if raw == u32::MAX { None } else { Some(raw) },
        portal_name,
        position,
    })
}

/// One channel connection.
pub struct Session {
    store: Arc<Store>,
    config: Arc<Config>,
    /// The migration this connection claimed, once it has claimed one.
    claimed: Option<ClaimedMigration>,
}

impl Session {
    pub fn new(store: Arc<Store>, config: Arc<Config>) -> Self {
        Session { store, config, claimed: None }
    }

    /// What the channel sends the moment the client connects: **nothing**.
    ///
    /// The login server sends the `0x0032` startup gate here, unprompted. That is right
    /// there and wrong here - it releases the login connection's startup loop, and this
    /// connection has no startup loop. It is also the most likely reason the first
    /// migrated connection was rejected with "The client is outdated".
    pub fn on_connect(&mut self) -> Vec<Reply> {
        Vec::new()
    }

    /// Handle one packet body, opcode included.
    ///
    /// **Answers nothing at all unless [`Config::set_field_probe`] is on**, which is why
    /// `tools/test-server.ps1` must be given `-SetFieldProbe`. Without it the migration
    /// hello goes unanswered and the client freezes on "Connecting..." - the exact failure
    /// the "always answer" rule exists to prevent, sitting in the default configuration.
    ///
    /// **The flag is a misnomer.** It dates from when the channel's only job was to answer
    /// the migration hello with a hand-built `SetField` and see whether the client accepted
    /// it. It now gates six handlers: the migration, the portal walk, field entry (NPCs and
    /// mobs), chat (the `!map` GM command) and the quest request. Renaming it would break
    /// the launch line in `STATUS.md` and in every fixture note, so it stays until something
    /// else about the launcher changes.
    ///
    /// The original reasoning still holds for what is *not* answered: a wrong reply moves
    /// the client into a state nobody has read, which is worse than silence. Every opcode
    /// below is one whose handler has been read, and unknown ones fall through to nothing.
    pub fn handle(&mut self, body: &[u8]) -> Vec<Reply> {
        let opcode = match body.get(..2) {
            Some(b) => u16::from_le_bytes([b[0], b[1]]),
            None => return Vec::new(),
        };
        if !self.config.set_field_probe {
            return Vec::new();
        }
        match opcode {
            CLIENT_MIGRATION_HELLO => {}
            CLIENT_TRANSFER_FIELD => return self.on_transfer_field(body.get(2..).unwrap_or(&[])),
            CLIENT_FIELD_ENTERED => return self.on_field_entered(),
            net::opcode::CLIENT_CHAT => return self.on_chat(body.get(2..).unwrap_or(&[])),
            net::script::CLIENT_QUEST_REQUEST => {
                return self.on_quest_request(body.get(2..).unwrap_or(&[]))
            }
            net::script::CLIENT_NPC_CLICK => {
                return self.on_npc_click(body.get(2..).unwrap_or(&[]))
            }
            _ => return Vec::new(),
        }
        // Always answer. An unanswered packet freezes the client's whole UI - every
        // button, including the quit prompt - and reads on screen as a crash. So a
        // character we cannot load falls back to the minimal record rather than silence.
        let (body, what) = match self.claimed_character() {
            Some(chr) => (
                net::opcode::set_field_with_character_dressed(
                    &chr,
                    self.config.world_id,
                    self.clock_base(),
                    self.config.channel_id,
                    &self.dressed(&chr),
                ),
                format!(
                    "SetField, characterData=1, presence[0] set so the character-stat block                      decodes, carrying map {} for character {} ({}). presence[0] is gate                      entry 7, settled in research/charrecord-presence-map.md; the map id                      sits at stat-block offset {}, settled in research/charstat-layout.md.                      Nothing here authenticates anybody.",
                    chr.map_id,
                    chr.id,
                    chr.name,
                    net::opcode::stat_block_map_id_at(chr.job),
                ),
            ),
            None => (
                net::opcode::set_field_minimal(self.clock_base(), self.config.channel_id),
                "SetField, characterData=1, MINIMAL record - the character could not be                  loaded, so this falls back to the all-flags-clear form. It is answered                  rather than dropped because an unanswered packet freezes the client's                  whole UI. It will NOT put the character on a map: with every presence                  flag clear the stat block never decodes, so there is no map id at all."
                    .to_string(),
            ),
        };
        vec![Reply { opcode: net::opcode::SET_FIELD, body, what }]
    }

    /// Populate the field the client has just finished entering.
    ///
    /// The client does **not** spawn NPCs from the map WZ - its field loader walks `life`
    /// only to preload art. The only code that builds a populated NPC takes a packet, so
    /// every NPC on every field is ours to send, and ours to re-send after every `SetField`
    /// because the pool is destroyed and rebuilt empty on each field entry.
    ///
    /// `research/npc-spawn.md` has the working, including how the routing was found.
    fn on_field_entered(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let empty: Vec<net::opcode::FieldNpc> = Vec::new();
        let out: Vec<Reply> = self.config.npcs.get(&chr.map_id).unwrap_or(&empty)
            .iter()
            .map(|npc| Reply {
                opcode: net::opcode::NPC_ENTER_FIELD,
                body: net::opcode::npc_enter_field(npc),
                what: format!(
                    "NpcEnterField: template {} at ({}, {}) on foothold {}, object id {} -                      the client cannot spawn this itself, it only preloads the art.",
                    npc.template_id, npc.x, npc.cy, npc.fh, npc.object_id
                ),
            })
            .collect();

        // Mobs, from the same WZ `life` walk that produced the NPCs and for the same
        // reason: the client's field loader only preloads `Mob/%07d.img` art, and the pool
        // is destroyed and rebuilt empty on every field entry, so they must be re-sent
        // after every SetField rather than once.
        //
        // **OFF BY DEFAULT since the run of 2026-08-19, because the body faults the
        // client.** TestCharD entered map 40 (Snail Hunting Ground I, 40 spawns); the two
        // NPCs dispatched cleanly and the client then died on the FIRST 0x03C6 with
        // 0xC0000005 at `0x141c810b0`. That is inside `FUN_141c81040`, and the faulting
        // instruction is `CMP qword ptr [RCX],RDX` after
        // `MOV RAX,[RSI+0x2b8] / LEA RCX,[RAX+0x828] / CMOVE RCX,RDX` - so **`mob+0x2b8`
        // was null** and the client dereferenced without a guard.
        //
        // Everything else on that entry worked, which is what makes the diagnosis narrow:
        // the character record decoded, the equipped list decoded all four items, and both
        // NPCs went through. Mobs are the only thing that changed the outcome.
        //
        // Turn back on with `--mobs` when the body is the variant under test.
        let no_mobs: Vec<net::mob::FieldMob> = Vec::new();
        let mut out = out;
        let mobs = if self.config.send_mobs {
            self.config.mobs.get(&chr.map_id).unwrap_or(&no_mobs)
        } else {
            &no_mobs
        };
        // A spawn point is not a mob. Map 40 has 40 spawn points and a real server keeps
        // about 30 of them filled for a solo player, so sending one per point
        // over-populates the field. The cap is NOT in the WZ - map 40's info node has a
        // mobRate but no capacity of any name - so it is our policy; see
        // config::spawn_capacity for what is measured and what is inferred.
        //
        // And which points are filled matters as much as how many: a mixed map keeps each
        // type's SHARE of the total, so this cannot just take the first N in WZ order.
        // Players ON THE FIELD, not on the channel. Always 1 today: there is no
        // field-occupancy tracking here at all, so the 6+ branch of config::spawn_capacity
        // is written and untaken. Adding occupancy is a change to this line.
        let players_here = 1;
        let alive = crate::config::spawn_capacity(mobs.len(), players_here);
        let alive = match self.config.mob_limit {
            Some(n) => alive.min(n),
            None => alive,
        };
        let chosen = crate::config::share_balanced(mobs, alive);
        out.extend(chosen.into_iter().map(|mob| {
            Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(mob),
                what: format!(
                    "MobEnterField: template {} at ({}, {}) on foothold {}, object id {},                      hp {} - {} bytes. The client cannot spawn this itself.",
                    mob.template_id, mob.x, mob.y, mob.fh, mob.object_id, mob.hp,
                    mob.body_len()
                ),
            }
        }));

        // The character is dressed by the SetField record itself now, not from here. This
        // used to push a 0x0138 UserAvatarModified as a guess at the equipment problem;
        // that opcode is **dead code at byte level** - its apply is guarded by a call to
        // 0x1407f5ce0, which is three bytes of `xor eax,eax; ret`, followed by TEST/JZ. No
        // trigger and no timing would ever have made it work, so sending it was noise in
        // the log. See net::opcode::USER_AVATAR_MODIFIED and research/naked-character.md.
        out
    }

    /// Make the NPC the client just asked about say something.
    ///
    /// **`0x0151` is the quest request, not an NPC click.** Its first `u32` is a *quest id*
    /// and its second is the NPC **template** id - the same template the server sent in
    /// `NpcEnterField`. An earlier note in `STATUS.md` read the first field as our own
    /// object id; our own logs disprove it, because every map's first NPC is given object
    /// id 1000 and the client answered 1000/1002/1003/1005 for four different NPCs, the
    /// same values in both sessions despite opposite visit orders. Full working:
    /// `research/npc-dialogue.md`.
    ///
    /// **The speaker is the template the client named**, which is what makes this safe: it
    /// is by construction a real `Npc.wz` id, and a bad one costs the portrait rather than
    /// faulting.
    ///
    /// **This is text on screen, not a quest.** No quest-result packet has been found, so
    /// nothing here advances any state - accepting the same quest twice will show the same
    /// message. Say so in the message rather than letting the screen imply otherwise.
    ///
    /// **Ordering.** A script must never be sent with or just before a `SetField`: field
    /// entry runs `FUN_142caa4e0`, which resets the script manager and tears the dialog
    /// down silently. This path is a reply to a click, which is long after field entry, so
    /// it is clear - but the constraint is why this does not simply fire on arrival.
    fn on_quest_request(&mut self, body: &[u8]) -> Vec<Reply> {
        // Always answer. An unanswered request freezes the client's whole UI, so a body
        // that does not parse still gets a reply - parse_quest_request only returns None
        // when the fixed 9-byte head does not fit, and then there is no template to speak
        // as, which is the one case where silence is all there is.
        let Some(req) = net::script::parse_quest_request(body) else {
            return Vec::new();
        };
        let text = format!(
            "Quest {} is not implemented on this server yet. Nothing you do here will \
             advance it - there is no quest state at all.",
            req.quest_id
        );
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(req.npc_template_id, &text, false, false),
            what: format!(
                "ScriptMessage Say from NPC template {} for quest {} (action {}) - text on \
                 screen only. No quest-result packet is known, so no state advances.",
                req.npc_template_id, req.quest_id, req.action
            ),
        }]
    }

    /// Make an NPC with **no quest** speak. This is the other half of goal 2.
    ///
    /// **There are two NPC-click packets and the server was answering only one.** Which one
    /// goes out is decided entirely inside the client, from `Quest.wz`: `FUN_1428de280`
    /// forks on whether the NPC has a non-empty script name, and only a menu line carrying a
    /// quest id reaches the `0x0151` builder. Every other outcome sends `0x00F2`. Robin on
    /// map 40 has no quests, so clicking them produced `0x00F2` and total silence on
    /// 2026-08-19. See `research/npc-click.md`.
    ///
    /// **The one thing that differs from the quest path**: `0x0151` hands us the NPC's
    /// *template* id, and `0x00F2` hands us the **object** id we chose - while `0x055B`'s
    /// speaker field wants a template. So this maps back through the same table that
    /// assigned the object id. Sending the object id straight through would not fault (the
    /// loader result is null-checked at `142a7b52a`) but would draw a portrait-less box.
    fn on_npc_click(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(click) = net::script::parse_npc_click(body) else { return Vec::new() };
        let Some(chr) = self.claimed_character() else { return Vec::new() };

        // The object id is only unique within a field, which is why the lookup is scoped to
        // the character's current map - config::load_npcs numbers from 1000 per map.
        let template = self
            .config
            .npcs
            .get(&chr.map_id)
            .and_then(|list| list.iter().find(|n| n.object_id == click.npc_object_id))
            .map(|n| n.template_id);

        let Some(template) = template else {
            // Nothing to speak as. Answering with a script whose speaker is not a real
            // Npc.wz id buys nothing, and this is not a request the client blocks on - the
            // 2026-08-19 capture shows the UI stayed live with 0x00F2 unanswered.
            return Vec::new();
        };

        let text = format!(
            "I have nothing to say yet. This server knows me as NPC template {template}, and \
             it has no dialogue script for me."
        );
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(template, &text, false, false),
            what: format!(
                "ScriptMessage Say from NPC template {} (object id {}, the id we assigned on \
                 map {}) - the NO-QUEST click path, 0x00F2. Text on screen only.",
                template, click.npc_object_id, chr.map_id
            ),
        }]
    }

    /// Each worn item with the stats its `Character.wz` template gives it.
    ///
    /// **The stats belong to the item template, not to the character**, so they are resolved
    /// here rather than persisted: `crates/store` keeps `(slot, itemId)` and nothing else,
    /// and a stat column in the database would be a second source of truth for a value the
    /// client already has its own copy of.
    ///
    /// An item with no template row goes out bare. That is the behaviour confirmed on screen
    /// on 2026-08-19 - the character was dressed, the items simply had no stats - so a
    /// missing or stale `gm-handbook/equips.txt` degrades to something known to work rather
    /// than to something untested.
    fn dressed(&self, chr: &net::opcode::Character) -> Vec<(u8, u32, net::opcode::EquipStats)> {
        chr.equips
            .iter()
            .map(|&(slot, item_id)| {
                let stats = self
                    .config
                    .equips
                    .get(&item_id)
                    .map(|t| t.fresh_stats())
                    .unwrap_or_default();
                (slot, item_id, stats)
            })
            .collect()
    }

    /// Move a character to a map and tell the client, persisting the move.
    ///
    /// Shared by the portal walk and the `/map` GM command, so both go through one path -
    /// a second copy of this is how the two would drift.
    fn go_to_map(&mut self, chr: &mut net::opcode::Character, map: u32, portal: u8, why: String)
        -> Vec<Reply>
    {
        chr.map_id = map;
        chr.portal = portal;
        let stored = self.store.set_character_map(chr.id, map);
        let warn = match stored {
            Ok(()) => String::new(),
            // Not fatal: the client is told where it is either way, and the next login puts
            // it back where it was.
            Err(e) => format!(" - WARNING: not stored ({e}), so this will not survive a relog"),
        };
        vec![Reply {
            opcode: net::opcode::SET_FIELD,
            body: net::opcode::set_field_with_character(
                chr,
                self.config.world_id,
                self.clock_base(),
                self.config.channel_id,
            ),
            what: format!("SetField, {why}, for character {} ({}){warn}", chr.id, chr.name),
        }]
    }

    /// GM commands typed into the chat box.
    ///
    /// **This is a debugging tool on a server where nothing authenticates**, so there is no
    /// permission check to write - every connection is already the same account, and adding
    /// one here would be theatre. Say so rather than implying otherwise.
    ///
    /// Chat is fire-and-forget: the client froze on none of the runs where it went
    /// unanswered, so a command that does nothing is safe.
    ///
    /// ## The prefix is `!`, not `/`, and that is not a preference
    ///
    /// **The client never transmits a `/` line.** The owner typed `/map 1` and the session's
    /// entire capture contains no `0x00E7` at all, while a plain "Hello" in the same tab had
    /// produced one. The client parses slash commands itself: `/find`, `/whisper`, `/party`,
    /// `/friend`, `/trade`, `/level` and `/h` are all baked into the executable as strings,
    /// and an unknown one is swallowed before it reaches the wire.
    ///
    /// So a server-side command has to look like ordinary chat. `!` is ordinary chat.
    fn on_chat(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(text) = net::opcode::parse_chat(payload) else { return Vec::new() };
        let text = text.trim();
        let Some(rest) = text.strip_prefix("!map ") else { return Vec::new() };
        let Ok(map) = rest.trim().parse::<u32>() else { return Vec::new() };

        // Refuse a map the client cannot load. A character sent to an id with no field image
        // is stranded with no way back except another command, and an id with no String.wz
        // name entry can take the client into a branch that does not return - see
        // research/map1-exists.md. Chat is fire-and-forget, so refusing is silent on screen;
        // the log line is the only feedback there is until an outbound notice exists.
        if !self.config.map_exists(map) {
            return Vec::new();
        }
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        // Portal 0 is the map's spawn point, which is where a GM warp should land.
        self.go_to_map(&mut chr, map, 0, format!("GM !map {map}"))
    }

    /// Answer the client walking into a portal.
    ///
    /// The reply is another `SetField` with `characterData = 1` - the **long** form, the one
    /// byte-identical to the packet that already worked, differing only in the map id at
    /// stat-block offset 84. The short `characterData = 0` form is the shape actually
    /// designed for "same character, new map", and it is probably correct here now that the
    /// client has a live field; but `research/transfer-field-request.md` could not prove its
    /// precondition (`world+0x2358`, the `CUserLocal` slot) is populated - an exhaustive scan
    /// found 222 readers and **no** store. Sending an unproven form to save 200 bytes would
    /// trade a working path for a guess. The long form is idempotent because the object it
    /// re-fetches is a lazy singleton.
    fn on_transfer_field(&mut self, payload: &[u8]) -> Vec<Reply> {
        let req = parse_transfer_field(payload);
        let chr = self.claimed_character();

        // Always answer. Even a request we cannot resolve gets a SetField for the map the
        // character is already on, because silence freezes the client's entire UI.
        let Some(mut chr) = chr else {
            return vec![Reply {
                opcode: net::opcode::SET_FIELD,
                body: net::opcode::set_field_minimal(self.clock_base(), self.config.channel_id),
                what: "transfer-field request, but the character could not be loaded -                        answered with the minimal record rather than dropped, because an                        unanswered packet freezes the client's whole UI."
                    .to_string(),
            }];
        };

        // Where the character ARRIVES. The source portal names its destination portal in
        // the WZ's `tn`, and the stat block wants that portal's index on the target map.
        // Without it every walk lands on the map's spawn point, which is right for a login
        // and wrong for a door - the character pops out somewhere else entirely.
        let mut arrival = 0u8;
        let (target, note) = match &req {
            Some(r) => match r.target_field.or_else(|| {
                self.config.portals.get(&(chr.map_id, r.portal_name.clone())).map(|(to, tn)| {
                    arrival = self
                        .config
                        .portal_index
                        .get(&(*to, tn.clone()))
                        .copied()
                        .unwrap_or(0);
                    *to
                })
            }) {
                Some(t) => (t, format!("portal {:?} -> map {t} portal {arrival}", r.portal_name)),
                None => (
                    chr.map_id,
                    format!(
                        "portal {:?} on map {} is NOT in the portal table, so this re-sends                          the current map rather than guessing a destination",
                        r.portal_name, chr.map_id
                    ),
                ),
            },
            None => (chr.map_id, "the body was too short to parse - re-sending the current map".to_string()),
        };


        self.go_to_map(&mut chr, target, arrival, note)
    }

    /// The character this connection claimed a migration for.
    ///
    /// `describe_hello` in `server.rs` claims the migration before `handle` runs, so by
    /// this point `claimed` is populated for a well-formed hello. The store has no
    /// lookup by character id alone, but a claim carries the account and world, and a
    /// character id is unique within those.
    fn claimed_character(&self) -> Option<net::opcode::Character> {
        let claimed = self.claimed.as_ref()?;
        self.store
            .characters_for(claimed.account_id, claimed.world_id)
            .ok()?
            .into_iter()
            .find(|c| c.id == claimed.character_id)
    }

    /// The 8 bytes the client stores as a server clock base, stamping its own tick beside
    /// them. A Windows `FILETIME` is the shape the reference server sends; nothing has been
    /// measured about what this client does with the value, so a plausible one is sent
    /// rather than zero.
    fn clock_base(&self) -> u64 {
        const FILETIME_1970: u64 = 116_444_736_000_000_000;
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        FILETIME_1970 + secs * 10_000_000
    }

    /// Claim the pending migration for a character, and say what happened.
    ///
    /// **The seed does not come back.** It was the design's assumption that the `u32` handed
    /// over in `0x0011` would return in `0x007D`; the capture says otherwise - it is absent
    /// from the body both plainly and under the obfuscated-block search - and what the
    /// client sends instead is the character id.
    ///
    /// So single use is carried entirely by the database row, which is where it always
    /// actually lived: a `u32` on the wire was never a secret, and this only removes the
    /// pretence that it was. Nothing here authenticates anybody.
    pub fn claim_for_character(&mut self, character_id: u32) -> String {
        match self.store.claim_migration_for_character(character_id) {
            Ok(Some(claimed)) => {
                let wrong_channel = claimed.world_id != self.config.world_id
                    || claimed.channel_id != self.config.channel_id;
                let note = format!(
                    "claimed the migration for character {} of account {} \
                     (world {} channel {})",
                    claimed.character_id, claimed.account_id, claimed.world_id, claimed.channel_id
                );
                self.claimed = Some(claimed);
                if wrong_channel {
                    format!(
                        "{note} - WRONG CHANNEL: this is world {} channel {}",
                        self.config.world_id, self.config.channel_id
                    )
                } else {
                    note
                }
            }
            Ok(None) => format!(
                "character {character_id} has no unconsumed migration - it was never \
                 minted, or already claimed, or it expired"
            ),
            Err(e) => format!("character {character_id} could not be checked: {e}"),
        }
    }

    /// The migration this connection claimed, if any.
    pub fn claimed(&self) -> Option<&ClaimedMigration> {
        self.claimed.as_ref()
    }
}

/// Where the character id sits in a `0x007D` body.
///
/// **Measured from a real capture, 2026-08-19**, decrypted with AES once the channel's
/// cipher was settled:
///
/// ```text
/// u32  0
/// u32  0
/// u32  characterId      <- 204, TestCharD
/// u8[6] MAC
/// u32  machine id
/// ...                    the same trailing identity block 0x0073 carries
/// ```
const HELLO_CHARACTER_AT: usize = 8;

/// The character id out of a `0x007D` body, or `None` if it is too short to hold one.
pub fn migration_hello_character(payload: &[u8]) -> Option<u32> {
    payload
        .get(HELLO_CHARACTER_AT..HELLO_CHARACTER_AT + 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> (Session, Arc<Store>, i64, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wanderer".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        let s = Session::new(store.clone(), Arc::new(Config::default()));
        (s, store, account_id, id)
    }

    /// The channel must not send the login server's startup gate. That packet is what a
    /// login connection needs and what a game connection almost certainly rejected.
    #[test]
    fn a_channel_says_nothing_on_connect() {
        let (mut s, _, _, _) = session();
        assert!(s.on_connect().is_empty());
    }

    #[test]
    fn a_pending_migration_is_claimed_once_and_remembered() {
        let (mut s, store, account_id, id) = session();
        store.create_migration(account_id, id, 0, 0).unwrap();

        let note = s.claim_for_character(id);
        assert!(note.contains("claimed the migration"), "{note}");
        assert!(!note.contains("WRONG CHANNEL"), "{note}");
        assert_eq!(s.claimed().unwrap().character_id, id);

        let mut other = Session::new(store, Arc::new(Config::default()));
        assert!(other.claim_for_character(id).contains("no unconsumed migration"));
    }

    #[test]
    fn a_migration_for_another_channel_is_reported_rather_than_silently_accepted() {
        let (_, store, account_id, id) = session();
        store.create_migration(account_id, id, 0, 7).unwrap();
        let config = Config { channel_id: 0, ..Config::default() };
        let mut s = Session::new(store, Arc::new(config));
        assert!(s.claim_for_character(id).contains("WRONG CHANNEL"));
    }

    #[test]
    fn a_character_with_no_migration_says_so_instead_of_failing() {
        let (mut s, _, _, _) = session();
        assert!(s.claim_for_character(999).contains("no unconsumed migration"));
    }

    /// The real 34 bytes the client sent when the owner walked into map 1's right-hand portal,
    /// copied out of `research/fixtures/character-on-map1-playable-world.log`. A parser
    /// tested against invented bytes proves only that it agrees with itself.
    #[test]
    fn the_captured_portal_request_parses() {
        let body = hex("64000000ad1500000000000000000000ffffffff05006f7574303053046d01000000");
        assert_eq!(body.len(), 34, "the capture is 34 bytes");

        let r = parse_transfer_field(&body).expect("the captured body parses");
        assert_eq!(r.target_field, None, "0xFFFFFFFF means 'resolve the portal name'");
        assert_eq!(r.portal_name, "out00", "map 1's portal 4, from the WZ");
        assert_eq!(r.position, Some((1107, 365)), "y is exactly the portal's own y");

    }

    /// The real `0x0151` the owner's client sent when they clicked Heena on map 1, from
    /// `research/fixtures/npcs-visible-quests-clicked-world.log`. A parser tested against
    /// invented bytes proves only that it agrees with itself.
    ///
    /// This is also the test that pins the retraction: field 1 is a **quest id**, not the
    /// object id we assigned. Every map's first NPC gets object id 1000 and the client
    /// answered 1000/1002/1003/1005 for four NPCs in the same order across two sessions
    /// with opposite visit orders, so it cannot be reading our numbering back.
    #[test]
    fn a_clicked_npc_is_answered_with_something_to_say() {
        let body = hex("01e8030000010000000c046d0100000000");
        assert_eq!(body.len(), 17, "the capture is 17 bytes");
        let req = net::script::parse_quest_request(&body).expect("the captured body parses");
        assert_eq!(req.action, 1);
        assert_eq!(req.quest_id, 1000, "field 1 is a quest id, not our object id");
        assert_eq!(req.npc_template_id, 1, "field 2 is the template we sent in 0x044F");

        let (mut s, store, account_id, id) = session();
        store.create_migration(account_id, id, 0, 0).unwrap();
        s.claim_for_character(id);

        let replies = s.on_quest_request(&body);
        assert_eq!(replies.len(), 1, "an unanswered request freezes the client's whole UI");
        assert_eq!(replies[0].opcode, net::script::SCRIPT_MESSAGE);

        // The speaker must be the template the client named. It is by construction a real
        // Npc.wz id, which is what keeps this safe - 0 is not one.
        let said = &replies[0].body;
        assert_eq!(
            u32::from_le_bytes(said[5..9].try_into().unwrap()),
            req.npc_template_id
        );
        assert_ne!(req.npc_template_id, 0);

        // Type 0 is Say. Anything else indexes a different entry of the 71-entry table and
        // reads a different body, and there is no resync point.
        assert_eq!(said[10], net::script::SCRIPT_TYPE_SAY);

        // A short body must not panic - these come off a socket.
        for n in 0..body.len() {
            let _ = s.on_quest_request(&body[..n]);
        }
    }

    /// The real 12 bytes the owner's client sent when they clicked Robin on map 40, from
    /// `research/fixtures/dressed-in-world-npc-click-00f2-world.log`. Answering only
    /// `0x0151` left every quest-less NPC silent, which is what that run measured.
    #[test]
    fn clicking_a_questless_npc_is_answered_as_its_template_not_its_object_id() {
        let body = hex("e803000001001301ffffffff");
        let click = net::script::parse_npc_click(&body).expect("the captured body parses");
        assert_eq!(click.npc_object_id, 1000, "the object id WE assigned, [npc+0x190]");
        assert_eq!((click.char_x, click.char_y), (1, 275), "the CHARACTER's position");

        // Map 40's NPCs, exactly as gm-handbook/npcs.txt has them: object ids 1000 and 1001
        // for templates 8 and 9. The lookup has to invert the numbering config::load_npcs
        // does, and it is per-map because that numbering restarts on every field.
        let npcs = vec![
            net::opcode::FieldNpc {
                object_id: 1000, template_id: 8, x: 69, cy: 275, fh: 30,
                rx0: 19, rx1: 119, f: 0,
            },
            net::opcode::FieldNpc {
                object_id: 1001, template_id: 9, x: 1602, cy: 215, fh: 59,
                rx0: 1552, rx1: 1652, f: 0,
            },
        ];
        let config = Config {
            npcs: [(40u32, npcs)].into_iter().collect(),
            ..Config::default()
        };

        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "TestCharD".to_string(), map_id: 40, ..Default::default()
        };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.create_migration(account_id, id, 0, 0).unwrap();
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(id);

        let replies = s.on_npc_click(&body);
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].opcode, net::script::SCRIPT_MESSAGE);

        // The speaker must be the TEMPLATE (8), never the object id (1000). An object id in
        // that field does not fault - the loader result is null-checked - it just draws a
        // box with no portrait, which is the kind of failure a run cannot explain.
        let said = &replies[0].body;
        assert_eq!(u32::from_le_bytes(said[5..9].try_into().unwrap()), 8);
        assert_ne!(u32::from_le_bytes(said[5..9].try_into().unwrap()), 1000);
        assert_eq!(said[10], net::script::SCRIPT_TYPE_SAY);

        // An object id that is not on this map has no template to speak as. Answering with
        // a made-up one buys nothing, and this request does not block: the capture shows
        // the UI stayed live with 0x00F2 unanswered.
        let mut unknown = body.clone();
        unknown[0] = 0xFF;
        assert!(s.on_npc_click(&unknown).is_empty());

        // Short bodies come off a socket and must not panic.
        for n in 0..body.len() {
            let _ = s.on_npc_click(&body[..n]);
        }
    }

    /// `!map <id>` typed into the chat box, from the real captured chat body.
    #[test]
    fn the_gm_map_command_moves_the_character() {
        // The exact shape the client sends: u32 tick, u16 length, text, u8 tab.
        fn chat(text: &str) -> Vec<u8> {
            let mut b = vec![0u8; 4];
            b.extend_from_slice(&(text.len() as u16).to_le_bytes());
            b.extend_from_slice(text.as_bytes());
            b.push(3); // the All tab
            b
        }
        assert_eq!(net::opcode::parse_chat(&chat("Hello")).as_deref(), Some("Hello"));
        assert_eq!(net::opcode::parse_chat(&chat("!map 40")).as_deref(), Some("!map 40"));

        // The real 12 bytes the owner sent, so the parser is tested against the client and not
        // only against its own encoder.
        let real = [0xe7, 0x5b, 0x64, 0x05, 0x05, 0x00, b'H', b'e', b'l', b'l', b'o', 0x03];
        assert_eq!(net::opcode::parse_chat(&real).as_deref(), Some("Hello"));

        // Short bodies come off a socket and must not panic.
        for n in 0..6 {
            assert_eq!(net::opcode::parse_chat(&real[..n]), None, "{n} bytes");
        }
    }

    /// A map that does not exist must not move the character anywhere.
    ///
    /// Being stranded is the mild failure. `research/map1-exists.md` found that a map with no
    /// `String.wz` name entry sends the client down a branch containing a **non-returning**
    /// `E_POINTER` call, and that 12 ids are named-but-absent while 6 are present-but-unnamed
    /// - so "it has a name" is not the same question as "it has a field".
    #[test]
    fn the_map_command_refuses_a_map_that_does_not_exist() {
        fn chat(text: &str) -> Vec<u8> {
            let mut body = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
            body.extend_from_slice(&[0u8; 4]);
            body.extend_from_slice(&(text.len() as u16).to_le_bytes());
            body.extend_from_slice(text.as_bytes());
            body.push(3);
            body
        }

        let (mut s, _st, _id, _acct) = session();
        // With a field list loaded, only ids in it are allowed.
        let cfg = crate::config::Config {
            fields: [1u32, 10, 40].into_iter().collect(),
            ..(*s.config).clone()
        };
        s.config = std::sync::Arc::new(cfg);
        assert!(s.handle(&chat("!map 999999999")).is_empty(), "a nonexistent map moves nobody");
        assert!(s.handle(&chat("!map 0")).is_empty(), "0 is not a map");

        // And an EMPTY list must not refuse everything - that would fail closed on a missing
        // generated file rather than on a real problem.
        let cfg = crate::config::Config { fields: Default::default(), ..(*s.config).clone() };
        assert!(cfg.map_exists(999_999_999), "an unknown table allows, it does not refuse");
        assert!(cfg.map_exists(1));
    }

    /// Only `/map` with a number is a command; ordinary chat must stay ordinary.
    #[test]
    fn ordinary_chat_is_not_a_command() {
        let (mut s, _store, _id, _acct) = session();
        for text in ["Hello", "/map", "/map abc", "/mapabc 1", "map 40", "/warp 40"] {
            let mut b = vec![0u8; 4];
            b.extend_from_slice(&(text.len() as u16).to_le_bytes());
            b.extend_from_slice(text.as_bytes());
            b.push(3);
            let mut body = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
            body.extend_from_slice(&b);
            assert!(s.handle(&body).is_empty(), "{text:?} should not move anybody");
        }
    }

    /// The portal table is generated from the client's `Map.wz`, so the loader has to cope
    /// with what a generator emits: a comment header, blank lines, and a fourth column it
    /// does not use. Rows are the real ones for maps 1 and 10 - the exact two-way route that
    /// stranded a character on map 10 when the table was a hand-typed stub.
    #[test]
    fn the_portal_table_loads_and_is_keyed_on_the_source_map() {
        let dir = std::env::temp_dir().join("maplecw-portal-test");
        std::fs::create_dir_all(&dir).expect("a temp dir");
        let path = dir.join("portals.txt");
        std::fs::write(
            &path,
            "# map, index, portal, target map, target portal

             1, 0, sp, 0, 
1, 4, out00, 10, in00
             10, 0, sp, 0, 
10, 1, in00, 1, out00
10, 2, out00, 20, in00
             not, a, valid, row, here
",
        )
        .expect("write");

        let (links, index) = crate::config::Config::load_portals(&path);
        assert_eq!(
            links.get(&(1, "out00".to_string())),
            Some(&(10, "in00".to_string())),
            "map 1's out00 leads to map 10's in00"
        );
        assert_eq!(links.get(&(10, "in00".to_string())), Some(&(1, "out00".to_string())));
        assert_eq!(links.get(&(1, "in00".to_string())), None, "keyed on the SOURCE map");

        // The arrival lookup is the other direction, and spawn points must be in it even
        // though they lead nowhere - `sp` is a perfectly good place to arrive.
        assert_eq!(index.get(&(10, "in00".to_string())), Some(&1), "map 10's in00 is index 1");
        assert_eq!(index.get(&(1, "sp".to_string())), Some(&0), "spawns are indexed too");
        assert_eq!(index.get(&(1, "out00".to_string())), Some(&4));

        let (l, i) = crate::config::Config::load_portals(std::path::Path::new("no-such-file"));
        assert!(l.is_empty() && i.is_empty(), "a missing file is empty, not a panic");
    }

    /// The client omits BOTH coordinates when the portal name is empty, so the body is 31
    /// bytes and x/y are not at a fixed offset from the end.
    #[test]
    fn a_nameless_portal_request_has_no_coordinates() {
        let mut body = vec![0u8; 31];
        body[16..20].copy_from_slice(&10u32.to_le_bytes()); // an explicit target field
        // length prefix at 20 stays 0 -> empty name
        let r = parse_transfer_field(&body).expect("a 31-byte body parses");
        assert_eq!(r.target_field, Some(10));
        assert_eq!(r.portal_name, "");
        assert_eq!(r.position, None, "no coordinates when the name is empty");
    }

    /// Too short to hold the fixed part is None, not a panic - the body comes off a socket.
    #[test]
    fn a_truncated_portal_request_is_rejected_not_panicked_on() {
        for n in 0..20 {
            assert_eq!(parse_transfer_field(&vec![0u8; n]), None, "{n} bytes should not parse");
        }
    }

    /// The offset came from a real capture; this is that capture.
    #[test]
    fn the_character_id_is_read_out_of_a_real_0x007d_body() {
        let body = hex(
            "0000000000000000cc000000aabbccddeeffdeadbeef00000000764d0000230000\
             00020000007d29595a7929595a3fc073dd1e000000",
        );
        assert_eq!(migration_hello_character(&body), Some(204));
    }

    #[test]
    fn a_short_hello_yields_no_character_rather_than_panicking() {
        for len in 0..12 {
            migration_hello_character(&vec![0u8; len]);
        }
        assert_eq!(migration_hello_character(&[0u8; 11]), None);
        assert_eq!(migration_hello_character(&[0u8; 12]), Some(0));
    }

    fn hex(s: &str) -> Vec<u8> {
        let clean: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        (0..clean.len() / 2)
            .map(|i| u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16).unwrap())
            .collect()
    }
}
