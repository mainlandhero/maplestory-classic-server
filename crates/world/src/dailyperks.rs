//! **`!tool`'s daily favours**: 1000 Leaf Points, a level, and a return to Henesys. Level-up and
//! the return are once per **character** per UTC day; Leaf Points are once per **account** -
//! see [`Perk::scope`].
//!
//! **The AP/SP reset was removed on 2026-10-02.** The owner: *"Remove the daily perk for AP and SP
//! reset since that can be done via Cash Shop now"* - the AP and SP Reset scrolls
//! (`net::cashitem`). Its store key `resetapsp` is retired, not reused: old claim rows under it
//! are inert. Removing it moved Return to Henesys from `#L3#` to `#L2#`; the menu is rebuilt for
//! every `!tool`, so no open menu can carry the old number across the change.
//!
//! The owner, 2026-09-08: *"repurpose the 'MapleStory Administrator' NPC into a quality-of-life
//! NPC ... Gain 1000 Leaf Points / Level up (grant exactly the EXP needed to reach the next
//! level) / Reset AP & SP ... each option is usable once per day, and the daily
//! allowance resets at UTC midnight."*
//!
//! And later the same day, which is what this module now implements: *"instead of losing the
//! Maple Admin quest. Introduce a new public command `!tool` to create a new NPC template so
//! that this can be created. Just use MapleStory administrator as the NPC icon."*
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ or a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred - policy nothing on this
//! machine can confirm.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A favour is
//! granted to whoever holds the socket, and the daily gate limits a *character*, not a person.
//!
//! # The NPC, and how it was picked rather than guessed
//!
//! Template **9010000**, `String.wz/Npc.img/9010000/name = "Maple Administrator"` in the classic client, "MapleStory Administrator" since the Signature Style backport renamed it (`crate::signaturestyle`). **[L]**
//!
//! Five templates in this client carry that name - `800016`, `900000`, `900001`, `900002` and
//! `9010000` - so the name alone does not identify one, and picking by name would have been a
//! coin flip between five. The placements settle it, from `gm-handbook/npcs.txt`: **[L]**
//!
//! ```text
//!   89000000, 800016    Truth Booth           an event room
//!  900000001, 900000    White Map with Mob    a GM test map
//!  900000002, 900001    Black Map with Mob    a GM test map
//!  900000002, 900002    Black Map with Mob    a GM test map
//!   10001000, 9010000   Henesys               the town
//! ```
//!
//! `9010000` is the only one standing anywhere a player walks, and it stands in the starting
//! town. The other four are on maps this server has never sent anybody to.
//!
//! Their own shipped lines are already those of a helper rather than a quest-giver, which is why
//! the greeting below is written to sit beside them rather than replace them: **[L]**
//!
//! ```text
//!  d0     "Hello! Welcome to Maple World!"
//!  idle1  "Feel free to talk to me anytime you need my help."
//! ```
//!
//! ## They have a quest, they keep it, and that is why this is a command
//!
//! Quest **500005** names `9010000` as both its start and its turn-in NPC, and its own
//! `QuestInfo.2` reads *"I've already collected all the mysterious letters and exchanged them
//! for a gift today. Let's visit the Maple Administrator in Henesys again tomorrow."* **[L]**
//!
//! For one day this feature *swallowed* that quest: both click packets were claimed for
//! template 9010000 and 500005 was never offered. The owner asked for it back, so the placed
//! Administrator in Henesys is now untouched by this module - `session::npc` has no branch on
//! their template at all - and the favours are reached by typing [`COMMAND`] instead.
//!
//! ## Why a command and not a summoned NPC object, which is what was asked for
//!
//! The literal reading of *"create a new NPC template"* is a second NPC **object** - a distinct
//! object id drawn from template 9010000 - summoned onto the map beside the player. A runtime
//! spawn is genuinely possible: `!npcecho` sent two extra NPCs with server-chosen object ids
//! `6000` / `6001` long after field entry and **they appeared on screen**
//! (`research/fixtures/damage-stub-and-npcecho-faded-world-extract.log`, 2026-08-21 00:16:09,
//! *"the copies still faded"*). **[L]**
//!
//! It is the **click** that cannot be routed, and `research/npc-click.md` §2 says why. The
//! client picks which packet a click sends inside `FUN_1428de280`, off its **own** tables:
//!
//! ```text
//!   FUN_141e39b50(npc)   "this NPC has a non-empty script name"
//!                        -> [npc+0x1f0], filled AT CONSTRUCTION from the quest singleton
//!                           by TEMPLATE id (141e362dc..141e362f3)
//!     yes -> FUN_141e3c5d0 -> the quest menu -> 0x0151  { u32 questId, u32 npcTEMPLATEid }
//!     no  ------------------------------------> 0x00F2  { u32 npcOBJECTid, ... }
//! ```
//!
//! **[L]** Only `0x00F2` carries an object id. Whenever the client thinks 9010000 has an
//! offerable quest - which is exactly when the owner wants quest 500005 to work - the click arrives
//! as `0x0151` carrying the template and nothing else, and a summoned copy drawn as 9010000 is
//! **byte-for-byte indistinguishable from the real Administrator standing in Henesys**. Giving
//! their quest back and routing a summoned copy's click to this menu are the same fork pointing
//! two ways.
//!
//! Four other templates share the name *Maple Administrator* and have no quest - `800016`,
//! `900000`, `900001`, `900002` - and one of those would fork to `0x00F2` and be routable. That
//! is a real option and it is **not** taken here: it changes the icon to a template whose
//! canvas nobody has rendered, it costs a client run to find out, and it buys a walk-and-click
//! where a command already puts the box on screen instantly. Written down rather than left
//! implicit, because it is the design the owner's sentence describes.
//!
//! So the "new NPC" this feature creates is a **speaker**, not a field object: a `0x055B` whose
//! speaker field is [`ADMIN_TEMPLATE`], which is the portrait - *"just use MapleStory
//! administrator as the NPC icon"* - and whose answers are told apart from every other NPC's by
//! [`MENU_PATH`] rather than by any template or object id. Nothing is spawned, so nothing can
//! leak across a map change, a relog, or ten [`COMMAND`]s in a row.
//!
//! # The menu is `0x055B` message type 6, and its indices are FIXED
//!
//! Same widget as `crate::taxi` and `crate::secondjob`, and proven on screen by the taxis -
//! `research/fixtures/type6-menu-renders-and-taxi-rides-world.log`, 2026-08-29. **[L]**
//!
//! **An option already used today is still listed, in the same position**, with
//! `(already used today)` appended. It would read better to drop it from the list; that would
//! also make `#L1#` mean "level up" on one day and "return to Henesys" on the next, and the
//! selection number is the only thing the answer carries. A menu whose indices depend on
//! hidden state is one relog away from paying out the wrong option. Fixed positions are worth
//! more than a tidy list, and there is a test that says the list is always three lines long.
//!
//! # The text must be ASCII, and nothing else will say so
//!
//! `net::packet::PacketWriter::str` is `s.chars().map(|c| c as u8)` - one byte per char, low
//! byte only. A curly apostrophe would reach the client as `0x19` with no error anywhere.
//! Asserted by a test.
//!
//! # This module is the decision and the words; the grants live in `session::npc`
//!
//! It owns the table, the perk set, the gate's scope and every sentence. It knows nothing
//! about `Session`, so every branch below is a unit test rather than a client run. The three
//! grants need `&mut Session` - `award_experience`, `gm_reset_ap`, `gm_reset_sp` and the cash
//! wallet - and live in `crates/world/src/session/npc.rs` beside the taxi and second-job
//! handlers, which is the same split `crate::taxi` uses.

