//! Inbound opcodes, recovered from the running client.
//!
//! The client's *outbound* opcodes are readable statically - every `FUN_1406ed520(buf, op)`
//! call site names one, which is where `docs/opcodes.md` comes from. **Inbound opcodes are
//! not.** The dispatcher `FUN_1415d60e0` tail-jumps into the Themida VM, and the handler
//! addresses appear nowhere as data: not in the image, and not in a gigabyte of live
//! memory. The mapping exists only inside the VM.
//!
//! So each of these is measured, not read: the client is made to dispatch a packet whose
//! opcode we vary, and an observable effect names the number. See `crates/grap-stub`.
//! Anything added here should say how it was established, because none of it can be
//! re-derived by reading the binary.

/// Reply to the client's `0x00A1` `Data.wz` request.
///
/// # What it does
///
/// On connect the client hashes `Data.wz`, sends `0x00A1` carrying that `u32`, and then
/// blocks in `recv` **on its UI thread** inside `FUN_1415e7090` until a dispatched packet
/// sets `conn+0x150`. Only `FUN_1415e5c20` does that, and this is the opcode that reaches
/// it. Until it arrives the client shows a blank, non-responding window - it never gets
/// as far as a login screen, which is why hunting for a "login" opcode was looking in the
/// wrong place entirely.
///
/// # Body
///
/// A **zigzag varint** length (`FUN_1406efcc0`: accumulate low 7 bits while the high bit
/// is set, then `(n >> 1) ^ -(n & 1)`), meaning:
///
/// | length | client does |
/// |---|---|
/// | `0` | nothing to patch - sets the flag and carries on |
/// | `> 0` | expects that many bytes, in 64 KB chunks, then writes `Data.wz` |
/// | `< 0` | deletes `Data.wz` and carries on |
///
/// So the minimal reply that releases the client is this opcode plus a single `0x00`.
///
/// # How it was established
///
/// By walking the opcode space inside the client: 0x0000 upward, rewriting the opcode of
/// one captured packet and re-dispatching it, watching `conn+0x150`. Found at 0x0032 after
/// two faults, and confirmed by the client reaching its login screen.
pub const DATA_WZ_PATCH: u16 = 0x0032;

/// The client's request that [`DATA_WZ_PATCH`] answers - outbound, so statically readable.
pub const CLIENT_DATA_WZ_REQUEST: u16 = 0x00A1;

/// Reply to the client's body-less `0x0080` login request.
///
/// # How it was established
///
/// Unlike [`DATA_WZ_PATCH`] this one was *read*, not walked. The Themida-virtualised
/// dispatcher only routes: it hands a stage its opcode, and the stage's `OnPacket` is
/// ordinary code. `FUN_141b25f30` is the login stage's, a plain `switch` naming every
/// login-stage opcode at once, and `case 0x10` calls `FUN_141b307b0`. Confirmed at runtime
/// by an `int3` watch, which saw that function entered while dispatching `0x0010`.
///
/// # Body
///
/// ```text
/// u8   result          0 = success; anything else raises a dialog
/// str  message         u16 length, then bytes
/// // when result == 0, the client goes on to read:
/// u8, 8 bytes, u32, u32, 4B, 4B, 4B, u32, u8, then two further sub-readers
/// // when result == 0x83: two more u32
/// ```
///
/// **Non-zero result codes are error message IDs**, resolved by `FUN_141803cd0` - see
/// `docs/client-messages.md`. `0x65` is 101, "You have been disconnected from the login
/// server", which is precisely the dialog it produces. `0x65` and `0x67` are *not* success
/// despite reaching a "proceed"-looking branch; that branch only re-sends `0x0080`.
pub const LOGIN_RESULT: u16 = 0x0010;

/// The client's login request that [`LOGIN_RESULT`] answers. Sent with an **empty body**,
/// and without any button press - the login form is vestigial.
pub const CLIENT_LOGIN_REQUEST: u16 = 0x0080;

/// `result == 0`. The client treats every other value as an error message ID.
pub const LOGIN_OK: u8 = 0;

/// The world list. One packet per world, then one whose `worldId` closes the list.
///
/// # How it was established
///
/// Read from the login stage's switch (`case 0xb`), then from the handler itself. **Take
/// the mode-5 fork**: `FUN_141b2fac0` opens with `if (session+0x68 == 5) { FUN_141b31ff0();
/// return; }`, and our client is mode 5, so `FUN_141b31ff0` is the live handler. Decoding
/// the outer function instead decodes something this client never runs.
///
/// # Why it is needed
///
/// Two reasons, and the second is the one that was missed for a long time.
///
/// 1. The login result makes the client look up its world in the list at `stage+0x100`
///    (`FUN_141b2c7c0`). Only this packet appends to it - `FUN_141b44520(stage+0x100, -1)`.
/// 2. **It enables the Login button.** The handler sets `stage+0x108` one line above the
///    append, and the `ClassicIntro` tick `FUN_14112a720` enables the control named
///    `"login"` only when that byte is non-zero. The screen builder creates the button
///    disabled, so without this packet it never becomes clickable - which is exactly the
///    "invalid session" state the owner describes.
///
/// Only a real world entry does either; the terminator returns before both.
///
/// # Body
///
/// ```text
/// u8   worldId          high bit set => terminator: u8 flag, u8 hasNotice, [notice]
/// str  worldName
/// u8   flag
/// str  eventDescription
/// u8   flag
/// u8   channelCount
///      repeat: str channelName, u32, u8, u8, u8, u8
/// u16  balloonCount
///      repeat: u16 x, u16 y, str message
/// u32
/// u8   hasExtra         non-zero pulls in a further sub-record
/// ```
pub const WORLD_LIST: u16 = 0x000B;

/// The `worldId` that ends a [`WORLD_LIST`] run. Any value with the high bit set works -
/// the client's test is `(char) worldId < 0`.
pub const WORLD_LIST_END: u8 = 0xFF;

/// The account record: who the client believes it is logged in as.
///
/// # How it was established
///
/// Read, like [`WORLD_LIST`], from the login stage's switch: `case 0` calls
/// `FUN_141b2dd00`. Found by working backwards from the screen instead of forwards from
/// the wire - `FUN_14112a720`, the `ClassicIntro` tick, renders the account object's
/// `+0x22f8` string whenever it is non-empty, and the *only* writer of that field is
/// `FUN_142cb8370`, whose only two callers are the handlers for this opcode and
/// [`ACCOUNT_INFO_ALT`]. So the name on the login screen cannot be computed by the client;
/// it has to be told.
///
/// **No mode fork.** Unlike [`WORLD_LIST`], `FUN_141b2dd00` does not branch on
/// `session+0x68 == 5`, so this is the live handler whether or not the mode patch is on.
///
/// # Body
///
/// ```text
/// u8   result           0 = success, and the only value that reaches the fields below
/// str  message          shown in the failure dialogs
/// u8   verifyState      0 or 1 proceed; 2 or 3 raise "accountHasNotBeenVerified"
/// u32                   read and discarded
/// // everything past here is read only when the result gate passes:
/// str  loginName        -> account+0x48
/// u64                   read and discarded
/// u32  accountId
/// u8
/// u32  flags            bit 21 triggers FUN_140d2d4e0 - keep it clear
/// u32, u8, str, u32     the u32 lands in account+0x22b8
/// u8, u8, 8B, 8B        the two eight-byte fields are FILETIMEs
/// u32, str, u32         the last u32 lands in account+0x28e0
/// u8                    read and discarded
/// u8                    -> stage+0x1a4
/// u8                    -> stage+0xdc
/// 8B                    -> account+0x2324
/// str  accountName      -> account+0x22f8, the string the login screen displays
/// ```
///
/// The result gate is `FUN_141b267c0(stage, result, 0, message)`, the same function that
/// turns a non-zero [`LOGIN_RESULT`] into a named dialog. It returns "proceed" for result
/// `0` **and** result `12`; every other value raises a notice and stops before the fields.
pub const ACCOUNT_INFO: u16 = 0x0000;

/// The shorter sibling of [`ACCOUNT_INFO`], `case 0x12` -> `FUN_141b2ee90`.
///
/// Same shape - `u8 result`, `str message`, the same result gate, and the same
/// `FUN_142cb8370` account-name write at the end - but it omits the `verifyState` byte and
/// the account-blocked sub-record, and its field list in between differs. Kept named
/// because it is the fallback if [`ACCOUNT_INFO`] turns out to be the wrong one of the two.
pub const ACCOUNT_INFO_ALT: u16 = 0x0012;

