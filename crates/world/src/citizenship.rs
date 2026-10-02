//! **Citizenship** - the rules, the record string, and the Community Board's postings.
//!
//! The owner, 2026-09-27: *"Great, make the implementation."* Everything here rests on
//! `research/citizenship-2026-09-27.md`; its labels carry over: **[L]** read off this client,
//! **[S]** the fan site only, **[D]** derived, **[I]** ours.
//!
//! # What the client does by itself, which is most of it
//!
//! The client keeps citizenship as quest 510000's ex record and **locks everything off it**:
//! quest start and completion (`st<town> == 1 && gr<town> >= required`, reason `0x50`), shop
//! rows, the NPCs' grade lines, the reward drawn in the quest window. So the server's job is
//! mostly to keep that one string right ([`record_value`]) and to refuse the same things the
//! client refuses, because nothing on the wire authenticates.
//!
//! # The Community Board - also the client, given one string per group
//!
//! `Quest/RecurringQuestGroup.img` puts the 71 board quests in four groups (18 + 17 + 18 + 18) **[L]**, and
//! `FUN_14070FAE0` - the client's general "may this quest start" check, 47 call sites, so the
//! quest list, the lightbulb and the Accept button all go through it - refuses any quest in a
//! group (reason `0x51`) unless the group's record lists it:
//!
//! ```text
//! group -> qrID (a quest) and qrKey (a key)
//! value  = quest qrID's ex record, looked up under qrKey      (FUN_1402E0240 -> FUN_1401938F0)
//! posted = value split on '|', each compared with "%d" of the quest id
//! ```
//!
//! So Henesys's daily board is quest 510001 = `q1_d=506005|506006`. **Nothing is posted that
//! is not in that string** - which is the owner's first rule (*"If the player has never done them,
//! do not show the quest as available for pick up"*) enforced by the client itself.
//!
//! A board quest the character has already **completed** goes through the start check's
//! completed-quest path, `FUN_14070FE30` at `0x1407104D5` **[L]**, and the client re-offers
//! it on its own terms:
//!
//! | | client says | reason |
//! |---|---|---|
//! | in the group's `doNotRepeat` | never again | `0x19` |
//! | daily group, completed today | not yet | `0x13` |
//! | weekly group, completed this week (weeks start **Monday**: `8 - weekday`) | not yet | `0x16` |
//! | otherwise | may start again | - |
//!
//! Which is the owner's other two rules (*"If the player has completed the quest, remain in the
//! completed tab until it is chosen again ... If the quest was previously completed by the
//! player, they become active again in the available quests"*): the completion stays in the
//! completed map, and posting it again is what brings it back. [`may_repeat`] is the server's
//! copy of that table, and `store::restart_quest` is the one way a completed row moves back.
//!
//! # What gets posted - ours, and the site's rules [S]
//!
//! `selectCount` is 1 in all four groups **[L]**. The data pairs the dailies: every odd quest is
//! a *First Greeting* with no prerequisite and sits in `doNotRepeat`; every even one is *Asking
//! After* the same resident and requires the odd one done **[L]**. So one daily pick is one
//! **resident** - both ids posted - and the client shows whichever half applies: a newcomer gets
//! the greeting (once, ever), everyone after gets the check-in.
//!
//! * **Daily:** one resident a day - a grade-1 resident, or at grade 5+ (and level 32, the
//!   level the leaders' check-ins need) one of the town leaders instead.
//! * **Weekly:** one donation a week, **at the highest tier the character reaches** - each
//!   donation is gated at a grade and pays `500 + 250 (g - 1)` **[L]**, and the site's "one
//!   donation a week" at "500 at grade 1, +250 per grade" is exactly that.
//!
//! **A tier is reached by grade AND level.** The donations all *start* at level 12 but
//! *finish* at `12 + 5 (g - 1)` (`Check.1.lvmin`) **[L]**, and the client will not turn one in
//! below that. The owner, 2026-10-01, with a screenshot of a grade-3 posting a character could
//! not complete: *"player having access to a weekly that they do not fulfill the level
//! requirement for"*.
//!
//! **One per character per period, chosen when the period starts and kept** ([`settle`],
//! `store::citizenship`'s `board_pick`). The same screenshot had three donations in progress:
//! the posting was recomputed from the grade every time, so each grade-up mid-week posted the
//! next tier's donation beside the one already taken. The owner: *"Weeklies should only be
//! allowed once per character, and the highest level weekly at time of weekly reset is
//! allowed"*, and *"once the user completes a daily, even if they advance in citizen rank,
//! they should not be offered a new daily quest"*. So:
//!
//! * the period's pick is made the first time the character is seen in it, as an active
//!   citizen, and stored - a grade or a level gained later waits for the next period;
//! * a board quest still in progress when the period turns **is** the pick (no second one);
//! * once one of the group's quests is turned in, the group posts nothing until the period
//!   turns - a daily's First Greeting included, so its check-in waits too;
//! * extra in-progress quests from before this rule (and any the character cannot finish at
//!   their level) are given up when the pick is made: the highest finishable one is kept.
//!
//! The picks walk a shuffled order of each tier, reshuffled every cycle, so every resident and
//! every donation comes round once per cycle. Seeded from the period number only - every
//! channel is its own process and they must all post the same thing without talking. The
//! per-character part is only which tier, and when.
//!
//! Days are **UTC**, like the Maple Administrator's (`store::dailyperks`).

use store::citizenship::{TownStanding, STATE_ACTIVE, STATE_FROZEN};

/// Henesys. **[L]** `FUN_1402C8AF0` accepts towns 1 and 2 only.
pub const HENESYS: u8 = 1;
/// Kerning City.
pub const KERNING_CITY: u8 = 2;

