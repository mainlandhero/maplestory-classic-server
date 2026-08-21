//! The NPC shop dialog: opening it (`0x0560`), the transaction result (`0x055F`), and the
//! client's one request opcode (`0x0104`).
//!
//! Labels are the house ones: **[L]** read off this client's listing, **[D]** derived from
//! two or more [L] facts, **[I]** inferred. Nothing here comes from the v214 reference -
//! its `NpcShopItem::encode` is a ~150-field record and this client's row is **31 bytes**,
//! so the two are not the same packet and the reference was used for nothing.
//!
//! Every address below was read with `python tools/listing.py` and cross-checked against
//! `python tools/reads.py <addr> 5`. The instrument was proved on its documented positive
//! control first (`tools/reads.py 0x140304100 2` -> `raw` at `140304138`, `u8` at
//! `140304144`, `u8` at `140304183`, then the run of `u16`), and the two tools agree on
//! both shop functions at depth 3 and depth 5. Raw dumps:
//! `research/msexe-shop-handler.txt`, `research/msexe-shop-rowdecode.txt`,
//! `research/msexe-shop-transaction.txt`, `research/msexe-shop-selltab.txt`,
//! `research/msexe-shop-activelist.txt`, `research/msexe-shop-routing.txt`.
//! Prose write-up: `research/npc-shop.md`.
//!
//! # Routing, [L]
//!
//! `CField::OnPacket` (`FUN_141820080`) - the same dispatcher that already delivers
//! `NPC_ENTER_FIELD` and the script message - reaches the shop through a two-opcode range
//! test at `141822011`:
//!
//! ```text
//! 141822011  lea  eax, [r9 - 0x55f]
//! 141822018  cmp  eax, 1
//! 14182201b  ja   0x14182202d       ; neither 0x55F nor 0x560
//! 14182201d  mov  rdx, rbx          ; the packet becomes arg 2
//! 141822020  mov  ecx, r9d          ; the OPCODE becomes arg 1
//! 141822023  call 0x140d225f0
//! ```
//!
//! so one function, `FUN_140d225f0(int opcode, CInPacket*)`, handles both, and forks on the
//! opcode in its first three instructions:
//!
//! ```text
//! 140d2262b  sub  ecx, 0x55f
//! 140d22631  je   0x140d22892       ; 0x055F -> the transaction result
//! 140d22637  cmp  ecx, 1
//! 140d2263a  jne  0x140d22fb5       ; neither -> return, silently
//!                                   ; fall through = 0x0560, the shop list
//! ```
//!
//! # Nothing here authenticates
//!
//! As everywhere else in this project, the channel socket carries no credentials. A shop
//! opens, and an item is bought or sold, purely because the packet arrived on the socket.
//! Every price and every quantity in a request is the client's word for it and must be
//! re-derived server-side.
//!
//! # The rule that costs a session if it is broken
//!
//! **`0x0104` sub-op [`SHOP_REQ_TRANSACTION`] must always be answered with a `0x055F`.**
//! `FUN_140d2c060` sets `[shopUI+0x14d8] = 1` at `140d2c77f` immediately after sending, and
//! its own first act at `140d2c0a6` is
//!
//! ```text
//! 140d2c0a6  cmp  dword ptr [rcx + 0x14d8], edi   ; edi = 0
//! 140d2c0ac  jne  0x140d2c4f7                     ; -> the epilogue. Nothing happens.
//! ```
//!
//! so while that latch is set **every further click on Buy or Sell is a silent no-op** - no
//! message, no sound, nothing in any log. This is the same shape as
//! [`crate::inventory::inventory_rejected`], where a refusal that sent nothing killed the
//! whole inventory UI for a session. [`shop_result`] exists so a refusal is one call. **[L]**
//!
//! **It is recoverable, and the exact wording matters because the inventory latch was not.**
//! `tools/fieldrefs.py 0x14d8` over the shop range finds **four** writers, not one:
//! `140d2c77f` sets it, and `140d22927` (the `0x055F` handler), `140d23082` (the `0x0560`
//! row decoder) and `140d258ff` (`CreateLayout`) all clear it. So a fresh [`OPEN_SHOP`]
//! un-sticks a shop that was left latched - the dialog is dead **until the next `0x055F` or
//! `0x0560`**, not for the session. Do not rely on that: closing and re-opening the shop is
//! something the player has to think to do, and there is nothing on screen to suggest it.
//!
//! # Two things the shop packet does NOT do
//!
//! * **It does not give the player the item, and it does not move mesos.** The success case
//!   of `0x055F` (`140d22951..140d229cf`) only adjusts the list's scroll position. The
//!   inventory change still has to go out as a `0x0070` ([`crate::inventory`]) and the meso
//!   balance still has to go out however stat changes go out. **[L]**
//! * **It does not decide what may be sold.** The client's own filter in `FUN_140d26870`
//!   (`140d2699e`, `140d269bf`, `140d269e0`) hides some items from the Sell tab, but the
//!   server is the only thing that can enforce the owner's "no quest items" rule - and
//!   `Store::sell_item` already does.

use crate::packet::{PacketReader, PacketWriter};

// ---------------------------------------------------------------------------------------
// Outbound: 0x0560, the shop list
// ---------------------------------------------------------------------------------------

