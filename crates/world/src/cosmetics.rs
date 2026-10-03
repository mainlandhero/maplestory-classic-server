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
///
/// **Each coupon gives its style in its DEFAULT colour** (the owner, 2026-10-02: *"Using the
/// collaboration hair/face coupon will change both hair and face to the default intended
/// color"*). Since the same day the collaboration colours are real art (`tools/collab_recolor.py`,
/// installed by `tools/backport_install.py`): Nexon's own pixels sit at the art's colour slot
/// and every other slot is a recolour. Hair colour is the id's last digit and eye colour its
/// hundreds digit; the slot names are the client's own (`MapleStory.exe` tables at
/// `0x143a49900` hair, `0x143a49940` eyes). The WZ's own `spec/cosmetic` still names the base
/// (colour 0) id - the dialog previews that; the server applies the default.
pub const COUPONS: [(u32, Kind, u32); 15] = [
    (2_543_137, Kind::Hair, 42_543), // Frieren Hair - Yellow (her white art; the owner's choice)
    (2_543_138, Kind::Hair, 42_563), // Frieren Hair (Ringlets) - Yellow
    (2_543_139, Kind::Hair, 42_553), // Frieren Hair (Sleep) - Yellow
    (2_543_140, Kind::Hair, 42_576), // Fern Hair - Violet
    (2_543_141, Kind::Hair, 42_581), // Stark Hair - Red
    (2_543_142, Kind::Hair, 42_595), // Himmel Hair - Blue
    (2_543_143, Kind::Hair, 42_604), // Übel Hair - Green
    (2_890_907, Kind::Face, 22_535), // Frieren Face - Emerald (her teal eyes)
    (2_890_908, Kind::Face, 22_636), // Fern Face - Violet
    (2_890_909, Kind::Face, 22_237), // Stark Face - Red
    (2_890_910, Kind::Face, 22_138), // Himmel Face - Blue
    (2_890_911, Kind::Face, 22_639), // Übel Face - Violet
    (2_890_912, Kind::Face, 22_640), // Aura Face - Violet (the owner: "purple"; blue art, recoloured)
    (2_890_913, Kind::Face, 22_741), // Linie Face - Amethyst (violet art, recoloured)
    (2_890_914, Kind::Face, 22_442), // Lügner Face - Hazel (blue art, recoloured)
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
        assert_eq!(for_coupon(2_543_143), Some((Kind::Hair, 42_604)), "Übel Hair Coupon - Green");
        assert_eq!(for_coupon(2_890_911), Some((Kind::Face, 22_639)), "Übel Face Coupon - Violet");
        assert_eq!(for_coupon(2_000_000), None, "a potion is not a coupon");
        assert_eq!(for_coupon(2_543_136), None, "one below the first hair coupon");
    }

    /// The defaults are the owner's, slot by slot (2026-10-02), and every one is a colour of its
    /// own style: a hair keeps its base (last digit cleared), a face its style (hundreds cleared).
    #[test]
    fn each_coupon_gives_its_style_in_the_owners_default_colour() {
        let slot = |id: u32, kind: Kind| match kind {
            Kind::Hair => (id - id % 10, id % 10),
            Kind::Face => (id - (id / 100 % 10) * 100, id / 100 % 10),
        };
        let want = [
            (42_540, 3), (42_560, 3), (42_550, 3), (42_570, 6), (42_580, 1), (42_590, 5), (42_600, 4),
            (22_035, 5), (22_036, 6), (22_037, 2), (22_038, 1), (22_039, 6), (22_040, 6), (22_041, 7), (22_042, 4),
        ];
        for (&(_, kind, id), &(base, default)) in COUPONS.iter().zip(want.iter()) {
            assert_eq!(slot(id, kind), (base, default), "{id}");
        }
    }
}