/// One town's fixed facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Town {
    pub id: u8,
    pub name: &'static str,
    /// The Town Clerk who signs the contract: Arthur, Roxy. **[L]** placements.
    pub clerk: u32,
    /// The Community Board NPC.
    pub board: u32,
    /// The map the clerk stands on - the Town Hall / Civic Center.
    pub hall: u32,
}

pub const TOWNS: [Town; 2] = [
    Town { id: HENESYS, name: "Henesys", clerk: 229, board: 235, hall: 10_001_007 },
    Town { id: KERNING_CITY, name: "Kerning City", clerk: 425, board: 431, hall: 10_003_007 },
];

pub fn town(id: u8) -> Option<&'static Town> {
    TOWNS.iter().find(|t| t.id == id)
}

pub fn town_of_clerk(npc_template: u32) -> Option<&'static Town> {
    TOWNS.iter().find(|t| t.clerk == npc_template)
}

/// The town-hall shopkeepers, whose grade-tagged rows (`data/shops.txt`, every one of them in
/// these six shops) are gated on THAT town's citizenship. **[L]** placements: Raymond, Oak,
/// Flint, Tommy on 10001007; Max, Weston, Ben, Jack on 10003007.
pub const SHOP_NPCS: [(u32, u8); 8] = [
    (232, HENESYS),
    (233, HENESYS),
    (234, HENESYS),
    (230, HENESYS),
    (428, KERNING_CITY),
    (429, KERNING_CITY),
    (430, KERNING_CITY),
    (426, KERNING_CITY),
];

pub fn town_of_shop(npc_template: u32) -> Option<u8> {
    SHOP_NPCS.iter().find(|(n, _)| *n == npc_template).map(|(_, t)| *t)
}

/// The level a character must be to sign. **[L]** 506000 / 506100 both `lvmin 12`, and the
/// site agrees.
pub const MIN_LEVEL: u32 = 12;

/// The ten grade names, **[L]** string pool `0x17CF..0x17D8` (and `data/shops.txt`'s tags).
pub const GRADE_NAMES: [&str; 10] = [
    "Traveler",
    "Visitor",
    "Helpful Stranger",
    "Recognized Guest",
    "Town Resident",
    "Trusted Neighbor",
    "Distinguished Citizen",
    "Town Patron",
    "Guardian of the Village",
    "Citizen of Honor",
];

pub const MAX_GRADE: u8 = 10;

/// **Contribution needed AT each grade to reach the next one**, index = grade - 1: 1,000 at
/// Traveler for Visitor, 2,000 at Visitor for Helpful Stranger ... 10,000 at Guardian of the
/// Village for Citizen of Honor. **[S]** - the client carries no table for it
/// (`research/citizenship-2026-09-27.md` §2.2).
///
/// **Per grade, not a running total**, and the contribution resets on promotion with the excess
/// kept. The owner, 2026-10-02, on why the points were "very off": *"From grade 1 to grade 2, it
/// requires 1000 contribution from when the player is in grade 1. After leveling to grade 2, the
/// contribution resets to 0 (player retains any excess that exceeds the grade limit), and it
/// requires 2000 contribution from the player when they are grade 2 ... From grade 1 to grade
/// 3, it means that total of 3000 contribution is required."* Until then the same numbers were
/// read as cumulative thresholds, so grade 3 came at 2,000 in total and grade 10 at 10,000
/// instead of 46,000. `store::citizenship` converted the stored totals once (see its docs).
pub const GRADE_REQUIREMENTS: [u32; 9] = [1_000, 2_000, 3_000, 4_000, 5_000, 6_000, 7_000, 8_000, 10_000];

/// The Reactivation fee, in mesos. **[S]** for grade 1 (the site's only figure); used for every
/// grade until a better number exists. Shown to the player in the contract window itself.
pub const REACTIVATION_FEE: u32 = 50_000;

/// **The Citizen of Honor earrings**, one per town: Henesys Earrings `1032021`, Kerning City
/// Earrings `1032022` - Lv 57, 42 MDEF, +2% crit damage, +2 avoid, 5 slots, untradeable, *"An
/// honorable earring awarded to the Citizen of Honor of each town."* **[L]** `equips.txt`, the
/// description [S]. The owner, 2026-09-29: *"When someone achieves that standing, they should
/// automatically receive the earring for those specific towns."*
pub fn honor_earring(town: u8) -> Option<u32> {
    match town {
        HENESYS => Some(1_032_021),
        KERNING_CITY => Some(1_032_022),
        _ => None,
    }
}

/// The server-wide congratulation - **the client's own sentence**, string `0x17D9`, which
/// nothing in this client loads (a byte scan for it as an immediate finds only jump
/// displacements, while the same scan finds `0x17CD` and `0x17DA` at their known readers).
pub fn honor_announcement(name: &str, town: u8) -> String {
    let town = crate::citizenship::town(town).map_or("their town", |t| t.name);
    format!("Let us all congratulate {name} for becoming a Citizen of Honor in {town}!")
}

pub fn grade_name(grade: u8) -> &'static str {
    GRADE_NAMES[usize::from(grade.clamp(1, MAX_GRADE) - 1)]
}

/// What the next grade needs, or `None` at the top.
pub fn next_threshold(grade: u8) -> Option<u32> {
    GRADE_REQUIREMENTS.get(usize::from(grade.max(1)) - 1).copied()
}

/// The active citizenship, if any. There is at most one - every writer keeps it that way.
pub fn active(towns: &[TownStanding]) -> Option<&TownStanding> {
    towns.iter().find(|t| t.is_active())
}