/// A MapleStory string: `u16` length, then the bytes. Mirrors `FUN_1406e9050`.
fn put_str(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&(s.len() as u16).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

/// One [`WORLD_LIST`] entry: a world with `channels` unnamed channels and no notices.
///
/// Deliberately minimal - every optional count is zero - because the point is to give
/// `FUN_141b2c7c0` a world to find, not to furnish a realistic server list.
pub fn world_list_entry(world_id: u8, name: &str, channels: u8) -> Vec<u8> {
    let mut out = vec![world_id];
    put_str(&mut out, name);
    out.push(0);
    put_str(&mut out, "");
    out.push(0);
    out.push(channels);
    for i in 0..channels {
        put_str(&mut out, &format!("{name}-{i}"));
        out.extend_from_slice(&0u32.to_le_bytes()); // capacity
        out.extend_from_slice(&[0, 0, 0, 0]);
    }
    out.extend_from_slice(&0u16.to_le_bytes()); // balloonCount
    out.extend_from_slice(&0u32.to_le_bytes());
    out.push(0); // hasExtra
    out
}

/// The packet that closes a [`WORLD_LIST`] run, with no notice.
pub fn world_list_end() -> Vec<u8> {
    vec![WORLD_LIST_END, 0, 0]
}

/// A successful [`ACCOUNT_INFO`] body: the client is logged in, and this is its name.
///
/// Every field the client does not display is zero. That is not laziness - a zero `flags`
/// keeps bit 21 clear, and the two `FILETIME` fields are handed to a conversion callback
/// whose behaviour on arbitrary bytes we have not established. The point of this packet is
/// the last string.
///
/// `login_name` is the `+0x48` field the client sends back in later requests;
/// `account_name` is the `+0x22f8` one that appears on the login screen.
pub fn account_info(login_name: &str, account_name: &str) -> Vec<u8> {
    let mut out = vec![LOGIN_OK];
    put_str(&mut out, ""); // message
    out.push(0); // verifyState: proceed
    out.extend_from_slice(&0u32.to_le_bytes()); // discarded

    put_str(&mut out, login_name); // -> account+0x48
    out.extend_from_slice(&0u64.to_le_bytes()); // discarded
    out.extend_from_slice(&0u32.to_le_bytes()); // accountId
    out.push(0);
    out.extend_from_slice(&0u32.to_le_bytes()); // flags - bit 21 must stay clear
    out.extend_from_slice(&0u32.to_le_bytes());
    out.push(0);
    put_str(&mut out, "");
    out.extend_from_slice(&0u32.to_le_bytes()); // -> account+0x22b8
    out.push(0);
    out.push(0);
    out.extend_from_slice(&0u64.to_le_bytes()); // FILETIME
    out.extend_from_slice(&0u64.to_le_bytes()); // FILETIME
    out.extend_from_slice(&0u32.to_le_bytes());
    put_str(&mut out, "");
    out.extend_from_slice(&0u32.to_le_bytes()); // -> account+0x28e0

    out.push(0); // discarded
    out.push(0); // -> stage+0x1a4
    out.push(0); // -> stage+0xdc
    out.extend_from_slice(&0u64.to_le_bytes()); // -> account+0x2324
    put_str(&mut out, account_name); // -> account+0x22f8, shown on screen
    out
}

/// Encode a zigzag varint, the length format [`DATA_WZ_PATCH`] expects.
///
/// Mirrors `FUN_1406efcc0` in the client: zigzag so the sign survives, then 7 bits per
/// byte with the high bit marking "more follows".
pub fn zigzag_varint(value: i32) -> Vec<u8> {
    let mut n = ((value << 1) ^ (value >> 31)) as u32;
    let mut out = Vec::new();
    loop {
        let byte = (n & 0x7F) as u8;
        n >>= 7;
        if n == 0 {
            out.push(byte);
            return out;
        }
        out.push(byte | 0x80);
    }
}

/// The whole body of a "your `Data.wz` is current" reply.
pub fn data_wz_up_to_date() -> Vec<u8> {
    zigzag_varint(0)
}

/// The client leaving the world it is in - the "Choose another world" button.
///
/// Handler `FUN_141b3bfd0`. The body is empty. **It must be answered**: unanswered, the
/// client blocks its entire UI on the reply - no dialog can be dismissed and even the quit
/// prompt stops responding, which reads as a crash and is not one. The reply it wants is
/// the world list again, exactly as at login.
///
/// Measured 2026-08-17: answering it with a world entry and terminator restored the UI.
pub const CLIENT_LEAVE_WORLD_REQUEST: u16 = 0x0082;

/// The client's request to *enter* character creation - the second-password check.
///
/// Sent by `FUN_141b282d0` when "Create a character" is clicked and its gate opens. The
/// body is a single one-character string holding `DAT_143275e10`, which is `.` - a
/// placeholder, because MapleStory Classic World never used a second password (the owner).
///
/// Measured on the wire 2026-08-17: `01 00 2e`, five times for five clicks.
pub const CLIENT_ENTER_CREATION_REQUEST: u16 = 0x00A8;

/// The reply to [`CLIENT_ENTER_CREATION_REQUEST`]: may the client open character creation?
///
/// Handler `FUN_141b39490`, a `switch` on the first byte. `0` reads one more byte and calls
/// `FUN_141b3f050(stage, 5, 0x14a)` - the transition to the NewChar screen. Every other
/// named case is a second-password failure, which is what identifies this exchange:
///
/// | code | notice |
/// |---|---|
/// | `0x00` | none - **proceed to character creation** |
/// | `0x14` | `incorrectPIC` |
/// | `0x39` | `incorrectPICWarningOverCount` |
/// | `0x3A` | `incorrectPICCloseByOverCount`, and back to the previous screen |
/// | `0x45` | the antimacro (captcha) flow |
///
/// # Body
///
/// ```text
/// u8   result      0 = proceed
/// u8               read and discarded on the success path
/// ```
pub const ENTER_CREATION_RESULT: u16 = 0x05F4;

/// A "yes, open character creation" body.
///
/// Two bytes, because the success path reads a second one before transitioning and the
/// readers throw on underrun.
pub fn enter_creation_permitted() -> Vec<u8> {
    vec![0, 0]
}

/// The name-check result: whether the name the client asked about may be used.
///
/// Answers the client's [`CLIENT_CHECK_NAME_REQUEST`]. Handler `FUN_141b33f30`, whose whole
/// body is a `switch` on the result byte - so the codes below are read off the client, not
/// guessed. The handler opens by clearing `stage+0xd4`, the same flag the request builder
/// sets, which is what ties the two opcodes together beyond mere adjacency.
///
/// # Body
///
/// ```text
/// str  name        echoed back
/// u8   result      0 = available; see the constants below
/// ```
pub const CHECK_NAME_RESULT: u16 = 0x0014;

/// The client's name-check request, sent from the character creation screen.
///
/// Built by `FUN_141b28950`, which refuses to send unless the screen is `5` (NewChar), no
/// request is already in flight, and the four ability points at `stage+0x220` sum to `25` -
/// otherwise it raises `useAllAP` locally. It also validates the name itself and raises
/// `cannotUseThisName` without sending, so a locally-invalid name never reaches us.
pub const CLIENT_CHECK_NAME_REQUEST: u16 = 0x0081;

/// [`CHECK_NAME_RESULT`] code `0`: the name is free. Raises the `availableName` confirm.
pub const NAME_AVAILABLE: u8 = 0;

/// [`CHECK_NAME_RESULT`] code `0x7A`: `alreadyUsedName`.
pub const NAME_ALREADY_USED: u8 = 0x7A;

/// [`CHECK_NAME_RESULT`] codes `0x79` and `0x7B`: `cannotUseThisName`.
pub const NAME_NOT_ALLOWED: u8 = 0x79;

/// The client's create-character request. **Measured, not read.**
///
/// Its builder is Themida-virtualised - `FUN_141122420`'s OK button calls `FUN_141b3fb10`,
/// which tail-jumps to `FUN_141b2cf30`, which `JMP`s into `.themida` - so this opcode
/// appears in neither half of `research/msexe-send-opcodes.txt` and no static scan can
/// find it. It came off the wire on 2026-08-17, 101 bytes.
///
/// # Body, from a real capture
///
/// ```text
/// str   name              "Hello"
/// u32                     0
/// u32                     0xFFFFFFFF
/// u32   race              0
/// u16   subJob            0
/// u32   str, dex, int, luk    10, 4, 5, 6 - the roll, and it must total 25
/// u32   gender            0
/// u32   skin              2
/// u32   hair              30001
/// u32   itemCount         6, then that many (u32 slot, u32 itemId) pairs:
///                           1 face 20001, 2 hair 30000, 3 top 1040002,
///                           4 bottom 1060002, 5 shoes 1072002, 6 weapon 1302000
/// ```
///
/// The four stats are the same four the client keeps at `stage+0x220` and refuses to send
/// unless they sum to 25, which is what ties the roll on screen to these bytes.
pub const CLIENT_CREATE_CHARACTER_REQUEST: u16 = 0x008A;

/// What the client asked for in a [`CLIENT_CREATE_CHARACTER_REQUEST`].
///
/// Parsed rather than assumed: the first version of this reply sent a default character and
/// the new character came back **naked**, because every choice on the creation screen lives
/// in this packet and nothing was reading it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub race: u32,
    pub sub_job: u16,
    pub strength: u16,
    pub dexterity: u16,
    pub intelligence: u16,
    pub luck: u16,
    pub gender: u8,
    pub skin: u8,
    pub hair: u32,
    /// `(categorySlot, itemId)` exactly as sent - 1 face, 2 hair, 3 top, 4 bottom,
    /// 5 shoes, 6 weapon.
    pub items: Vec<(u32, u32)>,
}

/// The creation screen's category numbers. These are **not** avatar equipment slots; see
/// [`CreateCharacterRequest::character`].
const ITEM_FACE: u32 = 1;
const ITEM_HAIR: u32 = 2;
const ITEM_TOP: u32 = 3;
const ITEM_BOTTOM: u32 = 4;
const ITEM_SHOES: u32 = 5;
const ITEM_WEAPON: u32 = 6;

