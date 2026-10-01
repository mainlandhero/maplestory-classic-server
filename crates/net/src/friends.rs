//! The friend list: `0x0193` from the client, `0x00A7` back.
//!
//! The owner, 2026-09-21: *"Tester2 just tried adding the owner as a friend, but nothing showed up on
//! The owner's screen."* It would not: the request arrived (one `0x0193`, with `str "Wisp"` and
//! `str "Default Group"`, in `world-ch0.log` at 01:35:06.649) and this server had no friend
//! code at all. `research/friends-2026-09-21.md` is the decode.
//!
//! # The pair, and how it was found
//!
//! The friend window reads its rows out of the container at `ctx+0x23C8`;
//! `tools/fieldrefs.py 0x23c8` over the context's code names all 23 functions that touch it,
//! and exactly one is a packet handler - `FUN_142defd40`, which
//! `research/msexe-gamestage-cases.txt` lists as **`0x00A7`**. **[L]** So the client's
//! `CNM*Friend*` (Nexon "Account Friends") layer is a different feature and is not involved.
//!
//! Both directions carry a **sub-op first**: the client's are `1..0x14` and the server's
//! `0x15..0x36`, one shared enum split down the middle. **[L]**
//!
//! # What is decoded and what is not
//!
//! Every message-only reply below is **[L]**: the case sets a string id and falls into one
//! shared "say it in chat" tail, and the ids were decrypted with `tools/dump_stringids.py`, so
//! the wording in each doc comment is this client's own. [`friend_list`] is **[L]** on its
//! shape and **[D]** on its meaning - it fills the manager the window draws from, but whether
//! those `{id, name}` rows ARE the drawn list or the *group* list is the one thing a screen
//! has to settle. [`FRIEND_ENTRY_LEN`] explains what is deliberately not built yet.

use crate::error::{NetError, Result};
use crate::{PacketReader, PacketWriter};

/// What the friend window sends. Ten builders, all `FUN_1406ed520(buf, 0x193)`. **[L]**
pub const CLIENT_FRIEND_REQUEST: u16 = 0x0193;

/// What the server answers with - `FUN_142defd40`, `0x00A7` in the game-stage table. **[L]**
pub const FRIEND_RESULT: u16 = 0x00A7;

/// A full friend record is **329 raw bytes**: `FUN_1402d4870` is `read_raw(pkt, dest, 0x149)`
/// and the manager's own search loop strides by `0x149`. **[L]**
///
/// # It is decoded now, and it had to be - 2026-09-22
///
/// The owner: *"Adding someone as a friend causes a fatal client crash to whoever the invitation
/// was sent to."* The client named the packet itself: `0x009E CLIENT_PACKET_REJECTED`, class
/// 1, reason `0x26` ("a decoder asked for more bytes than the packet had left"), echoing our
/// 34-byte `0x00A7` verbatim. The `0x1A` arm reads `u8, u32, u32, str, u32, u32, u32` - which
/// is **exactly** the 28-byte body this module used to build, to the byte - and then, when the
/// manager's parallel arrays are the same length, calls [`FRIEND_ENTRY_LEN`]'s reader. Nothing
/// was left. `research/friends-2026-09-21.md` section 6.
///
/// # The layout, and the four things that pin it
///
/// Three offsets are read off `FUN_142dec8f0`, the function that walks the loaded records:
/// `*rec` is the id it hands the balloon, `rec + 4` is walked by a `strlen` loop, and
/// **`rec[0x11] == 1`** is the test that decides to raise a request balloon at all. That
/// fixes the name field at `4..0x11`, i.e. 13 bytes. The fourth is the reference tree's
/// `Friend.encode` (v214, a **different game version** - a candidate generator, never a
/// fact): `int friendID, str13 name, byte flag, int channel, str17 group, byte mobile, int
/// friendAccountID, str13 nickname, str256 memo, int inShop`. It agrees with all three
/// measured offsets, and its `FriendFlag` enum reads `Friend(0), FriendRequest(1),
/// FriendOffline(2), FriendOnline(3)` - so the `== 1` the client tests for is that enum's
/// `FriendRequest`. Four instruments, one layout.
///
/// So: offsets `0`, `4` and `0x11` are **[L]**; everything from `0x12` on is **[I]** from the
/// reference and should be treated as a candidate.
///
/// # The tail, and why all 329 bytes are now accounted for
///
/// The reference's fields add up to 317, twelve short. Three things close the gap:
///
/// * **`0x145` is [L]**: sub-op `0x2D` reads the row's `+0x12` and `+0x145` and does nothing
///   at all unless one of them differs from what it carries, so `0x145` is the status word it
///   pairs with the channel. `0x145 + 4 = 0x149`, which ends the record exactly.
/// * That leaves `0x139`, `0x13D` and `0x141` - and `0x2D`'s detail block writes those three,
///   in that order, right after the name, each with `-1` meaning "leave alone".
/// * The reference has `inShop` as its last `int` (which lands on `0x139`) and its invite
///   packet carries **level, job, subJob** as three extra ints this version's `Friend` does
///   not have. Three words, three fields, one order.
///
/// ~~So `0x139` = inShop, `0x13D` = level, `0x141` = job~~ - **that reading was wrong, and the
/// screen said so**: both columns stayed blank (the owner, 2026-09-23). The row builder was then
/// found and read instead of inferred. `FUN_142cc3ea0(ctx, i)` is the context's "record *i*"
/// accessor; its caller `FUN_1411be0a0` builds each row, and reads the tail like this: **[L]**
///
/// ```text
/// *(u32*)(rec + 0x139)                       -> copied into the row as an int   = LV
/// FUN_1402b0250(*(u32*)(rec + 0x13d),
///               *(u32*)(rec + 0x141))        -> a string                        = JOB
/// ```
///
/// and `FUN_1402b0250(job, subJob)` is the **job-name lookup**: it builds a map keyed by job id
/// (`0 -> "Beginner"`, `100`, `110`, `111`, ...) and special-cases `job == 400 && subJob == 1`,
/// the Dual Blade. So **`0x139` = level, `0x13D` = job, `0x141` = subJob, `0x145` = status**.
/// The same row builder also reads `0x12` (channel), `0x16` (group), `0x2C` (nickname) and
/// `0x39` (memo), confirming the reference's layout for every one of them. The reference's
/// `inShop` does not exist in this version; the reference being right about the first 313
/// bytes is what made its last field look safe.
pub const FRIEND_ENTRY_LEN: usize = 0x149;

