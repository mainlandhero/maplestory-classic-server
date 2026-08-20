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
    /// The NPC conversation in progress, if any.
    conversation: Option<Conversation>,
    /// Where each NPC on the current field is in its idle chatter.
    chatter: Vec<Chatter>,
    /// Drives the chatter cadence. Seeded per session so two connections do not speak in
    /// lockstep, and seedable so a test can pin the sequence.
    rng: Xorshift,
    /// The last time [`Session::tick`] was called, in milliseconds since the connection
    /// opened.
    ///
    /// **A field entry needs to know the clock and does not get one**: `handle` takes bytes,
    /// not time. Without this, `reset_chatter` scheduled from zero, so an NPC on a map
    /// entered at t = 30 s came due at 3-9 s - already in the past - and the whole field
    /// spoke on the very next tick. It is at most one tick stale, which is 500 ms.
    clock_ms: u64,
    /// Every mob currently on this session's field, and how much HP it has left.
    ///
    /// **The server is the only thing that can move a mob's health bar.** A whole-`.text`
    /// scan found 11 stores to `mob+0x8b4` and only two inside the mob class - the
    /// constructor and the `0x03F0` handler - so the client never decrements a mob's HP on
    /// its own, however hard the player hits it. `research/mob-combat.md`.
    ///
    /// Keyed by object id and rebuilt on every field entry, because object ids are minted
    /// per field and a dead mob's id must never be reused.
    mob_hp: std::collections::HashMap<u32, u64>,
}

/// One NPC's place in its idle-chatter cycle.
///
/// **The ordering and the cadence are ours, because the client has neither.** Its own picker
/// is `rand() % n` twice with no cursor, so "in order" is a decision rather than a
/// reproduction - which is what the owner asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Chatter {
    object_id: u32,
    /// How many lines this NPC's `info/speak` group has. Zero means it never talks.
    lines: usize,
    /// The next line to send.
    cursor: usize,
    /// When it is next due, in milliseconds since the session's clock started.
    due_ms: u64,
}

/// The shortest an NPC waits between idle lines, in milliseconds.
///
/// **This is the client's own formula, not an invention.** `FUN_141e46d40` computes
/// `rand() % 6000 + 3000` for its idle timer, so three to nine seconds *is* this game's
/// cadence. **[L]** for the formula; that the unit is milliseconds is **[D]**, from the same
/// per-frame step decrementing a countdown loaded from a WZ `delay`.
///
/// The owner asked to match it rather than use a fixed interval. The lines still advance **in
/// order** - that part is ours, because the client's own picker is `rand() % n` with no
/// cursor - while the *timing* is the game's.
pub const CHATTER_MIN_MS: u64 = 3000;

/// The width of the random window above [`CHATTER_MIN_MS`]: the client's `rand() % 6000`.
pub const CHATTER_SPREAD_MS: u64 = 6000;

/// A tiny xorshift, so the cadence is random without `Session` reaching for a clock or a
/// global generator.
///
/// **Why not the `rand` crate.** `Session` is a pure state machine - bodies and time in,
/// bodies out - and that is what makes every exchange in this file a unit test rather than
/// something needing a live socket. A thread-local generator would put hidden state back in.
/// Seeding this from the session lets a test pin the exact sequence; a real generator is the
/// right call the moment something needs quality rather than variety.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Xorshift(u64);

impl Xorshift {
    fn next(&mut self) -> u64 {
        // xorshift64*, and the state must never be zero - it is a fixed point.
        let mut x = self.0 | 1;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A delay in the client's own window: 3000..=8999 ms.
    fn chatter_delay(&mut self) -> u64 {
        CHATTER_MIN_MS + self.next() % CHATTER_SPREAD_MS
    }
}

/// Where a conversation with an NPC currently is.
///
/// **This exists because `0x00F3` carries no line index.** The client answers a script box
/// with the box's own text echoed back and a single action byte, so which line the user was
/// on, and whether the box even had a Next button, are the server's to remember. Getting
/// that wrong is not a crash - `0x00F3` is not one of the latch setters - it is a
/// conversation that stops or repeats.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Conversation {
    npc_template: u32,
    /// `None` for a plain talk (`0x00F2`), which is a one-line conversation.
    quest_id: Option<u32>,
    /// The `Say` path being walked - `"0"`, then `"0.yes"` or `"0.no"` after a branch.
    path: String,
    /// The index of the line last sent.
    sent: usize,
    /// Whether the last box was a yes/no prompt. On those the answer byte is unambiguous.
    awaiting_yes_no: bool,
    /// Whether the last box was sent with `next` set. **The client collapses OK and Next
    /// into the same answer**, so this is the only thing that separates "advance" from
    /// "the user dismissed the last box".
    sent_with_next: bool,
}

