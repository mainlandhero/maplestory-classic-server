//! Skills on the wire: the presence-gated block in the character record, the packet that
//! changes a skill level, and the request the client sends when the player clicks `+`.
//!
//! Labels are the project's: **[L]** read off this client's listing or a capture, **[D]**
//! derived from two or more [L] facts, **[I]** inferred. Full working, with the instrument
//! controls and the "what I did NOT establish" list, is `research/skills.md`.
//!
//! # What was missing before this file
//!
//! **This server had never sent a single byte of skill data.** The skill window drew whatever
//! the client's own `Skill.wz` says a beginner has, every level read as zero, and a click on
//! `+` produced a `0x013B` that nothing answered.
//!
//! # The three shapes
//!
//! ```text
//! record, presence[8]   u8 bulk, u16 count, { u32 skillId, u32 level, raw8 expires,
//!                                             [u32 masterLevel if needs_master_level] }
//! packet 0x0081         u8 clearLatch, u8 showEffect, u8 (ignored), u16 count,
//!                       count x { u32 skillId, i32 level, u32 masterLevel, raw8 expires },
//!                       u8 tail
//! packet 0x013B (in)    u32 clientTick, u32 skillId, u32 count
//! ```
//!
//! # THE RECORD HAS NO LENGTH PREFIX AND NO RESYNC POINT
//!
//! Same rule as [`crate::quest`]: one wrong width here silently desynchronises every byte
//! after it, and the symptom on screen is an undressed character or no world entry - not a
//! wrong skill list. Two traps in this block specifically:
//!
//! * with `bulk != 0` the client reads the skill list and then **jumps to the end of the
//!   block** (`0x140306e24 JMP 0x1403073c1`), so the six delta lists the zero form needs are
//!   *not* read. [`SKILL_BLOCK_BULK`] is a constant here for that reason.
//! * with `bulk != 0` **and `count == 0`** the client jumps to the end at `0x140306d61`
//!   before reading anything else, so an empty block is exactly three bytes.
//! * `masterLevel` is **conditional in the record and unconditional in `0x0081`**. The
//!   predicate is [`needs_master_level`]; getting it wrong in the record moves every
//!   following byte.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A skill goes up
//! because a packet arrived on the socket.

use crate::error::{NetError, Result};
use crate::packet::{PacketReader, PacketWriter};

// ---------------------------------------------------------------------------------------
// The character-record block
// ---------------------------------------------------------------------------------------

/// The presence byte that switches on the **skill block** - gate entry 17.
///
/// **[L], and confirmed from two independent key tables.** The record decoder's gate is at
/// `0x140306d28` with key `0x143abf280`; the client's own *encoder* (`FUN_1402e5a30`, the
/// mirror function `crate::quest` already uses) gates the same block at `0x1402e6a4c` with
/// key `0x143abdff0`. Both keys have exactly one initialiser and both write byte **+8**:
///
/// ```text
/// 140023710  lea rcx,[0x143abf280] / call 0x140302c70   ; memset(key, 0, 100)
/// 140023722  mov byte ptr [0x143abf288], 1              ; key[8] = 1   <- decoder
/// 140022c20  lea rcx,[0x143abdff0] / call 0x140302c70
/// 140022c32  mov byte ptr [0x143abdff8], 1              ; key[8] = 1   <- encoder
/// ```
///
/// A gate fires iff `any(presence & key)` and every key is all-zero but one byte, so this is
/// one byte and nothing composite. `research/charrecord-presence-map.md` has the method.
///
/// **Why it is skills, not a name taken from a table.** The block's first loop reads
/// `u32 skillId, u32 level`, inserts into `charData+0x1039`, reads `raw8` into
/// `charData+0x10f9`, and then calls `FUN_140302650(skillId)` - a predicate whose literal
/// comparisons are `4340012`, `1120012`, `1320011`, `2121425`, `2320498`, `5120011`,
/// `5220206` and friends, i.e. **MapleStory skill ids**, and which divides its argument by
/// `10000` to get a job. `charData+0x1039` is then read by the skill-window module at
/// `0x14258bc0c`/`0x14258f430`/`0x142592920` (the same code that loads
/// `UI/Skill.img/entry`) and written by the `0x0081` handler. **[L]** the reads, **[D]** the
/// name.
pub const PRESENCE_SKILLS: usize = 8;

