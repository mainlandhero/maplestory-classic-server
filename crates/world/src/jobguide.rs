//! **Phil, the informant of Lith Harbor: choosing a job path, and the ride to the instructor.**
//!
//! The owner, 2026-08-29: *"For Phil in Victoria Island, users who are of Beginner job should have
//! the option of talking to Phil and choosing which job advancement they want to take. For any
//! particular job path they take, it should teleport the character immediately to the Job
//! Instructor of their choice on the appropriate map."*
//!
//! The client's own quest text asks for the same thing in as many words. `Quest.wz`'s
//! `10001.img` ("Phil's Call") `QuestInfo.2` reads *"Talking to #p101# will allow you to go
//! straight to your town of choice"*, and its `Say.1.yes.3` is *"once you've decided which job
//! you want to advance to, come talk to me again. #rI can send you to the town you need to get
//! to at a special discounted price, just for beginner adventurers!#k"*. **[L]** from
//! `gm-handbook/quests.json`. So this is not a feature bolted onto an NPC that has nothing to
//! do with it; it is the behaviour Phil's own script was written for and this client never
//! shipped.
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ or a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A ride is given
//! to whoever holds the socket.
//!
//! # What this module is, and what it deliberately is not
//!
//! It is the **pure decision and the words**: given a character and an answer byte, what does
//! Phil say next and where does the player end up. It sends no packets, touches no database
//! and knows nothing about `Session`, exactly like [`crate::jobs`] and [`crate::taxi`], so
//! every branch below is a unit test rather than a client run.
//!
//! **It is not wired.** `crates/world/src/session/` belongs to the coordinator; the patch is in
//! the hand-back report and the anchors are named in [`on_reply`]'s doc block. `CLAUDE.md`'s
//! *"built is not wired"*.
//!
//! # 1. Phil, with provenance
//!
//! | fact | value | evidence |
//! |---|---|---|
//! | template | **101** | `gm-handbook/npcs.txt`: `10000000, 101, 666, 347, 190, 616, 716, 0, Phil`. **[L]** |
//! | map | **10000000**, Lith Harbor | the same row; and it is theirs **only** spawn in the client - `awk -F', *' '$2==101'` over `npcs.txt` returns one line. **[L]** |
//! | quest | **10001**, "Phil's Call", `Check.0.npc = 101`, `Check.1.npc = 101` | `gm-handbook/quests.json`. **[L]** |
//! | dialogue | **they have no `d0` line at all** - only `idle0..2` / `info0..2` | `gm-handbook/npcstrings.txt`. **[L]** |
//!
//! That last row is why clicking Phil today prints *"This server has no dialogue for NPC
//! template 101 yet."* - observed, `previous-runs/world-20260821-230104.log 02:50:56.185`.
//!
//! # 2. **A click on Phil arrives as EITHER packet, and that decides the wiring**
//!
//! Measured rather than reasoned. One archived run has Phil on screen and clicked;
//! deduplicated on `(timestamp, line)` across all 418 logs in `previous-runs/` and
//! `research/fixtures/` - **event-level, not file-level**, because those two directories
//! overlap and a fixture copied mid-write hashes differently from its own run:
//!
//! ```text
//! 02:50:44.253 <- 0x0151  01 11270000 65000000 74025b01 00000000   action 1, quest 10001, npc 101
//! 02:50:49.984 <- 0x0151  02 11270000 65000000 75025b01 ffffffff   action 2, quest 10001, npc 101
//!    ... quest 10001 completed, 171 exp and ten potions paid ...
//! 02:50:56.185 <- 0x00F2  e9030000 75025b01 ffffffff               object id 1001 = Phil
//! 02:50:56.185 -> 0x055B  "This server has no dialogue for NPC template 101 yet."
//! ```
//!
//! **[L]**, `previous-runs/world-20260821-230104.log`. So while quest 10001 is offerable or
//! completable the client sends `0x0151` and the click never reaches `on_npc_click`; once it is
//! finished the same NPC sends `0x00F2`. `research/npc-click.md` §2.1 explains why - the client
//! picks between the two from its own quest tables - and this is that fork observed on Phil
//! specifically. **The wiring therefore hooks both handlers**, and a patch that only hooked
//! `on_npc_click` would do nothing for a beginner who has just met them.
//!
//! # 3. The dialogue shape: a chain of yes/no boxes, not a list
//!
//! **This client has no usable select-one-of-N script box.** All 71 entries of the `0x055B`
//! message-type jump table at `0x141f6f9f4` were enumerated - handler, body fields, and the
//! `0x00F3` answer shape of each - and the two halves a menu needs never occur together:
//! type `0x02` is a real list box (`u8 count` then `count` strings, `141f702e2..141f70349`)
//! whose answer is `u32 0, u8 2, u8` with **no index** (`141f703f2..141f70406`), and type
//! `0x13` answers with a genuine index but into a list the *client* owns. **[L]**, and see
//! [`crate::taxi`]'s module header, which carries that enumeration. A menu built on either
//! would look right on screen and answer every choice identically.
//!
//! There is **one unresolved candidate** and it is recorded rather than acted on; see §6.
//!
//! So Phil offers one path at a time on a **type 3** yes/no box - `BtYes`/`BtNo`, answer `1`
//! Yes / `0` No / `-1` closed, unambiguous (`research/script-reply.md` §3.2), and already on
//! The owner's screen. Yes rides, No offers the next. **Every box carries all four paths with the
//! current offer marked**, so it reads as a menu even though the mechanism is a chain.
//!
//! Type **3** and not `0x10`: `0x10` wears the quest captions `BtQYes`/`BtQNo` (`142a660b4`),
//! and choosing a career is not a quest. **[L]**
//!
//! # 4. `\n` is a line break, and it is two characters
//!
//! Backslash then `n`, not `0x0A`. In the `#`-token expander `FUN_142a45580`,
//! `142a45b3f cmp byte ptr [rax], 0xd` and `142a45b80 cmp byte ptr [rax], 0x5c` branch to the
//! same arm `0x142a4d9bd`; 144 `Say` lines in this client's `Quest.wz` use `\n` and no string
//! in `String.wz/Npc.img` contains a real `0x0A`. **[L]** Established by the taxi work and
//! taken as given here rather than re-derived. The same expander is on both the Say path
//! (`141f6fcb1`) and the yes/no path (`141f7098b`), so `#b` / `#k` behave the same in both.
//!
//! [`LINE_BREAK`] is that sequence. **It is duplicated from [`crate::taxi::LINE_BREAK`] on
//! purpose** - the two modules were written in parallel and neither may edit the other. One of
//! the two should go at integration; the constant, not the fact, is the duplicate.
//!
//! # 5. Where the player is sent
//!
//! **The routing table is [`crate::jobs::FIRST_JOBS`] itself** - see [`destination`]. It is not
//! retyped here, so a fifth branch, a moved instructor or a renamed job cannot appear in one
//! place and not the other. `crate::firstjob`'s header records that this client has **no Pirate
//! branch**, and there are four offers because that table has four rows.
//!
//! **The destination is the instructor's own map, not the town.** The owner asked for "the Job
//! Instructor of their choice on the appropriate map"; the WZ text offers only the town. Those
//! differ for all four, and [`crate::jobs::FirstJob`] carries both so the difference stays
//! visible. Sending the player to the *town* id would land them one map away from anybody who
//! can advance them.
//!
//! # 6. Phil routes; they never advance - and the path they route into has never been watched
//!
//! Nothing here returns a job, writes a job, or builds a job-change packet. The player arrives
//! beside the instructor and goes through `Session::advance_job_for`, which already exists and
//! already decides correctly.
//!
//! **That path has never been seen to run.** Zero `ScriptMessage … from instructor` lines exist
//! in 675 014 distinct archived events, and the search is not blind: the same sweep finds 516
//! `NpcEnterField` lines and 2 for Dark Lord specifically. The one archived instructor click -
//! Dark Lord, `previous-runs/world-20260827-210956.log 01:07:55.996`, a `0x00F2` - predates the
//! wiring commit `a2ffa46` (2026-08-28 14:50) and got the plain `d0` line. So the second half of
//! this feature is wired-but-unobserved, and the ride is what finally makes it reachable
//! without a GM command.
//!
//! ## The box is a type-6 menu now - the yes/no chain was unselectable
//!
//! The owner, 2026-09-15: *"Phil's dialogue to allow Beginners to choose a location to job advance
//! to does not work. The selection is fundamentally broken and cannot be selected by the
//! cursor."* The chain of yes/no boxes drew a list with a `#b> ` marker on one line - it
//! *looked* like a menu and was not one: the only controls were Yes and No, and the cursor had
//! nothing to pick. The taxi settled the alternative on 2026-08-29 (`research/fixtures/
//! type6-menu-renders-and-taxi-rides-world.log`): a server-sent `0x055B` type 6 with
//! `#L<n>#...#l` lines draws a selectable list, and its `0x00F3` carries the pick as
//! `u32 0, u8 6, u8 1, u32 sel` **[L]**. So Phil sends ONE menu - the same
//! `net::script::npc_menu` the taxi and the second-job instructors use - parked at
//! [`MENU_PATH`], and the pick rides. The guard (`board`) is unchanged and is still the only
//! thing that can produce a [`Step::Ride`].

