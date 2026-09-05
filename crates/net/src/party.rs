//! Parties: the wire, both directions.
//!
//! Full working, every enumeration and every named blind spot: **`research/party.md`**.
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ, its string
//! table or a capture; **[D]** derived from two or more [L] facts; **[I]** inferred - a
//! candidate nothing on this machine can confirm.
//!
//! # WIRED 2026-09-04, and the first `0x00A5` is on the wire
//!
//! This said *"THIS MODULE IS NOT WIRED. Nothing calls anything in this file"* and that
//! `0x00A5` *"has never been sent by this server"*, with a 452-log sweep behind it. Both were
//! true when written and neither is now: `crates/world/src/session/party.rs` routes `0x0182`
//! into the state machine, the owner pressed Create, and the party window drew.
//!
//! **The first send exposed a bug this file caused.** `party_created` and `refusal` both
//! built with `PacketWriter::with_opcode`, and `Reply::packet()` prepends the opcode too, so
//! the wire carried `a5 00 | a5 00 0e ...`. The client read `0xA5` as the result code, took
//! the default arm, and printed *"your party request failed"*. It hid because `UNKNOWN_ERROR`
//! is itself a default-arm code, so **every refusal produced exactly the message it intended**
//! - the defect was invisible until a real arm was sent. Bodies here start at the CODE byte.
//!
//! Still static: every layout below except `0x0E`. The success arms for join, leave, invite
//! and leader-change have not been sent.
//!
//! The one exception, and it is the reason this file can be specific at all: **the client
//! has sent exactly one `0x0182`**, and it is archived. See [`parse_request`].
//!
//! # The two opcodes are not two halves of one thing
//!
//! `crates/net/src/userpool.rs` already warns that inbound and outbound live in different
//! namespaces. Parties are the sharpest case of it in the whole client:
//!
//! | direction | opcode | body |
//! |---|---|---|
//! | server -> client | [`PARTY_RESULT`] `0x00A5` | a **flat** `u8 code` then code-dependent fields, read with the ordinary `CInPacket` primitives |
//! | client -> server | [`CLIENT_PARTY_REQUEST`] `0x0182` | a **FlatBuffers table**, written as one `w_raw` blob |
//! | client -> server | [`CLIENT_PARTY_INVITE_ANSWER`] `0x0183` | the same, a different schema |
//!
//! Inbound `0x0182` is a *different packet entirely* - `research/msexe-gamestage-cases.txt`
//! gives its case body as `INLINE FUN_1406e8c20 FUN_1406e8ae0`, a `u32` and a `u8`, nothing
//! to do with parties. **[L]** Do not answer a party request by echoing its opcode.
//!
//! # `0x00A5` is the party result, and the identification does not rest on any reference
//!
//! Three independent [L] legs, none of them a v214 opcode list:
//!
//! 1. `research/msexe-gamestage-cases.txt` - this client's own channel switch `FUN_142cbaa80`
//!    - gives case `0x00a5` as `INLINE FUN_1413bab80`.
//! 2. `FUN_1413bab80` resolves **27** of the 43 string ids in the contiguous party message
//!    block `0x0108..0x0132` as immediates of its own - 31 counting the depth-5 call graph -
//!    including *"You have created a new party."*, *"You have left the party."* and *"'%s'
//!    has joined the party."*. Twelve functions in the whole image touch that block at all
//!    and **no other one resolves more than four**, so it is not close.
//! 3. `python tools/callers.py 0x1413bab80` -> **1 call site, 0 tail jmps, 0 data pointers**,
//!    and the one call site is `0x142cbac6e`, inside `FUN_142cbaa80`.
//!
//! The instrument that found leg 2 was an enumeration, not a search: every `mov edx, imm32`
//! immediately before a `call 0x1408a9e40` in every one of the **120 981** `.pdata`
//! functions - 6 599 resolver call sites, **3 956** distinct string ids of the 6 165 that
//! exist. `research/party.md` §2 has the reproduction and the one filter that nearly lost the
//! answer.
//!
//! # The safe answer, and why it is worth a constant
//!
//! `CLAUDE.md`'s oldest rule is **always answer**: an unanswered packet freezes the client's
//! entire UI. A party request is the first opcode in a long time that this server can be
//! *sure* it cannot fully parse, so the refusal matters more than the success.
//!
//! [`request_failed`] is that answer. It is **two bytes on the wire** and the client's own
//! default arm handles it: `0x1413baf5b cmp eax,0x2c / ja 0x1413bcbb5`, and `0x1413bcbb5`
//! resolves string `0x012A` *"Due to an unknown error, your party request failed."*, shows
//! it, and falls into the ordinary epilogue **without reading another byte**. **[L]**
//!
//! [`SILENT_CODES`] and [`refusal`] widen that: **16 of the 29 non-default arms read nothing
//! at all** and each shows one specific message, so the server can refuse *accurately* with a
//! one-byte body. `tools/reads.py 0x1413bab80 3` - which walks helpers and tail `jmp`s -
//! reports no read site above `0x1413bcb15`, so every arm from `0x1413bc850` upward is
//! read-free **except `0x1E`**, whose `str` at `0x1413bcb15` is the highest read in the whole
//! 8 624-byte function. That is a property of the tail rather than of the arms I happened to
//! look at, and `0x1E` is the exception that has to be named for it to be true.

use crate::packet::PacketWriter;

/// `0x00A5`, server -> client. The **only** party packet in this direction.
///
/// Channel-stage case `0x00a5`, inlined body, one call to `FUN_1413bab80` (8 624 bytes).
/// **[L]** See the module docs for the three legs of the identification.
///
/// Body: `u8 code`, then fields that depend entirely on the code. The switch is
/// `0x1413baf58 add eax,-3 / cmp eax,0x2c / ja default`, a 45-entry table at `0x1413bcc50`
/// covering codes **`0x03..0x2F`**; everything else takes the default arm. **[L]**
pub const PARTY_RESULT: u16 = 0x00A5;

/// `0x0182`, client -> server. Every button in the party window except whisper and talk.
///
/// **This is the row `crates/net/src/names.rs` calls `CLIENT_PARTY_CREATE`, and that name is
/// wrong in the way that matters.** Create is *one of seven actions* multiplexed onto this
/// opcode, and the body is not a field list - see [`parse_request`] and [`action`].
///
/// Five builders, all in the party module, all `mov edx,0x182 / call 0x1406ed520` followed by
/// one call to the FlatBuffers encoder `FUN_1413bd0f0` and one to `SendPacket 0x1415d01c0`:
/// `FUN_1413b9eb0`, `FUN_1413ba250`, `FUN_1413ba710`, `FUN_1413bda50`, `FUN_1413bdb80`.
/// `research/msexe-send-opcodes.txt`, cross-checked against `tools/listing.py`. **[L]**
pub const CLIENT_PARTY_REQUEST: u16 = 0x0182;

