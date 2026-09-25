//! **Mob MP, MP-costing attacks and mob skills** - what each mob's own data says it may do,
//! and the numbers the server must hand the client for it to do them.
//!
//! The owner, 2026-09-24: *"King Slime in the PQ has different attacks other than chasing the
//! players. There should be a jump attack, and there should be a summon slime attack."* Then
//! *"The server should keep track of the King Slime's MP and send it to the mob controller"*,
//! *"We can do 10 MP every 5 seconds"*, and *"Please solve this for all mobs with skills, not
//! just King Slime"*.
//!
//! # Why no mob ever used an MP-costing attack or a skill
//!
//! `0x03E4 MobCtrlAck` carries, at body offset 7, a `u32` the handler (`141c820b7`) stores
//! obfuscated into `mob+0x3b0`. The attack chooser `FUN_141c7c900` reads that same field for
//! each candidate attack and **skips the attack when the attack's `+0x24` exceeds it**
//! (`141c7d38f..141c7d3b2`: `deobf(mob+0x3b0)`, `cmp [attack+0x24], eax / jle`, else skip
//! unless `[attack+0x90]` is set) **[L]**. This server sent `0` there on every
//! acknowledgement, so every attack with a cost was ruled out: **49 attacks on 41 mobs**,
//! King Slime's jump among them. That `+0x24` is `conMP` specifically is **[I]** - the loader
//! reads it by a route none of `xref.py`, `dataref.py` or an immediate scan could see; the
//! v214 reference's `ctrlAck` writes the mob's MP in exactly this slot **[C]**.
//!
//! Skills are offered in the same packet: offset 11 is a skill id and 15 its level, looked up
//! in the mob's template (`FUN_14049a080`); `0` short-circuits the skill block (`141c820f0`)
//! **[L]**. When the client performs one it reports it in `0x02FF`'s offset-8 `u64` - skill id
//! in the low 16 bits, level in the next 16 **[C]** - and the EFFECT is the server's to apply.
//!
//! # The data - `gm-handbook/mobskills.txt`, `tools/dump_mobskills.py`
//!
//! 193 mob templates; **61** have a skill or an MP-costing attack; **28** have skills, using 16
//! skill ids in 21 levels. Three families **[L]** (names by which mobs carry them - the client's
//! `String/MobSkill.img` has no names for these ids):
//!
//! * **200 - summon.** King Slime (3 Slimes, at or below 50% HP, every 15 s, at most 15),
//!   Mano (4 Snails + 2 Blue Snails, below 90%, every 10 s, at most 30), Lycanthrope (1
//!   Werewolf, below 75%, every 45 s, at most 5). **Applied here.**
//! * **120..126** - `prop`, `time`: debuffs on players (Fairies, Jr. Boogies, Thanatos,
//!   Gatekeeper, Mano's 126). **Not offered**: the player status packet they need is not built.
//! * **160..167** - `x 25`, `time 30`: buffs on the mob itself (Golems, Yetis, Pixies, Iron Hog,
//!   Werewolf, Fairies). **Not offered**: the mob status packet they need is not built.
//!
//! A skill whose effect the server cannot apply is not offered, because an offer makes the mob
//! play the cast and spend the MP for nothing.
//!
//! # MP
//!
//! Kept per live mob, starting full at `maxMP`. An MP-costing attack is charged when a move
//! report carries its action (`attackN` is wire action `12 + N`), once per that attack's
//! `afterDelay` (floored at 1 s) however many reports carry it; a summon is charged its
//! `mpCon`. Recovery is **every 5 seconds** (the owner) by the mob's own `mpRecovery`, or **10**
//! (the owner's number) where its data has none - 29 of the 61 do not. Lazily applied, so nothing
//! needs a timer.

use std::collections::HashMap;
use std::path::Path;

/// The owner's interval.
pub const MP_REGEN_EVERY_MS: u64 = 5_000;
/// The owner's amount, for a mob whose data has no `mpRecovery`.
pub const MP_REGEN_DEFAULT: u32 = 10;
/// The floor on the window an attack is charged once in, for the two attacks whose
/// `afterDelay` is 0: one animation spans more than one move report.
pub const MIN_CHARGE_WINDOW_MS: u64 = 1_000;
/// The summon skill.
pub const SUMMON: u32 = 200;
/// Wire action of `attack1`; `attackN` is this plus `N - 1` (`crate::mobattack::ACTION_NAMES`).
pub const ATTACK1_ACTION: u8 = 13;

