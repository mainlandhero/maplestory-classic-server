//! **The user pool - the packets that make one client draw *another* player.**
//!
//! Everything a second player on the field is made of arrives through `CUserPool`, the
//! singleton at `[0x143AC1B90]`, on inbound opcodes `0x0224..=0x039F`. This file holds the
//! opcode constants and the two bodies that are actually established. It is deliberately
//! thin: two sibling agents are decoding the rest as this is written, and **a table row
//! written from a quick read is a claim** - `CLAUDE.md` records `0x0467` being called "a
//! template preload list" for days when it is `SetNpcScriptable`.
//!
//! Tags: **[L]** read off a listing, a table or a capture; **[D]** derived from two or more
//! of those; **[I]** inferred.
//!
//! # NOTHING HERE IS WIRED, AND NOTHING HERE HAS EVER BEEN SEEN TO WORK
//!
//! Per `CLAUDE.md` § "Built is not wired", loudly, because on screen this is indistinguishable
//! from the code not existing:
//!
//! * Nothing in `crates/world/` sends any opcode in this range.
//! * **CORRECTED 2026-08-31. This used to read "no packet in `0x224..0x39F` has ever been
//!   observed to do anything", and that is false.** Three opcodes in this range are in the
//!   archive and two of them are recorded working *on the owner's screen*:
//!
//!   ```text
//!     0x02D1   49 archived files   the quest-finish fanfare, and the blue recovery number
//!     0x0315   18                  the revive dialog
//!     0x0231    4                  the chat line - it really did draw nothing
//!   ```
//!
//!   **[L]**, and the counts discriminate: an opcode this server never sends returns 0 files
//!   by the same grep. `STATUS.md` records the first two as confirmed on screen.
//!
//!   The old sentence mattered because it was the reason the *whole* remote family looked
//!   unproven. It is not: the route above the pool is live, and the only hop still unverified
//!   is whether `0x0224` puts a `CUser` in the pool - which is what the first two-client run
//!   tests and the one thing no amount of body work can substitute for.
//!
//!   The mistake is this project's most familiar one wearing a range: *"nothing new arrived"*
//!   is a different claim from *"this thing did not arrive"*, and only the second is worth
//!   making. `research/user-chat-round2.md` §1 owns the `0x0231` half, which still stands.
//! * The route *above* the pool is proved live - `0x3C6..0x44E` (mobs) and `0x44F..0x468`
//!   (NPCs) are routed by the same range chain in the same function and both arrive - so
//!   "the packet never reaches `CField::OnPacket`" is ruled out. Steps 4-6 below are not.
//!   **[L]**
//!
//! # The route, every hop with its address
//!
//! ```text
//! 141821e24  CField::OnPacket FUN_141820080
//!            lea eax,[r9-0x224] / cmp eax,0x17b / ja      ->  0x0224..0x039F
//!            mov rcx,[0x143AC1B90]   (NO null check)      ->  call FUN_1429b9300
//!
//! FUN_1429b9300, the pool router:
//!   0x0224, 0x0225                inline in the router itself
//!   0x0226..0x0292             -> FUN_1429bafb0    (1429b932e: lea eax,[rdx-0x226] / cmp 0x6c)
//!   0x0293..0x02C4             -> FUN_1429bb720    REMOTE: reads u32 charId, then dispatches
//!   0x02C5..0x039E             -> FUN_14289a3a0(ctx->localUser, op, pkt)   LOCAL
//! ```
//!
//! `research/user-chat-round2.md` §1 for the first two rows, `research/level-up.md` §6 for
//! the last two. All **[L]**.
//!
//! ## Two silent drops sit in front of every handler in `0x0226..0x0292`
//!
//! `FUN_1429bafb0` reads the leading `u32` itself at `0x1429bb08d` - so **that field is
//! shared by every opcode in its range and is not part of any handler's own body** - and then
//! calls `CUserPool::GetUser(id)` at `0x1429bb099`. **A null result returns having done
//! nothing** (`cmp esi,0x226 / jne 0x1429bb5b0`), and `0x0226` is the single opcode
//! special-cased before that test. **[L]**
//!
//! That is why [`USER_ENTER_FIELD`] comes first in any multiplayer sequence and why it is
//! the one thing in this file that cannot be guessed: until the pool holds a `CUser` with our
//! id, everything else in the range is dropped in silence.
//!
//! # These opcode numbers are also OUTBOUND opcodes, and they are unrelated
//!
//! `research/msexe-send-opcodes.txt` lists client->server builders for `0x0225`
//! (`FUN_141a00170`, `FUN_141a00910`), `0x0226` (`FUN_1428f4eb0`), `0x02A5`
//! (`FUN_142e1a330`) and `0x02AF` (`FUN_1428df800`). **[L]** The two directions are separate
//! namespaces; a grep of a capture for `0x0225` will find both, and only the `<-` lines are
//! the client sending. `0x0224` is the one constant here with no outbound twin.

/// First inbound opcode routed to `CUserPool`. `141821e24 lea eax,[r9-0x224]`. **[L]**
pub const USER_POOL_FIRST: u16 = 0x0224;

/// Last inbound opcode routed to `CUserPool`: `0x224 + 0x17b`, from the `cmp eax,0x17b / ja`
/// that follows. **[L]**
pub const USER_POOL_LAST: u16 = 0x039F;

/// First opcode the router hands to `FUN_1429bafb0`. `1429b932e lea eax,[rdx-0x226]`. **[L]**
pub const USER_POOL_BY_ID_FIRST: u16 = 0x0226;

/// Last of the same: `0x226 + 0x6c`, from `cmp eax,0x6c / ja`. **[L]**
pub const USER_POOL_BY_ID_LAST: u16 = 0x0292;

/// First opcode `FUN_1429bb720` - the **remote-user** handler - serves.
/// `research/level-up.md` §6. **[L]**
pub const USER_POOL_REMOTE_FIRST: u16 = 0x0293;

/// Last of the same. **[L]**
pub const USER_POOL_REMOTE_LAST: u16 = 0x02C4;

/// **The remote table's index base, which is NOT [`USER_POOL_REMOTE_FIRST`].**
///
/// `FUN_1429bb720` computes `index = opcode - 0x29e` into the table at `0x1429bbc34`
/// (`research/level-up.md` §6, corroborated independently in `research/user-hit.md` §5.3).
/// **[L]** for the base and the table; the eleven opcodes `0x0293..0x029D` between the range
/// start and the index base are **unexplained** - either the function bails on them before
/// the table or the range in the routing block is wider than the table it feeds. Nobody has
/// read that. **Do not reconcile the two numbers by assuming one of them; they are recorded
/// here disagreeing on purpose.** **[I]** that it matters at all.
pub const USER_POOL_REMOTE_TABLE_BASE: u16 = 0x029E;

/// First opcode routed to the **local** user, `FUN_14289a3a0(ctx->localUser, op, pkt)`, whose
/// own table at `0x14289d660` is indexed `opcode - 0x2c5`. `research/level-up.md` §6. **[L]**
pub const USER_POOL_LOCAL_FIRST: u16 = 0x02C5;

/// Last of the same. Note it stops one short of [`USER_POOL_LAST`]; that is what the routing
/// block records and it has not been re-read. **[L]**
pub const USER_POOL_LOCAL_LAST: u16 = 0x039E;

/// Is this opcode routed to `CUserPool` at all?
pub fn is_user_pool(opcode: u16) -> bool {
    (USER_POOL_FIRST..=USER_POOL_LAST).contains(&opcode)
}

/// **`0x0224` `UserEnterField` - spawn a remote player. THE BODY IS NOT DECODED HERE.**
///
/// # What is established
///
/// * The opcode is handled **inline in `FUN_1429b9300`**, before the `lea eax,[rdx-0x226]`
///   that sends `0x0226..0x0292` to `FUN_1429bafb0`. `research/level-up.md` §6. **[L]**
/// * `research/user-chat-round2.md` §7 names `FUN_1429ba3e0` as its handler and calls it
///   *"putting our own character into the user pool"*. That is a handler address plus a
///   sentence, **not a read of the body**.
/// * The ordering `0x224` enter / `0x225` leave / `0x226` chat matches the classic
///   `CUserPool` enum exactly, which is what `research/talking-back.md` §1.3 rests its
///   `0x0226 = UserChat` identification on. **[D]**, and it is the same [D] both these
///   constants inherit.
///
/// # What is NOT established, and why there is no builder
///
/// **The body.** Nobody has counted the reads in `FUN_1429ba3e0`. In this client a remote
/// player's spawn carries an avatar look block, and `crate::mob`'s history is the warning:
/// a body one byte short of what the handler reads desynchronises the stream with no length
/// prefix and no resync point, and `crate::userchat`'s two crashes were exactly that.
///
/// **A builder invented from the v214 reference would be a candidate, not a fact** -
/// `CLAUDE.md` scores that tree at 1 of 8 against a held-out control.
///
/// The answer will land in **`research/user-enter-field.md`**, which does not exist yet;
/// another agent is reading the handler as this is written. When it does, the builder goes
/// here and this doc block gets replaced rather than added to.
pub const USER_ENTER_FIELD: u16 = 0x0224;

