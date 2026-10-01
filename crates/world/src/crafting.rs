//! The six crafting professions: the recipe table, the mastery curve, and the two gates.
//!
//! The owner, 2026-09-21: *"We need to implement crafting in our server. After these quest
//! completions, they should unlock the appropriate crafting menu within the client."*
//! The decode is `research/crafting-2026-09-21.md`; the wire is `net::craft`; what a
//! character has learnt is `store::crafting`; the handler is `session/craft.rs`.
//!
//! # The window is the client's, and it only ever asks two questions
//!
//! The Crafting Journal reads `Etc/CraftRecipe.img` itself, so it already knows every
//! recipe, its ingredients and its price. It asks the server *"may I start this"* and, once
//! its own `ProcessTimeMS` animation has run, *"did it work"*. **This server owns the bag,
//! the mesos and the mastery; the client owns the list and the timer.**
//!
//! # A missing table is not an error
//!
//! `gm-handbook/` is generated and gitignored, so a clean checkout has no recipes. Same rule
//! as `world::chairs`: an empty table, and crafting answers
//! [`net::craft::CraftResult::Unavailable`] - *"This function is currently unavailable."* -
//! rather than pretending a craft failed for a reason the player could fix.

use std::collections::HashMap;
use std::path::Path;

/// One crafting recipe, as `tools/dump_craftrecipe.py` writes it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Recipe {
    /// `(craftLevel + profession * 10) * 1000 + index` - **the client's own key**, and the
    /// only thing a `0x02F6` carries that names a recipe. `research/crafting-2026-09-21.md`
    /// §2.
    pub key: u32,
    /// 0..5.
    pub profession: u8,
    /// The profession level this recipe needs, which is also the WZ node it sits under.
    pub craft_level: u32,
    /// How long the client's own animation runs. Recorded, not enforced: the server would
    /// only be guessing at the client's clock, and `Complete` arriving early is not the
    /// thing that costs anybody an item.
    pub process_time_ms: u32,
    /// Mesos the craft costs. 18 recipes have none.
    pub meso: u32,
    /// The optional ADDITIVE and how many of it. `(0, 0)` when the recipe has none.
    pub additive: (u32, u32),
    /// `(itemId, count)`, in the client's order.
    pub ingredients: Vec<(u32, u32)>,
    /// What the craft makes, and how many.
    pub result: (u32, u32),
    /// The **mastery** the craft pays. The client never reads this; it is ours to apply.
    pub result_exp: u32,
}

/// Every recipe, keyed by [`Recipe::key`].
pub type Recipes = HashMap<u32, Recipe>;

/// The station NPC a profession needs standing next to: `800010 + profession`.
///
/// `FUN_1401d2f70` looks exactly this template up in the NPC pool and compares positions.
/// **[D]**, `research/crafting-2026-09-21.md` §6.
pub const FIRST_STATION_TEMPLATE: u32 = 800_010;

/// How far from the station the client will still craft: `|npc.x - x| < 90`. **[D]**
pub const STATION_RANGE_X: i32 = 90;

/// And vertically: `npc.y` in `[y - 50, y + 50)`. **[D]**
pub const STATION_RANGE_Y: i32 = 50;

/// The station template for a profession, or `None` past the sixth.
pub fn station_template(profession: u8) -> Option<u32> {
    (profession < 6).then(|| FIRST_STATION_TEMPLATE + u32::from(profession))
}

/// Whether a character standing at `(x, y)` is close enough to a station at `(nx, ny)`.
///
/// The client's own test, both halves: x is symmetric, y is **not** - it is
/// `y - 50 <= ny < y + 50`.
pub fn within_station_range(x: i32, y: i32, nx: i32, ny: i32) -> bool {
    (nx - x).abs() < STATION_RANGE_X && (y - STATION_RANGE_Y..y + STATION_RANGE_Y).contains(&ny)
}

