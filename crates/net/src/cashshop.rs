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


// ===========================================================================================
// Inside the shop: 0x03E1 in, 0x05AE out
// ===========================================================================================

/// `0x03E1` - **everything the player does inside the shop except asking for the balance.**
///
/// One `u8` sub-op and then a payload whose shape depends on it. Twenty-two builders carry
/// this opcode; six of them live in the reachable class (`0x140D7xxxx`) and are the only ones
/// on the live path. `research/cash-shop-stage.md` section 5.2. **[L]** for the sub-op bytes,
/// which were re-read off the listing by hand.
///
/// # Leaving this unanswered wedges the shop
///
/// Every one of the six builders sets the in-flight latch `[stage+0x74] = 1` before it sends,
/// and the shop's UI blocks until something clears it. So this is one of the packets
/// `CLAUDE.md`'s "always answer" is actually about: the first Buy click of a session would
/// otherwise kill every later click, exactly the way `0x00D5` did before it was answered.
pub const CLIENT_CASH_SHOP_ACTION: u16 = 0x03E1;

/// `0x03E1` sub-op **buy**, short form. The builder is `FUN_140D785F0`.
///
/// The sub-op is a `cmov` - `mov edx,2 / mov eax,0x1f / cmovb edx,eax` - selected by whether a
/// computed id falls inside a 10 000-wide window at `0x08ADDAE0`, and the two forms differ in
/// length because two `u8 0` writes hang off the same test. **Both are a buy.** [L]
pub const ACTION_BUY: u8 = 0x02;

/// `0x03E1` sub-op **buy**, the other arm of the same `cmov`. See [`ACTION_BUY`].
pub const ACTION_BUY_ALT: u8 = 0x1F;

/// `0x03E1` sub-op **gift**: `u32, str, str` - id, recipient, message. `FUN_140D7B240`. [I]
pub const ACTION_GIFT: u8 = 0x03;

/// The three sub-ops that carry a cash item's `u64` serial and operate on it - moving one
/// between the locker and the Cash tab is the obvious candidate, and none of the three has
/// been seen. `0x0A` and `0x0B` take arguments, `0x1C` takes none. [I] on the meaning,
/// **[L]** on the numbers and the payload shapes.
pub const ACTION_ON_SERIAL: [u8; 3] = [0x0A, 0x0B, 0x1C];

/// `0x03E1` sub-op taking one bare `u32`. `FUN_140D7ADC0`. [I]
pub const ACTION_ONE_ID: u8 = 0x2B;

/// `0x05AE` - the multiplexed result. One `u8` sub-op, then a payload chosen by a byte index
/// table at `0x140D7E194` into a 22-entry jump table at `0x140D7E13C`.
pub const CASH_SHOP_RESULT: u16 = 0x05AE;

/// **The refusal that clears everything and keeps the player in the shop.**
///
/// This is not what `research/cash-shop-stage.md` section 11.4 recommended, and the
/// difference is load-bearing. Both tables were dumped from the image
/// (`tools/dump_va.py 0x140D7E194` and `0x140D7E13C`), so the case addresses below are read,
/// not inferred: **[L]**
///
/// ```text
///   sub-op 0x1A -> 0x140D7DE81      sub-op 0x1C -> 0x140D7DE81 (the same body)
///   sub-op 0x1E -> 0x140D7DE93
/// ```
///
/// and `0x140D7DE81` falls straight through into `0x140D7DE93`, so `0x1A` does everything
/// `0x1E` does **plus one call first**:
///
/// ```asm
/// 140d7de81  mov  ecx, 2 ; call 0x142aa2810      ; a UI call
/// 140d7de8b  call 0x140d74c70                    ; CANCEL THE PENDING PURCHASE
/// 140d7de93  mov  byte [rdi+0x74], 0             ; clear the in-flight latch <- 0x1E starts here
/// 140d7de97  READ u8 nReason
/// 140d7dea5  call 0x140d7c7f0                    ; show the message
/// ```
///
/// `FUN_140D74C70` empties a vector at `[stage+0x128]` and writes **`[stage+0x120] = 0`**
/// (`0x140D74DDB`), and that field is why `0x1E` is the wrong choice. The buy builder sets
/// `[stage+0x120] = 1` at `0x140D7A500`, and the **`0x05AD` arm resumes the purchase when it
/// sees a 1**:
///
/// ```asm
/// 140d736e0  mov  eax, [rbx+0x120]
/// 140d736e6  cmp  eax, 1 ; jne ...
/// 140d736eb  mov  [rbx+0x120], esi   ; = 0
/// 140d736f6  call 0x140d785f0        ; THE BUY BUILDER - the client re-sends the purchase
/// ```
///
/// So refusing with `0x1E` leaves the pending-purchase mode armed, and the **next wallet
/// reply** - the client polls `0x03E0` once a minute unprompted - makes the client buy again
/// on its own. `0x1A` clears it first and the same poll is then inert. **[L]**
///
/// The same chain rules out the tempting shortcut of answering a purchase with `0x05AD`
/// alone: it clears the latch, but it is precisely the packet that re-triggers the buy.
pub const RESULT_CANCEL_AND_STAY: u8 = 0x1A;