/// One mob template's kit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MobKit {
    pub max_mp: u32,
    /// Per [`MP_REGEN_EVERY_MS`]; already defaulted to [`MP_REGEN_DEFAULT`] when the data has none.
    pub regen: u32,
    /// `(attack index, conMP, afterDelay ms)` for the attacks that cost MP.
    pub attacks: Vec<(u8, u32, u64)>,
    /// `(skill, level)`.
    pub skills: Vec<(u32, u16)>,
}

/// One `MobSkill.img` level.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SkillLevel {
    pub mp_con: u32,
    pub interval_ms: u64,
    /// Offered only at or below this percent of max HP; `0` = no condition.
    pub hp_percent: u64,
    /// No more than this many of what it summons alive; `0` = no limit.
    pub limit: usize,
    /// The templates one cast summons - empty for anything but a summon.
    pub summons: Vec<u32>,
}

impl SkillLevel {
    /// Whether this server can apply the skill's effect. Only summons, today.
    pub fn applicable(&self, skill: u32) -> bool {
        skill == SUMMON && !self.summons.is_empty()
    }

    /// How many of one cast may be summoned without passing the limit.
    pub fn how_many(&self, alive: usize) -> usize {
        if self.limit == 0 {
            return self.summons.len();
        }
        self.summons.len().min(self.limit.saturating_sub(alive))
    }

    /// Whether to OFFER it now.
    pub fn may_offer(&self, mp: u32, hp: u64, max_hp: u64, ready: bool, alive: usize) -> bool {
        let hp_ok = self.hp_percent == 0 || (max_hp > 0 && hp.saturating_mul(100) <= max_hp.saturating_mul(self.hp_percent));
        ready && hp_ok && mp >= self.mp_con && self.how_many(alive) > 0
    }
}

/// The whole table.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MobSkillTable {
    pub mobs: HashMap<u32, MobKit>,
    pub levels: HashMap<(u32, u16), SkillLevel>,
}

impl MobSkillTable {
    /// Load `gm-handbook/mobskills.txt`. A missing file is an empty table: every mob then
    /// behaves as it did before this existed.
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path).map(|t| Self::parse(&t)).unwrap_or_default()
    }

    /// Parse the text of that file. Unreadable rows are skipped.
    pub fn parse(text: &str) -> Self {
        let mut t = Self::default();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split(',').map(str::trim).collect();
            let n = |i: usize| cols.get(i).and_then(|c| c.parse::<u64>().ok());
            match cols[0] {
                "mob" => {
                    let (Some(tid), Some(mp), Some(rec)) = (n(1), n(2), n(3)) else { continue };
                    let kit = t.mobs.entry(tid as u32).or_default();
                    kit.max_mp = mp as u32;
                    kit.regen = if rec == 0 { MP_REGEN_DEFAULT } else { rec as u32 };
                }
                "attack" => {
                    let (Some(tid), Some(i), Some(c), Some(d)) = (n(1), n(2), n(3), n(4)) else { continue };
                    t.mobs.entry(tid as u32).or_default().attacks.push((i as u8, c as u32, d));
                }
                "skill" => {
                    let (Some(tid), Some(s), Some(l)) = (n(1), n(2), n(3)) else { continue };
                    t.mobs.entry(tid as u32).or_default().skills.push((s as u32, l as u16));
                }
                "level" => {
                    let (Some(s), Some(l), Some(mp), Some(iv), Some(hp), Some(lim)) = (n(1), n(2), n(3), n(4), n(5), n(6)) else { continue };
                    let summons = cols.get(7).map(|c| c.split_whitespace().filter_map(|x| x.parse().ok()).collect()).unwrap_or_default();
                    t.levels.insert(
                        (s as u32, l as u16),
                        SkillLevel { mp_con: mp as u32, interval_ms: iv * 1_000, hp_percent: hp, limit: lim as usize, summons },
                    );
                }
                _ => {}
            }
        }
        t
    }

    pub fn kit(&self, template: u32) -> Option<&MobKit> {
        self.mobs.get(&template)
    }

    pub fn level(&self, skill: u32, level: u16) -> Option<&SkillLevel> {
        self.levels.get(&(skill, level))
    }
}

