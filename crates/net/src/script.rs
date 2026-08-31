//! The NPC script message - inbound `0x055B`, the packet that makes an NPC speak, and the
//! outbound `0x0151` quest request it answers.
//!
//! Labels used throughout, the same ones `research/npc-dialogue.md` uses:
//! **[L]** read off this client's listing, **[D]** derived from two or more [L] facts,
//! **[I]** inferred (including anything from the v214 reference at
//! `C:\Users\user\Desktop\ModernMapleSource`, which is a *different game version* and
//! scored 1 of 8 against a held-out control - candidates only, never evidence).
//!
//! Everything marked [L] below was re-read from `client-patched/MapleStory.exe` with
//! capstone on 2026-08-19, bounded by `.pdata`, and cross-checked against the seven read
//! primitives (`0x1406e8ae0` u8, `0x1406e8b80` u16, `0x1406e8c20` u32, `0x1406e8f00` u32
//! thunk, `0x1406e8f10` u64, `0x1406e9050` string, `0x1406e9170` raw). The instrument was
//! proved on a positive control first: it reproduces `FUN_140304b20`'s documented read
//! sequence (`raw` at `140304b95`, `u8` at `140304ba7`, `u32` at `140304bba`, then the
//! `u8/u8/str/u8/str/u8` run at `140304e79..140304f4c`) exactly as
//! `research/charrecord-decode.md` and `research/charrecord-flag7.md` record it.
//!
//! # NEVER SEND A SCRIPT WITH, OR JUST BEFORE, A `SetField`
//!
//! **[L]** `FUN_142caa4e0` - the field-enter routine - loads the script-manager singleton
//! from `0x143acedb0` at `142caac73` and immediately calls `FUN_141f6f200(it)` at
//! `142caac7a`. `FUN_141f6f200` is the script-manager **reset**: it zeroes the busy latch
//! and releases the UI objects the dialog lives in. So a `0x055B` that arrives with, or
//! anywhere before the client finishes acting on, a `SetField` is destroyed - the packet is
//! dispatched, the dialog is built, and then the field-enter path tears it back down. There
//! is no error and nothing in any log. **This has already cost a run.**
//!
//! Send a script only once the character is settled on a map - anywhere the existing
//! [`crate::opcode::NPC_ENTER_FIELD`] batch already works. In practice that means after the
//! `0x0238`/`0x024D` pair the client sends ~420 ms after a `SetField`, and in the common
//! case it means "in reply to the client's own `0x0151`", which cannot arrive before the
//! field is up.
//!
//! # What the client does *not* require
//!
//! * **No lock, no acknowledgement between packets. [L]** `[scriptMan+8]` is taken at
//!   `141f6f490` and released at the common exit `141f6f9c2` of the *same* dispatch, so the
//!   server may send another `0x055B` immediately.
//! * **No answer to `0x0151` first.** Measured in two sessions: three quest requests went
//!   unanswered with no freeze and the client kept sending movement. That is a property of
//!   this packet, not a general licence - see the "always answer" rule in `CLAUDE.md`.
//!
//! # Nothing here authenticates
//!
//! As everywhere else in this project, the channel socket carries no credentials. A script
//! message is accepted purely because it arrived on the socket.

use crate::packet::{PacketReader, PacketWriter};

// ---------------------------------------------------------------------------------------
// Inbound: 0x055B, the script message
// ---------------------------------------------------------------------------------------

/// `ScriptMessage` - the packet that puts an NPC dialog on screen.
///
/// **Routing, [L].** `CField::OnPacket` (`FUN_141820080`) - the same dispatcher that already
/// delivers `NPC_ENTER_FIELD` - reaches it through a two-opcode range test:
///
/// ```text
/// 141821f84  lea  eax, [r9 - 0x55b]
/// 141821f8b  cmp  eax, 1
/// 141821f8e  ja   0x141821fa4
/// 141821f90  mov  edx, r9d
/// 141821f93  mov  rcx, qword ptr [rip + 0x22ace16]   ; -> 0x143acedb0, the script manager
/// 141821f9a  call 0x141f6f320
/// ```
///
/// `0x141f6f320` has **no `.pdata` entry** - it is a bare 30-byte stub, which is why
/// `pdata_lookup` reports it falls in no function. It is a two-way tail dispatch [L]:
///
/// ```text
/// 141f6f320  sub  edx, 0x55b
/// 141f6f326  je   0x141f6f335
/// 141f6f328  cmp  edx, 1
/// 141f6f32b  jne  0x141f6f33d       ; neither -> ret, silently
/// 141f6f32d  mov  rdx, r8           ; 0x55C
/// 141f6f330  jmp  0x1412808a0
/// 141f6f335  mov  rdx, r8           ; 0x55B -> the packet pointer becomes arg 2
/// 141f6f338  jmp  0x141f6f350
/// ```
///
/// so the real handler is `FUN_141f6f350(scriptManager, CInPacket*)`, `141f6f350..141f6fb10`.
///
/// **[L]** There is no null check on the singleton at the call site. The argument that it is
/// live once the character is on a map is that `FUN_142caa4e0` dereferences the same global
/// unguarded on the field-entry path - strong, but it is not a construction trace, so a
/// script sent to a client that is *not* on a field is untested and would fault at
/// `141f6f406` if the singleton were null.
pub const SCRIPT_MESSAGE: u16 = 0x055B;

/// Message type `0` - **Say**, the one-line "an NPC says something" dialog.
///
/// **[L], mechanically.** Head field 6 indexes a 71-entry jump table of RVAs at
/// `0x141f6f9f4`; `71 * 4 = 0x11C` and `0x141f6f9f4 + 0x11C = 0x141f6fb10`, which is exactly
/// where `.pdata` ends the function - so the table has 71 entries and no more. Entry `[0]`
/// is `0x01f6f4b5` -> `0x141f6f4b5`, a stub that calls `0x141f6fb20` with
/// `(this, handle, headField2, speaker, packet, flags, headField8)`. `FUN_141f6fb20` is the
/// Say handler decoded below.
///
/// `[ui+0x2a8] = 0` is what makes `CreateLayout` pick `FUN_142a65740`, i.e.
/// `UI/UtilDlgEx.img/UtilDlgEx` - the NPC script dialog with `BtPrev / BtNext / BtOK /
/// BtClose`. **[D]**
pub const SCRIPT_TYPE_SAY: u8 = 0x00;

/// Message type `0x47` - force the open dialog shut.
///
/// **[L], and it has a trap.** `141f6f497` is `cmp r12d, 0x46 / ja 141f6f9c2`, so on the
/// *free* path (no dialog open) type `0x47` falls past the end of the jump table, releases
/// the latch and does nothing at all. It only acts when the latch is **held**: the busy
/// branch at `141f6f410` accepts `0x47` alone, reads one more `u8` at `141f6f46c`, computes
/// `result == 1`, and calls `FUN_1410dea80` on the global UI at `0x143aca0f0`.
///
/// So this is "close the dialog that is up", never "open nothing".
pub const SCRIPT_TYPE_FORCE_CLOSE: u8 = 0x47;

