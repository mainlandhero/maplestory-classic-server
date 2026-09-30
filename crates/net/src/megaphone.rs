//! **Megaphones** - the request that uses one, and the `0x00AC` lines that carry it.
//!
//! The owner, 2026-09-29: *"Super megaphone messages should send to clients across all channels (with
//! a pink background), and display the whisper icon depending on client selection. Megaphone
//! messages should send to client on the same channel (without the pink background), and display
//! the whisper icon depending on client selection."*
//!
//! Decompilation: `research/msexe-broadcast-0xac.txt` (the `0x00AC` handler `FUN_142d60d40`),
//! `research/msexe-megaphone.c` (`FUN_142db9320`, `FUN_1408da090`, `FUN_140417eb0`, the builder
//! `FUN_141a50140`) and `research/msexe-megaphone-extra.c` (`FUN_1408d6760`, `FUN_1408dcb80`).
//!
//! # The request: `0x0116`, the ten bytes plus the text and the whisper choice [L]
//!
//! The owner's own Super Megaphone, 2026-09-29 03:52:15, `research/fixtures/super-megaphone-attempt-
//! 2026-09-29-world.log`:
//!
//! ```text
//! 2e76e80f 0700 b15c4d00 0500 48656c6c6f 01
//! tick     slot item      "Hello"         whisper icon: yes
//! ```
//!
//! The builder `FUN_141a50140` writes exactly that - `u32, u16, u32, str, u8` - then a helper
//! whose writes belong to other items; the capture ends at the `u8`, so for these two the
//! helper wrote nothing. Both megaphones go through the one dialog and the one builder.
//!
//! # The line: `0x00AC`, and which type [L] / [D]
//!
//! The types that carry a whisper byte - 3, 8 and 10 - all read, after the message, the
//! sender's chat info (`FUN_142db9320` -> `FUN_1408d6760` -> `FUN_1408dcb80`):
//!
//! ```text
//! str, str, u32, u32, u8, u32, u32, str, u32, str, u32       [L], read order
//! ```
//!
//! and then post through `FUN_1415a8b80(info, text, KIND, channel, whisper, ...)`. The v214
//! reference names the eleven fields - name, message, account id, character id, world,
//! character id again, then zeros and empty strings (`Char::encodeChatInfo`) - which agrees
//! with the shape and is a **candidate** for the names, as everything from that tree is.
//!
//! * **Type 3** - `u8 channel, u8 whisper`, chat kind **`0xd`**. The v214 `ChatType` calls 13
//!   `SpeakerWorld`: the **Super Megaphone**, the pink one.
//! * **Type 8** - `u8 channel, u8 whisper, u32 item, ` then `FUN_1408da090`: a `u32` that is 1
//!   only when an item follows. Kind **`0xf`**, or `0x10` when the `u32` is in
//!   `5076100..5076200` (`FUN_140417eb0`). The reference calls 15 `ItemSpeaker`. **This is the
//!   Megaphone here**: the only type besides the two pink ones (3 and 10, both kind `0xd`)
//!   whose whisper icon follows the byte. Type 2, the plain channel megaphone (kind `0xc`,
//!   `SpeakerChannel`), reads **no whisper byte** at all, so it cannot do what the owner asked.
//!   **[D]** that kind `0xf` is drawn without the pink background: it is a different kind from
//!   the pink one, and nothing here has read the chat list's background table.
//!
//! The message must be `name : text`: the shared arm splits it at the separator
//! (`FUN_142db9e00`), checks the name against the reader's block list, and re-joins it with its
//! own `"%s : %s"`. And a reader below level 10 sees nothing (`9 < level`, both arms).

use crate::packet::{PacketReader, PacketWriter};

/// `Megaphone`, [L] `gm-handbook/items.txt`.
pub const MEGAPHONE: u32 = 5_070_000;
/// `Super Megaphone`, [L] `gm-handbook/items.txt`.
pub const SUPER_MEGAPHONE: u32 = 5_070_001;

/// `0x00AC` type 3 - the Super Megaphone's line (kind `0xd`).
pub const TYPE_SUPER_MEGAPHONE: u8 = 3;
/// `0x00AC` type 8 - used for the Megaphone (kind `0xf`). See the module docs for why not 2.
pub const TYPE_MEGAPHONE: u8 = 8;

/// A decoded megaphone use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MegaphoneUse {
    pub slot: u16,
    pub item_id: u32,
    pub text: String,
    /// The "show the whisper icon" box in the client's dialog.
    pub whisper: bool,
}

