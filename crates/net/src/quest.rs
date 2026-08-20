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

// ---------------------------------------------------------------------------------------
// The progress string - what actually makes `1 / 10` move
// ---------------------------------------------------------------------------------------

/// Characters per kill counter in the progress string. **[L]**
///
/// `FUN_14070cb70` is the client's "how many of mob *slot* has this character killed for
/// this quest" accessor, and it reads the answer straight out of the progress string kept
/// in the started-quest map (`charData+0x1273`, node `+0x18`):
///
/// ```text
/// 14070ccd8  mov   eax, 0x55555556
/// 14070ccdd  imul  dword ptr [rbx - 8]   ; length / 3
/// 14070cce7  lea   eax, [rdx + rdx*2]    ; (length / 3) * 3
/// 14070ccea  cmp   dword ptr [rbx - 8], eax
/// 14070cced  jne   <give up, return 0>   ; length MUST be a multiple of 3
/// ...
/// 14070cd1a  cmp   edx, ecx              ; length/3  vs  number of mob requirements
/// 14070cd1c  jb    <give up, return 0>   ; too short -> every counter reads 0
/// 14070cd22  lea   r8d, [r14 + r14*2]    ; start = slot * 3
/// 14070cd26  lea   r9d, [r8 + 3]         ; end   = slot * 3 + 3
/// 14070cd34  call  0x14019ce60           ; substr(start, end)
/// 14070cd46  call  0x142f11a94           ; and atoi it, base 10
/// ```
///
/// Both primitives were read rather than assumed: `0x14019ce60` bounds-checks `start` and
/// `end` against the length and allocates `end - start + 0x11`, so it is a substring;
/// `0x142f11a94` passes `r8d = 0xa` to `0x142f10268`, so it is a base-10 conversion.
pub const PROGRESS_FIELD_WIDTH: usize = 3;

/// The largest count a progress field can carry, because it is three decimal characters.
///
/// Nothing in this client needs it: the largest `count` in any `Check.<state>.mob.*` is
/// **200** (`gm-handbook/questreq.txt`). [`kill_progress`] saturates here rather than
/// letting a four-digit number push every later field along by one character, which would
/// break the multiple-of-three test above and silently zero **all** of the quest's counters
/// - not just the one that overflowed.
pub const MAX_PROGRESS_FIELD_VALUE: u32 = 999;

/// Build the progress string for a quest from one count per mob requirement, in slot order.
///
/// Slot order is the order of `gm-handbook/questreq.txt`'s `mob` rows, which is the order of
/// the client's own `QuestData+0x100` array. **[D]**, and checked against the client's data
/// three ways: quest 1009's summary reads `#o3# #a10091#\n#o4# #a10092#\n#o5# #a10093#`
/// against slots 0/1/2 = mobs 3/4/5, and quest **10113** is the discriminating case - its
/// slots are mobs **18 then 17**, and its summary is `#o18# #a101131#\n#o17# #a101132#`, so
/// the array is in `Quest.wz` key order and is *not* sorted by template id.
///
/// ```
/// use net::quest::kill_progress;
/// assert_eq!(kill_progress(&[4]), "004");            // quest 1006, four snails down
/// assert_eq!(kill_progress(&[10, 0, 3]), "010000003");
/// assert_eq!(kill_progress(&[]), "");                // a quest with no kill requirement
/// ```
pub fn kill_progress(counts: &[u32]) -> String {
    let mut s = String::with_capacity(counts.len() * PROGRESS_FIELD_WIDTH);
    for c in counts {
        let v = (*c).min(MAX_PROGRESS_FIELD_VALUE);
        s.push_str(&format!("{:0width$}", v, width = PROGRESS_FIELD_WIDTH));
    }
    s
}

