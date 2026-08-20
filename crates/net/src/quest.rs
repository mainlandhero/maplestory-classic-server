//! Quest state on the wire: the two presence-gated blocks in the character record, and the
//! packet that moves one quest between states.
//!
//! Labels are the project's: **[L]** read off this client's listing, **[D]** derived from two
//! or more [L] facts, **[I]** inferred. Full working, with the instrument controls, is
//! `research/quest-state.md`.
//!
//! # What was missing before this file
//!
//! Everything about quests worked *except* state. Clicking an NPC opened the real `Quest.wz`
//! dialogue and pressing Accept answered with the quest's `yes` branch - and then nothing
//! happened, because **no packet that accepts or completes a quest had ever been found in
//! either direction**, and the character record carried no quest list. Both are here now.
//!
//! # The three shapes
//!
//! ```text
//! record, presence[9]   u8 bulk, u16 count, { u32 questId, str  progress   }
//! record, presence[14]  u8 bulk, u16 count, { u32 questId, raw8 completedAt }
//! packet 0x0089 sub 1   u32 questId, u8 state, <payload chosen by state>
//! ```
//!
//! # THE RECORD HAS NO LENGTH PREFIX AND NO RESYNC POINT
//!
//! One wrong width here silently desynchronises every byte after it, and the symptom on
//! screen is an **undressed character or no world entry at all** - not a wrong quest list.
//! That is why [`QuestBook::started_block`] writes the bulk flag as a constant rather than
//! taking it from a caller: with `bulk != 0` the client does **not** read the trailing
//! removal `u16` (`test sil, sil / jnz` at `0x140307533`), so the flag and the block length
//! are the same decision. It is the `hasOverride` trap from `0x055B` in a place where the
//! blast radius is the whole record.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A quest is
//! accepted because a packet arrived on the socket.

use crate::packet::PacketWriter;

// ---------------------------------------------------------------------------------------
// The character-record blocks
// ---------------------------------------------------------------------------------------

/// The `u8` that opens each quest block: **1 = this is the whole list, replace what you
/// have**.
///
/// **[L], and read off the client's own encoder rather than guessed.** `FUN_1402e5a30` is the
/// mirror of the record decoder - 38 calls to the same gate helper `FUN_1402fa9a0` - and its
/// bulk arm literally writes this byte:
///
/// ```text
/// 1402e71c5  MOV   DL,1
/// 1402e71c7  CALL  0x1406ed840        ; write_u8(1)
/// 1402e71cc  MOVZX EDX,word [r15+0x127f]
/// 1402e71d7  CALL  0x1406ed940        ; write_u16(count)
/// ```
///
/// On the decoder side a non-zero value makes the client **clear its collection first**
/// (`FUN_1402e0fe0` for started, `FUN_1402e1020` for completed - each zeroes three
/// containers) and then **skip the trailing removal list entirely**. That is what a snapshot
/// wants: a quest the server has dropped stops being shown, rather than lingering because
/// nothing named it.
///
/// The zero form is a *delta* - add these, then erase those - and is what the update packet
/// exists for. It is deliberately not reachable from this module: a record is always a
/// snapshot.
pub const QUEST_BLOCK_BULK: u8 = 1;

/// One quest the character has accepted and not yet turned in.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StartedQuest {
    /// Read as a **`u32`** at `0x1403074f3` (the `0x1406e8f00` thunk), not the `u16` this
    /// game family usually uses. **[L]**
    pub quest_id: u32,
    /// Free-form progress, `u16` byte count then bytes, read at `0x140307501`.
    ///
    /// The client stores it verbatim at `node+0x18` in the map at `charData+0x1273` and
    /// nothing read here parses it, so `""` is a legal value and is what a freshly accepted
    /// quest gets. **[L]** for the storage, **[I]** for "empty is right" - if a quest with a
    /// kill counter shows no progress on screen, this is the field.
    pub progress: String,
}

/// One quest the character has finished.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CompletedQuest {
    pub quest_id: u32,
    /// Eight raw bytes, read at `0x1403075df` with `R8D = 8` as a compile-time constant, and
    /// stored as one qword at `node+0x14`.
    ///
    /// **A Windows FILETIME. [I]** - the width is [L] and so is the round trip
    /// (`FUN_1402e3390` hands the same qword back out), but no call site examined formats it.
    /// A wrong value costs a wrong date and **cannot** desynchronise the record, because the
    /// width is 8 either way. Use [`filetime_from_unix_secs`].
    pub completed_at: u64,
}