use store::dailyperks::{utc_date, SCOPE_ACCOUNT, SCOPE_CHARACTER};

// ---------------------------------------------------------------------------------------
// The NPC
// ---------------------------------------------------------------------------------------

/// The `Npc.wz` template id of the Maple Administrator in Henesys. **[L]** - see the module
/// docs for how this was told apart from the four other templates of the same name.
///
/// **Since 2026-09-08 this is an ICON and nothing else.** It is the speaker field of every
/// `0x055B` this feature sends, which is the portrait beside the text; it is *not* a template
/// this server routes clicks on. The NPC that stands on that template in Henesys belongs to
/// quest [`ADMIN_QUEST`] again and `session::npc` has no branch on them.
pub const ADMIN_TEMPLATE: u32 = 9_010_000;

/// The chat word that opens the menu, **without its `!`**: `!tool`.
///
/// **Public.** The owner, 2026-09-08: *"Introduce a new public command !tool"*. It is answered in
/// `session::gm::on_chat` **before** the GM gate, beside `!rates` and `!help`, so every account
/// can run it - see that function for why a refused command is said out loud instead.
///
/// What stops it being abused is that it grants nothing: it draws a box. The three favours
/// behind it are gated on a database row per UTC day, so a player who types this a thousand
/// times gets a thousand boxes and one day's allowance.
pub const COMMAND: &str = "tool";