/// `rec[0x11]`: an ordinary friend. **[I]** (the reference's `FriendFlag`).
pub const FLAG_FRIEND: u8 = 0;
/// `rec[0x11]`: **a request this character has not answered.** The client raises its own
/// *"Friend request from"* balloon for any record carrying this - `FUN_142dec8f0` tests
/// `rec[0x11] == 1` literally. **[L]** on the test, **[I]** on the name.
pub const FLAG_REQUEST: u8 = 1;
/// `rec[0x11]`: a friend who is not logged in. **[I]**
pub const FLAG_OFFLINE: u8 = 2;
/// `rec[0x11]`: a friend who is. **[I]**
pub const FLAG_ONLINE: u8 = 3;

/// One row of the friend manager, as the 329 bytes the client stores and strides over.
///
/// See [`FRIEND_ENTRY_LEN`] for which offsets are measured and which are candidates.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FriendRecord {
    /// `+0x00` **[L]**: the friend's character id - the number `FUN_142dec8f0` hands the
    /// balloon, and the one the Yes and No echo back.
    pub character_id: u32,
    /// `+0x04` **[L]**: 13 bytes, NUL-terminated. Longer names are truncated to 12 rather
    /// than running into the flag.
    pub name: String,
    /// `+0x11` **[L]**: one of `FLAG_*`. `1` is what makes the client pop the request.
    pub flag: u8,
    /// `+0x12` **[I]**: the channel they are on. The reference notes `0` means "the same
    /// character", so an offline friend gets `0`.
    pub channel: u32,
    /// `+0x16` **[I]**: 17 bytes, the group the window files them under.
    pub group: String,
    /// `+0x28` **[I]**: their account id. This server has no reason to reveal one, so it
    /// sends the character id.
    pub account_id: u32,
    /// `+0x2C` **[I]**: 13 bytes.
    pub nickname: String,
    /// `+0x39` **[I]**: 256 bytes.
    pub memo: String,
    /// `+0x139` **[L]**: the **LV** column, copied into the row as an int by `FUN_1411be0a0`.
    pub level: u32,
    /// `+0x13D` **[L]**: the **JOB** column, the first argument of the job-name lookup
    /// `FUN_1402b0250(job, subJob)`. `+0x141`, the subJob, goes out as 0.
    pub job: u32,
}

impl FriendRecord {
    /// A friend, online or not.
    ///
    /// `level` and `job` ride the record rather than a presence packet, which is what makes
    /// them show for an offline friend too - the owner, 2026-09-22: *"please also make sure that
    /// Job and level information show even if the character is offline."*
    pub fn friend(
        character_id: u32,
        name: &str,
        group: &str,
        channel: Option<u32>,
        level: u32,
        job: u32,
    ) -> FriendRecord {
        FriendRecord {
            character_id,
            name: name.to_string(),
            flag: if channel.is_some() { FLAG_ONLINE } else { FLAG_OFFLINE },
            channel: channel.unwrap_or(0),
            group: group.to_string(),
            account_id: character_id,
            level,
            job,
            ..FriendRecord::default()
        }
    }

    /// Somebody whose request is waiting - the record the client pops a balloon for.
    pub fn request(character_id: u32, name: &str, group: &str) -> FriendRecord {
        FriendRecord {
            character_id,
            name: name.to_string(),
            flag: FLAG_REQUEST,
            group: group.to_string(),
            account_id: character_id,
            ..FriendRecord::default()
        }
    }