/// Everything a character record says about quests.
///
/// Split in two because the client keeps two independent collections - `charData+0x1273` for
/// started and `charData+0x1347` for completed - gated by two different presence bytes and
/// written by two different blocks. A quest is in at most one of them: the update packet's
/// "completed" state adds to the second *and* erases from the first, in that order. **[L]**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QuestBook {
    pub started: Vec<StartedQuest>,
    pub completed: Vec<CompletedQuest>,
}

/// The most entries either block can carry, because the count is a `u16`.
///
/// **This is the client's limit, not ours** - unlike [`crate::opcode::MAX_INVENTORY_SLOTS`],
/// which is arbitrary. A book longer than this is truncated to a *self-consistent* block
/// (the count and the entries always agree) rather than being allowed to write a count that
/// wraps. The client ships 322 quests in total, so reaching it means something is wrong
/// upstream.
pub const MAX_QUESTS_PER_BLOCK: usize = u16::MAX as usize;

impl QuestBook {
    pub fn is_empty(&self) -> bool {
        self.started.is_empty() && self.completed.is_empty()
    }

    /// The `presence[9]` block: `u8 bulk, u16 count, { u32 questId, str progress }`.
    ///
    /// Gate `0x1403074a3`, key `0x143abf360`, region `[0x1403074c4, 0x14030756a)`, six reads -
    /// the same six `research/charrecord-loops.md` §5 row `#19` counted independently. **[L]**
    ///
    /// Every helper the region calls (`FUN_1402e0fe0`, `FUN_1402e0c30`, `FUN_1402e0ec0`)
    /// reads **zero** packet bytes, checked transitively with `tools/reads.py` at depth 3, so
    /// this block costs exactly what is written here.
    pub fn started_block(&self) -> Vec<u8> {
        started_quest_block(&self.started)
    }

    /// The `presence[14]` block: `u8 bulk, u16 count, { u32 questId, raw8 completedAt }`.
    ///
    /// Gate `0x14030757b`, key `0x143abf3d0`, region `[0x140307596, 0x140307633)`, six reads,
    /// matching row `#20`. **[L]**
    pub fn completed_block(&self) -> Vec<u8> {
        completed_quest_block(&self.completed)
    }

    /// How many bytes the two blocks add to a character record.
    pub fn record_len(&self) -> usize {
        self.started_block().len() + self.completed_block().len()
    }
}

/// What an **empty** quest block costs: the bulk `u8` and a zero `u16` count.
///
/// A book with nothing in it is still sent, and that is a decision: `bulk = 1` clears the
/// client's collections, so "this character has no quests" is a statement rather than a
/// silence.
pub const EMPTY_QUEST_BLOCK_LEN: usize = 3;

/// See [`QuestBook::started_block`].
pub fn started_quest_block(started: &[StartedQuest]) -> Vec<u8> {
    let n = started.len().min(MAX_QUESTS_PER_BLOCK);
    let mut w = PacketWriter::new();
    w.u8(QUEST_BLOCK_BULK); //          1403074c7
    w.u16(n as u16); //                 1403074e1
    for q in &started[..n] {
        w.u32(q.quest_id); //           1403074f3
        w.str(&q.progress); //          140307501
    }
    // No trailing removal u16: `test sil,sil / jnz 0x140307568` at 0x140307533 skips it
    // whenever the bulk byte was non-zero. Writing one here would desynchronise the record.
    w.into_vec()
}

/// See [`QuestBook::completed_block`].
pub fn completed_quest_block(completed: &[CompletedQuest]) -> Vec<u8> {
    let n = completed.len().min(MAX_QUESTS_PER_BLOCK);
    let mut w = PacketWriter::new();
    w.u8(QUEST_BLOCK_BULK); //          140307599
    w.u16(n as u16); //                 1403075b3
    for q in &completed[..n] {
        w.u32(q.quest_id); //           1403075c8
        w.u64(q.completed_at); //       1403075df, a raw 8-byte read
    }
    // Same as above: `test r14b,r14b / jnz 0x14030762e` at 0x140307600.
    w.into_vec()
}

/// Seconds between the FILETIME epoch (1601-01-01) and the Unix epoch.
pub const FILETIME_UNIX_EPOCH_SECS: u64 = 11_644_473_600;

