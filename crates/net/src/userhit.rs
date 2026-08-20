//! `0x00E5` - the client reporting that the player just took damage.
//!
//! **The client computes the damage and does NOT apply it.** That is the half that matters:
//! The owner was walked into by snails and their HP bar did not move, because nothing moved it.
//! `research/user-hit.md` establishes it three ways with different blind spots - an
//! exhaustive scan of every `.pdata` extent for writes to the HP field (18, with a control),
//! every `lea` of that field in the image (196, all feeding the *getter*), and a reachability
//! walk from the hit path (5716 functions, depth 8, zero HP writers, four controls reached).
//!
//! So the server is the authority: read the damage, subtract it, and send the new HP back in
//! a `0x007C`. The client is waiting to be told.
//!
//! # Not a latch
//!
//! Three of these went unanswered in the run of 2026-08-20 and the client played on for
//! minutes. That is also established from the listing rather than from the observation alone:
//! neither builder, nor the resolver tree, nor `FUN_142771360` touches the `+0x2330` latch
//! machinery. So a body this parser rejects costs a missed hit, not a dead session.
//!
//! # Length
//!
//! **At least** 147 bytes, not exactly. `FUN_142907b60` - a fifteenth builder, missing from
//! `research/msexe-send-opcodes.txt` entirely - appends a second struct. Requiring equality
//! would silently drop those.

/// The client telling us the player was hit.
pub const CLIENT_USER_HIT: u16 = 0x00E5;

/// The shortest body this can be. See the module docs on why it is a minimum.
pub const USER_HIT_MIN_LEN: usize = 147;

/// `u32` damage. **[D]**, and see [`UserHit::damage`] for what is and is not pinned.
const DAMAGE_AT: usize = 8;
/// `u32` the mob's object id - the one this server minted. **[L]**, three captures.
const MOB_OBJECT_ID_AT: usize = 46;
/// `u32` the mob's template id. **[L]**, three captures.
const MOB_TEMPLATE_AT: usize = 113;

/// The attack index, at body offset 4. `-1` is the body/touch attack - a mob with no
/// `attack<N>` node in its WZ, which is every snail. **[L]**
pub const ATTACK_INDEX_AT: usize = 4;

/// Who hit the player, for how much.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserHit {
    /// How much damage the client says it took.
    ///
    /// **Not discriminated by the captures.** All three carry damage `1` - a snail's
    /// `PADamage` - and `1` also appears at offsets 30, 34, 38, 42, 78 and 133 of the same
    /// bodies, so the value alone cannot pick this field out. Offset 8 is chosen because it
    /// is the `u32` immediately after the attack index, which is where `research/user-hit.md`
    /// places it from the serialiser. **A capture with damage other than 1 would settle it**,
    /// and until then a wrong reading here shows up as the wrong number of HP lost rather
    /// than as anything dramatic.
    pub damage: u32,
    /// The mob that did it - **our** object id, which is how this is known to be right.
    pub mob_object_id: u32,
    /// Its template id.
    pub mob_template_id: u32,
    /// `-1` means the body/touch attack rather than a numbered skill.
    pub attack_index: i32,
}

impl UserHit {
    /// Was this contact damage rather than a numbered mob attack?
    pub fn is_touch(&self) -> bool {
        self.attack_index == -1
    }
}

fn u32_at(body: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?))
}

/// Parse a [`CLIENT_USER_HIT`] body (opcode already stripped).
///
/// `None` for anything shorter than [`USER_HIT_MIN_LEN`]. Rejecting is safe here - see the
/// module docs on why this packet is not a latch.
pub fn parse_user_hit(body: &[u8]) -> Option<UserHit> {
    if body.len() < USER_HIT_MIN_LEN {
        return None;
    }
    Some(UserHit {
        damage: u32_at(body, DAMAGE_AT)?,
        mob_object_id: u32_at(body, MOB_OBJECT_ID_AT)?,
        mob_template_id: u32_at(body, MOB_TEMPLATE_AT)?,
        attack_index: u32_at(body, ATTACK_INDEX_AT)? as i32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The three real bodies from `research/fixtures/pickup-opcode-032c-named-world.log`.
    const CAPTURES: [(&str, u32); 3] = [
        ("00000000ffffffff0100000002002100b663ed0a0000000000000000000001000000010000000100000001000000d4070000d407000001000000000000000000000000000000000000490300008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000", 2004),
        ("00000000ffffffff0100000096048598f262ee0a0000000000000000000001000000010000000100000001000000d0070000d0070000000000000000000000000000000000000000006e0300008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000", 2000),
        ("00000000ffffffff01000000b1b02240a087ee0a0000000000000000000001000000010000000100000001000000d0070000d0070000000000000000000000000000000000000000000f0300008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000", 2000),
    ];

    fn body(hex: &str) -> Vec<u8> {
        (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
    }

    /// **Every field is checked against a value only the server could know.** The mob object
    /// ids are ones this server minted - 2000 and 2004 - which is what makes this a decode
    /// rather than a plausible reading of some bytes.
    #[test]
    fn the_three_real_captures_decode() {
        for (hex, expect_id) in CAPTURES {
            let b = body(hex);
            assert_eq!(b.len(), 147, "the captured length");
            let h = parse_user_hit(&b).expect("a real body must parse");
            assert_eq!(h.mob_object_id, expect_id, "the object id THIS SERVER minted");
            assert_eq!(h.mob_template_id, 2, "template 2 is the snail the owner was fighting");
            assert_eq!(h.damage, 1, "a snail's PADamage is 1");
            assert!(h.is_touch(), "no attack node on a snail, so the index is -1");
        }
    }

    /// Offset 4 is the attack index and **not** the damage. Reading it as damage would give
    /// `0xFFFFFFFF` and take the whole health bar off in one touch.
    #[test]
    fn the_attack_index_is_not_mistaken_for_the_damage() {
        let b = body(CAPTURES[0].0);
        assert_eq!(u32_at(&b, ATTACK_INDEX_AT).unwrap(), 0xFFFF_FFFF);
        assert_ne!(u32_at(&b, DAMAGE_AT).unwrap(), 0xFFFF_FFFF);
    }

    /// A short body is refused rather than read out of bounds. Safe because this packet does
    /// not latch the client - a rejected hit costs one hit, not the session.
    #[test]
    fn a_short_body_is_refused() {
        let b = body(CAPTURES[0].0);
        assert!(parse_user_hit(&b[..USER_HIT_MIN_LEN - 1]).is_none());
        assert!(parse_user_hit(&[]).is_none());
    }

    /// **Longer is accepted**, because a fifteenth builder appends a second struct and
    /// requiring exactly 147 would silently drop those hits.
    #[test]
    fn a_longer_body_still_parses() {
        let mut b = body(CAPTURES[0].0);
        b.extend_from_slice(&[0u8; 64]);
        assert_eq!(parse_user_hit(&b).unwrap().mob_object_id, 2004);
    }
}