/// **`OpenShop`** - server -> client. Puts `UI/UIWindow2.img/Shop2` on screen and fills it.
///
/// **[L]** `141822011` routes it; `140d2262b` forks to the fall-through path at `140d22640`.
///
/// Body:
///
/// ```text
/// off  size  read at      field
///   0     4  140d22643    npcTemplateId  -> shopUI+0x318
///   4     2  140d2264e    rowCount       (u16, ZERO-extended: movzx edi, ax)
///   6   31n  140d23030    rowCount rows, 31 bytes each - see ShopRow
///   *     1  140d23b96    trailing u8    -> shopUI+0x14dc
/// ```
///
/// # A count of zero does not open an empty shop - it opens nothing
///
/// **[L]** `140d22656  test edi, edi / jne 0x140d22819`. With `rowCount == 0` the client
/// takes the other arm entirely: it never calls the row decoder, never creates the shop UI,
/// puts a `CUIScriptMsg` on screen instead (`FUN_142a61900` at `140d2273d`, spoken by the
/// npc template it just read), and **sends `0x0104` sub-op [`SHOP_REQ_CLOSE`] back**
/// (`140d227a2`). So an empty shop is a dialog box, not a shop window.
pub const OPEN_SHOP: u16 = 0x0560;

/// **This is the WRONG window, and `0x055D` is the right one.** Found 2026-08-20.
///
/// `CField::OnPacket` dispatches four shop opcodes as two adjacent range tests:
/// `0x055D`/`0x055E` open and answer the **classic** counter, built from
/// `UI/UIShop.img/Shop` - art this client **has** - and `0x055F`/`0x0560` do the same for
/// **Shop2**, built from `UI/UIWindow2.img/Shop2`, which is **absent from this client's WZ**.
/// That absence is what kills the client, before it reads a single row byte, which is why one
/// correctly-formed row killed it exactly as twelve did.
///
/// Everything below still builds `0x0560`, on purpose. The classic body is longer and its
/// rows carry a **thirteen-field item structure** rather than an id and a price; sending a
/// `0x0560`-shaped body to `0x055D` would be a short packet to a handler expecting a long
/// one, which is the mistake that killed the client twice already for a different reason.
///
/// `research/classic-shop-opcode.md` has the decoded head, the two instruments that agree on
/// it, and every field still unnamed.

/// **`ShopTransactionResult`** - server -> client. The answer to every
/// [`SHOP_REQ_TRANSACTION`].
///
/// **[L]** Two `u8`s and nothing else, at `140d22931` and `140d22941`. The first **must be
/// exactly [`SHOP_DIALOG_KIND`]** - `140d22936  cmp al, 4 / jne 0x140d22fb5` jumps straight
/// to the epilogue - so a wrong first byte makes the whole packet a no-op *and leaves the
/// latch set*, which is the worst of both outcomes.
///
/// **[L]** The handler drops the packet before reading anything if no shop dialog is open:
/// `140d22892  mov rcx, qword ptr [rip + 0x2d85c87]` -> `0x143AA8520`, `test rcx, rcx /
/// je 0x140d22fb5`.
pub const SHOP_TRANSACTION_RESULT: u16 = 0x055F;

/// The first byte of a [`SHOP_TRANSACTION_RESULT`]: the dialog kind, and **4 is the only
/// value the handler accepts**.
///
/// **[L]** the gate at `140d22936`. **[D]** that 4 means "NPC shop": the shop UI's own
/// constructor passes the same literal to the base UI init -
/// `140d223fe  mov dword ptr [rsp + 0x28], 4` in `FUN_140d22310`, the function that
/// references `UI/UIWindow2.img/Shop2/backgrnd`.
pub const SHOP_DIALOG_KIND: u8 = 4;

