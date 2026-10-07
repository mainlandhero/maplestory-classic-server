//! Talking to people: the two click packets, the Say state machine, and the quest
//! journal that comes out of pressing Accept.
//!
//! Dialogue and quests share this file because they share `Conversation`: which line an
//! NPC says next depends on whether a quest branch was taken, and splitting them would
//! put one state machine in two places.

use super::*;

impl Session {

    /// Make the NPC the client just asked about say something.
    ///
    /// **`0x0151` is the quest request, not an NPC click.** Its first `u32` is a *quest id*
    /// and its second is the NPC **template** id - the same template the server sent in
    /// `NpcEnterField`. An earlier note in `STATUS.md` read the first field as our own
    /// object id; our own logs disprove it, because every map's first NPC is given object
    /// id 1000 and the client answered 1000/1002/1003/1005 for four different NPCs, the
    /// same values in both sessions despite opposite visit orders. Full working:
    /// `research/npc-dialogue.md`.
    ///
    /// **The speaker is the template the client named**, which is what makes this safe: it
    /// is by construction a real `Npc.wz` id, and a bad one costs the portrait rather than
    /// faulting.
    ///
    /// **This is text on screen, not a quest.** No quest-result packet has been found, so
    /// nothing here advances any state - accepting the same quest twice will show the same
    /// message. Say so in the message rather than letting the screen imply otherwise.
    ///
    /// **Ordering.** A script must never be sent with or just before a `SetField`: field
    /// entry runs `FUN_142caa4e0`, which resets the script manager and tears the dialog
    /// down silently. This path is a reply to a click, which is long after field entry, so
    /// it is clear - but the constraint is why this does not simply fire on arrival.
    pub(super) fn on_quest_request(&mut self, body: &[u8]) -> Vec<Reply> {
        // **Giving up a quest is action 3, and its body is FIVE bytes.** It has to be split
        // off before `parse_quest_request`, which needs a 9-byte head and returns `None` for
        // a forfeit - and `None` here is silence on the wire. That is exactly what two
        // captures show: the owner pressed give up, the packet arrived, and nothing came back, so
        // they pressed it again after a relog. `research/quest-forfeit.md`.
        //
        // Neither `0x01ED` nor `0x01A5` was the forfeit, though both went unanswered four
        // times in the same run and both looked like obvious candidates. `0x01ED` is the
        // client's own usage/log channel - the same two bodies appear byte-for-byte in a
        // capture from 2026-08-19 that contains **no quest traffic at all**, because the
        // subsystem did not exist yet.
        if net::questforfeit::is_forfeit(body) {
            return self.record_quest_forfeit(body);
        }
        // Always answer. An unanswered request freezes the client's whole UI, so a body
        // that does not parse still gets a reply - parse_quest_request only returns None
        // when the fixed 9-byte head does not fit, and then there is no template to speak
        // as, which is the one case where silence is all there is.
        let Some(req) = net::script::parse_quest_request(body) else {
            return Vec::new();
        };

        // **Phil is the job guide, and a click on them arrives as EITHER packet.** Measured:
        // `0x0151` while "Phil's Call" (10001) is offerable or completable, `0x00F2` once it
        // is finished - previous-runs/world-20260821-230104.log, 02:50:44 and 02:50:56. A
        // patch that hooked only `on_npc_click` would do nothing for a beginner who has just
        // met them.
        //
        // It must NOT swallow the quest: 10001 pays exp, mesos and ten potions, and its own
        // last line is *"come talk to me again"*. The quest runs first; the menu takes over
        // only once there is nothing left for it to say.
        if req.npc_template_id == crate::jobguide::PHIL_TEMPLATE && self.phil_has_nothing_left(&req) {
            if let Some(replies) = self.phil_job_guide() {
                return replies;
            }
        }
        // **There is deliberately no branch on the Maple Administrator here.** There was one for
        // a day: template 9010000 was claimed for `crate::dailyperks` and quest
        // `crate::dailyperks::ADMIN_QUEST` (500005) was never offered or turned in. The owner,
        // 2026-09-08: *"instead of losing the Maple Admin quest. Introduce a new public command
        // !tool"*. So they fall through to the ordinary quest path below like every other NPC,
        // and the favours are reached by typing the command - `Session::open_daily_perks`.
        //
        // Removing the branch is the whole restoration: 500005 has no `Say.0` in this client's
        // `Quest.wz` (only `Say.1.stop.item.0`), so the chain below lands on `path = None` and
        // their own `d0` - *"Hello! Welcome to Maple World!"* - which is exactly what they did
        // before the repurposing.

        // Which half of the quest's Say tree the action selects. With no quest state, a
        // start and an opening script both land on "0".
        let state = match req.action {
            net::script::QUEST_ACTION_COMPLETE | net::script::QUEST_ACTION_COMPLETE_SCRIPT => "1",
            _ => "0",
        };
        let quest = self.config.quests.get(&req.quest_id);

        // **The client runs the opening conversation itself, and re-sending it is a loop.**
        // The owner, 2026-08-19: *"Clicking 'Accept' starts the 'You must be the new traveler'
        // conversation again. That portion is incorrect, as the 'You must be the new
        // traveler' exists and gets handled on client side."* So `0x0151` is not "tell me
        // what this NPC says" - by the time it arrives the client has already shown the
        // opening and the user has pressed a button. Action **1** is that press, and what it
        // wants back is the **`yes` branch**: for quest 1000, "Thank you. #p2# is on the
        // hill to the east...".
        //
        // The decline branch is in the WZ too (`Say.0.no`) and the client very likely shows
        // it locally, the way it shows the opening - but no capture contains a decline, so
        // that is **[I]** and this does not act on it.
        let accepted = req.action == net::script::QUEST_ACTION_START;
        let completing = state == "1";
        let branch = format!("{state}.yes");
        // An unknown quest falls back to the NPC's own line - a one-line conversation
        // rather than silence.
        //
        // **The `"0"` fallback must not apply to a completion**, and that cost a run.
        // The owner clicked Sera holding quest 1000 and they recited Heena's tutorial: *"You must
        // be the new traveler..."*. The capture says exactly why -
        //
        //   <- 0x0151  02 e8030000 02000000     action 2, quest 1000, npc 2
        //   -> 0x055B  ... quest 1000, line 1 of 4 on path "0"
        //
        // - because quest 1000 has **no `Say.1`**. Its only `1.*` node is `1.stop.npc`, so
        // `contains_key("1")` is false and the old chain fell through to `"0"`, the opening.
        // Falling back to the *start* of a quest when asked to *finish* it is never right:
        // it is a conversation the player has already had, from the wrong NPC.
        //
        // What quest 1000 has instead is `Act.1.nextQuest = 1001`, and 1001's `Say.0` is the
        // line the owner expected - *"How am I going to hang all these up?"*. So a completion with
        // no completion dialogue **chains**, which is what the data is describing.
        let mut speaking_quest = req.quest_id;
        let path = match quest {
            Some(q) if accepted && q.say.contains_key(&branch) => Some(branch),
            // **An accept with no `yes` branch says NOTHING.** The owner, 2026-09-13: *"Nina's
            // quest dialogue seems to be repeated when accepting their 'What Sen wants to eat'
            // quest. They say the same two dialogues before and after I click 'Accept'."*
            // Quest 1003 has `Say.0` (two lines) and `Say.0.no`, and no `Say.0.yes`; the arm
            // below this one then chose `"0"` - the opening the client had just shown on its
            // own, which is the loop the comment above describes. world.log 04:06:17: the
            // 0x0151 accept, the record, then "line 1 of 2 on path 0" again. The client's own
            // window closes on Accept and the record is already on the wire, so there is
            // nothing a box has to say and no NPC d0 fallback either - `silent_accept`
            // returns before `say_line`.
            Some(_) if accepted => None,
            // **A turn-in is answered the way an accept is: the CLIENT already said `Say.1`.**
            // Players, 2026-10-02: *"quests are repeating lines in general"*. [L] from the
            // client's own request builder `FUN_141f0e4c0` (decompiled 2026-10-02,
            // `research/quest-dialogue-who-speaks-2026-10-02.md`): for state 0 AND state 1 it
            // loads `Say/<state>`, runs the local dialog over its lines (`FUN_141f13360`, or
            // `FUN_141f146a0` when the node has `ask`), and only then builds `0x0151` - action 1
            // for an offer, action 2 for a turn-in (with `FUN_141f15fb0` reading a reward pick off
            // the last line). The one dialog-free action-2 form is gated on `autoCompleteAction`,
            // which no quest in this client carries. So `Say.1` reached the screen twice - and on
            // a weekly donation its yes/no twice, which is how one player was dropped. The answer
            // is the `yes` branch, the same as an accept's `0.yes` (measured on screen
            // 2026-08-20), or nothing; a quiz (`ask` on `1`) has no `1.yes` and stays silent.
            Some(q) if req.action == net::script::QUEST_ACTION_COMPLETE && q.say.contains_key(state) => {
                (!q.say.contains_key(&format!("{state}.ask")) && q.say.contains_key(&branch)).then_some(branch)
            }
            Some(q) if q.say.contains_key(state) => Some(state.to_string()),
            // A completion whose quest has nothing to say hands over to the next quest in
            // the chain, and the conversation belongs to THAT quest from here on - the
            // client is told a quest id and it has to be the one whose lines these are.
            Some(q) if completing => match q.next_quest {
                Some(next) if self.config.quests.get(&next).is_some_and(|n| n.say.contains_key("0")) => {
                    speaking_quest = next;
                    Some("0".to_string())
                }
                _ => None,
            },
            Some(q) if q.say.contains_key("0") => Some("0".to_string()),
            _ => None,
        };
        // Known quest, Accept pressed, nothing to say: see the `Some(_) if accepted` arm. An
        // UNKNOWN quest is not this - it keeps the NPC's own line, so an accept the data
        // cannot explain still draws something rather than nothing.
        // ...and the same for a turn-in with nothing of its own to say and nothing to chain
        // to: the record goes out, the client's quest window closes, and the NPC's d0
        // greeting is not what a finished quest sounds like. Found by the 2026-09-13 audit
        // (`no_quest_answers_its_accept_or_turn_in_with_its_own_opening_lines`): quest 1002.
        let silent_accept = (accepted || completing) && quest.is_some() && path.is_none();
        // **A quiz is conducted by the CLIENT; the turn-in only finalises it.** The owner,
        // 2026-09-13, with five timestamped screenshots and the log beside them: for quest
        // 1016 the client drew the offer, the question and a "Yes, that's correct!" box
        // entirely on its own - `world.log` has ZERO inbound quest/script packets for the 41 s
        // those boxes were up (17:20:14..55, only toggles and telemetry) - and only THEN sent
        // the turn-in. The server used to answer that turn-in by asking the same question a
        // SECOND time (17:20:55 menu, 17:21:06 answer), so the player answered twice. The
        // client has the quest's `#L` choices and its `stop.0.answer` in `Quest.wz` and grades
        // them itself; it sends the turn-in only on a right answer, and when the server sent a
        // redundant menu the client dismissed it unanswered (`06 00`, world.log 16:58:34). So
        // a quiz turn-in records the completion and says nothing - the closing line is already
        // on screen. `quiz_answer_key` marks the shape: a completion path whose node has `ask`.
        let quiz_turn_in = completing
            && path.as_deref() == Some(state)
            && quest.is_some_and(|q| q.say.contains_key(&format!("{state}.ask")));
        self.conversation = Some(Conversation {
            npc_template: req.npc_template_id,
            quest_id: path.as_ref().map(|_| speaking_quest),
            path: path.unwrap_or_default(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        // **The acceptance is written down HERE**, and it took a run to find out why.
        //
        // `Session::accept_quest` was wired into `on_script_reply`'s yes/no branch, which
        // is where a `Yes` on a *server-driven* prompt would land. The client's Accept
        // button does not go there. The capture of 2026-08-20 is unambiguous:
        //
        //   <- 0x0151 CLIENT_QUEST_REQUEST  01 e8030000 01000000 43ffe501 00000000
        //                                   ^ action 1  ^ quest 1000
        //   -> 0x055B ScriptMessage Say ... for quest 1000 ... on path "0.yes"
        //
        // The server answered the `yes` branch - the dialogue was right, it always had
        // been - and never touched `quest_state`. The owner: *"I have tried accepting Heena's
        // quest, but unfortunately I do not see the quest as accepted in my quest book.
        // Changing maps also does nothing to help that."*
        //
        // The lesson is the one CLAUDE.md keeps making: the branch we were watching was
        // the one we had built, not the one the client uses.
        let mut out = Vec::new();
        // **Where the character was before any of this ran.** A quest that starts the
        // second-job test warps them, and a script box sent after a `SetField` is torn down
        // by field entry - so the closing `say_line` has to be skipped in exactly that case.
        // Comparing the map is deliberate rather than setting a flag: it is true for any
        // future quest that moves somebody, including ones nobody has written yet.
        let map_before = self.claimed_character().map(|c| c.map_id);
        if accepted {
            out.extend(self.record_quest_start(req.quest_id, req.npc_template_id));
        }
        // The turn-in records the completion - the record, the exp and the fanfare - for a
        // quiz exactly as for any other quest. What differs is only what is SAID afterwards:
        // a quiz says nothing (the client already drew the closing line), which the
        // `quiz_turn_in` arm below handles by returning right after this.
        if completing {
            out.extend(self.record_quest_complete(req.quest_id, speaking_quest));
        }
        let map_after = self.claimed_character().map(|c| c.map_id);
        if map_before != map_after {
            self.conversation = None;
            return out;
        }
        // **The bag was full and the quest did not move.** `record_quest_start` /
        // `record_quest_complete` parked the NPC's bag-full box; the closing line below
        // would say "well done, here you are" over a quest that is still in progress.
        if self.conversation.as_ref().is_some_and(|c| c.path == crate::questroom::REFUSAL_PATH) {
            return out;
        }
        // An accept whose quest has no `yes` branch: the record went out, the client's own
        // quest window has closed, and there is no line to put in a box - not the opening
        // again, and not the NPC's d0 greeting that `say_line` falls back to without a quest.
        //
        // A quiz turn-in is the same shape for the same reason: the client conducted the quiz
        // and drew its own "That's right!" box, so the completion record is the whole answer
        // and a `say_line` here would repeat the closing line - the second box the owner saw.
        if silent_accept || quiz_turn_in {
            self.conversation = None;
            return out;
        }
        // **The chain's last quest ends in a choice, not a line.** Turning in `The Proof of a
        // Hero` at the instructor is the moment the second advancement is earned; putting the
        // menu here means the player does not have to click the same NPC a second time to be
        // asked which path they want.
        //
        // **It is the LAST quest of the chain and no other.** `20000` is turned in at the
        // same NPC and ships its own `Say.1` - *"Whoa, you've definitely grown up!"* - and an
        // instructor test rather than a quest-id test would replace that line with a refusal
        // saying the player has no proof, which is true and is the wrong thing to say to
        // somebody who has just started the chain. Three of the four chain quests end
        // somewhere this must not fire.
        let ends_the_chain = crate::secondjob::branch_at(req.npc_template_id)
            .is_some_and(|b| b.chain.quests[3] == req.quest_id);
        if completing && ends_the_chain {
            if let Some(menu) = self.second_advancement_for(req.npc_template_id) {
                out.extend(menu);
                return out;
            }
        }
        out.extend(self.say_line(0));
        out
    }


    /// Write a completion down, and start the quest it chains into.
    ///
    /// `chained_to` is the quest whose dialogue is about to be spoken - the same id as
    /// `finished` when the quest has its own completion lines, and the next one in the chain
    /// when it does not.
    ///
    /// **The chained quest is started here rather than waiting for the client to ask.** The
    /// client is already mid-conversation with the NPC that starts it, and the capture shows
    /// it sends no second `0x0151`; if the server does not write the row, the player finishes
    /// the conversation holding neither quest.
    pub(super) fn record_quest_complete(&mut self, finished: u32, chained_to: u32) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = Vec::new();
        // **Room first, before the row moves.** Mint, 2026-09-17: *"quest continues to
        // complete despite this happening"* - quest 1008 wrote its completion, paid its
        // experience, took the letter back, and the store refused the hat into a full Equip
        // tab. The reward rows are drawn NOW (a `prop` pool is one roll), checked against the
        // bag as it is, and handed over below exactly as drawn. A shortfall is the NPC's
        // bag-full box and nothing else: no record, no experience, no take. `crate::questroom`.
        let chosen = self
            .config
            .quests
            .get(&finished)
            .map(|q| {
                let roll = self.rng.next();
                let chosen = crate::config::choose_rewards(&q.complete_rewards, chr.gender, roll);
                if chosen.len() != q.complete_rewards.len() {
                    crate::server::log(&format!(
                        "   quest {finished}: {} of {} reward rows handed over - {} prop-marked item(s) form a pool and ONE was drawn ({:?}); gender {} filtered the rest",
                        chosen.len(),
                        q.complete_rewards.len(),
                        q.complete_rewards.iter().filter(|r| r.prop > 0).count(),
                        chosen.iter().filter(|r| r.prop > 0).map(|r| r.id).collect::<Vec<_>>(),
                        chr.gender
                    ));
                }
                chosen
            })
            .unwrap_or_default();
        // Only an in-progress quest can be short of room: a repeat click on a finished quest
        // hands nothing over and must stay the silent no-op it is below.
        let in_progress = self
            .store
            .quest_row(chr.id, finished)
            .ok()
            .flatten()
            .is_some_and(|r| r.state == store::QuestState::InProgress);
        if in_progress {
            let short = self.quest_room_shortfall(&chr, &chosen);
            if !short.is_empty() {
                let speaker = self
                    .config
                    .quests
                    .get(&finished)
                    .and_then(|q| q.end_npc.or(q.start_npc))
                    .unwrap_or(0);
                return self.bag_full_refusal(finished, speaker, &short, "completed");
            }
        }
        // A turn-in that costs mesos (`Act.1.money` < 0): taken now, refused when short, and
        // handed back below if the completion is not recorded. `session/questmoney.rs`.
        let charged = if in_progress {
            let speaker = self.config.quests.get(&finished).and_then(|q| q.end_npc.or(q.start_npc)).unwrap_or(0);
            match self.take_quest_cost(finished, 1, speaker, "completed") {
                Ok(charged) => charged,
                Err(refusal) => return refusal,
            }
        } else {
            0
        };
        // Only a completion that was actually recorded earns a fanfare. Playing one for a
        // quest the character never started would be a sound with nothing behind it.
        let mut recorded = false;
        match self.store.complete_quest(chr.id, finished) {
            // `complete_quest` returns the FILETIME it stamped, which is the same value the
            // journal has to carry - re-deriving it here would put a different instant in the
            // packet from the one in the database.
            Ok(Some(at)) => {
                recorded = true;
                out.push(Reply {
                    opcode: net::quest::MESSAGE,
                    body: net::quest::quest_completed(finished, at as u64),
                    what: format!("quest {finished} completed for character {}", chr.id),
                })
            }
            // **A repeat turn-in is silent, and it is NOT an error.** Clicking an NPC again
            // after finishing its quest is an ordinary thing for a player to do; the
            // conversation that follows is the whole answer. What must not happen is a
            // payout, which is what used to happen - see below.
            //
            // Being asked to complete a quest that was never *started* is different: the two
            // books disagree, and that is worth a line on screen.
            Ok(None) => {
                let started = self
                    .store
                    .quest_row(chr.id, finished)
                    .ok()
                    .flatten()
                    .is_some();
                if !started {
                    out.push(Reply {
                        opcode: net::notice::CHAT_NOTICE,
                        body: net::notice::chat_notice(&format!(
                            "The server was asked to complete quest {finished}, which this character has not started."
                        )),
                        what: format!("quest {finished} completed but no row existed"),
                    });
                }
            }
            Err(e) => out.push(Reply {
                opcode: net::notice::CHAT_NOTICE,
                body: net::notice::chat_notice("Could not record that quest."),
                what: format!("quest {finished} completion NOT STORED: {e}"),
            }),
        }
        // **Everything below hangs off `recorded`, and that is the whole fix.** The owner,
        // 2026-08-21: *"I was able to complete the Heena quest multiple times, this is not
        // okay."*
        //
        // `store::complete_quest` guards on `state = InProgress` and had always refused a
        // second turn-in correctly. The refusal was then ignored: this payout sat *outside*
        // the match, so a repeat click re-paid `Act.1` - two experience per click on quest
        // 1001, for as many clicks as the player liked - while the journal row and the
        // fanfare, which were gated, stayed right.
        //
        // That asymmetry is why the existing turn-in test never caught it: it counted
        // fanfares, and the fanfare was the one effect that was correct. The store was the
        // authority all along and three call sites out of four were asking it.
        if !recorded {
            self.refund_quest_cost(finished, charged);
            return out;
        }
        out.extend(self.quest_cost_replies(finished, charged));
        out.extend(self.apply_quest_completion_rewards(finished, chosen));
        // The second-job test's 30 marbles - its data asks for them and never takes them.
        out.extend(self.take_test_marbles(finished));
        // `Act.1.citizenshipContr` - Contribution, and a grade-up when it crosses one.
        // After the rewards, so the item and EXP lines come first. `session/citizenship.rs`.
        out.extend(self.bank_citizenship_contribution(finished));
        // `Act.1.money`, at the Quest rate like the EXP - every quest (the owner, 2026-09-28).
        // Nothing paid it before. session/questmoney.rs.
        out.extend(self.pay_quest_mesos(finished, 1));
        // `Act.1.pop` - fame (the owner, 2026-10-04). Nothing paid it before. session/fame.rs.
        out.extend(self.pay_quest_fame(finished));
        // **The turn-in fanfare.** The owner, 2026-08-21: *"Quest finish still does not trigger
        // the SFX for quest finish."* It did not, because nothing sent one.
        //
        // `0x02D1` UserEffectLocal, one body byte, effect **15** = QuestClear. Pinned off
        // this client: `Sound/Game.img/QuestClear` has exactly two readers, and the one that
        // is not the pet-skill notice is the packet-driven effect handler `FUN_1427863f0`,
        // whose second switch sends index 15 to an arm that plays the animation and then
        // `FUN_1429f14c0(L"QuestClear", 100)`. `research/quest-complete-effect.md`.
        //
        // **Expect sound and no picture.** `Effect/BasicEff.img/QuestClear` is the one
        // referenced BasicEff node this client's WZ does not contain. The sound call is
        // unconditional after the animation call, so the fanfare still plays.
        //
        // Last, deliberately: it lands on a journal row that already says complete, on items
        // already given and on an EXP line already posted.
        //
        // No `if recorded` here any more - the early return above already guarantees it, and
        // a condition that can never be false reads like a guard while protecting nothing.
        out.push(Reply {
            opcode: net::questeffect::USER_EFFECT_LOCAL,
            body: net::questeffect::quest_clear_local(),
            what: format!(
                "UserEffectLocal QuestClear for quest {finished} - plays Sound/Game.img/QuestClear. The animation node Effect/BasicEff.img/QuestClear is NOT in this client's WZ, so sound and no picture is the EXPECTED result."
            ),
        });
        if chained_to != finished {
            out.extend(self.record_quest_start(chained_to, 0));
        }
        out
    }


    /// `0x0151` action 3 - the player pressed **give up** in the quest window.
    ///
    /// **The client cannot clear its own journal.** `tools/callers.py` on the started-map
    /// erase reports two functions, zero tail jmps and zero data pointers: the record decoder
    /// and the `0x0089` sub-1 handler. The forfeit path reaches neither, so the row only ever
    /// disappears when the server answers. **[L]**
    ///
    /// Two decisions, not details:
    ///
    /// * **`forget_completion = false`.** A forfeit undoes an *acceptance*. It must not
    ///   silently wipe a completion the player earned.
    /// * **The reply goes out even when the store had no row.** The thing being fixed is the
    ///   client's journal, and the case where the two books disagree is precisely the case
    ///   that matters.
    ///
    /// A forfeit can never reset a **completed** quest: the client's builder walks its own
    /// started map and returns without building anything if the id is not there. Re-running a
    /// finished chain is a server-side job, not a client request.
    pub(super) fn record_quest_forfeit(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(req) = net::questforfeit::parse_forfeit_request(body) else {
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let what = match self.store.forget_quest(chr.id, req.quest_id) {
            Ok(true) => format!(
                "quest {} given up by character {} ({}) and the row is gone",
                req.quest_id, chr.id, chr.name
            ),
            // `forget_quest` returns false for two different situations and they must not
            // be reported as one. "No row" is the books disagreeing; "already complete" is
            // the store refusing on purpose, and reporting that as a missing row would send
            // the next reader looking for a persistence bug that is not there.
            Ok(false) => match self.store.quest_row(chr.id, req.quest_id) {
                Ok(Some(row)) if row.state == store::QuestState::Complete => format!(
                    "quest {} give-up REFUSED for character {}: it is already complete, and a forfeit undoes an acceptance rather than a completion. The record is still sent so the journal agrees",
                    req.quest_id, chr.id
                ),
                _ => format!(
                    "quest {} given up but we had no row for character {}; the record is sent anyway so the client's journal agrees with ours",
                    req.quest_id, chr.id
                ),
            },
            Err(e) => format!(
                "quest {} given up but NOT REMOVED ({e}) - the journal clears now and the quest comes back on the next SetField",
                req.quest_id
            ),
        };
        vec![Reply {
            opcode: net::quest::MESSAGE,
            body: net::questforfeit::forfeit_reply(req.quest_id, false),
            what,
        }]
    }


    /// Pay out `Act.1` - the experience, and whatever the quest gives or takes back.
    ///
    /// The owner, 2026-08-21: *"Completing the quest 'Sera's Mirror' did not award me the quest
    /// reward as indicated."* It did not, because completion recorded the row and stopped.
    /// Quest 1001's `Act.1` is `exp 2` and `item 4031000 count -1` - two experience, and
    /// Heena keeps the mirror.
    ///
    /// **A negative count is a take and is now honoured**, which it was not when the giving
    /// direction went in. Taking what the player does not have is not an error: the quest is
    /// already being completed and refusing here would leave it half finished, so a missing
    /// item is reported and the completion stands.
    /// `chosen` is what `record_quest_complete` drew and checked room for - **one** of the
    /// `prop`-marked items (the owner, 2026-09-13: *"Maria gave me one of every single Headband
    /// item when it's suppose to be choose 1 randomly from the pool"*; `choose_rewards`), and
    /// the same rows the bag was measured against, so the hand-out cannot differ from the
    /// check.
    fn apply_quest_completion_rewards(&mut self, quest_id: u32, chosen: Vec<crate::config::RewardItem>) -> Vec<Reply> {
        let Some(quest) = self.config.quests.get(&quest_id) else { return Vec::new() };
        let exp = quest.complete_exp;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = Vec::new();
        for crate::config::RewardItem { id: item_id, count, .. } in chosen {
            let Some(inv) = store::InventoryType::for_item(item_id) else { continue };
            if count > 0 {
                let max_stack = self.config.shops.max_stack(item_id);
                let item = if inv == store::InventoryType::Equip {
                    store::Item::equip(item_id)
                } else {
                    store::Item::bundle(item_id, count.min(u16::MAX as i32) as u16)
                };
                match self.store.add_item(chr.id, inv, &item, max_stack) {
                    Ok(placed) => {
                        out.extend(self.inventory_added_replies(inv, &placed, "a quest reward"));
                        // **A quest's item line goes to the CHAT LOG, and it is a different
                        // opcode from the pick-up line.** The owner, 2026-08-21: *"Quest EXP and
                        // items should show up in the chat log as a gray text ... '<Item>
                        // x<quantity> earned. (<Tab>)'"*.
                        //
                        // `0x0089`'s item sub-case cannot do it: its only chat-log call is
                        // gated on the map's `fieldType == 0x56`, and none of this client's
                        // 426 maps has that type. All 36 sub-cases were enumerated and none
                        // draws that string into the chat - a verified negative, with the
                        // string resolver's 6994 call sites back-resolved and its 46
                        // unresolvable ones reported rather than dropped.
                        //
                        // The route is **`0x02D1` effect 8**, which posts at chat category
                        // **6** with the item name as a link - and category 6's colour
                        // constant is `0xFFBBBBBB`, grey. (Category 7, which `0x00BB` sends,
                        // is `0xFFFFFF00` - yellow. So the chat notice was never going to be
                        // the grey the owner asked for.) `research/message-subcases.md`.
                        out.push(Reply {
                            opcode: net::stats::USER_EFFECT_LOCAL,
                            body: net::message::item_gained_in_chat(item_id, count as u32),
                            what: format!(
                                "UserEffectLocal item line: quest {quest_id} rewarded {item_id} x{count} - chat category 6, which is grey. NOT the 0x0089 pick-up line, whose chat route is gated on a fieldType no map has"
                            ),
                        });
                    }
                    Err(e) => out.extend(self.notice(format!(
                        "Quest {quest_id} could not give you item {item_id}: {e}"
                    ))),
                }
                continue;
            }
            // A take. `count` is negative, so the amount is its magnitude.
            let want = count.unsigned_abs().min(u16::MAX as u32) as u16;
            match self.take_quest_item(chr.id, inv, item_id, want) {
                Ok(replies) => out.extend(replies),
                Err(e) => out.extend(self.notice(format!(
                    "Quest {quest_id} wanted to take back {item_id} and could not: {e}"
                ))),
            }
        }

        // **`Act.1.skill` - the crafting professions.** Before the experience line, so the
        // "you have learnt Tailoring" sentence sits with the items it was earned beside.
        // `session/craft.rs`.
        out.extend(self.grant_quest_skills(quest_id));

        // Last, so the experience line lands under the item lines the way a turn-in reads.
        // White: a quest reward is yours, not a share of somebody else's kill.
        //
        // **Multiplied by the Quest EXP rate**, the fourth `!setrates` field (the owner,
        // 2026-09-06: *"it should also now affect quest exp obtained from quest completion"*).
        // Separate from the kill rate on purpose: a 5x kill event need not make every
        // turn-in worth five levels, and the two are set independently.
        let quest_rate = self.rate(store::rates::RateKind::Quest);
        let paid = quest_rate.apply(exp);
        let why = if quest_rate.is_normal() {
            format!("quest {quest_id}")
        } else {
            format!("quest {quest_id} ({exp} at {quest_rate}x)")
        };
        out.extend(self.award_experience(paid, &why, true, true));
        out
    }


    /// Remove `count` of `item_id` from a bag, across as many slots as it takes.
    fn take_quest_item(
        &mut self,
        character_id: u32,
        inv: store::InventoryType,
        item_id: u32,
        count: u16,
    ) -> Result<Vec<Reply>, store::StoreError> {
        let mut left = count;
        let mut out = Vec::new();
        for row in self.store.bag_items(character_id, inv)? {
            if left == 0 {
                break;
            }
            if row.item.item_id != item_id {
                continue;
            }
            let held = row.item.kind.quantity();
            let take = held.min(left);
            self.store.remove_item(character_id, inv, row.slot, Some(take))?;
            left -= take;
            // **The slot's new count, not a removal, when some of the stack stays.** The owner,
            // 2026-10-03: handing in 10 of 37 cleared the whole stack from the window while the
            // other 27 were still in the bag, until a map change redrew it. A REMOVE draws an
            // empty slot whatever is left behind; `stack_change_replies` sends REMOVE only at 0.
            for mut r in self.stack_change_replies(inv, row.slot, held - take) {
                r.what = format!("{} - {take} x {item_id} taken back by a quest ({held} -> {})", r.what, held - take);
                out.push(r);
            }
        }
        // **`<Item> x<n> has been lost.` in the chat**, the twin of the reward's "earned" line
        // (the owner, 2026-10-04: *"the server should also show that the player has lost those
        // items in chat, similar to the items that they gain from a quest"*). One line for the
        // whole take, however many slots it came out of, and only for what actually left.
        let taken = count - left;
        if taken > 0 {
            out.push(Reply {
                opcode: net::stats::USER_EFFECT_LOCAL,
                body: net::message::item_lost_in_chat(item_id, u32::from(taken)),
                what: format!("UserEffectLocal item line: {item_id} x{taken} handed in - '... has been lost.', chat category 6, grey"),
            });
        }
        if left > 0 {
            // Not an error. The quest is completing either way; say so and carry on.
            out.extend(self.notice(format!(
                "A quest wanted {count} x {item_id} back and you only had {}.",
                count - left
            )));
        }
        Ok(out)
    }


    /// Write an accepted quest down, and tell the journal about it.
    ///
    /// Split from the handler so the *decision* to accept and the *recording* of it read
    /// separately - the decision is a packet field, the recording is a database write, and
    /// conflating them is how this ended up in the wrong handler in the first place.
    /// **Only a quest that was actually written down earns its `Act.0`.** The owner,
    /// 2026-08-21: *"I was able to complete the Heena quest multiple times, this is not
    /// okay."*
    ///
    /// `store::start_quest` is `INSERT OR IGNORE` and has always refused a second accept
    /// correctly. What ignored the refusal was this function: `grant_quest_start_items` and
    /// `apply_quest_hp` were called *outside* the match, so clicking Accept again handed the
    /// items over again. Quest 1001's `Act.0` is Sera's Mirror, so the mirror was farmable a
    /// click at a time; 1002's also sets HP, so Roger's apple was too.
    ///
    /// A **completed** quest gets no reply at all. Re-sending `quest_accepted` for one would
    /// put it back in the client's *started* list - which is the exact behaviour
    /// `start_quest`'s own doc warns about, and the loop this whole feature exists to end.
    /// An in-progress one still gets the record re-sent, because there the two books agree
    /// and the resend only restates it.
    pub(super) fn record_quest_start(&mut self, quest_id: u32, npc_template: u32) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        // Read before writing, so "already complete" can be told from "already started".
        // `start_quest` collapses both into `false` and the two need different answers.
        let mut before = self.store.quest_row(chr.id, quest_id).ok().flatten().map(|r| r.state);
        // **Citizenship.** A quest gated on a town's citizenship, or on the Community Board's
        // posting, is refused here the way the client refuses it (`0x50`, `0x51`, `0x13`,
        // `0x16`, `0x19`); and a COMPLETED board quest that is posted again and past its day or
        // week is picked up afresh - the one way a completed row moves back. Every other quest
        // passes straight through. `session/citizenship.rs`.
        let restart = match self.citizenship_start_gate(quest_id, npc_template) {
            Ok(restart) => restart,
            Err(refusal) => return refusal,
        };
        if restart {
            before = None;
        }
        if before == Some(store::QuestState::Complete) {
            // A daily already turned in today, offered by a client that still reads
            // `interval 0`: Arwen says so. session/dailyquest.rs.
            return self.daily_quest_refusal(quest_id, npc_template).unwrap_or_default();
        }
        // The same rule at the other end: a quest that hands something over on accept
        // (Sera's mirror, Roger's apple) is not accepted into a bag that cannot take it.
        if before.is_none() {
            let gives: Vec<crate::config::RewardItem> = self
                .config
                .quests
                .get(&quest_id)
                .map(|q| {
                    q.start_items
                        .iter()
                        .filter(|(_, c)| *c > 0)
                        .map(|&(id, count)| crate::config::RewardItem { id, count, prop: 0, gender: None })
                        .collect()
                })
                .unwrap_or_default();
            let short = self.quest_room_shortfall(&chr, &gives);
            if !short.is_empty() {
                return self.bag_full_refusal(quest_id, npc_template, &short, "accepted");
            }
        }
        // An accept that costs mesos (`Act.0.money` < 0, Nella's commission): taken now,
        // refused when short, handed back on every path below that does not record the start.
        let charged = if before.is_none() {
            match self.take_quest_cost(quest_id, 0, npc_template, "accepted") {
                Ok(charged) => charged,
                Err(refusal) => return refusal,
            }
        } else {
            0
        };
        let mut out = Vec::new();
        if restart {
            match self.store.restart_quest(chr.id, quest_id) {
                // The client still holds the completion: take it out of BOTH collections
                // first (state 0, forget_completion), then the ordinary accept below puts it
                // in the started one. Without the first, the quest would sit in both tabs.
                Ok(true) => out.push(Reply {
                    opcode: net::quest::MESSAGE,
                    body: net::quest::quest_forgotten(quest_id, true),
                    what: format!(
                        "quest {quest_id}: a completed Community Board quest picked up again for character {} - its completion is cleared, then it is accepted afresh",
                        chr.id
                    ),
                }),
                Ok(false) => {
                    self.refund_quest_cost(quest_id, charged);
                    return Vec::new();
                }
                Err(e) => {
                    self.refund_quest_cost(quest_id, charged);
                    return self.notice(format!("Quest {quest_id} could not be picked up again: {e}"));
                }
            }
        }
        let started = self.store.start_quest(chr.id, quest_id);
        if !matches!(started, Ok(true)) {
            self.refund_quest_cost(quest_id, charged);
        }
        let what = match &started {
            Ok(true) => format!(
                "quest {quest_id} accepted from NPC {npc_template} by character {} ({}) and stored",
                chr.id, chr.name
            ),
            Ok(false) => format!(
                "quest {quest_id} was already started for character {}; the record is re-sent so the journal agrees, and Act.0 is NOT paid again",
                chr.id
            ),
            Err(e) => format!(
                "quest {quest_id} accepted but NOT STORED ({e}) - the journal will show it until the next relog and then lose it"
            ),
        };
        out.push(Reply { opcode: net::quest::MESSAGE, body: net::quest::quest_accepted(quest_id), what });
        // Only on the transition. A row that already existed has already been paid.
        if before.is_none() {
            // The cost line, or `Act.0.money`'s payout, only for a row that was written (a cost
            // was refunded above otherwise).
            if matches!(started, Ok(true)) {
                out.extend(self.quest_cost_replies(quest_id, charged));
                out.extend(self.pay_quest_mesos(quest_id, 0));
            }
            out.extend(self.grant_quest_start_items(quest_id));
            out.extend(self.apply_quest_hp(quest_id, 0));
            // **Last, because it changes the field.** A `SetField` tears down whatever
            // dialogue is on screen, so anything with something to say has to have said it
            // by now. Only the four *Test of Qualification* quests produce anything here.
            out.extend(self.enter_test_field_on_quest_start(quest_id));
        }
        out
    }


    /// Set the character's HP because a quest said so - `Act.<state>.hp`, an authored key.
    ///
    /// The owner, 2026-08-21: *"When accepting their quest, it should automatically lower the
    /// user's health to 25/50, and give users a Roger's Apple to recover their HP with."*
    /// Without the first half the apple has nothing to do, which is why the quest reads as
    /// broken even once the item is handed over.
    ///
    /// **Capped at the maximum**, so a value larger than the character's max HP is a heal to
    /// full rather than a bar drawn past its own end - the same rule the potion path
    /// follows.
    ///
    /// Sends nothing when the quest names no HP for that state, which is every quest but
    /// 1002. The WZ has no `Act.<state>.hp` key at all; this behaviour lived in the script
    /// the client does not ship.
    pub(super) fn apply_quest_hp(&mut self, quest_id: u32, state: u8) -> Vec<Reply> {
        let Some(quest) = self.config.quests.get(&quest_id) else { return Vec::new() };
        let Some(&want) = quest.set_hp.get(&state) else { return Vec::new() };
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let was = chr.hp;
        // A quest's health is not a revive: only the revive path brings the dead back.
        if was == 0 {
            return Vec::new();
        }
        chr.hp = want.min(self.pools(&chr).max_hp);
        if chr.hp == was {
            return Vec::new();
        }
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not set your health for quest {quest_id}: {e}"));
        }
        vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange::hp_only(chr.hp).build(),
            what: format!(
                "StatChanged: quest {quest_id} state {state} set hp {was} -> {}/{} (Act.{state}.hp is an AUTHORED key - the WZ has none)",
                chr.hp, chr.max_hp
            ),
        }]
    }


    /// Hand over whatever `Act.0.item` says accepting this quest gives.
    ///
    /// The owner, 2026-08-20: *"when I talked to Sera, after their dialogue, they did not give me
    /// Sera's Mirror to be able to complete the quest."* Quest 1001's `Act.0.item.0` is
    /// `4031000` x1 and nothing read it, so the conversation happened and the bag stayed
    /// empty - and quest 1001's completion `Check.1.item.0` wants exactly that item, so the
    /// chain could not be finished by any route.
    ///
    /// **Only the giving direction.** A negative count in the WZ means "take it away" - quest
    /// 1001's `Act.1.item.0.count` is `-1` - and taking an item on completion is not wired,
    /// so finishing that quest leaves the mirror in the bag. Recorded rather than silently
    /// half-done: `Act.<state>.item` is read for state 0 only.
    /// The tabs `rows` would overflow, against the bag as it is now. Gives and takes are the
    /// rows' signs; the tab is the config's (`tab_for`, so a cash equip counts against Deco).
    fn quest_room_shortfall(
        &self,
        chr: &net::opcode::Character,
        rows: &[crate::config::RewardItem],
    ) -> Vec<crate::questroom::Shortfall> {
        let Ok(bag) = self.store.bag(chr.id) else { return Vec::new() };
        let mut gives = Vec::new();
        let mut takes = Vec::new();
        for r in rows {
            let Some(tab) = self.config.tab_for(r.id).or_else(|| store::InventoryType::for_item(r.id)) else { continue };
            let n = r.count.unsigned_abs().min(u16::MAX as u32) as u16;
            if r.count > 0 {
                gives.push((r.id, n, tab));
            } else if r.count < 0 {
                takes.push((r.id, n, tab));
            }
        }
        crate::questroom::shortfall(&bag, &gives, &takes, |id| self.config.shops.max_stack(id))
    }

    /// **The NPC says the bag is full, and the quest stays where it was.** The owner,
    /// 2026-09-17: *"the server should use the NPC dialogue and display an appropriate
    /// message to say that their bag is full, please make <x> amount of spaces in <y> tab."*
    ///
    /// A plain `Say` from `speaker` (no Next), parked under `questroom::REFUSAL_PATH` so the
    /// OK that closes it is answered silently and the `0x0151` handler skips its closing
    /// line. A quest with no NPC on record (a `complete_on_consume` one) gets the same words
    /// as a chat line - there is no box to put them in.
    fn bag_full_refusal(
        &mut self,
        quest_id: u32,
        speaker: u32,
        short: &[crate::questroom::Shortfall],
        at: &str,
    ) -> Vec<Reply> {
        let text = crate::questroom::refusal_text(short);
        crate::server::log(&format!(
            "   quest {quest_id}: NOT {at} - the bag has no room for what it hands over: {:?}. Nothing written, nothing paid.",
            short.iter().map(|s| format!("{:?} short {}", s.tab, s.slots)).collect::<Vec<_>>()
        ));
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
            what: format!(
                "ScriptMessage Say from NPC template {speaker}: quest {quest_id} not {at}, the bag is full - {}",
                short.iter().map(|s| format!("{} short {}", net::bag::BAG_TAB_NAMES[s.tab.index()], s.slots)).collect::<Vec<_>>().join(", ")
            ),
        }]
    }

    fn grant_quest_start_items(&mut self, quest_id: u32) -> Vec<Reply> {
        let Some(quest) = self.config.quests.get(&quest_id) else { return Vec::new() };
        let items = quest.start_items.clone();
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = Vec::new();
        for (item_id, count) in items {
            if count <= 0 {
                continue; // a take, which this does not do - see the doc comment
            }
            let Some(inv) = store::InventoryType::for_item(item_id) else {
                out.extend(self.notice(format!(
                    "Quest {quest_id} wanted to give you item {item_id}, whose id names no bag."
                )));
                continue;
            };
            let max_stack = self.config.shops.max_stack(item_id);
            let item = if inv == store::InventoryType::Equip {
                store::Item::equip(item_id)
            } else {
                store::Item::bundle(item_id, count.min(u16::MAX as i32) as u16)
            };
            match self.store.add_item(chr.id, inv, &item, max_stack) {
                Ok(placed) => {
                    out.extend(self.inventory_added_replies(inv, &placed, "given by a quest"));
                    // The chat-log line, same as the completion reward above - a quest
                    // handing an item over reads the same way whichever end it happens at.
                    out.push(Reply {
                        opcode: net::stats::USER_EFFECT_LOCAL,
                        body: net::message::item_gained_in_chat(item_id, count.max(1) as u32),
                        what: format!(
                            "UserEffectLocal item line: quest {quest_id} gave {item_id} x{count} - chat category 6, grey"
                        ),
                    });
                }
                Err(e) => out.extend(self.notice(format!(
                    "Quest {quest_id} could not give you item {item_id}: {e}"
                ))),
            }
        }
        out
    }


    /// Make an NPC with **no quest** speak. This is the other half of goal 2.
    ///
    /// **There are two NPC-click packets and the server was answering only one.** Which one
    /// goes out is decided entirely inside the client, from `Quest.wz`: `FUN_1428de280`
    /// forks on whether the NPC has a non-empty script name, and only a menu line carrying a
    /// quest id reaches the `0x0151` builder. Every other outcome sends `0x00F2`. Robin on
    /// map 40 has no quests, so clicking them produced `0x00F2` and total silence on
    /// 2026-08-19. See `research/npc-click.md`.
    ///
    /// **The one thing that differs from the quest path**: `0x0151` hands us the NPC's
    /// *template* id, and `0x00F2` hands us the **object** id we chose - while `0x055B`'s
    /// speaker field wants a template. So this maps back through the same table that
    /// assigned the object id. Sending the object id straight through would not fault (the
    /// loader result is null-checked at `142a7b52a`) but would draw a portrait-less box.
    pub(super) fn on_npc_click(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(click) = net::script::parse_npc_click(body) else { return Vec::new() };
        let Some(chr) = self.claimed_character() else { return Vec::new() };

        // The object id is only unique within a field, which is why the lookup is scoped to
        // the character's current map - config::load_npcs numbers from 1000 per map.
        let template = self
            .config
            .npcs
            .get(&chr.map_id)
            .and_then(|list| list.iter().find(|n| n.object_id == click.npc_object_id))
            .map(|n| n.template_id);

        let Some(template) = template else {
            // Nothing to speak as. Answering with a script whose speaker is not a real
            // Npc.wz id buys nothing, and this is not a request the client blocks on - the
            // 2026-08-19 capture shows the UI stayed live with 0x00F2 unanswered.
            return Vec::new();
        };

        // **A shopkeeper opens a shop instead of talking.** This is the last mile of goal F:
        // the packet has been decoded for a while and had nobody to send it to, because
        // nothing joined `data/shops.txt`'s NPC *name* onto the template id the click
        // carries. `Config::shop_by_template` is that join.
        //
        // **Except a shop a quest opens** (Arwen, Jane), which asks first and only someone who
        // has finished the quest: `None` from `quest_shop_click` means no counter at all, so
        // that shop is never opened here. session/questshop.rs.
        if questshop::quest_shop(template).is_some() {
            if let Some(replies) = self.quest_shop_click(template) {
                return replies;
            }
        } else if let Some(replies) = self.open_shop_for(template, chr.id) {
            return replies;
        }

        // **And a storage keeper opens a box instead of talking.** Same shape as the shop
        // branch above and for the same reason - Mr. Kim has no `d0` line at all, so before
        // this they fell through to a one-line conversation with nothing to say, which on
        // screen is indistinguishable from a click that did nothing.
        if let Some(replies) = self.open_storage_for(template) {
            return replies;
        }

        // **And Phil hands out a map instead of talking.** They have no `d0` line at all, so
        // before this a click on them printed the "no dialogue for template 101" placeholder -
        // observed, previous-runs/world-20260821-230104.log 02:50:56.185.
        if template == crate::jobguide::PHIL_TEMPLATE {
            if let Some(replies) = self.phil_job_guide() {
                return replies;
            }
        }

        // **And a taxi sells a ride instead of talking.** Same shape, and the same failure it
        // fixes: Lyn has no `d0` at all, so clicking their printed the placeholder.
        // **Shanks, before the taxi branch.** They are disjoint - they are not in `TAXIS` -
        // but they are the specific case and a specific case reads better first.
        if let Some(replies) = self.open_shanks_for(template) {
            return replies;
        }

        // **The beauty shops' eight NPCs**, each in its own shop: a salon owner's style-coupon
        // menu, its assistant's colour-coupon menu, a surgery owner's faces, its assistant's
        // skins, and a pointer at the Cash Shop for a
        // player without either. session/salon.rs.
        if let Some(replies) = self.open_salon_for(template) {
            return replies;
        }

        // **Lakelis and the party quest**, in Kerning City only. crate::firsttime.
        if let Some(replies) = self.open_first_time_together(template) {
            return replies;
        }

        // **Cloto**, on stages 1 to 5: TEMPORARILY clears the stage on a click, for this run
        // only. session/firsttime.rs.
        if let Some(replies) = self.open_cloto(template) {
            return replies;
        }

        // **Nella**, in any of the seven quest fields: the way out, and home from the Exit.
        if let Some(replies) = self.open_nella(template) {
            return replies;
        }
        // Joel, Cherry and Purin - the ship to Orbis. session/boat.rs.
        if let Some(replies) = self.open_boat_npc(template) {
            return replies;
        }
        // The Sleepywood Hotel Receptionist's two saunas. session/hotel.rs.
        if let Some(replies) = self.open_hotel_receptionist(template) {
            return replies;
        }
        // The jump quests' doors, wardens and goals - Shane, the Statues, Jake, the Ticket
        // Gate, Louis, the Exits, the piles and the chests. session/jumpquest.rs.
        if let Some(replies) = self.open_jump_quest_npc(template) {
            return replies;
        }

        if let Some(replies) = self.open_taxi_for(template) {
            return replies;
        }

        // **Arthur and Roxy sign citizenship contracts.** Level 12+, in their own hall; below
        // that they say their own line. session/citizenship.rs.
        if let Some(replies) = self.open_town_clerk(template) {
            return replies;
        }

        // **There is deliberately no daily-favour branch here either**, and it is the same
        // removal as the one in `on_quest_request`: clicking the Maple Administrator is a click
        // on the Maple Administrator again. `!tool` is what opens the favours now.

        // **And an instructor advances the job instead of talking.** Same shape again, and
        // the same failure it fixes: `world::jobs` has decided this correctly since it was
        // written, and nothing called it - `!job` was the only way to reach a job change, so
        // Dances with Balrog, Grendel, Athena Pierce and Dark Lord all just said a line.
        // `CLAUDE.md`'s "built is not wired", found by asking what has no caller.
        if let Some(replies) = self.advance_job_for(template) {
            return replies;
        }

        // **And at level 30 the same four instructors offer the SECOND advancement.** They
        // now answer two different questions and the order is the one above: `jobs` first,
        // because a beginner at 511 is asking about the first one, and this second because a
        // Swordsman is asking about the next.
        if let Some(replies) = self.second_advancement_for(template) {
            return replies;
        }

        // **And the examiner runs the test**, which is what the four `<Job> Job Instructor`
        // NPCs outside the towns exist for. They advance nobody - their own idle line says
        // so - they warp the player into the hidden field and take the marbles afterwards.
        if let Some(replies) = self.job_test_for(template) {
            return replies;
        }

        // **And the warden inside the hidden field is the only door out of it.** Those four
        // maps have exactly one portal each and it is the spawn point, so a click on this NPC
        // is not a convenience - without it a character in there is stuck.
        if let Some(replies) = self.job_test_exit_for(template) {
            return replies;
        }

        // **And at level 70, four NPCs in one room in El Nath give the THIRD advancement.**
        // Different templates from every other job NPC in the game - 1104 / 1105 / 1106 /
        // 1107 - so the order against the branches above does not matter and none of them can
        // claim a click meant for another.
        if let Some(replies) = self.third_advancement_for(template) {
            return replies;
        }

        // A quest-less NPC is a one-line conversation: its own `d0`. Going through the
        // same state machine means its OK is handled the way a quest's is, rather than
        // leaving a stale conversation behind for the next 0x00F3 to walk into.
        let _ = (click.npc_object_id, chr.map_id);
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: String::new(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        self.say_line(0)
    }


    /// Send the box for `index` of the conversation's current path, and remember what we
    /// sent so the answer can be interpreted.
    ///
    /// **The last line of a branchable conversation goes out as a yes/no prompt, not a Say**,
    /// and that is the whole reason the owner's Accept did nothing: a type-0 Say with `next = 0`
    /// draws `BtOK` and `BtClose`, so pressing it returns the same `action = 1` an OK does
    /// and the server has nothing to branch on. A type `0x10` box draws `BtQYes`/`BtQNo` and
    /// answers `1` for Yes and `0` for No, unambiguously. **[L]**, `research/script-reply.md`.
    /// **An instructor advances the job**, or says why not. `None` for any other NPC.
    ///
    /// # This was decided and unreachable
    ///
    /// `world::jobs::advancement_for` has been correct since the day it was written and had
    /// **no caller outside tests**: `!job` skips it deliberately (it is a debug command and
    /// must be able to set any job), so the four instructors fell through to an ordinary
    /// one-line conversation. On screen that is a job advancement that does not exist.
    ///
    /// # A refusal is still an answer, and it is said in the right order
    ///
    /// `jobs::refusal` puts the sentence together, and the order of its checks is the order
    /// the refusals should be *said* in: an eighth-level beginner is told to come back at ten,
    /// not that their LUK is short, because the second sentence is useless advice while the
    /// first is still true.
    ///
    /// # Every effect hangs off the transition
    ///
    /// The job write, the packet and the congratulation are all reached through one `Eligible`
    /// and one successful save. `CLAUDE.md`'s Heena section is about exactly this: the guard
    /// was asked and its answer ignored, because the payout sat outside the match. **If the
    /// save fails nothing else happens** - the character does not get a job packet for a job
    /// the database does not have.
    ///
    /// # Advancement is one-way
    ///
    /// The client's own quest text says *"a job advancement cannot be undone once made"*, so
    /// `AlreadyAdvanced` is a refusal rather than a re-offer. `!resetsp` does not undo it and
    /// is not meant to.
    pub(super) fn advance_job_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        match crate::jobs::advancement_for(&chr, template) {
            // Not an instructor at all - fall through to whatever this NPC normally does.
            crate::jobs::Advancement::NotAnInstructor => None,
            // **A character who already has a job is the SECOND advancement's business now,
            // and this must fall through rather than refuse.** It used to answer *"You have
            // already taken that step"*, which is true of the first advancement and is the
            // wrong sentence for a level-30 Swordsman standing in front of Dances with Balrog
            // asking for the next one. `research/second-job.md` section 9.1 asked for exactly
            // this, and the reason it is safe is narrow and worth stating: `jobs::
            // advancement_for` produces `AlreadyAdvanced` **only** when `first_job_at`
            // matched, i.e. only at 511 / 313 / 221 / 411 - and `secondjob::branch_at`
            // matches that same set of four exactly. So nothing is left unanswered by
            // falling through here; the next handler in the chain covers precisely the NPCs
            // this arm can name. `CLAUDE.md`'s "always answer" holds.
            crate::jobs::Advancement::AlreadyAdvanced { .. } => None,
            // **Eligible is a question, not a transition.** The owner, 2026-09-15: *"Currently the
            // moment you click on the first job instructors, you simply become that job. There
            // should be a yes or no dialogue (including the requirement to job advance for the
            // appropriate job) to make sure that the player is sure."* The box carries the
            // requirements and the one-way warning; the job changes only on Yes, in
            // `first_job_reply`, after the eligibility check is made a second time.
            crate::jobs::Advancement::Eligible { .. } => {
                let first = crate::jobs::first_job_at(template)?;
                self.conversation = Some(Conversation {
                    npc_template: template,
                    quest_id: None,
                    path: crate::jobs::ASK_PATH.to_string(),
                    sent: 0,
                    awaiting_yes_no: true,
                    sent_with_next: false,
                });
                let text = crate::jobs::confirmation(&chr, first);
                Some(vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_ask(template, &text, false),
                    what: format!("ScriptMessage AskYesNo from instructor {template}: the first job advancement's confirmation - {text:?}"),
                }])
            }
            // Every other arm is a refusal with a sentence already written for it.
            other => {
                let _ = &other;
                let text = crate::jobs::refusal(&chr, template)?;
                Some(self.instructor_says(template, &text))
            }
        }
    }

    /// **The player answered the instructor's question.** Yes advances; No and a closed box
    /// change nothing. The eligibility check runs again before the write - the level and the
    /// stat cannot have fallen in the meantime, but a Yes that arrives with no character
    /// claimed, or after `!job` changed the job, must not write a second advancement.
    fn first_job_reply(&mut self, template: u32, action: i8) -> Vec<Reply> {
        self.conversation = None;
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if action != net::script::SCRIPT_ACTION_YES {
            return self.instructor_says(template, "Take your time. Come back when you are ready to decide.");
        }
        match crate::jobs::advancement_for(&chr, template) {
            crate::jobs::Advancement::Eligible { job, job_name } => {
                let was = chr.job;
                chr.job = job;
                if let Err(e) = self.store.save_character_progress(&chr) {
                    // The save is the transition. Nothing follows a failed one.
                    return self.instructor_says(
                        template,
                        &format!("Something went wrong and your job was not changed: {e}"),
                    );
                }
                // **The Beginner's set, after the save and never before it.** The owner,
                // 2026-09-16: Grendel hands a just-advanced Magician a free Beginner's Wooden
                // Wand. The gift hangs off the transition: a failed save above gives nothing,
                // and a gift that cannot be placed (a full Equip tab) is SAID, not swallowed -
                // the job is still changed, because the job is the thing that was asked for.
                let (gifts, mut out) = self.starter_equips_for(template);
                let mut text = format!("Congratulations. You are now a {job_name}.");
                if !gifts.is_empty() {
                    text.push_str(&format!(" Take this with you - {} - it is yours, free.", gifts.join(" and ")));
                }
                text.push_str(" Open your skill window - you have skill points to spend.");
                let mut says = self.instructor_says(template, &text);
                says.append(&mut out);
                says.push(self.job_change_reply(was, job));
                says
            }
            _ => match crate::jobs::refusal(&chr, template) {
                Some(text) => self.instructor_says(template, &text),
                None => Vec::new(),
            },
        }
    }

    /// Hand over the instructor's `starter_equips`, one of each. Returns the names that were
    /// actually placed (for the instructor's sentence) and the packets that put them in the
    /// bag - the Add rows plus the grey chat line a quest reward draws, so a starter wand
    /// reads the same way a quest's mirror does. A gift that cannot be placed becomes a
    /// notice naming the item and the reason, and is left out of the sentence.
    fn starter_equips_for(&mut self, template: u32) -> (Vec<String>, Vec<Reply>) {
        let Some(first) = crate::jobs::first_job_at(template) else { return (Vec::new(), Vec::new()) };
        let mut names = Vec::new();
        let mut out = Vec::new();
        for &item_id in first.starter_equips {
            match self.give_item(item_id, 1, "the first job advancement's Beginner's equipment") {
                Ok((line, replies)) => {
                    out.extend(replies);
                    // The `what` is the log line: `line` says who got what into which slot.
                    out.push(Reply {
                        opcode: net::stats::USER_EFFECT_LOCAL,
                        body: net::message::item_gained_in_chat(item_id, 1),
                        what: format!(
                            "UserEffectLocal item line: {} ({template}) with the {} advancement - {line} - chat category 6, grey",
                            first.npc_name, first.job_name
                        ),
                    });
                    names.push(format!("a #b{}#k", self.item_name(item_id)));
                }
                Err(why) => {
                    out.extend(self.notice(format!(
                        "{} could not hand you item {item_id}: {why}",
                        first.npc_name
                    )));
                }
            }
        }
        (names, out)
    }

    /// Put Phil's job-path choice on screen, or say why not.
    ///
    /// `None` only when no character is claimed, which both callers have already ruled out.
    /// **Phil has no `d0` line**, so falling through here prints the "no dialogue for NPC
    /// template 101" placeholder, which is what they did before this existed.
    ///
    /// **One type-6 menu, parked at `jobguide::MENU_PATH`**, the way `open_taxi_for` and the
    /// second-job instructors do it. It was a chain of yes/no boxes until 2026-09-15 - built
    /// as the control beside the taxis' menu when "a server-sent type 6 draws its `#L`
    /// lines" was still **[D]** - and the owner found the leftover: *"The selection is
    /// fundamentally broken and cannot be selected by the cursor."* It could not be: a yes/no
    /// box has Yes and No and nothing else to click. The menu the taxi measured on screen
    /// (`research/fixtures/type6-menu-renders-and-taxi-rides-world.log`, **[L]**) is what they
    /// sends now.
    ///
    /// `awaiting_yes_no` is `false` for the same reason as the taxi's: a menu is not a yes/no
    /// box, and if `jobguide_menu_answer` were ever skipped a stray reply must not be taken
    /// for a quest Accept.
    fn phil_job_guide(&mut self) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        let step = crate::jobguide::opening(&chr);
        self.conversation = match step {
            crate::jobguide::Step::Menu { .. } => Some(Conversation {
                npc_template: crate::jobguide::PHIL_TEMPLATE,
                quest_id: None,
                path: crate::jobguide::MENU_PATH.to_string(),
                sent: 0,
                awaiting_yes_no: false,
                sent_with_next: false,
            }),
            _ => None,
        };
        Some(crate::jobguide::script_replies(&step))
    }

    /// The player answered Phil's menu. `None` means "not mine" - fall through.
    ///
    /// **Every effect hangs off `Step::Ride`.** `jobguide::on_pick` is the single decision
    /// and only `Ride` carries a destination, so a refusal cannot warp anyone by accident -
    /// there is no map id on any other arm to read. A closed box sends nothing (`Closed`).
    fn jobguide_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if !crate::jobguide::is_menu_path(&convo.path) {
            return None; // a taxi's, an instructor's, a quest's, or a plain talk's
        }
        // `None` is "not a type-6 body at all" - leave the conversation alone and let the
        // ordinary script path have it.
        let reply = net::script::parse_menu_reply(body)?;
        self.conversation = None;
        let Some(mut chr) = self.claimed_character() else { return Some(Vec::new()) };
        let step = crate::jobguide::on_pick(&chr, reply.selection, &self.config.fields);
        let mut out = crate::jobguide::script_replies(&step);
        if let crate::jobguide::Step::Ride(dest) = step {
            // Bound before the call: `go_to_map` takes `&mut chr`, so reading `chr.name` inside
            // its own argument list is a borrow conflict.
            let who = chr.name.clone();
            // **The notice goes BEFORE the SetField.** That is the order `gm_map` uses and the
            // order observed working - world-20260827-210956.log 01:07:51.738 has the
            // ChatNotice and the SetField at the same millisecond, notice first. And
            // `script_replies` deliberately returns nothing for a Ride: a script message with
            // or just before a SetField is torn down silently by field entry.
            out.extend(self.notice(crate::jobguide::arrival_line(dest)));
            out.extend(self.teleport(
                &mut chr,
                dest.map_id,
                format!(
                    "Phil's job guide: {who} chose {} and rides to {} ({}), where {} (template \
                     {}) is waiting. Phil grants no job - advance_job_for does.",
                    dest.job_name, dest.map_id, dest.map_name, dest.npc_name, dest.npc_template
                ),
            ));
        }
        Some(out)
    }

    /// Whether Phil has nothing left to say about the quest the client just named.
    ///
    /// Keeps "Phil's Call" working: the quest runs first and pays out, and the guide takes
    /// over only once it is finished, or is one this server has no data for (which would
    /// otherwise print the placeholder).
    fn phil_has_nothing_left(&self, req: &net::script::QuestRequest) -> bool {
        if self.config.quests.get(&req.quest_id).is_none() {
            return true;
        }
        let Some(chr) = self.claimed_character() else { return false };
        matches!(
            self.store.quest_row(chr.id, req.quest_id).ok().flatten().map(|r| r.state),
            Some(store::QuestState::Complete)
        )
    }

    /// **Shanks' yes/no.** `None` for any NPC that is not them, so the click chain carries on.
    ///
    /// `awaiting_yes_no` is `true` and the path is `shanks::ASK_PATH`: the flag is what makes
    /// the reply reach a yes/no handler at all, and the path is what stops the quest branch
    /// from mistaking the answer for a quest Accept.
    pub(super) fn open_shanks_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let map = self.claimed_character()?.map_id;
        if !crate::shanks::is_shanks(template, map) {
            return None;
        }
        let step = crate::shanks::opening();
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: crate::shanks::ASK_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        });
        Some(crate::shanks::script_replies(&step))
    }

    /// The player answered Shanks. `None` means "not mine" - fall through.
    ///
    /// **Every effect hangs off `Step::Sail`**: it is the only arm carrying a destination, so
    /// a refusal cannot warp anyone, and by the time one exists the fare is already taken in
    /// one transaction.
    fn shanks_reply(&mut self, action: i8) -> Vec<Reply> {
        self.conversation = None;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let step = crate::shanks::on_answer(&self.store, &self.config, &chr, action);
        self.shanks_step(chr, step)
    }

    /// **The player dismissed Shanks' waiver box**, so the boat sails - for nothing.
    ///
    /// Called from `on_script_reply` BEFORE its closed-box early return, because a Close on
    /// this box sails too: the decision was the Yes before it, and the fare is zero. The owner,
    /// 2026-09-16: the free line "does not show" - it was sent in the same batch as the
    /// `SetField` and torn down by field entry. This is the second half that fixes it.
    fn shanks_announce_dismissed(&mut self) -> Vec<Reply> {
        self.conversation = None;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let step = crate::shanks::on_announce_dismissed(&self.store, &self.config, &chr);
        self.shanks_step(chr, step)
    }

    /// The effects of a Shanks step. **Every warp hangs off `Step::Sail`**; an `Announce`
    /// parks the conversation and sends its box alone, with nothing after it.
    fn shanks_step(&mut self, mut chr: net::opcode::Character, step: crate::shanks::Step) -> Vec<Reply> {
        let mut out = crate::shanks::script_replies(&step);
        match step {
            crate::shanks::Step::Announce { .. } => {
                self.conversation = Some(Conversation {
                    npc_template: crate::shanks::TEMPLATE,
                    quest_id: None,
                    path: crate::shanks::ANNOUNCE_PATH.to_string(),
                    sent: 0,
                    awaiting_yes_no: false,
                    sent_with_next: false,
                });
            }
            crate::shanks::Step::Sail { fare, balance } => {
                // The balance moves before the screen does, exactly as the taxi does it.
                out.extend(self.meso_reply(chr.id));
                // **Only say it if something was taken.** A free crossing that announced
                // "You have lost mesos (-0)" would undo the moment the waiver box just built.
                if fare > 0 {
                    out.push(Reply {
                        opcode: net::message::MESSAGE,
                        body: net::message::meso_lost_line(fare),
                        what: format!("Message: grey chat line, Shanks' fare of {fare}"),
                    });
                }
                let why = format!(
                    "Shanks sailed character {} to Lith Harbor for {fare} mesos (balance {balance}){}",
                    chr.id,
                    if fare == 0 { " - FREE: a Beginner with Mai's Final Training complete" } else { "" }
                );
                out.extend(self.teleport(&mut chr, crate::shanks::DESTINATION_MAP, why));
            }
            crate::shanks::Step::Ask { .. } | crate::shanks::Step::Done { .. } => {}
        }
        out
    }

    /// **A taxi's menu.** `None` for any NPC that is not one, so the click chain carries on.
    ///
    /// `awaiting_yes_no` is deliberately `false`: a menu is not a yes/no box, and if this
    /// branch were ever skipped a stray reply must not be mistaken for a quest Accept.
    pub(super) fn open_taxi_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        // **The map is part of the question now.** Eurek the Alchemist stands in two towns and
        // is a ferry port in only one of them; in the other they are an ordinary NPC with their own
        // line. `taxi_for` carries the argument for that.
        let map = self.claimed_character()?.map_id;
        let taxi = crate::taxi::taxi_for(template, map)?;
        let step = crate::taxi::opening(taxi, &self.config)?;
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: crate::taxi::MENU_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        Some(crate::taxi::script_replies(taxi, &step))
    }

    /// The player answered a taxi's menu. `None` means "not mine" - fall through.
    ///
    /// **Every effect hangs off `Step::Ride`**: no other arm carries a map id, so a refusal
    /// cannot teleport anyone. By the time a `Ride` exists the fare is already taken, in one
    /// transaction.
    fn taxi_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if !crate::taxi::is_taxi_path(&convo.path) {
            return None; // Phil's, or a quest's, or a plain talk's
        }
        let mut chr = self.claimed_character()?;
        let taxi = crate::taxi::taxi_for(convo.npc_template, chr.map_id)?;
        // `None` is "not a type-6 body at all" - leave the conversation alone and let the
        // ordinary script path have it.
        let step = crate::taxi::on_reply(&self.store, &self.config, chr.id, taxi, body)?;
        self.conversation = None;
        let mut out = crate::taxi::script_replies(taxi, &step);
        if let crate::taxi::Step::Ride { map_id, map_name, fare, balance } = step {
            // The balance moves before the screen does. Field entry states it again ~420 ms
            // later; belt and braces, not a duplicate - the character record has no meso field.
            out.extend(self.meso_reply(chr.id));
            // **And say so on the right-hand side.** The owner: the client should be told it lost
            // the fare, the way it is told about a quest item. `meso_reply` moves the number
            // in the UI silently; this is the sentence. It goes BEFORE the `SetField` for the
            // same reason the arrival notice does - a field change tears script dialogs down,
            // and this is not one, but the ordering is the one already observed working.
            out.push(Reply {
                opcode: net::message::MESSAGE,
                // **The ride's fare, not the row's**: Lyn's way back to Southperry costs
                // 20,000 under a header that quoted 500, and the line must say what was taken.
                body: net::message::meso_lost_line(fare),
                what: format!(
                    "Message: Meso Penalty Applied (-{fare}) - the taxi fare, said out loud. The \
                     client owns the wording; we send the number. Whether a zero plain line \
                     also draws is UNMEASURED - see net::message::meso_penalty"
                ),
            });
            let why = crate::taxi::ride_note(taxi, &chr, map_id, &map_name, fare, balance);
            out.extend(self.teleport(&mut chr, map_id, why));
        }
        Some(out)
    }    /// How many of `item_id` this character is carrying, across every slot of its own tab.
    ///
    /// **Zero on any error**, deliberately: the callers are gates, and a database hiccup must
    /// read as "you are not carrying the proof" rather than as "you are". The failure
    /// direction is the one that refuses, not the one that grants.
    pub(super) fn held_count(&self, character_id: u32, item_id: u32) -> u32 {
        let Some(inv) = store::InventoryType::for_item(item_id) else { return 0 };
        self.store
            .bag_items(character_id, inv)
            .map(|rows| {
                rows.iter()
                    .filter(|r| r.item.item_id == item_id)
                    .map(|r| u32::from(r.item.kind.quantity()))
                    .sum()
            })
            .unwrap_or(0)
    }

    /// **The second advancement's choice box**, or the sentence saying why not. `None` for
    /// any NPC that is not one of the four first-job instructors.
    ///
    /// # Why the choice is a menu and the first advancement is not
    ///
    /// A first advancement has exactly one outcome per instructor, so there is nothing to
    /// pick. A second has two or three, and `secondjob::BRANCHES` carries them. The type-6
    /// list box is the client's own widget for that and it is now proven on screen by the
    /// taxis.
    ///
    /// # `StillABeginner` returns `None`, and that is not laziness
    ///
    /// `jobs::advancement_for` has already had its turn in [`Session::advance_job_for`]. A
    /// beginner standing here is entitled to a *first* advancement, and answering "you are
    /// not a Swordsman" would refuse someone who is one click away from becoming one.
    /// `secondjob::refusal_for` returns `None` for that arm for the same reason.
    pub(super) fn second_advancement_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        let branch = crate::secondjob::branch_at(template)?;
        let holds_proof = self.held_count(chr.id, branch.chain.proof_item) > 0;
        match crate::secondjob::advancement_for_holding(&chr, template, holds_proof) {
            crate::secondjob::Advancement::NotAnInstructor
            | crate::secondjob::Advancement::StillABeginner => None,
            crate::secondjob::Advancement::Choose { branch } => {
                self.conversation = Some(Conversation {
                    npc_template: template,
                    quest_id: None,
                    path: crate::secondjob::MENU_PATH.to_string(),
                    sent: 0,
                    // A menu is not a yes/no box. If this branch were ever skipped, a stray
                    // reply must not be mistaken for a quest Accept.
                    awaiting_yes_no: false,
                    sent_with_next: false,
                });
                Some(vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_menu(template, &crate::secondjob::menu_text(branch)),
                    what: format!(
                        "ScriptMessage type 6 MENU: {} offers character {} the second advancement - {} choices. Three of the ten job names are OURS, not the client's; see secondjob::NameSource",
                        branch.instructor_name,
                        chr.id,
                        branch.choices.len()
                    ),
                }])
            }
            // Every other arm is a refusal with a sentence already written for it, and the
            // sentence comes from `secondjob` so that this file and that one cannot drift.
            other => Some(self.instructor_says(template, &crate::secondjob::refusal_for(&other)?)),
        }
    }

    /// The player picked a second job off the menu. `None` means "not mine" - fall through.
    ///
    /// **The path is the only thing that says who asked**: a type-6 body carries no speaker,
    /// so this checks `secondjob::is_menu_path` before it decodes anything, exactly as the
    /// taxi's does. The two prefixes are disjoint.
    fn second_job_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if !crate::secondjob::is_menu_path(&convo.path) {
            return None; // a taxi's, Phil's, or a quest's
        }
        let reply = net::script::parse_menu_reply(body)?;
        let template = convo.npc_template;
        let branch = crate::secondjob::branch_at(template)?;
        // The box is gone from the screen either way; a stale conversation is what the next
        // reply walks into.
        self.conversation = None;
        // Closed rather than chosen. Nothing was decided, so nothing happens and nothing is
        // said - `0x00F3` does not hold the one-request latch, so silence here is safe.
        let selection = reply.selection?;
        let Some(choice) = crate::secondjob::choice_at(branch, selection) else {
            return Some(self.instructor_says(
                template,
                "That is not one of the paths I can offer you.",
            ));
        };
        Some(self.grant_second_job(template, choice.job))
    }

    /// Apply a second advancement. **Every effect hangs off the save.**
    ///
    /// The whole decision is re-run here rather than trusted from the menu: the selection
    /// arrives off a socket and nothing upstream had to have validated it, so a job from
    /// another branch comes back as a refusal instead of being granted. `CLAUDE.md`'s Heena
    /// section is about exactly this - the guard that was asked and then ignored.
    fn grant_second_job(&mut self, template: u32, chosen_job: u16) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let Some(branch) = crate::secondjob::branch_at(template) else { return Vec::new() };
        let holds_proof = self.held_count(chr.id, branch.chain.proof_item) > 0;
        // Re-checked at the point of granting, not only at the point of offering. Between the
        // two the player could have dropped, traded or stored the proof.
        if crate::secondjob::REQUIRE_PROOF_ITEM && !holds_proof {
            let outcome = crate::secondjob::Advancement::NeedsProof { branch };
            let Some(text) = crate::secondjob::refusal_for(&outcome) else { return Vec::new() };
            return self.instructor_says(template, &text);
        }
        let grant = match crate::secondjob::advancement_to(&chr, template, chosen_job) {
            Ok(g) => g,
            Err(refused) => {
                let Some(text) = crate::secondjob::refusal_for(&refused) else { return Vec::new() };
                return self.instructor_says(template, &text);
            }
        };
        let was = chr.job;
        chr.job = grant.job;
        if let Err(e) = self.store.save_character_progress(&chr) {
            // The save is the transition. Nothing follows a failed one - the character must
            // not get a job packet for a job the database does not have.
            return self.instructor_says(
                template,
                &format!("Something went wrong and your job was not changed: {e}"),
            );
        }
        let mut out = self.instructor_says(
            template,
            &format!(
                "Congratulations. You are now a {}. Open your skill window - there is a second \
                 page on it now, and points to spend.",
                grant.job_name
            ),
        );
        // `grant.sp_encoding_changes` is false for all ten second jobs, so the combined
        // `0x007C` `job_change_reply` already builds stays correct. Asserted rather than
        // assumed: the wrong SP shape desynchronises the whole packet rather than merely
        // losing the points.
        debug_assert!(!grant.sp_encoding_changes);
        out.push(self.job_change_reply(was, grant.job));
        // **The proof is spent.** It is the receipt for a test that happens once, and leaving
        // it in the bag would leave a second advancement's worth of evidence lying around for
        // a character who has already had one.
        if let Some(inv) = store::InventoryType::for_item(branch.chain.proof_item) {
            match self.take_quest_item(chr.id, inv, branch.chain.proof_item, 1) {
                Ok(replies) => out.extend(replies),
                Err(e) => out.extend(self.notice(format!(
                    "Your {} could not be taken back: {e}",
                    crate::secondjob::PROOF_ITEM_NAME
                ))),
            }
        }
        out
    }

    /// **A regular talk to one of the four Job Instructors.** `None` for any other NPC.
    ///
    /// The test is a quest (`secondjob::ExaminerTalk` has the owner's rules, 2026-09-26): accepting
    /// *Test of Qualification* sends the player in, and handing it in with the marbles is its
    /// completion. A regular talk only answers questions - and, while the test is under way and
    /// the marbles are short, offers to send them back in, on a Yes.
    pub(super) fn job_test_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        let branch = crate::secondjob::branch_examined_by(template)?;
        let marbles = self.held_count(chr.id, branch.chain.marble_item);
        let test = match self.store.quest_row(chr.id, branch.chain.quests[2]).ok().flatten().map(|r| r.state) {
            Some(store::QuestState::InProgress) => crate::secondjob::TestQuest::InProgress,
            Some(store::QuestState::Complete) => crate::secondjob::TestQuest::Completed,
            _ => crate::secondjob::TestQuest::NotStarted,
        };
        match crate::secondjob::examiner_talk(&chr, template, test, marbles)? {
            crate::secondjob::ExaminerTalk::OfferReEntry { question, .. } => {
                self.conversation = Some(Conversation {
                    npc_template: template,
                    quest_id: None,
                    path: crate::secondjob::REENTER_PATH.to_string(),
                    sent: 0,
                    awaiting_yes_no: true,
                    sent_with_next: false,
                });
                Some(vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_ask(template, &question, false),
                    what: format!("ScriptMessage AskYesNo from examiner {template}: back into the test area? - {question:?}"),
                }])
            }
            // **All thirty in hand: the talk IS the hand-in** (the owner, 2026-10-04: *"when the
            // player hands in the 30 marbles, they should be immediately accepting the quest that
            // gives them the proof of hero"*). It used to only point at the quest window. Now it
            // runs the client's own turn-in - `0x0151` action 2 for the Test of Qualification -
            // through the one path every turn-in takes, so the record, the experience, the
            // marbles leaving (`take_test_marbles`) and `Act.1.nextQuest` starting *Proof of
            // Qualification* with its Proof all happen in this one click, and nothing is
            // written twice: the store's `InProgress` guard still decides.
            crate::secondjob::ExaminerTalk::HandIn(_) => {
                let mut body = vec![net::script::QUEST_ACTION_COMPLETE];
                body.extend_from_slice(&branch.chain.quests[2].to_le_bytes());
                body.extend_from_slice(&template.to_le_bytes());
                body.extend_from_slice(&u32::MAX.to_le_bytes()); // no reward pick
                crate::server::log(&format!(
                    "   second-job test: {} talks to {} holding {marbles} marbles - handing the test in",
                    chr.name, branch.examiner_name
                ));
                Some(self.on_quest_request(&body))
            }
            crate::secondjob::ExaminerTalk::NothingMoreToTeach(line)
            | crate::secondjob::ExaminerTalk::NotReady(line)
            | crate::secondjob::ExaminerTalk::Passed(line) => Some(self.instructor_says(template, &line)),
        }
    }

    /// **The Test of Qualification takes its marbles on the turn-in.** All four test quests
    /// (`20002`/`20102`/`20202`/`20302`) gate completion on `Check.1.item` = 30 of the branch's
    /// marble and carry **no** `Act.1.item` take - the take belonged to the end script
    /// (`q20102e`) this client does not ship. So the marbles stayed in the bag after the test
    /// was passed (the owner, 2026-10-04: *"when the player hands in the 30 marbles"*). Nothing
    /// for any other quest. Called on a recorded completion only.
    fn take_test_marbles(&mut self, quest_id: u32) -> Vec<Reply> {
        let Some(branch) = crate::secondjob::BRANCHES.iter().find(|b| b.chain.quests[2] == quest_id) else {
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let marble = branch.chain.marble_item;
        let Some(inv) = store::InventoryType::for_item(marble) else { return Vec::new() };
        let count = u16::try_from(branch.chain.marble_count_items).unwrap_or(u16::MAX);
        match self.take_quest_item(chr.id, inv, marble, count) {
            Ok(replies) => replies,
            Err(e) => self.notice(format!("The test's {} could not be taken: {e}", crate::secondjob::MARBLE_ITEM_NAME)),
        }
    }

    /// The answer to the re-entry question. Yes sends them back in - **after asking again**
    /// whether the test is still under way and short, because the box may have sat on screen
    /// while the quest was handed in. No, or anything that changed, says nothing more.
    pub(super) fn test_reentry_answer(&mut self, template: u32, action: i8) -> Vec<Reply> {
        self.conversation = None;
        if action != net::script::SCRIPT_ACTION_YES {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(branch) = crate::secondjob::branch_examined_by(template) else { return Vec::new() };
        let in_progress = self
            .store
            .quest_row(chr.id, branch.chain.quests[2])
            .ok()
            .flatten()
            .is_some_and(|r| r.state == store::QuestState::InProgress);
        let marbles = self.held_count(chr.id, branch.chain.marble_item);
        if !in_progress || marbles >= branch.chain.marble_count_items {
            crate::server::log(&format!(
                "   second-job test: {} said Yes to re-entering, but the test is no longer short (in progress {in_progress}, {marbles} marbles); not sent in",
                chr.name
            ));
            return Vec::new();
        }
        self.enter_test_field(branch, &format!("{} goes back in from {}", chr.name, branch.examiner_name))
    }

    /// **The warden opens the door.** `None` for any NPC that is not one of the four.
    ///
    /// It never refuses. Those four maps carry exactly one portal each - the spawn point -
    /// so a warden that could say no is a warden that strands somebody.
    pub(super) fn job_test_exit_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let mut chr = self.claimed_character()?;
        let branch = crate::secondjob::branch_warded_by(template)?;
        let marbles = self.held_count(chr.id, branch.chain.marble_item);
        let step = crate::secondjob::warden_step(template, marbles)?;
        self.conversation = None;
        let who = chr.name.clone();
        let mut out = self.notice(step.line);
        // **Beside the instructor, not at the map's portal 0.** The owner, 2026-09-26: *"when leaving
        // the test area, the player should be placed right next to the spawn point at Magician
        // Job Instructor instead of at the origin of the map."* `secondjob::EXAMINER_SPAWN_PORTAL`.
        let beside = self
            .config
            .portal_index
            .get(&(step.to_map_id, crate::secondjob::EXAMINER_SPAWN_PORTAL.to_string()))
            .copied()
            .unwrap_or(0);
        out.extend(self.go_to_map(
            &mut chr,
            step.to_map_id,
            beside,
            format!(
                "second-job test: {who} leaves {} for {} ({}) at portal {beside} ({}, beside {}). That map id is the client's OWN returnMap and forcedReturn for the field, not a choice this server made",
                branch.test_field.map_name,
                step.to_map_id,
                branch.examiner_map_name,
                crate::secondjob::EXAMINER_SPAWN_PORTAL,
                branch.examiner_name
            ),
        ));
        Some(out)
    }

    /// **`startscript q20002s` and its three siblings**: starting the *Test of Qualification*
    /// puts the character in the hidden field.
    ///
    /// The client names four scripts it does not ship - `q20002s`, `q20102s`, `q20202s`,
    /// `q20302s` - and `research/quest-scripts.md` proved by enumeration that no body for any
    /// of them exists anywhere under `client-patched/`. This is that body: the quest's own
    /// `QuestInfo.1` says *"enter a hidden area, defeat the monsters there, and collect Black
    /// Marbles"*, and there is no other way in, so starting the quest and entering the field
    /// are the same event.
    ///
    /// Returns nothing for every other quest in the game.
    fn enter_test_field_on_quest_start(&mut self, quest_id: u32) -> Vec<Reply> {
        let Some(branch) = crate::secondjob::BRANCHES
            .iter()
            .find(|b| b.chain.quests[2] == quest_id)
        else {
            return Vec::new();
        };
        let who = self.claimed_character().map(|c| c.name).unwrap_or_default();
        self.enter_test_field(branch, &format!(
            "quest {quest_id} startscript (q{quest_id}s, which this client does NOT ship): {who} accepted the test"
        ))
    }

    /// **Into the hidden field**, with the words as a notice first - a script box sent with a
    /// `SetField` is torn down by the field entry. Shared by accepting the test and by the
    /// examiner's re-entry offer, so the two cannot drift.
    fn enter_test_field(&mut self, branch: &'static crate::secondjob::Branch, why: &str) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let field = branch.test_field;
        self.conversation = None;
        let mut out = self.notice(format!(
            "Into {} with you. Bring the {} back {} {}s - talk to the instructor inside when \
             you want to come out.",
            field.map_name,
            branch.examiner_name,
            branch.chain.marble_count_items,
            crate::secondjob::MARBLE_ITEM_NAME
        ));
        out.extend(self.go_to_map(
            &mut chr,
            field.map_id,
            0,
            format!("second-job test: {why} - enters {} ({})", field.map_id, field.map_name),
        ));
        out
    }

    /// **The third advancement**, or the sentence saying why not. `None` for any NPC that is
    /// not one of Tylus, Robeira, Rene or Arec.
    ///
    /// # It advances on the click, with no box in between
    ///
    /// The first advancement does the same; the second one asks, because a Swordsman has two
    /// or three destinations and someone has to pick. **A third-job character has exactly
    /// one** - `111` is the book under `110` in `Skill.wz`, ten times over - so a menu here
    /// would be a list of length one and a yes/no box would be a question with one answer.
    /// The owner, 2026-08-31, choosing between the options: *"No test - level 70 and click."*
    ///
    /// # This client ships no third-job test, and that is measured
    ///
    /// All 322 quests were enumerated and there is nothing above `20303`; every one-portal map
    /// in the archive was enumerated and the only four with a job NPC are the second job's;
    /// there are no third-job test mobs and no third-job items. `research/third-job.md` §3.
    /// So the gate is level and job, and there was never a chain to enforce instead.
    ///
    /// # Every effect hangs off the transition
    ///
    /// The grant is computed **before** the save, so a job that cannot be granted never
    /// reaches the database; then the write is the transition and the packet hangs off it.
    /// `CLAUDE.md`'s Heena section - the guard that was asked and its answer ignored.
    pub(super) fn third_advancement_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let mut chr = self.claimed_character()?;
        match crate::thirdjob::advancement_for(&chr, template) {
            // Not one of the four - fall through to whatever this NPC normally does.
            crate::thirdjob::Advancement::NotAnInstructor => None,
            crate::thirdjob::Advancement::Eligible { third, master } => {
                // Before the save, deliberately. `Eligible` already implies this succeeds, but
                // a `?` here after the write would leave the database advanced and the client
                // never told, which is the one state that cannot be recovered from on screen.
                let Some(grant) = crate::thirdjob::grant(third.job, chr.level) else {
                    return Some(self.instructor_says(
                        template,
                        "Something went wrong and your job was not changed.",
                    ));
                };
                let was = chr.job;
                chr.job = grant.job;
                if let Err(e) = self.store.save_character_progress(&chr) {
                    // The save is the transition. Nothing follows a failed one.
                    return Some(self.instructor_says(
                        template,
                        &format!("Something went wrong and your job was not changed: {e}"),
                    ));
                }
                let mut out = self.instructor_says(
                    template,
                    &format!(
                        "Then it is done. You are a {} now. {} has nothing left to teach you \
                         that your own road will not - open your skill window; there is a \
                         third page on it.",
                        grant.job_name, master.name
                    ),
                );
                // False for all ten - every `x10 -> x11` pair is on the same side of
                // `uses_extended_sp`. Asserted rather than assumed: the wrong SP shape
                // desynchronises the whole packet rather than merely losing the points.
                debug_assert!(!grant.sp_encoding_changes);
                out.push(self.job_change_reply(was, grant.job));
                Some(out)
            }
            // Every other arm is a refusal with a sentence already written for it, and the
            // sentence comes from `thirdjob` so this file and that one cannot drift.
            other => Some(self.instructor_says(template, &crate::thirdjob::refusal_for(&other)?)),
        }
    }

    // -----------------------------------------------------------------------------------
    // `!tool` - the three daily favours
    //
    // The decision, the table and every sentence live in `crate::dailyperks`; the gate lives
    // in `store::dailyperks`. What is here is the part that needs `&mut Session`: the wallet,
    // `award_experience`, and the two reset commands.
    //
    // **This used to hang off template 9010000 and no longer does.** The placed Maple
    // Administrator has quest 500005 back; nothing above this line names their template. The
    // entry point is `open_daily_perks`, called from the chat dispatcher.
    // -----------------------------------------------------------------------------------

    /// **`!tool` - put the daily-favour menu on screen.** The one entry point into this
    /// feature.
    ///
    /// This **never returns nothing**, even with no character claimed. It is a reply to typed
    /// chat, which holds no latch - but a command that silently does nothing and a command that
    /// silently works look identical on screen, and telling those apart has cost client
    /// launches (`session::gm`'s own module doc). So the empty case is a sentence.
    ///
    /// **The speaker is [`crate::dailyperks::ADMIN_TEMPLATE`], and that is the icon, not a
    /// routing key.** `0x055B`'s speaker field goes straight into the `Npc/%07d.img` loader
    /// (`research/npc-click.md` §3.2), so it is the portrait beside the text and nothing more -
    /// The owner: *"Just use MapleStory administrator as the NPC icon."* No NPC object is spawned:
    /// see the module doc in `crate::dailyperks` for the measurement that says a runtime spawn
    /// would work and the listing that says its click could not be told from theirs.
    ///
    /// `awaiting_yes_no` is deliberately `false`: a menu is not a yes/no box, and if this
    /// branch were ever skipped a stray reply must not be mistaken for a quest Accept.
    pub(super) fn open_daily_perks(&mut self) -> Vec<Reply> {
        let template = crate::dailyperks::ADMIN_TEMPLATE;
        let Some(chr) = self.claimed_character() else {
            // A dialogue rather than silence. There is nothing to offer, but saying so is the
            // difference between a command that refused and a command that is not wired.
            return self.admin_says(
                template,
                "I cannot find your record just now. Nothing has been used up - try me again.",
                "no character is claimed on this connection",
            );
        };
        let used = self.daily_perks_used(chr.id);
        // **Replaced rather than added to.** Ten `!tool`s in a row are ten boxes and one
        // conversation: the field holds a single `Option`, so nothing accumulates here, on the
        // map, or in the database. That is the whole lifetime story - there is no object to
        // clean up on a map change or a relog, because none was created.
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: crate::dailyperks::MENU_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        let text = crate::dailyperks::menu_text(used);
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(template, &text),
            what: format!(
                "ScriptMessage MENU (type 6) opened by {} for character {}: {} of {} favours already used on UTC day {} ({}). Speaker template {template} is the {}'s PORTRAIT only - the NPC standing in Henesys keeps quest {}",
                crate::dailyperks::COMMAND_TYPED,
                chr.id,
                used.iter().filter(|u| **u).count(),
                crate::dailyperks::PERKS.len(),
                store::today(),
                store::utc_date(store::today()),
                crate::dailyperks::ADMIN_NAME,
                crate::dailyperks::ADMIN_QUEST,
            ),
        }]
    }

    /// Which of the three this character has already had today, in `PERKS` order.
    ///
    /// **This is the display, not the gate**, and the difference matters: a database read that
    /// fails here marks the option **available**, because the claim itself is atomic and will
    /// refuse if it really was taken. Marking it used would lock somebody out of a favour they
    /// are owed on the strength of one failed `SELECT`. The gate can afford to fail closed;
    /// the picture of it cannot.
    fn daily_perks_used(&self, character_id: u32) -> [bool; crate::dailyperks::PERKS.len()] {
        let today = store::today();
        let mut out = [false; crate::dailyperks::PERKS.len()];
        for (i, perk) in crate::dailyperks::PERKS.iter().enumerate() {
            let Some(scope_id) = self.daily_perk_scope_id(*perk, character_id) else { continue };
            out[i] = match self.store.daily_claim_day(perk.scope(), scope_id, perk.store_key()) {
                Ok(Some(day)) => day >= today,
                Ok(None) => false,
                Err(e) => {
                    crate::server::log(&format!(
                        "dailyperks: could not read the {:?} claim for character {character_id}: {e} - showing it as AVAILABLE; the claim itself is the gate",
                        perk
                    ));
                    false
                }
            };
        }
        out
    }

    /// The id a perk's claim row is keyed on: the character for [`store::SCOPE_CHARACTER`],
    /// the account for [`store::SCOPE_ACCOUNT`].
    ///
    /// `None` only for an account-scoped perk with no claimed migration, which cannot happen
    /// today because all three perks ship character-scoped - see `dailyperks::Perk::scope`.
    fn daily_perk_scope_id(&self, perk: crate::dailyperks::Perk, character_id: u32) -> Option<i64> {
        if perk.scope() == store::SCOPE_ACCOUNT {
            return self.claimed().map(|c| c.account_id);
        }
        Some(i64::from(character_id))
    }

    /// The player picked a favour off the menu. `None` means "not mine" - fall through.
    ///
    /// **The path is the only thing that says who asked**: a type-6 body carries no speaker,
    /// so this checks `dailyperks::is_menu_path` before it decodes anything, exactly as the
    /// taxi's and the instructor's do. The three prefixes are disjoint and a test says so.
    fn daily_perk_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if !crate::dailyperks::is_menu_path(&convo.path) {
            return None; // a taxi's, an instructor's, Phil's, or a quest's
        }
        let reply = net::script::parse_menu_reply(body)?;
        let template = convo.npc_template;
        // The box is gone from the screen either way; a stale conversation is what the next
        // reply walks into.
        self.conversation = None;
        // Closed rather than chosen. Nothing was decided, so nothing happens and nothing is
        // said - `0x00F3` does not hold the one-request latch, so silence here is safe and is
        // measured (`research/script-reply.md` §5.1, §5.2). Claimed rather than fallen through,
        // so the Say-shaped decoder never sees a 10-byte type-6 body.
        let Some(selection) = reply.selection else { return Some(Vec::new()) };
        let Some(perk) = crate::dailyperks::perk_at(selection) else {
            return Some(self.admin_says(
                template,
                &crate::dailyperks::no_such_option(),
                &format!(
                    "selection {selection} names no favour; there are {} (a -2 is the client's own special path, 141f739b4). NOTHING CLAIMED",
                    crate::dailyperks::PERKS.len()
                ),
            ));
        };
        Some(self.grant_daily_perk(template, perk))
    }

    /// **Claim a day and then pay for it, in that order and never the other.**
    ///
    /// # Every effect hangs off the transition
    ///
    /// `CLAUDE.md`'s Heena section: `store::complete_quest` guarded correctly for weeks while
    /// the payout sat *outside* the match on its answer, so a repeat click re-paid the quest.
    /// The shape that avoids it here is three steps and the order is the whole design:
    ///
    /// 1. **Pre-check, before anything is claimed.** A character at the level cap, or one with
    ///    nothing spent to reset, is refused *without spending their day*. A refusal that costs
    ///    a day is worse than no feature.
    /// 2. **The claim**, [`store::Store::claim_daily_perk_now`] - one `BEGIN IMMEDIATE`, one
    ///    answer. This is the transition. `DailyClaimOutcome::AlreadyToday` returns here and
    ///    nothing below it runs; there is no field on that arm to pay from.
    /// 3. **The grant.** If it fails, the day is handed straight back with
    ///    [`store::Store::release_daily_perk`] and the player is told so.
    ///
    /// # Why the claim and the grant are not literally one SQL transaction
    ///
    /// Said plainly rather than implied. The brief asked for one transaction, and the claim
    /// **is** one - the read and the write that decide whether anything is owed cannot be
    /// interleaved by a second packet. The grant is not inside it, and cannot be: paying a
    /// favour means `save_character_progress`, `add_maple_points`, `take_ap_spend` and
    /// `forget_all_skills_and_refund`, each of which takes the store's own connection lock and
    /// opens its own transaction. Nesting them would deadlock on the `Mutex<Connection>` that
    /// makes `Store` `Sync`.
    ///
    /// So the property that actually holds is: **a grant is impossible without a transition,
    /// and a transition without a grant is undone.** The window between them is a few
    /// microseconds on one thread, and its failure mode is a returned day rather than a double
    /// payout. Step 3 is what makes that true, and it is the reason `Claimed` carries the
    /// previous day at all.
    fn grant_daily_perk(&mut self, template: u32, perk: crate::dailyperks::Perk) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self.admin_says(
                template,
                "I cannot find your record just now. Nothing has been used up.",
                "no character is claimed; NOTHING CLAIMED",
            );
        };
        // 0. **A cosmetic read, and it is NOT the guard.** Without it the reset says *"you have
        // nothing spent to give back"* on a second click - true, because the first click spent
        // it, but it answers a question nobody asked. Asking first whether the day is already
        // gone puts the right sentence on screen.
        //
        // It is deliberately a *read* placed before a *refusal*, never before a grant: it can
        // only ever turn one refusal into a different refusal. The transition below is
        // untouched and is still the only thing that authorises a payment, so a stale or
        // failed read here costs a worse sentence and nothing else.
        if self.daily_perks_used(chr.id)[perk.selection() as usize] {
            let day = self
                .daily_perk_scope_id(perk, chr.id)
                .and_then(|sid| self.store.daily_claim_day(perk.scope(), sid, perk.store_key()).ok())
                .flatten()
                .unwrap_or_else(store::today);
            return self.admin_says(
                template,
                &crate::dailyperks::already_used_today(perk, day),
                &format!(
                    "{perk:?} REFUSED: {} {} already claimed it on UTC day {day} ({}).                      NOTHING PAID",
                    perk.scope(),
                    self.daily_perk_scope_id(perk, chr.id).unwrap_or(i64::from(chr.id)),
                    store::utc_date(day)
                ),
            );
        }
        // 1. Refusals that must not cost a day.
        if let Some(refusal) = self.daily_perk_refusal(perk, &chr) {
            return self.admin_says(
                template,
                &refusal,
                &format!("{perk:?} REFUSED before the claim - NOTHING CLAIMED, the day is intact"),
            );
        }
        let Some(scope_id) = self.daily_perk_scope_id(perk, chr.id) else {
            return self.admin_says(
                template,
                "I cannot tell whose day this would be. Nothing has been used up.",
                &format!("{perk:?}: no scope id for scope {}; NOTHING CLAIMED", perk.scope()),
            );
        };
        let scope = perk.scope();
        let key = perk.store_key();

        // 2. THE TRANSITION.
        let outcome = match self.store.claim_daily_perk_now(scope, scope_id, key) {
            Ok(o) => o,
            Err(e) => {
                return self.admin_says(
                    template,
                    "My ledger will not open just now. Nothing has been used up - try me again.",
                    &format!("{perk:?}: the claim FAILED ({e}); NOTHING CLAIMED and NOTHING PAID"),
                )
            }
        };
        let store::DailyClaimOutcome::Claimed { day, previous } = outcome else {
            // Already had it today. There is no field on this arm to pay from, which is the
            // point of the type.
            return self.admin_says(
                template,
                &crate::dailyperks::already_used_today(perk, outcome.day()),
                &format!(
                    "{perk:?} REFUSED: {} {scope_id} already claimed it on UTC day {} ({}). \
                     NOTHING PAID",
                    scope,
                    outcome.day(),
                    store::utc_date(outcome.day())
                ),
            );
        };

        // 3. The grant. Nothing above this line has paid anything.
        crate::server::log(&crate::dailyperks::claim_note(perk, scope_id, day));
        match self.apply_daily_perk(template, perk, &chr) {
            Ok(replies) => replies,
            Err(why) => {
                // The day goes straight back. A player who is told nothing, given nothing and
                // locked out until tomorrow is the worst outcome this feature has.
                let restored = self.store.release_daily_perk(scope, scope_id, key, previous);
                let note = match restored {
                    Ok(()) => "the day has been given back",
                    Err(_) => "AND THE DAY COULD NOT BE GIVEN BACK - the claim row still says today",
                };
                self.admin_says(
                    template,
                    &crate::dailyperks::grant_failed_day_returned(perk, &why),
                    &format!("{perk:?} claimed for character {} on UTC day {day} and the grant FAILED ({why}); {note}", chr.id),
                )
            }
        }
    }

    /// The refusals that are decided **before** a day is spent. `None` means "go ahead".
    ///
    /// Leaf Points can always be granted, so it has no arm here.
    fn daily_perk_refusal(
        &self,
        perk: crate::dailyperks::Perk,
        chr: &net::opcode::Character,
    ) -> Option<String> {
        match perk {
            crate::dailyperks::Perk::LeafPoints => None,
            crate::dailyperks::Perk::LevelUp => {
                if chr.level >= crate::expcurve::MAX_LEVEL {
                    return Some(crate::dailyperks::already_max_level(chr.level));
                }
                // A level with no row in `data/exp-curve.txt` has no price, so there is no
                // "exactly the experience needed" to grant. Cannot happen with the shipped file
                // (1..119) and is a sentence rather than a panic because a missing data file
                // must not read as a frozen client.
                if self.config.exp_curve.to_next(chr.level).is_none() {
                    return Some(crate::dailyperks::no_curve_for_level(chr.level));
                }
                None
            }
            crate::dailyperks::Perk::ReturnToHenesys => {
                // Already there: a SetField to the map you are standing on is a reload, not a
                // rescue, and it must not eat the day.
                if chr.map_id == crate::dailyperks::HENESYS {
                    return Some(crate::dailyperks::already_in_henesys());
                }
                // A client whose field table cannot vouch for Henesys would be stranded, not
                // rescued - the same guard `!map` applies. Refused before the claim.
                if !self.config.map_exists(crate::dailyperks::HENESYS) {
                    return Some(
                        "I cannot find the road to Henesys just now. Nothing has been used up."
                            .to_string(),
                    );
                }
                None
            }
        }
    }

    /// Pay a favour that has already been claimed. `Err` gives the day back.
    ///
    /// **Each arm checks that its own effect actually landed**, rather than trusting that the
    /// call it made worked. `CLAUDE.md`: *"A test that checks one of several effects gives
    /// false confidence about the rest"* - and the same is true of a handler. `award_experience`
    /// answers a save failure with a chat notice rather than a signal, so the level is verified
    /// by re-reading the character; `gm_reset_ap` reports its own failure the same way, so the
    /// AP pool is re-read too.
    fn apply_daily_perk(
        &mut self,
        template: u32,
        perk: crate::dailyperks::Perk,
        chr: &net::opcode::Character,
    ) -> Result<Vec<Reply>, String> {
        match perk {
            // ---- 1000 Leaf Points ----------------------------------------------------
            //
            // **The wallet is per ACCOUNT** (`store::cash`) and so, since 2026-09-08, is the
            // allowance - the owner: *"Make leaf point claim per account."* 1000 a day means 1000 a
            // day however many characters the account has. `daily_perk_scope_id` returns the
            // account id for this perk, so the row this spends is the same one every character
            // on the account reads.
            //
            // Nothing pushes a wallet update to the client: the balance travels in the
            // `0x05AD` that goes out with `SetCashShop`, and the client's own poll is
            // throttled to once a minute. So the sentence carrying the new balance IS the
            // feedback until the shop is next opened.
            crate::dailyperks::Perk::LeafPoints => {
                let account_id = self
                    .claimed()
                    .map(|c| c.account_id)
                    .ok_or_else(|| "there is no account on this connection".to_string())?;
                let balance = self
                    .store
                    .add_maple_points(account_id, i64::from(crate::dailyperks::LEAF_POINTS_PER_CLAIM))
                    .map_err(|e| e.to_string())?;
                Ok(self.admin_says(
                    template,
                    &crate::dailyperks::granted(&crate::dailyperks::leaf_points_line(
                        crate::dailyperks::LEAF_POINTS_PER_CLAIM,
                        balance,
                    )),
                    &format!(
                        "daily perk LeafPoints PAID: account {account_id} +{} LP -> {balance}. Per-ACCOUNT wallet, per-CHARACTER claim",
                        crate::dailyperks::LEAF_POINTS_PER_CLAIM
                    ),
                ))
            }

            // ---- exactly one level ---------------------------------------------------
            //
            // **The same curve as everything else.** `config.exp_curve` is what a kill and a
            // quest turn-in both go through, so a level bought here costs what a level costs.
            // `to_next(level)` is the price of the NEXT level and `chr.exp` is what is already
            // banked toward it, so `need - exp` is *exactly* the shortfall and no more: the
            // character arrives at the new level with zero experience toward the one after it.
            //
            // **No rate multiplier.** `Session::rate` is applied by the *callers* of
            // `award_experience`, not inside it, so a 5x event does not turn this into five
            // levels - which would be a different feature.
            crate::dailyperks::Perk::LevelUp => {
                let need = self
                    .config
                    .exp_curve
                    .to_next(chr.level)
                    .ok_or_else(|| format!("level {} has no row in the experience curve", chr.level))?;
                // `max(1)` covers the one state the curve cannot: experience already at or past
                // the threshold, which means a level was banked and never applied. Awarding 0
                // would be a no-op that still spent the day.
                let gained = need.saturating_sub(chr.exp).max(1);
                let before = chr.level;
                let mut out = self.award_experience(
                    gained,
                    &format!("the {}'s daily level", crate::dailyperks::ADMIN_NAME),
                    true,
                    true,
                );
                // **The transition, re-read rather than assumed.** `award_experience` answers a
                // failed `save_character_progress` with a chat notice and no signal, so the only
                // honest check is whether the level actually moved.
                let after = self
                    .claimed_character()
                    .ok_or_else(|| "your record could not be read back".to_string())?;
                if after.level <= before {
                    return Err(format!(
                        "your level did not move (still {before} after {gained} experience)"
                    ));
                }
                out.extend(self.admin_says(
                    template,
                    &crate::dailyperks::granted(&crate::dailyperks::level_up_line(
                        before, after.level, gained,
                    )),
                    &format!(
                        "daily perk LevelUp PAID: character {} +{gained} exp (curve says level {before} costs {need}, {} was banked) -> level {}",
                        chr.id, chr.exp, after.level
                    ),
                ));
                Ok(out)
            }

            // ---- Return to Henesys --------------------------------------------------
            //
            // The grant IS the SetField. `go_to_map` does everything a portal walk does -
            // hands the mobs over, tells the map we left, moves the record - and the effect
            // is verified by re-reading the record's map rather than by trusting the call.
            //
            // **No Say afterwards, on purpose.** A `SetField` rebuilds the client's whole
            // screen; a dialogue box queued behind it would pop up over Henesys with nothing
            // to say that the teleport has not already said. The menu reply holds no latch,
            // so the SetField alone is a complete answer. The log line carries the receipt.
            crate::dailyperks::Perk::ReturnToHenesys => {
                let from = chr.map_id;
                let mut moved = chr.clone();
                let out = self.teleport(
                    &mut moved,
                    crate::dailyperks::HENESYS,
                    format!(
                        "daily perk ReturnToHenesys PAID: {} - the once-a-day escape for character {}, from map {from}",
                        crate::dailyperks::COMMAND_TYPED, chr.id
                    ),
                );
                let after = self
                    .claimed_character()
                    .ok_or_else(|| "your record could not be read back".to_string())?;
                if after.map_id != crate::dailyperks::HENESYS {
                    return Err(format!(
                        "the record still says map {} - the road to Henesys did not take",
                        after.map_id
                    ));
                }
                Ok(out)
            }
        }
    }

    /// One `0x055B` Say from the Maple Administrator, with no conversation left behind.
    ///
    /// Same shape and same reason as [`Session::instructor_says`]: this is a sentence the
    /// server composed, not a walk through a WZ line list, and leaving a stale `Conversation`
    /// behind it is how a later reply walks into the wrong state machine. `why` is the log
    /// label, so `world.log` records whether a day was spent without the sentence having to
    /// say so on screen.
    pub(super) fn admin_says(&self, template: u32, text: &str, why: &str) -> Vec<Reply> {
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(template, text, false, false),
            what: format!(
                "ScriptMessage Say from the {} (template {template}): {why} | {text:?}",
                crate::dailyperks::ADMIN_NAME
            ),
        }]
    }

    /// One `0x055B` from an instructor, with no conversation state behind it.
    ///
    /// Deliberately **not** routed through [`Session::say_line`]: that walks a WZ line list and
    /// leaves a `Conversation` for the next `0x00F3` to step through. This is a single
    /// sentence the server composed, and leaving a stale conversation behind it is how a later
    /// reply walks into the wrong state machine.
    fn instructor_says(&self, template: u32, text: &str) -> Vec<Reply> {
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(template, text, false, false),
            what: format!("ScriptMessage Say from instructor {template}: {text:?}"),
        }]
    }

    pub(super) fn say_line(&mut self, index: usize) -> Vec<Reply> {
        let Some(convo) = self.conversation.clone() else { return Vec::new() };
        let Some(lines) = self.say_lines(&convo) else {
            self.conversation = None;
            return Vec::new();
        };
        let Some(text) = lines.get(index).cloned() else {
            self.conversation = None;
            return Vec::new();
        };

        let last = index + 1 >= lines.len();
        // Only offer Accept/Decline while walking a state's own lines. On a branch the user
        // has already answered, and asking again is the loop the owner hit.
        let on_branch = convo.path.contains('.');
        // **The Test of Qualification ends its opening on Accept/Decline.** The owner, 2026-09-26:
        // *"Upon accepting the quest, it should teleport me into the test map."* The quest is
        // `startscript q20x02s`, which the client does NOT ship, so it hands the whole start to
        // the server (action 4) and draws no Accept of its own; `Say.0` has no `yes` branch
        // either. Before this the three lines ended on an OK that started nothing
        // (world-ch0.log 2026-09-26 02:03:02, three tries). The Yes lands in `accept_quest`,
        // which records the start and - on that transition only - warps them in.
        let branches = last
            && !on_branch
            && convo.quest_id.is_some()
            && (self.has_branch(&convo, "yes") || self.opens_the_test(&convo));
        let has_next = !last;

        // **A quiz's `#L` menu is never sent from here.** Quest 1013's `Say.1.0` carries four
        // `#L<n>#` choices; sent as a Say (type 0) it faulted the client 22 ms later
        // (`research/fixtures/rain-quiz-say-with-menu-tags-client-fault-*`). It is not sent as a
        // Say now either - it is not sent AT ALL: the client conducts the quiz from its own
        // `Quest.wz` and sends only the turn-in, which `on_quest_request` finalises without a
        // box. So `say_line` only ever walks non-quiz lines and a quiz path never reaches it.
        // `a_quiz_turn_in_completes_silently_because_the_client_conducts_the_quiz`.

        let body = if branches {
            net::script::npc_ask(convo.npc_template, &text, true)
        } else {
            net::script::npc_say(convo.npc_template, &text, false, has_next)
        };
        let what = format!(
            "ScriptMessage {} from NPC template {}{}, line {} of {} on path \"{}\"",
            if branches { "yes/no prompt" } else { "Say" },
            convo.npc_template,
            convo.quest_id.map(|q| format!(" for quest {q}")).unwrap_or_default(),
            index + 1,
            lines.len(),
            convo.path,
        );

        if let Some(c) = self.conversation.as_mut() {
            c.sent = index;
            c.awaiting_yes_no = branches;
            c.sent_with_next = has_next;
        }
        vec![Reply { opcode: net::script::SCRIPT_MESSAGE, body, what }]
    }



    /// The lines of the path the conversation is currently on.
    pub(super) fn say_lines(&self, convo: &Conversation) -> Option<Vec<String>> {
        match convo.quest_id {
            Some(q) => self.config.quests.get(&q)?.say.get(&convo.path).cloned(),
            // No quest: the NPC's own d0 line, as a one-line conversation.
            None => Some(vec![self.npc_line(convo.npc_template)]),
        }
    }


    /// Is this conversation the opening of a *Test of Qualification* the character has not
    /// started? Then its last line is the Accept box (`say_line`).
    pub(super) fn opens_the_test(&self, convo: &Conversation) -> bool {
        let Some(quest_id) = convo.quest_id else { return false };
        if convo.path != "0" || !crate::secondjob::is_test_quest(quest_id) {
            return false;
        }
        let Some(chr) = self.claimed_character() else { return false };
        self.store.quest_row(chr.id, quest_id).ok().flatten().is_none()
    }

    /// Does the current path have a `yes` / `no` branch under it?
    pub(super) fn has_branch(&self, convo: &Conversation, branch: &str) -> bool {
        let Some(q) = convo.quest_id.and_then(|q| self.config.quests.get(&q)) else {
            return false;
        };
        q.say.contains_key(&format!("{}.{}", convo.path, branch))
    }


    /// The client's answer to a script box.
    ///
    /// **An unanswered one costs a dead conversation and nothing else** - `0x00F3` is *not*
    /// one of the 37 functions that set `player->[0x2330]`, the one-request-outstanding
    /// latch, and the dialog is destroyed and the script-manager latch released before the
    /// packet is even built. So ending a conversation by sending nothing is safe, which is
    /// what this does whenever there is nothing left to say. **[L]**
    ///
    /// **The Say answer cannot distinguish OK from Next** - the client rewrites `BtOK` to
    /// the Next result at `142a59fb5`, so both arrive as `1`. The server therefore has to
    /// remember whether the box it sent had `next` set, and it does: `sent_with_next`. That
    /// is inference from our own state rather than something read off the wire, and it is
    /// worth knowing which of the two it is.
    pub(super) fn on_script_reply(&mut self, body: &[u8]) -> Vec<Reply> {
        // **A taxi's menu is message type 6, which `parse_script_reply` cannot decode.** Its
        // fall-through arm reads a Say-shaped `u32 echo, str, u8`; against a 10-byte type-6
        // body the string's u16 length wants two bytes that are not there, the reader errors,
        // and the whole packet is dropped - the same silent failure that cost Roger's quest.
        //
        // This comes FIRST, before the Say-shaped decoder, and it is the branch with a
        // precondition: it answers only when this session has a taxi conversation parked.
        if let Some(replies) = self.taxi_menu_answer(body) {
            return replies;
        }
        // The citizenship contract window (types 0x42..0x46, six-byte answers, two per
        // accepted contract) and the clerk's menu - each only while its own `citizenship.`
        // path is parked. session/citizenship.rs.
        if let Some(replies) = self.citizenship_contract_answer(body) {
            return replies;
        }
        if let Some(replies) = self.citizenship_menu_answer(body) {
            return replies;
        }
        // The salons' coupon menu and their pick-a-look box (type 0x0a) - each only when its
        // own salon conversation is parked. session/salon.rs.
        // Cloto's stage-1 menu for the party leader, and Lakelis' own. session/firsttime.rs.
        if let Some(replies) = self.cloto_menu_answer(body) {
            return replies;
        }
        if let Some(replies) = self.lakelis_menu_answer(body) {
            return replies;
        }
        // Joel's tickets and Cherry's boarding - `boat.` paths. session/boat.rs.
        if let Some(replies) = self.boat_menu_answer(body) {
            return replies;
        }
        if let Some(replies) = self.hotel_menu_answer(body) {
            return replies;
        }
        if let Some(replies) = self.jump_quest_menu_answer(body) {
            return replies;
        }
        if let Some(replies) = self.salon_menu_answer(body) {
            return replies;
        }
        if let Some(replies) = self.salon_choice_answer(body) {
            return replies;
        }
        if let Some(replies) = self.salon_mix_answer(body) {
            return replies;
        }
        // **And so is the second advancement's choice box.** Same precondition, disjoint
        // path prefix: each answers only when this session has *its* conversation parked, so
        // the order between the two does not matter and a test in each module says so.
        if let Some(replies) = self.second_job_menu_answer(body) {
            return replies;
        }
        // **And Phil's job guide**, since 2026-09-15 a menu rather than a yes/no chain -
        // `jobguide.menu`, a path the taxi and the instructors each assert they do not claim.
        if let Some(replies) = self.jobguide_menu_answer(body) {
            return replies;
        }
        // **And so is the Maple Administrator's.** Third feature on one packet type, same
        // precondition and a third disjoint path prefix, so the order between the three does
        // not matter - `dailyperks::this_menu_path_cannot_be_confused_with_a_taxi_or_an_
        // instructor` asserts all six directions.
        // A fourth disjoint prefix, `scroll.` - see `scrollnpc::is_scroll_path`, whose test
        // asserts it cannot be confused with the other three.
        if let Some(replies) = self.scroll_menu_answer(body) {
            return replies;
        }
        if let Some(replies) = self.gift_menu_answer(body) {
            return replies;
        }
        if let Some(replies) = self.daily_perk_menu_answer(body) {
            return replies;
        }
        // A fifth disjoint prefix, `package.frieren:` - the Frieren version chooser
        // (session/cashitem.rs). `the_receipt_path_cannot_be_confused_with_any_other_menu`
        // asserts the disjointness for it and for the receipt's `package.receipt`.
        if let Some(replies) = self.frieren_menu_answer(body) {
            return replies;
        }
        let Some(reply) = net::script::parse_script_reply(body) else { return Vec::new() };
        let Some(convo) = self.conversation.clone() else { return Vec::new() };

        // Shanks' waiver box sails on ANY dismissal, Close included, so it sits above the
        // closed-box early return. See `shanks_announce_dismissed`.
        if convo.path == crate::shanks::ANNOUNCE_PATH {
            return self.shanks_announce_dismissed();
        }

        // A jump quest's found box hands over on Close as well as OK, so it is answered before
        // a closed box is dropped. `jumpquest.` paths. session/jumpquest.rs.
        if let Some(replies) = self.jump_quest_script_answer(&convo.path, reply.action) {
            return replies;
        }

        if reply.action == net::script::SCRIPT_ACTION_CLOSED {
            self.conversation = None;
            return Vec::new();
        }


        // Shanks' own yes/no, before the generic quest branch: the quest arm would claim it,
        // find no quest id, and drop the conversation with NO PACKET SENT - and they would go
        // silent on the first Yes. (Phil's guide used to sit here for the same reason; it is
        // a type-6 menu now and is answered above, beside the taxi's.)
        if convo.path == crate::firsttime::CLOTO_INTRO_PATH {
            return self.cloto_intro_answer(reply.action);
        }
        if convo.path == crate::firsttime::NELLA_PATH {
            return self.nella_answer(reply.action);
        }
        // "Look at my wares?" from a shop a quest opens. session/questshop.rs.
        if convo.path == questshop::ASK_PATH {
            return self.quest_shop_answer(convo.npc_template, reply.action);
        }
        // The ships: a seller's Next, a boarder's or steward's yes/no, the Platform Usher's.
        // `boat.` paths, both routes. session/boat.rs.
        if let Some(replies) = self.boat_script_answer(reply.action, store::Store::unix_now()) {
            return replies;
        }
        if convo.path == crate::shanks::ASK_PATH {
            return self.shanks_reply(reply.action);
        }

        // The Job Instructor's "back into the test area?" - no quest id either, so it goes
        // before the generic quest branch for the same reason. The owner, 2026-09-26.
        if convo.path == crate::secondjob::REENTER_PATH {
            return self.test_reentry_answer(convo.npc_template, reply.action);
        }

        // The first job instructor's yes/no, before the generic quest branch for the same
        // reason: it has no quest id and would be dropped with NO PACKET SENT.
        if convo.path == crate::jobs::ASK_PATH {
            return self.first_job_reply(convo.npc_template, reply.action);
        }

        // `!scroll`'s confirm, before the generic quest branch and for exactly the reason
        // Shanks' is: the quest arm would claim it, find no quest id, and drop the
        // conversation with NO PACKET SENT - so a Yes would silently do nothing.
        if crate::scrollnpc::is_scroll_path(&convo.path) {
            return self.scroll_confirm_answer(reply.action);
        }

        if convo.awaiting_yes_no {
            let accepted = reply.action == net::script::SCRIPT_ACTION_YES;
            let branch = if accepted { "yes" } else { "no" };
            // **Record the acceptance before the branch-text check**, not after. A quest
            // whose `yes` path has no line in `Quest.wz` is still a quest the player just
            // accepted, and ordering these the other way would silently drop exactly those.
            //
            // **Only an OFFER's Yes accepts** - a Say path under state `0`. The weekly
            // donations' completion talk (`1`, "...are you saying you'd like to donate?") is a
            // yes/no too, and its Yes was being taken as accepting the quest again: on the live
            // server 2026-10-02 a turn-in answered Yes, the start gate refused ("already done
            // this week"), and the `1.yes` line followed the refusal - two `0x055B` boxes at
            // once. The client rejected the second (`0x009E`) and dropped 15 s later. The
            // turn-in is recorded by `0x0151` action 2 before this talk opens, so the Yes here
            // only reads the `1.yes` line.
            let offer = convo.path.split('.').next() == Some("0");
            let mut out = if accepted && offer { self.accept_quest(&convo) } else { Vec::new() };
            // **A refused accept is the whole answer.** `citizenship_start_gate` and the bag
            // check park their own box under `REFUSAL_PATH`; a branch line after it is a
            // second script box on screen at once, which is what took that client down.
            if self.conversation.as_ref().is_some_and(|c| c.path == crate::questroom::REFUSAL_PATH) {
                return out;
            }
            if !self.has_branch(&convo, branch) {
                self.conversation = None;
                return out;
            }
            if let Some(c) = self.conversation.as_mut() {
                c.path = format!("{}.{}", convo.path, branch);
                c.sent = 0;
            }
            out.extend(self.say_line(0));
            return out;
        }

        if !convo.sent_with_next {
            self.conversation = None; // that was an OK on the last box
            return Vec::new();
        }
        self.say_line(convo.sent + 1)
    }


    /// The player pressed Yes on a quest's offer: write it down, and tell the journal.
    ///
    /// **This is the half that was missing.** The dialogue already worked and already
    /// answered the `yes` branch, so on screen the conversation looked complete - but
    /// nothing in this crate had ever called `store::start_quest`, and the quest journal is
    /// built entirely from the `quest_state` table. The owner, after the run of 2026-08-19:
    /// *"Quests don't work yet as expected."*
    ///
    /// `0x0089` sub-case 1 is the client's quest-record update. It is **not** a packet the
    /// client blocks on, so a lost one costs a stale journal rather than a frozen UI - and
    /// that is why a database failure here still sends the record. The player sees the quest
    /// they just accepted; the label says it will not survive a relog.
    ///
    /// **It must not travel with a `SetField`.** `FUN_142d59e20` returns immediately when
    /// `world+0x2358` - the character-data object - is null, and that field measures `0x00`
    /// on the *first* `SetField` of a session. This path only ever runs from a live
    /// conversation, which is long after that. `crates/net/src/quest.rs` carries the working.
    pub(super) fn accept_quest(&mut self, convo: &Conversation) -> Vec<Reply> {
        let Some(quest_id) = convo.quest_id else { return Vec::new() };
        self.record_quest_start(quest_id, convo.npc_template)
    }


    /// What an NPC should actually say when talked to.
    ///
    /// `String.wz/Npc.img` has the real lines - `d0`, `d1`, ... - and the server was sending
    /// placeholder text. Only `d0` is used: the second and later lines need the "next"
    /// button and a `0x00F3` answer to page through, and neither is built.
    ///
    /// **`#p8#` is sent unexpanded, on purpose.** It is a name substitution and whether the
    /// client resolves it is not established. Sending it raw makes the screen answer the
    /// question - "Hello! I'm Robin." and "Hello! I'm #p8#." are different on sight, and
    /// neither reading requires a guess. If the client does not expand it, the fix is here.
    ///
    /// An NPC with no entry keeps the placeholder, which is honest about the state of the
    /// server rather than silently saying nothing.
    ///
    /// **Read fresh from [`crate::config::NpcStringTable`] on every box, not cached.** That is
    /// what makes `!npcreload` reach a conversation this session is already having: the string
    /// is fetched at the instant `say_line` builds the packet, so an amendment is live for the
    /// next click with no restart and no reconnect.
    pub(super) fn npc_line(&self, template: u32) -> String {
        self.config.npc_strings.dialogue_line(template).unwrap_or_else(|| {
            format!("This server has no dialogue for NPC template {template} yet.")
        })
    }


    /// The character's quest journal, as the record wants it.
    ///
    /// **This was built and then not connected for a day**, which on screen is
    /// indistinguishable from not existing: `crates/net/src/quest.rs`, the `quest_state`
    /// table and `set_field_with_character_dressed_quests` were all written, tested and
    /// never called. The owner, after the run of 2026-08-19: *"Quests don't work yet as
    /// expected."* That is what this is fixing.
    ///
    /// A database error yields an **empty** book rather than dropping the `SetField`. The
    /// record has no length prefix and no resync point, so the only two safe answers are a
    /// correct book and no book at all; an empty one is the second, and it costs a blank
    /// quest journal instead of a frozen client. The reason travels in the reply's label.
    pub(super) fn quest_book(&self, character_id: u32) -> (net::quest::QuestBook, String) {
        // Block #28: citizenship (quest 510000) and the Community Board's postings - built
        // FIRST, because settling the board can give up in-progress board quests (one per
        // period, `crate::citizenship::settle`) and the book read below must not carry them.
        let ex = self.quest_ex_records(character_id);
        match self.store.quest_book(character_id) {
            Ok(mut book) => {
                book.ex = ex;
                // The account's options (session/options.rs): game options into block #28
                // beside the citizenship, system options into block #32.
                let (game, system) = self.option_records();
                book.ex.extend(game);
                if self.config.system_options {
                    book.shared_ex = system;
                }
                let note = format!(
                    ", quests: {} started / {} completed / ex records [{}]",
                    book.started.len(),
                    book.completed.len(),
                    book.ex.iter().map(|(q, v)| format!("quest {q} = {v}")).collect::<Vec<_>>().join("; ")
                );
                (book, note)
            }
            Err(e) => (
                net::quest::QuestBook::default(),
                format!(", quests: EMPTY BOOK - could not read quest_state ({e})"),
            ),
        }
    }
}