/// Unix seconds to the 100-nanosecond FILETIME the record and the packet carry.
///
/// **[I] that the field is a FILETIME**, from this client's other time fields - see
/// [`CompletedQuest::completed_at`] and [`crate::opcode::ITEM_NEVER_EXPIRES`], which is
/// labelled the same way for the same reason. Negative and pre-1601 inputs saturate at zero
/// rather than wrapping, because a wrapped date is a value nobody can debug from a
/// screenshot.
pub fn filetime_from_unix_secs(secs: i64) -> u64 {
    let shifted = secs.saturating_add(FILETIME_UNIX_EPOCH_SECS as i64);
    if shifted <= 0 {
        return 0;
    }
    (shifted as u64).saturating_mul(10_000_000)
}

// ---------------------------------------------------------------------------------------
// The quest-result packet
// ---------------------------------------------------------------------------------------

/// The client's multiplexed "something about you changed" packet.
///
/// **Routing, [L].** `FUN_142cbaa80` - the game-stage dispatcher this project already uses
/// for `0x044F` and friends - has `case 0x89: FUN_142d43ee0(this, packet)`. That function
/// reads one `u8`, bounds it at `0x23`, and jumps through a 36-entry dword table at
/// **`0x142d44a88`** (base `0x140000000`, read straight out of the PE). Only entry
/// [`MESSAGE_QUEST_RECORD`] is decoded here; the other 35 are unread and this module does not
/// guess at them.
pub const MESSAGE: u16 = 0x0089;

/// Sub-case `1` of [`MESSAGE`]: **the quest record update**.
///
/// Table entry 1 is `0x142d43f42`, which is `mov rdx,rsi / mov rcx,r14 /
/// call 0x142d59e20`. `FUN_142d59e20` reads exactly five fields and no others -
/// `tools/reads.py 0x142d59e20 3`, transitively - and three of the five are mutually
/// exclusive payloads chosen by the state byte. **[L]**
pub const MESSAGE_QUEST_RECORD: u8 = 1;

/// State `0`: the character does not have this quest.
///
/// `FUN_142d5b750`'s `edi == 0` arm erases the id from the started map (`FUN_1402e0ec0`) and,
/// when the trailing `u8` is non-zero, from the completed map as well (`FUN_1402e3550`).
/// **[L]**
pub const QUEST_STATE_NONE: u8 = 0;

/// State `1`: in progress.
///
/// `FUN_142d5b750`'s `edi == 1` arm runs **`FUN_140711d70`** - the quest requirement checker
/// that `crate::script`'s `0x0151` documentation already names from the other direction -
/// and then inserts `{questId, progress}` through `FUN_1402e0eb0`, which is
/// `xor r9d,r9d / jmp FUN_1402e0c30`: the same insert the record's block uses, with the bulk
/// flag zero. **[L]**
///
/// The name is **[I]**; what is measured is the behaviour.
pub const QUEST_STATE_IN_PROGRESS: u8 = 1;

/// State `2`: completed.
///
/// `edi == 2` adds `{questId, completedAt}` to the completed map (`FUN_1402e33f0`) **and**
/// erases the id from the started map (`FUN_1402e0ec0`), in that order. **[L]**
pub const QUEST_STATE_COMPLETE: u8 = 2;

/// What a [`quest_record`] tells the client about one quest.
///
/// The payload is part of the state rather than a separate field because the client's three
/// reads are mutually exclusive - `cmp r14d,1 / jne`, `cmp r14d,2 / jne`,
/// `test r14d,r14d / jne` at `0x142d59f15`, `0x142d59f56` and `0x142d59f6f` - so a state and
/// a payload that disagree would leave the body a different length than the client reads.
/// **[L]**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestProgress {
    /// [`QUEST_STATE_NONE`]. `forget_completion` is the trailing `u8` read at `0x142d59f77`:
    /// non-zero also erases the quest from the **completed** collection, so a quest can be
    /// made repeatable. **[L]**
    None { forget_completion: bool },
    /// [`QUEST_STATE_IN_PROGRESS`], carrying the progress string read at `0x142d59f23`.
    InProgress { progress: String },
    /// [`QUEST_STATE_COMPLETE`], carrying the 8 raw bytes read at `0x142d59f68`.
    Complete { completed_at: u64 },
}

