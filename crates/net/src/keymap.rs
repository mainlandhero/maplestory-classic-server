//! The keyboard layout: `0x0199` in, `0x05F1`/`0x05F2`/`0x05F3` out.
//!
//! The owner, 2026-09-08: *"I have put Slash Blast on the Ctrl key, Power Strike on the Shift key,
//! and Iron Body on the A key. I have saved it, it's now performing this function locally,
//! but I want this to persist."*
//!
//! Full working, both directions, in `research/keyboard-layout-2026-09-08.md`. The three
//! things that decide the shape of this module:
//!
//! # 1. The save is a DELTA. The restore is the FULL table.
//!
//! `FUN_140c93...`'s sibling `FUN_141a0c340` walks 89 slots and encodes only those that
//! differ from a **shadow copy** `0x1bd` bytes later in the same object - `0x1bd = 445 =
//! 89 * 5`, the same table twice **[L]**. `0x05F1` overwrites the live table and then
//! refreshes the shadow from it, so the shadow is *"what the server last told us"* and every
//! `0x0199` is a diff against that.
//!
//! So the server cannot store what it receives and send it back: it must hold **all 89 slots**
//! and apply each delta on top. [`apply`] is that merge.
//!
//! # 2. The gate byte is INVERTED, and getting it wrong blanks a keyboard
//!
//! ```text
//! 1419ffcf4  call 0x1406e8ae0   ; Decode1 -> bool
//! 1419ffcfb  jne  <skip>        ; NON-ZERO -> skip the block, keep what the client has
//! 1419ffcfd  esi = 0x59         ; ZERO -> read all 89 slots
//! ```
//!
//! Zero means *read*. That is the opposite of the natural reading, and it is why
//! [`keymap_init_keep`] exists as its own function rather than as a `bool` argument: a
//! character with nothing stored must be sent the **non-zero** byte. Sending zero with an
//! empty table would unbind all 89 keys on the player's keyboard.
//!
//! # 3. We do not know the client's factory table, so we do not guess it
//!
//! For a character that has never been synced, the shadow the client diffs against is its own
//! factory layout, which is nowhere in this repo. [`CLIENT_DEFAULT_LAYOUT`] is `None` until
//! `tools/keymapdump.py` measures it off a running client, and while it is `None` this module
//! sends [`keymap_init_keep`] and restores nothing. **That is deliberate**: the failure mode of
//! guessing is every key on a player's keyboard moving, and the failure mode of waiting is the
//! feature staying exactly as broken as it is today.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials. A layout arrives on the say-so of whoever holds
//! the connection, like every other packet on it.

use crate::packet::{PacketReader, PacketWriter};

/// Client -> server. The player clicked CONFIRM in KEY BINDINGS, or changed a preset.
pub const CLIENT_KEYMAP_CHANGE: u16 = 0x0199;

/// Server -> client. The full key layout: gate byte, then [`SLOT_COUNT`] slots.
pub const KEYMAP_INIT: u16 = 0x05F1;

/// Server -> client. One `u32` into the client global `0x143AD1068`.
///
/// Paired with [`Change::OptA`]: subtype 1's builder `FUN_141a011c0` **reads that same
/// address**. Two independent code paths in the client agreeing on an address to the byte is
/// what makes this a pairing rather than a guess - `research/keyboard-layout-2026-09-08.md` §6.
pub const KEYMAP_OPT_A: u16 = 0x05F2;

/// Server -> client. One `u32` into the client global `0x143AD106C`. Pairs with
/// [`Change::OptB`], same control.
pub const KEYMAP_OPT_B: u16 = 0x05F3;

/// Slots in the table. `cmp esi, 0x59` in both the encoder and the decoder **[L]**.
pub const SLOT_COUNT: usize = 89;

/// The gate byte that makes the client READ the block. See the module docs - this is zero.
const GATE_READ: u8 = 0;

/// The gate byte that makes the client SKIP the block and keep what it has.
const GATE_KEEP: u8 = 1;

/// One key's binding. The index into the table is the **DirectInput scan code**, not a virtual
/// key: `0x1D` is LCtrl, `0x2A` is LShift, `0x1E` is A.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Slot {
    /// What kind of thing is bound. `1` is a skill - the only value observed so far, from the
    /// three skills the owner bound. Other values are unknown because nothing else has been bound.
    pub kind: u8,
    /// The skill id, menu id, or item id, depending on `kind`.
    pub action: u32,
}

