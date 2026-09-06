//! The **classic** NPC shop counter - `0x055D`, the one whose art this client actually has.
//!
//! Full working, every address and every control: **`research/classic-shop-opcode.md`** (which
//! opcode) and **`research/classic-shop-rows.md`** (the row, the price, the gates, the result
//! and the request). This module is §9 of the second file, built.
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ or a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred.
//!
//! # Why this exists next to [`crate::shop`] rather than replacing it
//!
//! **This client has two entirely separate shop windows on two adjacent opcode pairs.** [L]
//!
//! | | window | art | open | result |
//! |---|---|---|---|---|
//! | classic | `UI/UIShop.img/Shop` | **present** in `UI_000.wz` | **`0x055D`** | `0x055E` |
//! | Shop2 | `UI/UIWindow2.img/Shop2` | **absent from this client** | `0x0560` | `0x055F` |
//!
//! [`crate::shop`] builds `0x0560`. **It killed the client twice**, and the reason was never
//! our bytes: the Shop2 constructor loads `UI/UIWindow2.img/Shop2/backgrnd`, that image is not
//! in this client's WZ, the ResMan COM call fails, `_com_issue_errorex` throws and the unwinder
//! faults - *before a single row byte is read*, which is why `--shop-rows 1` changed nothing.
//!
//! [`crate::shop`] is kept rather than deleted because its `ShopRow` policy helpers, its
//! request parser and its result table are all still correct for what they describe, and
//! because the two-window fact is worth being able to point at.
//!
//! # The three sentences that decide whether a shop is on screen at all
//!
//! All three are failures that produce **no message anywhere** - not on screen, not in
//! `world.log`, not in the hook log. They read exactly like "the shop still does not work".
//!
//! * **[`SALE_END_PERMANENT`] is not optional.** `row+0xa4` is compared against the wall clock
//!   with no sentinel, so `0` is "this sale ended in 1601" and the row silently vanishes. A
//!   shop sent with zeros here opens completely empty.
//! * **[`ClassicShopRow::max_per_purchase`] must never be `0`.** Every purchase then fails with
//!   no message - the same trap Shop2's row offset 29 has. `ItemData::slot_max` is zero for
//!   2495 of the 2785 rows in `gm-handbook/itemdata.txt`, so it cannot be copied straight
//!   through.
//! * **`point_cost` and `required_item_count` must be `0` for a meso purchase**, or the buy is
//!   refused silently to us.
//!
//! # The one thing here that can crash rather than disappoint
//!
//! A row carries its Buy Back flag **after** every gate that can drop it. If a flagged row is
//! dropped - by `+0xf0`, by an expired `+0xa4`, by a level or quest gate - the trailing
//! item-slot byte is never consumed and **every following row is parsed one byte out of
//! phase**. [`ClassicShopRow::buy_back`] therefore sets no gates at all, and
//! [`classic_open_shop`] asserts that in a debug build.

use crate::packet::PacketWriter;

// ---------------------------------------------------------------------------------------
// Opcodes
// ---------------------------------------------------------------------------------------

/// `0x055D` - open the classic shop counter. **[L]**, `FUN_141fa66c0`'s arm at `0x141fa767b`.
///
/// **The handler has a modal guard.** If the shop singleton at `0x143AA8520` is not null the
/// packet is *discarded* and `FUN_141fb94c0` runs instead, so sending this twice does nothing,
/// and sending it while a `0x055B` script box is on screen does nothing. The shop branch must
/// therefore **replace** an NPC's dialogue rather than follow it.
pub const CLASSIC_OPEN_SHOP: u16 = 0x055D;

/// `0x055E` - the classic shop's result packet. **[L]**
///
/// A `u8` type, then a body that depends on it. Type `0` is the success arm and carries
/// `u8, u32, u32`; type `10` is a list refresh whose body is the head-less form (see
/// [`classic_shop_refresh`]).
pub const CLASSIC_SHOP_RESULT: u16 = 0x055E;

/// `0x00F5` - everything the classic shop window sends. **[L]**, `research/classic-shop-rows.md`
/// §6. `crate::shop::CLIENT_SHOP_REQUEST` (`0x0104`) is the Shop2 equivalent and is a different
/// opcode with a different body.
pub const CLIENT_CLASSIC_SHOP_REQUEST: u16 = 0x00F5;

// ---------------------------------------------------------------------------------------
// The two time constants, which are the client's own
// ---------------------------------------------------------------------------------------

