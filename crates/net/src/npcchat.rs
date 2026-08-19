//! NPC chat balloons - inbound `0x0453`, the packet that makes an NPC talk to itself.
//!
//! **Server-triggered, client-rendered**, and the split is the whole answer. The client
//! already holds everything it needs: `FUN_141e79060`, the 38 KB NPC template parser,
//! resolves `info/speak` into `template+0xE8` and each animation's `<anim>/speak` into
//! 0x38-byte speak groups at `template+0x108`; it holds `UI/ChatBalloon.img`; and it shows a
//! balloon for about five seconds. What it will not do is start one on its own.
//!
//! **The only code that creates a balloon is `FUN_141e3b510`, and its only two callers are
//! both inside `FUN_141e421f0`, the handler for inbound `0x0453`.** Zero vtable slots.
//! **[L]** So no field of the `0x044F` spawn body turns chatter on - which was the leading
//! alternative, and it had form, since `isEnabled` and `alpha` being zero is what made every
//! NPC invisible for days.
//!
//! Full working: `research/npc-chatter.md`.

use crate::PacketWriter;

/// `NpcChat` - make one NPC show a chat balloon.
pub const NPC_CHAT: u16 = 0x0453;

/// `nAction = -1`: show a line without changing the NPC's animation.
///
/// With no animation change the client indexes the **`info/speak`** list, which is the WZ
/// `n*` group. The other three groups - `finger`, `wink`, `heart`, the `f*`/`w*`/`h*`
/// prefixes - hang off their own animation nodes and need their own action value, which is
/// not established.
pub const NPC_CHAT_NO_ANIMATION: i8 = -1;

/// One chat balloon. **10 bytes.**
///
/// ```text
/// u32  objectId     the id the server gave the NPC in NpcEnterField - the pool reads it
/// i8   nAction      -1 for "no animation change, just talk"
/// i8   nChatIdx     index into the condition-filtered info/speak list
/// u32  unused
/// ```
///
/// **The ordering and the cadence are the server's**, because the client has neither: its
/// own picker is `rand() % n`, twice, with no cursor. Cycling in order is therefore a
/// decision rather than a reproduction - which is what the owner asked for. For cadence, the
/// client's own idle timer is `rand() % 6000 + 3000` milliseconds (`FUN_141e46d40`), so
/// three to nine seconds matches its feel. **[L]** for the formula; that the unit is
/// milliseconds is **[D]**, from the same per-frame step decrementing a countdown loaded
/// from a WZ `delay`.
///
/// **Two hazards, both from the same read.** `0x0465` and `0x0466` set `[npc+0x1a8]`, after
/// which this packet's handler **swallows the chat** - do not mix them. And the client has
/// its own driver that asks permission with outbound `0x0327` and latches `[npc+0x170] = 1`
/// until answered; **if that ever arrives it must be answered or that NPC goes silent
/// permanently.** It has never appeared in any capture.
pub fn npc_chat(object_id: u32, action: i8, chat_index: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(object_id);
    w.u8(action as u8);
    w.u8(chat_index);
    w.u32(0);
    w.into_vec()
}

/// The body is fixed at ten bytes.
pub const NPC_CHAT_LEN: usize = 10;

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact ten bytes for Robin's first idle line, which is the smallest thing that
    /// should put a balloon on screen.
    #[test]
    fn a_chat_balloon_is_ten_bytes_with_the_object_id_first() {
        let b = npc_chat(1000, NPC_CHAT_NO_ANIMATION, 0);
        assert_eq!(b.len(), NPC_CHAT_LEN);
        assert_eq!(b, vec![0xE8, 0x03, 0x00, 0x00, 0xFF, 0x00, 0, 0, 0, 0]);

        // The object id is what the pool keys on - it must be the id NpcEnterField gave the
        // NPC, not a template. A template id here silently addresses the wrong NPC or none.
        assert_eq!(u32::from_le_bytes(b[..4].try_into().unwrap()), 1000);

        // nAction is SIGNED. -1 as an unsigned byte is 0xFF, and sending 255 as a positive
        // action would select an animation group that does not exist.
        assert_eq!(b[4], 0xFF);
        assert_eq!(b[4] as i8, -1);

        // And the index really does move.
        assert_eq!(npc_chat(1000, NPC_CHAT_NO_ANIMATION, 3)[5], 3);
    }
}
