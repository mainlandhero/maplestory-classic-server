//! Telling the client its stats changed: EXP, level, HP/MP, AP/SP, meso.
//!
//! Every address, every read site, and the "what I did NOT establish" list is in
//! `research/level-up.md`. Read that before changing a field here.
//!
//! # NOT WIRED
//!
//! Nothing in `crates/world/` sends any of this. It compiles, it is tested, and on screen
//! it is identical to not existing - the state `CLAUDE.md` § "Built is not wired" exists to
//! stop being mistaken for working. `research/level-up.md` §8 says how to connect it.
//!
//! # The three packets
//!
//! | | | |
//! |---|---|---|
//! | every stat change, EXP included | outbound [`STAT_CHANGED`] `0x007C` | **read off the client** |
//! | the local level-up animation | **no packet** - see [`the_level_up_animation_is_client_side`] | **read off the client** |
//! | the effect other players see | outbound [`USER_EFFECT_REMOTE`] `0x02AF` | **read off the client** |
//!
//! # Two rules that decide whether the body parses at all
//!
//! The body has **no length prefix and no resync point**, exactly like the character
//! record. One wrong width desynchronises everything after it, silently.
//!
//! 1. **The mask is a `u32`.** `FUN_1402cbb50` reads it with `0x1406e8c20`, the 4-byte
//!    primitive, at `0x1402cbb71`. The v214 reference encodes an 8-byte mask here; copying
//!    that would push every value four bytes late. **[L]**
//! 2. **Values go out in ascending bit order**, because the client tests the bits in that
//!    order and reads as it goes. [`StatChange::build`] is written so the mask and the
//!    values come from one walk over the same list, so they cannot drift apart.

use crate::opcode::uses_extended_sp;
use crate::PacketWriter;

/// **`0x007C` StatChanged** - the one packet that carries level, HP/MP, AP/SP, EXP and meso.
///
/// Routed by `CWvsContext::OnPacket` (`FUN_142cbaa80`) through its dense 4-byte-RVA jump
/// table at `0x142cbd9d0`, indexed by `opcode - 0x70`: entry 12 is the stub at
/// `0x142cbab4f`, which is `call 0x142d54780`. **[L]**
///
/// Three checks, because a handler identification made from decompiled *text* has been
/// wrong here before:
///
/// * the whole 811-entry table was enumerated rather than sampled - 273 distinct stubs, one
///   of which (`0x142cbbc10`) fills 538 slots and is the `default:` arm. `0x142cbab4f` is
///   reached from **exactly one** opcode;
/// * the same enumeration puts `0x0070` on `FUN_142d51930`, the `InventoryOperation`
///   handler this project has confirmed on screen - the positive control;
/// * a `.text`-wide scan for **both** `E8 rel32` and `E9 rel32` finds one `call` into
///   `0x142d54780` and **no tail `jmp`**, and the call is at `0x142cbab55` inside
///   `FUN_142cbaa80`. `tools/callers.py` sees `E8` only, and asking it the same question
///   about the mask decoder gave a wrong answer - `research/level-up.md` §3.
///
/// None of the three can see an *indirect* call, and `research/level-up.md` §3 shows the
/// control where that matters.
///
/// And the handler corroborates itself: before it decodes anything it snapshots
/// `record+0x27`, `+0x5b`, `+0x73` and `+0x9b` - the fields `research/charstat-layout.md`
/// calls level, hp, mp and exp - so it can compare them afterwards. **[L]**
pub const STAT_CHANGED: u16 = 0x007C;

/// **`0x02D1` UserEffectLocal** - `u8 effect`, and nothing else for [`EFFECT_LEVEL_UP`].
///
/// `CField::OnPacket` hands `0x2C5..0x39E` to `FUN_14289a3a0(ctx->localUser, op, pkt)`,
/// which indexes the table at `0x14289d660` by `opcode - 0x2c5`. Index `0xC` is the stub at
/// `0x14289a439`, `call 0x1427863f0` - the effect handler, whose first read is a `u8`. **[L]**
///
/// **Probably redundant.** The `0x007C` handler already plays the level-up effect itself;
/// see [`the_level_up_animation_is_client_side`]. Whether sending both plays it twice is
/// **not established**, so this is here to be tried as a *separate* variant, never together
/// with the first `0x007C` test.
pub const USER_EFFECT_LOCAL: u16 = 0x02D1;

