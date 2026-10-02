//! **Citizenship in a session** - the Town Clerks' contracts, the quest gates, the payout, the
//! Community Board's postings and the town-hall shops' grade locks.
//!
//! The rules are `crate::citizenship`'s; the wire is `net::citizenship`'s; this file only joins
//! them to a character. `research/citizenship-2026-09-27.md` has the client side.
//!
//! # One conversation per contract, two answers
//!
//! A contract window answers **twice** when accepted: OK (`type, 1`), and later the stamp's end
//! (`0x47, type`), which closes it. The first is acted on; the second is swallowed. Both are
//! recognised only while this session has a contract parked ([`CONTRACT_PATH`] /
//! [`STAMP_PATH`]), and the offer is **recomputed at the answer** from the database rather than
//! trusted from when the window opened.
//!
//! **OK does not play the stamp - the SERVER does.** The owner, 2026-09-29, first client run:
//! *"when I click on Sign, the contract never went away."* The contract was signed (the log has
//! it) and the window sat there with its buttons greyed, forever. OK only disables the buttons
//! and sets `+0x320`; the stamp is started by `FUN_1410DEA80`, whose one caller is the `0x055B`
//! reader's **force-close branch** (`0x141F6F486`: type `0x47`, then `u8 == 1`). So after
//! acting on OK the server sends `net::script::script_force_close(1)`: the stamp plays, and 2 s
//! after it ends `FUN_1410DE710` sends the `0x47` answer and closes the window. A refused
//! contract sends `script_force_close(0)` instead - the window closes at once, no stamp.
//!
//! Nothing here authenticates: a contract is signed for whoever holds the socket.

use super::{Conversation, Reply, Session};
use crate::citizenship::{self as cz, ClerkOffer};
use net::citizenship::{Contract, ContractAnswer};
use store::citizenship::TownStanding;

/// The clerk's menu for a citizen of their own town: standing, or renounce.
pub(super) const MENU_PATH: &str = "citizenship.menu";
/// A contract window is open and unanswered.
pub(super) const CONTRACT_PATH: &str = "citizenship.contract";
/// A contract was accepted; its stamp-finished answer is still to come.
pub(super) const STAMP_PATH: &str = "citizenship.stamp";
/// Head field 1 of every contract window. The client echoes it and compares it to nothing.
const CONTRACT_HANDLE: u32 = 0x0000_C17E;

/// What each offer opens.
fn contract_for(town: u8, offer: ClerkOffer) -> Option<Contract> {
    Some(match offer {
        ClerkOffer::Oath => Contract::Oath { town },
        ClerkOffer::Transfer { old_town } => Contract::Transfer { new_town: town, old_town },
        ClerkOffer::Reactivation { grade } => Contract::Reactivation { town, grade, fee: cz::REACTIVATION_FEE },
        ClerkOffer::Renunciation { grade } => Contract::Renunciation { town, grade },
        ClerkOffer::Certificate { grade } => Contract::GradeUpdate { town, grade },
        ClerkOffer::TooLow => return None,
    })
}

/// The unix time the board and the repeat rules are judged at: the start of today, UTC. Both
/// are day-granular, and this keeps `crates/world` on the one clock `store::dailyperks` owns.
fn board_now() -> i64 {
    store::dailyperks::utc_day_start(store::dailyperks::today())
}

impl Session {
    fn standings(&self, character_id: u32) -> Vec<TownStanding> {
        self.store.citizenship(character_id).unwrap_or_default()
    }

    /// Block #28's entries for a `SetField`: quest 510000 (when the character ever signed) and
    /// the four board postings. Remembers the postings, so [`Session::board_tick`] can tell
    /// when a day turns under a player who has not changed map.
    ///
    /// **Called before the quest book is read** (`Session::quest_book`): settling the board can
    /// give up in-progress board quests, and the book must not carry them.
    pub(super) fn quest_ex_records(&self, character_id: u32) -> Vec<(u32, String)> {
        let towns = self.standings(character_id);
        let mut out = Vec::new();
        let value = cz::record_value(&towns);
        if !value.is_empty() {
            out.push((net::citizenship::CITIZENSHIP_QUEST, value));
        }
        let (board, _gave_up) = self.board(character_id);
        *self.board_sent.borrow_mut() = Some((store::dailyperks::today(), board.clone()));
        out.extend(board);
        out
    }

    fn ex_reply(quest_id: u32, value: &str, why: &str) -> Reply {
        Reply {
            opcode: net::quest::MESSAGE,
            body: net::citizenship::quest_ex_record(quest_id, value),
            what: format!("Message 13 (quest ex record): quest {quest_id} = {value:?} - {why}"),
        }
    }

    /// The journal's copy of a board quest the settle gave up.
    fn gave_up_reply(quest_id: u32) -> Reply {
        Reply {
            opcode: net::quest::MESSAGE,
            body: net::questforfeit::forfeit_reply(quest_id, false),
            what: format!("quest {quest_id} given up by the server: not this period's board quest (one per character)"),
        }
    }

    /// The four board records for `character_id`, each group settled for the period first
    /// ([`cz::settle`]): a new pick is stored, and in-progress board quests that are not it are
    /// given up. Returns the records and the quests given up.
    fn board(&self, character_id: u32) -> (Vec<(u32, String)>, Vec<u32>) {
        let towns = self.standings(character_id);
        let level = self.store.character_brief(character_id).ok().flatten().map_or(0, |c| c.level);
        let rows = self.store.quest_rows(character_id).unwrap_or_default();
        let now = board_now();
        let mut records = Vec::new();
        let mut gave_up = Vec::new();
        for group in &cz::BOARD_GROUPS {
            let (posted, dropped) = self.board_group(character_id, group, &towns, level, &rows, now);
            records.push((group.record_quest, cz::board_record(group, &posted)));
            gave_up.extend(dropped);
        }
        (records, gave_up)
    }

    /// One group: what it posts for this character now, and what it gave up.
    fn board_group(
        &self,
        character_id: u32,
        group: &cz::BoardGroup,
        towns: &[TownStanding],
        level: u32,
        rows: &[store::QuestRow],
        now: i64,
    ) -> (Vec<u32>, Vec<u32>) {
        let quests = &self.config.quests;
        let standing = towns.iter().find(|t| t.town == group.town && t.is_active());
        // Not a citizen there (the client locks these itself, `st != 1`), or no level to judge
        // by: what a grade-1 citizen would see, and nothing kept.
        let Some(standing) = standing.filter(|_| level > 0) else {
            return (cz::fresh_pick(group, quests, group.period(now), 1, level), Vec::new());
        };
        let board_rows: Vec<cz::BoardRow> = rows
            .iter()
            .filter(|r| group.contains(r.quest_id))
            .map(|r| cz::BoardRow {
                quest_id: r.quest_id,
                in_progress: r.state == store::QuestState::InProgress,
                completed_at: r.completed_at,
            })
            .collect();
        let stored = self.store.board_pick(character_id, group.record_quest).ok().flatten();
        let settled = cz::settle(group, quests, now, standing.grade, level, stored, &board_rows);
        if settled.save {
            let period = group.period(now);
            match self.store.set_board_pick(character_id, group.record_quest, period, &settled.pick) {
                Ok(()) => crate::server::log(&format!(
                    "   board {}: character {character_id} (grade {}, level {level}) gets {:?} for period {period}",
                    group.record_quest, standing.grade, settled.pick
                )),
                Err(e) => crate::server::log(&format!("   board {}: pick NOT kept ({e})", group.record_quest)),
            }
        }
        let mut gave_up = Vec::new();
        for &quest in &settled.drop {
            match self.store.forget_quest(character_id, quest) {
                Ok(true) => {
                    crate::server::log(&format!(
                        "   board {}: character {character_id}'s quest {quest} given up - one board quest per period",
                        group.record_quest
                    ));
                    gave_up.push(quest);
                }
                Ok(false) => {}
                Err(e) => crate::server::log(&format!("   board {}: quest {quest} NOT given up ({e})", group.record_quest)),
            }
        }
        (settled.posted, gave_up)
    }

    /// The citizenship record and the board postings, live, after anything changed them.
    fn citizenship_record_replies(&self, character_id: u32, why: &str) -> Vec<Reply> {
        let towns = self.standings(character_id);
        let mut out = vec![Self::ex_reply(net::citizenship::CITIZENSHIP_QUEST, &cz::record_value(&towns), why)];
        let (board, gave_up) = self.board(character_id);
        out.extend(gave_up.into_iter().map(Self::gave_up_reply));
        for (quest, value) in &board {
            out.push(Self::ex_reply(*quest, value, "the board, as this character's period stands"));
        }
        *self.board_sent.borrow_mut() = Some((store::dailyperks::today(), board));
        out
    }