/// Read one slot back out, **exactly the way the client does**, failure modes included.
///
/// `slots` is how many mob requirements the quest has. It is a parameter because the client
/// checks the string against it (`length / 3 >= slots`, `14070cd1a`) and answers **0** for
/// every slot when it does not hold - so a string that is one field short does not make one
/// counter wrong, it makes all of them read zero. Mirroring that here is the point: a server
/// that reads its own strings more forgivingly than the client would show progress in
/// `world.log` and none on screen.
///
/// ```
/// use net::quest::kill_count_at;
/// assert_eq!(kill_count_at("010000003", 2, 3), 3);
/// assert_eq!(kill_count_at("", 0, 1), 0);        // freshly accepted, nothing killed
/// assert_eq!(kill_count_at("01000", 0, 2), 0);   // not a multiple of 3 -> all zero
/// assert_eq!(kill_count_at("010", 0, 2), 0);     // too short for 2 slots -> all zero
/// ```
pub fn kill_count_at(progress: &str, slot: usize, slots: usize) -> u32 {
    if !progress.is_ascii() || !progress.len().is_multiple_of(PROGRESS_FIELD_WIDTH) {
        return 0;
    }
    let fields = progress.len() / PROGRESS_FIELD_WIDTH;
    if fields < slots || slot >= fields {
        return 0;
    }
    let start = slot * PROGRESS_FIELD_WIDTH;
    let field = &progress[start..start + PROGRESS_FIELD_WIDTH];
    // Mirror `atoi`, which consumes leading digits and stops at the first non-digit - so a
    // field of `"12x"` is 12 and one of `"x12"` is 0. Nothing this crate writes is ever
    // anything but three digits; this exists so that reading a string back gives the same
    // answer the client would give, whatever put it in the database.
    let digits: String =
        field.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().unwrap_or(0)
}

/// Every slot at once, as the server's own working state.
pub fn kill_counts(progress: &str, slots: usize) -> Vec<u32> {
    (0..slots).map(|s| kill_count_at(progress, s, slots)).collect()
}

/// Count one kill against one slot and return the new progress string, or `None` when
/// nothing changed.
///
/// `None` is the useful half. The client compares the incoming string against the one it
/// already holds and **skips all of its change handling when they are equal** -
/// `142d5be19 cmp rcx, rdx / je` in the `0x0089` state-1 arm, where `rcx` is the old string
/// saved off the map node at `142d5bde9` and `rdx` is the one that just arrived. **[L]** So
/// a `0x0089` carrying an unchanged string is not merely wasteful, it is invisible; there is
/// no point sending one, and a caller that treats "no change" as "send anyway" will not be
/// able to tell a working counter from a broken one.
///
/// The count saturates at `required` for the same reason the game does: past that the
/// requirement is met, the string stops changing, and the client stops being told.
pub fn count_kill(progress: &str, slot: usize, slots: usize, required: u32) -> Option<String> {
    let mut counts = kill_counts(progress, slots);
    let c = counts.get_mut(slot)?;
    if *c >= required {
        return None;
    }
    *c += 1;
    let next = kill_progress(&counts);
    if next == progress {
        None
    } else {
        Some(next)
    }
}

// ---------------------------------------------------------------------------------------
// The requirements, out of the client's own Quest.wz
// ---------------------------------------------------------------------------------------

/// "Kill `count` of mob template `template_id`", at position `slot` in the progress string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MobRequirement {
    /// Index into the progress string, in fields of [`PROGRESS_FIELD_WIDTH`].
    pub slot: usize,
    /// The `Check` state this belongs to. **Every mob row in this client is state 1**, the
    /// completion state; state 0 would gate accepting the quest.
    pub state: u32,
    pub template_id: u32,
    pub count: u32,
}

/// "Hold `count` of item `item_id`."
///
/// **`slot` is informational.** Unlike the mob list, nothing on the wire is addressed by it:
/// the client looks an item requirement up by **id**, both in the requirement checker
/// (`140712220 mov edx,[r14+rbx]` then `call 0x1403eb020`) and in the progress renderer
/// (`142743b5a`, the same two instructions), and compares the answer against `[entry+0x10]`.
/// It is carried so a caller can report requirements in the client's own order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemRequirement {
    pub slot: usize,
    pub state: u32,
    pub item_id: u32,
    pub count: u32,
}

