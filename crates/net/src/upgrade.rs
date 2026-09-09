//! `0x0236` - the scroll result, which is what makes a scroll *look* and *sound* like one.
//!
//! The owner, 2026-09-09: *"whenever the scrolling via `!scroll` succeeds or fails, it should also
//! broadcast the scroll success or scroll fail sound to everyone, just like regular
//! scrolling."*
//!
//! This packet is the whole of that. `research/scrolling-2026-09-09.md` §4 decoded it from the
//! handler `FUN_142792340`, which is the one table-A handler that references
//!
//! ```text
//! 0x1427933fa -> 0x143481cd0  L"EnchantSuccess_Delay"
//! 0x142793401 -> 0x143481d00  L"EnchantFailure_Delay"
//! ```
//!
//! and selects between them on `cmp byte [rbp+0x77], 1` - the [`result`](ItemUpgradeResult)
//! byte. **[L]** Those two names are field effects: the animation and the sound. Nothing else
//! the server can send produces them.
//!
//! # It is one opcode for both the scroller and the bystanders
//!
//! `CUserPool::OnPacket` routes `0x226..0x292` to `FUN_1429bafb0`, which reads a `u32`
//! character id **before** the handler runs, looks that user up in the pool and drops the
//! packet silently if there is no such user. `FUN_142792340` appears exactly once across all
//! four user-pool dispatch tables, so there is no separate "remote" form: the same 14 bytes go
//! to everyone, and the character id says who it happened to. **[L]**
//!
//! Only the *text* is local. The handler gates the chat line on `[vt+0x50]`
//! (`IsLocalUser`-shaped, **[I]**) and runs the effect on a shared tail for everyone.
//!
//! # Not a reply, and it does not carry the stat change
//!
//! It reads four fields, drives a message and an effect, and **touches no item data**. The new
//! stats reach the client in the `0x0070` InventoryOperation beside it. On a destroy
//! (`result = 2`) the item must be removed by `0x0070` as well - this packet alone prints
//! "the item is destroyed" and leaves it sitting on screen. **[D]**
//!
//! # The `0x00B8` unlock is deliberately not mentioned here
//!
//! A real scroll arrives as `0x0125`, whose client-side builder sets the `ctx+0x2330`
//! exclusive-request latch before the server has any say in it, so that path owes an unlock.
//! **`!scroll` is not that path**: it runs off NPC dialogue, `0x0125` is never built, and the
//! latch is never set. Sending an unlock for a latch nobody took would be a guess dressed as
//! caution. If `0x0125` is ever wired, that is where `0x00B8` belongs.

use crate::packet::PacketWriter;

/// Server -> the whole map: a scroll finished. Drives the chat line and the field effect.
pub const ITEM_UPGRADE_EFFECT: u16 = 0x0236;

/// Body length, opcode excluded. `research/scrolling-2026-09-09.md` §8.
///
/// **A length that fails downward is how this client dies.** The handler's four reads are
/// `u8`, `u8`, `u32`, `u32` after the dispatcher's own `u32`, and an underflowing `Decode`
/// walks off the end of the buffer - which is exactly what killed Tester2's client on the
/// first map-chair broadcast. A test asserts this against the builder.
pub const ITEM_UPGRADE_EFFECT_LEN: usize = 14;

/// The `enchantDlg` byte. **Zero is the only value this project sends.**
///
/// Zero selects the ordinary scroll presentation - the plain chat line and the
/// `EnchantSuccess_Delay` / `EnchantFailure_Delay` field effect. Non-zero routes into the
/// enchant-UI object, which was **not traced**, so it is not offered as a parameter: an
/// untraced branch reachable only by passing a number nobody has measured is a trap, not a
/// feature. **[D]**
pub const ENCHANT_DLG_PLAIN: u8 = 0;

