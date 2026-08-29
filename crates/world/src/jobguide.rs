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
//! ## The one open question, recorded so it is neither lost nor acted on
//!
//! **`0x055B` message type 6 may be a third list shape that the count-then-strings sweep does
//! not match, and it is deliberately NOT used here.** What is measured:
//!
//! * its body is `[u32 spk if flags&4], str` - **one** string, two reads and no others
//!   (`141f73789`, `141f73799`) - so a sweep looking for `u8 count` + N strings cannot see it;
//! * it builds dialog kind **6** (`141f738ba  FUN_142a61900(ui, 6, speaker, &text)`);
//! * its `0x00F3` **does** carry a value: `u32 0, u8 6, u8 1, u32 sel` on accept and
//!   `u32 0, u8 6, u8 0` on cancel, where `sel = FUN_142a64400(ui) = [ui+0x3d0]`
//!   (`141f7395c`..`141f739a4`);
//! * `CUIScriptMsg::OnButtonClicked` fills `[ui+0x3d0]` from `entry->[0x40]` of
//!   `[ui+0x3e0][ [ui+0x3d8] ]` - an array of entries each carrying its own id, with a
//!   highlighted-line cursor `FUN_142a591f6` decrements at `142a596f9` (`142a5a2a9`/`142a5a2ac`);
//! * and `research/npc-click.md` §2.1, **already in the repo**, records the client's own NPC
//!   quest menu doing exactly this: `141e3db1b FUN_142a61900(ui, 6, npcTemplateId, &text)` with
//!   a text built from the literal `#d#L%d# %s#l#k`, then `141e3dba6 sel = FUN_142a64400(ui)`
//!   and `141e3dbd8 q = arr[sel]`. One string in, a pick out.
//!
//! What is **not** measured, and why this is not built on: the fill site of `[ui+0x3e0]` was not
//! isolated - `FUN_142a61900` *releases and clears* it at `142a619f4..142a61a0b`, and whatever
//! repopulates it from the `#`-token expander later in the same function was not read. So "a
//! server-sent type 6 renders `#L` lines" is **[D]**, and **nobody has ever watched one draw**.
//! The yes/no chain is [L] on the owner's screen. Preferring the proven mechanism is the whole of
//! `CLAUDE.md`'s client-run budget rule; this note exists so the finding is not re-derived from
//! scratch, because settling it costs one `world.log` line on any launch that also sends a
//! type 6, and the payoff is a one-box menu instead of a four-box chain.

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

/// The prefix that marks `Conversation::path` as Phil's job guide.
///
/// **A quest path is `"0"`, `"0.yes"` or `"0.no"`** and a plain talk is `""`, so nothing the
/// existing state machine produces can collide with this - and it cannot collide with
/// [`crate::taxi::PATH_PREFIX`] either, which is the other module doing the same thing.
///
/// Riding in `path` is what lets this hold state without a new field on `Session`, which
/// matters twice over: `crates/world/src/session/mod.rs` belongs to the coordinator, and
/// `research/script-reply.md` §4 establishes there is nowhere else to put it - **a yes/no box
/// echoes neither the handle nor the text** (`141f70a50` writes a literal `0`), so the server
/// has no correlation token on the wire and must key the conversation on the session. **[L]**
pub const PATH_PREFIX: &str = "jobguide.";

/// The conversation path for "currently offering path `index`".
pub fn path(index: usize) -> String {
    format!("{PATH_PREFIX}{index}")
}

/// Which job path a conversation is offering, or `None` if this is not Phil's conversation.
pub fn offer_index(path: &str) -> Option<usize> {
    path.strip_prefix(PATH_PREFIX)?.parse().ok()
}

// ---------------------------------------------------------------------------------------
// The words
// ---------------------------------------------------------------------------------------

/// The line above the list on every box.
pub const HEADING: &str =
    "So you've made up your mind? Good. I can send you straight to the one who'll take you on.";

