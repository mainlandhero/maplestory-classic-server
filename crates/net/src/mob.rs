//! Mob spawn - inbound `0x03C6`, `MobEnterField`.
//!
//! Everything here is read out of `client-patched\MapleStory.exe`; the working is in
//! `research/mob-spawn.md`, and every field below carries the address the client reads it at.
//!
//! # This body killed a client once. The layout was never the problem; one byte's value was
//!
//! 2026-08-19: 40 of these went out on map 40 and the client took `0xC0000005` at
//! `0x141c810b0` on the **first** one. The hook writes its `opcode=… elapsed_us=` line after
//! the dispatch returns and no such line exists for `0x03C6`, so the fault is *inside* the
//! first mob's dispatch - not a volume problem, and `-MobLimit 1` reproduces it.
//!
//! A watch on the faulting function then measured the cause: **[`FieldMob::move_action`], body
//! offset 35, sent as `0`.** Zero means `action == 0`, and `action == 0` is the single case in
//! which the animation object `encodeInit` builds at `141c50da5` calls back into the mob's
//! *second* interface - `mob+0x2c0` - which `encodeInit` does not create until `141c50e77`,
//! `0x148` bytes later. The constant [`MOVE_ACTION_MIN_SAFE`] carries the whole chain.
//! `FieldMob::new` now defaults that byte to `2`.
//!
//! The **layout** is right and three instruments say so, so do not change it: the 52 reads in
//! `encodeInit`, a CFG dominator test that picks out exactly the 35 unconditional ones and
//! finds them identical to the 35 emitted here, and a WZ dump that settles both
//! template-driven blocks as absent for all 193 mobs. `research/mob-spawn.md` section 11.
//!
//! > **A retraction.** The first pass at this blamed `mob+0x2b8` and said no packet byte could
//! > reach it. The field is real and that sentence about it is true, but it is **the wrong
//! > field**: the faulting method's `this` is `mob+8`, the second base subobject the
//! > constructor gives its own vtable at `141c4cf1c`, so its `[this+0x2b8]` is **`mob+0x2c0`**.
//! > The error was reading a `this`-relative offset as if `this` were the primary pointer.
//! > `research/mob-spawn.md` section 11.7 records how it was caught.
//!
//! # The body is four decoders, not one
//!
//! ```text
//! FUN_141d33630   case 0x3c6                                       11 bytes
//!   u8 sealed, u32 objectId, u8 calcDamageIndex, u32 templateId, u8 forcedStatPresent
//!   |
//!   +- FUN_141d2efc0(pool, objectId): is this id already in the pool?
//!   |    FOUND -> FUN_141c76190 and RETURN. The body stops at 31 bytes.
//!   |    NEW   -> FUN_141c76190, then the virtual below.
//!   |
//!   FUN_141c76190                                                  20 bytes
//!     raw[20] = the temporary-stat presence mask (160 bits)
//!     FUN_14046fba0(stats, vec, &mask, packet, now)   0 bytes when the mask is 0
//!   |
//!   CALL [vtable+0x38] = FUN_141c4ff80  (NEW branch only)         106 bytes
//!     position, foothold, appear type, HP, and 30 more fields
//! ```
//!
//! `11 + 20 + 106 =` [`MOB_ENTER_FIELD_LEN`].
//!
//! # The record is not self-describing
//!
//! Whether the last 106 bytes are read at all depends on whether the **client** already holds
//! that object id. The mob pool is rebuilt on every field entry, so on a fresh field every
//! spawn takes the new-mob branch - but **re-sending a live object id makes the client stop
//! after 31 bytes and desynchronise everything after it in the stream.** Send each object id
//! once per field entry. (`141d33723` / `141d33757`, [L])

/// `MobEnterField`. Routed by `CField::OnPacket` (`FUN_141820080`) to the mob pool's
/// dispatcher `FUN_141D30E80` on singleton `[0x143ABFE00]`, whose `case 0x3c6` calls
/// `FUN_141d33630`. Read out of `research/msexe-mobpool.c`. **[L]**
pub const MOB_ENTER_FIELD: u16 = 0x03C6;

/// Length of a [`mob_enter_field`] body for an ordinary mob: **137 bytes**.
///
/// This is the minimum, and it is exact for a mob whose `appear_type` needs no option word
/// and whose template sets neither optional flag. Four things make it longer, all of them
/// visible in [`FieldMob`]:
///
/// | what | cost | decided by |
/// |---|---|---|
/// | `appear_type` is `-3`, `-6` or `>= 0` | +4 | us |
/// | `template_id` in [`SPECIAL_TEMPLATE_IDS`] | +1 | the template id |
/// | `template[0x104] != 0`, the WZ node **`patrol`** | +16 | the WZ template |
/// | `template[0x1a0] != 0`, the WZ property **`targetFromSvr`** | +4 | the WZ template |
///
/// Use [`FieldMob::body_len`] rather than this constant when any of them may apply - but for
/// *this* client both template rows are dead: **none of the 193 mob images in
/// `client-patched/Data/Mob/Mob_000.wz` carries `patrol` or `targetFromSvr`**, so 137 is exact
/// for every mob in the game. The machinery stays because the client's parse still depends on
/// the flags, and a later WZ would not have to agree. **[L]**
pub const MOB_ENTER_FIELD_LEN: usize = 137;

