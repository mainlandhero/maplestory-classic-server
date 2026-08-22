//! Mr. Kim's storage box, on the wire.
//!
//! The owner, 2026-08-22: *"Mr. Kim the storage keeper does not open the storage UI."*
//!
//! # Built is not wired, for days
//!
//! `crates/store/src/storage.rs` has had `storage`, `storage_slot`, `storage_deposit`,
//! `storage_withdraw`, `store_item` and `take_item` since before this file existed, backed by
//! real `storage` and `storage_item` tables. **Every one of them worked and none of them had
//! a caller.** `grep -rn storage crates/world crates/net` found nothing at all. On screen that
//! is indistinguishable from a feature nobody has started, which is exactly the failure mode
//! `CLAUDE.md`'s "Built is not wired" section exists for.
//!
//! What was missing was two opcodes, and `research/storage.md` found them without a client
//! run: `0x0572` out and `0x00F6` in.
//!
//! # Always answer, and here the latch has a name
//!
//! Every `0x00F6` sets `dlg+0x334` in the client, and **only an inbound `0x0572` clears it**.
//! A refusal that sends nothing leaves the storage window open with every button dead until
//! the player closes it by hand - so every path out of [`Session::on_storage_request`] emits
//! a `0x0572`, including the ones that change nothing.
//!
//! # The box is per ACCOUNT, and that is the one thing the client cannot confirm
//!
//! Nothing on the wire in either direction carries an owner - not the open packet, not any
//! request. `crates/store` keys on `account_id`, and `research/storage.md` §9 keeps that on
//! The owner's own instruction plus one weak reading (a refusal string names the *account*). It is
//! marked **[I]** there and it is marked [I] here: two characters on one account sharing a box
//! is the behaviour this ships, and one launch with two characters would settle it.

use super::*;