/// What the client should say and show. The four arms are the client's own, and every one of
/// them names a string in the client's table - `research/scrolling-2026-09-09.md` §4 has the
/// decrypted text for each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ItemUpgradeResult {
    /// `0x01B8` *"The scroll lights up, but the item winds up as if nothing happened."*
    Failed = 0,
    /// `0x01B6` *"The scroll lights up, and then its mysterious power has been transferred to
    /// the item."*
    Succeeded = 1,
    /// `0x01BA` *"The item is destroyed due to the overwhelming power of the scroll."*
    ///
    /// **Nothing in `crate::scrolls`' rules can produce this** - none of the three scrolls
    /// destroys an item. It is here because the byte is a four-way code and a two-way enum
    /// would misrepresent the wire, not because anything sends it.
    Destroyed = 2,
    /// `0x0DEB` - the refusal arm. Kept for the same reason as `Destroyed`.
    CannotBeUsed = 3,
}

impl ItemUpgradeResult {
    /// The plain success/failure split, which is all the three `!scroll` scrolls can produce.
    pub fn from_success(succeeded: bool) -> Self {
        if succeeded {
            Self::Succeeded
        } else {
            Self::Failed
        }
    }
}

/// Build the 14-byte body.
///
/// `character_id` is **the scroller**, on every copy of the packet including the bystanders':
/// the dispatcher looks the user up by it and drops the packet if that user is not in its
/// pool, so it addresses the subject, not the recipient.
pub fn item_upgrade_effect(
    character_id: u32,
    result: ItemUpgradeResult,
    scroll_item_id: u32,
    equip_item_id: u32,
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id); //         read by the dispatcher, before the handler runs
    w.u8(result as u8); //          0x142792381
    w.u8(ENCHANT_DLG_PLAIN); //     0x14279238c
    w.u32(scroll_item_id); //       0x14279239a - the first %s
    w.u32(equip_item_id); //        0x1427925a1 - the second %s
    w.into_vec()
}

/// The opcode is inside the range `CUserPool::OnPacket` hands to `FUN_1429bafb0`, which is
/// the dispatcher that reads the character id first. Outside it, the id would be read as the
/// body and everything after would be off by four.
const _: () = assert!(ITEM_UPGRADE_EFFECT >= 0x0226 && ITEM_UPGRADE_EFFECT - 0x0226 <= 0x50);

#[cfg(test)]
mod tests {
    use super::*;

    /// **The length, asserted against the builder rather than against a comment.** Every
    /// field the handler reads has to be there; a short body is a client fault, not a
    /// cosmetic bug.
    #[test]
    fn the_body_is_fourteen_bytes_whatever_the_result() {
        for r in [
            ItemUpgradeResult::Failed,
            ItemUpgradeResult::Succeeded,
            ItemUpgradeResult::Destroyed,
            ItemUpgradeResult::CannotBeUsed,
        ] {
            let body = item_upgrade_effect(200, r, 4_031_065, 1_402_043);
            assert_eq!(body.len(), ITEM_UPGRADE_EFFECT_LEN, "{r:?}");
        }
    }

    /// The field offsets, spelled out. `research/scrolling-2026-09-09.md` §8's table is the
    /// only thing standing between this and a silent off-by-one, so it is restated as bytes.
    #[test]
    fn the_fields_land_where_the_handler_reads_them() {
        let body = item_upgrade_effect(0x1234_5678, ItemUpgradeResult::Succeeded, 0x0A0B, 0x0C0D);
        assert_eq!(&body[0..4], &0x1234_5678u32.to_le_bytes(), "charId at 0");
        assert_eq!(body[4], 1, "result at 4");
        assert_eq!(body[5], ENCHANT_DLG_PLAIN, "enchantDlg at 5");
        assert_eq!(&body[6..10], &0x0A0Bu32.to_le_bytes(), "scrollItemId at 6");
        assert_eq!(&body[10..14], &0x0C0Du32.to_le_bytes(), "equipItemId at 10");
    }

    /// The two arms `!scroll` can actually reach, and they must not be swapped - one plays
    /// `EnchantSuccess_Delay` and the other `EnchantFailure_Delay`, and the client picks on
    /// `cmp byte, 1`.
    #[test]
    fn success_is_one_and_failure_is_zero() {
        assert_eq!(ItemUpgradeResult::from_success(true) as u8, 1);
        assert_eq!(ItemUpgradeResult::from_success(false) as u8, 0);
    }
}
