//! The two scrolls this client does not have, and what each does to an equip.
//!
//! The owner, 2026-09-09, specified items to be repurposed as scrolls MapleStory has and this
//! client's data does not. `research/scroll-command-2026-09-09.md` records the spec verbatim;
//! this module is the rules half of it and **nothing here talks to a client, a store or a
//! packet**. That is deliberate: every rule below is a pure function of the equip's current
//! state, its base template and one roll, so all of it is testable without a database.
//!
//! # The two
//!
//! | item | what it does |
//! |---|---|
//! | `4031065` Scroll of Secrets | **one item, three modes**, chosen in the dialogue - see [`SecretsMode`] |
//! | `4031066` Treasure Scroll | guarantees a **real** scroll the player is carrying; see [`apply_treasure`] |
//!
//! # `4001009` Event Trophy was dropped, and the reason is measured
//!
//! It was the Innocence Scroll until 2026-09-09, when the owner said *"Do not use event trophy
//! since it does not stack."* The client's own data agrees and says why: `itemdata.txt` gives
//! `4001009` **`slotMax = 0`**, against `1` for the other two. Innocence did not disappear with
//! it - it became [`SecretsMode::Innocence`], one of the three things a Scroll of Secrets can
//! be used as.
//!
//! **An honest note that outlives this change:** `4031065` and `4031066` carry `slotMax = 1`,
//! which is also not a stack. `crate::shops::max_stack` overrides both to
//! [`STACK_LIMIT`] server-side, and whether this client honours a server quantity above an
//! item's own `slotMax` is **[I]** - unmeasured, and the same question that took Event Trophy
//! out. 161 of the 359 Etc items already carry `slotMax = 200` if a swap is ever wanted.
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

/// `4031065` Scroll of Secrets - a Chaos, an Innocence or a Clean Slate, the player's choice.
pub const SCROLL_OF_SECRETS: u32 = 4_031_065;

/// `4031066` Treasure Scroll - guarantees a real scroll the player is carrying.
pub const TREASURE_SCROLL: u32 = 4_031_066;

/// Both repurposed ids, for the callers that need to treat them as a set - the drop-table
/// exemption and the stack override. **A list, not a range**: they are neighbours today and a
/// third could be anywhere.
pub const REPURPOSED: [u32; 2] = [SCROLL_OF_SECRETS, TREASURE_SCROLL];

/// **The modern client's own scrolls, backported** (the owner, 2026-10-01: *"Can we potentially
/// back port Chaos Scrolls, Clean Slate Scrolls, Innocence Scrolls from the modern client and
/// make it work like how we have in !scroll?"* - and *"We can use the success rates shown on
/// the modern items."*). Real Use-tab items, dragged onto an equip like any scroll (`0x0125`),
/// each one a [`SecretsMode`] at the rate its own tooltip states.
///
/// | id | modern name | mode | rate |
/// |---|---|---|---|
/// | 2049000..2049003 | Pure Clean Slate Scroll 1/3/5/20% | Clean Slate | 1, 3, 5, 20 |
/// | 2049100 | Chaos Scroll 60% | Chaos | 60 |
/// | **2049190** | Innocence Scroll 70% (modern **2049600**) | Innocence | 70 |
///
/// **Innocence is renumbered** (`tools/backport_install.py`'s `SCROLL_RENAMES`): this client's
/// applicability predicate `FUN_1404174b0` lets `2049000..2049199` onto any non-pet equip
/// (`0x14041752c..0x14041754d`) [L], but 2049600 falls to the "scroll category == equip
/// category" rule, which no equip meets - the drag would be refused before anything is sent.
pub const BACKPORTED: [(u32, SecretsMode, u32); 6] = [
    (2_049_000, SecretsMode::CleanSlate, 1),
    (2_049_001, SecretsMode::CleanSlate, 3),
    (2_049_002, SecretsMode::CleanSlate, 5),
    (2_049_003, SecretsMode::CleanSlate, 20),
    (2_049_100, SecretsMode::Chaos, 60),
    (2_049_190, SecretsMode::Innocence, 70),
];

/// The mode and success rate of a [`BACKPORTED`] scroll.
pub fn backported(item_id: u32) -> Option<(SecretsMode, u32)> {
    BACKPORTED.iter().find(|(id, _, _)| *id == item_id).map(|&(_, mode, pct)| (mode, pct))
}

/// `2530000` **Lucky Day Scroll**, backported. Dragged onto an equip (`0x0126`) it sets
/// [`net::opcode::ATTRIBUTE_LUCKY_DAY`]; the next scroll on that item then succeeds whatever
/// its rate, and spends the bit. The owner, 2026-10-01: *"the lucky day scroll will
/// automatically make the next scroll used on that item to automatically succeed without
/// respecting its success percentage."*
pub const LUCKY_DAY: u32 = 2_530_000;

/// The success chance of Chaos and Clean Slate when the daily free pass is spent, in percent.
pub const ROLLED_SUCCESS_PCT: u32 = 60;

/// **The global drop table: the four backported scrolls**, as `(item, chance per million)`.
///
/// The owner, 2026-10-01: *"Remove Scroll of Secrets and Treasure Scroll from global drop
/// tables. Here are the new global scrolls: Innocence scroll 70%: 1 in 1000 ... Chaos Scroll
/// 60%: 1 in 500 ... Pure Clean Slate Scroll 20%: 1 in 500 ... Lucky Day Scroll 100%: 1 in
/// 1000."* The two repurposed scrolls still work from a bag; they no longer drop.
///
/// The number that matters lives in `data/drops.txt`, not here - this is the copy the `!scroll`
/// dialogue renders, and a copy is a claim. `droptables`'
/// `the_global_table_is_the_four_backported_scrolls` asserts the file's global rows are exactly
/// these, so the two cannot drift. The global table is not scaled by the server's drop rate.
pub const GLOBAL_SCROLLS: [(u32, u32); 4] = [
    (2_049_190, 1_000), // Innocence Scroll 70% - 1 in 1000
    (2_049_100, 2_000), // Chaos Scroll 60% - 1 in 500
    (2_049_003, 2_000), // Pure Clean Slate Scroll 20% - 1 in 500
    (LUCKY_DAY, 1_000), // Lucky Day Scroll - 1 in 1000
];

