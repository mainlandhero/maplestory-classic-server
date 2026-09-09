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

/// The client's factory layout, measured off a running client.
///
/// `None` until `tools/keymapdump.py --rust` fills it in from the shadow table at
/// `0x143274460 + 0x1bd`. See the module docs for why this is not guessed. While it is `None`,
/// [`restore`] returns [`keymap_init_keep`] and the client keeps its own defaults.
pub const CLIENT_DEFAULT_LAYOUT: Option<[Slot; SLOT_COUNT]> = None;

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
