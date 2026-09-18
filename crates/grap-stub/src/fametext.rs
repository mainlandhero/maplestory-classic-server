//! The fame messages, without the doubled apostrophes. Four strings rewritten in place.
//!
//! The owner, 2026-09-18, with a screenshot of `'the owner' has raised 'Tester2''s level of fame.` and
//! `You have raised 'the owner''s level of fame.`: *"Too many apostrophes."* The server sends only
//! the names; the quotes and the possessive are the client's own templates, ids `0x00FA`,
//! `0x00FB`, `0x0103`, `0x0104` in the encrypted numeric string table
//! (`tools/dump_stringids.py`), each written as `'%s''s`. Same mechanism as `beautytext`: the
//! bytes are rewritten in place, same length, padded with trailing spaces the chat line does
//! not show.
//!
//! The bytes were produced with the entry's own key (`derive_key` on the seed byte each entry
//! starts with) and checked by decrypting them back. **Not** by XOR-ing the plaintext
//! difference onto the old bytes: that trick breaks wherever the original byte decrypted
//! through the table's NUL quirk (cipher == key, kept as the key byte), and `0x00FA` has one
//! such byte at offset 25. One wording per id was chosen so that no new byte is zero, which
//! would end the C string early - `0x0103` needed its second candidate.
//!
//! On by default; `fametext=off` in the session marker leaves the client untouched.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::hook::log;

/// One rewritten template.
pub struct Patch {
    /// The numeric string id, for the log.
    pub id: u16,
    /// Where the encrypted text starts: the entry's VA plus one (past the seed byte).
    pub text_va: usize,
    /// What it decrypts to today.
    pub old: &'static str,
    /// What it decrypts to after the patch, padded to the same length.
    pub new: &'static str,
    /// The bytes as they sit in the image.
    pub expect: &'static [u8],
    /// The bytes that decrypt to `new` under the entry's key.
    pub replace: &'static [u8],
}

pub const PATCHES: &[Patch] = &[
    Patch {
        id: 0x00FA,
        text_va: 0x1432dddf9,
        old: "You have raised '%s''s level of fame.",
        new: "You have raised %s's level of fame.  ",
        expect: &[0x42, 0xE0, 0x32, 0x13, 0xB6, 0xF8, 0xEF, 0xE2, 0x19, 0x04, 0xD7, 0x9A, 0xDF, 0x57, 0x57, 0x05, 0x3C, 0xAA, 0x34, 0x14, 0xF9, 0xEA, 0xB9, 0xEB, 0x5C, 0x76, 0xD3, 0x9F, 0x8C, 0x5D, 0x55, 0x05, 0x7D, 0xEE, 0x2A, 0x56, 0xF0],
        replace: &[0x42, 0xE0, 0x32, 0x13, 0xB6, 0xF8, 0xEF, 0xE2, 0x19, 0x04, 0xD7, 0x9A, 0xDF, 0x57, 0x57, 0x05, 0x3E, 0xFC, 0x60, 0x40, 0xFE, 0xF5, 0xFC, 0xF1, 0x5C, 0x1A, 0x96, 0x9C, 0xCA, 0x12, 0x55, 0x44, 0x76, 0xEA, 0x69, 0x13, 0xFE],
    },
    Patch {
        id: 0x00FB,
        text_va: 0x1432dde21,
        old: "You have dropped '%s''s level of fame.",
        new: "You have dropped %s's level of fame.  ",
        expect: &[0x6E, 0x71, 0xFB, 0x47, 0xD5, 0x52, 0x45, 0x6B, 0x52, 0x89, 0x1F, 0x88, 0x28, 0x14, 0x03, 0x2E, 0x17, 0x39, 0xAB, 0x14, 0x9A, 0x14, 0x40, 0x2E, 0x1E, 0x88, 0x1B, 0x82, 0x34, 0x44, 0x09, 0x2C, 0x17, 0x78, 0xEF, 0x0A, 0xD8, 0x1D],
        replace: &[0x6E, 0x71, 0xFB, 0x47, 0xD5, 0x52, 0x45, 0x6B, 0x52, 0x89, 0x1F, 0x88, 0x28, 0x14, 0x03, 0x2E, 0x17, 0x3B, 0xFD, 0x40, 0xCE, 0x13, 0x5F, 0x6B, 0x04, 0x88, 0x01, 0xC7, 0x37, 0x02, 0x46, 0x2C, 0x56, 0x73, 0xEB, 0x49, 0x9D, 0x13],
    },
    Patch {
        id: 0x0103,
        text_va: 0x1432de029,
        old: "'%s' has raised '%s''s level of fame.",
        new: "%s raised %s's level of fame.        ",
        expect: &[0x39, 0xAB, 0x14, 0x9A, 0x13, 0x5B, 0x6F, 0x01, 0xCD, 0x1F, 0x86, 0x31, 0x17, 0x03, 0x2E, 0x17, 0x39, 0xAB, 0x14, 0x9A, 0x14, 0x40, 0x2E, 0x1E, 0x88, 0x1B, 0x82, 0x34, 0x44, 0x09, 0x2C, 0x17, 0x78, 0xEF, 0x0A, 0xD8, 0x1D],
        replace: &[0x3B, 0xFD, 0x47, 0xCF, 0x52, 0x5A, 0x7D, 0x17, 0x89, 0x4D, 0xC2, 0x2B, 0x43, 0x15, 0x6A, 0x5B, 0x7B, 0xF8, 0x02, 0xD1, 0x13, 0x5C, 0x68, 0x52, 0x8B, 0x0C, 0x8A, 0x3D, 0x4A, 0x46, 0x6A, 0x17, 0x3E, 0xAE, 0x47, 0x9D, 0x13],
    },
    Patch {
        id: 0x0104,
        text_va: 0x1432de051,
        old: "'%s' has dropped '%s''s level of fame.",
        new: "%s has dropped %s's level of fame.    ",
        expect: &[0x1A, 0x39, 0xBC, 0x5D, 0x46, 0x0E, 0x7D, 0x96, 0xFA, 0xBF, 0xBC, 0xDF, 0xB8, 0xBC, 0xF1, 0x0A, 0x1D, 0x3B, 0xEA, 0x09, 0x41, 0x41, 0x6F, 0xC5, 0xB6, 0xBE, 0xB8, 0xD5, 0xA4, 0xEC, 0xFB, 0x08, 0x1D, 0x7A, 0xAE, 0x17, 0x03, 0x48],
        replace: &[0x18, 0x6F, 0xEF, 0x12, 0x07, 0x15, 0x3C, 0x81, 0xA8, 0xB4, 0xBE, 0xC0, 0xAD, 0xA8, 0xB4, 0x4B, 0x4E, 0x3B, 0xBC, 0x5A, 0x0A, 0x03, 0x6A, 0x80, 0xB6, 0xFB, 0xA1, 0xD6, 0xE8, 0xAA, 0xF5, 0x03, 0x58, 0x32, 0xEF, 0x5A, 0x46, 0x46],
    },
];

