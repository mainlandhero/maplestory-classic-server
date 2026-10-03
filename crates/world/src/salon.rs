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
//! | *Mystery skin* | - | surgery assistant | **no such item in this client** (`beauty.txt [coupons]` has ten, and 5153000 is the only skin one) |
//! | Signature Eye Color Coupon | 5152100 | surgery assistant | the player picks one of their face's eye colours |
//!
//! **Eye colour, 2026-10-02.** The owner: *"the collaboration hairs and eyes are not able to change
//! colors using our existing salon and plastic surgery methods, but we do have those files"*. No
//! desk changed eye colour at all - the owner kept it and the assistant did skins - while the Cash
//! Shop sold 5152100 (`research/beauty-2026-09-09.md`). The surgery assistant takes it now, beside
//! the skin coupon, the way a salon's assistant does hair colours: the player's own face in every
//! eye colour the client has art for - all nine for the collaboration faces `22035..22042`, whose
//! `22135..22842` are in `Character/Face` - from `Config::face_exists`, not from a list.
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
/// Signature Eye Color Coupon - the surgery assistant's second line. There is no Mystery one
/// on sale (5152300, Custom Colorblend, has no Commodity row).
pub const SIGNATURE_EYE_COLOR_COUPON: u32 = 5_152_100;
/// Custom Mix Dye Coupon - two hair colours and a ratio, at the salon assistant (the owner,
/// 2026-10-03). The client types it `0x18` MixHairColor and opens `UtilDlgEx_MixHair` for
/// it; `research/mix-dye-colorblend.md`. **Not on sale in this client's Cash Shop** - it
/// has no `Commodity.img` row (`research/beauty-2026-09-09.md` §2.2).
pub const CUSTOM_MIX_DYE_COUPON: u32 = 5_151_200;
/// Custom Colorblend Eye Color Coupon - two eye colours and a ratio, at the surgery assistant.
/// Typed `0xe` MixColorLens, dialog `UtilDlgEx_MixLens`. Not on sale either.
pub const CUSTOM_COLORBLEND_COUPON: u32 = 5_152_300;