/// **How many of one scroll fit in a bag slot.** The owner, 2026-09-09: *"can we make all of these
/// items stackable up to a 100 please?"*
///
/// Applied in `crate::shops::max_stack`, which is the one place that rule lives, and applied
/// as an OVERRIDE: `4031065` and `4031066` carry `info/slotMax = 1` in the client's own data,
/// which describes the quest props they are there rather than the scrolls they are here.
pub const STACK_LIMIT: u16 = 100;

/// The largest amount Chaos moves a stat by, in either direction.
///
/// The swing is `-5..=-1` or `1..=5`, **never zero**. The owner, 2026-10-01, of the Chaos Scroll:
/// *"the result will change the item either positively or negatively, it cannot roll 0."* It
/// was `-5..=5` inclusive of zero until then, from the 2026-09-09 wording *"up or down 0 to 5
/// points"*. A stat already at 0 cannot go down, so a downward roll there goes up instead - the
/// alternative is a successful Chaos that changed nothing, which is the outcome the owner ruled out.
pub const CHAOS_MAX_SWING: i32 = 5;

/// Which of the two repurposed items the player used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scroll {
    /// `4031065`, whose behaviour the player then chooses. See [`SecretsMode`].
    Secrets,
    /// `4031066`, which guarantees a real scroll the player is carrying.
    Treasure,
}

impl Scroll {
    /// `None` for anything that is not one of the two.
    pub fn from_item_id(item_id: u32) -> Option<Self> {
        match item_id {
            SCROLL_OF_SECRETS => Some(Scroll::Secrets),
            TREASURE_SCROLL => Some(Scroll::Treasure),
            _ => None,
        }
    }

    pub fn item_id(self) -> u32 {
        match self {
            Scroll::Secrets => SCROLL_OF_SECRETS,
            Scroll::Treasure => TREASURE_SCROLL,
        }
    }

    /// The name the dialogue uses.
    ///
    /// **These are the client's own names, not invented ones**, and that was checked rather
    /// than assumed on 2026-09-09: `String.wz/Etc.img` gives `4031065` *Scroll of Secrets* and
    /// `4031066` *Treasure Scroll*, character for character. They are repeated here because a
    /// dialogue string cannot read the WZ, and they must keep matching - the menu draws each
    /// row's `#i<itemId>#` icon beside the name, and an icon that disagrees with the text
    /// beside it is worse than no icon.
    ///
    /// What the client does **not** have is a description that fits: its `desc` for `4031066`
    /// is *"A map that shows where the jewels are hidden away."* `crate::scrollnpc::describe`
    /// supplies ours.
    pub fn name(self) -> &'static str {
        match self {
            Scroll::Secrets => "Scroll of Secrets",
            Scroll::Treasure => "Treasure Scroll",
        }
    }
}

/// The three things a Scroll of Secrets can be used as, chosen in the dialogue.
///
/// The owner's patch notes, 2026-09-09, are the specification and each line is one arm below:
///
/// * *"Chaos Scroll (100% first time of the day, otherwise 60%), randomly increases or
///   decreases one of the item's base stat by up to 5 points. Reduces enhancement slot by 1."*
/// * *"Innocence Scroll (100%), returns the item back to its unmodified base state, no random
///   base stats will be kept. Returns all enhancement slots. Does not require an enhancement
///   slot to use."*
/// * *"Clean Slate Scroll (100% first time of the day, otherwise 60%), returns a failed
///   enhancement slot of a previous scroll you have used upon the item. You cannot recover an
///   enhancement slot if the original scroll succeeded."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecretsMode {
    Chaos,
    Innocence,
    CleanSlate,
}

impl SecretsMode {
    /// Every mode, in the order the menu lists them.
    pub const ALL: [SecretsMode; 3] =
        [SecretsMode::Chaos, SecretsMode::Innocence, SecretsMode::CleanSlate];

