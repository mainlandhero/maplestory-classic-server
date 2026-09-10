//! The wall clock a field clock shows: **the machine's local time**, not UTC.
//!
//! `server::log` stamps its lines in UTC - it divides Unix seconds and never asks for a
//! zone - and that is the right thing for a log. A clock drawn on the wall of Ellinia
//! Station is a different thing: a player reads it against their own watch, and on the owner's
//! machine that watch is four hours behind the log. Sending UTC would hang a clock that is
//! wrong by four hours and looks exactly like a timezone bug in the client.
//!
//! No `chrono`. The workspace has no time crate and this needs one call: on Windows it is
//! `GetLocalTime`, declared here the same way `grap-stub` declares its kernel32 imports.
//! Anywhere else it falls back to UTC, and the test names that as a fallback rather than
//! hiding it.

use std::time::{SystemTime, UNIX_EPOCH};

/// `(hour, minute, second)` of the local wall clock. Hour is `0..24` - the client's own
/// set-time routine divides by 12 for its AM/PM flag, so the 24-hour value is what it wants.
#[cfg(windows)]
pub fn local_hms() -> (u8, u8, u8) {
    let mut t = Win32SystemTime::default();
    // SAFETY: `GetLocalTime` writes exactly one SYSTEMTIME - eight u16 fields in this order,
    // 16 bytes - into the pointer it is given, and it cannot fail.
    unsafe { GetLocalTime(&mut t) };
    (t.hour as u8, t.minute as u8, t.second as u8)
}

/// The non-Windows build has no local zone without a crate, so it is UTC. Named so a reader
/// on that platform is not surprised by the offset.
#[cfg(not(windows))]
pub fn local_hms() -> (u8, u8, u8) {
    utc_hms()
}

/// `(hour, minute, second)` in UTC, from the same arithmetic `server::log` uses.
pub fn utc_hms() -> (u8, u8, u8) {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    (((secs / 3600) % 24) as u8, ((secs / 60) % 60) as u8, (secs % 60) as u8)
}

/// Win32 `SYSTEMTIME`. **A layout read from a header is a claim** - `CLAUDE.md`'s minidump
/// section is about exactly this - so the test below checks it against the call rather than
/// against the header: a wrong field order gives a local-minus-UTC offset that is not a
/// whole quarter hour.
#[cfg(windows)]
#[repr(C)]
#[derive(Default)]
struct Win32SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn GetLocalTime(out: *mut Win32SystemTime);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both clocks are in range. Cheap, and it is the half a wrong `cfg` would break.
    #[test]
    fn both_clocks_are_in_range() {
        for (h, m, s) in [local_hms(), utc_hms()] {
            assert!(h < 24, "hour {h}");
            assert!(m < 60, "minute {m}");
            assert!(s < 60, "second {s}");
        }
    }

    /// **Local minus UTC is a whole number of quarter hours.** Every real zone offset is,
    /// and this is the check that can disagree with the struct: read `day_of_week` where
    /// `hour` should be, or swap minute and second, and the offset stops being one.
    ///
    /// Sampled inside one UTC second - the two clocks are read at different instants, so a
    /// minute boundary between the reads would fake a one-minute offset. Retried rather than
    /// widened.
    #[test]
    fn local_time_is_utc_plus_a_whole_quarter_hour() {
        for _ in 0..5 {
            let before = utc_hms();
            let local = local_hms();
            let after = utc_hms();
            if before != after {
                continue; // straddled a second; sample again
            }
            let utc_min = i32::from(before.0) * 60 + i32::from(before.1);
            let local_min = i32::from(local.0) * 60 + i32::from(local.1);
            let offset = (local_min - utc_min).rem_euclid(24 * 60);
            assert_eq!(
                offset % 15,
                0,
                "local {:02}:{:02} minus UTC {:02}:{:02} is {offset} minutes, not a zone offset",
                local.0,
                local.1,
                before.0,
                before.1
            );
            // Seconds do not depend on the zone at all.
            assert_eq!(local.2, before.2, "the second hand must agree");
            return;
        }
        panic!("five samples in a row straddled a second boundary");
    }

    /// The struct is 16 bytes - the size `GetLocalTime` writes. Pinned so a stray field
    /// cannot turn the call into a four-byte stack overrun that happens to work.
    #[cfg(windows)]
    #[test]
    fn the_win32_struct_is_the_size_the_call_writes() {
        assert_eq!(std::mem::size_of::<Win32SystemTime>(), 16);
    }
}
