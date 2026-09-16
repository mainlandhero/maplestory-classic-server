//! `0x0114` - **using a Cash-tab item**, the slot coupons and the AP/SP reset scrolls.
//!
//! # It was unknown for exactly one turn, and one click settled it
//!
//! Cash items ask the *server* to act: their own `spec` is a `script` name and an `npc`, with
//! `info/notConsume = 1`, so the client applies nothing itself. Which packet carried that was
//! **not established** - `0x010E`'s ten id ranges (`research/summon-sacks-2026-09-09.md` §2)
//! do not include `568xxxx`, and no archived run had a cash-item use in it. Guessing an
//! opcode is how this project has previously moved a client into a state nobody has read, so
//! the launch plan asked for one capture instead.
//!
//! The owner, 2026-09-09, using an Equip Tab 5-slot Coupon:
//!
//! ```text
//! 02:02:17.218 <- 0x0114 UNKNOWN, 10 byte body dd6b3601030080ab5600
//! 02:02:17.218 -> 0x007C StatChanged: UNLOCK ONLY. 0x0114 is not handled ...
//! ```
//!
//! `0x0056ab80` is **5680000** and the slot is **3**, which is exactly where the log shows the
//! coupon was given. Three attempts in that run, all answered with the unlock and nothing
//! else.
//!
//! # The body is the summoning sack's shape, and that is worth saying
//!
//! `u32 tick, u16 slot, u32 itemId` - ten bytes, identical to `0x0111`. It is **not** the
//! fourteen-byte `0x010E` shape, so a parser borrowed from `useitem.rs` and its `len() < 14`
//! guard would reject every one of these.
//!
//! It is already in [`crate::dropmoney::LATCHING_REQUESTS`], so an unanswered one costs not
//! one coupon but every later inventory action in the session.

/// Client -> server: "I used this item in my Cash tab."
pub const CLIENT_USE_CASH_ITEM: u16 = 0x0114;

/// **`0x0116` - the AP and SP Reset Scrolls.** Same ten-byte body as `0x0114`, different
/// opcode, and the difference cost the owner a launch.
///
/// The owner, 2026-09-10: *"I just tried using the AP Reset Scroll and the SP Reset Scroll, it did
/// not work and it did not take the item."* `world.log` has both presses, and neither is a
/// `0x0114`:
///
/// ```text
/// 02:59:52.956 <- 0x0116 UNKNOWN, 10 byte body 998891060400f40e4d00
///                                              tick     slot 4  0x4d0ef4 = 5050100 AP Reset
/// 02:59:55.907 <- 0x0116 UNKNOWN, 10 byte body 15949106050079124d00
///                                              tick     slot 5  0x4d1279 = 5051001 SP Reset
/// ```
///
/// The reset arms were written against `0x0114` on 2026-09-09 from the coupon capture and
/// never fired, because the client routes these two items through their own builder. Neither
/// item has a `spec` in `Item/Cash/0505.img` (just `cash 1`, and `reqLevel 10` on the AP one),
/// so the client sends the bare use and the server decides what a reset means - which here is
/// the same full refund `!resetap` / `!resetsp` do. [`parse_use_cash_item`] reads it.
pub const CLIENT_USE_STAT_RESET_ITEM: u16 = 0x0116;

/// Body length. **Ten, like `0x0111`; not fourteen, like `0x010E`.**
pub const USE_CASH_ITEM_BODY_LEN: usize = 10;

/// A decoded [`CLIENT_USE_CASH_ITEM`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCashItem {
    /// `FUN_1429e3ef0()`. Nothing reads it.
    pub tick: u32,
    /// The item's slot in the **Cash** tab, 1-based.
    pub slot: u16,
    /// The item id the client believes is in that slot. Checked against the slot rather than
    /// trusted, for the same reason the sack's is.
    pub item_id: u32,
    /// **The pet the item is for**, as its `petLockerSN` - present on the pet-targeted items.
    ///
    /// The owner's run of 2026-09-15, `world-ch0.log` 02:56:50..02:57:46: the four pet skill items
    /// (`5190000..3`) and the Pet Name Tag (`5170000`) all arrive on `0x0116` with **eight more
    /// bytes** after the ten - `46 4b 4c 00 d7 00 00 00` = `pet_serial(215, 5000006)` byte for
    /// byte, the serial this server put in the pet's Cash item and in its `0x0277`. **[L]**
    /// `None` on the ten-byte form the reset scrolls send.
    pub pet_serial: Option<u64>,
    /// **The new name**, on the Pet Name Tag only: a `u16`-prefixed string after the serial -
    /// `05 00 44 75 6d 6d 79` = "Dummy" in the same capture. **[L]** `None` otherwise.
    pub text: Option<String>,
}

