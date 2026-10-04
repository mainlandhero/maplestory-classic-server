//! **The Mix Dye window's colour names fade in and out while the cursor sits on a swatch.**
//! One vtable slot, wrapped.
//!
//! The owner, 2026-10-03, with a screenshot of `UtilDlgEx_MixHair`: *"on hover over a color, the
//! label for the color fades in and out repeatedly really fast, is there a way we can stop this
//! behavior?"* - and, offered the choice, kept the names: *"Keep names, no flicker"*.
//!
//! # What the window does - `research/mix-dye-colorblend.md` §8
//!
//! The palette builder `FUN_142a92890` lays a tooltip LABEL over each colour swatch (same x, y,
//! width and height, the colour's name as its text) and then writes `0` to the label's `+0x11cc`
//! (`142a92cde`). The class default is `1` (`FUN_141aeed80`). **[L]**
//!
//! `+0x11cc` is read by one method only: the label's mouse hit-test, `FUN_14170f740`, vtable
//! slot `0x1433fa330`. **[L]**:
//!
//! ```text
//! inside (FUN_141710510: 0 <= x < +0x48, 0 <= y < +0x4c):
//!     +0x11cc == 0 -> forward the move to the label's own handler (+8 vtable +0x20),
//!                     then answer "not hit" - so the swatch underneath keeps the click
//!     +0x11cc != 0 -> answer "hit" - the label keeps the mouse
//! outside: reset the label's tooltip (+0x78) and forward to the base
//! ```
//!
//! The forwarded move lands in `FUN_14170f560`, which - with no show delay set - calls
//! `FUN_14170f990` -> `FUN_142645c50`, and that **rebuilds the tooltip's whole canvas** every
//! time it is called. **[I], the run's question:** the window hit-tests its controls every
//! frame, not only when the mouse moves, so a pass-through label rebuilds its tooltip - fade
//! and all - every frame while the cursor rests on it. A label that keeps the mouse never
//! takes that branch, which is why no other tooltip in the client does this.
//!
//! # The fix: forward a pass-through label's move only when the cursor actually moved
//!
//! The slot is pointed at [`hit_test`]. For a label with `+0x11cc != 0` - every other label
//! in the client - and for any point outside, it is the original, called as it was. For a
//! pass-through label with the cursor inside at the SAME point as its last forward, it answers
//! "not hit" without forwarding, which is exactly what the original answers there, minus the
//! rebuild. The swatch still gets the click; the name is built once and left to finish its
//! fade.
//!
//! **What would make it not work, and how the log tells.** If the flicker comes from somewhere
//! else - a hover-leave sent every frame, say - the tooltip would now vanish and stay gone
//! until the mouse moves. The counters say which: [`report`] logs how many hit-tests came in,
//! how many were forwarded and how many were held. Thousands held at a still cursor is the
//! per-frame reading confirmed; a handful is the reading refuted.
//!
//! **Under Wine** (the Mac client) this is a pointer write into `.rdata` through
//! `VirtualProtect`, which Wine implements; nothing here leans on a Windows-only facility.
//!
//! On by default; `mixtooltip=off` in the session marker leaves the slot alone.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, AtomicUsize, Ordering};

use crate::hook::log;

/// The label class's vtable slot holding its hit-test, less the image base.
pub const SLOT_RVA: usize = 0x1433fa330 - 0x140000000;
/// `FUN_14170f740`, less the image base - what the slot must hold before it is touched.
pub const HIT_TEST_RVA: usize = 0x14170f740 - 0x140000000;

/// The pass-through flag. `0` = forward and answer "not hit" (the mix palette's labels).
const PASS_THROUGH_FLAG: usize = 0x11cc;
/// The label's width and height, as `FUN_141710510` reads them.
const WIDTH: usize = 0x48;
const HEIGHT: usize = 0x4c;

const PAGE_READWRITE: u32 = 0x04;

extern "system" {
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

type HitTest = unsafe extern "C" fn(*mut u8, i32, i32) -> u64;

static APPLIED: AtomicBool = AtomicBool::new(false);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);

/// The last pass-through label a move was forwarded to, and where.
static LAST_LABEL: AtomicUsize = AtomicUsize::new(0);
static LAST_X: AtomicI32 = AtomicI32::new(i32::MIN);
static LAST_Y: AtomicI32 = AtomicI32::new(i32::MIN);

/// Hit-tests on a pass-through label with the cursor inside; of those, forwarded and held.
static INSIDE: AtomicU64 = AtomicU64::new(0);
static FORWARDED: AtomicU64 = AtomicU64::new(0);
static HELD: AtomicU64 = AtomicU64::new(0);
/// Summary lines written so far - capped, so a long session cannot flood the log.
static REPORTS: AtomicU64 = AtomicU64::new(0);
const MAX_REPORTS: u64 = 12;
/// A summary every this many inside hit-tests.
const REPORT_EVERY: u64 = 600;

/// Whether a hit at `(x, y)` on the label `label` should be forwarded: the first time, and
/// whenever the point or the label changes. Pure, so the rule is a unit test.
pub fn should_forward(last: (usize, i32, i32), label: usize, x: i32, y: i32) -> bool {
    last != (label, x, y)
}

