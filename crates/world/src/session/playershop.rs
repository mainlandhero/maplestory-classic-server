//! **Player stores**: the room, its sign on the map, and the shelf.
//!
//! The owner, 2026-10-04: *"I just tried to open a player store, but nothing happened. Might need to
//! decompile everything and make it work."* `net::playershop` carries the wire format and where
//! each field was read; this is the join.
//!
//! * **Create** - a Store Permit used (`0x017F` mode 0). The owner's window opens in setup
//!   ([`net::playershop::STATE_SETUP`]) and the sign goes up over their head, drawn
//!   `cannotEnter` until the store opens.
//! * **List / take back** (`0x0180` modes 2 / 1) - only in setup, as the window itself enforces.
//!   An item listed leaves the bag for `store::playershop`'s escrow, so a crash cannot lose it.
//! * **Open** (`0x0180` mode 0) - the window and the sign say open; a double-click on the sign
//!   now visits (`0x017F` mode 2).
//! * **Buy** (`0x0180` mode 3) - one transaction moves the units, the price, and the price less
//!   the window's own 3% fee to the owner. Everybody in the store sees the shelf change.
//! * **Close** - the owner's "close store" (`0x017F` mode 1), the owner's window closing
//!   (mode 3), or the owner leaving the channel: every line goes back to the owner's bag, every
//!   visitor's window closes with *"The shop has been closed."*, and the sign comes down.
//!
//! **Hired merchants** (2026-10-04, later: *"I just tried double clicking the elf hired merchant and it
//! doesn't do anything."*, then *"We need to make it stay behind. Also it needs to be setup every 24
//! hours or it will be automatically removed"*). The double-click asks first (`0x0181`); a yes
//! (`0x00A1`) sends the client on to the title prompt and a create with room type 4, which opens the
//! same window with the merchant's figure in the owner's seat. On the map the merchant is an
//! **employee** (`0x0622` / `0x0623`, the Free Market's employee pool), not a sign over the owner:
//!
//! * it stands where the owner set it up, **whether or not the owner is there** - once it has been
//!   opened, the owner closing the window, changing map or leaving the game leaves it selling;
//! * it is a row in `store::playershop`'s `hired_merchants`, so it is restored on its channel after
//!   a server restart, and its shelf stays out of the owner's bag at login while it stands;
//! * a sale pays the owner's wallet at once, online or not, less the 3% fee;
//! * the owner double-clicking it puts it in **maintenance** (setup: list, take back, open again;
//!   visitors are put out); "close store" closes it and the shelf comes home;
//! * **24 hours after setup it closes by itself** (`store::playershop::HIRED_MERCHANT_SECS`, the
//!   item's own words), checked every few seconds by whichever session ticks: the shelf goes back
//!   to the owner's bag now if they are on this channel, else at their next login.
//!
//! The room lives beside the trade and game rooms (`Rooms::shops`), keyed by the owner's
//! character id - which is also the id the sign carries and a double-click sends back.

use super::{Reply, Session};
use net::playershop as wire;

/// One seat of a store.
#[derive(Debug, Clone)]
struct ShopSeat {
    character_id: u32,
    name: String,
    look: Vec<u8>,
}

/// One open store.
#[derive(Debug, Clone)]
pub(crate) struct ShopRoom {
    /// The owner's character id.
    id: u32,
    /// [`wire::ROOM_TYPE_STORE`] or [`wire::ROOM_TYPE_HIRED_MERCHANT`].
    room_type: u32,
    owner_account: u32,
    owner_name: String,
    title: String,
    permit: u32,
    state: u32,
    map: crate::fields::FieldKey,
    /// The owner in 0, visitors in 1..=3. A hired merchant's seat 0 is empty while its owner is away.
    seats: [Option<ShopSeat>; wire::SEATS],
    /// Visitors the owner banned - refused at the door until the store closes.
    banned: Vec<u32>,
    /// Opened at least once. A hired merchant whose owner walks away before ever opening it is
    /// cancelled; one that has been open stays.
    ever_opened: bool,
    /// Where a hired merchant stands.
    x: i16,
    y: i16,
    /// Unix seconds - a hired merchant's 24 hours run from here.
    opened_at: i64,
}

impl ShopRoom {
    fn seat_of(&self, character: u32) -> Option<usize> {
        self.seats.iter().position(|s| s.as_ref().is_some_and(|s| s.character_id == character))
    }

    fn members(&self) -> [Option<wire::Member>; wire::SEATS] {
        let mut m = self.seats.clone().map(|s| s.map(|s| wire::Member { character_id: s.character_id, name: s.name, look: s.look }));
        // An away owner is still the merchant's owner: a name, no figure.
        if m[0].is_none() {
            m[0] = Some(wire::Member { character_id: self.id, name: self.owner_name.clone(), look: Vec::new() });
        }
        m
    }

    fn is_merchant(&self) -> bool {
        self.room_type == wire::ROOM_TYPE_HIRED_MERCHANT
    }

    fn employee(&self) -> wire::Employee {
        wire::Employee {
            id: self.id,
            item: self.permit,
            owner_name: self.owner_name.clone(),
            title: self.title.clone(),
            x: self.x,
            y: self.y,
            state: self.state,
        }
    }

    fn expired(&self, now: i64) -> bool {
        self.is_merchant() && now >= self.opened_at + store::playershop::HIRED_MERCHANT_SECS
    }

    fn occupants(&self) -> Vec<u32> {
        self.seats.iter().flatten().map(|s| s.character_id).collect()
    }

    fn sign(&self) -> wire::Sign {
        wire::Sign { room_type: self.room_type, shop_id: self.id, state: self.state, title: self.title.clone(), permit: self.permit }
    }

}

/// Unix seconds, for a hired merchant's 24 hours.
fn unix_now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

/// How often a session looks for hired merchants whose 24 hours are up.
const MERCHANT_CHECK: std::time::Duration = std::time::Duration::from_secs(10);

/// Is `item_id` the item that opens a room of `room_type`: a Store Permit (`514xxxx`, the family
/// the sign has art for) for a store, a Hired Merchant (`503xxxx`) for a hired merchant.
fn is_permit_for(room_type: u32, item_id: u32) -> bool {
    match room_type {
        wire::ROOM_TYPE_STORE => item_id / 10_000 == 514,
        wire::ROOM_TYPE_HIRED_MERCHANT => item_id / 10_000 == 503,
        _ => false,
    }
}

impl Session {
    fn with_shops<T>(&self, f: impl FnOnce(&mut Vec<ShopRoom>) -> T) -> T {
        f(&mut self.fields.trades().shops)
    }

    /// The store `character` is in, as `(store id, seat)`.
    fn shop_seat_of(&self, character: u32) -> Option<(u32, usize)> {
        self.with_shops(|s| s.iter().find_map(|r| Some((r.id, r.seat_of(character)?))))
    }

    /// Whether this character is in a store, their own or somebody else's - for the busy check.
    pub(super) fn in_shop(&self, character: u32) -> bool {
        self.shop_seat_of(character).is_some()
    }

    /// The sign this character's own store puts over their head, for their `0x0224`.
    pub(super) fn own_store_sign(&self, character: u32) -> Option<wire::Sign> {
        self.with_shops(|s| s.iter().find(|r| r.id == character && !r.is_merchant()).map(ShopRoom::sign))
    }

    /// Whether `owner` has a hired merchant standing - on any channel, from the row.
    fn live_merchant(&self, owner: u32) -> bool {
        self.store.hired_merchant(owner).ok().flatten().is_some_and(|m| !m.expired(unix_now()))
    }

