//! Return Scrolls: which items are one, where each goes, and when a use must be refused.
//!
//! The owner, 2026-08-29: *"there are several different Return Scrolls in the game. Return Scroll
//! to specific town when used should allow the player to teleport back to the town of
//! choosing depending on the scroll (unless not on the same Continent. For example Return
//! Scroll to El Nath should not be allowed on Victoria Island). Return Scroll to Nearest Town
//! should adhere to the map's return map and teleport the player there."*
//!
//! Three questions, and each is answered from the client's own data rather than from a table
//! somebody typed. `tools/dump_returnscrolls.py` reads all three and writes
//! `gm-handbook/returnscrolls.txt`; the tests at the bottom re-read that file and fail when
//! anything baked in here has drifted away from it.
//!
//! # 1. Which items, and where to — [L], measured
//!
//! `Item.wz` gives a consumable's warp destination as `<item>/spec/moveTo`. **Ten items in
//! the whole item tree carry that key** — the scan covers Consume, Etc, Install, Cash,
//! Special and Pet, 1025 items, and all ten hits are `0203xxxx`:
//!
//! ```text
//!  2030000  Return Scroll - Nearest Town        moveTo 999999999   <- the sentinel
//!  2030001  Return Scroll to Lith Harbor        moveTo  10000000
//!  2030002  Return Scroll to Ellinia            moveTo  10002000
//!  2030003  Return Scroll to Perion             moveTo  10004000
//!  2030004  Return Scroll to Henesys            moveTo  10001000
//!  2030005  Return Scroll to Kerning City       moveTo  10003000
//!  2030006  Return Scroll to Sleepywood         moveTo  10005000
//!  2030007  Return Scroll to Forgotten Hollow   moveTo  10006000
//!  2030008  Return Scroll to Orbis              moveTo  20000000
//!  2030009  Return Scroll to El Nath            moveTo  20001000
//! ```
//!
//! Every one of those nine literal destinations is a field that is **its own `returnMap` and
//! flagged `town == 1`** — one of the 21 real town centres `research/return-maps.md` §4
//! enumerates. [L] So the destinations need no walking; they are already town squares.
//!
//! **There is no Return Scroll to anywhere on Maple Island.** [L] That is not an omission in
//! this table, it is the client's own item list — which is worth knowing before a client run,
//! because Maple Island is the whole of this server's on-foot world today
//! (`research/return-maps.md` §5). Every one of the nine literal scrolls is refused there and
//! that is correct; only `2030000` does anything without a `!map` first.
//!
//! # 2. What a continent is — [L] from `String.wz/Map.img`, then fitted to a band rule
//!
//! `String.wz/Map.img` is not a flat id table. Its **six top-level children are the client's
//! own regions** — `MapleIsland`, `VictoriaIsland`, `Ossyria`, `Other`, `Event`, `Dev` — and
//! all 432 named maps hang under exactly one of them. [L] That is the client answering
//! The owner's question directly; nothing here is guessed from the shape of an id.
//!
//! [`continent_of_map`] is that partition expressed as three ranges, and the ranges were
//! **scored against it, not assumed**: `tools/dump_returnscrolls.py` compares them to all 432
//! rows every run and exits non-zero on a single disagreement. Today it is **0 of 432**. [L]
//!
//! The honest label on the range form is therefore **[D]**, and the blind spot is worth
//! writing down so it can be quoted: a band rule cannot be *wrong* on any map this client
//! ships, because it was fitted to them, but it is not *read* — a later client that files a
//! `21xxxxxx` map under `VictoriaIsland` would break it silently. The generated file is what
//! makes that loud, which is why `the_band_rule_still_matches_the_client` exists.
//!
//! **The control that says the partition is real data and not the digits.** A rule keyed on
//! the id's leading `1e6` digits — an equally natural thing to write — splits `Other` three
//! ways, into `80`, `88` and `89`. `String.wz` says `88000000 Victoria Road` and `89000000
//! Truth Booth` are in the *same* group as `80002000 Free Market`. [L] Nothing about the
//! numbers says that; only the image does. And `Victoria Road` is the sharper half of it: its
//! **name** says Victoria Island and the client files it under `Other`, so a classifier built
//! from names — or from a human reading the list — gets that row wrong.
//!
//! # 3. `Other`, `Event` and `Dev` are not continents, and that needs a fallback
//!
//! `continent_of_map` returns `None` for them, deliberately: the Free Market, the party quest
//! and the three test maps are not places a town scroll has an opinion about. But **four job
//! advancement dungeons are filed under `Other` while standing physically on Victoria
//! Island**, and refusing a Return Scroll to Ellinia inside `Magician's Tree Dungeon` would be
//! a wrong refusal a player would notice: [L]
//!
//! ```text
//!  80001000 Ant Tunnel For Bowman      Other   returnMap 10001090 -> Henesys      (Victoria)
//!  80001100 Magician's Tree Dungeon    Other   returnMap 10002070 -> Ellinia      (Victoria)
//!  80001200 Thief's Construction Site  Other   returnMap 10003080 -> Kerning City (Victoria)
//!  80001300 Warrior's Rocky Mountain   Other   returnMap 10004023 -> Perion       (Victoria)
//!  88000000 Victoria Road              Other   returnMap 10001000 -> Henesys      (Victoria)
//!  80003000..80003400  FIVE fields with NO String.wz row at all   -> Forgotten Hollow
//!  80003500            a sixth, and its returnMap is ITSELF       -> no continent either way
//! ```
//!
//! So [`continent_of`] falls back to **the continent of the field's own nearest-town anchor**
//! when the field itself has no continent. [D] That is one lookup in a table this server
//! already loads (`Config::revive_field`), and it is what makes those ten fields behave like
//! the island they are on.
//!
//! The six `80003000..80003500` rows are the other half of that argument and they are the
//! reason the fallback is not optional: they are field images this client ships with **no
//! `String.wz` name and therefore no group at all**. `config.rs` already knew about them from
//! the other direction — "6 present but unnamed". A classifier that reads only `Map.img` is
//! undefined on 6 of 426 fields, which is exactly the shape of instrument this project keeps
//! being burnt by, so it is not left undefined.
//!
//! Measured, over all 426 field images: the field's own group and its anchor's group disagree
//! on **12**, and the twelve are worth enumerating rather than summarising, because the
//! obvious summary is off by one: [L]
//!
//! ```text
//!   4   the job-advancement dungeons  80001000 / 80001100 / 80001200 / 80001300
//!   1   88000000 Victoria Road
//!   5   80003000 / 80003100 / 80003200 / 80003300 / 80003400   -> Forgotten Hollow
//!   2   20000022 / 20000023, the To Orbis ship cabins          -> Ellinia Station
//! ```
//!
//! **Five of the six unnamed fields, not six.** `80003500` anchors to *itself*, so it has no
//! continent under either rule and cannot disagree with itself. A nearest-town scroll there
//! warps to `80003500`; a named scroll is refused. That is the data's own answer, not a gap.
//!
//! The two cabins keep `Ossyria`, because the fallback only fires when the field has no
//! continent of its own. Neither cabin is reachable in this server today. [L]
//!
//! # 4. Nearest Town resolves from `returnMap`, and this re-derives it rather than assuming
//!
//! `config.rs` records that `forcedReturn` is *not* the revive column, and the brief for this
//! work was explicit that a conclusion drawn for death does not automatically transfer. It
//! does transfer, and for a return scroll the argument is **stronger** than it was for death:
//!
//! * `forcedReturn` is the sentinel `999999999` on **354 of 426** fields. [L] An item called
//!   "Return Scroll - Nearest Town" that does nothing on 83% of the map is not the item.
//! * Where `forcedReturn` *is* set, its target is **not a town on 54 of 72**. [L] The item's
//!   own name says Town. `Dead Mine I` would send you to `Forest of Dead Trees IV`.
//! * `returnMap` is on **426 of 426** and is never the sentinel. [L]
//!
//! And the reason to use the `reviveMap` **column** rather than raw `returnMap`: `reviveMap`
//! is one unconditional hop plus a walk to the first `town == 1`, which is what turns
//! `The Grave of Mushmom -> Ant Tunnel Park` into `-> Sleepywood`. Raw `returnMap` lands in a
//! town 388 times of 426; the walked column lands in one **393** times. [L] For an item whose
//! name is "Nearest **Town**" the walked column is the right one for the same reason it was
//! right for death — and the unconditional first hop matters here too, or a scroll used in
//! `Southperry Armor Store` warps you to `Southperry Armor Store`.
//!
//! The 33 fields whose walk ends on a real non-town (party quest stages, event stages, the
//! test maps) still teleport, to that field. Refusing there would strand a character where
//! landing on a real field cannot. [D] None of the 33 is reachable today. [L]
//!
//! # 5. What happens to the scroll on a refusal
//!
//! **Nothing. The item is not consumed.** Every effect hangs off the transition, not off the
//! request — `CLAUDE.md`'s Heena-quest rule — so this module answers with an [`Outcome`] and
//! the caller does *all* of consuming, warping and replying only inside the
//! [`Outcome::Teleport`] arm. There is no path where the stack goes down and the character
//! stays put, and none where it moves without paying.
//!
//! A refusal still **answers**, in [`crate::session`]: a notice plus an empty `StatChanged`,
//! whose byte 0 clears the client's `0x010E` request latch. A silent refusal costs not one
//! scroll but every later item use for the rest of the session.

