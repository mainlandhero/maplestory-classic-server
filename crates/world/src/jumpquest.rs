//! **The jump quests** - the Forest of Patience (Ellinia), the Deep Forest of Patience
//! (Sleepywood) and the Construction Site B1 to B3 (Kerning City's subway).
//!
//! `research/jump-quests-2026-10-04.md` has the evidence. In short:
//!
//! * **Every NPC in the three courses names a server-side script.** The client has no script
//!   bodies, and **no map portal leads into any of the 22 maps** **[L]**.
//! * **The ways in:**
//!   * **Shane** in Ellinia, for the Forest of Patience;
//!   * **the Mysterious Statue** in Sleepywood, for the Deep Forest;
//!   * **Jake's tickets** at the Subway Ticketing Booth, used at **the Ticket Gate**, for the
//!     Construction Site.
//! * **The ways out:**
//!   * **the NPC at the end of each course**, which warps the player back to town;
//!   * **Louis**, **the Crumbling Statue** and **the Exits**, which stand all through the
//!     courses for a player who gives up.
//!
//! The owner's calls, 2026-10-04:
//!
//! * **Entry.** Shane lets a player in **free**. He still refuses a stranger who has never
//!   taken Sabitrama's errand, which is his own `d0` line. *"Allow players to choose which part
//!   of the forest they would like to go depending on which flower they choose to hunt for"* -
//!   so Shane and the Statue both offer a menu of the courses.
//! * **Rewards.** *"Forest and Deep Forest can give random scrolls and consumables, it should
//!   function similar to the Companion PQ box, and should be exposed on the drops webpage as
//!   'Jump Quest Reward' in addition to completing the quest by either giving the required
//!   item"*.
//!   * Every finished course gives one prize from each of [`SLOTS`]. Those are the Companion's
//!     Magic Box's own **use** and **scroll** slots.
//!   * The course's quest item is added when that quest is in progress.
//!   * The Construction Site's chests included - the owner, the same day: *"the chest also
//!     gives jump quest rewards in addition to the quest item."*
//! * **Tickets.** Jake: B1 at level 20 for 500 mesos, B2 at 30 for 1 200, B3 at 40 for 2 000
//!   (the old GMS script, as recalled - accepted by the owner).
//!
//! Nothing here authenticates: a course is entered by whoever holds the socket.

use crate::magicbox::{Prize, Slot};

/// Shane - Ellinia, the Forest of Patience's door.
pub const SHANE: u32 = 306;
/// Louis - the bottom of every Forest of Patience step: *"If you want to leave this place,
/// come talk to me."*
pub const LOUIS: u32 = 314;
/// The pile of flowers at the top of Forest step 2, and the pile of herbs at the top of step 5.
pub const FLOWER_PILE: u32 = 315;
pub const HERB_PILE: u32 = 316;
/// The Mysterious Statue - Sleepywood, the Deep Forest's door.
pub const MYSTERIOUS_STATUE: u32 = 609;
/// The Crumbling Statue - the bottom of every Deep Forest step.
pub const CRUMBLING_STATUE: u32 = 610;
/// The pink, blue and white flower piles - the tops of Deep Forest steps 2, 4 and 7.
pub const PINK_PILE: u32 = 611;
pub const BLUE_PILE: u32 = 612;
pub const WHITE_PILE: u32 = 613;
/// Jake, who sells the tickets, and the Ticket Gate - both at the Subway Ticketing Booth.
pub const JAKE: u32 = 417;
pub const TICKET_GATE: u32 = 418;
/// The Exit - all through the Construction Site.
pub const EXIT: u32 = 419;
/// The Treasure Chests in the B1, B2 and B3 Subway Depots.
pub const CHEST_B1: u32 = 420;
pub const CHEST_B2: u32 = 421;
pub const CHEST_B3: u32 = 422;

pub const ELLINIA: u32 = 10_002_000;
pub const SLEEPYWOOD: u32 = 10_005_000;
pub const TICKET_BOOTH: u32 = 10_003_060;

/// **Where each way out lands**: a named spot beside the NPC that let the player in. All three
/// are target-less, script-less portals that nothing else uses **[L]**: Ellinia `herb` (290,
/// -2884) beside Shane, Sleepywood `forest00` (1102, 254) beside the Statue, and the Booth's
/// `out01` (287, 186) beside the Ticket Gate.
pub const ELLINIA_LANDING: (u32, &str) = (ELLINIA, "herb");
pub const SLEEPYWOOD_LANDING: (u32, &str) = (SLEEPYWOOD, "forest00");
pub const BOOTH_LANDING: (u32, &str) = (TICKET_BOOTH, "out01");