/// The `u8` that opens the skill block: **1 = this is the whole list, replace what you have**.
///
/// **[L].** On the decoder side a non-zero value makes the client clear three collections
/// first - `FUN_1402fe4a0` on `charData+0x1039`, `+0x1009` and `+0x1021` at `0x140306d6e`,
/// `0x140306d7a`, `0x140306d86` - and then **skip the six delta lists entirely**
/// (`0x140306e24 JMP 0x1403073c1`). That is what a snapshot wants: a skill the server has
/// dropped stops being shown.
///
/// The zero form is a delta - six `u16`-counted lists in a fixed order (levels changed,
/// levels removed, expirations changed, expirations removed, master levels changed, master
/// levels removed) - and is deliberately not reachable from this module. A record is always
/// a snapshot.
pub const SKILL_BLOCK_BULK: u8 = 1;

/// What an **empty** skill block costs: the bulk `u8` and a zero `u16` count.
///
/// **[L], and it really is three and not more.** `0x140306d5f TEST EDI,EDI /
/// 0x140306d61 JZ 0x1403073c3` leaves the block before the three collection clears and
/// before any entry, and `0x1403073c3` is the next gate.
pub const EMPTY_SKILL_BLOCK_LEN: usize = 3;

/// The most entries the block can carry, because the count is a `u16` read at `0x140306d57`.
///
/// This is the client's limit, not ours. A longer list is truncated to a *self-consistent*
/// block rather than being allowed to write a count that wraps.
pub const MAX_SKILLS_PER_BLOCK: usize = u16::MAX as usize;

/// "This skill never expires", as a Windows FILETIME.
///
/// **[I], and it is the same convention and the same value as
/// [`crate::opcode::ITEM_NEVER_EXPIRES`]** (2079-01-01). Nothing in this client was read to
/// establish what it does with an expired skill; zero is a *valid* FILETIME (1601-01-01), so
/// sending zero is sending "expired four centuries ago" rather than "unset". That is exactly
/// the mistake `npc_enter_field` made with `isEnabled`/`alpha`, so it is not repeated here.
pub const SKILL_NEVER_EXPIRES: u64 = crate::opcode::ITEM_NEVER_EXPIRES;

/// One skill the character knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skill {
    /// `u32` at `0x140306d96` in the record, `0x142d57fe5` in `0x0081`. **[L]**
    pub id: u32,
    /// `u32` at `0x140306da3` in the record. **[L]**
    ///
    /// In `0x0081` the same field is read at `0x142d57ff4` and **treated as signed**
    /// (`TEST R15D,R15D / JNS` at `0x142d5803f`): a negative level *removes* the skill. The
    /// record's bulk path has no such branch, so this stays `u32` here and
    /// [`SkillChange::Forget`] carries the removal instead.
    pub level: u32,
    /// `u32` at `0x140306df9`, **only when [`needs_master_level`] is true**. **[L]**
    pub master_level: u32,
    /// Eight raw bytes at `0x140306dcf`, a FILETIME. See [`SKILL_NEVER_EXPIRES`]. **[L]**
    pub expires_at: u64,
}

impl Skill {
    /// A skill at `level` that never expires and has no master level.
    pub fn at_level(id: u32, level: u32) -> Self {
        Skill { id, level, master_level: 0, expires_at: SKILL_NEVER_EXPIRES }
    }

    /// How many bytes this entry costs **in the character record**.
    pub fn record_len(&self) -> usize {
        4 + 4 + 8 + if needs_master_level(self.id) { 4 } else { 0 }
    }
}