    /// The 329 bytes, at the offsets [`FRIEND_ENTRY_LEN`] documents.
    ///
    /// ```
    /// use net::friends::{FriendRecord, FRIEND_ENTRY_LEN, FLAG_REQUEST};
    /// let rec = FriendRecord::request(214, "Tester2", "Default Group").encode();
    /// assert_eq!(rec.len(), FRIEND_ENTRY_LEN);
    /// assert_eq!(&rec[0..4], &214u32.to_le_bytes());   // the id the balloon echoes
    /// assert_eq!(&rec[4..11], b"Tester2");             // a C string, 13 bytes wide
    /// assert_eq!(rec[0x10], 0);                        // its terminator fits
    /// assert_eq!(rec[0x11], FLAG_REQUEST);             // and this is what pops the balloon
    /// ```
    pub fn encode(&self) -> Vec<u8> {
        let mut b = vec![0u8; FRIEND_ENTRY_LEN];
        b[0x00..0x04].copy_from_slice(&self.character_id.to_le_bytes());
        fixed(&mut b, 0x04, 13, &self.name);
        b[0x11] = self.flag;
        b[0x12..0x16].copy_from_slice(&self.channel.to_le_bytes());
        fixed(&mut b, 0x16, 17, &self.group);
        // +0x27 mobile: this client has no mobile login, so 0.
        b[0x28..0x2C].copy_from_slice(&self.account_id.to_le_bytes());
        fixed(&mut b, 0x2C, 13, &self.nickname);
        fixed(&mut b, 0x39, 256, &self.memo);
        b[0x139..0x13D].copy_from_slice(&self.level.to_le_bytes());
        b[0x13D..0x141].copy_from_slice(&self.job.to_le_bytes());
        // +0x141 subJob stays 0: it only changes the name for job 400 (subJob 1 = Dual Blade),
        // which this client's classes never reach.
        // +0x145 is the status word; only 0x2D writes it, and it is left at zero here on
        // purpose - see `friend_status`.
        b
    }
}

/// A fixed-width NUL-terminated field: at most `width - 1` bytes of text, so the terminator
/// can never be pushed into the field that follows.
fn fixed(dst: &mut [u8], at: usize, width: usize, s: &str) {
    let n = s.as_bytes().len().min(width - 1);
    dst[at..at + n].copy_from_slice(&s.as_bytes()[..n]);
}

/// Sub-op 0x15: **the friend list itself** - `u32 count`, then `count * 329` bytes in one
/// block. **[L]**, `FUN_142deb010`: it sizes three parallel arrays, then
/// `read_raw(pkt, arr, count * 0x149)` in a single call, resolves two flags per record, and
/// refreshes the window. The arm then calls `FUN_142dec8f0`, which is what raises a balloon
/// for every record whose [`FriendRecord::flag`] is [`FLAG_REQUEST`].
///
/// **This, not [`friend_list`], is what the window draws.** Confirmed on screen 2026-09-22:
/// the Buddy tab lists "Default Group (1/1)" with a NAME / JOB / LV row, so the measured half
/// of the layout is right. JOB and LV came back **blank**, which is the unknown 12-byte tail
/// (`0x13D`..`0x149`) going out as zeros - see [`FriendRecord`].
///
/// ```
/// use net::friends::{friend_records, FriendRecord, FRIEND_ENTRY_LEN};
/// let body = friend_records(&[FriendRecord::friend(214, "Tester2", "Default Group", Some(0), 21, 300)]);
/// assert_eq!(body[0], 0x15);
/// assert_eq!(&body[1..5], &1u32.to_le_bytes());
/// assert_eq!(body.len(), 5 + FRIEND_ENTRY_LEN);
/// ```
pub fn friend_records(records: &[FriendRecord]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(0x15);
    w.u32(records.len() as u32);
    for r in records {
        w.bytes(&r.encode());
    }
    w.into_vec()
}

// ---------------------------------------------------------------------------------------
// 0x0193 - the requests
// ---------------------------------------------------------------------------------------

