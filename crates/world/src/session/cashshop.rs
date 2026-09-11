//! The Cash Shop: entering it, the balance, and getting back out.
//!
//! The owner, 2026-08-22: *"Please build the Cash Shop, make sure the experience is functional."*
//!
//! # There is no migrate and no second server
//!
//! Both research passes reached this independently, and it reverses what `research/cash-shop.md`
//! §4 said twice. `0x01A0..0x01A3` are the four cases of **one stage forwarder**,
//! `FUN_142097EE0`, and `CField::OnPacket` chains into it - which is exactly why this server's
//! `!map` `0x01A0` already works mid-session on a live channel socket. So `0x01A3` goes out on
//! the connection the client is already using, and `crates/cashshop` is not needed.
//!
//! # The round trip
//!
//! ```text
//! <- 0x00D5  the button            ->  0x01A3 SetCashShop  +  0x05AD wallet
//! <- 0x03E0  "what is my balance"  ->  0x05AD wallet
//! <- 0x03E1  a click in the shop   ->  see `on_cash_shop_action` - a buy is 0x05AE 0x0C
//!                                      then 0x05AD, IN THAT ORDER
//! <- 0x00D1  Exit, EMPTY body      ->  0x01A0 SetField, characterData = 1
//! ```
//!
//! # `0x00D1` means two different things and the length is the only thing that separates them
//!
//! A portal walk sends **35 bytes**; the cash shop's Exit button sends **none**. Getting that
//! split backwards would make every portal in the game try to leave a cash shop, so the
//! dispatcher checks the length and this module owns only the empty case.
//!
//! # What has been on a wire, and what has not
//!
//! This block used to say *"none of this has ever been on a wire"*. Most of it now has been.
//!
//! **Confirmed on a client, 2026-08-25 and 2026-08-26:** the window draws; `0x01A3` reached
//! its handler (one hook line, dispatching that exact opcode), so it is **read** rather than
//! the `[D]` it was; the stage's own `OnPacket` accepted both wallets; both balance fields
//! read back in the right order; the `0x03E0` poll fired twice in 103 s, so `0x05AD` clears
//! its own latch; an empty `0x00D1` brought the field back with its NPCs; and three buy
//! clicks produced three refusals with no ejection.
//!
//! **Not yet on a wire:** the purchase reply `0x05AE 0x0C`, the cash-item record it carries -
//! eleven of whose fifteen fields have **no reader anywhere in the cash shop**, so a wrong
//! value there fails silently - and the two locker moves, which are refused.

use super::*;

impl Session {
    /// `0x00D5` - the player clicked Cash Shop. Send them in.
    ///
    /// # This replaced a `0x0070` workaround, and the workaround has to go
    ///
    /// Until now this answered with `inventory_rejected()`, whose only job was to clear the
    /// exclusive-request latch at `[ctx+0x2330]` so the button would fire more than once.
    /// **`0x01A3` clears that latch itself** (`FUN_142CBE8F0` at `0x142CBE918`), so sending
    /// both would be sending a packet whose entire purpose is already served - and a stray
    /// `0x0070` tells the client an inventory operation completed when none did.
    pub(super) fn on_cash_shop_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(claimed) = self.claimed() else { return Vec::new() };
        let account_id = claimed.account_id;

        let seen = match net::cashshop::parse_cash_shop_request(body) {
            Some(req) => format!("client tick {}, flag {}", req.tick, req.flag),
            // Answer anyway: the latch was set by the client's builder before we ever saw the
            // body, so failing to parse it is no reason to leave the button dead.
            None => format!("unreadable {}-byte body {body:02x?}", body.len()),
        };