impl Slot {
    /// An unbound slot. `FUN_1401de960` is the client's own clear-to-zero and writes exactly
    /// this: `mov byte [rcx],0 ; mov dword [rcx+1],0` **[L]**.
    pub const EMPTY: Slot = Slot { kind: 0, action: 0 };

    pub fn is_empty(self) -> bool {
        self.kind == 0 && self.action == 0
    }
}

/// The client's factory layout - preset 0 of the three const tables in the image.
///
/// The owner, 2026-09-12: *"Saving keyboard layout still does not work. I tried putting both Power
/// Strike on control and Slash Blast on shift. It did not survive a re-login."* The save had
/// worked (three rows in `character_keymap`); this was `None`, so [`restore`] sent nothing.
///
/// Measured from the file rather than a process, and the file said more than the process
/// would have: `0x143274460` is in **`.rdata`, read-only** (characteristics `0x40000040`), so
/// it cannot be a live table the dialog rewrites - it is a CONST table, and `0x1bd` is not a
/// shadow offset but the stride of a preset array: three 89-slot layouts back to back, 41
/// slots bound each, kinds 4/5/6 only, and only preset 0 has Q, W, E and I on menus - the
/// layout the KEY BINDINGS dialog shows. The live and shadow tables are initialised from it,
/// which the one CONFIRM delta seen agrees with (LCtrl was basic 52 there). **[L]** for the
/// bytes; `tools/keymapdump.py --exe` re-derives them with the shape and known-key controls.
// Read from client-patched/MapleStory.exe by `tools/keymapdump.py --exe --rust`:
// preset 0 of the three const layouts at 0x143274460 (.rdata, read-only), the one
// with Q, W, E and I on menus - the factory layout the KEY BINDINGS dialog shows.
// 41 of 89 slots bound. [L]
pub const CLIENT_DEFAULT_LAYOUT: Option<[Slot; 89]> = Some([
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 46 },  // Esc
    Slot { kind: 4, action: 10 },  // 1
    Slot { kind: 4, action: 12 },  // 2
    Slot { kind: 4, action: 13 },  // 3
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 8 },  // Q
    Slot { kind: 4, action: 5 },  // W
    Slot { kind: 4, action: 0 },  // E
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 1 },  // I
    Slot { kind: 4, action: 4 },  // O
    Slot { kind: 4, action: 19 },  // P
    Slot { kind: 4, action: 6 },  // [
    Slot { kind: 4, action: 15 },  // ]
    Slot { kind: 0, action: 0 },
    Slot { kind: 5, action: 52 },  // LCtrl
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 2 },  // S
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 11 },  // H
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 3 },  // K
    Slot { kind: 4, action: 55 },  // L
    Slot { kind: 4, action: 58 },  // ;
    Slot { kind: 4, action: 16 },  // '
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 9 },
    Slot { kind: 5, action: 50 },  // Z
    Slot { kind: 5, action: 51 },  // X
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 7 },  // M
    Slot { kind: 4, action: 45 },  // ,
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 5, action: 53 },  // LAlt
    Slot { kind: 5, action: 54 },  // Space
    Slot { kind: 0, action: 0 },
    Slot { kind: 6, action: 100 },  // F1
    Slot { kind: 6, action: 101 },  // F2
    Slot { kind: 6, action: 102 },  // F3
    Slot { kind: 6, action: 103 },  // F4
    Slot { kind: 6, action: 104 },  // F5
    Slot { kind: 6, action: 105 },  // F6
    Slot { kind: 6, action: 106 },  // F7
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 22 },  // F9
    Slot { kind: 4, action: 14 },  // F10
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 47 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 5, action: 302 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 5, action: 300 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 5, action: 301 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 5, action: 303 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 48 },
    Slot { kind: 0, action: 0 },
]);

/// One binding out of a `0x0199` subtype 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Binding {
    /// DirectInput scan code, `0 .. SLOT_COUNT`.
    pub key: u8,
    pub slot: Slot,
}

