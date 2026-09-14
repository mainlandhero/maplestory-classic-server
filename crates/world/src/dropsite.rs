//! **Where a mob actually is when it dies**, which is not where this server thought it was.
//!
//! `crate::footholds` answers *"how far does a drop sink or rise once it is placed"*. This
//! module answers the question one step earlier and, until now, wrongly: *"what point is it
//! placed at in the first place"*.
//!
//! # The bug, stated as a measurement rather than as a theory
//!
//! A live player: *"the mob drops from a moving mob seem to be dropping from an awkward
//! location not related to the current mob location mid-movement."*
//!
//! `session/combat.rs::on_mob_move` stores `(req.x, req.y)` - the `0x02FF` **path head** -
//! as the mob's position, and `deal_to_mob` reads it back as `died_at`. The comment above
//! that call has said since the day it was written that the head is *"the path's START, so
//! this is at most one report stale - about half a second, which is a few pixels for a
//! snail"*.
//!
//! **The first half is right and the second half is not, and it was never measured.**
//!
//! ## What was measured, 2026-09-08
//!
//! Every `0x02FF` in `previous-runs/` and `research/fixtures/`, deduplicated on
//! `(timestamp, opcode, body)` rather than on the file, because `research/fixtures/` holds
//! copies and `CLAUDE.md` records eleven pairs that are the same run and hash differently.
//! `tools/mobmove_lag.py` re-derives all of it, and it does the dedup itself rather than
//! trusting the caller:
//!
//! ```text
//!   818 948  raw 0x02FF log lines
//!   642 431  distinct events   <- 176 517 were fixture copies of a previous-runs original
//!   642 431  parsed, every one satisfying the 14 + 21*count path-length identity
//!         0  bodies that would not parse
//! ```
//!
//! **The raw count is not the stable number and must not be quoted as one.** Copying the two
//! decisive runs into `research/fixtures/` under names that say what they prove - which is
//! what `CLAUDE.md` says to do, because `previous-runs/` is a rolling buffer - took the raw
//! line count to **1 013 795** and left the distinct count at **642 431**, unchanged by a
//! single event. That is the file-vs-event rule demonstrating itself: a glob would have
//! reported 58% more evidence than exists.
//!
//! Per `(run, mob object id)`, comparing report N's **last path element** against report
//! N+1's **head**:
//!
//! ```text
//!   640 371  consecutive report pairs
//!   622 044  head(N+1) == last element of N, at element offset 1     (97.1%)
//!   141 903  head(N+1) == head(N)                                    (22.2%)
//! ```
//!
//! So the head is where the mob **was when the path began**, and the last element is where
//! it **ended up**. The distance between the two, over all 642 431 reports:
//!
//! ```text
//!   |dx|  mean 67.8   median 41   p90 108   p99 861
//!   |dy|  mean  9.9   median  0   p90   0   p99 325
//!   |dx| > 25 px (half the client's own pick-up box):  403 225   (62.8%)
//!   |dx| > 50 px                                     :  220 481   (34.3%)
//! ```
//!
//! **[L]** all of it. The median mob drop was placed 41 pixels from the corpse, and one
//! report in three by more than 50. That is the "awkward location", and it is not a few
//! pixels for a snail.
//!
//! # Three controls, because "97.1% agree" is exactly the shape of answer this repo distrusts
//!
//! `CLAUDE.md`: *a field whose meaning depends on a type byte will decode as garbage without
//! saying so*, and *enumerate before you filter*. The 21-byte element is `u8`, `i16 x`,
//! `i16 y`, then 16 bytes this module never reads, and the `x`/`y` claim is
//! [`PATH_ELEMENT_XY_AT`]. Three ways it could have been wrong, each checked:
//!
//! | control | if the claim were wrong | measured |
//! |---|---|---|
//! | is offset 1 the step's **start** rather than its end? | `elem[0]@1 == head` would be **100%** of multi-element paths | **18.9%** of 439 485 |
//! | is the last element a degenerate "stand still" step, so the match is vacuous? | `elem[last]@1 == elem[last-1]@1` would dominate | **6.4%** |
//! | does the match only hold when the mob did not move? | it would collapse on the pairs that walked | **96.5%** on the 498 468 pairs whose head **did** move, 99.2% on the 141 903 that did not |
//!
//! The second and third are the ones that matter: a rule that only agreed with itself while
//! nothing was happening would have scored well on the headline number and been useless.
//!
//! # The unit, checked the only way that can come back false
//!
//! `CLAUDE.md` puts the unit before the arithmetic, and names the failure: a number that is
//! right in a unit nobody checked. Here the check is an **identity**, not a reading of a
//! comment - the element's `(x, y)` is byte-for-byte the same pair the client itself sends
//! as the next report's head, 622 044 times. Same origin, same signedness, same pixels. A
//! different unit could not be equal to the head; it could only be proportional to it.
//!
//! # The second half of the same bug: **a path element is not 21 bytes, and a jump is 9**
//!
//! The owner, 2026-09-08, after the fix above shipped: *"Mob drop placement is still wonky,
//! particularly when a mob is jumping, the loot drops below the current platform."*
//!
//! Everything above stayed true and the walk that implemented it was wrong. It stepped the
//! path in **uniform 21-byte cells**. The client does not: `FUN_1404b2630` reads a **command
//! byte** at `0x1404b26ee` and dispatches through a 79-entry jump table into thirteen case
//! bodies of different lengths - which `net::usermove::element_len` has held, read out of the
//! image, since 2026-08-29. It was decoded for the **player's** `0x00D9` and never applied to
//! the mob's `0x02FF`, although `FUN_141d57c60` hands both to the same `FUN_1404b2000`.
//! `CLAUDE.md`'s *built is not wired*, inside one packet.
//!
//! ## The identity that could come back false, and did
//!
//! `tools/mobmove_lag.py` checks `p - start == 14 + 21 * count`. That check **cannot fail** -
//! it is arithmetic the walk itself performs. The check that can is the **residue**: a
//! `0x02FF` body ends in a fixed 30-byte tail, so `len(body) - (path start + 14 + 21*count)`
//! must be 30 for every body if the element really is uniform. Over every `0x02FF` in
//! `previous-runs/` and `research/fixtures/`, deduplicated on `(timestamp, opcode, body)`:
//!
//! ```text
//!   860 656  distinct events, 0 unparsed
//!   834 115  residue 30   (96.916%)   <- all elements 21 bytes
//!    26 052  residue 18   ( 3.027%)   <- ONE element 12 bytes shorter than assumed
//!       489  residue  6   ( 0.057%)   <- two of them
//! ```
//!
//! A 12-byte quantum, not noise. Solving for the length of each leading byte by requiring
//! residue 30 gives **`0x00` -> 21, `0x01` -> 9, `0x02` -> 9**, and then **860 656 of 860 656
//! bodies close exactly, 100.0000%**. Those three lengths are rows of
//! [`net::usermove::element_len`]'s table, derived from the client's jump table before any of
//! this was measured: `1 + 16 + 4 = 21` for `0x00`, `1 + 4 + 4 = 9` for the `0x1404b28c6`
//! case, whose commands are `0x01 0x02 0x12 0x15 0x30..0x35 0x46`. **[L]**, twice, by
//! instruments that share no code.
//!
//! ## The 2.9% residue in the measurement above was the jumps
//!
//! Re-run on the same 858 538 consecutive pairs, splitting on whether the path contains a
//! short element:
//!
//! ```text
//!                      pairs    uniform-21 last == next head      type-aware
//!   plain paths      832 062      830 827  (99.85%)             830 827  (99.85%)
//!   short paths       26 476            0  ( 0.00%)              26 248  (99.14%)
//!   TOTAL            858 538      830 827  (96.77%)             857 075  (99.83%)
//! ```
//!
//! **Zero of 26 476**, not a degraded match - the uniform walk lands 12 bytes late, inside the
//! zero-filled tail, and decodes `(0, 0)`. 26 476 of the 27 711 disagreeing pairs (95.5%) are
//! short-element paths; the ~0.14% that remain were always there and are a different question.
//!
//! And the short paths are the jumping ones, measured rather than assumed by their name -
//! vertical span of one path, over its position-carrying elements:
//!
//! ```text
//!   plain paths   median   0   p90   0   p99  10   over 100 px:  0.03%
//!   short paths   median  78   p90 142   p99 197   over 100 px: 13.81%
//! ```
//!
//! ## "Below the current platform", against the real 94 089-segment table
//!
//! Both decodes of every short-element path, run through [`crate::footholds::Footholds`] on
//! the map that run was on:
//!
//! ```text
//!   25 389  both decodes found a floor
//!   25 380  land on a DIFFERENT foothold   (100.0%)
//!   23 357  the old decode lands BELOW the right one  (92.0%), modal error 300-600 px
//!      823  the old decode finds no floor at all (3.2%); the corrected one: 7 (0.03%)
//! ```
//!
//! **The foothold snap was never at fault.** Handed the corrected position it moves the drop
//! **0 px in 81.3% of cases, 1 px in 18.6%, and never more than 2** - the mob is standing on
//! that foothold, so there is nothing to fall. Handed `(0, 0)` it does exactly what it is
//! supposed to do and drops to the floor beneath `x = 0`. The answer to *"snap upward or
//! downward"* is **neither**: the input was wrong.
//!
//! One packet says all of it, from the owner's own 2026-09-08 17:24:53 run, mob 2025 on map
//! 10001010 - see [`tests::JUMP_N`]:
//!
//! ```text
//!   element 1 is command 0x01, 9 bytes            <- the jump
//!   uniform-21 "last element"   -> (   0,    0)   -> foothold 133 at y  215
//!   type-aware last element     -> (1240, -391)   -> foothold  51 at y -391, moved 0 px
//!   the next report's head       =  (1240, -391)  <- the client agrees
//! ```
//!
//! **606 pixels below the platform the mob died on**, which is the sentence the owner wrote.
//!
//! That 606 is asserted rather than quoted, and it earned its place: the first draft of this
//! block said 524, off a tuple misread by one field in a throwaway script, and
//! `on_the_real_map_the_old_decode_dropped_the_loot_below_the_platform` failed on it. A number
//! in a doc comment is a claim; put it where something can disagree.
//!
//! # What this module does NOT do
//!
//! It does not move a drop sideways, does not roll a table, does not decide who owns a drop,
//! and does not re-implement [`crate::footholds`]. `session/combat.rs` still owns the
//! stagger, the landing and the audience walk; this only corrects the point they all start
//! from.