/// **Quest 510000's ex record**, exactly as the client parses it: `key=value` joined by `;`,
/// `st/gr/ct` + town, town order. **[L]** keys `FUN_1402C8870`, separators `FUN_140193890`.
/// Empty for a character who never signed - and then no record is sent at all.
pub fn record_value(towns: &[TownStanding]) -> String {
    let mut sorted: Vec<&TownStanding> = towns.iter().collect();
    sorted.sort_by_key(|t| t.town);
    sorted
        .iter()
        .map(|t| format!("st{0}={1};gr{0}={2};ct{0}={3}", t.town, t.state, t.grade, t.contribution))
        .collect::<Vec<_>>()
        .join(";")
}

/// Whether `towns` meets a quest's `Check.0.citizenshipTown/Grade`: the same rule as the
/// client's (`st == 1 && gr >= required`).
pub fn meets(towns: &[TownStanding], town: u8, grade: u8) -> bool {
    towns.iter().any(|t| t.town == town && t.is_active() && t.grade >= grade)
}

// ---------------------------------------------------------------------------------------
// Signing, transferring, reactivating, renouncing
// ---------------------------------------------------------------------------------------

/// What the Town Clerk of `town` offers this character. Pure; the session opens the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClerkOffer {
    /// Below level 12.
    TooLow,
    /// Never signed anywhere: the Oath.
    Oath,
    /// Active elsewhere and never signed here: the Transfer (the old town freezes).
    Transfer { old_town: u8 },
    /// Signed here before and frozen now: the Reactivation, for [`REACTIVATION_FEE`].
    Reactivation { grade: u8 },
    /// Already a citizen here, with a grade-up certificate owed.
    Certificate { grade: u8 },
    /// Already a citizen here, nothing owed: the Renunciation is what is left to offer.
    Renunciation { grade: u8 },
}

pub fn clerk_offer(level: u32, towns: &[TownStanding], here: u8) -> ClerkOffer {
    if level < MIN_LEVEL {
        return ClerkOffer::TooLow;
    }
    match towns.iter().find(|t| t.town == here) {
        Some(t) if t.is_active() && t.grade > t.certified_grade => ClerkOffer::Certificate { grade: t.grade },
        Some(t) if t.is_active() => ClerkOffer::Renunciation { grade: t.grade },
        Some(t) => ClerkOffer::Reactivation { grade: t.grade },
        None => match active(towns) {
            Some(elsewhere) => ClerkOffer::Transfer { old_town: elsewhere.town },
            None => ClerkOffer::Oath,
        },
    }
}

/// The standings after an accepted contract. `None` for a contract that does not apply to
/// these standings any more - re-checked at the answer, not trusted from the offer.
pub fn after_contract(towns: &[TownStanding], here: u8, offer: ClerkOffer) -> Option<Vec<TownStanding>> {
    let mut out: Vec<TownStanding> = towns.to_vec();
    let freeze_others = |v: &mut Vec<TownStanding>| {
        for t in v.iter_mut().filter(|t| t.town != here && t.is_active()) {
            t.state = STATE_FROZEN;
        }
    };
    match offer {
        ClerkOffer::Oath | ClerkOffer::Transfer { .. } => {
            if towns.iter().any(|t| t.town == here) {
                return None;
            }
            freeze_others(&mut out);
            out.push(TownStanding { town: here, state: STATE_ACTIVE, grade: 1, contribution: 0, certified_grade: 1 });
        }
        ClerkOffer::Reactivation { .. } => {
            let t = out.iter_mut().find(|t| t.town == here && !t.is_active())?;
            t.state = STATE_ACTIVE;
            freeze_others(&mut out);
        }
        ClerkOffer::Renunciation { .. } => {
            let t = out.iter_mut().find(|t| t.town == here && t.is_active())?;
            t.state = STATE_FROZEN;
        }
        ClerkOffer::Certificate { grade } => {
            let t = out.iter_mut().find(|t| t.town == here && t.is_active())?;
            t.certified_grade = t.certified_grade.max(grade);
        }
        ClerkOffer::TooLow => return None,
    }
    out.sort_by_key(|t| t.town);
    Some(out)
}

// ---------------------------------------------------------------------------------------
// Contribution
// ---------------------------------------------------------------------------------------

/// What a turn-in banks at `grade`: the flat amount, or the formula evaluated the way the
/// client draws it. `0` for a formula this evaluator cannot read - and the caller logs it.
pub fn contribution_for(contr: &crate::config::CitizenshipContr, grade: u8) -> u32 {
    if let Some(a) = contr.amount {
        return a;
    }
    contr
        .formula
        .as_deref()
        .and_then(|f| eval_formula(f, i64::from(grade)))
        .map_or(0, |v| u32::try_from(v.max(0)).unwrap_or(u32::MAX))
}

/// The client's little formula language: integers, `citizenshipGrade`, `+ - x * /`,
/// parentheses. **[L]** for the one string it has to read; the grammar is the obvious one and
/// only `"100 + ( ( citizenshipGrade - 1 ) x 50 )"` exists in this client's data.
pub fn eval_formula(src: &str, grade: i64) -> Option<i64> {
    let toks: Vec<String> = src
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let mut pos = 0;
    let v = expr(&toks, &mut pos, grade)?;
    (pos == toks.len()).then_some(v)
}

fn expr(t: &[String], p: &mut usize, g: i64) -> Option<i64> {
    let mut v = term(t, p, g)?;
    while let Some(op) = t.get(*p).map(String::as_str) {
        match op {
            "+" => { *p += 1; v = v.checked_add(term(t, p, g)?)?; }
            "-" => { *p += 1; v = v.checked_sub(term(t, p, g)?)?; }
            _ => break,
        }
    }
    Some(v)
}