/// The client's own "there is no literal map id here" sentinel.
///
/// The same number `portal/<n>/tm` uses for a spawn point and `info/forcedReturn` uses for
/// "this field ejects nobody" — `research/return-maps.md` §1. On a scroll it means **resolve
/// the destination from the field the character is standing on**. Never warp to it.
pub const NO_MAP: u32 = 999_999_999;

/// A continent a character can stand on and travel between.
///
/// Exactly the three of `String.wz/Map.img`'s six groups that are places. `Other`, `Event`
/// and `Dev` are the other three and are deliberately not represented — a value that cannot
/// be constructed cannot be compared equal to itself by accident, and "the Free Market and
/// the party quest are on the same continent" is not a statement this type should be able to
/// make.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Continent {
    MapleIsland,
    VictoriaIsland,
    Ossyria,
}

impl Continent {
    /// What to call it on screen. The client's own group names, spaced for a chat line.
    pub fn name(self) -> &'static str {
        match self {
            Continent::MapleIsland => "Maple Island",
            Continent::VictoriaIsland => "Victoria Island",
            Continent::Ossyria => "Ossyria",
        }
    }

    /// The `String.wz/Map.img` group name this corresponds to, for the file cross-check.
    pub fn group(self) -> &'static str {
        match self {
            Continent::MapleIsland => "MapleIsland",
            Continent::VictoriaIsland => "VictoriaIsland",
            Continent::Ossyria => "Ossyria",
        }
    }
}

