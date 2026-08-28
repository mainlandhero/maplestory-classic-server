//! The client's attack packet, parsed field for field — including the **skill id**.
//!
//! Opcodes [`crate::combat::USER_MELEE_ATTACK`] (`0x00DF`),
//! [`crate::combat::USER_SHOOT_ATTACK`] (`0x00E0`) and
//! [`crate::combat::USER_MAGIC_ATTACK`] (`0x00E1`) share **one** body: three group-A
//! builders call the same header encoder `FUN_140f31fe0`, the same target-list encoder
//! `FUN_140f31f60` and the same trailer `FUN_14083b270`. The world log says so on every
//! line — `0x00E1 CLIENT_MAGIC_ATTACK (same body as 0x00DF)`. **[L]**
//!
//! `0x00E2` is **not** in that set. It uses the classic inline shape and never calls
//! `FUN_140f31fe0`; see `research/mob-combat.md` §1.2.
//!
//! Working: `research/mob-combat.md` §1.1–§1.6, `research/attack-skill-id.md`,
//! `research/damage-formula.md` §3.
//!
//! # What is measured, and against how much
//!
//! Every claim below is checked against **434 real captured bodies from 25 distinct
//! captures** — every `0x00DF`/`0x00E0`/`0x00E1` this repository holds.
//! `tools/extract_attack_bodies.py` is the extractor; re-run it over a fresh capture and
//! point [`env::CORPUS`] at the result to re-run the sweep.
//!
//! ```text
//!   n    opcode   skill id   level   attack type                body len
//! 353    0x00DF          0       0   "User Melee"                    229
//!  43    0x00DF          0       0   "User Melee"                    127   <- no target
//!  29    0x00DF          0       0   "User Melee"                    221
//!   4    0x00E1    2001003       7   "User Magic Skill System"       260
//!   3    0x00E1    2001003       7   "User Magic Skill System"       268
//!   1    0x00DF          0       0   "User Melee"                    213
//!   1    0x00E1       1000       3   "User Magic"                    229
//! ```
//!
//! **All 434 are consumed to the last byte by this parser** — and 7 of them only because
//! the trailer's conditional tail is included, which is where `research/mob-combat.md` §1.4
//! is wrong. See [`AttackTrailer`].
//!
//! ## 434, not 724: `research/fixtures/` holds copies of `previous-runs/`
//!
//! Scanning both directories naively finds **724 bodies in 45 files**, and **20 of those 45
//! are byte-identical copies of another** (36 duplicates across all 156 world logs in the
//! repo) — `attack-skill-id-three-snails-control-world.log` *is*
//! `previous-runs/world-20260822-213856.log` *is* `cash-shop-click-sent-nothing-world.log`,
//! and the Magic Claw run is present three times (two fixture names plus the live
//! `world.log`). `--dedupe` on the extractor drops them and names what it dropped.
//!
//! This matters to one number in particular. `research/attack-skill-id.md` §1 reports
//! **14 Magic Claw casts and 2 Three Snails**. Both are exactly doubled: the client sent
//! **7 Magic Claws in one session and one single Three Snails**. The *finding* is
//! untouched — the corroboration was never the count, it was that two skills carried two
//! levels that each match a number known from an unrelated source — but the Three Snails
//! control is **one packet**, and anything that leans on it should say so.
//!
//! `research/damage-formula.md` §3 spotted one of these pairs by hand and warned about
//! pooling them. There are 36.
//!
//! Nothing here says the copies should not exist — `research/fixtures/` is a deliberate
//! rescue from a rolling buffer and `CLAUDE.md` asks for it. It says that **counting a
//! corpus assembled from both directories is counting sessions twice**, and that the
//! extractor should be the thing that knows it.
//!
//! # The target block is NOT inferred against this client
//!
//! An earlier round of this work had to hedge the target block as `[I]`, because the only
//! capture then available carried zero targets. That is no longer true:
//!
//! * **391 of 434 bodies carry `targetCount == 1`**, with a real mob object id and a real
//!   client-computed damage. 43 carry zero, and those are the pre-mob-size-fix era.
//! * **No body has ever carried more than one target**, so the per-target *stride* is
//!   exercised by nothing. A two-target swing is the one shape this parser has never seen.
//! * **No target has ever set `has_object`** ([`AttackTarget::has_object`]) or a non-zero
//!   [`AttackTarget::mode`], so those two branches are still `[L]` from the listing and
//!   `[I]` against the wire.
//!
//! # Always answer
//!
//! `CLAUDE.md`: an unanswered packet freezes the client's entire UI, quit prompt included.
//! [`parse`] returning `Err` means *"this parser did not understand the bytes"* and **never**
//! means *"do not reply"*. Log the [`AttackError`], apply what you can, answer anyway.
//! [`Attack::truncated`] is the softer half of the same rule: a target whose optional
//! sub-object could not be stepped over stops the walk, keeps everything read so far, and
//! is still a perfectly good basis for a reply.
//!
//! # This overlaps `crate::combat::parse_attack`
//!
//! [`crate::combat::parse_attack`] skips the header and keeps `tick`, `x`, `y`,
//! `attack_type` and the targets. It is **correct on everything it reads** — this module
//! does not retract any of it — but it cannot see the skill id, it types the coordinates
//! unsigned, and it stops one `u8` into the trailer. Nothing here changes `combat.rs`;
//! the coordinator decides which one the session uses.

use crate::combat::{is_attack_opcode, USER_MAGIC_ATTACK, USER_MELEE_ATTACK, USER_SHOOT_ATTACK};
use crate::packet::PacketReader;

// -------------------------------------------------------------------------------------
// The header's shape, as a table rather than as three magic numbers
// -------------------------------------------------------------------------------------

/// One writer call in `FUN_140f31fe0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldWidth {
    /// `call 0x1406ed840`, one byte.
    U8,
    /// `call 0x1406ed940`, two bytes.
    U16,
    /// `call 0x1406ed9d0`, four bytes.
    U32,
    /// `call 0x1406edc80`, a `u16` byte-length then that many bytes.
    Str,
}

impl FieldWidth {
    /// Bytes on the wire, for the fixed widths. `Str` has no fixed size.
    const fn fixed_bytes(self) -> usize {
        match self {
            FieldWidth::U8 => 1,
            FieldWidth::U16 => 2,
            FieldWidth::U32 => 4,
            FieldWidth::Str => 0,
        }
    }
}

/// Every field `FUN_140f31fe0` writes, in order, with the offset it lands on in the source
/// object.
///
/// `.pdata` bounds the encoder to `0x140f31fe0..0x140f321f0`. It contains **39 `call`s to
/// writer primitives and one `jmp` tail-call** to the `u8` writer at `0x140f321eb`, and no
/// conditional branch of any kind — so the header is unconditional and this table is the
/// whole of it. **[L]**
///
/// # The tail jump is a field, and this repo has lost a byte to it twice
///
/// A scan that counts only `call` sees **39** fields, makes the header 97 fixed bytes, and
/// then has to invent a byte somewhere else to reconcile with the 127-byte capture. That
/// happened once in the scan and once again by hand while the table was being written up.
/// `the_tail_jump_is_the_fortieth_field` pins it, so the census fails in `cargo test` rather
/// than on the wire.
///
/// The `Option<u16>` is the field's offset in the source struct, from
/// `research/mob-combat.md` §1.3. `None` is a `setne dl` — a boolean computed into a
/// register, with no struct offset to name.
pub const FIELD_WIDTHS: [(FieldWidth, Option<u16>); 40] = [
    (FieldWidth::U8, Some(0x00)),  //  0  body   0
    (FieldWidth::U8, Some(0x04)),  //  1  body   1
    (FieldWidth::U32, Some(0x08)), //  2  body   2   skill id
    (FieldWidth::U8, Some(0x0c)),  //  3  body   6   skill level
    (FieldWidth::U8, None),        //  4  body   7   setne dl
    (FieldWidth::U32, Some(0x14)), //  5  body   8
    (FieldWidth::U32, Some(0x18)), //  6  body  12
    (FieldWidth::U8, None),        //  7  body  16   setne dl
    (FieldWidth::U32, Some(0x20)), //  8  body  17
    (FieldWidth::U32, Some(0x24)), //  9  body  21
    (FieldWidth::U8, Some(0x28)),  // 10  body  25
    (FieldWidth::U32, Some(0x2c)), // 11  body  26
    (FieldWidth::U32, Some(0x30)), // 12  body  30   tick
    (FieldWidth::U16, Some(0x34)), // 13  body  34   x
    (FieldWidth::U16, Some(0x38)), // 14  body  36   y
    (FieldWidth::U32, Some(0x3c)), // 15  body  38
    (FieldWidth::U16, Some(0x40)), // 16  body  42
    (FieldWidth::U16, Some(0x44)), // 17  body  44
    (FieldWidth::U8, None),        // 18  body  46   setne dl
    (FieldWidth::U8, Some(0x48)),  // 19  body  47
    (FieldWidth::U8, None),        // 20  body  48   setne dl
    (FieldWidth::U8, None),        // 21  body  49   setne dl
    (FieldWidth::U32, Some(0x50)), // 22  body  50
    (FieldWidth::U32, Some(0x54)), // 23  body  54
    (FieldWidth::U16, Some(0x58)), // 24  body  58
    (FieldWidth::U32, Some(0x60)), // 25  body  60
    (FieldWidth::U16, Some(0x64)), // 26  body  64
    (FieldWidth::U16, Some(0x68)), // 27  body  66
    (FieldWidth::U16, Some(0x6c)), // 28  body  68
    (FieldWidth::U16, Some(0x70)), // 29  body  70
    (FieldWidth::U32, Some(0x74)), // 30  body  72
    (FieldWidth::U8, None),        // 31  body  76   setne dl
    (FieldWidth::U32, Some(0x7c)), // 32  body  77
    (FieldWidth::Str, Some(0x80)), // 33  body  81   attack type
    (FieldWidth::U32, Some(0x88)), // 34  body  93
    (FieldWidth::U32, Some(0x90)), // 35  body  97
    (FieldWidth::U32, Some(0x94)), // 36  body 101
    (FieldWidth::U16, Some(0x98)), // 37  body 105
    (FieldWidth::U16, Some(0x9c)), // 38  body 107
    (FieldWidth::U8, Some(0xa4)),  // 39  body 109  <- the tail jmp at 0x140f321eb
];