    /// The hired merchants standing on this channel, from their rows - once per channel process,
    /// by the first session that asks. One that expired while the server was down, or has nothing
    /// on its shelf, is closed instead; its shelf comes home at its owner's next login.
    fn ensure_merchants_loaded(&mut self) {
        if std::mem::replace(&mut self.fields.trades().merchants_loaded, true) {
            return;
        }
        let rows = match self.store.hired_merchants_on_channel(self.config.channel_id) {
            Ok(r) => r,
            Err(e) => {
                crate::server::log(&format!("   store: hired merchants for channel {} did not load ({e}).", self.config.channel_id));
                return;
            }
        };
        let now = unix_now();
        for m in rows {
            let empty = !self.store.shop_has_stock(m.owner_id).unwrap_or(false);
            if m.expired(now) || empty {
                let _ = self.store.close_hired_merchant(m.owner_id);
                crate::server::log(&format!(
                    "   store: hired merchant {} {:?} not restored - {}; its shelf goes home at the owner's next login.",
                    m.owner_id,
                    m.title,
                    if empty { "nothing left for sale" } else { "its 24 hours ran out while the server was down" }
                ));
                continue;
            }
            crate::server::log(&format!("   store: hired merchant {} {:?} restored on map {} at ({}, {}).", m.owner_id, m.title, m.map, m.x, m.y));
            let room = ShopRoom {
                id: m.owner_id,
                room_type: wire::ROOM_TYPE_HIRED_MERCHANT,
                owner_account: m.owner_account,
                owner_name: m.owner_name,
                title: m.title,
                permit: m.permit,
                state: wire::STATE_OPEN,
                map: crate::fields::FieldKey { map: m.map, instance: m.instance },
                seats: [None, None, None, None],
                banned: Vec::new(),
                ever_opened: true,
                x: m.x,
                y: m.y,
                opened_at: m.opened_at,
            };
            self.with_shops(|s| {
                s.retain(|r| r.id != room.id);
                s.push(room);
            });
        }
    }

    /// The hired merchants on `map`, for somebody arriving there - the client rebuilds its employee
    /// pool on every field entry, like every other pool.
    pub(super) fn merchants_on_entry(&mut self, map: crate::fields::FieldKey) -> Vec<Reply> {
        self.ensure_merchants_loaded();
        let here: Vec<wire::Employee> = self.with_shops(|s| s.iter().filter(|r| r.is_merchant() && r.map == map).map(ShopRoom::employee).collect());
        here.into_iter()
            .map(|e| Reply {
                opcode: wire::EMPLOYEE_ENTER,
                what: format!("EmployeeEnterField: {}'s hired merchant {:?} at ({}, {}), {}", e.owner_name, e.title, e.x, e.y, if e.state == wire::STATE_OPEN { "open" } else { "in maintenance" }),
                body: wire::employee_enter(&e),
            })
            .collect()
    }

    /// `0x0622` (`Some`) or `0x0623` (`None`) to everyone on a hired merchant's map.
    fn show_employee(&self, map: crate::fields::FieldKey, id: u32, e: Option<&wire::Employee>) {
        let reply = match e {
            Some(e) => Reply {
                opcode: wire::EMPLOYEE_ENTER,
                body: wire::employee_enter(e),
                what: format!("EmployeeEnterField: hired merchant {id} {:?}, {}", e.title, if e.state == wire::STATE_OPEN { "open" } else { "in maintenance" }),
            },
            None => Reply { opcode: wire::EMPLOYEE_LEAVE, body: wire::employee_leave(id), what: format!("EmployeeLeaveField: hired merchant {id} is gone") },
        };
        let n = self.bus().publish_to_map(map, reply);
        crate::server::log(&format!("   store: hired merchant {id} shown to {n} player(s) on the map."));
    }

    /// **Hired merchants whose 24 hours are up close** - every [`MERCHANT_CHECK`], from `tick`.
    pub(super) fn merchant_tick(&mut self) -> Vec<Reply> {
        {
            let mut rooms = self.fields.trades();
            if rooms.merchant_checked_at.is_some_and(|t| t.elapsed() < MERCHANT_CHECK) {
                return Vec::new();
            }
            rooms.merchant_checked_at = Some(std::time::Instant::now());
        }
        self.ensure_merchants_loaded();
        self.expire_merchants(unix_now())
    }

    /// Close every hired merchant on this channel that has stood its 24 hours at `now`.
    fn expire_merchants(&mut self, now: i64) -> Vec<Reply> {
        let due: Vec<ShopRoom> = self.with_shops(|s| {
            let (gone, keep): (Vec<_>, Vec<_>) = std::mem::take(s).into_iter().partition(|r| r.expired(now));
            *s = keep;
            gone
        });
        let mut out = Vec::new();
        for room in due {
            out.extend(self.close_room(room, "stood its 24 hours", None));
        }
        out
    }

    /// **A store or hired merchant closes**: everybody inside but `skip` is put out with *"The shop
    /// has been closed."*, the sign or the merchant comes down, and the shelf goes home - into the
    /// owner's bag now when the owner is this session or on this channel, else at their next login
    /// (a bag another channel's session is drawing is not one to change under it). `room` has
    /// already been taken out of the table.
    fn close_room(&mut self, room: ShopRoom, why: &str, skip: Option<u32>) -> Vec<Reply> {
        let me = self.claimed_character().map_or(0, |c| c.id);
        let shop = room.id;
        let mut out = Vec::new();
        crate::server::log(&format!("   store: {} {shop} {:?} CLOSED - {why}.", if room.is_merchant() { "hired merchant" } else { "store" }, room.title));
        for (i, s) in room.seats.iter().enumerate() {
            if let Some(s) = s.as_ref().filter(|s| Some(s.character_id) != skip) {
                self.to_seat(me, s.character_id, wire::SHOP_EVENT, wire::seat_left(i as u8, wire::LEAVE_SHOP_CLOSED), format!("StoreEvent mode 3: store {shop} closed - {} (seat {i}) out, \"The shop has been closed.\"", s.name), &mut out);
            }
        }
        if room.is_merchant() {
            let _ = self.store.close_hired_merchant(shop);
            self.show_employee(room.map, shop, None);
        } else {
            self.show_store_sign(room.map, shop, None);
            self.store_owner_spawn_changed(shop);
        }
        if shop == me {
            out.extend(self.store_give_back(shop, &format!("had their store close ({why})")));
        } else if self.bus().character_online(shop) {
            let mut theirs = self.store_give_back(shop, &format!("had their hired merchant close ({why})"));
            let kind = if room.is_merchant() { "Hired Merchant" } else { "store" };
            theirs.extend(self.notice(format!("Your {kind} {:?} has closed ({why}); anything it did not sell is back in your inventory.", room.title)));
            for r in theirs {
                let _ = self.bus().publish_to_character_anywhere(shop, r);
            }
        } else {
            crate::server::log(&format!("   store: owner {shop} is not on this channel; the shelf stays held and comes home at their next login."));
        }
        out
    }

    /// The shelf as the window draws it.
    fn shelf_rows(&self, owner: u32) -> Vec<wire::Row> {
        match self.store.shop_lines(owner) {
            Ok(lines) => lines
                .iter()
                .map(|l| wire::Row {
                    inv_type: l.inv_type.as_u8(),
                    bundles: u32::from(l.bundles),
                    per_bundle: u32::from(l.per_bundle),
                    price: l.price,
                    item: self.item_blob(&l.item),
                })
                .collect(),
            Err(e) => {
                crate::server::log(&format!("   store: store {owner}'s shelf did not load ({e}); drawn empty."));
                Vec::new()
            }
        }
    }

    /// A packet to one seat: our own comes back in the returned vector, anyone else's goes
    /// through the bus.
    fn to_seat(&self, me: u32, to: u32, opcode: u16, body: Vec<u8>, what: String, out: &mut Vec<Reply>) {
        let reply = Reply { opcode, body, what };
        if to == me {
            out.push(reply);
        } else if !self.bus().publish_to_character_anywhere(to, reply) {
            crate::server::log(&format!("   store: character {to} was not reachable for a store packet."));
        }
    }

    /// `0x0234` to everyone on the store's map, the owner included.
    fn show_store_sign(&self, map: crate::fields::FieldKey, owner: u32, sign: Option<&wire::Sign>) {
        let what = match sign {
            Some(s) => format!(
                "UserStoreBalloon: character {owner}'s store {:?}, {}",
                s.title,
                if s.state == wire::STATE_OPEN { "open" } else { "setting up" }
            ),
            None => format!("UserStoreBalloon: character {owner}'s store is gone - sign down"),
        };
        let n = self.bus().publish_to_map(map, Reply { opcode: wire::USER_SHOP_SIGN, body: wire::sign(owner, sign), what });
        crate::server::log(&format!("   store: sign for store {owner} sent to {n} player(s) on the map."));
    }

    /// The owner's cached `0x0224` carries the sign: rebuilt here when it is ours, through
    /// [`crate::broadcast::Event::MiniRoomChanged`] when it is not.
    fn store_owner_spawn_changed(&mut self, owner: u32) {
        if self.claimed_character().is_some_and(|c| c.id == owner) {
            self.refresh_own_spawn();
        } else {
            self.bus().publish_event_to_character(owner, crate::broadcast::Event::MiniRoomChanged);
        }
    }