/// **`0x0225` `UserLeaveField` - remove a remote player from the pool.**
///
/// Handled inline in `FUN_1429b9300` alongside [`USER_ENTER_FIELD`] (`research/level-up.md`
/// §6). **[L]** for the routing.
///
/// **The name is [D], from the enum ordering** described on [`USER_ENTER_FIELD`], not from a
/// read of the handler. See [`user_leave_field`] for what that costs.
pub const USER_LEAVE_FIELD: u16 = 0x0225;

/// **`0x0226` `UserChat` - the balloon over a character and the chat line.**
///
/// The same constant as [`crate::userchat::USER_CHAT_TWO_STRINGS`], restated here because
/// this is where the routing lives; a test below pins the two together.
///
/// Body: `u32 userId` - **read by `FUN_1429bafb0`, not by the handler** - then
/// `u8, u32, str, str, <list>, u8, u8, u8` through `FUN_1427847a0` -> `FUN_1427834b0`.
/// `research/talking-back.md` §1.3. **[L]** for the shape, **[D]** for the routing being
/// `UserChat`.
///
/// **It is the one opcode in `0x0226..0x0292` that survives a null `GetUser`** - the special
/// case at `0x1429bb0ac` - which is why `research/user-chat-round2.md` recommends `0x0231`
/// over it until a player is really in the pool. No builder here: `crate::userchat` owns
/// this body, including the 25-byte speaker object that killed the client twice.
pub const USER_CHAT: u16 = crate::userchat::USER_CHAT_TWO_STRINGS;

/// **`0x02A5` - a remote player taking a hit.** No builder; see below.
///
/// Remote table index 7: stub `1429bb9b2` -> `FUN_1429d48c0` -> `FUN_14025d750` ->
/// `FUN_14025da80`, the HITINFO decoder, whose 44 reads mirror the 44 fields of the
/// encoder the client's own `0x00E5` uses. `0x29e + 7 = 0x2a5`. `research/user-hit.md`
/// §5.3. **[L]**
///
/// *Control for that table decode:* index 17 comes out as [`USER_EFFECT_REMOTE`], which
/// `research/level-up.md` §6 established independently. The table reproduces a known answer.
///
/// # Body: `u32 charId` then the HITINFO **echoed from the client's own `0x00E5`**
///
/// There is no builder because there is nothing to build: the 147 bytes are the client's,
/// and the server's job is to pass them on. Two things must be true of the caller and
/// neither is expressible here yet:
///
/// * **Require at least 147 bytes; do not require exactly 147.** `FUN_142907b60` appends a
///   second structure after the HITINFO at `142907c88`, so a `==` check would reject a body
///   this client can legitimately send. All three captures are exactly 147.
///   `research/user-hit.md` §8.5. **[L]**
/// * `crates/net` has **no type for the HITINFO**; `research/user-hit.md` §9 says so loudly
///   and that is still true.
pub const USER_HIT_REMOTE: u16 = 0x02A5;

/// The HITINFO the client sends and the server passes on: **147 bytes, no optional fields.**
///
/// `FUN_14025da80` decodes it with **zero branches of any kind** - 44 direct primitive calls
/// and a `ret`. So there is no gated field and no legal short form; a body of any other
/// length is a different packet. **[L]**
pub const USER_HIT_REMOTE_HITINFO_LEN: usize = 147;

/// **Where the damage number lives, and it is the server's to fill in.**
///
/// HITINFO `+0xa8`, read at `0x14025dcd0`. The handler draws the remote damage from this
/// field and gates the 1500 ms flinch on it being `> 0`.
///
/// **The client sends `0` here - in 331 of 331 event-deduplicated captured bodies** - and its
/// own builder `FUN_1428aa0a0` has no write to that slot at all. So it is a server-fill
/// field, and this is what makes a plain echo of the client's bytes wrong: it calls the
/// damage renderer with `0`, which is the MISS path, and plays no animation. Nothing errors
/// and nothing appears. **[L]**
///
/// This module used to say of `0x02A5`: *"There is no builder because there is nothing to
/// build: the 147 bytes are the client's, and the server's job is to pass them on."* That was
/// wrong in the direction that fails silently.
pub const USER_HIT_REMOTE_DAMAGE_AT: usize = 143;

/// **`0x02A5` - what every OTHER client draws when somebody is hit by a mob.**
///
/// The owner, 2026-09-03: *"when one client is getting hurt by mobs, the other clients should also
/// be displaying the damage that the client is taking and the blinking expression."*
///
/// The body is the client's own 147-byte HITINFO with [`USER_HIT_REMOTE_DAMAGE_AT`]
/// overwritten, behind a `u32 charId` the router reads at `0x1429bb745`. **A charId absent
/// from the receiver's pool jumps straight to the epilogue** - a clean no-op - which is what
/// makes broadcasting this safe. **[L]**
///
/// # `damage` is what was APPLIED, not what the client claimed
///
/// The inbound `+8` is the client's own figure. Sending that would put a number on everyone
/// else's screen that disagrees with the HP bar the hurt player is watching, because the
/// server caps and applies its own. Pass the applied amount.
///
/// # The sign, and the colour it picks
///
/// Positive. The client negates it at `0x1429d4ecb` before handing it to the renderer
/// `0x142771360`, and a negative amount there selects digit set 3 - the damage colour
/// (`research/damage-number-draw.md` §2). Passing a negative would draw a **blue recovery**
/// number instead. `0` is not "no damage": it is the MISS path, and it is what the client's
/// own bytes already carry.
///
/// # Not yet on a screen
///
/// Every field above is read off the client's listing, and the tail makes the **same four
/// calls in the same order** as the client's own hit path - the action latch, the 1500 ms
/// flinch, the 5000 ms avatar effect, then the number. That is a structural mirror, so the
/// claim that this draws is **[D]**. One two-client run falsifies it: a violet number and a
/// flinch over the hurt player on the other screen, or nothing.
///
/// Returns `None` rather than truncating or padding a body of the wrong length: a short
/// HITINFO is a decoder disagreement, and guessing at one is how three crashes started.
pub fn user_hit_remote(char_id: u32, hit_info: &[u8], damage: i32) -> Option<Vec<u8>> {
    let src = hit_info.get(..USER_HIT_REMOTE_HITINFO_LEN)?;
    let mut info = [0u8; USER_HIT_REMOTE_HITINFO_LEN];
    info.copy_from_slice(src);
    info[USER_HIT_REMOTE_DAMAGE_AT..USER_HIT_REMOTE_DAMAGE_AT + 4]
        .copy_from_slice(&damage.to_le_bytes());

    let mut w = crate::PacketWriter::new();
    w.u32(char_id);
    w.bytes(&info);
    Some(w.into_vec())
}

/// **`0x02AF` `UserEffectRemote` - `u32 charId, u8 effect`.** What every *other* client on
/// the field sees.
///
/// Remote table index 17, `0x29e + 0x11 = 0x2af`: stub `0x1429bba36` -> `FUN_1427863f0`,
/// whose first read is the `u8` effect type at `14278644e`. `tools/reads.py 0x1429bb720 1`
/// shows **exactly one direct read** before the dispatch, so the `u32 charId` the router
/// takes is the whole prefix. `research/level-up.md` §6. **[L]**
///
/// The same constant as [`crate::stats::USER_EFFECT_REMOTE`], restated here because this is
/// the file that owns the routing; a test pins them together. The builder is
/// [`user_effect_remote`].
pub const USER_EFFECT_REMOTE: u16 = crate::stats::USER_EFFECT_REMOTE;

