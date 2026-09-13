//! Opcode names, and the logging policy both servers follow.
//!
//! # Why this exists
//!
//! The owner, 2026-08-19: every server needs enough logging for known *and* unknown packets
//! that an unexpected result can be explained from the log afterwards, without a re-run.
//! A client launch costs a manual elevated run, so a run that produced a surprise and no
//! evidence is a run spent twice.
//!
//! That turns into two rules, and they pull in opposite directions:
//!
//! * **An unknown packet is the evidence, so it is never truncated.** Every byte of
//!   anything this codebase cannot name goes in the log, however long it is.
//! * **A known packet is not evidence, so it is capped.** A 1200-byte login result we
//!   built ourselves would bury the lines either side of it, and those lines are usually
//!   what is being read.
//!
//! Both directions are named, because a log that says `0x0078` and a log that says
//! `0x0078 CLIENT_SELECT_CHARACTER_REQUEST` cost the same to write and not the same to
//! read at two in the morning.

use crate::opcode::*;

/// How much of a **known** packet's body goes in the log.
///
/// Bodies are logged at all because not logging them cost a diagnosis once: a create reply
/// the client ignored had to be reconstructed by hand to find that it differed from a
/// working one in two bytes.
pub const KNOWN_BODY_BYTES: usize = 96;