/// **`0x02AF` UserEffectRemote** - `u32 charId, u8 effect`. What *other* players see.
///
/// `FUN_1429b9300` hands `0x293..0x2C4` to `FUN_1429bb720`, which reads a **`u32` character
/// id** at `0x1429bb745` (`tools/reads.py 0x1429bb720 1` shows that is its only direct read),
/// looks the user up in the pool, then indexes the table at `0x1429bbc34` by
/// `opcode - 0x29e`. Index `0x11` is the stub at `0x1429bba36`, `call 0x1427863f0`. **[L]**
///
/// It is dropped in silence if `charId` is not a user currently in the pool.
pub const USER_EFFECT_REMOTE: u16 = 0x02AF;

/// Effect **0**, the level-up animation.
///
/// `FUN_1427863f0`'s second switch (`0x14278bd8d`, table `0x142791348`, `effect <= 0x54`)
/// sends index `0` to the arm at `0x14278bd99`, which loads `[0x143a46f48]` - a qword
/// holding `0x1432ae4a0`, the UTF-16 string `Effect/BasicEff.img/LevelUp`, dereferenced out
/// of the PE. That arm reads **nothing** from the packet. **[L]**
pub const EFFECT_LEVEL_UP: u8 = 0;

/// Which mask bit is which stat.
///
/// The bit *order* and every width are read off `FUN_1402cbb50` - see the table in
/// `research/level-up.md` §3, which enumerates **every** conditional bit test in that
/// function in address order rather than looking for the ones it expected. The *names* of
/// [`STR`], [`DEX`], [`INT`], [`LUK`], [`MAX_HP`], [`MAX_MP`], [`AP`], [`FAME`] and [`MESO`]
/// come from the v214 reference plus the record offsets they land in, and are **[I]**.
pub mod bits {
    /// `u8 skin`, then a `u32` - two values, one bit. The `u32` lands at `record+0x1b`, the
    /// slot the character record's read #7 fills with a constant zero. **[L]**
    pub const SKIN: u32 = 0x0000_0001;
    /// `u32` -> `record+0x1f`.
    pub const FACE: u32 = 0x0000_0002;
    /// `u32` -> `record+0x23`.
    pub const HAIR: u32 = 0x0000_0004;
    /// **Bit 3 is not decoded by this client.** `FUN_1402cbb50` has no test for it: the
    /// instruction after bit 2's block is `test bpl, 0x10`. Setting it does nothing and
    /// sending a value for it desynchronises the body. **[L]**
    pub const NOT_DECODED_BIT_3: u32 = 0x0000_0008;
    /// `u32` -> obfuscated `record+0x27`. **Raising this is what plays the level-up
    /// animation** - see [`super::the_level_up_animation_is_client_side`].
    pub const LEVEL: u32 = 0x0000_0010;
    /// `u16 job`, then `u16 subJob` - two values, one bit. **[L]**
    pub const JOB: u32 = 0x0000_0020;
    /// `u16` -> obfuscated `record+0x3b`.
    pub const STR: u32 = 0x0000_0040;
    /// `u16` -> obfuscated `record+0x43`.
    pub const DEX: u32 = 0x0000_0080;
    /// `u16` -> obfuscated `record+0x4b`.
    pub const INT: u32 = 0x0000_0100;
    /// `u16` -> obfuscated `record+0x53`.
    pub const LUK: u32 = 0x0000_0200;
    /// `u32` -> obfuscated `record+0x5b`.
    pub const HP: u32 = 0x0000_0400;
    /// `u32` -> obfuscated `record+0x67`.
    pub const MAX_HP: u32 = 0x0000_0800;
    /// `u32` -> obfuscated `record+0x73`.
    pub const MP: u32 = 0x0000_1000;
    /// `u32` -> obfuscated `record+0x7f`.
    pub const MAX_MP: u32 = 0x0000_2000;
    /// `u16` -> obfuscated `record+0x8b`.
    pub const AP: u32 = 0x0000_4000;
    /// Either a `u16` or a counted table - see [`super::Sp`]. **[L]**
    pub const SP: u32 = 0x0000_8000;
    /// `u64` -> obfuscated `record+0x9b`. The **new total**, not a delta.
    pub const EXP: u32 = 0x0001_0000;
    /// `u32` -> obfuscated `record+0xb3`.
    pub const FAME: u32 = 0x0002_0000;
    /// `u64` -> obfuscated `record+0xbf`.
    ///
    /// `record+0xbf` appears nowhere in the `SetField` stat block's 29 reads, so this bit is
    /// currently the **only** way to give the client a meso balance. **[L]** for the
    /// absence, **[I]** for the name.
    pub const MESO: u32 = 0x0004_0000;

    /// Every bit `FUN_1402cbb50` actually tests: 0-2 and 4-18. Bit 3 and everything from
    /// bit 19 up are absent from the function entirely. **[L]**
    pub const DECODED: u32 = 0x0007_FFF7;
}

