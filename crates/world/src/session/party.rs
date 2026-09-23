//! **Party requests, answered.**
//!
//! The owner, 2026-09-01: *"There are already party UI elements in the game."* There are - the
//! client ships the whole window, `UI_000.wz/UserList.img/Party` with all eight buttons and a
//! scrollable member list, `PartyHP.img` with the gauge, and 89 party strings. **[L]**
//! `research/party.md`.
//!
//! # Why this file exists before parties do
//!
//! Because `0x0182` was reaching this server and **going unanswered**. One archived capture
//! has it, from the owner pressing Create:
//!
//! ```text
//!   16:48:31.299 <- 0x0182 UNKNOWN, 68 byte body 1000000000000a000e00...
//!   16:48:31.299    0x0182 UNKNOWN is not answered yet
//! ```
//!
//! `CLAUDE.md`'s first expensive rule: *an unanswered packet freezes the client's entire UI -
//! every button, including the quit prompt - and reads on screen as a crash.* A refusal is an
//! answer; silence is not. So this file answers every party request, today, whether or not
//! there is a party system behind it.
//!
//! # What it answers with, and why that exact code
//!
//! `0x00A5` `PARTY_RESULT` carries a `u8 code` into a 45-entry jump table at `0x1413bcc50`.
//! **Sixteen of the twenty-nine non-default arms read nothing after the code**, and neither
//! does the default. **[L]** `net::party::refusal` will only build a code whose arm is one of
//! those, so this cannot produce a packet the client would read fields out of and run off the
//! end of.
//!
//! [`net::party::request_failed`] is the default arm - *"Due to an unknown error, your party
//! request failed."* Two bytes past the opcode, and the handler returns normally.
//!
//! # What is NOT here yet
//!
//! Membership. `crate::party::Parties` is written and tested and needs a home on the channel
//! that outlives a session; that holder is being added to `Fields` in a separate pass. Until
//! it lands, every request is refused **specifically** rather than ignored, and the request is
//! logged with what it actually asked for - so the first two-client run produces a record of
//! which buttons the window sends even though none of them works yet.

use crate::Reply;

/// What an action byte is called, for the log line only.
///
/// The seven names are the client's own: `FUN_1411c9540`, the party window's button
/// dispatcher, compares the UTF-16 literals `create`, `invite`, `expel`, `leave`, `pickup`
/// and `leader`. **[L]** `research/party.md`.
///
/// An unknown byte is printed as unknown rather than mapped to the nearest thing - six of the
/// seven actions have never been seen on this wire, so the first run to produce one must not
/// have it silently relabelled.
fn action_name(action: u8) -> &'static str {
    use net::party::action;
    match action {
        action::CREATE => "create",
        action::LEAVE => "leave",
        action::SET_PICKUP_RIGHTS => "pickup rights",
        action::INVITE => "invite",
        action::JOIN_REQUEST => "join request",
        action::EXPEL => "expel",
        action::CHANGE_LEADER => "change leader",
        _ => "UNKNOWN - not one of the seven the party window sends",
    }
}

impl super::Session {
    /// **Who counts as "us" for a drop**, from the channel's real party membership.
    ///
    /// The seam `crate::mobshare::Party::of` was written for. It was `Party::solo` for every
    /// caller until parties were wired, which is why every drop was private: solo is the
    /// correct answer for a character in no party and the wrong one for everybody else.
    ///
    /// The owner set the rule: *"All members of a party should see all drops killed by members of
    /// the party."* `crate::party::Audience` guarantees it always contains the character it
    /// was asked about, so a party that has somehow forgotten its own member still cannot
    /// hide that member's drops from them.
    pub(super) fn party_for(&self, character: u32) -> crate::mobshare::Party {
        let audience = self.fields.parties().audience(character);
        crate::mobshare::Party::of(character, audience.as_slice().iter().copied())
    }

    /// Apply one party request and answer it.
    ///
    /// **The answer is the transition's, not the request's.** `Parties::apply` returns either
    /// the effects or a `Refusal`, and every reply below hangs off which one came back - the
    /// quest-payout lesson in `CLAUDE.md`, where the store's refusal was captured into a log
    /// string and then ignored by the payout beside it.
    pub(super) fn run_party_request(&mut self, actor: u32, request: crate::party::Request) -> Vec<Reply> {
        let described = format!("{request:?}");
        // Wall-clock seconds, the one clock every connection shares - `Session::clock_ms`
        // restarts per connection and the invite table is shared across all of them. `apply`
        // ages invites itself; this call is here to LOG the lapses, which apply discards.
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        for (party, target) in self.fields.parties().expire_invites(now) {
            crate::server::log(&format!(
                "   party: invite to character {target} for party {party} LAPSED after \
                 {}s of no answer and was dropped - no packet is sent, the client faded its \
                 own dialog. crate::party::INVITE_TTL_SECS",
                crate::party::INVITE_TTL_SECS
            ));
        }
        // **Through the hub when there is one.** The hub echoes the request to every channel
        // in one order and each applies it to its replica; this session gets the outcome
        // back through `collect_party_outcomes` a tick later and answers then. Without a hub
        // (no `--link`, or the hub is down) the request is applied here, synchronously, as it
        // always was. `session/worldlink.rs`.
        if self.send_party_request_to_hub(actor, now, &request) {
            crate::server::log(&format!("   party: {described} by character {actor} sent to the hub; the answer follows its echo"));
            return Vec::new();
        }
        let outcome = self.fields.parties().apply(now, actor, request);
        self.party_outcome_replies(actor, &described, outcome)
    }

