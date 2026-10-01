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
        // **Numbered from 1, as a player counts them.** The client never formats a channel
        // number where a player reads one: whispers, `/find`, the buddy window and a friend's
        // status line all hand the index to `FUN_142cb92f0`, which returns THIS string out
        // of the name array at `singleton+0x2cb8` verbatim. Named `{name}-{i}` until
        // 2026-09-25, so a friend on the first channel read "Scania-0" while Change Channel
        // - which draws its own labels - called the same channel 1. The owner: *"can we please
        // show players as channels 1 and 2? Since that's what the UI says in change
        // channels."* Only the NAME moves; every index on the wire stays 0-based.
        put_str(&mut out, &format!("{name}-{}", u16::from(i) + 1));
        out.extend_from_slice(&0u32.to_le_bytes()); // user count
        // The client reads exactly four u8s here - confirmed in its own decoder
        // `FUN_141b2fac0`, the login stage's `case 0xb`, which reads them into chan+0x0c,
        // +0x10, +0x14 and +0x18. These were all zero until 2026-08-19, which told the
        // client that **every** channel was channel 0 of world 0, and the Change Channel
        // dialog then listed none at all.
        //
        // `[world, index, ...]` is [I] - the shape matches this packet family - but the
        // **fourth byte is [D] and it is the one that makes a row clickable**. The chain,
        // every link read off the listing (`research/channel-select.md`):
        //
        //     chan+0x18  ->  vecB[i]  (FUN_141b2c7c0 at 141b2ca89)
        //                ->  singleton+0x2cc8  (FUN_142cb8e10, arg 6, at 142cb8ed9)
        //                ->  FUN_142cb9510(i), which returns arr[i]
        //
        // `FUN_142cb9510` has six callers - Draw, the mouse hit test, the Change button and
        // three keyboard navigators - and **all six test the result with a bare
        // `test eax, eax`**. There is no enum and no magic value: any non-zero int enables
        // the row. The mouse gate at `142a31810` takes `je` on zero, which is precisely
        // "the click produces nothing on the wire", and that is what the owner measured when they
        // clicked CH.2 and the capture contained no new opcode at all.
        //
        // **The third byte stays 0.** It feeds a *different* array, +0x2cc0 via
        // `FUN_142cb9490`, read as the opposite polarity at `142a317f1` - non-zero there
        // blocking the row. That reading is untested, and the fourth byte's matching
        // reading has now been falsified by a run, so treat this one as equally unproven.
        //
        // Not the user count: enumerating every memory operand in `FUN_141b2c7c0` and
        // dropping the rsp-based ones leaves `[rax]` x4, `[rax+0x14]` and `[rax+0x18]` and
        // nothing else, so the `u32` above reaches no gate. An earlier note in `STATUS.md`
        // proposed a non-zero user count as the cheapest experiment; that was wrong.
        out.extend_from_slice(&[world_id, i, 0, CHANNEL_ENABLED]);
    }
    out.extend_from_slice(&0u16.to_le_bytes()); // balloonCount
    out.extend_from_slice(&0u32.to_le_bytes());
    out.push(0); // hasExtra
    out
}

/// The fourth trailing `u8` of a channel entry. **Back to `0`, and that is a measurement.**
///
/// A static read of `FUN_142cb9510` concluded this byte was the Change Channel row's enable
/// flag and that any non-zero value would light it up. It was set to `1` and **the run of
/// 2026-08-19 came back with the dialog completely empty** - no rows at all, where `0` had
/// listed CH.1 and CH.2. The login server had advertised both channels and the world list
/// went out, so the count reached the wire; the byte itself emptied the list.
///
/// **So the derivation is falsified as stated.** The chain it read - `chan+0x18` ->
/// `vecB[i]` -> `singleton+0x2cc8` -> `FUN_142cb9510`, six callers all testing with a bare
/// `test eax, eax` - may every link be right and still not mean what it was taken to mean,
/// because something upstream drops the entry before it ever reaches that array. A byte that
/// *hides* a channel would produce exactly this; so would a length or a type field.
///
/// What is still measured, and is the only firm ground here:
///
/// | 4th byte | what the owner saw |
/// |---|---|
/// | `0` | CH.1 and CH.2 both listed. CH.2 grey, and clicking it sends **nothing** |
/// | `1` | **no channels listed at all** - but see below, this was confounded |
///
/// # Why it is `1` again, and why that is not a repeat of the same mistake
///
/// The `1` row above was measured **before the priming fix**. At that time the dialog was
/// empty by default and populated only by accident - `research/channel-select.md` section 9
/// explains the mechanism - so "1 emptied it" and "0 populated it" are **both** consistent
/// with the byte having no effect at all on the row count. The byte was un-measured, not
/// disproven, and it could not be measured until rows appeared reliably. They now do:
/// The owner saw CH.1 and CH.2 on 2026-08-20.
///
/// What has since been read rather than guessed (`research/channel-two-greyed.md`):
///
/// * `singleton+0x2cc8` is a **dword array, stride 4**, indexed by channel - `142cb9528
///   mov eax,[rcx+rax*4]`. Not a bitmask and not a byte array, which is the shape two
///   earlier wrong answers in this project assumed.
/// * A whole-image `fieldrefs.py 0x2cc8 --write` finds **one fill in the entire image**,
///   `142cb8ed9`, which steals a `std::vector<int>`'s buffer - so the array's length is the
///   channel count for free.
/// * This byte is what lands in it: per channel the entry ends `u8 worldId, u8 index,
///   u8, u8`, and the fourth is the one read into `chan+0x18`.
/// * **Draw and click read the same predicate.** All six callers of `FUN_142cb9510` test
///   its result with a bare `test eax,eax`, and the function is in no pointer table - so
///   there is no draw/click split and one byte moves all six.
///
/// **The third byte stays `0`** - it is the adult-channel flag, and its gate needs
/// `singleton+0x2250 >= 18` where that field is initialised to `-1` and its only setter has
/// no callers. That gate can never open.
///
/// **Enabling this makes CH.2 clickable, which makes the client able to send `0x00D2`.**
/// That is why `crates/world` answers it now; an unanswered packet freezes the whole UI.
pub const CHANNEL_ENABLED: u8 = 1;

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

/// The client picking a world on the **WorldSelect** screen.
///
/// **Captured 2026-08-19** in `research/fixtures/world-select-0076-login.log`, the first run
/// that ever reached that screen. 171 bytes, and unanswered it leaves the client on
/// "Connecting to server..." forever - the "always answer" rule, on a screen this server had
/// never shown before.
///
/// The body is a machine report. Plainly visible in the capture: `7f 00 00 01` (the local IP,
/// 127.0.0.1), then a `u16`-length CPU string ("<the CPU model>..."), an
/// OS string ("<the OS name>"), memory figures, then the timezone
/// ("Eastern Standard Time"), country ("US") and locale ("en-US") as length-prefixed strings.
///
/// **The head is not parsed and the chosen world and channel are not read out of it.** The
/// reference's `handleSelectWorld` decodes `type, worldId, channel` from the first three
/// bytes and its tail (localIP, cpuName, osName, ram) lines up with ours - but its head is
/// longer than what is on the wire here, so the offsets do not transfer. Reading the choice
/// out of this packet is what the **channel swap** will need, and it should come from the
/// client's own builder rather than from the reference.
pub const CLIENT_SELECT_WORLD: u16 = 0x0076;

/// The client sending a line of chat.
///
/// Captured 2026-08-19 - the owner typed "Hello" into the All tab and this went out:
///
/// ```text
/// e7 5b 64 05   u32, a counter or tick - not read
/// 05 00         u16 length
/// 48 65 6c 6c 6f  "Hello"
/// 03            u8, the tab - "All" was 3
/// ```
///
/// Nothing froze when it went unanswered, so chat is fire-and-forget.
pub const CLIENT_CHAT: u16 = 0x00E7;

/// The text of a [`CLIENT_CHAT`] body, or `None` if it is too short or not valid UTF-8.
///
/// The leading `u32` is skipped rather than interpreted - it looks like a tick and nothing
/// depends on it.
pub fn parse_chat(body: &[u8]) -> Option<String> {
    const LEN_AT: usize = 4;
    let len = u16::from_le_bytes(body.get(LEN_AT..LEN_AT + 2)?.try_into().ok()?) as usize;
    let text = body.get(LEN_AT + 2..LEN_AT + 2 + len)?;
    String::from_utf8(text.to_vec()).ok()
}

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
    /// Experience toward the next level.
    ///
    /// `u64` because that is the width the record reads - see the field list on
    /// [`character_record`] - not because anything here counts that high.
    ///
    /// **There is no SP field beside this one, and that is deliberate.** The stat block
    /// forks on the job: non-extended jobs send a plain `u16 sp`, but extended-SP jobs send
    /// a *count followed by per-pool entries*, and job `0` - the default, and every
    /// character this project has - takes the extended branch. See [`uses_extended_sp`].
    /// A single `sp: u16` would therefore be unrepresentable for exactly the characters we
    /// have, so the pool encoding gets read before SP gets a field. Nothing needs it until
    /// job advancement.
    pub exp: u64,
    pub map_id: u32,
    /// Which portal on [`Self::map_id`] the character stands at.
    ///
    /// Not persisted - it is per-arrival, not per-character. `0` is the map's spawn point,
    /// which is where a login should put you. A portal walk sets this to the index of the
    /// portal named by the source portal's `tn`, so the character arrives at the matching
    /// door rather than back at the spawn.
    pub portal: u8,
    /// **Fame**, stat row 22 of the record, `characters.fame`. Signed: a defame can take it
    /// below zero. The two arrows on another player's Character Info window move it -
    /// `store::fame` has the rules. Added 2026-09-18; a literal 0 on the wire before that.
    pub fame: i32,
    /// `(slot, itemId)` pairs for the avatar's visible equipment.
    pub equips: Vec<(u8, u32)>,
    /// What is in the **Equip tab of the bag** - owned, not worn.
    ///
    /// This is the field that makes an unequip survive a map change. Taking an item off
    /// used to move it on screen and nowhere else, so the next `SetField` re-dressed the
    /// character from the `equipment` rows, which had never changed.
    ///
    /// It is **not** in the `characters` table - it lives in `inventory`, keyed by
    /// `(character_id, inv_type, slot)`, and `crates/store/src/character.rs` skips it in the
    /// destructure for that reason. Only the Equip tab is here because only the Equip tab is
    /// in the character record: see [`equipped_block_with_bag`] for why the other three
    /// lists after it are not bags at all.
    pub equip_bag: Vec<crate::bag::BagEquip>,
    /// How many slots each of the six inventories has - the size of the bag.
    ///
    /// **The client does not assume this; it is told.** See [`INVENTORY_SLOT_ORDER`] for
    /// what each index is and [`inventory_size_block`] for how it reaches the wire. It is
    /// on the character rather than in config because a slot count is per-character and
    /// increasable - buying slots is a thing this game does - so a constant here would be
    /// a value that could never go up.
    pub inventory_slots: [u16; INVENTORY_COUNT],
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
            exp: 0,
            map_id: START_MAP_ID,
            portal: 0, // the map's spawn point
            fame: 0,
            equips: Vec::new(),
            equip_bag: Vec::new(),
            inventory_slots: default_inventory_slots(),
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
/// The 108-byte character-stat block - `FUN_140302e30` with `param_3 == 0`.
///
/// Shared, because it is literally the same client function on both paths: the character
/// list reaches it through `FUN_1403094b0`, and the `SetField` character record reaches it
/// through `FUN_140304b20` at `0x140304e71`. Those are the *only* two callers in the image
/// (`python tools/callers.py 0x140302e30`). Both pass `param_3 = 0`, read out of the
/// caller's `R9` home slot at `[RBP+0x3118]`, so both get this identical layout.
///
/// ## Offset 84 is the map id, and that is measured
///
/// It had been a literal zero here since this block was written, while `map_id` went into
/// the *trailer* at record offset 120 - a placement that rested on nothing, and whose test
/// could not fail (it passed on any `u32 == 1` anywhere in the record). Three independent
/// lines put the map at **offset 84**:
///
/// 1. The `u32` read at `0x14030325e` is mangled into a 12-byte heap object hung off
///    `record + 0xfb`, with the rolling-checksum seed `0x9a65`. `FUN_1402fa540` is the
///    byte-for-byte inverse of that encoder - same `0x2a` chain, same seed, same rotate.
/// 2. `SetField` then calls `FUN_1402fa540(user + 0xf3)` and hands the result to a lookup
///    keyed by `PTR_s_mapName_143a49020`, which dereferences to the ASCII string `mapName`.
///    The neighbouring literal is `MAP` spliced with TAB, CR and LF - the client's usual
///    trick for defeating a string search.
/// 3. It sits immediately before `portal`, which is exactly where `CharacterStat` puts the
///    map. Structural only, and it agrees.
///
/// The all-zero record made the client fade to black and then fault on a scope guard over
/// an uninitialised stack local - map `0` never loaded. `0` is not a map; see
/// [`START_MAP_ID`] and `research/setfield-fault-shape.md`.
///
/// ## This changes the character-list path too, and that is the safer direction
///
/// Both callers share this block, so the list record now carries the map at 84 where it
/// used to carry a literal zero. That is a second change on a screen that already works,
/// which normally argues for leaving it alone - but here it argues the other way:
///
/// * No new code runs. `FUN_140302e30` always wrote this field into the obfuscated slot at
///   `record+0xf3`; only the value being encoded changes.
/// * If anything ever *reads* it, `1` resolves and `0` does not. The reader is
///   `FUN_1403999e0`, which looks a map property up and, **on a miss**, takes a branch
///   containing a non-returning `E_POINTER` call. Map 1 has a `String.wz` entry
///   ("Mushroom Town - West Entrance") so it hits; map 0 has none. See
///   `research/map1-exists.md`.
/// * Keeping the two paths byte-identical is what lets the character list stand as evidence
///   for these bytes at all. Sending different bytes in the record than in the list, which
///   a live client has accepted, would throw that away.
///
/// The failure mode is also early and unambiguous: character select is exercised before the
/// world test on every run, so a regression here shows up before the interesting part.
/// The compact avatar look - what `FUN_1402ee8d0` reads.
///
/// Gender, skin, face, job, hair, then `(slot u8, itemId u32)` pairs terminated by `0xFF`.
/// **These exact bytes have been through this exact client reader on the wire** - they are
/// what dresses the characters on the character-select screen.
///
/// Shared, because the same reader is reached from more than one packet: the character list
/// (`FUN_1403094b0`), and the three inbound channel opcodes `0x0107`, `0x0114` and
/// **`0x0138`** - see [`USER_AVATAR_MODIFIED`].
///
/// # The two maps, and where a cash equip goes
///
/// The owner, 2026-09-12: *"I last had Cobalt wear the entire Ubel outfit, but upon a fresh
/// login, I do not see those cash items equipped anymore."* The database had them - worn
/// slots 105, 107, 108 and 111 - and this function put those numbers on the wire as-is.
/// The reader's loop guard is `(u8)(slot - 1) < 0x1f` (`0x1402ee9c0`), so every one of
/// them was decoded and **discarded**: 5 bytes each, no garment. **[L]**
///
/// The look has two `0xFF`-terminated maps, both indexed 1..=31: the first lands at
/// `look + 0x39 + slot*4`, the second at `look + 0xb9 + slot*4`. The first is the drawn
/// one - its index 0, at `+0x39`, is the hair, which is not optional on any screen.
/// **[D]** from the reader; the reference server calls the two `hairEquips` and
/// `unseenEquips`, which is the same split. So [`look_maps`] puts a cash item worn at
/// `100 + s` into the **first** map at `s`, and the ordinary item it covers, if any, into
/// the **second** map at `s`. A slot with no cash item goes into the first map as before,
/// and a character with no cash equips produces byte-identical output to what has been
/// through this reader on screen since 2026-08-19.
pub fn avatar_look(chr: &Character) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(chr.gender);
    out.push(chr.skin);
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&chr.face.to_le_bytes());
    out.extend_from_slice(&u32::from(chr.job).to_le_bytes());
    out.push(0); // read and discarded
    out.extend_from_slice(&chr.hair.to_le_bytes()); // equipment array index 0
    let LookLayout { drawn, covered, weapon_sticker } = look_layout(&chr.equips);
    for (slot, item) in &drawn {
        out.push(*slot);
        out.extend_from_slice(&item.to_le_bytes());
    }
    out.push(0xFF); // end of the equipment map - the drawn one, look+0x39
    for (slot, item) in &covered {
        out.push(*slot);
        out.extend_from_slice(&item.to_le_bytes());
    }
    out.push(0xFF); // end of the second map - what a cash item covers, look+0xb9
    // Four u32s: look+0x2d, +0x31, +0x35, +0x1c1. The first is the WEAPON STICKER - the cash
    // weapon cover drawn over the real weapon, which stays in slot 11 of the drawn map so
    // the stance still comes from it. [I: the reference server's field order, weaponSticker
    // / weapon / subWeapon; the classic client ships covers (01702001.img) so the field is
    // exercised by the real thing.] the owner, 2026-09-12: with the cover in slot 11 of the
    // drawn map instead, character select drew Ubel's weapon wrong while the field - which
    // dresses from the worn list itself - drew it right.
    out.extend_from_slice(&weapon_sticker.to_le_bytes());
    for _ in 0..3 {
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

/// A cash equip is worn at `100 + its base slot`: the store keeps it there, the client's
/// Deco tab sends `-105` for a cash overall, and both of the look's maps are indexed by the
/// base slot alone.
pub const CASH_EQUIP_SLOT_BASE: u8 = 100;

/// The worn slots a cash equip can occupy - [`EQUIP_SLOTS`] shifted by
/// [`CASH_EQUIP_SLOT_BASE`]. The record's second equipped block (`presence[44]`) stores
/// `1..=31` of them, checked exactly like the first (`LEA EAX,[RCX-1] / CMP EAX,0x1e / JA`
/// at `0x1403066b8`). **[L]**
pub const CASH_EQUIP_SLOTS: std::ops::RangeInclusive<u8> = 101..=131;

/// Split the worn list into the look's two maps: `(drawn, covered)`.
///
/// Every entry in both is a base slot in [`EQUIP_SLOTS`]. A cash equip at `100 + s` is
/// drawn at `s`; the ordinary equip at `s` is then *covered* - sent in the second map so
/// the client still knows what is underneath. Without a cash item at `s`, the ordinary
/// equip is drawn. Anything outside both ranges is dropped here rather than paid for on
/// the wire. Order is by base slot, so the output is stable whatever order the rows came
/// in.
pub fn look_maps(equips: &[(u8, u32)]) -> (Vec<(u8, u32)>, Vec<(u8, u32)>) {
    let LookLayout { drawn, covered, .. } = look_layout(equips);
    (drawn, covered)
}

/// The weapon slot, and the id family of a **cash weapon cover** - the one cash item that
/// does not go into the drawn map. A cover (`1702001` is the classic client's own) is drawn
/// over the real weapon by the look's weapon-sticker field while the weapon itself stays in
/// slot 11, because the stance comes from the weapon's type and a cover has none.
pub const WEAPON_SLOT: u8 = 11;
pub const WEAPON_COVER_IDS: std::ops::Range<u32> = 1_700_000..1_800_000;

/// Where each worn item goes in the look.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LookLayout {
    /// The first map, `look+0x39`: what is drawn, by base slot.
    pub drawn: Vec<(u8, u32)>,
    /// The second map, `look+0xb9`: the ordinary item under a cash one, by base slot.
    pub covered: Vec<(u8, u32)>,
    /// The `u32` after the maps, `look+0x2d`: a cash weapon cover, or 0.
    pub weapon_sticker: u32,
}

/// [`look_maps`] plus the weapon sticker. The owner, 2026-09-12: *"for Ubel's weapon, I do not
/// see the proper rendering of it on character select. (It does show up fine in the game
/// world)"* - the field dresses from the worn list itself; the select screen reads this
/// look, and the cover had been put in slot 11 of the drawn map with the real weapon
/// demoted to the covered map. A cover is not a weapon: it goes in the sticker field, and
/// the weapon keeps its slot. **[I]** from the reference's field order; the screen is the
/// test.
pub fn look_layout(equips: &[(u8, u32)]) -> LookLayout {
    let mut out = LookLayout::default();
    for slot in EQUIP_SLOTS {
        let base = equips.iter().find(|&&(s, _)| s == slot).map(|&(_, id)| id);
        let cash = equips
            .iter()
            .find(|&&(s, _)| s == slot + CASH_EQUIP_SLOT_BASE)
            .map(|&(_, id)| id);
        if slot == WEAPON_SLOT {
            if let Some(cover) = cash.filter(|id| WEAPON_COVER_IDS.contains(id)) {
                out.weapon_sticker = cover;
                if let Some(base) = base {
                    out.drawn.push((slot, base));
                }
                continue;
            }
        }
        match (cash, base) {
            (Some(cash), Some(base)) => {
                out.drawn.push((slot, cash));
                out.covered.push((slot, base));
            }
            (Some(cash), None) => out.drawn.push((slot, cash)),
            (None, Some(base)) => out.drawn.push((slot, base)),
            (None, None) => {}
        }
    }
    out
}

/// Dress a character already standing on a field - **another player's copy of one**.
///
/// **Sent since 2026-09-18**, for a look change (equip, hair, face, pet hat), to everyone
/// else on the map: `Session::broadcast_look_change`. The client's apply is behind a `je`
/// that is always taken in the shipped image (below), and the launcher's
/// `grap_stub::avatarmod` patch turns that jump into two nops, so a launched client applies
/// it in place - no leave, no enter, no blink. On a client without the patch it is still the
/// silent no-op described below.
///
/// **Not for the local character.** Sending it for our own id clears one dword and returns
/// (`research/user-enter-field.md` §0); the player's own redraw is the `0x007C` look bits
/// (`world::session::beautycoupon::look_stat_changed`) and the `SetField` record.
///
/// The history, kept because the reading of the bytes is still right - only its
/// consequence changed once a patch was on the table:
///
/// This was the attempted way around the equipment blocker, and it could never have worked
/// unpatched. The blocker itself is gone - the item decode is at `vtable+0x358`, not the
/// `+0x330` accessor `research/equip-block.md` named, and RTTI was never needed to find it -
/// so the local character is dressed by the `SetField` record; see [`equipped_block`].
///
/// **Why `0x0138` is dead, at byte level.** Its apply is guarded by a call to `0x1407f5ce0`,
/// which is three bytes - `33 c0 c3`, `xor eax,eax; ret` - followed by `TEST EAX,EAX / JZ`,
/// so the branch is always taken and `FUN_1420dd920` is unreachable. **[L]**, by decoding
/// the `rel32` rather than trusting the decompiler. No trigger and no timing would have
/// changed that. An earlier note here blamed an empty pool at `user+0x1200`; that
/// explanation was **wrong**, and the real one is stronger because it needs no run to check.
///
/// The builder is kept because it is one line over [`avatar_look`] and because `0x0107` and
/// `0x0114` read the same compact look - `0x0107` formats it into the client's own log
/// (`"[BP:%02d] %d"` for 32 body parts) and applies nothing, which makes it a potential
/// free read-back instrument if that log is ever found to be readable.
///
/// The original reading of the handler, kept because it is still what the bytes say once
/// the guard is passed **[L]**:
///
/// ```c
/// uVar2 = FUN_1406e8c20(packet);                  // a character id
/// lVar3 = FUN_1429b6c90(DAT_143ac1b90, uVar2);    // look that user up in a pool
/// if (lVar3 != 0) {                               // ONLY if found
///     FUN_1402ee8d0(&look, packet, ..., 0);       // read the compact avatar look
///     FUN_142797be0(lVar3, &look);                // and apply it to that user
/// }
/// ```
///
/// `DAT_143ac1b90` is a user pool looked up by id, and whether the **local** character is
/// in it was never established. It no longer matters: nothing downstream of the lookup can
/// run.
pub const USER_AVATAR_MODIFIED: u16 = 0x0138;