/// Whether the character record carries a `masterLevel` `u32` for this skill.
///
/// This is `FUN_140302650`, and the record's conditional read at `0x140306df9` sits behind
/// `CALL 0x140302650 / TEST EAX,EAX / JZ 0x140306e15`. **Getting it wrong desynchronises the
/// rest of the record**, so the working is spelled out rather than summarised.
///
/// # It is false for every job MapleCW can produce, and that is proved rather than assumed
///
/// The function ends:
///
/// ```text
/// 1403027ee  MOV  ECX,EDI            ; EDI = skillId / 10000, the job
/// 1403027f0  CALL 0x140286e90        ; ESI = f(job)   - the job's tier
/// 1403027f9  CALL 0x140302540        ; AL  = g(job)
/// 140302800  JNE  0x14030284c        ;   g(job) != 0            -> return 0
///            <six literal skill ids> ->                            return 1
/// 140302832  CMP  ESI,4
/// 140302835  JNE  0x14030284c        ;   f(job) != 4            -> return 0
/// 140302837  MOV  EAX,1              ;                             return 1
/// ```
///
/// and `FUN_140302540(job)`, for a job outside its four jump-table sets, is exactly
/// `f(job) == 4` (`1403025ac CALL 0x140286e90 / CMP EAX,4`). So for an ordinary job the two
/// tests are complements: `f(job) == 4` returns 0 at `0x140302800`, and `f(job) != 4` returns
/// 0 at `0x140302835`. **Only the six literal ids can reach `return 1`.** **[D]**
///
/// Job `0` - every character this project has - is provable without knowing `f` at all. With
/// `esi = job = 0`:
///
/// * if `g(0) == 0`, `0x14030279b CMP ESI,ECX / JE` fires on `0 % 1000 == 0` -> **0**;
/// * if `g(0) != 0`, the branch at `0x140302782` skips to `0x1403027b2` and the *second*
///   `g(0)` at `0x1403027f9` returns 0 there too -> **0**.
///
/// Both arms, no unknowns. **[L]**
///
/// # The six
///
/// `0x140302802 .. 0x140302830`, all Dual Blade (job 43x), all **[L]**.
pub fn needs_master_level(skill_id: u32) -> bool {
    matches!(
        skill_id,
        4_311_003 | 4_321_006 | 4_330_009 | 4_331_002 | 4_340_007 | 4_341_004
    )
}

/// The `presence[8]` block: `u8 bulk, u16 count, { u32 id, u32 level, raw8 expires, [u32 ml] }`.
///
/// Gate `0x140306d28`, key `0x143abf280`, region `[0x140306d44, 0x1403073c3)`, 21 reads -
/// loops `#13`..`#19` of `research/charrecord-loops.md` §3, counted independently.
///
/// The client's own encoder writes the same fields in the same order at `0x1402e6a75`
/// (`u8`), `0x1402e6a8e` (`u16`), `0x1402e6b2b`/`0x1402e6b35` (two `u32`), `0x1402e6b95`
/// (`raw 8`) and `0x1402e6bea` (the conditional `u32`, behind the same
/// `CALL 0x140302650` at `0x1402e6b9c`). **[L]**
///
/// ```
/// use net::skills::{skill_block, Skill, EMPTY_SKILL_BLOCK_LEN};
/// assert_eq!(skill_block(&[]), vec![1, 0, 0]);
/// assert_eq!(skill_block(&[]).len(), EMPTY_SKILL_BLOCK_LEN);
/// // Three Snails at level 1: 3 header bytes + 4 + 4 + 8, no master level.
/// assert_eq!(skill_block(&[Skill::at_level(1000, 1)]).len(), 3 + 16);
/// ```
pub fn skill_block(skills: &[Skill]) -> Vec<u8> {
    let n = skills.len().min(MAX_SKILLS_PER_BLOCK);
    let mut w = PacketWriter::new();
    w.u8(SKILL_BLOCK_BULK); //                  140306d47
    w.u16(n as u16); //                         140306d57
    for s in &skills[..n] {
        w.u32(s.id); //                         140306d96
        w.u32(s.level); //                      140306da3
        w.u64(s.expires_at); //                 140306dcf, raw 8
        if needs_master_level(s.id) {
            w.u32(s.master_level); //           140306df9, behind FUN_140302650
        }
    }
    // No delta lists: `TEST AL,AL / JZ 0x140306e29` at 0x140306d4f sends a non-zero bulk
    // flag down the arm that ends with `JMP 0x1403073c1`. Writing the six zero u16 the
    // other arm reads would desynchronise the record.
    w.into_vec()
}

/// How many bytes [`skill_block`] will produce for this list, without building it.
pub fn skill_block_len(skills: &[Skill]) -> usize {
    let n = skills.len().min(MAX_SKILLS_PER_BLOCK);
    EMPTY_SKILL_BLOCK_LEN + skills[..n].iter().map(Skill::record_len).sum::<usize>()
}

// ---------------------------------------------------------------------------------------
// 0x0081 - the skill-record update
// ---------------------------------------------------------------------------------------