/// The mastery one profession level costs, straight off the client's `FUN_1401d2320`.
///
/// ```text
/// 1401d2335  MOV EAX,0x32                  ; 50
/// 1401d234e  MOVSD XMM6,[0x143273950]      ; 1.32, used for every level up to 10
/// 1401d2381  CVTTSD2SI EAX,XMM0            ; truncate
/// 1401d2387  ADD EAX,0x64                  ; + 100
/// ```
///
/// -> `50, 166, 319, 521, 787, 1138, 1602, 2214, 3022, 4089`, and **0 past level 10**, which
/// is the client's way of saying "this is the last level".
///
/// # This disagrees with the table the owner sent, deliberately
///
/// meowdb's *Crafting EXP (Mastery) Table* is `50, 115, 199, 308, 450, 635, 875, 1187,
/// 1593, 2120` - `trunc(prev * 1.3) + 50`. The client draws its bar as
/// `exp / mastery_exp_needed(level)` using the function above, so a server on meowdb's
/// numbers would level a profession while the bar still read 74%. The client's own curve
/// wins for that reason alone; swapping the two constants below is the whole change if the
/// numbers should follow meowdb instead.
///
/// ```
/// use world::crafting::mastery_exp_needed;
/// assert_eq!(mastery_exp_needed(1), 50);
/// assert_eq!(mastery_exp_needed(2), 166);
/// assert_eq!(mastery_exp_needed(10), 4089);
/// assert_eq!(mastery_exp_needed(11), 0);
/// ```
pub fn mastery_exp_needed(level: u32) -> u32 {
    if level == 0 || level > net::craft::MAX_PROFESSION_LEVEL {
        return 0;
    }
    let mut need = 50f64;
    for _ in 2..=level {
        need = (need * 1.32).trunc() + 100.0;
    }
    need as u32
}

/// The highest profession level a character of this level may reach.
///
/// The client's own sentence, `FUN_1410ec9c0`: *"To level up %s further, your character must
/// be level %d or higher"*, with `(professionLevel + 1) * 5`. So level `L` needs character
/// level `L * 5` - and level 1 is the starter quest's own `lvmin 10`. meowdb's table agrees
/// (`char_lv = 5 x craft_lv`).
///
/// ```
/// use world::crafting::mastery_level_cap;
/// assert_eq!(mastery_level_cap(10), 2);   // a level-10 character can reach profession 2
/// assert_eq!(mastery_level_cap(37), 7);
/// assert_eq!(mastery_level_cap(200), 10); // never past the skill's own maxLevel
/// ```
pub fn mastery_level_cap(character_level: u32) -> u32 {
    (character_level / 5).clamp(1, net::craft::MAX_PROFESSION_LEVEL)
}

/// Read `gm-handbook/craftrecipes.txt`. A missing or unreadable file gives an empty table.
///
/// A row that does not parse is **skipped rather than defaulted**, for the reason
/// `world::chairs::load_chairs` gives: a recipe whose numbers could not be read has to
/// behave like a recipe this server has never heard of, not like a free one.
pub fn load_recipes(path: &Path) -> Recipes {
    let mut out = Recipes::new();
    let Ok(text) = std::fs::read_to_string(path) else { return out };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split(',').map(str::trim).collect();
        if f.len() < 14 {
            continue;
        }
        let n = |i: usize| f.get(i).and_then(|v| v.parse::<u32>().ok());
        let (
            Some(key),
            Some(profession),
            Some(craft_level),
            Some(process_time_ms),
            Some(meso),
            Some(additive_item),
            Some(additive_count),
            Some(result_item),
            Some(result_count),
            Some(result_exp),
        ) = (n(0), n(1), n(2), n(7), n(8), n(9), n(10), n(11), n(12), n(13))
        else {
            continue;
        };
        if profession > 5 || result_item == 0 {
            continue;
        }
        let mut ingredients = Vec::new();
        for pair in f.get(14).copied().unwrap_or("").split('|') {
            let Some((id, count)) = pair.split_once(':') else { continue };
            let (Ok(id), Ok(count)) = (id.trim().parse::<u32>(), count.trim().parse::<u32>())
            else {
                continue;
            };
            ingredients.push((id, count));
        }
        out.insert(
            key,
            Recipe {
                key,
                profession: profession as u8,
                craft_level,
                process_time_ms,
                meso,
                additive: (additive_item, additive_count),
                ingredients,
                result: (result_item, result_count),
                result_exp,
            },
        );
    }
    out
}

/// The chat line a craft puts in the log, in the client's own wording.
///
/// The client's `"Craft %d %s"` is the window's own label; the mastery line is ours, and it
/// is worded the way the screenshot the owner sent words it.
pub fn mastery_line(profession_name: &str, gained: u32) -> String {
    format!("{profession_name}'s mastery increased. (+{gained})")
}

/// What the Crafting Journal calls each tab - the client's own strings at `0x1432a1b88`.
pub const PROFESSION_NAMES: [&str; 6] = [
    "Smithing",
    "Weaponcrafting",
    "Tailoring",
    "Woodcrafting",
    "Leatherworking",
    "Arcforge",
];

/// A profession's name, or `""` past the sixth.
pub fn profession_name(profession: u8) -> &'static str {
    PROFESSION_NAMES.get(profession as usize).copied().unwrap_or("")
}

