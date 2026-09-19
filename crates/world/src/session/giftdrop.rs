//! `!giftdrop` / `!giftall` - queue a compensation gift for a player or for every account, and
//! hand it over through the Maple Administrator's box. `crate::giftdrop` has the words and the
//! why; `store::gifts` the rows and the seven-day expiry.
//!
//! Entry points, all here:
//!
//! * [`Session::giftdrop_command`] / [`Session::giftall_command`] - the chat words. A GM with
//!   arguments queues; anyone typing the bare `!giftdrop` opens their own box.
//! * [`Session::open_gift_drop`] - the box for the oldest pending gift, parked under
//!   `giftdrop.<row id>`. Also what the first move after a field entry calls when something
//!   is waiting ([`Session::gift_drop_replies`]), and what an `Event::GiftDrop` from the
//!   queuing GM's session calls when the player is on this channel right now.
//! * [`Session::gift_menu_answer`] - Claim, Refuse or Cancel. Claim runs the quest room check
//!   first and refuses a full tab with the quests' own box, leaving the gift queued; the row
//!   is settled BEFORE the item moves (the Heena rule: every effect hangs off the transition).
//!   Cancel and closing the box leave it queued - for later, or for another character of the
//!   same account.

use super::{Conversation, Reply, Session};

impl Session {
    /// `!giftdrop ...` as typed. `is_gm` is the caller's flag, already read by `on_chat`.
    ///
    /// **A non-GM with arguments is said out loud**, the way every GM word is for them: the
    /// public form is the bare word, and a player typing `!giftdrop Tester2 1302000` has
    /// typed text, not a command they may run.
    pub(super) fn giftdrop_command(&mut self, arg: &str, is_gm: bool, text: &str) -> Vec<Reply> {
        if arg.trim().is_empty() {
            return self.open_gift_drop();
        }
        if !is_gm {
            return self.say_out_loud(text);
        }
        let usage = "!giftdrop <player> <itemId> [count] [message]. Try !giftdrop Tester2 2000000 10 Sorry about the crash.";
        let mut parts = arg.split_whitespace();
        let Some(player) = parts.next() else { return self.gm_ack(usage.to_string()) };
        let Ok(Some(target)) = self.store.character_id_by_name(player) else {
            return self.gm_ack(format!("!giftdrop REFUSED: no character named {player:?}."));
        };
        let rest: Vec<&str> = parts.collect();
        self.queue_gift(store::GiftTarget::Character(target), &rest.join(" "), usage, player)
    }

    /// `!giftall <itemId> [count] [message...]` - one gift per account, GM only.
    pub(super) fn giftall_command(&mut self, arg: &str) -> Vec<Reply> {
        let usage = "!giftall <itemId> [count] [message]. Try !giftall 2000000 10 Thanks for testing.";
        self.queue_gift(store::GiftTarget::Account(0), arg, usage, "every account")
    }

    /// The shared half of the two GM words: parse `<itemId> [count] [message...]`, refuse
    /// what `!item` refuses, write the row(s), and open the box on every screen this channel
    /// can reach. `target` of `Account(0)` means "every account".
    fn queue_gift(&mut self, target: store::GiftTarget, spec: &str, usage: &str, who: &str) -> Vec<Reply> {
        let word = match target {
            store::GiftTarget::Character(_) => crate::giftdrop::COMMAND_TYPED,
            store::GiftTarget::Account(_) => "!giftall",
        };
        let mut parts = spec.split_whitespace();
        let Some(Ok(item_id)) = parts.next().map(str::parse::<u32>) else {
            return self.gm_ack(format!("{word}: the item id is missing or not a number. {usage}"));
        };
        // A count is optional and numeric; anything after it is the message. A message that
        // starts with a number would be eaten as the count, so write the count out then.
        let rest: Vec<&str> = parts.collect();
        let (count, message) = match rest.first().and_then(|c| c.parse::<u16>().ok()) {
            Some(c) => (c.max(1), rest[1..].join(" ")),
            None => (1, rest.join(" ")),
        };
        // The same three refusals `!item` makes, before anything is written: a pet has no
        // body this server can build, an id outside every tab draws nothing, and an id the
        // client has no name for almost certainly has no art.
        if net::inventory::is_pet(item_id) {
            return self.gm_ack(format!("{word} REFUSED: {item_id} is a PET; sending one as a bundle kills the client."));
        }
        let Some(tab) = self.config.tab_for(item_id) else {
            return self.gm_ack(format!("{word} REFUSED: {item_id} is not in any inventory tab."));
        };
        if !self.config.item_names.contains_key(&item_id) && !self.config.shops.item_data.contains_key(&item_id) {
            return self.gm_ack(format!("{word} REFUSED: {item_id} is not in this client's Item.wz, so it has nothing to draw."));
        }
        let sender = self.claimed_character().map(|c| c.name).unwrap_or_default();
        let now = store::Store::unix_now();
        let name = self.item_name(item_id);
        let note = if message.is_empty() { String::new() } else { format!(" - {message:?}") };
        let days = store::GIFT_TTL_SECS / 86_400;
        match target {
            store::GiftTarget::Character(character) => {
                let id = match self.store.queue_gift(target, item_id, count, &message, &sender, now) {
                    Ok(id) => id,
                    Err(e) => return self.gm_ack(format!("{word} REFUSED: could not queue it: {e}")),
                };
                crate::server::log(&format!(
                    "   giftdrop: {sender} queued gift #{id} for {who} ({character}): {count}x {name} ({item_id}) into the {tab:?} tab, message {message:?}, expires in {days} days"
                ));
                // On this channel right now: their box opens at once. Elsewhere: at their
                // next field entry. Either way the row is what carries it.
                let delivered = self.bus().send_to_character(character, crate::broadcast::Event::GiftDrop);
                self.gm_ack(format!(
                    "Gift #{id} queued for {who}: {count}x {name} ({item_id}){note}; expires in {days} days. {}",
                    if delivered { "They are on this channel; their box is opening now." } else { "They are not on this channel; it opens at their next login." }
                ))
            }
            store::GiftTarget::Account(_) => {
                let n = match self.store.queue_gift_for_every_account(item_id, count, &message, &sender, now) {
                    Ok(n) => n,
                    Err(e) => return self.gm_ack(format!("{word} REFUSED: could not queue it: {e}")),
                };
                crate::server::log(&format!(
                    "   giftall: {sender} queued {count}x {name} ({item_id}) for {n} account(s), message {message:?}, expires in {days} days"
                ));
                // Everyone playing on this channel gets the box now; the rest at their next
                // field entry. `send_to_character` cannot address an account, so it is one
                // event per online character - each session opens at most one box.
                let online = self.bus().online_characters();
                let mut opened = 0;
                for c in online {
                    if self.bus().send_to_character(c, crate::broadcast::Event::GiftDrop) {
                        opened += 1;
                    }
                }
                self.gm_ack(format!(
                    "Gift queued for {n} account(s), {who}: {count}x {name} ({item_id}){note}; expires in {days} days. Any character of an account can claim it once. {opened} player(s) on this channel are being shown it now; the rest at their next login. Accounts created after this moment are not included."
                ))
            }
        }
    }

