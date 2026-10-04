//! Player-to-player trade: the miniroom protocol, and the popup that never appeared.
//!
//! The owner, 2026-09-09: *"Tester2 also sent Cobalt a trade request, but the trade request pop up
//! never showed up on Cobalt's side."* Full decode in `research/trade-2026-09-09.md`; the parts
//! this module rests on are quoted where they are used.
//!
//! Labels: **[L]** read off this client's listing or a capture, **[D]** derived, **[I]** policy.
//!
//! # The invite is TWO packets, and the first one is easy to miss
//!
//! The archived capture, 8 ms apart [L]:
//!
//! ```text
//! 05:44:25.087 <- 0x017E  9 bytes  00000000 01000000 00      mode 0 = create, roomType 1
//! 05:44:25.095 <- 0x017E  8 bytes  05000000 d5000000         mode 5 = invite, target 213
//! ```
//!
//! `FUN_141826bc0` builds both from one stack buffer via `COutPacket::Init`, so a reader who
//! greps for the invite alone sees only half of it - which is what happened first.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials. An invite is relayed on the say-so of whoever
//! holds the connection, and the target is not checked for being on the same map.

use crate::packet::{PacketReader, PacketWriter};

/// Client -> server. Every miniroom action: create, invite, accept, decline and 20 more modes.
///
/// **It does not latch** - it is absent from [`crate::dropmoney::LATCHING_REQUESTS`], checked
/// with `0x0143` and `0x01FD` present as the positive control, which is consistent with every
/// archived run logging it `not answered yet` while the client played on [L].
pub const CLIENT_MINIROOM: u16 = 0x017E;

/// Server -> client. The answer, and mode 5 is the one that raises the invite popup.
///
/// Pinned to `FUN_141C3D3E0`, whose sole caller is `CField::OnPacket` at `0x141821ac8`, by
/// three controls [L]: the handler's 27-slot table size matches `(0x18223C4-0x1822358)/4`;
/// `research/storage.md` decoded that **same** table independently and pinned `0x0572` to
/// storage, which the decode reproduces exactly; and the handler's own decrypted strings are
/// *"You can't establish a miniroom right here"* and *"'%s' has denied the invitation"*.
pub const MINIROOM_RESULT: u16 = 0x0575;

/// `roomType` for a trade room, from the create packet's second field [L].
pub const ROOM_TYPE_TRADE: u32 = 1;

/// The invite's `type` field: an ordinary trade. **This value is load-bearing** - see
/// [`invite`].
pub const INVITE_TRADE: u32 = 1;

/// The invite's `type` field: a cash-item trade. The only other accepted value.
pub const INVITE_CASH_TRADE: u32 = 2;