/// The conversation path of an owner's or assistant's coupon menu.
pub const MENU_PATH: &str = "salon.menu";
/// The conversation path of the pick-a-look box, whatever it shows.
pub const CHOICE_PATH: &str = "salon.choice";
/// The pick-a-look box for eye colours, which the assistant's desk (skins) cannot name back.
pub const EYE_CHOICE_PATH: &str = "salon.choice.eyes";
/// The mix box (`net::script::npc_mix`): Custom Mix Dye or Custom Colorblend.
pub const MIX_PATH: &str = "salon.mix";

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
    /// A surgery assistant's second line: eye colours (see the module docs).
    EyeColours,
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
            (Desk::EyeColours, Tier::Signature) => Some(SIGNATURE_EYE_COLOR_COUPON),
            (Desk::EyeColours, Tier::Mystery) => None,
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
            (Desk::EyeColours, _) => "pick an eye colour",
        }
    }

    /// What the desk sells, for the no-coupon line.
    fn wares(self) -> &'static str {
        match self {
            Desk::Styles => "a new style",
            Desk::Colours => "a new colour",
            Desk::Faces => "a new face",
            Desk::Skins => "a new skin tone",
            Desk::EyeColours => "a new eye colour",
        }
    }

    /// The mix coupon this desk takes on its [`MENU_MIX`] line: the salon assistant's Custom
    /// Mix Dye, the surgery assistant's Custom Colorblend. `None` for an owner.
    pub fn mix_coupon(self) -> Option<u32> {
        match self {
            Desk::Colours => Some(CUSTOM_MIX_DYE_COUPON),
            Desk::Skins => Some(CUSTOM_COLORBLEND_COUPON),
            _ => None,
        }
    }

    /// The desk whose Signature coupon this NPC also takes, on a third menu line: the surgery
    /// assistant's eye colours beside its skins.
    pub fn second(self) -> Option<Desk> {
        match self {
            Desk::Skins => Some(Desk::EyeColours),
            _ => None,
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
/// The [`Desk::second`] desk's Signature line.
pub const MENU_SECOND: u32 = 2;
/// The [`Desk::mix_coupon`] line.
pub const MENU_MIX: u32 = 3;

/// The coupon menu for a desk: one line per coupon the player holds, in the client's own
/// `#d#L%d# %s#l#k` line format with the item's icon and name. `None` when they hold neither.
pub fn menu_text(desk: Desk, has_signature: bool, has_mystery: bool, has_second: bool, has_mix: bool) -> Option<String> {
    let second = desk.second().filter(|_| has_second);
    let mix = desk.mix_coupon().filter(|_| has_mix);
    if !has_signature && !has_mystery && second.is_none() && mix.is_none() {
        return None;
    }
    let mut lines = vec![match desk {
        Desk::Styles | Desk::Faces => "Which coupon would you like to use today?",
        Desk::Colours => "A new colour, then? Which coupon would you like to use?",
        Desk::Skins | Desk::EyeColours => "A new skin tone or eye colour, then? Which coupon would you like to use?",
    }
    .to_string()];
    for (tier, held, sel) in [(Tier::Signature, has_signature, MENU_SIGNATURE), (Tier::Mystery, has_mystery, MENU_MYSTERY)] {
        if let (true, Some(c)) = (held, desk.coupon(tier)) {
            lines.push(format!("#d#L{sel}##i{c}# #t{c}# - {}#l#k", desk.menu_line(tier)));
        }
    }
    if let Some(d) = second {
        if let Some(c) = d.coupon(Tier::Signature) {
            lines.push(format!("#d#L{MENU_SECOND}##i{c}# #t{c}# - {}#l#k", d.menu_line(Tier::Signature)));
        }
    }
    if let Some(c) = mix {
        lines.push(format!("#d#L{MENU_MIX}##i{c}# #t{c}# - {}#l#k", mix_line(desk)));
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
    if let Some(eye) = desk.second().and_then(|d| d.coupon(Tier::Signature)) {
        lines.push(format!("#i{eye}# #b#t{eye}##k - a new eye colour, you choose"));
    }
    if let Some(mix) = desk.mix_coupon() {
        lines.push(format!("#i{mix}# #b#t{mix}##k - {}", mix_line(desk)));
    }
    lines.push(String::new());
    lines.push(match (desk.coupon(Tier::Mystery), desk.second()) {
        (None, None) => "You can buy it in the #bCash Shop#k.".to_string(),
        _ => "You can buy either in the #bCash Shop#k.".to_string(),
    });
    // The mix coupons have no Commodity row in this client, so the Cash Shop cannot sell them
    // (`research/beauty-2026-09-09.md` 2.2) - the line must not say it does.
    if let Some(mix) = desk.mix_coupon() {
        lines.push(format!("The #b#t{mix}##k is not sold there."));
    }
    lines.join("\r\n")
}

/// What the mix line does, in the menu and the no-coupon list.
pub fn mix_line(desk: Desk) -> &'static str {
    match desk {
        Desk::Skins | Desk::EyeColours => "blend two eye colours",
        _ => "mix two hair colours",
    }
}

/// The prompt over the mix box.
pub fn mix_prompt(desk: Desk) -> &'static str {
    match desk {
        Desk::Skins | Desk::EyeColours => {
            "Pick two eye colours and how much of each. Your face stays as it is."
        }
        _ => "Pick two colours and how much of each. Your style stays as it is.",
    }
}

/// The prompt over the pick-a-look box.
pub fn choice_prompt(desk: Desk) -> &'static str {
    match desk {
        Desk::Styles => "Pick any of these styles. Your hair colour stays as it is. Which one will it be?",
        Desk::Colours => "Pick a colour. Your style stays as it is. Which one will it be?",
        Desk::Faces => "Pick any of these faces. Your eye colour stays as it is. Which one will it be?",
        Desk::Skins => "Pick a skin tone. Which one will it be?",
        Desk::EyeColours => "Pick an eye colour. Your face stays as it is. Which one will it be?",
    }
}

