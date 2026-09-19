//! **The hair salons: Mystery and Signature Hair Coupons at Henesys' and Kerning City's NPCs.**
//!
//! The owner, 2026-09-18: *"Coupons offer different hair styles depending on the location they are
//! used at."* - with the COT rotation's REG and VIP hair lists for Henesys and for Kerning
//! City - then *"Males should only be picked out of the male hair pool. Females only being
//! offered the female ones."*, *"If the clients clicked on the NPC without having the
//! required cash items, the dialogue should direct them with the item link icon of the
//! required item and ask them to purchase it in the Cash Shop."*, and the corrected
//! assignment: *"In Henesys: Dr. Feeble takes the signature ones (player choice REG faces).
//! Denma the Owner takes mystery ones (randomly chosen VIP faces)"* and *"Don Giovanni takes
//! the Mystery VIP coupons that randomly changes the hair. Andre takes the REG Signature
//! coupons that allows users to choose"*. ("faces" is a slip: the lists are hair ids, the
//! coupons are the two Cash-tab hair coupons, the follow-up says hair pools. This is hair.)
//!
//! # Two salons, two coupons each
//!
//! | salon | map | Signature (player picks, REG) | Mystery (random, VIP) |
//! |---|---|---|---|
//! | Henesys Plastic Surgery | 10001043 | Dr. Feeble 214 | Denma the Owner 213 |
//! | Kerning City Hair Salon | 10003005 | Andre 414 | Don Giovanni 413 |
//!
//! `gm-handbook/npcs.txt` places each pair on exactly that map. The lists are the COT
//! rotation the owner supplied, split by the gender the site marks on each style; every one of
//! the 51 base ids is in this client's own hair data with all eight colours
//! (`gm-handbook/beauty.txt`; a test checks it). The client's own `BeautyPreview.img` lists
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
//! The Signature NPC's choice is the client's own "pick a look" script box, message type `0x0a`
//! (`net::script::npc_avatar`), which draws the player wearing each style. The Mystery NPC asks a
//! yes/no first, because a random pick spends the coupon. Neither has been on a screen.

use crate::config::Config;

/// Denma the Owner - Henesys, the Mystery (random VIP) side.
pub const DENMA: u32 = 213;
/// Dr. Feeble - Henesys, the Signature (player picks, REG) side.
pub const DR_FEEBLE: u32 = 214;
/// Henesys Plastic Surgery, where both stand.
pub const HENESYS_SALON_MAP: u32 = 10_001_043;
/// Don Giovanni - Kerning City, the Mystery side.
pub const DON_GIOVANNI: u32 = 413;
/// Andre - Kerning City, the Signature side.
pub const ANDRE: u32 = 414;
/// Kerning City Hair Salon, where both stand.
pub const KERNING_SALON_MAP: u32 = 10_003_005;

/// Signature Hair Coupon - the player chooses from the REG list.
pub const SIGNATURE_COUPON: u32 = 5_150_100;
/// Mystery Hair Coupon - a random style from the VIP list.
pub const MYSTERY_COUPON: u32 = 5_150_000;

/// The conversation path the Signature NPC's avatar box is parked at.
pub const CHOICE_PATH: &str = "salon.choice";
/// The conversation path the Mystery NPC's yes/no is parked at.
pub const MYSTERY_PATH: &str = "salon.mystery";

/// Henesys REG, male: Metro, Line Scratch, Mane, Shaggy Wax, Cabana Boy, Dragon Layered.
pub const HENESYS_REG_MALE: &[u32] = &[30_050, 30_170, 30_180, 30_210, 30_330, 30_380];
/// Henesys REG, female: Monica, Miru, Angelica, Lori, Rose, Swooshy Ponytail.
pub const HENESYS_REG_FEMALE: &[u32] = &[31_110, 31_120, 31_150, 31_160, 31_230, 31_360];
/// Henesys VIP, male: Rockstar, Catalyst, Military Buzzcut, Fantasy, Topknot, Wind, Tribal Buzz.
pub const HENESYS_VIP_MALE: &[u32] = &[30_040, 30_060, 30_080, 30_100, 30_140, 30_200, 30_400];
/// Henesys VIP, female: Rockstar Hair, Stella, Perfect Stranger, Pigtails, Roxy, Boyish.
pub const HENESYS_VIP_FEMALE: &[u32] = &[31_050, 31_070, 31_210, 31_270, 31_320, 31_400];
/// Kerning REG, male: Antagonist, Medium Cornrows, Bowl Cut, Chestnut, Mohecan Shaggy 'Do,
/// Astro, Shaggy Dragon.
pub const KERNING_REG_MALE: &[u32] = &[30_130, 30_150, 30_190, 30_240, 30_280, 30_350, 30_370];
/// Kerning REG, female: Cutie Hair, Francesca, Parted Pomp, Jolie, Bowl Cut, Chantelle.
pub const KERNING_REG_FEMALE: &[u32] = &[31_000, 31_020, 31_080, 31_130, 31_250, 31_300];
/// Kerning VIP, male: Rockstar, All Back, Mo Rawk, Fireball, Bald Spot, Old Man 'Do.
pub const KERNING_VIP_MALE: &[u32] = &[30_040, 30_070, 30_090, 30_110, 30_270, 30_290];
/// Kerning VIP, female: Veronica, Rockstar Hair, Acorn, Pei Pei, Rastafari, Naomi, Rae.
pub const KERNING_VIP_FEMALE: &[u32] = &[31_010, 31_050, 31_060, 31_140, 31_170, 31_290, 31_340];

/// Which salon a map is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Salon {
    Henesys,
    Kerning,
}

