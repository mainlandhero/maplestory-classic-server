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
/// **CONFIRMED ON A CLIENT, 2026-08-25.** This block used to end *"the opcode NUMBER is [D],
/// not [L] - no SetCashShop has ever been on this wire"*. It has been now: the hook logged one
/// entry to `0x14209AD60` dispatching this exact opcode, the window drew, and the stage's own
/// `OnPacket` then accepted two wallets. The number is **read**, and the three discriminators
/// above are corroboration rather than the whole case.
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
/// **Confirmed on a client 2026-08-26.** Three buy clicks, three `0x1A` refusals: the player
/// was not ejected, the shop stayed usable, and Exit still worked. First `0x05AE` ever on
/// this wire.
///
/// # `[stage+0x120]` is a KIND, not a boolean, and that limits this sub-op's reach
///
/// It holds the **in-flight request kind** - `1` buy, `2` gift, `4`/`5`/`6` for the three
/// queued operations - and `FUN_140D74C70` clears it by emptying the vector at
/// `[stage+0x128]`, which **is the queue**. So `0x1A` is the right refusal for a buy and for
/// a gift, and the wrong one for anything queued, because it silently discards whatever else
/// was pending. [`RESULT_QUEUE_REFUSED`] is the one to use there. `research/cash-shop-actions.md`.
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

/// **`0x05AE` sub-op `0x3D` - the refusal for the QUEUED operations**, which is not `0x1A`.
///
/// # The five non-buy sub-ops are one QUEUE, and that changes what a refusal costs
///
/// `research/cash-shop-actions.md`, 2026-08-26. `FUN_140D74A70` pops **32-byte request
/// records** with a `kind` at offset 0 - `4`, `5`, `6` for `0x0A`, `0x0B`, `0x1C` - and
/// **`[stage+0x120]` holds the in-flight KIND, not a boolean.** Each success arm gates on its
/// own kind (`cmp [rsi+0x120], 4` in the `0x19` arm) and then pumps the next request.
///
/// So [`RESULT_CANCEL_AND_STAY`] is right for a buy and **wrong here**: `FUN_140D74C70`
/// empties the vector at `[stage+0x128]`, which is the queue itself. The delete UI enqueues
/// one request per selected item, so refusing the first with `0x1A` would silently discard
/// the rest - the player would see one message and lose four actions with no explanation.
///
/// `0x3D` reads a **`u16`** reason, where `0x1A` reads a `u8`. **[L]** on the width and the
/// sub-op; **[I]** on whether the value indexes the same 127-entry message table
/// `FUN_140D7C7F0` uses, which is why this server sends the generic one and nothing clever.
pub const RESULT_QUEUE_REFUSED: u8 = 0x3D;

/// Body length of a [`cash_shop_queue_refusal`]: a `u8` sub-op and a `u16` reason.
pub const CASH_SHOP_QUEUE_REFUSAL_LEN: usize = 3;

/// Build the refusal for a queued operation. See [`RESULT_QUEUE_REFUSED`].
pub fn cash_shop_queue_refusal(reason: u16) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(RESULT_QUEUE_REFUSED);
    w.u16(reason);
    w.into_vec()
}

/// **The one `0x03E1` sub-op that does not latch, so it does not have to be answered.**
///
/// Every other reachable builder writes `[stage+0x74] = 1` before it sends, and the UI blocks
/// until something clears it. `0x2B` does not: `research/cash-shop-actions.md` grepped for the
/// same write pattern inside its builder and found nothing, **with a positive control** - the
/// identical search found both sites in `0x0A`, in the same call.
///
/// That matters because the cheapest answer available, `0x1A`, would **discard the whole
/// queue** (see [`RESULT_QUEUE_REFUSED`]). Answering a packet that did not latch, with a
/// packet that destroys unrelated pending work, is worse than silence - so this one is the
/// documented exception to "always answer", and the exception is narrow, measured, and logged
/// every time it is taken.
pub const ACTION_NO_LATCH: u8 = 0x2B;

