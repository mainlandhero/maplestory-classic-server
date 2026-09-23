//! `0x01BC` - **the field clock**, decoded 2026-09-10.
//!
//! The owner, with a screenshot of Ellinia Station: *"the server clock does not seem to work. It
//! just stays on 00:00."* The widget is the map's own - `010002090.img` has a top-level
//! `clock` node beside `back`, `foothold` and `portal` - and the client builds it at field
//! entry (`FUN_141841430`, called from the field init) and then waits to be told the time.
//! Nothing ever told it.
//!
//! # Read out of the client, not out of a name table
//!
//! `CField::OnPacket` (`FUN_141820080`) dispatches `0x01A4..0x0223` through a 128-entry
//! table at `0x141822158`; the whole table is in `research/msexe-field-cases.txt`. Case
//! `0x01BC` is a virtual call through slot 59 of the field vtable, and the implementation is
//! `FUN_1418564d0`: it reads a `u8` type, refuses anything above `0x11`, and jumps through an
//! 18-entry table at `0x141856f9c`. The arms whose bodies have been read:
//!
//! ```text
//!   type 0   u32 seconds       -> an event-timer window (FUN_142d98870); 0 or less destroys it
//!   type 1   u8 hour, u8 minute, u8 second
//!                              -> fetch the map clock from the field's +0x220 holder and
//!                                 call FUN_1415ea5c0(widget, h, m, s)         <- THIS ONE
//!   type 2   u32 seconds       -> a countdown built by FUN_141839a60
//!   type 3   no arm; the table entry is the epilogue
//!   type 4   u32, u32          -> a gauge window (UI resource 0x310)
//!   5..17    read but not needed here; 8, 11, 12 and 15 have no arm either
//! ```
//!
//! The v214 reference's `ClockType` lists EventTimer 0, HMSClock 1, SecondsClock 2 and
//! TimerGauge 4 with exactly those shapes. That tree scores 1 of 8 here, so the shapes above
//! carry the claim and the name merely agrees with them.
//!
//! **The unit.** `FUN_1415ea5c0` divides the hour by 12 for an AM/PM flag, stores `hour % 12`
//! with 0 shown as 12, then the minute and the second, and stamps `GetTickCount` so the widget
//! keeps ticking on its own. So the body carries a **24-hour** wall-clock hour and the client
//! makes the 12-hour AM/PM display in the screenshot from it.
//!
//! # Only on a map that has the widget
//!
//! The type-1 arm fetches the widget with `FUN_1418a7dd0`, which **throws** (code `0x431`)
//! when the holder is empty, and there is no null check between the fetch and the set. A map
//! without a `clock` node builds no widget. So this packet is sent only where
//! `gm-handbook/clocks.txt` says the map declares one - `world::config::Config::clocks` -
//! and never as a blanket "every field entry" packet.
//!
//! # What is deliberately not here
//!
//! Types 0, 2 and 4 are measured and would be four lines each. They are not built because
//! nothing sends them, and `CLAUDE.md`'s "built is not wired" is about exactly that shape of
//! module. The shapes are written down above so the day something needs a countdown, it
//! starts from a reading rather than a guess.

use crate::PacketWriter;

/// The field clock opcode. Case `0x01BC` of `CField::OnPacket`'s dense switch.
pub const FIELD_CLOCK: u16 = 0x01BC;

/// The type byte for a wall clock: hour, minute, second.
pub const CLOCK_TYPE_HMS: u8 = 1;

/// Body length of a type-1 clock: the type byte and three time bytes.
pub const CLOCK_HMS_LEN: usize = 4;

/// The type byte for a **countdown**: one `u32` of seconds.
pub const CLOCK_TYPE_SECONDS: u8 = 2;

/// Body length of a type-2 clock: the type byte and the `u32`.
pub const CLOCK_SECONDS_LEN: usize = 5;

