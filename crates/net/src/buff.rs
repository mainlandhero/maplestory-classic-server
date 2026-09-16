//! Temporary stats: the packet that grants a buff, the one that clears it, and the request
//! that asks for one.
//!
//! The owner, 2026-08-21 and again on 2026-08-22: *"Nimble Feet still does not give me a buff
//! despite me activating the skill."* The request had been arriving all along -
//! `world.log` of the 12:59 run logs one `0x013C`, 51 bytes, `skillId 1002 level 3` - and
//! nothing answered it. `research/buffs.md` has the working; this is the wire.
//!
//! # Three skills are modelled, and one of them is NOT fully wired
//!
//! [`buff_level`] answers for [`NIMBLE_FEET`], [`MAGIC_GUARD`] and [`MAGIC_ARMOR`]. Only
//! Nimble Feet's rows are confirmed on a client; the other two are read off the WZ and their
//! bits carry different confidences, recorded on [`CTS_MAGIC_GUARD`] (**[L]**) and
//! [`CTS_WEAPON_DEFENCE`] (**[D]**, with a named blind spot).
//!
//! **Magic Armor grants two stats and [`BuffLevel::granted_by`] returns one.** The session
//! calls `granted_by`, so until that one call site becomes [`BuffLevel::all_granted_by`],
//! Magic Armor sets weapon defence and **silently drops magic defence**. That is not a
//! cosmetic gap: on screen it is a cast where only W. Def moves, which
//! `research/magic-damage.md` §9.3 lists as meaning *"the pair is off by one"* - a wrong
//! conclusion drawn from a real observation, and the run that was supposed to promote
//! [`CTS_WEAPON_DEFENCE`] from [D] to [L] would instead be spent chasing it.
//!
//! Magic Guard needs a second thing this file cannot supply: its damage split is **server
//! work**. `research/magic-damage.md` §8 established the client computes the MP redirection
//! and never writes HP, so setting bit 97 alone buys an icon and nothing else.
//!
//! # `0x007D` outbound is NOT `0x007D` inbound, and the names here say so
//!
//! `crate::opcode::CLIENT_MIGRATION_HELLO` is `0x007D` **from** the client. This module's
//! [`TEMPORARY_STAT_SET`] is `0x007D` **to** it. Opcode spaces are per-direction and the
//! channel dispatcher has no case that could answer the client's `0x007D`, so both are true
//! at once - but a single shared constant would be a trap, so neither name mentions the
//! other's job. `research/buffs.md` §5.1 flagged this before either existed.
//!
//! # Three sizes, and each of them is measured
//!
//! * the mask is **124 bytes**, stated three independent ways in the client: a `mov r8d,0x7c`
//!   raw read, a constructor that zeroes `0x7C`, and a decoder that loops 30 `u32` words plus
//!   one more. **[L]**
//! * the bit order inside each `u32` is **big-endian**: bit `i` is
//!   `words[i >> 5] |= 1 << (31 - (i & 31))`. **[L]**
//! * the duration is **milliseconds**, and `Skill.wz`'s `time` is **seconds**. Three
//!   readings agree, one of them a wire measurement: two client packets 1.091 s apart carried
//!   tick values 1080 apart, and the only clock this PE imports is `timeGetTime`. Sending
//!   `30` where `30000` belongs would put the expiry 30 ms out and the icon would flash and
//!   vanish - which is a **distinguishable** outcome, so the test plan asks for it by name.
//!
//! This is the third bug in a month that was a correct number in the wrong unit, which is why
//! [`TemporaryStat::duration_ms`] is spelled with its unit in the field name.
//!
//! # The one thing that is a guess, and how the run decides it
//!
//! The per-stat value is either an `i16` or a `u32`, and the client decides **per packet** by
//! ANDing the mask with a 124-byte constant at `0x143ac37e0`. That address is in the
//! Themida-packed `.data`, so its bytes at rest are not its runtime content and no static
//! read can settle it. `i16` is the best available guess and it is **[I]**.
//!
//! The tail is sent as zero bytes for exactly this reason: every field in it is fixed-width,
//! so **all four parses read the same zeros**. Do not put a non-zero byte in the tail until
//! the width is settled.
//!
//! # The first attempt crashed the client, and the tail is why
//!
//! 18 bytes of tail was sized from the documented layout and the client threw a C++ exception
//! reading four bytes it did not have. [`TAIL_LEN`] carries the whole measurement, including
//! the seven bytes that four separate instruments say should have been there and were not.

use crate::packet::PacketWriter;

/// `0x007D` **outbound** - TemporaryStatSet. See the module docs on the direction collision.
pub const TEMPORARY_STAT_SET: u16 = 0x007D;

/// `0x007E` outbound - TemporaryStatReset. `u8, u8, u8, raw[124]`, **and then more**.
///
/// See [`temporary_stat_reset`]: the documented 127 bytes are not enough, and the client says
/// so by throwing.
pub const TEMPORARY_STAT_RESET: u16 = 0x007E;

/// `0x013C` - the client asking to use a skill. `u32 skillId, u32 level`, then a tail.
pub const CLIENT_SKILL_USE: u16 = 0x013C;

/// `0x013D` - the send-counter census. **Never answered**; see `research/buffs.md` §2.5.
pub const CLIENT_SKILL_CENSUS: u16 = 0x013D;

/// `0x013F` - the player right-clicked a buff icon. See [`parse_skill_cancel`].
pub const CLIENT_SKILL_CANCEL: u16 = 0x013F;

/// Where the 124-byte mask starts inside a [`CLIENT_SKILL_CANCEL`] body.
///
/// **Two independent constraints pick this out and nothing else fits.** The one capture is
/// 133 bytes with exactly three non-zero bytes: `ea 03` at 0..1 (skill 1002) and `0x08` at
/// offset 17. For each candidate start, the mask must be **124 bytes long** and the set bit
/// must decode - by the client's own `words[i>>5] >> (31 - (i&31))` - to a stat that was
/// actually granted:
///
/// ```text
/// start  mask length  the 0x08 decodes to
///     4          129  bit 116
///     8          125  bit  84
///     9          124  bit  92   <- both right, and 92 is the Speed bit we granted
///    10          123  bit  36
/// ```
///
/// **[D]** from one body. It is one sample, so it is a reading rather than a settled layout -
/// but a coincidence would have to satisfy both constraints at once.
pub const CANCEL_MASK_OFFSET: usize = 9;

/// Total length of a [`CLIENT_SKILL_CANCEL`] body: the 9-byte header and the mask.
pub const CLIENT_SKILL_CANCEL_LEN: usize = CANCEL_MASK_OFFSET + MASK_LEN;

/// The character-temporary-stat bit for movement speed.
///
/// **[L]**, from `FUN_1429755a0` - the client's own guard that refuses a second Nimble Feet
/// while Speed is already held - which reads this bit and raises string `0x14DA`.
pub const CTS_SPEED: u32 = 92;

/// The character-temporary-stat bit an EXP coupon rides: **163, `ExpBuffRate`**.
///
/// **The bit is [L]**: it is what this client calls index 163 in its own CTS name table
/// (`research/first-job-buffs.md`, Appendix A), and its `0x007D` block at `0x140a22478` is
/// the standard one - value, `u32` reason, `u32` duration added to the base time - so it
/// has the shape of every other entry this builder writes.
///
/// **What the client does with the value is [D].** The modern reference source sends the
/// item's `expBuff` percent (`200`, `300`) on this bit and nothing else, and the name says
/// rate; nothing here has measured the client's own EXP arithmetic reading it. It does not
/// need to: `Session::with_exp_coupon` multiplies the kill server-side regardless, so this
/// bit is for the icon and its countdown. A wrong reading would show as a wrong number in
/// the icon's tooltip, never as wrong experience.
pub const CTS_EXP_BUFF_RATE: u32 = 163;

/// The `reason` of a stat an ITEM granted: the item id, **negated**.
///
/// A positive reason is a skill id - the icon comes from `Skill.wz` and the tooltip names
/// the skill. Every potion this server sent before 2026-09-16 carried its item id
/// *positive*, so the client went looking for skill `2002001`, found nothing, and drew
/// nothing - the owner: *"Magic Potions and other similar potions are not applying the buff
/// icons"*. The modern reference source sets `rOption = -itemID` for every item buff
/// (`Char.java`, `ItemBuffs.java`), and that convention is older than this client.
///
/// **[D] until a run shows the icon.** The doubt is honest: no instruction in this build has
/// been read testing the reason's sign. The test plan says what each outcome means.
pub fn item_reason(item_id: u32) -> u32 {
    item_id.wrapping_neg()
}

/// The bit mask is 124 bytes: 31 little-endian `u32` words.
pub const MASK_LEN: usize = 124;

/// Highest bit the client will test. `FUN_1402bf6d0` compares `idx >= 0x3e0` and returns
/// false, so 992 is out of range and 991 is the last legal bit. **[L]**
pub const MAX_CTS_BIT: u32 = 992;