/// A single shop row - **31 bytes, fixed, with no branch anywhere in the run**.
///
/// `FUN_140d23030(shopUI, packet, npcTemplateId, rowCount)`, `140d23030..140d23ebb`. The
/// loop body starts at `140d230da` and the back edge is `140d23b86  jl 0x140d230da`. Ten
/// reads in the whole function, nine of them in the loop; `tools/reads.py` at depth 3 and
/// depth 5 and `tools/listing.py` all give the same ten.
///
/// ```text
/// off  size  read at      stored at            what it is
///   0     4  140d2317d    row+0x20 (obf.)      row key    - echoed in the buy request
///   4     4  140d23346    row+0x38 (obf.)      itemId
///   8     4  140d2351a    row+0x58, |v|+0x5c   price, SIGNED - see below
///  12     4  140d2352d    row+0x60             unknown
///  16     4  140d23538    row+0x64             unknown
///  20     4  140d23543    row+0x6c             unknown
///  24     4  140d2354e    row+0x70             unknown
///  28     1  140d23559    row+0x74             disabled flag - non-zero HIDES the row
///  29     2  140d23564    row+0x68             max quantity per purchase
/// ```
///
/// # The sign of `price` chooses the tab. This is the finding that makes the Sell tab work
///
/// **[L]** The row is appended to one of two lists at `140d23ac5`:
///
/// ```text
/// 140d23ac5  cmp  dword ptr [rbx + 0x58], 0
/// 140d23ac9  mov  rcx, qword ptr [rbp - 0x10]   ; &shopUI+0x338
/// 140d23acd  jg   0x140d23ad3
/// 140d23acf  mov  rcx, qword ptr [rbp - 0x48]   ; &shopUI+0x340
/// 140d23ad3  call 0x140d2d0a0                   ; push_back
/// ```
///
/// and `FUN_140d2d040`, the "which list is the active tab" accessor, is
///
/// ```text
/// 140d2d068  cmp   dword ptr [rcx + 0x90], 0    ; the tab control's selected index
/// 140d2d06f  mov   eax, 0x340
/// 140d2d074  mov   edx, 0x338
/// 140d2d079  cmove eax, edx                     ; tab 0 -> +0x338, else -> +0x340
/// ```
///
/// So **`price > 0` is a Buy row and `price <= 0` is a Sell row**, and the two lists are
/// the two tabs. **[L]**
///
/// The Sell tab then shows `+0x340` intersected with what the player is carrying:
/// `FUN_140d26870` walks the character's inventory and, for each item, linear-searches
/// `[shopUI+0x340]` comparing the row's **itemId** (`row+0x38`, at `140d26a61`) against the
/// inventory item's id. A null or empty `+0x340` makes that search bail at `140d26a1a`, so
/// **a shop that sends no negative-price rows has an empty Sell tab**. **[L]**
///
/// # `row_key` is the server's to choose, and it is not the item id
///
/// **[L]** The only two uses of `row+0x20` are (1) it is written verbatim into the
/// transaction request at `140d2c747..140d2c756`, and (2) it keys a hash map on the
/// *character* at `140d2604b`, whose only effect on a hit is one extra `u32` in the tooltip
/// block (`140d26170  mov eax, dword ptr [rcx + 0x14]`); a miss falls through to the same
/// place. So a value the server invents cannot break anything, and the server needs it to
/// be unique because it is the only thing that comes back. This module uses the row's
/// **index in the packet**, and [`ShopRow::row_key`] is public so a caller may do otherwise.
///
/// # The four unknown `u32`s
///
/// **[L]** that they are read at those four addresses and **[L]** that their only consumer
/// is `FUN_140d25dc0`, which copies `+0x58/+0x60/+0x64/+0x6c/+0x70` into a 21-byte block at
/// `140d2601c..140d2603c` for the tooltip. **Not established** what any of them means. They
/// are sent as zero, which is what an absent optional costs everywhere else in this record
/// family; the buy path and the list build never read them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShopRow {
    /// Row 0: the token the client hands back in [`ShopTransaction::row_key`]. **Not** the
    /// item id.
    pub row_key: u32,
    /// Row 4: the real `Item.wz` id. Fed to `GetInventoryType` (`FUN_1403e8af0`) at
    /// `140d2c2d2` and matched against inventory items at `140d26a68`, so it has to be
    /// real. **[L]**
    pub item_id: u32,
    /// Row 8, **signed**. Positive puts the row in the Buy tab and is the price per unit
    /// the client charges the player; negative puts it in the Sell tab and `|price|` is
    /// what it shows as the payout. The client checks `|price| * quantity` against the
    /// player's mesos itself at `140d2c242..140d2c260`. **[L]**
    pub price: i32,
    /// Row 12/16/20/24. Read, copied to the tooltip block, meaning **not established**.
    pub unknown: [u32; 4],
    /// Row 28. **Non-zero hides the row and disables buying it**, on both paths:
    /// `140d2c15e  cmp byte ptr [rsi + 0x74], 0 / jne` bails out of the whole transaction,
    /// and `140d26b09` skips the row when building the Sell tab. Send `false`. **[L]**
    pub disabled: bool,
    /// Row 29: the most the player may take in one purchase. The client puts it in the
    /// "How many are you willing to buy?" box as the maximum (`140d2c1f5`) and rejects the
    /// answer if it exceeds it (`140d2c239  cmp ebx, dword ptr [rsi + 0x68] / jg` -> bail).
    /// **Zero makes every quantity fail**, silently. **[L]**
    pub max_per_purchase: u16,
}

impl ShopRow {
    /// A Buy row: the NPC sells `item_id` at `price` each, up to `max_per_purchase` at once.
    pub fn buy(row_key: u32, item_id: u32, price: u32, max_per_purchase: u16) -> Self {
        Self {
            row_key,
            item_id,
            price: i32::try_from(price).unwrap_or(i32::MAX),
            unknown: [0; 4],
            disabled: false,
            max_per_purchase,
        }
    }

    /// A Sell row: the NPC buys `item_id` back for `unit_price` each. The price goes on the
    /// wire **negated**, which is what puts the row in the Sell tab.
    pub fn sell(row_key: u32, item_id: u32, unit_price: u32) -> Self {
        Self {
            row_key,
            item_id,
            price: -i32::try_from(unit_price).unwrap_or(i32::MAX),
            unknown: [0; 4],
            disabled: false,
            // A sell is always quantity 1 (140d2c16b sets it and nothing on that path
            // overwrites it), but the field is still read, and zero is the value that makes
            // a quantity fail. One is the smallest honest answer.
            max_per_purchase: 1,
        }
    }

    /// Which tab this row lands in - `140d23ac5`'s `cmp`/`jg`.
    pub fn is_buy_row(&self) -> bool {
        self.price > 0
    }

    fn write(&self, w: &mut PacketWriter) {
        w.u32(self.row_key); //             140d2317d
        w.u32(self.item_id); //             140d23346
        w.i32(self.price); //               140d2351a
        w.u32(self.unknown[0]); //          140d2352d
        w.u32(self.unknown[1]); //          140d23538
        w.u32(self.unknown[2]); //          140d23543
        w.u32(self.unknown[3]); //          140d2354e
        w.bool(self.disabled); //           140d23559
        w.u16(self.max_per_purchase); //    140d23564
    }
}

/// Bytes one [`ShopRow`] occupies: `4+4+4 + 4*4 + 1 + 2`.
pub const SHOP_ROW_LEN: usize = 31;