/// `row+0xa4`, the sale end. **The single most dangerous zero in this module.**
///
/// `150842304000000000` - the client's own "permanent" file time. **[L]** It is compared
/// against the wall clock with **no sentinel value**: there is no encoding of "no end date", so
/// a `0` here is a sale that ended in 1601 and the row is dropped before it is ever drawn.
///
/// The failure is completely silent. The shop opens, it is empty, and nothing anywhere says
/// why - which is why this constant is named rather than written as a literal at the call site.
pub const SALE_END_PERMANENT: u64 = 150_842_304_000_000_000;

/// `row+0x9c`, the sale start. `94354848000000000`, the client's own zero time. **[L]**
///
/// Unlike [`SALE_END_PERMANENT`] this one is compared the other way round, so a `0` would also
/// pass - but the pair is sent as the client's own values so that a capture of ours is
/// byte-comparable with a capture of a real server's.
pub const SALE_START_ZERO_TIME: u64 = 94_354_848_000_000_000;

// ---------------------------------------------------------------------------------------
// The row
// ---------------------------------------------------------------------------------------

/// The wire width of one ordinary row: **157 bytes**. **[D]**, from the field census in
/// `research/classic-shop-rows.md` §3, and asserted against the golden vector in the tests.
///
/// A buy-back row is **158**: it carries one further `u8`, the item-slot type.
pub const CLASSIC_ROW_LEN: usize = 157;

/// The extra bytes a **rechargeable** row carries, and the reason the client killed itself
/// on every shop that stocked one.
///
/// At `0x1404ba57f` the row decoder branches **on the item id**: `2070000..2079999` and
/// `2330000..2339999` read **8 raw bytes** into `row+0x40`; everything else reads an `i16`
/// into `row+0x1c`. This builder wrote the `i16` unconditionally, so every star row was
/// **6 bytes short** and the client read off the end of the packet.
///
/// What that produced: `0x009E` with exception class 1 and reason `0x26` - a literal
/// `MOV EDX,0x26` inside the read primitives at `0x1406e8bf2`/`0x1406e8c91`, meaning **"not
/// enough bytes left"** - then an escaped exception and `0xC0000005` in a destructor during
/// unwind. Size was never involved: Serabi's 138-row, 21 687-byte shop was accepted because
/// they stock no star, and Mina's 5 673 bytes was refused because they stock Subi.
///
/// **13 of the 39 shops in `data/shops.txt` stock one.**
const RECHARGEABLE_ROW_EXTRA: usize = 6;

/// The extra byte a buy-back row carries. See [`CLASSIC_ROW_LEN`].
pub const BUY_BACK_ROW_EXTRA: usize = 1;

/// The fixed head: `u32 a, u8 hasB, u32 c, u32 d, u16 nameLen, u32 e, u16 nRows` = 21 bytes.
/// **[D]** The name is an empty string, so its length prefix is the whole of it.
pub const CLASSIC_HEAD_LEN: usize = 21;

/// One row of a classic shop counter.
///
/// # Every field that is not here is a zero, and that is a decision
///
/// `research/classic-shop-rows.md` §0 lists **ten fields** that the client reads and stores and
/// for which *no consumer was traced* on the plain meso-purchase path - `+0x04`, `+0x30`,
/// `+0xcc`, `+0xd0`, `+0xd8`, `+0xdc`, `+0xe0`, `+0xe8`, `+0xec`, `+0xf8`. They are sent as
/// zero. That is **[L]** that they are read and **not established** what they mean, and the
/// named blind spot is that `tools/fieldrefs.py` matches `[reg + disp]` only, so a use through
/// a copy of the struct under another base register is invisible to it - and `FUN_141fbc610`
/// copies the whole 0x130-byte row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassicShopRow {
    /// `row+0x08`. The item this row sells.
    pub item_id: u32,
    /// `row+0x38`, **the price**, in mesos. A `u64` on the wire.
    ///
    /// This is the field a shop cannot be wrong about, and it is the one
    /// `classic-shop-opcode.md` explicitly could not identify. `classic-shop-rows.md` §2 named
    /// it. **[L]**
    pub price: u64,
    /// `row+0x1c`. How many the row hands over per unit bought - a stack size, not a limit.
    pub bundle_quantity: i16,
    /// `row+0x10c`. **Never `0`**: a zero makes every purchase fail with no message at all.
    /// See the module header.
    pub max_per_purchase: i16,
    /// `row+0x0c`. Remaining stock. Irrelevant while `stock_limit` is `0`, which is how every
    /// row this server sends is built.
    pub remaining_stock: u32,
    /// The Sell-tab flag, the first of the two trailing bytes.
    pub sell: bool,
    /// The Buy Back flag, the second trailing byte. **A flagged row must carry no gates** - see
    /// the module header, this is the desync.
    pub buy_back: bool,
    /// `row+0x40`, the **recharge unit price** in thousandths of a meso - written as an IEEE
    /// double, and **only for a rechargeable id** (`crate::bag::bundle_has_serial`); every
    /// other row has no such field. `0` keeps the Recharge arm unreachable for the row:
    /// `FUN_141fb9240` refuses when the double is `0.0` (`research/classic-shop-rows.md`
    /// §3 row 41a). Thousandths rather than a float so the row stays `Eq`.
    pub unit_price_milli: u32,
}

