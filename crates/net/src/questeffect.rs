//! The quest-finish fanfare: `0x02D1` `UserEffectLocal` with effect **15**.
//!
//! Full working, with every address and every control:
//! **`research/quest-complete-effect.md`**.
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ or a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred.
//!
//! # Why this exists
//!
//! The owner, after a run on 2026-08-21:
//!
//! > *"Quest finish still does not trigger the SFX for quest finish (this is a different SFX
//! > than quest completion)."*
//!
//! Turning a quest in sent five packets and none of them plays a sound. The one that does is
//! **not** a sub-case of `0x0089` and **not** something the client does by itself when the
//! quest record changes state — both of those were checked and both are negative, with the
//! blind spots named, in `research/quest-complete-effect.md` §4. It is a dedicated opcode.
//!
//! # This module builds nothing of its own
//!
//! [`crate::stats`] already carries the two effect opcodes and their bodies, decoded for the
//! level-up work. Re-implementing the wire format here would give this client two builders
//! that could drift apart, so every function below delegates. **What is new here is the
//! effect id, and the evidence for it.**
//!
//! # The sound is in the WZ. The animation is not.
//!
//! `Sound_001.wz/Game.img/QuestClear` exists — 7410 bytes, 2411 ms. **[L]**
//!
//! `Effect_000.wz/BasicEff.img` has **40** nodes and `QuestClear` is not one of them, nor is
//! it in `Effect/_Canvas/_Canvas_000.wz/BasicEff.img`. **[L]** Of the seven
//! `Effect/BasicEff.img/*` paths this client holds, it is the only one whose node was cut.
//!
//! **So expect sound and no picture**, and say so before the run rather than after it. The
//! sound call is unconditional after the animation call — no branch, and nothing tests the
//! animation's return value — so the missing art cannot suppress the fanfare:
//!
//! ```text
//! 14278e18a  call 0x140e16070    ; the animation; returns 0 on a node it cannot resolve
//! 14278e18f  xor  r8d, r8d
//! 14278e192  lea  edx, [r8 + 0x64]
//! 14278e196  mov  rcx, [rip+0x12ba303]   ; [0x143A484A0] = L"QuestClear"
//! 14278e19d  call 0x1429f14c0            ; the sound, at volume 100
//! ```
//!
//! **[D]** that a missing node cannot throw: `FUN_140e16070` null-checks the resolver's
//! result at `140e16173` and returns 0, and `FUN_140dc12e0` reaches its `_com_issue_errorex`
//! at `140dc13a0` only on a node that *did* resolve. The named blind spot, because a hedged
//! negative here is worth nothing without one: the innermost lookup is a virtual call
//! (`14090de8b call [rax+0x48]`) that I did not disassemble. If that raises rather than
//! returning a failing HRESULT, this reasoning does not apply and effect 15 must be dropped.
//!
//! # There is no EXP-gain sound in this client
//!
//! `Sound/Game.img` contains `IncEXP` and `questCount`, and **neither name occurs anywhere in
//! the executable, in ASCII or UTF-16**, while the same scan finds `PickUpItem`, `Portal`,
//! `QuestAlert`, `LevelUp` and `JobChanged`. They are dead assets. **[L]** So the second half
//! of the owner's report cannot be answered with a sound, and it does not need to be answered with
//! a packet either: the `0x0089` kind-3 EXP line is **already going out** at quest completion
//! — `research/quest-complete-effect.md` §6 has the bytes off the wire.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials.

// ---------------------------------------------------------------------------------------
// Routing — restated rather than aliased, so a reader of this file sees the numbers
// ---------------------------------------------------------------------------------------

