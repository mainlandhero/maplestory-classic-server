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
//! # 4. There are FOUR tables, and the fourth is the controller - 2026-09-14
//!
//! The owner: *"Whenever there are customization to keybindings in the controller settings, it is
//! not getting saved properly, and when clients switch maps, their controller settings are
//! completely screwed up."*
//!
//! The byte after the subtype in a subtype-0 `0x0199` - read and thrown away here as "a flag
//! that is always 0" - is **the table the delta belongs to**. The builder `FUN_141a0c340`
//! takes the table pointer in `rcx` and that byte in `dl`; its two callers set them together
//! **[L]**:
//!
//! ```text
//! 141a00849  imul rcx, rbx, 0x37a ; lea rax,[0x143ad0280] ; movzx edx, bl   ; keyboard preset rbx
//! 141a008ef  mov  dl, 3           ; lea rcx,[0x143ad1070]                    ; the controller
//! ```
//!
//! Every keyboard capture (20 of them) had 0 there; both captures from the run where the owner
//! edited the Controller tab had **3**. So every controller binding they saved was merged into
//! the keyboard table, and the `0x05F1` at the next map change did two wrong things at once:
//! it put controller buttons' actions on keyboard scan codes 0..0x27, and it sent preset 3 as
//! *keep* - which is not "leave it alone". The handler **resets all four tables to keyboard
//! preset 0** before it reads a gate (`0x1419ffc73`, `mov rdx, r12` for every iteration), so
//! keep on preset 3 hands the controller a keyboard layout. The client's own controller
//! default is a separate const at `0x143274b20` ([`CLIENT_CONTROLLER_LAYOUT`]), copied into
//! the preset-3 static once at init (`0x1419ff905`) and never again.
//!
//! The 53-entry delta the client sent after that (`world-ch0.log` 2026-09-14 00:58:49) is the
//! proof: it is exactly the controller default XOR keyboard preset 0 - it opens with the
//! controller table's first twelve buttons verbatim and goes on to *unbind* Q, W and E, keys a
//! controller does not have. The client was reporting how far its controller table had been
//! pushed from its shadow.
//!
//! So: [`Change::Bindings`] carries its table, the store keeps one row per (table, key), and
//! [`restore`] sends all four tables READ, each as its own factory plus its own deltas.
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

// Presets 1 and 2, the same way. The 0x05F1 handler reads FOUR gated tables
// (`cmp r15d, 4` at 0x1419ffdb1), one per preset; these two go out as read
// so the dialog's alternatives stay what the client shipped, and preset 3 -
// which has no const table in the image - goes out as keep. [L]
pub const CLIENT_PRESETS_1_AND_2: [[Slot; 89]; 2] = [
    [
        Slot { kind: 0, action: 0 },
        Slot { kind: 4, action: 46 },  // Esc
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
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 4, action: 2 },  // Y
        Slot { kind: 4, action: 0 },  // U
        Slot { kind: 4, action: 1 },  // I
        Slot { kind: 4, action: 4 },  // O
        Slot { kind: 4, action: 19 },  // P
        Slot { kind: 4, action: 6 },  // [
        Slot { kind: 4, action: 15 },  // ]
        Slot { kind: 0, action: 0 },
        Slot { kind: 5, action: 52 },  // LCtrl
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 4, action: 11 },  // H
        Slot { kind: 4, action: 8 },  // J
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
        Slot { kind: 4, action: 5 },  // N
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
        Slot { kind: 4, action: 12 },
        Slot { kind: 5, action: 302 },
        Slot { kind: 4, action: 13 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 5, action: 300 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 5, action: 301 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 5, action: 303 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 4, action: 10 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 4, action: 48 },
        Slot { kind: 0, action: 0 },
    ],
    [
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
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 4, action: 2 },  // Y
        Slot { kind: 4, action: 0 },  // U
        Slot { kind: 4, action: 1 },  // I
        Slot { kind: 4, action: 4 },  // O
        Slot { kind: 4, action: 19 },  // P
        Slot { kind: 4, action: 6 },  // [
        Slot { kind: 4, action: 15 },  // ]
        Slot { kind: 0, action: 0 },
        Slot { kind: 5, action: 52 },  // LCtrl
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 0, action: 0 },
        Slot { kind: 4, action: 11 },  // H
        Slot { kind: 4, action: 8 },  // J
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
        Slot { kind: 4, action: 5 },  // N
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
        Slot { kind: 0, action: 0 },
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
        Slot { kind: 4, action: 47 },
    ],
];

