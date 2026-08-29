//! **`0x00D9`, the client reporting where it walked** - and the only live source of the
//! player's position this server has.
//!
//! Full working: `research/user-move.md`. Every claim carries the address it was read at;
//! **[L]** is off the listing, **[D]** is derived from two or more [L] or from the captures,
//! **[I]** is inferred.
//!
//! # Why this exists
//!
//! Nothing else on the wire says where the player is standing. `0x0107` - the inventory
//! move, and therefore the **drop** - carries a slot and a count and no position at all, so a
//! server that wants to put a dropped item on the ground at the player's feet has to have
//! been listening to this packet. The client's own pick-up sweep is a box of
//! `x-0x19..x+0x19` by `y-0x32..y+0x0a` around the player (`FUN_14179c9d0`), so an item
//! placed more than ~25 px away is drawn and unreachable - and on screen that is
//! indistinguishable from the drop never happening.
//!
//! # The head is 10 bytes, and then it is the SAME path block the mob packet carries
//!
//! `FUN_1409f6eb0` is the only builder of `0x00D9` in the image
//! (`research/msexe-send-opcodes.txt`, one row). Between the `COutPacket(0xd9)` at
//! `0x1409f965d` and the `SendPacket` at `0x1409f9769` it makes exactly five calls that
//! touch the packet, and the code between them is **straight-line - not one branch** - so
//! the head is fixed at ten bytes with nothing gated: **[L]**
//!
//! ```text
//! 1409f9677  w_u8    <- FUN_141829fb0(field)
//! 1409f968f  w_u32   <- FUN_14185cdc0(field)
//! 1409f969f  w_u32   <- FUN_1429e3ef0()      the tick
//! 1409f96b1  w_u8    <- byte [rsp+0x6e]
//! 1409f96d2  call FUN_141d57c60              the movement path
//! ```
//!
//! `FUN_141d57c60` is the **same** path encoder the mob's `0x02FF` calls at `0x141cb8353`,
//! so `crate::mobmove`'s path head is this packet's path head, field for field. The two
//! packets diverge **only in the head**: the mob's is a long variable one (object id, move
//! id, two counted arrays, a gated eleven-`u32` block); the player's is these ten bytes.
//!
//! # The head's x/y is where the walk STARTED, not where the player is
//!
//! This is the correction that matters, and it is measured rather than argued.
//! [`UserMove::start_x`] is the path head's position, and across **1082 captured bodies**
//! every packet's *last element* is the *next* packet's `start` - 1000 consecutive pairs
//! match exactly and the 64 that do not are all map changes. **[D]**
//!
//! **That 1082 is the archive as it stood when the pairing was measured, and this paragraph
//! keeps it deliberately.** The archive now holds **5221** distinct captured bodies; the
//! *walk closure* claim has been re-measured over all of them ([`element_len`]), the
//! *pairing* claim above has not, and restating it against the larger number would be
//! reporting a measurement nobody made.
//!
//! So the head lags by one movement report, which is ~510 ms of walking:
//!
//! | | |
//! |---|---|
//! | standing still | the path starts and ends in the same place, so the head is exact |
//! | walking | measured 14-50 px behind in `melee-collector-runs-once-per-swing-world.log` |
//!
//! Fifty pixels is twice the pick-up box. [`UserMove::x`] is therefore the **end** of the
//! path, which the same capture puts 2-31 px from where the independent `0x00DF` attack
//! packet says the player was, at gaps of 71-502 ms.
//!
//! # Retaining the path so it can be rebroadcast
//!
//! For two clients to see each other walk, the server has to hand player A's path to player
//! B. `crate::mobmove::mob_move_broadcast` already does the mob version of this, and the
//! reason it is allowed to copy rather than re-encode is that **the same encoder writes both
//! packets**: `FUN_141d57c60`, called at `0x1409f96d2` for `0x00D9` and at `0x141cb8353` for
//! `0x02FF`. So the bytes are re-emitted verbatim rather than decoded and rebuilt. **[L]**
//!
//! [`UserMove::path`] and [`UserMove::path_with_key_states`] hand a caller that span, and
//! **both refuse unless [`UserMove::walk_closed`]** - see those methods for why the refusal
//! is the API rather than a flag beside it.
//!
//! ## The two spans are not interchangeable, and `crate::mobmove` is the reason to say so
//!
//! `FUN_141d57c60` writes the path block and then a **key-state trailer**: a nibble count at
//! `0x141d580e5` and the packed nibbles at `0x141d580f0..0x141d5818e`. The reader side,
//! `FUN_141d598b0`, reads that count at `0x141d5991f` **only when its third argument is
//! non-zero**, and the two known call sites disagree - `141cb8346 MOV R8D,R13D` for `0x02FF`
//! against `141c82000 XOR R8D,R8D` for `0x03D9`. That is why
//! [`crate::mobmove::MobMoveRequest::path`] stops *before* the count byte. **[L]**
//!
//! `0x00D9` is on the writing side of that same gate and does carry the trailer - every one
//! of the captured bodies is consumed exactly by head + path + trailer, and none is consumed
//! without it. So this file exposes **two** spans, and which one an outbound remote-move
//! packet wants depends on what that packet's own reader passes in `r8d`. **Nobody has read
//! that call site**; until someone does, neither span may be assumed. See
//! [`UserMove::path_with_key_states`].
//!
//! # Nothing here builds a reply, and that is what makes a mis-parse safe
//!
//! This is an inbound-only decode. Every other packet-shaped mistake in this project has
//! cost a session because a wrong length went **out**; the worst this can do is give the
//! caller a stale position. [`parse_user_move`] says so explicitly through
//! [`UserMove::walk_closed`]: when the element walk does not land exactly on the end of the
//! body, the position falls back to the last one it did read rather than the caller getting
//! nothing.
//!
//! **That stops being true the moment the path is re-emitted**, which is what the span
//! accessors are for - and it is why they are gated rather than merely documented.
//!
//! **`0x00D9` does not appear to need an answer.** 1082 of them went unanswered across every
//! captured session in `research/fixtures/`, with the client continuing to play for minutes
//! afterwards, so it does not latch the way `0x0107` does. **[D]** That is a measured
//! negative over the captures rather than a proof about the client.
//!
//! That count is now **5221** distinct bodies over 172 world logs, and the server still
//! answers none of them - `crates/world/src/session/mod.rs` returns an empty `Vec`. The
//! negative therefore holds over five times the evidence it was written against, but it is
//! still the same *kind* of claim: nothing has ever answered one, so nothing is known about
//! what happens if something does.

