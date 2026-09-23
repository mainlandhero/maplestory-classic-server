//! **"First Time Together" - the entry gate.** The maps, the quest row and everything the
//! client holds are written up in `research/first-time-together-pq.md`.
//!
//! The owner, 2026-09-22, with a screenshot of Lakelis mid-sentence: *"The entry fails if the
//! person talking is not a party leader, if the person is not in a party with all members
//! that are level 21 or above, is not in a party of at least two players. The maximum for
//! the party quest will be 4 players. Once the entry requirement is fulfilled, all party
//! members will be teleported into Stage 1."* And, in the message after it: *"all party
//! members must be online and in the same map (Kerning City)"*.
//!
//! **This is deliberately a change from the client's own text**, which says *"a party of
//! four adventurers, all level 21 or higher"* (`Quest.wz` 10311, and Lakelis says it again
//! in their Say node). Four is the **maximum** here and two is the minimum, so a pair can run
//! it. The quest text is left alone: it is the client's string table, it is not a rule this
//! server reads, and rewriting the client to match a server rule is the wrong direction.
//!
//! # What this module is and is not
//!
//! It is the gate: who may start, what Lakelis says when they may not, the per-channel
//! registry of runs ([`Runs`]), and each run's cleared stages. The instance id a run gets is
//! the `instance` half of `crate::fields::FieldKey`, so mobs, drops and broadcasts are the
//! run's own. It is **not** the stage rules: a stage is cleared today by clicking Cloto
//! ([`CLOTO`], TEMPORARY), not by coupons, ropes or platforms -
//! `research/first-time-together-pq.md` §4 has those.

use crate::party::{CharacterId, Party, PartyId};

/// Lakelis, in Kerning City. The only one of the three sisters placed in the world.
pub const LAKELIS: u32 = 800_000;
/// Kerning City - where Lakelis stands. The entry is refused anywhere else, because the
/// template alone is not a place.
pub const ENTRY_MAP: u32 = 10_003_000;
/// `<1st Stage>`; the party lands here.
pub const STAGE_1: u32 = 80_000_000;
/// `<Exit>`, and every stage's `forcedReturn`.
pub const EXIT_MAP: u32 = 80_000_600;

/// Every member must be at least this level. The client's quest text says 21 too.
pub const MIN_LEVEL: u16 = 21;
/// **Two, not four.** The owner's rule; the client's text is not a server rule.
pub const MIN_PARTY: usize = 2;
/// Four, as both the owner and the client say.
pub const MAX_PARTY: usize = 4;

/// **TEMPORARY - the minimum Lakelis actually enforces while instancing is being tested.**
///
/// The owner, 2026-09-23: *"I would like to do a temporary test to make sure the instancing works.
/// Please temporarily allow party of 1s to enter via Lakelis in Kerning City."* The rule is
/// still [`MIN_PARTY`], and [`check`] still enforces it and its tests still pin it; only the
/// session's call goes through [`check_min`] with this instead. **Put it back to
/// `MIN_PARTY` when the test is over** - `STATUS.md` lists it as a temporary switch.
pub const ENTRY_MIN_PARTY: usize = 1;

/// **How long a party has.** The owner, 2026-09-22: *"the party quest lacked a timer, since it
/// has to be finished within the time limit or its members will be kicked out into the exit
/// map."* Then a screenshot of the widget itself, reading **29:32** a few seconds into a
/// run: the limit is **thirty minutes**, and the client draws it as Min/Sec under the words
/// "Time Left". That screenshot is also what confirms the widget is the type-2 countdown
/// (`net::clock::clock_seconds`) rather than the map's own clock.
pub const TIME_LIMIT_S: u32 = 1_800;

/// The Ligator, stage 1's only monster - 22 of them.
pub const LIGATOR: u32 = 800_000;
/// The coupon a Ligator drops, one per kill. `data/drops.txt` carries the rate and
/// `droptables.rs` has the test that keeps it at 100%.
pub const COUPON: u32 = 4_001_001;

