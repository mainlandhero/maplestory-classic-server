//! The three scrolls this client does not have, and what each does to an equip.
//!
//! The owner, 2026-09-09, specified three items to be repurposed as scrolls MapleStory has and this
//! client's data does not. `research/scroll-command-2026-09-09.md` records the spec verbatim;
//! this module is the rules half of it and **nothing here talks to a client, a store or a
//! packet**. That is deliberate: every rule below is a pure function of the equip's current
//! state, its base template and one roll, so all of it is testable without a database.
//!
//! # The three
//!
//! | item | acts as | what it does |
//! |---|---|---|
//! | `4001009` Event Trophy | Innocence 100% | back to base stats AND base slots; needs no free slot |
//! | `4031065` Scroll of Secrets | Chaos 60% | **always** eats a slot; on success moves one stat by -5..+5 |
//! | `4031066` Treasure Scroll | Clean Slate 60% | returns one slot that a **failed** scroll took |
//!
//! # Two answers from the owner that are rules, not defaults
//!
//! **A failed Chaos still eats the slot.** Asked directly and answered *"failed chaos scroll
//! will eat a slot"*. That is what gives Clean Slate anything to do, and it is why
//! [`Applied::slot_spent`] is set on both arms of the Chaos branch rather than only on success.
//!
//! **The 100% first-use gate is per scroll type, per character, per day.** Answered *"per
//! scroll type per character for the daily gate"*. This module does not know about days - the
//! caller passes [`Chance::Guaranteed`] or [`Chance::Rolled`], because "is this their first
//! Chaos today" is a store question and mixing it in here would make every rule need a
//! database to test.
//!
//! # Clean Slate counts, it does not remember
//!
//! The owner, correcting an earlier reading: *"The Clean Slate should work for any previously failed
//! scroll on the item. If the item previously had 2 failed scroll slots, the player is allowed
//! to use 2 clean slate scrolls on the item. It doesn't necessarily need to be immediately
//! after the failed scroll."*
//!
//! So the per-equip state is a **count** of slots lost to failures, not a flag about the last
//! action. Their earlier sentence - *"This will not return an enhancement slot if the previous
//! scroll action was successful"* - is that count being zero, and needs no separate rule.
//!
//! # Where the count lives, and why not in `EquipStats`
//!
//! [`EquipState::failed_slots`] is **server-only and never reaches the wire.** It is not in
//! `net::opcode::EquipStats`, because that struct maps exhaustively onto both the packet and
//! the 26 stored columns - adding a field there would change a packet.
//!
//! The client does carry a field that means this in the real game: `EquipOptions::unknown_b1`,
//! `item+0x102`, which the v214 reference calls `cuc`, the successful-upgrade count. Using it
//! would have needed no new storage at all. **It was checked and rejected**: a field scan of
//! `+0x102` returns four sites and two of them are `lea rcx, [reg+0x102]`, the address handed
//! off to code nobody has read. `CLAUDE.md` records that exact shape defeating a write-scan
//! once already (`mob+0x42c`, found only by dropping the `--write` filter), so "nothing names
//! it" is not "nothing reads it", and writing a client-visible field on that evidence is the
//! assumption this file exists to avoid.

use net::opcode::EquipStatSet;

/// `4001009` Event Trophy, acting as an Innocence Scroll 100%.
pub const INNOCENCE: u32 = 4_001_009;

/// `4031065` Scroll of Secrets, acting as a Chaos Scroll 60%.
pub const CHAOS: u32 = 4_031_065;

/// `4031066` Treasure Scroll, acting as a Clean Slate 60%.
pub const CLEAN_SLATE: u32 = 4_031_066;

/// The success chance of Chaos and Clean Slate when the daily free pass is spent, in percent.
pub const ROLLED_SUCCESS_PCT: u32 = 60;

/// **How many of one scroll fit in a bag slot.** The owner, 2026-09-09: *"can we make all of these
/// items stackable up to a 100 please?"*
///
/// Applied in `crate::shops::max_stack`, which is the one place that rule lives, and applied
/// as an OVERRIDE: `4031065` and `4031066` carry `info/slotMax = 1` in the client's own data,
/// which describes the quest props they are there rather than the scrolls they are here.
pub const STACK_LIMIT: u16 = 100;

