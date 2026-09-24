//! `0x01B6` - **the field effect**: the "Clear!" banner, its fanfare, and the gate that opens
//! over a party-quest portal. Read 2026-09-23 for First Time Together's stage clear.
//!
//! # Which opcode, and how that was settled
//!
//! `CField::OnPacket`'s dense table (`research/msexe-field-cases.txt`) was anchored on two
//! opcodes this server already sends and the client already draws: `GROUP_MESSAGE 0x01B1`
//! and `WHISPER 0x01B3`. The v214 reference orders the same stretch `GROUP_MESSAGE`,
//! `WHISPER`, `SUMMON_ITEM_INAVAILABLE`, `FIELD_EFFECT`, which puts the effect at `0x01B6`
//! **[C]** - and case `0x01B6` is `FUN_14184a570`, which is the thing that actually settles
//! it **[L]**: it reads one `u8`, refuses anything above `0x50`, and jumps through an
//! 81-entry table at `0x14185366c`. The arms used here:
//!
//! ```text
//!   type 2   str name                  -> FUN_141b7ec60(field, name, -1)   object state
//!   type 3   str name, u8 on           -> FUN_141b7f3b0                    (not used)
//!   type 4   str path                  -> FUN_141859220                    screen
//!   type 7   str path, u32 volume (capped at 0x80), u32, u32, u32
//!                                      -> FUN_1429f5560                    sound
//! ```
//!
//! Those four shapes are exactly v214's `ObjectStateByString 2`, `DisableEffectObject 3`,
//! `Screen 4` and `PlaySound 7` - one layout agreeing field for field with a tree that scores
//! 1 of 8 elsewhere, so the listing carries the claim and the names merely agree.
//!
//! # The paths are relative, and the client says so
//!
//! `FUN_141859220` looks for `.img` in the string (`strstr` against `0x143385a50`); when it is
//! absent it formats the path through **client string `0x723` = `Map/Effect.img/%s`**. The
//! sound arm does the same with **`0x76B` = `Sound/Field.img/%s`**. Both ids were decrypted
//! with `tools/dump_stringids.py`. And both targets exist in this client's data **[L]**:
//! `Map_000.wz/Effect.img` has `quest/party/clear/0..4`, and `Sound_001.wz/Field.img` has
//! `Party1/Clear` (3 753 ms).
//!
//! # The gate
//!
//! Every stage of First Time Together has an `obj` named **`gate`** (`obj_effect/quest/gate`)
//! drawn over its `next00` portal **[L]**, and nothing else in those maps carries a name.
//! Type 2 with that name is what advances it. **What `-1` means inside `FUN_141b7ec60` has
//! not been read** - it is taken to be "advance to the next state", which is what the v214
//! scripts use this for, and which is why the gate is sent **once per field entry** and
//! never twice to the same screen.

use crate::PacketWriter;

/// The field effect opcode. Case `0x01B6` of `CField::OnPacket`.
pub const FIELD_EFFECT: u16 = 0x01B6;

/// `str name` - change the state of the map object carrying that name.
pub const TYPE_OBJECT_STATE: u8 = 2;
/// `str path` - an animation over the whole screen, from `Map/Effect.img`.
pub const TYPE_SCREEN: u8 = 4;
/// `str path, u32 volume, u32, u32, u32` - a sound, from `Sound/Field.img`.
pub const TYPE_SOUND: u8 = 7;

/// The party-quest "Clear!" banner.
pub const SCREEN_PARTY_CLEAR: &str = "quest/party/clear";
/// Its fanfare.
pub const SOUND_PARTY_CLEAR: &str = "Party1/Clear";
/// The party-quest "WRONG" banner - `Map_000.wz/Effect.img/quest/party/wrong/0..4` [L].
pub const SCREEN_PARTY_WRONG: &str = "quest/party/wrong";
/// Its sound - `Sound_001.wz/Field.img/Party1/Failed`, 1 764 ms [L].
pub const SOUND_PARTY_FAILED: &str = "Party1/Failed";
/// The object over each First Time Together `next00` portal.
pub const OBJECT_GATE: &str = "gate";

/// The loudest the client plays; it clamps anything above to this (`cmova esi, 0x80`).
pub const MAX_VOLUME: u32 = 0x80;

/// Type 4: an animation from `Map/Effect.img`, e.g. [`SCREEN_PARTY_CLEAR`].
pub fn screen(path: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(TYPE_SCREEN);
    w.str(path);
    w.into_vec()
}

/// Type 7: a sound from `Sound/Field.img`. The three trailing `u32`s are read and passed
/// on; zero is what v214 sends for all three and what is sent here.
pub fn sound(path: &str, volume: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(TYPE_SOUND);
    w.str(path);
    w.u32(volume.min(MAX_VOLUME));
    w.u32(0);
    w.u32(0);
    w.u32(0);
    w.into_vec()
}

/// Type 2: advance the named map object - [`OBJECT_GATE`] to open a stage's portal.
pub fn object_state(name: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(TYPE_OBJECT_STATE);
    w.str(name);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn maple_str(s: &str) -> Vec<u8> {
        let mut v = (s.len() as u16).to_le_bytes().to_vec();
        v.extend_from_slice(s.as_bytes());
        v
    }

    /// Each arm is its type byte and exactly the fields the listing reads, in order.
    #[test]
    fn each_effect_is_its_type_byte_and_the_fields_the_client_reads() {
        let mut want = vec![TYPE_SCREEN];
        want.extend(maple_str("quest/party/clear"));
        assert_eq!(screen(SCREEN_PARTY_CLEAR), want);

        let mut want = vec![TYPE_OBJECT_STATE];
        want.extend(maple_str("gate"));
        assert_eq!(object_state(OBJECT_GATE), want);

        let mut want = vec![TYPE_SOUND];
        want.extend(maple_str("Party1/Clear"));
        want.extend(100u32.to_le_bytes());
        want.extend([0u8; 12]);
        assert_eq!(sound(SOUND_PARTY_CLEAR, 100), want);
    }

    /// The client clamps the volume at `0x80`; sending more would be a number that lies.
    #[test]
    fn the_volume_is_clamped_where_the_client_clamps_it() {
        let b = sound(SOUND_PARTY_CLEAR, 1_000);
        let at = 1 + 2 + SOUND_PARTY_CLEAR.len();
        assert_eq!(&b[at..at + 4], &MAX_VOLUME.to_le_bytes());
    }

    /// The paths are relative - the client prefixes `Map/Effect.img/` and `Sound/Field.img/`
    /// itself when the string has no `.img` in it. A full path would still work, but would
    /// be a second way of saying the same thing.
    #[test]
    fn the_paths_are_relative_to_the_image_the_client_prefixes() {
        for p in [SCREEN_PARTY_CLEAR, SOUND_PARTY_CLEAR] {
            assert!(!p.contains(".img"), "{p}");
        }
    }
}
