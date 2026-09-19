//! **The hair salons: styles from the owners, colours from the assistants.**
//!
//! The owner, 2026-09-18, in four messages, the last correcting the NPCs: *"the Hair Salon Owners
//! will take both the VIP mystery (random) or REG Signature Coupons. The player get to choose
//! which one they want to use if they have both. The dialogue should list out all available
//! coupons for use, or ask the player to purchase one from the Cash Shop if there isn't one
//! detected in the inventory. The Hair Salon assistants will now deal with Hair Color. Hair
//! color has two coupons as well, mystery and signature. Signature REG coupons lets users
//! choose, while mystery rolls a random one from the list with equal chances. The hair
//! colors function the same across the different towns."* Earlier: *"Males should only be
//! picked out of the male hair pool. Females only being offered the female ones."*, the
//! Cash Shop pointer *"with the item link icon of the required item"*, and *"If the user
//! comes in with Black Hair, they will be changed to a Black Hair Equivalent version of the
//! resulting hair style"* - the style keeps the colour, and the colour keeps the style.
//!
//! # Who takes what
//!
//! | salon | map | owner (styles) | assistant (colours) |
//! |---|---|---|---|
//! | Henesys Hair Salon | 10001044 | Natalie 215 | Brittany 216 |
//! | Kerning City Hair Salon | 10003005 | Don Giovanni 413 | Andre 414 |
//!
//! `gm-handbook/npcs.txt` places each pair on exactly that map; the plastic surgeons next
//! door in Henesys (Denma, Dr. Feeble, `10001043`) are not salon NPCs - an earlier version of
//! this file had them, from a slip in the brief.
//!
//! | coupon | id | at | what |
//! |---|---|---|---|
//! | Signature Hair Coupon | 5150100 | owner | the player picks from the salon's REG list |
//! | Mystery Hair Coupon | 5150000 | owner | a random style from the salon's VIP list |
//! | Signature Color Coupon | 5151100 | assistant | the player picks one of the eight colours |
//! | Mystery Hair Color Coupon | 5151000 | assistant | a random colour, each of the eight equally |
//!
//! The style lists are the COT rotation the owner supplied, per salon and per gender; every one
//! of the 51 base ids is in this client's own hair data with all eight colours
//! (`gm-handbook/beauty.txt`; a test reads it). The eight colours are one palette for every
//! style - the id's last digit, `0` black through `7` brown - so the colour coupons need no
//! list of their own and work the same in both towns.
//!
//! # The dialogue
//!
//! An owner or an assistant lists the coupons the player holds as a type-6 menu, one line
//! per kind, each with the item's icon; with none, a Say links both coupons and names the
//! Cash Shop. A Signature pick opens the client's own "pick a look" box (message type `0x0a`,
//! `net::script::npc_avatar`): the styles for the player's gender, or the player's own style
//! in all eight colours. A Mystery line spends the coupon on the roll at once - the line
//! says so, and picking it is the consent. Applying a look is the beauty-coupon sequence:
//! write it, spend the coupon, one `0x007C` with the HAIR bit, the field told.

use crate::config::Config;

/// Natalie - the Henesys Hair Salon's owner.
pub const NATALIE: u32 = 215;
/// Brittany - the Henesys Hair Salon's assistant, "in charge of dyeing hair".
pub const BRITTANY: u32 = 216;
/// Henesys Hair Salon.
pub const HENESYS_SALON_MAP: u32 = 10_001_044;
/// Don Giovanni - the Kerning City Hair Salon's owner.
pub const DON_GIOVANNI: u32 = 413;
/// Andre - the Kerning City Hair Salon's assistant.
pub const ANDRE: u32 = 414;
/// Kerning City Hair Salon.
pub const KERNING_SALON_MAP: u32 = 10_003_005;

pub const SIGNATURE_HAIR_COUPON: u32 = 5_150_100;
pub const MYSTERY_HAIR_COUPON: u32 = 5_150_000;
pub const SIGNATURE_COLOR_COUPON: u32 = 5_151_100;
pub const MYSTERY_COLOR_COUPON: u32 = 5_151_000;

/// The conversation path of an owner's or assistant's coupon menu.
pub const MENU_PATH: &str = "salon.menu";
/// The conversation path of the pick-a-look box, styles or colours.
pub const CHOICE_PATH: &str = "salon.choice";