/// The largest amount Chaos moves a stat by, in either direction.
///
/// The owner: *"randomly rolls one item stat to go up or down 0 to 5 points"*, so the swing is
/// `-5..=5` **inclusive of zero** - a roll that moves nothing is a success that did nothing,
/// which is different from a failure and must not be reported as one.
pub const CHAOS_MAX_SWING: i32 = 5;

/// Which of the three, from the item id the player used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scroll {
    Innocence,
    Chaos,
    CleanSlate,
}

impl Scroll {
    /// `None` for anything that is not one of the three.
    pub fn from_item_id(item_id: u32) -> Option<Self> {
        match item_id {
            INNOCENCE => Some(Scroll::Innocence),
            CHAOS => Some(Scroll::Chaos),
            CLEAN_SLATE => Some(Scroll::CleanSlate),
            _ => None,
        }
    }

    pub fn item_id(self) -> u32 {
        match self {
            Scroll::Innocence => INNOCENCE,
            Scroll::Chaos => CHAOS,
            Scroll::CleanSlate => CLEAN_SLATE,
        }
    }

    /// The name the dialogue uses. The client has no string for these, so we supply one.
    pub fn name(self) -> &'static str {
        match self {
            Scroll::Innocence => "Event Trophy",
            Scroll::Chaos => "Scroll of Secrets",
            Scroll::CleanSlate => "Treasure Scroll",
        }
    }

    /// Does this one roll at all? Innocence is 100% by specification, always.
    pub fn always_succeeds(self) -> bool {
        matches!(self, Scroll::Innocence)
    }
}

/// Whether this use gets the daily free pass.
///
/// The caller decides - it is a per-scroll-type, per-character, per-UTC-day store question,
/// and keeping it out of here is what lets every rule be tested without a database.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chance {
    /// The player's first use of this scroll type today: succeeds outright.
    Guaranteed,
    /// Any later use: [`ROLLED_SUCCESS_PCT`].
    Rolled,
}

/// The equip as it stands, plus the one server-only number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EquipState {
    /// `EquipOptions::remaining_enhancements` - the client's *"Remaining Enhancements: %d"*.
    pub remaining: u8,
    /// **Server-only.** How many slots this item has lost to FAILED scrolls and not yet had
    /// returned. Never sent; see the module docs.
    pub failed_slots: u8,
    /// The item's current stats.
    pub stats: EquipStatSet,
}

/// What the item's template says it started as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EquipBase {
    /// `tuc` from `gm-handbook/equips.txt`. The client compares
    /// `remaining_enhancements` against this, so nothing may exceed it.
    pub tuc: u8,
    /// The template's own stats - what Innocence reverts to, and the set Chaos may touch.
    pub stats: EquipStatSet,
}

/// Every stat Chaos may move, as `(name, getter, setter)`.
///
/// **Exhaustive over [`EquipStatSet`] on purpose.** A stat missing from this table is a stat
/// Chaos can never roll, silently - so the test `the_stat_table_covers_every_field` destructures
/// an `EquipStatSet` and fails to compile if a field is added and not listed here.
#[allow(clippy::type_complexity)]
const STATS: &[(&str, fn(&EquipStatSet) -> u16, fn(&mut EquipStatSet, u16))] = &[
    ("STR", |s| s.inc_str, |s, v| s.inc_str = v),
    ("DEX", |s| s.inc_dex, |s, v| s.inc_dex = v),
    ("INT", |s| s.inc_int, |s, v| s.inc_int = v),
    ("LUK", |s| s.inc_luk, |s, v| s.inc_luk = v),
    ("Max HP", |s| s.inc_mhp, |s, v| s.inc_mhp = v),
    ("Max MP", |s| s.inc_mmp, |s, v| s.inc_mmp = v),
    ("Speed", |s| s.inc_speed, |s, v| s.inc_speed = v),
    ("Jump", |s| s.inc_jump, |s, v| s.inc_jump = v),
    ("Weapon Attack", |s| s.inc_pad, |s, v| s.inc_pad = v),
    ("Magic Attack", |s| s.inc_mad, |s, v| s.inc_mad = v),
    ("Weapon Defense", |s| s.inc_pdd, |s, v| s.inc_pdd = v),
    ("Magic Defense", |s| s.inc_mdd, |s, v| s.inc_mdd = v),
    ("Accuracy", |s| s.inc_acc, |s, v| s.inc_acc = v),
    ("Avoidability", |s| s.inc_eva, |s, v| s.inc_eva = v),
    ("Critical", |s| s.inc_crt, |s, v| s.inc_crt = v),
    ("Craft", |s| s.inc_crd, |s, v| s.inc_crd = v),
    ("WAT", |s| s.inc_wat, |s, v| s.inc_wat = v),
];