/// Body of a [`USER_AVATAR_MODIFIED`]: the character id, then the compact look.
pub fn user_avatar_modified(chr: &Character) -> Vec<u8> {
    let mut b = chr.id.to_le_bytes().to_vec();
    b.extend_from_slice(&avatar_look(chr));
    b
}

pub fn character_stat_block(chr: &Character, world_id: u32) -> Vec<u8> {
    let mut out = Vec::new();
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
    // Was a hardcoded zero until 2026-08-20. Persisting experience without sending it would
    // have been the "built is not wired" failure exactly: the database would fill up and the
    // bar would sit at zero, which on screen is indistinguishable from nothing working.
    out.extend_from_slice(&chr.exp.to_le_bytes()); // exp
    // A literal 0 until 2026-09-18: `characters.fame` did not exist, so every Character Info
    // window said 0. Signed - a defame can take it below zero - and the row is read as u32.
    out.extend_from_slice(&(chr.fame as u32).to_le_bytes()); // fame
    debug_assert_eq!(out.len(), stat_block_map_id_at(chr.job), "the map id moved");
    out.extend_from_slice(&chr.map_id.to_le_bytes()); // <- the field id
    out.push(chr.portal); // which portal on the map the character arrives at
    out.extend_from_slice(&0u16.to_le_bytes()); // subJob
    out.push(0);
    out.extend_from_slice(&0u64.to_le_bytes()); // FILETIME
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out
}

/// Where the map id sits inside [`character_stat_block`], for a job on the **extended-SP**
/// branch. Use [`stat_block_map_id_at`] rather than this constant unless the job is known.
pub const STAT_BLOCK_MAP_ID_AT: usize = 84;

/// Length of [`character_stat_block`] for an **extended-SP** job. See [`stat_block_len`].
pub const STAT_BLOCK_LEN: usize = 108;

/// Where the map id sits, for a given job.
///
/// The `sp` field is one byte on the extended-SP branch and a `u16` otherwise, so a
/// plain-SP job shifts the map id and everything after it by one. A `debug_assert` in
/// [`character_stat_block`] caught this the first time the map was placed by constant -
/// getting it wrong puts the map id one byte out and desyncs the rest of the block, and
/// the record has no length prefix anywhere to resynchronise on.
pub fn stat_block_map_id_at(job: u16) -> usize {
    if uses_extended_sp(job) { STAT_BLOCK_MAP_ID_AT } else { STAT_BLOCK_MAP_ID_AT + 1 }
}

/// Length of [`character_stat_block`] for a given job: 108, or 109 on the plain-SP branch.
pub fn stat_block_len(job: u16) -> usize {
    if uses_extended_sp(job) { STAT_BLOCK_LEN } else { STAT_BLOCK_LEN + 1 }
}