    /// The name of the *real* MapleStory scroll this mode imitates - which is what the patch
    /// notes call it, and therefore what the player will call it.
    pub fn name(self) -> &'static str {
        match self {
            SecretsMode::Chaos => "Chaos Scroll",
            SecretsMode::Innocence => "Innocence Scroll",
            SecretsMode::CleanSlate => "Clean Slate Scroll",
        }
    }

    /// Stable across restarts and readable in a log, because it is half of the daily-perk key
    /// and a key that changes shape strands every claim written under the old one.
    pub fn key(self) -> &'static str {
        match self {
            SecretsMode::Chaos => "chaos",
            SecretsMode::Innocence => "innocence",
            SecretsMode::CleanSlate => "cleanslate",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        SecretsMode::ALL.into_iter().find(|m| m.key() == key)
    }

    /// Does this mode roll at all?
    ///
    /// **Innocence never does** - the patch note gives it a flat `(100%)` with no daily
    /// qualifier, where Chaos and Clean Slate both read *"100% first time of the day,
    /// otherwise 60%"*. So Innocence must not consume a daily pass either: spending one on a
    /// mode that could not have used it would silently cost the player their free Chaos.
    pub fn always_succeeds(self) -> bool {
        matches!(self, SecretsMode::Innocence)
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
    /// A [`BACKPORTED`] scroll at its own rate - **Innocence included**, which only ever
    /// succeeds outright under the other two.
    Percent(u32),
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
///
/// # Every name here is the client's own tooltip label
///
/// And that is a correction, not a style note. This table used to call `inc_pad` **"Weapon
/// Attack"** and `inc_wat` **"WAT"**, which is the pair swapped - the same swap that made
/// `equip_base` copy a weapon's attack into bit 8 and put `Attack Power: +200 (0 +200)` on
/// The owner's screen. `net::opcode::EquipStatSet` carries each field's string id from the client's
/// own table: bit 8 `inc_pad` is `Attack Power: +%d` (`0x0380`) and bit 16 `inc_wat` is
/// `Weapon Attack: +%d` (`0x037F`).
///
/// It matters beyond tidiness because these strings are **read out to the player** - a Chaos
/// that says *"gained 3 WAT"* while the tooltip line beside it says *Weapon Attack* is a
/// message the player cannot reconcile with what they are looking at. `Avoidability`,
/// `Critical` and `Craft` were wrong the same way and are now `Evasion`, `Critical Rate` and
/// `Critical Damage`.
#[allow(clippy::type_complexity)]
const STATS: &[(&str, fn(&EquipStatSet) -> u16, fn(&mut EquipStatSet, u16))] = &[
    ("STR", |s| s.inc_str, |s, v| s.inc_str = v),
    ("DEX", |s| s.inc_dex, |s, v| s.inc_dex = v),
    ("INT", |s| s.inc_int, |s, v| s.inc_int = v),
    ("LUK", |s| s.inc_luk, |s, v| s.inc_luk = v),
    ("MaxHP", |s| s.inc_mhp, |s, v| s.inc_mhp = v),
    ("MaxMP", |s| s.inc_mmp, |s, v| s.inc_mmp = v),
    ("Speed", |s| s.inc_speed, |s, v| s.inc_speed = v),
    ("Jump", |s| s.inc_jump, |s, v| s.inc_jump = v),
    ("Attack Power", |s| s.inc_pad, |s, v| s.inc_pad = v),
    ("Magic Attack", |s| s.inc_mad, |s, v| s.inc_mad = v),
    ("Weapon Def.", |s| s.inc_pdd, |s, v| s.inc_pdd = v),
    ("Magic Def.", |s| s.inc_mdd, |s, v| s.inc_mdd = v),
    ("Accuracy", |s| s.inc_acc, |s, v| s.inc_acc = v),
    ("Evasion", |s| s.inc_eva, |s, v| s.inc_eva = v),
    ("Critical Rate", |s| s.inc_crt, |s, v| s.inc_crt = v),
    ("Critical Damage", |s| s.inc_crd, |s, v| s.inc_crd = v),
    ("Weapon Attack", |s| s.inc_wat, |s, v| s.inc_wat = v),
];

/// A refusal, with the line the player is shown.
///
/// **Every refusal carries its own wording.** `CLAUDE.md`: a refusal reported to nobody looks
/// exactly like a frozen UI, and this project has spent runs chasing that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Chaos and the Treasure Scroll both need a slot to take.
    NoSlotsLeft,
    /// Clean Slate needs a failure to undo.
    NothingToRestore,
    /// Clean Slate would push the item past its own template's `tuc`.
    AlreadyAtBase,
    /// The Treasure Scroll found no real scroll in the bag that fits the chosen equip.
    NoScrollFits,
}

impl Refusal {
    pub fn line(self) -> &'static str {
        match self {
            Refusal::NoSlotsLeft => "That item has no enhancement slots left.",
            Refusal::NothingToRestore => "That item has no failed enhancement slots to restore.",
            Refusal::AlreadyAtBase => "That item already has all of its enhancement slots.",
            Refusal::NoScrollFits => {
                "You are not carrying a scroll that fits that item. Bring me one made for it."
            }
        }
    }
}

/// What happened, and the state to write back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Applied {
    pub after: EquipState,
    /// Did the roll succeed? Innocence is always `true`.
    pub succeeded: bool,
    /// Did this use consume an enhancement slot? True for Chaos on **both** arms, and for
    /// every Treasure Scroll - the owner: *"while subtracting an item enhancement"*.
    pub slot_spent: bool,
    /// Which stats moved and by how much, for the line the player reads.
    ///
    /// **A list rather than one pair**, because the Treasure Scroll applies a real scroll and
    /// a real scroll can grant several stats at once - `2040800` grants Attack Power *and*
    /// Accuracy. Chaos pushes at most one; Clean Slate and Innocence push none.
    pub changes: Vec<(&'static str, i32)>,
    /// **The item is gone.** Only a real scroll with a non-zero `cursed` can do this, and only
    /// on the failure arm.
    ///
    /// Kept separate from `succeeded` because they are different questions: a destroy IS a
    /// failure, and a caller that treats `!succeeded` as "the item survived and lost a slot"
    /// would leave a destroyed item in the bag. Every construction below sets it explicitly.
    pub destroyed: bool,
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
    mode: SecretsMode,
    base: &EquipBase,
    state: &EquipState,
    chance: Chance,
    roll: u64,
) -> Result<Applied, Refusal> {
    match mode {
        // Reverts everything, and needs no free slot - the owner: "Does not require an enhancement
        // slot to use." So it has no refusal at all, and it clears the failed count because
        // there are no longer any failures to undo.
        //
        // "no random base stats will be kept" is `stats: base.stats` and nothing else: the
        // template's own set, not the current set with the rolls unwound, so an item that has
        // been scrolled a dozen times still lands exactly on the template.
        // A backported Innocence (70%) that misses changes nothing: no slot, no stat.
        SecretsMode::Innocence if !innocence_succeeds(chance, roll) => Ok(Applied {
            after: *state,
            succeeded: false,
            slot_spent: false,
            changes: Vec::new(),
            destroyed: false,
        }),
        SecretsMode::Innocence => Ok(Applied {
            after: EquipState { remaining: base.tuc, failed_slots: 0, stats: base.stats },
            succeeded: true,
            slot_spent: false,
            changes: Vec::new(),
            // Nothing a Scroll of Secrets does can destroy an item, on any of its three modes.
            destroyed: false,
        }),

        SecretsMode::Chaos => {
            if state.remaining == 0 {
                return Err(Refusal::NoSlotsLeft);
            }
            let succeeded = succeeds(chance, roll);
            let mut after = *state;
            // **Both arms.** The owner: "failed chaos scroll will eat a slot."
            after.remaining -= 1;
            let mut changes = Vec::new();
            if succeeded {
                if let Some(change) = roll_one_stat(base, &mut after.stats, roll) {
                    changes.push(change);
                }
            } else {
                // The slot it just ate is now a slot Clean Slate can give back.
                after.failed_slots = after.failed_slots.saturating_add(1);
            }
            Ok(Applied { after, succeeded, slot_spent: true, changes, destroyed: false })
        }

        SecretsMode::CleanSlate => {
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
            Ok(Applied { after, succeeded, slot_spent: false, changes: Vec::new(), destroyed: false })
        }
    }
}