/// **`0x00D9`** - `CUserLocal` reporting its own movement. Client to server.
///
/// Built by `FUN_1409f6eb0`, whose `COutPacket` is at `0x1409f965d`. **[L]**
pub const CLIENT_USER_MOVE: u16 = 0x00D9;

/// The ten bytes before the movement path. See the module docs for the four writes. **[L]**
pub const USER_MOVE_HEAD_LEN: usize = 10;

/// The path's own head: `u32`, `i16 x`, `i16 y`, `u16`, `u16`, `i16 count`.
///
/// Read at `1404b2650`, `1404b265b`, `1404b2675`, `1404b2690`, `1404b269c`, `1404b26a9` -
/// the same six reads `crate::mobmove::MOB_PATH_HEAD_LEN` counts, because it is the same
/// block written by the same encoder. **[L]**
pub const MOVE_PATH_HEAD_LEN: usize = 14;

/// The bytes every element reads after its own case body: `u8`, `u16`, `u8`.
///
/// The common tail at `0x1404b2d7f` (`1404b2d82`, `1404b2db7`, `1404b2dce`). Two commands
/// skip it - see [`element_len`]. **[L]**
pub const ELEMENT_COMMON_TAIL_LEN: usize = 4;

/// How many bytes one path element occupies, **including its leading command byte**.
///
/// # Where this table comes from
///
/// `FUN_1404b2630` reads the command at `0x1404b26ee` and dispatches through a **79-entry**
/// jump table: `cmp eax,0x4e / ja default`, a byte index table at `0x1404b2f24` and a dword
/// offset table at `0x1404b2ef0`, both read straight out of the image. The 79 commands map
/// onto **13 case bodies**, and each body is a straight run of reads ending in a `jmp` to the
/// loop tail. **[L]**
///
/// | case | commands | payload | tail |
/// |---|---|---|---|
/// | `0x1404b2755` | `0x00 0x08 0x13 0x37..0x3c 0x44 0x4e` | 8 x u16 | yes |
/// | `0x1404b2755` | `0x0f 0x11` | 9 x u16 - the extra one is gated at `1404b27de` on the command being exactly one of these two | yes |
/// | `0x1404b2846` | `0x2c 0x36` | 5 x u16 | yes |
/// | `0x1404b28c6` | `0x01 0x02 0x12 0x15 0x30..0x35 0x46` | 2 x u16 | yes |
/// | `0x1404b292e` | 29 commands | none | yes |
/// | `0x1404b297b` | `0x18 0x21` | u32 | yes |
/// | `0x1404b29f8` | `0x17` | 2 x u32 | yes |
/// | `0x1404b2a85` | `0x03..0x07 0x09..0x0b 0x0d 0x29 0x2a` | 3 x u16, u32 | yes |
/// | `0x1404b2b23` | `0x16` | 3 x u16 | yes |
/// | `0x1404b2b6c` | `0x0e 0x10` | 3 x u16 | yes |
/// | `0x1404b2beb` | `0x14 0x48 0x49` | 2 x u16, then `0x1404b28f5`'s 2 x u16 | yes |
/// | `0x1404b2c20` | `0x27` | 5 x u16 | yes |
/// | `0x1404b2ca0` | `0x0c` | u8 | **no** - `jmp 0x1404b2ec0` skips it |
/// | `0x1404b2e58` | `0x3d 0x3f` | 7 x u16 | **no** - same |
///
/// Anything above `0x4e` takes the `ja` at `0x1404b2734`, fails the `0x3d`/`0x3f` test at
/// `0x1404b2d65` and falls into the common tail with no payload: five bytes.
///
/// # The check that this table is right
///
/// It is not fitted to the captures; the captures are the check. Every distinct captured
/// `0x00D9` body in `research/fixtures/` and `previous-runs/` walks with this table and
/// lands **exactly** on the end of the body - **5221 of 5221** on 2026-08-29. **[D]**
///
/// `every_captured_body_walks_closed_and_the_span_accounts_for_it` is that sweep, and it
/// deduplicates on `(timestamp, opcode, body)` rather than on the file, for the reason
/// `CLAUDE.md` gives: `research/fixtures/` holds copies of `previous-runs/`, and eleven of
/// those pairs are the *same run* copied mid-write, so they hash differently. The raw line
/// count over both directories is **8236**; a file-hash-only pass gives **7765**; the
/// event-level count is **5221**.
///
/// This file previously said **1082**, and that number is not being retracted - it was the
/// archive of the day. 172 world logs are in scope now. What is new is only that the
/// closure claim has been re-measured against all of them; the *pairing* claim in the module
/// docs (one report's end is the next one's start) has **not** been re-run at this size and
/// still stands on its original 1064 pairs.
pub fn element_len(command: u8) -> usize {
    let (payload, tail) = match command {
        0x00 | 0x08 | 0x13 | 0x37..=0x3c | 0x44 | 0x4e => (16, true),
        0x0f | 0x11 => (18, true),
        0x2c | 0x36 => (10, true),
        0x01 | 0x02 | 0x12 | 0x15 | 0x30..=0x35 | 0x46 => (4, true),
        0x19..=0x20 | 0x22..=0x26 | 0x28 | 0x2b | 0x2d..=0x2f | 0x3e | 0x40..=0x43 | 0x45
        | 0x47 | 0x4a..=0x4d => (0, true),
        0x18 | 0x21 => (4, true),
        0x17 => (8, true),
        0x03..=0x07 | 0x09..=0x0b | 0x0d | 0x29 | 0x2a => (10, true),
        0x16 => (6, true),
        0x0e | 0x10 => (6, true),
        0x14 | 0x48 | 0x49 => (8, true),
        0x27 => (10, true),
        0x0c => (1, false),
        0x3d | 0x3f => (14, false),
        _ => (0, true),
    };
    1 + payload + if tail { ELEMENT_COMMON_TAIL_LEN } else { 0 }
}

