//! Mob behaviour - inbound `0x03D2` `MobChangeController`, and the client's `0x02FF` reply.
//!
//! Full working: `research/mob-behaviour.md`. Every claim below carries the address it was
//! read at; **[L]** is out of the listing, **[D]** is derived from two or more [L], **[I]** is
//! inferred.
//!
//! # The server does not author mob behaviour. It hands a mob to a client
//!
//! Mobs spawned and rendered on map 40 on 2026-08-19 and stood completely still, and the
//! client would not even aim an attack at one 49 pixels away. The missing packet is not a
//! movement packet - it is this one, which transfers responsibility for the mob.
//!
//! A movement *path* can be pushed from the server, by `0x03D9` (`FUN_141c813b0`, whose second
//! body byte splits `action * 2 + facing` exactly as `encodeInit` does at `141c50dbf`) - but
//! that is the rebroadcast of some other client's report to clients which do **not** control
//! the mob, its encoding is undecoded here, and with one player on the field there is nobody
//! to send it to. **[L]**
//!
//! What the client has instead is a **sender**: mob primary-vtable slot 22, `FUN_141cb6880`,
//! which builds outbound [`MOB_MOVE_REQUEST`] and fills it from
//! `FUN_141d57c60` - the same movement-path encoder the player's own `0x00D9` uses. Slot 19,
//! `FUN_141c8d1b0`, stashes slot 22 in a local at `141c8d31f` and then builds a list of
//! 12-byte elements out of the random source `FUN_142f04924`. **The controlling client rolls
//! the mob's wander itself and reports it.** **[D]**
//!
//! # The one virtual call that separates spawn from behaviour
//!
//! Enumerate the indirect calls each handler makes:
//!
//! ```text
//! FUN_141d33630  0x03C6  [rax+0x10] [rax+0x20] [rax] [rax] [rax+0x38] [rax] [rax] [rax] [r8+0x108]
//! FUN_141d34a70  0x03D2  [rax] [rax] [rax+0x38]  ***[rax+0x40]***
//! ```
//!
//! `[rax+0x40]` is mob vtable slot 8 = `FUN_141c54200`, called with a hardcoded `EDX = 1`
//! (`141d34ca9`) and **never called by the spawn handler**. It switches the mob's animation
//! object on:
//!
//! ```text
//! 141c5420a  MOV  RDI,[RCX+0x2c0]   both interfaces must exist - encodeInit builds them
//! 141c5421d  MOV  RAX,[RCX+0x2b8]   ...and both are null-checked, so this cannot fault
//! 141c54241  CALL 0x1409c5080       already running? then nothing
//! 141c5424e  LEA  EDX,[RAX+3]       RAX == 0 here, so 3
//! 141c54254  CALL 0x141c4ff30       mob's obfuscated state at mob+0x2e4 := 3
//! 141c54261  CALL 0x141c55750(mob, 1)
//! ```
//!
//! and `mob+0x2e4` is read straight back by the move sender at `141cb864a`. So a mob that is
//! spawned and never granted is created, pooled, drawn - and never switched on. **[D]**
//!
//! # Nothing has to be answered
//!
//! The client's move id lives in `mob+0x2f4` and is incremented locally (`141cb7ecb` reads,
//! `141cb7ef3` writes). A capstone scan of the mob address range for `[reg+0x2f4]` finds 20
//! sites and **none of them is inside any of the eight mob-pool packet handlers**, so there is
//! no `MobCtrlAck` here and [`MOB_MOVE_REQUEST`] is a notification, not a request. It is still
//! worth logging: it arriving at all is the cheapest confirmation this analysis is right.
//! **[L]**

use crate::mob::{FieldMob, MOB_ENTER_FIELD_LEN, MOB_TEMP_STAT_MASK_LEN};

/// `MobChangeController`. Mob-pool dispatcher `FUN_141d30e80`, `case 0x3d2` at table entry
/// `0x141d31184 + 0xc * 4` -> `0x141d30ee0`. **[L]**
pub const MOB_CHANGE_CONTROLLER: u16 = 0x03D2;

/// The client's mob-move report, **outbound from the client** - we only ever parse it.
///
/// `FUN_141cb6880` contains exactly one `COutPacket` construction, `141cb7ea5 MOV EDX,0x2ff`
/// followed by `CALL 0x1406ed520`. **[L]**
pub const MOB_MOVE_REQUEST: u16 = 0x02FF;