/// What a `0x0199` asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    /// Subtype 0: key bindings that differ from the client's shadow table.
    Bindings(Vec<Binding>),
    /// Subtype 1: one `u32`, restored by [`KEYMAP_OPT_A`].
    OptA(u32),
    /// Subtype 2: one `u32`, restored by [`KEYMAP_OPT_B`].
    OptB(u32),
    /// Subtype 3: the selected preset, bounded `< 4` by `cmp ebx,4 / jae` at `0x141a01871`.
    ///
    /// **No inbound opcode was found for this**, so a stored preset cannot currently be
    /// restored. Recorded rather than guessed - `research/keyboard-layout-2026-09-08.md` §6.
    Preset(u8),
}

/// Parse a `0x0199` body - everything after the two opcode bytes.
///
/// Returns `None` on a body that does not decode, which includes a subtype we have never
/// seen. **A `None` here must not become silence on the wire**: `0x0199` has not been observed
/// to latch, but the caller decides that, not this function.
pub fn parse_change(body: &[u8]) -> Option<Change> {
    let mut c = PacketReader::new(body);
    let subtype = c.u8().ok()?;
    match subtype {
        0 => {
            // The two `0xFFFFFFFF` words are read and discarded: every capture has had both
            // set to -1 and nothing in the client's builder computes them from the layout.
            // They are preserved in the research file rather than invented here.
            let _flag = c.u8().ok()?;
            let _a = c.u32().ok()?;
            let _b = c.u32().ok()?;
            let count = c.u8().ok()?;
            let mut out = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let key = c.u8().ok()?;
                let kind = c.u8().ok()?;
                let action = c.u32().ok()?;
                // A key outside the table would index past 89 slots on the way back out.
                // Refuse the whole packet rather than silently dropping one binding: a
                // partially-applied layout is worse than a rejected one, because the player
                // sees some of their change survive and cannot tell what did not.
                if key as usize >= SLOT_COUNT {
                    return None;
                }
                out.push(Binding { key, slot: Slot { kind, action } });
            }
            Some(Change::Bindings(out))
        }
        1 => Some(Change::OptA(c.u32().ok()?)),
        2 => Some(Change::OptB(c.u32().ok()?)),
        3 => {
            let preset = c.u8().ok()?;
            if preset >= 4 {
                return None;
            }
            Some(Change::Preset(preset))
        }
        _ => None,
    }
}

/// Merge a delta into a full layout. This is the whole reason the server stores 89 slots.
pub fn apply(layout: &mut [Slot; SLOT_COUNT], bindings: &[Binding]) {
    for b in bindings {
        layout[b.key as usize] = b.slot;
    }
}

/// `0x05F1` carrying the full layout. Gate byte zero, then 89 slots.
pub fn keymap_init(layout: &[Slot; SLOT_COUNT]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(GATE_READ);
    for slot in layout.iter() {
        w.u8(slot.kind);
        w.u32(slot.action);
    }
    w.into_vec()
}

/// `0x05F1` that tells the client to keep the layout it already has.
///
/// The gate byte is **non-zero**, which is the skip. Sent to a character with nothing stored,
/// and whenever [`CLIENT_DEFAULT_LAYOUT`] has not been measured.
pub fn keymap_init_keep() -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(GATE_KEEP);
    w.into_vec()
}

/// `0x05F2` or `0x05F3`: one `u32`.
pub fn keymap_opt(value: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(value);
    w.into_vec()
}

