//! **Mob vacuum: the controller's mob-move reports, checked against what an honest client sends.**
//!
//! The owner, 2026-10-03: *"identifying potential cheaters that are 'vacuuming' mobs to a
//! specific location by modifying their client, or forcibly making the mobs only move in one
//! direction."* **Identify, not refuse**: a finding is written to [`SUSPECTS_LOG`] and the
//! channel log, and the report is still applied and acknowledged exactly as before.
//!
//! # Why the reports can be checked at all
//!
//! The controlling client runs each mob's movement and reports it in `0x02FF`
//! (`net::mobmove`): a path HEAD (where the walk began) and up to a few path elements, each
//! with a position, a velocity and a duration. An honest client's report is **one continuous
//! physical walk**, and the archive says how continuous. Measured 2026-10-03 over **432 641**
//! deduplicated archived reports (`research/fixtures/` and `previous-runs/`, deduplicated on
//! `(timestamp, body)` per `CLAUDE.md`):
//!
//! | measure | honest | here |
//! |---|---|---|
//! | the new head vs the previous report's end, same mob | 0 px in 99.9%; the rest are map re-entries and the old controller-handover bug, both reset here | [`JUMP_PX`] |
//! | speed of one element, `abs(dx) * 1000 / duration` | max **253** px/s over 924 855 elements; never a 0-ms element that moves | [`MAX_SPEED`] |
//! | other mobs within 25 px of a mob's end | max **4** (5 mobs together), 17 times | [`STACK_MOBS`] |
//! | `abs(L - R) / (L + R)` of element `vx` signs, 2-minute windows of 100+ moving elements | max **0.094** over 117 windows | [`ONE_WAY`] |
//!
//! Every threshold sits well past the honest maximum, and each also has to PERSIST
//! ([`JUMPS_TO_FLAG`], [`FAST_TO_FLAG`], [`STACK_MS`], the window sizes) before anything is
//! written, so a single odd report never names anybody.
//!
//! **[D]** for the element duration: the common tail is `u8, u16, u8` (`research/user-move.md`
//! §3) and the `u16` summed over a report is **1080 ms in 100% of reports to the 99.99th
//! percentile** - a value that only a duration would produce. The `vx` at element `+5` is
//! **[I]** (the v83 layout `x, y, vx, vy, fh`); it is only used for its sign.
//!
//! # What this cannot see
//!
//! Only this session's own reports - the mobs it controls. A vacuum works on the mobs the
//! cheating client simulates, which are exactly those (`crate::mobshare`), so that is the
//! right vantage point; a mob controlled by somebody else is that somebody's report.
//!
//! **Nothing authenticates.** A finding names the character this connection claimed, not a
//! person.

use std::collections::{HashMap, HashSet, VecDeque};

/// Where findings are written, beside the channel log (`crate::server::record_beside_log`).
pub const SUSPECTS_LOG: &str = "mob-suspects.log";

/// A head this far (either axis) from the same mob's previous end is a teleport. Honest: 0 px
/// in 99.9% of consecutive reports.
pub const JUMP_PX: i32 = 150;
/// Consecutive reports further apart than this are not compared - control may have passed to
/// somebody else and back.
pub const JUMP_GAP_MS: u64 = 3_000;
/// Teleports within [`FLAG_WINDOW_MS`] before a finding: this many, across at least two mobs
/// (or [`JUMPS_ONE_MOB`] of a single one).
pub const JUMPS_TO_FLAG: usize = 3;
/// See [`JUMPS_TO_FLAG`].
pub const JUMPS_ONE_MOB: usize = 5;
/// The window teleports and too-fast elements are counted in.
pub const FLAG_WINDOW_MS: u64 = 30_000;

/// An element faster than this, in px/s, is not a walk. Honest maximum 253.
pub const MAX_SPEED: i32 = 600;
/// ...and it must move at least this far, so a rounding pixel over a short element is not one.
pub const MIN_FAST_DX: i32 = 40;
/// Too-fast elements within [`FLAG_WINDOW_MS`] before a finding.
pub const FAST_TO_FLAG: usize = 2;

/// This many mobs (the mob itself included) inside a [`STACK_BOX`]-pixel box is a stack.
/// Honest maximum 5.
pub const STACK_MOBS: usize = 10;
/// Half-width of the stack box, both axes.
pub const STACK_BOX: i32 = 25;
/// Only mobs reported this recently count towards a stack.
pub const STACK_FRESH_MS: u64 = 3_000;
/// A stack must persist this long before a finding.
pub const STACK_MS: u64 = 3_000;

