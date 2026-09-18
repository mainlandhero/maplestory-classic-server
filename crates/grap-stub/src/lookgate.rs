//! **The item-info gate that keeps a 4xxxx hair out of every icon and tooltip.** One
//! immediate byte, rewritten in place.
//!
//! The owner, 2026-09-18, two screenshots of another player's Character Info: the classic hairs
//! and faces draw in the ITEM tab, Fern Face draws, and *"Fern hair continues to be blank"*
//! - no cell icon, a white tooltip preview. Then, to the proposal of renumbering the
//! backported hairs: *"Instead of potentially changing the hair Ids, can we just make changes
//! to the display logic instead?"*
//!
//! The display logic is `CItemInfo`'s node getter, `FUN_14039e630`
//! (`research/msexe-iteminfo-getitem.c`). Its first decision is which tree an id lives in:
//!
//! ```text
//! 14039e662  lea  eax,[r8-0xf4240] ; cmp eax,0xf4240 ; jb  EQUIP    id in 1000000..1999999
//! 14039e674  imul (id / 10000)
//! 14039e688  cmp  ebx,3                                                   <- this byte
//! 14039e68b  jle  EQUIP                                                   id / 10000 <= 3
//! ...                                                                     else: the Item trees
//! ```
//!
//! Classic faces are `2xxxx` and classic hairs `3xxxx`, so `3` was every look id this
//! client shipped with. The backported hairs are `42540..42607`, `42570 / 10000 = 4`, and
//! the else branch looks them up under `Item/` where nothing is - no node, no `info/icon`,
//! no tooltip image. The avatar renderer and the name lookup take other paths, which is why
//! the hair draws on the character and is named in the list. **The path builder the EQUIP
//! branch then calls already knows 4xxxx**: `FUN_1403e18a0` maps `id / 10000` of 3, 4 and 6
//! to `Character/Hair/%08d.img` and 2 and 5 to `Character/Face/%08d.img` **[L]**. So the
//! gate is the only thing behind the classic limit, and `6` is the number the path builder
//! itself stops at. **[L]** for the gate and the builder; **[I]** that nothing else in the
//! equip branch rejects an id above 39999.
//!
//! Same discipline as `beautytext` and `hitnumber`: read-before-write against the exact bytes
//! expected (`83 fb 03 0f 8e`, the cmp and the start of the jle), then read back. On by
//! default; `lookgate=off` in the session marker leaves the client untouched.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::hook::log;

/// `cmp ebx, 3` at `0x14039e688`, less the image base. The immediate is the third byte.
pub const GATE_RVA: usize = 0x14039e688 - 0x140000000;

/// `cmp ebx, 3 ; jle rel32` - the five bytes the site must hold before it is touched.
pub const EXPECT: [u8; 5] = [0x83, 0xfb, 0x03, 0x0f, 0x8e];

/// The same with `cmp ebx, 6`: every `id / 10000` the path builder maps to Hair or Face.
pub const REPLACE: [u8; 5] = [0x83, 0xfb, 0x06, 0x0f, 0x8e];

const PAGE_EXECUTE_READWRITE: u32 = 0x40;

extern "system" {
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

static APPLIED: AtomicBool = AtomicBool::new(false);

/// Apply the patch unless `lookgate=off` is in the marker. Idempotent; logs once either way.
pub unsafe fn install() {
    if APPLIED.swap(true, Ordering::SeqCst) {
        return;
    }
    if crate::session::marker_token("lookgate=").as_deref() == Some("off") {
        log("LOOKGATE: lookgate=off in the marker - a 4xxxx hair keeps no icon and no tooltip image");
        return;
    }

    let at = crate::hook::base() + GATE_RVA;
    if !crate::session::can_read(at, EXPECT.len()) {
        log(&format!("***** LOOKGATE: {at:#x} is not readable - NOT patched *****"));
        return;
    }
    let found = std::slice::from_raw_parts(at as *const u8, EXPECT.len()).to_vec();
    if found == REPLACE {
        log(&format!("LOOKGATE: {at:#x} already holds the patch; nothing to do"));
        return;
    }
    if found != EXPECT {
        log(&format!(
            "***** LOOKGATE: refusing to patch {at:#x} - expected {EXPECT:02x?} (cmp ebx,3 ; jle) \
             and found {found:02x?}. The item-info getter moved in this build. NOT patched *****"
        ));
        return;
    }

    let mut old = 0u32;
    if VirtualProtect(at as *mut c_void, REPLACE.len(), PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        log("***** LOOKGATE: VirtualProtect failed - NOT patched *****");
        return;
    }
    std::ptr::copy_nonoverlapping(REPLACE.as_ptr(), at as *mut u8, REPLACE.len());
    VirtualProtect(at as *mut c_void, REPLACE.len(), old, &mut old);

    let after = std::slice::from_raw_parts(at as *const u8, REPLACE.len()).to_vec();
    if after == REPLACE {
        log(&format!(
            "LOOKGATE: patched {at:#x} - CItemInfo's id gate is `id / 10000 <= 6` instead of \
             `<= 3`, so a backported hair (42540..42607) resolves under Character/Hair like a \
             classic one: its info/icon draws in lists and its tooltip has an image"
        ));
    } else {
        log(&format!(
            "***** LOOKGATE: wrote the patch at {at:#x} and read back {after:02x?}, which is not \
             {REPLACE:02x?}. The write did not take *****"
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RVA -> file offset through the section table (the same helper `session.rs`'s tests
    /// carry; the on-disk image is not laid out like the mapped one).
    fn file_offset(bytes: &[u8], rva: usize) -> usize {
        let u16_at = |o: usize| u16::from_le_bytes([bytes[o], bytes[o + 1]]) as usize;
        let u32_at = |o: usize| u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap()) as usize;
        let pe = u32_at(0x3c);
        let nsec = u16_at(pe + 6);
        let first = pe + 24 + u16_at(pe + 20);
        for i in 0..nsec {
            let s = first + i * 40;
            let (vsize, va, rsize, raw) = (u32_at(s + 8), u32_at(s + 12), u32_at(s + 16), u32_at(s + 20));
            if va <= rva && rva < va + vsize.max(rsize) {
                return raw + (rva - va);
            }
        }
        panic!("rva {rva:#x} is in no section");
    }

    /// The five bytes at the gate in the shipped image are exactly what `install` insists on,
    /// read from `client-patched/MapleStory.exe` - so a build that moves the getter fails
    /// here rather than logging a refusal on the owner's machine.
    #[test]
    fn the_gate_bytes_are_what_the_image_holds() {
        let exe = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../client-patched/MapleStory.exe");
        let Ok(bytes) = std::fs::read(&exe) else {
            eprintln!("skipped: {} is not present", exe.display());
            return;
        };
        let off = file_offset(&bytes, GATE_RVA);
        assert_eq!(&bytes[off..off + EXPECT.len()], &EXPECT, "cmp ebx,3 ; jle at {:#x}", GATE_RVA + 0x140000000);
        assert_ne!(EXPECT, REPLACE);
        assert_eq!(REPLACE[2], 6, "the path builder's last Hair case");
    }
}
