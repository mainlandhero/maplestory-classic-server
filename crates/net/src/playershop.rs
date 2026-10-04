//! **Player stores**: the store window, the sign over the owner's head, and every request the
//! window sends.
//!
//! The owner, 2026-10-04: *"I just tried to open a player store, but nothing happened."* The create
//! had arrived (`world-ch0.log`: `0x017F`, `00000000 03000000 0700 "garbage" 0f00 216e4e00` -
//! store, the title, permit in Cash slot 15, item 5140001) and nothing handled the opcode.
//!
//! A store is **not** the trade/Omok miniroom (`0x017E` / `0x0575`): it has two opcodes of its own
//! each way and its own dialog class (`FUN_140d984d0`, vtable `0x14336F0C0`, 0x438 bytes). Decoded
//! statically on 2026-10-04 from `client-patched/MapleStory.exe`; the walk is
//! `research/player-store-2026-10-04.md`. **[L]** read off the listing, **[D]** derived.
//!
//! ```text
//! client -> server
//!   0x017F  u32 mode                         the room            (FUN_142cd4550, FUN_140d91c40, ...)
//!             0  create  raw4 type, str title, u16 permit slot, u32 permit item
//!             1  close the store (the owner's "storeclose", after its own Yes/No)
//!             2  visit   u32 shop id         (a double-click on the sign, FUN_1428b7600)
//!             3  leave                       (the window closing, FUN_140d91f30)
//!             5  chat    str
//!             6  ban     u16 seat            (FUN_140d93f50 via vt+0x1d8)
//!   0x0180  u32 mode                         the shelf
//!             0  open the store ("storeopen")
//!             1  take back  u16 row
//!             2  list    u8 tab, u16 bag slot, u16 bundles, u16 per bundle, u64 price
//!             3  buy     u8 row, u16 bundles, u64 total, u32 (a hash the server ignores)
//!             4  collect (a hired merchant's revenue)
//!
//! server -> client
//!   0x0576  u32 mode   to an OPEN window     (FUN_140d90e90)
//!             0  the seats      -> vt+0x1a8 -> FUN_140d923e0
//!             1  the state      raw4         -> FUN_140d927f0
//!             2  a chat line    u32 seat, u32, u32 id, str name, str text
//!             3  a seat empties u32 seat, raw4 reason - one's OWN seat closes one's window
//!   0x0577  u32 mode   the room              (FUN_140d91000)
//!             0  a message      u32 code
//!             1  close the window
//!             2  OPEN the window (only when none is open) - see `open_window`
//!             3  u32 code: 0 closes the window, anything else is a message
//!             4  the state      raw4
//!             6  nothing - releases the request latch
//!   0x0578  u32 mode   the shelf, unasked    (FUN_140d949c0)
//!             0  the rows
//!   0x0579  u32 mode   the answer to a shelf request (FUN_140d9fe50); releases the latch
//!             0  u32 code      a room-session message
//!             1  u8 has, [rows]   a listing or a take-back went through
//!             2  u8 has, [rows]   a purchase went through
//!             3  u32 code      a store message
//!   0x0234  the sign over the owner's head (user pool, FUN_142795a70)
//! ```
//!
//! Every `0x017F` / `0x0180` the window sends behind its request latch (`FUN_142cc4430(.., 1)`)
//! is answered by a `0x0577` or a `0x0579`, which release it - **always answer**: a store left
//! waiting refuses every later button.

use crate::PacketReader;
use crate::PacketWriter;