    /// **This connection is going away for good.** The character keeps their seat while
    /// another member is online; if they led, the crown goes to the highest-level member who
    /// is still online. The LAST member online to go - leader or not - ends the party: the owner,
    /// 2026-09-18, *"If everyone is offline, the party shouldn't exist?"*
    ///
    /// The owner, 2026-09-18: *"when a party leader disconnects from the game, the party leader
    /// needs to be handed over to the next highest level player automatically"*, then *"Only
    /// the party leader should be handed off. The disconnected client should remain in the
    /// party. The party should persist even if all members have disconnected. If the leader
    /// position cannot be handed off to an online player, then the entire party should be
    /// disbanded."* Until this, nothing told the registry about a disconnect at all.
    ///
    /// The successor is chosen HERE because the registry knows neither levels nor presence and
    /// the hub keeps no database: among the other members, those online on this channel
    /// (`Bus::character_online`) or on another (the hub directory, `link.everyone()`), the
    /// highest level from the store, ties to the earliest joined. `last_online` says no
    /// other member was found online at all, which the registry reads as "disband". It rides as
    /// [`crate::party::Request::Disconnect`] through `run_party_request`, so with a hub every
    /// channel applies the same request in the same order and answers from the echo, and
    /// without one it is applied here - exactly as a Leave would be.
    ///
    /// Called from `on_log_out` and, for a crash or a dropped socket, from `Drop`; the flag
    /// makes the second call a no-op. A channel change never calls it (`handing_over`). The
    /// replies to the departing character are discarded - there is nobody to send them to;
    /// every other member is reached through `deliver`.
    pub(super) fn leave_party_on_disconnect(&mut self) {
        if self.party_told_of_disconnect {
            return;
        }
        let Some(me) = self.claimed_character().map(|c| c.id) else { return };
        let Some(party) = self.fields.parties().party_of(me).cloned() else { return };
        self.party_told_of_disconnect = true;
        // Who else is in the game: this channel's bus, or the hub's directory for every channel.
        let hub_online: std::collections::HashSet<u32> = crate::link::current()
            .map(|l| l.everyone().into_iter().map(|(id, _)| id).collect())
            .unwrap_or_default();
        let others_online: Vec<u32> = party
            .members
            .iter()
            .copied()
            .filter(|&c| c != me && (self.bus().character_online(c) || hub_online.contains(&c)))
            .collect();
        let last_online = others_online.is_empty();
        let leads = party.leader == me;
        // The heir, when one is needed: the highest level among those online; the first
        // maximum in join order wins a tie.
        let mut best: Option<(u32, u32)> = None; // (level, id)
        if leads {
            for &id in &others_online {
                let level = self.store.character_brief(id).ok().flatten().map(|b| b.level).unwrap_or(0);
                if best.map_or(true, |(l, _)| level > l) {
                    best = Some((level, id));
                }
            }
        }
        let successor = best.map(|(_, id)| id);
        crate::server::log(&format!(
            "   party: character {me}{} is leaving the game - {}",
            if leads { format!(", leader of party {}", party.id) } else { format!(" of party {}", party.id) },
            match (last_online, successor) {
                (true, _) => "no other member is online, so the party is disbanded".to_string(),
                (false, Some(s)) => format!("leadership goes to {s}, its highest-level member online; {me} keeps a seat"),
                (false, None) => format!("{} still online, so {me} keeps a seat and the leader stays {}", others_online.len(), party.leader),
            }
        ));
        let _ = self.run_party_request(me, crate::party::Request::Disconnect { successor, last_online });
    }

    /// **A member who logs back in gets their party window.** A seat persists across a
    /// disconnect now, so the window has to be rebuilt from the registry at the login field
    /// entry - the same `0x0D` push a rights change or a leader change uses. Nothing when
    /// the character is in no party.
    pub(super) fn party_window_on_login(&self) -> Vec<Reply> {
        let Some(me) = self.claimed_character().map(|c| c.id) else { return Vec::new() };
        let Some(party) = self.fields.parties().party_id_of(me) else { return Vec::new() };
        let Some(block) = self.party_block(party) else { return Vec::new() };
        vec![Reply {
            opcode: net::party::PARTY_RESULT,
            body: net::party::party_state(Some(&block)),
            what: format!(
                "PartyResult PARTY_STATE (0x0D): character {me} logged in still a member of party {party} - the window rebuilt from the registry"
            ),
        }]
    }

    /// **The party window after a level-up.** The owner, 2026-09-22: *"When a party member levels
    /// up, the level up does not reflect in the party list."*
    ///
    /// `party_block` has always read the levels correctly - it takes this connection's own
    /// live character for its own seat and the store's row for everyone else - so the row
    /// was never wrong when it was BUILT. It was simply never rebuilt: the `0x0D` push went
    /// out on joins, leaves, leader and rights changes, and a level is none of those, so
    /// every other client kept drawing the number it was handed when the member joined.
    ///
    /// Sent to the **whole party including the leveller**, because the `0x007C` that carries
    /// the new level updates the character's own stats and not their row in the party
    /// window. Built once here rather than per recipient: the block describes the party, not
    /// the reader, and this connection is the only one that can see its own new level
    /// without re-reading the store.
    pub(super) fn party_window_after_level_up(&mut self) -> Vec<Reply> {
        let Some(me) = self.claimed_character().map(|c| c.id) else { return Vec::new() };
        let Some(party) = self.fields.parties().party_id_of(me) else { return Vec::new() };
        let Some(block) = self.party_block(party) else { return Vec::new() };
        let body = net::party::party_state(Some(&block));
        let what = |who: String| {
            format!("PartyResult PARTY_STATE (0x0D) to {who}: character {me} levelled, so party {party}'s rows are re-sent")
        };
        let mut told = 0;
        for member in self.recipients_for(party, None) {
            if member == me {
                continue;
            }
            if self.deliver_anywhere(member, Reply {
                opcode: net::party::PARTY_RESULT,
                body: body.clone(),
                what: what(format!("character {member}")),
            }) {
                told += 1;
            }
        }
        if told > 0 {
            crate::server::log(&format!("   party: character {me} levelled - party {party}'s window re-sent to {told} other member(s)"));
        }
        vec![Reply { opcode: net::party::PARTY_RESULT, body, what: what("themselves".to_string()) }]
    }