/// Head flag `0x0004`: an **extra speaker `u32` rides in the Say body**, overriding the head's
/// speaker template.
///
/// **[L]** `141f6fb7a  test sil, 4 / je 141f6fb8b` gates the `u32` read at `141f6fb83`.
/// This bit changes the body length and there is no resynchronisation point after it, so
/// [`Say`] derives it from [`Say::body_speaker_override`] rather than letting a caller set it
/// by hand.
pub const SCRIPT_FLAG_BODY_SPEAKER: u16 = 0x0004;

/// Head flag `0x0020`: dialog style 1.
///
/// **[L]** `141f6fbbe  test sil, 0x20` -> `ebx = 1`, which becomes the `CUIScriptMsg` ctor's
/// second argument and `[ui+0x2a0]`.
pub const SCRIPT_FLAG_STYLE_1: u16 = 0x0020;

/// Head flag `0x0080`: dialog style 2.
///
/// **[L], and this corrects `research/npc-dialogue.md` §2.3**, which named `0x20`/`0x40` as
/// the style bits. The listing is:
///
/// ```text
/// 141f6fbbe  test  sil, 0x20
/// 141f6fbc2  je    0x141f6fbcb
/// 141f6fbc4  mov   ebx, 1
/// 141f6fbc9  jmp   0x141f6fbd5
/// 141f6fbcb  movzx ebx, sil
/// 141f6fbcf  shr   ebx, 6
/// 141f6fbd2  and   ebx, 2
/// ```
///
/// `(flags >> 6) & 2` is non-zero only for bit **7** (`0x80`): `0x40 >> 6 = 1`, `1 & 2 = 0`.
/// So the second style bit is `0x0080`, not `0x0040`. The listing wins; the research file has
/// been corrected.
pub const SCRIPT_FLAG_STYLE_2: u16 = 0x0080;

/// Head flags `0x0006`: sets `[ui+0x6b8]`.
///
/// **[L]** `FUN_142a62ba0` is `test dl, 6 / mov [rcx+0x6b4], edx / setne al /
/// mov [rcx+0x6b8], eax` at `142a62ba6..142a62bba`. Cosmetic as far as anything measured
/// here goes; listed so the bit is not mistaken for spare.
pub const SCRIPT_FLAGS_SETS_6B8: u16 = 0x0006;

/// One "an NPC says a line" message, message type [`SCRIPT_TYPE_SAY`].
///
/// # The wire body, read for read
///
/// Head, `FUN_141f6f350`, in address order which is execution order (the reads are a straight
/// run of `call` sites with no back edge) [L]:
///
/// ```text
/// off  size  read at      field
///   0     4  141f6f382    handle          -> r15d; echoed as the first u32 of 0x00F3
///   4     1  141f6f38d    (unused)        -> ebp; never read on the Say path
///   5     4  141f6f398    speakerTemplate -> esi
///   9     1  141f6f3a2    hasOverride     presence flag for the next field
///  10     4  141f6f3b1    speakerOverride ONLY IF hasOverride != 0   (u32 thunk 0x1406e8f00)
///  10+    1  141f6f3e6    messageType     -> r12d, indexes the 71-entry table at 0x141f6f9f4
///  11+    2  141f6f3f2    flags           -> r14d
///  13+    1  141f6f3fe    (head field 8)  -> the CUIScriptMsg ctor, [ui+0x2a4]
/// ```
///
/// `+` means "add 4 if `hasOverride` was non-zero". With no override the head is
/// [`SCRIPT_HEAD_LEN`] = 14 bytes and the message type sits at offset 10.
///
/// Then the Say handler, `FUN_141f6fb20`, six reads and no others in the whole function [L]:
///
/// ```text
///       4  141f6fb6b    echo            echoed as the third field of 0x00F3
///       4  141f6fb83    bodySpeaker     ONLY IF flags & 0x04
///     2+n  141f6fb93    text            u16 BYTE count, then n bytes
///       1  141f6fb9c    prev            -> [ui+0x2f8]
///       1  141f6fba8    next            -> [ui+0x2fc]
///       4  141f6fbb7    trailing u32    -> the CUIScriptMsg ctor's 4th argument
/// ```
///
/// **26 bytes plus the text**, so 32 for `"Hello."`. Each optional `u32` adds exactly 4.
///
/// # The one field that must be real
///
/// **[L]+[D]** `speakerTemplate` (or whichever override supersedes it) reaches
/// `FUN_142a61900(ui, msgType, speaker, &text)`, which stores it at `[ui+0x2cc]`
/// (`142a61997  mov dword ptr [rcx + 0x2cc], r8d`). `[ui+0x2cc]` is then loaded straight into
/// `FUN_141e77b70` at `142a7b514` - the NPC template loader `research/npc-spawn.md`
/// identified, the one that formats `Npc/%07d.img`. Use a template id that exists in
/// `Npc.wz`; `gm-handbook/npcs.txt` has them. **`0` is not one.**
///
/// **A refinement the listing forces, [L]:** the load result is null-checked at `142a7b52a`
/// and falls back to `[ui+0x6f0]`, and if that is also null the whole speaker-image block is
/// skipped (`je 0x142a7b6ca`). So a bad template most likely degrades to a dialog with no
/// portrait rather than a fault. That is a *softer* claim than "must exist or it breaks", and
/// it is what the code says. It is still not a reason to send zero.
///
/// # Fields you may choose freely
///
/// `handle` and `echo` come straight back in the client's `0x00F3` answer and nothing in the
/// client compares them to anything [L] - `FUN_141f6fb20`'s immediate-answer path at
/// `141f6fd33..141f6fd9e` builds `0x00F3` as `u32 handle, u8 0, u32 echo, str text,
/// u8 result` and sends it. So the server defines their meaning. Head field 2 and head field
/// 8 are read and are provably never consulted on the Say path; zero is as good as anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Say {
    /// Head field 1. Echoed verbatim as the first `u32` of the client's `0x00F3`. Use it as a
    /// correlation token; the client attaches no meaning to it. **[L]** that it is echoed,
    /// **not established** what it was originally for.
    pub handle: u32,
    /// Head field 2. Read at `141f6f38d` and never used on the Say path. **[L]**
    pub head_field_2: u8,
    /// Head field 3: the `Npc.wz` template whose portrait and name the dialog wears.
    pub speaker_template: u32,
    /// Head field 5, present iff `Some`. When it is `> 0` *and* a global gate holds
    /// (`[0x143aa84a0]` non-null, a second global non-zero, and `FUN_142cc1e30()` non-zero)
    /// it replaces [`Say::speaker_template`] via `cmovg esi, r14d` at `141f6f3df`. **[L]**
    ///
    /// The `u32` is read whenever the presence byte is non-zero, gate or no gate - so this is
    /// purely a length question for the server.
    pub head_speaker_override: Option<u32>,
    /// Body field 10, present iff `Some`; sets [`SCRIPT_FLAG_BODY_SPEAKER`] in `flags`.
    /// Overrides the speaker unconditionally (`mov r14d, eax` at `141f6fb88`, no gate). **[L]**
    pub body_speaker_override: Option<u32>,
    /// Head field 7, minus [`SCRIPT_FLAG_BODY_SPEAKER`] - that bit is derived from
    /// [`Say::body_speaker_override`] so the two can never disagree, because they disagreeing
    /// desynchronises the rest of the body with no resynchronisation point.
    pub style_flags: u16,
    /// Head field 8. Reaches the `CUIScriptMsg` ctor as `[ui+0x2a4]`. **[L]**
    pub head_field_8: u8,
    /// Body field 9. Echoed as the third field of `0x00F3`. **[L]**
    pub echo: u32,
    /// The line the NPC says. Written as a `u16` **byte** count then that many bytes - not
    /// NUL-terminated, not UTF-16. A zero-length string renders an empty box. **[L]**
    pub text: String,
    /// Body field 12 -> `[ui+0x2f8]`, the Prev button. **[L]**
    pub prev: bool,
    /// Body field 13 -> `[ui+0x2fc]`, the Next button. **[L]**
    pub next: bool,
    /// Body field 14, the `CUIScriptMsg` ctor's 4th argument. **[L]** that it is read and
    /// passed; **not established** what it means. Zero is what this project sends.
    pub trailing: u32,
}