/// Client -> server: the store room.
pub const CLIENT_SHOP_ROOM: u16 = 0x017F;
/// Client -> server: the store's shelf.
pub const CLIENT_SHOP_SHELF: u16 = 0x0180;
/// Server -> client: an event in an open store window.
pub const SHOP_EVENT: u16 = 0x0576;
/// Server -> client: the store room - open, close, messages.
pub const SHOP_ROOM: u16 = 0x0577;
/// Server -> client: the shelf, unasked (somebody bought something).
pub const SHOP_ROWS: u16 = 0x0578;
/// Server -> client: the answer to a shelf request.
pub const SHOP_ANSWER: u16 = 0x0579;
/// Client -> server: **may I put up a hired merchant?** - a Hired Merchant (`503xxxx`) double-clicked
/// in the Cash tab. `FUN_142cd4b70`: `u16 cash slot, u32 item`, which the client also keeps at
/// `ctx+0x415c` / `+0x4160` and acts on only when [`HIRED_CHECK_RESULT`] says yes [L]. The owner,
/// 2026-10-04: *"I just tried double clicking the elf hired merchant and it doesn't do anything."* -
/// nine of these in `world-ch0.log`, each `1000 70c04c00` (slot 16, 5030000), none answered.
pub const CLIENT_HIRED_CHECK: u16 = 0x0181;
/// Server -> client: the answer to [`CLIENT_HIRED_CHECK`]. `FUN_142d1cdf0`, `CWvsContext` case
/// `0x00A1` (`research/msexe-gamestage-cases.txt`) [L]:
///
/// ```text
/// raw4 mode   0 -> raw4 code: 0 = YES - FUN_142cd4550(ctx, slot, item, 1), the title prompt and
///                                       then 0x017F mode 0 with room type 3 + 1 = 4
///                             2, 3 = 0x17C7 "You already have an open Hired Merchant."
///                             else  = 0x178A "An unknown error has occurred."
///             1, 2 -> raw4 code, the same messages; any other mode reads nothing more
/// then        ctx+0x2330 = 0 - the request latch, released whatever the answer
/// ```
pub const HIRED_CHECK_RESULT: u16 = 0x00A1;
/// [`hired_check_result`] codes.
pub const HIRED_OK: u32 = 0;
/// See [`HIRED_OK`].
pub const HIRED_ALREADY_OPEN: u32 = 2;
/// See [`HIRED_OK`].
pub const HIRED_ERROR: u32 = 1;

/// `0x0181`: the cash slot and the item.
pub fn parse_hired_check(body: &[u8]) -> Option<(i16, u32)> {
    let mut c = PacketReader::new(body);
    Some((c.i16().ok()?, c.u32().ok()?))
}

/// `0x00A1` mode 0 with `code` ([`HIRED_OK`] lets the client go on to the title prompt).
pub fn hired_check_result(code: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(0);
    w.u32(code);
    w.into_vec()
}

/// Server -> client: **a hired merchant appears** - `EmployeeEnterField`. `CField::OnPacket`'s
/// default arm sends `0x0621..=0x0625` to `FUN_140d8f740(field+0xd0, ...)`, the Free Market's
/// employee pool, which handles `0x0622` (`FUN_140d8f7d0`) and `0x0623` and nothing else [L].
/// Ignored on a field that is not a market (`FUN_14182e530`). See [`employee_enter`].
pub const EMPLOYEE_ENTER: u16 = 0x0622;
/// Server -> client: **a hired merchant is gone** - `u32 id`; the pool drops the record whose
/// `+0x58` matches [L].
pub const EMPLOYEE_LEAVE: u16 = 0x0623;

/// Server -> client: **UserStoreBalloon** - `FUN_142795a70`, the user-pool row after `0x0233`
/// (`research/user-pool-tables.md`). `u32 charId` then [`sign_block`].
pub const USER_SHOP_SIGN: u16 = 0x0234;

/// The store a Store Permit opens. `0x0577` mode 2 builds the store window for 3 and 4 alike
/// (`FUN_140d91330`); the sign draws `backgrnd_PlayerShop/<permit>` for 3 (`FUN_141595e60`) [L].
pub const ROOM_TYPE_STORE: u32 = 3;
/// A hired merchant: the same window (`FUN_140d984d0` for 3 and 4 alike), which draws the
/// merchant's own figure (`UI/HiredShop.img/<permit>/employee`) in place of the owner when the
/// permit is a `503xxxx` (`FUN_140d9a090`: `FUN_140417ed0(permit) == 0xB`; `FUN_140d9f4e0` then
/// skips seat 0's avatar) [L]. Served here as a store its owner stays beside - see
/// `world::session::playershop`.
pub const ROOM_TYPE_HIRED_MERCHANT: u32 = 4;

/// The window's state `[dlg+0x2a8]` (`FUN_140d94020`). In 1 the owner may list, take back and
/// open (each checks `== 1` first, else *"Not available in the current state."*) [L].
pub const STATE_SETUP: u32 = 1;
/// Open for business. The sign's own copy must say 2 for a double-click to visit
/// (`FUN_1429b64e0`: `cmp [rax+8], 2`), and it draws `canEnter` (`FUN_141595e60`) [L].
pub const STATE_OPEN: u32 = 2;

