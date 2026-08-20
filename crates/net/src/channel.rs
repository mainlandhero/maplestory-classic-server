//! Why the in-game **Change Channel** dialog has no rows, and the one field that fixes it.
//!
//! Full working, every link read off `client-patched/MapleStory.exe`:
//! `research/channel-select.md` §9.
//!
//! # The mechanism in one paragraph
//!
//! The channel rows are drawn from an array at `singleton+0x2cb8`, and the dialog's row
//! loop is bounded by that array's length. **Exactly one function in the image fills it -
//! `FUN_142cb8e10` - and it has exactly one call site.** The route to it is
//! `LOGIN_RESULT (0x0010)` -> `FUN_141b307b0` -> (mode 5) `FUN_141b32860` ->
//! `FUN_141b2c7c0` -> `FUN_142cb8e10`, and the third arrow carries a guard:
//!
//! ```text
//! 141b32ec6  u32 -> esi                          the login result's worldId
//! 141b32ed0  u32 -> ebx                          the login result's channelId
//! 141b32edb  call FUN_142cb9230   -> singleton+0x2258   the world it already has
//! 141b32ee0  cmp esi, eax
//! 141b32ee2  jne 141b32ef1                       differs -> rebuild
//! 141b32ee8  call FUN_142cb9260   -> singleton+0x2260   the channel it already has
//! 141b32eed  cmp ebx, eax
//! 141b32eef  je  141b32f01                       BOTH equal -> SKIP the rebuild
//! 141b32efc  call FUN_141b2c7c0                  the rebuild
//! ```
//!
//! It is a cache check. A freshly started client holds **(world 0, channel 0)** - the value
//! two of its own reset routines write to mean "nothing loaded" - and this server has always
//! sent `login_result(0, 0, ..)`. Both compares match, the rebuild is skipped, the array
//! stays null, and the dialog draws its world logo and **zero channel rows**. That is the
//! screen exactly as the owner describes it.
//!
//! # The two constraints on the value we may send instead
//!
//! Both are read from the listing, not guessed:
//!
//! 1. **The world id must be one the client has in its world list.** `FUN_141b2c7c0` walks
//!    `stage+0x100` comparing `world->[0] == arg2`; not found leaves the index at `-1` and
//!    the function returns without touching anything (`141b2c88b: xor al,al`). So the world
//!    id stays whatever [`crate::opcode::world_list_entry`] advertised.
//! 2. **The channel index must be valid for that world.** `FUN_141b20d80(world, channel)` at
//!    `141b2c8d4` returns false unless `0 <= channel < len(world->[0x28])`, and a false
//!    there bails the same way. So the priming value cannot simply be a large sentinel.
//!
//! Intersecting those with "must not equal the client's idle `(0, 0)`" leaves exactly one
//! move when the world id is `0`: **send a channel index of `1`**, which requires at least
//! two channels to be advertised. See [`priming_channel`].
//!
//! # Why sending the "wrong" channel at login costs nothing
//!
//! `FUN_142cb8e10` stores the value in `singleton+0x2260`, which the dialog reads as
//! "the channel you are on" (`dialog+0x29c`) to grey it out and exclude it from clicking.
//! It would be wrong for as long as the client sits on the character-select screen - where
//! nothing displays a channel - and then **`SetField` overwrites it**: the `0x01A0` handler
//! `FUN_142097f80` reads a `u32` at body offset 8 and passes it straight to
//! `FUN_142cb91e0`, the `+0x2260` setter, at `14209806f`. That field is already the real
//! channel in [`crate::opcode::set_field_head`], and it is re-sent on every field entry, so
//! by the time the dialog can be opened the current channel is correct again.

use crate::PacketReader;

/// The `(world, channel)` pair a client that has loaded no channel list holds.
///
/// **[D]** from two of the client's own routines, neither of which this server triggers:
/// `FUN_142ce9600` writes `0` to `+0x2258` and `+0x2260` while nulling all three row
/// arrays, and the session teardown `FUN_142cad420` saves `+0x2258` into `+0x2264` and then
/// writes `0` over it. `0` is what this client means by "no world / no channel".
///
/// It is **not** [L]: the object is `HeapAlloc(heap, 0, 0x41a8)` - flags `0`, so no
/// `HEAP_ZERO_MEMORY` - and its constructor `FUN_142ca5c50` nulls the three arrays but never
/// writes these two ints. A recycled heap block could therefore start with garbage here, in
/// which case the compare fails and the list populates **by accident**. That is the best
/// available explanation for the one run in which CH.1 and CH.2 appeared and could never be
/// reproduced; see `research/channel-select.md` §9.6.
pub const IDLE_WORLD: u32 = 0;