/// The direction window.
pub const ONE_WAY_WINDOW_MS: u64 = 120_000;
/// Moving elements a window needs before its balance means anything.
pub const ONE_WAY_MIN_ELEMENTS: u32 = 300;
/// Distinct mobs a window needs - one mob pressed against a wall is not a hack.
pub const ONE_WAY_MIN_MOBS: usize = 3;
/// `abs(L - R) / (L + R)` at or above this is one-directional. Honest maximum 0.094.
pub const ONE_WAY: f64 = 0.7;

/// One step of a reported path: where it ends, how fast it says it is going, and how long it took.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Step {
    pub x: i16,
    pub y: i16,
    /// `None` for an element too short to carry one.
    pub vx: Option<i16>,
    /// `None` for an element without the common tail.
    pub duration_ms: Option<u16>,
}

/// The positioned steps of a path block (`MobMoveRequest::path`). Elements that carry no
/// position inherit the previous one and are skipped; a malformed block yields what decoded.
pub fn steps(path: &[u8], element_count: i16) -> Vec<Step> {
    let mut out = Vec::new();
    let Ok(count) = usize::try_from(element_count) else { return out };
    let mut at = net::mobmove::MOB_PATH_HEAD_LEN;
    for _ in 0..count {
        let Some(&command) = path.get(at) else { break };
        let len = net::usermove::element_len(command);
        let Some(e) = path.get(at..at + len) else { break };
        if net::usermove::element_carries_position(command) {
            let i16_at = |o: usize| i16::from_le_bytes([e[o], e[o + 1]]);
            let vx = (len >= net::mobmove::MOB_PATH_ELEMENT_LEN).then(|| i16_at(5));
            let duration_ms = net::usermove::element_has_common_tail(command)
                .then(|| u16::from_le_bytes([e[len - 3], e[len - 2]]));
            out.push(Step { x: i16_at(1), y: i16_at(3), vx, duration_ms });
        }
        at += len;
    }
    out
}

/// What the watch concluded. Each is already past its persistence rule.
#[derive(Debug, Clone, PartialEq)]
pub enum Finding {
    /// Mobs reappearing far from where they were, between two reports.
    Teleports { count: usize, mobs: usize, last_mob: u32, from: (i16, i16), to: (i16, i16) },
    /// Path elements faster than any walk.
    TooFast { count: usize, mob: u32, dx: i32, duration_ms: u16 },
    /// Many mobs held on one spot.
    Stacked { mobs: usize, at: (i16, i16), for_ms: u64 },
    /// Every mob walking the same way.
    OneWay { left: u32, right: u32, mobs: usize },
}

impl Finding {
    /// The kind, for throttling and the log.
    pub fn kind(&self) -> &'static str {
        match self {
            Finding::Teleports { .. } => "TELEPORT",
            Finding::TooFast { .. } => "TOO-FAST",
            Finding::Stacked { .. } => "STACKED",
            Finding::OneWay { .. } => "ONE-WAY",
        }
    }

    /// One readable sentence with the numbers and the honest baseline beside them.
    pub fn describe(&self) -> String {
        match self {
            Finding::Teleports { count, mobs, last_mob, from, to } => format!(
                "{count} mob teleports in {}s across {mobs} mob(s); mob {last_mob} jumped from {from:?} to {to:?} between two reports (honest: 0 px)",
                FLAG_WINDOW_MS / 1000
            ),
            Finding::TooFast { count, mob, dx, duration_ms } => format!(
                "{count} path steps faster than {MAX_SPEED} px/s in {}s; mob {mob} moved {dx} px in {duration_ms} ms (honest max 253 px/s)",
                FLAG_WINDOW_MS / 1000
            ),
            Finding::Stacked { mobs, at, for_ms } => format!(
                "{mobs} mobs held within {STACK_BOX} px of {at:?} for {:.1}s (honest max 5)",
                *for_ms as f64 / 1000.0
            ),
            Finding::OneWay { left, right, mobs } => format!(
                "mob steps {left} left / {right} right across {mobs} mobs in {}s, imbalance {:.2} (honest max 0.09)",
                ONE_WAY_WINDOW_MS / 1000,
                imbalance(*left, *right)
            ),
        }
    }
}

fn imbalance(left: u32, right: u32) -> f64 {
    let n = left + right;
    if n == 0 { 0.0 } else { f64::from(left.abs_diff(right)) / f64::from(n) }
}