/// `(itemId, spec/moveTo)` for every Return Scroll this client ships.
///
/// **Read from `Item.wz`, not typed from memory** — `tools/dump_returnscrolls.py` produced
/// these ten rows by walking all 1025 items in the tree, and
/// [`the_baked_table_still_matches_the_client`] re-reads its output and fails on a drift. The
/// scan enumerates before it filters: ten of 1025 items carry `spec/moveTo` and all ten are
/// here. [L]
///
/// It is baked rather than loaded because ten rows do not justify a `Config` field, and
/// because a missing generated file would otherwise silently turn every return scroll into
/// "this item does nothing" — the failure mode this project has paid for repeatedly. The
/// cross-check test is what stops "baked" becoming "stale".
pub const SCROLLS: [(u32, u32); 10] = [
    (2_030_000, NO_MAP),      // Return Scroll - Nearest Town
    (2_030_001, 10_000_000),  // Return Scroll to Lith Harbor
    (2_030_002, 10_002_000),  // Return Scroll to Ellinia
    (2_030_003, 10_004_000),  // Return Scroll to Perion
    (2_030_004, 10_001_000),  // Return Scroll to Henesys
    (2_030_005, 10_003_000),  // Return Scroll to Kerning City
    (2_030_006, 10_005_000),  // Return Scroll to Sleepywood
    (2_030_007, 10_006_000),  // Return Scroll to Forgotten Hollow
    (2_030_008, 20_000_000),  // Return Scroll to Orbis
    (2_030_009, 20_001_000),  // Return Scroll to El Nath
];

