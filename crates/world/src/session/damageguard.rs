//! **The damage guard, on a live swing** - the rules are `crate::damageguard`; this is where the
//! attacker's real numbers are gathered and each claimed hit is let through or dropped.
//!
//! Priced once per swing, BEFORE the swing's ammo is spent: the stack that was thrown still
//! holds its stars at that point, so its attack is counted even when this throw empties it.

use crate::damage::{Attacker, Stats, WeaponClass};
use crate::damageguard::{self, Mode, Verdict};
use crate::magic::MagicAttacker;

use super::Session;

/// What one swing is priced against.
#[derive(Debug, Clone)]
pub(crate) enum Pricing {
    /// The guard is off, or there is no claimed character.
    Off,
    /// The model cannot price this swing; every hit is applied. The reason goes in the log.
    Unchecked(String),
    Physical { attacker: Attacker, class: WeaponClass, skill_id: u32, what: String },
    /// A skill whose hit is a set number, not a formula: Three Snails' `fixdamage`, Shadow
    /// Meso's mesos x `x`. No weapon or stat enters it.
    Fixed { ceiling: u64, what: String },
    Magic { attacker: MagicAttacker, what: String },
}

impl Session {
    /// Price a swing. **Never refuses anything itself** - see [`Session::guarded_damage`].
    pub(super) fn price_swing(&self, opcode: u16, payload: &[u8]) -> Pricing {
        if self.config.damage_guard == Mode::Off {
            return Pricing::Off;
        }
        let Some(chr) = self.claimed_character() else { return Pricing::Off };
        let magic = opcode == net::combat::USER_MAGIC_ATTACK;
        if !magic && opcode != net::combat::USER_MELEE_ATTACK && opcode != net::combat::USER_SHOOT_ATTACK {
            return Pricing::Unchecked(format!("opcode {opcode:#06x} is not melee, shoot or magic"));
        }
        let skill = net::attack::parse(opcode, payload).ok().and_then(|p| p.skill());
        let (skill_id, level) = match skill {
            Some((id, claimed)) => {
                let level = self.store.skill_level(chr.id, id).ok().filter(|l| *l > 0).unwrap_or(u32::from(claimed));
                (id, level)
            }
            None => (0, 0),
        };
        let row = (skill_id != 0).then(|| self.config.firstjob.level(skill_id, level).copied()).flatten();
        if skill_id != 0 && row.is_none() {
            return Pricing::Unchecked(format!("skill {skill_id} level {level} is not in the skill table"));
        }
        // **A set number, not a formula** - priced before the weapon, which does not enter it.
        if let Some(r) = row {
            if let Some(fixed) = r.fix_damage.filter(|n| *n > 0) {
                return Pricing::Fixed {
                    ceiling: u64::from(fixed),
                    what: format!("skill {skill_id} lv{level}: fixdamage {fixed}"),
                };
            }
            if skill_id == damageguard::SHADOW_MESO {
                if let (Some(mesos), Some(times)) = (r.money_con, r.x.and_then(|x| u64::try_from(x).ok())) {
                    return Pricing::Fixed {
                        ceiling: u64::from(mesos) * times,
                        what: format!("Shadow Meso lv{level}: {mesos} mesos x {times}"),
                    };
                }
            }
        }

        // Every worn item, scrolled or template, as the login record sends them.
        let worn = self.dressed(&chr);
        let gear = |f: fn(&net::opcode::EquipStatSet) -> u16| -> u32 {
            worn.iter().map(|(_, _, s)| u32::from(f(&s.stats))).sum()
        };
        let stats = Stats {
            strength: u32::from(chr.strength) + gear(|s| s.inc_str),
            dexterity: u32::from(chr.dexterity) + gear(|s| s.inc_dex),
            intelligence: u32::from(chr.intelligence) + gear(|s| s.inc_int),
            luck: u32::from(chr.luck) + gear(|s| s.inc_luk),
        };

        // **The weapon must have known numbers, or nothing is priced.** Every real weapon has
        // attack or magic attack; a weapon reading 0/0 means its template was not found -
        // `gm-handbook/equips.txt` missing or stale on the server box - and pricing against 0
        // would cap every honest hit to a point or two. Measured: a test fixture with no
        // templates priced a sword's swing at a ceiling of 1.
        let Some((weapon, class, weapon_stats)) = worn
            .iter()
            .find_map(|(_, id, st)| WeaponClass::from_item_id(*id).map(|c| (*id, c, st.stats)))
        else {
            return Pricing::Unchecked("no weapon in hand".to_string());
        };
        if weapon_stats.inc_wat == 0 && weapon_stats.inc_mad == 0 {
            return Pricing::Unchecked(format!(
                "weapon {weapon} reads 0 attack and 0 magic attack - its template is missing (gm-handbook/equips.txt?)"
            ));
        }

        if magic {
            let Some(mad) = row.and_then(|r| r.mad_percent).filter(|m| *m > 0) else {
                return Pricing::Unchecked(format!("skill {skill_id} has no Basic Attack (mad) column"));
            };
            let buff = u32::try_from(self.held_value(net::jobbuffs::CTS_MAGIC_ATTACK).max(0)).unwrap_or(0);
            let magic_total = crate::magic::magic_total_seed(stats.intelligence) + gear(|s| s.inc_mad) + buff;
            let attacker = MagicAttacker { magic_total, intelligence: stats.intelligence, mastery: 0, skill_magic_percent: mad };
            return Pricing::Magic {
                attacker,
                what: format!("magic skill {skill_id} lv{level}: magic {magic_total}, INT {}, {mad}%", stats.intelligence),
            };
        }

        let percent = match row {
            None => 100,
            // Arrow Bomb's `damage` column is 0 at every level; its percent is in its tooltip.
            Some(r) => match r.damage_percent.filter(|p| *p > 0).or(r.tooltip_damage_percent.filter(|p| *p > 0)) {
                Some(p) => p,
                None => return Pricing::Unchecked(format!("skill {skill_id} has no damage column")),
            },
        };
        // The best ammunition the attacker holds for this weapon - the server does not know
        // which stack was thrown, so the strongest one is the honest ceiling.
        let ammo = match class {
            WeaponClass::Bow => Some(2_060_000..=2_060_999),
            WeaponClass::Crossbow => Some(2_061_000..=2_061_999),
            WeaponClass::Claw => Some(2_070_000..=2_079_999),
            _ => None,
        };
        let ammo_attack = ammo
            .and_then(|range| {
                self.store
                    .bag_items(chr.id, store::InventoryType::Use)
                    .ok()?
                    .iter()
                    .filter(|r| range.contains(&r.item.item_id) && r.item.kind.quantity() > 0)
                    .filter_map(|r| self.config.shops.item_data.get(&r.item.item_id).map(|d| d.ammo_attack))
                    .max()
            })
            .unwrap_or(0);
        let buff = u32::try_from(self.held_value(net::jobbuffs::CTS_WEAPON_ATTACK).max(0)).unwrap_or(0);
        let total_watk = gear(|s| s.inc_wat) + ammo_attack + buff;
        let attacker = Attacker { total_watk, stats, mastery: 0, attack_power: buff, skill_damage_percent: percent };
        Pricing::Physical {
            attacker,
            class,
            skill_id,
            what: format!(
                "weapon {weapon}, skill {skill_id} lv{level} {percent}%, attack {total_watk} (ammo {ammo_attack}, buff {buff}), \
                 STR {} DEX {} INT {} LUK {}",
                stats.strength, stats.dexterity, stats.intelligence, stats.luck
            ),
        }
    }

