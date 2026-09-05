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
//! # RETRACTED: `0x02FF` **does** have an acknowledgement, and it is [`MOB_CTRL_ACK`]
//!
//! The first version of this file said: *"A capstone scan of the mob address range for
//! `[reg+0x2f4]` finds 20 sites and none of them is inside any of the eight mob-pool packet
//! handlers, so there is no `MobCtrlAck` here."*
//!
//! **The scan was right and the intersection was wrong.** Re-run today it still returns
//! exactly 20 sites, and `141c821f8 MOV EDX,[RDI+0x2f4]` is one of them - inside
//! `FUN_141c82060`, which `tools/callers.py` says has exactly one caller,
//! `141d32ba3` in the second dispatcher, which is **`case 0x3E4`**. The intersection was made
//! against "the eight mob-pool handlers" at a time when only the *first* jump table was
//! known; the second table's 102 opcodes had not been found yet. This is the same mistake
//! section 2.2 retracts one level up, made twice in one file.
//!
//! `FUN_141c82060` reads a `u16`, de-obfuscates the client's own move counter from the pair
//! `mob+0x2f0`/`mob+0x2f4` - the identical pair the move builder increments at `141cb7ecb` -
//! compares them, and writes the mob's animation state accordingly. That is an
//! acknowledgement. **[L]**
//!
//! ```text
//! 141c821f8  MOV   EDX,[RDI + 0x2f4]        the obfuscation key
//! 141c821fe  LEA   RCX,[RDI + 0x2f0]        the obfuscated move id
//! 141c82205  CALL  0x1401ab420              -> AX = the client's current move id
//! 141c8220d  MOVSX EAX,word [RSP + 0x60]    the move id WE just acknowledged
//! 141c82212  CMP   EAX,ECX
//! 141c82214  JNS   141c8221d                ack >= current -> 3, or 4 if the u8 is set
//! 141c82216  MOV   EBX,2                    ack <  current -> 2
//! 141c82232  MOV   [RDI + 0x2e4],EAX        ... obfuscated into mob+0x2e4/0x2e8/0x2ec
//! ```
//!
//! # What was measured on 2026-08-19, and what is still inferred
//!
//! Thirty mobs were granted control at `01:21:36.919`-`.924`. All thirty sent **exactly one**
//! `0x02FF` at `01:21:37.122`-`.125`, every one of them carrying `moveId = 1`, and then
//! nothing ever again. The owner: *"the mobs moved for half a second before freezing again."*
//! That the counter never reached 2 is **[L]** - it is in `world.log`.
//!
//! **[I], and it needs a client run:** that sending [`MOB_CTRL_ACK`] is what lets the client
//! roll a second step. The mechanism is there - the ack re-runs the same slot-8 activation
//! `0x03D2` runs (`141c8208f CALL [RAX+0x40]` with `EDX = 1`), and slot 8 is a no-op while
//! the animation is still running (`141c54248 JNE ret`), which is exactly the shape of a
//! pump - but the instruction that *blocks* the second roll has not been found, only the
//! instruction that would unblock it. Do not write this up as measured.

use crate::mob::{FieldMob, MOB_ENTER_FIELD_LEN, MOB_TEMP_STAT_MASK_LEN};

/// `MobChangeController`. Mob-pool dispatcher `FUN_141d30e80`, `case 0x3d2` at table entry
/// `0x141d31184 + 0xc * 4` -> `0x141d30ee0`. **[L]**
pub const MOB_CHANGE_CONTROLLER: u16 = 0x03D2;

/// The client's mob-move report, **outbound from the client** - we only ever parse it.
///
/// `FUN_141cb6880` contains exactly one `COutPacket` construction, `141cb7ea5 MOV EDX,0x2ff`
/// followed by `CALL 0x1406ed520`. **[L]**
pub const MOB_MOVE_REQUEST: u16 = 0x02FF;

/// **`MobMove` - the server's rebroadcast of a path to clients which do NOT control the mob.**
///
/// Second mob-pool jump table, `0x141d33448`, entry 0 -> stub `141d32b8d` ->
/// `FUN_141c813b0`. Read out of the image by `scratchpad/table2.py`, and corroborated:
/// `tools/callers.py 0x141c813b0` returns exactly one call site, `141d32b93`, which is that
/// stub. **[L]**
pub const MOB_MOVE: u16 = 0x03D9;

/// **`MobCtrlAck` - the answer to [`MOB_MOVE_REQUEST`].**
///
/// Same table, entry `0x3E4 - 0x3D9 = 11` -> stub `141d32b9d` -> `FUN_141c82060`, whose only
/// caller is `141d32ba3` in that stub. **[L]** See the module retraction: this file previously
/// said no such packet existed.
pub const MOB_CTRL_ACK: u16 = 0x03E4;