/// Where a movement-path element carries its own `(i16 x, i16 y)`, **after its command byte**.
///
/// **One, not zero.** Every element begins with the command `u8` that decides its length;
/// a command that carries a position writes it as its first two `u16`
/// (`net::usermove::element_carries_position`, whose doc block reads that off the case
/// bodies). Dumped from a real body (`previous-runs/world-20260907-233531.log`, mob 2020,
/// move 2357), all three elements command `0x00`:
///
/// ```text
///  path head  00000000 a402 bbfe 7d00 0000 0300      x = 676, y = -325, 3 elements
///  elem 0     00 d002 bbfe 7d000000 2a00000000000000 02 5c010000     (720, -325)
///  elem 1     00 2a03 bbfe 7d000000 2b00000000000000 02 d0020000     (810, -325)
///  elem 2     00 2c03 bbfe 7d000000 2c00000000000000 02 0c000000     (812, -325)
/// ```
///
/// and the very next `0x02FF` for mob 2020 opens its head with `2c03 bbfe` - **(812, -325)**.
///
/// Reading this at offset 0 instead would give `(11264, -17661)` for that element: a
/// plausible-looking pair of `i16`s, off the map, and nothing in the format would complain.
/// That is why the offset is scored against 640 371 real pairs rather than counted off a
/// hex dump.
pub const PATH_ELEMENT_XY_AT: usize = 1;

/// Where the path in a `0x02FF` body **ends** - the mob's position now.
///
/// The **last element that carries a position of its own**, which is the rule
/// `research/user-move.md` §3.1 read off the client: a command that does not carry one
/// re-stores the previous element's pair (`0x1404b28c9` and four siblings), so the position
/// after the path is the last one the wire actually supplied.
///
/// # This walks with the client's own length table, and that is the fix
///
/// Stepping in uniform [`net::mobmove::MOB_PATH_ELEMENT_LEN`] cells is wrong for **3.04%** of
/// real bodies and *silently* wrong - see the module docs. The lengths come from
/// [`net::usermove::element_len`], which is the 79-entry jump table at `0x1404b2f24`, and the
/// same `FUN_1404b2000` decodes both packets.
///
/// `None` when the path carries no elements (the mob reported a position and no steps), when
/// the count is negative, when the block is too short to hold the elements it claims, and
/// when no element in it carried a position at all. In every one of those the caller should
/// keep the head, which is what [`reported_position`] does.
///
/// # It tolerates a `path` that is LONGER than the elements need, deliberately
///
/// `net::mobmove::parse_mob_move` slices [`net::mobmove::MobMoveRequest::path`] as
/// `MOB_PATH_HEAD_LEN + count * MOB_PATH_ELEMENT_LEN`, so for a path containing a short
/// element that slice runs 12 or 24 bytes into the body's tail. Walking from the **front**
/// with the real lengths reaches the correct last element either way, so this function is
/// right both before and after that slice is corrected. It is the trailing bound that is
/// checked - the elements must *fit* - not an exact equality that would break on the fix.
pub fn path_end(path: &[u8], element_count: i16) -> Option<(i16, i16)> {
    let count = usize::try_from(element_count).ok()?;
    if count == 0 {
        return None;
    }
    let mut at = net::mobmove::MOB_PATH_HEAD_LEN;
    let mut last = None;
    for _ in 0..count {
        // `?`, not a `break`: a block that cannot hold the elements it claims is malformed,
        // and the documented answer to that is the head rather than a partial decode.
        let command = *path.get(at)?;
        if net::usermove::element_carries_position(command) {
            let xy = path.get(at + PATH_ELEMENT_XY_AT..at + PATH_ELEMENT_XY_AT + 4)?;
            last = Some((
                i16::from_le_bytes([xy[0], xy[1]]),
                i16::from_le_bytes([xy[2], xy[3]]),
            ));
        }
        // Every one of the 256 command bytes has a length - anything past 0x4e falls into the
        // five-byte common tail - so this always advances and the loop is bounded by `count`.
        at = at.checked_add(net::usermove::element_len(command))?;
    }
    // The last element's own bytes must have been inside the block, not merely its command.
    if at > path.len() {
        return None;
    }
    last
}

