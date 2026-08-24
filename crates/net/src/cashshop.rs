//! `0x00D5` - the Cash Shop button, which has been on the wire twice and was read as noise.
//!
//! # The correction, first, because it is the whole point of this file
//!
//! For two sessions this project reported that clicking Cash Shop **sent nothing**. It did.
//! `0x00D5` was in `world.log` both times, and it was dismissed because it arrives inside a
//! burst that also carries `0x0420`..`0x0426`, and that burst lands near the end of a session:
//!
//! ```text
//! 03:19:39.284  <- 0x0422     7 bytes
//! 03:19:39.293  <- 0x0421  1114 bytes
//! 03:19:39.293  <- 0x0420   133 bytes
//! 03:19:39.293  <- 0x0423   274 bytes
//! 03:19:39.293  <- 0x0426    20 bytes
//! 03:19:39.293  <- 0x00D5     5 bytes   e929ba0500
//! ```
//!
//! The whole burst was filed as "shutdown telemetry" **as a unit**, without separating the
//! opcodes in it. The control that breaks it costs one `grep` over the archived runs:
//!
//! ```text
//!                        0x00D5   0x0420
//!   five runs, no Cash Shop click     0        0..2      <- 0x0420 IS telemetry
//!   the two runs with a click         1        1         <- 0x00D5 only ever appears here
//! ```
//!
//! **[L]** `0x0420` appears without `0x00D5`; `0x00D5` never appears without a click. That is
//! `CLAUDE.md`'s "enumerate before you filter" exactly - the filter was *when it arrived*
//! rather than *which opcode it was*, and it produced a clean, confident, wrong negative that
//! stood for two days.
//!
//! # It is an EXCLUSIVE REQUEST, which is why only the first click of a session was ever seen
//!
//! `FUN_142caee70` - the sender, confirmed by a watch that fired on every click - checks the
//! same three fields `FUN_142cc42d0` gates every exclusive request on, inline: **[L]**
//!
//! ```asm
//! 142caef7c  cmp [ctx+0x2338], 0  ; jne -> silent return
//! 142caef88  cmp [ctx+0x2330], 0  ; jne -> silent return   <- the exclusive-request latch
//! 142caef9e  tick - [ctx+0x2334] < 0x1f4 -> silent return  <- 500 ms
//! ```
//!
//! And a peek on `[ctx+0x2330]` across three clicks five seconds apart says the rest:
//!
//! ```text
//! click 1  23:19:39.284  [ctx+0x2330] = 0   -> passed, sent 0x00D5, and SET the latch
//! click 2  23:19:44.915  [ctx+0x2330] = 1   -> silent return
//! click 3  23:19:50.866  [ctx+0x2330] = 1   -> silent return
//! ```
//!
//! **[L]** So the button is not broken and never was: it fires once, latches, and waits for a
//! reply that never comes. `research/cash-shop.md` part six.

use crate::opcode::{character_record_for_set_field_with_quests_and_skills, Character, EquipStats};
use crate::packet::{PacketReader, PacketWriter};

/// `0x00D5` - "take me to the Cash Shop".
///
/// Built by `FUN_142caee70`, whose `COutPacket` constructor at `0x142caf180` is recorded as
/// this opcode in `research/msexe-send-opcodes.txt`. The watch that fired on that function on
/// every click, and the `0x00D5` that appeared on the same millisecond, are two independent
/// readings of the same event. **[L]**
pub const CLIENT_CASH_SHOP_REQUEST: u16 = 0x00D5;

/// A decoded [`CLIENT_CASH_SHOP_REQUEST`]. Five bytes: a client tick and one byte.
///
/// `e9 29 ba 05 | 00` from the 2026-08-22 capture. The field order is `w_u32` then `w_u8`,
/// which is what `tools/encodes.py` reads out of the builder at `0x142caf192` and
/// `0x142caf1a0` - so the split is measured rather than a plausible reading of five bytes.
/// **[L]**
///
/// The server needs neither field to decide anything today; they are parsed so the log can
/// show them and so a body of the wrong length is noticed rather than assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CashShopRequest {
    /// The client's own millisecond tick, the same clock `0x013C` carries.
    pub tick: u32,
    pub flag: u8,
}