/// The fixed part of an [`OPEN_SHOP`] body: `u32 npcTemplateId`, `u16 rowCount`, and the
/// trailing `u8` after the rows.
pub const OPEN_SHOP_FIXED_LEN: usize = 7;

/// Build an [`OPEN_SHOP`] body (no opcode - the caller prepends it, the same way
/// [`crate::opcode::npc_enter_field`] is used).
///
/// `npc_template_id` must be a real `Npc.wz` id - it lands in `shopUI+0x318` at `140d23063`
/// and is the value the client sends back in [`SHOP_REQ_REOPEN`]. **[L]**
///
/// **The count is a `u16`.** `140d2264e` is `FUN_1406e8b80`, and there is no length prefix
/// and no resynchronisation point anywhere in this record - a `u8` here would shift every
/// row by one byte and the client would read 31-byte rows out of the middle of the list.
///
/// # Panics
///
/// Never. More than `u16::MAX` rows would be truncated by the `as` cast, and the body would
/// then disagree with its own count with nothing to resynchronise on; callers with an
/// unbounded list should slice it. In practice the largest shop in `data/shops.txt` is Don
/// Hwang's, with **98** rows - 196 once the sell-back rows are added, well inside a `u16`.
///
/// ```
/// use net::shop::{open_shop, ShopRow, OPEN_SHOP, OPEN_SHOP_FIXED_LEN, SHOP_ROW_LEN};
/// let body = open_shop(21, &[ShopRow::buy(0, 2000000, 50, 100)]);
/// assert_eq!(OPEN_SHOP, 0x0560);
/// assert_eq!(body.len(), OPEN_SHOP_FIXED_LEN + SHOP_ROW_LEN);
/// ```
pub fn open_shop(npc_template_id: u32, rows: &[ShopRow]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(npc_template_id); //                 140d22643
    w.u16(rows.len() as u16); //               140d2264e - a u16, not a u8
    for row in rows {
        row.write(&mut w);
    }
    w.u8(0); //                                140d23b96 -> shopUI+0x14dc
    let out = w.into_vec();
    debug_assert_eq!(out.len(), OPEN_SHOP_FIXED_LEN + rows.len() * SHOP_ROW_LEN);
    out
}

// ---------------------------------------------------------------------------------------
// Outbound: 0x055F, the transaction result
// ---------------------------------------------------------------------------------------

/// What the client does with a [`SHOP_TRANSACTION_RESULT`]'s second byte.
///
/// **[L]** `140d22946  movzx edi, al`, then either the success arm at `140d22951` (`al == 0`)
/// or the error arm at `140d229d1`, which does `dec edi / cmp edi, 0xd / ja <default>` and
/// indexes a **14-entry** jump table at `0x140d22fe8` (decoded in
/// `research/msexe-shop-routing.txt`). Codes above 14 reach the default: **no message at
/// all**, but the latch is still cleared, so they are safe if uninformative.
///
/// The message text is the client's own, resolved by string id through `FUN_1408a9e40` and
/// decrypted here with `tools/dump_stringids.py`. **[L]**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ShopResult {
    /// `0` - success. The client shows nothing, adjusts the list scroll position, and
    /// **changes neither the bag nor the meso count** - those are separate packets.
    Success = 0,
    /// `1` - "That's not something I barter. I'll show you the list again for you to choose
    /// from." (string `0xC57`). Also re-requests the shop; see [`ShopResult::rerequests`].
    UnknownItem = 1,
    /// `4` - "Are you begging?" (`0xC58`). The client's own wording for a price problem.
    NotEnoughMesos = 4,
    /// `5` - "Why do you carry around so much Mesos? Come back after emptying your
    /// pockets." (`0xC59`) - the meso cap.
    MesoLimit = 5,
    /// `6` - "Taking your precious time, huh? I don't barter this item anymore." (`0xC5A`).
    /// Re-requests the shop.
    NoLongerSold = 6,
    /// `7` - "I'm done bartering this item for the day. Come back tomorrow." (`0xC5B`).
    /// Re-requests the shop.
    SoldOutToday = 7,
    /// `8` - "Why are you obsessing over this item? I won't barter anymore." (`0xC5C`).
    /// Re-requests the shop.
    BuyLimitReached = 8,
    /// `9` - "You're greedy. How about you lower the quantity?" (`0xC5D`).
    QuantityTooHigh = 9,
    /// `10` - "Items or mesos cannot be moved.\r\nPlease contact customer support."
    /// (`0xDAB`).
    CannotMove = 10,
    /// `11` - "That cannot be done in the current world." (`0x12D6`).
    WrongWorld = 11,
    /// `12` - "Please check if your inventory is full or not." (`0x4AA`). **This is the one
    /// to send when [`crate::inventory`]'s bag is full**, and it is the closest match to
    /// `StoreError::BagFull`.
    InventoryFull = 12,
    /// `13` - "I'm a little preoccupied right now, so come back later." (`0xC5E`).
    /// Re-requests the shop. The best generic "the server refused and cannot say why".
    Busy = 13,
    /// `14` - "Item cannot be obtained when incapacitated." (`0x14E8`).
    Incapacitated = 14,
}

impl ShopResult {
    pub fn code(self) -> u8 {
        self as u8
    }

