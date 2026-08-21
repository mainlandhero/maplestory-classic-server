//! Giving a quest up - `0x0151` **action 3** - and the two opcodes that are not it.
//!
//! Labels are the project's: **[L]** read off this client's listing or off a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred. Full working, with every
//! instrument control, is `research/quest-forfeit.md`.
//!
//! # The request
//!
//! ```text
//! 0x0151   u8 action = 3, u32 questId          5 bytes, and nothing else
//! ```
//!
//! **[L], measured twice**, in
//! `research/fixtures/quest-forfeit-0151-action3-on-the-wire-world.log` (`03:31:18.936`) and
//! `research/fixtures/quest-forfeit-0151-action3-resent-after-relog-world.log`
//! (`03:45:59.236`), both `03 e9030000` - action 3, quest 1001. Both went unanswered, and the
//! second run's `SetField` proves why the owner had to do it twice: it opens with **2 started / 1
//! completed**, the same book the first run ended with.
//!
//! **[L], statically**, from the client's only builder for it, `FUN_142d9ad50`:
//!
//! ```text
//! 142d9ae19  mov  edx, 0x151
//! 142d9ae23  call 0x1406ed520      ; COutPacket(0x0151)
//! 142d9ae29  mov  dl, 3
//! 142d9ae30  call 0x1406ed840      ; u8  3
//! 142d9ae35  mov  edx, ebx
//! 142d9ae3c  call 0x1406ed9d0      ; u32 questId
//! 142d9ae46  call 0x1415d01c0      ; send
//! ```
//!
//! # `crate::script::parse_quest_request` CANNOT read this body
//!
//! It reads a fixed 9-byte head - `u8 action, u32 questId, u32 npcTemplateId` - and a forfeit
//! carries no NPC at all. On a 5-byte body the second `u32` fails and the whole parse returns
//! `None`, which `Session::on_quest_request` turns into `Vec::new()`: **silence**. That is not
//! a hypothetical; it is what both captures show. Route action 3 here *before* the general
//! parser - [`is_forfeit`] exists for exactly that test.
//!
//! # The client CANNOT clear the journal row by itself - the reply is mandatory
//!
//! **[L], and this is the load-bearing fact.** `FUN_1402e0ec0` is the client's erase-from-the-
//! started-quest-map (`crate::quest` names it from the other direction) and
//! `tools/callers.py` finds it reachable **three ways in total and only two functions**:
//! `FUN_140304b20`, the character-record decoder, and `FUN_142d5b750`, the `0x0089` sub-1
//! handler. Zero tail `jmp`s, zero pointers to it in data. `FUN_142d9ad50` does not reach it,
//! and neither does the local-effect call it makes on the way (`FUN_142ce5f80`, whose 35 call
//! sites contain no `1402e0xxx`).
//!
//! So nothing about pressing "give up" removes the quest on the client. The row goes when -
//! and only when - the server answers with [`forfeit_reply`], which is `0x0089` sub 1 state 0.
//!
//! # What the client refuses to ask for
//!
//! `FUN_142d9ad50` walks the character's own started-quest map (`charData+0x1273`, hashed by
//! `questId % charData+0x127b`) and **returns without building anything** when the id is not
//! there (`142d9adbb`, `142d9adcd`, `142d9adcf` all jump to the epilogue). **[L]**
//!
//! A *completed* quest is not in that map. So **a forfeit can never reset a finished quest** -
//! the packet is not sent, and no server change can make it be. Re-running a chain from the
//! start needs the server to volunteer [`forfeit_reply`] with `forget_completion = true`,
//! which is what a GM command is for. See `research/quest-forfeit.md` §6.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A quest is
//! forfeited because a packet arrived on the socket.

use crate::packet::PacketReader;

// ---------------------------------------------------------------------------------------
// 0x0151 action 3 - the forfeit request
// ---------------------------------------------------------------------------------------

/// Tag 3: **give this quest up**. Built only by `FUN_142d9ad50`. **[L]**
///
/// It is not one of the six tags `crate::script` documents, and that is not an oversight
/// there - it is the shape of the mistake `CLAUDE.md` calls *enumerate before you filter*.
/// The six came from enumerating the six `0x0151` sites inside **one** function,
/// `FUN_141f0e4c0`. `research/msexe-send-opcodes.txt` lists **nine** sites in **four**
/// functions; the other three carry tags 0, 3 and 7.
pub const QUEST_ACTION_RESIGN: u8 = 3;