/// Cloto - on stages 1 to 5 (`80000000`..`80000400`), and nowhere else [L]
/// (`gm-handbook/npcs.txt`). The stage NPC.
///
/// **TEMPORARY behaviour.** The owner, 2026-09-23: *"Lakelis in every stage when clicked on should
/// send the "stage clear" opcode and enable the portal to go to the next stage."* The NPC on
/// every stage is Cloto, not Lakelis (who stands only in Kerning City), so this is them. For
/// the test they clear the stage on a click; the real rules (coupons, ropes, platforms) are
/// `research/first-time-together-pq.md` §4 and are not built.
pub const CLOTO: u32 = 800_001;

/// What Cloto says as they clear a stage, during the temporary test.
pub const CLOTO_CLEARED: &str =
    "Stage cleared! The portal is open - go through it to reach the next stage.";
/// What they say when this party has already cleared the stage they stand on.
pub const CLOTO_ALREADY: &str = "This stage is already cleared. The portal is open.";
/// What the `next00` portal says while this party's stage is still closed.
pub const PORTAL_CLOSED: &str = "The portal is not open yet. Clear the stage first.";

/// Arrival portal on every stage - each `next00`'s `tn` [L].
pub const ARRIVAL_PORTAL: &str = "st00";
/// The forward portal on stages 1 to 5.
pub const NEXT_PORTAL: &str = "next00";

/// Nella - in every one of the seven fields, including the Exit. The way out.
pub const NELLA: u32 = 800_002;
/// The conversation path Nella's yes/no is parked under.
pub const NELLA_PATH: &str = "firsttime.nella";
/// Kerning City, where Nella sends someone standing on the Exit map.
pub const TOWN_MAP: u32 = ENTRY_MAP;

/// What Nella asks, inside the quest.
pub const NELLA_LEAVE: &str =
    "If you want to leave this place, come talk to me. Shall I send you out?";
/// What Nella asks on the Exit map.
pub const NELLA_TOWN: &str = "Shall I send you back to Kerning City?";

/// The conversation path Lakelis' yes/no is parked under.
pub const ASK_PATH: &str = "firsttime.ask";

/// What Lakelis says when clicked - the line in the owner's screenshot, verbatim.
pub const GREETING: &str =
    "How about you and your party members attempt a quest together? Here you'll find \
     obstacles and problems that must be overcome with great teamwork.";

/// Why an entry was refused. One variant per rule the owner gave, so the line a player reads
/// names the thing they have to change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// Not in a party at all.
    NoParty,
    /// In a party, but somebody else leads it.
    NotLeader,
    /// A party of one.
    TooSmall { size: usize },
    /// More than four.
    TooLarge { size: usize },
    /// At least one member is under [`MIN_LEVEL`]; the lowest is named.
    Underlevelled { name: String, level: u16 },
    /// A member is playing on this channel but standing somewhere other than
    /// [`ENTRY_MAP`].
    Elsewhere { name: String },
    /// A member is not visible on this channel at all.
    ///
    /// **This deliberately does not claim they are offline.** A party is hub-replicated and
    /// spans channels, so a member missing from this channel's bus is either logged out or
    /// playing on another channel, and this process cannot tell which. Saying "not here
    /// with you" is true of both; saying "offline" would be asserting the half nobody looked
    /// up - the same rule as [`Refusal::UnknownMember`].
    NotHere { name: String },
    /// A member's record could not be read, so their level is unknown. Refusing is the
    /// only safe answer: letting them in would be asserting a level nobody looked up.
    UnknownMember { character: CharacterId },
}