/// See [`IDLE_WORLD`].
pub const IDLE_CHANNEL: u32 = 0;

/// The channel index to put in a `LOGIN_RESULT`, so the client rebuilds its channel list.
///
/// `None` means **this world cannot populate the dialog at all**: with a single channel the
/// only index that passes `FUN_141b20d80` is `0`, and `(world 0, channel 0)` is the pair the
/// client already holds. Advertise two channels or accept an empty dialog - there is no
/// third option, and a caller that gets `None` should fall back to the true channel and log
/// why rather than send an index the client will reject.
///
/// When the world id is not [`IDLE_WORLD`] the guard already fails on the world compare, so
/// the honest value is returned untouched.
pub fn priming_channel(world_id: u32, channel_id: u32, channel_count: u32) -> Option<u32> {
    if channel_count == 0 {
        return None;
    }
    if channel_id >= channel_count {
        // Out of range for FUN_141b20d80; the rebuild would bail before doing anything.
        return None;
    }
    if (world_id, channel_id) != (IDLE_WORLD, IDLE_CHANNEL) {
        return Some(channel_id);
    }
    // world 0, channel 0: the client already holds this pair. Take the lowest valid index
    // that is not it.
    (1 < channel_count).then_some(1)
}

/// **Change Channel.** The client's request when the dialog's Change button is pressed.
///
/// Built by `FUN_1418287f0` (`research/msexe-packet-fields.txt:86` already named it; what
/// was missing was that this is the Change Channel action). Reached from `FUN_142a316e0`,
/// the dialog's OK handler, which re-checks the row's enable predicate first.
///
/// **Answered since 2026-08-20** by `crates/world`'s `Session::on_change_channel`, with a
/// migration for the target channel - the same shape as [`crate::opcode::MIGRATE_COMMAND`]
/// at login. That shape is still **[I]**: no capture of a successful channel change exists,
/// and this comment does not upgrade it.
///
/// It had to be answered in the same change that set [`crate::opcode::CHANNEL_ENABLED`] to
/// `1`, because draw and click read the same predicate in the client - there was no way to
/// make a channel row look enabled without making it able to send this, and an unanswered
/// packet freezes the client's whole UI including the quit prompt.
pub const CLIENT_CHANGE_CHANNEL: u16 = 0x00D2;

/// A parsed [`CLIENT_CHANGE_CHANNEL`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeChannelRequest {
    /// **0-based**, matching this repo. The client's dialog labels it `CH.{n+1}`.
    pub target_channel: u8,
    /// Which of the two possible field orders the body turned out to have. Recorded rather
    /// than assumed - see [`ChangeChannelRequest::parse`].
    pub preamble_first: bool,
}

/// The 14-byte integrity preamble every one of these requests carries begins with the
/// client's own literal `100`. Measured in two captured `0x00D1` bodies, which are
/// `64000000 ad150000 00000000 0000` followed by the gameplay fields.
const PREAMBLE_MAGIC: [u8; 4] = [0x64, 0x00, 0x00, 0x00];

/// Where the target channel sits when the preamble comes first.
const TARGET_AFTER_PREAMBLE: usize = 14;

impl ChangeChannelRequest {
    /// Parse the body, **detecting the field order instead of assuming it**.
    ///
    /// `research/channel-select.md` §6 read the encode sequence at `141828925`-`14182896a`
    /// as `u8 targetChannel`, `u32`, then the shared 14-byte `FUN_140c7b890` preamble - i.e.
    /// preamble last. But `0x00D1`'s builder calls the same preamble helper **first**, and
    /// call order is not wire order, so that reading is **[D] and unsettled**; no capture of
    /// a `0x00D2` exists in this repo.
    ///
    /// Rather than commit to one, this discriminates on the preamble's own first four bytes.
    /// The target channel is `0..=29` (the dialog has 30 `ch/N` bitmaps), so it can never be
    /// `0x64`, and the preamble always starts with it. **The discriminator is measured; the
    /// layout it selects is still the thing the first real capture must confirm.**
    ///
    /// `None` means the body is too short or the channel is out of range. The caller must
    /// still answer - a refusal, not silence.
    pub fn parse(payload: &[u8]) -> Option<Self> {
        let preamble_first = payload.get(..4)? == PREAMBLE_MAGIC;
        let at = if preamble_first { TARGET_AFTER_PREAMBLE } else { 0 };
        let mut r = PacketReader::new(payload.get(at..)?);
        let target_channel = r.u8().ok()?;
        if target_channel >= MAX_CHANNELS_IN_DIALOG {
            return None;
        }
        Some(ChangeChannelRequest { target_channel, preamble_first })
    }
}

