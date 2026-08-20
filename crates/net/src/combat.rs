//! Combat: the client's attack packet, the mob's death, and the stat change.
//!
//! Full working, with every address: **`research/mob-combat.md`**.
//!
//! # The one thing to read before using any of this
//!
//! **This client computes its own damage.** The attack body carries a per-target list of
//! `u64` damage values that the *client* worked out, so the numbers on screen do not come
//! back from us - a client that has already computed a number is not waiting to be told
//! what it is. That splits the owner's report in two:
//!
//! | symptom | who owns it |
//! |---|---|
//! | no damage numbers appear | the **client**, which must put the mob in its own target list |
//! | the mob never dies, no EXP | the **server**: read the packet, apply damage, answer |
//!
//! This module builds the second half. It cannot build the first, and neither can any
//! reply.
//!
//! # And the measurement that says the first half is real
//!
//! `world.log` from 2026-08-19 contains exactly one attack - `0x00DF`, 127 bytes, at
//! `23:58:33.878`, standing at `(473, 395)` with mob `2000` at `(424, 395)`. Its target
//! count is **zero**. All 127 bytes are accounted for field for field ([`ATTACK_HEADER_FIXED`]
//! and [`ATTACK_TRAILER_LEN`]), so that zero is read, not assumed.
//!
//! `research/mob-combat.md` §8 has the run that separates "they swung out of range" from
//! "the client will not target our mobs": attack a snail four or five times and look at
//! whether any `0x00DF` body is longer than 127 bytes.

use crate::error::Result;
use crate::packet::{PacketReader, PacketWriter};

// ---------------------------------------------------------------------------------------
// Opcodes
// ---------------------------------------------------------------------------------------

/// Melee attack, **client -> server**. Captured in `world.log`.
///
/// Built by `FUN_1428baa50`, `FUN_1428c1fa0` and `FUN_1429a6590`, each of which is
/// `COutPacket(0xDF)` followed by exactly three encoders - `FUN_140f31fe0` (the header),
/// `FUN_140f31f60` (the target list) and `FUN_14083b270` (the trailer). **[L]**
///
/// Three *other* builders in the image also emit `0x00DF` with a different, inline body
/// (`FUN_1428bc4d0`, `FUN_1428c01b0`, `FUN_1428c5aa0`). None of them writes a string, and
/// the captured body contains one, so the capture came from the three-encoder group and
/// that is what [`parse_attack`] reads. **[D]**
pub const USER_MELEE_ATTACK: u16 = 0x00DF;

/// Ranged attack, **client -> server**. Same body as [`USER_MELEE_ATTACK`].
///
/// `FUN_1429a7ab0` calls the identical `FUN_140f31fe0` header encoder - one of the six call
/// sites `tools/callers.py 0x140f31fe0` reports. **[L]**
pub const USER_SHOOT_ATTACK: u16 = 0x00E0;

/// Magic attack, **client -> server**. Same body as [`USER_MELEE_ATTACK`].
///
/// `FUN_1428cd6d0` and `FUN_1429a7090`, both through `FUN_140f31fe0`. **[L]**
pub const USER_MAGIC_ATTACK: u16 = 0x00E1;

/// The fourth attack opcode. **Do not parse it with [`parse_attack`].**
///
/// Its only builder, `FUN_1428d07c0`, writes the classic inline shape
/// (`u8, u8, u32, u32, …`) and never calls `FUN_140f31fe0`, so its body is a different
/// layout that has not been decoded. It has never been seen on the wire here. **[L]**
pub const USER_BODY_ATTACK: u16 = 0x00E2;

/// A mob leaves the field - **server -> client**, and the only way a mob dies.
///
/// Mob pool `FUN_141d30e80` `case 0x3d1` -> `FUN_141d33c70`, `.pdata`-bounded to
/// `0x141d33c70..0x141d3426c`. See [`mob_leave_field`]. **[L]**
pub const MOB_LEAVE_FIELD: u16 = 0x03D1;

/// `CWvsContext::OnStatChanged` - **server -> client**. EXP, HP, level, AP and SP all ride
/// this one packet. Game-stage `FUN_142cbaa80` `case 0x7c` -> `FUN_142d54780`. **[L]**
///
/// See [`StatChange`] for the mask and [`stat_changed`] for the body.
pub const STAT_CHANGED: u16 = 0x007C;

/// Is this one of the attack opcodes [`parse_attack`] understands?
///
/// [`USER_BODY_ATTACK`] is deliberately **not** in the set - it has a different body.
pub fn is_attack_opcode(opcode: u16) -> bool {
    matches!(
        opcode,
        USER_MELEE_ATTACK | USER_SHOOT_ATTACK | USER_MAGIC_ATTACK
    )
}

// ---------------------------------------------------------------------------------------
// The attack packet
// ---------------------------------------------------------------------------------------

/// Bytes the attack header costs on top of its one variable-length string.
///
/// `FUN_140f31fe0` writes **40 fields** with no conditional branch anywhere in its
/// `0x140f31fe0..0x140f321f0` range: **12 x `u8`, 11 x `u16`, 16 x `u32`, one `str`**. That
/// is `12 + 22 + 64 = 98` fixed bytes plus the string's `2 + len`, so a header is
/// `98 + 2 + len` = **110** for `"User Melee"`. **[L]**
///
/// # The 40th field is a tail jump, and missing it costs a byte
///
/// The function ends `jmp 0x1406ed840` at `0x140f321eb` - a tail call to the `u8` writer.
/// A scan that counts only `call` sees 39 fields, makes the header 109 bytes, and then has
/// to find a stray byte elsewhere to reconcile with the 127-byte capture. It did, and it
/// was wrong. **A tail `jmp` into a writer is a field.**
///
/// The write-side primitive set was then enumerated the way `tools/reads.py` enumerates
/// the read side - every `jmp rel32` in `.text`/`.boot` landing on one of the seven
/// `COutPacket` writers and preceded by `0xCC` padding. There is **exactly one writer
/// thunk**, `0x1406ee000` -> `0x1406ed840` (`u8`), and **none of the four encoders in this
/// module calls it**. So the counts here are over the full primitive set, not a filtered
/// one. **[L]**
pub const ATTACK_HEADER_FIXED: usize = 98;

