//! **The station ship** - `0x01BF` and `0x01C0`, the two opcodes a ship field handles itself.
//!
//! Read out of the client on 2026-09-26: `research/ship-contimove-2026-09-26.md`, with the
//! decompilation in `research/msexe-contimove.c`. A map whose `info/fieldType` is 2 builds a
//! ship field, and its packet handler `FUN_140d6cc90` takes these two opcodes before falling
//! through to `CField::OnPacket` **[L]**. On any other field both land in `CField::OnPacket`'s
//! default arm and are ignored **[L]**, so one reaching the wrong map does nothing.
//!
//! At Ellinia Station (`shipKind 0`, `x 1545`, `x0 2100`, `tMove 15`) both routines play the
//! `Whistle` sound and slide the ship over 15 seconds: **arrive** from `x0` in to `x`, **leave**
//! from `x` out to `x0`. Every one of these packets is an animation, not a placement.

/// `0x01BF` - a ship move: `u8 type, u8 state`. `FUN_140d6cda0`.
pub const CONTI_MOVE: u16 = 0x01BF;
/// `0x01C0` - a ship state: `u8 state, u8 flag`. Inline in `FUN_140d6cc90`.
pub const CONTI_STATE: u16 = 0x01C0;

/// `0x01BF` type 12, state 6 -> `FUN_140d6aba0`: **the ship arrives.** Any other state byte
/// under type 12 does nothing **[L]** (`140d6cfab cmp al, 6`).
pub fn ship_arrives() -> Vec<u8> {
    vec![12, 6]
}

/// `0x01BF` type 8, state 2 -> `FUN_140d6a610`: **the ship leaves.** Any other state byte
/// under type 8 does nothing **[L]** (`140d6cde9 cmp al, 2`).
pub fn ship_leaves() -> Vec<u8> {
    vec![8, 2]
}

/// `0x01C0` for a player arriving on the field: state 1 is arrive, state 2 is leave, both only
/// at a `shipKind 0` field **[L]** (the arms at `140d6ccf9` and `140d6cd38`). The flag byte is
/// read in every case and only used by states 3 and 4, so it is 0.
pub fn station_state(docked: bool) -> Vec<u8> {
    vec![if docked { 1 } else { 2 }, 0]
}

/// `0x01BF` type 10, state 4 -> `FUN_140d6b130` plus a sound: **the Crimson Balrog's ship
/// comes alongside.** It needs a `shipKind 1` field - the deck, `20000022` - and loads that
/// field's own `shipObj` (`ship/ossyria/97`) as a layer at its `x`/`y` **[L]**
/// (`research/ship-contimove-2026-09-26.md`). At the station (`shipKind 0`) it returns at once.
pub fn enemy_ship_arrives() -> Vec<u8> {
    vec![10, 4]
}

/// `0x01BF` type 10, state 5 -> `FUN_140d6bdb0`: the Balrog's ship goes. Not sent today: the
/// passengers leave the deck at the arrival, and the instance with them.
pub fn enemy_ship_leaves() -> Vec<u8> {
    vec![10, 5]
}

/// `0x01C0` for a player arriving on an invaded deck: states 3 and 4 share one arm, which calls
/// the same `FUN_140d6b130` **only when `shipKind == 1` and the flag byte is 1** **[L]**
/// (`140d6cd1e`..`140d6cd33`). 3 is used; 4 would be indistinguishable.
pub fn deck_invaded() -> Vec<u8> {
    vec![3, 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bytes are the arms' own constants, pinned so that a change is a decision.
    #[test]
    fn the_bodies_are_the_arms_the_client_checks_for() {
        assert_eq!(ship_arrives(), [12, 6], "type 12 is the table's last entry; state must be 6");
        assert_eq!(ship_leaves(), [8, 2], "type 8; state must be 2");
        assert_eq!(station_state(true), [1, 0], "states 0, 1, 6 arrive");
        assert_eq!(station_state(false), [2, 0], "states 2, 5 leave");
        assert_eq!((CONTI_MOVE, CONTI_STATE), (0x01BF, 0x01C0), "140d6cca2 sub ecx,0x1bf; then cmp ecx,1");
        assert_eq!(enemy_ship_arrives(), [10, 4], "type 10: state 4 brings the Balrog's ship");
        assert_eq!(enemy_ship_leaves(), [10, 5], "and 5 takes it away");
        assert_eq!(deck_invaded(), [3, 1], "the flag must be 1 or the arm does nothing");
    }
}
