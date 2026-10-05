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

/// **The goal at the end of the course `map` is part of.** An area's courses are runs of
/// consecutive maps, each ending at its goal's map, so it is the first goal of the same area
/// at or after `map`. `None` outside the 22 maps.
pub fn course_goal(map: u32) -> Option<&'static Goal> {
    let area = area_of(map)?;
    GOALS.iter().filter(|g| area_of(g.map) == Some(area) && g.map >= map).min_by_key(|g| g.map)
}

/// **The pity timer.** The owner, 2026-10-04: *"for all of the jump quests, start a 1 hour timer
/// (per player), this is the pity timer. When the player has expended all 1 hour of it, a yellow
/// notice text in chat will remind them every 5 minute that they have spent over an hour on
/// this jump quest, `!skipjq` will become available to them which removes them from the jump
/// quest instance, gives them the jump quest quest item only without the rewards themselves
/// such as consumable and scroll."*
///
/// The hour starts when a door (Shane, the Statue, the Ticket Gate) sends the player in, and
/// counts only time spent on that course while online (`store::jumpquest`).
///
/// **It is stopped only three ways** - the owner, 2026-10-04: *"The timer should only be stopped
/// when the player leaves the area via the exit NPC, finish the quest, or use the skip
/// command."* A disconnect keeps it, and a log in back on the course carries on from the time
/// already spent.
///
/// **A row left behind is removed only once its player is seen outside that course** - the
/// owner: *"Make sure the server does not destroy stale rows unless the player they are
/// tracking are no longer within the jump quest area."* That covers a return scroll, a death,
/// a GM warp, or a log in that lands somewhere else.
pub const PITY_SECS: u64 = 3_600;
/// One reminder at the hour, then one every five minutes.
pub const REMIND_SECS: u64 = 300;
/// The command, typed `!skipjq`. Open to everyone.
pub const SKIP_COMMAND: &str = "skipjq";

/// How many reminders `spent` seconds have earned: none before the hour, one at it, one more
/// every [`REMIND_SECS`] after.
pub fn notices_due(spent_secs: u64) -> u32 {
    if spent_secs < PITY_SECS {
        return 0;
    }
    u32::try_from(1 + (spent_secs - PITY_SECS) / REMIND_SECS).unwrap_or(u32::MAX)
}

/// The yellow reminder.
pub fn reminder_text(spent_secs: u64) -> String {
    format!(
        "You have spent over an hour on this jump quest ({} minutes). Type !{SKIP_COMMAND} to leave it with its quest item - \
         the other rewards stay at the top.",
        spent_secs / 60
    )
}

/// **This connection's share of the pity timer**: when it last looked, the time it has
/// counted and not yet written, and whether it has seen the character on a course. The total
/// lives in `store::jumpquest`; this only batches the writes, one per [`FLUSH_MS`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PityClock {
    pub next_check_ms: u64,
    pub last_ms: Option<u64>,
    pub pending_ms: u64,
    /// Set once this connection has ended any run while the character is off every course, so
    /// it does that once per stay off a course - a log in on a town map after logging out on a
    /// course included - rather than once a second.
    pub cleared: bool,
    /// A door has just started an hour: the next field entry explains the countdown
    /// ([`ENTRY_NOTICE`]), once.
    pub announce: bool,
}

/// How often the tick looks, and how much time it gathers before writing it.
pub const CHECK_MS: u64 = 1_000;
pub const FLUSH_MS: u64 = 10_000;

/// **What the countdown means**, in yellow on the first field entry after a door starts the
/// hour. The owner, 2026-10-04: *"upon entry, we should tell the player that "Time Left"
/// indicates timer until Jump Quest skip becomes active. Try their best, but they are allowed
/// to skip the jump quest after an hour of attempts."*
pub const ENTRY_NOTICE: &str = "The \"Time Left\" clock counts down to when you may skip this jump quest. Try your best - \
                                but after an hour of attempts you may type !skipjq to leave with its quest item.";

