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
        0x00DC => "CLIENT_FIELD_ENTERED (once per SetField, ~420 ms after; empty body)",
        0x00E7 => "CLIENT_CHAT (u32 tick, u16-length text, u8 tab)",
        0x0231 => "USER_CHAT (balloon over the head, and the chat log line)",
        0x0107 => "CLIENT_INVENTORY_MOVE (u32 tick, u8 invType, i16 src, i16 dst, i16 count)",
        0x00F2 => "CLIENT_NPC_CLICK (u32 npcObjectId, i16 charX, i16 charY, u32; the NO-QUEST click path)",
        0x00F3 => "CLIENT_SCRIPT_REPLY (u32 handle, u8 msgType, u32 echo, str the box's own text, i8 action)",
        0x00F6 => "CLIENT_STORAGE (u8 mode: 4 take out, 5 put in, 6 sort, 7 mesos i64, 8 close)",
        0x013C => "CLIENT_SKILL_USE (u32 skillId, u32 level, then a tail)",
        0x013F => "CLIENT_SKILL_CANCEL (u32 skillId, 5 bytes, raw[124] CTS mask; RETRIES every ~180ms)",
        0x00D5 => "CLIENT_CASH_SHOP_REQUEST (u32 tick, u8; an EXCLUSIVE REQUEST - it latches ctx+0x2330 and only fires once until answered)",
        0x007E => "TEMPORARY_STAT_RESET (u8,u8,u8, raw[124] mask, tail)",
        0x0572 => "STORAGE_RESULT (u8 mode: 24 open, 13 put ok, 15 refresh, 10/11/16/17 refusals)",
        0x01BE => "CLIENT_LOG_OUT (empty body; POISONS SetField until 0x0106 answers it)",
        0x00BB => "CHAT_NOTICE",
        0x0106 => "LOG_OUT_RESULT",
        0x0151 => "CLIENT_QUEST_REQUEST (u8 action, u32 questId, u32 npcTemplateId, shape-dependent tail)",
        0x0182 => "CLIENT_PARTY_CREATE",
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

        // Outbound, so that a run's log does not read as if the server were guessing.
        0x007C => "STAT_CHANGED (u8 excl, u8 quiet, u8 1, u32 mask, fields in bit order)",
        0x01A0 => "SET_FIELD",
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
fn never_truncate(opcode: u16) -> bool {
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
    matches!(
        opcode,
        0x008F | 0x0090 | 0x0091 | 0x00D9 | 0x00DF | 0x00E0 | 0x00E1 | 0x0151 | 0x01A0 | 0x02FF
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