/// [`COMMAND`] as the player types it, for help text and log lines. One constant so the `!`
/// cannot end up in one place and not the other.
pub const COMMAND_TYPED: &str = "!tool";

/// What `String.wz/Npc.img/9010000/name` calls them. For log lines and for the greeting.
pub const ADMIN_NAME: &str = "MapleStory Administrator";

/// The map they stand on: Henesys. **[L]** `gm-handbook/npcs.txt`. Used only by the test that
/// checks the table against the dump, so a future WZ that moves their fails loudly here rather
/// than quietly on screen.
pub const ADMIN_MAP: u32 = 10_001_000;

/// The quest the placed Administrator carries in the client's own data. **[L]**
/// `gm-handbook/questlines.txt` rows `500005 Check 0.npc` and `500005 Check 1.npc`.
///
/// **This feature no longer touches it.** It was swallowed for one day - both click packets on
/// template [`ADMIN_TEMPLATE`] were claimed - and the owner asked for it back on 2026-09-08:
/// *"instead of losing the Maple Admin quest"*. The constant survives the change because the
/// reason this is a command rather than a summoned NPC is precisely that this quest exists; see
/// the module doc.
pub const ADMIN_QUEST: u32 = 500_005;

// ---------------------------------------------------------------------------------------
// The three numbers that are policy
// ---------------------------------------------------------------------------------------

/// **The owner's number**: *"Gain 1000 Leaf Points"*. **[I]**, and one constant rather than a
/// per-level or per-day-streak table, because a second rule here is a decision nobody made.
pub const LEAF_POINTS_PER_CLAIM: u32 = 1_000;

/// The line break: the **two characters** backslash and `n`, not a real `0x0A`.
///
/// The client's `#`-token expander treats a token beginning with `\` and one beginning with
/// `\r` identically - both reach the line flush. **[L]**, `crate::taxi`'s module doc carries
/// the addresses. Read from `crate::taxi` rather than restated so one client run measures the
/// line break for every menu in the server at once.
pub const LINE_BREAK: &str = crate::taxi::LINE_BREAK;

// ---------------------------------------------------------------------------------------
// Telling this menu's answer from a taxi's or an instructor's
// ---------------------------------------------------------------------------------------

/// The prefix every daily-perk conversation path starts with.
///
/// Disjoint from `crate::taxi::PATH_PREFIX` (`"taxi."`) and
/// `crate::secondjob::MENU_PATH`, which is what lets three features share one packet type -
/// a type-6 `0x00F3` carries no speaker, so session state is the only discriminator. There is
/// a test asserting none of the three can claim another's answer.
pub const PATH_PREFIX: &str = "dailyperk.";

/// The `Conversation::path` that marks a live daily-perk menu. Built from [`PATH_PREFIX`] so
/// the two cannot drift.
pub const MENU_PATH: &str = concat!("dailyperk.", "menu");

/// Is this conversation a daily-perk menu waiting for an answer?
///
/// Exact rather than prefixed: `"dailyperk."` alone, or some future
/// `"dailyperk.something-else"`, is not a live menu and must not be routed as one. Same rule
/// and same reasoning as [`crate::taxi::is_taxi_path`].
pub fn is_menu_path(path: &str) -> bool {
    path == MENU_PATH
}

// ---------------------------------------------------------------------------------------
// The perks
// ---------------------------------------------------------------------------------------

/// One of the three favours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Perk {
    /// [`LEAF_POINTS_PER_CLAIM`] into the cash wallet - the currency every price tag in the
    /// cash shop is quoted in.
    LeafPoints,
    /// Exactly the experience needed to reach the next level, from the same
    /// `crate::expcurve::ExpCurve` every kill and every turn-in uses.
    LevelUp,
    /// A `SetField` to Henesys - [`HENESYS`], its first spawn point. The owner, 2026-09-14:
    /// *"Since there may be unexpected outcomes as we implement the server, the player may
    /// become stuck in certain maps. There should be an additional option also limited to
    /// once a day to teleport the player directly to Henesys."* An escape hatch, priced at a
    /// day so it is not a free taxi.
    ReturnToHenesys,
}

/// The three, in menu order. **The index into this array IS the `#L` number** - see
/// [`perk_at`] - so the order is load-bearing rather than cosmetic. New favours go on the
/// END: a player's muscle memory for "#L1 is level up" is worth more than a tidy grouping.
pub const PERKS: [Perk; 3] = [Perk::LeafPoints, Perk::LevelUp, Perk::ReturnToHenesys];

