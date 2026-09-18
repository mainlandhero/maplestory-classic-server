//! Let `0x0138 UserAvatarModified` dress a character that is already on the field, by
//! opening the one jump that keeps the client's own apply from running.
//!
//! The owner, 2026-09-18: *"the equipment does take effect immediately, but there's a weird super
//! brief character blink as it disappears and reappears on the map. The regular maplestory
//! does not have this behavior. The equipment updates are instant."* And, the same run:
//! *"whenever a piece of equipment is changed on a client with a pet, the pet completely
//! respawns and appear sad/hungry until moments later."* Both are the same thing: without an
//! in-place update the server's only way to redress another player's copy of a character is
//! `0x0225` then `0x0224`, which destroys and rebuilds the remote `CUser` and its pet.
//!
//! # The gate
//!
//! `0x0138` reaches `FUN_142797be0(user, look)`, which walks the list at `user+0x1200` and,
//! for each node, asks `FUN_1407f5ce0([node+0x30c])` before calling the dress primitive
//! `FUN_140f80140` through the shim at `0x1420dd920`. `FUN_1407f5ce0` is three bytes of
//! `xor eax,eax; ret` - a `/OPT:ICF`-folded "always false" - so the `je` after it is always
//! taken and the look is never applied. `research/beauty-2026-09-09.md` §4.2 and
//! `research/naked-character.md` §5.1 established that at byte level, and a launch measured
//! the primitive never firing. **[L]**
//!
//! ```text
//! 142797de6  call 0x1407f5ce0       ; xor eax,eax; ret
//! 142797deb  test eax, eax          ; 85 c0
//! 142797ded  je   0x142797e07       ; 74 18   <- this
//! 142797def  mov  rdx, r15          ; the look
//! 142797df5  call 0x1420dd920       ; -> FUN_140f80140(node+0x78, look, 0)
//! 142797e02  call 0x1420dd220       ; (node, 1)
//! ```
//!
//! `FUN_140f80140` is the real dress routine: `tools/callers.py` finds it called from five
//! functions in the `0x1429c...` user-pool region, the code that builds a remote character
//! from a `0x0224`. So the two bytes at `142797ded` become `90 90`, the loop body runs for
//! every node, and the apply that Nexon wrote runs on the objects Nexon put in that list.
//! The stub itself is left alone: with ICF it may be every `return 0` in the image.
//!
//! # What is not established
//!
//! What the nodes at `user+0x1200` are, and so whether the apply lands on the drawn avatar
//! or on something else. **[I]**. The server sends `0x0138` for a look change by default
//! (`world::Config::look_change_reenter = false`) and the observer's screen is the test: gear
//! changes with no blink means the list holds the avatar; nothing changes means the list is
//! empty for a remote user and the fallback is `-LookReenter`; a `CLIENT FAULT` on the
//! *observer* means the nodes are not what the apply expects, and the fallback is
//! `-NoAvatarModPatch -LookReenter`. Plan step TO(c).

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::hook::log;

/// `0x142797deb` - `test eax,eax` followed by the `je` that skips the apply.
const AVATAR_APPLY_GATE_RVA: usize = 0x142797deb - 0x140000000;

/// `test eax, eax ; je +0x18`. All four bytes are checked so a different build is a log line.
pub const EXPECT: [u8; 4] = [0x85, 0xC0, 0x74, 0x18];

/// The same `test`, then two `nop`s where the `je` was: the apply always runs.
pub const REPLACE: [u8; 4] = [0x85, 0xC0, 0x90, 0x90];

const PAGE_EXECUTE_READWRITE: u32 = 0x40;

extern "system" {
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

static APPLIED: AtomicBool = AtomicBool::new(false);

/// Apply the patch unless `avatarmod=off` is in the marker. Idempotent; logs once either way.
///
/// Read-before-write, then read-back, the same discipline as `hitnumber` and `beautytext`.
pub unsafe fn install() {
    if APPLIED.swap(true, Ordering::SeqCst) {
        return;
    }
    if crate::session::marker_token("avatarmod=").as_deref() == Some("off") {
        log("AVATARMOD: avatarmod=off in the marker - 0x0138 stays a no-op; the server must use -LookReenter for other players to see a look change");
        return;
    }

    let at = crate::hook::base() + AVATAR_APPLY_GATE_RVA;
    if !crate::session::can_read(at, EXPECT.len()) {
        log(&format!(
            "***** AVATARMOD: {at:#x} is not readable - NOT patched. 0x0138 stays a no-op *****"
        ));
        return;
    }

    let found = std::slice::from_raw_parts(at as *const u8, EXPECT.len()).to_vec();
    if found == REPLACE {
        log(&format!("AVATARMOD: {at:#x} already holds the patch; nothing to do"));
        return;
    }
    if found != EXPECT {
        log(&format!(
            "***** AVATARMOD: refusing to patch {at:#x} - expected {EXPECT:02x?} (test eax,eax; \
             je +0x18) and found {found:02x?}. Either the RVA is wrong for this build or \
             something else is already there. NOT patched; 0x0138 stays a no-op *****"
        ));
        return;
    }

    let mut old = 0u32;
    if VirtualProtect(at as *mut c_void, REPLACE.len(), PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log("***** AVATARMOD: VirtualProtect failed - NOT patched *****");
        return;
    }
    std::ptr::copy_nonoverlapping(REPLACE.as_ptr(), at as *mut u8, REPLACE.len());
    VirtualProtect(at as *mut c_void, REPLACE.len(), old, &mut old);

    let after = std::slice::from_raw_parts(at as *const u8, REPLACE.len()).to_vec();
    if after == REPLACE {
        log(&format!(
            "AVATARMOD: patched {at:#x} - the je at 0x142797ded is two nops, so a 0x0138 \
             UserAvatarModified now runs FUN_140f80140 on every node of the user's +0x1200 \
             list. Another player's equip or hair change should redraw their copy in place, \
             no blink, pet untouched. If it still blinks the server is on -LookReenter; if \
             nothing changes the list is empty for a remote user; a CLIENT FAULT here is the \
             apply on the wrong object - relaunch with -NoAvatarModPatch -LookReenter"
        ));
    } else {
        log(&format!(
            "***** AVATARMOD: wrote the patch at {at:#x} and read back {after:02x?}, which is \
             not {REPLACE:02x?}. The write did not take; 0x0138 stays a no-op *****"
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The patch changes exactly the two bytes of the `je` and nothing else, and what it
    /// changes them to is two one-byte `nop`s - so the instruction stream stays aligned and
    /// the `test` before it is untouched.
    #[test]
    fn only_the_je_changes_and_it_becomes_two_nops() {
        assert_eq!(EXPECT.len(), REPLACE.len());
        assert_eq!(&EXPECT[..2], &[0x85, 0xC0], "test eax,eax");
        assert_eq!(&REPLACE[..2], &EXPECT[..2], "the test is kept");
        assert_eq!(EXPECT[2], 0x74, "je rel8");
        assert_eq!(EXPECT[3], 0x18, "to 0x142797e07 = 0x142797def + 0x18");
        assert_eq!(0x142797defu64 + 0x18, 0x142797e07, "the jump's target is the loop's next-node step");
        assert_eq!(&REPLACE[2..], &[0x90, 0x90]);
        assert_eq!(AVATAR_APPLY_GATE_RVA + 0x140000000 + 2, 0x142797ded, "the je sits two bytes past the patch start");
    }
}