/// `0x05AE` sub-op that shows the message and **leaves the pending purchase armed**. See
/// [`RESULT_CANCEL_AND_STAY`] for why that matters; this is here to be named, not used.
pub const RESULT_MESSAGE_ONLY: u8 = 0x1E;

/// `0x05AE` sub-op that shows the message and then **sends `0x00D1` and drops the stage** -
/// the player is thrown out of the shop. `0x140D7DE31`, which ends `call 0x140d73bf0`. [L]
/// Named so it is recognisable, and deliberately not used.
pub const RESULT_MESSAGE_AND_EJECT: u8 = 0x05;

/// Reason bytes for [`cash_shop_refusal`]. `FUN_140D7C7F0` indexes `reason - 1` into a
/// 127-entry jump table at `0x140D7D95C` and shows the message it names; the mapping is not
/// linear and was decrypted with `tools/dump_stringids.py`. **[L]**
///
/// **Reason 0 is not silent.** `0x140D7C811 dec edx` then `cmp edx, 0x7e / ja` sends anything
/// outside `1..=0x7F` to `0x140D7D3A3`, which loads string 661 - the generic error. There is
/// no reason byte that shows nothing, which is why this server cannot yet report a
/// *successful* purchase: every `0x05AE` arm that clears the latch also puts a message on
/// screen, and taking a player's money behind an error message is worse than refusing.
pub mod reason {
    /// 600 - "Request timed out. Please try again."
    pub const TIMED_OUT: u8 = 0x01;
    /// 661 - "Due to an unknown error, the Cash Shop request has failed." The generic no,
    /// and what the client's own abort path uses.
    pub const UNKNOWN_ERROR: u8 = 0x02;
    /// 601 - "You don't have enough cash."
    pub const NOT_ENOUGH_CASH: u8 = 0x03;
    /// 614 - "You have too many Cash Items. Please clear Cash slot and try again."
    pub const TOO_MANY_CASH_ITEMS: u8 = 0x0A;
    /// 1194 - "Please check if your inventory is full or not."
    pub const CHECK_INVENTORY: u8 = 0x1A;
    /// 1832 - "You cannot buy this item because it is sold out."
    pub const SOLD_OUT: u8 = 0x20;
    /// 616 - "You have reached the daily maximum purchase limit for the Cash Shop."
    pub const DAILY_LIMIT: u8 = 0x2B;
}

/// Body length of a [`cash_shop_refusal`]: the sub-op and the reason.
pub const CASH_SHOP_REFUSAL_LEN: usize = 2;

/// Build the refusal: cancel the pending purchase, clear the in-flight latch, show `reason`,
/// and leave the player standing in the shop. See [`RESULT_CANCEL_AND_STAY`].
pub fn cash_shop_refusal(reason: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(RESULT_CANCEL_AND_STAY);
    w.u8(reason);
    w.into_vec()
}

/// A decoded [`CLIENT_CASH_SHOP_ACTION`]: the sub-op, and everything after it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CashShopAction<'a> {
    pub sub_op: u8,
    pub rest: &'a [u8],
}

impl CashShopAction<'_> {
    /// Is this one of the two buy arms?
    pub fn is_buy(&self) -> bool {
        self.sub_op == ACTION_BUY || self.sub_op == ACTION_BUY_ALT
    }
}