impl Refusal {
    /// The line Lakelis says. Each one states the rule rather than only the failure, so a
    /// player can fix it without guessing.
    pub fn line(&self) -> String {
        match self {
            Refusal::NoParty => format!(
                "This is a trial for a party, not for one adventurer. Form a party of \
                 {MIN_PARTY} to {MAX_PARTY}, all level {MIN_LEVEL} or higher, and have your \
                 leader speak to me."
            ),
            Refusal::NotLeader => {
                "Only the leader of a party may take us up on this. Ask whoever leads yours \
                 to speak to me."
                    .to_string()
            }
            Refusal::TooSmall { size } => format!(
                "There are only {size} of you. Bring at least {MIN_PARTY} - this is a trial \
                 that cannot be passed alone."
            ),
            Refusal::TooLarge { size } => format!(
                "There are {size} of you, and I can only send {MAX_PARTY} inside. Leave some \
                 behind and speak to me again."
            ),
            Refusal::Underlevelled { name, level } => format!(
                "#b{name}#k is only level {level}. Every one of you must be level \
                 {MIN_LEVEL} or higher before I can send you in."
            ),
            Refusal::Elsewhere { name } => format!(
                "#b{name}#k is not here in Kerning City. Everyone must be standing with you \
                 before I can send you in."
            ),
            Refusal::NotHere { name } => format!(
                "#b{name}#k is not online here with you. Everyone in the party must be here \
                 in Kerning City before I can send you in."
            ),
            Refusal::UnknownMember { character } => format!(
                "I cannot see everyone in your party right now (character {character}). Try \
                 again in a moment."
            ),
        }
    }
}

/// One member, as the gate needs them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub character: CharacterId,
    pub name: String,
    pub level: u16,
    /// Playing on this channel right now - the broadcast bus has a presence for them.
    pub online: bool,
    /// That presence is on [`ENTRY_MAP`]. Implies `online`.
    pub here: bool,
}

impl Candidate {
    /// A candidate standing in Kerning City - the shape that passes the presence rules.
    pub fn present(character: CharacterId, name: &str, level: u16) -> Self {
        Self { character, name: name.to_string(), level, online: true, here: true }
    }
}

/// **The gate.** `who` clicked Lakelis; `party` is their party, if any; `levels` answers
/// for each member id, `None` when the record could not be read.
///
/// Pure, so every rule the owner gave is a unit test rather than a client run. The order of the
/// checks is the order a player meets them: be in a party, lead it, be the right size, all
/// be **here**, and only then be the right level - a party with someone still in Henesys is
/// told that before it is told about a level, because that is the one they have to fix
/// first and the absent member's level is not something they can act on yet.
pub fn check(
    who: CharacterId,
    party: Option<&Party>,
    levels: impl FnMut(CharacterId) -> Option<Candidate>,
) -> Result<Vec<Candidate>, Refusal> {
    check_min(MIN_PARTY, who, party, levels)
}

/// [`check`] with the minimum party size given. Exists for [`ENTRY_MIN_PARTY`], the
/// temporary solo test, so the rule itself does not have to move to allow it.
pub fn check_min(
    min_party: usize,
    who: CharacterId,
    party: Option<&Party>,
    mut levels: impl FnMut(CharacterId) -> Option<Candidate>,
) -> Result<Vec<Candidate>, Refusal> {
    let Some(party) = party else { return Err(Refusal::NoParty) };
    if party.leader != who {
        return Err(Refusal::NotLeader);
    }
    let size = party.members.len();
    if size < min_party {
        return Err(Refusal::TooSmall { size });
    }
    if size > MAX_PARTY {
        return Err(Refusal::TooLarge { size });
    }
    let mut out = Vec::with_capacity(size);
    for &member in &party.members {
        let Some(c) = levels(member) else {
            return Err(Refusal::UnknownMember { character: member });
        };
        out.push(c);
    }
    // **Everyone online and in Kerning City.** The owner, 2026-09-22: "all party members must be
    // online and in the same map (Kerning City)". Checked before the level, and in join
    // order so the party reads about one absent member at a time rather than a list.
    if let Some(away) = out.iter().find(|c| !c.online) {
        return Err(Refusal::NotHere { name: away.name.clone() });
    }
    if let Some(away) = out.iter().find(|c| !c.here) {
        return Err(Refusal::Elsewhere { name: away.name.clone() });
    }
    // The LOWEST under-levelled member is named, not the first in join order, so the same
    // party always reads the same line whoever happens to be listed first.
    if let Some(low) = out.iter().filter(|c| c.level < MIN_LEVEL).min_by_key(|c| c.level) {
        return Err(Refusal::Underlevelled { name: low.name.clone(), level: low.level });
    }
    Ok(out)
}