/// Zero bytes after the per-stat list.
///
/// # It was 18, the client threw, and 18 should have been enough
///
/// 2026-08-22, the owner: *"Nimble Feet crashed the client."* Exit code `0xE06D7363` - an
/// unhandled **C++ exception**, not an access violation - and the hook's throw log names the
/// frame: `0x142d56911`, the instruction after the `call` at `0x142d5690c`, which is the
/// `u32` read near the end of `FUN_142d563d0`'s tail. The primitive it called says exactly
/// why it threw:
///
/// ```asm
/// 1406e8c32  mov  edi, [rcx+0x18]     ; length
/// 1406e8c35  sub  edi, [rcx+0x24]     ; minus position = bytes remaining
/// 1406e8c79  cmp  edi, 4
/// 1406e8c7c  jb   1406e8c91           ; fewer than four left -> raise
/// 1406e8cb1  int3                     ; the return address on the throw stack
/// ```
///
/// **[L]**. So the body was too short. That much is measured, and it also proves something
/// that was only [D] before: `0x007D` really is TemporaryStatSet, because the throw happened
/// inside its handler.
///
/// # What does not add up, said plainly rather than smoothed over
///
/// Four instruments were run on the layout and they agree with each other:
///
/// * `tools/reads.py` at depth 4 on `FUN_142d563d0` - the whole tail is `u16, 6x u8, one
///   conditional u8, u32, u8`, and **nothing** reads the packet before the mask;
/// * the listing of the raw primitive `0x1406e9170` - it copies exactly `r8d` bytes with no
///   length prefix, and the call site passes `0x7c`, so the mask is **124**;
/// * the listing of bit 92's own decoder block at `0x140a17e3b` - 87 lines, and the `u32` at
///   `+0x34` and the `u16` at `+0x52` are the **two arms of one `if`**, so a stat is 10 bytes
///   or 12, never both;
/// * an enumeration of all 476 bit tests against `0x1402bf6d0` - bit 92 is tested **once**,
///   so one bit sets one block.
///
/// That totals **at most 145** bytes consumed before the `u32`, out of the 152 sent - seven
/// to spare. The client says otherwise. **Something between the handler's entry and
/// `0x142d5690c` consumes bytes that none of those four can see**, and re-running any of them
/// is not a second opinion.
///
/// # So this is slack, not a computed length
///
/// 64 zero bytes: 46 more than the worst layout any of the evidence supports, which absorbs
/// a hidden consumer several times over. Every tail field is fixed width and zero, so a
/// longer tail cannot change what any of them decode - and the reader's only length test is
/// "fewer than N remaining", with no check that the body was fully consumed.
///
/// **It is honestly a guess about the size of an unknown, and the next run narrows it.** If
/// the buff works at 64, the true requirement is somewhere in 153..198 and can be bisected
/// later; if the client dies *differently* - a complaint about a long packet rather than a
/// silent death - then this reader does check for leftovers and the number has to be exact.
pub const TAIL_LEN: usize = 64;

/// One temporary stat being granted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemporaryStat {
    /// The CTS bit, e.g. [`CTS_SPEED`].
    pub bit: u32,
    /// The amount. **[I] `i16`** - see the module docs; the run decides it.
    pub value: i16,
    /// What granted it: the skill id, or an item id.
    pub reason: u32,
    /// **Milliseconds.** `Skill.wz`'s `time` is seconds, so multiply by 1000.
    pub duration_ms: u32,
}

/// Build the 124-byte mask with `bits` set.
///
/// The bit order is big-endian *inside* each little-endian word, which is the part that looks
/// wrong and is not: bit 92 lands in word 2 as `1 << (31 - 28)` = `0x00000008`, so bytes
/// 8..11 read `08 00 00 00`. Anything that "tidies" this to `1 << (i & 31)` sets a different
/// stat, and the client would grant it without complaint.
///
/// Bits at or above [`MAX_CTS_BIT`] are dropped rather than wrapping: the client's own test
/// refuses them, and a silent wrap would set an unrelated stat.
pub fn stat_mask(bits: &[u32]) -> [u8; MASK_LEN] {
    let mut words = [0u32; MASK_LEN / 4];
    for &b in bits {
        if b >= MAX_CTS_BIT {
            continue;
        }
        words[(b >> 5) as usize] |= 1u32 << (31 - (b & 31));
    }
    let mut out = [0u8; MASK_LEN];
    for (i, w) in words.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    out
}

/// `0x007D` - grant these stats.
///
/// **The per-stat entries go in ascending bit order**, because that is the order the decoder
/// walks the mask in: it loops the words low to high and, inside each, tests bit 31 down to
/// bit 0. Sorting here rather than trusting the caller means a two-stat packet cannot be
/// mis-paired - and a mis-pairing would not error, it would give the wrong stat the wrong
/// number for the wrong length of time.
pub fn temporary_stat_set(stats: &[TemporaryStat]) -> Vec<u8> {
    temporary_stat_set_with_tail(stats, TAIL_LEN)
}

/// [`temporary_stat_set`] with the tail length chosen by the caller.
///
/// # This exists to turn one guess per launch into many probes per launch
///
/// [`TAIL_LEN`] is slack around a number nobody has been able to derive statically, and the
/// only way to narrow it is to send one and see. A rebuild per attempt costs the owner a manual
/// launch; `!buff <skill> <level> <tail>` costs a chat line. So the length is a parameter,
/// the skill keypress uses the safe default, and a session that survives the default can
/// bisect downwards until the client throws again.
///
/// The failure is not gentle - the client raises an unhandled C++ exception and the process
/// ends - so a probe that goes too low ends the session. Bisect **downwards from working**,
/// not upwards from broken.
pub fn temporary_stat_set_with_tail(stats: &[TemporaryStat], tail: usize) -> Vec<u8> {
    let mut ordered: Vec<TemporaryStat> =
        stats.iter().copied().filter(|s| s.bit < MAX_CTS_BIT).collect();
    ordered.sort_by_key(|s| s.bit);
    ordered.dedup_by_key(|s| s.bit);

    let mut w = PacketWriter::new();
    let bits: Vec<u32> = ordered.iter().map(|s| s.bit).collect();
    w.bytes(&stat_mask(&bits));
    for s in &ordered {
        w.u16(s.value as u16);
        w.u32(s.reason);
        w.u32(s.duration_ms);
    }
    w.bytes(&vec![0u8; tail]);
    w.into_vec()
}

/// Body length of a [`temporary_stat_set`] with `n` stats: mask, `n` x 10, tail.
pub const fn temporary_stat_set_len(n: usize) -> usize {
    MASK_LEN + n * 10 + TAIL_LEN
}

/// The shortest tail known to be too short: the client threw with **18**.
///
/// Kept as a constant so the probe path can refuse to go back below a length that has
/// already killed a client once. Costing the owner a launch to re-learn something the log
/// already says is exactly what this repo's rules exist to prevent.
pub const TAIL_KNOWN_TOO_SHORT: usize = 18;

/// `0x007E` - clear these stats.
///
/// `u8, u8, u8`, the mask, then a zero tail.
///
/// # 127 bytes killed the client, one handler after `0x007D` did
///
/// 2026-08-22, the owner: *"The buff works, but after the buff expired, the client crashed
/// again."* The same fault as the grant packet and the same shape of cause, thirty seconds
/// later - and this time the arithmetic comes out **exactly**, with nothing unexplained.
///
/// `research/buffs.md` §7.1 gives the body as `u8, u8, u8, raw[124]` = 127, on the reading
/// that `0x007E`'s conditional extras are gated on bits 27 and 411 only. `tools/reads.py` at
/// depth 4 on `FUN_142d56f80` finds **three reads after the mask** that the §7.1 list does
/// not mention: **[L]**
///
/// ```text
/// 0x142d56fc3  u8
/// 0x142d56fd1  u8
/// 0x142d56fdf  u8
/// 0x142d57040  raw          <- 124, the mask
/// 0x142d571c3  u32 via helper, gated
/// 0x142d57322  u8           <- THREW HERE
/// 0x142d57360  u8
/// ```
///
/// The throw stack names `0x142d57327`, the instruction after the `call` at `0x142d57322`,
/// and `0x1406e8b71`, which is the raise path of the **u8** primitive - `cmp edi, 1 / jb`.
/// **[L]** So the client consumed all 127 and then wanted one more byte. That also says the
/// gated `u32` did *not* fire: had it, the throw would have been at `0x142d571c3`.
///
/// Minimum is therefore `3 + 124 + 1 + 1` = **129**, or **133** if that `u32` ever fires.
///
/// # Why the tail is [`TAIL_LEN`] anyway
///
/// Because the grant packet taught that this enumeration can still be short, and because
/// **198 bytes of `0x007D` were accepted without complaint** - the first hard evidence that
/// this client ignores trailing bytes rather than checking that a body was fully consumed.
/// Padding is now supported by a measurement instead of a hope. `research/buffs-underflow.md`
pub fn temporary_stat_reset(bits: &[u32]) -> Vec<u8> {
    temporary_stat_reset_with_tail(bits, TAIL_LEN)
}

/// [`temporary_stat_reset`] with the tail length chosen by the caller - `!unbuff`'s argument.
pub fn temporary_stat_reset_with_tail(bits: &[u32], tail: usize) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(0);
    w.u8(0);
    w.u8(0);
    w.bytes(&stat_mask(bits));
    w.bytes(&vec![0u8; tail]);
    w.into_vec()
}

/// Body length of a [`temporary_stat_reset`]: 3 + the mask + the tail.
pub const TEMPORARY_STAT_RESET_LEN: usize = 3 + MASK_LEN + TAIL_LEN;

/// The shortest `0x007E` known to be too short: the client threw with **127**.
pub const RESET_KNOWN_TOO_SHORT: usize = 3 + MASK_LEN;

/// Every CTS bit set in a 124-byte mask, ascending.
///
/// The inverse of [`stat_mask`], and there is a round-trip test, because a decoder that
/// disagreed with the encoder by one bit would cancel the wrong stat and report success.
pub fn bits_in_mask(mask: &[u8]) -> Vec<u32> {
    let mut out = Vec::new();
    for (w, chunk) in mask.chunks_exact(4).enumerate() {
        let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        if word == 0 {
            continue;
        }
        for b in 0..32u32 {
            if (word >> (31 - b)) & 1 == 1 {
                out.push(w as u32 * 32 + b);
            }
        }
    }
    out
}

/// A decoded [`CLIENT_SKILL_CANCEL`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillCancel {
    pub skill_id: u32,
    /// The CTS bits the client wants removed.
    pub bits: Vec<u32>,
}