pub fn character_record(chr: &Character, world_id: u32) -> Vec<u8> {
    let mut out = Vec::new();

    // --- the stat block, FUN_140302e30 with param_3 == 0
    out.extend_from_slice(&character_stat_block(chr, world_id));

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
    out.extend_from_slice(&avatar_look(chr));
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
/// **They are also the only thing that fills the Change Channel dialog, and they are
/// compared before they are used.** `141b32ee2`/`141b32eef` skip the rebuild entirely when
/// *both* equal what the client already holds, and a fresh client holds `(0, 0)` - so
/// `login_result(0, 0, ..)` leaves the dialog with no rows at all. Pick the channel with
/// [`crate::channel::priming_channel`]; the working is `research/channel-select.md` §9.
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

/// The login-result code that draws the client's `notRegisteredID` notice - "this is not a
/// registered ID". `docs/session.md` has the whole table, read out of `FUN_141b2a280`.
///
/// Chosen for the connection the login server cannot attribute to any launcher sign-in: as
/// far as this server is concerned, nobody has logged in, and that is the notice that says so.
/// Codes -1, 6, 8, 9 and 12 would draw the generic "trouble logging in, ask support", which
/// tells the person nothing they can act on.
pub const LOGIN_REFUSED_NOT_REGISTERED: u8 = 5;

/// **The login-result code that draws the client's `loginAlready` notice.**
///
/// The owner, 2026-09-08: *"the server should not allow the same account to login twice, there
/// should be an existing message to say that the account is already logged in."* There is:
/// this is it, and the wording is the client's own.
///
/// # How this was verified against THIS client, not against a reference
///
/// Three independent readings, in decreasing order of what they are worth:
///
/// 1. **The notice exists and says the right thing.** `Login.img /Notice/text/loginAlready` is
///    a real canvas in this client's own WZ - 215x86, format 1 - and rendering it reads
///    **"That ID is already logged in. Please try again later"**. `CLAUDE.md` warns that much
///    of this client's wording is bitmaps rather than strings, so it was rendered rather than
///    grepped:
///
///    ```text
///    wz-dump canvas client-patched/Data/UI/_Canvas/_Canvas_000.wz Login.img <out> loginAlready
///    python tools/wz_png.py <out>
///    ```
///
/// 2. **The gate maps 7 to it.** `FUN_141b267c0` - the same gate [`LOGIN_RESULT`] and
///    [`ACCOUNT_INFO`] go through - switches on `result + 1`, so its `case 8` is wire code 7,
///    and that case loads `L"loginAlready"` (`research/msexe-decoders3.c:178`). It then
///    `break`s to the tail that `return 0`s, which is a **hard refusal**: the client shows the
///    dialog and does not proceed to read the body. Contrast `-1`, `6`, `8`, `9`, which reach
///    the same refusal but with the useless generic `loginTroubleAskSupport`, and `0` and
///    `0x0C`, which proceed.
///
///    Wire code `0x4E` (78) is the second case that names `loginAlready`
///    (`research/msexe-decoders3.c:335`). 7 is used because it is the plainer of the two and
///    the one the twin function corroborates.
///
/// 3. **The twin agrees.** `FUN_141b2a280` is a near-duplicate with its own copy of the table,
///    tabulated in `docs/session.md`, and it maps 7 to `loginAlready` as well. Two separately
///    decompiled functions is corroboration; the reference tree scoring `AlreadyConnected(7)`
///    is **not** evidence and is recorded only because it happens to match - it scored 1 of 8
///    against a held-out control.
///
/// **[D], not [L]: nobody has yet sent this client a `0x0010` with result 7 and watched the
/// screen.** Step 1 is measured, steps 2 and 3 are read out of a decompilation. The client run
/// that would upgrade it costs one launch and is in the test plan.
pub const LOGIN_REFUSED_ALREADY_LOGGED_IN: u8 = 7;

/// A login result that REFUSES. Same shape as [`login_result`] with the result byte set, so a
/// client that reads past the code - the handler gates on it, but a gate is a claim - finds
/// the fields it expects rather than the end of the packet.
///
/// The owner, 2026-09-05: *"enforce login"*. This is what the login server sends to a connection
/// it cannot tie to a launcher sign-in, in place of the fallback account's character list.
pub fn login_refused(code: u8) -> Vec<u8> {
    let mut out = login_result(0, 0, &[]);
    out[0] = code;
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
/// `GameIn`. Confirmed on the wire: it moved a real client to a real channel. Full decode,
/// including the obfuscated tail, in `docs/opcodes.md`.
///
/// **The reference server calls this wire shape `selectCharacterResult`**, and matching it
/// field for field says that is the better name - it carries a result byte, a message
/// string and the obfuscated tail, none of which the reference's own `migrateCommand` has.
/// Both descriptions are of the same packet: answering "I picked this character" *is* how
/// the server hands over a channel. The name here is kept because it says what the packet
/// does for us.
///
/// The reference also numbers *its* `MigrateCommand` `0x11`. **That is a coincidence and
/// nothing rests on it** - see `research/msexe-gamestage-opcodes.md`, where the same
/// alignment scored 1 of 8 on this range when run blind.
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

/// **`SetField`** - the packet that puts a character into a map, and the one the client is
/// waiting for after it migrates. Handler `FUN_142097f80`, 11726 bytes.
///
/// **CONFIRMED on a live client 2026-08-19**: the client's dispatcher entered
/// `FUN_142097f80` *while dispatching opcode `0x01A0`*, reached from `0x141b26567` - inside
/// the **login** stage's `OnPacket`, which is the `if (opcode - 0x1a0 < 4)` chain described
/// below - and passed both early returns.
///
/// Identified statically 2026-08-19. Three lines agree, and the body layout is a separate
/// question written up in `research/msexe-stage-setfield.md`:
///
/// * **Block arithmetic.** `0x0070` is `InventoryOperation`, decoded to assembly, and in
///   the reference source that opcode is the *first* of the character-data block. mscw's
///   game dispatcher `FUN_142cbaa80` starts there and stops at `0x019f` - spanning exactly
///   304 values, the same size as the reference's block. So `0x01a0` begins the next block,
///   which is the stage block, which begins with `SetField`.
/// * **A different dispatcher takes over at exactly that boundary.** `0x01a0..0x01a3` goes
///   to `FUN_142097ee0`, a stage object's *virtual* `OnPacket` rather than the world
///   singleton's. Correct, because `SetField` is addressed to the stage.
/// * **The handler announces a channel change.** It reads a channel id, compares it
///   with the one the world already holds, and on a difference displays a literal
///   that reads "Channel" with a CR, a TAB, a CR and an LF spliced through it -
///   which is why searching the image for the word never found this handler.
///
/// **No stage transition is needed before sending it.** The login stage's own `OnPacket`
/// (`FUN_141b25f30`) ends with
///
/// ```c
/// if (opcode - 0x1a0 < 4)        FUN_142097ee0(this, opcode, packet);   // this packet
/// else if (opcode - 0x51 < 0x1f) FUN_141b82b00(this, opcode, packet);
/// ```
///
/// so it chains to the stage base class for exactly this range. The client will act on
/// `SetField` while it still believes it is in the login stage, which is where it sits
/// while showing "Connecting...".
/// One NPC standing on a field, as the client's NPC pool reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldNpc {
    /// The pool's hash key. **Must be unique per NPC on the field.** A repeat makes the
    /// handler take its "already present" path and return after 4 bytes without reading the
    /// rest of the body, which desynchronises nothing but silently drops the NPC.
    pub object_id: u32,
    /// Goes straight into `Npc/%07d.img`. On a miss the client fires assert `0x431` and then
    /// decodes anyway, down an untested path - so send one that exists.
    pub template_id: u32,
    pub x: i16,
    pub cy: i16,
    /// Foothold id, looked up in the field's foothold map and used without a null check.
    pub fh: u16,
    /// Walk range.
    pub rx0: i16,
    pub rx1: i16,
    /// Facing. Worst case a wrong value makes the NPC face the other way.
    pub f: u8,
}

/// `NpcEnterField` - put one NPC on the field the client is standing in.
///
/// **Routing, established 2026-08-19** (`research/npc-spawn.md`): the NPC pool's dispatcher
/// is `FUN_141e75800` over `0x44F..0x468`, reached from **`FUN_141820080`** - a dispatcher
/// this project had not previously found, covering `0x1a4..0x5ab`, which is the range
/// `research/msexe-gamestage-dispatch.md` left unexplained.
///
/// **The client does not spawn NPCs from the map WZ.** Its field loader walks `life` only to
/// preload `Npc/%07d.img` resources; the only code that produces a populated NPC takes a
/// `CInPacket *`. So every NPC on every field is the server's job.
pub const NPC_ENTER_FIELD: u16 = 0x044F;

/// The body of an [`NPC_ENTER_FIELD`]: **64 bytes**, every field mandatory.
///
/// The layout was proved mechanically rather than by eye - the decoder has a single `RET`,
/// deleting any one read makes the rest unreachable, and that same walk proves address order
/// is execution order. The `raw[8]` length is read from the two dominating `mov edi,8`.
///
/// ## Two of these fields being zero is why NPCs were invisible
///
/// The first version sent zero for everything the client's own decompilation did not name,
/// on a "no readable consumer" argument. The packet was **dispatched** - a watch on
/// `FUN_141e75800` fired on `0x044F` - and no NPC appeared, with no fault and nothing in any
/// log. The layout was not the problem: it was re-derived from the listing read for read and
/// matches, 8 + 38 + 8 + 8 + 2 = the 64 bytes we send.
///
/// The reference server's `Npc::encode` lines up **20-for-20** with the client's read
/// sequence, and names two of those zeros:
///
/// * **read 12 is `isEnabled`.** Zero means the NPC is disabled. It also gates roughly 7000
///   bytes of the decoder - reads 13-20 sit past a gap that size, which is what a flag
///   guarding the rest of the setup looks like.
/// * **read 19 is `alpha`.** Zero means fully transparent.
///
/// A disabled, transparent NPC is created, inserted into the pool, and invisible - exactly
/// what was on screen.
///
/// **Provenance, stated plainly.** The *layout* is `[L]`, read off this client's listing.
/// The *meanings* are `[I]` from a different game version whose opcode guesses scored 1 of 8
/// against a held-out control. What makes this worth acting on is not the reference's
/// authority but the 20-for-20 structural agreement with a read sequence derived
/// independently from this binary, plus the fact that "disabled" and "alpha 0" are precisely
/// the two values that produce an invisible NPC rather than a crash. If NPCs still do not
/// appear, `enabled` and `alpha` are the first things to doubt, not the last.
pub fn npc_enter_field(npc: &FieldNpc) -> Vec<u8> {
    let mut b = Vec::with_capacity(NPC_ENTER_FIELD_LEN);
    b.extend_from_slice(&npc.object_id.to_le_bytes()); //      pool key
    b.extend_from_slice(&npc.template_id.to_le_bytes()); //    Npc/%07d.img
    b.extend_from_slice(&npc.x.to_le_bytes()); //          1   u16 -> +0x3f0
    b.extend_from_slice(&npc.cy.to_le_bytes()); //         2   u16 -> +0x3f4
    // **These two are DRAW ORDER, and `-1` means "let the client decide". Do not touch
    // them.** `research/npc-fade.md`, 2026-08-21.
    //
    // They were previously written up as a per-NPC override of an animation-clock pair,
    // which made them look like a candidate for the fade-in. They are not: they feed
    // `FUN_141e4a5d0`, which writes `((A*3000 - B) * 10) - 0x3FFF8AD5` into Gr2D layer
    // `vtable+0x198` - a `get`/`put` int pair on the layer interface - and stamps it onto
    // every other layer the NPC owns. **[L]** for the arithmetic and the interface; **[D]**
    // that it is a z, from the constant family: every member is `-2^30 + 30000 - k` with k
    // 5..13 (NPCs 5, mobs 9, boss parts 11-13), and one CNpc block uses a constant where
    // `2^30 - it` is exactly `7 * 3000 * 10` - so `A` is a Map.wz object layer index 0..7.
    //
    // **Sending `0`/`0` puts the NPC at layer 0, offset 0 - behind the map's background
    // tiles on most maps.** That is the same shape as the `isEnabled = 0` incident that made
    // every NPC invisible for days, and it is why this carries a warning rather than a value.
    b.extend_from_slice(&(-1i32).to_le_bytes()); //        3   u32 -> +0x5a8  z layer
    b.extend_from_slice(&(-1i32).to_le_bytes()); //        4   u32 -> +0x5ac  z within layer
    // **REVERTED 2026-08-21, on screen evidence, and do not "fix" this again without one.**
    //
    // A static pass established - carefully, and marked **[L]** - that byte 20 is the facing
    // bool (`[npc+0x1ac]`, five reads and every one a `cmp` against zero) and byte 21 is
    // `[npc+0x1a0]`, the animation **action** passed to the displayer's `vtable[0x118]`, the
    // same field `0x0453`'s `nAction` addresses. On that reading the two below were the wrong
    // way round, so they were swapped and byte 21 was given the `0` the constructor defaults
    // to. `research/npc-appear.md`.
    //
    // **The next run killed the client**, and the comparison is as clean as this project
    // ever gets. Two `0x044F` bodies for Sera (template 2) on map 1, differing in exactly
    // these two bytes and identical everywhere else:
    //
    //   ...ffffffff ffffffff 00 01 0800...   they chattered lines 1, 2 and 3 over 23 s
    //   ...ffffffff ffffffff 01 00 0800...   their FIRST chat line -> 0x009E, two C++ throws,
    //                                        CLIENT FAULT 0xC0000005, and no dispatch line
    //                                        for the 0x0453 at all
    //
    // The chat packet itself is innocent: `e9030000ff0000000000` is byte-for-byte the body
    // that had worked 11 seconds earlier on map 40. Only the spawn changed.
    //
    // So the *attribution* may well be right and the conclusion drawn from it was not: `0` is
    // evidently not a safe action for an NPC with a full animation set - Sera has nine nodes
    // including `move`, while Heena (byte 21 = 0 in both runs, fine in both) has no `move`
    // and template 9 on map 40 has no animation nodes at all. That is a hypothesis, marked
    // **[I]**, and it is not what is being shipped. What is shipped is the byte pair that is
    // **measured to work**.
    //
    // `CLAUDE.md`: when a static report contradicts what the screen did, the screen wins.
    b.push(0); //                                          5   u8  -> +0x1ac
    b.push(u8::from(npc.f == 0)); //                       6   u8  -> +0x1a0, see above
    b.extend_from_slice(&npc.fh.to_le_bytes()); //         7   u16 foothold
    b.extend_from_slice(&npc.rx0.to_le_bytes()); //        8   u16 walk range low
    b.extend_from_slice(&npc.rx1.to_le_bytes()); //        9   u16 walk range high
    b.extend_from_slice(&npc.cy.to_le_bytes()); //        10   u16 y again
    b.extend_from_slice(&npc.cy.to_le_bytes()); //        11   u16 y again
    b.push(1); //                                         12   u8  ENABLED
    b.extend_from_slice(&0u32.to_le_bytes()); //          13   u32
    b.extend_from_slice(&0u32.to_le_bytes()); //          14   u32 present item id
    b.push(0); //                                         15   u8  present item state
    b.extend_from_slice(&(-1i32).to_le_bytes()); //       16   u32 present item time
    b.extend_from_slice(&[0u8; 8]); //                    17   raw[8]
    b.extend_from_slice(&0u32.to_le_bytes()); //          18   u32 notice board type
    // **Not ALPHA**, which is what this said and what made it a suspect every time an NPC
    // failed to appear. `FUN_141e57510` *adds* the value to the current clock
    // (`lea esi,[r14+rax]`, `rax` a clock read) and never calls `put_color`. So this reads as
    // "255 ms from now". The value is left alone deliberately - NPCs render correctly with
    // it, and re-guessing a field on the strength of a corrected label is how the last three
    // unit bugs happened. **[L]** that it is not alpha; the meaning is still open.
    b.extend_from_slice(&255u32.to_le_bytes()); //        19   u32 a clock offset, NOT alpha
    b.extend_from_slice(&0u16.to_le_bytes()); //          20   str, empty
    debug_assert_eq!(b.len(), NPC_ENTER_FIELD_LEN);
    b
}

/// Length of an [`npc_enter_field`] body.
pub const NPC_ENTER_FIELD_LEN: usize = 64;

/// **`0x0451` NpcChangeController** - the other way to put an NPC on the field.
///
/// The NPC pool has two creation routes and this project has only ever used one. From
/// `research/npc-spawn.md` §3.1, read off `FUN_141e75800`'s 20-entry jump table **[L]**:
///
/// | opcode | what it does to the new object |
/// |---|---|
/// | `0x044F` NpcEnterField | `or [obj+0x38], 1`, then the 20-field body |
/// | **`0x0451`** | `u8 flag; u32 id;` - when `flag != 0` and the id is **new**: allocate, read `u32 templateId`, load the template, **`mov byte [obj+0x38], 2`**, insert, then **the identical body** |
///
/// So the two differ in exactly one thing: the state byte the object is created with, `1`
/// against `2`. Everything after it is the same decoder, `FUN_141e36b20`.
///
/// # Why this is worth sending
///
/// **Mobs get both halves and NPCs never have.** `on_field_entered` sends `0x03C6`
/// MobEnterField *and* `0x03D2` MobChangeController for every mob, and mobs are confirmed
/// instant on screen. NPCs have only ever been sent `0x044F`, and they fade.
///
/// Two static passes went looking for a fade *field inside `0x044F`* and correctly found
/// none - all 20 reads attributed, no alpha, no visibility timer. Neither asked what **other
/// packets** the pool accepts. That is `CLAUDE.md`'s "enumerate before you filter" exactly:
/// the right list was the pool's opcode table, not the body's field list.
///
/// **This is a hypothesis, not a finding.** Nothing establishes that `obj+0x38` reaches the
/// renderer; what is established is that it is the only difference between the two routes,
/// and that the route we do not use is the one the working case (mobs) uses.
///
/// # The id must be NEW
///
/// `flag != 0` on an id the pool already holds does **not** take the allocate path, and what
/// it does instead is not decoded. Send this for an object id the client has not seen, or
/// send it *instead of* `0x044F` - never after one for the same id.
pub fn npc_change_controller(npc: &FieldNpc, flag: u8) -> Vec<u8> {
    debug_assert_ne!(flag, 0, "flag 0 detaches an NPC instead of creating one");
    let mut b = Vec::with_capacity(NPC_CHANGE_CONTROLLER_LEN);
    b.push(flag);
    b.extend_from_slice(&npc_enter_field(npc));
    debug_assert_eq!(b.len(), NPC_CHANGE_CONTROLLER_LEN);
    b
}

/// `0x0451`. See [`npc_change_controller`].
pub const NPC_CHANGE_CONTROLLER: u16 = 0x0451;

/// One leading flag byte plus the whole [`npc_enter_field`] body.
pub const NPC_CHANGE_CONTROLLER_LEN: usize = 1 + NPC_ENTER_FIELD_LEN;

/// `0x0452` - the NPC pool's **appear-effect switch**, and the first server-reachable lever
/// on the NPC fade that anybody has found.
///
/// # Why this is the fade and the creation packets were not
///
/// The owner, 2026-08-21, after `!npcecho`: *"The copy that was spawned in additionally faded in as
/// well after !npcecho was executed. The original Heena stayed on the screen."* Both packets
/// that create an NPC - `0x044F` and `0x0451` - produced a fade, which looked like proof the
/// server had no lever. It is not: **both of them run the same decoder body**,
/// `FUN_141e36b20`, so a difference between the two could never have shown up.
///
/// Inside that shared body, gated on `DAT_143ad2d30 == 0`: **[L]**
///
/// ```text
/// FUN_14019b780(&DAT_143ad68a0, 0x90)   allocate a 0x90-byte object from a pool
/// FUN_140d13c80(obj, npc)               construct it against this NPC
/// FUN_142df7280(DAT_143ac18d8) -> +0xc  a clock value
/// FUN_140d13cc0(obj, that)              stamp it
/// npc[0xaf] = obj ; FUN_140d13270(obj)  hang it on the NPC and start it
/// ```
///
/// A per-object, timestamped, started-on-creation thing is an animation, and it is created
/// **only when that global is zero**.
///
/// `0x0452` is what sets the global: `u32 v`, then `DAT_143ad2d30 = (v != 0)`, then it walks
/// every NPC in the pool. And the two walks settle what the object is, because they are not
/// symmetrical guesses - `v == 0` runs the **identical** allocate/construct/stamp sequence
/// quoted above on every existing NPC, and `v != 0` calls `FUN_141e64690` to tear it down.
/// One packet both creates and destroys the same thing the creation path creates. **[L]** for
/// the two branches, **[D]** for "that object is the appear animation".
///
/// # The polarity is inverted, which is why this takes a bool
///
/// The wire value is *disable*: `v = 0` leaves the effect **on** (the global stays zero, so
/// creation builds the object), `v != 0` turns it **off**. Sending a raw integer here would
/// invert on somebody eventually, so the argument is what the caller means.
///
/// Nothing about this is confirmed on a screen yet. `!npcfx off` then `!npcecho` is the test.
pub fn npc_appear_effect(enabled: bool) -> Vec<u8> {
    let mut b = Vec::with_capacity(4);
    // `DAT_143ad2d30 = (v != 0)`, and the effect is built only while that global is 0.
    b.extend_from_slice(&u32::from(!enabled).to_le_bytes());
    b
}

/// `0x0452`. See [`npc_appear_effect`].
pub const NPC_APPEAR_EFFECT: u16 = 0x0452;

pub const SET_FIELD: u16 = 0x01A0;

/// The 33-byte **fixed head** of a `SetField`, and nothing after it.
///
/// This is a **delivery probe, not a playable packet.** It exists to answer one question a
/// run can settle cheaply and static reading cannot: does an `0x01A0` actually arrive at
/// `FUN_142097f80`? The handler's two early returns are silent, so "the client did nothing"
/// is otherwise indistinguishable from "the client never got it". Arm a watch on the
/// handler's entry and the question becomes a yes or no.
///
/// It cannot put a character in a map. `characterData` is `0` here, and the branch that
/// does put a character in a map is the other one - it calls `FUN_140304b20`, an 18525-byte
/// character-record decoder that is not decoded yet. See `research/msexe-stage-setfield.md`.
///
/// Layout, every field read off the disassembly:
///
/// ```text
/// u8[8]  server clock base   the client stamps a local tick beside it on receipt
/// u32    channel id          a change from the current one shows the "Channel" toast
/// u8     -> world+0x226c
/// u32    -> world+0x2884
/// u8     if 1, resets a tree on the world object - kept 0
/// u32    read and discarded by the client
/// u32    u32
/// u8     characterData       0 = the short branch; 1 = the full character record
/// u16    string count        0 skips the whole string block in one jump
/// ```
///
/// The trailing zero pad is there because the short branch keeps reading, and a body that
/// runs out mid-read makes the client throw rather than simply stop. The pad is a
/// **guess at a length**, not a decode - the short branch is unread past its first byte.
pub fn set_field_head(clock: u64, channel: u32, pad: usize) -> Vec<u8> {
    let mut b = Vec::with_capacity(33 + pad);
    b.extend_from_slice(&clock.to_le_bytes());
    b.extend_from_slice(&channel.to_le_bytes());
    b.push(0);
    b.extend_from_slice(&0u32.to_le_bytes());
    b.push(0);
    b.extend_from_slice(&0u32.to_le_bytes());
    b.extend_from_slice(&0u32.to_le_bytes());
    b.extend_from_slice(&0u32.to_le_bytes());
    b.push(SET_FIELD_NO_CHARACTER_DATA);
    b.extend_from_slice(&0u16.to_le_bytes());
    debug_assert_eq!(b.len(), SET_FIELD_HEAD_LEN);
    b.resize(SET_FIELD_HEAD_LEN + pad, 0);
    b
}

/// The head is 33 bytes: `8 + 4 + 1 + 4 + 1 + 4 + 4 + 4 + 1 + 2`.
pub const SET_FIELD_HEAD_LEN: usize = 33;

/// `characterData = 0` - the short branch, which carries no character record.
pub const SET_FIELD_NO_CHARACTER_DATA: u8 = 0;

/// `characterData = 1` - the branch that carries a character record, and the only one that
/// can put a character in a map. The short branch **faults this client**, measured.
pub const SET_FIELD_WITH_CHARACTER_DATA: u8 = 1;

/// The smallest `SetField` this client will read without faulting.
///
/// Everything after the 33-byte head is **zero**, and that is not laziness - it is what the
/// census in `research/charrecord-loops.md` says the minimum is:
///
/// ```text
/// head[33]     characterData = 1, string count = 0
/// u32 x3       read before the record decoder is called
/// raw[100]     the record's presence array - all flags clear
/// u8 u32 u8 u32 u8 u8    the record's 7-field minimum, all zero
/// u8 = 0       terminator: at 142098435 a zero here jumps past the next seven reads
/// ...          zeros, so any read past the traced path takes a zero
/// ```
///
/// **Why a zero tail is safe and a short body is not.** The frame carries its own length,
/// so bytes the client never reads are simply ignored - surplus costs nothing. Running
/// *out* of body mid-read makes it throw. Every gate on the traced path skips on zero, so
/// zeros are also the value that keeps it on the shortest path.
///
/// **This is now a fallback, not the packet the server sends.** It is used only when a
/// character cannot be loaded, because an unanswered migration hello freezes the client's
/// whole UI - see [`set_field_with_character`] for the real one.
///
/// **What it cannot do is land the character on a map.** Every presence flag is clear, so
/// the character-stat block never decodes and there is no map id at all. The two questions
/// this doc used to call open are both settled: `presence[0]` switches the stat block on
/// (`research/charrecord-presence-map.md`) and the map id sits at stat-block offset 84
/// (`research/charstat-layout.md`). Both are confirmed on screen - a character stands on
/// map 1.
pub fn set_field_minimal(clock: u64, channel: u32) -> Vec<u8> {
    let mut b = set_field_head(clock, channel, 0);
    b[SET_FIELD_CHARACTER_DATA_AT] = SET_FIELD_WITH_CHARACTER_DATA;
    b.resize(SET_FIELD_HEAD_LEN + SET_FIELD_MINIMAL_TAIL, 0);
    b
}

/// The character record `FUN_140304b20` reads, with the character-stat block switched on.
///
/// **The bare region is 224 bytes** for an extended-SP job, 225 for a plain-`u16 sp` one -
/// that is the figure the whole walk was controlled against, and it is what this table
/// describes. What the server actually sends is longer, because two more presence bytes are
/// set: `+12` for the inventory sizes and `+11` plus 127 per equip for the equipped list.
/// A dressed level-1 with four equips is 755 bytes. Layout from
/// `research/charrecord-flag7.md`, which re-assembled the region out of the binary and
/// diffed all 322 bytes against `client-patched/MapleStory.exe` with zero mismatches:
///
/// ```text
/// off  len  what                                        value
///   0  100  presence array (field 1)                    byte 0 = 1, rest 0
/// 100    1  u8   -> dword [param_1+0x1001]              0
/// 101    4  u32  a duration added to a tick             0
/// 105    1  u8   loop #1 count                          0  (its body reads a u32)
/// 106    4  u32  loop #2 count                          0  (its body reads u32 + raw 8)
/// 110    1  u8   bool; non-zero pulls in two more loops 0
/// ---------- gate #7 fires here, at 0x140304e49 ----------
/// 111  108  the stat block, param_3 == 0                character_stat_block
/// 219    1  u8   -> dword [param_1+0x118b]              0
/// 220    1  u8   optional-string flag A                 0 skips the string
/// 221    1  u8   optional-string flag B                 0
/// 222    1  u8   optional-string flag C                 0
/// ---------- gate #7 region ends ----------
/// 223   12  six u16 inventory sizes, loop #6            24 each  (gate #13, presence[7])
/// ---------- gate #6's second region: the equipped list ----------
/// 235    1  u8   ungated, after every gate              0
/// ```
///
/// The last row is genuinely last: it is the read at `0x140308b3f`, past every gate, so the
/// equipped list goes **in front of it** rather than after. The two gated blocks are drawn
/// in the order the client reads them, which is the order their gates appear in the
/// function - loop #6 at `0x140305de8`, then the equipped list's gate at `0x1403061a0`.
///
/// **Why `presence[0]`.** `FUN_1402fa9a0` is a 100-byte bytewise AND: a gate computes
/// `out[i] = presence[i] & key[i]` and runs its block if any byte of `out` is set. Each
/// gate's key is built at startup by a CRT dynamic initialiser that zeroes 100 bytes and
/// then sets exactly **one** to `1`, so a gate fires iff its one presence byte is set. The
/// gate guarding the stat decoder is entry 7 and its byte is index **0**. Full working and
/// the whole 40-row table: `research/charrecord-presence-map.md`.
///
/// The zeros are not laziness - every count and flag here is one the client uses to *skip*,
/// so zero is the value that keeps it on the shortest path. The one field that must not be
/// zero is the map id inside the stat block; see [`character_stat_block`].
pub fn character_record_for_set_field(chr: &Character, world_id: u32) -> Vec<u8> {
    let bare: Vec<(u8, u32, EquipStats)> =
        chr.equips.iter().map(|&(s, i)| (s, i, EquipStats::default())).collect();
    character_record_for_set_field_with(chr, world_id, &bare)
}

/// The same record, with each equip's stats supplied by the caller.
///
/// **The stats are not the character's to know.** They come from the item *template* in
/// `Character.wz`, which `crates/world` loads and `crates/store` does not persist - a stat
/// in the database would be a second source of truth for a value the client already has a
/// copy of. So the caller resolves them and passes them in, and
/// [`character_record_for_set_field`] keeps the all-zero behaviour that was confirmed on
/// screen on 2026-08-19.
pub fn character_record_for_set_field_with(
    chr: &Character,
    world_id: u32,
    equips: &[(u8, u32, EquipStats)],
) -> Vec<u8> {
    let mut out = vec![0u8; PRESENCE_ARRAY_LEN];
    out[PRESENCE_CHARACTER_STAT] = 1;
    out[PRESENCE_EQUIPPED] = 1;
    out[PRESENCE_INVENTORY_SIZE] = 1;
    out.extend_from_slice(&[0u8; 11]); // the six head fields at 100..111, all zero
    debug_assert_eq!(out.len(), STAT_BLOCK_AT);
    out.extend_from_slice(&character_stat_block(chr, world_id));
    // 219..223: one u8, then the three optional-string flags. A zero flag skips its string.
    out.extend_from_slice(&[0u8; 4]);
    // Loop #6 runs here, six turns, and gate entry 13 takes one u16 a turn because
    // presence[7] is set. It is BEFORE the equipped list: the loop is at 0x140305de8 and
    // the equipped list's gate is at 0x1403061a0. The record has no length prefix and no
    // resync point, so this order is the whole of what makes the bytes after it readable.
    out.extend_from_slice(&inventory_size_block(&chr.inventory_slots));
    // Gate entry 6 fires here, because presence[2] is set. The Equip tab's contents ride
    // in the same block, sized by the same number the presence[7] block just sent - a
    // position past that is decoded by the client and thrown away.
    out.extend_from_slice(&equipped_block_with_bag(
        equips,
        &chr.equip_bag,
        chr.inventory_slots[0],
    ));
    // The worn CASH equips, behind presence[44]. The block sits at 0x14030661c, after the
    // presence[2] region and before the skill gate (0x140306d28) and both quest gates, so
    // it goes here and nowhere else. Absent when nothing cash is worn, so the record stays
    // byte-identical to the one confirmed on screen for every character that has none.
    if equips.iter().any(|(slot, _, _)| CASH_EQUIP_SLOTS.contains(slot)) {
        out[PRESENCE_CASH_EQUIPPED] = 1;
        out.extend_from_slice(&cash_equipped_block(equips));
    }
    out.push(0); // the final ungated read, at 0x140308b3f
    out
}

/// The presence byte that opens the **second equipped block** - the worn cash equips.
///
/// `research/charrecord-presence-map.md` row 44: key `0x143abeb80`, gate `0x140306632`,
/// region `[0x140306654, 0x140306830)`. The gate itself scans presence bytes 44..99 for
/// any non-zero value (`CMP ECX,0x64 / JB` at `0x14030664a`), so this byte is the lowest
/// that opens it and the one the client's own encoder writes. **[L]**
///
/// The owner, 2026-09-12: a relog undressed Cobalt. The look ([`avatar_look`]) is what draws the
/// avatar, and it was dropping the cash slots; but the *equip window* reads this block, and
/// without it the worn cash items are nowhere in the client at all - not drawn, not listed,
/// and not removable, while the server still holds them in slots 105.. . Both halves are
/// needed and this is the second one.
pub const PRESENCE_CASH_EQUIPPED: usize = 44;

/// The second equipped block, read at `0x140306654..0x140306830`. **[L]**, every row from
/// the listing, and it mirrors the first block's shape exactly:
///
/// ```text
/// u8   flagA                    0x14030665c  MUST be 0: non-zero skips FUN_14030b6f0(6)
///                                            and with it the u16 terminator below
/// per worn cash item:
///   u16 slot                    0x14030668c  BASE slot, 1..=31 (the worn slot minus 100);
///                                            stored at charData + 0x3a8 + slot*0x10, or
///                                            keyed -100-slot when [rbp+0x3118] is set
///   item                        0x1403066ae  FUN_1403095e0 - the same pooled item factory
///                                            as the first block, so equipped_item applies
/// u16  0                        end of the list
/// u16  0                        FUN_14030b6f0(closure, 6): the Deco BAG. Sent empty - the
///                               Deco tab is restored by 0x0070 mode 5 on field entry like
///                               the other bags, and a second copy here would double it
/// u16  0, u16 0                 FUN_14030b9e0(6): indices 0 and 1 only (FUN_140255790),
///                               the 1200 and 1800 ranges. Nothing this server makes goes
///                               there
/// ```
///
/// `9 + entries` bytes. Slots outside [`CASH_EQUIP_SLOTS`] are skipped: the client would
/// decode and discard them, exactly as the first block does with anything outside 1..=31.
pub fn cash_equipped_block(equips: &[(u8, u32, EquipStats)]) -> Vec<u8> {
    let mut b = Vec::new();
    b.push(0); // flagA - 0x140306663 TEST AL / SETNE R14B; R14B set skips b6f0(6)
    for (slot, item_id, stats) in equips {
        if !CASH_EQUIP_SLOTS.contains(slot) {
            continue;
        }
        let base = slot - CASH_EQUIP_SLOT_BASE;
        b.extend_from_slice(&u16::from(base).to_le_bytes());
        b.extend_from_slice(&equipped_item(*item_id, stats));
    }
    b.extend_from_slice(&0u16.to_le_bytes()); // end of the worn cash list
    b.extend_from_slice(&0u16.to_le_bytes()); // FUN_14030b6f0(6) - the Deco bag, empty here
    b.extend_from_slice(&[0u8; 4]); // FUN_14030b9e0(6) - two lists, indices 0 and 1
    b
}

/// The fixed cost of [`cash_equipped_block`]: `flagA` plus four `u16` terminators.
pub const CASH_EQUIPPED_BLOCK_OVERHEAD: usize = 1 + 2 + 2 + 4;

/// Field 1 of the character record: the presence array that gates 43 blocks.
pub const PRESENCE_ARRAY_LEN: usize = 100;

/// The presence byte that switches on the character-stat block - gate entry 7, whose key
/// mask is all-zero except this index. The key byte's value is exactly `1`, so any value
/// with bit 0 set works here; `1` is also what the reference server sends.
pub const PRESENCE_CHARACTER_STAT: usize = 0;

/// Where [`character_stat_block`] starts inside the character record.
pub const STAT_BLOCK_AT: usize = 111;

/// The presence byte that switches on the **equipped-item list** - gate entry 6.
///
/// Read the same way [`PRESENCE_CHARACTER_STAT`] was: the gate at `0x1403061a0` carries key
/// `0x143abedb0`, whose CRT initialiser at `0x140023442` is `MOV byte ptr [0x143abedb2],1`,
/// so that key mask is all-zero except byte **2**. **[L]**
///
/// **Setting this byte opens three list readers, not one.** The gated region calls
/// `FUN_14030b6f0` and `FUN_14030b9e0` right after the equipped loop and **both re-gate
/// through this same presence byte** - `FUN_1403023d0(out, 1)` resolves to key
/// `0x143abdb20`, whose initialiser also sets byte 2. `FUN_14030b9e0` then runs its reader
/// three times, for outer index 2, 3 and 4. So this byte costs **four extra `u16`
/// terminators** beyond the equipped list's own, and the record has no length prefix and no
/// resync point - omit them and everything after desynchronises silently.
///
/// Full working: `research/naked-character.md`.
pub const PRESENCE_EQUIPPED: usize = 2;

/// The presence byte that switches on the **six inventory sizes** - gate entry 13.
///
/// This is the bag. With this byte clear the client sizes each inventory from whatever the
/// array already holds, and the server never gets a say.
///
/// **Read off the listing rather than inferred.** `FUN_140304b20` runs a loop of exactly
/// **six** turns - `MOV R15D,0x6` at `0x140305def`, `SUB R15,0x1 / JNZ` at `0x14030608e` -
/// over six pointers based at `char+0x5d8`, stepping `8` a turn (`ADD RSI,0x8`). Inside
/// that loop, gate entry 13 - key `0x143abf0c0`, whose CRT initialiser sets byte **7** -
/// guards exactly one packet read: the `u16` at `0x140305e48`. A sweep of the whole loop
/// body, `0x140305de8` to `0x140306098`, finds **no other call to any of the eight read
/// primitives**, so this byte costs six `u16` and nothing else. **[L]**
///
/// **What the value means.** The client reads `V`, and if `V` differs from the array's
/// current `count - 1` it resizes the array to `V + 1` (`INC EDX` at `0x140305e69`, then
/// `FUN_14030ee00`). It then walks slot indices `0..=V`. So `V` is the **slot count**, with
/// index 0 the unused hole that makes MapleStory's slots 1-based.
///
/// **And this is why a bag can refuse everything.** When the byte is clear the value used
/// is `count - 1` computed at `0x140305e0f` - and the two instructions before it,
/// `TEST RAX,RAX / MOV EAX,EDI`, make `count` **zero** for a null array, so the default is
/// **-1**. The very next thing the client does with it is `CMP dword [RSP+0x60],0 / JL` at
/// `0x140305f09`, which skips that inventory's whole slot walk. A negative bag has no slots
/// at all. **[L]** for the arithmetic; whether these arrays *are* null at decode time is
/// **not** established - that lives in the constructor of the character-data object, which
/// has not been read. It is the leading explanation for the unequip that never reaches the
/// wire, not a proven one.
pub const PRESENCE_INVENTORY_SIZE: usize = 7;

/// How many inventories the record sizes: **six**, from the loop's own trip count.
pub const INVENTORY_COUNT: usize = 6;

/// What each index of [`Character::inventory_slots`] is.
///
/// The order is the client's, taken from the **jump table at `0x1403093bc`** that picks each
/// turn's gate key from the loop counter. Its six entries resolve to keys `0x143abedb0`,
/// `ed40`, `ecd0`, `ec60`, `ebf0`, `eb80` - gate entries 6, 5, 4, 3, 2, 1, whose presence
/// bytes are **2, 3, 4, 5, 6 and 44**. **[L]**
///
/// Byte 2 is measured elsewhere: it is the byte that switches on the equipped-item list
/// (`research/naked-character.md`), so index 0 is the equip inventory. **[D]**
///
/// **The names come off the screen.** The owner's inventory window, 2026-08-19, has exactly six
/// tabs: `Equip`, `Use`, `Set Up`, `Etc`, `Cash`, `Deco`. Six tabs against a loop whose trip
/// count is the literal `6`, in an order whose first five match the reference server's
/// `DBChar` ordinals 2..6 - so the tabs and the turns are the same six things. **[D]**
///
/// `Deco` is the one that mattered: this table said "not identified at all" until the
/// screenshot, and no amount of further reading would have named it, because the name is
/// not in the decoder - it is in the UI.
///
/// **Nothing on the wire depends on these names**, because the server sends every inventory
/// the same size. A wrong name here costs a comment, not a byte - which is why the table
/// was safe to ship with a hole in it.
pub const INVENTORY_SLOT_ORDER: [&str; INVENTORY_COUNT] =
    ["equip", "use", "setup", "etc", "cash", "deco"];

/// The bag a new character gets: **30 slots** in each inventory.
///
/// **30 because that is what this client's own window holds**: the owner, 2026-08-19, sending a
/// screenshot of the inventory - six rows of five, filled, with nothing below the fold.
///
/// It was 24 for one commit. 24 is the classic MapleStory starting bag and it is what the
/// reference server uses, and it was wrong here for the same reason the reference is wrong
/// about most numbers: this is a different game version, and its window is five wide rather
/// than four. Nothing measured ever said 24; it was carried over from a family resemblance.
///
/// **What the screenshot does NOT settle** is whether those thirty cells are the array's
/// size or the window's. A viewport drawn 5x6 with a scrollbar would look identical at any
/// slot count above 30. That is the question the next client run answers, and it is why
/// `--inventory-slots` exists - see `research/inventory-slots.md`.
///
/// It is a **default, not a limit**: the field is a `u16` and the client resizes to whatever
/// arrives, which is what makes buying slots expressible.
pub const DEFAULT_INVENTORY_SLOTS: u16 = 30;

/// The Deco tab's size, which is also its ceiling: **150, from the first login.**
///
/// The owner, 2026-09-12: *"The deco inventory does not have expansion coupons because it starts
/// out at the maximum 150 slots."* The other five start at [`DEFAULT_INVENTORY_SLOTS`] and
/// grow by coupon; this one has no coupon in the client (`SlotCoupon` names Equip, Use, Set
/// Up, Etc, Cash and storage) and never needs one. 30 here was the uniform default nobody
/// chose for this tab - a cash equip past the thirtieth would have been refused at placement.
pub const DECO_INVENTORY_SLOTS: u16 = 150;

/// The Cash tab's size, which is also its ceiling: **150, from the first login.**
///
/// The owner, 2026-09-13: *"The user's Cash tab in the Player Inventory should also come with 150
/// slots by default, just like the Deco tab."* So it is [`MAX_INVENTORY_SLOTS`] from the start,
/// exactly as Deco is, and the Cash slot coupon has nothing left to add - `session::cashitem`
/// already answers a coupon at the ceiling with *"already at the maximum of 150 slots"* rather
/// than failing, so no other path changes.
pub const CASH_INVENTORY_SLOTS: u16 = 150;

/// The six starting sizes, indexed like [`Character::inventory_slots`]: four at
/// [`DEFAULT_INVENTORY_SLOTS`], **Cash and Deco full** at [`CASH_INVENTORY_SLOTS`] /
/// [`DECO_INVENTORY_SLOTS`]. The order is [`INVENTORY_SLOT_ORDER`], so those are the last two.
pub const fn default_inventory_slots() -> [u16; INVENTORY_COUNT] {
    let mut slots = [DEFAULT_INVENTORY_SLOTS; INVENTORY_COUNT];
    slots[INVENTORY_COUNT - 2] = CASH_INVENTORY_SLOTS;
    slots[INVENTORY_COUNT - 1] = DECO_INVENTORY_SLOTS;
    slots
}

/// The largest slot count a bag can reach: **150**.
///
/// The owner, 2026-08-19: *"The inventory slots starts at 30 default and is expandable up to 125
/// slots using slot expansion USE items of that type of tab."* - and the client's own Etc
/// coupon text, on screen 2026-09-11: *"You can have up to 150 slots."* The client's number
/// wins over the remembered one; the Deco tab is 150 from the start
/// ([`DECO_INVENTORY_SLOTS`]), which is the same ceiling seen from the other side.
///
/// It was 100 for one commit, which was mine and arbitrary, then 125. Nothing in the decoder
/// bounds `V` at all - it is a `u16` and the resize takes whatever arrives - so this cap
/// exists only so a typo cannot ask the client to allocate 65535 slots and walk them six
/// times over.
pub const MAX_INVENTORY_SLOTS: u16 = 150;

/// **Never send fewer than 30.** The owner, 2026-08-19: *"the minimum number has to be 30, setting
/// it below 30 has no use."* The window draws a fixed 5x6 grid, so a smaller number cannot
/// show on screen - which is why the run at 10 could not answer the question it was designed
/// for, and why the WATCH is what answered it instead.
pub const MIN_INVENTORY_SLOTS: u16 = DEFAULT_INVENTORY_SLOTS;

/// The six inventory sizes, in the order the client reads them.
///
/// Twelve bytes, always - six `u16`, no count and no terminator, because the loop's trip
/// count is baked into the client at `0x140305def`. Each value is clamped to
/// [`MAX_INVENTORY_SLOTS`]; see there for why that cap is ours rather than the client's.
pub fn inventory_size_block(slots: &[u16; INVENTORY_COUNT]) -> Vec<u8> {
    let mut b = Vec::with_capacity(INVENTORY_SIZE_BLOCK_LEN);
    for &count in slots {
        let count = count.clamp(MIN_INVENTORY_SLOTS, MAX_INVENTORY_SLOTS);
        b.extend_from_slice(&count.to_le_bytes());
    }
    b
}

/// What [`inventory_size_block`] adds to the record: six `u16`, unconditionally.
pub const INVENTORY_SIZE_BLOCK_LEN: usize = INVENTORY_COUNT * 2;

// ---------------------------------------------------------------------------------------
// The quest blocks. Bodies live in `crate::quest`; the presence bytes and the record
// assembly are here, beside the other two presence bytes. Full working, with the instrument
// controls, is `research/quest-state.md`.
// ---------------------------------------------------------------------------------------

/// The presence byte that switches on the **started-quest list** - gate entry 19.
///
/// Read the way [`PRESENCE_INVENTORY_SIZE`] was, out of the key's own CRT initialiser rather
/// than assumed. The gate at `0x1403074a3` carries key `0x143abf360`, and the initialiser at
/// `0x140023690` is `memset(0x143abf360, 0, 100)` followed by
/// `MOV byte ptr [0x143abf369],1` - `0x369 - 0x360` = byte **9**. **[L]**
///
/// **A second, independent measurement lands on the same index.** `FUN_1402e5a30` is the
/// client's own *encoder* for this record, 38 calls to the same gate helper
/// `FUN_1402fa9a0` - and it uses a **different key table**: its started-quest gate at
/// `0x1402e719e` carries
/// key `0x143abe0d0`, whose initialiser at `0x140022ba0` sets `[0x143abe0d9]`, byte **9**
/// again. Two tables, two sets of initialisers, one byte index. **[L]**
///
/// **What it costs**: the region `[0x1403074c4, 0x14030756a)`, six reads - the same six
/// `research/charrecord-loops.md` §5 row `#19` counted from a different direction. The three
/// helpers it calls (`FUN_1402e0fe0` clear, `FUN_1402e0c30` insert, `FUN_1402e0ec0` erase)
/// read **zero** packet bytes, checked transitively with `tools/reads.py` at depth 3.
///
/// **Why it is quests**: the helpers drive `charData+0x1273`, a hash map keyed by quest id;
/// `+0x1273` is read by `FUN_14070fe30` / `FUN_140711b40` / `FUN_140711e50`, the three
/// `.pdata` functions surrounding `FUN_140711d70` - the quest requirement checker
/// `crate::script` already documents from the `0x0151` side - and by `FUN_141f0e110`, in the
/// same block as the `0x0151` builder itself. **[D]**
pub const PRESENCE_QUEST_STARTED: usize = 9;

/// The presence byte that switches on the **completed-quest list** - gate entry 20.
///
/// Same method: gate `0x14030757b`, key `0x143abf3d0`, initialiser `0x140023670` writing
/// `[0x143abf3de]` - byte **14**. The encoder agrees from its own table: gate `0x1402e7414`,
/// key `0x143abe140`, initialiser `[0x143abe14e]`, byte **14**. **[L]**
///
/// Region `[0x140307596, 0x140307633)`, six reads, row `#20`. Its collection is
/// `charData+0x1347`, read back by `FUN_1402e3390` - which has **31 call sites in 23
/// functions**, i.e. the quest UI.
pub const PRESENCE_QUEST_COMPLETED: usize = 14;

/// The character record with the quest blocks in it.
///
/// **Why this is a separate function rather than a parameter on
/// [`character_record_for_set_field_with`]**: that function's output is byte-for-byte what
/// has been confirmed on screen, and this one is built by *appending* to it rather than by
/// rebuilding it. The two quest blocks go between the equipped list and the final ungated
/// `u8`, which is the record's last byte - so the whole edit is "pop the tail, add the
/// blocks, put the tail back". Nothing that already works is re-derived.
///
/// ```text
/// ...  ..  the equipped list                presence[2],  gate 0x1403061a0
///  ..  ..  STARTED quests                   presence[9],  gate 0x1403074a3
///  ..  ..  COMPLETED quests                 presence[14], gate 0x14030757b
///  ..   1  the final ungated u8             0x140308b3f
/// ```
///
/// The order is not a choice: `0x1403061a0 < 0x1403074a3 < 0x14030757b < 0x140308b3f`, and
/// the decoder is one straight run of gates with no back edge between them. **[L]**
///
/// **Nothing lies between the equipped list and the started-quest block for this server**,
/// and the easy version of that claim is wrong, so it is spelled out:
///
/// * The five *static* gates in `[0x140306632, 0x1403074a3)` key on presence bytes 44, 21,
///   27, 8 and 15, all clear here. **[L]**
/// * The two *dynamic* gates in that range select among entries 0-6, whose presence bytes
///   are 0(none), 44, 6, 5, 4, 3 and **2** - and byte 2 is the equipped list's, so one of
///   them **can** fire. It costs nothing anyway: `0x1403068fd`'s region reads nothing, and
///   `0x1403069ef`'s only read (the `u32` at `0x140306a0d`, inside the fixed-3 loop #9) sits
///   behind a per-turn switch selecting presence bytes 4, 5 and 6 - all clear - which also
///   makes loop #10's count zero. **[L]**
/// * And the decisive point is not static: `presence[2]` is already set and the record
///   already decodes on screen, so those gates behave identically before and after this.
///
/// Neither 9 nor 14 is reachable through a dynamic gate, and each appears in exactly one row
/// of the 40-row table in `research/charrecord-presence-map.md`. **[D]**
///
/// **An empty book still costs six bytes** - `01 00 00` twice - and that is deliberate: the
/// bulk flag makes the client *clear* its collections, so a quest the server has dropped
/// stops being shown. See [`crate::quest::QUEST_BLOCK_BULK`].
///
/// > **The record has no length prefix and no resync point.** If these six bytes are wrong
/// > the symptom is an undressed character or no world entry at all - not a wrong quest
/// > list.
pub fn character_record_for_set_field_with_quests(
    chr: &Character,
    world_id: u32,
    equips: &[(u8, u32, EquipStats)],
    quests: &crate::quest::QuestBook,
) -> Vec<u8> {
    character_record_for_set_field_with_quests_and_skills(chr, world_id, equips, quests, &[])
}

/// The record with quests **and skills**.
///
/// # The skill block is `presence[8]`, and it goes before the quest blocks
///
/// Gate `0x140306d28`, key `0x143abf280`, whose single initialiser writes byte **+8**; the
/// client's own *encoder* gates the same block with a different key that also writes byte 8.
/// Its position is after the equipped list (`0x1403061a0`) and before both quest gates
/// (`0x1403074a3`) - and the record has no length prefix and no resync point, so that order
/// is the whole of what makes the bytes after it readable. `research/skills.md`.
///
/// **A character with no skills sends no block and does not set the byte**, so the record is
/// byte-identical to what this server sent before skills existed. That is deliberate: it
/// keeps the change to characters that have actually raised something, which is the only
/// case anyone has looked at.
pub fn character_record_for_set_field_with_quests_and_skills(
    chr: &Character,
    world_id: u32,
    equips: &[(u8, u32, EquipStats)],
    quests: &crate::quest::QuestBook,
    skills: &[crate::skills::Skill],
) -> Vec<u8> {
    let mut out = character_record_for_set_field_with(chr, world_id, equips);
    // The base record's last byte is the ungated read at 0x140308b3f, past every gate this
    // server sets. Every block below comes before it, so they go in front of it.
    let tail = out.pop().expect("the character record is never empty");
    debug_assert_eq!(tail, 0, "the final ungated u8 at 0x140308b3f is the byte being moved");

    if !skills.is_empty() {
        out[crate::skills::PRESENCE_SKILLS] = 1;
        out.extend_from_slice(&crate::skills::skill_block(skills));
    }

    out[PRESENCE_QUEST_STARTED] = 1;
    out[PRESENCE_QUEST_COMPLETED] = 1;
    out.extend_from_slice(&quests.started_block());
    out.extend_from_slice(&quests.completed_block());
    // Block #28, the quest ex records - citizenship (510000) and the Community Board's
    // postings (510001..510004). Presence 16's gate is at 0x140308a36, after the completed
    // block's and before the final byte; presence 10/11/12/15, whose regions sit between,
    // are never set by this server. `research/citizenship-2026-09-27.md` §5.2.
    if !quests.ex.is_empty() {
        out[crate::citizenship::PRESENCE_QUEST_EX] = 1;
        out.extend_from_slice(&crate::citizenship::quest_ex_block(&quests.ex));
    }
    out.push(tail);
    out
}

/// [`set_field_with_character_dressed`], plus the quest blocks.
///
/// Deliberately a parallel function rather than an extra argument: the existing one is the
/// packet that has been confirmed on screen, and a signature change would touch every caller
/// of a thing that currently works.
pub fn set_field_with_character_dressed_quests(
    chr: &Character,
    world_id: u32,
    clock: u64,
    channel: u32,
    equips: &[(u8, u32, EquipStats)],
    quests: &crate::quest::QuestBook,
    skills: &[crate::skills::Skill],
) -> Vec<u8> {
    let mut b = set_field_head(clock, channel, 0);
    b[SET_FIELD_CHARACTER_DATA_AT] = SET_FIELD_WITH_CHARACTER_DATA;
    b.extend_from_slice(&[0u8; 12]); // three u32s the caller reads before the record decoder
    b.extend_from_slice(&character_record_for_set_field_with_quests_and_skills(
        chr, world_id, equips, quests, skills,
    ));
    // 142098435, and the same 384-byte margin set_field_with_character_dressed carries: a
    // zero u8 here jumps past the next seven reads, and surplus bytes are never looked at
    // because the frame carries its own length.
    b.extend_from_slice(&[0u8; 1 + 384]);
    b
}

/// One equipped item on the wire, for item type 1, **with every bitmask zero**.
///
/// It is **125 bytes whichever way `hasCashSN` goes**: the 8 bytes that flag controls are
/// read either by the base decode into `+0x38` or by the equip decode into `+0x4d`. **[D]**
///
/// This is the floor, not the size. Each set bit of the four masks adds its field's width -
/// see [`EquipStats::extra_len`].
pub const EQUIPPED_ITEM_LEN: usize = 125;

/// The `u8` item type that selects the equip decode.
///
/// `FUN_1403095e0` reads this byte and dispatches: 1 to `FUN_14030ddb0` (equip),
/// 2 to `FUN_14030db00` (bundle), 3 to `FUN_14030e340` (pet). Anything else leaves the item
/// null and reads nothing further. **[L]**
pub const EQUIPPED_ITEM_TYPE: u8 = 1;

/// The equip slots the client keeps. A slot outside this range is decoded and **discarded**.
///
/// `LEA EAX,[RCX-1] / CMP EAX,0x1e / JA` at `0x140306229` stores the item at
/// `record + 0x1a8 + slot*0x10` only for `1 <= slot <= 31`. **[L]** So a cash-equip slot
/// costs 127 bytes of wire and achieves nothing *here*; [`equipped_block`] drops them, and
/// they travel in the second equipped block instead - [`cash_equipped_block`], behind
/// [`PRESENCE_CASH_EQUIPPED`], where the same check accepts the worn slot minus 100.
pub const EQUIP_SLOTS: std::ops::RangeInclusive<u8> = 1..=31;

/// "This item never expires", as a Windows FILETIME.
///
/// **[I], and it is the first field to change if a run comes back "no fault, still naked".**
/// Zero is a valid FILETIME - it is 1601-01-01, an item that expired four centuries ago -
/// and no client-side expiry check was found in this binary. This constant is the value
/// every MapleStory server sends (2079-01-01) and comes from convention, not from
/// `MapleStory.exe`. It costs nothing to send, so it is sent.
///
/// The lesson it is hedging against is [`npc_enter_field`]: that body was structurally
/// perfect and produced nothing on screen because two *values* were zero.
pub const ITEM_NEVER_EXPIRES: u64 = 150_842_304_000_000_000;

// ---------------------------------------------------------------------------------------
// The equip item's four bitmasks. Full working, with the labels and their evidence, is in
// `research/equip-stats.md`.
// ---------------------------------------------------------------------------------------

/// How many optional `u16` `FUN_140303800`'s `u32` mask gates. **[L]**
pub const EQUIP_STAT_BITS: usize = 17;

/// How many optional fields `FUN_140303b40`'s own `u32` mask gates. **[L]**
pub const EQUIP_OPTION_BITS: usize = 21;

/// The 17 optional `u16` behind one `FUN_140303800` mask, in **mask-bit order**.
///
/// Bit `k` is present iff bit `k` of the mask is set, and the client decodes it into an
/// 8-byte obfuscated slot at `base + 8k` - value at `+0`, integrity dword at `+4`. **[L]**,
/// `research/msexe-itemslot-800.txt`, bounded `0x140303800 .. 0x140303a6c` by `.pdata`.
///
/// # The wire is a raw `u16`. The obfuscation is the client's, and it is not ours to write
///
/// The 8-byte stride with a dword beside it is a ZtlSecure-style slot - `{key, key^value,
/// checksum}` - and the obvious worry is that the server has to send the *encoded* form, or
/// a checksum, or that the writer and the tooltip's reader disagree. **They cannot.** **[L]**
///
/// `FUN_140303800` reads the plain `u16` off the packet with `0x1406e8b80` and then calls
/// `FUN_1402f7010(value, base + 8k)`, storing the return at `base + 8k + 4`. That function
/// calls the client's own byte PRNG `FUN_1407386b0(&DAT_143ac1ab0)` twice, writes
/// `dst[i] = key_i` and `dst[2+i] = key_i ^ value_i`, and accumulates
/// `chk = ror32(chk ^ key_i, 5) + (key_i ^ value_i)` from `0xbaadf00d`. The tooltip's getter
/// `FUN_1401ab420(p, chk)` returns `((p[3]^p[1]) << 8) | (p[2]^p[0])` and recomputes the
/// same recurrence - term for term, an exact inverse pair.
///
/// So the key is invented at decode time, inside the function that reads the packet. There
/// is exactly one writer of these slots on this path and it is the packet decoder: a server
/// could not send an encoded value even in principle, and anything it sent in a checksum
/// field would be overwritten. **Send the raw little-endian `u16` and nothing else.**
///
/// Two near-misses worth not re-deriving: `FUN_1402f70a0` is **byte-identical** to
/// `FUN_1402f7010` (an un-folded duplicate, not a second encoding), and the `0x9a65` /
/// `FUN_1402fa540` scheme in `research/charstat-layout.md` is a **different** mechanism -
/// it is the rolling re-key of the secure slot at `item+0x20` that holds the itemId, not
/// these fields. Full working: `research/equip-stats.md` section 11.1.
///
/// **Every label below was read from this binary, not from the game family**, by pairing
/// the offset the tooltip's getter `FUN_1401ab420(item + off, [item + off + 4])` reads with
/// the string id `FUN_1408a9e40` resolves beside it, and cross-checking against the WZ
/// property name the `ITEMINFO` loader stores at the base offset the same line uses. **[L]**
///
/// The family expectation (`.. PAD, MAD, PDD, MDD, ACC, EVA, Craft, Speed, Jump`) is
/// **wrong for this client**: Speed and Jump are bits 6-7, there is no Craft field, and
/// bits 14-16 are Critical Rate, Critical Damage and a separate Weapon Attack. Building
/// this from the family expectation would have put `incPAD` at bit 6.
///
/// # These are the item's TOTAL stats, not increments over the WZ template
///
/// An earlier pass of this file claimed the opposite and labelled it [L]. It was wrong -
/// the evidence was only that the WZ value and this value are passed to the same function,
/// and the function was never read. Two independent [L] readings settle it:
///
/// * `FUN_142699710`, the stat-line assembler, prints **this value alone** as the headline
///   number (`R8D` is loaded from arg 5 at `0x142699745` and is still the vararg at the
///   `CALL 0x14019ba10` at `0x142699771`), then computes
///   `leftover = this - ITEMINFO.inc - baseline - timed` to fill in a ` (%d +%d +%d`
///   breakdown. It **subtracts** the template value; it never adds it.
/// * `FUN_14038d3c0` compares the two **directly, in the same units**:
///   `CMP AX, word ptr [RDI+0xba]` at `0x14038d434` is this field's `inc_str` against
///   `ITEMINFO.incSTR`. That test is meaningless if one side is an increment.
///
/// So a fresh Undershirt must be sent with `inc_pdd = 6`, copied from
/// `01040002.img/info/incPDD`. Each field below names the WZ property it mirrors, and
/// [`EQUIP_STAT_WZ_PROPERTIES`] gives the same mapping in bit order for a generator.
///
/// # The print guard is real, and a run has now tested it
///
/// `FUN_142699710` opens with a guard, at `0x142699749`:
///
/// ```text
/// MOV  R8D,[RBP+0x6f]   ; arg 5 - THIS value
/// TEST R8D,R8D
/// JG   print
/// TEST R9D,R9D          ; arg 4 - a baseline from a fourth struct
/// JLE  return           ; both <= 0 -> no line at all
/// ```
///
/// The WZ value is **not** in the guard, so it can never make a line appear on its own, and
/// a zero here would suppress its own line. All **[L]**.
///
/// **The run-1 tooltip screenshot tests this and it holds.** That item's `ITEMINFO.incPDD`
/// is 6 and we sent 0, and the box has **no** `Weapon Def.` line - which is what "the guard
/// is on arg 5" predicts and nothing else does. The same screenshot has
/// `Remaining Enhancements: 0` and `Scissors Usages Available : 0` in it, and both are
/// emitted **behind** `FUN_1426b20f0`'s `ITEMINFO` gate (`TEST R15,R15 / JZ 0x1426b3d8e` at
/// `0x1426b223e`, emits at `0x1426b3a72` and `0x1426b3cb3`), so that gate was open, all four
/// stat helpers ran without throwing, and `FUN_14269a1d0` was not silently dropping lines.
/// **[D]**, transcript in `research/equip-stats.md` section 11.4.1.
///
/// # Run 2 is the one that is not explained
///
/// Sending `inc_pdd = 6` and `inc_wat = 17` produced no stat line either. Every link is
/// measured except the observation: the wire is verified byte for byte out of `world.log`,
/// the decode is structural, the gate was open in run 1 and nothing we changed feeds it
/// (its only input is the itemId at `item+0x20`), a from-template copy would have shown the
/// template's 6 in run 1 and did not (`FUN_1403d2200` fills the stat block from `ITEMINFO`),
/// and the guard is corroborated above.
///
/// **The run-2 tooltip transcript settles what it is not.** For 1060002 (trousers, sent
/// `incPDD = 4`, `tuc = 7`, `scissorUses = 0xFF`) the box reads `Remaining Enhancements: 0`
/// and `Scissors Usages Available : 0` beside the missing stat line. Those two are direct
/// prints of `item+0xfa` and `item+0x1a6` through `FUN_1401b0050`, and they printed rather
/// than throwing - so the checksums are valid and the values really are zero. **Three
/// fields, two widths, one object, all zero, with the `ITEMINFO` gate open.** The tooltip's
/// object carries none of our optional fields while its identity fields are right.
///
/// **No packet change is indicated by any of it**, and four candidate homes for that object
/// are eliminated statically: the equipped array at `record + 0x1a8 + slot*0x10` stores the
/// decoded object by refcounted pointer (`INC.LOCK [RBX+8]` / `MOV [RDI+8],RBX` at
/// `0x14030629b`/`0x1403062ac`, no copy); `FUN_14030b560` is dead unless the itemId is in
/// 1660000..1669999; the four lists `presence[2]` opens beside the equipped list are the
/// **bag** inventories, where empty is correct and yields no item at all; and
/// `FUN_1403d2200`, the client's build-an-item-from-an-id utility, fills the stat block from
/// `ITEMINFO` and would have printed `Weapon Def.: +4`. `research/equip-stats.md` 11.4.5.
///
/// What is left is a second object versus our object holding zeros, separated by pointer
/// identity in one run - section 11.6.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EquipStatSet {
    /// bit 0 - `STR: +%d` (0x0663). WZ template: `info/incSTR`, `ITEMINFO+0xba`
    pub inc_str: u16,
    /// bit 1 - `DEX: +%d` (0x0664). WZ template: `info/incDEX`, `ITEMINFO+0xbc`
    pub inc_dex: u16,
    /// bit 2 - `INT: +%d` (0x0665). WZ template: `info/incINT`, `ITEMINFO+0xbe`
    pub inc_int: u16,
    /// bit 3 - `LUK: +%d` (0x0666). WZ template: `info/incLUK`, `ITEMINFO+0xc0`
    pub inc_luk: u16,
    /// bit 4 - `MaxHP: +%d` (0x0668). WZ template: `info/incMHP`, `ITEMINFO+0xc2`
    pub inc_mhp: u16,
    /// bit 5 - `MaxMP: +%d` (0x0669). WZ template: `info/incMMP`, `ITEMINFO+0xc4`
    pub inc_mmp: u16,
    /// bit 6 - `Speed: +%d` (0x038A). WZ template: `info/incSpeed`, `ITEMINFO+0xdc`
    pub inc_speed: u16,
    /// bit 7 - `Jump: +%d` (0x038B). WZ template: `info/incJump`, `ITEMINFO+0xde`
    pub inc_jump: u16,
    /// bit 8 - `Attack Power: +%d` (0x0380). WZ template: `info/incPAD`, `ITEMINFO+0xcc`
    pub inc_pad: u16,
    /// bit 9 - `Magic Attack: +%d` (0x0381). WZ template: `info/incMAD`, `ITEMINFO+0xce`
    pub inc_mad: u16,
    /// bit 10 - `Weapon Def.: +%d` (0x0383). WZ template: `info/incPDD`, `ITEMINFO+0xd0`
    pub inc_pdd: u16,
    /// bit 11 - `Magic Def.: +%d` (0x0384). WZ template: `info/incMDD`, `ITEMINFO+0xd2`
    pub inc_mdd: u16,
    /// bit 12 - `Accuracy: +%d` (0x0385). WZ template: `info/incACC`, `ITEMINFO+0xd4`
    pub inc_acc: u16,
    /// bit 13 - `Evasion: +%d` (0x0386). WZ template: `info/incEVA`, `ITEMINFO+0xd6`
    pub inc_eva: u16,
    /// bit 14 - `Critical Rate: +%d` (0x0387). WZ template: `info/incCRT`, `ITEMINFO+0xd8`
    pub inc_crt: u16,
    /// bit 15 - `Critical Damage: +%d` (0x0388). WZ template: `info/incCRD`, `ITEMINFO+0xda`
    pub inc_crd: u16,
    /// bit 16 - `Weapon Attack: +%d` (0x037F). WZ template: `info/incWAT`, `ITEMINFO+0xca`
    pub inc_wat: u16,
}