use std::collections::HashSet;

use net::opcode::Character;

use crate::jobs::{self, FirstJob};

// ---------------------------------------------------------------------------------------
// Who Phil is
// ---------------------------------------------------------------------------------------

/// Phil's `String.wz/Npc.img` template id. **[L]** `gm-handbook/npcs.txt`.
pub const PHIL_TEMPLATE: u32 = 101;

/// The one map Phil stands on: Lith Harbor. **[L]** - their only `life` entry in the client.
pub const PHIL_MAP: u32 = 10_000_000;

/// "Phil's Call". Its `Check.0.npc` and `Check.1.npc` are both `101`, which is why a click on
/// Phil arrives as `0x0151` until it is finished and as `0x00F2` afterwards. **[L]**
pub const PHIL_QUEST: u32 = 10_001;

/// The two characters this client's own text uses for a line break - backslash, `n`, **not**
/// `0x0A`. See the module header §4. **[L]**
///
/// Written as an escaped backslash so the bytes on the wire are `0x5C 0x6E`.
///
/// **Duplicated from [`crate::taxi::LINE_BREAK`] on purpose**: the two modules were built in
/// parallel and neither may edit the other. De-duplicate at integration.
pub const LINE_BREAK: &str = "\\n";

// ---------------------------------------------------------------------------------------
// The routing table - derived, never retyped
// ---------------------------------------------------------------------------------------