/// Which of the three courses a map belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area {
    Forest,
    DeepForest,
    ConstructionSite,
}

/// `None` for every map outside the 22.
pub fn area_of(map: u32) -> Option<Area> {
    match map {
        10_002_040..=10_002_044 => Some(Area::Forest),
        10_005_040..=10_005_046 => Some(Area::DeepForest),
        10_003_100..=10_003_109 => Some(Area::ConstructionSite),
        _ => None,
    }
}

impl Area {
    /// Where its way out lands.
    pub fn landing(self) -> (u32, &'static str) {
        match self {
            Area::Forest => ELLINIA_LANDING,
            Area::DeepForest => SLEEPYWOOD_LANDING,
            Area::ConstructionSite => BOOTH_LANDING,
        }
    }

    /// The NPC who shows a player out partway.
    pub fn warden(self) -> u32 {
        match self {
            Area::Forest => LOUIS,
            Area::DeepForest => CRUMBLING_STATUE,
            Area::ConstructionSite => EXIT,
        }
    }

    /// What the warden asks. The words are this server's.
    pub fn leave_question(self) -> &'static str {
        match self {
            Area::Forest => "Had enough of the forest? I can take you back to #bEllinia#k. Do you want to leave?",
            Area::DeepForest => {
                "Once I lay my hand on the statue, a strange light covers me and it feels like I'm being pulled \
                 back to where I came from. Do I want to go back to #bSleepywood#k?"
            }
            Area::ConstructionSite => "Do you want to leave the construction site and go back to the #bSubway Ticketing Booth#k?",
        }
    }
}

/// One course a door NPC offers: the `#L` line, what it is hunted for, which steps, and where
/// it starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Course {
    pub line: u32,
    pub item: u32,
    pub steps: &'static str,
    pub start_map: u32,
}

/// **The Forest of Patience is two courses [L].** Step 2 has no exit portal; its pile of
/// flowers ends the first. Step 3 is where the second begins, ending at step 5's herbs.
pub const FOREST_COURSES: [Course; 2] = [
    Course { line: 0, item: 4_031_025, steps: "Steps 1-2", start_map: 10_002_040 }, // Pink Anthurium
    Course { line: 1, item: 4_031_026, steps: "Steps 3-5", start_map: 10_002_042 }, // Double-Rooted Red Ginseng
];

/// **The Deep Forest is three [L]**: steps 2, 4 and 7 have no exit and a flower pile on top,
/// so 3 and 5 can only be started by a warp.
pub const DEEP_FOREST_COURSES: [Course; 3] = [
    Course { line: 0, item: 4_031_042, steps: "Steps 1-2", start_map: 10_005_040 }, // Pink Viola
    Course { line: 1, item: 4_031_043, steps: "Steps 3-4", start_map: 10_005_042 }, // Blue Viola
    Course { line: 2, item: 4_031_044, steps: "Steps 5-7", start_map: 10_005_044 }, // White Viola
];

/// **The end of a course**: the NPC standing there, its map, the quest it serves, that quest's
/// item and how many the quest wants at once, whether it also gives a [`SLOTS`] prize, and
/// what it is called in a sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Goal {
    pub npc: u32,
    pub map: u32,
    pub quest: u32,
    pub item: u32,
    pub count: u16,
    pub prize: bool,
    pub noun: &'static str,
}

/// The quest columns are `gm-handbook/quests.json` **[L]**. A quest's item counts are
/// handed in all at once (John wants 10, then 20, then 30), so one visit gives the whole
/// count, as the reference's `viola_*.py` do.
pub const GOALS: [Goal; 8] = [
    Goal { npc: FLOWER_PILE, map: 10_002_041, quest: 10_509, item: 4_031_025, count: 1, prize: true, noun: "pile of flowers" },
    Goal { npc: HERB_PILE, map: 10_002_044, quest: 10_510, item: 4_031_026, count: 1, prize: true, noun: "pile of herbs" },
    Goal { npc: PINK_PILE, map: 10_005_041, quest: 10_006, item: 4_031_042, count: 10, prize: true, noun: "pile of pink flowers" },
    Goal { npc: BLUE_PILE, map: 10_005_043, quest: 10_007, item: 4_031_043, count: 20, prize: true, noun: "pile of blue flowers" },
    Goal { npc: WHITE_PILE, map: 10_005_046, quest: 10_008, item: 4_031_044, count: 30, prize: true, noun: "pile of white flowers" },
    Goal { npc: CHEST_B1, map: 10_003_102, quest: 10_312, item: 4_031_039, count: 1, prize: true, noun: "treasure chest" },
    Goal { npc: CHEST_B2, map: 10_003_105, quest: 10_313, item: 4_031_040, count: 1, prize: true, noun: "treasure chest" },
    Goal { npc: CHEST_B3, map: 10_003_109, quest: 10_314, item: 4_031_041, count: 1, prize: true, noun: "treasure chest" },
];