/// The target list's own head: `u32 count` plus two more `u32`s, written by
/// `FUN_140f31f60` before the per-target loop. The **first** of the three is the loop
/// bound (`cmp [rdi],ebx / jle`), so it is the count. **[L]**
pub const ATTACK_TARGETS_HEAD_LEN: usize = 12;

/// `FUN_14083b270(packet, 0)`: a `u32` then a `u8`, unconditionally, after the targets.
/// **[L]**
pub const ATTACK_TRAILER_LEN: usize = 5;

/// The length of the one attack captured on 2026-08-19: `"User Melee"`, no targets.
///
/// `ATTACK_HEADER_FIXED + 2 + 10 + ATTACK_TARGETS_HEAD_LEN + ATTACK_TRAILER_LEN` = **127**,
/// which is exactly what `world.log` recorded. Every byte is accounted for. **[D]**
pub const CAPTURED_EMPTY_ATTACK_LEN: usize = 127;

/// One hit on one target. `nDamage` of these follow each target's head.
///
/// `FUN_140f31bb0` writes them as `u8 !=0`, `u8 !=0`, `u64`, stride `0x10` in the source
/// struct and **10 bytes on the wire**. **[L]** The two flags are unexplained; in this game
/// family one of them is "critical" - that is **[I]** and nothing here reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackHit {
    /// `[t+0x28+i*0x10] != 0`.
    pub flag_a: bool,
    /// `[t+0x29+i*0x10] != 0`.
    pub flag_b: bool,
    /// `[t+0x30+i*0x10]`, a full `u64`. The client computed this.
    pub damage: u64,
}

/// One target of an attack.
///
/// # Which field is the mob id is not settled
///
/// The block opens with two `u32`s, `[t+0x10]` then `[t+0x14]`. The capture carried no
/// targets, so neither has ever been seen with a value in it, and the code that fills them
/// lives in a 15 kB target-collection function that was not read. [`Self::object_id`] is
/// the **first**, which is the order the reference server uses - **[I]**, and
/// [`Self::second`] is kept so the answer can be checked without re-parsing.
///
/// One client run settles it: object ids on map 40 are 2000-2039.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackTarget {
    /// `[t+0x10]`. **Candidate** mob object id - see the type's note.
    pub object_id: u32,
    /// `[t+0x14]`. Unexplained.
    pub second: u32,
    /// The damage the client says it dealt, one entry per hit.
    pub hits: Vec<AttackHit>,
}

impl AttackTarget {
    /// Total damage across every hit, saturating.
    pub fn total_damage(&self) -> u64 {
        self.hits
            .iter()
            .fold(0u64, |acc, h| acc.saturating_add(h.damage))
    }
}

/// A parsed [`USER_MELEE_ATTACK`] / [`USER_SHOOT_ATTACK`] / [`USER_MAGIC_ATTACK`].
///
/// Only the fields with a name are ones two things agree on. The other 30 header fields
/// were all zero in the one capture except five, none of which is explained - see
/// `research/mob-combat.md` §7.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackRequest {
    /// Header field 33, at body offset 81: the attack's own type **as a string**, e.g.
    /// `"User Melee"`. The client picks it out of a 60-entry table of 16-byte slots at
    /// `0x1432b3f10` (`User Normal, User Melee, User Shoot, User Magic, User Body, …`).
    /// **[L]**
    pub attack_type: String,
    /// Header field 12, body offset 30. Cross-checked against the `0x00D9` movement packet
    /// 852 ms earlier: the two ticks differ by 870, so the field is a millisecond tick.
    /// **[L]**
    pub tick: u32,
    /// Header fields 13/14, body offsets 34 and 36. The capture's `473, 395` is the exact
    /// position the next `0x00D9` reports the player walking away from. **[L]**
    pub x: u16,
    /// See [`Self::x`].
    pub y: u16,
    /// The targets the **client** decided it hit. Empty is a real, common answer - it is
    /// what the one capture contains.
    pub targets: Vec<AttackTarget>,
    /// How many targets the packet said it had. Equal to `targets.len()` unless
    /// [`Self::truncated`].
    pub target_count: u32,
    /// True when a target block could not be skipped and parsing stopped early.
    ///
    /// A target's tail contains an optional sub-object whose `raw(n)` length has not been
    /// resolved (`FUN_14025d3c0`), so a target that sets that flag cannot be stepped over
    /// to reach the next one. Everything already in [`Self::targets`] is still good; the
    /// rest of the packet was not read.
    ///
    /// **This is never a reason not to reply.** See the module docs on the always-answer
    /// rule.
    pub truncated: bool,
}

/// Parse an attack body (opcode already consumed).
///
/// Errors only on a genuine underrun. A body that parses but whose later targets could not
/// be skipped comes back with [`AttackRequest::truncated`] set and the targets that were
/// read - because losing the fourth mob of a five-mob swing is worth far less than losing
/// the reply.
pub fn parse_attack(body: &[u8]) -> Result<AttackRequest> {
    let mut r = PacketReader::new(body);

    // -- header: FUN_140f31fe0, 40 fields, no branches --------------------------------
    r.skip(30)?; //                       0..29   fields 0-11
    let tick = r.u32()?; //              30       field 12  141f31…  the tick
    let x = r.u16()?; //                 34       field 13
    let y = r.u16()?; //                 36       field 14
    r.skip(43)?; //                      38..80   fields 15-32
    let attack_type = r.str()?; //       81       field 33
    r.skip(17)?; //                      +12      fields 34-39, incl. the tail-jump u8

    // -- target list: FUN_140f31f60 ---------------------------------------------------
    let target_count = r.u32()?;
    let _ = r.u32()?;
    let _ = r.u32()?;

    let mut targets = Vec::new();
    let mut truncated = false;
    for _ in 0..target_count {
        match parse_attack_target(&mut r) {
            Ok((target, complete)) => {
                targets.push(target);
                if !complete {
                    truncated = true;
                    break;
                }
            }
            Err(_) => {
                truncated = true;
                break;
            }
        }
    }

    Ok(AttackRequest {
        attack_type,
        tick,
        x,
        y,
        targets,
        target_count,
        truncated,
    })
}