/// **`0x02D1` `UserEffectLocal`** — body `u8 effect`, one byte.
///
/// `CField::OnPacket` hands `0x2C5..=0x39E` to the local user's switch `FUN_14289a3a0`,
/// which is a plain `opcode - 0x2c5` index into the table at `0x14289d660`: **[L]**
///
/// ```text
/// 14289a3f3  lea  eax, [rdx - 0x2c5]
/// 14289a3f9  cmp  eax, 0xd9 / ja <default>
/// 14289a40d  mov  edx, [rcx + rax*4 + 0x289d660]
/// 14289a417  jmp  rdx
///    index 0x0C -> 14289a439  mov rdx,r13 / mov rcx,r15 / call 0x1427863f0
/// ```
///
/// `0x2C5 + 0x0C = 0x02D1`, and `FUN_14289a3a0` reads **nothing** before dispatching, so the
/// whole body belongs to the effect handler.
///
/// Identical to [`crate::stats::USER_EFFECT_LOCAL`]; a test below pins the two together.
pub const USER_EFFECT_LOCAL: u16 = crate::stats::USER_EFFECT_LOCAL;

/// **`0x02AF` `UserEffectRemote`** — body `u32 charId, u8 effect`, five bytes.
///
/// What *other* players' clients are sent. See [`quest_clear_remote`] for why this server
/// has nowhere to send it yet.
pub const USER_EFFECT_REMOTE: u16 = crate::stats::USER_EFFECT_REMOTE;

// ---------------------------------------------------------------------------------------
// The effect id
// ---------------------------------------------------------------------------------------

/// Effect **15** — the quest-finish fanfare.
///
/// `FUN_1427863f0` reads one `u8` at `0x14278644e` and runs **two** switches on it. Both had
/// to be read, and the first one is what makes the body a single byte:
///
/// ```text
/// 142786482  lea   ecx, [rbx - 8]                      ; FIRST switch, on effect-8
/// 1427864a8  cmp   ecx, 0x45 / ja 0x14278bd20
/// 1427864b4  movzx eax, byte [0x142791300 + idx]       ; two-level MSVC table
/// 1427864bc  mov   ecx, dword [0x14279129c + eax*4]
///
///   effect 15 -> byte-table index 24 -> 0x14278bd20, the DEFAULT arm: no packet read
///
/// 14278bd5d  movzx ebx, byte [rbp+0x80]                ; SECOND switch, on the effect itself
/// 14278bd7d  cmp   ebx, 0x54 / ja <exit>
/// 14278bd8d  mov   ecx, dword [0x142791348 + ebx*4]
///
///   index 15 -> 0x14278e11d, extent [0x14278e11d, 0x14278e1a7)
/// ```
///
/// That arm loads `[0x143A46F58]` — a qword holding `0x1432AE518`, the UTF-16 string
/// `Effect/BasicEff.img/QuestClear` — and `[0x143A484A0]` — `0x1432B1F68`, the bare name
/// `QuestClear` — and plays them. `tools/listing.py` reports **zero** `READ` sites inside the
/// extent. All **[L]**.
///
/// `python tools/dataref.py 0x143a46f58` and `0x143a484a0` each give exactly **two** readers
/// and they are the same two functions: this arm, and `FUN_1428a1460` — which is opcode
/// `0x02D8`, the **pet-skill** notice (string ids `0x9F5`/`0x9F6`/`0x9F9`, *"The Auto-Pickup
/// Skill has been added…"*) reusing the same fanfare. Checked, not assumed. **[L]**
///
/// # The neighbours, which is why 15 is believable and not just located
///
/// Resolving every `.data` string slot the handler reads: **[L]**
///
/// ```text
///  0  Effect/BasicEff.img/LevelUp             + sound LevelUp
/// 13  (sound Portal only, no animation)
/// 14  Effect/BasicEff.img/JobChanged          + sound JobChanged
/// 15  Effect/BasicEff.img/QuestClear          + sound QuestClear      <- this one
/// 17  Effect/ItemEff.img/%d
/// 21  sounds EnchantSuccess / EnchantFailure
/// 83  Effect/BasicEff.img/CitizenshipGet
/// 84  Effect/BasicEff.img/CitizenshipGradeUp
/// ```
///
/// The v83-era reference enum runs `PLAY_PORTAL_SE(7), JOB_CHANGED(8), QUEST_COMPLETE(9)`
/// with `LEVEL_UP(0)` — the same three consecutive, in the same order, at a constant `+6`.
/// That is **[I]** corroboration and nothing more; the reference scored 1 of 8 against a
/// held-out control here. Every number above is read out of this client's own jump table.
pub const EFFECT_QUEST_CLEAR: u8 = 0x0F;