    /// The damage one target takes from this swing. A hit more than 25% over its ceiling is
    /// **capped to the limit** (enforce) or kept whole (log) - either way the attacker is
    /// written to `damage-suspects.log` and the channel log.
    pub(super) fn guarded_damage(&self, pricing: &Pricing, target: &net::combat::AttackTarget) -> u64 {
        let (enforce, ceiling_of, what): (bool, Box<dyn Fn(bool) -> Option<u64>>, &str) = match pricing {
            Pricing::Off => return target.total_damage(),
            Pricing::Unchecked(why) => {
                if target.total_damage() > 0 {
                    crate::server::log(&format!("   damage guard: unchecked hit on {} - {why}", target.object_id));
                }
                return target.total_damage();
            }
            Pricing::Physical { attacker, class, skill_id, what } => (
                self.config.damage_guard == Mode::Enforce,
                Box::new(move |crit| damageguard::physical_ceiling(attacker, *class, *skill_id, crit)),
                what.as_str(),
            ),
            Pricing::Fixed { ceiling, what } => (
                self.config.damage_guard == Mode::Enforce,
                Box::new(move |crit| {
                    Some(if crit { (*ceiling as f64 * damageguard::CRIT_HEADROOM) as u64 } else { *ceiling })
                }),
                what.as_str(),
            ),
            Pricing::Magic { attacker, what } => (
                self.config.damage_guard == Mode::Enforce,
                Box::new(move |crit| Some(damageguard::magic_hit_ceiling(attacker, crit))),
                what.as_str(),
            ),
        };
        let mut total = 0u64;
        for hit in &target.hits {
            let Some(ceiling) = ceiling_of(hit.flag_b) else {
                total = total.saturating_add(hit.damage);
                continue;
            };
            match damageguard::judge(hit.damage, ceiling) {
                Verdict::Allowed { .. } => total = total.saturating_add(hit.damage),
                Verdict::Capped { ceiling, limit } => {
                    let (name, id, account) = self
                        .claimed_character()
                        .map(|c| (c.name.clone(), c.id, self.claimed.as_ref().map_or(0, |m| m.account_id)))
                        .unwrap_or_default();
                    let action = if enforce {
                        format!("CAPPED to {limit}")
                    } else {
                        "logged, applied in full (--damage-guard log)".to_string()
                    };
                    let line = format!(
                        "SUSPECT {name} (character {id}, account {account}): claimed {} on mob {}{} - expected at most \
                         {ceiling}, limit {limit} (+{}%). {action}. {what}",
                        hit.damage,
                        target.object_id,
                        if hit.flag_b { " (critical)" } else { "" },
                        damageguard::TOLERANCE_PERCENT
                    );
                    crate::server::log(&format!("   damage guard: {line}"));
                    damageguard::record_suspect(&line);
                    total = total.saturating_add(if enforce { limit } else { hit.damage });
                }
            }
        }
        total
    }
}