/// Sub-op 1: add a friend by name. `FUN_141828b90`.
pub const REQUEST_ADD: u8 = 1;
/// Sub-op 2: **the popup's Yes.** `FUN_14180b750`, the balloon's confirm handler, calls
/// `FUN_141829a70(ctx, balloon+0x328, kind == 0x0F)`, which writes `(flag != 0) + 2`. So a
/// kind-`0x0E` balloon - the one [`friend_request_popup`] raises - answers with **2**, and the
/// account-friend variant (`0x0F`) with 3. **[L]**
pub const REQUEST_ACCEPT: u8 = 2;
/// Sub-op 3: the same Yes from the account-friend balloon.
pub const REQUEST_ACCEPT_ACCOUNT: u8 = 3;
/// Sub-ops 4 and 5: `u32, str`. `FUN_141829640`, reached from the friend WINDOW
/// (`FUN_1411c54e0`), not from the popup. **[I]** which.
pub const REQUEST_BY_ID_NAMED: u8 = 4;
/// Sub-op 6: **the popup's No.** `FUN_14180c6e0` - the balloon's cancel handler - calls
/// `FUN_1418297d0(ctx, balloon+0x328, kind == 0x0F, 0)`, which writes `(flag != 0) + 6`.
/// **[L]**
pub const REQUEST_REFUSE: u8 = 6;
/// Sub-op 7: the same No from the account-friend balloon.
pub const REQUEST_REFUSE_ACCOUNT: u8 = 7;
/// Sub-op 0x14: the client handing back its group names. `FUN_1411c2010`.
pub const REQUEST_GROUPS: u8 = 0x14;

/// One `0x0193` from the client.
///
/// **The sub-op is kept on every arm**, because which of `2`/`3`, `4`/`5` and `6`/`7` is
/// accept, refuse or delete is **[I]** - the pairs differ by one flag byte in the client and
/// nothing read so far names them. The handler logs the number it got, so the next run's log
/// says which one the Accept button sends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FriendRequest {
    /// Sub-op 1: `str name, str group, str ?, u8 flag, [str ? when flag]`.
    Add {
        /// Who to add, as typed.
        name: String,
        /// The group the window put them in - `"Default Group"` in the captured body.
        group: String,
        /// A third string, empty in the capture. Kept rather than dropped.
        note: String,
        /// The flag byte, `1` in the capture.
        flag: u8,
    },
    /// **Sub-ops 2 and 3: the popup's Yes.** `character_id` is whatever
    /// [`friend_request_popup`] put in its first `u32`, echoed back.
    Accept { sub_op: u8, character_id: u32 },
    /// **Sub-ops 6 and 7: the popup's No.**
    Refuse { sub_op: u8, character_id: u32 },
    /// Sub-ops 0x0B, 0x12, 0x13: a character id and nothing else.
    ById { sub_op: u8, character_id: u32 },
    /// Sub-ops 4, 5: a character id and a string.
    ByIdNamed { sub_op: u8, character_id: u32, name: String },
    /// Sub-op 0x0C: `u8, u32, u32, str, str`. The memo/category shape.
    Rearrange { sub_op: u8, character_id: u32 },
    /// Sub-op 0x14: the client's own group names, by index.
    Groups { groups: Vec<(u8, String)> },
    /// Any sub-op this server does not model. **Answered anyway** - see the module docs of
    /// `world::session::friends`.
    Unknown { sub_op: u8 },
}

impl FriendRequest {
    /// The sub-op byte this request arrived under, whatever shape it took.
    pub fn sub_op(&self) -> u8 {
        match self {
            FriendRequest::Add { .. } => REQUEST_ADD,
            FriendRequest::Accept { sub_op, .. }
            | FriendRequest::Refuse { sub_op, .. }
            | FriendRequest::ById { sub_op, .. }
            | FriendRequest::ByIdNamed { sub_op, .. }
            | FriendRequest::Rearrange { sub_op, .. }
            | FriendRequest::Unknown { sub_op } => *sub_op,
            FriendRequest::Groups { .. } => REQUEST_GROUPS,
        }
    }

    /// Parse the body **after** the opcode.
    ///
    /// ```
    /// use net::friends::{FriendRequest};
    /// // The captured body, byte for byte: world-ch0.log 01:35:06.649.
    /// let body = [
    ///     0x01, 0x04, 0x00, b'W', b'i', b's', b'p',
    ///     0x0d, 0x00, b'D', b'e', b'f', b'a', b'u', b'l', b't', b' ', b'G', b'r', b'o', b'u', b'p',
    ///     0x00, 0x00, 0x01, 0x00, 0x00,
    /// ];
    /// assert_eq!(
    ///     FriendRequest::parse(&body).unwrap(),
    ///     FriendRequest::Add {
    ///         name: "Wisp".to_string(),
    ///         group: "Default Group".to_string(),
    ///         note: String::new(),
    ///         flag: 1,
    ///     },
    /// );
    /// ```
    pub fn parse(body: &[u8]) -> Result<Self> {
        let mut r = PacketReader::new(body);
        let sub_op = r.u8()?;
        Ok(match sub_op {
            REQUEST_ADD => FriendRequest::Add {
                name: r.str()?,
                group: r.str()?,
                note: r.str().unwrap_or_default(),
                flag: r.u8().unwrap_or(0),
            },
            REQUEST_ACCEPT | REQUEST_ACCEPT_ACCOUNT => {
                FriendRequest::Accept { sub_op, character_id: r.u32()? }
            }
            // The refuse carries a flag byte and sometimes a string; neither changes what it
            // means, and the id is the only field this server acts on.
            REQUEST_REFUSE | REQUEST_REFUSE_ACCOUNT => {
                FriendRequest::Refuse { sub_op, character_id: r.u32()? }
            }
            0x0B | 0x12 | 0x13 => {
                FriendRequest::ById { sub_op, character_id: r.u32()? }
            }
            4 | 5 => FriendRequest::ByIdNamed {
                sub_op,
                character_id: r.u32()?,
                name: r.str().unwrap_or_default(),
            },
            0x0C => {
                let _flag = r.u8()?;
                FriendRequest::Rearrange { sub_op, character_id: r.u32()? }
            }
            REQUEST_GROUPS => {
                let n = r.u32()?;
                let mut groups = Vec::new();
                // The count is the client's; a body that stops early ends the list rather
                // than failing the packet - the request still has to be answered.
                for _ in 0..n.min(64) {
                    let (Ok(index), Ok(name)) = (r.u8(), r.str()) else { break };
                    groups.push((index, name));
                }
                FriendRequest::Groups { groups }
            }
            other => FriendRequest::Unknown { sub_op: other },
        })
    }
}