/// Skill points, in whichever of the two encodings the client's fork will read.
///
/// # The fork, and the trap in it
///
/// `FUN_1402cbb50` does **not** just read a `u16` for [`bits::SP`]. It reads the **job back
/// out of the character record** (`lea rcx,[rbx+0x33]`, the obfuscated getter, at
/// `0x1402cbd9c`) and calls `FUN_1403024c0`, whose decoded bit masks are exactly
/// [`uses_extended_sp`] - job `0`, `job-100`/`job-200` against `0x1c0701c01`,
/// `job-300`/`400`/`500` against `0x701c01`, plus `430..=439`. **[L]**
///
/// > **[bits::JOB] is processed earlier in the same packet.** So a `0x007C` carrying both
/// > job and sp must encode the sp for the **new** job. [`Sp::matches_job`] is the check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sp {
    /// A single `u16`, for jobs where [`uses_extended_sp`] is false. Read at `0x1402cbeb5`
    /// into obfuscated `record+0x93`. **[L]**
    Plain(u16),
    /// `u8 count`, then `count` x ([`SpPool`]), for jobs where [`uses_extended_sp`] is true,
    /// which includes job `0` - every character this project has. The client sums the
    /// amounts into `record+0xef`. **[L]**
    Extended(Vec<SpPool>),
}

impl Sp {
    /// The empty table - one zero byte and nothing else.
    ///
    /// This is the only extended encoding that has ever left this server:
    /// `net::opcode::character_record` writes exactly this in the stat block, and a
    /// character built that way has entered the world. Prefer it until §7 item 5 of
    /// `research/level-up.md` is closed.
    pub fn empty_extended() -> Self {
        Sp::Extended(Vec::new())
    }

    /// Whether this encoding is the one the client will read for `job`.
    ///
    /// `job` must be the job the record holds **after** this packet's [`bits::JOB`] value is
    /// applied, not before.
    pub fn matches_job(&self, job: u16) -> bool {
        matches!(self, Sp::Extended(_)) == uses_extended_sp(job)
    }

    fn write(&self, w: &mut PacketWriter) {
        match self {
            Sp::Plain(v) => {
                w.u16(*v);
            }
            Sp::Extended(pools) => {
                // `u8 count` - the client reads it at 0x1402cbdd1 and loops that many times.
                w.u8(u8::try_from(pools.len()).unwrap_or(u8::MAX));
                for p in pools.iter().take(usize::from(u8::MAX)) {
                    w.u8(p.job_level);
                    w.u32(p.amount);
                }
            }
        }
    }
}

/// One entry of an [`Sp::Extended`] table: `u8`, then `u32`.
///
/// The `u32` is the amount - `FUN_1402cbb50` sums the `u32`s into the record's SP total at
/// `+0xef`, which is what identifies which of the two is the amount. **[L]**
///
/// **What [`SpPool::job_level`] must contain is NOT established.** `charstat-layout.md` calls
/// it a job level; nothing in this client was read to confirm what value it expects. It
/// matters the first time the server awards SP to a beginner, because job `0` takes the
/// extended branch. **[I]**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpPool {
    /// **Meaning not established.** See the type's own note.
    pub job_level: u8,
    /// Skill points in this pool.
    pub amount: u32,
}