/// `0x0183`, client -> server. The invite / join-request **answer**.
///
/// Same encoder family, different schema: the struct is `{u8 op, u8 answer, u64 value}` and
/// the op byte the client writes is **`0x1B`** (`0x1413bb0b4 mov byte [rbp-0x68],0x1b`),
/// which is also the [`PARTY_RESULT`] code that carries the invite-outcome messages. So
/// request codes and result codes share a numbering space. **[L]**
///
/// Two builders inside the result handler itself (`0x1413bb0ae`, `0x1413bb39b`) and four in
/// the dialog module `FUN_14180b750` / `FUN_14180c6e0`. **[L]**
pub const CLIENT_PARTY_INVITE_ANSWER: u16 = 0x0183;

/// The `u8` action at slot 0 of a [`CLIENT_PARTY_REQUEST`] body.
///
/// Every value here is **[L]**, read off the party window's own button dispatcher
/// `FUN_1411c9540`. That function compares the clicked node's name against wide-string
/// literals in `.rdata` and routes to a builder; the builder passes the action byte in `cl`
/// (or writes it as an immediate). The chain, button name to action:
///
/// ```text
/// "create"  0x1411c9600 -> FUN_1413b9bc0 -> FUN_1413bdb80(cl = 0)          action 0
/// "leave"   0x1411c97a3 -> FUN_1413ba250  (immediate 1 at 0x1413ba32c)     action 1
/// "pickup"  0x1411c982b -> FUN_1413b9d90 -> FUN_1413bdb80(cl = 2)          action 2
/// "invite"  0x1411c9651 -> FUN_1411cac30 -> FUN_1413b9eb0 (immediate 3)    action 3
/// "expel"   0x1411c971a -> FUN_1413ba590 -> FUN_1413bda50(cl = 5)          action 5
/// "leader"  0x1411c987c -> FUN_1411cb1d0 -> FUN_1413ba470 -> ...(cl = 6)   action 6
/// ```
///
/// The literals were read out of the image as UTF-16: `0x14327f958` is `create`,
/// `0x143286378` is `invite`, `0x14338dc58` is `expel`, `0x14336f790` is `leave`,
/// `0x14338dc68` is `pickup`, `0x14338dbc8` is `leader`. They are the same node names
/// `UI/UI_000.wz/UserList.img/Party` carries as `button:create` ... `button:leader`. **[L]**
pub mod action {
    /// Create a party. Payload: the party name, which the client defaults to
    /// `String.wz` id `0x0BAC` *"%s's Party"* formatted with the character's name. **[L]**
    pub const CREATE: u8 = 0;
    /// Leave the party. No payload. **[L]**
    pub const LEAVE: u8 = 1;
    /// Change the party's item pick-up rights. Gated locally on being the leader - the
    /// builder resolves `0x0126` *"You are not the master of the party."* **[L]**
    pub const SET_PICKUP_RIGHTS: u8 = 2;
    /// Invite a character to the party. **[L]**
    pub const INVITE: u8 = 3;
    /// Ask to join someone else's party. **[D]** - `FUN_1413ba710` writes the immediate 4
    /// and resolves `0x0123` *"You are already in a %s ... ask to join a new %s?"*, but it
    /// has **zero callers** by all three of `tools/callers.py`'s scans, so which control
    /// reaches it is not established.
    pub const JOIN_REQUEST: u8 = 4;
    /// Expel a member. **[L]** - the `expel` button.
    pub const EXPEL: u8 = 5;
    /// Hand the leadership to another member. **[L]** - the `leader` button.
    pub const CHANGE_LEADER: u8 = 6;
}

/// [`PARTY_RESULT`] codes. **Every constant here is a switch arm this client really has**,
/// read off the 45-entry jump table at `0x1413bcc50`. **[L]**
///
/// The *meanings* are the strings each arm resolves, which is as direct as evidence gets:
/// the arm for [`CREATE_OK`] is the one that shows *"You have created a new party."*.
///
/// What is **not** established is the body of the arms that read fields. Those are marked
/// below, and [`refusal`] will not build them.
pub mod result {
    /// `u32, u32, str, u32, u32, u32`, then the client **answers with `0x0183`** and, on one
    /// branch, opens a dialog. The invite / join-request notification. Body **[L]**, which
    /// of invite and join-request this is versus [`INVITE_NOTIFY_B`] is **[I]**.
    pub const INVITE_NOTIFY_A: u8 = 0x03;
    /// The sibling of [`INVITE_NOTIFY_A`]: identical read shape, identical `0x0183` answer,
    /// a different dialog constructor (`FUN_14180f2d0` rather than `FUN_141808b90`). **[L]**
    pub const INVITE_NOTIFY_B: u8 = 0x06;
    /// `u8`. Unnamed - the arm resolves no string. **[L]**
    pub const UNNAMED_0D: u8 = 0x0D;
    /// **"You have created a new party."** Reads `u32 partyId, u8, u32, u32, u32, i16, i16`,
    /// then **two sub-decoders that are not read** (`FUN_1406f2830`, `FUN_1406f2560`), then a
    /// `u32`. **Do not build this** until the sub-decoders are decoded. **[L]** for the arm,
    /// the first seven fields and the string; the tail is unread.
    pub const CREATE_OK: u8 = 0x0E;
    /// **"Already have joined a party."** No fields. **[L]**
    /// **`0x0E` - the party exists and you lead it.** The success answer to a create.
    ///
    /// Its arm reads a `PARTYBLOCK`-shaped body with no gate on any field;
    /// [`super::party_created`] is the builder and `research/party-result-0x00A5.md` §7 is the
    /// working. **[L]** on every width, and the three `u32` / two `i16` between the id and the
    /// seat are of unestablished meaning.
    pub const CREATED: u8 = 0x0E;