/// Build a type-1 [`FIELD_CLOCK`] body: `u8 1, u8 hour, u8 minute, u8 second`.
///
/// `hour` is `0..24`. The client divides by 12 itself, so passing a 12-hour value would
/// show every afternoon as morning.
pub fn clock_hms(hour: u8, minute: u8, second: u8) -> Vec<u8> {
    debug_assert!(hour < 24, "hour {hour} is not a 24-hour value");
    debug_assert!(minute < 60, "minute {minute}");
    debug_assert!(second < 60, "second {second}");
    let mut w = PacketWriter::new();
    w.u8(CLOCK_TYPE_HMS); // 141856509: the type, switched at 0x141856511
    w.u8(hour); //          141856569 -> esi
    w.u8(minute); //        141856574 -> edi
    w.u8(second); //        14185657f -> ebx, then FUN_1415ea5c0(widget, esi, edi, ebx)
    w.into_vec()
}

/// Build a type-2 [`FIELD_CLOCK`] body: `u8 2, u32 seconds` - the countdown in the corner.
///
/// # This one is safe on a map with no `clock` node, and that had to be checked
///
/// The type-1 arm above fetches the map's widget with `FUN_1418a7dd0`, which **throws** when
/// the holder is empty - which is why `clock_hms` is sent only to maps in
/// `gm-handbook/clocks.txt`. The party quest's seven fields declare no `clock` node at all,
/// so sending type 1 there would kill the client.
///
/// Type 2 does not fetch. Decompiled 2026-09-22 (`research/msexe-clock-countdown.c`): the
/// arm at `FUN_1418564d0` case 2 reads the `u32` and calls `FUN_141839a60`, which
/// **allocates** a `0x2c8` object (`FUN_14019b780`), constructs it (`FUN_14189f7e0`) and
/// stores it into the very `+0x220` holder type 1 reads from (`FUN_1418a74f0`). Its only
/// `0x431` throws are on the slot it has *just* filled. So it builds its own widget and
/// needs nothing from the map. **[L]**
///
/// `0` is not special here and is not used as "stop": the countdown simply shows zero.
pub fn clock_seconds(seconds: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(CLOCK_TYPE_SECONDS); // 141856511: the switch
    w.u32(seconds); //           the u32 FUN_1406e8c20 reads, passed to FUN_141839a60
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The countdown is the type byte and one little-endian `u32`, and nothing else.
    #[test]
    fn a_countdown_is_the_type_byte_and_a_u32() {
        let b = clock_seconds(1_800);
        assert_eq!(b.len(), CLOCK_SECONDS_LEN);
        assert_eq!(b[0], CLOCK_TYPE_SECONDS);
        assert_eq!(&b[1..], &1_800u32.to_le_bytes());
        assert_eq!(clock_seconds(0), vec![CLOCK_TYPE_SECONDS, 0, 0, 0, 0], "zero is a value, not a stop");
    }

    /// **Four bytes, type first, then hour, minute, second** - the order the handler reads
    /// them into `esi`, `edi`, `ebx` and passes on. A swapped pair would draw a plausible
    /// time that is simply wrong, which is the kind of bug that survives a glance.
    #[test]
    fn a_wall_clock_is_the_type_byte_then_hour_minute_second() {
        let b = clock_hms(14, 5, 9);
        assert_eq!(b.len(), CLOCK_HMS_LEN);
        assert_eq!(b, vec![1, 14, 5, 9]);
        assert_eq!(b[0], CLOCK_TYPE_HMS, "type 1 is the only arm that takes three bytes");
    }

    /// **24-hour, not 12.** The client's own set-time routine does the division by 12, so
    /// the afternoon must go out as 13..23. This pins that an afternoon hour is passed
    /// through unchanged rather than folded.
    #[test]
    fn the_hour_goes_out_as_a_24_hour_value() {
        assert_eq!(clock_hms(0, 0, 0)[1], 0, "midnight is 0; the client shows it as 12");
        assert_eq!(clock_hms(12, 0, 0)[1], 12);
        assert_eq!(clock_hms(23, 59, 59), vec![1, 23, 59, 59]);
    }

    /// The opcode is the dense-switch case, and it is inside the range that switch covers.
    #[test]
    fn the_opcode_is_inside_the_field_switch() {
        assert_eq!(FIELD_CLOCK, 0x01BC);
        assert!((0x01A4..=0x0223).contains(&FIELD_CLOCK), "outside CField's dense switch");
    }
}