/// One quest's requirements.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QuestRequirements {
    pub mobs: Vec<MobRequirement>,
    pub items: Vec<ItemRequirement>,
}

impl QuestRequirements {
    /// How many fields the progress string must carry: one per mob requirement.
    ///
    /// A quest with no kill requirement wants the **empty** string, not `"000"`. The
    /// client's length test is `length / 3 >= slots`, so a longer string is tolerated - but
    /// an accurate one costs nothing and keeps the wire form the same shape every time.
    pub fn progress_slots(&self) -> usize {
        self.mobs.len()
    }

    /// The progress string for a quest just accepted: zero-filled to the right length.
    ///
    /// The empty string works too - the client's `length / 3 >= slots` test fails, every
    /// counter reads 0, and `#a` renders `0 / 10` from the fallback. **[L]**, and it is what
    /// [`quest_accepted`] sends today. Zero-filling is preferred here only because it makes
    /// the accepted form identical in shape to every later update, so exactly one code path
    /// builds a progress string.
    pub fn fresh_progress(&self) -> String {
        kill_progress(&vec![0u32; self.progress_slots()])
    }

    /// Are the kill requirements met by this progress string?
    ///
    /// This is the server's half of the client's own gate: `FUN_140711e50` fetches the same
    /// count through `FUN_14070cb70` and compares it against the requirement's `+0x04`
    /// (`1407128ce cmp r14d, [rdi+rax+4] / jl 1407128e7` -> `r12d = 6`). **[L]**
    pub fn kills_met(&self, progress: &str) -> bool {
        let slots = self.progress_slots();
        self.mobs.iter().all(|m| kill_count_at(progress, m.slot, slots) >= m.count)
    }

    /// Are the item requirements met? `held` answers "how many of this item does the
    /// character have", across every inventory.
    ///
    /// **The client asks the same question of the bag, live, every time it checks** - it
    /// keeps no running total and the progress string has no field for items. So this is a
    /// pure function of the inventory at the moment it is called, and there is nothing for
    /// the server to maintain per pick-up.
    pub fn items_met(&self, mut held: impl FnMut(u32) -> u32) -> bool {
        self.items.iter().all(|i| held(i.item_id) >= i.count)
    }
}

/// Every quest's requirements, parsed from `gm-handbook/questreq.txt`.
///
/// That file is **generated** by `tools/dump_quests.py` from the client's own
/// `Quest/QuestData/QuestData_000.wz` and is gitignored; regenerate it, never edit it. This
/// type takes the file's *contents* rather than a path so the crate stays free of any
/// opinion about the working directory - `crates/world` already reads the other
/// `gm-handbook` tables that way.
#[derive(Debug, Clone, Default)]
pub struct QuestRequirementTable {
    by_quest: std::collections::BTreeMap<u32, QuestRequirements>,
    by_mob: std::collections::BTreeMap<u32, Vec<u32>>,
}