/// Controller level `0`. **This DESPAWNS the mob - it does not merely release control.**
///
/// `141d30ef5 TEST EBP,EBP / JE 141d30f1c` takes the zero branch straight into the pool's
/// erase path: `FUN_141d51320(pool+0x38, …)`, `FUN_141d51670(pool+0x68, &id)`,
/// `FUN_141d51700(pool+0xa8, &id)`. The body stops after 5 bytes. **[L]**
pub const CONTROL_RELEASE: u8 = 0;

/// Controller level `1` - the client owns this mob's movement. **Send this one.**
///
/// Any non-zero level triggers the slot-8 activation, because `141d34ca9` passes a hardcoded
/// `1` to it. `1` additionally makes `FUN_141cc1e40` (`CMP byte [rcx+0x960],1 / SETA AL`)
/// return 0, which skips a block in `FUN_141c54430` that dereferences `mob+0x2c0` with the
/// same unguarded `CMOVE RDX,0xfa4` shape that killed the client on 2026-08-19. **[L]**
pub const CONTROL_NORMAL: u8 = 1;

/// Controller level `2` - the only other value the client distinguishes.
///
/// `FUN_141cc1e40` returns `level > 1`, and that flag is `FUN_141c54430`'s second argument.
/// The v214 reference calls the same flag "chase"/aggro. **[I]** for the meaning, **[L]** for
/// the `> 1` test.
///
/// Safe only for a mob that has already run `encodeInit` (i.e. one we spawned with `0x03C6`),
/// because the block it unlocks reads `mob+0x2c0` without a null check. Prefer
/// [`CONTROL_NORMAL`].
pub const CONTROL_AGGRO: u8 = 2;

/// Body length of a [`mob_change_controller`] for an ordinary mob: **87 bytes**.
///
/// 11-byte head + 20-byte temporary-stat mask + the **56-byte** tail `FUN_141c54060` reads.
/// That tail is a strict prefix of `encodeInit`: 25 read sites, `.pdata`-bounded to
/// `0x141c54060..0x141c541eb`, ending at body offset 87 with the `u8` at `141c541a2`. **[D]**
pub const MOB_CHANGE_CONTROLLER_LEN: usize = 87;

/// Body length of a [`mob_change_controller_spawning`]: the same **137** bytes as a
/// `MobEnterField`, because `FUN_141d34a70`'s new-mob branch calls the same `[vtable+0x38]`
/// `encodeInit` (`141d34c5f`). **[L]**
pub const MOB_CHANGE_CONTROLLER_SPAWNING_LEN: usize = MOB_ENTER_FIELD_LEN;

/// Does `FUN_141c54060` read the extra appear-option word for this mob?
///
/// **This is NOT `FieldMob::has_appear_option`.** The `0x03D2` existing-mob tail accepts only
/// `>= 0` and `-3`; `encodeInit` also accepts `-6`. Read at `141c54101`:
///
/// ```asm
/// 141c54101  MOV   ECX,0x80000000
/// 141c54106  LEA   EAX,[RSI + RCX]      ; appearType + 0x80000000
/// 141c54109  TEST  ECX,EAX              ; bit 31 set  <=>  appearType >= 0
/// 141c5410b  JNE   read
/// 141c5410d  CMP   ESI,-3
/// 141c54110  JNE   skip
/// ```
///
/// Nothing sends `-6` today, so the two predicates never disagree in practice - but a builder
/// that copied `encodeInit`'s would desynchronise the rest of the body if one ever did. **[L]**
pub fn reads_appear_option(mob: &FieldMob) -> bool {
    mob.appear_type >= 0 || mob.appear_type == -3
}

/// Exact [`mob_change_controller`] body length for this mob.
///
/// The three optional blocks inside offsets 31..86 all apply; the fourth
/// (`FieldMob::target_from_server`, at offset 103) does **not**, because this body ends at 87.
pub fn change_controller_len(mob: &FieldMob) -> usize {
    MOB_CHANGE_CONTROLLER_LEN
        + usize::from(reads_appear_option(mob)) * 4
        + usize::from(mob.has_special_template_byte())
        + if mob.patrol.is_some() { 16 } else { 0 }
}