/// The id the player ends up wearing after a hair STYLE change: `base` in their current
/// colour when that colour exists for it, `base` itself otherwise. **A mixed colour comes
/// along** when the new style draws both of its colours - the colour is the player's, not the
/// style's - and drops to the plain first colour when it does not.
pub fn with_current_colour(base: u32, current_hair: u32, config: &Config) -> u32 {
    let colour = unmixed(current_hair) % 10;
    let candidate = base + colour;
    let plain = if colour != 0 && config.hair_exists(candidate) { candidate } else { base };
    match blend_of(current_hair) {
        Some(b) if plain == base + u32::from(b.base) && config.hair_exists(base + u32::from(b.mix)) => {
            plain * 1000 + u32::from(b.mix) * 100 + u32::from(b.percent)
        }
        _ => plain,
    }
}

/// The same for a face: `base` at the player's current eye colour (the hundreds digit)
/// when that exists, `base` itself otherwise - a blended eye colour kept the same way.
pub fn face_with_current_eye_colour(base: u32, current_face: u32, config: &Config) -> u32 {
    let colour = (unmixed(current_face) / 100) % 10;
    let candidate = base + colour * 100;
    let plain = if colour != 0 && config.face_exists(candidate) { candidate } else { base };
    match blend_of(current_face) {
        Some(b) if plain == base + u32::from(b.base) * 100 && config.face_exists(base + u32::from(b.mix) * 100) => {
            plain * 1000 + u32::from(b.mix) * 100 + u32::from(b.percent)
        }
        _ => plain,
    }
}

/// The player's current hair style (its base id, colour digit 0), mixed or not.
pub fn base_of(hair: u32) -> u32 {
    let hair = unmixed(hair);
    hair - hair % 10
}

/// The colour variants of the player's current style that this client draws, in colour
/// order - what the salon assistant's pick box shows. Empty when the base itself is unknown
/// to the hair table, so a bare table refuses rather than offers ids that do not draw.
pub fn colour_variants(current_hair: u32, config: &Config) -> Vec<u32> {
    let base = base_of(current_hair);
    (0..COLOURS.len() as u32).map(|c| base + c).filter(|id| config.hair_exists(*id)).collect()
}

/// The eye colours of the player's current face that this client draws, colour 0..8 in
/// order - the surgery assistant's pick box for the Signature Eye Color Coupon. A face id is
/// `style + 100 * eye`, so the style is the id with its hundreds digit cleared. Empty when the
/// face table does not know the style, so a bare table refuses rather than offers ids that do
/// not draw.
pub fn eye_colour_variants(current_face: u32, config: &Config) -> Vec<u32> {
    let style = face_style_of(current_face);
    (0..9).map(|c| style + c * 100).filter(|id| config.face_exists(*id)).collect()
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
    COLOURS.get((unmixed(hair) % 10) as usize).copied().unwrap_or("?")
}

/// A face's style (eye colour digit 0), blended or not.
pub fn face_style_of(face: u32) -> u32 {
    let face = unmixed(face);
    face - (face / 100 % 10) * 100
}

// ---------------------------------------------------------------------------------------
// Mix Dye and Colorblend: two colours and a ratio, inside the ordinary id.
// ---------------------------------------------------------------------------------------

/// **A mixed hair or blended face is the plain id times 1000, plus `mix * 100 + percent`.**
/// [L] off the client (`research/mix-dye-colorblend.md`): every look helper first does
/// `if id > 9_999_999 { id /= 1000 }` (`FUN_14041a0f0`, `FUN_14041a7e0`, `FUN_14041a780`);
/// the hair composer `FUN_14041a8d0` builds `(style + base) * 1000 + mix * 100 + percent`,
/// the face composer `FUN_14041a820` the same with the eye colour in the hundreds digit, and
/// `FUN_14041a520` / `FUN_14041a480` take them apart again. So a mixed look needs no new
/// field anywhere: it is a bigger `u32` in the same `hair` and `face` slots.
pub const MIXED_ABOVE: u32 = 9_999_999;

