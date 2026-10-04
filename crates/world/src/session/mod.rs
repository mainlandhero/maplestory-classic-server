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

/// What `!help` prints, and what an unknown command is told.
///
/// One string so the two cannot drift - a help text that lists a command the dispatcher
/// does not have is worse than no help text. `session::gm::tests` checks every word here
/// against the dispatcher.
///
/// Pruned 2026-09-06 on the owner's instruction: the per-kind rate setters, `!migsweep`,
/// `!npcfx`, `!buff`, `!unbuff`, `!buy`, `!locker` and `!kit` are gone.
const GM_COMMANDS: &str =
    "GM commands: !map <mapId>, !item <itemId> [count], !exp <amount>, !heal, !setrates <exp> <meso> <drop> <quest> <party%>, !rates, !announce [message], !job <jobId>, !npcecho [dx], !nx [amount], !lp [amount], !meso [amount], !resetap, !resetsp, !learn [level] | !learn <skillId> <level>, !craft [profession] [level] [mastery], !npcreload [templateId], !hair <hairId>, !face <faceId>, !giftdrop <player> <itemId> [count] [message], !giftall <itemId> [count] [message], !registrationcode, !recoverycode <email|username>, !online, !track <character>, !citizenship [<town> <state|grade|contr> <value>], !help";

