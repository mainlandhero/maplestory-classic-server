//! **The beauty shops: hair salons and plastic surgeries, owners and assistants.**
//!
//! The owner, 2026-09-18, on the salons: *"the Hair Salon Owners will take both the VIP mystery
//! (random) or REG Signature Coupons. The player get to choose which one they want to use if
//! they have both. The dialogue should list out all available coupons for use, or ask the
//! player to purchase one from the Cash Shop if there isn't one detected in the inventory.
//! The Hair Salon assistants will now deal with Hair Color. Hair color has two coupons as
//! well, mystery and signature. Signature REG coupons lets users choose, while mystery rolls
//! a random one from the list with equal chances. The hair colors function the same across
//! the different towns."* Then, the same day: *"The Plastic Surgeon Owners at Henesys and
//! Kerning will handle the REG (Signature) or VIP (Mystery) face coupons. The assistants will
//! take the Skin Signature or Skin Mystery coupons."* Earlier: *"Males should only be picked
//! out of the male hair pool. Females only being offered the female ones."*, the Cash Shop
//! pointer *"with the item link icon of the required item"*, and *"If the user comes in with
//! Black Hair, they will be changed to a Black Hair Equivalent version of the resulting hair
//! style"* - a style keeps its colour, a colour keeps its style; the same for a face and its
//! eye colour.
//!
//! # Who takes what
//!
//! | shop | map | owner | assistant |
//! |---|---|---|---|
//! | Henesys Hair Salon | 10001044 | Natalie 215 - hair styles | Brittany 216 - hair colours |
//! | Kerning City Hair Salon | 10003005 | Don Giovanni 413 - hair styles | Andre 414 - hair colours |
//! | Henesys Plastic Surgery | 10001043 | Denma the Owner 213 - faces | Dr. Feeble 214 - skins |
//! | Orbis Plastic Surgery | 20000031 | Franz the Owner 1018 - faces | Riza the Assistant 1019 - skins |
//!
//! `gm-handbook/npcs.txt` places each pair on exactly that map. **Kerning City has no
//! plastic surgery in this client** - `gm-handbook/maps.txt` has exactly two, Henesys and
//! Orbis - so the second surgery the owner named is Orbis's, the one that exists.
//!
//! | coupon | id | at | what |
//! |---|---|---|---|
//! | Signature Hair Coupon | 5150100 | salon owner | the player picks from the salon's REG list |
//! | Mystery Hair Coupon | 5150000 | salon owner | a random style from the salon's VIP list |
//! | Signature Color Coupon | 5151100 | salon assistant | the player picks one of the eight colours |
//! | Mystery Hair Color Coupon | 5151000 | salon assistant | a random colour, each of the eight equally |
//! | Signature Face Coupon | 5152200 | surgery owner | the player picks from the REG face list |
//! | Mystery Face Coupon | 5152000 | surgery owner | a random face from the VIP face list |
//! | Signature Skin Color Coupon | 5153000 | surgery assistant | the player picks one of the seven skins |
//! | *Mystery skin* | - | surgery assistant | **no such item in this client** (`beauty.txt [coupons]` has ten, and 5153000 is the only skin one), so the assistant's menu has one line |
//!
//! The hair lists are the COT rotation the owner supplied, per salon and per gender; every base
//! id is in this client's own hair data with all eight colours (`gm-handbook/beauty.txt`; a
//! test reads it). The eight hair colours are one palette for every style - the id's last
//! digit, `0` black through `7` brown. The face lists are the COT "Salon coupons" set, one
//! list for both surgeries; a face's eye colour is its hundreds digit (`20003` at colour 1 is
//! `20103`), nine per style. The skins are the seven this client has art for -
//! `Character/0000200X.img`, X in `0..=6`, rendered on 2026-09-18: light, tan, brown, pale,
//! ashen (blue-grey), white, pink. The COT site's `#10 Pink` and `#11 Warm` and its `#5
//! Green` have no body image here and would draw an invisible player, so they are not
//! offered.
//!
//! # The dialogue
//!
//! An owner or an assistant lists the coupons the player holds as a type-6 menu, one line
//! per kind, each with the item's icon; with none, a Say links the desk's coupons and names
//! the Cash Shop. A Signature pick opens the client's own "pick a look" box (message type
//! `0x0a`, `net::script::npc_avatar`): the client classifies the box by its FIRST candidate
//! id - `id / 10000` in `{3,4,6}` is hair, in `{2,5}` face, and anything under 24000 is a
//! skin (`FUN_142a91f30`, via `FUN_1402538e0` / `FUN_1402538a0` / `FUN_140253930`; the skin
//! test adds 12000 to a small id first) - so a skin box carries the plain skin numbers. A
//! Mystery line spends the coupon on the roll at once - the line says so, and picking it is
//! the consent. Applying a look is the beauty-coupon sequence: write it, spend the coupon,
//! one `0x007C` with the HAIR / FACE / SKIN bit, the field told.

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
/// Denma the Owner - Henesys Plastic Surgery.
pub const DENMA: u32 = 213;
/// Dr. Feeble - Henesys Plastic Surgery's assistant.
pub const DR_FEEBLE: u32 = 214;
/// Henesys Plastic Surgery.
pub const HENESYS_SURGERY_MAP: u32 = 10_001_043;
/// Franz the Owner - Orbis Plastic Surgery.
pub const FRANZ: u32 = 1_018;
/// Riza the Assistant - Orbis Plastic Surgery.
pub const RIZA: u32 = 1_019;
/// Orbis Plastic Surgery.
pub const ORBIS_SURGERY_MAP: u32 = 20_000_031;