    /// **The board turns at midnight UTC under a player who stays put.** The postings went
    /// out with the last `SetField`; once the day (or the Monday) turns, the ones that changed
    /// go out again as `0x0089` sub-case 13. From the tick, never beside a `SetField`.
    pub(super) fn board_tick(&mut self) -> Vec<Reply> {
        if self.in_cash_shop {
            return Vec::new();
        }
        let today = store::dailyperks::today();
        let sent = match &*self.board_sent.borrow() {
            Some((day, sent)) if *day != today => sent.clone(),
            _ => return Vec::new(),
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let (board, gave_up) = self.board(chr.id);
        let mut out: Vec<Reply> = gave_up.into_iter().map(Self::gave_up_reply).collect();
        out.extend(
            board
                .iter()
                .filter(|rec| !sent.contains(rec))
                .map(|(quest, value)| Self::ex_reply(*quest, value, "the board turned over (UTC)")),
        );
        *self.board_sent.borrow_mut() = Some((today, board));
        out
    }

    // -----------------------------------------------------------------------------------
    // The Town Clerks
    // -----------------------------------------------------------------------------------

    /// A click on Arthur or Roxy in their own hall. `None` for anyone else, and for a
    /// character below level 12 - who hears the clerk's own line.
    pub(super) fn open_town_clerk(&mut self, template: u32) -> Option<Vec<Reply>> {
        let town = cz::town_of_clerk(template)?;
        let chr = self.claimed_character()?;
        if chr.map_id != town.hall {
            return None;
        }
        // **An earring still owed is handed over first**, and that is this click's whole
        // answer: a Citizen of Honor whose Equip tab was full at the turn-in collects it here.
        let (earring, handed) = self.grant_honor_earring(chr.id, town.id);
        if handed {
            let mut out = earring;
            out.extend(self.clerk_says(
                template,
                format!(
                    "Here it is at last - the #b#t{}##k, awarded to the Citizen of Honor of {}. Wear it with pride!",
                    cz::honor_earring(town.id).unwrap_or(0),
                    town.name
                ),
                "the Citizen of Honor earring, owed since the grade-up",
            ));
            return Some(out);
        }
        let towns = self.standings(chr.id);
        let offer = cz::clerk_offer(chr.level, &towns, town.id);
        match offer {
            ClerkOffer::TooLow => None,
            ClerkOffer::Renunciation { grade } => {
                let t = towns.iter().find(|t| t.town == town.id)?;
                let next = cz::next_threshold(grade)
                    .map(|n| format!(" of the #b{n}#k the next grade needs"))
                    .unwrap_or_else(|| " - the highest grade there is".to_string());
                let text = format!(
                    "You are a #b{}#k of {}, grade {grade}, with #b{}#k Contribution{next}. Is there something I can do for you?\r\n#L0#No, thank you.#l\r\n#L1#I would like to renounce my citizenship.#l",
                    cz::grade_name(grade),
                    town.name,
                    t.contribution
                );
                self.conversation = Some(Conversation {
                    npc_template: template,
                    quest_id: None,
                    path: MENU_PATH.to_string(),
                    sent: 0,
                    awaiting_yes_no: false,
                    sent_with_next: false,
                });
                Some(vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_menu(template, &text),
                    what: format!("ScriptMessage MENU from the {} clerk: standing, or renounce", town.name),
                }])
            }
            // **A reactivation the purse cannot pay is said, not opened.** The window's OK
            // stamps the contract before the server has answered; refusing after the stamp
            // would be a contract drawn as signed that was not.
            ClerkOffer::Reactivation { .. } if self.store.mesos(chr.id).unwrap_or(0) < cz::REACTIVATION_FEE => {
                Some(self.clerk_says(
                    template,
                    format!(
                        "Welcome back! Resuming your citizenship in {} costs #b{} mesos#k. Come back when you have it.",
                        town.name,
                        cz::REACTIVATION_FEE
                    ),
                    "reactivation refused before the window: short of the fee",
                ))
            }
            offer => Some(self.open_contract(template, town.id, offer)),
        }
    }

    fn clerk_says(&mut self, template: u32, text: String, why: &str) -> Vec<Reply> {
        self.conversation = None;
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(template, &text, false, false),
            what: format!("ScriptMessage Say from town clerk {template}: {why}"),
        }]
    }

    fn open_contract(&mut self, clerk: u32, town: u8, offer: ClerkOffer) -> Vec<Reply> {
        let Some(contract) = contract_for(town, offer) else { return Vec::new() };
        self.conversation = Some(Conversation {
            npc_template: clerk,
            quest_id: None,
            path: CONTRACT_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::citizenship::contract(CONTRACT_HANDLE, clerk, contract),
            what: format!(
                "ScriptMessage 0x{:02X} contract window {contract:?} from clerk {clerk} ({offer:?})",
                contract.message_type()
            ),
        }]
    }

    /// The clerk's menu came back. `None` when it is not parked.
    pub(super) fn citizenship_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if convo.path != MENU_PATH {
            return None;
        }
        let reply = net::script::parse_menu_reply(body)?;
        self.conversation = None;
        if reply.selection != Some(1) {
            return Some(Vec::new());
        }
        let town = cz::town_of_clerk(convo.npc_template)?;
        let chr = self.claimed_character()?;
        match cz::clerk_offer(chr.level, &self.standings(chr.id), town.id) {
            offer @ ClerkOffer::Renunciation { .. } => Some(self.open_contract(convo.npc_template, town.id, offer)),
            _ => Some(Vec::new()),
        }
    }

    /// A contract window's answer. Runs BEFORE the Say decoder, which would drop these
    /// six-byte bodies. `None` when no contract is parked or the body is not a contract answer.
    pub(super) fn citizenship_contract_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if convo.path != CONTRACT_PATH && convo.path != STAMP_PATH {
            return None;
        }
        match net::citizenship::parse_contract_answer(body)? {
            ContractAnswer::Cancelled { .. } | ContractAnswer::StampDone { .. } => {
                self.conversation = None;
                Some(Vec::new())
            }
            ContractAnswer::Accepted { .. } if convo.path == STAMP_PATH => Some(Vec::new()),
            ContractAnswer::Accepted { message_type } => {
                if let Some(c) = self.conversation.as_mut() {
                    c.path = STAMP_PATH.to_string();
                }
                // Every contract that is signed changes the stored standing, and every refusal
                // leaves it alone - so the difference is the answer to "stamp or close".
                let id = self.claimed_character().map(|c| c.id);
                let before = id.map(|id| self.standings(id));
                let mut out = self.sign_contract(convo.npc_template, message_type);
                let signed = id.is_some() && id.map(|id| self.standings(id)) != before;
                out.push(Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::script_force_close(u8::from(signed)),
                    what: if signed {
                        format!("ScriptMessage 0x47 force-close, result 1: the contract window (0x{message_type:02X}) plays its STAMP, then answers 0x47 and closes itself ~2 s later")
                    } else {
                        format!("ScriptMessage 0x47 force-close, result 0: the contract (0x{message_type:02X}) was NOT signed - the window closes with no stamp")
                    },
                });
                Some(out)
            }
        }
    }

    /// OK was pressed. **Every effect hangs off one successful write** of the new standings.
    fn sign_contract(&mut self, clerk: u32, message_type: u8) -> Vec<Reply> {
        let Some(town) = cz::town_of_clerk(clerk) else { return Vec::new() };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let towns = self.standings(chr.id);
        let offer = cz::clerk_offer(chr.level, &towns, town.id);
        if contract_for(town.id, offer).map(Contract::message_type) != Some(message_type) {
            return self.notice(format!(
                "That contract no longer applies - nothing was signed. ({offer:?}, window 0x{message_type:02X})"
            ));
        }
        let Some(after) = cz::after_contract(&towns, town.id, offer) else {
            return self.notice("That contract no longer applies - nothing was signed.".to_string());
        };
        let mut out = Vec::new();
        if matches!(offer, ClerkOffer::Reactivation { .. }) {
            match self.store.add_mesos(chr.id, -i64::from(cz::REACTIVATION_FEE)) {
                Ok(_) => {
                    out.extend(self.meso_reply(chr.id));
                    out.push(Reply {
                        opcode: net::message::MESSAGE,
                        body: net::message::meso_lost_line(cz::REACTIVATION_FEE),
                        what: format!("Message: the {} reactivation fee, {} mesos", town.name, cz::REACTIVATION_FEE),
                    });
                }
                Err(e) => {
                    return self.notice(format!(
                        "Your citizenship in {} was not resumed: the {} meso fee could not be taken ({e}).",
                        town.name,
                        cz::REACTIVATION_FEE
                    ))
                }
            }
        }
        if let Err(e) = self.store.set_citizenship(chr.id, &after) {
            if matches!(offer, ClerkOffer::Reactivation { .. }) {
                let _ = self.store.add_mesos(chr.id, i64::from(cz::REACTIVATION_FEE));
                out.extend(self.meso_reply(chr.id));
            }
            out.extend(self.notice(format!("The contract could not be recorded, and nothing changed: {e}")));
            return out;
        }
        crate::server::log(&format!(
            "   citizenship: {} ({}) signed {offer:?} with {} - now {:?}",
            chr.name, chr.id, town.name, after
        ));
        out.extend(self.citizenship_record_replies(chr.id, "a contract was signed"));
        let line = match offer {
            ClerkOffer::Oath | ClerkOffer::Transfer { .. } => {
                out.push(Self::citizenship_effect(net::citizenship::EFFECT_CITIZENSHIP_GET, "CitizenshipGet"));
                format!("You are now a citizen of {} - a {}.", town.name, cz::grade_name(1))
            }
            ClerkOffer::Reactivation { grade } => {
                out.push(Self::citizenship_effect(net::citizenship::EFFECT_CITIZENSHIP_GET, "CitizenshipGet"));
                format!("Welcome back! Your citizenship in {} is active again - a {}.", town.name, cz::grade_name(grade))
            }
            ClerkOffer::Renunciation { .. } => {
                format!("You have renounced your citizenship in {}. Your grade is kept for your return.", town.name)
            }
            ClerkOffer::Certificate { grade } => {
                format!("Your certificate as a {} of {} has been issued.", cz::grade_name(grade), town.name)
            }
            ClerkOffer::TooLow => String::new(),
        };
        if !line.is_empty() {
            out.extend(self.notice(line));
        }
        out
    }

    fn citizenship_effect(effect: u8, name: &str) -> Reply {
        Reply {
            opcode: net::questeffect::USER_EFFECT_LOCAL,
            body: net::stats::user_effect_local(effect),
            what: format!("UserEffectLocal {effect} {name} - Effect/BasicEff.img/{name}"),
        }
    }

    // -----------------------------------------------------------------------------------
    // Quests
    // -----------------------------------------------------------------------------------

    /// The citizenship half of accepting a quest: `Ok(true)` when it is a completed board
    /// quest being picked up again (the caller restarts the row), `Ok(false)` to carry on as
    /// usual, `Err(replies)` for a refusal - said by `speaker` and parked like the bag-full
    /// one, so the caller's closing line does not follow it.
    ///
    /// The client refuses all of these itself (`0x50`, `0x51`, `0x13`, `0x16`, `0x19`); this
    /// is the same table on the side of the socket that has to be believed.
    pub(super) fn citizenship_start_gate(&mut self, quest_id: u32, speaker: u32) -> Result<bool, Vec<Reply>> {
        let Some(quest) = self.config.quests.get(&quest_id) else { return Ok(false) };
        let check = quest.citizenship_check;
        let group = cz::board_group_of(quest_id);
        if check.is_none() && group.is_none() {
            return Ok(false);
        }
        let Some(chr) = self.claimed_character() else { return Ok(false) };
        let towns = self.standings(chr.id);
        if let Some((town, grade)) = check {
            if !cz::meets(&towns, town, grade) {
                let name = cz::town(town).map_or("that town", |t| t.name);
                return Err(self.citizenship_refusal(
                    quest_id,
                    speaker,
                    format!("This is for citizens of {name} of grade {grade} (#b{}#k) or higher.", cz::grade_name(grade)),
                ));
            }
        }
        let Some(group) = group else { return Ok(false) };
        let now = board_now();
        // **This character's posting** - one per period, kept from when the period started.
        let level = self.store.character_brief(chr.id).ok().flatten().map_or(0, |c| c.level);
        let rows = self.store.quest_rows(chr.id).unwrap_or_default();
        let (posted, _gave_up) = self.board_group(chr.id, group, &towns, level, &rows, now);
        if !posted.contains(&quest_id) {
            let turned_in = rows.iter().any(|r| {
                group.contains(r.quest_id) && r.completed_at.is_some_and(|at| group.period(at) == group.period(now))
            });
            let text = match (turned_in, group.weekly) {
                (true, true) => "You have already done this week's notice. The board changes on Monday.".to_string(),
                (true, false) => "You have already done today's notice. Come back tomorrow.".to_string(),
                (false, weekly) => {
                    format!("That notice is not on the board for you {}.", if weekly { "this week" } else { "today" })
                }
            };
            return Err(self.citizenship_refusal(quest_id, speaker, text));
        }
        let row = self.store.quest_row(chr.id, quest_id).ok().flatten();
        match row {
            Some(r) if r.state == store::QuestState::Complete => {
                if cz::may_repeat(group, quest_id, r.completed_at.unwrap_or(0), now) {
                    Ok(true)
                } else if group.do_not_repeat(quest_id) {
                    Err(self.citizenship_refusal(quest_id, speaker, "A first greeting only happens once.".to_string()))
                } else {
                    let when = if group.weekly { "this week - the board changes on Monday" } else { "today - come back tomorrow" };
                    Err(self.citizenship_refusal(quest_id, speaker, format!("You have already done this one {when}.")))
                }
            }
            _ => Ok(false),
        }
    }

    fn citizenship_refusal(&mut self, quest_id: u32, speaker: u32, text: String) -> Vec<Reply> {
        crate::server::log(&format!("   quest {quest_id}: NOT accepted - {text}"));
        if speaker == 0 {
            return self.notice(text);
        }
        self.conversation = Some(Conversation {
            npc_template: speaker,
            quest_id: None,
            path: crate::questroom::REFUSAL_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(speaker, &text, false, false),
            what: format!("ScriptMessage Say from NPC template {speaker}: quest {quest_id} not accepted - {text}"),
        }]
    }

    /// `Act.1.citizenshipContr`, paid on a turn-in that was actually recorded: the line, the
    /// record, and - only on a grade-up - the effect and the pointer to the clerk.
    ///
    /// The amount uses the grade **at the turn-in**, which is the grade the quest window drew
    /// the reward with. Nothing is banked for a town the character is not an active citizen
    /// of (the client would not have let them start it; a GM or a renunciation in between can).
    pub(super) fn bank_citizenship_contribution(&mut self, quest_id: u32) -> Vec<Reply> {
        let Some(contr) = self.config.quests.get(&quest_id).and_then(|q| q.citizenship_contr.clone()) else {
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let towns = self.standings(chr.id);
        let Some(before) = towns.iter().find(|t| t.town == contr.town && t.is_active()).copied() else {
            crate::server::log(&format!(
                "   quest {quest_id}: {} is not an active citizen of town {} - no Contribution banked",
                chr.name, contr.town
            ));
            return Vec::new();
        };
        // **Contribution is the quest's own number, never rated.** The Community Board's dailies
        // and weeklies paid at the Quest rate from 2026-09-28 (*"10x as well"*); the owner reverted
        // that on 2026-10-02 after a grade-2 character banked 1 000 for a daily and 5 000 for a
        // weekly and landed on grade 7 - the grade table is 1 000 a grade, so at 10x one weekly
        // was five grades. Quest MESOS still pay at the rate (`pay_quest_mesos`).
        let amount = cz::contribution_for(&contr, before.grade);
        if amount == 0 {
            crate::server::log(&format!("   quest {quest_id}: Contribution 0 from {contr:?} - nothing banked"));
            return Vec::new();
        }
        let after = match self.store.add_contribution(chr.id, contr.town, amount) {
            Ok(Some(after)) => after,
            Ok(None) => return Vec::new(),
            Err(e) => return self.notice(format!("Your Contribution could not be recorded: {e}")),
        };
        let regraded = cz::regrade(after);
        if regraded.grade != after.grade {
            if let Err(e) = self.store.set_citizenship(chr.id, &[regraded]) {
                return self.notice(format!("Your new grade could not be recorded: {e}"));
            }
        }
        let banked = cz::Banked { after: regraded, grade_before: before.grade };
        let mut out = vec![Reply {
            opcode: net::quest::MESSAGE,
            body: net::citizenship::contribution_gained(contr.town, amount),
            what: format!(
                "Message 35: +{amount} Contribution to town {} for quest {quest_id} ({} -> {})",
                contr.town, before.contribution, regraded.contribution
            ),
        }];
        out.extend(self.citizenship_record_replies(chr.id, "Contribution banked"));
        if banked.graded_up() {
            out.push(Self::citizenship_effect(net::citizenship::EFFECT_CITIZENSHIP_GRADE_UP, "CitizenshipGradeUp"));
            // **Citizen of Honor**: the town's earring, and every channel is told. Only on the
            // transition to 10 - the grade never falls, so this runs once per town.
            if regraded.grade == cz::MAX_GRADE && before.grade < cz::MAX_GRADE {
                out.extend(self.grant_honor_earring(chr.id, contr.town).0);
                out.extend(self.announce_everywhere(&cz::honor_announcement(&chr.name, contr.town)));
            }
            let town = cz::town(contr.town);
            out.extend(self.notice(format!(
                "Your citizenship grade in {} is now {} - {}! See {} at the {} for your certificate.",
                town.map_or("your town", |t| t.name),
                regraded.grade,
                cz::grade_name(regraded.grade),
                if contr.town == cz::HENESYS { "Arthur" } else { "Roxy" },
                if contr.town == cz::HENESYS { "Town Hall" } else { "Civic Center" },
            )));
        }
        out
    }

    /// `Act.1.money`, at the Quest rate - the same multiplier quest EXP gets. **Every quest**:
    /// The owner, 2026-09-28, first for the Community Board and then *"for all quests"*. On a
    /// recorded turn-in only (the caller's early return), so a repeat click pays nothing.
    pub(super) fn pay_quest_mesos(&mut self, quest_id: u32) -> Vec<Reply> {
        let base = self.config.quests.get(&quest_id).map_or(0, |q| q.complete_money);
        if base == 0 {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let rate = self.rate(store::rates::RateKind::Quest);
        let paid = u32::try_from(rate.apply(u64::from(base))).unwrap_or(u32::MAX).min(i32::MAX as u32);
        if let Err(e) = self.store.add_mesos(chr.id, i64::from(paid)) {
            return self.notice(format!("Quest {quest_id}'s {paid} mesos could not be paid: {e}"));
        }
        let mut out = self.meso_reply(chr.id);
        out.push(Reply {
            opcode: net::message::MESSAGE,
            body: net::message::meso_gained(paid as i32),
            what: format!("Message: quest {quest_id} paid {paid} mesos ({base} at {rate}x)"),
        });
        out
    }

    /// **The Citizen of Honor earring** for `town`, if this character is grade 10 there and
    /// has not had it. `(replies, handed over)`. A full Equip tab is not a loss: the notice
    /// says so, nothing is marked, and the town's clerk hands it over on the next talk
    /// ([`Session::open_town_clerk`]). Marked with a test-and-set, so it can never come twice.
    pub(super) fn grant_honor_earring(&mut self, character_id: u32, town: u8) -> (Vec<Reply>, bool) {
        let Some(item_id) = cz::honor_earring(town) else { return (Vec::new(), false) };
        let honored = self.standings(character_id).iter().any(|t| t.town == town && t.grade >= cz::MAX_GRADE);
        if !honored || self.store.honor_earring_given(character_id, town).unwrap_or(true) {
            return (Vec::new(), false);
        }
        let clerk = if town == cz::HENESYS { "Arthur at the Henesys Town Hall" } else { "Roxy at the Kerning City Civic Center" };
        let inv = store::InventoryType::Equip;
        match self.store.add_item(character_id, inv, &store::Item::equip(item_id), 1) {
            Ok(placed) => {
                if let Err(e) = self.store.mark_honor_earring(character_id, town) {
                    crate::server::log(&format!("   citizenship: earring {item_id} handed to {character_id} but NOT marked: {e}"));
                }
                crate::server::log(&format!("   citizenship: {character_id} is a Citizen of Honor of town {town} - earring {item_id} handed over"));
                let mut out = self.inventory_added_replies(inv, &placed, "the Citizen of Honor earring");
                out.push(self.item_chat_line(item_id, 1));
                (out, true)
            }
            Err(store::StoreError::BagFull { .. }) => (
                self.notice(format!(
                    "Your Citizen of Honor earring is waiting for you. Make room in your Equip tab and see {clerk}."
                )),
                false,
            ),
            Err(e) => (self.notice(format!("Your Citizen of Honor earring could not be handed over: {e}. See {clerk}.")), false),
        }
    }

    /// **A blue `[Notice]` line on every channel** (`net::broadcast::notice`): this session's
    /// own copy comes back in the replies, everyone else online gets it through the world hub
    /// (`crate::link::everyone` + `deliver_anywhere`), or this channel's bus when no hub is
    /// linked.
    pub(super) fn announce_everywhere(&self, text: &str) -> Vec<Reply> {
        let reply = Reply {
            opcode: net::broadcast::BROADCAST_MSG,
            body: net::broadcast::notice(text),
            what: format!("BroadcastMsg type 0 (blue [Notice]), to every channel: {text}"),
        };
        let me = self.claimed_character().map(|c| c.id);
        let others: Vec<u32> = match crate::link::installed() {
            Some(link) => link.everyone().into_iter().map(|(id, _)| id).collect(),
            None => self.bus().everyone_here().into_iter().map(|(id, _)| id).collect(),
        };
        let mut told = 0;
        for id in others.into_iter().filter(|id| Some(*id) != me) {
            if self.deliver_anywhere(id, reply.clone()) {
                told += 1;
            }
        }
        crate::server::log(&format!("   announce: {text:?} - told {told} other player(s)"));
        vec![reply]
    }

    // -----------------------------------------------------------------------------------
    // Shops
    // -----------------------------------------------------------------------------------

    /// The `(town, grade)` a row of this shop is locked behind, for the wire's `+0x104/+0x108`
    /// and for the refusal: the shop's own town, the row's tag.
    pub(super) fn shop_row_citizenship(template: u32, min_grade: Option<u8>) -> Option<(u8, u8)> {
        Some((cz::town_of_shop(template)?, min_grade?))
    }

    /// Whether this character may buy a row locked behind `(town, grade)`.
    pub(super) fn may_buy_gated(&self, character_id: u32, town: u8, grade: u8) -> bool {
        cz::meets(&self.standings(character_id), town, grade)
    }

    // -----------------------------------------------------------------------------------
    // !citizenship
    // -----------------------------------------------------------------------------------

    /// `!citizenship` - show; `!citizenship <town> <state|grade|contr> <value>` - set, the
    /// client's own GM command's shape (`/citizenship <townID> <state|grade|contr> <val>`, a
    /// string in the executable). Setting `state 1` freezes any other active town, so the
    /// one-active-town rule holds for GMs too.
    pub(super) fn gm_citizenship(&mut self, arg: &str) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self.gm_ack("!citizenship REFUSED: no character is claimed on this connection.".to_string());
        };
        let mut towns = self.standings(chr.id);
        let words: Vec<&str> = arg.split_whitespace().collect();
        if words.is_empty() {
            let v = cz::record_value(&towns);
            return self.gm_ack(format!(
                "{}: {}. `!citizenship <1|2> <state|grade|contr> <value>` sets one.",
                chr.name,
                if v.is_empty() { "no citizenship".to_string() } else { v }
            ));
        }
        let (Some(town), Some(field), Some(value)) = (
            words.first().and_then(|w| w.parse::<u8>().ok()).filter(|t| cz::town(*t).is_some()),
            words.get(1).copied(),
            words.get(2).and_then(|w| w.parse::<u32>().ok()),
        ) else {
            return self.gm_ack("!citizenship <1|2> <state|grade|contr> <value> - town 1 Henesys, 2 Kerning City.".to_string());
        };
        if !towns.iter().any(|t| t.town == town) {
            towns.push(TownStanding { town, state: store::citizenship::STATE_FROZEN, grade: 1, contribution: 0, certified_grade: 1 });
        }
        for t in towns.iter_mut() {
            if t.town == town {
                match field {
                    "state" | "st" => t.state = value.min(255) as u8,
                    "grade" | "gr" => {
                        t.grade = value.clamp(1, u32::from(cz::MAX_GRADE)) as u8;
                        t.certified_grade = t.certified_grade.min(t.grade);
                    }
                    "contr" | "ct" => t.contribution = value,
                    other => return self.gm_ack(format!("!citizenship: {other:?} is not state, grade or contr.")),
                }
            }
        }
        if field.starts_with("st") && value == u32::from(store::citizenship::STATE_ACTIVE) {
            for t in towns.iter_mut().filter(|t| t.town != town && t.is_active()) {
                t.state = store::citizenship::STATE_FROZEN;
            }
        }
        if let Err(e) = self.store.set_citizenship(chr.id, &towns) {
            return self.gm_ack(format!("!citizenship FAILED and nothing changed: {e}"));
        }
        let mut out = self.gm_ack(format!("{} is now {}.", chr.name, cz::record_value(&towns)));
        out.extend(self.citizenship_record_replies(chr.id, "!citizenship"));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CitizenshipContr, Config, Quest};
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::citizenship::{STATE_ACTIVE, STATE_FROZEN};
    use store::Store;

    const HALL_H: u32 = 10_001_007;
    const HALL_K: u32 = 10_003_007;
    /// Flint, the Henesys scroll shop - a town-hall shop with grade-locked rows.
    const FLINT: u32 = 234;
    const LOCKED_SCROLL: u32 = 2_040_804;
    const OPEN_POTION: u32 = 2_000_000;

    /// Every board quest with its WZ gate and pay (`crate::citizenship`'s fixture, checked
    /// against the real data there).
    fn quests() -> std::collections::HashMap<u32, Quest> {
        let mut m = std::collections::HashMap::new();
        let formula = || Some("100 + ( ( citizenshipGrade - 1 ) x 50 )".to_string());
        for (first, town) in [(506_001u32, 1u8), (506_101, 2)] {
            for q in first..first + 18 {
                let grade = if q < first + 14 { 1 } else { 5 };
                m.insert(q, Quest {
                    citizenship_check: Some((town, grade)),
                    citizenship_contr: Some(CitizenshipContr { town, amount: None, formula: formula() }),
                    complete_money: 351,
                    complete_min_level: if grade == 5 { 32 } else { 12 },
                    ..Quest::default()
                });
            }
        }
        let weeklies: [(u32, u8, Vec<u8>); 2] = [
            (506_019, 1, vec![1, 1, 1, 1, 1, 1, 2, 3, 3, 3, 4, 5, 5, 6, 7, 8, 9]),
            (506_119, 2, vec![1, 1, 1, 1, 1, 1, 2, 3, 3, 3, 4, 5, 5, 5, 6, 7, 8, 9]),
        ];
        for (first, town, gates) in weeklies {
            for (i, g) in gates.into_iter().enumerate() {
                m.insert(first + i as u32, Quest {
                    citizenship_check: Some((town, g)),
                    citizenship_contr: Some(CitizenshipContr { town, amount: Some(500 + 250 * (u32::from(g) - 1)), formula: None }),
                    complete_money: 351,
                    complete_min_level: 12 + 5 * (u32::from(g) - 1),
                    // The donations' completion talk is a yes/no in the client's data - "1" asks,
                    // "1.yes" thanks (506024's own lines, gm-handbook/questlines.txt).
                    say: [
                        ("1".to_string(), vec!["Are you saying you'd like to donate?".to_string()]),
                        ("1.yes".to_string(), vec!["Thank you so much!".to_string()]),
                    ]
                    .into_iter()
                    .collect(),
                    ..Quest::default()
                });
            }
        }
        // A story arc step (Bruce's Dilemma): Contribution 50, mesos, not a board quest.
        m.insert(STORY_QUEST, Quest {
            citizenship_check: Some((1, 2)),
            citizenship_contr: Some(CitizenshipContr { town: 1, amount: Some(50), formula: None }),
            complete_money: 351,
            ..Quest::default()
        });
        m
    }

    const STORY_QUEST: u32 = 506_036;

    fn npc(object_id: u32, template_id: u32) -> net::opcode::FieldNpc {
        net::opcode::FieldNpc { object_id, template_id, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }
    }

    fn resident(level: u32, map: u32, mesos: u32) -> (Arc<Store>, Session, u32) {
        let (store, mut all) = residents(&["Resident"], level, map, mesos);
        let (s, id) = all.remove(0);
        (store, s, id)
    }

    /// Several characters on ONE channel - one store, one config, one `Fields` (so one bus),
    /// each already entered on `map`.
    fn residents(names: &[&str], level: u32, map: u32, mesos: u32) -> (Arc<Store>, Vec<(Session, u32)>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let mut cfg = Config::default();
        cfg.fields.insert(HALL_H);
        cfg.fields.insert(HALL_K);
        cfg.npcs.insert(HALL_H, vec![npc(900, 229), npc(901, FLINT)]);
        cfg.npcs.insert(HALL_K, vec![npc(910, 425)]);
        cfg.quests = quests();
        let item = |item_id, min_grade| crate::shops::ShopItem {
            item_id,
            name: format!("item {item_id}"),
            buy_price: 100,
            sell_price: 10,
            min_grade,
            quest_item: false,
            trade_blocked: false,
        };
        let shop = crate::shops::Shop {
            npc: "Flint".to_string(),
            role: "Town Scroll Shop".to_string(),
            map_label: "Henesys Town Hall".to_string(),
            items: vec![item(OPEN_POTION, None), item(LOCKED_SCROLL, Some(5))],
        };
        let mut item_data = std::collections::HashMap::new();
        item_data.insert(OPEN_POTION, crate::shops::ItemData { price: 10, slot_max: 100, ..Default::default() });
        item_data.insert(LOCKED_SCROLL, crate::shops::ItemData { price: 10, slot_max: 1, ..Default::default() });
        cfg.shops = crate::shops::ShopTable { shops: vec![shop], item_data, problems: Vec::new() };
        cfg.shop_by_template.insert(FLINT, 0);
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let (config, fields) = (Arc::new(cfg), Arc::new(Fields::new()));
        let mut out = Vec::new();
        for name in names {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map, level, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, map).unwrap();
            store.set_mesos(id, mesos).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            if names.len() > 1 {
                let _ = s.on_field_entered();
            }
            out.push((s, id));
        }
        (store, out)
    }

    fn click(object_id: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        b.extend_from_slice(&object_id.to_le_bytes());
        b.extend_from_slice(&[0u8; 4]);
        b.extend_from_slice(&u32::MAX.to_le_bytes());
        b
    }

    fn answer(first: u8, second: u8) -> Vec<u8> {
        let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        b.extend_from_slice(&CONTRACT_HANDLE.to_le_bytes());
        b.push(first);
        b.push(second);
        b
    }

    fn menu_pick(line: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(net::script::SCRIPT_TYPE_MENU);
        b.push(1);
        b.extend_from_slice(&line.to_le_bytes());
        b
    }

    fn quest_request(action: u8, quest_id: u32, npc: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_QUEST_REQUEST.to_le_bytes().to_vec();
        b.push(action);
        b.extend_from_slice(&quest_id.to_le_bytes());
        b.extend_from_slice(&npc.to_le_bytes());
        b.extend_from_slice(&[0u8; 8]);
        b
    }

    /// The script window's message type, off a `0x055B` body (offset 10, no override).
    fn window(out: &[Reply]) -> Option<u8> {
        out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| r.body[10])
    }

    fn ex_line(out: &[Reply], quest: u32) -> Option<String> {
        out.iter()
            .filter(|r| r.opcode == net::quest::MESSAGE && r.body[0] == net::citizenship::MESSAGE_QUEST_EX)
            .find(|r| r.body[1..5] == quest.to_le_bytes())
            .map(|r| String::from_utf8_lossy(&r.body[7..]).to_string())
    }

    fn has_effect(out: &[Reply], effect: u8) -> bool {
        out.iter().any(|r| r.opcode == net::questeffect::USER_EFFECT_LOCAL && r.body == net::stats::user_effect_local(effect))
    }

    fn accepted(out: &[Reply], quest: u32) -> bool {
        out.iter().any(|r| r.opcode == net::quest::MESSAGE && r.body == net::quest::quest_accepted(quest))
    }

    fn standing(store: &Store, id: u32) -> String {
        crate::citizenship::record_value(&store.citizenship(id).unwrap())
    }

    fn active_in(town: u8, grade: u8, contribution: u32) -> TownStanding {
        TownStanding { town, state: STATE_ACTIVE, grade, contribution, certified_grade: grade }
    }

    /// Arthur opens the Oath for a level-12 newcomer; OK signs it (the record, the effect),
    /// the stamp's second answer is swallowed, and the next click is the citizen's menu.
    #[test]
    fn the_oath_makes_a_citizen_and_the_stamp_answer_is_swallowed() {
        let (store, mut s, id) = resident(12, HALL_H, 0);
        let out = s.handle(&click(900));
        assert_eq!(window(&out), Some(net::citizenship::CONTRACT_OATH), "{out:?}");
        let out = s.handle(&answer(0x42, 1));
        assert_eq!(standing(&store, id), "st1=1;gr1=1;ct1=0");
        // **The server starts the stamp**: a force-close with result 1, after everything else.
        // Without it the window sits there with greyed buttons forever (the owner's first run).
        let last = out.last().unwrap();
        assert_eq!((last.opcode, &last.body), (net::script::SCRIPT_MESSAGE, &net::script::script_force_close(1)), "{out:?}");
        assert_eq!(ex_line(&out, 510_000).as_deref(), Some("st1=1;gr1=1;ct1=0"));
        assert!(ex_line(&out, 510_001).is_some_and(|v| v.starts_with("q1_d=")), "the board follows");
        assert!(has_effect(&out, net::citizenship::EFFECT_CITIZENSHIP_GET));
        assert!(s.handle(&answer(0x47, 0x42)).is_empty(), "the stamp's answer closes it, silently");
        let out = s.handle(&click(900));
        assert_eq!(window(&out), Some(net::script::SCRIPT_TYPE_MENU), "a citizen gets the menu, not a second Oath");
    }

    /// **A contract that no longer applies closes with no stamp.** The Transfer window is open
    /// at Roxy; behind it the character becomes a (frozen) Kerning citizen, so OK is refused:
    /// nothing changes and the server sends the force-close with result 0, not 1.
    #[test]
    fn a_refused_contract_closes_the_window_without_a_stamp() {
        let (store, mut s, id) = resident(30, HALL_K, 0);
        store.set_citizenship(id, &[active_in(1, 3, 2_400)]).unwrap();
        assert_eq!(window(&s.handle(&click(910))), Some(net::citizenship::CONTRACT_TRANSFER));
        let behind = TownStanding { town: 2, state: STATE_FROZEN, grade: 1, contribution: 0, certified_grade: 1 };
        store.set_citizenship(id, &[behind]).unwrap();
        let before = standing(&store, id);
        let out = s.handle(&answer(0x43, 1));
        assert_eq!(standing(&store, id), before, "nothing signed");
        assert_eq!(out.last().map(|r| r.body.clone()), Some(net::script::script_force_close(0)), "{out:?}");
        assert!(s.handle(&answer(0x47, 0x43)).is_empty(), "the close's own 0x47 answer is swallowed");
    }

    /// Cancel signs nothing; an answer with no window parked signs nothing; below level 12
    /// the clerk just talks.
    #[test]
    fn cancel_and_too_low_sign_nothing() {
        let (store, mut s, id) = resident(12, HALL_H, 0);
        let _ = s.handle(&click(900));
        assert!(s.handle(&answer(0x42, 0)).is_empty());
        assert!(store.citizenship(id).unwrap().is_empty());
        let _ = s.handle(&answer(0x42, 1));
        assert!(store.citizenship(id).unwrap().is_empty(), "no parked contract, no signature");

        let (store, mut s, id) = resident(11, HALL_H, 0);
        let out = s.handle(&click(900));
        assert_ne!(window(&out), Some(net::citizenship::CONTRACT_OATH));
        assert!(store.citizenship(id).unwrap().is_empty());
    }

    /// Roxy offers a Henesys citizen the Transfer; signing freezes Henesys with its grade.
    /// Back at Arthur it is the Reactivation, for the fee.
    #[test]
    fn a_transfer_freezes_the_old_town_and_a_reactivation_charges_the_fee() {
        let (store, mut s, id) = resident(30, HALL_K, 60_000);
        store.set_citizenship(id, &[active_in(1, 3, 2_400)]).unwrap();
        let out = s.handle(&click(910));
        assert_eq!(window(&out), Some(net::citizenship::CONTRACT_TRANSFER));
        let _ = s.handle(&answer(0x43, 1));
        let _ = s.handle(&answer(0x47, 0x43));
        assert_eq!(standing(&store, id), format!("st1={STATE_FROZEN};gr1=3;ct1=2400;st2=1;gr2=1;ct2=0"));

        store.set_character_map(id, HALL_H).unwrap();
        let out = s.handle(&click(900));
        assert_eq!(window(&out), Some(net::citizenship::CONTRACT_REACTIVATION));
        let out = s.handle(&answer(0x44, 1));
        assert_eq!(store.mesos(id).unwrap(), 60_000 - crate::citizenship::REACTIVATION_FEE);
        assert_eq!(standing(&store, id), format!("st1=1;gr1=3;ct1=2400;st2={STATE_FROZEN};gr2=1;ct2=0"));
        assert!(out.iter().any(|r| r.body == net::message::meso_lost_line(crate::citizenship::REACTIVATION_FEE)));
    }

    /// Short of the fee, the clerk says so instead of opening a window that would stamp.
    #[test]
    fn a_reactivation_the_purse_cannot_pay_is_said_not_opened() {
        let (store, mut s, id) = resident(30, HALL_H, 49_999);
        store
            .set_citizenship(id, &[TownStanding { town: 1, state: STATE_FROZEN, grade: 2, contribution: 1_000, certified_grade: 2 }])
            .unwrap();
        let out = s.handle(&click(900));
        assert_eq!(window(&out), Some(net::script::SCRIPT_TYPE_SAY));
        assert_eq!(store.mesos(id).unwrap(), 49_999);
    }

    /// The menu's second line opens the Renunciation; signing it freezes the town.
    #[test]
    fn renouncing_goes_through_the_menu_and_keeps_the_grade() {
        let (store, mut s, id) = resident(30, HALL_H, 0);
        store.set_citizenship(id, &[active_in(1, 4, 3_100)]).unwrap();
        let _ = s.handle(&click(900));
        let out = s.handle(&menu_pick(1));
        assert_eq!(window(&out), Some(net::citizenship::CONTRACT_RENUNCIATION));
        let _ = s.handle(&answer(0x45, 1));
        assert_eq!(standing(&store, id), format!("st1={STATE_FROZEN};gr1=4;ct1=3100"));
    }

    /// The board gates, end to end: not a citizen -> refused; a citizen -> only what is posted;
    /// done today -> refused; done yesterday and posted again -> picked up afresh; a First
    /// Greeting -> never again.
    #[test]
    fn board_quests_follow_the_posting_and_the_repeat_rules() {
        let (store, mut s, id) = resident(30, HALL_H, 0);
        let group = &crate::citizenship::BOARD_GROUPS[0];
        let posted = crate::citizenship::fresh_pick(group, &s.config.quests, group.period(board_now()), 1, 30);
        let asking_after = posted[1];
        let unposted = (506_001..=506_014).find(|q| !posted.contains(q)).unwrap();

        let out = s.handle(&quest_request(1, posted[0], 235));
        assert!(!accepted(&out, posted[0]), "not a citizen: refused");
        assert!(store.quest_row(id, posted[0]).unwrap().is_none());

        store.set_citizenship(id, &[active_in(1, 1, 0)]).unwrap();
        let out = s.handle(&quest_request(1, unposted, 235));
        assert!(!accepted(&out, unposted), "not on the board today");
        assert!(store.quest_row(id, unposted).unwrap().is_none());

        let out = s.handle(&quest_request(1, asking_after, 235));
        assert!(accepted(&out, asking_after));
        let _ = s.handle(&quest_request(2, asking_after, 235));
        assert_eq!(store.quest_row(id, asking_after).unwrap().unwrap().state, store::QuestState::Complete);

        let out = s.handle(&quest_request(1, asking_after, 235));
        assert!(!accepted(&out, asking_after), "done today");

        let mut row = store.quest_row(id, asking_after).unwrap().unwrap();
        row.completed_at = Some(board_now() - 3_600);
        store.save_quest(id, &row).unwrap();
        let out = s.handle(&quest_request(1, asking_after, 235));
        assert!(out.iter().any(|r| r.body == net::quest::quest_forgotten(asking_after, true)), "the completion is cleared first");
        assert!(accepted(&out, asking_after));
        assert_eq!(store.quest_row(id, asking_after).unwrap().unwrap().state, store::QuestState::InProgress);

        let first = posted[0];
        store
            .save_quest(id, &store::QuestRow { quest_id: first, state: store::QuestState::Complete, progress: String::new(), started_at: 0, completed_at: Some(board_now() - 3_600) })
            .unwrap();
        let out = s.handle(&quest_request(1, first, 235));
        assert!(!accepted(&out, first), "a First Greeting never comes back");
    }

    /// A turn-in banks the formula at the current grade, crosses 1000 into grade 2 with the
    /// effect, and Arthur then hands over the certificate - the 0x46 window - once.
    #[test]
    fn a_turn_in_banks_contribution_grades_up_and_the_certificate_waits_at_the_clerk() {
        let (store, mut s, id) = resident(30, HALL_H, 0);
        store.set_citizenship(id, &[active_in(1, 1, 950)]).unwrap();
        let group = &crate::citizenship::BOARD_GROUPS[0];
        let quest = crate::citizenship::fresh_pick(group, &s.config.quests, group.period(board_now()), 1, 30)[1];
        let _ = s.handle(&quest_request(1, quest, 235));
        let out = s.handle(&quest_request(2, quest, 235));
        assert!(out.iter().any(|r| r.opcode == net::quest::MESSAGE && r.body == net::citizenship::contribution_gained(1, 100)), "{out:?}");
        assert_eq!(standing(&store, id), "st1=1;gr1=2;ct1=50", "1,000 spent on the promotion, 50 carried");
        assert_eq!(ex_line(&out, 510_000).as_deref(), Some("st1=1;gr1=2;ct1=50"));
        assert!(has_effect(&out, net::citizenship::EFFECT_CITIZENSHIP_GRADE_UP));

        let out = s.handle(&click(900));
        assert_eq!(window(&out), Some(net::citizenship::CONTRACT_GRADE_UPDATE));
        let _ = s.handle(&answer(0x46, 1));
        let _ = s.handle(&answer(0x47, 0x46));
        assert_eq!(store.citizenship(id).unwrap()[0].certified_grade, 2);
        assert_eq!(window(&s.handle(&click(900))), Some(net::script::SCRIPT_TYPE_MENU), "once");

        let mut row = store.quest_row(id, quest).unwrap().unwrap();
        row.completed_at = Some(board_now() - 3_600);
        store.save_quest(id, &row).unwrap();
        let _ = s.handle(&quest_request(1, quest, 235));
        let out = s.handle(&quest_request(2, quest, 235));
        assert!(out.iter().any(|r| r.body == net::citizenship::contribution_gained(1, 150)), "grade 2 pays 150");
        assert!(!has_effect(&out, net::citizenship::EFFECT_CITIZENSHIP_GRADE_UP), "no grade-up, no effect");
    }

    /// **Contribution is never rated; mesos are.** At a 10x Quest rate a grade-2 daily banks its
    /// own 150 Contribution (the owner reverted the 10x on 2026-10-02) and still pays 3510 mesos
    /// (351 x 10); a story arc step banks its flat 50 and its mesos at the rate like every quest's.
    #[test]
    fn board_quests_bank_flat_contribution_and_pay_mesos_at_the_quest_rate() {
        let (store, mut s, id) = resident(30, HALL_H, 0);
        store.set_rate(store::rates::RateKind::Quest, store::rates::Rate::from_per_cent(1_000), 1).unwrap();
        store.set_citizenship(id, &[active_in(1, 2, 1_000)]).unwrap();
        let group = &crate::citizenship::BOARD_GROUPS[0];
        let quest = crate::citizenship::fresh_pick(group, &s.config.quests, group.period(board_now()), 2, 30)[1];
        let _ = s.handle(&quest_request(1, quest, 235));
        let out = s.handle(&quest_request(2, quest, 235));
        assert!(out.iter().any(|r| r.body == net::citizenship::contribution_gained(1, 150)), "150 at grade 2, NOT x10: {out:?}");
        assert!(!out.iter().any(|r| r.body == net::citizenship::contribution_gained(1, 1_500)), "the old 10x is gone");
        assert_eq!(store.mesos(id).unwrap(), 3_510, "351 x10");
        assert!(out.iter().any(|r| r.body == net::message::meso_gained(3_510)));

        let _ = s.handle(&quest_request(1, STORY_QUEST, 206));
        let out = s.handle(&quest_request(2, STORY_QUEST, 206));
        assert!(out.iter().any(|r| r.body == net::citizenship::contribution_gained(1, 50)), "a story step stays flat");
        assert_eq!(store.mesos(id).unwrap(), 7_020, "its mesos at the rate too");
        let _ = s.handle(&quest_request(2, STORY_QUEST, 206));
        assert_eq!(store.mesos(id).unwrap(), 7_020, "a repeat turn-in pays nothing");
    }

    const HENESYS_EARRINGS: u32 = 1_032_021;

    fn holds(store: &Store, id: u32, item: u32) -> usize {
        store.bag_items(id, store::InventoryType::Equip).unwrap().iter().filter(|r| r.item.item_id == item).count()
    }

    /// **Citizen of Honor** (the owner, 2026-09-29): the turn-in that reaches grade 10 hands over the
    /// town's earring and tells every player - the achiever's own blue `[Notice]` in the
    /// replies, another player's through the bus. Once: grade 10 is reached once per town.
    #[test]
    fn reaching_citizen_of_honor_hands_over_the_earring_and_tells_everyone() {
        let (store, mut ss) = residents(&["Honored", "Onlooker"], 60, HALL_H, 0);
        let id = ss[0].1;
        store.set_citizenship(id, &[active_in(1, 9, 9_950)]).unwrap();
        let group = &crate::citizenship::BOARD_GROUPS[0];
        let quest = crate::citizenship::fresh_pick(group, &ss[0].0.config.quests, group.period(board_now()), 9, 60)[1];
        let _ = ss[0].0.handle(&quest_request(1, quest, 235));
        let out = ss[0].0.handle(&quest_request(2, quest, 235));
        assert_eq!(standing(&store, id), "st1=1;gr1=10;ct1=450", "500 at grade 9 crosses its 10,000; 450 carried");
        assert_eq!(holds(&store, id, HENESYS_EARRINGS), 1, "the Henesys Earrings, in the Equip tab");
        assert!(store.honor_earring_given(id, 1).unwrap());
        let sentence = crate::citizenship::honor_announcement("Honored", 1);
        assert_eq!(sentence, "Let us all congratulate Honored for becoming a Citizen of Honor in Henesys!");
        let blue = net::broadcast::notice(&sentence);
        assert!(out.iter().any(|r| r.opcode == net::broadcast::BROADCAST_MSG && r.body == blue), "{out:?}");
        let heard = ss[1].0.tick(1_000);
        assert!(heard.iter().any(|r| r.opcode == net::broadcast::BROADCAST_MSG && r.body == blue), "the other player: {heard:?}");

        // Grade 10 again (another daily) is not a transition: no second earring, no second notice.
        let mut row = store.quest_row(id, quest).unwrap().unwrap();
        row.completed_at = Some(board_now() - 3_600);
        store.save_quest(id, &row).unwrap();
        let _ = ss[0].0.handle(&quest_request(1, quest, 235));
        let out = ss[0].0.handle(&quest_request(2, quest, 235));
        assert_eq!(holds(&store, id, HENESYS_EARRINGS), 1, "once");
        assert!(!out.iter().any(|r| r.opcode == net::broadcast::BROADCAST_MSG));
    }

    /// A full Equip tab loses nothing: the earring waits, unmarked, and Arthur hands it over on
    /// the next talk - that click's whole answer - and never again.
    #[test]
    fn a_full_equip_tab_leaves_the_earring_with_the_clerk() {
        let (store, mut s, id) = resident(60, HALL_H, 0);
        store.set_citizenship(id, &[active_in(1, 9, 9_950)]).unwrap();
        while store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1_002_000), 1).is_ok() {}
        let group = &crate::citizenship::BOARD_GROUPS[0];
        let quest = crate::citizenship::fresh_pick(group, &s.config.quests, group.period(board_now()), 9, 60)[1];
        let _ = s.handle(&quest_request(1, quest, 235));
        let out = s.handle(&quest_request(2, quest, 235));
        assert_eq!(holds(&store, id, HENESYS_EARRINGS), 0);
        assert!(!store.honor_earring_given(id, 1).unwrap(), "not marked - it is still owed");
        assert!(out.iter().any(|r| r.what.contains("earring is waiting")), "{out:?}");
        assert!(out.iter().any(|r| r.opcode == net::broadcast::BROADCAST_MSG), "the achievement is announced anyway");

        let first = store.bag_items(id, store::InventoryType::Equip).unwrap()[0].slot;
        store.remove_item(id, store::InventoryType::Equip, first, None).unwrap();
        let out = s.handle(&click(900));
        assert_eq!(holds(&store, id, HENESYS_EARRINGS), 1, "Arthur hands it over");
        assert_eq!(window(&out), Some(net::script::SCRIPT_TYPE_SAY));
        assert!(store.honor_earring_given(id, 1).unwrap());
        let out = s.handle(&click(900));
        assert_ne!(window(&out), Some(net::script::SCRIPT_TYPE_SAY), "next time, the ordinary clerk");
        assert_eq!(holds(&store, id, HENESYS_EARRINGS), 1);
    }

    /// **One weekly per character per week, at the tier reached when the week began** (the
    /// owner, 2026-10-01). A grade-up mid-week posts nothing new and the gate refuses the next
    /// tier's donation; after the turn-in the group posts nothing at all until Monday.
    #[test]
    fn a_weekly_is_one_per_week_and_a_grade_up_waits_for_monday() {
        let (store, mut s, id) = resident(30, HALL_H, 0);
        store.set_citizenship(id, &[active_in(1, 1, 0)]).unwrap();
        let group = &crate::citizenship::BOARD_GROUPS[1];
        let period = group.period(board_now());
        let tier1 = crate::citizenship::fresh_pick(group, &s.config.quests, period, 1, 30)[0];
        let tier4 = crate::citizenship::fresh_pick(group, &s.config.quests, period, 4, 30)[0];
        let (book, _) = s.quest_book(id);
        assert!(book.ex.contains(&(510_002, format!("q1_w={tier1}"))), "{:?}", book.ex);

        store.set_citizenship(id, &[active_in(1, 4, 3_000)]).unwrap();
        let out = s.handle(&quest_request(1, tier4, 235));
        assert!(!accepted(&out, tier4), "grade 4 since Monday: still this week's tier-1 donation");
        assert!(accepted(&s.handle(&quest_request(1, tier1, 235)), tier1));
        let out = s.handle(&quest_request(2, tier1, 235));
        assert_eq!(ex_line(&out, 510_002).as_deref(), Some("q1_w="), "turned in: nothing more this week");
        let out = s.handle(&quest_request(1, tier4, 235));
        assert!(!accepted(&out, tier4));
        assert!(out.iter().any(|r| r.what.contains("already done this week")), "{out:?}");
    }

    /// **A Yes on the donation's turn-in talk is one box, and accepts nothing.** The live server,
    /// 2026-10-02: a weekly was turned in from the quest window, the completion talk asked
    /// "...are you saying you'd like to donate?", and the Yes was taken as ACCEPTING the quest
    /// again - the start gate refused it and the `1.yes` line followed the refusal, two script
    /// boxes at once. The client rejected the second (`0x009E`) and dropped. Replayed here with
    /// the captured answer body (`00000000 10 01`: handle 0, quest yes/no, Yes).
    #[test]
    fn a_yes_on_the_donation_turn_in_talk_is_one_box_and_accepts_nothing() {
        let (store, mut s, id) = resident(30, HALL_H, 0);
        store.set_citizenship(id, &[active_in(1, 1, 0)]).unwrap();
        let group = &crate::citizenship::BOARD_GROUPS[1];
        let weekly = crate::citizenship::fresh_pick(group, &s.config.quests, group.period(board_now()), 1, 30)[0];
        assert!(accepted(&s.handle(&quest_request(1, weekly, 235)), weekly));
        // Since 2026-10-02 the turn-in itself no longer re-asks: the client asked "...are you
        // saying you'd like to donate?" before it sent this. The answer is the thank-you.
        let out = s.handle(&quest_request(2, weekly, 229));
        let boxes: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).collect();
        assert_eq!(boxes.len(), 1, "{out:?}");
        assert!(boxes[0].what.contains("path \"1.yes\"") && !boxes[0].what.contains("yes/no"), "the thank-you, not the question again: {}", boxes[0].what);

        // And a Yes arriving anyway (the old talk still open on a client) is still one box at most.

        let mut yes = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        yes.extend_from_slice(&[0, 0, 0, 0, 0x10, 0x01]);
        let out = s.handle(&yes);
        let boxes: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).collect();
        assert!(boxes.len() <= 1, "never two script boxes at once - that is what dropped the client: {out:?}");
        assert!(!out.iter().any(|r| r.what.contains("not accepted")), "nothing tried to accept it again: {out:?}");
        assert_eq!(store.quest_row(id, weekly).unwrap().unwrap().state, store::QuestState::Complete);
    }

    /// The same for a daily: turned in, then a grade-up to 5 offers no leader today.
    #[test]
    fn a_daily_turned_in_is_the_days_last_even_after_a_grade_up() {
        let (store, mut s, id) = resident(40, HALL_H, 0);
        store.set_citizenship(id, &[active_in(1, 4, 3_990)]).unwrap();
        let group = &crate::citizenship::BOARD_GROUPS[0];
        let period = group.period(board_now());
        let resident_pair = crate::citizenship::fresh_pick(group, &s.config.quests, period, 4, 40);
        let leader = crate::citizenship::fresh_pick(group, &s.config.quests, period, 5, 40);
        assert!(accepted(&s.handle(&quest_request(1, resident_pair[1], 235)), resident_pair[1]));
        let out = s.handle(&quest_request(2, resident_pair[1], 235));
        assert_eq!(standing(&store, id), "st1=1;gr1=5;ct1=240", "graded up to 5 on that turn-in");
        assert_eq!(ex_line(&out, 510_001).as_deref(), Some("q1_d="), "and the daily board is empty");
        let out = s.handle(&quest_request(1, leader[1], 235));
        assert!(!accepted(&out, leader[1]), "no leader daily after today's daily");
    }

    /// **The live server's characters**: three donations in progress from before the rule,
    /// the top one needing level 22. The next field entry keeps the one the character can
    /// finish at the highest tier, gives up the rest, and the quest book never shows them.
    #[test]
    fn extra_weeklies_in_progress_are_given_up_at_the_next_field_entry() {
        let (store, s, id) = resident(21, HALL_H, 0);
        store.set_citizenship(id, &[active_in(1, 3, 2_500)]).unwrap();
        for quest in [506_019, 506_025, 506_026] {
            assert!(store.start_quest(id, quest).unwrap());
        }
        let (book, _) = s.quest_book(id);
        let started: Vec<u32> = store
            .quest_rows(id)
            .unwrap()
            .into_iter()
            .filter(|r| r.state == store::QuestState::InProgress)
            .map(|r| r.quest_id)
            .collect();
        assert_eq!(started, vec![506_025], "the level-17 donation stays; 506019 and the level-22 one go");
        assert_eq!(book.started.len(), 1, "the journal the client gets agrees");
        assert!(book.ex.contains(&(510_002, "q1_w=506025".to_string())), "{:?}", book.ex);
    }

    /// The field entry's quest book carries quest 510000 and all four board postings.
    #[test]
    fn the_set_field_record_carries_the_citizenship_and_the_board() {
        let (store, s, id) = resident(30, HALL_H, 0);
        let (book, _) = s.quest_book(id);
        assert_eq!(book.ex.iter().map(|(q, _)| *q).collect::<Vec<_>>(), vec![510_001, 510_002, 510_003, 510_004], "no citizenship, no 510000");
        store.set_citizenship(id, &[active_in(2, 1, 0)]).unwrap();
        let (book, _) = s.quest_book(id);
        assert_eq!(book.ex[0], (510_000, "st2=1;gr2=1;ct2=0".to_string()));
        assert_eq!(book.ex.len(), 5);
    }

    /// The day turns under a player who has not moved: the tick re-sends what changed, once.
    #[test]
    fn the_board_is_re_sent_when_the_day_turns() {
        let (_store, mut s, id) = resident(30, HALL_H, 0);
        let _ = s.quest_book(id);
        assert!(s.board_tick().is_empty(), "same day");
        let stale: Vec<(u32, String)> = (510_001..=510_004).map(|q| (q, "q1_d=1".to_string())).collect();
        *s.board_sent.borrow_mut() = Some((store::dailyperks::today() - 1, stale));
        assert_eq!(s.board_tick().len(), 4, "all four differ from the stale ones");
        assert!(s.board_tick().is_empty(), "and not again");
    }

    /// Flint's grade-locked row goes out with its town and grade and is refused to a
    /// non-citizen who asks anyway; the open row still sells; a Town Resident may buy it.
    #[test]
    fn a_grade_locked_row_is_marked_and_refused() {
        let (store, mut s, id) = resident(30, HALL_H, 10_000);
        let out = s.handle(&click(901));
        assert!(out.iter().any(|r| r.opcode == net::classicshop::CLASSIC_OPEN_SHOP), "the shop opens");
        let rows = s.open_shop.clone().unwrap().1;
        assert_eq!((rows[1].citizenship_town, rows[1].citizenship_grade), (1, 5));
        assert_eq!((rows[0].citizenship_town, rows[0].citizenship_grade), (0, 0));

        let buy = |row: u16, item: u32| {
            let mut b = net::classicshop::CLIENT_CLASSIC_SHOP_REQUEST.to_le_bytes().to_vec();
            b.push(0);
            b.extend_from_slice(&row.to_le_bytes());
            b.extend_from_slice(&item.to_le_bytes());
            b.extend_from_slice(&1u16.to_le_bytes());
            b
        };
        let _ = s.handle(&buy(1, LOCKED_SCROLL));
        assert_eq!(store.mesos(id).unwrap(), 10_000, "refused: not a citizen");
        let _ = s.handle(&buy(0, OPEN_POTION));
        assert_eq!(store.mesos(id).unwrap(), 9_900, "the open row sells");
        store.set_citizenship(id, &[active_in(1, 5, 4_000)]).unwrap();
        let _ = s.handle(&buy(1, LOCKED_SCROLL));
        assert_eq!(store.mesos(id).unwrap(), 9_800, "a Town Resident may");
    }
}