/// The packet that changes skill levels outside a field entry.
///
/// **Routing, [L].** `FUN_142cbaa80`, the game-stage dispatcher, has
/// `case 0x81: FUN_142d57f20(this, packet)` - `research/msexe-gamestage-cases.txt` line 10.
/// That handler writes the same three collections the record's `presence[8]` block fills
/// (`charData+0x1039` levels, `+0x10b1` master levels, `+0x10f9` expirations) and reads the
/// skill-window singleton `0x143aca600`, so the identification does not rest on a name
/// table.
///
/// > **It is a silent no-op before the first `SetField`.** `0x142d57f7e` is
/// > `CALL 0x142cbe730 / TEST RAX,RAX / JZ 0x142d58b85` and `FUN_142cbe730` is
/// > `MOV RAX,[RCX+0x2358] / RET` - the character-data object, which `STATUS.md` records as
/// > measuring `0x00` on the first `SetField`. Send this **after** the record, never instead
/// > of it. **[L]**
pub const CHANGE_SKILL_RECORD_RESULT: u16 = 0x0081;

/// What one entry of a [`change_skill_record_result`] says about one skill.
///
/// The level field is read **signed** at `0x142d57ff4` and branched on at `0x142d5803f`
/// (`TEST R15D,R15D / JNS`): the negative arm erases the id from `charData+0x1039` and
/// `+0x1009`, the non-negative arm inserts. Both arms then read `masterLevel` and the
/// 8-byte expiration unconditionally, so **every entry is 20 bytes whichever way it goes**.
/// **[L]**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillChange {
    /// The skill is now at this level, with this master level and expiry.
    Learn(Skill),
    /// Erase the skill. Encoded as a negative level, which is the only thing that reaches
    /// the erase arm.
    Forget { id: u32 },
}

impl SkillChange {
    fn write(&self, w: &mut PacketWriter) {
        match self {
            SkillChange::Learn(s) => {
                w.u32(s.id); //             142d57fe5
                w.i32(i32::try_from(s.level).unwrap_or(i32::MAX)); // 142d57ff4
                w.u32(s.master_level); //   142d58229, unconditional here
                w.u64(s.expires_at); //     142d58539, raw 8
            }
            SkillChange::Forget { id } => {
                w.u32(*id);
                w.i32(-1);
                w.u32(0);
                w.u64(0);
            }
        }
    }
}

/// Bytes per entry of [`change_skill_record_result`]. **[L]**, see [`SkillChange`].
pub const SKILL_CHANGE_ENTRY_LEN: usize = 20;

/// The [`CHANGE_SKILL_RECORD_RESULT`] **body** - no opcode; the caller prepends it, the same
/// way [`crate::quest::quest_record`] is used.
///
/// ```text
/// u8   clear_request_latch   142d57f4a   != 0 -> FUN_142cc4430(user, 0)
/// u8   show_effect           142d57f60   != 0 -> the level-up effect and strings 0xf6e/0xf6f
/// u8   0                     142d57f71   read, and never used - AL is clobbered by the
///                                        CALL 0x142cbe730 four instructions later
/// u16  count                 142d57fad   0 -> straight to the trailing u8
/// count x {
///   u32 skillId              142d57fe5
///   i32 level                142d57ff4   signed; < 0 erases
///   u32 masterLevel          142d58229   UNCONDITIONAL, unlike the record
///   raw8 expires             142d58539
/// }
/// u8   tail                  142d5888d   -> FUN_1428a7f00(uiSingleton, v), always read
/// ```
///
/// # `clear_request_latch` defaults on, and that is the load-bearing byte
///
/// `FUN_142cc4430(user, 0)` sets `user+0x2330 = 0`. The client's skill-up sender
/// (`FUN_142d4bd80`) refuses to send at all while `user+0x2330 != 0`
/// (`FUN_142cc42d0`, `0x142cc42e8 CMP dword [rcx+0x2330],0 / JNE -> return 0`) and sets it
/// to `1` immediately after sending. **So the client sends exactly one `0x013B` per session
/// until a packet clears the latch** - which is precisely what
/// `research/fixtures/skill-window-close-faults-world.log` shows: one `0x013B`, ever.
/// `crate::stats::StatChange::excl_request_sent` is the same byte on `0x007C` and
/// `research/npc-click.md` §4.3 is where the latch was first identified. **[L]**
///
/// ```
/// use net::skills::{change_skill_record_result, Skill, SkillChange};
/// let body = change_skill_record_result(
///     true, true, &[SkillChange::Learn(Skill::at_level(1000, 1))]);
/// assert_eq!(body.len(), 3 + 2 + 20 + 1);
/// assert_eq!(body[0], 1);
/// ```
pub fn change_skill_record_result(
    clear_request_latch: bool,
    show_effect: bool,
    changes: &[SkillChange],
) -> Vec<u8> {
    let n = changes.len().min(u16::MAX as usize);
    let mut w = PacketWriter::new();
    w.bool(clear_request_latch); //  142d57f4a
    w.bool(show_effect); //          142d57f60
    w.u8(0); //                      142d57f71 - read and discarded
    w.u16(n as u16); //              142d57fad
    for c in &changes[..n] {
        c.write(&mut w);
    }
    w.u8(0); //                      142d5888d - the trailing u8, always read
    w.into_vec()
}