/// The profession a chat argument names: `0`..`5`, or one of [`PROFESSION_NAMES`],
/// case-insensitively. For `!craft`.
pub fn profession_from_word(word: &str) -> Option<u8> {
    if let Ok(n) = word.parse::<u8>() {
        return (n < 6).then_some(n);
    }
    PROFESSION_NAMES
        .iter()
        .position(|n| n.eq_ignore_ascii_case(word))
        .map(|i| i as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The curve is the client's, and the last level is marked by a 0 rather than by a
    /// number nobody can reach.
    #[test]
    fn the_mastery_curve_is_the_clients_own() {
        let want = [50, 166, 319, 521, 787, 1138, 1602, 2214, 3022, 4089];
        for (i, w) in want.iter().enumerate() {
            assert_eq!(mastery_exp_needed(i as u32 + 1), *w, "level {}", i + 1);
        }
        assert_eq!(mastery_exp_needed(0), 0);
        assert_eq!(mastery_exp_needed(11), 0);
        // The meowdb curve, for the record: if this ever becomes the wanted one it is the
        // two constants in `mastery_exp_needed`, and this assertion is what changes with it.
        assert_ne!(mastery_exp_needed(2), 115);
    }

    /// The character-level gate, at the three places it bends.
    #[test]
    fn the_character_level_caps_the_profession() {
        assert_eq!(mastery_level_cap(1), 1, "below 5 the cap is still the starting level");
        assert_eq!(mastery_level_cap(10), 2);
        assert_eq!(mastery_level_cap(14), 2);
        assert_eq!(mastery_level_cap(15), 3);
        assert_eq!(mastery_level_cap(50), 10);
        assert_eq!(mastery_level_cap(199), 10);
    }

    /// The station test, including the asymmetric y the client actually uses.
    #[test]
    fn the_station_has_to_be_within_ninety_across_and_fifty_up() {
        assert_eq!(station_template(2), Some(800_012));
        assert_eq!(station_template(6), None);
        assert!(within_station_range(100, 200, 189, 200));
        assert!(!within_station_range(100, 200, 190, 200), "90 is out, not in");
        assert!(within_station_range(100, 200, 100, 151));
        assert!(!within_station_range(100, 200, 100, 149));
        assert!(within_station_range(100, 200, 100, 249));
        assert!(!within_station_range(100, 200, 100, 250));
    }

    /// The table parses, and the control recipe is the one quest 80011 asks for.
    #[test]
    fn the_table_reads_the_dumpers_rows() {
        let text = "# a comment\n\
                    1000, 0, 1, 0, 92000000, 1, 0, 3000, 100, 0, 0, 4010100, 1, 3, 4010000:5\n\
                    1002, 0, 1, 2, 92000000, 1, 0, 5000, 1000, 4130002, 1, 1002017, 1, 20, 1002016:1|4010101:2\n\
                    nonsense\n\
                    9, 9, 9\n";
        let dir = std::env::temp_dir().join(format!("maplecw-craft-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("craftrecipes.txt");
        std::fs::write(&path, text).unwrap();
        let table = load_recipes(&path);
        assert_eq!(table.len(), 2, "the short and the unparseable rows are skipped");
        let r = &table[&1000];
        assert_eq!(r.profession, 0);
        assert_eq!(r.craft_level, 1);
        assert_eq!(r.meso, 100);
        assert_eq!(r.additive, (0, 0));
        assert_eq!(r.ingredients, vec![(4_010_000, 5)]);
        assert_eq!(r.result, (4_010_100, 1));
        assert_eq!(r.result_exp, 3);
        let r = &table[&1002];
        assert_eq!(r.additive, (4_130_002, 1));
        assert_eq!(r.ingredients, vec![(1_002_016, 1), (4_010_101, 2)]);
        // A missing file is an empty table, not a panic.
        assert!(load_recipes(&dir.join("nothing-here.txt")).is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    /// The tab names, and what `!craft` accepts for them.
    #[test]
    fn a_profession_can_be_named_by_number_or_by_word() {
        assert_eq!(profession_from_word("2"), Some(2));
        assert_eq!(profession_from_word("tailoring"), Some(2));
        assert_eq!(profession_from_word("ArcForge"), Some(5));
        assert_eq!(profession_from_word("6"), None);
        assert_eq!(profession_from_word("baking"), None);
        assert_eq!(profession_name(4), "Leatherworking");
        assert_eq!(mastery_line("Tailoring", 3), "Tailoring's mastery increased. (+3)");
    }
}