/// The `Character.wz` / `Item.wz` `info/` property that supplies each stat bit's value, in
/// mask-bit order, for a generator that reads the template out of the WZ. **[L]**, from the
/// `ITEMINFO` loader listing `research/msexe-iteminfo-load.txt`.
pub const EQUIP_STAT_WZ_PROPERTIES: [&str; EQUIP_STAT_BITS] = [
    "incSTR", "incDEX", "incINT", "incLUK", "incMHP", "incMMP", "incSpeed", "incJump", "incPAD",
    "incMAD", "incPDD", "incMDD", "incACC", "incEVA", "incCRT", "incCRD", "incWAT",
];

impl EquipStatSet {
    /// Build a set from the 17 WZ template values, in the order of
    /// [`EQUIP_STAT_WZ_PROPERTIES`]. A missing property is 0.
    pub fn from_wz_template(values: [u16; EQUIP_STAT_BITS]) -> Self {
        Self {
            inc_str: values[0],
            inc_dex: values[1],
            inc_int: values[2],
            inc_luk: values[3],
            inc_mhp: values[4],
            inc_mmp: values[5],
            inc_speed: values[6],
            inc_jump: values[7],
            inc_pad: values[8],
            inc_mad: values[9],
            inc_pdd: values[10],
            inc_mdd: values[11],
            inc_acc: values[12],
            inc_eva: values[13],
            inc_crt: values[14],
            inc_crd: values[15],
            inc_wat: values[16],
        }
    }