    pub const CREATE_REFUSED_ALREADY_IN_ONE: u8 = 0x0F;
    /// The leave / expel / disband family: `u32, u8, u8, str`, and the two `u8` select
    /// between *"You have been expelled from the party."*, *"You have left the party."*,
    /// *"'%s' have been expelled"*, *"'%s' have left"*, *"You have quit as the leader of the
    /// party. The party has been disbanded."* and *"You have left the party since the party
    /// leader quit."* **[L]** for the arm and the six strings; which `u8` selects which is
    /// **not** established.
    pub const WITHDRAW: u8 = 0x10;
    /// **"You have yet to join a party."** No fields. **[L]**
    pub const NOT_IN_A_PARTY: u8 = 0x11;
    /// **"Leaving the party is restricted while on this map."** No fields. **[L]**
    pub const LEAVE_BLOCKED_HERE: u8 = 0x12;
    /// The join family: `str`, then *"'%s' has joined the party."* or *"You have joined the
    /// party."* **[L]** for the arm and both strings.
    pub const JOIN: u8 = 0x13;
    /// `u8`, then **"You have joined the party."** **[L]**
    pub const JOIN_2: u8 = 0x14;
    /// **"Already have joined a party."** No fields. **[L]**
    pub const JOIN_REFUSED_ALREADY_IN_ONE: u8 = 0x15;
    /// **"The party you're trying to join is already in full capacity."** No fields. **[L]**
    pub const JOIN_REFUSED_FULL: u8 = 0x16;
    /// Handled and completely silent - the arm is inside the epilogue's cleanup. **[L]**
    pub const SILENT_17: u8 = 0x17;
    /// **"You are on a map where the other party cannot complete this action."** **[L]**
    pub const OTHER_PARTY_BLOCKED_HERE: u8 = 0x19;
    /// **"That cannot be used on this map."** No fields. **[L]**
    pub const NOT_USABLE_HERE: u8 = 0x1D;
    /// `str`, then **"Party invites cannot be sent to '%s''s current location."** **[L]**
    pub const INVITE_BLOCKED_THERE: u8 = 0x1E;
    /// **"Kicking someone from the party is restricted while on this map."** **[L]**
    pub const EXPEL_BLOCKED_HERE: u8 = 0x1F;
    /// The invite / join-request **outcome** family, shared by codes `0x1B` and `0x1C`.
    /// Reads `raw` then `str`, and covers eleven messages - *"You have invited '%s' to your
    /// party."*, *"%s has denied the party request."*, *"'%s' is already in a party."* and
    /// the rest. **[L]** for the arm and the strings; the `raw` length is unread.
    pub const INVITE_OUTCOME: u8 = 0x1B;
    /// See [`INVITE_OUTCOME`] - the same arm.
    pub const INVITE_OUTCOME_2: u8 = 0x1C;
    /// `u32, u8`, then **"%s has become the leader of the party."** or *"Due to the party
    /// leader disconnecting from the game, %s has been assigned as the new leader."* The
    /// `u8` presumably picks between them; that is **[I]**. **[L]** for the arm and fields.
    pub const LEADER_CHANGED: u8 = 0x22;
    /// **"This can only be given to a party member within the vicinity."** No fields. **[L]**
    pub const LEADER_NOT_NEARBY: u8 = 0x23;
    /// **"Unable to hand over the leadership post; No party member is currently within the
    /// vicinity of the party leader."** No fields. **[L]**
    pub const LEADER_NOBODY_NEARBY: u8 = 0x24;
    /// **"You may only change with the party member that's on the same channel."** **[L]**
    pub const LEADER_OTHER_CHANNEL: u8 = 0x25;
    /// **"The party leader reached the entry wait time limit, and a new party leader has been
    /// selected."** No fields. **[L]**
    pub const LEADER_TIMEOUT: u8 = 0x27;
    /// Handled, reads nothing, shows nothing. **[L]**
    pub const SILENT_28: u8 = 0x28;
    /// Handled, reads nothing, shows nothing. **[L]**
    pub const SILENT_29: u8 = 0x29;
    /// `u8, u32, raw`. Unnamed - the arm resolves no string. **[L]**
    pub const UNNAMED_2A: u8 = 0x2A;
    /// **"Cannot be done in the current map."** No fields. **[L]**
    pub const NOT_HERE: u8 = 0x2C;
    /// **"The party leader has changed the party status to Public / Private."** Reads through
    /// `FUN_1406f2560` (`str, u8`). **[L]** for the arm and the strings.
    pub const PUBLIC_PRIVATE: u8 = 0x2D;
    /// `u8, u32, u32, u32`. Unnamed. **[L]**
    pub const UNNAMED_2F: u8 = 0x2F;

    /// The code [`super::request_failed`] sends. Any value outside `0x03..=0x2F` and any of
    /// `0x04 0x05 0x07..0x0C 0x18 0x1A 0x20 0x21 0x26 0x2B 0x2E` reaches the same default
    /// arm; `0x04` is chosen because it is inside the table, so it exercises the same jump
    /// the client takes for a code it simply does not implement. **[L]**
    pub const UNKNOWN_ERROR: u8 = 0x04;
}

/// The [`PARTY_RESULT`] codes whose arm reads **nothing at all** after the code byte.
///
/// A one-byte body for any of these is safe by construction, which is what makes it usable
/// as an *always answer* refusal. Derived by intersecting the 45-entry jump table walk with
/// `tools/reads.py 0x1413bab80 3`, whose highest reported read site in the whole 8 624-byte
/// function is `0x1413bcb15` - inside the `0x1E` arm. Every arm above `0x1413bc850` is
/// read-free, and that is a statement about the tail rather than about the arms I looked at.
/// **[L]**
///
/// `0x17`, `0x28` and `0x29` are in the list and show **no message**: safe, and invisible.
pub const SILENT_CODES: &[u8] = &[
    result::CREATE_REFUSED_ALREADY_IN_ONE,
    result::NOT_IN_A_PARTY,
    result::LEAVE_BLOCKED_HERE,
    result::JOIN_REFUSED_ALREADY_IN_ONE,
    result::JOIN_REFUSED_FULL,
    result::SILENT_17,
    result::OTHER_PARTY_BLOCKED_HERE,
    result::NOT_USABLE_HERE,
    result::EXPEL_BLOCKED_HERE,
    result::LEADER_NOT_NEARBY,
    result::LEADER_NOBODY_NEARBY,
    result::LEADER_OTHER_CHANNEL,
    result::LEADER_TIMEOUT,
    result::SILENT_28,
    result::SILENT_29,
    result::NOT_HERE,
    result::UNKNOWN_ERROR,
];

/// Whether `code` is a [`PARTY_RESULT`] code whose body is the code byte and nothing else.
///
/// The default arm makes this true for **every** code outside `0x03..=0x2F` as well, which is
/// why the range test is here rather than a plain lookup in [`SILENT_CODES`].
pub fn is_silent_code(code: u8) -> bool {
    !(0x03..=0x2F).contains(&code) || SILENT_CODES.contains(&code)
}

/// **How many seats a `PARTYBLOCK` always carries.** Six, and it is not count-driven.
///
/// `FUN_1406f2fd0` reads exactly six `MEMBER`s with `mov edi, 6` before it reads anything
/// count-driven, so the loop bound is in the code rather than on the wire. **[L]**
///
/// It is also why [`party_seat_is_in_range`] exists: code `0x2F` warns about an index above
/// five and then writes twenty bytes past the 120-byte array anyway (`0x1413bc488`), so the
/// bound has to be enforced on this side.
pub const PARTY_SEATS: usize = 6;

