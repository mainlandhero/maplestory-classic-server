//! The client's one-way reports - answered with nothing, on purpose, and said so.
//!
//! The owner, 2026-09-12: *"Please handle all of the opcodes."* Enumerating the archive
//! (`previous-runs/world*.log`, deduplicated events) gave the inbound opcodes that still
//! logged as `UNKNOWN ... is not answered yet`: `0x013D` (send-counter census, every 30 s),
//! `0x02F4`, `0x01ED` (the client's log channel), `0x01A5` (skill checksums), `0x02DE`,
//! `0x0184`, `0x0194` (once per field entry each), `0x00B8`, the `0x0420..0x0426` leaving
//! burst, `0x0425` (a resource census), and three undecoded ones (`0x01C1`, `0x01B9`,
//! `0x0226`). Every one is a report the client sends without waiting: they arrived
//! unanswered hundreds of times while the client played on, and none latches.
//!
//! So "handling" them is three things, none of which is a reply:
//!
//! * a **name** in `net::names`, so a run's `UNKNOWN` list means what it should;
//! * membership in `net::names::is_client_report`, which turns the server's
//!   `is not answered yet` line into `a client report; nothing is expected back`;
//! * an explicit dispatcher arm that lands here, so the disposition is a decision in code
//!   rather than the fall-through - and so the two reports that carry something worth a
//!   line (`0x0422`'s reason for leaving, `0x0425`'s count) get one.
//!
//! **`0x013D` must not be answered** (`research/buffs.md` §2). Nothing here ever returns a
//! packet, and `tests.rs` pins that for every report opcode.

use super::{Reply, Session};

impl Session {
    /// Log what a report says, when it says anything, and answer nothing.
    pub(super) fn on_client_report(&mut self, opcode: u16, body: &[u8]) -> Vec<Reply> {
        match opcode {
            0x0422 => {
                // u32 reason, then a u16-length string. Reason 4 is the Cash Shop click and
                // reason 2 the channel-change path (research/cash-shop-migrate.md); the
                // others are unmapped and worth seeing in the log as they come.
                let reason = body.get(..4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
                let text = body
                    .get(4..6)
                    .map(|b| u16::from_le_bytes([b[0], b[1]]) as usize)
                    .and_then(|n| body.get(6..6 + n))
                    .map(|s| String::from_utf8_lossy(s).into_owned());
                crate::server::log(&format!(
                    "   leaving the field: reason {} {:?} - the client announcing a field exit, \
                     not a request; the 0x0420/0x0421/0x0423/0x0426 records beside it are the same event",
                    reason.map_or("?".to_string(), |r| r.to_string()),
                    text.unwrap_or_default()
                ));
            }
            0x0425 => {
                let count = body.get(..4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
                crate::server::log(&format!(
                    "   resource census: {} record(s) in {} bytes (Sound/, Map/ paths); a report, nothing expected back",
                    count.map_or("?".to_string(), |c| c.to_string()),
                    body.len()
                ));
            }
            _ => {}
        }
        Vec::new()
    }
}