/// What a client asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    /// Mode 0: open a room. `room_type` 1 is a trade.
    Create { room_type: u32 },
    /// Mode 5: invite `target` (a character id) into the room just created.
    Invite { target: u32 },
    /// Mode 3: accept an invite, echoing the `ticket` the invite carried.
    Accept { ticket: u32 },
    /// Mode 6: decline. The client sends `reason` 4 normally, and `0xB` when it already has
    /// a miniroom open [L].
    Decline { ticket: u32, reason: u32 },
    /// Mode `0x10` sub 0: put `quantity` of the item in bag slot `bag_slot` of tab `inv_type`
    /// (the wire's tab number, 1 = Equip .. 5 = Cash) into trade slot `trade_slot` (1..=9).
    /// Captured 2026-10-03, 21 eggs from Use slot 4 into trade slot 1:
    /// `10000000 00000000 02 0400 1500 01` [L].
    PutItem { inv_type: u8, bag_slot: i16, quantity: u16, trade_slot: u8 },
    /// Mode `0x10` sub 1: offer `amount` mesos. Captured the same day, 3000 mesos:
    /// `10000000 01000000 b80b000000000000` - a **u64** [L]. Whether it is the new total or an
    /// increment is not read off the client; it is taken as the total, which is what the room
    /// echo stores (`mesos[seat] = amount`, `FUN_14214A9C0`) **[I]**.
    PutMesos { amount: u64 },
    /// Mode `0x10` sub 2: **the Trade button**. `u8 count`, then per item THIS side put in a
    /// `(u32 itemId, u32 checksum)` pair. Captured 2026-10-03 [L]: a side with only mesos in
    /// sent `10000000 02000000 00`; a side with arrows and a scroll in sent count 2,
    /// `(0x1F6EE0 = 2060000, ..)` and `(0x1F4000 = 2048000, ..)` - its own two items.
    TradeConfirm { items: Vec<(u32, u32)> },
    /// Mode `0x10` sub 5: the answer a client gives BY ITSELF when told its partner pressed
    /// Trade (inbound sub 2, `FUN_14214B090`): the same pairs for every item it sees on the
    /// PARTNER's side (`items[1]`). Lets the server check both screens show one table.
    TradeVerify { items: Vec<(u32, u32)> },
    /// Mode `0x10` with any other sub-action. Carried so the log names it.
    TradeOther { sub: u32 },
    /// Mode 8: a line typed in the trade window's chat. `u32` (a client tick - it rose
    /// between the two captured lines), then the text. Captured 2026-10-03:
    /// `08000000 0a0c4704 0500 68656c6c6f` = "hello" [L].
    Chat { text: String },
    /// Mode `0x0C`, body nothing but the mode: the trade window was closed by its own player
    /// (`FUN_142147160`, the trade dialog's `vt+0x138`, sends it on close result 2, then closes
    /// its own window) [L].
    Leave,
    /// One of the other modes. Carried rather than dropped so a handler can log which.
    Other { mode: u32 },
}

/// Decode a `0x017E` body - everything after the two opcode bytes.
pub fn parse_request(body: &[u8]) -> Option<Request> {
    let mut c = PacketReader::new(body);
    let mode = c.u32().ok()?;
    Some(match mode {
        0 => Request::Create { room_type: c.u32().ok()? },
        3 => Request::Accept { ticket: c.u32().ok()? },
        5 => Request::Invite { target: c.u32().ok()? },
        6 => {
            let ticket = c.u32().ok()?;
            Request::Decline { ticket, reason: c.u32().ok()? }
        }
        ROOM_LEAVE => Request::Leave,
        ROOM_CHAT => {
            let _tick = c.u32().ok()?;
            Request::Chat { text: c.str().ok()? }
        }
        TRADE_ACTION => match c.u32().ok()? {
            TRADE_PUT_ITEM => Request::PutItem {
                inv_type: c.u8().ok()?,
                bag_slot: c.i16().ok()?,
                quantity: c.u16().ok()?,
                trade_slot: c.u8().ok()?,
            },
            TRADE_PUT_MESOS => Request::PutMesos { amount: c.u64().ok()? },
            sub @ (TRADE_CONFIRM | TRADE_VERIFY) => {
                let n = c.u8().ok()?;
                let mut items = Vec::with_capacity(usize::from(n));
                for _ in 0..n {
                    items.push((c.u32().ok()?, c.u32().ok()?));
                }
                if sub == TRADE_CONFIRM {
                    Request::TradeConfirm { items }
                } else {
                    Request::TradeVerify { items }
                }
            }
            sub => Request::TradeOther { sub },
        },
        other => Request::Other { mode: other },
    })
}