// ---------------------------------------------------------------------------------------
// 0x013B - the inbound skill-up request
// ---------------------------------------------------------------------------------------

/// What the client sends when the player clicks `+` on a skill.
///
/// **[L], from two directions.** The builder is `FUN_142d4bd80`:
///
/// ```text
/// 142d4bda8  MOV  EDX,0x1f4 / CALL 0x142cc42d0   ; a 500 ms throttle AND the latch check
/// 142d4bdb6  MOV  EDX,0x13b / CALL 0x1406ed520   ; begin packet 0x013B
/// 142d4bdc6  CALL 0x1429e3ef0 / write u32        ; the client's own tick
/// 142d4bdd7  write u32 EDI                       ; skillId
/// 142d4bde3  write u32 ESI                       ; count
/// 142d4bdf9  MOV  EDX,1 / CALL 0x142cc4430       ; SET the one-request-outstanding latch
/// ```
///
/// and the capture agrees byte for byte -
/// `research/fixtures/skill-window-close-faults-world.log`, 22:17:30.209:
///
/// ```text
/// <- 0x013B, 12 byte body  ac88b80b e8030000 01000000
///                          tick     1000     1
/// ```
///
/// `1000` is Three Snails, which is the skill the owner clicked.
pub const CLIENT_USER_SKILL_UP_REQUEST: u16 = 0x013B;

/// The decoded body of a [`CLIENT_USER_SKILL_UP_REQUEST`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillUpRequest {
    /// The client's own tick at the moment it built the packet. Not a server clock, not
    /// checked here, and it is the client that rate-limits itself with it.
    pub client_tick: u32,
    /// Which skill. **[L]**
    pub skill_id: u32,
    /// How many points to spend. `1` for `BtSpUp`, `min(sp, maxLevel - level)` for
    /// `BtSpUpAll` - the client computes it at `0x142586dcb` and clamps it before sending,
    /// but the server must clamp again: **nothing here authenticates, and nothing stops a
    /// crafted body carrying any number at all.**
    pub count: u32,
}

impl SkillUpRequest {
    /// Parse the body **after** the opcode.
    ///
    /// ```
    /// use net::skills::SkillUpRequest;
    /// let body = hex_body();
    /// let r = SkillUpRequest::parse(&body).unwrap();
    /// assert_eq!((r.skill_id, r.count), (1000, 1));
    /// # fn hex_body() -> Vec<u8> {
    /// #     let mut v = Vec::new();
    /// #     v.extend_from_slice(&0x0bb888acu32.to_le_bytes());
    /// #     v.extend_from_slice(&1000u32.to_le_bytes());
    /// #     v.extend_from_slice(&1u32.to_le_bytes());
    /// #     v
    /// # }
    /// ```
    pub fn parse(body: &[u8]) -> Result<Self> {
        let mut r = PacketReader::new(body);
        let client_tick = r.u32()?;
        let skill_id = r.u32()?;
        let count = r.u32()?;
        Ok(SkillUpRequest { client_tick, skill_id, count })
    }
}

/// The beginner skill ids, so a first character has something to spend a point on.
///
/// **[L] from this client's own `Skill.wz` numbering** - the ids Three Snails / Recovery /
/// Nimble Feet occupy in job `0`, and `1000` is the one the client actually put on the wire
/// in the capture above. The *names* are convention; the id `1000` is measured.
pub const BEGINNER_SKILLS: [u32; 3] = [1000, 1001, 1002];