/// The name of an opcode, if this codebase has one for it.
///
/// `None` is the interesting answer: it means the packet is undecoded, and the servers
/// log those in full and mark them so a `grep UNKNOWN` over a run finds every one.
pub fn opcode_name(opcode: u16) -> Option<&'static str> {
    Some(match opcode {
        // Server -> client.
        ACCOUNT_INFO => "ACCOUNT_INFO",
        WORLD_LIST => "WORLD_LIST",
        MIGRATE_COMMAND => "MIGRATE_COMMAND",
        LOGIN_RESULT => "LOGIN_RESULT",
        ACCOUNT_INFO_ALT => "ACCOUNT_INFO_ALT",
        CHECK_NAME_RESULT => "CHECK_NAME_RESULT",
        CREATE_CHARACTER_RESULT => "CREATE_CHARACTER_RESULT",
        DELETE_CHARACTER_RESULT => "DELETE_CHARACTER_RESULT",
        DATA_WZ_PATCH => "DATA_WZ_PATCH",
        ENTER_CREATION_RESULT => "ENTER_CREATION_RESULT",

        // Client -> server.
        CLIENT_SELECT_CHARACTER_REQUEST => "CLIENT_SELECT_CHARACTER_REQUEST",
        CLIENT_LOGIN_REQUEST => "CLIENT_LOGIN_REQUEST",
        CLIENT_CHECK_NAME_REQUEST => "CLIENT_CHECK_NAME_REQUEST",
        CLIENT_LEAVE_WORLD_REQUEST => "CLIENT_LEAVE_WORLD_REQUEST",
        CLIENT_CREATE_CHARACTER_REQUEST => "CLIENT_CREATE_CHARACTER_REQUEST",
        CLIENT_DELETE_CHARACTER_REQUEST => "CLIENT_DELETE_CHARACTER_REQUEST",
        CLIENT_DATA_WZ_REQUEST => "CLIENT_DATA_WZ_REQUEST",
        CLIENT_ENTER_CREATION_REQUEST => "CLIENT_ENTER_CREATION_REQUEST",

        // Established by reading the client, not by guessing at the bytes.
        0x0073 => "CLIENT_SESSION_IDENTITY (never answered)",
        0x007D => "CLIENT_MIGRATION_HELLO (character id at offset 8, then MAC and machine id)",
        0x008F..=0x0091 => "CLIENT_ELOG (the client's own error log; decode_elog.py)",
        // **The client handing back a packet it could not process, and it is worth its
        // weight.** Named 2026-08-28: a `0x055E` type 10 went out, the client replied with a
        // 2075-byte `0x009E` and then faulted. The body is a 14-byte header, then the
        // **rejected packet in full, opcode included** - 14 + 2 + 2059 = 2075, checked
        // against the exact bytes we had sent.
        //
        // That echo is what localised the crash to one packet in one reading of one log,
        // with no client run of its own. Anything appearing here is a packet the client
        // refused; the header's meaning is **not** decoded.
        0x009E => "CLIENT_PACKET_REJECTED (14-byte header, then the offending packet with its opcode)",

        // Established 2026-08-19 by finding each builder in the client. Five of the eight
        // names previously here were WRONG - they had been inferred from the shape of the
        // bytes. Evidence per opcode in `research/msexe-client-opcodes.md`.
        //
        // 0x00C0 is the one that mattered: it was logged as "CLIENT_HELLO", and it is the
        // opposite of a hello. FUN_141b2a660 builds it *only* when the auth call
        // FUN_141d60eb0(user, pass, 0xc9, 0) fails, and the body is the launch mode plus
        // the error code. Our captures carry 0x4E20 = 20000, a Nexon Passport error - so
        // the client has been reporting a failed login on every run and the log called it
        // a greeting.
        0x00C0 => "CLIENT_AUTH_FAILURE_REPORT (mode, then a Passport error code)",
        0x0070 => "CLIENT_ENV_REPORT (subtype-multiplexed; the 100 is a literal, not our version)",
        0x032C => "CLIENT_DROP_PICK_UP (u32 dropObjectId at body offset 13; MEASURED 2026-08-20)",
        0x00D9 => "CLIENT_USER_MOVE (the client's own movement; x/y are the path's END, not its head)",
        0x0071 => "CLIENT_ENV_DETAIL (its 1/100/0 are literals, not an echo of ours)",
        0x0079 => "CLIENT_LOAD_TIMING_REPORT",
        0x007A => "CLIENT_TASK_TIMING_REPORT (four task durations, then their sum)",
        0x00A6 => "CLIENT_STATUS_CODE",
        0x00BF => "CLIENT_TITLE_SCREEN_READY",

        // The channel connection. Each of these was named by the CLIENT identifying its own
        // request - the owner used a feature and the capture showed what went out - which has now
        // cost no static analysis at all and has never been wrong. The exception is 0x0151:
        // it was first recorded as "NPC click" with its first u32 read as our object id, and
        // our own logs disproved that (every map's first NPC is object id 1000, yet the
        // client answered 1000/1002/1003/1005). It is the quest request and that field is a
        // quest id. See research/npc-dialogue.md.
        0x00D1 => "CLIENT_TRANSFER_FIELD (portal; 0xFFFFFFFF target means resolve the name)",
        0x014A => "CLIENT_PORTAL_SCRIPT (u8, str portal, i16 x, i16 y) - a SCRIPT portal (pt 7/8); never 0x00D1",
        0x014C => "CLIENT_PORTAL_TELEPORT (u8, str from, str to, i16 x, i16 y, i16 x2, i16 y2, u8) - an in-map hidden-portal hop the client performs itself",
        0x00DC => "CLIENT_FIELD_ENTERED (once per SetField, ~420 ms after; empty body)",
        0x00E7 => "CLIENT_CHAT (u32 tick, u16-length text, u8 tab)",
        0x0231 => "USER_CHAT (balloon over the head, and the chat log line)",
        0x02B2 => "USER_HP_REMOTE (u32 charId, u32 hp, u32 maxHp) - a party member's HUD gauge and over-head bar",
        0x0107 => "CLIENT_INVENTORY_MOVE (u32 tick, u8 invType, i16 src, i16 dst, i16 count)",
        0x00F2 => "CLIENT_NPC_CLICK (u32 npcObjectId, i16 charX, i16 charY, u32; the NO-QUEST click path)",
        0x00F3 => "CLIENT_SCRIPT_REPLY (u32 handle, u8 msgType, u32 echo, str the box's own text, i8 action)",
        0x00F6 => "CLIENT_STORAGE (u8 mode: 4 take out, 5 put in, 6 sort, 7 mesos i64, 8 close)",
        0x0114 => "CLIENT_USE_CASH_ITEM (u32 tick, u16 slot, u32 itemId) - coupons, the collection box, set coupons",
        0x0116 => "CLIENT_USE_STAT_RESET_ITEM (u32 tick, u16 slot, u32 itemId) - the AP and SP Reset Scrolls",
        0x0165 => "CLIENT_BEAUTY_COUPON_CONFIRM (u16 slot, u32 itemId, u16) - the Beauty Coupon dialog's Confirm",
        0x013C => "CLIENT_SKILL_USE (u32 skillId, u32 level, then a tail)",
        0x013F => "CLIENT_SKILL_CANCEL (u32 skillId, 5 bytes, raw[124] CTS mask; RETRIES every ~180ms)",
        0x00D5 => "CLIENT_CASH_SHOP_REQUEST (u32 tick, u8; an EXCLUSIVE REQUEST - it latches ctx+0x2330 and only fires once until answered)",
        0x01A3 => "SET_CASH_SHOP (FILETIME, the character record, u8 u8 u8, u16 u16 u32 list counts)",
        0x05AD => "CASH_SHOP_WALLET (u32 nxCredit, u32 maplePoint, u32 discarded) - the ONLY packet carrying a balance",
        0x03E0 => "CLIENT_CASH_SHOP_QUERY (empty; the client throttles it to 60s)",
        // The buy layout is MEASURED - three captures, 2026-08-26, serial at payload offset 7.
        // The rest of the sub-ops are one queue of 32-byte records; see research/cash-shop-actions.md.
        0x03E1 => "CLIENT_CASH_SHOP_ACTION (u8 sub-op: 0x02/0x1F buy = u8,u32,u8,u8,u32 SN,u32; 0x03 gift; 0x0A/0x0B/0x1C the QUEUED move and delete; 0x2B the ONE that does not latch, so it is deliberately unanswered)",
        0x05AE => "CASH_SHOP_RESULT (u8 sub-op; 0x1A + u8 reason refuses a BUY and empties the queue, 0x3D + u16 reason refuses a QUEUED op without emptying it, 0x05/0x07 EJECT the player)",
        0x007E => "TEMPORARY_STAT_RESET (u8,u8,u8, raw[124] mask, tail)",
        0x0572 => "STORAGE_RESULT (u8 mode: 24 open, 13 put ok, 15 refresh, 10/11/16/17 refusals)",
        0x01BE => "CLIENT_LOG_OUT (empty body; POISONS SetField until 0x0106 answers it)",
        0x00BB => "CHAT_NOTICE",
        0x0106 => "LOG_OUT_RESULT",
        0x0151 => "CLIENT_QUEST_REQUEST (u8 action, u32 questId, u32 npcTemplateId, shape-dependent tail)",
        // **Not "create".** Create is action 0 of SEVEN in a FlatBuffers table - create,
        // leave, pickup-rights, invite, join-request, expel, change-leader - read off the
        // party window's own button dispatcher `FUN_1411c9540`, which compares the UTF-16
        // literals `create`/`invite`/`expel`/`leave`/`pickup`/`leader`. The old name came
        // from one capture of one button press and was committed in a change about NPC
        // dialogue. `research/party.md`.
        0x0182 => "CLIENT_PARTY_REQUEST",
        0x0183 => "CLIENT_PARTY_INVITE_ANSWER",
        0x00A5 => "PARTY_RESULT",
        0x0238 => "CLIENT_FIRST_FIELD_ENTRY (empty; only on the very first entry, not per SetField)",
        0x024D => "CLIENT_FIRST_FIELD_ENTRY_2 (empty; built by FUN_142caa4e0 alongside 0x0238)",

        // The mob move report. Named from the client's own builder rather than from a
        // capture: FUN_141cb6880 is mob primary-vtable slot 22 and contains exactly one
        // COutPacket construction, `141cb7ea5 MOV EDX,0x2ff`. It carries the mob's object
        // id, a local move counter, and a movement path built by FUN_141d57c60 - the same
        // encoder the player's own 0x00D9 uses. **This arriving at all is the confirmation
        // that MobChangeController works**; nothing in the client's mob pool acknowledges
        // it. research/mob-behaviour.md.
        0x02FF => "CLIENT_MOB_MOVE (u32 mobObjectId, u16 moveId, then a conditional tail and the path)",

        // The three attack opcodes that share one body: header FUN_140f31fe0, targets
        // FUN_140f31f60, trailer FUN_14083b270. The client works out its own damage and
        // sends it as a u64 per hit, so nothing we reply supplies a number. Their BODIES
        // are the only evidence there is about whether the client targets our mobs, which
        // is why all three are in never_truncate(). research/mob-combat.md.
        0x00DF => "CLIENT_MELEE_ATTACK (u64 damages per target; the CLIENT computes them)",
        0x00E0 => "CLIENT_SHOOT_ATTACK (same body as 0x00DF)",
        0x00E1 => "CLIENT_MAGIC_ATTACK (same body as 0x00DF)",

        // **Handled for weeks, never named.** Each of these has a dispatcher arm in
        // crates/world/src/session/mod.rs and a module in this crate whose doc block is the
        // evidence; they logged as UNKNOWN because nobody added the row here. Found
        // 2026-09-12 by resolving every constant the dispatcher matches and diffing against
        // this table - `every_opcode_the_world_dispatcher_matches_has_a_name` keeps it so.
        0x00D2 => "CLIENT_CHANGE_CHANNEL (a Change Channel row; LATCHES until answered)",
        0x00DA => "CLIENT_CHAIR_CANCEL (u16 chairId, 0xFFFF for none) - stand up",
        0x00DB => "CLIENT_CHAIR_SIT (the Set Up chair's item id and slot)",
        0x00E5 => "CLIENT_USER_HIT (the player took damage; 147 bytes; answered with STAT_CHANGED)",
        0x00F5 => "CLIENT_CLASSIC_SHOP_REQUEST (the classic shop window; research/classic-shop-rows.md)",
        0x0104 => "CLIENT_SHOP_REQUEST (the Shop2 window; a different body from 0x00F5)",
        0x010E => "CLIENT_USE_ITEM (u32 tick, u16 slot, u32 itemId; the Use tab)",
        0x0111 => "CLIENT_SUMMON_SACK (the summoning sack in this slot)",
        0x032F => "CLIENT_REACTOR_HIT (u32 objectId, u32 hitOption, u16 delay, u32 skillId - a breakable box struck; net::reactor)",
        0x0330 => "CLIENT_REACTOR_TOUCH (u32 objectId, ...; a reactor walked into; not answered)",
        0x0125 => "CLIENT_ITEM_UPGRADE (u32 tick, u16 scroll slot, u16 dst slot, ...) - a scroll",
        0x0138 => "CLIENT_ABILITY_UP (inbound; the OUTBOUND 0x0138 is USER_AVATAR_MODIFIED)",
        0x0139 => "CLIENT_ABILITY_MASS_UP (inbound)",
        0x013B => "CLIENT_USER_SKILL_UP_REQUEST (u32 tick, u32 skillId)",
        0x0143 => "CLIENT_DROP_MONEY (8 bytes; the 'how many will you drop' prompt)",
        0x017E => "CLIENT_MINIROOM (trade invite and the rest of the miniroom ops; does NOT latch)",
        0x0199 => "CLIENT_KEYMAP_CHANGE (u8 subtype; 0 = a delta of key bindings) - CONFIRM in KEY BINDINGS",
        0x01E7 => "CLIENT_REVIVE_ON_SPOT (the tombstone's revive-here button)",
        0x0453 => "NPC_CHAT (outbound; one NPC's chat balloon)",

        // **Client reports: sent without waiting, and answered with nothing on purpose.**
        // Every one of these arrived unanswered in the archive while the client played on,
        // which is the measurement that they do not latch; the shapes and provenance are
        // from the client's builders (research/msexe-send-opcodes.txt) and the notes cited.
        // `is_client_report()` lists them so the servers' logs say "a report; nothing is
        // expected back" instead of "not answered yet", and `grep UNKNOWN` over a run is
        // left meaning what it should: a packet nobody has seen before.
        0x013D => "CLIENT_SEND_COUNTER_CENSUS (every 30 s; counts of packets sent; MUST NOT be answered - research/buffs.md sec 2)",
        0x02F4 => "CLIENT_REPORT_02F4 (u32, u32, i32; a few per session; builder FUN_142937ba0; meaning undecoded)",
        0x01ED => "CLIENT_LOG_CHANNEL (u32 kind, then a kind-specific record; 104 builder sites - research/cash-shop.md sec 3.5)",
        0x01A5 => "CLIENT_SKILL_CHECKSUMS (n1 x {skillId, checksum}, n2 x {skillId, level, checksum}; after every skill-up)",
        0x02DE => "CLIENT_FIELD_ENTRY_FLAG (u8; once per field entry, alongside 0x00DC)",
        0x0184 => "CLIENT_FIELD_ENTRY_REPORT (body from FUN_142df2760; once per field entry)",
        0x0194 => "CLIENT_FIELD_ENTRY_REPORT_2 (u8; once per field entry; builder FUN_142defc50)",
        0x00B8 => "CLIENT_TOGGLE_B8 (u8 0/1; a few per session; builder FUN_142e40530)",
        0x0420 => "CLIENT_LEAVE_FIELD_RECORD (str timestamp, u32 charId, str name, ...; the leaving burst)",
        0x0421 => "CLIENT_LEAVE_FIELD_RECORD_2 (same head as 0x0420, ~1.3 KB; the leaving burst)",
        0x0422 => "CLIENT_LEAVE_FIELD_REASON (u32 reason, str) - 'I am leaving the field, reason N' [L]",
        0x0423 => "CLIENT_LEAVE_FIELD_RECORD_3 (same head as 0x0420; the leaving burst)",
        0x0425 => "CLIENT_RESOURCE_CENSUS (u32 count, then {u32, str path, u32 x3} records; Sound/Map paths)",
        0x0426 => "CLIENT_LEAVE_FIELD_TAIL (20 bytes: i32 -1, i32 -1, u32, ...; closes the leaving burst)",
        // Undecoded but not unknown: seen repeatedly, never froze anything, no reply builder
        // found. Named so a run's UNKNOWN list is only the genuinely new; still logged in
        // full (never_truncate) because their bytes are the only evidence there is.
        0x01C1 => "CLIENT_UNDECODED_01C1 (u32 charId, str name; ~7 per day; builder FUN_142927ac0)",
        0x01B9 => "CLIENT_UNDECODED_01B9 (empty; ~27 per day; builder FUN_142db77a0)",
        0x0226 => "CLIENT_UNDECODED_0226 (10 bytes; rare; builder FUN_1428f4eb0)",

        // Outbound, so that a run's log does not read as if the server were guessing.
        0x007C => "STAT_CHANGED (u8 excl, u8 quiet, u8 1, u32 mask, fields in bit order)",
        0x01A0 => "SET_FIELD",
        0x0478 => "REACTOR_CHANGE_STATE (outbound; u32 objectId, u8 state, pos, u16 delay, u8 eventIdx, u32 stateLength, u32 owner)",
        0x0484 => "REACTOR_ENTER_FIELD (outbound; u8 0, u32 objectId, u32 templateId, u8 state, pos, u8 flip, str name)",
        0x0485 => "REACTOR_LEAVE_FIELD (outbound; u32 objectId)",
        0x05F1 => "FUNC_KEY_MAPPED_INIT (outbound; four gated 89-slot preset tables, then a quickslot flag - net::keymap)",
        0x05F2 => "KEYMAP_OPT_A (outbound; one u32 - net::keymap)",
        0x05F3 => "KEYMAP_OPT_B (outbound; one u32 - net::keymap)",
        0x01BC => "FIELD_CLOCK (u8 type; type 1 = u8 hour, u8 minute, u8 second, 24-hour)",
        0x03D1 => "MOB_LEAVE_FIELD (u32 objectId, u8 deathType, u8, [u32, u32])",
        0x03D2 => "MOB_CHANGE_CONTROLLER (u8 level - 0 DESPAWNS - then the mob body)",
        0x03F0 => "MOB_HP_CHANGE (u32 objectId, u32 hp, u8 showBar) - moves the health bar",
        0x044F => "NPC_ENTER_FIELD",
        0x055B => "SCRIPT_MESSAGE",

        // 0x00BC is deliberately NOT named. Three u32 of 1033 look like an en-US LCID
        // triple, but that is a guess about a value rather than a reading of code, and the
        // builder could not be found. The negative is trustworthy for a reason worth
        // keeping: the same search cannot find 0x0078 either, and 0x0078 is confirmed,
        // answered and working - so the method cannot see whatever emits these. Leaving it
        // unnamed makes it log in full, which is what an undecoded packet deserves.

        _ => return None,
    })
}