/// `0x0575` mode 5: raise "Trade request from <name>" on the invited player's screen.
///
/// # The body, all five reads counted rather than eyeballed
///
/// `tools/reads.py 0x141c3e110 4` reports **exactly five reads, all direct, none gated**, and
/// the listing confirms the first conditional branch is *after* all five [L]:
///
/// ```text
/// u32 mode = 5
/// u32 type          1 = trade, 2 = cash trade
/// u32 id            looked up locally; a HIT auto-declines - see below
/// str inviterName   u16 length prefix then that many bytes
/// u32               read and DISCARDED - but it must still be sent
/// u32 ticket        echoed back by the accept/decline
/// ```
///
/// **22 + `name.len()` bytes.** A body one field short is how `0x02AD` killed a client on this
/// same day, so the length is asserted in a test.
///
/// # Why `type` is the whole bug
///
/// `FUN_14180FA70` is handed fields 2, 4 and 6, and its first act is [L]:
///
/// ```text
/// 14180fa9d  cmp r9d, 1 / jne ...      type 1 -> balloon kind 0x15, "Trade request from"
/// 14180faa9  cmp r9d, 2 / jne 14180fe32    anything else -> RETURN. No popup. No error.
/// ```
///
/// Corroborated by `FUN_140426CB0`, which is literally `return n == 1 || n == 2;` and is
/// called on this same field from both the inbound and the outbound menu path [L]. So a
/// zeroed or misplaced `type` reproduces the owner's symptom exactly: no popup, no complaint.
///
/// And it is the **only** path, not merely one of them. A scan of every writer of
/// `balloon+0x300` - the field that selects which balloon is drawn - returns 31 sites in the
/// UI image, and **30 of them store a literal immediate**, one dedicated setter per balloon
/// kind. Neither `0x15` nor `0x16` is among those immediates. The single site that can produce
/// either is the computed `eax` at `14180fab8`, inside the `type ∈ {1,2}` gate above [L]. That
/// is what makes a wrong `type` a complete and silent failure rather than a degraded one:
/// there is no second way to build this popup.
///
/// The instrument caught its own near-miss and that is why the number is trustworthy - the
/// first scan filtered for stores of `0x15`/`0x16` and so **structurally could not see** the
/// one writer that computes the value. `research/trade-2026-09-09.md` §2 records both runs.
///
/// # `id` and the auto-decline
///
/// Field 3 goes to a lookup in a collection on the session global, and a **hit** makes the
/// client auto-decline with reason 4 and show nothing. The helper has 19 call sites across
/// unrelated subsystems, so which collection it is has **not** been determined; the inviter's
/// character id is the natural candidate **[I]**. That is what is sent here, and the failure
/// mode if it is wrong is a silent auto-decline rather than a crash - the same symptom as
/// today, so trying it costs nothing that is not already lost.
pub fn invite(kind: u32, id: u32, inviter_name: &str, ticket: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(5);
    w.u32(kind);
    w.u32(id);
    w.str(inviter_name);
    w.u32(0); // read and discarded by the client, and still required
    w.u32(ticket);
    w.into_vec()
}

/// The byte count [`invite`] produces, so a caller can assert it without rebuilding.
pub fn invite_len(inviter_name: &str) -> usize {
    22 + inviter_name.len()
}

// ---------------------------------------------------------------------------------------
// 0x0575 mode 4 - the packet that OPENS the trade window
// ---------------------------------------------------------------------------------------

/// Mode 4 with `A == 0`: **create the room window and fill it.**
///
/// The owner, 2026-09-22: *"Tester2 just sent the owner a trade request, but after the owner accepts it, the
/// Trade window did not open."* It did not because this packet did not exist:
/// `research/trade-2026-09-09.md` §3 left mode 4's payload undecoded, since it runs through a
/// virtual call on whichever miniroom class the room type selects. That call is now read.
///
/// # The body, from the two functions that consume it
///
/// `FUN_141C3D980` (the mode switch) and `FUN_141C3ED00` (the payload), both **[L]**:
///
/// ```text
/// u32 mode = 4
/// u32 A    = 0          non-zero is a notice code instead, and the body stops at 12 bytes
/// u32 B    = roomType   1 -> new(0x1678) + FUN_142146D90, the TRADE dialog
/// u32                   -> room+0x308, read only when A == 0
/// u8  capacity          -> room+0x2fc: the loop bound over the member slots (FUN_141c3cef0
///                          walks 0..capacity), so 2 for a trade
/// u8  mySlot            -> room+0x2f8: which slot the RECIPIENT is, indexed by FUN_141c3d0f0
/// repeat until a byte with bit 7 set, or >= 8:
///     u8   slot
///     ...  avatar look  the vtable slot +0x1C8 for the trade class is FUN_141C423D0, and its
///                       ONLY packet read is FUN_1402ee8d0 - the same avatar decoder
///                       `0x0224` uses, i.e. `opcode::avatar_look`
///     u32  characterId  -> member[slot]+0
///     str  name         -> member[slot]+8
///     u16               -> member[slot]+0x10
/// u8  0xFF              ends the list
/// ```
///
/// # The one thing still unread, and what it costs
///
/// After the member loop the handler makes a final virtual call on a UI singleton -
/// `(*DAT_143aa8520)[0x188]`, resolved through the vtable that
/// `FUN_14177fe00` installs (`0x1433D31C8`) to `FUN_142bf2540`, which
/// `tools/reads.py 0x142bf2540 4` reports as reading **nothing**. **[D]**, because the slot
/// arithmetic lands in a region shared with the class's second vtable. If that resolution is
/// wrong the body is short by whatever it does read, and a short body is how `0x02AD` killed
/// a client - so plan step 13 watches for a fault right after Accept rather than assuming.
pub const ROOM_OPEN_MODE: u32 = 4;