impl Say {
    /// The smallest Say that is worth sending: a real speaker, a line, no buttons.
    pub fn new(speaker_template: u32, text: impl Into<String>) -> Self {
        Self {
            handle: 0,
            head_field_2: 0,
            speaker_template,
            head_speaker_override: None,
            body_speaker_override: None,
            style_flags: 0,
            head_field_8: 0,
            echo: 0,
            text: text.into(),
            prev: false,
            next: false,
            trailing: 0,
        }
    }

    /// Set the correlation token the client echoes back in `0x00F3`.
    pub fn with_handle(mut self, handle: u32) -> Self {
        self.handle = handle;
        self
    }

    /// Show the Prev and/or Next buttons instead of a bare OK box.
    pub fn with_buttons(mut self, prev: bool, next: bool) -> Self {
        self.prev = prev;
        self.next = next;
        self
    }

    /// Head field 7 as it goes on the wire: the caller's style bits, plus
    /// [`SCRIPT_FLAG_BODY_SPEAKER`] iff a body speaker override is actually present.
    pub fn flags(&self) -> u16 {
        let mut f = self.style_flags & !SCRIPT_FLAG_BODY_SPEAKER;
        if self.body_speaker_override.is_some() {
            f |= SCRIPT_FLAG_BODY_SPEAKER;
        }
        f
    }

    /// Exactly how many bytes [`Say::body`] will produce.
    ///
    /// `26 + text` fixed, `+4` per optional `u32`. Both optionals change the length and the
    /// client has no way to resynchronise if the server and the flags disagree, which is why
    /// the flag bit is derived rather than set.
    pub fn body_len(&self) -> usize {
        SAY_FIXED_LEN
            + self.text.chars().count()
            + 4 * usize::from(self.head_speaker_override.is_some())
            + 4 * usize::from(self.body_speaker_override.is_some())
    }

    /// The `0x055B` **body** - no opcode; the caller prepends [`SCRIPT_MESSAGE`], the same way
    /// [`crate::opcode::npc_enter_field`] is used.
    pub fn body(&self) -> Vec<u8> {
        let mut w = PacketWriter::new();
        // -- head, FUN_141f6f350 --
        w.u32(self.handle); //                          141f6f382
        w.u8(self.head_field_2); //                     141f6f38d
        w.u32(self.speaker_template); //                141f6f398
        match self.head_speaker_override {
            Some(v) => {
                w.u8(1); //                             141f6f3a2  presence
                w.u32(v); //                            141f6f3b1  read only when presence != 0
            }
            None => {
                w.u8(0); //                             141f6f3a2
            }
        }
        w.u8(SCRIPT_TYPE_SAY); //                       141f6f3e6
        w.u16(self.flags()); //                         141f6f3f2
        w.u8(self.head_field_8); //                     141f6f3fe
        // -- Say body, FUN_141f6fb20 --
        w.u32(self.echo); //                            141f6fb6b
        if let Some(v) = self.body_speaker_override {
            w.u32(v); //                                141f6fb83, gated on flags & 0x04
        }
        w.str(&self.text); //                           141f6fb93, u16 byte count then bytes
        w.bool(self.prev); //                           141f6fb9c
        w.bool(self.next); //                           141f6fba8
        w.u32(self.trailing); //                        141f6fbb7
        let out = w.into_vec();
        debug_assert_eq!(out.len(), self.body_len());
        out
    }
}

/// The head every `0x055B` carries, whatever its message type, when `hasOverride` is `0`:
/// `4 + 1 + 4 + 1` then `1 + 2 + 1`. The message type sits at offset **10** and the flags at
/// **11..13** - both shift by 4 when a head speaker override is present.
pub const SCRIPT_HEAD_LEN: usize = 14;

/// The fixed part of a Say body: everything except the text bytes and the two optional
/// `u32`s. [`SCRIPT_HEAD_LEN`] plus `4 + 2 + 1 + 1 + 4`.
pub const SAY_FIXED_LEN: usize = SCRIPT_HEAD_LEN + 12;

/// The minimum-viable "an NPC says one line" body.
///
/// `speaker_template` must be a real `Npc.wz` id - see [`Say`]. For map 1's Heena that is `1`,
/// and it is the **template**, not our object id: the object id is a pool key we invent
/// (`crates/world/src/config.rs` starts them at 1000) and means nothing to the template
/// loader.
///
/// ```
/// use net::script::{npc_say, SCRIPT_MESSAGE};
/// let body = npc_say(1, "Hello.", false, false);
/// assert_eq!(SCRIPT_MESSAGE, 0x055B);
/// assert_eq!(body.len(), 32);
/// ```
pub fn npc_say(speaker_template: u32, text: &str, prev: bool, next: bool) -> Vec<u8> {
    Say::new(speaker_template, text)
        .with_buttons(prev, next)
        .body()
}

/// A [`SCRIPT_TYPE_FORCE_CLOSE`] body: shut whatever dialog is open.
///
/// **[L]** The head is read in full before the latch is examined, so the same
/// [`SCRIPT_HEAD_LEN`]-byte head goes out; the busy branch then reads one `u8` at
/// `141f6f46c` and compares it to `1` (`cmp al, 1 / sete dl`). Everything else in the head is
/// ignored on this path.
///
/// **This is a no-op when no dialog is open** - see [`SCRIPT_TYPE_FORCE_CLOSE`]. It is not a
/// way to reset the client's script state from the server.
pub fn script_force_close(result: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(0); //                    handle
    w.u8(0); //                     head field 2
    w.u32(0); //                    speaker - unread on this path
    w.u8(0); //                     hasOverride
    w.u8(SCRIPT_TYPE_FORCE_CLOSE); // message type
    w.u16(0); //                    flags
    w.u8(0); //                     head field 8
    w.u8(result); //                141f6f46c
    w.into_vec()
}