/// The Treasure Scroll: apply a **real** scroll's increments with guaranteed success.
///
/// The owner, 2026-09-09: *"Use this Scroll to automatically succeed the next scroll of your
/// choosing via the GUI options and apply those stat increases to the item immediately while
/// subtracting an item enhancement."* Asked which scrolls the menu offers, they chose *only a
/// real scroll you are carrying* - so this takes the increments of a scroll the caller has
/// already found in the bag and confirmed fits the equip.
///
/// # Three things it deliberately does not consult
///
/// * **`ScrollTemplate::success`.** The guarantee is the whole item. A 10% scroll and a 100%
///   scroll are the same here, which is exactly why the menu shows the normal rate: the
///   difference the Treasure Scroll makes is only visible if the player can see what they
///   were spared.
/// * **`ScrollTemplate::cursed`.** A guaranteed success never reaches the failure arm, so
///   nothing can be destroyed. That is a consequence of the rule and not a special case -
///   there is no destroy path in this function to disable.
/// * **[`Chance`] and the daily pass.** The Treasure Scroll is not gated on a day; its
///   scarcity is the 0.01% drop. Nothing here claims a pass, so a Treasure Scroll cannot
///   silently cost the player their free Chaos.
///
/// # It still costs a slot, and therefore it can still be refused
///
/// A slot is subtracted, so an item with none is refused **before** anything is spent - the
/// caller has nothing to half-apply, and neither the Treasure Scroll nor the real scroll
/// leaves the bag.
///
/// Failed slots are untouched: this cannot fail, so it banks nothing for a Clean Slate, and it
/// does not repay one either. A slot spent here is simply spent.
pub fn apply_treasure(
    base: &EquipBase,
    state: &EquipState,
    increments: &EquipStatSet,
) -> Result<Applied, Refusal> {
    let _ = base;
    if state.remaining == 0 {
        return Err(Refusal::NoSlotsLeft);
    }
    let mut after = *state;
    after.remaining -= 1;
    let mut changes = Vec::new();
    for (name, get, set) in STATS {
        let granted = get(increments);
        if granted == 0 {
            continue;
        }
        let now = get(&after.stats);
        // Saturating: a stat is `u16` on the wire, and wrapping a weapon's attack to a small
        // number would read on screen as the scroll having *removed* the stat it granted.
        let next = now.saturating_add(granted);
        set(&mut after.stats, next);
        changes.push((*name, i32::from(next) - i32::from(now)));
    }
    // A guarantee never reaches a failure arm, so it never reaches a destroy either.
    Ok(Applied { after, succeeded: true, slot_spent: true, changes, destroyed: false })
}

/// **A real scroll, rolled** - the client's own scrolling window, `0x0125`.
///
/// The owner, 2026-09-09: *"Just tried scrolling the topwear, it did not work."* It did not: the
/// opcode was decoded that day and never handled. This is the rule behind it.
///
/// `success_pct` and `cursed_pct` come straight from `gm-handbook/scrolls.txt`, which is the
/// client's own `0204.img`. Across all 208 scrolls `success` takes exactly three values - 100,
/// 60 and 10 - and **156 of them have `cursed = 0`**, so most scrolls cannot destroy anything.
///
/// # Three outcomes, and the third is why [`Applied::destroyed`] exists
///
/// * **success** - the increments are added and one slot is spent.
/// * **failure** - one slot is spent and banked as a failed slot, so a Clean Slate can undo it
///   exactly as it can undo a failed Chaos. Nothing else changes.
/// * **destroyed** - a failure that additionally rolled under `cursed_pct`. The item is gone;
///   the caller must remove it rather than write `after` back.
///
/// A destroy is a *kind of* failure, not an alternative to one, which is why `succeeded` is
/// false on that arm too. A caller reading only `succeeded` would write the item back with a
/// slot missing and leave a destroyed item in the bag, so `destroyed` is checked first.
///
/// # The two rolls are independent residues
///
/// `roll % 100` decides success and `(roll / 100) % 100` decides the destroy. Deriving the
/// second from the first would make "failed" and "destroyed" the same event for some scrolls
/// and impossible for others.
pub fn apply_real(
    base: &EquipBase,
    state: &EquipState,
    success_pct: u16,
    cursed_pct: u16,
    increments: &EquipStatSet,
    roll: u64,
) -> Result<Applied, Refusal> {
    let _ = base;
    if state.remaining == 0 {
        return Err(Refusal::NoSlotsLeft);
    }
    let succeeded = (roll % 100) < u64::from(success_pct);
    let mut after = *state;
    // **Every arm spends the slot**, which is the real game's rule and the same one the owner gave
    // for Chaos: "failed chaos scroll will eat a slot".
    after.remaining -= 1;

    if !succeeded {
        after.failed_slots = after.failed_slots.saturating_add(1);
        let destroyed = cursed_pct > 0 && ((roll / 100) % 100) < u64::from(cursed_pct);
        return Ok(Applied {
            after,
            succeeded: false,
            slot_spent: true,
            changes: Vec::new(),
            destroyed,
        });
    }

    let mut changes = Vec::new();
    for (name, get, set) in STATS {
        let granted = get(increments);
        if granted == 0 {
            continue;
        }
        let now = get(&after.stats);
        // Saturating for the same reason `apply_treasure` saturates: a wrap would read on
        // screen as the scroll having removed the stat it granted.
        let next = now.saturating_add(granted);
        set(&mut after.stats, next);
        changes.push((*name, i32::from(next) - i32::from(now)));
    }
    Ok(Applied { after, succeeded: true, slot_spent: true, changes, destroyed: false })
}

