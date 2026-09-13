//! **Which hairstyle or face a beauty coupon applies** - the `spec/cosmetic` of each coupon.
//!
//! Read out of `client-patched/Data/Item/Consume/Consume_000.wz` on 2026-09-12 (`0254.img`
//! for hair, `0289.img` for faces) with `wz-dump cat`, after the Signature Style backport
//! installed them. The client previews the cosmetic from this same node; the server applies
//! it. Fifteen rows, one per coupon the backport ships.
//!
//! A table rather than a WZ read at runtime, for the same reason the rest of `world` loads
//! generated files: the server does not parse WZ. If the backport ever renumbers a face (see
//! `STATUS.md`, 2026-09-12: the classic client's own faces are `20000..21825` and the
//! backported `22035..22042` do not open the dialog), this table changes with it and the
//! test below says so.

/// What a coupon changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Hair,
    Face,
}

/// `(coupon item id, what it changes, the cosmetic id it applies)`.
pub const COUPONS: [(u32, Kind, u32); 15] = [
    (2_543_137, Kind::Hair, 42_540), // Frieren Hair
    (2_543_138, Kind::Hair, 42_560), // Frieren Hair (Ringlets)
    (2_543_139, Kind::Hair, 42_550), // Frieren Hair (Sleep)
    (2_543_140, Kind::Hair, 42_570), // Fern Hair
    (2_543_141, Kind::Hair, 42_580), // Stark Hair
    (2_543_142, Kind::Hair, 42_590), // Himmel Hair
    (2_543_143, Kind::Hair, 42_600), // Übel Hair
    (2_890_907, Kind::Face, 22_035), // Frieren Face
    (2_890_908, Kind::Face, 22_036), // Fern Face
    (2_890_909, Kind::Face, 22_037), // Stark Face
    (2_890_910, Kind::Face, 22_038), // Himmel Face
    (2_890_911, Kind::Face, 22_039), // Übel Face
    (2_890_912, Kind::Face, 22_040), // Aura Face
    (2_890_913, Kind::Face, 22_041), // Linie Face
    (2_890_914, Kind::Face, 22_042), // Lügner Face
];

/// What `item_id` applies, if it is a beauty coupon this server knows.
pub fn for_coupon(item_id: u32) -> Option<(Kind, u32)> {
    COUPONS.iter().find(|(c, _, _)| *c == item_id).map(|&(_, k, id)| (k, id))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every hair coupon names a hair id and every face coupon a face id, by the id spaces
    /// the client uses (`3xxxx`/`4xxxx` hair, `2xxxx` face); the ids are distinct; and the
    /// two the owner pressed resolve to what their tooltips say.
    #[test]
    fn coupons_resolve_to_the_right_kind_of_id_and_nothing_collides() {
        let mut seen = std::collections::HashSet::new();
        for &(coupon, kind, id) in &COUPONS {
            assert!(seen.insert(coupon), "{coupon} listed twice");
            match kind {
                Kind::Hair => assert!((30_000..100_000).contains(&id), "{coupon} -> {id}"),
                Kind::Face => assert!((20_000..30_000).contains(&id), "{coupon} -> {id}"),
            }
            assert_eq!(for_coupon(coupon), Some((kind, id)));
        }
        assert_eq!(for_coupon(2_543_143), Some((Kind::Hair, 42_600)), "Übel Hair Coupon");
        assert_eq!(for_coupon(2_890_911), Some((Kind::Face, 22_039)), "Übel Face Coupon");
        assert_eq!(for_coupon(2_000_000), None, "a potion is not a coupon");
        assert_eq!(for_coupon(2_543_136), None, "one below the first hair coupon");
    }
}