    /// The fields in mask-bit order. **This array's order is the wire format** - the mask
    /// and the payload are both derived from it, so they cannot disagree.
    fn in_bit_order(&self) -> [u16; EQUIP_STAT_BITS] {
        [
            self.inc_str,
            self.inc_dex,
            self.inc_int,
            self.inc_luk,
            self.inc_mhp,
            self.inc_mmp,
            self.inc_speed,
            self.inc_jump,
            self.inc_pad,
            self.inc_mad,
            self.inc_pdd,
            self.inc_mdd,
            self.inc_acc,
            self.inc_eva,
            self.inc_crt,
            self.inc_crd,
            self.inc_wat,
        ]
    }

    /// The `u32` mask: bit `k` is set exactly when field `k` is non-zero.
    pub fn mask(&self) -> u32 {
        let mut mask = 0u32;
        for (bit, value) in self.in_bit_order().into_iter().enumerate() {
            if value != 0 {
                mask |= 1 << bit;
            }
        }
        mask
    }

    /// Mask plus payload.
    pub fn wire_len(&self) -> usize {
        4 + 2 * self.mask().count_ones() as usize
    }

    /// Payload only - what this costs beyond the mask that is always sent.
    pub fn optional_len(&self) -> usize {
        self.wire_len() - 4
    }

    /// Where bit `bit` lands in the item object, for a set decoded at `base`.
    /// The integrity dword the getter checks is at the returned offset `+ 4`. **[L]**
    pub const fn object_offset(base: usize, bit: usize) -> usize {
        base + 8 * bit
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.mask().to_le_bytes());
        for value in self.in_bit_order() {
            if value != 0 {
                out.extend_from_slice(&value.to_le_bytes());
            }
        }
    }
}

/// The 21 mixed-width optional fields behind `FUN_140303b40`'s own `u32` mask.
///
/// Widths read off `research/msexe-itemslot-b40.txt`; the struct base is `item + 0xfa`
/// (`FUN_140303b40`'s base `item + 0x62`, plus the `0x98` its own fields start at). Nine of
/// the twenty-one offsets are confirmed independently by a tooltip getter reading exactly
/// that address. **[L]**
///
/// Fields nothing was established about keep an `unknown_b<n>` name on purpose. They are
/// **not** known to be unused - only unidentified - and sending a non-zero value for one
/// would add its width to the wire for no known effect.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EquipOptions {
    /// bit 0, `u8` at `item+0xfa` - `Remaining Enhancements: %d` (0x039E). The client
    /// prints this straight from the packet with **no WZ fallback**, and only when
    /// `ITEMINFO.tuc` is non-zero. `01040002.img/info/tuc` is 7, so a fresh Undershirt
    /// showing 0 here is showing our zero. **[L]**
    ///
    /// **The client does compare this against `ITEMINFO.tuc`**, so it is not a free field:
    /// `FUN_14038d3c0` opens with `LEA RCX,[RBX+0xfa] / CALL FUN_1401b0050 /
    /// CMP AL, byte ptr [RDI+0xb8]` at `0x14038d41c`, `RDI` being the `ITEMINFO`. One path
    /// takes `>=` as "not fresh", the other `>`. **[L]** Keep this in `0..=tuc`; `tuc`
    /// itself is the unused-item boundary.
    pub remaining_enhancements: u8,
    /// bit 1, `u8` at `item+0x102`. Nothing in this binary names it; the v214
    /// reference calls this bit `cuc` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b1: u8,
    /// bit 2, `u16` at `item+0x10a` - the **attribute bitfield**; see [`ATTRIBUTE_BIT_3`].
    /// Twelve one-line client accessors read one bit each out of this. **[L]**
    pub attribute: u16,
    /// bit 3, `u8` at `item+0x112`. Nothing in this binary names it; the v214
    /// reference calls this bit `levelUpType` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b3: u8,
    /// bit 4, `u8` at `item+0x11a`. Nothing in this binary names it; the v214
    /// reference calls this bit `level` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b4: u8,
    /// bit 5, `u64` at `item+0x122`. Nothing in this binary names it; the v214
    /// reference calls this bit `exp` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b5: u64,
    /// bit 6, `u32` at `item+0x13a`. Nothing in this binary names it; the v214
    /// reference calls this bit `durability` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b6: u32,
    /// bit 7, `u32` at `item+0x146` - a positive value plus a WZ check prints
    /// `Golden Hammer reforging applied` (0x0DA7). **[L]**
    pub golden_hammer: u32,
    /// bit 8, `u8` at `item+0x152` - `Required Level: -%d` (0x0394). A **reduction**; the
    /// requirement itself is `ITEMINFO.reqLevel`, from the WZ. **[L]**
    pub level_requirement_reduction: u8,
    /// bit 9, `u16` at `item+0x15a`. Nothing in this binary names it; the v214
    /// reference calls this bit `specialAttribute` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b9: u16,
    /// bit 10, `u32` at `item+0x162`. Nothing in this binary names it; the v214
    /// reference calls this bit `durabilityMax` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b10: u32,
    /// bit 11, `u8` at `item+0x16e`. Nothing in this binary names it; the v214
    /// reference calls this bit `iIncReq` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b11: u8,
    /// bit 12, `u8` at `item+0x176`. Nothing in this binary names it; the v214
    /// reference calls this bit `growthEnchant` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b12: u8,
    /// bit 13, `u8` at `item+0x17e`. Nothing in this binary names it; the v214
    /// reference calls this bit `psEnchant` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b13: u8,
    /// bit 14, `u8` at `item+0x186` - `Boss Damage +%d%%` (0x0D65). **[L]**
    pub boss_damage_percent: u8,
    /// bit 15, `u8` at `item+0x18e` - `Ignored Enemy DEF : +%d%%` (0x0672). **[L]**
    pub ignore_enemy_def_percent: u8,
    /// bit 16, `u8` at `item+0x196` - `Damage: +%d%%` (0x0389). **[L]**
    pub damage_percent: u8,
    /// bit 17, `u8` at `item+0x19e` - `All Stats: +%d%%` (0x0671). **[L]**
    pub all_stats_percent: u8,
    /// bit 18, `u8` at `item+0x1a6` - `Scissors Usages Available : %d` (0x03A0), and **the
    /// cause of "Cannot be Traded when equipped"**. See [`NO_SCISSOR_RESTRICTION`].
    pub scissor_uses: u8,
    /// bit 19, `u64` at `item+0x1ae`. Nothing in this binary names it; the v214
    /// reference calls this bit `exGradeOption` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b19: u64,
    /// bit 20, `u32` at `item+0x1c6`. Nothing in this binary names it; the v214
    /// reference calls this bit `hyperUpgrade` - **[I]**, a 1-of-8 source, and section 10.3
    /// of `research/equip-stats.md` says why that is a candidate and not a name.
    pub unknown_b20: u32,
}

impl EquipOptions {
    /// `(value, width in bytes)` in mask-bit order. The widths are the listing's, and this
    /// array is the single source for both the mask and the payload.
    fn in_bit_order(&self) -> [(u64, usize); EQUIP_OPTION_BITS] {
        [
            (u64::from(self.remaining_enhancements), 1),
            (u64::from(self.unknown_b1), 1),
            (u64::from(self.attribute), 2),
            (u64::from(self.unknown_b3), 1),
            (u64::from(self.unknown_b4), 1),
            (self.unknown_b5, 8),
            (u64::from(self.unknown_b6), 4),
            (u64::from(self.golden_hammer), 4),
            (u64::from(self.level_requirement_reduction), 1),
            (u64::from(self.unknown_b9), 2),
            (u64::from(self.unknown_b10), 4),
            (u64::from(self.unknown_b11), 1),
            (u64::from(self.unknown_b12), 1),
            (u64::from(self.unknown_b13), 1),
            (u64::from(self.boss_damage_percent), 1),
            (u64::from(self.ignore_enemy_def_percent), 1),
            (u64::from(self.damage_percent), 1),
            (u64::from(self.all_stats_percent), 1),
            (u64::from(self.scissor_uses), 1),
            (self.unknown_b19, 8),
            (u64::from(self.unknown_b20), 4),
        ]
    }

    /// The width, in wire bytes, of option bit `bit`. Panics outside `0..21`.
    pub fn width_of(bit: usize) -> usize {
        Self::default().in_bit_order()[bit].1
    }

    /// The `u32` mask: bit `k` is set exactly when field `k` is non-zero.
    pub fn mask(&self) -> u32 {
        let mut mask = 0u32;
        for (bit, (value, _)) in self.in_bit_order().into_iter().enumerate() {
            if value != 0 {
                mask |= 1 << bit;
            }
        }
        mask
    }

    /// Mask plus payload.
    pub fn wire_len(&self) -> usize {
        4 + self
            .in_bit_order()
            .into_iter()
            .filter(|(value, _)| *value != 0)
            .map(|(_, width)| width)
            .sum::<usize>()
    }

    /// Payload only.
    pub fn optional_len(&self) -> usize {
        self.wire_len() - 4
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.mask().to_le_bytes());
        for (value, width) in self.in_bit_order() {
            if value != 0 {
                out.extend_from_slice(&value.to_le_bytes()[..width]);
            }
        }
    }
}

/// At or below this many scissor uses, the client treats the item as karma-restricted.
///
/// `FUN_1402fd610`, the item's `vtable+0x200`, is seven instructions: it returns false for a
/// cash item (serial at `item+0x38` non-zero) and otherwise `scissor_uses <= 0x14`. **[L]**
pub const SCISSOR_RESTRICTED_MAX: u8 = 0x14;

/// What [`EquipOptions::scissor_uses`] has to carry so an item is not trade-blocked.
///
/// **This is the answer to "Cannot be Traded when equipped".** With `scissor_uses = 0`,
/// `vtable+0x200` is true, and `FUN_14038cf10` then reports a trade restriction for any item
/// whose WZ `tradeBlock` is 0 - which is every starter item. `FUN_1403e8e00` turns that into
/// string 0x03C6, `Cannot be Traded when equipped`, and the same predicate is why
/// `Scissors Usages Available : 0` printed as well. One byte causes both lines.
///
/// **[D]**, upgraded from [I] once the one function that singles out `0xFF` was read.
/// `FUN_1403e8d40` returns 1 only when this field reads exactly `0xFF` **and** the item is
/// otherwise unrestricted, and its single caller uses that to add string 0x0F66,
/// `"However, this will restrict the Scissors count."`, to the Rebirth Flame confirmation.
/// So `0xFF` means *"no karma restriction yet"* - the warning exists because applying the
/// flame would impose one. That is the value we want, and it walks into nothing else: the
/// only behaviour it changes is a warning line that is correct for an unrestricted item.
///
/// Still not [L]: nothing in the binary was found that *writes* `0xFF`. The threshold
/// ([`SCISSOR_RESTRICTED_MAX`]) is literal; the sentinel is derived from two consumers.
///
/// The v214 reference agrees - `ItemData.java` gives a freshly created equip
/// `setCuttable((short) -1)`, encodes the field as a byte, and puts it at the same mask bit.
/// That is **[I]** and does not raise the label: the tree is a different game version and
/// scored 1 of 8 against a held-out control. It agrees here; it disagrees with eleven of the
/// seventeen [`EquipStatSet`] bits, which is why it is a corroboration and not a source.
pub const NO_SCISSOR_RESTRICTION: u8 = 0xFF;

/// Attribute bit 3 - the item's `vtable+0x28`, and the first thing `FUN_14038cf10` tests.
///
/// Set, it suppresses the trade line outright (`JNZ -> return 0` at `0x14038cf30`). **[L]**
/// It is **not** the recommended fix: it costs two bytes rather than one, it does not touch
/// the `Scissors Usages Available` line, and three other predicates read the same bit for
/// reasons that were not established. Prefer [`NO_SCISSOR_RESTRICTION`].
pub const ATTRIBUTE_BIT_3: u16 = 1 << 3;

/// Everything the four bitmasks in one equipped item can carry.
///
/// [`Default`] is **all masks zero**, which is the 125-byte body confirmed on screen on
/// 2026-08-19. Every field added here widens the item, and the record has no length prefix
/// and no resync point - which is exactly why the mask is computed from the fields rather
/// than stored beside them.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EquipStats {
    /// `FUN_140303800` called from `FUN_140303b40` - decoded at `item + 0x62`, and the set
    /// the tooltip shows as the "+n" on each stat line.
    pub stats: EquipStatSet,
    /// `FUN_140303b40`'s own mask - decoded at `item + 0xfa`.
    pub options: EquipOptions,
    /// `FUN_140303800` at `0x1403043f6` - decoded at `item + 0x26b`. Always sent. Nothing in
    /// the tooltip reads it and its meaning is **not established**; leave it zero.
    pub second_stats: EquipStatSet,
    /// `FUN_140303800` at `0x140304411` - decoded at `item + 0x313`, and read **only** when
    /// the `u8` tailFlag before it is non-zero. The tooltip passes `item + 0x313 + 8k` as a
    /// second value on every stat line. `Some` sets the tailFlag; `None` leaves it 0.
    pub timed_stats: Option<EquipStatSet>,
}

impl EquipStats {
    /// Bytes this adds to [`EQUIPPED_ITEM_LEN`]. The three masks and the tailFlag are
    /// already counted there, so only the payloads and the whole fourth set are extra.
    pub fn extra_len(&self) -> usize {
        self.stats.optional_len()
            + self.options.optional_len()
            + self.second_stats.optional_len()
            + self.timed_stats.map_or(0, |set| set.wire_len())
    }

    /// A fresh, unmodified item as a real server would send it.
    ///
    /// `template` is the item's own WZ `info/inc*` block - see
    /// [`EQUIP_STAT_WZ_PROPERTIES`] and [`EquipStatSet::from_wz_template`] - **not** zero:
    /// the client reads these fields as the item's total stats and compares them directly
    /// against the template, so an item sent with zeros is an item with no stats, and
    /// `FUN_142699710`'s guard would suppress each zero field's own line.
    ///
    /// **Sending the real values did not produce a stat line**, on the run of 2026-08-19,
    /// and that is still unexplained. The bytes are right - `world.log`'s `0x01A0` body
    /// parses to four 129-byte items with mask 1 = `0x400`/value 6 for 1040003 and
    /// `0x10000`/value 17 for 1302000 - so the suppression is downstream of the packet.
    /// Nothing here is known to need changing; see the `EquipStatSet` docs and
    /// `research/equip-stats.md` section 11.
    ///
    /// One loose end this call has: `tuc` is sent **equal** to `ITEMINFO.tuc`, which is the
    /// "not fresh" side of `FUN_14038d3c0`'s `CMP AL, byte ptr [RDI+0xb8] / JNC` at
    /// `0x14038d41c`. `tools/callers.py` finds 0 direct callers of that function, but it
    /// only sees `call rel32` and cannot see a vtable or indirect call, so that zero is not
    /// evidence of absence. `tuc - 1` is the cheap variant if a run ever needs one.
    ///
    /// `tuc` is the item's WZ `info/tuc`. The client compares this field against
    /// `ITEMINFO.tuc`, so it must not exceed it; `tuc` itself is the unused-item value.
    ///
    /// Scissor uses are set to [`NO_SCISSOR_RESTRICTION`], which is what clears both
    /// `Scissors Usages Available : 0` and `Cannot be Traded when equipped`.
    ///
    /// This is **not** what [`equipped_block`] sends. It lengthens the item by two bytes per
    /// non-zero stat plus two, so it changes the character record's length - a one-variant
    /// client test of its own, and `tools/channel_smoke.py` hard-codes
    /// `EQUIPPED_ITEM_LEN = 125` and would have to be relaxed in the same change.
    pub fn fresh(template: EquipStatSet, tuc: u8) -> Self {
        Self {
            stats: template,
            options: EquipOptions {
                remaining_enhancements: tuc,
                scissor_uses: NO_SCISSOR_RESTRICTION,
                ..EquipOptions::default()
            },
            ..Self::default()
        }
    }
}

/// One equipped item, as `FUN_140304100` - the type-1 `vtable+0x358` decode - reads it.
///
/// Every row is **[L]**, read off `research/msexe-itemslot-equip-decode.txt` and its five
/// sub-decoder listings; the address in each comment is where that read happens.
/// `research/naked-character.md` section 3.3 is the table this mirrors row for row.
///
/// **Why this is only 125 bytes.** Three of the fields are `u32` bitmasks and every bit of
/// each gates one optional read - 17 `u16` in `FUN_140303800`, 21 mixed-width fields in
/// `FUN_140303b40`. All-zero masks read nothing past the mask itself, which is what
/// collapses a modern equip record from several hundred unknown bytes to this.
///
/// **The one item family this does not describe** is `itemId / 10000 == 166`, which pulls in
/// `FUN_1402cb4f0` at `0x14030435e` as well. The debug assertion is there because such an
/// item would silently make the body a different length, and the record has no resync point.
///
/// **How the vtable was found, since `equip-block.md` once called this unreadable.** The
/// item classes carry no RTTI, but they do not need to: the type-1 constructor
/// `FUN_1402f7da0` stores its vtable with `LEA RAX,[0x14327E1D8]` at `0x1402f7dbd`, and the
/// positive control that this really is the item vtable is `vtable+0x88` - it reads
/// `b8 01 00 00 00 c3`, literally `return 1`, matching the 1/2/3 the release function
/// switches on. `+0x330`, which that document called the decode, is `FUN_1402fbb30` =
/// `return this + 0x242`, an accessor.
pub fn equipped_item(item_id: u32, stats: &EquipStats) -> Vec<u8> {
    equipped_item_with_cash_sn(item_id, stats, None)
}

