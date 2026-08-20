//! `0x0089` — the packet that writes to the client's on-screen message area.
//!
//! Full working, with every address and every control: **`research/client-messages.md`**.
//!
//! Labels are the project's: **[L]** read off this client's listing or its own data,
//! **[D]** derived from two or more [L] facts, **[I]** inferred.
//!
//! # Why this exists
//!
//! The owner, on seeing EXP and pick-up lines in the chat log:
//!
//! > *"When the drops are picked up and when the players receive EXP, it should actually
//! > show on the right hand side of the client. We should not be outputting in the chat log
//! > regarding level ups and item pickups."*
//!
//! This server uses [`crate::notice::chat_notice`] (`0x00BB`) for both, and `0x00BB` is
//! `FUN_1415eca30(&text, 7)` — a **chat-log** post, category 7. **[L]** The message area is
//! a different destination entirely, reached by `FUN_142572050` on the singleton
//! `0x143AD30D8`, and the packet that reaches it is `0x0089`.
//!
//! # The client writes the words; we send numbers
//!
//! `You received EXP (+211)` is composed **inside the client**, from string id `0x00C1`
//! (`'You received EXP (+%lld)'`) and the `u64` in this packet's body. So this is a
//! structured packet, not a formatted-string one, and there is no text field anywhere in
//! it. **[L]** — the format specifier is in the client's own encrypted string table, which
//! `tools/dump_stringids.py` decrypts; its positive control is id `0x533`,
//! `'[Welcome] Welcome to MapleStory!!'`.
//!
//! The owner's screenshot says `You have gained experience (+211)`. That wording is **not** in
//! this client — it is a different build. Ours will read `You received EXP (+211)`.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials.

use crate::packet::PacketWriter;

// ---------------------------------------------------------------------------------------
// Routing
// ---------------------------------------------------------------------------------------

/// The opcode. Identical to [`crate::quest::MESSAGE`], and deliberately re-stated rather
/// than aliased so a reader of either module sees the number.
///
/// `FUN_142cbaa80` case `0x89` -> `FUN_142d43ee0`, which reads one `u8` at `0x142d43f0c`,
/// bounds it at `0x23` and jumps through a 36-entry dword table at `0x142d44a88`. **[L]**
pub const MESSAGE: u16 = 0x0089;

/// The first body byte: which kind of message this is.
///
/// Every value here is read off the jump table at `0x142d44a88` and the handler it lands
/// on. **[L]** The 28 values with no constant are unread — see
/// `research/client-messages.md` §2 — and **19, 29, 32 and 33 have no handler at all**,
/// sharing the out-of-range target `0x142d44a6a`.
pub mod kind {
    /// A drop was picked up. `FUN_142d59360`, and it takes a second, **signed**, sub-mode
    /// byte — see [`super::pickup`]. **[L]**
    pub const DROP_PICKUP: u8 = 0;
    /// A quest changed state. `FUN_142d59e20`, already decoded in
    /// [`crate::quest`] as `MESSAGE_QUEST_RECORD`. **[L]**
    pub const QUEST_RECORD: u8 = 1;
    /// **Experience gained.** `FUN_142d5da20`. **[L]**
    pub const EXPERIENCE: u8 = 3;
    /// Skill points. Inline at `0x142d43fc0`: `u16`, `u8`, then string `0xC3`
    /// (`'You have received some SP! (+%d)'`) or `0xC4`/`0xC5` for a job-advancement
    /// grant. **Not built here** — the two fields are not both understood. **[L]** for the
    /// widths.
    pub const SKILL_POINTS: u8 = 4;
    /// Fame. Inline at `0x142d44070`: one `i32`; `>= 0` uses string `0xDF`
    /// (`'You have gained fame. (+%d)'`), `< 0` uses `0xE0`. **[L]**
    pub const FAME: u8 = 5;
    /// Mesos, as a *stat change* rather than a pick-up. `FUN_142d5f990`, `u64, u32, str`,
    /// string `0xE1`. **Not built here** — the trailing string is unread. **[L]**
    pub const MESO: u8 = 6;
}

/// The second body byte of [`kind::DROP_PICKUP`], read **signed** at `0x142d59393`.
///
/// `4` returns immediately and `-1` sets `world+0x37a4 = 1` and returns; everything else
/// indexes an 11-entry table at `0x142d59d60` with `index = mode + 5`. **[L]**
pub mod pickup {
    /// An item entered the bag. `u32 itemId, u32 count, u8 where`. **[L]**
    pub const ITEM: i8 = 0;
    /// Mesos entered the purse. `u8, i32 gain, u16 smallChange, i32 bonus`. **[L]**
    pub const MESO: i8 = 1;
}