/// Body length of a [`CLIENT_CASH_SHOP_REQUEST`].
pub const CASH_SHOP_REQUEST_LEN: usize = 5;

/// Parse a `0x00D5` body (opcode already stripped). `None` if it is not five bytes.
pub fn parse_cash_shop_request(body: &[u8]) -> Option<CashShopRequest> {
    let mut r = PacketReader::new(body);
    let tick = r.u32().ok()?;
    let flag = r.u8().ok()?;
    Some(CashShopRequest { tick, flag })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The owner's real `0x00D5`, off the wire.** 2026-08-22, the first click of the session.
    #[test]
    fn the_real_request_parses() {
        let body = [0xe9, 0x29, 0xba, 0x05, 0x00];
        assert_eq!(body.len(), CASH_SHOP_REQUEST_LEN);
        let r = parse_cash_shop_request(&body).expect("five bytes");
        assert_eq!(r.tick, 0x05ba29e9, "w_u32 at 0x142caf192");
        assert_eq!(r.flag, 0, "w_u8 at 0x142caf1a0");

        // A body of the wrong length is refused rather than read past the end or padded.
        assert!(parse_cash_shop_request(&body[..4]).is_none());
        assert!(parse_cash_shop_request(&[]).is_none());
    }
}


// ===========================================================================================
// The reply: SetCashShop
// ===========================================================================================

/// `0x01A3` - **SetCashShop**, and it goes out on the connection the client is already using.
///
/// # There is no migrate, and that reverses what this project believed for two days
///
/// `research/cash-shop.md` §4 described a separate cash-shop process reached by a migrate, and
/// said so twice. Two agents reached the same answer independently and it is not that:
/// `0x01A0..0x01A3` are the **four cases of one stage forwarder**, `FUN_142097EE0`, and
/// `CField::OnPacket` chains into it. The ladder is four arms and no more: **[L]**
///
/// ```asm
/// 142097ee0  sub edx, 0x1a0 ; je  -> 0x01A0  field
/// 142097ee8  sub edx, 1     ; je  -> 0x01A1  farm field
/// 142097eed  sub edx, 1     ; je  -> 0x01A2  auction field
/// 142097ef2  cmp edx, 1     ; jne -> default
///                                  -> 0x01A3 at 14209ad60
/// ```
///
/// That is exactly why this server's `!map` `0x01A0` already works mid-session on a live
/// channel socket, and the `0x01A3` handler **never reads its `this`**, so it does not care
/// which stage forwarded it. `0x001A`, the socket-level migrate, has **no cash-shop branch at
/// all** - three reads, `u8 ok, u32 ip, u16 port`, no destination and no seed - so a separate
/// process remains a *deployment* choice that would still have to send this packet. **[L]**
///
/// # Why it is `0x01A3` of the four
///
/// Three independent discriminators, and the first is a closed loop: **[L]/[D]**
///
/// * `FUN_142caee70` - the `0x00D5` builder - writes `[CWvsContext+0x2d10]`
///   (`mov [rbx+0x2d10], edi` at `0x142caf06f`). `FUN_142cc42c0` reads it, two instruments
///   agree those are the only two touchers, and the getter has **one caller image-wide**: the
///   `0x01A3` handler. Request and response, closed.
/// * only `0x01A3` reads packet bytes after the character record.
/// * its stage's `OnPacket` reaches `originalPrice`, `discount`, `bombSale`, `mileageRate`,
///   `forcedCategory`; the same scan on its siblings returns `Town Map` for `0x01A1` and
///   `Etc/GlobalMarketData.img` for `0x01A2`. **The instrument discriminates** rather than
///   matching everything, which is what makes the positive worth anything.
///
/// **The opcode NUMBER is `[D]`, not `[L]`.** No `SetCashShop` has ever been on this wire.
/// `0x01A0` is the only member of the block with a live confirmation, and if this draws
/// nothing the number is the first thing to doubt.
pub const SET_CASH_SHOP: u16 = 0x01A3;