/// The eight colours, by the id's last digit.
pub const COLOURS: [&str; 8] = ["Black", "Red", "Orange", "Blonde", "Green", "Blue", "Purple", "Brown"];

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

/// What an NPC does: the owner's styles or the assistant's colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Desk {
    Styles,
    Colours,
}

/// Which of a desk's two coupons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// REG - the player picks.
    Signature,
    /// VIP - a random pick.
    Mystery,
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

    /// The desk an NPC of this salon sits at.
    pub fn desk_of(self, npc_template: u32) -> Option<Desk> {
        match (self, npc_template) {
            (Salon::Henesys, NATALIE) | (Salon::Kerning, DON_GIOVANNI) => Some(Desk::Styles),
            (Salon::Henesys, BRITTANY) | (Salon::Kerning, ANDRE) => Some(Desk::Colours),
            _ => None,
        }
    }

    /// The style pool this salon offers for `tier` to a character of `gender` (0 male, 1
    /// female - the record's byte).
    pub fn styles(self, tier: Tier, gender: u8) -> &'static [u32] {
        match (self, tier, gender) {
            (Salon::Henesys, Tier::Signature, 0) => HENESYS_REG_MALE,
            (Salon::Henesys, Tier::Signature, _) => HENESYS_REG_FEMALE,
            (Salon::Henesys, Tier::Mystery, 0) => HENESYS_VIP_MALE,
            (Salon::Henesys, Tier::Mystery, _) => HENESYS_VIP_FEMALE,
            (Salon::Kerning, Tier::Signature, 0) => KERNING_REG_MALE,
            (Salon::Kerning, Tier::Signature, _) => KERNING_REG_FEMALE,
            (Salon::Kerning, Tier::Mystery, 0) => KERNING_VIP_MALE,
            (Salon::Kerning, Tier::Mystery, _) => KERNING_VIP_FEMALE,
        }
    }
}

impl Desk {
    /// The coupon this desk takes for `tier`.
    pub fn coupon(self, tier: Tier) -> u32 {
        match (self, tier) {
            (Desk::Styles, Tier::Signature) => SIGNATURE_HAIR_COUPON,
            (Desk::Styles, Tier::Mystery) => MYSTERY_HAIR_COUPON,
            (Desk::Colours, Tier::Signature) => SIGNATURE_COLOR_COUPON,
            (Desk::Colours, Tier::Mystery) => MYSTERY_COLOR_COUPON,
        }
    }

    /// The menu's line for `tier`, said as what picking it does.
    pub fn menu_line(self, tier: Tier) -> &'static str {
        match (self, tier) {
            (Desk::Styles, Tier::Signature) => "pick a style from my board",
            (Desk::Styles, Tier::Mystery) => "a surprise VIP style - I choose, you wear it",
            (Desk::Colours, Tier::Signature) => "pick a colour",
            (Desk::Colours, Tier::Mystery) => "a surprise colour - any of the eight, I roll it",
        }
    }
}

/// Which salon and desk an NPC click is, if the NPC is one of the four and the player is in
/// its salon. The map is checked because it is the thing the owner's rule is about ("depending
/// on the location they are used at"), not only the template.
pub fn desk_for(npc_template: u32, map_id: u32) -> Option<(Salon, Desk)> {
    let salon = Salon::of_map(map_id)?;
    let desk = salon.desk_of(npc_template)?;
    Some((salon, desk))
}

/// The menu's `#L` numbers, fixed so an answer can be read back without the menu: 0 is the
/// Signature line, 1 the Mystery line. A line the player does not hold the coupon for is
/// simply not in the text, and an answer naming it is refused.
pub const MENU_SIGNATURE: u32 = 0;
pub const MENU_MYSTERY: u32 = 1;

/// The coupon menu for a desk: one line per coupon the player holds, in the client's own
/// `#d#L%d# %s#l#k` line format with the item's icon and name. `None` when they hold neither.
pub fn menu_text(desk: Desk, has_signature: bool, has_mystery: bool) -> Option<String> {
    if !has_signature && !has_mystery {
        return None;
    }
    let mut lines = vec![match desk {
        Desk::Styles => "Which coupon would you like to use today?",
        Desk::Colours => "A new colour, then? Which coupon would you like to use?",
    }
    .to_string()];
    for (tier, held, sel) in [(Tier::Signature, has_signature, MENU_SIGNATURE), (Tier::Mystery, has_mystery, MENU_MYSTERY)] {
        if held {
            let c = desk.coupon(tier);
            lines.push(format!("#d#L{sel}##i{c}# #t{c}# - {}#l#k", desk.menu_line(tier)));
        }
    }
    Some(lines.join("\r\n"))
}