impl ClassicShopRow {
    /// An ordinary Buy-tab row: no gates, unlimited stock, sold for mesos.
    pub fn buy(item_id: u32, price: u64, max_per_purchase: i16) -> Self {
        Self {
            item_id,
            price,
            bundle_quantity: 1,
            // **Clamped away from zero here rather than at the call site.** A zero reaches the
            // client as "every purchase fails silently", and a caller that computed it from
            // `ItemData::slot_max` has no way to know: that column is zero for 2495 of 2785
            // rows.
            max_per_purchase: max_per_purchase.max(1),
            remaining_stock: 0,
            sell: false,
            buy_back: false,
            unit_price_milli: 0,
        }
    }

    /// The same row, rechargeable at `unit_price_milli` thousandths of a meso per unit.
    ///
    /// Only meaningful on a `207xxxx`/`233xxxx` id - on anything else the field is not on
    /// the wire and the value is dropped, which is the right outcome for a data error.
    pub fn with_unit_price(self, unit_price_milli: u32) -> Self {
        Self { unit_price_milli, ..self }
    }

    /// The unit price as the client reads it, or `None` when this row carries no such field.
    pub fn unit_price(&self) -> Option<f64> {
        crate::bag::bundle_has_serial(self.item_id)
            .then(|| f64::from(self.unit_price_milli) / 1000.0)
    }

    /// A Sell-tab row - the price the NPC pays.
    pub fn sell(item_id: u32, price: u64, max_per_purchase: i16) -> Self {
        Self { sell: true, ..Self::buy(item_id, price, max_per_purchase) }
    }

    /// A Buy Back row: something the player sold, offered back at `price`.
    ///
    /// `stack` becomes both the remaining stock and the per-purchase cap, so the player cannot
    /// buy back more than they sold.
    ///
    /// **This row deliberately carries no gates**, and [`classic_open_shop`] re-checks that.
    /// A gated buy-back row that gets dropped takes its trailing item-slot byte with it and
    /// desynchronises every row after it - the one failure here that crashes rather than
    /// disappoints.
    pub fn buy_back(item_id: u32, price: u64, stack: i16) -> Self {
        Self {
            item_id,
            price,
            bundle_quantity: 1,
            max_per_purchase: stack.max(1),
            remaining_stock: u32::try_from(stack.max(1)).unwrap_or(1),
            sell: false,
            buy_back: true,
            unit_price_milli: 0,
        }
    }

    /// This row's width on the wire. See [`CLASSIC_ROW_LEN`].
    pub fn wire_len(&self) -> usize {
        CLASSIC_ROW_LEN
            + if self.buy_back { BUY_BACK_ROW_EXTRA } else { 0 }
            + if crate::bag::bundle_has_serial(self.item_id) { RECHARGEABLE_ROW_EXTRA } else { 0 }
    }