/// Which family a `0x03E1` sub-op belongs to, and therefore how to refuse it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionFamily {
    /// `0x02` / `0x1F`. Refuse with [`RESULT_CANCEL_AND_STAY`] - measured to work on a client.
    Buy,
    /// `0x0A`, `0x0B`, `0x1C` - the 32-byte-record queue. Refuse with [`RESULT_QUEUE_REFUSED`].
    Queued,
    /// `0x2B`. Does not latch; see [`ACTION_NO_LATCH`].
    NoLatch,
    /// `0x03` gift, and anything unrecognised. `0x1A` is the only arm that clears
    /// `[stage+0x120]`, and a gift sets it to **2** - a value **no site in the reachable class
    /// compares against**, so an unanswered gift would block every later buy, move and delete
    /// while the UI still looked alive. That makes `0x1A` the right answer here despite
    /// costing the queue.
    Other,
}

/// Classify a sub-op. See [`ActionFamily`].
pub fn action_family(sub_op: u8) -> ActionFamily {
    if sub_op == ACTION_BUY || sub_op == ACTION_BUY_ALT {
        ActionFamily::Buy
    } else if ACTION_ON_SERIAL.contains(&sub_op) {
        ActionFamily::Queued
    } else if sub_op == ACTION_NO_LATCH {
        ActionFamily::NoLatch
    } else {
        ActionFamily::Other
    }
}

/// **`0x05AE` sub-op `0x19` - an item arrives in the BAG. This is the MOVE reply, not the buy.**
///
/// # It was used for a purchase once and that was wrong
///
/// The owner, 2026-08-27: *"it automatically goes into the 'Item Inventory', when it should go into
/// the 'Cash Inventory'."* This is the reply to `0x03E1` sub-op `0x0A`, *move a locker item
/// into inventory slot N*, and it did exactly that. The purchase reply is
/// [`RESULT_ITEM_TO_LOCKER`]. Kept here because the locker-to-bag move still needs it.
///
/// # This contradicts what this file said a day ago, and the correction matters
///
/// `research/cash-shop-stage.md` §6.2.1 - and my own summary of it - claimed **every** arm
/// that clears the in-flight latch also calls `FUN_140D7C7F0` and puts a message on screen,
/// so a *successful* purchase could not be reported. That was an enumeration of the six arms
/// whose bodies are **inline** in `FUN_140D7DCA0`, and it missed the two that **delegate to
/// sub-functions**. `0x19` and `0x1B` are silent. `research/cash-shop-buy-done.md` checked it
/// twice - a listing-extent grep, and `tools/callers.py` over `FUN_140D7C7F0` whole-image in
/// all three modes - and neither delegate appears in either list.
///
/// Same shape as the mistake `CLAUDE.md` keeps recording: searching a **known list** rather
/// than enumerating, and getting a clean confident wrong answer out of it.
///
/// ```text
/// u8   0x19
/// u8   bRelease      MUST BE NON-ZERO
/// u16  nPOS          1-based slot; the client derives the TAB from the item id itself
/// <GW_ItemSlot>      the ordinary item body, leading type byte included - the same blob
///                    `inventory_added` carries, which has been on a live wire for months
/// u8   bEffect       0
/// ```
///
/// # Both latches, and I read both branches rather than trusting the report
///
/// Two fields gate the shop, not one: eight of the ten request entry points refuse at their
/// first instruction while **either** `[stage+0x74]` or `[stage+0x120]` is non-zero.
///
/// ```asm
/// 140d7f8f1  READ u8                    ; bRelease
/// 140d7f8f6  test al, al
/// 140d7f8f8  je   140d7f8fe             ; ZERO SKIPS THE CLEAR - hence "must be non-zero"
/// 140d7f8fa  mov  byte [rsi+0x74], r12b ; r12b = 0
/// ...
/// 140d80348  cmp  [rsi+0x120], 4        ; the QUEUED kind. A buy is kind 1, so this fails
/// 140d8034f  jne  140d80358
/// 140d80351  mov  [rsi+0x120], r12d
/// 140d80358  call 0x140d74a70           ; CALLED EITHER WAY - and this is what saves a buy
/// ```
///
/// `FUN_140D74A70` then tests `[this+0x120] != 0` at `0x140D74A90` and jumps to `0x140D74B40`,
/// which calls **`FUN_140D74C70`** - the cancel that writes `[stage+0x120] = 0` at
/// `0x140D74DDB`. So a buy's kind-1 is cleared through the cancel path rather than the
/// `cmp ..., 4` path. **[L]**, read here, because two agents described this arm differently
/// and the difference decides whether a purchase leaves the shop wedged.
pub const RESULT_ITEM_GRANTED: u8 = 0x19;