/// A running instance of the quest - one per party that has entered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    /// Distinct for the life of the process. **Not** the party id: a party may run the
    /// quest more than once, and each run is its own field.
    pub id: u32,
    pub party: PartyId,
    pub members: Vec<CharacterId>,
    /// Unix seconds. **A wall clock, deliberately**: `Session::tick`'s `now_ms` counts from
    /// the moment that one connection opened, so two members would hold two different
    /// deadlines for the same run. `store::Store::unix_now`.
    pub deadline_unix: i64,
    /// **The stages THIS run has cleared**, by map id. Per instance and never shared - the owner,
    /// 2026-09-23: *"just because one party instance cleared, doesn't mean that all party
    /// instances cleared."* It lives on the instance, so there is nowhere else for it to be.
    pub cleared: Vec<u32>,
}

impl Instance {
    /// Seconds left, saturating at zero.
    pub fn remaining_s(&self, now_unix: i64) -> u32 {
        u32::try_from((self.deadline_unix - now_unix).max(0)).unwrap_or(0)
    }

    /// Whether the clock has run out.
    pub fn is_over(&self, now_unix: i64) -> bool {
        now_unix >= self.deadline_unix
    }
}

/// **This channel's running quests.** Owned by `crate::fields::Fields` (`Fields::runs`),
/// beside the party registry, so it is exactly as wide as a channel.
///
/// It was a process-wide `static` until 2026-09-23, and that was found by the test harness:
/// tests run in parallel in one process, every session test's characters start at id 200 in
/// its own in-memory store, and every session tick expires runs against the real clock - so
/// four tests each holding "character 200" in a run saw and swept each other's runs. The
/// three that existed had been passing on timing. A channel's runs belong to the channel,
/// and giving each `Fields` its own is what makes a test's world its own.
#[derive(Debug)]
pub struct Runs {
    next_id: u32,
    live: Vec<Instance>,
}

impl Default for Runs {
    /// Instance ids start at 1: 0 is `FieldKey`'s shared world and must never name a run.
    fn default() -> Self {
        Self { next_id: 1, live: Vec::new() }
    }
}

impl Runs {
    /// Open a new instance for `party`. Any instance that party already had is dropped
    /// first, so a re-entry cannot leave the old one behind holding the old member list.
    pub fn open(&mut self, party: PartyId, members: Vec<CharacterId>, now_unix: i64) -> Instance {
        self.live.retain(|i| i.party != party);
        let inst = Instance {
            id: self.next_id,
            party,
            members,
            deadline_unix: now_unix + i64::from(TIME_LIMIT_S),
            cleared: Vec::new(),
        };
        self.next_id += 1;
        self.live.push(inst.clone());
        inst
    }

    /// **Take every instance whose clock has run out**, removing them as they are handed
    /// over so two sessions ticking at once cannot both expire the same run and warp its
    /// members twice.
    pub fn take_expired(&mut self, now_unix: i64) -> Vec<Instance> {
        let (over, still) = self.live.drain(..).partition::<Vec<_>, _>(|i| i.is_over(now_unix));
        self.live = still;
        over
    }

    /// Drop one character out of their run - they left the party, or walked out through
    /// Nella. Returns the instance they were in. An instance with nobody left is forgotten.
    pub fn drop_member(&mut self, character: CharacterId) -> Option<Instance> {
        let at = self.live.iter().position(|i| i.members.contains(&character))?;
        let was = self.live[at].clone();
        self.live[at].members.retain(|&m| m != character);
        if self.live[at].members.is_empty() {
            self.live.remove(at);
        }
        Some(was)
    }

    /// The instance `character` is running, if any.
    pub fn instance_of(&self, character: CharacterId) -> Option<Instance> {
        self.live.iter().find(|i| i.members.contains(&character)).cloned()
    }