        // The client stops drawing the field from here until the Exit button. See
        // `Session::in_cash_shop` - this suppresses the idle chatter and nothing else.
        self.in_cash_shop = true;
        // **A shopper is not on the map any more, and everyone else has to be told.**
        //
        // The Cash Shop is a different stage: the client tears the field down and the player
        // is not standing anywhere. Without this they stay drawn on whatever map they left,
        // frozen, until they come back - and if they log out from inside the shop they stay
        // drawn until the connection drops.
        //
        // The return trip needs nothing: leaving the shop runs `on_field_entered`, which
        // announces the arrival again. `Bus::leave_field` is idempotent, so the `Drop` path
        // and the log-out path can both still run.
        self.leave_the_field();
        // And the mobs it was driving are handed to somebody still on that map, for the same
        // reason: a shopper is not on the map, and a mob whose controller is looking at the
        // Cash Shop is a mob nobody is moving. Field entry re-claims on the way back, so this
        // costs the shopper nothing and unfreezes everybody else's screen while it browses.
        self.hand_over_mobs(chr.map_id);
        let (quests, _) = self.quest_book(chr.id);
        let skills = self.store.skills(chr.id).unwrap_or_default();
        let mut out = vec![Reply {
            opcode: net::cashshop::SET_CASH_SHOP,
            body: net::cashshop::set_cash_shop(
                &chr,
                self.config.world_id,
                self.clock_base(),
                &self.dressed(&chr),
                &quests,
                &skills,
            ),
            what: format!(
                "SetCashShop: {seen}. 0x01A3 on THIS socket - there is no migrate; 0x01A0..0x01A3 \
                 are four arms of one stage forwarder and CField::OnPacket chains into it. The \
                 opcode number was [D] and is now READ: the hook logged an entry to \
                 0x14209ad60 dispatching it, and the window drew. It also clears ctx+0x2330 itself, \
                 which is why no 0x0070 goes with it"
            ),
        }];
        out.extend(self.cash_wallet_reply(account_id, "sent unprompted with SetCashShop"));
        out
    }

    /// `0x03E0` - "what is my balance". Empty body, and the client throttles it to 60 s.
    pub(super) fn on_cash_shop_query(&mut self) -> Vec<Reply> {
        let Some(claimed) = self.claimed() else { return Vec::new() };
        self.cash_wallet_reply(claimed.account_id, "answering 0x03E0")
    }

    /// One `0x05AD`. **The only packet that carries a balance** - no `CWvsContext` opcode does,
    /// so without this the shop has nothing to spend whatever the database says.
    fn cash_wallet_reply(&self, account_id: i64, why: &str) -> Vec<Reply> {
        let w = self.store.cash_wallet(account_id).unwrap_or_default();
        vec![Reply {
            opcode: net::cashshop::CASH_SHOP_WALLET,
            body: net::cashshop::cash_shop_wallet(w.nx, w.maple_points),
            what: format!(
                "CashShopWallet: {} NX, {} maple points for account {account_id} - {why}. \
                 It also clears the client's own 60-second query latch at [this+0x74]",
                w.nx, w.maple_points
            ),
        }]
    }

    /// `0x00D1` with an **empty** body - the Exit button, not a portal.
    ///
    /// The answer is an ordinary `SetField` for the map the character is already on, which is
    /// the same packet `!map` sends. Leaving is therefore re-entering the field they never
    /// really left.
    ///
    /// **Nothing on the `0x01A3` path writes `[ctx+0x31fc]`**, and that field is one of the
    /// six gates the Cash Shop button itself checks - `> 1` raises *"You cannot go into the
    /// cash shop. Please try again later."* If leaving turns out to work but a second entry
    /// does not, that is the field to look at, and it is written down here because the run
    /// that finds it will otherwise look like a fresh mystery.
    pub(super) fn on_cash_shop_exit(&mut self) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        // `go_to_map` to the map they are already on: it is the one path that sends a DRESSED
        // record with quests and skills, and using it rather than hand-rolling a SetField is
        // what stopped equips losing their stats on every portal in 2026-08-19.
        self.in_cash_shop = false;
        let (here, portal) = (chr.map_id, chr.portal);
        self.go_to_map(&mut chr, here, portal, "leaving the Cash Shop".to_string())
    }
}