pub const SIGNATURE_HAIR_COUPON: u32 = 5_150_100;
pub const MYSTERY_HAIR_COUPON: u32 = 5_150_000;
pub const SIGNATURE_COLOR_COUPON: u32 = 5_151_100;
pub const MYSTERY_COLOR_COUPON: u32 = 5_151_000;
pub const SIGNATURE_FACE_COUPON: u32 = 5_152_200;
pub const MYSTERY_FACE_COUPON: u32 = 5_152_000;
pub const SIGNATURE_SKIN_COUPON: u32 = 5_153_000;

/// The conversation path of an owner's or assistant's coupon menu.
pub const MENU_PATH: &str = "salon.menu";
/// The conversation path of the pick-a-look box, whatever it shows.
pub const CHOICE_PATH: &str = "salon.choice";

/// The eight hair colours, by the id's last digit.
pub const COLOURS: [&str; 8] = ["Black", "Red", "Orange", "Blonde", "Green", "Blue", "Purple", "Brown"];

/// The skins this client draws, `(skin byte, name)`, in the order the box shows them.
pub const SKINS: [(u8, &str); 7] = [(0, "Light"), (1, "Tan"), (2, "Brown"), (3, "Pale"), (4, "Ashen"), (5, "White"), (6, "Pink")];

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

/// Faces REG, male: Dramatic Face, Alert Face, Babyface Pout, Wisdom Glance, Curious Dog,
/// Insomniac Daze, Look of Wonder.
pub const FACE_REG_MALE: &[u32] = &[20_003, 20_005, 20_006, 20_010, 20_012, 20_013, 20_014];
/// Faces REG, female: Babyface Pout, Pucker Up Face, Look of Death, Wisdom Glance,
/// Hypnotized Look, Curious Look.
pub const FACE_REG_FEMALE: &[u32] = &[21_005, 21_006, 21_009, 21_010, 21_011, 21_014];
/// Faces VIP, male: Rebel's Fire, Sad Innocence, Worrisome Glare, Smart Aleck, Wisdom Glance,
/// Cool Guy Gaze.
pub const FACE_VIP_MALE: &[u32] = &[20_004, 20_007, 20_008, 20_009, 20_010, 20_011];
/// Faces VIP, female: Strong Stare, Angel Glow, Dollface Look, Hopeless Gaze, Wisdom Glance,
/// Soul's Window, Wide-eyed Girl.
pub const FACE_VIP_FEMALE: &[u32] = &[21_003, 21_004, 21_007, 21_008, 21_010, 21_012, 21_013];

