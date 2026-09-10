//! The five **5-slot coupons**, `5680000`..`5680004`.
//!
//! The owner, 2026-09-09: *"Also make sure that these items when used, also increase the
//! appropriate maximum slots up by 5"* - and then, from the item's own tooltip, *"Also seems
//! like I was wrong, inventory can be only expanded up to 150 slots."* See [`MAX_SLOTS`].
//!
//! ```text
//!   5680000  Storage Room 5-slot Coupon
//!   5680001  Equip Tab 5-slot Coupon
//!   5680002  Use Tab 5-slot Coupon
//!   5680003  Set Up Tab 5-Slot Coupon
//!   5680004  Etc Tab 5-Slot Coupon
//! ```
//!
//! **This module is the rule and nothing else** - which tab, how many, and the ceiling. It
//! touches no store and no packet, so all of it is testable without a database, and the
//! session joins it to both.
//!
//! # The tab widens on screen, and that took a second decode
//!
//! The first version of this wrote the row and told the player to change maps, because
//! nothing could change the count live. **`0x007B` InventoryGrow** was decoded afterwards -
//! while chasing the station clock, of all things - and `net::inventory::inventory_grow`
//! carries the working. Storage has no equivalent packet and still needs the window reopened,
//! which the notice says only for storage.
//!
//! # The trigger was unknown for exactly one turn, and then the owner captured it
//!
//! These are Cash items whose own `spec` is `{"script": "cash_5680000", "npc": 9010000}` with
//! `info/notConsume = 1`, so the client applies nothing itself - it asks the server to act.
//! `0x010E`'s ten id ranges do not include `568xxxx`, so it was not that opcode, and no
//! archived run had a cash-item use in it. Rather than guess, the plan asked for one click.
//!
//! It is **`0x0114`**, and the body is the same ten-byte shape as the summoning sack:
//!
//! ```text
//! 02:02:17.218 <- 0x0114 UNKNOWN, 10 byte body dd6b3601030080ab5600
//!                                              tick     slot 3  0x56ab80 = 5680000
//! ```
//!
//! Three attempts in that run, all answered with the latch unlock and nothing else.
//! `net::cashitem` decodes it and `session::cashitem` handles it.

/// **The ceiling, and it is the client's own number rather than mine.**
///
/// The owner first said 200 and then corrected it from the item's own tooltip: *"Also seems like I
/// was wrong, inventory can be only expanded up to 150 slots."* The coupon says so itself -
///
/// > *"Double-click to expand your Equip tab by 5 slots. You can have up to 150 slots."*
///
/// which is much better evidence than a policy number, because it is what the player is told
/// and therefore what they will hold the server to.
///
/// Applied as a clamp rather than a refusal: a coupon used at 148 takes the tab to 150 and is
/// spent, which is the reading that cannot leave a player holding a coupon they can never use.
pub const MAX_SLOTS: u16 = 150;

/// How many slots one coupon adds, before the clamp.
pub const SLOTS_PER_COUPON: u16 = 5;

/// What one coupon widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotCoupon {
    /// The account's storage trunk, **not** a character's bag - `Store::set_storage_slots`.
    Storage,
    /// One of the four bag tabs.
    Tab(store::InventoryType),
}

impl SlotCoupon {
    /// The five ids, in the order the item table lists them.
    pub const ALL: [(u32, SlotCoupon); 5] = [
        (5_680_000, SlotCoupon::Storage),
        (5_680_001, SlotCoupon::Tab(store::InventoryType::Equip)),
        (5_680_002, SlotCoupon::Tab(store::InventoryType::Use)),
        (5_680_003, SlotCoupon::Tab(store::InventoryType::Setup)),
        (5_680_004, SlotCoupon::Tab(store::InventoryType::Etc)),
    ];

    /// `None` for anything that is not one of the five.
    ///
    /// **A table rather than arithmetic on the id.** The ids are consecutive and the tabs are
    /// not in the store enum's order, so `id - 5680001` as a tab index would be wrong for
    /// three of the four and would look right for the first.
    pub fn for_item(item_id: u32) -> Option<Self> {
        Self::ALL.iter().find(|(id, _)| *id == item_id).map(|(_, c)| *c)
    }

