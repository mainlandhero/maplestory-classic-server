//! Death and revive.
//!
//! # The client never opens the revive dialog by itself
//!
//! The owner, 2026-08-21: *"My HP hit 0, I see the tombstone on my character, but I do not see the
//! revive confirmation."* The tombstone comes from `hp = 0` in a `0x007C`; the dialog does
//! not, and nothing the client does on its own will open it. **A packet opens it, and the
//! opcode is [`SHOW_REVIVE_DIALOG`] = `0x0315`.** `research/revive.md`.
//!
//! The chain to it was enumerated twice with different blind spots. `CUIRevive` (0x260 bytes,
//! vtable `0x14338acf8`, `UI/Revive.img` loaded through slot 4) is constructed at **exactly
//! one site**, `0x142cb5d77` inside `FUN_142cb5cb0` - `tools/callers.py` reports 1 call, 0
//! tail jmps and 0 data pointers, and `tools/dataref.py` on the dialog global `0x143acaa78`
//! finds 13 references with a **single write**, in that constructor. That constructor's only
//! reachable caller is index `0x50` of the local-user table at `0x14289d660`, and
//! `0x2C5 + 0x50 = 0x0315`. *Control:* index `0xC` decodes to `0x02D1`, which
//! `research/level-up.md` had established independently.
//!
//! # `hp = 0` must arrive first, and that is a real ordering constraint
//!
//! The handler gates on a **client-side HP test of the server's own value**:
//! `0x14289b732 lea rcx,[rax+0x5b] / mov edx,[rax+0x63] / call 0x1401ba9d0 / test eax,eax /
//! jg <skip>`, where `rax = world[0x2358]` - the same field `0x007C` bit 10 writes. So a
//! `0x0315` that arrives while the client still believes HP is positive is **silently
//! dropped**. Send the `0x007C` first. `hp = 0` is necessary, not sufficient.
//!
//! # Two of the eight fields matter and the rest are zero
//!
//! Eight unconditional reads, cross-checked between the in-switch arm and its out-of-line
//! duplicate `FUN_142903cd0`, and against `tools/reads.py`, which reports exactly these eight
//! in this order: **[L]**
//!
//! ```text
//! u32 a   bit 0 must be SET     - otherwise the handler returns without opening anything
//! u32 b   must NOT be 9         - 9 takes a different arm
//! u32 c   unused by the opener
//! u8  d   unused
//! u32 e   seconds; the client multiplies by 1000
//! u32 f   seconds
//! u8  g   unused
//! u8  h   unused
//! ```
//!
//! **Both failure modes are completely silent**, which is why this module sends a fixed body
//! rather than exposing the fields: there is no reply to tell you that you got `a` wrong.
//!
//! # The free instrument: `0x02C6`
//!
//! When the dialog does **not** end up open, the client reports it outbound as
//! [`CLIENT_REVIVE_REPORT`] = `0x02C6`, 44 bytes, carrying the mode (`0` = "my HP was still
//! positive", `2` = refused), the opener's return code (`1` = not in a field stage, `2` =
//! stage busy, `4` = the 2000 ms UI block every revive click sets), the HP the client thinks
//! it has, and field `b` echoed back.
//!
//! **Success is silence.** So a run that produces no `0x02C6` and no dialog means something
//! other than the opener refused, and a run that produces one tells you which of four things
//! went wrong without a second launch. That is why this needed no opcode sweep.

use crate::packet::PacketWriter;

/// `0x0315` - open the revive dialog. See the module docs.
pub const SHOW_REVIVE_DIALOG: u16 = 0x0315;

/// `0x01E7` - the client asking to revive **on the spot** rather than in town.
///
/// Body is `u8 0, u8 <dialog+0x240>`. In practice the second byte is `0` for any dialog this
/// server opens, and the button is **hidden** at create time unless an obfuscated counter at
/// `world[0x2368]+0x200` is greater than zero - the same counter re-centres the town button
/// when it is absent. That counter being the Respawn Token count is **[D]**, from string
/// `0x0857`; nothing this server sends alters it, so expect the town button and only the town
/// button. Answered anyway, because an unanswered request freezes the client's whole UI.
pub const CLIENT_REVIVE_ON_SPOT: u16 = 0x01E7;

/// `0x02C6` - the client reporting that the revive dialog did **not** open. See the module
/// docs; success is silence.
pub const CLIENT_REVIVE_REPORT: u16 = 0x02C6;