impl QuestRequirementTable {
    /// Parse the file. Unparseable and commented lines are skipped; the format is
    /// `questId <TAB> state <TAB> kind <TAB> slot <TAB> id <TAB> count`.
    pub fn parse(text: &str) -> Self {
        let mut t = QuestRequirementTable::default();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split('\t').map(str::trim).collect();
            if f.len() < 6 {
                continue;
            }
            let (quest_id, state, slot, id, count) = match (
                f[0].parse::<u32>(),
                f[1].parse::<u32>(),
                f[3].parse::<usize>(),
                f[4].parse::<u32>(),
                f[5].parse::<u32>(),
            ) {
                (Ok(q), Ok(s), Ok(sl), Ok(i), Ok(c)) => (q, s, sl, i, c),
                _ => continue,
            };
            let entry = t.by_quest.entry(quest_id).or_default();
            match f[2] {
                "mob" => {
                    entry.mobs.push(MobRequirement { slot, state, template_id: id, count });
                    let q = t.by_mob.entry(id).or_default();
                    if !q.contains(&quest_id) {
                        q.push(quest_id);
                    }
                }
                "item" => {
                    entry.items.push(ItemRequirement { slot, state, item_id: id, count })
                }
                _ => continue,
            }
        }
        // The file is already in slot order, but a table whose correctness depends on the
        // order of lines in a generated file is one regeneration away from being wrong in
        // the quiet direction - and `slot` is the field the wire format is addressed by.
        for r in t.by_quest.values_mut() {
            r.mobs.sort_by_key(|m| m.slot);
            r.items.sort_by_key(|i| i.slot);
        }
        t
    }

    pub fn get(&self, quest_id: u32) -> Option<&QuestRequirements> {
        self.by_quest.get(&quest_id)
    }

    pub fn len(&self) -> usize {
        self.by_quest.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_quest.is_empty()
    }

    /// Which quests care that this mob template died.
    ///
    /// **A template can appear in several quests** - template 13 is named by six of them -
    /// so a kill has to fan out rather than stop at the first match.
    pub fn quests_for_mob(&self, template_id: u32) -> &[u32] {
        self.by_mob.get(&template_id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// The slots in `quest_id` that this mob template fills. Normally one; a quest is free
    /// to name the same template twice and nothing in the client stops it.
    pub fn slots_for_mob(&self, quest_id: u32, template_id: u32) -> Vec<&MobRequirement> {
        self.get(quest_id)
            .map(|r| r.mobs.iter().filter(|m| m.template_id == template_id).collect())
            .unwrap_or_default()
    }
}

/// One kill, applied to one started quest: the new progress string, or `None` if this kill
/// changes nothing.
///
/// `None` covers three cases that are all "send no packet": the quest has no requirement on
/// this template, the counter is already at its target, and the string came back identical.
/// See [`count_kill`] for why an identical string is worse than useless.
pub fn apply_kill(
    reqs: &QuestRequirements,
    progress: &str,
    template_id: u32,
) -> Option<String> {
    let slots = reqs.progress_slots();
    let mut next: Option<String> = None;
    for m in reqs.mobs.iter().filter(|m| m.template_id == template_id) {
        let base = next.as_deref().unwrap_or(progress);
        if let Some(s) = count_kill(base, m.slot, slots, m.count) {
            next = Some(s);
        }
    }
    next
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

    // ----------------------------------------------------------------------------------
    // The progress string
    // ----------------------------------------------------------------------------------

    /// Sam's Suggestion, quest 1006: one mob requirement, template 2, ten of them.
    ///
    /// The whole point of the file is this string, so it is pinned as bytes rather than
    /// described. `"004"` is what the client turns into `4 / 10`.
    fn sams_suggestion() -> QuestRequirements {
        QuestRequirements {
            mobs: vec![MobRequirement { slot: 0, state: 1, template_id: 2, count: 10 }],
            items: Vec::new(),
        }
    }

    #[test]
    fn four_snails_is_the_three_bytes_zero_zero_four() {
        assert_eq!(kill_progress(&[4]), "004");
        assert_eq!(kill_progress(&[4]).as_bytes(), b"004");
        assert_eq!(kill_count_at("004", 0, 1), 4);
    }

    /// The whole `0x0089` body for one kill, byte for byte. `01` sub-case, `u32` quest id,
    /// `01` state, then the string as `u16` length + bytes.
    #[test]
    fn the_progress_packet_for_sams_fourth_snail_is_these_bytes() {
        let body = quest_record(
            1006,
            &QuestProgress::InProgress { progress: kill_progress(&[4]) },
        );
        assert_eq!(
            body,
            vec![
                MESSAGE_QUEST_RECORD, // 01        142d43f0c
                0xEE, 0x03, 0x00, 0x00, //         142d59e74, quest 1006
                QUEST_STATE_IN_PROGRESS, // 01     142d59e7f
                0x03, 0x00, //                     142d59f23, the string's u16 length
                b'0', b'0', b'4', //               "004"
            ]
        );
        assert_eq!(MESSAGE, 0x0089);
    }

    /// Three slots, and the middle one is not the first: quest 1009 kills mobs 3, 4 and 5.
    #[test]
    fn each_slot_is_three_characters_at_its_own_offset() {
        let p = kill_progress(&[10, 7, 0]);
        assert_eq!(p, "010007000");
        assert_eq!(kill_count_at(&p, 0, 3), 10);
        assert_eq!(kill_count_at(&p, 1, 3), 7);
        assert_eq!(kill_count_at(&p, 2, 3), 0);
    }

    /// The client's two length gates, both of which answer **0 for every slot** rather than
    /// for the offending one. `14070ccea` rejects a length that is not a multiple of three;
    /// `14070cd1a` rejects one shorter than the quest's mob count.
    #[test]
    fn a_malformed_string_reads_as_zero_everywhere_not_just_where_it_is_wrong() {
        assert_eq!(kill_count_at("0100", 0, 1), 0, "length 4 is not a multiple of 3");
        assert_eq!(kill_count_at("010", 0, 2), 0, "one field for a two-mob quest");
        assert_eq!(kill_count_at("010007", 0, 2), 10, "and two fields is enough");
        assert_eq!(kill_count_at("010007", 1, 2), 7);
    }

    /// A freshly accepted quest has nothing killed, and the empty string is a legal way to
    /// say so - the client's short-string path returns 0 rather than faulting.
    #[test]
    fn an_empty_string_is_zero_kills_and_not_an_error() {
        assert_eq!(kill_count_at("", 0, 1), 0);
        assert_eq!(sams_suggestion().fresh_progress(), "000");
        assert_eq!(kill_progress(&[]), "", "a quest with no kill requirement");
    }

    /// Saturating at three digits rather than overflowing into a fourth. A four-character
    /// field would fail the multiple-of-three test and zero **every** counter in the quest.
    #[test]
    fn a_count_past_999_saturates_rather_than_widening_the_field() {
        let p = kill_progress(&[5000, 1]);
        assert_eq!(p, "999001");
        assert_eq!(p.len() % PROGRESS_FIELD_WIDTH, 0);
        assert_eq!(kill_count_at(&p, 1, 2), 1);
    }

    /// `None` means "send nothing". The client skips all of its change handling when the
    /// string it receives equals the one it holds (`142d5be19 cmp rcx, rdx / je`), so an
    /// unchanged string is invisible, not merely wasteful.
    #[test]
    fn a_kill_that_changes_nothing_reports_nothing() {
        let r = sams_suggestion();
        assert_eq!(apply_kill(&r, "000", 2).as_deref(), Some("001"));
        assert_eq!(apply_kill(&r, "009", 2).as_deref(), Some("010"));
        assert_eq!(apply_kill(&r, "010", 2), None, "already at ten of ten");
        assert_eq!(apply_kill(&r, "000", 99), None, "not this quest's monster");
    }

    /// Ten kills take the quest from unmet to met, and the string is the only state.
    #[test]
    fn ten_snails_satisfies_sams_suggestion() {
        let r = sams_suggestion();
        let mut p = r.fresh_progress();
        for _ in 0..10 {
            assert!(!r.kills_met(&p));
            p = apply_kill(&r, &p, 2).expect("each of the first ten kills counts");
        }
        assert_eq!(p, "010");
        assert!(r.kills_met(&p));
        assert!(r.items_met(|_| 0), "1006 has no item requirement");
    }

    // ----------------------------------------------------------------------------------
    // The requirement table
    // ----------------------------------------------------------------------------------

    /// Slot order is `Quest.wz` key order, **not** template order. Quest 10113 is the case
    /// that tells them apart: its slots are mobs 18 then 17, and the client's own
    /// `demandSummary` reads `#o18# #a101131#` then `#o17# #a101132#`.
    #[test]
    fn the_table_keeps_wz_key_order_and_not_template_order() {
        let text = "# a comment\n\
                    10113\t1\tmob\t0\t18\t20\n\
                    10113\t1\tmob\t1\t17\t20\n\
                    10113\t1\titem\t0\t2020000\t1\n\
                    1006\t1\tmob\t0\t2\t10\n";
        let t = QuestRequirementTable::parse(text);
        assert_eq!(t.len(), 2);
        let q = t.get(10113).expect("10113");
        assert_eq!(q.mobs[0].template_id, 18);
        assert_eq!(q.mobs[1].template_id, 17);
        assert_eq!(q.progress_slots(), 2);
        assert_eq!(q.fresh_progress(), "000000");
        // Killing mob 17 must move the SECOND field, not the first.
        assert_eq!(apply_kill(q, "000000", 17).as_deref(), Some("000001"));
        assert_eq!(apply_kill(q, "000000", 18).as_deref(), Some("001000"));
    }

    /// A kill has to fan out: template 13 is named by six quests in this client.
    #[test]
    fn one_template_can_belong_to_several_quests() {
        let text = "2000\t1\tmob\t0\t13\t5\n\
                    2001\t1\tmob\t0\t13\t20\n\
                    2002\t1\tmob\t0\t99\t1\n";
        let t = QuestRequirementTable::parse(text);
        assert_eq!(t.quests_for_mob(13).to_vec(), vec![2000u32, 2001]);
        assert_eq!(t.quests_for_mob(99).to_vec(), vec![2002u32]);
        assert!(t.quests_for_mob(12345).is_empty());
    }

    /// Item requirements carry no progress field, and `items_met` asks the bag.
    #[test]
    fn item_requirements_are_answered_by_the_bag_and_not_by_the_string() {
        let text = "1005\t1\tmob\t0\t1\t3\n\
                    1005\t1\titem\t0\t4000000\t3\n";
        let t = QuestRequirementTable::parse(text);
        let q = t.get(1005).expect("1005");
        assert_eq!(q.progress_slots(), 1, "one field, for the mob only");
        assert_eq!(q.fresh_progress(), "000");
        assert!(!q.items_met(|_| 2));
        assert!(q.items_met(|id| if id == 4_000_000 { 3 } else { 0 }));
    }

    /// The generated table is the instrument this module is aimed at, so drift in it is
    /// caught here rather than on a client run. Skipped when the file is absent, because
    /// `gm-handbook/` is generated and gitignored.
    #[test]
    fn the_generated_requirement_file_still_says_what_this_module_assumes() {
        let path = std::path::Path::new("../../gm-handbook/questreq.txt");
        let Ok(text) = std::fs::read_to_string(path) else {
            return;
        };
        let t = QuestRequirementTable::parse(&text);
        assert!(t.len() > 100, "the file parsed to almost nothing: {}", t.len());

        let sam = t.get(1006).expect("quest 1006, Sam's Suggestion");
        assert_eq!(
            sam.mobs,
            vec![MobRequirement { slot: 0, state: 1, template_id: 2, count: 10 }]
        );
        assert!(sam.items.is_empty());

        // 10113's slots are the discriminating case for ordering; see the test above.
        let q = t.get(10113).expect("quest 10113");
        assert_eq!(
            q.mobs.iter().map(|m| m.template_id).collect::<Vec<_>>(),
            vec![18, 17]
        );

        for (quest_id, r) in t.by_quest.iter() {
            for (i, m) in r.mobs.iter().enumerate() {
                assert_eq!(m.slot, i, "quest {} has a gap in its mob slots", quest_id);
                assert_eq!(m.state, 1, "quest {} has a mob check outside state 1", quest_id);
                assert!(
                    m.count <= MAX_PROGRESS_FIELD_VALUE,
                    "quest {} wants {} kills, which will not fit in three digits",
                    quest_id,
                    m.count
                );
            }
            assert_eq!(r.fresh_progress().len(), r.mobs.len() * PROGRESS_FIELD_WIDTH);
        }
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