// ---------------------------------------------------------------------------------------
// Outbound from the client: 0x0151, the quest request
// ---------------------------------------------------------------------------------------

/// The client's **quest request** - *not* "NPC click".
///
/// `STATUS.md` §2 once called field 1 an object id. It is not, and our own logs are the
/// evidence: `crates/world/src/config.rs` numbers every map's first NPC `1000`, yet the
/// client answered `1000 / 1002 / 1003 / 1005` for four different NPCs, identically across
/// two sessions with opposite visit orders. Field 1 is a value the client holds per NPC, and
/// the builder says which: a **quest id**. Field 2 *does* match the template we sent, in all
/// four captures. **[D]**
///
/// **The builder is `FUN_141f0e4c0`, `141f0e4c0..141f0ee7f`, and it has six `0x0151` sites -
/// re-verified from the listing here, not taken from a table [L]:**
///
/// | site | tag | body after the `u8` tag | bytes |
/// |---|---|---|---|
/// | `141f0e6e9` | 6 | `u32 q, u32 n, u16 x, u16 y` | 13 |
/// | `141f0e797` | 2 | `u32 q, u32 n, u32 0xFFFFFFFF` | 13 |
/// | `141f0e829` | 4 | `u32 q, u32 n, u16 x, u16 y` | 13 |
/// | `141f0e8cc` | 5 | `u32 q, u32 n, u16 x, u16 y` | 13 |
/// | `141f0ec94` | 2 | `u32 q, u32 n, [u16 x, u16 y], u32 sel` | 17 or 13 |
/// | `141f0ed61` | 1 | `u32 q, u32 n, [u16 x, u16 y], u32 sel` | 17 or 13 |
///
/// The tag bytes are `mov dl, 6 / 2 / 4 / 5 / 2 / 1` at `141f0e6fb`, `141f0e7a9`,
/// `141f0e83b`, `141f0e8de`, `141f0eca6`, `141f0ed73`. On the last two sites the `x,y` pair
/// is skipped when `FUN_1407155d0(questId)` returns non-zero (`test eax, eax / jne` at
/// `141f0ece4` and `141f0edb1`) - a property of the *quest*, not of the click, and the only
/// way a 17-byte body becomes 13.
///
/// `x, y` are the **character's own** position, written as `u16` from two `LONG`s
/// (`movzx edx, word ptr [rsp+0x58]`): the pair `(-256, 215)` in one capture appears verbatim
/// in a `0x00D9` movement packet six seconds earlier. **[D]**
///
/// **There is also an unrelated inbound `case 0x151`** in the channel dispatcher
/// (`FUN_142da9d00`). Directions are separate namespaces; do not conflate them.
/// Message type **0x10**: the quest yes/no prompt, `BtQYes` / `BtQNo`.
///
/// **This is what an "Accept" button actually needs, and sending a Say instead is why
/// The owner's Accept did nothing on 2026-08-19.** A type-0 Say with `next = 0` draws `BtOK` and
/// `BtClose` - not Accept and Decline - so pressing it returned the same `action = 1` an OK
/// does, and the server had nothing to branch on. **[L]**, `research/script-reply.md`.
///
/// Type **3** is the plain `BtYes`/`BtNo` pair; `0x10` is the quest-flavoured one, and with
/// `flags & 0x10` it draws `BtQStart`/`BtQAfter` instead.
/// `0x055B` message type **6**: the list box.
///
/// **[L]**, re-read here at `141f73740` rather than taken from another module - see the
/// module doc for why this file carries its own copy.
pub const SCRIPT_TYPE_MENU: u8 = 6;

/// The fixed part of a type-6 body: [`SCRIPT_HEAD_LEN`] plus the `u16` string
/// length. No `echo`, no `prev`/`next`, no trailing `u32` - `FUN_141f73740` makes **two**
/// reads and no others (`141f73789`, `141f73799`). **[L]**
pub const MENU_FIXED_LEN: usize = SCRIPT_HEAD_LEN + 2;

/// What the player did with the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuReply {
    /// The `#L` number of the line clicked, or `None` when the box was closed.
    pub selection: Option<u32>,
}

/// Decode a `0x00F3` that is answering a [`SCRIPT_TYPE_MENU`] box.
///
/// **[`parse_script_reply`] cannot do this, and it fails silently.** That function
/// branches on the message type and its fall-through arm reads a Say-shaped
/// `u32 echo, str text, u8 action`; against a 10-byte type-6 body the string's `u16` length
/// needs two bytes that do not exist, `PacketReader` errors, and the whole packet is dropped.
/// That is exactly the shape of the bug that cost Roger's quest, recorded in that same file.
/// So this is a separate decoder and the wiring patch calls it **first**.
///
/// `None` means "not a type-6 answer" - the caller must fall through to the ordinary script
/// path rather than treat it as a cancel.
///
/// ```text
/// accepted   u32 0, u8 6, u8 1, u32 selection    10 bytes   141f7398e / 141f73995
/// cancelled  u32 0, u8 6, u8 0                    6 bytes   141f7397c
/// ```
pub fn parse_menu_reply(body: &[u8]) -> Option<MenuReply> {
    let mut r = crate::packet::PacketReader::new(body);
    let _handle = r.u32().ok()?;
    if r.u8().ok()? != SCRIPT_TYPE_MENU {
        return None;
    }
    // The accept byte is written as a literal 1 or a literal 0 and nothing else. Anything
    // other than 1 is treated as "closed", which is the safe direction: it routes nobody.
    if r.u8().ok()? != 1 {
        return Some(MenuReply { selection: None });
    }
    // An accepted box **must** carry the selection. A truncated one is not a cancel - it is a
    // body this server does not understand, and inventing a `None` here would silently turn a
    // malformed packet into "the player pressed Close", which is a free ride away from being
    // a bug that matters.
    let selection = r.u32().ok()?;
    Some(MenuReply { selection: Some(selection) })
}

/// The `0x055B` **body** for a menu - no opcode; the caller prepends
/// [`net::script::SCRIPT_MESSAGE`].
///
/// # This is where the type-6 menu lives
///
/// `crate::world::taxi` carried the only copy, and its own doc block asked for it to be moved
/// here *"next to `npc_say` and `npc_ask`, where neither agent's churn can reach them"* -
/// `crates/net/` was not that agent's to edit. `crate::world::secondjob` is the second caller,
/// which is the point at which a note about a duplicate becomes a duplicate.
///
/// `flags = 0` deliberately: bit `0x04` would add a speaker `u32` to the body and **there is
/// no resynchronisation point** if the bit and the value disagree, and bit `0x01` would
/// remove the Close button - the player's only way out if the list does not render.
pub fn npc_menu(speaker_template: u32, text: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(0); //                     141f6f382  handle - type 6 writes a literal 0 back
    w.u8(0); //                      141f6f38d  head field 2, unread on this path
    w.u32(speaker_template); //      141f6f398
    w.u8(0); //                      141f6f3a2  hasOverride - 0 means no u32 follows
    w.u8(SCRIPT_TYPE_MENU); //       141f6f3e6  message type
    w.u16(0); //                     141f6f3f2  flags
    w.u8(0); //                      141f6f3fe  head field 8
    w.str(text); //                  141f73799  u16 BYTE count, then bytes
    w.into_vec()
}