    /// **[L]** Codes 1, 2, 3, 6, 7, 8 and 13 make the client immediately send a
    /// [`SHOP_REQ_REOPEN`] and call `[shopUIvtbl+0x138]`. The chain of `cmp edi, N / je
    /// 0x140d22a20` at `140d229df..140d22a18` is the whole set, and `140d22a20` sets the
    /// "re-request" flag the tail at `140d22f51` tests.
    ///
    /// **A server that sends one of these owes a fresh [`OPEN_SHOP`] straight after.**
    pub fn rerequests(self) -> bool {
        matches!(
            self,
            ShopResult::UnknownItem
                | ShopResult::NoLongerSold
                | ShopResult::SoldOutToday
                | ShopResult::BuyLimitReached
                | ShopResult::Busy
        )
    }
}

/// Length of a [`shop_result`] body: two `u8`s, and there is nothing else in the packet.
pub const SHOP_RESULT_LEN: usize = 2;

/// Build a [`SHOP_TRANSACTION_RESULT`] body (no opcode).
///
/// **Send one for every [`SHOP_REQ_TRANSACTION`], including - most of all - the refusals.**
/// See the module docs: the alternative is a shop dialog whose Buy and Sell buttons do
/// nothing for the rest of the session, with nothing on screen or in any log to say why.
///
/// ```
/// use net::shop::{shop_result, ShopResult, SHOP_DIALOG_KIND, SHOP_RESULT_LEN};
/// let b = shop_result(ShopResult::Success);
/// assert_eq!(b, vec![SHOP_DIALOG_KIND, 0]);
/// assert_eq!(b.len(), SHOP_RESULT_LEN);
/// ```
pub fn shop_result(result: ShopResult) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(SHOP_DIALOG_KIND); //   140d22931 - anything else and the packet is discarded
    w.u8(result.code()); //      140d22941
    w.into_vec()
}

// ---------------------------------------------------------------------------------------
// Inbound: 0x0104, the client's shop request
// ---------------------------------------------------------------------------------------

/// **The client's only shop request opcode.** Every sub-op below rides on it.
///
/// **[L]** Found by enumeration rather than by guessing at an opcode map. The set is closed
/// over the shop UI class's **76 vtable methods** at `0x14336c6e0`, not over an address
/// range: exactly five of them reach the `COutPacket` ctor, three are the shop's own and
/// two are inherited base-class methods that build `0x02D9` and one other packet, neither
/// of them `0x0104`. The first version of this claim used an address range and would have
/// missed both - see `research/npc-shop.md` §4.0.
///
/// | site | sub-op | body after the `u8` |
/// |---|---|---|
/// | `140d22f61` in `FUN_140d225f0` | 0 | `u32 npcTemplateId` |
/// | `140d2c736` in `FUN_140d2c060` | 1 | `u32 rowKey, u16 quantity, u16 slot` |
/// | `140d227a2` in `FUN_140d225f0` | 2 | nothing |
/// | `140d23efe` in `FUN_140d23ed0` | 2 | nothing |
/// | `14216a837` in `FUN_14216a810` | 3 | `u32` - **not** built by the shop UI; unread |
pub const CLIENT_SHOP_REQUEST: u16 = 0x0104;

/// Sub-op `0`: "send me this NPC's shop again".
///
/// **[L]** `140d22f58..140d22f79`. Sent unprompted by the client after any
/// [`ShopResult::rerequests`] result, carrying `shopUI+0x318` - the npc template id from the
/// [`OPEN_SHOP`] that opened the dialog. The right answer is a fresh [`OPEN_SHOP`].
pub const SHOP_REQ_REOPEN: u8 = 0;

/// Sub-op `1`: buy or sell. **Must be answered with a [`SHOP_TRANSACTION_RESULT`].**
pub const SHOP_REQ_TRANSACTION: u8 = 1;

/// Sub-op `2`: the dialog is closing.
///
/// **[L]** Two senders, and neither waits for anything. `FUN_140d23ed0` sends it and then
/// calls `FUN_14177fef0` - the UI teardown - in the next instruction, and
/// `FUN_140d225f0`'s zero-row arm sends it having never built a dialog. Nothing latches, so
/// **no answer is required**; the server should drop its own shop state and say nothing.
pub const SHOP_REQ_CLOSE: u8 = 2;

/// Sub-op `3`: `u8 3, u32`. Built by `FUN_14216a810`, which is **not** part of the shop UI
/// (`0x1421xxxxx`, reached from `FUN_142163340` and `FUN_142163f00` after a yes/no box).
///
/// **Not established** what it is. It is listed so that an unrecognised sub-op on the wire
/// has a name rather than looking like a corrupt packet, and so nobody re-derives the same
/// dead end. Answering it is untested; see `research/npc-shop.md`.
pub const SHOP_REQ_UNKNOWN_3: u8 = 3;

/// A decoded [`CLIENT_SHOP_REQUEST`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopRequest {
    /// [`SHOP_REQ_REOPEN`] - answer with a fresh [`OPEN_SHOP`] for this template.
    Reopen { npc_template_id: u32 },
    /// [`SHOP_REQ_TRANSACTION`] - answer with a [`SHOP_TRANSACTION_RESULT`], always.
    Transaction(ShopTransaction),
    /// [`SHOP_REQ_CLOSE`] - nothing is owed.
    Close,
    /// [`SHOP_REQ_UNKNOWN_3`], or any sub-op this module does not know. The raw body is
    /// kept so a caller can log it; **it is deliberately not an error**, because a caller
    /// that turns a parse failure into silence is how a UI freezes.
    Other { sub_op: u8 },
}

