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
//! # Nothing here builds a reply, and that is what makes a mis-parse safe
//!
//! This is an inbound-only decode. Every other packet-shaped mistake in this project has
//! cost a session because a wrong length went **out**; the worst this can do is give the
//! caller a stale position. [`parse_user_move`] says so explicitly through
//! [`UserMove::walk_closed`]: when the element walk does not land exactly on the end of the
//! body, the position falls back to the last one it did read rather than the caller getting
//! nothing.
//!
//! **`0x00D9` does not appear to need an answer.** 1082 of them went unanswered across every
//! captured session in `research/fixtures/`, with the client continuing to play for minutes
//! afterwards, so it does not latch the way `0x0107` does. **[D]** That is a measured
//! negative over the captures rather than a proof about the client.

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
/// It is not fitted to the captures; the captures are the check. Every one of **1082**
/// captured `0x00D9` bodies in `research/fixtures/` and `previous-runs/` walks with this
/// table and lands **exactly** on the end of the body. **[D]**
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
    /// so for all 1082 captured bodies. A `false` here means this client emitted a shape the
    /// table does not know, and [`UserMove::x`] is then the last good position rather than a
    /// guess. Nothing is built from this packet, so `false` costs accuracy and nothing else.
    pub walk_closed: bool,
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
    };
    if element_count < 0 {
        return Some(m);
    }

    let mut p = MOVE_PATH_HEAD_LEN;
    for _ in 0..element_count {
        let Some(&command) = path.get(p) else { return Some(m) };
        if element_carries_position(command) {
            let Some(xy) = path.get(p + 1..p + 5) else { return Some(m) };
            m.x = i16::from_le_bytes([xy[0], xy[1]]);
            m.y = i16::from_le_bytes([xy[2], xy[3]]);
        }
        p += element_len(command);
    }

    // The key-state trailer, written by FUN_141d57c60 after the path returns: a count at
    // 0x141d580e5 and then the loop at 0x141d580f0..0x141d5818e, which packs two 4-bit
    // entries into each byte. It is why every body is ten-ish bytes longer than the path.
    let Some(&key_count) = path.get(p) else { return Some(m) };
    p += 1 + usize::from(key_count).div_ceil(2);
    m.key_count = key_count;
    m.walk_closed = p == path.len();
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
}