/// One target block, `FUN_140f31bb0`.
///
/// Returns the target and whether the reader is positioned at the **next** target. The
/// second half of the block is skipped rather than decoded, because none of it is
/// understood and all of it is fixed-width or length-prefixed - with the single exception
/// noted on [`AttackRequest::truncated`].
fn parse_attack_target(r: &mut PacketReader) -> Result<(AttackTarget, bool)> {
    let object_id = r.u32()?; //  [t+0x10]
    let second = r.u32()?; //     [t+0x14]
    let n_damage = r.u8()?; //    [t+0x20], written u8; the loop bound is the dword there
    let mut hits = Vec::with_capacity(n_damage as usize);
    for _ in 0..n_damage {
        hits.push(AttackHit {
            flag_a: r.bool()?,
            flag_b: r.bool()?,
            damage: r.u64()?,
        });
    }
    let target = AttackTarget {
        object_id,
        second,
        hits,
    };

    // 4 u8, 6 u16, 2 u32, 3 u8, 1 u32, 4 u8, 4 u16, 3 u32  = 4+12+8+3+4+4+8+12 = 55
    r.skip(55)?;

    // A std::map: _Myhead at [t+0x190], _Mysize at [t+0x198]. The u16 written immediately
    // before the tree walk IS the walk's length - that is the MSVC map layout, and it is
    // the only thing that makes the pair loop parseable at all.
    let pairs = r.u16()?;
    r.skip(pairs as usize * 8)?;

    // [t+0x1a0]: an optional sub-object, FUN_14025d3c0 -> u32, u32, raw(n). `n` is not
    // resolved, so this is where the parse has to stop.
    if r.bool()? {
        return Ok((target, false));
    }

    // [t+0x1b0] is a three-way, NOT a flag: `sub ecx,1 / je <1> / cmp ecx,1 / jne <done>`.
    match r.u8()? {
        1 => {
            let _ = r.str()?;
            let _ = r.u32()?;
            let _ = r.u8()?;
            if r.bool()? {
                // FUN_141fde400 -> u32, str
                let _ = r.u32()?;
                let _ = r.str()?;
            }
        }
        2 => {
            let _ = r.str()?;
            let _ = r.u32()?;
            let _ = r.u8()?;
        }
        _ => {}
    }

    Ok((target, true))
}

// ---------------------------------------------------------------------------------------
// The mob dies
// ---------------------------------------------------------------------------------------

/// `deathType` values, read off `FUN_141d33c70`'s own branches.
///
/// The two trailing `u32`s are gated by `FUN_1402b3b80(deathType)`, a 12-entry jump table
/// at `0x1402b3ba4` read straight out of the image: **true for 0, 1, 4, 6, 7, 8, 9, 10, 11;
/// false for 2, 3, 5 and anything >= 12.** [`mob_leave_field`] handles that. **[L]**
pub mod death {
    /// Remove the mob with no animation and no notification.
    ///
    /// `141d33dc1 test edi,edi / jne` takes the immediate-teardown path -
    /// `FUN_141c543c0` / `FUN_141c543e0` / `FUN_141c54dd0`. **[L]**
    pub const IMMEDIATE: u8 = 0;

    /// The killed-in-combat death: `141d33dab cmp edi,1 / jne` falls into
    /// `FUN_142d156a0(DAT_…)` - the **only** type that notifies a global singleton before
    /// the removal tail. That is the shape of "tell the quest / monster-book system
    /// something died". **[L]** for the branch, **[I]** for the name.
    ///
    /// This is what [`super::mob_leave_field`] should be sent with when a player kills a
    /// mob. It has never been on the wire.
    pub const ANIMATED: u8 = 1;

    /// `141d33eac cmp edi,6 / jne` - its own path, ending in a different pool operation.
    /// Not used here; recorded so nobody has to re-read the ladder.
    pub const TYPE_6: u8 = 6;

    /// `141d33f0e cmp edi,9 / jne` - reads **one extra `u32`** at `141d33f16` and hands it
    /// to `FUN_141ceb8d0`. **[L]**
    pub const TYPE_9: u8 = 9;
}

/// Does this `deathType` pull in the two `u32`s after the second `u8`?
///
/// The 12 dwords at `0x1402b3ba4` resolve to the "return 1" arm at `0x1402b3b9b` for
/// indices 0, 1, 4, 6, 7, 8, 9, 10, 11 and to the "return 0" arm at `0x1402b3ba1` for
/// 2, 3, 5. `CMP ECX,0xB / JA` sends anything >= 12 to the zero arm. **[L]**
pub fn death_type_carries_pair(death_type: u8) -> bool {
    matches!(death_type, 0 | 1 | 4 | 6 | 7 | 8 | 9 | 10 | 11)
}

/// `MobLeaveField` body: take a mob off the field, optionally with its death animation.
///
/// ```text
/// u32  objectId          141d33c92
/// u8   deathType         141d33ca5
/// u8   (unnamed)         141d33cb0   -> FUN_141c56670(mob, v)
/// [u32 a]                141d33cdb   -> FUN_141c56640(mob, deathType, a)   } only when
/// [u32 b]                141d33cea   -> FUN_141c56660(mob, b)              } the pair is on
/// [u32]                  141d33cfa   only when deathType == 4
/// [u32]                  141d33f16   only when deathType == 9
/// ```
///
/// **14 bytes** for [`death::ANIMATED`]. The two `u32`s go out as zero - what they mean is
/// not established, and `FUN_141c56640`/`FUN_141c56660` are plain field setters, so a zero
/// is a value rather than a missing structure.
///
/// Death types 4 and 9 are **not** supported: they need a further `u32` this builder does
/// not write, and sending one short would underrun the client's reader, which throws.
pub fn mob_leave_field(object_id: u32, death_type: u8) -> Vec<u8> {
    debug_assert!(
        death_type != 4 && death_type != 9,
        "death types 4 and 9 read an extra u32 this builder does not write"
    );
    let mut w = PacketWriter::new();
    w.u32(object_id);
    w.u8(death_type);
    w.u8(0);
    if death_type_carries_pair(death_type) {
        w.u32(0);
        w.u32(0);
    }
    w.into_vec()
}

// ---------------------------------------------------------------------------------------
// StatChanged
// ---------------------------------------------------------------------------------------