/// The `0x05F1` body to send at login, or `None` if there is nothing safe to send.
///
/// `None` in either of two cases, and in both the caller must send **no packet at all**:
///
/// * the character has saved nothing, so there is nothing to restore;
/// * [`CLIENT_DEFAULT_LAYOUT`] has not been measured, so the slots the player never touched
///   would go out as zeros and unbind their keyboard.
///
/// # Why `None` rather than [`keymap_init_keep`]
///
/// The keep form is a real packet and its no-op path - `0x1419ffd21` onward, past the `jne`
/// that skips the loop - **has not been read**. Today the server sends nothing at all and the
/// client is demonstrably fine, so returning `None` preserves a measured-good behaviour
/// instead of trading it for an unmeasured one to save a branch. `keymap_init_keep` is kept
/// because it is part of the protocol and the gate byte it carries is the thing most likely
/// to be got backwards; nothing sends it yet.
pub fn restore(stored: Option<&[Binding]>) -> Option<Vec<u8>> {
    let (Some(bindings), Some(mut layout)) = (stored, CLIENT_DEFAULT_LAYOUT) else {
        return None;
    };
    if bindings.is_empty() {
        return None;
    }
    apply(&mut layout, bindings);
    Some(keymap_init(&layout))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The capture from the owner's live client, byte for byte, and the three bindings they named
    /// before the packet was opened. `research/keyboard-layout-2026-09-08.md` §1.
    const WISP_CONFIRM: &[u8] = &[
        0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x1d, 0x01, 0x2a,
        0x46, 0x0f, 0x00, 0x1e, 0x01, 0x28, 0x46, 0x0f, 0x00, 0x2a, 0x01, 0x29, 0x46, 0x0f,
        0x00,
    ];

    #[test]
    fn the_live_capture_decodes_to_the_three_skills_wisp_named() {
        assert_eq!(WISP_CONFIRM.len(), 29, "the captured body was 29 bytes");
        let Some(Change::Bindings(b)) = parse_change(WISP_CONFIRM) else {
            panic!("subtype 0 did not decode");
        };
        assert_eq!(b.len(), 3);
        // 0x1D LCtrl -> Slash Blast, 0x1E A -> Iron Body, 0x2A LShift -> Power Strike.
        assert_eq!(b[0], Binding { key: 0x1D, slot: Slot { kind: 1, action: 1_001_002 } });
        assert_eq!(b[1], Binding { key: 0x1E, slot: Slot { kind: 1, action: 1_001_000 } });
        assert_eq!(b[2], Binding { key: 0x2A, slot: Slot { kind: 1, action: 1_001_001 } });
    }

    #[test]
    fn a_truncated_body_is_refused_rather_than_half_applied() {
        for n in 0..WISP_CONFIRM.len() {
            let got = parse_change(&WISP_CONFIRM[..n]);
            assert!(
                !matches!(got, Some(Change::Bindings(ref v)) if v.len() == 3),
                "a {n}-byte prefix must not decode as the full three bindings"
            );
        }
    }

    #[test]
    fn a_key_past_the_table_is_refused_whole() {
        let mut body = WISP_CONFIRM.to_vec();
        body[11] = SLOT_COUNT as u8; // one past the last valid scan code
        assert_eq!(parse_change(&body), None);
    }

    #[test]
    fn the_other_subtypes_decode() {
        assert_eq!(parse_change(&[1, 7, 0, 0, 0]), Some(Change::OptA(7)));
        assert_eq!(parse_change(&[2, 9, 0, 0, 0]), Some(Change::OptB(9)));
        assert_eq!(parse_change(&[3, 2]), Some(Change::Preset(2)));
        // `cmp ebx,4 / jae` - the client will not send 4 and we will not accept it.
        assert_eq!(parse_change(&[3, 4]), None);
    }

    #[test]
    fn the_init_body_is_the_length_the_client_reads() {
        let layout = [Slot::EMPTY; SLOT_COUNT];
        // 1 gate byte + 89 * (u8 + u32).
        assert_eq!(keymap_init(&layout).len(), 1 + SLOT_COUNT * 5);
    }

    /// The gate is inverted and this is the test that says so out loud. If someone ever
    /// "fixes" it to the natural reading, this fails rather than a player's keyboard blanking.
    #[test]
    fn zero_means_read_and_nonzero_means_keep() {
        assert_eq!(keymap_init(&[Slot::EMPTY; SLOT_COUNT])[0], 0);
        assert_ne!(keymap_init_keep()[0], 0);
        assert_eq!(keymap_init_keep().len(), 1, "the keep form carries no table at all");
    }

    #[test]
    fn a_delta_merges_onto_the_layout_and_leaves_the_rest_alone() {
        let mut layout = [Slot::EMPTY; SLOT_COUNT];
        layout[0x10] = Slot { kind: 5, action: 42 };
        let Some(Change::Bindings(b)) = parse_change(WISP_CONFIRM) else { unreachable!() };
        apply(&mut layout, &b);
        assert_eq!(layout[0x1D], Slot { kind: 1, action: 1_001_002 });
        assert_eq!(layout[0x10], Slot { kind: 5, action: 42 }, "an untouched key survives");
        assert_eq!(layout[0x11], Slot::EMPTY);
    }

    /// Until the factory table is measured, restore must NEVER send the read gate - that is
    /// the difference between doing nothing and unbinding every key a player has.
    #[test]
    fn restore_refuses_to_send_a_table_it_cannot_build() {
        let Some(Change::Bindings(b)) = parse_change(WISP_CONFIRM) else { unreachable!() };
        if CLIENT_DEFAULT_LAYOUT.is_none() {
            assert_eq!(restore(Some(&b)), None, "no factory table means send NOTHING");
        }
        assert_eq!(restore(None), None, "never saved means send nothing");
        assert_eq!(restore(Some(&[])), None, "an empty layout means send nothing");
    }

    /// The table is preset 0 of the image, not a guess: 41 bound, menus on Q/W/E/I, attack
    /// on LCtrl - and the owner's CONFIRM merges onto it with everything else untouched.
    #[test]
    fn the_factory_table_is_preset_0_and_a_saved_delta_restores_onto_it() {
        let factory = CLIENT_DEFAULT_LAYOUT.expect("measured 2026-09-12");
        assert_eq!(factory.iter().filter(|s| s.kind != 0 || s.action != 0).count(), 41);
        for (code, (kind, action)) in [(0x10, (4, 8)), (0x11, (4, 5)), (0x12, (4, 0)), (0x17, (4, 1))] {
            assert_eq!(factory[code], Slot { kind, action }, "key {code:#x}");
        }
        assert_eq!(factory[0x1D], Slot { kind: 5, action: 52 }, "LCtrl is the basic attack");
        assert_eq!(factory[0x39], Slot { kind: 5, action: 54 }, "Space is the jump");
        assert!(factory.iter().all(|s| matches!(s.kind, 0 | 4 | 5 | 6)), "no skill or item is factory-bound");

        let Some(Change::Bindings(b)) = parse_change(WISP_CONFIRM) else { unreachable!() };
        let body = restore(Some(&b)).expect("a saved layout now goes out");
        assert_eq!(body[0], 0, "the READ gate");
        assert_eq!(body.len(), 1 + SLOT_COUNT * 5);
        let slot = |code: usize| {
            (body[1 + code * 5], u32::from_le_bytes(body[2 + code * 5..6 + code * 5].try_into().unwrap()))
        };
        assert_eq!(slot(0x1D), (1, 1_001_002), "LCtrl carries the saved skill");
        assert_eq!(slot(0x2A), (1, 1_001_001));
        assert_eq!(slot(0x10), (4, 8), "Q keeps its factory menu");
        assert_eq!(slot(0x39), (5, 54), "Space keeps the jump");
        let bound = (0..SLOT_COUNT).filter(|&c| slot(c) != (0, 0)).count();
        assert_eq!(bound, 41 + 2, "A and LShift were unbound in the factory table; LCtrl was already bound");
    }

    /// Once the factory table IS measured, a stored layout must actually go out, with the
    /// read gate and the player's binding merged on top of the untouched defaults.
    #[test]
    fn restore_sends_the_merged_table_once_the_defaults_exist() {
        // Stands in for CLIENT_DEFAULT_LAYOUT until keymapdump.py fills it in.
        let mut factory = [Slot::EMPTY; SLOT_COUNT];
        factory[0x10] = Slot { kind: 4, action: 99 };
        let Some(Change::Bindings(b)) = parse_change(WISP_CONFIRM) else { unreachable!() };
        apply(&mut factory, &b);
        let body = keymap_init(&factory);
        assert_eq!(body[0], 0, "the read gate");
        assert_eq!(body.len(), 1 + SLOT_COUNT * 5);
        // Q keeps its factory binding; LCtrl carries Slash Blast.
        assert_eq!(body[1 + 0x10 * 5], 4);
        assert_eq!(body[1 + 0x1D * 5], 1);
        assert_eq!(
            u32::from_le_bytes(body[1 + 0x1D * 5 + 1..1 + 0x1D * 5 + 5].try_into().unwrap()),
            1_001_002
        );
    }
}