    /// Forget an instance. Returns whether one went.
    pub fn close(&mut self, id: u32) -> bool {
        let before = self.live.len();
        self.live.retain(|i| i.id != id);
        self.live.len() != before
    }

    /// **Clear `stage` for the run `character` is in.** `Some(true)` when this call cleared
    /// it, `Some(false)` when that run had already cleared it, `None` when `character` is in
    /// no run or `stage` is not a stage with a way forward. Only the one run changes.
    pub fn clear_stage(&mut self, character: CharacterId, stage: u32) -> Option<bool> {
        next_stage(stage)?;
        let run = self.live.iter_mut().find(|i| i.members.contains(&character))?;
        if run.cleared.contains(&stage) {
            return Some(false);
        }
        run.cleared.push(stage);
        Some(true)
    }

    /// Whether the run `character` is in has cleared `stage`. `false` for no run at all.
    pub fn is_cleared(&self, character: CharacterId, stage: u32) -> bool {
        self.live.iter().any(|i| i.members.contains(&character) && i.cleared.contains(&stage))
    }
}

/// Where a stage's `next00` leads: stage 1 to 5 go one on, and the last stage (`80000400`)
/// goes to `<Bonus>` (`80000500`). `None` for the Bonus, the Exit, and anything else -
/// neither has a forward portal [L].
pub fn next_stage(stage: u32) -> Option<u32> {
    (is_quest_map(stage) && stage < 80_000_500).then_some(stage + 100)
}