/// Which shop a map is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shop {
    HenesysSalon,
    KerningSalon,
    HenesysSurgery,
    OrbisSurgery,
}

/// What an NPC does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Desk {
    /// A salon owner: hair styles.
    Styles,
    /// A salon assistant: hair colours.
    Colours,
    /// A surgery owner: faces.
    Faces,
    /// A surgery assistant: skin tones.
    Skins,
}

/// Which of a desk's coupons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// REG - the player picks.
    Signature,
    /// VIP - a random pick.
    Mystery,
}

impl Shop {
    pub fn of_map(map_id: u32) -> Option<Shop> {
        match map_id {
            HENESYS_SALON_MAP => Some(Shop::HenesysSalon),
            KERNING_SALON_MAP => Some(Shop::KerningSalon),
            HENESYS_SURGERY_MAP => Some(Shop::HenesysSurgery),
            ORBIS_SURGERY_MAP => Some(Shop::OrbisSurgery),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Shop::HenesysSalon => "Henesys Hair Salon",
            Shop::KerningSalon => "Kerning City Hair Salon",
            Shop::HenesysSurgery => "Henesys Plastic Surgery",
            Shop::OrbisSurgery => "Orbis Plastic Surgery",
        }
    }

    /// The desk an NPC of this shop sits at.
    pub fn desk_of(self, npc_template: u32) -> Option<Desk> {
        match (self, npc_template) {
            (Shop::HenesysSalon, NATALIE) | (Shop::KerningSalon, DON_GIOVANNI) => Some(Desk::Styles),
            (Shop::HenesysSalon, BRITTANY) | (Shop::KerningSalon, ANDRE) => Some(Desk::Colours),
            (Shop::HenesysSurgery, DENMA) | (Shop::OrbisSurgery, FRANZ) => Some(Desk::Faces),
            (Shop::HenesysSurgery, DR_FEEBLE) | (Shop::OrbisSurgery, RIZA) => Some(Desk::Skins),
            _ => None,
        }
    }

    /// The hair-style pool this salon offers for `tier` to a character of `gender` (0 male,
    /// 1 female - the record's byte). Empty for a surgery.
    pub fn styles(self, tier: Tier, gender: u8) -> &'static [u32] {
        match (self, tier, gender) {
            (Shop::HenesysSalon, Tier::Signature, 0) => HENESYS_REG_MALE,
            (Shop::HenesysSalon, Tier::Signature, _) => HENESYS_REG_FEMALE,
            (Shop::HenesysSalon, Tier::Mystery, 0) => HENESYS_VIP_MALE,
            (Shop::HenesysSalon, Tier::Mystery, _) => HENESYS_VIP_FEMALE,
            (Shop::KerningSalon, Tier::Signature, 0) => KERNING_REG_MALE,
            (Shop::KerningSalon, Tier::Signature, _) => KERNING_REG_FEMALE,
            (Shop::KerningSalon, Tier::Mystery, 0) => KERNING_VIP_MALE,
            (Shop::KerningSalon, Tier::Mystery, _) => KERNING_VIP_FEMALE,
            (Shop::HenesysSurgery | Shop::OrbisSurgery, _, _) => &[],
        }
    }
}