/// Effect **0**, re-exported so a caller can see that this table is the level-up table.
///
/// Restated from [`crate::stats::EFFECT_LEVEL_UP`] and pinned equal to it by a test.
pub const EFFECT_LEVEL_UP: u8 = crate::stats::EFFECT_LEVEL_UP;

// ---------------------------------------------------------------------------------------
// Builders
// ---------------------------------------------------------------------------------------

/// The body of a [`USER_EFFECT_LOCAL`] carrying [`EFFECT_QUEST_CLEAR`]. **One byte.**
///
/// Send it with opcode [`USER_EFFECT_LOCAL`] **from a settled field**, after the quest record
/// and the rewards. See the two preconditions below and
/// `research/quest-complete-effect.md` §7 for the packet order.
///
/// # Two ways it does nothing, in silence
///
/// Both sit between the handler's two switches and apply to **every** effect, level-up
/// included. Effects `0x4F`, `0x50` and `0x51` bypass them; 15 does not, and neither does 0.
/// **[L]** - corrected 2026-09-08, it read `0x52` here for days. The ladder subtracts as it
/// goes, so only the first comparison is against a literal effect id:
/// `sub ecx,0x4f; je` is `0x4F`, then `sub ecx,1; je` is `0x50`, and `cmp ecx,1; je` matches
/// when `ecx` already holds `effect - 0x50`, so it is **`0x51`**. Reading the third
/// comparison's operand as an effect id is what produced the wrong row.
///
/// ```text
/// 14278bd29  call 0x141892840      ; the current field
/// 14278bd31  je   0x14278bd45      ;   NULL -> bl = 1
/// 14278bd3b  call 0x14182ffd0      ;   else bl = field->[0xa8]->[0x2d9], a field-info byte
/// 14278bd4a  call 0x142826340      ; a user-state predicate -> al
/// 14278bd75  test dl,dl / je <exit>  ; either non-zero -> NOTHING HAPPENS
/// ```
///
/// * **No field object -> dropped.** This is the same rule `research/quest-state.md` already
///   imposes on `0x0089`: send it in reply to something the client sent from a settled field,
///   never alongside a `SetField`.
/// * `FUN_142826340` is non-zero while the user is in morph/transform state `0x1a`, `0x1b`,
///   `0x1c` or `0x20`, and on two specific field ids. **[L]** for the reads; that they are
///   all false in an ordinary session is **[I]**.
///
/// Neither is a blocking request, so a dropped effect costs a missing sound and not a frozen
/// UI.
pub fn quest_clear_local() -> Vec<u8> {
    crate::stats::user_effect_local(EFFECT_QUEST_CLEAR)
}

/// The body of a [`USER_EFFECT_REMOTE`] carrying [`EFFECT_QUEST_CLEAR`] — `u32 charId, u8`.
///
/// **Do not send this to the player who finished the quest.** `FUN_1429bb720` reads the
/// `u32` and looks it up in the *remote* user pool; a client does not hold itself there, so
/// it would be dropped in silence.
///
/// It is also not sendable to anyone else yet: `research/exp-sharing.md` records that a
/// channel is a process and each connection is a thread with its own socket, so this server
/// has no way to push a packet into another session. This builder exists so that when that
/// gap is closed the byte layout is already pinned, not so it can be called today.
pub fn quest_clear_remote(char_id: u32) -> Vec<u8> {
    crate::stats::user_effect_remote(char_id, EFFECT_QUEST_CLEAR)
}

// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// The routing constants, so a future edit that renumbers one fails here rather than on
    /// The owner's screen.
    #[test]
    fn the_opcodes_are_the_two_effect_entries() {
        // FUN_14289a3a0: index = opcode - 0x2c5, table 0x14289d660, index 0x0C -> 0x1427863f0
        assert_eq!(USER_EFFECT_LOCAL, 0x02D1);
        assert_eq!(0x2C5 + 0x0C, USER_EFFECT_LOCAL);
        // FUN_1429bb720: index = opcode - 0x29e, table 0x1429bbc34, index 0x11 -> 0x1427863f0
        assert_eq!(USER_EFFECT_REMOTE, 0x02AF);
        assert_eq!(0x29E + 0x11, USER_EFFECT_REMOTE);
    }

    /// One packet, two modules. This file must never grow a second wire format.
    #[test]
    fn this_module_does_not_fork_the_wire_format() {
        assert_eq!(USER_EFFECT_LOCAL, crate::stats::USER_EFFECT_LOCAL);
        assert_eq!(USER_EFFECT_REMOTE, crate::stats::USER_EFFECT_REMOTE);
        assert_eq!(EFFECT_LEVEL_UP, crate::stats::EFFECT_LEVEL_UP);
        assert_eq!(quest_clear_local(), crate::stats::user_effect_local(0x0F));
        assert_eq!(quest_clear_remote(206), crate::stats::user_effect_remote(206, 0x0F));
    }

    /// **The number this file exists for.** Table `0x142791348`, index 15, arm `0x14278e11d`,
    /// which reads `[0x143A46F58]` and `[0x143A484A0]`.
    #[test]
    fn the_quest_clear_effect_is_fifteen() {
        assert_eq!(EFFECT_QUEST_CLEAR, 15);
        assert_eq!(EFFECT_QUEST_CLEAR, 0x0F);
        // Its neighbours in the same table, from the resource strings each arm loads.
        assert_eq!(EFFECT_LEVEL_UP, 0, "index 0 -> Effect/BasicEff.img/LevelUp");
        assert_eq!(EFFECT_QUEST_CLEAR - 1, 14, "index 14 -> .../JobChanged");
        assert_eq!(EFFECT_QUEST_CLEAR - 2, 13, "index 13 -> sound Portal");
    }

    /// The local body is **one byte and nothing else**, because effect 15 takes the first
    /// switch's default arm (byte-table index 24 -> `0x14278bd20`) and the second switch's
    /// arm at `0x14278e11d` contains no packet read at all.
    #[test]
    fn the_local_body_is_exactly_one_byte() {
        let b = quest_clear_local();
        assert_eq!(b.len(), 1, "FUN_1427863f0's only read is the u8 at 14278644e");
        assert_eq!(b, vec![0x0F]);
    }

    /// The remote body is the `u32` `FUN_1429bb720` reads at `0x1429bb745` followed by the
    /// same effect byte. Little-endian, like every other field on this wire.
    #[test]
    fn the_remote_body_is_the_char_id_then_the_effect() {
        let b = quest_clear_remote(206);
        assert_eq!(b.len(), 5);
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 206);
        assert_eq!(b[4], EFFECT_QUEST_CLEAR);
        // 200 is FIRST_CHARACTER_ID; a body that renumbered from 1 has bitten this project
        // before, so use a real id in the test rather than a small one.
        assert_eq!(quest_clear_remote(200)[0], 200);
    }

    /// The effect byte is bounded at `0x54` by `14278bd7d cmp ebx,0x54 / ja <exit>`; anything
    /// above it leaves the handler without doing anything. 15 is comfortably inside.
    #[test]
    fn the_effect_byte_is_inside_the_switchs_bound() {
        const { assert!(EFFECT_QUEST_CLEAR <= 0x54, "14278bd7d cmp ebx,0x54 / ja") };
    }

    /// This is **not** the `0x0089` message packet, and confusing the two is the mistake this
    /// whole investigation had to rule out first. `research/quest-complete-effect.md` §4.
    #[test]
    fn this_is_not_the_message_packet() {
        assert_ne!(USER_EFFECT_LOCAL, crate::message::MESSAGE);
        assert_ne!(USER_EFFECT_REMOTE, crate::message::MESSAGE);
        assert_ne!(USER_EFFECT_LOCAL, crate::quest::MESSAGE);
    }
}
