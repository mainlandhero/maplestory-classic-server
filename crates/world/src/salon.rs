//! **The hair salon: Mystery and Signature Hair Coupons at Henesys' two NPCs.**
//!
//! The owner, 2026-09-18: *"Coupons offer different hair styles depending on the location they are
//! used at. In Henesys: Dr. Feeble takes mystery ones (randomly chosen VIP faces), Denma the
//! Owner takes the signature ones (player choice REG faces)."* - with two screenshots of the
//! COT rotation's Henesys REG and VIP hair lists - then *"Males should only be picked out of
//! the male hair pool. Females only being offered the female ones."* and *"If the clients
//! clicked on the NPC without having the required cash items, the dialogue should direct
//! them with the item link icon of the required item and ask them to purchase it in the
//! Cash Shop."*
//!
//! ("faces" in that first message is a slip: the lists are hair ids, the coupons are the two
//! Cash-tab hair coupons, and the follow-up says hair pools. This is hair.)
//!
//! # The two coupons, the two NPCs, the two lists
//!
//! | NPC | template | map | coupon | how | list |
//! |---|---|---|---|---|---|
//! | Denma the Owner | 213 | 10001043 Henesys Plastic Surgery | 5150100 Signature Hair Coupon | the player picks | Henesys **REG** |
//! | Dr. Feeble | 214 | 10001043 | 5150000 Mystery Hair Coupon | a random pick | Henesys **VIP** |
//!
//! Both NPCs stand in `10001043` (`gm-handbook/npcs.txt`), which the client calls Henesys
//! Plastic Surgery; the Hair Salon next door (`10001044`) holds Natalie and Brittany, who do
//! nothing yet. The lists are the COT rotation the owner supplied, split by the gender the site
//! marks on each style; every one of the 25 base ids is in this client's own hair data with
//! all eight colours (`gm-handbook/beauty.txt`). The client's own `BeautyPreview.img` lists
//! for these coupons are different and broader - they are the generic previews, not a
//! salon's stock - and are deliberately not used.
//!
//! # What a pick does
//!
//! The style is a BASE id (colour digit 0). The player keeps their current hair **colour**:
//! `new = base + (current % 10)`, every listed base having colours 0..7 - so a colour digit
//! 8 (60 of the classic bases go to 8) falls back to 0 rather than to an id that does not
//! draw. Then, in the order the beauty coupons established (`session/beautycoupon.rs`): the
//! look is written, the coupon leaves the Cash tab, the player's own avatar is redrawn by a
//! `0x007C` with the HAIR bit, and the other clients are told.
//!
//! # The box
//!
//! Denma's choice is the client's own "pick a look" script box, message type `0x0a`
//! (`net::script::npc_avatar`), which draws the player wearing each style. Dr. Feeble asks a
//! yes/no first, because a random pick spends the coupon. Neither has been on a screen.

use crate::config::Config;

/// Denma the Owner - the Signature (choice) side.
pub const DENMA: u32 = 213;
/// Dr. Feeble - the Mystery (random) side.
pub const DR_FEEBLE: u32 = 214;
/// Henesys Plastic Surgery, where both stand.
pub const HENESYS_SALON_MAP: u32 = 10_001_043;

/// Signature Hair Coupon - the player chooses from the REG list.
pub const SIGNATURE_COUPON: u32 = 5_150_100;
/// Mystery Hair Coupon - a random style from the VIP list.
pub const MYSTERY_COUPON: u32 = 5_150_000;

/// The conversation path Denma's avatar box is parked at.
pub const CHOICE_PATH: &str = "salon.choice";
/// The conversation path Dr. Feeble's yes/no is parked at.
pub const MYSTERY_PATH: &str = "salon.mystery";