    /// `0x0181`: **may this character put up a hired merchant?** Yes when the Cash slot holds the
    /// Hired Merchant it names and they have no store open; the client then asks for a title and
    /// sends the create. Always answered - `0x00A1` releases the request latch either way.
    pub(super) fn on_hired_check(&mut self, body: &[u8]) -> Vec<Reply> {
        let answer = |code: u32, why: String| {
            crate::server::log(&format!("   store: hired-merchant check answered {code} - {why}."));
            vec![Reply {
                opcode: wire::HIRED_CHECK_RESULT,
                body: wire::hired_check_result(code),
                what: format!("HiredMerchantCheck: {} ({why})", if code == wire::HIRED_OK { "yes, ask for a title" } else { "no" }),
            }]
        };
        let Some(chr) = self.claimed_character() else { return answer(wire::HIRED_ERROR, "no character".into()) };
        let Some((slot, item)) = wire::parse_hired_check(body) else {
            return answer(wire::HIRED_ERROR, format!("a {} byte body that did not decode", body.len()));
        };
        if self.with_shops(|s| s.iter().any(|r| r.id == chr.id)) || self.live_merchant(chr.id) {
            return answer(wire::HIRED_ALREADY_OPEN, format!("character {} already has a store or a hired merchant open", chr.id));
        }
        if let Some(why) = self.busy_for_trade() {
            return answer(wire::HIRED_ERROR, format!("character {} is busy: {why}", chr.id));
        }
        let held = u16::try_from(slot).ok().and_then(|s| self.store.inventory_slot(chr.id, store::InventoryType::Cash, s).ok().flatten());
        if !is_permit_for(wire::ROOM_TYPE_HIRED_MERCHANT, item) || held.map(|i| i.item_id) != Some(item) {
            return answer(wire::HIRED_ERROR, format!("Cash slot {slot} does not hold Hired Merchant {item}"));
        }
        answer(wire::HIRED_OK, format!("character {} may put up Hired Merchant {item} from Cash slot {slot}", chr.id))
    }

    // -----------------------------------------------------------------------------------
    // 0x017F - the room
    // -----------------------------------------------------------------------------------