/// **Which of a door's courses to offer.** The owner, 2026-10-04: *"if they are currently on an
/// in progress quest, we should only show them the option of the one relevant to their quest.
/// Only after they completed all of the quest should we allow them to choose any of the jump
/// quest areas."*
///
/// And, the same day: *"When players are between two chains of quests, the server should only
/// offer the option that they have already completed (and not any new ones they haven't been
/// through the quests of)."*
///
/// `stages[i]` is where the player is on course `i`'s quest. In order:
/// * **any in progress** - only those courses;
/// * **some completed, not all** (between two quests of a chain) - only the completed ones;
/// * **all completed, or none touched** - every course. A player who never took the door's
///   quests is not a "player with quests", and the doors that take strangers (the Statue,
///   Jake, the gate) let them choose.
pub fn offered(stages: &[QuestStage]) -> Vec<usize> {
    let all = || (0..stages.len()).collect::<Vec<usize>>();
    let with = |want: QuestStage| (0..stages.len()).filter(|&i| stages[i] == want).collect::<Vec<usize>>();
    let in_progress = with(QuestStage::InProgress);
    if !in_progress.is_empty() {
        return in_progress;
    }
    let done = with(QuestStage::Complete);
    if done.is_empty() || done.len() == stages.len() {
        return all();
    }
    done
}

/// Where a player is on one course's quest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestStage {
    NotTaken,
    InProgress,
    Complete,
}

/// **The Henesys Pet Park's jump quest** - the Pet-Walking Road (10001052), walked into from
/// Henesys Park through `in01`. The owner, 2026-10-04: *"we missed a "non-quest" jump quest which
/// is the henesys pet park jump quest ... this should reward pet closeness along with jump quest
/// rewards when the player has brought the letter up to the top."*
///
/// [L] from the client:
/// * Trainer Bartos (222) stands at the bottom (-2108, 236) and Trainer Frod (223) at the top
///   (-1593, -1588); both name server scripts.
/// * Bartos's Letter (4031035) is *"A letter from Bartos the Instructor. Needs to be delivered
///   to Trainer Frod."*
/// * No quest uses the letter, so this course has **no pity timer** (the owner).
///
/// [R] for the flow (`pet_lifeitem.py`, `pet_letter.py`):
/// * Bartos hands over the letter.
/// * Frod takes it and boosts the pet's closeness.
/// * Nobody is warped: the hidden `h005` beside Frod already leads back down to Bartos.
pub const PET_WALKING_ROAD: u32 = 10_001_052;
pub const BARTOS: u32 = 222;
pub const FROD: u32 = 223;
pub const BARTOS_LETTER: u32 = 4_031_035;
/// What Frod adds to the summoned pet's closeness - the owner's number, 2026-10-04: *"We should
/// reward 20 closeness instead."* (It had been 2, the old GMS script as recalled.)
pub const PET_PARK_CLOSENESS: u32 = 20;
pub const BARTOS_PATH: &str = "jumpquest.bartos";

/// Frod's box: his own words, the closeness, and the reward.
pub fn frod_text(prizes: &[(&str, Prize)]) -> String {
    let mut text = String::from(
        "Eh, that's my brother's letter! Ahhh... you followed my brother's advice and trained your pet and got up here, \
         huh? Nice!! Since you worked hard to get here, I'll boost your intimacy level with your pet.",
    );
    text.push_str(&format!(r"\n\n#bCloseness#k +{PET_PARK_CLOSENESS}"));
    for &(slot, (id, q)) in prizes {
        let mut label = slot.to_string();
        if let Some(c) = label.get_mut(..1) {
            c.make_ascii_uppercase();
        }
        let amount = if q > 1 { format!(" x{q}") } else { String::new() };
        text.push_str(&format!(r"\n#b{label}#k: #i{id}# #t{id}#{amount}"));
    }
    text
}