impl QuestProgress {
    /// The `u8` the client switches on.
    pub fn state(&self) -> u8 {
        match self {
            QuestProgress::None { .. } => QUEST_STATE_NONE,
            QuestProgress::InProgress { .. } => QUEST_STATE_IN_PROGRESS,
            QuestProgress::Complete { .. } => QUEST_STATE_COMPLETE,
        }
    }
}

/// The [`MESSAGE`] **body** - no opcode; the caller prepends it, the same way
/// [`crate::opcode::npc_enter_field`] is used.
///
/// ```
/// use net::quest::{quest_record, QuestProgress, MESSAGE, MESSAGE_QUEST_RECORD};
/// let body = quest_record(1000, &QuestProgress::InProgress { progress: String::new() });
/// assert_eq!(MESSAGE, 0x0089);
/// assert_eq!(body[0], MESSAGE_QUEST_RECORD);
/// assert_eq!(&body[1..5], &1000u32.to_le_bytes());
/// ```
///
/// # Two ways this packet does nothing, silently
///
/// Both are at the top of `FUN_142d59e20` and both `return` without touching anything. **[L]**
///
/// * `FUN_141b1f960([0x143abea80], 1)` - a bit-0 test on `+4` of a singleton with 171
///   references, one of them `FUN_141820080` (`CField::OnPacket`). What the bit *means* is
///   **not established**; that a set bit makes this packet a no-op is.
/// * `FUN_142cbe730(world)` is `mov rax,[rcx+0x2358]; ret` - the character-data object - and
///   a null one returns immediately. `STATUS.md` records that field measuring **`0x00` on the
///   first `SetField`** and non-zero on every later one.
///
/// So **do not send this with, or just before, a `SetField`.** It is the same rule
/// `crate::script` carries for `0x055B`, arrived at from a different function, and the
/// consequence is milder: this packet is not one the client blocks on, so a lost one costs a
/// stale journal rather than a frozen UI.
pub fn quest_record(quest_id: u32, progress: &QuestProgress) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(MESSAGE_QUEST_RECORD); //          142d43f0c, the sub-case selector
    w.u32(quest_id); //                     142d59e74
    w.u8(progress.state()); //              142d59e7f
    match progress {
        QuestProgress::None { forget_completion } => {
            w.bool(*forget_completion); //  142d59f77
        }
        QuestProgress::InProgress { progress } => {
            w.str(progress); //             142d59f23
        }
        QuestProgress::Complete { completed_at } => {
            w.u64(*completed_at); //        142d59f68, a raw 8-byte read
        }
    }
    w.into_vec()
}

/// "You have this quest now." The body a quest acceptance sends.
pub fn quest_accepted(quest_id: u32) -> Vec<u8> {
    quest_record(quest_id, &QuestProgress::InProgress { progress: String::new() })
}

/// "This quest is finished", with the completion time as a FILETIME.
pub fn quest_completed(quest_id: u32, completed_at: u64) -> Vec<u8> {
    quest_record(quest_id, &QuestProgress::Complete { completed_at })
}