/// `spec/moveTo` for `item_id`, or `None` if it is not a Return Scroll.
///
/// [`NO_MAP`] is a real answer here and means "the nearest town" — it is **not** a failure,
/// and a caller that treats it as a map id warps the character to field 999999999, which does
/// not exist.
pub fn destination(item_id: u32) -> Option<u32> {
    SCROLLS.iter().find(|(id, _)| *id == item_id).map(|(_, to)| *to)
}

/// Is this item one of the ten?
pub fn is_return_scroll(item_id: u32) -> bool {
    destination(item_id).is_some()
}

/// Which continent a map id is on, from its own id alone.
///
/// **This is the band form of `String.wz/Map.img`'s partition and it is scored against it.**
/// See the module doc §2: 432 named maps, 0 disagreements, checked by
/// `tools/dump_returnscrolls.py` on every run and by
/// [`the_band_rule_still_matches_the_client`] in this file.
///
/// `None` means "no continent", which is a real answer for `Other`, `Event` and `Dev` — and
/// also for a map id this client does not have. Use [`continent_of`] rather than this, unless
/// you specifically want "the field's own group with no fallback".
pub fn continent_of_map(map: u32) -> Option<Continent> {
    match map {
        0..=999_999 => Some(Continent::MapleIsland),
        10_000_000..=19_999_999 => Some(Continent::VictoriaIsland),
        20_000_000..=29_999_999 => Some(Continent::Ossyria),
        _ => None,
    }
}

/// Which continent a character standing on `map` counts as being on.
///
/// `anchor` is the field's nearest-town anchor — `Config::revive_field(map)`, the `reviveMap`
/// column. It is consulted **only when the field itself has no continent**, which is the
/// eleven `Other` and unnamed fields listed in the module doc §3. A field with a continent of
/// its own keeps it, so the two `To Orbis` ship cabins stay `Ossyria` rather than becoming
/// `VictoriaIsland` because they anchor to `Ellinia Station`. [D]
///
/// One hop only, and that is not a shortcut: `reviveMap` is already a walked, cycle-free
/// column (`research/return-maps.md` §5), so a second hop could only follow it out of a town.
pub fn continent_of(map: u32, anchor: Option<u32>) -> Option<Continent> {
    continent_of_map(map).or_else(|| anchor.and_then(continent_of_map))
}

/// What a use of this item on this field should do.
///
/// Deliberately **not** `Result`: `NotAScroll` is not an error, it is "ask the potion table
/// instead", and it is the only variant that must not produce a message on screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Not one of the ten. The caller falls through to whatever else uses items.
    NotAScroll,
    /// Consume the item and send the character to `to`. `why` is the log line.
    Teleport { to: u32, why: String },
    /// Refuse. **The item is not consumed**, and `why` completes the sentence
    /// "That item could not be used: …".
    Refused { why: String },
}