/// **One seat. `None` is an empty one, and an empty one is FOUR BYTES.**
///
/// The single most important thing about this structure: `0x1406f2848 test eax,eax / je` -
/// **if the leading `u32 charId` is zero the decoder returns having read four bytes** and
/// nothing else. So an empty seat is a zero dword, *not* a zeroed 155-byte record. Writing
/// the long form for an empty seat shifts every field after it and is the same class of
/// failure as the seat index that killed both clients. **[L]**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Member {
    /// Nonzero. A zero here is what [`write_member`] treats as "empty seat".
    pub char_id: u32,
    /// Copied into thirteen bytes by the client, so longer names are its problem, not ours.
    pub name: String,
    /// Job id, level, and the two the decoder reads beside them. Widths are **[L]**; what
    /// three of the four MEAN is not established and they are sent as given.
    /// `member+0x14`. **[D]** and reasonably firm: `FUN_1402b0250` builds a table keyed
    /// `0` -> "Beginner", `0x64` -> "Swordsman", `0x6e` -> "Fighter", and this is what it is
    /// looked up by.
    pub job: u32,
    /// `member+0x18`. **WATCH THIS ONE ON THE NEXT RUN.** `+0x18` is the *second argument*
    /// to that same lookup, so level may belong at `+0x1c` instead - in which case the party
    /// row shows a level of **0** while the job name is right.
    ///
    /// Deliberately not moved yet: the framing fix is going out on its own, because changing
    /// two things in one launch has already produced one unexplained crash here.
    pub level: u32,
    /// `member+0x18` - the job-name lookup's second argument. Meaning unestablished; sent
    /// as zero, and the party row rendered correctly with it zero.
    pub unknown_b: u32,
    pub unknown_d: u32,
}

/// Write one seat, in whichever of its two shapes applies.
///
/// `None`, or a `char_id` of zero, writes the four-byte empty form and returns - see
/// [`Member`] for why that is the whole record rather than a truncation.
pub fn write_member(w: &mut crate::PacketWriter, seat: Option<&Member>) {
    let Some(m) = seat.filter(|m| m.char_id != 0) else {
        w.u32(0);
        return;
    };
    w.u32(m.char_id); //     1406f2843
    w.str(&m.name); //       1406f285a  copied to 13 bytes
    w.u32(m.job); //         1406f2889  +0x14
    // **The level is the THIRD u32, not the second, and the screen said so.**
    //
    // Predicted [D] from `FUN_1402b0250` - `+0x18` is the second argument to the job-name
    // lookup rather than a field of its own - and then measured: we sent level 18 in the
    // second slot and zero in the third, and the party window showed **"Magician" beside a
    // level of 0**. Job right, level zero, which is exactly what reading the third slot
    // produces. **[L]**, 2026-09-04, first party ever drawn by this server.
    w.u32(m.unknown_b); //   1406f2894  +0x18  the job lookup's second argument
    w.u32(m.level); //       1406f289f  +0x1c
    w.u32(m.unknown_d); //   1406f28aa  +0x20
    w.u8(0); //              1406f28b5
    w.u32(0); //             1406f28c3
    w.u64(0); //             1406f28ce
    w.zeros(0x78); //        1406f28e4  raw
}

/// Is this seat index one the client can survive?
///
/// **Code `0x2F` does not stop at six.** It warns about an index above five and then writes
/// twenty bytes past the end of a 120-byte array regardless (`0x1413bc488`), so nothing may
/// hand it one. Kept as a named predicate rather than a comment because
/// `CLAUDE.md`: *a comment describing a guarantee is not the guarantee*.
pub fn party_seat_is_in_range(seat: usize) -> bool {
    seat < PARTY_SEATS
}

/// **`0x0E` - a party now exists, and you lead it.** The reply to a successful create.
///
/// ```text
///   u32 partyId          1413bbb95
///   u8                   1413bbbaa
///   u32, u32, u32        1413bbbb8 / bbbc3 / bbbce
///   i16, i16             1413bbbd9 / bbbe1
///   MEMBER               1413bbbef   the leader's own seat
///   str                  1413bbbfa
///   u8, u8               1413bbc00 / bbc04
///   u32 leaderCharId     1413bbc08
/// ```
///
/// Every field is unconditional - **[L]**, this arm has no gate. The three `u32` and two
/// `i16` between the id and the seat are sent as zero: their widths are read off the listing
/// and their meanings are **not established**, and zero is the value a party with nothing in
/// it yet should carry. That is an assumption and it is written down as one.
pub fn party_created(party_id: u32, name: &str, leader: &Member) -> Vec<u8> {
    // **`new`, not `with_opcode` - the opcode is prepended by `Reply::packet()`.**
    //
    // This said the opposite for a day, and the sentence it said it in was wrong in both
    // halves: `with_opcode` is used by no other outbound builder in `crates/net`, and the
    // test that "caught" it was pinning the doubled shape rather than the right one.
    let mut w = PacketWriter::new();
    w.u8(result::CREATED);
    w.u32(party_id);
    w.u8(0);
    w.u32(0);
    w.u32(0);
    w.u32(0);
    w.i16(0);
    w.i16(0);
    write_member(&mut w, Some(leader));
    w.str(name);
    w.u8(0);
    w.u8(0);
    w.u32(leader.char_id);
    w.into_vec()
}

/// A [`PARTY_RESULT`] carrying only its code. **Refuses to build a code that reads fields.**
///
/// This is [`is_silent_code`] enforced at the builder rather than described above it -
/// `CLAUDE.md`: *"a comment describing a guarantee is not the guarantee"*. A `0x00A5` whose
/// arm expects a `str` and finds an empty body is the exact shape that has killed this client
/// twice, so the type system is not asked to be careful; the function returns `None`.
pub fn refusal(code: u8) -> Option<Vec<u8>> {
    if !is_silent_code(code) {
        return None;
    }
    // `new`, not `with_opcode`. See `party_created`: `Reply::packet()` prepends the opcode,
    // so putting it here sends it twice.
    let mut w = PacketWriter::new();
    w.u8(code);
    Some(w.into_vec())
}

/// The answer to a party request this server cannot handle.
///
/// Two bytes on the wire past the opcode's own two. The client shows *"Due to an unknown
/// error, your party request failed."* and returns normally from the handler. **[L]**
///
/// Use it for **every** [`CLIENT_PARTY_REQUEST`] the world does not act on. An unanswered
/// one freezes the client's whole UI.
pub fn request_failed() -> Vec<u8> {
    refusal(result::UNKNOWN_ERROR).expect("UNKNOWN_ERROR is in SILENT_CODES")
}

