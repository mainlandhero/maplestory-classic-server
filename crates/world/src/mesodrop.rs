//! Refusing a meso drop **and still answering it** - and, since 2026-09-09, the refusals only.
//!
//! The owner, 2026-09-08: *"I have attempted to drop 10 mesos and 5000 mesos, none of these
//! attempts worked, but I lose all functionality in being able to interact with my
//! inventory."*
//!
//! # Mesos CAN be dropped now, and this module is no longer where that is decided
//!
//! The owner, 2026-09-09: *"I still cannot drop mesos."* They were right, and the reason was this
//! module: it decoded `0x0143` and then refused every one of them. The 09-08 work fixed the
//! **freeze** - the thing whose absence turned a refused drop into a dead inventory for the
//! rest of the session - and it was never a step towards making the drop happen. A patch note
//! of mine implied otherwise, which is how the gap surfaced.
//!
//! `Session::on_drop_money` in `session/ground.rs` now does the drop, because it needs three
//! things this module deliberately does not have: the field to put the object in, the
//! player's position, and the foothold table to rest it on. What is left here is the refusal
//! half - [`refuse`] - and the general unlock for latching opcodes nothing implements.
//!
//! Everything below about the latch is unchanged and is why [`refuse`] exists at all: a
//! refusal that answers with nothing is worse than a refusal.
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

/// The lines the player sees. Written down here so the tests and the packets cannot drift.
///
/// **There is no longer a blanket "mesos cannot be dropped".** They can, since 2026-09-09 -
/// `Session::on_drop_money` in `session/ground.rs` does it. What remains here is the refusal
/// half, and each refusal now says *which* one it is, because "nothing happened" with no reason
/// is the symptom this project has spent runs chasing.
pub const NOT_ENOUGH: &str = "You do not have that many mesos.";

/// The signed-amount refusal. See `Session::on_drop_money`: `0x0143` carries an `i32` and the
/// client's own check lets a negative through, so this is a guard against crediting the player,
/// not a tidiness check.
pub const NOT_A_POSITIVE_AMOUNT: &str = "That is not an amount you can drop.";

/// The same guard the item drop uses: a drop placed where the server does not know the player
/// is standing is drawn outside the client's pick-up box and cannot be collected.
pub const WALK_FIRST: &str = "Walk a step first, then try again.";