/// The body length, fixed. Eight fields: `4+4+4+1+4+4+1+1`.
pub const SHOW_REVIVE_DIALOG_LEN: usize = 23;

/// Bit 0 of field `a`. Without it the handler returns having opened nothing, and says so to
/// nobody.
const OPEN_FLAG: u32 = 1;

/// Build the `0x0315` that opens the revive dialog.
///
/// The body is fixed on purpose. Of the eight fields only two are read for anything the
/// server cares about - `a`'s bit 0, and `b` not being `9` - and getting either wrong fails
/// **silently**, so exposing them as parameters would be handing a caller two ways to produce
/// a dialog that never appears and no diagnostic. The two timer fields are seconds and `0`
/// means no timer.
///
/// **Send a `0x007C` carrying `hp = 0` before this**, or the client's own HP test drops it.
pub fn show_revive_dialog() -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(OPEN_FLAG); // a - bit 0 set, or nothing opens
    w.u32(0); // b - anything but 9
    w.u32(0); // c
    w.u8(0); // d
    w.u32(0); // e - seconds, 0 = no timer
    w.u32(0); // f - seconds
    w.u8(0); // g
    w.u8(0); // h
    w.into_vec()
}

/// How much experience death costs, as a percentage of the character's current total.
///
/// The owner, 2026-08-21: *"reduce their EXP by 10% unless they are level 10 or below."* Both
/// halves are theirs; neither is read out of the client.
pub const DEATH_EXP_PENALTY_PERCENT: u64 = 10;

/// The highest level that pays no experience penalty on death. The owner's number.
pub const DEATH_PENALTY_FREE_MAX_LEVEL: u32 = 10;

/// The HP a revived character comes back with. The owner's number.
pub const REVIVE_HP: u32 = 50;

/// What death costs `exp` at `level`, in experience points.
///
/// Returns `0` at or below [`DEATH_PENALTY_FREE_MAX_LEVEL`]. Truncating division, so a
/// character with less than ten experience loses none - which is the right way round: the
/// alternative rounds a beginner's whole total away.
pub fn death_exp_loss(level: u32, exp: u64) -> u64 {
    if level <= DEATH_PENALTY_FREE_MAX_LEVEL {
        return 0;
    }
    exp * DEATH_EXP_PENALTY_PERCENT / 100
}

/// **`0x02D1` effect `0x41` - the blue number over the player's head.**
///
/// The owner, 2026-08-21: *"the idle recovery should pop up with a blue number of the recovery
/// amount above the player's head."*
///
/// # This is not the `0x007C` recovery trailer, and that trailer can never draw anything
///
/// The obvious-looking route was `0x007C`'s second optional trailer, `hpRecovery`/`mpRecovery`,
/// which the client hands to `FUN_140fd31f0`. It was sent, three times in one run, on a bar
/// that visibly moved - and nothing was drawn. `research/recovery-number.md` says why:
/// **`FUN_140fd31f0` is not a renderer.** Its entire call list is two tick functions, a
/// getter twice, and a tail `jmp` to an hour-rollover. It accumulates the recovery into
/// running totals at `+0x208`/`+0x210`, clamps each against `maxHp - oldHp` to separate
/// effective healing from wasted, keeps per-hour averages, and resets after `0x36ee80` ms.
/// It is a **statistics counter**. No body of that packet was ever going to put a number on
/// screen.
///
/// The intermediate theory - that the `oldHp` snapshot is taken *after* the mask block stores
/// the new total, so the delta is zero - is also dead, and deliberately recorded because it
/// would have cost a client run to test. The snapshots are taken **65 bytes earlier**, at
/// `142d5485c`/`142d5486b`, into `[rbp-0x58]` and callee-saved `r15`, where the mask block at
/// `142d5489d` cannot reach them. Dropping the hp/mp bits would have produced the identical
/// blank screen.
///
/// # What does draw it
///
/// `FUN_142771360(pUser, N, ...)` forks on the **sign** of `N` at `142771489`: positive
/// selects digit set **2 = `NoBlue`**, negative set **3 = `NoViolet`**. Of the eleven call
/// sites of the digit-set loader `FUN_140e0ccb0`, exactly one ever asks for set 2, and it is
/// that positive branch. So the blue number is *the same renderer as the damage number, with
/// a positive argument*.
///
/// Three local-user packets reach it with a positive `N`. This is the first: effect `0x41`,
/// body `i32 amount, i32 delayMs, i32 id`, queued on `pUser+0x39b8` and drained by
/// `CUser::Update`. It is the one where **the server picks the number**, which the potion
/// effect `0x02F2` does not - that reads the amount out of the WZ `spec` node.
///
/// # The gate that is not measured
///
/// Effects `0x41` and `0x23` both pass a suppression check at `14278bd75`
/// (`FUN_142826340` on the user, a field flag byte). Both flags should be zero for a visible
/// character on an ordinary map, and **neither has been measured**. If the hook log shows
/// `0x02D1` dispatched *and returned* with nothing on screen, that gate is where to look -
/// and swapping `0x41` for `0x23` does **not** test it, because they share it. **[I]**.
pub const EFFECT_RECOVERY_NUMBER: u8 = 0x41;