/// A [`STAT_CHANGED`] body under construction: set the fields that changed, leave the rest
/// `None`.
///
/// The mask and the values are produced by one walk over the same ordered list in
/// [`StatChange::build`], so a field cannot be announced in the mask and then not written,
/// or written in the wrong place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatChange {
    /// Byte 1. Non-zero makes the client call `FUN_142cc4430(ctx, 0)`, which is
    /// `[ctx+0x2330] = 0; [ctx+0x2334] = tick` - **clearing the one-request-outstanding
    /// latch** that `research/npc-click.md` §4.3 identified. `FUN_142cc52a0`, one of the
    /// clearers that file names, is byte-for-byte the same shape with a literal zero.
    ///
    /// Defaults to `true`, because clearing a latch that is not set is harmless and leaving
    /// a set one alone silently drops every later request in its class. **[L]** shape,
    /// **[D]** name.
    pub excl_request_sent: bool,
    /// Byte 2. Read into a local and used once, at `0x142d549be`: `0` makes the client test
    /// `mask & 0x40030` (level, job, exp, meso) for one UI refresh, non-zero narrows that to
    /// `mask & 0x30`. v214 sends `0` and so does this. **[L]** the two tests, **[I]** the
    /// name and the value.
    pub secondary: u8,
    /// Byte 3. Stored at `[ctx+0x3bbc]`. Ten functions write that field and three of them
    /// write the literal `1`, one being `FUN_142caa4e0` - the field-entry routine. v214
    /// sends `1` and so does this. **[L]** destination, **[I]** meaning.
    pub context_flag: u8,

    /// [`bits::SKIN`] - `(skin, the u32 that follows it)`.
    pub skin: Option<(u8, u32)>,
    /// [`bits::FACE`].
    pub face: Option<u32>,
    /// [`bits::HAIR`].
    pub hair: Option<u32>,
    /// [`bits::LEVEL`]. Setting this to a value **higher** than the client's current level
    /// is what plays the level-up animation - [`the_level_up_animation_is_client_side`].
    pub level: Option<u32>,
    /// [`bits::JOB`] - `(job, subJob)`. Both are written; the second lands plainly at
    /// `record+0x10c`.
    pub job: Option<(u16, u16)>,
    /// [`bits::STR`].
    pub strength: Option<u16>,
    /// [`bits::DEX`].
    pub dexterity: Option<u16>,
    /// [`bits::INT`].
    pub intelligence: Option<u16>,
    /// [`bits::LUK`].
    pub luck: Option<u16>,
    /// [`bits::HP`].
    pub hp: Option<u32>,
    /// [`bits::MAX_HP`].
    pub max_hp: Option<u32>,
    /// [`bits::MP`].
    pub mp: Option<u32>,
    /// [`bits::MAX_MP`].
    pub max_mp: Option<u32>,
    /// [`bits::AP`].
    pub ap: Option<u16>,
    /// [`bits::SP`] - read [`Sp`]'s note about the fork before setting this beside
    /// [`StatChange::job`].
    pub sp: Option<Sp>,
    /// [`bits::EXP`] - the character's **new total**, not the amount gained. The client
    /// subtracts its own snapshot to draw the "+N EXP" indicator; see
    /// [`the_client_computes_the_exp_gain_itself`].
    pub exp: Option<u64>,
    /// [`bits::FAME`].
    pub fame: Option<u32>,
    /// [`bits::MESO`] - the new balance.
    pub meso: Option<u64>,

    /// The first optional trailer: `u8 flag`, then `u8` when set. v214 calls it `charmOld`.
    /// **[L]** shape, **[I]** name.
    pub charm: Option<u8>,
    /// The second optional trailer: `u8 flag`, then `u32 hpRecovery, u32 mpRecovery` when
    /// set. The client hands both to `FUN_140fd31f0` alongside its snapshots of
    /// `record+0x5b` and `+0x73`, which is the argument that those two are hp and mp rather
    /// than maxHp and maxMp. **[L]** shape, **[D]** the pairing.
    pub recovery: Option<(u32, u32)>,
}

impl Default for StatChange {
    /// The head bytes v214 sends - `1, 0, 1` - and no stats.
    fn default() -> Self {
        StatChange {
            excl_request_sent: true,
            secondary: 0,
            context_flag: 1,
            skin: None,
            face: None,
            hair: None,
            level: None,
            job: None,
            strength: None,
            dexterity: None,
            intelligence: None,
            luck: None,
            hp: None,
            max_hp: None,
            mp: None,
            max_mp: None,
            ap: None,
            sp: None,
            exp: None,
            fame: None,
            meso: None,
            charm: None,
            recovery: None,
        }
    }
}

impl StatChange {
    /// An empty change with the default head bytes.
    pub fn new() -> Self {
        Self::default()
    }

    /// "You gained EXP" - the smallest useful [`STAT_CHANGED`], **17 body bytes**.
    ///
    /// `new_total` is the character's whole EXP, not the award.
    pub fn exp(new_total: u64) -> Self {
        StatChange { exp: Some(new_total), ..Self::default() }
    }

    /// **The reply a `0x00E5` user-hit report needs** - the new HP and nothing else,
    /// **13 body bytes**. `research/user-hit.md` is the decode of the report itself.
    ///
    /// `new_hp` is the character's HP **after** the damage, not the delta: the client
    /// stores whatever arrives straight into `charstat+0x5b` and never subtracts.
    ///
    /// This is required, not cosmetic. The client **does not** decrement its own HP when it
    /// sends `0x00E5`; measured in `research/user-hit.md` §4 by enumerating every write to
    /// `charstat+0x5b` in the image and showing that none is reachable from the hit path.
    /// Until a `0x007C` arrives the bar does not move, which is exactly what the owner saw.
    ///
    /// Sending `0` is **not** a death packet on its own. The `0x007C` handler's HP arm
    /// (`0x142d5617c`) only gates one UI write on `hp > 0`; it plays no death sequence.
    /// See `research/user-hit.md` §6 for where that investigation stopped.
    pub fn hp_only(new_hp: u32) -> Self {
        StatChange { hp: Some(new_hp), ..Self::default() }
    }

