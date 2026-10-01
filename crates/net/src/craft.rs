//! Crafting: `0x02F6` from the client, `0x0398` back, and the packed mastery level.
//!
//! The owner, 2026-09-21: *"We need to implement crafting in our server. After these quest
//! completions, they should unlock the appropriate crafting menu within the client."*
//! `research/crafting-2026-09-21.md` is the decode; the short form:
//!
//! * The Crafting Journal is a **client-side window** over `Etc/CraftRecipe.img`. It asks to
//!   start a craft, runs the recipe's `ProcessTimeMS` animation itself, then asks to finish.
//! * Six professions, six ordinary skills (`92000000`..`92050000`). **The tab unlocks on the
//!   skill's level alone** - no flag, no opcode - so granting the skill on the quest turn-in
//!   is the whole of "unlock the menu".
//! * A profession skill's **`level` field carries the mastery too**:
//!   `(level << 24) | masteryExp`. See [`packed_mastery`].
//!
//! Every field below is off the client's own listing except where a doc comment says
//! otherwise, and the two request shapes were read at `FUN_1410e9c00` (mode 0),
//! `FUN_1410e9d20` (mode 3) and `FUN_1410e9d90` (mode 1).

use crate::error::{NetError, Result};
use crate::{PacketReader, PacketWriter};

/// What the crafting window sends. `FUN_1406ed520(buf, 0x2f6)` at `1410e9c65`, `1410e9d43`
/// and `1410e9db3`. **[L]**
pub const CLIENT_CRAFT_REQUEST: u16 = 0x02F6;

/// What the server answers with. User-pool table `D` (`FUN_14289a3a0`) routes
/// `op 0x0398 idx 0xd3` to the crafting window's `FUN_1410e7970`. **[L]**
pub const CRAFT_RESULT: u16 = 0x0398;

/// The six professions, in the client's own order - `FUN_1401d24c0`. **[L]**
pub const PROFESSION_SKILLS: [u32; 6] = [92_000_000, 92_010_000, 92_020_000, 92_030_000, 92_040_000, 92_050_000];

/// The highest level a profession reaches: `Skill/9200.img`'s `common/maxLevel`. **[L]**
pub const MAX_PROFESSION_LEVEL: u32 = 10;

/// The profession index (0..5) a skill id belongs to, or `None` for any other skill.
///
/// ```
/// use net::craft::profession_of_skill;
/// assert_eq!(profession_of_skill(92_020_000), Some(2));
/// assert_eq!(profession_of_skill(1_000), None);
/// ```
pub fn profession_of_skill(skill_id: u32) -> Option<u8> {
    PROFESSION_SKILLS.iter().position(|&s| s == skill_id).map(|i| i as u8)
}

/// The skill a profession index grants, or `None` past the sixth.
pub fn profession_skill(profession: u8) -> Option<u32> {
    PROFESSION_SKILLS.get(profession as usize).copied()
}

/// The mastery exp a profession skill's record carries under its level.
///
/// 24 bits, because that is what `FUN_1407b5350` masks off (`raw & 0x00FFFFFF`).
pub const MAX_MASTERY_EXP: u32 = 0x00FF_FFFF;

/// **A profession skill's `level` field is `(level << 24) | masteryExp`.**
///
/// `FUN_1407b3df0` takes `raw >> 24` as the level for a mastery skill and `FUN_1407b5350`
/// takes `raw & 0xFFFFFF` as the exp - the same `entry+0x14`, read twice. **[D]**
///
/// ```
/// use net::craft::{packed_mastery, unpack_mastery};
/// assert_eq!(packed_mastery(1, 10), (1 << 24) | 10);
/// assert_eq!(unpack_mastery(packed_mastery(3, 200)), (3, 200));
/// ```
pub fn packed_mastery(level: u32, exp: u32) -> u32 {
    (level.min(0xFF) << 24) | exp.min(MAX_MASTERY_EXP)
}

/// The inverse of [`packed_mastery`], for reading a record back.
pub fn unpack_mastery(packed: u32) -> (u32, u32) {
    (packed >> 24, packed & MAX_MASTERY_EXP)
}

// ---------------------------------------------------------------------------------------
// 0x02F6 - the request
// ---------------------------------------------------------------------------------------