/// The dialog cannot show more rows than it has bitmaps: `ChannelChange.img/Channel/ch`
/// holds `0..29`, and the hit test lays them out six per row.
pub const MAX_CHANNELS_IN_DIALOG: u8 = 30;

#[cfg(test)]
mod tests {
    use super::*;

    /// The bug, stated as a test: what this server sends today primes nothing.
    #[test]
    fn world_zero_channel_zero_is_the_pair_the_client_already_holds() {
        assert_eq!(priming_channel(0, 0, 2), Some(1));
        assert_ne!(priming_channel(0, 0, 2), Some(IDLE_CHANNEL));
    }

    /// One channel cannot populate the dialog, and the caller has to be told so rather
    /// than handed an index `FUN_141b20d80` will reject.
    #[test]
    fn a_single_channel_world_cannot_prime_the_list() {
        assert_eq!(priming_channel(0, 0, 1), None);
        assert_eq!(priming_channel(0, 0, 0), None);
    }

    /// Constraint 2: the index must be in range for the world, whatever else is true.
    #[test]
    fn the_priming_index_is_always_in_range() {
        for count in 1..8u32 {
            if let Some(c) = priming_channel(0, 0, count) {
                assert!(c < count, "channel {c} is not valid for {count} channel(s)");
            }
        }
    }

    /// A non-idle pair already fails the guard on its own, so nothing is invented.
    #[test]
    fn a_world_that_is_not_zero_keeps_the_truthful_channel() {
        assert_eq!(priming_channel(1, 0, 2), Some(0));
        assert_eq!(priming_channel(0, 1, 2), Some(1));
    }

    /// Preamble-last: the first byte is the channel, and `0x64` can never collide with it.
    #[test]
    fn preamble_last_reads_the_channel_from_byte_zero() {
        let mut body = vec![1u8, 0, 0, 0, 0];
        body.extend_from_slice(&PREAMBLE_MAGIC);
        body.extend_from_slice(&[0u8; 10]);
        let req = ChangeChannelRequest::parse(&body).expect("parsed");
        assert_eq!(req.target_channel, 1);
        assert!(!req.preamble_first);
    }

    /// Preamble-first: the same 19-byte body with the halves swapped still yields 1.
    #[test]
    fn preamble_first_reads_the_channel_after_it() {
        let mut body = Vec::from(PREAMBLE_MAGIC);
        body.extend_from_slice(&[0u8; 10]);
        body.push(1);
        body.extend_from_slice(&[0u8; 4]);
        let req = ChangeChannelRequest::parse(&body).expect("parsed");
        assert_eq!(req.target_channel, 1);
        assert!(req.preamble_first);
    }

    /// No channel index the dialog can produce is `0x64`, which is what makes the
    /// discriminator safe.
    #[test]
    fn a_channel_index_can_never_look_like_the_preamble() {
        assert!(u32::from(MAX_CHANNELS_IN_DIALOG) < u32::from(PREAMBLE_MAGIC[0]));
    }

    /// A short or nonsensical body is reported, not guessed at. The caller still answers.
    #[test]
    fn a_body_that_cannot_be_read_is_none_rather_than_a_guess() {
        assert_eq!(ChangeChannelRequest::parse(&[]), None);
        assert_eq!(ChangeChannelRequest::parse(&[1, 0]), None);
        let mut wild = vec![MAX_CHANNELS_IN_DIALOG, 0, 0, 0, 0];
        wild.extend_from_slice(&[0u8; 14]);
        assert_eq!(ChangeChannelRequest::parse(&wild), None);
    }
}