fn term(t: &[String], p: &mut usize, g: i64) -> Option<i64> {
    let mut v = atom(t, p, g)?;
    while let Some(op) = t.get(*p).map(String::as_str) {
        match op {
            "x" | "*" => { *p += 1; v = v.checked_mul(atom(t, p, g)?)?; }
            "/" => { *p += 1; v = v.checked_div(atom(t, p, g)?)?; }
            _ => break,
        }
    }
    Some(v)
}

fn atom(t: &[String], p: &mut usize, g: i64) -> Option<i64> {
    let tok = t.get(*p)?.as_str();
    *p += 1;
    match tok {
        "(" => {
            let v = expr(t, p, g)?;
            (t.get(*p)?.as_str() == ")").then(|| *p += 1)?;
            Some(v)
        }
        "citizenshipGrade" => Some(g),
        n => n.parse().ok(),
    }
}

/// A contribution applied: the standing after, and the grade before, so the caller can tell a
/// grade-up (and only a grade-up) plays the effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Banked {
    pub after: TownStanding,
    pub grade_before: u8,
}

impl Banked {
    pub fn graded_up(&self) -> bool {
        self.after.grade > self.grade_before
    }
}

/// Promote a standing as far as its contribution reaches: each grade's requirement is taken off
/// and the excess carried, so one large turn-in can cross several grades. Never down, never
/// above ten; at ten the contribution simply keeps counting.
pub fn regrade(mut t: TownStanding) -> TownStanding {
    t.grade = t.grade.clamp(1, MAX_GRADE);
    while let Some(need) = next_threshold(t.grade) {
        if t.contribution < need {
            break;
        }
        t.contribution -= need;
        t.grade += 1;
    }
    t
}

// ---------------------------------------------------------------------------------------
// The Community Board
// ---------------------------------------------------------------------------------------

/// One `RecurringQuestGroup.img` entry. **[L]** loader `FUN_140720C10`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardGroup {
    pub town: u8,
    pub weekly: bool,
    /// `qrID` - the quest whose ex record carries the posting.
    pub record_quest: u32,
    /// `qrKey`.
    pub key: &'static str,
    /// `list`, contiguous in all four.
    pub first: u32,
    pub last: u32,
    /// `doNotRepeat`: every odd id of a daily group, none in a weekly one.
    pub odd_do_not_repeat: bool,
}

pub const BOARD_GROUPS: [BoardGroup; 4] = [
    BoardGroup { town: HENESYS, weekly: false, record_quest: 510_001, key: "q1_d", first: 506_001, last: 506_018, odd_do_not_repeat: true },
    BoardGroup { town: HENESYS, weekly: true, record_quest: 510_002, key: "q1_w", first: 506_019, last: 506_035, odd_do_not_repeat: false },
    BoardGroup { town: KERNING_CITY, weekly: false, record_quest: 510_003, key: "q1_d", first: 506_101, last: 506_118, odd_do_not_repeat: true },
    BoardGroup { town: KERNING_CITY, weekly: true, record_quest: 510_004, key: "q1_w", first: 506_119, last: 506_136, odd_do_not_repeat: false },
];

impl BoardGroup {
    pub fn contains(&self, quest_id: u32) -> bool {
        (self.first..=self.last).contains(&quest_id)
    }
    pub fn do_not_repeat(&self, quest_id: u32) -> bool {
        self.odd_do_not_repeat && self.contains(quest_id) && quest_id % 2 == 1
    }
    /// The period number `now` falls in: the UTC day, or the Monday-started UTC week.
    pub fn period(&self, unix_secs: i64) -> i64 {
        let day = store::dailyperks::utc_day(unix_secs);
        if self.weekly { week(day) } else { day }
    }
}

/// The Monday-started week a UTC day falls in. Day 0 (1970-01-01) was a Thursday, so day 4 is
/// the first Monday and starts week 1. The client's weekly gate is `8 - weekday` days from the
/// completion (weekday 0 = Sunday), which also turns over on Monday **[L]**.
pub fn week(utc_day: i64) -> i64 {
    (utc_day + 3).div_euclid(7)
}

pub fn board_group_of(quest_id: u32) -> Option<&'static BoardGroup> {
    BOARD_GROUPS.iter().find(|g| g.contains(quest_id))
}

/// One postable unit and the grade gate it sits behind: a daily resident (both halves) or one
/// weekly donation.
fn selections(group: &BoardGroup, quests: &std::collections::HashMap<u32, crate::config::Quest>) -> Vec<(u8, Vec<u32>)> {
    let gate = |q: u32| quests.get(&q).and_then(|q| q.citizenship_check).map_or(1, |(_, g)| g);
    if group.odd_do_not_repeat {
        (group.first..=group.last).step_by(2).map(|odd| (gate(odd), vec![odd, odd + 1])).collect()
    } else {
        (group.first..=group.last).map(|q| (gate(q), vec![q])).collect()
    }
}

/// splitmix64 - a fixed, portable mixer, so every channel process draws the same shuffle.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The `period`th pick from `n` options: a shuffled order per cycle of `n` periods, so each
/// option comes round exactly once per cycle.
fn pick(n: usize, period: i64, salt: u64) -> usize {
    if n <= 1 {
        return 0;
    }
    let cycle = period.div_euclid(n as i64);
    let index = period.rem_euclid(n as i64) as usize;
    let mut order: Vec<usize> = (0..n).collect();
    let mut state = mix(salt ^ (cycle as u64).wrapping_mul(0x2545_F491_4F6C_DD1D));
    for i in (1..n).rev() {
        state = mix(state);
        order.swap(i, (state % (i as u64 + 1)) as usize);
    }
    order[index]
}