/// [`equipped_item`] carrying a **cash serial** in the item's own `+0x38`.
///
/// # Why the serial, and why it is the same length
///
/// The cash shop's locker-to-bag reply (`0x05AE` sub-op `0x19`, `FUN_140D7F8A0`) reads the
/// item it carries and then erases **the item's `+0x38`** from the locker map - `cmp qword
/// [item+0x38], -1 ; je` then `FUN_140D85C10(stage+0x150, &sn)`
/// (`research/cash-shop-buy-done.md` section 3.1, **[L]**). The owner, 2026-09-11 03:47: the
/// coupon *"showed up in my item inventory but did not disappear from Cash Inventory"* - the
/// body had `hasCashSN = 0`, so `+0x38` was the constructor's zero, the erase looked for
/// serial 0, and the panel kept the row. The next drag named a locker slot the server had
/// already emptied, which is the *"unknown error"* that followed.
///
/// With the flag set the base decode reads `raw[8]` into `+0x38` at `0x14030379d` instead of
/// zeroing it (`0x1403037a4`), and the equip decode then **skips** the `raw[8]` at
/// `0x14030429e` because `0x14030428a` tests `[this+0x38] != 0` first. **[L]**, both
/// listings. So the body is the same 125 bytes plus mask extras either way - and that test
/// is on the *value*, not the flag, which is why `cash_sn` is a `NonZeroU64`: a flag with a
/// zero serial would read both `raw[8]`s and be eight bytes off.
pub fn equipped_item_with_cash_sn(
    item_id: u32,
    stats: &EquipStats,
    cash_sn: Option<std::num::NonZeroU64>,
) -> Vec<u8> {
    debug_assert_ne!(item_id / 10000, 166, "a 166xxxx item reads FUN_1402cb4f0 as well");
    let mut b = Vec::with_capacity(EQUIPPED_ITEM_LEN + stats.extra_len());
    b.push(EQUIPPED_ITEM_TYPE); // 1403095fb  u8   the factory's type byte

    // FUN_1403035a0, the base decode shared by all three item types. **[L]** - the read list
    // below was verified 2026-08-19 by enumerating every call target in its listing
    // (`research/msexe-itemslot-base.txt`, 0x1403035a0..0x1403037f5): 2x 0x1406e8c20,
    // 2x 0x1406e8ae0, 2x 0x1406e9170 and three non-readers. Nothing else reads the packet
    // here, which is what fixes the offset the three mask u32s land on.
    b.extend_from_slice(&item_id.to_le_bytes()); //           1403035c5  u32  itemId
    // A non-zero hasCashSN pulls in a u64 cash serial at 14030379d and drops the raw[8] at
    // 14030429e - same total, different layout.
    match cash_sn {
        Some(sn) => {
            b.push(1); //                                     140303787  u8   hasCashSN
            b.extend_from_slice(&sn.get().to_le_bytes()); //  14030379d  raw[8] -> +0x38
        }
        None => b.push(0), //                                 140303787  u8   hasCashSN
    }
    b.extend_from_slice(&ITEM_NEVER_EXPIRES.to_le_bytes()); //1403037b9  u64  dateExpire
    b.extend_from_slice(&0u32.to_le_bytes()); //              1403037c1  u32  -> +0x48
    b.push(0); //                                             1403037cc  u8   -> +0x4c (bool)

    // FUN_140303b40(this+0x62): two bitmasks, 17 and 21 optional reads. Both masks are
    // computed from the fields, so a bit can never be set without its bytes following.
    stats.stats.encode_into(&mut b); //                       14030381d  u32  statMask + 17
    stats.options.encode_into(&mut b); //                     140303b66  u32  optMask  + 21

    // Back in FUN_140304100.
    b.extend_from_slice(&[0u8; 13]); //                       140304138  raw[13] char[13] name
    b.push(0); //                                             140304144  u8   -> blob +0x3af
    b.push(0); //                                             140304183  u8   -> blob +0x3b7
    // 1403041c2 / 1df / 1fc / 219 / 236 / 253 / 270: seven u16, into +0x3bf .. +0x3ef.
    b.extend_from_slice(&[0u8; 14]);
    if cash_sn.is_none() {
        b.extend_from_slice(&[0u8; 8]); //                    14030429e  raw[8], [+0x38] == 0
    }
    // 1403042b7  FUN_1402cce00: raw[8], raw[8], u32, u32, u32, u32.
    b.extend_from_slice(&[0u8; 32]);
    b.extend_from_slice(&[0u8; 12]); //                       1403042c6  FUN_1402cd090: raw[8], u32
    b.extend_from_slice(&0u32.to_le_bytes()); //              1403042ce  u32  -> +0x23e
    // 1403042dc / 2f9 / 316: three u16, into +0x3f7, +0x3ff, +0x407.
    b.extend_from_slice(&[0u8; 6]);
    b.push(0); //                                             14030436d  u8   -> blob +0x303
    b.push(0); //                                             1403043b1  u8   -> blob +0x30b
    // 1403043f6  FUN_140303800(this+0x26b): the third mask and its 17 optional u16.
    stats.second_stats.encode_into(&mut b);
    // 1403043fe  tailFlag. Non-zero reads a fourth FUN_140303800 at 140304411, into
    // this+0x313 - the set the tooltip shows beside each stat line.
    match stats.timed_stats {
        Some(timed) => {
            b.push(1);
            timed.encode_into(&mut b);
        }
        None => b.push(0),
    }

    debug_assert_eq!(b.len(), EQUIPPED_ITEM_LEN + stats.extra_len());
    b
}

/// The whole gate-entry-6 region: the equipped list, and the four lists it drags in with it.
///
/// ```text
/// u8   flagA                    0 - a non-zero value would skip FUN_14030b6f0 below
/// repeat:
///     u16  slot                 1403061fc (first) / 1403062c9 (subsequent)
///     item body                 FUN_1403095e0 at 14030621e, 125 bytes
/// u16  0                        terminator of the equipped list
/// u16  0                        FUN_14030b6f0's one list, read because flagA == 0
/// u16  0, u16 0, u16 0          FUN_14030b9e0's three lists
/// ```
///
/// So the region is `11 + 127 * equips` bytes, and a character with the four starter equips
/// takes the whole record from 224 bytes to **743**. **[L]** for the layout; the total is
/// arithmetic on it.
///
/// **flagA is deliberately 0.** A non-zero value at `0x1403062dd` skips `FUN_14030b6f0` and
/// its terminator - one byte less and one more thing to get wrong. `FUN_14030b9e0` cannot be
/// skipped at all.
/// **This still sends [`EquipStats::default`] - the all-zero body confirmed on screen.**
/// [`EquipStats::plain`] is the version that clears "Cannot be Traded when equipped", and
/// switching to it is a deliberate one-variant change; [`equipped_block_with`] takes it.
pub fn equipped_block(equips: &[(u8, u32)]) -> Vec<u8> {
    let with_stats: Vec<(u8, u32, EquipStats)> =
        equips.iter().map(|&(slot, item_id)| (slot, item_id, EquipStats::default())).collect();
    equipped_block_with(&with_stats)
}

/// [`equipped_block`], with per-item stats.
///
/// Splitting this out keeps the byte-for-byte record that was confirmed on screen reachable
/// while the stats become settable. The item body has no length prefix, so the only thing
/// keeping the record in sync is that each item's mask is derived from its own fields.
pub fn equipped_block_with(equips: &[(u8, u32, EquipStats)]) -> Vec<u8> {
    equipped_block_with_bag(equips, &[], DEFAULT_INVENTORY_SLOTS)
}

/// [`equipped_block_with`], plus the **contents of the Equip tab** - items the character
/// owns but is not wearing.
///
/// # The four lists after the equipped one are not four bags
///
/// That was the working assumption for a day and it is wrong. `presence[2]` opens
/// `FUN_14030b6f0(closure, 1)` and `FUN_14030b9e0` three times, and only the **first** of
/// those four is an inventory tab: `[R14 + 0x5d0]` with `R15 = 1` is `charData + 0x5d8`,
/// index 1 of the six the size loop walks - the Equip tab. `FUN_14030b9e0`'s three land in
/// `charData + 0x5b8`/`+0x5c0`/`+0x5c8` and accept positions **3000-3031, 3100-3131 and
/// 3200-3231**, out of tables at `0x14327dd50` and `0x14327dd68`. Nothing this server can
/// create addresses a position in those ranges, so they stay empty terminators.
///
/// **The Use / Set Up / Etc / Cash bags are still in the record**, further along and behind
/// their own presence bytes (type 2 -> byte 3, 3 -> 4, 4 -> 5, 5 -> 6, 6 -> 44). Those bytes
/// are clear, which is exactly why today's ten zero bytes are byte-correct. Sending them
/// costs more than a list each - see `research/bag-lists.md` section 6, which prices it.
///
/// `slots` is the Equip tab's slot count, the same `V` [`inventory_size_block`] sends: a
/// position outside `1..=slots` is decoded by the client and thrown away, so
/// [`crate::bag::equipped_tail`] drops those rather than paying for them.
pub fn equipped_block_with_bag(
    equips: &[(u8, u32, EquipStats)],
    equip_bag: &[crate::bag::BagEquip],
    slots: u16,
) -> Vec<u8> {
    let mut b = Vec::new();
    b.push(0); // 1403061cc  flagA - MUST stay 0: TEST SIL,SIL / JNZ at 1403062dd skips the
               //            FUN_14030b6f0 call entirely, i.e. the Equip bag cannot be sent
    for (slot, item_id, stats) in equips {
        if !EQUIP_SLOTS.contains(slot) {
            continue; // decoded and thrown away by the client - see EQUIP_SLOTS
        }
        b.extend_from_slice(&u16::from(*slot).to_le_bytes());
        b.extend_from_slice(&equipped_item(*item_id, stats));
    }
    b.extend_from_slice(&crate::bag::equipped_tail(equip_bag, slots));
    b
}

/// The fixed cost of [`equipped_block`]: `flagA` plus five `u16` terminators.
pub const EQUIPPED_BLOCK_OVERHEAD: usize = 1 + 2 + 2 + 6;

/// One **zero-stat** equipped item plus its `u16` slot - what each equip adds to the record.
/// A non-default [`EquipStats`] adds [`EquipStats::extra_len`] on top.
pub const EQUIPPED_ENTRY_LEN: usize = 2 + EQUIPPED_ITEM_LEN;

/// A `SetField` carrying a real character on a real map.
///
/// This is the packet that should put a character in the world. It differs from
/// [`set_field_minimal`] in exactly two ways, and both were unknown until 2026-08-19:
/// `presence[0]` is set, which switches on the stat block, and that block carries
/// `chr.map_id` at its offset 84.
pub fn set_field_with_character(chr: &Character, world_id: u32, clock: u64, channel: u32) -> Vec<u8> {
    let bare: Vec<(u8, u32, EquipStats)> =
        chr.equips.iter().map(|&(s, i)| (s, i, EquipStats::default())).collect();
    set_field_with_character_dressed(chr, world_id, clock, channel, &bare)
}

/// The same `SetField`, with each equip's template stats supplied by the caller.
///
/// See [`character_record_for_set_field_with`] for why the stats arrive this way rather than
/// hanging off [`Character`].
pub fn set_field_with_character_dressed(
    chr: &Character,
    world_id: u32,
    clock: u64,
    channel: u32,
    equips: &[(u8, u32, EquipStats)],
) -> Vec<u8> {
    let mut b = set_field_head(clock, channel, 0);
    b[SET_FIELD_CHARACTER_DATA_AT] = SET_FIELD_WITH_CHARACTER_DATA;
    b.extend_from_slice(&[0u8; 12]); // three u32s the caller reads before the record decoder
    b.extend_from_slice(&character_record_for_set_field_with(chr, world_id, equips));
    // 142098435: a zero u8 here jumps past the next seven reads. The margin after it means
    // a read past the traced path takes a zero rather than running the body out - the
    // client's readers throw on underrun, and the frame carries its own length so surplus
    // bytes are simply never looked at.
    b.extend_from_slice(&[0u8; 1 + 384]);
    b
}

/// Offset of the `characterData` byte in the head, read off the disassembly at `142098169`.
pub const SET_FIELD_CHARACTER_DATA_AT: usize = 30;

/// Zeros after the head: 12 for the three `u32`s, 112 for the record's minimum, 1 for the
/// terminator, and a margin so a read past the traced path still takes a zero rather than
/// running the body out.
pub const SET_FIELD_MINIMAL_TAIL: usize = 12 + 112 + 1 + 384;


#[cfg(test)]
mod set_field_tests {
    /// The NPC body is a fixed 64 bytes with every field mandatory, and the client's decoder
    /// has no length prefix anywhere - one wrong width desynchronises the rest.
    #[test]
    fn an_npc_enter_field_body_is_64_bytes_with_the_fields_where_the_client_reads_them() {
        let heena = FieldNpc {
            object_id: 1000, template_id: 1, x: -46, cy: 305, fh: 66, rx0: -64, rx1: -26, f: 1,
        };
        let b = npc_enter_field(&heena);
        assert_eq!(b.len(), NPC_ENTER_FIELD_LEN);
        assert_eq!(b.len(), 64);

        assert_eq!(&b[0..4], &1000u32.to_le_bytes(), "objectId");
        assert_eq!(&b[4..8], &1u32.to_le_bytes(), "templateId");
        // Negative coordinates are sign-extended, not clamped: -46 goes out as D2 FF.
        assert_eq!(&b[8..10], &[0xD2, 0xFF], "x = -46");
        assert_eq!(&b[10..12], &305i16.to_le_bytes(), "cy");
        assert_eq!(&b[12..16], &(-1i32).to_le_bytes(), "read 3");
        assert_eq!(&b[16..20], &(-1i32).to_le_bytes(), "read 4");
        // **These two were swapped on 2026-08-21 and swapped straight back the same day.**
        // A static pass read byte 20 as the facing bool and byte 21 as the animation action;
        // acting on it killed the client on Sera's first idle line, with two `0x044F` bodies
        // differing in exactly these bytes and nothing else. The builder carries the whole
        // comparison. Heena has `f = 1`, so both bytes are 0 for their either way - which is
        // why they were fine in both runs and could not discriminate.
        assert_eq!(b[20], 0, "read 5, and Heena cannot tell the two layouts apart");
        assert_eq!(b[21], 0, "read 6 carries !flip, and this NPC has f = 1");
        assert_eq!(&b[22..24], &66u16.to_le_bytes(), "fh, the foothold");
        assert_eq!(&b[24..26], &[0xC0, 0xFF], "rx0 = -64");
        assert_eq!(&b[26..28], &[0xE6, 0xFF], "rx1 = -26");
        assert_eq!(&b[28..30], &305i16.to_le_bytes(), "read 10 repeats y");
        assert_eq!(&b[30..32], &305i16.to_le_bytes(), "read 11 repeats y");
        assert_eq!(&b[62..64], &0u16.to_le_bytes(), "a zero-length trailing string");

        // The two that made every NPC invisible. Both were zero in the first version: the
        // packet was dispatched, nothing appeared, and nothing was logged anywhere.
        assert_eq!(b[32], 1, "read 12 is ENABLED - zero means the NPC is disabled");
        // Read 19 is **not** alpha, though it was labelled so here and in the builder, and
        // that made it a suspect every time an NPC failed to appear. `FUN_141e57510` adds it
        // to the current clock and never calls `put_color`. The value is unchanged because
        // NPCs render correctly with it; only the claim about what it means is withdrawn.
        assert_eq!(
            &b[58..62],
            &255u32.to_le_bytes(),
            "read 19 is a clock offset, NOT alpha - the value stands, the label did not"
        );

        // **Sera is the NPC that discriminates, and this pins the layout they survived.**
        // `f = 0`, so `!flip` is 1, and it must go in byte **21**. The other arrangement -
        // `[20] = 1, [21] = 0` - is the one that made their first idle line kill the client on
        // 2026-08-21: `0x009E`, two C++ throws and an access violation, with no dispatch
        // line for the `0x0453` at all.
        //
        // If this assertion is ever flipped again it needs a client run behind it, not a
        // listing.
        let sera = FieldNpc { f: 0, ..heena };
        let sera_body = npc_enter_field(&sera);
        assert_eq!(sera_body[20], 0, "read 5 stays 0 - measured good");
        assert_eq!(sera_body[21], 1, "read 6 carries !flip, and Sera has f = 0");
    }

    /// `0x0451` is the same body behind one flag byte, and the length is the hazard.
    ///
    /// The pool's other creation route. `research/npc-spawn.md` §3.1: `u8 flag; u32 id;` and
    /// then, when the flag is non-zero and the id is new, `u32 templateId` followed by the
    /// **identical** 20-field body - 65 bytes against 64.
    ///
    /// Pinning byte-for-byte equality with [`npc_enter_field`] is the point: if the two ever
    /// diverge, one of them is a short packet, and this client has been killed twice by one.
    #[test]
    fn the_controller_packet_is_the_enter_field_body_behind_one_flag_byte() {
        let heena = FieldNpc {
            object_id: 6000, template_id: 1, x: -46, cy: 305, fh: 66, rx0: -64, rx1: -26, f: 1,
        };
        let enter = npc_enter_field(&heena);
        let ctrl = npc_change_controller(&heena, 1);

        assert_eq!(ctrl.len(), NPC_CHANGE_CONTROLLER_LEN);
        assert_eq!(ctrl.len(), enter.len() + 1, "exactly one byte of difference");
        assert_eq!(ctrl[0], 1, "the flag - 0 would DETACH an NPC instead of creating one");
        assert_eq!(&ctrl[1..], &enter[..], "and the rest is the same decoder's body");
    }

    /// The appear-effect switch is four bytes and its polarity is **inverted** on the wire.
    ///
    /// `0x0452` reads one `u32` and stores `DAT_143ad2d30 = (v != 0)`; the creation path
    /// builds the appear object only while that global is **zero**. So "enabled" is `0` and
    /// "disabled" is `1`, which is exactly the kind of thing that silently inverts the first
    /// time somebody passes a raw integer - hence the bool, and hence this test.
    #[test]
    fn the_npc_appear_effect_switch_inverts_on_the_wire() {
        assert_eq!(npc_appear_effect(true), vec![0, 0, 0, 0], "enabled -> v = 0");
        assert_eq!(npc_appear_effect(false), vec![1, 0, 0, 0], "disabled -> v = 1");
        assert_ne!(
            npc_appear_effect(true),
            npc_appear_effect(false),
            "the two must not encode the same"
        );
        assert_eq!(NPC_APPEAR_EFFECT, 0x0452);
    }

    /// Two NPCs on one field must not share an object id: the pool keys on it, and a repeat
    /// makes the client's handler return after four bytes and silently drop the NPC.
    #[test]
    fn npcs_on_a_field_have_distinct_object_ids() {
        let a = FieldNpc {
            object_id: 1000, template_id: 1, x: -46, cy: 305, fh: 66, rx0: -64, rx1: -26, f: 1,
        };
        let b = FieldNpc {
            object_id: 1001, template_id: 2, x: 833, cy: 125, fh: 8, rx0: 783, rx1: 883, f: 0,
        };
        assert_ne!(a.object_id, b.object_id);
        assert_ne!(npc_enter_field(&a)[0..4], npc_enter_field(&b)[0..4]);
    }

    use super::*;

    /// The head's length is load-bearing: every offset in
    /// `research/msexe-stage-setfield.md` is measured from the client's disassembly, and a
    /// field in the wrong place moves `characterData` and the string count with it.
    #[test]
    fn the_head_is_thirty_three_bytes_with_the_fields_where_the_client_reads_them() {
        let b = set_field_head(0x1122_3344_5566_7788, 7, 0);
        assert_eq!(b.len(), SET_FIELD_HEAD_LEN);
        assert_eq!(&b[0..8], &0x1122_3344_5566_7788u64.to_le_bytes());
        assert_eq!(&b[8..12], &7u32.to_le_bytes(), "channel id at offset 8");
        assert_eq!(b[30], SET_FIELD_NO_CHARACTER_DATA, "characterData at offset 30");
        assert_eq!(&b[31..33], &0u16.to_le_bytes(), "string count at offset 31");
    }

    /// The byte at 17 asks the client to reset a tree on its world object. Nothing wants
    /// that here, and it is one of the two fields that does something on a non-zero value.
    #[test]
    fn the_tree_reset_byte_is_not_set() {
        assert_eq!(set_field_head(0, 0, 0)[12 + 5], 0);
    }

    /// The one byte that decides which branch the client takes, and the branch it does
    /// *not* take faults it. Worth a test of its own.
    #[test]
    fn the_minimal_packet_asks_for_the_character_record_branch() {
        let b = set_field_minimal(1, 0);
        assert_eq!(b[SET_FIELD_CHARACTER_DATA_AT], SET_FIELD_WITH_CHARACTER_DATA);
        assert_eq!(b[SET_FIELD_CHARACTER_DATA_AT], 1);
    }

    /// Everything after the head is zero: the presence array, the record, the terminator.
    /// A stray non-zero byte would set a flag and pull in a block we have not built.
    #[test]
    fn everything_after_the_head_is_zero() {
        let b = set_field_minimal(0x1122_3344_5566_7788, 3);
        assert!(
            b[SET_FIELD_HEAD_LEN..].iter().all(|&x| x == 0),
            "a non-zero byte after the head sets a presence flag"
        );
        assert_eq!(&b[8..12], &3u32.to_le_bytes(), "the head still carries the channel");
        assert_eq!(&b[31..33], &0u16.to_le_bytes(), "the string count is still zero");
    }

    /// The body has to outlast the reads the client makes, because running out mid-read
    /// makes it throw, while surplus is simply never read.
    #[test]
    fn the_body_is_longer_than_the_minimum_the_client_reads() {
        let b = set_field_minimal(0, 0);
        let traced = SET_FIELD_HEAD_LEN + 12 + 112 + 1;
        assert!(b.len() > traced, "{} is not longer than the traced path {}", b.len(), traced);
    }

    #[test]
    fn the_pad_extends_the_body_without_moving_a_field() {
        let plain = set_field_head(9, 3, 0);
        let padded = set_field_head(9, 3, 64);
        assert_eq!(padded.len(), SET_FIELD_HEAD_LEN + 64);
        assert_eq!(&padded[..SET_FIELD_HEAD_LEN], &plain[..]);
        assert!(padded[SET_FIELD_HEAD_LEN..].iter().all(|&x| x == 0));
    }
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
        // The 5th byte from the end changed from 00 to 01 on 2026-08-20 - that is
        // CHANNEL_ENABLED, and it is what makes a channel row clickable. Its doc comment
        // carries the whole working, including why the earlier `1` measurement was
        // confounded and does not contradict this.
        assert_eq!(
            hex(&world_list_entry(0, "Scania", 1)),
            "0006005363616e6961000000000108005363616e69612d31000000000000000100000000000000"
        );
        assert_eq!(CHANNEL_ENABLED, 1, "the byte the golden string above encodes");
        assert_eq!(hex(&world_list_end()), "ff0000");