/// Does this element carry a position of its own, or inherit the previous one?
///
/// The two obfuscated slots at element `+0x00` and `+0x40` are the pair the common tail
/// deobfuscates into `rbp`/`r15` at `0x1404b2e0b`/`0x1404b2e1b` and that the **next**
/// element's case body re-stores when it has no position of its own (`0x1404b28c9`,
/// `0x1404b2931`, `0x1404b297e`, ...). So a command that writes those two slots from the
/// wire is carrying `x, y`, and one that re-stores them is standing still. **[L]**
///
/// Every such case reads them as its **first two `u16`**, immediately after the command
/// byte, which is why [`parse_user_move`] can take them without decoding the rest.
pub fn element_carries_position(command: u8) -> bool {
    matches!(
        command,
        0x00 | 0x08
            | 0x13
            | 0x37..=0x3c
            | 0x44
            | 0x4e
            | 0x0f
            | 0x11
            | 0x2c
            | 0x36
            | 0x03..=0x07
            | 0x09..=0x0b
            | 0x0d
            | 0x29
            | 0x2a
            | 0x16
            | 0x14
            | 0x48
            | 0x49
            | 0x27
    )
}

/// A parsed [`CLIENT_USER_MOVE`].
///
/// The field a caller almost always wants is [`UserMove::x`] / [`UserMove::y`] - **where the
/// player ended up**, not where they set off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserMove {
    /// Body 0, from `FUN_141829fb0(field)` at `0x1409f9670`. `0` in every captured body.
    /// **[L]** for the source, meaning not established.
    pub field_flag: u8,
    /// Body 1, from `FUN_14185cdc0(field)` at `0x1409f9689`.
    ///
    /// Constant for a whole field and **changes when the map changes** - in
    /// `melee-collector-runs-once-per-swing-world.log` it goes from `0x4dca5bef` to
    /// `0x5a9f8081` on the packet where the position also jumps. A field key by behaviour;
    /// the name is **[I]**. Nothing here uses it. **[D]** for the per-field behaviour.
    pub field_key: u32,
    /// Body 5, from `FUN_1429e3ef0()` at `0x1409f9694`. **A millisecond tick.**
    ///
    /// Two captures agree: consecutive packets 26.525 s apart in the log differ by
    /// `0x6798` = 26 520, and the 510 ms cadence shows up as `0x1FE` every time. It is the
    /// same clock `net::combat::AttackRequest::tick` reads. **[D]**
    pub tick: u32,
    /// Body 9, from `byte [rsp+0x6e]` at `0x1409f96a4`; the same local is tested at
    /// `0x1409f96f9` to decide whether the client also runs a field-level update. `0` in
    /// every captured body. **[L]** for the source.
    pub flag: u8,
    /// The path head's position: where this report **starts**, which is where the previous
    /// report ended. Not the player's current position - see the module docs.
    pub start_x: i16,
    /// See [`UserMove::start_x`].
    pub start_y: i16,
    /// The path head's element count, read as a **signed** `i16` at `0x1404b26a9`
    /// (`movsx ecx,ax / test ecx,ecx / jle` bails). **[L]**
    pub element_count: i16,
    /// **Where the player is now**: the position of the last element that carried one, or
    /// [`UserMove::start_x`] if none did.
    pub x: i16,
    /// See [`UserMove::x`].
    pub y: i16,
    /// The key-state trailer's count, from the byte at `0x141d580e5`.
    pub key_count: u8,
    /// **Did the element walk land exactly on the end of the body?**
    ///
    /// This is the self-check that makes the decode falsifiable: with the [`element_len`]
    /// table right, the walk plus the key trailer consumes the body to the byte, and it does
    /// so for all 5221 distinct captured bodies. A `false` here means this client emitted a
    /// shape the table does not know, and [`UserMove::x`] is then the last good position
    /// rather than a guess.
    ///
    /// **For a position, `false` costs accuracy. For a rebroadcast it is fatal**, which is
    /// why [`UserMove::path`] and [`UserMove::path_with_key_states`] return `None` on it
    /// instead of leaving the check to the caller.
    pub walk_closed: bool,
    /// Bytes of the **movement path block** - the path head plus every element - starting at
    /// body offset [`USER_MOVE_HEAD_LEN`].
    ///
    /// Diagnostic on its own: when [`UserMove::walk_closed`] is `false` this is simply where
    /// the walk gave up, and it may point past the end of the body. Read it through
    /// [`UserMove::path`], which refuses in that case.
    pub path_len: usize,
    /// Bytes of the **key-state trailer** that `FUN_141d57c60` writes after the path: the
    /// count at `0x141d580e5` plus `key_count.div_ceil(2)` packed nibbles. **[L]**
    ///
    /// `0` when the walk never reached it. See [`UserMove::path_with_key_states`].
    pub key_states_len: usize,
}