/// The whole body: one `u8` and one `u32`. **[L]**
pub const FORFEIT_BODY_LEN: usize = 5;

/// A decoded forfeit request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForfeitRequest {
    pub quest_id: u32,
}

/// Cheap test for "this `0x0151` body is a forfeit", for a dispatcher that has to choose a
/// parser before it knows the shape.
///
/// Deliberately tests the **action byte only** and not the length: the length is a
/// consequence, and a caller that keys off `len() == 5` would silently stop working if the
/// client ever appended a field. [`parse_forfeit_request`] does check the length.
pub fn is_forfeit(body: &[u8]) -> bool {
    body.first() == Some(&QUEST_ACTION_RESIGN)
}

/// Decode a `0x0151` body that [`is_forfeit`] accepted.
///
/// Returns `None` for anything that is not exactly `03` plus a `u32`. A caller must not turn
/// that into silence - see the "always answer" rule in `CLAUDE.md`; answer with the quest
/// untouched rather than with nothing.
pub fn parse_forfeit_request(body: &[u8]) -> Option<ForfeitRequest> {
    if !is_forfeit(body) || body.len() != FORFEIT_BODY_LEN {
        return None;
    }
    let mut r = PacketReader::new(body);
    let _action = r.u8().ok()?;
    Some(ForfeitRequest { quest_id: r.u32().ok()? })
}

/// Build the body the **client** sends, so a test or `tools/channel_smoke.py` can drive the
/// server without a launch.
///
/// The server never sends this; it exists because a round trip against a byte-exact capture
/// is the only cheap check that this module and the client agree.
pub fn forfeit_request(quest_id: u32) -> Vec<u8> {
    let mut b = Vec::with_capacity(FORFEIT_BODY_LEN);
    b.push(QUEST_ACTION_RESIGN);
    b.extend_from_slice(&quest_id.to_le_bytes());
    b
}

/// The **body** of the reply that actually clears the journal: `0x0089` sub 1, state 0.
///
/// A thin wrapper over [`crate::quest::quest_forgotten`] rather than a second builder, so
/// there is exactly one place that knows the state byte. It exists for its name and its
/// documentation: the `0x0089` this answers with is not optional decoration, it is the only
/// thing in the protocol that can remove a started quest from the client's own map.
///
/// `forget_completion` is the trailing `u8`:
///
/// * `false` - clear the **started** map only. The right answer to a player-initiated
///   forfeit: the quest goes back to "not started" and the NPC offers it again.
/// * `true` - also clear the **completed** map, making the quest offerable from scratch.
///   The client will never ask for this (see the module docs), so it is a GM-command tool.
///
/// **Do not send this with, or just before, a `SetField`.** `crate::quest` records both ways
/// `0x0089` silently does nothing, and a null `charData` on the first `SetField` is one of
/// them.
pub fn forfeit_reply(quest_id: u32, forget_completion: bool) -> Vec<u8> {
    crate::quest::quest_forgotten(quest_id, forget_completion)
}

// ---------------------------------------------------------------------------------------
// The two opcodes that are NOT the forfeit
// ---------------------------------------------------------------------------------------

/// `0x01A5` - a periodic **counter flush**, and the reason it was mistaken for quest state.
///
/// It carries two key/value lists whose keys look like quest ids in the one map this project
/// tests on, because Mushroom Town's first three NPCs are given object ids 1000/1001/1002 and
/// its first three quests are numbered 1000/1001/1002. They are neither. See
/// [`parse_counter_report`].
pub const CLIENT_COUNTER_FLUSH: u16 = 0x01A5;

/// `0x01ED` - the client's own log/usage channel. **104 builder call sites** across the whole
/// image (`research/msexe-send-opcodes.txt`), from the game dispatcher to the Meso Market
/// window. Its first `u32` is a **report kind**, not a count: kinds `1`, `2`, `0x44` and
/// `0x60` all appear in captures with four different body shapes.
///
/// Kinds 1 and 2 carry the same two entry shapes [`CounterReport`] does, one list per packet.
pub const CLIENT_USAGE_REPORT: u16 = 0x01ED;

