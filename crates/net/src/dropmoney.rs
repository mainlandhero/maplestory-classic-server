//! `0x0143` - the player asking to drop mesos on the floor, and the reply that lets the
//! inventory move again afterwards.
//!
//! The owner, 2026-09-08: *"I have attempted to drop 10 mesos and 5000 mesos, none of these
//! attempts worked, but I lose all functionality in being able to interact with my
//! inventory."*
//!
//! That is not two bugs. It is one: **the request was never answered**, and this request
//! latches.
//!
//! # The opcode, and how it was established
//!
//! `research/msexe-send-opcodes.txt` names one builder for `0x0143`, `FUN_142d4cb40`, and
//! its listing is the whole story. **[L]**
//!
//! ```text
//! 142d4cb5f  movsxd rdi, edx              the amount, sign-extended
//! 142d4cb68  mov    edx, 0xc8
//! 142d4cb6d  call   142cc42d0             THE GATE - 200 ms, and refuses while +0x2330 is set
//! 142d4cb74  je     142d4cd21             ... refused: return having sent NOTHING
//! 142d4cbb3  lea    rcx, [rax+0xbf]       the player's money
//! 142d4cbc5  cmp    rdi, rax
//! 142d4cbc8  jle    142d4cc09             amount <= money: proceed. else UI message 0x9d
//! 142d4cc8f  mov    edx, 0x143            <- the opcode
//! 142d4cca2  call  1429e3ef0 -> encode    u32 tick
//! 142d4ccb6  mov   edx, edi -> encode     u32 amount
//! 142d4ccd2  mov    edx, 1
//! 142d4ccda  call   142cc4430             THE LATCH - [player+0x2330] = 1
//! ```
//!
//! The read of the player's money at `+0xbf` is the field `stat::MESO` writes, so the
//! affordability check identifies this builder as the money one and not a neighbour.
//!
//! Two captures agree, three days apart, deduplicated by `(timestamp, opcode, body)` across
//! `previous-runs/` and `research/fixtures/`: **[L]**
//!
//! ```text
//! previous-runs/world-20260905-225220.log  02:52:05.677 <- 0x0143  61ea3d05 0a000000
//! world.log                                05:30:07.798 <- 0x0143  75571b10 0a000000
//! ```
//!
//! The first `u32` differs wildly between them (a tick), the second is identical and is
//! `10` - the amount the owner said they typed. Each session contains exactly **one** `0x0143`
//! although they tried twice, because the gate at `142d4cb6d` swallowed the second attempt
//! before it could be built.
//!
//! # Why the whole inventory dies with it
//!
//! `142cc42d0` is the client's one-exclusive-request-at-a-time gate and `142cc4430` is its
//! setter:
//!
//! ```text
//! 142cc42da  cmp dword [rcx+0x2338], 0 / jne -> return 0
//! 142cc42e8  cmp dword [rcx+0x2330], 0 / jne -> return 0      <- the latch
//! 142cc4439  mov dword [rcx+0x2330], edx                      <- the setter
//! ```
//!
//! 69 outbound opcodes call that setter with a non-zero value - `0x010E` use-item,
//! `0x0138`/`0x0139` AP, `0x00D5` cash shop, this one - and `0x0107` makes 70 by storing to
//! `+0x2330` inline instead. Every one of them is refused, silently and before a byte is
//! built, while the latch is set. So an unanswered `0x0143` does not fail one drop; it kills
//! the inventory, the AP buttons, the cash shop and the item drop for the rest of the
//! session, which is exactly what the owner saw.

use crate::packet::{PacketReader, PacketWriter};

/// **Inbound `0x0143`, 8 bytes.** The player confirmed the "How many will you drop?" prompt
/// over the meso box.
pub const CLIENT_DROP_MONEY: u16 = 0x0143;

/// The body is two `u32`s and nothing else - `FUN_142d4cb40` encodes twice and stops.
pub const DROP_MONEY_BODY_LEN: usize = 8;

/// A parsed [`CLIENT_DROP_MONEY`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropMoney {
    /// The client's tick at the moment it built the packet. Never trusted for anything.
    pub tick: u32,
    /// How many mesos the player asked to drop.
    ///
    /// **Signed on purpose.** `142d4cb5f movsxd rdi, edx` sign-extends it and the only
    /// client-side check is `cmp rdi, rax / jle` against the player's money, which a
    /// negative number passes. Nothing here acts on the value, but a handler that ever does
    /// must not read it as a `u32`. **[L]**
    pub amount: i32,
}