    /// The packets for one applied request - the refusal, or the effects.
    pub(super) fn party_outcome_replies(
        &mut self,
        actor: u32,
        described: &str,
        outcome: Result<Vec<crate::party::Effect>, crate::party::Refusal>,
    ) -> Vec<Reply> {
        match outcome {
            Err(refusal) => {
                // **A specific refusal, not the blanket one.** `Refusal::result_code` is a
                // code the client has a real message for - "you are already in a party",
                // "the party is full" - and `net::party::refusal` REFUSES to build a code
                // whose arm reads fields, so this cannot become the crash it is guarding
                // against. If it ever returns `None`, fall back rather than send nothing:
                // silence freezes the window.
                let code = refusal.result_code();
                let body = net::party::refusal(code).unwrap_or_else(net::party::request_failed);
                vec![Reply {
                    opcode: net::party::PARTY_RESULT,
                    body,
                    what: format!(
                        "PartyResult code {code:#04x} to character {actor}: {described} refused \
                         - {}. The refusal is the whole answer; nothing changed",
                        refusal.message()
                    ),
                }]
            }
            Ok(effects) => self.party_effects(actor, described, effects),
        }
    }

    /// Turn the state machine's effects into packets.
    ///
    /// **Only the effects whose body is decoded produce one.** Everything else is logged in
    /// the words "the client was NOT told" rather than reported as success, because a
    /// `0x00A5` whose arm reads a field we did not send is the exact shape that killed two
    /// clients three times this week.
    fn party_effects(
        &mut self,
        actor: u32,
        described: &str,
        effects: Vec<crate::party::Effect>,
    ) -> Vec<Reply> {
        use crate::party::Effect;

        let mut out = Vec::new();
        let mut undecoded = Vec::new();
        for effect in &effects {
            crate::server::log(&format!("   party: {effect:?}"));
            match effect {
                Effect::Created { party, leader } => {
                    let Some(chr) = self.claimed_character().filter(|c| c.id == *leader) else {
                        undecoded.push(format!("{effect:?} (the leader is not this session)"));
                        continue;
                    };
                    let name = self
                        .fields
                        .parties()
                        .party(*party)
                        .map(|p| p.name.clone())
                        .unwrap_or_default();
                    out.push(Reply {
                        opcode: net::party::PARTY_RESULT,
                        body: net::party::party_created(*party, &name, &self.party_member(&chr)),
                        what: format!(
                            "PartyResult CREATED: party {party} exists and character {leader}                              leads it. research/party-result-0x00A5.md"
                        ),
                    });
                }
                // **The invite, both halves.** `research/party-result-0x00A5.md` §5.4 and §5.7.
                Effect::Invited { party, from, target } => {
                    let target_name = self.name_of(*target);
                    // To the inviter: 0x1B outcome 0, "You have invited '%s' to your party." [L].
                    // Until 2026-09-05 this was UNKNOWN_ERROR, which is the sentence the owner saw.
                    let outcome = Reply {
                        opcode: net::party::PARTY_RESULT,
                        body: net::party::invite_outcome(
                            net::party::invite_outcome::INVITED,
                            &target_name,
                        ),
                        what: format!(
                            "PartyResult INVITE OUTCOME 0 to character {from}: \"You have invited \
                             '{target_name}' to your party.\" [L]"
                        ),
                    };
                    self.deliver(*from, actor, outcome, &mut out);
                    // To the target: 0x03, six unconditional fields. The SHAPE is [L]; which
                    // value the dialog draws where, beyond field 2, is what the next client run
                    // measures - see `net::party::invite_notify`.
                    let (inviter_name, level, job) = match self.claimed_character() {
                        Some(c) if c.id == *from => (c.name.clone(), u32::from(c.level), u32::from(c.job)),
                        _ => (self.name_of(*from), 0, 0),
                    };
                    let notify = Reply {
                        opcode: net::party::PARTY_RESULT,
                        body: net::party::invite_notify(*from, *party, &inviter_name, level, job),
                        what: format!(
                            "PartyResult INVITE NOTIFY (0x03) to character {target}: inviter {from} \
                             {inviter_name:?}, party {party} as field 2 (echoed back in 0x0183), \
                             level {level} and job {job} as fields 4-5 [I]. Opens the invite dialog"
                        ),
                    };
                    if !self.deliver_anywhere(*target, notify) {
                        crate::server::log(&format!(
                            "   party: character {target} is not online on any channel this process can \
                             reach, so the invite dialog was NOT delivered. The invite is recorded"
                        ));
                    }
                }
                // The invitee declined (or the invite was overtaken). The leader is told
                // "%s has denied the party request." [L]; the invitee, if this is their own
                // decline, gets code 0x17 - an arm inside the epilogue that shows nothing [L] -
                // because an unanswered 0x0183 is a packet nobody has measured the client
                // surviving, and UNKNOWN_ERROR would print a lie.
                Effect::InviteDropped { party, target, reason } => {
                    let target_name = self.name_of(*target);
                    if let Some(leader) = self.fields.parties().party(*party).map(|p| p.leader) {
                        // The invitee's client named the outcome; the leader is shown that
                        // sentence and no other. `research/party-result-0x00A5.md` §5.7.
                        use crate::party::DeclineReason;
                        use net::party::invite_outcome as oc;
                        let (code, sentence) = match reason {
                            DeclineReason::Refused => (oc::DENIED, "has denied the party request"),
                            DeclineReason::Blocking => {
                                (oc::BLOCKING, "is currently blocking any party invitations")
                            }
                            DeclineReason::Busy => (oc::BUSY, "is taking care of another invitation"),
                            DeclineReason::AlreadyInvited => {
                                (oc::ALREADY_INVITED, "- you have already invited them")
                            }
                        };
                        let denied = Reply {
                            opcode: net::party::PARTY_RESULT,
                            body: net::party::invite_outcome(code, &target_name),
                            what: format!(
                                "PartyResult INVITE OUTCOME {code} to character {leader}: \
                                 \"{target_name} {sentence}.\" [L]"
                            ),
                        };
                        self.deliver(leader, actor, denied, &mut out);
                    }
                    if actor == *target {
                        if let Some(body) = net::party::refusal(net::party::result::SILENT_17) {
                            out.push(Reply {
                                opcode: net::party::PARTY_RESULT,
                                body,
                                what: format!(
                                    "PartyResult 0x17 to character {actor}: the decline is acknowledged \
                                     with an arm that draws nothing [L]"
                                ),
                            });
                        }
                    }
                }
                // Everyone in the party, the joiner included, gets 0x13: the joiner's name and
                // then the WHOLE PARTYBLOCK - "'%s' has joined the party." / "You have joined
                // the party." and the window's six seats in one packet. [L] shape. On
                // 2026-09-05 this went out as the name alone and both clients threw, reported
                // the packet in 0x009E and closed their sockets - see `net::party::PartyBlock`.
                Effect::Joined { party, who } => {
                    let who_name = self.name_of(*who);
                    let Some(block) = self.party_block(*party) else {
                        undecoded.push(format!("{effect:?} (party {party} is not in the table)"));
                        continue;
                    };
                    let members: Vec<u32> =
                        block.seats.iter().map(|m| m.char_id).filter(|&id| id != 0).collect();
                    let occupied = members.len();
                    for member in members {
                        let reply = Reply {
                            opcode: net::party::PARTY_RESULT,
                            body: net::party::joined(&who_name, &block),
                            what: format!(
                                "PartyResult JOIN (0x13) to character {member}: {who_name:?} joined \
                                 party {party}; the six-seat PARTYBLOCK follows the name, \
                                 {occupied} occupied [L shape]"
                            ),
                        };
                        self.deliver(member, actor, reply, &mut out);
                    }
                }
                // A member left or was expelled, and the party lives on. `0x10` with
                // `still_exists` true, carrying the party AFTER the departure so every window
                // redraws with the right seats. The leaver sees the first-person wording off
                // their own id, the rest the third-person - the client picks from `char_id`.
                Effect::Departed { party, who, how, remaining } => {
                    let who_name = self.name_of(*who);
                    let expelled = matches!(how, crate::party::Departure::Expelled);
                    // The party still exists (Disbanded is a separate effect), so the block is
                    // built from its current membership. `remaining` is that list.
                    let block = self.party_block(*party);
                    let _ = remaining;
                    for member in self.recipients_for(*party, Some(*who)) {
                        let body = match &block {
                            Some(b) => net::party::member_left(*who, expelled, &who_name, Some(b)),
                            // The party is gone from the table already (a one-member remnant
                            // that dissolved); tell them it no longer exists rather than
                            // nothing. Should not happen while Departed is distinct from
                            // Disbanded, but a missing block must not drop the packet.
                            None => net::party::member_left(*who, expelled, &who_name, None),
                        };
                        let reply = Reply {
                            opcode: net::party::PARTY_RESULT,
                            body,
                            what: format!(
                                "PartyResult WITHDRAW (0x10) to character {member}: {who_name:?} \
                                 left party {party} ({how:?}) [L]"
                            ),
                        };
                        self.deliver(member, actor, reply, &mut out);
                    }
                    // **And out of the party quest.** A run belongs to a party; somebody who
                    // is no longer in it is no longer in the run. session/firsttime.rs.
                    let ejected = self.eject_from_party_quest(*who, "they left the party");
                    out.extend(ejected);
                }
                // The leader left, so the party is gone. `0x10` with `still_exists` false and
                // `char_id` = the leader who quit: the leader reads "you disbanded", everyone
                // else "left since the leader quit", off that one id. `members` is everyone
                // who was in it at the end, this connection included.
                Effect::Disbanded { party, members } => {
                    let leader = actor; // the disband is driven by the leader's own Leave
                    // The party is gone, so nobody is in its run any more.
                    for member in members.clone() {
                        let ejected = self.eject_from_party_quest(member, "the party disbanded");
                        out.extend(ejected);
                    }
                    for member in members {
                        let reply = Reply {
                            opcode: net::party::PARTY_RESULT,
                            body: net::party::member_left(leader, false, "", None),
                            what: format!(
                                "PartyResult WITHDRAW (0x10, disband) to character {member}: \
                                 party {party} disbanded by leader {leader} [L]"
                            ),
                        };
                        self.deliver(*member, actor, reply, &mut out);
                    }
                }
                // The leadership moved. Push the window first so both ids are seats the client
                // already holds, then narrate the change. `0x22` is silently dropped unless the
                // new leader is already a seat (`research/party-result-0x00A5.md` §5.5).
                Effect::LeaderChanged { party, from, to } => {
                    if let Some(block) = self.party_block(*party) {
                        for member in self.recipients_for(*party, None) {
                            let refresh = Reply {
                                opcode: net::party::PARTY_RESULT,
                                body: net::party::party_state(Some(&block)),
                                what: format!(
                                    "PartyResult PARTY_STATE (0x0D) to character {member}: window \
                                     refreshed before the leader change in party {party}"
                                ),
                            };
                            self.deliver(member, actor, refresh, &mut out);
                        }
                    }
                    crate::server::log(&format!(
                        "   party: leadership of party {party} moved from {from} to {to}; 0x22 \
                         narration is not built, the window refresh above carries the new leader"
                    ));
                }
                // The pick-up-rights mode toggled. `0x2D` IS the client's rights-changed
                // packet (`net::party::party_status`): every member's client stores the byte,
                // says "The party's item pick-up rights changed to Party Leader / All" and
                // relabels the window. Until 2026-09-14 this pushed a bare 0x0D, whose block
                // carried a constant 0 in that byte - which is "nothing happened".
                Effect::PickupRightsChanged { party, rights } => {
                    if let Some(block) = self.party_block(*party) {
                        let leader_only = *rights == crate::party::PICKUP_LEADER_ONLY;
                        for member in self.recipients_for(*party, None) {
                            let reply = Reply {
                                opcode: net::party::PARTY_RESULT,
                                body: net::party::party_status(&block.name, block.is_public, leader_only),
                                what: format!(
                                    "PartyResult 0x2D to character {member}: party {party} pick-up rights -> {} (byte {rights}); the client shows 'The party''s item pick-up rights changed to ...' and relabels the window",
                                    if leader_only { "Party Leader" } else { "All" }
                                ),
                            };
                            self.deliver(member, actor, reply, &mut out);
                        }
                    }
                }
            }
        }

        if !undecoded.is_empty() {
            // **Said out loud, not implied.** These transitions are real - drops and
            // experience follow them from now - and the client is not being told, because
            // their bodies are not decoded yet.
            crate::server::log(&format!(
                "   party: {described} by character {actor} changed state and these effects                  were NOT sent, their 0x00A5 bodies being undecoded: {undecoded:?}"
            ));
        }
        if out.is_empty() {
            // An unanswered request freezes the client's whole UI, so something goes back
            // even when nothing we can build applies.
            out.push(Reply {
                opcode: net::party::PARTY_RESULT,
                body: net::party::request_failed(),
                what: format!(
                    "PartyResult UNKNOWN_ERROR to character {actor} after a SUCCESSFUL                      {described}: the state changed and no decoded packet describes it"
                ),
            });
        }
        out
    }