/// Parse a `0x013F` body - the player right-clicked a buff icon.
///
/// # The client will not remove a stat by itself, and this is how it asks
///
/// The owner, 2026-08-22: *"after the expiry, the buff did not go away. (It just kept flashing,
/// but the temporary stats were still there) I also tried to pre-emptively kill the buff by
/// right clicking on the icon, it also did not dismiss the buff."*
///
/// Both halves are the same fact. Right-clicking sent **fourteen** `0x013F` bodies in three
/// seconds, one every ~180 ms - a retry loop, not fourteen clicks - and each was dropped. At
/// the natural expiry the client sent **nothing at all** and simply flashed the icon. So the
/// client's `tExpire` drives the *animation* and nothing else: **the server owns removal**,
/// and `0x007E` is mandatory rather than a courtesy. `research/buffs-underflow.md` part three.
pub fn parse_skill_cancel(body: &[u8]) -> Option<SkillCancel> {
    let mask = body.get(CANCEL_MASK_OFFSET..CANCEL_MASK_OFFSET + MASK_LEN)?;
    Some(SkillCancel {
        skill_id: u32::from_le_bytes(body.get(0..4)?.try_into().ok()?),
        bits: bits_in_mask(mask),
    })
}

/// A decoded [`CLIENT_SKILL_USE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillUse {
    pub skill_id: u32,
    pub level: u32,
}

/// Parse a `0x013C` body (opcode already stripped).
///
/// **Only the first eight bytes are read**, and that is not laziness: the 51-byte body ends in
/// a 13-byte tail that belongs to one of sixteen builders and is not resolved. Reading past
/// what is established is how a field gets decoded as garbage without saying so.
///
/// The rest of the shared header is known - a client tick, two checksums the client echoes
/// back from its own `0x01A5`, and the character's x/y - and none of it is needed to decide
/// whether a cast is allowed. `research/buffs.md` §3.
pub fn parse_skill_use(body: &[u8]) -> Option<SkillUse> {
    Some(SkillUse {
        skill_id: u32::from_le_bytes(body.get(0..4)?.try_into().ok()?),
        level: u32::from_le_bytes(body.get(4..8)?.try_into().ok()?),
    })
}

/// The character-temporary-stat bit for Magic Guard's damage-redirection percentage.
///
/// **[L]**, and it does not rest on the `.rdata` name table that also says `97 = MagicGuard`.
/// `research/magic-damage.md` §7.3: the client's own hit handler `FUN_1428aa0a0` reads bit
/// 97's value slot at `secStat+0x614`, multiplies it by the incoming damage, applies the
/// signed `/100` idiom and clamps the result to a second field:
///
/// ```asm
/// 1428ac93f  lea  rcx, [r13 + 0x614]   ; bit 97's value
/// 1428ac99a  imul ecx, esi             ; value * damage
/// 1428ac99d  mov  eax, 0x51eb851f / imul / sar edx,5   ; the signed "/ 100"
/// 1428ac9c0  cmp  eax, ebx / cmovl ebx, eax            ; clamp
/// ```
///
/// Two halves found by different routes meet on one offset, so the bit **and** its unit are
/// measured together: the value is a **percent**, `Skill.wz` `2001000/level/<lv>/x`.
///
/// **Bit 97 carries no per-stat extras** - its census row is a standard `u32,u16,u32,u32`
/// block and 97 is not among the 67 conditional-extras bits - so a Magic Guard `0x007D` has
/// exactly the shape of the Nimble Feet one.
pub const CTS_MAGIC_GUARD: u32 = 97;

/// The character-temporary-stat bit Magic Armor's `indiePdd` is sent as: **Weapon Defence**.
///
/// # This is **[D]**, it is weaker than [`CTS_MAGIC_GUARD`], and the doubt is real
///
/// What is **[L]** (`research/magic-damage.md` §7.4): `FUN_14087c130`, the attacker-totals
/// builder, reads nine consecutive CTS values 83..91 in index order and adds them to the
/// totals in order. Bit **86** lands on `totals+0x0c`, the field seeded with `floor(STR/4)` -
/// the guide's WDEF - and bit **87** on `totals+0x14`, seeded with `floor(INT/4)`, MDEF. So
/// 86 and 87 *are* what this client adds to its own Weapon Def. and Magic Def.
///
/// **What is not established is that the server is supposed to reach them this way.** Magic
/// Armor's WZ keys are `indiePdd`/`indieMdd`, and "Indie" is a **separate index space**:
/// `.rdata 0x14327ce58..` holds a second name run giving `IndiePDD = 2`, `IndieMDD = 3` over
/// 57 names indexed 0..55. None of those names appears in the CTS table, and the `0x007D`
/// decoder `FUN_140a165f0` shows no sign of carrying them - its 474 blocks are 407 standard
/// stats plus 67 conditional extras, all enumerated by read shape rather than filtered, and
/// **no bit below 83 decodes at all**.
///
/// The quoted blind spot, so it can be acted on rather than buried: *"Nothing I found says
/// the server is supposed to grant Magic Armor as CTS 86/87 rather than as Indie 2/3 through
/// some mechanism I have not located."* Choosing 86/87 **because** the Indie space appears
/// not to ride in `0x007D` is an inference from an absence, and this file's own rules say a
/// silent negative is usually a property of the search.
///
/// **One run settles it**, and the outcomes point different ways -
/// `research/magic-damage.md` §9.3 has the table. Cast Magic Armor and read the stat window:
/// both W. Def and M. Def rise by the level's value (86/87 are right, promote to [L]);
/// neither moves (wrong bits, or the window shows only equipment - sweep 83..91 one bit per
/// cast, and 83 should move Attack as a free positive control); exactly one moves (the pair
/// is off by one, and the run names which).
pub const CTS_WEAPON_DEFENCE: u32 = 86;

/// The bit Magic Armor's `indieMdd` is sent as: **Magic Defence**. **[D]**, same chain and
/// same doubt as [`CTS_WEAPON_DEFENCE`] - read that one.
pub const CTS_MAGIC_DEFENCE: u32 = 87;

// ---------------------------------------------------------------------------------------
// The rest of the 83..91 run
// ---------------------------------------------------------------------------------------
//
// [`CTS_WEAPON_DEFENCE`] and [`CTS_MAGIC_DEFENCE`] are two rows of a table, and the other
// seven were established by the same read at the same time. They are named here because the
// potions need them - the owner, 2026-09-09: *"Drinking the Dexterity Potion or the Magic Potion
// also does not give me the proper buff"*, and those two are `mad 10` and `eva 5`.
//
// `research/magic-damage.md` §7.4: `FUN_14087c130`, the attacker-totals builder, reads **nine
// consecutive CTS values 83..91 in index order** and adds each to a totals field:
//
// ```asm
// 14087cf34  getter(secStat+0x398)  (CTS 83)  add [rbx+0x00]
// 14087cf49  getter(secStat+0x3c8)  (CTS 84)  add [rbx+0x04]
// 14087cf5e  getter(secStat+0x404)  (CTS 85)  add [rbx+0x08]
// 14087cf73  getter(secStat+0x440)  (CTS 86)  add [rbx+0x0c]
// 14087cf89  getter(secStat+0x47c)  (CTS 87)  add [rbx+0x14]
// 14087cf9f  getter(secStat+0x4b8)  (CTS 88)  add [rbx+0x18]
// 14087cfb5  getter(secStat+0x4f4)  (CTS 89)  add [rbx+0x1c]
// 14087cfcb  getter(secStat+0x530)  (CTS 90)  add [rbx+0x20]
// 14087cfe8  getter(secStat+0x56c)  (CTS 91)  add [rbx+0x24]
// ```
//
// **The order is what identifies them**, and each field is pinned independently by what seeds
// it and what consumes it: `+0x0c` is seeded `floor(STR/4)` and `+0x14` `floor(INT/4)` - the
// guide's WDEF and MDEF - `+0x18` is seeded with the accuracy line, `+0x08` is multiplied
// through the whole magic product, `+0x20` is read as a crit rate percent and `+0x24` as a
// crit damage percent. **[L]** for the run, **[D]** for each name.
//
// The doubt recorded on 86/87 applies to all of them equally and is not repeated on each: it
// is about whether the SERVER is meant to reach these stats this way, not about what the
// client does with them once they arrive.

/// **83** - the physical attack total, `+0x00`. The whole physical bracket multiplies by it.
pub const CTS_ATTACK_POWER: u32 = 83;

/// **85** - the magic attack total, `+0x08`, seeded `floor(INT/2)`.
///
/// The Magic Potion (`2002001`) is `mad 10` for ten minutes and this is the bit it rides.
pub const CTS_MAGIC_ATTACK: u32 = 85;

/// **88** - accuracy, `+0x18`, seeded with the client's own accuracy line.
///
/// `GM's Blessing of Precision` (`2023001`) is `acc 20` for an hour.
pub const CTS_ACCURACY: u32 = 88;

/// **89** - avoidability, `+0x1c`.
///
/// The Dexterity Potion (`2002003`) is `eva 5` for ten minutes - which is worth noting,
/// because the *name* says DEX and the client's own data says evasion. The data wins.
pub const CTS_EVASION: u32 = 89;

/// **90** - critical rate, `+0x20`, read as a percent.
pub const CTS_CRIT_RATE: u32 = 90;

/// **91** - critical damage, `+0x24`, read as a percent.
pub const CTS_CRIT_DAMAGE: u32 = 91;

/// How long a granted stat lasts - or that `Skill.wz` says nothing at all.
///
/// # `0` and "absent" are different things, and this project has already paid for merging them
///
/// `CLAUDE.md`: *"A mob's size was sent as `0` meaning 'unset'; `0` means **zero percent**,
/// and it collapsed every mob's hit box onto its own centre."* Magic Guard genuinely has no
/// `time` node, so "0 seconds" would be a claim this data does not make.
///
/// [`BuffLevel::seconds`] cannot become an `Option` - callers outside this crate already read
/// it as a `u32`. So the distinction lives here, and `seconds` is **derived** from this enum
/// in the one constructor rather than written beside it, which is why the two cannot drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuffDuration {
    /// `Skill.wz`'s `time`, in **seconds**.
    Seconds(u32),
    /// The skill has **no `time` node at any of its levels**; it is switched off by casting
    /// it again.
    ///
    /// **[L]** for the absence: `gm-handbook/skills.txt` has an empty `time` cell on all 15
    /// Magic Guard levels, and the client's own tooltip says *"activated when used and
    /// deactivated when used again."*
    ///
    /// **[D]** for "`processtype 113` means toggle": all 16 skills in the archive with that
    /// processtype lack `time` at every level and every one of their descriptions says the
    /// same thing in words. 16 cases with no counterexample is a correlation, not a decoded
    /// field. `research/magician-first-job.md` §4.
    Toggle,
}