/// Decide what a Return Scroll use does, without performing any part of it.
///
/// `field_exists` is a **required argument rather than a caller's afterthought**, and that is
/// the point of the signature. `session::gm_map` refuses a map with no field image in
/// `gm-handbook/fields.txt` because a bad id strands or kills the client, and the owner lost a
/// session to exactly that on 2026-08-20. A teleport built here must not be weaker than the
/// one built there, so the destination cannot be produced without the check having been run.
///
/// `nearest_town` is `Config::revive_field(from_map)` — `None` when the map has no row, which
/// means the table is missing or the id is not a field this client ships. Both refuse rather
/// than guess a town: a `!map`-invented id has no defensible "nearest town" and picking one
/// would warp a character somewhere nobody asked for.
///
/// **Nothing is consumed, moved or sent by this function.** It returns an intention, so that
/// every effect in the caller hangs off the single [`Outcome::Teleport`] match arm.
pub fn resolve(
    item_id: u32,
    from_map: u32,
    nearest_town: Option<u32>,
    field_exists: impl Fn(u32) -> bool,
) -> Outcome {
    let Some(move_to) = destination(item_id) else {
        return Outcome::NotAScroll;
    };

    // --- "Nearest Town": the destination is a property of the field, not of the item ------
    //
    // No continent check on this path, and it needs none: the answer came out of the field
    // the character is standing on, so it is on that field's own continent by construction.
    if move_to == NO_MAP {
        let Some(town) = nearest_town else {
            return Outcome::Refused {
                why: format!(
                    "map {from_map} has no row in the return-map table, so this server does \
                     not know its nearest town. Regenerate with: python tools/dump_returnmaps.py"
                ),
            };
        };
        if town == NO_MAP || town == 0 {
            return Outcome::Refused {
                why: format!("map {from_map}'s nearest town resolved to nothing ({town})"),
            };
        }
        if !field_exists(town) {
            return Outcome::Refused {
                why: format!(
                    "map {from_map}'s nearest town is {town}, which has no field image in \
                     this client - warping there would strand you"
                ),
            };
        }
        return Outcome::Teleport {
            to: town,
            why: format!(
                "Return Scroll - Nearest Town: map {from_map} anchors to {town} \
                 (gm-handbook/returnmaps.txt reviveMap)"
            ),
        };
    }

    // --- a named town --------------------------------------------------------------------
    if !field_exists(move_to) {
        return Outcome::Refused {
            why: format!(
                "this scroll points at map {move_to}, which has no field image in this client \
                 - warping there would strand you"
            ),
        };
    }
    let here = continent_of(from_map, nearest_town);
    let there = continent_of(move_to, None);
    match (here, there) {
        (Some(a), Some(b)) if a == b => Outcome::Teleport {
            to: move_to,
            why: format!("Return Scroll to map {move_to}: both on {}", a.name()),
        },
        (Some(a), Some(b)) => Outcome::Refused {
            why: format!(
                "that scroll goes to {}, and you are on {} - a return scroll does not cross \
                 continents",
                b.name(),
                a.name()
            ),
        },
        (None, _) => Outcome::Refused {
            why: format!(
                "map {from_map} is not on any continent this server can name, so it cannot \
                 tell whether that scroll's destination is on the same one"
            ),
        },
        (_, None) => Outcome::Refused {
            why: format!(
                "this scroll's destination, map {move_to}, is not on any continent this \
                 server can name"
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Every field id this client ships, for the `field_exists` argument in a unit test.
    /// The real one is `Config::map_exists`; these are the handful the tests warp between.
    fn shipped(map: u32) -> bool {
        [
            1, 10, 20, 21, 30, 40, 60, 61, 1010, 10_000_000, 10_001_000, 10_002_000,
            10_002_070, 10_003_000, 10_004_000, 10_005_000, 10_005_078, 10_006_000,
            20_000_000, 20_001_000, 20_001_064, 80_001_100, 80_002_000, 88_000_000,
        ]
        .contains(&map)
    }

    /// Nothing exists. Used to prove the guard is not decorative.
    fn nothing(_: u32) -> bool {
        false
    }

    // ---- the two cross-checks against the client's own data ------------------------------

    /// `gm-handbook/returnscrolls.txt`, or `None` if it has not been generated on this
    /// machine.
    ///
    /// Returning `None` rather than failing matches `session::tests`' handling of
    /// `returnmaps.txt`: `gm-handbook/` is gitignored generated data and a checkout without it
    /// must still run the suite. Both callers **print** when they skip, because a test that
    /// silently passes by not running is the instrument this project keeps warning about.
    fn generated() -> Option<String> {
        for path in ["gm-handbook/returnscrolls.txt", "../../gm-handbook/returnscrolls.txt"] {
            if let Ok(text) = std::fs::read_to_string(path) {
                return Some(text);
            }
        }
        println!(
            "SKIPPED: gm-handbook/returnscrolls.txt is not on this machine. Generate it with: \
             python tools/dump_returnscrolls.py"
        );
        None
    }

    /// The baked [`SCROLLS`] table against the one the client's `Item.wz` actually contains.
    ///
    /// This is the whole reason the generated file exists. A constant that came from reading
    /// data is a claim, and `CLAUDE.md` is explicit that it has to be asserted against
    /// something that can disagree.
    #[test]
    fn the_baked_table_still_matches_the_client() {
        let Some(text) = generated() else { return };
        let mut from_file: Vec<(u32, u32)> = Vec::new();
        for line in text.lines() {
            let line = line.trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 3 || f[0] != "scroll" {
                continue;
            }
            from_file.push((f[1].parse().unwrap(), f[2].parse().unwrap()));
        }
        assert!(
            !from_file.is_empty(),
            "the generated file parsed to zero scroll rows - the reader is broken, which \
             would make this test pass by finding nothing"
        );
        from_file.sort_unstable();
        let mut baked = SCROLLS.to_vec();
        baked.sort_unstable();
        assert_eq!(
            baked, from_file,
            "SCROLLS has drifted from the client. Re-run: python tools/dump_returnscrolls.py"
        );
    }

    /// [`continent_of_map`]'s three ranges against all 432 of `String.wz/Map.img`'s rows.
    ///
    /// The band rule is **[D]** - fitted, not read - and this is what keeps it honest. It also
    /// asserts the two halves separately: every named map in one of the three real groups is
    /// classified, and every named map in `Other`/`Event`/`Dev` comes back `None`. A rule that
    /// answered `Some` for everything would pass the first half alone.
    #[test]
    fn the_band_rule_still_matches_the_client() {
        let Some(text) = generated() else { return };
        let (mut checked, mut none_checked) = (0usize, 0usize);
        for line in text.lines() {
            let line = line.trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 3 || f[0] != "map" {
                continue;
            }
            let map: u32 = f[1].parse().unwrap();
            let group = f[2];
            match continent_of_map(map) {
                Some(c) => {
                    assert_eq!(c.group(), group, "map {map}");
                    checked += 1;
                }
                None => {
                    assert!(
                        !["MapleIsland", "VictoriaIsland", "Ossyria"].contains(&group),
                        "map {map} is {group} in String.wz and the band rule calls it no \
                         continent at all"
                    );
                    none_checked += 1;
                }
            }
        }
        assert!(checked > 300, "only {checked} rows classified - the reader found almost \
                                nothing and this test proved nothing");
        assert!(none_checked > 0, "no Other/Event/Dev rows were seen, so the negative half \
                                   of this test never ran");
    }

    /// `Victoria Road` is the row that says the partition is data and not arithmetic.
    ///
    /// Its **name** says Victoria Island; `String.wz` files it under `Other` with the Free
    /// Market. This asserts against the generated file rather than against a literal, so it
    /// cannot pass by agreeing with a constant written next to it.
    #[test]
    fn the_control_row_is_grouped_by_the_client_and_not_by_its_name() {
        let Some(text) = generated() else { return };
        let mut groups: HashMap<u32, String> = HashMap::new();
        for line in text.lines() {
            let f: Vec<&str> = line.trim_end().split('\t').collect();
            if f.len() >= 3 && f[0] == "map" {
                if let Ok(id) = f[1].parse::<u32>() {
                    groups.insert(id, f[2].to_string());
                }
            }
        }
        assert_eq!(
            groups.get(&88_000_000).map(String::as_str),
            Some("Other"),
            "88000000 Victoria Road is grouped Other by the client, whatever its name says"
        );
        assert_eq!(groups.get(&80_002_000).map(String::as_str), Some("Other"));
        assert_eq!(groups.get(&10_001_000).map(String::as_str), Some("VictoriaIsland"));
        // ...and the six unnamed field images are in neither, which is why the anchor
        // fallback in `continent_of` is not optional.
        assert!(groups.get(&80_003_000).is_none(), "80003000 has no String.wz row");
        assert_eq!(continent_of_map(88_000_000), None, "and the band rule agrees");
        assert_eq!(continent_of_map(80_003_000), None);
    }

    // ---- the table ----------------------------------------------------------------------

    #[test]
    fn ten_scrolls_one_of_which_is_the_sentinel() {
        assert_eq!(SCROLLS.len(), 10);
        assert_eq!(SCROLLS.iter().filter(|(_, to)| *to == NO_MAP).count(), 1);
        assert_eq!(destination(2_030_009), Some(20_001_000), "El Nath");
        assert_eq!(destination(2_030_000), Some(NO_MAP), "Nearest Town");
        assert!(is_return_scroll(2_030_004));
        assert!(!is_return_scroll(2_000_000), "the Red Potion is not a scroll");
        assert_eq!(destination(2_000_000), None);
    }

    /// Nine literal destinations, and every one of them a real town centre on a real
    /// continent. Seven Victoria, two Ossyria, **none on Maple Island**.
    #[test]
    fn every_literal_destination_is_a_town_on_a_named_continent() {
        let mut by_continent: HashMap<&str, usize> = HashMap::new();
        for (item, to) in SCROLLS {
            if to == NO_MAP {
                continue;
            }
            let c = continent_of_map(to)
                .unwrap_or_else(|| panic!("item {item} points at {to}, no continent"));
            *by_continent.entry(c.name()).or_default() += 1;
        }
        assert_eq!(by_continent.get("Victoria Island"), Some(&7));
        assert_eq!(by_continent.get("Ossyria"), Some(&2));
        assert_eq!(
            by_continent.get("Maple Island"),
            None,
            "this client ships no Maple Island return scroll, which is why every literal \
             scroll is refused on the only continent the server can walk to today"
        );
    }

    // ---- the continent rule --------------------------------------------------------------

    #[test]
    fn the_three_bands_and_the_three_non_continents() {
        assert_eq!(continent_of_map(1), Some(Continent::MapleIsland));
        assert_eq!(continent_of_map(1021), Some(Continent::MapleIsland));
        assert_eq!(continent_of_map(10_001_000), Some(Continent::VictoriaIsland));
        assert_eq!(continent_of_map(10_007_040), Some(Continent::VictoriaIsland));
        assert_eq!(continent_of_map(20_001_075), Some(Continent::Ossyria));
        assert_eq!(continent_of_map(80_002_000), None, "Free Market");
        assert_eq!(continent_of_map(90_000_000), None, "Maple Hill, an event field");
        assert_eq!(continent_of_map(900_000_000), None, "White Map, a test field");
    }

    /// The fallback, on the field that motivated it. Without it a Return Scroll to Ellinia is
    /// refused inside `Magician's Tree Dungeon`, which sits on Victoria Island.
    #[test]
    fn a_field_with_no_group_takes_its_continent_from_its_anchor() {
        assert_eq!(continent_of_map(80_001_100), None, "the field itself has no group");
        assert_eq!(
            continent_of(80_001_100, Some(10_002_000)),
            Some(Continent::VictoriaIsland),
            "Magician's Tree Dungeon anchors to Ellinia"
        );
        // One of the six field images with no String.wz row at all.
        assert_eq!(continent_of(80_003_000, Some(10_006_000)), Some(Continent::VictoriaIsland));
        // A field WITH a group keeps it, anchor or no anchor: the To Orbis cabin stays
        // Ossyria even though it anchors to Ellinia Station.
        assert_eq!(
            continent_of(20_000_022, Some(10_002_090)),
            Some(Continent::Ossyria),
            "the fallback must not override a field that has a continent of its own"
        );
        // And no anchor at all is still no continent, rather than a guess.
        assert_eq!(continent_of(80_002_000, None), None);
        assert_eq!(continent_of(80_002_000, Some(80_002_000)), None, "Free Market anchors \
                    to itself, which is still not a continent");
        // The sixth unnamed field, and the reason the count in the doc block is 5 and not 6:
        // `80003500`'s returnMap is itself, so the fallback has nothing to fall back to.
        assert_eq!(continent_of(80_003_500, Some(80_003_500)), None);
    }

    // ---- resolve -------------------------------------------------------------------------

    #[test]
    fn a_potion_is_not_a_scroll_and_produces_no_message() {
        assert_eq!(resolve(2_000_000, 40, Some(60), shipped), Outcome::NotAScroll);
    }

    /// The owner's own example, and the reason for the whole continent rule.
    #[test]
    fn an_el_nath_scroll_is_refused_on_victoria_island() {
        let out = resolve(2_030_009, 10_001_000, Some(10_001_000), shipped);
        let Outcome::Refused { why } = out else { panic!("expected a refusal, got {out:?}") };
        assert!(why.contains("Ossyria"), "{why}");
        assert!(why.contains("Victoria Island"), "{why}");
    }

    #[test]
    fn the_same_scroll_works_while_standing_on_ossyria() {
        let out = resolve(2_030_009, 20_001_064, Some(20_001_000), shipped);
        assert_eq!(
            out,
            Outcome::Teleport {
                to: 20_001_000,
                why: "Return Scroll to map 20001000: both on Ossyria".to_string()
            },
            "Dead Mine I is on Ossyria and El Nath is its own island's town"
        );
    }

    /// The mirror of the owner's example, and the case a player will actually hit first: there is
    /// no Maple Island scroll, so on Maple Island every literal scroll is refused.
    #[test]
    fn every_literal_scroll_is_refused_on_maple_island() {
        for (item, to) in SCROLLS {
            if to == NO_MAP {
                continue;
            }
            let out = resolve(item, 40, Some(60), shipped);
            assert!(
                matches!(out, Outcome::Refused { .. }),
                "item {item} -> {to} should be refused on map 40, got {out:?}"
            );
        }
    }

    #[test]
    fn the_nearest_town_scroll_uses_the_maps_own_anchor() {
        assert_eq!(
            resolve(2_030_000, 40, Some(60), shipped),
            Outcome::Teleport {
                to: 60,
                why: "Return Scroll - Nearest Town: map 40 anchors to 60 \
                      (gm-handbook/returnmaps.txt reviveMap)"
                    .to_string()
            }
        );
        // And it crosses no continent by construction, so it works where a literal scroll
        // does not - including out of a field with no continent of its own.
        assert!(matches!(
            resolve(2_030_000, 80_001_100, Some(10_002_000), shipped),
            Outcome::Teleport { to: 10_002_000, .. }
        ));
    }

    /// The nearest-town scroll with nothing to resolve against. It must refuse, not warp to
    /// the sentinel and not pick a default town.
    #[test]
    fn a_map_with_no_return_row_refuses_rather_than_guessing() {
        let out = resolve(2_030_000, 40, None, shipped);
        let Outcome::Refused { why } = out else { panic!("expected a refusal, got {out:?}") };
        assert!(why.contains("dump_returnmaps.py"), "{why}");

        // The sentinel is not a map id, and a table that somehow carries it must not be
        // followed. 999999999 has no field image, so this is a second, independent guard.
        assert!(matches!(resolve(2_030_000, 40, Some(NO_MAP), shipped), Outcome::Refused { .. }));
        assert!(matches!(resolve(2_030_000, 40, Some(0), shipped), Outcome::Refused { .. }));
    }

    /// The guard `gm_map` has, on this path. `field_exists` is an argument rather than an
    /// afterthought precisely so this cannot be forgotten.
    #[test]
    fn a_destination_with_no_field_image_is_refused_on_both_paths() {
        let out = resolve(2_030_004, 10_001_090, Some(10_001_000), nothing);
        let Outcome::Refused { why } = out else { panic!("expected a refusal, got {out:?}") };
        assert!(why.contains("strand"), "{why}");

        let out = resolve(2_030_000, 40, Some(60), nothing);
        let Outcome::Refused { why } = out else { panic!("expected a refusal, got {out:?}") };
        assert!(why.contains("strand"), "{why}");
    }

    /// Standing somewhere with no continent and no useful anchor: a literal scroll is refused
    /// with a reason that names the problem, rather than being allowed through.
    #[test]
    fn a_field_on_no_continent_refuses_a_literal_scroll() {
        let out = resolve(2_030_004, 80_002_000, Some(80_002_000), shipped);
        let Outcome::Refused { why } = out else { panic!("expected a refusal, got {out:?}") };
        assert!(why.contains("not on any continent"), "{why}");
    }

    /// The same-map case, stated so it is a decision rather than an accident: using a Henesys
    /// scroll in Henesys succeeds and costs the scroll. It is the same transition as any other
    /// success and gets no special arm - see `session::consume`'s doc block for why refusing
    /// was rejected.
    #[test]
    fn a_scroll_used_in_its_own_town_is_a_normal_success() {
        assert!(matches!(
            resolve(2_030_004, 10_001_000, Some(10_001_000), shipped),
            Outcome::Teleport { to: 10_001_000, .. }
        ));
    }
}