/// The destination an offer number names. **The table is [`jobs::FIRST_JOBS`]**; the offer
/// index is its index, so the two cannot drift.
pub fn destination(index: usize) -> Option<&'static FirstJob> {
    jobs::FIRST_JOBS.get(index)
}

/// Every destination with the offer number that reaches it, in table order.
pub fn destinations() -> impl Iterator<Item = (usize, &'static FirstJob)> {
    jobs::FIRST_JOBS.iter().enumerate()
}

// ---------------------------------------------------------------------------------------
// Holding the conversation's place, without a new field on Session
// ---------------------------------------------------------------------------------------

/// The one `Conversation::path` Phil's menu parks at. `jobguide.` is the prefix the old
/// per-offer chain used, kept so nothing that checked "is this Phil's" changes shape; a quest
/// path is `"0"` / `"0.yes"` / `"0.no"`, a plain talk is `""`, and the taxi and the
/// second-job instructors park at their own constants (a test says the three differ).
pub const PATH_PREFIX: &str = "jobguide.";

/// The menu's path.
pub const MENU_PATH: &str = "jobguide.menu";

/// Whether a conversation path is Phil's menu.
pub fn is_menu_path(path: &str) -> bool {
    path == MENU_PATH
}

// ---------------------------------------------------------------------------------------
// The words
// ---------------------------------------------------------------------------------------

/// The line above the list on every box.
pub const HEADING: &str =
    "So you've made up your mind? Good. I can send you straight to the one who'll take you on.";