/// Decode a [`CLIENT_USE_CASH_ITEM`] body. `None` when it is shorter than the client sends.
/// The optional tail (a pet serial, then a string) is read when the bytes are there and
/// ignored when they are not - a truncated string is `None`, not a refusal of the whole item.
pub fn parse_use_cash_item(body: &[u8]) -> Option<UseCashItem> {
    if body.len() < USE_CASH_ITEM_BODY_LEN {
        return None;
    }
    let pet_serial = body
        .get(10..18)
        .map(|b| u64::from_le_bytes(b.try_into().expect("eight bytes")));
    let text = body.get(18..20).and_then(|l| {
        let n = usize::from(u16::from_le_bytes([l[0], l[1]]));
        body.get(20..20 + n).map(|t| String::from_utf8_lossy(t).into_owned())
    });
    Some(UseCashItem {
        tick: u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        slot: u16::from_le_bytes([body[4], body[5]]),
        item_id: u32::from_le_bytes([body[6], body[7], body[8], body[9]]),
        pet_serial,
        text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two pet-targeted shapes of `0x0116`, byte for byte from `world-ch0.log`
    /// 2026-09-15: an Auto HP Potion Skill (`5190000`) from Cash slot 2, and a Pet Name Tag
    /// (`5170000`) from slot 6 renaming the pet "Dummy". Both carry the pet's serial,
    /// `(215 << 32) | 5000006`, after the ten bytes the reset scrolls stop at.
    #[test]
    fn the_pet_skill_item_and_the_name_tag_carry_the_pets_serial_and_the_tag_a_name() {
        let skill = hex("f98e4e20020070314f00464b4c00d7000000");
        let r = parse_use_cash_item(&skill).unwrap();
        assert_eq!((r.slot, r.item_id), (2, 5_190_000));
        assert_eq!(r.pet_serial, Some((215u64 << 32) | 5_000_006), "the Husky's serial, as the server minted it");
        assert_eq!(r.text, None);

        let tag = hex("393f4f20060050e34e00464b4c00d7000000050044756d6d79");
        let r = parse_use_cash_item(&tag).unwrap();
        assert_eq!((r.slot, r.item_id), (6, 5_170_000));
        assert_eq!(r.pet_serial, Some((215u64 << 32) | 5_000_006));
        assert_eq!(r.text.as_deref(), Some("Dummy"));

        // A ten-byte reset-scroll body has neither, and a string cut short is None.
        let r = parse_use_cash_item(&skill[..10]).unwrap();
        assert_eq!((r.pet_serial, r.text), (None, None));
        let r = parse_use_cash_item(&tag[..23]).unwrap();
        assert_eq!(r.text, None);
    }

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len() / 2).map(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap()).collect()
    }

    /// **The owner's own packet**, byte for byte out of `world.log`, decoded to what they did: an
    /// Equip Tab 5-slot Coupon out of Cash slot 3.
    ///
    /// The capture is the fixture because it is the only thing that can disagree with a
    /// field-offset table. It also cross-checks against a *different* line of the same log -
    /// *"Giving Cobalt 1x Storage Room 5-slot Coupon (5680000) -> Cash tab, slot 3"* - so the
    /// slot and the id are each confirmed by something other than this decode.
    #[test]
    fn the_captured_coupon_use_decodes_to_the_slot_it_was_given_to() {
        // 02:02:17.218 <- 0x0114 UNKNOWN, 10 byte body dd6b3601030080ab5600
        let body = [0xdd, 0x6b, 0x36, 0x01, 0x03, 0x00, 0x80, 0xab, 0x56, 0x00];
        let r = parse_use_cash_item(&body).expect("ten bytes is what the client sends");
        assert_eq!(r.tick, 0x0136_6bdd);
        assert_eq!(r.slot, 3, "the Cash slot the log says it was given to");
        assert_eq!(r.item_id, 5_680_000, "Storage Room 5-slot Coupon");
    }

    /// **Ten, not fourteen.** `0x010E`'s parser has a `len() < 14` guard, and borrowing it
    /// here would reject every real request - with the symptom being "the coupon does
    /// nothing", indistinguishable from the unhandled opcode this replaced.
    #[test]
    fn ten_bytes_is_enough_and_nine_is_not() {
        assert_eq!(USE_CASH_ITEM_BODY_LEN, 10);
        assert!(parse_use_cash_item(&[0u8; 10]).is_some());
        for n in 0..USE_CASH_ITEM_BODY_LEN {
            assert_eq!(parse_use_cash_item(&[0u8; 10][..n]), None, "{n} bytes");
        }
        // Longer still parses: the length is a minimum, and refusing a trailing byte would be
        // a guess about a client only ever seen sending ten.
        assert!(parse_use_cash_item(&[0u8; 14]).is_some());
    }
}