/// Reading side of a packet body, mirroring the client's readers closely enough that a
/// short body is an error rather than a panic.
struct Reader<'a> {
    body: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    // No u8 reader: every scalar in a create request is a u32 or u16, including the ones
    // that hold a byte's worth of meaning like gender and skin.
    fn u16(&mut self) -> Option<u16> {
        let b = self.body.get(self.at..self.at + 2)?;
        self.at += 2;
        Some(u16::from_le_bytes([b[0], b[1]]))
    }
    fn u32(&mut self) -> Option<u32> {
        let b = self.body.get(self.at..self.at + 4)?;
        self.at += 4;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
    fn str(&mut self) -> Option<String> {
        let n = self.u16()? as usize;
        let b = self.body.get(self.at..self.at + n)?;
        self.at += n;
        Some(String::from_utf8_lossy(b).into_owned())
    }
}

impl CreateCharacterRequest {
    /// Decode a `0x008A` body. Returns `None` on anything short or malformed.
    ///
    /// The layout is in [`CLIENT_CREATE_CHARACTER_REQUEST`]. It was measured, not read -
    /// the builder is inside the Themida VM - so this parser is written against one real
    /// capture and pinned to it by test.
    pub fn parse(body: &[u8]) -> Option<Self> {
        let mut r = Reader { body, at: 0 };
        let name = r.str()?;
        let _ = r.u32()?;
        let _ = r.u32()?;
        let race = r.u32()?;
        let sub_job = r.u16()?;
        let strength = r.u32()? as u16;
        let dexterity = r.u32()? as u16;
        let intelligence = r.u32()? as u16;
        let luck = r.u32()? as u16;
        let gender = r.u32()? as u8;
        let skin = r.u32()? as u8;
        let hair = r.u32()?;
        let count = r.u32()?;
        let mut items = Vec::new();
        for _ in 0..count {
            items.push((r.u32()?, r.u32()?));
        }
        Some(CreateCharacterRequest {
            name,
            race,
            sub_job,
            strength,
            dexterity,
            intelligence,
            luck,
            gender,
            skin,
            hair,
            items,
        })
    }

    fn item(&self, category: u32) -> Option<u32> {
        self.items.iter().find(|(c, _)| *c == category).map(|(_, id)| *id)
    }

    /// Turn the request into the character to send back.
    ///
    /// # The two slot numberings are different
    ///
    /// The request numbers its items by *creation category* - 3 is "top" because top is the
    /// third picker on the screen. The avatar look numbers them by **equipment slot**, and
    /// those are the classic MapleStory ones: 5 top, 6 bottom, 7 shoes, 11 weapon. Copying
    /// the request's numbers straight across would dress the character in the wrong slots,
    /// which renders as nothing at all.
    ///
    /// Face and hair are not equipment: they are fields of the stat block and of the look.
    pub fn character(&self, id: u32) -> Character {
        let mut equips = Vec::new();
        for (category, slot) in [
            (ITEM_TOP, 5u8),
            (ITEM_BOTTOM, 6),
            (ITEM_SHOES, 7),
            (ITEM_WEAPON, 11),
        ] {
            if let Some(item) = self.item(category) {
                equips.push((slot, item));
            }
        }
        Character {
            id,
            name: self.name.clone(),
            gender: self.gender,
            skin: self.skin,
            face: self.item(ITEM_FACE).unwrap_or(20000),
            // The request sends the hair twice: a bare `hair` field carrying the colour
            // variant, and category 2 carrying the base style. The variant is the one the
            // player picked, so it is the one that goes back.
            hair: if self.hair != 0 {
                self.hair
            } else {
                self.item(ITEM_HAIR).unwrap_or(30000)
            },
            strength: self.strength,
            dexterity: self.dexterity,
            intelligence: self.intelligence,
            luck: self.luck,
            equips,
            ..Character::default()
        }
    }
}

/// The character creation result.
///
/// Handler `FUN_141b36a10`. On success it decodes a full character record with
/// `FUN_1403094b0`, registers it, and calls `FUN_141b3f050(stage, 4, 0x14a)` - back to
/// character select with the new character in the list.
///
/// # Body
///
/// ```text
/// u8   result      0 = success
/// // when result == 0:
/// u32  worldId     must equal stage+0x1c0 or the handler returns having done nothing
/// ..   one character record
/// u8               read and discarded
/// ```
///
/// **Two corrections to what a reference encoder suggests.** The trailing byte is *not* a
/// "return to character select" switch in this client - `FUN_1406e8ae0(param_2)` reads it
/// and throws the value away, and the transition to screen 4 is unconditional. And the
/// `worldId` is a **gate**: if it does not match `stage+0x1c0` the handler returns
/// immediately, having registered nothing and shown nothing, which is indistinguishable
/// from the packet never arriving.
///
/// # Result codes, read from the handler's switch
///
/// | code | notice |
/// |---|---|
/// | `0x00` | success |
/// | `0x09` | `insufficientCharacterSlot` |
/// | `0x0A` | `loginTimeOut` |
/// | `0x63` | `unavailableClass` |
/// | `0x69` | `characterCreationRestrictedWorld`, and back to character select |
/// | other | `cannotProcessRequest` |
///
/// # This client does not enter the game world here
///
/// The owner recalls the live service dropping a new character straight into the starter map.
/// Whatever does that, it is not this handler: the success path ends at screen 4
/// unconditionally. So it has to be a *further* packet the server sends afterwards. The
/// candidate is inbound `0x0011` (`FUN_141b36f60`), which the opcode-name mapping in
/// `docs/character.md` calls `SelectCharacterResult` - the packet that would carry a
/// channel server address. Not investigated; noted so it is not rediscovered from scratch.
pub const CREATE_CHARACTER_RESULT: u16 = 0x0015;

/// The client's request to delete a character - the Delete button.
///
/// Builder `FUN_141b28750`, and it is **not virtualised**, so this was read rather than
/// captured. It raises `confirmDeleteCharacterPermanently`, and only if the player confirms
/// does it build the packet:
///
/// ```text
/// FUN_1406ed520(pkt, 0x8b)      // opcode
/// FUN_1406ed9d0(pkt, *charId)   // one u32: the selected character's id
/// FUN_1415d01c0(pkt)            // send
/// *(stage + 0xd4) = 1           // "delete in flight"
/// ```
///
/// **The in-flight flag is why an unanswered delete is worse than a slow one.** The
/// function returns early while `stage+0xd4 != 0`, and only [`DELETE_CHARACTER_RESULT`]
/// clears it - so a delete that is never answered disables the button for the rest of the
/// session, on top of the usual whole-UI freeze.
///
/// Found in `research/msexe-send-opcodes.txt`, which lists it against `FUN_141b28750` and
/// `FUN_141b2cb70`. Worth noting against the create request, which is *absent* from that
/// table because its builder is inside the Themida VM: absence there means virtualised,
/// not non-existent.
pub const CLIENT_DELETE_CHARACTER_REQUEST: u16 = 0x008B;

/// The delete result, `FUN_141b34970`.
///
/// ```text
/// u32  characterId
/// u8   result        0 = deleted
/// ```
///
/// **Correction:** this used to be documented as "a single `u32` character id". The handler
/// reads the id with `FUN_1406e8c20` and then a byte with `FUN_1406e8ae0`, and switches on
/// the byte. Sending four bytes only would underrun the second read, and the client's
/// readers throw on underrun.
///
/// # The result codes, and the trap in them
///
/// The switch names `6`, `9`, `10`, `0x10`, `0x12`, `0x14` and a **default**. The default
/// branch is the one that calls `FUN_14108d9b0(characterId)` - the removal. So an
/// unrecognised non-zero code **still deletes the character on screen**, which is the
/// opposite of what a refusal wants.
///
/// Use [`DELETE_FAILED`] to refuse. It is `6`, which raises `loginTroubleAskSupport` and
/// leaves the list alone.
pub const DELETE_CHARACTER_RESULT: u16 = 0x0016;

/// Delete succeeded; the client removes the character from its list.
pub const DELETE_OK: u8 = 0;

/// Refuse a delete. **Not any non-zero value** - see [`DELETE_CHARACTER_RESULT`], where
/// every code the switch does not name falls through to the branch that removes the
/// character anyway.
pub const DELETE_FAILED: u8 = 6;

/// Body of a [`DELETE_CHARACTER_RESULT`].
pub fn delete_character_result(character_id: u32, code: u8) -> Vec<u8> {
    let mut out = character_id.to_le_bytes().to_vec();
    out.push(code);
    out
}

/// The client's name field is a **fixed 13-byte block**, not a length-prefixed string.
/// `FUN_140302e30` reads it with `FUN_1406e9170(packet, record + 0xc, 0xd)`.
pub const CHARACTER_NAME_LEN: usize = 13;

/// One character, in the fields the client actually reads out of a record.
///
/// Everything the client stores but never shows on the character select screen is left out
/// and sent as zero - see [`character_record`] for which fields those are.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character {
    pub id: u32,
    pub name: String,
    pub gender: u8,
    pub skin: u8,
    pub face: u32,
    pub hair: u32,
    pub level: u32,
    pub job: u16,
    pub strength: u16,
    pub dexterity: u16,
    pub intelligence: u16,
    pub luck: u16,
    pub hp: u32,
    pub max_hp: u32,
    pub mp: u32,
    pub max_mp: u32,
    pub ap: u16,
    pub map_id: u32,
    /// `(slot, itemId)` pairs for the avatar's visible equipment.
    pub equips: Vec<(u8, u32)>,
}