/// Packets that must be logged **whole** even though they have a name.
///
/// Having a name is normally the licence to truncate, because a named packet is one we
/// already understand. The client's own error log is the exception that proves the rule:
/// it is named, it is 500-2600 bytes, and every byte of it is evidence. Naming it without
/// this exemption would have quietly re-broken the instrument that found
/// `INVALID_CLIENT_VERSION` - the truncation is what hid it for weeks in the first place.
pub fn never_truncate(opcode: u16) -> bool {
    // 0x0151's trailer is shape-dependent - tag plus remaining length is all there is to go
    // on, because the client sends no marker for it - so its bytes are still evidence even
    // though its head is settled. 0x01A0 is the character record, which has no length prefix
    // and no resync point; if a run ever desynchronises, the whole packet is the only thing
    // that will say where.
    // 0x02FF is named but only its first six bytes are read. The rest is two count-prefixed
    // u16 lists, a block gated on a mob field, and the movement path itself - all of it
    // unread, and all of it the evidence that would settle how this client encodes a path.
    // Truncating it would hide exactly the bytes the next question needs.
    // 0x00DF/0x00E0/0x00E1 are named but their target list is the whole question. Every
    // attack this project has captured came from a session with no mobs in it, so a
    // zero target count has never been a measurement of anything; the next run's bodies
    // are what settle whether the client targets our mobs, and a truncated body cannot.
    // 0x00D9 got a name on 2026-08-20 and has to be exempted in the same breath. Its bodies
    // run just past the 96-byte cap, and what falls off the end is the key-state trailer -
    // the exact part `tools/encodes.py` was found to be blind to, because it is written by a
    // loop rather than by a named primitive. Naming it without this would have re-hidden the
    // bytes that caught that bug.
    // The three CLIENT_UNDECODED_* reports and 0x02F4: named so they stop reading as new,
    // kept whole because nothing about them is settled.
    matches!(
        opcode,
        0x008F | 0x0090 | 0x0091 | 0x00D9 | 0x00DF | 0x00E0 | 0x00E1 | 0x0151 | 0x01A0 | 0x02FF
            | 0x01C1 | 0x01B9 | 0x0226 | 0x02F4
    )
}