/// **Grant a client control of a mob it already has** - the packet that makes a mob move.
///
/// Send this once per mob, **after** its `MobEnterField`. The client then runs the mob's
/// wander locally and reports each path as [`MOB_MOVE_REQUEST`].
///
/// # Why the mob must already be in the pool
///
/// `FUN_141d34a70` forks on `FUN_141d2efc0(pool, objectId)` at `141d34ab7`:
///
/// ```text
/// found     -> FUN_141c54060   body ends at 87
/// not found -> [vtable+0x38]   body ends at 137, and the mob is CREATED
/// ```
///
/// The mob pool is destroyed and rebuilt on every field entry, so "already in the pool" means
/// *this* field entry. If you are not certain, use [`mob_change_controller_spawning`] - a
/// short body against the long branch desynchronises everything after it in the stream, with
/// no length prefix and no resync point.
///
/// # `level`
///
/// [`CONTROL_NORMAL`]. [`CONTROL_RELEASE`] here **despawns**, and this function will not build
/// it - use [`mob_release_controller`], whose name says what it does.
///
/// # The layout
///
/// Identical to `mob::mob_enter_field` for the first 87 bytes except byte 0, which is the
/// controller level rather than `sealedInsteadDead`. `mob_change_controller_is_the_enter_field_prefix`
/// pins that against the real builder rather than against a copy of it.
pub fn mob_change_controller(mob: &FieldMob, level: u8) -> Vec<u8> {
    debug_assert!(
        level != CONTROL_RELEASE,
        "level 0 despawns the mob rather than releasing it - use mob_release_controller"
    );

    let mut b = Vec::with_capacity(change_controller_len(mob));

    // -- head: the dispatcher itself, then FUN_141d34a70's first read -----------------
    b.push(level); //                                       0   u8  141d30ee3 controller level
    b.extend_from_slice(&mob.object_id.to_le_bytes()); //    1   u32 141d30eee pool key
    b.push(1); //                                           5   u8  141d30efc -> FUN_141c76190 arg3
    b.extend_from_slice(&mob.template_id.to_le_bytes()); //  6   u32 141d34aac Mob/%07d.img
    b.push(0); //                                          10   u8  141d34c6a forced stat: NO

    // -- temporary stats: FUN_141c76190, 20 bytes ------------------------------------
    b.extend_from_slice(&[0u8; MOB_TEMP_STAT_MASK_LEN]); //     141c76276 mask, no bits set

    // -- FUN_141c54060, 56 bytes: encodeInit's offsets 31..86 ------------------------
    b.extend_from_slice(&mob.x.to_le_bytes()); //           31  i16 141c54083 x
    b.extend_from_slice(&mob.y.to_le_bytes()); //           33  i16 141c5408b y
    b.push(mob.move_action); //                             35  u8  141c54093 READ AND DISCARDED
    if mob.has_special_template_byte() {
        b.push(0); //                                           u8  141c540ae 3 template ids
    }
    b.extend_from_slice(&mob.fh.to_le_bytes()); //          36  i16 141c540b6 foothold
    b.extend_from_slice(&mob.home_fh.to_le_bytes()); //     38  i16 141c540be home foothold
    b.push(0); //                                           40  u8  141c540c6 -> mob+0x116c
    b.push(mob.appear_type as u8); //                       41  i8  141c540d7 appearType
    if reads_appear_option(mob) {
        b.extend_from_slice(&mob.appear_option.to_le_bytes()); // u32 141c54115
    }
    b.extend_from_slice(&0i16.to_le_bytes()); //            42  i16 141c540e2 -> mob+0x1228
    b.extend_from_slice(&0i16.to_le_bytes()); //            44  i16 141c540f3 -> mob+0x122c
    b.extend_from_slice(&mob.hp_scale_percent.to_le_bytes()); // 46 u32 141c5411d HP scale %
    b.extend_from_slice(&mob.hp.to_le_bytes()); //          50  u64 141c5412b CURRENT HP
    b.extend_from_slice(&0u32.to_le_bytes()); //            58  u32 141c54133 effect item id
    if let Some(patrol) = mob.patrol {
        for v in patrol {
            b.extend_from_slice(&v.to_le_bytes()); //           u32 141c5414b.. patrol range
        }
    }
    b.extend_from_slice(&0u32.to_le_bytes()); //            62  u32 141c5416b
    b.extend_from_slice(&0u32.to_le_bytes()); //            66  u32 141c54173
    b.extend_from_slice(&0u32.to_le_bytes()); //            70  u32 141c5417b
    // Offset 74 MUST stay negative. `141c541ab CMP EBP,-1 / JLE skip` guards
    // `141c541c6 MOV dword [RDI+0xcd0],1`, and `141cb6907 CMP dword [R12+0xcd0],0 / JNE` is
    // the FIRST thing the mob-move sender does - a non-zero value there and the mob can never
    // report a move. encodeInit has the identical pair at 141c530af / 141c530b8.
    b.extend_from_slice(&(-1i32).to_le_bytes()); //         74  i32 141c54183 must be < 0
    b.extend_from_slice(&0i32.to_le_bytes()); //            78  i32 141c5418d
    b.extend_from_slice(&0u32.to_le_bytes()); //            82  u32 141c54197
    b.push(0); //                                           86  u8  141c541a2

    debug_assert_eq!(b.len(), change_controller_len(mob));
    b
}