/// The eleven outcomes `0x1B` can carry. `research/party-result-0x00A5.md` §5.7, all **[L]**:
/// the arm's second jump table has one message per value and reads nothing else.
pub mod invite_outcome {
    /// *"You have invited '%s' to your party."* - what the leader sees after a successful invite.
    pub const INVITED: i32 = 0;
    /// *"%s is currently blocking any party invitations."*
    pub const BLOCKING: i32 = 1;
    /// *"'%s' is taking care of another invitation."*
    pub const BUSY: i32 = 2;
    /// *"You have already invited '%s' to your party."*
    pub const ALREADY_INVITED: i32 = 3;
    /// *"%s has denied the party request."*
    pub const DENIED: i32 = 4;
    /// *"'%s' could not be found in the current server."*
    pub const NOT_FOUND: i32 = 7;
    /// *"Party cannot be found. Please check the party info once again."*
    pub const NO_SUCH_PARTY: i32 = 8;
    /// *"The party you're trying to join is already in full capacity."*
    pub const FULL: i32 = 9;
    /// *"%s' is already in a party."*
    pub const ALREADY_IN_A_PARTY: i32 = 10;
}

/// `0x1B` - the invite **outcome**, to the inviter. `raw[4] outcome (i32), str name`, and the
/// body is always exactly `4 + 2 + len(name)` bytes past the code. **[L]**
/// (`research/party-result-0x00A5.md` §5.7.) Until 2026-09-05 a successful invite was
/// answered with `UNKNOWN_ERROR` because this was undecoded - which is what the owner saw as *"Due
/// to an unknown error, your party request failed"* after an invite that had in fact worked.
pub fn invite_outcome(outcome: i32, name: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(result::INVITE_OUTCOME);
    // Read as four raw bytes and used as an i32 - written as its little-endian bits.
    w.u32(outcome as u32);
    w.str(name);
    w.into_vec()
}

/// `0x03` - an invite arriving at the **target**. Six unconditional fields, so the body never
/// varies in shape; their meanings are mixed. `research/party-result-0x00A5.md` §5.4:
///
/// | # | | |
/// |---|---|---|
/// | 1 | `u32` | looked up in a client-side blocked-user list - **a character id [D]**: the inviter's |
/// | 2 | `u32` | **echoed back in the `0x0183` answer** - the invite's identity [D]: the party id |
/// | 3 | `str` | handed to the dialog - the inviter's name **[I]** |
/// | 4, 5, 6 | `u32` | handed to the dialog, meanings **not established** - sent as the inviter's level, job and 0 **[I]** |
///
/// The shape is [L]; which value goes where beyond field 2 is what the next client run
/// measures, and the reader accepts zeros for all of 4-6.
pub fn invite_notify(inviter_id: u32, party_id: u32, inviter_name: &str, level: u32, job: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(result::INVITE_NOTIFY_A);
    w.u32(inviter_id);
    w.u32(party_id);
    w.str(inviter_name);
    w.u32(level);
    w.u32(job);
    w.u32(0);
    w.into_vec()
}

/// `0x13` - *"'%s' has joined the party."* / *"You have joined the party."*, `str`. **[L]** shape;
/// which of the two strings the client picks is its own business (`research/party.md` table).
pub fn joined(name: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(result::JOIN);
    w.str(name);
    w.into_vec()
}

/// The invitee's answer, `0x0183`. `{u8 op, u8 answer, u64 value}` encoded as a FlatBuffers
/// table (`research/party.md`: op is `0x1B`, and `value` is field 2 of the `0x03` that opened
/// the dialog). **Slot order is [D]** - the struct's field order, which is how the `0x0182`
/// tables were laid out - and **the answer byte's values are [D]/[I]**: the auto-decline path
/// sends `1`; the dialog's two buttons send two other constants and which is accept has not
/// been measured. The world server acts only on what is established and logs the rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InviteAnswer {
    pub op: u8,
    pub answer: u8,
    pub value: u64,
}

/// The answer byte the client sends when it declines WITHOUT a dialog (`0x1413bafd3`). **[L]**
pub const INVITE_ANSWER_AUTO_DECLINE: u8 = 1;

pub fn parse_invite_answer(body: &[u8]) -> Option<InviteAnswer> {
    let root = read_u32(body, 0)? as usize;
    let (table, vtable) = table_at(body, root)?;
    let op = table_u8(body, table, vtable, 0)?.unwrap_or(0);
    let answer = table_u8(body, table, vtable, 1)?.unwrap_or(0);
    let value = table_u64(body, table, vtable, 2)?.unwrap_or(0);
    Some(InviteAnswer { op, answer, value })
}

/// What a [`CLIENT_PARTY_REQUEST`] asked for, as far as its body can be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyRequest {
    /// One of [`action`]. Slot 0 of the root table; **absent means 0**, which is
    /// [`action::CREATE`] - FlatBuffers omits a field equal to its schema default.
    pub action: u8,
    /// The payload union's type tag, slot 1. `5` for create and pick-up rights, `1` for
    /// leave, `2` for invite, `3` for join-request, `4` for expel and change-leader.
    /// **[L]** from the five builders' `struct+8`.
    pub payload_tag: u8,
    /// Slot 0 of the payload table **when the tag says it is a string** - tags 2 (invite)
    /// and 5 (create / pick-up rights). `None` otherwise.
    ///
    /// For invite this is **the target character's name**: `FUN_1413b9eb0` takes a `char*`,
    /// `strlen`s it and the union emits a FlatBuffers string. The client does NOT resolve it.
    /// **[L]**
    pub name: Option<String>,
    /// Slot 0 of the payload table **when the tag says it is an id** - tag 4, expel and
    /// change leader.
    ///
    /// **This is why the tag has to be looked at first.** Slot 0 of tag 4 is a `u64`
    /// character id, and reading it as a string uoffset - which this decoder used to do for
    /// every tag - returned `Some("")` for ids 1 and 4 instead of failing. That is
    /// `CLAUDE.md`'s *"look at the discriminator before you read the union"*, the same shape
    /// as the `_HEAP_FAILURE_INFORMATION.Address` decode, and it never errors: it prints
    /// something.
    ///
    /// Both wrappers resolve the name against the client's OWN member list first
    /// (`FUN_1413b8ec0` -> `FUN_1406f1ff0`) and send the id, so a server that re-resolved a
    /// name here would be wrong twice - the client never sent one, and it already proved the
    /// target is a member. **[L]**
    pub target_id: Option<u32>,
}