/// Seats in a store window: the owner in 0, visitors in 1..=3 (`FUN_140d99cf0` sizes the member
/// array to four) [L].
pub const SEATS: usize = 4;
/// Lines one store may list - sent as `[dlg+0x38c]`, which the window checks before a listing
/// (`FUN_140d9bfe0`: `max <= rows` -> *"Not available in the current state."*) [L]. The number is
/// this server's.
pub const MAX_LINES: u16 = 16;

/// **How far apart two stores must stand** - the client's own spacing, enforced by the server for
/// every kind of store. Before it sends a create (`FUN_142cd4550`), the client walks the user pool
/// (`FUN_1429b5ef0`) and refuses with *"You can't open a store here"* when the new store's area
/// overlaps another player's store sign - each area `FUN_14073a9b0`'s `(-60, -120, 60, 10)` around
/// the player's feet (`0x143299f08`) [L]. Two such areas overlap exactly when the stores are less
/// than 120 px apart sideways AND less than 130 px apart up and down. The client never checks
/// hired merchants (the employee pool is not in that walk) and the server is the only check a
/// modified client cannot skip.
pub const STORE_SPACING_X: i32 = 120;
/// See [`STORE_SPACING_X`].
pub const STORE_SPACING_Y: i32 = 130;

/// Whether a store at `(ax, ay)` stands too close to one at `(bx, by)` ([`STORE_SPACING_X`]).
pub fn too_close(ax: i16, ay: i16, bx: i16, by: i16) -> bool {
    (i32::from(ax) - i32::from(bx)).abs() < STORE_SPACING_X && (i32::from(ay) - i32::from(by)).abs() < STORE_SPACING_Y
}

/// `0x0576` mode 3 reasons. The window shows *"The shop has been closed."* (`0x177B`) on closing for
/// 3, 5, 7 and 8 (`FUN_140d98ed0`) and nothing for the rest [L].
pub const LEAVE_QUIET: u32 = 0;
/// See [`LEAVE_QUIET`].
pub const LEAVE_SHOP_CLOSED: u32 = 3;

/// `0x0577` message codes (`FUN_140d91680`) [L].
pub mod room_error {
    /// `SID_ROOMSESSION_ROOMNOTFOUND`
    pub const ROOM_NOT_FOUND: u32 = 8;
    /// `SID_ROOMSESSION_NOTENABLESTATE`
    pub const NOT_ENABLED: u32 = 9;
    /// `SID_PLAYERSHOP_ERROR_DENIEDUSER` - banned from this store.
    pub const DENIED: u32 = 0xC;
    /// `SID_ROOMSESSION_FULL`
    pub const FULL: u32 = 0x12;
    /// `0x1776` *"You can't open a store here"*
    pub const NOT_HERE: u32 = 0x1C;
    /// `SID_ERROR_UNKOWN`
    pub const UNKNOWN: u32 = 2;
}

/// `0x0579` mode 3 message codes (`FUN_140da0030`) [L].
pub mod shelf_error {
    /// `SID_PLAYERSHOP_ERROR_NOSTOCK`
    pub const NO_STOCK: u32 = 6;
    /// `SID_PLAYERSHOP_ERROR_INVALIDITEM`
    pub const INVALID_ITEM: u32 = 7;
    /// `SID_PLAYERSHOP_ERROR_TRADERESTRAINTITEM`
    pub const UNTRADEABLE: u32 = 10;
    /// `SID_PLAYERSHOP_ERROR_DEINEDCASHITEM`
    pub const CASH_ITEM: u32 = 0xC;
    /// `SID_PLAYERSHOP_ERROR_SELLERSMESOLIMIT`
    pub const SELLER_MESO_LIMIT: u32 = 0xF;
    /// `SID_PLAYERSHOP_ERROR_INSUFFICIENTMESO`
    pub const NOT_ENOUGH_MESOS: u32 = 0x10;
    /// `SID_ROOMSESSION_NOTENABLESTATE`
    pub const NOT_ENABLED: u32 = 0x14;
    /// `SID_ERROR_UNKOWN`
    pub const UNKNOWN: u32 = 1;
}

