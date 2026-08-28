//! **Mob attacks: the one bit that has been off since the packet was written.**
//!
//! Full working, with every address and every measurement: `research/mob-attack-skills.md`.
//! **[L]** is read off the listing or off a capture, **[D]** derived from two or more [L],
//! **[I]** inferred.
//!
//! # THIS MODULE IS NOT WIRED
//!
//! `crates/world/src/session/` belongs to the coordinator, so nothing here is called yet.
//! The single change is `crates/world/src/session/combat.rs`, in `on_mob_move`:
//!
//! ```text
//! body: net::mobmove::mob_ctrl_ack(req.object_id, req.move_id, false),
//! body: net::mobmove::mob_ctrl_ack(req.object_id, req.move_id, crate::mobattack::grant_attack()),
//! ```
//!
//! # Why
//!
//! `CMob::Update` (`FUN_141c626c0`) reads the mob's obfuscated controller state and skips
//! **1 594 bytes** — the whole attack-and-skill selection — unless it is exactly `4`:
//!
//! ```text
//! 141c63824  lea   rcx,[r12 + 0x2e4]     the state
//! 141c6382c  mov   edx,[r12 + 0x2ec]     its key
//! 141c63834  call  0x1401ba9d0           -> the plaintext
//! 141c63839  cmp   eax, 4
//! 141c6383c  jne   0x141c63e7c           <-- not 4: nothing below runs
//! ```
//!
//! Inside that block, and **nowhere else in the function**, are the only two calls that ask
//! a mob template for an attack or a skill: `141c63cf7 GetAttack(template, action - 13)` and
//! `141c63e2a GetSkill(template, mob+0x2f8, mob+0x2fc, 0)`. Every branch and call in the
//! whole 20 427-byte function with a target inside the block — 28 of them — originates
//! inside the block, and the function contains no indirect jump, so `state == 4` dominates
//! the lot. **[L]**
//!
//! That is dominance *within `CMob::Update`*, not within the client: the chooser
//! `FUN_141c7c900` has 13 call sites in 9 functions and a `GetAttack` of its own. It
//! answers through out-parameters rather than setting the mob's action, and `CMob::Update`
//! is the only place a chosen action reaches `FUN_141c551f0` and then the wire — but the
//! claim that rules the other eight out is the archive below, not the branch scan. **[D]**
//!
//! The only server-driven writer of state `4` is the `0x03E4 MobCtrlAck` handler:
//!
//! ```text
//! 141c8221d  test  r13d, r13d      the u8 at MobCtrlAck body offset 6
//! 141c82220  setne bl
//! 141c82223  add   ebx, 3          -> 3 when clear, 4 when set
//! ```
//!
//! and this server has sent `false` there since the packet existed. Three independent
//! measurements over the de-duplicated archive agree that the block has never run:
//!
//! * **131 003** distinct `0x02FF` mob-move reports: **zero** in the attack action range
//!   and **zero** in the skill range. The only two actions a mob has ever performed here
//!   are `-1` (nothing) and `7` = `hit1`, the flinch from the player's own swing.
//! * **257** distinct `0x00E5 USER_HIT`: every one `attackIndex -1`, contact damage.
//! * **`0x0313`** — which the block arms by setting `mob+0xc8c` at `141c63df1`, the field's
//!   only writer of `1` in the mob code range — has never arrived, in 78 distinct inbound
//!   opcodes.