/// `A` for "open a room". Any other value makes the client draw a notice and read no further.
pub const ROOM_OPEN_CREATE: u32 = 0;

/// A trade has two seats. This is the `capacity` byte, and the loop bound the client walks.
pub const TRADE_CAPACITY: u8 = 2;

/// The member list ends on a byte the client reads as negative (`test al,al / js`).
pub const MEMBER_LIST_END: u8 = 0xFF;

/// One seat of a trade window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomMember {
    /// 0 or 1 for a trade. A slot `>= 8` ends the list as surely as [`MEMBER_LIST_END`].
    pub slot: u8,
    pub character_id: u32,
    pub name: String,
    /// [`crate::opcode::avatar_look`] of that character - the same bytes `0x0224` carries.
    pub look: Vec<u8>,
}

/// Build the room-open body. `my_slot` is **the slot of the player this copy is sent to**,
/// so the two sides of one trade get two different packets.
pub fn room_open(my_slot: u8, capacity: u8, members: &[RoomMember]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(ROOM_OPEN_MODE);
    w.u32(ROOM_OPEN_CREATE);
    w.u32(ROOM_TYPE_TRADE);
    w.u32(0); //            -> room+0x308; nothing this server sends is read back out of it
    w.u8(capacity); //      -> room+0x2fc
    w.u8(my_slot); //       -> room+0x2f8
    for m in members {
        w.u8(m.slot);
        w.bytes(&m.look);
        w.u32(m.character_id);
        w.str(&m.name);
        w.u16(0); //        -> member+0x10, whose reader is not identified
    }
    w.u8(MEMBER_LIST_END);
    w.into_vec()
}

/// What [`room_open`] will produce, without building it.
pub fn room_open_len(members: &[RoomMember]) -> usize {
    // 12 for the three mode words, 4 for the one that lands in room+0x308, then the two
    // flag bytes, the members, and the terminator.
    18 + members.iter().map(|m| 1 + m.look.len() + 4 + 2 + m.name.len() + 2).sum::<usize>() + 1
}

// ---------------------------------------------------------------------------------------
// 0x0575 mode 6 - how the invite went, in the inviter's chat
// ---------------------------------------------------------------------------------------

/// Mode 6: **the invite's result, for the inviter.** `FUN_141C3E360`, reached straight from
/// the mode table (`141c3d44f jmp`) with no check on an open dialog - which matters, because
/// the inviter has no window yet. [L]
///
/// ```text
/// raw 4 result       0 -> nothing at all
/// switch result-1 (15 entries, table 0x141C3E79C), each a string id, then
/// FUN_1415eca30(text, 0xB): a chat line in category 11, the client's own red-pink
///   1   0x197  "Unable to find the character."              no further read
///   2   0x1C8  "'%s' is doing something else right now."   str name
///   3   0x1C9  "'%s' has denied the invitation."           str name (and closes an open
///                                                            miniroom dialog, vt+0x138)
///   4   0x1CA  "'%s' is currently not accepting any invitation."   str name
///   15  0x1CB  "'%s' is a character that cannot trade cash items." str name
///   5..12, 14  the Rock-Paper-Scissors challenge strings; 13 nothing
/// ```
pub const INVITE_RESULT: u32 = 6;
/// "Unable to find the character." - reads no name.
pub const INVITE_NOT_FOUND: u32 = 1;
/// "'%s' is doing something else right now."
pub const INVITE_BUSY: u32 = 2;
/// "'%s' has denied the invitation."
pub const INVITE_DENIED: u32 = 3;

