//! Stop the client drawing its own damage number, which for touch damage is always `1`.
//!
//! The owner, 2026-08-28: *"this is the official client and this behavior does not exist (showing 2
//! damage numbers by mobs) means that something is wrong with our implementation."* They were
//! right that two numbers is wrong. What follows is why the fix is a client patch and not a
//! packet, and it is a measurement rather than a preference.
//!
//! # The client cannot compute touch damage. 198 observations, no exceptions.
//!
//! Across **160 distinct archived world logs**, every mob→player hit this project has ever
//! captured carries `attack index -1` - contact damage, not a numbered mob skill - and **every
//! one claims damage `1`**, whatever the mob. That includes template 1003, `PADamage` 149,
//! which *has* attack nodes in its own WZ and still hit with its body.
//!
//! The mechanism is four instructions above the gate, in `FUN_14288ac30`, the `0x00E5` builder:
//!
//! ```text
//! 14288b400  test r13, r13
//! 14288b403  je   0x14288b40b
//! 14288b405  mov  edi, [r13 + 0x50]   ; from the mob's ATTACK record
//! 14288b40b  mov  edi, r14d           ; ...or ZERO, when there is none
//! ```
//!
//! A contact hit has no attack record - `research/touch-damage.md` §3 - so `edi` is `0`, and
//! the damage function it feeds floors at `1`. **[D]**
//!
//! # And no packet can turn the draw off
//!
//! The draw is gated on `GetOption(0xAE, 0) == 0` **and** `user+0x544a != 0`. Both are
//! constants on this client:
//!
//! * option `0xAE` is **read at eleven sites in `.text` and written at none** - byte-scanned
//!   for `mov ecx/edx, 0xAE`, with the two already-known sites as the positive control. The
//!   getter returns its default on a miss and every site passes `0`. **[L]**
//! * `user+0x544a` is set to `1` by `0x142883687 mov qword [rsi+0x5448], 0x10001` - an
//!   immediate, not a computed value. **[L]**
//!
//! The one lever that existed, the client's own `/hitdamagetest 0` console command over
//! `0x00EA`, was **refused by its permission gate**: the command was echoed into chat and no
//! `0x0189` ever came back, which is the only proof it ran.
//!
//! So the server can compute the right number - `world::damage::incoming_damage` runs the real
//! formula over the mob's `PADamage` and the player's defence, and that is the number the HP
//! bar already moves by - but it cannot stop the client drawing its own wrong one. Hence a
//! patch.
//!
//! # What it patches, and why this one call and not the flag
//!
//! `0x1428aca14` is the **only** `call 0x142771360` in `FUN_1428aa0a0`, a 14 497-byte
//! function: grepped over the full listing in `research/msexe-1428aa0a0.txt`, and
//! `tools/callers.py` agrees. Five bytes, `e8 47 49 ec ff`, verified against the on-disk
//! image. Replacing it with `nop`s removes that one draw.
//!
//! **It does not touch our own number.** `research/damage-number-suppress.md` §1: the `0x02D1`
//! queue drain `FUN_14281e390` is one of fifteen renderer callers with **no `0x544a` gate**, so
//! it is not on this path at all. **[L]**
//!
//! Clearing the flag instead would have been tidier and is not available: the only writer that
//! can be reached is the console command, and it is refused.
//!
//! # This is a patch, and it is honest about being one
//!
//! It removes a wrong number rather than making it right. The client's number could only be
//! made right by the server supplying touch damage in a packet, and **which packet that is has
//! not been found** - `research/damage-number-two-numbers.md` §4. Until it is, one correct
//! number beats two disagreeing ones.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::hook::log;

/// `0x1428aca14` - the renderer call inside the block gated on `user+0x544a`.
const HIT_NUMBER_DRAW_RVA: usize = 0x1428ACA14 - 0x140000000;

/// `call 0x142771360` - what must be there. The rel32 was recomputed from the target and
/// matches the image byte for byte, so a different build cannot be patched by accident.
const EXPECT: [u8; 5] = [0xE8, 0x47, 0x49, 0xEC, 0xFF];

/// Five `nop`s. The call takes no arguments off the stack and returns nothing that is read,
/// so removing it needs no other adjustment.
const REPLACE: [u8; 5] = [0x90, 0x90, 0x90, 0x90, 0x90];

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

    let at = crate::hook::base() + HIT_NUMBER_DRAW_RVA;
    if !crate::session::can_read(at, EXPECT.len()) {
        log(&format!(
            "***** HITNUMBER: {at:#x} is not readable - NOT patched. The client will keep \
             drawing its own damage number, which for contact damage is always 1 *****"
        ));
        return;
    }

    let found = std::slice::from_raw_parts(at as *const u8, EXPECT.len()).to_vec();
    if found != EXPECT {
        log(&format!(
            "***** HITNUMBER: refusing to patch {at:#x} - expected {EXPECT:02x?} \
             (call 0x142771360) and found {found:02x?}. Either the RVA is wrong for this build \
             or something else is already there. NOT patched *****"
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
            "***** HITNUMBER: {at:#x} is now {after:02x?} (five nops). The client no longer \
             draws its own hit number. OURS is untouched - the 0x02D1 drain has no 0x544a gate \
             - so exactly one number should appear, and it should be the one the HP bar moves \
             by. If NO number appears, ours is not drawing and this patch is not the reason *****"
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

    /// The rel32 in [`EXPECT`] is recomputed from the call's own target rather than copied,
    /// so a transcription error in the constant fails here rather than on the owner's machine.
    ///
    /// A byte patch written from a mistyped constant is the exact shape of instrument this
    /// project keeps paying for: it would refuse, log "found ...", and read as "the patch
    /// does not work on this build".
    #[test]
    fn the_expected_bytes_are_a_call_to_the_renderer() {
        const SITE: i64 = 0x1428ACA14;
        const TARGET: i64 = 0x142771360;
        let rel = (TARGET - (SITE + 5)) as i32;
        let mut want = vec![0xE8u8];
        want.extend_from_slice(&rel.to_le_bytes());
        assert_eq!(EXPECT.to_vec(), want, "EXPECT must be call {TARGET:#x} from {SITE:#x}");
    }

    /// Same length in and out, or the instruction after the call moves.
    #[test]
    fn the_patch_is_length_preserving_and_all_nops() {
        assert_eq!(EXPECT.len(), REPLACE.len());
        assert!(REPLACE.iter().all(|b| *b == 0x90), "anything else changes what executes");
    }

    /// The RVA must be inside the image, and it is derived from the VA rather than typed.
    #[test]
    fn the_rva_is_the_va_less_the_image_base() {
        assert_eq!(HIT_NUMBER_DRAW_RVA, 0x1428ACA14 - 0x140000000);
        assert_eq!(HIT_NUMBER_DRAW_RVA, 0x28ACA14);
    }
}