/// Controller level `0`. **This RELEASES control. It does not despawn a live mob.**
///
/// # This said the opposite for two weeks, and the owner corrected it
///
/// The old text: *"This DESPAWNS the mob - it does not merely release control … takes the
/// zero branch straight into the pool's erase path."* **Straight** was the wrong word, and it
/// was tagged **[L]**. The zero branch has two guards in front of the erase and a live mob
/// stops at the second:
///
/// ```text
///   141d30f69  call [rax+0x48]      slot 9  -> non-zero, continue
///   141d30f7c  call [rax+0x40]      slot 8, edx=0  <- THE RELEASE
///   141d30f82  call 0x141c543c0     the in-field flag -> 1
///   141d30f89  jne  0x141d3116a     -> THE EPILOGUE. The erase is never reached
///   141d30f8f  …                    erase from pool+0x38 / +0x68 / +0xa8
/// ```
///
/// `FUN_141c543c0` reads an obfuscated triple at `mob+0x2d8` whose complete writer set is the
/// constructor (0), `0x03C6` MobEnterField (**1**, both branches) and `0x03D1` leave (0).
/// Nothing else writes it. So any mob that entered the field the normal way returns 1 here
/// and takes the branch to the epilogue. **[L]**
///
/// Slot 8 with `edx == 0` is the exact mirror of the grant, and it has its **own 190-byte
/// `.pdata` entry** - which is why nobody had read it: every earlier pass read the `edx = 1`
/// arm. Grant sets the running state and starts the animation with live coordinates; release
/// clears it and passes zeros. **Neither touches the pool.** Confirmed from the other end:
/// `0x03E4` MobCtrlAck calls slot 9 and, if it is 0, calls slot 8 with `1` to **start** -
/// complementary to stopping.
///
/// # The one case where level 0 really does erase
///
/// A mob a client knows only from a **137-byte `0x03D2`** never had the in-field flag set,
/// because that spawn path does not call the setter. For that mob the guard returns 0 and the
/// erase runs. That is almost certainly where the original reading came from: true, for the
/// one spawn path this server never uses. **Never pair
/// [`mob_change_controller_spawning`] with a release.**
///
/// # It had never been sent
///
/// 505 archived log files, 240 788 events deduplicated on `(timestamp, direction, opcode,
/// body)`: **2 664** `0x03D2` bodies at level 1 and **zero** at level 0. The claim shaped the
/// whole mob-sharing design - "never rotate control while its holder is present" - and was
/// never once tested against the client.
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
/// [`CONTROL_NORMAL`]. [`CONTROL_RELEASE`] does not belong in THIS builder - a grant and a
/// release are opposite acts - and this function will not build
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
        "level 0 is a RELEASE and belongs in mob_release_controller, not in a grant"
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
        "level 0 is a RELEASE and belongs in mob_release_controller, not in a grant"
    );
    let mut b = crate::mob::mob_enter_field(mob);
    b[0] = level; // 141d30ee3 reads byte 0 as the controller level, not as `sealed`
    b
}

/// Level 0: **remove the mob from the client's pool.** Five bytes, and the handler reads no
/// more.
///
/// **This is the client's revoke, and it RELEASES** - see [`CONTROL_RELEASE`], which owns
/// that fact. What follows is the walk as it was first read, kept because the misreading is
/// instructive: `141d30f1c` onward looks the
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

/// One 21-byte movement-path element, the unit both `0x02FF` and [`MOB_MOVE`] carry.
///
/// 21 is measured two ways. From the image: `FUN_1404b2630` (the reader
/// `FUN_141d598b0` calls) walks a count-prefixed list. From the wire: the 30 captured
/// `0x02FF` bodies of 2026-08-19 are **111, 132, 153 and 174** bytes, an exact 21-byte
/// ladder over element counts 1, 2, 3 and 4. **[D]**
///
/// The field split inside the element is **[I]** past `x`/`y` - those two are confirmed
/// against an independent capture (see [`MobMoveRequest::x`]) and the rest is not. Nothing
/// here needs it: the block is re-emitted verbatim.
pub const MOB_PATH_ELEMENT_LEN: usize = 21;

/// The fixed head of a movement path: `u32`, `i16 x`, `i16 y`, `u16`, `u16`, `i16 count`.
///
/// Read at `1404b2650`, `1404b265b`, `1404b2675`, `1404b2690`, `1404b269c`, `1404b26a9`. The
/// two `i16`s are stored XOR-obfuscated at `this+0x20`/`this+0x28` with keys at `+0x24`/`+0x2c`,
/// which is how they were identified as coordinates rather than as an opaque dword. **[L]**
pub const MOB_PATH_HEAD_LEN: usize = 14;