/// The body length of [`recovery_number`]: the effect byte plus three `i32`s.
pub const RECOVERY_NUMBER_LEN: usize = 1 + 4 + 4 + 4;

/// Build the `0x02D1` that floats a blue `amount` over the player.
///
/// `delay_ms` is how long the client waits before drawing it; `0` is immediate. The trailing
/// id is an effect discriminator the drain loop carries and this server has no use for.
///
/// Send it **after** the `0x007C` that moved the bar, so the number and the bar agree on
/// screen. Nothing here reads the character's HP - the caller supplies the amount, and the
/// amount is what was *recovered*, not the new total.
pub fn recovery_number(amount: i32, delay_ms: i32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(EFFECT_RECOVERY_NUMBER);
    w.u32(amount as u32);
    w.u32(delay_ms as u32);
    w.u32(0);
    w.into_vec()
}

// ===========================================================================================
// Making the client run its own console command
// ===========================================================================================

/// `0x00EA` - **a string the client splits on carriage returns and runs as console commands.**
///
/// # The name in the opcode table is a claim, and it is not what the code does
///
/// `research/msexe-gamestage-opcodes.md` calls this *ScriptProgressMessage*. `FUN_142D9FA90`
/// reads one string, splits it on `"\r"`, echoes each line into the chat log as `> <line>` -
/// that is the "message" the name saw - and then hands **each line to the slash-command
/// dispatcher** at `0x1418CD030` -> `0x14202E460`. The dispatch is not in the name.
/// `CLAUDE.md`'s *"a table row written from a quick read is a claim"*, again. **[L]**
///
/// # What this is for
///
/// The owner, 2026-08-28: *"I really need this 1 to go away to make this portion perfect."*
///
/// The client draws its own stub damage number at **one instruction**, `0x1428ACA14`, the only
/// renderer call in a 14 497-byte function - and that call sits inside a `0x356`-byte block
/// gated on `user+0x544a`. Clear the byte and the block is skipped whole.
///
/// The byte has a **second writer** nobody had found: `0x142883687` stores a `qword` at
/// `[rsi+0x5448]`, and `0x5448 + 2` is `0x544a`. An earlier pass looked for stores whose
/// displacement *equalled* `0x544a` and honestly reported "one writer" with three named blind
/// spots. All three were real and none of them was this: a **wider store at a lower
/// displacement**. The fix was to stop asking *which operand equals this address* and start
/// asking *which operand's byte range covers it*.
///
/// The command is **`/hitdamagetest`**, name string at `0x1434262A8`, description *"Test hit
/// damage"*, and its handler writes `user+0x544a = (atoi(argv[0]) != 0)`.
pub const RUN_CONSOLE_COMMAND: u16 = 0x00EA;

/// The command that turns the client's own damage number off. See [`RUN_CONSOLE_COMMAND`].
pub const HIDE_HIT_DAMAGE: &str = "/hitdamagetest 0";

/// Build a `0x00EA`. The body is one string; the client splits it on `"\r"` itself.
///
/// **Every line is echoed into the chat window as `> <line>` before it is dispatched**, so
/// this is not silent and should not be sent repeatedly.
pub fn run_console_command(command: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.str(command);
    w.into_vec()
}

/// `0x0189` - what the client sends back after `/hitdamagetest` has run.
///
/// `u32 0x13D`, then the `u8` flag it just stored. **This is the positive control**: the
/// command's permission gate is `[D]`, not `[L]`, so the only way to know it passed is that
/// the client tells us. No `0x0189` means the gate refused and the number is still drawn.
pub const CONSOLE_COMMAND_RESULT: u16 = 0x0189;