/// What a store window asked of the room (`0x017F`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomRequest {
    /// A Store Permit used: `FUN_142cd4550` - `raw4 0, raw4 type, str title, u16 slot, u32 item`.
    Create { room_type: u32, title: String, permit_slot: i16, permit: u32 },
    Close,
    Visit { shop: u32 },
    Leave,
    Chat { text: String },
    Ban { seat: u16 },
    Other { mode: u32 },
}

pub fn parse_room(body: &[u8]) -> Option<RoomRequest> {
    let mut c = PacketReader::new(body);
    Some(match c.u32().ok()? {
        0 => {
            let room_type = c.u32().ok()?;
            let title = c.str().ok()?;
            let permit_slot = c.i16().ok()?;
            RoomRequest::Create { room_type, title, permit_slot, permit: c.u32().ok()? }
        }
        1 => RoomRequest::Close,
        2 => RoomRequest::Visit { shop: c.u32().ok()? },
        3 => RoomRequest::Leave,
        5 => RoomRequest::Chat { text: c.str().ok()? },
        6 => RoomRequest::Ban { seat: c.u16().ok()? },
        mode => RoomRequest::Other { mode },
    })
}

/// What a store window asked of the shelf (`0x0180`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfRequest {
    Open,
    TakeBack { row: u16 },
    /// `FUN_140d9bfe0` at `140d9c8e2`: `u8 tab, u16 slot, u16 bundles, u16 per bundle, u64 price`.
    /// A star stack is sent as one bundle of one (`140d9c937`) [L].
    List { inv_type: u8, slot: i16, bundles: u16, per_bundle: u16, price: u64 },
    /// `FUN_140d9d750` at `140d9e91e`: `u8 row, u16 bundles, u64 total, u32` [L]. The total is
    /// what the buyer's window computed - `bundles x the row's price` (`140d9e5a0`).
    Buy { row: u8, bundles: u16, total: u64 },
    Collect,
    Other { mode: u32 },
}

pub fn parse_shelf(body: &[u8]) -> Option<ShelfRequest> {
    let mut c = PacketReader::new(body);
    Some(match c.u32().ok()? {
        0 => ShelfRequest::Open,
        1 => ShelfRequest::TakeBack { row: c.u16().ok()? },
        2 => {
            let inv_type = c.u8().ok()?;
            let slot = c.i16().ok()?;
            let bundles = c.u16().ok()?;
            let per_bundle = c.u16().ok()?;
            ShelfRequest::List { inv_type, slot, bundles, per_bundle, price: c.u64().ok()? }
        }
        3 => {
            let row = c.u8().ok()?;
            let bundles = c.u16().ok()?;
            ShelfRequest::Buy { row, bundles, total: c.u64().ok()? }
        }
        4 => ShelfRequest::Collect,
        mode => ShelfRequest::Other { mode },
    })
}

/// One seat of a store window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub character_id: u32,
    pub name: String,
    /// `crate::opcode::avatar_look` - the window draws it as `avatar<seat>`. Empty sends
    /// `hasLook = 0`: a hired merchant's owner, away from it, is a name with no figure.
    pub look: Vec<u8>,
}

/// The seats: `u32 count`, then per seat `FUN_14073a430` -
/// `u32 id, u32, str name, u8 hasLook, [avatar look]` (`FUN_1402ee8d0`) [L]. An empty seat is id 0.
///
/// **All four seats, always.** `FUN_140d923e0` fills seat `i` from the `i`-th entry and **returns
/// early** when the count passes the window's four - leaving the rest of the packet unread, which
/// for the window-open body is the shelf.
fn seats(w: &mut PacketWriter, seats: &[Option<Member>; SEATS]) {
    w.u32(SEATS as u32);
    for seat in seats {
        match seat {
            Some(m) => {
                w.u32(m.character_id);
                w.u32(0); // +4, cleared with the id when a seat empties; nothing read here draws it
                w.str(&crate::notice::ascii_fold(&m.name));
                w.u8(u8::from(!m.look.is_empty()));
                w.bytes(&m.look);
            }
            None => {
                w.u32(0);
                w.u32(0);
                w.str("");
                w.u8(0);
            }
        }
    }
}