/// The `where` byte of [`pickup::ITEM`], which picks the sentence. **[L]**
///
/// ```text
/// 0 -> string 0xEC  '%s x%d earned. (%s)'
/// 1 -> string 0xEE  '%s x%d earned. (%s and Bag)'
/// 2 -> string 0xED  '%s x%d earned. (%s / Bag)'
/// ```
///
/// **Anything else abandons the message**: `142d5978b test dil,dil / je`,
/// `142d59790 sub ecx,1 / je`, `142d59795 cmp ecx,1 / je`, and the fall-through is
/// `jmp 0x142d59d22`, the handler's exit. So this is a three-value enum, not a flag.
pub mod item_slot {
    /// The ordinary inventory. **[L]**
    pub const INVENTORY: u8 = 0;
    /// `'… (%s and Bag)'`. **[L]**
    pub const INVENTORY_AND_BAG: u8 = 1;
    /// `'… (%s / Bag)'`. **[L]**
    pub const BAG: u8 = 2;
}

// ---------------------------------------------------------------------------------------
// Experience
// ---------------------------------------------------------------------------------------

/// The body of a [`kind::EXPERIENCE`] message with no bonus lines.
///
/// `FUN_1408cfcf0(dst, packet)` is the whole reader, and the caller `FUN_142d5da20`
/// **zeroes the destination struct** (`142d5da5e`..`142d5daf6`) before calling it. That is
/// what makes the short form safe: **[L]**
///
/// ```text
/// u8   white        -> dst+0x00   1408cfd0b   colour selector on the message-area post
/// u64  exp          -> dst+0x08   1408cfd18   the number the sentence is built from
/// u8   in_chat      -> dst+0x10   1408cfd24   0 = the message area, non-zero = the chat log
/// u64  bonus_mask   -> a local    1408cfd44   raw(8); each set bit costs a further field
/// [u8  quest_bonus  -> dst+0x24]  1408cfd7e   ONLY when in_chat != 0
/// [u8               -> dst+0x28]  1408cfd95   ONLY when quest_bonus > 0
/// ```
///
/// With `in_chat == 0` and `bonus_mask == 0` the reader performs exactly four reads and
/// stops: the `dst+0x10 == 0` branch reads `dst+0x24` **out of the pre-zeroed struct**
/// rather than off the wire, and `test eax,eax / jle` then skips the last conditional `u8`.
/// The body is `1 + 8 + 1 + 8` = [`EXPERIENCE_BODY_LEN`] bytes plus the kind byte. **[L]**
///
/// **`in_chat` costs a byte of its own.** `1408cfd75 cmp dword [rbx+0x10],0 / je` means a
/// non-zero third field makes the client read a fifth field. This function writes it — a
/// zero, which then makes `1408cfd8e test eax,eax / jle` skip the sixth. Getting that wrong
/// sends a body one byte short, which is how `CLAUDE.md` records this client being killed
/// twice.
///
/// # `in_chat` is the byte the owner's request turns on
///
/// ```asm
/// 142d5efe9  cmp  dword [rbp+0xc0], 0        ; this field
/// 142d5eff0  jne  142d5f4d0                  ;   non-zero -> FUN_1415eca30(text, 6), the CHAT LOG
/// 142d5eff6  mov  rcx, [rip -> 0x143AD30D8]
/// 142d5effd  je   142d5f4d0                  ;   no message-area object -> chat log anyway
/// 142d5f027  call 0x142572050                ;   zero -> the MESSAGE AREA
/// ```
///
/// **[L]** for the fork and for both destinations being distinct. That the message-area
/// destination is specifically the bottom-right stack in the owner's screenshot is **[I]** — see
/// `research/client-messages.md` §7, which also gives the one-byte experiment that settles
/// it.
///
/// # `white`
///
/// `142d5f014 cmp dword [rbp+0xb0], ebx / je` picks `r8d = 4` when the byte is zero and
/// `r8d = 0` when it is not, and `r8d` is `FUN_142572050`'s third argument. **[L]** for the
/// branch; what it does on screen is **not established**, which is why [`exp_gained`] sends
/// the zero the client's other senders would.
///
/// # No mask parameter, on purpose
///
/// Every set mask bit makes the client read further fields, and a mask with no fields
/// behind it underruns its packet reader. Use [`experience_with_bonuses`], which writes
/// them in the client's own read order and refuses a bit it does not know the width of.
pub fn experience(exp: u64, white: bool, in_chat: bool) -> Vec<u8> {
    experience_with_bonuses(exp, white, in_chat, &[])
}

