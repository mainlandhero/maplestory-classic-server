//! The friend list: `0x0193` in, `0x00A7` out, and the client's own request popup.
//!
//! The owner, 2026-09-21: *"Tester2 just tried adding the owner as a friend, but nothing showed up on
//! The owner's screen."* `research/friends-2026-09-21.md` is the decode, [`net::friends`] the
//! wire, `store::friends` the rows.
//!
//! # The popup, and no chat commands
//!
//! The owner, 2026-09-22: *"When someone sends a buddy request, there should not be any chat
//! commands. Please use the client's built in UI elements, there should be one similar pop up
//! just like the party invitation, chat invitation, or trade invitation."*
//!
//! There is one, and it is the same balloon family those three use. String `0x0438` is
//! *"Friend request from"*; the balloon that draws it is kind `0x0E`
//! (`balloon+0x300`), its dedicated setter is `FUN_14180e3d0`, and the `0x00A7` handler
//! raises it from the arm for **sub-op `0x1A`** - [`net::friends::friend_request_popup`].
//!
//! **Both buttons are decoded too**, which is what retired the chat commands:
//!
//! * **Yes** -> `FUN_14180b750` calls `FUN_141829a70(ctx, balloon+0x328, ...)`, i.e. `0x0193`
//!   **sub-op 2**, carrying back the `u32` the server chose.
//! * **No** -> `FUN_14180c6e0` calls `FUN_1418297d0(..)`, i.e. **sub-op 6**, the same number.
//!
//! So this server puts the **requester's character id** in that field and the answer
//! identifies the pairing with nothing to store.
//!
//! # The 329-byte record, and the crash it caused - 2026-09-22
//!
//! The owner: *"Adding someone as a friend causes a fatal client crash to whoever the invitation
//! was sent to."* It did, and the client said why: `0x009E CLIENT_PACKET_REJECTED` class 1
//! reason `0x26`, echoing our `0x00A7` verbatim. The `0x1A` arm reads seven fields - exactly
//! the 28 bytes this server built - and then reads **one 329-byte friend record** off the same
//! packet. There was nothing left, so the read threw and the client dropped the connection
//! 3.5 s later.
//!
//! That record is decoded now ([`net::friends::FriendRecord`]) and two things follow:
//!
//! * the popup carries one, describing the requester with `flag = FLAG_REQUEST`;
//! * **`0x19` was never the list.** Its arm clears and refills its own `{id, name}` map; the
//!   rows the window strides over are the three parallel arrays beside it, and nothing had
//!   ever filled them. The list is [`net::friends::friend_records`], sub-op `0x15`, sent after
//!   `0x19` rather than instead of it. (A first version of this note claimed `0x19` *empties*
//!   the record array. It does not - it empties its map. The send order is a preference, not a
//!   requirement.)
//!
//! # The loop, 2026-09-22
//!
//! Every list reply ends in a window refresh, and a refreshed window hands its group names
//! back as `0x0193` sub-op `0x14`. Answering **that** with a list closed the ring: 32 566 round
//! trips, a 42 MB log and a frozen client. Sub-op `0x14` is a report and is answered with
//! nothing. See [`Session::on_friend_request`].
//!
//! # Always answer
//!
//! Every arm ends in a `0x00A7`. The friend window is one of the client's exclusive-request
//! windows, and an unanswered request is what leaves a UI latched.

use super::{Reply, Session};

/// **How long a friend request stays on screen before the server cancels it.**
///
/// The owner, 2026-09-22: *"The friend request does appear, but it should have a timeout if not
/// accepted within a certain amount of time."*
///
/// 60 s, and it is measured from **when the balloon was raised**, not from when the request
/// was made. That distinction is the whole design: a request can be made while the target is
/// offline or on the other channel, and the client only ever sees it at their next field
/// entry - so a timeout counted from the asking would have expired every offline request
/// before its balloon could be drawn. Counted from the offer, an offline request waits
/// patiently in the store and then gets its minute on screen.
///
/// **A logout before the minute is up is not an answer.** The row stays `Pending`, and the
/// next login offers it again with a fresh clock. Only the timeout, a Yes or a No removes it.
pub const FRIEND_REQUEST_TIMEOUT_MS: u64 = 60_000;