pub const SCRIPT_TYPE_QUEST_YES_NO: u8 = 0x10;

/// Message type 3: the plain `BtYes` / `BtNo` prompt.
pub const SCRIPT_TYPE_YES_NO: u8 = 0x03;

/// A yes/no prompt. Body after the shared head is just `[u32 speaker if flags & 4], str`.
///
/// **On this box the answer byte is unambiguous** - `1` Yes, `0` No, `0xFF` closed - which
/// is exactly what a Say cannot tell you, because the client rewrites `BtOK` to the `Next`
/// result at `142a59fb5` and both come back as `1`.
pub fn npc_ask(speaker_template: u32, text: &str, quest_flavoured: bool) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(0); //                                    handle
    w.u8(0); //                                     head field 2
    w.u32(speaker_template);
    w.u8(0); //                                     hasOverride
    w.u8(if quest_flavoured { SCRIPT_TYPE_QUEST_YES_NO } else { SCRIPT_TYPE_YES_NO });
    w.u16(0); //                                    flags: no body speaker, no BtQStart pair
    w.u8(0); //                                     head field 8
    w.str(text);
    w.into_vec()
}

/// What the user pressed, from a [`CLIENT_SCRIPT_REPLY`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptReply {
    pub handle: u32,
    /// The **echoed message type** - the same literal the server put in the head. This is
    /// the reliable "which box is this answering" discriminator.
    ///
    /// The first reading of this packet put it after the second `u32`; the capture could not
    /// tell, because all nine leading bytes were zero either way. The builder can. **[L]**
    pub message_type: u8,
    pub echo: u32,
    /// The text of the box, echoed back byte for byte. Confirmed at two lengths in one
    /// capture: 16 bytes for an NPC's `d0` line, 162 for a quest opening.
    pub text: String,
    /// **Signed.** On a Say: `1` = Next *or* OK (the client collapses them), `0` = Prev,
    /// `-1` = END CHAT. On a yes/no box: `1` = Yes, `0` = No, `-1` = closed.
    pub action: i8,
}

/// The client's answer to a script message.
pub const CLIENT_SCRIPT_REPLY: u16 = 0x00F3;

/// Decode a [`CLIENT_SCRIPT_REPLY`] body (no opcode).
pub fn parse_script_reply(body: &[u8]) -> Option<ScriptReply> {
    let mut r = PacketReader::new(body);
    let handle = r.u32().ok()?;
    let message_type = r.u8().ok()?;

    // **A yes/no box echoes NOTHING, and reading an echo here cost Roger's quest.**
    //
    // The owner, 2026-08-21: *"Roger's Apple quest does not subtract the player's HP, and does
    // not grant the player a Roger's Apple."* Both are effects of accepting, and the accept
    // never landed - because this function returned `None` for it and
    // `Session::on_script_reply` turns `None` into silence.
    //
    // The two shapes, both measured in one capture:
    //
    // ```text
    // Say     122 bytes  00000000 00 00000000 6e00 <110 chars> 01
    //                    handle   ^type=Say   ^echo ^the box's own text   ^action
    // yes/no    6 bytes  00000000 10 01
    //                    handle   ^type=0x10  ^action
    // ```
    //
    // So a yes/no reply carries handle, type and action and stops. Reading an `echo` there
    // needs four bytes that do not exist, `PacketReader` errors, and the whole packet is
    // dropped. The mirror of the outbound rule already recorded in this module: a Say body
    // has a `u32 echo` before its string and a yes/no box does not.
    if message_type == SCRIPT_TYPE_YES_NO || message_type == SCRIPT_TYPE_QUEST_YES_NO {
        let action = r.u8().ok()? as i8;
        return Some(ScriptReply { handle, message_type, echo: 0, text: String::new(), action });
    }

    let echo = r.u32().ok()?;
    let text = r.str().ok()?;
    let action = r.u8().ok()? as i8;
    Some(ScriptReply { handle, message_type, echo, text, action })
}

/// The answer byte for Yes, and for Next on a Say.
pub const SCRIPT_ACTION_YES: i8 = 1;
/// No on a yes/no box; Prev on a Say.
pub const SCRIPT_ACTION_NO: i8 = 0;
/// The user closed the box. **Send nothing more** - the conversation is over.
pub const SCRIPT_ACTION_CLOSED: i8 = -1;

#[cfg(test)]
mod yes_no_reply_tests {
    use super::*;

    /// **The six bytes that broke Roger's quest**, verbatim from `world.log` 17:39:24.904.
    ///
    /// A yes/no box replies with `handle, type, action` and nothing else. Reading an `echo`
    /// after the type needs four bytes that are not there, so the packet was dropped and the
    /// Accept never reached `Session::on_script_reply` - which is why the quest granted no
    /// apple and took no HP.
    #[test]
    fn a_yes_no_reply_is_six_bytes_with_no_echo_and_no_text() {
        let body = [0x00, 0x00, 0x00, 0x00, 0x10, 0x01];
        let r = parse_script_reply(&body).expect("six bytes is a whole yes/no reply");
        assert_eq!(r.message_type, SCRIPT_TYPE_QUEST_YES_NO);
        assert_eq!(r.action, SCRIPT_ACTION_YES);
        assert!(r.text.is_empty(), "a yes/no box echoes nothing back");

        // No, on the same shape.
        let no = parse_script_reply(&[0x00, 0x00, 0x00, 0x00, 0x10, 0x00]).unwrap();
        assert_eq!(no.action, SCRIPT_ACTION_NO);
        // And the plain yes/no type behaves the same way.
        let plain = parse_script_reply(&[0x00, 0x00, 0x00, 0x00, 0x03, 0x01]).unwrap();
        assert_eq!(plain.action, SCRIPT_ACTION_YES);
    }

    /// The long form still parses - the fix must not cost the Say path.
    ///
    /// This is the other real body from the same capture, trimmed: handle, type Say, echo,
    /// a counted string, then the action.
    #[test]
    fn a_say_reply_still_carries_its_echo_and_its_text() {
        let mut body = vec![0u8; 4]; // handle
        body.push(SCRIPT_TYPE_SAY);
        body.extend_from_slice(&0u32.to_le_bytes()); // echo
        body.extend_from_slice(&5u16.to_le_bytes()); // str len
        body.extend_from_slice(b"Hello");
        body.push(SCRIPT_ACTION_YES as u8);
        let r = parse_script_reply(&body).expect("the long form is unchanged");
        assert_eq!(r.message_type, SCRIPT_TYPE_SAY);
        assert_eq!(r.text, "Hello");
        assert_eq!(r.action, SCRIPT_ACTION_YES);
    }