/// One row of the shelf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// The tab it came from.
    pub inv_type: u8,
    /// Bundles left. **0 draws the row SOLD OUT**: the row widget (`FUN_140d94d20`) lays the
    /// `soldout` canvas over a row whose `+0x11b0` (this field) is below 1, still with its price,
    /// and the selection refresh (`FUN_140d9ccb0`) will not select it [L].
    pub bundles: u32,
    pub per_bundle: u32,
    /// Mesos a bundle.
    pub price: u64,
    /// A whole `GW_ItemSlot`, type byte first (`world::session::Session::item_blob`).
    pub item: Vec<u8>,
}

/// The shelf: `u16 count`, then per row `FUN_140429ce0` -
/// `u16, u16, u32 bundles, u32 per bundle, u64 price, u8 hasItem, [GW_ItemSlot]` [L]. The window's
/// buy dialog offers up to `bundles x per bundle`, takes multiples of `per bundle`, and prices
/// `count / per bundle x price` (`FUN_140d9d750`); the row's label is *"%d for %s mesos"* with the
/// per-bundle count and the price (`FUN_140d94d20`). The two leading `u16`s are not read by
/// anything on those paths; the tab goes in the first.
fn rows(w: &mut PacketWriter, rows: &[Row]) {
    w.u16(rows.len() as u16);
    for r in rows {
        w.u16(u16::from(r.inv_type));
        w.u16(0);
        w.u32(r.bundles);
        w.u32(r.per_bundle);
        w.u64(r.price);
        w.u8(1);
        w.bytes(&r.item);
    }
}

/// Everything the store window is opened with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub room_type: u32,
    /// The owner's character id - also the sign's id, which a double-click sends back.
    pub shop_id: u32,
    pub title: String,
    pub state: u32,
    pub owner_id: u32,
    /// Compared with the viewer's own account (`ctx+0x2204`, `FUN_140d9a330`) for the "collect"
    /// button.
    pub owner_account: u32,
    pub owner_name: String,
    /// The Store Permit's item id. A hired merchant's (`FUN_140417ed0 == 0xB`) draws an employee.
    pub permit: u32,
    pub seats: [Option<Member>; SEATS],
    pub rows: Vec<Row>,
}

/// `0x0577` mode 2, result 0: **open the store window.** `FUN_140d91330` [L]:
///
/// ```text
/// u32 2  u32 result 0
/// u32 type           3 or 4 build the store window; anything else builds nothing
/// u32 shop id        -> ctor, [dlg+0x298]
/// str title          -> ctor, [dlg+0x2a0]
/// raw4 state         -> [dlg+0x2a8]
/// -- vt+0x170 = FUN_140d9a090:
/// u32 owner id       -> [dlg+0x378]  the window is the owner's when this is the viewer
/// u32 owner account  -> [dlg+0x37c]
/// str owner name     -> [dlg+0x380]
/// u32 permit item    -> [dlg+0x388]
/// u32 max lines      -> [dlg+0x38c]
/// seats              vt+0x178 -> FUN_140d923e0
/// shelf              FUN_140d9abc0
/// ```
///
/// Nothing is read when a store window is already open (`DAT_143aa8520 != 0`).
pub fn open_window(win: &Window) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(2);
    w.u32(0);
    w.u32(win.room_type);
    w.u32(win.shop_id);
    w.str(&crate::notice::ascii_fold(&win.title));
    w.u32(win.state);
    w.u32(win.owner_id);
    w.u32(win.owner_account);
    w.str(&crate::notice::ascii_fold(&win.owner_name));
    w.u32(win.permit);
    w.u32(u32::from(MAX_LINES));
    seats(&mut w, &win.seats);
    rows(&mut w, &win.rows);
    w.into_vec()
}

/// `0x0577` mode 2 with a nonzero result: no window, the result's message ([`room_error`]).
pub fn enter_refused(code: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(2);
    w.u32(code);
    w.into_vec()
}

/// `0x0577` mode 1: close the window (`vt+0x138(2)`), and release the latch.
pub fn close_window() -> Vec<u8> {
    1u32.to_le_bytes().to_vec()
}

/// `0x0577` mode 4: the window's state ([`STATE_OPEN`]), and release the latch.
pub fn state(state: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(4);
    w.u32(state);
    w.into_vec()
}

/// `0x0577` mode 6: nothing but the latch release.
pub fn release() -> Vec<u8> {
    6u32.to_le_bytes().to_vec()
}