    /// `0x017F` - the store room.
    pub(super) fn on_shop_room(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            crate::server::log("   store: 0x017F with no claimed character; the latch is released and nothing else happens.");
            return self.store_release("no character");
        };
        let Some(req) = wire::parse_room(body) else {
            crate::server::log(&format!("   store: 0x017F did not decode ({} byte body); the latch is released.", body.len()));
            return self.store_release("an undecodable request");
        };
        match req {
            wire::RoomRequest::Create { room_type, title, permit_slot, permit } => self.store_create(&chr, room_type, title, permit_slot, permit),
            wire::RoomRequest::Visit { shop } => self.store_visit(&chr, shop),
            wire::RoomRequest::Close => self.store_leave(&chr, "pressed Close Store", true),
            wire::RoomRequest::Leave => self.store_leave(&chr, "closed the store window", false),
            wire::RoomRequest::Chat { text } => self.store_chat(&chr, &text),
            wire::RoomRequest::Ban { seat } => self.store_ban(&chr, seat),
            wire::RoomRequest::Other { mode } => {
                crate::server::log(&format!("   store: character {} sent store room mode {mode}, which is not handled; the latch is released.", chr.id));
                self.store_release("an unhandled mode")
            }
        }
    }

    /// `0x0577` mode 6: nothing but the request latch's release.
    fn store_release(&self, why: &str) -> Vec<Reply> {
        vec![Reply { opcode: wire::SHOP_ROOM, body: wire::release(), what: format!("StoreRoom mode 6: release the request latch ({why})") }]
    }

    fn store_refuse_entry(&self, chr: &net::opcode::Character, code: u32, why: &str) -> Vec<Reply> {
        crate::server::log(&format!("   store: character {} refused - {why} (code {code:#x}).", chr.id));
        vec![Reply {
            opcode: wire::SHOP_ROOM,
            body: wire::enter_refused(code),
            what: format!("StoreRoom mode 2, result {code:#x}: no window - {why}"),
        }]
    }

    /// **Mode 0: a Store Permit was used.** The owner's window opens in setup, and the sign goes
    /// up drawn `cannotEnter`.
    fn store_create(&mut self, chr: &net::opcode::Character, room_type: u32, title: String, permit_slot: i16, permit: u32) -> Vec<Reply> {
        if let Some(why) = self.busy_for_trade() {
            return self.store_refuse_entry(chr, wire::room_error::NOT_ENABLED, &format!("busy: {why}"));
        }
        if self.in_shop(chr.id) || self.with_shops(|s| s.iter().any(|r| r.id == chr.id)) {
            return self.store_refuse_entry(chr, wire::room_error::NOT_ENABLED, "already in a store, or their hired merchant stands");
        }
        // One shelf per character: a hired merchant standing anywhere holds it.
        if self.live_merchant(chr.id) {
            return self.store_refuse_entry(chr, wire::room_error::NOT_ENABLED, "their hired merchant is still standing");
        }
        let held = u16::try_from(permit_slot)
            .ok()
            .and_then(|slot| self.store.inventory_slot(chr.id, store::InventoryType::Cash, slot).ok().flatten());
        if !is_permit_for(room_type, permit) || held.map(|i| i.item_id) != Some(permit) {
            return self.store_refuse_entry(chr, wire::room_error::UNKNOWN, &format!("Cash slot {permit_slot} does not hold the item {permit} that opens a room of type {room_type}"));
        }
        // Anything a crash left listed comes home first: a new store starts with an empty shelf.
        let mut out = self.store_give_back(chr.id, "opened a store with lines still held from one that never closed");
        let at = self.remote_at();
        // **Not on top of another store** - any kind, a hired merchant whose owner is long gone
        // included. The client checks only stores whose owner stands beside them.
        self.ensure_merchants_loaded();
        let here = self.field_of(chr);
        let neighbour = self.with_shops(|s| {
            s.iter()
                .find(|r| r.id != chr.id && r.map == here && wire::too_close(r.x, r.y, at.x, at.y))
                .map(|r| (r.owner_name.clone(), r.x, r.y))
        });
        if let Some((name, x, y)) = neighbour {
            return self.store_refuse_entry(
                chr,
                wire::room_error::NOT_HERE,
                &format!("({}, {}) is too close to {name}'s store at ({x}, {y}) - stores stand at least {} px apart", at.x, at.y, wire::STORE_SPACING_X),
            );
        }
        let _ = self.store.close_hired_merchant(chr.id); // an expired row, if any
        let (account_id, _) = self.speaker_of(chr);
        let title = if title.trim().is_empty() { format!("{}'s store", chr.name) } else { title };
        let room = ShopRoom {
            id: chr.id,
            room_type,
            owner_account: account_id,
            owner_name: chr.name.clone(),
            title,
            permit,
            state: wire::STATE_SETUP,
            map: self.field_of(chr),
            seats: [Some(ShopSeat { character_id: chr.id, name: chr.name.clone(), look: net::opcode::avatar_look(chr) }), None, None, None],
            banned: Vec::new(),
            ever_opened: false,
            x: at.x,
            y: at.y,
            opened_at: unix_now(),
        };
        if room.is_merchant() {
            let row = store::playershop::HiredMerchant {
                owner_id: room.id,
                owner_account: room.owner_account,
                owner_name: room.owner_name.clone(),
                title: room.title.clone(),
                permit,
                channel: self.config.channel_id,
                map: room.map.map,
                instance: room.map.instance,
                x: room.x,
                y: room.y,
                opened_at: room.opened_at,
            };
            if let Err(e) = self.store.open_hired_merchant(&row) {
                return self.store_refuse_entry(chr, wire::room_error::UNKNOWN, &format!("the hired merchant could not be recorded ({e})"));
            }
        }
        let window = self.store_window(&room);
        let (map, sign, employee) = (room.map, room.sign(), room.employee());
        let merchant = room.is_merchant();
        self.with_shops(|s| {
            s.retain(|r| r.id != chr.id);
            s.push(room);
        });
        crate::server::log(&format!(
            "   store: character {} opened a {} {:?} with item {permit} (Cash slot {permit_slot}) at ({}, {}). Window open in setup.",
            chr.id,
            if merchant { "hired merchant" } else { "store" },
            sign.title,
            employee.x,
            employee.y
        ));
        out.push(Reply {
            opcode: wire::SHOP_ROOM,
            body: wire::open_window(&window),
            what: format!("StoreRoom mode 2: open the store window {:?} for its owner, in setup", sign.title),
        });
        if merchant {
            self.show_employee(map, chr.id, Some(&employee));
        } else {
            self.show_store_sign(map, chr.id, Some(&sign));
            self.store_owner_spawn_changed(chr.id);
        }
        out
    }

    fn store_window(&self, room: &ShopRoom) -> wire::Window {
        wire::Window {
            room_type: room.room_type,
            shop_id: room.id,
            title: room.title.clone(),
            state: room.state,
            owner_id: room.id,
            owner_account: room.owner_account,
            owner_name: room.owner_name.clone(),
            permit: room.permit,
            seats: room.members(),
            rows: self.shelf_rows(room.id),
        }
    }

    /// **Mode 2: a double-click on an open store's sign.** The visitor takes the first free seat
    /// and gets the whole window; everybody already inside gets the seats, which posts
    /// *"<name> has entered."*.
    fn store_visit(&mut self, chr: &net::opcode::Character, shop: u32) -> Vec<Reply> {
        self.ensure_merchants_loaded();
        if shop == chr.id {
            return self.merchant_maintenance(chr);
        }
        if let Some(why) = self.busy_for_trade() {
            return self.store_refuse_entry(chr, wire::room_error::NOT_ENABLED, &format!("busy: {why}"));
        }
        let me = ShopSeat { character_id: chr.id, name: chr.name.clone(), look: net::opcode::avatar_look(chr) };
        let here = self.field_of(chr);
        let seated = self.with_shops(|s| {
            let Some(r) = s.iter_mut().find(|r| r.id == shop) else { return Err((wire::room_error::ROOM_NOT_FOUND, "no such store")) };
            if r.map != here {
                return Err((wire::room_error::ROOM_NOT_FOUND, "the store is on another map"));
            }
            if r.state != wire::STATE_OPEN {
                return Err((wire::room_error::NOT_ENABLED, "the store is not open yet"));
            }
            if r.banned.contains(&me.character_id) {
                return Err((wire::room_error::DENIED, "banned from this store"));
            }
            let Some(seat) = (1..wire::SEATS).find(|i| r.seats[*i].is_none()) else {
                return Err((wire::room_error::FULL, "the store is full"));
            };
            r.seats[seat] = Some(me);
            Ok((seat, r.clone()))
        });
        let (seat, room) = match seated {
            Ok(s) => s,
            Err((code, why)) => return self.store_refuse_entry(chr, code, why),
        };
        crate::server::log(&format!("   store: character {} visited store {shop} and took seat {seat}.", chr.id));
        let mut out = vec![Reply {
            opcode: wire::SHOP_ROOM,
            body: wire::open_window(&self.store_window(&room)),
            what: format!("StoreRoom mode 2: open store {shop}'s window for visitor {} (seat {seat})", chr.id),
        }];
        let seats = room.members();
        for other in room.occupants().into_iter().filter(|id| *id != chr.id) {
            self.to_seat(chr.id, other, wire::SHOP_EVENT, wire::seats_changed(&seats), format!("StoreEvent mode 0: {} sat in seat {seat} of store {shop}", chr.name), &mut out);
        }
        out
    }

    /// **The owner double-clicks their own hired merchant: maintenance.** They take seat 0, the
    /// merchant goes back to setup (list, take back, open again) and draws `cannotEnter`, and anybody
    /// browsing is put out. A click on one's own store sign is nothing - its owner is already inside.
    fn merchant_maintenance(&mut self, chr: &net::opcode::Character) -> Vec<Reply> {
        let me = ShopSeat { character_id: chr.id, name: chr.name.clone(), look: net::opcode::avatar_look(chr) };
        let here = self.field_of(chr);
        let entered = self.with_shops(|s| {
            let r = s.iter_mut().find(|r| r.id == chr.id && r.is_merchant() && r.seats[0].is_none() && r.map == here)?;
            let out: Vec<(usize, u32)> = (1..wire::SEATS).filter_map(|i| Some((i, r.seats[i].take()?.character_id))).collect();
            r.seats[0] = Some(me);
            r.state = wire::STATE_SETUP;
            Some((r.clone(), out))
        });
        let Some((room, put_out)) = entered else {
            return self.store_release("a click on one's own sign");
        };
        crate::server::log(&format!("   store: character {} is back at their hired merchant - maintenance, {} visitor(s) put out.", chr.id, put_out.len()));
        let mut out = Vec::new();
        for (seat, id) in put_out {
            self.to_seat(chr.id, id, wire::SHOP_EVENT, wire::seat_left(seat as u8, wire::LEAVE_QUIET), format!("StoreEvent mode 3: hired merchant {} in maintenance - visitor out of seat {seat}", chr.id), &mut out);
        }
        self.show_employee(room.map, room.id, Some(&room.employee()));
        out.push(Reply {
            opcode: wire::SHOP_ROOM,
            body: wire::open_window(&self.store_window(&room)),
            what: format!("StoreRoom mode 2: open hired merchant {}'s window for its owner, in maintenance", chr.id),
        });
        out
    }

    /// **A hired merchant's owner steps away** - closes the window, changes map, leaves the game -
    /// after it has been opened: it stays. In maintenance it opens again if anything is on the
    /// shelf, and closes if nothing is.
    fn merchant_owner_away(&mut self, chr: &net::opcode::Character, why: &str) -> Vec<Reply> {
        let has_lines = self.store.shop_has_stock(chr.id).unwrap_or(false);
        let left = self.with_shops(|s| {
            let r = s.iter_mut().find(|r| r.id == chr.id)?;
            r.seats[0] = None;
            if r.state == wire::STATE_SETUP && has_lines {
                r.state = wire::STATE_OPEN;
            }
            Some(r.clone())
        });
        let Some(room) = left else { return Vec::new() };
        if room.state != wire::STATE_OPEN {
            self.with_shops(|s| s.retain(|r| r.id != chr.id));
            return self.close_room(room, &format!("its owner {why} with nothing on the shelf"), Some(chr.id));
        }
        crate::server::log(&format!("   store: character {} {why}; their hired merchant {:?} stays open.", chr.id, room.title));
        self.show_employee(room.map, room.id, Some(&room.employee()));
        let mut out = Vec::new();
        for id in room.occupants() {
            self.to_seat(chr.id, id, wire::SHOP_EVENT, wire::seat_left(0, wire::LEAVE_QUIET), format!("StoreEvent mode 3: hired merchant {}'s owner stepped away", chr.id), &mut out);
        }
        out
    }

    /// **The owner closes the store, or anybody leaves it.**
    ///
    /// A visitor leaving frees their seat and the others see *"<name> has left."*; their own window
    /// has already closed itself. The owner leaving - "close store" (`close_pressed`), the window
    /// closing, or the channel - closes the store: every line back into the owner's bag, every
    /// visitor's window closed with *"The shop has been closed."*, the sign down. The owner's
    /// window is closed by its own-seat `0x0576` mode 3 when it is still open, which also releases
    /// the latch "close store" set.
    fn store_leave(&mut self, chr: &net::opcode::Character, why: &str, close_pressed: bool) -> Vec<Reply> {
        let Some((shop, seat)) = self.shop_seat_of(chr.id) else {
            crate::server::log(&format!("   store: character {} {why} with no store here; nothing to close.", chr.id));
            return if close_pressed { self.store_release("close with no store") } else { Vec::new() };
        };
        let mut out = Vec::new();
        if seat != 0 {
            let room = self.with_shops(|s| {
                let r = s.iter_mut().find(|r| r.id == shop)?;
                r.seats[seat] = None;
                Some(r.clone())
            });
            crate::server::log(&format!("   store: character {} {why} - left seat {seat} of store {shop}.", chr.id));
            if let Some(room) = room {
                for other in room.occupants() {
                    self.to_seat(chr.id, other, wire::SHOP_EVENT, wire::seat_left(seat as u8, wire::LEAVE_QUIET), format!("StoreEvent mode 3: seat {seat} of store {shop} is empty ({} left)", chr.name), &mut out);
                }
            }
            if close_pressed {
                out.extend(self.store_release("a visitor pressed close"));
            }
            return out;
        }
        let stays = self.with_shops(|s| s.iter().any(|r| r.id == shop && r.is_merchant() && r.ever_opened));
        if stays && !close_pressed {
            return self.merchant_owner_away(chr, why);
        }
        let Some(room) = self.with_shops(|s| {
            let i = s.iter().position(|r| r.id == shop)?;
            Some(s.remove(i))
        }) else {
            return out;
        };
        out.extend(self.close_room(room, &format!("its owner {why}"), Some(chr.id)));
        if close_pressed {
            out.push(Reply {
                opcode: wire::SHOP_EVENT,
                body: wire::seat_left(0, wire::LEAVE_QUIET),
                what: format!("StoreEvent mode 3: the owner's own seat - store {shop}'s window closes"),
            });
        }
        out
    }

    /// **Mode 5: a line in the store's chat**, to every seat - the window draws nothing of its own.
    fn store_chat(&mut self, chr: &net::opcode::Character, text: &str) -> Vec<Reply> {
        let Some((shop, seat)) = self.shop_seat_of(chr.id) else {
            crate::server::log(&format!("   store: character {} chatted with no store here; nothing sent.", chr.id));
            return Vec::new();
        };
        let everyone = self.with_shops(|s| s.iter().find(|r| r.id == shop).map(ShopRoom::occupants).unwrap_or_default());
        let mut out = Vec::new();
        for to in everyone {
            self.to_seat(chr.id, to, wire::SHOP_EVENT, wire::chat(seat as u8, chr.id, &chr.name, text), format!("StoreEvent mode 2: {} (seat {seat}) in store {shop}: {text:?}", chr.name), &mut out);
        }
        out
    }

    /// **Mode 6: the owner bans the visitor in `seat`.** Their window closes, the others see them
    /// leave, and they are refused at the door until the store closes.
    fn store_ban(&mut self, chr: &net::opcode::Character, seat: u16) -> Vec<Reply> {
        let seat = usize::from(seat);
        let banned = self.with_shops(|s| {
            let r = s.iter_mut().find(|r| r.id == chr.id)?;
            let gone = r.seats.get_mut(seat).filter(|_| seat != 0)?.take()?;
            r.banned.push(gone.character_id);
            Some((gone, r.occupants()))
        });
        let mut out = Vec::new();
        if let Some((gone, rest)) = banned {
            crate::server::log(&format!("   store: store {} banned {} ({}) from seat {seat}.", chr.id, gone.name, gone.character_id));
            for to in std::iter::once(gone.character_id).chain(rest) {
                self.to_seat(chr.id, to, wire::SHOP_EVENT, wire::seat_left(seat as u8, wire::LEAVE_QUIET), format!("StoreEvent mode 3: {} banned from seat {seat} of store {}", gone.name, chr.id), &mut out);
            }
        } else {
            crate::server::log(&format!("   store: character {} asked to ban seat {seat}, which is not a visitor of their store.", chr.id));
        }
        out.extend(self.store_release("a ban"));
        out
    }

    // -----------------------------------------------------------------------------------
    // 0x0180 - the shelf
    // -----------------------------------------------------------------------------------

    /// `0x0180` - the store's shelf. Every answer is a `0x0577` or a `0x0579`, which release the
    /// request latch the window set.
    pub(super) fn on_shop_shelf(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return crate::mesodrop::unlock_unhandled_latching_request(wire::CLIENT_SHOP_SHELF);
        };
        let Some(req) = wire::parse_shelf(body) else {
            crate::server::log(&format!("   store: 0x0180 did not decode ({} byte body).", body.len()));
            return self.shelf_refused(&chr, wire::shelf_error::UNKNOWN, "an undecodable request");
        };
        let Some((shop, seat)) = self.shop_seat_of(chr.id) else {
            // No window here to read a 0x0579 - the plain unlock is what releases the latch.
            crate::server::log(&format!("   store: character {} sent a shelf request ({req:?}) with no store here.", chr.id));
            return crate::mesodrop::unlock_unhandled_latching_request(wire::CLIENT_SHOP_SHELF);
        };
        let state = self.with_shops(|s| s.iter().find(|r| r.id == shop).map_or(0, |r| r.state));
        let owner = seat == 0;
        match req {
            wire::ShelfRequest::Open if owner && state == wire::STATE_SETUP => self.store_open(&chr),
            wire::ShelfRequest::List { inv_type, slot, bundles, per_bundle, price } if owner && state == wire::STATE_SETUP => {
                self.store_list(&chr, inv_type, slot, bundles, per_bundle, price)
            }
            wire::ShelfRequest::TakeBack { row } if owner && state == wire::STATE_SETUP => self.store_take_back(&chr, row),
            wire::ShelfRequest::Buy { row, bundles, total } if !owner && state == wire::STATE_OPEN => self.store_buy(&chr, shop, row, bundles, total),
            // A hired merchant's "collect": every sale here has already paid the owner, so there is
            // nothing held to collect - the shelf, and the latch released.
            wire::ShelfRequest::Collect if owner => {
                crate::server::log(&format!("   store: store {shop}'s owner pressed collect; sales pay the owner at once, so nothing is held."));
                vec![Reply { opcode: wire::SHOP_ANSWER, body: wire::shelf_done(false, &self.shelf_rows(shop)), what: format!("StoreAnswer mode 1: collect - nothing held in store {shop}") }]
            }
            other => self.shelf_refused(&chr, wire::shelf_error::NOT_ENABLED, &format!("{other:?} from seat {seat} of a store in state {state}")),
        }
    }

    fn shelf_refused(&self, chr: &net::opcode::Character, code: u32, why: &str) -> Vec<Reply> {
        crate::server::log(&format!("   store: character {}'s shelf request refused - {why} (code {code:#x}).", chr.id));
        vec![Reply { opcode: wire::SHOP_ANSWER, body: wire::shelf_refused(code), what: format!("StoreAnswer mode 3, code {code:#x}: {why}") }]
    }

    /// **Mode 0: open for business.** The window and the sign say open.
    fn store_open(&mut self, chr: &net::opcode::Character) -> Vec<Reply> {
        if !self.store.shop_has_stock(chr.id).unwrap_or(false) {
            return self.shelf_refused(chr, wire::shelf_error::NOT_ENABLED, "nothing is for sale");
        }
        let Some(room) = self.with_shops(|s| {
            let r = s.iter_mut().find(|r| r.id == chr.id)?;
            r.state = wire::STATE_OPEN;
            r.ever_opened = true;
            Some(r.clone())
        }) else {
            return self.shelf_refused(chr, wire::shelf_error::NOT_ENABLED, "the store is gone");
        };
        crate::server::log(&format!("   store: store {} {:?} is OPEN.", chr.id, room.title));
        if room.is_merchant() {
            self.show_employee(room.map, room.id, Some(&room.employee()));
        } else {
            self.show_store_sign(room.map, chr.id, Some(&room.sign()));
            self.store_owner_spawn_changed(chr.id);
        }
        vec![
            Reply { opcode: wire::SHOP_ROOM, body: wire::state(wire::STATE_OPEN), what: format!("StoreRoom mode 4: store {} is open", chr.id) },
            // Open Store locked the owner's whole UI (FUN_142aa27f0(4)); only a 0x0579 unlocks it.
            Reply { opcode: wire::SHOP_ANSWER, body: wire::shelf_unlock(), what: format!("StoreAnswer mode 0, code 0: store {} - nothing to say; releases the UI lock Open Store took", chr.id) },
        ]
    }

    /// **Mode 2: an item onto the shelf** - out of the bag into `store::playershop`'s escrow.
    fn store_list(&mut self, chr: &net::opcode::Character, inv_type: u8, slot: i16, bundles: u16, per_bundle: u16, price: u64) -> Vec<Reply> {
        let Ok(inv) = store::InventoryType::from_wire(i16::from(inv_type)) else {
            return self.shelf_refused(chr, wire::shelf_error::INVALID_ITEM, &format!("there is no inventory tab {inv_type}"));
        };
        if inv == store::InventoryType::Cash {
            return self.shelf_refused(chr, wire::shelf_error::CASH_ITEM, "cash items cannot be sold in a store");
        }
        let Ok(slot) = u16::try_from(slot) else {
            return self.shelf_refused(chr, wire::shelf_error::INVALID_ITEM, "only items in the inventory can be listed");
        };
        let Some(held) = self.store.inventory_slot(chr.id, inv, slot).ok().flatten() else {
            return self.shelf_refused(chr, wire::shelf_error::INVALID_ITEM, &format!("{inv:?} slot {slot} is empty"));
        };
        if store::ItemRules::trade_blocked(held.item_id) {
            return self.shelf_refused(chr, wire::shelf_error::UNTRADEABLE, &format!("item {} cannot be traded", held.item_id));
        }
        if price == 0 || price > u64::from(u32::MAX) {
            return self.shelf_refused(chr, wire::shelf_error::INVALID_ITEM, &format!("a price of {price} mesos"));
        }
        let (line, left) = match self.store.list_shop_item(chr.id, inv, slot, bundles, per_bundle, price, wire::MAX_LINES) {
            Ok(l) => l,
            Err(e) => return self.shelf_refused(chr, wire::shelf_error::INVALID_ITEM, &format!("it could not be listed ({e})")),
        };
        crate::server::log(&format!(
            "   store: store {} listed item {} - {} bundle(s) of {} at {price} mesos each - from {inv:?} slot {slot} ({left} left there).",
            chr.id, line.item.item_id, line.bundles, line.per_bundle
        ));
        let mut out = self.stack_change_replies(inv, slot, left);
        out.push(Reply {
            opcode: wire::SHOP_ANSWER,
            body: wire::shelf_done(false, &self.shelf_rows(chr.id)),
            what: format!("StoreAnswer mode 1: item {} on store {}'s shelf", line.item.item_id, chr.id),
        });
        out
    }

    /// **Mode 1: an item off the shelf** and back into the bag.
    fn store_take_back(&mut self, chr: &net::opcode::Character, row: u16) -> Vec<Reply> {
        let config = self.config.clone();
        let (inv, changed) = match self.store.unlist_shop_item(chr.id, row, &|id| config.shops.max_stack(id)) {
            Ok(r) => r,
            Err(e) => return self.shelf_refused(chr, wire::shelf_error::INVALID_ITEM, &format!("row {row} could not come back ({e})")),
        };
        crate::server::log(&format!("   store: store {} took row {row} back into {inv:?}.", chr.id));
        let mut out = self.inventory_added_replies(inv, &changed, "taken back off the store shelf");
        out.push(Reply {
            opcode: wire::SHOP_ANSWER,
            body: wire::shelf_done(false, &self.shelf_rows(chr.id)),
            what: format!("StoreAnswer mode 1: row {row} off store {}'s shelf", chr.id),
        });
        out
    }

    /// **Mode 3: a visitor buys.** One transaction (`Store::buy_from_shop`); the buyer is shown what
    /// arrived and their wallet, the owner their wallet, and everybody in the store the new shelf.
    fn store_buy(&mut self, chr: &net::opcode::Character, shop: u32, row: u8, bundles: u16, total: u64) -> Vec<Reply> {
        let config = self.config.clone();
        let sale = match self.store.buy_from_shop(shop, chr.id, u16::from(row), bundles, Some(total), &|id| config.shops.max_stack(id)) {
            Ok(s) => s,
            Err(e) => {
                let (code, extra) = match &e {
                    store::StoreError::NotEnoughMesos { .. } => (wire::shelf_error::NOT_ENOUGH_MESOS, None),
                    store::StoreError::WalletCap { .. } => (wire::shelf_error::SELLER_MESO_LIMIT, None),
                    store::StoreError::BagFull { .. } => (wire::shelf_error::UNKNOWN, Some("Your inventory is full.".to_string())),
                    _ => (wire::shelf_error::NO_STOCK, None),
                };
                let mut out = self.shelf_refused(chr, code, &format!("buying row {row} x {bundles} for {total} from store {shop} failed ({e})"));
                if let Some(text) = extra {
                    out.extend(self.notice(text));
                }
                // The window may be drawing a shelf that has changed under it.
                out.push(Reply { opcode: wire::SHOP_ROWS, body: wire::shelf(&self.shelf_rows(shop)), what: format!("StoreRows: store {shop}'s shelf, redrawn after a refused purchase") });
                return out;
            }
        };
        crate::server::log(&format!(
            "   store: character {} bought {} x item {} from store {shop} for {} mesos; the owner got {} after the {}% fee.",
            chr.id,
            sale.units,
            sale.item_id,
            sale.paid,
            sale.proceeds,
            store::playershop::SALES_FEE_PERCENT
        ));
        let rows = self.shelf_rows(shop);
        let mut out = Vec::new();
        for (inv, changed) in &sale.placed {
            out.extend(self.inventory_added_replies(*inv, changed, "bought from a store"));
        }
        out.extend(self.meso_reply(chr.id));
        out.push(Reply { opcode: wire::SHOP_ANSWER, body: wire::shelf_done(true, &rows), what: format!("StoreAnswer mode 2: bought row {row} x {bundles} from store {shop}") });
        let others = self.with_shops(|s| s.iter().find(|r| r.id == shop).map(ShopRoom::occupants).unwrap_or_default());
        // A hired merchant's owner away from it still sees the wallet rise, if they are on this
        // channel; anywhere else the store already holds the new balance.
        if !others.contains(&shop) && self.bus().character_online(shop) {
            for r in self.meso_reply(shop) {
                let _ = self.bus().publish_to_character_anywhere(shop, Reply { what: format!("{} - a sale at their hired merchant", r.what), ..r });
            }
        }
        for to in others.into_iter().filter(|id| *id != chr.id) {
            if to == shop {
                for r in self.meso_reply(shop) {
                    self.to_seat(chr.id, shop, r.opcode, r.body, format!("{} - a sale in their store", r.what), &mut out);
                }
            }
            self.to_seat(chr.id, to, wire::SHOP_ROWS, wire::shelf(&rows), format!("StoreRows: store {shop}'s shelf after {}'s purchase", chr.name), &mut out);
        }
        // **Sold out: the store closes.** The owner, 2026-10-04: *"once the shop is out of items, the
        // store should automatically close."* Everybody inside - the buyer and the owner too - gets
        // their window closed with "The shop has been closed.", after the purchase answer above.
        if !self.store.shop_has_stock(shop).unwrap_or(true) {
            if let Some(room) = self.with_shops(|s| {
                let i = s.iter().position(|r| r.id == shop)?;
                Some(s.remove(i))
            }) {
                out.extend(self.close_room(room, "everything sold", None));
            }
        }
        out
    }

    /// **Every line of this character's store back in the bag**, and the packets that show it.
    /// Lines that no longer fit stay held and come back at the next login.
    fn store_give_back(&mut self, character: u32, why: &str) -> Vec<Reply> {
        let config = self.config.clone();
        let returned = match self.store.return_shop_escrow(character, &|id| config.shops.max_stack(id)) {
            Ok(r) => r,
            Err(e) => {
                crate::server::log(&format!("   store: could not give character {character} their shelf back ({e}); it stays held for the next login."));
                return Vec::new();
            }
        };
        if returned.placed.is_empty() && returned.kept == 0 {
            return Vec::new();
        }
        crate::server::log(&format!(
            "   store: character {character} {why}: {} line(s) back in the bag{}.",
            returned.placed.len(),
            if returned.kept > 0 { format!(", {} line(s) did NOT fit and stay held until the next login", returned.kept) } else { String::new() }
        ));
        let mut out = Vec::new();
        for (inv, changed) in &returned.placed {
            out.extend(self.inventory_added_replies(*inv, changed, "back from the store shelf"));
        }
        out
    }

    /// From the login, after the claim: anything a crash left on a store shelf goes back into the
    /// bag before the record that draws it is built.
    pub(super) fn return_shop_escrow_at_login(&mut self) {
        let Some(chr) = self.claimed_character() else { return };
        match self.store.hired_merchant(chr.id) {
            Ok(Some(m)) if !m.expired(unix_now()) => {
                crate::server::log(&format!("   store: character {} logged in; their hired merchant {:?} still stands, so its shelf stays on it.", chr.id, m.title));
                return;
            }
            Ok(Some(m)) => {
                let _ = self.store.close_hired_merchant(chr.id);
                crate::server::log(&format!("   store: character {}'s hired merchant {:?} stood its 24 hours while they were away.", chr.id, m.title));
            }
            _ => {}
        }
        let _ = self.store_give_back(chr.id, "logged in with lines still held from a store that is no longer open");
    }

    /// From `Drop`: an owner leaving the channel closes the store; a visitor frees the seat.
    pub(super) fn leave_shop_on_disconnect(&mut self) {
        if let Some(chr) = self.claimed_character() {
            if self.in_shop(chr.id) {
                let _ = self.store_leave(&chr, "left the channel", false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    const MAP: u32 = 104_040_000;

    fn channel() -> (Arc<store::Store>, Arc<crate::config::Config>, Arc<crate::fields::Fields>) {
        let store = Arc::new(store::Store::open_in_memory().unwrap());
        (store, Arc::new(crate::config::Config::default()), Arc::new(crate::fields::Fields::new()))
    }

    fn join(store: &Arc<store::Store>, config: &Arc<crate::config::Config>, fields: &Arc<crate::fields::Fields>, name: &str) -> (Session, u32) {
        let account = store.create_account(&name.to_lowercase(), "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: name.to_string(), map_id: MAP, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
        s.claim_for_character(id);
        s.on_field_entered();
        s.collect_mail();
        (s, id)
    }

    fn room(body: &[u8]) -> Vec<u8> {
        [&wire::CLIENT_SHOP_ROOM.to_le_bytes()[..], body].concat()
    }

    fn shelf(body: &[u8]) -> Vec<u8> {
        [&wire::CLIENT_SHOP_SHELF.to_le_bytes()[..], body].concat()
    }

    fn of(out: &[Reply], opcode: u16) -> Vec<Vec<u8>> {
        out.iter().filter(|r| r.opcode == opcode).map(|r| r.body.clone()).collect()
    }

    fn create(slot: i16, permit: u32) -> Vec<u8> {
        let mut w = net::PacketWriter::new();
        w.u32(0).u32(3).str("Bargains").i16(slot).u32(permit);
        room(w.as_slice())
    }

    /// The 2026-10-04 double-click: a Hired Merchant in Cash slot 16 is a yes, and the create that
    /// follows opens a type-4 window; a second check while it is open is "already open".
    #[test]
    fn a_hired_merchant_is_checked_then_opened() {
        let (store, config, fields) = channel();
        let (mut owner, owner_id) = join(&store, &config, &fields, "Wisp");
        store.add_item(owner_id, store::InventoryType::Cash, &store::Item::bundle(5_030_000, 1), 1).unwrap();
        let slot = store.bag_items(owner_id, store::InventoryType::Cash).unwrap()[0].slot;
        let mut w = net::PacketWriter::new();
        w.u16(slot).u32(5_030_000);
        let check = [&wire::CLIENT_HIRED_CHECK.to_le_bytes()[..], w.as_slice()].concat();
        assert_eq!(of(&owner.dispatch(&check), wire::HIRED_CHECK_RESULT), vec![wire::hired_check_result(wire::HIRED_OK)]);

        let mut w = net::PacketWriter::new();
        w.u32(0).u32(4).str("Elf").i16(slot as i16).u32(5_030_000);
        let opened = of(&owner.dispatch(&room(w.as_slice())), wire::SHOP_ROOM);
        assert_eq!(&opened[0][..12], &[2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0], "mode 2, result 0, type 4");
        assert!(owner.own_store_sign(owner_id).is_none(), "a hired merchant is an employee on the map, not a sign over its owner");
        assert_eq!(store.hired_merchant(owner_id).unwrap().map(|m| m.permit), Some(5_030_000));
        assert_eq!(of(&owner.dispatch(&check), wire::HIRED_CHECK_RESULT), vec![wire::hired_check_result(wire::HIRED_ALREADY_OPEN)]);
    }

    /// **The merchant stays behind.** Set up, an item listed, opened; the owner closes the window
    /// and leaves the game - the merchant stands, a visitor arriving sees it, walks in and buys, and
    /// the owner's wallet rises while they are away. At 24 hours it closes, and the rest of the
    /// shelf comes home at the owner's next login.
    #[test]
    fn a_hired_merchant_stays_sells_and_expires() {
        let (store, config, fields) = channel();
        let (mut owner, owner_id) = join(&store, &config, &fields, "Wisp");
        store.add_item(owner_id, store::InventoryType::Cash, &store::Item::bundle(5_030_000, 1), 1).unwrap();
        let slot = store.bag_items(owner_id, store::InventoryType::Cash).unwrap()[0].slot as i16;
        store.add_item(owner_id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 30), 100).unwrap();
        let use_slot = store.bag_items(owner_id, store::InventoryType::Use).unwrap()[0].slot as i16;

        let mut w = net::PacketWriter::new();
        w.u32(0).u32(4).str("Elf").i16(slot).u32(5_030_000);
        owner.dispatch(&room(w.as_slice()));
        assert!(store.hired_merchant(owner_id).unwrap().is_some(), "recorded at setup");
        let mut w = net::PacketWriter::new();
        w.u32(2).u8(2).i16(use_slot).u16(3).u16(10).u64(500);
        owner.dispatch(&shelf(w.as_slice()));
        owner.dispatch(&shelf(&0u32.to_le_bytes()));
        // The window closes (mode 3) and the owner leaves the game: the merchant stays.
        assert!(owner.dispatch(&room(&3u32.to_le_bytes())).is_empty());
        drop(owner);
        assert!(store.hired_merchant(owner_id).unwrap().is_some());
        assert_eq!(store.shop_lines(owner_id).unwrap().len(), 1);

        let (mut buyer, buyer_id) = join(&store, &config, &fields, "Pebble");
        store.set_mesos(buyer_id, 2_000).unwrap();
        let here = crate::fields::FieldKey { map: MAP, instance: 0 };
        let seen = of(&buyer.merchants_on_entry(here), wire::EMPLOYEE_ENTER);
        assert_eq!(seen.len(), 1, "an arrival sees the merchant");
        assert_eq!(&seen[0][..8], &[&owner_id.to_le_bytes()[..], &5_030_000u32.to_le_bytes()[..]].concat()[..]);

        let mut w = net::PacketWriter::new();
        w.u32(2).u32(owner_id);
        let out = buyer.dispatch(&room(w.as_slice()));
        assert_eq!(&of(&out, wire::SHOP_ROOM)[0][..12], &[2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0], "the merchant's window, owner away");
        let mut w = net::PacketWriter::new();
        w.u32(3).u8(0).u16(2).u64(1_000).u32(0);
        buyer.dispatch(&shelf(w.as_slice()));
        assert_eq!(store.mesos(buyer_id).unwrap(), 1_000);
        assert_eq!(store.mesos(owner_id).unwrap(), 970, "paid while offline, less 3%");

        // 24 hours on: closed, the row gone, the last bundle still held for the owner's login.
        buyer.dispatch(&room(&3u32.to_le_bytes()));
        let opened = store.hired_merchant(owner_id).unwrap().unwrap().opened_at;
        buyer.expire_merchants(opened + store::playershop::HIRED_MERCHANT_SECS);
        assert!(store.hired_merchant(owner_id).unwrap().is_none());
        assert!(of(&buyer.merchants_on_entry(here), wire::EMPLOYEE_ENTER).is_empty());
        assert_eq!(store.shop_lines(owner_id).unwrap().len(), 1, "held for the next login");
        let _owner = rejoin(&store, &config, &fields, "wisp", owner_id);
        assert!(store.shop_lines(owner_id).unwrap().is_empty());
        assert_eq!(store.bag_items(owner_id, store::InventoryType::Use).unwrap().iter().map(|i| i.item.kind.quantity()).sum::<u16>(), 10);
    }

    fn rejoin(store: &Arc<store::Store>, config: &Arc<crate::config::Config>, fields: &Arc<crate::fields::Fields>, account: &str, id: u32) -> Session {
        let account = store.get_account(account).unwrap().unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
        s.claim_for_character(id);
        s.on_field_entered();
        s
    }

    /// **A store too close to another is refused** with the client's "You can't open a store here",
    /// and one far enough away opens.
    #[test]
    fn a_store_too_close_to_another_is_refused() {
        let (store, config, fields) = channel();
        let (mut a, a_id) = join(&store, &config, &fields, "Wisp");
        let (mut b, b_id) = join(&store, &config, &fields, "Pebble");
        for id in [a_id, b_id] {
            store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_140_001, 1), 1).unwrap();
        }
        let slot = |id| store.bag_items(id, store::InventoryType::Cash).unwrap()[0].slot as i16;
        a.last_position = Some((100, 0));
        assert_eq!(&of(&a.dispatch(&create(slot(a_id), 5_140_001)), wire::SHOP_ROOM)[0][..8], &[2, 0, 0, 0, 0, 0, 0, 0]);
        b.last_position = Some((180, 0));
        assert_eq!(of(&b.dispatch(&create(slot(b_id), 5_140_001)), wire::SHOP_ROOM), vec![wire::enter_refused(wire::room_error::NOT_HERE)]);
        b.last_position = Some((220, 0));
        assert_eq!(&of(&b.dispatch(&create(slot(b_id), 5_140_001)), wire::SHOP_ROOM)[0][..8], &[2, 0, 0, 0, 0, 0, 0, 0], "120 px apart is far enough");
    }

    /// **The last bundle sold closes the store**, the buyer's window included, and the sold-out
    /// record does not come back to the owner as an item.
    #[test]
    fn selling_out_closes_the_store() {
        let (store, config, fields) = channel();
        let (mut owner, owner_id) = join(&store, &config, &fields, "Wisp");
        let (mut buyer, buyer_id) = join(&store, &config, &fields, "Pebble");
        store.add_item(owner_id, store::InventoryType::Cash, &store::Item::bundle(5_140_001, 1), 1).unwrap();
        let permit = store.bag_items(owner_id, store::InventoryType::Cash).unwrap()[0].slot as i16;
        store.add_item(owner_id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 20), 100).unwrap();
        let use_slot = store.bag_items(owner_id, store::InventoryType::Use).unwrap()[0].slot as i16;
        store.set_mesos(buyer_id, 1_000).unwrap();
        owner.dispatch(&create(permit, 5_140_001));
        let mut w = net::PacketWriter::new();
        w.u32(2).u8(2).i16(use_slot).u16(2).u16(10).u64(100);
        owner.dispatch(&shelf(w.as_slice()));
        owner.dispatch(&shelf(&0u32.to_le_bytes()));
        let mut w = net::PacketWriter::new();
        w.u32(2).u32(owner_id);
        buyer.dispatch(&room(w.as_slice()));

        // One bundle: the store stays open.
        let mut w = net::PacketWriter::new();
        w.u32(3).u8(0).u16(1).u64(100).u32(0);
        let out = buyer.dispatch(&shelf(w.as_slice()));
        assert!(of(&out, wire::SHOP_EVENT).is_empty());
        // The second: sold out. The buyer is answered, then put out with "The shop has been closed."
        let out = buyer.dispatch(&shelf(w.as_slice()));
        assert_eq!(of(&out, wire::SHOP_ANSWER).len(), 1);
        assert_eq!(of(&out, wire::SHOP_EVENT), vec![wire::seat_left(1, wire::LEAVE_SHOP_CLOSED)]);
        assert!(!buyer.in_shop(buyer_id) && !owner.in_shop(owner_id));
        assert!(store.shop_lines(owner_id).unwrap().is_empty(), "the sold-out record goes with the store");
        assert!(store.bag_items(owner_id, store::InventoryType::Use).unwrap().is_empty(), "nothing came back");
        assert_eq!(store.bag_items(buyer_id, store::InventoryType::Use).unwrap()[0].item.kind.quantity(), 20);
    }

    /// A permit in the Cash tab, a store opened, an item listed and the store opened for
    /// business; a visitor walks in and buys; the owner closes and the rest comes home.
    #[test]
    fn a_store_opens_sells_and_closes() {
        let (store, config, fields) = channel();
        let (mut owner, owner_id) = join(&store, &config, &fields, "Wisp");
        let (mut buyer, buyer_id) = join(&store, &config, &fields, "Pebble");
        store.add_item(owner_id, store::InventoryType::Cash, &store::Item::bundle(5_140_001, 1), 1).unwrap();
        let permit_slot = store.bag_items(owner_id, store::InventoryType::Cash).unwrap()[0].slot as i16;
        store.add_item(owner_id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 50), 100).unwrap();
        let use_slot = store.bag_items(owner_id, store::InventoryType::Use).unwrap()[0].slot as i16;
        store.set_mesos(buyer_id, 10_000).unwrap();

        // A hired-merchant check for a Store Permit is a no; a create of the wrong type opens nothing.
        let mut w = net::PacketWriter::new();
        w.u16(permit_slot as u16).u32(5_140_001);
        let out = owner.dispatch(&[&wire::CLIENT_HIRED_CHECK.to_le_bytes()[..], w.as_slice()].concat());
        assert_eq!(of(&out, wire::HIRED_CHECK_RESULT), vec![wire::hired_check_result(wire::HIRED_ERROR)]);
        let mut w = net::PacketWriter::new();
        w.u32(0).u32(4).str("Bargains").i16(permit_slot).u32(5_140_001);
        let out = owner.dispatch(&room(w.as_slice()));
        assert_eq!(of(&out, wire::SHOP_ROOM), vec![wire::enter_refused(wire::room_error::UNKNOWN)]);

        // A permit that is not in that slot opens nothing.
        let out = owner.dispatch(&create(permit_slot, 5_140_002));
        assert_eq!(of(&out, wire::SHOP_ROOM), vec![wire::enter_refused(wire::room_error::UNKNOWN)]);

        let out = owner.dispatch(&create(permit_slot, 5_140_001));
        let opened = of(&out, wire::SHOP_ROOM);
        assert_eq!(opened.len(), 1);
        assert_eq!(&opened[0][..8], &[2, 0, 0, 0, 0, 0, 0, 0], "mode 2, result 0");
        assert_eq!(owner.own_store_sign(owner_id).map(|s| s.state), Some(wire::STATE_SETUP));

        // The buyer cannot get in before the store is open.
        let mut w = net::PacketWriter::new();
        w.u32(2).u32(owner_id);
        let visit = room(w.as_slice());
        let out = buyer.dispatch(&visit);
        assert_eq!(of(&out, wire::SHOP_ROOM), vec![wire::enter_refused(wire::room_error::NOT_ENABLED)]);

        // Two bundles of ten potions at 300 a bundle.
        let mut w = net::PacketWriter::new();
        w.u32(2).u8(2).i16(use_slot).u16(2).u16(10).u64(300);
        let out = owner.dispatch(&shelf(w.as_slice()));
        let done = of(&out, wire::SHOP_ANSWER);
        assert_eq!(&done[0][..7], &[1, 0, 0, 0, 1, 1, 0], "mode 1, has rows, one row");
        assert_eq!(store.bag_items(owner_id, store::InventoryType::Use).unwrap()[0].item.kind.quantity(), 30);

        let out = owner.dispatch(&shelf(&0u32.to_le_bytes()));
        assert_eq!(of(&out, wire::SHOP_ROOM), vec![wire::state(wire::STATE_OPEN)]);
        assert_eq!(of(&out, wire::SHOP_ANSWER), vec![wire::shelf_unlock()], "Open Store's UI lock is released");

        let out = buyer.dispatch(&visit);
        assert_eq!(&of(&out, wire::SHOP_ROOM)[0][..8], &[2, 0, 0, 0, 0, 0, 0, 0], "the visitor's window opens");

        // One bundle, at the price the window computed. The row stays, one bundle fewer.
        let mut w = net::PacketWriter::new();
        w.u32(3).u8(0).u16(1).u64(300).u32(0);
        let out = buyer.dispatch(&shelf(w.as_slice()));
        assert_eq!(&of(&out, wire::SHOP_ANSWER)[0][..7], &[2, 0, 0, 0, 1, 1, 0], "mode 2: bought, one row");
        assert_eq!(store.mesos(buyer_id).unwrap(), 9_700);
        assert_eq!(store.mesos(owner_id).unwrap(), 291, "300 less the 3% fee");

        // The owner closes: the last bundle comes home, the visitor is put out.
        let out = owner.dispatch(&room(&1u32.to_le_bytes()));
        assert_eq!(of(&out, wire::SHOP_EVENT), vec![wire::seat_left(0, wire::LEAVE_QUIET)]);
        assert_eq!(store.bag_items(owner_id, store::InventoryType::Use).unwrap()[0].item.kind.quantity(), 40);
        assert!(owner.own_store_sign(owner_id).is_none());
        assert!(!buyer.in_shop(buyer_id));
    }
}