/// Refuse a meso drop, **and still answer it**.
///
/// # The unlock is not optional and is not a detail
///
/// `0x0143`'s builder sets `player+0x2330` the moment it sends, and the gate refuses every
/// later latching request while that is non-zero. So a refusal that returns nothing does not
/// fail one drop - it kills the inventory, the ability-point buttons, the cash shop and the
/// item drop for the rest of the session. The owner, 2026-09-08: *"none of these attempts worked,
/// but I lose all functionality in being able to interact with my inventory."*
///
/// The `StatChanged` goes **first** and the chat line second, for the same reason
/// `session::ground` sends the `0x0070` before the `0x046E`: the packet that releases the
/// client's UI should not be queued behind one that does not.
///
/// `player_line` is `None` for causes the player can do nothing about - a malformed body, a
/// store error - which are logged rather than narrated. A refusal they *can* act on always
/// carries one, because a silent refusal and a frozen inventory look identical on screen.
pub fn refuse(reason: &str, player_line: Option<&str>) -> Vec<Reply> {
    let mut out = vec![Reply {
        opcode: net::combat::STAT_CHANGED,
        body: net::dropmoney::exclusive_request_unlock(),
        what: format!(
            "StatChanged: REFUSING the meso drop - {reason}. Empty mask, nothing changed. \
             bExclRequestSent = 1 is the whole point: it is the FIRST thing 142d54780 acts on \
             (142d547bb calls 142cc4430 with 0), and without it the client's +0x2330 latch \
             stays set and every later inventory action, AP click, cash-shop click and item \
             drop is dropped before it is built."
        ),
    }];
    if let Some(line) = player_line {
        out.push(Reply {
            opcode: net::notice::CHAT_NOTICE,
            body: net::notice::chat_notice(line),
            what: format!("ChatNotice: {line}"),
        });
    }
    crate::server::log(&format!("   mesos: drop REFUSED - {reason}"));
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

    // The balance-moving tests live in `session::tests` now, with the handler that moves it:
    // `a_meso_drop_leaves_the_character_and_reaches_the_floor` and its three siblings. This
    // module no longer touches a store, so it no longer builds a character to check one.

    /// **Every refusal answers, and the unlock leads.**
    ///
    /// `CLAUDE.md`: the quest turn-in test counted fanfares while the experience doubled
    /// beside it. So this asserts what the packet IS, not merely that one came back - the
    /// first reply is the stat change, its `bExclRequestSent` byte is set, and its mask is
    /// empty so the refusal announces no stat and draws no meso effect.
    #[test]
    fn a_refusal_answers_with_the_unlock_first_and_an_empty_mask() {
        let out = refuse("asked for 9 with 7 in hand", Some(NOT_ENOUGH));
        assert_eq!(out[0].opcode, net::combat::STAT_CHANGED, "the unlock leads");
        assert_eq!(out[0].body[0], 1, "bExclRequestSent - the byte the whole fix is about");
        assert_eq!(
            u32::from_le_bytes([out[0].body[3], out[0].body[4], out[0].body[5], out[0].body[6]]),
            0,
            "an empty mask: the refusal announces no stat and draws no meso effect"
        );
        assert_eq!(out[1].opcode, net::notice::CHAT_NOTICE, "and the reason, second");
    }

    /// A refusal must never touch the floor or the bag.
    ///
    /// Answering a meso request with an inventory packet is the "wrong reply" half of
    /// `CLAUDE.md`'s rule: it would clear the latch and leave the client's bag running an
    /// entry loop about a slot that never changed.
    #[test]
    fn a_refusal_creates_no_drop_and_no_bag_packet() {
        let out = refuse("whatever the cause", Some(WALK_FIRST));
        assert!(!out.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD));
        assert!(!out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION));
    }

    /// A cause the player can do nothing about is logged, not narrated - but it is still
    /// answered, because the client latched before anyone had an opinion about the bytes.
    #[test]
    fn a_causeless_refusal_still_unlocks_and_says_nothing_on_screen() {
        let out = refuse("an unreadable 3 byte body (expected 8)", None);
        assert_eq!(out.len(), 1, "no chat line");
        assert_eq!(out[0].opcode, net::combat::STAT_CHANGED);
        assert_eq!(out[0].body[0], 1);
    }

    /// The three player-facing lines are distinct, so a run's screenshot says which refusal
    /// fired. Three identical strings would pass every other test in this file.
    #[test]
    fn the_three_refusal_lines_are_distinct() {
        let all = [NOT_ENOUGH, NOT_A_POSITIVE_AMOUNT, WALK_FIRST];
        for (i, a) in all.iter().enumerate() {
            assert!(!a.is_empty());
            for b in &all[i + 1..] {
                assert_ne!(a, b, "two refusals that read the same cannot be told apart");
            }
        }
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

    /// **The two opcodes that store the latch INLINE**, added 2026-09-09 after both were found
    /// to be live freezes in shipped code.
    ///
    /// `0x0111` is the summoning sack and `0x0125` is dragging a scroll onto an equip. Neither
    /// calls `0x142cc4430`, which is what the scan that built [`net::dropmoney::LATCHING_REQUESTS`]
    /// looked for; both write `[reg+0x2330]` directly and gate on it first. Until this, using
    /// either one killed the inventory, the AP buttons and the cash shop for the rest of the
    /// session - the same failure the owner reported for the meso drop on 2026-09-08, sitting
    /// unfixed in two more places.
    ///
    /// This test exists so a future prune of that list cannot quietly drop them again: they
    /// look like ordinary entries, and the evidence for them is a disassembly note rather than
    /// a capture.
    #[test]
    fn the_two_inline_latching_opcodes_are_covered() {
        for op in [0x0111u16, 0x0125] {
            assert!(
                net::dropmoney::latches_the_exclusive_request(op),
                "{op:#06X} stores +0x2330 inline; dropping it re-freezes the client"
            );
            // And the whole point: the catch-all must produce a real unlock for them.
            let out = unlock_unhandled_latching_request(op);
            assert_eq!(out.len(), 1);
            assert_eq!(out[0].body[0], 1, "bExclRequestSent");
        }
    }
}