/// Where a new character starts: **map 1, "Mushroom Town - West Entrance"**.
///
/// The owner, 2026-08-19. It had never been set - `Character::default` carried `map_id: 0`, and
/// nothing on the creation path overrode it.
///
/// **Zero is not a map.** The game's own table in `String.wz` has no entry for it; the
/// lowest real id is 1 (`gm-handbook/maps.txt`, regenerate with `tools/dump_names.py`). So
/// a stored `0` means "never assigned" rather than a place, which is what makes repairing
/// existing rows safe rather than presumptuous.
pub const START_MAP_ID: u32 = 1;

impl Default for Character {
    /// A level 1 beginner with the classic starting roll.
    ///
    /// `job` is `0`, which matters: the stat block's SP field forks on the job, and `0`
    /// takes the extended-SP branch. See [`uses_extended_sp`].
    fn default() -> Self {
        Character {
            id: 1,
            name: String::new(),
            gender: 0,
            skin: 0,
            face: 20000,
            hair: 30000,
            level: 1,
            job: 0,
            strength: 12,
            dexterity: 5,
            intelligence: 4,
            luck: 4,
            hp: 50,
            max_hp: 50,
            mp: 5,
            max_mp: 5,
            ap: 0,
            map_id: START_MAP_ID,
            equips: Vec::new(),
        }
    }
}

/// Whether the stat block sends an extended SP table for this job rather than a single
/// `u16`.
///
/// `FUN_140302e30` branches on the job it just read, and the branch is a bit test against
/// three literal masks. Decoding them gives exactly the explorer job tree - 100/110/111/112
/// /120/121/122/130/131/132 and the same shape at 200, 300, 400 and 500, plus 430-439 - and
/// that is the strongest single check that `+0x33` really is the job and that every field
/// before it is in the right place.
///
/// Job `0`, a beginner, also takes this branch.
pub fn uses_extended_sp(job: u16) -> bool {
    if job == 0 {
        return true;
    }
    if (430..440).contains(&job) {
        return true;
    }
    let branch = job % 100;
    match job / 100 {
        1 | 2 => matches!(branch, 0 | 10..=12 | 20..=22 | 30..=32),
        3..=5 => matches!(branch, 0 | 10..=12 | 20..=22),
        _ => false,
    }
}

/// A fixed-width, NUL-padded field. Mirrors `FUN_1406e9170`, which reads a byte count.
fn put_fixed(out: &mut Vec<u8>, s: &str, len: usize) {
    let bytes = s.as_bytes();
    let taken = bytes.len().min(len);
    out.extend_from_slice(&bytes[..taken]);
    out.extend(std::iter::repeat_n(0u8, len - taken));
}

/// One character record: the stat block, a short trailer, then the avatar look.
///
/// # How it was established
///
/// Read from `FUN_1403094b0`, which is `FUN_140302e30` (the stat block) followed by four
/// fields and then `FUN_1402ee8d0` (the look). Independently cross-checked field for field
/// against `CharacterStat.encode` in a modern MapleStory server source, which agrees on the
/// whole head - id, two log ids, the 13-byte name, gender, skin, a zero, face, hair, level,
/// job, the four stats, hp/maxHp/mp/maxMp, ap, then the SP fork. Two sources deriving the
/// same order independently is why this is worth trusting further than the usual static
/// read - but it has still never been on the wire.
///
/// # Body
///
/// ```text
/// u32  characterId
/// u32  characterIdForLog
/// u32  worldIdForLog
/// 13B  name                fixed width, NUL padded
/// u8   gender
/// u8   skin
/// u32                      always zero in the reference encoder
/// u32  face
/// u32  hair
/// u32  level
/// u16  job
/// u16  str, dex, int, luk
/// u32  hp, maxHp, mp, maxMp
/// u16  ap
///      extended-SP jobs: u8 count, then count x (u8 jobLevel, u32 sp)
///      everything else:  u16 sp
/// u64  exp
/// u32  fame
/// u32
/// u8   portal
/// u16  subJob
/// u8
/// 8B   FILETIME
/// u32, u32
/// // trailer, FUN_1403094b0
/// u32, u64, u32, u64
/// // avatar look, FUN_1402ee8d0
/// u8   gender
/// u8   skin
/// u32                      unused
/// u32  face
/// u32  job                 stored at +0x1bd, away from the look block
/// u8                       read and discarded
/// u32  hair                equipment array index 0; the pair loop cannot reach it
/// u8/u32 pairs             equipment, terminated by slot 0xFF. Slots 1..31 only
/// u8/u32 pairs             a second map at +0xb9, terminated by slot 0xFF
/// u32, u32, u32, u32
/// u32                      taken modulo 360
/// u8
/// u32
/// 4B, 128B, u32, 13B
/// ```
///
/// **The client's readers throw on underrun**, so a record that is short by one byte is not
/// a rendering glitch - it is an exception inside the packet handler.
pub fn character_record(chr: &Character, world_id: u32) -> Vec<u8> {
    let mut out = Vec::new();

    // --- the stat block, FUN_140302e30 with param_3 == 0
    out.extend_from_slice(&chr.id.to_le_bytes());
    out.extend_from_slice(&chr.id.to_le_bytes()); // characterIdForLog
    out.extend_from_slice(&world_id.to_le_bytes()); // worldIdForLog
    put_fixed(&mut out, &chr.name, CHARACTER_NAME_LEN);
    out.push(chr.gender);
    out.push(chr.skin);
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&chr.face.to_le_bytes());
    out.extend_from_slice(&chr.hair.to_le_bytes());
    out.extend_from_slice(&chr.level.to_le_bytes());
    out.extend_from_slice(&chr.job.to_le_bytes());
    out.extend_from_slice(&chr.strength.to_le_bytes());
    out.extend_from_slice(&chr.dexterity.to_le_bytes());
    out.extend_from_slice(&chr.intelligence.to_le_bytes());
    out.extend_from_slice(&chr.luck.to_le_bytes());
    out.extend_from_slice(&chr.hp.to_le_bytes());
    out.extend_from_slice(&chr.max_hp.to_le_bytes());
    out.extend_from_slice(&chr.mp.to_le_bytes());
    out.extend_from_slice(&chr.max_mp.to_le_bytes());
    out.extend_from_slice(&chr.ap.to_le_bytes());
    if uses_extended_sp(chr.job) {
        out.push(0); // no SP pools
    } else {
        out.extend_from_slice(&0u16.to_le_bytes()); // sp
    }
    out.extend_from_slice(&0u64.to_le_bytes()); // exp
    out.extend_from_slice(&0u32.to_le_bytes()); // fame
    out.extend_from_slice(&0u32.to_le_bytes());
    out.push(0); // portal
    out.extend_from_slice(&0u16.to_le_bytes()); // subJob
    out.push(0);
    out.extend_from_slice(&0u64.to_le_bytes()); // FILETIME
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());

    // --- the trailer, FUN_1403094b0
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&chr.map_id.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());

    // --- the avatar look, FUN_1402ee8d0
    //
    // The field order here was wrong until 2026-08-17, and the symptom was a created
    // character rendering with the wrong hair and no equipment while its name, level and
    // stats - which come from the stat block above - were all correct.
    //
    // `FUN_1402ee8d0` reads three `u32`s, a discarded byte, then one more `u32`, and the
    // destinations say what they are: the third goes to `+0x1bd`, far from the look block,
    // and the last goes to `+0x39`, which is index 0 of the equipment array the pair loop
    // fills at `+0x39 + slot*4`. That loop rejects anything outside slots 1..31, so index 0
    // can *only* be written by this standalone field - and it is the hair.
    out.push(chr.gender);
    out.push(chr.skin);
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&chr.face.to_le_bytes());
    out.extend_from_slice(&u32::from(chr.job).to_le_bytes());
    out.push(0); // read and discarded
    out.extend_from_slice(&chr.hair.to_le_bytes()); // equipment array index 0
    for (slot, item) in &chr.equips {
        out.push(*slot);
        out.extend_from_slice(&item.to_le_bytes());
    }
    out.push(0xFF); // end of the equipment map
    out.push(0xFF); // end of the second map
    for _ in 0..4 {
        out.extend_from_slice(&0u32.to_le_bytes());
    }
    out.extend_from_slice(&0u32.to_le_bytes()); // taken modulo 360
    out.push(0);
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&[0u8; 128]);
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&[0u8; CHARACTER_NAME_LEN]);
    out
}

/// How many zero bytes to append after the decoded part of a [`LOGIN_RESULT`].
///
/// The fields past `slotCount` are still not decoded. What is established is that the reply
/// which carried the client to character select was `00 00 00` plus 253 zero bytes and was
/// accepted, so a generous zero tail satisfies whatever the client reads there.
/// Under-providing is not a cosmetic problem: the readers throw on underrun.
const LOGIN_RESULT_TAIL_PAD: usize = 200;