    /// A packet for `to`: onto this connection's own reply list when `to` is the actor,
    /// otherwise through the bus to whichever connection on this channel plays `to`. A
    /// recipient who is not on this channel is logged, not invented.
    fn deliver(&self, to: u32, actor: u32, reply: Reply, out: &mut Vec<Reply>) {
        if to == actor {
            out.push(reply);
        } else if !self.deliver_anywhere(to, reply) {
            crate::server::log(&format!(
                "   party: character {to} is not online on any channel this process can reach and was NOT told"
            ));
        }
    }

    /// A character's name for a packet, by id; `#id` when the store has no such row.
    fn name_of(&self, id: u32) -> String {
        self.store
            .character_name(id)
            .ok()
            .flatten()
            .unwrap_or_else(|| format!("#{id}"))
    }

    /// This character as a party seat.
    fn party_member(&self, chr: &net::opcode::Character) -> net::party::Member {
        net::party::Member {
            char_id: chr.id,
            name: chr.name.clone(),
            job: u32::from(chr.job),
            level: u32::from(chr.level),
            unknown_b: 0,
            unknown_d: 0,
        }
    }

    /// **Broadcast this character's HP to the party members standing on its field.**
    ///
    /// The owner, 2026-09-05: *"Party member HP should've been broadcasted to party members on the
    /// same map when the party is formed, the picture shows that the HP bar is completely blank
    /// for members that are not the current client."* The packet is `0x02B2`
    /// (`net::userpool::user_hp_remote`, chain in its doc): the receiving client turns
    /// `(hp, max)` into the percent its party HUD gauge and over-head bar read.
    ///
    /// Called every tick. It sends only when `(hp, max_hp, who is here)` differs from the last
    /// send, so one comparison covers all three triggers the owner's sentence implies: the party
    /// forming (recipients go from none to some), a member arriving on this map (recipients
    /// grow), and HP moving (regen, damage, potions, level-up). A member who leaves the map or
    /// the party simply drops out of the recipient list; when they come back the list differs
    /// again and they are sent the current value. Latency is one tick, about 100 ms.
    ///
    /// The receiver must hold this character in its user pool - that is what
    /// `Bus::publish_to_character`'s map match guarantees, and the spawn was posted to the
    /// same mailbox earlier, so it is read first. A charId the pool does not hold is a clean
    /// no-op at the client's router, so an ordering slip costs a bar, never a crash.
    pub(super) fn party_hp_tick(&mut self) {
        let Some(chr) = self.claimed_character() else {
            self.last_party_hp = None;
            return;
        };
        let members = match self.fields.parties().party_of(chr.id) {
            Some(p) if p.members.len() > 1 => p.members.clone(),
            _ => {
                self.last_party_hp = None;
                return;
            }
        };
        // **Not until this character's own spawn has reached the field.** Measured on a
        // channel change back into ch0, `world-ch0.log` 2026-09-19:
        //
        // ```text
        //   01:36:38.043  <- the owner     MIGRATION HELLO
        //   01:36:38.045  -> the owner     SetField
        //   01:36:38.164  -> Tester2  0x02B2  the owner 229/247        <- HP, 121 ms after the hello
        //   01:36:38.320  <- the owner     0x00DC CLIENT_FIELD_ENTERED
        //   01:36:38.384  -> Tester2  0x0224 UserEnterField: the owner <- Tester2 first hears of them
        //   01:36:53.025  -> Tester2  0x02B2  the owner 239/247        <- regen, 15 s later
        // ```
        //
        // The HP arrived **220 ms before** the spawn that creates the `CUser` it describes,
        // so the client dropped it - and then `last_party_hp` below cached the value and
        // suppressed every resend until regen happened to change it fifteen seconds later.
        // The owner: *"the owner's HP bar was unavailable on Tester2's screen until much later."*
        //
        // A session is claimed and carrying a map from `SetField`, but it has no presence on
        // the bus until `announce_field_entry` publishes its spawn on `0x00DC` - so presence
        // for *this* map is exactly the question "has my spawn gone out here yet", and it
        // needs no new state. Checked against `chr.map_id` rather than `is_some()` because a
        // map change leaves the previous field's presence in place until the new one lands.
        if self.bus().map_of(self.subscriber) != Some(chr.map_id) {
            self.last_party_hp = None;
            return;
        }
        let others: Vec<u32> = members.into_iter().filter(|&m| m != chr.id).collect();
        let here = self.bus().characters_on(chr.map_id, &others);
        if here.is_empty() {
            self.last_party_hp = None;
            return;
        }
        let hp = u32::try_from(chr.hp).unwrap_or(0);
        // The maximum the OTHER client should draw the bar against is the one this client
        // draws for itself - base plus Max HP Increase - or a 447/447 member shows as
        // over-full on a partner's screen. `Session::pools`.
        let max_hp = self.pools(&chr).max_hp;
        let now = (hp, max_hp, here);
        if self.last_party_hp.as_ref() == Some(&now) {
            return;
        }
        let mut every_one_delivered = true;
        for member in &now.2 {
            let reply = Reply {
                opcode: net::userpool::USER_HP_REMOTE,
                body: net::userpool::user_hp_remote(chr.id, hp, max_hp),
                what: format!(
                    "UserHP (0x02B2) to character {member}: {} has {hp}/{max_hp} - the party \
                     HUD gauge and over-head bar for this character on their screen [L chain, \
                     unseen on a screen]",
                    chr.id
                ),
            };
            // A member who left the map between the presence read and now is a `false`
            // here. That used to be discarded and the value cached anyway, so a delivery
            // that failed was never retried while hp, max and the recipient list all stayed
            // the same - which for a player standing still at full HP is forever.
            every_one_delivered &= self.bus().publish_to_character(*member, chr.map_id, reply);
        }
        // Cache only what actually went out, so a failed send is retried next tick rather
        // than remembered as sent.
        self.last_party_hp = every_one_delivered.then_some(now);
    }

