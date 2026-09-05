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
        let outcome = self.fields.parties().apply(now, actor, request);
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
            Ok(effects) => self.party_effects(actor, &described, effects),
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
                    if !self.bus().publish_to_character_anywhere(*target, notify) {
                        crate::server::log(&format!(
                            "   party: character {target} is not on this channel, so the invite \
                             dialog was NOT delivered. The invite is recorded; a target on another \
                             channel will not see it until invites cross channels"
                        ));
                    }
                }
                // The invitee declined (or the invite was overtaken). The leader is told
                // "%s has denied the party request." [L]; the invitee, if this is their own
                // decline, gets code 0x17 - an arm inside the epilogue that shows nothing [L] -
                // because an unanswered 0x0183 is a packet nobody has measured the client
                // surviving, and UNKNOWN_ERROR would print a lie.
                Effect::InviteDropped { party, target } => {
                    let target_name = self.name_of(*target);
                    if let Some(leader) = self.fields.parties().party(*party).map(|p| p.leader) {
                        let denied = Reply {
                            opcode: net::party::PARTY_RESULT,
                            body: net::party::invite_outcome(
                                net::party::invite_outcome::DENIED,
                                &target_name,
                            ),
                            what: format!(
                                "PartyResult INVITE OUTCOME 4 to character {leader}: \"{target_name} \
                                 has denied the party request.\" [L]"
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
                // Everyone in the party, the joiner included, gets 0x13 with the joiner's name:
                // "'%s' has joined the party." / "You have joined the party." [L] shape. Whether
                // the window's member list refreshes on it is NOT established - the next run says.
                Effect::Joined { party, who } => {
                    let who_name = self.name_of(*who);
                    let members = self
                        .fields
                        .parties()
                        .party(*party)
                        .map(|p| p.members.clone())
                        .unwrap_or_default();
                    for member in members {
                        let reply = Reply {
                            opcode: net::party::PARTY_RESULT,
                            body: net::party::joined(&who_name),
                            what: format!(
                                "PartyResult JOIN (0x13) to character {member}: {who_name:?} joined \
                                 party {party} [L shape; member-list refresh is [I]]"
                            ),
                        };
                        self.deliver(member, actor, reply, &mut out);
                    }
                }
                other => undecoded.push(format!("{other:?}")),
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
        } else if !self.bus().publish_to_character_anywhere(to, reply) {
            crate::server::log(&format!(
                "   party: character {to} is not on this channel and was NOT told"
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
            // sent as field 2 of the 0x03 that opened the dialog. The reader's slot order is
            // [D]; the answer byte is [L] for the auto-decline (1) and [I] for the dialog's two
            // buttons - so the raw body is logged every time, and the decision is stated as a
            // decision: 1 declines, anything else is treated as ACCEPT. If a real Decline click
            // lands here as a join, the value in this log line is the decline constant.
            match net::party::parse_invite_answer(body) {
                Some(answer) => {
                    crate::server::log(&format!(
                        "   party: 0x0183 invite answer op={:#04x} answer={} value={} raw={body:02x?}",
                        answer.op, answer.answer, answer.value
                    ));
                    let party = u32::try_from(answer.value).unwrap_or(0);
                    if answer.answer == net::party::INVITE_ANSWER_AUTO_DECLINE {
                        Ok(crate::party::Request::Decline { party })
                    } else {
                        crate::server::log(&format!(
                            "   party: answer byte {} is not the auto-decline value; ACTING AS ACCEPT \
                             [I] - the dialog's two button values have not been measured. If this \
                             was a Decline, {} is the decline constant",
                            answer.answer, answer.answer
                        ));
                        Ok(crate::party::Request::Accept { party })
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
        let config = Arc::new(Config { set_field_probe: true, ..Config::default() });
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