/// `0x0576` mode 0: the seats changed - somebody came in. A seat whose id changed posts
/// *"%s has entered."* in the window's chat (`SID_MAPLECHAT_USERENTER`).
pub fn seats_changed(s: &[Option<Member>; SEATS]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(0);
    seats(&mut w, s);
    w.into_vec()
}

/// `0x0576` mode 2: a chat line. `FUN_14073a8d0`: `u32 seat, u32, u32 character id, str name,
/// str text` [L]; the window colours by seat (`vt+0x1b8`), drops a line from a blocked character
/// (`FUN_142d01050`), and writes `name : text`.
pub fn chat(seat: u8, character_id: u32, name: &str, text: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(2);
    w.u32(u32::from(seat));
    w.u32(0);
    w.u32(character_id);
    w.str(&crate::notice::ascii_fold(name));
    w.str(&crate::notice::ascii_fold(text));
    w.into_vec()
}

/// `0x0576` mode 3: seat `seat` is empty now. `FUN_140d925e0` [L]: the reader's OWN seat closes
/// the reader's window (storing `reason`, and releasing the latch); anyone else's posts *"%s has
/// left."* and clears it.
pub fn seat_left(seat: u8, reason: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(3);
    w.u32(u32::from(seat));
    w.u32(reason);
    w.into_vec()
}

/// `0x0578` mode 0: the shelf, unasked - after somebody else's purchase.
pub fn shelf(r: &[Row]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(0);
    rows(&mut w, r);
    w.into_vec()
}

/// `0x0579` mode 1 (a listing or a take-back) or 2 (a purchase): it went through, here is the
/// shelf. Releases the latch.
pub fn shelf_done(bought: bool, r: &[Row]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(if bought { 2 } else { 1 });
    w.u8(1);
    rows(&mut w, r);
    w.into_vec()
}

/// `0x0579` mode 0 with code 0: **nothing to say, and the window let go.** `FUN_140d9fe50` reads
/// the code, finds no message for 0 (`FUN_140d91680` case 0), releases the request latch and
/// redraws; the opcode's own handler (`FUN_140d94bb0`) then calls `FUN_142aa2810(4)`.
///
/// **That second call is the one that matters.** "Open Store" (`FUN_140d99620`) and a purchase
/// (`FUN_140d9d750`) each call `FUN_142aa27f0(4)` after sending - an id into a set that blocks the
/// whole UI while it is non-empty - and `FUN_142aa2810(4)` is the only removal, reached from
/// `0x0579` and nothing else in the store [L]. The owner, 2026-10-04: *"the player also loses all
/// ability to close the shop. The button becomes not clickable and the UI is completely
/// unresponsive"* - Open Store had been answered with `0x0577` mode 4 alone.
pub fn shelf_unlock() -> Vec<u8> {
    vec![0; 8]
}

/// `0x0579` mode 3: a shelf request refused, with the store's message ([`shelf_error`]).
/// Releases the latch.
pub fn shelf_refused(code: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(3);
    w.u32(code);
    w.into_vec()
}

/// What the sign over a store owner's head says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sign {
    pub room_type: u32,
    pub shop_id: u32,
    /// [`STATE_OPEN`] lets a double-click visit and draws `canEnter`.
    pub state: u32,
    pub title: String,
    /// The permit, which picks the sign's art: `backgrnd_PlayerShop/5140001` .. `5140004` in
    /// `UI/RoomSessionBalloon.img` [L].
    pub permit: u32,
}

/// The store block - `FUN_14073a7b0` into `user+0x1148`, the same 18-or-more bytes in `0x0234` and
/// at `0x0224`'s offset 455 (`research/user-enter-field.md`). **Every field is read whatever the
/// type** - the reader has no branch - so "no store" is all five, zeroed [L]:
///
/// ```text
/// raw4 type    +0x1148   0 = no sign
/// u32  id      +0x114c   sent back by a double-click (FUN_14276f7a0)
/// raw4 state   +0x1150   2 = open: the click visits, the sign draws canEnter
/// str  title   +0x1158
/// u32  permit  +0x1160   the sign's art
/// ```
pub fn sign_block(w: &mut PacketWriter, s: Option<&Sign>) {
    match s {
        Some(s) => {
            w.u32(s.room_type);
            w.u32(s.shop_id);
            w.u32(s.state);
            w.str(&crate::notice::ascii_fold(&s.title));
            w.u32(s.permit);
        }
        None => {
            w.u32(0);
            w.u32(0);
            w.u32(0);
            w.str("");
            w.u32(0);
        }
    }
}