/// Build a [`RESULT_ITEM_GRANTED`].
///
/// **Send [`cash_shop_wallet`] straight after it, never before.** While `[stage+0x120]` is
/// still 1 the wallet arm calls the buy builder back and the client buys again; `0x19` zeroes
/// it first, which is what makes the wallet inert. Order is the whole safety argument.
pub fn cash_shop_item_granted(slot: u16, item_blob: &[u8]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(RESULT_ITEM_GRANTED);
    w.u8(1); // bRelease - a zero here would leave the shop blocked
    w.u16(slot);
    w.bytes(item_blob);
    w.u8(0); // bEffect
    w.into_vec()
}

/// **`0x05AE` sub-op `0x0C` - the bought item appears in the CASH INVENTORY.**
///
/// # This replaces `0x19`, and `0x19` was my error
///
/// The owner, 2026-08-27: *"Buying the Mystery Hair Coupon is fine, but it automatically goes into
/// the 'Item Inventory', when it should go into the 'Cash Inventory'."* They are right, and the
/// packet was doing exactly what it says: [`RESULT_ITEM_GRANTED`] is the reply to `0x03E1`
/// sub-op `0x0A`, *move a locker item into inventory slot N*. Using it for a purchase asked
/// the client to put the coupon in the bag, and it did.
///
/// `0x0C` is `FUN_140D7E5E0`. It decodes **one** 71-byte cash-item record, inserts it into the
/// locker map at `[stage+0x150]`, repaints the Cash Inventory panel at `[stage+0xc8]`, and
/// drops the commodity from the cart. **[L]**
///
/// ```text
/// u8            0x0C
/// <71 bytes>    the cash-item record - see `cash_item_record`
/// u32           0
/// u8            0
/// ```
///
/// # It clears NEITHER latch, and that is correct
///
/// The previous pass went looking for an arm that cleared both, and the premise was wrong. A
/// purchase does not need one: the **wallet** does it. `0x05AD`'s handler writes
/// `[stage+0x74] = 0` **unconditionally** at `0x140D736DC`, then reads `[stage+0x120]`, sees
/// the buy's `1`, zeroes it and calls the buy builder back - and that re-entry lands on the
/// builder's *completion* path, which fetches **string 590, "You have successfully made the
/// purchase."** at `0x140D78DE1`. Verified here: `0x140D78DD8 mov edx, 0x24e` is 590.
///
/// So send `0x0C` **then** [`CASH_SHOP_WALLET`], and the client supplies its own success
/// message. That also explains the symptom the owner reported alongside the wrong panel - *"it also
/// did not have a success message and sound effect"*. With `0x19` we cleared `[stage+0x120]`
/// ourselves, so the wallet never re-entered the builder and the completion path never ran.
/// **The missing message was a consequence of the wrong packet, not a second bug.**
///
/// # The risk, named
///
/// If that re-entry *sends* rather than completes, the client buys again and the loop only
/// stops when the wallet or the bag runs out. It is bounded and server-authoritative, but the
/// run must watch for a balance that keeps dropping.
pub const RESULT_ITEM_TO_LOCKER: u8 = 0x0C;

/// The cash-item record `FUN_1402D0950` reads: **71 bytes**, trailing flag zero.
pub const CASH_ITEM_RECORD_LEN: usize = 71;

/// Build one cash-item record.
///
/// # Four of fifteen fields have a reader, and the rest are written anyway
///
/// `research/cash-shop-cash-inventory.md` §6 re-ran the displacement scan in **both** shapes -
/// record-relative as well as object-relative, which is what the previous pass got wrong,
/// because the code reaches the record through a pointer to `obj+0x20`. Result: only wire
/// `+0`, `+16`, `+20` and `+67` are read anywhere in the cash shop. The quantity at `+24` and
/// the expiry at `+39` have **no reader at all** - a stronger negative than the previous pass
/// reached, and it still names three blind spots rather than claiming the field is unused.
///
/// The rest are written as zero. That is deliberate: a value invented for a field nothing
/// reads is a claim, and this project has paid for those.
///
/// * `serial` - the map key. Must be non-zero and must not be `-1`; `FUN_140D75850` drops
///   `-1`, and a duplicate collides in the map. Every later `0x0A`/`0x0B`/`0x1C` names the
///   item by this. **[L]**
/// * `item_id` - drives `FUN_1403E8AF0` on every later move. **[L]**
/// * `commodity_sn` - keeps `0x0C` on the string-590 branch. `0x140D7E70B` gates a *meso*
///   message on the 80-90 M band, and every commodity in this build is 92 M-160 M, so the
///   branch is inert either way - but the SN is the value that is probably right. **[L]** for
///   the branch, **[I]** for the name.
/// * `refundable` - `+67`, read by the delete builder's refundable gate (string 651). Zero.
pub fn cash_item_record(serial: u64, item_id: u32, commodity_sn: u32, quantity: u16) -> Vec<u8> {
    cash_item_record_owned(serial, item_id, commodity_sn, quantity, 0, 0)
}