/// One session's watch over the mobs it controls, on one field at a time.
#[derive(Debug)]
pub struct MobWatch<F: PartialEq + Copy = crate::fields::FieldKey> {
    field: Option<F>,
    /// object id -> (when, where its last path ended)
    seen: HashMap<u32, (u64, i16, i16)>,
    jumps: VecDeque<(u64, u32)>,
    fast: VecDeque<u64>,
    stack_since: Option<u64>,
    stack_last: u64,
    dir_start: Option<u64>,
    left: u32,
    right: u32,
    dir_mobs: HashSet<u32>,
}

impl<F: PartialEq + Copy> Default for MobWatch<F> {
    fn default() -> Self {
        MobWatch {
            field: None,
            seen: HashMap::new(),
            jumps: VecDeque::new(),
            fast: VecDeque::new(),
            stack_since: None,
            stack_last: 0,
            dir_start: None,
            left: 0,
            right: 0,
            dir_mobs: HashSet::new(),
        }
    }
}

impl<F: PartialEq + Copy> MobWatch<F> {
    /// Take one report. `head` is the path head, `path` its positioned steps, `now_ms` the
    /// session clock. Returns what crossed a line on this report.
    pub fn observe(&mut self, field: F, now_ms: u64, mob: u32, head: (i16, i16), path: &[Step]) -> Vec<Finding> {
        if self.field != Some(field) {
            // A new field: every id means a different mob now. Measured - the honest jumps
            // in the archive are whole maps re-entered with the same object ids.
            *self = MobWatch { field: Some(field), ..MobWatch::default() };
        }
        let mut out = Vec::new();

        // -- teleports: the head should be where the last report ended -------------------
        if let Some(&(t, ex, ey)) = self.seen.get(&mob) {
            let (dx, dy) = ((i32::from(head.0) - i32::from(ex)).abs(), (i32::from(head.1) - i32::from(ey)).abs());
            if now_ms.saturating_sub(t) <= JUMP_GAP_MS && dx.max(dy) > JUMP_PX {
                self.jumps.push_back((now_ms, mob));
                prune(&mut self.jumps, now_ms, |j| j.0);
                let mobs: HashSet<u32> = self.jumps.iter().map(|j| j.1).collect();
                let one = self.jumps.iter().filter(|j| j.1 == mob).count();
                if (self.jumps.len() >= JUMPS_TO_FLAG && mobs.len() >= 2) || one >= JUMPS_ONE_MOB {
                    out.push(Finding::Teleports { count: self.jumps.len(), mobs: mobs.len(), last_mob: mob, from: (ex, ey), to: head });
                }
            }
        }

        // -- speed, and the direction tally ------------------------------------------------
        let (mut px, mut end) = (head.0, head);
        for s in path {
            let dx = (i32::from(s.x) - i32::from(px)).abs();
            if let Some(d) = s.duration_ms {
                if dx >= MIN_FAST_DX && (d == 0 || dx * 1000 / i32::from(d) > MAX_SPEED) {
                    self.fast.push_back(now_ms);
                    prune(&mut self.fast, now_ms, |t| *t);
                    if self.fast.len() >= FAST_TO_FLAG {
                        out.push(Finding::TooFast { count: self.fast.len(), mob, dx, duration_ms: d });
                    }
                }
            }
            match s.vx {
                Some(v) if v < 0 => self.left += 1,
                Some(v) if v > 0 => self.right += 1,
                _ => {}
            }
            if s.vx.is_some_and(|v| v != 0) {
                self.dir_mobs.insert(mob);
            }
            px = s.x;
            end = (s.x, s.y);
        }
        let start = *self.dir_start.get_or_insert(now_ms);
        if now_ms.saturating_sub(start) >= ONE_WAY_WINDOW_MS {
            if self.left + self.right >= ONE_WAY_MIN_ELEMENTS
                && self.dir_mobs.len() >= ONE_WAY_MIN_MOBS
                && imbalance(self.left, self.right) >= ONE_WAY
            {
                out.push(Finding::OneWay { left: self.left, right: self.right, mobs: self.dir_mobs.len() });
            }
            (self.dir_start, self.left, self.right) = (Some(now_ms), 0, 0);
            self.dir_mobs.clear();
        }

        // -- stacking ----------------------------------------------------------------------
        self.seen.insert(mob, (now_ms, end.0, end.1));
        let together = self
            .seen
            .values()
            .filter(|(t, x, y)| {
                now_ms.saturating_sub(*t) <= STACK_FRESH_MS
                    && (i32::from(*x) - i32::from(end.0)).abs() <= STACK_BOX
                    && (i32::from(*y) - i32::from(end.1)).abs() <= STACK_BOX
            })
            .count();
        if together >= STACK_MOBS {
            let since = *self.stack_since.get_or_insert(now_ms);
            self.stack_last = now_ms;
            if now_ms - since >= STACK_MS {
                out.push(Finding::Stacked { mobs: together, at: end, for_ms: now_ms - since });
            }
        } else if now_ms.saturating_sub(self.stack_last) > STACK_FRESH_MS {
            self.stack_since = None;
        }
        out
    }
}