/// `0x0575` mode 6. `name` goes on the wire for every result but
/// [`INVITE_NOT_FOUND`], which reads none - a name there would be bytes the client never
/// reads, harmless, but not what it expects.
pub fn invite_result(result: u32, name: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(INVITE_RESULT);
    w.u32(result);
    if result != INVITE_NOT_FOUND {
        w.str(name);
    }
    w.into_vec()
}

// ---------------------------------------------------------------------------------------
// Mode 0x10 - what goes INTO the trade window, and mode 0x0C - leaving it
// ---------------------------------------------------------------------------------------

/// Mode `0x10`, both directions: the trade room's own actions, a `u32` sub-action first.
///
/// Inbound it is not in the miniroom handler's `3..=0xD` table, so it takes the default arm
/// (`141c3d791`), which hands `(mode, packet)` to the open dialog's `vt+0x178`. For the trade
/// dialog (vtable `0x143431CC8`, the same one `research/trade-2026-09-09.md` resolved) that is
/// `FUN_14214A9C0`, read in full [L]:
///
/// ```text
/// if mode != 0x10: return          ; nothing read
/// raw 4 -> sub
///   sub 0 -> FUN_14214AE50   u8 seat, u8 tradeSlot (1-based, 9 of them), GW_ItemSlot
///                            (FUN_140303530: u8 type then the type's body - the same
///                            decoder the bag and storage use) -> items[seat][tradeSlot-1]
///   sub 1 ->                 u8 seat, u64 mesos             -> mesos[seat] = mesos
///   sub 2 -> FUN_14214B090   reads nothing: SENDS 0x017E 0x10/5 with the CRC of every
///                            item in items[1], and sets room+0x504
///   sub 6 -> FUN_14214B3B0   reads nothing
/// ```
pub const TRADE_ACTION: u32 = 0x10;
/// Sub-action 0: an item.
pub const TRADE_PUT_ITEM: u32 = 0;
/// Sub-action 1: mesos.
pub const TRADE_PUT_MESOS: u32 = 1;
/// Sub-action 2 - outbound the Trade button, inbound "your partner pressed Trade" (no body:
/// `FUN_14214A9C0` reads nothing more, and calls `FUN_14214B090`, which marks the partner as
/// ready - `room+0x504 = 1` and a redraw, the indicator - and sends [`TRADE_VERIFY`]).
pub const TRADE_CONFIRM: u32 = 2;
/// Sub-action 5, outbound only: the checksums a client sends back on an inbound
/// [`TRADE_CONFIRM`].
pub const TRADE_VERIFY: u32 = 5;

/// `0x0575` mode `0x10` sub 2: **your partner pressed Trade.** 8 bytes.
pub fn partner_confirmed() -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(TRADE_ACTION);
    w.u32(TRADE_CONFIRM);
    w.into_vec()
}

/// The trade grid: the client's own loop bound over `items[seat]` (`cmp r14d, 9`) [L].
pub const TRADE_SLOTS: u8 = 9;

/// **The seat byte is RELATIVE: 0 is the player this copy goes to, 1 is their partner.** [D]
///
/// The two offers live in fixed arrays - `items` at `room+0x508`, mesos at `room+0x518` -
/// indexed by the packet's byte alone; nothing on either path reads `room+0x2f8` (`mySlot`),
/// and the only writers of `room+0x518` are the two inbound readers. The draw routine
/// `FUN_142148A40` draws `mesos[0]` then `mesos[1]` at fixed places, and the meso check in
/// `FUN_14214AAA0` takes `mesos[1] - mesos[0]` as the net gain - partner's minus mine.
/// Against the member list, which is **absolute** (`room_open`'s `slot`, `mySlot` saying
/// which is you). The reference server (a different version, a candidate only) sends 0 to
/// the putter and 1 to the partner, which agrees. Plan step 43 settles it on screen.
pub const SEAT_SELF: u8 = 0;
/// See [`SEAT_SELF`].
pub const SEAT_PARTNER: u8 = 1;