/// How many characters the account may have. `3` in MapleStory Classic World (the owner).
///
/// This is not decoration. `FUN_141b282d0`, the only reader, is the create-a-character
/// gate: it takes `slotCount - stage+0xe4 - 1`, clamps it at zero, and looks up the
/// character in that slot. If the slot is occupied it raises a "no more characters" notice
/// and sends nothing. Sending `0` clamped the index to `0`, which is the *first* character -
/// so with one character in the list the button could never do anything.
pub const CHARACTER_SLOTS: u32 = 3;

/// A successful [`LOGIN_RESULT`], carrying the character list.
///
/// # The list is inside this packet
///
/// This is the thing that was missing while the character select screen sat inert. There is
/// no separate character-list opcode. `FUN_141b307b0` reads the head below, then calls two
/// sub-readers: `FUN_14108d290` for the scheduled-deletion list and `FUN_14108bdf0` for the
/// display order and the characters themselves. Our old reply was zero-padded, so the
/// client read a count of `0` and drew an empty screen - which is exactly why every "Create
/// a character" click sent nothing.
///
/// # Body
///
/// ```text
/// u8   result          0 = success
/// str  message
/// u8
/// 8B   FILETIME
/// u32  worldId         looked up in the world list from WORLD_LIST
/// u32  channelId
/// 4B, 4B, 4B
/// u32
/// u8
/// u32  deletionCount   then count x (u32 characterId, 8B FILETIME)
/// u32  orderCount      then count x u32 characterId
/// u8   characterCount  then count x character record
/// u8                   -> stage+0xdc, and it must be 1
/// u8                   -> stage+0xe0, and it must be non-zero
/// u32  slotCount       -> the character manager's slot count
/// ..   an undecoded tail
/// ```
///
/// `worldId` and `channelId` are not filler: `FUN_141b2c7c0` looks the world up in the list
/// built from [`WORLD_LIST`], so they have to name a world we actually sent.
///
/// **The three fields after the list are the create-a-character gate.** `FUN_141b282d0`
/// refuses unless `stage+0xdc == 1` *and* `stage+0xe0 != 0` *and* the slot computed from
/// `slotCount` is free. All three were inside the zero padding, which is why the button did
/// nothing on the run that otherwise reached character select correctly.
pub fn login_result(world_id: u32, channel_id: u32, characters: &[Character]) -> Vec<u8> {
    let mut out = vec![LOGIN_OK];
    put_str(&mut out, ""); // message
    out.push(0);
    out.extend_from_slice(&0u64.to_le_bytes()); // FILETIME
    out.extend_from_slice(&world_id.to_le_bytes());
    out.extend_from_slice(&channel_id.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&0u32.to_le_bytes());
    out.push(0);

    out.extend_from_slice(&0u32.to_le_bytes()); // no scheduled deletions

    out.extend_from_slice(&(characters.len() as u32).to_le_bytes());
    for chr in characters {
        out.extend_from_slice(&chr.id.to_le_bytes());
    }

    out.push(characters.len() as u8);
    for chr in characters {
        out.extend_from_slice(&character_record(chr, world_id));
    }

    // The create-a-character gate. Both bytes are tested, not stored - see the doc above.
    out.push(1); // -> stage+0xdc, tested for exactly 1
    out.push(1); // -> stage+0xe0, tested for non-zero
    out.extend_from_slice(&CHARACTER_SLOTS.to_le_bytes());

    out.extend(std::iter::repeat_n(0u8, LOGIN_RESULT_TAIL_PAD));
    out
}

/// The reply to the client's [`CLIENT_CHECK_NAME_REQUEST`].
///
/// The client echoes nothing itself - it compares the name we send back, so it has to be
/// the name that was asked about.
pub fn check_name_result(name: &str, code: u8) -> Vec<u8> {
    let mut out = Vec::new();
    put_str(&mut out, name);
    out.push(code);
    out
}

/// A successful [`CREATE_CHARACTER_RESULT`]: here is the character you just made.
pub fn create_character_result(world_id: u32, chr: &Character) -> Vec<u8> {
    let mut out = vec![LOGIN_OK];
    out.extend_from_slice(&world_id.to_le_bytes());
    out.extend_from_slice(&character_record(chr, world_id));
    out.push(1); // return to character select
    out
}

/// "You have no free character slot" - the account is full.
///
/// From the create-result handler's own switch; it maps this code to the
/// `insufficientCharacterSlot` notice.
pub const CREATE_INSUFFICIENT_SLOT: u8 = 0x09;

/// A refusal with no specific notice. Anything the handler's switch does not name falls
/// through to `cannotProcessRequest`, which is the honest answer when the server refused
/// for a reason the client has no wording for - a duplicate name, or a failed write.
pub const CREATE_CANNOT_PROCESS: u8 = 0x01;

/// A refused [`CREATE_CHARACTER_RESULT`]. Any non-zero code raises a notice and leaves the
/// client on the creation screen.
pub fn create_character_failed(code: u8) -> Vec<u8> {
    vec![code]
}

/// The client asking to enter the world with one character. Captured twice, with ids 203
/// and 204, 73 bytes each:
///
/// ```text
/// u32  0
/// str  "."          a placeholder PIC
/// u32  characterId
/// u8
/// str  MAC list
/// str  machine id
/// ```
///
/// Unanswered, this is what leaves the client sitting on "Connecting..." forever.
pub const CLIENT_SELECT_CHARACTER_REQUEST: u16 = 0x0078;

/// Where the character id sits in a [`CLIENT_SELECT_CHARACTER_REQUEST`], given the leading
/// `u32` and a one-character PIC string. Parsed rather than assumed - see
/// [`SelectCharacterRequest::parse`].
const SELECT_PIC_AT: usize = 4;

/// The client's select-character request, parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectCharacterRequest {
    pub character_id: u32,
}

impl SelectCharacterRequest {
    /// Walk the leading `u32` and the PIC string rather than indexing a fixed offset: the
    /// PIC is a placeholder `"."` today but it is a real string field, so a fixed offset
    /// would break the moment a PIC is set.
    pub fn parse(payload: &[u8]) -> Option<Self> {
        let pic_len = payload
            .get(SELECT_PIC_AT..SELECT_PIC_AT + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]) as usize)?;
        let at = SELECT_PIC_AT + 2 + pic_len;
        let id = payload.get(at..at + 4)?;
        Some(SelectCharacterRequest {
            character_id: u32::from_le_bytes([id[0], id[1], id[2], id[3]]),
        })
    }
}

/// **The migration packet** - `case 0x11` of the login stage, `FUN_141b36f60`.
///
/// Identified statically 2026-08-19 with no client run. It is the only handler in that
/// switch that builds a `sockaddr_in` (`htons` occurs exactly once across all fourteen
/// decompiled case handlers, and it is here), and on success it transitions to the string
/// `GameIn`. Full decode, including the obfuscated tail, in `docs/opcodes.md`.
pub const MIGRATE_COMMAND: u16 = 0x0011;

/// Migration accepted. The result byte goes through `FUN_141b267c0`, the same gate as
/// [`ACCOUNT_INFO`] and [`LOGIN_RESULT`], which returns "proceed" for its `default` case -
/// and `0` lands in the default.
pub const MIGRATE_OK: u8 = 0;

/// Refuse a migration. **Not any non-zero value**, for three separate reasons the gate and
/// its caller give:
///
/// * `-1`, `6`, `8` and `9` raise `loginTroubleAskSupport` and then **fall through to the
///   default**, so the client shows an error *and migrates anyway*.
/// * `0x0F` (`notRegisteredAccount`) opens a browser at a Nexon URL.
/// * `0x0C`, `0x22`, `0x27`, `0x37`, `0x43`, `0x80` and `0x8E` are intercepted by
///   `FUN_141b36f60` *before* the gate and return without a message.
///
/// `0x0A` raises `loginTimeout` and `break`s, so the gate returns 0 and the handler stops.
pub const MIGRATE_REFUSED: u8 = 0x0A;

/// The key for the migration packet's obfuscated tail. Zero is a legal key and makes the
/// transform a fixed one; nothing in the client requires it to vary.
const MIGRATE_TAIL_KEY: u32 = 0;

/// Undo one word of the client's in-place tail transform.
///
/// `FUN_141b36f60` walks the tail in aligned `u32` steps and computes
///
/// ```text
/// plain = (((key ^ raw) + 0x369F144D + (key >> 7)) ^ 0xAAAABBBB) - (4n * key)
/// ```
///
/// for the word at byte offset `4n`. This is that, inverted. Everything in it comes from
/// the packet itself, so there is no key material to discover.
fn migrate_tail_word(plain: u32, key: u32, offset: u32) -> u32 {
    let t = (plain.wrapping_add(offset.wrapping_mul(key))) ^ 0xAAAA_BBBB;
    t.wrapping_sub(0x369F_144D).wrapping_sub(key >> 7) ^ key
}

/// What the client computes from a tail word - the forward direction, so a test can prove
/// [`migrate_tail_word`] inverts it rather than asserting a hand-computed constant.
#[cfg(test)]
fn migrate_tail_word_forward(raw: u32, key: u32, offset: u32) -> u32 {
    let t = (key ^ raw)
        .wrapping_add(0x369F_144D)
        .wrapping_add(key >> 7);
    (t ^ 0xAAAA_BBBB).wrapping_sub(offset.wrapping_mul(key))
}