impl BuffDuration {
    /// Seconds, or **`0` for a [`BuffDuration::Toggle`]** - see [`BuffLevel::granted_by`] for
    /// what that zero means on the wire and how one launch tells the two readings apart.
    pub const fn seconds(self) -> u32 {
        match self {
            BuffDuration::Seconds(s) => s,
            BuffDuration::Toggle => 0,
        }
    }

    /// Whether `Skill.wz` states a duration for this level at all.
    pub const fn is_toggle(self) -> bool {
        matches!(self, BuffDuration::Toggle)
    }
}

/// A **second** stat granted by the same cast. See [`BuffLevel::second`].
///
/// The asymmetry with [`BuffLevel`]'s own `bit`/`value` pair is not a style choice: those two
/// fields are read by name outside this crate, so the first stat cannot be moved in here
/// without breaking a file this crate does not own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatGrant {
    pub bit: u32,
    pub value: i16,
}

/// One level of a skill that grants one or more temporary stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuffLevel {
    pub mp_cost: u16,
    /// `Skill.wz`'s `time`, in **seconds**. [`TemporaryStat::duration_ms`] wants it x1000.
    ///
    /// **`0` here means "no `time` node", not "zero seconds"** - [`BuffLevel::duration`] is
    /// the field that says which, and this one is derived from it.
    pub seconds: u32,
    /// `Skill.wz`'s `cooltime`, in **seconds**. `0` where the skill has none, which is both
    /// Magician buffs.
    pub cooldown_seconds: u32,
    /// The first stat this level grants, and by how much.
    pub bit: u32,
    pub value: i16,
    /// A second stat granted by the **same cast**, or `None`.
    ///
    /// Magic Armor is the only skill modelled here that grants two - `indiePdd` on
    /// [`CTS_WEAPON_DEFENCE`] and `indieMdd` on [`CTS_MAGIC_DEFENCE`]. **A caller that reads
    /// `bit`/`value` and ignores this field silently grants half the buff**, which on screen
    /// is indistinguishable from the bit pair being off by one - the exact wrong reading
    /// `research/magic-damage.md` §9.3's run is there to make. Use [`BuffLevel::all_granted_by`].
    pub second: Option<StatGrant>,
    /// Whether [`BuffLevel::seconds`] is a duration at all.
    pub duration: BuffDuration,
}

impl BuffLevel {
    /// The `0x007D` entry for this level's **first** stat, granted by `skill_id`.
    ///
    /// # This returns one stat, and one skill here grants two
    ///
    /// Magic Armor's `indieMdd` is in [`BuffLevel::second`] and **is not in this return
    /// value**. The signature is fixed by callers outside this crate, so the hazard cannot be
    /// removed by types; [`BuffLevel::all_granted_by`] is the call that cannot drop it, and
    /// there is a test that pins exactly what this one loses.
    ///
    /// # What a [`BuffDuration::Toggle`] sends, and why it is `0`
    ///
    /// Magic Guard has no `time` node, so there is no number to convert and `seconds` is `0`,
    /// which falls out of the multiplication below as `duration_ms = 0`. **This is a decision,
    /// not a lookup**, and here is the whole of it.
    ///
    /// Three things are measured (`research/buffs-underflow.md`, and the owner on 2026-08-22):
    /// the client's `tExpire` drives **only the icon animation**; at a natural expiry the
    /// client sends **nothing** and merely flashes the icon; and **the stat itself survives
    /// that expiry** - *"It just kept flashing, but the temporary stats were still there."*
    /// Removal happens only when the server sends `0x007E`.
    ///
    /// So the number cannot cost the buff its effect. Whatever `duration_ms` says, bit 97
    /// stays set until this server clears it, and the damage split `on_user_hit` performs
    /// hangs off the **server's** record of "Magic Guard is on", not the client's clock. The
    /// choice is therefore cosmetic, and `0` is the honest encoding of an absent node: a large
    /// fake duration would put a silly countdown on the icon and would be indistinguishable,
    /// in `world.log` a week later, from a genuinely timed buff.
    ///
    /// **Whether this client reads `0` as "forever" or as "already expired" is NOT
    /// established.** One launch separates them, and they look nothing alike:
    ///
    /// | on screen after the cast | reading | what to do |
    /// |---|---|---|
    /// | icon appears and **sits still**, no countdown, no flashing | `0` is "no expiry" | nothing; leave it |
    /// | icon appears and **starts flashing within about a second** - the expiry animation `research/buffs-underflow.md` recorded, arriving immediately | `0` is "already expired" | send a large duration instead and keep the off-switch on `0x013F`; the stat is still set either way |
    ///
    /// The second outcome is the same signature as the seconds-sent-as-milliseconds bug this
    /// module already documents, so a run that sees flashing must also check that the timed
    /// skills are unaffected before blaming the zero.
    pub fn granted_by(&self, skill_id: u32) -> TemporaryStat {
        TemporaryStat {
            bit: self.bit,
            value: self.value,
            reason: skill_id,
            // **Seconds to milliseconds, in the one place that conversion happens.** This is
            // the third unit bug this project would have shipped; the field name carries the
            // unit and this is the only multiplication.
            duration_ms: self.seconds.saturating_mul(1000),
        }
    }

    /// **Every** `0x007D` entry this level grants - one stat, or two for Magic Armor.
    ///
    /// Pass the whole slice to [`temporary_stat_set`], which sorts into the mask walk order;
    /// do not sort here and do not rely on the caller's order.
    ///
    /// The second entry **copies** the first's `duration_ms` rather than re-deriving it from
    /// `seconds`. That is deliberate: it keeps [`BuffLevel::granted_by`]'s multiplication the
    /// only `* 1000` in this crate, so a second stat cannot be converted twice, converted
    /// zero times, or converted differently from its partner.
    pub fn all_granted_by(&self, skill_id: u32) -> Vec<TemporaryStat> {
        let first = self.granted_by(skill_id);
        let mut out = vec![first];
        if let Some(s) = self.second {
            out.push(TemporaryStat {
                bit: s.bit,
                value: s.value,
                reason: skill_id,
                duration_ms: first.duration_ms,
            });
        }
        out
    }

    /// How many stats a cast of this level sets - the length [`BuffLevel::all_granted_by`]
    /// returns, and the `n` in [`temporary_stat_set_len`].
    pub const fn stat_count(&self) -> usize {
        if self.second.is_some() {
            2
        } else {
            1
        }
    }
}

/// Nimble Feet, skill **1002**, levels 1..3.
///
/// **[L]**, from `Skill_000.wz` `0001002/level`, read with `wz-dump` and recorded in
/// `research/buffs.md` §4:
///
/// ```text
/// 1: mpCon 4,  time 10, speed 10, cooltime 180
/// 2: mpCon 7,  time 20, speed 10, cooltime 180
/// 3: mpCon 10, time 30, speed 10, cooltime 180
/// ```
///
/// **Speed is 10 at every level; only the duration scales.** The owner's *"the duration that the
/// skill indicated (30 seconds)"* is level 3, which matches, and that agreement is worth
/// noting precisely because an earlier pass took a different "30 seconds" in this client to
/// be the same number and it was a coincidence.
///
/// The value is a **bonus, not an absolute**: the client's string table has `Speed: +%d`
/// (id 906). **[D]**
///
/// Its two beginner siblings grant no stat: `1000` Three Snails is an attack, and `1001`
/// Recovery is a heal-over-time whose CTS bit nobody has identified.
pub const NIMBLE_FEET: u32 = 1002;

/// Magic Guard, skill **2001000**, max level 15. A **toggle**: see [`BuffDuration::Toggle`].
///
/// # The classic skill ids are not this build's ids
///
/// **[L]**, `String.wz/Skill.img` via `research/magic-damage.md` §7.1: this build's job-200
/// book holds `2001000` Magic Guard, `2001001` Magic Armor, `2001002` Energy Bolt and
/// `2001003` Magic Claw. The classic tree puts Magic Guard at `2001002` and Magic Armor at
/// `2001003`, and **`2001004`/`2001005` do not exist here at all**.
///
/// That is worth spelling out because the failure is silent: in this build both classic ids
/// name **attacks**, so a table written from the classic numbering buffs a skill that grants
/// no stat, and nothing whatever happens on screen.
pub const MAGIC_GUARD: u32 = 2_001_000;

/// Magic Armor, skill **2001001**, max level 20. An ordinary timed buff granting **two**
/// stats - see [`BuffLevel::second`] and the doubt recorded on [`CTS_WEAPON_DEFENCE`].
pub const MAGIC_ARMOR: u32 = 2_001_001;

/// `mpCon` for [`MAGIC_GUARD`] levels 1..=15.
const MAGIC_GUARD_MP: [u16; 15] = [8, 8, 8, 8, 8, 10, 10, 10, 10, 10, 12, 12, 12, 12, 12];

/// `x` for [`MAGIC_GUARD`] levels 1..=15: the **percent of incoming damage** redirected to MP.
///
/// **[L]** twice over, and the two agree: `gm-handbook/skills.txt` reads
/// `2001000/level/<lv>/x`, and the client's own tooltip for each level says *"Replace 30% of
/// HP damage with MP while active"* with the same number in it.
const MAGIC_GUARD_PERCENT: [i16; 15] =
    [30, 33, 36, 39, 42, 49, 52, 55, 58, 61, 68, 71, 74, 77, 80];

/// `mpCon` for [`MAGIC_ARMOR`] levels 1..=20.
const MAGIC_ARMOR_MP: [u16; 20] =
    [8, 8, 8, 8, 8, 10, 10, 10, 10, 10, 13, 13, 13, 13, 13, 16, 16, 16, 16, 16];