/// Henesys REG, male: Metro, Line Scratch, Mane, Shaggy Wax, Cabana Boy, Dragon Layered.
pub const HENESYS_REG_MALE: &[u32] = &[30_050, 30_170, 30_180, 30_210, 30_330, 30_380];
/// Henesys REG, female: Monica, Miru, Angelica, Lori, Rose, Swooshy Ponytail.
pub const HENESYS_REG_FEMALE: &[u32] = &[31_110, 31_120, 31_150, 31_160, 31_230, 31_360];
/// Henesys VIP, male: Rockstar, Catalyst, Military Buzzcut, Fantasy, Topknot, Wind, Tribal Buzz.
pub const HENESYS_VIP_MALE: &[u32] = &[30_040, 30_060, 30_080, 30_100, 30_140, 30_200, 30_400];
/// Henesys VIP, female: Rockstar Hair, Stella, Perfect Stranger, Pigtails, Roxy, Boyish.
pub const HENESYS_VIP_FEMALE: &[u32] = &[31_050, 31_070, 31_210, 31_270, 31_320, 31_400];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    /// Denma: the player picks from the REG list.
    Signature,
    /// Dr. Feeble: a random style from the VIP list.
    Mystery,
}

impl Service {
    pub fn coupon(self) -> u32 {
        match self {
            Service::Signature => SIGNATURE_COUPON,
            Service::Mystery => MYSTERY_COUPON,
        }
    }

    pub fn npc(self) -> u32 {
        match self {
            Service::Signature => DENMA,
            Service::Mystery => DR_FEEBLE,
        }
    }

    /// The pool for a character of `gender` (0 male, 1 female - the record's byte).
    pub fn styles(self, gender: u8) -> &'static [u32] {
        match (self, gender) {
            (Service::Signature, 0) => HENESYS_REG_MALE,
            (Service::Signature, _) => HENESYS_REG_FEMALE,
            (Service::Mystery, 0) => HENESYS_VIP_MALE,
            (Service::Mystery, _) => HENESYS_VIP_FEMALE,
        }
    }
}

/// Which service an NPC click is, if the NPC is one of the two and the player is in the
/// salon. A template number alone is not enough: `gm-handbook/npcs.txt` places these two
/// only in `10001043` today, but the map is the thing the owner's rule is about ("depending on
/// the location they are used at"), so it is checked.
pub fn service_for(npc_template: u32, map_id: u32) -> Option<Service> {
    if map_id != HENESYS_SALON_MAP {
        return None;
    }
    match npc_template {
        DENMA => Some(Service::Signature),
        DR_FEEBLE => Some(Service::Mystery),
        _ => None,
    }
}

/// The id the player ends up wearing: `base` in their current colour when that colour
/// exists for it, `base` itself otherwise.
pub fn with_current_colour(base: u32, current_hair: u32, config: &Config) -> u32 {
    let colour = current_hair % 10;
    let candidate = base + colour;
    if colour != 0 && config.hair_exists(candidate) {
        candidate
    } else {
        base
    }
}

/// A random index into `pool` from a raw roll. Pure so the pick is a unit test.
pub fn pick(pool: &[u32], roll: u64) -> Option<u32> {
    if pool.is_empty() {
        return None;
    }
    Some(pool[(roll % pool.len() as u64) as usize])
}

/// The line for a player who has no coupon: the item's icon and name as a link (`#i..#`,
/// `#t..#` - the tokens the scroll NPC already uses), and where to buy it.
pub fn no_coupon_line(service: Service) -> String {
    let coupon = service.coupon();
    let (what, how) = match service {
        Service::Signature => ("a Signature Hair Coupon", "pick any style on my board"),
        Service::Mystery => ("a Mystery Hair Coupon", "get a surprise from my VIP list"),
    };
    format!(
        "Looking for a new look? Bring me {what} and I'll {how}.\r\n\r\n#i{coupon}# #b#t{coupon}##k\r\n\r\nYou can buy one in the #bCash Shop#k."
    )
}

/// Denma's prompt over the avatar box.
pub fn choice_prompt() -> &'static str {
    "Welcome! With your Signature Hair Coupon you can pick any of these styles. Your hair colour stays as it is. Which one will it be?"
}