/// Bytes after the character record, before the margin: `u8 u8 u8`, `u16`, `u16`, `u32`.
pub const CASH_SHOP_TAIL_LEN: usize = 3 + 2 + 2 + 4;

/// The same margin `set_field_with_character_dressed_quests` carries, and for the same reason.
///
/// Surplus bytes are never looked at - the frame carries its own length - and this project has
/// now been short **twice in one week**: `0x007D` threw at 152 bytes and `0x007E` threw at 127,
/// both because a read walk under-counted a tail. Being long costs nothing and being short
/// kills the client, so the asymmetry decides it.
pub const CASH_SHOP_MARGIN: usize = 384;

/// Build a `0x01A3` body.
///
/// ```text
/// raw[8]              server clock base           read at 14209ad98, identical to SetField's
/// <character record>  FUN_140304B20(...)          read at 14209ade8, the SAME record SetField
///                                                 sends - not a cash-shop variant
/// u8, u8, u8          read and discarded          140d71efd / f02 / f0d
/// u16 nModifiedCommodity   0 jumps the whole loop 142d46be0
/// u16 nNotice              0 skips the loop       142d47287
/// u32 nSpecial             <= 0 skips the loop    142d474fa
/// ```
///
/// **All three counts are zero here.** The client ships `Etc/Commodity.img` and reads it
/// itself, so the modified-commodity list is a *delta*, not the catalogue - an empty one means
/// "nothing differs from your own data", which is exactly true of this server today.
/// `research/cash-shop-items.md` has the 159 rows that data contains.
///
/// **There is no fixed head beyond the FILETIME.** `SetField` reads a `u32 channel`, four more
/// fields and a `u16 stringCount` before its record; `0x01A3` reads none of them. Sending
/// SetField's head here would shift the record by 25 bytes and decode garbage. **[L]**
pub fn set_cash_shop(
    chr: &Character,
    world_id: u32,
    clock: u64,
    equips: &[(u8, u32, EquipStats)],
    quests: &crate::quest::QuestBook,
    skills: &[crate::skills::Skill],
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.bytes(&clock.to_le_bytes());
    w.bytes(&character_record_for_set_field_with_quests_and_skills(
        chr, world_id, equips, quests, skills,
    ));
    w.bytes(&[0u8; 3]);
    w.u16(0); // nModifiedCommodity - no row differs from the client's own Commodity.img
    w.u16(0); // nNotice
    w.u32(0); // nSpecial
    w.bytes(&vec![0u8; CASH_SHOP_MARGIN]);
    w.into_vec()
}

/// `0x00D1` with an **empty body** - the cash shop's Exit button.
///
/// The same opcode as the field transfer, which is why this needs saying: a portal walk sends
/// **35 bytes** and the Exit button sends **none**. The dispatcher has to split on the length,
/// and getting that backwards would make every portal in the game try to leave a cash shop.
/// `FUN_1410BCDA0`'s `exit` / `checkCash` / `chargeCash` / `cartBuy` chain. **[L]**
pub const CASH_SHOP_EXIT_BODY_LEN: usize = 0;

/// `0x05AD` - the wallet, and the only packet that carries it.
///
/// `u32 nxCredit, u32 maplePoint, u32 <read, range-checked, discarded>`. **[L]**
///
/// No `CWvsContext` opcode carries a balance: the two wallet setters have 4 and 2 call sites
/// image-wide and every one is in this class. So a cash shop with no `0x05AD` shows nothing to
/// spend, whatever the server thinks the player has.
///
/// # It is both a reply and an unprompted greeting
///
/// `0x03E0` is the request - empty body, throttled to once every 60 s, latched on `[this+0x74]`
/// which the `0x05AD` arm clears at `0x140D736DC`. Request and reply measured on both ends of
/// the same latch. **[L]** But nothing requires it to be a reply, and sending it straight after
/// `SetCashShop` means the balance is on screen before the player can ask.
pub const CASH_SHOP_WALLET: u16 = 0x05AD;