/// The `count` a **Craft All** carries: `FUN_1410e9c00`'s `param_2 != 0` arm writes
/// `0xffffffff` instead of the quantity box. **[L]**
pub const CRAFT_ALL: u32 = 0xFFFF_FFFF;

/// What the crafting window asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CraftRequest {
    /// Mode 0: begin one craft. The window then animates `ProcessTimeMS` and sends
    /// [`CraftRequest::Complete`].
    Begin {
        /// 0..5, the tab. Written as four raw bytes at `1410e9c47`.
        profession: u32,
        /// `(craftLevel + profession * 10) * 1000 + index` - see the module docs.
        recipe_key: u32,
        /// The ADDITIVE checkbox.
        use_additive: bool,
        /// The quantity box, or [`CRAFT_ALL`]. **The client loops by itself**: it sends one
        /// Begin/Complete pair per item, so this is how many it *intends*, not how many this
        /// one pair makes.
        count: u32,
    },
    /// Mode 1: the window was closed, or the player cancelled.
    Cancel,
    /// Mode 3: `ProcessTimeMS` has elapsed - finish the craft that is running.
    Complete,
    /// Any other mode. Kept rather than refused, because **always answer**: an unanswered
    /// packet freezes the client's whole UI.
    Unknown(u32),
}

impl CraftRequest {
    /// Parse the body **after** the opcode.
    ///
    /// ```
    /// use net::craft::CraftRequest;
    /// let mut body = Vec::new();
    /// body.extend_from_slice(&0u32.to_le_bytes());      // mode 0
    /// body.extend_from_slice(&2u32.to_le_bytes());      // Tailoring
    /// body.extend_from_slice(&21_000u32.to_le_bytes()); // level 1, index 0
    /// body.push(1);                                     // use the additive
    /// body.extend_from_slice(&3u32.to_le_bytes());      // three of them
    /// assert_eq!(
    ///     CraftRequest::parse(&body).unwrap(),
    ///     CraftRequest::Begin { profession: 2, recipe_key: 21_000, use_additive: true, count: 3 },
    /// );
    /// ```
    pub fn parse(body: &[u8]) -> Result<Self> {
        let mut r = PacketReader::new(body);
        let mode = r.u32()?;
        match mode {
            0 => Ok(CraftRequest::Begin {
                profession: r.u32()?,
                recipe_key: r.u32()?,
                use_additive: r.u8()? != 0,
                count: r.u32()?,
            }),
            1 => Ok(CraftRequest::Cancel),
            3 => Ok(CraftRequest::Complete),
            other => Ok(CraftRequest::Unknown(other)),
        }
    }
}