/// One entry of the 8-byte list: a `u32` key and a `u32` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counter {
    pub key: u32,
    pub value: u32,
}

/// One entry of the 12-byte list: a **composite** `(u32, u32)` key and a `u32` value.
///
/// The pair really is a composite key and not `(id, count, hash)`: the client's map node
/// compares **both** halves before descending (`142d17e08 cmp [rdx+0x1c],eax / jne /
/// 142d17e0d cmp [rdx+0x20],r11d`), **[L]** - and a capture shows key `1000` twice in one
/// packet with `key2` of `1` and `3`, which no quest list could do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairCounter {
    pub key1: u32,
    pub key2: u32,
    pub value: u32,
}

/// A decoded [`CLIENT_COUNTER_FLUSH`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CounterReport {
    pub singles: Vec<Counter>,
    pub pairs: Vec<PairCounter>,
}

/// Decode a `0x01A5` body.
///
/// ```text
/// u32 countA,  countA * { u32 key, u32 value }
/// u32 countB,  countB * { u32 key1, u32 key2, u32 value }
/// ```
///
/// **[L]**, read off `FUN_142d17960`, the client's *only* builder for this opcode (two call
/// sites, both inside it - `research/msexe-send-opcodes.txt` lines 1086-1087). Both counts are
/// written as a placeholder `u32` and **back-patched** after the loop
/// (`1406ed640` = tell, `1406eda30` = poke at offset), which is why they are counts rather
/// than kinds:
///
/// ```text
/// 142d17a90  call 0x1406ed640     ; r15 = offset of the count field
/// 142d17a9e  call 0x1406ed9d0     ; u32 0            <- placeholder
///   142d17c58  call 0x1406ed9d0   ;   u32 key        \ per entry
///   142d17c63  call 0x1406ed9d0   ;   u32 value      /
///   142d17c68  inc  dword [rsp+0x20]
/// 142d17d2c  call 0x1406eda30     ; poke the real count back at r15
/// ```
///
/// Verified against **six** captured bodies of five different lengths - see the tests. This
/// module does not build one: the packet is client-to-server only.
pub fn parse_counter_report(body: &[u8]) -> Option<CounterReport> {
    let mut r = PacketReader::new(body);
    let n_single = r.u32().ok()? as usize;
    // A count is a client-supplied length. Bound it by what is left rather than trusting it,
    // or a corrupt body allocates gigabytes before failing.
    if n_single.saturating_mul(8) > r.remaining() {
        return None;
    }
    let mut singles = Vec::with_capacity(n_single);
    for _ in 0..n_single {
        singles.push(Counter { key: r.u32().ok()?, value: r.u32().ok()? });
    }
    let n_pair = r.u32().ok()? as usize;
    if n_pair.saturating_mul(12) > r.remaining() {
        return None;
    }
    let mut pairs = Vec::with_capacity(n_pair);
    for _ in 0..n_pair {
        pairs.push(PairCounter {
            key1: r.u32().ok()?,
            key2: r.u32().ok()?,
            value: r.u32().ok()?,
        });
    }
    Some(CounterReport { singles, pairs })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    // -----------------------------------------------------------------------------------
    // The forfeit
    // -----------------------------------------------------------------------------------

    /// The exact body the client put on the wire, twice, on 2026-08-20.
    /// `research/fixtures/quest-forfeit-0151-action3-on-the-wire-world.log:20870`.
    #[test]
    fn the_captured_forfeit_body_is_action_3_and_quest_1001() {
        let body = hex("03e9030000");
        assert_eq!(body.len(), FORFEIT_BODY_LEN);
        assert!(is_forfeit(&body));
        let req = parse_forfeit_request(&body).expect("the captured body parses");
        assert_eq!(req.quest_id, 1001);
    }

    /// Round trip against the same capture, so the builder cannot drift from the parser.
    #[test]
    fn the_builder_reproduces_the_captured_body_byte_for_byte() {
        assert_eq!(forfeit_request(1001), hex("03e9030000"));
    }

    /// The whole reason this file exists: the general parser drops the forfeit on the floor,
    /// and a dropped `0x0151` is silence.
    #[test]
    fn the_general_quest_parser_cannot_read_a_forfeit() {
        let body = hex("03e9030000");
        assert!(
            crate::script::parse_quest_request(&body).is_none(),
            "script::parse_quest_request needs a 9-byte head; a forfeit is 5 bytes"
        );
        assert!(parse_forfeit_request(&body).is_some(), "and this one must");
    }

    /// Every other captured `0x0151` shape must be refused here, or a dispatcher that tries
    /// this parser first would swallow an accept or a completion.
    #[test]
    fn no_other_captured_0151_body_is_mistaken_for_a_forfeit() {
        for b in [
            "01e80300000100000043ffe50100000000", // action 1, accept quest 1000, 17 bytes
            "02e80300000200000043ffe501ffffffff", // action 2, complete 1000, 17 bytes
            "04ea030000030000003dff1301",         // action 4, opening script 1002, 13 bytes
        ] {
            let body = hex(b);
            assert!(!is_forfeit(&body), "{b} is not a forfeit");
            assert!(parse_forfeit_request(&body).is_none(), "{b} must not parse as one");
        }
    }

    #[test]
    fn a_truncated_or_overlong_forfeit_is_refused_rather_than_guessed() {
        assert!(parse_forfeit_request(&hex("03e903")).is_none());
        assert!(parse_forfeit_request(&hex("03e903000000")).is_none());
        assert!(parse_forfeit_request(&[]).is_none());
    }

    /// The reply is `0x0089` sub 1, `u32 questId`, state 0, then the forget-completion `u8`.
    /// Seven bytes, and the state byte is what makes the row disappear.
    #[test]
    fn the_reply_is_a_quest_record_in_state_zero() {
        let body = forfeit_reply(1001, false);
        assert_eq!(body, hex("01e903000000 00"));
        assert_eq!(body[0], crate::quest::MESSAGE_QUEST_RECORD);
        assert_eq!(body[5], crate::quest::QUEST_STATE_NONE);
        assert_eq!(body[6], 0, "started map only");

        let reset = forfeit_reply(1000, true);
        assert_eq!(reset, hex("01e803000000 01"));
        assert_eq!(reset[6], 1, "and the completed map too - the GM-command form");
    }

    // -----------------------------------------------------------------------------------
    // 0x01A5 - six real bodies, five lengths, one layout
    // -----------------------------------------------------------------------------------

    /// `equip-into-empty-hat-slot-kills-client-world.log:103` (06:08:33.647) and the same
    /// 28 bytes in five other runs.
    #[test]
    fn counter_report_28_bytes_one_of_each() {
        let b = hex("01000000 e8030000 6987f2eb 01000000 e8030000 01000000 0bccceb9");
        assert_eq!(b.len(), 28);
        let r = parse_counter_report(&b).unwrap();
        assert_eq!(r.singles, vec![Counter { key: 1000, value: 0xeb_f2_87_69 }]);
        assert_eq!(r.pairs, vec![PairCounter { key1: 1000, key2: 1, value: 0xb9_ce_cc_0b }]);
    }

    /// Same file, line 210 (06:09:59.316): two singles, no pairs.
    #[test]
    fn counter_report_24_bytes_two_singles_and_an_empty_pair_list() {
        let b = hex("02000000 e9030000 4a50dd2e ea030000 d78ec0e4 00000000");
        assert_eq!(b.len(), 24);
        let r = parse_counter_report(&b).unwrap();
        assert_eq!(r.singles.len(), 2);
        assert_eq!(r.singles[1].key, 1002);
        assert!(r.pairs.is_empty());
    }

    /// Same file, line 253 (06:10:09.323): the mirror image - no singles, two pairs.
    /// This body is the one that settles the field order: a leading `00000000` that is a
    /// *count* and not a key.
    #[test]
    fn counter_report_32_bytes_no_singles_and_two_pairs() {
        let b = hex("00000000 02000000 e9030000 01000000 72dd593b ea030000 01000000 f893a090");
        assert_eq!(b.len(), 32);
        let r = parse_counter_report(&b).unwrap();
        assert!(r.singles.is_empty());
        assert_eq!(r.pairs.len(), 2);
        assert_eq!(r.pairs[0].key1, 1001);
        assert_eq!(r.pairs[0].key2, 1);
    }

    /// Same file, line 6758 (06:11:52.223) - and byte-identical in
    /// `attack-with-mobs-present-zero-targets-world.log:3479` on a different day.
    ///
    /// **This is the body that kills the quest reading.** The keys are 0x057BCF00 and
    /// 0x057BF610 - 92 131 584 and 92 140 048. There is no quest with those ids, and the two
    /// share one value, which is what a hash does and an id does not.
    #[test]
    fn counter_report_keys_are_not_quest_ids() {
        let b = hex("02000000 00cf7b05 1611a5ac 10f67b05 1611a5ac 00000000");
        let r = parse_counter_report(&b).unwrap();
        assert_eq!(r.singles[0].key, 0x057B_CF00);
        assert_eq!(r.singles[1].key, 0x057B_F610);
        assert_eq!(r.singles[0].value, r.singles[1].value);
        assert!(r.singles[0].key > 322 * 100_000, "far outside every quest id in this client");
    }

    /// `quest-forfeit-0151-action3-resent-after-relog-world.log`, 03:45:51.735 - the longest
    /// captured body, three of each.
    #[test]
    fn counter_report_68_bytes_three_of_each() {
        let b = hex(
            "03000000 \
             e8030000 6987f2eb e9030000 4a50dd2e ea030000 d78ec0e4 \
             03000000 \
             e8030000 03000000 2cadaa3c \
             e9030000 03000000 1f7b52ff \
             ea030000 02000000 c0710232",
        );
        assert_eq!(b.len(), 68);
        let r = parse_counter_report(&b).unwrap();
        assert_eq!(r.singles.len(), 3);
        assert_eq!(r.pairs.len(), 3);
        assert_eq!(r.pairs[0].key2, 3, "key 1000 with a SECOND key of 3, not 1");
    }

    /// Same file, 03:46:01.756. Twenty bytes, and it decodes with the same rule as the 68.
    #[test]
    fn counter_report_20_bytes() {
        let b = hex("00000000 01000000 e8030000 01000000 0bccceb9");
        let r = parse_counter_report(&b).unwrap();
        assert!(r.singles.is_empty());
        assert_eq!(r.pairs, vec![PairCounter { key1: 1000, key2: 1, value: 0xb9_ce_cc_0b }]);
    }

    /// The same 12-byte entry shape appears in `0x01ED` kind 1 - and there it carries key
    /// **1000 twice**, with second keys 1 and 3. A quest list cannot contain a quest twice,
    /// so the pair is a composite key, exactly as the client's map node comparison says.
    ///
    /// `quest-forfeit-0151-action3-resent-after-relog-world.log`, 03:45:50.706.
    #[test]
    fn the_same_key_appears_twice_in_one_usage_report() {
        let b = hex(
            "04000000 \
             e8030000 01000000 0bccceb9 \
             e8030000 03000000 2cadaa3c \
             e9030000 03000000 1f7b52ff \
             ea030000 02000000 c0710232",
        );
        // Decoded as the pair list alone: prepend a zero single-count and read it as 0x01A5.
        let mut body = 0u32.to_le_bytes().to_vec();
        body.extend_from_slice(&b);
        let r = parse_counter_report(&body).unwrap();
        assert_eq!(r.pairs.len(), 4);
        assert_eq!(r.pairs[0].key1, 1000);
        assert_eq!(r.pairs[1].key1, 1000, "1000 twice - not a quest id");
        assert_ne!(r.pairs[0].key2, r.pairs[1].key2);
    }

    /// A count that claims more entries than the body holds must be refused, not trusted.
    #[test]
    fn an_impossible_count_is_refused() {
        assert!(parse_counter_report(&hex("ffffffff e8030000 6987f2eb")).is_none());
        assert!(parse_counter_report(&hex("00000000 ffffffff")).is_none());
        assert!(parse_counter_report(&hex("0000")).is_none());
    }
}