/// The temporary-stat presence mask: **20 raw bytes, 160 bits**, read at `141c76276`.
///
/// `FUN_141c76190` zeroes exactly 20 bytes (`XORPS`/`MOVUPS` for 16, `MOV dword,0` for 4) and
/// immediately reads 20 into them, then passes the buffer as `param_3` to `FUN_14046fba0`,
/// which bounds its bit index at `CMP EDI,0xa0`. Every one of that function's **331** packet
/// reads sits behind a test of a bit in this mask, so **all-zero costs nothing further**.
/// That was the question `research/mob-spawn.md` used to stop on. **[L]**
pub const MOB_TEMP_STAT_MASK_LEN: usize = 20;

/// Object ids that are a multiple of this take an unexplored path in `FUN_141d33630`.
///
/// Both branches compute `uVar6 == (uVar6 / 0xb2) * 0xb2`; a multiple ends up calling through
/// slot 2 of a stack functor whose vtable is `PTR_LAB_143409208`. It reads no packet bytes, so
/// it is not a desync - but it is unexplored, and avoiding it is free. **[L]**
pub const OBJECT_ID_MULTIPLE_TO_AVOID: u32 = 178;

/// Three template ids for which `FUN_14045b1a0` returns 1, which makes the client read **one
/// extra byte** right after `move_action` (`141c5043d`).
///
/// Read off `14045b1a0`: `SUB ECX,0x87f4b0 / JZ` then `SUB ECX,0x64 / JZ` then
/// `CMP ECX,0x107a7d / JZ`, i.e. `0x87f4b0`, `0x87f514`, `0x986f91`. **[L]**
///
/// The v214 reference gates the same byte on `8910000 || 8910100 || 9990033` - the same
/// *shape*, three different *numbers*. A clean illustration of why that source names fields
/// and never settles values. **[I]** for the correspondence, **[L]** for these three.
pub const SPECIAL_TEMPLATE_IDS: [u32; 3] = [8_909_488, 8_909_588, 9_990_545];

/// The smallest [`FieldMob::move_action`] that does **not** crash the client: **2**.
///
/// Body offset 35 is `action * 2 + facing` - the client splits it that way itself at
/// `141c50dba` (`AND EDI,1` for facing, `SAR EAX,1` for action, then a 16-entry jump table on
/// `action - 1`). We sent **0** on 2026-08-19 and the client took `0xC0000005`. This is why:
///
/// ```text
/// 141c503f2  the byte is read and stashed XOR-obfuscated in mob+0x3dc / mob+0x3e0
/// 141c50cfd  EDI = ROL([mob+0x3e0],5) XOR [mob+0x3dc]      the plaintext, back again
/// 141c50da5  CALL R14 = iface->vtable[0x118] = FUN_142ac10d0, with EDI as argument 7
/// 142ac1123  a pure forwarding shim: args 5-8 copied verbatim to FUN_1409c50a0
/// 1409c6852  EBX = [RBP+0x140] = argument 7 = our byte
/// 1409c6858  TEST EBX,0xfffffffe
/// 1409c685e  JNE  1409c687f                    <-- >= 2 SKIPS the call below
/// 1409c687a  CALL [mob8_vtable + 8] = FUN_141c81040
/// 141c81094  reads mob+0x2c0 ... which encodeInit does not create until 141c50e77
/// 141c810b0  dereferences 0x848 and dies
/// ```
///
/// So **`move_action & !1 == 0` walks into a client bug**: the callback happens `0x148` bytes
/// before the field it needs exists. Anything `>= 2` takes the `JNE` and never enters it.
/// Measured, not guessed - the watch on `141c81040` logged `rdx=0 r8=0 r9=0`, which matches
/// `XOR EDX,EDX / XOR R8D,R8D / MOV R9D,EBX&1` only when `EBX == 0`. **[L]**
pub const MOVE_ACTION_MIN_SAFE: u8 = 2;

/// Is this object id safe to use as a mob's pool key?
///
/// Zero pulls in a second `u32` read at `141d336a1` and desynchronises the body; multiples of
/// [`OBJECT_ID_MULTIPLE_TO_AVOID`] take an unexplored branch. **[L]**
pub fn object_id_is_usable(id: u32) -> bool {
    id != 0 && id % OBJECT_ID_MULTIPLE_TO_AVOID != 0
}

/// The first usable object id at or after `id`. Never returns 0 or a multiple of 178.
pub fn next_usable_object_id(id: u32) -> u32 {
    let mut id = if id == 0 { 1 } else { id };
    while !object_id_is_usable(id) {
        id = id.wrapping_add(1);
        if id == 0 {
            id = 1;
        }
    }
    id
}