/// A decoded client [`MOB_MOVE_REQUEST`].
///
/// # The whole body, and it is checked against 30 real ones
///
/// Field order is `FUN_141cb6880`'s encode order, read out of the image with
/// `scratchpad/encodes.py` (a mirror of `tools/reads.py` sharing its loader and its
/// positive control). The 46-byte head, the path, and the 29-byte tail account for **30 of
/// 30** captured bodies to the byte - see `mob_move_layout_decodes_the_captured_body`.
///
/// Two landmarks make that more than an arithmetic coincidence:
///
/// * body offsets 29 and 33 are both `0x00ffddcc`, and `141cb827e` is
///   `MOV EBX,0xffddcc` - a literal in the encoder appearing where the layout predicts it;
/// * the tail's `u8` at `141cb85fe` is `mob+0x960`, the **controller level**, and every
///   captured body carries `1` there - the value this server granted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MobMoveRequest {
    /// `141cb7eb7 MOV EDX,[R12 + 0x3a0]` -> `Encode4`. `mob+0x3a0` is the **object id**: the
    /// only two writers in mob code are the constructor and `141c4ffb4 MOV [RSI+0x3a0],EBX`
    /// inside `encodeInit`, whose `EBX` is the object id the spawn handler passes at
    /// `141d33923`. **[L]**
    pub object_id: u32,
    /// `141cb7efb MOVZX EDX,DI` -> `Encode2`. The client's own counter, `deobf(mob+0x2f0)`
    /// **plus one** (`141cb7ee0 LEA EDI,[RAX+1]`), written back at `141cb7ef3`.
    ///
    /// **Echo this back in [`mob_ctrl_ack`].** `141c82212 CMP EAX,ECX / JNS` compares what we
    /// send against this same counter, and only `ack >= current` reaches the state-3 branch.
    /// **[L]**
    pub move_id: u16,
    /// Body offset 6, `141cb7f20`. A packed byte, `((a << 2) | b) << 2 | c`.
    ///
    /// Goes straight into [`MOB_MOVE`] body offset 4, where `141c813d0` reads bit 0 and bit 2.
    /// Every captured body has `0x00`. **[L]** for the position, **[D]** for the mapping.
    pub packed: u8,
    /// Body offset 7, `141cb7f31`. Goes into [`MOB_MOVE`] body offset 5, which `141c813f1`
    /// reads and then splits `action * 2 + facing` at `141c81766 SHR EBP,1` - the same split
    /// `encodeInit` does at `141c50dbf`.
    ///
    /// **`0xFF` means "no action"**: `141c8176d CMP AL,0xFF / JE` skips the whole animation
    /// block. Every one of the 30 captured bodies carries `0xFF`. **[L]**
    pub move_action: u8,
    /// Where the client now believes the mob is, from the path head at `1404b265b`.
    ///
    /// **Confirmed against an independent capture.** `research/mob-behaviour.md` §0 records
    /// the combat agent decoding `0x00DF` on the same map with mob 2000 at `(424, 395)`.
    /// This decoder reads mob 2000's `0x02FF` as `(424, 395)`. Two different packets, two
    /// different agents, same pixel. **[L]**
    pub x: i16,
    /// See [`MobMoveRequest::x`]. `1404b2675`.
    pub y: i16,
    /// `1404b26a9`, a **signed** `i16` (`MOVSX ECX,AX / TEST / JLE bail`).
    pub element_count: i16,
    /// The path block exactly as it arrived: [`MOB_PATH_HEAD_LEN`] plus
    /// `element_count * `[`MOB_PATH_ELEMENT_LEN`], and **nothing after it**.
    ///
    /// This is what [`mob_move_broadcast`] re-emits, byte for byte. It deliberately stops
    /// before `0x02FF`'s trailing nibble-count `u8`: that byte is read at `141d5991f`
    /// **gated on `FUN_141d598b0`'s third argument**, and the two call sites pass different
    /// values - `141cb8346 MOV R8D,R13D` for `0x02FF` (so the byte is there) against
    /// `141c82000 XOR R8D,R8D` for [`MOB_MOVE`] (so it is not). Copying the path with that
    /// byte still attached would put one byte too many in every rebroadcast. **[L]**
    pub path: Vec<u8>,
}