impl Session {
    pub fn new(store: Arc<Store>, config: Arc<Config>) -> Self {
        // Any non-zero seed will do; the config's address is simply something that differs
        // between connections in the same process.
        let seed = Arc::as_ptr(&config) as u64 | 1;
        Session {
            store,
            config,
            claimed: None,
            conversation: None,
            chatter: Vec::new(),
            rng: Xorshift(seed),
            clock_ms: 0,
            mob_hp: std::collections::HashMap::new(),
        }
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

    /// What the server should send when nothing has arrived.
    ///
    /// **This is the only unsolicited path in the whole server**, and it exists because NPC
    /// idle chatter is server-triggered and client-rendered: the client holds the lines, the
    /// balloon art and a five-second display timer, but the only code that creates a balloon
    /// is reached from inbound `0x0453`. Nothing in the `0x044F` spawn body turns it on.
    ///
    /// `now_ms` is milliseconds since the connection started, and it is a parameter rather
    /// than a clock read so this stays a pure function of state and time - the same reason
    /// `Session` has no socket. Every exchange in this file is a unit test because of it.
    ///
    /// Returns at most one balloon per NPC per call. A tick that falls a long way behind
    /// does **not** burst: the next due time is computed from `now_ms`, not from the missed
    /// one, so a stalled connection resumes at the normal cadence instead of emitting a
    /// backlog the client would show as a flicker.
    pub fn tick(&mut self, now_ms: u64) -> Vec<Reply> {
        self.clock_ms = now_ms;
        if !self.config.set_field_probe || self.config.chatter_off {
            return Vec::new();
        }
        let mut out = Vec::new();
        for c in &mut self.chatter {
            if c.lines == 0 || now_ms < c.due_ms {
                continue;
            }
            let index = c.cursor % c.lines;
            c.cursor = c.cursor.wrapping_add(1);
            c.due_ms = now_ms + self.rng.chatter_delay();
            out.push(Reply {
                opcode: net::npcchat::NPC_CHAT,
                body: net::npcchat::npc_chat(
                    c.object_id,
                    net::npcchat::NPC_CHAT_NO_ANIMATION,
                    u8::try_from(index).unwrap_or(0),
                ),
                what: format!(
                    "NpcChat: object id {}, line {} of {} - idle chatter. The client holds \
                     the text; only the index goes on the wire.",
                    c.object_id,
                    index + 1,
                    c.lines
                ),
            });
        }
        out
    }

    /// Rebuild the idle-chatter cycle for the field the character has just entered.
    ///
    /// The NPC pool is destroyed and rebuilt on every field entry, so the cursors go with
    /// it - an object id from the previous map addresses nothing, or worse, something else.
    fn reset_chatter(&mut self, map: u32, now_ms: u64) {
        let empty: Vec<net::opcode::FieldNpc> = Vec::new();
        // The rng is moved out and back so the closure below can take it mutably while the
        // config is borrowed immutably.
        let mut rng = std::mem::replace(&mut self.rng, Xorshift(1));
        let chatter: Vec<Chatter> = self
            .config
            .npcs
            .get(&map)
            .unwrap_or(&empty)
            .iter()
            .map(|npc| Chatter {
                object_id: npc.object_id,
                lines: self
                    .config
                    .npc_strings
                    .get(&npc.template_id)
                    .map(|s| s.info.len())
                    .unwrap_or(0),
                cursor: 0,
                // Stagger by position on the field so they do not all speak at once.
                // The first line waits a full random interval too, so a field does not
                // erupt the moment it loads.
                due_ms: now_ms + rng.chatter_delay(),
            })
            .collect();
        self.rng = rng;
        self.chatter = chatter;
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
            op if net::combat::is_attack_opcode(op) => {
                return self.on_attack(body.get(2..).unwrap_or(&[]))
            }
            net::inventory::CLIENT_INVENTORY_MOVE => {
                return self.on_inventory_move(body.get(2..).unwrap_or(&[]))
            }
            net::mobmove::MOB_MOVE_REQUEST => return self.on_mob_move(body.get(2..).unwrap_or(&[])),
            net::notice::CLIENT_LOG_OUT => return self.on_log_out(),
            net::script::CLIENT_SCRIPT_REPLY => {
                return self.on_script_reply(body.get(2..).unwrap_or(&[]))
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
            Some(chr) => {
                let (quests, quest_note) = self.quest_book(chr.id);
                (
                net::opcode::set_field_with_character_dressed_quests(
                    &chr,
                    self.config.world_id,
                    self.clock_base(),
                    self.config.channel_id,
                    &self.dressed(&chr),
                    &quests,
                ),
                format!(
                    "SetField, characterData=1, presence[0] set so the character-stat block decodes, carrying map {} for character {} ({}). presence[0] is gate entry 7, settled in research/charrecord-presence-map.md; the map id sits at stat-block offset {}, settled in research/charstat-layout.md{}. Nothing here authenticates anybody.",
                    chr.map_id,
                    chr.id,
                    chr.name,
                    net::opcode::stat_block_map_id_at(chr.job),
                    quest_note,
                ),
            )
            }
            None => (
                net::opcode::set_field_minimal(self.clock_base(), self.config.channel_id),
                "SetField, characterData=1, MINIMAL record - the character could not be loaded, so this falls back to the all-flags-clear form. It is answered rather than dropped because an unanswered packet freezes the client's whole UI. It will NOT put the character on a map: with every presence flag clear the stat block never decodes, so there is no map id at all."
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
        // The NPC pool is destroyed and rebuilt on every field entry, so the chatter cursors
        // go with it: an object id from the previous map addresses nothing here, or worse,
        // addresses a different NPC.
        self.reset_chatter(chr.map_id, self.clock_ms);
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
        // A new field means new object ids. Anything remembered from the last one is stale
        // and, worse, could collide - so it goes.
        self.mob_hp.clear();
        for mob in chosen {
            self.mob_hp.insert(mob.object_id, mob.hp);
            out.push(Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(mob),
                what: format!(
                    "MobEnterField: template {} at ({}, {}) on foothold {}, object id {}, hp {} - {} bytes. The client cannot spawn this itself.",
                    mob.template_id, mob.x, mob.y, mob.fh, mob.object_id, mob.hp,
                    mob.body_len()
                ),
            });

            // **And then hand the mob to the client, which is what makes it move.**
            //
            // Spawning a mob does not animate it. The owner, 2026-08-19: six snails rendered on
            // map 40 and stood completely still. The server does not drive mob movement in
            // this game - it grants CONTROL of a mob to a client, and that client then runs
            // the wander and the idle animation locally and reports each path back as
            // `0x02FF`. Without this packet a mob is a picture.
            //
            // It explains the second symptom too. The combat agent decoded a real attack
            // from the same session: the owner at (473, 395), mob 2000 at (424, 395) - 49 pixels
            // away on the same ground line - and the attack carried **zero targets**. The
            // client would not aim at a mob nobody had given it. One packet, both symptoms.
            //
            // Order matters: `after` its MobEnterField, per research/mob-behaviour.md §3.
            // And the level must not be 0 - that DESPAWNS rather than releases, which is why
            // `mob_release_controller` exists under its own name.
            out.push(Reply {
                opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                body: net::mobmove::mob_change_controller(mob, net::mobmove::CONTROL_NORMAL),
                what: format!(
                    "MobChangeController: object id {} to this client, level {} - {} bytes. The client runs the mob's movement and reports it as 0x02FF.",
                    mob.object_id,
                    net::mobmove::CONTROL_NORMAL,
                    net::mobmove::change_controller_len(mob)
                ),
            });
        }

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
        // Which half of the quest's Say tree the action selects. With no quest state, a
        // start and an opening script both land on "0".
        let state = match req.action {
            net::script::QUEST_ACTION_COMPLETE | net::script::QUEST_ACTION_COMPLETE_SCRIPT => "1",
            _ => "0",
        };
        let quest = self.config.quests.get(&req.quest_id);

        // **The client runs the opening conversation itself, and re-sending it is a loop.**
        // The owner, 2026-08-19: *"Clicking 'Accept' starts the 'You must be the new traveler'
        // conversation again. That portion is incorrect, as the 'You must be the new
        // traveler' exists and gets handled on client side."* So `0x0151` is not "tell me
        // what this NPC says" - by the time it arrives the client has already shown the
        // opening and the user has pressed a button. Action **1** is that press, and what it
        // wants back is the **`yes` branch**: for quest 1000, "Thank you. #p2# is on the
        // hill to the east...".
        //
        // The decline branch is in the WZ too (`Say.0.no`) and the client very likely shows
        // it locally, the way it shows the opening - but no capture contains a decline, so
        // that is **[I]** and this does not act on it.
        let accepted = req.action == net::script::QUEST_ACTION_START;
        let branch = format!("{state}.yes");
        // An unknown quest falls back to the NPC's own line - a one-line conversation
        // rather than silence.
        let path = match quest {
            Some(q) if accepted && q.say.contains_key(&branch) => Some(branch),
            Some(q) if q.say.contains_key(state) => Some(state.to_string()),
            Some(q) if q.say.contains_key("0") => Some("0".to_string()),
            _ => None,
        };
        self.conversation = Some(Conversation {
            npc_template: req.npc_template_id,
            quest_id: path.as_ref().map(|_| req.quest_id),
            path: path.unwrap_or_default(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        self.say_line(0)
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

        // A quest-less NPC is a one-line conversation: its own `d0`. Going through the
        // same state machine means its OK is handled the way a quest's is, rather than
        // leaving a stale conversation behind for the next 0x00F3 to walk into.
        let _ = (click.npc_object_id, chr.map_id);
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: String::new(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        self.say_line(0)
    }

    /// Send the box for `index` of the conversation's current path, and remember what we
    /// sent so the answer can be interpreted.
    ///
    /// **The last line of a branchable conversation goes out as a yes/no prompt, not a Say**,
    /// and that is the whole reason the owner's Accept did nothing: a type-0 Say with `next = 0`
    /// draws `BtOK` and `BtClose`, so pressing it returns the same `action = 1` an OK does
    /// and the server has nothing to branch on. A type `0x10` box draws `BtQYes`/`BtQNo` and
    /// answers `1` for Yes and `0` for No, unambiguously. **[L]**, `research/script-reply.md`.
    fn say_line(&mut self, index: usize) -> Vec<Reply> {
        let Some(convo) = self.conversation.clone() else { return Vec::new() };
        let Some(lines) = self.say_lines(&convo) else {
            self.conversation = None;
            return Vec::new();
        };
        let Some(text) = lines.get(index).cloned() else {
            self.conversation = None;
            return Vec::new();
        };

        let last = index + 1 >= lines.len();
        // Only offer Accept/Decline while walking a state's own lines. On a branch the user
        // has already answered, and asking again is the loop the owner hit.
        let on_branch = convo.path.contains('.');
        let branches =
            last && !on_branch && convo.quest_id.is_some() && self.has_branch(&convo, "yes");
        let has_next = !last;

        let body = if branches {
            net::script::npc_ask(convo.npc_template, &text, true)
        } else {
            net::script::npc_say(convo.npc_template, &text, false, has_next)
        };
        let what = format!(
            "ScriptMessage {} from NPC template {}{}, line {} of {} on path \"{}\"",
            if branches { "yes/no prompt" } else { "Say" },
            convo.npc_template,
            convo.quest_id.map(|q| format!(" for quest {q}")).unwrap_or_default(),
            index + 1,
            lines.len(),
            convo.path,
        );

        if let Some(c) = self.conversation.as_mut() {
            c.sent = index;
            c.awaiting_yes_no = branches;
            c.sent_with_next = has_next;
        }
        vec![Reply { opcode: net::script::SCRIPT_MESSAGE, body, what }]
    }

    /// The lines of the path the conversation is currently on.
    fn say_lines(&self, convo: &Conversation) -> Option<Vec<String>> {
        match convo.quest_id {
            Some(q) => self.config.quests.get(&q)?.say.get(&convo.path).cloned(),
            // No quest: the NPC's own d0 line, as a one-line conversation.
            None => Some(vec![self.npc_line(convo.npc_template)]),
        }
    }

    /// Does the current path have a `yes` / `no` branch under it?
    fn has_branch(&self, convo: &Conversation, branch: &str) -> bool {
        let Some(q) = convo.quest_id.and_then(|q| self.config.quests.get(&q)) else {
            return false;
        };
        q.say.contains_key(&format!("{}.{}", convo.path, branch))
    }

    /// The client's answer to a script box.
    ///
    /// **An unanswered one costs a dead conversation and nothing else** - `0x00F3` is *not*
    /// one of the 37 functions that set `player->[0x2330]`, the one-request-outstanding
    /// latch, and the dialog is destroyed and the script-manager latch released before the
    /// packet is even built. So ending a conversation by sending nothing is safe, which is
    /// what this does whenever there is nothing left to say. **[L]**
    ///
    /// **The Say answer cannot distinguish OK from Next** - the client rewrites `BtOK` to
    /// the Next result at `142a59fb5`, so both arrive as `1`. The server therefore has to
    /// remember whether the box it sent had `next` set, and it does: `sent_with_next`. That
    /// is inference from our own state rather than something read off the wire, and it is
    /// worth knowing which of the two it is.
    fn on_script_reply(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(reply) = net::script::parse_script_reply(body) else { return Vec::new() };
        let Some(convo) = self.conversation.clone() else { return Vec::new() };

        if reply.action == net::script::SCRIPT_ACTION_CLOSED {
            self.conversation = None;
            return Vec::new();
        }

        if convo.awaiting_yes_no {
            let accepted = reply.action == net::script::SCRIPT_ACTION_YES;
            let branch = if accepted { "yes" } else { "no" };
            // **Record the acceptance before the branch-text check**, not after. A quest
            // whose `yes` path has no line in `Quest.wz` is still a quest the player just
            // accepted, and ordering these the other way would silently drop exactly those.
            let mut out = if accepted { self.accept_quest(&convo) } else { Vec::new() };
            if !self.has_branch(&convo, branch) {
                self.conversation = None;
                return out;
            }
            if let Some(c) = self.conversation.as_mut() {
                c.path = format!("{}.{}", convo.path, branch);
                c.sent = 0;
            }
            out.extend(self.say_line(0));
            return out;
        }

        if !convo.sent_with_next {
            self.conversation = None; // that was an OK on the last box
            return Vec::new();
        }
        self.say_line(convo.sent + 1)
    }

    /// The player pressed Yes on a quest's offer: write it down, and tell the journal.
    ///
    /// **This is the half that was missing.** The dialogue already worked and already
    /// answered the `yes` branch, so on screen the conversation looked complete - but
    /// nothing in this crate had ever called `store::start_quest`, and the quest journal is
    /// built entirely from the `quest_state` table. The owner, after the run of 2026-08-19:
    /// *"Quests don't work yet as expected."*
    ///
    /// `0x0089` sub-case 1 is the client's quest-record update. It is **not** a packet the
    /// client blocks on, so a lost one costs a stale journal rather than a frozen UI - and
    /// that is why a database failure here still sends the record. The player sees the quest
    /// they just accepted; the label says it will not survive a relog.
    ///
    /// **It must not travel with a `SetField`.** `FUN_142d59e20` returns immediately when
    /// `world+0x2358` - the character-data object - is null, and that field measures `0x00`
    /// on the *first* `SetField` of a session. This path only ever runs from a live
    /// conversation, which is long after that. `crates/net/src/quest.rs` carries the working.
    fn accept_quest(&mut self, convo: &Conversation) -> Vec<Reply> {
        let Some(quest_id) = convo.quest_id else { return Vec::new() };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let what = match self.store.start_quest(chr.id, quest_id) {
            Ok(true) => format!(
                "quest {quest_id} accepted from NPC {} by character {} ({}) and stored",
                convo.npc_template, chr.id, chr.name
            ),
            Ok(false) => format!(
                "quest {quest_id} was already started for character {}; the record is re-sent so the journal agrees",
                chr.id
            ),
            Err(e) => format!(
                "quest {quest_id} accepted but NOT STORED ({e}) - the journal will show it until the next relog and then lose it"
            ),
        };
        vec![Reply { opcode: net::quest::MESSAGE, body: net::quest::quest_accepted(quest_id), what }]
    }

    /// What an NPC should actually say when talked to.
    ///
    /// `String.wz/Npc.img` has the real lines - `d0`, `d1`, ... - and the server was sending
    /// placeholder text. Only `d0` is used: the second and later lines need the "next"
    /// button and a `0x00F3` answer to page through, and neither is built.
    ///
    /// **`#p8#` is sent unexpanded, on purpose.** It is a name substitution and whether the
    /// client resolves it is not established. Sending it raw makes the screen answer the
    /// question - "Hello! I'm Robin." and "Hello! I'm #p8#." are different on sight, and
    /// neither reading requires a guess. If the client does not expand it, the fix is here.
    ///
    /// An NPC with no entry keeps the placeholder, which is honest about the state of the
    /// server rather than silently saying nothing.
    fn npc_line(&self, template: u32) -> String {
        self.config
            .npc_strings
            .get(&template)
            .and_then(|s| s.dialogue.first())
            .cloned()
            .unwrap_or_else(|| {
                format!("This server has no dialogue for NPC template {template} yet.")
            })
    }

    /// The character's quest journal, as the record wants it.
    ///
    /// **This was built and then not connected for a day**, which on screen is
    /// indistinguishable from not existing: `crates/net/src/quest.rs`, the `quest_state`
    /// table and `set_field_with_character_dressed_quests` were all written, tested and
    /// never called. The owner, after the run of 2026-08-19: *"Quests don't work yet as
    /// expected."* That is what this is fixing.
    ///
    /// A database error yields an **empty** book rather than dropping the `SetField`. The
    /// record has no length prefix and no resync point, so the only two safe answers are a
    /// correct book and no book at all; an empty one is the second, and it costs a blank
    /// quest journal instead of a frozen client. The reason travels in the reply's label.
    fn quest_book(&self, character_id: u32) -> (net::quest::QuestBook, String) {
        match self.store.quest_book(character_id) {
            Ok(book) => {
                let note = format!(
                    ", quests: {} started / {} completed",
                    book.started.len(),
                    book.completed.len()
                );
                (book, note)
            }
            Err(e) => (
                net::quest::QuestBook::default(),
                format!(", quests: EMPTY BOOK - could not read quest_state ({e})"),
            ),
        }
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
            .map(|&(slot, item_id)| (slot, item_id, self.template_stats(item_id)))
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
        // **Dressed, exactly like the migration's SetField.** This sent the bare form until
        // 2026-08-19, which is why the owner's items had their stats on entering the world and
        // lost them the moment they used a portal or `!map`: every record after the first
        // carried EquipStats::default(), all zeros. It also explains the whole "the tooltip
        // reads a different object" investigation - they had reached map 40 with `!map`, so
        // the record they were hovering really did contain zeros. There was never a second
        // object.
        let dressed = self.dressed(chr);
        let (quests, quest_note) = self.quest_book(chr.id);
        vec![Reply {
            opcode: net::opcode::SET_FIELD,
            body: net::opcode::set_field_with_character_dressed_quests(
                chr,
                self.config.world_id,
                self.clock_base(),
                self.config.channel_id,
                &dressed,
                &quests,
            ),
            what: format!(
                "SetField, {why}, for character {} ({}){warn}{quest_note}",
                chr.id, chr.name
            ),
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

        // **Anything that is not a command is said out loud.** The client draws nothing for
        // its own chat: typing sends `0x00E7` and stops. The owner, 2026-08-19, typed "Hello",
        // "Hello2" and "Hello3" and saw nothing at all, because this function matched them
        // against `!map`, found nothing, and returned an empty reply. The balloon and the
        // chat-log line both come from `0x0231` coming back - see net::userchat.
        if !text.starts_with('!') {
            return self.say_out_loud(text);
        }
        let Some(rest) = text.strip_prefix("!map ") else {
            return self.notice(format!("{text}: not a command. Try !map <id>."));
        };
        let rest = rest.trim();
        let Ok(map) = rest.parse::<u32>() else {
            return self.notice(format!("!map: \"{rest}\" is not a map id."));
        };

        // Refuse a map the client cannot load. A character sent to an id with no field image
        // is stranded with no way back except another command. The owner asked for the refusal to
        // say so on screen rather than only in the log, which needed the outbound chat line
        // this now sends - see net::notice::CHAT_NOTICE.
        if !self.config.map_exists(map) {
            return self.notice(format!(
                "!map: {map} has no field image in this client, so it would strand you."
            ));
        }
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        // Portal 0 is the map's spawn point, which is where a GM warp should land.
        self.go_to_map(&mut chr, map, 0, format!("GM !map {map}"))
    }

    /// Say something as the player: a balloon over the head and a line in the chat log.
    ///
    /// **An empty message is dropped rather than sent.** The client's own box will not
    /// submit one, so an empty `0x00E7` means something else is going on, and a balloon
    /// with no text is a worse answer than none.
    ///
    /// This is a **local echo, not a broadcast**: it goes back to the one connection that
    /// spoke. There is nobody else on the field to send it to yet - the server has no
    /// concept of a second player in a field - and saying so here is cheaper than
    /// rediscovering it when there is.
    fn say_out_loud(&mut self, text: &str) -> Vec<Reply> {
        if text.is_empty() {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        vec![Reply {
            opcode: net::userchat::USER_CHAT,
            body: net::userchat::user_chat(chr.id, text),
            what: format!("UserChat: {} ({}) says {:?}", chr.id, chr.name, text),
        }]
    }

    /// One line in the client's chat window.
    ///
    /// `force = 1` is not optional: with `0` the client shows only the first line after each
    /// field entry and silently drops the rest, which reads exactly like the feature being
    /// broken. See `net::notice::CHAT_NOTICE`.
    fn notice(&self, text: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::notice::CHAT_NOTICE,
            body: net::notice::chat_notice(&text),
            what: format!("ChatNotice: {text}"),
        }]
    }

    /// The client reporting where a mob it controls has moved to. **This must be answered.**
    ///
    /// # A retraction, and the reason it happened is worth more than the fix
    ///
    /// This handler returned nothing for one run, on the strength of
    /// `research/mob-behaviour.md` §6: a scan for `mob+0x2f4` found 20 sites and *"not one of
    /// them is inside any of the eight mob-pool packet handlers"*. Re-running that scan gives
    /// the **identical 20 sites** - it was never wrong. What was wrong is the set it was
    /// intersected against: there are **110** mob-pool handlers, not eight, and §2.2 of that
    /// same document had already said so. One of the 20 is `141c821f8` inside
    /// `FUN_141c82060`, whose only caller is the stub for **`0x03E4`**.
    ///
    /// Enumerate before you filter, failed twice inside one file. On screen it looked like
    /// mobs moving for half a second and then freezing forever.
    ///
    /// # What the answer does
    ///
    /// `0x03E4` **MobCtrlAck** de-obfuscates the client's own move counter from
    /// `mob+0x2f0`/`+0x2f4` - the pair the sender incremented - compares it against the
    /// `move_id` we echo (`141c82212 CMP EAX,ECX / JNS`, so `ack >= current` passes), and
    /// re-runs slot 8 `FUN_141c54200`. That slot no-ops when the animation is already running
    /// (`141c54248 JNE ret`), which is what makes it a pump rather than an initialiser.
    ///
    /// # The broadcast half, which has no recipient yet
    ///
    /// `0x03D9` is the *rebroadcast to every other client on the field* - the owner's point that a
    /// second player must see the same movement. `net::mobmove::mob_move_broadcast` builds it
    /// and is tested, but this server has no field-occupancy registry: a `Session` is one
    /// connection and knows of no other. **It is deliberately not sent to the mover** - that
    /// would be a different packet than the one they are owed. Wiring it needs the player
    /// list that `config::spawn_capacity`'s `players_here = 1` is also waiting on.
    fn on_mob_move(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(req) = net::mobmove::parse_mob_move(payload) else {
            return Vec::new();
        };
        vec![Reply {
            opcode: net::mobmove::MOB_CTRL_ACK,
            body: net::mobmove::mob_ctrl_ack(req.object_id, req.move_id, false),
            what: format!(
                "MobCtrlAck: mob {} move {} acknowledged. Without this the client runs one \
                 simulation step and stops - measured twice, 30 grants and 30 reports all \
                 with moveId 1, then silence.",
                req.object_id, req.move_id
            ),
        }]
    }

    /// Move an item, and **write it down**.
    ///
    /// **Answering this is not optional.** `FUN_142cc5b00` sets `player->[0x2330]` to 1 the
    /// moment it sends `0x0107`, and its own gate 2 at `142cc5b5d` refuses every later
    /// request while that latch is set. Only an inbound handler clears it, and for this
    /// packet that means the `bExclRequestSent` byte of our `0x0070`. So an unanswered
    /// `0x0107` does not fail one drag - it silently kills every inventory action for the
    /// rest of the session. Same class as Log Out and `world->[0x33f4]`.
    ///
    /// **Every path here answers, including every refusal.** A refusal is
    /// [`net::inventory::inventory_rejected`], which moves nothing and still clears the
    /// latch; answering with a chat notice and no `0x0070` is what killed the whole
    /// inventory UI on 2026-08-19.
    ///
    /// # This is goal I
    ///
    /// Until 2026-08-19 the unequip moved an item on screen and nowhere else: the record's
    /// equipped list is built from the `equipment` rows, so the next `SetField` - a portal, a
    /// `!map`, a relog - put the item straight back on. The owner: *"items taken off should
    /// persist as is during transitions from map to map."*
    ///
    /// The store now has somewhere to put it. `Store::unequip_to_bag` deletes the
    /// `equipment` row and inserts the `inventory` row **in one transaction**, so there is no
    /// instant in which the item is in both places or neither - which was the whole reason
    /// the old code refused to touch the database at all. Per-item stats travel with it, so
    /// a scrolled item does not come back flattened.
    fn on_inventory_move(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(m) = net::inventory::parse_inventory_move(payload) else {
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else {
            return self.inventory_refused(&m, "no character is claimed on this connection");
        };

        if let Some(equip_slot) = m.equipped_slot() {
            let dst = u16::try_from(m.dst).ok();
            return match self.store.unequip_to_bag(chr.id, equip_slot, dst) {
                Ok(row) => self.inventory_moved(
                    &m,
                    format!(
                        "unequipped slot {equip_slot} into Equip bag slot {} - item {}, STORED, so the next SetField will not re-dress it",
                        row.slot, row.item.item_id
                    ),
                ),
                Err(e) => self.inventory_refused(&m, &format!("unequip refused: {e}")),
            };
        }

        // An equip: out of a bag slot, into a negative worn slot.
        if m.inv_type == net::inventory::INV_EQUIP && m.src > 0 && m.dst < 0 {
            let Ok(worn) = u8::try_from(-i32::from(m.dst)) else {
                return self.inventory_refused(&m, "the worn slot does not fit in a u8");
            };
            let Ok(src) = u16::try_from(m.src) else {
                return self.inventory_refused(&m, "the bag slot does not fit in a u16");
            };
            return match self.store.equip_from_bag(chr.id, src, worn) {
                Ok(item_id) => self.inventory_moved(
                    &m,
                    format!("equipped item {item_id} from Equip bag slot {src} into slot {worn}"),
                ),
                Err(e) => self.inventory_refused(&m, &format!("equip refused: {e}")),
            };
        }

        // Bag to bag. The client sends -1 for `count` when the item is not a bundle.
        let Ok(inv) = store::InventoryType::from_wire(i16::from(m.inv_type)) else {
            return self.inventory_refused(&m, &format!("invType {} is not a bag", m.inv_type));
        };
        let (Ok(src), Ok(dst)) = (u16::try_from(m.src), u16::try_from(m.dst)) else {
            return self.inventory_refused(&m, "a bag-to-bag move needs two positive slots");
        };
        let count = (m.count >= 0).then(|| m.count as u16);
        let max_stack = self.max_stack(&chr, inv, src);
        match self.store.move_item(chr.id, inv, src, dst, count, max_stack) {
            Ok(outcome) => self.inventory_moved(&m, format!("{inv:?} bag: {outcome:?}")),
            Err(e) => self.inventory_refused(&m, &format!("move refused: {e}")),
        }
    }

    /// How many of the item in `slot` fit in one stack, from `info/slotMax`.
    ///
    /// **`0` and `1` both mean "does not stack"**, and `0` is what every equip has because
    /// the property is simply absent - 290 of 2785 items carry one. An unknown item also
    /// lands here, and treating it as non-stacking is the safe direction: the worst case is
    /// a merge that does not happen, against a merge that silently destroys the overflow.
    fn max_stack(&self, chr: &net::opcode::Character, inv: store::InventoryType, slot: u16) -> u16 {
        let Ok(items) = self.store.bag_items(chr.id, inv) else { return 1 };
        let Some(row) = items.iter().find(|i| i.slot == slot) else { return 1 };
        self.config
            .shops
            .item_data
            .get(&row.item.item_id)
            .map(|d| d.slot_max.max(1))
            .unwrap_or(1)
    }

    /// The `0x0070` that says a move happened.
    fn inventory_moved(&self, m: &net::inventory::InventoryMove, why: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_move_result(m.inv_type, m.src, m.dst),
            what: format!(
                "InventoryOperation: move invType {} slot {} -> {}. {why}. The first byte is 1, which clears the client's +0x2330 request latch; a 0 there would block every later inventory action.",
                m.inv_type, m.src, m.dst
            ),
        }]
    }

    /// The `0x0070` that says nothing happened - and it is **still a reply**.
    ///
    /// `nCount` is 0, so the client skips the entry loop entirely and moves no item, but
    /// `bExclRequestSent` is 1 and that is what unlocks the UI. A refusal that sends nothing
    /// is not a refusal; it is a dead inventory for the rest of the session.
    fn inventory_refused(&self, m: &net::inventory::InventoryMove, why: &str) -> Vec<Reply> {
        vec![Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_rejected(),
            what: format!(
                "InventoryOperation: REFUSING invType {} slot {} -> {} with nCount 0 - {why}. Nothing moves, but bExclRequestSent = 1 clears the +0x2330 latch.",
                m.inv_type, m.src, m.dst
            ),
        }]
    }

    /// The player swung at something.
    ///
    /// **The client has already worked out the damage.** Each target block in `0x00DF` carries
    /// the numbers it intends to show, so nothing is sent back to make them appear - what the
    /// server owes is the *consequence*: the health bar, and the death.
    ///
    /// Two things this deliberately does not do yet:
    ///
    /// * **No EXP.** `0x007C` bit 16 carries it and `net::combat::stat_changed` builds it, but
    ///   `Character` has no `exp` field and nothing persists one, so crediting a kill would
    ///   mean inventing a number that vanishes at the next login.
    /// * **No drops.** Nothing decodes the drop pool yet - `STATUS.md` goal C.
    ///
    /// **A miss is not an error.** An attack with no targets is exactly what the client sends
    /// when it swings at empty air, and on 2026-08-19 it was also - wrongly - reported as
    /// proof that the client would not target our mobs at all. That claim came from combining
    /// two different sessions and is retracted; `research/mob-combat.md` §17.
    fn on_attack(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Ok(attack) = net::combat::parse_attack(payload) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for target in &attack.targets {
            let Some(hp_before) = self.mob_hp.get(&target.object_id).copied() else {
                continue; // not a mob of ours, or already dead and removed
            };
            let hit = net::combat::apply_damage(hp_before, target.total_damage());
            if hit.died {
                // The id must never come back. A later hit on a corpse finds nothing here
                // and is ignored, which is what mob_hit_replies expects.
                self.mob_hp.remove(&target.object_id);
            } else {
                self.mob_hp.insert(target.object_id, hit.hp_after);
            }
            for (opcode, body) in net::combat::mob_hit_replies(target.object_id, &hit) {
                out.push(Reply {
                    opcode,
                    body,
                    what: format!(
                        "mob {} took {} ({} -> {}){}",
                        target.object_id,
                        hit.damage_applied,
                        hit.hp_before,
                        hit.hp_after,
                        if hit.died { " - DEAD, leaving the field" } else { "" }
                    ),
                });
            }
        }
        out
    }

    /// Answer Log Out, and **this is not optional in the way most replies are**.
    ///
    /// `0x01BE`'s builder sets `world->[0x33f4] = 1`, and the `SetField` handler
    /// `FUN_142097f80` tests that byte immediately after reading its 8-byte FILETIME and
    /// returns to its epilogue if it is set. **Only `0x0106` clears it.** So an unanswered
    /// Log Out does not just leave the player stuck on the field - it makes **every
    /// subsequent `SetField` vanish in silence**: no dialog, no fault, nothing in any log.
    /// Same failure class as the `player->[0x2330]` latch in `research/npc-click.md`, far
    /// worse blast radius.
    ///
    /// The message must not be empty; an empty string is a no-op in the client.
    ///
    /// **The teardown is in place rather than a reconnect** - the handler builds no
    /// `sockaddr` and constructs a login stage directly. Whether the client also closes this
    /// socket is not settled, and the next run answers it for free: watch whether the
    /// following packet lands in `login.log` or `world.log`.
    fn on_log_out(&mut self) -> Vec<Reply> {
        // The conversation and the field's chatter belong to a session that is ending.
        self.conversation = None;
        self.chatter.clear();
        vec![Reply {
            opcode: net::notice::LOG_OUT_RESULT,
            body: net::notice::log_out_result("Returning to the login screen."),
            what: "LogOutResult - and answering this is what clears world->[0x33f4]. Until it is cleared the client silently drops every SetField."
                .to_string(),
        }]
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
                what: "transfer-field request, but the character could not be loaded -                        answered with the minimal record rather than dropped, because an unanswered packet freezes the client's whole UI."
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
                        "portal {:?} on map {} is NOT in the portal table, so this re-sends the current map rather than guessing a destination",
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
        let mut chr = self
            .store
            .characters_for(claimed.account_id, claimed.world_id)
            .ok()?
            .into_iter()
            .find(|c| c.id == claimed.character_id)?;
        // The bag override, applied here rather than at either SetField site so a portal
        // walk and a migration cannot disagree about it. See Config::inventory_slots.
        if let Some(slots) = self.config.inventory_slots {
            chr.inventory_slots = [slots; net::opcode::INVENTORY_COUNT];
        }
        // **The Equip tab's contents, and this is goal I.** Loaded on the same funnel as the
        // slot counts for the same reason: both `SetField` sites go through here, and the
        // one thing that must never differ between a migration and a portal walk is what the
        // character is carrying. Before this, an unequip moved an item on screen and nowhere
        // else, so the next field entry re-dressed from `equipment` rows that had not
        // changed and the item came back on.
        //
        // A read failure yields an EMPTY bag rather than dropping the character. The record
        // has no length prefix and no resync point, so an empty Equip tab costs a bag that
        // looks empty for one field entry; no character at all costs the minimal record and
        // a player who is nowhere.
        chr.equip_bag = match self.store.bag(chr.id) {
            Ok(bag) => bag
                .items_in(store::InventoryType::Equip)
                .filter_map(|row| {
                    let store::ItemKind::Equip(stored) = row.item.kind else {
                        return None; // a bundle in the Equip tab is not representable
                    };
                    Some(net::bag::BagEquip {
                        pos: row.slot,
                        item_id: row.item.item_id,
                        // None means "derive from the template", which is what every row
                        // written before per-item stats existed says.
                        stats: stored.unwrap_or_else(|| self.template_stats(row.item.item_id)),
                    })
                })
                .collect(),
            Err(_) => Vec::new(),
        };
        Some(chr)
    }

    /// The stats an item's `Character.wz` template gives it, or all-zero if it has none.
    ///
    /// Split out of [`Session::dressed`] so a worn item and a bagged one cannot disagree
    /// about what the same item id is worth.
    fn template_stats(&self, item_id: u32) -> net::opcode::EquipStats {
        self.config.equips.get(&item_id).map(|t| t.fresh_stats()).unwrap_or_default()
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

    /// A session with the migration already claimed, which is what every quest test needs.
    fn claimed_session() -> (Session, Arc<Store>, u32) {
        let (mut s, store, account_id, id) = session();
        store.create_migration(account_id, id, 0, 0).unwrap();
        let note = s.claim_for_character(id);
        assert!(note.contains("claimed the migration"), "{note}");
        (s, store, id)
    }

    /// The `0x00F3` body: u32 handle, u8 messageType, u32 echo, a u16-prefixed string, u8
    /// action. Built here rather than hand-hexed so a change to the parser breaks this too.
    fn script_reply(action: i8) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u32.to_le_bytes()); // handle
        b.push(0); //                                messageType
        b.extend_from_slice(&0u32.to_le_bytes()); // echo
        b.extend_from_slice(&0u16.to_le_bytes()); // empty text
        b.push(action as u8);
        b
    }

    /// Pressing Yes writes the quest down. **This is the half that was missing**: the
    /// dialogue already answered the `yes` branch, so the conversation looked finished on
    /// screen while nothing had ever called `start_quest`.
    #[test]
    fn saying_yes_to_a_quest_stores_it_and_sends_the_quest_record() {
        let (mut s, store, id) = claimed_session();
        let convo = Conversation {
            npc_template: 2100,
            quest_id: Some(1000),
            path: "0".to_string(),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        };

        let out = s.accept_quest(&convo);
        assert_eq!(out.len(), 1, "one quest record");
        assert_eq!(out[0].opcode, net::quest::MESSAGE);
        assert_eq!(out[0].body, net::quest::quest_accepted(1000));
        assert!(out[0].what.contains("and stored"), "{}", out[0].what);

        let rows = store.quest_rows(id).unwrap();
        assert_eq!(rows.len(), 1, "the quest is in quest_state");
        assert_eq!(rows[0].quest_id, 1000);
        assert_eq!(rows[0].state, store::QuestState::InProgress);
    }

    /// The ordering decision, pinned. `Config::default()` has no quest text at all, so
    /// `has_branch` is false and the conversation ends immediately - and the acceptance must
    /// still have been recorded, because a quest whose `yes` path has no line is still a
    /// quest the player accepted.
    #[test]
    fn a_yes_with_no_branch_text_still_records_the_acceptance() {
        let (mut s, store, id) = claimed_session();
        s.conversation = Some(Conversation {
            npc_template: 2100,
            quest_id: Some(1000),
            path: "0".to_string(),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        });
        assert!(s.config.quests.is_empty(), "this test is about the no-branch-text case");

        let out = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_YES));
        assert_eq!(out.len(), 1, "the quest record, and no dialogue line");
        assert_eq!(out[0].opcode, net::quest::MESSAGE);
        assert_eq!(store.quest_rows(id).unwrap().len(), 1, "recorded despite the empty branch");
        assert!(s.conversation.is_none(), "the conversation is over");
    }

    /// No means no: nothing is written and nothing is sent.
    #[test]
    fn saying_no_to_a_quest_records_nothing() {
        let (mut s, store, id) = claimed_session();
        s.conversation = Some(Conversation {
            npc_template: 2100,
            quest_id: Some(1000),
            path: "0".to_string(),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        });

        assert!(s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_NO)).is_empty());
        assert!(store.quest_rows(id).unwrap().is_empty());
    }

