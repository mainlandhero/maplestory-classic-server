//! The wall clock a field clock shows: **UTC**, which is this server's time.
//!
//! The owner, 2026-09-10, after the first version sent the machine's local time: *"The clock is
//! reflecting EDT time of the machine, this needs to read the UTC time."* So the station
//! clock is server time, the way the original game's was, and it has one useful property
//! for this project: it reads the same as every stamp in `world.log`, which `server::log`
//! derives with exactly this arithmetic. A screenshot of the wall and a line in the log
//! can be matched by eye.
//!
//! The local-time version - `GetLocalTime` through a kernel32 extern - was removed rather
//! than left beside this, because an unused second clock is precisely the kind of code that
//! gets wired by mistake later.

use std::time::{SystemTime, UNIX_EPOCH};

/// `(hour, minute, second)` in UTC. Hour is `0..24` - the client's own set-time routine
/// divides by 12 for its AM/PM flag, so the 24-hour value is what it wants.
pub fn utc_hms() -> (u8, u8, u8) {
    hms_of(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
}

/// The split, separated so the arithmetic can be pinned against known instants.
pub fn hms_of(unix_secs: u64) -> (u8, u8, u8) {
    (
        ((unix_secs / 3600) % 24) as u8,
        ((unix_secs / 60) % 60) as u8,
        (unix_secs % 60) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Known instants, so the split cannot drift from `server::log`'s without a test
    /// saying so. 1 757 462 400 is 2025-09-10 00:00:00 UTC.
    #[test]
    fn the_split_matches_known_instants() {
        assert_eq!(hms_of(0), (0, 0, 0));
        assert_eq!(hms_of(1_757_462_400), (0, 0, 0));
        assert_eq!(hms_of(1_757_462_400 + 13 * 3600 + 7 * 60 + 9), (13, 7, 9));
        assert_eq!(hms_of(86_399), (23, 59, 59), "the last second of a day");
        assert_eq!(hms_of(86_400), (0, 0, 0), "and it wraps");
    }

    /// The live clock is in range and is UTC: it agrees with the raw Unix arithmetic
    /// sampled around it, so a zone conversion cannot have crept back in.
    #[test]
    fn the_live_clock_is_utc_and_in_range() {
        let before = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let (h, m, s) = utc_hms();
        let after = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        assert!(h < 24 && m < 60 && s < 60, "{h:02}:{m:02}:{s:02}");
        assert!(
            (h, m, s) == hms_of(before) || (h, m, s) == hms_of(after),
            "{h:02}:{m:02}:{s:02} is neither {:?} nor {:?}",
            hms_of(before),
            hms_of(after)
        );
    }
}
