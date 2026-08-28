//! Let the client compute its own contact damage, by clearing one byte it sets on itself.
//!
//! The owner, 2026-08-28, twice: *"this is the official client and this behavior does not exist"*,
//! and then *"try harder to see how the client can compute its own mob damage and then sending
//! that to the server instead. This is one of the ways that both the server and client can
//! agree."* Both times they were right, and the second push is what produced this.
//!
//! # The client DOES compute contact damage. It is thrown away four instructions later.
//!
//! `FUN_14288ac30` is the `0x00E5` builder. When there is no attack record - a contact hit -
//! it branches to `0x14288b2d8`, and that path is a real calculation: **[L]**
//!
//! ```text
//! 14288b2dc  cmp  dword [rdi+0xe8], 0
//! 14288b2e3  jle  0x14288ca17            ; zero attack power -> bail out entirely
//! 14288b2f0  mov  edx, [rsi+0x309c]
//! 14288b2f6  call 0x1401ba9d0            ; the de-obfuscating getter
//! 14288b30d  call 0x1401ba9d0
//! 14288b312  movd xmm1, dword [rdi+0xe8] ; the mob's attack power
//! 14288b326  mulsd xmm0, xmm1
//! 14288b32a  divsd xmm0, [0x143277e78]   ; = 100.0, read off .rdata
//! 14288b337  cvttsd2si r12d, xmm0        ; <- a REAL damage, in r12d
//! ```
//!
//! `r12d` then survives untouched to `0x14288b3f0`, where the gate is:
//!
//! ```text
//! 14288b415  call 0x14090d160            ; GetOption(0xAE, 0)
//! 14288b41c  jne  0x14288b483            ; option non-zero -> keep the computed damage
//! 14288b41e  cmp  byte [r15+0x544a], al
//! 14288b425  je   0x14288b483            ; flag zero      -> keep the computed damage
//! 14288b45e  call 0x140266ea0            ; otherwise: REPLACE it
//! 14288b463  mov  r12d, eax
//! ```
//!
//! and `FUN_140266ea0` is handed a struct whose only populated field came from
//! `[r13+0x50]`, or **zero when there is no attack record**. That is why every contact hit in
//! **198 of 198 captures across 160 archived logs** reports `1`: not because the client cannot
//! work the damage out, but because a good number is computed and then discarded.
//!
//! # So the fix is to stop the discard, and both sides then agree by construction
//!
//! The client sends what it drew, in the same `0x00E5` the server already parses. There is no
//! second number to reconcile - which is exactly the property the owner asked for.
//!
//! Of the two gate conditions, option `0xAE` is unreachable: **read at eleven sites in `.text`
//! and written at none** (byte-scanned, with the two known sites as the positive control), and
//! the getter returns its `edx` default of `0` on a miss. **[L]**
//!
//! The other is reachable, because the client sets it on itself with an immediate:
//!
//! ```text
//! 142883687  48 c7 86 48 54 00 00  01 00 01 00
//!            mov qword ptr [rsi+0x5448], 0x00010001
//! ```
//!
//! Little-endian, that writes `01` to `+0x5448` and `01` to **`+0x544a`**. Changing the
//! immediate to `0x00000001` clears `+0x544a` and leaves `+0x5448` as it was - and `+0x5448`
//! is never read anywhere (`research/damage-number-suppress.md` §8 lists it as written once and
//! never looked at). **One byte, at instruction offset 9.**
//!
//! # What this replaces
//!
//! An earlier version of this file nopped the renderer call at `0x1428aca14` - it *hid* the
//! wrong number. That was the wrong fix and it was written before the touch path had been
//! read: it left the client still **sending** `1`, so the server and the client still
//! disagreed, and the disagreement was merely off screen. Clearing the flag fixes the number
//! at its source, which the drawer and the builder both then use.
//!
//! # Both gate sites move together, and that is the point
//!
//! `0x544a` is compared in **eight** functions, `FUN_14288ac30` (the builder) and
//! `FUN_1428aa0a0` (the drawer) among them. Clearing the byte takes every one of them down
//! its `je`, so the number that is computed, the number that is sent and the number that is
//! drawn are the same number.
//!
//! # What is still not established
//!
//! Whether `[rdi+0xe8]` is non-zero for our mobs. **If it is zero the client bails at
//! `14288b2e3` and this patch changes nothing**, which is a real possible outcome and the test
//! plan says so rather than assuming success. `research/damage-number-two-numbers.md`.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::hook::log;

/// `0x142883687` - the client's own `mov qword ptr [rsi+0x5448], 0x00010001`.
const HIT_DAMAGE_FLAG_STORE_RVA: usize = 0x142883687 - 0x140000000;

/// The whole instruction, as it must be found. Checking all eleven bytes rather than the one
/// being changed is what makes "wrong build" a log line instead of a silent corruption.
const EXPECT: [u8; 11] =
    [0x48, 0xC7, 0x86, 0x48, 0x54, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00];

/// The same instruction with the immediate `0x00010001` -> `0x00000001`: `+0x5448` keeps the
/// `1` it always had, `+0x544a` becomes `0`.
const REPLACE: [u8; 11] =
    [0x48, 0xC7, 0x86, 0x48, 0x54, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00];