/// Bytes an [`kind::EXPERIENCE`] body costs after the kind byte, with `in_chat = false`
/// and no bonuses.
pub const EXPERIENCE_BODY_LEN: usize = 1 + 8 + 1 + 8;

/// One extra EXP line under the main one, selected by a mask bit.
///
/// The bit -> string mapping is in `research/client-messages.md` §3; e.g. `0x80000` is
/// `'You received bonus EXP (+%lld)'` and `0x1000000` is `'Field Bonus EXP (+%lld)'`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpBonus {
    /// One of [`EXP_BONUS_FIELDS`]' bits. Any other value is refused.
    pub bit: u64,
    /// The amount. Written as the width [`EXP_BONUS_FIELDS`] gives for this bit.
    pub value: u64,
    /// Only bit `0x8000` carries a second field, a `u32` at `dst+0x100` (`1408d00af`).
    /// Ignored for every other bit. **[L]**
    pub tail: u32,
}

/// Every mask bit `FUN_1408cfcf0` tests, **in the order it reads them**, with the width of
/// the field behind each. **[L]**
///
/// The order is not the numeric order and that matters: `0x20` is read **before** `0x10`
/// (`1408cfda0` then `1408cfdb8`), and `0x8000` is read **last of all**, after
/// `0x100000000`, carrying two fields (`1408d009a` raw8 then `1408d00af` raw4).
///
/// Bits `0x02`, `0x08`, `0x400000`, `0x8000000`, `0x40000000` and everything above bit 32
/// are **never tested**: a body setting one is read exactly as if it were clear.
pub const EXP_BONUS_FIELDS: &[(u64, usize)] = &[
    (0x0000_0001, 8), // 1408cfd5c -> dst+0x18
    (0x0000_0004, 1), // 1408cfd6a -> dst+0x20   the only ONE-byte bonus
    (0x0000_0020, 8), // 1408cfdb3 -> dst+0x38   read BEFORE 0x10
    (0x0000_0010, 8), // 1408cfdcb -> dst+0x30
    (0x0000_0040, 8), // 1408cfde3 -> dst+0x40
    (0x0000_0080, 8), // 1408cfdfb -> dst+0x48
    (0x0000_0100, 8), // 1408cfe17 -> dst+0x50
    (0x0000_0200, 8), // 1408cfe33 -> dst+0x58
    (0x0000_0400, 8), // 1408cfe4f -> dst+0x60
    (0x0000_0800, 8), // 1408cfe6b -> dst+0x68
    (0x0000_1000, 8), // 1408cfe87 -> dst+0x70
    (0x0000_2000, 8), // 1408cfea3 -> dst+0x78
    (0x0000_4000, 8), // 1408cfec2 -> dst+0x80
    (0x0001_0000, 8), // 1408cfee1 -> dst+0x88
    (0x0002_0000, 8), // 1408cff00 -> dst+0x90
    (0x0004_0000, 8), // 1408cff1f -> dst+0x98
    (0x0008_0000, 8), // 1408cff3e -> dst+0xa0   'You received bonus EXP'
    (0x0010_0000, 8), // 1408cff5d -> dst+0xa8
    (0x0020_0000, 8), // 1408cff7c -> dst+0xb0
    (0x0080_0000, 8), // 1408cff9b -> dst+0xb8
    (0x0100_0000, 8), // 1408cffba -> dst+0xc8
    (0x0200_0000, 8), // 1408cffd9 -> dst+0xd0
    (0x0400_0000, 8), // 1408cfff8 -> dst+0xd8
    (0x1000_0000, 8), // 1408d0017 -> dst+0xe0
    (0x2000_0000, 8), // 1408d0036 -> dst+0xe8
    (0x8000_0000, 8), // 1408d0056 -> dst+0xf0
    (0x1_0000_0000, 8), // 1408d007b -> dst+0xc0
    (0x0000_8000, 12), // 1408d009a raw8 -> dst+0xf8 AND 1408d00af raw4 -> dst+0x100
];