/// **Spawn a mob and grant control in one packet** - the 137-byte form, for a mob the client
/// does *not* already have.
///
/// `FUN_141d34a70`'s not-found branch builds the mob (`FUN_140495990` -> `FUN_141d3a540`),
/// inserts it into `pool+0x38` and `pool+0x68` exactly as the spawn handler does, and then
/// calls the same `[vtable+0x38]` `encodeInit` at `141d34c5f`. So the body is
/// `mob::mob_enter_field`'s, with byte 0 carrying the controller level instead of
/// `sealedInsteadDead`. **[L]**
///
/// This is deliberately built *from* [`crate::mob::mob_enter_field`] rather than beside it:
/// the two bodies cannot drift apart, and any future correction to the spawn layout lands here
/// for free.
///
/// **Not the recommended path today.** `0x03C6` is the one mob packet this client has actually
/// accepted on screen; using `0x03D2` as the spawn changes two things at once.
pub fn mob_change_controller_spawning(mob: &FieldMob, level: u8) -> Vec<u8> {
    debug_assert!(
        level != CONTROL_RELEASE,
        "level 0 despawns the mob rather than releasing it - use mob_release_controller"
    );
    let mut b = crate::mob::mob_enter_field(mob);
    b[0] = level; // 141d30ee3 reads byte 0 as the controller level, not as `sealed`
    b
}

/// Level 0: **remove the mob from the client's pool.** Five bytes, and the handler reads no
/// more.
///
/// This is the only "revoke" the client has, and it is a despawn: `141d30f1c` onward looks the
/// object id up in `pool+0x68`, calls `[vtable+0x48]` (returns without doing anything if it is
/// 0), `[vtable+0x40](mob, 0)` and `FUN_141c543c0` (returns if non-zero), then erases the mob
/// from all three pool containers. **[L]**
///
/// Named for what it does rather than for the field it sets, because "release control" is what
/// the level byte looks like and deleting the mob is what happens.
pub fn mob_release_controller(object_id: u32) -> Vec<u8> {
    let mut b = Vec::with_capacity(5);
    b.push(CONTROL_RELEASE); //                         0  u8  141d30ee3
    b.extend_from_slice(&object_id.to_le_bytes()); //   1  u32 141d30eee
    b
}

/// The two fields at the front of a client [`MOB_MOVE_REQUEST`] body.
///
/// Only these two are parsed. The rest of that body is long and conditional - two
/// count-prefixed `u16` lists, an eleven-word block gated on `mob+0x10b0`, and finally the
/// movement path itself through `FUN_141d57c60` - and **nothing here needs it**, because no
/// mob-pool handler acknowledges a move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MobMoveRequest {
    /// `141cb7eb7 MOV EDX,[R12 + 0x3a0]` -> `Encode4`. `mob+0x3a0` is the **object id**: the
    /// only two writers in mob code are the constructor and `141c4ffb4 MOV [RSI+0x3a0],EBX`
    /// inside `encodeInit`, whose `EBX` is the object id the spawn handler passes at
    /// `141d33923`. **[L]**
    pub object_id: u32,
    /// `141cb7efb MOVZX EDX,DI` -> `Encode2`. A per-mob counter at `mob+0x2f4`, read at
    /// `141cb7ecb` and written back incremented at `141cb7ef3`. Purely local to the client -
    /// no packet handler in the mob pool touches that field. **[L]**
    pub move_id: u16,
}