/// Parse a [`CLIENT_DROP_MONEY`] body (opcode already stripped).
///
/// `None` for anything that is not exactly the eight bytes the builder writes. The caller
/// must still answer a `None` - the client latched before it knew whether we could read it.
pub fn parse_drop_money(body: &[u8]) -> Option<DropMoney> {
    if body.len() != DROP_MONEY_BODY_LEN {
        return None;
    }
    let mut r = PacketReader::new(body);
    let tick = r.u32().ok()?;
    let amount = r.i32().ok()?;
    Some(DropMoney { tick, amount })
}

/// A [`crate::combat::STAT_CHANGED`] whose only job is to clear `player+0x2330`.
///
/// # Why this packet and not an `0x0070`
///
/// The `0x0070` refusal in [`crate::inventory::inventory_rejected`] would also clear the
/// latch, but it is an *InventoryOperation*: it tells the client's bag to run an entry loop
/// with `nCount = 0`. Nothing about a meso drop touches a bag slot, and answering a request
/// with a packet about a different subsystem is the "wrong reply" half of the rule in
/// `CLAUDE.md` - it clears the latch and leaves the client somewhere nobody has read.
///
/// `0x007C` is the packet whose *first action* is the unlock, before it looks at anything
/// else at all: **[L]**
///
/// ```text
/// 142d547ad  call 1406e8ae0        read u8 bExclRequestSent
/// 142d547b2  test al, al
/// 142d547b4  je   142d547c0
/// 142d547b6  xor  edx, edx
/// 142d547b8  mov  rcx, r14
/// 142d547bb  call 142cc4430        [ctx+0x2330] = 0   <- the unlock
/// 142d547c0  ...                   only now the character null-check, the flags, the mask
/// ```
///
/// So the unlock does not depend on the mask, on the character being loaded, or on any
/// branch below it.
///
/// # Why the mask is empty
///
/// Nothing changed, so there is nothing to announce - and an empty mask is the one shape
/// that provably fires no effect. `142d549fc mov eax, edi / and eax, 0x40000 / mov [rbp+8],
/// eax` stores *the meso bit of the mask*, and `142d56242 cmp [rbp+8], 0 / je` is the only
/// gate on the meso-gain effect `FUN_142d9bae0`. With mask `0` that branch is not taken, so
/// no "+0 mesos" number is drawn. **[L]**
///
/// The client also never deducted anything locally - `FUN_142d4cb40` reads the money for its
/// affordability check and writes nothing - so there is no stale HUD to correct.
///
/// Nine bytes, and `crate::combat::stat_changed` documents that the handler reads exactly
/// nine plus the mask decoder.
pub fn exclusive_request_unlock() -> Vec<u8> {
    crate::combat::stat_changed(&crate::combat::StatChange {
        excl_request: true,
        ..Default::default()
    })
}

/// The 69 outbound opcodes whose builder calls `FUN_142cc4430` with a non-zero value - i.e.
/// **every request the client refuses to send again until an inbound packet answers it.**
///
/// # How this list was produced, and where it is blind
///
/// For each `(opcode, builder)` in `research/msexe-send-opcodes.txt`, the builder was
/// disassembled and searched for `call 0x142cc4430` preceded by a `mov edx, <imm>` that is
/// not zero. Positive controls: `0x010E`, `0x0138`, `0x0139` and `0x00D5` are all present
/// and all four are independently documented in this repo as latching. Negative control:
/// `0x00D9` **is absent**, and 1082 archived `0x00D9`s went unanswered with the client
/// playing on for minutes.
///
/// The first version of this scan matched `mov edx, 0x1` and returned **four** opcodes with
/// the known control `0x0143` missing, because capstone prints small immediates in decimal.
/// That is the failure `CLAUDE.md` describes: a clean, confident number from an instrument
/// nobody checked.
///
/// **The blind spot, stated:** a builder that stores to `+0x2330` *inline* rather than
/// calling the setter is invisible here. At least one does - `FUN_142cc5b00`, the `0x0107`
/// inventory move, at `142cc5f01` - so `0x0107` is added by hand and this list is a **lower
/// bound**, not the set. **[L]** for the 69, **[D]** for the claim that it is nearly all of
/// them.
pub const LATCHING_REQUESTS: &[u16] = &[
    0x00D2, 0x00D3, 0x00D5, 0x00D6, 0x00D8, 0x00DA, 0x00DB, 0x00EC, 0x00EF, 0x00F8, 0x0105,
    0x0106, 0x0107, 0x010C, 0x010D, 0x010E, 0x0114, 0x0116, 0x011A, 0x011C, 0x0120, 0x0121,
    0x0123, 0x0124, 0x012C, 0x012D, 0x0132, 0x0136, 0x0137, 0x0138, 0x0139, 0x013B, 0x013C,
    0x013E, 0x0140, 0x0143, 0x0147, 0x014A, 0x0159, 0x0165, 0x0166, 0x0170, 0x017F, 0x0180,
    0x0186, 0x0189, 0x0190, 0x0197, 0x019D, 0x01A3, 0x01B4, 0x01B5, 0x01B7, 0x01B8, 0x01BD,
    0x01DC, 0x01FD, 0x01FF, 0x023E, 0x023F, 0x0245, 0x0248, 0x0260, 0x029B, 0x02E3, 0x02E5,
    0x02EA, 0x02ED, 0x02F6, 0x02F7,
];

