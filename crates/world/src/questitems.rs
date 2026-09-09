//! **Which items a drop table may offer a given player**, when the item is a quest item.
//!
//! The owner, from a screenshot of a full ETC tab: *"Omok Piece: Slime - Quest Item - A
//! slime-shaped Omok piece to play Omok."* Players are collecting quest items for quests
//! they have never accepted. This module answers one question and nothing else:
//!
//! > may this item id be offered to this character right now?
//!
//! Where a drop lands, who owns it, and how long it lives are all `crate::drops` and
//! `crate::droptables`. This module never mints, moves or expires anything.
//!
//! # The flag is `info/quest`, and it is NOT the id range
//!
//! `gm-handbook/itemdata.txt` column 3 is `info/quest`, read straight out of the client's
//! own `Item.wz` by `tools/dump_itemdata.py` and already parsed into
//! [`crate::shops::ItemData::quest`] - the same field the "do not allow quest items to be
//! sold" rule uses. **119 of the 2785 items in that file carry it.** Enumerated, not
//! sampled: the whole file was counted before anything was filtered, and
//! [`tests::the_flag_count_is_the_whole_file_enumerated`] re-counts it against the real
//! file so a regeneration that changes the number fails loudly.
//!
//! **The `4000000+` ETC range is a different set and it is the wrong one.** Of the 359 ETC
//! rows in that file only 117 carry `info/quest`; `4000001` *Snail Shell* - which every
//! low-level mob drops and no quest gates - carries `0`. Filtering on the range would have
//! removed the ordinary mob-drop ETC items that quests *consume* but do not *own*, which is
//! 112 further ids (Blue Snail Shell, Mushroom Spore, Pure Water, Firewood, ...). Those are
//! required by quests and are **not** quest items, and they must keep dropping for
//! everybody. [`tests::the_id_range_is_not_the_flag`] pins both directions.
//!
//! The positive control is the item from the screenshot: **4031047, Omok Piece: Slime,
//! `info/quest = 1`, required by quest 80002.** If this module's detection ever stops
//! flagging it, the detection is wrong and the data is not - see
//! [`tests::the_screenshot_item_is_flagged_and_mapped`].
//!
//! # Which quest an item belongs to
//!
//! `gm-handbook/questreq.txt`'s `item` rows - the client's own `Check` requirements, the
//! same file `net::quest::QuestRequirementTable` already parses for kill quests. Every one
//! of the 329 item rows is `state 1` (gates COMPLETING), which is what "this quest wants you
//! to be holding it" means. **104 of the 119 flagged items are named by at least one quest**
//! and one item can belong to several.
//!
//! # The orphan policy, stated rather than defaulted
//!
//! **15 flagged items are named by no quest in this client's `Quest.wz`.** Life Scroll,
//! Bartos's Letter, the four construction-site and Ellinia/Ludibrium tickets, Betty's
//! report, and so on - script props for conversations, not collection targets.
//!
//! > **An orphan quest item never drops.** [`ORPHANS_DROP`] is `false`.
//!
//! That is the safe direction and it is a policy, **[I]**, not a measurement. The failure it
//! avoids is the one the owner reported: an item nobody can ever hand in, accumulating in a bag
//! that has a finite number of slots. The failure it risks is an item a script hands out via
//! a drop that nobody has yet built - and none of the 15 is in `data/drops.txt`, so today
//! this decision changes nothing at all. It is written down here so that changing it is one
//! edit rather than an argument.
//!
//! # The Dark Marble is exempt, deliberately
//!
//! `crate::secondjob::marble_for_kill` gates the four Dark Marbles on **(mob template, map)**
//! - a rule strictly narrower than "has the quest", since the only route into a test field is
//! `enter_test_field_on_quest_start`, which *is* the quest starting. Gating them here as well
//! would let this module silently veto `secondjob::MARBLE_DROP_IS_CERTAIN`, and would break a
//! GM who teleports into a test field to check it. So [`is_exempt`] excludes them and
//! `secondjob` stays the single owner of that decision.
//!
//! # Fail-open, and it says so
//!
//! `gm-handbook/` is generated and gitignored, so both inputs **can** be absent. An empty
//! table flags nothing and therefore filters nothing, which is exactly the behaviour that
//! shipped before this module existed. [`QuestItems::is_armed`] exists so the startup banner
//! can say which of the two it is - `CLAUDE.md`'s `!map` guard is the precedent, and the
//! lesson attached to it is that a guard which quietly disappears is worse than one that
//! refuses.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