    /// The other half: an accepted quest has to come back on the next field entry, or the
    /// journal is empty every time the player walks through a portal. This asserts the
    /// **bytes** in the record, not that a function was called.
    #[test]
    fn the_field_entry_setfield_carries_the_quest_journal() {
        let (mut s, store, id) = claimed_session();
        store.start_quest(id, 1000).unwrap();

        let mut chr = s.claimed_character().expect("the claim resolves to a character");
        let replies = s.go_to_map(&mut chr, 40, 0, "a test portal walk".to_string());
        let sf = replies
            .iter()
            .find(|r| r.opcode == net::opcode::SET_FIELD)
            .expect("a field entry always sends a SetField");

        let expected = net::quest::started_quest_block(&[net::quest::StartedQuest {
            quest_id: 1000,
            progress: String::new(),
        }]);
        assert!(
            sf.body.windows(expected.len()).any(|w| w == expected.as_slice()),
            "the started-quest block is not in the record"
        );
        assert!(sf.what.contains("1 started / 0 completed"), "{}", sf.what);
    }

    /// A character with no quests still sends both blocks. They are three bytes each and the
    /// record has no length prefix, so "send nothing when there is nothing" desynchronises
    /// everything after it.
    #[test]
    fn an_empty_journal_still_costs_two_blocks() {
        let (s, _store, _id) = claimed_session();
        let (book, note) = s.quest_book(s.claimed().unwrap().character_id);
        assert!(book.started.is_empty());
        assert!(book.completed.is_empty());
        assert!(note.contains("0 started / 0 completed"), "{note}");
        assert_eq!(book.started_block().len(), net::quest::EMPTY_QUEST_BLOCK_LEN);
        assert_eq!(book.completed_block().len(), net::quest::EMPTY_QUEST_BLOCK_LEN);
    }