/// The tier a menu answer names, given what the menu offered.
pub fn tier_of_selection(selection: u32, has_signature: bool, has_mystery: bool) -> Option<Tier> {
    match selection {
        MENU_SIGNATURE if has_signature => Some(Tier::Signature),
        MENU_MYSTERY if has_mystery => Some(Tier::Mystery),
        _ => None,
    }
}

/// The line for a player with neither coupon: both linked (`#i..#`, `#t..#` - the tokens the
/// scroll NPC already uses), and where to buy them.
pub fn no_coupon_line(desk: Desk) -> String {
    let (sig, mys) = (desk.coupon(Tier::Signature), desk.coupon(Tier::Mystery));
    let what = match desk {
        Desk::Styles => "a new style",
        Desk::Colours => "a new colour",
    };
    format!(
        "Looking for {what}? Bring me one of these and I'll take care of you.\r\n\r\n#i{sig}# #b#t{sig}##k - you choose\r\n#i{mys}# #b#t{mys}##k - a surprise\r\n\r\nYou can buy either in the #bCash Shop#k."
    )
}

/// The prompt over the pick-a-look box.
pub fn choice_prompt(desk: Desk) -> &'static str {
    match desk {
        Desk::Styles => "Pick any of these styles. Your hair colour stays as it is. Which one will it be?",
        Desk::Colours => "Pick a colour. Your style stays as it is. Which one will it be?",
    }
}

/// The id the player ends up wearing after a STYLE change: `base` in their current colour
/// when that colour exists for it, `base` itself otherwise.
pub fn with_current_colour(base: u32, current_hair: u32, config: &Config) -> u32 {
    let colour = current_hair % 10;
    let candidate = base + colour;
    if colour != 0 && config.hair_exists(candidate) {
        candidate
    } else {
        base
    }
}

/// The player's current style (its base id, colour digit 0).
pub fn base_of(hair: u32) -> u32 {
    hair - hair % 10
}

/// The colour variants of the player's current style that this client draws, in colour
/// order - what the assistant's pick box shows. Empty when the base itself is unknown to
/// the hair table, so a bare table refuses rather than offers ids that do not draw.
pub fn colour_variants(current_hair: u32, config: &Config) -> Vec<u32> {
    let base = base_of(current_hair);
    (0..COLOURS.len() as u32).map(|c| base + c).filter(|id| config.hair_exists(*id)).collect()
}

/// A random element of `pool` from a raw roll. Pure so the pick is a unit test.
pub fn pick(pool: &[u32], roll: u64) -> Option<u32> {
    if pool.is_empty() {
        return None;
    }
    Some(pool[(roll % pool.len() as u64) as usize])
}