/// One hired merchant on the map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    /// The pool's key, `+0x58` - and what a double-click sends back as `0x017F` mode 2
    /// (`FUN_140d90060` returns it). The owner's character id, the store's id.
    pub id: u32,
    /// The Hired Merchant item, `+0x5c`: the figure is `UI/HiredShop.img/<item>/employee` and its
    /// balloon `UI/HiredShop.img/<item>` (`FUN_140d8df90`, `FUN_140d8e230`) [L].
    pub item: u32,
    /// `+0x60`.
    pub owner_name: String,
    /// `+0x68`, drawn in the balloon.
    pub title: String,
    pub x: i16,
    pub y: i16,
    /// `+0x70`: [`STATE_OPEN`] draws `canEnter`, anything else `cannotEnter` [L].
    pub state: u32,
}

/// `0x0622`: `FUN_140d8f7d0` - `u32 id, u32 item, str owner, str title, i16 x, i16 y, raw4 state`,
/// in that order on the listing (`140d8f80a` .. `140d8f8a8`) [L]. A record with the same id is
/// replaced, so this is also how a merchant's state changes.
pub fn employee_enter(e: &Employee) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(e.id);
    w.u32(e.item);
    w.str(&crate::notice::ascii_fold(&e.owner_name));
    w.str(&crate::notice::ascii_fold(&e.title));
    w.i16(e.x);
    w.i16(e.y);
    w.u32(e.state);
    w.into_vec()
}

/// `0x0623`: the merchant `id` is gone.
pub fn employee_leave(id: u32) -> Vec<u8> {
    id.to_le_bytes().to_vec()
}

