//! **The damage guard: a hit more than 25% above what this character could deal is CAPPED.**
//!
//! The owner, 2026-10-02: *"make sure that the server has some protection in place so it refuses
//! the damage reported by the player if it exceeds what it expects by 15% or more"*, and *"Please
//! also log the suspecting damage cheaters."* Asked which way to lean when the model is unsure,
//! they first chose to drop the hit, then the same day: *"instead of dropping attacks 15%+ over,
//! change that to 25%+ over, and cap the attack plus log instead of dropping the attack all
//! together."* So a hit over [`limit`] is reduced TO the limit - the most it may be - not to
//! the ceiling: capping at the ceiling would make a hit one point over the line worth less
//! than one point under it.
//!
//! # What "expects" means here
//!
//! This client computes every hit itself (`crate::damage` module docs); the server never knew
//! the number until now. The expectation is the **top** of the window the client's own formula
//! allows - `damage::physical_window` maximised over every attack action, because which action
//! was played is still not parsed - with headroom for the criticals the model knows about:
//!
//! * a hit the client flags **critical** (`AttackHit::flag_b`, **[D]**, `damage::CRIT_MULTIPLIER`
//!   has the measurement) may reach [`CRIT_HEADROOM`] times the top;
//! * any other hit [`damage::CRIT_MULTIPLIER`] times it. That 1.2 is not a crit allowance any
//!   more - it is the margin the model's own unknowns need (the mastery term, the meowdb
//!   constants marked **[I]**), kept so the 25% is the owner's margin and not eaten by ours.
//!
//! Defence, element and the level gap only ever **reduce** a hit, so leaving them out keeps the
//! ceiling a ceiling. Then [`limit`] adds the owner's [`TOLERANCE_PERCENT`].
//!
//! # What is never refused
//!
//! **A hit the model cannot price is applied and logged "unchecked", never dropped.** Refusing
//! on a guess would make honest hits vanish, which on screen is a mob that stops dying - the
//! failure `research/damage-formula.md` §9.1 warned about. Unchecked:
//!
//! * a skill whose `damage` / `mad` column is absent and whose tooltip states no percent;
//! * no weapon, or a weapon class with no multiplier;
//! * any attack opcode but melee, shoot and magic.
//!
//! # And the packet is still answered
//!
//! Dropping a hit means the mob's HP does not move for it. The attack is otherwise handled in
//! full - the MP and ammo are spent, the swing is broadcast, and every reply still goes out,
//! because an unanswered attack freezes the client (`CLAUDE.md`, *always answer*).
//!
//! # Nothing authenticates
//!
//! The guard bounds what a socket may claim; it does not identify who is behind it.

use crate::damage::{self, Attacker, WeaponClass};
use crate::magic::{self, CritStats, MagicAttacker};

/// The owner's margin: a hit more than **this many percent** over the expected maximum is capped.
/// 15 until the owner changed it the same day.
pub const TOLERANCE_PERCENT: u64 = 25;

/// The multiple of the formula's top a hit flagged critical may reach. Generous on purpose:
/// critical damage is a per-character field (`damage::CRIT_RATE`'s limitation note) and the
/// skills that raise it are not read yet, so this is the largest bonus a classic job reaches
/// (Critical Throw / Critical Shot +100% on a +20% base, rounded up).
pub const CRIT_HEADROOM: f64 = 2.5;

/// **Every thief and archer skill is priced** - the owner, 2026-10-02: *"Three Snails should be easy
/// since the damage is fixed. Why can we not create formulas for thief and archer skills?"* The
/// first version skipped six; each turned out to be in data already on the server:
///
/// | skill | how it is priced | source |
/// |---|---|---|
/// | Three Snails | its `fixdamage`, exactly (15 / 25 / 35) | **[L]** `skills.txt` |
/// | Shadow Meso | `moneyCon` x `x` - "throws 200 mesos to deal 8x damage" | **[L]** its own tooltip and columns |
/// | Lucky Seven | the claw formula with the client's own 3.0 multiplier on LUK | **[L]** `damage::weapon_multiplier_for`, read at `14025e447` |
/// | Avenger | the ordinary claw formula at its `damage` % | **[L]** - only Lucky Seven has its own multiplier |
/// | Power Knockback | the bow's own formula at 105%+ - the bow "swung as a club" deals less for any archer | **[I]** that DEX-primary bow beats the club |
/// | Arrow Bomb | its `damage` column is 0; the tooltip's "damage 80%".."140%" | **[L]** the client's tooltip text |
pub const SHADOW_MESO: u32 = 4_111_003;