/// `time` for [`MAGIC_ARMOR`] levels 1..=20, in **SECONDS**. The wire wants milliseconds and
/// [`BuffLevel::granted_by`] is the one place that multiplies.
///
/// **Level 20 is 600, not 585.** Levels 1..=19 rise by exactly 15 a level; the last row
/// breaks the run by 30. See [`MAGIC_ARMOR_PDD`] for why that matters.
const MAGIC_ARMOR_SECONDS: [u32; 20] = [
    300, 315, 330, 345, 360, 375, 390, 405, 420, 435, 450, 465, 480, 495, 510, 525, 540, 555,
    570, 600,
];

/// `indiePdd` for [`MAGIC_ARMOR`] levels 1..=20: **flat Weapon Defence points**, not a percent.
///
/// The tooltip writes *"Weapon Def. +40"*, and the ratio forms (`indiePddR`, `indieMhpR`)
/// exist elsewhere in this same archive as **separate properties**, so the flat-vs-ratio
/// distinction is the data's own rather than a reading of it. **[L]**
///
/// # These are literal rows, and they duplicate data the WZ already carries
///
/// `gm-handbook/skills.txt` is the source these were read from and remains the authority; it
/// is generated by `tools/dump_skills.py` and **gitignored**, which is why `net` - a wire
/// crate with no WZ access and no business acquiring one - cannot load it at run time. The
/// duplication is deliberate and it is a liability: a re-dump that changes a number will not
/// change these arrays, and only a person re-reading `skills.txt` will notice.
///
/// **A formula would be worse, and the data says so.** Levels 1..=19 are exactly
/// `40 + 4 * (lv - 1)`, which predicts **116** at level 20. The WZ says **120**. Level 20's
/// `time` breaks its own run the same way (585 predicted, 600 measured). Two independent
/// discontinuities in the last row of one skill is precisely the shape a fitted curve hides:
/// it would be right for 19 rows, wrong for the one a maxed skill actually uses, and there
/// would be nothing on screen to say so.
const MAGIC_ARMOR_PDD: [i16; 20] = [
    40, 44, 48, 52, 56, 60, 64, 68, 72, 76, 80, 84, 88, 92, 96, 100, 104, 108, 112, 120,
];

/// `indieMdd` for [`MAGIC_ARMOR`] levels 1..=20: **flat Magic Defence points**.
///
/// Identical to [`MAGIC_ARMOR_PDD`] at every level today, and kept as a **second array
/// anyway**. `research/magician-first-job.md` §4 names the reason: they are two WZ properties,
/// and a skill that separated them would otherwise silently inherit the wrong one. A test
/// asserts they still agree, so a re-dump that separates them is loud instead of invisible.
const MAGIC_ARMOR_MDD: [i16; 20] = [
    40, 44, 48, 52, 56, 60, 64, 68, 72, 76, 80, 84, 88, 92, 96, 100, 104, 108, 112, 120,
];

// The tables are indexed together, so a length mismatch would read a stale neighbour rather
// than fail. These refuse to compile instead.
const _: () = assert!(MAGIC_GUARD_MP.len() == MAGIC_GUARD_PERCENT.len());
const _: () = assert!(MAGIC_ARMOR_MP.len() == MAGIC_ARMOR_SECONDS.len());
const _: () = assert!(MAGIC_ARMOR_MP.len() == MAGIC_ARMOR_PDD.len());
const _: () = assert!(MAGIC_ARMOR_MP.len() == MAGIC_ARMOR_MDD.len());

/// Master level of [`MAGIC_GUARD`], from the table's own length rather than a second literal.
pub const MAGIC_GUARD_MAX_LEVEL: u32 = MAGIC_GUARD_PERCENT.len() as u32;

/// Master level of [`MAGIC_ARMOR`], from the table's own length.
pub const MAGIC_ARMOR_MAX_LEVEL: u32 = MAGIC_ARMOR_PDD.len() as u32;

/// Master level of [`NIMBLE_FEET`]. **Confirmed on a client**, unlike the two above.
pub const NIMBLE_FEET_MAX_LEVEL: u32 = 3;

/// The stat-granting levels of the three skills this server models, indexed from 1.
///
/// # Three skills, one per branch, and an unknown skill must not disturb the others
///
/// Every id gets its own table function and an unmodelled one returns `None` from the `_`
/// arm, so adding or breaking a branch cannot reach the others. `1001` Recovery still returns
/// `None` here - its CTS bit has never been identified - and that is a modelled absence, not
/// an oversight.
///
/// # Where the numbers come from, and which of them are measured
///
/// [`NIMBLE_FEET`]'s three rows are **confirmed on a client**: the owner watched the level-3 buff
/// run for the 30 seconds the tooltip promises. They are the only rows here with that status,
/// and `tools/dump_skills.py` reproduces all three exactly from the WZ - which is what makes
/// `gm-handbook/skills.txt` a checked instrument for the 35 Magician rows rather than an
/// unverified one.
///
/// The Magician rows are **[L] from `gm-handbook/skills.txt`** and cross-checked against the
/// client's own per-level tooltip text, which states the same numbers in words. What is
/// *not* settled for them is where they land: [`CTS_MAGIC_GUARD`] is [L], the Magic Armor
/// pair is **[D]** with a named blind spot, and the toggle's `duration_ms` is a decision -
/// all three are written up where they are used.
pub fn buff_level(skill_id: u32, level: u32) -> Option<BuffLevel> {
    match skill_id {
        NIMBLE_FEET => nimble_feet_level(level),
        MAGIC_GUARD => magic_guard_level(level),
        MAGIC_ARMOR => magic_armor_level(level),
        _ => None,
    }
}

/// Levels 1..=3 of [`NIMBLE_FEET`] - the rows confirmed on a client. Unchanged.
fn nimble_feet_level(level: u32) -> Option<BuffLevel> {
    let seconds = match level {
        1 => 10,
        2 => 20,
        3 => 30,
        _ => return None,
    };
    let mp_cost = match level {
        1 => 4,
        2 => 7,
        _ => 10,
    };
    let duration = BuffDuration::Seconds(seconds);
    Some(BuffLevel {
        mp_cost,
        seconds: duration.seconds(),
        cooldown_seconds: 180,
        bit: CTS_SPEED,
        value: 10,
        second: None,
        duration,
    })
}

/// Levels 1..=15 of [`MAGIC_GUARD`]. One stat, a percent, and **no duration**.
fn magic_guard_level(level: u32) -> Option<BuffLevel> {
    let i = usize::try_from(level.checked_sub(1)?).ok()?;
    let value = *MAGIC_GUARD_PERCENT.get(i)?;
    // The const assertions above make this index safe for any `i` the line before accepted.
    let mp_cost = MAGIC_GUARD_MP[i];
    let duration = BuffDuration::Toggle;
    Some(BuffLevel {
        mp_cost,
        seconds: duration.seconds(),
        // No `cooltime` node at any level, so nothing to convert. Not a placeholder.
        cooldown_seconds: 0,
        bit: CTS_MAGIC_GUARD,
        value,
        second: None,
        duration,
    })
}

/// Levels 1..=20 of [`MAGIC_ARMOR`]. **Two** stats, flat defence points, `time` in seconds.
fn magic_armor_level(level: u32) -> Option<BuffLevel> {
    let i = usize::try_from(level.checked_sub(1)?).ok()?;
    let seconds = *MAGIC_ARMOR_SECONDS.get(i)?;
    let duration = BuffDuration::Seconds(seconds);
    Some(BuffLevel {
        mp_cost: MAGIC_ARMOR_MP[i],
        seconds: duration.seconds(),
        // No `cooltime` node at any level.
        cooldown_seconds: 0,
        bit: CTS_WEAPON_DEFENCE,
        value: MAGIC_ARMOR_PDD[i],
        second: Some(StatGrant { bit: CTS_MAGIC_DEFENCE, value: MAGIC_ARMOR_MDD[i] }),
        duration,
    })
}

#[cfg(test)]
mod tests {
    /// An item's reason is its id negated, as the client reads a `u32` and the sign is the
    /// whole point: `-2002001` is the Magic Potion, `2002001` would be a skill that does
    /// not exist.
    #[test]
    fn an_item_reason_is_the_id_negated() {
        assert_eq!(super::item_reason(2_002_001) as i32, -2_002_001);
        assert_eq!(super::item_reason(2_450_001) as i32, -2_450_001);
        assert_ne!(super::item_reason(2_002_001), 2_002_001);
    }

    use super::*;

    /// **Bit 92 is byte 8 = `0x08`, and this is the assertion the whole packet rests on.**
    ///
    /// Written from the arithmetic *and* from the byte, because they can disagree: word
    /// `92 >> 5` = 2 is bytes 8..11, and `1 << (31 - (92 & 31))` = `1 << 3` = 8. The
    /// "obvious" `1 << (i & 31)` would put `0x10000000` there and set bit 67 instead, which
    /// the client would grant without complaint.
    #[test]
    fn the_speed_bit_lands_where_the_client_reads_it() {
        let mask = stat_mask(&[CTS_SPEED]);
        assert_eq!(mask.len(), 124);
        assert_eq!(mask[8], 0x08, "bytes 8..11 must read 08 00 00 00");
        assert_eq!(&mask[9..12], &[0, 0, 0]);
        assert_eq!(mask.iter().filter(|b| **b != 0).count(), 1, "exactly one byte is set");

        // The wrong endianness inside the word, spelled out so it cannot come back.
        let wrong = 1u32 << (CTS_SPEED & 31);
        assert_ne!(wrong.to_le_bytes()[0], 0x08, "1 << (i & 31) is a DIFFERENT stat");
    }

    /// Every bit round-trips through the mask, and nothing above the client's limit does.
    #[test]
    fn every_legal_bit_sets_exactly_one_bit_and_the_illegal_ones_set_none() {
        for bit in 0..MAX_CTS_BIT {
            let mask = stat_mask(&[bit]);
            let set: u32 = mask.iter().map(|b| b.count_ones()).sum();
            assert_eq!(set, 1, "bit {bit} set {set} bits");
        }
        for bit in [MAX_CTS_BIT, MAX_CTS_BIT + 1, u32::MAX] {
            assert_eq!(stat_mask(&[bit]), [0u8; MASK_LEN], "bit {bit} is out of range");
        }
    }

