//! The five Leaf Point Exchange Coupons: double-click one and the Leaf Points land on the
//! account.
//!
//! The owner, 2026-09-17: *"There are 5 Leaf Point Exchange Coupons ... Can you make it so that
//! when you use one of these items, it gives the player who used them the appropriate amount
//! of Leaf Points in their account?"*
//!
//! # What the client sends, and why it is `0x0114` and not `0x010E`
//!
//! The coupons are `2430004..=2430008` in `Item.wz/Consume/0243.img`, each with
//! `spec/script = consume_243000N` and `spec/npc = 9010000` - scripted consumables, not
//! potions. The Use-tab double-click dispatcher (`FUN_141784fa0`, inventory-type arm 2 at
//! `1417854a6`) walks a chain of item-id predicates; `FUN_1404169b0` (the potion families,
//! `2000000..`, `2010000..`, `2020000..`, `2050000..`, ...) gates `0x010E`, and 243 is not
//! among them. `FUN_140416ea0` returns 1 for `[2430000, 2440000)` and `[2630000, 2640000)`
//! (minus `FUN_140416d60`'s handful of specific exclusions, none of them these five) and
//! gates **`0x0114`** - the same `u32 tick, u16 slot, u32 itemId` body the Cash-tab coupons
//! use (`net::cashitem`). **[L]** from the listing, 2026-09-17; never yet on a wire. The
//! difference from every other `0x0114` this server answers: the slot is in the **Use** tab.
//!
//! # Leaf Points are the cash wallet's `maple_points`
//!
//! `store::cash::CashWallet::maple_points` is the pot the Cash Shop spends
//! (`buy_cash_item`); NX is the other pot and buys nothing. So a coupon is
//! `Store::add_maple_points(account, n)` - the account's, not the character's, which is why
//! the handler needs the claim.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials. A coupon is honoured for whoever holds the
//! connection the bag belongs to.

/// `(item id, Leaf Points)`, the five rows of `gm-handbook/items.txt` named
/// `Leaf Points (N) Exchange Coupon`. **[L]** the ids and names; the amounts are the names.
pub const COUPONS: [(u32, u32); 5] = [
    (2_430_004, 1_000),
    (2_430_005, 5_000),
    (2_430_006, 10_000),
    (2_430_007, 50_000),
    (2_430_008, 100_000),
];

/// The Leaf Points a coupon is worth, or `None` for anything that is not one.
pub fn leaf_points_for(item_id: u32) -> Option<u32> {
    COUPONS.iter().find(|(id, _)| *id == item_id).map(|(_, n)| *n)
}

/// The line the player reads: the amount, and the new balance. Thousands separated the way
/// the coupon's own name writes them.
pub fn received_line(points: u32, balance: u32) -> String {
    format!(
        "You received {} Leaf Points. You now have {} Leaf Points.",
        with_commas(points),
        with_commas(balance)
    )
}

fn with_commas(n: u32) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The five ids and amounts, exactly as `items.txt` names them; nothing else is a coupon.
    #[test]
    fn the_five_coupons_and_only_those() {
        assert_eq!(leaf_points_for(2_430_004), Some(1_000));
        assert_eq!(leaf_points_for(2_430_005), Some(5_000));
        assert_eq!(leaf_points_for(2_430_006), Some(10_000));
        assert_eq!(leaf_points_for(2_430_007), Some(50_000));
        assert_eq!(leaf_points_for(2_430_008), Some(100_000));
        for other in [2_430_003, 2_430_009, 2_000_000, 5_430_004, 0] {
            assert_eq!(leaf_points_for(other), None, "{other}");
        }
        assert!(COUPONS.windows(2).all(|w| w[0].1 < w[1].1), "ascending, so a typo shows");
        assert!(COUPONS.iter().all(|(id, _)| store::InventoryType::for_item(*id) == Some(store::InventoryType::Use)), "all in the Use tab");
    }

    #[test]
    fn the_line_writes_thousands_the_way_the_coupon_name_does() {
        assert_eq!(with_commas(0), "0");
        assert_eq!(with_commas(999), "999");
        assert_eq!(with_commas(1_000), "1,000");
        assert_eq!(with_commas(100_000), "100,000");
        assert_eq!(with_commas(1_234_567), "1,234,567");
        let line = received_line(5_000, 105_500);
        assert_eq!(line, "You received 5,000 Leaf Points. You now have 105,500 Leaf Points.");
        assert!(line.is_ascii());
    }
}