/// Bytes the header costs on top of its one variable-length string.
///
/// Derived from [`FIELD_WIDTHS`], not written down: **12 × `u8` + 11 × `u16` + 16 × `u32`
/// = 98**. A `"User Melee"` header is `98 + 2 + 10` = **110**. **[D]**
pub const HEADER_FIXED_LEN: usize = {
    let mut total = 0;
    let mut i = 0;
    while i < FIELD_WIDTHS.len() {
        total += FIELD_WIDTHS[i].0.fixed_bytes();
        i += 1;
    }
    total
};

/// `FUN_140f31f60` writes `u32 count` and two more `u32`s before the per-target loop.
///
/// The **first** of the three is the loop bound — `140f31f97 cmp [rdi],ebx / jle` — so it
/// is the count and the other two are not. **[L]**
pub const TARGET_LIST_HEAD_LEN: usize = 12;

/// The fixed, undecoded run inside one target block, between the damage list and the
/// `std::map` count: 4 × `u8`, 6 × `u16`, 2 × `u32`, 3 × `u8`, 1 × `u32`, 4 × `u8`,
/// 4 × `u16`, 3 × `u32` = **55**. `research/mob-combat.md` §1.6. **[L]**
pub const TARGET_MIDDLE_LEN: usize = 55;

/// The trailer's unconditional part: `FUN_14083b270` opens with a `u32` and a `u8` before
/// its first branch. See [`AttackTrailer`] — the `u8` is a **flag**, and more can follow.
pub const TRAILER_FIXED_LEN: usize = 5;

/// The length of the 2026-08-19 capture: `"User Melee"`, zero targets, trailer flag clear.
///
/// `98 + 2 + 10 + 12 + 5` = **127**, which is exactly what `world.log:353` recorded.
/// **[D]**
pub const EMPTY_MELEE_LEN: usize = HEADER_FIXED_LEN + 2 + 10 + TARGET_LIST_HEAD_LEN + TRAILER_FIXED_LEN;

/// Header field 9's value in **433 of the 434 distinct captured bodies**.
///
/// `research/attack-skill-id.md` §3 saw it in three captures nine days and three characters
/// apart and called it a constant, refuting an earlier `[I]` that it was a per-attack nonce.
/// Over the full corpus that is very nearly right and not quite: the one exception is a
/// `0x00DF` at `02:25:52.054` in `previous-runs/world-20260819-222734.log`. That swing
/// carries `0xDD01C6A9`, and it is also the only body in the corpus with header field 8 = 25
/// and field 34 = 262 — three fields moving together, so whatever it is, it is not noise.
///
/// So: **near-constant, one outlier, meaning still unknown.** Not a checksum over the
/// packet — it does not move when the tick, the position or the damage move. **[D]**
pub const FIELD_9_USUAL: u32 = 0x0834_AE9F;

/// The trailer's leading `u32` in **all 434 distinct captured bodies**, from 25 captures
/// over nine days.
///
/// It is `FUN_1402b29a0()`'s return value, and that function is *not* a constant returner —
/// it calls `0x1429e3ef0`, tests a global, and writes two globals. It has simply returned
/// the same number every time anyone captured it. **[L]** on the value, nothing on the
/// meaning.
pub const TRAILER_USUAL_VALUE: u32 = 0x8FDA_E880;

// -------------------------------------------------------------------------------------
// The parsed packet
// -------------------------------------------------------------------------------------

/// The 40 fields of `FUN_140f31fe0`, in wire order.
///
/// Seven have a name that two independent things agree on. The other 33 are carried as
/// `f<n>`, numbered exactly as `research/mob-combat.md` §1.3's table numbers them, with the
/// source-struct offset in the doc comment. **A field is not named here until something
/// other than its shape says what it is.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackHeader {
    /// Field 0, body offset 0, struct `+0x00`. `0` in all 434 distinct captures.
    pub f0: u8,

    /// Field 1, body offset 1, struct `+0x04`.
    ///
    /// **Equal to the target count in all 434 distinct captures** — `1` in the 391 bodies with a
    /// target, `0` in the 43 without. Nothing has ever carried more than one target, so
    /// "count" and "boolean" are indistinguishable here. **[D]** on the correlation,
    /// nothing on the name.
    pub f1: u8,

    /// Field 2, body offset 2, struct `+0x08`: **the skill id**, `0` for a plain swing.
    ///
    /// `2001003` = Magic Claw in 7 bodies, `1000` = Three Snails in **1**, `0` in 426.
    /// Corroborated by [`Self::skill_level`], which independently matches a number known
    /// from a different source in both cases — see `research/attack-skill-id.md` §1.
    /// **[L]**
    pub skill_id: u32,

    /// Field 3, body offset 6, struct `+0x0c`: **the skill level**.
    ///
    /// `7` on all 7 Magic Claws, and the owner had put exactly 7 points into Magic Claw. `3` on
    /// the Three Snails, and Three Snails' `maxLevel` in this client's `Skill.wz` is
    /// exactly 3. Two skills, two levels, each matching a number known from a completely
    /// unrelated source. **[L]**
    ///
    /// The corroboration is the *match*, not the sample size — but be honest about the
    /// sample size: the Magic Claws are 7 casts in **one session**, and the Three Snails is
    /// **one packet**. See the module docs on `research/fixtures/` duplication.
    pub skill_level: u8,

    /// Field 4, body offset 7. A `setne dl`, so a boolean. `false` in all 434.
    pub f4: bool,

    /// Field 5, body offset 8, struct `+0x14`.
    ///
    /// Zero on every melee swing, and **one fixed non-zero value per skill**:
    /// `0xEB6218B0` on all 7 Magic Claws, `0x3CAAAD2C` on the one Three Snails. Together
    /// with [`Self::f6`] that is 64 bits that depend only on which skill was cast. **[D]**
    /// on the correlation; a per-skill key or hash is **[I]** and nothing here reads it.
    pub f5: u32,

    /// Field 6, body offset 12, struct `+0x18`. See [`Self::f5`] — same behaviour:
    /// `0xB73AA1C1` on every Magic Claw, `0xEBF28769` on every Three Snails, `0` on melee.
    pub f6: u32,

    /// Field 7, body offset 16. A `setne dl`. Varies: `false` in 284, `true` in 150.
    pub f7: bool,

    /// Field 8, body offset 17, struct `+0x20`. Six values across the corpus:
    /// 5, 6, 7, 16, 17, 25.
    pub f8: u32,

    /// Field 9, body offset 21, struct `+0x24`. See [`FIELD_9_USUAL`] — the same value in
    /// 433 of 434 distinct bodies, with one outlier.
    pub f9: u32,

    /// Field 10, body offset 25, struct `+0x28`. `1` in 425, `8` in 9.
    pub f10: u8,

    /// Field 11, body offset 26, struct `+0x2c`. `4` in 417, `6` in 17.
    pub f11: u32,

    /// Field 12, body offset 30, struct `+0x30`: **the tick**, in milliseconds.
    ///
    /// The 2026-08-19 capture's `0x06EEAC37` is 870 ticks after the `0x00D9` movement
    /// packet 852 ms earlier — agreement to 18 ms. **[L]**
    ///
    /// It is also the one header field where every single body in the corpus carries a
    /// different value: 434 distinct out of 434.
    pub tick: u32,

    /// Field 13, body offset 34, struct `+0x34`: **x**.
    ///
    /// Written by the client's `u16` writer and read back here as **signed**: the corpus
    /// contains `0xFFD2` and `0xFF33`, which are −46 and −205 as ordinary map coordinates
    /// and absurd as unsigned. Every other coordinate in this crate is `i16` for the same
    /// reason. **[D]**
    ///
    /// The 2026-08-19 capture's `473, 395` is the exact position the next `0x00D9` reports
    /// the player walking away from. **[L]**
    pub x: i16,

    /// Field 14, body offset 36, struct `+0x38`: **y**. See [`Self::x`].
    pub y: i16,

    /// Field 15, body offset 38, struct `+0x3c`. `0` in 387; otherwise small: 6, 7, 23,
    /// 24, 25, 38.
    pub f15: u32,

    /// Field 16, body offset 42, struct `+0x40`. **Equal to [`Self::x`] in all 434
    /// captures.** Signed for the same reason. **[D]**
    pub f16: i16,

    /// Field 17, body offset 44, struct `+0x44`.
    ///
    /// Equal to [`Self::y`] in 427 captures and **different in 7** — every Magic Claw,
    /// and only those. So fields 13/14 and 16/17 are two *separate* positions that happen
    /// to coincide for a melee swing, not one position written twice. **[D]** That the
    /// second is the spell's own origin is **[I]**.
    pub f17: i16,

    /// Field 18, body offset 46. A `setne dl`. `false` in 425, `true` in 9.
    pub f18: bool,
    /// Field 19, body offset 47, struct `+0x48`. `0` in all 434.
    pub f19: u8,
    /// Field 20, body offset 48. A `setne dl`. `false` in all 434.
    pub f20: bool,
    /// Field 21, body offset 49. A `setne dl`. `false` in all 434.
    pub f21: bool,
    /// Field 22, body offset 50, struct `+0x50`. `0` in all 434.
    pub f22: u32,
    /// Field 23, body offset 54, struct `+0x54`. `0` in all 434.
    pub f23: u32,
    /// Field 24, body offset 58, struct `+0x58`. `0` in all 434.
    pub f24: u16,
    /// Field 25, body offset 60, struct `+0x60`. `0` in all 434.
    pub f25: u32,
    /// Field 26, body offset 64, struct `+0x64`. `0` in all 434.
    pub f26: u16,
    /// Field 27, body offset 66, struct `+0x68`. `0` in all 434.
    pub f27: u16,
    /// Field 28, body offset 68, struct `+0x6c`. `0` in all 434.
    pub f28: u16,
    /// Field 29, body offset 70, struct `+0x70`. `0` in all 434.
    pub f29: u16,
    /// Field 30, body offset 72, struct `+0x74`. `1` in 431, `0` in 3.
    pub f30: u32,
    /// Field 31, body offset 76. A `setne dl`. `true` in 433, `false` in 1.
    pub f31: bool,
    /// Field 32, body offset 77, struct `+0x7c`. `0` in all 434.
    pub f32: u32,

    /// Field 33, body offset 81, struct `+0x80`: **the attack type, as a string**.
    ///
    /// The client names its own attack type on the wire. It picks the string out of a
    /// 60-entry table of 16-byte slots at `0x1432b3f10` — `User Normal, User Melee,
    /// User Shoot, User Magic, User Body, …, User Melee Skill System, User Magic Skill
    /// System, …`. Three of them appear in the corpus: `"User Melee"` (426),
    /// `"User Magic Skill System"` (7) and `"User Magic"` (1). **[L]**
    ///
    /// Note that Three Snails, a skill, is sent as plain `"User Magic"` while Magic Claw
    /// is `"User Magic Skill System"`, so the string is **not** a reliable "is this a
    /// skill" test. [`Self::skill_id`] is.
    pub attack_type: String,

    /// Field 34, body offset 93, struct `+0x88`. `393` in 270, `305` in 146, `350` in 9,
    /// `576` in 1, `262` in 1, and **`0` in every one of the 7 Magic Claws**.
    pub f34: u32,
    /// Field 35, body offset 97, struct `+0x90`. `0` in all 434.
    pub f35: u32,
    /// Field 36, body offset 101, struct `+0x94`. `0` in all 434.
    pub f36: u32,
    /// Field 37, body offset 105, struct `+0x98`. `0` in all 434.
    pub f37: u16,
    /// Field 38, body offset 107, struct `+0x9c`. `0` in all 434.
    pub f38: u16,
    /// Field 39, body offset 109, struct `+0xa4`. `0` in all 434.
    ///
    /// **This is the tail `jmp` at `0x140f321eb`.** It is a field. See [`FIELD_WIDTHS`].
    pub f39: u8,
}

