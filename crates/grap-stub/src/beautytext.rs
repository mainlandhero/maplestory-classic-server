//! The Beauty Coupon dialog draws the item's name in white on a white panel. Six bytes fix it.
//!
//! The owner, 2026-09-11, with two screenshots: *"In the Beauty Coupon use dialogue, I do not seem
//! to be able to see Übel Hair Coupon because it's completely white"* - and then, on the
//! plain-ASCII control, *"Frieren's hair also show up as white."* So it is not the backport's
//! string encoding; it is the client.
//!
//! # The string is the client's own, and the colour is written into it
//!
//! The sentence is not in `String.wz` and not in the exe as plain text, because the client's
//! numeric string table is XOR-encrypted (`tools/dump_stringids.py`). Decrypted, id `0x0464`
//! reads:
//!
//! ```text
//! Would you like to use #fc0xffffffff#%s?
//! ```
//!
//! `#fc` is the text engine's font-colour code and `0xffffffff` is opaque white - right for
//! the modern client's dark panel, invisible on this client's `UtilDlgEx_Beauty` panel, which
//! is white. The name is white for every item, which is exactly what the two screenshots
//! show. **[L]**: the id is fetched twice in `FUN_142dc8100` (`mov edx, 0x464` at
//! `0x142dc82bd` and `0x142dc8356`), the only two such sites in the image.
//!
//! # The patch
//!
//! `ffffffff` -> `ff000000`: opaque black, the colour the rest of the sentence draws in. Same
//! length, so the string is rewritten in place. The table is encrypted with a per-string
//! repeating key, so the bytes on disk are not the ASCII; they were computed with the same
//! key schedule the dump tool uses (`derive_key` on seed 157, the entry's first byte) and
//! checked by decrypting them back. Six bytes change - the six hex digits that differ - at
//! `0x1432e8f2e`, which is string offset 29 of the entry at `0x1432e8f10`.
//!
//! A property that needs no key to check, and the test pins it: for every changed byte,
//! `new ^ old == b'0' ^ b'f' == 0x56`, because XOR encryption preserves the plaintext
//! difference. None of the new bytes is zero, so the C string keeps its length.
//!
//! # When it is applied, and the one way it can silently not matter
//!
//! At hook install, the same window as `hitnumber`. The client decrypts a string when it is
//! fetched, and this dialog fetches long after the hook is in, so the patched bytes are what
//! it decrypts. **[I]** that no eager decrypt-and-cache of the whole table runs before the
//! hook: the read-back below proves the bytes were written, not that they were read. If the
//! name is still white with the hook's "BEAUTYTEXT: patched" line in the log, that is the
//! reading, and the fallback is the same six bytes written into `client-patched/MapleStory.exe`
//! by the launcher instead.
//!
//! On by default; `beautytext=off` in the session marker leaves the client untouched.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::hook::log;

/// Where the six bytes live: `0x1432e8f2e`, less the image base.
pub const BEAUTY_NAME_COLOUR_RVA: usize = 0x1432e8f2e - 0x140000000;

/// The entry for string id `0x0464`, for the record: seed byte, then the encrypted text.
pub const STRING_ENTRY_VA: usize = 0x1432e8f10;

/// The encrypted `ffffff` - the last six hex digits of `0xffffffff` - as they sit in the image.
pub const EXPECT: [u8; 6] = [0xbd, 0xa8, 0xd6, 0xae, 0xaa, 0xf2];

/// The same six positions encrypting `000000`, which makes the colour `0xff000000`: black.
pub const REPLACE: [u8; 6] = [0xeb, 0xfe, 0x80, 0xf8, 0xfc, 0xa4];

const PAGE_READWRITE: u32 = 0x04;