/// What `!skipjq` says before the hour is up.
pub fn not_yet_text(spent_secs: u64) -> String {
    let left = PITY_SECS.saturating_sub(spent_secs);
    let minutes = left.div_ceil(60).max(1);
    format!(
        "!{SKIP_COMMAND} becomes available after an hour on a jump quest. {minutes} more minute{} to go.",
        if minutes == 1 { "" } else { "s" }
    )
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

/// **How close a player must stand to take what is at the end of a course**: within 250 px of
/// the NPC, straight-line, both measured at the feet (the player's last reported position, the
/// NPC's foothold `cy`). The owner, 2026-10-04: *"It should require the player to be within 250
/// px of the NPC."* The reference's own checks are 225-275 px, one axis each.
pub const REACH_PX: i64 = 250;

pub fn within_reach(player: (i16, i16), npc: (i16, i16)) -> bool {
    let dx = i64::from(player.0) - i64::from(npc.0);
    let dy = i64::from(player.1) - i64::from(npc.1);
    dx * dx + dy * dy <= REACH_PX * REACH_PX
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
        for (map, npc) in [(ELLINIA, SHANE), (SLEEPYWOOD, MYSTERIOUS_STATUE), (TICKET_BOOTH, JAKE), (TICKET_BOOTH, TICKET_GATE), (PET_WALKING_ROAD, BARTOS), (PET_WALKING_ROAD, FROD)] {
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

    /// Every course map belongs to the goal at its end, and every door sends the player onto a
    /// course whose goal is the one it advertises.
    #[test]
    fn every_course_map_belongs_to_the_goal_at_its_end() {
        let expect = [
            (10_002_040..=10_002_041, FLOWER_PILE),
            (10_002_042..=10_002_044, HERB_PILE),
            (10_005_040..=10_005_041, PINK_PILE),
            (10_005_042..=10_005_043, BLUE_PILE),
            (10_005_044..=10_005_046, WHITE_PILE),
            (10_003_100..=10_003_102, CHEST_B1),
            (10_003_103..=10_003_105, CHEST_B2),
            (10_003_106..=10_003_109, CHEST_B3),
        ];
        for (maps, npc) in expect {
            for m in maps {
                assert_eq!(course_goal(m).map(|g| g.npc), Some(npc), "map {m}");
            }
        }
        assert_eq!(course_goal(ELLINIA), None);
        for c in FOREST_COURSES.iter().chain(&DEEP_FOREST_COURSES) {
            assert_eq!(course_goal(c.start_map).map(|g| g.item), Some(c.item), "{}", c.steps);
        }
        assert_eq!(TICKETS.map(|t| course_goal(t.area_one).map(|g| g.npc)), [Some(CHEST_B1), Some(CHEST_B2), Some(CHEST_B3)]);
    }

    /// The owner's hour, then one reminder every five minutes.
    #[test]
    fn the_pity_timer_reminds_at_the_hour_and_every_five_minutes_after() {
        assert_eq!(notices_due(0), 0);
        assert_eq!(notices_due(3_599), 0);
        assert_eq!(notices_due(3_600), 1);
        assert_eq!(notices_due(3_899), 1);
        assert_eq!(notices_due(3_900), 2);
        assert_eq!(notices_due(3_600 + 300 * 10), 11);
        assert!(reminder_text(3_900).contains("65 minutes") && reminder_text(3_900).contains("!skipjq"));
        assert!(not_yet_text(0).contains("60 more minutes"), "{}", not_yet_text(0));
        assert!(not_yet_text(3_559).contains("1 more minute "), "{}", not_yet_text(3_559));
    }

    /// An in-progress quest narrows the menu to its own course; with none in progress every
    /// course is offered.
    #[test]
    fn a_quest_in_progress_narrows_the_door_to_its_course() {
        use QuestStage::{Complete as C, InProgress as P, NotTaken as N};
        assert_eq!(offered(&[C, P, N]), vec![1], "in progress: that course only");
        assert_eq!(offered(&[P, N]), vec![0]);
        assert_eq!(offered(&[C, N, N]), vec![0], "between the chain's quests: only what is completed");
        assert_eq!(offered(&[C, C, N]), vec![0, 1]);
        assert_eq!(offered(&[C, C, C]), vec![0, 1, 2], "all completed: every course");
        assert_eq!(offered(&[N, N, N]), vec![0, 1, 2], "never touched: every course");
        assert!(ENTRY_NOTICE.contains("\"Time Left\"") && ENTRY_NOTICE.contains("!skipjq") && ENTRY_NOTICE.contains("an hour"));
        let t = frod_text(&[("use", (2_000_001, 100)), ("scroll", (2_040_801, 1))]);
        assert!(t.contains(r"\n\n#bCloseness#k +20\n#bUse#k: #i2000001# #t2000001# x100\n#bScroll#k"), "{t}");
        assert_eq!(area_of(PET_WALKING_ROAD), None, "the road is not a pity-timer course");
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
        assert!(within_reach((146 + 250, -3626), (146, -3626)), "250 px across is in");
        assert!(within_reach((146, -3626 + 250), (146, -3626)), "250 px below is in");
        assert!(within_reach((146 + 150, -3626 + 200), (146, -3626)), "150 across and 200 down is exactly 250");
        assert!(!within_reach((146 + 251, -3626), (146, -3626)), "251 px is out");
        assert!(!within_reach((146 + 180, -3626 + 180), (146, -3626)), "180 and 180 is 255 straight-line: out");
        assert!(!within_reach((-455, 247), (146, -3626)), "Louis's spot at the bottom of step 5");
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