/// **What each beauty coupon can give, for the Cash Shop's preview panel** (`0x05B9`,
/// `net::cashshop::beauty_preview`): `(coupon, [male, female])`. A hair coupon lists every
/// style ANY salon offers for that tier - the preview is one list per coupon and the pools are
/// per salon - in first-seen order without repeats; a face coupon lists the surgery pool, which
/// is the same in both. Base ids: the panel draws each one in the colour the player picks.
/// The owner, 2026-09-26: *"the Mystery Hair and Signature Hair Coupon should show previews."*
pub fn cash_shop_previews() -> Vec<(u32, [Vec<u32>; 2])> {
    let salons = [Shop::HenesysSalon, Shop::KerningSalon];
    let hair = |tier: Tier, gender: u8| {
        let mut out: Vec<u32> = Vec::new();
        for shop in salons {
            for &id in shop.styles(tier, gender) {
                if !out.contains(&id) {
                    out.push(id);
                }
            }
        }
        out
    };
    vec![
        (MYSTERY_HAIR_COUPON, [hair(Tier::Mystery, 0), hair(Tier::Mystery, 1)]),
        (SIGNATURE_HAIR_COUPON, [hair(Tier::Signature, 0), hair(Tier::Signature, 1)]),
        (MYSTERY_FACE_COUPON, [faces(Tier::Mystery, 0).to_vec(), faces(Tier::Mystery, 1).to_vec()]),
        (SIGNATURE_FACE_COUPON, [faces(Tier::Signature, 0).to_vec(), faces(Tier::Signature, 1).to_vec()]),
    ]
}

/// The face pool for `tier` and `gender` - the same in both surgeries.
pub fn faces(tier: Tier, gender: u8) -> &'static [u32] {
    match (tier, gender) {
        (Tier::Signature, 0) => FACE_REG_MALE,
        (Tier::Signature, _) => FACE_REG_FEMALE,
        (Tier::Mystery, 0) => FACE_VIP_MALE,
        (Tier::Mystery, _) => FACE_VIP_FEMALE,
    }
}

impl Desk {
    /// The coupon this desk takes for `tier`; `None` when the client has no such item.
    pub fn coupon(self, tier: Tier) -> Option<u32> {
        match (self, tier) {
            (Desk::Styles, Tier::Signature) => Some(SIGNATURE_HAIR_COUPON),
            (Desk::Styles, Tier::Mystery) => Some(MYSTERY_HAIR_COUPON),
            (Desk::Colours, Tier::Signature) => Some(SIGNATURE_COLOR_COUPON),
            (Desk::Colours, Tier::Mystery) => Some(MYSTERY_COLOR_COUPON),
            (Desk::Faces, Tier::Signature) => Some(SIGNATURE_FACE_COUPON),
            (Desk::Faces, Tier::Mystery) => Some(MYSTERY_FACE_COUPON),
            (Desk::Skins, Tier::Signature) => Some(SIGNATURE_SKIN_COUPON),
            (Desk::Skins, Tier::Mystery) => None,
        }
    }

    /// The menu's line for `tier`, said as what picking it does.
    pub fn menu_line(self, tier: Tier) -> &'static str {
        match (self, tier) {
            (Desk::Styles, Tier::Signature) => "pick a style from my board",
            (Desk::Styles, Tier::Mystery) => "a surprise VIP style - I choose, you wear it",
            (Desk::Colours, Tier::Signature) => "pick a colour",
            (Desk::Colours, Tier::Mystery) => "a surprise colour - any of the eight, I roll it",
            (Desk::Faces, Tier::Signature) => "pick a face from my board",
            (Desk::Faces, Tier::Mystery) => "a surprise VIP face - I choose, you wear it",
            (Desk::Skins, Tier::Signature) => "pick a skin tone",
            (Desk::Skins, Tier::Mystery) => "a surprise skin tone",
        }
    }

    /// What the desk sells, for the no-coupon line.
    fn wares(self) -> &'static str {
        match self {
            Desk::Styles => "a new style",
            Desk::Colours => "a new colour",
            Desk::Faces => "a new face",
            Desk::Skins => "a new skin tone",
        }
    }
}