/// What the guard does with a hit over the limit. `--damage-guard enforce|log|off`, default
/// **enforce** (the owner's choice). `log` keeps the measurement and applies the hit anyway -
/// the switch to reach for if honest hits ever start vanishing, without a rebuild.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Not even computed.
    Off,
    /// Computed and logged; the hit is applied.
    Log,
    /// Computed, logged, and the hit is dropped.
    #[default]
    Enforce,
}

impl Mode {
    pub fn parse(s: &str) -> Option<Mode> {
        match s.to_ascii_lowercase().as_str() {
            "off" => Some(Mode::Off),
            "log" => Some(Mode::Log),
            "enforce" | "on" => Some(Mode::Enforce),
            _ => None,
        }
    }
}

/// The most a hit may be: `ceiling` plus [`TOLERANCE_PERCENT`], rounded up. A claim above it is
/// capped to it.
pub fn limit(ceiling: u64) -> u64 {
    ceiling.saturating_add((ceiling.saturating_mul(TOLERANCE_PERCENT) + 99) / 100)
}

/// Is a claimed hit over the limit, and so capped?
pub fn refused(claimed: u64, ceiling: u64) -> bool {
    claimed > limit(ceiling)
}

/// The expected maximum of one **physical** hit (melee or shoot), headroom included.
/// `None` when no action gives this weapon a multiplier - an unchecked hit, not a refusal.
pub fn physical_ceiling(attacker: &Attacker, class: WeaponClass, skill_id: u32, critical: bool) -> Option<u64> {
    let mut best: Option<f64> = None;
    for action in [
        damage::AttackAction::One,
        damage::AttackAction::Swing,
        damage::AttackAction::Stab,
        damage::AttackAction::Shoot,
        damage::AttackAction::Mixed,
    ] {
        if let Some((_, hi)) = damage::physical_window(attacker, class, action, skill_id) {
            best = Some(best.map_or(hi, |b: f64| b.max(hi)));
        }
    }
    let headroom = if critical { CRIT_HEADROOM } else { damage::CRIT_MULTIPLIER };
    best.map(|hi| damage::finish(hi * headroom))
}

/// The expected maximum of one **magic** hit, headroom included, element at its strongest.
pub fn magic_hit_ceiling(attacker: &MagicAttacker, critical: bool) -> u64 {
    // `crit_ceiling_multiplier` is `1 + damage_percent / 100` once the rate is non-zero, so
    // these two give the same 1.2 / 2.5 headroom as the physical side.
    let damage_percent = if critical { ((CRIT_HEADROOM - 1.0) * 100.0) as u32 } else { 20 };
    magic::magic_ceiling(attacker, CritStats { rate_percent: 100, damage_percent })
}

/// One hit's verdict, for the log and the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Allowed { ceiling: u64 },
    /// Over the limit: the hit counts as `limit`, not as what was claimed.
    Capped { ceiling: u64, limit: u64 },
}

/// Judge one claimed hit against its ceiling.
pub fn judge(claimed: u64, ceiling: u64) -> Verdict {
    if refused(claimed, ceiling) {
        Verdict::Capped { ceiling, limit: limit(ceiling) }
    } else {
        Verdict::Allowed { ceiling }
    }
}

/// The file the suspects go in: `damage-suspects.log`, beside the channel's own log, shared by
/// every channel. One line per refused hit, so a name that appears a hundred times is easy to
/// see. Read it with `Select-String` or any text editor; it is never rotated, because it only
/// grows when somebody is claiming damage their character cannot deal.
pub const SUSPECTS_LOG: &str = "damage-suspects.log";

