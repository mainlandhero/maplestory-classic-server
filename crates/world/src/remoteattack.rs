//! **`0x029E..=0x02A1` — a remote player swung, and everyone else on the map sees it.**
//!
//! The other half of the owner's 2026-08-29 sentence. `crates/world/src/broadcast.rs` quotes it
//! in full; the movement half is `Session::publish_user_move` and `net::userpool::0x0293`.
//! This module is the attack half:
//!
//! > *"their movements and their attacks need to be broadcasted and shown on all clients."*
//!
//! Tags on every claim: **[L]** read off this client's listing, **[D]** derived from two or
//! more [L], **[I]** inferred.
//!
//! # THIS HAS NEVER BEEN ON A WIRE
//!
//! `CLAUDE.md` § "Built is not wired", loudly, because on screen an unwired packet is
//! indistinguishable from one that does not exist.
//!
//! The archive was grepped with a **working positive control** rather than eyeballed:
//! `0x02FF` (mob move in) appears in **76** of the 428 files under `previous-runs/` and
//! `research/fixtures/`, and `0x00DF` in **57**, so the search can find a positive.
//! `0x029E`, `0x029F`, `0x02A0` and `0x02A1` appear in **no world log at all**. The four
//! hits that do exist are artefacts and are named here so nobody counts them twice:
//!
//! * `research/fixtures/sweep-01f2-03c7-exit.log` sends all four with a 32-byte junk body
//!   at `01:05:21`–`01:05:23`. That file's own header is `tools/handshake_probe.py`'s
//!   login-stage sweep on a fresh socket, so `CField::OnPacket` was never the active
//!   dispatcher and `FUN_1429b9300` was never reached. **It says nothing about any handler
//!   in this file.**
//! * `research/fixtures/probe-0080-1000-clean-miss.log` and
//!   `probe-walk-0080-065a-wrong-target.log` match `0x02A0` on the line
//!   `probe: at 0x02A0 (faults so far 68)` — the probe's own progress counter, not a packet.
//!
//! **My first control choice was wrong and is worth recording.** `0x03D9` — the mob-move
//! rebroadcast — was tried first and returned zero files, which looked like a broken grep.
//! It is not: `session/combat.rs::on_mob_move` says `0x03D9` is built and deliberately not
//! sent. A control has to be something known to be present, not merely something known to
//! exist in the source.
//!
//! # Which opcode, and how confident
//!
//! **`0x029E..=0x02A1`, four opcodes, one handler `FUN_1429d2ee0`. [L], and re-derived
//! here rather than taken from the note.** Table B at `0x1429bbc34` holds the *same stub*
//! `0x1429bb95e` at indices `0x00..0x03`, and the throttled path at `0x1429bbaea`
//! independently special-cases the same four before falling through. Two separate pieces
//! of control flow name the same set. `research/user-pool-tables.md` §4.
//!
//! **Which of the four means melee, shoot, magic or body is [I], and it costs nothing.**
//! `FUN_1429d2ee0` stores the opcode at `[rbp-0x74]` in its prologue **and never reads it
//! back** — a stack-slot scan for `(modrm & 0xC7) == 0x45 && disp8 == 0x8C` finds exactly
//! one access, the store, against a control of seven for the neighbouring slot `-0x7C`.
//! So the client's rendering cannot depend on which of the four carried it.
//! [`remote_attack_opcode`] mirrors the inbound order because that is the only ordering
//! with any argument behind it; if it is wrong, nothing on screen changes.
//!
//! # The body is a RE-ENCODE, not an echo — and that is the difference from the move
//!
//! `0x0293` copies the movement path byte for byte, because `research/user-move.md` §2
//! shows the same encoder `FUN_141d57c60` writes the block in both packets. **That licence
//! does not exist here.** The inbound attack's header is 40 fields (`net::attack::FIELD_WIDTHS`);
//! the broadcast's is **13** (`FUN_140f32200`). The inbound target block carries a 55-byte
//! middle, a `std::map` of pairs, an optional sub-object and a three-way mode; the outbound
//! carries **six** bytes of head, the hits, and **eight** bytes of tail. Echoing the inbound
//! body would desynchronise the client's stream on the first swing.
//!
//! What *is* true is stronger and more useful: **the two packets read and write the same
//! struct**, so every outbound field has an inbound source and the projection is mechanical.
//! Three independent offsets pin the correspondence, all **[L]**:
//!
//! | | inbound encoder | outbound decoder |
//! |---|---|---|
//! | target-list head | `[rdi+0] [rdi+4] [rdi+8]`, `140f31f75`.. | `[r15+0] [r15+4] [r15+8]`, `140f32523`.. |
//! | target array | `rdi + 0x10 + i*0x1d8`, `140f31fa6` | `r15 + 0x10 + i*0x1d8`, `140f3251f`/`140f3261f` |
//! | one hit | bools `[t+0x28] [t+0x29]`, `u64 [t+0x30]`, stride `0x10` | the same three, the same stride |
//!
//! The object id is `[t+0x10]` on both sides and the hit count is `[t+0x20]` on both sides.
//! Six coincidences is a shared type, not a resemblance. **[D]**
//!
//! ## The whole body
//!
//! ```text
//!   0  u32   charId        <- FUN_1429bb720 at 1429bb745, ONE direct read before dispatch
//!   4  u16   see below     <- 1429d2f3f, straight-line, -> [user + 0x406c]
//!   6  ...   43-byte header, 13 fields, FUN_140f32200 - NO branches between the reads
//!  49  u32   targetCount   <- the loop bound, 140f32546 `cmp [r15],r14d / jle`
//!  53  u32   list head b
//!  57  u32   list head c
//!  61  ...   targetCount target blocks:
//!              u32  objectId       <- ZERO SKIPS THE REST OF THE ENTRY, see below
//!              u16  hitCount
//!              hitCount x (u8 flagA, u8 flagB, u64 damage)
//!              u8, u8, u8, u16, u16, u8
//!            END - nothing follows, and there is no trailer
//! ```
//!
//! `tools/reads.py 0x1429d2ee0 6` reports **three** read sites and no more: the `u16`, the
//! header call and the target-list call. Reproduced at depth 1, 4 and 6. **[L]**
//!
//! ## `u32 objectId == 0` is a control-flow fork, and it is the one way to kill the client
//!
//! `140f32564 test eax,eax / je 140f3261c` jumps **straight to the loop tail**. An entry
//! whose object id is zero consumes four bytes and nothing else — so writing a zero id
//! followed by the rest of the block leaves the client reading our `u16 hitCount` as the
//! *next* entry's `u32 objectId`. There is no length prefix to resynchronise on. This
//! builder therefore **drops zero-id targets and shortens the count**, which is safe
//! because the count is only ever a loop bound (`140f32546`, `140f32626`). See
//! [`drawable_targets`].
//!
//! # The `u16` at body offset 4: the attacker's level
//!
//! `1429d2f47 mov dword [r14+0x406c], eax`. Scanning `0x142700000..0x142a00000` for
//! `[reg+0x406c]` gives 19 sites, and the one that settles this is
//! **`1429ce315`, inside `FUN_1429ce270`** — the `CUser::Init` decoder that `0x0224`
//! UserEnterField runs. `research/user-enter-field.md` §2 row 12 is that read: body offset
//! 12, `u32`, `+0x406c`, *"level [I]"*, and `net::userpool::user_enter_field` already fills
//! it with `chr.level`. **[L]** that the two packets write one dword.
//!
//! So the argument for sending the level here is **consistency, not identification**: two
//! packets set the same field, and sending a different number in the second would silently
//! change it on every swing. If `+0x406c` is not the level, this server is at least wrong
//! in one direction rather than two. Independently, the v214 reference's `UserRemote.attack`
//! encodes `chr.getLevel()` a few fields into the same packet — a **candidate only**, from a
//! tree `CLAUDE.md` scores 1 of 8, and its field order does not otherwise match this client
//! at all.
//!
//! **What is not established:** what `+0x406c` means. `0x1429c7536` initialises it to
//! `0x3e8` = 1000, which is not a plausible level, and `0x02A3` writes it from a `u8`.
//! Three widths for one dword across three packets. Nobody has read the four functions that
//! consume it (`FUN_1429d7250`, `0x1429d84f0`, `0x1429da0a0`, `0x1429db230`).
//!
//! # Damage: absolute, u64, and the unit question was asked
//!
//! `CLAUDE.md` § "The unit, not the arithmetic". The hit's damage is the `u64` at
//! `[t+0x30]` on **both** sides of the wire — `140f31c1d mov rdx,[rbx+7]` writes it,
//! `140f325b3` reads it back into the same slot at the same stride. Same field, same
//! width, no scaling: **an absolute damage number, exactly as the attacker's own client
//! computed it.** It is not a percentage. The percentage in this neighbourhood is
//! `0x03F0`'s mob HP, which is a different packet and already handled in
//! `net::combat::mob_hit_replies`.
//!
//! **The number sent is the attacker's claim, not the server's applied damage.** The
//! attacker's own screen already drew the claim — the client computed it and never asked
//! — so sending observers the server's clamped figure would make two screens disagree
//! about one swing. `session/combat.rs` still applies its own arithmetic to the mob's HP;
//! this packet is about what the animation draws.
//!
//! # The four widenings, and the one that is a sign
//!
//! Six of the thirteen header fields change width between the two encoders. Five are
//! harmless; **the position pair is not.**
//!
//! `x` and `y` live at struct `+0x34`/`+0x38`. The inbound encoder writes them
//! `movzx edx, word ptr [rbx+0x34]` — 16 bits out of a dword — and the outbound decoder
//! reads a **full `u32`** back into the same dword. `net::attack` already types them `i16`
//! because the corpus contains `0xFFD2` and `0xFF33`, which are −46 and −205 as map
//! coordinates and absurd as unsigned. So the faithful reconstruction of the sender's dword
//! is a **sign extension**, and zero-extending would put a character at x = 65490.
//! [`user_attack_remote`] sign-extends all four coordinate fields and
//! `a_negative_coordinate_is_sign_extended_not_zero_extended` pins it against the one
//! captured body that has a negative one. **[D]**