    /// A session whose character is wearing four items, which is what a real one wears.
    fn dressed_session() -> (Session, Arc<Store>, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Wanderer".to_string(),
            equips: vec![(5, 1040002), (6, 1060002), (7, 1072001), (11, 1302000)],
            ..Default::default()
        };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.create_migration(account_id, id, 0, 0).unwrap();
        let mut s = Session::new(store.clone(), Arc::new(Config::default()));
        assert!(s.claim_for_character(id).contains("claimed the migration"));
        (s, store, id)
    }

    /// The `0x0107` body: u32 tick, i8 invType, i16 src, i16 dst, i16 count.
    fn inventory_move(inv_type: i8, src: i16, dst: i16, count: i16) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(inv_type as u8);
        b.extend_from_slice(&src.to_le_bytes());
        b.extend_from_slice(&dst.to_le_bytes());
        b.extend_from_slice(&count.to_le_bytes());
        b
    }

    /// **Goal I, end to end.** The owner: *"items taken off should persist as is during
    /// transitions from map to map."*
    ///
    /// Take the hat off, then walk a portal, and read the record that comes back: the hat
    /// must be gone from the equipped list and present in the Equip tab. Before the store
    /// had somewhere to put it this test could not be written - the unequip touched no row,
    /// so the next `SetField` re-dressed the character and the item came back on.
    #[test]
    fn an_unequipped_item_is_still_off_after_a_map_change() {
        let (mut s, store, id) = dressed_session();

        let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -5, 1, -1));
        assert_eq!(out.len(), 1, "exactly one reply, and it is never zero");
        assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION);
        assert_eq!(out[0].body[0], 1, "bExclRequestSent - without it the UI locks up");
        assert!(out[0].what.contains("STORED"), "{}", out[0].what);

        // The database, not the reply.
        let worn: Vec<u8> = store.equipped_items(id).unwrap().iter().map(|e| e.slot).collect();
        assert_eq!(worn, vec![6, 7, 11], "slot 5 is off");
        let bagged: Vec<(u16, u32)> = store
            .bag(id)
            .unwrap()
            .items_in(store::InventoryType::Equip)
            .map(|i| (i.slot, i.item.item_id))
            .collect();
        assert_eq!(bagged, vec![(1, 1040002)], "and it is in the Equip tab, slot 1");

        // The wire, on the next field entry.
        let mut chr = s.claimed_character().expect("the claim resolves");
        assert!(!chr.equips.iter().any(|&(slot, _)| slot == 5), "not worn any more");
        assert_eq!(chr.equip_bag.len(), 1, "and the record will carry it");

        let replies = s.go_to_map(&mut chr, 40, 0, "a test portal walk".to_string());
        let sf = replies
            .iter()
            .find(|r| r.opcode == net::opcode::SET_FIELD)
            .expect("a field entry always sends a SetField");

        let expected = net::bag::equipped_tail(
            &[net::bag::BagEquip::plain(1, 1040002)],
            net::opcode::DEFAULT_INVENTORY_SLOTS,
        );
        assert!(
            sf.body.windows(expected.len()).any(|w| w == expected.as_slice()),
            "the Equip tab's list is not in the record"
        );
        // And it costs what one bag equip costs, rather than merely appearing somewhere. A
        // 125-byte item body is mostly zeros, so "the ten empty bytes are absent" is NOT a
        // usable check - the pattern occurs inside any item. The length delta is exact.
        let mut bare = chr.clone();
        bare.equip_bag.clear();
        let without = s.go_to_map(&mut bare, 40, 0, "the same walk, empty bag".to_string());
        let without = &without
            .iter()
            .find(|r| r.opcode == net::opcode::SET_FIELD)
            .expect("a SetField")
            .body;
        assert_eq!(
            sf.body.len() - without.len(),
            2 + net::opcode::EQUIPPED_ITEM_LEN,
            "one u16 position plus one type-1 item body"
        );
    }

    /// Putting it back on is the same transaction in reverse, and it also persists.
    #[test]
    fn equipping_from_the_bag_moves_the_row_back() {
        let (mut s, store, id) = dressed_session();
        s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -5, 1, -1));

        let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, -5, -1));
        assert_eq!(out[0].body[0], 1, "still answered");
        assert!(out[0].what.contains("equipped item 1040002"), "{}", out[0].what);

        let worn: Vec<u8> = store.equipped_items(id).unwrap().iter().map(|e| e.slot).collect();
        assert_eq!(worn, vec![5, 6, 7, 11], "back on");
        assert_eq!(store.bag(id).unwrap().items.len(), 0, "and out of the bag");
    }

    /// A move the store refuses is still answered, and with the byte that unlocks the UI.
    ///
    /// **This is the failure that cost a whole session on 2026-08-19**: a refusal sent as a
    /// chat notice left `player+0x2330` latched, and every later inventory action was dropped
    /// by the client before it was built.
    #[test]
    fn a_refused_move_still_clears_the_request_latch() {
        let (mut s, store, id) = dressed_session();

        // Slot 9 is empty, so this cannot succeed.
        let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -9, 1, -1));
        assert_eq!(out.len(), 1, "a refusal is a packet");
        assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION);
        assert_eq!(out[0].body, net::inventory::inventory_rejected());
        assert_eq!(out[0].body[0], 1, "bExclRequestSent, even on a refusal");
        assert!(out[0].what.contains("REFUSING"), "{}", out[0].what);

        assert_eq!(store.equipped_items(id).unwrap().len(), 4, "nothing moved");
        assert!(store.bag(id).unwrap().is_empty());
    }

    /// A connection with no claimed migration is answered too, rather than dropped.
    #[test]
    fn an_unclaimed_connection_is_refused_rather_than_ignored() {
        let (mut s, _, _, _) = session();
        let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -5, 1, -1));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].body, net::inventory::inventory_rejected());
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

    /// Accepting a quest answers with the **yes branch**, not the opening again.
    ///
    /// The owner, 2026-08-19: *"Clicking 'Accept' starts the 'You must be the new traveler'
    /// conversation again. That portion is incorrect, as [it] exists and gets handled on
    /// client side."* By the time `0x0151` arrives the client has already shown the opening
    /// and the user has pressed a button; action 1 is that press.
    #[test]
    fn accepting_a_quest_answers_with_the_yes_branch() {
        let path = std::path::Path::new("../../gm-handbook/questlines.txt");
        if !path.exists() {
            return; // generated data, gitignored
        }
        let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
        let want = config.quests[&1000].say["0.yes"][0].clone();
        assert!(want.contains("hill to the east"), "{want}");

        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.create_migration(account_id, id, 0, 0).unwrap();
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(id);

        // The real 0x0151 the owner's client sent on pressing Accept: action 1, quest 1000.
        let replies = s.on_quest_request(&hex("01e8030000010000000c046d0100000000"));
        assert_eq!(replies.len(), 1);
        let (message_type, text, _) = script_text(&replies[0].body);
        assert_eq!(text, want, "Accept must answer with the yes branch");

        // And it must be a plain Say, NOT another Accept/Decline prompt - the user has
        // already answered, and asking again is the loop the owner hit.
        assert_eq!(message_type, net::script::SCRIPT_TYPE_SAY);

        // The branch is one line, so OK ends the conversation rather than repeating it.
        let done = s.on_script_reply(&reply_bytes(&text, message_type, 1));
        assert!(done.is_empty(), "the conversation must end, not loop");
    }

    /// A multi-line branch still pages, so the machine is not special-cased to one line.
    #[test]
    fn a_multi_line_path_still_pages_in_order() {
        let mut quests = std::collections::HashMap::new();
        quests.insert(
            42u32,
            crate::config::Quest {
                name: "Test".into(),
                say: [("0.yes".to_string(), vec!["one".to_string(), "two".to_string()])]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
        );
        let config = Config { quests, ..Config::default() };
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.create_migration(account_id, id, 0, 0).unwrap();
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(id);

        let mut body = vec![1u8]; // action 1, accept
        body.extend_from_slice(&42u32.to_le_bytes());
        body.extend_from_slice(&1u32.to_le_bytes());
        let replies = s.on_quest_request(&body);
        let (ty, text, after) = script_text(&replies[0].body);
        assert_eq!(text, "one");
        assert_eq!(replies[0].body[after + 1], 1, "next must be set - there is a line 2");

        let next = s.on_script_reply(&reply_bytes(&text, ty, 1));
        assert_eq!(script_text(&next[0].body).1, "two");
    }


    /// A 0x00F3 whose action is -1 - the user closed the box - ends the conversation and
    /// sends nothing. An unanswered 0x00F3 costs only a dead conversation: it is not one of
    /// the 37 setters of the player->[0x2330] latch.
    #[test]
    fn closing_a_box_ends_the_conversation_silently() {
        let path = std::path::Path::new("../../gm-handbook/questlines.txt");
        if !path.exists() {
            return;
        }
        let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.create_migration(account_id, id, 0, 0).unwrap();
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(id);
        s.on_quest_request(&hex("01e8030000010000000c046d0100000000"));

        assert!(s.on_script_reply(&reply_bytes("anything", 0, -1i8 as u8)).is_empty());
        // And a second reply with no conversation open must not panic or answer.
        assert!(s.on_script_reply(&reply_bytes("anything", 0, 1)).is_empty());
        // Short bodies come off a socket.
        for n in 0..12 {
            let _ = s.on_script_reply(&vec![0u8; n]);
        }
    }

    /// Pull the text out of a script-message body. The shared head is 14 bytes; a Say then
    /// has a `u32 echo` before its string and a yes/no box does not.
    fn script_text(body: &[u8]) -> (u8, String, usize) {
        let message_type = body[10];
        let at = if message_type == net::script::SCRIPT_TYPE_SAY { 18 } else { 14 };
        let len = u16::from_le_bytes([body[at], body[at + 1]]) as usize;
        let text = String::from_utf8(body[at + 2..at + 2 + len].to_vec()).unwrap();
        (message_type, text, at + 2 + len)
    }

    /// The client's 0x00F3, in the shape the captures show: the box's own text echoed back.
    fn reply_bytes(text: &str, message_type: u8, action: u8) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u32.to_le_bytes()); // handle
        b.push(message_type);
        b.extend_from_slice(&0u32.to_le_bytes()); // echo
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b.push(action);
        b
    }

    /// A refused `!map` says why on screen now, and Log Out is answered.
    ///
    /// The Log Out half is the one that matters beyond politeness: until `0x0106` clears
    /// `world->[0x33f4]`, the client drops every `SetField` in silence.
    #[test]
    fn a_bad_map_says_why_and_log_out_is_answered() {
        let path = std::path::Path::new("../../gm-handbook/fields.txt");
        if !path.exists() {
            return; // generated data, gitignored
        }
        let config = Config {
            set_field_probe: true,
            fields: Config::load_fields(path),
            ..Config::default()
        };
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.create_migration(account_id, id, 0, 0).unwrap();
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(id);

        fn chat(text: &str) -> Vec<u8> {
            let mut b = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
            b.extend_from_slice(&[0u8; 4]);
            b.extend_from_slice(&(text.len() as u16).to_le_bytes());
            b.extend_from_slice(text.as_bytes());
            b.push(3);
            b
        }

        // A map with no field image is refused, and the refusal reaches the screen.
        let replies = s.handle(&chat("!map 104040000"));
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].opcode, net::notice::CHAT_NOTICE);
        assert_eq!(replies[0].body[0], 1, "force must be 1 or only the first line shows");
        let len = u16::from_le_bytes([replies[0].body[1], replies[0].body[2]]) as usize;
        let text = String::from_utf8(replies[0].body[3..3 + len].to_vec()).unwrap();
        assert!(text.contains("104040000"), "the notice must name the id: {text}");

        // So is a non-numeric one, rather than being swallowed.
        assert_eq!(s.handle(&chat("!map banana"))[0].opcode, net::notice::CHAT_NOTICE);

        // A real map still warps and does NOT produce a notice.
        assert_eq!(s.handle(&chat("!map 40"))[0].opcode, net::opcode::SET_FIELD);

        // **Anything that is not a command is said out loud.** The client draws nothing for
        // its own chat, so a server that answers nothing is a player typing into a void -
        // which is exactly what the owner got on 2026-08-19 from "Hello", "Hello2", "Hello3".
        let said = s.handle(&chat("Hello"));
        assert_eq!(said.len(), 1, "chat was swallowed");
        assert_eq!(said[0].opcode, net::userchat::USER_CHAT);
        let body = &said[0].body;
        assert_eq!(
            u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
            id,
            "the balloon has to be attached to the speaker"
        );
        let len = u16::from_le_bytes([body[5], body[6]]) as usize;
        assert_eq!(&body[7..7 + len], b"Hello");
        assert_eq!(body.len(), net::userchat::USER_CHAT_OVERHEAD + len,
                   "both trailing bytes must be there - the client reads past the text");

        // An unknown command says so rather than vanishing, and is NOT spoken aloud.
        let unknown = s.handle(&chat("!nope"));
        assert_eq!(unknown.len(), 1);
        assert_eq!(unknown[0].opcode, net::notice::CHAT_NOTICE);

        // And an empty line is dropped: a balloon with no text is worse than none.
        assert!(s.handle(&chat("")).is_empty());

        // Log out is answered, with a non-empty message - an empty one is a client no-op.
        let mut body = net::notice::CLIENT_LOG_OUT.to_le_bytes().to_vec();
        body.truncate(2);
        let out = s.handle(&body);
        assert_eq!(out.len(), 1, "an unanswered log out makes every later SetField vanish");
        assert_eq!(out[0].opcode, net::notice::LOG_OUT_RESULT);
        assert!(u16::from_le_bytes([out[0].body[0], out[0].body[1]]) > 0);
    }

    /// Idle chatter: the only unsolicited packet the server sends.
    ///
    /// Pins the three things a client run cannot easily show - the cadence window, that the
    /// lines advance in order and wrap, and that a stalled connection does not burst.
    #[test]
    fn npcs_chatter_in_order_on_the_clients_own_cadence() {
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
        let mut strings = std::collections::HashMap::new();
        strings.insert(
            8u32,
            crate::config::NpcStrings {
                name: "Robin".into(),
                info: (0..4).map(|i| format!("line {i}")).collect(),
                ..Default::default()
            },
        );
        // Template 9 has no info lines at all - it must never speak, and must not panic on
        // the modulo either.
        strings.insert(9u32, crate::config::NpcStrings::default());

        let config = Config {
            set_field_probe: true,
            npcs: [(40u32, npcs)].into_iter().collect(),
            npc_strings: strings,
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
        s.on_field_entered();

        // Nothing is due before the minimum wait, ever.
        assert!(s.tick(0).is_empty());
        assert!(s.tick(CHATTER_MIN_MS - 1).is_empty(), "3s is the floor");

        // Drive it and collect the lines Robin says, and when.
        let mut said = Vec::new();
        let mut gaps = Vec::new();
        let mut last = 0u64;
        for now in (0..120_000).step_by(250) {
            for reply in s.tick(now) {
                let body = &reply.body;
                assert_eq!(body.len(), net::npcchat::NPC_CHAT_LEN);
                let who = u32::from_le_bytes(body[..4].try_into().unwrap());
                assert_eq!(who, 1000, "only the NPC with lines may speak");
                assert_eq!(body[4] as i8, net::npcchat::NPC_CHAT_NO_ANIMATION);
                said.push(body[5]);
                if last > 0 {
                    gaps.push(now - last);
                }
                last = now;
            }
        }

        assert!(said.len() > 10, "only {} lines in two minutes", said.len());
        // In order, wrapping at the line count. This is the half that is ours.
        for (i, idx) in said.iter().enumerate() {
            assert_eq!(*idx as usize, i % 4, "line {i} out of order");
        }
        // And the cadence is the client's own window. The tick granularity can only make a
        // gap look longer, never shorter, so the floor is the strict check.
        assert!(
            gaps.iter().all(|g| *g >= CHATTER_MIN_MS),
            "a gap below the 3s floor: {:?}",
            gaps.iter().filter(|g| **g < CHATTER_MIN_MS).collect::<Vec<_>>()
        );
        let ceiling = CHATTER_MIN_MS + CHATTER_SPREAD_MS + 500;
        assert!(gaps.iter().all(|g| *g <= ceiling), "a gap above 9s: {gaps:?}");
        // Randomised, not fixed - a constant interval would be a regression to what the owner
        // asked to move away from.
        assert!(gaps.iter().collect::<std::collections::HashSet<_>>().len() > 2, "{gaps:?}");
    }

    /// Entering a field late must not make the whole map speak at once.
    ///
    /// `handle` takes bytes and not time, so a field entry has no clock of its own. Before
    /// the session carried one, `reset_chatter` scheduled from zero and every NPC on a map
    /// entered after the first ten seconds was already overdue.
    #[test]
    fn entering_a_field_late_does_not_make_everyone_speak_at_once() {
        let npcs = (0..3)
            .map(|i| net::opcode::FieldNpc {
                object_id: 1000 + i, template_id: 8, x: 0, cy: 0, fh: 1,
                rx0: 0, rx1: 0, f: 0,
            })
            .collect::<Vec<_>>();
        let mut strings = std::collections::HashMap::new();
        strings.insert(
            8u32,
            crate::config::NpcStrings {
                info: (0..4).map(|i| format!("line {i}")).collect(),
                ..Default::default()
            },
        );
        let config = Config {
            set_field_probe: true,
            npcs: [(40u32, npcs)].into_iter().collect(),
            npc_strings: strings,
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

        // Half a minute of ticks, then walk into the field - a portal walk, or a !map.
        //
        // **Inclusive of 30_000 on purpose.** `on_field_entered` schedules from the
        // session's own clock, which is the last tick it was given - so a range ending at
        // 29_500 would put the floor at 32_500 while the assertion below measured it from
        // 30_000. That is a real 500ms window, and with three NPCs each drawing from a
        // 6000ms spread it made this test fail about one run in six, on a seed that comes
        // from a heap address and so changes with what else the suite ran.
        for now in (0..=30_000).step_by(500) {
            s.tick(now);
        }
        s.on_field_entered();

        // Nothing may be due before the 3s floor measured from NOW, not from zero.
        for now in (30_000..30_000 + CHATTER_MIN_MS).step_by(250) {
            assert!(
                s.tick(now).is_empty(),
                "an NPC spoke {}ms after a late field entry",
                now - 30_000
            );
        }
        // And when they do start, they do not all go at once - three NPCs each drawing an
        // independent delay from a 6000ms window colliding exactly is the thing to notice.
        let mut first_tick_counts = Vec::new();
        for now in (30_000 + CHATTER_MIN_MS..50_000).step_by(250) {
            let n = s.tick(now).len();
            if n > 0 {
                first_tick_counts.push(n);
            }
        }
        assert!(!first_tick_counts.is_empty(), "nobody ever spoke");
        assert!(
            first_tick_counts.iter().any(|&n| n < 3),
            "every tick spoke for all three: {first_tick_counts:?}"
        );
    }

    /// The record carries a bag, and `--inventory-slots` can change what is in it.
    ///
    /// The default and the override are checked in the SAME test on purpose: 24 is also the
    /// number a client could plausibly have defaulted to on its own, so only a value that
    /// could not have come from anywhere else proves the field is being read. That is the
    /// same argument the flag's doc makes for spending a client launch at 32 rather than 24.
    #[test]
    fn the_record_sizes_the_bag_and_the_override_reaches_it() {
        fn record_of(config: Config) -> Vec<u8> {
            let store = Arc::new(Store::open_in_memory().unwrap());
            let account = store.create_account("maplecw", "correct horse battery").unwrap();
            let chr = net::opcode::Character {
                name: "TestCharD".to_string(), map_id: 1, ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::new(store, Arc::new(config));
            s.claim_for_character(id);
            let chr = s.claimed_character().expect("the claimed character");
            net::opcode::character_record_for_set_field(&chr, 0)
        }

        // Where the six u16 live: after the 100-byte presence array, the eleven head bytes,
        // the stat block and the four string flags.
        let sizes_at = net::opcode::STAT_BLOCK_AT + net::opcode::stat_block_len(0) + 4;
        let read = |record: &[u8]| -> Vec<u16> {
            (0..net::opcode::INVENTORY_COUNT)
                .map(|i| {
                    let at = sizes_at + i * 2;
                    u16::from_le_bytes([record[at], record[at + 1]])
                })
                .collect()
        };

        let plain = record_of(Config { set_field_probe: true, ..Config::default() });
        assert_eq!(
            read(&plain),
            vec![net::opcode::DEFAULT_INVENTORY_SLOTS; net::opcode::INVENTORY_COUNT],
            "a new character reached the wire with no bag"
        );
        assert_eq!(plain[net::opcode::PRESENCE_INVENTORY_SIZE], 1);

        let forced = record_of(Config {
            set_field_probe: true,
            inventory_slots: Some(32),
            ..Config::default()
        });
        assert_eq!(read(&forced), vec![32u16; net::opcode::INVENTORY_COUNT]);
        // And nothing else moved: same length, and the only differing bytes are the twelve.
        assert_eq!(plain.len(), forced.len());
        let differing: Vec<usize> =
            (0..plain.len()).filter(|&i| plain[i] != forced[i]).collect();
        assert!(
            differing.iter().all(|&i| (sizes_at..sizes_at + 12).contains(&i)),
            "the override changed bytes outside the bag: {differing:?}"
        );
        assert!(!differing.is_empty(), "the override changed nothing at all");
    }

    /// A connection that stalls must not emit a backlog when it comes back.
    #[test]
    fn a_late_tick_does_not_burst() {
        let npcs = vec![net::opcode::FieldNpc {
            object_id: 1000, template_id: 8, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0,
        }];
        let mut strings = std::collections::HashMap::new();
        strings.insert(
            8u32,
            crate::config::NpcStrings {
                info: (0..4).map(|i| format!("line {i}")).collect(),
                ..Default::default()
            },
        );
        let config = Config {
            set_field_probe: true,
            npcs: [(40u32, npcs)].into_iter().collect(),
            npc_strings: strings,
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
        s.on_field_entered();

        // Ten minutes with no ticks at all, then one. Exactly one line, not sixty.
        assert_eq!(s.tick(600_000).len(), 1);
        assert!(s.tick(600_001).is_empty(), "the next one waits the full interval");
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
