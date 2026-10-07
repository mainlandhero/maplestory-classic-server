//! **Another player's buffs, on everybody else's screen** - `0x02B0` set, `0x02B1` reset.
//!
//! The owner, 2026-10-05: *"Currently other players are not able to see when the thief players
//! enter dark sight on other players screens."* `0x007D`/`0x007E` (`crate::buff`) only ever reach
//! the caster; nothing told the rest of the map, and `0x0224` spawned every player with an
//! all-zero remote mask.
//!
//! # Found by enumerating the callers of the remote decoder **[L]**
//!
//! `CSecondaryStat::DecodeForRemote`, `FUN_140a46e50`, has exactly two callers
//! (`tools/callers.py`: 2 call sites, 0 tail jumps, 0 data pointers):
//!
//! * `0x1429ce270` - `UserInit`, the `0x0224` body (`crate::userpool`);
//! * `0x1429d62f0` - table C's handler for **`0x02B0`** (`research/user-pool-tables.md` §9).
//!
//! Table C is the remote family: the dispatcher consumes a `u32` character id and finds that
//! user in its own pool before the handler runs, so every body here starts with the id of the
//! player whose buff it is. `0x02B1`'s handler `0x1429d6500` is the reset beside it.
//!
//! ```text
//! 0x02B0   u32 charId                       (consumed by the dispatcher)
//!          DecodeForRemote:  raw[124] mask, the set bits' fields, the 23-byte tail
//!          u16                              0x1429d632e - the delay
//!          u8                               0x1429d6339
//!
//! 0x02B1   u32 charId
//!          raw[124] mask                    0x1429d658f
//!          u8, read only when the mask meets a constant set   0x1429d65f6
//! ```
//!
//! # Dark Sight costs no bytes in the remote decoder **[L]**
//!
//! `0x140a473da mov edx,0x63` - bit 99 - `call 0x1402bf6d0` (the bit test), then
//! `mov edx,1 / call 0x140897670` and straight on to bit 0x68. **No read**: the remote
//! decoder sets the flag to 1 from the mask alone. So a Dark Sight `0x02B0` is the mask with bit
//! 99, then the same 23-byte tail an all-zero mask reads (`crate::userpool::REMOTE_STAT_TAIL_LEN`).
//!
//! # Padded, deliberately
//!
//! Two reads are conditional on constant masks this pass did not decode: `0x02B1`'s `u8` and the
//! `0x14087ae30` branch before it. **This client ignores trailing bytes** - 198 bytes of `0x007D`
//! were accepted (`crate::buff`'s `temporary_stat_reset` docs) - so both bodies carry
//! [`PAD_LEN`] zero bytes after the last known read: a conditional read that does fire reads a
//! zero, where a short body would underrun. Same reasoning as `crate::buff::TAIL_LEN`.

use crate::PacketWriter;

/// Another player's buff switched on.
pub const REMOTE_TEMPORARY_STAT_SET: u16 = 0x02B0;
/// Another player's buff switched off.
pub const REMOTE_TEMPORARY_STAT_RESET: u16 = 0x02B1;

/// Zero bytes after the last known read - see the module docs.
pub const PAD_LEN: usize = 16;

/// The remote bits this server may set: those `FUN_140a46e50` reads **nothing** for, so a
/// mask with them and the plain 23-byte tail is a complete body. Only Dark Sight today.
pub const ZERO_BYTE_REMOTE_BITS: [u32; 1] = [crate::jobbuffs::CTS_DARK_SIGHT];

/// `0x02B0`: `character`'s `bits` switched on, for everybody else on the map.
pub fn remote_stat_set(character: u32, bits: &[u32]) -> Vec<u8> {
    debug_assert!(bits.iter().all(|b| ZERO_BYTE_REMOTE_BITS.contains(b)), "a bit whose remote decode reads bytes: {bits:?}");
    let mut w = PacketWriter::new();
    w.u32(character);
    w.bytes(&crate::buff::stat_mask(bits));
    w.zeros(crate::userpool::REMOTE_STAT_TAIL_LEN);
    w.u16(0); // delay
    w.u8(0);
    w.zeros(PAD_LEN);
    w.into_vec()
}

/// `0x02B1`: `character`'s `bits` switched off.
pub fn remote_stat_reset(character: u32, bits: &[u32]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character);
    w.bytes(&crate::buff::stat_mask(bits));
    w.u8(0);
    w.zeros(PAD_LEN);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both bodies against the read order above: the id, the mask with bit 99 at the client's own
    /// position, then the fixed reads and the pad.
    #[test]
    fn the_dark_sight_bodies_are_laid_out_the_way_the_handlers_read_them() {
        let set = remote_stat_set(215, &[crate::jobbuffs::CTS_DARK_SIGHT]);
        assert_eq!(set.len(), 4 + crate::buff::MASK_LEN + crate::userpool::REMOTE_STAT_TAIL_LEN + 2 + 1 + PAD_LEN);
        assert_eq!(&set[..4], &215u32.to_le_bytes());
        assert_eq!(&set[4..4 + crate::buff::MASK_LEN], &crate::buff::stat_mask(&[99])[..]);
        // Bit 99: word 3, bit 31 - (99 & 31) = 28 -> 0x10000000, little-endian.
        assert_eq!(&set[4 + 12..4 + 16], &[0x00, 0x00, 0x00, 0x10]);
        assert!(set[4 + crate::buff::MASK_LEN..].iter().all(|&b| b == 0), "tail, delay, byte and pad are zero");

        let reset = remote_stat_reset(215, &[crate::jobbuffs::CTS_DARK_SIGHT]);
        assert_eq!(reset.len(), 4 + crate::buff::MASK_LEN + 1 + PAD_LEN);
        assert_eq!(&reset[4..4 + crate::buff::MASK_LEN], &set[4..4 + crate::buff::MASK_LEN]);
    }
}