    /// Who should be told about a change to `party`: its current members, plus `also` when it
    /// is a character no longer in the party who still needs the packet - the one who just
    /// left. Deduplicated, so passing a still-present member changes nothing.
    fn recipients_for(&self, party: crate::party::PartyId, also: Option<u32>) -> Vec<u32> {
        let mut who: Vec<u32> = self
            .fields
            .parties()
            .party(party)
            .map(|p| p.members.clone())
            .unwrap_or_default();
        if let Some(extra) = also {
            if !who.contains(&extra) {
                who.push(extra);
            }
        }
        who
    }

    /// The whole party as the client's `PARTYBLOCK` wants it: six seats, occupied ones
    /// first, each with a name, job and level. `None` when the party is not in the table.
    fn party_block(&self, party: crate::party::PartyId) -> Option<net::party::PartyBlock> {
        let (name, leader, members) = {
            let parties = self.fields.parties();
            let p = parties.party(party)?;
            (p.name.clone(), p.leader, p.members.clone())
        };
        let leader_only_pickup = self
            .fields
            .parties()
            .party(party)
            .is_some_and(|p| p.pickup_rights == crate::party::PICKUP_LEADER_ONLY);
        let mut block = net::party::PartyBlock {
            party_id: party,
            leader_char_id: leader,
            name,
            leader_only_pickup,
            ..Default::default()
        };
        if members.len() > net::party::PARTY_SEATS {
            crate::server::log(&format!(
                "   party: party {party} has {} members and the client draws {} seats - the \
                 rest are NOT sent",
                members.len(),
                net::party::PARTY_SEATS
            ));
        }
        for (seat, id) in members.iter().take(net::party::PARTY_SEATS).enumerate() {
            block.seats[seat] = self.seat_for(*id);
        }
        Some(block)
    }