/// Effect `0` = **LevelUp**. `FUN_1427863f0`'s second switch is
/// `ecx = [0x142791348 + effectType*4]`; index 0 is `0x14278bd99`, the arm that reads
/// `[0x143a46f48]` = `Effect/BasicEff.img/LevelUp`. **That arm contains no packet read**, so
/// the body ends at the effect byte. `research/level-up.md` §6. **[L]**
///
/// Restated from [`crate::stats::EFFECT_LEVEL_UP`] and pinned equal to it by a test.
pub const EFFECT_LEVEL_UP: u8 = crate::stats::EFFECT_LEVEL_UP;

/// Build a [`USER_EFFECT_REMOTE`] body: `u32 charId, u8 effect`. **Five bytes.**
///
/// Delegates to [`crate::stats::user_effect_remote`] rather than restating it, so the two
/// cannot drift; this exists so a caller reasoning about the user pool does not have to know
/// that the level-up effect happens to live in `stats`.
///
/// Send it to every client on the field **except** the one whose character levelled - that
/// player's own animation comes from the `0x007C` stat change, and
/// [`crate::stats::the_level_up_animation_is_client_side`] documents why.
pub fn user_effect_remote(char_id: u32, effect: u8) -> Vec<u8> {
    crate::stats::user_effect_remote(char_id, effect)
}

/// Build a [`USER_LEAVE_FIELD`] body: **one `u32` character id, and that is the whole claim.**
///
/// # This was [I] until 2026-08-29 and is now [L]
///
/// **`FUN_1429ba980` reads exactly one `u32`, at `0x1429ba9a2`, and nothing else** -
/// counted at depth 4, `research/user-enter-field.md` §1. It then hashes that id, unlinks
/// the node from `pool+0xf8` and calls its deleting destructor, which is also what settles
/// enter-versus-leave from the *bodies* rather than from enum order: `0x0224` allocates a
/// `0x4438`-byte `CUser` and inserts it.
///
/// The superseded argument is kept below, because it was the reasoning that made this
/// safe to write before anyone had read the function, and it names its own weak point.
///
/// # The old [I] argument What supports it:
///
/// * the enum ordering `0x224` enter / `0x225` leave / `0x226` chat, which
///   `research/talking-back.md` §1.3 calls a match for the classic `CUserPool` exactly, and
///   on the strength of which `0x0226` was identified as `UserChat` **[D]**;
/// * every other identified packet in this range that names a character starts with a `u32`
///   character id - `0x0226` (`FUN_1429bafb0` at `0x1429bb08d`), `0x02A5` and `0x02AF`
///   (`FUN_1429bb720` at `1429bb745`). **[L]** for those three, **[I]** for `0x0225`
///   following them, because `0x0225` is handled *inline in the router* and therefore does
///   **not** go through either of the two functions that read that prefix.
///
/// That last clause is the weak point and it is deliberately not smoothed over: the two
/// functions known to read a leading `u32` are the two this opcode never reaches.
///
/// A wrong body here is not free. `CLAUDE.md`: an unanswered or malformed packet freezes the
/// client's entire UI and reads on screen as a crash. **Send [`USER_ENTER_FIELD`] first and
/// get a remote player on screen before anyone tries removing one** - a leave for a player
/// who was never added is the one test where a wrong length and a correct length look the
/// same.
pub fn user_leave_field(char_id: u32) -> Vec<u8> {
    let mut w = crate::PacketWriter::new();
    w.u32(char_id);
    w.into_vec()
}

/// Where one character is standing, which the character record does not carry.
///
/// `Character` describes who someone is; this is where they are right now, and it comes
/// from a different place entirely - the client's own `0x00D9` movement reports
/// (`crate::usermove`), held per session. Kept separate so a caller cannot forget it: a
/// remote player built from the database alone would appear at whatever position the
/// struct defaulted to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RemoteAt {
    /// Body offset 426. **[D]** - `r8d` of the `vtable[0x118]` call at `1429ce852`.
    pub x: i16,
    /// Body offset 428. **[D]** - `r9d` of the same call, read at `1429ce85f`.
    pub y: i16,
    /// Body offset 430, `+0x6e4`. Stance and facing. **[D]**
    pub move_action: u8,
    /// Body offset 431, through `FUN_142df6c50([0x143AC18D8], v)`. **[D]**
    ///
    /// `0` is legal and means "not standing on a foothold" - the client resolves it
    /// itself. Sending a foothold id from a *different* map is not legal and is the
    /// mistake to watch for when this is wired to a stale position.
    pub foothold: i16,
}

/// The remote temporary-stat mask: **124 bytes**, all clear meaning "no buffs".
///
/// Read as one raw block at `1429ce4e4` and handed to `FUN_140a46e50` -
/// `research/buffs.md` documents the same decoder. **[L]** for the length.
///
/// **124, not 132.** The v214 reference's equivalent is 132, and that difference is the
/// single clearest evidence that the reference is a different version rather than a
/// superset - `CLAUDE.md` scores it 1 of 8. Taking the reference's number here would put
/// every byte after offset 179 eight places out, with no length prefix anywhere in the
/// body to resynchronise on.
pub const REMOTE_STAT_MASK_LEN: usize = 124;

/// **The twenty-three bytes `FUN_140a46e50` reads AFTER the mask, unconditionally.**
///
/// ```text
///   0x140a4a007  u8
///   0x140a4a024  u8
///   0x140a4a041  u32
///   0x140a4a10e  call 0x140862470   -> u32, u32, u32, u32 COUNT, then COUNT x u32
///   0x140a4a29e  u8
/// ```
///
/// So the remote temporary-stat **block is 147 bytes**, of which 124 are the mask.
/// [`REMOTE_STAT_MASK_LEN`] was never wrong; the block is not the mask.
///
/// # This sentence said SEVEN for a day, and the number below it was 7, and both were wrong
///
/// The four `u32` behind the `call` are the sixteen bytes that killed two clients the first
/// time they stood on one map - the count was read out of the avatar look's face id. The
/// prose is called out here because it is the third value this constant has had (0, then 7,
/// then 23) and each time the doc block agreed with the constant. **A comment that agrees
/// with the code it sits on is not a check on it**; what settled it was the client's own
/// throw stack, frame for frame.
///
/// # The mask gate is [L] now, and it is why the tail ends here
///
/// An 8-iteration loop at `0x140a4a1a5` dispatches `vtable[+0x30](obj, packet)` and could
/// read any amount. Two passes left it **[I]** "presumed mask-gated" and named it as their
/// blind spot. Opened: the gate is `if ((received_mask & CONST[i]) != 0)`, and **we send 124
/// zero bytes**, so `0 & x == 0` whatever `CONST[i]` holds - the answer rests on no unknown
/// at all. All eight iterations skip and the vtable call is never reached. `[L]`,
/// `research/remote-stat-mask-gating.md`.
///
/// It is not vacuously false, either: the eight constants at `0x143abd320` have popcounts
/// 324-388 and 736 of the 992 mask bits reach at least one. **The first remote buff this
/// server sends will walk into that call**, and its callees cannot be sized without a
/// runtime watch.
///
/// # Why this was missed, and it is a lesson about dumps rather than about decoders
///
/// `research/msexe-secondarystat-remote-140a46e50.txt` **declares** `0x140a46e50 ..
/// 0x140a4a58f (14143 bytes)` and **stops at `0x140a46ff7`** - 3 945 bytes on disk, 423
/// bytes into the function, about 3% of it. Its own title reads *"same mask, short list"*,
/// which sounds like a claim about the data and is a claim about the dump. Everything after
/// the truncation was invisible, including all four of these reads.
///
/// # Why the fourth byte is not conditional, when six of its neighbours are
///
/// (Read "the fourth byte" as the `u8` at `0x140a4a29e`; the `call` at `0x140a4a10e` is
/// unconditional for the same reason and by the same argument.)
///
/// `reads.py` finds reads at `0x140a4a218`, `24a`, `266`, `282`, `2d3` and `2ef` in the same
/// neighbourhood, and those *are* mask-gated. The discriminator needs no control-flow graph:
/// **an instruction is bypassable only if some jump before it targets past it, or some `ret`
/// precedes it.** In this function there are 186 jumps and **all 186 have literal targets**
/// (no `jmp rax`, no jump table), there is exactly **one `ret`, at `0x140a4a58e`**, after all
/// four reads, and `callers.py` gives one entry - 2 call sites, 0 tail jumps, 0 data
/// pointers. So a flat scan of jump targets is sound.
///
/// The gate on three of the neighbours is `0x140a4a240`, whose bytes are `74 54` - `je +0x54`
/// - targeting **`0x140a4a296`**. And `0x140a4a296` is the `mov rcx,[rsp+0x190]` that *sets
/// up* the read at `0x140a4a29e`. **The branch that skips the neighbours lands on the
/// instruction feeding this read**, so it happens whether the branch is taken or not. No jump
/// in the span targets past `0x140a4a296`.
///
/// # The precedent, one decoder over
///
/// `crates/net/src/buff.rs` records this exact error class in the sibling: `0x007E`'s body
/// was documented as 127 bytes, the owner reported *"Nimble Feet crashed the client"* with exit
/// `0xE06D7363`, and `FUN_142d56f80` turned out to read **three more** after the same mask.
/// A mask length is not a block length, twice now.
///
/// # What is [L] here and what is not
///
/// The **count** is [L] - four reads, of those widths, at those addresses. That the right
/// **values** are zeros is **[I]**: they are handed to four setters on the stat object, none
/// of which reads the packet again, and zero is what the rest of an empty stat block already
/// is. `CSecondaryStat::EncodeForRemote` would settle it, and it is **not in this binary** -
/// the only 124-byte wire writer is `FUN_142973160`, the client's outbound cancel, which
/// writes no tail at all. That negative is bounded: the scan sees a mask move only with an
/// immediate length in `r8d`.
pub const REMOTE_STAT_TAIL_LEN: usize = 23;