/// One mob standing on a field, as the client's mob pool reads it.
///
/// The fields with names are the ones two instruments agree on. Everything the client reads
/// and this struct does not expose is sent as zero, with two deliberate exceptions noted in
/// [`mob_enter_field`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldMob {
    /// The pool's hash key, read at `141d3368e`. **Must be non-zero and should not be a
    /// multiple of 178** - see [`object_id_is_usable`]. Must also be unique among the mobs
    /// currently alive on the client's field: a repeat makes the handler take its
    /// "already in the pool" branch and stop reading after 31 bytes. **[L]**
    pub object_id: u32,
    /// Read at `141d33701` and passed straight to `FUN_140495990`, the `Mob/%07d.img` loader.
    ///
    /// **It must exist.** The template supplies max HP, and the HP field is divided by it at
    /// `141c50502` with an `IDIV` that has **no zero check** - a template with no max HP kills
    /// the client with a divide error. **[L]**
    pub template_id: u32,
    /// `141c4ffc5`, sign-extended `MOVSX R8D,AX`, low dword of the point at `mob+0xd30`. **[D]**
    pub x: i16,
    /// `141c501b0`, sign-extended, high dword of the same point. **[D]**
    pub y: i16,
    /// `141c50456`. Looked up in the field's foothold map, `FUN_142df6c50(DAT_143ac18d8, v)` -
    /// the same map `research/npc-spawn.md` identified for NPC read 9, which returns null on a
    /// miss and is not null-checked at the call site. Send the WZ value. **[D]**
    pub fh: i16,
    /// `141c50465`, looked up in the same foothold map. The reference calls it
    /// `homeFoothold`. **[D]** for the lookup, **[I]** for the name.
    pub home_fh: i16,
    /// `141c504e9`, a `u64`. Turned into a **percentage** immediately:
    /// `mob+0xb60 = hp * 100 / FUN_141c8a730(mob)`, where that function returns the template's
    /// max HP scaled by [`FieldMob::hp_scale_percent`]. **[D]**
    ///
    /// **This is the mob's answer to the NPC `alpha` bug.** Zero here is a structurally valid
    /// packet that produces a mob at 0% HP. Send the template's `maxHP` from `Mob/%07d.img`
    /// for a full bar.
    pub hp: u64,
    /// `141c503f2`, stored XOR-obfuscated at `mob+0x3dc`/`mob+0x3e0`. It is
    /// **`action * 2 + facing`** - `141c50dba` splits it exactly that way. **[L]**
    ///
    /// # This byte killed the client, and it is the only one that did
    ///
    /// **Never send a value below [`MOVE_ACTION_MIN_SAFE`].** `0` and `1` are `action == 0`,
    /// and `1409c6858 TEST EBX,0xfffffffe / JNE` makes `action == 0` the one case that calls
    /// back into the mob's second interface **before `encodeInit` has created it** - see
    /// [`MOVE_ACTION_MIN_SAFE`] for the whole chain and `research/mob-spawn.md` section 11.
    ///
    /// [`FieldMob::new`] therefore defaults to `2` (action 1, facing 0), not `0`. The
    /// previous default of `0` is what `0xC0000005 at 0x141c810b0` was.
    pub move_action: u8,
    /// `141c50485`, read as a **signed** byte into `mob+0x1168`.
    ///
    /// `141c504c2`-`141c504cb` is `CMP EBX,-0x3 / JZ take`, `CMP EBX,-0x6 / JZ take`,
    /// `TEST EBX,EBX / JS skip` - so values of `-3`, `-6` or `>= 0` make the client read a
    /// further `u32` ([`FieldMob::appear_option`]), and only a negative value other than
    /// `-3`/`-6` does not. **[L]**
    ///
    /// The reference's condition is character-for-character the same, including the unusual
    /// `-6`, and it annotates the value `// init -> -2, -1 else`. That is why the default is
    /// `-2`. **[I]** - if mobs pop in wrong, `-1` is the one-byte alternative.
    pub appear_type: i8,
    /// Only emitted when [`FieldMob::appear_type`] is `-3`, `-6` or `>= 0` (`141c504d0`).
    pub appear_option: u32,
    /// `141c504db` -> `mob+0x8c0`. `FUN_141c8a730` multiplies the template's max HP by it and
    /// divides by 100 **unless it is 0 or 100**, both of which mean "unscaled". Send 100.
    /// **[D]** - the arithmetic is in `FUN_141c8a730`, the position is the listing.
    pub hp_scale_percent: u32,
    /// Four `i32` read at `141c5052c`..`141c50552`, emitted **only when the WZ template's byte
    /// at `+0x104` is non-zero** (`CMP byte ptr [RCX + 0x104],R15B` at `141c50520`). **[L]**
    ///
    /// The server has to know this about the template, because the client's parse depends on
    /// it. **Settled 2026-08-19:** `template+0x104` is written from the WZ node **`patrol`** -
    /// `FUN_14047d990` does `GetItem(node, u"patrol")` at `140483bea` and then
    /// `MOV byte [RAX+0x104],1` at `140483c64` or `,0` at `140483d23`. **No mob image in this
    /// client has a `patrol` node**, so `None` is right for all 193 of them. **[L]**
    pub patrol: Option<[i32; 4]>,
    /// One `u32` at `141c50ac0`, emitted **only when the template's byte at `+0x1a0` is
    /// non-zero** (`FUN_140479e70` is `MOVZX EAX,byte ptr [RCX+0x1a0]; RET`).
    ///
    /// **Settled 2026-08-19:** that byte is `GetInt(node, u"targetFromSvr", 0) != 0`, written
    /// at `140480979` right after the `lea` of the name at `14048094f`. **No mob image in this
    /// client sets `targetFromSvr`**, so `None` is right for all 193. **[L]**
    pub target_from_server: Option<u32>,
}