impl Session {
    /// `0x03E1` - **a click inside the shop.** A buy now completes; the rest are refused.
    ///
    /// # The claim this used to make was wrong, and it blocked the feature for two days
    ///
    /// This block said a purchase could not be reported at all, because *"every arm that
    /// clears the in-flight latch also calls `FUN_140D7C7F0`, which puts a message on screen.
    /// There is no silent one."* The first sentence is true of the six arms whose bodies are
    /// **inline** in `FUN_140D7DCA0`. It is false for the two that **delegate to
    /// sub-functions**, and those two - `0x19` and `0x1B` - are silent.
    ///
    /// That is `CLAUDE.md`'s oldest failure wearing new clothes: a **known list** was searched
    /// instead of the space being enumerated, and it produced a clean, confident, wrong
    /// negative that a whole design was then built around. `research/cash-shop-buy-done.md`
    /// broke it with `tools/callers.py` over `FUN_140D7C7F0` whole-image, in all three modes.
    ///
    /// # What each family gets
    ///
    /// ```text
    /// 0x02 / 0x1F  buy      -> 0x05AE 0x0C + 0x05AD, in that order   (or 0x1A + u8 reason)
    /// 0x0A/0x0B/0x1C queue  -> 0x05AE 0x3D + u16 reason  - 0x1A would EMPTY the queue
    /// 0x03 gift, unknown    -> 0x05AE 0x1A + u8 reason
    /// 0x2B                  -> nothing. It is the one builder that does not latch
    /// ```
    ///
    /// # The serial's offset is measured now
    ///
    /// Three captures, 2026-08-26, all at payload offset 7, each resolving to the item the owner
    /// said they had clicked. The offset-walk that found it is kept as the fallback for the
    /// builder's short arm, which has never been seen -
    /// [`crate::commodity::CommodityTable::identify_serial`].
    pub(super) fn on_cash_shop_action(&mut self, body: &[u8]) -> Vec<Reply> {
        use net::cashshop::reason;

        let Some(action) = net::cashshop::parse_cash_shop_action(body) else {
            // An empty body still latched before it was sent, so it still has to be answered.
            return self.refuse_cash_shop(
                reason::UNKNOWN_ERROR,
                "an EMPTY 0x03E1 - no sub-op byte. Answered anyway: the builder set \
                 [stage+0x74] before it sent, and the shop blocks until that is cleared"
                    .to_string(),
            );
        };

        // **Not every sub-op takes the same refusal, and using one for all of them destroys
        // work.** `research/cash-shop-actions.md`: `0x0A`/`0x0B`/`0x1C` are a QUEUE of 32-byte
        // records, and `0x1A` empties the queue vector - so refusing the first of five queued
        // deletes with it would silently drop the other four.
        match net::cashshop::action_family(action.sub_op) {
            net::cashshop::ActionFamily::Buy => {}
            net::cashshop::ActionFamily::Queued => {
                return self.refuse_cash_shop_queue(
                    reason::UNKNOWN_ERROR as u16,
                    format!(
                        "0x03E1 sub-op 0x{:02X}, a QUEUED operation (move or delete), {} byte \
                         payload {:02x?} - not built. Refused with 0x3D rather than 0x1A, which \
                         would empty the whole queue and lose whatever else is pending",
                        action.sub_op,
                        action.rest.len(),
                        action.rest
                    ),
                )
            }
            // **The documented exception to "always answer".** This one does not latch, and
            // the only refusal available would discard the queue. Measured, with a positive
            // control - see net::cashshop::ACTION_NO_LATCH.
            //
            // **There is nowhere to log this from, so the label carries it instead.**
            // `Session` is a pure state machine with no socket and no logger - the only text
            // that reaches `world.log` is a `Reply`'s `what`, and this path sends no reply.
            // So the explanation lives on the INBOUND line: `net::names` labels `0x03E1` with
            // "0x2B the ONE that does not latch, so it is deliberately unanswered", which puts
            // it in front of whoever reads the log without needing them to find this comment.
            // A deliberate silence that is explained nowhere becomes a bug report later.
            net::cashshop::ActionFamily::NoLatch => return Vec::new(),
            net::cashshop::ActionFamily::Other => {
                return self.refuse_cash_shop(
                    reason::UNKNOWN_ERROR,
                    format!(
                        "0x03E1 sub-op 0x{:02X}{}, {} byte payload {:02x?} - not built. 0x1A is \
                         right here even though it costs the queue: it is the only arm that \
                         clears [stage+0x120], and a gift sets that to 2, a value nothing in \
                         the reachable class compares against - so an unanswered gift blocks \
                         every later buy, move and delete while the UI still looks alive",
                        action.sub_op,
                        if action.sub_op == net::cashshop::ACTION_GIFT { " GIFT" } else { "" },
                        action.rest.len(),
                        action.rest
                    ),
                )
            }
        }

        let Some(claimed) = self.claimed() else { return Vec::new() };
        let account_id = claimed.account_id;
        // **Leaf Points, not NX.** Measured 2026-08-25: every price tag in the shop reads LP,
        // and a client holding 10,000 NX and 0 LP refused the purchase itself and sent no
        // 0x03E1 at all. The affordability gate is client-side and it reads this balance, so
        // the server has to agree with it or the two disagree about what "afford" means.
        let lp = self.store.cash_wallet(account_id).unwrap_or_default().maple_points;

        // Which u32 in the payload is the commodity serial. See the module doc.
        let found = self.config.commodity.identify_serial(action.rest);
        let seen = format!(
            "0x03E1 sub-op 0x{:02X} BUY, {} byte payload {:02x?}",
            action.sub_op,
            action.rest.len(),
            action.rest
        );

        match found {
            crate::commodity::SerialMatch::One { offset, sn } => {
                // Cloned out because `self` is borrowed mutably below.
                let row = self.config.commodity.get(sn).cloned();
                let Some(row) = row else { return self.refuse_cash_shop(reason::SOLD_OUT, seen) };
                let what = format!(
                    "{seen}. THE SERIAL IS AT OFFSET {offset}: SN {sn} = {}x {} ({}), \
                     {} LP, {} days, on sale {}. Wallet holds {lp} LP",
                    row.count, row.name, row.item_id, row.price, row.period_days, row.on_sale
                );
                if !row.on_sale {
                    return self.refuse_cash_shop(reason::SOLD_OUT, format!("{what} - NOT on sale"));
                }
                if lp < row.price {
                    return self.refuse_cash_shop(
                        reason::NOT_ENOUGH_CASH,
                        format!("{what} - CANNOT AFFORD IT. `!lp {}` would cover it", row.price),
                    );
                }
                self.complete_purchase(account_id, &row, what)
            }
            crate::commodity::SerialMatch::None => self.refuse_cash_shop(
                reason::SOLD_OUT,
                format!(
                    "{seen} - NO commodity serial anywhere in it. Candidates read at every \
                     offset: {:?}. Either the payload does not carry an SN, or \
                     gm-handbook/commodity.txt is not loaded ({} rows)",
                    net::cashshop::u32_candidates(action.rest),
                    self.config.commodity.len()
                ),
            ),
            crate::commodity::SerialMatch::Several(hits) => self.refuse_cash_shop(
                reason::UNKNOWN_ERROR,
                format!(
                    "{seen} - TWO OR MORE offsets hold a real serial: {hits:?}. Refused rather \
                     than resolved; picking the first would be a guess wearing a decode"
                ),
            ),
        }
    }