/// The full [`kind::EXPERIENCE`] body, with any number of bonus lines.
///
/// Fields go out in [`EXP_BONUS_FIELDS`] order regardless of the order they are passed in,
/// because that is the order the client reads them.
///
/// # Panics
///
/// On a `bit` that is not in [`EXP_BONUS_FIELDS`], and on a duplicate bit. Both would make
/// the mask and the fields behind it disagree, and the client has no length prefix to
/// resynchronise on.
pub fn experience_with_bonuses(
    exp: u64,
    white: bool,
    in_chat: bool,
    bonuses: &[ExpBonus],
) -> Vec<u8> {
    let mut mask = 0u64;
    for b in bonuses {
        assert!(
            EXP_BONUS_FIELDS.iter().any(|(bit, _)| *bit == b.bit),
            "{:#x} is not a bit FUN_1408cfcf0 reads; the body would be short",
            b.bit
        );
        assert!(mask & b.bit == 0, "bit {:#x} given twice", b.bit);
        mask |= b.bit;
    }

    let mut w = PacketWriter::new();
    w.u8(kind::EXPERIENCE);
    w.bool(white);
    w.u64(exp);
    w.bool(in_chat);
    w.u64(mask);

    // 1408cfd75: a non-zero `in_chat` makes the client read one more u8, and a zero there
    // makes 1408cfd8e skip the one after it. Both or neither.
    if in_chat {
        w.u8(0);
    }

    for (bit, width) in EXP_BONUS_FIELDS {
        let Some(b) = bonuses.iter().find(|b| b.bit == *bit) else {
            continue;
        };
        match width {
            1 => {
                w.u8(b.value as u8);
            }
            12 => {
                w.u64(b.value);
                w.u32(b.tail);
            }
            _ => {
                w.u64(b.value);
            }
        }
    }
    w.into_vec()
}

/// `You received EXP (+n)`, in the message area, nothing else. **This is the one to use.**
///
/// `in_chat = false` keeps it out of the chat log — which is the whole of the owner's request —
/// and no bonuses keeps the body at its minimum, so no field the client reads is left
/// unwritten.
///
/// It is **cosmetic and independent of the EXP itself**: the bar moves because of
/// [`crate::combat::stat_changed`] (`0x007C`, bit `EXP`), which carries the new total.
/// Send both; neither implies the other.
pub fn exp_gained(exp: u64) -> Vec<u8> {
    experience(exp, false, false)
}

// ---------------------------------------------------------------------------------------
// Pick-ups
// ---------------------------------------------------------------------------------------

/// The body of a [`pickup::ITEM`] message.
///
/// ```text
/// u8   0                    the kind, DROP_PICKUP
/// u8   quiet                142d59380, read BEFORE the sub-mode; see below
/// i8   0                    the sub-mode, ITEM
/// u32  itemId               142d59716
/// u32  count                142d59720
/// u8   where                142d5972a, one of item_slot::*
/// ```
///
/// **[L]** throughout.
///
/// # Two ways the client draws nothing
///
/// * **An item id it cannot name.** The client resolves the name itself
///   (`FUN_140398ba0`) and bails at `142d59752`/`142d5975f` on a null or empty one. So an
///   id outside its `itemdata` produces silence, not a broken line.
/// * **`count <= 0`.** `142d59814 test esi,esi / jle` skips the formatting but **not** the
///   post, leaving the client to post a string it never built. [`item_gained`] refuses to
///   build that body.
///
/// # `quiet`
///
/// The first `u8` is shared by every sub-mode. In the meso arm, non-zero accumulates the
/// gain into `world+0x37a0` instead of calling `FUN_142d9bae0(world, gain)` — the meso-gain
/// effect. **[L]** for the branch, **[I]** for the name. Nothing in the item arm reads it.
pub fn item_pickup(item_id: u32, count: u32, slot: u8, quiet: bool) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(kind::DROP_PICKUP);
    w.bool(quiet);
    w.i8(pickup::ITEM);
    w.u32(item_id);
    w.u32(count);
    w.u8(slot);
    w.into_vec()
}

/// `<name> xN earned. (<tab>)`, in the message area. **Use this one.**
///
/// # Panics
///
/// When `count == 0`. A zero count makes the client skip building the string and then post
/// it anyway; refusing here is cheaper than finding out on the owner's screen.
pub fn item_gained(item_id: u32, count: u32) -> Vec<u8> {
    assert!(
        count >= 1,
        "count 0 makes the client post a string it never formatted (142d59814 jle)"
    );
    item_pickup(item_id, count, item_slot::INVENTORY, false)
}