/// One selectable line per destination: `#d#L<n># <job> - <instructor> in <map> (needs 35
/// STAT)#l#k`, the same token shape the client's own quest menu and the taxi use.
///
/// The stat requirement is on each line because Phil does **not** check it - they route, the
/// instructor refuses - so a player who cannot yet qualify can see that before spending the
/// trip.
pub fn menu_line(index: usize, dest: &FirstJob) -> String {
    format!(
        "#d#L{index}# {} - {} in {} (needs {} {})#l#k",
        dest.job_name,
        dest.npc_name,
        dest.map_name,
        jobs::STAT_MINIMUM,
        dest.stat.label(),
    )
}

/// The whole menu: the heading, a blank line, then one [`menu_line`] per destination - the
/// blank line is the client's own layout, as the taxi's builder says.
pub fn menu_text() -> String {
    let mut out = String::from(HEADING);
    out.push_str(LINE_BREAK);
    for (i, dest) in destinations() {
        out.push_str(LINE_BREAK);
        out.push_str(&menu_line(i, dest));
    }
    out
}

/// What Phil says when the box is closed with nothing picked.
pub fn farewell() -> String {
    "Take your time, then. I will be right here when you have decided.".to_string()
}

/// The chat line that goes out with the ride, naming who to look for on arrival.
pub fn arrival_line(dest: &FirstJob) -> String {
    format!(
        "Phil sends you to {}. {} is here - talk to them to become {} {}.",
        dest.map_name,
        dest.npc_name,
        article(dest.job_name),
        dest.job_name
    )
}