fn succeeds(chance: Chance, roll: u64) -> bool {
    match chance {
        Chance::Guaranteed => true,
        Chance::Rolled => (roll % 100) < u64::from(ROLLED_SUCCESS_PCT),
        Chance::Percent(pct) => (roll % 100) < u64::from(pct),
    }
}

/// Innocence rolls only at a [`Chance::Percent`]; `!scroll`'s Innocence is a flat 100%.
fn innocence_succeeds(chance: Chance, roll: u64) -> bool {
    match chance {
        Chance::Percent(pct) => (roll % 100) < u64::from(pct),
        Chance::Guaranteed | Chance::Rolled => true,
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
    // Ten outcomes, -5..=-1 and 1..=5: never zero (`CHAOS_MAX_SWING`'s docs).
    let span = (CHAOS_MAX_SWING * 2) as u64;
    let r = ((roll / 100 / 64) % span) as i32;
    let mut delta = if r < CHAOS_MAX_SWING { r - CHAOS_MAX_SWING } else { r - CHAOS_MAX_SWING + 1 };
    let now = i32::from(get(stats));
    if now == 0 && delta < 0 {
        delta = -delta; // nothing to take away, so it goes up rather than moving nothing
    }
    // Clamped at zero: `EquipStatSet` is `u16`, so a negative stat is not representable, and
    // clamping is the only behaviour that does not silently wrap to 65535. A stat of 2 rolled
    // -5 lands on 0, a change of -2 - still a change.
    let next = (now + delta).max(0);
    set(stats, next as u16);
    Some((name, next - now))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base(tuc: u8, wat: u16) -> EquipBase {
        EquipBase { tuc, stats: EquipStatSet { inc_wat: wat, ..Default::default() } }
    }

    fn state(remaining: u8, failed: u8, wat: u16) -> EquipState {
        EquipState {
            remaining,
            failed_slots: failed,
            stats: EquipStatSet { inc_wat: wat, ..Default::default() },
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
    fn the_two_item_ids_map_and_nothing_else_does() {
        assert_eq!(Scroll::from_item_id(4_031_065), Some(Scroll::Secrets));
        assert_eq!(Scroll::from_item_id(4_031_066), Some(Scroll::Treasure));
        // The control: a neighbouring id is not a scroll, and neither is the Event Trophy
        // that used to be one - a stale mapping here would put a dead item back on the menu.
        assert_eq!(Scroll::from_item_id(4_031_067), None);
        assert_eq!(Scroll::from_item_id(4_001_009), None);
        assert_eq!(Scroll::from_item_id(2_000_000), None);
        assert_eq!(REPURPOSED.len(), 2);
        assert!(REPURPOSED.iter().all(|id| Scroll::from_item_id(*id).is_some()));
    }

    /// A mode's daily-perk key is stored, so it must survive a reordering of the enum. Keys
    /// built from a discriminant would silently re-map every claim ever written.
    #[test]
    fn every_mode_key_round_trips_and_they_are_distinct() {
        for mode in SecretsMode::ALL {
            assert_eq!(SecretsMode::from_key(mode.key()), Some(mode));
        }
        let mut keys: Vec<&str> = SecretsMode::ALL.iter().map(|m| m.key()).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), 3);
        assert_eq!(SecretsMode::from_key("nonsense"), None);
        assert_eq!(SecretsMode::from_key(""), None);
    }

    /// Innocence reverts BOTH halves and needs no free slot - the one that has no refusal.
    #[test]
    fn innocence_restores_base_stats_and_every_slot_from_any_state() {
        let b = base(7, 5);
        // The worst case: no slots left, three failures banked, stats driven far off base.
        let s = state(0, 3, 99);
        let out = apply(SecretsMode::Innocence, &b, &s, Chance::Rolled, 0).unwrap();
        assert!(out.succeeded, "Innocence is 100% by specification");
        assert_eq!(out.after.remaining, 7, "all enhancement slots returned");
        assert_eq!(out.after.stats.inc_wat, 5, "back to the template's stats");
        assert_eq!(out.after.failed_slots, 0, "no failures left to undo");
        assert!(!out.slot_spent);
    }

    /// **A failed Chaos still eats the slot.** The owner, asked directly.
    #[test]
    fn a_failed_chaos_still_eats_a_slot_and_banks_a_failure() {
        let b = base(7, 5);
        let s = state(7, 0, 5);
        // roll % 100 == 99 -> above 60, a failure.
        let out = apply(SecretsMode::Chaos, &b, &s, Chance::Rolled, 99).unwrap();
        assert!(!out.succeeded);
        assert!(out.slot_spent, "the slot is eaten on failure too - that is the rule");
        assert_eq!(out.after.remaining, 6);
        assert_eq!(out.after.failed_slots, 1, "and it becomes a slot Clean Slate can return");
        assert_eq!(out.after.stats.inc_wat, 5, "a failure changes no stat");
    }

    /// A successful Chaos also eats the slot, and moves exactly one stat within +-5.
    #[test]
    fn a_successful_chaos_eats_the_slot_and_moves_one_base_stat() {
        let b = base(7, 20);
        for roll in [0u64, 1, 59, 100, 1_000, 12_345, 999_999] {
            let out = apply(SecretsMode::Chaos, &b, &state(7, 0, 20), Chance::Guaranteed, roll).unwrap();
            assert!(out.succeeded);
            assert!(out.slot_spent);
            assert_eq!(out.after.remaining, 6);
            assert_eq!(out.after.failed_slots, 0, "a success banks nothing for Clean Slate");
            let (name, delta) = out.changes.first().copied().expect("a base stat exists, so one moved");
            assert_eq!(name, "Weapon Attack", "the only non-zero base stat");
            assert!((-CHAOS_MAX_SWING..=CHAOS_MAX_SWING).contains(&delta), "delta {delta}");
            assert_eq!(i32::from(out.after.stats.inc_wat), 20 + delta);
        }
    }

    /// **Chaos never rolls 0** - the owner, 2026-10-01: *"the result will change the item either
    /// positively or negatively, it cannot roll 0."* Over 20 000 rolls every one of the ten
    /// outcomes -5..=-1, 1..=5 appears and 0 never does; and on a stat Chaos already drove to
    /// 0 it still moves - upward - rather than succeeding at nothing.
    #[test]
    fn chaos_never_rolls_zero_and_reaches_every_other_swing() {
        let b = base(7, 20);
        let mut seen = std::collections::BTreeSet::new();
        // Spread like the session's random u64s: the swing reads `roll / 6400`, which 0..20 000
        // would barely move.
        for i in 0..20_000u64 {
            let roll = i.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let out = apply(SecretsMode::Chaos, &b, &state(7, 0, 20), Chance::Guaranteed, roll).unwrap();
            let (_, delta) = out.changes[0];
            assert_ne!(delta, 0, "roll {roll}");
            seen.insert(delta);
        }
        assert_eq!(seen.into_iter().collect::<Vec<_>>(), vec![-5, -4, -3, -2, -1, 1, 2, 3, 4, 5]);
        for i in 0..2_000u64 {
            let roll = i.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let out = apply(SecretsMode::Chaos, &b, &state(7, 0, 0), Chance::Guaranteed, roll).unwrap();
            let (_, delta) = out.changes[0];
            assert!(delta > 0, "roll {roll}: a stat at 0 must go up, got {delta}");
        }
    }

    /// **Only stats the base item has.** A stat the template leaves at zero must never move.
    #[test]
    fn chaos_never_touches_a_stat_the_template_does_not_have() {
        let b = base(7, 20); // weapon attack only
        for roll in 0..2_000u64 {
            let out = apply(SecretsMode::Chaos, &b, &state(7, 0, 20), Chance::Guaranteed, roll).unwrap();
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
            let out = apply(SecretsMode::Chaos, &b, &state(7, 0, 1), Chance::Guaranteed, roll).unwrap();
            // 1 - 5 would be -4; as a u16 that is 65532, which would be a legendary weapon.
            assert!(out.after.stats.inc_wat <= 6, "roll {roll} gave {}", out.after.stats.inc_wat);
        }
    }

    #[test]
    fn chaos_is_refused_with_no_slots_and_the_state_is_untouched() {
        let b = base(7, 5);
        let s = state(0, 2, 5);
        assert_eq!(apply(SecretsMode::Chaos, &b, &s, Chance::Guaranteed, 0), Err(Refusal::NoSlotsLeft));
    }

    /// **Clean Slate counts; it does not remember an order.** Two failures, two restores.
    #[test]
    fn two_failed_slots_allow_two_clean_slates_at_any_later_time() {
        let b = base(7, 5);
        let mut s = state(5, 2, 5); // two slots lost to failures earlier
        for expect_remaining in [6u8, 7] {
            let out = apply(SecretsMode::CleanSlate, &b, &s, Chance::Guaranteed, 0).unwrap();
            assert!(out.succeeded);
            assert_eq!(out.after.remaining, expect_remaining);
            s = out.after;
        }
        assert_eq!(s.failed_slots, 0);
        // A third is refused: the count, not the history, is what gates it.
        assert_eq!(
            apply(SecretsMode::CleanSlate, &b, &s, Chance::Guaranteed, 0),
            Err(Refusal::NothingToRestore)
        );
    }

    /// A successful scroll leaves nothing for Clean Slate - the owner's original sentence, which
    /// falls out of the count rather than needing a rule of its own.
    #[test]
    fn clean_slate_is_refused_after_a_successful_chaos() {
        let b = base(7, 20);
        let after_success =
            apply(SecretsMode::Chaos, &b, &state(7, 0, 20), Chance::Guaranteed, 0).unwrap().after;
        assert_eq!(after_success.failed_slots, 0);
        assert_eq!(
            apply(SecretsMode::CleanSlate, &b, &after_success, Chance::Guaranteed, 0),
            Err(Refusal::NothingToRestore)
        );
    }

    /// A failed Clean Slate gives nothing back and takes nothing away.
    #[test]
    fn a_failed_clean_slate_costs_only_the_scroll() {
        let b = base(7, 5);
        let s = state(5, 2, 5);
        let out = apply(SecretsMode::CleanSlate, &b, &s, Chance::Rolled, 99).unwrap();
        assert!(!out.succeeded);
        assert!(!out.slot_spent);
        assert_eq!(out.after, s, "nothing moved at all");
    }

    /// Clean Slate may not push an item past its own template.
    #[test]
    fn clean_slate_will_not_exceed_the_templates_tuc() {
        let b = base(7, 5);
        assert_eq!(
            apply(SecretsMode::CleanSlate, &b, &state(7, 1, 5), Chance::Guaranteed, 0),
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
            assert!(apply(SecretsMode::Chaos, &b, &s, Chance::Guaranteed, roll).unwrap().succeeded);
        }
        let wins = (0..100u64)
            .filter(|&r| apply(SecretsMode::Chaos, &b, &s, Chance::Rolled, r).unwrap().succeeded)
            .count();
        assert_eq!(wins, 60, "ROLLED_SUCCESS_PCT is 60 and the residue is roll % 100");
    }

    /// An equip with no base stats at all is a success that changed nothing - and must not be
    /// reported as a failure, because the player still paid a slot for it.
    #[test]
    fn a_decorative_equip_succeeds_and_moves_nothing() {
        let b = EquipBase { tuc: 5, stats: EquipStatSet::default() };
        let out = apply(SecretsMode::Chaos, &b, &state(5, 0, 0), Chance::Guaranteed, 7).unwrap();
        assert!(out.succeeded);
        assert!(out.slot_spent);
        assert!(out.changes.is_empty());
    }

    /// **A real scroll succeeds, fails, or destroys - and the slot goes on every arm.**
    ///
    /// The owner: *"Just tried scrolling the topwear, it did not work."* `0x0125` was decoded and
    /// never handled. `success` and `cursed` are the client's own numbers.
    #[test]
    fn a_real_scroll_rolls_success_failure_and_destruction() {
        let b = base(7, 100);
        let grant = EquipStatSet { inc_wat: 5, ..Default::default() };

        // 100% never fails, whatever the roll.
        for roll in [0u64, 59, 99, 12_345] {
            let out = apply_real(&b, &state(7, 0, 100), 100, 0, &grant, roll).unwrap();
            assert!(out.succeeded, "roll {roll}");
            assert!(!out.destroyed);
            assert_eq!(out.after.stats.inc_wat, 105);
            assert_eq!(out.after.remaining, 6, "a slot goes even on a guaranteed scroll");
        }

        // 10% fails on a roll of 50. The slot still goes, and it is banked for a Clean Slate.
        let out = apply_real(&b, &state(7, 0, 100), 10, 0, &grant, 50).unwrap();
        assert!(!out.succeeded);
        assert!(!out.destroyed, "cursed is 0 on 156 of the 208 scrolls");
        assert_eq!(out.after.remaining, 6);
        assert_eq!(out.after.failed_slots, 1, "a Clean Slate can undo a real scroll's failure");
        assert_eq!(out.after.stats.inc_wat, 100, "a failure grants nothing");

        // The destroy arm: a failure AND a cursed roll under the percentage.
        let out = apply_real(&b, &state(7, 0, 100), 10, 50, &grant, 50).unwrap();
        assert!(!out.succeeded);
        assert!(out.destroyed, "roll/100 % 100 == 0, which is under 50");
        // The two residues are independent: same success roll, a cursed roll that misses.
        let out = apply_real(&b, &state(7, 0, 100), 10, 50, &grant, 50 + 100 * 90).unwrap();
        assert!(!out.succeeded, "the success residue is unchanged");
        assert!(!out.destroyed, "but 90 is not under 50");
    }

    /// A scroll needs a slot, and an item with none is refused before anything is spent.
    #[test]
    fn a_real_scroll_is_refused_when_no_slots_are_left() {
        let b = base(7, 100);
        let grant = EquipStatSet { inc_wat: 5, ..Default::default() };
        assert_eq!(
            apply_real(&b, &state(0, 3, 100), 100, 0, &grant, 0),
            Err(Refusal::NoSlotsLeft)
        );
    }

    /// **A destroy is a failure, not an alternative to one.** A caller that reads only
    /// `succeeded` would write the item back with a slot missing and leave a destroyed item in
    /// the bag, so both flags must agree about that.
    #[test]
    fn a_destroyed_item_also_reports_the_scroll_as_failed() {
        let b = base(7, 100);
        let grant = EquipStatSet { inc_wat: 5, ..Default::default() };
        let out = apply_real(&b, &state(7, 0, 100), 10, 100, &grant, 50).unwrap();
        assert!(out.destroyed);
        assert!(!out.succeeded, "destroyed implies failed");
        // And nothing a Scroll of Secrets does can ever set it.
        for mode in SecretsMode::ALL {
            for roll in [0u64, 99, 4_242] {
                if let Ok(a) = apply(mode, &b, &state(7, 1, 100), Chance::Rolled, roll) {
                    assert!(!a.destroyed, "{mode:?} must never destroy an item");
                }
            }
        }
        assert!(!apply_treasure(&b, &state(7, 0, 100), &grant).unwrap().destroyed);
    }

    /// **The Treasure Scroll succeeds and spends a slot.** The owner: *"automatically succeed the
    /// next scroll of your choosing … and apply those stat increases to the item immediately
    /// while subtracting an item enhancement."*
    #[test]
    fn a_treasure_scroll_always_succeeds_and_costs_one_slot() {
        let b = base(7, 100);
        let grant = EquipStatSet { inc_wat: 3, inc_acc: 1, ..Default::default() };
        let out = apply_treasure(&b, &state(7, 2, 100), &grant).unwrap();
        assert!(out.succeeded, "the guarantee is the whole item");
        assert!(out.slot_spent);
        assert_eq!(out.after.remaining, 6);
        assert_eq!(out.after.stats.inc_wat, 103, "the increment is added to what is there");
        assert_eq!(out.after.stats.inc_acc, 1);
        // **It banks no failure and repays none.** It cannot fail, so there is nothing for a
        // Clean Slate to pick up, and the two failures it found are left exactly as they were.
        assert_eq!(out.after.failed_slots, 2);
        // Every granted stat is reported, not just the first - the player spent two items.
        assert_eq!(out.changes.len(), 2, "{:?}", out.changes);
        assert!(out.changes.contains(&("Weapon Attack", 3)), "{:?}", out.changes);
        assert!(out.changes.contains(&("Accuracy", 1)), "{:?}", out.changes);
    }

    /// A slot is subtracted, so an item with none is refused - **before** anything is spent.
    #[test]
    fn a_treasure_scroll_is_refused_with_no_slots_left() {
        let b = base(7, 100);
        let grant = EquipStatSet { inc_wat: 3, ..Default::default() };
        assert_eq!(apply_treasure(&b, &state(0, 0, 100), &grant), Err(Refusal::NoSlotsLeft));
    }

    /// **A stat at the ceiling saturates rather than wrapping.** `EquipStatSet` is `u16`, and
    /// a wrap would take a weapon from 65 535 attack to 2 - which reads on screen as the
    /// scroll having removed the stat it was supposed to grant.
    #[test]
    fn a_treasure_scroll_saturates_instead_of_wrapping() {
        let b = base(7, 1);
        let mut s = state(7, 0, 1);
        s.stats.inc_wat = u16::MAX;
        let grant = EquipStatSet { inc_wat: 10, ..Default::default() };
        let out = apply_treasure(&b, &s, &grant).unwrap();
        assert_eq!(out.after.stats.inc_wat, u16::MAX);
        // And the reported delta is what actually happened, not what was promised.
        assert_eq!(out.changes, vec![("Weapon Attack", 0)]);
    }

    /// A scroll that grants nothing still succeeds and still costs the slot - and reports no
    /// change rather than a fictional one.
    #[test]
    fn a_treasure_scroll_granting_nothing_is_still_a_success_that_cost_a_slot() {
        let b = base(7, 100);
        let out = apply_treasure(&b, &state(7, 0, 100), &EquipStatSet::default()).unwrap();
        assert!(out.succeeded);
        assert!(out.slot_spent);
        assert_eq!(out.after.remaining, 6);
        assert!(out.changes.is_empty());
    }

    /// **The stat names are the client's own tooltip labels**, and this pair is the one that
    /// was crossed: bit 8 `inc_pad` is `Attack Power` (string `0x0380`) and bit 16 `inc_wat`
    /// is `Weapon Attack` (`0x037F`). The table used to have them the other way round, so a
    /// Chaos on a weapon announced a change to "WAT" while the tooltip line beside it said
    /// *Weapon Attack*.
    #[test]
    fn the_stat_labels_are_the_clients_own_and_the_attack_pair_is_not_crossed() {
        let named = |field: fn(&EquipStatSet) -> u16| {
            let mut probe = EquipStatSet::default();
            for (name, get, set) in STATS {
                set(&mut probe, 1);
                let hit = field(&probe) == 1;
                set(&mut probe, 0);
                if hit {
                    return *name;
                }
                let _ = get;
            }
            "not in the table"
        };
        assert_eq!(named(|s| s.inc_wat), "Weapon Attack");
        assert_eq!(named(|s| s.inc_pad), "Attack Power");
        assert_eq!(named(|s| s.inc_eva), "Evasion");
        assert_eq!(named(|s| s.inc_crd), "Critical Damage");
        // No two labels are the same, or a change report could name the wrong line.
        let mut names: Vec<&str> = STATS.iter().map(|(n, _, _)| *n).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), STATS.len());
    }

    /// Every refusal says something, and no two say the same thing - a screenshot has to be
    /// able to name which one fired.
    #[test]
    fn the_refusal_lines_are_distinct_and_non_empty() {
        let all = [
            Refusal::NoSlotsLeft,
            Refusal::NothingToRestore,
            Refusal::AlreadyAtBase,
            Refusal::NoScrollFits,
        ];
        for (i, a) in all.iter().enumerate() {
            assert!(!a.line().is_empty());
            for b in &all[i + 1..] {
                assert_ne!(a.line(), b.line());
            }
        }
    }

    /// **The backported rates are the modern tooltips'**, and Innocence rolls only at one.
    #[test]
    fn backported_scrolls_roll_at_their_own_rate() {
        assert_eq!(backported(2_049_100), Some((SecretsMode::Chaos, 60)));
        assert_eq!(backported(2_049_003), Some((SecretsMode::CleanSlate, 20)));
        assert_eq!(backported(2_049_190), Some((SecretsMode::Innocence, 70)));
        assert_eq!(backported(2_049_600), None, "the modern id is not used - the client refuses it");
        assert!(BACKPORTED.iter().all(|(id, _, _)| (2_049_000..2_049_200).contains(id)), "the range the client lets onto any equip");

        let base = EquipBase { tuc: 7, stats: EquipStatSet { inc_pdd: 10, ..Default::default() } };
        let worn = EquipState { remaining: 3, failed_slots: 2, stats: EquipStatSet { inc_pdd: 15, ..Default::default() } };
        // roll % 100 = 69 succeeds at 70; 70 does not.
        let hit = apply(SecretsMode::Innocence, &base, &worn, Chance::Percent(70), 69).unwrap();
        assert!(hit.succeeded);
        assert_eq!(hit.after.stats, base.stats);
        let miss = apply(SecretsMode::Innocence, &base, &worn, Chance::Percent(70), 70).unwrap();
        assert!(!miss.succeeded && !miss.slot_spent);
        assert_eq!(miss.after, worn, "a missed Innocence changes nothing");
        assert!(apply(SecretsMode::Innocence, &base, &worn, Chance::Rolled, 99).unwrap().succeeded, "!scroll's is still 100%");
        assert!(!apply(SecretsMode::CleanSlate, &base, &worn, Chance::Percent(1), 1).unwrap().succeeded, "1% misses at roll 1");
        assert!(apply(SecretsMode::CleanSlate, &base, &worn, Chance::Percent(1), 100).unwrap().succeeded, "and hits at roll 0");
    }
}