/// What a player who is not a GM is shown by `!help`, and all they may run. The owner,
/// 2026-09-06: *"A player should only be shown commands that they are allowed to execute."*
const PLAYER_COMMANDS: &str = "Commands: !tool, !scroll, !giftdrop, !rates, !online, !help";

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
    /// The chair the player is sitting on, if any - `net::chair`, `session/chair.rs`.
    ///
    /// Set by `0x00DB` and cleared by `0x00DA`. It scales the idle tick and nothing else; the
    /// client's own on-screen seating is not driven from here.
    seated_chair: Option<u32>,
    /// The **map** seat index the player is on, if any - a bench, not a Set Up item.
    ///
    /// Kept apart from [`Self::seated_chair`] on purpose: the two are different fields in the
    /// client (`CUser+0x3c28` the seat index, `+0x4080` the chair item id) reached by
    /// different opcodes, and the Set Up path is CONFIRMED on two screens while this one is
    /// new. This exists so the stand-up only sends `0x0252` to a player who actually took a
    /// map seat, leaving the confirmed path byte-for-byte as it was.
    seated_map_seat: Option<u16>,
    /// Mixed into each `!scroll` roll so two scrolls in one session cannot share one.
    scroll_roll_counter: u64,
    /// **The EXP coupon that is running, if one is.** `(percent of normal, expires, item id)`.
    ///
    /// The owner, 2026-09-09: *"Once the EXP buff is applied, either the 2x or the 3x EXP buff, are
    /// we sure that the EXP gained is actually properly being modified?"* It was not - the
    /// coupon was consumed, granted nothing, and `Restores::exp_percent` was read by no code
    /// at all. It is read now, by `Session::with_exp_coupon`.
    ///
    /// **Not in `buffs`**, which is a list of CTS bits with a wire representation. This has no
    /// measured CTS bit in this repo and is a server-side rate, so putting it there would
    /// imply a packet that is never sent.
    ///
    /// Session state, so it does NOT survive a channel change or a relog. That is a real
    /// limitation and it is the honest one to ship first: the alternative is a stored expiry
    /// on the character row, which is a schema change and a migration for a fifteen-minute
    /// item.
    exp_coupon: Option<crate::consumables::ExpCoupon>,
    /// The source address this connection arrived from, if the socket reported one.
    ///
    /// **Recorded and reported, never decisive.** Two clients on one machine share it, so
    /// it cannot separate them - which is exactly the case the owner asked about.
    peer: Option<String>,
    /// The **full** source address, port included.
    ///
    /// `peer` above is the address alone and is only ever a log line. This is the one the
    /// operating system's TCP table is keyed by, and the port is not optional there: every
    /// client on this machine shares `127.0.0.1`, so the port is the whole of what picks one
    /// socket out. `store::peerowner::owning_pid` takes a `SocketAddr` for exactly that
    /// reason, and an IP-only lookup would match the first row and return a confident wrong
    /// pid rather than nothing.
    ///
    /// `None` for every constructed-in-a-test session, which is why the attestation below
    /// degrades to "presents nothing" rather than failing.
    peer_addr: Option<std::net::SocketAddr>,
    /// The server's own end of the accepted socket - which interface this client reached.
    ///
    /// Under `--advertise auto` a directly-connected client is told THIS host, with the
    /// target channel's port, when it changes channel. `None` in every test-built session,
    /// where the listed host is used instead. `net::advertise`.
    local_addr: Option<std::net::SocketAddr>,
    // `asked_to_hide_hit_damage` was removed 2026-08-28 with the packet it gated. `0x00EA`
    // carrying "/hitdamagetest 0" reached the client and was ECHOED into chat, and no
    // `0x0189` ever came back - so the command's permission gate refused it and the stub
    // number is still drawn. See `session::field::on_field_entered`.
    /// The NPC conversation in progress, if any.
    conversation: Option<Conversation>,
    /// Requesters whose friend popup this session has raised, and **when** - the `now_ms` of
    /// the tick the balloon went up on. `session/friends.rs`.
    ///
    /// It is a map rather than a set for two jobs at once. A pending request lives in the
    /// store until it is answered, so without it every map change would raise the balloon
    /// again for something the player has already left unanswered; and the timestamp is what
    /// [`Session::friend_timeout_tick`] measures the offer against. Per session, so a relog
    /// offers it again with a fresh clock.
    friend_popups_raised: std::collections::HashMap<u32, u64>,
    /// **A feed waiting for its line** - `session/pet.rs`. Set when the pet eats: the map's
    /// copy of the eating packet (`0x027E`) and the clock time it falls back at. The owner's
    /// client reports the line it picked (`0x0203`) and everyone else is shown THAT line
    /// (`0x0279`); if no report arrives by then, the map gets the `0x027E` as it used to.
    pending_pet_line: Option<(u64, Reply)>,
    /// **The effect item switched on** - Shadow Style and the rest of `501xxxx`, `0` for none.
    /// `session/emote.rs`. Saved (`store::effectitem`) and restored at claim, so it survives a
    /// relog and a channel change (the owner, 2026-09-30); carried in this character's `0x0224` so
    /// everyone else on the field, arriving or already there, sees it.
    active_effect_item: u32,
    /// **The Community Board postings last sent**, and the UTC day they were for -
    /// `session/citizenship.rs`. Set where the `SetField` builds its quest book (a `&self`
    /// path, hence the cell) and read by the tick, which re-sends what changed when the day
    /// turns under a player who has not moved.
    board_sent: std::cell::RefCell<Option<(i64, Vec<(u32, String)>)>>,
    /// Whether this session has told its friends it is online yet - `session/friends.rs`.
    ///
    /// **Not done at claim**, which is the obvious place and the wrong one: presence is
    /// registered when the character enters a field, so a notice sent at claim would tell
    /// everyone this character is offline. It goes out on the FIRST field entry instead, and
    /// this flag is what keeps it from firing again on every portal.
    announced_presence: bool,
    /// The name an Open (mode 0) asked to invite. With the hub up the room's id is only known
    /// once the echo comes back, so the invite waits here for it - `session/messenger.rs`.
    pending_messenger_invite: Option<String>,
    /// The craft this session accepted and is waiting to finish - `session/craft.rs`.
    ///
    /// **A craft is two packets**: the window asks to begin, animates the recipe's own
    /// `ProcessTimeMS`, then asks to finish. Nothing is taken until the second, and the
    /// second is refused outright when this is `None`, so a `0x02F6 mode 3` on its own
    /// cannot conjure an item.
    pending_craft: Option<craft::PendingCraft>,
    /// **Is the client showing the Cash Shop rather than the field?**
    ///
    /// The owner, 2026-08-26: *"we should fix NPC idle chatter when player is in cash shop."* The
    /// chatter tick is the server's only unsolicited path and it does not know what the client
    /// is looking at, so about forty `0x0453`s per visit were going out addressed to NPC object
    /// ids on a field the client had stopped drawing.
    ///
    /// Set when `0x01A3` goes out and cleared when the Exit button's `SetField` does, plus
    /// again on any field entry - a field entry means the client is in the field stage
    /// whatever route it took there, so that clear is the one that cannot be forgotten.
    ///
    /// **It gates the chatter and nothing else.** Drops still sweep, mobs still respawn and
    /// buffs still expire while the shop is open: those are properties of the world, not of
    /// what is on screen, and stopping them here would make a bug in the drop table look like
    /// a bug in the shop. That is the same reasoning `chatter_off` already carries in
    /// [`Session::tick`].
    in_cash_shop: bool,
    /// Where each NPC on the current field is in its idle chatter.
    chatter: Vec<Chatter>,
    /// Drives the chatter cadence. Seeded per session so two connections do not speak in
    /// lockstep, and seedable so a test can pin the sequence.
    rng: Xorshift,
    /// **The mob-vacuum watch** over the mobs this connection controls - `session/mobwatch.rs`.
    mob_watch: mobwatch::MobWatch,
    /// **The forced-miss and low-damage watch** over this connection's `0x00E5` -
    /// `session/hitwatch.rs`.
    hit_watch: hitwatch::HitWatch,
    /// How often each kind of suspicion may be written (`mob-suspects.log`).
    suspect_throttle: mobwatch::Throttle,
    /// The last time [`Session::tick`] was called, in milliseconds since the connection
    /// opened.
    ///
    /// **A field entry needs to know the clock and does not get one**: `handle` takes bytes,
    /// not time. Without this, `reset_chatter` scheduled from zero, so an NPC on a map
    /// entered at t = 30 s came due at 3-9 s - already in the past - and the whole field
    /// spoke on the very next tick. It is at most one tick stale, which is 500 ms.
    clock_ms: u64,
    /// Summoned mobs whose summoning animation is still playing: `(due_ms, map, objectId)`.
    /// [`Session::tick`] sends each its `0x03E8` when due. `session/summonsack.rs`.
    pending_suspend_resets: Vec<(u64, crate::fields::FieldKey, u32)>,
    /// The NPC whose shop is open, and the rows **exactly as they went on the wire**.
    ///
    /// The client hands back only a `row_key`, so the rows have to be kept to turn one back
    /// into an item and a price. Keeping the sent copy rather than re-deriving it is the
    /// point: a re-derivation that disagreed by one row would charge the wrong price for the
    /// right-looking click, and nothing on either side would notice.
    /// The counter the player has open, and **the rows exactly as they went on the wire**.
    ///
    /// `net::classicshop`, not `net::shop`: this client has two shop windows and the art for
    /// the `0x0560` one is **not in its WZ**, so that packet killed the client twice before a
    /// single row byte was read. The classic `0x055D` window is the one that exists.
    ///
    /// The rows are kept verbatim because a request names a **row index**, and the only
    /// defensible reading of that index is the list we actually sent.
    open_shop: Option<(u32, Vec<net::classicshop::ClassicShopRow>)>,
    // **The Buy Back ring was removed on 2026-08-28, the day it first went out.** It held
    // what the player had sold, 15 deep, and it existed to fill the Buy Back tab.
    //
    // `UI/UIShop.img/Shop` in this client has **16 nodes and no `repurchaseInfo`**, and
    // carries exactly `TabBuy` and `TabSell`. The `0x055E` type 10 that presented the ring
    // *selects the Buy Back tab*, so it reached for art that is not here, threw, and faulted
    // the unwinder - the same failure as Shop2, at the same address. See `session::shop`.

    /// The storage keeper whose window is open, by **template** id.
    ///
    /// Kept for one reason: the deposit fee is a property of the keeper, and a put-in
    /// request does not name them. `research/storage.md` §11.4 - *"charge `trunkPut`"* - and
    /// `trunkPut` is `Npc.wz`'s, per template, 100 for nine of the ten and 150 for Mr. Thalj.
    /// Guessing 100 would be right nine times in ten and quietly wrong once, which is the
    /// worst shape a number can have here.
    ///
    /// `None` means no window is open, and a put-in that arrives then is refused rather than
    /// charged a fee nobody can name.
    open_storage: Option<u32>,

    /// The temporary stats this character is holding, one entry per CTS bit.
    ///
    /// Kept on the **session** rather than in the database on purpose: a buff is measured
    /// against `clock_ms`, which restarts with the connection, so a persisted expiry would
    /// be compared against a clock that no longer means the same thing. Losing buffs on
    /// relog is also what this game family does.
    buffs: Vec<crate::session::buff::ActiveBuff>,
    /// When Dragon Blood next drains, in session milliseconds. Meaningful only while CTS 105
    /// is held; `buff::dragon_blood_tick`.
    dragon_blood_next_ms: u64,
    /// When MP Eater may proc again: its row carries `cooltime 5`. `combat::mp_eater`.
    mp_eater_ready_ms: u64,

    /// `skill id -> the session millisecond it may be cast again`. `Skill.wz`'s `cooltime`,
    /// which is **seconds** there and milliseconds here.
    skill_ready_ms: std::collections::HashMap<u32, u64>,

    /// Everything alive on this **channel's** maps: mobs, their positions, and the floor.
    ///
    /// **Shared by every connection, not owned by this one.** It used to be four maps on the
    /// `Session`, which meant a mob's position died with the player looking at it and a
    /// second player would have seen an empty field. The owner set the model: *"Mob locations are
    /// stored per channel instance per map."* See `crate::fields`.
    fields: std::sync::Arc<crate::fields::Fields>,

    /// Where the character last told us it was standing, if it ever has.
    ///
    /// **The drop position problem, and it is a real one.** `0x0107` carries no coordinates,
    /// and the client's pick-up sweep is a box of `x-0x19..x+0x19` by `y-0x32..y+0x0a` around
    /// the *player* - so an item dropped more than about 25 pixels off is drawn, and cannot
    /// be picked up, and on screen that is identical to nothing having happened.
    ///
    /// Fed from `0x00D9`, the client reporting its own movement, and from the attack
    /// request's own coordinates. Both are the client's word for where it is.
    ///
    /// **The `0x00D9` value is the END of the movement path, not the head's coordinates.**
    /// The head reports where the walk *started* and lags by one report - measured 14-50 px
    /// behind while running, and 50 px is twice the width of the pick-up box, so using it
    /// would have produced exactly the ambiguous "nothing on the ground" result this field
    /// exists to avoid. The path's end is exact when standing still, which is the case a
    /// drag out of the inventory window is. `research/user-move.md`.
    ///
    /// A drop with no position at all is refused rather than guessed, because a guessed one
    /// loses the item to a spot the player cannot reach.
    last_position: Option<(i16, i16)>,
    /// The claimed character's name, read once at claim time for [`Session::log_tag`].
    log_name: Option<String>,
    /// The stance and facing this character last reported, `(action << 1) | facing`.
    ///
    /// `None` until a move arrives. **Not defaulted to `0`**: that is action 0 facing right,
    /// a value this client never sends, and announcing it is why every remote player faced
    /// right. `net::userpool::MOVE_ACTION_STANDING` is the fallback instead.
    last_move_action: Option<u8>,
    /// The pet this session has summoned, if any - session-only, put away by a relog.
    /// `session/pet.rs`.
    active_pet: Option<pet::ActivePet>,
    /// Where the summoned pet last reported walking to, on which field, with its stance -
    /// read off its `0x0202`. What a player arriving later is told, so the pet does not
    /// appear where it was summoned and then snap. `session/pet.rs` `on_pet_move`.
    pet_position: Option<(crate::fields::FieldKey, i16, i16, Option<u8>)>,
    /// When the summoned pet next loses a fullness, on the session clock. `None` with no pet
    /// out. session/pet.rs `pet_hunger_tick`.
    pet_hunger_due_ms: Option<u64>,
    /// Feeds on an already-full pet this session; the first is free. `crate::petlevel::feed`.
    pet_overfeeds: u32,
    /// **A field entry summoned the pet, and the client has not yet shown it is live.** Set by
    /// `pet_entry_replies`, cleared by the first move packet after it, which sends the pet a
    /// put-away + summon + item write - the sequence a re-summon and a feed both use and that
    /// works, because it lands on a pet the client has FINISHED building. session/pet.rs
    /// `pet_settle_replies`.
    pet_settle_pending: bool,
    /// **A field entry found a gift waiting.** Set by `arm_gift_drop` after the bag restore,
    /// cleared by the first move after it, which sends the notice and the Administrator's
    /// box - the same "the client is provably live" moment the pet settle uses, so a
    /// `0x055B` never lands with a `SetField`. session/giftdrop.rs.
    gift_drop_pending: bool,
    /// **This connection is ending because the character is moving to another channel**, so
    /// its drop is a handover, not a departure: the party keeps the seat. Set by
    /// `on_change_channel` once the migration is minted. `session/party.rs`
    /// `leave_party_on_disconnect`.
    handing_over: bool,
    /// The party has already been told this connection went away (log out does it before
    /// the socket closes; `Drop` does it for a crash), so `Drop` does not say it twice.
    party_told_of_disconnect: bool,
    /// The party window has been pushed once this session, at the first field entry: a seat
    /// persists across a disconnect (while another member is online), so a returning member's window is rebuilt from the
    /// registry. Once, not per portal. session/party.rs `party_window_on_login`.
    party_window_sent: bool,
    /// Replies that must wait for the NEXT field entry (`0x00DC`), because they are dropped in
    /// silence while the client has no field - a revive's Safety Charm lines are the case
    /// that needs it (`session/field.rs` `revive`). Drained at the end of `on_field_entered`.
    after_field_entry: Vec<Reply>,

    /// Session milliseconds of the last thing the player did: moved, attacked, or was hit.
    ///
    /// Idleness is the absence of packets rather than a packet of its own - the client never
    /// says "I am idle". See `crate::session::regen`.
    last_activity_ms: u64,

    /// When the next regeneration tick is due, or `None` when the timer is not armed.
    ///
    /// `None` is both "they are busy" and "they are already full", and neither needs a timer.
    next_regen_ms: Option<u64>,
    /// Recovery (skill 1001) in progress, if any.
    ///
    /// Not in `buffs` with the stat buffs: Recovery has no CTS bit, so there is nothing to
    /// put there. It is HP arriving over time, which the server owns outright.
    /// `crate::session::recovery`.
    recovering: Option<recovery::Recovering>,
    /// What this character's HP was last broadcast as to party members on its field, and to
    /// whom: `(hp, max_hp, recipients)`. `party_hp_tick` resends when any of the three
    /// differs, which is how one comparison covers "the party formed", "a member arrived on
    /// this map" and "my HP changed". `None` when there is nobody to tell.
    last_party_hp: Option<(u32, u32, Vec<u32>)>,

    /// This connection's mailbox on the channel's message bus.
    ///
    /// Taken at construction rather than at field entry, so that the `Drop` below
    /// always has one to hand back. A connection that dies before it ever reaches a
    /// field still has to be cleaned up, and a mailbox that is only created on
    /// success is missing from exactly the failure case that needs it.
    ///
    /// `crate::broadcast`.
    subscriber: crate::broadcast::SubscriberId,

    /// What this connection last put in the client's scrolling banner, `None` for "nothing".
    ///
    /// The banner is not pushed to anyone - every session works out what should be on screen
    /// from the shared `server_rates` table and sends only when its own answer changes. This
    /// is that answer. Re-sending an identical string is not a no-op on screen: the client
    /// resets the banner object before it looks at the flag, so it would restart the scroll
    /// twice a second. See `crate::session::rates`.
    banner_shown: Option<String>,
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
    pub(super) fn next(&mut self) -> u64 {
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

/// Hand the mailbox back, and tell the field this player is gone.
///
/// **This is the only departure path that always runs.** A clean log out goes
/// through `on_log_out`, a channel change through `on_change_channel` - and a client
/// that is killed, or a socket that dies, sends neither. If only the tidy exits
/// announced a departure, a crashed client would leave a character standing on the
/// field of every other player until the channel restarted, which is the one
/// multiplayer bug that cannot be cleaned up from the client side.
///
/// `Bus::part` is idempotent, so the ordinary path - leave the field, then close the
/// socket - still announces exactly one departure. `crate::broadcast::Bus::part`.
impl Drop for Session {
    fn drop(&mut self) {
        // **Where they stood, as the spawn point they come back in at** - a log off, a dropped
        // socket, a crash, a channel change. First, while the character is still claimed.
        self.remember_spawn_point("the connection closed");
        // **The party, before anything else is torn down**: a dropped socket, a crash, a kill.
        // A channel change is a handover and says nothing here (`handing_over`); a log out
        // already said it. `session/party.rs` `leave_party_on_disconnect`.
        if !self.handing_over {
            self.leave_party_on_disconnect();
        }
        // Out of any party-quest run, a channel change included. session/firsttime.rs.
        self.leave_party_quest_on_disconnect();
        // And off any ship to Orbis. session/boat.rs.
        self.leave_ship_on_disconnect();
        // Out of any trade window, so the partner's closes too. A channel change included:
        // a trade room belongs to the channel. session/trade.rs.
        self.leave_trade_on_disconnect();
        self.leave_game_on_disconnect();
        // An owner leaving closes their store and the shelf comes home. session/playershop.rs.
        self.leave_shop_on_disconnect();
        // The hub's directory: this character no longer plays on this channel. Before
        // `part`, which is the local equivalent. `session/worldlink.rs`.
        self.announce_offline_to_link();
        self.fields.bus().part(self.subscriber);
        // **And their friends' windows.** After `part`, deliberately: the friends redraw their
        // list from the roster, and this connection has to be out of it before they ask or
        // they will be told this character is still online. `session/friends.rs`.
        if !self.handing_over {
            self.notify_friends_of_presence(false);
            // **Out of any Maple Chat room**, or the others keep drawing an empty seat. A
            // channel change is a handover and keeps the seat: the room belongs to the world.
            if let Some(chr) = self.claimed_character() {
                let seated = self.fields.messengers().room_of(chr.id).is_some();
                if seated {
                    let _ = self.run_messenger_request(chr.id, crate::messenger::Request::Disconnect);
                }
            }
        }
        // **And its mobs go back, or they stop moving for everybody.**
        //
        // A connection that dies without logging out - the socket drops, the client crashes,
        // the process is killed - has already left through `Bus::part` above. Its *mob
        // claims* are separate state, and without this they stay held by a `SessionId` that
        // will never exist again: every mob it controlled is permanently uncontrolled, no
        // other client is ever granted it, and on screen they simply stand still forever.
        //
        // That failure has no error and no log line of its own, which is what makes it worth
        // a line here rather than at the three orderly exits (`go_to_map`, channel change,
        // log out) that already release. Those are the paths a player takes; this is the one
        // a crash takes, and it is the one nobody would think to test.
        //
        // Idempotent, like `Bus::part` beside it: the orderly paths have usually handed
        // over already and this finds no map still held.
        //
        // A handover rather than a bare release, and this is the exit where that matters
        // most: a crash is the one departure the leaving player does not see, so the only
        // screens left to get it wrong are other people's. `Bus::part` above has already
        // removed this mailbox, so it cannot pick itself as the successor.
        self.hand_over_all_mobs();
    }
}

mod ability;
pub(crate) mod damageguard;
pub(crate) mod mobdebuff;
mod boat;
mod hotel;
mod jumpquest;
mod megaphone;
mod weather;
mod firsttime;
mod buff;
mod buffcarry;
mod chair;
mod beautycoupon;
mod salon;
mod cashitem;
mod charinfo;
mod citizenship;
mod emote;
mod fame;
mod friends;
mod cashshop;
mod giftdrop;
mod combat;
mod craft;
mod consume;
mod field;
mod gm;
mod ground;
mod inventory;
mod keymap;
mod reactor;
mod mobskill;
mod multiplayer;
mod npc;
mod options;
mod party;
mod groupchat;
mod whisper;
mod messenger;
pub mod worldlink;
mod pet;
mod pools;
mod mobwatch;
mod hitwatch;
mod questmoney;
mod rates;
mod recovery;
mod regen;
mod reports;
mod realscroll;
mod scroll;
mod summonsack;
mod shop;
mod storage;
pub(crate) mod minigame;
pub(crate) mod playershop;
pub(crate) mod trade;
mod skills;
#[cfg(test)]
mod tests;

impl Session {
    /// A session on a channel of its own. Every test uses this; the server does not.
    pub fn new(store: Arc<Store>, config: Arc<Config>) -> Self {
        Self::joining(store, config, std::sync::Arc::new(crate::fields::Fields::new()))
    }

    /// A session joining a channel that already exists, sharing its fields with every other
    /// connection on it. **This is what the server uses** - see `crate::fields`.
    pub fn joining(
        store: Arc<Store>,
        config: Arc<Config>,
        fields: std::sync::Arc<crate::fields::Fields>,
    ) -> Self {
        // Any non-zero seed will do; the config's address is simply something that differs
        // between connections in the same process.
        let seed = Arc::as_ptr(&config) as u64 | 1;
        // Before the `Arc` is moved into the struct.
        let subscriber = fields.bus().join();
        Session {
            store,
            config,
            subscriber,
            claimed: None,
            seated_chair: None,
            seated_map_seat: None,
            scroll_roll_counter: 0,
            exp_coupon: None,
            peer: None,
            peer_addr: None,
            local_addr: None,
            conversation: None,
            in_cash_shop: false,
            chatter: Vec::new(),
            rng: Xorshift(seed),
            mob_watch: mobwatch::MobWatch::default(),
            hit_watch: hitwatch::HitWatch::default(),
            suspect_throttle: mobwatch::Throttle::default(),
            clock_ms: 0,
            pending_suspend_resets: Vec::new(),
            fields,
            open_shop: None,
            open_storage: None,
            buffs: Vec::new(),
            dragon_blood_next_ms: 0,
            mp_eater_ready_ms: 0,
            skill_ready_ms: std::collections::HashMap::new(),
            last_position: None,
            friend_popups_raised: std::collections::HashMap::new(),
            pending_pet_line: None,
            active_effect_item: 0,
            board_sent: std::cell::RefCell::new(None),
            announced_presence: false,
            pending_messenger_invite: None,
            pending_craft: None,
            log_name: None,
            last_move_action: None,
            active_pet: None,
            pet_position: None,
            pet_hunger_due_ms: None,
            pet_overfeeds: 0,
            pet_settle_pending: false,
            gift_drop_pending: false,
            handing_over: false,
            party_told_of_disconnect: false,
            party_window_sent: false,
            after_field_entry: Vec::new(),
            banner_shown: None,
            last_activity_ms: 0,
            next_regen_ms: None,
            recovering: None,
            last_party_hp: None,
        }
    }

    /// Record the address this connection came from.
    ///
    /// A builder rather than a parameter so `joining`'s signature and its many test callers
    /// are unchanged. See the `peer` field: this is evidence for a log line, not a guard.
    pub fn with_peer(mut self, peer: impl Into<String>) -> Self {
        self.peer = Some(peer.into());
        self
    }

    /// Record the **full** address this connection came from, port included.
    ///
    /// Supersedes [`Session::with_peer`] on the server path and **sets both fields**, so the
    /// log line is unchanged and `claim_for_character` gains the one fact it can attest with.
    /// See the `peer_addr` field for why the port is not optional.
    ///
    /// `with_peer` is kept rather than replaced, and that is deliberate: roughly sixty test
    /// call sites use it, and they must keep resolving to `peer_addr: None` so they keep
    /// presenting no credential. An address without a port is still worth logging; it simply
    /// cannot attest.
    pub fn with_peer_addr(mut self, addr: std::net::SocketAddr) -> Self {
        self.peer = Some(addr.ip().to_string());
        self.peer_addr = Some(addr);
        self
    }

    /// Record the server's end of the accepted socket. See the `local_addr` field.
    pub fn with_local_addr(mut self, addr: std::net::SocketAddr) -> Self {
        self.local_addr = Some(addr);
        self
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
        if !self.config.answer_packets {
            return Vec::new();
        }
        // **First**, because this is the path that carries another player's movement
        // to a client that is standing still and therefore sending nothing. `handle`
        // covers the busy case; this covers the idle one, and between them a
        // broadcast waits at most one tick.
        let mut out = self.collect_mail();
        // Party requests answered by the hub's echo. `session/worldlink.rs`.
        out.extend(self.collect_party_outcomes());
        out.extend(self.collect_messenger_outcomes());
        // Expire drops BEFORE the chatter switch is consulted. `chatter_off` turns off NPC
        // idle lines and nothing else; if the sweep sat after it, a run with chatter
        // disabled would leave items on the floor forever and the bug would look like the
        // drop table rather than the switch.
        let here = self.claimed_character().map(|c| self.field_of(&c)).unwrap_or_default();
        out.extend(self.fields.with_drops(here, |d| d.sweep(here, now_ms)));
        // Refill spawn points whose WZ timer has come due. Before the chatter switch for the
        // same reason the sweep is: `chatter_off` turns off NPC idle lines and nothing else,
        // and a run with it set should not also stop the world respawning.
        out.extend(self.spawn_due_mobs(here, now_ms));
        // Broken boxes whose reactorTime has run out. session/reactor.rs.
        out.extend(self.spawn_due_reactors(here, now_ms));
        // Summoned mobs whose animation has ended become targetable. `session/summonsack.rs`.
        out.extend(self.suspend_reset_tick(now_ms));
        // The event banner. Wall-clock, not `now_ms` - see `crate::session::rates`.
        out.extend(self.banner_tick());
        // Idle regeneration, which uses `now_ms` rather than the wall clock - it is a
        // property of one player rather than of the server. `crate::session::regen`.
        out.extend(self.regen_tick(now_ms));
        // Recovery's heal-over-time. Beside idle regen because it is the same shape - a
        // `0x007C` and a blue number - and after it so that a tick carrying both puts the two
        // stat changes in a stable order.
        out.extend(self.recovery_tick(now_ms));
        // **Party HP to the members on this field.** After regen and recovery, so a tick that
        // changed HP broadcasts the value it just saved. Nothing comes back to this
        // connection; the packets go out over the bus. `crate::session::party::party_hp_tick`.
        self.party_hp_tick();
        // The party quest's clock. Before the buffs for no reason beyond a stable order.
        out.extend(self.party_quest_timer_tick());
        // The ships to Orbis: sail what is due, land what has arrived. session/boat.rs.
        out.extend(self.boat_tick());
        // Hired merchants that have stood their 24 hours close. session/playershop.rs.
        out.extend(self.merchant_tick());
        // Buffs whose time is up. After regen so a `0x007C` and a `0x007E` in the same
        // tick arrive in the order the client draws them.
        out.extend(self.buff_tick(now_ms));
        out.extend(self.dragon_blood_tick(now_ms));
        // The summoned pet's fullness, one down every five minutes. session/pet.rs.
        out.extend(self.pet_hunger_tick(now_ms));
        // A friend request nobody answered. Before the chatter switch, because an unanswered
        // invitation expiring is not idle chatter and a run with `chatter_off` should still
        // do it. `session/friends.rs`.
        out.extend(self.friend_timeout_tick(now_ms));
        // A feed whose line never came back. session/pet.rs.
        self.pet_line_fallback_tick(now_ms);
        // The Community Board turning over at midnight UTC. session/citizenship.rs.
        out.extend(self.board_tick());
        if self.config.chatter_off {
            return out;
        }
        // **Nobody in the Cash Shop wants forty balloons a visit.** The client is not drawing
        // the field, so every one of these is addressed to an object it has put away. See
        // `Session::in_cash_shop`.
        if self.in_cash_shop {
            return out;
        }
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
        // One snapshot for the whole field rather than a lock acquisition per NPC. The guard
        // is released inside `snapshot()`, so nothing is held while the vector is built.
        //
        // **This count is cached until the next field entry**, which is the one thing
        // `!npcreload` does not reach live. It does not matter today: the authored overlay
        // carries `d<n>` rows only and cannot change an `info` count - see
        // `config::parse_npc_dialogue_overlay`.
        let strings = self.config.npc_strings.snapshot();
        let chatter: Vec<Chatter> = self
            .config
            .npcs
            .get(&map)
            .unwrap_or(&empty)
            .iter()
            .map(|npc| Chatter {
                object_id: npc.object_id,
                lines: strings.get(&npc.template_id).map(|s| s.info.len()).unwrap_or(0),
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
    /// **Answers nothing at all when [`Config::answer_packets`] is off** - which now takes
    /// `--silent-channel`, asked for by name. Without answers the migration
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
        let mut out = self.dispatch(body);
        // **After** whatever this packet asked for, never before or inside it. A
        // reply sequence like a `SetField` and its field contents is a script the
        // client walks in order, and another player's movement spliced into the
        // middle of one is the kind of reordering that is invisible in a log and
        // fatal on screen.
        out.extend(self.collect_mail());
        out.extend(self.collect_party_outcomes());
        out.extend(self.collect_messenger_outcomes());
        out
    }

    /// The dispatch itself. `handle` wraps it so that every arm below can `return`
    /// early - which most of them do - without each one having to remember to
    /// collect this connection's mail on the way out.
    fn dispatch(&mut self, body: &[u8]) -> Vec<Reply> {
        let opcode = match body.get(..2) {
            Some(b) => u16::from_le_bytes([b[0], b[1]]),
            None => return Vec::new(),
        };
        if !self.config.answer_packets {
            return Vec::new();
        }
        match opcode {
            CLIENT_MIGRATION_HELLO => {}
            // **`0x00D1` is two different requests and only the LENGTH separates them.** A
            // portal walk sends 35 bytes; the Cash Shop's Exit button sends none. Getting this
            // split backwards would make every portal in the game try to leave a cash shop.
            CLIENT_TRANSFER_FIELD
                if body.len() == 2 + net::cashshop::CASH_SHOP_EXIT_BODY_LEN =>
            {
                return self.on_cash_shop_exit()
            }
            CLIENT_TRANSFER_FIELD => return self.on_transfer_field(body.get(2..).unwrap_or(&[])),
            // A SCRIPT portal (pt 7/8) never sends 0x00D1; it sends this. Found 2026-09-10
            // when the Ellinia Station door stayed dead with its destination in the table.
            net::portalscript::CLIENT_PORTAL_SCRIPT => {
                return self.on_portal_script(body.get(2..).unwrap_or(&[]))
            }
            net::storage::CLIENT_STORAGE => {
                return self.on_storage_request(body.get(2..).unwrap_or(&[]))
            }
            net::revive::CLIENT_REVIVE_ON_SPOT => {
                return self.on_revive_on_spot(body.get(2..).unwrap_or(&[]))
            }
            CLIENT_FIELD_ENTERED => return self.on_field_entered(),
            net::opcode::CLIENT_CHAT => return self.on_chat(body.get(2..).unwrap_or(&[])),
            net::script::CLIENT_QUEST_REQUEST => {
                return self.on_quest_request(body.get(2..).unwrap_or(&[]))
            }
            // A breakable box struck. session/reactor.rs.
            net::reactor::CLIENT_REACTOR_HIT => {
                return self.on_reactor_hit(body.get(2..).unwrap_or(&[]))
            }
            // A double-click on a pet in the Cash tab. session/pet.rs.
            net::pet::CLIENT_PET_ACTIVATE => {
                return self.on_pet_activate(body.get(2..).unwrap_or(&[]))
            }
            // The pet walked. Forwarded to the map so other players see it. session/pet.rs.
            net::pet::CLIENT_PET_MOVE => {
                return self.on_pet_move(body.get(2..).unwrap_or(&[]))
            }
            // Pet Food from the Use tab. session/pet.rs, net::petfood.
            net::petfood::CLIENT_USE_PET_FOOD => {
                return self.on_use_pet_food(body.get(2..).unwrap_or(&[]))
            }
            // The line the owner's pet just said, after that feed. Relayed so every screen
            // shows the same bubble. session/pet.rs (2026-09-25).
            net::pet::CLIENT_PET_LINE_REPORT => {
                return self.on_pet_line_report(body.get(2..).unwrap_or(&[]))
            }
            // An emote (F1-F7, Queasy...): relayed to the rest of the map as 0x02A6.
            // session/emote.rs (2026-09-29).
            net::userpool::CLIENT_EMOTION => return self.on_emotion(body.get(2..).unwrap_or(&[])),
            // Shadow Style and the other effect items: switched on or off, relayed as 0x02A8
            // and kept in this character's 0x0224. session/emote.rs (2026-09-29).
            net::userpool::CLIENT_EFFECT_ITEM => return self.on_effect_item(body.get(2..).unwrap_or(&[])),
            // **The pet reached a drop.** The same handler as the player's request: it finds
            // the drop by the pet offset (byte 17) and takes it with the pet's leave type.
            // Seven of these went unanswered on 2026-09-15 - "Husky does not loot".
            net::pet::CLIENT_PET_PICK_UP => {
                return self.on_pick_up(net::pet::CLIENT_PET_PICK_UP, body.get(2..).unwrap_or(&[]))
            }
            op if net::combat::is_attack_opcode(op) => {
                return self.on_attack(op, body.get(2..).unwrap_or(&[]))
            }
            net::inventory::CLIENT_INVENTORY_MOVE => {
                return self.on_inventory_move(body.get(2..).unwrap_or(&[]))
            }
            // Consolidate Item: merge the tab's stacks. session/inventory.rs.
            net::inventory::CLIENT_GATHER_ITEMS => {
                return self.on_gather_items(body.get(2..).unwrap_or(&[]))
            }
            // Sort Items: the consolidate, then biggest stack first, then name. Same file.
            net::inventory::CLIENT_SORT_ITEMS => {
                return self.on_sort_items(body.get(2..).unwrap_or(&[]))
            }
            net::mobmove::MOB_MOVE_REQUEST => return self.on_mob_move(body.get(2..).unwrap_or(&[])),
            // CONFIRM in the KEY BINDINGS dialog. Stored, answered with nothing - see
            // session/keymap.rs for why silence is safe here and how that was measured.
            net::keymap::CLIENT_KEYMAP_CHANGE => {
                return self.on_keymap_change(body.get(2..).unwrap_or(&[]))
            }
            // Sitting down and standing up. BOTH still return the exclusive-request unlock -
            // see session/chair.rs; without it the client cannot even ask to stand.
            // The trade invite. Answered with a real packet on the invite arm and with
            // nothing elsewhere - see session/trade.rs; 0x017E does not latch.
            net::trade::CLIENT_MINIROOM => {
                return self.on_miniroom(body.get(2..).unwrap_or(&[]))
            }
            // Player stores: the room and the shelf. Both latch, and every answer releases the
            // latch - see session/playershop.rs.
            net::playershop::CLIENT_SHOP_ROOM => {
                return self.on_shop_room(body.get(2..).unwrap_or(&[]))
            }
            net::playershop::CLIENT_SHOP_SHELF => {
                return self.on_shop_shelf(body.get(2..).unwrap_or(&[]))
            }
            net::playershop::CLIENT_HIRED_CHECK => {
                return self.on_hired_check(body.get(2..).unwrap_or(&[]))
            }
            net::chair::CLIENT_CHAIR_SIT => {
                return self.on_chair_sit(body.get(2..).unwrap_or(&[]))
            }
            net::chair::CLIENT_CHAIR_CANCEL => {
                return self.on_chair_cancel(body.get(2..).unwrap_or(&[]))
            }
            // The client telling us where it walked. **Answered with nothing, deliberately**
            // - 1082 of these went unanswered across every captured session and the client
            // played on for minutes, so this is not one of the packets that latches.
            //
            // `x`/`y` are the END of the path, not its start: the head's coordinates lag by
            // one report and were measured 14-50 px behind while running, and 50 px is twice
            // the width of the client's pick-up box. The end is exact when standing still,
            // which is the case a drag out of the inventory window is.
            // The Crafting Journal. Two packets per item - see session/craft.rs - and every
            // mode is answered, including the ones this server does not model: the client
            // latches its request flag on send and never crafts again until it is cleared.
            // The friend window. Every sub-op is answered, including the ones this server
            // does not model yet - see session/friends.rs.
            net::friends::CLIENT_FRIEND_REQUEST => {
                return self.on_friend_request(body.get(2..).unwrap_or(&[]))
            }
            net::craft::CLIENT_CRAFT_REQUEST => {
                return self.on_craft_request(body.get(2..).unwrap_or(&[]))
            }
            net::skills::CLIENT_USER_SKILL_UP_REQUEST => {
                return self.on_skill_up(body.get(2..).unwrap_or(&[]))
            }
            // **The buff request.** Unlike almost everything else here it does NOT latch -
            // the 2026-08-22 cast went entirely unanswered and the client played on for four
            // more minutes - so a refusal may be a chat line. See session/buff.rs.
            net::buff::CLIENT_SKILL_USE => {
                return self.on_skill_use(body.get(2..).unwrap_or(&[]))
            }
            // Right-click on a buff icon. It **retries every ~180 ms** until answered - the
            // 2026-08-22 run took fourteen of them in three seconds - so this is not one of
            // the packets that can be left alone.
            net::buff::CLIENT_SKILL_CANCEL => {
                return self.on_skill_cancel(body.get(2..).unwrap_or(&[]))
            }
            // The Cash Shop button. An EXCLUSIVE REQUEST - it latches ctx+0x2330 on send, so
            // leaving it unanswered costs every later click of the session AND the pick-up
            // sweep, which gates on the same field. See session/cashshop.rs.
            net::cashshop::CLIENT_CASH_SHOP_REQUEST => {
                return self.on_cash_shop_request(body.get(2..).unwrap_or(&[]))
            }
            // "What is my balance." Empty body, and the client throttles it to once a minute.
            net::cashshop::CLIENT_CASH_SHOP_QUERY => return self.on_cash_shop_query(),
            // Everything else done inside the shop - Buy above all. Each of the six builders
            // sets [stage+0x74] before it sends and the shop's UI blocks until something
            // clears it, so this is one of the packets "always answer" is really about: the
            // first Buy click would otherwise kill every later click of the session.
            net::cashshop::CLIENT_CASH_SHOP_ACTION => {
                return self.on_cash_shop_action(body.get(2..).unwrap_or(&[]))
            }
            // **Two AP opcodes, not one.** 0x0138 is a single + click and 0x0139 is the
            // bulk dialog; answering only the second still looks broken to anyone using the
            // button, which is what happened. Both latch ctx+0x2330 on send and only a
            // 0x007C clears it, so both are answered on every path - see session/ability.rs.
            net::abilityup::CLIENT_ABILITY_UP => {
                return self.on_ability_up(body.get(2..).unwrap_or(&[]))
            }
            net::abilityup::CLIENT_ABILITY_MASS_UP => {
                return self.on_ability_mass_up(body.get(2..).unwrap_or(&[]))
            }
            // Double-clicking a potion. Answered on every path, including refusals: there
            // was exactly ONE 0x010E in a run where the owner used more than one item, which is
            // the same request-latch signature 0x0107 and the AP requests have.
            net::useitem::CLIENT_USE_ITEM => {
                return self.on_use_item(body.get(2..).unwrap_or(&[]))
            }
            // A Return Scroll: the client's own opcode for the 0203 range, same body as
            // 0x010E, same latch. The handler has answered scrolls since 2026-08-29; this arm
            // is what makes it reachable (2026-09-18). session/consume.rs.
            net::useitem::CLIENT_USE_RETURN_SCROLL => {
                return self.on_use_return_scroll(body.get(2..).unwrap_or(&[]))
            }
            // A pet's Auto HP / Auto MP drinking the owner's potion - the same walk, and the
            // same latch (2026-09-25). session/consume.rs.
            net::useitem::CLIENT_PET_USE_ITEM => {
                return self.on_pet_use_item(body.get(2..).unwrap_or(&[]))
            }
            net::userhit::CLIENT_USER_HIT => {
                return self.on_user_hit(body.get(2..).unwrap_or(&[]))
            }
            net::usermove::CLIENT_USER_MOVE => {
                let payload = body.get(2..).unwrap_or(&[]);
                if let Some(m) = net::usermove::parse_user_move(payload) {
                    self.note_own_position(m.x, m.y, m.move_action);
                    self.note_activity();
                    // **The packet the whole bus exists for.** The owner, 2026-08-29:
                    // *"the client's own movement is completely disregarded ...
                    // their movements and their attacks need to be broadcasted
                    // and shown on all clients."* This is the movement half.
                    //
                    // Still no reply to the mover: 1082 captured `0x00D9`s went
                    // unanswered with the client playing on for minutes
                    // afterwards, so it does not latch the way `0x0107` does.
                    // The broadcast goes to everyone *else*.
                    self.publish_user_move(&m, payload);
                }
                // **The pet's settle, on the first move after a field entry.** session/pet.rs.
                // And a waiting gift's box, the same way. session/giftdrop.rs.
                let mut out = self.pet_settle_replies();
                out.extend(self.gift_drop_replies());
                return out;
            }
            // **Party requests are answered, even though there is no party system.** One
            // archived `0x0182` exists - the owner pressing Create - and the log line beside it
            // says "is not answered yet". That is the frozen-UI failure `CLAUDE.md` opens
            // with, so both party opcodes get a specific refusal. `session::party`.
            net::party::CLIENT_PARTY_REQUEST | net::party::CLIENT_PARTY_INVITE_ANSWER => {
                return self.on_party_request(opcode, body.get(2..).unwrap_or(&[]))
            }
            // Party chat. `session::groupchat`; the send sets no latch, so an unbuilt kind
            // may go unanswered.
            net::groupmessage::CLIENT_GROUP_MESSAGE => {
                return self.on_group_message(body.get(2..).unwrap_or(&[]))
            }
            // Whispers and /find. `session::whisper`.
            net::whisper::CLIENT_WHISPER => return self.on_whisper(body.get(2..).unwrap_or(&[])),
            // Maple Chat. `session::messenger`.
            net::messenger::CLIENT_MESSENGER => return self.on_messenger(body.get(2..).unwrap_or(&[])),
            net::notice::CLIENT_LOG_OUT => return self.on_log_out(),
            net::script::CLIENT_SCRIPT_REPLY => {
                return self.on_script_reply(body.get(2..).unwrap_or(&[]))
            }
            net::script::CLIENT_NPC_CLICK => {
                return self.on_npc_click(body.get(2..).unwrap_or(&[]))
            }
            net::channel::CLIENT_CHANGE_CHANNEL => {
                return self.on_change_channel(body.get(2..).unwrap_or(&[]))
            }
            // **`0x00F5` is the CLASSIC counter's request opcode**, and it is the one this
            // server will actually receive: the Shop2 window whose `0x0104` the arm below
            // answers is never opened, because its art does not exist in this client.
            net::classicshop::CLIENT_CLASSIC_SHOP_REQUEST => {
                return self.on_classic_shop_request(body.get(2..).unwrap_or(&[]))
            }
            // Kept answering although nothing can now open the window that sends it. It costs
            // one arm, and the alternative is an unanswered request if that assumption is ever
            // wrong - which is the failure this project has paid for most often.
            net::shop::CLIENT_SHOP_REQUEST => {
                return self.on_shop_request(body.get(2..).unwrap_or(&[]))
            }
            // The pick-up request, whichever of the six it turns out to be. Its opcode
            // **cannot be found statically** - the chain runs into the Themida VM - so the
            // arm accepts the whole unclaimed range and lets one walk over one drop name it.
            // Answering all six is safe: none is built by anything in the image, and a body
            // that carries no live drop id gets a notice rather than a guess.
            op if crate::drops::may_be_the_pick_up_request(op) => {
                return self.on_pick_up(op, body.get(2..).unwrap_or(&[]))
            }
            // **`0x0143` is the meso drop, and it LATCHES.** The owner, 2026-09-08: *"I have
            // attempted to drop 10 mesos and 5000 mesos, none of these attempts worked, but I
            // lose all functionality in being able to interact with my inventory."* The
            // client's builder `FUN_142d4cb40` sets `player+0x2330` through `142cc4430` the
            // moment it sends, so an unanswered one does not fail a drop - it kills the
            // inventory, the ability-point buttons, the cash shop and the item drop for the
            // rest of the session. **Mesos CAN be dropped now** (2026-09-09) - the owner: *"I still
            // cannot drop mesos"*, and they were right, because the 09-08 fix only stopped the
            // freeze. `Session::on_drop_money` places the coins; `crate::mesodrop::refuse`
            // handles every path that does not, and still answers.
            net::dropmoney::CLIENT_DROP_MONEY => {
                return self.on_drop_money(body.get(2..).unwrap_or(&[]));
            }
            // **`0x0125` is the client's own scrolling window**, and it latches too. The owner,
            // 2026-09-09: *"Just tried scrolling the topwear, it did not work."* It did not:
            // the opcode was decoded in full that day and never handled, so it fell through
            // to the unlock arm below and the player got a cleared latch and nothing else -
            // four times in one run, at 00:59:09 and 00:59:13 in `world.log`.
            //
            // `on_item_upgrade` answers every path, including the refusals: `result = 3`
            // exists precisely so a refusal is still an answer.
            net::upgrade::CLIENT_ITEM_UPGRADE => {
                return self.on_item_upgrade(body.get(2..).unwrap_or(&[]));
            }
            // **`0x0126` - the Lucky Day Scroll dragged onto an equip** (2026-10-01). The same
            // body and the same latch as `0x0125`; `session/realscroll.rs`.
            net::upgrade::CLIENT_ITEM_ENHANCER => {
                return self.on_item_enhancer(body.get(2..).unwrap_or(&[]));
            }
            // **`0x0111` is the summoning sack**, and it latches for the same reason.
            // The owner, 2026-09-09: *"I just also tried summoning the GM Black Sack Jr. Balrog
            // lvl 80"* - it arrived and fell through to the unlock arm below, exactly as
            // `research/summon-sacks-2026-09-09.md` predicted it would.
            net::summon::CLIENT_SUMMON_SACK => {
                return self.on_summon_sack(body.get(2..).unwrap_or(&[]));
            }
            // **`0x0114` is a Cash-tab item**, and it latches like the rest of the family.
            // The owner, 2026-09-09: *"I just tried using the Equip expansion coupon"* and *"Using
            // the AP and SP reset cash items also does not perform the function."* Both are
            // this opcode; their own capture named it after the plan asked for one click.
            net::cashitem::CLIENT_USE_CASH_ITEM => {
                return self.on_use_cash_item(body.get(2..).unwrap_or(&[]));
            }
            // The AP and SP Reset Scrolls come through their own opcode, same body. The owner's
            // two presses on 2026-09-10 were both 0x0116 and both went unanswered.
            net::cashitem::CLIENT_USE_STAT_RESET_ITEM => {
                return self.on_use_stat_reset_item(body.get(2..).unwrap_or(&[]));
            }
            // The Beauty Coupon dialog's Confirm. The owner's Übel Hair press arrived twice on this
            // opcode on 2026-09-12 and nothing answered it.
            net::beautycoupon::CLIENT_BEAUTY_COUPON_CONFIRM => {
                return self.on_beauty_coupon_confirm(body.get(2..).unwrap_or(&[]));
            }
            // A double-click on another player: their Character Info. Both client-side
            // builders set the same latch, so this is answered on every path. session/charinfo.rs.
            // **`0x02EB` - the client's options changed.** Kept per account and sent back in
            // the record at every field entry; nothing is expected back. session/options.rs.
            net::clientsettings::CLIENT_OPTIONS_CHANGED => {
                return self.on_options_changed(body.get(2..).unwrap_or(&[]));
            }
            net::charinfo::CLIENT_CHARACTER_INFO_REQUEST => {
                return self.on_character_info_request(body.get(2..).unwrap_or(&[]));
            }
            // The fame arrows on that window. session/fame.rs.
            net::fame::CLIENT_GIVE_FAME => return self.on_give_fame(body.get(2..).unwrap_or(&[])),
            // Anything else whose CLIENT-SIDE builder sets that same exclusive-request latch.
            // Not implemented, but silence here freezes the UI, so it gets the nine-byte
            // unlock and nothing else - an empty mask says nothing about any subsystem.
            // Placed last so it can only fire on opcodes no specific arm above answers.
            //
            // Deliberately a measured whitelist rather than "answer everything unknown":
            // blanket-answering would send a stat change in reply to 750 archived environment
            // reports and 23 261 mob moves, for which returning nothing is correct. Three
            // opcodes have arrived unhandled in the archive and frozen a client - `0x0143`,
            // `0x01FD` and `0x02F6` - and this arm would have caught all three.
            op if net::dropmoney::latches_the_exclusive_request(op) => {
                return crate::mesodrop::unlock_unhandled_latching_request(op)
            }
            // The client's one-way reports (0x013D census, 0x01ED log channel, the
            // 0x0420..0x0426 leaving burst, and the rest of net::names::is_client_report).
            // Answered with nothing ON PURPOSE - 0x013D must not be answered - and routed
            // through session/reports.rs so the choice is a decision, not a fall-through,
            // and so the two that carry something readable get a log line. Placed after
            // the latch whitelist, which none of them is in (tests pin both facts).
            op if net::names::is_client_report(op) => {
                return self.on_client_report(op, body.get(2..).unwrap_or(&[]))
            }
            _ => return Vec::new(),
        }
        // Never log in onto a party-quest stage: the Exit instead. Before the record is
        // read, so the SetField below carries it. session/firsttime.rs.
        self.keep_out_of_party_quest_on_login();
        // Nor onto a ship: Ellinia Station instead. session/boat.rs.
        self.keep_off_ship_on_login();
        // Nor onto a map the client has no field for: it crashes. session/field.rs.
        self.keep_off_missing_map_on_login();
        // Always answer. An unanswered packet freezes the client's whole UI - every
        // button, including the quit prompt - and reads on screen as a crash. So a
        // character we cannot load falls back to the minimal record rather than silence.
        let (body, what) = match self.claimed_character() {
            Some(chr) => {
                // The first announcement to the field stands at the spawn point the record
                // names, not at the origin - the same rule as a portal walk in `go_to_map`,
                // for the same reason: a `0x0224` at `(0, 0)` that self-heals on the first
                // step is a visible snap on every other screen. `None` when unknown.
                if self.last_position.is_none() {
                    self.last_position = self.config.portal_positions.get(&(chr.map_id, chr.portal)).copied();
                    self.last_move_action = self.last_position.map(|_| net::userpool::MOVE_ACTION_LANDING);
                }
                let (quests, quest_note) = self.quest_book(chr.id);
                let skills = self.store.skills(chr.id).unwrap_or_default();
                (
                net::opcode::set_field_with_character_dressed_quests(
                    &chr,
                    self.config.world_id,
                    self.clock_base(),
                    self.config.channel_id,
                    &self.dressed(&chr),
                    &quests,
                    &skills,
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
        // The saved key layout, AFTER the SetField and never before it. A reply sequence is a
        // script the client walks in order, and the keymap manager belongs to the stage this
        // SetField is building. session/keymap.rs.
        let mut out = vec![Reply { opcode: net::opcode::SET_FIELD, body, what }];
        out.extend(self.keymap_replies());
        // The skill-point pools, after the SetField for the same reason as the keymap: the
        // record's stat block carries an empty SP table, so the login field's skill window
        // would read 0 in every pool. This is the login-time SetField; `go_to_map` does the
        // same on a portal walk and a channel change. session/skills.rs.
        if let Some(chr) = self.claimed_character() {
            out.extend(self.skill_point_reply(&chr));
        }
        // And the pet's long-range pickup box. session/pet.rs.
        out.push(self.pet_pickup_range_reply());
        // The buffs the last channel was holding, re-sent and held here so this channel's tick
        // takes them down. Nothing on a login from character select. session/buffcarry.rs.
        out.extend(self.carry_buffs_in());
        out
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


    /// The character this connection claimed a migration for.
    ///
    /// `describe_hello` in `server.rs` claims the migration before `handle` runs, so by
    /// this point `claimed` is populated for a well-formed hello. The store has no
    /// lookup by character id alone, but a claim carries the account and world, and a
    /// character id is unique within those.
    /// **Which field this character is standing in** - the map, and which copy of it.
    ///
    /// `instance` is 0 everywhere except inside a party quest, where it is the id of the run
    /// the character is on. Derived rather than cached: `is_quest_map` is a range check and
    /// runs first, so the registry lock is only taken on the seven quest fields, and there
    /// is no cached copy to go stale when somebody is warped out by the timer or by leaving
    /// the party.
    pub(super) fn field_of(&self, chr: &net::opcode::Character) -> crate::fields::FieldKey {
        // **The three ship fields are instanced by voyage** - the same derivation, from
        // `crate::boat`'s registry instead of the runs'. Off a voyage, the shared copy.
        if crate::boat::is_ship_map(chr.map_id) {
            let voyage = self.fields.voyages().voyage_of(chr.id);
            return match voyage {
                Some(v) => crate::fields::FieldKey::instanced(chr.map_id, v.id),
                None => crate::fields::FieldKey::world(chr.map_id),
            };
        }
        if !crate::firsttime::is_quest_map(chr.map_id) {
            return crate::fields::FieldKey::world(chr.map_id);
        }
        let run = self.fields.runs().instance_of(chr.id);
        match run {
            Some(run) => crate::fields::FieldKey::instanced(chr.map_id, run.id),
            // On a quest map but in no run - a GM who walked in with !map, or somebody whose
            // run ended under them. The shared copy of the field is the honest answer.
            None => crate::fields::FieldKey::world(chr.map_id),
        }
    }

    /// [`Session::field_of`] for the claimed character. Prefer `field_of` where the caller
    /// already holds one: this re-reads the character from the store. Tests only today.
    #[cfg(test)]
    pub(super) fn field(&self) -> crate::fields::FieldKey {
        match self.claimed_character() {
            Some(chr) => self.field_of(&chr),
            None => crate::fields::FieldKey::default(),
        }
    }

    fn claimed_character(&self) -> Option<net::opcode::Character> {
        let claimed = self.claimed.as_ref()?;
        let mut chr = self
            .store
            .characters_for(claimed.account_id, claimed.world_id)
            .ok()?
            .into_iter()
            .find(|c| c.id == claimed.character_id)?;
        // **The spawn point they come back in at** (the owner, 2026-09-26): the one recorded when
        // they last left - nearest to where they stood - but only on the map it was recorded
        // on; a portal index means nothing on any other map. `store::spawnpoint`. Applied here
        // so every re-entry (login, channel change, leaving the Cash Shop) agrees.
        if let Ok(Some((map, portal))) = self.store.spawn_point(chr.id) {
            let known = self.config.portal_positions.is_empty() || self.config.portal_positions.contains_key(&(map, portal));
            if map == chr.map_id && known {
                chr.portal = portal;
            }
        }
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
        use store::migration::{ClaimOutcome, MigrationEvidence, PeerPolicy};

        // **What this connection can prove, and it is no longer nothing.**
        //
        // Nothing on the wire changed and nothing on the wire can: the client does not carry
        // the seed back - measured 2026-08-29, 115 distinct hello bodies against all 74 seeds
        // ever minted, plain and both endiannesses and the XOR form the decompiler predicts,
        // 8510 trials, zero hits, with the character id at offset 8 passing as a positive
        // control on all 115 - and its identity block is per-machine rather than per-launch.
        //
        // What changed is that the SERVER can derive the credential instead of being handed
        // one: peer address -> the process the kernel's TCP table says owns that socket -> the
        // login claim the launcher registered for that process -> that claim's token_hash.
        // `store::Store::attest_channel_connection` is that walk, and it is the only thing
        // that can mint the `AttestedTokenHash` `with_token_hash` demands - a hash out of a
        // packet body cannot reach it, which is a type error rather than a review comment.
        //
        // **The outcome is logged every time, attested or not.** A silent fallback is the
        // exact failure this feature exists to prevent, and there is a second reason: a run
        // made with `--bind-migrations` still OFF at the login server now measures whether
        // turning it on would work, so the flag can be flipped on evidence rather than hope.
        //
        // The address is still recorded and still never decisive - two clients on one machine
        // share it - so it is attached to whatever the attestation produced rather than being
        // the attestation.
        let (attested_note, evidence) = match self.peer_addr {
            Some(addr) => {
                let attestation = self.store.attest_channel_connection(addr);
                let why = attestation.why();
                let evidence = match attestation {
                    store::migration::Attestation::Attested(hash) => {
                        MigrationEvidence::with_token_hash(hash)
                    }
                    // Every other outcome presents NO credential. That refuses a bound row and
                    // is a no-op for an unbound one, and it is deliberate in both directions: a
                    // fallback here would let an unattributable connection past the binding by
                    // simply being unattributable, which is the hole the binding closes.
                    _ => MigrationEvidence::none(),
                };
                (why, evidence)
            }
            None => (
                "NOT ATTESTED: this session has no socket address, so the owning process \
                 could not be looked up at all. A bound migration will be REFUSED. On the \
                 server path this means Session::with_peer_addr was not called - see \
                 crates/world/src/server.rs; in a test it is simply the default."
                    .to_string(),
                MigrationEvidence::none(),
            ),
        };
        let evidence = match self.peer.as_deref() {
            Some(p) => evidence.from_peer(p),
            None => evidence,
        };

        let mut accept = |claimed: store::ClaimedMigration, how: &str, mismatch: bool| {
            let wrong_channel = claimed.world_id != self.config.world_id
                || claimed.channel_id != self.config.channel_id;
            let mut note = format!(
                "claimed the migration for character {} of account {} {how} (world {} channel {})",
                claimed.character_id, claimed.account_id, claimed.world_id, claimed.channel_id
            );
            if mismatch {
                note.push_str(
                    " - PEER MISMATCH: this connection's address differs from the one the                      migration was minted for. Not refused (two clients on one machine share                      an address, and a dual-stack client changes it legitimately), but worth                      reading if impersonation is suspected"
                );
            }
            if wrong_channel {
                note.push_str(&format!(
                    " - WRONG CHANNEL: this is world {} channel {}",
                    self.config.world_id, self.config.channel_id
                ));
            }
            self.log_name = self.store.character_brief(claimed.character_id).ok().flatten().map(|c| c.name);
            self.claimed = Some(claimed);
            note
        };

        // The attestation sentence goes in FRONT of whatever happened next, so `world.log`
        // records what this connection could present *and* what it got, on one line, in that
        // order. Reading only the second half is how "REFUSED" becomes an unexplained outage.
        //
        // **A prefix rather than a replacement, and that is what keeps it safe.** Every
        // existing assertion on this string is `.contains(..)` - there is no `assert_eq!` on
        // it anywhere in the crate - so prefixing moves no test. Appending would have been
        // equally safe; leading with what the connection could prove is the useful order.
        // The address policy is the channel's config: Require by default since 2026-09-05.
        // For an off-box client the address is the only fact this connection shares with the
        // login connection that minted the row, so requiring it IS the enforcement there.
        let policy: PeerPolicy = self.config.peer_policy;
        let outcome = match self.store.claim_migration_for_character_with(character_id, &evidence, policy) {
            Ok(ClaimOutcome::Claimed { migration, peer_mismatch }) => {
                accept(migration, "by character id", peer_mismatch)
            }

            // **A refusal is never retried as a weaker claim.** Falling through to the
            // channel route here would be exactly the downgrade the binding exists to
            // prevent: the row demanded a credential and this connection did not have it.
            Ok(ClaimOutcome::Refused(why)) => format!(
                "REFUSED the migration for character {character_id}: {why}. The client is                  still answered - it gets the minimal SetField - because an unanswered packet                  freezes its entire UI. It will NOT enter the world."
            ),

            // A channel migration cannot be claimed by character id - `0x001A` is
            // `u8 ok, u32 ip, u16 port` and carries no character - so this is the ordinary
            // channel-change case, not an error.
            Ok(ClaimOutcome::NoMigration) => match self.store.claim_sole_migration_for_channel_with(
                self.config.world_id, self.config.channel_id, &evidence, policy,
            ) {
                Ok(ClaimOutcome::Claimed { migration, peer_mismatch }) => accept(
                    migration,
                    &format!(
                        "by CHANNEL, not by character id - the hello said {character_id}, which                          a channel migrate cannot carry"
                    ),
                    peer_mismatch,
                ),
                Ok(ClaimOutcome::Refused(why)) => format!(
                    "REFUSED the sole pending migration on this channel: {why}. Answered with                      the minimal SetField; the character does not enter the world."
                ),
                Ok(ClaimOutcome::NoMigration) => format!(
                    "character {character_id} has no unconsumed migration, and this world and                      channel has no single pending one to fall back on - it was never minted,                      or already claimed, or it expired, or there is more than one and                      ambiguity is refused rather than guessed"
                ),
                Err(e) => format!("character {character_id} could not be checked: {e}"),
            },
            Err(e) => format!("character {character_id} could not be checked: {e}"),
        };
        // The pet that was out at the last log-out is out again. Here, after the claim has
        // settled and before the login SetField is built, so that record's Cash item already
        // says `active = 1` and the first field entry re-summons it. session/pet.rs.
        self.restore_active_pet();
        // **A trade a crash interrupted gives its offer back** - into the bag, before the
        // login record that draws the bag is built. session/trade.rs.
        self.return_trade_escrow_at_login();
        // **And a store's shelf** - the same, for a store a crash left open. session/playershop.rs.
        self.return_shop_escrow_at_login();
        // **A crafting quest finished before this server read `Act.1.skill` still counts.**
        // Here for the same reason the pet is: after the claim, before the login `SetField`,
        // so the record that builds the Crafting Journal's tabs already carries the skill.
        // `session/craft.rs`.
        self.reconcile_crafting_quests();
        // **The effect item switched on last time** (Shadow Style...), restored before the
        // first 0x0224 is built - saved between logins, the owner 2026-09-30. session/emote.rs.
        if let Some(id) = self.claimed.as_ref().map(|c| c.character_id) {
            self.active_effect_item = self.restored_effect_item(id);
        }
        format!("{attested_note} || {outcome}")
    }


    /// The migration this connection claimed, if any.
    pub fn claimed(&self) -> Option<&ClaimedMigration> {
        self.claimed.as_ref()
    }

    /// **Who this connection is, for the log** - `Wisp#215`, or `nobody` before the migration
    /// hello has claimed anyone. The owner, 2026-09-14: *"Channel logs need to log the character
    /// who is sending those opcodes, the server reply opcodes also need to log which character
    /// this opcode is destined for."* Cached name, one store read per claim, so a packet line
    /// costs no query.
    pub fn log_tag(&self) -> String {
        match (&self.claimed, &self.log_name) {
            (Some(c), Some(name)) => format!("{name}#{}", c.character_id),
            (Some(c), None) => format!("#{}", c.character_id),
            _ => "nobody".to_string(),
        }
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