/// Body of a [`MIGRATE_COMMAND`] that sends the client to `addr` as `character_id`.
///
/// `seed` is the `u32` the client stashes at `DAT_143ac80b0` (XORed with a replicated
/// random byte) and **sends back in outbound `0x007D` on the new connection** - so it is
/// the server's own hand-off token, not something the client invents. That prediction is
/// the cheapest possible check on this whole decode.
///
/// The layout is in `docs/opcodes.md`. Two fields are deliberately zero rather than
/// configurable: the second of the three `u32` after the character id makes the client load
/// `Etc/SpecialServerInfo.img` when non-zero, and the flags byte has two bits the client
/// reads separately.
pub fn migrate(addr: std::net::SocketAddrV4, character_id: u32, seed: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(59);
    out.push(MIGRATE_OK);
    out.extend_from_slice(&0u16.to_le_bytes()); // message, empty
    out.push(0);

    // Straight into sockaddr_in.sin_addr, which is network order - so the octets go on the
    // wire in order. The port is read as a plain u16 and htons()'d by the client.
    out.extend_from_slice(&addr.ip().octets());
    out.extend_from_slice(&addr.port().to_le_bytes());

    out.extend_from_slice(&character_id.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // -> DAT_143ac2040
    out.extend_from_slice(&0u32.to_le_bytes()); // -> DAT_143ac2044: SpecialServerInfo.img
    out.extend_from_slice(&0u32.to_le_bytes()); // -> _DAT_143ac2160
    out.push(0); // flags: bit 0 and bit 1 are read separately
    out.extend_from_slice(&0u32.to_le_bytes());
    out.push(0);
    out.push(0); // read and discarded
    out.extend_from_slice(&0u32.to_le_bytes()); // read and discarded
    out.push(0); // read and discarded
    out.push(0); // read and discarded
    out.extend_from_slice(&[0u8; 8]);

    // The tail: key, length, then `length` obfuscated bytes. Four is the smallest length
    // that covers the client's one 4-byte read, and it leaves no trailing partial word -
    // so only the aligned-word branch of the transform runs.
    out.extend_from_slice(&MIGRATE_TAIL_KEY.to_le_bytes());
    out.extend_from_slice(&4u32.to_le_bytes());
    out.extend_from_slice(&migrate_tail_word(seed, MIGRATE_TAIL_KEY, 0).to_le_bytes());
    out
}

/// A refused [`MIGRATE_COMMAND`]: the result byte, a message for the dialog, and the third
/// byte.
///
/// The third `u8` is **not optional on the refusal path**. `FUN_141b36f60` reads it before
/// it looks at the result code at all, and the client's readers throw on underrun - so a
/// two-field refusal would fault instead of showing the dialog.
pub fn migrate_refused(message: &str) -> Vec<u8> {
    let mut out = vec![MIGRATE_REFUSED];
    let bytes = message.as_bytes();
    out.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
    out.extend_from_slice(bytes);
    out.push(0);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The decoder is `((n & 1) * -2 + 1) * ((n >> 1) + (n & 1))` in the client; this is
    /// the same mapping, and it is the round trip that matters.
    fn decode(bytes: &[u8]) -> i32 {
        let mut n: u32 = 0;
        let mut shift = 0;
        for b in bytes {
            n |= u32::from(b & 0x7F) << shift;
            if b & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        ((n >> 1) as i32) ^ -((n & 1) as i32)
    }

    #[test]
    fn zigzag_round_trips_including_the_signs_that_matter() {
        // 0 means "nothing to patch", negative means "delete Data.wz", positive is a
        // byte count - all three steer the client down different paths.
        for v in [0, 1, -1, 2, -2, 63, 64, -64, 8192, -8192, i32::MAX, i32::MIN] {
            assert_eq!(decode(&zigzag_varint(v)), v, "round trip failed for {v}");
        }
    }

    #[test]
    fn the_up_to_date_reply_is_a_single_zero_byte() {
        // The client reads one varint and stops, so this is the entire body.
        assert_eq!(data_wz_up_to_date(), vec![0x00]);
    }

    /// Walks the entry the way `FUN_141b31ff0` does and returns how many bytes it read.
    /// If the builder and the client's read sequence ever disagree, this stops matching
    /// the buffer length.
    fn read_world_entry(b: &[u8]) -> usize {
        let mut i = 0;
        let take_str = |i: &mut usize| {
            let n = u16::from_le_bytes([b[*i], b[*i + 1]]) as usize;
            *i += 2 + n;
        };
        i += 1; // worldId
        take_str(&mut i); // worldName
        i += 1; // flag
        take_str(&mut i); // eventDescription
        i += 1; // flag
        let channels = b[i];
        i += 1;
        for _ in 0..channels {
            take_str(&mut i); // channelName
            i += 4 + 4; // u32, then four u8
        }
        let balloons = u16::from_le_bytes([b[i], b[i + 1]]);
        i += 2;
        for _ in 0..balloons {
            i += 4; // u16 x, u16 y
            take_str(&mut i);
        }
        i += 4; // u32
        i += 1; // hasExtra
        i
    }

    #[test]
    fn a_world_entry_is_read_back_exactly_as_it_was_built() {
        // Zero channels and several channels exercise the loop boundary, which is where a
        // field-order mistake would otherwise hide.
        for channels in [0u8, 1, 3] {
            let body = world_list_entry(0, "Scania", channels);
            assert_eq!(
                read_world_entry(&body),
                body.len(),
                "client's read sequence disagrees with the builder for {channels} channels"
            );
        }
    }

    /// The probe is driven from the command line with hex bodies, so the bytes that go on
    /// the wire are typed by hand. Pinning them here means a change to the builder breaks
    /// this test rather than silently disagreeing with a command in `STATUS.md`.
    #[test]
    fn the_one_world_list_we_actually_send_has_these_exact_bytes() {
        let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02x}")).collect::<String>();
        assert_eq!(
            hex(&world_list_entry(0, "Scania", 1)),
            "0006005363616e6961000000000108005363616e69612d30000000000000000000000000000000"
        );
        assert_eq!(hex(&world_list_end()), "ff0000");
    }

    #[test]
    fn the_terminator_trips_the_clients_signed_test() {
        // The client's check is `(char) worldId < 0`, not `== 0xFF`.
        let end = world_list_end();
        assert!((end[0] as i8) < 0, "terminator must have its high bit set");
        // A real world id must not accidentally look like one.
        assert!((world_list_entry(0, "Scania", 1)[0] as i8) >= 0);
    }

    /// Walks a successful [`ACCOUNT_INFO`] body the way `FUN_141b2dd00` does and returns
    /// how many bytes it consumed. The client's readers throw on underrun, so a builder
    /// that is short by one field would abort the handler mid-way - and a builder that is
    /// merely *misaligned* would silently hand the account-name setter the wrong bytes.
    /// Both show up here as a length mismatch.
    fn read_account_info(b: &[u8]) -> usize {
        let mut i = 0;
        let take_str = |i: &mut usize| {
            let n = u16::from_le_bytes([b[*i], b[*i + 1]]) as usize;
            *i += 2 + n;
        };
        assert_eq!(b[i], LOGIN_OK, "only the success path reads the fields below");
        i += 1; // result
        take_str(&mut i); // message
        let verify_state = b[i];
        assert!(verify_state <= 1, "2 and 3 raise accountHasNotBeenVerified");
        i += 1;
        i += 4; // discarded u32

        take_str(&mut i); // loginName
        i += 8; // discarded u64
        i += 4; // accountId
        i += 1;
        let flags = u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        assert_eq!(flags >> 21 & 1, 0, "bit 21 pulls in FUN_140d2d4e0");
        i += 4;
        i += 4;
        i += 1;
        take_str(&mut i);
        i += 4; // -> account+0x22b8
        i += 1 + 1;
        i += 8 + 8; // two FILETIMEs
        i += 4;
        take_str(&mut i);
        i += 4; // -> account+0x28e0

        i += 1; // discarded
        i += 1; // -> stage+0x1a4
        i += 1; // -> stage+0xdc
        i += 8; // -> account+0x2324
        take_str(&mut i); // accountName
        i
    }

    #[test]
    fn the_account_info_body_is_read_back_exactly_as_it_was_built() {
        for (login, account) in [
            ("", ""),
            ("maplecw", "e***@example.com"),
            ("a-much-longer-login-name", "wisp****@example.com"),
        ] {
            let body = account_info(login, account);
            assert_eq!(
                read_account_info(&body),
                body.len(),
                "client's read sequence disagrees with the builder for {login:?}"
            );
        }
    }

    #[test]
    fn the_account_name_is_the_last_thing_in_the_body() {
        // It is the field the login screen displays, and the one whose position is easiest
        // to get wrong, so pin it from the other end: the body must end with exactly the
        // string, length prefix included.
        let name = "e***@example.com";
        let body = account_info("maplecw", name);
        let tail = 2 + name.len();
        assert_eq!(&body[body.len() - name.len()..], name.as_bytes());
        assert_eq!(
            u16::from_le_bytes([body[body.len() - tail], body[body.len() - tail + 1]]) as usize,
            name.len()
        );
    }

    /// Same reason as the world list: these bytes get typed onto a command line by hand.
    #[test]
    fn the_account_info_we_actually_send_has_these_exact_bytes() {
        let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02x}")).collect::<String>();
        // Spelled out in pieces rather than as one blob, because the interesting claim is
        // that the two strings sit at those two positions and everything between them is
        // zero - which a 204-character literal would hide rather than state.
        let expected = format!(
            "{}{}{}{}",
            "0000000000000000",                     // result, message, verifyState, u32
            "07006d61706c656377",                   // loginName "maplecw"
            "0".repeat(134),                        // every field the screen does not show
            "1000652a2a2a406578616d706c652e636f6d", // accountName "e***@example.com"
        );
        assert_eq!(hex(&account_info("maplecw", "e***@example.com")), expected);
        assert_eq!(expected.len() / 2, 102, "one packet body, 102 bytes");
    }

    #[test]
    fn a_zero_padded_body_also_decodes_to_zero() {
        // Why the in-process walk worked with 512 bytes of padding: the first byte is a
        // complete varint, so one probe body satisfied the handler for every opcode.
        assert_eq!(decode(&[0u8; 512]), 0);
    }

    /// Walks a character record the way `FUN_1403094b0` does and returns how many bytes it
    /// consumed. The stat block's SP field forks on the job it just read, so this walker
    /// only stays in step if the job really is where the builder puts it.
    fn read_character_record(b: &[u8]) -> usize {
        let mut i = 0;
        // FUN_140302e30, the stat block
        i += 4 + 4 + 4; // id, characterIdForLog, worldIdForLog
        i += CHARACTER_NAME_LEN;
        i += 1 + 1; // gender, skin
        i += 4 + 4 + 4 + 4; // zero, face, hair, level
        let job = u16::from_le_bytes([b[i], b[i + 1]]);
        i += 2;
        i += 2 * 4; // str, dex, int, luk
        i += 4 * 4; // hp, maxHp, mp, maxMp
        i += 2; // ap
        if uses_extended_sp(job) {
            let pools = b[i];
            i += 1;
            i += pools as usize * (1 + 4);
        } else {
            i += 2; // sp
        }
        i += 8; // exp
        i += 4 + 4; // fame, u32
        i += 1; // portal
        i += 2; // subJob
        i += 1;
        i += 8; // FILETIME
        i += 4 + 4;
        // FUN_1403094b0's own fields
        i += 4 + 8 + 4 + 8;
        // FUN_1402ee8d0, the avatar look
        i += 1 + 1 + 4 + 4 + 4; // gender, skin, unused, face, job
        i += 1; // discarded
        i += 4; // hair, which is equipment index 0
        for _ in 0..2 {
            while b[i] != 0xFF {
                i += 1 + 4; // slot, itemId
            }
            i += 1; // the 0xFF terminator
        }
        i += 4 * 4;
        i += 4; // taken modulo 360
        i += 1;
        i += 4;
        i += 4 + 128 + 4 + CHARACTER_NAME_LEN;
        i
    }

    #[test]
    fn the_avatar_look_puts_face_and_hair_where_the_client_reads_them() {
        // This is the one field order in the record that a round-trip test cannot check:
        // the round trip above only *skips* the look block by size, so it passed happily
        // while face and hair were each written one field too early. The symptom was a
        // created character with the wrong hair and no equipment, and correct name, level
        // and stats - because those come from the stat block instead.
        //
        // `FUN_1402ee8d0` reads exactly this run of bytes, so assert the run itself rather
        // than an offset computed from the fields before it.
        let chr = Character {
            face: 0xAAAA_AAAA,
            hair: 0xBBBB_BBBB,
            job: 0x0CCC,
            gender: 1,
            skin: 3,
            ..Character::default()
        };
        let record = character_record(&chr, 0);

        let mut want = Vec::new();
        want.push(1u8); // gender
        want.push(3u8); // skin
        want.extend_from_slice(&0u32.to_le_bytes()); // unused
        want.extend_from_slice(&0xAAAA_AAAAu32.to_le_bytes()); // face
        want.extend_from_slice(&0x0000_0CCCu32.to_le_bytes()); // job, -> +0x1bd
        want.push(0); // read and discarded
        want.extend_from_slice(&0xBBBB_BBBBu32.to_le_bytes()); // hair, -> equip index 0

        let at = record
            .windows(want.len())
            .position(|w| w == want)
            .expect("the avatar look must read gender, skin, 0, face, job, pad, hair");

        // And the equipment map starts immediately after it, so a slot the client would
        // reject (it takes 1..31 only) cannot be hiding in that gap.
        assert_eq!(
            record[at + want.len()],
            0xFF,
            "an empty equipment map should terminate straight away"
        );
    }

    #[test]
    fn a_character_record_is_read_back_exactly_as_it_was_built() {
        // Equipment exercises the two 0xFF-terminated maps, and a non-explorer job takes
        // the other side of the SP fork - the one place in the record where a wrong job
        // offset would shift every later field without any other symptom.
        for (job, equips) in [
            (0u16, vec![]),
            (110, vec![(5u8, 1040036u32), (6, 1060026), (7, 1072038)]),
            (2000, vec![(11, 1302000)]),
        ] {
            let chr = Character {
                name: "Testy".into(),
                job,
                equips,
                ..Character::default()
            };
            let body = character_record(&chr, 0);
            assert_eq!(
                read_character_record(&body),
                body.len(),
                "client's read sequence disagrees with the builder for job {job}"
            );
        }
    }

    #[test]
    fn the_job_fork_matches_the_masks_decoded_from_the_client() {
        // FUN_140302e30 bit-tests three literal masks. These are the jobs they select, and
        // the fact that they spell out the explorer tree is the evidence that +0x33 is the
        // job at all - so a change here is a change to that claim.
        for job in [0, 100, 110, 111, 112, 120, 121, 122, 130, 131, 132] {
            assert!(uses_extended_sp(job), "explorer job {job} should extend SP");
        }
        for job in [200, 232, 300, 322, 400, 422, 430, 434, 439, 500, 522] {
            assert!(uses_extended_sp(job), "explorer job {job} should extend SP");
        }
        for job in [101, 113, 123, 133, 323, 429, 440, 523, 1000, 2000, 3000] {
            assert!(!uses_extended_sp(job), "job {job} should send a plain u16 sp");
        }
    }

    /// Walks a login result the way `FUN_141b307b0` and its two sub-readers do, stopping
    /// where the decode stops - the tail past the slot count is padding we do not claim to
    /// understand.
    fn read_login_result_through_the_gate(b: &[u8]) -> usize {
        let mut i = 0;
        i += 1; // result
        let msg = u16::from_le_bytes([b[i], b[i + 1]]) as usize;
        i += 2 + msg;
        i += 1;
        i += 8; // FILETIME
        i += 4 + 4; // worldId, channelId
        i += 4 + 4 + 4;
        i += 4;
        i += 1;
        // FUN_14108d290
        let deletions = u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        i += 4;
        i += deletions as usize * (4 + 8);
        // FUN_14108bdf0
        let order = u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        i += 4;
        i += order as usize * 4;
        let count = b[i];
        i += 1;
        for _ in 0..count {
            i += read_character_record(&b[i..]);
        }
        // The create-a-character gate.
        assert_eq!(b[i], 1, "stage+0xdc is tested for exactly 1");
        i += 1;
        assert_ne!(b[i], 0, "stage+0xe0 is tested for non-zero");
        i += 1;
        i += 4; // slotCount
        i
    }

    #[test]
    fn the_login_result_lands_the_client_exactly_at_the_padding() {
        // The list is inside this packet, so an off-by-one anywhere above it desynchronises
        // the record decode rather than producing a short read - the failure mode that
        // would otherwise look like "the client just does not draw the characters".
        for count in [0usize, 1, 3] {
            let chars: Vec<Character> = (0..count)
                .map(|i| Character {
                    id: 100 + i as u32,
                    name: format!("Hero{i}"),
                    ..Character::default()
                })
                .collect();
            let body = login_result(0, 0, &chars);
            assert_eq!(
                read_login_result_through_the_gate(&body),
                body.len() - LOGIN_RESULT_TAIL_PAD,
                "read sequence disagrees with the builder for {count} characters"
            );
        }
    }

    /// `FUN_141b282d0`'s own arithmetic: the slot it checks is `slotCount - stage+0xe4 - 1`
    /// clamped at zero, and creation is allowed only when no character occupies it. Written
    /// as the client writes it - signed, then clamped - because the clamp is the whole
    /// story: it is what turned a slot count of `0` into "look at the first character".
    fn client_will_offer_creation(slot_count: i32, characters: i32) -> bool {
        let purchased = 0; // stage+0xe4, which this reply never sets
        let slot = (slot_count - purchased - 1).max(0);
        slot >= characters
    }

    #[test]
    fn the_slot_count_leaves_a_free_slot_the_client_will_accept() {
        // Why the button did nothing on the run that otherwise worked.
        assert!(
            !client_will_offer_creation(0, 1),
            "a zero slot count clamps to the first character, which is occupied"
        );
        let slots = CHARACTER_SLOTS as i32;
        assert!(client_will_offer_creation(slots, 0), "empty account");
        assert!(client_will_offer_creation(slots, 1), "one of three");
        assert!(client_will_offer_creation(slots, 2), "two of three");
        assert!(
            !client_will_offer_creation(slots, 3),
            "a full account must not be offered creation"
        );
    }

    /// The exact 101 bytes the client sent on 2026-08-17, creating "Hello" with a
    /// 7/5/7/6 roll... no: 10/4/5/6. Kept verbatim because the builder is virtualised and
    /// this capture is the only specification that exists.
    const CAPTURED_CREATE_REQUEST: &str = "\
0500 48656c6c6f 00000000 ffffffff 00000000 0000 \
0a000000 04000000 05000000 06000000 \
00000000 02000000 31750000 \
06000000 \
01000000 214e0000 02000000 30750000 03000000 82de0f00 \
04000000 a22c1000 05000000 825b1000 06000000 f0dd1300";

    fn captured() -> Vec<u8> {
        let hex: String = CAPTURED_CREATE_REQUEST.chars().filter(|c| !c.is_whitespace()).collect();
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn the_captured_create_request_parses_field_for_field() {
        let body = captured();
        assert_eq!(body.len(), 101, "the capture is 101 bytes");
        let req = CreateCharacterRequest::parse(&body).expect("should parse");
        assert_eq!(req.name, "Hello");
        assert_eq!(req.race, 0);
        assert_eq!(req.sub_job, 0);
        // The four the client refuses to send unless they total 25.
        assert_eq!(
            (req.strength, req.dexterity, req.intelligence, req.luck),
            (10, 4, 5, 6)
        );
        assert_eq!(
            req.strength + req.dexterity + req.intelligence + req.luck,
            25
        );
        assert_eq!(req.gender, 0);
        assert_eq!(req.skin, 2);
        assert_eq!(req.hair, 30001);
        assert_eq!(
            req.items,
            vec![
                (1, 20001),
                (2, 30000),
                (3, 1040002),
                (4, 1060002),
                (5, 1072002),
                (6, 1302000),
            ]
        );
    }

    #[test]
    fn the_created_character_wears_what_was_asked_for() {
        // The bug this exists to prevent: the first reply sent a default character and the
        // new character came back naked on screen.
        let chr = CreateCharacterRequest::parse(&captured()).unwrap().character(200);
        assert_eq!(chr.name, "Hello");
        assert_eq!(chr.face, 20001);
        assert_eq!(chr.hair, 30001, "the colour variant, not the base style");
        assert_eq!(chr.skin, 2);
        // Creation categories 3/4/5/6 become equipment slots 5/6/7/11 - copying the
        // request's own numbering across would put the clothes in slots that render as
        // nothing.
        assert_eq!(
            chr.equips,
            vec![(5, 1040002), (6, 1060002), (7, 1072002), (11, 1302000)]
        );
        // And it must still survive the client's own read sequence.
        let record = character_record(&chr, 0);
        assert_eq!(read_character_record(&record), record.len());
    }

    #[test]
    fn a_truncated_create_request_is_rejected_rather_than_guessed() {
        let body = captured();
        for cut in [0, 1, 6, 20, 50, 100] {
            assert!(
                CreateCharacterRequest::parse(&body[..cut]).is_none(),
                "a {cut}-byte body must not parse"
            );
        }
    }

    #[test]
    fn the_one_character_list_we_actually_send_has_these_exact_bytes() {
        // Same reason as the world list: this body gets typed onto a command line, so it
        // has to be pinned somewhere that fails loudly when the builder changes.
        let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02x}")).collect::<String>();
        let chr = Character {
            id: 1,
            name: "Maple".into(),
            ..Character::default()
        };
        let body = login_result(0, 0, std::slice::from_ref(&chr));
        // 50 bytes of head, one 327-byte record, the 6-byte gate, then the undecoded tail.
        assert_eq!(character_record(&chr, 0).len(), 327);
        assert_eq!(body.len(), 50 + 327 + 6 + LOGIN_RESULT_TAIL_PAD);
        // Everything up to the order count is zero; then one order id and one character.
        let head = format!("{}{}{}{}", "0".repeat(82), "01000000", "01000000", "01");
        assert_eq!(hex(&body)[..head.len()], head);
        // The name sits 12 bytes into the record, which starts at byte 50.
        assert_eq!(&body[50 + 12..50 + 17], b"Maple");
    }

    /// A new character starts somewhere real. Zero is not a map - the game's own table
    /// has no entry for it - so a default of 0 meant every character spawned nowhere.
    #[test]
    fn a_new_character_starts_on_the_start_map() {
        assert_eq!(START_MAP_ID, 1);
        assert_eq!(Character::default().map_id, START_MAP_ID);
        assert_ne!(Character::default().map_id, 0, "0 is not a map");
    }

    /// And it has to survive into the record the client actually reads, not just sit in
    /// the struct.
    #[test]
    fn the_start_map_reaches_the_character_record() {
        let chr = Character { name: "Wanderer".to_string(), ..Character::default() };
        let record = character_record(&chr, 0);
        let found = record
            .windows(4)
            .any(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]) == START_MAP_ID);
        assert!(found, "the start map is not in the encoded record");
    }

    /// The tail transform has to invert for every key, not just the zero we send. If this
    /// ever fails, the client reads a different seed than the one we meant.
    #[test]
    fn migrate_tail_word_inverts_the_clients_transform() {
        for key in [0u32, 1, 0x369F_144D, 0xAAAA_BBBB, 0xFFFF_FFFF, 0x1234_5678] {
            for offset in [0u32, 4, 8, 0x100] {
                for plain in [0u32, 1, 0xDEAD_BEEF, 0xFFFF_FFFF] {
                    let raw = migrate_tail_word(plain, key, offset);
                    assert_eq!(
                        migrate_tail_word_forward(raw, key, offset),
                        plain,
                        "key={key:#x} offset={offset} plain={plain:#x}"
                    );
                }
            }
        }
    }

    /// Every field the client reads must be present, or `FUN_1406e9170` underruns and
    /// throws. 59 bytes is the whole read sequence added up.
    #[test]
    fn migrate_body_is_the_length_the_client_reads() {
        let body = migrate("127.0.0.1:8484".parse().unwrap(), 203, 0);
        assert_eq!(body.len(), 59);
    }

    /// The address goes into `sin_addr` unconverted, so the octets are in order on the
    /// wire; the port is a plain little-endian `u16` the client htons()es itself.
    #[test]
    fn migrate_carries_the_address_the_client_will_connect_to() {
        let body = migrate("10.0.0.7:9001".parse().unwrap(), 0x0000_00CB, 0);
        assert_eq!(&body[4..8], &[10, 0, 0, 7], "octets in order");
        assert_eq!(&body[8..10], &9001u16.to_le_bytes(), "port little-endian");
        assert_eq!(&body[10..14], &0x0000_00CBu32.to_le_bytes(), "character id");
    }

    /// The second of the three `u32` after the character id makes the client go and load
    /// `Etc/SpecialServerInfo.img`. It must stay zero.
    #[test]
    fn migrate_does_not_ask_for_special_server_info() {
        let body = migrate("127.0.0.1:8484".parse().unwrap(), 203, 0xAAAA_AAAA);
        assert_eq!(&body[14..26], &[0u8; 12], "the three u32 after the id are all zero");
    }

    /// The seed is what the client stashes and sends back in `0x007D`. It has to survive
    /// the tail transform, so read it back the way the client will.
    #[test]
    fn migrate_seed_survives_the_tail_transform() {
        let seed = 0x1BAD_C0DE;
        let body = migrate("127.0.0.1:8484".parse().unwrap(), 203, seed);
        let key = u32::from_le_bytes(body[47..51].try_into().unwrap());
        let len = u32::from_le_bytes(body[51..55].try_into().unwrap());
        let raw = u32::from_le_bytes(body[55..59].try_into().unwrap());
        assert_eq!(key, 0, "we send a zero key");
        assert_eq!(len, 4, "one aligned word, no partial tail");
        assert_eq!(migrate_tail_word_forward(raw, key, 0), seed);
    }

    /// A refusal still has to carry the third byte: the handler reads it before it looks
    /// at the result code.
    #[test]
    fn migrate_refusal_carries_the_byte_read_before_the_gate() {
        let body = migrate_refused("not your character");
        assert_eq!(body[0], MIGRATE_REFUSED);
        assert_eq!(body.len(), 1 + 2 + "not your character".len() + 1);
        assert_eq!(*body.last().unwrap(), 0);
    }

    /// The PIC is a real string field even though it is a placeholder ".", so the id is
    /// found by walking it, not by a fixed offset.
    #[test]
    fn select_character_request_walks_the_pic() {
        let mut body = 0u32.to_le_bytes().to_vec();
        body.extend_from_slice(&1u16.to_le_bytes());
        body.push(b'.');
        body.extend_from_slice(&204u32.to_le_bytes());
        assert_eq!(SelectCharacterRequest::parse(&body).unwrap().character_id, 204);

        // A longer PIC moves the id and must still be found.
        let mut body = 0u32.to_le_bytes().to_vec();
        body.extend_from_slice(&6u16.to_le_bytes());
        body.extend_from_slice(b"123456");
        body.extend_from_slice(&205u32.to_le_bytes());
        assert_eq!(SelectCharacterRequest::parse(&body).unwrap().character_id, 205);
    }

    #[test]
    fn select_character_request_rejects_a_short_body() {
        assert_eq!(SelectCharacterRequest::parse(&[0, 0, 0, 0, 1, 0, b'.']), None);
        assert_eq!(SelectCharacterRequest::parse(&[]), None);
    }
}