/// The `0x007C` stat mask, bit for bit.
///
/// Decoded by `FUN_1402cbb50` - **whose `.pdata` entry ends at `0x1402cbbb4` while the
/// function runs to `0x1402cbf70` across five entries**. A scan bounded by the first entry
/// reports three bits out of eighteen, cleanly and confidently. `.pdata` bounds a chunk,
/// not a function; check whether the last instruction is a `ret`.
///
/// Every bit is named without a single reference-server guess. The decoder writes into the
/// same object the **character record's** stat decoder `FUN_140302e30` fills, and
/// `research/charstat-layout.md` already published that object's offset table - so
/// "bit 16 writes `+0x9b`" and "record read 21 is `exp` and writes `+0x9b`" are two
/// independent readings that agree. **[D]** throughout, on two [L] sources.
pub mod stat {
    /// `u8` skin **and** the record's always-zero `u32` - one bit, two fields.
    /// `1402cbb7f`, `1402cbb8a` -> `+0x1a`, `+0x1b`.
    pub const SKIN: u32 = 0x0000_0001;
    /// `u32` -> `+0x1f`. `1402cbb9b`.
    pub const FACE: u32 = 0x0000_0002;
    /// `u32` -> `+0x23`. `1402cbbac`.
    pub const HAIR: u32 = 0x0000_0004;
    /// **Bit 3 is not read at all.** `test bpl,4` is followed straight by `test bpl,0x10`;
    /// nothing in the decoder tests `0x8`. The reference server's `PET` slot. **[L]**
    pub const UNREAD_BIT_3: u32 = 0x0000_0008;
    /// `u32`, obfuscated into `+0x27`. `1402cbbc2`.
    pub const LEVEL: u32 = 0x0000_0010;
    /// `u16` job **and** `u16` subJob - one bit, two fields. `1402cbbf8`, `1402cbc0f`
    /// -> `+0x33` and `+0x10c`.
    pub const JOB: u32 = 0x0000_0020;
    /// `u16` -> `+0x3b`. `1402cbc24`.
    pub const STR: u32 = 0x0000_0040;
    /// `u16` -> `+0x43`. `1402cbc40`.
    pub const DEX: u32 = 0x0000_0080;
    /// `u16` -> `+0x4b`. `1402cbc5d`.
    pub const INT: u32 = 0x0000_0100;
    /// `u16` -> `+0x53`. `1402cbc7a`.
    pub const LUK: u32 = 0x0000_0200;
    /// `u32`, obfuscated into `+0x5b`. `1402cbc97`.
    pub const HP: u32 = 0x0000_0400;
    /// `u32`, obfuscated into `+0x67`. `1402cbccd`.
    pub const MAX_HP: u32 = 0x0000_0800;
    /// `u32`, obfuscated into `+0x73`. `1402cbd03`.
    pub const MP: u32 = 0x0000_1000;
    /// `u32`, obfuscated into `+0x7f`. `1402cbd39`.
    pub const MAX_MP: u32 = 0x0000_2000;
    /// `u16` -> `+0x8b`. `1402cbd75`.
    pub const AP: u32 = 0x0000_4000;
    /// **Job-dependent width** - see [`super::stat_changed`]. `1402cbd8f` onward.
    pub const SP: u32 = 0x0000_8000;
    /// **`u64`** -> `+0x9b`. `1402cbed8`, the only `u64` read before bit 18.
    pub const EXP: u32 = 0x0001_0000;
    /// `u32`, obfuscated into `+0xb3`. `1402cbefb`.
    pub const FAME: u32 = 0x0002_0000;
    /// **`u64`** -> `+0xbf`. `1402cbf3f`. The one field with no character-record
    /// equivalent, which is why it is the last.
    pub const MESO: u32 = 0x0004_0000;

    /// Every bit the client reads. Anything outside this is dropped on the floor: the
    /// decoder returns straight after bit 18 (`1402cbf59 mov eax,ebp / … / ret`). **[L]**
    pub const ALL_READ: u32 = SKIN
        | FACE
        | HAIR
        | LEVEL
        | JOB
        | STR
        | DEX
        | INT
        | LUK
        | HP
        | MAX_HP
        | MP
        | MAX_MP
        | AP
        | SP
        | EXP
        | FAME
        | MESO;
}

/// One `0x007C` StatChanged. Set only the fields that changed; the mask follows.
///
/// **Several stats travel in one packet.** The decoder is one straight run of bit tests in
/// ascending order over one `u32` mask, so a level-up sending `level`, `max_hp`, `max_mp`,
/// `ap` and `sp` together is exactly what the client expects - it does not need five
/// packets. **[L]**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StatChange {
    /// Release the client's one-request-outstanding latch.
    ///
    /// Non-zero makes the handler call `FUN_142cc4430(ctx, 0)`, which is literally
    /// `[ctx+0x2330] = 0` plus a tick stamp at `+0x2334`. That latch is the mechanism
    /// `STATUS.md` §2d describes: **37** functions set it, **7** clear it, and every later
    /// request in the class is silently dropped until one of the 7 runs. Set this on any
    /// stat change that answers a request the player made. **[L]**
    pub excl_request: bool,
    /// The second header `u8`. **`false` is the ordinary case.**
    ///
    /// `142d549be` uses it to pick which mask bits get an effect - `test edi,0x40030`
    /// (level | job | meso) when it is zero, `test dil,0x30` (level | job) when it is not -
    /// and `142d56242` only fires the meso-gain effect `FUN_142d9bae0` when it is zero. So
    /// a non-zero byte **suppresses** the meso effect. **[L]** for the branches.
    pub quiet: bool,
    /// Skin, bit [`stat::SKIN`]. Costs a `u8` **and** a `u32`.
    pub skin: Option<u8>,
    /// Face, bit [`stat::FACE`].
    pub face: Option<u32>,
    /// Hair, bit [`stat::HAIR`].
    pub hair: Option<u32>,
    /// Level, bit [`stat::LEVEL`].
    ///
    /// **The client notices its own level-up from this field.** `142d54827` deobfuscates
    /// the stored level *before* the mask decode, `142d549b2` again *after*, and
    /// `142d549c4 cmp eax,[rbp+0x250] / jle` takes a branch only when the new one is
    /// greater. Whether that alone plays the animation is **not** established. **[L]** for
    /// the compare.
    pub level: Option<u32>,
    /// Job and subJob, bit [`stat::JOB`]. One bit, **two** `u16`s on the wire.
    pub job: Option<(u16, u16)>,
    /// STR, bit [`stat::STR`].
    pub strength: Option<u16>,
    /// DEX, bit [`stat::DEX`].
    pub dexterity: Option<u16>,
    /// INT, bit [`stat::INT`].
    pub intelligence: Option<u16>,
    /// LUK, bit [`stat::LUK`].
    pub luck: Option<u16>,
    /// Current HP, bit [`stat::HP`]. **This is the field that moves the HP bar** when a mob
    /// hurts the player.
    pub hp: Option<u32>,
    /// Max HP, bit [`stat::MAX_HP`].
    pub max_hp: Option<u32>,
    /// Current MP, bit [`stat::MP`].
    pub mp: Option<u32>,
    /// Max MP, bit [`stat::MAX_MP`].
    pub max_mp: Option<u32>,
    /// Ability points, bit [`stat::AP`].
    pub ap: Option<u16>,
    /// Skill points, bit [`stat::SP`]. **The width depends on the job** - see
    /// [`stat_changed`], and set [`Self::job_for_sp`] whenever this is used.
    pub sp: Option<SpChange>,
    /// **This is the field that moves the EXP bar.** Bit [`stat::EXP`], a `u64`.
    pub exp: Option<u64>,
    /// Fame / popularity, bit [`stat::FAME`].
    pub fame: Option<u32>,
    /// Meso, bit [`stat::MESO`], a `u64`.
    pub meso: Option<u64>,
    /// The job the **client currently has stored**, which is what decides the SP encoding.
    ///
    /// `1402cbd99` reads the job back out of the client's own object (`+0x33`) and hands it
    /// to `FUN_1403024c0` - the same predicate the character record uses,
    /// [`crate::opcode::uses_extended_sp`]. It does **not** use a job sent in this packet,
    /// so if a packet changes both job and SP the client decodes SP with the **old** job.
    /// **[L]**
    pub job_for_sp: u16,
}