/// A body that is not even four bytes long is not a mode. Kept as its own check so the
/// caller can still answer something rather than drop the packet.
pub fn require_a_mode(body: &[u8]) -> Result<()> {
    if body.len() < 4 {
        return Err(NetError::PacketUnderflow { offset: 0, need: 4, have: body.len() });
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// 0x0398 - the answer
// ---------------------------------------------------------------------------------------

/// Why a craft did not happen - and the sentence the client draws for it.
///
/// `FUN_1410ec340` maps the code to a string id; the text is this client's own, decrypted
/// from its string table. **[L]** Codes 1, 2, 3, 4, 6, 7 and 9 (the mask `0x2de`) also reset
/// the window; **code 5 deliberately does not**, so it must not be used for anything but "a
/// craft is already running".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CraftResult {
    /// No message; the craft happened.
    Ok = 0,
    /// "An unknown error has occurred."
    UnknownError = 1,
    /// "An error occurred. Please try again."
    TryAgain = 2,
    /// "You do not have enough materials to craft this item."
    NotEnoughMaterials = 3,
    /// "You do not have enough inventory space to craft this item."
    NotEnoughSpace = 4,
    /// "Crafting Process is already underway. Please wait until it's finished before
    /// crafting again."
    AlreadyCrafting = 5,
    /// "Your skill level is not high enough to craft this item."
    SkillTooLow = 6,
    /// "You do not have enough mesos to craft this item."
    NotEnoughMesos = 7,
    /// "Crafting is only available near crafting tools."
    NotNearTools = 9,
    /// "This function is currently unavailable."
    Unavailable = 10,
}

impl CraftResult {
    /// The number on the wire.
    pub fn code(self) -> u32 {
        self as u32
    }
}

/// Mode 4: the answer to [`CraftRequest::Begin`]. `Ok` starts the client's animation.
pub fn craft_begin_result(result: CraftResult) -> Vec<u8> {
    two_u32(4, result.code())
}

/// Mode 7: the answer to [`CraftRequest::Complete`]. `Ok` increments the window's counter
/// and, if more were asked for, makes it send the next [`CraftRequest::Begin`] by itself.
pub fn craft_complete_result(result: CraftResult) -> Vec<u8> {
    two_u32(7, result.code())
}

/// Mode 5: put the window back to idle. The answer to [`CraftRequest::Cancel`].
pub fn craft_cancelled() -> Vec<u8> {
    one_u32(5)
}

/// Mode 6: clear the request latch and change nothing else.
///
/// This is the answer for a mode this server does not model: it unblocks the client without
/// pretending a craft finished. **Always answer.**
pub fn craft_acknowledged() -> Vec<u8> {
    one_u32(6)
}

fn one_u32(mode: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(mode);
    w.into_vec()
}

fn two_u32(mode: u32, v: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(mode);
    w.u32(v);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The three modes the window sends, round-tripped from bodies built by hand.
    #[test]
    fn the_three_request_modes_parse() {
        let mut begin = 0u32.to_le_bytes().to_vec();
        begin.extend_from_slice(&0u32.to_le_bytes());
        begin.extend_from_slice(&1_000u32.to_le_bytes());
        begin.push(0);
        begin.extend_from_slice(&CRAFT_ALL.to_le_bytes());
        assert_eq!(
            CraftRequest::parse(&begin).unwrap(),
            CraftRequest::Begin { profession: 0, recipe_key: 1_000, use_additive: false, count: CRAFT_ALL }
        );
        assert_eq!(CraftRequest::parse(&1u32.to_le_bytes()).unwrap(), CraftRequest::Cancel);
        assert_eq!(CraftRequest::parse(&3u32.to_le_bytes()).unwrap(), CraftRequest::Complete);
        assert_eq!(CraftRequest::parse(&9u32.to_le_bytes()).unwrap(), CraftRequest::Unknown(9));
        // A short body is refused rather than read past.
        assert!(CraftRequest::parse(&[0u8; 3]).is_err());
        // A Begin that stops after the mode is a truncation, not a Begin with zeroes.
        assert!(CraftRequest::parse(&0u32.to_le_bytes()).is_err());
    }

    /// The four answers are the mode and, for the two that carry one, the result code.
    #[test]
    fn the_answers_are_a_mode_and_maybe_a_code() {
        assert_eq!(craft_begin_result(CraftResult::Ok), vec![4, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            craft_complete_result(CraftResult::NotEnoughMesos),
            vec![7, 0, 0, 0, 7, 0, 0, 0]
        );
        assert_eq!(craft_cancelled(), vec![5, 0, 0, 0]);
        assert_eq!(craft_acknowledged(), vec![6, 0, 0, 0]);
        // The code that must never be sent for anything but "already running" is 5, and it
        // is the only one the client does NOT reset the window on.
        assert_eq!(CraftResult::AlreadyCrafting.code(), 5);
        assert_eq!(CraftResult::NotNearTools.code(), 9);
    }

    /// The six skills, and the packing the record carries them under.
    #[test]
    fn the_professions_and_their_packed_mastery() {
        for (i, id) in PROFESSION_SKILLS.iter().enumerate() {
            assert_eq!(profession_of_skill(*id), Some(i as u8));
            assert_eq!(profession_skill(i as u8), Some(*id));
        }
        assert_eq!(profession_skill(6), None);
        // Level 1 with no mastery is 0x01000000 - NOT 1, which is what a record that forgot
        // the packing would send, and which the client would read as level 0.
        assert_eq!(packed_mastery(1, 0), 0x0100_0000);
        assert_eq!(unpack_mastery(0x0A00_0032), (10, 50));
        // The exp is 24 bits; a larger number is clamped rather than allowed to carry into
        // the level byte.
        assert_eq!(unpack_mastery(packed_mastery(2, u32::MAX)), (2, MAX_MASTERY_EXP));
    }
}