fn article(word: &str) -> &'static str {
    match word.chars().next() {
        Some('A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    }
}

// ---------------------------------------------------------------------------------------
// The guard
// ---------------------------------------------------------------------------------------

/// Why Phil will not offer the choice, or `None` when they will.
///
/// **A refusal is still an answer.** `CLAUDE.md`'s first expensive rule is that an unanswered
/// packet freezes the client's entire UI - every button, including the quit prompt - so every
/// arm returns words.
///
/// `jobs::refusal` does **not** fit: it returns `None` for template 101 because
/// `jobs::advancement_for` correctly calls Phil `NotAnInstructor`, and "not this module's
/// business" is a different answer from "no". So Phil owns their sentences, and reuses
/// [`jobs::LEVEL_MINIMUM`] rather than restating 10.
///
/// # Why the level gate is here and the stat gate is not
///
/// A beginner below [`jobs::LEVEL_MINIMUM`] who is routed anyway lands on a map with no portal
/// home, beside somebody who will refuse them - so the level check saves a stranding, and it is
/// the client's own gate: quest 10001's `Check.1.lvmin` is `10`. **[L]**
///
/// The **stat** requirement is deliberately not checked. It is `[I]` and unverifiable
/// (`jobs::STAT_MINIMUM`'s doc block), a player may reasonably want to travel first and spend
/// AP later, and refusing on it here would put the same policy in two places. Every offer line
/// names it instead.
pub fn refusal(chr: &Character) -> Option<String> {
    if chr.job != 0 {
        let named = jobs::FIRST_JOBS
            .iter()
            .find(|j| j.job == chr.job)
            .map(|j| format!("a {}", j.job_name))
            .unwrap_or_else(|| format!("job {}", chr.job));
        return Some(format!(
            "You are already {named}. A job advancement cannot be undone once made, so there is \
             nowhere left for me to send you."
        ));
    }
    if chr.level < jobs::LEVEL_MINIMUM {
        return Some(format!(
            "Come back when you have reached Level {}. You are only Level {}, and the \
             instructors will not even talk to you yet.",
            jobs::LEVEL_MINIMUM,
            chr.level
        ));
    }
    None
}

/// Whether this character gets the choice rather than a sentence.
pub fn offers_choice(chr: &Character) -> bool {
    refusal(chr).is_none()
}

// ---------------------------------------------------------------------------------------
// The step
// ---------------------------------------------------------------------------------------

/// What the session must do next. **Every arm is exhaustive and only one of them carries a
/// destination.**
///
/// That is the point of the type, and it comes straight out of `CLAUDE.md`'s Heena section:
/// *"Every effect hangs off the transition, not off the request."* The Heena quest paid out
/// twice per click because the payout sat outside the `match` on the store's answer. Here the
/// destination is **unreachable** unless every guard passed - there is no field on any other
/// arm to read a map id out of - so a warp cannot be performed beside a refusal by accident.
/// A caller would have to invent a map id to get it wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// Put the menu on screen and park `Conversation::path` at [`MENU_PATH`].
    Menu { text: String },
    /// **The choice is made and every guard passed.** Warp to `dest.map_id`; `dest` is the
    /// instructor waiting there. Nothing that assumed a refusal may precede this.
    Ride(&'static FirstJob),
    /// Say `text` on a plain OK box and drop the conversation. `why` is the log label.
    ///
    /// **A refusal is a reply.** `CLAUDE.md`'s *always answer*: this arm carries "you are
    /// already a Swordsman", "come back at Level 10" and "I cannot send you there", and none
    /// of them may be silence.
    Done { text: String, why: String },
    /// The player closed the box. **Send nothing** and drop the conversation - `0x00F3` is
    /// not one of the 37 setters of the client's one-request latch (`research/script-reply.md`
    /// section 5), so silence here is measured safe, and a "goodbye" box on every Escape
    /// would be noise.
    Closed,
}

/// Open the conversation: the refusal, or the menu.
///
/// **Never `None`.** Phil has no `d0` line, so falling through would print the "no dialogue for
/// template 101" placeholder, which is what they did before this existed.
pub fn opening(chr: &Character) -> Step {
    if let Some(text) = refusal(chr) {
        return Step::Done {
            text,
            why: format!(
                "Phil ({PHIL_TEMPLATE}) job guide NOT offered: character is job {} at level {}",
                chr.job, chr.level
            ),
        };
    }
    Step::Menu { text: menu_text() }
}

/// The player answered the menu: `selection` is the `#L` number picked, `None` when the box
/// was closed. The guard runs again on the pick - the box is modal but the socket is not.
pub fn on_pick(chr: &Character, selection: Option<u32>, fields: &HashSet<u32>) -> Step {
    let Some(selection) = selection else { return Step::Closed };
    let Some(dest) = destination(selection as usize) else {
        return Step::Done {
            text: "That is not one of the paths I can send you on.".to_string(),
            why: format!(
                "Phil job guide: the menu answered with selection {selection} and there are {} path(s) - a body off a socket, not the box we sent",
                jobs::FIRST_JOBS.len()
            ),
        };
    };
    board(chr, dest, fields)
}

/// Every guard, in one place. **This is the transition and it is the only thing that can
/// produce a [`Step::Ride`].**
///
/// The order matters: the character is re-checked *after* the pick, not only before the offer.
/// The box is modal but the socket is not, and nothing authenticates - a `!job`, a level-up or
/// a stale box from before a relog can change the answer between the offer and the click.
fn board(chr: &Character, dest: &'static FirstJob, fields: &HashSet<u32>) -> Step {
    if let Some(text) = refusal(chr) {
        return Step::Done {
            text,
            why: format!(
                "Phil job guide REFUSED a ride to {} ({}): character is job {} at level {}. NO TELEPORT",
                dest.map_id, dest.map_name, chr.job, chr.level
            ),
        };
    }
    if fields.is_empty() {
        return Step::Done {
            text: format!(
                "I cannot send you to {} right now - I could not check that the road is still \
                 there, and guessing would leave you nowhere.",
                dest.map_name
            ),
            why: format!(
                "Phil job guide REFUSED a ride to {}: the field table is empty, so the map could not be checked and a bad id would kill the client. Regenerate gm-handbook/fields.txt with tools/dump_portals.py, or restart the server so it loads. NO TELEPORT",
                dest.map_id
            ),
        };
    }
    if !fields.contains(&dest.map_id) {
        return Step::Done {
            text: format!(
                "I cannot send you to {} - that road is not on any map I have.",
                dest.map_name
            ),
            why: format!(
                "Phil job guide REFUSED a ride to {}: no field image in this client, so it would strand the character. NO TELEPORT",
                dest.map_id
            ),
        };
    }
    Step::Ride(dest)
}

/// The `0x055B` a step puts on screen.
///
/// This lives here rather than in the session so the packets are covered by this file's tests
/// instead of by a wiring patch nobody can compile in isolation - the smaller the part that
/// exists only as a diff in a report, the less of it can be wrong.
///
/// # [`Step::Ride`] deliberately produces NOTHING
///
/// **A script message must never travel with, or just before, a `SetField`.** Field entry runs
/// `FUN_142caa4e0`, which calls the script-manager reset `FUN_141f6f200` at `142caac7a`: the
/// dialog is built and then torn down, silently, with nothing in any log. `CLAUDE.md` records
/// that this has already cost a run. So a ride confirms itself with the new map and a chat
/// notice - which is not a script box - and says nothing through this function.
///
/// Not answering is safe here and that is measured, not hoped: `0x00F3` is **not** one of the
/// 37 setters of the client's one-request-outstanding latch `player->[0x2330]`, and the dialog
/// is destroyed and the latch released before the packet is even sent.
/// `research/script-reply.md` §5.1, §5.2. **[L]**
pub fn script_replies(step: &Step) -> Vec<crate::Reply> {
    match step {
        // Type 6, the list box the taxi measured on screen: one string, `#L` lines inside it.
        Step::Menu { text } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(PHIL_TEMPLATE, text),
            what: format!(
                "ScriptMessage MENU (type 6) from Phil ({PHIL_TEMPLATE}): {} selectable line(s), one per first job - {text:?}",
                jobs::FIRST_JOBS.len()
            ),
        }],
        // A plain OK box: no prev, no next, so the client draws `BtOK` and `BtClose` and the
        // conversation is over whichever the player presses.
        Step::Done { text, why } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(PHIL_TEMPLATE, text, false, false),
            what: format!("ScriptMessage Say from Phil ({PHIL_TEMPLATE}): {why}"),
        }],
        Step::Ride { .. } | Step::Closed => Vec::new(),
    }
}

// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// A beginner: job 0, at whatever level is asked for.
    fn beginner(level: u32) -> Character {
        Character {
            id: 200,
            name: "Tester".to_string(),
            level,
            job: 0,
            strength: 4,
            dexterity: 4,
            intelligence: 4,
            luck: 4,
            map_id: PHIL_MAP,
            ..Character::default()
        }
    }

    /// The field table as the server would have it, containing all four destinations.
    fn fields_with_all_destinations() -> HashSet<u32> {
        jobs::FIRST_JOBS.iter().map(|j| j.map_id).chain([PHIL_MAP]).collect()
    }

    fn pick(chr: &Character, index: u32) -> Step {
        on_pick(chr, Some(index), &fields_with_all_destinations())
    }

    /// The three numbers that decide whether any of this reaches the right NPC.
    #[test]
    fn phils_identity_is_the_one_the_client_data_gives() {
        assert_eq!(PHIL_TEMPLATE, 101, "gm-handbook/npcs.txt row for map 10000000");
        assert_eq!(PHIL_MAP, 10_000_000, "Lith Harbor");
        assert_eq!(PHIL_QUEST, 10_001, "Phil's Call - Check.0.npc and Check.1.npc are both 101");
        assert!(jobs::first_job_at(PHIL_TEMPLATE).is_none(), "Phil routes; they must not advance anybody");
        assert!(jobs::first_job_at(511).is_some());
    }

    /// **The menu is the table**, one `#L<n>#` line per first job, numbered by table index, the
    /// stat requirement on every line, no Pirate, and the token shape the client's own quest
    /// menu and the taxi use - which is what makes the lines selectable.
    #[test]
    fn the_menu_is_one_selectable_line_per_first_job() {
        let text = menu_text();
        let lines: Vec<&str> = text.split(LINE_BREAK).collect();
        assert_eq!(lines[0], HEADING);
        assert_eq!(lines[1], "", "the client's own blank line under the heading");
        assert_eq!(lines.len(), 2 + jobs::FIRST_JOBS.len());
        for (i, dest) in destinations() {
            let line = lines[2 + i];
            assert!(line.starts_with(&format!("#d#L{i}# ")), "line {i} is not selectable: {line}");
            assert!(line.ends_with("#l#k"), "{line}");
            assert!(line.contains(dest.job_name) && line.contains(dest.npc_name) && line.contains(dest.map_name), "{line}");
            assert!(line.contains(&format!("{} {}", jobs::STAT_MINIMUM, dest.stat.label())), "{line}");
        }
        assert!(!text.to_lowercase().contains("pirate"), "this client has no Pirate branch");
        assert!(!text.contains("#b> "), "the old marker is gone - a marker is not a cursor");
        assert!(text.is_ascii(), "PacketWriter::str truncates to one byte");
        assert!(!text.contains('\n') && !text.contains('\r'), "0x0A is not a line break to this client");
        assert_eq!(LINE_BREAK.as_bytes(), &[0x5C, 0x6E]);
    }

    /// A `Menu` is a **type 6** box from Phil, a `Done` is a Say, and a `Ride` and a `Closed`
    /// send no script at all - a script with or just before a `SetField` is torn down.
    #[test]
    fn each_step_produces_the_packet_it_should() {
        let out = script_replies(&Step::Menu { text: menu_text() });
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].opcode, net::script::SCRIPT_MESSAGE);
        assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_MENU, "type 6, the list box");
        assert_eq!(&out[0].body[5..9], &PHIL_TEMPLATE.to_le_bytes());
        assert_eq!(out[0].body, net::script::npc_menu(PHIL_TEMPLATE, &menu_text()));
        let out = script_replies(&Step::Done { text: "No.".to_string(), why: "because".to_string() });
        assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_SAY);
        assert!(script_replies(&Step::Ride(destination(0).unwrap())).is_empty());
        assert!(script_replies(&Step::Closed).is_empty());
    }

    /// The path is ours alone: not a quest's, not the taxi's, not the second-job menu's.
    #[test]
    fn the_conversation_path_is_ours_and_only_ours() {
        assert!(is_menu_path(MENU_PATH));
        for foreign in ["", "0", "0.yes", "0.no", "1", "taxi.0", "taxi.menu", "jobguide", "jobguide.0"] {
            assert!(!is_menu_path(foreign), "{foreign:?} is not Phil's menu");
        }
        assert_ne!(PATH_PREFIX, crate::taxi::PATH_PREFIX);
        assert_ne!(MENU_PATH, crate::taxi::MENU_PATH);
        assert!(!crate::taxi::is_taxi_path(MENU_PATH));
        assert!(!crate::secondjob::is_menu_path(MENU_PATH));
    }

    /// **Beginners only, and a non-Beginner gets words rather than silence.**
    #[test]
    fn a_non_beginner_is_refused_in_words_and_never_ridden() {
        for advanced in [100u16, 200, 300, 400, 110, 500] {
            let mut chr = beginner(50);
            chr.job = advanced;
            let no_text = refusal(&chr).unwrap_or_else(|| panic!("job {advanced} must refuse"));
            assert!(!offers_choice(&chr));
            match opening(&chr) {
                Step::Done { text, .. } => assert_eq!(text, no_text),
                other => panic!("job {advanced} opened with {other:?}"),
            }
            // The guard is honoured on the pick, not only on the offer.
            for index in 0..jobs::FIRST_JOBS.len() as u32 {
                match pick(&chr, index) {
                    Step::Done { text, .. } => assert_eq!(text, no_text),
                    other => panic!("job {advanced} pick {index} produced {other:?}"),
                }
            }
            assert_eq!(script_replies(&opening(&chr)).len(), 1);
        }
        let mut swordsman = beginner(50);
        swordsman.job = 100;
        assert!(refusal(&swordsman).unwrap().contains("Swordsman"));
    }

    /// Below the level minimum: "come back at ten", no menu, no ride. At it: the menu.
    #[test]
    fn a_beginner_below_level_ten_is_refused_and_not_ridden() {
        for level in 0..jobs::LEVEL_MINIMUM {
            let chr = beginner(level);
            let text = refusal(&chr).unwrap_or_else(|| panic!("level {level} must refuse"));
            assert!(text.contains(&jobs::LEVEL_MINIMUM.to_string()), "{text}");
            assert!(matches!(opening(&chr), Step::Done { .. }), "level {level}");
            assert!(matches!(pick(&chr, 0), Step::Done { .. }), "level {level}");
        }
        let ten = beginner(jobs::LEVEL_MINIMUM);
        assert_eq!(refusal(&ten), None);
        assert!(matches!(opening(&ten), Step::Menu { .. }));
        assert!(matches!(pick(&ten, 0), Step::Ride(_)));
    }

    /// **The stat is deliberately not a gate here.** The instructor refuses; Phil routes.
    #[test]
    fn phil_does_not_check_the_stat_the_instructor_checks() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        assert_eq!(chr.strength, 4);
        assert!(matches!(pick(&chr, 0), Step::Ride(_)), "Phil routes anyway");
        let dest = destination(0).unwrap();
        let text = jobs::refusal(&chr, dest.npc_template).expect("the instructor refuses");
        assert!(text.contains(dest.stat.label()) && text.contains("35"), "{text}");
    }

    /// Closing the box says nothing; a selection off the table says something and rides nowhere.
    #[test]
    fn a_closed_box_is_silent_and_a_bad_selection_is_answered() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        let fields = fields_with_all_destinations();
        assert_eq!(on_pick(&chr, None, &fields), Step::Closed);
        assert!(script_replies(&on_pick(&chr, None, &fields)).is_empty());
        for bad in [jobs::FIRST_JOBS.len() as u32, 99, u32::MAX] {
            match on_pick(&chr, Some(bad), &fields) {
                Step::Done { text, why } => {
                    assert!(!text.is_empty());
                    assert!(why.contains(&format!("selection {bad}")), "{why}");
                }
                other => panic!("selection {bad} produced {other:?}"),
            }
        }
    }

    /// Every pick rides to its own instructor, on the **instructor's** map - not the town.
    #[test]
    fn each_pick_rides_to_its_own_instructors_own_map() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        for (index, expected) in destinations() {
            match pick(&chr, index as u32) {
                Step::Ride(dest) => {
                    assert_eq!(dest.npc_template, expected.npc_template, "pick {index}");
                    assert_eq!(dest.map_id, expected.map_id);
                    assert_ne!(dest.map_id, dest.town_id, "{} stands one map inside {}", dest.npc_name, dest.town_id);
                    assert_ne!(dest.map_id, PHIL_MAP);
                }
                other => panic!("pick {index} produced {other:?}"),
            }
        }
        let maps: HashSet<u32> = jobs::FIRST_JOBS.iter().map(|j| j.map_id).collect();
        assert_eq!(maps.len(), jobs::FIRST_JOBS.len());
    }

    /// **Not weaker than `!map`'s guard.** An empty field table and a missing field image both
    /// refuse, for different stated reasons, and neither may ride.
    #[test]
    fn a_map_that_cannot_be_entered_refuses_and_does_not_ride() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        let empty = HashSet::new();
        match on_pick(&chr, Some(0), &empty) {
            Step::Done { why, .. } => {
                assert!(why.contains("field table is empty"), "{why}");
                assert!(why.contains("NO TELEPORT"), "{why}");
            }
            other => panic!("an empty field table must refuse, got {other:?}"),
        }
        let mut partial = fields_with_all_destinations();
        let missing = destination(1).unwrap();
        partial.remove(&missing.map_id);
        match on_pick(&chr, Some(1), &partial) {
            Step::Done { text, why } => {
                assert!(text.contains(missing.map_name), "{text}");
                assert!(why.contains("no field image"), "{why}");
            }
            other => panic!("a missing field image must refuse, got {other:?}"),
        }
        for index in [0u32, 2, 3] {
            assert!(matches!(on_pick(&chr, Some(index), &partial), Step::Ride(_)), "pick {index} must still ride");
        }
    }

    /// The arrival line names the map and the instructor, with the article right.
    #[test]
    fn the_arrival_line_names_the_instructor_and_the_map() {
        for (_, dest) in destinations() {
            let line = arrival_line(dest);
            assert!(line.contains(dest.map_name) && line.contains(dest.npc_name) && line.contains(dest.job_name), "{line}");
            assert!(line.is_ascii());
        }
        assert_eq!(article("Archer"), "an");
        assert_eq!(article("Swordsman"), "a");
    }
}