    /// Write it. Field order is `research/classic-shop-rows.md` §3, which two instruments agree
    /// on - the listing at `0x141f9f5e0` and the decompiler.
    fn write(&self, w: &mut PacketWriter) {
        w.u32(self.remaining_stock); // +0x0c
        w.u32(0); // +0x04   read, stored, no traced consumer
        w.u32(self.item_id); // +0x08
        w.u32(0); // +0x20   buy-tab tag - KEEP 0 or the Sell tab vanishes
        w.u32(0); // +0x10   stock limit; 0 = unlimited
        w.u32(0); // +0x24
        w.u64(0); // +0x28   purchased-item expiry; 0 reads as "none"
        w.u32(0); // +0x30
        w.u64(self.price); // +0x38   *** THE PRICE ***
        w.u32(0); // +0x48   required item id
        w.u32(0); // +0x4c   required item count - MUST be 0 for a meso purchase
        w.u32(0); // +0x50   pointShop type
        w.u32(0); // +0x54   pointShopWsr type
        w.u32(0); // +0x58   point cost - MUST be 0 or the purchase is refused
        w.u8(0); // optional block A: OFF
        w.u32(0); // +0x78
        w.u32(0); // +0x7c   purchase limit; 0 = unlimited
        w.u8(0); // optional block B: kind 0, nothing follows
        w.u32(0); // +0x90   signed level gate
        w.u16(0); // +0x94   min level
        w.u16(0); // +0x98   max level
        w.u8(0); // +0xf0    *** MUST BE 0 or the row is destroyed before block C ***
        w.u64(SALE_START_ZERO_TIME); // +0x9c
        w.u64(SALE_END_PERMANENT); // +0xa4  *** or the row VANISHES ***
        w.u32(0); // +0xac   quest id
        w.u16(0); // +0xb0   quest state
        w.u8(0); // +0xb4
        w.u32(0); // +0xb8
        w.str(""); // +0xc0  counter name
        w.u32(0); // +0xc8   counter threshold; 0 disables the test entirely
        w.u32(0); // +0xcc
        w.str(""); // +0xd0
        w.u32(0); // +0xd8
        w.u32(0); // +0xdc
        w.str(""); // +0xe0
        w.u32(0); // +0xe8
        w.u8(0); // +0xec
        w.str(""); // +0xf8
        w.u32(0); // +0x100
        w.u32(0); // +0x104  required citizenship type
        w.u32(0); // +0x108  required citizenship grade
        // **The client picks the width of this field from the item id, so we must too.**
        // `0x1404ba57f`: a rechargeable id takes 8 raw bytes into `row+0x40`; anything else
        // takes this `i16` into `row+0x1c`. Writing the `i16` for a star left the client 6
        // bytes short and it read past the end - reason `0x26`, "not enough bytes left".
        //
        // The 8 bytes are an IEEE-754 double, the recharge price per unit: `FUN_141fb9240`
        // refuses to recharge when it is `0.0`, so a row built without `with_unit_price`
        // keeps the Recharge arm unreachable - which was every row until 2026-09-06, when
        // The owner asked for stars to be rechargeable at the general stores. The same two id
        // ranges are already read off a different site in `crate::bag`, so they are shared
        // rather than restated. Little-endian bits of the double, which is what `movsd`
        // at `0x141faf1e2` loads straight out of the row.
        if let Some(unit) = self.unit_price() {
            w.u64(unit.to_bits()); // +0x40  double unitPrice
        } else {
            w.i16(self.bundle_quantity); // +0x1c
        }
        w.i16(self.max_per_purchase); // +0x10c  *** NEVER 0 ***
        w.u8(u8::from(self.sell));
        w.u8(u8::from(self.buy_back));
        if self.buy_back {
            // **Item-slot type 0 consumes exactly one byte and decodes nothing.** 1/2/3 would
            // be a real `GW_ItemSlot` into `row+0x120`. The row renderer null-checks that slot
            // (`0x141faef78`) so `0` is safe; what it draws instead is **[I]** - the item
            // template, from the shape.
            w.u8(0);
        }
    }
}

// ---------------------------------------------------------------------------------------
// The packets
// ---------------------------------------------------------------------------------------

/// The head-less body: everything `FUN_141f9f5e0` reads, i.e. the open body **without** the
/// leading `u32 a` and `u8 hasB`. Shared by [`classic_open_shop`] and [`classic_shop_refresh`].
fn list_body(w: &mut PacketWriter, rows: &[ClassicShopRow]) {
    w.u32(0); // c - no traced individual consumer; only a 16-byte bulk movups reads it
    w.u32(0); // d - echoed back in every request
    w.str(""); // the counter's name, copied with a 0x1f-character limit
    w.u32(0); // e - echoed back in every request
    w.u16(u16::try_from(rows.len()).unwrap_or(u16::MAX));
    for row in rows {
        row.write(w);
    }
}