use net::attack::{Attack, AttackTarget};
use net::PacketWriter;

/// First of the four. `research/user-pool-tables.md` §4, table B index `0x00`. **[L]**
pub const USER_ATTACK_REMOTE_FIRST: u16 = net::userpool::USER_ATTACK_REMOTE_FIRST;

/// Last of the four, table B index `0x03`. **[L]**
pub const USER_ATTACK_REMOTE_LAST: u16 = net::userpool::USER_ATTACK_REMOTE_LAST;

/// `u32 charId` then the `u16` of the module docs.
pub const PREFIX_LEN: usize = 6;

/// `FUN_140f32200`: ten `u32` and three `u8`, straight-line, no branches. **[L]**
pub const HEADER_LEN: usize = 43;

/// `FUN_140f32440`'s three leading `u32`s, the first of which is the loop bound. **[L]**
pub const TARGET_LIST_HEAD_LEN: usize = 12;

/// One hit: `u8`, `u8`, `u64`. The same ten bytes the inbound packet spends on it. **[L]**
pub const HIT_LEN: usize = 10;

/// One target block minus its hits: `u32 objectId`, `u16 hitCount`, then the eight-byte
/// tail `u8 u8 u8 u16 u16 u8`. **[L]**
pub const TARGET_FIXED_LEN: usize = 14;

/// Every field of the inbound target block's 55 raw bytes, as `(struct offset, width)` in
/// **wire order**, read off `FUN_140f31bb0` at `0x140f31c34..0x140f31dd5`. **[L]**
///
/// `net::attack::AttackTarget::middle` carries those 55 bytes and does not decode them,
/// which is right for a parser and not enough for a re-encoder: the broadcast wants six of
/// these fields and they are scattered through it. This table is how their byte offsets are
/// **derived** rather than counted by hand — see [`middle_at`] and
/// `the_middle_map_reproduces_the_length_the_parser_measured`.
const MIDDLE_LAYOUT: [(u16, usize); 27] = [
    (0x118, 1), // setne
    (0x11c, 1),
    (0x120, 1),
    (0x124, 1), // -> the broadcast's third tail byte
    (0x128, 2),
    (0x12c, 2),
    (0x130, 2),
    (0x134, 2),
    (0x138, 2),
    (0x13c, 2),
    (0x148, 4), // -> the broadcast's first tail u16, truncated
    (0x14c, 4), // -> the broadcast's second tail u16, truncated
    (0x140, 1), // -> the broadcast's first tail byte
    (0x144, 1), // -> the broadcast's second tail byte
    (0x150, 1), // -> the broadcast's last tail byte
    (0x154, 4),
    (0x158, 1), // setne
    (0x15c, 1),
    (0x160, 1),
    (0x164, 1), // setne
    (0x168, 2),
    (0x16c, 2),
    (0x170, 2),
    (0x174, 2),
    (0x018, 4), // out of order in the struct, in order on the wire
    (0x01c, 4),
    (0x188, 4),
];

