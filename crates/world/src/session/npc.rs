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
        // Always answer. An unanswered request freezes the client's whole UI, so a body
        // that does not parse still gets a reply - parse_quest_request only returns None
        // when the fixed 9-byte head does not fit, and then there is no template to speak
        // as, which is the one case where silence is all there is.
        let Some(req) = net::script::parse_quest_request(body) else {
            return Vec::new();
        };
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
        if accepted {
            out.extend(self.record_quest_start(req.quest_id, req.npc_template_id));
        }
        if completing {
            out.extend(self.record_quest_complete(req.quest_id, speaking_quest));
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
            Ok(None) => out.push(Reply {
                opcode: net::notice::CHAT_NOTICE,
                body: net::notice::chat_notice(&format!(
                    "The server was asked to complete quest {finished}, which this character                      has not started."
                )),
                what: format!("quest {finished} completed but no row existed"),
            }),
            Err(e) => out.push(Reply {
                opcode: net::notice::CHAT_NOTICE,
                body: net::notice::chat_notice("Could not record that quest."),
                what: format!("quest {finished} completion NOT STORED: {e}"),
            }),
        }
        out.extend(self.apply_quest_completion_rewards(finished));
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
        if recorded {
            out.push(Reply {
                opcode: net::questeffect::USER_EFFECT_LOCAL,
                body: net::questeffect::quest_clear_local(),
                what: format!(
                    "UserEffectLocal QuestClear for quest {finished} - plays Sound/Game.img/QuestClear. The animation node Effect/BasicEff.img/QuestClear is NOT in this client's WZ, so sound and no picture is the EXPECTED result."
                ),
            });
        }
        if chained_to != finished {
            out.extend(self.record_quest_start(chained_to, 0));
        }
        out
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
    fn apply_quest_completion_rewards(&mut self, quest_id: u32) -> Vec<Reply> {
        let Some(quest) = self.config.quests.get(&quest_id) else { return Vec::new() };
        let items = quest.complete_items.clone();
        let exp = quest.complete_exp;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = Vec::new();

        for (item_id, count) in items {
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
                        out.push(Reply {
                            opcode: net::message::MESSAGE,
                            body: net::message::item_gained(item_id, count as u32),
                            what: format!("Message: quest {quest_id} rewarded {item_id} x{count}"),
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

        // Last, so the experience line lands under the item lines the way a turn-in reads.
        // White: a quest reward is yours, not a share of somebody else's kill.
        out.extend(self.award_experience(exp, &format!("quest {quest_id}"), true));
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
            let take = row.item.kind.quantity().min(left);
            self.store.remove_item(character_id, inv, row.slot, Some(take))?;
            left -= take;
            out.push(Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_removed(inv.as_u8() as i8, row.slot as i16),
                what: format!(
                    "InventoryOperation REMOVE: {take} x {item_id} from {inv:?} slot {} - taken back by a quest",
                    row.slot
                ),
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
    pub(super) fn record_quest_start(&mut self, quest_id: u32, npc_template: u32) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let what = match self.store.start_quest(chr.id, quest_id) {
            Ok(true) => format!(
                "quest {quest_id} accepted from NPC {npc_template} by character {} ({}) and stored",
                chr.id, chr.name
            ),
            Ok(false) => format!(
                "quest {quest_id} was already started for character {}; the record is re-sent so the journal agrees",
                chr.id
            ),
            Err(e) => format!(
                "quest {quest_id} accepted but NOT STORED ({e}) - the journal will show it until the next relog and then lose it"
            ),
        };
        let mut out =
            vec![Reply { opcode: net::quest::MESSAGE, body: net::quest::quest_accepted(quest_id), what }];
        out.extend(self.grant_quest_start_items(quest_id));
        out
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
                    out.push(Reply {
                        opcode: net::message::MESSAGE,
                        body: net::message::item_gained(item_id, count.max(1) as u32),
                        what: format!("Message: quest {quest_id} gave {item_id} x{count}"),
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
        if let Some(replies) = self.open_shop_for(template, chr.id) {
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
        let branches =
            last && !on_branch && convo.quest_id.is_some() && self.has_branch(&convo, "yes");
        let has_next = !last;

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
        let Some(reply) = net::script::parse_script_reply(body) else { return Vec::new() };
        let Some(convo) = self.conversation.clone() else { return Vec::new() };

        if reply.action == net::script::SCRIPT_ACTION_CLOSED {
            self.conversation = None;
            return Vec::new();
        }

        if convo.awaiting_yes_no {
            let accepted = reply.action == net::script::SCRIPT_ACTION_YES;
            let branch = if accepted { "yes" } else { "no" };
            // **Record the acceptance before the branch-text check**, not after. A quest
            // whose `yes` path has no line in `Quest.wz` is still a quest the player just
            // accepted, and ordering these the other way would silently drop exactly those.
            let mut out = if accepted { self.accept_quest(&convo) } else { Vec::new() };
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
    pub(super) fn npc_line(&self, template: u32) -> String {
        self.config
            .npc_strings
            .get(&template)
            .and_then(|s| s.dialogue.first())
            .cloned()
            .unwrap_or_else(|| {
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
        match self.store.quest_book(character_id) {
            Ok(book) => {
                let note = format!(
                    ", quests: {} started / {} completed",
                    book.started.len(),
                    book.completed.len()
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