impl UserMove {
    /// Body offsets of the **movement path block alone** - path head plus elements, stopping
    /// before the key-state trailer.
    ///
    /// This is the exact analogue of [`crate::mobmove::MobMoveRequest::path`], which stops
    /// before `0x02FF`'s nibble-count byte because `141c82000 XOR R8D,R8D` means `0x03D9`
    /// never reads one.
    ///
    /// **`None` unless [`UserMove::walk_closed`].** A path we could not walk is a path we
    /// must not re-emit: the body would go out at a length the client did not expect, and
    /// this project has killed the client twice that way (`crate::userchat`'s two crashes,
    /// 23 bytes short and 4 bytes short). The refusal is the return value rather than a
    /// separate flag because `CLAUDE.md`'s "a guard whose answer is ignored is not a guard"
    /// was written about three call sites that logged a refusal and carried on.
    pub fn path_span(&self) -> Option<std::ops::Range<usize>> {
        if !self.walk_closed {
            return None;
        }
        let end = USER_MOVE_HEAD_LEN.checked_add(self.path_len)?;
        Some(USER_MOVE_HEAD_LEN..end)
    }

    /// Body offsets of the path block **and** the key-state trailer - everything after the
    /// ten-byte head.
    ///
    /// When [`UserMove::walk_closed`] this is always `USER_MOVE_HEAD_LEN..body.len()`, which
    /// is what makes it checkable: `span.len() + USER_MOVE_HEAD_LEN == body.len()` for every
    /// captured body, and the sweep asserts exactly that.
    ///
    /// # Which of the two spans an outbound packet wants is NOT established
    ///
    /// `FUN_141d598b0` reads the trailer's count byte at `0x141d5991f` only when its third
    /// argument is non-zero, and the two known call sites disagree (`141cb8346 MOV R8D,R13D`
    /// for `0x02FF`, `141c82000 XOR R8D,R8D` for `0x03D9`). **[L]** Nobody has read the
    /// corresponding call site for a remote *user* movement packet, so which span to copy is
    /// open - see `crate::userpool`, where that opcode is a documented gap.
    ///
    /// **`None` unless [`UserMove::walk_closed`]**, for the reason [`UserMove::path`] gives.
    pub fn path_with_key_states_span(&self) -> Option<std::ops::Range<usize>> {
        if !self.walk_closed {
            return None;
        }
        let end = USER_MOVE_HEAD_LEN
            .checked_add(self.path_len)?
            .checked_add(self.key_states_len)?;
        Some(USER_MOVE_HEAD_LEN..end)
    }

    /// The movement path block, borrowed out of the **same body** [`parse_user_move`] read.
    ///
    /// `None` when the walk did not close, and also when `body` is not the body this was
    /// parsed from - the slice is taken with `get`, so a shorter one refuses rather than
    /// panicking. See [`UserMove::path_span`].
    pub fn path<'a>(&self, body: &'a [u8]) -> Option<&'a [u8]> {
        body.get(self.path_span()?)
    }

    /// The path block **plus** the key-state trailer: everything after the ten-byte head.
    ///
    /// See [`UserMove::path_with_key_states_span`] for why there are two of these and why
    /// picking between them is not yet decidable.
    pub fn path_with_key_states<'a>(&self, body: &'a [u8]) -> Option<&'a [u8]> {
        body.get(self.path_with_key_states_span()?)
    }
}

