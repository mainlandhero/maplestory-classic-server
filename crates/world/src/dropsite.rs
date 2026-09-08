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
//! # What this module does NOT do
//!
//! It does not move a drop sideways, does not roll a table, does not decide who owns a drop,
//! and does not re-implement [`crate::footholds`]. `session/combat.rs` still owns the
//! stagger, the landing and the audience walk; this only corrects the point they all start
//! from.

/// Where a 21-byte movement-path element carries its own `(i16 x, i16 y)`.
///
/// **One, not zero.** Every element begins with a `u8` this module does not read; the
/// coordinates follow it. Dumped from a real body (`previous-runs/world-20260907-233531.log`,
/// mob 2020, move 2357):
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
/// `None` when the path carries no elements (the mob reported a position and no steps), when
/// the count is negative, or when the block is too short to hold the elements it claims. In
/// every one of those the caller should keep the head, which is what
/// [`reported_position`] does.
///
/// `path` is [`net::mobmove::MobMoveRequest::path`] exactly as the parser produced it:
/// `MOB_PATH_HEAD_LEN` bytes then `element_count * MOB_PATH_ELEMENT_LEN`. The length is
/// re-checked here rather than assumed, because this function is also the thing a test hands
/// a deliberately ragged buffer.
pub fn path_end(path: &[u8], element_count: i16) -> Option<(i16, i16)> {
    let count = usize::try_from(element_count).ok()?;
    if count == 0 {
        return None;
    }
    let head = net::mobmove::MOB_PATH_HEAD_LEN;
    let elem = net::mobmove::MOB_PATH_ELEMENT_LEN;
    // The LAST element, addressed from the front. Addressing it from the end of the slice
    // would silently read a byte of the caller's trailer if `path` ever carried one.
    let at = head.checked_add(count.checked_sub(1)?.checked_mul(elem)?)?;
    let last = path.get(at..at.checked_add(elem)?)?;
    let xy = last.get(PATH_ELEMENT_XY_AT..PATH_ELEMENT_XY_AT + 4)?;
    let x = i16::from_le_bytes([xy[0], xy[1]]);
    let y = i16::from_le_bytes([xy[2], xy[3]]);
    Some((x, y))
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
    const FALL_N1: &str = "e9070000230d00ff000000000000000000000000000000000001000000ccddff\
00ccddff005087d93c0000000002000000008b007ffe5e0052000800008b007dfe5e0052003d0000000000000002\
5a0000008b007dfe5e00520000000000000000000600000001b300d5fd0600000000bc0030feb300f1ff00000000\
00000000060e010000fc00abfeb3009e02000000000000000006680100000001bbfe590000000000000000000000\
06180000000e01bbfe770000002d00000000000000028d0000002601bbfe7d0000002e0000000000000002c30000\
00000faa8ebe44ce727a2e000000b8b9160b000000000300000000010000";

    fn req(hex: &str) -> net::mobmove::MobMoveRequest {
        let body: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect();
        net::mobmove::parse_mob_move(&body).expect("a real captured body must parse")
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

    /// Every element of a real path decodes to a coordinate on the map, not to a wild
    /// `i16` pair. The off-by-one that reads offset 0 gives `(11264, -17661)` for one of
    /// these and no format check would catch it.
    #[test]
    fn every_element_of_a_real_path_decodes_to_a_sane_coordinate() {
        for hex in [WALK_N, WALK_N1, FALL_N, FALL_N1] {
            let r = req(hex);
            let n = usize::try_from(r.element_count).unwrap();
            assert_eq!(
                r.path.len(),
                net::mobmove::MOB_PATH_HEAD_LEN + n * net::mobmove::MOB_PATH_ELEMENT_LEN,
                "the path length identity, or the element walk below is off its grid"
            );
            for i in 1..=n {
                let at = net::mobmove::MOB_PATH_HEAD_LEN
                    + (i - 1) * net::mobmove::MOB_PATH_ELEMENT_LEN
                    + PATH_ELEMENT_XY_AT;
                let x = i16::from_le_bytes([r.path[at], r.path[at + 1]]);
                let y = i16::from_le_bytes([r.path[at + 2], r.path[at + 3]]);
                // Every coordinate in gm-handbook/footholds.txt lies in x -5570..6338,
                // y -4187..3900. A decode off by one byte leaves that box immediately.
                assert!((-6000..7000).contains(&x), "element {i} x = {x} is off the map");
                assert!((-5000..4500).contains(&y), "element {i} y = {y} is off the map");
            }
        }
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