    /// The player-facing name of what it widened.
    pub fn what(self) -> &'static str {
        match self {
            SlotCoupon::Storage => "storage",
            SlotCoupon::Tab(store::InventoryType::Equip) => "Equip tab",
            SlotCoupon::Tab(store::InventoryType::Use) => "Use tab",
            SlotCoupon::Tab(store::InventoryType::Setup) => "Set Up tab",
            SlotCoupon::Tab(store::InventoryType::Etc) => "Etc tab",
            SlotCoupon::Tab(_) => "inventory",
        }
    }
}

/// What a coupon takes `current` to. `None` means it is already at the ceiling.
///
/// **`None` is a refusal and it must reach the player**, because the alternative - clamping
/// silently - spends a coupon and changes nothing, which is the shape of bug `CLAUDE.md`'s
/// Heena section is about. A coupon used at 148 is *not* a refusal: it goes to 150 and is
/// spent, and this returns `Some(150)` for it.
pub fn widened(current: u16) -> Option<u16> {
    if current >= MAX_SLOTS {
        return None;
    }
    Some(current.saturating_add(SLOTS_PER_COUPON).min(MAX_SLOTS))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every id maps to the tab its NAME says, and nothing else maps at all.
    ///
    /// **The mapping is the risky part.** The ids are consecutive, so `id - 5680001` as an
    /// index is the obvious shortcut and it is wrong: `store::InventoryType`'s own order is
    /// Equip, Use, Setup, Etc, Cash, and a coupon that widened the wrong tab would look like
    /// it worked to anyone not counting the tab they were watching.
    #[test]
    fn each_coupon_widens_the_tab_its_name_promises() {
        assert_eq!(SlotCoupon::for_item(5_680_000), Some(SlotCoupon::Storage));
        assert_eq!(
            SlotCoupon::for_item(5_680_001),
            Some(SlotCoupon::Tab(store::InventoryType::Equip))
        );
        assert_eq!(
            SlotCoupon::for_item(5_680_002),
            Some(SlotCoupon::Tab(store::InventoryType::Use))
        );
        assert_eq!(
            SlotCoupon::for_item(5_680_003),
            Some(SlotCoupon::Tab(store::InventoryType::Setup))
        );
        assert_eq!(
            SlotCoupon::for_item(5_680_004),
            Some(SlotCoupon::Tab(store::InventoryType::Etc))
        );
        // The control: neighbours are not coupons.
        assert_eq!(SlotCoupon::for_item(5_680_005), None);
        assert_eq!(SlotCoupon::for_item(5_679_999), None);
        assert_eq!(SlotCoupon::for_item(2_000_000), None);
        // And no two ids share a target, which an off-by-one table would produce.
        let mut targets: Vec<String> =
            SlotCoupon::ALL.iter().map(|(_, c)| format!("{:?}", c)).collect();
        targets.sort();
        targets.dedup();
        assert_eq!(targets.len(), 5, "two coupons widen the same thing");
    }

    /// **+5, and 150 is a ceiling rather than a step.**
    #[test]
    fn a_coupon_adds_five_and_stops_at_the_ceiling() {
        assert_eq!(widened(24), Some(29));
        assert_eq!(widened(100), Some(105));
        assert_eq!(widened(140), Some(145));
        // The partial step: 148 + 5 would be 153, and it must land exactly on the ceiling
        // rather than overshoot or refuse.
        assert_eq!(widened(148), Some(MAX_SLOTS), "a partial step still spends the coupon");
        assert_eq!(widened(149), Some(MAX_SLOTS));
        // At the ceiling it refuses, so the caller can keep the coupon and say why.
        assert_eq!(widened(MAX_SLOTS), None);
        assert_eq!(widened(u16::MAX), None, "and above it, which a bad write could produce");
    }

    /// Repeated use walks to exactly 150 and then stops, rather than oscillating or
    /// overshooting - the property a player actually experiences.
    #[test]
    fn using_coupons_until_they_stop_lands_exactly_on_the_ceiling() {
        let mut slots = 24u16;
        let mut used = 0;
        while let Some(next) = widened(slots) {
            assert!(next > slots, "a coupon that changes nothing must refuse instead");
            slots = next;
            used += 1;
            assert!(used < 100, "it never stopped");
        }
        assert_eq!(slots, MAX_SLOTS);
        // 24 -> 149 in 25 steps of five, then one partial step to 150.
        assert_eq!(used, 26);
    }
}