/// The highest level a beginner skill reaches. **[L]**, out of this client's own `Skill.wz`.
///
/// Not game knowledge and not a guess. `Skill_000.wz`'s `000.img` was read with
/// `target/release/wz-dump cat`, and all three beginner skills agree twice over:
///
/// ```text
/// 0001000  masterLevel=3  level entries=3  highest=3
/// 0001001  masterLevel=3  level entries=3  highest=3
/// 0001002  masterLevel=3  level entries=3  highest=3
/// ```
///
/// The `level` sub-tree having exactly three numbered children is the stronger half: a
/// `masterLevel` field could be a maximum the job never reaches, but a level table cannot
/// describe a level it does not contain. There is **no `maxLevel` key at all** on these
/// three, which is why this is named for what was measured.
///
/// Re-derive it with:
/// `target/release/wz-dump cat "client-patched/Data/Skill/Skill_000.wz" 000.img`
pub const BEGINNER_SKILL_MAX_LEVEL: u32 = 3;

/// The reason a [`SkillUpRequest`] was refused, for a server that wants to say so.
///
/// This type exists because of the **always answer** rule: `0x013B` sets a latch in the
/// client, and a refusal that sends nothing leaves that latch set forever. Answer with
/// [`change_skill_record_result`] carrying `clear_request_latch = true` and no changes even
/// when the answer is "no".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillUpRefusal {
    /// The character has no skill points left.
    NoSkillPoints,
    /// The character does not have access to this skill.
    NotYours,
    /// Already at the maximum level this server allows.
    AtMaxLevel,
    /// `count` was zero, or so large it cannot be honoured.
    BadCount,
}

/// A `0x0081` body that answers a refused [`SkillUpRequest`] without changing anything.
///
/// Clears the latch, shows no effect, carries no entries. **12 bytes** cost against a
/// permanently wedged skill button.
pub fn skill_up_refused(_why: SkillUpRefusal) -> Vec<u8> {
    change_skill_record_result(true, false, &[])
}