/// How skill points are encoded, chosen by the job the client already holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpChange {
    /// The non-extended form: a plain `u16` -> `+0x93`. `1402cbeb5`.
    Plain(u16),
    /// The extended form: `u8 count`, then `count x (u8 jobLevel, u32 sp)`, summed into
    /// `+0xef`. `1402cbdd1` reads the count, `1402cbdf4` the job level, `1402cbdff` the
    /// amount. Identical to the character record's SP fork. **[L]**
    ///
    /// **Job 0 is extended**, so a Beginner's SP goes out this way.
    Pools(Vec<(u8, u32)>),
}

impl StatChange {
    /// The mask this change will send. Public so a caller can assert on it.
    pub fn mask(&self) -> u32 {
        let mut m = 0;
        if self.skin.is_some() {
            m |= stat::SKIN;
        }
        if self.face.is_some() {
            m |= stat::FACE;
        }
        if self.hair.is_some() {
            m |= stat::HAIR;
        }
        if self.level.is_some() {
            m |= stat::LEVEL;
        }
        if self.job.is_some() {
            m |= stat::JOB;
        }
        if self.strength.is_some() {
            m |= stat::STR;
        }
        if self.dexterity.is_some() {
            m |= stat::DEX;
        }
        if self.intelligence.is_some() {
            m |= stat::INT;
        }
        if self.luck.is_some() {
            m |= stat::LUK;
        }
        if self.hp.is_some() {
            m |= stat::HP;
        }
        if self.max_hp.is_some() {
            m |= stat::MAX_HP;
        }
        if self.mp.is_some() {
            m |= stat::MP;
        }
        if self.max_mp.is_some() {
            m |= stat::MAX_MP;
        }
        if self.ap.is_some() {
            m |= stat::AP;
        }
        if self.sp.is_some() {
            m |= stat::SP;
        }
        if self.exp.is_some() {
            m |= stat::EXP;
        }
        if self.fame.is_some() {
            m |= stat::FAME;
        }
        if self.meso.is_some() {
            m |= stat::MESO;
        }
        m
    }

    /// The EXP bar, and nothing else. `excl_request` is on because gaining EXP is the
    /// answer to something the player did.
    pub fn exp_only(exp: u64) -> Self {
        Self {
            excl_request: true,
            exp: Some(exp),
            ..Self::default()
        }
    }

    /// The HP bar, and nothing else - what a mob hurting the player needs.
    pub fn hp_only(hp: u32) -> Self {
        Self {
            excl_request: true,
            hp: Some(hp),
            ..Self::default()
        }
    }
}

/// The third header `u8`, which goes to `ctx+0x3bbc`.
///
/// A scan of every instruction in `0x142c..0x142f` with that displacement finds **seven
/// writes and no reads**. Four of them - `FUN_142caa4e0` (field entry), `FUN_142ca5c50`,
/// `FUN_142cad420`, `FUN_142d62e30` - store the literal **`1`**; the other three are
/// `0x007C`, `0x007D` and `0x007E` storing whatever arrived. So `1` is the value the client
/// writes for itself, and that is why it is what we send. **[D]**
pub const STAT_CHANGED_BYTE3: u8 = 1;