impl Salon {
    pub fn of_map(map_id: u32) -> Option<Salon> {
        match map_id {
            HENESYS_SALON_MAP => Some(Salon::Henesys),
            KERNING_SALON_MAP => Some(Salon::Kerning),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Salon::Henesys => "Henesys",
            Salon::Kerning => "Kerning City",
        }
    }

    /// The pool this salon offers for `service` to a character of `gender` (0 male, 1
    /// female - the record's byte).
    pub fn styles(self, service: Service, gender: u8) -> &'static [u32] {
        match (self, service, gender) {
            (Salon::Henesys, Service::Signature, 0) => HENESYS_REG_MALE,
            (Salon::Henesys, Service::Signature, _) => HENESYS_REG_FEMALE,
            (Salon::Henesys, Service::Mystery, 0) => HENESYS_VIP_MALE,
            (Salon::Henesys, Service::Mystery, _) => HENESYS_VIP_FEMALE,
            (Salon::Kerning, Service::Signature, 0) => KERNING_REG_MALE,
            (Salon::Kerning, Service::Signature, _) => KERNING_REG_FEMALE,
            (Salon::Kerning, Service::Mystery, 0) => KERNING_VIP_MALE,
            (Salon::Kerning, Service::Mystery, _) => KERNING_VIP_FEMALE,
        }
    }

    /// Which service an NPC of this salon gives.
    pub fn service_of(self, npc_template: u32) -> Option<Service> {
        match (self, npc_template) {
            (Salon::Henesys, DR_FEEBLE) | (Salon::Kerning, ANDRE) => Some(Service::Signature),
            (Salon::Henesys, DENMA) | (Salon::Kerning, DON_GIOVANNI) => Some(Service::Mystery),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    /// The player picks from the salon's REG list.
    Signature,
    /// A random style from the salon's VIP list.
    Mystery,
}

impl Service {
    pub fn coupon(self) -> u32 {
        match self {
            Service::Signature => SIGNATURE_COUPON,
            Service::Mystery => MYSTERY_COUPON,
        }
    }
}

/// Which salon and service an NPC click is, if the NPC is one of the four and the player is
/// in that NPC's salon. The map is checked because it is the thing the owner's rule is about
/// ("depending on the location they are used at"), not only the template.
pub fn service_for(npc_template: u32, map_id: u32) -> Option<(Salon, Service)> {
    let salon = Salon::of_map(map_id)?;
    let service = salon.service_of(npc_template)?;
    Some((salon, service))
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

/// The Signature NPC's prompt over the avatar box.
pub fn choice_prompt() -> &'static str {
    "Welcome! With your Signature Hair Coupon you can pick any of these styles. Your hair colour stays as it is. Which one will it be?"
}

/// The Mystery NPC's yes/no before spending the coupon.
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
            (KERNING_REG_MALE, "male"),
            (KERNING_VIP_MALE, "male"),
            (KERNING_REG_FEMALE, "female"),
            (KERNING_VIP_FEMALE, "female"),
        ] {
            for id in pool {
                let (g, colours) = bases.get(id).unwrap_or_else(|| panic!("{id} is not a hair base in this client"));
                assert_eq!(g, gender, "{id}");
                assert!(colours.starts_with("0-7"), "{id} colours {colours}");
            }
        }
    }

    /// The corrected assignment (the owner, "I take back the previous instructions"): Dr. Feeble
    /// and Andre take the Signature coupon, Denma and Don Giovanni the Mystery one; each NPC
    /// only in its own salon.
    #[test]
    fn the_four_npcs_serve_only_in_their_salon_and_pools_split_by_gender() {
        assert_eq!(service_for(DR_FEEBLE, HENESYS_SALON_MAP), Some((Salon::Henesys, Service::Signature)));
        assert_eq!(service_for(DENMA, HENESYS_SALON_MAP), Some((Salon::Henesys, Service::Mystery)));
        assert_eq!(service_for(ANDRE, KERNING_SALON_MAP), Some((Salon::Kerning, Service::Signature)));
        assert_eq!(service_for(DON_GIOVANNI, KERNING_SALON_MAP), Some((Salon::Kerning, Service::Mystery)));
        assert_eq!(service_for(DENMA, 10_001_044), None, "not in the Hair Salon next door");
        assert_eq!(service_for(DENMA, KERNING_SALON_MAP), None, "a Henesys NPC is not a Kerning one");
        assert_eq!(service_for(215, HENESYS_SALON_MAP), None, "Natalie is not one of the four");
        assert_eq!(Salon::Henesys.styles(Service::Signature, 0), HENESYS_REG_MALE);
        assert_eq!(Salon::Henesys.styles(Service::Signature, 1), HENESYS_REG_FEMALE);
        assert_eq!(Salon::Henesys.styles(Service::Mystery, 0), HENESYS_VIP_MALE);
        assert_eq!(Salon::Henesys.styles(Service::Mystery, 1), HENESYS_VIP_FEMALE);
        assert_eq!(Salon::Kerning.styles(Service::Signature, 0), KERNING_REG_MALE);
        assert_eq!(Salon::Kerning.styles(Service::Signature, 1), KERNING_REG_FEMALE);
        assert_eq!(Salon::Kerning.styles(Service::Mystery, 0), KERNING_VIP_MALE);
        assert_eq!(Salon::Kerning.styles(Service::Mystery, 1), KERNING_VIP_FEMALE);
        for pool in [HENESYS_REG_MALE, HENESYS_VIP_MALE, KERNING_REG_MALE, KERNING_VIP_MALE] {
            assert!(pool.iter().all(|id| (30_000..31_000).contains(id)));
        }
        for pool in [HENESYS_REG_FEMALE, HENESYS_VIP_FEMALE, KERNING_REG_FEMALE, KERNING_VIP_FEMALE] {
            assert!(pool.iter().all(|id| (31_000..32_000).contains(id)));
        }
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
