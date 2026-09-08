//! Refusing a meso drop **and still answering it**.
//!
//! The owner, 2026-09-08: *"I have attempted to drop 10 mesos and 5000 mesos, none of these
//! attempts worked, but I lose all functionality in being able to interact with my
//! inventory."*
//!
//! # The policy is unchanged; only the silence is fixed
//!
//! Players still cannot drop mesos. Nothing in this module spends, moves or creates a meso,
//! and nothing in it puts an object on the floor. What it adds is the **reply** - the thing
//! whose absence turned a refused drop into a dead inventory for the rest of the session.
//!
//! # What was measured
//!
//! `net::dropmoney` carries the listing: `0x0143`'s builder `FUN_142d4cb40` consults the
//! exclusive-request gate `142cc42d0` on the way in and sets `[player+0x2330] = 1` through
//! `142cc4430` on the way out, and the gate refuses every later latching request while that
//! field is non-zero.
//!
//! On the server side it was case **(a) - not handled at all**. `world.log`, 2026-09-08:
//!
//! ```text
//! 05:30:07.798 <- 0x0143 UNKNOWN, 8 byte body 75571b100a000000
//! 05:30:07.798    0x0143 UNKNOWN is not answered yet, and it is UNKNOWN ...
//! ```
//!
//! `0x0143` reached `Session::dispatch`'s `_ => return Vec::new()` and nothing went back.
//! The same session then ran for another eight minutes - mob moves, user moves, mob-control
//! acks all flowing - and contains **not one** further latching request of any kind. The
//! 2026-09-05 session shows the other half: `0x0107` at 02:51:36 and `0x032C` at 02:51:40
//! both *before* the `0x0143` at 02:52:05, and none after.
//!
//! # Item drops do NOT share this
//!
//! An item drop is `0x0107` with `dst == 0`, which routes into `session::ground` where every
//! path - including every refusal - returns a `0x0070` carrying `bExclRequestSent = 1`. That
//! was made true on 2026-08-19 after the same failure. Mesos are a different opcode and were
//! simply never wired.

use crate::session::Reply;

/// The line the player sees. Written down here so the test and the packet cannot drift.
pub const REFUSAL: &str = "Mesos cannot be dropped on this server.";

/// Answer a `0x0143` without dropping anything.
///
/// # Order
///
/// The `0x007C` unlock goes **first** and the chat line second, for the same reason
/// `session::ground` sends the `0x0070` before the `0x046E`: the packet that releases the
/// client's UI should not be queued behind one that does not.
///
/// # `mesos_now`
///
/// Read for the log line only, and it is read *after* the decision not to spend, so a run's
/// `world.log` records the balance this handler left alone. `store::Store::mesos` is the
/// only store call in this module and there is no writing counterpart to it here.
///
/// # A malformed body is answered too
///
/// The client latched before the server had any opinion about the bytes, so `None` from the
/// parser changes the log line and nothing else. Returning early on a parse failure is the
/// exact shape of the bug being fixed.
pub fn on_drop_money(store: &store::Store, character_id: Option<u32>, payload: &[u8]) -> Vec<Reply> {
    let asked = net::dropmoney::parse_drop_money(payload);
    let mesos_now = character_id.and_then(|id| store.mesos(id).ok());

    let asked_for = match asked {
        Some(m) => format!("{} mesos", m.amount),
        None => format!(
            "an unreadable {} byte body (expected {})",
            payload.len(),
            net::dropmoney::DROP_MONEY_BODY_LEN
        ),
    };
    let balance = match mesos_now {
        Some(v) => format!("{v}"),
        None => "unknown - no character is claimed on this connection".to_string(),
    };

    let mut out = vec![Reply {
        opcode: net::combat::STAT_CHANGED,
        body: net::dropmoney::exclusive_request_unlock(),
        what: format!(
            "StatChanged: REFUSING the meso drop of {asked_for} - mesos cannot be dropped on \
             this server. Empty mask, nothing changed, balance still {balance}. \
             bExclRequestSent = 1 is the whole point: it is the FIRST thing 142d54780 acts on \
             (142d547bb calls 142cc4430 with 0), and without it the client's +0x2330 latch \
             stays set and every later inventory action, AP click, cash-shop click and item \
             drop is dropped before it is built."
        ),
    }];
    out.push(Reply {
        opcode: net::notice::CHAT_NOTICE,
        body: net::notice::chat_notice(REFUSAL),
        what: format!("ChatNotice: {REFUSAL}"),
    });
    out
}