/// [`cash_item_record`] with the two owner ids filled.
///
/// **Why the ids stopped being zero, 2026-09-10 night.** Two measuring runs showed the
/// client receiving all six locker rows, inserting them, repainting, and placing six row
/// widgets on the panel's grid (x 15..205 step 38, y 62 - the hook log has the calls) -
/// and drawing nothing in them. Everything read says the rows should draw; the two fields
/// no reader was found for are the last thing in the record that could decide "is this
/// mine", and a scan that found no reader has been wrong in this file before (section 6 of
/// the research re-ran one). The account id the client holds is the 0 from ACCOUNT_INFO;
/// the character id is the record's own. Filling them costs nothing and is one of the two
/// variables on the run that follows, separable by panel.
pub fn cash_item_record_owned(
    serial: u64,
    item_id: u32,
    commodity_sn: u32,
    quantity: u16,
    account_id: u32,
    character_id: u32,
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.bytes(&serial.to_le_bytes()); // +0   liSN            [L] read
    w.u32(account_id); // +8   dwAccountID     [I], no reader found; the client's own is 0
    w.u32(character_id); // +12  dwCharacterID   [I], no reader found
    w.u32(item_id); // +16  nItemID         [L] read
    w.u32(commodity_sn); // +20  nCommodityID    [L] read
    w.u16(quantity); // +24  nNumber         [I], NO READER
    w.bytes(&[0u8; 13]); // +26  sBuyCharacterName [I], no reader
    // +39 expiry FILETIME. **Was eight zero bytes and that is the prime suspect.**
    //
    // The owner, 2026-08-28: *"I got the dialogue that the purchase was successful, but I do not
    // see it in my Cash Inventory."* The record reached the stage - the hook logged the
    // dispatch with `rdx=0x5ae` - and the success message appeared, but the message comes
    // from the WALLET re-entering the buy builder, so it never proved `0x0C` did anything.
    //
    // `research/cash-shop-cash-inventory.md` established this field has **no reader anywhere
    // in the cash shop**, and that is still true - but "no reader in the cash shop" is not
    // "no reader", and a locker PANEL that hides expired items would live in the UI code that
    // scan did not cover. Eight zeros is 1601-01-01, which is expired by four centuries.
    //
    // So it now carries the same never-expires sentinel `crate::bag::bundle_item` puts in an
    // ordinary item, which HAS been on a wire for months. **[I]** that this is the cause; the
    // run says so, and if the item still does not appear the field is exonerated rather than
    // suspected.
    w.bytes(&crate::opcode::ITEM_NEVER_EXPIRES.to_le_bytes());
    w.u32(0); // +47
    w.bytes(&0f64.to_le_bytes()); // +51  f64
    w.u32(0); // +59
    w.u32(0); // +63
    w.u8(0); // +67  refundable      [D] read
    w.u8(0); // +68
    w.u8(0); // +69
    w.u8(0); // +70  trailing flag - 0 ENDS the record
    w.into_vec()
}

/// Build a [`RESULT_ITEM_TO_LOCKER`]. Send [`cash_shop_wallet`] straight after it.
pub fn cash_shop_item_to_locker(record: &[u8]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(RESULT_ITEM_TO_LOCKER);
    w.bytes(record);
    w.u32(0);
    w.u8(0);
    w.into_vec()
}

/// **`0x05AE` sub-op `0x1B`, and it is a trap in BOTH of its forms.** Named so nobody uses it.
///
/// Its first `u8` is a form selector, which neither earlier file had: **zero** carries the full
/// record, non-zero carries SN, item id and a `u32` slot whose *sign* picks the verify path.
/// Both forms end at `0x140D81125` calling `FUN_1401ABD80(tabArray + slot * 16)` with **no null
/// check** - the out-of-range test above it only logs. **Never use it for a purchase**:
/// `FUN_140230CB0` returns 0 and the tail then resets index 0 of a possibly-null array.
pub const RESULT_MOVED_TO_LOCKER: u8 = 0x1B;