impl AttackHeader {
    /// Bytes this header occupies on the wire, recomputed from its own contents.
    ///
    /// Deliberately **not** the parser's cursor arithmetic — this is the independent half
    /// of the byte-accounting check. `PacketReader::str` maps each wire byte to one `char`,
    /// so the wire length of the string is its `chars().count()`, not its UTF-8 `len()`.
    pub fn encoded_len(&self) -> usize {
        HEADER_FIXED_LEN + 2 + self.attack_type.chars().count()
    }

    /// Did the client cast a skill, rather than swing?
    ///
    /// The skill id, not the attack-type string — see [`Self::attack_type`] for why the
    /// string cannot answer this.
    pub fn is_skill(&self) -> bool {
        self.skill_id != 0
    }
}

/// One hit on one target — `flag_a`, `flag_b`, `u64 damage`, stride `0x10` in the source
/// struct and **10 bytes on the wire**. `FUN_140f31bb0`. **[L]**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackHit {
    /// `[t+0x28+i*0x10] != 0`. **`false` on all 361 hits** counted in
    /// `research/damage-formula.md` §3.2 and on every hit in the wider corpus.
    /// Unexplained.
    pub flag_a: bool,

    /// `[t+0x29+i*0x10] != 0`: **the critical flag**.
    ///
    /// Measured, not inferred, and from **one** capture rather than pooled across sessions
    /// — `previous-runs/world-20260820-181822.log`, one character, 69 hits:
    ///
    /// | | |
    /// |---|---|
    /// | `flag_b == 0` | 64 hits, damages 15–20 |
    /// | `flag_b == 1` | 5 hits, damages 20, 21, 22, 24 |
    ///
    /// Every flagged hit is at or above the maximum of the 64 unflagged ones and one is
    /// above it, so the flag is not noise; 5/69 is consistent with a 5% base rate.
    /// **[D]**, `research/damage-formula.md` §3.2.
    ///
    /// The **multiplier** is not settled. `×1.2` is consistent with the four observed crit
    /// values and is not measured.
    pub flag_b: bool,

    /// `[t+0x30+i*0x10]`, a full `u64`. **The client computed this.** Nothing the server
    /// sends supplies it, and a client that has already worked out a number is not waiting
    /// to be told what it is.
    pub damage: u64,
}

/// The `mode` tail of a target block — `[t+0x1b0]`, a **three-way**, not a flag.
///
/// `mov ecx,[rsi+0x1b0] / sub ecx,1 / je <one> / cmp ecx,1 / jne <done>`, so 1 and 2 take
/// different branches and 0 takes neither. **[L]** Distinguishing a real 3-way from a
/// boolean is the difference between a parser that works and one that desynchronises.
///
/// **`mode` is 0 in all 434 distinct captured bodies**, so both arms below are `[L]` from the
/// listing and have never been seen on the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackTargetMode {
    /// `mode == 1`: `str`, `u32`, `u8`, `u8 hasSub`, and if set `FUN_141fde400` -> `u32`,
    /// `str`.
    One {
        /// The leading `str`.
        text: String,
        /// The `u32` after it.
        value: u32,
        /// The `u8` after that.
        flag: u8,
        /// `FUN_141fde400`'s pair, present only when the fourth byte was non-zero.
        sub: Option<(u32, String)>,
    },
    /// `mode == 2`: `str`, `u32`, `u8`.
    Two {
        /// The leading `str`.
        text: String,
        /// The `u32` after it.
        value: u32,
        /// The trailing `u8`.
        flag: u8,
    },
}

impl AttackTargetMode {
    fn encoded_len(&self) -> usize {
        match self {
            AttackTargetMode::One {
                text, sub, ..
            } => {
                2 + text.chars().count()
                    + 4
                    + 1
                    + 1
                    + sub
                        .as_ref()
                        .map_or(0, |(_, s)| 4 + 2 + s.chars().count())
            }
            AttackTargetMode::Two { text, .. } => 2 + text.chars().count() + 4 + 1,
        }
    }
}

/// One target of an attack — `FUN_140f31bb0`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackTarget {
    /// `[t+0x10]`: **the mob's object id**.
    ///
    /// This started life as `[I]` — "the first of two `u32`s, which is the reference
    /// server's order" — because the only capture then available had no targets. It is now
    /// **[D]**: across every capture in the repo the first `u32` takes the values 2000,
    /// 2001, 2002, 2003, 2004, 2006, 2007, 2009, 2013, 2016, 2022, 2032, 2033 and 2034 —
    /// every one of them an id this server itself assigned on map 40, whose range is
    /// 2000–2039. See `research/damage-formula.md` §3.1.
    pub object_id: u32,

    /// `[t+0x14]`. Unexplained. `2` or `0` on melee, `8` or `10` on Magic Claw, `5` on
    /// Three Snails — never a plausible object id, which is the other half of why
    /// [`Self::object_id`] is the first `u32` and not this one.
    pub t14: u32,

    /// `[t+0x20]`, the damage count, **written `u8` while the loop bound is the dword
    /// there**. `1` on every melee and Three Snails hit, `2` on every Magic Claw. **[L]**
    pub hit_count: u8,

    /// `hit_count` hits. See [`AttackHit`].
    pub hits: Vec<AttackHit>,

    /// The 55 fixed bytes between the damage list and the pair map — `[t+0x118]` through
    /// `[t+0x188]`. Carried raw because none of it is understood and all of it is
    /// fixed-width. See [`TARGET_MIDDLE_LEN`].
    pub middle: [u8; TARGET_MIDDLE_LEN],

    /// `[t+0x198]`, the `u16` immediately before the pair walk.
    ///
    /// The walk is a red-black-tree in-order traversal with no count written anywhere:
    /// `_Myhead` at `+0x190` and `_Mysize` at `+0x198` is the MSVC `std::map` layout, so
    /// the `u16` written one instruction earlier **is** the element count. **[D]** Getting
    /// this wrong is the difference between a parser that works and one that
    /// desynchronises — and it is checked hard: the corpus contains bodies of 213, 221 and
    /// 229 bytes that are otherwise identical in shape and differ by exactly 8 bytes per
    /// pair.
    pub pair_count: u16,

    /// `pair_count` × (`u32 key`, `u32 value`). The keys observed are 0, 1, 2 and 7.
    pub pairs: Vec<(u32, u32)>,

    /// `[t+0x1a0] != 0`. **`false` in all 434 distinct captures.**
    ///
    /// When set, `FUN_14025d3c0` writes `u32`, `u32`, `raw(n)` — and `n` is **not
    /// resolved**, so a target that sets this cannot be stepped over to reach the next one.
    /// [`parse`] stops there and reports [`AttackTruncation::TargetSubObject`].
    pub has_object: bool,

    /// `[t+0x1b0]`. **`0` in all 434 distinct captures.** See [`AttackTargetMode`] — a three-way.
    ///
    /// `None` when [`Self::has_object`] stopped the walk before this was read.
    pub mode: Option<u8>,

    /// The payload of [`Self::mode`] 1 or 2. `None` for mode 0 and for a stopped walk.
    pub mode_payload: Option<AttackTargetMode>,
}

impl AttackTarget {
    /// Total damage across every hit, saturating.
    pub fn total_damage(&self) -> u64 {
        self.hits
            .iter()
            .fold(0u64, |acc, h| acc.saturating_add(h.damage))
    }

    /// Did any hit set the critical flag? See [`AttackHit::flag_b`].
    pub fn any_critical(&self) -> bool {
        self.hits.iter().any(|h| h.flag_b)
    }

    /// Bytes this block occupies on the wire, recomputed from its own contents.
    ///
    /// `None` when [`Self::has_object`] is set, because the sub-object's `raw(n)` length is
    /// unresolved and therefore the block's length genuinely is not known.
    pub fn encoded_len(&self) -> Option<usize> {
        if self.has_object {
            return None;
        }
        let mode_len = match self.mode {
            None => 0,
            Some(_) => 1 + self.mode_payload.as_ref().map_or(0, |m| m.encoded_len()),
        };
        Some(
            4 + 4
                + 1
                + self.hits.len() * 10
                + TARGET_MIDDLE_LEN
                + 2
                + self.pairs.len() * 8
                + 1
                + mode_len,
        )
    }
}