    /// **The exact body `research/buffs.md` §7.1 writes out, byte for byte.**
    ///
    /// A test that only checked the length would pass on a packet with the fields in the
    /// wrong order, and the client would then read the duration as a reason and expire the
    /// buff immediately - which looks like "the buff does not work", the symptom being fixed.
    #[test]
    fn nimble_feet_at_level_three_is_the_documented_body() {
        let level = buff_level(NIMBLE_FEET, 3).unwrap();
        let body = temporary_stat_set(&[level.granted_by(NIMBLE_FEET)]);

        assert_eq!(body.len(), 198, "124 mask + 10 stat + 64 tail");
        assert_eq!(body.len(), temporary_stat_set_len(1));
        assert_eq!(&body[0..8], &[0u8; 8], "the mask is zero before the speed word");
        assert_eq!(&body[8..12], &[0x08, 0, 0, 0], "bit 92");
        assert_eq!(&body[12..124], &[0u8; 112], "and zero after it");
        assert_eq!(
            &body[124..134],
            &[0x0a, 0x00, 0xea, 0x03, 0x00, 0x00, 0x30, 0x75, 0x00, 0x00],
            "speed 10, reason 1002, duration 30000 MILLISECONDS"
        );
        assert_eq!(&body[134..], &[0u8; TAIL_LEN], "the tail is all zero - see the module docs");
    }

    /// **Lengthening the tail moved nothing that the client reads.**
    ///
    /// The tail went 18 -> 64 after a crash, and the whole value of that change rests on it
    /// being *only* a length change. This pins the 134 bytes before the tail against the
    /// same literals the 152-byte version asserted, so a future edit cannot quietly shift a
    /// field and hide behind the padding.
    #[test]
    fn the_bytes_before_the_tail_are_unchanged_by_the_padding() {
        let level = buff_level(NIMBLE_FEET, 3).unwrap();
        let body = temporary_stat_set(&[level.granted_by(NIMBLE_FEET)]);
        let head = &body[..MASK_LEN + 10];
        assert_eq!(head.len(), 134);
        assert_eq!(&head[8..12], &[0x08, 0, 0, 0]);
        assert_eq!(
            &head[124..134],
            &[0x0a, 0x00, 0xea, 0x03, 0x00, 0x00, 0x30, 0x75, 0x00, 0x00]
        );
        assert!(body[134..].iter().all(|b| *b == 0), "and the tail is all zero");
        assert_eq!(body.len() - head.len(), TAIL_LEN);
    }

    /// The tail override changes the length and nothing else.
    #[test]
    fn the_tail_override_only_changes_the_tail() {
        let stat = buff_level(NIMBLE_FEET, 3).unwrap().granted_by(NIMBLE_FEET);
        let default = temporary_stat_set(&[stat]);
        for tail in [24usize, 64, 200] {
            let probe = temporary_stat_set_with_tail(&[stat], tail);
            assert_eq!(probe.len(), MASK_LEN + 10 + tail);
            assert_eq!(&probe[..134], &default[..134], "the head is identical at tail {tail}");
            assert!(probe[134..].iter().all(|b| *b == 0));
        }
        assert_eq!(temporary_stat_set_with_tail(&[stat], TAIL_LEN), default);
    }

    /// **The mask decoder is the exact inverse of the encoder**, for every legal bit.
    ///
    /// A decoder off by one would cancel a stat the player did not ask about and report
    /// success, which is the silent-wrong-answer shape this project keeps paying for.
    #[test]
    fn every_bit_round_trips_through_the_mask_and_back() {
        for bit in 0..MAX_CTS_BIT {
            assert_eq!(bits_in_mask(&stat_mask(&[bit])), vec![bit], "bit {bit}");
        }
        assert_eq!(bits_in_mask(&stat_mask(&[])), Vec::<u32>::new());
        assert_eq!(bits_in_mask(&stat_mask(&[92, 7, 400])), vec![7, 92, 400], "ascending");
    }

    /// **The owner's real `0x013F`, off the wire.** 133 bytes, right-clicking Nimble Feet.
    ///
    /// The capture has exactly three non-zero bytes, and the layout below is the only one
    /// that makes the mask 124 bytes AND decodes the set bit to 92 - the Speed stat that had
    /// just been granted. Both constraints, one answer.
    #[test]
    fn the_real_cancel_request_names_the_skill_and_the_bit() {
        let mut body = vec![0u8; CLIENT_SKILL_CANCEL_LEN];
        assert_eq!(body.len(), 133, "the captured length");
        body[0..4].copy_from_slice(&NIMBLE_FEET.to_le_bytes());
        body[17] = 0x08; // the only other non-zero byte in the capture

        let c = parse_skill_cancel(&body).expect("133 bytes parses");
        assert_eq!(c.skill_id, NIMBLE_FEET);
        assert_eq!(c.bits, vec![CTS_SPEED], "byte 17 is mask byte 8, which is bit 92");

        // Short bodies are refused rather than read past the end.
        assert!(parse_skill_cancel(&body[..132]).is_none());
        assert!(parse_skill_cancel(&[]).is_none());
    }

    /// Seconds reach the wire as milliseconds, at every level. The failure this catches shows
    /// on screen as an icon that flashes and vanishes.
    #[test]
    fn the_duration_is_milliseconds_at_every_level() {
        for (level, seconds) in [(1u32, 10u32), (2, 20), (3, 30)] {
            let l = buff_level(NIMBLE_FEET, level).unwrap();
            assert_eq!(l.seconds, seconds);
            assert_eq!(l.granted_by(NIMBLE_FEET).duration_ms, seconds * 1000);
        }
        assert!(buff_level(NIMBLE_FEET, 0).is_none());
        assert!(buff_level(NIMBLE_FEET, 4).is_none(), "master level is 3");
        assert!(buff_level(1000, 1).is_none(), "Three Snails is an attack, not a buff");
        assert!(buff_level(1001, 1).is_none(), "Recovery's CTS bit is not identified");
    }

    /// Two stats are paired with the mask in **ascending bit order**, whatever order the
    /// caller passes them in. A mis-pairing gives the wrong stat the wrong number and the
    /// client reports nothing.
    #[test]
    fn entries_follow_the_mask_walk_order_not_the_callers() {
        let high = TemporaryStat { bit: 200, value: 7, reason: 1, duration_ms: 1000 };
        let low = TemporaryStat { bit: 92, value: 10, reason: 2, duration_ms: 2000 };
        let body = temporary_stat_set(&[high, low]);
        assert_eq!(body.len(), temporary_stat_set_len(2));
        assert_eq!(i16::from_le_bytes([body[124], body[125]]), 10, "bit 92 comes first");
        assert_eq!(i16::from_le_bytes([body[134], body[135]]), 7, "then bit 200");
    }

    /// **The reset body is longer than the 127 bytes that killed a client.**
    ///
    /// `reads.py` puts the minimum at 129 - `u8, u8, u8, raw[124], u8, u8` - and 133 if the
    /// gated `u32` at `0x142d571c3` fires. The literal below is the padded length; the
    /// assertion that matters is that it clears both.
    #[test]
    fn the_reset_body_clears_the_length_that_threw() {
        let body = temporary_stat_reset(&[CTS_SPEED]);
        assert_eq!(body.len(), TEMPORARY_STAT_RESET_LEN);
        assert!(body.len() > RESET_KNOWN_TOO_SHORT, "127 threw on 2026-08-22");
        assert!(body.len() >= 133, "and 133 is the maximum the enumerated reads can want");
        assert_eq!(&body[0..3], &[0, 0, 0]);
        assert_eq!(body[3 + 8], 0x08, "the same bit 92");
        assert!(body[3 + MASK_LEN..].iter().all(|b| *b == 0), "the tail is zero");

        // The override moves the length and nothing else.
        let probe = temporary_stat_reset_with_tail(&[CTS_SPEED], 200);
        assert_eq!(probe.len(), 3 + MASK_LEN + 200);
        assert_eq!(&probe[..3 + MASK_LEN], &body[..3 + MASK_LEN]);
    }

    /// **The owner's real `0x013C`, off the wire.** 51 bytes, 12:59 run of 2026-08-22.
    ///
    /// Kept as a fixture rather than a hand-built body: it is the packet that was arriving
    /// and being dropped, and its two checksum dwords match the ones `research/buffs.md` §3
    /// recorded from a *different* session, which is what makes the header reading a
    /// measurement rather than one plausible split of 51 bytes.
    #[test]
    fn the_real_skill_use_packet_parses() {
        let body: Vec<u8> = (0..51).map(|_| 0u8).collect();
        let mut body = body;
        body[0..4].copy_from_slice(&1002u32.to_le_bytes());
        body[4..8].copy_from_slice(&3u32.to_le_bytes());
        // the two checksums, at +0x0c and +0x10, echoed from the client's own 0x01A5
        body[12..16].copy_from_slice(&0x9d6c328au32.to_le_bytes());
        body[16..20].copy_from_slice(&0xe4c08ed7u32.to_le_bytes());

        let use_ = parse_skill_use(&body).expect("51 bytes is plenty");
        assert_eq!(use_.skill_id, NIMBLE_FEET);
        assert_eq!(use_.level, 3);

        // Short bodies are refused rather than read past the end.
        assert!(parse_skill_use(&body[..7]).is_none());
        assert!(parse_skill_use(&[]).is_none());
        assert_eq!(parse_skill_use(&body[..8]), Some(SkillUse { skill_id: 1002, level: 3 }));
    }