    /// Short bodies come off a socket, and neither shape may panic.
    #[test]
    fn every_truncation_of_both_shapes_is_refused_rather_than_panicked_on() {
        let yes_no = [0x00, 0x00, 0x00, 0x00, 0x10, 0x01];
        for n in 0..yes_no.len() {
            assert!(parse_script_reply(&yes_no[..n]).is_none(), "yes/no len {n}");
        }
        let mut say = vec![0u8; 4];
        say.push(SCRIPT_TYPE_SAY);
        say.extend_from_slice(&0u32.to_le_bytes());
        say.extend_from_slice(&2u16.to_le_bytes());
        say.extend_from_slice(b"hi");
        say.push(1);
        for n in 0..say.len() {
            assert!(parse_script_reply(&say[..n]).is_none(), "say len {n}");
        }
    }
}

pub const CLIENT_QUEST_REQUEST: u16 = 0x0151;

/// The plain "I clicked this NPC" request. **Not** [`CLIENT_QUEST_REQUEST`].
///
/// ```text
/// u32  npcObjectId    the object id the server gave the NPC in NpcEnterField
/// i16  charX          the CHARACTER's position, not the NPC's
/// i16  charY
/// u32  tail           -1 at three of the four build sites
/// ```
///
/// **Which of the two requests goes out is decided entirely inside the client, by
/// `Quest.wz`.** `FUN_1428de280` is the click handler and forks on `FUN_141e39b50(npc)` -
/// "does this NPC have a non-empty script name" - a string the client fills at construction
/// from its own quest singleton. If a menu line the user picks carries a quest id in
/// `npc->[0x200]/[0x208]/[0x210]`, the client sends `0x0151`; **every other outcome sends
/// this.** So an NPC with no quests can only ever be talked to through `0x00F2`, and a
/// server that answers only `0x0151` is silent for exactly those NPCs. Robin on map 40
/// (template 8) is one, which is why clicking them did nothing on 2026-08-19. **[L]**
///
/// **Field 1 really is our object id, and this time not by correlation.** It is
/// `[npc+0x190]`, written from `NpcEnterField`'s objectId by the `CNpc` constructor at
/// `141e35f59` and by the body decoder `FUN_141e36b20` at `141e36b73`. That matters because
/// the *previous* "read off our own data" claim on this project was wrong: every map's first
/// NPC is given object id 1000, so a `1000` in a packet proves nothing on its own. Here the
/// code reads back the field the spawn packet wrote, which is a different kind of evidence.
///
/// **Fields 2 and 3 are the CHARACTER's position, not the NPC's**, and the first reading of
/// this packet had that wrong. The capture's `01 00 13 01` is `x=1, y=275`; Robin's `cy` is
/// also 275 only because the player was standing on the same ground line. All four build
/// sites read the local user singleton - three through the identical
/// `[0x143aa8518]+8 -> vtbl[0x30]` sequence that also feeds `0x00D9`, one through `rsi` -
/// and that same pair appears verbatim in both `0x00D9` packets of the same capture. **[L]**
///
/// Full working: `research/npc-click.md`.
pub const CLIENT_NPC_CLICK: u16 = 0x00F2;

/// A decoded [`CLIENT_NPC_CLICK`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NpcClick {
    /// `[npc+0x190]` - the object id **the server chose**, not a template id. The reply's
    /// speaker field wants a *template*, so this has to be mapped back through whatever
    /// assigned it.
    pub npc_object_id: u32,
    /// The character's own position, from the local-user singleton.
    pub char_x: i16,
    pub char_y: i16,
    /// `-1` at three of the four build sites; the fourth writes `[npc+0x288]`, an animation
    /// index that is also reachable as `-1`. The four could not be told apart from the wire.
    pub tail: u32,
}

/// Decode a [`CLIENT_NPC_CLICK`] body (no opcode - the payload after it).
///
/// The body is a fixed 12 bytes. A short one returns `None`, which a caller must not turn
/// into silence: see [`NpcClick`] and the "always answer" rule.
pub fn parse_npc_click(body: &[u8]) -> Option<NpcClick> {
    let mut r = PacketReader::new(body);
    Some(NpcClick {
        npc_object_id: r.u32().ok()?,
        char_x: r.i16().ok()?,
        char_y: r.i16().ok()?,
        tail: r.u32().ok()?,
    })
}


/// Tag 1: quest state is not in-progress and the quest has no start script. Carries `sel`.
/// Candidate name `AcceptQuest` **[I]**; what is *measured* is the gate. **[L]** the gate.
pub const QUEST_ACTION_START: u8 = 1;
/// Tag 2: quest state is in-progress. Carries `sel`, the reward chosen; the early form sends
/// `0xFFFFFFFF` for "no choice". Candidate name `CompleteQuest` **[I]**.
pub const QUEST_ACTION_COMPLETE: u8 = 2;
/// Tag 4: not started **and** the quest has a start script. Candidate `OpeningScript` **[I]**.
pub const QUEST_ACTION_OPENING_SCRIPT: u8 = 4;
/// Tag 5: in-progress **and** the quest has an end script. Candidate `CompleteScript` **[I]**.
pub const QUEST_ACTION_COMPLETE_SCRIPT: u8 = 5;
/// Tag 6: a requirement check failed (`FUN_140711d70` returned non-zero). **[L]** the gate;
/// "a refusal / lost-item report" is **[I]**.
pub const QUEST_ACTION_REQUIREMENT_FAILED: u8 = 6;

/// A decoded [`CLIENT_QUEST_REQUEST`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestRequest {
    /// The leading `u8`: which of the six builder sites sent this. See `QUEST_ACTION_*`.
    pub action: u8,
    /// `obj+0x20` - the quest id. Key to six accessors on one global table and to the
    /// requirement checker `FUN_140711d70`; the builder range-tests it against `40000..40999`
    /// and `30051..30079` and reaches the `Quest.wz` node-name strings
    /// (`ask`, `stop`, `/1/stop/%s`, `BtQYes`, `BtQNo`, ...). **[L]**
    pub quest_id: u32,
    /// `obj+0x24` - the NPC **template** id. Matches the template the server sent in
    /// `NPC_ENTER_FIELD` in all four captures. **[L]**
    pub npc_template_id: u32,
    /// The character's own position, when the shape carries it.
    pub pos: Option<(i16, i16)>,
    /// The trailing `u32` on tags 1 and 2: the selection. It was `0` in every capture,
    /// because no dialog was open when the requests were sent.
    pub selection: Option<u32>,
}