/// The packet's trailer — `FUN_14083b270(packet, obj)`.
///
/// # `research/mob-combat.md` §1.4 is wrong about this, and the corpus is what caught it
///
/// §1.4 says the trailer "writes a final `u32` + `u8`". That describes the first two writes
/// and stops. `FUN_14083b270` is **2612 bytes**, `.pdata`-merged over five entries
/// (`0x14083b270..0x14083bca4`), and its shape is:
///
/// ```asm
/// 14083b27e  call 0x1402b29a0            ; -> eax
/// 14083b288  call w_u32                  ; u32  value        <- always
/// 14083b293  setne dl                    ; (obj != NULL)
/// 14083b296  call w_u8                   ; u8   has_object   <- always
/// 14083b29e  je   0x14083bbf4            ; NULL -> return, nothing else written
/// 14083b2b0  call w_u32                  ; u32  [obj+0x1c]
/// 14083b2bf  je   0x14083bbe2            ; empty list -> straight to the terminator
///   loop:
/// 14083b2f8  call w_u32                  ; u32  [node+0x1c]  <- a type tag
/// 14083b302  cmp  ebx,0x29 / ja          ; 42-way switch on tag-1, one arm per type
/// 14083bbe2  mov  edx,0xffffffff / w_u32 ; u32  0xFFFFFFFF   <- the terminator
/// ```
///
/// So the `u8` is a **flag**, and when it is set the packet does not end after 5 bytes.
///
/// **7 of the 434 captured bodies set it** — every Magic Claw and only those. Each carries
/// exactly 8 more bytes: `u32 0x00000000` then the `u32 0xFFFFFFFF` terminator, i.e. an
/// empty node list. A parser that stops at 5 bytes leaves those 8 unread, which is exactly
/// how this was found. **[L]** for the listing, **[D]** for the match to the wire.
///
/// The 42 node types are **not** decoded and nothing in the corpus exercises them, so a
/// non-empty list is carried raw in [`Self::extra`] rather than guessed at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackTrailer {
    /// `FUN_1402b29a0()`'s return value. [`TRAILER_USUAL_VALUE`] in all 434 distinct captures.
    pub value: u32,

    /// `(obj != NULL)`. `true` on every Magic Claw, `false` on everything else.
    pub has_object: bool,

    /// Everything after the flag, raw. Empty when [`Self::has_object`] is clear; the 8
    /// bytes `00 00 00 00 FF FF FF FF` on every capture that sets it.
    ///
    /// Raw and not decoded because the node list's 42-way type switch is not decoded, and
    /// because this is the **end of the body** — reading it as bytes cannot desynchronise
    /// anything downstream.
    pub extra: Vec<u8>,
}

impl AttackTrailer {
    /// Bytes this trailer occupies on the wire, recomputed from its own contents.
    pub fn encoded_len(&self) -> usize {
        TRAILER_FIXED_LEN + self.extra.len()
    }

    /// Does [`Self::extra`] end in the `0xFFFFFFFF` the listing writes at `0x14083bbe2`?
    ///
    /// `true` on all 21 captures that set [`Self::has_object`]. A `false` here on a body
    /// that set the flag would mean the node list is non-empty and the raw bytes contain
    /// something nobody has decoded — worth logging, never worth refusing over.
    pub fn ends_with_terminator(&self) -> bool {
        self.extra.len() >= 4 && self.extra[self.extra.len() - 4..] == [0xFF, 0xFF, 0xFF, 0xFF]
    }
}

/// Why the target walk stopped before it had read `target_count` targets.
///
/// **None of these is a reason not to reply.** Everything already parsed is good.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackTruncation {
    /// A target set `[t+0x1a0]`, whose sub-object carries a `raw(n)` of unresolved length,
    /// so the next target's offset is unknown. See [`AttackTarget::has_object`]. Never
    /// observed in the 434-body corpus.
    TargetSubObject {
        /// Index of the target that stopped the walk.
        index: u32,
    },
    /// The body ran out inside a target block.
    ShortTarget {
        /// Index of the target being read.
        index: u32,
        /// Byte offset the reader had reached.
        offset: usize,
    },
    /// The body ran out before the trailer.
    ShortTrailer {
        /// Byte offset the reader had reached.
        offset: usize,
    },
}

/// A parsed attack packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attack {
    /// Which of `0x00DF` / `0x00E0` / `0x00E1` this was. All three share the body; the
    /// opcode is the only thing that distinguishes melee from shoot from magic, because
    /// [`AttackHeader::attack_type`] does not reliably.
    pub opcode: u16,

    /// All 40 header fields.
    pub header: AttackHeader,

    /// `FUN_140f31f60`'s first `u32`, which is the loop bound. `1` in 391 captures, `0` in
    /// 43, never anything else.
    pub target_count: u32,

    /// `FUN_140f31f60`'s second `u32`. `0` in 433 captures, `1` in the single Three Snails.
    pub list_head_b: u32,

    /// `FUN_140f31f60`'s third `u32`. `0` in all 434.
    pub list_head_c: u32,

    /// The targets the **client** decided it hit. An empty list is a real, common answer.
    pub targets: Vec<AttackTarget>,

    /// Set when the walk stopped early. `targets` holds everything read before that.
    pub truncated: Option<AttackTruncation>,

    /// The trailer, when the parse reached it. `None` only alongside a
    /// [`Self::truncated`].
    pub trailer: Option<AttackTrailer>,

    /// Bytes the parser consumed. Compare against the body length: they are equal for all
    /// 434 distinct captures, and a parser that stops early is the single most likely bug here.
    pub consumed: usize,
}

impl Attack {
    /// Bytes this packet occupies on the wire, **recomputed from the parsed values** rather
    /// than read off the parser's cursor.
    ///
    /// This is the independent half of the byte-accounting check: [`Self::consumed`] is
    /// what the reader did, and this is what the parsed contents say it should have done.
    /// Asserting `consumed == recomputed_len() == body.len()` catches a parser that stops
    /// early *and* a parser that stops early for a reason its own struct agrees with.
    ///
    /// `None` when a target could not be measured (see [`AttackTarget::encoded_len`]) or
    /// the parse was truncated.
    pub fn recomputed_len(&self) -> Option<usize> {
        if self.truncated.is_some() {
            return None;
        }
        let mut total = self.header.encoded_len() + TARGET_LIST_HEAD_LEN;
        for t in &self.targets {
            total += t.encoded_len()?;
        }
        total += self.trailer.as_ref()?.encoded_len();
        Some(total)
    }

    /// Total damage the client claims across every target and hit, saturating.
    pub fn total_damage(&self) -> u64 {
        self.targets
            .iter()
            .fold(0u64, |acc, t| acc.saturating_add(t.total_damage()))
    }

    /// Did any hit on any target set the critical flag? See [`AttackHit::flag_b`].
    pub fn any_critical_target(&self) -> bool {
        self.targets.iter().any(|t| t.any_critical())
    }

    /// The skill this attack used, or `None` for a plain swing.
    pub fn skill(&self) -> Option<(u32, u8)> {
        if self.header.skill_id == 0 {
            None
        } else {
            Some((self.header.skill_id, self.header.skill_level))
        }
    }
}

// -------------------------------------------------------------------------------------
// Errors
// -------------------------------------------------------------------------------------

/// Why [`parse`] could not turn a body into an [`Attack`].
///
/// **An `Err` from this module is a logging event, not a policy.** `CLAUDE.md`'s
/// always-answer rule: an unanswered packet freezes the client's whole UI, including the
/// quit prompt, and reads on screen as a crash. There is no error in here that a caller
/// should turn into "drop the packet".
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AttackError {
    /// The opcode is not one of the three that share this body.
    ///
    /// `0x00E2` lands here on purpose: `research/mob-combat.md` §1.2 shows it uses the
    /// classic inline shape and never calls `FUN_140f31fe0`, so parsing it with this
    /// layout would produce confident nonsense.
    #[error("0x{opcode:04X} is not an attack opcode this layout covers")]
    NotAnAttack {
        /// The opcode that was handed in.
        opcode: u16,
    },

    /// The body ended inside the 40-field header.
    #[error(
        "attack header ran out at field {field} (body offset {offset}, {have} bytes left, \
         body is {body_len})"
    )]
    ShortHeader {
        /// Index into [`FIELD_WIDTHS`] of the field being read.
        field: usize,
        /// Byte offset the reader had reached.
        offset: usize,
        /// Bytes still available there.
        have: usize,
        /// Total body length.
        body_len: usize,
    },

    /// The body ended inside `FUN_140f31f60`'s three-`u32` head.
    #[error("attack target list head ran out at body offset {offset} (body is {body_len})")]
    ShortTargetListHead {
        /// Byte offset the reader had reached.
        offset: usize,
        /// Total body length.
        body_len: usize,
    },
}

// -------------------------------------------------------------------------------------
// The parser
// -------------------------------------------------------------------------------------