/// Every tier's pick for `group` in `period`: `(gate grade, quest ids)`, ascending grade.
pub fn tier_picks(
    group: &BoardGroup,
    quests: &std::collections::HashMap<u32, crate::config::Quest>,
    period: i64,
) -> Vec<(u8, Vec<u32>)> {
    let all = selections(group, quests);
    let mut tiers: Vec<u8> = all.iter().map(|(g, _)| *g).collect();
    tiers.sort_unstable();
    tiers.dedup();
    tiers
        .into_iter()
        .map(|tier| {
            let options: Vec<&Vec<u32>> = all.iter().filter(|(g, _)| *g == tier).map(|(_, q)| q).collect();
            let salt = u64::from(group.record_quest) << 8 | u64::from(tier);
            (tier, options[pick(options.len(), period, salt)].clone())
        })
        .collect()
}

/// The pick a character reaching `grade` and `level` gets in `period`: the highest tier whose
/// grade gate they meet and whose quests they can turn in (`Check.1.lvmin`). Empty when no
/// tier qualifies.
pub fn fresh_pick(
    group: &BoardGroup,
    quests: &std::collections::HashMap<u32, crate::config::Quest>,
    period: i64,
    grade: u8,
    level: u32,
) -> Vec<u32> {
    tier_picks(group, quests, period)
        .into_iter()
        .rfind(|(g, ids)| *g <= grade && ids.iter().all(|q| finish_level(quests, *q) <= level))
        .map(|(_, ids)| ids)
        .unwrap_or_default()
}

fn finish_level(quests: &std::collections::HashMap<u32, crate::config::Quest>, quest_id: u32) -> u32 {
    quests.get(&quest_id).map_or(0, |q| q.complete_min_level)
}

/// One of the character's quest rows in a board group, as [`settle`] reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardRow {
    pub quest_id: u32,
    pub in_progress: bool,
    pub completed_at: Option<i64>,
}

/// What [`settle`] decided for one group.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Settled {
    /// The period's pick - store it when `save` is set.
    pub pick: Vec<u32>,
    pub save: bool,
    /// In-progress quests to give up: they are not this period's one.
    pub drop: Vec<u32>,
    /// What the board record lists now: the pick, or nothing once one was turned in.
    pub posted: Vec<u32>,
}

/// The whole posting rule for one group and one ACTIVE citizen - see the module docs.
/// `stored` is the kept `(period, pick)`, `rows` the character's quest rows in this group.
pub fn settle(
    group: &BoardGroup,
    quests: &std::collections::HashMap<u32, crate::config::Quest>,
    now: i64,
    grade: u8,
    level: u32,
    stored: Option<(i64, Vec<u32>)>,
    rows: &[BoardRow],
) -> Settled {
    let period = group.period(now);
    let mut out = Settled::default();
    match stored {
        Some((p, pick)) if p == period => out.pick = pick,
        _ => {
            out.save = true;
            let gate = |q: u32| quests.get(&q).and_then(|q| q.citizenship_check).map_or(1, |(_, g)| g);
            let keep = rows
                .iter()
                .filter(|r| r.in_progress && group.contains(r.quest_id) && finish_level(quests, r.quest_id) <= level)
                .max_by_key(|r| (gate(r.quest_id), r.quest_id))
                .map(|r| r.quest_id);
            out.drop = rows
                .iter()
                .filter(|r| r.in_progress && group.contains(r.quest_id) && Some(r.quest_id) != keep)
                .map(|r| r.quest_id)
                .collect();
            out.pick = match keep {
                // A daily resident is both halves; the client shows whichever applies.
                Some(q) if group.odd_do_not_repeat => {
                    let odd = if q % 2 == 1 { q } else { q - 1 };
                    vec![odd, odd + 1]
                }
                Some(q) => vec![q],
                None => fresh_pick(group, quests, period, grade, level),
            };
        }
    }
    let turned_in = rows
        .iter()
        .any(|r| group.contains(r.quest_id) && r.completed_at.is_some_and(|at| group.period(at) == period));
    out.posted = if turned_in { Vec::new() } else { out.pick.clone() };
    out
}

/// A group's record value, `qrKey=id|id`.
pub fn board_record(group: &BoardGroup, posted: &[u32]) -> String {
    let list = posted.iter().map(u32::to_string).collect::<Vec<_>>().join("|");
    format!("{}={list}", group.key)
}