fn prune<T>(q: &mut VecDeque<T>, now_ms: u64, at: impl Fn(&T) -> u64) {
    while q.front().is_some_and(|e| now_ms.saturating_sub(at(e)) > FLAG_WINDOW_MS) {
        q.pop_front();
    }
}

/// How often one kind of finding may be written for one session: the first at once, then at
/// most one line per minute carrying how many were held back.
pub const LOG_EVERY_MS: u64 = 60_000;

/// Per-kind throttle shared by this file and `session/hitwatch.rs`.
#[derive(Debug, Default)]
pub struct Throttle {
    last: HashMap<&'static str, (u64, u32)>,
}

impl Throttle {
    /// `Some(held back since the last line)` when a line for `kind` may be written now.
    pub fn pass(&mut self, kind: &'static str, now_ms: u64) -> Option<u32> {
        match self.last.get_mut(kind) {
            Some((t, held)) if now_ms.saturating_sub(*t) < LOG_EVERY_MS => {
                *held += 1;
                None
            }
            Some((t, held)) => {
                let n = *held;
                (*t, *held) = (now_ms, 0);
                Some(n)
            }
            None => {
                self.last.insert(kind, (now_ms, 0));
                Some(0)
            }
        }
    }
}

impl super::Session {
    /// Feed one accepted `0x02FF` (the controller gate already passed) to the vacuum watch,
    /// and write down whatever it concludes. Never changes what the report does.
    pub(super) fn watch_mob_move(&mut self, map: crate::fields::FieldKey, req: &net::mobmove::MobMoveRequest) {
        let path = steps(&req.path, req.element_count);
        let now = self.clock_ms;
        let findings = self.mob_watch.observe(map, now, req.object_id, (req.x, req.y), &path);
        for f in findings {
            self.record_suspicion(f.kind(), &format!("map {map}: {}", f.describe()));
        }
    }