/// Which shop and desk an NPC click is, if the NPC is one of the eight and the player is in
/// its shop. The map is checked because it is the thing the owner's rule is about ("depending
/// on the location they are used at"), not only the template.
pub fn desk_for(npc_template: u32, map_id: u32) -> Option<(Shop, Desk)> {
    let shop = Shop::of_map(map_id)?;
    let desk = shop.desk_of(npc_template)?;
    Some((shop, desk))
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
        Desk::Styles | Desk::Faces => "Which coupon would you like to use today?",
        Desk::Colours => "A new colour, then? Which coupon would you like to use?",
        Desk::Skins => "A new skin tone, then? Which coupon would you like to use?",
    }
    .to_string()];
    for (tier, held, sel) in [(Tier::Signature, has_signature, MENU_SIGNATURE), (Tier::Mystery, has_mystery, MENU_MYSTERY)] {
        if let (true, Some(c)) = (held, desk.coupon(tier)) {
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

/// The line for a player with none of the desk's coupons: each linked (`#i..#`, `#t..#` -
/// the tokens the scroll NPC already uses), and where to buy them.
pub fn no_coupon_line(desk: Desk) -> String {
    let mut lines = vec![format!("Looking for {}? Bring me one of these and I'll take care of you.", desk.wares()), String::new()];
    if let Some(sig) = desk.coupon(Tier::Signature) {
        lines.push(format!("#i{sig}# #b#t{sig}##k - you choose"));
    }
    if let Some(mys) = desk.coupon(Tier::Mystery) {
        lines.push(format!("#i{mys}# #b#t{mys}##k - a surprise"));
    }
    lines.push(String::new());
    lines.push(match desk.coupon(Tier::Mystery) {
        Some(_) => "You can buy either in the #bCash Shop#k.".to_string(),
        None => "You can buy it in the #bCash Shop#k.".to_string(),
    });
    lines.join("\r\n")
}

/// The prompt over the pick-a-look box.
pub fn choice_prompt(desk: Desk) -> &'static str {
    match desk {
        Desk::Styles => "Pick any of these styles. Your hair colour stays as it is. Which one will it be?",
        Desk::Colours => "Pick a colour. Your style stays as it is. Which one will it be?",
        Desk::Faces => "Pick any of these faces. Your eye colour stays as it is. Which one will it be?",
        Desk::Skins => "Pick a skin tone. Which one will it be?",
    }
}

/// The id the player ends up wearing after a hair STYLE change: `base` in their current
/// colour when that colour exists for it, `base` itself otherwise.
pub fn with_current_colour(base: u32, current_hair: u32, config: &Config) -> u32 {
    let colour = current_hair % 10;
    let candidate = base + colour;
    if colour != 0 && config.hair_exists(candidate) {
        candidate
    } else {
        base
    }
}

/// The same for a face: `base` at the player's current eye colour (the hundreds digit)
/// when that exists, `base` itself otherwise.
pub fn face_with_current_eye_colour(base: u32, current_face: u32, config: &Config) -> u32 {
    let colour = (current_face / 100) % 10;
    let candidate = base + colour * 100;
    if colour != 0 && config.face_exists(candidate) {
        candidate
    } else {
        base
    }
}

/// The player's current hair style (its base id, colour digit 0).
pub fn base_of(hair: u32) -> u32 {
    hair - hair % 10
}

/// The colour variants of the player's current style that this client draws, in colour
/// order - what the salon assistant's pick box shows. Empty when the base itself is unknown
/// to the hair table, so a bare table refuses rather than offers ids that do not draw.
pub fn colour_variants(current_hair: u32, config: &Config) -> Vec<u32> {
    let base = base_of(current_hair);
    (0..COLOURS.len() as u32).map(|c| base + c).filter(|id| config.hair_exists(*id)).collect()
}

/// The skin numbers the surgery assistant's pick box shows - `SKINS` in order. The client
/// reads a candidate under 24000 as a skin, so these go on the wire as they are.
pub fn skin_candidates() -> Vec<u32> {
    SKINS.iter().map(|(s, _)| u32::from(*s)).collect()
}

/// A random element of `pool` from a raw roll. Pure so the pick is a unit test.
pub fn pick(pool: &[u32], roll: u64) -> Option<u32> {
    if pool.is_empty() {
        return None;
    }
    Some(pool[(roll % pool.len() as u64) as usize])
}

/// The hair colour's name for a notice.
pub fn colour_name(hair: u32) -> &'static str {
    COLOURS.get((hair % 10) as usize).copied().unwrap_or("?")
}