        // The fourth trailing u8 of each channel entry is what makes its row clickable in
        // the Change Channel dialog, and the third must stay 0 - it feeds a second array
        // that is checked with the OPPOSITE polarity, so a non-zero value there blocks the
        // row. Pin both, because the difference between them is one byte and one sign.
        let two = world_list_entry(0, "Scania", 2);
        let mut at = 1 + 2 + 6 + 1 + 2 + 1 + 1; // id, name, flag, "", flag, channel count
        for i in 0..2u8 {
            at += 2 + "Scania-0".len() + 4; // the channel name and its user count
            assert_eq!(&two[at..at + 4], &[0, i, 0, CHANNEL_ENABLED], "channel {i}");
            // **The third byte must stay 0**, and unlike the fourth this one has a reason
            // that cannot change: it is the adult-channel flag, and its gate needs
            // `singleton+0x2250 >= 18` where that field starts at -1 and its only setter has
            // no callers of any kind. The gate can never open, so a non-zero value here can
            // only block the row. research/channel-two-greyed.md.
            assert_eq!(two[at + 2], 0, "the adult-channel flag - its gate can never open");
            at += 4;
        }
    }

    /// **Channel names count from 1**, the way Change Channel labels them - channel index 0
    /// is "Scania-1". The index bytes after each name stay 0-based: those are what the client
    /// sends back to pick a channel, and the rename must not touch them.
    #[test]
    fn channel_names_count_from_one_while_their_indexes_stay_zero_based() {
        let two = world_list_entry(0, "Scania", 2);
        let mut at = 1 + 2 + 6 + 1 + 2 + 1 + 1;
        for (i, want) in ["Scania-1", "Scania-2"].iter().enumerate() {
            let len = usize::from(u16::from_le_bytes([two[at], two[at + 1]]));
            assert_eq!(&two[at + 2..at + 2 + len], want.as_bytes(), "channel index {i}");
            at += 2 + len + 4;
            assert_eq!(two[at + 1], i as u8, "the index itself is still {i}");
            at += 4;
        }
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

    /// The map id has to land on **the offset the client reads it from**.
    ///
    /// Experience reaches the wire, at the offset the client reads it from.
    ///
    /// Anchored **relative to the map id** rather than to a constant of its own: the map id
    /// already has a test that pins it against the client, and `exp` is a `u64` followed by
    /// a `u32 fame` immediately before it, so `map_id_at - 12` is derived from something
    /// checked rather than from a second hand-counted offset that could drift on its own.
    ///
    /// The discrimination half is the point. Until 2026-08-20 this field was a hardcoded
    /// zero, and a test that only asserted "a zero is here" would have passed against the
    /// hardcoded version, against the wired version, and against a version that wired it to
    /// the wrong field.
    #[test]
    fn experience_lands_on_the_offset_the_client_reads() {
        let base = Character { name: "Grinder".to_string(), ..Character::default() };
        let at = stat_block_map_id_at(base.job) - 12;

        let none = character_stat_block(&base, 0);
        assert_eq!(u64::from_le_bytes(none[at..at + 8].try_into().unwrap()), 0);

        let earned = Character { exp: 0x0102_0304_0506_0708, ..base.clone() };
        let block = character_stat_block(&earned, 0);
        assert_eq!(
            u64::from_le_bytes(block[at..at + 8].try_into().unwrap()),
            0x0102_0304_0506_0708,
            "the experience is not at stat-block offset {at}"
        );
        assert_eq!(block.len(), none.len(), "experience must not change the block's length");
        // Exactly those eight bytes moved. A field written into the wrong place would show
        // up here as a second difference, or as none at all.
        let differs: Vec<usize> =
            (0..block.len()).filter(|&i| block[i] != none[i]).collect();
        assert_eq!(differs, (at..at + 8).collect::<Vec<_>>());
    }

    /// This test used to scan the whole record for any `u32 == START_MAP_ID` and pass if it
    /// found one. That could not fail: `START_MAP_ID` is 1, and a 1 appears in a record for
    /// a dozen unrelated reasons. It passed for months while the map id sat at offset 120,
    /// which is in the character-list trailer and is not on the `SetField` path at all.
    /// Now it checks the one offset that matters, and a wrong placement fails it.
    #[test]
    fn the_start_map_lands_on_the_offset_the_client_reads() {
        let chr = Character { name: "Wanderer".to_string(), ..Character::default() };
        let stat = character_stat_block(&chr, 0);
        assert_eq!(stat.len(), stat_block_len(chr.job), "the stat block changed length");
        let at = stat_block_map_id_at(chr.job);
        assert_eq!(
            u32::from_le_bytes([stat[at], stat[at + 1], stat[at + 2], stat[at + 3]]),
            START_MAP_ID,
            "the map id is not at stat-block offset {at}"
        );

        // And it must be discriminating: a different map has to move this exact u32, and
        // nothing else in the block.
        let moved = Character { map_id: 104_040_000, ..chr.clone() };
        let other = character_stat_block(&moved, 0);
        let differing: Vec<usize> =
            (0..stat.len()).filter(|&i| stat[i] != other[i]).collect();
        assert!(
            differing.iter().all(|&i| (at..at + 4).contains(&i)),
            "changing the map id changed bytes outside {at}..{}: {differing:?}",
            at + 4
        );
        assert!(!differing.is_empty(), "changing the map id changed nothing");
    }

    /// The record the `SetField` path sends: the stat block has to be switched on, and it
    /// has to sit where the client's gate leaves the stream pointer.
    #[test]
    fn the_set_field_record_switches_the_stat_block_on() {
        let chr = Character { name: "Wanderer".to_string(), ..Character::default() };
        assert!(chr.equips.is_empty(), "the default character wears nothing");
        let record = character_record_for_set_field(&chr, 0);
        let fixed = PRESENCE_ARRAY_LEN + 11 + stat_block_len(chr.job) + 4 + 1;
        assert_eq!(
            record.len(),
            fixed + INVENTORY_SIZE_BLOCK_LEN + EQUIPPED_BLOCK_OVERHEAD
        );
        assert!(uses_extended_sp(chr.job), "the default job is on the extended-SP branch");
        assert_eq!(
            record.len(),
            247,
            "224 bytes of record, 12 for the six inventory sizes, and the 11 the equipped \
             gate costs even when empty"
        );

        // And a plain-SP job is one byte longer - the stat block's SP fork is real and
        // encoded, and it shifts everything after it including the equipped block.
        let plain = Character { job: 900, ..chr.clone() };
        assert!(!uses_extended_sp(plain.job));
        assert_eq!(character_record_for_set_field(&plain, 0).len(), 248);

        // The bag: six u16 between the string flags and the equipped list - five at the
        // default, Deco at its 150. Their POSITION is the load-bearing part - the record
        // has no resync point, so twelve bytes in the wrong place silently ruins the
        // equipped list behind them.
        let sizes_at = STAT_BLOCK_AT + stat_block_len(chr.job) + 4;
        assert_eq!(
            &record[sizes_at..sizes_at + INVENTORY_SIZE_BLOCK_LEN],
            &inventory_size_block(&default_inventory_slots())[..]
        );
        // Indexing by the loop variable rather than zipping, on purpose: the assertion is
        // that the record's Nth pair of bytes matches INVENTORY_SLOT_ORDER's Nth entry, and
        // a zip would hide an off-by-one between the two by construction.
        #[allow(clippy::needless_range_loop)]
        for i in 0..INVENTORY_COUNT {
            let at = sizes_at + i * 2;
            assert_eq!(
                u16::from_le_bytes([record[at], record[at + 1]]),
                match INVENTORY_SLOT_ORDER[i] {
                    "deco" => DECO_INVENTORY_SLOTS,
                    "cash" => CASH_INVENTORY_SLOTS,
                    _ => DEFAULT_INVENTORY_SLOTS,
                },
                "inventory {} ({}) did not get the default bag",
                i,
                INVENTORY_SLOT_ORDER[i]
            );
        }

        // presence[0] switches on gate entry 7 (the stat block) and presence[2] on entry 6
        // (the equipped list). Every other flag must stay clear - each one that is set pulls
        // in a whole block we do not build, and the record has no resync point.
        assert_eq!(record[PRESENCE_CHARACTER_STAT], 1, "the stat block is not switched on");
        assert_eq!(record[PRESENCE_EQUIPPED], 1, "the equipped list is not switched on");
        assert_eq!(record[PRESENCE_INVENTORY_SIZE], 1, "the bag is not sized");
        let expected = [PRESENCE_CHARACTER_STAT, PRESENCE_EQUIPPED, PRESENCE_INVENTORY_SIZE];
        assert!(
            record[..PRESENCE_ARRAY_LEN]
                .iter()
                .enumerate()
                .all(|(i, &b)| expected.contains(&i) || b == 0),
            "a presence flag other than the stat block, the equipped list and the \
             inventory sizes is set"
        );

        // The six head fields between the array and the gate are counts and flags the
        // client uses to skip. A non-zero byte in there pulls in loops that read.
        assert!(
            record[PRESENCE_ARRAY_LEN..STAT_BLOCK_AT].iter().all(|&b| b == 0),
            "a head count or flag is non-zero, which would pull in extra reads"
        );

        // The stat block starts where the gate leaves off, and carries the map.
        assert_eq!(
            &record[STAT_BLOCK_AT..STAT_BLOCK_AT + stat_block_len(chr.job)],
            &character_stat_block(&chr, 0)[..]
        );
        let at = STAT_BLOCK_AT + stat_block_map_id_at(chr.job);
        assert_eq!(
            u32::from_le_bytes([record[at], record[at + 1], record[at + 2], record[at + 3]]),
            START_MAP_ID
        );
    }

    /// One equipped item is 125 bytes, and every field is at the offset the client reads it
    /// at. There is no length prefix anywhere in the record, so a single wrong width
    /// desynchronises everything after it - silently.
    #[test]
    fn an_equipped_item_is_125_bytes_with_the_fields_where_the_client_reads_them() {
        let hat = equipped_item(1002357, &EquipStats::default());
        assert_eq!(hat.len(), EQUIPPED_ITEM_LEN);
        assert_eq!(hat.len(), 125);

        // 1403095fb: the factory reads a type byte and dispatches on it. Anything other
        // than 1, 2 or 3 leaves the item null and reads nothing further, which would
        // desynchronise the rest of the record.
        assert_eq!(hat[0], 1);

        // 1403035c5: the item id, immediately after the type byte.
        assert_eq!(u32::from_le_bytes([hat[1], hat[2], hat[3], hat[4]]), 1002357);

        // 140303787: hasCashSN. Non-zero moves an 8-byte field from +0x4d to +0x38.
        assert_eq!(hat[5], 0);
        // And with a serial it is STILL 125 bytes: the raw[8] moves from 14030429e (+0x4d)
        // to 14030379d (+0x38), flag 1 and the serial right after the item id, and every
        // byte from dateExpire onward is the same as the plain body until the dropped raw[8].
        let sn = std::num::NonZeroU64::new(0x1_0000_0003).unwrap();
        let cash_hat = equipped_item_with_cash_sn(1002357, &EquipStats::default(), Some(sn));
        assert_eq!(cash_hat.len(), EQUIPPED_ITEM_LEN, "same length either way");
        assert_eq!(cash_hat[5], 1);
        assert_eq!(&cash_hat[6..14], &sn.get().to_le_bytes(), "14030379d  raw[8] -> +0x38");
        assert_eq!(&cash_hat[14..14 + 65], &hat[6..6 + 65], "dateExpire .. the seven u16 - shifted by 8");
        assert_eq!(&cash_hat[14 + 65..], &hat[6 + 65 + 8..], "after the dropped raw[8] at 14030429e");

        // 1403037b9: dateExpire, and it must NOT be zero - see ITEM_NEVER_EXPIRES.
        let expires = u64::from_le_bytes(hat[6..14].try_into().unwrap());
        assert_eq!(expires, ITEM_NEVER_EXPIRES);
        assert_ne!(expires, 0, "zero is 1601-01-01, an item that expired long ago");

        // 14030381d and 140303b66: the two bitmasks. Every bit of each gates one optional
        // read - 17 u16 and 21 mixed-width fields - so a stray bit here adds bytes the
        // client expects and we do not send.
        assert_eq!(u32::from_le_bytes(hat[19..23].try_into().unwrap()), 0, "statMask");
        assert_eq!(u32::from_le_bytes(hat[23..27].try_into().unwrap()), 0, "optMask");

        // 1403043f6 and 1403043fe: the third mask and the tail flag, the last two fields.
        assert_eq!(u32::from_le_bytes(hat[120..124].try_into().unwrap()), 0, "third mask");
        assert_eq!(hat[124], 0, "tailFlag - non-zero reads a fourth mask");

        // Everything else is zero. That is a claim about the layout, not laziness: the
        // masks read nothing, and the 13-byte name is a buffer whose terminator the client
        // writes itself.
        let carries_a_value = |i: usize| i == 0 || (1..5).contains(&i) || (6..14).contains(&i);
        let stray: Vec<usize> =
            (0..hat.len()).filter(|&i| hat[i] != 0 && !carries_a_value(i)).collect();
        assert!(
            stray.is_empty(),
            "only the type byte, the item id and dateExpire carry a value: {stray:?}"
        );

        // The item id is the only thing that changes between two items.
        let coat = equipped_item(1040010, &EquipStats::default());
        let differing: Vec<usize> =
            (0..hat.len()).filter(|&i| hat[i] != coat[i]).collect();
        assert!(!differing.is_empty(), "two different items produced identical bytes");
        assert!(
            differing.iter().all(|&i| (1..5).contains(&i)),
            "two items differ outside the itemId field: {differing:?}"
        );
    }

    /// One stat set with exactly one bit set, written out by hand.
    ///
    /// This deliberately restates the bit order rather than reusing `in_bit_order`. If the
    /// two ever disagree the tests below fail, which is the point: the order is the wire
    /// format and `research/equip-stats.md` section 2 is its evidence.
    fn stat_set_with(bit: usize, value: u16) -> EquipStatSet {
        let mut s = EquipStatSet::default();
        match bit {
            0 => s.inc_str = value,
            1 => s.inc_dex = value,
            2 => s.inc_int = value,
            3 => s.inc_luk = value,
            4 => s.inc_mhp = value,
            5 => s.inc_mmp = value,
            6 => s.inc_speed = value,
            7 => s.inc_jump = value,
            8 => s.inc_pad = value,
            9 => s.inc_mad = value,
            10 => s.inc_pdd = value,
            11 => s.inc_mdd = value,
            12 => s.inc_acc = value,
            13 => s.inc_eva = value,
            14 => s.inc_crt = value,
            15 => s.inc_crd = value,
            16 => s.inc_wat = value,
            _ => panic!("FUN_140303800 has 17 bits, not {}", bit + 1),
        }
        s
    }

    /// One option set with exactly one bit set. Same reasoning as [`stat_set_with`].
    fn options_with(bit: usize, value: u64) -> EquipOptions {
        let mut o = EquipOptions::default();
        match bit {
            0 => o.remaining_enhancements = value as u8,
            1 => o.unknown_b1 = value as u8,
            2 => o.attribute = value as u16,
            3 => o.unknown_b3 = value as u8,
            4 => o.unknown_b4 = value as u8,
            5 => o.unknown_b5 = value,
            6 => o.unknown_b6 = value as u32,
            7 => o.golden_hammer = value as u32,
            8 => o.level_requirement_reduction = value as u8,
            9 => o.unknown_b9 = value as u16,
            10 => o.unknown_b10 = value as u32,
            11 => o.unknown_b11 = value as u8,
            12 => o.unknown_b12 = value as u8,
            13 => o.unknown_b13 = value as u8,
            14 => o.boss_damage_percent = value as u8,
            15 => o.ignore_enemy_def_percent = value as u8,
            16 => o.damage_percent = value as u8,
            17 => o.all_stats_percent = value as u8,
            18 => o.scissor_uses = value as u8,
            19 => o.unknown_b19 = value,
            20 => o.unknown_b20 = value as u32,
            _ => panic!("FUN_140303b40's mask has 21 bits, not {}", bit + 1),
        }
        o
    }

    /// The widths `FUN_140303b40` reads for its 21 optional fields, in bit order, read off
    /// `research/msexe-itemslot-b40.txt`. The item body has no length prefix, so one wrong
    /// width here would silently move every byte after it - including the four terminators
    /// the record needs and the whole rest of the character.
    const OPTION_WIDTHS: [usize; EQUIP_OPTION_BITS] =
        [1, 1, 2, 1, 1, 8, 4, 4, 1, 2, 4, 1, 1, 1, 1, 1, 1, 1, 1, 8, 4];

    /// Setting stat bit `k` sets exactly bit `k` of the mask and adds exactly two bytes.
    #[test]
    fn every_stat_bit_costs_one_mask_bit_and_two_wire_bytes() {
        for bit in 0..EQUIP_STAT_BITS {
            let set = stat_set_with(bit, 0x1234);
            assert_eq!(set.mask(), 1 << bit, "stat bit {bit} set the wrong mask bit");
            assert_eq!(set.wire_len(), 4 + 2, "stat bit {bit} is not a u16");

            let stats = EquipStats { stats: set, ..EquipStats::default() };
            let item = equipped_item(1040002, &stats);
            assert_eq!(item.len(), EQUIPPED_ITEM_LEN + 2, "stat bit {bit} moved the body");

            // 14030381d: the mask, then the one value it turned on, immediately after it.
            assert_eq!(u32::from_le_bytes(item[19..23].try_into().unwrap()), 1 << bit);
            assert_eq!(u16::from_le_bytes(item[23..25].try_into().unwrap()), 0x1234);
            // 140303b66: the option mask has moved by exactly the two bytes.
            assert_eq!(u32::from_le_bytes(item[25..29].try_into().unwrap()), 0, "optMask");
        }

        // All seventeen at once: 17 * 2 bytes, and every mask bit set.
        let mut all = EquipStatSet::default();
        for bit in 0..EQUIP_STAT_BITS {
            all = merge_stats(all, stat_set_with(bit, u16::try_from(bit + 1).unwrap()));
        }
        assert_eq!(all.mask(), (1 << EQUIP_STAT_BITS) - 1);
        assert_eq!(all.wire_len(), 4 + 2 * EQUIP_STAT_BITS);
        let item = equipped_item(1040002, &EquipStats { stats: all, ..EquipStats::default() });
        assert_eq!(item.len(), EQUIPPED_ITEM_LEN + 2 * EQUIP_STAT_BITS);
        // The values come out in bit order, which is what the client reads them in.
        for bit in 0..EQUIP_STAT_BITS {
            let at = 23 + 2 * bit;
            assert_eq!(
                u16::from_le_bytes(item[at..at + 2].try_into().unwrap()),
                u16::try_from(bit + 1).unwrap(),
                "stat bit {bit} is out of order on the wire"
            );
        }
    }

    /// Field-wise OR of two stat sets, so the test above can build "all bits set" without
    /// restating the 17 field names a third time.
    fn merge_stats(a: EquipStatSet, b: EquipStatSet) -> EquipStatSet {
        EquipStatSet {
            inc_str: a.inc_str | b.inc_str,
            inc_dex: a.inc_dex | b.inc_dex,
            inc_int: a.inc_int | b.inc_int,
            inc_luk: a.inc_luk | b.inc_luk,
            inc_mhp: a.inc_mhp | b.inc_mhp,
            inc_mmp: a.inc_mmp | b.inc_mmp,
            inc_speed: a.inc_speed | b.inc_speed,
            inc_jump: a.inc_jump | b.inc_jump,
            inc_pad: a.inc_pad | b.inc_pad,
            inc_mad: a.inc_mad | b.inc_mad,
            inc_pdd: a.inc_pdd | b.inc_pdd,
            inc_mdd: a.inc_mdd | b.inc_mdd,
            inc_acc: a.inc_acc | b.inc_acc,
            inc_eva: a.inc_eva | b.inc_eva,
            inc_crt: a.inc_crt | b.inc_crt,
            inc_crd: a.inc_crd | b.inc_crd,
            inc_wat: a.inc_wat | b.inc_wat,
        }
    }

    /// Setting option bit `k` sets exactly bit `k` and adds exactly that field's width -
    /// 1, 2, 4 or 8 bytes, per `FUN_140303b40`'s listing.
    // The index IS the subject: bit k must cost OPTION_WIDTHS[k]. Iterating the widths and
    // counting along would assume the correspondence this test exists to check.
    #[allow(clippy::needless_range_loop)]
    #[test]
    fn every_option_bit_costs_one_mask_bit_and_its_own_width() {
        for bit in 0..EQUIP_OPTION_BITS {
            let width = OPTION_WIDTHS[bit];
            assert_eq!(EquipOptions::width_of(bit), width, "width table disagrees at {bit}");

            // A value that fills the field, so a too-narrow write would truncate visibly.
            let value = match width {
                1 => 0x7f,
                2 => 0x7f11,
                4 => 0x7f11_2233,
                _ => 0x7f11_2233_4455_6677,
            };
            let opts = options_with(bit, value);
            assert_eq!(opts.mask(), 1 << bit, "option bit {bit} set the wrong mask bit");
            assert_eq!(opts.wire_len(), 4 + width, "option bit {bit} is not {width} bytes");

            let stats = EquipStats { options: opts, ..EquipStats::default() };
            let item = equipped_item(1040002, &stats);
            assert_eq!(item.len(), EQUIPPED_ITEM_LEN + width, "option bit {bit} moved the body");

            // With no stat bits the option mask is still at 23..27, and its one field after.
            assert_eq!(u32::from_le_bytes(item[23..27].try_into().unwrap()), 1 << bit);
            let mut got = [0u8; 8];
            got[..width].copy_from_slice(&item[27..27 + width]);
            assert_eq!(u64::from_le_bytes(got), value, "option bit {bit} lost bytes");
        }
    }

    /// The third mask is always sent and the fourth only when the tailFlag says so.
    #[test]
    fn the_third_and_fourth_stat_sets_sit_at_the_end_of_the_body() {
        // Zero stats: third mask at 120..124, tailFlag at 124, and that is the last byte.
        let plain = equipped_item(1040002, &EquipStats::default());
        assert_eq!(plain.len(), 125);
        assert_eq!(u32::from_le_bytes(plain[120..124].try_into().unwrap()), 0);
        assert_eq!(plain[124], 0);

        // 1403043f6: the third set (item+0x26b) is gated by its own mask, nothing else.
        let third = EquipStats { second_stats: stat_set_with(3, 9), ..EquipStats::default() };
        let item = equipped_item(1040002, &third);
        assert_eq!(item.len(), EQUIPPED_ITEM_LEN + 2);
        assert_eq!(u32::from_le_bytes(item[120..124].try_into().unwrap()), 1 << 3);
        assert_eq!(u16::from_le_bytes(item[124..126].try_into().unwrap()), 9);
        assert_eq!(item[126], 0, "tailFlag still last");

        // 1403043fe: Some(..) sets the tailFlag, which is what makes 140304411 read a
        // fourth mask. None must leave it zero or the client reads four bytes we never sent.
        let timed =
            EquipStats { timed_stats: Some(stat_set_with(16, 0x0101)), ..EquipStats::default() };
        let item = equipped_item(1040002, &timed);
        assert_eq!(item.len(), EQUIPPED_ITEM_LEN + 4 + 2);
        assert_eq!(item[124], 1, "tailFlag must be non-zero when a fourth set follows");
        assert_eq!(u32::from_le_bytes(item[125..129].try_into().unwrap()), 1 << 16);
        assert_eq!(u16::from_le_bytes(item[129..131].try_into().unwrap()), 0x0101);
    }

    /// The mask can never disagree with the payload, because there is only one source for
    /// both. This is the property the whole struct exists for: the item body has no length
    /// prefix and no resync point, so a mask bit without its bytes desynchronises the rest
    /// of the character record and world entry with it.
    #[test]
    fn a_mask_bit_is_set_exactly_when_its_bytes_are_on_the_wire() {
        let stats = EquipStats {
            stats: EquipStatSet { inc_str: 5, inc_pdd: 6, inc_wat: 7, ..EquipStatSet::default() },
            options: EquipOptions {
                remaining_enhancements: 7,
                attribute: ATTRIBUTE_BIT_3,
                scissor_uses: NO_SCISSOR_RESTRICTION,
                unknown_b5: 0x1122_3344_5566_7788,
                ..EquipOptions::default()
            },
            ..EquipStats::default()
        };

        // Walk the body the way FUN_140303800 and FUN_140303b40 do and land exactly on the
        // third mask - the same walk tools/channel_smoke.py does against the real wire.
        let item = equipped_item(1040002, &stats);
        let mut at = 19;
        let stat_mask = u32::from_le_bytes(item[at..at + 4].try_into().unwrap());
        at += 4;
        assert_eq!(stat_mask, (1 << 0) | (1 << 10) | (1 << 16));
        for bit in 0..EQUIP_STAT_BITS {
            if stat_mask & (1 << bit) != 0 {
                at += 2;
            }
        }
        let opt_mask = u32::from_le_bytes(item[at..at + 4].try_into().unwrap());
        at += 4;
        assert_eq!(opt_mask, 1 | (1 << 2) | (1 << 5) | (1 << 18));
        // Walking the bit index, not the widths: this is decoding a mask, and the bit
        // number is what selects the width.
        #[allow(clippy::needless_range_loop)]
        for bit in 0..EQUIP_OPTION_BITS {
            if opt_mask & (1 << bit) != 0 {
                at += OPTION_WIDTHS[bit];
            }
        }
        // 40..120 of the zero-stat body: the name buffer through the two blob bytes.
        at += 120 - 27;
        assert_eq!(
            u32::from_le_bytes(item[at..at + 4].try_into().unwrap()),
            0,
            "the walk did not land on the third mask - a width is wrong"
        );
        assert_eq!(at + 4 + 1, item.len(), "the tailFlag is not the last byte");
        assert_eq!(item.len(), EQUIPPED_ITEM_LEN + stats.extra_len());
    }

    /// A fresh Undershirt (1040002) as `Character.wz` describes it, and the three wrong
    /// lines it fixes.
    ///
    /// `01040002.img/info` has `incPDD 6` and `tuc 7` and nothing else, so a real server
    /// sends `inc_pdd = 6`. The client reads that as the item's **total** Weapon Def. -
    /// `FUN_142699710` prints it as the headline and *subtracts* `ITEMINFO.incPDD` to get
    /// the scroll leftover, and `FUN_14038d3c0` compares the two in the same units at
    /// `0x14038d434`. Sending 0 trips that function's opening guard
    /// (`arg5 <= 0 && arg4 <= 0 -> return`) and suppresses the whole stat section, which is
    /// why the owner's screenshot had no stat lines at all.
    ///
    /// `FUN_1402fd610` (the item's `vtable+0x200`) is `cashSerial == 0 && scissorUses <=
    /// 0x14`. With zero uses that is true, which makes `FUN_14038cf10` report a trade
    /// restriction for any item whose WZ `tradeBlock` is 0 - every starter item - and also
    /// makes `FUN_1403e8c40` print `Scissors Usages Available : 0`.
    #[test]
    fn a_fresh_item_carries_its_template_stats_an_upgrade_count_and_no_karma_restriction() {
        // Constant by construction, and that is the point: it pins the two constants
        // against each other so a future edit to either cannot quietly put the value back
        // inside the range the client treats as restricted.
        #[allow(clippy::assertions_on_constants)]
        {
            assert!(
                NO_SCISSOR_RESTRICTION > SCISSOR_RESTRICTED_MAX,
                "the whole point is to land above the 0x14 the client compares against"
            );
        }

        // 01040002.img/info: incPDD 6, tuc 7. Everything else absent.
        let mut template = [0u16; EQUIP_STAT_BITS];
        template[10] = 6;
        assert_eq!(EQUIP_STAT_WZ_PROPERTIES[10], "incPDD", "bit 10 is Weapon Def.");
        let template = EquipStatSet::from_wz_template(template);
        assert_eq!(template.inc_pdd, 6);

        let stats = EquipStats::fresh(template, 7);
        assert_eq!(stats.stats.inc_pdd, 6, "the total, not an increment over the WZ");
        assert_eq!(stats.options.remaining_enhancements, 7, "01040002.img/info/tuc");
        assert_eq!(stats.options.scissor_uses, NO_SCISSOR_RESTRICTION);
        assert_eq!(stats.stats.mask(), 1 << 10);
        assert_eq!(stats.options.mask(), (1 << 0) | (1 << 18));

        // One u16 and two u8: four bytes, and that is the whole cost of all three fixes.
        assert_eq!(stats.extra_len(), 4);
        let item = equipped_item(1040002, &stats);
        assert_eq!(item.len(), 129);
        assert_eq!(u32::from_le_bytes(item[19..23].try_into().unwrap()), 1 << 10, "statMask");
        assert_eq!(u16::from_le_bytes(item[23..25].try_into().unwrap()), 6, "incPDD");
        assert_eq!(u32::from_le_bytes(item[25..29].try_into().unwrap()), (1 << 0) | (1 << 18));
        assert_eq!(item[29], 7, "bit 0: remaining enhancements");
        assert_eq!(item[30], NO_SCISSOR_RESTRICTION, "bit 18: scissor uses");

        // remaining_enhancements is not free: FUN_14038d3c0 compares it against tuc.
        assert!(
            stats.options.remaining_enhancements <= 7,
            "a count above ITEMINFO.tuc is outside the range the client models"
        );

        // And the fallback is still one call away, byte-for-byte what ran on screen.
        assert_eq!(equipped_item(1040002, &EquipStats::default()).len(), EQUIPPED_ITEM_LEN);
    }

    /// The equipped block costs 11 bytes even when the character wears nothing, because
    /// presence[2] gates three list readers and not one. Forgetting the four extra
    /// terminators is the single most likely way to break the record.
    #[test]
    fn the_equipped_block_carries_five_terminators_not_one() {
        let empty = equipped_block(&[]);
        assert_eq!(empty.len(), EQUIPPED_BLOCK_OVERHEAD);
        assert_eq!(empty.len(), 11);
        assert!(empty.iter().all(|&b| b == 0), "flagA and all five terminators are zero");

        // flagA at 1403061cc. Zero is what makes FUN_14030b6f0 run, which is why its
        // terminator is one of the five.
        assert_eq!(empty[0], 0);

        // Four starter equips: the four slots TestCharD actually has in the database, which
        // are the values the client's own slot validator FUN_140253980 assigns.
        let equips = vec![(5u8, 1040002u32), (6, 1060002), (7, 1072001), (11, 1302000)];
        let block = equipped_block(&equips);
        assert_eq!(block.len(), EQUIPPED_BLOCK_OVERHEAD + 4 * EQUIPPED_ENTRY_LEN);
        assert_eq!(block.len(), 11 + 4 * 127);

        // Each entry is a u16 slot then the 125-byte item, in the order given.
        let mut at = 1;
        for (slot, item_id) in &equips {
            assert_eq!(u16::from_le_bytes([block[at], block[at + 1]]), u16::from(*slot));
            assert_eq!(
                &block[at + 2..at + 2 + EQUIPPED_ITEM_LEN],
                &equipped_item(*item_id, &EquipStats::default())[..]
            );
            at += EQUIPPED_ENTRY_LEN;
        }

        // Then five u16 zeros and nothing else.
        assert!(block[at..].iter().all(|&b| b == 0));
        assert_eq!(block.len() - at, 10);

        // A slot outside 1..=31 is decoded by the client and thrown away, so it costs 127
        // bytes of wire and achieves nothing. Drop it rather than send it.
        let with_cash = vec![(5u8, 1040002u32), (105, 1040002), (0, 1040002)];
        assert_eq!(
            equipped_block(&with_cash).len(),
            EQUIPPED_BLOCK_OVERHEAD + EQUIPPED_ENTRY_LEN,
            "slots 105 and 0 are outside 1..=31 and should not be sent"
        );
    }

    /// The owner, 2026-09-12: Cobalt relogged undressed. The store had the Ubel set in slots
    /// 105/107/108/111 and the look sent those numbers raw, which the reader discards.
    #[test]
    fn a_cash_equip_is_drawn_at_its_base_slot_and_the_item_it_covers_goes_to_the_second_map() {
        // Cobalt's actual rows, in store order: four ordinary, four cash.
        let worn = vec![
            (5u8, 1040021u32),
            (6, 1062999),
            (7, 1072999),
            (11, 1322999),
            (105, 1054562),
            (107, 1074238),
            (108, 1082878),
            (111, 1703726),
        ];
        let layout = look_layout(&worn);
        let (drawn, covered) = look_maps(&worn);
        assert_eq!((&layout.drawn, &layout.covered), (&drawn, &covered));
        assert_eq!(
            drawn,
            vec![(5, 1054562), (6, 1062999), (7, 1074238), (8, 1082878), (11, 1322999)],
            "cash over the base slot; the bottom, which has no cash cover, stays; and the \
             WEAPON stays - the cover is not a weapon"
        );
        assert_eq!(
            covered,
            vec![(5, 1040021), (7, 1072999)],
            "only the covered ones - slot 8 had nothing under the gloves, and the weapon is \
             not covered, it is stickered"
        );
        assert_eq!(layout.weapon_sticker, 1703726, "Ubel's Cane rides in the sticker field");
        // A cover with no weapon under it: sticker set, slot 11 empty. A cash item in slot
        // 111 that is NOT a cover (nothing this server makes, but the rule has to say) goes
        // the ordinary way.
        assert_eq!(
            look_layout(&[(111, 1703726)]),
            LookLayout { drawn: vec![], covered: vec![], weapon_sticker: 1703726 }
        );
        assert_eq!(
            look_layout(&[(11, 1322999), (111, 1302000)]),
            LookLayout { drawn: vec![(11, 1302000)], covered: vec![(11, 1322999)], weapon_sticker: 0 }
        );
        for (slot, _) in drawn.iter().chain(covered.iter()) {
            assert!(EQUIP_SLOTS.contains(slot), "every wire slot is one the reader keeps");
        }

        // Nothing cash: byte-identical to the list that has been through the reader.
        let plain: Vec<(u8, u32)> = worn[..4].to_vec();
        assert_eq!(look_maps(&plain), (plain.clone(), Vec::new()));
        // Garbage slots are dropped, not sent.
        assert_eq!(look_maps(&[(0, 1), (32, 2), (100, 3), (132, 4)]), (Vec::new(), Vec::new()));

        // And the bytes: the drawn map, 0xFF, the covered map, 0xFF, straight after hair.
        let chr = Character { hair: 0xBBBB_BBBB, equips: worn, ..Character::default() };
        let look = avatar_look(&chr);
        let hair_at = look
            .windows(4)
            .position(|w| w == 0xBBBB_BBBBu32.to_le_bytes())
            .expect("hair is in the look");
        let mut want = Vec::new();
        for (slot, id) in drawn.iter().chain(std::iter::once(&(0xFFu8, 0u32))) {
            want.push(*slot);
            if *slot != 0xFF {
                want.extend_from_slice(&id.to_le_bytes());
            }
        }
        for (slot, id) in &covered {
            want.push(*slot);
            want.extend_from_slice(&id.to_le_bytes());
        }
        want.push(0xFF);
        want.extend_from_slice(&1703726u32.to_le_bytes()); // the sticker, look+0x2d
        want.extend_from_slice(&[0u8; 12]); // +0x31, +0x35, +0x1c1
        assert_eq!(&look[hair_at + 4..hair_at + 4 + want.len()], &want[..]);
        // The client's walk still lands on the end of the record.
        let record = character_record(&chr, 0);
        assert_eq!(read_character_record(&record), record.len());
    }

    /// The second half of the same bug: without this block the worn cash items are in no
    /// list the client has, so the equip window shows them empty and they cannot be taken
    /// off. Shape from the listing at 0x140306654..0x140306830 - flagA, entries at the BASE
    /// slot, then four u16 terminators.
    #[test]
    fn the_cash_equipped_block_uses_base_slots_and_carries_four_terminators() {
        let empty = cash_equipped_block(&[]);
        assert_eq!(empty.len(), CASH_EQUIPPED_BLOCK_OVERHEAD);
        assert_eq!(empty.len(), 9);
        assert!(empty.iter().all(|&b| b == 0));

        let worn = vec![
            (5u8, 1040021u32, EquipStats::default()), // ordinary - not this block's
            (105, 1054562, EquipStats::default()),
            (111, 1703726, EquipStats::default()),
            (132, 1, EquipStats::default()), // past the 31 the reader keeps - dropped
            (100, 1, EquipStats::default()), // base 0 - dropped
        ];
        let block = cash_equipped_block(&worn);
        assert_eq!(block.len(), CASH_EQUIPPED_BLOCK_OVERHEAD + 2 * EQUIPPED_ENTRY_LEN);
        assert_eq!(block[0], 0, "flagA stays 0 so FUN_14030b6f0(6) runs and eats its u16");
        let mut at = 1;
        for (base, item_id) in [(5u16, 1054562u32), (11, 1703726)] {
            assert_eq!(u16::from_le_bytes([block[at], block[at + 1]]), base, "BASE slot, not 105");
            assert_eq!(
                &block[at + 2..at + 2 + EQUIPPED_ITEM_LEN],
                &equipped_item(item_id, &EquipStats::default())[..]
            );
            at += EQUIPPED_ENTRY_LEN;
        }
        assert_eq!(&block[at..], &[0u8; 8], "list end, Deco bag end, two b9e0(6) lists");
    }

    /// The block is gated by presence[44], sits right after the first equipped block, and
    /// is absent - byte for byte - for a character wearing nothing cash.
    #[test]
    fn a_character_wearing_cash_opens_presence_44_and_one_without_sends_the_old_record() {
        let chr = Character {
            equips: vec![(5, 1040002), (6, 1060002), (7, 1072001), (11, 1302000)],
            ..Character::default()
        };
        let plain: Vec<(u8, u32, EquipStats)> =
            chr.equips.iter().map(|&(s, i)| (s, i, EquipStats::default())).collect();
        let without = character_record_for_set_field_with(&chr, 0, &plain);
        assert_eq!(without[PRESENCE_CASH_EQUIPPED], 0);

        let mut dressed = plain.clone();
        dressed.push((105, 1054562, EquipStats::default()));
        dressed.push((111, 1703726, EquipStats::default()));
        let mut chr_cash = chr.clone();
        chr_cash.equips.extend([(105, 1054562), (111, 1703726)]);
        let with = character_record_for_set_field_with(&chr_cash, 0, &dressed);
        assert_eq!(with[PRESENCE_CASH_EQUIPPED], 1);
        assert_eq!(
            with.len() - without.len(),
            CASH_EQUIPPED_BLOCK_OVERHEAD + 2 * EQUIPPED_ENTRY_LEN,
            "exactly one block of two entries was added"
        );
        // Placement: everything before the block is the old record minus its final byte,
        // and the block is followed by that byte alone.
        let block = cash_equipped_block(&dressed);
        let head = without.len() - 1;
        assert_eq!(&with[PRESENCE_ARRAY_LEN..head], &without[PRESENCE_ARRAY_LEN..head]);
        assert_eq!(&with[head..head + block.len()], &block[..]);
        assert_eq!(&with[head + block.len()..], &[0u8], "then the ungated read at 0x140308b3f");
        // The look inside the same record drew the cash items at the base slot.
        let look = avatar_look(&chr_cash);
        let mut cash_over_coat = vec![5u8];
        cash_over_coat.extend_from_slice(&1054562u32.to_le_bytes());
        assert!(look.windows(5).any(|w| w == cash_over_coat.as_slice()));
    }

    /// A dressed character's record is 743 bytes, and the equipped block sits between the
    /// three optional-string flags and the final ungated read. That position is the whole
    /// point: it came from walking all 18660 bytes of FUN_140304b20, and the walk's control
    /// is that with presence = {0} it reproduces the 224-byte record already on the wire.
    #[test]
    fn a_dressed_character_puts_the_equipped_block_after_the_string_flags() {
        let chr = Character {
            name: "TestCharD".to_string(),
            equips: vec![(5, 1040002), (6, 1060002), (7, 1072001), (11, 1302000)],
            ..Character::default()
        };
        let record = character_record_for_set_field(&chr, 0);
        assert_eq!(record.len(), 755, "four equips and a bag take the record to 755");

        // The stat block is still where the gate leaves the stream pointer, and still
        // carries the map. Equipment must not have moved it.
        let block_at = STAT_BLOCK_AT + stat_block_len(chr.job);
        assert_eq!(
            &record[STAT_BLOCK_AT..block_at],
            &character_stat_block(&chr, 0)[..]
        );
        let map_at = STAT_BLOCK_AT + stat_block_map_id_at(chr.job);
        assert_eq!(
            u32::from_le_bytes(record[map_at..map_at + 4].try_into().unwrap()),
            START_MAP_ID
        );

        // 219..223: the u8 and the three optional-string flags, all zero so no string is
        // read. Then loop #6's six inventory sizes, then the gate fires.
        assert_eq!(&record[block_at..block_at + 4], &[0, 0, 0, 0]);
        let equipped_at = block_at + 4 + INVENTORY_SIZE_BLOCK_LEN;
        assert_eq!(
            &record[equipped_at..record.len() - 1],
            &equipped_block(&chr.equips)[..]
        );

        // And one ungated u8 after the whole region, at 0x140308b3f.
        assert_eq!(*record.last().unwrap(), 0);

        // Undressing the character shortens the record by exactly four entries and changes
        // nothing before the block - the strongest single check that the block is placed
        // where the walk says.
        let naked = Character { equips: Vec::new(), ..chr.clone() };
        let bare = character_record_for_set_field(&naked, 0);
        assert_eq!(record.len() - bare.len(), 4 * EQUIPPED_ENTRY_LEN);
        assert_eq!(&record[..equipped_at], &bare[..equipped_at]);
    }

    /// The whole packet, and the one difference from the minimal form that was accepted.
    #[test]
    fn set_field_with_character_differs_from_minimal_only_where_intended() {
        let chr = Character { name: "Wanderer".to_string(), ..Character::default() };
        let full = set_field_with_character(&chr, 0, 7, 3);
        let minimal = set_field_minimal(7, 3);

        // The 33-byte head is unchanged - that head reached the right handler on a live
        // client, so nothing in it should move.
        assert_eq!(&full[..SET_FIELD_HEAD_LEN], &minimal[..SET_FIELD_HEAD_LEN]);
        assert_eq!(full[SET_FIELD_CHARACTER_DATA_AT], SET_FIELD_WITH_CHARACTER_DATA);

        // The record begins after the head and the three u32s.
        let record_at = SET_FIELD_HEAD_LEN + 12;
        assert_eq!(full[record_at + PRESENCE_CHARACTER_STAT], 1);
        assert_eq!(minimal[record_at + PRESENCE_CHARACTER_STAT], 0, "the old form set no flag");
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

    /// A refusal is the success shape with the code byte changed: nothing a client reads past
    /// the code can run off the end.
    #[test]
    fn a_refused_login_is_the_success_shape_with_the_code_set() {
        let refused = login_refused(LOGIN_REFUSED_NOT_REGISTERED);
        let ok = login_result(0, 0, &[]);
        assert_eq!(refused[0], 5, "notRegisteredID");
        assert_eq!(ok[0], LOGIN_OK);
        assert_eq!(refused.len(), ok.len());
        assert_eq!(&refused[1..], &ok[1..]);
    }

    /// **The "already logged in" code is 7 and is not one of the codes that PROCEED.**
    ///
    /// `FUN_141b267c0` returns 1 - carry on and read the body - for exactly `0` and `0x0C`.
    /// Sending a refusal that the client proceeds on would show a dialog and then log the
    /// second player in anyway, which is the failure mode `MIGRATE_REFUSED`'s doc block
    /// records for `-1`, `6`, `8` and `9` on the migrate path.
    #[test]
    fn the_already_logged_in_code_refuses_rather_than_proceeding() {
        assert_eq!(LOGIN_REFUSED_ALREADY_LOGGED_IN, 7);
        for proceeds in [LOGIN_OK, 0x0C] {
            assert_ne!(
                LOGIN_REFUSED_ALREADY_LOGGED_IN, proceeds,
                "code {proceeds} makes FUN_141b267c0 return 1, so the client would log in anyway"
            );
        }
        // And it is a distinct notice from the one an unattributed connection gets, or a
        // player told "already logged in" could not tell it from "sign in through the
        // launcher" - two different problems with two different fixes.
        assert_ne!(LOGIN_REFUSED_ALREADY_LOGGED_IN, LOGIN_REFUSED_NOT_REGISTERED);

        let refused = login_refused(LOGIN_REFUSED_ALREADY_LOGGED_IN);
        assert_eq!(refused[0], 7);
        // Same well-formed body as a success, so a client that reads past the gate finds
        // fields rather than the end of the packet. "Always answer" is about the shape too.
        assert_eq!(refused.len(), login_result(0, 0, &[]).len());
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