/// **Where the client says the mob is**, out of one `0x02FF`.
///
/// The end of the reported path, falling back to the path head. This is the value that
/// belongs in `Fields::note_position_from`; the head alone is one whole path stale, which is
/// this module's whole subject.
///
/// It **never fails**. A body that parsed at all has a head, and the head is the old
/// behaviour - so the worst case here is exactly what shipped before, never a refusal and
/// never a guess. `CLAUDE.md`'s always-answer rule is about the wire and this is not a wire
/// path, but the same reasoning applies: a movement handler that declined would freeze a mob.
pub fn reported_position(req: &net::mobmove::MobMoveRequest) -> (i16, i16) {
    path_end(&req.path, req.element_count).unwrap_or((req.x, req.y))
}

/// Where the foothold id sits inside a **21- or 23-byte** element: the fifth `u16` after the
/// command byte. Those are the two cases at `1404b2755` that read 8 or 9 `u16`s - `x, y, vx,
/// vy, fh, ...` - and they are the only element shapes that carry one; the shorter cases
/// inherit position and say nothing about the floor. **[L]** for the element table
/// (`research/user-move.md` §3), **[D]** for `fh` being the fifth, which is the v83 layout
/// and is checked on screen by whether a joining client's mobs stop snapping.
pub const PATH_ELEMENT_FH_AT: usize = 9;

/// **The foothold under the end of the reported path**, or `None` if the last element that
/// carried a position was one of the short shapes, which say nothing about the floor.
///
/// The owner, 2026-09-14: *"the second client also sees mobs that the previous client has control
/// over snap to their position on their screen, which is jarring."* `Fields::as_seen` was
/// already sending the current `x, y` to a joining client - but with the **spawn point's**
/// foothold, so the client placed the mob at the reported position and then dropped it onto
/// the floor it was told about, wherever that was. The end of the path names the floor the
/// mob is actually standing on, in the same element as its position.
pub fn path_end_foothold(path: &[u8], element_count: i16) -> Option<i16> {
    let count = usize::try_from(element_count).ok()?;
    if count == 0 {
        return None;
    }
    let mut at = net::mobmove::MOB_PATH_HEAD_LEN;
    let mut last = None;
    for _ in 0..count {
        let command = *path.get(at)?;
        let len = net::usermove::element_len(command);
        if net::usermove::element_carries_position(command) {
            // Only the eight/nine-u16 shapes carry a foothold; a shorter positioned element
            // (2 x u16, 3 x u16...) has no fifth u16 and must not be read as if it had.
            last = if len >= net::mobmove::MOB_PATH_ELEMENT_LEN {
                path.get(at + PATH_ELEMENT_FH_AT..at + PATH_ELEMENT_FH_AT + 2)
                    .map(|b| i16::from_le_bytes([b[0], b[1]]))
            } else {
                None
            };
        }
        at = at.checked_add(len)?;
    }
    if at > path.len() {
        return None;
    }
    last
}

/// [`path_end_foothold`] for one `0x02FF`; `None` when the path says nothing about the floor.
pub fn reported_foothold(req: &net::mobmove::MobMoveRequest) -> Option<i16> {
    path_end_foothold(&req.path, req.element_count)
}