    /// One member as a seat: this connection's own character when it is them (the live
    /// level), else the store's row, else the id with a placeholder name - never an empty
    /// seat, because an empty seat is four bytes and shifts every field after it.
    fn seat_for(&self, id: u32) -> net::party::Member {
        if let Some(chr) = self.claimed_character().filter(|c| c.id == id) {
            return self.party_member(&chr);
        }
        match self.store.character_brief(id) {
            Ok(Some(brief)) => net::party::Member {
                char_id: id,
                name: brief.name,
                job: brief.job,
                level: brief.level,
                unknown_b: 0,
                unknown_d: 0,
            },
            _ => net::party::Member {
                char_id: id,
                name: format!("#{id}"),
                ..Default::default()
            },
        }
    }


    /// Answer `0x0182` `CLIENT_PARTY_REQUEST` and `0x0183` `CLIENT_PARTY_INVITE_ANSWER`.
    ///
    /// **Always answers.** Every path here returns exactly one `0x00A5`, including the one
    /// where the body does not parse - a body this server cannot read is still a request the
    /// client is blocking on, and `net::party::parse_request` returning `None` says nothing
    /// about whether the window is waiting.
    pub(super) fn on_party_request(&mut self, opcode: u16, body: &[u8]) -> Vec<Reply> {
        let who = self
            .claimed_character()
            .map(|c| format!("character {} ({})", c.id, c.name))
            .unwrap_or_else(|| "an unclaimed connection".to_string());

        // **What the request asks for, as a `crate::party::Request` where one can be built.**
        //
        // Only two of the seven actions can be built from what `parse_request` recovers.
        // `invite`, `expel` and `change leader` all name a TARGET, and this decoder reads one
        // string at slot 0 of the payload table - which is the party name in the create it was
        // written against, and something else in the others. `0x0183`'s invite answer carries
        // a party id nobody has read.
        //
        // That is a limit of the decoder, not of the party system: `!party` drives every one
        // of them from chat, and the bytes of anything refused here are logged so the next
        // capture can decode the shape rather than guessing at it.
        let request = if opcode == net::party::CLIENT_PARTY_REQUEST {
            match net::party::parse_request(body) {
                Some(req) => match req.action {
                    net::party::action::CREATE => {
                        // The client always supplies a name; `Parties::create` refuses an
                        // empty one, so a shape that lost it is refused rather than defaulted.
                        Ok(crate::party::Request::Create {
                            name: req.name.clone().unwrap_or_default(),
                        })
                    }
                    net::party::action::LEAVE => Ok(crate::party::Request::Leave),
                    // **Pick-up rights.** The client sends the mode in slot 1 of a tag-5
                    // payload (default 1). This server stores it and echoes the party window
                    // so the leader stops seeing "unknown error"; it does not gate who may
                    // pick up within a party - drop visibility is by membership. Absent slot
                    // means the default, exactly as the create does.
                    net::party::action::SET_PICKUP_RIGHTS => {
                        Ok(crate::party::Request::SetPickupRights { rights: req.pickup.unwrap_or(1) })
                    }
                    // **Invite carries a NAME and the server resolves it.** The client does
                    // not: `FUN_1413b9eb0` takes a `char*`, `strlen`s it, and the union emits
                    // a FlatBuffers string. **[L]**
                    net::party::action::INVITE => match req.name.as_deref() {
                        Some(name) if !name.is_empty() => {
                            match self.store.character_id_by_name(name) {
                                Ok(Some(target)) => Ok(crate::party::Request::Invite { target }),
                                _ => Err(format!("no character named {name:?}")),
                            }
                        }
                        _ => Err("an invite with no name".to_string()),
                    },
                    // **Expel and change-leader carry an ID, already resolved.** Both wrappers
                    // look the name up against the client's OWN member list
                    // (`FUN_1413b8ec0` -> `FUN_1406f1ff0`) and send the `u32`. Re-resolving a
                    // name here would be wrong twice: the client never sent one, and it has
                    // already proved the target is a member. **[L]**
                    a if a == net::party::action::EXPEL
                        || a == net::party::action::CHANGE_LEADER =>
                    {
                        match req.target_id {
                            // Id 0 is never a valid character here - a party slot is occupied
                            // exactly when its id is non-zero. **[L]**
                            Some(target) if target != 0 => {
                                Ok(if a == net::party::action::EXPEL {
                                    crate::party::Request::Expel { target }
                                } else {
                                    crate::party::Request::ChangeLeader { target }
                                })
                            }
                            _ => Err(format!(
                                "action {a} ({}) carried no usable character id: {:?}",
                                action_name(a),
                                req.target_id
                            )),
                        }
                    }
                    other => Err(format!(
                        "action {other} ({}) is not routed. payload tag {}, name {:?}, id \
                         {:?}, {} bytes: {body:02x?}",
                        action_name(other),
                        req.payload_tag,
                        req.name,
                        req.target_id,
                        body.len()
                    )),
                },
                None => Err(format!(
                    "a {}-byte body this decoder cannot read: {body:02x?}. Not a failure to \
                     hide - parse_request reads one payload shape, so this is most likely a \
                     shape nobody has decoded yet, and these bytes are what the next pass needs",
                    body.len()
                )),
            }
        } else {
            // `0x0183`: `{u8 op, u8 answer, u64 value}`, value being the party id this server
            // sent as field 2 of the 0x03. **The answer byte is the outcome, in the 0x1B
            // numbering** - `net::party::invite_answer`, read off the client on 2026-09-05:
            //
            //   0  the dialog is opening. Sent by the 0x03 handler ITSELF, before any click.
            //   1  blocking invitations, 2 busy with another, 3 already holds this invite -
            //      sent by the handler instead of a dialog.
            //   4  the Decline button.  5  the Accept button.
            //
            // The first 0x0183 this server ever decoded was a 0, one millisecond after the
            // 0x03, and "anything but 1 is an accept" turned it into a join nobody had
            // clicked. Zero changes nothing now, and nothing is sent back for it: the client
            // is not waiting, and the leader was told "You have invited" when the invite went
            // out.
            match net::party::parse_invite_answer(body) {
                Some(answer) => {
                    use crate::party::{DeclineReason, Request};
                    use net::party::invite_answer as ia;
                    crate::server::log(&format!(
                        "   party: 0x0183 invite answer op={:#04x} answer={} ({}) value={} raw={body:02x?}",
                        answer.op,
                        answer.answer,
                        ia::describe(answer.answer),
                        answer.value
                    ));
                    let party = u32::try_from(answer.value).unwrap_or(0);
                    match answer.answer {
                        ia::RECEIVED => return Vec::new(),
                        ia::ACCEPTED => Ok(Request::Accept { party }),
                        ia::DECLINED => Ok(Request::Decline { party, reason: DeclineReason::Refused }),
                        ia::BLOCKING => Ok(Request::Decline { party, reason: DeclineReason::Blocking }),
                        ia::BUSY => Ok(Request::Decline { party, reason: DeclineReason::Busy }),
                        ia::ALREADY_INVITED => {
                            Ok(Request::Decline { party, reason: DeclineReason::AlreadyInvited })
                        }
                        other => Err(format!(
                            "invite answer {other} is not one of the six values the client's code \
                             emits (0..=5) - body {body:02x?}"
                        )),
                    }
                }
                None => Err(format!(
                    "invite answer, {}-byte body {body:02x?} did not parse as a FlatBuffers \
                     table - these bytes are what the next decode pass needs",
                    body.len()
                )),
            }
        };

        let Some(actor) = self.claimed_character().map(|c| c.id) else {
            return vec![Reply {
                opcode: net::party::PARTY_RESULT,
                body: net::party::request_failed(),
                what: format!("PartyResult UNKNOWN_ERROR to {who}: no character is claimed"),
            }];
        };

        match request {
            Ok(request) => self.run_party_request(actor, request),
            Err(why) => {
                // **Still always answered.** A request this server cannot read is still a
                // client blocked on a reply, and an unanswered one freezes the whole UI -
                // every button, including the quit prompt.
                crate::server::log(&format!("   party: NOT ROUTED for {who}: {why}"));
                vec![Reply {
                    opcode: net::party::PARTY_RESULT,
                    body: net::party::request_failed(),
                    what: format!("PartyResult UNKNOWN_ERROR to {who}: {why}"),
                }]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use store::Store;

    use crate::config::Config;
    use crate::fields::Fields;
    use crate::session::Session;

    /// A claimed session, built the way `session::multiplayer`'s tests build theirs.
    fn session() -> Session {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Leader".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let config = Arc::new(Config::default());
        let mut s = Session::joining(store, config, Arc::new(Fields::new()));
        s.claim_for_character(id);
        s
    }

    fn hex_body(h: &str) -> Vec<u8> {
        let h: String = h.chars().filter(|c| !c.is_whitespace()).collect();
        (0..h.len() / 2).map(|i| u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).unwrap()).collect()
    }

    /// **Every party request is answered, including one this server cannot decode.**
    ///
    /// The second half is the point. A handler that answers only what it understands leaves
    /// the client blocked on everything else, and six of the seven party actions have never
    /// been seen on this wire.
    #[test]
    fn a_party_request_is_always_answered() {
        let mut s = session();

        // The real captured create, from `previous-runs` - action absent (0 = create),
        // payload tag 5, the string "TestCharD's Party".
        let captured = hex_body(
            "1000000000000a000e000000070008000a000000000000050c00000000000600080004000600\
             000004000000110000005465737443686172442773205061727479000000",
        );
        let out = s.on_party_request(net::party::CLIENT_PARTY_REQUEST, &captured);
        assert_eq!(out.len(), 1, "exactly one answer");
        assert_eq!(out[0].opcode, net::party::PARTY_RESULT);
        // **The captured create now CREATES.** This used to assert the log said "create";
        // asserting on the state is the difference between a handler that reads the packet
        // and one that acts on it.
        assert!(
            s.fields.parties().party_of(200).is_some(),
            "the archived create packet must produce a real party"
        );
        assert!(s.fields.parties().is_leader(200), "and its sender leads it");

        // **And the client is TOLD.** `0x0E` CREATED, carrying the party id, the leader's
        // own seat and the name they typed - not the placeholder refusal this used to send.
        // **Asserted on `packet()`, which is what reaches the framer**, not on `body`.
        // The version of this that read `body[0..2] == PARTY_RESULT` passed while the wire
        // carried `a5 00 | a5 00 0e ...` - the opcode twice, because the builder wrote it and
        // `Reply::packet()` wrote it again. The client read `0xA5` as the result code, fell
        // through to the default arm, and printed "your party request failed".
        //
        // It hid because `UNKNOWN_ERROR` is ITSELF a default-arm code, so every refusal test
        // passed on a doubled packet that produced the message it intended. Only a real arm
        // could expose it.
        let wire = out[0].packet();
        assert_eq!(wire[0..2], net::party::PARTY_RESULT.to_le_bytes(), "the opcode, ONCE");
        assert_eq!(wire[2], net::party::result::CREATED, "then the code");
        assert_ne!(wire[3], 0xA5, "and not the opcode a second time");
        assert!(
            out[0].body.windows(6).any(|w| w == b"Leader"),
            "the leader's seat is in the body"
        );
        assert!(
            out[0].body.windows(17).any(|w| w == b"TestCharD's Party"),
            "and the name they typed"
        );

        // A body this decoder cannot read is still answered.
        let out = s.on_party_request(net::party::CLIENT_PARTY_REQUEST, &[0xff, 0x00]);
        assert_eq!(out.len(), 1, "an undecodable body is still a blocked client");
        assert_eq!(out[0].opcode, net::party::PARTY_RESULT);

        // An empty body too - the shortest thing the client could possibly send.
        assert_eq!(s.on_party_request(net::party::CLIENT_PARTY_REQUEST, &[]).len(), 1);

        // And the invite answer, which is a different opcode with a different body.
        let out = s.on_party_request(net::party::CLIENT_PARTY_INVITE_ANSWER, &[0x1b, 1, 0, 0]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].opcode, net::party::PARTY_RESULT);
    }

    /// **The dispatcher actually routes it**, which is the half a handler test cannot show.
    ///
    /// `on_party_request` being correct is worth nothing if `0x0182` still falls through to
    /// the unknown-opcode path, which is exactly where it was before this file existed. So
    /// this goes in through `Session::handle` with the opcode on the front, the way the
    /// socket loop does.
    #[test]
    fn the_dispatcher_routes_both_party_opcodes() {
        let mut s = session();
        for opcode in [net::party::CLIENT_PARTY_REQUEST, net::party::CLIENT_PARTY_INVITE_ANSWER] {
            let mut packet = opcode.to_le_bytes().to_vec();
            packet.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]);
            let out = s.handle(&packet);
            assert_eq!(
                out.iter().filter(|r| r.opcode == net::party::PARTY_RESULT).count(),
                1,
                "{opcode:#06x} must be answered by the dispatcher, not dropped: {out:?}"
            );
        }
        // The control: an opcode this server really does not handle stays unanswered, so the
        // assertion above is about routing and not about `handle` answering everything.
        assert!(
            s.handle(&[0xFE, 0xFF, 0x00]).iter().all(|r| r.opcode != net::party::PARTY_RESULT),
            "0xFFFE is not a party packet"
        );
    }

    /// The refusal is a code whose client-side arm **reads no fields**, so it cannot run off
    /// the end of a two-byte body.
    #[test]
    fn the_refusal_is_a_read_free_code() {
        let body = net::party::request_failed();
        assert_eq!(body.len(), 1, "the code byte alone - Reply::packet adds the opcode");
        let code = body[0];
        assert!(
            net::party::is_silent_code(code),
            "code {code} must be one whose handler arm reads nothing"
        );
    }
}