/// `0x055D` - open the counter. `npc_template_id` is the window factory's only argument.
///
/// That reading is **[I]**, not [L]: `a` is passed straight to `FUN_141f9f400`, and "the NPC
/// template" is the obvious meaning rather than a traced one.
///
/// # Panics
///
/// Debug builds only, and on a programming error rather than on data: a buy-back row is
/// checked to carry no gate, because a dropped flagged row desynchronises every row after it.
/// Since [`ClassicShopRow::buy_back`] is the only way to set the flag and it sets no gates,
/// this cannot fire from `data/shops.txt` content.
pub fn classic_open_shop(npc_template_id: u32, rows: &[ClassicShopRow]) -> Vec<u8> {
    debug_assert!(
        rows.iter().all(|r| !r.buy_back || r.max_per_purchase > 0),
        "a buy-back row must be ungated; see classic-shop-rows.md 9.3"
    );
    let mut w = PacketWriter::new();
    w.u32(npc_template_id); // a
    w.u8(0); // hasB - when non-zero a further u32 follows, into window+0x1658
    list_body(&mut w, rows);
    w.into_vec()
}

/// `0x055E` **type 10** - replace the list without reopening the window.
///
/// Sent after a sell (so the item appears in Buy Back) and after a buy back (so it leaves).
/// The body is the head-less form, which is why [`list_body`] exists.
pub fn classic_shop_refresh(rows: &[ClassicShopRow]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(RESULT_REFRESH_LIST);
    list_body(&mut w, rows);
    w.into_vec()
}

/// `0x055E` type `10`, the list refresh. See [`classic_shop_refresh`].
pub const RESULT_REFRESH_LIST: u8 = 10;

/// `0x055E` type `0`, the success arm: `u8, u32 itemId, u32 remainingStock`. **[L]**
pub const RESULT_SUCCESS: u8 = 0;

/// A `0x055E` type-0 success.
///
/// **This packet moves nothing by itself.** It has to be followed by the `0x0070` inventory
/// delta and the `0x007C` meso change, exactly as `research/npc-shop.md` §3.2 records for
/// Shop2 - a result alone leaves the item and the mesos where they were while the window
/// cheerfully says the purchase worked.
pub fn classic_shop_success(item_id: u32, remaining_stock: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(RESULT_SUCCESS);
    w.u8(0);
    w.u32(item_id);
    w.u32(remaining_stock);
    w.into_vec()
}

/// A `0x055E` refusal. The type byte selects the client's own message text.
///
/// **Answer every `0x00F5` with one of these.** `shopUI+0x4b0` latches on send, and only a
/// result clears it: an unanswered buy leaves the window alive but every further click a
/// silent no-op, which reads as the shop half-working. Closing and re-clicking the NPC
/// recovers it; another `0x055D` alone does not, because of the modal guard.
pub fn classic_shop_refused(result_type: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(result_type);
    w.into_vec()
}

/// The refusal this server sends when it will not complete a purchase.
///
/// **`2` is "you do not have enough mesos"** in the `0x055E` table (§8). It is used as the
/// generic refusal deliberately: every refusal this server actually produces is either that or
/// a full inventory, and a wrong-but-plausible message beats the silent latch.
pub const RESULT_NOT_ENOUGH_MESOS: u8 = 2;

/// `0x055E` type `3` - the inventory is full. **[L]**, from the message table.
pub const RESULT_INVENTORY_FULL: u8 = 3;

// ---------------------------------------------------------------------------------------
// The request
// ---------------------------------------------------------------------------------------

/// What the classic shop window asked for. **[L]**, `research/classic-shop-rows.md` §6.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicShopRequest {
    /// `u8 0`: buy, or buy back when `row_index` names a row that was flagged.
    Buy { row_index: u16, item_id: u32, quantity: u16 },
    /// `u8 1`: sell from an inventory slot.
    Sell { inventory_slot: u16, item_id: u32, quantity: u16 },
    /// `u8 2`: recharge. Only reachable if a row carried a non-zero rechargeable double, so a
    /// shop this server builds never sees one.
    Recharge { inventory_slot: u16 },
    /// `u8 3`: the window closed. **Nothing is latched** - drop the state and send nothing.
    Close,
}