/// A body with no sub-op byte at all.
pub fn require_a_sub_op(body: &[u8]) -> Result<()> {
    if body.is_empty() {
        return Err(NetError::PacketUnderflow { offset: 0, need: 1, have: 0 });
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// 0x00A7 - the answers
// ---------------------------------------------------------------------------------------

/// A reply that is only a sentence: the case sets a string id and falls into one shared
/// "put it in the chat log" tail, so the whole body is the sub-op byte. **[L]**
///
/// The text beside each name is this client's own, decrypted from its string table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FriendNotice {
    /// 0x1C - "Your buddy list is full."
    YourListIsFull = 0x1C,
    /// 0x1D - "The user's buddy list is full"
    TheirListIsFull = 0x1D,
    /// 0x1E - "That character is already registered as your buddy."
    AlreadyYourBuddy = 0x1E,
    /// 0x1F - "Account buddy request already sent."
    RequestAlreadySent = 0x1F,
    /// 0x20 - "That player is waiting to be added as a buddy."
    TheyAreWaiting = 0x20,
    /// 0x21 - "You can't enter yourself as your buddy."
    NotYourself = 0x21,
    /// 0x22 - "Gamemaster is not available as a buddy."
    NotAGameMaster = 0x22,
    /// 0x23 - "That character is not registered."
    NoSuchCharacter = 0x23,
    /// 0x25 - "You are still waiting to be added as a buddy."
    YouAreWaiting = 0x25,
    /// 0x2A - "The request to add a Friend has been canceled."
    Cancelled = 0x2A,
    /// 0x30 - "The request was denied due to an unknown error."
    UnknownError = 0x30,
}

impl FriendNotice {
    /// The sub-op byte, which is the whole body.
    pub fn sub_op(self) -> u8 {
        self as u8
    }
}

/// Sub-op 0x19: the manager's `{id, name}` **name cache**, replacing whatever it held.
///
/// **[L]** on the shape - the case clears the list and then reads `u32 count` and
/// `count x { u32, str }`.
///
/// **It is not the list the window draws, and it was a mistake to use it as one (2026-09-22).**
/// The arm clears and refills the `std::map` at `manager+0x18` (size at `+0x20`), which is an
/// id-to-name lookup table. The rows the window strides over live in the three parallel arrays
/// at `manager+0x00`, `+0x08` and `+0x10`, and [`friend_records`] is what fills those.
///
/// **Correction, same day:** this comment first said `0x19` *empties the record array*. It does
/// not - it empties its own map, and the record arrays are untouched. The order this server
/// sends them in is therefore not load-bearing; it was kept because a name cache that arrives
/// before the rows it names cannot be stale.
///
/// ```
/// use net::friends::friend_list;
/// let body = friend_list(&[(215, "Wisp".to_string())]);
/// assert_eq!(body[0], 0x19);
/// assert_eq!(&body[1..5], &1u32.to_le_bytes());
/// assert_eq!(&body[5..9], &215u32.to_le_bytes());
/// assert_eq!(&body[9..11], &4u16.to_le_bytes());
/// assert_eq!(&body[11..15], b"Wisp");
/// ```
pub fn friend_list(rows: &[(u32, String)]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(0x19);
    w.u32(rows.len() as u32);
    for (id, name) in rows {
        w.u32(*id);
        w.str(name);
    }
    w.into_vec()
}

/// Sub-op 0x1A: **the client's own "Friend request from <name>" popup** - the same balloon
/// the party, chat and trade invites use.
///
/// The owner, 2026-09-22: *"When someone sends a buddy request, there should not be any chat
/// commands. Please use the client's built in UI elements, there should be one similar pop up
/// just like the party invitation, chat invitation, or trade invitation."*
///
/// # How the balloon was found
///
/// String `0x0438` is *"Friend request from"*, the exact parallel of the trade invite's
/// caption. `FindConstArgCalls` on the string resolver puts it in `FUN_1418099f0`, which draws
/// whichever balloon `balloon+0x300` selects - **kind `0x0E` or `0x0F`** raise this one. The
/// dedicated setter for `0x0E` is `FUN_14180e3d0` (one of the 31 writers of that field, 30 of
/// which store a literal), and its callers are `FUN_142dec8f0` and **`FUN_142defd40` - the
/// `0x00A7` handler**, at the arm for sub-op `0x1A`. All **[L]**.
///
/// # The body
///
/// ```text
/// u8   flag      0 raises balloon kind 0x0E; non-zero the account-friend kind 0x0F
/// u32  echo      -> balloon+0x328, and the ONLY field the answer carries back
/// u32  id        looked up locally (FUN_142d01050) the way the trade invite's id is
/// str  name      -> balloon+800, the name the caption is completed with
/// u32  a         -> balloon+0x338
/// u32  b
/// u32  c
/// rec  FRIEND_ENTRY_LEN bytes - the requester, appended to the manager's own array
/// ```
///
/// **The record is not optional, and leaving it off killed the client** (2026-09-22). The arm
/// appends a row to the manager and reads it straight off this packet; the seven fields above
/// are exactly 28 bytes, so a body that stops there is consumed to the byte and the next read
/// throws `0x26`. See [`FRIEND_ENTRY_LEN`].
///
/// **The server chooses what comes back**: Yes sends [`REQUEST_ACCEPT`] with `echo`, No sends
/// [`REQUEST_REFUSE`] with the same number. This server puts the requester's character id
/// there, so the answer identifies the pairing with nothing to store.
///
/// ```
/// use net::friends::{friend_request_popup, FriendRecord, FRIEND_ENTRY_LEN};
/// let rec = FriendRecord::request(214, "Tester2", "Default Group");
/// let body = friend_request_popup(214, &rec);
/// assert_eq!(body[0], 0x1A);
/// assert_eq!(body[1], 0);                            // kind 0x0E, whose Yes is sub-op 2
/// assert_eq!(&body[2..6], &214u32.to_le_bytes());     // echo
/// // 1 + 1 + 4 + 4 + (2 + 7) + 12, and then the record that used to be missing.
/// assert_eq!(body.len(), 31 + FRIEND_ENTRY_LEN);
/// ```
pub fn friend_request_popup(echo: u32, record: &FriendRecord) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(0x1A);
    w.u8(0); //                 flag: the ordinary friend balloon, kind 0x0E
    w.u32(echo);
    w.u32(record.character_id);
    w.str(&record.name);
    w.u32(0);
    w.u32(0);
    w.u32(0);
    w.bytes(&record.encode());
    w.into_vec()
}