impl FieldMob {
    /// An ordinary field mob: full HP, no scaling, no optional template blocks, `appear_type`
    /// `-2`, and `move_action` [`MOVE_ACTION_MIN_SAFE`]. `object_id` is nudged to the next
    /// usable value rather than trusted.
    ///
    /// `move_action` is **2, not 0**. Zero is the value that crashed the client on
    /// 2026-08-19; the constant's docs carry the eight-step chain from that byte to
    /// `0xC0000005 at 0x141c810b0`.
    pub fn new(object_id: u32, template_id: u32, x: i16, y: i16, fh: i16, hp: u64) -> Self {
        Self {
            object_id: next_usable_object_id(object_id),
            template_id,
            x,
            y,
            fh,
            home_fh: fh,
            hp,
            move_action: MOVE_ACTION_MIN_SAFE,
            appear_type: -2,
            appear_option: 0,
            hp_scale_percent: 100,
            patrol: None,
            target_from_server: None,
        }
    }

    /// Does this mob's `appear_type` make the client read the extra option word? **[L]**
    pub fn has_appear_option(&self) -> bool {
        self.appear_type == -3 || self.appear_type == -6 || self.appear_type >= 0
    }

    /// Does this template id make the client read one extra byte after `move_action`? **[L]**
    pub fn has_special_template_byte(&self) -> bool {
        SPECIAL_TEMPLATE_IDS.contains(&self.template_id)
    }

    /// Exact body length for this mob, [`MOB_ENTER_FIELD_LEN`] plus whatever the optional
    /// blocks add.
    pub fn body_len(&self) -> usize {
        MOB_ENTER_FIELD_LEN
            + usize::from(self.has_appear_option()) * 4
            + usize::from(self.has_special_template_byte())
            + if self.patrol.is_some() { 16 } else { 0 }
            + if self.target_from_server.is_some() { 4 } else { 0 }
    }
}