/// Byte offset of struct field `off` inside [`net::attack::AttackTarget::middle`].
///
/// `None` for an offset the inbound block does not carry, which is the honest answer and
/// not a zero.
const fn middle_at(off: u16) -> Option<usize> {
    let mut at = 0;
    let mut i = 0;
    while i < MIDDLE_LAYOUT.len() {
        if MIDDLE_LAYOUT[i].0 == off {
            return Some(at);
        }
        at += MIDDLE_LAYOUT[i].1;
        i += 1;
    }
    None
}

/// Which outbound opcode carries an inbound attack.
///
/// **The assignment is [I] and free.** `FUN_1429d2ee0` serves all four and never reads the
/// opcode back (module docs), so a wrong row here changes nothing the client draws. It
/// mirrors the inbound order, which is the only argument available.
///
/// `0x00E2` `USER_BODY_ATTACK` returns `None`: `net::combat::is_attack_opcode` excludes it,
/// nothing in this server ever parses one, and `net::attack::parse` refuses it because its
/// builder `FUN_1428d07c0` writes a different, undecoded layout. `0x02A1` is therefore the
/// one of the four this server will never send.
pub fn remote_attack_opcode(inbound: u16) -> Option<u16> {
    match inbound {
        net::combat::USER_MELEE_ATTACK => Some(0x029E),
        net::combat::USER_SHOOT_ATTACK => Some(0x029F),
        net::combat::USER_MAGIC_ATTACK => Some(0x02A0),
        _ => None,
    }
}

/// **How many targets the remote client can hold, and it does not check.**
///
/// `FUN_1429d2ee0` decodes the target list into a **fixed 15-slot array on its own stack
/// frame** at `rbp+0x1b0`, and nothing clamps the count it reads off the wire. Established
/// three independent ways, which is why this is **[L]** and not a guess:
///
/// * `lea r8d,[rdi+0xf]` - the slot arithmetic;
/// * the frame's `memset` length `0x1bb0`, which is exactly `8 + 15 * 0x1d8`;
/// * the callee's own `0x1ba8`, which is exactly `15 * 0x1d8`.
///
/// **Target index 15 writes from `rbp+0x1d68` to `rbp+0x1eac`.** The stack cookie sits at
/// `rbp+0x1d60` and the return address at `rbp+0x1db8`, so the sixteenth target steps over
/// both. That is a stack smash in **every other player's client**, from a 1.1 KB packet.
///
/// # Why this server is the thing that has to stop it
///
/// An honest client cannot produce one - its *own* attack builder uses the same fifteen
/// slots. But `CLAUDE.md`'s standing note is that **nothing authenticates**: the channel
/// socket carries no credentials, so "the client would not do that" is not a property this
/// server may rely on. `net::attack::parse` bounds the target count only by
/// `MAX_PACKET_LEN`, so roughly 246 000 targets parse, and [`user_attack_remote`] wrote one
/// block per target with no cap. One modified client could have smashed the stack of every
/// other player on its map.
///
/// So the clamp is here, at the **relay**, which is the only place that sees the packet on
/// its way to somebody else.
pub const MAX_REMOTE_TARGETS: usize = 15;

/// The targets this broadcast may carry, in order.
///
/// **A target whose object id is zero is dropped**, because the client's decoder treats a
/// zero id as "this entry is four bytes and nothing else" (`140f32564 test eax,eax / je`).
/// Keeping it and writing the whole block would desynchronise the stream; keeping it and
/// writing only the id would be a second encoding for the same list. Dropping it costs
/// nothing: nobody can see a mob with no object id, and the count field is only a loop
/// bound.
///
/// Zero ids have never been observed — `net::attack`'s corpus notes 391 of 434 bodies with
/// exactly one target and a real id — so this is a guard against a crafted or corrupt body,
/// which is exactly the kind this socket cannot rule out. **Nothing here authenticates
/// anybody.**
pub fn drawable_targets(attack: &Attack) -> impl Iterator<Item = &AttackTarget> {
    // **`take` is the clamp, and it is here rather than at the call sites on purpose.**
    // Three places compute over this list - the length, the body and the log line - and a
    // clamp applied at two of them is a packet whose declared count disagrees with its own
    // contents, which is worse than either whole answer. See [`MAX_REMOTE_TARGETS`].
    attack.targets.iter().filter(|t| t.object_id != 0).take(MAX_REMOTE_TARGETS)
}

/// Bytes [`user_attack_remote`] will produce, **recomputed from the parsed contents** rather
/// than from the writer's cursor.
///
/// This is the independent half of the byte accounting, the same shape
/// `net::attack::Attack::recomputed_len` has on the inbound side. A body of the wrong length
/// on this wire has killed this client twice.
pub fn body_len(attack: &Attack) -> usize {
    PREFIX_LEN
        + HEADER_LEN
        + TARGET_LIST_HEAD_LEN
        + drawable_targets(attack)
            .map(|t| TARGET_FIXED_LEN + hits_of(t).len() * HIT_LEN)
            .sum::<usize>()
}

/// The hits a target block will carry. Clamped so the `u16` count and the number of blocks
/// written can never disagree, whatever a hostile body claimed.
fn hits_of(t: &AttackTarget) -> &[net::attack::AttackHit] {
    let n = t.hits.len().min(u16::MAX as usize);
    &t.hits[..n]
}