/// Parse a client mob-move report. The caller has already stripped the opcode.
///
/// `None` only if the body is malformed - too short for the head, or a count that runs off
/// the end. A body that parses is guaranteed to have a [`MobMoveRequest::path`] of exactly
/// `14 + 21 * element_count` bytes, which is what [`mob_move_broadcast`] needs.
pub fn parse_mob_move(body: &[u8]) -> Option<MobMoveRequest> {
    let mut c = Cursor::new(body);

    // -- head, FUN_141cb6880's encode order 141cb7ec6 .. 141cb82f0 --------------------
    let object_id = c.u32()?; //                       0   141cb7ec6  mob+0x3a0
    let move_id = c.u16()?; //                         4   141cb7f05  deobf(mob+0x2f0) + 1
    let packed = c.u8()?; //                           6   141cb7f20
    let move_action = c.u8()?; //                      7   141cb7f31
    c.skip(8)?; //                                     8   141cb7f44  u64
    c.skip(2)?; //                                    16   141cb7f55, 141cb7f65
    let n1 = c.u8()?; //                              18   141cb7f80/91  mob+0x8c8
    c.skip(usize::from(n1) * 4)?; //                        141cb7ff2 + 141cb803a per element
    let n2 = c.u8()?; //                                    141cb8063/71  mob+0x8d0
    c.skip(usize::from(n2) * 2)?; //                        141cb80ca per element
    let gate = c.u32()?; //                                 141cb80e9  mob+0x10b0
    if gate != 0 {
        c.skip(11 * 4)?; //                                 141cb810c .. 141cb81e4
    }
    c.skip(1 + 5 * 4 + 1)?; //                              141cb820e .. 141cb82f0

    // -- the movement path, FUN_141d57c60 -> FUN_1404b2000 ---------------------------
    let path_start = c.pos;
    c.skip(4)?; //                                          1404b2650 -> this+0x40
    let x = c.i16()?; //                                    1404b265b -> obf this+0x20
    let y = c.i16()?; //                                    1404b2675 -> obf this+0x28
    c.skip(4)?; //                                          1404b2690, 1404b269c
    let element_count = c.i16()?; //                        1404b26a9, signed
    if element_count < 0 {
        return None;
    }
    c.skip(element_count as usize * MOB_PATH_ELEMENT_LEN)?;
    let path = body.get(path_start..c.pos)?.to_vec();

    Some(MobMoveRequest {
        object_id,
        move_id,
        packed,
        move_action,
        x,
        y,
        element_count,
        path,
    })
}

/// **Acknowledge a [`MOB_MOVE_REQUEST`]** - the packet this file previously said did not
/// exist.
///
/// Send one per `0x02FF`, back to the client that sent it, echoing its `move_id`.
///
/// # The body, 26 bytes, every read unconditional
///
/// The dispatcher `FUN_141d32b30` reads the object id itself at `141d32b4d` and looks the mob
/// up; **an unknown object id returns silently** (`141d32b62 JE` to the epilogue) rather than
/// faulting, so a stale id costs nothing. `FUN_141c82060`'s eight reads then run
/// straight-line - the function's only branch before them is `141c82082`, and both arms
/// converge at `141c82092 - so there is no gate and the length is fixed. **[L]**
///
/// | off | size | read at | what |
/// |---|---|---|---|
/// | 0 | u32 | `141d32b4d` | object id (the dispatcher's, not the handler's) |
/// | 4 | u16 | `141c8209e` | **the move id being acknowledged** |
/// | 6 | u8 | `141c820ab` | `!= 0` makes the resulting state `4` instead of `3` |
/// | 7 | u32 | `141c820b7` | obfuscated into `mob+0x3b0`/`+0x3b4`/`+0x3b8`. **[I]**: MP |
/// | 11 | u32 | `141c820f0` | skill id - `0` short-circuits the whole skill block |
/// | 15 | u16 | `141c820fa` | skill level, paired with the above into `FUN_14049a080` |
/// | 17 | u32 | `141c82105` | `0` skips the `FUN_140401b30` call at `141c821f3` |
/// | 21 | u32 | `141c82114` | **read and discarded** - `EAX` is clobbered by the next read |
/// | 25 | u8 | `141c8211c` | **read and discarded** |
///
/// # Why `move_id` must be the one that arrived
///
/// `141c82212 CMP EAX,ECX / JNS` - `EAX` is what we send, `ECX` the client's own counter.
/// Only `ack >= current` takes the `3`/`4` branch; a lower value takes `MOV EBX,2`. Echoing
/// exactly what arrived is the only value that is always on the right side of that compare
/// without inventing state. **[L]**
pub fn mob_ctrl_ack(object_id: u32, move_id: u16, next_attack_possible: bool) -> Vec<u8> {
    let mut b = Vec::with_capacity(MOB_CTRL_ACK_LEN);
    b.extend_from_slice(&object_id.to_le_bytes()); //  0   u32 141d32b4d (the dispatcher)
    b.extend_from_slice(&move_id.to_le_bytes()); //    4   u16 141c8209e
    b.push(u8::from(next_attack_possible)); //         6   u8  141c820ab
    b.extend_from_slice(&0u32.to_le_bytes()); //       7   u32 141c820b7  [I] MP
    b.extend_from_slice(&0u32.to_le_bytes()); //      11   u32 141c820f0  skill id: none
    b.extend_from_slice(&0u16.to_le_bytes()); //      15   u16 141c820fa  skill level
    b.extend_from_slice(&0u32.to_le_bytes()); //      17   u32 141c82105  0 -> skip 140401b30
    b.extend_from_slice(&0u32.to_le_bytes()); //      21   u32 141c82114  discarded
    b.push(0); //                                     25   u8  141c8211c  discarded
    debug_assert_eq!(b.len(), MOB_CTRL_ACK_LEN);
    b
}