/// Where [`Perk::ReturnToHenesys`] lands: Henesys in this client's numbering
/// (`gm-handbook/maps.txt` row `10001000, Henesys` - **not** the `100000000` of other
/// versions), at spawn portal 0, which `gm-handbook/portals.txt` puts at (112, 197).
pub const HENESYS: u32 = 10_001_000;
/// The spawn point in [`HENESYS`] the escape lands on.
pub const HENESYS_PORTAL: u8 = 0;

impl Perk {
    /// The string this perk's claim row is keyed on. **Stored in a database**, so it is a
    /// wire format: changing one of these silently gives every character their daily allowance
    /// back. There is a test pinning all three.
    pub fn store_key(self) -> &'static str {
        match self {
            Perk::LeafPoints => "leafpoints",
            Perk::LevelUp => "levelup",
            Perk::ReturnToHenesys => "henesys",
        }
    }

    /// **Which allowance this perk spends: the account for Leaf Points, the character for the
    /// other two.**
    ///
    /// The owner, 2026-09-08: *"Make leaf point claim per account."* The reason the question came up
    /// at all is that the pot and the allowance were on different footings: the cash wallet is
    /// per **account** (`store::cash`, and `!lp`'s own help line says so), so while the claim
    /// was per character a player with three characters banked 3 x [`LEAF_POINTS_PER_CLAIM`] a
    /// day into one shared wallet. Now the allowance is on the same footing as the pot it pays
    /// into, and 1000 a day means 1000 a day however many characters the account has.
    ///
    /// **Level-up and the return to Henesys stay per character on purpose.** Both change one
    /// character's own row, so an account-wide allowance would mean levelling one character
    /// spends the other five's turn - a coupling nobody asked for. The rule that falls out is
    /// worth stating: *a perk's allowance belongs to whatever its grant actually modifies.*
    ///
    /// No migration was needed for the switch, because the claim table is keyed on
    /// `(scope, scope_id, perk)` and the two scopes are different rows. An account that had
    /// already spent a character-scoped Leaf Point claim today therefore got one more that
    /// day, once. That is a one-off, on a feature that has never been on a client.
    pub fn scope(self) -> &'static str {
        match self {
            Perk::LeafPoints => SCOPE_ACCOUNT,
            Perk::LevelUp | Perk::ReturnToHenesys => SCOPE_CHARACTER,
        }
    }

    /// The menu line, as the player reads it.
    pub fn label(self) -> &'static str {
        match self {
            Perk::LeafPoints => "Gain 1000 Leaf Points",
            Perk::LevelUp => "Level up",
            Perk::ReturnToHenesys => "Return to Henesys (if you are stuck)",
        }
    }

    /// The `#L` number this perk sits on. The inverse of [`perk_at`].
    pub fn selection(self) -> u32 {
        PERKS.iter().position(|p| *p == self).unwrap_or(0) as u32
    }
}

/// The perk a menu number names, or `None` for a number this menu never offered.
///
/// **`0xFFFFFFFE` is a real thing the client sends** on its own special path
/// (`141f739b4  cmp edi, -2`) and is not a line the server wrote. It lands here as `None` like
/// any other out-of-range number, which is the only safe reading. **[L]**
pub fn perk_at(selection: u32) -> Option<Perk> {
    PERKS.get(usize::try_from(selection).ok()?).copied()
}

// ---------------------------------------------------------------------------------------
// The words
// ---------------------------------------------------------------------------------------

/// One selectable line, in **the client's own format**: `#d#L%d# %s#l#k`.
///
/// Not invented - it is the literal `FUN_141e3c5d0` uses to build the client's *own* NPC menu
/// (`research/msexe-packet-fields.txt`, row `0x00F2`), and `crate::taxi::menu_line` uses the
/// same one. Read from there rather than restated. **[L]**
pub fn menu_line(selection: u32, text: &str) -> String {
    crate::taxi::menu_line(selection, text)
}

/// What they say above the list. Written to sit beside their own shipped `d0` and `idle1`.
pub fn header() -> String {
    "Hello! Welcome to Maple World! I can help you with one of each of these a day - \
     the list refills at #bmidnight UTC#k."
        .to_string()
}