/// Mob animation/action names, indexed by the action number the client uses on the wire.
///
/// Read out of the image, not from a reference server. The runtime table lives at
/// `0x143AAA670` and is **BSS**, so it cannot be read from the file; `FUN_140009690` fills
/// it one slot at a time (`mov rdx,[rip+A] / lea rcx,[rip+B] / call 0x1401a5890`, `B`
/// walking the table by 8), and the `.data` slots `A` points at *do* have file bytes.
/// **[L]**
///
/// *Positive control.* `FUN_141cc5b80` accepts exactly `idx - 0x33 <= 0xf`, i.e. 51..66 and
/// nothing else, and is called with `[skill+0x28] + 0x33`. This table says 51..66 is
/// `skillAfter1..skillAfter16` — sixteen entries, exactly that window, and exactly what you
/// look up after a skill. `the_action_table_matches_the_clients_own_range_checks` pins it.
pub const ACTION_NAMES: [&str; 86] = [
    "move", "stand", "jump", "fly", "rope", "regen",
    "bomb", "hit1", "hit2", "hitF", "die1", "die2",
    "dieF", "attack1", "attack2", "attack3", "attack4", "attack5",
    "attack6", "attack7", "attack8", "attack9", "attack10", "attack11",
    "attack12", "attack13", "attack14", "attack15", "attack16", "attackF",
    "skill1", "skill2", "skill3", "skill4", "skill5", "skill6",
    "skill7", "skill8", "skill9", "skill10", "skill11", "skill12",
    "skill13", "skill14", "skill15", "skill16", "skillF", "chase",
    "miss", "say", "eye", "skillAfter1", "skillAfter2", "skillAfter3",
    "skillAfter4", "skillAfter5", "skillAfter6", "skillAfter7", "skillAfter8", "skillAfter9",
    "skillAfter10", "skillAfter11", "skillAfter12", "skillAfter13", "skillAfter14",
    "skillAfter15", "skillAfter16", "sleep", "wakeup", "patrolSense",
    "patrolUserdetect", "patrolAttractdetect", "patrolAttractarrive", "transform",
    "skillUse", "skillFail", "revive", "sealed", "runaway", "directionAct1",
    "remove", "flip", "groggy", "heal", "forceChange", "forceChangeAfter",
];

/// First action that is a numbered mob attack: `attack1`.
///
/// `141cb6a0a LEA EAX,[R15-0xd] / CMP EAX,0x10 / JA` in the move builder, and the identical
/// `141c63ce3 ADD EDX,-0xd / CMP EDX,0x10 / JA` in the selector. The template lookup is
/// `GetAttack(action - 13)`, so `attack1` is **attack index 0**. **[L]**
pub const ATTACK_ACTION_FIRST: i16 = 13;
/// Last numbered-attack action: `attackF`. Seventeen in all — `cmp .., 0x10` is inclusive.
pub const ATTACK_ACTION_LAST: i16 = 29;

/// First action that is a mob skill: `skill1`. `141c63e0c ADD EAX,-0x1e / CMP EAX,0x10 / JA`.
/// **[L]**
pub const SKILL_ACTION_FIRST: i16 = 30;
/// Last mob-skill action: `skillF`.
pub const SKILL_ACTION_LAST: i16 = 46;

/// The action a mob reports when it is doing nothing.
///
/// On the wire this is the byte `0xFF`, and it gets there by a different route from every
/// other action — see [`decode_move_action`].
pub const ACTION_NONE: i16 = -1;

/// Split `0x02FF` body offset 7 (equally, `0x03D9` offset 5) into `(action, facing)`.
///
/// # The rule is a three-instruction function, not a convention
///
/// ```text
/// 1402b3b30  cmp   ecx, 0x55       ; the action, UNSIGNED
/// 1402b3b33  setbe al
/// 1402b3b36  ret
/// ```
///
/// called at `141cb7e88` in the move builder. On the true side the byte is packed
/// (`141cb7e97 ADD BL,BL / OR AL,BL` — `action * 2 | facing`); on the false side the raw
/// action is written straight out (`141cb7ea1 MOV byte [RSP+0x5c],BL`). **[L]**
///
/// `-1` is `0xFFFFFFFF` unsigned, so "no action" takes the *raw* side and arrives as
/// `0xFF`. That asymmetry is what makes the reading unambiguous: a packed `-1` would be
/// `0xFE` or `0xFF` depending on facing, and across 131 003 archived reports `0xFE` never
/// appears while `0xFF` appears 130 731 times.
///
/// Facing is meaningless for the raw case, so it is reported as `0`.
pub fn decode_move_action(byte: u8) -> (i16, u8) {
    // The packed range is action 0..=0x55, i.e. bytes 0..=0xAB.
    if byte <= 0x55 * 2 + 1 {
        (i16::from(byte >> 1), byte & 1)
    } else {
        (i16::from(byte as i8), 0)
    }
}

/// Is this action one of `attack1..attackF`?
pub fn is_attack_action(action: i16) -> bool {
    (ATTACK_ACTION_FIRST..=ATTACK_ACTION_LAST).contains(&action)
}