/// `rec[0x145]`: the status word `0x2D` pairs with the channel. **[I]** on the values.
pub const STATUS_OFFLINE: u32 = 0;
/// See [`STATUS_OFFLINE`].
pub const STATUS_ONLINE: u32 = 1;

/// Sub-op 0x2D: **a friend logged in, logged out or changed channel.**
///
/// The owner, 2026-09-22, three reports that are all this one packet: *"Tester2 was not able to
/// check the owner's current map location"*, *"once the owner logs off, the buddy list also remains
/// showing the owner is still online"*, and *"when Tester2 logs in after the owner, the owner was not
/// informed of the fact that Tester2 has logged in."*
///
/// ```text
/// u32  characterId
/// u32  accountId      only consulted when `account` is set
/// u8   status    -> rec+0x145
/// u32  channel   -> rec+0x12
/// u8   account        1 = look the row up by accountId, AND a detail block follows
/// u8   announce       1 = say "[Friend] %s has logged in." (string 0x03EE)
/// ```
///
/// **The arm only does anything when something changed.** It reads the row's current
/// `rec+0x12` and `rec+0x145`, and if both already equal what this packet carries it frees
/// the string and returns - no write, no line. So the announcement rides a *transition*,
/// which is why this server leaves `0x145` at zero in [`friend_records`]: a list followed by
/// a login notify is then always a change. **[L]**, `FUN_142defd40` case `0x2d`.
///
/// **The row is found by `FUN_142debab0`**, which matches `rec+0x00` against `characterId`
/// when `account` is clear, and `rec+0x28` against `accountId` - but only for rows whose
/// `rec[0x11]` is in `5..=8` - when it is set. That is two more confirmations of the
/// inferred layout, and it is why this server sends `account = 0`.
///
/// **What this does not carry:** the detail block (`str name`, then three `u32` into
/// `rec+0x139`, `+0x13D` and `+0x141`, each with `-1` meaning "leave alone") is read **only**
/// when `account` is set, so it cannot be used to fill the window's blank JOB and LV columns
/// for an ordinary friend. Those live in the record itself, and which of the three words is
/// which is still **[I]**.
///
/// ```
/// use net::friends::{friend_status, STATUS_ONLINE};
/// let b = friend_status(215, 215, STATUS_ONLINE, 0, true);
/// assert_eq!(b[0], 0x2D);
/// assert_eq!(&b[1..5], &215u32.to_le_bytes());
/// assert_eq!(b[9], 1);            // status
/// assert_eq!(&b[10..14], &0u32.to_le_bytes());
/// assert_eq!(b[14], 0);           // not an account friend, so no detail block follows
/// assert_eq!(b[15], 1);           // and say it out loud
/// assert_eq!(b.len(), 16);
/// ```
pub fn friend_status(character_id: u32, account_id: u32, status: u32, channel: u32, announce: bool) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(0x2D);
    w.u32(character_id);
    w.u32(account_id);
    w.u8(status as u8);
    w.u32(channel);
    w.u8(0); //                 not an account friend: match on the character id, no detail
    w.u8(u8::from(announce));
    w.into_vec()
}