/// **May an item that carries `info/quest` but belongs to no quest ever drop?**
///
/// `false`. See the module docs - this is the stated policy, **[I]**, and the single place
/// to change it.
pub const ORPHANS_DROP: bool = false;

/// One character's eligibility: who they are, and the quests they have **in progress**.
///
/// Completed quests are deliberately absent rather than carried and filtered later. A
/// completed quest is not a reason to keep handing out its item, and a set that contained
/// both states would need every reader to remember which one it wanted - which is the shape
/// of the bug this project keeps re-finding.
pub type Eligibility = (u32, BTreeSet<u32>);

/// The in-progress quest ids out of a character's stored quest rows.
///
/// **In progress only.** `store::Store::quest_rows` returns completed rows too, and the
/// whole point of this filter is that a finished quest stops earning its item.
pub fn active_quest_ids(rows: &[store::QuestRow]) -> BTreeSet<u32> {
    rows.iter()
        .filter(|r| r.state == store::QuestState::InProgress)
        .map(|r| r.quest_id)
        .collect()
}

/// **The eligibility of everyone who could be handed a drop from one kill.**
///
/// One database read per character, taken once per kill rather than once per item: a kill
/// drops several things and nobody's quest book changes between two items falling off the
/// same mob. The order is the caller's and is preserved - `session::combat` walks its damage
/// ranking and stops at the first candidate it can deliver to.
///
/// **A read that fails is treated as "no quests in progress", and says so in the log.** That
/// direction is deliberate: the alternative is handing out a quest item because the database
/// hiccuped, which is the bug this whole module exists to stop, and an item that did not drop
/// is recoverable where a bag full of them is what the owner reported. It is logged rather than
/// swallowed because a silent refusal is the other failure this project keeps paying for.
pub fn audience_for(store: &store::Store, characters: &[u32]) -> Vec<Eligibility> {
    characters
        .iter()
        .map(|&id| match store.quest_rows(id) {
            Ok(rows) => (id, active_quest_ids(&rows)),
            Err(e) => {
                crate::server::log(&format!(
                    "   quest items: could not read character {id}'s quest rows ({e}); treating \
                     them as having NO quest in progress, so no quest item will be offered to \
                     them from this kill"
                ));
                (id, BTreeSet::new())
            }
        })
        .collect()
}

/// Items this module must never gate, whatever their flag says.
///
/// Two groups, and both are cases where another module owns the item with a narrower rule.
///
/// **The four Dark Marbles** - `crate::secondjob` owns them; see the module docs.
///
/// **The two scrolls `crate::scrolls` repurposes**, added 2026-09-09, and this one would
/// otherwise have shipped silently broken. `4031065` and `4031066` carry `info/quest = 1` in
/// the client's own data and are named by **no quest in this client** - 0 hits across
/// `questlines.txt` and `questreq.txt` - which makes them orphans, and [`ORPHANS_DROP`] is
/// `false`. So the global drop rows the owner asked for would have been suppressed at runtime and
/// the two scrolls would never have appeared. At 0.01% that is indistinguishable from bad luck
/// for a very long time.
///
/// The orphan policy's own stated purpose is to stop *"an item nobody can ever hand in,
/// accumulating in a bag"*. That premise does not hold here: these two have a use, `!scroll`,
/// and are consumed by it. So the exemption is the policy applied, not an exception to it.
///
/// `4001009` used to be listed here as **not** needing an exemption - its `info/quest` is 0,
/// so nothing gated it. It was dropped from the feature entirely on 2026-09-09 because its
/// `slotMax` is 0, and the note is kept only so the next reader does not go looking for it.
pub fn is_exempt(item_id: u32) -> bool {
    crate::secondjob::is_marble(item_id) || crate::scrolls::REPURPOSED.contains(&item_id)
}

/// Which items are quest items, and which quests want each of them.
#[derive(Debug, Clone, Default)]
pub struct QuestItems {
    /// Every id whose `info/quest` is 1. **The whole file, enumerated before any filter.**
    flagged: BTreeSet<u32>,
    /// `itemId -> quest ids that name it in a Check`. Only flagged items are kept: an
    /// ordinary item that a quest happens to consume is not gated by this module at all, and
    /// keeping its quests here would invite a reader to gate it.
    by_item: BTreeMap<u32, Vec<u32>>,
    /// How many item requirement rows the quest file yielded, flagged or not. For the banner
    /// and for [`QuestItems::is_armed`]: zero here with a non-empty `flagged` means the quest
    /// half did not load, which would send every quest item to the orphan policy.
    item_requirement_rows: usize,
}