    /// Which mask bits this change will announce.
    ///
    /// Never sets [`bits::NOT_DECODED_BIT_3`] and never sets anything above bit 18, because
    /// there is no field that could.
    pub fn mask(&self) -> u32 {
        let mut m = 0u32;
        if self.skin.is_some() {
            m |= bits::SKIN;
        }
        if self.face.is_some() {
            m |= bits::FACE;
        }
        if self.hair.is_some() {
            m |= bits::HAIR;
        }
        if self.level.is_some() {
            m |= bits::LEVEL;
        }
        if self.job.is_some() {
            m |= bits::JOB;
        }
        if self.strength.is_some() {
            m |= bits::STR;
        }
        if self.dexterity.is_some() {
            m |= bits::DEX;
        }
        if self.intelligence.is_some() {
            m |= bits::INT;
        }
        if self.luck.is_some() {
            m |= bits::LUK;
        }
        if self.hp.is_some() {
            m |= bits::HP;
        }
        if self.max_hp.is_some() {
            m |= bits::MAX_HP;
        }
        if self.mp.is_some() {
            m |= bits::MP;
        }
        if self.max_mp.is_some() {
            m |= bits::MAX_MP;
        }
        if self.ap.is_some() {
            m |= bits::AP;
        }
        if self.sp.is_some() {
            m |= bits::SP;
        }
        if self.exp.is_some() {
            m |= bits::EXP;
        }
        if self.fame.is_some() {
            m |= bits::FAME;
        }
        if self.meso.is_some() {
            m |= bits::MESO;
        }
        m
    }

    /// Build the [`STAT_CHANGED`] body - opcode not included, per the house style.
    ///
    /// ```text
    /// u8   bExclRequestSent
    /// u8   secondary
    /// u8   contextFlag
    /// u32  mask                    <- FOUR bytes, not eight; see the module note
    ///      values in ascending bit order
    /// u8   hasCharm     [u8]
    /// u8   hasRecovery  [u32 u32]
    /// ```
    pub fn build(&self) -> Vec<u8> {
        let mut w = PacketWriter::new();
        w.bool(self.excl_request_sent);
        w.u8(self.secondary);
        w.u8(self.context_flag);
        w.u32(self.mask());

        // Ascending bit order, and the ONLY place the order is written down. Keep this list
        // in the same sequence as `mask()` above and as the table in research/level-up.md.
        if let Some((skin, extra)) = self.skin {
            w.u8(skin);
            w.u32(extra);
        }
        if let Some(v) = self.face {
            w.u32(v);
        }
        if let Some(v) = self.hair {
            w.u32(v);
        }
        if let Some(v) = self.level {
            w.u32(v);
        }
        if let Some((job, sub_job)) = self.job {
            w.u16(job);
            w.u16(sub_job);
        }
        if let Some(v) = self.strength {
            w.u16(v);
        }
        if let Some(v) = self.dexterity {
            w.u16(v);
        }
        if let Some(v) = self.intelligence {
            w.u16(v);
        }
        if let Some(v) = self.luck {
            w.u16(v);
        }
        if let Some(v) = self.hp {
            w.u32(v);
        }
        if let Some(v) = self.max_hp {
            w.u32(v);
        }
        if let Some(v) = self.mp {
            w.u32(v);
        }
        if let Some(v) = self.max_mp {
            w.u32(v);
        }
        if let Some(v) = self.ap {
            w.u16(v);
        }
        if let Some(sp) = &self.sp {
            sp.write(&mut w);
        }
        if let Some(v) = self.exp {
            w.u64(v);
        }
        if let Some(v) = self.fame {
            w.u32(v);
        }
        if let Some(v) = self.meso {
            w.u64(v);
        }

        match self.charm {
            Some(v) => {
                w.u8(1);
                w.u8(v);
            }
            None => {
                w.u8(0);
            }
        }
        match self.recovery {
            Some((hp, mp)) => {
                w.u8(1);
                w.u32(hp);
                w.u32(mp);
            }
            None => {
                w.u8(0);
            }
        }
        w.into_vec()
    }
}

/// Build a [`USER_EFFECT_LOCAL`] body. One byte.
pub fn user_effect_local(effect: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(effect);
    w.into_vec()
}

/// Build a [`USER_EFFECT_REMOTE`] body - what every *other* client on the field is sent.
pub fn user_effect_remote(char_id: u32, effect: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(char_id);
    w.u8(effect);
    w.into_vec()
}