impl Session {
    /// `0x0193` - everything the friend window sends.
    pub(super) fn on_friend_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let request = match net::friends::FriendRequest::parse(body) {
            Ok(r) => r,
            Err(e) => {
                crate::server::log(&format!("   friends: unreadable 0x0193 body ({e})"));
                return vec![self.friend_notice(net::friends::FriendNotice::UnknownError, "the body did not parse")];
            }
        };
        match request {
            net::friends::FriendRequest::Add { name, group, .. } => self.friend_add(&name, &group),
            net::friends::FriendRequest::Groups { groups } => {
                // **A REPORT, AND ANSWERING IT IS AN INFINITE LOOP.** Measured 2026-09-22:
                // `world-ch0.log` grew to 42 MB in one sitting, 32 566 round trips of
                // `0x0193` sub-op 0x14 -> `0x00A7` 0x19 + 0x15 -> sub-op 0x14, one every
                // millisecond. The owner: *"the owner's client started lagging a lot"* and *"opening
                // the buddy list crashes/freezes the client"* - both are this.
                //
                // The cycle is structural: every list reply ends in a window refresh
                // (`FUN_142deea90` / `FUN_142debd10`), and a refreshed window hands its group
                // names back. So a list reply *causes* this packet, and answering this packet
                // with a list closes the ring.
                //
                // "Always answer" is about a request the UI is waiting on. This is not one -
                // it is the same shape as `0x013D` and `0x00B8`, a client report - and the
                // proof is on screen: the buddy list draws correctly now that nothing comes
                // back. Nothing is stored either; the group a friend sits in is already on
                // the row that built the record.
                crate::server::log(&format!(
                    "   friends: the client's {} group name(s): {:?} - a REPORT, nothing is sent back (answering it loops; session/friends.rs)",
                    groups.len(),
                    groups.iter().map(|(i, n)| format!("{i}:{n}")).collect::<Vec<_>>()
                ));
                Vec::new()
            }
            net::friends::FriendRequest::Accept { sub_op, character_id } => {
                self.friend_answer(character_id, true, sub_op)
            }
            net::friends::FriendRequest::Refuse { sub_op, character_id } => {
                self.friend_answer(character_id, false, sub_op)
            }
            other => {
                // **The sub-op is logged with its number, and nothing is changed.** The
                // popup's own two buttons are handled above; anything else is the friend
                // WINDOW's own operations (rename a group, block, delete), which are not
                // built.
                let id = match &other {
                    net::friends::FriendRequest::ById { character_id, .. }
                    | net::friends::FriendRequest::ByIdNamed { character_id, .. }
                    | net::friends::FriendRequest::Rearrange { character_id, .. } => Some(*character_id),
                    _ => None,
                };
                let waiting = self
                    .claimed_character()
                    .and_then(|chr| self.store.friend_requests_waiting(chr.id).ok())
                    .map(|w| w.iter().map(|f| format!("{} (#{})", f.name, f.friend_id)).collect::<Vec<_>>())
                    .unwrap_or_default();
                crate::server::log(&format!(
                    "   friends: 0x0193 sub-op {} ({:?}) is NOT MODELLED - character {:?}, and this list is waiting on {:?}. Nothing changed. research/friends-2026-09-21.md section 1",
                    other.sub_op(),
                    other,
                    id,
                    waiting
                ));
                self.friend_list_reply(&format!("sub-op {} is not modelled", other.sub_op()))
            }
        }
    }

    /// Sub-op 1: add somebody by name.
    ///
    /// Every refusal is the client's own sentence - see [`net::friends::FriendNotice`] - and
    /// nothing is written on any of them (`store::request_friend` decides, and its answer is
    /// used rather than logged: the Heena rule).
    fn friend_add(&mut self, name: &str, group: &str) -> Vec<Reply> {
        use net::friends::FriendNotice;
        use store::friends::FriendRequestOutcome as O;
        let Some(chr) = self.claimed_character() else {
            return vec![self.friend_notice(FriendNotice::UnknownError, "no character is claimed")];
        };
        let Ok(Some(target)) = self.store.character_id_by_name(name) else {
            return vec![self.friend_notice(FriendNotice::NoSuchCharacter, &format!("nobody is called {name:?}"))];
        };
        let group = if group.trim().is_empty() { "Default Group" } else { group };
        let outcome = match self.store.request_friend(chr.id, target, group) {
            Ok(o) => o,
            Err(e) => {
                return vec![self.friend_notice(FriendNotice::UnknownError, &format!("the store refused: {e}"))]
            }
        };
        crate::server::log(&format!(
            "   friends: {} (#{}) asked to add {name} (#{target}) into {group:?} -> {outcome:?}",
            chr.name, chr.id
        ));
        match outcome {
            O::Asked => {
                let mut out = vec![Reply {
                    opcode: net::friends::FRIEND_RESULT,
                    body: net::friends::friend_request_sent(name),
                    what: format!("FriendResult 0x1B: \"Buddy request successfully sent to {name}.\""),
                }];
                out.extend(self.friend_list_reply("a request was sent"));
                // **The other side is told, wherever they are.** On this channel it arrives
                // now; otherwise the row is already written and their next field entry says
                // it (`friend_entry_replies`).
                let delivered = self.bus().send_to_character(target, crate::broadcast::Event::FriendRequest);
                crate::server::log(&format!(
                    "   friends: {name} (#{target}) {}",
                    if delivered { "is on this channel; they are being told now" } else { "is not on this channel; they will be told at their next field entry" }
                ));
                out
            }
            // They had already asked, so asking back settles it: both lists gain a friend.
            O::AcceptedTheirs => {
                let mut out = self.friend_list_reply(&format!("{name}'s request was accepted by asking back"));
                out.push(Self::now_your_friend(name));
                self.bus().send_to_character(target, crate::broadcast::Event::FriendRequest);
                out
            }
            O::AlreadyFriends => vec![self.friend_notice(FriendNotice::AlreadyYourBuddy, name)],
            O::AlreadyAsked => vec![self.friend_notice(FriendNotice::RequestAlreadySent, name)],
            O::TheyAreWaiting => vec![self.friend_notice(FriendNotice::TheyAreWaiting, name)],
            O::NoSuchCharacter => vec![self.friend_notice(FriendNotice::NoSuchCharacter, name)],
            O::Yourself => vec![self.friend_notice(FriendNotice::NotYourself, name)],
            O::YourListIsFull => vec![self.friend_notice(FriendNotice::YourListIsFull, name)],
            O::TheirListIsFull => vec![self.friend_notice(FriendNotice::TheirListIsFull, name)],
        }
    }

    /// This character's friend list: the `0x19` name cache, then the `0x15` records.
    ///
    /// **Both, in that order.** `0x19` is an id-to-name cache and `0x15` is the rows; the
    /// order is a preference (a cache that arrives before the rows it names cannot be stale)
    /// rather than a requirement. Only accepted friends are listed: a request that is waiting
    /// is an invitation rather than a row, and it arrives as the client's own balloon
    /// ([`Self::friend_popup`]).
    ///
    /// **Every call here causes the client to hand its group names back** - the refresh at the
    /// end of both arms does it - so nothing may answer that report. See
    /// [`Session::on_friend_request`].
    ///
    /// The `flag` byte is where online-ness lives, so the roster is asked - the world hub when
    /// one is linked, this channel's bus otherwise - and a friend who is playing gets
    /// `FLAG_ONLINE` and their channel. Nothing here authenticates; presence is presence.
    pub(super) fn friend_list_reply(&self, why: &str) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        self.friend_list_for(chr.id, &chr.name, why)
    }

    /// The same two packets, for **any** character - so one session can hand another's window
    /// its rows without waiting for that session's own tick.
    ///
    /// [`Self::notify_friends_of_presence`] needs exactly this: the order of the three packets
    /// it sends is load-bearing, and a mailbox delivers published replies ahead of queued
    /// events, so routing the list through the bus would have put it *after* the `0x2D` that
    /// depends on it. Measured by the test that pins the order.
    pub(super) fn friend_list_for(&self, character_id: u32, name: &str, why: &str) -> Vec<Reply> {
        let friends: Vec<store::friends::Friend> = self
            .store
            .friends(character_id)
            .unwrap_or_default()
            .into_iter()
            .filter(|f| f.state == store::friends::FriendState::Accepted)
            .collect();
        let rows: Vec<(u32, String)> = friends.iter().map(|f| (f.friend_id, f.name.clone())).collect();
        let records: Vec<net::friends::FriendRecord> = friends
            .iter()
            .map(|f| {
                // Level and job come from the row, not from presence, so they are there for
                // an offline friend too.
                let brief = self.store.character_brief(f.friend_id).ok().flatten();
                net::friends::FriendRecord::friend(
                    f.friend_id,
                    &f.name,
                    &f.group,
                    self.friend_channel(f.friend_id),
                    brief.as_ref().map(|c| c.level).unwrap_or(0),
                    brief.as_ref().map(|c| c.job).unwrap_or(0),
                )
            })
            .collect();
        let online = records.iter().filter(|r| r.flag == net::friends::FLAG_ONLINE).count();
        vec![
            Reply {
                opcode: net::friends::FRIEND_RESULT,
                body: net::friends::friend_list(&rows),
                what: format!("FriendResult 0x19: the id-to-name map, {} row(s) for {name} - {why}", rows.len()),
            },
            Reply {
                opcode: net::friends::FRIEND_RESULT,
                body: net::friends::friend_records(&records),
                what: format!(
                    "FriendResult 0x15: THE LIST, {} record(s) ({online} online) for {name} - {why}",
                    records.len()
                ),
            },
        ]
    }

    /// Which channel a friend is playing on, or `None` when they are not.
    ///
    /// The hub knows the whole world; without one, only this channel can be answered, and a
    /// friend on channel 2 then reads as offline. That is a smaller lie than claiming they are
    /// here.
    fn friend_channel(&self, character_id: u32) -> Option<u32> {
        match crate::link::installed() {
            Some(link) => link.everyone().into_iter().find(|(id, _)| *id == character_id).map(|(_, e)| e.channel),
            None => {
                if self.bus().character_online(character_id) {
                    Some(self.config.channel_id)
                } else {
                    None
                }
            }
        }
    }

    /// The list, plus **the client's own popup** for every request still waiting.
    ///
    /// Sent on field entry and whenever somebody asks while this character is on the channel.
    /// A popup is raised **once per session per requester**: the request survives in the
    /// store either way, and re-raising it on every map change would put a balloon on screen
    /// for something the player already refused to answer.
    pub(super) fn friend_entry_replies(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        // **The first field entry is where a login is announced**, because it is the first
        // moment the roster says this character is playing. See `Session::announced_presence`.
        if !self.announced_presence {
            self.announced_presence = true;
            self.notify_friends_of_presence(true);
        }
        let waiting = self.store.friend_requests_waiting(chr.id).unwrap_or_default();
        let mut out = self.friend_list_reply("a field entry");
        let now = self.clock_ms;
        for f in &waiting {
            if self.friend_popups_raised.contains_key(&f.friend_id) {
                continue;
            }
            // The offer's clock starts here, not when the row was written - see
            // `FRIEND_REQUEST_TIMEOUT_MS`.
            self.friend_popups_raised.insert(f.friend_id, now);
            out.push(self.friend_popup(f.friend_id, &f.name, &f.group));
        }
        out
    }

    /// **A friend request nobody answered, [`FRIEND_REQUEST_TIMEOUT_MS`] after its balloon
    /// went up.**
    ///
    /// The store decides, not this clock: `answer_friend_request(.., false)` is guarded on the
    /// row still being `Pending`, and **every effect here hangs off its answer** - the Heena
    /// rule. A `false` means the player pressed Yes or No in the meantime, or the requester
    /// took it back, and then nothing is sent and the entry is simply dropped.
    ///
    /// Both sides get the client's own sentence for it, `0x2A` *"The request to add a Friend
    /// has been canceled."* - the target because their balloon may still be on screen with
    /// nothing behind it, and the requester because otherwise they wait forever on a request
    /// that no longer exists.
    ///
    /// **[I]:** whether `0x2A` also dismisses the balloon. The string is the right one and it
    /// is the client's own; what the arm does to a live kind-`0x0E` balloon is not decoded, so
    /// a stale balloon whose Yes now answers nothing is possible. That path is safe - the
    /// answer finds no pending row, says so in the log, and re-sends the list - and it is
    /// plan step 12(h).
    pub(super) fn friend_timeout_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let due: Vec<u32> = self
            .friend_popups_raised
            .iter()
            .filter(|(_, raised)| now_ms.saturating_sub(**raised) >= FRIEND_REQUEST_TIMEOUT_MS)
            .map(|(id, _)| *id)
            .collect();
        let mut out = Vec::new();
        for requester_id in due {
            self.friend_popups_raised.remove(&requester_id);
            match self.store.answer_friend_request(chr.id, requester_id, false) {
                Ok(true) => {
                    let name = self.character_name(requester_id);
                    crate::server::log(&format!(
                        "   friends: {} (#{}) never answered {name} (#{requester_id}); the request is CANCELLED after {} s",
                        chr.name,
                        chr.id,
                        FRIEND_REQUEST_TIMEOUT_MS / 1000
                    ));
                    out.push(self.friend_notice(
                        net::friends::FriendNotice::Cancelled,
                        &format!("{name}'s request timed out unanswered"),
                    ));
                    out.extend(self.friend_list_reply("a request timed out"));
                    // The asker is told too, wherever they are, and their list loses the row.
                    self.bus().publish_to_character_anywhere(
                        requester_id,
                        Reply {
                            opcode: net::friends::FRIEND_RESULT,
                            body: net::friends::friend_notice(net::friends::FriendNotice::Cancelled),
                            what: format!(
                                "FriendResult 0x2A: \"The request to add a Friend has been canceled.\" - {} never answered",
                                chr.name
                            ),
                        },
                    );
                    self.bus().send_to_character(requester_id, crate::broadcast::Event::FriendRequest);
                }
                Ok(false) => crate::server::log(&format!(
                    "   friends: the offer to character {} from #{requester_id} timed out, but the store says nothing is waiting - it was already answered. Nothing sent",
                    chr.id
                )),
                Err(e) => crate::server::log(&format!(
                    "   friends: cancelling the timed-out request from #{requester_id} failed: {e}"
                )),
            }
        }
        out
    }

    /// **Tell every friend that this character just came online, or just went offline.**
    ///
    /// The owner, 2026-09-22: *"when Tester2 logs in after the owner, the owner was not informed"*, and
    /// *"once the owner logs off, the buddy list also remains showing the owner is still online."* Both
    /// were the same gap: the list was only ever re-sent on the *friend's own* field entry, so
    /// somebody else's login or logout reached them not at all.
    ///
    /// **Two packets per friend, and the order matters.** The list goes first, because it
    /// carries `rec[0x11]` - the flag that greys the row - and this server leaves `rec+0x145`
    /// at zero in it. The `0x2D` then flips that word, which is what the client tests for a
    /// change before it says *"[Friend] %s has logged in."* Sent the other way round, the list
    /// would overwrite the status and the line would be announced against a row that already
    /// agreed.
    ///
    /// Only accepted friends are told, and only ones the bus can reach - an offline friend
    /// needs no notice, because their next login sends them the whole list anyway.
    pub(super) fn notify_friends_of_presence(&self, online: bool) {
        let Some(chr) = self.claimed_character() else { return };
        let channel = self.config.channel_id;
        let friends = self.store.friends(chr.id).unwrap_or_default();
        let mut told = 0;
        for f in friends.iter().filter(|f| f.state == store::friends::FriendState::Accepted) {
            // Their list, built here rather than asked for through the bus: a mailbox hands out
            // published replies before queued events, so an event would arrive AFTER the
            // `0x2D` below and overwrite the status word it just set.
            let mut reached = false;
            for reply in self.friend_list_for(f.friend_id, &f.name, "a friend's presence changed") {
                reached = self.bus().publish_to_character_anywhere(f.friend_id, reply);
            }
            if !reached {
                continue;
            }
            self.bus().publish_to_character_anywhere(
                f.friend_id,
                Reply {
                    opcode: net::friends::FRIEND_RESULT,
                    body: net::friends::friend_status(
                        chr.id,
                        chr.id,
                        if online { net::friends::STATUS_ONLINE } else { net::friends::STATUS_OFFLINE },
                        if online { channel } else { 0 },
                        online,
                    ),
                    what: format!(
                        "FriendResult 0x2D: {} is {} - {}",
                        chr.name,
                        if online { format!("ONLINE on channel {}", channel + 1) } else { "OFFLINE".to_string() },
                        if online { "and their client says so out loud" } else { "quietly" }
                    ),
                },
            );
            told += 1;
        }
        if told > 0 {
            crate::server::log(&format!(
                "   friends: {} (#{}) went {}; {told} friend(s) on this channel were told",
                chr.name,
                chr.id,
                if online { "ONLINE" } else { "offline" }
            ));
        }
    }

    /// A character's name, or `#id` when the row has outlived them.
    fn character_name(&self, character_id: u32) -> String {
        self.store
            .character_brief(character_id)
            .ok()
            .flatten()
            .map(|c| c.name)
            .unwrap_or_else(|| format!("#{character_id}"))
    }

    /// The `0x00A7` sub-op `0x1A` that raises *"Friend request from <name>"*.
    ///
    /// The `u32` the balloon keeps and hands back is the **requester's character id**, so the
    /// Yes and No that follow name the pairing by themselves.
    fn friend_popup(&self, requester_id: u32, name: &str, group: &str) -> Reply {
        let record = net::friends::FriendRecord::request(requester_id, name, group);
        Reply {
            opcode: net::friends::FRIEND_RESULT,
            body: net::friends::friend_request_popup(requester_id, &record),
            what: format!(
                "FriendResult 0x1A: the \"Friend request from {name}\" balloon (kind 0x0E), echoing character {requester_id}, with the {}-byte record the arm reads",
                net::friends::FRIEND_ENTRY_LEN
            ),
        }
    }

    /// **The popup's Yes (sub-op 2) and No (sub-op 6).**
    ///
    /// `character_id` is the number this server put in the popup, so it is the requester.
    /// `store::answer_friend_request` decides - a `false` from it means there was nothing
    /// waiting, and then nothing is written and nothing is sent but a refreshed list.
    fn friend_answer(&mut self, requester_id: u32, accept: bool, sub_op: u8) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let name = self.character_name(requester_id);
        // Answered, so the offer's clock stops - otherwise `friend_timeout_tick` would still
        // fire a minute later and ask the store to cancel something already settled.
        self.friend_popups_raised.remove(&requester_id);
        match self.store.answer_friend_request(chr.id, requester_id, accept) {
            Ok(true) => {
                crate::server::log(&format!(
                    "   friends: {} (#{}) {} {name} (#{requester_id}) from the popup (sub-op {sub_op})",
                    chr.name,
                    chr.id,
                    if accept { "ACCEPTED" } else { "refused" }
                ));
                let mut out = self.friend_list_reply(if accept { "a request was accepted" } else { "a request was refused" });
                if !accept {
                    // The asker's own client has a sentence for this one.
                    self.bus().publish_to_character_anywhere(
                        requester_id,
                        Reply {
                            opcode: net::friends::FRIEND_RESULT,
                            body: net::friends::friend_declined(&chr.name),
                            what: format!("FriendResult 0x32: \"{} has declined the friend request.\"", chr.name),
                        },
                    );
                } else {
                    out.push(Self::now_your_friend(&name));
                }
                // Their list changed too, and only their session can redraw it.
                self.bus().send_to_character(requester_id, crate::broadcast::Event::FriendRequest);
                out
            }
            Ok(false) => {
                crate::server::log(&format!(
                    "   friends: character {} answered a request from {name} (#{requester_id}) that is not waiting (sub-op {sub_op}); nothing changed",
                    chr.id
                ));
                self.friend_list_reply("an answer with nothing waiting")
            }
            Err(e) => {
                crate::server::log(&format!("   friends: answering {requester_id} failed: {e}"));
                vec![self.friend_notice(net::friends::FriendNotice::UnknownError, "the store refused the answer")]
            }
        }
    }

    /// *"<name> is now your friend."*, in the client's own system colour.
    ///
    /// The owner, 2026-09-22: *"`Tester2 is now your friend` should also show in client opcode
    /// instead of a message we write"*, then *"if it has to be a chat message we send, can we
    /// send it as a red system message?"*
    ///
    /// **It has to be one.** All 6165 strings in this client's table were searched: there is no
    /// *"is now your friend"* among them, and the nearest the client owns is `0x03EE`
    /// *"[Friend] %s has logged in."* - a different event. So the sentence is ours, and the
    /// most that can be inherited is the **drawing**: [`net::broadcast::SYSTEM_LINE`] ends in
    /// the same printer, with the same kind (`0xb`), as `0x00A7` sub-op `0x32` *"%s has
    /// declined the friend request."* - the line already on screen in that colour.
    fn now_your_friend(name: &str) -> Reply {
        Reply {
            opcode: net::broadcast::BROADCAST_MSG,
            body: net::broadcast::system_line(&format!("{name} is now your friend.")),
            what: format!("BroadcastMsg type 5: \"{name} is now your friend.\" in the system colour (chat kind 0xb)"),
        }
    }

    /// One of the client's own sentences, and a log line saying which and why.
    fn friend_notice(&self, notice: net::friends::FriendNotice, why: &str) -> Reply {
        Reply {
            opcode: net::friends::FRIEND_RESULT,
            body: net::friends::friend_notice(notice),
            what: format!("FriendResult {notice:?} (sub-op {:#04x}) - {why}", notice.sub_op()),
        }
    }
}