/// Is this action one of `skill1..skillF`?
pub fn is_skill_action(action: i16) -> bool {
    (SKILL_ACTION_FIRST..=SKILL_ACTION_LAST).contains(&action)
}

/// `GetAttack`'s argument for an attack action - `attack1` is index **0**. `141c63ce3`.
pub fn attack_index(action: i16) -> Option<u8> {
    is_attack_action(action).then(|| (action - ATTACK_ACTION_FIRST) as u8)
}

/// The client's own name for an action, for a log line that can be read without this file.
pub fn action_name(action: i16) -> &'static str {
    if action < 0 {
        return "none";
    }
    ACTION_NAMES.get(action as usize).copied().unwrap_or("?")
}

/// One line describing what a mob just reported doing. **This is the instrument**: it turns
/// `world.log` into a measurement of whether the change in [`grant_attack`] worked, with no
/// probe and no second launch.
pub fn describe_move_action(byte: u8) -> String {
    let (action, facing) = decode_move_action(byte);
    if action == ACTION_NONE {
        return format!("action {action} (none), byte 0x{byte:02x}");
    }
    let extra = match attack_index(action) {
        Some(i) => format!(" - NUMBERED MOB ATTACK, GetAttack index {i}"),
        None if is_skill_action(action) => " - MOB SKILL".to_string(),
        None => String::new(),
    };
    format!(
        "action {action} ({}) facing {facing}, byte 0x{byte:02x}{extra}",
        action_name(action)
    )
}