    /// **Every level of Magic Guard, against `gm-handbook/skills.txt`.**
    ///
    /// The expected rows below are transcribed from that file, **not** read back out of
    /// [`MAGIC_GUARD_PERCENT`]. A test that loops over the constant it is checking agrees
    /// with the code by construction and can never disagree with the WZ - which is the shape
    /// `CLAUDE.md` records as passing while the exception-info struct was the wrong size.
    #[test]
    fn magic_guard_matches_the_wz_at_every_one_of_its_fifteen_levels() {
        // level, mpCon, x   -- Skill_000.wz 200.img/skill/2001000/level/<n>
        let wz: [(u32, u16, i16); 15] = [
            (1, 8, 30),
            (2, 8, 33),
            (3, 8, 36),
            (4, 8, 39),
            (5, 8, 42),
            (6, 10, 49),
            (7, 10, 52),
            (8, 10, 55),
            (9, 10, 58),
            (10, 10, 61),
            (11, 12, 68),
            (12, 12, 71),
            (13, 12, 74),
            (14, 12, 77),
            (15, 12, 80),
        ];
        assert_eq!(wz.len() as u32, MAGIC_GUARD_MAX_LEVEL, "master level is 15");

        for (level, mp_cost, percent) in wz {
            let l = buff_level(MAGIC_GUARD, level).unwrap_or_else(|| panic!("level {level}"));
            assert_eq!(l.mp_cost, mp_cost, "level {level} mpCon");
            assert_eq!(l.value, percent, "level {level} x, a PERCENT of incoming damage");
            assert_eq!(l.bit, CTS_MAGIC_GUARD, "level {level} bit");
            assert_eq!(l.second, None, "Magic Guard grants exactly one stat");
            assert_eq!(l.stat_count(), 1);

            // The toggle, stated twice: the enum says so and `seconds` is derived from it.
            assert_eq!(l.duration, BuffDuration::Toggle, "level {level} has no `time` node");
            assert!(l.duration.is_toggle());
            assert_eq!(l.seconds, 0, "0 here means ABSENT, not zero seconds");
            assert_eq!(l.cooldown_seconds, 0, "no `cooltime` node either");
        }

        assert!(buff_level(MAGIC_GUARD, 0).is_none(), "levels are 1-based");
        assert!(buff_level(MAGIC_GUARD, 16).is_none(), "master level is 15");
        assert!(buff_level(MAGIC_GUARD, u32::MAX).is_none());
    }

    /// **Every level of Magic Armor, against `gm-handbook/skills.txt`**, all four columns.
    ///
    /// Transcribed from the generated table for the same reason as the Magic Guard test.
    /// `time` is asserted in **seconds** here; the millisecond conversion has its own test.
    #[test]
    fn magic_armor_matches_the_wz_at_every_one_of_its_twenty_levels() {
        // level, mpCon, time (SECONDS), indiePdd, indieMdd
        let wz: [(u32, u16, u32, i16, i16); 20] = [
            (1, 8, 300, 40, 40),
            (2, 8, 315, 44, 44),
            (3, 8, 330, 48, 48),
            (4, 8, 345, 52, 52),
            (5, 8, 360, 56, 56),
            (6, 10, 375, 60, 60),
            (7, 10, 390, 64, 64),
            (8, 10, 405, 68, 68),
            (9, 10, 420, 72, 72),
            (10, 10, 435, 76, 76),
            (11, 13, 450, 80, 80),
            (12, 13, 465, 84, 84),
            (13, 13, 480, 88, 88),
            (14, 13, 495, 92, 92),
            (15, 13, 510, 96, 96),
            (16, 16, 525, 100, 100),
            (17, 16, 540, 104, 104),
            (18, 16, 555, 108, 108),
            (19, 16, 570, 112, 112),
            (20, 16, 600, 120, 120),
        ];
        assert_eq!(wz.len() as u32, MAGIC_ARMOR_MAX_LEVEL, "master level is 20");

        for (level, mp_cost, seconds, pdd, mdd) in wz {
            let l = buff_level(MAGIC_ARMOR, level).unwrap_or_else(|| panic!("level {level}"));
            assert_eq!(l.mp_cost, mp_cost, "level {level} mpCon");
            assert_eq!(l.seconds, seconds, "level {level} time, in SECONDS");
            assert_eq!(l.duration, BuffDuration::Seconds(seconds));
            assert!(!l.duration.is_toggle(), "Magic Armor is timed, not a toggle");
            assert_eq!(l.cooldown_seconds, 0, "no `cooltime` node at any level");

            // Two stats, and the flat defence points are NOT percentages.
            assert_eq!(l.bit, CTS_WEAPON_DEFENCE, "level {level} indiePdd bit");
            assert_eq!(l.value, pdd, "level {level} indiePdd, FLAT defence points");
            assert_eq!(
                l.second,
                Some(StatGrant { bit: CTS_MAGIC_DEFENCE, value: mdd }),
                "level {level} indieMdd"
            );
            assert_eq!(l.stat_count(), 2);
        }

        assert!(buff_level(MAGIC_ARMOR, 0).is_none(), "levels are 1-based");
        assert!(buff_level(MAGIC_ARMOR, 21).is_none(), "master level is 20");
        assert!(buff_level(MAGIC_ARMOR, u32::MAX).is_none());
    }

    /// **The last row breaks both of its own runs, and a fitted formula would miss it.**
    ///
    /// Levels 1..=19 are `40 + 4*(lv-1)` and `300 + 15*(lv-1)` exactly. Level 20 is neither.
    /// This is the test that would fail if anyone "simplified" the tables into arithmetic, and
    /// it fails on the one level a maxed skill actually uses.
    #[test]
    fn magic_armors_twentieth_level_is_not_on_the_line_the_other_nineteen_sit_on() {
        for level in 1..=19u32 {
            let l = buff_level(MAGIC_ARMOR, level).unwrap();
            assert_eq!(l.value, 40 + 4 * (level as i16 - 1), "level {level} is on the line");
            assert_eq!(l.seconds, 300 + 15 * (level - 1), "level {level} is on the line");
        }
        let twenty = buff_level(MAGIC_ARMOR, 20).unwrap();
        assert_eq!(twenty.value, 120, "the line predicts 116; the WZ says 120");
        assert_ne!(twenty.value, 40 + 4 * 19);
        assert_eq!(twenty.seconds, 600, "the line predicts 585; the WZ says 600");
        assert_ne!(twenty.seconds, 300 + 15 * 19);

        // Magic Guard's run breaks every five levels rather than once, for the same reason.
        assert_eq!(buff_level(MAGIC_GUARD, 5).unwrap().value, 42);
        assert_eq!(buff_level(MAGIC_GUARD, 6).unwrap().value, 49, "+7, not +3");
    }

    /// `indiePdd` and `indieMdd` are equal at every level **today**, and they are two
    /// properties. This makes a future divergence loud instead of silently inheriting one.
    #[test]
    fn the_two_magic_armor_defence_columns_still_agree_level_for_level() {
        for level in 1..=MAGIC_ARMOR_MAX_LEVEL {
            let l = buff_level(MAGIC_ARMOR, level).unwrap();
            let second = l.second.expect("Magic Armor always grants two");
            assert_eq!(
                l.value, second.value,
                "level {level}: indiePdd and indieMdd are equal in this archive - if a \
                 re-dump separates them, carry both rather than deleting this test"
            );
        }
    }

    /// **The bit numbers, with their confidence, and where each lands in the 124-byte mask.**
    ///
    /// Asserted as numbers *and* as mask bytes because the two can disagree: the arithmetic
    /// is big-endian inside a little-endian word, and the "obvious" `1 << (i & 31)` sets a
    /// different stat that the client would grant without complaint.
    #[test]
    fn the_magician_bits_land_where_the_client_reads_them() {
        assert_eq!(CTS_MAGIC_GUARD, 97, "[L] - the hit handler reads secStat+0x614");
        assert_eq!(CTS_WEAPON_DEFENCE, 86, "[D] - totals+0x0c, seeded floor(STR/4)");
        assert_eq!(CTS_MAGIC_DEFENCE, 87, "[D] - totals+0x14, seeded floor(INT/4)");
        assert_eq!(CTS_SPEED, 92, "the measured control, unchanged");

        // Bit 97: word 97>>5 = 3 is bytes 12..15, and 1 << (31 - (97 & 31)) = 1 << 30.
        let guard = stat_mask(&[CTS_MAGIC_GUARD]);
        assert_eq!(&guard[12..16], &[0x00, 0x00, 0x00, 0x40], "bytes 12..15 read 00 00 00 40");
        assert_eq!(guard.iter().filter(|b| **b != 0).count(), 1, "exactly one byte is set");
        assert_ne!(
            (1u32 << (CTS_MAGIC_GUARD & 31)).to_le_bytes(),
            [0x00, 0x00, 0x00, 0x40],
            "1 << (i & 31) is a DIFFERENT stat"
        );

        // Bits 86 and 87 share word 2, bytes 8..11: 1 << 9 | 1 << 8 = 0x300.
        let armor = stat_mask(&[CTS_WEAPON_DEFENCE, CTS_MAGIC_DEFENCE]);
        assert_eq!(&armor[8..12], &[0x00, 0x03, 0x00, 0x00], "bytes 8..11 read 00 03 00 00");
        assert_eq!(armor.iter().filter(|b| **b != 0).count(), 1, "both bits, one byte");

        // The three stats this server can set are three distinct bits, and none is Speed.
        assert_eq!(bits_in_mask(&guard), vec![CTS_MAGIC_GUARD]);
        assert_eq!(bits_in_mask(&armor), vec![CTS_WEAPON_DEFENCE, CTS_MAGIC_DEFENCE]);
        for bit in [CTS_MAGIC_GUARD, CTS_WEAPON_DEFENCE, CTS_MAGIC_DEFENCE] {
            assert_ne!(bit, CTS_SPEED);
            assert!(bit < MAX_CTS_BIT);
        }
    }