/// "You do not have this quest." `forget_completion` also clears it from the completed list.
pub fn quest_forgotten(quest_id: u32, forget_completion: bool) -> Vec<u8> {
    quest_record(quest_id, &QuestProgress::None { forget_completion })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The empty book is three bytes a block, and those three bytes are not optional: the
    /// bulk flag *clears* the client's collections, which is how a dropped quest stops being
    /// shown.
    #[test]
    fn an_empty_book_is_a_bulk_flag_and_a_zero_count() {
        let book = QuestBook::default();
        assert!(book.is_empty());
        assert_eq!(book.started_block(), vec![QUEST_BLOCK_BULK, 0x00, 0x00]);
        assert_eq!(book.completed_block(), vec![QUEST_BLOCK_BULK, 0x00, 0x00]);
        assert_eq!(book.started_block().len(), EMPTY_QUEST_BLOCK_LEN);
        assert_eq!(book.record_len(), 2 * EMPTY_QUEST_BLOCK_LEN);
    }

    /// Every offset is where the client's decoder reads that field, and the record has no
    /// resync point after any of them.
    #[test]
    fn the_started_block_is_the_bytes_the_decoder_reads() {
        let b = started_quest_block(&[StartedQuest { quest_id: 1000, progress: String::new() }]);
        assert_eq!(b[0], QUEST_BLOCK_BULK, "1403074c7 bulk");
        assert_eq!(&b[1..3], &1u16.to_le_bytes(), "1403074e1 count");
        assert_eq!(&b[3..7], &1000u32.to_le_bytes(), "1403074f3 questId, a u32 not a u16");
        assert_eq!(&b[7..9], &0u16.to_le_bytes(), "140307501 string byte count");
        assert_eq!(b.len(), 9, "and nothing after it - no removal u16 in the bulk form");

        let with_text =
            started_quest_block(&[StartedQuest { quest_id: 1000, progress: "007".into() }]);
        assert_eq!(&with_text[7..9], &3u16.to_le_bytes());
        assert_eq!(&with_text[9..12], b"007");
        assert_eq!(with_text.len(), 12);
    }

    /// `0x1403075df` is `raw(8)` with `R8D` a compile-time constant, so the time is eight
    /// bytes whatever it holds - it cannot change the block's length.
    #[test]
    fn the_completed_block_carries_a_fixed_eight_byte_time() {
        let b = completed_quest_block(&[CompletedQuest { quest_id: 1000, completed_at: 7 }]);
        assert_eq!(b[0], QUEST_BLOCK_BULK, "140307599 bulk");
        assert_eq!(&b[1..3], &1u16.to_le_bytes(), "1403075b3 count");
        assert_eq!(&b[3..7], &1000u32.to_le_bytes(), "1403075c8 questId");
        assert_eq!(&b[7..15], &7u64.to_le_bytes(), "1403075df raw(8)");
        assert_eq!(b.len(), 15);

        // A different value is the same length. That is the whole point of the [I] on the
        // FILETIME reading being harmless.
        let other =
            completed_quest_block(&[CompletedQuest { quest_id: 1000, completed_at: u64::MAX }]);
        assert_eq!(other.len(), b.len());
    }

    /// A block's count and its entries can never disagree, even if a caller hands over more
    /// quests than a `u16` can count. A count that wrapped would desynchronise the record.
    #[test]
    fn the_count_always_matches_the_entries_written() {
        for n in [0usize, 1, 2, 17] {
            let started: Vec<StartedQuest> = (0..n)
                .map(|i| StartedQuest { quest_id: 1000 + i as u32, progress: String::new() })
                .collect();
            let b = started_quest_block(&started);
            assert_eq!(&b[1..3], &(n as u16).to_le_bytes());
            // 3 head + 6 per entry (u32 id + empty u16-prefixed string)
            assert_eq!(b.len(), 3 + 6 * n);

            let completed: Vec<CompletedQuest> = (0..n)
                .map(|i| CompletedQuest { quest_id: 1000 + i as u32, completed_at: 1 })
                .collect();
            let c = completed_quest_block(&completed);
            assert_eq!(&c[1..3], &(n as u16).to_le_bytes());
            assert_eq!(c.len(), 3 + 12 * n);
        }
    }

    /// The count is a `u16` and the entries are written to match it, so an over-long book is
    /// truncated rather than allowed to lie about its length.
    #[test]
    fn an_over_long_book_stays_self_consistent() {
        assert_eq!(MAX_QUESTS_PER_BLOCK, 65535);
        let started: Vec<StartedQuest> = (0..MAX_QUESTS_PER_BLOCK + 3)
            .map(|i| StartedQuest { quest_id: i as u32, progress: String::new() })
            .collect();
        let b = started_quest_block(&started);
        assert_eq!(&b[1..3], &(MAX_QUESTS_PER_BLOCK as u16).to_le_bytes());
        assert_eq!(b.len(), 3 + 6 * MAX_QUESTS_PER_BLOCK);
    }

    /// The update packet, field for field against `FUN_142d59e20`.
    #[test]
    fn the_quest_record_packet_is_the_five_reads_the_client_makes() {
        let accepted = quest_accepted(1000);
        assert_eq!(accepted[0], MESSAGE_QUEST_RECORD, "the 0x0089 sub-case, read at 142d43f0c");
        assert_eq!(&accepted[1..5], &1000u32.to_le_bytes(), "142d59e74 questId");
        assert_eq!(accepted[5], QUEST_STATE_IN_PROGRESS, "142d59e7f state");
        assert_eq!(&accepted[6..8], &0u16.to_le_bytes(), "142d59f23 progress string");
        assert_eq!(accepted.len(), 8);

        let done = quest_completed(1000, 0x0102_0304_0506_0708);
        assert_eq!(done[5], QUEST_STATE_COMPLETE);
        assert_eq!(&done[6..14], &0x0102_0304_0506_0708u64.to_le_bytes(), "142d59f68 raw(8)");
        assert_eq!(done.len(), 14);

        let gone = quest_forgotten(1000, false);
        assert_eq!(gone[5], QUEST_STATE_NONE);
        assert_eq!(gone[6], 0, "142d59f77 - and non-zero also clears the completed list");
        assert_eq!(gone.len(), 7);
        assert_eq!(quest_forgotten(1000, true)[6], 1);
    }

    /// The state byte and the payload are one value, so they cannot disagree - which they
    /// would have to for the body to be a different length than the client reads.
    #[test]
    fn the_state_byte_always_names_the_payload_that_follows() {
        for (progress, state, len) in [
            (QuestProgress::None { forget_completion: true }, QUEST_STATE_NONE, 7),
            (QuestProgress::InProgress { progress: "abc".into() }, QUEST_STATE_IN_PROGRESS, 11),
            (QuestProgress::Complete { completed_at: 1 }, QUEST_STATE_COMPLETE, 14),
        ] {
            assert_eq!(progress.state(), state);
            let b = quest_record(1000, &progress);
            assert_eq!(b[5], state);
            assert_eq!(b.len(), len);
        }
    }

    /// A sanity anchor rather than a client measurement: the FILETIME epoch conversion, and
    /// the saturation that keeps a bad clock from wrapping into a plausible-looking date.
    #[test]
    fn the_filetime_conversion_pins_a_known_pair() {
        // 1970-01-01 in FILETIME is 116444736000000000, the constant every Win32 example
        // carries.
        assert_eq!(filetime_from_unix_secs(0), 116_444_736_000_000_000);
        assert_eq!(filetime_from_unix_secs(1), 116_444_736_010_000_000);
        // Before 1601 there is no representable value; zero rather than a wrap.
        assert_eq!(filetime_from_unix_secs(-(FILETIME_UNIX_EPOCH_SECS as i64)), 0);
        assert_eq!(filetime_from_unix_secs(i64::MIN), 0);
        assert_ne!(filetime_from_unix_secs(i64::MAX), 0);
    }

    /// The opcode and the sub-case, so a rename or a typo cannot go unnoticed.
    #[test]
    fn the_opcode_is_the_one_the_dispatcher_switches_on() {
        assert_eq!(MESSAGE, 0x0089); // FUN_142cbaa80 case 0x89 -> FUN_142d43ee0
        assert_eq!(MESSAGE_QUEST_RECORD, 1); // table 0x142d44a88[1] -> FUN_142d59e20
    }

    // -----------------------------------------------------------------------------------
    // The record assembly. These live here rather than in `opcode.rs` so the quest work is
    // one file's worth of edit; the functions under test are
    // `crate::opcode::character_record_for_set_field_with_quests` and its `SetField`.
    // -----------------------------------------------------------------------------------

    use crate::opcode::{
        character_record_for_set_field, character_record_for_set_field_with_quests,
        set_field_with_character_dressed_quests, Character, EquipStats, PRESENCE_ARRAY_LEN,
        PRESENCE_CHARACTER_STAT, PRESENCE_EQUIPPED, PRESENCE_INVENTORY_SIZE,
        PRESENCE_QUEST_COMPLETED, PRESENCE_QUEST_STARTED,
    };

    fn dressed() -> Character {
        Character {
            name: "Quester".into(),
            equips: vec![(5, 1_040_002), (6, 1_060_002), (7, 1_072_003), (11, 1_302_000)],
            ..Character::default()
        }
    }

    fn equips_of(chr: &Character) -> Vec<(u8, u32, EquipStats)> {
        chr.equips.iter().map(|&(s, i)| (s, i, EquipStats::default())).collect()
    }

    /// The two presence bytes are 9 and 14, and setting them turns nothing else on: each
    /// appears in exactly one row of the 40-row gate table, and neither is reachable through
    /// a dynamic gate (those select only entries 0-6, presence bytes 0/44/6/5/4/3/2).
    #[test]
    fn the_record_sets_exactly_five_presence_bytes() {
        assert_eq!(PRESENCE_QUEST_STARTED, 9);
        assert_eq!(PRESENCE_QUEST_COMPLETED, 14);

        let chr = dressed();
        let record =
            character_record_for_set_field_with_quests(&chr, 0, &equips_of(&chr), &QuestBook::default());
        let expected = [
            PRESENCE_CHARACTER_STAT,
            PRESENCE_EQUIPPED,
            PRESENCE_INVENTORY_SIZE,
            PRESENCE_QUEST_STARTED,
            PRESENCE_QUEST_COMPLETED,
        ];
        for byte in expected {
            assert_eq!(record[byte], 1, "presence[{byte}] is not switched on");
        }
        assert!(
            record[..PRESENCE_ARRAY_LEN]
                .iter()
                .enumerate()
                .all(|(i, &b)| (b == 0) != expected.contains(&i)),
            "a presence byte outside the five opens a block nobody wrote"
        );
    }

    /// **The record has no length prefix and no resync point**, so where the blocks go is the
    /// whole of what makes the bytes after them readable. They go after the equipped list and
    /// in front of the final ungated `u8` at `0x140308b3f`, started first.
    #[test]
    fn the_blocks_go_in_front_of_the_final_ungated_byte() {
        let chr = dressed();
        let equips = equips_of(&chr);
        let bare = character_record_for_set_field(&chr, 0);

        let book = QuestBook {
            started: vec![StartedQuest { quest_id: 1000, progress: "007".into() }],
            completed: vec![CompletedQuest { quest_id: 1001, completed_at: 42 }],
        };
        let with = character_record_for_set_field_with_quests(&chr, 0, &equips, &book);

        // Everything from the end of the presence array to the base record's last byte is
        // byte-identical - the stat block, the bag and the equipped list do not move.
        assert_eq!(
            &with[PRESENCE_ARRAY_LEN..bare.len() - 1],
            &bare[PRESENCE_ARRAY_LEN..bare.len() - 1]
        );
        // Inside the presence array, exactly the two quest bytes changed.
        for i in 0..PRESENCE_ARRAY_LEN {
            let expected_change = i == PRESENCE_QUEST_STARTED || i == PRESENCE_QUEST_COMPLETED;
            assert_eq!(with[i] != bare[i], expected_change, "presence[{i}]");
        }
        assert_eq!(bare[PRESENCE_QUEST_STARTED], 0, "the base record does not set it");
        assert_eq!(with[PRESENCE_QUEST_STARTED], 1);

        let inserted = &with[bare.len() - 1..with.len() - 1];
        let mut expected = book.started_block();
        expected.extend_from_slice(&book.completed_block());
        assert_eq!(inserted, &expected[..], "started first, completed second");

        assert_eq!(*with.last().unwrap(), *bare.last().unwrap(), "0x140308b3f still last");
        assert_eq!(with.len(), bare.len() + book.record_len());
    }

    /// An empty book costs six bytes, and that is a decision rather than an accident: the
    /// bulk flag clears the client's collections, so a dropped quest stops being shown.
    #[test]
    fn an_empty_book_costs_exactly_six_bytes() {
        let chr = dressed();
        let equips = equips_of(&chr);
        let bare = character_record_for_set_field(&chr, 0);
        let with =
            character_record_for_set_field_with_quests(&chr, 0, &equips, &QuestBook::default());
        assert_eq!(with.len(), bare.len() + 6);
        assert_eq!(
            &with[bare.len() - 1..with.len() - 1],
            &[QUEST_BLOCK_BULK, 0, 0, QUEST_BLOCK_BULK, 0, 0]
        );
    }

    /// The `SetField` carrying the quest blocks differs from the plain dressed one by exactly
    /// the record's growth - the head, the three leading `u32`s and the 385-byte margin are
    /// all unchanged.
    #[test]
    fn the_set_field_grows_by_the_blocks_and_nothing_else() {
        let chr = dressed();
        let equips = equips_of(&chr);
        let book = QuestBook {
            started: vec![StartedQuest { quest_id: 1000, progress: String::new() }],
            completed: Vec::new(),
        };
        let plain = crate::opcode::set_field_with_character_dressed(&chr, 0, 0, 0, &equips);
        let quested = set_field_with_character_dressed_quests(&chr, 0, 0, 0, &equips, &book);
        assert_eq!(quested.len(), plain.len() + book.record_len());
        assert_eq!(&quested[..40], &plain[..40], "the head is untouched");
    }
}