/// `mob_ctrl_ack`'s body length. Fixed - there is no conditional read in `FUN_141c82060`.
pub const MOB_CTRL_ACK_LEN: usize = 26;

/// **Rebroadcast a controller's path to every OTHER client on the field.**
///
/// This is the packet the owner's framing names: *"the server needs all of the clients to see the
/// same mob movement."* It is **not** what unfreezes the controller - with one player on the
/// field there is nobody to send it to, and the run of 2026-08-19 had exactly one player.
/// Build it for the second player, not for the first.
///
/// # Never send this to the client that sent the `0x02FF`
///
/// The controller is simulating the mob locally; `FUN_141c813b0` writes its position, its
/// animation and `mob+0xcd0` from the packet (`141c81784 MOV [R14+0xcd0],0`). Feeding a
/// client its own path back would fight its own simulation. **[I]** as to what it would look
/// like on screen - the client has not been run with two connections - but the handler
/// plainly overwrites state the controller owns.
///
/// # The body
///
/// `FUN_141c813b0`'s read order, `tools/reads.py 0x141c813b0 3`, with the gates read off the
/// listing. It is `0x02FF`'s body with the move id gone, two `u8`s gone, the pre-path block
/// replaced by one `u32`, and one `u8` after the path. **[L]**
///
/// | off | size | read at | what |
/// |---|---|---|---|
/// | 0 | u32 | `141d32b4d` | object id (the dispatcher's) |
/// | 4 | u8 | `141c813d0` | bit 0 and bit 2 are used; this is `0x02FF` offset 6 |
/// | 5 | u8 | `141c813f1` | `action * 2 + facing`; **`0xFF` = none**. `0x02FF` offset 7 |
/// | 6 | u64 | `141c814c1` | zero in all 30 captures |
/// | 14 | u8 | `141c814f4` | count, then that many `(u16, u16)` into `mob+0x8c8` |
/// | .. | u8 | `141c815fe` | count, then that many `u16` into `mob+0x8d0` |
/// | .. | u32 | `141c81635` | gate -> `mob+0x10b0`; if non-zero, eleven more `u32` |
/// | .. | u32 | `141c8173e` | outside that gate - `141c81648 JE` lands just before it |
/// | .. | | `141c82009` | the path, [`MobMoveRequest::path`] verbatim |
/// | .. | u8 | `141c82011` | non-zero reaches `[vtable]` and `mob+0x988`; send `0` |
///
/// The path is copied rather than re-encoded, which is safe because both packets reach the
/// **same** element codec - `FUN_141d57c60 -> FUN_1404b2000` writing and
/// `FUN_141d598b0 -> FUN_1404b2630` reading, adjacent functions with mirrored `u32 u16 u8`
/// signatures. The one asymmetry is the trailing nibble byte, and
/// [`MobMoveRequest::path`] documents why it is already excluded. **[D]**
pub fn mob_move_broadcast(req: &MobMoveRequest) -> Vec<u8> {
    let mut b = Vec::with_capacity(MOB_MOVE_FIXED_LEN + req.path.len());
    b.extend_from_slice(&req.object_id.to_le_bytes()); //  0  u32 141d32b4d
    b.push(req.packed); //                                 4  u8  141c813d0
    b.push(req.move_action); //                            5  u8  141c813f1
    b.extend_from_slice(&0u64.to_le_bytes()); //           6  u64 141c814c1
    b.push(0); //                                         14  u8  141c814f4  mob+0x8c8: empty
    b.push(0); //                                         15  u8  141c815fe  mob+0x8d0: empty
    b.extend_from_slice(&0u32.to_le_bytes()); //          16  u32 141c81635  gate off
    b.extend_from_slice(&0u32.to_le_bytes()); //          20  u32 141c8173e
    b.extend_from_slice(&req.path); //                    24      141c82009
    b.push(0); //                                             u8  141c82011
    debug_assert_eq!(b.len(), MOB_MOVE_FIXED_LEN + req.path.len());
    b
}

/// Everything in a [`MOB_MOVE`] body that is not the path: 24 bytes before it, 1 after.
pub const MOB_MOVE_FIXED_LEN: usize = 25;