/// Read a [`CLIENT_PARTY_REQUEST`] body. `None` for anything that does not decode.
///
/// # The body is FlatBuffers, and that is measured twice
///
/// `tools/encodes.py 0x1413bd0f0` reports **exactly one write**, a `w_raw`. So the whole body
/// is one opaque blob. **[L]** Reading the builder shows what kind: it grows a 0x400 buffer
/// downward (`mov qword [rbp-9], rcx / sub rcx, r13`), pads to 4 (`neg rdx / and edx,3`),
/// writes `u16` field offsets into a side table and finally stores a signed offset back to
/// it. That is FlatBuffers' vtable layout, and the archived capture parses cleanly as one.
/// **[D]**
///
/// # The capture
///
/// `research/fixtures/portal-arrival-fixed-chat-and-party-world.log` line 267, 16:48:31.299,
/// 68 bytes. It is the **only** `0x0182` in 452 archived logs, and the fixture's own name says
/// what it is: `portal-arrival-fixed-chat-and-party`. Decoded:
///
/// ```text
/// 00  10 00 00 00              root uoffset -> 0x10
/// 06  0a 00 0e 00              vtable: size 10, table size 14
/// 0a  00 00  07 00  08 00      slot 0 absent, slot 1 at +7, slot 2 at +8
/// 10  0a 00 00 00              table: soffset 10 back to the vtable at 0x06
/// 17  05                       slot 1: union tag = 5
/// 18  0c 00 00 00              slot 2: uoffset -> 0x24
/// 1e  06 00 08 00 04 00        payload vtable: size 6, table size 8, slot 0 at +4
/// 24  06 00 00 00              payload table: soffset 6
/// 28  04 00 00 00              slot 0: uoffset -> 0x2c
/// 2c  11 00 00 00              string, 17 bytes
/// 30  "TestCharD's Party"
/// ```
///
/// Slot 0 absent means `action = 0` = [`action::CREATE`], tag 5 is what `FUN_1413bdb80`
/// writes, and the name is `String.wz` `0x0BAC` *"%s's Party"* with the character's name.
/// Three independent facts agreeing on one 68-byte capture. **[L]**
///
/// # What this decoder does NOT establish
///
/// **One capture, one action.** `action` and `payload_tag` are plain table slots and are read
/// the same way whatever the action is, so those two fields should survive. The payload
/// schema is only known for tag 5, and only for its first slot. Every other action's payload
/// is **unread**; a decoded `name` of `None` means "this decoder did not find one", never
/// "the packet did not carry one".
pub fn parse_request(body: &[u8]) -> Option<PartyRequest> {
    let root = read_u32(body, 0)? as usize;
    let (table, vtable) = table_at(body, root)?;
    let action = table_u8(body, table, vtable, 0)?.unwrap_or(0);
    let payload_tag = table_u8(body, table, vtable, 1)?.unwrap_or(0);

    // The payload is best-effort: a body whose action and tag read cleanly is still worth
    // acting on, and a payload this decoder cannot follow must not turn into "no request".
    //
    // **The tag decides how slot 0 is read**, and reading it the same way for every tag is
    // what made an expel of character 4 decode as an empty name instead of failing.
    let mut name = None;
    let mut target_id = None;
    if let Some(Some(off)) = table_uoffset(body, table, vtable, 2) {
        if let Some((ptable, pvtable)) = table_at(body, off) {
            match payload_tag {
                // 2 = invite, 5 = create / pick-up rights. Slot 0 is a string.
                2 | 5 => {
                    if let Some(Some(soff)) = table_uoffset(body, ptable, pvtable, 0) {
                        name = read_string(body, soff);
                    }
                }
                // 4 = expel / change leader. Slot 0 is a `u64` character id, zero-extended
                // from the `u32` the client resolved locally.
                4 => {
                    if let Some(Some(id)) = table_u64(body, ptable, pvtable, 0) {
                        target_id = u32::try_from(id).ok();
                    }
                }
                // 1 = leave: the table is always empty. 3 = join request: its `i64` slot 0 is
                // of unestablished meaning and its builder has zero callers by three separate
                // scans, so nothing reads it here rather than inventing a meaning.
                _ => {}
            }
        }
    }
    Some(PartyRequest { action, payload_tag, name, target_id })
}

// --- the smallest FlatBuffers reader that can answer the question above ------------------
//
// Deliberately not a dependency. Every accessor is bounds-checked and every failure is
// `None`; this parses bytes that arrive over a socket, and `CLAUDE.md` has two entries about
// a decoder that produced a plausible-looking answer instead of an error.

fn slice(b: &[u8], at: usize, n: usize) -> Option<&[u8]> {
    // `at + n` is a checked add on purpose: a hostile uoffset near usize::MAX would wrap and
    // hand `get` a range that looks reasonable.
    b.get(at..at.checked_add(n)?)
}

fn read_u16(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(slice(b, at, 2)?.try_into().ok()?))
}

fn read_u32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(slice(b, at, 4)?.try_into().ok()?))
}

fn read_i32(b: &[u8], at: usize) -> Option<i32> {
    Some(i32::from_le_bytes(slice(b, at, 4)?.try_into().ok()?))
}

/// `(table start, vtable start)` for the table whose header sits at `at`.
///
/// The soffset is *subtracted*, and it may legitimately be negative, so this is `i64`
/// arithmetic rather than `usize` - a `usize` subtraction here underflows to an enormous
/// index on a hostile body and every later `get` then fails for the wrong reason.
fn table_at(b: &[u8], at: usize) -> Option<(usize, usize)> {
    let soffset = read_i32(b, at)? as i64;
    let vt = at as i64 - soffset;
    if vt < 0 || vt as usize >= b.len() {
        return None;
    }
    Some((at, vt as usize))
}

/// The byte offset of field `slot` inside the table, or `None` if the field is absent.
fn field_offset(b: &[u8], table: usize, vtable: usize, slot: usize) -> Option<Option<usize>> {
    let vt_size = read_u16(b, vtable)? as usize;
    let entry = 4 + slot * 2;
    if entry + 2 > vt_size {
        return Some(None); // the vtable is shorter than this slot: absent, not malformed
    }
    let voffset = read_u16(b, vtable + entry)? as usize;
    if voffset == 0 {
        return Some(None); // present in the vtable, written as its default
    }
    Some(Some(table + voffset))
}

fn table_u8(b: &[u8], table: usize, vtable: usize, slot: usize) -> Option<Option<u8>> {
    match field_offset(b, table, vtable, slot)? {
        None => Some(None),
        Some(at) => Some(Some(*b.get(at)?)),
    }
}

/// A field holding a `u64`, little-endian.
///
/// Tag 4's character id. The client zero-extends a `u32` into it (`mov eax, eax` before the
/// store), so the high dword is always zero and the caller narrows it back - but the field on
/// the wire is eight bytes and eight-aligned, and reading four would leave the rest of the
/// table misaligned. **[L]**
fn table_u64(b: &[u8], table: usize, vtable: usize, slot: usize) -> Option<Option<u64>> {
    match field_offset(b, table, vtable, slot)? {
        None => Some(None),
        Some(at) => {
            let bytes = slice(b, at, 8)?;
            Some(Some(u64::from_le_bytes(bytes.try_into().ok()?)))
        }
    }
}