/// `0x0575` mode `0x10` sub 0: `item` (a whole `GW_ItemSlot`, type byte first - what
/// `Session::item_blob` makes) is in trade slot `trade_slot` of `seat`'s side.
///
/// **Nothing on the client writes its own offer** - the arrays are only filled from here - so
/// the player who put the item needs this echo as much as the partner does.
pub fn put_item(seat: u8, trade_slot: u8, item: &[u8]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(TRADE_ACTION);
    w.u32(TRADE_PUT_ITEM);
    w.u8(seat);
    w.u8(trade_slot);
    w.bytes(item);
    w.into_vec()
}

/// `0x0575` mode `0x10` sub 1: `seat`'s side now offers `mesos`. **17 bytes** [L].
pub fn put_mesos(seat: u8, mesos: u64) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(TRADE_ACTION);
    w.u32(TRADE_PUT_MESOS);
    w.u8(seat);
    w.u64(mesos);
    w.into_vec()
}

/// Mode 8, both directions: the room's chat.
///
/// Inbound it is the table's `8` arm (`141c3d5bc`), which hands the packet to the dialog's
/// `vt+0x198` = `FUN_141C3F300` [L]:
///
/// ```text
/// raw 4 sub
///   sub 1   u8 kind, str name -> FUN_141C414F0: a room notice ("[%s] has entered.",
///           "[%s] has left." ...), not chat
///   sub 0   u8 slot      the speaker's ABSOLUTE member slot; for a trade (room+0x304 == 1)
///                        compared with mySlot to pick the colour (mine 0, theirs 2)
///           str name
///           str text
///           u32          (the reference names it the partner's id)
///           u32          (the speaker's id)
///           chat info    FUN_1408D6760 - the block `megaphone::chat_info` writes, proven by
///                        party chat and the megaphones
/// ```
///
/// The client draws nothing when its player types: the line comes back from the server, to
/// both windows. The reference server (a different version) sends this same shape.
pub const ROOM_CHAT: u32 = 8;
/// Sub-action 0 of [`ROOM_CHAT`]: a player's line.
pub const ROOM_CHAT_LINE: u32 = 0;

/// `0x0575` mode 8 sub 0: `who`, sitting in member `slot`, said `text`. `partner` is the
/// other player in the room.
pub fn chat(slot: u8, who: &crate::megaphone::Speaker, partner: u32, text: &str) -> Vec<u8> {
    let text = crate::notice::ascii_fold(text);
    let mut w = PacketWriter::new();
    w.u32(ROOM_CHAT);
    w.u32(ROOM_CHAT_LINE);
    w.u8(slot);
    w.str(who.name);
    w.str(&text);
    w.u32(partner);
    w.u32(who.character_id);
    crate::megaphone::chat_info(&mut w, who, &text);
    w.into_vec()
}

/// Mode `0x0C`, both directions: somebody left the room.
///
/// Inbound it is the table's `0xC` arm, the dialog's `vt+0x168` = `FUN_141C3EF70` [L]:
/// `u8 slot` (the **absolute** member slot, `room_open`'s), `raw 4 reason`, then the member is
/// cleared and `vt+0x190` = `FUN_14214AAA0` runs. That compares `slot` with the room's own
/// `mySlot` (`FUN_141C3F8B0` is `mov eax,[rcx+0x2f8]`): **only when they match does the window
/// close**, with the message `reason` picks. A slot that is not the recipient's just redraws.
/// So the window that should close is told its OWN slot.
pub const ROOM_LEAVE: u32 = 0x0C;