/// **One mob's MP, kept by the server.** Regenerates lazily - whole ticks since
/// `last_regen_ms` are added when it is next touched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MobMp {
    pub mp: u32,
    pub last_regen_ms: u64,
    /// When an attack was last charged, and which, so one animation is charged once.
    pub last_charge: Option<(u8, u64)>,
}

impl MobMp {
    /// A mob just seen: full.
    pub fn full(max: u32, now_ms: u64) -> Self {
        Self { mp: max, last_regen_ms: now_ms, last_charge: None }
    }

    /// Apply the regeneration owed up to `now_ms`: whole ticks only, the clock advancing by
    /// whole ticks; a full pool banks nothing - its clock simply moves to now.
    pub fn regen(&mut self, max: u32, per_tick: u32, now_ms: u64) {
        if self.mp >= max {
            self.mp = max;
            self.last_regen_ms = now_ms;
            return;
        }
        let ticks = now_ms.saturating_sub(self.last_regen_ms) / MP_REGEN_EVERY_MS;
        if ticks == 0 {
            return;
        }
        let gained = u32::try_from(ticks).unwrap_or(u32::MAX).saturating_mul(per_tick);
        self.mp = self.mp.saturating_add(gained).min(max);
        self.last_regen_ms += ticks * MP_REGEN_EVERY_MS;
    }

    /// Spend `cost` now, floored at zero. A full pool's regen clock starts here.
    pub fn spend(&mut self, max: u32, cost: u32, now_ms: u64) {
        if self.mp >= max {
            self.last_regen_ms = now_ms;
        }
        self.mp = self.mp.saturating_sub(cost);
    }

    /// **A move report carrying attack `index`'s action**: charged once per its window.
    /// `true` when this report was the one charged.
    pub fn attacked(&mut self, kit: &MobKit, index: u8, now_ms: u64) -> bool {
        let Some(&(_, cost, delay)) = kit.attacks.iter().find(|(i, _, _)| *i == index) else { return false };
        self.regen(kit.max_mp, kit.regen, now_ms);
        let window = delay.max(MIN_CHARGE_WINDOW_MS);
        if self.last_charge.is_some_and(|(i, at)| i == index && now_ms.saturating_sub(at) < window) {
            return false;
        }
        self.spend(kit.max_mp, cost, now_ms);
        self.last_charge = Some((index, now_ms));
        true
    }
}