/// A field holding a uoffset, resolved to an absolute index.
///
/// The outer `Option` is "the vtable could not be read"; the inner one is "this slot is
/// absent", which is an ordinary, expected answer.
fn table_uoffset(b: &[u8], table: usize, vtable: usize, slot: usize) -> Option<Option<usize>> {
    let at = match field_offset(b, table, vtable, slot)? {
        Some(at) => at,
        None => return Some(None),
    };
    let rel = read_u32(b, at)? as usize;
    Some(Some(at.checked_add(rel)?))
}

fn read_string(b: &[u8], at: usize) -> Option<String> {
    let len = read_u32(b, at)? as usize;
    let bytes = slice(b, at.checked_add(4)?, len)?;
    String::from_utf8(bytes.to_vec()).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The one archived `0x0182`, body only, opcode stripped.
    ///
    /// `research/fixtures/portal-arrival-fixed-chat-and-party-world.log:267`.
    const ARCHIVED_CREATE: &str = "1000000000000a000e000000070008000a0000000000000\
                                   50c00000000000600080004000600000004000000110000\
                                   005465737443686172442773205061727479000000";

    fn hex(s: &str) -> Vec<u8> {
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        (0..s.len() / 2)
            .map(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn the_archived_capture_is_68_bytes() {
        // The log line says "68 byte body" and the transcription has to agree with it, or
        // every offset below is being checked against something I typed rather than
        // something the client sent.
        assert_eq!(hex(ARCHIVED_CREATE).len(), 68);
    }

    #[test]
    fn the_archived_capture_decodes_as_create_with_its_name() {
        let got = parse_request(&hex(ARCHIVED_CREATE)).expect("the capture must decode");
        assert_eq!(
            got,
            PartyRequest {
                action: action::CREATE,
                payload_tag: 5,
                name: Some("TestCharD's Party".to_string()),
                // Tag 5's slot 0 is a string, so no id is recovered - and asking for one
                // would be the misread this field exists to prevent.
                target_id: None,
            }
        );
    }

    /// **The tag decides how slot 0 is read, and this pins the discriminator.**
    ///
    /// Tag 4 (expel / change leader) puts a `u64` character id there. Reading it as a string
    /// uoffset - the way every tag used to be read - does not error: for ids 1 and 4 it
    /// returned `Some("")`. `CLAUDE.md`: *look at the discriminator before you read the
    /// union*, the same shape as decoding a heap failure's `Address` without its type byte.
    ///
    /// There is exactly ONE party packet in 530 archived logs, so this asserts the property
    /// on the body we HAVE rather than on one assembled from a listing: a string payload
    /// yields a name and no id, and the two fields are never both populated.
    #[test]
    fn the_payload_tag_decides_which_field_is_populated() {
        let got = parse_request(&hex(ARCHIVED_CREATE)).expect("the archived create");
        assert_eq!(got.payload_tag, 5, "the create's tag");
        assert_eq!(got.target_id, None, "a string payload yields no id");
        assert!(got.name.is_some(), "and it still yields its name");
        assert!(
            got.name.is_none() || got.target_id.is_none(),
            "name and id are the two readings of one slot and cannot both apply"
        );
    }

    #[test]
    fn no_prefix_of_the_capture_panics_and_none_of_them_invents_the_name() {
        let full = hex(ARCHIVED_CREATE);
        // The string's 17 bytes start at 0x30, so the shortest prefix that really contains
        // the name is 0x30 + 17 = 65 - and the archived body is 68. **The last three bytes
        // are FlatBuffers' alignment padding**, which is independent corroboration of the
        // decode: nothing in the layout above is left over.
        const NAME_ENDS_AT: usize = 0x30 + 17;
        assert_eq!(full.len() - NAME_ENDS_AT, 3, "three bytes of trailing padding");
        for n in 0..full.len() {
            // Every prefix. None may panic, and none *shorter than the name* may hand the
            // name back - a decoder that produces a plausible answer instead of an error is
            // the failure `CLAUDE.md` records twice.
            match parse_request(&full[..n]) {
                Some(got) if n < NAME_ENDS_AT => {
                    assert_ne!(got.name.as_deref(), Some("TestCharD's Party"), "prefix {n}");
                }
                _ => {}
            }
        }
        // The string's length prefix starts at 0x2c, so 0x2c bytes is the longest prefix
        // that cannot possibly carry the name, and it must still yield the action.
        let head = parse_request(&full[..0x2c]).expect("the table itself is complete by 0x2c");
        assert_eq!(head.action, action::CREATE);
        assert_eq!(head.payload_tag, 5);
        assert_eq!(head.name, None);
    }

    #[test]
    fn an_empty_body_is_none() {
        assert!(parse_request(&[]).is_none());
    }

    #[test]
    fn a_wild_root_offset_is_none() {
        let mut b = hex(ARCHIVED_CREATE);
        b[0] = 0xFF;
        b[1] = 0xFF;
        assert!(parse_request(&b).is_none());
    }

    #[test]
    fn a_wild_soffset_is_none_not_an_underflow() {
        let mut b = hex(ARCHIVED_CREATE);
        // The table's soffset, at 0x10, points back to the vtable. Make it point forward
        // past the end of the buffer instead.
        b[0x10] = 0x00;
        b[0x11] = 0x00;
        b[0x12] = 0x00;
        b[0x13] = 0x80;
        assert!(parse_request(&b).is_none());
    }

    #[test]
    fn a_missing_payload_still_yields_the_action() {
        // Zero slot 2's voffset in the vtable: the payload becomes absent. The action and
        // the tag must still come back, because they are what the world actually switches
        // on and losing them would turn a partial decode into no decode at all.
        let mut b = hex(ARCHIVED_CREATE);
        b[0x0E] = 0;
        b[0x0F] = 0;
        let got = parse_request(&b).expect("action and tag survive a missing payload");
        assert_eq!(got.action, action::CREATE);
        assert_eq!(got.payload_tag, 5);
        assert_eq!(got.name, None);
    }

    #[test]
    fn refusal_builds_the_one_byte_body_for_every_silent_code() {
        for &code in SILENT_CODES {
            let pkt = refusal(code).unwrap_or_else(|| panic!("{code:#04x} must build"));
            // **One byte. The opcode is `Reply::packet()`'s to add**, and this asserted
            // three for a day - which is how `a5 00 | a5 00 04` reached the client.
            assert_eq!(pkt, vec![code], "the body is the code and nothing else");
        }
    }

    #[test]
    fn refusal_will_not_build_a_code_whose_arm_reads_fields() {
        // Every one of these has at least one READ in its arm. Building a bodyless one is
        // the failure that has killed this client twice, so the builder has to say no.
        for code in [
            result::INVITE_NOTIFY_A,
            result::INVITE_NOTIFY_B,
            result::UNNAMED_0D,
            result::CREATE_OK,
            result::WITHDRAW,
            result::JOIN,
            result::JOIN_2,
            result::INVITE_BLOCKED_THERE,
            result::INVITE_OUTCOME,
            result::INVITE_OUTCOME_2,
            result::LEADER_CHANGED,
            result::UNNAMED_2A,
            result::PUBLIC_PRIVATE,
            result::UNNAMED_2F,
        ] {
            assert!(refusal(code).is_none(), "{code:#04x} reads fields and must be refused");
        }
    }

    #[test]
    fn every_code_outside_the_table_is_silent() {
        // The client's own bound check is `add eax,-3 / cmp eax,0x2c / ja default`, so
        // anything below 0x03 or above 0x2F takes the read-free default arm.
        for code in [0x00u8, 0x01, 0x02, 0x30, 0x7F, 0xFF] {
            assert!(is_silent_code(code));
            assert!(refusal(code).is_some());
        }
    }

    /// **The body starts at the CODE. `Reply::packet()` puts the opcode in front of it.**
    ///
    /// This asserted `[0xA5, 0x00, UNKNOWN_ERROR]` for a day, and passed, and the wire
    /// carried `a5 00 | a5 00 04`. It could not fail on the packet it was written for:
    /// `UNKNOWN_ERROR` is itself a default-arm code, so the client read `0xA5`, fell to the
    /// default arm, and printed **the message the packet intended**. The refusal worked by
    /// accident and hid the defect from every refusal test there is.
    ///
    /// `party_created` is what exposed it - `0x0E` is a real arm, and it was never entered.
    #[test]
    fn a_result_body_starts_at_the_code_byte_and_not_at_the_opcode() {
        assert_eq!(request_failed(), vec![result::UNKNOWN_ERROR]);
        assert_eq!(refusal(result::JOIN_REFUSED_FULL), Some(vec![result::JOIN_REFUSED_FULL]));

        // And the one that has a body: it starts with the code, and `0xA5` appears nowhere
        // in the first two bytes.
        let m = Member { char_id: 214, name: "Tester2".into(), ..Member::default() };
        let created = party_created(1, "Tester2's Party", &m);
        assert_eq!(created[0], result::CREATED, "the code is the first byte");
        assert_ne!(created[0], 0xA5, "not the opcode - that is Reply::packet's job");
    }

    #[test]
    fn the_three_opcodes_are_distinct_and_not_reused() {
        // 0x0182 inbound is a completely different packet (`u32, u8`). Nothing in this
        // module may send it, and this test exists so that a future edit that reaches for
        // "the party opcode" has to notice there are three of them.
        assert_eq!(PARTY_RESULT, 0x00A5);
        assert_eq!(CLIENT_PARTY_REQUEST, 0x0182);
        assert_eq!(CLIENT_PARTY_INVITE_ANSWER, 0x0183);
        assert_ne!(PARTY_RESULT, CLIENT_PARTY_REQUEST);
    }

    #[test]
    fn the_seven_actions_are_seven_distinct_values() {
        let all = [
            action::CREATE,
            action::LEAVE,
            action::SET_PICKUP_RIGHTS,
            action::INVITE,
            action::JOIN_REQUEST,
            action::EXPEL,
            action::CHANGE_LEADER,
        ];
        let mut seen = all.to_vec();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), all.len());
        assert_eq!(seen, vec![0, 1, 2, 3, 4, 5, 6]);
    }
}