/// Build the body of a `0x029E..=0x02A1`: **what the other clients on this map are shown.**
///
/// `char_id` is the attacker and `level` is the value for `[user + 0x406c]` — send the same
/// number `net::userpool::user_enter_field` put at its body offset 12, which is the
/// character's level. See the module docs for why consistency is the argument.
///
/// The header and the target list are projected field for field out of `attack`; nothing is
/// invented and nothing is echoed verbatim. Two things deliberately do **not** cross:
///
/// * **the inbound body's own bytes**, because the two encoders are not symmetric — 40
///   header fields in, 13 out;
/// * **anything the receiving client resolves for itself** — the pair map, the optional
///   sub-object, the three-way mode and the trailer are all inbound-only and the broadcast's
///   decoder never reads them.
///
/// The attacker's character id is supplied by the *caller* from its own claimed migration,
/// never taken from the packet: nothing on this socket authenticates anybody, so a body
/// claiming to be somebody else must not be able to move somebody else's character.
pub fn user_attack_remote(char_id: u32, level: u16, attack: &Attack) -> Vec<u8> {
    let h = &attack.header;
    let mut w = PacketWriter::new();

    w.u32(char_id); //                     0   1429bb745, before the dispatch
    w.u16(level); //                       4   1429d2f3f -> [user + 0x406c]

    // -- FUN_140f32200, 13 fields, in the order it reads them --------------------------
    w.u32(h.skill_id); //                  +0x08  skill id
    w.u32(u32::from(h.skill_level)); //    +0x0c  u8 in, u32 out
    w.u8(u8::from(h.f7)); //               +0x1c  the inbound `setne` at 140f32043
    w.u32(widen(h.x)); //                  +0x34  x   - SIGN extended, see the module docs
    w.u32(widen(h.y)); //                  +0x38  y   - likewise
    w.u32(widen(h.f16)); //                +0x40  the second position pair
    w.u32(widen(h.f17)); //                +0x44
    w.u32(h.f8); //                        +0x20
    w.u8(h.f11 as u8); //                  +0x2c  u32 in, u8 out: 4 or 6 in all 434 bodies
    w.u8(h.f19); //                        +0x48  0 in all 434
    w.u32(h.f22); //                       +0x50  0 in all 434
    w.u32(h.f23); //                       +0x54  0 in all 434
    w.u32(h.f25); //                       +0x60  0 in all 434

    // -- FUN_140f32440's head. The FILTERED count, or the walk desynchronises. ----------
    let targets: Vec<&AttackTarget> = drawable_targets(attack).collect();
    w.u32(targets.len() as u32); //        [r15+0]  the loop bound at 140f32546
    w.u32(attack.list_head_b); //          [r15+4]
    w.u32(attack.list_head_c); //          [r15+8]

    for t in targets {
        let hits = hits_of(t);
        w.u32(t.object_id); //             [t+0x10]  never zero here - drawable_targets
        w.u16(hits.len() as u16); //       [t+0x20]  u8 in, u16 out
        for hit in hits {
            w.u8(u8::from(hit.flag_a)); // [t+0x28]
            w.u8(u8::from(hit.flag_b)); // [t+0x29]  the critical flag
            w.u64(hit.damage); //          [t+0x30]  ABSOLUTE, not a percentage
        }
        // The eight-byte tail, six fields the inbound packet carries inside its 55 raw
        // bytes. `middle_at` derives each offset from MIDDLE_LAYOUT rather than hard-coding
        // it, so a mis-transcribed width fails a test instead of shifting the tail.
        w.u8(middle_u8(t, 0x140));
        w.u8(middle_u8(t, 0x144));
        w.u8(middle_u8(t, 0x124));
        w.u16(middle_u32(t, 0x148) as u16); // u32 in, u16 out
        w.u16(middle_u32(t, 0x14c) as u16);
        w.u8(middle_u8(t, 0x150));
    }

    let body = w.into_vec();
    debug_assert_eq!(body.len(), body_len(attack), "the body length is wrong");
    body
}

/// Sign-extend a coordinate into the dword the broadcast's decoder reads.
///
/// The whole reason this is a named function with a doc comment: `as u32` on an `i16` in
/// Rust already sign-extends, so the correct code and the incorrect code differ by an
/// invisible `as u16` in the middle. Naming it makes the choice reviewable.
const fn widen(v: i16) -> u32 {
    v as i32 as u32
}

/// One `u8` out of the inbound target's undecoded middle block, by **struct** offset.
///
/// `0` when the offset is not in the block or the block is short, which cannot happen —
/// `middle` is a fixed `[u8; 55]` — and is written as a fallback rather than an `unwrap`
/// because a panic here would take down a live connection over a field nobody has named.
fn middle_u8(t: &AttackTarget, off: u16) -> u8 {
    match middle_at(off) {
        Some(at) => t.middle.get(at).copied().unwrap_or(0),
        None => 0,
    }
}

/// One `u32` out of the same block, little-endian.
fn middle_u32(t: &AttackTarget, off: u16) -> u32 {
    let Some(at) = middle_at(off) else { return 0 };
    match t.middle.get(at..at + 4) {
        Some(b) => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        None => 0,
    }
}

