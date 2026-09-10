//! **Which gender an equip is for**, and why a character can be wearing one it cannot show.
//!
//! The owner, 2026-09-09, with a character-select screenshot: *"Cobalt does not seem to be wearing
//! a top, but does have a top equipped when logging into the game."*
//!
//! The server was sending it. `avatar_look` puts `05 95 de 0f 00` on the wire - slot 5, item
//! `1040021` - and the client has `Coat/01040021.img`, so both ends have everything. The
//! client still would not draw it, and it is right not to: **`1040021` is a MALE top and
//! Cobalt is female.**
//!
//! # The rule is the client's own, read out of its data rather than remembered
//!
//! `Etc.wz/MakeCharInfo.img` partitions the starter items by gender, and the partition is the
//! fourth digit:
//!
//! ```text
//!   male   shirt 1040001 1040002 1040003      female shirt 1041001 .. 1041004
//!   male   pants 1060001 1060002              female pants 1061001 1061002
//!   shoes  1072000 .. 1072003                 both lists, identically
//!   weapon 1302000 1312000 1322000            both lists, identically
//! ```
//!
//! So `(id / 1000) % 10` is **0 male, 1 female, 2 either** - and the two lists agreeing
//! exactly on the `xxx2xxx` rows is what makes 2 mean "either" rather than "a third gender".
//!
//! # It explains all five of Cobalt's items, with no exceptions
//!
//! ```text
//!   1002997 Nemi Hat                    digit 2  unisex  drawn
//!   1040021 Blue Sergeant               digit 0  MALE    NOT drawn   <- the report
//!   1062999 Wizet Plain Suit Pants      digit 2  unisex  drawn
//!   1072999 Wizet Plain Shoes           digit 2  unisex  drawn
//!   1322999 Wizet Secret Agent Suitcase digit 2  unisex  drawn
//! ```
//!
//! Four of four unisex items render and the one gendered item does not. That is a
//! discriminator rather than a coincidence: had the top been unisex and still missing, or had
//! a unisex item also been missing, this reading would be dead.
//!
//! # So the bug is the server's, one step earlier
//!
//! The client is behaving correctly. What should not have happened is the **equip**: nothing
//! checked, so a female character put on a male top and the two ends have disagreed about them
//! appearance ever since. [`may_wear`] is that check.

/// Who an equip is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipGender {
    Male,
    Female,
    /// Either. Shoes, weapons, hats and most accessories.
    Either,
}

/// The gender byte a [`crate::opcode::Character`] carries for a man.
pub const MALE: u8 = 0;
/// And for a woman.
pub const FEMALE: u8 = 1;

/// Which gender `item_id` is for.
///
/// **Anything that is not an equip is [`EquipGender::Either`]**, because this digit only means
/// gender inside the `1xxxxxx` space - `2040021` is a scroll, not a male anything.
pub fn equip_gender(item_id: u32) -> EquipGender {
    if item_id / 1_000_000 != 1 {
        return EquipGender::Either;
    }
    match (item_id / 1_000) % 10 {
        0 => EquipGender::Male,
        1 => EquipGender::Female,
        // **2 is "either", and so is anything else.** Only 0, 1 and 2 occur in this client's
        // equip ids; treating an unexpected digit as a restriction would refuse an item on a
        // guess, and refusing is the direction that loses a player their gear.
        _ => EquipGender::Either,
    }
}

/// May a character of `gender` wear `item_id`?
///
/// `gender` is the character record's own byte - [`MALE`] or [`FEMALE`].
pub fn may_wear(item_id: u32, gender: u8) -> bool {
    match equip_gender(item_id) {
        EquipGender::Either => true,
        EquipGender::Male => gender == MALE,
        EquipGender::Female => gender == FEMALE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Cobalt's five items, and the one that does not render is the one that cannot.**
    #[test]
    fn the_reported_character_is_explained_item_by_item() {
        let cobalt = FEMALE;
        assert!(may_wear(1_002_997, cobalt), "Nemi Hat is unisex");
        assert!(!may_wear(1_040_021, cobalt), "Blue Sergeant is a MALE top - the report");
        assert!(may_wear(1_062_999, cobalt), "Wizet Plain Suit Pants are unisex");
        assert!(may_wear(1_072_999, cobalt), "Wizet Plain Shoes are unisex");
        assert!(may_wear(1_322_999, cobalt), "the suitcase is unisex");
        // And a man could have worn the top, which is what makes this a restriction rather
        // than the item being broken.
        assert!(may_wear(1_040_021, MALE));
    }

    /// The rule against the client's own `MakeCharInfo.img` partition, both directions.
    ///
    /// A test that only checked the male list would pass on a function that called everything
    /// male, so each row is asserted for the gender it belongs to **and** against the other.
    #[test]
    fn the_starter_lists_split_exactly_the_way_the_digit_says() {
        for male_only in [1_040_001, 1_040_002, 1_040_003, 1_060_001, 1_060_002] {
            assert_eq!(equip_gender(male_only), EquipGender::Male, "{male_only}");
            assert!(may_wear(male_only, MALE) && !may_wear(male_only, FEMALE), "{male_only}");
        }
        for female_only in [1_041_001, 1_041_002, 1_041_003, 1_041_004, 1_061_001, 1_061_002] {
            assert_eq!(equip_gender(female_only), EquipGender::Female, "{female_only}");
            assert!(may_wear(female_only, FEMALE) && !may_wear(female_only, MALE), "{female_only}");
        }
        // The rows that appear IDENTICALLY in both lists - which is what proves 2 is "either"
        // rather than a third gender.
        for shared in [1_072_000, 1_072_001, 1_072_002, 1_072_003, 1_302_000, 1_312_000, 1_322_000]
        {
            assert_eq!(equip_gender(shared), EquipGender::Either, "{shared}");
            assert!(may_wear(shared, MALE) && may_wear(shared, FEMALE), "{shared}");
        }
    }

    /// **The digit means nothing outside the equip space.** A scroll is not a male anything,
    /// and reading it as one would refuse `2040021` to every woman on the server.
    #[test]
    fn only_equips_are_gendered() {
        for not_an_equip in [2_040_021, 2_000_000, 4_031_065, 5_680_000, 3_010_005] {
            assert_eq!(equip_gender(not_an_equip), EquipGender::Either, "{not_an_equip}");
            assert!(may_wear(not_an_equip, MALE) && may_wear(not_an_equip, FEMALE));
        }
    }
}