/// Body of a [`STAT_CHANGED`].
///
/// ```text
/// u8   exclRequest       142d547ad
/// u8   quiet             142d547d4
/// u8   1                 142d547e2   -> ctx+0x3bbc, see STAT_CHANGED_BYTE3
/// u32  mask              1402cbb71
///      ... set bits, in ASCENDING bit order ...
/// u8   0                 142d548aa   optional-follower flag, off
/// u8   0                 142d548d2   optional-follower flag, off
/// ```
///
/// **Nine bytes with an empty mask**, and `tools/reads.py` says `FUN_142d54780` reads
/// nothing else at all - nine direct reads plus the mask decoder, and that is the whole
/// function. **[L]**
///
/// Field order is the bit order. It has to be: the decoder is one straight run of
/// `test`/`bt` in ascending index with no reordering anywhere.
pub fn stat_changed(change: &StatChange) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.bool(change.excl_request);
    w.bool(change.quiet);
    w.u8(STAT_CHANGED_BYTE3);
    w.u32(change.mask());

    // bit 0 - skin: a u8 AND a u32
    if let Some(v) = change.skin {
        w.u8(v);
        w.u32(0);
    }
    if let Some(v) = change.face {
        w.u32(v);
    }
    if let Some(v) = change.hair {
        w.u32(v);
    }
    if let Some(v) = change.level {
        w.u32(v);
    }
    // bit 5 - job AND subJob
    if let Some((job, sub_job)) = change.job {
        w.u16(job);
        w.u16(sub_job);
    }
    if let Some(v) = change.strength {
        w.u16(v);
    }
    if let Some(v) = change.dexterity {
        w.u16(v);
    }
    if let Some(v) = change.intelligence {
        w.u16(v);
    }
    if let Some(v) = change.luck {
        w.u16(v);
    }
    if let Some(v) = change.hp {
        w.u32(v);
    }
    if let Some(v) = change.max_hp {
        w.u32(v);
    }
    if let Some(v) = change.mp {
        w.u32(v);
    }
    if let Some(v) = change.max_mp {
        w.u32(v);
    }
    if let Some(v) = change.ap {
        w.u16(v);
    }
    if let Some(sp) = &change.sp {
        match sp {
            SpChange::Plain(v) => {
                debug_assert!(
                    !crate::opcode::uses_extended_sp(change.job_for_sp),
                    "job {} takes the extended SP branch; a plain u16 desynchronises \
                     everything after it",
                    change.job_for_sp
                );
                w.u16(*v);
            }
            SpChange::Pools(pools) => {
                debug_assert!(
                    crate::opcode::uses_extended_sp(change.job_for_sp),
                    "job {} takes the plain-u16 SP branch; a pool list desynchronises \
                     everything after it",
                    change.job_for_sp
                );
                w.u8(pools.len() as u8);
                for (job_level, amount) in pools {
                    w.u8(*job_level);
                    w.u32(*amount);
                }
            }
        }
    }
    if let Some(v) = change.exp {
        w.u64(v);
    }
    if let Some(v) = change.fame {
        w.u32(v);
    }
    if let Some(v) = change.meso {
        w.u64(v);
    }

    // The two trailing optional-follower flags, both off. Neither is understood:
    // FUN_1428a7f00(global, u8) and FUN_140fd31f0(global, u32, u32, …).
    w.u8(0);
    w.u8(0);
    w.into_vec()
}

// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// The one real attack this project has ever captured: `world.log`, 2026-08-19,
    /// `23:58:33.878`, map 40, standing at `(473, 395)`.
    ///
    /// It is here because it is the only end-to-end evidence for the layout, and because
    /// **its target count is zero** - the fact the whole reply design rests on.
    const CAPTURED_MELEE: &str = concat!(
        "0000000000000000000000000000000001050000009fae3408010400000037ac",
        "ee06d9018b0100000000d9018b01000000000000000000000000000000000000",
        "00000000000000000100000001000000000a0055736572204d656c6565890100",
        "000000000000000000000000000000000000000000000000000080e8da8f00",
    );

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn the_captured_attack_parses_field_for_field() {
        let body = hex(CAPTURED_MELEE);
        assert_eq!(body.len(), CAPTURED_EMPTY_ATTACK_LEN);

        let a = parse_attack(&body).expect("the real capture must parse");
        assert_eq!(a.attack_type, "User Melee");
        assert_eq!(a.tick, 0x06ee_ac37);
        assert_eq!(a.x, 473);
        assert_eq!(a.y, 395);
        assert_eq!(a.target_count, 0, "the capture registered NO mob");
        assert!(a.targets.is_empty());
        assert!(!a.truncated);
    }

    /// The arithmetic that makes 127 come out exactly, spelled out so a future change to
    /// any one constant fails here rather than on the wire.
    /// The field census, and the arithmetic that makes 127 come out exactly. A future
    /// change to any one constant fails here rather than on the wire.
    #[test]
    fn the_captured_length_is_accounted_for_byte_for_byte() {
        // FUN_140f31fe0: 12 u8 (one of them the tail jump), 11 u16, 16 u32, 1 str.
        assert_eq!(ATTACK_HEADER_FIXED, 12 * 1 + 11 * 2 + 16 * 4);
        let header = ATTACK_HEADER_FIXED + 2 + "User Melee".len();
        assert_eq!(header, 110);
        assert_eq!(
            header + ATTACK_TARGETS_HEAD_LEN + ATTACK_TRAILER_LEN,
            CAPTURED_EMPTY_ATTACK_LEN
        );
    }

    /// The 40th field is a tail `jmp` into the `u8` writer. **A 39-field header cannot
    /// account for the captured packet at all**, and that is the discrimination: 127 is
    /// reachable with 40 fields and not with 39.
    #[test]
    fn a_thirty_nine_field_header_cannot_explain_the_capture() {
        let tail = 2 + "User Melee".len() + ATTACK_TARGETS_HEAD_LEN + ATTACK_TRAILER_LEN;
        assert_eq!(ATTACK_HEADER_FIXED + tail, CAPTURED_EMPTY_ATTACK_LEN);
        assert_ne!(
            ATTACK_HEADER_FIXED - 1 + tail,
            CAPTURED_EMPTY_ATTACK_LEN,
            "dropping the tail-jump field must NOT still add up, or this proves nothing"
        );
    }

    /// The parser must consume the captured body **exactly** - no byte left over and no
    /// byte short. This is the end-to-end check on every skip length in the header.
    #[test]
    fn the_captured_attack_is_consumed_to_the_last_byte() {
        let body = hex(CAPTURED_MELEE);
        let mut r = PacketReader::new(&body);
        r.skip(30).unwrap();
        assert_eq!(r.u32().unwrap(), 0x06ee_ac37);
        r.skip(4).unwrap();
        r.skip(43).unwrap();
        assert_eq!(r.str().unwrap(), "User Melee");
        r.skip(17).unwrap();
        assert_eq!(r.position(), 110, "the header is 110 bytes for this string");
        assert_eq!(r.u32().unwrap(), 0, "the target count");
        r.skip(8).unwrap();
        r.skip(ATTACK_TRAILER_LEN).unwrap();
        assert_eq!(r.remaining(), 0);
    }

    /// A hand-built attack that connects, to exercise the target block. Not a capture: the
    /// layout is from the listing and has never been on the wire.
    #[test]
    fn a_target_with_two_hits_round_trips() {
        let mut b = Vec::new();
        b.extend(std::iter::repeat_n(0u8, 30));
        b.extend_from_slice(&1234u32.to_le_bytes()); // tick
        b.extend_from_slice(&100u16.to_le_bytes()); // x
        b.extend_from_slice(&200u16.to_le_bytes()); // y
        b.extend(std::iter::repeat_n(0u8, 43));
        b.extend_from_slice(&10u16.to_le_bytes());
        b.extend_from_slice(b"User Melee");
        b.extend(std::iter::repeat_n(0u8, 17));
        // target list head
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        // one target
        b.extend_from_slice(&2000u32.to_le_bytes());
        b.extend_from_slice(&7u32.to_le_bytes());
        b.push(2);
        for d in [17u64, 23u64] {
            b.push(1);
            b.push(0);
            b.extend_from_slice(&d.to_le_bytes());
        }
        b.extend(std::iter::repeat_n(0u8, 55));
        b.extend_from_slice(&0u16.to_le_bytes()); // no map pairs
        b.push(0); // no sub-object
        b.push(0); // mode 0
        // trailer
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(0);

        let a = parse_attack(&b).unwrap();
        assert_eq!(a.target_count, 1);
        assert!(!a.truncated);
        assert_eq!(a.targets.len(), 1);
        assert_eq!(a.targets[0].object_id, 2000);
        assert_eq!(a.targets[0].second, 7);
        assert_eq!(a.targets[0].hits.len(), 2);
        assert_eq!(a.targets[0].total_damage(), 40);
        assert!(a.targets[0].hits[0].flag_a);
        assert!(!a.targets[0].hits[0].flag_b);
    }

    /// The `u16` before the tree walk is a **count of pairs**, not a flag. If it were read
    /// as a flag the eight bytes after it would be taken as part of the next field and the
    /// parse would go off the rails.
    #[test]
    fn the_map_pairs_are_counted_not_flagged() {
        let mut b = Vec::new();
        b.extend(std::iter::repeat_n(0u8, 30));
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes());
        b.extend(std::iter::repeat_n(0u8, 43));
        b.extend_from_slice(&4u16.to_le_bytes());
        b.extend_from_slice(b"Melee");
        b.truncate(b.len() - 1); // "Mele", length 4
        b.extend(std::iter::repeat_n(0u8, 17));
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&2001u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(0); // no hits
        b.extend(std::iter::repeat_n(0u8, 55));
        b.extend_from_slice(&3u16.to_le_bytes()); // THREE pairs
        b.extend(std::iter::repeat_n(0xAAu8, 24));
        b.push(0);
        b.push(0);
        b.extend_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
        b.push(0x5A);

        let a = parse_attack(&b).unwrap();
        assert_eq!(a.attack_type, "Mele");
        assert!(!a.truncated, "three pairs must be skipped as 24 bytes");
        assert_eq!(a.targets[0].object_id, 2001);
    }

    /// A target that sets the un-decoded sub-object flag stops the parse instead of
    /// guessing at a length - and says so.
    #[test]
    fn an_undecodable_target_truncates_rather_than_desynchronising() {
        let mut b = Vec::new();
        b.extend(std::iter::repeat_n(0u8, 30));
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes());
        b.extend(std::iter::repeat_n(0u8, 43));
        b.extend_from_slice(&0u16.to_le_bytes()); // empty type string
        b.extend(std::iter::repeat_n(0u8, 17));
        b.extend_from_slice(&2u32.to_le_bytes()); // TWO targets
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&2002u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(1);
        b.push(0);
        b.push(0);
        b.extend_from_slice(&99u64.to_le_bytes());
        b.extend(std::iter::repeat_n(0u8, 55));
        b.extend_from_slice(&0u16.to_le_bytes());
        b.push(1); // the sub-object we cannot skip

        let a = parse_attack(&b).unwrap();
        assert!(a.truncated);
        assert_eq!(a.target_count, 2);
        assert_eq!(a.targets.len(), 1, "the damage we did read is still usable");
        assert_eq!(a.targets[0].total_damage(), 99);
    }

    /// Always answer: a body that is nonsense must not make the caller panic. It may error,
    /// but the caller still owes the client a reply.
    #[test]
    fn a_short_body_errors_instead_of_panicking() {
        for n in 0..CAPTURED_EMPTY_ATTACK_LEN {
            let body = vec![0u8; n];
            let _ = parse_attack(&body);
        }
    }

    #[test]
    fn only_the_three_shared_body_opcodes_are_attacks() {
        assert!(is_attack_opcode(USER_MELEE_ATTACK));
        assert!(is_attack_opcode(USER_SHOOT_ATTACK));
        assert!(is_attack_opcode(USER_MAGIC_ATTACK));
        assert!(
            !is_attack_opcode(USER_BODY_ATTACK),
            "0x00E2's only builder does not use FUN_140f31fe0, so its body is unknown"
        );
    }

    // -- the death packet ------------------------------------------------------------

    #[test]
    fn the_death_pair_table_matches_the_image() {
        // 0x1402b3ba4, twelve dwords, resolved against the two return arms.
        for t in [0u8, 1, 4, 6, 7, 8, 9, 10, 11] {
            assert!(death_type_carries_pair(t), "type {t} should carry the pair");
        }
        for t in [2u8, 3, 5, 12, 13, 255] {
            assert!(!death_type_carries_pair(t), "type {t} should not");
        }
    }

    #[test]
    fn an_animated_death_is_fourteen_bytes() {
        let b = mob_leave_field(2000, death::ANIMATED);
        assert_eq!(b.len(), 14);
        assert_eq!(&b[0..4], &2000u32.to_le_bytes());
        assert_eq!(b[4], 1);
        assert_eq!(b[5], 0);
        assert_eq!(&b[6..14], &[0u8; 8]);
    }

    #[test]
    fn a_type_that_carries_no_pair_is_six_bytes() {
        assert_eq!(mob_leave_field(2000, 2).len(), 6);
        assert_eq!(mob_leave_field(2000, 3).len(), 6);
        assert_eq!(mob_leave_field(2000, 5).len(), 6);
    }

    #[test]
    fn an_immediate_removal_still_carries_the_pair() {
        assert_eq!(mob_leave_field(2000, death::IMMEDIATE).len(), 14);
    }

    // -- StatChanged -----------------------------------------------------------------

    #[test]
    fn an_empty_stat_change_is_nine_bytes() {
        let b = stat_changed(&StatChange::default());
        assert_eq!(b.len(), 9);
        assert_eq!(b[0], 0);
        assert_eq!(b[1], 0);
        assert_eq!(b[2], STAT_CHANGED_BYTE3);
        assert_eq!(&b[3..7], &0u32.to_le_bytes());
        assert_eq!(&b[7..9], &[0, 0]);
    }

    #[test]
    fn exp_is_bit_sixteen_and_eight_bytes_wide() {
        let c = StatChange::exp_only(1_234_567);
        assert_eq!(c.mask(), stat::EXP);
        let b = stat_changed(&c);
        assert_eq!(b.len(), 9 + 8);
        assert_eq!(b[0], 1, "gaining EXP answers a request, so excl is set");
        assert_eq!(&b[3..7], &0x0001_0000u32.to_le_bytes());
        assert_eq!(&b[7..15], &1_234_567u64.to_le_bytes());
    }

    #[test]
    fn hp_is_bit_ten_and_four_bytes_wide() {
        let b = stat_changed(&StatChange::hp_only(42));
        assert_eq!(b.len(), 9 + 4);
        assert_eq!(&b[3..7], &0x0000_0400u32.to_le_bytes());
        assert_eq!(&b[7..11], &42u32.to_le_bytes());
    }

    /// Several stats in one packet, in ascending bit order. This is the shape a level-up
    /// needs, and the order is what the decoder's straight run of `bt`s requires.
    #[test]
    fn a_level_up_shaped_change_is_one_packet_in_bit_order() {
        let c = StatChange {
            excl_request: true,
            level: Some(11),
            max_hp: Some(120),
            max_mp: Some(60),
            ap: Some(5),
            sp: Some(SpChange::Pools(vec![(1, 3)])),
            job_for_sp: 0, // Beginner: extended
            ..StatChange::default()
        };
        assert_eq!(
            c.mask(),
            stat::LEVEL | stat::MAX_HP | stat::MAX_MP | stat::AP | stat::SP
        );

        let b = stat_changed(&c);
        let mut r = PacketReader::new(&b);
        assert!(r.bool().unwrap());
        assert!(!r.bool().unwrap());
        assert_eq!(r.u8().unwrap(), STAT_CHANGED_BYTE3);
        assert_eq!(r.u32().unwrap(), c.mask());
        assert_eq!(r.u32().unwrap(), 11); // level      bit 4
        assert_eq!(r.u32().unwrap(), 120); // maxHp     bit 11
        assert_eq!(r.u32().unwrap(), 60); // maxMp      bit 13
        assert_eq!(r.u16().unwrap(), 5); // ap          bit 14
        assert_eq!(r.u8().unwrap(), 1); // sp pool count bit 15
        assert_eq!(r.u8().unwrap(), 1); // job level
        assert_eq!(r.u32().unwrap(), 3); // sp
        assert_eq!(r.u8().unwrap(), 0);
        assert_eq!(r.u8().unwrap(), 0);
        assert_eq!(r.remaining(), 0);
    }

    /// Skin is one bit and two fields; job is one bit and two fields. Both are easy to get
    /// wrong and both desynchronise everything after them.
    #[test]
    fn the_two_bits_that_carry_two_fields_each() {
        let skin = stat_changed(&StatChange {
            skin: Some(3),
            ..StatChange::default()
        });
        assert_eq!(skin.len(), 9 + 1 + 4);

        let job = stat_changed(&StatChange {
            job: Some((100, 7)),
            ..StatChange::default()
        });
        assert_eq!(job.len(), 9 + 2 + 2);
        assert_eq!(&job[7..9], &100u16.to_le_bytes());
        assert_eq!(&job[9..11], &7u16.to_le_bytes());
    }

    /// The non-extended SP branch. **Most real jobs are extended in this client** - job 110
    /// (Fighter) is, because `uses_extended_sp` matches branch `10..=12` - so the plain
    /// `u16` is the rarer path and it needs a job that genuinely takes it.
    #[test]
    fn a_plain_sp_job_writes_a_u16() {
        assert!(
            crate::opcode::uses_extended_sp(110),
            "if job 110 ever stops being extended, this test is testing nothing"
        );
        let c = StatChange {
            sp: Some(SpChange::Plain(9)),
            job_for_sp: 900, // GM: job/100 == 9 falls to the predicate's `_ => false`
            ..StatChange::default()
        };
        assert!(!crate::opcode::uses_extended_sp(c.job_for_sp));
        assert_eq!(stat_changed(&c).len(), 9 + 2);
    }

    /// Bit 3 exists in the mask space and the client never reads it, so nothing here may
    /// ever set it - a bit that is set but not read shifts every later field.
    #[test]
    fn nothing_can_set_the_bit_the_client_does_not_read() {
        assert_eq!(stat::ALL_READ & stat::UNREAD_BIT_3, 0);
        let every = StatChange {
            skin: Some(0),
            face: Some(0),
            hair: Some(0),
            level: Some(0),
            job: Some((0, 0)),
            strength: Some(0),
            dexterity: Some(0),
            intelligence: Some(0),
            luck: Some(0),
            hp: Some(0),
            max_hp: Some(0),
            mp: Some(0),
            max_mp: Some(0),
            ap: Some(0),
            sp: Some(SpChange::Pools(Vec::new())),
            exp: Some(0),
            fame: Some(0),
            meso: Some(0),
            job_for_sp: 0,
            ..StatChange::default()
        };
        assert_eq!(every.mask(), stat::ALL_READ);
        assert_eq!(every.mask() & stat::UNREAD_BIT_3, 0);
    }

    /// The whole mask, byte-counted, so any width error shows up as a length.
    #[test]
    fn every_readable_bit_has_the_width_the_decoder_expects() {
        let every = StatChange {
            skin: Some(1),
            face: Some(2),
            hair: Some(3),
            level: Some(4),
            job: Some((5, 6)),
            strength: Some(7),
            dexterity: Some(8),
            intelligence: Some(9),
            luck: Some(10),
            hp: Some(11),
            max_hp: Some(12),
            mp: Some(13),
            max_mp: Some(14),
            ap: Some(15),
            sp: Some(SpChange::Pools(vec![(1, 16)])),
            exp: Some(17),
            fame: Some(18),
            meso: Some(19),
            job_for_sp: 0,
            ..StatChange::default()
        };
        // skin 1+4, face 4, hair 4, level 4, job 2+2, str/dex/int/luk 2 each,
        // hp/maxHp/mp/maxMp 4 each, ap 2, sp 1+(1+4), exp 8, fame 4, meso 8
        let fields = 5 + 4 + 4 + 4 + 4 + 8 + 16 + 2 + 6 + 8 + 4 + 8;
        assert_eq!(stat_changed(&every).len(), 9 + fields);
    }
}