/// `MobEnterField` - put one mob on the field the client is standing in.
///
/// # Where the layout came from
///
/// The head is 7 reads in `FUN_141d33630` bounded by `.pdata` to
/// `[0x141d33630, 0x141d33c5e)`. The 20-byte mask is `FUN_141c76190`. The 106-byte tail is the
/// **virtual** `vtable+0x38` = `FUN_141c4ff80`, whose 52 reads were counted twice - 50 in the
/// listing, 52 in the decompiler - and reconciled by finding `0x1406e8ef0`, a bare
/// `JMP 0x1406e8b80` that is an **eighth** packet-read primitive the project's table does not
/// list. The 106 is a shortest-path solve over that function's control-flow graph, and it
/// agrees read-for-read with a hand trace of the listing.
///
/// All eight concrete mob classes share slot 7, and each vtable was reached through the
/// constructor that installs it rather than by aligning tables - so this is not the
/// `/OPT:ICF` trap.
///
/// # The two fields not sent as zero, and why
///
/// * **byte 5, `calcDamageIndex`** is sent as **1**, not 0. It is the reference's initialised
///   value; the client stores it at `mob+0x620` and no consumer was traced. **[I]**
/// * **offset 74** is sent as **-1**. `141c506a1` is `CMP R14D,-0x1 / JLE skip`, so a negative
///   value skips a `FUN_141cdf1e0(mob, v - 13, ...)` call that a zero would enter with an
///   out-of-range index. It costs no bytes either way. **[L]**
///
/// # Fields I am least sure of, most dangerous first
///
/// 1. ~~**[`FieldMob::patrol`] and [`FieldMob::target_from_server`]**~~ - **settled**, and they
///    are absent for every mob this client has. The parser is `FUN_14047d990`, not
///    `FUN_140495990`, which is only the cache.
/// 2. **[`FieldMob::hp`]** - structurally free to be zero, and zero means a mob at 0% HP.
///    Exactly the shape of the NPC `isEnabled`/`alpha` bug.
/// 3. **[`FieldMob::appear_type`]** - `-2` from the reference's own annotation. `-1` is the
///    alternative and costs the same bytes.
/// 4. **byte 0 (`sealedInsteadDead`) and byte 5 (`calcDamageIndex`)** - both go to a plain
///    byte slot (`mob+0x508`, `mob+0x620`) with no consumer traced on the entry path.
/// 5. **offsets 40, 42, 44** - a `u8` and two `i16` between `home_fh` and `hp_scale_percent`
///    with no traced consumer. The reference has a single `teamForMCarnival` byte in that
///    region, which is one byte where this client reads five.
/// 6. **the 17 remaining zeros in `encodeInit`.** Sent as zero on a "no readable consumer"
///    argument, which is the argument `research/setfield-zero-audit.md` exists to distrust.
pub fn mob_enter_field(mob: &FieldMob) -> Vec<u8> {
    debug_assert!(
        object_id_is_usable(mob.object_id),
        "object id must be non-zero and not a multiple of 178"
    );

    let mut b = Vec::with_capacity(mob.body_len());

    // -- head: FUN_141d33630, 11 bytes ------------------------------------------------
    b.push(0); //                                          0   u8  141d3365a sealed?
    b.extend_from_slice(&mob.object_id.to_le_bytes()); //   1   u32 141d3368e pool key
    b.push(1); //                                          5   u8  141d336f3 calcDamageIndex
    b.extend_from_slice(&mob.template_id.to_le_bytes()); // 6   u32 141d33701 Mob/%07d.img
    b.push(0); //                                         10   u8  141d33734 forced stat: NO

    // -- temporary stats: FUN_141c76190, 20 bytes -------------------------------------
    b.extend_from_slice(&[0u8; MOB_TEMP_STAT_MASK_LEN]); //   141c76276 mask, no bits set

    // -- encodeInit: the virtual FUN_141c4ff80, 106 bytes -----------------------------
    b.extend_from_slice(&mob.x.to_le_bytes()); //          31  i16 141c4ffc5 x
    b.extend_from_slice(&mob.y.to_le_bytes()); //          33  i16 141c501b0 y
    debug_assert!(
        mob.move_action >= MOVE_ACTION_MIN_SAFE,
        "move_action 0 or 1 makes the client dereference an uninitialised mob+0x2c0 \
         (0xC0000005 at 0x141c810b0) - see MOVE_ACTION_MIN_SAFE"
    );
    b.push(mob.move_action); //                            35  u8  141c503f2 action*2+facing
    if mob.has_special_template_byte() {
        b.push(0); //                                          u8  141c5043d 3 template ids
    }
    b.extend_from_slice(&mob.fh.to_le_bytes()); //         36  i16 141c50456 foothold
    b.extend_from_slice(&mob.home_fh.to_le_bytes()); //    38  i16 141c50465 home foothold
    b.push(0); //                                          40  u8  141c50474 -> mob+0x116c
    b.push(mob.appear_type as u8); //                      41  i8  141c50485 appearType
    if mob.has_appear_option() {
        b.extend_from_slice(&mob.appear_option.to_le_bytes()); // u32 141c504d0
    }
    b.extend_from_slice(&0i16.to_le_bytes()); //           42  i16 141c50499 -> mob+0x1228
    b.extend_from_slice(&0i16.to_le_bytes()); //           44  i16 141c504aa -> mob+0x122c
    b.extend_from_slice(&mob.hp_scale_percent.to_le_bytes()); // 46 u32 141c504db HP scale %
    b.extend_from_slice(&mob.hp.to_le_bytes()); //         50  u64 141c504e9 CURRENT HP
    b.extend_from_slice(&0u32.to_le_bytes()); //           58  u32 141c5050e effect item id
    if let Some(patrol) = mob.patrol {
        for v in patrol {
            b.extend_from_slice(&v.to_le_bytes()); //          u32 141c5052c.. patrol range
        }
    }
    b.extend_from_slice(&0u32.to_le_bytes()); //           62  u32 141c5056f -> mob+0xb68
    b.extend_from_slice(&0u32.to_le_bytes()); //           66  u32 141c505a8 -> mob+0xb6c
    b.extend_from_slice(&0u32.to_le_bytes()); //           70  u32 141c50621 refImgMobId: none
    b.extend_from_slice(&(-1i32).to_le_bytes()); //        74  i32 141c5066f  < 0 skips a call
    b.extend_from_slice(&0i32.to_le_bytes()); //           78  i32 141c50680
    b.extend_from_slice(&0u32.to_le_bytes()); //           82  u32 141c5068a
    b.push(0); //                                          86  u8  141c50694
    b.extend_from_slice(&0u32.to_le_bytes()); //           87  u32 141c506bd count = 0
    b.extend_from_slice(&0u32.to_le_bytes()); //           91  u32 141c507bb -> mob+0xd64
    b.push(0); //                                          95  u8  141c507c9 gate: costs 8
    b.extend_from_slice(&0u32.to_le_bytes()); //           96  u32 141c50808 count = 0
    b.push(0); //                                         100  u8  141c509d2 gate: costs 120
    b.extend_from_slice(&0u16.to_le_bytes()); //          101  str 141c50a66 empty
    if let Some(target) = mob.target_from_server {
        b.extend_from_slice(&target.to_le_bytes()); //         u32 141c50ac0 template+0x1a0
    }
    b.extend_from_slice(&0u32.to_le_bytes()); //          103  u32 141c50ada count = 0
    b.push(0); //                                         107  u8  141c532ab gate, no reads
    b.push(0); //                                         108  u8  141c534d9 -> mob+0xfdc
    b.extend_from_slice(&0u32.to_le_bytes()); //          109  u32 141c534ea -> mob+0xfe0
    b.extend_from_slice(&0u32.to_le_bytes()); //          113  u32 141c534f8 gate: costs 4
    b.extend_from_slice(&0u32.to_le_bytes()); //          117  u32 141c53518 gate, no reads
    b.extend_from_slice(&0u32.to_le_bytes()); //          121  u32 141c535ae
    b.extend_from_slice(&0u32.to_le_bytes()); //          125  u32 141c535b8 -> mob+0x1118
    b.extend_from_slice(&0u32.to_le_bytes()); //          129  u32 141c53695 count = 0
    b.extend_from_slice(&0u32.to_le_bytes()); //          133  u32 141c536df

    debug_assert_eq!(b.len(), mob.body_len());
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tutorial_mob() -> FieldMob {
        // Map 30's six mobs are template 1 in gm-handbook/mobs.txt. The geometry here is a
        // placeholder - the point of the test is the offsets, not the coordinates.
        FieldMob::new(3000, 1, 250, -80, 12, 15)
    }

    #[test]
    fn a_mob_enter_field_body_is_137_bytes_with_the_fields_where_the_client_reads_them() {
        let m = tutorial_mob();
        let b = mob_enter_field(&m);
        assert_eq!(b.len(), MOB_ENTER_FIELD_LEN);
        assert_eq!(b.len(), 137);
        assert_eq!(b.len(), m.body_len());

        // head - FUN_141d33630
        assert_eq!(b[0], 0, "141d3365a sealed?");
        assert_eq!(&b[1..5], &3000u32.to_le_bytes(), "141d3368e objectId");
        assert_eq!(b[5], 1, "141d336f3 calcDamageIndex");
        assert_eq!(&b[6..10], &1u32.to_le_bytes(), "141d33701 templateId");
        assert_eq!(b[10], 0, "141d33734 forced stat block absent");

        // the temporary-stat mask - FUN_141c76190
        assert_eq!(&b[11..31], &[0u8; 20], "141c76276 raw[20] mask");
        assert_eq!(MOB_TEMP_STAT_MASK_LEN, 20);

        // encodeInit - FUN_141c4ff80
        assert_eq!(&b[31..33], &250i16.to_le_bytes(), "141c4ffc5 x");
        assert_eq!(&b[33..35], &(-80i16).to_le_bytes(), "141c501b0 y");
        assert_eq!(b[35], 2, "141c503f2 action*2+facing - NEVER 0, see MOVE_ACTION_MIN_SAFE");
        assert_eq!(&b[36..38], &12i16.to_le_bytes(), "141c50456 fh");
        assert_eq!(&b[38..40], &12i16.to_le_bytes(), "141c50465 homeFh");
        assert_eq!(b[40], 0, "141c50474");
        assert_eq!(b[41] as i8, -2, "141c50485 appearType");
        assert_eq!(&b[42..44], &0i16.to_le_bytes(), "141c50499");
        assert_eq!(&b[44..46], &0i16.to_le_bytes(), "141c504aa");
        assert_eq!(&b[46..50], &100u32.to_le_bytes(), "141c504db hp scale %");
        assert_eq!(&b[50..58], &15u64.to_le_bytes(), "141c504e9 hp");
        assert_eq!(&b[58..62], &0u32.to_le_bytes(), "141c5050e effect item id");
        assert_eq!(&b[62..66], &0u32.to_le_bytes(), "141c5056f");
        assert_eq!(&b[66..70], &0u32.to_le_bytes(), "141c505a8");
        assert_eq!(&b[70..74], &0u32.to_le_bytes(), "141c50621 refImgMobId");
        assert_eq!(&b[74..78], &(-1i32).to_le_bytes(), "141c5066f must be < 0");
        assert_eq!(&b[78..82], &0i32.to_le_bytes(), "141c50680");
        assert_eq!(&b[82..86], &0u32.to_le_bytes(), "141c5068a");
        assert_eq!(b[86], 0, "141c50694");
        assert_eq!(&b[87..91], &0u32.to_le_bytes(), "141c506bd count");
        assert_eq!(&b[91..95], &0u32.to_le_bytes(), "141c507bb");
        assert_eq!(b[95], 0, "141c507c9 gate");
        assert_eq!(&b[96..100], &0u32.to_le_bytes(), "141c50808 count");
        assert_eq!(b[100], 0, "141c509d2 gate over raw[120]");
        assert_eq!(&b[101..103], &0u16.to_le_bytes(), "141c50a66 empty string");
        assert_eq!(&b[103..107], &0u32.to_le_bytes(), "141c50ada count");
        assert_eq!(b[107], 0, "141c532ab gate");
        assert_eq!(b[108], 0, "141c534d9");
        assert_eq!(&b[109..113], &0u32.to_le_bytes(), "141c534ea");
        assert_eq!(&b[113..117], &0u32.to_le_bytes(), "141c534f8 gate");
        assert_eq!(&b[117..121], &0u32.to_le_bytes(), "141c53518 gate");
        assert_eq!(&b[121..125], &0u32.to_le_bytes(), "141c535ae");
        assert_eq!(&b[125..129], &0u32.to_le_bytes(), "141c535b8");
        assert_eq!(&b[129..133], &0u32.to_le_bytes(), "141c53695 count");
        assert_eq!(&b[133..137], &0u32.to_le_bytes(), "141c536df");
    }

    /// The opcode is `case 0x3c6` of `FUN_141D30E80`, not a guess.
    #[test]
    fn the_opcode_is_the_mob_pools_enter_field_case() {
        assert_eq!(MOB_ENTER_FIELD, 0x03C6);
        assert!((0x03C6..=0x044E).contains(&MOB_ENTER_FIELD), "inside the mob pool range");
    }

    /// Zero pulls in a second `u32` at `141d336a1`; multiples of 178 take an unexplored
    /// branch. Neither may reach the wire.
    #[test]
    fn object_ids_are_never_zero_and_never_a_multiple_of_178() {
        assert!(!object_id_is_usable(0));
        assert!(!object_id_is_usable(178));
        assert!(!object_id_is_usable(178 * 17));
        assert!(object_id_is_usable(1));
        assert!(object_id_is_usable(3000));

        assert_eq!(next_usable_object_id(0), 1);
        assert_eq!(next_usable_object_id(178), 179);
        assert_eq!(next_usable_object_id(356), 357);
        assert_eq!(next_usable_object_id(3000), 3000);

        // and the constructor applies it rather than trusting the caller
        assert_eq!(FieldMob::new(178, 1, 0, 0, 0, 1).object_id, 179);
        assert_eq!(FieldMob::new(0, 1, 0, 0, 0, 1).object_id, 1);
        for id in [0u32, 178, 178 * 3, 65_360] {
            let m = FieldMob::new(id, 1, 0, 0, 0, 1);
            let b = mob_enter_field(&m);
            let on_wire = u32::from_le_bytes([b[1], b[2], b[3], b[4]]);
            assert!(object_id_is_usable(on_wire), "{on_wire} reached the wire");
        }
    }

    /// `141c504c2`-`141c504cb`: `-3`, `-6` and every non-negative value make the client read a
    /// further `u32`. Everything else must not carry one.
    #[test]
    fn the_appear_option_word_appears_exactly_when_the_client_reads_it() {
        for (appear, extra) in [
            (-1i8, false),
            (-2, false),
            (-3, true),
            (-4, false),
            (-5, false),
            (-6, true),
            (-7, false),
            (0, true),
            (1, true),
            (127, true),
        ] {
            let mut m = tutorial_mob();
            m.appear_type = appear;
            m.appear_option = 0xDEAD_BEEF;
            assert_eq!(m.has_appear_option(), extra, "appearType {appear}");

            let b = mob_enter_field(&m);
            assert_eq!(b.len(), MOB_ENTER_FIELD_LEN + usize::from(extra) * 4);
            assert_eq!(b[41] as i8, appear);
            if extra {
                assert_eq!(&b[42..46], &0xDEAD_BEEFu32.to_le_bytes(), "141c504d0 option");
                // and everything after it has shifted by four
                assert_eq!(&b[50..54], &100u32.to_le_bytes(), "hp scale, +4");
                assert_eq!(&b[54..62], &15u64.to_le_bytes(), "hp, +4");
            }
        }
    }

    /// The byte that killed the client on 2026-08-19, pinned so it cannot come back.
    ///
    /// `1409c6858 TEST EBX,0xfffffffe / JNE` skips the callback into the mob's second
    /// interface for every `move_action >= 2`, and takes it for `0` and `1` - and that
    /// callback reads `mob+0x2c0`, which `encodeInit` does not create until `0x148` bytes
    /// later. `FieldMob::new` must never hand out a mob in that window.
    #[test]
    fn move_action_is_never_the_value_that_dereferences_an_uninitialised_mob() {
        assert_eq!(MOVE_ACTION_MIN_SAFE, 2);
        // the constructor, whatever it is handed
        for id in [1u32, 3000, 65_360] {
            let m = FieldMob::new(id, 1, 0, 0, 0, 1);
            assert!(m.move_action >= MOVE_ACTION_MIN_SAFE, "FieldMob::new gave {}", m.move_action);
            assert_eq!(mob_enter_field(&m)[35], MOVE_ACTION_MIN_SAFE);
        }
        // and the guard the client applies: only `& 0xfffffffe == 0` is fatal
        for v in 0u8..=7 {
            let fatal = v & 0xFE == 0;
            assert_eq!(fatal, v < MOVE_ACTION_MIN_SAFE, "move_action {v}");
        }
        // the byte reaches the wire verbatim, and the offset does not move
        let mut m = tutorial_mob();
        m.move_action = 5;
        let b = mob_enter_field(&m);
        assert_eq!(b[35], 5, "141c503f2");
        assert_eq!(&b[36..38], &12i16.to_le_bytes(), "fh still at 36");
        assert_eq!(b.len(), MOB_ENTER_FIELD_LEN);
    }

    /// Three template ids add one byte after `move_action` (`FUN_14045b1a0`).
    #[test]
    fn the_three_special_template_ids_add_one_byte_after_move_action() {
        for id in SPECIAL_TEMPLATE_IDS {
            let mut m = tutorial_mob();
            m.template_id = id;
            assert!(m.has_special_template_byte());
            let b = mob_enter_field(&m);
            assert_eq!(b.len(), MOB_ENTER_FIELD_LEN + 1);
            assert_eq!(b[35], 2, "141c503f2 action*2+facing - NEVER 0, see MOVE_ACTION_MIN_SAFE");
            assert_eq!(b[36], 0, "141c5043d the extra byte");
            assert_eq!(&b[37..39], &12i16.to_le_bytes(), "fh, shifted by one");
        }
        // and an ordinary template does not
        assert!(!tutorial_mob().has_special_template_byte());
        assert_eq!(SPECIAL_TEMPLATE_IDS, [8_909_488, 8_909_588, 9_990_545]);
    }

    /// The two blocks the WZ template decides. Both default to absent; when present they land
    /// exactly where the client reads them.
    #[test]
    fn the_template_driven_blocks_are_optional_and_correctly_placed() {
        let mut m = tutorial_mob();
        assert_eq!(m.body_len(), 137);

        m.patrol = Some([-100, 100, 7, 8]);
        assert_eq!(m.body_len(), 153);
        let b = mob_enter_field(&m);
        assert_eq!(b.len(), 153);
        assert_eq!(&b[58..62], &0u32.to_le_bytes(), "141c5050e effect item id");
        assert_eq!(&b[62..66], &(-100i32).to_le_bytes(), "141c5052c patrol[0]");
        assert_eq!(&b[66..70], &100i32.to_le_bytes(), "patrol[1]");
        assert_eq!(&b[70..74], &7i32.to_le_bytes(), "patrol[2]");
        assert_eq!(&b[74..78], &8i32.to_le_bytes(), "patrol[3]");
        assert_eq!(&b[90..94], &(-1i32).to_le_bytes(), "141c5066f, +16");

        m.patrol = None;
        m.target_from_server = Some(0x1234_5678);
        assert_eq!(m.body_len(), 141);
        let b = mob_enter_field(&m);
        assert_eq!(&b[101..103], &0u16.to_le_bytes(), "141c50a66 empty string");
        assert_eq!(&b[103..107], &0x1234_5678u32.to_le_bytes(), "141c50ac0");
        assert_eq!(&b[107..111], &0u32.to_le_bytes(), "141c50ada count, +4");

        // all four at once, and the length still adds up
        let mut m = tutorial_mob();
        m.template_id = SPECIAL_TEMPLATE_IDS[0];
        m.appear_type = -3;
        m.patrol = Some([0; 4]);
        m.target_from_server = Some(0);
        assert_eq!(m.body_len(), 137 + 1 + 4 + 16 + 4);
        assert_eq!(mob_enter_field(&m).len(), m.body_len());
    }

    /// HP is the mob's `alpha`: a structurally perfect body with `hp = 0` is a mob at 0%.
    /// `mob+0xb60 = hp * 100 / maxHp`, and the `IDIV` at `141c50502` has no zero check.
    #[test]
    fn hp_and_its_scale_are_carried_verbatim() {
        let mut m = tutorial_mob();
        m.hp = 15;
        m.hp_scale_percent = 100;
        let b = mob_enter_field(&m);
        assert_eq!(&b[46..50], &100u32.to_le_bytes());
        assert_eq!(&b[50..58], &15u64.to_le_bytes());

        m.hp = u32::MAX as u64 + 1;
        assert_eq!(&mob_enter_field(&m)[50..58], &4_294_967_296u64.to_le_bytes());

        // FieldMob::new must never hand out an unscaled-to-zero mob by accident
        let d = FieldMob::new(3000, 1, 0, 0, 0, 999);
        assert_eq!(d.hp, 999);
        assert_eq!(d.hp_scale_percent, 100);
        assert_eq!(d.appear_type, -2);
        assert_eq!(d.home_fh, d.fh);
    }

    /// Every negative coordinate and foothold must go on the wire sign-extended, because the
    /// client reads all three with `MOVSX`.
    #[test]
    fn coordinates_and_footholds_are_signed() {
        let mut m = tutorial_mob();
        m.x = -46;
        m.y = -1;
        m.fh = -2;
        m.home_fh = -3;
        let b = mob_enter_field(&m);
        assert_eq!(&b[31..33], &[0xD2, 0xFF], "x = -46, 141c4ffca MOVSX");
        assert_eq!(&b[33..35], &[0xFF, 0xFF], "y = -1");
        assert_eq!(&b[36..38], &[0xFE, 0xFF], "fh = -2");
        assert_eq!(&b[38..40], &[0xFD, 0xFF], "homeFh = -3");
    }
}