/// The whole menu string: the header, a blank line, then one line per entry of [`PERKS`].
///
/// `used` is per-perk and in [`PERKS`] order: `true` marks an option already spent today. A
/// spent option keeps its position and its number - see the module doc for why that matters
/// more than a tidy list.
///
/// The blank line is the client's own layout: every one of the 33 authored menus in
/// `gm-handbook/questlines.txt` is written `<question> \n\n#L0# ...`. **[L]**
pub fn menu_text(used: [bool; PERKS.len()]) -> String {
    let mut out = header();
    out.push_str(LINE_BREAK);
    for (i, perk) in PERKS.iter().enumerate() {
        out.push_str(LINE_BREAK);
        let label = if used[i] {
            format!("{} (already used today)", perk.label())
        } else {
            perk.label().to_string()
        };
        out.push_str(&menu_line(i as u32, &label));
    }
    out
}

/// The refusal for the escape when the character is already standing in Henesys. Decided
/// before the claim, so it costs nothing: a teleport to where you already are is a no-op, and
/// a no-op must not eat the day.
pub fn already_in_henesys() -> String {
    "You are already in Henesys! Save this one for a day you are actually stuck.".to_string()
}

/// The refusal for an option that has already been used today.
///
/// **A refusal is a reply.** `CLAUDE.md`'s *always answer*: this cannot be silence, and it
/// cannot be an error in place of a dialogue.
pub fn already_used_today(perk: Perk, day: i64) -> String {
    format!(
        "You have already had #b{}#k today ({} UTC). Come back after #bmidnight UTC#k \
         and it will be waiting.",
        perk.label(),
        utc_date(day)
    )
}

/// The selection named no line this menu offered - including the client's own `-2`.
pub fn no_such_option() -> String {
    format!(
        "I do not know that one. There are only {} things I can help you with today.",
        PERKS.len()
    )
}

/// A perk was claimed and the grant went through. `detail` is the perk's own sentence.
pub fn granted(detail: &str) -> String {
    format!("{detail} That is one a day - come back after #bmidnight UTC#k.")
}

/// Leaf Points went in. The balance is stated because nothing pushes a wallet update: the
/// number reaches the screen in the `0x05AD` that travels with `SetCashShop`, and the client's
/// own poll for it is throttled to once a minute (`session::cashshop`, `!lp`'s own help line).
/// So the sentence is the only place the player sees it before opening the shop.
pub fn leaf_points_line(added: u32, balance: u32) -> String {
    format!(
        "There you go - #b{added} Leaf Points#k, and you now hold #b{balance}#k. \
         They are the LP the Cash Shop prices everything in, and the wallet belongs to your \
         #baccount#k rather than to this character."
    )
}

/// A level was granted, and how much experience it took.
pub fn level_up_line(from: u32, to: u32, exp: u64) -> String {
    format!("#b{exp}#k experience - exactly what you needed. You are level #b{to}#k now, up from #b{from}#k.")
}

/// The character is at the cap, so there is no next level to buy. **Refused before the claim
/// is taken**, so it costs nobody their day.
pub fn already_max_level(level: u32) -> String {
    format!(
        "You are level #b{level}#k, and that is as high as this world goes. \
         I have nothing to give you there - and I have not used up your day."
    )
}

/// The experience curve has no row for this level, so there is no number to grant. Cannot
/// happen with the shipped `data/exp-curve.txt` (levels 1..119) and is a sentence rather than
/// a panic because a missing data file must not read as a frozen client.
pub fn no_curve_for_level(level: u32) -> String {
    format!(
        "Something is wrong with my records and I cannot tell what level #b{level}#k costs. \
         Your day is untouched - tell whoever runs this world."
    )
}


/// The grant failed after the day was claimed, and the day has been given back.
///
/// This sentence exists because the alternative is the worst outcome the feature has: a player
/// who is told nothing, receives nothing, and cannot try again until tomorrow.
pub fn grant_failed_day_returned(perk: Perk, why: &str) -> String {
    format!(
        "Something went wrong and #b{}#k did not happen: {why}. \
         I have #bnot#k used up your day - try me again.",
        perk.label()
    )
}

