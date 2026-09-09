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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// One of the other 20 modes. Carried rather than dropped so a handler can log which.
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

#[cfg(test)]
mod tests {
    use super::*;

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