/// Reading a `0x013B` body that is not 12 bytes long is not an error to return - it is an
/// error to log. See [`SkillUpRequest::parse`]; this exists so a caller can say why.
pub fn require_exact_len(body: &[u8]) -> Result<()> {
    if body.len() == 12 {
        Ok(())
    } else {
        Err(NetError::PacketUnderflow { offset: body.len(), need: 12, have: body.len() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_block_is_three_bytes_and_they_are_the_ones_the_client_reads() {
        // 140306d47 u8 bulk, 140306d57 u16 count. A zero count jumps to 0x1403073c3 at
        // 140306d61, before the three collection clears, so nothing else is on the wire.
        assert_eq!(skill_block(&[]), vec![SKILL_BLOCK_BULK, 0x00, 0x00]);
        assert_eq!(skill_block(&[]).len(), EMPTY_SKILL_BLOCK_LEN);
    }

    #[test]
    fn one_beginner_skill_is_sixteen_bytes_of_body() {
        let b = skill_block(&[Skill::at_level(1000, 1)]);
        assert_eq!(
            b,
            [
                &[1u8][..],                                 // bulk
                &1u16.to_le_bytes()[..],                    // count
                &1000u32.to_le_bytes()[..],                 // 140306d96 skillId
                &1u32.to_le_bytes()[..],                    // 140306da3 level
                &SKILL_NEVER_EXPIRES.to_le_bytes()[..],     // 140306dcf raw 8
            ]
            .concat()
        );
        assert_eq!(b.len(), EMPTY_SKILL_BLOCK_LEN + 16);
        assert_eq!(b.len(), skill_block_len(&[Skill::at_level(1000, 1)]));
    }

    #[test]
    fn beginner_skills_never_carry_a_master_level() {
        // Proved off the listing in `needs_master_level`'s own documentation: for job 0 both
        // arms of FUN_140302650 return 0. If this ever flips, the record desynchronises.
        for id in BEGINNER_SKILLS {
            assert!(!needs_master_level(id), "skill {id} must not carry a master level");
            assert_eq!(Skill::at_level(id, 1).record_len(), 16);
        }
    }

    #[test]
    fn the_six_dual_blade_ids_do_carry_one() {
        for id in [4_311_003u32, 4_321_006, 4_330_009, 4_331_002, 4_340_007, 4_341_004] {
            assert!(needs_master_level(id));
            assert_eq!(Skill::at_level(id, 1).record_len(), 20);
        }
        // and a neighbour of one of them does not - the predicate is a set, not a range
        assert!(!needs_master_level(4_331_003));
    }

    #[test]
    fn a_master_level_skill_puts_the_extra_u32_last_in_its_entry() {
        let s = Skill { id: 4_331_002, level: 30, master_level: 30, expires_at: 0 };
        let b = skill_block(&[s]);
        assert_eq!(b.len(), EMPTY_SKILL_BLOCK_LEN + 20);
        // 140306df9 reads the master level AFTER the raw 8 at 140306dcf.
        assert_eq!(&b[3..7], &4_331_002u32.to_le_bytes());
        assert_eq!(&b[7..11], &30u32.to_le_bytes());
        assert_eq!(&b[11..19], &0u64.to_le_bytes());
        assert_eq!(&b[19..23], &30u32.to_le_bytes());
    }

    #[test]
    fn the_block_is_truncated_self_consistently() {
        let many = vec![Skill::at_level(1000, 1); MAX_SKILLS_PER_BLOCK + 5];
        let b = skill_block(&many);
        let count = u16::from_le_bytes([b[1], b[2]]) as usize;
        assert_eq!(count, MAX_SKILLS_PER_BLOCK);
        assert_eq!(b.len(), EMPTY_SKILL_BLOCK_LEN + count * 16);
    }

    #[test]
    fn change_result_entries_are_twenty_bytes_whichever_way_they_go() {
        let learn = change_skill_record_result(
            true,
            true,
            &[SkillChange::Learn(Skill::at_level(1000, 2))],
        );
        let forget = change_skill_record_result(true, false, &[SkillChange::Forget { id: 1000 }]);
        assert_eq!(learn.len(), 3 + 2 + SKILL_CHANGE_ENTRY_LEN + 1);
        assert_eq!(forget.len(), learn.len());
        // 142d58229 reads the master level and 142d58539 the expiration on BOTH arms.
        assert_eq!(&learn[0..3], &[1, 1, 0]);
        assert_eq!(&learn[3..5], &1u16.to_le_bytes());
        assert_eq!(&learn[5..9], &1000u32.to_le_bytes());
        assert_eq!(&learn[9..13], &2i32.to_le_bytes());
        // a Forget is a negative level, which is the only thing that reaches the erase arm
        assert_eq!(&forget[9..13], &(-1i32).to_le_bytes());
        assert_eq!(*learn.last().unwrap(), 0, "142d5888d is always read");
    }

    #[test]
    fn an_empty_change_result_is_six_bytes_and_still_clears_the_latch() {
        let b = skill_up_refused(SkillUpRefusal::NoSkillPoints);
        assert_eq!(b, vec![1, 0, 0, 0, 0, 0]);
        assert_eq!(b[0], 1, "142d57f4a: non-zero clears user+0x2330");
        assert_eq!(&b[3..5], &0u16.to_le_bytes(), "142d57fad: no entries");
        assert_eq!(b.len(), 6);
    }

    #[test]
    fn the_captured_skill_up_request_parses_to_three_snails() {
        // research/fixtures/skill-window-close-faults-world.log, 22:17:30.209:
        //   <- 0x013B UNKNOWN, 12 byte body ac88b80be803000001000000
        let body = [
            0xac, 0x88, 0xb8, 0x0b, // client tick
            0xe8, 0x03, 0x00, 0x00, // 1000 - Three Snails
            0x01, 0x00, 0x00, 0x00, // 1 point
        ];
        let r = SkillUpRequest::parse(&body).unwrap();
        assert_eq!(r.client_tick, 0x0bb8_88ac);
        assert_eq!(r.skill_id, 1000);
        assert_eq!(r.count, 1);
        assert_eq!(r.skill_id, BEGINNER_SKILLS[0]);
        require_exact_len(&body).unwrap();
    }

    #[test]
    fn a_short_skill_up_request_is_an_error_and_not_a_panic() {
        assert!(SkillUpRequest::parse(&[0u8; 11]).is_err());
        assert!(require_exact_len(&[0u8; 11]).is_err());
        assert!(require_exact_len(&[0u8; 13]).is_err());
    }

    #[test]
    fn the_presence_byte_is_eight_and_is_not_one_the_server_already_uses() {
        assert_eq!(PRESENCE_SKILLS, 8);
        // The bytes this server sets today, from crate::opcode and crate::quest.
        for other in [0usize, 2, 7, 9, 14] {
            assert_ne!(PRESENCE_SKILLS, other);
        }
    }
}