/// Parse a [`CLIENT_USER_MOVE`] body. The caller has already stripped the opcode.
///
/// `None` only when the body is too short to contain the fixed head and the path head - 24
/// bytes. Anything longer comes back with a position; see [`UserMove::walk_closed`].
pub fn parse_user_move(body: &[u8]) -> Option<UserMove> {
    let head = body.get(..USER_MOVE_HEAD_LEN)?;
    let path = body.get(USER_MOVE_HEAD_LEN..)?;
    let ph = path.get(..MOVE_PATH_HEAD_LEN)?;

    let field_flag = head[0];
    let field_key = u32::from_le_bytes([head[1], head[2], head[3], head[4]]);
    let tick = u32::from_le_bytes([head[5], head[6], head[7], head[8]]);
    let flag = head[9];

    // 0..4 is a u32 nothing has been traced to; every captured body has it zero.
    let start_x = i16::from_le_bytes([ph[4], ph[5]]);
    let start_y = i16::from_le_bytes([ph[6], ph[7]]);
    // 8..12 are two u16 read at 1404b2690 / 1404b269c, meaning not established.
    let element_count = i16::from_le_bytes([ph[12], ph[13]]);

    let mut m = UserMove {
        field_flag,
        field_key,
        tick,
        flag,
        start_x,
        start_y,
        element_count,
        x: start_x,
        y: start_y,
        key_count: 0,
        walk_closed: false,
        path_len: MOVE_PATH_HEAD_LEN,
        key_states_len: 0,
    };
    if element_count < 0 {
        return Some(m);
    }

    // One labelled block rather than five early `return`s: every exit has to leave
    // `path_len` describing how far the walk actually got, and separate returns are how one
    // of them gets missed.
    let mut p = MOVE_PATH_HEAD_LEN;
    'walk: {
        for _ in 0..element_count {
            let Some(&command) = path.get(p) else { break 'walk };
            if element_carries_position(command) {
                let Some(xy) = path.get(p + 1..p + 5) else { break 'walk };
                m.x = i16::from_le_bytes([xy[0], xy[1]]);
                m.y = i16::from_le_bytes([xy[2], xy[3]]);
            }
            p += element_len(command);
        }

        // The key-state trailer, written by FUN_141d57c60 after the path returns: a count at
        // 0x141d580e5 and then the loop at 0x141d580f0..0x141d5818e, which packs two 4-bit
        // entries into each byte. It is why every body is ten-ish bytes longer than the path.
        let Some(&key_count) = path.get(p) else { break 'walk };
        m.key_count = key_count;
        m.key_states_len = 1 + usize::from(key_count).div_ceil(2);
        m.walk_closed = p + m.key_states_len == path.len();
    }
    // `p` is the end of the element walk on every exit, closed or not. When it is not
    // closed it may point past the end of the body, which is why the span accessors refuse
    // on `walk_closed` before they ever touch this.
    m.path_len = p;
    Some(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 14:01:40.792 in `research/fixtures/character-on-map1-playable-world.log`.
    const MAP1_FIRST: &str = "0057a301a8c139cc04000000000043ffab010000000003000043ffd7010000a401000000000000ffff06d200000043ffe50100000000000000000000ffff061e00000043ffe501000000002b0000000000ffff040e010011000000000000000000";
    /// 14:02:07.317, the very next one on that connection.
    const MAP1_SECOND: &str = "0057a301a859a1cc04000000000043ffe5010000000003000043ffe501000000002b0000000000ffff048601000046ffe5015a0000002b0000000000ffff02400000004cffe50184000000320000000000ffff0238000011000000000000404404";
    /// 05:31:41.008 in `research/fixtures/melee-collector-runs-once-per-swing-world.log`.
    const MELEE: &str = "0081809f5a8aaa1f080000000000f2018b017e000000030000f8018b014e000000250000000000ffff043c000000fc018b01060000002500000000000500085a000000fc018b010000000025000000000005000868010011000000000000000000";

    fn body(hex: &str) -> Vec<u8> {
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
            .collect()
    }

    /// Every field of a real captured body, at the offsets the builder writes them.
    #[test]
    fn the_head_is_ten_bytes_and_the_path_starts_at_ten() {
        let b = body(MAP1_FIRST);
        assert_eq!(b.len(), 97, "the world.log line says 97");
        let m = parse_user_move(&b).expect("a 97-byte body is not short");

        assert_eq!(m.field_flag, 0, "body 0, FUN_141829fb0");
        assert_eq!(m.field_key, 0xa801_a357, "body 1, FUN_14185cdc0");
        assert_eq!(m.tick, 0x04cc_39c1, "body 5, the tick");
        assert_eq!(m.flag, 0, "body 9");
        assert_eq!((m.start_x, m.start_y), (-189, 427), "the path head, body 14 and 16");
        assert_eq!(m.element_count, 3);
        assert_eq!(m.key_count, 0x11, "17 key states, so 9 packed bytes");
        assert!(m.walk_closed, "the walk must land exactly on the end of the body");
    }

    /// The tick is a millisecond clock: two captured packets 26.525 s apart in `world.log`.
    #[test]
    fn the_tick_delta_matches_the_log_timestamps() {
        let first = parse_user_move(&body(MAP1_FIRST)).unwrap();
        let second = parse_user_move(&body(MAP1_SECOND)).unwrap();
        let delta = second.tick - first.tick;
        assert_eq!(delta, 26_520, "14:01:40.792 -> 14:02:07.317 is 26 525 ms");
    }

    /// **The property that says the decode is right**, and it needs no field names:
    /// one report ends exactly where the next one starts.
    #[test]
    fn one_report_ends_where_the_next_one_starts() {
        let first = parse_user_move(&body(MAP1_FIRST)).unwrap();
        let second = parse_user_move(&body(MAP1_SECOND)).unwrap();
        assert!(first.walk_closed && second.walk_closed);
        assert_eq!(
            (first.x, first.y),
            (second.start_x, second.start_y),
            "1000 of 1064 consecutive captured pairs do this; the 64 that do not are map changes"
        );
        // And the end really is somewhere else - otherwise this test would pass on a bug
        // that simply returned the start.
        assert_ne!((first.x, first.y), (first.start_x, first.start_y));
        assert_eq!((first.x, first.y), (-189, 485));
        assert_eq!((second.x, second.y), (-180, 485));
    }

    /// Cross-checked against a different packet decoded by a different agent.
    #[test]
    fn the_end_position_agrees_with_the_attack_packet() {
        let m = parse_user_move(&body(MELEE)).unwrap();
        assert!(m.walk_closed);
        assert_eq!((m.start_x, m.start_y), (498, 395));
        assert_eq!((m.x, m.y), (508, 395));
        // 05:31:41.669, 661 ms later, the 0x00DF attack in the same capture puts the player
        // at (522, 395) - still walking right along the same ground line. The head would
        // have said 498, the end says 508.
        assert_eq!(m.y, 395, "the attack packet's y, exactly");
    }

    /// The 79 commands of the jump table at `0x1404b2f24`, censused by length.
    ///
    /// The counts come from the table itself - 13 case bodies covering 13, 2, 11, 29, 2, 1,
    /// 11, 1, 2, 3, 1 and 1 commands plus the two that reach `0x1404b2e58` - so this fails if
    /// [`element_len`]'s ranges drift from what was read.
    #[test]
    fn the_element_table_is_the_jump_tables_census() {
        let mut census = std::collections::BTreeMap::new();
        for c in 0x00u8..=0x4e {
            *census.entry(element_len(c)).or_insert(0) += 1;
        }
        assert_eq!(
            census,
            [(2, 1), (5, 29), (9, 13), (11, 3), (13, 4), (15, 16), (21, 11), (23, 2)]
                .into_iter()
                .collect()
        );
        assert_eq!(census.values().sum::<i32>(), 0x4f, "79 commands, all of them");
        // Past the table's end: `ja` at 0x1404b2734, then the common tail alone.
        for c in 0x4fu8..=0xff {
            assert_eq!(element_len(c), 5, "command {c:#04x} is past the jump table");
        }
    }

    /// The two commands that skip the four-byte common tail, and the one gated extra u16.
    #[test]
    fn the_three_exceptions_in_the_element_table() {
        assert_eq!(element_len(0x0c), 2, "0x1404b2ca0 jumps straight to the loop tail");
        assert_eq!(element_len(0x3d), 15, "0x1404b2e58, seven u16 and no tail");
        assert_eq!(element_len(0x3f), 15);
        assert_eq!(element_len(0x00), 21, "the ordinary element - the mob path's 21 bytes");
        assert_eq!(element_len(0x0f), 23, "gated at 1404b27de on the command itself");
        assert_eq!(element_len(0x11), 23);
    }

    /// A command with no position of its own leaves the player where the last one put them.
    #[test]
    fn only_some_commands_carry_a_position() {
        assert!(element_carries_position(0x00));
        assert!(element_carries_position(0x0f));
        assert!(!element_carries_position(0x01), "re-stores the previous pair at 1404b28c9");
        assert!(!element_carries_position(0x19), "0x1404b292e reads nothing at all");
        assert!(!element_carries_position(0x0c));
        let carriers = (0x00u8..=0x4e).filter(|&c| element_carries_position(c)).count();
        assert_eq!(carriers, 31, "eleven cases carry x,y as their first two u16");
    }

    /// A truncated body still yields a usable position rather than nothing.
    ///
    /// Nothing is built from this packet, so the safe failure is a stale position, not a
    /// refusal - and `walk_closed` is how the caller can tell.
    #[test]
    fn a_truncated_body_degrades_instead_of_failing() {
        let full = body(MAP1_FIRST);
        let cut = &full[..full.len() - 12];
        let m = parse_user_move(cut).expect("still longer than the fixed head");
        assert!(!m.walk_closed, "the walk cannot land on the end of a truncated body");
        assert_eq!((m.start_x, m.start_y), (-189, 427));
        assert_eq!((m.x, m.y), (-189, 485), "the last element it did read");
    }

    #[test]
    fn a_body_too_short_for_the_two_heads_is_none() {
        assert!(parse_user_move(&[]).is_none());
        assert!(parse_user_move(&[0u8; 23]).is_none(), "24 is the minimum");
        let m = parse_user_move(&[0u8; 24]).expect("exactly the two heads");
        assert_eq!(m.element_count, 0);
        assert!(!m.walk_closed, "no room for the key-state count byte");
    }

    /// A negative count is the client's own bail (`movsx / jle` at `0x1404b26a9`), so it must
    /// not become a huge unsigned loop here.
    #[test]
    fn a_negative_element_count_stops_the_walk() {
        let mut b = body(MAP1_FIRST);
        b[USER_MOVE_HEAD_LEN + 12] = 0xff;
        b[USER_MOVE_HEAD_LEN + 13] = 0xff;
        let m = parse_user_move(&b).unwrap();
        assert_eq!(m.element_count, -1);
        assert!(!m.walk_closed);
        assert_eq!((m.x, m.y), (m.start_x, m.start_y), "no element was read");
    }

    #[test]
    fn the_opcode_is_the_one_the_captures_carry() {
        assert_eq!(CLIENT_USER_MOVE, 0x00D9);
        assert_eq!(USER_MOVE_HEAD_LEN + MOVE_PATH_HEAD_LEN, 24);
    }

    // ---- the path span --------------------------------------------------------------

    /// The whole body, minus the ten-byte head, is exactly what the span covers - and the
    /// two spans differ by exactly the key-state trailer.
    #[test]
    fn the_span_accounts_for_every_byte_after_the_head() {
        for hex in [MAP1_FIRST, MAP1_SECOND, MELEE] {
            let b = body(hex);
            let m = parse_user_move(&b).expect("a real body parses");
            assert!(m.walk_closed);

            let full = m
                .path_with_key_states(&b)
                .expect("a closed walk hands over its bytes");
            assert_eq!(USER_MOVE_HEAD_LEN + full.len(), b.len(), "1409f96d2 writes the rest");
            assert_eq!(full, &b[USER_MOVE_HEAD_LEN..]);

            let path = m.path(&b).expect("a closed walk hands over its bytes");
            assert_eq!(path.len(), m.path_len);
            assert_eq!(path, &full[..m.path_len]);
            assert_eq!(
                full.len() - path.len(),
                m.key_states_len,
                "the two spans differ by the key-state trailer and nothing else"
            );
            assert_eq!(m.key_states_len, 1 + usize::from(m.key_count).div_ceil(2), "141d580e5");
            assert_eq!(
                b[USER_MOVE_HEAD_LEN + m.path_len],
                m.key_count,
                "the trailer's count byte is the first byte the short span excludes"
            );
        }
    }

    /// **A path we could not walk is a path we must not re-emit.**
    ///
    /// The refusal is the return value, not a flag the caller may skip reading. `CLAUDE.md`:
    /// a guard whose answer is ignored is not a guard.
    #[test]
    fn a_walk_that_did_not_close_refuses_to_hand_over_a_span() {
        let full = body(MAP1_FIRST);
        let cut = &full[..full.len() - 12];
        let m = parse_user_move(cut).expect("still longer than the fixed head");
        assert!(!m.walk_closed);
        assert_eq!(m.path_span(), None);
        assert_eq!(m.path_with_key_states_span(), None);
        assert_eq!(m.path(cut), None);
        assert_eq!(m.path_with_key_states(cut), None);
        // ...while the position still comes back, which is the asymmetry the module docs
        // name: `false` costs accuracy for a reader and is fatal for a rebroadcast.
        assert_eq!((m.x, m.y), (-189, 485));

        // The client's own bail on a negative count is the same refusal.
        let mut lying = body(MAP1_FIRST);
        lying[USER_MOVE_HEAD_LEN + 12] = 0xff;
        lying[USER_MOVE_HEAD_LEN + 13] = 0xff;
        let m = parse_user_move(&lying).unwrap();
        assert_eq!(m.path(&lying), None, "1404b26a9 is signed and the client bails");
    }

    /// A span is only meaningful against the body it was parsed from, and handing it a
    /// different one must refuse rather than panic.
    #[test]
    fn a_span_taken_against_the_wrong_body_refuses_rather_than_panicking() {
        let b = body(MAP1_FIRST);
        let m = parse_user_move(&b).unwrap();
        assert!(m.path_with_key_states(&b[..b.len() - 1]).is_none());
        assert!(m.path(&[]).is_none());
        assert!(m.path(&b).is_some(), "the right body still works");
    }

    /// The path span is the same block `crate::mobmove` re-emits, so its head parses the
    /// same way - the check that "same encoder" is more than a sentence.
    #[test]
    fn the_span_starts_with_the_same_path_head_the_mob_packet_carries() {
        let b = body(MAP1_FIRST);
        let m = parse_user_move(&b).unwrap();
        let path = m.path(&b).unwrap();
        assert_eq!(MOVE_PATH_HEAD_LEN, crate::mobmove::MOB_PATH_HEAD_LEN, "same encoder");
        assert!(path.len() >= MOVE_PATH_HEAD_LEN);
        assert_eq!(i16::from_le_bytes([path[4], path[5]]), m.start_x, "1404b265b");
        assert_eq!(i16::from_le_bytes([path[6], path[7]]), m.start_y, "1404b2675");
        assert_eq!(i16::from_le_bytes([path[12], path[13]]), m.element_count, "1404b26a9");
    }

    // ---- the whole capture archive --------------------------------------------------

    /// Knobs for the archive sweep below.
    mod archive {
        /// Point the sweep at a different checkout. Default: the parent of `crates/net/`.
        pub const ROOT: &str = "MAPLECW_REPO_ROOT";
        /// `02:04:19.486 <- 0x00D9 UNKNOWN, 97 byte body 0057a301...`
        pub const BODY_MARK: &str = " byte body ";
    }

    fn repo_root() -> std::path::PathBuf {
        if let Ok(p) = std::env::var(archive::ROOT) {
            return std::path::PathBuf::from(p);
        }
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("crates/net sits two levels under the repo root")
            .to_path_buf()
    }

    /// `previous-runs/world*.log` and `research/fixtures/*world*.log`, the same two sets
    /// `tools/extract_attack_bodies.py` scans. The live `world.log` in the repo root is
    /// deliberately **not** here: it may be being written as this runs.
    fn world_logs(root: &std::path::Path) -> (Vec<std::path::PathBuf>, Vec<String>) {
        let mut files = Vec::new();
        let mut absent = Vec::new();
        for (dir, needs_prefix) in [
            (root.join("previous-runs"), true),
            (root.join("research").join("fixtures"), false),
        ] {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                absent.push(dir.display().to_string());
                continue;
            };
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !name.ends_with(".log") {
                    continue;
                }
                let wanted = if needs_prefix {
                    name.starts_with("world")
                } else {
                    name.contains("world")
                };
                if wanted {
                    files.push(entry.path());
                }
            }
        }
        files.sort();
        (files, absent)
    }

    /// Split one world-log line into `(timestamp, stated length, body hex)`, for inbound
    /// `0x00D9` only. The stated length is carried separately from the hex, which gives a
    /// free consistency check on every record.
    fn captured_user_move(line: &str) -> Option<(&str, usize, &str)> {
        let mut it = line.split_whitespace();
        let stamp = it.next()?;
        if it.next()? != "<-" {
            return None; // `->` is server to client; a user move is never that
        }
        if it.next()?.trim_end_matches(',') != "0x00D9" {
            return None;
        }
        let at = line.find(archive::BODY_MARK)?;
        let stated: usize = line[..at].split_whitespace().next_back()?.parse().ok()?;
        Some((stamp, stated, line[at + archive::BODY_MARK.len()..].trim_end()))
    }

    fn unhex(hex: &str) -> Option<Vec<u8>> {
        if !hex.len().is_multiple_of(2) {
            return None;
        }
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok())
            .collect()
    }

    /// **Every distinct `0x00D9` this repository holds, walked and spanned.**
    ///
    /// # Deduplicate the EVENTS, not the files
    ///
    /// `research/fixtures/` holds *copies* of `previous-runs/`, so a glob over both counts a
    /// capture once per name it has - and a content hash is not enough either, because a
    /// fixture is copied while the run is still being written, so eleven such pairs are the
    /// same run and hash differently (`CLAUDE.md`). The key here is
    /// `(timestamp, opcode, body)`.
    ///
    /// On 2026-08-29 that is **8236 raw body lines over 172 files -> 5221 distinct events**.
    /// A file-hash-only pass gives 7765, i.e. it still double-counts 2544 of them.
    ///
    /// # This must not be a skip in disguise
    ///
    /// `previous-runs/` is gitignored and rolls; `research/fixtures/` is committed. If both
    /// are missing the sweep says so loudly and says it asserted nothing. If they are there,
    /// an empty result is a failure - "the sweep found nothing" and "the sweep did not run"
    /// must not look the same.
    #[test]
    fn every_captured_body_walks_closed_and_the_span_accounts_for_it() {
        use std::io::BufRead;

        let root = repo_root();
        let (files, absent) = world_logs(&root);
        if files.is_empty() {
            eprintln!(
                "SKIPPED every_captured_body_walks_closed_and_the_span_accounts_for_it: \
                 THIS TEST ASSERTED NOTHING. No world logs under {}. \
                 previous-runs/ is gitignored and rolls, research/fixtures/ is committed, so \
                 an empty result means the checkout is partial or the root is wrong. \
                 Directories not readable: {absent:?}. Set {} to the checkout to fix it.",
                root.display(),
                archive::ROOT,
            );
            return;
        }

        let mut raw = 0usize;
        let mut seen: std::collections::HashSet<(String, u16, Vec<u8>)> =
            std::collections::HashSet::new();
        let mut bodies: Vec<(String, Vec<u8>)> = Vec::new();
        // A record whose stated length disagrees with its hex is a format change and a hard
        // failure - unless it is the file's LAST line, which is the known way a fixture gets
        // copied out of a run that is still being written.
        let mut malformed: Vec<String> = Vec::new();
        let mut truncated_tails = 0usize;

        for path in &files {
            let Ok(fh) = std::fs::File::open(path) else {
                malformed.push(format!("{}: cannot open", path.display()));
                continue;
            };
            let mut pending: Option<String> = None;
            for line in std::io::BufReader::new(fh).lines() {
                let Ok(line) = line else { continue };
                if let Some(stale) = pending.take() {
                    malformed.push(stale); // a later line exists, so it was not a torn tail
                }
                let Some((stamp, stated, hex)) = captured_user_move(&line) else {
                    continue;
                };
                let decoded = unhex(hex);
                match decoded {
                    Some(b) if b.len() == stated => {
                        raw += 1;
                        let where_ = format!("{} {stamp}", path.display());
                        if seen.insert((stamp.to_string(), CLIENT_USER_MOVE, b.clone())) {
                            bodies.push((where_, b));
                        }
                    }
                    other => {
                        pending = Some(format!(
                            "{} {stamp}: stated {stated} bytes, hex carries {}",
                            path.display(),
                            other.map_or_else(|| "odd-length/non-hex".to_string(), |b| b
                                .len()
                                .to_string()),
                        ));
                    }
                }
            }
            if pending.take().is_some() {
                truncated_tails += 1; // last line of the file; a torn copy, not a format change
            }
        }

        let mut not_closed: Vec<String> = Vec::new();
        let mut closed = 0usize;
        let mut elements = 0usize;
        for (where_, b) in &bodies {
            let where_ = format!("{where_} {} bytes", b.len());
            let m = parse_user_move(b)
                .unwrap_or_else(|| panic!("{where_}: shorter than the two heads"));
            if !m.walk_closed {
                not_closed.push(where_);
                continue;
            }

            // The round trip: head + span == body, to the byte.
            let span = m.path_with_key_states_span().expect("a closed walk has a span");
            assert_eq!(span.start, USER_MOVE_HEAD_LEN, "{where_}");
            assert_eq!(
                USER_MOVE_HEAD_LEN + span.len(),
                b.len(),
                "{where_}: the span plus the head must be the whole body"
            );
            let full = m.path_with_key_states(b).expect("a closed walk has a span");
            assert_eq!(full, &b[USER_MOVE_HEAD_LEN..], "{where_}");

            // ...and the short span is the same block minus exactly the key-state trailer.
            let path = m.path(b).expect("a closed walk has a span");
            assert_eq!(path.len(), m.path_len, "{where_}");
            assert_eq!(path.len() + m.key_states_len, full.len(), "{where_}");
            assert_eq!(
                m.key_states_len,
                1 + usize::from(m.key_count).div_ceil(2),
                "{where_}: 141d580e5 plus the packed nibbles"
            );
            assert_eq!(
                b[USER_MOVE_HEAD_LEN + m.path_len],
                m.key_count,
                "{where_}: the count byte is the first byte the short span excludes"
            );
            assert!(path.len() >= MOVE_PATH_HEAD_LEN, "{where_}");
            assert_eq!(
                i16::from_le_bytes([path[12], path[13]]),
                m.element_count,
                "{where_}: 1404b26a9"
            );
            elements += m.element_count.max(0) as usize;
            closed += 1;
        }

        eprintln!(
            "0x00D9 sweep: {} files, {raw} raw body lines, {} distinct (timestamp, opcode, \
             body) events, {closed} walked closed, {} did not, {elements} path elements. \
             {truncated_tails} file(s) ended in a torn line.",
            files.len(),
            bodies.len(),
            not_closed.len(),
        );

        assert!(malformed.is_empty(), "world-log format changed: {malformed:#?}");
        assert!(
            !bodies.is_empty(),
            "{} world logs were readable and not one carried a 0x00D9 - the line format this \
             sweep matches must have changed, because these captures are full of them",
            files.len()
        );
        assert!(
            not_closed.is_empty(),
            "{} of {} distinct bodies did not walk closed with element_len's table: {:#?}",
            not_closed.len(),
            bodies.len(),
            &not_closed[..not_closed.len().min(20)]
        );
        assert_eq!(closed, bodies.len());
    }
}