/// Read a `0x0116` megaphone body (without the opcode). `None` when it is short.
pub fn parse_megaphone_use(body: &[u8]) -> Option<MegaphoneUse> {
    let mut r = PacketReader::new(body);
    let _tick = r.u32().ok()?;
    let slot = r.u16().ok()?;
    let item_id = r.u32().ok()?;
    let text = r.str().ok()?;
    // Absent means "no": the byte is the last thing the builder writes, and a body without it
    // is a shape nobody has captured - it gets the conservative answer, not an error.
    let whisper = r.u8().map(|b| b != 0).unwrap_or(false);
    Some(MegaphoneUse { slot, item_id, text, whisper })
}

/// Who is speaking - the chat-info block every megaphone type reads after its message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Speaker<'a> {
    pub name: &'a str,
    pub account_id: u32,
    pub character_id: u32,
    pub world: u8,
}

/// The line as the client wants it: `name : text`.
pub fn line(name: &str, text: &str) -> String {
    format!("{name} : {text}")
}

fn chat_info(w: &mut PacketWriter, who: &Speaker, line: &str) {
    w.str(who.name);
    w.str(line);
    w.u32(who.account_id);
    w.u32(who.character_id);
    w.u8(who.world);
    w.u32(who.character_id);
    w.u32(0);
    w.str("");
    w.u32(0);
    w.str("");
    w.u32(0);
}

/// `0x00AC` type 3: a Super Megaphone line, pink, with its channel and whisper icon.
pub fn super_megaphone(who: &Speaker, text: &str, channel: u8, whisper: bool) -> Vec<u8> {
    let line = line(who.name, text);
    let mut w = PacketWriter::new();
    w.u8(TYPE_SUPER_MEGAPHONE);
    w.str(&line);
    chat_info(&mut w, who, &line);
    w.u8(channel);
    w.u8(u8::from(whisper));
    w.into_vec()
}

/// `0x00AC` type 8: a Megaphone line, not pink, with its channel and whisper icon. The item id
/// is the Megaphone's own - outside `5076100..5076200`, so kind `0xf` - and the `u32` after it
/// is 0: no item attached, so nothing more is read.
pub fn megaphone(who: &Speaker, text: &str, channel: u8, whisper: bool) -> Vec<u8> {
    let line = line(who.name, text);
    let mut w = PacketWriter::new();
    w.u8(TYPE_MEGAPHONE);
    w.str(&line);
    chat_info(&mut w, who, &line);
    w.u8(channel);
    w.u8(u8::from(whisper));
    w.u32(MEGAPHONE);
    w.u32(0);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's capture, byte for byte.
    #[test]
    fn the_captured_super_megaphone_use_decodes() {
        let body = hex("2e76e80f0700b15c4d00050048656c6c6f01");
        let u = parse_megaphone_use(&body).unwrap();
        assert_eq!(u, MegaphoneUse { slot: 7, item_id: SUPER_MEGAPHONE, text: "Hello".into(), whisper: true });
        let mut off = body.clone();
        *off.last_mut().unwrap() = 0;
        assert!(!parse_megaphone_use(&off).unwrap().whisper);
        assert!(parse_megaphone_use(&body[..9]).is_none());
    }

    /// The bodies walk the client's reads in order: type, message, the eleven chat-info fields,
    /// then the type's own tail.
    #[test]
    fn the_lines_are_the_shapes_the_client_reads() {
        let who = Speaker { name: "Wisp", account_id: 7, character_id: 215, world: 0 };
        let s = super_megaphone(&who, "Hello", 1, true);
        let mut r = PacketReader::new(&s);
        assert_eq!(r.u8().unwrap(), 3);
        assert_eq!(r.str().unwrap(), "the owner : Hello");
        assert_eq!(r.str().unwrap(), "Wisp");
        assert_eq!(r.str().unwrap(), "the owner : Hello");
        assert_eq!((r.u32().unwrap(), r.u32().unwrap(), r.u8().unwrap(), r.u32().unwrap()), (7, 215, 0, 215));
        assert_eq!((r.u32().unwrap(), r.str().unwrap(), r.u32().unwrap(), r.str().unwrap(), r.u32().unwrap()), (0, String::new(), 0, String::new(), 0));
        assert_eq!((r.u8().unwrap(), r.u8().unwrap()), (1, 1), "channel, whisper");
        assert!(r.u8().is_err(), "nothing after");

        let m = megaphone(&who, "Hi", 0, false);
        assert_eq!(m[0], 8);
        let tail = &m[m.len() - 10..];
        assert_eq!(tail, [0, 0, 0xb0, 0x5c, 0x4d, 0, 0, 0, 0, 0], "channel 0, no whisper, 5070000, no item");
        assert!(!(5_076_100..5_076_200).contains(&MEGAPHONE), "so kind 0xf, not the item megaphone's 0x10");
    }

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }
}