/// The buy-or-sell request, [`SHOP_REQ_TRANSACTION`].
///
/// **[L]** `FUN_140d2c060`'s tail, `140d2c72d..140d2c77a`:
///
/// ```text
/// 140d2c73c  mov   dl, 1                    ; the sub-op
/// 140d2c747  lea   rcx, [rsi + 0x20]        ; the selected row's key, de-obfuscated
/// 140d2c74b  call  0x14019a5d0
/// 140d2c756  call  0x1406ed9d0              ; u32 rowKey
/// 140d2c75b  movzx edx, word ptr [rsp + 0x60]
/// 140d2c764  call  0x1406ed940              ; u16 quantity
/// 140d2c769  movzx edx, r15w
/// 140d2c771  call  0x1406ed940              ; u16 slot
/// ```
///
/// # Which direction is this?
///
/// **The wire does not say, and that is the design.** The client picks the row out of one
/// of the two tabs and sends only its key, so **the server tells buy from sell by looking
/// up `row_key` in the list it sent** - a row it sent with a positive price is a buy and a
/// row it sent with a negative price is a sell. [`ShopTransaction::looks_like_sell`] exists
/// as a corroborating hint and **not** as the decision, for the reason below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShopTransaction {
    /// The [`ShopRow::row_key`] of the row that was clicked.
    pub row_key: u32,
    /// How many. **[L]** On a buy this is what the player typed, already range-checked by
    /// the client against [`ShopRow::max_per_purchase`] (`140d2c231`, `140d2c239`) - which
    /// is a client check and therefore not a check. On a sell it is always `1`
    /// (`140d2c16b  mov dword ptr [rsp + 0x60], 1`, and nothing on that path overwrites it).
    pub quantity: u16,
    /// The player's inventory slot the client found the item in.
    ///
    /// **[L]** `0` on the buy path (`140d2c168  mov r15d, edi`, `edi = 0`); on the other
    /// path it is the slot index the scan at `140d2c2f0..140d2c3f0` stopped on, or
    /// `row+0x50` when the row already carries one (`140d2c421`).
    pub slot: u16,
}

impl ShopTransaction {
    /// A **hint**, not a decision: the buy path always sends slot `0`, so a non-zero slot
    /// cannot have come from it. **[L]** for the buy side.
    ///
    /// It is not the other way round - the sell path's scan starts at slot 1 but nothing
    /// proves it can never report 0 - and in any case a client-supplied field is not
    /// evidence about a server-side list. Use the row key.
    pub fn looks_like_sell(&self) -> bool {
        self.slot != 0
    }
}