/// Parse an attack body. The opcode has already been split off the front.
///
/// # This never panics and never rejects
///
/// * Every read is bounds-checked; there is no slicing, no indexing and no arithmetic that
///   can overflow on a hostile body. `parsing_any_prefix_of_every_fixture_never_panics` and
///   `parsing_pseudo_random_bodies_never_panics` are the checks.
/// * A body that is too short to hold a header comes back as [`AttackError::ShortHeader`],
///   which is *information for the log*. **The caller still answers.**
/// * A body whose *targets* run out, or whose target sets the unresolved sub-object flag,
///   is **not** an error at all: it comes back `Ok` with [`Attack::truncated`] set and every
///   target read so far intact. Losing the fourth mob of a five-mob swing is worth far less
///   than losing the reply.
///
/// # Byte accounting
///
/// On a clean parse, `attack.consumed == body.len()` and
/// `attack.recomputed_len() == Some(body.len())`. Both hold for all 434 captured bodies.
pub fn parse(opcode: u16, body: &[u8]) -> Result<Attack, AttackError> {
    if !is_attack_opcode(opcode) {
        return Err(AttackError::NotAnAttack { opcode });
    }

    let mut r = PacketReader::new(body);
    let header = parse_header(&mut r, body.len())?;

    // -- target list head: FUN_140f31f60, three u32s, the first is the loop bound -------
    let (target_count, list_head_b, list_head_c) = match (r.u32(), r.u32(), r.u32()) {
        (Ok(a), Ok(b), Ok(c)) => (a, b, c),
        _ => {
            return Err(AttackError::ShortTargetListHead {
                offset: r.position(),
                body_len: body.len(),
            })
        }
    };

    let mut targets = Vec::new();
    let mut truncated = None;
    for index in 0..target_count {
        match parse_target(&mut r) {
            Ok(target) => {
                let stop = target.has_object;
                targets.push(target);
                if stop {
                    truncated = Some(AttackTruncation::TargetSubObject { index });
                    break;
                }
            }
            Err(offset) => {
                truncated = Some(AttackTruncation::ShortTarget { index, offset });
                break;
            }
        }
    }

    let trailer = if truncated.is_some() {
        None
    } else {
        match parse_trailer(&mut r) {
            Some(t) => Some(t),
            None => {
                truncated = Some(AttackTruncation::ShortTrailer {
                    offset: r.position(),
                });
                None
            }
        }
    };

    Ok(Attack {
        opcode,
        header,
        target_count,
        list_head_b,
        list_head_c,
        targets,
        truncated,
        trailer,
        consumed: r.position(),
    })
}

/// A convenience over [`parse`] for a body that still has its opcode on the front.
///
/// Returns [`AttackError::NotAnAttack`] with opcode `0` for a buffer too short to hold one,
/// which is the only sane thing to say about two missing bytes.
pub fn parse_with_opcode(packet: &[u8]) -> Result<Attack, AttackError> {
    if packet.len() < 2 {
        return Err(AttackError::NotAnAttack { opcode: 0 });
    }
    let opcode = u16::from_le_bytes([packet[0], packet[1]]);
    parse(opcode, &packet[2..])
}

/// Which of the three attack opcodes, spelled out for a log line.
pub fn attack_opcode_name(opcode: u16) -> Option<&'static str> {
    match opcode {
        USER_MELEE_ATTACK => Some("melee"),
        USER_SHOOT_ATTACK => Some("shoot"),
        USER_MAGIC_ATTACK => Some("magic"),
        _ => None,
    }
}

/// `FUN_140f31fe0`, 40 fields, straight-line, no branches.
///
/// Each read reports *which* field ran out, because "the header is short" and "the header
/// is short at field 33, so the string length was garbage" are different bugs.
fn parse_header(r: &mut PacketReader, body_len: usize) -> Result<AttackHeader, AttackError> {
    macro_rules! rd {
        ($field:expr, $m:ident) => {{
            let offset = r.position();
            match r.$m() {
                Ok(v) => v,
                Err(_) => {
                    return Err(AttackError::ShortHeader {
                        field: $field,
                        offset,
                        have: body_len.saturating_sub(offset),
                        body_len,
                    })
                }
            }
        }};
    }

    Ok(AttackHeader {
        f0: rd!(0, u8),
        f1: rd!(1, u8),
        skill_id: rd!(2, u32),
        skill_level: rd!(3, u8),
        f4: rd!(4, bool),
        f5: rd!(5, u32),
        f6: rd!(6, u32),
        f7: rd!(7, bool),
        f8: rd!(8, u32),
        f9: rd!(9, u32),
        f10: rd!(10, u8),
        f11: rd!(11, u32),
        tick: rd!(12, u32),
        x: rd!(13, i16),
        y: rd!(14, i16),
        f15: rd!(15, u32),
        f16: rd!(16, i16),
        f17: rd!(17, i16),
        f18: rd!(18, bool),
        f19: rd!(19, u8),
        f20: rd!(20, bool),
        f21: rd!(21, bool),
        f22: rd!(22, u32),
        f23: rd!(23, u32),
        f24: rd!(24, u16),
        f25: rd!(25, u32),
        f26: rd!(26, u16),
        f27: rd!(27, u16),
        f28: rd!(28, u16),
        f29: rd!(29, u16),
        f30: rd!(30, u32),
        f31: rd!(31, bool),
        f32: rd!(32, u32),
        attack_type: rd!(33, str),
        f34: rd!(34, u32),
        f35: rd!(35, u32),
        f36: rd!(36, u32),
        f37: rd!(37, u16),
        f38: rd!(38, u16),
        f39: rd!(39, u8),
    })
}

/// One target block, `FUN_140f31bb0`.
///
/// `Err(offset)` is an underrun at that byte offset; the caller turns it into an
/// [`AttackTruncation`] rather than an error, because the reply matters more.
fn parse_target(r: &mut PacketReader) -> Result<AttackTarget, usize> {
    macro_rules! rd {
        ($m:ident) => {
            match r.$m() {
                Ok(v) => v,
                Err(_) => return Err(r.position()),
            }
        };
    }

    let object_id = rd!(u32); // [t+0x10]
    let t14 = rd!(u32); //      [t+0x14]
    let hit_count = rd!(u8); // [t+0x20], written u8; the loop bound is the dword there
    let mut hits = Vec::with_capacity(hit_count as usize);
    for _ in 0..hit_count {
        hits.push(AttackHit {
            flag_a: rd!(bool),
            flag_b: rd!(bool),
            damage: rd!(u64),
        });
    }

    let mut middle = [0u8; TARGET_MIDDLE_LEN];
    match r.bytes(TARGET_MIDDLE_LEN) {
        Ok(b) => middle.copy_from_slice(b),
        Err(_) => return Err(r.position()),
    }

    // `_Myhead` at [t+0x190], `_Mysize` at [t+0x198] is the MSVC std::map layout, so the
    // u16 written one instruction before the in-order walk IS the walk's length.
    let pair_count = rd!(u16);
    let mut pairs = Vec::with_capacity(pair_count as usize);
    for _ in 0..pair_count {
        let k = rd!(u32);
        let v = rd!(u32);
        pairs.push((k, v));
    }

    let has_object = rd!(bool);
    if has_object {
        // FUN_14025d3c0 -> u32, u32, raw(n), and `n` is not resolved. Stop here rather
        // than desynchronise: everything above is still good.
        return Ok(AttackTarget {
            object_id,
            t14,
            hit_count,
            hits,
            middle,
            pair_count,
            pairs,
            has_object,
            mode: None,
            mode_payload: None,
        });
    }

    // [t+0x1b0] is a three-way, NOT a flag.
    let mode = rd!(u8);
    let mode_payload = match mode {
        1 => {
            let text = rd!(str);
            let value = rd!(u32);
            let flag = rd!(u8);
            let sub = if rd!(bool) {
                let a = rd!(u32);
                let b = rd!(str);
                Some((a, b))
            } else {
                None
            };
            Some(AttackTargetMode::One {
                text,
                value,
                flag,
                sub,
            })
        }
        2 => {
            let text = rd!(str);
            let value = rd!(u32);
            let flag = rd!(u8);
            Some(AttackTargetMode::Two { text, value, flag })
        }
        _ => None,
    };

    Ok(AttackTarget {
        object_id,
        t14,
        hit_count,
        hits,
        middle,
        pair_count,
        pairs,
        has_object,
        mode: Some(mode),
        mode_payload,
    })
}

/// `FUN_14083b270`. See [`AttackTrailer`] for why the `u8` is a flag and not the end.
fn parse_trailer(r: &mut PacketReader) -> Option<AttackTrailer> {
    let value = r.u32().ok()?;
    let has_object = r.bool().ok()?;
    // Everything after the flag belongs to the optional block, and it is the end of the
    // body, so taking it raw cannot desynchronise anything. The node list's 42-way type
    // switch is not decoded and is deliberately not guessed at.
    let extra = if has_object {
        let n = r.remaining();
        r.bytes(n).map(<[u8]>::to_vec).unwrap_or_default()
    } else {
        Vec::new()
    };
    Some(AttackTrailer {
        value,
        has_object,
        extra,
    })
}

/// Environment knobs for the test sweep.
pub mod env {
    /// Point this at a file written by `tools/extract_attack_bodies.py` and
    /// `cargo test -p net attack` re-runs every assertion in this module over the whole
    /// corpus instead of over the eight embedded fixtures.
    ///
    /// Pass `--dedupe` unless you specifically want the duplicated count: 36 of the
    /// repository's world logs are byte-identical copies of another one.
    ///
    /// ```text
    /// cd C:\MapleCW
    /// python tools\extract_attack_bodies.py --dedupe --out corpus.txt
    /// set MAPLECW_ATTACK_CORPUS=C:\MapleCW\corpus.txt
    /// cargo test -p net attack
    /// ```
    pub const CORPUS: &str = "MAPLECW_ATTACK_CORPUS";
}