/// **May the mob's next action be an attack?** This is `MobCtrlAck` body offset 6.
///
/// # Why this is unconditional, and why that is the *smaller* change
///
/// The client owns every other precondition and checks all of them itself:
///
/// * the mob has attacks at all - `141c7c9a2 CMP dword [template+0x344],0 / JLE bail`, and
///   `template+0x344` is written by the WZ parser right after it reads the node named
///   `attack`;
/// * the per-attack cooldown - `[attack+0x9c]` is added to the current tick at `141c63d09`
///   and tested at `141c63923 CMP EAX,R13D / JG bail`;
/// * range, target and the rest of `FUN_141c7c900`, 17 702 bytes of it.
///
/// And the permission is not a mode: the move builder drops the state to `1` or `2` at
/// `141cb8743` immediately before it sends, so **one ack buys exactly one action.** A
/// server-side rate limiter on top of that would be a second unmeasured variable in the
/// same run, which is the thing `CLAUDE.md` says not to do.
///
/// # It does NOT enable mob skills
///
/// Offsets 11 and 15 of the same packet are the skill id and level, and `net::mobmove`
/// sends `0`. `141c8214b TEST ESI,ESI / JE 141c821c3` short-circuits the whole skill block
/// on a zero id, so flipping this bit unlocks `attack1..attackF` and leaves
/// `skill1..skillF` exactly as inert as they are today. Keep it that way for the first run.
pub fn grant_attack() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The table is only worth having if it reproduces the client's own range checks.
    /// These four are read off the listing, and each one is a `cmp` this table has to agree
    /// with or the base address was wrong.
    #[test]
    fn the_action_table_matches_the_clients_own_range_checks() {
        // 141cc5b80: `LEA EAX,[R14-0x33] / CMP EAX,0xf / JBE` - exactly skillAfter1..16.
        assert_eq!(ACTION_NAMES[0x33], "skillAfter1");
        assert_eq!(ACTION_NAMES[0x33 + 0xf], "skillAfter16");
        // 141cb6a0a / 141c63ce3: action - 0xd <= 0x10.
        assert_eq!(ACTION_NAMES[ATTACK_ACTION_FIRST as usize], "attack1");
        assert_eq!(ACTION_NAMES[ATTACK_ACTION_LAST as usize], "attackF");
        // 141c63e0c: action - 0x1e <= 0x10.
        assert_eq!(ACTION_NAMES[SKILL_ACTION_FIRST as usize], "skill1");
        assert_eq!(ACTION_NAMES[SKILL_ACTION_LAST as usize], "skillF");
        // 141cb6b07 / 141c63355: action - 7 <= 2.
        assert_eq!(
            [ACTION_NAMES[7], ACTION_NAMES[8], ACTION_NAMES[9]],
            ["hit1", "hit2", "hitF"]
        );
        // 1402b3b30 bounds the packed range at 0x55, and the table has 86 = 0x56 entries.
        assert_eq!(ACTION_NAMES.len(), 0x55 + 1);
    }

    /// **The bytes the client actually sent.** Every distinct `0x02FF` in the archive
    /// carries one of exactly three values at body offset 7, and this is what they mean.
    /// If `decode_move_action` ever stops agreeing with this, the histogram in
    /// `research/mob-attack-skills.md` stops meaning what it says.
    #[test]
    fn the_three_bytes_the_archive_contains_decode_the_way_the_finding_needs() {
        // 130 731 reports.
        assert_eq!(decode_move_action(0xff), (ACTION_NONE, 0));
        // 95 and 177 reports: hit1, both facings. The flinch, not an attack.
        assert_eq!(decode_move_action(0x0e), (7, 0));
        assert_eq!(decode_move_action(0x0f), (7, 1));
        assert_eq!(action_name(7), "hit1");
        for b in [0xffu8, 0x0e, 0x0f] {
            let (a, _) = decode_move_action(b);
            assert!(!is_attack_action(a), "byte 0x{b:02x} is not an attack");
            assert!(!is_skill_action(a), "byte 0x{b:02x} is not a skill");
        }
    }

    /// `0xFE` never appears in the archive and `0xFF` appears 130 731 times. That is only
    /// consistent with `-1` taking the *raw* branch of `1402b3b30`, which is the fact the
    /// whole reading rests on - so pin the boundary rather than the two convenient values.
    #[test]
    fn the_packed_range_ends_where_the_unsigned_compare_does() {
        // 0x55 * 2 + 1 = 0xab is the last packed byte: action 0x55, facing 1.
        assert_eq!(decode_move_action(0xab), (0x55, 1));
        // One past it is no longer a packed action; it reads as a raw negative.
        assert_eq!(decode_move_action(0xac), (-84, 0));
        // And `-1` packed would have been 0xfe/0xff; raw it is only ever 0xff.
        assert_eq!(decode_move_action(0xfe), (-2, 0));
    }

    /// What a run with the flag on has to produce for the finding to be right, spelled out
    /// as bytes so the log line can be grepped for.
    #[test]
    fn an_attack_action_is_recognisable_on_the_wire() {
        // attack1 facing 0 .. attackF facing 1 is bytes 0x1a..=0x3b.
        assert_eq!(decode_move_action(0x1a), (13, 0));
        assert_eq!(attack_index(13), Some(0));
        assert_eq!(decode_move_action(0x3b), (29, 1));
        assert_eq!(attack_index(29), Some(16));
        // and the skill band starts immediately after it.
        assert_eq!(decode_move_action(0x3c), (30, 0));
        assert!(is_skill_action(30));
        assert_eq!(attack_index(30), None);

        assert!(describe_move_action(0x1a).contains("NUMBERED MOB ATTACK"));
        assert!(describe_move_action(0x1a).contains("index 0"));
        assert!(describe_move_action(0x3c).contains("MOB SKILL"));
        assert!(!describe_move_action(0xff).contains("ATTACK"));
        assert!(!describe_move_action(0x0f).contains("ATTACK"));
    }

    /// The whole change, so a future edit that "temporarily" turns it off fails here rather
    /// than silently restoring three days of a mob that cannot fight back.
    #[test]
    fn the_grant_is_the_bit_that_141c8221d_turns_into_state_four() {
        assert!(grant_attack(), "141c8221d SETNE BL / ADD EBX,3 -> state 4");
        assert_eq!(u8::from(grant_attack()), 1, "MobCtrlAck body offset 6");
    }

    /// Out-of-range and negative actions must not panic or index past the table.
    #[test]
    fn an_unknown_action_names_itself_rather_than_panicking() {
        assert_eq!(action_name(-1), "none");
        assert_eq!(action_name(-99), "none");
        assert_eq!(action_name(85), "forceChangeAfter");
        assert_eq!(action_name(86), "?");
        assert_eq!(action_name(i16::MAX), "?");
        for b in 0..=u8::MAX {
            let (a, _) = decode_move_action(b);
            let _ = action_name(a);
            let _ = describe_move_action(b);
        }
    }
}