/// A refusal, with the line the player is shown.
///
/// **Every refusal carries its own wording.** `CLAUDE.md`: a refusal reported to nobody looks
/// exactly like a frozen UI, and this project has spent runs chasing that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Chaos needs a slot to take.
    NoSlotsLeft,
    /// Clean Slate needs a failure to undo.
    NothingToRestore,
    /// Clean Slate would push the item past its own template's `tuc`.
    AlreadyAtBase,
}

impl Refusal {
    pub fn line(self) -> &'static str {
        match self {
            Refusal::NoSlotsLeft => "That item has no enhancement slots left.",
            Refusal::NothingToRestore => "That item has no failed enhancement slots to restore.",
            Refusal::AlreadyAtBase => "That item already has all of its enhancement slots.",
        }
    }
}

/// What happened, and the state to write back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Applied {
    pub after: EquipState,
    /// Did the roll succeed? Innocence is always `true`.
    pub succeeded: bool,
    /// Did this use consume an enhancement slot? True for Chaos on **both** arms.
    pub slot_spent: bool,
    /// The stat Chaos moved and by how much, for the line the player reads. `None` when
    /// nothing moved a stat - a failed Chaos, a Clean Slate, or an Innocence.
    pub stat_change: Option<(&'static str, i32)>,
}

/// Apply one scroll. `roll` is any number; only its residues are used, so a caller may pass a
/// single random value.
///
/// # The order is the rule
///
/// `CLAUDE.md`'s Heena lesson - *every effect hangs off the transition, not the request* - so
/// this returns `Err` **before** producing any state when the item cannot take the scroll, and
/// the caller has nothing to half-apply. The scroll item is consumed by the caller on every
/// path that returns `Ok`, success or failure, per the owner's *"the item is deducted from the
/// player's inventory and the scroll action performed."*
pub fn apply(
    scroll: Scroll,
    base: &EquipBase,
    state: &EquipState,
    chance: Chance,
    roll: u64,
) -> Result<Applied, Refusal> {
    match scroll {
        // Reverts everything, and needs no free slot - the owner: "This scroll does not need
        // enhancement slots available on the item to be used." So it has no refusal at all,
        // and it clears the failed count because there are no longer any failures to undo.
        Scroll::Innocence => Ok(Applied {
            after: EquipState { remaining: base.tuc, failed_slots: 0, stats: base.stats },
            succeeded: true,
            slot_spent: false,
            stat_change: None,
        }),

        Scroll::Chaos => {
            if state.remaining == 0 {
                return Err(Refusal::NoSlotsLeft);
            }
            let succeeded = succeeds(chance, roll);
            let mut after = *state;
            // **Both arms.** The owner: "failed chaos scroll will eat a slot."
            after.remaining -= 1;
            let mut stat_change = None;
            if succeeded {
                if let Some((name, delta)) = roll_one_stat(base, &mut after.stats, roll) {
                    stat_change = Some((name, delta));
                }
            } else {
                // The slot it just ate is now a slot Clean Slate can give back.
                after.failed_slots = after.failed_slots.saturating_add(1);
            }
            Ok(Applied { after, succeeded, slot_spent: true, stat_change })
        }

        Scroll::CleanSlate => {
            if state.failed_slots == 0 {
                return Err(Refusal::NothingToRestore);
            }
            // Cannot exceed the template. The client compares `remaining_enhancements` against
            // `ITEMINFO.tuc`, so a value above it is a number the client has an opinion about.
            if state.remaining >= base.tuc {
                return Err(Refusal::AlreadyAtBase);
            }
            let succeeded = succeeds(chance, roll);
            let mut after = *state;
            if succeeded {
                after.remaining += 1;
                after.failed_slots -= 1;
            }
            // A failed Clean Slate takes nothing - it only fails to give.
            Ok(Applied { after, succeeded, slot_spent: false, stat_change: None })
        }
    }
}