// The CONTROLLER factory table, preset 3, from 0x143274b20 (.rdata) - what the
// keymap init copies into the preset-3 static at 0x143ad1070, and what the 0x05F1
// handler does NOT reset it to (it resets every preset to keyboard preset 0). The
// index is a controller button, not a scan code. 22 of 89 bound. [L]
pub const CLIENT_CONTROLLER_LAYOUT: [Slot; 89] = [
    Slot { kind: 5, action: 53 },  // button 0x0
    Slot { kind: 4, action: 401 },  // button 0x1
    Slot { kind: 4, action: 400 },  // button 0x2
    Slot { kind: 5, action: 52 },  // button 0x3
    Slot { kind: 5, action: 51 },  // button 0x4
    Slot { kind: 5, action: 50 },  // button 0x5
    Slot { kind: 4, action: 0 },  // button 0x6
    Slot { kind: 4, action: 3 },  // button 0x7
    Slot { kind: 4, action: 2 },  // button 0x8
    Slot { kind: 4, action: 1 },  // button 0x9
    Slot { kind: 4, action: 46 },  // button 0xa
    Slot { kind: 4, action: 14 },  // button 0xb
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
    Slot { kind: 0, action: 0 },
    Slot { kind: 5, action: 53 },  // button 0x18
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 5 },  // button 0x1c
    Slot { kind: 4, action: 9 },  // button 0x1d
    Slot { kind: 4, action: 8 },  // button 0x1e
    Slot { kind: 4, action: 55 },  // button 0x1f
    Slot { kind: 5, action: 53 },  // button 0x20
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 4, action: 22 },  // button 0x24
    Slot { kind: 4, action: 19 },  // button 0x25
    Slot { kind: 4, action: 6 },  // button 0x26
    Slot { kind: 4, action: 4 },  // button 0x27
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
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
    Slot { kind: 0, action: 0 },
];

/// The table index the controller's bindings travel under: the fourth of [`PRESET_COUNT`],
/// `mov dl, 3` at `0x141a008ef` **[L]**. Tables 0..2 are the keyboard's three presets.
pub const CONTROLLER_TABLE: u8 = 3;

/// **What the client's table `table` holds before any delta**, and therefore the base every
/// stored delta for that table sits on. `None` for a table index the client does not have,
/// and for table 0 while [`CLIENT_DEFAULT_LAYOUT`] is unmeasured.
///
/// One subtlety, measured 2026-09-14: the `0x05F1` handler resets **every** table to keyboard
/// preset 0 before its gate, including the controller's. That is what the client's shadow
/// becomes once it has received a `0x05F1` with keep on table 3, and it is NOT what this
/// returns for table 3 - this returns the controller's own const, because the shadow a delta
/// is diffed against is whatever the last `0x05F1` put there, and the last `0x05F1` is ours:
/// this table plus the stored deltas. Before any `0x05F1` the shadow is the init copy of this
/// same const. Either way the base is this, provided the server never sends table 3 as keep.
pub fn factory(table: u8) -> Option<[Slot; SLOT_COUNT]> {
    match table {
        0 => CLIENT_DEFAULT_LAYOUT,
        1 => Some(CLIENT_PRESETS_1_AND_2[0]),
        2 => Some(CLIENT_PRESETS_1_AND_2[1]),
        CONTROLLER_TABLE => Some(CLIENT_CONTROLLER_LAYOUT),
        _ => None,
    }
}

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
    /// Subtype 0: key bindings that differ from the client's shadow copy of ONE table -
    /// keyboard preset 0..2, or [`CONTROLLER_TABLE`]. Module docs §4.
    Bindings { table: u8, bindings: Vec<Binding> },
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
            // The table this delta is a diff of. `movzx edx, bl` (keyboard preset) or
            // `mov dl, 3` (controller) at the builder's two call sites; module docs §4. Until
            // 2026-09-14 this was read and dropped as "a flag that is always 0", and it was
            // 0 in every capture until somebody opened the Controller tab.
            let table = c.u8().ok()?;
            if table as usize >= PRESET_COUNT {
                return None;
            }
            // The two `0xFFFFFFFF` words are read and discarded: every capture has had both
            // set to -1 and nothing in the client's builder computes them from the layout.
            // They are preserved in the research file rather than invented here.
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
            Some(Change::Bindings { table, bindings: out })
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