/// Decode a [`CLIENT_QUEST_REQUEST`] body (no opcode - the payload after it).
///
/// The 9-byte head is fixed and is **[L]**. The trailer is shape-dependent and the rule below
/// is **[D]** from the six-site table above: the client sends no length or type marker for
/// it, so tag plus remaining length is all there is to go on.
///
/// ```text
/// tag 1, 2   8 trailing bytes -> x, y, selection
/// tag 1, 2   4 trailing bytes -> selection only   (FUN_1407155d0(quest) was non-zero)
/// tag 4,5,6  4 trailing bytes -> x, y
/// ```
///
/// A shape outside that table still returns `Some` with both optionals `None` rather than
/// `None`, deliberately: `None` here is the kind of value a caller turns into "drop it", and
/// an unanswered packet freezes the client's whole UI. `None` is returned only when the fixed
/// head genuinely does not fit.
pub fn parse_quest_request(body: &[u8]) -> Option<QuestRequest> {
    let mut r = PacketReader::new(body);
    let action = r.u8().ok()?;
    let quest_id = r.u32().ok()?;
    let npc_template_id = r.u32().ok()?;

    let carries_selection = matches!(action, QUEST_ACTION_START | QUEST_ACTION_COMPLETE);
    let (pos, selection) = match (carries_selection, r.remaining()) {
        (true, 8) => (Some(r.pos().ok()?), Some(r.u32().ok()?)),
        (true, 4) => (None, Some(r.u32().ok()?)),
        (false, 4) => (Some(r.pos().ok()?), None),
        _ => (None, None),
    };

    Some(QuestRequest { action, quest_id, npc_template_id, pos, selection })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every offset here is the address the client reads that field at, and the client's
    /// decoder has no length prefix anywhere in the head - one wrong width silently shifts
    /// everything after it.
    #[test]
    fn a_minimum_say_body_is_32_bytes_with_the_fields_where_the_client_reads_them() {
        let b = npc_say(1, "Hello.", false, false);
        assert_eq!(b.len(), 32);
        assert_eq!(b.len(), SAY_FIXED_LEN + "Hello.".len());

        assert_eq!(&b[0..4], &0u32.to_le_bytes(), "141f6f382 handle");
        assert_eq!(b[4], 0, "141f6f38d head field 2, unread on the Say path");
        assert_eq!(&b[5..9], &1u32.to_le_bytes(), "141f6f398 speaker template");
        assert_eq!(b[9], 0, "141f6f3a2 hasOverride - 0 means no u32 follows");
        assert_eq!(b[10], SCRIPT_TYPE_SAY, "141f6f3e6 message type");
        assert_eq!(&b[11..13], &0u16.to_le_bytes(), "141f6f3f2 flags");
        assert_eq!(b[13], 0, "141f6f3fe head field 8");
        assert_eq!(&b[14..18], &0u32.to_le_bytes(), "141f6fb6b echo");
        assert_eq!(&b[18..20], &6u16.to_le_bytes(), "141f6fb93 text length");
        assert_eq!(&b[20..26], b"Hello.", "141f6fb93 text bytes");
        assert_eq!(b[26], 0, "141f6fb9c prev");
        assert_eq!(b[27], 0, "141f6fba8 next");
        assert_eq!(&b[28..32], &0u32.to_le_bytes(), "141f6fbb7 trailing u32");

        // The flat bytes written out in research/npc-dialogue.md section 3.
        assert_eq!(
            b,
            vec![
                0x00, 0x00, 0x00, 0x00, // handle
                0x00, // head field 2
                0x01, 0x00, 0x00, 0x00, // speaker template
                0x00, // hasOverride
                0x00, // message type = Say
                0x00, 0x00, // flags
                0x00, // head field 8
                0x00, 0x00, 0x00, 0x00, // echo
                0x06, 0x00, // text length
                0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2e, // "Hello."
                0x00, // prev
                0x00, // next
                0x00, 0x00, 0x00, 0x00, // trailing
            ]
        );
    }

    /// The speaker template goes straight into `FUN_141e77b70` at `142a7b514` via
    /// `[ui+0x2cc]`, so it must be a real `Npc.wz` id and it must be at offset 5.
    #[test]
    fn the_speaker_template_is_a_real_npc_id_at_offset_five() {
        let b = npc_say(9010000, "Hi.", false, false);
        assert_eq!(&b[5..9], &9_010_000u32.to_le_bytes());
        // Nothing here can enforce "the id exists"; what it can enforce is that we never
        // ship the placeholder that made NPCs invisible last time.
        assert_ne!(&b[5..9], &0u32.to_le_bytes());
    }

    /// Both optional `u32`s change the body length and **there is no resynchronisation
    /// point**. If the presence byte or the flag bit disagrees with what is actually on the
    /// wire, every field after it is read from the wrong offset and the client throws or
    /// draws garbage.
    #[test]
    fn each_optional_u32_lengthens_the_body_by_exactly_four() {
        let base = Say::new(1, "Hello.");
        let base_len = base.body().len();
        assert_eq!(base_len, 32);

        let head = Say { head_speaker_override: Some(2), ..base.clone() };
        assert_eq!(head.body().len(), base_len + 4, "head field 5 costs 4");
        assert_eq!(head.body()[9], 1, "141f6f3a2 presence byte is set");
        assert_eq!(&head.body()[10..14], &2u32.to_le_bytes(), "141f6f3b1");
        // Everything after it has shifted by exactly 4.
        assert_eq!(head.body()[14], SCRIPT_TYPE_SAY, "message type moved 10 -> 14");

        let body = Say { body_speaker_override: Some(3), ..base.clone() };
        assert_eq!(body.body().len(), base_len + 4, "body field 10 costs 4");
        assert_eq!(&body.body()[18..22], &3u32.to_le_bytes(), "141f6fb83");
        assert_eq!(body.body()[9], 0, "the head presence byte is NOT the same flag");

        let both = Say {
            head_speaker_override: Some(2),
            body_speaker_override: Some(3),
            ..base.clone()
        };
        assert_eq!(both.body().len(), base_len + 8);
        assert_eq!(both.body_len(), both.body().len(), "body_len() agrees with body()");
    }

    /// `141f6fb7a  test sil, 4` is the *only* thing that tells the client a body speaker
    /// `u32` is coming. The bit is derived from the value so the two can never disagree.
    #[test]
    fn the_body_speaker_flag_tracks_the_value_it_announces() {
        let without = Say::new(1, "Hi.");
        assert_eq!(without.flags() & SCRIPT_FLAG_BODY_SPEAKER, 0);
        assert_eq!(&without.body()[11..13], &0u16.to_le_bytes());

        let with = Say { body_speaker_override: Some(7), ..without.clone() };
        assert_eq!(with.flags() & SCRIPT_FLAG_BODY_SPEAKER, SCRIPT_FLAG_BODY_SPEAKER);
        assert_eq!(&with.body()[11..13], &SCRIPT_FLAG_BODY_SPEAKER.to_le_bytes());

        // A caller who sets the bit by hand without supplying a value cannot desynchronise
        // the body: the derived flag wins.
        let lying = Say { style_flags: SCRIPT_FLAG_BODY_SPEAKER, ..without.clone() };
        assert_eq!(lying.flags(), 0, "the bit is stripped when there is no value");
        assert_eq!(lying.body().len(), SAY_FIXED_LEN + 3, "\"Hi.\" is 3 bytes");

        // Style bits survive alongside it. 0x0080 is style 2 - NOT 0x0040; see
        // SCRIPT_FLAG_STYLE_2 for why the research file was wrong about that.
        let styled = Say { style_flags: SCRIPT_FLAG_STYLE_2, ..with.clone() };
        assert_eq!(styled.flags(), SCRIPT_FLAG_STYLE_2 | SCRIPT_FLAG_BODY_SPEAKER);
    }

    /// `FUN_1406e9050` reads a `u16` byte count and then that many bytes, and it throws on
    /// underrun (`research/msexe-packet-readers.c:110`). Not NUL-terminated, not UTF-16.
    #[test]
    fn the_text_is_u16_length_prefixed_and_not_nul_terminated() {
        let b = npc_say(1, "Hi", false, false);
        assert_eq!(&b[18..20], &2u16.to_le_bytes(), "u16 byte count");
        assert_eq!(&b[20..22], b"Hi");
        assert_eq!(b[22], 0, "the next byte is prev, not a NUL terminator");
        assert_eq!(b.len(), SAY_FIXED_LEN + 2);

        let empty = npc_say(1, "", false, false);
        assert_eq!(&empty[18..20], &0u16.to_le_bytes(), "a zero-length string is legal");
        assert_eq!(empty.len(), SAY_FIXED_LEN);

        // Game text is not UTF-8; high bytes must reach the wire unmangled and the count
        // must stay a *byte* count.
        let high: String = vec![0xC4u8 as char, 0xE9u8 as char].into_iter().collect();
        let b = npc_say(1, &high, false, false);
        assert_eq!(&b[18..20], &2u16.to_le_bytes());
        assert_eq!(&b[20..22], &[0xC4, 0xE9]);

        let long = "x".repeat(300);
        let b = npc_say(1, &long, false, false);
        assert_eq!(&b[18..20], &300u16.to_le_bytes());
        assert_eq!(b.len(), SAY_FIXED_LEN + 300);
    }

    /// `[ui+0x2f8]` / `[ui+0x2fc]`, set by `FUN_142a62bf0` at `142a62c03` / `142a62c09`.
    #[test]
    fn prev_and_next_are_the_last_two_bytes_before_the_trailing_u32() {
        let b = npc_say(1, "Hello.", true, true);
        assert_eq!(b[26], 1, "prev");
        assert_eq!(b[27], 1, "next");
        assert_eq!(b.len(), 32, "buttons do not change the length");

        let next_only = npc_say(1, "Hello.", false, true);
        assert_eq!(next_only[26], 0);
        assert_eq!(next_only[27], 1);
    }

    /// The force-close body is the same 14-byte head plus one `u8`; the head is read in full
    /// before the latch is checked at `141f6f406`.
    #[test]
    fn a_force_close_is_the_head_plus_one_result_byte() {
        let b = script_force_close(1);
        assert_eq!(SCRIPT_HEAD_LEN, 14, "4+1+4+1 then 1+2+1, no override");
        assert_eq!(b.len(), SCRIPT_HEAD_LEN + 1);
        assert_eq!(b[10], SCRIPT_TYPE_FORCE_CLOSE, "message type at offset 10");
        assert_eq!(b[SCRIPT_HEAD_LEN], 1, "141f6f46c result, straight after the head");
    }

    /// Both captured `0x0151` bodies from `research/npc-dialogue.md` §1.5, decoded.
    #[test]
    fn the_two_captured_quest_requests_decode() {
        // 01 e8030000 01000000 0c04 6d01 00000000   (17)
        let seventeen = [
            0x01, 0xe8, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0c, 0x04, 0x6d, 0x01, 0x00,
            0x00, 0x00, 0x00,
        ];
        let q = parse_quest_request(&seventeen).expect("17-byte body");
        assert_eq!(q.action, QUEST_ACTION_START);
        assert_eq!(q.quest_id, 1000);
        assert_eq!(q.npc_template_id, 1);
        assert_eq!(q.pos, Some((1036, 365)));
        assert_eq!(q.selection, Some(0));

        // 04 ea030000 03000000 8402 d700            (13)
        let thirteen = [
            0x04, 0xea, 0x03, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x84, 0x02, 0xd7, 0x00,
        ];
        let q = parse_quest_request(&thirteen).expect("13-byte body");
        assert_eq!(q.action, QUEST_ACTION_OPENING_SCRIPT);
        assert_eq!(q.quest_id, 1002);
        assert_eq!(q.npc_template_id, 3);
        assert_eq!(q.pos, Some((644, 215)));
        assert_eq!(q.selection, None, "tags 4/5/6 carry no selection");
    }

    /// Field 1 is the quest id, not our object id. Four NPCs across two sessions answered
    /// 1000 / 1002 / 1003 / 1005 while every map's first NPC has object id 1000.
    #[test]
    fn field_one_is_a_quest_id_and_field_two_is_the_template_we_sent() {
        for (bytes, quest, template) in [
            ([0x01u8, 0xeb, 0x03, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00], 1003u32, 4u32),
            ([0x01u8, 0xed, 0x03, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00], 1005, 7),
        ] {
            let q = parse_quest_request(&bytes).expect("the 9-byte head alone is enough");
            assert_eq!(q.quest_id, quest);
            assert_eq!(q.npc_template_id, template);
        }
    }

    /// Tags 1 and 2 lose the `x,y` pair when `FUN_1407155d0(questId)` is non-zero. Length is
    /// the only signal, so the parse has to key on it.
    #[test]
    fn a_tag_two_without_a_position_is_selection_only() {
        // The early tag-2 form: u32 q, u32 n, u32 0xFFFFFFFF - "no reward chosen".
        let b = [
            0x02, 0xe8, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
        ];
        let q = parse_quest_request(&b).unwrap();
        assert_eq!(q.action, QUEST_ACTION_COMPLETE);
        assert_eq!(q.pos, None);
        assert_eq!(q.selection, Some(0xFFFF_FFFF));
    }

    /// A short body is the only thing that returns `None`. An unrecognised *trailer* must
    /// not, because a caller that treats `None` as "drop it" would leave the packet
    /// unanswered, and an unanswered packet freezes the client's whole UI.
    #[test]
    fn only_a_truncated_head_fails_to_parse() {
        assert!(parse_quest_request(&[]).is_none());
        assert!(parse_quest_request(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).is_none());

        let head_only = [0x01u8, 0xe8, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00];
        assert!(parse_quest_request(&head_only).is_some());

        let odd_trailer = [
            0x01u8, 0xe8, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xaa, 0xbb, 0xcc,
        ];
        let q = parse_quest_request(&odd_trailer).expect("an odd trailer is still parsed");
        assert_eq!(q.quest_id, 1000);
        assert_eq!(q.pos, None);
        assert_eq!(q.selection, None);
    }

    /// The opcode itself, so a rename or a typo cannot go unnoticed.
    #[test]
    fn the_opcodes_are_the_ones_the_dispatchers_test_for() {
        assert_eq!(SCRIPT_MESSAGE, 0x055B); // 141821f84: lea eax, [r9 - 0x55b]
        assert_eq!(CLIENT_QUEST_REQUEST, 0x0151); // 141f0e6e9: mov edx, 0x151
    }
}