#[cfg(test)]
mod invite_tests {
    use super::*;

    /// The outcome body is exactly `code + 4 + 2 + len(name)`, the length the arm reads.
    #[test]
    fn the_invite_outcome_is_the_length_the_client_reads() {
        let body = invite_outcome(invite_outcome::INVITED, "Tester2");
        assert_eq!(body.len(), 1 + 4 + 2 + 7);
        assert_eq!(body[0], result::INVITE_OUTCOME);
        assert_eq!(&body[1..5], &0i32.to_le_bytes());
        let denied = invite_outcome(invite_outcome::DENIED, "Tester2");
        assert_eq!(&denied[1..5], &4i32.to_le_bytes());
    }

    /// Six fields, all present, in the order the arm reads them.
    #[test]
    fn the_invite_notify_has_six_unconditional_fields() {
        let body = invite_notify(213, 1, "Cobalt", 18, 200);
        assert_eq!(body.len(), 1 + 4 + 4 + (2 + 6) + 4 + 4 + 4);
        assert_eq!(body[0], result::INVITE_NOTIFY_A);
        assert_eq!(&body[1..5], &213u32.to_le_bytes(), "field 1, the inviter");
        assert_eq!(&body[5..9], &1u32.to_le_bytes(), "field 2, echoed back in 0x0183");
        assert_eq!(&body[9..11], &6u16.to_le_bytes());
        assert_eq!(&body[11..17], b"Cobalt");
        assert_eq!(&body[17..21], &18u32.to_le_bytes());
        assert_eq!(&body[21..25], &200u32.to_le_bytes());
        assert_eq!(&body[25..29], &0u32.to_le_bytes());
    }

    #[test]
    fn joined_is_the_code_and_a_string() {
        let body = joined("Tester2");
        assert_eq!(body, [&[result::JOIN][..], &7u16.to_le_bytes(), b"Tester2"].concat());
    }

    /// A hand-built FlatBuffers table `{op, answer, value}` in slot order: root offset, then
    /// the vtable, then the table whose soffset points back at it. This pins the READER; the
    /// slot order it assumes is the [D] part and a real capture is what checks it.
    #[test]
    fn the_invite_answer_reader_walks_a_three_slot_table() {
        let mut b = Vec::new();
        b.extend_from_slice(&14u32.to_le_bytes()); // root: the table starts at 14
        // vtable at 4: size 10, table size 16, slots at +4, +5, +8
        for v in [10u16, 16, 4, 5, 8] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        // table at 14: soffset = table - vtable = 10
        b.extend_from_slice(&10i32.to_le_bytes());
        b.push(0x1B); // op
        b.push(1); // answer
        b.extend_from_slice(&[0, 0]); // padding to the u64
        b.extend_from_slice(&7u64.to_le_bytes()); // value: the party id we sent as field 2
        assert_eq!(b.len(), 30);
        assert_eq!(
            parse_invite_answer(&b),
            Some(InviteAnswer { op: 0x1B, answer: INVITE_ANSWER_AUTO_DECLINE, value: 7 })
        );
        assert_eq!(parse_invite_answer(&b[..8]), None, "a truncated body is None, not a guess");
    }
}