const PAGE_EXECUTE_READWRITE: u32 = 0x40;

extern "system" {
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

static APPLIED: AtomicBool = AtomicBool::new(false);

/// Apply the patch when `hitnumber=off` is in the marker. Idempotent; logs once either way.
///
/// Read-before-write, then read-back: a write that silently did not take looks exactly like a
/// patch that did not help, and on screen those are the same picture.
pub unsafe fn install() {
    if APPLIED.swap(true, Ordering::SeqCst) {
        return;
    }
    if crate::session::marker_token("hitnumber=").as_deref() != Some("off") {
        return;
    }

    let at = crate::hook::base() + HIT_DAMAGE_FLAG_STORE_RVA;
    if !crate::session::can_read(at, EXPECT.len()) {
        log(&format!(
            "***** HITNUMBER: {at:#x} is not readable - NOT patched. The client will keep \
             discarding the contact damage it computed and reporting 1 *****"
        ));
        return;
    }

    let found = std::slice::from_raw_parts(at as *const u8, EXPECT.len()).to_vec();
    if found != EXPECT {
        log(&format!(
            "***** HITNUMBER: refusing to patch {at:#x} - expected {EXPECT:02x?} \
             (mov qword [rsi+0x5448], 0x10001) and found {found:02x?}. Either the RVA is wrong \
             for this build or something else is already there. NOT patched *****"
        ));
        return;
    }

    let mut old = 0u32;
    if VirtualProtect(at as *mut c_void, REPLACE.len(), PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log("***** HITNUMBER: VirtualProtect failed - NOT patched *****");
        return;
    }
    std::ptr::copy_nonoverlapping(REPLACE.as_ptr(), at as *mut u8, REPLACE.len());
    VirtualProtect(at as *mut c_void, REPLACE.len(), old, &mut old);

    let after = std::slice::from_raw_parts(at as *const u8, REPLACE.len()).to_vec();
    if after == REPLACE {
        log(&format!(
            "***** HITNUMBER: {at:#x} is now {after:02x?} - user+0x544a will be 0. The client \
             should now KEEP the contact damage it computes at 0x14288b337 instead of \
             replacing it, so the number it DRAWS and the number it SENDS in 0x00E5 are the \
             same real number. Watch world.log: 'The CLIENT claimed N' should stop being 1. If \
             it is still 1, the mob's attack power at mob+0xe8 is zero and the client bailed \
             at 0x14288b2e3 before computing anything *****"
        ));
    } else {
        log(&format!(
            "***** HITNUMBER: wrote the patch at {at:#x} and read back {after:02x?}, which is \
             not {REPLACE:02x?}. The write did not take; treat the client as UNPATCHED *****"
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The patch changes **one byte**, and it is the third byte of the immediate.
    ///
    /// Asserted as a difference rather than by eye: a second changed byte would alter the
    /// displacement or the opcode, and the instruction would stop being the one that was read.
    #[test]
    fn exactly_one_byte_changes_and_it_is_in_the_immediate() {
        assert_eq!(EXPECT.len(), REPLACE.len(), "length must be preserved");
        let differ: Vec<usize> =
            (0..EXPECT.len()).filter(|i| EXPECT[*i] != REPLACE[*i]).collect();
        assert_eq!(differ, vec![9], "only instruction offset 9 may change");
        assert_eq!(EXPECT[9], 0x01);
        assert_eq!(REPLACE[9], 0x00);
    }

    /// The immediate goes from `0x00010001` to `0x00000001`, which is what clears `+0x544a`
    /// while leaving `+0x5448` alone. Decoded from the bytes rather than trusted.
    #[test]
    fn the_immediate_clears_offset_544a_and_keeps_5448() {
        let before = u32::from_le_bytes(EXPECT[7..11].try_into().unwrap());
        let after = u32::from_le_bytes(REPLACE[7..11].try_into().unwrap());
        assert_eq!(before, 0x0001_0001);
        assert_eq!(after, 0x0000_0001);
        // Byte 0 of the immediate lands at +0x5448, byte 2 at +0x544a.
        assert_eq!(before.to_le_bytes()[0], 1, "+0x5448 was 1");
        assert_eq!(after.to_le_bytes()[0], 1, "and stays 1 - it is never read either way");
        assert_eq!(before.to_le_bytes()[2], 1, "+0x544a was 1: the gate passed");
        assert_eq!(after.to_le_bytes()[2], 0, "+0x544a is now 0: the gate fails, damage kept");
    }

    /// The displacement really is `0x5448`, so the byte being cleared really is `+0x544a`.
    #[test]
    fn the_store_targets_5448() {
        assert_eq!(u32::from_le_bytes(EXPECT[3..7].try_into().unwrap()), 0x5448);
        assert_eq!(0x5448 + 2, 0x544a, "the immediate's third byte is the gate byte");
    }

    /// The RVA is derived from the VA rather than typed.
    #[test]
    fn the_rva_is_the_va_less_the_image_base() {
        assert_eq!(HIT_DAMAGE_FLAG_STORE_RVA, 0x142883687 - 0x140000000);
        assert_eq!(HIT_DAMAGE_FLAG_STORE_RVA, 0x2883687);
    }
}
