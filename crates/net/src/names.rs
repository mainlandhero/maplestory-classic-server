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

        // Seen on the wire and identified, but nothing acts on them. Naming them keeps
        // them out of the UNKNOWN bucket, which is reserved for genuinely unread packets.
        0x0073 => "CLIENT_SESSION_IDENTITY (never answered)",
        0x007D => "CLIENT_MIGRATION_HELLO (carries the migration seed)",
        0x0079 => "CLIENT_CHARACTER_REPORT (never answered)",
        0x007A => "CLIENT_DISCONNECT_NOTICE (never answered)",
        0x00BC => "CLIENT_LOCALE (1033 three times, never answered)",
        0x00C0 => "CLIENT_HELLO (never answered)",
        0x0070 => "CLIENT_VERSION_ECHO (never answered)",
        0x0071 => "CLIENT_VERSION_DETAIL (never answered)",
        0x00A6 => "CLIENT_ASSET_TICK (never answered)",
        0x00BF => "CLIENT_READY (never answered)",

        _ => return None,
    })
}

/// One packet as a log line body: hex, truncated only if the opcode is known.
///
/// Returns the hex and whether anything was left out, so the caller can say so.
pub fn body_hex(opcode: u16, body: &[u8]) -> String {
    let cap = if opcode_name(opcode).is_some() { KNOWN_BODY_BYTES } else { usize::MAX };
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

    #[test]
    fn a_known_body_is_capped_so_it_does_not_bury_its_neighbours() {
        let body = vec![0xABu8; 4096];
        let hex = body_hex(LOGIN_RESULT, &body);
        assert!(hex.contains("capped"), "{hex}");
        assert!(hex.len() < 300);
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