fn succeeds(chance: Chance, roll: u64) -> bool {
    match chance {
        Chance::Guaranteed => true,
        Chance::Rolled => (roll % 100) < u64::from(ROLLED_SUCCESS_PCT),
    }
}

/// Move one stat the base item actually has, by `-5..=5`.
///
/// `None` when the item has no non-zero base stat at all - a decorative equip. That is a
/// success that changed nothing, not a failure, and the caller says so.
fn roll_one_stat(
    base: &EquipBase,
    stats: &mut EquipStatSet,
    roll: u64,
) -> Option<(&'static str, i32)> {
    // **"This will only change stat that the item already has as a base stat."** So the
    // candidate set is the template's non-zero stats, not the item's current ones - a stat
    // Chaos previously drove to zero is still a candidate.
    let candidates: Vec<usize> =
        (0..STATS.len()).filter(|&i| (STATS[i].1)(&base.stats) > 0).collect();
    if candidates.is_empty() {
        return None;
    }
    // Independent residues: the choice of stat and the size of the swing must not be locked
    // to each other, or a given roll could never produce some pairs at all.
    let pick = candidates[(roll / 100) as usize % candidates.len()];
    let (name, get, set) = STATS[pick];
    let span = (CHAOS_MAX_SWING * 2 + 1) as u64; // -5..=5
    let delta = ((roll / 100 / 64) % span) as i32 - CHAOS_MAX_SWING;
    let now = i32::from(get(stats));
    // Clamped at zero: `EquipStatSet` is `u16`, so a negative stat is not representable, and
    // clamping is the only behaviour that does not silently wrap to 65535.
    let next = (now + delta).max(0);
    set(stats, next as u16);
    Some((name, next - now))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base(tuc: u8, pad: u16) -> EquipBase {
        EquipBase { tuc, stats: EquipStatSet { inc_pad: pad, ..Default::default() } }
    }

    fn state(remaining: u8, failed: u8, pad: u16) -> EquipState {
        EquipState {
            remaining,
            failed_slots: failed,
            stats: EquipStatSet { inc_pad: pad, ..Default::default() },
        }
    }

    /// **The stat table must cover every field of `EquipStatSet`.**
    ///
    /// A field missing from `STATS` is a stat Chaos can never roll, and nothing else would
    /// say so. This destructures the struct, so adding a field breaks the build here rather
    /// than quietly shrinking the candidate set.
    #[test]
    fn the_stat_table_covers_every_field() {
        let EquipStatSet {
            inc_str: _, inc_dex: _, inc_int: _, inc_luk: _, inc_mhp: _, inc_mmp: _,
            inc_speed: _, inc_jump: _, inc_pad: _, inc_mad: _, inc_pdd: _, inc_mdd: _,
            inc_acc: _, inc_eva: _, inc_crt: _, inc_crd: _, inc_wat: _,
        } = EquipStatSet::default();
        assert_eq!(STATS.len(), 17, "every EquipStatSet field must be rollable by Chaos");
    }

    #[test]
    fn the_three_item_ids_map_and_nothing_else_does() {
        assert_eq!(Scroll::from_item_id(4_001_009), Some(Scroll::Innocence));
        assert_eq!(Scroll::from_item_id(4_031_065), Some(Scroll::Chaos));
        assert_eq!(Scroll::from_item_id(4_031_066), Some(Scroll::CleanSlate));
        // The control: a neighbouring id is not a scroll.
        assert_eq!(Scroll::from_item_id(4_031_067), None);
        assert_eq!(Scroll::from_item_id(2_000_000), None);
    }

    /// Innocence reverts BOTH halves and needs no free slot - the one that has no refusal.
    #[test]
    fn innocence_restores_base_stats_and_every_slot_from_any_state() {
        let b = base(7, 5);
        // The worst case: no slots left, three failures banked, stats driven far off base.
        let s = state(0, 3, 99);
        let out = apply(Scroll::Innocence, &b, &s, Chance::Rolled, 0).unwrap();
        assert!(out.succeeded, "Innocence is 100% by specification");
        assert_eq!(out.after.remaining, 7, "all enhancement slots returned");
        assert_eq!(out.after.stats.inc_pad, 5, "back to the template's stats");
        assert_eq!(out.after.failed_slots, 0, "no failures left to undo");
        assert!(!out.slot_spent);
    }

    /// **A failed Chaos still eats the slot.** The owner, asked directly.
    #[test]
    fn a_failed_chaos_still_eats_a_slot_and_banks_a_failure() {
        let b = base(7, 5);
        let s = state(7, 0, 5);
        // roll % 100 == 99 -> above 60, a failure.
        let out = apply(Scroll::Chaos, &b, &s, Chance::Rolled, 99).unwrap();
        assert!(!out.succeeded);
        assert!(out.slot_spent, "the slot is eaten on failure too - that is the rule");
        assert_eq!(out.after.remaining, 6);
        assert_eq!(out.after.failed_slots, 1, "and it becomes a slot Clean Slate can return");
        assert_eq!(out.after.stats.inc_pad, 5, "a failure changes no stat");
    }

    /// A successful Chaos also eats the slot, and moves exactly one stat within +-5.
    #[test]
    fn a_successful_chaos_eats_the_slot_and_moves_one_base_stat() {
        let b = base(7, 20);
        for roll in [0u64, 1, 59, 100, 1_000, 12_345, 999_999] {
            let out = apply(Scroll::Chaos, &b, &state(7, 0, 20), Chance::Guaranteed, roll).unwrap();
            assert!(out.succeeded);
            assert!(out.slot_spent);
            assert_eq!(out.after.remaining, 6);
            assert_eq!(out.after.failed_slots, 0, "a success banks nothing for Clean Slate");
            let (name, delta) = out.stat_change.expect("a base stat exists, so one moved");
            assert_eq!(name, "Weapon Attack", "the only non-zero base stat");
            assert!((-CHAOS_MAX_SWING..=CHAOS_MAX_SWING).contains(&delta), "delta {delta}");
            assert_eq!(i32::from(out.after.stats.inc_pad), 20 + delta);
        }
    }

    /// **Only stats the base item has.** A stat the template leaves at zero must never move.
    #[test]
    fn chaos_never_touches_a_stat_the_template_does_not_have() {
        let b = base(7, 20); // weapon attack only
        for roll in 0..2_000u64 {
            let out = apply(Scroll::Chaos, &b, &state(7, 0, 20), Chance::Guaranteed, roll).unwrap();
            let a = out.after.stats;
            assert_eq!(a.inc_str, 0, "roll {roll}");
            assert_eq!(a.inc_mhp, 0, "roll {roll}");
            assert_eq!(a.inc_mad, 0, "roll {roll}");
        }
    }

    /// A stat driven down cannot go negative - `EquipStatSet` is `u16` and would wrap.
    #[test]
    fn a_stat_rolled_below_zero_clamps_instead_of_wrapping() {
        let b = base(7, 3);
        for roll in 0..2_000u64 {
            let out = apply(Scroll::Chaos, &b, &state(7, 0, 1), Chance::Guaranteed, roll).unwrap();
            // 1 - 5 would be -4; as a u16 that is 65532, which would be a legendary weapon.
            assert!(out.after.stats.inc_pad <= 6, "roll {roll} gave {}", out.after.stats.inc_pad);
        }
    }

    #[test]
    fn chaos_is_refused_with_no_slots_and_the_state_is_untouched() {
        let b = base(7, 5);
        let s = state(0, 2, 5);
        assert_eq!(apply(Scroll::Chaos, &b, &s, Chance::Guaranteed, 0), Err(Refusal::NoSlotsLeft));
    }

    /// **Clean Slate counts; it does not remember an order.** Two failures, two restores.
    #[test]
    fn two_failed_slots_allow_two_clean_slates_at_any_later_time() {
        let b = base(7, 5);
        let mut s = state(5, 2, 5); // two slots lost to failures earlier
        for expect_remaining in [6u8, 7] {
            let out = apply(Scroll::CleanSlate, &b, &s, Chance::Guaranteed, 0).unwrap();
            assert!(out.succeeded);
            assert_eq!(out.after.remaining, expect_remaining);
            s = out.after;
        }
        assert_eq!(s.failed_slots, 0);
        // A third is refused: the count, not the history, is what gates it.
        assert_eq!(
            apply(Scroll::CleanSlate, &b, &s, Chance::Guaranteed, 0),
            Err(Refusal::NothingToRestore)
        );
    }

    /// A successful scroll leaves nothing for Clean Slate - the owner's original sentence, which
    /// falls out of the count rather than needing a rule of its own.
    #[test]
    fn clean_slate_is_refused_after_a_successful_chaos() {
        let b = base(7, 20);
        let after_success =
            apply(Scroll::Chaos, &b, &state(7, 0, 20), Chance::Guaranteed, 0).unwrap().after;
        assert_eq!(after_success.failed_slots, 0);
        assert_eq!(
            apply(Scroll::CleanSlate, &b, &after_success, Chance::Guaranteed, 0),
            Err(Refusal::NothingToRestore)
        );
    }

    /// A failed Clean Slate gives nothing back and takes nothing away.
    #[test]
    fn a_failed_clean_slate_costs_only_the_scroll() {
        let b = base(7, 5);
        let s = state(5, 2, 5);
        let out = apply(Scroll::CleanSlate, &b, &s, Chance::Rolled, 99).unwrap();
        assert!(!out.succeeded);
        assert!(!out.slot_spent);
        assert_eq!(out.after, s, "nothing moved at all");
    }

    /// Clean Slate may not push an item past its own template.
    #[test]
    fn clean_slate_will_not_exceed_the_templates_tuc() {
        let b = base(7, 5);
        assert_eq!(
            apply(Scroll::CleanSlate, &b, &state(7, 1, 5), Chance::Guaranteed, 0),
            Err(Refusal::AlreadyAtBase)
        );
    }

    /// **The daily pass is the whole difference between the two chances**, and this pins the
    /// 60 so a change to it cannot pass silently.
    #[test]
    fn the_guaranteed_pass_always_succeeds_and_the_rolled_one_is_sixty_percent() {
        let b = base(7, 20);
        let s = state(7, 0, 20);
        for roll in 0..100u64 {
            assert!(apply(Scroll::Chaos, &b, &s, Chance::Guaranteed, roll).unwrap().succeeded);
        }
        let wins = (0..100u64)
            .filter(|&r| apply(Scroll::Chaos, &b, &s, Chance::Rolled, r).unwrap().succeeded)
            .count();
        assert_eq!(wins, 60, "ROLLED_SUCCESS_PCT is 60 and the residue is roll % 100");
    }

    /// An equip with no base stats at all is a success that changed nothing - and must not be
    /// reported as a failure, because the player still paid a slot for it.
    #[test]
    fn a_decorative_equip_succeeds_and_moves_nothing() {
        let b = EquipBase { tuc: 5, stats: EquipStatSet::default() };
        let out = apply(Scroll::Chaos, &b, &state(5, 0, 0), Chance::Guaranteed, 7).unwrap();
        assert!(out.succeeded);
        assert!(out.slot_spent);
        assert_eq!(out.stat_change, None);
    }

    /// Every refusal says something, and no two say the same thing - a screenshot has to be
    /// able to name which one fired.
    #[test]
    fn the_refusal_lines_are_distinct_and_non_empty() {
        let all = [Refusal::NoSlotsLeft, Refusal::NothingToRestore, Refusal::AlreadyAtBase];
        for (i, a) in all.iter().enumerate() {
            assert!(!a.line().is_empty());
            for b in &all[i + 1..] {
                assert_ne!(a.line(), b.line());
            }
        }
    }
}