/// `0x05AE` sub-op **`0x04` - `Res_LoadLocker_Done`: the whole Cash Inventory, at once.**
///
/// `FUN_140D7E1F0` [L], `research/cash-shop-cash-inventory.md` section 3.4:
///
/// ```text
/// u8   0x04
/// u8   bShowMessage
/// [u32 nOverLimitCount]      only if bShowMessage != 0
/// u16  count
/// count x the 71-byte record   the locker map is CLEARED first
/// u16 u16 u16 u16             read and discarded
/// -> repaint; if bShowMessage: string 4186, the "items over the limit" WARNING
/// ```
///
/// **This is the packet for shop entry, and `0x0C` is not.** The owner, 2026-09-10, with a
/// screenshot of the shop opening onto a dialog: *"The moment I enter cash shop, I receive
/// this dialogue. This should not happen."* The dialog was *"You have successfully made the
/// purchase."* - `0x0C`'s own success message, which the entry listing of that morning sent
/// once per stored row. `0x0C` is the reply to a BUY and says so on screen; `0x04` inserts
/// the same records and says nothing when its flag is 0.
pub const RESULT_LOAD_LOCKER: u8 = 0x04;

/// Build a [`RESULT_LOAD_LOCKER`]: every record the locker holds, message flag 0.
///
/// Replaces the client's locker map wholesale, so it must carry ALL of the account's rows -
/// a partial list would make the rest vanish from the panel while their rows sit in the
/// database, which is the bug the entry listing was written to fix.
pub fn cash_shop_load_locker(records: &[Vec<u8>]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(RESULT_LOAD_LOCKER);
    w.u8(0); // bShowMessage: 0, so no nOverLimitCount and no dialog
    w.u16(records.len() as u16);
    for r in records {
        w.bytes(r);
    }
    for _ in 0..4 {
        w.u16(0); // read and discarded
    }
    w.into_vec()
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

/// Offset of the commodity SN inside a buy payload, **measured**.
///
/// # The capture, 2026-08-26
///
/// Three clicks on three different items, and every byte agrees with the shape
/// `research/cash-shop-stage.md` section 5.2 read off the builder - `u8, u32, [u8, u8], u32,
/// u32`, the LONG arm of the `cmov` pair. Sixteen bytes on the wire, one of them the sub-op:
///
/// ```text
/// 02 | 01 | 02 00 00 00 | 00 00 | 00 68 89 09 | 00 00 00 00     SN 160000000 Brown Puppy
/// 02 | 01 | 02 00 00 00 | 00 00 | a0 ee 8a 09 | 00 00 00 00     SN 160100000 Red Hat
/// 02 | 01 | 02 00 00 00 | 00 00 | e1 fb 8d 09 | 00 00 00 00     SN 160300001 Water of Life
/// ^    ^    ^              ^       ^             ^
/// |    |    |              |       |             u32, zero on all three
/// |    |    |              |       THE COMMODITY SN, little-endian
/// |    |    |              two u8s, zero on all three
/// |    |    u32, 2 on all three
/// |    u8, 1 on all three
/// sub-op 0x02
/// ```
///
/// **[L]** for the offset - three different serials landed at it and each resolved to the
/// item the owner said they had clicked. The four fields that did not vary are **[I]**: three
/// captures cannot tell a constant from a field that happens to be zero, and this file should
/// not pretend otherwise.
pub const BUY_SERIAL_OFFSET: usize = 7;

/// Length of the buy payload, after the sub-op byte. `1 + 4 + 1 + 1 + 4 + 4`.
pub const BUY_PAYLOAD_LEN: usize = 15;

/// Read the commodity SN out of a buy payload at its measured offset.
///
/// `None` if the payload is too short - which is the short arm of the `cmov`, two bytes
/// briefer, and has never been seen. The caller falls back to [`u32_candidates`] there rather
/// than guessing, so an unseen form identifies itself instead of decoding as garbage.
pub fn parse_buy_serial(rest: &[u8]) -> Option<u32> {
    let at = BUY_SERIAL_OFFSET;
    Some(u32::from_le_bytes(rest.get(at..at + 4)?.try_into().ok()?))
}

/// **Every `u32` that could be hiding in a payload, at every byte offset.**
///
/// This was the instrument that *found* [`BUY_SERIAL_OFFSET`], and it is kept as the fallback
/// and the cross-check rather than deleted. The buy builder has two forms of different
/// length, and only the long one has been captured, so a short-form payload would put the
/// serial somewhere else and the walk is what would notice.
///
/// It cannot invent an answer: it only returns values that are really in the client's own
/// `Commodity.img`. The SNs occupy a few hundred of four billion, so a spurious match is not
/// a realistic worry, and two matches are reported rather than resolved.
/// `CLAUDE.md`'s "enumerate before you filter".
pub fn u32_candidates(rest: &[u8]) -> Vec<(usize, u32)> {
    (0..rest.len().saturating_sub(3))
        .map(|i| (i, u32::from_le_bytes([rest[i], rest[i + 1], rest[i + 2], rest[i + 3]])))
        .collect()
}

/// **`0x03E1` sub-op `0x0A` - move a locker item into the character's bag.**
///
/// The owner, 2026-09-10: *"The player can choose to move the coupon out of the Cash Inventory
/// into the regular inventory in the Cash Tab."* Until this was built the server refused every
/// one of these with `0x3D`; no archived run contains a single attempt, so it had never been
/// exercised. `research/cash-shop-actions.md` §3 has the builder, `FUN_140D74E10`, read in
/// full:
///
/// ```text
/// u8      0x0A
/// u8[8]   liCashItemSN      the serial this server put in the 0x0C record
/// u32     nItemID
/// u8      nInventoryType    1..5, derived by the CLIENT from the item id
/// u16     nSlotPosition     the destination slot, which the client requires to be EMPTY
/// ```
///
/// The client checks the destination slot is empty and the item is in its locker map before
/// it sends, so a well-formed request is one the client already believes is legal. The server
/// checks all of it again anyway - nothing on this socket is authenticated.
pub const ACTION_MOVE_LOCKER_TO_BAG: u8 = 0x0A;

/// Payload length of an [`ACTION_MOVE_LOCKER_TO_BAG`] after the sub-op byte.
pub const LOCKER_TO_BAG_LEN: usize = 8 + 4 + 1 + 2;

/// A parsed [`ACTION_MOVE_LOCKER_TO_BAG`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockerToBag {
    pub serial: u64,
    pub item_id: u32,
    pub inv_type: u8,
    pub slot: u16,
}