/// The skin's name for a notice.
pub fn skin_name(skin: u8) -> &'static str {
    SKINS.iter().find(|(s, _)| *s == skin).map(|(_, n)| *n).unwrap_or("?")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn beauty_rows(section: &str) -> Option<std::collections::HashMap<u32, (String, String)>> {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../gm-handbook/beauty.txt");
        let Ok(text) = std::fs::read_to_string(path) else {
            eprintln!("skipped: {path} is not present (python tools/dump_beauty.py)");
            return None;
        };
        let mut inside = false;
        let mut rows = std::collections::HashMap::new();
        for line in text.lines() {
            if line.starts_with('[') {
                inside = line.starts_with(section);
                continue;
            }
            if !inside || !line.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                continue;
            }
            let cols: Vec<&str> = line.split(", ").collect();
            rows.insert(cols[0].parse::<u32>().unwrap(), (cols[1].to_string(), cols[2].to_string()));
        }
        Some(rows)
    }

    /// **The Cash Shop's preview lists** (`0x05B9`, the owner 2026-09-26): the four coupons, a
    /// non-empty list for BOTH genders each, hair lists in base ids (colour digit 0) that are
    /// the union of the salons' pools for that tier, and every id one this client can draw.
    #[test]
    fn the_cash_shop_previews_are_every_style_each_coupon_can_give() {
        let previews = cash_shop_previews();
        let coupons: Vec<u32> = previews.iter().map(|(c, _)| *c).collect();
        assert_eq!(coupons, vec![MYSTERY_HAIR_COUPON, SIGNATURE_HAIR_COUPON, MYSTERY_FACE_COUPON, SIGNATURE_FACE_COUPON]);
        for (coupon, lists) in &previews {
            for (gender, ids) in lists.iter().enumerate() {
                assert!(!ids.is_empty(), "{coupon} gender {gender}: an empty list is the bug");
                let mut seen = std::collections::HashSet::new();
                assert!(ids.iter().all(|id| seen.insert(*id)), "{coupon} gender {gender}: no repeats");
            }
        }
        for (tier, coupon) in [(Tier::Mystery, MYSTERY_HAIR_COUPON), (Tier::Signature, SIGNATURE_HAIR_COUPON)] {
            let lists = &previews.iter().find(|(c, _)| *c == coupon).unwrap().1;
            for gender in 0..2u8 {
                for shop in [Shop::HenesysSalon, Shop::KerningSalon] {
                    for id in shop.styles(tier, gender) {
                        assert!(lists[gender as usize].contains(id), "{coupon}: {id} from {} is offered", shop.name());
                    }
                }
                assert!(lists[gender as usize].iter().all(|id| id % 10 == 0), "base ids: the panel colours them");
            }
        }
        if let (Some(hair), Some(face)) = (beauty_rows("[hair]"), beauty_rows("[face]")) {
            for (coupon, lists) in &previews {
                for id in lists.iter().flatten() {
                    let known = if *coupon / 1000 == 5150 { hair.contains_key(id) } else { face.contains_key(id) };
                    assert!(known, "{coupon}: {id} has no art in this client");
                }
            }
        }
    }

    /// Every listed hair base is one this client can draw, with all eight colours, and every
    /// listed face with all nine eye colours. Read from `gm-handbook/beauty.txt`; skipped
    /// loudly without it.
    #[test]
    fn every_listed_style_and_face_exists_in_the_client_with_all_colours() {
        let Some(hair) = beauty_rows("[hair]") else { return };
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
                let (g, colours) = hair.get(id).unwrap_or_else(|| panic!("{id} is not a hair base in this client"));
                assert_eq!(g, gender, "{id}");
                assert!(colours.starts_with("0-7"), "{id} colours {colours}");
            }
        }
        let face = beauty_rows("[face]").unwrap();
        for (pool, gender) in [(FACE_REG_MALE, "male"), (FACE_VIP_MALE, "male"), (FACE_REG_FEMALE, "female"), (FACE_VIP_FEMALE, "female")] {
            for id in pool {
                let (g, colours) = face.get(id).unwrap_or_else(|| panic!("{id} is not a face style in this client"));
                assert_eq!(g, gender, "{id}");
                assert_eq!(colours, "0-8", "{id}");
            }
        }
    }

    /// Owners take styles or faces, assistants colours or skins, each pair only in its own
    /// shop; the pools split by shop, tier and gender.
    #[test]
    fn owners_and_assistants_each_in_their_own_shop() {
        assert_eq!(desk_for(NATALIE, HENESYS_SALON_MAP), Some((Shop::HenesysSalon, Desk::Styles)));
        assert_eq!(desk_for(BRITTANY, HENESYS_SALON_MAP), Some((Shop::HenesysSalon, Desk::Colours)));
        assert_eq!(desk_for(DON_GIOVANNI, KERNING_SALON_MAP), Some((Shop::KerningSalon, Desk::Styles)));
        assert_eq!(desk_for(ANDRE, KERNING_SALON_MAP), Some((Shop::KerningSalon, Desk::Colours)));
        assert_eq!(desk_for(DENMA, HENESYS_SURGERY_MAP), Some((Shop::HenesysSurgery, Desk::Faces)));
        assert_eq!(desk_for(DR_FEEBLE, HENESYS_SURGERY_MAP), Some((Shop::HenesysSurgery, Desk::Skins)));
        assert_eq!(desk_for(FRANZ, ORBIS_SURGERY_MAP), Some((Shop::OrbisSurgery, Desk::Faces)));
        assert_eq!(desk_for(RIZA, ORBIS_SURGERY_MAP), Some((Shop::OrbisSurgery, Desk::Skins)));
        assert_eq!(desk_for(NATALIE, KERNING_SALON_MAP), None, "a Henesys NPC is not a Kerning one");
        assert_eq!(desk_for(DENMA, HENESYS_SALON_MAP), None, "the surgeon is not a barber");
        assert_eq!(Shop::HenesysSalon.styles(Tier::Signature, 0), HENESYS_REG_MALE);
        assert_eq!(Shop::HenesysSalon.styles(Tier::Signature, 1), HENESYS_REG_FEMALE);
        assert_eq!(Shop::HenesysSalon.styles(Tier::Mystery, 0), HENESYS_VIP_MALE);
        assert_eq!(Shop::HenesysSalon.styles(Tier::Mystery, 1), HENESYS_VIP_FEMALE);
        assert_eq!(Shop::KerningSalon.styles(Tier::Signature, 0), KERNING_REG_MALE);
        assert_eq!(Shop::KerningSalon.styles(Tier::Signature, 1), KERNING_REG_FEMALE);
        assert_eq!(Shop::KerningSalon.styles(Tier::Mystery, 0), KERNING_VIP_MALE);
        assert_eq!(Shop::KerningSalon.styles(Tier::Mystery, 1), KERNING_VIP_FEMALE);
        assert!(Shop::HenesysSurgery.styles(Tier::Signature, 0).is_empty());
        assert_eq!(faces(Tier::Signature, 0), FACE_REG_MALE);
        assert_eq!(faces(Tier::Signature, 1), FACE_REG_FEMALE);
        assert_eq!(faces(Tier::Mystery, 0), FACE_VIP_MALE);
        assert_eq!(faces(Tier::Mystery, 1), FACE_VIP_FEMALE);
        for pool in [HENESYS_REG_MALE, HENESYS_VIP_MALE, KERNING_REG_MALE, KERNING_VIP_MALE] {
            assert!(pool.iter().all(|id| (30_000..31_000).contains(id)));
        }
        for pool in [HENESYS_REG_FEMALE, HENESYS_VIP_FEMALE, KERNING_REG_FEMALE, KERNING_VIP_FEMALE] {
            assert!(pool.iter().all(|id| (31_000..32_000).contains(id)));
        }
        assert!(FACE_REG_MALE.iter().chain(FACE_VIP_MALE).all(|id| (20_000..21_000).contains(id)));
        assert!(FACE_REG_FEMALE.iter().chain(FACE_VIP_FEMALE).all(|id| (21_000..22_000).contains(id)));
        assert_eq!(Desk::Styles.coupon(Tier::Signature), Some(5_150_100));
        assert_eq!(Desk::Styles.coupon(Tier::Mystery), Some(5_150_000));
        assert_eq!(Desk::Colours.coupon(Tier::Signature), Some(5_151_100));
        assert_eq!(Desk::Colours.coupon(Tier::Mystery), Some(5_151_000));
        assert_eq!(Desk::Faces.coupon(Tier::Signature), Some(5_152_200));
        assert_eq!(Desk::Faces.coupon(Tier::Mystery), Some(5_152_000));
        assert_eq!(Desk::Skins.coupon(Tier::Signature), Some(5_153_000));
        assert_eq!(Desk::Skins.coupon(Tier::Mystery), None, "this client has no mystery skin item");
    }

    /// The menu lists exactly the coupons held, with icons, under fixed #L numbers; an
    /// answer naming an unheld line is refused; neither held is no menu at all; the skin
    /// desk's no-coupon line names its one coupon.
    #[test]
    fn the_menu_lists_what_is_held_and_an_answer_is_read_back_against_it() {
        assert_eq!(menu_text(Desk::Styles, false, false), None);
        let both = menu_text(Desk::Styles, true, true).unwrap();
        assert!(both.contains("#L0##i5150100# #t5150100#") && both.contains("#L1##i5150000# #t5150000#"), "{both}");
        let only_mystery = menu_text(Desk::Colours, false, true).unwrap();
        assert!(only_mystery.contains("#L1##i5151000#") && !only_mystery.contains("#L0#"), "{only_mystery}");
        let faces = menu_text(Desk::Faces, true, true).unwrap();
        assert!(faces.contains("#L0##i5152200#") && faces.contains("#L1##i5152000#"), "{faces}");
        let skins = menu_text(Desk::Skins, true, true).unwrap();
        assert!(skins.contains("#L0##i5153000#") && !skins.contains("#L1#"), "no mystery skin line: {skins}");
        assert_eq!(tier_of_selection(0, true, true), Some(Tier::Signature));
        assert_eq!(tier_of_selection(1, true, true), Some(Tier::Mystery));
        assert_eq!(tier_of_selection(0, false, true), None, "the Signature line was not offered");
        assert_eq!(tier_of_selection(7, true, true), None);
        let line = no_coupon_line(Desk::Colours);
        assert!(line.contains("#i5151100#") && line.contains("#i5151000#") && line.contains("Cash Shop"), "{line}");
        let line = no_coupon_line(Desk::Skins);
        assert!(line.contains("#i5153000#") && !line.contains("#i5153100#") && line.contains("buy it in"), "{line}");
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

    /// A face keeps its eye colour across a style change when the client draws that
    /// colour, and drops to colour 0 when it does not; skins go on the wire as their bytes.
    #[test]
    fn a_face_keeps_its_eye_colour_and_skins_are_the_seven_drawn() {
        let mut config = Config::default();
        for c in 0..9 {
            config.face_ids.insert(20_003 + c * 100);
        }
        config.face_ids.insert(20_010);
        assert_eq!(face_with_current_eye_colour(20_003, 21_405, &config), 20_403, "colour 4 kept");
        assert_eq!(face_with_current_eye_colour(20_003, 20_005, &config), 20_003, "colour 0 stays");
        assert_eq!(face_with_current_eye_colour(20_010, 21_405, &config), 20_010, "20410 unknown: base");
        assert_eq!(skin_candidates(), vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(skin_name(4), "Ashen");
        assert_eq!(skin_name(9), "?");
    }
}