    /// **Take the price and hand the item over**, or change nothing at all.
    ///
    /// # The pair, and why the order is the whole safety argument
    ///
    /// ```text
    /// 0x05AE 0x0C   the item, into the CASH INVENTORY
    /// 0x05AD        the debited balance - and the three other things it does
    /// ```
    ///
    /// **This pair used to be `0x19` then `0x05AD`, and that was wrong twice over.** `0x19` is
    /// the reply to a locker-to-bag *move*, so the coupon arrived in the Item Inventory; and
    /// because `0x19` cleared `[stage+0x120]` itself, the wallet never re-entered the buy
    /// builder and the client's own success message never ran. One packet, two symptoms.
    ///
    /// The wallet is what does the work here. It writes `[stage+0x74] = 0` unconditionally,
    /// then reads `[stage+0x120]`, sees the buy's `1`, zeroes it, and re-enters the buy builder
    /// on its **completion** path - which fetches string 590, *"You have successfully made the
    /// purchase."* So `0x0C` deliberately clears **neither** latch and the order is what makes
    /// the sequence work. [`net::cashshop::RESULT_ITEM_TO_LOCKER`] has the addresses.
    ///
    /// # Two stores, so a compensating transaction rather than one
    ///
    /// The wallet and the bag are different tables with different invariants, so this debits
    /// first - `add_maple_points` **refuses** a negative balance rather than clamping, which
    /// makes the debit the affordability check too - and refunds if the bag will not take the
    /// item. Every effect hangs off the transition: nothing is reported unless both halves
    /// succeeded, and a refund that itself fails is said out loud rather than swallowed.
    ///
    /// # Pets are refused here, not sent
    ///
    /// `net::inventory::is_pet`. Sending a pet as a bundle kills this client - measured
    /// 2026-08-26 - and four of the 159 sale rows are pets.
    fn complete_purchase(
        &mut self,
        account_id: i64,
        row: &crate::commodity::Commodity,
        what: String,
    ) -> Vec<Reply> {
        use net::cashshop::reason;

        if net::inventory::is_pet(row.item_id) {
            return self.refuse_cash_shop(
                reason::UNKNOWN_ERROR,
                format!(
                    "{what} - REFUSED: {} is a PET and this server cannot build a type-3 item \
                     body yet. Sending one as a bundle killed the client on 2026-08-26",
                    row.item_id
                ),
            );
        }
        // **Into the LOCKER, which is the shop's Cash Inventory panel** - not the bag.
        // `store::buy_cash_item` checks the balance, debits Leaf Points and places the item in
        // one transaction, which is what it was written to do before the wrong packet sent the
        // purchase down the bag path.
        let Some(inv) = store::InventoryType::for_item(row.item_id) else {
            return self.refuse_cash_shop(
                reason::UNKNOWN_ERROR,
                format!("{what} - REFUSED: item {} is in no inventory tab", row.item_id),
            );
        };
        let item = if inv == store::InventoryType::Equip {
            store::Item::equip(row.item_id)
        } else {
            store::Item::bundle(row.item_id, row.count.max(1))
        };
        // **The reason has to be the store's reason, not a default.** Until 2026-09-10 every
        // store error came back as NOT_ENOUGH_CASH, and the one that actually fired was a
        // schema gap ("no such column: failed_slots" on `cash_locker`) - so the owner, holding
        // 105,500 LP, was told they could not afford a 100 LP coupon. A wrong reason is worse
        // than the generic one: it sends the player to check the one thing that is fine.
        let placed = match self.store.buy_cash_item(account_id, &item, row.price) {
            Ok(l) => l,
            Err(e @ store::StoreError::NotEnoughMesos { .. }) => {
                return self.refuse_cash_shop(
                    reason::NOT_ENOUGH_CASH,
                    format!("{what} - the purchase was refused and NOTHING changed: {e}"),
                )
            }
            Err(e @ store::StoreError::StorageFull { .. }) => {
                return self.refuse_cash_shop(
                    reason::TOO_MANY_CASH_ITEMS,
                    format!("{what} - the LOCKER is full; NOTHING changed: {e}"),
                )
            }
            Err(e) => {
                return self.refuse_cash_shop(
                    reason::UNKNOWN_ERROR,
                    format!(
                        "{what} - a SERVER error, not the wallet; NOTHING changed: {e}. The \
                         client will say 'unknown error', which is the truth here"
                    ),
                )
            }
        };

        // The serial the client will name this item by in every later move or delete.
        //
        // **Derived, not stored, and that is a limitation worth stating.** It must be non-zero
        // and must never be `-1` (`FUN_140D75850` drops that), and it must not collide in the
        // stage's locker map. `(account, slot)` satisfies all three and survives a relog - but
        // it changes if the item ever changes slot, so the moment `0x0A`/`0x0B` are built this
        // wants a real column on `cash_locker` instead.
        let serial = (account_id as u64) << 32 | u64::from(placed.slot);
        let record =
            net::cashshop::cash_item_record(serial, row.item_id, row.sn, row.count.max(1));

        let mut out = vec![Reply {
            opcode: net::cashshop::CASH_SHOP_RESULT,
            body: net::cashshop::cash_shop_item_to_locker(&record),
            what: format!(
                "CashShopResult 0x0C BOUGHT -> CASH INVENTORY: {}x {} ({}) for {} LP, locker slot {}, serial {serial:#x}. NOT 0x19 - that is the reply to a locker->bag MOVE, and putting a purchase through it is what put the owner's coupon in the Item Inventory. {what}",
                row.count, row.name, row.item_id, row.price, placed.slot
            ),
        }];
        // **The wallet, second, and it does three things.** It writes [stage+0x74] = 0
        // unconditionally at 0x140D736DC, then sees the buy's [stage+0x120] == 1, zeroes it,
        // and re-enters the buy builder on its COMPLETION path - which is what fetches string
        // 590, "You have successfully made the purchase." So the success message the owner found
        // missing is the client's own, and it was missing because 0x19 had already cleared
        // [stage+0x120] and the re-entry never happened.
        out.extend(self.cash_wallet_reply(account_id, "the debited balance, AFTER the 0x0C"));
        out
    }

    /// One `0x05AE` sub-op `0x3D`: refuse a **queued** operation without emptying the queue.
    ///
    /// The reason is a `u16` here, not the `u8` [`Session::refuse_cash_shop`] sends.
    fn refuse_cash_shop_queue(&self, reason: u16, why: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::cashshop::CASH_SHOP_RESULT,
            body: net::cashshop::cash_shop_queue_refusal(reason),
            what: format!(
                "CashShopResult: refusing with sub-op 0x3D reason 0x{reason:04X}. {why}"
            ),
        }]
    }

    /// One `0x05AE`: cancel the pending purchase, clear the in-flight latch, show `reason`,
    /// and leave the player in the shop. See [`net::cashshop::RESULT_CANCEL_AND_STAY`] for
    /// why it is sub-op `0x1A` and not the `0x1E` the research file recommended.
    fn refuse_cash_shop(&self, reason: u8, why: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::cashshop::CASH_SHOP_RESULT,
            body: net::cashshop::cash_shop_refusal(reason),
            what: format!(
                "CashShopResult: refusing with sub-op 0x1A reason 0x{reason:02X}. {why}"
            ),
        }]
    }
}