    /// Write one suspicion to [`SUSPECTS_LOG`] and the channel log, throttled per kind.
    pub(super) fn record_suspicion(&mut self, kind: &'static str, detail: &str) {
        let Some(held) = self.suspect_throttle.pass(kind, self.clock_ms) else { return };
        let account = self.claimed.as_ref().map_or(0, |c| c.account_id);
        let who = self
            .claimed_character()
            .map_or_else(|| "an unclaimed connection".to_string(), |c| format!("character {} (id {}, account {account})", c.name, c.id));
        let more = if held > 0 { format!(" [+{held} more in the last minute]") } else { String::new() };
        let line = format!("SUSPECT {kind} {who}: {detail}{more}");
        crate::server::log(&format!("   {line}"));
        crate::server::record_beside_log(SUSPECTS_LOG, &line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn walk(x: i16, dx: i16, ms: u16) -> Step {
        Step { x: x + dx, y: 100, vx: Some(if dx < 0 { -60 } else { 60 }), duration_ms: Some(ms) }
    }

    /// **An honest wander names nobody**: three mobs walking back and forth at 60 px/s, each
    /// report starting where the last ended, for ten minutes.
    #[test]
    fn an_honest_wander_is_never_a_finding() {
        let mut w = MobWatch::<u32>::default();
        let mut pos = [100i16, 400, 700];
        for tick in 0..600u64 {
            for (i, mob) in [2000u32, 2001, 2002].into_iter().enumerate() {
                let dx = if (tick / 5 + i as u64) % 2 == 0 { 65 } else { -65 };
                let step = walk(pos[i], dx, 1080);
                assert!(w.observe(1, tick * 1080, mob, (pos[i], 100), &[step]).is_empty(), "tick {tick}");
                pos[i] = step.x;
            }
        }
    }

    /// **A vacuum: mobs reappear at one point.** Every report's head is the cheat's spot, not
    /// where the mob last was - teleports, and then a stack.
    #[test]
    fn mobs_pulled_to_one_point_are_teleports_and_then_a_stack() {
        let mut w = MobWatch::<u32>::default();
        let mut got = Vec::new();
        for i in 0..12u32 {
            let x = 100 + 80 * i as i16;
            assert!(w.observe(1, 0, 2000 + i, (x, 300), &[walk(x, 10, 1080)]).iter().all(|f| !matches!(f, Finding::Stacked { .. })));
        }
        for t in 1..6u64 {
            for i in 0..12u32 {
                got.extend(w.observe(1, t * 1080, 2000 + i, (500, 100), &[Step { x: 500, y: 100, vx: Some(0), duration_ms: Some(1080) }]));
            }
        }
        assert!(got.iter().any(|f| matches!(f, Finding::Teleports { mobs, .. } if *mobs >= 2)), "{got:?}");
        assert!(got.iter().any(|f| matches!(f, Finding::Stacked { mobs, .. } if *mobs >= STACK_MOBS)), "{got:?}");
    }

    /// **Dragged inside the path**: a step of 300 px in 100 ms is 3000 px/s. One is not enough;
    /// two in the window are.
    #[test]
    fn a_path_faster_than_any_walk_is_flagged_on_the_second() {
        let mut w = MobWatch::<u32>::default();
        assert!(w.observe(1, 0, 2000, (0, 100), &[walk(0, 300, 100)]).is_empty());
        let f = w.observe(1, 1080, 2000, (300, 100), &[walk(300, 300, 100)]);
        assert!(matches!(f[..], [Finding::TooFast { count: 2, dx: 300, duration_ms: 100, .. }]), "{f:?}");
        // A zero-duration step that moves is never honest.
        let mut w = MobWatch::<u32>::default();
        let _ = w.observe(1, 0, 2000, (0, 100), &[walk(0, 50, 0)]);
        assert!(!w.observe(1, 1080, 2000, (50, 100), &[walk(50, 50, 0)]).is_empty());
    }

    /// **Everything walking left** for two minutes across several mobs, against the honest
    /// worst of 0.09.
    #[test]
    fn mobs_that_only_walk_one_way_are_flagged_when_the_window_closes() {
        let mut w = MobWatch::<u32>::default();
        let mut found = Vec::new();
        let mut pos = [1000i16; 4];
        for tick in 0..=120u64 {
            for (i, mob) in (2000u32..2004).enumerate() {
                let steps: Vec<Step> = (0..3).map(|k| walk(pos[i] - 20 * k, -20, 360)).collect();
                found.extend(w.observe(1, tick * 1080, mob, (pos[i], 100), &steps));
                pos[i] = steps[2].x;
            }
        }
        assert!(found.iter().any(|f| matches!(f, Finding::OneWay { right: 0, mobs: 4, .. })), "{found:?}");
    }

    /// **A new field forgets the old one** - the archive's honest jumps were exactly this.
    #[test]
    fn changing_field_is_not_a_teleport() {
        let mut w = MobWatch::<u32>::default();
        for i in 0..5u32 {
            let _ = w.observe(1, 0, 2000 + i, (100 * i as i16, 100), &[]);
        }
        for i in 0..5u32 {
            assert!(w.observe(2, 1000, 2000 + i, (2000, 900), &[]).is_empty());
        }
    }

    /// The steps decode out of a real archived body: mob 2000's report of 2026-08-21 carries two
    /// elements, durations summing to 1080 ms (every honest report does).
    #[test]
    fn steps_decode_a_captured_path() {
        let hex = "d0070000010000ff000000000000000000000000000000000001000000ccddff00ccddff005087d93c000000000100000000a8018b0100000000020000c2018b012b0000002300000000000000027f020000d5018b012b000000250000000000000002b9010000000faa8ebe57f5c2992500000000000000000000000300000000010000";
        let body: Vec<u8> = (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect();
        let req = net::mobmove::parse_mob_move(&body).expect("a real body");
        let s = steps(&req.path, req.element_count);
        assert_eq!(s.len(), 2, "{s:?}");
        assert_eq!(s.iter().filter_map(|s| s.duration_ms).map(u32::from).sum::<u32>(), 1080, "{s:?}");
        assert!(s.iter().all(|s| s.vx.is_some_and(|v| v.abs() < 300)), "{s:?}");
    }

    #[test]
    fn the_throttle_writes_the_first_then_one_a_minute_with_the_count() {
        let mut t = Throttle::default();
        assert_eq!(t.pass("A", 0), Some(0));
        assert_eq!(t.pass("A", 1_000), None);
        assert_eq!(t.pass("B", 1_000), Some(0), "kinds are separate");
        assert_eq!(t.pass("A", 2_000), None);
        assert_eq!(t.pass("A", 61_000), Some(2));
    }
}