/// Length of a [`USER_ENTER_FIELD`] body with an **empty** name and **no** equipped
/// items. See [`user_enter_field_len`] for the real one.
///
/// The last field is the `u32` count at offset 504, so the body ends at 508. Every one
/// of the 60-odd offsets between 0 and 504 chains exactly, which is the check that this
/// number is a total rather than a guess.
pub const USER_ENTER_FIELD_MIN_LEN: usize = 508 + REMOTE_STAT_TAIL_LEN;


/// Byte offset of the avatar look inside a [`USER_ENTER_FIELD`] body, for an empty name.
///
/// `1429ce6a9  call 0x1402ee8d0` - the same compact-look reader `0x0107`, `0x0114` and
/// `0x0138` use, called with the same `(&look, pkt, &str, 0)` shape. **[L]**
pub const USER_ENTER_FIELD_LOOK_AT: usize = 187 + REMOTE_STAT_TAIL_LEN;

/// Byte offset of `x` inside a [`USER_ENTER_FIELD`] body, for an empty name and no equips.
pub const USER_ENTER_FIELD_POS_AT: usize = 426 + REMOTE_STAT_TAIL_LEN;

/// Exactly how long [`user_enter_field`] will be for this character.
///
/// Two things move: the name (a `u16`-prefixed string, so its bytes are added on top of
/// the 2-byte empty form) and the equipped list inside the avatar look (5 bytes each).
pub fn user_enter_field_len(chr: &crate::opcode::Character) -> usize {
    USER_ENTER_FIELD_MIN_LEN + chr.name.len() + 5 * chr.equips.len()
}