/// The plain id under a mixed one, or the id itself.
pub fn unmixed(id: u32) -> u32 {
    if id > MIXED_ABOVE {
        id / 1000
    } else {
        id
    }
}

/// Whether a plain look id is a hair rather than a face. Hair ids start at 30000 and face
/// ids sit at 20000..29999 (`research/beauty-2026-09-09.md` §3) - and once mixed both are
/// eight digits, so the size of the mixed id cannot tell them apart.
fn is_hair(id: u32) -> bool {
    unmixed(id) >= 30_000
}

/// Two colours and a ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blend {
    /// The colour in the plain id: a hair's last digit, a face's hundreds digit.
    pub base: u8,
    /// The second colour.
    pub mix: u8,
    /// `1..=99`. The client treats `(a, b, p)` and `(b, a, 100 - p)` as the same look
    /// (`FUN_14041a2a0`), so which colour it is the share of only matters for wording.
    pub percent: u8,
}

impl Blend {
    /// The mix box's answer, `(base * 10 + mix) * 1000 + percent` (`FUN_14041a3d0`), or
    /// `None` outside what the box itself accepts: both colours under 10, percent `1..=99`
    /// (`FUN_14041a270`).
    pub fn from_reply(value: u32) -> Option<Blend> {
        let (base, mix, percent) = (value / 10_000, (value / 1000) % 10, value % 1000);
        (base < 10 && mix < 10 && (1..=99).contains(&percent))
            .then(|| Blend { base: base as u8, mix: mix as u8, percent: percent as u8 })
    }
}

/// The blend inside a mixed id, `None` for a plain one.
pub fn blend_of(id: u32) -> Option<Blend> {
    if id <= MIXED_ABOVE {
        return None;
    }
    let low = id % 1000;
    let base = if is_hair(id) { unmixed(id) % 10 } else { unmixed(id) / 100 % 10 };
    Some(Blend { base: base as u8, mix: (low / 100) as u8, percent: (low % 100) as u8 })
}

/// The hair a Custom Mix Dye gives: the player's style, `blend.base` and `blend.mix` mixed.
/// `None` - nothing spent - for a colour outside the eight (`FUN_14041a8d0` leaves the hair
/// alone there) or one this client has no art for in that style.
pub fn mixed_hair(current_hair: u32, blend: Blend, config: &Config) -> Option<u32> {
    let style = base_of(current_hair);
    let (a, b) = (style + u32::from(blend.base), style + u32::from(blend.mix));
    (blend.base < 8 && blend.mix < 8 && config.hair_exists(a) && config.hair_exists(b))
        .then(|| a * 1000 + u32::from(blend.mix) * 100 + u32::from(blend.percent))
}

/// The face a Custom Colorblend gives: the player's face, eye colours `blend.base` and
/// `blend.mix` blended (`FUN_14041a820`). `None` for an eye colour this face has no art for.
pub fn blended_face(current_face: u32, blend: Blend, config: &Config) -> Option<u32> {
    let style = face_style_of(current_face);
    let (a, b) = (style + u32::from(blend.base) * 100, style + u32::from(blend.mix) * 100);
    (blend.base < 9 && blend.mix < 9 && config.face_exists(a) && config.face_exists(b))
        .then(|| a * 1000 + u32::from(blend.mix) * 100 + u32::from(blend.percent))
}

/// Whether `new` is the look the player already has, the way the client decides it
/// (`FUN_1401a9eb0` / `FUN_1401a9e00` -> `FUN_14041a2a0`): the same id, or the same style
/// with the two colours swapped and the ratio turned round. The client refuses those itself,
/// and the server must not spend a coupon on one either.
pub fn same_look(current: u32, new: u32) -> bool {
    if current == new {
        return true;
    }
    let style = |id: u32| if is_hair(id) { base_of(id) } else { face_style_of(id) };
    match (blend_of(current), blend_of(new)) {
        (Some(a), Some(b)) => {
            style(current) == style(new)
                && a.base == b.mix
                && a.mix == b.base
                && u32::from(a.percent) == 100 - u32::from(b.percent)
        }
        _ => false,
    }
}