const PAGE_READWRITE: u32 = 0x04;

extern "system" {
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

static APPLIED: AtomicBool = AtomicBool::new(false);

/// Apply the four patches unless `fametext=off` is in the marker. Idempotent; logs once per
/// string. Read-before-write, then read-back, the same discipline as `beautytext`.
pub unsafe fn install() {
    if APPLIED.swap(true, Ordering::SeqCst) {
        return;
    }
    if crate::session::marker_token("fametext=").as_deref() == Some("off") {
        log("FAMETEXT: fametext=off in the marker - the fame messages keep their doubled apostrophes");
        return;
    }
    for p in PATCHES {
        let at = crate::hook::base() + (p.text_va - 0x140000000);
        if !crate::session::can_read(at, p.expect.len()) {
            log(&format!("***** FAMETEXT: string {:#06x} at {at:#x} is not readable - NOT patched *****", p.id));
            continue;
        }
        let found = std::slice::from_raw_parts(at as *const u8, p.expect.len()).to_vec();
        if found == p.replace {
            log(&format!("FAMETEXT: string {:#06x} already holds the patch; nothing to do", p.id));
            continue;
        }
        if found != p.expect {
            log(&format!(
                "***** FAMETEXT: refusing to patch string {:#06x} at {at:#x} - the bytes are not the ones this build was read from. NOT patched *****",
                p.id
            ));
            continue;
        }
        let mut old = 0u32;
        if VirtualProtect(at as *mut c_void, p.replace.len(), PAGE_READWRITE, &mut old) == 0 {
            log(&format!("***** FAMETEXT: VirtualProtect failed for string {:#06x} - NOT patched *****", p.id));
            continue;
        }
        std::ptr::copy_nonoverlapping(p.replace.as_ptr(), at as *mut u8, p.replace.len());
        VirtualProtect(at as *mut c_void, p.replace.len(), old, &mut old);
        let after = std::slice::from_raw_parts(at as *const u8, p.replace.len()).to_vec();
        if after == p.replace {
            log(&format!("FAMETEXT: string {:#06x} now reads {:?} (was {:?})", p.id, p.new.trim_end(), p.old));
        } else {
            log(&format!("***** FAMETEXT: wrote string {:#06x} and read back something else; the write did not take *****", p.id));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Same length in and out, no zero byte (a C-string terminator), the format directives
    /// kept in the same number and order, and the four ids are the fame ones.
    #[test]
    fn every_patch_keeps_its_length_its_directives_and_never_writes_a_terminator() {
        assert_eq!(PATCHES.len(), 4);
        let ids: Vec<u16> = PATCHES.iter().map(|p| p.id).collect();
        assert_eq!(ids, vec![0x00FA, 0x00FB, 0x0103, 0x0104]);
        for p in PATCHES {
            assert_eq!(p.expect.len(), p.replace.len(), "{:#06x}: same length", p.id);
            assert_eq!(p.old.len(), p.expect.len(), "{:#06x}: old text is the entry's length", p.id);
            assert_eq!(p.new.len(), p.replace.len(), "{:#06x}: new text padded to the entry's length", p.id);
            assert!(!p.replace.contains(&0), "{:#06x}: a zero byte would end the string", p.id);
            assert_eq!(p.old.matches("%s").count(), p.new.matches("%s").count(), "{:#06x}: the directives", p.id);
            assert!(!p.new.contains("''"), "{:#06x}: no doubled apostrophe", p.id);
            assert!(!p.new.contains("'%s'"), "{:#06x}: the name is not quoted", p.id);
            assert!(p.new.contains("%s's"), "{:#06x}: the possessive survives", p.id);
        }
    }
}