/// The whole list, with the offer at `index` marked, followed by the question.
///
/// **Every box carries all four**, which is what makes a chain of yes/no prompts read as a
/// menu: the player can see what they are declining towards instead of answering four
/// unrelated questions.
///
/// The stat requirement is on each line because Phil does **not** check it - they route, the
/// instructor refuses - so a player who cannot yet qualify can see that before spending the
/// trip.
pub fn offer_text(index: usize) -> Option<String> {
    let current = destination(index)?;
    let mut out = String::from(HEADING);
    out.push_str(LINE_BREAK);
    out.push_str(LINE_BREAK);
    for (i, dest) in destinations() {
        // `#b` ... `#k` on the line being offered. Safe on this path: `FUN_142a61900` reaches
        // the `#`-token expander, and this client's own Heena `d0` ships `#b...#k` through the
        // same packet. **[L]**
        if i == index {
            out.push_str("#b> ");
        } else {
            out.push_str("   ");
        }
        out.push_str(&format!(
            "{} - {} in {} (needs {} {})",
            dest.job_name,
            dest.npc_name,
            dest.map_name,
            jobs::STAT_MINIMUM,
            dest.stat.label(),
        ));
        if i == index {
            out.push_str("#k");
        }
        out.push_str(LINE_BREAK);
    }
    out.push_str(LINE_BREAK);
    out.push_str(&format!(
        "Shall I send you to {} in {}?",
        current.npc_name, current.map_name
    ));
    Some(out)
}

/// What Phil says when the last path has been declined.
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
    /// Send `text` as another yes/no box from Phil and set `Conversation::path` to `path`.
    /// The conversation stays open.
    Ask { text: String, path: String },
    /// **The choice is made and every guard passed.** Warp to `dest.map_id`; `dest` is the
    /// instructor waiting there. Nothing that assumed a refusal may precede this.
    Ride(&'static FirstJob),
    /// Say `text` on a plain OK box and drop the conversation. `why` is the log label.
    ///
    /// **A refusal is a reply.** `CLAUDE.md`'s *always answer*: this arm carries "you are
    /// already a Swordsman", "come back at Level 10" and "I cannot send you there", and none
    /// of them may be silence.
    Done { text: String, why: String },
}

/// Open the conversation: the refusal, or the first offer.
///
/// **Never `None`.** Phil has no `d0` line, so falling through would print the "no dialogue for
/// template 101" placeholder, which is what they do today.
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
    match offer_text(0) {
        Some(text) => Step::Ask { text, path: path(0) },
        // Unreachable with the shipped table, but a yes/no box offering nothing is a dead end
        // on screen and this says so instead.
        None => Step::Done {
            text: farewell(),
            why: "Phil job guide: jobs::FIRST_JOBS is empty, so there is nothing to offer"
                .to_string(),
        },
    }
}