/// The attack index a move report's action byte names, if it is an attack
/// (`attack1..attack16`, either facing).
pub fn attack_index(move_action: u8) -> Option<u8> {
    if move_action == 0xFF {
        return None;
    }
    let action = move_action >> 1;
    (ATTACK1_ACTION..ATTACK1_ACTION + 16).contains(&action).then(|| action - ATTACK1_ACTION)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KING: &str = "\
mob, 800003, 100, 10
attack, 800003, 0, 10, 3000
skill, 800003, 200, 1
mob, 1068, 30, 0
attack, 1068, 1, 2, 0
skill, 1068, 160, 1
skill, 1068, 161, 1
level, 200, 1, 0, 15, 50, 15, 7 7 7
level, 160, 1, 5, 40, 0, 0,
";

    /// The file's shape, including the default regen for a mob whose data has none.
    #[test]
    fn the_table_reads_every_row_kind() {
        let t = MobSkillTable::parse(KING);
        let king = t.kit(800_003).unwrap();
        assert_eq!((king.max_mp, king.regen), (100, 10));
        assert_eq!(king.attacks, vec![(0, 10, 3_000)]);
        assert_eq!(king.skills, vec![(200, 1)]);
        assert_eq!(t.kit(1068).unwrap().regen, MP_REGEN_DEFAULT, "no mpRecovery in the data: the owner's 10");
        let s = t.level(200, 1).unwrap();
        assert_eq!((s.mp_con, s.interval_ms, s.hp_percent, s.limit), (0, 15_000, 50, 15));
        assert_eq!(s.summons, vec![7, 7, 7]);
        assert!(s.applicable(200));
        assert!(!t.level(160, 1).unwrap().applicable(160), "a buff is not applied, so not offered");
    }

    /// **The owner's regen and the data's cost.** Full at 100; a jump costs 10 once, however many
    /// reports carry it inside its 3 s; 10 back every 5 s, whole ticks only, never above 100.
    #[test]
    fn the_mp_is_spent_by_attacks_and_comes_back_every_five_seconds() {
        let t = MobSkillTable::parse(KING);
        let king = t.kit(800_003).unwrap();
        let at = 1_000_000u64;
        let mut m = MobMp::full(100, at);
        assert!(m.attacked(king, 0, at));
        assert_eq!(m.mp, 90);
        assert!(!m.attacked(king, 0, at + 500), "the same jump in a second report is not charged twice");
        assert!(!m.attacked(king, 3, at + 500), "an attack the mob does not have costs nothing");
        m.regen(100, 10, at + 4_999);
        assert_eq!(m.mp, 90, "not a whole tick yet");
        m.regen(100, 10, at + 5_000);
        assert_eq!(m.mp, 100, "10 back after 5 s");
        // Ten jumps, 3 s apart: 100 spent, five ticks (5 .. 25 s) = 50 back.
        let mut m = MobMp::full(100, at);
        for k in 0..10 {
            assert!(m.attacked(king, 0, at + k * 3_000));
        }
        assert_eq!(m.mp, 50, "{m:?}");
        let mut low = MobMp { mp: 5, last_regen_ms: at, last_charge: None };
        low.spend(100, 10, at);
        assert_eq!(low.mp, 0, "floored, not wrapped");
        // A zero afterDelay still charges one animation once per second.
        let yeti = t.kit(1068).unwrap();
        let mut y = MobMp::full(30, at);
        assert!(y.attacked(yeti, 1, at));
        assert!(!y.attacked(yeti, 1, at + 999));
        assert!(y.attacked(yeti, 1, at + 1_000));
    }

    /// The summon's own conditions: HP threshold, cooldown, MP, and the limit - each alone
    /// refuses; a cast near the limit is trimmed.
    #[test]
    fn a_summon_follows_its_own_data() {
        let t = MobSkillTable::parse(KING);
        let s = t.level(200, 1).unwrap();
        assert!(s.may_offer(100, 8_410, 16_820, true, 0), "exactly half");
        assert!(!s.may_offer(100, 8_411, 16_820, true, 0), "above half");
        assert!(!s.may_offer(100, 100, 16_820, false, 0), "cooling down");
        assert!(!s.may_offer(100, 100, 16_820, true, 15), "at the limit");
        assert_eq!(s.how_many(13), 2);
        let costly = SkillLevel { mp_con: 20, ..s.clone() };
        assert!(!costly.may_offer(19, 100, 16_820, true, 0), "not enough MP");
        assert_eq!(attack_index(26), Some(0));
        assert_eq!(attack_index(27), Some(0));
        assert_eq!(attack_index(28), Some(1));
        assert_eq!(attack_index(0xFF), None);
        assert_eq!(attack_index(14), None, "hit1 is not an attack");
    }

    /// **The real file, when it has been generated**: the survey's counts and King Slime's
    /// rows exactly as its `Mob.wz` and `MobSkill.img` read.
    #[test]
    fn the_generated_file_has_every_mob_and_king_slime_as_read() {
        let t = MobSkillTable::load(Path::new("../../gm-handbook/mobskills.txt"));
        if t.mobs.is_empty() {
            return; // generated, gitignored - python tools/dump_mobskills.py
        }
        assert_eq!(t.mobs.len(), 61);
        assert_eq!(t.mobs.values().map(|k| k.attacks.len()).sum::<usize>(), 49);
        assert_eq!(t.mobs.values().filter(|k| !k.skills.is_empty()).count(), 28);
        let king = t.kit(800_003).unwrap();
        assert_eq!((king.max_mp, king.regen, king.attacks.clone(), king.skills.clone()), (100, 10, vec![(0, 10, 3_000)], vec![(200, 1)]));
        assert_eq!(t.level(200, 1).unwrap().summons, vec![7, 7, 7]);
        // Every summon level names real mobs; nothing else is applicable.
        let applicable: Vec<(u32, u16)> = t.levels.iter().filter(|((s, _), l)| l.applicable(*s)).map(|(k, _)| *k).collect();
        assert!(applicable.iter().all(|(s, _)| *s == SUMMON) && applicable.len() == 3, "{applicable:?}");
    }
}