pub fn goal_for(npc: u32) -> Option<&'static Goal> {
    GOALS.iter().find(|g| g.npc == npc)
}

/// **Shane opens the door to anyone who has taken Sabitrama's first errand** (10509, in
/// progress or done). A stranger gets his `d0`: *"I can't let some stranger like you enter my
/// property."*
pub const SHANE_KEY_QUEST: u32 = 10_509;

/// One of Jake's tickets: the `#L` line, the floor, the ticket, the level it needs, its price,
/// and the floor's Area 1, which is where the Ticket Gate sends its holder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ticket {
    pub line: u32,
    pub floor: &'static str,
    pub item: u32,
    pub min_level: u8,
    pub price: u32,
    pub area_one: u32,
}

pub const TICKETS: [Ticket; 3] = [
    Ticket { line: 0, floor: "Construction Site B1", item: 4_031_036, min_level: 20, price: 500, area_one: 10_003_100 },
    Ticket { line: 1, floor: "Construction Site B2", item: 4_031_037, min_level: 30, price: 1_200, area_one: 10_003_103 },
    Ticket { line: 2, floor: "Construction Site B3", item: 4_031_038, min_level: 40, price: 2_000, area_one: 10_003_106 },
];

/// **The Jump Quest Reward**: one prize from each slot, every finished course - the piles and
/// the chests alike. The Companion's Magic Box's own use and scroll slots, so the two tables cannot drift.
pub const SLOTS: [Slot; 2] = [
    Slot { name: "use", prizes: &crate::magicbox::USE },
    Slot { name: "scroll", prizes: &crate::magicbox::SCROLLS },
];

/// The row the drops page's seven-day counts keep the reward under, as though it were a
/// monster and a finished course a kill - beside the box's own [`crate::magicbox::BOX`]. Not a
/// mob, item or map id in this client: mob templates are four digits, maps eight.
pub const STATS_ROW: u32 = 999_000_001;

/// **How close a player must stand to take what is at the top.** The reference refuses a click
/// from 225-275 px away; these are looser, because the point is only that the bottom of the
/// step (over 3 000 px below every pile) cannot reach the top.
pub const REACH_X: i32 = 600;
pub const REACH_Y: i32 = 300;

pub fn within_reach(player: (i16, i16), npc: (i16, i16)) -> bool {
    (i32::from(player.0) - i32::from(npc.0)).abs() <= REACH_X && (i32::from(player.1) - i32::from(npc.1)).abs() <= REACH_Y
}

/// How many of a goal's quest items to hand over: what the quest still wants, while it is in
/// progress. A finished or untouched quest gets none, and nobody is topped past the count.
pub fn quest_item_owed(in_progress: bool, held: u32, count: u16) -> u16 {
    if !in_progress {
        return 0;
    }
    u16::try_from(u32::from(count).saturating_sub(held)).unwrap_or(0)
}

/// The conversation paths. One prefix, so `on_script_reply` can never take one for another
/// NPC's.
pub const PATH_PREFIX: &str = "jumpquest.";
pub const SHANE_PATH: &str = "jumpquest.shane";
pub const STATUE_PATH: &str = "jumpquest.statue";
pub const JAKE_PATH: &str = "jumpquest.jake";
pub const GATE_PATH: &str = "jumpquest.gate";
pub const LEAVE_PATH: &str = "jumpquest.leave";
/// `jumpquest.found:<npc>:<id>x<qty>,...` - the box saying what was found, and the prizes it
/// promised, which are handed over when it is dismissed.
pub const FOUND_PREFIX: &str = "jumpquest.found:";

pub fn found_path(npc: u32, prizes: &[Prize]) -> String {
    let list: Vec<String> = prizes.iter().map(|(id, q)| format!("{id}x{q}")).collect();
    format!("{FOUND_PREFIX}{npc}:{}", list.join(","))
}

pub fn parse_found_path(path: &str) -> Option<(u32, Vec<Prize>)> {
    let rest = path.strip_prefix(FOUND_PREFIX)?;
    let (npc, list) = rest.split_once(':')?;
    let npc = npc.parse().ok()?;
    let mut prizes = Vec::new();
    for p in list.split(',').filter(|p| !p.is_empty()) {
        let (id, q) = p.split_once('x')?;
        prizes.push((id.parse().ok()?, q.parse().ok()?));
    }
    Some((npc, prizes))
}