    /// The oldest gift this character can claim - its own or its account's - at `now`.
    fn pending_gifts(&self) -> Vec<store::Gift> {
        let Some(claimed) = self.claimed.as_ref() else { return Vec::new() };
        self.store
            .pending_gifts(claimed.character_id, claimed.account_id, store::Store::unix_now())
            .unwrap_or_default()
    }

    /// Open the box for the oldest pending gift, or say there is nothing.
    pub(super) fn open_gift_drop(&mut self) -> Vec<Reply> {
        let template = crate::dailyperks::ADMIN_TEMPLATE;
        let Some(chr) = self.claimed_character() else {
            return self.admin_says(template, "I cannot find your record just now. Try me again.", "no character is claimed on this connection");
        };
        let pending = self.pending_gifts();
        let Some(gift) = pending.first().cloned() else {
            return self.admin_says(template, &crate::giftdrop::nothing_to_claim(), "no pending gift");
        };
        // Replaced rather than added to: one conversation, however many times this is typed.
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: crate::giftdrop::menu_path(gift.id),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        let for_account = matches!(gift.target, store::GiftTarget::Account(_));
        let days = gift.days_left(store::Store::unix_now());
        let text = crate::giftdrop::menu_text(&gift.message, gift.item_id, gift.count, &gift.sender, for_account, days, pending.len() - 1);
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(template, &text),
            what: format!(
                "ScriptMessage MENU (type 6): gift #{} for character {} ({}) - {}x {} ({}), expires in {days} day(s), {} more waiting. Claim = 0, Refuse = 1, Cancel = 2; speaker template {template} is the {}'s portrait only",
                gift.id, chr.id, if for_account { "the account's" } else { "their own" }, gift.count, self.item_name(gift.item_id), gift.item_id, pending.len() - 1, crate::dailyperks::ADMIN_NAME
            ),
        }]
    }

    /// **The first move after a field entry**, when the entry found something waiting:
    /// the notice and the box. Idempotent: the flag is cleared before anything is sent.
    pub(super) fn gift_drop_replies(&mut self) -> Vec<Reply> {
        if !self.gift_drop_pending {
            return Vec::new();
        }
        self.gift_drop_pending = false;
        let n = self.pending_gifts().len();
        if n == 0 {
            return Vec::new();
        }
        let mut out = self.notice(crate::giftdrop::waiting_notice(n));
        out.extend(self.open_gift_drop());
        out
    }

    /// Arm [`Session::gift_drop_replies`] if this character has a gift waiting. Called after
    /// the bag restore on every field entry, so a gift queued while the player was on another
    /// channel or offline is offered the moment they are back and moving.
    pub(super) fn arm_gift_drop(&mut self) {
        if !self.pending_gifts().is_empty() {
            self.gift_drop_pending = true;
        }
    }

    /// Claim, Refuse or Cancel. `None` means "not a gift box" - fall through to the next
    /// decoder.
    pub(super) fn gift_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        let gift_id = crate::giftdrop::gift_id_from_path(&convo.path)?;
        let reply = net::script::parse_menu_reply(body)?;
        let template = convo.npc_template;
        self.conversation = None;
        // Closed: a Cancel without the line. The gift stays queued, and silence is safe for a
        // type-6 reply (it holds no latch - research/script-reply.md).
        let Some(selection) = reply.selection else { return Some(Vec::new()) };
        let Some(claimed) = self.claimed.clone() else { return Some(Vec::new()) };
        let (character_id, account_id) = (claimed.character_id, claimed.account_id);
        let Some(chr) = self.claimed_character() else { return Some(Vec::new()) };
        let now = store::Store::unix_now();
        let Ok(Some(gift)) = self.store.gift(gift_id) else {
            return Some(self.admin_says(template, &crate::giftdrop::nothing_to_claim(), &format!("gift #{gift_id} no longer exists")));
        };
        let mine = match gift.target {
            store::GiftTarget::Character(c) => c == character_id,
            store::GiftTarget::Account(a) => a == account_id,
        };
        if !mine {
            return Some(self.admin_says(template, &crate::giftdrop::nothing_to_claim(), &format!("gift #{gift_id} is {:?}, not character {character_id} / account {account_id}", gift.target)));
        }
        match selection {
            crate::giftdrop::SELECT_CLAIM => {
                // Room first, before the transition: a full tab gets the quests' own box and
                // the row stays pending for the next try.
                let Some(tab) = self.config.tab_for(gift.item_id).or_else(|| store::InventoryType::for_item(gift.item_id)) else {
                    return Some(self.admin_says(template, "That item has nowhere to go in your bag. Ask a GM.", &format!("gift #{gift_id}: item {} has no tab", gift.item_id)));
                };
                let Ok(bag) = self.store.bag(chr.id) else { return Some(Vec::new()) };
                let short = crate::questroom::shortfall(&bag, &[(gift.item_id, gift.count, tab)], &[], |id| self.config.shops.max_stack(id));
                if !short.is_empty() {
                    crate::server::log(&format!("   giftdrop: gift #{gift_id} NOT claimed - {short:?}; it stays queued"));
                    return Some(self.admin_says(template, &crate::questroom::refusal_text(&short), &format!("gift #{gift_id} kept: the bag has no room")));
                }
                // The transition, then the item. A second click on a stale box, an expired
                // row, or the account's other character having got there first all land here
                // with `false` and give nothing.
                match self.store.settle_gift(gift_id, character_id, account_id, true, now) {
                    Ok(true) => {}
                    _ => return Some(self.admin_says(template, &crate::giftdrop::nothing_to_claim(), &format!("gift #{gift_id} was already settled or has expired; NOTHING GIVEN"))),
                }
                let item = if tab == store::InventoryType::Equip { store::Item::equip(gift.item_id) } else { store::Item::bundle(gift.item_id, gift.count) };
                let placed = match self.store.add_item(chr.id, tab, &item, self.config.shops.max_stack(gift.item_id)) {
                    Ok(rows) => rows,
                    Err(e) => {
                        crate::server::log(&format!("   giftdrop: gift #{gift_id} settled but add_item failed ({e}); the row is NOT reopened automatically - a GM must requeue it"));
                        return Some(self.admin_says(template, "Something went wrong placing that in your bag. Please tell a GM.", &format!("gift #{gift_id}: add_item failed: {e}")));
                    }
                };
                crate::server::log(&format!("   giftdrop: character {} claimed gift #{gift_id} ({:?}): {}x {} into {tab:?}", chr.id, gift.target, gift.count, gift.item_id));
                let mut out = self.inventory_added_replies(tab, &placed, &format!("gift #{gift_id} claimed"));
                out.extend(self.admin_says(template, &crate::giftdrop::claimed_text(gift.item_id, gift.count), &format!("gift #{gift_id} claimed")));
                out.extend(self.next_gift_box());
                Some(out)
            }
            crate::giftdrop::SELECT_REFUSE => {
                let settled = matches!(self.store.settle_gift(gift_id, character_id, account_id, false, now), Ok(true));
                crate::server::log(&format!("   giftdrop: character {} refused gift #{gift_id} (settled: {settled})", chr.id));
                let mut out = self.admin_says(template, &crate::giftdrop::refused_text(gift.item_id), &format!("gift #{gift_id} refused"));
                out.extend(self.next_gift_box());
                Some(out)
            }
            crate::giftdrop::SELECT_CANCEL => {
                crate::server::log(&format!("   giftdrop: character {} cancelled the box for gift #{gift_id}; it stays queued", chr.id));
                Some(self.admin_says(template, &crate::giftdrop::cancelled_text(), &format!("gift #{gift_id} kept, by choice")))
            }
            other => Some(self.admin_says(template, &crate::dailyperks::no_such_option(), &format!("selection {other} on gift #{gift_id}; nothing settled"))),
        }
    }

    /// When more gifts wait, open the next box behind the answer's line.
    fn next_gift_box(&mut self) -> Vec<Reply> {
        if self.pending_gifts().is_empty() {
            Vec::new()
        } else {
            self.open_gift_drop()
        }
    }
}