/// Leave reasons, `FUN_14214AAA0`'s table at `0x14214AE08` (`reason - 1`, 15 entries), the
/// strings decrypted with `tools/dump_stringids.py` [L]. 2 and 4..=7 close with no message.
/// `0x01CD` "Trade cancelled."
pub const LEAVE_CANCELLED: u32 = 1;
/// `0x01CC` "Trade cancelled. by the other character."
pub const LEAVE_CANCELLED_BY_PARTNER: u32 = 3;
/// `0x01CE` "Trade successful. Please check the results.", or `0x01CF` with the mesos received
/// when the client's own meso stat rose since the window opened - so a completion must send
/// the new balance first.
pub const LEAVE_TRADE_DONE: u32 = 9;
/// `0x01D6` "...the other person's on a different map."
pub const LEAVE_DIFFERENT_MAP: u32 = 11;
/// `0x01DA` "There was a problem trading the item. Please try again."
pub const LEAVE_PROBLEM: u32 = 12;
/// `0x01D0` "Trade unsuccessful."
pub const LEAVE_UNSUCCESSFUL: u32 = 13;

/// `0x0575` mode `0x0C`: member `slot` left, for `reason`. **9 bytes.**
pub fn room_leave(slot: u8, reason: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(ROOM_LEAVE);
    w.u8(slot);
    w.u32(reason);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mode 6: result then the name, except "not found", which reads no name.
    #[test]
    fn the_invite_result_carries_a_name_except_not_found() {
        assert_eq!(invite_result(INVITE_BUSY, "Wisp"), vec![6, 0, 0, 0, 2, 0, 0, 0, 4, 0, b'W', b'i', b's', b'p']);
        assert_eq!(invite_result(INVITE_NOT_FOUND, "Wisp"), vec![6, 0, 0, 0, 1, 0, 0, 0]);
    }

    /// The two puts captured 2026-10-03, byte for byte.
    #[test]
    fn the_captured_puts_decode() {
        let eggs = [0x10u8, 0, 0, 0, 0, 0, 0, 0, 0x02, 0x04, 0x00, 0x15, 0x00, 0x01];
        assert_eq!(
            parse_request(&eggs),
            Some(Request::PutItem { inv_type: 2, bag_slot: 4, quantity: 21, trade_slot: 1 })
        );
        let mesos = [0x10u8, 0, 0, 0, 1, 0, 0, 0, 0xb8, 0x0b, 0, 0, 0, 0, 0, 0];
        assert_eq!(parse_request(&mesos), Some(Request::PutMesos { amount: 3000 }));
        assert_eq!(parse_request(&[0x0c, 0, 0, 0]), Some(Request::Leave));
        let hello = [8u8, 0, 0, 0, 0x0a, 0x0c, 0x47, 0x04, 5, 0, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(parse_request(&hello), Some(Request::Chat { text: "hello".into() }));
        assert_eq!(parse_request(&[0x10, 0, 0, 0, 7, 0, 0, 0]), Some(Request::TradeOther { sub: 7 }));
        // The Trade button, both captured bodies of 2026-10-03.
        assert_eq!(parse_request(&[0x10, 0, 0, 0, 2, 0, 0, 0, 0]), Some(Request::TradeConfirm { items: vec![] }));
        let two = [
            0x10u8, 0, 0, 0, 2, 0, 0, 0, 2, 0xe0, 0x6e, 0x1f, 0x00, 0xab, 0xde, 0xda, 0xde, 0x00, 0x40, 0x1f, 0x00, 0x83, 0x38, 0x14, 0xd9,
        ];
        assert_eq!(
            parse_request(&two),
            Some(Request::TradeConfirm { items: vec![(2_060_000, 0xdedadeab), (2_048_000, 0xd9143883)] })
        );
        assert_eq!(parse_request(&two[..24]), None, "a pair one byte short does not decode");
        assert_eq!(partner_confirmed(), vec![0x10, 0, 0, 0, 2, 0, 0, 0]);
        assert_eq!(parse_request(&eggs[..13]), None, "a put one byte short does not decode");
    }

    /// The field order each inbound reader takes, and the lengths.
    #[test]
    fn the_inbound_trade_bodies_are_laid_out_as_read() {
        let m = put_mesos(SEAT_PARTNER, 3000);
        assert_eq!(m.len(), 17);
        assert_eq!(&m[0..8], &[0x10, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(m[8], 1, "seat");
        assert_eq!(u64::from_le_bytes(m[9..17].try_into().unwrap()), 3000);

        let i = put_item(SEAT_SELF, 1, &[2, 0xAA, 0xBB]);
        assert_eq!(i, vec![0x10, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0xAA, 0xBB]);

        assert_eq!(room_leave(1, LEAVE_CANCELLED_BY_PARTNER), vec![0x0c, 0, 0, 0, 1, 3, 0, 0, 0]);
    }

    /// The two captured bodies, byte for byte, 8 ms apart in `world.log`.
    #[test]
    fn the_captured_create_and_invite_decode() {
        let create = [0u8, 0, 0, 0, 1, 0, 0, 0, 0];
        assert_eq!(create.len(), 9);
        assert_eq!(parse_request(&create), Some(Request::Create { room_type: ROOM_TYPE_TRADE }));

        // 05000000 d5000000 - mode 5, target 0xD5 = 213 = Cobalt.
        let inv = [5u8, 0, 0, 0, 0xd5, 0, 0, 0];
        assert_eq!(inv.len(), 8);
        assert_eq!(parse_request(&inv), Some(Request::Invite { target: 213 }));
    }

    #[test]
    fn accept_and_decline_decode() {
        assert_eq!(
            parse_request(&[3, 0, 0, 0, 9, 0, 0, 0, 0, 0]),
            Some(Request::Accept { ticket: 9 })
        );
        assert_eq!(
            parse_request(&[6, 0, 0, 0, 9, 0, 0, 0, 4, 0, 0, 0]),
            Some(Request::Decline { ticket: 9, reason: 4 })
        );
    }

    #[test]
    fn an_unknown_mode_is_carried_not_dropped() {
        assert_eq!(parse_request(&[0x0b, 0, 0, 0]), Some(Request::Other { mode: 0x0b }));
        assert_eq!(parse_request(&[1, 0]), None, "too short to hold a mode");
    }

    /// **22 + name.** A body one field short is how `0x02AD` killed a client on 2026-09-09;
    /// if this ever fails downward the popup packet is a client-killer.
    #[test]
    fn the_invite_is_twenty_two_plus_the_name() {
        for name in ["Tester2", "", "aVeryLongCharacterName"] {
            let b = invite(INVITE_TRADE, 214, name, 7);
            assert_eq!(b.len(), 22 + name.len(), "name {name:?}");
            assert_eq!(b.len(), invite_len(name));
        }
    }

    /// The field order, and the one that decides whether anything is drawn at all.
    #[test]
    fn the_type_field_is_second_and_must_be_one_or_two() {
        let b = invite(INVITE_TRADE, 214, "Tester2", 7);
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 5, "mode");
        assert_eq!(u32::from_le_bytes(b[4..8].try_into().unwrap()), INVITE_TRADE);
        assert_eq!(u32::from_le_bytes(b[8..12].try_into().unwrap()), 214, "the id");
        assert_eq!(u16::from_le_bytes(b[12..14].try_into().unwrap()), 7, "u16 length prefix");
        assert_eq!(&b[14..21], b"Tester2");
        assert_eq!(u32::from_le_bytes(b[21..25].try_into().unwrap()), 0, "discarded, still sent");
        assert_eq!(u32::from_le_bytes(b[25..29].try_into().unwrap()), 7, "ticket");
        // Only 1 and 2 draw anything - FUN_140426CB0 is `n == 1 || n == 2`.
        assert!(matches!(INVITE_TRADE, 1) && matches!(INVITE_CASH_TRADE, 2));
    }

    /// It must stay OUT of the latching list: this opcode is answered with a real packet or
    /// with nothing, and the unlock would be a lie about what happened.
    #[test]
    fn the_miniroom_opcode_does_not_latch() {
        assert!(!crate::dropmoney::latches_the_exclusive_request(CLIENT_MINIROOM));
        // The control: the list can speak.
        assert!(crate::dropmoney::latches_the_exclusive_request(0x0143));
    }
}