/// Documentation only: **the server does not send a level-up effect to the player who
/// levelled.**
///
/// `FUN_142d54780` re-reads the level out of the character-stat record after the mask block
/// has run and compares it with the snapshot it took before:
///
/// ```text
/// 142d54ae2  mov  edx,[r13 + 0x2f]
/// 142d54ae6  mov  rcx,r15                   ; r15 = record + 0x27, the level
/// 142d54ae9  call 0x1401ba9d0               ; newLevel
/// 142d54aee  cmp  eax,[rbp + 0x250]         ; the pre-decode snapshot
/// 142d54af4  jle  142d55b52                 ; not higher -> nothing happens
/// ...
/// 142d54bdb  mov  rdx,qword [rip + 0xcf2366]   ; [0x143a46f48] -> L"Effect/BasicEff.img/LevelUp"
/// 142d54be6  call 0x140dc12e0
/// ```
///
/// `tools/dataref.py 0x143a46f48` finds exactly two readers of that pointer: this one, and
/// the packet-driven effect handler behind [`USER_EFFECT_LOCAL`]. **[L]**
///
/// So a single [`STAT_CHANGED`] carrying a higher [`bits::LEVEL`] plays the animation. What
/// is **not** established is whether also sending [`USER_EFFECT_LOCAL`] plays it twice.
pub fn the_level_up_animation_is_client_side() {}

/// Documentation only: **the server never sends an EXP delta or an EXP message.**
///
/// The `0x007C` handler snapshots `record+0x9b` before decoding and, when [`bits::EXP`] is
/// set, computes the difference itself - adding `FUN_14087ec50(oldLevel)`, the EXP-to-next-
/// level lookup, when the level also changed - and hands it to the on-screen indicator at
/// `0x140fd3110`. **[L]**
///
/// That is also what names bit 16 `exp` from a *consumer* rather than from the reference.
///
/// `FUN_14087ec50`'s table is at `0x143AC2400`, 121 `u64` entries indexed by level 1..=120.
/// It **cannot be read off the disk**: that address is in `.data`'s uninitialised tail
/// (`vsize 0xa2aa8`, `rsize 0x67400`), so it is zero in the file and filled at run time.
/// `research/level-up.md` §7 item 6 records the trap that produced 120 confident wrong
/// numbers from it.
pub fn the_client_computes_the_exp_gain_itself() {}

#[cfg(test)]
mod tests {
    use super::*;

    /// The head is three bytes and the mask is **four**. v214 puts an 8-byte mask here; that
    /// would push every value four bytes late in a body with no resync point.
    #[test]
    fn an_exp_only_change_is_seventeen_bytes_and_byte_exact() {
        let b = StatChange::exp(0x0102_0304_0506_0708).build();
        assert_eq!(
            b,
            vec![
                1, // bExclRequestSent
                0, // secondary
                1, // contextFlag
                0x00, 0x00, 0x01, 0x00, // mask = 0x10000, four bytes
                0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // u64 exp
                0, // no charm
                0, // no recovery
            ]
        );
        assert_eq!(b.len(), 17);
    }

    /// An empty change is still a well-formed packet: head, a zero mask, two zero trailers.
    #[test]
    fn an_empty_change_still_frames() {
        let b = StatChange::new().build();
        assert_eq!(b, vec![1, 0, 1, 0, 0, 0, 0, 0, 0]);
    }

    fn mask_of(b: &[u8]) -> u32 {
        u32::from_le_bytes([b[3], b[4], b[5], b[6]])
    }

    #[test]
    fn every_field_announces_exactly_its_own_bit() {
        let mut sc = StatChange::new();
        sc.skin = Some((1, 0));
        assert_eq!(mask_of(&sc.build()), bits::SKIN);

        for (set, want) in [
            (Box::new(|s: &mut StatChange| s.face = Some(1)) as Box<dyn Fn(&mut StatChange)>, bits::FACE),
            (Box::new(|s: &mut StatChange| s.hair = Some(1)), bits::HAIR),
            (Box::new(|s: &mut StatChange| s.level = Some(1)), bits::LEVEL),
            (Box::new(|s: &mut StatChange| s.job = Some((1, 0))), bits::JOB),
            (Box::new(|s: &mut StatChange| s.strength = Some(1)), bits::STR),
            (Box::new(|s: &mut StatChange| s.dexterity = Some(1)), bits::DEX),
            (Box::new(|s: &mut StatChange| s.intelligence = Some(1)), bits::INT),
            (Box::new(|s: &mut StatChange| s.luck = Some(1)), bits::LUK),
            (Box::new(|s: &mut StatChange| s.hp = Some(1)), bits::HP),
            (Box::new(|s: &mut StatChange| s.max_hp = Some(1)), bits::MAX_HP),
            (Box::new(|s: &mut StatChange| s.mp = Some(1)), bits::MP),
            (Box::new(|s: &mut StatChange| s.max_mp = Some(1)), bits::MAX_MP),
            (Box::new(|s: &mut StatChange| s.ap = Some(1)), bits::AP),
            (Box::new(|s: &mut StatChange| s.sp = Some(Sp::Plain(1))), bits::SP),
            (Box::new(|s: &mut StatChange| s.exp = Some(1)), bits::EXP),
            (Box::new(|s: &mut StatChange| s.fame = Some(1)), bits::FAME),
            (Box::new(|s: &mut StatChange| s.meso = Some(1)), bits::MESO),
        ] {
            let mut s = StatChange::new();
            set(&mut s);
            assert_eq!(mask_of(&s.build()), want, "one field must set exactly one bit");
        }
    }