/// Parse the head of a client mob-move report. `None` if the body is too short to hold it.
///
/// The caller has already stripped the opcode; `body` is what follows it.
pub fn parse_mob_move(body: &[u8]) -> Option<MobMoveRequest> {
    if body.len() < 6 {
        return None;
    }
    Some(MobMoveRequest {
        object_id: u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        move_id: u16::from_le_bytes([body[4], body[5]]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mob::{mob_enter_field, MOVE_ACTION_MIN_SAFE};

    fn snail() -> FieldMob {
        // Template 2 is map 40's snail; the numbers are placeholders, the offsets are not.
        FieldMob::new(2000, 2, -100, 200, 7, 45)
    }

    /// 11 + 20 + 56. If this number moves, `FUN_141c54060`'s read list moved with it.
    #[test]
    fn the_ordinary_body_is_eighty_seven_bytes() {
        let m = snail();
        let b = mob_change_controller(&m, CONTROL_NORMAL);
        assert_eq!(b.len(), MOB_CHANGE_CONTROLLER_LEN);
        assert_eq!(b.len(), change_controller_len(&m));
    }

    /// The whole claim of `research/mob-behaviour.md` section 3 in one assertion: the
    /// change-controller body **is** the enter-field body's first 87 bytes, with byte 0
    /// reinterpreted. Checked against the real spawn builder, not against a copy of it.
    #[test]
    fn mob_change_controller_is_the_enter_field_prefix() {
        let m = snail();
        let spawn = mob_enter_field(&m);
        let grant = mob_change_controller(&m, CONTROL_NORMAL);

        assert_eq!(grant[0], CONTROL_NORMAL, "141d30ee3 reads byte 0 as the level");
        assert_eq!(spawn[0], 0, "141d3365a reads byte 0 as sealedInsteadDead");
        assert_eq!(
            &grant[1..],
            &spawn[1..grant.len()],
            "offsets 1..87 must be byte-identical - FUN_141c54060 is a strict prefix of encodeInit"
        );
    }

    /// The head, field by field, at the address the client reads each one.
    #[test]
    fn the_head_is_read_where_the_listing_says() {
        let m = snail();
        let b = mob_change_controller(&m, CONTROL_AGGRO);
        assert_eq!(b[0], CONTROL_AGGRO, "141d30ee3");
        assert_eq!(&b[1..5], &2000u32.to_le_bytes(), "141d30eee objectId");
        assert_eq!(b[5], 1, "141d30efc -> FUN_141c76190 arg3");
        assert_eq!(&b[6..10], &2u32.to_le_bytes(), "141d34aac templateId");
        assert_eq!(b[10], 0, "141d34c6a forced-stat gate");
        assert_eq!(&b[11..31], &[0u8; 20], "141c76276 all-zero mask costs nothing");
    }

    /// Offset 74 is the one value in this body that decides whether the mob can ever move.
    /// A `0` there sets `mob+0xcd0`, and `141cb6907` bails on `mob+0xcd0 != 0` before the
    /// mob-move builder writes a single byte.
    #[test]
    fn offset_74_stays_negative_or_the_mob_can_never_report_a_move() {
        let b = mob_change_controller(&snail(), CONTROL_NORMAL);
        let v = i32::from_le_bytes([b[74], b[75], b[76], b[77]]);
        assert!(v < 0, "141c541ab CMP EBP,-1 / JLE skips MOV [RDI+0xcd0],1");
        assert_eq!(v, -1);
    }

    /// `FUN_141c54060` reads offset 35 at `141c54093` and never stores the result, so this
    /// packet cannot restance an existing mob - but the byte still occupies a slot, and a
    /// wrong width there would desynchronise the 51 bytes after it.
    #[test]
    fn move_action_occupies_offset_35_even_though_this_handler_discards_it() {
        let b = mob_change_controller(&snail(), CONTROL_NORMAL);
        assert_eq!(b[35], MOVE_ACTION_MIN_SAFE, "141c54093");
    }

    /// The `0x03D2` tail's appear-option gate is NOT `encodeInit`'s: `-6` reads the option
    /// word in the spawn body and does not read it here (`141c54101`).
    #[test]
    fn the_appear_option_gate_differs_from_the_spawn_bodys() {
        let mut m = snail();

        m.appear_type = -2;
        assert!(!reads_appear_option(&m));
        assert!(!m.has_appear_option());

        m.appear_type = -3;
        assert!(reads_appear_option(&m));
        assert!(m.has_appear_option());

        m.appear_type = 0;
        assert!(reads_appear_option(&m));
        assert!(m.has_appear_option());

        // The one value where the two handlers disagree.
        m.appear_type = -6;
        assert!(!reads_appear_option(&m), "141c54101 tests only >= 0 and -3");
        assert!(m.has_appear_option(), "141c504c2 also accepts -6");
    }

    /// Each optional block inside offsets 31..86 lengthens the body; the one at offset 103
    /// does not, because this body ends at 87.
    #[test]
    fn the_optional_blocks_inside_the_first_eighty_seven_bytes_are_the_only_ones() {
        let mut m = snail();
        m.appear_type = 0; // reads the option word
        assert_eq!(change_controller_len(&m), MOB_CHANGE_CONTROLLER_LEN + 4);
        assert_eq!(mob_change_controller(&m, CONTROL_NORMAL).len(), change_controller_len(&m));

        let mut m = snail();
        m.patrol = Some([1, 2, 3, 4]);
        assert_eq!(change_controller_len(&m), MOB_CHANGE_CONTROLLER_LEN + 16);
        assert_eq!(mob_change_controller(&m, CONTROL_NORMAL).len(), change_controller_len(&m));

        // target_from_server sits at body offset 103, past the end of this body.
        let mut m = snail();
        m.target_from_server = Some(0x1234_5678);
        assert_eq!(change_controller_len(&m), MOB_CHANGE_CONTROLLER_LEN);
        assert_eq!(mob_change_controller(&m, CONTROL_NORMAL).len(), MOB_CHANGE_CONTROLLER_LEN);
    }

    /// The spawning form must stay exactly the enter-field body, or the two have drifted.
    #[test]
    fn the_spawning_form_is_the_enter_field_body_with_byte_zero_replaced() {
        let m = snail();
        let spawn = mob_enter_field(&m);
        let both = mob_change_controller_spawning(&m, CONTROL_NORMAL);
        assert_eq!(both.len(), MOB_CHANGE_CONTROLLER_SPAWNING_LEN);
        assert_eq!(both.len(), m.body_len());
        assert_eq!(both[0], CONTROL_NORMAL);
        assert_eq!(&both[1..], &spawn[1..]);
    }

    /// Level 0 is a despawn and the handler reads exactly five bytes.
    #[test]
    fn release_is_five_bytes_and_is_a_despawn() {
        let b = mob_release_controller(2000);
        assert_eq!(b.len(), 5);
        assert_eq!(b[0], CONTROL_RELEASE, "141d30ef5 TEST EBP,EBP / JE");
        assert_eq!(&b[1..5], &2000u32.to_le_bytes());
    }

    #[test]
    fn the_move_report_head_parses_and_short_bodies_do_not_panic() {
        let mut body = Vec::new();
        body.extend_from_slice(&2000u32.to_le_bytes());
        body.extend_from_slice(&7u16.to_le_bytes());
        body.extend_from_slice(&[0xAA; 40]); // the conditional tail we deliberately ignore

        let got = parse_mob_move(&body).expect("6 bytes is enough");
        assert_eq!(got.object_id, 2000);
        assert_eq!(got.move_id, 7);

        assert_eq!(parse_mob_move(&body[..5]), None);
        assert_eq!(parse_mob_move(&[]), None);
    }

    /// The two opcodes, so a typo in either constant fails here rather than on the wire.
    #[test]
    fn the_opcodes_are_the_ones_read_out_of_the_image() {
        assert_eq!(MOB_CHANGE_CONTROLLER, 0x03D2, "jump table entry at 0x141d31184 + 0x30");
        assert_eq!(MOB_MOVE_REQUEST, 0x02FF, "141cb7ea5 MOV EDX,0x2ff");
    }
}