/// Parse the payload after the `0x0A` sub-op. `None` unless it is exactly the measured shape.
pub fn parse_locker_to_bag(rest: &[u8]) -> Option<LockerToBag> {
    if rest.len() != LOCKER_TO_BAG_LEN {
        return None;
    }
    let mut r = PacketReader::new(rest);
    let serial = r.u64().ok()?;
    let item_id = r.u32().ok()?;
    let inv_type = r.u8().ok()?;
    let slot = r.u16().ok()?;
    Some(LockerToBag { serial, item_id, inv_type, slot })
}

#[cfg(test)]
mod action_tests {
    use super::*;

    /// The locker-to-bag request, in the builder's shape, and every truncation refused.
    #[test]
    fn the_locker_to_bag_request_parses_in_the_builders_shape() {
        let mut b = Vec::new();
        b.extend_from_slice(&0x0000_0001_0000_0003u64.to_le_bytes()); // account 1, locker slot 3
        b.extend_from_slice(&5_680_004u32.to_le_bytes()); // Etc Tab 5-slot Coupon
        b.push(5); // the Cash tab
        b.extend_from_slice(&7u16.to_le_bytes()); // into slot 7
        assert_eq!(b.len(), LOCKER_TO_BAG_LEN);
        let r = parse_locker_to_bag(&b).unwrap();
        assert_eq!(r.serial, 0x0000_0001_0000_0003);
        assert_eq!(r.item_id, 5_680_004);
        assert_eq!(r.inv_type, 5);
        assert_eq!(r.slot, 7);
        for n in 0..b.len() {
            assert_eq!(parse_locker_to_bag(&b[..n]), None, "{n} bytes parsed");
        }
        let mut long = b.clone();
        long.push(0);
        assert_eq!(parse_locker_to_bag(&long), None, "the 0x0B shape is one byte wider and is NOT this");
        assert!(ACTION_ON_SERIAL.contains(&ACTION_MOVE_LOCKER_TO_BAG), "it is one of the queue");
    }

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

