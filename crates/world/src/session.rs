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
            let branch = if reply.action == net::script::SCRIPT_ACTION_YES { "yes" } else { "no" };
            if !self.has_branch(&convo, branch) {
                self.conversation = None;
                return Vec::new();
            }
            if let Some(c) = self.conversation.as_mut() {
                c.path = format!("{}.{}", convo.path, branch);
                c.sent = 0;
            }
            // The quest itself still does not advance - there is no quest-result packet -
            // so this shows the branch's text and nothing more.
            return self.say_line(0);
        }

        if !convo.sent_with_next {
            self.conversation = None; // that was an OK on the last box
            return Vec::new();
        }
        self.say_line(convo.sent + 1)
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
        // **Dressed, exactly like the migration's SetField.** This sent the bare form until
        // 2026-08-19, which is why the owner's items had their stats on entering the world and
        // lost them the moment they used a portal or `!map`: every record after the first
        // carried EquipStats::default(), all zeros. It also explains the whole "the tooltip
        // reads a different object" investigation - they had reached map 40 with `!map`, so
        // the record they were hovering really did contain zeros. There was never a second
        // object.
        let dressed = self.dressed(chr);
        vec![Reply {
            opcode: net::opcode::SET_FIELD,
            body: net::opcode::set_field_with_character_dressed(
                chr,
                self.config.world_id,
                self.clock_base(),
                self.config.channel_id,
                &dressed,
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
            what: "LogOutResult - and answering this is what clears world->[0x33f4]. Until                    it is cleared the client silently drops every SetField."
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
        Some(chr)
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