/// The body of a [`pickup::MESO`] message.
///
/// ```text
/// u8   0                    the kind, DROP_PICKUP
/// u8   quiet                142d59380
/// i8   1                    the sub-mode, MESO
/// u8   notice_lost          142d593e7  non-zero -> string 0xA6, 'A portion was not found
///                                      after falling on the ground.', in the CHAT LOG
/// i32  gain                 142d593f3  the TOTAL, bonus included
/// u16  small_change         142d593fe  non-zero -> string 0xE5, 'Spotting Small Change'
/// i32  bonus                142d5940a  > 0 -> 0xE2 'additional Mesos'
///                                      < 0 -> 0xE3 'Meso Penalty Applied'
/// ```
///
/// The plain line is `gain - bonus`: `142d59482 mov ebx,r14d / sub ebx,edi`, then string
/// `0xE1` `'You have gained mesos (+%I64d)'`. **[L]**
///
/// A cap check runs first — `FUN_142cbec20(world)` against `0x746a_5287_fe`
/// (499 999 999 998) — and over it the client shows string `0x12D3`
/// `'You cannot obtain any more Mesos.'` and no gain line. **[L]**
pub fn meso_pickup(
    gain: i32,
    bonus: i32,
    small_change: u16,
    notice_lost: bool,
    quiet: bool,
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(kind::DROP_PICKUP);
    w.bool(quiet);
    w.i8(pickup::MESO);
    w.bool(notice_lost);
    w.i32(gain);
    w.u16(small_change);
    w.i32(bonus);
    w.into_vec()
}

/// `You have gained mesos (+n)`, in the message area, nothing else. **Use this one.**
pub fn meso_gained(gain: i32) -> Vec<u8> {
    meso_pickup(gain, 0, 0, false, false)
}

// ---------------------------------------------------------------------------------------
// Fame — small, fully read, and here because it is one field
// ---------------------------------------------------------------------------------------

/// The body of a [`kind::FAME`] message: one `i32`, read at `0x142d44073`. **[L]**
///
/// `142d4407e test eax,eax / jns` picks string `0xDF` `'You have gained fame. (+%d)'` for
/// `>= 0` and `0xE0` `'You have lost fame (%d)'` for negative, and the line is posted with
/// `FUN_1415eca30(text, 6)` — **the chat log** — with no message-area post and no flag to
/// change that. **[L]**
pub fn fame(delta: i32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(kind::FAME);
    w.i32(delta);
    w.into_vec()
}

// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// The routing constants, so a future edit that renumbers one fails here.
    #[test]
    fn the_opcode_and_the_kind_bytes_are_the_jump_table_entries() {
        assert_eq!(MESSAGE, 0x0089); // FUN_142cbaa80 case 0x89 -> FUN_142d43ee0
        assert_eq!(MESSAGE, crate::quest::MESSAGE, "one packet, two modules");
        assert_eq!(kind::DROP_PICKUP, 0); // table[0]  = 142d43f32 -> FUN_142d59360
        assert_eq!(kind::QUEST_RECORD, crate::quest::MESSAGE_QUEST_RECORD);
        assert_eq!(kind::EXPERIENCE, 3); // table[3]  = 142d43fb0 -> FUN_142d5da20
        assert_eq!(kind::SKILL_POINTS, 4); // table[4]  = 142d43fc0
        assert_eq!(kind::FAME, 5); // table[5]  = 142d44070
        assert_eq!(kind::MESO, 6); // table[6]  = 142d44109 -> FUN_142d5f990
    }

    /// Every byte of the message the owner asked for, in order, against `FUN_1408cfcf0`.
    #[test]
    fn the_experience_body_is_the_four_fields_the_reader_reads() {
        let b = exp_gained(211);
        assert_eq!(b.len(), 1 + EXPERIENCE_BODY_LEN, "19 bytes with the kind byte");
        assert_eq!(b[0], kind::EXPERIENCE);
        assert_eq!(b[1], 0, "white   -> dst+0x00, 1408cfd0b");
        assert_eq!(
            u64::from_le_bytes(b[2..10].try_into().unwrap()),
            211,
            "exp -> dst+0x08, 1408cfd18; the ONLY number the sentence needs"
        );
        assert_eq!(b[10], 0, "in_chat -> dst+0x10, 1408cfd24: 0 = the message area");
        assert_eq!(
            u64::from_le_bytes(b[11..19].try_into().unwrap()),
            0,
            "bonus mask -> the raw8 at 1408cfd44"
        );
    }

    /// **The one that matters for the owner's request.** A non-zero third field is what sends
    /// the line to the chat log instead (`142d5efe9 cmp / jne 142d5f4d0`), so the default
    /// helper must never set it.
    #[test]
    fn exp_gained_keeps_the_line_out_of_the_chat_log() {
        assert_eq!(exp_gained(1)[10], 0);
        assert_eq!(experience(1, false, true)[10], 1, "the other way round");
    }

    /// **The byte that would have gone out short.** `1408cfd75 cmp dword [rbx+0x10],0 / je`
    /// makes a non-zero `in_chat` cost a fifth field. A body without it underruns the
    /// client's reader.
    #[test]
    fn in_chat_costs_a_byte_of_its_own() {
        assert_eq!(experience(7, false, false).len(), 1 + EXPERIENCE_BODY_LEN);
        let chatty = experience(7, false, true);
        assert_eq!(chatty.len(), 1 + EXPERIENCE_BODY_LEN + 1);
        assert_eq!(
            *chatty.last().unwrap(),
            0,
            "and it must be zero, or 1408cfd8e reads a SIXTH field"
        );
    }

    /// Every set mask bit has its field written, in the client's read order — which is not
    /// the numeric order.
    #[test]
    fn bonus_fields_go_out_in_the_clients_read_order() {
        let b = experience_with_bonuses(
            50,
            true,
            false,
            &[
                ExpBonus { bit: 0x10, value: 3, tail: 0 },
                ExpBonus { bit: 0x20, value: 9, tail: 0 },
            ],
        );
        assert_eq!(b[1], 1, "white");
        assert_eq!(u64::from_le_bytes(b[11..19].try_into().unwrap()), 0x30, "the mask");
        assert_eq!(b.len(), 1 + EXPERIENCE_BODY_LEN + 16);
        // 1408cfdb3 (bit 0x20) is read BEFORE 1408cfdcb (bit 0x10).
        assert_eq!(u64::from_le_bytes(b[19..27].try_into().unwrap()), 9, "bit 0x20 first");
        assert_eq!(u64::from_le_bytes(b[27..35].try_into().unwrap()), 3, "then bit 0x10");
    }

    /// Bit `0x04` is one byte and bit `0x8000` is twelve. A uniform 8 would desynchronise
    /// everything after them.
    #[test]
    fn the_two_odd_width_bonus_bits_are_not_eight_bytes() {
        let one = experience_with_bonuses(1, false, false, &[ExpBonus { bit: 4, value: 2, tail: 0 }]);
        assert_eq!(one.len(), 1 + EXPERIENCE_BODY_LEN + 1);
        assert_eq!(*one.last().unwrap(), 2);

        let two = experience_with_bonuses(
            1,
            false,
            false,
            &[ExpBonus { bit: 0x8000, value: 5, tail: 6 }],
        );
        assert_eq!(two.len(), 1 + EXPERIENCE_BODY_LEN + 12);
        assert_eq!(u64::from_le_bytes(two[19..27].try_into().unwrap()), 5);
        assert_eq!(u32::from_le_bytes(two[27..31].try_into().unwrap()), 6);
    }

    /// A bit the client never tests would set the mask and write a field the client does
    /// not read - a body one field long. Refused rather than sent.
    #[test]
    #[should_panic(expected = "is not a bit")]
    fn an_untested_mask_bit_is_refused() {
        let _ = experience_with_bonuses(1, false, false, &[ExpBonus { bit: 2, value: 0, tail: 0 }]);
    }

    /// The ordered field table must contain no duplicates and only bits the listing shows
    /// being tested.
    #[test]
    fn the_bonus_field_table_is_a_set() {
        let mut seen = 0u64;
        for (bit, width) in EXP_BONUS_FIELDS {
            assert!(seen & bit == 0, "{bit:#x} listed twice");
            assert!(bit.count_ones() == 1, "{bit:#x} is not a single bit");
            assert!(matches!(width, 1 | 8 | 12), "{bit:#x} has width {width}");
            seen |= bit;
        }
        assert_eq!(EXP_BONUS_FIELDS.len(), 28, "28 bits are tested, not 32");
        // The bits FUN_1408cfcf0 never tests.
        for absent in [0x2u64, 0x8, 0x40_0000, 0x800_0000, 0x4000_0000] {
            assert!(seen & absent == 0, "{absent:#x} is not tested by the client");
        }
    }

    /// The item pick-up, field for field against `FUN_142d59360`'s ITEM arm.
    #[test]
    fn the_item_pickup_body_matches_the_item_arm() {
        let b = item_gained(4000019, 3);
        assert_eq!(b.len(), 1 + 1 + 1 + 4 + 4 + 1);
        assert_eq!(b[0], kind::DROP_PICKUP);
        assert_eq!(b[1], 0, "quiet, the u8 at 142d59380");
        assert_eq!(b[2] as i8, pickup::ITEM, "the SIGNED sub-mode at 142d59393");
        assert_eq!(u32::from_le_bytes(b[3..7].try_into().unwrap()), 4000019);
        assert_eq!(u32::from_le_bytes(b[7..11].try_into().unwrap()), 3);
        assert_eq!(b[11], item_slot::INVENTORY, "142d5972a; 3+ abandons the message");
    }

    /// `where` is a three-value enum, not a flag: `142d5978b`/`142d59790`/`142d59795`, with
    /// the fall-through jumping to the handler's exit.
    #[test]
    fn the_item_slot_byte_has_exactly_three_legal_values() {
        for slot in [item_slot::INVENTORY, item_slot::INVENTORY_AND_BAG, item_slot::BAG] {
            assert!(slot <= 2);
            assert_eq!(item_pickup(1302000, 1, slot, false)[11], slot);
        }
    }

    /// A zero count makes the client skip the formatting and post the unbuilt string.
    #[test]
    #[should_panic(expected = "count 0")]
    fn a_zero_count_is_refused_rather_than_sent() {
        let _ = item_gained(1302000, 0);
    }

    /// The meso pick-up, field for field, including the two signed fields.
    #[test]
    fn the_meso_pickup_body_matches_the_meso_arm() {
        let b = meso_pickup(120, -20, 7, true, false);
        assert_eq!(b.len(), 1 + 1 + 1 + 1 + 4 + 2 + 4);
        assert_eq!(b[0], kind::DROP_PICKUP);
        assert_eq!(b[1], 0, "quiet");
        assert_eq!(b[2] as i8, pickup::MESO);
        assert_eq!(b[3], 1, "notice_lost -> string 0xA6, 142d593e7");
        assert_eq!(i32::from_le_bytes(b[4..8].try_into().unwrap()), 120);
        assert_eq!(u16::from_le_bytes(b[8..10].try_into().unwrap()), 7);
        assert_eq!(
            i32::from_le_bytes(b[10..14].try_into().unwrap()),
            -20,
            "a negative bonus is string 0xE3, 'Meso Penalty Applied'"
        );
    }

    /// The plain line the client draws is `gain - bonus` (`142d59482 sub ebx,edi`), so the
    /// simple helper must leave `bonus` at zero or the number on screen is not the number
    /// we meant.
    #[test]
    fn meso_gained_leaves_the_bonus_at_zero_so_the_line_reads_the_full_gain() {
        let b = meso_gained(1234);
        assert_eq!(i32::from_le_bytes(b[4..8].try_into().unwrap()), 1234);
        assert_eq!(i32::from_le_bytes(b[10..14].try_into().unwrap()), 0);
        assert_eq!(u16::from_le_bytes(b[8..10].try_into().unwrap()), 0);
        assert_eq!(b[3], 0, "no 'a portion was lost' line");
    }

    /// Fame is one `i32` and the sign picks the sentence.
    #[test]
    fn fame_is_one_signed_int() {
        assert_eq!(fame(5), vec![kind::FAME, 5, 0, 0, 0]);
        let lost = fame(-5);
        assert_eq!(lost[0], kind::FAME);
        assert_eq!(i32::from_le_bytes(lost[1..5].try_into().unwrap()), -5);
    }

    /// `0x00BB` is still the right tool for GM feedback and this module must not be
    /// mistaken for a replacement for it. The two have different destinations, and the
    /// difference is the whole point of this file.
    #[test]
    fn this_is_not_the_chat_notice() {
        assert_ne!(MESSAGE, crate::notice::CHAT_NOTICE);
    }
}