// -------------------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Nine real bodies, chosen to cover every distinct shape in the 434-body corpus:
    // both skills, both magic string variants, every pair count 1..4, the zero-target
    // case, the trailer's optional block, a critical hit and the one field-9 outlier.
    //
    // Extracted with `python tools/extract_attack_bodies.py`. The provenance line on each
    // is the log, the timestamp and the length, so any one of them can be found again.

    /// `world.log` 04:01:06.251, `0x00E1`, 260 bytes. Magic Claw, level 7, mob 2006, two
    /// hits of 1, three map pairs, trailer object present.
    const MAGIC_CLAW_260: &str = "00016b881e000700b01862ebc1a13ab701070000009fae340801060000007a88791a05fdd7000000000005fdbb000000000000000000000000000000000000000000000000000000010000000100000000170055736572204d6167696320536b696c6c2053797374656d0000000000000000000000000000000000010000000000000000000000d60700000a000000020000010000000000000000000100000000000000010002077dfcd70081fcd7007dfcbb00c2010000000000000200000a000000000001015cfca200a2fcd300000000004d5a227e00000000030000000000c4a264860100000035ec090102000000011ef164000080e8da8f0100000000ffffffff";

    /// `world.log` 04:01:12.912, `0x00E1`, 268 bytes. The same cast with **four** map
    /// pairs instead of three — the eight-byte step that pins the pair count.
    const MAGIC_CLAW_268: &str = "00016b881e000700b01862ebc1a13ab700050000009fae340801060000007ea2791a99fdd7000000000099fdbb000000000000000000000000000000000000000000000000000000010000000100000000170055736572204d6167696320536b696c6c2053797374656d0000000000000000000000000000000000010000000000000000000000d107000008000000020000010000000000000000000100000000000000010700079dfed7009dfed7009dfebb00c20100000000000002000008000000000001017ffea100bbfed7000000000047aab6fd00000000040000000000acec525601000000936e0ea8020000006ff5507607000000d9dd5173000080e8da8f0100000000ffffffff";

    /// `previous-runs/world-20260822-213856.log` 01:38:21.847, `0x00E1`, 229 bytes.
    /// Three Snails, level 3, mob 2007, one hit of 40. Note the attack-type string is the
    /// plain `"User Magic"`, not `"User Magic Skill System"`.
    const THREE_SNAILS_229: &str = "0001e803000003002cadaa3c6987f2eb00070000009fae340801060000003d1537006605d700000000006605d70000000000000000000000000000000000000000000000000000000100000000000000000a0055736572204d616769634002000000000000000000000000000000010000000100000000000000d707000005000000010000280000000000000001000007eb05d700ed05d700ec05bb0040020000000000000101000500000000000101db05b5000006d700000000006431c2c3000000000300000000004d3b2be0010000007ad973870700000002f0c080000080e8da8f00";

    /// `previous-runs/world-20260820-121055.log` 16:10:28.598, `0x00DF`, 229 bytes.
    /// A plain swing that connected: mob 2002, one hit of 19, not critical.
    const MELEE_229: &str = "0001000000000000000000000000000001050000009fae340801040000003b80680a70028b010000000070028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d20700000200000001000013000000000000000000000736028b0136028b0135027b01890100000000000001000002000000000001012302710148028b01000000007e6c3c6600000000030000000000d5c057820100000092e9bc2707000000bc6509e5000080e8da8f00";

    /// `previous-runs/world-20260820-181822.log` 22:11:21.501, `0x00DF`, 229 bytes.
    /// **A critical**: mob 2002, one hit of 24 with `flag_b` set. That log is the single
    /// capture `research/damage-formula.md` §3.2 draws the crit finding from.
    const MELEE_229_CRIT: &str = "0001000000000000000000000000000000050000009fae3408010400000034e8b20bb7028b0100000000b7028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d2070000020000000100011800000000000000010700070d038b010d038b010303770189010000000000000100000200000000000101f8026a0123038b01000000007e6c3c6600000000030000000000d5c057820100000092e9bc2707000000bc6509e5000080e8da8f00";

    /// `previous-runs/world-20260820-121055.log` 16:10:27.504, `0x00DF`, 221 bytes.
    /// Two map pairs.
    const MELEE_221: &str = "0001000000000000000000000000000001050000009fae34080104000000e57b680a8d028b01000000008d028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d2070000020000000100000f00000000000000010007075e028b015f028b015f027b01890100000000000001000002000000000001014d02720172028b01000000007e6c3c6600000000020000000000d5c057820100000092e9bc27000080e8da8f00";

    /// `previous-runs/world-20260821-134202.log` 17:40:16.657, `0x00DF`, 213 bytes.
    /// One map pair.
    const MELEE_213: &str = "0001000000000000000000000000000000050000009fae34080104000000ce08e10f97025f000000000097025f0000000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d1070000010000000100000f0000000000000001000307f5025f00f3025f00ed02460089010000000000000100000100000000000101e9023a0000035200000000000ccb966200000000010000000000062e1fc5000080e8da8f00";

    /// `previous-runs/world-20260819-222734.log` 02:25:35.692, `0x00DF`, 127 bytes.
    /// The zero-target shape — the one `research/mob-combat.md` §1 accounts for byte for
    /// byte.
    const MELEE_EMPTY_127: &str = "0000000000000000000000000000000000050000009fae340801040000005a437507e5028b0100000000e5028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c6565890100000000000000000000000000000000000000000000000000000080e8da8f00";

    /// `previous-runs/world-20260819-222734.log` 02:25:52.054, `0x00DF`, 127 bytes.
    /// **The field-9 outlier**, and the only body in the corpus with a negative y.
    const MELEE_EMPTY_127_ODD: &str = "000000000000000000000000000000000119000000a9c601dd010400000056837507c10533ff17000000c10533ff00000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c6565060100000000000000000000000000000000000000000000000000000080e8da8f00";

    /// Every embedded fixture as (name, opcode, hex).
    fn fixtures() -> Vec<(&'static str, u16, &'static str)> {
        vec![
            ("magic_claw_260", USER_MAGIC_ATTACK, MAGIC_CLAW_260),
            ("magic_claw_268", USER_MAGIC_ATTACK, MAGIC_CLAW_268),
            ("three_snails_229", USER_MAGIC_ATTACK, THREE_SNAILS_229),
            ("melee_229", USER_MELEE_ATTACK, MELEE_229),
            ("melee_229_crit", USER_MELEE_ATTACK, MELEE_229_CRIT),
            ("melee_221", USER_MELEE_ATTACK, MELEE_221),
            ("melee_213", USER_MELEE_ATTACK, MELEE_213),
            ("melee_empty_127", USER_MELEE_ATTACK, MELEE_EMPTY_127),
            ("melee_empty_127_odd", USER_MELEE_ATTACK, MELEE_EMPTY_127_ODD),
        ]
    }

    fn body(hex: &str) -> Vec<u8> {
        assert!(hex.len().is_multiple_of(2), "odd-length fixture hex");
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("fixture hex"))
            .collect()
    }

    fn parsed(hex: &str, opcode: u16) -> (Attack, usize) {
        let b = body(hex);
        let a = parse(opcode, &b).expect("fixture must parse");
        (a, b.len())
    }

    // -- the field census, which is where the byte count comes from -------------------

    #[test]
    fn the_field_census_matches_the_listing() {
        let u8s = FIELD_WIDTHS
            .iter()
            .filter(|(w, _)| *w == FieldWidth::U8)
            .count();
        let u16s = FIELD_WIDTHS
            .iter()
            .filter(|(w, _)| *w == FieldWidth::U16)
            .count();
        let u32s = FIELD_WIDTHS
            .iter()
            .filter(|(w, _)| *w == FieldWidth::U32)
            .count();
        let strs = FIELD_WIDTHS
            .iter()
            .filter(|(w, _)| *w == FieldWidth::Str)
            .count();

        // 12 u8, 11 u16, 16 u32, one str - research/mob-combat.md 1.3.
        assert_eq!((u8s, u16s, u32s, strs), (12, 11, 16, 1));
        assert_eq!(u8s + u16s + u32s + strs, 40);
        assert_eq!(u8s + u16s * 2 + u32s * 4, 98);
        assert_eq!(HEADER_FIXED_LEN, 98);
    }

    #[test]
    fn the_tail_jump_is_the_fortieth_field() {
        // The trap this repo has paid for twice: a scan that counts only `call` sees 39
        // fields. Drop the tail-jump u8 and the 2026-08-19 capture no longer reconciles.
        let without_tail: usize = FIELD_WIDTHS[..39]
            .iter()
            .map(|(w, _)| w.fixed_bytes())
            .sum();
        assert_eq!(without_tail, 97, "39 calls give 97 fixed bytes");
        assert_eq!(HEADER_FIXED_LEN, 98, "the jmp is the 40th");
        assert_eq!(EMPTY_MELEE_LEN, 127);
        assert_eq!(
            without_tail + 2 + 10 + TARGET_LIST_HEAD_LEN + TRAILER_FIXED_LEN,
            126,
            "a 39-field header lands the capture one byte short - which is exactly what \
             happened, twice"
        );
        assert_eq!(FIELD_WIDTHS[39].1, Some(0xa4));
    }

    #[test]
    fn the_field_offsets_run_in_order_and_land_where_the_doc_says() {
        // research/mob-combat.md 1.3's `off` column, recomputed from the widths.
        let mut off = 0usize;
        let mut offsets = Vec::new();
        for (w, _) in FIELD_WIDTHS.iter() {
            offsets.push(off);
            off += w.fixed_bytes();
        }
        assert_eq!(offsets[2], 2, "skill id at body offset 2");
        assert_eq!(offsets[3], 6, "skill level at body offset 6");
        assert_eq!(offsets[12], 30, "tick");
        assert_eq!(offsets[13], 34, "x");
        assert_eq!(offsets[14], 36, "y");
        assert_eq!(offsets[33], 81, "the attack-type string");
        // The struct offsets are ascending wherever they are known: a table typo would
        // show up as a step backwards.
        let known: Vec<u16> = FIELD_WIDTHS.iter().filter_map(|(_, s)| *s).collect();
        assert!(
            known.windows(2).all(|w| w[0] < w[1]),
            "struct offsets must ascend: {known:04x?}"
        );
    }

    // -- the headline: the skill id ---------------------------------------------------

    #[test]
    fn magic_claw_carries_skill_2001003_at_level_7() {
        for hex in [MAGIC_CLAW_260, MAGIC_CLAW_268] {
            let (a, _) = parsed(hex, USER_MAGIC_ATTACK);
            assert_eq!(a.header.skill_id, 2_001_003);
            assert_eq!(a.header.skill_level, 7);
            assert_eq!(a.skill(), Some((2_001_003, 7)));
            assert!(a.header.is_skill());
            assert_eq!(a.header.attack_type, "User Magic Skill System");
        }
    }

    #[test]
    fn three_snails_carries_skill_1000_at_level_3() {
        let (a, _) = parsed(THREE_SNAILS_229, USER_MAGIC_ATTACK);
        assert_eq!(a.header.skill_id, 1000);
        assert_eq!(a.header.skill_level, 3);
        // The attack-type string is NOT what tells you a skill was cast: Three Snails is
        // sent as plain "User Magic", the same string a non-skill magic attack would use.
        assert_eq!(a.header.attack_type, "User Magic");
        assert!(a.header.is_skill());
    }

    #[test]
    fn a_plain_swing_carries_skill_zero() {
        for (name, op, hex) in fixtures() {
            if !name.starts_with("melee") {
                continue;
            }
            let (a, _) = parsed(hex, op);
            assert_eq!(a.header.skill_id, 0, "{name}");
            assert_eq!(a.header.skill_level, 0, "{name}");
            assert_eq!(a.skill(), None, "{name}");
            assert!(!a.header.is_skill(), "{name}");
            assert_eq!(a.header.attack_type, "User Melee", "{name}");
        }
    }

    #[test]
    fn the_two_skill_words_at_offsets_8_and_12_are_per_skill_constants() {
        // Zero on every swing, and one fixed pair per skill. This is the pair of fields
        // research/mob-combat.md 1.3 records as "0, unexplained" - they are only zero
        // because every body it had was a swing.
        for hex in [MAGIC_CLAW_260, MAGIC_CLAW_268] {
            let (a, _) = parsed(hex, USER_MAGIC_ATTACK);
            assert_eq!(a.header.f5, 0xEB62_18B0);
            assert_eq!(a.header.f6, 0xB73A_A1C1);
        }
        let (snails, _) = parsed(THREE_SNAILS_229, USER_MAGIC_ATTACK);
        assert_eq!(snails.header.f5, 0x3CAA_AD2C);
        assert_eq!(snails.header.f6, 0xEBF2_8769);
        let (melee, _) = parsed(MELEE_229, USER_MELEE_ATTACK);
        assert_eq!((melee.header.f5, melee.header.f6), (0, 0));
    }

    // -- byte accounting, which is what catches a parser that stops early -------------

    #[test]
    fn every_fixture_is_accounted_for_byte_for_byte() {
        for (name, op, hex) in fixtures() {
            let b = body(hex);
            let a = parse(op, &b).unwrap_or_else(|e| panic!("{name}: {e}"));
            assert!(a.truncated.is_none(), "{name}: {:?}", a.truncated);
            assert_eq!(a.consumed, b.len(), "{name}: parser stopped early");
            // The independent half: the length the parsed CONTENTS imply, not the cursor.
            assert_eq!(
                a.recomputed_len(),
                Some(b.len()),
                "{name}: contents do not add up to the captured length"
            );
        }
    }

    #[test]
    fn the_lengths_decompose_the_way_the_doc_says_they_do() {
        // 110 + 12 + 5 = 127 with targetCount 0, from research/mob-combat.md 1.4.
        let (empty, len) = parsed(MELEE_EMPTY_127, USER_MELEE_ATTACK);
        assert_eq!(len, 127);
        assert_eq!(empty.header.encoded_len(), 110);
        assert_eq!(empty.target_count, 0);
        assert_eq!(empty.targets.len(), 0);
        assert_eq!(
            empty.header.encoded_len() + TARGET_LIST_HEAD_LEN + TRAILER_FIXED_LEN,
            127
        );

        // The pair count is worth 8 bytes each, and nothing else differs between these
        // three. That is the check that the u16 before the tree walk really is a count.
        let (p1, l1) = parsed(MELEE_213, USER_MELEE_ATTACK);
        let (p2, l2) = parsed(MELEE_221, USER_MELEE_ATTACK);
        let (p3, l3) = parsed(MELEE_229, USER_MELEE_ATTACK);
        assert_eq!((l1, l2, l3), (213, 221, 229));
        assert_eq!(
            (
                p1.targets[0].pair_count,
                p2.targets[0].pair_count,
                p3.targets[0].pair_count
            ),
            (1, 2, 3)
        );
        assert_eq!(l2 - l1, 8);
        assert_eq!(l3 - l2, 8);
    }

    // -- the target block, against real targets ---------------------------------------

    #[test]
    fn a_connecting_swing_names_the_mob_and_its_damage() {
        let (a, _) = parsed(MELEE_229, USER_MELEE_ATTACK);
        assert_eq!(a.target_count, 1);
        assert_eq!(a.targets.len(), 1);
        let t = &a.targets[0];
        // Map 40's object ids are 2000-2039, which is what makes the FIRST u32 the id.
        assert_eq!(t.object_id, 2002);
        assert!(
            (2000..=2039).contains(&t.object_id),
            "the first u32 must be an id this server assigned"
        );
        assert_eq!(t.t14, 2, "the second u32 is not an object id");
        assert_eq!(t.hit_count, 1);
        assert_eq!(t.hits.len(), 1);
        assert_eq!(t.hits[0].damage, 19);
        assert!(!t.hits[0].flag_a);
        assert!(!t.hits[0].flag_b);
        assert_eq!(t.total_damage(), 19);
        assert!(!t.any_critical());
        assert!(!t.has_object);
        assert_eq!(t.mode, Some(0));
        assert_eq!(t.mode_payload, None);
    }

    #[test]
    fn flag_b_is_the_critical_flag() {
        // research/damage-formula.md 3.2: in world-20260820-181822.log, flag_b == 0 gave
        // 64 hits of 15..20 and flag_b == 1 gave 5 hits of 20, 21, 22, 24. A 24 cannot be
        // a draw from a distribution whose observed maximum is 20.
        let (crit, _) = parsed(MELEE_229_CRIT, USER_MELEE_ATTACK);
        let ct = &crit.targets[0];
        assert_eq!(ct.hits[0].damage, 24);
        assert!(ct.hits[0].flag_b, "the 24-damage hit is flagged");
        assert!(!ct.hits[0].flag_a, "flag_a was 0 on all 361 measured hits");
        assert!(ct.any_critical());

        let (plain, _) = parsed(MELEE_229, USER_MELEE_ATTACK);
        assert_eq!(plain.targets[0].hits[0].damage, 19);
        assert!(!plain.targets[0].hits[0].flag_b);
        assert!(!plain.any_critical_target());
        // Same character, same map, same weapon: the ONLY difference between these two
        // bodies' damage is the flag, and the flagged one is above the unflagged max.
        assert!(ct.hits[0].damage > plain.targets[0].hits[0].damage);
    }

    #[test]
    fn magic_claw_hits_twice_and_three_snails_once() {
        let (claw, _) = parsed(MAGIC_CLAW_260, USER_MAGIC_ATTACK);
        let t = &claw.targets[0];
        assert_eq!(t.object_id, 2006);
        assert_eq!(t.hit_count, 2, "Magic Claw's attackCount is 2");
        assert_eq!(t.hits.len(), 2);
        assert_eq!(t.hits.iter().map(|h| h.damage).collect::<Vec<_>>(), vec![1, 1]);
        assert_eq!(claw.total_damage(), 2);

        let (snails, _) = parsed(THREE_SNAILS_229, USER_MAGIC_ATTACK);
        let s = &snails.targets[0];
        assert_eq!(s.object_id, 2007);
        assert_eq!(s.hit_count, 1);
        assert_eq!(s.hits[0].damage, 40);
    }

    #[test]
    fn header_field_1_equals_the_target_count() {
        // True in all 434 distinct captured bodies. Never > 1, so "count" and "boolean" are still
        // indistinguishable - that is what the assertion is allowed to say and no more.
        for (name, op, hex) in fixtures() {
            let (a, _) = parsed(hex, op);
            assert_eq!(a.header.f1 as u32, a.target_count, "{name}");
            assert!(a.target_count <= 1, "{name}: a multi-target swing at last");
        }
    }

    // -- the trailer, which is where mob-combat.md 1.4 is wrong ------------------------

    #[test]
    fn the_trailer_flag_gates_eight_more_bytes() {
        // FUN_14083b270 writes u32, u8 (obj != NULL), and if the flag is set a u32 plus a
        // node list plus the 0xFFFFFFFF terminator at 0x14083bbe2. Every Magic Claw sets
        // it with an empty list: 00000000 FFFFFFFF.
        for hex in [MAGIC_CLAW_260, MAGIC_CLAW_268] {
            let (a, len) = parsed(hex, USER_MAGIC_ATTACK);
            let t = a.trailer.as_ref().expect("trailer");
            assert_eq!(t.value, TRAILER_USUAL_VALUE);
            assert!(t.has_object, "Magic Claw sets the trailer's object flag");
            assert_eq!(
                t.extra,
                vec![0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF],
                "u32 [obj+0x1c] = 0, empty node list, then the terminator"
            );
            assert!(t.ends_with_terminator());
            assert_eq!(t.encoded_len(), 13);

            // The point of the whole test, stated as the number that would have been
            // wrong: a parser that takes mob-combat.md 1.4 at its word and stops after
            // `u32 + u8` reaches exactly eight bytes short of the captured length.
            let targets_len: usize = a
                .targets
                .iter()
                .map(|t| t.encoded_len().expect("no sub-object in the corpus"))
                .sum();
            let naive = a.header.encoded_len() + TARGET_LIST_HEAD_LEN + targets_len + TRAILER_FIXED_LEN;
            assert_eq!(naive, len - 8, "a 5-byte trailer leaves 8 bytes unread");
            assert_eq!(a.consumed, len);
        }

        for (hex, op) in [
            (MELEE_229, USER_MELEE_ATTACK),
            (MELEE_213, USER_MELEE_ATTACK),
            (MELEE_EMPTY_127, USER_MELEE_ATTACK),
            (THREE_SNAILS_229, USER_MAGIC_ATTACK),
        ] {
            let (a, _) = parsed(hex, op);
            let t = a.trailer.as_ref().expect("trailer");
            assert_eq!(t.value, TRAILER_USUAL_VALUE);
            assert!(!t.has_object);
            assert!(t.extra.is_empty());
            assert_eq!(t.encoded_len(), TRAILER_FIXED_LEN);
        }
    }

    // -- the two corrections to what was already written down --------------------------

    #[test]
    fn field_9_is_near_constant_with_one_named_outlier() {
        // research/attack-skill-id.md 3 called it a constant off three captures. It is the
        // same value in 433 of 434 distinct bodies; this is the exception, and it comes
        // with two other fields moving at the same time.
        for (name, op, hex) in fixtures() {
            let (a, _) = parsed(hex, op);
            if name == "melee_empty_127_odd" {
                assert_eq!(a.header.f9, 0xDD01_C6A9, "the one outlier");
                assert_eq!(a.header.f8, 25);
                assert_eq!(a.header.f34, 262);
            } else {
                assert_eq!(a.header.f9, FIELD_9_USUAL, "{name}");
            }
        }
    }

    #[test]
    fn the_coordinates_are_signed() {
        let (odd, _) = parsed(MELEE_EMPTY_127_ODD, USER_MELEE_ATTACK);
        assert_eq!(odd.header.x, 1473);
        assert_eq!(odd.header.y, -205, "0xFF33 is a map coordinate, not 65331");
        assert_eq!(odd.header.f17, -205);

        let (m, _) = parsed(MELEE_229, USER_MELEE_ATTACK);
        assert_eq!((m.header.x, m.header.y), (624, 395));
    }

    #[test]
    fn the_second_position_tracks_the_first_except_on_a_skill_cast() {
        // f13 == f16 in all 434; f14 == f17 in 427 and differs in the 7 Magic Claws.
        for (name, op, hex) in fixtures() {
            let (a, _) = parsed(hex, op);
            assert_eq!(a.header.f16, a.header.x, "{name}: x is written twice");
            if name.starts_with("magic_claw") {
                assert_ne!(
                    a.header.f17, a.header.y,
                    "{name}: the two positions are separate fields, not one written twice"
                );
            } else {
                assert_eq!(a.header.f17, a.header.y, "{name}");
            }
        }
    }

    #[test]
    fn the_tick_matches_the_capture_the_doc_cross_checked() {
        // research/mob-combat.md 1.5 pinned the tick against a 0x00D9 852 ms earlier.
        // Every fixture from a given session must have ticks that only move forward.
        let (a, _) = parsed(MELEE_221, USER_MELEE_ATTACK);
        let (b, _) = parsed(MELEE_229, USER_MELEE_ATTACK);
        // Same log, 16:10:27.504 and 16:10:28.598 - 1094 ms apart on the clock.
        let delta = b.header.tick - a.header.tick;
        assert!(
            (1000..1200).contains(&delta),
            "tick delta {delta} should match the 1094 ms between the two log lines"
        );
    }

    // -- always answer: nothing in here may panic or refuse ----------------------------

    #[test]
    fn parsing_any_prefix_of_every_fixture_never_panics() {
        for (name, op, hex) in fixtures() {
            let full = body(hex);
            for n in 0..=full.len() {
                // Err is a logging event, never a refusal, and never a panic.
                if let Ok(a) = parse(op, &full[..n]) {
                    assert!(a.consumed <= n, "{name}[..{n}]: read past the end");
                    if n == full.len() {
                        assert!(a.truncated.is_none(), "{name}: full body must be clean");
                        assert_eq!(a.consumed, n, "{name}: full body must be exact");
                    }
                    // A prefix strictly shorter than the header cannot be Ok at all.
                    assert!(
                        n >= HEADER_FIXED_LEN,
                        "{name}[..{n}]: parsed a body shorter than the fixed header"
                    );
                }
            }
            // A truncated body with a trailer object is NOT detectable and this test does
            // not pretend otherwise: AttackTrailer::extra is "the rest of the body", so a
            // prefix that cuts into it looks like a shorter but complete packet. That is
            // a property of the 42-way node switch being undecoded, and it is why
            // ends_with_terminator() exists for a caller that wants to notice.
        }
    }

    #[test]
    fn parsing_pseudo_random_bodies_never_panics() {
        // A body the client never sent still has to not take the server down. An LCG so
        // the corpus is deterministic and this test cannot go green by luck one day and
        // red the next.
        let mut state: u64 = 0x2026_0828_0000_00DF;
        let mut next = || {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (state >> 33) as u8
        };
        for len in 0..400usize {
            let buf: Vec<u8> = (0..len).map(|_| next()).collect();
            for op in [USER_MELEE_ATTACK, USER_SHOOT_ATTACK, USER_MAGIC_ATTACK] {
                if let Ok(a) = parse(op, &buf) {
                    assert!(a.consumed <= buf.len());
                }
            }
        }
    }

    #[test]
    fn a_body_that_is_far_too_short_reports_which_field_ran_out() {
        let err = parse(USER_MELEE_ATTACK, &[0u8; 3]).unwrap_err();
        match err {
            AttackError::ShortHeader {
                field,
                offset,
                have,
                body_len,
            } => {
                assert_eq!(field, 2, "the u32 skill id at body offset 2");
                assert_eq!(offset, 2);
                assert_eq!(have, 1);
                assert_eq!(body_len, 3);
            }
            other => panic!("wrong error: {other:?}"),
        }
        // And an empty body says field 0, not something arbitrary.
        match parse(USER_MELEE_ATTACK, &[]).unwrap_err() {
            AttackError::ShortHeader { field, .. } => assert_eq!(field, 0),
            other => panic!("wrong error: {other:?}"),
        }
    }

    #[test]
    fn a_body_that_ends_in_the_target_list_head_says_so() {
        let full = body(MELEE_EMPTY_127);
        // 110 bytes of header plus 4 of the three-u32 head.
        let err = parse(USER_MELEE_ATTACK, &full[..114]).unwrap_err();
        assert!(
            matches!(err, AttackError::ShortTargetListHead { body_len: 114, .. }),
            "{err:?}"
        );
    }

    #[test]
    fn a_body_that_ends_inside_a_target_is_truncated_not_an_error() {
        // The always-answer rule in code: losing the fourth mob of a five-mob swing is
        // worth far less than losing the reply.
        let full = body(MELEE_229);
        let a = parse(USER_MELEE_ATTACK, &full[..130]).expect("must still parse");
        assert_eq!(a.target_count, 1);
        assert!(a.targets.is_empty());
        assert!(matches!(
            a.truncated,
            Some(AttackTruncation::ShortTarget { index: 0, .. })
        ));
        assert_eq!(a.trailer, None);
        assert_eq!(a.recomputed_len(), None, "a truncated parse cannot be measured");
    }

    #[test]
    fn only_the_three_shared_body_opcodes_are_accepted() {
        let b = body(MELEE_229);
        for op in [USER_MELEE_ATTACK, USER_SHOOT_ATTACK, USER_MAGIC_ATTACK] {
            assert!(parse(op, &b).is_ok(), "0x{op:04X}");
            assert!(attack_opcode_name(op).is_some());
        }
        // 0x00E2 uses the classic inline shape and never calls FUN_140f31fe0. Parsing it
        // with this layout would produce confident nonsense.
        assert_eq!(
            parse(0x00E2, &b),
            Err(AttackError::NotAnAttack { opcode: 0x00E2 })
        );
        assert_eq!(attack_opcode_name(0x00E2), None);
        assert_eq!(
            parse(0x0000, &b),
            Err(AttackError::NotAnAttack { opcode: 0x0000 })
        );
    }

    #[test]
    fn parse_with_opcode_splits_the_front_off() {
        let b = body(MELEE_229);
        let mut packet = USER_MELEE_ATTACK.to_le_bytes().to_vec();
        packet.extend_from_slice(&b);
        let a = parse_with_opcode(&packet).expect("must parse");
        assert_eq!(a.opcode, USER_MELEE_ATTACK);
        assert_eq!(a.targets[0].object_id, 2002);
        assert_eq!(parse_with_opcode(&[0x01]), Err(AttackError::NotAnAttack { opcode: 0 }));
    }

    // -- the whole corpus, when it is available ---------------------------------------

    /// Re-run every assertion over the full capture corpus rather than the nine embedded
    /// fixtures.
    ///
    /// `previous-runs/` is gitignored and rolls, so this cannot be an unconditional test.
    /// It is not a skip in disguise either: when the variable is set it fails loudly if
    /// the file is unreadable or empty, because "the sweep found nothing" and "the sweep
    /// did not run" must not look the same.
    #[test]
    fn the_whole_capture_corpus_is_accounted_for_byte_for_byte() {
        let path = match std::env::var(env::CORPUS) {
            Ok(p) => p,
            Err(_) => return,
        };
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is set to {path} and cannot be read: {e}", env::CORPUS));
        let mut n = 0usize;
        let mut with_targets = 0usize;
        let mut skills: std::collections::BTreeMap<(u32, u8), usize> =
            std::collections::BTreeMap::new();
        for line in text.lines() {
            let mut it = line.split('\t');
            let (op, _len, src, stamp, hex) = match (
                it.next(),
                it.next(),
                it.next(),
                it.next(),
                it.next(),
            ) {
                (Some(a), Some(b), Some(c), Some(d), Some(e)) => (a, b, c, d, e),
                _ => panic!("bad corpus line: {line}"),
            };
            let opcode = u16::from_str_radix(op.trim_start_matches("0x"), 16)
                .unwrap_or_else(|_| panic!("bad opcode {op}"));
            let b = body(hex);
            let where_ = format!("{src} {stamp} 0x{opcode:04X} {} bytes", b.len());
            let a = parse(opcode, &b).unwrap_or_else(|e| panic!("{where_}: {e}"));
            assert!(a.truncated.is_none(), "{where_}: {:?}", a.truncated);
            assert_eq!(a.consumed, b.len(), "{where_}: parser stopped early");
            assert_eq!(a.recomputed_len(), Some(b.len()), "{where_}");
            assert_eq!(a.header.f1 as u32, a.target_count, "{where_}");
            let t = a.trailer.as_ref().expect("trailer");
            assert_eq!(t.value, TRAILER_USUAL_VALUE, "{where_}");
            assert_eq!(t.extra.len(), if t.has_object { 8 } else { 0 }, "{where_}");
            if t.has_object {
                assert!(t.ends_with_terminator(), "{where_}");
            }
            if !a.targets.is_empty() {
                with_targets += 1;
            }
            *skills
                .entry((a.header.skill_id, a.header.skill_level))
                .or_default() += 1;
            n += 1;
        }
        assert!(n > 0, "{} pointed at an empty corpus", env::CORPUS);
        eprintln!("corpus: {n} bodies, {with_targets} with a target, skills {skills:?}");
    }
}