/// The general form: answer **any** latching request this server does not implement.
///
/// # Why a whitelist rather than answering everything
///
/// `Session::dispatch`'s `_ => Vec::new()` is deliberate, and its doc block says why: a
/// wrong reply moves the client into a state nobody has read, which is worse than silence.
/// Blanket-answering every unknown opcode would send a stat change in reply to 750 archived
/// `0x0070` environment reports and 23 261 mob moves.
///
/// [`net::dropmoney::LATCHING_REQUESTS`] is narrower and measured: the opcodes whose own
/// builder sets `+0x2330`. For those, silence is not the safe default - it is the failure -
/// and the packet sent back is the one whose first action is the unlock and whose mask is
/// empty, so it says nothing about any subsystem.
///
/// This arm belongs **after** every specific handler, so it can only ever fire on an opcode
/// nothing else answers.
///
/// # What it would have caught
///
/// Deduplicating inbound packets by `(timestamp, opcode, body)` over every archived
/// `world*.log` in `previous-runs/` and `research/fixtures/`, exactly three latching opcodes
/// have ever arrived unhandled: **[L]**
///
/// ```text
///   0x0143   2 events   the meso drop
///   0x01FD   1 event    body 00000000 01 0600 "Cobalt" - a whisper or /find
///   0x02F6   1 event    body 01000000 - not identified
/// ```
///
/// All three froze the UI. Only the first has ever been reported, because the other two
/// happened at the end of their sessions.
pub fn unlock_unhandled_latching_request(opcode: u16) -> Vec<Reply> {
    vec![Reply {
        opcode: net::combat::STAT_CHANGED,
        body: net::dropmoney::exclusive_request_unlock(),
        what: format!(
            "StatChanged: UNLOCK ONLY. {opcode:#06X} is not handled, but its client-side \
             builder sets the +0x2330 exclusive-request latch, so leaving it unanswered \
             would silently refuse every later inventory action, AP click and cash-shop \
             click for the rest of the session. Empty mask - this changes nothing and says \
             nothing about any subsystem; it only clears the latch at 142d547bb."
        ),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use store::Store;

    /// A character with a balance, so "unchanged" is a claim that can come back false.
    fn character_with(mesos: u32) -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wanderer".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.set_mesos(id, mesos).unwrap();
        assert_eq!(store.mesos(id).unwrap(), mesos, "the fixture itself must hold");
        (store, id)
    }

    /// **All three effects, not one.**
    ///
    /// `CLAUDE.md`: the quest turn-in test counted fanfares while the experience doubled
    /// beside it, because the fanfare was the one effect that was already gated correctly.
    /// A meso-drop refusal has three things to say and this asserts all three - a reply went
    /// out, the balance did not move, and nothing was put on the floor.
    #[test]
    fn a_meso_drop_is_answered_costs_nothing_and_creates_no_drop() {
        let (store, id) = character_with(5_000);
        let body = net::dropmoney::drop_money_request(0x101b_5775, 10);

        let out = on_drop_money(&store, Some(id), &body);

        // 1. A REPLY WAS SENT. Not "no error was returned" - a packet exists, it is the
        //    stat change, and its first byte is the one that clears +0x2330.
        assert!(!out.is_empty(), "an unanswered 0x0143 IS the bug");
        let unlock = out
            .iter()
            .find(|r| r.opcode == net::combat::STAT_CHANGED)
            .expect("a 0x007C must go back");
        assert_eq!(unlock.body[0], 1, "bExclRequestSent - the byte the whole fix is about");
        assert_eq!(
            u32::from_le_bytes([unlock.body[3], unlock.body[4], unlock.body[5], unlock.body[6]]),
            0,
            "an empty mask: the refusal announces no stat and draws no meso effect"
        );

        // 2. THE BALANCE IS UNCHANGED. The handler was handed the store and did not spend.
        assert_eq!(store.mesos(id).unwrap(), 5_000, "a refused drop costs nothing");

        // 3. NO DROP OBJECT. A drop is observable as a DROP_ENTER_FIELD; there is none, and
        //    no inventory operation either.
        assert!(
            !out.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD),
            "nothing may reach the floor"
        );
        assert!(
            !out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION),
            "a meso drop touches no bag slot, so it must not answer with a bag packet"
        );

        // 4. And the player is told why, rather than watching nothing happen.
        assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE));
    }

    /// 5000 was the owner's second attempt. It never left their client - the latch had already
    /// eaten it - so this is the run that must work once the latch is being cleared.
    #[test]
    fn the_five_thousand_meso_attempt_is_answered_the_same_way() {
        let (store, id) = character_with(5_000);
        let out = on_drop_money(&store, Some(id), &net::dropmoney::drop_money_request(1, 5_000));
        assert_eq!(out[0].opcode, net::combat::STAT_CHANGED);
        assert_eq!(out[0].body[0], 1);
        assert_eq!(store.mesos(id).unwrap(), 5_000);
    }

    /// The unlock goes first. The packet that frees the UI is not queued behind chat.
    #[test]
    fn the_unlock_is_the_first_reply() {
        let (store, id) = character_with(1);
        let out = on_drop_money(&store, Some(id), &net::dropmoney::drop_money_request(1, 1));
        assert_eq!(out[0].opcode, net::combat::STAT_CHANGED, "the unlock leads");
    }

    /// **A malformed body is still answered.** Returning early on a parse failure is the
    /// exact shape of the bug: the client latched before the bytes were ever inspected.
    #[test]
    fn an_unreadable_body_is_answered_rather_than_dropped() {
        let (store, id) = character_with(100);
        for junk in [vec![], vec![0u8; 3], vec![0u8; 40]] {
            let out = on_drop_money(&store, Some(id), &junk);
            assert_eq!(out[0].opcode, net::combat::STAT_CHANGED, "len {}", junk.len());
            assert_eq!(out[0].body[0], 1);
        }
        assert_eq!(store.mesos(id).unwrap(), 100);
    }

    /// No claimed character is not a reason to go quiet either - the client latched anyway.
    #[test]
    fn an_unclaimed_connection_is_answered_too() {
        let (store, _id) = character_with(100);
        let out = on_drop_money(&store, None, &net::dropmoney::drop_money_request(1, 10));
        assert_eq!(out[0].opcode, net::combat::STAT_CHANGED);
        assert_eq!(out[0].body[0], 1);
    }

    /// A negative amount is a refusal like any other, and must not be read as four billion.
    #[test]
    fn a_negative_amount_changes_nothing_and_is_reported_as_negative() {
        let (store, id) = character_with(7);
        let out = on_drop_money(&store, Some(id), &net::dropmoney::drop_money_request(1, -1));
        assert_eq!(store.mesos(id).unwrap(), 7);
        assert!(out[0].what.contains("-1 mesos"), "{}", out[0].what);
    }

    /// The general arm answers, and answers with nothing but the unlock.
    #[test]
    fn the_general_unlock_says_nothing_except_that_the_latch_is_clear() {
        let out = unlock_unhandled_latching_request(0x01FD);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].opcode, net::combat::STAT_CHANGED);
        assert_eq!(out[0].body[0], 1);
        assert_eq!(out[0].body.len(), 9);
        assert!(out[0].what.contains("0x01FD"), "{}", out[0].what);
    }

    /// The two other latching opcodes seen unhandled in the archive, and the control that
    /// says this predicate does not simply return true.
    #[test]
    fn the_predicate_covers_the_archive_and_still_discriminates() {
        for op in [0x0143u16, 0x01FD, 0x02F6] {
            assert!(net::dropmoney::latches_the_exclusive_request(op), "{op:#06X}");
        }
        for op in [0x00D9u16, 0x02FF, 0x00E7, 0x0070] {
            assert!(!net::dropmoney::latches_the_exclusive_request(op), "{op:#06X}");
        }
    }
}