/// A bounds-checked little-endian reader. Every `?` here is a body that ran short, which is
/// the failure `research/mob-behaviour.md` §9 says to make loud rather than silent.
struct Cursor<'a> {
    b: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(b: &'a [u8]) -> Self {
        Cursor { b, pos: 0 }
    }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.b.get(self.pos..self.pos.checked_add(n)?)?;
        self.pos += n;
        Some(s)
    }

    fn skip(&mut self, n: usize) -> Option<()> {
        self.take(n).map(|_| ())
    }

    fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|s| s[0])
    }

    fn u16(&mut self) -> Option<u16> {
        self.take(2).map(|s| u16::from_le_bytes([s[0], s[1]]))
    }

    fn i16(&mut self) -> Option<i16> {
        self.u16().map(|v| v as i16)
    }

    fn u32(&mut self) -> Option<u32> {
        self.take(4)
            .map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
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

    /// A release is exactly five bytes, and the handler stops there.
    ///
    /// Called `..._and_is_a_despawn` until 2026-09-04. The length is what it always checked;
    /// the NAME asserted something it never tested, and a passing test with a wrong name in
    /// its title is part of how that claim kept its footing for two weeks.
    #[test]
    fn release_is_five_bytes() {
        let b = mob_release_controller(2000);
        assert_eq!(b.len(), 5);
        assert_eq!(b[0], CONTROL_RELEASE, "141d30ef5 TEST EBP,EBP / JE");
        assert_eq!(&b[1..5], &2000u32.to_le_bytes());
    }

    /// The two opcodes, so a typo in either constant fails here rather than on the wire.
    #[test]
    fn the_opcodes_are_the_ones_read_out_of_the_image() {
        assert_eq!(MOB_CHANGE_CONTROLLER, 0x03D2, "jump table entry at 0x141d31184 + 0x30");
        assert_eq!(MOB_MOVE_REQUEST, 0x02FF, "141cb7ea5 MOV EDX,0x2ff");
        assert_eq!(MOB_MOVE, 0x03D9, "second table 0x141d33448 entry 0 -> FUN_141c813b0");
        assert_eq!(MOB_CTRL_ACK, 0x03E4, "second table entry 11 -> FUN_141c82060");
        for op in [MOB_MOVE, MOB_CTRL_ACK] {
            assert!(
                (0x03D9..=0x044D).contains(&op),
                "the second dispatcher's domain is LEA EAX,[RSI-0x3d9] / CMP EAX,0x74"
            );
        }
    }

    // ---- real wire data, 2026-08-19 -------------------------------------------------
    //
    // Two of the thirty `0x02FF` bodies from `world.log`, chosen for their element counts:
    // one element and four. The whole ladder in that capture is 111 / 132 / 153 / 174 bytes,
    // which is 90 + 21*n - the same 21 the path element decoder walks.

    /// Object 2001, one path element, 111 bytes.
    const CAPTURED_ONE_ELEMENT: &str = "d1070000010000ff0000000000000000000000000000000000\
        01000000ccddff00ccddff005087d93c00000000010000000070038b010000000001000070038b0100000\
        00028000000000000000438040000000faa8ebe5293b8f32800000000000000000000000300000000010000";

    /// Object 2000, four path elements, 174 bytes - and the mob the combat agent independently
    /// placed at `(424, 395)`.
    const CAPTURED_FOUR_ELEMENTS: &str = "d0070000010000ff000000000000000000000000000000000\
        001000000ccddff00ccddff005087d93c000000000100000000a8018b0100000000040000a6018b01d5ff\
        00002300000000000000035a000000a3018b01000000002300000000000000025a000000c2018b012b000\
        000230000000000000002f1020000c8018b012b00000025000000000000000293000000000faa8ebe57f5\
        c2992500000000000000000000000300000000010000";

    fn unhex(s: &str) -> Vec<u8> {
        let clean: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        assert!(clean.len().is_multiple_of(2), "odd hex length");
        (0..clean.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&clean[i..i + 2], 16).expect("hex"))
            .collect()
    }

    /// **The layout claim, against bytes the client actually sent.**
    ///
    /// This is the test that would have caught a wrong field width. It does not check a
    /// prefix - it checks that the head, the path and the 29-byte tail account for the body
    /// *exactly*, which is only true if every field before the path has the right size.
    #[test]
    fn mob_move_layout_decodes_the_captured_body() {
        for (hex, want_len, want_obj, want_elems, want_xy) in [
            (CAPTURED_ONE_ELEMENT, 111usize, 2001u32, 1i16, (880i16, 395i16)),
            (CAPTURED_FOUR_ELEMENTS, 174, 2000, 4, (424, 395)),
        ] {
            let body = unhex(hex);
            assert_eq!(body.len(), want_len, "fixture transcription");

            let got = parse_mob_move(&body).expect("a real body parses");
            assert_eq!(got.object_id, want_obj);
            assert_eq!(got.move_id, 1, "every mob reported move id 1 and stopped");
            assert_eq!(got.packed, 0x00, "141cb7f20");
            assert_eq!(got.move_action, 0xFF, "141cb7f31; 0xFF is 'no action' to 141c8176d");
            assert_eq!((got.x, got.y), want_xy);
            assert_eq!(got.element_count, want_elems);
            assert_eq!(
                got.path.len(),
                MOB_PATH_HEAD_LEN + want_elems as usize * MOB_PATH_ELEMENT_LEN
            );

            // The head is 46 bytes, and the tail is 29 plus the nibble-count byte the
            // rebroadcast does not carry. If any field width above were wrong this would
            // not add up.
            let head = 46;
            let tail = 1 + 29;
            assert_eq!(head + got.path.len() + tail, body.len(), "the body is fully accounted for");

            // `141cb827e MOV EBX,0xffddcc`, encoded twice at 141cb828c and 141cb829a.
            // A literal from the encoder turning up where the layout predicts it.
            assert_eq!(&body[29..33], &0x00ff_ddccu32.to_le_bytes(), "141cb828c");
            assert_eq!(&body[33..37], &0x00ff_ddccu32.to_le_bytes(), "141cb829a");

            // The tail's `u8` at 141cb85fe is `mob+0x960` - the controller level we granted.
            assert_eq!(
                body[body.len() - 3],
                CONTROL_NORMAL,
                "141cb85fe echoes mob+0x960: the client is telling us our own grant landed"
            );
        }
    }

    /// The path slice must stop before `0x02FF`'s trailing nibble-count byte, because
    /// `141c82000 XOR R8D,R8D` means `0x03D9` never reads one.
    #[test]
    fn the_path_slice_excludes_the_nibble_byte_that_only_0x02ff_carries() {
        let body = unhex(CAPTURED_ONE_ELEMENT);
        let got = parse_mob_move(&body).unwrap();
        assert_eq!(got.path.len(), 14 + 21);
        // The byte immediately after the path in the ORIGINAL body is the nibble count, and
        // it is zero in the capture. It must not be inside `path`.
        assert_eq!(body[46 + got.path.len()], 0, "141d5991f, gated on r8d");
        assert_eq!(&got.path[..], &body[46..46 + got.path.len()]);
    }

    #[test]
    fn a_short_or_ragged_body_is_none_rather_than_a_panic() {
        let body = unhex(CAPTURED_ONE_ELEMENT);
        for cut in [0usize, 1, 5, 7, 20, 45, 46, 60, 80] {
            let _ = parse_mob_move(&body[..cut]); // must not panic
        }
        assert_eq!(parse_mob_move(&[]), None);
        assert_eq!(parse_mob_move(&body[..45]), None, "cut inside the head");
        assert_eq!(parse_mob_move(&body[..60]), None, "cut inside the path");
        assert!(parse_mob_move(&body).is_some());

        // A count that claims more elements than the body holds must fail, not truncate.
        let mut lying = body.clone();
        lying[46 + 12] = 0x7F; // the i16 element count, low byte
        assert_eq!(parse_mob_move(&lying), None);
    }

    /// A negative element count is `MOVSX ECX,AX / TEST ECX,ECX / JLE` in the client - it
    /// bails. Here it must not become a huge `usize`.
    #[test]
    fn a_negative_element_count_is_rejected_rather_than_sign_extended() {
        let mut body = unhex(CAPTURED_ONE_ELEMENT);
        body[46 + 12] = 0xFF;
        body[46 + 13] = 0xFF; // count = -1
        assert_eq!(parse_mob_move(&body), None, "1404b26a9 is signed");
    }

    // ---- 0x03E4 MobCtrlAck ----------------------------------------------------------

    /// 26 bytes, fixed, and each field at the offset its read sits at.
    #[test]
    fn the_ctrl_ack_is_twenty_six_bytes_laid_out_where_the_reads_are() {
        let b = mob_ctrl_ack(2000, 7, false);
        assert_eq!(b.len(), MOB_CTRL_ACK_LEN);
        assert_eq!(&b[0..4], &2000u32.to_le_bytes(), "141d32b4d, the dispatcher");
        assert_eq!(&b[4..6], &7u16.to_le_bytes(), "141c8209e, the acknowledged move id");
        assert_eq!(b[6], 0, "141c820ab");
        assert_eq!(&b[7..11], &[0; 4], "141c820b7");
        assert_eq!(&b[11..15], &[0; 4], "141c820f0 skill id: 0 short-circuits");
        assert_eq!(&b[15..17], &[0; 2], "141c820fa skill level");
        assert_eq!(&b[17..21], &[0; 4], "141c82105 0 skips 140401b30");
        assert_eq!(&b[21..25], &[0; 4], "141c82114, read and discarded");
        assert_eq!(b[25], 0, "141c8211c, read and discarded");
    }

    /// `141c8221d TEST R13D,R13D / SETNE BL / ADD EBX,3` - the flag picks state 3 or 4, and
    /// it is the only field besides the move id that changes the client's behaviour.
    #[test]
    fn the_ctrl_acks_one_behavioural_flag_is_a_bare_boolean() {
        assert_eq!(mob_ctrl_ack(1, 1, false)[6], 0, "-> state 3");
        assert_eq!(mob_ctrl_ack(1, 1, true)[6], 1, "-> state 4");
        // and nothing else moves
        let a = mob_ctrl_ack(1, 1, false);
        let c = mob_ctrl_ack(1, 1, true);
        assert_eq!(a[..6], c[..6]);
        assert_eq!(a[7..], c[7..]);
    }

    /// The whole point of the packet: echoing the client's own move id lands on the
    /// `JNS` side of `141c82212 CMP EAX,ECX`.
    #[test]
    fn the_ctrl_ack_round_trips_the_move_id_from_a_real_report() {
        let req = parse_mob_move(&unhex(CAPTURED_ONE_ELEMENT)).unwrap();
        let ack = mob_ctrl_ack(req.object_id, req.move_id, false);
        assert_eq!(&ack[0..4], &req.object_id.to_le_bytes());
        assert_eq!(&ack[4..6], &req.move_id.to_le_bytes());
    }

    // ---- 0x03D9 MobMove -------------------------------------------------------------

    /// The rebroadcast is 25 fixed bytes wrapped around the controller's own path block.
    #[test]
    fn the_rebroadcast_wraps_the_path_verbatim() {
        let req = parse_mob_move(&unhex(CAPTURED_FOUR_ELEMENTS)).unwrap();
        let b = mob_move_broadcast(&req);

        assert_eq!(b.len(), MOB_MOVE_FIXED_LEN + req.path.len());
        assert_eq!(b.len(), 25 + 14 + 4 * 21);
        assert_eq!(&b[0..4], &2000u32.to_le_bytes(), "141d32b4d");
        assert_eq!(b[4], req.packed, "141c813d0 <- 0x02FF offset 6");
        assert_eq!(b[5], req.move_action, "141c813f1 <- 0x02FF offset 7");
        assert_eq!(&b[6..14], &[0; 8], "141c814c1, zero in all 30 captures");
        assert_eq!(b[14], 0, "141c814f4, mob+0x8c8 list empty");
        assert_eq!(b[15], 0, "141c815fe, mob+0x8d0 list empty");
        assert_eq!(&b[16..20], &[0; 4], "141c81635 gate off, so no eleven-word block");
        assert_eq!(&b[20..24], &[0; 4], "141c8173e, outside that gate");
        assert_eq!(&b[24..24 + req.path.len()], &req.path[..], "141c82009");
        assert_eq!(*b.last().unwrap(), 0, "141c82011");
    }

    /// The path inside the rebroadcast still carries the controller's coordinates, so a
    /// second client is told the same pixel the first one reported.
    #[test]
    fn the_rebroadcast_carries_the_controllers_own_coordinates() {
        let req = parse_mob_move(&unhex(CAPTURED_FOUR_ELEMENTS)).unwrap();
        assert_eq!((req.x, req.y), (424, 395));
        let b = mob_move_broadcast(&req);
        // path head: u32, i16 x, i16 y - at body offset 24 + 4.
        assert_eq!(i16::from_le_bytes([b[28], b[29]]), 424, "1404b265b");
        assert_eq!(i16::from_le_bytes([b[30], b[31]]), 395, "1404b2675");
    }

    /// A rebroadcast must never be one byte longer than the client expects, which is what
    /// copying `0x02FF`'s path *including* its nibble byte would do.
    #[test]
    fn the_rebroadcast_is_shorter_than_the_report_it_came_from() {
        for hex in [CAPTURED_ONE_ELEMENT, CAPTURED_FOUR_ELEMENTS] {
            let body = unhex(hex);
            let req = parse_mob_move(&body).unwrap();
            let out = mob_move_broadcast(&req);
            // 0x02FF = 46 head + path + 1 nibble + 29 tail; 0x03D9 = 24 + path + 1.
            assert_eq!(body.len() - out.len(), 46 + 1 + 29 - 25);
            assert!(out.len() < body.len());
        }
    }

    /// An empty path is structurally legal (`1404b26a9`'s `JLE` bails), and the builder must
    /// not produce a body that claims elements it does not carry.
    #[test]
    fn a_zero_element_path_still_builds_a_well_formed_rebroadcast() {
        let req = MobMoveRequest {
            object_id: 2000,
            move_id: 3,
            packed: 0,
            move_action: 0xFF,
            x: 10,
            y: 20,
            element_count: 0,
            path: vec![0; MOB_PATH_HEAD_LEN],
        };
        let b = mob_move_broadcast(&req);
        assert_eq!(b.len(), MOB_MOVE_FIXED_LEN + MOB_PATH_HEAD_LEN);
    }
}
