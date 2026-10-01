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
//! # Two callers, and only one of them ever latches
//!
//! **`!scroll`** runs off NPC dialogue. `0x0125` is never built, the `ctx+0x2330` latch is
//! never set, and there is nothing to unlock.
//!
//! **`0x0125`** - the client's own scrolling window, handled since 2026-09-09 in
//! `world::session::realscroll` - does latch: the client's builder sets `+0x2330` before the
//! server has any say in it. It is answered by `0x0236` plus a `0x0070`, and the `0x0070`
//! carries `bExclRequestSent = 1`, which is what clears the latch.
//!
//! **No `0x00B8` on either path.** The first reading of `research/scrolling-2026-09-09.md`
//! concluded a third packet was needed; that correction was itself corrected the same day -
//! the scan behind it looked at inline writes and missed the setter *call*. `0x007C` already
//! unlocks, confirmed on the owner's screen.

use crate::packet::PacketWriter;

/// Client -> server: **"put this scroll on that equip."** The client's own scrolling UI.
///
/// The owner, 2026-09-09: *"Just tried scrolling the topwear, it did not work."* It did not: this
/// opcode was decoded in full on 2026-09-09 and never handled, so the server answered it with
/// nothing but the latch unlock. `world.log` of that run:
///
/// ```text
/// 00:59:09.274 <- 0x0125 UNKNOWN, 11 byte body 5696fc000d000100fbff00
/// 00:59:09.275 -> 0x007C StatChanged: UNLOCK ONLY. 0x0125 is not handled ...
/// ```
///
/// which decodes to exactly what they did: scroll in Use slot **13**, onto `dstInvType 1` slot
/// **-5**, the worn topwear.
pub const CLIENT_ITEM_UPGRADE: u16 = 0x0125;

/// **`0x0126` - an "enhancer" scroll dragged onto an equip**: the Lucky Day / Protection
/// family. The owner, 2026-10-01: *"We should allow drag scrolling for lucky day scroll."*
///
/// **[L]**: in the drag handler `FUN_1417df1d0`, a scroll the applicability predicate
/// (`FUN_1404174b0`) refuses goes to `FUN_1417ea820`, which is true for `2530000..2532999`
/// (and subtypes `0x30..0x32`, `0x37`, `0x38`, `0x3D`); that branch shows the client's own
/// confirm (`0x0F13` *"You've selected the %s. Do you want to use the %s on it?"*) and calls
/// `FUN_142cc7bf0` - **the same five fields as [`CLIENT_ITEM_UPGRADE`]**, from the same four
/// arguments (`0x1417e01fd..0x1417e0222` beside `0x1417dfb2b..0x1417dfb50`). It sets the same
/// `+0x2330` latch, so it is answered on every path, exactly like `0x0125`.
/// [`parse_item_upgrade`] reads it.
pub const CLIENT_ITEM_ENHANCER: u16 = 0x0126;

/// Body length of a [`CLIENT_ITEM_UPGRADE`]. Measured on the wire four times in one run.
pub const ITEM_UPGRADE_REQUEST_LEN: usize = 11;

/// A decoded [`CLIENT_ITEM_UPGRADE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemUpgradeRequest {
    /// `FUN_1429e3ef0()`. Echo nothing, ignore it.
    pub tick: u32,
    /// The scroll's slot. Inventory type 2 (Use) or 5 (Cash); **always > 0**. **[L]**
    pub src_slot: u16,
    /// `1` = equip inventory, `6` = the extended equip window. Never anything else. **[L]**
    pub dst_inv_type: u16,
    /// The equip's slot, **signed**: negative means a worn item. The owner's topwear was `-5`.
    pub dst_slot: i16,
    /// Observed 0 (inventory drag) and 1 (the other UI path). **Meaning not established** -
    /// accept both and do not branch on it, which is what the decode note says in as many
    /// words. Carried so a future capture can be compared against it rather than re-derived.
    pub flag: u8,
}

/// Decode a [`CLIENT_ITEM_UPGRADE`] body. `None` when it is not the length the client sends.
///
/// **`dst_slot` is read as an `i16` and that is the whole point.** Read unsigned, the owner's `-5`
/// is 65531, which names no slot and would turn every worn-item scroll into a refusal.
pub fn parse_item_upgrade(body: &[u8]) -> Option<ItemUpgradeRequest> {
    if body.len() < ITEM_UPGRADE_REQUEST_LEN {
        return None;
    }
    let u16_at = |i: usize| u16::from_le_bytes([body[i], body[i + 1]]);
    Some(ItemUpgradeRequest {
        tick: u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        src_slot: u16_at(4),
        dst_inv_type: u16_at(6),
        dst_slot: u16_at(8) as i16,
        flag: body[10],
    })
}

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

    /// **The real body the owner's client sent**, byte for byte out of `world.log`, decoded to
    /// exactly what they were doing on screen: a scroll in Use slot 13 onto their worn topwear.
    ///
    /// A capture is the only thing that can disagree with a field-offset table, so the table
    /// is asserted against one rather than against itself.
    #[test]
    fn the_captured_request_decodes_to_what_was_on_the_screen() {
        // 00:59:09.274 <- 0x0125 UNKNOWN, 11 byte body 5696fc000d000100fbff00
        let body = [0x56, 0x96, 0xfc, 0x00, 0x0d, 0x00, 0x01, 0x00, 0xfb, 0xff, 0x00];
        let r = parse_item_upgrade(&body).expect("11 bytes is the length the client sends");
        assert_eq!(r.tick, 0x00fc_9656);
        assert_eq!(r.src_slot, 13, "the scroll's Use slot");
        assert_eq!(r.dst_inv_type, 1, "1 = the equip inventory");
        assert_eq!(r.dst_slot, -5, "NEGATIVE - a worn item. Topwear is slot 5");
        assert_eq!(r.flag, 0, "an inventory drag");
    }

    /// **`dst_slot` must be signed.** Read as a `u16`, the owner's `-5` is 65531 - a slot that
    /// names nothing, so every attempt to scroll something you are WEARING would refuse, and
    /// wearing it is the only way to scroll it from that window.
    #[test]
    fn a_worn_slot_is_negative_not_a_huge_positive() {
        let body = [0, 0, 0, 0, 1, 0, 1, 0, 0xfb, 0xff, 0];
        let r = parse_item_upgrade(&body).unwrap();
        assert_eq!(r.dst_slot, -5);
        assert!(r.dst_slot < 0, "the sign is what says 'worn'");
        // A bag slot is positive and must stay so.
        let body = [0, 0, 0, 0, 1, 0, 1, 0, 0x05, 0x00, 0];
        assert_eq!(parse_item_upgrade(&body).unwrap().dst_slot, 5);
    }

    /// A short body decodes to nothing rather than to a default - a default here would scroll
    /// whatever happens to be in slot 0.
    #[test]
    fn a_short_body_is_refused_rather_than_defaulted() {
        for n in 0..ITEM_UPGRADE_REQUEST_LEN {
            assert_eq!(parse_item_upgrade(&[0u8; ITEM_UPGRADE_REQUEST_LEN][..n]), None, "{n} bytes");
        }
        assert!(parse_item_upgrade(&[0u8; ITEM_UPGRADE_REQUEST_LEN]).is_some());
    }
}