extern "system" {
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

static APPLIED: AtomicBool = AtomicBool::new(false);

/// Apply the patch unless `beautytext=off` is in the marker. Idempotent; logs once either way.
///
/// Read-before-write, then read-back, the same discipline as `hitnumber`: a write that did not
/// take and a patch that did not help look identical on screen.
pub unsafe fn install() {
    if APPLIED.swap(true, Ordering::SeqCst) {
        return;
    }
    if crate::session::marker_token("beautytext=").as_deref() == Some("off") {
        log("BEAUTYTEXT: beautytext=off in the marker - the Beauty Coupon dialog keeps its white item name");
        return;
    }

    let at = crate::hook::base() + BEAUTY_NAME_COLOUR_RVA;
    if !crate::session::can_read(at, EXPECT.len()) {
        log(&format!(
            "***** BEAUTYTEXT: {at:#x} is not readable - NOT patched. The Beauty Coupon dialog \
             will keep drawing the item name in white *****"
        ));
        return;
    }

    let found = std::slice::from_raw_parts(at as *const u8, EXPECT.len()).to_vec();
    if found == REPLACE {
        log(&format!("BEAUTYTEXT: {at:#x} already holds the patch; nothing to do"));
        return;
    }
    if found != EXPECT {
        log(&format!(
            "***** BEAUTYTEXT: refusing to patch {at:#x} - expected {EXPECT:02x?} (the encrypted \
             'ffffff' of string 0x0464) and found {found:02x?}. Either the string table moved \
             in this build or the base key differs. NOT patched *****"
        ));
        return;
    }

    let mut old = 0u32;
    if VirtualProtect(at as *mut c_void, REPLACE.len(), PAGE_READWRITE, &mut old) == 0 {
        log("***** BEAUTYTEXT: VirtualProtect failed - NOT patched *****");
        return;
    }
    std::ptr::copy_nonoverlapping(REPLACE.as_ptr(), at as *mut u8, REPLACE.len());
    VirtualProtect(at as *mut c_void, REPLACE.len(), old, &mut old);

    let after = std::slice::from_raw_parts(at as *const u8, REPLACE.len()).to_vec();
    if after == REPLACE {
        log(&format!(
            "BEAUTYTEXT: patched {at:#x} - string 0x0464 now reads 'Would you like to use \
             #fc0xff000000#%s?', so the Beauty Coupon dialog draws the item name in BLACK. If \
             it is still white on screen, the client decrypted the table before this hook ran; \
             the fallback is the same six bytes in the exe file"
        ));
    } else {
        log(&format!(
            "***** BEAUTYTEXT: wrote the patch at {at:#x} and read back {after:02x?}, which is \
             not {REPLACE:02x?}. The write did not take; the name stays white *****"
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **XOR encryption preserves the plaintext difference**, so the patch can be checked
    /// without the key: every changed byte differs from the original by exactly `'f' ^ '0'`.
    /// A wrong offset, a wrong key, or a typo in either array breaks this.
    #[test]
    fn every_byte_moves_by_f_xor_zero_and_none_becomes_a_terminator() {
        assert_eq!(EXPECT.len(), 6, "the six hex digits ffffff -> 000000");
        for (i, (o, n)) in EXPECT.iter().zip(REPLACE.iter()).enumerate() {
            assert_eq!(o ^ n, b'f' ^ b'0', "byte {i}: {o:#04x} -> {n:#04x}");
            assert_ne!(*n, 0, "byte {i} would terminate the C string early");
        }
    }

    /// The six bytes sit at string offset 29 of the entry - `#fc0xff` is 27 characters into
    /// `Would you like to use #fc0xff`, plus the seed byte at offset 0 of the entry.
    #[test]
    fn the_patch_sits_where_the_hex_digits_are() {
        let text = b"Would you like to use #fc0xffffffff#%s?";
        let hex_start = text.iter().position(|&b| b == b'#').unwrap() + "#fc0x".len();
        let changed_start = hex_start + 2; // the leading "ff" (alpha) is kept
        assert_eq!(&text[changed_start..changed_start + 6], b"ffffff");
        assert_eq!(STRING_ENTRY_VA + 1 + changed_start, 0x1432e8f2e);
        assert_eq!(BEAUTY_NAME_COLOUR_RVA, 0x032e8f2e);
    }
}