/// How many preset tables `0x05F1` carries. The handler's outer loop is `cmp r15d, 4 / jl`
/// at `0x1419ffdb1`; `0x0199` subtype 3 bounds the selected preset by the same 4. **[L]**
pub const PRESET_COUNT: usize = 4;

/// `0x05F1`: **four** gated tables, one per preset, then the quickslot gate.
///
/// # This was one table on 2026-09-12 and it killed the client at field entry
///
/// The owner: *"Client exited immediately upon logging into the game world."* The client named
/// the packet - `0x009E CLIENT_PACKET_REJECTED`, reason `0x26`, then the whole `0x05F1`
/// verbatim - and faulted 3 ms later (`research/fixtures/keymap-0x05F1-one-preset-rejected-
/// 0x009E-then-fault-*.log`). `research/keyboard-layout-2026-09-08.md` §3 had read the slot
/// loop and stopped; `tools/reads.py 0x1419ffc00 2` lists FOUR read sites and the listing
/// around them says what the packet is (`tools/dis_at.py 0x1419ffc00 0x400`) **[L]**:
///
/// ```text
/// for preset in 0..4:                      0x1419ffc50  loop head, r15d = preset
///     (the table is first reset to the const KEYBOARD preset 0 - r12, for all four)
///     u8  gate                             0x1419ffcf4  0 = READ, non-zero = keep the reset
///     if gate == 0: 89 x { u8 kind; u32 action }        0x1419ffd0b, FUN_1401de920
///     (the table is then copied to its shadow at +0x1bd)
/// u8  quickslots                           0x1419ffdbe  non-zero = READ 32 x u32
/// if quickslots != 0: 32 x raw 4           0x1419ffdec, 0x143ad13f0..0x143ad1470
/// ```
///
/// # And it was three tables and a keep gate until 2026-09-14, which wrecked the controller
///
/// "Keep" keeps the RESET, not the table the client had - and the reset is keyboard preset 0
/// for every index, the controller's included (module docs §4). So all four go out READ now:
/// `tables[3]` is the controller's, [`CLIENT_CONTROLLER_LAYOUT`] plus its deltas.
///
/// **The quickslot gate is 0 because its 32 values are unmeasured**, not because 0 is known
/// to be right: 0 skips to `FUN_1401de860` at `0x1419ffe34`, which has not been read. If the
/// quickslot bar comes up wrong, that call is where to look.
pub fn keymap_init(tables: &[[Slot; SLOT_COUNT]; PRESET_COUNT]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    for table in tables {
        w.u8(GATE_READ);
        for slot in table.iter() {
            w.u8(slot.kind);
            w.u32(slot.action);
        }
    }
    w.u8(0); // quickslots: unmeasured, so not sent
    w.into_vec()
}

/// The length of [`keymap_init`]'s body: four read tables and the quickslot gate.
pub const KEYMAP_INIT_LEN: usize = PRESET_COUNT * (1 + SLOT_COUNT * 5) + 1;