/// `0x0234` for `character`: their store's sign, or none (`None` takes it down).
pub fn sign(character: u32, s: Option<&Sign>) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character);
    sign_block(&mut w, s);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The create the owner's client actually sent, 2026-10-04 (`world-ch0.log` 18:14:31).
    #[test]
    fn the_captured_create_parses() {
        let body = hex("00000000030000000700676172626167650f00216e4e00");
        assert_eq!(
            parse_room(&body),
            Some(RoomRequest::Create { room_type: 3, title: "garbage".into(), permit_slot: 15, permit: 5_140_001 })
        );
        assert_eq!(parse_room(&hex("0200000037000000")), Some(RoomRequest::Visit { shop: 55 }));
        assert_eq!(parse_room(&hex("060000000200")), Some(RoomRequest::Ban { seat: 2 }));
    }

    #[test]
    fn stores_closer_than_the_clients_own_areas_are_too_close() {
        assert!(too_close(0, 0, 119, 0));
        assert!(!too_close(0, 0, 120, 0), "areas of +-60 just touch at 120");
        assert!(too_close(0, 0, -119, 129));
        assert!(!too_close(0, 0, 0, 130), "a level above");
    }

    /// The hired-merchant check the owner's client sent nine times, 2026-10-04 20:47.
    #[test]
    fn the_captured_hired_check_parses_and_the_yes_is_two_zero_words() {
        assert_eq!(parse_hired_check(&hex("100070c04c00")), Some((16, 5_030_000)));
        assert_eq!(hired_check_result(HIRED_OK), vec![0; 8]);
        assert_eq!(hired_check_result(HIRED_ALREADY_OPEN), hex("0000000002000000"));
    }

    #[test]
    fn the_shelf_requests_parse_in_the_clients_order() {
        let mut w = PacketWriter::new();
        w.u32(2).u8(2).i16(7).u16(3).u16(10).u64(1500);
        assert_eq!(
            parse_shelf(w.as_slice()),
            Some(ShelfRequest::List { inv_type: 2, slot: 7, bundles: 3, per_bundle: 10, price: 1500 })
        );
        let mut w = PacketWriter::new();
        w.u32(3).u8(1).u16(2).u64(3000).u32(0xdead);
        assert_eq!(parse_shelf(w.as_slice()), Some(ShelfRequest::Buy { row: 1, bundles: 2, total: 3000 }));
        assert_eq!(parse_shelf(&hex("010000000400")), Some(ShelfRequest::TakeBack { row: 4 }));
        assert_eq!(parse_shelf(&hex("00000000")), Some(ShelfRequest::Open));
    }

    /// The window-open body, field by field in `FUN_140d91330` / `FUN_140d9a090`'s order, with
    /// all four seats written and one row.
    #[test]
    fn the_window_opens_in_the_readers_order() {
        let win = Window {
            room_type: ROOM_TYPE_STORE,
            shop_id: 215,
            title: "Shop".into(),
            state: STATE_SETUP,
            owner_id: 215,
            owner_account: 9,
            owner_name: "Wisp".into(),
            permit: 5_140_001,
            seats: [Some(Member { character_id: 215, name: "Wisp".into(), look: vec![0xAA; 3] }), None, None, None],
            rows: vec![Row { inv_type: 2, bundles: 5, per_bundle: 10, price: 300, item: vec![2, 0xBB] }],
        };
        let b = open_window(&win);
        let mut c = PacketReader::new(&b);
        assert_eq!((c.u32().unwrap(), c.u32().unwrap(), c.u32().unwrap(), c.u32().unwrap()), (2, 0, 3, 215));
        assert_eq!(c.str().unwrap(), "Shop");
        assert_eq!((c.u32().unwrap(), c.u32().unwrap(), c.u32().unwrap()), (STATE_SETUP, 215, 9));
        assert_eq!(c.str().unwrap(), "Wisp");
        assert_eq!((c.u32().unwrap(), c.u32().unwrap()), (5_140_001, 16));
        assert_eq!(c.u32().unwrap(), 4, "every seat");
        assert_eq!((c.u32().unwrap(), c.u32().unwrap()), (215, 0));
        assert_eq!(c.str().unwrap(), "Wisp");
        assert_eq!(c.u8().unwrap(), 1);
        assert_eq!(c.bytes(3).unwrap(), &[0xAA; 3]);
        for _ in 1..SEATS {
            assert_eq!((c.u32().unwrap(), c.u32().unwrap(), c.str().unwrap(), c.u8().unwrap()), (0, 0, String::new(), 0));
        }
        assert_eq!(c.u16().unwrap(), 1, "one row");
        assert_eq!((c.u16().unwrap(), c.u16().unwrap()), (2, 0));
        assert_eq!((c.u32().unwrap(), c.u32().unwrap(), c.u64().unwrap(), c.u8().unwrap()), (5, 10, 300, 1));
        assert_eq!(c.bytes(2).unwrap(), &[2, 0xBB]);
        assert_eq!(c.remaining(), 0);
    }

    #[test]
    fn a_merchant_enters_in_the_readers_order() {
        let e = Employee { id: 215, item: 5_030_000, owner_name: "Wisp".into(), title: "Elf".into(), x: -5, y: 34, state: STATE_OPEN };
        let b = employee_enter(&e);
        let mut c = PacketReader::new(&b);
        assert_eq!((c.u32().unwrap(), c.u32().unwrap()), (215, 5_030_000));
        assert_eq!((c.str().unwrap(), c.str().unwrap()), ("Wisp".to_string(), "Elf".to_string()));
        assert_eq!((c.i16().unwrap(), c.i16().unwrap(), c.u32().unwrap()), (-5, 34, 2));
        assert_eq!(c.remaining(), 0);
        assert_eq!(employee_leave(215), 215u32.to_le_bytes());
    }

    /// The sign is the same 18 bytes empty or full apart from the title, and every field is
    /// written either way.
    #[test]
    fn the_sign_writes_every_field() {
        assert_eq!(sign(215, None), [&215u32.to_le_bytes()[..], &[0; 12], &[0, 0], &[0; 4]].concat());
        let s = Sign { room_type: 3, shop_id: 215, state: STATE_OPEN, title: "Hi".into(), permit: 5_140_001 };
        let b = sign(215, Some(&s));
        assert_eq!(b.len(), 4 + 18 + 2);
        assert_eq!(&b[4..16], &[3, 0, 0, 0, 215, 0, 0, 0, 2, 0, 0, 0]);
        assert_eq!(&b[16..20], b"\x02\x00Hi");
        assert_eq!(&b[20..], &5_140_001u32.to_le_bytes());
    }

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }
}