/// The wrapped hit-test. See the module docs.
unsafe extern "C" fn hit_test(this: *mut u8, x: i32, y: i32) -> u64 {
    let original: HitTest = std::mem::transmute::<usize, HitTest>(ORIGINAL.load(Ordering::Relaxed));
    let pass_through = *(this.add(PASS_THROUGH_FLAG) as *const i32) == 0;
    if !pass_through {
        return original(this, x, y);
    }
    let width = *(this.add(WIDTH) as *const i32);
    let height = *(this.add(HEIGHT) as *const i32);
    let inside = x >= 0 && y >= 0 && x < width && y < height;
    let label = this as usize;
    if !inside {
        // Left it: the next time the cursor comes back, the name is shown again.
        if LAST_LABEL.load(Ordering::Relaxed) == label {
            LAST_LABEL.store(0, Ordering::Relaxed);
        }
        return original(this, x, y);
    }
    INSIDE.fetch_add(1, Ordering::Relaxed);
    let last = (LAST_LABEL.load(Ordering::Relaxed), LAST_X.load(Ordering::Relaxed), LAST_Y.load(Ordering::Relaxed));
    if should_forward(last, label, x, y) {
        LAST_LABEL.store(label, Ordering::Relaxed);
        LAST_X.store(x, Ordering::Relaxed);
        LAST_Y.store(y, Ordering::Relaxed);
        FORWARDED.fetch_add(1, Ordering::Relaxed);
        report();
        return original(this, x, y);
    }
    HELD.fetch_add(1, Ordering::Relaxed);
    report();
    // Exactly what the original answers for a pass-through label with the cursor inside.
    0
}

/// A summary line every [`REPORT_EVERY`] inside hit-tests, at most [`MAX_REPORTS`] of them.
fn report() {
    let inside = INSIDE.load(Ordering::Relaxed);
    if inside % REPORT_EVERY != 0 || REPORTS.fetch_add(1, Ordering::Relaxed) >= MAX_REPORTS {
        return;
    }
    log(&format!(
        "MIXTOOLTIP: {inside} hit-tests inside a pass-through label - {} forwarded (the cursor \
         moved), {} held (same point). Many held at a still cursor = the window hit-tests every \
         frame, and each held one is a tooltip rebuild that no longer happens",
        FORWARDED.load(Ordering::Relaxed),
        HELD.load(Ordering::Relaxed)
    ));
}

/// Point the slot at [`hit_test`] unless `mixtooltip=off` is in the marker. Idempotent; logs
/// once either way. Read-before-write and read-back, as `beautytext` and `lookgate` do.
pub unsafe fn install() {
    if APPLIED.swap(true, Ordering::SeqCst) {
        return;
    }
    if crate::session::marker_token("mixtooltip=").as_deref() == Some("off") {
        log("MIXTOOLTIP: mixtooltip=off in the marker - the Mix Dye window's colour names keep flickering");
        return;
    }
    let base = crate::hook::base();
    let slot = base + SLOT_RVA;
    let expect = base + HIT_TEST_RVA;
    if !crate::session::can_read(slot, 8) {
        log(&format!("***** MIXTOOLTIP: {slot:#x} is not readable - NOT patched *****"));
        return;
    }
    let found = *(slot as *const usize);
    if found != expect {
        log(&format!(
            "***** MIXTOOLTIP: refusing to patch {slot:#x} - expected {expect:#x} (FUN_14170f740, \
             the label hit-test) and found {found:#x}. NOT patched *****"
        ));
        return;
    }
    ORIGINAL.store(found, Ordering::SeqCst);
    let mut old = 0u32;
    if VirtualProtect(slot as *mut c_void, 8, PAGE_READWRITE, &mut old) == 0 {
        log("***** MIXTOOLTIP: VirtualProtect failed - NOT patched *****");
        return;
    }
    *(slot as *mut usize) = hit_test as *const () as usize;
    VirtualProtect(slot as *mut c_void, 8, old, &mut old);
    if *(slot as *const usize) == hit_test as *const () as usize {
        log(&format!(
            "MIXTOOLTIP: slot {slot:#x} now wraps the label hit-test {expect:#x}: a pass-through \
             label (the Mix Dye / Colorblend palette's colour names) forwards the mouse only when \
             it moves. Every other label is untouched"
        ));
    } else {
        log(&format!("***** MIXTOOLTIP: wrote {slot:#x} and it did not take - the colour names keep flickering *****"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A still cursor on the same label is held; a moved cursor, another label, or the first
    /// sighting is forwarded.
    #[test]
    fn only_a_move_or_a_new_label_is_forwarded() {
        assert!(should_forward((0, i32::MIN, i32::MIN), 0x1000, 5, 5), "first sighting");
        assert!(!should_forward((0x1000, 5, 5), 0x1000, 5, 5), "the same point: held");
        assert!(should_forward((0x1000, 5, 5), 0x1000, 6, 5), "moved");
        assert!(should_forward((0x1000, 5, 5), 0x2000, 5, 5), "another swatch's label");
    }

    #[test]
    fn the_slot_and_the_function_are_the_listings() {
        assert_eq!(SLOT_RVA, 0x033fa330);
        assert_eq!(HIT_TEST_RVA, 0x0170f740);
    }
}