/// **Which point a kill's drops fall from**, in the order the answers are worth trusting.
///
/// 1. `reported` - where the client last said the mob was. After [`reported_position`] this
///    is the end of its last path rather than the start.
/// 2. `spawn` - the spawn point out of `Map.wz`. A mob that has never reported is standing
///    exactly there, and `gm-handbook/mobs.txt` names the very foothold it stands on. This
///    is strictly better than the third answer and was not being used: `Fields::mob_position`
///    returns `None` for such a mob although `LiveMob::spawn` is right beside the field it
///    reads, which is why [`crate::fields::Fields::mob_site`] exists.
/// 3. `player` - the killer's own last known position. The owner: *"they should drop from the
///    killed mob's position, not from the player character position"*, so this is a last
///    resort and not a default.
///
/// `None` only when all three are absent, and the caller then drops nothing and says so -
/// an item placed where the player cannot reach is indistinguishable on screen from no item
/// at all.
pub fn death_site(
    reported: Option<(i16, i16)>,
    spawn: Option<(i16, i16)>,
    player: Option<(i16, i16)>,
) -> Option<(i16, i16)> {
    reported.or(spawn).or(player)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two consecutive `0x02FF` bodies for **mob 2020** out of
    /// `previous-runs/world-20260907-233531.log`, move ids 2357 and 2358.
    ///
    /// The mob walks 136 px. The head of the first says 676; it finishes at 812, which is
    /// what the second body's head then reports. A drop placed on the head lands 136 px
    /// behind the corpse.
    const WALK_N: &str = "e4070000350900ff000000000000000000000000000000000001000000ccddff\
00ccddff005087d93c000000000200000000a402bbfe7d000000030000d002bbfe7d0000002a0000000000000002\
5c0100002a03bbfe7d0000002b0000000000000002d00200002c03bbfe7d0000002c00000000000000020c000000\
000faa8ebef35a46172b000000b8b9160b000000000300000000010000";
    const WALK_N1: &str = "e4070000360900ff000000000000000000000000000000000001000000ccddff\
00ccddff005087d93c0000000002000000002c03bbfe7d0000000200008403bbfe7d0000002c0000000000000002\
c4020000b303bbfe7d00000025000000000000000274010000000faa8ebe8e489f2e25000000b8b9160b00000000\
0300000000010000";

    /// Two consecutive bodies for **mob 2025** out of
    /// `previous-runs/world-20260907-205055.log`, a **twelve**-element path in which the mob
    /// falls off something: head `(13, -505)`, end `(139, -385)`.
    ///
    /// This is the fixture that makes the bug more than cosmetic. 120 px of the error is
    /// **vertical**, and the client's pick-up box reaches 50 px above the player's feet and
    /// **10 below** (`research/item-drop.md`, read off the listing). An item placed 120 px
    /// above the floor the mob landed on cannot be collected however long anyone stands
    /// under it, and on screen that is identical to the mob having dropped nothing.
    const FALL_N: &str = "e9070000220d00ff000000000000000000000000000000000001000000ccddff\
00ccddff005087d93c0000000002000000000d0007fe7d0000000c0000270007fe7d000000ae0000000000000002\
cd0000002f0007fe7d00000000000000000000000641000000510050fe7d001c020000000000000000060e010000\
59007cfe3e0000000000000000000000064900000063007cfe7d0000003900000000000000026b00000068007cfe\
7c000f003900000000000000022c0000006e007dfe7c000f003a00000000000000022e00000070007dfe7300cfff\
3a00000000000000021200000077007afe7d00f8ff3b00000000000000023c000000820079fe7c00f8ff3c000000\
000000000266000000850079fe5e0052003c0000000000000002160000008b007ffe5e0052003d00000000000000\
0244000000000faa8ebee8e1d4693c000000b8b9160b000000000300000000010000";
    /// **And `FALL_N1` is itself a jump**, which nobody noticed when it was added this
    /// morning: element 2 of its eight is a `0x01`. It was named for the fall in `FALL_N`,
    /// and the uniform walk read its end as `(0, 0)` from the day it landed in this file.
    /// `the_two_walks_agree_on_an_ordinary_path_and_the_uniform_one_is_wrong_on_a_jump`
    /// enumerates the types rather than trusting the names.
    const FALL_N1: &str = "e9070000230d00ff000000000000000000000000000000000001000000ccddff\
00ccddff005087d93c0000000002000000008b007ffe5e0052000800008b007dfe5e0052003d0000000000000002\
5a0000008b007dfe5e00520000000000000000000600000001b300d5fd0600000000bc0030feb300f1ff00000000\
00000000060e010000fc00abfeb3009e02000000000000000006680100000001bbfe590000000000000000000000\
06180000000e01bbfe770000002d00000000000000028d0000002601bbfe7d0000002e0000000000000002c30000\
00000faa8ebe44ce727a2e000000b8b9160b000000000300000000010000";

    /// **The jump, from the owner's own run on the day they reported it.**
    ///
    /// `previous-runs/world-20260908-135813.log`, `17:24:53.507`, mob **2025** on map
    /// **10001010**, move ids 4736 and 4737. Seven elements, and element 1 is command
    /// **`0x01`** - nine bytes, `net::usermove::element_len`'s `0x1404b28c6` case, carrying no
    /// position of its own.
    ///
    /// ```text
    ///   elem 0  cmd 0x00  21 B   (1147, -566)   the head, restated
    ///   elem 1  cmd 0x01   9 B   -              the jump
    ///   elem 2  cmd 0x00  21 B   (1181, -643)   up
    ///   elem 3  cmd 0x00  21 B   (1248, -400)
    ///   elem 4  cmd 0x00  21 B   (1249, -391)
    ///   elem 5  cmd 0x00  21 B   (1254, -391)
    ///   elem 6  cmd 0x00  21 B   (1240, -391)   and the next report's head is exactly this
    /// ```
    ///
    /// The uniform-21 walk puts the "last element" 12 bytes late, in the zero-filled tail, and
    /// reads **`(0, 0)`**. That is a coordinate, it is on the map, and nothing in the format
    /// complains - the failure this repo keeps paying for.
    const JUMP_N: &str = concat!(
        "e9070000801200ff000000000000000000000000000000000001000000ccddff00ccddff005087d93c000000",
        "0002000000007b04cafd7d0000000700007b04cafd7d000000000000000000000006000000017d00d5fd0600",
        "0000009d047dfd7d00f1ff0000000000000000060e010000e00470fe7d009e020000000000000000061c0200",
        "00e10479fe3e0000000000000000000000060d000000e60479fe3e0000003300000000000000024d000000d8",
        "0479fe83ff0000330000000000000003b4000000000faa8ebe7592fba433000000b8b9160b00000000030000",
        "0000010000",
    );
    const JUMP_N1: &str = concat!(
        "e9070000811200ff000000000000000000000000000000000001000000ccddff00ccddff005087d93c000000",
        "000200000000d80479fe83ff0000010000510479fe83ff000033000000000000000338040000000faa8ebe75",
        "92fba433000000b8b9160b000000000300000000010000",
    );

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    fn req(s: &str) -> net::mobmove::MobMoveRequest {
        net::mobmove::parse_mob_move(&hex(s)).expect("a real captured body must parse")
    }

    // ------------------------------------------------------------------------------
    // The measurement, as a test that can come back false
    // ------------------------------------------------------------------------------

    /// **The bug and the fix in one assertion, on real bytes.**
    ///
    /// Report N's head is where the mob started; report N+1's head is where it finished. If
    /// the two were the same number this test would be vacuous, so that is asserted first.
    #[test]
    fn the_path_head_is_where_the_mob_started_and_the_last_element_is_where_it_arrived() {
        let a = req(WALK_N);
        let b = req(WALK_N1);
        assert_eq!(a.object_id, b.object_id, "the same mob, or the pair proves nothing");
        assert_eq!(a.move_id + 1, b.move_id, "consecutive reports, N then N+1");

        assert_eq!((a.x, a.y), (676, -325), "the head: where the walk began");
        assert_eq!((b.x, b.y), (812, -325), "the next head: where it actually ended up");
        assert_ne!((a.x, a.y), (b.x, b.y), "the mob really moved, or this test is vacuous");

        assert_eq!(path_end(&a.path, a.element_count), Some((812, -325)));
        assert_eq!(reported_position(&a), (b.x, b.y), "one report ahead of the old behaviour");

        let stale = (a.x, a.y);
        let real = reported_position(&a);
        assert_eq!((real.0 - stale.0).abs(), 136, "136 px is what the old code got wrong by");
    }

    /// **The floor travels with the position.** The owner, 2026-09-14: a joining client saw the
    /// other client's mobs *"snap to their position on their screen, which is jarring"* -
    /// `Fields::as_seen` sent the reported `(x, y)` with the SPAWN POINT's foothold, so the
    /// client placed the mob and then dropped it onto the wrong floor.
    ///
    /// On the same captured pair: the last element of `WALK_N` names foothold 44 under
    /// `(812, -325)`, and `WALK_N1`'s head - where the client says the walk continues from -
    /// begins on the same floor. A path whose last positioned element is a short shape gives
    /// `None`, never a garbage fifth `u16`.
    #[test]
    fn the_end_of_the_path_names_the_floor_under_it() {
        let a = req(WALK_N);
        let b = req(WALK_N1);
        assert_eq!(path_end_foothold(&a.path, a.element_count), Some(44), "the floor under (812, -325)");
        assert_eq!(reported_foothold(&a), Some(44));
        // The next report starts on that floor: its first element is the same shape and the
        // same fifth u16. Read it the same way, off the first element rather than the last.
        let first = &b.path[net::mobmove::MOB_PATH_HEAD_LEN..];
        assert!(net::usermove::element_len(first[0]) >= net::mobmove::MOB_PATH_ELEMENT_LEN);
        let fh = i16::from_le_bytes([first[PATH_ELEMENT_FH_AT], first[PATH_ELEMENT_FH_AT + 1]]);
        assert_eq!(fh, 44, "the walk that follows begins on the floor the last one ended on");

        // A path that ends in a short positioned element (a jump, 9 bytes) says nothing
        // about the floor and must say so.
        let f = req(FALL_N1);
        let mut at = net::mobmove::MOB_PATH_HEAD_LEN;
        let mut last_len = 0;
        for _ in 0..usize::try_from(f.element_count).unwrap() {
            let c = f.path[at];
            if net::usermove::element_carries_position(c) {
                last_len = net::usermove::element_len(c);
            }
            at += net::usermove::element_len(c);
        }
        if last_len < net::mobmove::MOB_PATH_ELEMENT_LEN {
            assert_eq!(path_end_foothold(&f.path, f.element_count), None);
        } else {
            assert!(path_end_foothold(&f.path, f.element_count).is_some());
        }
    }

    /// **The vertical half, which is the half that makes a drop uncollectable.**
    ///
    /// The client's pick-up box reaches 10 px below the player's feet. This mob ends its
    /// path 120 px below where the head says it is, so a drop on the head is 120 px up in
    /// the air - twelve times outside the box, in the direction the box is smallest.
    #[test]
    fn a_falling_mob_is_misplaced_vertically_by_more_than_the_client_can_reach() {
        let a = req(FALL_N);
        let b = req(FALL_N1);
        assert_eq!(a.element_count, 12, "a twelve-step path, not a single hop");
        assert_eq!((a.x, a.y), (13, -505), "the head");
        assert_eq!(reported_position(&a), (139, -385), "where it really is");
        assert_eq!(reported_position(&a), (b.x, b.y), "and the client agrees, next report");

        // A smaller y is HIGHER UP - `crate::footholds` measures that over all 426 maps.
        let (_, stale_y) = (a.x, a.y);
        let (_, real_y) = reported_position(&a);
        assert!(stale_y < real_y, "the stale point is ABOVE the mob's real feet");
        assert_eq!(real_y - stale_y, 120);
        const PICK_UP_BOX_BELOW_FEET: i16 = 10;
        assert!(
            real_y - stale_y > PICK_UP_BOX_BELOW_FEET,
            "a drop this far above the floor is outside the client's own pick-up box"
        );
    }

    // ------------------------------------------------------------------------------
    // The jump: the element is not 21 bytes, and that is the whole of the second bug
    // ------------------------------------------------------------------------------

    /// **The command byte decides the length, and a `0x01` is nine bytes.**
    ///
    /// Walked with `net::usermove::element_len`, this real seven-element path closes on the
    /// body's 30-byte tail. Walked in uniform 21-byte cells it overruns by exactly 12, which
    /// is the whole mechanism.
    #[test]
    fn a_jump_element_is_nine_bytes_and_the_uniform_walk_overruns_by_twelve() {
        let body = hex(JUMP_N);
        let r = req(JUMP_N);
        assert_eq!(r.object_id, 2025);
        assert_eq!(r.element_count, 7);

        // The path starts where `parse_mob_move` put it: the body minus the path block minus
        // the tail. Locate it by the head's own bytes rather than by a magic number.
        let at = body
            .windows(r.path.len())
            .position(|w| w == r.path.as_slice())
            .expect("the parser's path slice is a subslice of the body");

        let mut off = at + net::mobmove::MOB_PATH_HEAD_LEN;
        let mut kinds = Vec::new();
        for _ in 0..r.element_count {
            let command = body[off];
            kinds.push((command, net::usermove::element_len(command)));
            off += net::usermove::element_len(command);
        }
        assert_eq!(
            kinds,
            vec![(0x00, 21), (0x01, 9), (0x00, 21), (0x00, 21), (0x00, 21), (0x00, 21), (0x00, 21)],
            "element 1 is the jump: command 0x01, the 0x1404b28c6 case, nine bytes"
        );
        assert!(!net::usermove::element_carries_position(0x01), "and it carries no position");

        // The identity that can come back false. Every 0x02FF ends in a 30-byte tail; over
        // 860 656 archived bodies this is 30 for all of them with the type-aware walk, and
        // 30 / 18 / 6 with the uniform one.
        const TAIL_LEN: usize = 30;
        assert_eq!(body.len() - off, TAIL_LEN, "the type-aware walk closes on the tail");
        let uniform_end = at
            + net::mobmove::MOB_PATH_HEAD_LEN
            + 7 * net::mobmove::MOB_PATH_ELEMENT_LEN;
        assert_eq!(
            body.len() - uniform_end,
            TAIL_LEN - 12,
            "the uniform walk eats 12 bytes of the tail - the 12-byte quantum in the archive"
        );
    }

    /// **The bug the owner saw, and the fix, on that packet.**
    ///
    /// The uniform-21 "last element" lands in the zero-filled tail and reads `(0, 0)`, a
    /// perfectly plausible coordinate that no format check rejects. The type-aware walk reads
    /// `(1240, -391)`, which is what the client itself puts in the next report's head.
    #[test]
    fn a_jumping_mobs_position_was_decoded_as_zero_zero_and_is_now_the_real_pixel() {
        let a = req(JUMP_N);
        let b = req(JUMP_N1);
        assert_eq!(a.object_id, b.object_id, "the same mob, or the pair proves nothing");
        assert_eq!(a.move_id + 1, b.move_id, "consecutive reports, N then N+1");

        assert_eq!((a.x, a.y), (1147, -566), "the head: where the jump began");
        assert_eq!(reported_position(&a), (1240, -391), "where the mob actually is");
        assert_eq!(reported_position(&a), (b.x, b.y), "and the client agrees, next report");

        // What shipped: the last 21-byte cell, addressed from the front exactly as before.
        let stale_at = net::mobmove::MOB_PATH_HEAD_LEN
            + (a.element_count as usize - 1) * net::mobmove::MOB_PATH_ELEMENT_LEN;
        let cell = &a.path[stale_at + PATH_ELEMENT_XY_AT..stale_at + PATH_ELEMENT_XY_AT + 4];
        let uniform = (
            i16::from_le_bytes([cell[0], cell[1]]),
            i16::from_le_bytes([cell[2], cell[3]]),
        );
        assert_eq!(uniform, (0, 0), "the old walk read the tail and called it a position");
        assert_ne!(uniform, reported_position(&a), "or this test is vacuous");
    }

    /// **`parse_mob_move` over-slices the path, and `path_end` must not care.**
    ///
    /// The parser takes `14 + 21 * count` bytes, so for this body it takes **161** where the
    /// elements occupy **149**. That is a real (small) fault of its own - see the report - and
    /// it is why [`path_end`] walks from the front and checks only that the elements *fit*.
    /// This asserts the inequality the walk depends on, not today's exact numbers, so
    /// correcting the parser cannot break it.
    #[test]
    fn the_parsers_path_slice_is_at_least_as_long_as_the_elements_it_holds() {
        let r = req(JUMP_N);
        let mut need = net::mobmove::MOB_PATH_HEAD_LEN;
        for _ in 0..r.element_count {
            need += net::usermove::element_len(r.path[need]);
        }
        assert_eq!(need, 149, "the elements really occupy 149 bytes");
        assert!(
            r.path.len() >= need,
            "path_end can only walk a slice that holds its elements: {} < {need}",
            r.path.len()
        );
        // Today the parser hands over 161 (`14 + 21 * 7`); with the patch in the 2026-09-08
        // report it hands over 149. Both are accepted **on purpose** - asserting one of them
        // would make this test a tripwire on the parser rather than a check on `path_end`.
        let over_sliced = net::mobmove::MOB_PATH_HEAD_LEN + 7 * net::mobmove::MOB_PATH_ELEMENT_LEN;
        assert!(
            r.path.len() == need || r.path.len() == over_sliced,
            "the slice is neither the elements ({need}) nor the uniform overrun \
             ({over_sliced}): {}",
            r.path.len()
        );
        assert_eq!(path_end(&r.path, r.element_count), Some((1240, -391)));
        // And on a slice cut down to exactly the elements, which is what the fixed parser
        // would hand over.
        assert_eq!(path_end(&r.path[..need], r.element_count), Some((1240, -391)));
    }

    /// **Where the two decodes actually put the drop, on the real map 10001010.**
    ///
    /// `gm-handbook/` is generated and gitignored, so this skips itself on a fresh clone.
    ///
    /// This is the assertion that carries the owner's sentence. The snap is not asked to behave
    /// differently; it is asked the right question and answers `moved = 0`, because the mob
    /// was standing on that foothold.
    #[test]
    fn on_the_real_map_the_old_decode_dropped_the_loot_below_the_platform() {
        let fh_path = std::path::Path::new("../../gm-handbook/footholds.txt");
        if !fh_path.exists() {
            eprintln!("skipped: gm-handbook/ is generated and gitignored");
            return;
        }
        const MAP: u32 = 10_001_010; // the map that run was on, from world.log's SetField line
        let t = crate::footholds::Footholds::load(fh_path);
        let real = reported_position(&req(JUMP_N));
        assert_eq!(real, (1240, -391));

        let good = t.landing(MAP, real.0, real.1).expect("the mob was standing on a floor");
        assert_eq!(good.y, -391, "the platform the mob died on");
        assert_eq!(good.moved, 0, "nothing to fall - the snap is not the bug");

        let bad = t.landing(MAP, 0, 0).expect("something is under x = 0 too");
        assert_ne!(bad.foothold, good.foothold, "a different platform entirely");
        assert!(
            bad.y - good.y > 500,
            "the old decode landed {} px below the right platform, not {}",
            bad.y - good.y,
            "above it"
        );
        assert_eq!(bad.y - good.y, 606, "606 px below - the owner's 'below the current platform'");
        assert_eq!((bad.foothold, bad.y), (133, 215), "the floor beneath x = 0 on that map");
    }

    /// Every element of a real path decodes to a coordinate on the map, not to a wild
    /// `i16` pair. The off-by-one that reads offset 0 gives `(11264, -17661)` for one of
    /// these and no format check would catch it.
    ///
    /// The walk is type-aware, so the jump fixture is in scope. Note what the uniform walk
    /// does to it and why no bounds check would have caught it: after the 9-byte element
    /// every cell is 12 bytes late, which lands in the zero middle of the *next* element, so
    /// elements 2 through 6 all read `(0, 0)` - inside this test's box, on the map, and
    /// wrong. **A silently plausible decode, not a wild one.**
    #[test]
    fn every_element_of_a_real_path_decodes_to_a_sane_coordinate_typed() {
        for hex_body in [WALK_N, WALK_N1, FALL_N, FALL_N1, JUMP_N, JUMP_N1] {
            let r = req(hex_body);
            let mut at = net::mobmove::MOB_PATH_HEAD_LEN;
            let mut carriers = 0;
            for i in 0..r.element_count {
                let command = r.path[at];
                if net::usermove::element_carries_position(command) {
                    carriers += 1;
                    let x = i16::from_le_bytes([r.path[at + 1], r.path[at + 2]]);
                    let y = i16::from_le_bytes([r.path[at + 3], r.path[at + 4]]);
                    // Every coordinate in gm-handbook/footholds.txt lies in x -5570..6338,
                    // y -4187..3900. A decode off by one byte leaves that box immediately.
                    assert!((-6000..7000).contains(&x), "element {i} x = {x} is off the map");
                    assert!((-5000..4500).contains(&y), "element {i} y = {y} is off the map");
                }
                at += net::usermove::element_len(command);
            }
            assert!(carriers > 0, "a path with no position at all would be a new case");
            assert!(at <= r.path.len(), "the elements fit inside the parser's slice");
        }
    }

    /// **The two walks must agree on an all-ordinary path and disagree on a jumping one**,
    /// and which fixture is which is *measured here* rather than assumed from its name.
    ///
    /// # `FALL_N1` was a jump the whole time
    ///
    /// This test was first written as "these four plain fixtures" and it failed, because
    /// `FALL_N1` carries a `0x01` at element 2. It was added to this file this morning, named
    /// for what its author was looking at - the fall in `FALL_N` - and nobody enumerated its
    /// element types. `CLAUDE.md`: *a fixture's name says what its author was looking at, not
    /// everything the file contains.* So the partition below is computed, and both buckets are
    /// asserted non-empty so it can never quietly become one-sided.
    ///
    /// This also replaces `every_element_of_a_real_path_decodes_to_a_sane_coordinate`, which
    /// asserted `path.len() == 14 + 21 * count` as "the path length identity". That equality
    /// is a property of `parse_mob_move`'s slice, **not** of the format, and stating it as a
    /// law is how the uniform walk survived.
    #[test]
    fn the_two_walks_agree_on_an_ordinary_path_and_the_uniform_one_is_wrong_on_a_jump() {
        let (mut plain, mut jumping) = (0, 0);
        for hex_body in [WALK_N, WALK_N1, FALL_N, FALL_N1, JUMP_N, JUMP_N1] {
            let r = req(hex_body);
            let n = usize::try_from(r.element_count).unwrap();

            let mut at = net::mobmove::MOB_PATH_HEAD_LEN;
            let mut on_the_uniform_grid = true;
            for i in 0..n {
                on_the_uniform_grid &= at
                    == net::mobmove::MOB_PATH_HEAD_LEN + i * net::mobmove::MOB_PATH_ELEMENT_LEN;
                at += net::usermove::element_len(r.path[at]);
            }

            let last = net::mobmove::MOB_PATH_HEAD_LEN
                + (n - 1) * net::mobmove::MOB_PATH_ELEMENT_LEN
                + PATH_ELEMENT_XY_AT;
            let uniform = Some((
                i16::from_le_bytes([r.path[last], r.path[last + 1]]),
                i16::from_le_bytes([r.path[last + 2], r.path[last + 3]]),
            ));
            if on_the_uniform_grid {
                plain += 1;
                assert_eq!(
                    path_end(&r.path, r.element_count),
                    uniform,
                    "an all-ordinary path is a 21-byte ladder and both walks must land on it"
                );
            } else {
                jumping += 1;
                assert_ne!(
                    path_end(&r.path, r.element_count),
                    uniform,
                    "a short element shifts the grid, so a disagreement is the whole point"
                );
            }
        }
        assert!(plain >= 3, "the ordinary case must be covered: {plain}");
        assert_eq!(jumping, 2, "FALL_N1 and JUMP_N both carry a 0x01 - enumerated, not assumed");
    }

    // ------------------------------------------------------------------------------
    // The edges, none of which may panic or refuse
    // ------------------------------------------------------------------------------

    /// A report with no steps keeps the head. This is the "mob stood still" case and it is
    /// the one where the old behaviour was already right.
    #[test]
    fn a_path_with_no_elements_keeps_the_head() {
        let mut r = req(WALK_N);
        r.element_count = 0;
        r.path.truncate(net::mobmove::MOB_PATH_HEAD_LEN);
        assert_eq!(path_end(&r.path, r.element_count), None);
        assert_eq!(reported_position(&r), (676, -325), "the head, unchanged");
    }

    /// A negative count is the client's own `MOVSX / JLE` bail. `parse_mob_move` already
    /// refuses one; this function must not be the second place that has to be told.
    #[test]
    fn a_negative_element_count_claims_nothing() {
        let r = req(WALK_N);
        assert_eq!(path_end(&r.path, -1), None);
        assert_eq!(path_end(&r.path, i16::MIN), None);
    }

    /// A count that runs off the end of the block yields `None` rather than reading past it
    /// or panicking, and `reported_position` then falls back to what shipped before.
    #[test]
    fn a_count_longer_than_the_block_falls_back_instead_of_reading_past_it() {
        let mut r = req(WALK_N);
        assert_eq!(path_end(&r.path, 4), None, "one more element than the block holds");
        assert_eq!(path_end(&r.path, i16::MAX), None, "and no overflow on the multiply");
        r.element_count = i16::MAX;
        assert_eq!(reported_position(&r), (676, -325), "the head, never a panic");
        // Every truncation of a real path, including inside an element and inside the head.
        for cut in 0..r.path.len() {
            let _ = path_end(&r.path[..cut], 3);
        }
        assert_eq!(path_end(&[], 1), None);
    }

    /// The three-element walk read at every count from 1 to 3 gives that element's own end,
    /// so the function is addressing the grid rather than always landing on the last bytes
    /// of the slice.
    #[test]
    fn each_element_is_addressed_from_the_front_of_the_block() {
        let r = req(WALK_N);
        assert_eq!(path_end(&r.path, 1), Some((720, -325)));
        assert_eq!(path_end(&r.path, 2), Some((810, -325)));
        assert_eq!(path_end(&r.path, 3), Some((812, -325)));
    }

    // ------------------------------------------------------------------------------
    // Which point a kill drops from
    // ------------------------------------------------------------------------------

    /// The precedence, all four cases, because the middle one is new: a mob that never
    /// reported used to fall through to the **player's** feet although its spawn point was
    /// known - which is the same "not related to the mob location" complaint in a quieter
    /// form.
    #[test]
    fn a_kill_drops_from_the_mob_then_its_spawn_point_then_the_player() {
        let mob = Some((500, 395));
        let spawn = Some((300, 395));
        let player = Some((1000, 395));
        assert_eq!(death_site(mob, spawn, player), mob, "the live position wins");
        assert_eq!(death_site(None, spawn, player), spawn, "not the player: the owner's rule");
        assert_eq!(death_site(None, None, player), player, "a last resort, not a default");
        assert_eq!(death_site(None, None, None), None, "and then nothing is dropped");
    }

    // ------------------------------------------------------------------------------
    // The whole capture archive, against an identity that can come back false
    // ------------------------------------------------------------------------------

    /// **Every `0x02FF` this repository holds, walked with the element table.**
    ///
    /// # Why this and not the length identity
    ///
    /// `tools/mobmove_lag.py` checked `p - start == 14 + 21 * count` and called it "the
    /// identity that makes the walk above more than arithmetic that happened to fit". It is
    /// exactly that arithmetic: the walk skips `21 * count` and then asserts it skipped
    /// `21 * count`. It **cannot** fail, which is why it never did while 3% of bodies were
    /// being decoded as `(0, 0)`.
    ///
    /// The check here is the **residue**. A `0x02FF` body is head, path, then a fixed 30-byte
    /// tail, so `body.len() - (path start + sum of element lengths)` must be 30 for every
    /// body. Nothing in the walk guarantees that, and under the uniform-21 assumption it comes
    /// back 18 or 6 for 26 541 of them.
    ///
    /// Both halves are asserted, so the test is never vacuous:
    ///
    /// * the type-aware walk closes on 30 for **every** body, and
    /// * the uniform-21 walk does **not**, for a number greater than zero.
    ///
    /// `previous-runs/` and `research/fixtures/` are gitignored, so this skips itself on a
    /// fresh clone rather than failing it. It deduplicates on `(timestamp, body)` because
    /// `research/fixtures/` holds copies - `CLAUDE.md`'s file-vs-event rule.
    #[test]
    fn every_captured_mob_path_closes_on_the_thirty_byte_tail() {
        /// Head + path + this. Measured, not read off a comment: it is what the residue is
        /// for the 96.9% of bodies whose elements really are all 21 bytes.
        const TAIL_LEN: usize = 30;
        const BODY_MARK: &str = " byte body ";

        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("crates/world sits two levels under the repo root")
            .to_path_buf();

        let mut files = Vec::new();
        for (dir, needs_prefix) in [
            (root.join("previous-runs"), true),
            (root.join("research").join("fixtures"), false),
        ] {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                let wanted = name.ends_with(".log")
                    && if needs_prefix { name.starts_with("world") } else { name.contains("world") };
                if wanted {
                    files.push(entry.path());
                }
            }
        }
        files.sort();
        if files.is_empty() {
            eprintln!("skipped: previous-runs/ and research/fixtures/ are gitignored");
            return;
        }

        let mut seen = std::collections::HashSet::new();
        let (mut distinct, mut unparsed, mut typed_closed, mut uniform_closed) = (0, 0, 0, 0);
        let mut bad = Vec::new();
        for f in &files {
            let Ok(text) = std::fs::read_to_string(f) else { continue };
            for line in text.lines() {
                let mut it = line.split_whitespace();
                let Some(stamp) = it.next() else { continue };
                if it.next() != Some("<-") {
                    continue; // `->` is server to client; a mob move is never that
                }
                if it.next().map(|o| o.trim_end_matches(',')) != Some("0x02FF") {
                    continue;
                }
                let Some(at) = line.find(BODY_MARK) else { continue };
                let hex_body = line[at + BODY_MARK.len()..].trim_end();
                if hex_body.is_empty() || !hex_body.len().is_multiple_of(2) {
                    continue;
                }
                if !seen.insert((stamp.to_string(), hex_body.to_string())) {
                    continue;
                }
                distinct += 1;
                let Some(body) = (0..hex_body.len() / 2)
                    .map(|i| u8::from_str_radix(&hex_body[i * 2..i * 2 + 2], 16).ok())
                    .collect::<Option<Vec<u8>>>()
                else {
                    unparsed += 1;
                    continue;
                };
                let Some(r) = net::mobmove::parse_mob_move(&body) else {
                    unparsed += 1;
                    continue;
                };
                // Where the parser's path block starts inside the body.
                let Some(start) = body.windows(r.path.len()).position(|w| w == r.path.as_slice())
                else {
                    unparsed += 1;
                    continue;
                };

                let mut at = start + net::mobmove::MOB_PATH_HEAD_LEN;
                let mut fits = true;
                for _ in 0..r.element_count {
                    match body.get(at) {
                        Some(&command) => at += net::usermove::element_len(command),
                        None => {
                            fits = false;
                            break;
                        }
                    }
                }
                if fits && body.len().checked_sub(at) == Some(TAIL_LEN) {
                    typed_closed += 1;
                } else if bad.len() < 5 {
                    bad.push(format!("{}: {hex_body}", f.display()));
                }

                let uniform = start
                    + net::mobmove::MOB_PATH_HEAD_LEN
                    + r.element_count as usize * net::mobmove::MOB_PATH_ELEMENT_LEN;
                if body.len().checked_sub(uniform) == Some(TAIL_LEN) {
                    uniform_closed += 1;
                }
            }
        }

        eprintln!(
            "0x02FF archive: {} logs, {distinct} distinct events, {unparsed} unparsed, \
             {typed_closed} close on a {TAIL_LEN}-byte tail with element_len, {uniform_closed} \
             with a uniform {}",
            files.len(),
            net::mobmove::MOB_PATH_ELEMENT_LEN,
        );
        assert!(distinct > 100_000, "the archive should hold hundreds of thousands: {distinct}");
        assert_eq!(unparsed, 0, "every captured body must still parse");
        assert_eq!(
            typed_closed, distinct,
            "the element table must close every body. First few that did not: {bad:#?}"
        );
        // The other half: without this the test would pass just as happily on the old walk.
        assert!(
            uniform_closed < distinct,
            "the uniform {}-byte walk closes every body too, so this test proves nothing - \
             either the archive no longer contains a jump, or the table has been flattened",
            net::mobmove::MOB_PATH_ELEMENT_LEN,
        );
    }

    // ------------------------------------------------------------------------------
    // The drop still lands on a foothold
    // ------------------------------------------------------------------------------

    /// **The composition, on the fixture map from `crate::footholds`.**
    ///
    /// Placing the drop at the corrected position and then asking `Footholds::landing` must
    /// put it on a real, non-wall segment at that x. The stale position and the real one
    /// land on *different* footholds here, which is the on-screen bug: the item appears on
    /// the ledge the mob walked off rather than on the ground it died on.
    #[test]
    fn a_drop_at_the_corrected_position_lands_on_a_foothold_under_it() {
        // The picture from `crate::footholds::tests`: a ledge at y=100 spanning x 100..300,
        // a wall down its right edge, and the ground at y=200 spanning x 300..500.
        let t = crate::footholds::Footholds::parse(
            "7, 1, 100, 100, 300, 100, 0, 2\n\
             7, 2, 300, 100, 300, 200, 1, 3\n\
             7, 3, 300, 200, 500, 200, 2, 0\n",
        );
        // A mob that started on the ledge and walked off the end of it.
        let stale = (150i16, 100i16);
        let real = (400i16, 200i16);

        let a = t.landing(7, stale.0, stale.1).expect("the ledge is under the stale point");
        let b = t.landing(7, real.0, real.1).expect("the ground is under the real one");
        assert_ne!(a.foothold, b.foothold, "two different platforms - the visible bug");
        assert_eq!((a.foothold, a.y), (1, 100), "the stale drop sits up on the ledge");
        assert_eq!((b.foothold, b.y), (3, 200), "the real one is on the ground below");
        assert_eq!(b.moved, 0, "the mob was standing on it, so nothing to fall");

        // And the property the whole placement rests on: whatever comes back is ON a
        // walkable segment of that map, at that x.
        for (x, y) in [stale, real] {
            let l = t.landing(7, x, y).expect("both points have floor under them");
            let fh = t.on_map(7).iter().find(|f| f.id == l.foothold).expect("a real foothold");
            assert!(!fh.is_wall(), "a wall is never landed on");
            assert_eq!(fh.surface_y(i32::from(x)), Some(i32::from(l.y)));
        }
    }

    /// **Against the real 94 089-segment table, when it is there.**
    ///
    /// `gm-handbook/` is generated and gitignored, so this skips itself on a fresh clone
    /// rather than failing it - the arrangement `crate::footholds` already uses.
    ///
    /// It checks the thing item 3 of the brief asks for and that no other test covers: that
    /// a drop placed at a **mob's own** position, staggered by the four offsets the kill
    /// path actually uses, comes to rest on a foothold of that map. The mob positions are
    /// `gm-handbook/mobs.txt`, which is generated from the same `Map.wz` as the footholds
    /// and names the foothold each mob stands on.
    #[test]
    fn every_mob_spawn_point_places_its_drops_on_a_real_foothold() {
        let fh_path = std::path::Path::new("../../gm-handbook/footholds.txt");
        let mob_path = std::path::Path::new("../../gm-handbook/mobs.txt");
        if !fh_path.exists() || !mob_path.exists() {
            eprintln!("skipped: gm-handbook/ is generated and gitignored");
            return;
        }
        let t = crate::footholds::Footholds::load(fh_path);
        let text = std::fs::read_to_string(mob_path).expect("readable");

        let mut checked = 0usize;
        let mut on_a_foothold = 0usize;
        let mut no_surface = 0usize;
        for line in text.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            let (Some(Ok(map)), Some(Ok(x)), Some(Ok(y))) = (
                f.first().map(|v| v.parse::<u32>()),
                f.get(2).map(|v| v.parse::<i16>()),
                f.get(3).map(|v| v.parse::<i16>()),
            ) else {
                continue;
            };
            if !t.has_map(map) {
                continue;
            }
            // The four offsets `session/combat.rs` produces for one to four drops.
            for k in -2i16..=1 {
                let sx = x.saturating_add(k * crate::drops::DROP_STAGGER_PX);
                checked += 1;
                match t.landing(map, sx, y) {
                    Some(l) => {
                        let fh = t
                            .on_map(map)
                            .iter()
                            .find(|s| s.id == l.foothold)
                            .expect("the landing names a foothold of that map");
                        assert!(!fh.is_wall(), "map {map}: a wall is never a landing");
                        assert_eq!(
                            fh.surface_y(i32::from(sx)),
                            Some(i32::from(l.y)),
                            "map {map}: the landing is not on the segment it names"
                        );
                        on_a_foothold += 1;
                    }
                    // No surface on that vertical: the caller keeps the mob's own position,
                    // which is a place a mob was standing and therefore a floor.
                    None => no_surface += 1,
                }
            }
        }
        assert!(checked > 30_000, "the mob file should give tens of thousands: {checked}");
        assert_eq!(
            on_a_foothold + no_surface,
            checked,
            "every placement either lands on a real foothold or declines"
        );
        // The declines are the ones that keep the corpse's position; they are a minority and
        // a jump in them would mean the placement had started guessing.
        assert!(
            no_surface * 4 < checked,
            "more than a quarter of placements found no floor at all: {no_surface} of {checked}"
        );
    }
}