    /// Bit 3 has no test in `FUN_1402cbb50`, and neither has anything above bit 18. Nothing
    /// this builder can be asked to do may put a value there.
    #[test]
    fn the_mask_never_leaves_the_bits_the_client_decodes() {
        let everything = StatChange {
            skin: Some((1, 0)),
            face: Some(1),
            hair: Some(1),
            level: Some(1),
            job: Some((1, 2)),
            strength: Some(1),
            dexterity: Some(1),
            intelligence: Some(1),
            luck: Some(1),
            hp: Some(1),
            max_hp: Some(1),
            mp: Some(1),
            max_mp: Some(1),
            ap: Some(1),
            sp: Some(Sp::Plain(1)),
            exp: Some(1),
            fame: Some(1),
            meso: Some(1),
            charm: Some(1),
            recovery: Some((1, 2)),
            ..StatChange::new()
        };
        let m = everything.mask();
        assert_eq!(m & bits::NOT_DECODED_BIT_3, 0, "bit 3 is not decoded by this client");
        assert_eq!(m & !bits::DECODED, 0, "nothing above bit 18 is decoded either");
        assert_eq!(m, bits::DECODED, "and every bit that IS decoded has a field");
    }

    /// The client tests the bits in ascending order and reads as it goes, so the values must
    /// be in that order. This pins the whole sequence with distinguishable widths.
    #[test]
    fn values_follow_the_mask_in_ascending_bit_order() {
        let sc = StatChange {
            level: Some(0xAAAA_AAAA),   // bit 4,  u32
            job: Some((0xBBBB, 0xCCCC)), // bit 5,  u16 + u16
            hp: Some(0xDDDD_DDDD),      // bit 10, u32
            ap: Some(0xEEEE),           // bit 14, u16
            exp: Some(0xF0F0_F0F0_F0F0_F0F0), // bit 16, u64
            meso: Some(0x1111_2222_3333_4444), // bit 18, u64
            ..StatChange::new()
        };
        let b = sc.build();
        let v = &b[7..b.len() - 2];
        let mut want = Vec::new();
        want.extend_from_slice(&0xAAAA_AAAAu32.to_le_bytes());
        want.extend_from_slice(&0xBBBBu16.to_le_bytes());
        want.extend_from_slice(&0xCCCCu16.to_le_bytes());
        want.extend_from_slice(&0xDDDD_DDDDu32.to_le_bytes());
        want.extend_from_slice(&0xEEEEu16.to_le_bytes());
        want.extend_from_slice(&0xF0F0_F0F0_F0F0_F0F0u64.to_le_bytes());
        want.extend_from_slice(&0x1111_2222_3333_4444u64.to_le_bytes());
        assert_eq!(v, &want[..]);
    }

    /// Two bits carry two values each: skin is `u8 + u32`, job is `u16 + u16`.
    #[test]
    fn skin_and_job_each_carry_two_values() {
        let skin = StatChange { skin: Some((7, 0x0102_0304)), ..StatChange::new() }.build();
        assert_eq!(&skin[7..skin.len() - 2], &[7, 0x04, 0x03, 0x02, 0x01]);

        let job = StatChange { job: Some((0x0100, 0x0302)), ..StatChange::new() }.build();
        assert_eq!(&job[7..job.len() - 2], &[0x00, 0x01, 0x02, 0x03]);
    }

    /// `u8 count`, then `count` x (`u8`, `u32`). An empty table is one zero byte.
    #[test]
    fn the_extended_sp_table_is_a_counted_list() {
        let empty = StatChange { sp: Some(Sp::empty_extended()), ..StatChange::new() }.build();
        assert_eq!(&empty[7..empty.len() - 2], &[0]);

        let two = StatChange {
            sp: Some(Sp::Extended(vec![
                SpPool { job_level: 1, amount: 3 },
                SpPool { job_level: 2, amount: 0x0102_0304 },
            ])),
            ..StatChange::new()
        }
        .build();
        assert_eq!(
            &two[7..two.len() - 2],
            &[2, 1, 3, 0, 0, 0, 2, 0x04, 0x03, 0x02, 0x01]
        );

        let plain = StatChange { sp: Some(Sp::Plain(0x0201)), ..StatChange::new() }.build();
        assert_eq!(&plain[7..plain.len() - 2], &[0x01, 0x02]);
    }