/// **What the box says was found.** The breaks are the two characters `\` `n`, which this
/// client renders as a line break (`crate::magicbox::lakelis_text`).
pub fn found_text(goal: &Goal, quest_item: Option<(u32, u16)>, prizes: &[(&str, Prize)], town: &str) -> String {
    if quest_item.is_none() && prizes.is_empty() {
        return format!(r"The {} is empty.\n\n(You will be taken back to {town}.)", goal.noun);
    }
    let verb = if goal.noun.ends_with("chest") { "open" } else { "search" };
    let mut text = format!("You {verb} the {} and find:", goal.noun);
    let amount = |q: u16| if q > 1 { format!(" x{q}") } else { String::new() };
    if let Some((id, q)) = quest_item {
        text.push_str(&format!(r"\n\n#i{id}# #b#t{id}##k{}", amount(q)));
    }
    if !prizes.is_empty() {
        text.push_str(r"\n");
    }
    for &(slot, (id, q)) in prizes {
        let mut label = slot.to_string();
        if let Some(c) = label.get_mut(..1) {
            c.make_ascii_uppercase();
        }
        text.push_str(&format!(r"\n#b{label}#k: #i{id}# #t{id}#{}", amount(q)));
    }
    text.push_str(&format!(r"\n\n(You will be taken back to {town}.)"));
    text
}

/// The name a landing is said with.
pub fn town_name(landing: (u32, &str)) -> &'static str {
    match landing.0 {
        ELLINIA => "Ellinia",
        SLEEPYWOOD => "Sleepywood",
        _ => "the Subway Ticketing Booth",
    }
}