/// Does this inbound opcode latch `player+0x2330` on the way out of the client?
///
/// See [`LATCHING_REQUESTS`], including the sentence about what it cannot see.
pub fn latches_the_exclusive_request(opcode: u16) -> bool {
    LATCHING_REQUESTS.binary_search(&opcode).is_ok()
}

/// The body a builder writes, for tests that need one.
pub fn drop_money_request(tick: u32, amount: i32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(tick);
    w.i32(amount);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two captured bodies, three days apart, both `10` mesos.
    ///
    /// Deduplicated by `(timestamp, opcode, body)` across `previous-runs/` and
    /// `research/fixtures/` - the two directories hold copies of each other and a file-level
    /// count double-counts, which `CLAUDE.md` records producing wrong numbers in the
    /// write-up of that very lesson.
    #[test]
    fn the_two_captured_meso_drops_decode_as_ten_mesos() {
        let sept5 = [0x61, 0xea, 0x3d, 0x05, 0x0a, 0x00, 0x00, 0x00];
        let sept8 = [0x75, 0x57, 0x1b, 0x10, 0x0a, 0x00, 0x00, 0x00];

        let a = parse_drop_money(&sept5).expect("8 bytes parse");
        let b = parse_drop_money(&sept8).expect("8 bytes parse");

        assert_eq!(a.amount, 10, "the owner typed 10");
        assert_eq!(b.amount, 10, "and 10 again three days later");
        // The discriminator: the ticks differ, so the second field is not part of a
        // constant header. That is what makes "u32 tick, u32 amount" a reading rather than
        // a guess.
        assert_ne!(a.tick, b.tick);
        assert_eq!(a.tick, 0x053d_ea61);
        assert_eq!(b.tick, 0x101b_5775);
    }

    #[test]
    fn a_body_of_the_wrong_length_is_refused_rather_than_half_read() {
        assert!(parse_drop_money(&[]).is_none());
        assert!(parse_drop_money(&[0; 7]).is_none());
        assert!(parse_drop_money(&[0; 9]).is_none());
        assert!(parse_drop_money(&[0; 8]).is_some());
    }

    /// The amount is signed. A `u32` read would turn -1 into four billion.
    #[test]
    fn the_amount_is_signed_because_the_client_sign_extends_it() {
        let b = drop_money_request(7, -1);
        assert_eq!(parse_drop_money(&b).unwrap().amount, -1);
    }

    /// The one byte this packet exists to deliver, and the empty mask beside it.
    #[test]
    fn the_unlock_sets_the_exclusive_byte_and_changes_no_stat() {
        let b = exclusive_request_unlock();
        assert_eq!(b.len(), 9, "three header bytes, a u32 mask, two trailing flags");
        assert_eq!(b[0], 1, "bExclRequestSent - read at 142d547ad, before anything else");
        assert_eq!(b[1], 0, "not quiet; with an empty mask nothing gates on it anyway");
        assert_eq!(b[2], crate::combat::STAT_CHANGED_BYTE3);
        let mask = u32::from_le_bytes([b[3], b[4], b[5], b[6]]);
        assert_eq!(mask, 0, "no stat changed, so no bit is set");
        // The meso-gain effect is gated on `mask & 0x40000` (142d549fc / 142d56242). An
        // empty mask cannot reach it, which is why this is the shape that draws nothing.
        assert_eq!(mask & crate::combat::stat::MESO, 0);
    }

    /// **Verify the instrument.** The controls that made the enumeration believable.
    #[test]
    fn the_latching_list_holds_its_controls_and_is_sorted() {
        // Positive controls - each documented as latching elsewhere in this repo.
        for op in [0x0107u16, 0x010E, 0x0138, 0x0139, 0x00D5, 0x0143] {
            assert!(latches_the_exclusive_request(op), "{op:#06X} must be in the list");
        }
        // Negative control: 1082 archived user-moves went unanswered and nothing froze.
        assert!(!latches_the_exclusive_request(0x00D9), "0x00D9 must NOT be in the list");
        // `binary_search` is a lie on an unsorted slice, and it fails silently.
        assert!(
            LATCHING_REQUESTS.windows(2).all(|w| w[0] < w[1]),
            "LATCHING_REQUESTS must be sorted and free of duplicates"
        );
    }
}