/// Build a [`USER_ENTER_FIELD`] body: **put this character on someone else's screen.**
///
/// Full field table with the read address for every one of the 65 fields:
/// `research/user-enter-field.md` §2. The shape, in three parts:
///
/// ```text
///   0  u32  userId          any nonzero value
///   4  u32  charId          NONZERO - see below
///   8  u32  fieldCheck
///  12  ...  CUser::Init, 62 fields, decoded by FUN_1429ce270
/// 187  ...  avatar_look(), unchanged
/// ```
///
/// # The character id must not be zero, and that is a real branch
///
/// `1429ba43b` reads the `u32` at +4; if it is **zero** the client reads a *fourth*
/// `u32` at `1429ba44d` and uses that as the id instead. **[L]** Both encodings decode -
/// the v214 reference writes `id, 0, id`, which is exactly what the long arm consumes -
/// but this builder writes the short one, so a zero id here would silently eat the four
/// bytes of `level` and shift the whole body. Ids start at 200
/// (`store::FIRST_CHARACTER_ID`), so this is a guard rather than a live hazard.
///
/// # What this deliberately does not carry
///
/// Guild block, fame, pets, familiars, mounts, rings, miniroom, damage skin and the
/// chair are all sent as their absent form. Two are load-bearing rather than merely
/// empty:
///
/// * **the miniroom dword at 451 is four zero bytes**, and a nonzero value there opens a
///   further nine-field block ending in a chat post (§2.3);
/// * **no mount or vehicle equip**, because `1429cfc0a` reads an extra `u32` gated on
///   the *client's own* equipped item ids rather than on anything in this packet (§3).
///   That read is invisible to a length check here and would desync the tail.
///
/// # Do not send this twice for the same character
///
/// A `0x0224` naming an id already in the pool **returns in silence without reading the
/// body** (`1429ba556`), so it cannot be used to update anyone - and one naming *our
/// own* id clears a dword and returns. **[L]** for both. To move a remote player, use
/// the remote family at `0x293..0x2C4`; to redress one, leave then enter.
pub fn user_enter_field(chr: &crate::opcode::Character, at: RemoteAt) -> Vec<u8> {
    debug_assert_ne!(chr.id, 0, "a zero character id takes the four-u32 header branch");
    // Everything after the name shifts by its bytes; everything after the look shifts by
    // its equips. Both are folded into the offset assertions below.
    let shift = chr.name.len();
    let equips = 5 * chr.equips.len();

    let mut w = crate::PacketWriter::new();
    w.u32(chr.id); //  0  userId - the same id; nothing reads it apart from CUser::CUser
    w.u32(chr.id); //  4  charId, and it is the hash key
    w.u32(0); //       8  fieldCheck

    w.u32(chr.level); //          12
    w.str(&chr.name); //          16  user+0x10d8, the name the chat line prints
    w.str(""); //                 18  parent name, deprecated
    w.u32(0); //                  20  guild id
    w.str(""); //                 24  guild name
    w.u16(0); //                  26  guild logo background
    w.u8(0); //                   28  ...its colour
    w.u16(0); //                  29  guild logo
    w.u8(0); //                   31  ...its colour
    w.u32(0); //                  32
    w.u32(0); //                  36
    w.u8(chr.gender); //          40
    // 41 - **left at `0`, and that is now checked rather than assumed.** This was briefly
    // sent as `-1` on the strength of a diagnosis that pointed one struct-base out; the
    // local `CUser` holds `0` here, so `0` was right all along. See offset 416.
    w.u32(0); //                  41
    w.u32(0); //                  45  name-tag mark
    w.u8(0); //                   49
    w.u32(0); //                  50
    w.u8(0); //                   54  stored as (v == 1)
    debug_assert_eq!(w.len(), 55 + shift, "the stat mask moved");
    w.zeros(REMOTE_STAT_MASK_LEN); // 55  no buffs
    // The tail the decoder reads unconditionally after the mask - see REMOTE_STAT_TAIL_LEN.
    // Written as four typed fields rather than seven zero bytes so the shape is visible at
    // the one place a future edit would break it.
    w.u8(0); //                   179  0x140a4a007
    w.u8(0); //                   180  0x140a4a024
    w.u32(0); //                  181  0x140a4a041
    // **The four this builder did not know about, and they killed two clients.**
    //
    // `0x140a4a10e call 0x140862470` sits between the `u32` above and the `u8` below, and it
    // is unconditional. `FUN_140862470` reads **four `u32`** and then a `count`-driven loop of
    // `u32` - the fourth of the four IS the count. **[L]**, from the listing:
    //
    // ```text
    //   140862498  READ u32
    //   1408624a2  READ u32
    //   1408624ad  READ u32
    //   1408624b8  READ u32      <- the count
    //   1408624cb  READ u32      <- the loop body, `count` times
    // ```
    //
    // Sixteen bytes. Without them the client took its `count` from body offset 203, which in
    // a real character is **the low half of the avatar look's face id**:
    //
    // ```text
    //   Cobalt   count at body offset 203 = 0x22000000 = 570 425 344 entries
    //   Tester2  count at body offset 204 = 0x21000000 = 553 648 128 entries
    // ```
    //
    // the `0x22`/`0x21` being the low byte of `face` three bytes on.
    //
    // **It does not ask for 2.3 GB, and saying so would send the next reader looking for an
    // allocation that never happens.** There is no bulk read and no `reserve`: the loop walks
    // four bytes at a time. Both processes completed **116 iterations** and threw on the
    // 117th, with three bytes left - `467 = 4 x 116 + 3`, and 467 is exactly what follows the
    // count field in both bodies. Measured out of the reader object in two crash dumps
    // (`research/0x0224-dump-read-position.md`), not derived from this listing.
    //
    // That is also why a 128-byte pad did nothing: it buys 32 more iterations out of 570
    // million. Zeros are not a terminator for a length-prefixed list.
    //
    // **The throw stack in the hook log is this call chain, frame for frame.** Every return
    // address on it is a statically confirmed call site, which is what makes this [L] rather
    // than a story that fits:
    //
    // ```text
    //   0x1406e8cb1   the throw path inside FUN_1406e8c20, the u32 primitive
    //   0x1408624d0 = 0x1408624cb + 5   the return of the LOOP read
    //   0x140a4a113 = 0x140a4a10e + 5   the return of the call this comment is about
    // ```
    w.u32(0); //                  185  0x140862498
    w.u32(0); //                  189  0x1408624a2
    w.u32(0); //                  193  0x1408624ad
    w.u32(0); //                  197  0x1408624b8  the COUNT - zero, so the loop does not run
    w.u8(0); //                   201  0x140a4a29e
    debug_assert_eq!(
        w.len(),
        55 + REMOTE_STAT_MASK_LEN + REMOTE_STAT_TAIL_LEN + shift,
        "the stat block is the mask plus its tail"
    );
    w.u16(chr.job); //            186
    w.u16(0); //                  188  sub-job
    w.u32(0); //                  190

    debug_assert_eq!(w.len(), USER_ENTER_FIELD_LOOK_AT + shift, "the avatar look moved");
    w.bytes(&crate::opcode::avatar_look(chr)); // 187

    w.u32(0); //                  382  driver id
    w.u32(0); //                  386  passenger id
    w.u8(0); //                   390
    w.u32(0); //                  391
    w.u32(0); //                  395
    w.u32(0); //                  399
    w.u8(0); //                   403  no trailing string
    w.u32(0); //                  404  damage skin
    w.str(""); //                 408
    w.str(""); //                 410
    w.u32(0); //                  412
    // **416 is the seat index, and `0` is a valid seat.** This is what killed both clients
    // on every two-client run.
    //
    // `movsx ecx, ax` - **sign-extended**, verified from raw bytes (`0f bf c8`) and from the
    // live image, so `ff ff` becomes `0xFFFFFFFF`. A zero-extending read would have made this
    // 65535 and failed a third time in exactly the same place.
    //
    // It reaches `CUser+0x3c28`, and from there an accessor on an array hanging off the
    // field:
    //
    // ```text
    //   r8  = [field_info + 0x178]        the array base    -> NULL on map 40
    //   ecx = r8 ? [r8-8] : 0             the element count -> 0
    //   if (idx >= 0 && idx < count) goto ok
    //       call 0x142e54290(...)         a REPORTER. It does not throw
    //   ok: return r8 + 48*idx + 8        ->  0 + 0 + 8  =  8
    // ```
    //
    // and `0x140f9295e mov rcx,[rax]` dereferences 8. `RAX = 8` with `R8 = 0` admits only
    // `idx == 0`; `-1` gives `0xFFFFFFFFFFFFFFD8` and never reaches the load.
    //
    // **Five facts fix `-1`, two of them literals in the client's own code:**
    //
    // * `0x142769d49 mov dword ptr [rsi+0x3c28], 0xFFFFFFFF` - the client's own initialiser;
    // * `FUN_142834020` is `IsSitting`: `(m_0x3c18 && ...) || m_0x3c28 != -1` - and `bSit` is
    //   a field in the faulting function's own format string, which is what finally named it;
    // * all three call sites of the accessor gate on `cmp eax, -1`;
    // * the local `CUser` holds `-1` here in **all four** dumps; every packet-built remote
    //   held `0`;
    // * a range scan for `0x3c28` across all of `.text` finds 27 sites and exactly **two**
    //   that write it - this decoder and that initialiser. Nothing overwrites it after `Init`.
    //
    // # It was diagnosed at offset 41 first, and that is worth keeping
    //
    // The static enumeration was right about the field, the accessor and the sentinel, and
    // wrong about **which byte feeds it** - because `[rcx+0x3b28]` is read with `this = r15`,
    // and `r15 = param_1 + 0x100` is a base-subobject pointer. A right number in one frame of
    // reference, looked up in a table indexed by another.
    //
    // What caught it was a diff nobody could have argued with: taking the remote `CUser` of
    // the same character across the run before the `-1` and the run after, and keeping only
    // dwords that went `0 -> 0xFFFFFFFF`, returns **exactly one offset** - and it is `0x100`
    // below where the table said to look.
    //
    // **The general form, which costs nothing to remember and cost two runs to learn: an
    // offset table is meaningless without the base it is indexed against, and neither file in
    // `research/` states one.** Rows 12, 41, 179, 181 and 416 have now been checked against a
    // dump. The rest of that table has not, and could be `0x100` out in either direction.
    w.i16(-1); //                 416  the seat index; -1 = not seated
    w.u32(0); //                  418  chair item id
    w.u32(0); //                  422

    debug_assert_eq!(
        w.len(),
        USER_ENTER_FIELD_POS_AT + shift + equips,
        "the position moved - x and y are what put the character somewhere real"
    );
    w.i16(at.x); //               426
    w.i16(at.y); //               428
    w.u8(at.move_action); //      430
    w.i16(at.foothold); //        431

    w.u8(0); //                   433
    w.u8(0); //                   434
    w.u8(0); //                   435  no chair to decode
    w.u8(0); //                   436  no pets
    w.u8(0); //                   437  no familiars
    w.u32(0); //                  438  taming-mob level
    w.u32(0); //                  442  taming-mob exp
    w.u32(0); //                  446  taming-mob fatigue
    w.u8(0); //                   450
    w.zeros(4); //                451  NO MINIROOM - see the doc block
    // 455, FUN_14073a7b0, unconditional: raw4, u32, raw4, str, u32 = 18 bytes.
    w.zeros(4);
    w.u32(0);
    w.zeros(4);
    w.str("");
    w.u32(0);
    w.u8(0); //                   473
    w.u8(0); //                   474  couple ring
    w.u8(0); //                   475  friendship ring
    w.u8(0); //                   476  marriage ring
    w.u8(0); //                   477
    w.u8(0); //                   478  the bitmask; bit 0x20 would add a u32
    w.u32(0); //                  479
    w.u32(0); //                  483
    w.u32(0); //                  487  FUN_142834df0 count
    w.u8(0); //                   491  FUN_142835840, both unconditional
    w.u32(0); //                  492
    w.u32(0); //                  496  FUN_1428358a0
    w.u32(0); //                  500  count
    w.u32(0); //                  504  count

    debug_assert_eq!(w.len(), user_enter_field_len(chr), "the body length is wrong");
    w.into_vec()
}

/// **`0x0293`** - a remote player walked. Server to client.
///
/// ## How this was found, because no name in any table was involved
///
/// By reachability. `research/user-pool-tables.md` enumerated all **315** distinct handler
/// functions across the pool's four dispatch tables and walked the call graph to depth 6:
/// **exactly two** reach `FUN_1404b2630`, the client's 79-command movement-path decoder.
/// One is `0x02F5` in the *local* table; the other is this. `FUN_1429d2e70` is 102 bytes
/// and has one read site - the call into `FUN_141d598b0`. **[L]**
///
/// It sits in a **second** jump table on `FUN_1429bb720` (`0x1429bbcd0`/`0x1429bbd10`,
/// byte-indexed) that the first enumeration did not know about. The two tables are exactly
/// complementary - 22 opcodes in one, 15 in the other, none in both - so walking only the
/// dense table at `0x1429bbc34` would have missed the move entirely.
pub const USER_MOVE_REMOTE: u16 = 0x0293;

/// **`0x029E..=0x02A1`** - a remote player attacked. Four opcodes, **one** handler
/// (`FUN_1429d2ee0`), corroborated by the throttle at `0x1429bbaea` special-casing the same
/// four. **[L]** for the set.
///
/// Which is melee, which is shoot, which is magic and which is body is **[I]** - and it
/// costs nothing if it is wrong, because the handler stores the opcode and never reads it
/// back. `research/user-pool-tables.md`.
pub const USER_ATTACK_REMOTE_FIRST: u16 = 0x029E;
/// See [`USER_ATTACK_REMOTE_FIRST`].
pub const USER_ATTACK_REMOTE_LAST: u16 = 0x02A1;

