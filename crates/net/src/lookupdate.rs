//! `0x02AE` - redress another player's copy of a character IN PLACE: the look, decoded
//! straight into the pooled user, then the avatar rebuilt. No leave, no enter, no blink, the
//! pet copy untouched.
//!
//! The owner, 2026-09-18: *"The leave-and-enter path causes the pets to reload for that client, and
//! it causes a brief blink. That is undesirable. Please find another suitable way without
//! leave-and-enter."* `research/remote-redress-2026-09-18.md` is the decode; the short form:
//!
//! * `0x0138 UserAvatarModified` can never do it: its apply walks the user's **summoned** map
//!   (`user+0x1200`, filled only by the `0x03A0..0x03C5` pool packets) and re-dresses those
//!   objects, never the player. With its gate opened by a hook patch it walked an empty map
//!   (measured 14:07); with it closed it walked nothing. Retired.
//! * `0x02AE` is a user-pool by-id packet on the **same router, gates and dispatch table as
//!   the chair relay `0x02AD`** (confirmed on two screens): the pool finds the user by id,
//!   `FUN_1429d5290(user, packet)` reads a flag, and with bit 0 set decodes the compact look
//!   into `user+0x130` and calls `FUN_140f80200(user+0x100, 0,0,0,0)` - the rebuild that
//!   dressed the user in the first place. Then the same two post-passes every `0x0224` runs.
//!
//! **[L]** for the routing (read from the raw table bytes with `0x02AD` as the control), the 18
//! reads and every store; **[D]** that the rebuild draws the new look - `0x02AE` with a real body
//! has never been on a wire. Plan step TO(c) is the test.
//!
//! **The router looks the id up in the pool hash only**, so it cannot address the local
//! player: send it to the OTHER sessions on the field, carrying the changed character's id.
//! And the body length must be exact - the chair relay one byte short faulted a client.

use crate::opcode::{avatar_look, Character};
use crate::PacketWriter;

/// The in-place redress. Body, at the reading addresses in the research §3.2:
///
/// ```text
/// u32  characterId          the router's read; must be in the observer's pool
/// u8   flag = 1             bit 0: a look follows (bits 1, 2 would add one u8 each; clear)
/// ...  avatar_look(chr)     195 + 5 per worn item, the same bytes 0x0224 carries
/// u8   0                    couple ring: none
/// u8   0                    friendship ring: none
/// u8   0                    marriage: none (the three fields are cleared)
/// u32  0                    -> user+0x2e08, what the 0x0224 body already sent there
/// u32  0                    -> user+0x4358, likewise
/// ```
pub const USER_LOOK_UPDATE_REMOTE: u16 = 0x02AE;

/// The look only; the ring and marriage bytes are left clear.
const FLAG_LOOK: u8 = 1;

/// Fixed bytes around the look: id 4, flag 1, three ring bytes, two trailing u32.
const LOOK_UPDATE_FIXED_LEN: usize = 4 + 1 + 3 + 4 + 4;

/// The compact look with both item maps empty.
const AVATAR_LOOK_EMPTY_LEN: usize = 195;

/// Length of a [`user_look_update_remote`] body: **211 + 5 per worn item.**
pub fn user_look_update_remote_len(chr: &Character) -> usize {
    LOOK_UPDATE_FIXED_LEN + AVATAR_LOOK_EMPTY_LEN + 5 * chr.equips.len()
}

/// Build a [`USER_LOOK_UPDATE_REMOTE`] body for `chr`, whose look just changed.
pub fn user_look_update_remote(chr: &Character) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(chr.id);
    w.u8(FLAG_LOOK);
    w.bytes(&avatar_look(chr));
    w.u8(0); // couple
    w.u8(0); // friendship
    w.u8(0); // marriage
    w.u32(0); // user+0x2e08
    w.u32(0); // user+0x4358
    let out = w.into_vec();
    debug_assert_eq!(out.len(), user_look_update_remote_len(chr));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The length is exact, and it moves by five per worn item** - the chair relay at
    /// 12 bytes instead of 13 faulted a client, so the arithmetic is pinned against the
    /// builder rather than trusted.
    #[test]
    fn the_body_is_211_bytes_plus_five_per_worn_item_and_starts_with_the_id_and_the_flag() {
        let mut chr = Character { id: 215, ..Default::default() };
        let b = user_look_update_remote(&chr);
        assert_eq!(b.len(), 211);
        assert_eq!(b.len(), user_look_update_remote_len(&chr));
        assert_eq!(&b[..4], &215u32.to_le_bytes());
        assert_eq!(b[4], 1, "bit 0: a look follows");
        assert_eq!(&b[5..5 + 195], &avatar_look(&chr)[..], "the same look 0x0224 carries");
        assert_eq!(&b[200..], &[0u8; 11], "no rings, no marriage, the two trailing u32s zero");

        chr.equips = vec![(1, 1_002_005), (5, 1_040_002), (11, 1_302_000)];
        let b = user_look_update_remote(&chr);
        assert_eq!(b.len(), 211 + 15);
        assert_eq!(&b[5..5 + 195 + 15], &avatar_look(&chr)[..]);
    }

    /// The opcode sits in the user pool's compacted third switch beside the chair relay -
    /// the routing the research verified from the table bytes with `0x02AD` as the control.
    #[test]
    fn the_opcode_is_the_chair_relays_neighbour_in_the_third_switch() {
        assert_eq!(USER_LOOK_UPDATE_REMOTE, crate::chair::USER_SIT_REMOTE + 1);
        assert!((crate::userpool::USER_POOL_REMOTE_FIRST..=crate::userpool::USER_POOL_REMOTE_LAST).contains(&USER_LOOK_UPDATE_REMOTE));
    }
}