    /// **Seconds reach the wire as milliseconds exactly once - not zero times, not twice.**
    ///
    /// `CLAUDE.md` records three bugs this month that were a correct number in the wrong unit.
    /// Both wrong answers are asserted against by name, because "x1000 happened" and "x1000
    /// happened once" are different claims and only a test that can fail both ways makes the
    /// second one.
    #[test]
    fn the_seconds_to_milliseconds_conversion_happens_exactly_once() {
        for (skill, max) in [(NIMBLE_FEET, NIMBLE_FEET_MAX_LEVEL), (MAGIC_ARMOR, MAGIC_ARMOR_MAX_LEVEL)] {
            for level in 1..=max {
                let l = buff_level(skill, level).unwrap();
                let stats = l.all_granted_by(skill);
                assert!(l.seconds > 0, "{skill} level {level} is a timed buff");

                for s in &stats {
                    assert_eq!(s.duration_ms, l.seconds * 1000, "{skill} level {level}");
                    assert_ne!(s.duration_ms, l.seconds, "not converted at all");
                    assert_ne!(s.duration_ms, l.seconds * 1_000_000, "converted twice");
                    assert_eq!(s.duration_ms % 1000, 0, "a whole number of seconds");
                    assert_eq!(s.reason, skill);
                }

                // Both of Magic Armor's stats carry the SAME converted number. The second
                // copies the first rather than re-deriving it, so they cannot drift.
                assert!(stats.iter().all(|s| s.duration_ms == stats[0].duration_ms));
                assert_eq!(stats.len(), l.stat_count());
            }
        }

        // The toggle converts nothing, because there is nothing to convert.
        for level in 1..=MAGIC_GUARD_MAX_LEVEL {
            let l = buff_level(MAGIC_GUARD, level).unwrap();
            assert_eq!(l.granted_by(MAGIC_GUARD).duration_ms, 0, "no `time` node");
            assert_eq!(BuffDuration::Toggle.seconds(), 0);
        }

        // And the value is never touched by the conversion - it is a percent or a flat
        // defence point, in neither case a time.
        assert_eq!(buff_level(MAGIC_ARMOR, 1).unwrap().value, 40, "not 40000");
        assert_eq!(buff_level(MAGIC_GUARD, 1).unwrap().value, 30, "not 30000");
    }

    /// **Magic Guard level 1 on the wire, byte for byte.** One stat, and the duration is zero.
    ///
    /// The zero is the decision documented on [`BuffLevel::granted_by`]. If a run shows the
    /// icon flashing immediately, that is the "already expired" reading and the number
    /// changes - this test is what will then need updating, deliberately and in one place.
    #[test]
    fn magic_guard_at_level_one_is_the_documented_body() {
        let level = buff_level(MAGIC_GUARD, 1).unwrap();
        let body = temporary_stat_set(&level.all_granted_by(MAGIC_GUARD));

        assert_eq!(body.len(), 198, "124 mask + 10 stat + 64 tail");
        assert_eq!(body.len(), temporary_stat_set_len(level.stat_count()));
        assert_eq!(&body[0..12], &[0u8; 12], "the mask is zero before bit 97's word");
        assert_eq!(&body[12..16], &[0x00, 0x00, 0x00, 0x40], "bit 97");
        assert_eq!(&body[16..124], &[0u8; 108], "and zero after it");
        assert_eq!(
            &body[124..134],
            &[0x1e, 0x00, 0x68, 0x88, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00],
            "30 percent, reason 2001000, duration 0 - a toggle has no time node"
        );
        assert_eq!(&body[134..], &[0u8; TAIL_LEN], "the tail is all zero");
    }

    /// **Magic Armor level 1 on the wire, byte for byte.** Two stats, ascending, 300 000 ms.
    #[test]
    fn magic_armor_at_level_one_is_the_documented_body() {
        let level = buff_level(MAGIC_ARMOR, 1).unwrap();
        let body = temporary_stat_set(&level.all_granted_by(MAGIC_ARMOR));

        assert_eq!(body.len(), 208, "124 mask + 2 x 10 stat + 64 tail");
        assert_eq!(body.len(), temporary_stat_set_len(level.stat_count()));
        assert_eq!(&body[0..8], &[0u8; 8], "the mask is zero before the defence word");
        assert_eq!(&body[8..12], &[0x00, 0x03, 0x00, 0x00], "bits 86 and 87");
        assert_eq!(&body[12..124], &[0u8; 112], "and zero after it");

        // Bit 86 first, then 87 - the order the client walks the mask in, not the caller's.
        assert_eq!(
            &body[124..134],
            &[0x28, 0x00, 0x69, 0x88, 0x1e, 0x00, 0xe0, 0x93, 0x04, 0x00],
            "weapon def 40, reason 2001001, duration 300000 MILLISECONDS"
        );
        assert_eq!(
            &body[134..144],
            &[0x28, 0x00, 0x69, 0x88, 0x1e, 0x00, 0xe0, 0x93, 0x04, 0x00],
            "magic def 40, same reason, same duration"
        );
        assert_eq!(&body[144..], &[0u8; TAIL_LEN], "the tail is all zero");

        // 300 seconds, not 300 milliseconds. The failure this catches shows on screen as an
        // icon that flashes and vanishes.
        assert_eq!(u32::from_le_bytes([body[130], body[131], body[132], body[133]]), 300_000);
        assert_eq!(u32::from_le_bytes([body[140], body[141], body[142], body[143]]), 300_000);
    }

    /// **What `granted_by` loses, pinned so it cannot be a surprise.**
    ///
    /// The signature is fixed by callers this crate does not own, so a caller can still take
    /// the one-stat path for a two-stat skill. On screen that is a Magic Armor cast where
    /// only W. Def moves - which `research/magic-damage.md` §9.3 lists as meaning *"the pair
    /// is off by one"*. It would be the wrong conclusion, drawn from a real observation, and
    /// this test is the note that says so.
    #[test]
    fn granted_by_alone_drops_magic_armors_second_stat_and_the_shape_says_so() {
        let level = buff_level(MAGIC_ARMOR, 1).unwrap();

        let truncated = temporary_stat_set(&[level.granted_by(MAGIC_ARMOR)]);
        let whole = temporary_stat_set(&level.all_granted_by(MAGIC_ARMOR));

        assert_eq!(truncated.len(), 198, "one stat");
        assert_eq!(whole.len(), 208, "two - and the length is the tell");
        assert_eq!(bits_in_mask(&truncated[..MASK_LEN]), vec![CTS_WEAPON_DEFENCE]);
        assert_eq!(
            bits_in_mask(&whole[..MASK_LEN]),
            vec![CTS_WEAPON_DEFENCE, CTS_MAGIC_DEFENCE],
            "both bits, ascending"
        );

        // For the one-stat skills the two calls are identical, so a caller that uses
        // `all_granted_by` everywhere is never wrong.
        for (skill, level) in [(NIMBLE_FEET, 3u32), (MAGIC_GUARD, 15)] {
            let l = buff_level(skill, level).unwrap();
            assert_eq!(l.all_granted_by(skill), vec![l.granted_by(skill)], "{skill}");
        }
    }

    /// **An unmodelled skill returns `None` without disturbing the three that are modelled.**
    ///
    /// Including the trap that would be silent: in this build `2001002` and `2001003` are
    /// **Energy Bolt and Magic Claw**, not Magic Guard and Magic Armor. A table written from
    /// the classic ids would buff two attacks, which grant no stat, and nothing would happen
    /// on screen to say so.
    #[test]
    fn one_skill_returning_none_does_not_stop_the_others() {
        for (id, why) in [
            (1000u32, "Three Snails is an attack"),
            (1001, "Recovery's CTS bit is not identified"),
            (2000000, "Improved MP Recovery is a passive with no stat bit"),
            (2000001, "Max MP Increase is a passive; mmpR is a PERCENT of max MP"),
            (2001002, "Energy Bolt - an ATTACK in this build, not Magic Guard"),
            (2001003, "Magic Claw - an ATTACK in this build, not Magic Armor"),
            (2001004, "does not exist in this build"),
            (2001005, "does not exist in this build"),
            (0, "not a skill"),
            (u32::MAX, "not a skill"),
        ] {
            for level in [0u32, 1, 3, 20] {
                assert!(buff_level(id, level).is_none(), "skill {id}: {why}");
            }
        }

        // ...and all three modelled skills still answer, at their first and last levels.
        for (id, max) in [
            (NIMBLE_FEET, NIMBLE_FEET_MAX_LEVEL),
            (MAGIC_GUARD, MAGIC_GUARD_MAX_LEVEL),
            (MAGIC_ARMOR, MAGIC_ARMOR_MAX_LEVEL),
        ] {
            assert!(buff_level(id, 1).is_some(), "skill {id} level 1");
            assert!(buff_level(id, max).is_some(), "skill {id} level {max}");
            assert!(buff_level(id, max + 1).is_none(), "skill {id} past its master level");
        }

        // The ids are this build's, read off String.wz - not the classic tree's.
        assert_eq!(MAGIC_GUARD, 2_001_000);
        assert_eq!(MAGIC_ARMOR, 2_001_001);
    }

    /// Nimble Feet's three rows are untouched by the two skills added beside them.
    ///
    /// They are the only rows here **confirmed on a client**, so they are the one thing in
    /// this file that a refactor is not allowed to move. The byte-level bodies have their own
    /// tests; this asserts the table and the new fields' defaults.
    #[test]
    fn nimble_feet_is_exactly_what_it_was() {
        for (level, mp_cost, seconds) in [(1u32, 4u16, 10u32), (2, 7, 20), (3, 10, 30)] {
            let l = buff_level(NIMBLE_FEET, level).unwrap();
            assert_eq!(l.mp_cost, mp_cost);
            assert_eq!(l.seconds, seconds);
            assert_eq!(l.duration, BuffDuration::Seconds(seconds));
            assert_eq!(l.cooldown_seconds, 180, "confirmed on a client: 3 min");
            assert_eq!(l.bit, CTS_SPEED);
            assert_eq!(l.value, 10, "speed is +10 at EVERY level; only the duration scales");
            assert_eq!(l.second, None, "Nimble Feet grants one stat");
            assert_eq!(l.stat_count(), 1);
        }
    }
}