/// Dr. Feeble's yes/no before spending the coupon.
pub fn mystery_prompt() -> &'static str {
    "Feeling lucky? Your Mystery Hair Coupon gets you one of my VIP styles - I pick, you wear it. Your hair colour stays as it is. Shall I go ahead?"
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every listed base id is one this client can draw, with the colour a player keeps.
    /// Read from `gm-handbook/beauty.txt`; skipped loudly without it.
    #[test]
    fn every_listed_style_exists_in_the_client_with_all_colours() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../gm-handbook/beauty.txt");
        let Ok(text) = std::fs::read_to_string(path) else {
            eprintln!("skipped: {path} is not present (python tools/dump_beauty.py)");
            return;
        };
        let mut in_hair = false;
        let mut bases = std::collections::HashMap::new();
        for line in text.lines() {
            if line.starts_with("[hair]") {
                in_hair = true;
                continue;
            }
            if line.starts_with('[') {
                in_hair = false;
            }
            if !in_hair || !line.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                continue;
            }
            let cols: Vec<&str> = line.split(", ").collect();
            bases.insert(cols[0].parse::<u32>().unwrap(), (cols[1].to_string(), cols[2].to_string()));
        }
        for (pool, gender) in [
            (HENESYS_REG_MALE, "male"),
            (HENESYS_VIP_MALE, "male"),
            (HENESYS_REG_FEMALE, "female"),
            (HENESYS_VIP_FEMALE, "female"),
        ] {
            for id in pool {
                let (g, colours) = bases.get(id).unwrap_or_else(|| panic!("{id} is not a hair base in this client"));
                assert_eq!(g, gender, "{id}");
                assert!(colours.starts_with("0-7"), "{id} colours {colours}");
            }
        }
    }

    #[test]
    fn the_two_npcs_serve_only_in_the_salon_and_pools_split_by_gender() {
        assert_eq!(service_for(DENMA, HENESYS_SALON_MAP), Some(Service::Signature));
        assert_eq!(service_for(DR_FEEBLE, HENESYS_SALON_MAP), Some(Service::Mystery));
        assert_eq!(service_for(DENMA, 10_001_044), None, "not in the Hair Salon next door");
        assert_eq!(service_for(215, HENESYS_SALON_MAP), None, "Natalie is not one of the two");
        assert_eq!(Service::Signature.styles(0), HENESYS_REG_MALE);
        assert_eq!(Service::Signature.styles(1), HENESYS_REG_FEMALE);
        assert_eq!(Service::Mystery.styles(0), HENESYS_VIP_MALE);
        assert_eq!(Service::Mystery.styles(1), HENESYS_VIP_FEMALE);
        assert!(HENESYS_REG_MALE.iter().all(|id| (30_000..31_000).contains(id)));
        assert!(HENESYS_REG_FEMALE.iter().all(|id| (31_000..32_000).contains(id)));
        assert!(HENESYS_VIP_MALE.iter().all(|id| (30_000..31_000).contains(id)));
        assert!(HENESYS_VIP_FEMALE.iter().all(|id| (31_000..32_000).contains(id)));
        assert_eq!(Service::Signature.coupon(), 5_150_100);
        assert_eq!(Service::Mystery.coupon(), 5_150_000);
    }

    #[test]
    fn a_random_pick_covers_the_whole_pool_and_nothing_outside_it() {
        let seen: std::collections::HashSet<u32> = (0..200u64).filter_map(|r| pick(HENESYS_VIP_FEMALE, r)).collect();
        assert_eq!(seen.len(), HENESYS_VIP_FEMALE.len());
        assert!(seen.iter().all(|id| HENESYS_VIP_FEMALE.contains(id)));
        assert_eq!(pick(&[], 5), None);
    }

    #[test]
    fn the_no_coupon_line_links_the_coupon_and_names_the_cash_shop() {
        let line = no_coupon_line(Service::Mystery);
        assert!(line.contains("#i5150000#"), "{line}");
        assert!(line.contains("#t5150000#"), "{line}");
        assert!(line.contains("Cash Shop"), "{line}");
        assert!(no_coupon_line(Service::Signature).contains("#i5150100#"));
    }
}