/// Whether `map` is one of the seven quest fields.
pub fn is_quest_map(map: u32) -> bool {
    (STAGE_1..=EXIT_MAP).contains(&map) && map % 100 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn party(leader: u32, members: &[u32]) -> Party {
        Party {
            id: 1,
            name: "P".into(),
            leader,
            members: members.to_vec(),
            pickup_rights: 0,
        }
    }

    fn at(level: u16) -> impl FnMut(CharacterId) -> Option<Candidate> {
        move |id| Some(Candidate::present(id, &format!("C{id}"), level))
    }

    /// Each of the owner's four rules, refused on its own terms.
    #[test]
    fn the_gate_states_every_rule_wisp_gave() {
        assert_eq!(check(1, None, at(30)), Err(Refusal::NoParty));
        assert_eq!(check(2, Some(&party(1, &[1, 2])), at(30)), Err(Refusal::NotLeader));
        assert_eq!(check(1, Some(&party(1, &[1])), at(30)), Err(Refusal::TooSmall { size: 1 }));
        assert_eq!(
            check(1, Some(&party(1, &[1, 2, 3, 4, 5])), at(30)),
            Err(Refusal::TooLarge { size: 5 })
        );
        // Two is enough and four is allowed - the two ends of the owner's range.
        assert!(check(1, Some(&party(1, &[1, 2])), at(21)).is_ok(), "two at exactly 21");
        assert!(check(1, Some(&party(1, &[1, 2, 3, 4])), at(21)).is_ok(), "four at exactly 21");
    }

    /// One level under and nobody goes; the line names the lowest member, not the first.
    #[test]
    fn a_single_underlevelled_member_refuses_the_whole_party() {
        let p = party(1, &[1, 2, 3]);
        let levels = |id: CharacterId| {
            let level = match id {
                1 => 30,
                2 => 20, // one short
                _ => 14, // lower still, and listed last
            };
            Some(Candidate::present(id, &format!("C{id}"), level))
        };
        let err = check(1, Some(&p), levels).unwrap_err();
        assert_eq!(err, Refusal::Underlevelled { name: "C3".into(), level: 14 }, "the LOWEST");
        assert!(err.line().contains("C3") && err.line().contains("21"), "{}", err.line());
        // Raise them all to exactly the minimum and it passes.
        assert!(check(1, Some(&p), at(MIN_LEVEL)).is_ok());
    }

    /// A member whose record cannot be read refuses rather than being assumed eligible.
    #[test]
    fn an_unreadable_member_refuses_rather_than_being_assumed_high_enough() {
        let p = party(1, &[1, 2]);
        let err = check(1, Some(&p), |id| (id == 1).then(|| Candidate::present(1, "A", 99)));
        assert_eq!(err, Err(Refusal::UnknownMember { character: 2 }));
    }

    /// **Everyone has to be online and standing in Kerning City.** The owner, 2026-09-22. An
    /// absent member is refused before any level is mentioned, and the two cases are
    /// worded apart: away on this channel says Kerning City, invisible to it does not
    /// claim they are offline.
    #[test]
    fn a_member_who_is_not_in_kerning_city_with_you_refuses_the_entry() {
        let p = party(1, &[1, 2]);
        // Online, but standing somewhere else.
        let away = |id: CharacterId| {
            let mut c = Candidate::present(id, &format!("C{id}"), 30);
            if id == 2 {
                c.here = false;
            }
            Some(c)
        };
        let err = check(1, Some(&p), away).unwrap_err();
        assert_eq!(err, Refusal::Elsewhere { name: "C2".into() });
        assert!(err.line().contains("Kerning City"), "{}", err.line());

        // Not visible on this channel at all.
        let gone = |id: CharacterId| {
            let mut c = Candidate::present(id, &format!("C{id}"), 30);
            if id == 2 {
                c.online = false;
                c.here = false;
            }
            Some(c)
        };
        let err = check(1, Some(&p), gone).unwrap_err();
        assert_eq!(err, Refusal::NotHere { name: "C2".into() });
        let line = err.line();
        assert!(line.contains("not online here"), "{line}");
        assert!(!line.contains("offline"), "must not claim offline when it could be another channel: {line}");

        // **Absence outranks level**: the away member is also under-levelled, and the line
        // still names the absence, because that is the one the party can act on.
        let both = |id: CharacterId| {
            let mut c = Candidate::present(id, &format!("C{id}"), 30);
            if id == 2 {
                c.here = false;
                c.level = 5;
            }
            Some(c)
        };
        assert_eq!(check(1, Some(&p), both).unwrap_err(), Refusal::Elsewhere { name: "C2".into() });
    }

    /// An instance is per RUN, not per party, and a re-entry replaces the old one.
    #[test]
    fn an_instance_is_per_run_and_a_re_entry_replaces_the_old_one() {
        let mut runs = Runs::default();
        let first = runs.open(7_001, vec![11, 12], 1_000);
        let second = runs.open(7_001, vec![11, 12], 1_000);
        assert_ne!(first.id, second.id, "a second run is a different field");
        assert_eq!(runs.instance_of(11).map(|i| i.id), Some(second.id), "only the newer one is live");
        assert!(runs.close(second.id));
        assert_eq!(runs.instance_of(11), None);
        assert!(!runs.close(second.id), "closing twice is not an error but changes nothing");
    }

    /// **The clock is a wall clock and it runs out.** The owner, 2026-09-22: members are kicked
    /// to the exit map when the limit passes.
    #[test]
    fn an_instance_runs_out_and_is_handed_over_exactly_once() {
        let mut runs = Runs::default();
        let inst = runs.open(7_100, vec![21, 22], 10_000);
        assert_eq!(inst.deadline_unix, 10_000 + i64::from(TIME_LIMIT_S));
        assert_eq!(inst.remaining_s(10_000), TIME_LIMIT_S);
        assert_eq!(inst.remaining_s(10_000 + 60), TIME_LIMIT_S - 60);
        assert!(!inst.is_over(inst.deadline_unix - 1));
        assert!(inst.is_over(inst.deadline_unix));
        assert_eq!(inst.remaining_s(inst.deadline_unix + 999), 0, "saturates rather than wrapping");

        // Nothing is over yet, so nothing is taken.
        assert!(runs.take_expired(10_000).iter().all(|i| i.id != inst.id));
        // Past the deadline it comes out ONCE - a second tick, from the other member's
        // session, must not warp anybody a second time.
        let over = runs.take_expired(inst.deadline_unix);
        assert_eq!(over.iter().filter(|i| i.id == inst.id).count(), 1);
        assert!(runs.take_expired(inst.deadline_unix).iter().all(|i| i.id != inst.id));
        assert_eq!(runs.instance_of(21), None, "and it is gone from the registry");
    }

    /// A member who leaves is dropped from the run; the last one out closes it.
    #[test]
    fn dropping_members_empties_and_then_forgets_the_instance() {
        let mut runs = Runs::default();
        let inst = runs.open(7_200, vec![31, 32], 10_000);
        assert_eq!(runs.drop_member(31).map(|i| i.id), Some(inst.id));
        assert_eq!(runs.instance_of(31), None, "they are out");
        assert_eq!(runs.instance_of(32).map(|i| i.id), Some(inst.id), "the other one is still in");
        assert_eq!(runs.drop_member(32).map(|i| i.id), Some(inst.id));
        assert_eq!(runs.instance_of(32), None);
        assert_eq!(runs.drop_member(32), None, "nobody left, and the run is forgotten");
    }

    /// **A clear belongs to one run.** The owner, 2026-09-23: *"just because one party instance
    /// cleared, doesn't mean that all party instances cleared."* Two runs on the same stage;
    /// clearing one leaves the other closed, and a second clear is reported as not new.
    #[test]
    fn a_stage_clear_belongs_to_one_run_and_no_other() {
        let mut runs = Runs::default();
        let a = runs.open(7_300, vec![41], 10_000);
        let b = runs.open(7_301, vec![42], 10_000);
        assert_ne!(a.id, b.id);
        assert!(!runs.is_cleared(41, STAGE_1) && !runs.is_cleared(42, STAGE_1));

        assert_eq!(runs.clear_stage(41, STAGE_1), Some(true));
        assert!(runs.is_cleared(41, STAGE_1));
        assert!(!runs.is_cleared(42, STAGE_1), "the other run's stage 1 is still closed");
        assert!(!runs.is_cleared(41, STAGE_1 + 100), "and so is this run's next stage");
        assert_eq!(runs.clear_stage(41, STAGE_1), Some(false), "already cleared is not new");

        assert_eq!(runs.clear_stage(99_999, STAGE_1), None, "not in a run");
        assert_eq!(runs.clear_stage(42, 80_000_500), None, "the Bonus has no way forward");
        runs.close(a.id);
        runs.close(b.id);
    }

    /// Stage 1 to 5 go one on, the last stage goes to the Bonus, and nothing leaves the
    /// Bonus or the Exit by `next00`.
    #[test]
    fn next00_walks_the_stages_in_order() {
        assert_eq!(next_stage(80_000_000), Some(80_000_100));
        assert_eq!(next_stage(80_000_300), Some(80_000_400));
        assert_eq!(next_stage(80_000_400), Some(80_000_500));
        assert_eq!(next_stage(80_000_500), None);
        assert_eq!(next_stage(EXIT_MAP), None);
        assert_eq!(next_stage(ENTRY_MAP), None);
    }

    /// **The temporary solo switch moves the session's minimum, not the rule.** `check` still
    /// refuses a party of one; `check_min` with the entry minimum lets it through.
    #[test]
    fn the_solo_test_switch_does_not_move_the_rule() {
        let solo = party(1, &[1]);
        assert_eq!(check(1, Some(&solo), at(30)), Err(Refusal::TooSmall { size: 1 }));
        let entered = check_min(ENTRY_MIN_PARTY, 1, Some(&solo), at(30));
        if ENTRY_MIN_PARTY <= 1 {
            assert!(entered.is_ok(), "{entered:?}");
        } else {
            assert_eq!(entered, Err(Refusal::TooSmall { size: 1 }));
        }
    }

    /// The seven quest fields, and nothing either side of them.
    #[test]
    fn the_quest_maps_are_the_seven_and_only_the_seven() {
        for m in [80_000_000, 80_000_100, 80_000_200, 80_000_300, 80_000_400, 80_000_500, 80_000_600] {
            assert!(is_quest_map(m), "{m}");
        }
        for m in [10_003_000, 80_000_001, 80_000_050, 80_000_700, 79_999_900] {
            assert!(!is_quest_map(m), "{m}");
        }
    }
}