/// Parse a `0x00F5`. `None` for a body too short to read, which is answered rather than
/// dropped: see [`classic_shop_refused`].
pub fn parse_classic_shop_request(body: &[u8]) -> Option<ClassicShopRequest> {
    let u16_at = |i: usize| -> Option<u16> {
        Some(u16::from_le_bytes(body.get(i..i + 2)?.try_into().ok()?))
    };
    let u32_at = |i: usize| -> Option<u32> {
        Some(u32::from_le_bytes(body.get(i..i + 4)?.try_into().ok()?))
    };
    match *body.first()? {
        0 => Some(ClassicShopRequest::Buy {
            row_index: u16_at(1)?,
            item_id: u32_at(3)?,
            quantity: u16_at(7)?,
        }),
        1 => Some(ClassicShopRequest::Sell {
            inventory_slot: u16_at(1)?,
            item_id: u32_at(3)?,
            quantity: u16_at(7)?,
        }),
        2 => Some(ClassicShopRequest::Recharge { inventory_slot: u16_at(1)? }),
        3 => Some(ClassicShopRequest::Close),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The golden vector from `research/classic-shop-rows.md` §9.1, byte for byte.**
    ///
    /// This is the whole reason that section wrote the bytes out: an implementation can be
    /// diffed against them rather than re-derived, and a field written in the wrong order or
    /// the wrong width fails here rather than on the owner's machine. A shop packet has killed this
    /// client twice.
    ///
    /// Lucy is NPC template 21. Row one is Red Potion (2000000) at 50 mesos; row two is a Buy
    /// Back entry for the same item at 25 with three in the stack.
    /// Transcribed **programmatically** from the 21-line hex dump in
    /// `research/classic-shop-rows.md` §9.1, not by hand: the first attempt at this constant
    /// was retyped and reflowed, and a mistyped golden vector is worse than no golden vector
    /// because it makes a wrong implementation pass.
    const GOLDEN: &str = concat!(
        "150000000000000000000000000000000000000200000000000000000080841e",
        "0000000000000000000000000000000000000000000000000032000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "00000000000000000040e0fd3b374f01008005bb46e617020000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "000000000000000000000000010064000000030000000000000080841e000000",
        "0000000000000000000000000000000000000000000019000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "00000000000040e0fd3b374f01008005bb46e617020000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "00000000000000000001000300000100",
    );

    fn golden() -> Vec<u8> {
        let hex: String = GOLDEN.chars().filter(|c| !c.is_whitespace()).collect();
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn the_two_item_shop_matches_the_researched_bytes_exactly() {
        let rows = [
            ClassicShopRow::buy(2_000_000, 50, 100),
            ClassicShopRow::buy_back(2_000_000, 25, 3),
        ];
        let built = classic_open_shop(21, &rows);
        let want = golden();
        assert_eq!(
            built.len(),
            want.len(),
            "length: built {} want {} (head {CLASSIC_HEAD_LEN} + {CLASSIC_ROW_LEN} + {})",
            built.len(),
            want.len(),
            CLASSIC_ROW_LEN + BUY_BACK_ROW_EXTRA
        );
        if built != want {
            let at = built.iter().zip(&want).position(|(a, b)| a != b).unwrap();
            panic!(
                "first difference at byte {at}: built {:02x} want {:02x}\n built {:02x?}\n want  {:02x?}",
                built[at],
                want[at],
                &built[at.saturating_sub(8)..(at + 8).min(built.len())],
                &want[at.saturating_sub(8)..(at + 8).min(want.len())],
            );
        }
        assert_eq!(built.len(), 336, "the researched total for this example");
    }

    /// The row width is a census, so a field added or dropped fails here rather than
    /// desynchronising the client's row loop.
    #[test]
    fn a_row_is_157_bytes_and_a_buy_back_row_is_158() {
        let mut w = PacketWriter::new();
        ClassicShopRow::buy(2_000_000, 50, 100).write(&mut w);
        assert_eq!(w.len(), CLASSIC_ROW_LEN);
        assert_eq!(w.len(), 157);

        let mut w = PacketWriter::new();
        ClassicShopRow::buy_back(2_000_000, 25, 3).write(&mut w);
        assert_eq!(w.len(), CLASSIC_ROW_LEN + BUY_BACK_ROW_EXTRA);
        assert_eq!(w.len(), 158);
    }

    /// The head is 21 bytes, and an empty shop is head-only.
    #[test]
    fn the_head_is_twenty_one_bytes() {
        assert_eq!(classic_open_shop(21, &[]).len(), CLASSIC_HEAD_LEN);
    }

    /// **The two silent killers, asserted as bytes on the wire rather than as intent.**
    ///
    /// Both of these produce no message anywhere when wrong. `SALE_END_PERMANENT` at `0` empties
    /// the shop; `max_per_purchase` at `0` makes every purchase fail. Reading them back out of
    /// the built packet is the only check that cannot pass while the wire is wrong.
    #[test]
    fn the_sale_end_and_the_purchase_cap_reach_the_wire_non_zero() {
        let body = classic_open_shop(21, &[ClassicShopRow::buy(2_000_000, 50, 0)]);
        let row = &body[CLASSIC_HEAD_LEN..];

        // +0xa4, the sale end: 83 bytes into the row, after the 8-byte sale start.
        let sale_end = u64::from_le_bytes(row[91..99].try_into().unwrap());
        assert_eq!(sale_end, SALE_END_PERMANENT, "a zero here empties the shop silently");
        assert_ne!(sale_end, 0);

        // +0x10c, the cap: the third and fourth bytes from the end of an ordinary row.
        let cap = i16::from_le_bytes(row[153..155].try_into().unwrap());
        assert_eq!(cap, 1, "a caller's 0 must be clamped, not passed through");
        assert_ne!(cap, 0, "a zero here makes every purchase fail with no message");
    }

    /// The price is a `u64` at the researched offset, and it is the field a shop cannot be
    /// wrong about.
    #[test]
    fn the_price_is_a_u64_at_row_offset_36() {
        let body = classic_open_shop(21, &[ClassicShopRow::buy(2_000_000, 1_234_567, 10)]);
        let row = &body[CLASSIC_HEAD_LEN..];
        assert_eq!(u64::from_le_bytes(row[36..44].try_into().unwrap()), 1_234_567);
        // and the item id is where the golden vector puts it
        assert_eq!(u32::from_le_bytes(row[8..12].try_into().unwrap()), 2_000_000);
    }

    /// A buy-back row must carry no gate, because a dropped flagged row eats the byte that
    /// keeps the row loop in phase.
    #[test]
    fn a_buy_back_row_carries_no_gate() {
        let r = ClassicShopRow::buy_back(2_000_000, 25, 3);
        assert!(r.buy_back);
        assert!(r.max_per_purchase > 0);
        assert_eq!(r.remaining_stock, 3);
        // A zero stack would clamp to one rather than produce an uncappable row.
        assert_eq!(ClassicShopRow::buy_back(2_000_000, 25, 0).max_per_purchase, 1);
    }

    /// The refresh body is the open body without the leading `u32 a` and `u8 hasB`, plus its
    /// own type byte. Asserted as an arithmetic identity so it cannot drift from the builder.
    #[test]
    fn the_refresh_body_is_the_open_body_without_the_five_byte_head() {
        let rows = [ClassicShopRow::buy(2_000_000, 50, 100)];
        let open = classic_open_shop(21, &rows);
        let refresh = classic_shop_refresh(&rows);
        assert_eq!(refresh.len(), open.len() - 5 + 1);
        assert_eq!(refresh[0], RESULT_REFRESH_LIST);
        assert_eq!(&refresh[1..], &open[5..]);
    }

    /// Every request arm parses, and a short body is `None` rather than a panic - it still has
    /// to be answered.
    /// **A rechargeable row is six bytes wider, and that is what killed the client.**
    ///
    /// `0x1404ba57f` branches on the item id: `2070000..2079999` and `2330000..2339999` read
    /// **8 raw bytes** into `row+0x40`, everything else reads an `i16` into `row+0x1c`. This
    /// builder wrote the `i16` for every row, so a star row went out 6 bytes short, the client
    /// read past the end, and answered `0x009E` class 1 reason `0x26` - "not enough bytes
    /// left" - then died in a destructor during unwind.
    ///
    /// The census that settled it: Lucy x3, Flora, Karl and Serabi were all ACCEPTED, Serabi
    /// at 138 rows and 21 687 bytes. Mina and Luna were REFUSED at 5 673 and 7 243. **The only
    /// thing that separates the two groups is a Subi Throwing Star.** Size never mattered.
    #[test]
    fn a_rechargeable_row_is_six_bytes_wider_than_an_ordinary_one() {
        let ordinary = ClassicShopRow::buy(2_000_000, 50, 100); // Red Potion
        let subi = ClassicShopRow::buy(2_070_000, 500, 100); // Subi Throwing Stars
        let dart = ClassicShopRow::buy(2_330_000, 500, 100); // the other range

        assert_eq!(ordinary.wire_len(), CLASSIC_ROW_LEN);
        assert_eq!(subi.wire_len(), CLASSIC_ROW_LEN + 6);
        assert_eq!(dart.wire_len(), CLASSIC_ROW_LEN + 6);

        // The ranges are half-open at the top; the id one past the end is an ordinary row.
        assert_eq!(ClassicShopRow::buy(2_080_000, 1, 1).wire_len(), CLASSIC_ROW_LEN);
        assert_eq!(ClassicShopRow::buy(2_069_999, 1, 1).wire_len(), CLASSIC_ROW_LEN);

        // And the built packet agrees with the declared width - the bug was that these two
        // agreed with each other while disagreeing with the client.
        let body = classic_open_shop(9_000_000, &[ordinary.clone(), subi.clone()]);
        assert_eq!(body.len(), CLASSIC_HEAD_LEN + ordinary.wire_len() + subi.wire_len());
        assert_eq!(body.len(), CLASSIC_HEAD_LEN + 2 * CLASSIC_ROW_LEN + 6);
    }

    /// **A rechargeable row carries its unit price as a little-endian IEEE double**, in the
    /// eight bytes that used to be written as zero. The owner, 2026-09-06: stars should be
    /// rechargeable at the general stores. `0.3` is Subi's `info/unitPrice`, read out of
    /// `Item/Consume/0207.img` the same day.
    ///
    /// The zero-priced row still writes `0.0`, which keeps the client's Recharge arm
    /// unreachable for it - `FUN_141fb9240` tests the double against zero. And a potion
    /// given a price drops it: the field is not on the wire for a non-rechargeable id, so
    /// the row width cannot move.
    #[test]
    fn a_rechargeable_row_writes_its_unit_price_as_a_little_endian_double() {
        let subi = ClassicShopRow::buy(2_070_000, 500, 100).with_unit_price(300);
        assert_eq!(subi.unit_price(), Some(0.3));
        assert_eq!(subi.wire_len(), CLASSIC_ROW_LEN + 6, "the price does not widen the row");

        let potion = ClassicShopRow::buy(2_000_000, 50, 100).with_unit_price(300);
        assert_eq!(potion.unit_price(), None, "a potion has no such field on the wire");
        assert_eq!(potion.wire_len(), CLASSIC_ROW_LEN);

        let body = classic_open_shop(9_000_000, &[subi]);
        assert_eq!(body.len(), CLASSIC_HEAD_LEN + subi.wire_len());
        // The double is the 8 bytes before `i16 maxPerPurchase, u8 sell, u8 buyBack`.
        let at = body.len() - 12;
        let bits = u64::from_le_bytes(body[at..at + 8].try_into().unwrap());
        assert_eq!(f64::from_bits(bits), 0.3, "{:02x?}", &body[at..at + 8]);

        let unpriced = classic_open_shop(9_000_000, &[ClassicShopRow::buy(2_070_000, 500, 100)]);
        assert_eq!(unpriced.len(), body.len());
        assert_eq!(&unpriced[at..at + 8], &[0u8; 8], "no price is still 0.0, and Recharge stays unreachable");
    }

    #[test]
    fn every_request_arm_parses_and_a_short_body_does_not_panic() {
        let mut buy = vec![0u8];
        buy.extend_from_slice(&7u16.to_le_bytes());
        buy.extend_from_slice(&2_000_000u32.to_le_bytes());
        buy.extend_from_slice(&3u16.to_le_bytes());
        assert_eq!(
            parse_classic_shop_request(&buy),
            Some(ClassicShopRequest::Buy { row_index: 7, item_id: 2_000_000, quantity: 3 })
        );

        let mut sell = vec![1u8];
        sell.extend_from_slice(&2u16.to_le_bytes());
        sell.extend_from_slice(&2_000_000u32.to_le_bytes());
        sell.extend_from_slice(&5u16.to_le_bytes());
        assert_eq!(
            parse_classic_shop_request(&sell),
            Some(ClassicShopRequest::Sell { inventory_slot: 2, item_id: 2_000_000, quantity: 5 })
        );

        assert_eq!(
            parse_classic_shop_request(&[2, 4, 0]),
            Some(ClassicShopRequest::Recharge { inventory_slot: 4 })
        );
        assert_eq!(parse_classic_shop_request(&[3]), Some(ClassicShopRequest::Close));

        for n in 0..buy.len() {
            let _ = parse_classic_shop_request(&buy[..n]);
        }
        assert_eq!(parse_classic_shop_request(&[]), None);
    }
}