/// Append one line to [`SUSPECTS_LOG`] - `crate::server::record_beside_log`, which every
/// suspects file shares.
pub fn record_suspect(line: &str) {
    crate::server::record_beside_log(SUSPECTS_LOG, line);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **More than 25% over is capped, to the limit; up to 25% over is not touched.** At a
    /// ceiling of 100 the limit is 125: 125 passes as it is, 126 and 500 both count as 125.
    #[test]
    fn a_hit_more_than_twenty_five_percent_over_is_capped_to_the_limit() {
        assert_eq!(limit(100), 125);
        assert!(!refused(100, 100));
        assert!(!refused(125, 100));
        assert!(refused(126, 100));
        assert_eq!(judge(500, 100), Verdict::Capped { ceiling: 100, limit: 125 });
        assert_eq!(judge(110, 100), Verdict::Allowed { ceiling: 100 });
        assert_eq!(limit(7), 9, "rounded UP, so a small ceiling is never stricter than 25%");
        assert!(!refused(9, 7));
    }

    /// **A critical hit gets crit headroom; an ordinary one does not**, on the same attacker.
    /// The ratio is exactly CRIT_HEADROOM / CRIT_MULTIPLIER before truncation.
    #[test]
    fn a_critical_hit_is_allowed_more_than_an_ordinary_one() {
        let a = Attacker {
            total_watk: 40,
            stats: damage::Stats { strength: 60, dexterity: 20, intelligence: 4, luck: 4 },
            mastery: 0,
            attack_power: 0,
            skill_damage_percent: 100,
        };
        let plain = physical_ceiling(&a, WeaponClass::from_item_id(1_302_000).unwrap(), 0, false).unwrap();
        let crit = physical_ceiling(&a, WeaponClass::from_item_id(1_302_000).unwrap(), 0, true).unwrap();
        assert!(plain > 0);
        let ratio = crit as f64 / plain as f64;
        assert!((ratio - CRIT_HEADROOM / damage::CRIT_MULTIPLIER).abs() < 0.05, "{plain} vs {crit}");
    }

    /// Arrow Bomb's percent is read out of its tooltip, and a stun's "30%" is not mistaken for it.
    #[test]
    fn the_tooltip_damage_percent_is_the_one_after_the_word_damage() {
        use crate::firstjob::tooltip_damage_percent as t;
        assert_eq!(t("MP -14; Stun chance 30% for 2 sec; damage 80%"), Some(80));
        assert_eq!(t("MP -28; Stun chance 60% for 3 sec; damage 140%"), Some(140));
        assert_eq!(t("MP -8; Damage 60%"), Some(60));
        assert_eq!(t("Snail Shell -1; MP -3; Damage 15"), None, "no percent sign: a fixed number, not a percent");
        assert_eq!(t("Throws 200 Mesos to deal 8x damage"), None);
    }

    #[test]
    fn the_mode_parses_and_defaults_to_enforce() {
        assert_eq!(Mode::default(), Mode::Enforce, "the owner's choice, 2026-10-02");
        assert_eq!(Mode::parse("LOG"), Some(Mode::Log));
        assert_eq!(Mode::parse("off"), Some(Mode::Off));
        assert_eq!(Mode::parse("enforce"), Some(Mode::Enforce));
        assert_eq!(Mode::parse("maybe"), None);
    }

    /// **The suspects file appends one dated line per suspect, and keeps what was there.**
    #[test]
    fn the_suspects_file_appends_a_dated_line_each_time() {
        let dir = std::env::temp_dir().join(format!("maplecw-suspects-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let _ = std::fs::remove_file(dir.join(SUSPECTS_LOG));
        crate::server::record_in(&dir, SUSPECTS_LOG, "SUSPECT Wisp (character 200, account 1): first");
        crate::server::record_in(&dir, SUSPECTS_LOG, "SUSPECT Wisp (character 200, account 1): second");
        let text = std::fs::read_to_string(dir.join(SUSPECTS_LOG)).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2, "{text}");
        assert!(lines[0].ends_with("first") && lines[1].ends_with("second"));
        assert!(lines[0].contains(" UTC SUSPECT "), "dated: {}", lines[0]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A magic ceiling grows with the crit flag the same way.
    #[test]
    fn a_critical_magic_hit_is_allowed_more_too() {
        let m = MagicAttacker { magic_total: 60, intelligence: 60, mastery: 0, skill_magic_percent: 30 };
        assert!(magic_hit_ceiling(&m, true) > magic_hit_ceiling(&m, false));
    }
}