/// A log label for a claim that was taken, said the way `world.log` wants to read it.
pub fn claim_note(perk: Perk, scope_id: i64, day: i64) -> String {
    // Names the row it actually stamped. This used to say "for character {id}" unconditionally
    // and print "scope account" beside it, which is a line that contradicts itself in a log
    // somebody will one day read to answer "why did this player get two".
    format!(
        "daily perk {:?} CLAIMED for {} {scope_id}, UTC day {day} ({}). This is the \
         transition; every effect below hangs off it",
        perk,
        perk.scope(),
        utc_date(day)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- the NPC -------------------------------------------------------------------------

    /// **The template, against the dump it was read from.** `gm-handbook/` is generated and
    /// gitignored, so this degrades to a no-op on a clean checkout - and therefore asserts a
    /// positive control first, the way `crate::taxi`'s equivalent does. Without the control an
    /// empty file would make this test pass by finding nothing, which is the exact failure
    /// `CLAUDE.md` spends a section on.
    #[test]
    fn the_administrator_stands_in_henesys_and_is_the_only_one_who_does() {
        let npcs = std::path::Path::new("../../gm-handbook/npcs.txt");
        let strings = std::path::Path::new("../../gm-handbook/npcstrings.txt");
        if !npcs.exists() || !strings.exists() {
            return;
        }
        let text = std::fs::read_to_string(npcs).expect("npcs.txt");
        let rows: Vec<(u32, u32)> = text
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| {
                let f: Vec<&str> = l.split(',').map(str::trim).collect();
                Some((f.first()?.parse().ok()?, f.get(1)?.parse().ok()?))
            })
            .collect();
        assert!(rows.len() > 250, "positive control: {} placements loaded", rows.len());

        let names = std::fs::read_to_string(strings).expect("npcstrings.txt");
        let name_of = |t: u32| -> Option<String> {
            names.lines().find_map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                (f.len() == 3 && f[0].parse::<u32>().ok()? == t && f[1] == "name")
                    .then(|| f[2].to_string())
            })
        };
        assert_eq!(name_of(322).as_deref(), Some("Joel"), "positive control on the name join");

        assert_eq!(name_of(ADMIN_TEMPLATE).as_deref(), Some(ADMIN_NAME));
        assert!(
            rows.contains(&(ADMIN_MAP, ADMIN_TEMPLATE)),
            "template {ADMIN_TEMPLATE} is not placed on map {ADMIN_MAP}"
        );
        // **The discriminator, asserted rather than remembered.** Five templates share the
        // name; exactly one of them stands anywhere a player can walk to, and that is the whole
        // reason this template was chosen. If a future dump places another of them in a town,
        // the choice needs re-making and this fails.
        // The backport renamed 9010000 alone, so the four others still carry the classic name:
        // an Administrator is either.
        let is_admin = |t: u32| matches!(name_of(t).as_deref(), Some(n) if n == ADMIN_NAME || n == "Maple Administrator");
        let same_name: Vec<u32> = rows.iter().map(|(_, t)| *t).filter(|t| is_admin(*t)).collect();
        let in_towns: Vec<u32> = rows
            .iter()
            .filter(|(m, t)| is_admin(*t) && (10_000_000..10_100_000).contains(m))
            .map(|(_, t)| *t)
            .collect();
        assert!(same_name.len() >= 4, "several templates share the name: {same_name:?}");
        assert_eq!(in_towns, vec![ADMIN_TEMPLATE], "only one of them stands in a town");
    }

    /// **The command word, pinned.** `on_chat` splits on the `!` and then on whitespace, so a
    /// word carrying either would be unreachable - and it is matched against a `&str` literal
    /// arm, so a word with an upper-case letter would simply never fire. None of those failures
    /// says anything on screen; they all read as "the command does nothing".
    #[test]
    fn the_command_word_is_something_the_chat_dispatcher_can_reach() {
        assert_eq!(COMMAND, "tool", "the owner asked for !tool by name");
        assert_eq!(COMMAND_TYPED, format!("!{COMMAND}"), "the two must not drift");
        assert!(!COMMAND.contains('!'), "on_chat strips the ! before matching");
        assert!(!COMMAND.contains(char::is_whitespace), "on_chat splits the name off at the first space");
        assert!(COMMAND.chars().all(|c| c.is_ascii_lowercase()), "the match arm is case-sensitive");
        assert!(!COMMAND.is_empty(), "the empty name is already the dispatcher's error arm");
    }

    /// **The quest is the Administrator's again, and this module only names it.** Nothing here
    /// may route a click, so the one thing worth asserting is that the constant still points at
    /// the quest whose existence is the whole reason this is a command - see the module doc.
    /// Degrades to a no-op on a clean checkout, so it asserts a positive control first.
    #[test]
    fn the_administrators_own_quest_is_real_and_starts_and_ends_at_her() {
        let quests = std::path::Path::new("../../gm-handbook/questlines.txt");
        if !quests.exists() {
            return;
        }
        let text = std::fs::read_to_string(quests).expect("questlines.txt");
        let quest = ADMIN_QUEST.to_string();
        let template = ADMIN_TEMPLATE.to_string();
        let rows: Vec<Vec<&str>> = text.lines().map(|l| l.split('\t').collect()).collect();
        assert!(rows.len() > 1_000, "positive control: {} rows loaded", rows.len());
        let npc_rows: Vec<&Vec<&str>> = rows
            .iter()
            .filter(|f| f.len() == 4 && f[0] == quest && f[1] == "Check")
            .filter(|f| f[2] == "0.npc" || f[2] == "1.npc")
            .collect();
        assert!(
            !npc_rows.is_empty(),
            "quest {ADMIN_QUEST} names no NPC - the reason this feature is a command has gone"
        );
        for row in npc_rows {
            assert_eq!(
                row[3], template,
                "quest {ADMIN_QUEST} {} is not template {ADMIN_TEMPLATE}",
                row[2]
            );
        }
    }

    // -- the menu ------------------------------------------------------------------------

    /// **The selection number is the index into [`PERKS`], in both directions.** Everything
    /// downstream trusts this: a mismatch pays out the wrong option to somebody who clicked
    /// the right line.
    #[test]
    fn the_menu_number_and_the_perk_agree_in_both_directions() {
        for (i, perk) in PERKS.iter().enumerate() {
            assert_eq!(perk_at(i as u32), Some(*perk), "line {i}");
            assert_eq!(perk.selection(), i as u32);
        }
        assert_eq!(perk_at(PERKS.len() as u32), None, "one past the end is not an option");
        assert_eq!(perk_at(u32::MAX), None, "and neither is the client's own -2");
        assert_eq!(PERKS.len(), 3, "Leaf Points and a level (2026-09-08) and the Henesys escape (2026-09-14); the AP/SP reset went to the Cash Shop on 2026-10-02");
    }

    /// **A used option keeps its number.** This is the invariant the module doc argues for,
    /// and the one that would fail silently: with a shorter list on a day when one option is
    /// spent, `#L1#` names a different perk and the payout follows the wrong line.
    #[test]
    fn a_used_option_keeps_its_line_and_its_number() {
        let fresh = menu_text([false; PERKS.len()]);
        for (i, perk) in PERKS.iter().enumerate() {
            assert!(fresh.contains(&menu_line(i as u32, perk.label())), "{perk:?} is missing");
        }
        // Every one of the eight used/unused combinations still draws three lines, and each
        // perk is still on its own number.
        for mask in 0u8..8 {
            let used = [mask & 1 != 0, mask & 2 != 0, mask & 4 != 0];
            let text = menu_text(used);
            for (i, perk) in PERKS.iter().enumerate() {
                assert!(
                    text.contains(&format!("#L{i}# {}", perk.label())),
                    "mask {mask:03b}: {perk:?} left line {i}"
                );
            }
            assert_eq!(text.matches("#L").count(), PERKS.len(), "mask {mask:03b}: one line per favour");
            assert_eq!(
                text.matches("(already used today)").count(),
                used.iter().filter(|u| **u).count(),
                "mask {mask:03b}: the used markers"
            );
        }
    }

    /// The menu is one string with the client's own layout: header, blank line, then the list.
    #[test]
    fn the_menu_opens_with_the_header_and_a_blank_line() {
        let text = menu_text([false; PERKS.len()]);
        assert!(text.starts_with(&header()));
        assert!(
            text.starts_with(&format!("{}{LINE_BREAK}{LINE_BREAK}#d#L0#", header())),
            "header, blank line, then line 0: {text:?}"
        );
        assert_eq!(LINE_BREAK, "\\n", "two characters, not a real newline");
        assert!(!text.contains('\n'), "a real 0x0A would not reach the line flush");
    }

    /// **Every sentence this module can put on screen is ASCII.** `PacketWriter::str` writes
    /// one byte per char and drops the high bytes silently, so a curly apostrophe anywhere in
    /// here becomes `0x19` on the client with no error in any log.
    #[test]
    fn every_sentence_is_ascii() {
        let mut all: Vec<String> = vec![
            header(),
            no_such_option(),
            menu_text([false, true, false]),
            already_in_henesys(),
            leaf_points_line(LEAF_POINTS_PER_CLAIM, 4_200),
            level_up_line(12, 13, 1_003),
            already_max_level(120),
            no_curve_for_level(500),
            granted("Done."),
        ];
        for perk in PERKS {
            all.push(perk.label().to_string());
            all.push(already_used_today(perk, 20_704));
            all.push(grant_failed_day_returned(perk, "the database said no"));
            all.push(claim_note(perk, 200, 20_704));
        }
        all.push(ADMIN_NAME.to_string());
        // The command word reaches the screen twice - in `!help` and in the log line that says
        // who opened the box - so it is held to the same rule as every sentence.
        all.push(COMMAND.to_string());
        all.push(COMMAND_TYPED.to_string());
        for s in all {
            assert!(s.is_ascii(), "not ASCII: {s:?}");
        }
    }

    // -- the gate's keys -----------------------------------------------------------------

    /// **The store keys are a wire format**, because they are in a database that outlives the
    /// build. Renaming one hands every character their allowance back, silently and once.
    #[test]
    fn the_store_keys_are_pinned_and_distinct() {
        assert_eq!(Perk::LeafPoints.store_key(), "leafpoints");
        assert_eq!(Perk::LevelUp.store_key(), "levelup");
        assert_eq!(Perk::ReturnToHenesys.store_key(), "henesys");
        assert!(PERKS.iter().all(|p| p.store_key() != "resetapsp"), "the retired reset key is never reused");
        let mut keys: Vec<&str> = PERKS.iter().map(|p| p.store_key()).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), PERKS.len(), "two perks sharing a key would share an allowance");
    }

    /// **A perk's allowance belongs to whatever its grant modifies.** Leaf Points pay into an
    /// account-wide wallet, so the allowance is account-wide; the other two change one
    /// character's own row, so they are per character. Stated as a test in BOTH directions, so
    /// that flipping either one is a deliberate act rather than a side effect - asserting only
    /// the Leaf Point half would pass against a version that made all three account-scoped and
    /// silently coupled the six characters' level-ups.
    #[test]
    fn leaf_points_are_scoped_to_the_account_and_the_other_two_to_the_character() {
        assert_eq!(Perk::LeafPoints.scope(), store::SCOPE_ACCOUNT);
        for perk in [Perk::LevelUp, Perk::ReturnToHenesys] {
            assert_eq!(
                perk.scope(),
                store::SCOPE_CHARACTER,
                "{perk:?} changes one character's own row, so it must not spend the account's turn"
            );
        }
        assert_ne!(store::SCOPE_CHARACTER, store::SCOPE_ACCOUNT);
    }

    // -- the discriminator ---------------------------------------------------------------

    /// **Three features share message type 6 and only session state tells them apart.** A
    /// type-6 `0x00F3` carries no speaker, so if two of these path prefixes ever overlapped,
    /// one feature would silently eat the other's answers. Same check `crate::taxi` and
    /// `crate::jobguide` make about each other.
    #[test]
    fn this_menu_path_cannot_be_confused_with_a_taxi_or_an_instructor() {
        assert!(is_menu_path(MENU_PATH));
        assert!(!is_menu_path(PATH_PREFIX), "the bare prefix is not a live menu");
        assert!(!is_menu_path("dailyperk.something-else"));
        assert!(!is_menu_path(""), "a plain talk is not this menu");
        assert!(!is_menu_path("0") && !is_menu_path("0.yes"), "nor is a quest path");

        assert!(!crate::taxi::is_taxi_path(MENU_PATH), "a taxi must not claim this answer");
        assert!(!crate::secondjob::is_menu_path(MENU_PATH), "nor an instructor");
        assert!(!is_menu_path(crate::taxi::MENU_PATH), "and this must not claim a taxi's");
        assert!(!is_menu_path(crate::secondjob::MENU_PATH));
        assert!(
            !MENU_PATH.starts_with(crate::taxi::PATH_PREFIX)
                && !crate::taxi::MENU_PATH.starts_with(PATH_PREFIX),
            "the two prefixes are disjoint"
        );
        assert!(MENU_PATH.starts_with(PATH_PREFIX), "and the constant is built from the prefix");
    }

    // -- the numbers ---------------------------------------------------------------------

    #[test]
    fn the_leaf_point_grant_is_remys_thousand() {
        assert_eq!(LEAF_POINTS_PER_CLAIM, 1_000);
        assert!(leaf_points_line(LEAF_POINTS_PER_CLAIM, 1_000).contains("1000 Leaf Points"));
    }

    /// The refusals name the day they are refusing for, so a screenshot is enough to tell a
    /// gate that is working from a clock that is wrong.
    #[test]
    fn a_refusal_says_which_utc_day_it_is_refusing_for() {
        let line = already_used_today(Perk::LevelUp, 20_704);
        assert!(line.contains("2026-09-08"), "{line}");
        assert!(line.contains("Level up"), "{line}");
        assert!(line.contains("midnight UTC"), "{line}");
    }
}