/// Body length of a [`CASH_SHOP_WALLET`]: three `u32`s.
pub const CASH_SHOP_WALLET_LEN: usize = 12;

/// `0x03E0` - the client asking for its balance. **Empty body**, throttled 60 s.
pub const CLIENT_CASH_SHOP_QUERY: u16 = 0x03E0;

/// Build a `0x05AD`.
///
/// The third `u32` is read, range-checked and discarded, so it goes out as zero - the same
/// rule this project applies to every field whose meaning is not established.
pub fn cash_shop_wallet(nx: u32, maple_points: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(nx);
    w.u32(maple_points);
    w.u32(0);
    w.into_vec()
}

#[cfg(test)]
mod reply_tests {
    use super::*;
    use crate::opcode::Character;

    fn body() -> Vec<u8> {
        let chr = Character { name: "GoodTest".to_string(), ..Default::default() };
        set_cash_shop(&chr, 0, 0x0123_4567_89ab_cdef, &[], &crate::quest::QuestBook::default(), &[])
    }

    /// **The head is eight bytes and nothing else.**
    ///
    /// `SetField` puts a `u32 channel` and four more fields before its record. `0x01A3` reads
    /// none of them, so borrowing SetField's head would shift the character record by 25
    /// bytes and decode garbage. This pins the one thing that would fail silently.
    #[test]
    fn the_head_is_the_filetime_alone() {
        let b = body();
        assert_eq!(&b[0..8], &0x0123_4567_89ab_cdefu64.to_le_bytes(), "the clock, then the record");
        assert_ne!(
            &b[0..crate::opcode::SET_FIELD_HEAD_LEN],
            &crate::opcode::set_field_head(0x0123_4567_89ab_cdef, 0, 0)[..],
            "this must NOT be SetField's head"
        );
    }

    /// The wallet is three `u32`s and the balances land where the client reads them.
    #[test]
    fn the_wallet_is_three_u32s() {
        let b = cash_shop_wallet(12_345, 678);
        assert_eq!(b.len(), CASH_SHOP_WALLET_LEN);
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 12_345, "nxCredit");
        assert_eq!(u32::from_le_bytes(b[4..8].try_into().unwrap()), 678, "maplePoint");
        assert_eq!(u32::from_le_bytes(b[8..12].try_into().unwrap()), 0, "read and discarded");
    }

    /// The three list counts are zero and the margin is zero, so every loop is skipped.
    #[test]
    fn the_three_list_counts_are_zero_and_the_margin_is_zero() {
        let b = body();
        let tail_at = b.len() - CASH_SHOP_MARGIN - CASH_SHOP_TAIL_LEN;
        assert_eq!(&b[tail_at..tail_at + CASH_SHOP_TAIL_LEN], &[0u8; CASH_SHOP_TAIL_LEN]);
        assert!(b[tail_at + CASH_SHOP_TAIL_LEN..].iter().all(|x| *x == 0), "the margin is zero");
        assert_eq!(b.len() - tail_at, CASH_SHOP_TAIL_LEN + CASH_SHOP_MARGIN);
    }

    /// **The record is the SAME one SetField sends**, byte for byte - not a cash-shop variant.
    ///
    /// If these ever diverge, one of the two packets is decoding a record the client did not
    /// build for it, and the failure would be a garbled character rather than an error.
    #[test]
    fn the_record_is_the_one_set_field_already_sends() {
        let chr = Character { name: "GoodTest".to_string(), ..Default::default() };
        let record = character_record_for_set_field_with_quests_and_skills(
            &chr, 0, &[], &crate::quest::QuestBook::default(), &[],
        );
        let b = body();
        assert_eq!(&b[8..8 + record.len()], &record[..]);
        assert_eq!(b.len(), 8 + record.len() + CASH_SHOP_TAIL_LEN + CASH_SHOP_MARGIN);
    }
}