/// The player answered one of Phil's yes/no boxes.
///
/// `action` is `research/script-reply.md`'s signed answer byte: **`1` Yes, `0` No**. `-1` (the
/// box was closed) never reaches here - `Session::on_script_reply` consumes it before any of
/// this, drops the conversation and sends nothing, which is safe because `0x00F3` is not one of
/// the 37 setters of the client's one-request-outstanding latch. **[L]** Any other value ends
/// the conversation politely rather than silently.
///
/// `fields` is `Config::fields` - the map ids with a field image, from `gm-handbook/fields.txt`.
///
/// # The map guard is not weaker than `!map`'s
///
/// `Session::gm_map` refuses twice: once when the id is not in the table, and once when the
/// table is **empty**, because `Config::map_exists` is fail-open and an unchecked bad id kills
/// the client - the owner lost a session to `!map 45` on 2026-08-20. Both are reproduced below, with
/// different sentences so the reason is visible rather than merged.
///
/// # Wire it like this
///
/// In `crates/world/src/session/npc.rs`, in `on_script_reply`, after the `SCRIPT_ACTION_CLOSED`
/// arm and **before** the `convo.awaiting_yes_no` block:
///
/// ```text
/// if let Some(index) = crate::jobguide::offer_index(&convo.path) {
///     return self.jobguide_reply(&convo, index, reply.action);
/// }
/// ```
///
/// Before that block is not a style preference, and it is the same hazard the taxi hits:
/// `awaiting_yes_no` is true on Phil's box too, so the quest branch would otherwise take it -
/// `accept_quest` returns nothing for a conversation with no quest id, `has_branch` is false,
/// and the conversation would be dropped with **no packet sent**. Phil would go silent on the
/// first Yes.
pub fn on_reply(chr: &Character, index: usize, action: i8, fields: &HashSet<u32>) -> Step {
    let Some(dest) = destination(index) else {
        return Step::Done {
            text: farewell(),
            why: format!(
                "Phil job guide: the conversation was on offer {index} and there are {} path(s)",
                jobs::FIRST_JOBS.len()
            ),
        };
    };
    match action {
        net::script::SCRIPT_ACTION_YES => board(chr, dest, fields),
        net::script::SCRIPT_ACTION_NO => match offer_text(index + 1) {
            Some(text) => Step::Ask { text, path: path(index + 1) },
            None => Step::Done {
                text: farewell(),
                why: format!(
                    "Phil job guide: declined the last of {} path(s)",
                    jobs::FIRST_JOBS.len()
                ),
            },
        },
        other => Step::Done {
            text: farewell(),
            why: format!(
                "Phil job guide: answer byte {other} is neither Yes (1) nor No (0); ending the \
                 conversation with a line rather than silence"
            ),
        },
    }
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
        // Type **3**, not `0x10`. Both draw a yes/no pair whose answer byte is unambiguous -
        // `1` Yes, `0` No, `-1` closed - but `0x10` wears the quest captions `BtQYes`/`BtQNo`
        // (`142a660b4`), and choosing a career is not a quest. **[L]**
        Step::Ask { text, .. } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_ask(PHIL_TEMPLATE, text, false),
            what: format!(
                "ScriptMessage yes/no (type 3, BtYes/BtNo) from Phil ({PHIL_TEMPLATE}): {text:?}"
            ),
        }],
        // A plain OK box: no prev, no next, so the client draws `BtOK` and `BtClose` and the
        // conversation is over whichever the player presses.
        Step::Done { text, why } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(PHIL_TEMPLATE, text, false, false),
            what: format!("ScriptMessage Say from Phil ({PHIL_TEMPLATE}): {why}"),
        }],
        Step::Ride { .. } => Vec::new(),
    }
}

// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// A beginner: job 0, at whatever level is asked for.
    fn beginner(level: u32) -> Character {
        Character {
            // 200 is FIRST_CHARACTER_ID. A record renumbered from 1 has bitten this project
            // before, so the fixtures here use a real id rather than a small one.
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

    fn yes(chr: &Character, index: usize) -> Step {
        on_reply(chr, index, net::script::SCRIPT_ACTION_YES, &fields_with_all_destinations())
    }

    fn no(chr: &Character, index: usize) -> Step {
        on_reply(chr, index, net::script::SCRIPT_ACTION_NO, &fields_with_all_destinations())
    }

    // -- effect 1: who Phil is -----------------------------------------------------------

    /// The three numbers that decide whether any of this reaches the right NPC. If a later
    /// edit moves Phil, this fails here rather than on the owner's screen.
    #[test]
    fn phils_identity_is_the_one_the_client_data_gives() {
        assert_eq!(PHIL_TEMPLATE, 101, "gm-handbook/npcs.txt row for map 10000000");
        assert_eq!(PHIL_MAP, 10_000_000, "Lith Harbor");
        assert_eq!(PHIL_QUEST, 10_001, "Phil's Call - Check.0.npc and Check.1.npc are both 101");
        // Phil is not an instructor and must never be mistaken for one.
        assert!(
            jobs::first_job_at(PHIL_TEMPLATE).is_none(),
            "Phil routes; they must not advance anybody"
        );
        // The control, so the line above is not a property of a broken lookup.
        assert!(jobs::first_job_at(511).is_some());
    }

    // -- effect 2: the offers are the table ----------------------------------------------

    /// **The offers are the table.** Derived, never hand-typed - so the absence of a Pirate
    /// branch (`crate::firstjob`'s header, [L]) cannot be undone by an edit here.
    #[test]
    fn the_offers_are_exactly_the_first_job_table_and_nothing_else() {
        let listed: Vec<(usize, u16)> = destinations().map(|(i, j)| (i, j.job)).collect();
        assert_eq!(listed, vec![(0, 100), (1, 200), (2, 300), (3, 400)]);
        assert_eq!(listed.len(), jobs::FIRST_JOBS.len());

        // There is a box for every row and no box after the last one.
        for (i, _) in destinations() {
            assert!(offer_text(i).is_some(), "offer {i} must exist");
        }
        assert!(
            offer_text(jobs::FIRST_JOBS.len()).is_none(),
            "a fifth offer would be a choice nobody can advance"
        );

        // Every box carries the whole list, so a chain reads as a menu.
        let text = offer_text(0).unwrap();
        for (_, dest) in destinations() {
            assert!(text.contains(dest.job_name), "{} missing from the box", dest.job_name);
            assert!(text.contains(dest.npc_name), "{} missing", dest.npc_name);
            assert!(text.contains(dest.map_name), "{} missing", dest.map_name);
        }
        assert!(!text.to_lowercase().contains("pirate"), "this client has no Pirate branch");
    }

    /// Exactly one line is marked, it is the one being offered, and the question names that
    /// instructor. A box that marks the wrong line asks about one job and rides to another.
    #[test]
    fn each_box_marks_its_own_offer_and_asks_about_it() {
        for (i, dest) in destinations() {
            let text = offer_text(i).unwrap();
            assert_eq!(text.matches("#b> ").count(), 1, "offer {i}: exactly one marked line");
            assert_eq!(text.matches("#k").count(), 1, "offer {i}: exactly one close token");
            // The marked line is this destination's.
            let marked = text
                .split(LINE_BREAK)
                .find(|l| l.starts_with("#b> "))
                .unwrap_or_else(|| panic!("offer {i} has no marked line"));
            assert!(marked.contains(dest.job_name), "offer {i} marks the wrong line: {marked}");
            assert!(marked.contains(dest.npc_name), "{marked}");
            // ...and so does the question at the bottom.
            let question = text.rsplit(LINE_BREAK).next().unwrap();
            assert!(question.contains(dest.npc_name), "offer {i} asks about: {question}");
            assert!(question.contains(dest.map_name), "{question}");
            // The stat requirement is on every line, because Phil does not enforce it.
            assert_eq!(
                text.matches(&jobs::STAT_MINIMUM.to_string()).count(),
                jobs::FIRST_JOBS.len()
            );
        }
    }

    /// **`\n` is two characters, and `0x0A` is not a line break to this client.** Getting this
    /// wrong runs the whole list onto one line.
    #[test]
    fn the_line_break_is_a_backslash_and_an_n_not_a_newline_byte() {
        assert_eq!(LINE_BREAK.as_bytes(), &[0x5C, 0x6E], "142a45b80 cmp byte [rax], 0x5c");
        assert_eq!(LINE_BREAK.len(), 2);
        for (i, _) in destinations() {
            let text = offer_text(i).unwrap();
            assert!(!text.contains('\n'), "offer {i} contains a real 0x0A: {text:?}");
            assert!(!text.contains('\r'), "offer {i} contains a real 0x0D");
            // Heading, four rows, a blank line before the question: six breaks.
            assert_eq!(text.matches(LINE_BREAK).count(), 2 + jobs::FIRST_JOBS.len() + 1);
        }
        // **The text must be ASCII.** `net::packet::PacketWriter::str` writes
        // `s.chars().map(|c| c as u8)` - one byte per char, truncated to the low byte - so a
        // curly apostrophe would reach the client as a different character with no error.
        assert!(offer_text(0).unwrap().is_ascii(), "PacketWriter::str truncates to one byte");
        assert!(farewell().is_ascii());
        assert!(HEADING.is_ascii());
    }

    // -- effect 3: the packets -----------------------------------------------------------

    /// An `Ask` is a **type 3** box, a `Done` is a Say, and a `Ride` sends no script at all.
    /// That last one is not an omission: a script with or just before a `SetField` is torn
    /// down silently by field entry.
    #[test]
    fn each_step_produces_the_packet_it_should_and_a_ride_produces_none() {
        let ask = Step::Ask { text: "Well?".to_string(), path: path(0) };
        let out = script_replies(&ask);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].opcode, net::script::SCRIPT_MESSAGE);
        // Offset 10 is the message type: 3, the plain BtYes/BtNo pair.
        assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_YES_NO, "type 3, not 0 and not 0x10");
        assert_ne!(
            out[0].body[10],
            net::script::SCRIPT_TYPE_QUEST_YES_NO,
            "0x10 draws BtQYes/BtQNo, which is a quest's captions and not a career choice"
        );
        // ...and the speaker at offset 5 is Phil, because a bad template costs the portrait.
        assert_eq!(&out[0].body[5..9], &PHIL_TEMPLATE.to_le_bytes());

        let done = Step::Done { text: "No.".to_string(), why: "because".to_string() };
        let out = script_replies(&done);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_SAY, "a Done is a plain OK box");
        assert_eq!(&out[0].body[5..9], &PHIL_TEMPLATE.to_le_bytes());

        assert!(
            script_replies(&Step::Ride(destination(0).unwrap())).is_empty(),
            "a script must never travel with a SetField - FUN_142caa4e0 tears it down"
        );
    }

    /// The real box bytes for every offer, so a text change cannot silently produce a body the
    /// client reads short.
    #[test]
    fn every_offer_builds_a_body_the_client_can_read() {
        for (i, _) in destinations() {
            let text = offer_text(i).unwrap();
            let Step::Ask { text: sent, .. } = on_reply_open(i) else { panic!("offer {i}") };
            assert_eq!(sent, text);
            let body = net::script::npc_ask(PHIL_TEMPLATE, &text, false);
            // head 14 bytes, then u16 byte count, then the text.
            assert_eq!(body.len(), net::script::SCRIPT_HEAD_LEN + 2 + text.len());
            assert_eq!(
                &body[14..16],
                &u16::try_from(text.len()).unwrap().to_le_bytes(),
                "u16 BYTE count at 141f70879"
            );
            assert_eq!(&body[16..], text.as_bytes());
            assert_eq!(body[9], 0, "hasOverride - 0 means no u32 follows");
            assert_eq!(&body[11..13], &0u16.to_le_bytes(), "flags - bit 4 would add a u32");
        }
    }

    /// Helper: the `Ask` a given offer index would send, without going through an answer.
    fn on_reply_open(index: usize) -> Step {
        match offer_text(index) {
            Some(text) => Step::Ask { text, path: path(index) },
            None => Step::Done { text: farewell(), why: String::new() },
        }
    }

    // -- effect 4: the conversation's place ----------------------------------------------

    /// The path round-trips, and **nothing the existing state machine produces looks like
    /// ours** - a quest path is `"0"` / `"0.yes"` / `"0.no"` and a plain talk is `""`.
    #[test]
    fn the_conversation_path_is_ours_and_only_ours() {
        for i in 0..jobs::FIRST_JOBS.len() {
            assert_eq!(offer_index(&path(i)), Some(i));
        }
        for foreign in ["", "0", "0.yes", "0.no", "1", "1.yes", "taxi.0", "taxi.3", "jobguide"] {
            assert_eq!(offer_index(foreign), None, "{foreign:?} is not Phil's");
        }
        // And it cannot collide with the other module doing the same trick.
        //
        // `taxi` no longer has per-offer paths - it parks ONE conversation at `MENU_PATH`,
        // because its dialogue was rebuilt as a single type-6 menu box while this one stayed
        // a yes/no chain. So the collision check is against that constant instead of against
        // a per-index path, and the intent is unchanged: neither module may claim the other's
        // conversation.
        assert_ne!(PATH_PREFIX, crate::taxi::PATH_PREFIX);
        assert!(offer_index(crate::taxi::MENU_PATH).is_none());
        assert!(!crate::taxi::is_taxi_path(&path(0)));
    }

    // -- effect 5: the refusals ----------------------------------------------------------

    /// **Beginners only, and a non-Beginner gets words rather than silence.** `CLAUDE.md`'s
    /// "always answer": an unanswered packet freezes the client's whole UI.
    #[test]
    fn a_non_beginner_is_refused_in_words_and_never_ridden() {
        for advanced in [100u16, 200, 300, 400, 110, 500] {
            let mut chr = beginner(50);
            chr.job = advanced;
            let no_text = refusal(&chr).unwrap_or_else(|| panic!("job {advanced} must refuse"));
            assert!(!no_text.is_empty());
            assert!(!offers_choice(&chr));
            // The opening is a sentence, not an offer.
            match opening(&chr) {
                Step::Done { text, .. } => assert_eq!(text, no_text),
                other => panic!("job {advanced} opened with {other:?}"),
            }
            // And the guard is honoured on the transition, not only on the offer: a stale box
            // from before a `!job` must not still ride.
            for index in 0..jobs::FIRST_JOBS.len() {
                match yes(&chr, index) {
                    Step::Done { text, .. } => assert_eq!(text, no_text),
                    other => panic!("job {advanced} offer {index} produced {other:?}"),
                }
            }
            // Every refusal reaches the screen as a packet.
            assert_eq!(script_replies(&opening(&chr)).len(), 1);
        }
        // A first-job character is told which job they have, by name.
        let mut swordsman = beginner(50);
        swordsman.job = 100;
        assert!(refusal(&swordsman).unwrap().contains("Swordsman"));
    }

    /// Below [`jobs::LEVEL_MINIMUM`] the answer is "come back at ten", and no ride happens -
    /// routing a level-3 beginner to the Warriors' Sanctuary strands them beside somebody who
    /// will refuse them anyway.
    #[test]
    fn a_beginner_below_level_ten_is_refused_and_not_ridden() {
        for level in 0..jobs::LEVEL_MINIMUM {
            let chr = beginner(level);
            let text = refusal(&chr).unwrap_or_else(|| panic!("level {level} must refuse"));
            assert!(text.contains(&jobs::LEVEL_MINIMUM.to_string()), "{text}");
            assert!(matches!(opening(&chr), Step::Done { .. }), "level {level}");
            assert!(matches!(yes(&chr, 0), Step::Done { .. }), "level {level}");
        }
        // Exactly at the boundary the complaint stops and the offer appears.
        let ten = beginner(jobs::LEVEL_MINIMUM);
        assert_eq!(refusal(&ten), None);
        assert!(matches!(opening(&ten), Step::Ask { .. }));
        assert!(matches!(yes(&ten, 0), Step::Ride(_)));
    }

    /// **The stat is deliberately not a gate here.** A level-10 beginner with 4 in everything
    /// is still routed - the instructor refuses, which is where that policy lives.
    #[test]
    fn phil_does_not_check_the_stat_the_instructor_checks() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        assert_eq!(chr.strength, 4);
        assert!(matches!(yes(&chr, 0), Step::Ride(_)), "Phil routes anyway");
        // And the instructor is the one who says no, with the sentence that names the stat.
        let dest = destination(0).unwrap();
        let text = jobs::refusal(&chr, dest.npc_template).expect("the instructor refuses");
        assert!(text.contains(dest.stat.label()) && text.contains("35"), "{text}");
    }

    // -- effect 6: the chain -------------------------------------------------------------

    /// **No walks to the next offer, and the last No ends with a line rather than silence.**
    #[test]
    fn no_walks_the_chain_and_the_last_no_says_goodbye() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        let last = jobs::FIRST_JOBS.len() - 1;
        for index in 0..last {
            match no(&chr, index) {
                Step::Ask { text, path: p } => {
                    assert_eq!(offer_index(&p), Some(index + 1), "No must advance the path");
                    assert_eq!(text, offer_text(index + 1).unwrap());
                }
                other => panic!("No on offer {index} produced {other:?}"),
            }
        }
        match no(&chr, last) {
            Step::Done { text, .. } => assert_eq!(text, farewell()),
            other => panic!("the last No produced {other:?}"),
        }
        // Declining everything must never ride.
        for index in 0..jobs::FIRST_JOBS.len() {
            assert!(!matches!(no(&chr, index), Step::Ride(_)), "No rode at offer {index}");
        }
    }

    /// An answer byte that is neither Yes nor No ends the conversation with a line. The client
    /// only sends `1`, `0` and `-1`, and `-1` is consumed before this - but a body off a socket
    /// is not a promise, and silence is the one outcome that reads as a crash.
    #[test]
    fn an_unexpected_answer_byte_still_says_something() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        let fields = fields_with_all_destinations();
        for action in [2i8, 7, -3, i8::MIN, i8::MAX] {
            match on_reply(&chr, 0, action, &fields) {
                Step::Done { text, .. } => assert!(!text.is_empty(), "action {action}"),
                other => panic!("action {action} produced {other:?}"),
            }
        }
        // A path pointing past the table is the same shape of problem.
        match on_reply(&chr, 99, net::script::SCRIPT_ACTION_YES, &fields) {
            Step::Done { text, .. } => assert!(!text.is_empty()),
            other => panic!("offer 99 produced {other:?}"),
        }
    }

    // -- effect 7: the ride --------------------------------------------------------------

    /// Every offer rides to its own instructor, on the **instructor's** map - not the town.
    /// Getting this wrong costs one of the owner's manual launches and leaves them standing in a
    /// town with nobody in it.
    #[test]
    fn each_yes_rides_to_its_own_instructors_own_map() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        for (index, expected) in destinations() {
            match yes(&chr, index) {
                Step::Ride(dest) => {
                    assert_eq!(dest.npc_template, expected.npc_template, "offer {index}");
                    assert_eq!(dest.map_id, expected.map_id);
                    assert_ne!(
                        dest.map_id, dest.town_id,
                        "{} stands one map inside {}, not in it",
                        dest.npc_name, dest.town_id
                    );
                    assert_ne!(dest.map_id, PHIL_MAP, "the ride has to go somewhere");
                }
                other => panic!("offer {index} produced {other:?}"),
            }
        }
        // The four are distinct maps - a table edit that collapsed two would send half the
        // chain to the wrong instructor and every other test here would still pass.
        let maps: HashSet<u32> = jobs::FIRST_JOBS.iter().map(|j| j.map_id).collect();
        assert_eq!(maps.len(), jobs::FIRST_JOBS.len());
    }

    // -- effect 8: the map guard ---------------------------------------------------------

    /// **Not weaker than `!map`'s guard.** `Session::gm_map` refuses an id with no field image
    /// *and* refuses when the table is empty, because `Config::map_exists` is fail-open and
    /// The owner lost a session to `!map 45` on 2026-08-20. Both are reproduced, and neither may
    /// produce a ride.
    #[test]
    fn a_map_that_cannot_be_entered_refuses_and_does_not_ride() {
        let chr = beginner(jobs::LEVEL_MINIMUM);

        // (a) the table is empty - "could not be checked", which is NOT the same reason as
        //     "does not exist", and merging them would send the next reader after a bug that
        //     is not there.
        let empty = HashSet::new();
        match on_reply(&chr, 0, net::script::SCRIPT_ACTION_YES, &empty) {
            Step::Done { text, why } => {
                assert!(!text.is_empty());
                assert!(why.contains("field table is empty"), "{why}");
                assert!(why.contains("NO TELEPORT"), "{why}");
            }
            other => panic!("an empty field table must refuse, got {other:?}"),
        }

        // (b) the table is loaded but this destination is missing from it.
        let mut partial = fields_with_all_destinations();
        let missing = destination(1).unwrap();
        partial.remove(&missing.map_id);
        match on_reply(&chr, 1, net::script::SCRIPT_ACTION_YES, &partial) {
            Step::Done { text, why } => {
                assert!(text.contains(missing.map_name), "{text}");
                assert!(why.contains("no field image"), "{why}");
                assert!(!why.contains("field table is empty"), "wrong reason: {why}");
            }
            other => panic!("a missing field image must refuse, got {other:?}"),
        }
        // The control: the other three still ride, so (b) is a property of the missing map and
        // not of a guard that refuses everything.
        for index in [0usize, 2, 3] {
            assert!(matches!(
                on_reply(&chr, index, net::script::SCRIPT_ACTION_YES, &partial),
                Step::Ride(_)
            ));
        }
    }

    // -- effect 9: arriving in a state where the instructor works ------------------------

    /// **The ride has to land somewhere the existing advancement path works**, or the feature
    /// is a teleport to nobody. This is the effect a test of the warp alone would miss.
    ///
    /// **It walks `on_reply`, not the table, and that is not a detail.** An earlier draft
    /// iterated `destinations()` directly, and an injected "route to the town the WZ text
    /// names" bug left it green while five of its neighbours went red - a test of the offer
    /// that could not see the ride. It now asks the router the same question the wire asks it.
    #[test]
    fn the_character_arrives_where_advance_job_for_will_work() {
        let traveller = beginner(jobs::LEVEL_MINIMUM);
        for (index, promised) in destinations() {
            let Step::Ride(dest) = yes(&traveller, index) else {
                panic!("offer {index} did not produce a ride")
            };
            assert_eq!(dest.job, promised.job, "the ride matches the box that was offered");
            // The destination NPC is the instructor for this branch - not a second-job examiner.
            let at = jobs::first_job_at(dest.npc_template)
                .unwrap_or_else(|| panic!("offer {index} lands on a non-instructor"));
            assert_eq!(
                at.map_id, dest.map_id,
                "offer {index} must land on the map the instructor actually stands on, not on \
                 the town the WZ text names"
            );

            let mut arrived = beginner(jobs::LEVEL_MINIMUM);
            arrived.map_id = dest.map_id;
            match dest.stat {
                jobs::Stat::Strength => arrived.strength = jobs::STAT_MINIMUM,
                jobs::Stat::Dexterity => arrived.dexterity = jobs::STAT_MINIMUM,
                jobs::Stat::Intelligence => arrived.intelligence = jobs::STAT_MINIMUM,
                jobs::Stat::Luck => arrived.luck = jobs::STAT_MINIMUM,
            }
            assert_eq!(
                jobs::advancement_for(&arrived, dest.npc_template),
                jobs::Advancement::Eligible { job: dest.job, job_name: dest.job_name },
                "offer {index} must arrive able to advance"
            );
        }
    }

    /// **Phil never advances anybody.** Requirement 4: no path through this module produces a
    /// job, and Phil's own template is not in the instructor table, so `advance_job_for(101)`
    /// falls through to whatever an ordinary NPC does.
    #[test]
    fn phil_routes_and_the_instructor_advances() {
        let chr = beginner(jobs::LEVEL_MINIMUM);
        assert_eq!(jobs::advancement_for(&chr, PHIL_TEMPLATE), jobs::Advancement::NotAnInstructor);
        assert_eq!(jobs::refusal(&chr, PHIL_TEMPLATE), None, "Phil is not the instructor's job");
        // `on_reply` takes `&Character`, so it cannot change one; what this pins is that the
        // value it hands back carries a *destination* and never a job to write.
        let before = chr.job;
        let Step::Ride(dest) = yes(&chr, 0) else { panic!("expected a ride") };
        assert_eq!(chr.job, before, "Phil changed nobody's job");
        assert_eq!(dest.job, 100, "the job is the instructor's to grant, and it is 100 there");
    }

    // -- the words a player reads --------------------------------------------------------

    /// Every outcome has something to say, and the arrival line names who to look for.
    #[test]
    fn every_outcome_has_words_and_the_arrival_names_who_to_find() {
        for (_, dest) in destinations() {
            let line = arrival_line(dest);
            assert!(line.contains(dest.npc_name), "{line}");
            assert!(line.contains(dest.map_name), "{line}");
            assert!(line.contains(dest.job_name), "{line}");
            assert!(line.is_ascii());
        }
        assert!(arrival_line(destination(2).unwrap()).contains("an Archer"), "Archer takes 'an'");
        assert!(arrival_line(destination(0).unwrap()).contains("a Swordsman"));
        assert!(!farewell().is_empty());
        // No step is ever silent except a Ride, which is silent on purpose.
        let chr = beginner(jobs::LEVEL_MINIMUM);
        for step in [opening(&chr), no(&chr, 3), yes(&beginner(1), 0)] {
            assert_eq!(script_replies(&step).len(), 1, "{step:?} must reach the screen");
        }
    }
}