    /// **Each family gets the refusal that does not destroy something else.**
    ///
    /// The buy keeps `0x1A`, which is the one measured on a client. The queued operations get
    /// `0x3D`, because `0x1A` empties the queue vector and the delete UI enqueues one request
    /// per selected item. `0x2B` gets nothing at all - it does not latch.
    #[test]
    fn the_refusal_matches_the_family() {
        assert_eq!(action_family(ACTION_BUY), ActionFamily::Buy);
        assert_eq!(action_family(ACTION_BUY_ALT), ActionFamily::Buy);
        for sub in ACTION_ON_SERIAL {
            assert_eq!(action_family(sub), ActionFamily::Queued, "sub-op 0x{sub:02X}");
        }
        assert_eq!(action_family(ACTION_NO_LATCH), ActionFamily::NoLatch);
        assert_eq!(action_family(ACTION_GIFT), ActionFamily::Other);
        assert_eq!(action_family(0x77), ActionFamily::Other, "and so does anything unknown");

        // The two refusals are different packets, and the widths are not interchangeable:
        // 0x1A reads a u8 reason and 0x3D reads a u16.
        let q = cash_shop_queue_refusal(2);
        assert_eq!(q.len(), CASH_SHOP_QUEUE_REFUSAL_LEN);
        assert_eq!(q[0], 0x3D);
        assert_eq!(u16::from_le_bytes([q[1], q[2]]), 2);
        assert_ne!(cash_shop_refusal(2).len(), q.len(), "u8 reason vs u16 reason");
    }

    /// **The grant's fixed head, and the byte that would silently break it.**
    ///
    /// `bRelease` is asserted non-zero because a zero there skips `mov [rsi+0x74], r12b`
    /// entirely - the item would arrive and the shop would stay blocked, which is the kind of
    /// half-success that reads on screen as "the cash shop is broken" and in the log as fine.
    #[test]
    fn the_grant_releases_the_latch_and_carries_the_blob() {
        let blob = [2u8, 0xaa, 0xbb]; // a bundle body, type byte first
        let b = cash_shop_item_granted(7, &blob);
        assert_eq!(b[0], RESULT_ITEM_GRANTED);
        assert_ne!(b[1], 0, "bRelease: a zero SKIPS the latch clear at 140d7f8f8");
        assert_eq!(u16::from_le_bytes([b[2], b[3]]), 7, "nPOS, 1-based");
        assert_eq!(&b[4..4 + blob.len()], &blob[..], "the ordinary item body, verbatim");
        assert_eq!(*b.last().unwrap(), 0, "bEffect");
        assert_eq!(b.len(), 5 + blob.len());

        // The trap arm must never be what we build.
        assert_ne!(RESULT_ITEM_GRANTED, RESULT_MOVED_TO_LOCKER);
    }

    /// **The record is 71 bytes and the trailing flag is what ends it.**
    ///
    /// A non-zero flag there would tell the client a whole `GW_ItemSlot` follows, and it would
    /// read one out of whatever came next.
    #[test]
    fn the_cash_item_record_is_seventy_one_bytes_and_terminates() {
        let r = cash_item_record(0x1234_5678_9abc_def0, 5150000, 150000000, 1);
        assert_eq!(r.len(), CASH_ITEM_RECORD_LEN);
        assert_eq!(u64::from_le_bytes(r[0..8].try_into().unwrap()), 0x1234_5678_9abc_def0);
        assert_eq!(u32::from_le_bytes(r[16..20].try_into().unwrap()), 5150000, "nItemID at +16");
        assert_eq!(u32::from_le_bytes(r[20..24].try_into().unwrap()), 150000000, "SN at +20");
        assert_eq!(u16::from_le_bytes(r[24..26].try_into().unwrap()), 1, "quantity at +24");
        assert_eq!(r[70], 0, "the trailing flag MUST be zero or a GW_ItemSlot is read next");

        // The two serials the client rejects outright.
        assert_ne!(u64::from_le_bytes(r[0..8].try_into().unwrap()), 0);
        assert_ne!(u64::from_le_bytes(r[0..8].try_into().unwrap()), u64::MAX, "-1 is dropped");
    }