/// Sub-op 0x1B: *"Buddy request successfully sent to %s."* - the requester's own line. **[L]**
pub fn friend_request_sent(name: &str) -> Vec<u8> {
    one_string(0x1B, name)
}

/// Sub-op 0x32: *"%s has declined the friend request."* **[L]**
pub fn friend_declined(name: &str) -> Vec<u8> {
    one_string(0x32, name)
}

/// Sub-op 0x36: *"%s is currently not accepting any friends."* **[L]**
pub fn friend_not_accepting(name: &str) -> Vec<u8> {
    one_string(0x36, name)
}

/// One of the sentence-only replies: the sub-op byte and nothing else.
pub fn friend_notice(notice: FriendNotice) -> Vec<u8> {
    vec![notice.sub_op()]
}

fn one_string(sub_op: u8, name: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(sub_op);
    w.str(name);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The captured add, and the shapes of the sub-ops whose meaning is still [I] - the
    /// point of keeping the number is that the handler can log it.
    #[test]
    fn the_request_shapes_parse_and_keep_their_sub_op() {
        let add = FriendRequest::parse(&[
            1, 4, 0, b'W', b'i', b's', b'p', 1, 0, b'G', 0, 0, 1,
        ])
        .unwrap();
        assert_eq!(add.sub_op(), 1);
        assert!(matches!(&add, FriendRequest::Add { name, group, flag: 1, .. } if name == "Wisp" && group == "G"));

        let mut by_id = vec![0x12u8];
        by_id.extend_from_slice(&215u32.to_le_bytes());
        assert_eq!(
            FriendRequest::parse(&by_id).unwrap(),
            FriendRequest::ById { sub_op: 0x12, character_id: 215 }
        );

        let mut named = vec![5u8];
        named.extend_from_slice(&215u32.to_le_bytes());
        named.extend_from_slice(&2u16.to_le_bytes());
        named.extend_from_slice(b"Hi");
        assert_eq!(
            FriendRequest::parse(&named).unwrap(),
            FriendRequest::ByIdNamed { sub_op: 5, character_id: 215, name: "Hi".to_string() }
        );

        // The popup's two buttons: Yes is 2 (3 from the account-friend balloon), No is 6
        // (7), and both carry back the u32 the popup was built with.
        let mut yes = vec![REQUEST_ACCEPT];
        yes.extend_from_slice(&215u32.to_le_bytes());
        assert_eq!(
            FriendRequest::parse(&yes).unwrap(),
            FriendRequest::Accept { sub_op: 2, character_id: 215 }
        );
        let mut no = vec![REQUEST_REFUSE];
        no.extend_from_slice(&215u32.to_le_bytes());
        no.push(1); // the flag byte the refuse carries, which changes nothing
        assert_eq!(
            FriendRequest::parse(&no).unwrap(),
            FriendRequest::Refuse { sub_op: 6, character_id: 215 }
        );
        assert_eq!(FriendRequest::parse(&[7, 215, 0, 0, 0]).unwrap().sub_op(), 7);

        // The group list, and a truncated one: the list ends, the packet still parses,
        // because it still has to be answered.
        let mut groups = vec![0x14u8];
        groups.extend_from_slice(&2u32.to_le_bytes());
        groups.push(0);
        groups.extend_from_slice(&1u16.to_le_bytes());
        groups.push(b'A');
        assert_eq!(
            FriendRequest::parse(&groups).unwrap(),
            FriendRequest::Groups { groups: vec![(0, "A".to_string())] }
        );

        assert_eq!(FriendRequest::parse(&[0x40]).unwrap(), FriendRequest::Unknown { sub_op: 0x40 });
        assert!(FriendRequest::parse(&[]).is_err());
    }

    /// Every reply is its sub-op byte first, and the sentence-only ones are nothing else.
    #[test]
    fn the_replies_lead_with_their_sub_op() {
        assert_eq!(friend_notice(FriendNotice::NotYourself), vec![0x21]);
        assert_eq!(friend_notice(FriendNotice::NoSuchCharacter), vec![0x23]);
        assert_eq!(friend_notice(FriendNotice::AlreadyYourBuddy), vec![0x1E]);
        assert_eq!(friend_request_sent("Wisp"), vec![0x1B, 4, 0, b'W', b'i', b's', b'p']);
        assert_eq!(friend_declined("Wisp")[0], 0x32);
        assert_eq!(friend_not_accepting("Wisp")[0], 0x36);
        // An empty list is a real answer: it is how a list is cleared.
        assert_eq!(friend_list(&[]), vec![0x19, 0, 0, 0, 0]);
        // The entry length, pinned against what `FUN_1402d4870` reads.
        assert_eq!(FRIEND_ENTRY_LEN, 329);
        // The popup: sub-op, flag 0 (balloon kind 0x0E), the echoed id, the looked-up id,
        // the name, three zero words - **and the record, whose absence killed the client**.
        let record = FriendRecord::request(214, "Tester2", "Default Group");
        let popup = friend_request_popup(214, &record);
        assert_eq!(popup[0], 0x1A);
        assert_eq!(popup[1], 0);
        assert_eq!(&popup[2..6], &214u32.to_le_bytes());
        assert_eq!(&popup[6..10], &214u32.to_le_bytes());
        assert_eq!(&popup[10..12], &7u16.to_le_bytes());
        assert_eq!(&popup[12..19], b"Tester2");
        assert_eq!(popup.len(), 31 + FRIEND_ENTRY_LEN);
        assert_eq!(&popup[31..], &record.encode()[..]);
    }

    /// **The record's three measured offsets, and the fields that follow them.**
    ///
    /// The client's own reads are the only authority for `0`, `4` and `0x11`; the rest comes
    /// from the reference tree and is pinned here so a change to it is a deliberate one.
    #[test]
    fn the_329_byte_record_puts_every_field_where_the_client_reads_it() {
        let rec = FriendRecord {
            character_id: 215,
            name: "Wisp".to_string(),
            flag: FLAG_ONLINE,
            channel: 1,
            group: "Buddies".to_string(),
            account_id: 9,
            nickname: "E".to_string(),
            memo: "hi".to_string(),
            level: 21,
            job: 300,
        }
        .encode();
        assert_eq!(rec.len(), FRIEND_ENTRY_LEN);
        assert_eq!(&rec[0x00..0x04], &215u32.to_le_bytes()); // [L]
        assert_eq!(&rec[0x04..0x08], b"Wisp"); //               [L]
        assert_eq!(&rec[0x08..0x11], &[0u8; 9]); //             the rest of the 13 bytes
        assert_eq!(rec[0x11], FLAG_ONLINE); //                  [L]
        assert_eq!(&rec[0x12..0x16], &1u32.to_le_bytes());
        assert_eq!(&rec[0x16..0x1D], b"Buddies");
        assert_eq!(rec[0x27], 0); //                            mobile
        assert_eq!(&rec[0x28..0x2C], &9u32.to_le_bytes());
        assert_eq!(&rec[0x2C..0x2D], b"E");
        assert_eq!(&rec[0x39..0x3B], b"hi");
        // Where the row builder FUN_1411be0a0 reads them - [L], not the reference's guess.
        assert_eq!(&rec[0x139..0x13D], &21u32.to_le_bytes()); // LV
        assert_eq!(&rec[0x13D..0x141], &300u32.to_le_bytes()); // JOB, into the job-name lookup
        assert_eq!(&rec[0x141..0x145], &[0u8; 4]); //            subJob
        assert_eq!(&rec[0x145..0x149], &[0u8; 4]); //            the status word 0x2D owns

        // **A name longer than the field cannot reach the flag byte.** 13 bytes wide means 12
        // characters and a terminator, and the flag is what the client tests for `1`.
        let long = FriendRecord::friend(1, "AbcdefghijklmnopQ", "g", None, 1, 0).encode();
        assert_eq!(&long[0x04..0x10], b"Abcdefghijkl");
        assert_eq!(long[0x10], 0);
        assert_eq!(long[0x11], FLAG_OFFLINE);

        // Two records in one list, back to back with no separator: the client reads the whole
        // block in one call and strides 0x149.
        let body = friend_records(&[FriendRecord::request(1, "A", "g"), FriendRecord::friend(2, "B", "g", Some(0), 1, 0)]);
        assert_eq!(body[0], 0x15);
        assert_eq!(&body[1..5], &2u32.to_le_bytes());
        assert_eq!(body.len(), 5 + 2 * FRIEND_ENTRY_LEN);
        assert_eq!(body[5 + 0x11], FLAG_REQUEST);
        assert_eq!(body[5 + FRIEND_ENTRY_LEN + 0x11], FLAG_ONLINE);
        assert_eq!(friend_records(&[]), vec![0x15, 0, 0, 0, 0]);
    }
}
