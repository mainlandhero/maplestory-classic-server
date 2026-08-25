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
//! <- 0x00D1  Exit, EMPTY body      ->  0x01A0 SetField, characterData = 1
//! ```
//!
//! # `0x00D1` means two different things and the length is the only thing that separates them
//!
//! A portal walk sends **35 bytes**; the cash shop's Exit button sends **none**. Getting that
//! split backwards would make every portal in the game try to leave a cash shop, so the
//! dispatcher checks the length and this module owns only the empty case.
//!
//! # None of this has ever been on a wire
//!
//! `0x01A0` is the only member of the stage block with a live confirmation. The `0x01A3`
//! number is **[D]** from three independent discriminators, not **[L]**, and if the shop draws
//! nothing that number is the first thing to doubt. Said here rather than discovered later.

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
                 opcode NUMBER is [D] from three discriminators, not [L] - nothing in this block \
                 but 0x01A0 has ever been confirmed on a wire. It also clears ctx+0x2330 itself, \
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
        let (here, portal) = (chr.map_id, chr.portal);
        self.go_to_map(&mut chr, here, portal, "leaving the Cash Shop".to_string())
    }
}

impl Session {
    /// `0x03E1` - **a click inside the shop**, and today every one of them is refused.
    ///
    /// # Why a refusal rather than a purchase
    ///
    /// The store side of a purchase is built and tested (`store::buy_cash_item` debits and
    /// places in one transaction), and this handler can price a real sale row. What is
    /// missing is the packet that tells the client a purchase **succeeded** - and there is
    /// no way to fake it:
    ///
    /// * `0x05AE`'s live sub-ops were enumerated from the client's own index and jump tables.
    ///   Every arm that clears the in-flight latch also calls `FUN_140D7C7F0`, which puts a
    ///   message on screen. **There is no silent one** - `reason = 0` is out of range and
    ///   lands on string 661, the generic error (`0x140D7C818 cmp edx, 0x7e / ja`).
    /// * answering with `0x05AD` - the wallet - would clear the latch *and* re-trigger the
    ///   purchase, because the `0x05AD` arm calls the buy builder when `[stage+0x120] == 1`
    ///   and the buy builder is what set it. That is a farming loop, not a purchase.
    /// * the remaining candidate, a `0x05AE` carrying a cash-item record, needs
    ///   `FUN_1402D0950`'s **71-byte** record, whose field *meanings* past the serial are
    ///   explicitly not established. `CLAUDE.md` records two client deaths this month from
    ///   shipping a packet built on a read count that turned out to be short.
    ///
    /// So: take nothing, say no, and keep the shop alive. **Every effect hangs off the
    /// transition** - the Heena quest rule - and a purchase the client is never told about is
    /// not a transition. `!buy <sn>` performs the same sale from the field, where the result
    /// can be reported with packets that have been on a wire.
    ///
    /// # What this run is actually for
    ///
    /// No real `0x03E1` has ever been captured, so the buy payload's layout is known only
    /// from its builder. Rather than pick an offset for the serial, every offset is read and
    /// checked against the client's own sale list - see
    /// [`crate::commodity::CommodityTable::identify_serial`]. One click puts the answer in
    /// `world.log`, and the instrument cannot invent one: it can only return an SN that is
    /// really in `Commodity.img`.
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

        if !action.is_buy() {
            let known = action.sub_op == net::cashshop::ACTION_GIFT
                || net::cashshop::ACTION_ON_SERIAL.contains(&action.sub_op)
                || action.sub_op == net::cashshop::ACTION_ONE_ID;
            return self.refuse_cash_shop(
                reason::UNKNOWN_ERROR,
                format!(
                    "0x03E1 sub-op 0x{:02X} ({}), {} byte payload {:02x?} - not built. \
                     Refused rather than ignored",
                    action.sub_op,
                    if known { "a reachable builder" } else { "NOT one of the six reachable builders" },
                    action.rest.len(),
                    action.rest
                ),
            );
        }

        let Some(claimed) = self.claimed() else { return Vec::new() };
        let account_id = claimed.account_id;
        let nx = self.store.cash_wallet(account_id).unwrap_or_default().nx;

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
                     {} NX, {} days, on sale {}. Wallet holds {nx} NX",
                    row.count, row.name, row.item_id, row.price, row.period_days, row.on_sale
                );
                if !row.on_sale {
                    return self.refuse_cash_shop(reason::SOLD_OUT, format!("{what} - NOT on sale"));
                }
                if nx < row.price {
                    return self.refuse_cash_shop(
                        reason::NOT_ENOUGH_CASH,
                        format!("{what} - CANNOT AFFORD IT. `!nx {}` would cover it", row.price),
                    );
                }
                self.refuse_cash_shop(
                    reason::UNKNOWN_ERROR,
                    format!(
                        "{what} - AFFORDABLE, AND STILL REFUSED. Nothing was debited: this \
                         server has no way to tell the client a purchase succeeded, so taking \
                         the price would leave an error on screen and the money gone. \
                         `!buy {sn}` performs exactly this sale from the field"
                    ),
                )
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