    /// The locker packet wraps the record and nothing else moves.
    #[test]
    fn the_locker_packet_is_the_sub_op_then_the_record_then_five_zeros() {
        let r = cash_item_record(7, 5150000, 150000000, 1);
        let b = cash_shop_item_to_locker(&r);
        assert_eq!(b[0], RESULT_ITEM_TO_LOCKER);
        assert_eq!(&b[1..1 + CASH_ITEM_RECORD_LEN], &r[..], "the record, verbatim");
        assert_eq!(b.len(), 1 + CASH_ITEM_RECORD_LEN + 4 + 1);
        assert!(b[1 + CASH_ITEM_RECORD_LEN..].iter().all(|x| *x == 0));

        // 0x19 puts an item in the BAG and 0x0C puts it in the LOCKER. Confusing them is the
        // bug the owner reported, so they are pinned apart.
        assert_ne!(RESULT_ITEM_TO_LOCKER, RESULT_ITEM_GRANTED);
        assert_ne!(RESULT_ITEM_TO_LOCKER, RESULT_MOVED_TO_LOCKER, "1B is a trap in both forms");
    }

    /// **The locker reload carries every record, flag 0, and the four trailing u16s** - the
    /// exact shape FUN_140D7E1F0 reads. Flag 0 is the whole point: with it set the client
    /// reads an extra u32 and shows the over-limit warning; with 0x0C instead it thanks the
    /// player for a purchase they did not make.
    #[test]
    fn the_locker_reload_is_flag_zero_count_records_and_four_trailing_words() {
        let a = cash_item_record(0x1_0000_0001, 5680004, 0, 1);
        let b2 = cash_item_record(0x1_0000_0002, 5680002, 0, 1);
        let p = cash_shop_load_locker(&[a.clone(), b2.clone()]);
        assert_eq!(p[0], RESULT_LOAD_LOCKER);
        assert_ne!(p[0], RESULT_ITEM_TO_LOCKER, "0x0C is the purchase reply and pops a dialog");
        assert_eq!(p[1], 0, "bShowMessage 0: no nOverLimitCount, no dialog");
        assert_eq!(u16::from_le_bytes([p[2], p[3]]), 2);
        assert_eq!(&p[4..4 + CASH_ITEM_RECORD_LEN], &a[..]);
        assert_eq!(&p[4 + CASH_ITEM_RECORD_LEN..4 + 2 * CASH_ITEM_RECORD_LEN], &b2[..]);
        assert_eq!(p.len(), 4 + 2 * CASH_ITEM_RECORD_LEN + 8, "four trailing u16s");
        // An empty locker is a legal reload: count 0 clears the panel.
        let e = cash_shop_load_locker(&[]);
        assert_eq!(e, vec![RESULT_LOAD_LOCKER, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
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

    /// **The three real buy requests, off the wire.**
    ///
    /// The owner clicked Brown Puppy, Red Hat and Water of Life, in that order, on 2026-08-26 -
    /// the first `0x03E1`s this project has ever seen. Each is asserted against the SN of the
    /// item they said they clicked, which is what makes this a check rather than a recording:
    /// the bytes and the name were established independently.
    #[test]
    fn the_three_real_buy_requests_parse() {
        let captures: [(&[u8], u32, &str); 3] = [
            (&[0x02, 0x01, 0x02, 0, 0, 0, 0, 0, 0x00, 0x68, 0x89, 0x09, 0, 0, 0, 0], 160_000_000, "Brown Puppy"),
            (&[0x02, 0x01, 0x02, 0, 0, 0, 0, 0, 0xa0, 0xee, 0x8a, 0x09, 0, 0, 0, 0], 160_100_000, "Red Hat"),
            (&[0x02, 0x01, 0x02, 0, 0, 0, 0, 0, 0xe1, 0xfb, 0x8d, 0x09, 0, 0, 0, 0], 160_300_001, "Water of Life"),
        ];
        for (body, sn, name) in captures {
            let a = parse_cash_shop_action(body).expect("a sub-op byte");
            assert!(a.is_buy(), "{name}: sub-op 0x{:02X}", a.sub_op);
            assert_eq!(a.rest.len(), BUY_PAYLOAD_LEN, "{name}: the long cmov arm");
            assert_eq!(parse_buy_serial(a.rest), Some(sn), "{name}");
            // And the walk, which found the offset in the first place, still agrees with it.
            assert!(u32_candidates(a.rest).contains(&(BUY_SERIAL_OFFSET, sn)), "{name}");
        }
    }

    /// A payload too short to reach the measured offset yields `None` rather than reading
    /// past the end - the short arm of the `cmov`, which has never been captured.
    #[test]
    fn a_short_buy_payload_is_refused_rather_than_misread() {
        assert!(parse_buy_serial(&[0u8; BUY_PAYLOAD_LEN - 5]).is_none());
        assert!(parse_buy_serial(&[]).is_none());
        assert_eq!(parse_buy_serial(&[0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0]), Some(1));
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
