//! Pet Food: the request, and the numbers the owner set.
//!
//! The owner, 2026-09-15: *"Pets should decrease their fullness by 1 every 5 minutes. Using a pet
//! food should recover the current active pet's fullness by 30 and their closeness by 1."*
//!
//! # The request is `0x0112`, and the client's own item-use switch says so **[L]**
//!
//! Nothing had ever captured a pet food being used, so the opcode was read off the client.
//! `FUN_1428af6d0`, the item-use dispatcher, tests the item id by category and picks a
//! builder per range; the arm at `0x1428b022f`:
//!
//! ```text
//! 1428b022f  lea eax, [rcx - 0x205940]      ; itemId - 2120000
//! 1428b0235  cmp eax, 0x2710                ; < 10000  -> 2120000..2129999, the pet foods
//! 1428b023a  jae  next arm                  ; (next: 0x227c20 = 2260000, mount food -> 0x0113)
//! 1428b0246  call FUN_142ccb350             ; the 0x0112 builder
//! ```
//!
//! and `FUN_142ccb350` writes `CTOR, u32, u16, u32, SEND` (`tools/encodes.py 0x142ccb350`) -
//! the tick, the Use-tab slot and the item id, the same three fields `0x010E` opens with. The
//! neighbours agree with the reference server's order (use item, cancel effect, summon sack,
//! **pet food**, mount food, cash item = `0x010E, 0x010F, 0x0111, 0x0112, 0x0113, 0x0114`
//! here), which is corroboration rather than the evidence.
//!
//! # What a feed does - `crate::petlevel` has the closeness table
//!
//! `+30` fullness (capped at 100) and `+1` closeness, the owner's numbers. The wiki page they pasted
//! adds the two edges this module carries as constants: feeding a pet that is already full is
//! an **overfeed**, and every overfeed after the first costs a closeness; and a pet whose
//! fullness reaches 0 **goes home** and loses a closeness. Lowered closeness never lowers the
//! level.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials; a feed is on the say-so of whoever holds it.

use crate::packet::PacketReader;

/// Client -> server: a Use-tab item in the pet-food range was double-clicked.
pub const CLIENT_USE_PET_FOOD: u16 = 0x0112;

/// The item-id range the client sends through [`CLIENT_USE_PET_FOOD`]: `0x1428b022f`.
pub const PET_FOOD_IDS: std::ops::Range<u32> = 2_120_000..2_130_000;

/// Whether an item id is a pet food, by the client's own test.
pub fn is_pet_food(item_id: u32) -> bool {
    PET_FOOD_IDS.contains(&item_id)
}

/// Fullness one feed restores. The owner: *"recover the current active pet's fullness by 30"*.
pub const PET_FOOD_FULLNESS: u8 = 30;

/// Closeness one feed earns. The owner: *"and their closeness by 1"*.
pub const PET_FOOD_CLOSENESS: u16 = 1;

/// A pet is full at this. The wire field is a `u8` (`net::bag`, `1403045cc`).
pub const PET_FULLNESS_MAX: u8 = 100;

/// How often a summoned pet loses one fullness. The owner: *"decrease their fullness by 1 every 5
/// minutes"*.
pub const PET_HUNGER_INTERVAL_MS: u64 = 5 * 60 * 1000;

/// A decoded [`CLIENT_USE_PET_FOOD`]: `u32 tick, u16 slot, u32 itemId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsePetFood {
    pub tick: u32,
    /// The Use-tab slot, 1-based.
    pub slot: u16,
    /// The item id the client believes is there - checked against the slot, never trusted.
    pub item_id: u32,
}

/// Decode the ten-byte body. `None` when it is short.
pub fn parse_use_pet_food(body: &[u8]) -> Option<UsePetFood> {
    let mut r = PacketReader::new(body);
    let tick = r.u32().ok()?;
    let slot = r.u16().ok()?;
    let item_id = r.u32().ok()?;
    Some(UsePetFood { tick, slot, item_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_range_is_the_clients_own_test_and_the_body_is_tick_slot_item() {
        assert!(is_pet_food(2_120_000), "Pet Food");
        assert!(is_pet_food(2_129_999));
        assert!(!is_pet_food(2_130_000));
        assert!(!is_pet_food(2_260_000), "mount food goes through 0x0113");
        let mut b = 0x2050_8e0au32.to_le_bytes().to_vec();
        b.extend_from_slice(&3u16.to_le_bytes());
        b.extend_from_slice(&2_120_000u32.to_le_bytes());
        assert_eq!(parse_use_pet_food(&b), Some(UsePetFood { tick: 0x2050_8e0a, slot: 3, item_id: 2_120_000 }));
        assert_eq!(parse_use_pet_food(&b[..9]), None);
        assert_eq!(PET_FOOD_FULLNESS, 30);
        assert_eq!(PET_FOOD_CLOSENESS, 1);
        assert_eq!(PET_HUNGER_INTERVAL_MS, 300_000);
    }
}