/// The client's one-way reports: packets it sends without waiting for anything back.
///
/// Answering nothing is the **correct** handling of every opcode here - `0x013D` in
/// particular must not be answered (`research/buffs.md` §2). The list exists so that the
/// servers' "is not answered" log line can say so, and so that a reviewer reading a run does
/// not mistake a report for a request the server dropped. Membership is a measurement:
/// every one of these has arrived unanswered in the archive, many hundreds of times, with
/// the client playing on - and none is in `dropmoney::LATCHING_REQUESTS`.
///
/// Not here, deliberately: anything the dispatcher answers, and anything that latches.
pub fn is_client_report(opcode: u16) -> bool {
    matches!(
        opcode,
        // Login connection: environment, timings, status codes, the client's own error log,
        // the session identity it announces, and the undecoded 0x00BC.
        0x0070 | 0x0071 | 0x0073 | 0x0079 | 0x007A | 0x008F | 0x0090 | 0x0091 | 0x00A6
            | 0x00BC | 0x00BF | 0x00C0
        // Channel connection.
            | 0x013D | 0x02F4 | 0x01ED | 0x01A5 | 0x02DE | 0x0184 | 0x0194 | 0x00B8
            | 0x00DC | 0x0238 | 0x024D
            | 0x0420 | 0x0421 | 0x0422 | 0x0423 | 0x0424 | 0x0425 | 0x0426
            | 0x01C1 | 0x01B9 | 0x0226
    )
}