/// Build a [`USER_MOVE_REMOTE`] body: `u32 charId` then the movement path **verbatim**.
///
/// ```text
///   0  u32  charId          read by FUN_1429bb720 at 1429bb745, before the dispatch
///   4  ...  the path block, copied byte for byte out of the client's own 0x00D9
///           END - nothing follows it
/// ```
///
/// # Pass [`crate::usermove::UserMove::path`], never `path_with_key_states`
///
/// This is the one detail that would have cost a client launch. The client's own `0x00D9`
/// **carries** a key-state trailer - a count byte then two 4-bit entries per byte - and
/// `0x0293` **does not read it**: `1429d2eb5 XOR R8D,R8D` before the call at `1429d2ec6`,
/// byte-identical to the `141c82000` that makes outbound `MOB_MOVE` drop `0x02FF`'s
/// trailing count. **[L]** Appending those bytes leaves them in the framer's buffer for
/// whatever the client decodes next.
///
/// `UserMove::path` returns `None` unless the element walk closed, and that `None` **must
/// be treated as a refusal to rebroadcast**, not as an empty path.
///
/// *A correction worth keeping, because the obvious check gives the wrong answer:* the
/// **encoder** always writes the trailer - `141d580e5` is unconditional once the path is
/// written, both branches converging at `141d580df` - so comparing call sites alone says
/// the client sends no key states, which 1082 measured bodies disprove. Only the decoder
/// chooses whether to read it.
pub fn user_move_remote(char_id: u32, path: &[u8]) -> Vec<u8> {
    let mut w = crate::PacketWriter::new();
    w.u32(char_id);
    w.bytes(path);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn someone(name: &str, equips: &[(u8, u32)]) -> crate::opcode::Character {
        crate::opcode::Character {
            id: 200,
            name: name.to_string(),
            level: 8,
            job: 100,
            gender: 0,
            equips: equips.to_vec(),
            ..Default::default()
        }
    }

    /// **The number this whole builder rests on.** `research/user-enter-field.md` §2
    /// walks 60-odd fields from offset 0 to the `u32` count at 504; if any one of them
    /// is the wrong width the total is not 508, and the client has no length prefix
    /// anywhere in this body to resynchronise on.
    #[test]
    fn an_empty_name_and_no_equips_is_the_531_byte_minimum() {
        let body = user_enter_field(&someone("", &[]), RemoteAt::default());
        assert_eq!(body.len(), USER_ENTER_FIELD_MIN_LEN);
        // **531. It was 508, then 515 on 2026-08-31, and both were wrong.**
        //
        // The tail after the 124-byte mask is `u8`, `u8`, `u32`, then an unconditional
        // `call 0x140862470` that reads FOUR more `u32` - the fourth being a count - and then
        // a `u8`. Twenty-three bytes, not seven. The missing sixteen made the client take its
        // count from the low half of the avatar look's face id and demand 2.28 GB; two clients
        // died of it the first time they stood on one map.
        //
        // **This test is the reason the literal is spelled out beside the constant, and it
        // still did not catch it.** `assert_eq!(body.len(), 515)` disagreed with the client
        // and agreed with the builder, exactly like the `MINIDUMP_EXCEPTION_INFORMATION` size
        // test in `CLAUDE.md`: a number that came from reading a listing, asserted against
        // another number from the same reading. The listing was not re-read for the call that
        // makes no read of its own.
        assert_eq!(body.len(), 531);
        assert_eq!(
            REMOTE_STAT_MASK_LEN + REMOTE_STAT_TAIL_LEN,
            147,
            "the block is the mask plus the tail, and the mask was never the wrong number"
        );
    }

    /// Only two things move, and both move by a known amount. This is what makes a
    /// length regression a failing test rather than a client that closes itself.
    #[test]
    fn only_the_name_and_the_equips_change_the_length() {
        let equips = [(5u8, 1_040_002u32), (6, 1_060_002), (7, 1_072_001)];

        assert_eq!(
            user_enter_field(&someone("Wanderer", &[]), RemoteAt::default()).len(),
            531 + 8,
            "an 8-character name adds exactly its bytes"
        );
        assert_eq!(
            user_enter_field(&someone("", &equips), RemoteAt::default()).len(),
            531 + 15,
            "three equips add five bytes each"
        );

        let chr = someone("Wanderer", &equips);
        let body = user_enter_field(&chr, RemoteAt::default());
        assert_eq!(body.len(), 531 + 8 + 15);
        assert_eq!(body.len(), user_enter_field_len(&chr), "the predictor agrees");
    }

    /// `1429ce6a9 call 0x1402ee8d0` - the same reader the character-select screen has
    /// already accepted these exact bytes through. If this ever stops being a verbatim
    /// copy, the character is dressed from a different encoder than the one that works.
    #[test]
    fn the_avatar_look_is_the_same_bytes_at_the_offset_the_client_reads() {
        let chr = someone("", &[(5, 1_040_002)]);
        let body = user_enter_field(&chr, RemoteAt::default());
        let look = crate::opcode::avatar_look(&chr);

        assert_eq!(look.len(), 195 + 5, "195 + 5 per equip");
        assert_eq!(
            &body[USER_ENTER_FIELD_LOOK_AT..USER_ENTER_FIELD_LOOK_AT + look.len()],
            &look[..],
            "the look must be byte-identical and start at 187"
        );
    }

    /// The four fields that decide where the character is drawn. A body whose length is
    /// right and whose position is in the wrong place puts someone at the map origin,
    /// which reads on screen as "the broadcast does not work".
    #[test]
    fn the_position_lands_where_the_client_reads_it() {
        let at = RemoteAt { x: -1234, y: 567, move_action: 4, foothold: 89 };
        let body = user_enter_field(&someone("", &[]), at);
        let p = USER_ENTER_FIELD_POS_AT;

        assert_eq!(i16::from_le_bytes([body[p], body[p + 1]]), -1234, "x at 426");
        assert_eq!(i16::from_le_bytes([body[p + 2], body[p + 3]]), 567, "y at 428");
        assert_eq!(body[p + 4], 4, "moveAction at 430");
        assert_eq!(i16::from_le_bytes([body[p + 5], body[p + 6]]), 89, "foothold at 431");
    }

    /// The header is three `u32`s **only** because the id is nonzero. A zero at +4 makes
    /// the client read a fourth one (`1429ba44d`) and everything after it shifts.
    #[test]
    fn the_header_is_three_u32s_and_the_id_is_not_zero() {
        let chr = someone("", &[]);
        let body = user_enter_field(&chr, RemoteAt::default());

        assert_eq!(u32::from_le_bytes(body[4..8].try_into().unwrap()), chr.id);
        assert_ne!(
            u32::from_le_bytes(body[4..8].try_into().unwrap()),
            0,
            "a zero here takes the four-u32 branch and eats the level field"
        );
        // The field right after the header is the level, which is only true while the
        // header is three words long.
        assert_eq!(u32::from_le_bytes(body[12..16].try_into().unwrap()), 8);
    }

    /// **Offset 416 is `-1`, and `0` there killed both clients on every two-client run.**
    ///
    /// It is the seat index. `0` is a valid seat, and the client's accessor **reports** an
    /// out-of-range index and then honours it - returning `base + 48*idx + 8`, which for a
    /// null base and index 0 is the address `8`. The client's own initialiser writes
    /// `0xFFFFFFFF` here and `IsSitting` tests `!= -1`.
    ///
    /// **Read `movsx`, so the sign matters.** `i16(-1)` is `ff ff` and sign-extends to
    /// `0xFFFFFFFF`. A `u16` of 65535 would be the same two bytes and the same value here -
    /// but writing it as `-1` is what makes the intent survive the next edit.
    ///
    /// Anchored on the position dword at 426, which `RemoteAt::default()` puts at `(0, 0)` and
    /// which the world log independently reports - so a moved base breaks the anchor rather
    /// than letting a fixed offset assert into the wrong field. That is not hypothetical here:
    /// this exact fault was first diagnosed at offset **41**, from a table indexed against a
    /// struct base `0x100` away from the one the faulting getter uses.
    #[test]
    fn the_seat_index_at_416_is_minus_one_and_not_zero() {
        for (name, equips) in [("", &[][..]), ("Cobalt", &[(5u8, 1_040_002u32)][..])] {
            let chr = someone(name, equips);
            let body = user_enter_field(&chr, RemoteAt::default());
            let shift = name.len() + 5 * equips.len();

            // The seat is ten bytes ahead of the position, across the chair item id and one
            // more dword. Derived from the two published constants rather than re-typed, so a
            // tail change moves it instead of silently pointing somewhere else.
            let seat = USER_ENTER_FIELD_POS_AT - 10 + shift;
            assert_eq!(
                i16::from_le_bytes(body[seat..seat + 2].try_into().unwrap()),
                -1,
                "0 here is a VALID seat index and the client dereferences it"
            );

            // The anchor. `RemoteAt::default()` is (0, 0) and the world log reports the same,
            // so a moved base breaks this before the assertion above can pass into the wrong
            // field. Not hypothetical: this fault was first diagnosed at offset 41, out of a
            // table indexed against a struct base 0x100 from the one the getter uses.
            let pos = USER_ENTER_FIELD_POS_AT + shift;
            assert_eq!(&body[pos..pos + 4], &[0, 0, 0, 0], "the position anchors the seat");
            assert!(seat + 2 <= body.len() && pos + 4 <= body.len());
        }
    }

    /// The miniroom dword at 451 opens a nine-field block ending in a chat post if it is
    /// anything but zero. Four zero bytes is not decoration.
    #[test]
    fn there_is_no_miniroom_and_no_chair() {
        // **This test could not fail, and it was found by an agent rather than by running.**
        //
        // Every assertion in it reads a fixed offset and expects ZERO, out of a body that is
        // zero almost everywhere. It passed for the 508-byte layout, it passed for the wrong
        // 515-byte one, and it would have passed on the exact body that killed two clients.
        // The offsets were literals from the original numbering and never moved when the
        // stat tail did - twice.
        //
        // Two changes make it a check again:
        //
        // 1. the offsets are derived from `REMOTE_STAT_TAIL_LEN`, so they follow the layout
        //    instead of being re-typed and forgotten;
        // 2. it is **anchored** first. The job is the nearest field with a value we choose,
        //    and if the anchor is wrong every offset below it is measured from nowhere - so
        //    the anchor failing is what stops the zero checks reporting false comfort.
        let body = user_enter_field(&someone("", &[]), RemoteAt::default());

        // The anchor. `100` is the beginner job id `someone()` builds with.
        assert_eq!(
            u16::from_le_bytes([body[202], body[203]]),
            100,
            "the job anchors every offset below; if this moved, nothing else here means anything"
        );

        // The rest, in the zero-tail numbering `research/user-enter-field.md` uses, shifted by
        // the tail exactly as every field after offset 179 is.
        let at = |zero_tail_offset: usize| zero_tail_offset + REMOTE_STAT_TAIL_LEN;
        assert_eq!(&body[at(451)..at(455)], &[0, 0, 0, 0], "miniroom absent");
        assert_eq!(
            u32::from_le_bytes(body[at(418)..at(422)].try_into().unwrap()),
            0,
            "chair item id"
        );
        assert_eq!(body[at(435)], 0, "no chair object to decode");
        assert_eq!(body[at(436)], 0, "no pets");
        assert_eq!(body[at(437)], 0, "no familiars");

        // And the offsets are inside the body, which is the other thing a literal cannot
        // promise: `at(455)` past the end would panic rather than pass, but only if something
        // asks. This asks.
        assert!(at(455) <= body.len(), "the miniroom dword must be inside the body");
    }

    /// The 124-byte stat mask, and the number that is easy to take from the wrong place.
    #[test]

    fn the_remote_stat_block_is_the_124_byte_mask_plus_a_twenty_three_byte_tail() {
        // The mask length was never the wrong number - the v214 reference's 132 is a
        // different version, and taking it would put every later field eight places out.
        assert_eq!(REMOTE_STAT_MASK_LEN, 124, "124 here; the v214 reference has 132");

        // **The tail was 7 and it is 23, and the sixteen it was missing killed two clients.**
        //
        // `FUN_140a46e50` reads `u8, u8, u32` after the mask, then makes an unconditional
        // `call 0x140862470` - which reads four more `u32`, the fourth a COUNT - and then a
        // final `u8`. The call reads nothing itself, which is why a listing walk that looked
        // for read primitives went straight past it.
        assert_eq!(REMOTE_STAT_TAIL_LEN, 23);

        let body = user_enter_field(&someone("", &[]), RemoteAt::default());
        let block = 55..55 + REMOTE_STAT_MASK_LEN + REMOTE_STAT_TAIL_LEN;
        assert_eq!(block, 55..202);
        assert!(body[block].iter().all(|&b| b == 0), "all clear means no buffs");

        // **The job is what says the tail landed in the right place.** It is the first field
        // after the block with a value we choose, so a tail of any other length reads as
        // something else.
        assert_eq!(u16::from_le_bytes([body[202], body[203]]), 100, "job right after");

        // Both previous positions, kept rather than replaced. A test that only checks the
        // current offset passes for a builder that is wrong in a NEW direction.
        for (was, why) in [(179usize, "the 508-byte layout"), (186, "the 515-byte one")] {
            assert_ne!(
                u16::from_le_bytes([body[was], body[was + 1]]),
                100,
                "the job must not still be at {was} - that was {why}"
            );
        }

        // **The count is the byte that mattered.** It is the fourth `u32` the called function
        // reads, and zero is what stops its loop running. Non-zero here demands four bytes
        // per unit from a body that does not have them - at face value, 570 million of them.
        assert_eq!(
            u32::from_le_bytes([body[197], body[198], body[199], body[200]]),
            0,
            "the count at 197 must be zero or the client reads until it throws"
        );
    }

    /// One `u32` and nothing else - `FUN_1429ba980` reads exactly one, at `1429ba9a2`.
    #[test]
    fn a_leave_is_four_bytes() {
        assert_eq!(user_leave_field(207), 207u32.to_le_bytes().to_vec());
        assert_eq!(user_leave_field(207).len(), 4);
    }

    /// `0x0293` sits in the remote range, and the four attack opcodes sit above it in the
    /// same range. A number outside it would be dispatched somewhere else entirely.
    #[test]
    fn the_move_and_attack_opcodes_are_inside_the_remote_range() {
        assert_eq!(USER_MOVE_REMOTE, 0x0293);
        assert_eq!(USER_MOVE_REMOTE, USER_POOL_REMOTE_FIRST, "it is the first one");
        assert_eq!(USER_ATTACK_REMOTE_FIRST, 0x029E);
        assert_eq!(USER_ATTACK_REMOTE_LAST, 0x02A1);
        assert_eq!(USER_ATTACK_REMOTE_LAST - USER_ATTACK_REMOTE_FIRST, 3, "four of them");
        for op in [USER_MOVE_REMOTE, USER_ATTACK_REMOTE_FIRST, USER_ATTACK_REMOTE_LAST] {
            let remote = USER_POOL_REMOTE_FIRST..=USER_POOL_REMOTE_LAST;
            assert!(remote.contains(&op), "{op:#06x}");
            assert!(is_user_pool(op), "{op:#06x}");
        }
    }

    /// The move body is the id and then the path, with **nothing** in between and nothing
    /// after. Four bytes of head is the whole difference from the client's own ten.
    #[test]
    fn a_remote_move_is_an_id_then_the_path_verbatim() {
        let path = [0x11u8, 0x22, 0x33, 0x44, 0x55];
        let body = user_move_remote(201, &path);

        assert_eq!(u32::from_le_bytes(body[0..4].try_into().unwrap()), 201);
        assert_eq!(&body[4..], &path, "copied byte for byte");
        assert_eq!(body.len(), 4 + path.len(), "no trailer, no padding");
    }

    /// The rule that makes a rebroadcast safe: a path we could not walk is a path we must
    /// not re-emit, and `UserMove::path` refuses rather than returning a short slice.
    #[test]
    fn a_path_that_did_not_walk_closed_cannot_be_rebroadcast() {
        // Ten-byte head, a path head claiming one element, and a truncated element.
        let mut body = vec![0u8; crate::usermove::USER_MOVE_HEAD_LEN];
        body.extend_from_slice(&[0u8; 12]);
        body.extend_from_slice(&1i16.to_le_bytes()); // element_count = 1
        body.push(0x00); // a 17-byte element, and the body ends here

        let m = crate::usermove::parse_user_move(&body).expect("long enough to parse");
        assert!(!m.walk_closed, "the walk must not close on a truncated element");
        assert!(m.path(&body).is_none(), "and the span must refuse");
    }

    /// The opcodes, so a typo fails here rather than on the wire.
    #[test]
    fn the_opcodes_are_the_ones_the_routing_names() {
        assert_eq!(USER_ENTER_FIELD, 0x0224);
        assert_eq!(USER_LEAVE_FIELD, 0x0225);
        assert_eq!(USER_CHAT, 0x0226);
        assert_eq!(USER_HIT_REMOTE, 0x02A5);
        assert_eq!(USER_EFFECT_REMOTE, 0x02AF);
    }

    /// The two constants restated from other modules must stay equal to their originals.
    #[test]
    fn the_restated_constants_are_pinned_to_their_originals() {
        assert_eq!(USER_CHAT, crate::userchat::USER_CHAT_TWO_STRINGS);
        assert_eq!(USER_EFFECT_REMOTE, crate::stats::USER_EFFECT_REMOTE);
        assert_eq!(EFFECT_LEVEL_UP, crate::stats::EFFECT_LEVEL_UP);
        assert_eq!(EFFECT_LEVEL_UP, 0);
    }

    /// `lea eax,[r9-0x224] / cmp eax,0x17b / ja` at `0x141821e24`, and the sub-ranges inside
    /// it. The boundaries are the assertion: one off either end and a packet goes somewhere
    /// else entirely.
    #[test]
    fn the_pool_range_is_the_one_cfield_onpacket_tests() {
        assert_eq!(USER_POOL_FIRST, 0x0224);
        assert_eq!(USER_POOL_LAST, USER_POOL_FIRST + 0x17B, "cmp eax,0x17b");
        assert!(!is_user_pool(0x0223));
        assert!(is_user_pool(USER_POOL_FIRST));
        assert!(is_user_pool(USER_POOL_LAST));
        assert!(!is_user_pool(0x03A0));

        for op in [USER_ENTER_FIELD, USER_LEAVE_FIELD, USER_CHAT, USER_HIT_REMOTE, USER_EFFECT_REMOTE]
        {
            assert!(is_user_pool(op), "{op:#06x}");
        }

        // 1429b932e lea eax,[rdx-0x226] / cmp eax,0x6c
        assert_eq!(USER_POOL_BY_ID_FIRST, 0x0226);
        assert_eq!(USER_POOL_BY_ID_LAST, USER_POOL_BY_ID_FIRST + 0x6C);
        assert!((USER_POOL_BY_ID_FIRST..=USER_POOL_BY_ID_LAST).contains(&USER_CHAT));

        // enter and leave are handled inline, BEFORE that lea - so they are outside it, and
        // neither ever reaches the function that reads the shared leading u32.
        assert_eq!(USER_POOL_BY_ID_FIRST - USER_ENTER_FIELD, 2, "0x224 is inline");
        assert_eq!(USER_POOL_BY_ID_FIRST - USER_LEAVE_FIELD, 1, "0x225 is inline");
    }

    /// The remote table's arithmetic, and the discrepancy the constant's doc block refuses
    /// to reconcile.
    #[test]
    fn the_remote_table_indices_reproduce_both_known_opcodes() {
        assert_eq!(USER_POOL_REMOTE_TABLE_BASE + 0x07, USER_HIT_REMOTE, "stub 1429bb9b2");
        assert_eq!(USER_POOL_REMOTE_TABLE_BASE + 0x11, USER_EFFECT_REMOTE, "stub 1429bba36");
        for op in [USER_HIT_REMOTE, USER_EFFECT_REMOTE] {
            assert!((USER_POOL_REMOTE_FIRST..=USER_POOL_REMOTE_LAST).contains(&op));
        }
        // The eleven opcodes between the routed range and the table base are unexplained:
        // 0x293..0x29d reach FUN_1429bb720 and would index the table negatively. Unread.
        assert_eq!(USER_POOL_REMOTE_TABLE_BASE - USER_POOL_REMOTE_FIRST, 11);
        // The local range sits entirely above the remote one and stops one short of the
        // outer range's end. Both are recorded as read, not reconciled.
        assert_eq!(USER_POOL_LOCAL_FIRST, USER_POOL_REMOTE_LAST + 1);
        assert_eq!(USER_POOL_LOCAL_LAST + 1, USER_POOL_LAST);
        assert_eq!(crate::stats::USER_EFFECT_LOCAL, USER_POOL_LOCAL_FIRST + 0x0C);
    }

    /// Five bytes, little-endian, and the same bytes `crate::stats` builds.
    #[test]
    fn the_remote_effect_body_is_five_bytes() {
        let b = user_effect_remote(200, EFFECT_LEVEL_UP);
        assert_eq!(b, vec![200, 0, 0, 0, 0]);
        assert_eq!(b.len(), 5);
        assert_eq!(b, crate::stats::user_effect_remote(200, EFFECT_LEVEL_UP));
        assert_eq!(user_effect_remote(0x1234_5678, 3), vec![0x78, 0x56, 0x34, 0x12, 3]);
    }

    /// One `u32`, and the test says out loud that its width is the inferred part.
    #[test]
    fn the_leave_body_is_one_little_endian_u32() {
        assert_eq!(user_leave_field(200), vec![200, 0, 0, 0]);
        assert_eq!(user_leave_field(0x1234_5678), vec![0x78, 0x56, 0x34, 0x12]);
        assert_eq!(
            user_leave_field(1).len(),
            4,
            "[I]: nobody has counted FUN_1429b9300's inline 0x225 reads"
        );
    }

    /// **The damage field is overwritten, and everything else is passed through untouched.**
    ///
    /// The whole packet is the client's own bytes with four of them replaced, so the test
    /// that matters is that exactly those four move. A builder that rebuilt the HITINFO from
    /// parsed fields would be a second decoder to get wrong.
    #[test]
    fn only_the_damage_dword_differs_from_the_clients_own_bytes() {
        // A recognisable HITINFO: every byte distinct-ish, so a shifted copy shows up.
        let info: Vec<u8> = (0..USER_HIT_REMOTE_HITINFO_LEN).map(|i| (i % 251) as u8).collect();
        let out = user_hit_remote(0x1234_5678, &info, 1234).expect("147 bytes is the right length");

        assert_eq!(out.len(), 4 + USER_HIT_REMOTE_HITINFO_LEN, "charId then the HITINFO");
        assert_eq!(&out[..4], &0x1234_5678u32.to_le_bytes(), "the router reads this first");

        let sent = &out[4..];
        let at = USER_HIT_REMOTE_DAMAGE_AT;
        assert_eq!(
            i32::from_le_bytes(sent[at..at + 4].try_into().unwrap()),
            1234,
            "the server's number, not the client's"
        );
        // Everything either side is byte-identical.
        assert_eq!(&sent[..at], &info[..at], "nothing before the damage moved");
        assert_eq!(&sent[at + 4..], &info[at + 4..], "and nothing after it");
    }

    /// **A body of the wrong length is refused, not padded.**
    ///
    /// The decoder has zero branches, so 147 is the only legal length; anything else means
    /// this server and that client disagree about the packet, and padding would hide it.
    #[test]
    fn a_hitinfo_of_the_wrong_length_is_refused() {
        assert!(user_hit_remote(200, &[0u8; USER_HIT_REMOTE_HITINFO_LEN - 1], 1).is_none());
        assert!(user_hit_remote(200, &[], 1).is_none());
        // Longer IS accepted - the inbound body carries a trailer past the HITINFO and the
        // first 147 bytes are the part the remote decoder reads.
        assert!(user_hit_remote(200, &[0u8; 200], 1).is_some());
    }

    /// The gap this module opened on 2026-08-29 is closed, and this is the tripwire that
    /// says so. `research/user-enter-field.md` decoded all 65 fields; if the file goes
    /// away, the builder above is resting on nothing anyone can check.
    #[test]
    fn the_enter_field_decode_is_still_on_disk() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("crates/net sits two levels under the repo root")
            .to_path_buf();
        let answer = root.join("research").join("user-enter-field.md");
        assert!(
            answer.is_file(),
            "{} is where every offset in user_enter_field came from",
            answer.display()
        );
    }
}