/// Parse a `0x03E1` body (opcode already stripped). `None` only if it is empty.
pub fn parse_cash_shop_action(body: &[u8]) -> Option<CashShopAction<'_>> {
    let (sub_op, rest) = body.split_first()?;
    Some(CashShopAction { sub_op: *sub_op, rest })
}

/// **Every `u32` that could be hiding in a payload, at every byte offset.**
///
/// This exists because nothing in this project has ever seen a real `0x03E1`. The buy
/// builder's payload was read off the listing as `u8, u32, [u8, u8], u32, u32` - two forms of
/// different length - and **which of those `u32`s is the commodity SN is not established**.
///
/// Rather than pick one and be quietly wrong, the caller walks every offset and asks the
/// commodity table which candidates are real serials. The SNs this client ships run
/// `92000000..160300005` out of a 4-billion-wide space, so a spurious match is not a
/// realistic worry, and a payload that produces **two** matches is a finding the caller can
/// report rather than a coin to flip. `CLAUDE.md`'s "enumerate before you filter".
pub fn u32_candidates(rest: &[u8]) -> Vec<(usize, u32)> {
    (0..rest.len().saturating_sub(3))
        .map(|i| (i, u32::from_le_bytes([rest[i], rest[i + 1], rest[i + 2], rest[i + 3]])))
        .collect()
}

#[cfg(test)]
mod action_tests {
    use super::*;

    /// The refusal is two bytes and both of them are the ones the listing needs.
    #[test]
    fn the_refusal_is_the_sub_op_that_cancels_the_pending_purchase() {
        let b = cash_shop_refusal(reason::NOT_ENOUGH_CASH);
        assert_eq!(b.len(), CASH_SHOP_REFUSAL_LEN);
        assert_eq!(b[0], 0x1A, "0x140D7DE81 - cancels [stage+0x120] before clearing the latch");
        assert_eq!(b[1], 0x03, "601, you don't have enough cash");

        // The two that must NOT be used by accident, pinned so a swap is a test failure and
        // not a run spent wondering why the player got ejected or bought twice.
        assert_ne!(RESULT_CANCEL_AND_STAY, RESULT_MESSAGE_AND_EJECT);
        assert_ne!(RESULT_CANCEL_AND_STAY, RESULT_MESSAGE_ONLY);
    }

    /// The sub-op splits off and the rest is handed on whole.
    #[test]
    fn an_action_is_a_sub_op_and_a_tail() {
        let a = parse_cash_shop_action(&[0x02, 1, 2, 3]).expect("one byte is enough");
        assert!(a.is_buy());
        assert_eq!(a.rest, &[1, 2, 3]);

        assert!(parse_cash_shop_action(&[ACTION_BUY_ALT]).expect("no tail").is_buy());
        assert!(!parse_cash_shop_action(&[ACTION_GIFT]).expect("gift").is_buy());
        assert!(parse_cash_shop_action(&[]).is_none(), "an empty body names no sub-op");
    }

    /// **The serial walk finds a value at its true offset whichever form the buy takes.**
    ///
    /// Both candidate layouts are built here around the same SN and the walk has to find it
    /// in each, because the whole point is that we do not know which one the client sends.
    #[test]
    fn the_serial_walk_finds_the_sn_at_whatever_offset_it_sits() {
        let sn: u32 = 130000000; // Regular Store Permit, gm-handbook/commodity.txt

        // u8, u32 sn, u32, u32
        let mut short = vec![0u8];
        short.extend_from_slice(&sn.to_le_bytes());
        short.extend_from_slice(&[0u8; 8]);
        assert!(u32_candidates(&short).contains(&(1, sn)), "found at offset 1");

        // u8, u32 sn, u8, u8, u32, u32 - two bytes longer
        let mut long = vec![0u8];
        long.extend_from_slice(&sn.to_le_bytes());
        long.extend_from_slice(&[0u8; 10]);
        assert!(u32_candidates(&long).contains(&(1, sn)));

        // A payload too short to hold a u32 yields nothing rather than reading past the end.
        assert!(u32_candidates(&[1, 2, 3]).is_empty());
        assert_eq!(u32_candidates(&[1, 2, 3, 4]).len(), 1);
    }
}