/// Where a goal's way out lands.
pub fn goal_landing(goal: &Goal) -> (u32, &'static str) {
    area_of(goal.map).map_or(BOOTH_LANDING, Area::landing)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every goal stands on its own map in its own course, and leads back to that course's
    /// town; every course start and ticket floor is in the course it names.
    #[test]
    fn every_goal_and_door_is_in_its_own_course() {
        for g in &GOALS {
            let area = area_of(g.map).expect("a course map");
            let want = match g.npc {
                FLOWER_PILE | HERB_PILE => Area::Forest,
                PINK_PILE | BLUE_PILE | WHITE_PILE => Area::DeepForest,
                _ => Area::ConstructionSite,
            };
            assert_eq!(area, want, "{}", g.npc);
            assert!(g.prize, "the owner: every goal gives the reward, the chests included");
        }
        assert!(FOREST_COURSES.iter().all(|c| area_of(c.start_map) == Some(Area::Forest)));
        assert!(DEEP_FOREST_COURSES.iter().all(|c| area_of(c.start_map) == Some(Area::DeepForest)));
        assert!(TICKETS.iter().all(|t| area_of(t.area_one) == Some(Area::ConstructionSite)));
        assert_eq!(area_of(ELLINIA), None);
        assert_eq!(goal_landing(goal_for(PINK_PILE).unwrap()), SLEEPYWOOD_LANDING);
        assert_eq!(goal_landing(goal_for(CHEST_B3).unwrap()), BOOTH_LANDING);
        for (i, c) in FOREST_COURSES.iter().chain(&DEEP_FOREST_COURSES).enumerate() {
            let course = if i < 2 { &FOREST_COURSES[..] } else { &DEEP_FOREST_COURSES[..] };
            assert_eq!(course[c.line as usize], *c, "a line is its own index");
        }
    }

    /// **The courses are the client's [L]**: each goal's map is the last step of a course,
    /// and each course's quest item is its goal's. Read against the generated handbook; skipped
    /// when it is absent.
    #[test]
    fn the_goals_stand_where_the_client_puts_them() {
        let Ok(npcs) = std::fs::read_to_string("../../gm-handbook/npcs.txt") else { return };
        for g in &GOALS {
            let row = format!("{}, {}, ", g.map, g.npc);
            assert!(npcs.lines().any(|l| l.starts_with(&row)), "NPC {} is not on map {}", g.npc, g.map);
        }
        for (map, npc) in [(ELLINIA, SHANE), (SLEEPYWOOD, MYSTERIOUS_STATUE), (TICKET_BOOTH, JAKE), (TICKET_BOOTH, TICKET_GATE)] {
            assert!(npcs.lines().any(|l| l.starts_with(&format!("{map}, {npc}, "))), "{npc} on {map}");
        }
        let Ok(portals) = std::fs::read_to_string("../../gm-handbook/portals.txt") else { return };
        for (map, name) in [ELLINIA_LANDING, SLEEPYWOOD_LANDING, BOOTH_LANDING] {
            assert!(
                portals.lines().any(|l| l.starts_with(&format!("{map}, ")) && l.split(", ").nth(2) == Some(name)),
                "{name} on {map}"
            );
        }
        let course_items: Vec<u32> = FOREST_COURSES.iter().chain(&DEEP_FOREST_COURSES).map(|c| c.item).collect();
        let goal_items: Vec<u32> = GOALS.iter().filter(|g| area_of(g.map) != Some(Area::ConstructionSite)).map(|g| g.item).collect();
        assert_eq!(course_items, goal_items, "each course is hunted for its goal's item");
    }

    /// The quest gets what it still wants, while it is in progress, and never more.
    #[test]
    fn the_quest_item_is_topped_up_to_the_count_only_while_in_progress() {
        assert_eq!(quest_item_owed(true, 0, 10), 10);
        assert_eq!(quest_item_owed(true, 4, 10), 6);
        assert_eq!(quest_item_owed(true, 10, 10), 0);
        assert_eq!(quest_item_owed(true, 50, 10), 0);
        assert_eq!(quest_item_owed(false, 0, 10), 0, "finished or never taken: nothing");
    }

    #[test]
    fn reach_is_a_box_around_the_npc() {
        assert!(within_reach((146, -3626), (146, -3626)));
        assert!(within_reach((146 + 600, -3626 + 300), (146, -3626)));
        assert!(!within_reach((-455, 247), (146, -3626)), "Louis's spot at the bottom of step 5");
        assert!(!within_reach((146 + 601, -3626), (146, -3626)));
    }

    #[test]
    fn a_found_path_carries_its_prizes() {
        let p = found_path(PINK_PILE, &[(2_000_001, 100), (2_040_801, 1)]);
        assert!(p.starts_with(PATH_PREFIX));
        assert_eq!(parse_found_path(&p), Some((PINK_PILE, vec![(2_000_001, 100), (2_040_801, 1)])));
        assert_eq!(parse_found_path(&found_path(CHEST_B1, &[])), Some((CHEST_B1, vec![])));
        assert_eq!(parse_found_path("jumpquest.found:x:"), None);
        for path in [SHANE_PATH, STATUE_PATH, JAKE_PATH, GATE_PATH, LEAVE_PATH] {
            assert!(path.starts_with(PATH_PREFIX) && !path.starts_with(FOUND_PREFIX));
        }
    }

    /// The box names the quest item and every prize, with the escape the client renders and no
    /// real newline bytes; a goal with nothing at all for the player says it is empty.
    #[test]
    fn the_found_box_says_what_was_found() {
        let g = goal_for(PINK_PILE).unwrap();
        let t = found_text(g, Some((4_031_042, 10)), &[("use", (2_000_001, 100)), ("scroll", (2_040_801, 1))], "Sleepywood");
        assert!(t.starts_with("You search the pile of pink flowers and find:"), "{t}");
        assert!(t.contains(r"#i4031042# #b#t4031042##k x10"), "{t}");
        assert!(t.contains(r"\n#bUse#k: #i2000001# #t2000001# x100"), "{t}");
        assert!(t.contains(r"\n#bScroll#k: #i2040801# #t2040801#\n"), "{t}");
        assert!(t.ends_with(r"\n\n(You will be taken back to Sleepywood.)"), "{t}");
        assert!(!t.contains('\n') && !t.contains('\r'));
        let empty = found_text(goal_for(CHEST_B2).unwrap(), None, &[], "the Subway Ticketing Booth");
        assert!(empty.starts_with("The treasure chest is empty."), "{empty}");
        let coin = found_text(goal_for(CHEST_B1).unwrap(), Some((4_031_039, 1)), &[], "the Subway Ticketing Booth");
        assert!(coin.starts_with("You open the treasure chest and find:") && coin.contains("#t4031039##k\\n"), "{coin}");
    }

    /// Jake's prices and levels are the ones the owner accepted, and the reward's slots are the
    /// box's own lists.
    #[test]
    fn the_tickets_and_the_reward_are_the_owners() {
        assert_eq!(TICKETS.map(|t| (t.min_level, t.price)), [(20, 500), (30, 1_200), (40, 2_000)]);
        assert_eq!(TICKETS.map(|t| t.item), [4_031_036, 4_031_037, 4_031_038]);
        assert_eq!(SLOTS.map(|s| s.name), ["use", "scroll"]);
        assert_eq!(SLOTS[0].prizes, &crate::magicbox::USE[..]);
        assert_eq!(SLOTS[1].prizes, &crate::magicbox::SCROLLS[..]);
        assert_ne!(STATS_ROW, crate::magicbox::BOX);
    }
}