    /// The fork reads the job **out of the record**, after this packet's job bit has been
    /// applied. Getting it backwards desynchronises everything after the SP field.
    #[test]
    fn the_sp_encoding_has_to_match_the_job_the_record_will_hold() {
        // Job 0 - every character this project has - takes the extended branch.
        assert!(Sp::empty_extended().matches_job(0));
        assert!(!Sp::Plain(3).matches_job(0));
        // The four first jobs are extended too.
        for job in [100, 200, 300, 400, 500] {
            assert!(Sp::empty_extended().matches_job(job), "job {job}");
        }
        // A job that is not in the masks takes the plain u16.
        assert!(Sp::Plain(3).matches_job(101));
        assert!(!Sp::empty_extended().matches_job(101));
    }

    /// A level-up is one packet. Pinned end to end because it is the packet goal D needs.
    #[test]
    fn a_level_up_is_one_stat_changed() {
        let sc = StatChange {
            level: Some(2),
            hp: Some(70),
            max_hp: Some(70),
            mp: Some(25),
            max_mp: Some(25),
            ap: Some(5),
            sp: Some(Sp::empty_extended()),
            exp: Some(4),
            ..StatChange::new()
        };
        let m = sc.mask();
        assert_eq!(
            m,
            bits::LEVEL | bits::HP | bits::MAX_HP | bits::MP | bits::MAX_MP | bits::AP
                | bits::SP | bits::EXP
        );
        let b = sc.build();
        // 3 head + 4 mask + (4 level + 4+4 hp/maxhp + 4+4 mp/maxmp + 2 ap + 1 sp + 8 exp)
        // + 1 + 1 trailers
        assert_eq!(b.len(), 3 + 4 + 31 + 2);
        assert_eq!(mask_of(&b), m);
        // the level really is the first value, so the client's `newLevel > oldLevel` test
        // sees it
        assert_eq!(u32::from_le_bytes([b[7], b[8], b[9], b[10]]), 2);
    }

    /// The answer to a `0x00E5` user-hit report, pinned byte for byte.
    ///
    /// Head `1, 0, 1`; mask `0x400` = bit 10 = hp, which is the bit the client tests at
    /// `0x142d5617c` (`bt r12d, 0xa`) and the same bit `0x1402cbc8e` tests before it stores
    /// the value at `charstat+0x5b`. Then the u32, then the two absent trailers.
    #[test]
    fn the_reply_to_a_user_hit_is_thirteen_bytes() {
        let b = StatChange::hp_only(44).build();
        assert_eq!(
            b,
            vec![
                1, 0, 1, // head
                0x00, 0x04, 0x00, 0x00, // mask = bits::HP, bit 10
                44, 0, 0, 0, // the new HP, u32 LE
                0, // charm absent
                0, // recovery absent
            ]
        );
        assert_eq!(b.len(), 13);
        assert_eq!(mask_of(&b), bits::HP);
        assert_eq!(bits::HP, 1 << 10);
    }

    /// HP zero is a legal value on the wire and must not be silently dropped by `Option`
    /// handling - the server is the authority on death and `0` is how it says so.
    #[test]
    fn hp_zero_still_sets_the_bit_and_carries_the_value() {
        let b = StatChange::hp_only(0).build();
        assert_eq!(mask_of(&b), bits::HP, "hp: Some(0) must still announce bit 10");
        assert_eq!(u32::from_le_bytes([b[7], b[8], b[9], b[10]]), 0);
        assert_eq!(b.len(), 13);
    }

    /// The two trailers are flag-then-value, and both are normally absent.
    #[test]
    fn the_optional_trailers_are_flag_then_value() {
        let b = StatChange { charm: Some(9), recovery: Some((3, 4)), ..StatChange::new() }.build();
        assert_eq!(&b[7..], &[1, 9, 1, 3, 0, 0, 0, 4, 0, 0, 0]);
    }

    #[test]
    fn the_effect_bodies_are_the_shapes_the_two_dispatchers_read() {
        assert_eq!(user_effect_local(EFFECT_LEVEL_UP), vec![0]);
        // u32 charId first - FUN_1429bb720 reads it before it can even find the user.
        assert_eq!(user_effect_remote(200, EFFECT_LEVEL_UP), vec![200, 0, 0, 0, 0]);
        assert_eq!(EFFECT_LEVEL_UP, 0);
    }

    #[test]
    fn the_opcodes_are_the_ones_read_off_the_jump_tables() {
        assert_eq!(STAT_CHANGED, 0x007C);
        assert_eq!(USER_EFFECT_LOCAL, 0x02D1);
        assert_eq!(USER_EFFECT_REMOTE, 0x02AF);
    }
}