impl QuestItems {
    /// Build from the already-parsed item table and the raw `questreq.txt` text.
    ///
    /// It takes `item_data` rather than a second path so there is exactly **one** parse of
    /// `itemdata.txt` in the process and no way for the sell rule and the drop rule to
    /// disagree about which items are quest items.
    pub fn build(item_data: &HashMap<u32, crate::shops::ItemData>, questreq_text: &str) -> Self {
        let flagged: BTreeSet<u32> =
            item_data.iter().filter(|(_, d)| d.quest).map(|(id, _)| *id).collect();
        let mut by_item: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        let mut rows = 0usize;
        for line in questreq_text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // questId <TAB> state <TAB> kind <TAB> slot <TAB> id <TAB> count. Same shape as
            // `net::quest::QuestRequirementTable::parse`; a short or unparseable row is
            // skipped rather than half-read, for the reason `shops::load_item_data` gives.
            let f: Vec<&str> = line.split('\t').map(str::trim).collect();
            if f.len() < 6 || f[2] != "item" {
                continue;
            }
            let (Ok(quest_id), Ok(item_id)) = (f[0].parse::<u32>(), f[4].parse::<u32>()) else {
                continue;
            };
            rows += 1;
            if !flagged.contains(&item_id) {
                continue;
            }
            let qs = by_item.entry(item_id).or_default();
            if !qs.contains(&quest_id) {
                qs.push(quest_id);
            }
        }
        for qs in by_item.values_mut() {
            qs.sort_unstable();
        }
        QuestItems { flagged, by_item, item_requirement_rows: rows }
    }

    /// Read the quest half from a file. A missing file is not an error - see the module docs
    /// on fail-open - and leaves every flagged item an orphan, which
    /// [`QuestItems::is_armed`] reports as *not armed*.
    pub fn load(item_data: &HashMap<u32, crate::shops::ItemData>, questreq: &Path) -> Self {
        let text = std::fs::read_to_string(questreq).unwrap_or_default();
        Self::build(item_data, &text)
    }

    /// **Is this filter actually able to do anything?**
    ///
    /// Both halves have to be present. Flags with no quest rows would send every quest item
    /// to the orphan policy and silently stop 12 real drop-table rows; quest rows with no
    /// flags would gate nothing. The startup banner prints this so a missing `gm-handbook/`
    /// is visible rather than inferred from an empty bag.
    pub fn is_armed(&self) -> bool {
        !self.flagged.is_empty() && self.item_requirement_rows > 0
    }

    /// Does this item carry `info/quest`?
    pub fn is_quest_item(&self, item_id: u32) -> bool {
        self.flagged.contains(&item_id)
    }

    /// The quests that name this item in a `Check`. Empty for an ordinary item **and** for a
    /// flagged item no quest wants - [`QuestItems::is_orphan`] separates those two.
    pub fn quests_for_item(&self, item_id: u32) -> &[u32] {
        self.by_item.get(&item_id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// A quest item that no quest in this client asks for. See [`ORPHANS_DROP`].
    pub fn is_orphan(&self, item_id: u32) -> bool {
        self.is_quest_item(item_id) && !self.by_item.contains_key(&item_id)
    }

    /// **May this item be offered to a character whose in-progress quests are `active`?**
    ///
    /// Three answers, in the order they are decided:
    ///
    /// * not a quest item, or exempt -> **yes**, unconditionally. An ordinary drop is not
    ///   this module's business and must be completely unaffected.
    /// * a quest item no quest wants -> [`ORPHANS_DROP`].
    /// * otherwise -> yes iff one of its quests is in progress **now**. A completed quest is
    ///   not in `active`, so finishing a quest stops its item dropping, which is the half of
    ///   the request a "have you ever touched this quest" test would get wrong.
    pub fn may_receive(&self, item_id: u32, active: &BTreeSet<u32>) -> bool {
        if is_exempt(item_id) || !self.is_quest_item(item_id) {
            return true;
        }
        match self.by_item.get(&item_id) {
            None => ORPHANS_DROP,
            Some(quests) => quests.iter().any(|q| active.contains(q)),
        }
    }

    /// Every character in `audience` who may be offered this item, in the order given.
    ///
    /// The order is the caller's ranking and is load-bearing: `combat` walks it and stops at
    /// the first candidate it can actually deliver to, so re-ordering here would change who
    /// gets the drop.
    pub fn recipients_for(&self, item_id: u32, audience: &[Eligibility]) -> Vec<u32> {
        audience
            .iter()
            .filter(|(_, active)| self.may_receive(item_id, active))
            .map(|(id, _)| *id)
            .collect()
    }

    /// **May this one character, drawn from an audience already read, receive this item?**
    ///
    /// The lookup form of [`QuestItems::may_receive`], for a caller that has a ranking to walk
    /// and does not want a database read per step. A character who is **not in the audience**
    /// is refused a gated item rather than allowed one: their quests were never read, and
    /// "not measured" must not read as "eligible".
    pub fn may_receive_in(&self, item_id: u32, character: u32, audience: &[Eligibility]) -> bool {
        if is_exempt(item_id) || !self.is_quest_item(item_id) {
            return true;
        }
        audience
            .iter()
            .find(|(id, _)| *id == character)
            .is_some_and(|(_, active)| self.may_receive(item_id, active))
    }

    /// Is there anybody in `audience` this item may be offered to?
    ///
    /// `true` for an empty audience only when the item is not gated at all - an audience of
    /// nobody cannot make a quest item eligible.
    pub fn any_may_receive(&self, item_id: u32, audience: &[Eligibility]) -> bool {
        if is_exempt(item_id) || !self.is_quest_item(item_id) {
            return true;
        }
        audience.iter().any(|(_, active)| self.may_receive(item_id, active))
    }

    /// **Remove from a roll every quest item nobody in `audience` can be given.**
    ///
    /// This is the whole feature in one call, and it is deliberately a filter over the rolled
    /// result rather than over the table: the table is shared by every player on the map and
    /// eligibility is not.
    ///
    /// **Mesos are untouched.** `Rolled::is_mesos` is id `0`, which carries no `info/quest`
    /// row and is therefore not flagged, so it falls out of `may_receive`'s first arm - but
    /// the guard is written here as well because the rule *"players still cannot drop
    /// mesos"* lives one function away and must not be reachable from this one.
    pub fn filter_roll(&self, rolled: &mut Vec<crate::droptables::Rolled>, audience: &[Eligibility]) {
        rolled.retain(|r| r.is_mesos() || self.any_may_receive(r.item_id, audience));
    }

    /// How many items carry `info/quest`. For the startup banner.
    pub fn flagged_count(&self) -> usize {
        self.flagged.len()
    }

    /// How many of those are named by at least one quest.
    pub fn mapped_count(&self) -> usize {
        self.by_item.len()
    }

    /// How many are named by none - the ones [`ORPHANS_DROP`] decides.
    pub fn orphan_count(&self) -> usize {
        self.flagged.len() - self.by_item.len()
    }

    /// Every flagged id, ascending. For a GM command or a test that wants to enumerate
    /// before it filters.
    pub fn flagged(&self) -> impl Iterator<Item = u32> + '_ {
        self.flagged.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::droptables::{DropTables, Rolled, MESOS};
    use crate::shops::ItemData;

    /// The real generated files, or `None` on a clean checkout - `gm-handbook/` is generated
    /// and gitignored, so every test that touches it has to be skippable.
    fn real() -> Option<QuestItems> {
        let items = Path::new("../../gm-handbook/itemdata.txt");
        let reqs = Path::new("../../gm-handbook/questreq.txt");
        if !items.exists() || !reqs.exists() {
            return None;
        }
        let data = crate::shops::load_item_data(items);
        if data.is_empty() {
            return None;
        }
        Some(QuestItems::load(&data, reqs))
    }

    /// A tiny hand-built table: one quest item wanted by quest 80002, one orphan quest item,
    /// one ordinary item. Enough to state every rule without depending on generated data.
    fn fixture() -> QuestItems {
        let mut data = HashMap::new();
        data.insert(4_031_047, ItemData { quest: true, ..ItemData::default() }); // Omok piece
        data.insert(4_031_035, ItemData { quest: true, ..ItemData::default() }); // orphan
        data.insert(4_000_001, ItemData { price: 1, ..ItemData::default() }); // Snail Shell
        QuestItems::build(
            &data,
            "# questId\tstate\tkind\tslot\tid\tcount\n\
             80002\t1\titem\t0\t4031047\t1\n\
             80002\t1\tmob\t0\t100\t10\n\
             1005\t1\titem\t0\t4000000\t3\n",
        )
    }

    fn active(ids: &[u32]) -> BTreeSet<u32> {
        ids.iter().copied().collect()
    }

    #[test]
    fn the_fixture_table_is_the_shape_the_rules_assume() {
        let q = fixture();
        assert!(q.is_armed());
        assert_eq!(q.flagged_count(), 2);
        assert_eq!(q.mapped_count(), 1);
        assert_eq!(q.orphan_count(), 1);
        assert_eq!(q.quests_for_item(4_031_047), &[80002]);
        assert!(q.is_orphan(4_031_035));
        // An item required by a quest but NOT flagged is not in the map at all: it is an
        // ordinary drop and this module must not know about it.
        assert!(!q.is_quest_item(4_000_000));
        assert!(q.quests_for_item(4_000_000).is_empty());
    }

    // ---------------------------------------------------------------- the four rules

    #[test]
    fn an_active_quest_yields_the_item() {
        let q = fixture();
        assert!(q.may_receive(4_031_047, &active(&[80002])));
    }

    #[test]
    fn no_quest_yields_no_item() {
        let q = fixture();
        assert!(!q.may_receive(4_031_047, &active(&[])));
        // And a different quest being active is not the same as this one being active.
        assert!(!q.may_receive(4_031_047, &active(&[80003, 1005])));
    }

    /// **A completed quest is not an active quest**, and this is stated through the real
    /// `QuestRow` states rather than through a hand-built id set, because the whole risk here
    /// is a reader who forgets that `quest_rows` returns both.
    #[test]
    fn a_completed_quest_yields_no_item() {
        let q = fixture();
        let row = |quest_id, state| store::QuestRow {
            quest_id,
            state,
            progress: String::new(),
            started_at: 0,
            completed_at: None,
        };
        let doing = [row(80002, store::QuestState::InProgress)];
        let done = [row(80002, store::QuestState::Complete)];
        assert!(q.may_receive(4_031_047, &active_quest_ids(&doing)), "in progress: it drops");
        assert!(!q.may_receive(4_031_047, &active_quest_ids(&done)), "completed: it stops");
        // The row still exists in the database - this is the difference between "has a row"
        // and "is in progress", which is the mistake this test is here to make impossible.
        assert_eq!(active_quest_ids(&done).len(), 0);
        assert_eq!(active_quest_ids(&doing).len(), 1);
    }

    #[test]
    fn a_non_quest_item_is_unaffected() {
        let q = fixture();
        for who in [active(&[]), active(&[80002]), active(&[1, 2, 3])] {
            assert!(q.may_receive(4_000_001, &who), "Snail Shell is not a quest item");
            assert!(q.may_receive(2_000_000, &who), "an id with no row at all is not gated");
            assert!(q.may_receive(MESOS, &who), "mesos are not an item and are never gated");
        }
    }

    #[test]
    fn an_orphan_quest_item_never_drops() {
        let q = fixture();
        assert!(!ORPHANS_DROP, "the policy this test is pinning");
        for who in [active(&[]), active(&[80002]), active(&[1005])] {
            assert!(!q.may_receive(4_031_035, &who));
        }
    }

    // ---------------------------------------------------------------- per recipient

    /// **Two players killing the same mob can differ**, which is the whole reason this is a
    /// filter over a roll and not over the table.
    #[test]
    fn eligibility_is_per_character_not_per_mob() {
        let q = fixture();
        let audience: Vec<Eligibility> =
            vec![(200, active(&[])), (201, active(&[80002])), (202, active(&[1005]))];
        assert_eq!(q.recipients_for(4_031_047, &audience), vec![201]);
        assert!(!q.may_receive_in(4_031_047, 200, &audience));
        assert!(q.may_receive_in(4_031_047, 201, &audience));
        // A character nobody read is refused a GATED item and allowed an ordinary one.
        assert!(!q.may_receive_in(4_031_047, 999, &audience), "unmeasured is not eligible");
        assert!(q.may_receive_in(4_000_001, 999, &audience));
        // The ordinary item goes to everybody, in the caller's ranking order.
        assert_eq!(q.recipients_for(4_000_001, &audience), vec![200, 201, 202]);
        assert!(q.any_may_receive(4_031_047, &audience));
        assert!(!q.any_may_receive(4_031_047, &audience[..1]));
    }

    #[test]
    fn an_empty_audience_cannot_make_a_quest_item_eligible() {
        let q = fixture();
        assert!(!q.any_may_receive(4_031_047, &[]));
        assert!(q.any_may_receive(4_000_001, &[]), "but an ordinary item is not gated");
    }

    /// **The database round trip, not a hand-built id set.**
    ///
    /// `audience_for` is the only place the per-player answer actually comes from, and the
    /// distinction it has to get right - a row that exists but is `Complete` - is a property
    /// of `store::complete_quest`, not of anything in this file. So it is asserted against a
    /// real store that has actually started and completed a quest.
    #[test]
    fn the_audience_reads_in_progress_from_the_store_and_not_completed() {
        let q = fixture();
        let db = store::Store::open_in_memory().unwrap();
        let account = db.create_account("wisp", "correct horse battery").unwrap();
        let make = |name: &str| {
            db.create_character(
                account,
                0,
                &net::opcode::Character { name: name.into(), ..Default::default() },
            )
            .unwrap()
            .id
        };
        // `doing` is on 80002; `done` handed it in; `never` has not touched it.
        let (doing, done, never) = (make("Doing"), make("Done"), make("Never"));
        assert!(db.start_quest(doing, 80002).unwrap());
        assert!(db.start_quest(done, 80002).unwrap());
        assert!(db.complete_quest(done, 80002).unwrap().is_some());

        let audience = audience_for(&db, &[doing, done, never]);
        assert_eq!(audience.len(), 3, "one entry per character, in the order given");
        assert_eq!(audience[0], (doing, active(&[80002])));
        assert_eq!(audience[1], (done, active(&[])), "completed is not active");
        assert_eq!(audience[2], (never, active(&[])));

        assert_eq!(q.recipients_for(4_031_047, &audience), vec![doing]);
        assert!(q.any_may_receive(4_031_047, &audience));
        assert!(!q.any_may_receive(4_031_047, &audience[1..]), "only the finished and the never");

        let mut got = DropTables::parse(TABLE).roll(2, &mut || 0);
        q.filter_roll(&mut got, &audience[1..]);
        assert_eq!(rolled_ids(&got), vec![MESOS, 4_000_001]);
    }

    // ---------------------------------------------------------------- over a real roll

    const TABLE: &str = "\
2 | 0       | 100 | 12 | 12 | 1 | mesos
2 | 4000001 | 100 | 1  | 1  | 9 | Snail Shell
2 | 4031047 | 100 | 1  | 1  | 5 | Omok Piece: Slime
2 | 4031035 | 100 | 1  | 1  | 5 | Bartos's Letter
";

    fn rolled_ids(rolled: &[Rolled]) -> Vec<u32> {
        rolled.iter().map(|r| r.item_id).collect()
    }

    #[test]
    fn the_filter_runs_over_a_real_roll_and_keeps_mesos() {
        let q = fixture();
        let t = DropTables::parse(TABLE);
        assert!(t.problems.is_empty(), "{:?}", t.problems);

        // Nobody has the quest: the Omok piece and the orphan both go, mesos and the Snail
        // Shell both stay.
        let mut got = t.roll(2, &mut || 0);
        assert_eq!(rolled_ids(&got), vec![MESOS, 4_000_001, 4_031_047, 4_031_035]);
        q.filter_roll(&mut got, &[(200, active(&[]))]);
        assert_eq!(rolled_ids(&got), vec![MESOS, 4_000_001]);
        assert!(got[0].is_mesos(), "mesos survived the filter");

        // One member of the audience has 80002: the piece comes back, the orphan does not.
        let mut got = t.roll(2, &mut || 0);
        q.filter_roll(&mut got, &[(200, active(&[])), (201, active(&[80002]))]);
        assert_eq!(rolled_ids(&got), vec![MESOS, 4_000_001, 4_031_047]);
    }

    /// **The mesos rule is one function away and must not be reachable from this one.**
    ///
    /// `crate::droptables::MESOS` is id 0 and a player-initiated drop of mesos is refused
    /// elsewhere; this asserts only that the quest filter never removes a meso roll and never
    /// invents one, at any eligibility.
    #[test]
    fn the_filter_neither_removes_nor_invents_a_meso_drop() {
        let q = fixture();
        let t = DropTables::parse(TABLE);
        for who in [vec![], vec![(200u32, active(&[]))], vec![(200, active(&[80002]))]] {
            let mut got = t.roll(2, &mut || 0);
            let before = got.iter().filter(|r| r.is_mesos()).count();
            q.filter_roll(&mut got, &who);
            let after = got.iter().filter(|r| r.is_mesos()).count();
            assert_eq!((before, after), (1, 1), "audience {who:?}");
        }
    }

    /// A Dark Marble is `secondjob`'s to decide and must pass through untouched.
    #[test]
    fn a_dark_marble_is_exempt_however_it_is_flagged() {
        let mut data = HashMap::new();
        data.insert(4_031_017, ItemData { quest: true, ..ItemData::default() });
        let q = QuestItems::build(&data, "20002\t1\titem\t0\t4031018\t1\n");
        assert!(q.is_quest_item(4_031_017), "it does carry the flag");
        assert!(q.is_orphan(4_031_017), "and no Check names it, so the policy would refuse it");
        assert!(is_exempt(4_031_017));
        assert!(q.may_receive(4_031_017, &active(&[])), "the exemption wins");
    }

    /// **The call shape `session::combat` has to use, type-checked here.**
    ///
    /// That function is the coordinator's file and this agent may not edit it, so the patch
    /// handed back cannot be compiled in place. What *can* be compiled is every call in it,
    /// with the same types: an `Arc<Store>` field rather than a `&Store`, a `Vec<u32>` party
    /// roster on one arm and a `&[u32]` ranking on the other, and the per-candidate lookup
    /// inside the walk. A patch that type-checks here is not proof that it applies, but a
    /// patch that did not would be a wasted round trip.
    #[test]
    fn the_call_shape_the_drop_site_uses_type_checks() {
        let q = fixture();
        let db = std::sync::Arc::new(store::Store::open_in_memory().unwrap());
        let ranked: &[u32] = &[200, 201];
        let party_here: Vec<u32> = vec![201, 202];
        for party in [false, true] {
            let audience =
                audience_for(&db, if party { party_here.as_slice() } else { ranked });
            let mut rolled = DropTables::parse(TABLE).roll(2, &mut || 0);
            q.filter_roll(&mut rolled, &audience);
            // Nobody in either audience has 80002, so only the ungated rows survive.
            assert_eq!(rolled_ids(&rolled), vec![MESOS, 4_000_001]);
            for r in &rolled {
                for &c in ranked {
                    assert!(q.may_receive_in(r.item_id, c, &audience));
                }
            }
        }
    }

    /// **Players still cannot drop mesos, and nothing here is near that rule.**
    ///
    /// `crate::drops::DropTable::drop_item` - the player's own `0x0070` path - hard-codes
    /// `meso: 0`, so a bag drop is structurally an item and there is no branch that could
    /// make it money. `CLAUDE.md`: a comment describing a guarantee is not the guarantee, and
    /// that line had no test on it. It does now, and it is here rather than in `drops.rs`
    /// because that file belongs to the agent that owns where a drop lands.
    #[test]
    fn a_player_bag_drop_is_never_mesos() {
        let mut t = crate::drops::DropTable::default();
        let placed = t.drop_item(crate::drops::DropFromBag {
            map_id: 104_040_000,
            character_id: 200,
            inv_type: store::InventoryType::Etc,
            slot: 1,
            item: store::Item::bundle(4_000_001, 3),
            x: 0,
            y: 0,
            now_ms: 0,
        });
        let live = t.get(placed.object_id).expect("the drop is on the floor");
        assert!(!live.is_meso(), "a bag drop is always an item");
        assert_eq!(live.meso, 0);
    }

    // ---------------------------------------------------------------- fail-open

    #[test]
    fn an_empty_table_is_not_armed_and_gates_nothing() {
        let q = QuestItems::default();
        assert!(!q.is_armed());
        for id in [4_031_047, 4_031_035, 4_000_001, MESOS] {
            assert!(q.may_receive(id, &active(&[])), "{id} must pass an unarmed filter");
        }
        let mut got = DropTables::parse(TABLE).roll(2, &mut || 0);
        let before = got.len();
        q.filter_roll(&mut got, &[(200, active(&[]))]);
        assert_eq!(got.len(), before, "an unarmed filter is the pre-feature behaviour");
    }

    /// Flags loaded but quest rows missing is **not** armed. Without this it would look
    /// armed, send all 119 items to the orphan policy, and silently stop 12 live drop rows.
    #[test]
    fn flags_without_quest_rows_are_not_armed() {
        let mut data = HashMap::new();
        data.insert(4_031_047, ItemData { quest: true, ..ItemData::default() });
        let q = QuestItems::build(&data, "# only comments\n");
        assert!(!q.is_armed());
        assert_eq!(q.orphan_count(), 1, "it WOULD refuse them, which is why is_armed exists");
    }

    // ---------------------------------------------------------------- the real data

    /// **Enumerate before you filter.** The whole file is counted, and the number is the same
    /// one `crate::shops`' own drift test asserts from the other direction.
    #[test]
    fn the_flag_count_is_the_whole_file_enumerated() {
        let Some(q) = real() else { return };
        assert!(q.is_armed(), "both generated files are present, so the filter must be live");
        assert_eq!(q.flagged_count(), 119, "items carrying info/quest in itemdata.txt");
        assert_eq!(q.mapped_count(), 104, "of those, named by at least one quest");
        assert_eq!(q.orphan_count(), 15, "and named by none - ORPHANS_DROP decides these");
    }

    /// **The positive control.** The item in the owner's screenshot must be flagged, and must map
    /// to the quest that wants it. A detection that cannot see this is wrong; the data is not.
    #[test]
    fn the_screenshot_item_is_flagged_and_mapped() {
        let Some(q) = real() else { return };
        assert!(q.is_quest_item(4_031_047), "4031047 Omok Piece: Slime - the screenshot item");
        assert_eq!(q.quests_for_item(4_031_047), &[80002]);
        assert!(!q.is_orphan(4_031_047));
        assert!(!q.may_receive(4_031_047, &active(&[])), "and it stops for a player with no quest");
        assert!(q.may_receive(4_031_047, &active(&[80002])), "and drops for one doing 80002");
    }

    /// **The negative control, which is the half that says the instrument discriminates.**
    ///
    /// `4000001` Snail Shell sits in the same `4000000+` ETC range and is not a quest item.
    /// It is also *required* by no quest at all, while its neighbours 4000002 and 4000009
    /// are required by quest 1011 and are still not flagged - so "required by a quest" and
    /// "is a quest item" are different sets and the id range is neither of them.
    #[test]
    fn the_id_range_is_not_the_flag() {
        let Some(q) = real() else { return };
        assert!(!q.is_quest_item(4_000_001), "Snail Shell: in the range, flag 0");
        assert!(!q.is_quest_item(4_000_002), "Blue Snail Shell: required by quest 1011, flag 0");
        assert!(!q.is_quest_item(4_000_009), "Orange Mushroom Cap: five quests want it, flag 0");
        assert!(q.is_quest_item(4_000_000), "Jr. Sentinel Shellpiece: same range, flag 1");
        // And the flag is not confined to the range either.
        assert!(q.is_quest_item(1_002_138), "Chief Stan Hat is an EQUIP carrying info/quest");
        assert!(q.is_quest_item(1_302_009), "and so is a sword");
        // Every one of these is in the range; only two of them are flagged.
        let in_range = (4_000_000..5_000_000).filter(|i| q.is_quest_item(*i)).count();
        assert_eq!(in_range, 117, "117 of the 119 are ETC; the other two are equips");
    }

    /// **What this actually changes in `data/drops.txt`**, counted rather than asserted from
    /// memory. Twelve ids, fifteen rows - and every one of them is a real quest's item, so
    /// the orphan policy removes nothing that a mob can currently drop.
    #[test]
    fn the_live_drop_table_is_affected_in_exactly_twelve_places() {
        let Some(q) = real() else { return };
        let path = Path::new("../../data/drops.txt");
        let Ok(text) = std::fs::read_to_string(path) else { return };
        // **Every row in the file**, read here rather than through `DropTables` because that
        // type is addressed by template id and enumerating it would mean guessing the ids -
        // which is the "searching a known list" failure `CLAUDE.md` names twice.
        let mut rows = 0usize;
        let mut gated: BTreeSet<u32> = BTreeSet::new();
        let mut gated_rows = 0usize;
        let mut orphans: BTreeSet<u32> = BTreeSet::new();
        for line in text.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let cols: Vec<&str> = line.split('|').map(str::trim).collect();
            if cols.len() < 5 {
                continue;
            }
            let Ok(item_id) = cols[1].parse::<u32>() else { continue };
            rows += 1;
            if cols[0] == crate::droptables::GLOBAL_KEY {
                // Exempt items are allowed: `is_exempt` names the two scrolls and says why -
                // they carry `info/quest` but belong to no quest, and without the exemption
                // ORPHANS_DROP would have silently suppressed the rows the owner asked for.
                assert!(
                    !q.is_quest_item(item_id) || is_exempt(item_id),
                    "a gated quest item in the GLOBAL table: {item_id}"
                );
            }
            if q.is_quest_item(item_id) && !is_exempt(item_id) {
                gated.insert(item_id);
                gated_rows += 1;
                if q.is_orphan(item_id) {
                    orphans.insert(item_id);
                }
            }
        }
        // 993 scraped rows plus the two GLOBAL scroll rows added by hand on 2026-09-09.
        // Three until later the same day, when Event Trophy was dropped from the feature for
        // not stacking.
        //
        // Counted rather than adjusted: if a re-scrape drops the hand-written rows this falls
        // to 993 and fails, which is the point - the file's own header warns that hand edits
        // do not survive a scrape, and the scrolls would otherwise stop dropping silently.
        assert_eq!(rows, 995, "every parseable row in data/drops.txt");
        assert_eq!(gated_rows, 9, "rows this filter can now remove (15 quest-item rows, 6 marble)");
        assert!(
            gated.contains(&4_031_047),
            "the screenshot item must be one of the rows this changes: {gated:?}"
        );
        assert_eq!(gated.len(), 8, "quest-item ids in data/drops.txt, marbles excluded: {gated:?}");
        assert!(orphans.is_empty(), "no droppable quest item is an orphan today: {orphans:?}");
    }
}