/// The two colour names of a mixed hair, for a notice: "Black and Red".
pub fn hair_blend_name(hair: u32) -> Option<String> {
    let b = blend_of(hair)?;
    let name = |c: u8| COLOURS.get(usize::from(c)).copied().unwrap_or("?");
    Some(format!("{} and {}", name(b.base), name(b.mix)))
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
        assert_eq!(menu_text(Desk::Styles, false, false, false, false), None);
        assert_eq!(menu_text(Desk::Styles, false, false, true, false), None, "only the surgery assistant has a second line");
        let both = menu_text(Desk::Styles, true, true, false, false).unwrap();
        assert!(both.contains("#L0##i5150100# #t5150100#") && both.contains("#L1##i5150000# #t5150000#"), "{both}");
        let only_mystery = menu_text(Desk::Colours, false, true, false, false).unwrap();
        assert!(only_mystery.contains("#L1##i5151000#") && !only_mystery.contains("#L0#"), "{only_mystery}");
        let faces = menu_text(Desk::Faces, true, true, false, false).unwrap();
        assert!(faces.contains("#L0##i5152200#") && faces.contains("#L1##i5152000#"), "{faces}");
        let skins = menu_text(Desk::Skins, true, true, false, false).unwrap();
        assert!(skins.contains("#L0##i5153000#") && !skins.contains("#L1#"), "no mystery skin line: {skins}");
        assert!(!skins.contains("#L2#"), "no eye line without the eye coupon: {skins}");
        let both = menu_text(Desk::Skins, true, false, true, false).unwrap();
        assert!(both.contains("#L0##i5153000#") && both.contains("#L2##i5152100# #t5152100#"), "{both}");
        let eyes_only = menu_text(Desk::Skins, false, false, true, false).unwrap();
        assert!(eyes_only.contains("#L2##i5152100#") && !eyes_only.contains("#L0#"), "{eyes_only}");
        assert_eq!(tier_of_selection(0, true, true), Some(Tier::Signature));
        assert_eq!(tier_of_selection(1, true, true), Some(Tier::Mystery));
        assert_eq!(tier_of_selection(0, false, true), None, "the Signature line was not offered");
        assert_eq!(tier_of_selection(7, true, true), None);
        let line = no_coupon_line(Desk::Colours);
        assert!(line.contains("#i5151100#") && line.contains("#i5151000#") && line.contains("Cash Shop"), "{line}");
        let line = no_coupon_line(Desk::Skins);
        assert!(line.contains("#i5153000#") && line.contains("#i5152100#") && !line.contains("#i5153100#"), "{line}");
        assert!(line.contains("buy either in"), "two coupons now: {line}");
        assert_eq!(Desk::EyeColours.coupon(Tier::Signature), Some(5_152_100));
        assert_eq!(Desk::EyeColours.coupon(Tier::Mystery), None);
    }

    /// **Every eye colour the client draws, for any face** - the collaboration faces included,
    /// which is what the owner reported missing (2026-10-02). A face is `style + 100 * eye`; the
    /// player's current eye colour does not change the list; an unknown style offers nothing.
    #[test]
    fn eye_colours_are_the_hundreds_digit_and_only_the_ones_that_draw() {
        let mut face_ids = std::collections::HashSet::new();
        for c in 0..9 {
            face_ids.insert(22_035 + c * 100); // Frieren Face, all nine
        }
        face_ids.insert(20_000); // a classic face with only colours 0 and 3 known
        face_ids.insert(20_300);
        let config = Config { face_ids, ..Config::default() };
        let all: Vec<u32> = (0..9).map(|c| 22_035 + c * 100).collect();
        assert_eq!(eye_colour_variants(22_035, &config), all);
        assert_eq!(eye_colour_variants(22_535, &config), all, "the current eye colour is not the style");
        assert_eq!(eye_colour_variants(20_300, &config), vec![20_000, 20_300], "only the ones with art");
        assert!(eye_colour_variants(21_999, &config).is_empty());
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

    /// **The mix arithmetic is the client's**, case by case (`research/mix-dye-colorblend.md`).
    /// Black (0) and Blue (5) at 30 on Frieren Hair 42540 is `42540 * 1000 + 5 * 100 + 30`; eye
    /// colours 2 and 7 at 60 on face 22035 put colour 2 in the hundreds digit first. The box's
    /// value `(base*10+mix)*1000+percent` reads back; a 0 or 100 percent, a hair colour past
    /// the eight and a colour with no art are refused; the swapped twin is the same look; and a
    /// style or colour change on a mixed hair reads the plain id under it.
    #[test]
    fn mixed_ids_are_the_plain_id_times_1000_plus_mix_and_percent() {
        let mut hair_ids = std::collections::HashSet::new();
        for c in 0..8 {
            hair_ids.insert(42_540 + c);
            hair_ids.insert(30_030 + c);
        }
        let face_ids: std::collections::HashSet<u32> = (0..9).map(|c| 22_035 + c * 100).collect();
        let config = Config { hair_ids, face_ids, ..Config::default() };

        assert_eq!(Blend::from_reply(5_030), Some(Blend { base: 0, mix: 5, percent: 30 }));
        assert_eq!(Blend::from_reply(27_060), Some(Blend { base: 2, mix: 7, percent: 60 }));
        assert_eq!(Blend::from_reply(5_000), None, "0 percent is not a mix");
        assert_eq!(Blend::from_reply(5_100), None, "nor is 100");
        assert_eq!(Blend::from_reply(105_030), None, "a colour past 9");

        let blue = Blend { base: 0, mix: 5, percent: 30 };
        let mixed = mixed_hair(42_542, blue, &config).unwrap();
        assert_eq!(mixed, 42_540_530, "the style kept, colour 0, then mix 5 at 30");
        assert_eq!((unmixed(mixed), base_of(mixed), colour_name(mixed)), (42_540, 42_540, "Black"));
        assert_eq!(blend_of(mixed), Some(blue));
        assert_eq!(hair_blend_name(mixed).as_deref(), Some("Black and Blue"));
        assert_eq!(mixed_hair(42_540, Blend { base: 0, mix: 8, percent: 30 }, &config), None, "colour 8 is not a hair colour");
        assert_eq!(mixed_hair(31_000, blue, &config), None, "no art for that style");

        let eyes = blended_face(22_135, Blend { base: 2, mix: 7, percent: 60 }, &config).unwrap();
        assert_eq!(eyes, 22_235_760, "eye colour 2 in the hundreds digit, then 7 at 60");
        assert_eq!(face_style_of(eyes), 22_035);
        assert_eq!(blend_of(eyes), Some(Blend { base: 2, mix: 7, percent: 60 }));
        assert_eq!(eye_colour_variants(eyes, &config).len(), 9, "a blended face still offers its plain eye colours");

        assert!(same_look(mixed, mixed));
        assert!(same_look(mixed, 42_545_070), "Blue and Black at 70 is Black and Blue at 30");
        assert!(!same_look(mixed, 42_545_030), "the ratio turned the other way is a different look");
        assert!(!same_look(42_540, mixed), "plain and mixed differ");

        // A style change keeps the mix where the new style draws both colours.
        assert_eq!(with_current_colour(30_030, mixed, &config), 30_030_530);
        let only_black: std::collections::HashSet<u32> = [30_030].into_iter().collect();
        let bare = Config { hair_ids: only_black, ..Config::default() };
        assert_eq!(with_current_colour(30_030, mixed, &bare), 30_030, "no blue art: the plain colour");
        assert_eq!(face_with_current_eye_colour(22_035, eyes, &config), 22_235_760);
    }
}