/// `0x05F1` that tells the client to keep every preset as its reset copy of the factory
/// table. Four keep gates and a zero quickslot gate - the same shape as [`keymap_init`],
/// with nothing read. Nothing sends it; see [`restore`].
pub fn keymap_init_keep() -> Vec<u8> {
    let mut w = PacketWriter::new();
    for _ in 0..PRESET_COUNT {
        w.u8(GATE_KEEP);
    }
    w.u8(0);
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
/// `stored` is every binding the character has saved, each with the table it was saved
/// against. `None` in either of two cases, and in both the caller must send **no packet at
/// all**:
///
/// * the character has saved nothing, so there is nothing to restore - the client's own
///   init copies are exactly the four factories this would send;
/// * [`CLIENT_DEFAULT_LAYOUT`] has not been measured, so the slots the player never touched
///   would go out as zeros and unbind their keyboard.
///
/// A stored row for a table the client does not have is skipped, not fatal: it cannot be
/// sent anywhere, and refusing the whole restore over it would lose the other three tables.
///
/// # Why `None` rather than [`keymap_init_keep`]
///
/// The keep form is a real packet and, as of 2026-09-14, a measured-bad one: keep means
/// "keep the reset to keyboard preset 0", for all four tables. Today the server sends nothing
/// at all for a fresh character and the client is demonstrably fine, so returning `None`
/// preserves a measured-good behaviour. `keymap_init_keep` is kept because it is part of the
/// protocol and the gate byte it carries is the thing most likely to be got backwards;
/// nothing sends it.
pub fn restore(stored: Option<&[(u8, Binding)]>) -> Option<Vec<u8>> {
    let (Some(bindings), Some(keyboard)) = (stored, CLIENT_DEFAULT_LAYOUT) else {
        return None;
    };
    if bindings.is_empty() {
        return None;
    }
    let mut tables = [keyboard, CLIENT_PRESETS_1_AND_2[0], CLIENT_PRESETS_1_AND_2[1], CLIENT_CONTROLLER_LAYOUT];
    for (table, b) in bindings {
        if let Some(t) = tables.get_mut(*table as usize) {
            t[b.key as usize] = b.slot;
        }
    }
    Some(keymap_init(&tables))
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
        let Some(Change::Bindings { table, bindings: b }) = parse_change(WISP_CONFIRM) else {
            panic!("subtype 0 did not decode");
        };
        assert_eq!(table, 0, "the keyboard's preset 0");
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
                !matches!(got, Some(Change::Bindings { ref bindings, .. }) if bindings.len() == 3),
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

    /// The body a restore sends when nothing is stored on top: the four factories.
    fn factories() -> [[Slot; SLOT_COUNT]; PRESET_COUNT] {
        [
            CLIENT_DEFAULT_LAYOUT.expect("measured 2026-09-12"),
            CLIENT_PRESETS_1_AND_2[0],
            CLIENT_PRESETS_1_AND_2[1],
            CLIENT_CONTROLLER_LAYOUT,
        ]
    }

    #[test]
    fn the_init_body_is_the_length_the_client_reads() {
        // 4 x (1 gate byte + 89 * (u8 + u32)) + the quickslot gate.
        assert_eq!(keymap_init(&factories()).len(), KEYMAP_INIT_LEN);
        assert_eq!(KEYMAP_INIT_LEN, 1785);
        // Four gates where the handler reads them, then the quickslot gate. The one-table
        // form (446 bytes) put the second gate past the end and the client rejected it; the
        // three-and-a-keep form (1340) handed the controller a keyboard layout.
        let b = keymap_init(&factories());
        let table = 1 + SLOT_COUNT * 5;
        for n in 0..PRESET_COUNT {
            assert_eq!(b[n * table], 0, "table {n}: READ - keep would keep the reset to keyboard preset 0");
        }
        assert_eq!(b[4 * table], 0, "quickslots: not sent");
        assert_eq!(b.len(), 4 * table + 1);
        // Presets 1 and 2 and the controller are the image's, byte for byte.
        let shipped = [&CLIENT_PRESETS_1_AND_2[0], &CLIENT_PRESETS_1_AND_2[1], &CLIENT_CONTROLLER_LAYOUT];
        for (n, preset) in shipped.iter().enumerate() {
            let at = (n + 1) * table + 1;
            for (i, slot) in preset.iter().enumerate() {
                assert_eq!(b[at + i * 5], slot.kind);
                assert_eq!(&b[at + i * 5 + 1..at + i * 5 + 5], &slot.action.to_le_bytes());
            }
        }
    }

    /// The two captures from the Controller tab, 2026-09-14 (`world-ch0.log` 00:56:30 and
    /// 00:58:49): the byte after the subtype is 3, and the 53-entry one opens with the
    /// controller factory table's own first twelve buttons - the client reporting how far
    /// its controller table had been pushed from a shadow that our keep gate had made a
    /// keyboard.
    #[test]
    fn a_controller_delta_names_table_3_and_the_factory_matches_the_capture() {
        let one = [0x00u8, 0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x08, 0x01, 0xe8, 0x03, 0x00, 0x00];
        let Some(Change::Bindings { table, bindings }) = parse_change(&one) else { panic!("did not decode") };
        assert_eq!(table, CONTROLLER_TABLE);
        assert_eq!(bindings, vec![Binding { key: 8, slot: Slot { kind: 1, action: 1000 } }], "button 8 -> skill 1000");

        // The first 192 hex characters the log kept of the 329-byte body.
        let head = "0003ffffffffffffffff350005350000000104910100000204900100000305340000000405330000000505320000000604000000000704030000000804020000000904010000000a042e0000000b040e00000010000000000011000000000012";
        let bytes: Vec<u8> = (0..head.len() / 2).map(|i| u8::from_str_radix(&head[2 * i..2 * i + 2], 16).unwrap()).collect();
        assert_eq!(bytes[1], CONTROLLER_TABLE);
        assert_eq!(bytes[10], 53, "53 entries: the controller default XOR keyboard preset 0");
        for i in 0..12 {
            let at = 11 + i * 6;
            let (key, kind) = (bytes[at] as usize, bytes[at + 1]);
            let action = u32::from_le_bytes(bytes[at + 2..at + 6].try_into().unwrap());
            assert_eq!(key, i);
            assert_eq!(CLIENT_CONTROLLER_LAYOUT[key], Slot { kind, action }, "button {key:#x} is the controller factory's");
        }
        // ...and then it unbinds Q, W and E - scan codes, on a controller - because the
        // shadow it was diffed against was keyboard preset 0, where those are menus.
        for (i, code) in [0x10usize, 0x11].iter().enumerate() {
            let at = 11 + (12 + i) * 6;
            assert_eq!(bytes[at] as usize, *code);
            assert_eq!(&bytes[at + 1..at + 6], &[0, 0, 0, 0, 0]);
            assert_ne!(CLIENT_DEFAULT_LAYOUT.unwrap()[*code], Slot::EMPTY);
        }
        assert_eq!(bytes[95], 0x12, "E is next; the log's 96-byte cap ends on its key byte");
        assert_eq!(CLIENT_CONTROLLER_LAYOUT.iter().filter(|s| **s != Slot::EMPTY).count(), 22);
        assert!(CLIENT_CONTROLLER_LAYOUT[0x28..].iter().all(|s| *s == Slot::EMPTY), "buttons stop at 0x27");
        assert_eq!(parse_change(&[0, 4, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0]), None, "table 4 does not exist");
    }

    /// A controller delta restores onto the CONTROLLER factory, in the fourth table, and
    /// leaves the keyboard's three exactly as the image ships them.
    #[test]
    fn a_controller_binding_restores_into_table_3_over_the_controller_factory() {
        let stored = [(CONTROLLER_TABLE, Binding { key: 8, slot: Slot { kind: 1, action: 1000 } })];
        let body = restore(Some(&stored)).expect("something is stored");
        let table = 1 + SLOT_COUNT * 5;
        let slot = |t: usize, code: usize| {
            let at = t * table + 1 + code * 5;
            (body[at], u32::from_le_bytes(body[at + 1..at + 5].try_into().unwrap()))
        };
        assert_eq!(slot(3, 8), (1, 1000), "button 8 carries the skill");
        assert_eq!(slot(3, 0), (5, 53), "button 0 keeps its controller default");
        assert_eq!(slot(3, 0x10), (0, 0), "no Q on a controller");
        assert_eq!(slot(0, 8), (0, 0), "and the keyboard's scan code 8 is untouched");
        assert_eq!(slot(0, 0x10), (4, 8), "Q is still the keyboard's menu");
        assert_eq!(slot(0, 0x1D), (5, 52));
        // A row for a table the client does not have is skipped, not fatal.
        let junk = [(9u8, Binding { key: 1, slot: Slot { kind: 1, action: 1 } })];
        assert_eq!(restore(Some(&junk)).unwrap(), keymap_init(&factories()));
    }

    /// The gate is inverted and this is the test that says so out loud. If someone ever
    /// "fixes" it to the natural reading, this fails rather than a player's keyboard blanking.
    #[test]
    fn zero_means_read_and_nonzero_means_keep() {
        assert_eq!(keymap_init(&factories())[0], 0);
        let keep = keymap_init_keep();
        assert_eq!(keep.len(), PRESET_COUNT + 1, "four keep gates and the quickslot gate");
        assert!(keep[..PRESET_COUNT].iter().all(|&g| g != 0));
        assert_eq!(keep[PRESET_COUNT], 0);
    }

    #[test]
    fn a_delta_merges_onto_the_layout_and_leaves_the_rest_alone() {
        let mut layout = [Slot::EMPTY; SLOT_COUNT];
        layout[0x10] = Slot { kind: 5, action: 42 };
        let Some(Change::Bindings { bindings: b, .. }) = parse_change(WISP_CONFIRM) else { unreachable!() };
        apply(&mut layout, &b);
        assert_eq!(layout[0x1D], Slot { kind: 1, action: 1_001_002 });
        assert_eq!(layout[0x10], Slot { kind: 5, action: 42 }, "an untouched key survives");
        assert_eq!(layout[0x11], Slot::EMPTY);
    }

    /// Until the factory table is measured, restore must NEVER send the read gate - that is
    /// the difference between doing nothing and unbinding every key a player has.
    #[test]
    fn restore_refuses_to_send_a_table_it_cannot_build() {
        let Some(Change::Bindings { bindings: b, .. }) = parse_change(WISP_CONFIRM) else { unreachable!() };
        let b: Vec<(u8, Binding)> = b.into_iter().map(|b| (0, b)).collect();
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

        let Some(Change::Bindings { bindings: b, .. }) = parse_change(WISP_CONFIRM) else { unreachable!() };
        let b: Vec<(u8, Binding)> = b.into_iter().map(|b| (0, b)).collect();
        let body = restore(Some(&b)).expect("a saved layout now goes out");
        assert_eq!(body[0], 0, "the READ gate");
        assert_eq!(body.len(), KEYMAP_INIT_LEN);
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
        let mut tables = factories();
        tables[0][0x10] = Slot { kind: 4, action: 99 };
        let Some(Change::Bindings { bindings: b, .. }) = parse_change(WISP_CONFIRM) else { unreachable!() };
        apply(&mut tables[0], &b);
        let body = keymap_init(&tables);
        assert_eq!(body[0], 0, "the read gate");
        assert_eq!(body.len(), KEYMAP_INIT_LEN);
        // Q keeps its factory binding; LCtrl carries Slash Blast.
        assert_eq!(body[1 + 0x10 * 5], 4);
        assert_eq!(body[1 + 0x1D * 5], 1);
        assert_eq!(
            u32::from_le_bytes(body[1 + 0x1D * 5 + 1..1 + 0x1D * 5 + 5].try_into().unwrap()),
            1_001_002
        );
    }
}