/// Whether a COMPLETED board quest may be picked up again at `now` - the client's own table,
/// module docs. Posting is checked separately.
pub fn may_repeat(group: &BoardGroup, quest_id: u32, completed_at: i64, now: i64) -> bool {
    !group.do_not_repeat(quest_id) && group.period(completed_at) < group.period(now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CitizenshipContr, Quest};
    use std::collections::HashMap;

    fn standing(town: u8, state: u8, grade: u8, contribution: u32) -> TownStanding {
        TownStanding { town, state, grade, contribution, certified_grade: grade }
    }

    /// The grade gates exactly as this client's `Quest.wz` has them - see
    /// `gm-handbook/quests.json`: residents 1, the four leaders 5; donations
    /// 1 x6, 2, 3 x3, 4, 5 x2 (Kerning x3), 6, 7, 8, 9.
    fn board_quests() -> HashMap<u32, Quest> {
        let mut m = HashMap::new();
        // `Check.1.lvmin`: 12 for a resident, 32 for a leader, `12 + 5 (g - 1)` for a donation.
        let mut put = |id: u32, town: u8, grade: u8| {
            let complete_min_level = if (506_019..=506_035).contains(&id) || (506_119..=506_136).contains(&id) {
                12 + 5 * (u32::from(grade) - 1)
            } else if grade == 5 {
                32
            } else {
                12
            };
            m.insert(id, Quest { citizenship_check: Some((town, grade)), complete_min_level, ..Quest::default() });
        };
        for (first, town) in [(506_001u32, 1u8), (506_101, 2)] {
            for q in first..first + 14 { put(q, town, 1); }
            for q in first + 14..first + 18 { put(q, town, 5); }
        }
        let hen = [1, 1, 1, 1, 1, 1, 2, 3, 3, 3, 4, 5, 5, 6, 7, 8, 9];
        for (i, g) in hen.iter().enumerate() { put(506_019 + i as u32, 1, *g); }
        let ker = [1, 1, 1, 1, 1, 1, 2, 3, 3, 3, 4, 5, 5, 5, 6, 7, 8, 9];
        for (i, g) in ker.iter().enumerate() { put(506_119 + i as u32, 2, *g); }
        m
    }

    /// The fixture above IS this client's data, and the loader reads every citizenship key:
    /// 86 gated quests, 86 contributions, and every board quest in its group's town. Skipped
    /// when `gm-handbook/` has not been generated (it is gitignored).
    #[test]
    fn the_board_fixture_and_the_loader_agree_with_this_clients_quest_data() {
        let path = std::path::Path::new("../../gm-handbook/questlines.txt");
        if !path.exists() {
            return;
        }
        let real = crate::config::load_quests(path);
        assert_eq!(real.values().filter(|q| q.citizenship_check.is_some()).count(), 86);
        assert_eq!(real.values().filter(|q| q.citizenship_contr.is_some()).count(), 86);
        assert!(real.values().filter_map(|q| q.citizenship_contr.as_ref()).all(|c| (c.amount.is_some() || c.formula.is_some()) && (c.town == 1 || c.town == 2)));
        let fixture = board_quests();
        for (id, q) in &fixture {
            assert_eq!(real[id].citizenship_check, q.citizenship_check, "quest {id}");
            assert_eq!(real[id].complete_min_level, q.complete_min_level, "quest {id}: Check.1.lvmin");
        }
        assert_eq!(real[&506_035].citizenship_contr.as_ref().unwrap().amount, Some(2_500));
        // `Act.1.money`: 255 quests, all positive, all read (paid at the Quest rate since
        // 2026-09-28). 506001's is 351.
        assert_eq!(real.values().filter(|q| q.complete_money > 0).count(), 255);
        assert_eq!(real[&506_001].complete_money, 351);
        assert!(real[&506_001].citizenship_contr.as_ref().unwrap().formula.is_some());
    }

    #[test]
    fn the_record_is_the_clients_key_value_string_in_town_order() {
        assert_eq!(record_value(&[]), "");
        assert_eq!(
            record_value(&[standing(2, STATE_ACTIVE, 1, 0), standing(1, STATE_FROZEN, 5, 4150)]),
            "st1=2;gr1=5;ct1=4150;st2=1;gr2=1;ct2=0"
        );
    }

    #[test]
    fn grades_follow_the_thresholds_and_stop_at_ten() {
        // Per grade, the excess carried: the owner's own example - 3,000 in all from grade 1 to 3.
        let up = |grade: u8, contribution: u32| {
            let t = regrade(standing(1, 1, grade, contribution));
            (t.grade, t.contribution)
        };
        assert_eq!(up(1, 999), (1, 999));
        assert_eq!(up(1, 1_000), (2, 0), "1,000 at grade 1 is Visitor, and the counter resets");
        assert_eq!(up(1, 1_250), (2, 250), "the excess is kept");
        assert_eq!(up(2, 1_999), (2, 1_999), "grade 2 needs 2,000 of its own");
        assert_eq!(up(1, 3_000), (3, 0), "grade 1 to 3 is 3,000 in total");
        assert_eq!(up(9, 9_999), (9, 9_999));
        assert_eq!(up(9, 10_400), (10, 400), "Citizen of Honor needs 10,000 at grade 9");
        assert_eq!(up(1, 46_000), (10, 0), "1 to 10 is 46,000");
        assert_eq!(up(10, 70_000), (10, 70_000), "at ten it keeps counting");
        assert_eq!(next_threshold(1), Some(1_000));
        assert_eq!(next_threshold(9), Some(10_000));
        assert_eq!(next_threshold(10), None);
        assert_eq!(regrade(standing(1, 1, 4, 100)).grade, 4, "never down");
        assert_eq!(grade_name(10), "Citizen of Honor");
    }

    /// The daily formula at every grade: 100 at 1, +50 each. And the flat weeklies.
    #[test]
    fn the_formula_is_evaluated_the_way_the_client_draws_it() {
        let f = CitizenshipContr { town: 1, amount: None, formula: Some("100 + ( ( citizenshipGrade - 1 ) x 50 )".into()) };
        let got: Vec<u32> = (1..=10).map(|g| contribution_for(&f, g)).collect();
        assert_eq!(got, vec![100, 150, 200, 250, 300, 350, 400, 450, 500, 550]);
        assert_eq!(contribution_for(&CitizenshipContr { town: 1, amount: Some(2_500), formula: None }, 1), 2_500);
        assert_eq!(eval_formula("2 * (3 + 4)", 0), Some(14));
        assert_eq!(eval_formula("100 +", 1), None, "a malformed formula pays nothing rather than guessing");
        assert_eq!(eval_formula("( 1", 1), None);
    }

    #[test]
    fn the_clerk_offers_the_one_contract_that_applies() {
        let active_h = [standing(1, STATE_ACTIVE, 3, 2_500)];
        assert_eq!(clerk_offer(11, &[], 1), ClerkOffer::TooLow);
        assert_eq!(clerk_offer(12, &[], 1), ClerkOffer::Oath);
        assert_eq!(clerk_offer(30, &active_h, 2), ClerkOffer::Transfer { old_town: 1 });
        assert_eq!(clerk_offer(30, &active_h, 1), ClerkOffer::Renunciation { grade: 3 });
        let owed = [TownStanding { certified_grade: 2, ..active_h[0] }];
        assert_eq!(clerk_offer(30, &owed, 1), ClerkOffer::Certificate { grade: 3 });
        let frozen = [standing(1, STATE_FROZEN, 3, 2_500), standing(2, STATE_ACTIVE, 1, 0)];
        assert_eq!(clerk_offer(30, &frozen, 1), ClerkOffer::Reactivation { grade: 3 });
    }

    /// Every accepted contract leaves at most ONE active town, and keeps a frozen town's
    /// grade and contribution for its return.
    #[test]
    fn every_contract_leaves_at_most_one_active_town() {
        let oath = after_contract(&[], 1, ClerkOffer::Oath).unwrap();
        assert_eq!(oath, vec![standing(1, STATE_ACTIVE, 1, 0)]);
        let moved = after_contract(&[standing(1, STATE_ACTIVE, 3, 2_500)], 2, ClerkOffer::Transfer { old_town: 1 }).unwrap();
        assert_eq!(moved, vec![standing(1, STATE_FROZEN, 3, 2_500), standing(2, STATE_ACTIVE, 1, 0)]);
        let back = after_contract(&moved, 1, ClerkOffer::Reactivation { grade: 3 }).unwrap();
        assert_eq!(back, vec![standing(1, STATE_ACTIVE, 3, 2_500), standing(2, STATE_FROZEN, 1, 0)]);
        let gone = after_contract(&back, 1, ClerkOffer::Renunciation { grade: 3 }).unwrap();
        assert!(gone.iter().all(|t| !t.is_active()));
        for s in [&oath, &moved, &back, &gone] {
            assert!(s.iter().filter(|t| t.is_active()).count() <= 1);
        }
        // Stale offers are refused at the answer, not applied.
        assert_eq!(after_contract(&moved, 2, ClerkOffer::Oath), None, "already signed here");
        assert_eq!(after_contract(&[], 1, ClerkOffer::Renunciation { grade: 1 }), None);
        assert_eq!(after_contract(&[], 1, ClerkOffer::TooLow), None);
    }

    #[test]
    fn the_four_groups_are_the_clients_and_cover_every_board_quest() {
        let q = board_quests();
        for g in &BOARD_GROUPS {
            for id in g.first..=g.last {
                assert!(q.contains_key(&id), "{id}");
                assert_eq!(q[&id].citizenship_check.unwrap().0, g.town);
                assert_eq!(board_group_of(id), Some(g));
            }
        }
        assert!(BOARD_GROUPS[0].do_not_repeat(506_001) && !BOARD_GROUPS[0].do_not_repeat(506_002));
        assert!(!BOARD_GROUPS[1].do_not_repeat(506_019), "weeklies all repeat");
        assert_eq!(board_group_of(506_036), None, "the story arcs are not on the board");
        assert_eq!(board_group_of(506_000), None);
    }

    /// One resident a day, a leader instead at grade 5 and level 32; one donation a week at the
    /// highest tier grade AND level reach.
    #[test]
    fn a_fresh_pick_is_the_highest_tier_grade_and_level_both_reach() {
        let q = board_quests();
        for period in 0..40 {
            let d = fresh_pick(&BOARD_GROUPS[0], &q, period, 1, 200);
            assert_eq!(d.len(), 2);
            assert_eq!(d[0] % 2, 1, "the greeting half");
            assert_eq!(d[1], d[0] + 1, "and its check-in");
            assert!(d[0] < 506_015, "no leader below grade 5");
            let leader = fresh_pick(&BOARD_GROUPS[0], &q, period, 5, 32);
            assert_eq!(leader.len(), 2, "ONE daily: the leader instead, not as well");
            assert!(leader[0] >= 506_015);
            assert_eq!(fresh_pick(&BOARD_GROUPS[0], &q, period, 5, 31), d, "grade 5 below level 32: a resident");
            for grade in 1..=10u8 {
                let w = fresh_pick(&BOARD_GROUPS[1], &q, period, grade, 200);
                assert_eq!(w.len(), 1);
                assert_eq!(q[&w[0]].citizenship_check.unwrap().1, grade.min(9), "grade {grade}");
            }
            // The screenshot: grade 3, below level 22 - the level-17 tier, not the grade-3 one.
            let w = fresh_pick(&BOARD_GROUPS[1], &q, period, 3, 21);
            assert_eq!(q[&w[0]].citizenship_check.unwrap().1, 2);
            assert_eq!(q[&w[0]].complete_min_level, 17);
            assert!(fresh_pick(&BOARD_GROUPS[1], &q, period, 3, 11).is_empty(), "nothing anyone below 12 can finish");
        }
    }

    /// Every option comes round once per cycle, and the picks move.
    #[test]
    fn each_resident_comes_round_once_per_cycle() {
        let q = board_quests();
        let n = 7;
        for cycle in 0..5 {
            let mut seen: Vec<u32> = (cycle * n..cycle * n + n).map(|p| fresh_pick(&BOARD_GROUPS[2], &q, p, 1, 200)[0]).collect();
            seen.sort_unstable();
            assert_eq!(seen, (0..7).map(|i| 506_101 + 2 * i).collect::<Vec<_>>(), "cycle {cycle}");
        }
        let days: Vec<u32> = (0..14).map(|p| fresh_pick(&BOARD_GROUPS[0], &q, p, 1, 200)[0]).collect();
        assert!(days.windows(2).filter(|w| w[0] != w[1]).count() >= 10, "{days:?}");
    }

    #[test]
    fn the_records_are_the_clients_key_then_pipe_separated_ids() {
        assert_eq!(board_record(&BOARD_GROUPS[0], &[506_005, 506_006]), "q1_d=506005|506006");
        assert_eq!(board_record(&BOARD_GROUPS[1], &[506_025]), "q1_w=506025");
        assert_eq!(board_record(&BOARD_GROUPS[3], &[]), "q1_w=", "nothing posted is an empty list, not a missing key");
    }

    fn row(quest_id: u32, in_progress: bool, completed_at: Option<i64>) -> BoardRow {
        BoardRow { quest_id, in_progress, completed_at }
    }

    /// **Frozen for the period.** The pick is made once; a grade or level gained later does not
    /// move it, and the next period does.
    #[test]
    fn the_pick_is_made_once_per_period_and_kept() {
        let q = board_quests();
        let weekly = &BOARD_GROUPS[1];
        let monday = (20_724 * 86_400) as i64; // 2026-09-28
        let first = settle(weekly, &q, monday, 1, 30, None, &[]);
        assert!(first.save && first.drop.is_empty());
        assert_eq!(q[&first.pick[0]].citizenship_check.unwrap().1, 1);
        assert_eq!(first.posted, first.pick);
        let stored = Some((weekly.period(monday), first.pick.clone()));
        let later = settle(weekly, &q, monday + 3 * 86_400, 4, 40, stored.clone(), &[]);
        assert!(!later.save, "same week: kept");
        assert_eq!(later.posted, first.pick, "a grade-up mid-week posts nothing new");
        let next = settle(weekly, &q, monday + 7 * 86_400, 4, 40, stored, &[]);
        assert!(next.save);
        assert_eq!(q[&next.pick[0]].citizenship_check.unwrap().1, 4, "the next Monday: the new tier");
    }

    /// **Turned in, then nothing** until the period turns - the daily's check-in half included.
    #[test]
    fn a_turn_in_empties_the_group_for_the_rest_of_the_period() {
        let q = board_quests();
        let daily = &BOARD_GROUPS[0];
        let now = (20_730 * 86_400) as i64;
        let pick = fresh_pick(daily, &q, daily.period(now), 1, 30);
        let stored = Some((daily.period(now), pick.clone()));
        let greeted = settle(daily, &q, now, 5, 40, stored.clone(), &[row(pick[0], false, Some(now + 600))]);
        assert!(greeted.posted.is_empty(), "the First Greeting was today's daily: no check-in, no leader");
        let yesterday = settle(daily, &q, now, 5, 40, stored, &[row(pick[0], false, Some(now - 86_400))]);
        assert_eq!(yesterday.posted, pick, "a turn-in yesterday does not count against today");
    }

    /// **The screenshot**: three donations in progress, the top one needing level 22. The
    /// highest one the character can finish is kept as the week's; the others are given up.
    #[test]
    fn extra_in_progress_quests_are_given_up_and_the_highest_finishable_one_kept() {
        let q = board_quests();
        let weekly = &BOARD_GROUPS[1];
        let now = (20_730 * 86_400) as i64;
        // 506_019 tier 1 (lv 12), 506_025 tier 2 (lv 17), 506_026 tier 3 (lv 22).
        let rows = [row(506_019, true, None), row(506_025, true, None), row(506_026, true, None)];
        let s = settle(weekly, &q, now, 3, 21, None, &rows);
        assert_eq!(s.pick, vec![506_025]);
        assert_eq!(s.posted, vec![506_025]);
        let mut dropped = s.drop.clone();
        dropped.sort_unstable();
        assert_eq!(dropped, vec![506_019, 506_026]);
        assert!(s.save);

        // One in progress from last week carries over as this week's - no second donation.
        let carried = settle(weekly, &q, now, 9, 99, Some((weekly.period(now) - 1, vec![506_019])), &[row(506_019, true, None)]);
        assert_eq!(carried.pick, vec![506_019]);
        assert!(carried.drop.is_empty());

        // A daily check-in in progress keeps its resident, both halves.
        let daily = settle(&BOARD_GROUPS[0], &q, now, 1, 30, None, &[row(506_006, true, None)]);
        assert_eq!(daily.pick, vec![506_005, 506_006]);
    }

    /// Weeks turn over on Monday, UTC: 2026-09-27 is a Sunday, 2026-09-28 a Monday.
    #[test]
    fn weeks_start_on_monday_and_repeats_follow_the_clients_table() {
        let sunday = 20_723; // 2026-09-27
        assert_eq!(store::dailyperks::utc_date(sunday), "2026-09-27");
        assert_eq!(week(sunday) + 1, week(sunday + 1), "Sunday -> Monday turns the week");
        assert_eq!(week(sunday + 1), week(sunday + 7), "Monday .. Sunday is one week");
        let day = |d: i64| d * 86_400 + 3_600;
        let daily = &BOARD_GROUPS[0];
        let weekly = &BOARD_GROUPS[1];
        assert!(!may_repeat(daily, 506_002, day(sunday), day(sunday) + 3_000), "same day");
        assert!(may_repeat(daily, 506_002, day(sunday), day(sunday + 1)));
        assert!(!may_repeat(daily, 506_001, day(sunday), day(sunday + 30)), "a First Greeting never repeats");
        assert!(!may_repeat(weekly, 506_019, day(sunday + 1), day(sunday + 7)), "same week");
        assert!(may_repeat(weekly, 506_019, day(sunday), day(sunday + 1)), "Sunday's donation, Monday");
    }
}