/// A one-line summary for the `Reply::what` field and the world log.
///
/// Says what crossed and what did not, because a log line that only says "sent" is how
/// `CLAUDE.md`'s "built is not wired" happens in the first place.
pub fn describe(char_id: u32, opcode: u16, attack: &Attack, len: usize) -> String {
    let targets: Vec<&AttackTarget> = drawable_targets(attack).collect();
    let dropped = attack.targets.len() - targets.len();
    let damage: u64 = targets
        .iter()
        .fold(0u64, |acc, t| acc.saturating_add(t.total_damage()));
    format!(
        "UserAttackRemote {opcode:#06x}: character {char_id} hit {} mob(s) for {damage} \
         (skill {}, level {}), {len} bytes re-encoded from the inbound {:#06x} - 13 header \
         fields of 40, no trailer{}. NEVER SEEN ON A WIRE: research/user-pool-tables.md \
         §4 is static analysis. Nothing here authenticates anybody.",
        targets.len(),
        attack.header.skill_id,
        attack.header.skill_level,
        attack.opcode,
        if dropped > 0 {
            format!(", {dropped} zero-id target(s) DROPPED - 140f32564 would skip them")
        } else {
            String::new()
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::attack::parse;

    // ---------------------------------------------------------------------------------
    // Real captured bodies, copied with their provenance from `net::attack`'s own test
    // module (they are `#[cfg(test)]` there and cannot be imported). Each line names the
    // log, the timestamp and the length, so any of them can be found again.
    // ---------------------------------------------------------------------------------

    /// `previous-runs/world-20260820-121055.log` 16:10:28.598, `0x00DF`, 229 bytes.
    /// A plain swing that connected: mob 2002, one hit of 19, not critical.
    const MELEE_229: &str = "0001000000000000000000000000000001050000009fae340801040000003b80680a70028b010000000070028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d20700000200000001000013000000000000000000000736028b0136028b0135027b01890100000000000001000002000000000001012302710148028b01000000007e6c3c6600000000030000000000d5c057820100000092e9bc2707000000bc6509e5000080e8da8f00";

    /// `previous-runs/world-20260820-181822.log` 22:11:21.501, `0x00DF`, 229 bytes.
    /// **A critical**: mob 2002, one hit of 24 with `flag_b` set.
    const MELEE_229_CRIT: &str = "0001000000000000000000000000000000050000009fae3408010400000034e8b20bb7028b0100000000b7028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d2070000020000000100011800000000000000010700070d038b010d038b010303770189010000000000000100000200000000000101f8026a0123038b01000000007e6c3c6600000000030000000000d5c057820100000092e9bc2707000000bc6509e5000080e8da8f00";

    /// `world.log` 04:01:06.251, `0x00E1`, 260 bytes. Magic Claw, level 7, mob 2006,
    /// **two** hits of 1 each, trailer object present.
    const MAGIC_CLAW_260: &str = "00016b881e000700b01862ebc1a13ab701070000009fae340801060000007a88791a05fdd7000000000005fdbb000000000000000000000000000000000000000000000000000000010000000100000000170055736572204d6167696320536b696c6c2053797374656d0000000000000000000000000000000000010000000000000000000000d60700000a000000020000010000000000000000000100000000000000010002077dfcd70081fcd7007dfcbb00c2010000000000000200000a000000000001015cfca200a2fcd300000000004d5a227e00000000030000000000c4a264860100000035ec090102000000011ef164000080e8da8f0100000000ffffffff";

    /// `previous-runs/world-20260819-222734.log` 02:25:35.692, `0x00DF`, 127 bytes.
    /// The zero-target shape.
    const MELEE_EMPTY_127: &str = "0000000000000000000000000000000000050000009fae340801040000005a437507e5028b0100000000e5028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c6565890100000000000000000000000000000000000000000000000000000080e8da8f00";

    /// `previous-runs/world-20260819-222734.log` 02:25:52.054, `0x00DF`, 127 bytes.
    /// **The only body in the whole 434-body corpus with a negative coordinate**, which is
    /// what makes it the fixture the sign-extension claim can fail against.
    const MELEE_EMPTY_127_ODD: &str = "000000000000000000000000000000000119000000a9c601dd010400000056837507c10533ff17000000c10533ff00000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c6565060100000000000000000000000000000000000000000000000000000080e8da8f00";

    fn bytes(hex: &str) -> Vec<u8> {
        assert!(hex.len() % 2 == 0, "odd-length fixture hex");
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("fixture hex"))
            .collect()
    }

    fn attack(hex: &str, opcode: u16) -> Attack {
        let b = bytes(hex);
        let a = parse(opcode, &b).expect("a real captured body must parse");
        assert!(a.truncated.is_none(), "fixture parsed truncated: {:?}", a.truncated);
        assert_eq!(a.consumed, b.len(), "the fixture parser stopped early");
        a
    }

    fn le32(b: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
    }
    fn le16(b: &[u8], at: usize) -> u16 {
        u16::from_le_bytes(b[at..at + 2].try_into().unwrap())
    }
    fn le64(b: &[u8], at: usize) -> u64 {
        u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
    }

    // -- the tables, which are where every byte offset comes from ---------------------

    /// The middle map is a **claim about a listing**, so it is asserted against something
    /// that can disagree: the length `net::attack`'s parser independently measured from
    /// the wire. `research/mob-combat.md` §1.6 counts the same fields as
    /// "4 u8, 6 u16, 2 u32, 3 u8, 1 u32, 4 u8, 4 u16, 3 u32".
    #[test]
    fn the_middle_map_reproduces_the_length_the_parser_measured() {
        let total: usize = MIDDLE_LAYOUT.iter().map(|(_, w)| w).sum();
        assert_eq!(total, net::attack::TARGET_MIDDLE_LEN);
        assert_eq!(total, 55);
        assert_eq!(MIDDLE_LAYOUT.len(), 27, "27 writes at 140f31c34..140f31dd5");

        // The six the broadcast wants, and the four that must NOT collide with them.
        assert_eq!(middle_at(0x124), Some(3));
        assert_eq!(middle_at(0x148), Some(16));
        assert_eq!(middle_at(0x14c), Some(20));
        assert_eq!(middle_at(0x140), Some(24));
        assert_eq!(middle_at(0x144), Some(25));
        assert_eq!(middle_at(0x150), Some(26));
        assert_eq!(middle_at(0x118), Some(0), "the block starts at +0x118");
        assert_eq!(middle_at(0x188), Some(51), "and its last field is a u32");
        assert_eq!(middle_at(0x999), None, "an offset the block does not carry");
    }

    /// Four opcodes, one handler, and the one this server will never send.
    #[test]
    fn the_opcodes_are_the_four_table_b_serves() {
        assert_eq!(USER_ATTACK_REMOTE_FIRST, 0x029E);
        assert_eq!(USER_ATTACK_REMOTE_LAST, 0x02A1);
        assert_eq!(remote_attack_opcode(net::combat::USER_MELEE_ATTACK), Some(0x029E));
        assert_eq!(remote_attack_opcode(net::combat::USER_SHOOT_ATTACK), Some(0x029F));
        assert_eq!(remote_attack_opcode(net::combat::USER_MAGIC_ATTACK), Some(0x02A0));
        assert_eq!(
            remote_attack_opcode(net::combat::USER_BODY_ATTACK),
            None,
            "0x00E2 uses an undecoded layout and net::attack refuses it"
        );
        assert_eq!(remote_attack_opcode(0x0000), None);
        for op in [0x029Eu16, 0x029F, 0x02A0, 0x02A1] {
            assert!(net::userpool::is_user_pool(op), "{op:#06x} must reach CUserPool");
            assert!((USER_ATTACK_REMOTE_FIRST..=USER_ATTACK_REMOTE_LAST).contains(&op));
        }
        // Every opcode this server can produce is one of the four, and none of them is
        // the movement opcode - a collision there would move a character on every swing.
        for inbound in [0x00DFu16, 0x00E0, 0x00E1] {
            let out = remote_attack_opcode(inbound).unwrap();
            assert_ne!(out, net::userpool::USER_MOVE_REMOTE);
        }
    }

    // -- the body, field for field, against a real captured swing ----------------------

    /// **The whole layout in one test.** Every offset is asserted against the value the
    /// inbound parser read, so a field written in the wrong order fails here rather than
    /// on a client.
    #[test]
    fn a_real_swing_becomes_the_body_the_listing_describes() {
        let a = attack(MELEE_229, net::combat::USER_MELEE_ATTACK);
        assert_eq!(a.targets.len(), 1, "the fixture has exactly one target");
        assert_eq!(a.targets[0].object_id, 2002);
        assert_eq!(a.targets[0].hits.len(), 1);
        assert_eq!(a.targets[0].hits[0].damage, 19);

        let b = user_attack_remote(207, 33, &a);

        assert_eq!(le32(&b, 0), 207, "charId at 0 - 1429bb745");
        assert_eq!(le16(&b, 4), 33, "the u16 at 4 - 1429d2f3f");

        // The 13-field header, in FUN_140f32200's read order.
        assert_eq!(le32(&b, 6), a.header.skill_id, "+0x08 skill id");
        assert_eq!(le32(&b, 10), u32::from(a.header.skill_level), "+0x0c level");
        assert_eq!(b[14], u8::from(a.header.f7), "+0x1c the setne");
        assert_eq!(le32(&b, 15), widen(a.header.x), "+0x34 x");
        assert_eq!(le32(&b, 19), widen(a.header.y), "+0x38 y");
        assert_eq!(le32(&b, 23), widen(a.header.f16), "+0x40");
        assert_eq!(le32(&b, 27), widen(a.header.f17), "+0x44");
        assert_eq!(le32(&b, 31), a.header.f8, "+0x20");
        assert_eq!(b[35], a.header.f11 as u8, "+0x2c u32 in, u8 out");
        assert_eq!(b[36], a.header.f19, "+0x48");
        assert_eq!(le32(&b, 37), a.header.f22, "+0x50");
        assert_eq!(le32(&b, 41), a.header.f23, "+0x54");
        assert_eq!(le32(&b, 45), a.header.f25, "+0x60");
        assert_eq!(PREFIX_LEN + HEADER_LEN, 49, "the target list starts at 49");

        // The target list head.
        assert_eq!(le32(&b, 49), 1, "one target - the loop bound at 140f32546");
        assert_eq!(le32(&b, 53), a.list_head_b);
        assert_eq!(le32(&b, 57), a.list_head_c);

        // The one target block.
        assert_eq!(le32(&b, 61), 2002, "[t+0x10] object id");
        assert_eq!(le16(&b, 65), 1, "[t+0x20] hit count, u8 in and u16 out");
        assert_eq!(b[67], u8::from(a.targets[0].hits[0].flag_a), "[t+0x28]");
        assert_eq!(b[68], u8::from(a.targets[0].hits[0].flag_b), "[t+0x29]");
        assert_eq!(le64(&b, 69), 19, "[t+0x30] damage, absolute and 64-bit");

        // The eight-byte tail, projected out of the inbound 55-byte middle.
        let m = &a.targets[0].middle;
        assert_eq!(b[77], m[24], "+0x140");
        assert_eq!(b[78], m[25], "+0x144");
        assert_eq!(b[79], m[3], "+0x124");
        assert_eq!(le16(&b, 80), le32(m, 16) as u16, "+0x148, u32 in and u16 out");
        assert_eq!(le16(&b, 82), le32(m, 20) as u16, "+0x14c");
        assert_eq!(b[84], m[26], "+0x150");

        assert_eq!(b.len(), 85, "6 + 43 + 12 + (6 + 10 + 8)");
        assert_eq!(b.len(), body_len(&a), "and the predictor agrees");
    }

    /// **The body is SHORTER than the inbound one, and by a lot.** This is the claim that
    /// separates a re-encode from an echo: if a later edit ever "simplified" this into a
    /// copy of the inbound bytes, the length would jump from 85 to 229.
    #[test]
    fn the_broadcast_is_a_reduced_re_encode_and_not_an_echo() {
        for (name, hex, op) in [
            ("melee_229", MELEE_229, net::combat::USER_MELEE_ATTACK),
            ("magic_claw_260", MAGIC_CLAW_260, net::combat::USER_MAGIC_ATTACK),
            ("melee_empty_127", MELEE_EMPTY_127, net::combat::USER_MELEE_ATTACK),
        ] {
            let inbound = bytes(hex);
            let a = attack(hex, op);
            let b = user_attack_remote(207, 33, &a);
            assert!(
                b.len() < inbound.len(),
                "{name}: {} out vs {} in - the outbound header is 13 fields of 40",
                b.len(),
                inbound.len()
            );
            // And it is not a prefix of the inbound body either, which a lazy "truncate
            // the echo" would be.
            assert_ne!(&b[..], &inbound[..b.len()], "{name}: this is not a truncation");
        }
    }

    /// The unit test, in the sense `CLAUDE.md` means it. The one captured body with a
    /// negative coordinate: `0xFF33` is −205, and zero-extending it would put the attacker
    /// at x = 65331.
    #[test]
    fn a_negative_coordinate_is_sign_extended_not_zero_extended() {
        let a = attack(MELEE_EMPTY_127_ODD, net::combat::USER_MELEE_ATTACK);
        assert_eq!(a.header.y, -205, "the fixture's y really is negative");
        assert_eq!(a.header.f17, -205);

        let b = user_attack_remote(207, 33, &a);
        assert_eq!(le32(&b, 19), 0xFFFF_FF33, "+0x38 y sign-extended");
        assert_eq!(le32(&b, 19) as i32, -205);
        assert_ne!(le32(&b, 19), 0x0000_FF33, "zero-extension is the bug this pins");
        assert_eq!(le32(&b, 27) as i32, -205, "+0x44 too");
        // ...and a positive one is unchanged, so the fix is not a blanket sign flip.
        assert_eq!(le32(&b, 15) as i32, i32::from(a.header.x));
        assert!(a.header.x > 0);
    }

    /// Two hits on one target: the ten-byte stride, and both damages arriving. A stride
    /// bug shows up here and nowhere else, because every other fixture has one hit.
    #[test]
    fn a_two_hit_cast_carries_both_damages_at_the_ten_byte_stride() {
        let a = attack(MAGIC_CLAW_260, net::combat::USER_MAGIC_ATTACK);
        assert_eq!(a.header.skill_id, 2_001_003, "Magic Claw");
        assert_eq!(a.targets.len(), 1);
        assert_eq!(a.targets[0].hits.len(), 2, "Magic Claw hits twice");

        let b = user_attack_remote(207, 33, &a);
        assert_eq!(le16(&b, 65), 2, "the count says two");
        assert_eq!(le64(&b, 69), a.targets[0].hits[0].damage);
        assert_eq!(le64(&b, 69 + HIT_LEN), a.targets[0].hits[1].damage);
        assert_eq!(b.len(), 85 + HIT_LEN, "exactly one more hit than a melee swing");
        assert_eq!(b.len(), body_len(&a));
        // The skill reaches the other client, which is what picks the animation.
        assert_eq!(le32(&b, 6), 2_001_003);
        assert_eq!(le32(&b, 10), 7, "and its level");
    }

    /// The critical flag is a per-hit byte and it crosses. Asserted against the one capture
    /// `research/damage-formula.md` §3.2 draws the crit finding from, and against a
    /// non-critical sibling in the same test so "always true" cannot pass.
    #[test]
    fn the_critical_flag_crosses_and_a_plain_hit_does_not_set_it() {
        let crit = attack(MELEE_229_CRIT, net::combat::USER_MELEE_ATTACK);
        assert!(crit.targets[0].hits[0].flag_b, "the fixture is the critical one");
        assert_eq!(user_attack_remote(207, 33, &crit)[68], 1);
        assert_eq!(le64(&user_attack_remote(207, 33, &crit), 69), 24);

        let plain = attack(MELEE_229, net::combat::USER_MELEE_ATTACK);
        assert!(!plain.targets[0].hits[0].flag_b);
        assert_eq!(user_attack_remote(207, 33, &plain)[68], 0);
    }

    /// A swing at empty air is a real, common packet - 43 of the 434 captured bodies - and
    /// it must still broadcast, or a miss looks like a frozen character to everyone else.
    #[test]
    fn a_swing_that_hit_nothing_is_still_a_valid_body() {
        let a = attack(MELEE_EMPTY_127, net::combat::USER_MELEE_ATTACK);
        assert_eq!(a.target_count, 0);

        let b = user_attack_remote(207, 33, &a);
        assert_eq!(le32(&b, 49), 0, "no targets");
        assert_eq!(b.len(), PREFIX_LEN + HEADER_LEN + TARGET_LIST_HEAD_LEN);
        assert_eq!(b.len(), 61);
        assert_eq!(b.len(), body_len(&a));
    }

    /// **The desynchronisation guard.** `140f32564 test eax,eax / je 140f3261c` makes a
    /// zero object id four bytes and nothing else, so a zero-id entry written in full would
    /// have the client read our `u16` hit count as the next entry's id. The count must
    /// shrink with the list.
    #[test]
    fn a_zero_object_id_target_is_dropped_and_the_count_follows_it() {
        let mut a = attack(MELEE_229, net::combat::USER_MELEE_ATTACK);
        let good = a.targets[0].clone();
        let mut zero = good.clone();
        zero.object_id = 0;
        // A hostile body with a zero-id entry in the middle of two real ones.
        a.targets = vec![good.clone(), zero, good];
        a.target_count = 3;

        assert_eq!(drawable_targets(&a).count(), 2, "the zero-id one is not drawable");
        let b = user_attack_remote(207, 33, &a);
        assert_eq!(le32(&b, 49), 2, "the COUNT shrinks with the list, or the walk desyncs");
        assert_eq!(le32(&b, 61), 2002, "the first survivor");
        // 61 + one block (24) = 85 is where the second one starts.
        assert_eq!(le32(&b, 85), 2002, "the second survivor, immediately after");
        assert_eq!(b.len(), 61 + 24 * 2);
        assert_eq!(b.len(), body_len(&a));
    }

    /// `target_count` is what the sender *claimed*; `targets` is what the parser *read*.
    /// A truncated parse leaves the two disagreeing, and writing the claim would promise
    /// blocks that are not there.
    #[test]
    fn the_count_written_is_the_number_of_blocks_written_not_the_senders_claim() {
        let mut a = attack(MELEE_229, net::combat::USER_MELEE_ATTACK);
        a.target_count = 9; // the sender claimed nine and the parser read one
        a.truncated = Some(net::attack::AttackTruncation::ShortTarget { index: 1, offset: 0 });

        let b = user_attack_remote(207, 33, &a);
        assert_eq!(le32(&b, 49), 1, "one block written, so one is promised");
        assert_ne!(le32(&b, 49), 9);
        assert_eq!(b.len(), 85, "and the body still adds up");
        assert_eq!(b.len(), body_len(&a));
    }

    /// Every fixture: the writer's cursor and the independent recomputation agree, and the
    /// body is never empty. The inbound side has the identical check for the identical
    /// reason - a wrong length on this wire has killed this client twice.
    #[test]
    fn every_fixture_round_trips_its_own_length() {
        for (name, hex, op) in [
            ("melee_229", MELEE_229, net::combat::USER_MELEE_ATTACK),
            ("melee_229_crit", MELEE_229_CRIT, net::combat::USER_MELEE_ATTACK),
            ("magic_claw_260", MAGIC_CLAW_260, net::combat::USER_MAGIC_ATTACK),
            ("melee_empty_127", MELEE_EMPTY_127, net::combat::USER_MELEE_ATTACK),
            ("melee_empty_127_odd", MELEE_EMPTY_127_ODD, net::combat::USER_MELEE_ATTACK),
        ] {
            let a = attack(hex, op);
            let b = user_attack_remote(207, 33, &a);
            assert_eq!(b.len(), body_len(&a), "{name}");
            assert!(b.len() >= 61, "{name}: shorter than the head");
            assert_eq!(le32(&b, 0), 207, "{name}: the id the CALLER supplied");
            // The count and the blocks must agree, walked independently of the writer.
            let count = le32(&b, 49) as usize;
            let mut at = 61;
            for _ in 0..count {
                let hits = le16(&b, at + 4) as usize;
                at += TARGET_FIXED_LEN + hits * HIT_LEN;
            }
            assert_eq!(at, b.len(), "{name}: the walk must land on the end of the body");
        }
    }

    /// The character id comes from the caller, never from the packet. Nothing on this
    /// socket authenticates anybody, so a body cannot be allowed to name a victim.
    #[test]
    fn the_character_id_is_the_callers_and_nothing_in_the_body_can_change_it() {
        let a = attack(MELEE_229, net::combat::USER_MELEE_ATTACK);
        for id in [200u32, 207, 0x1234_5678] {
            assert_eq!(le32(&user_attack_remote(id, 33, &a), 0), id);
        }
        assert_eq!(le16(&user_attack_remote(200, 0, &a), 4), 0);
        assert_eq!(le16(&user_attack_remote(200, 200, &a), 4), 200);
    }

    // -- the bus: who receives it, who does not, and what must not coalesce -------------

    use crate::broadcast::{Bus, Presence};
    use crate::session::Reply;

    fn presence(character: u32, map: u32) -> Presence {
        Presence {
            character,
            map,
            spawn: Reply { opcode: 0x0224, body: Vec::new(), what: format!("spawn {character}") },
            farewell: Reply {
                opcode: 0x0225,
                body: Vec::new(),
                what: format!("farewell {character}"),
            },
            companions: Vec::new(),
        }
    }

    fn swing(char_id: u32) -> Reply {
        let a = attack(MELEE_229, net::combat::USER_MELEE_ATTACK);
        Reply {
            opcode: remote_attack_opcode(net::combat::USER_MELEE_ATTACK).unwrap(),
            body: user_attack_remote(char_id, 33, &a),
            what: describe(char_id, 0x029E, &a, 85),
        }
    }

    /// The three delivery claims in one test, because `CLAUDE.md` is explicit that a test
    /// covering one of N effects gives false confidence about the rest: **the other player
    /// receives it, the attacker does not receive their own, and the body that arrives is
    /// the body that was built.**
    #[test]
    fn a_swing_reaches_the_other_player_and_not_the_attacker() {
        let bus = Bus::new();
        let watcher = bus.join();
        let attacker = bus.join();
        bus.enter_field(watcher, presence(200, 104_040_000));
        bus.enter_field(attacker, presence(201, 104_040_000));
        let _ = bus.drain(watcher);
        let _ = bus.drain(attacker);

        let sent = swing(201);
        bus.publish(attacker, 104_040_000, sent.clone(), None);

        let mail = bus.drain(watcher);
        assert_eq!(mail.len(), 1, "the watcher should be told: {mail:?}");
        assert_eq!(mail[0].opcode, 0x029E);
        assert_eq!(mail[0].body, sent.body, "byte for byte what was built");
        assert_eq!(le32(&mail[0].body, 0), 201, "and it names the attacker");
        assert!(
            bus.drain(attacker).is_empty(),
            "the attacker must NOT be sent their own swing - their client drew it already"
        );
    }

    /// **An attack is an event, not a state.** `Bus::publish`'s `supersedes` must be
    /// `None`, and this is what proves it matters: three swings published the way movement
    /// is published collapse into one, and a fight renders as a single hit.
    ///
    /// Both halves are asserted in one test so the contrast is the measurement - asserting
    /// only that three survive would also pass against a bus that had stopped superseding
    /// anything at all, which is a different bug.
    #[test]
    fn three_swings_are_three_packets_and_would_not_be_with_a_supersede_key() {
        let bus = Bus::new();
        let watcher = bus.join();
        let attacker = bus.join();
        bus.enter_field(watcher, presence(200, 7));
        bus.enter_field(attacker, presence(201, 7));
        let _ = bus.drain(watcher);

        for _ in 0..3 {
            bus.publish(attacker, 7, swing(201), None);
        }
        assert_eq!(bus.drain(watcher).len(), 3, "two swings are two events");

        // The same three with the movement key, to show the coalescing is real and is
        // exactly what `None` is avoiding.
        for _ in 0..3 {
            bus.publish(attacker, 7, swing(201), Some(201));
        }
        assert_eq!(
            bus.drain(watcher).len(),
            1,
            "with a supersede key a whole fight would render as one hit"
        );
    }

    /// A player on another map hears nothing, and a connection that has not entered a
    /// field collects no backlog. Both are properties of the bus rather than of this
    /// module, and both are asserted here because this is the first packet that will
    /// exercise them at a real cadence.
    #[test]
    fn a_swing_does_not_cross_maps_or_reach_a_connection_in_no_field() {
        let bus = Bus::new();
        let elsewhere = bus.join();
        let waiting = bus.join();
        let attacker = bus.join();
        bus.enter_field(elsewhere, presence(200, 100_000_000));
        bus.enter_field(attacker, presence(201, 104_040_000));
        let _ = bus.drain(elsewhere);

        bus.publish(attacker, 104_040_000, swing(201), None);

        assert!(bus.drain(elsewhere).is_empty(), "a different map is a different field");
        assert!(bus.drain(waiting).is_empty(), "no field, no mail");
    }

    /// The log line has to say the thing that is easy to forget. `CLAUDE.md`: an unwired
    /// packet looks exactly like one that does not exist, and a line that only says "sent"
    /// is how that happens.
    #[test]
    fn the_log_line_names_the_damage_and_admits_it_is_unproven() {
        let a = attack(MELEE_229, net::combat::USER_MELEE_ATTACK);
        let line = describe(207, 0x029E, &a, 85);
        assert!(line.contains("207"), "{line}");
        assert!(line.contains("19"), "the damage: {line}");
        assert!(line.contains("NEVER SEEN ON A WIRE"), "{line}");
        assert!(line.contains("authenticates"), "{line}");

        // ...and it says so when a target was dropped, rather than dropping it in silence.
        let mut hostile = a.clone();
        hostile.targets[0].object_id = 0;
        assert!(describe(207, 0x029E, &hostile, 61).contains("DROPPED"));
    }

    /// **A sixteenth target would smash the receiving client's stack, so there is never one.**
    ///
    /// `FUN_1429d2ee0` decodes the target list into a fixed **15**-slot array on its own
    /// frame at `rbp+0x1b0` and checks no bound. Index 15 writes from `rbp+0x1d68` to
    /// `rbp+0x1eac`, over the stack cookie at `rbp+0x1d60` and past the return address at
    /// `rbp+0x1db8`.
    ///
    /// An honest client cannot send one - its own builder uses the same fifteen slots - but
    /// **nothing authenticates the channel socket**, and `net::attack::parse` bounds the
    /// target count only by the 16 MiB packet limit. So the relay is the only thing standing
    /// between one modified client and every other player's process.
    ///
    /// The assertion is on the **declared count, the block count and the length together**,
    /// because a clamp applied to some of those and not the others produces a packet whose
    /// header disagrees with its own body - which is a worse packet than the one it replaced.
    #[test]
    fn a_swing_at_more_targets_than_the_client_can_hold_is_clamped() {
        let mut a = attack(MELEE_229, net::combat::USER_MELEE_ATTACK);
        // The control: the fixture is a real captured one-target swing, and it is not
        // clamped. A test that only checks the ceiling passes on a builder that emits
        // nothing at all.
        assert_eq!(drawable_targets(&a).count(), 1, "the fixture is one real target");

        // Twenty targets, all with distinct non-zero object ids so `drawable_targets`
        // cannot be the thing shortening the list.
        let one = a.targets[0].clone();
        a.targets = (0..20u32)
            .map(|n| {
                let mut t = one.clone();
                t.object_id = 2000 + n;
                t
            })
            .collect();
        assert_eq!(a.targets.len(), 20, "the parser itself imposes no ceiling");

        assert_eq!(
            drawable_targets(&a).count(),
            MAX_REMOTE_TARGETS,
            "the relay must offer at most the fifteen slots the client has"
        );

        let body = user_attack_remote(200, 1, &a);
        // The declared count, at the head of the target list.
        // `PREFIX_LEN + HEADER_LEN` is 49, asserted by
        // `the_body_is_the_prefix_the_header_and_the_target_list`, so the count sits there.
        let declared = le32(&body, PREFIX_LEN + HEADER_LEN);
        assert_eq!(
            declared as usize, MAX_REMOTE_TARGETS,
            "the count the client will loop on"
        );
        // And the length agrees, so the client's loop lands exactly on the end of the body
        // rather than reading into whatever follows.
        assert_eq!(
            body.len(),
            body_len(&a),
            "the declared length and the built body must agree after the clamp too"
        );

        // The fifteen that survive are the FIRST fifteen, in order - so a clamp cannot
        // silently reorder who got hit.
        for (n, t) in drawable_targets(&a).enumerate() {
            assert_eq!(t.object_id, 2000 + n as u32, "target {n} is out of order");
        }
    }

}