/// One packet as a log line body: hex, truncated only when we both know the opcode and do
/// not need its bytes.
pub fn body_hex(opcode: u16, body: &[u8]) -> String {
    let cap = if opcode_name(opcode).is_some() && !never_truncate(opcode) {
        KNOWN_BODY_BYTES
    } else {
        usize::MAX
    };
    let shown = body.len().min(cap);
    let mut out = String::with_capacity(shown * 2 + 24);
    for b in &body[..shown] {
        out.push_str(&format!("{b:02x}"));
    }
    if body.len() > shown {
        out.push_str(&format!("...(+{} bytes, known opcode so capped)", body.len() - shown));
    }
    out
}

/// The `0x00XX NAME` prefix for a log line. Unknown opcodes are marked so that a
/// `grep UNKNOWN` over a run finds every packet nobody has decoded.
pub fn label(opcode: u16) -> String {
    match opcode_name(opcode) {
        Some(name) => format!("0x{opcode:04X} {name}"),
        None => format!("0x{opcode:04X} UNKNOWN"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every constant the world dispatcher matches, resolved to its value. On 2026-09-12
    /// seventeen of these had arms and modules and no row here, so they logged as UNKNOWN
    /// for weeks. The values are spelled here rather than imported so that renaming a
    /// constant cannot silently drop one from the check.
    #[test]
    fn every_opcode_the_world_dispatcher_matches_has_a_name() {
        for op in [
            0x00BB, 0x00D2, 0x00D5, 0x00D9, 0x00DA, 0x00DB, 0x00E5, 0x00E7, 0x00F2, 0x00F3,
            0x00F5, 0x00F6, 0x0104, 0x0107, 0x010E, 0x0111, 0x0114, 0x0116, 0x0125, 0x0138,
            0x0139, 0x013B, 0x013C, 0x013F, 0x0143, 0x014A, 0x0151, 0x0165, 0x017E, 0x0182,
            0x0183, 0x0199, 0x01A0, 0x01BE, 0x01E7, 0x02FF, 0x032F, 0x03E0, 0x03E1, 0x0453,
        ] {
            assert!(opcode_name(op).is_some(), "0x{op:04X} is dispatched but has no name");
        }
    }

    /// A report is named (so it stops reading as new) - except 0x00BC, whose namelessness
    /// is a deliberate decision recorded above - and a report never latches, because a
    /// latching request that is answered with nothing freezes the client.
    #[test]
    fn every_client_report_is_named_and_none_of_them_latches() {
        let reports: Vec<u16> = (0u16..=0x5FF).filter(|&op| is_client_report(op)).collect();
        assert!(reports.len() > 20, "{reports:04X?}");
        for op in &reports {
            if *op == 0x00BC || *op == 0x0424 {
                continue; // 0x00BC: deliberately unnamed. 0x0424: reserved in the burst, never seen.
            }
            assert!(opcode_name(*op).is_some(), "report 0x{op:04X} has no name");
            assert!(
                !crate::dropmoney::latches_the_exclusive_request(*op),
                "0x{op:04X} latches, so it is a request, not a report"
            );
        }
        // Things that are answered are not reports.
        for op in [0x00E5u16, 0x0107, 0x0199, 0x02FF, 0x00D9] {
            assert!(!is_client_report(op), "0x{op:04X} is answered by the world server");
        }
    }

    #[test]
    fn every_opcode_the_servers_build_has_a_name() {
        for op in [
            ACCOUNT_INFO,
            WORLD_LIST,
            LOGIN_RESULT,
            MIGRATE_COMMAND,
            CHECK_NAME_RESULT,
            CREATE_CHARACTER_RESULT,
            DELETE_CHARACTER_RESULT,
            DATA_WZ_PATCH,
            ENTER_CREATION_RESULT,
        ] {
            assert!(opcode_name(op).is_some(), "0x{op:04X} has no name");
        }
    }

    #[test]
    fn every_opcode_a_server_dispatches_on_has_a_name() {
        for op in [
            CLIENT_LOGIN_REQUEST,
            CLIENT_LEAVE_WORLD_REQUEST,
            CLIENT_ENTER_CREATION_REQUEST,
            CLIENT_CHECK_NAME_REQUEST,
            CLIENT_CREATE_CHARACTER_REQUEST,
            CLIENT_DELETE_CHARACTER_REQUEST,
            CLIENT_SELECT_CHARACTER_REQUEST,
            CLIENT_DATA_WZ_REQUEST,
        ] {
            assert!(opcode_name(op).is_some(), "0x{op:04X} has no name");
        }
    }

    /// The rule that makes a surprising run explainable: whatever we cannot name, we keep
    /// in full.
    #[test]
    fn an_unknown_body_is_never_truncated() {
        let body = vec![0xABu8; 4096];
        let hex = body_hex(0xFFFE, &body);
        assert_eq!(hex.len(), 8192, "an unknown body must be logged whole");
        assert!(!hex.contains("capped"));
    }

    /// The regression this nearly shipped: naming the ELog opcodes made them truncatable,
    /// which would have silently disabled the cheapest instrument in the project.
    #[test]
    fn the_client_error_log_is_logged_whole_even_though_it_is_named() {
        let body = vec![0xABu8; 2600];
        for op in [0x008Fu16, 0x0090, 0x0091] {
            assert!(opcode_name(op).is_some(), "0x{op:04X} should be named");
            let hex = body_hex(op, &body);
            assert_eq!(hex.len(), 5200, "0x{op:04X} was truncated");
            assert!(!hex.contains("capped"));
        }
    }

    #[test]
    fn a_known_body_is_capped_so_it_does_not_bury_its_neighbours() {
        let body = vec![0xABu8; 4096];
        let hex = body_hex(LOGIN_RESULT, &body);
        assert!(hex.contains("capped"), "{hex}");
        assert!(hex.len() < 300);
    }

    /// Nothing may assert a `guess:` name any more - the eight that did were checked
    /// against the client's own builders and five of them were wrong. If a future opcode
    /// genuinely cannot be established, leave it unnamed so it logs in full.
    #[test]
    fn no_name_is_still_an_unverified_guess() {
        for op in 0u16..=0xFFFF {
            if let Some(name) = opcode_name(op) {
                assert!(!name.starts_with("guess:"), "0x{op:04X} still asserts a guess: {name:?}");
            }
        }
    }

    /// 0x00BC has no located builder, so it must stay unnamed and therefore untruncated.
    #[test]
    fn the_undecoded_locale_packet_stays_unknown_and_whole() {
        assert_eq!(opcode_name(0x00BC), None);
        assert!(label(0x00BC).contains("UNKNOWN"));
        assert_eq!(body_hex(0x00BC, &vec![0xABu8; 500]).len(), 1000);
    }

    #[test]
    fn unknown_opcodes_are_greppable() {
        assert!(label(0xFFFE).contains("UNKNOWN"));
        assert!(!label(LOGIN_RESULT).contains("UNKNOWN"));
        assert!(label(LOGIN_RESULT).contains("LOGIN_RESULT"));
    }

    #[test]
    fn an_empty_body_does_not_panic() {
        assert_eq!(body_hex(0x1234, &[]), "");
    }
}