/// The colour's name for a notice.
pub fn colour_name(hair: u32) -> &'static str {
    COLOURS.get((hair % 10) as usize).copied().unwrap_or("?")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every listed base id is one this client can draw, with all eight colours. Read from
    /// `gm-handbook/beauty.txt`; skipped loudly without it.
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

    /// Owners take styles, assistants colours, each pair only in its own salon; the pools
    /// split by salon, tier and gender; the plastic surgeons are nobody here.
    #[test]
    fn owners_take_styles_assistants_take_colours_each_in_their_own_salon() {
        assert_eq!(desk_for(NATALIE, HENESYS_SALON_MAP), Some((Salon::Henesys, Desk::Styles)));
        assert_eq!(desk_for(BRITTANY, HENESYS_SALON_MAP), Some((Salon::Henesys, Desk::Colours)));
        assert_eq!(desk_for(DON_GIOVANNI, KERNING_SALON_MAP), Some((Salon::Kerning, Desk::Styles)));
        assert_eq!(desk_for(ANDRE, KERNING_SALON_MAP), Some((Salon::Kerning, Desk::Colours)));
        assert_eq!(desk_for(NATALIE, KERNING_SALON_MAP), None, "a Henesys NPC is not a Kerning one");
        assert_eq!(desk_for(213, 10_001_043), None, "Denma, the plastic surgeon, is not a salon NPC");
        assert_eq!(desk_for(214, HENESYS_SALON_MAP), None, "nor Dr. Feeble, wherever they stood");
        assert_eq!(Salon::Henesys.styles(Tier::Signature, 0), HENESYS_REG_MALE);
        assert_eq!(Salon::Henesys.styles(Tier::Signature, 1), HENESYS_REG_FEMALE);
        assert_eq!(Salon::Henesys.styles(Tier::Mystery, 0), HENESYS_VIP_MALE);
        assert_eq!(Salon::Henesys.styles(Tier::Mystery, 1), HENESYS_VIP_FEMALE);
        assert_eq!(Salon::Kerning.styles(Tier::Signature, 0), KERNING_REG_MALE);
        assert_eq!(Salon::Kerning.styles(Tier::Signature, 1), KERNING_REG_FEMALE);
        assert_eq!(Salon::Kerning.styles(Tier::Mystery, 0), KERNING_VIP_MALE);
        assert_eq!(Salon::Kerning.styles(Tier::Mystery, 1), KERNING_VIP_FEMALE);
        for pool in [HENESYS_REG_MALE, HENESYS_VIP_MALE, KERNING_REG_MALE, KERNING_VIP_MALE] {
            assert!(pool.iter().all(|id| (30_000..31_000).contains(id)));
        }
        for pool in [HENESYS_REG_FEMALE, HENESYS_VIP_FEMALE, KERNING_REG_FEMALE, KERNING_VIP_FEMALE] {
            assert!(pool.iter().all(|id| (31_000..32_000).contains(id)));
        }
        assert_eq!(Desk::Styles.coupon(Tier::Signature), 5_150_100);
        assert_eq!(Desk::Styles.coupon(Tier::Mystery), 5_150_000);
        assert_eq!(Desk::Colours.coupon(Tier::Signature), 5_151_100);
        assert_eq!(Desk::Colours.coupon(Tier::Mystery), 5_151_000);
    }

    /// The menu lists exactly the coupons held, with icons, under fixed #L numbers; an
    /// answer naming an unheld line is refused; neither held is no menu at all.
    #[test]
    fn the_menu_lists_what_is_held_and_an_answer_is_read_back_against_it() {
        assert_eq!(menu_text(Desk::Styles, false, false), None);
        let both = menu_text(Desk::Styles, true, true).unwrap();
        assert!(both.contains("#L0##i5150100# #t5150100#") && both.contains("#L1##i5150000# #t5150000#"), "{both}");
        let only_mystery = menu_text(Desk::Colours, false, true).unwrap();
        assert!(only_mystery.contains("#L1##i5151000#") && !only_mystery.contains("#L0#"), "{only_mystery}");
        assert_eq!(tier_of_selection(0, true, true), Some(Tier::Signature));
        assert_eq!(tier_of_selection(1, true, true), Some(Tier::Mystery));
        assert_eq!(tier_of_selection(0, false, true), None, "the Signature line was not offered");
        assert_eq!(tier_of_selection(7, true, true), None);
        let line = no_coupon_line(Desk::Colours);
        assert!(line.contains("#i5151100#") && line.contains("#i5151000#") && line.contains("Cash Shop"), "{line}");
    }

    #[test]
    fn colours_are_the_last_digit_and_a_roll_covers_all_eight_equally() {
        assert_eq!(base_of(31_233), 31_230);
        assert_eq!(colour_name(31_233), "Blonde");
        assert_eq!(colour_name(31_235), "Blue");
        assert_eq!(colour_name(30_050), "Black");
        assert_eq!(colour_name(31_237), "Brown");
        let pool: Vec<u32> = (0..8).map(|c| 31_230 + c).collect();
        let mut counts = [0u32; 8];
        for r in 0..800u64 {
            counts[(pick(&pool, r).unwrap() % 10) as usize] += 1;
        }
        assert!(counts.iter().all(|&n| n == 100), "{counts:?}");
        let seen: std::collections::HashSet<u32> = (0..200u64).filter_map(|r| pick(HENESYS_VIP_FEMALE, r)).collect();
        assert_eq!(seen.len(), HENESYS_VIP_FEMALE.len());
        assert_eq!(pick(&[], 5), None);
    }
}
