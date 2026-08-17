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
}