impl Session {
    /// A storage keeper was clicked. `None` if `template` is not one.
    ///
    /// Sits beside `open_shop_for` in `on_npc_click` and before the conversation fallback,
    /// for the same reason: a keeper opens a window instead of talking, and Mr. Kim has no
    /// `d0` line to fall back to anyway - which is precisely why clicking them did nothing
    /// visible rather than doing something wrong.
    pub(super) fn open_storage_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let fee = net::storage::storage_fee(template)?;
        let claimed = self.claimed()?;
        let account_id = claimed.account_id;
        let boxx = match self.store.storage(account_id) {
            Ok(b) => b,
            Err(e) => {
                return Some(self.notice(format!("Your storage could not be opened: {e}")));
            }
        };
        let (slots, mesos, count) = (boxx.slots, boxx.mesos, boxx.items.len());
        let per_type = self.storage_blobs(&boxx);
        Some(vec![Reply {
            opcode: net::storage::STORAGE_RESULT,
            body: net::storage::open_storage(
                template,
                slots.min(u16::from(u8::MAX)) as u8,
                u64::from(mesos),
                &per_type,
            ),
            what: format!(
                "StorageResult OPEN at NPC template {template} (deposit fee {fee}, withdrawing is free on all ten keepers) for account {account_id}: {count} item(s), {mesos} mesos, {slots} slots. The template id, NOT the object id - the client reads Npc.wz from it to find the fee"
            ),
        }])
    }

    /// `0x00F6` - take out, put in, sort, move mesos, close.
    ///
    /// **Every branch answers.** See the module docs: the client latches on send and only a
    /// `0x0572` releases it.
    pub(super) fn on_storage_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(claimed) = self.claimed() else { return Vec::new() };
        let account_id = claimed.account_id;
        let Some(req) = net::storage::StorageRequest::parse(body) else {
            // A body we cannot read is our problem, not a reason to wedge the window.
            return self.storage_refusal(
                net::storage::RESULT_TRUNK_REFRESH,
                account_id,
                format!("unreadable 0x00F6 body {body:02x?} - answered anyway to clear the latch"),
            );
        };
        match req {
            net::storage::StorageRequest::Close => Vec::new(),
            net::storage::StorageRequest::Sort => self.storage_refusal(
                net::storage::RESULT_TRUNK_REFRESH,
                account_id,
                "sort: the box is re-sent unchanged, which is a legal no-op".to_string(),
            ),
            net::storage::StorageRequest::Mesos { amount } => {
                self.storage_mesos(account_id, amount)
            }
            // Item movement is not built yet, and it says so rather than silently doing
            // nothing: the window stays usable and the log names the request.
            other => self.storage_refusal(
                net::storage::RESULT_TRUNK_REFRESH,
                account_id,
                format!("{other:?} is not implemented yet - the box is re-sent unchanged"),
            ),
        }
    }

    /// Move mesos between the purse and the box.
    ///
    /// # The two sign conventions are opposite, and this is the only place they meet
    ///
    /// **On the wire** (`0x00F6` mode 7) a *positive* amount **withdraws** from the box into
    /// the purse. **In the store** `move_storage_mesos` takes a positive amount to mean
    /// **deposit** - *"Positive deposits, negative withdraws"*, straight from its doc block.
    ///
    /// So the wire value is **negated** on the way in. Getting this backwards would not
    /// error, would not crash, and would not look wrong in any log: it would quietly move
    /// money the other way, which is the worst shape a bug can have. There is a test.
    ///
    /// The store does both halves in one transaction, which is the property that matters -
    /// mesos that leave one side and never arrive at the other are the same duplication bug
    /// as an item in two containers.
    fn storage_mesos(&mut self, account_id: i64, amount: i64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let direction = if amount >= 0 { "withdraw" } else { "deposit" };
        match self.store.move_storage_mesos(account_id, chr.id, -amount) {
            Ok((purse, boxed)) => {
                let mut out = self.storage_refusal(
                    net::storage::RESULT_TRUNK_REFRESH,
                    account_id,
                    format!(
                        "{direction} {}: purse now {purse}, box now {boxed}. The wire's sign is INVERTED from the store's - positive on 0x00F6 means withdraw",
                        amount.abs()
                    ),
                );
                // The stat block has no meso field at all, so `0x007C` bit 18 is the only way
                // this client is ever told a balance.
                out.extend(self.meso_reply(chr.id));
                out
            }
            Err(store::StoreError::NotEnoughMesos { have, want }) => self.storage_refusal(
                net::storage::RESULT_NOT_ENOUGH_MESOS,
                account_id,
                format!("tried to {direction} {want} with {have} available"),
            ),
            Err(e) => self.storage_refusal(
                net::storage::RESULT_TRUNK_REFRESH,
                account_id,
                format!("meso move failed: {e}"),
            ),
        }
    }

    /// Send the box as it currently stands, with `mode` on the front.
    ///
    /// Used for refusals and for no-ops alike: in both cases the right answer is "here is the
    /// box, unchanged", and the mode byte is what tells the client which message to show.
    fn storage_refusal(&mut self, mode: u8, account_id: i64, why: String) -> Vec<Reply> {
        // A box that cannot be read still has to produce a packet, or the latch never
        // clears. An empty one is the honest fallback: it says "nothing in here" rather than
        // inventing contents, and the `what` line below names the mode so the log says which
        // path produced it.
        let boxx = match self.store.storage(account_id) {
            Ok(b) => b,
            Err(_) => store::StorageBox { mesos: 0, slots: 0, items: Vec::new() },
        };
        let (slots, mesos) = (boxx.slots, boxx.mesos);
        let per_type = self.storage_blobs(&boxx);
        vec![Reply {
            opcode: net::storage::STORAGE_RESULT,
            body: net::storage::storage_refresh(
                mode,
                slots.min(u16::from(u8::MAX)) as u8,
                u64::from(mesos),
                &per_type,
            ),
            what: format!("StorageResult mode {mode}: {why}. Every 0x00F6 is answered - the client latches dlg+0x334 on send and only a 0x0572 clears it"),
        }]
    }

    /// The six per-type item lists, encoded with the **bag's** encoders.
    ///
    /// Storage and the bag are decoded by the same client function, so there is exactly one
    /// item encoding in this server and this is not it - `equipped_item` and `bundle_item`
    /// already emit the leading type byte.
    fn storage_blobs(
        &self,
        boxx: &store::StorageBox,
    ) -> [Vec<Vec<u8>>; net::storage::INVENTORY_TYPES] {
        let mut out: [Vec<Vec<u8>>; net::storage::INVENTORY_TYPES] = Default::default();
        for it in &boxx.items {
            let Some(inv) = store::InventoryType::for_item(it.item.item_id) else { continue };
            let idx = inv.index();
            if idx >= out.len() {
                continue;
            }
            let blob = match it.item.kind {
                store::ItemKind::Equip(stats) => {
                    let stats = stats.unwrap_or_else(|| self.template_stats(it.item.item_id));
                    net::opcode::equipped_item(it.item.item_id, &stats)
                }
                store::ItemKind::Bundle { quantity } => net::bag::bundle_item(
                    it.item.item_id,
                    quantity,
                    0,
                    &[0u8; net::bag::BUNDLE_OWNER_LEN],
                ),
            };
            out[idx].push(blob);
        }
        out
    }
}