/// Decode a [`CLIENT_SHOP_REQUEST`] body (no opcode - the payload after it).
///
/// Returns `None` **only** when the sub-op byte itself is missing, and
/// [`ShopRequest::Other`] for a sub-op whose shape is unknown. That asymmetry is on
/// purpose: a caller that maps `None` onto "ignore it" would leave a
/// [`SHOP_REQ_TRANSACTION`] unanswered and latch the dialog for the rest of the session.
///
/// A truncated `Reopen` or `Transaction` also degrades to [`ShopRequest::Other`] rather
/// than to `None`, so it is still answerable.
pub fn parse_shop_request(body: &[u8]) -> Option<ShopRequest> {
    let mut r = PacketReader::new(body);
    let sub_op = r.u8().ok()?;
    Some(match sub_op {
        SHOP_REQ_REOPEN => match r.u32() {
            Ok(npc_template_id) => ShopRequest::Reopen { npc_template_id },
            Err(_) => ShopRequest::Other { sub_op },
        },
        SHOP_REQ_TRANSACTION => {
            match (r.u32(), r.u16(), r.u16()) {
                (Ok(row_key), Ok(quantity), Ok(slot)) => {
                    ShopRequest::Transaction(ShopTransaction { row_key, quantity, slot })
                }
                _ => ShopRequest::Other { sub_op },
            }
        }
        SHOP_REQ_CLOSE => ShopRequest::Close,
        _ => ShopRequest::Other { sub_op },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two opcodes the client's own range test uses, so a rename or a typo cannot go
    /// unnoticed. `141822011: lea eax, [r9 - 0x55f] / cmp eax, 1`.
    #[test]
    fn the_opcodes_are_the_ones_cfield_onpacket_tests_for() {
        assert_eq!(SHOP_TRANSACTION_RESULT, 0x055F);
        assert_eq!(OPEN_SHOP, 0x0560);
        assert_eq!(OPEN_SHOP - SHOP_TRANSACTION_RESULT, 1, "one range test covers both");
        assert_eq!(CLIENT_SHOP_REQUEST, 0x0104); // 140d2c72d: mov edx, 0x104
    }

    /// Every offset here is the address the client reads that field at. The record has no
    /// length prefix and no resynchronisation point, so one wrong width silently shifts
    /// every row after it.
    #[test]
    fn an_open_shop_body_puts_every_field_where_the_client_reads_it() {
        let rows = [ShopRow::buy(0, 2_000_000, 50, 100)];
        let b = open_shop(21, &rows);

        assert_eq!(&b[0..4], &21u32.to_le_bytes(), "140d22643 npcTemplateId");
        assert_eq!(&b[4..6], &1u16.to_le_bytes(), "140d2264e rowCount, a u16");
        // The row starts at 6 and is 31 bytes.
        assert_eq!(&b[6..10], &0u32.to_le_bytes(), "140d2317d rowKey");
        assert_eq!(&b[10..14], &2_000_000u32.to_le_bytes(), "140d23346 itemId");
        assert_eq!(&b[14..18], &50i32.to_le_bytes(), "140d2351a price");
        assert_eq!(&b[18..22], &0u32.to_le_bytes(), "140d2352d");
        assert_eq!(&b[22..26], &0u32.to_le_bytes(), "140d23538");
        assert_eq!(&b[26..30], &0u32.to_le_bytes(), "140d23543");
        assert_eq!(&b[30..34], &0u32.to_le_bytes(), "140d2354e");
        assert_eq!(b[34], 0, "140d23559 disabled");
        assert_eq!(&b[35..37], &100u16.to_le_bytes(), "140d23564 max per purchase");
        assert_eq!(b[37], 0, "140d23b96 the trailing u8, after ALL the rows");
        assert_eq!(b.len(), 38);
        assert_eq!(b.len(), OPEN_SHOP_FIXED_LEN + SHOP_ROW_LEN);
    }

    /// The count is read by `FUN_1406e8b80` at `140d2264e` - a `u16`. A `u8` there would
    /// put the first row one byte early and every field of every row would be wrong, with
    /// nothing in the record to resynchronise on.
    #[test]
    fn the_row_count_is_a_u16_and_the_rows_are_exactly_thirty_one_bytes_each() {
        assert_eq!(SHOP_ROW_LEN, 31, "4+4+4 + 4*4 + 1 + 2");

        for n in [0usize, 1, 2, 6, 63, 300] {
            let rows: Vec<ShopRow> =
                (0..n).map(|i| ShopRow::buy(i as u32, 2_000_000, 50, 100)).collect();
            let b = open_shop(21, &rows);
            assert_eq!(&b[4..6], &(n as u16).to_le_bytes(), "count for n = {n}");
            assert_eq!(b.len(), OPEN_SHOP_FIXED_LEN + n * SHOP_ROW_LEN, "length for n = {n}");
            // The trailing u8 is the LAST byte, after every row - 140d23b96 is reached
            // after the loop's back edge at 140d23b86 falls through.
            assert_eq!(*b.last().unwrap(), 0);
        }

        // 300 rows is past a u8 and the count must still be right.
        let rows: Vec<ShopRow> = (0..300).map(|i| ShopRow::buy(i, 2_000_000, 50, 100)).collect();
        let b = open_shop(21, &rows);
        assert_eq!(&b[4..6], &300u16.to_le_bytes());
        assert_ne!(b[4], b.len() as u8, "a u8 count would have aliased here");
    }

    /// `140d23ac5  cmp dword ptr [rbx + 0x58], 0 / jg` picks `+0x338` (Buy) over `+0x340`
    /// (Sell). A sell row therefore has to reach the wire **negated**, and the field has to
    /// be written as a signed 32-bit value.
    #[test]
    fn the_sign_of_the_price_is_what_chooses_the_tab() {
        let buy = ShopRow::buy(0, 2_000_000, 50, 100);
        assert!(buy.is_buy_row());
        assert_eq!(&open_shop(21, &[buy])[14..18], &50i32.to_le_bytes());

        let sell = ShopRow::sell(1, 4_000_000, 7);
        assert!(!sell.is_buy_row(), "a sell row must NOT land in the buy list");
        assert_eq!(sell.price, -7);
        assert_eq!(&open_shop(21, &[sell])[14..18], &(-7i32).to_le_bytes());
        // Two's complement, so the top bit is set - that is what `jg` tests.
        assert_eq!(open_shop(21, &[sell])[17] & 0x80, 0x80);

        // A zero price is NOT a buy row: `jg` is strictly greater.
        let free = ShopRow { price: 0, ..buy };
        assert!(!free.is_buy_row());

        // A sell row still carries a non-zero max_per_purchase, because zero is the value
        // that makes `140d2c239  cmp ebx, [rsi+0x68] / jg` reject every quantity.
        assert_eq!(sell.max_per_purchase, 1);
        assert!(!sell.disabled, "140d2c15e bails the whole transaction on a non-zero byte");
    }

    /// `140d22936  cmp al, 4 / jne 0x140d22fb5` - the epilogue. A wrong first byte discards
    /// the packet **and leaves the latch set**.
    #[test]
    fn a_result_is_the_dialog_kind_then_the_code_and_nothing_else() {
        assert_eq!(SHOP_DIALOG_KIND, 4);
        for (r, code) in [
            (ShopResult::Success, 0u8),
            (ShopResult::UnknownItem, 1),
            (ShopResult::NotEnoughMesos, 4),
            (ShopResult::InventoryFull, 12),
            (ShopResult::Busy, 13),
            (ShopResult::Incapacitated, 14),
        ] {
            let b = shop_result(r);
            assert_eq!(b.len(), SHOP_RESULT_LEN);
            assert_eq!(b[0], SHOP_DIALOG_KIND, "140d22931");
            assert_eq!(b[1], code, "140d22941");
            assert_eq!(r.code(), code);
        }
    }

    /// The seven codes at `140d229df..140d22a18` that make the client send a
    /// [`SHOP_REQ_REOPEN`] on its own. Codes 2 and 3 are in the client's set too and share
    /// string `0xC57` with code 1; this enum does not name them, so the test pins the ones
    /// it does name and the count of the rest is recorded in `research/npc-shop.md`.
    #[test]
    fn the_results_that_make_the_client_re_ask_for_the_list() {
        for r in [
            ShopResult::UnknownItem,
            ShopResult::NoLongerSold,
            ShopResult::SoldOutToday,
            ShopResult::BuyLimitReached,
            ShopResult::Busy,
        ] {
            assert!(r.rerequests(), "{r:?} owes a fresh OPEN_SHOP");
        }
        for r in [
            ShopResult::Success,
            ShopResult::NotEnoughMesos,
            ShopResult::MesoLimit,
            ShopResult::QuantityTooHigh,
            ShopResult::CannotMove,
            ShopResult::WrongWorld,
            ShopResult::InventoryFull,
            ShopResult::Incapacitated,
        ] {
            assert!(!r.rerequests(), "{r:?} must not");
        }
    }

    /// The four sub-ops, in the byte order the builders write them.
    #[test]
    fn the_client_requests_decode() {
        // 140d22f58: u8 0, u32 npcTemplateId
        let reopen = [0x00u8, 0x15, 0x00, 0x00, 0x00];
        assert_eq!(
            parse_shop_request(&reopen),
            Some(ShopRequest::Reopen { npc_template_id: 21 })
        );

        // 140d2c72d: u8 1, u32 rowKey, u16 quantity, u16 slot
        let buy = [0x01u8, 0x03, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00];
        let ShopRequest::Transaction(t) = parse_shop_request(&buy).unwrap() else {
            panic!("sub-op 1 is a transaction");
        };
        assert_eq!(t.row_key, 3);
        assert_eq!(t.quantity, 10);
        assert_eq!(t.slot, 0);
        assert!(!t.looks_like_sell(), "the buy path writes slot 0 at 140d2c168");

        let sell = [0x01u8, 0x07, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04, 0x00];
        let ShopRequest::Transaction(t) = parse_shop_request(&sell).unwrap() else {
            panic!("sub-op 1 is a transaction");
        };
        assert_eq!(t.row_key, 7);
        assert_eq!(t.quantity, 1, "the sell path forces 1 at 140d2c16b");
        assert_eq!(t.slot, 4);
        assert!(t.looks_like_sell());

        // 140d23efe / 140d227a2: u8 2 and nothing else
        assert_eq!(parse_shop_request(&[0x02]), Some(ShopRequest::Close));

        // 14216a837: u8 3, u32 - not the shop UI, shape known, meaning not established
        assert_eq!(
            parse_shop_request(&[0x03, 0x01, 0x00, 0x00, 0x00]),
            Some(ShopRequest::Other { sub_op: 3 })
        );
    }

    /// Only a body with no sub-op byte at all returns `None`. Everything else has to remain
    /// answerable, because a caller that treats `None` as "drop it" latches the dialog -
    /// exactly what `inventory_rejected` exists to prevent on `0x0107`.
    #[test]
    fn a_truncated_request_is_still_answerable() {
        assert!(parse_shop_request(&[]).is_none());

        // Sub-op 1 with a short body: still a request, still owed a 0x055F.
        assert_eq!(
            parse_shop_request(&[0x01, 0x03, 0x00]),
            Some(ShopRequest::Other { sub_op: 1 })
        );
        assert_eq!(
            parse_shop_request(&[0x00]),
            Some(ShopRequest::Other { sub_op: 0 }),
            "a Reopen with no template id is still a request"
        );
        assert_eq!(
            parse_shop_request(&[0x63]),
            Some(ShopRequest::Other { sub_op: 0x63 })
        );

        // Trailing bytes on a Close are ignored rather than rejected.
        assert_eq!(parse_shop_request(&[0x02, 0xaa, 0xbb]), Some(ShopRequest::Close));
    }

    /// Lucy, NPC template 21, with the six rows `data/shops.txt` gives them, plus the six
    /// sell-back rows the Sell tab needs. The point of the test is the arithmetic the
    /// client does on the wire: 4 + 2 + 12*31 + 1.
    #[test]
    fn lucys_shop_is_the_size_the_client_will_read() {
        // (itemId, buy price, sell price). The buy price is `data/shops.txt`; the item id
        // and the sell price are both `gm-handbook/itemdata.txt`, i.e. the client's own
        // `Item.wz info/price`. Neither column is typed from memory - the first draft of
        // this test guessed 2022000/1/2 for the fruit and all three were wrong.
        let catalogue = [
            (2_000_000u32, 50u32, 5u32), // Red Potion
            (2_000_001, 150, 15),        // Orange Potion
            (2_010_001, 20, 2),          // Apple
            (2_010_002, 30, 3),          // Egg
            (2_010_004, 50, 5),          // Orange
            (2_120_000, 35, 15),         // Pet Food
        ];
        let mut rows = Vec::new();
        for (i, (id, buy, _)) in catalogue.iter().enumerate() {
            rows.push(ShopRow::buy(i as u32, *id, *buy, 100));
        }
        for (i, (id, _, sell)) in catalogue.iter().enumerate() {
            rows.push(ShopRow::sell((catalogue.len() + i) as u32, *id, *sell));
        }
        let b = open_shop(21, &rows);

        assert_eq!(&b[0..4], &21u32.to_le_bytes(), "Lucy");
        assert_eq!(&b[4..6], &12u16.to_le_bytes());
        assert_eq!(b.len(), 7 + 12 * 31);
        assert_eq!(b.len(), 379);

        // Six rows in the Buy tab, six in the Sell tab.
        assert_eq!(rows.iter().filter(|r| r.is_buy_row()).count(), 6);
        assert_eq!(rows.iter().filter(|r| !r.is_buy_row()).count(), 6);

        // Every row key is distinct - it is the only thing the client hands back.
        let mut keys: Vec<u32> = rows.iter().map(|r| r.row_key).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), rows.len());
    }
}