/// The `u32` that identifies a `/hitdamagetest` result inside [`CONSOLE_COMMAND_RESULT`].
pub const HIT_DAMAGE_TEST_ID: u32 = 0x13D;

#[cfg(test)]
mod tests {
    use super::*;

    /// **The console packet is one string and the command is the one that clears the flag.**
    ///
    /// The command text is pinned because it is the whole packet: `/hitdamagetest` with a
    /// **zero** argument. Its handler stores `atoi(argv[0]) != 0`, so `/hitdamagetest 1` would
    /// switch the client's stub damage number back ON - the opposite of what the owner asked for,
    /// from a one-character difference.
    #[test]
    fn the_console_command_turns_the_stub_number_off_not_on() {
        assert_eq!(HIDE_HIT_DAMAGE, "/hitdamagetest 0");
        assert!(HIDE_HIT_DAMAGE.ends_with(" 0"), "a 1 here would turn the number ON");

        let b = run_console_command(HIDE_HIT_DAMAGE);
        assert_eq!(u16::from_le_bytes([b[0], b[1]]) as usize, HIDE_HIT_DAMAGE.len());
        assert_eq!(&b[2..], HIDE_HIT_DAMAGE.as_bytes(), "one string, nothing else");

        // The client splits on carriage returns, so a command must not contain one - it would
        // become two commands, and the second would be whatever followed.
        assert!(!HIDE_HIT_DAMAGE.as_bytes().contains(&13u8), "one line, one command");
    }

    /// Twenty-three bytes, and the two fields that matter carry what the handler tests for.
    ///
    /// The length is asserted against the sum of the eight documented widths rather than
    /// against `23` alone, so a field whose width is corrected in the doc block cannot leave
    /// a stale total passing beside it.
    #[test]
    fn the_revive_dialog_body_is_the_shape_the_handler_reads() {
        let b = show_revive_dialog();
        assert_eq!(b.len(), 4 + 4 + 4 + 1 + 4 + 4 + 1 + 1);
        assert_eq!(b.len(), SHOW_REVIVE_DIALOG_LEN);

        let a = u32::from_le_bytes(b[0..4].try_into().unwrap());
        assert_eq!(a & 1, 1, "bit 0 of field a must be set or nothing opens");

        let second = u32::from_le_bytes(b[4..8].try_into().unwrap());
        assert_ne!(second, 9, "field b must not be 9 - that is a different arm");
    }

    /// The penalty is the owner's rule, and the boundary is the half worth pinning: level 10 pays
    /// nothing and level 11 pays.
    #[test]
    fn death_costs_a_tenth_of_experience_above_level_ten() {
        assert_eq!(death_exp_loss(1, 5_000), 0);
        assert_eq!(death_exp_loss(10, 5_000), 0, "level 10 is inclusive - 'level 10 or below'");
        assert_eq!(death_exp_loss(11, 5_000), 500);
        assert_eq!(death_exp_loss(50, 1_234), 123, "truncating, not rounding");
    }

    /// A character with almost nothing loses nothing rather than all of it.
    #[test]
    fn a_tiny_experience_total_is_not_wiped() {
        assert_eq!(death_exp_loss(20, 9), 0);
        assert_eq!(death_exp_loss(20, 0), 0);
    }

    /// Thirteen bytes: the effect byte and three `i32`s, with the amount where the drain loop
    /// reads it.
    ///
    /// The sign is the whole mechanism - `FUN_142771360` forks on it to choose digit set 2
    /// (blue) over set 3 (violet) - so a builder that silently made a negative amount
    /// positive would draw the wrong colour and nothing would say so.
    #[test]
    fn the_recovery_number_carries_a_signed_amount() {
        let b = recovery_number(10, 0);
        assert_eq!(b.len(), RECOVERY_NUMBER_LEN);
        assert_eq!(b[0], EFFECT_RECOVERY_NUMBER);
        assert_eq!(i32::from_le_bytes(b[1..5].try_into().unwrap()), 10);
        assert_eq!(i32::from_le_bytes(b[5..9].try_into().unwrap()), 0, "no delay");

        let negative = recovery_number(-7, 250);
        assert_eq!(
            i32::from_le_bytes(negative[1..5].try_into().unwrap()),
            -7,
            "the sign must survive - it is what selects the colour"
        );
        assert_eq!(i32::from_le_bytes(negative[5..9].try_into().unwrap()), 250);
    }
}
