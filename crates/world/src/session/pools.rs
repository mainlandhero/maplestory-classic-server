//! The maximum HP and MP a character can actually hold - **as the client draws them**.
//!
//! The owner, 2026-09-06, with a screenshot of `HP 358 / 447`: *"Max HP Increase results in Cobalt
//! having more HP on client side, but natural regeneration does not regenerate that amount,
//! which seems to be meaning that the server thinks that Cobalt is at max HP already."*
//!
//! Every clause was right. The database held `358 / 358`, `world.log` said *"HP is full"*, and
//! the screen said 447 because **the client applies Max HP Increase's `mhpR` itself, on top
//! of the maximum this server sends** - `447 = 358 + ⌊358 × 25 / 100⌋`. That was
//! `research/magician-first-job.md` §8 experiment A, answered in the direction that means
//! the server must not fold the percent into its packet (the client would apply it again)
//! and must instead raise every ceiling it enforces to the same number.
//!
//! This module is that number, computed **on demand from the skill level** rather than
//! stored. The owner's second sentence - *"when the server receives an increase in skill, if that
//! skill is a passive such as increasing HP for the user, it should be accounted for on the
//! server side"* - falls out of that: the point lands in `character_skills`, and the next
//! regen tick, potion, level-up or party-bar packet reads the new level and the new ceiling
//! with it. Nothing has to be notified.
//!
//! # And the worn items, which are the same mistake at a flat rate
//!
//! The owner, 2026-09-13, with a screenshot of `HP 194 / 199`: *"There are rare instances of when
//! the server and the player does not agree what is the max HP for the user. Currently the
//! character the owner has 199 max, but passive recovery only recovers up to 194 and stops."*
//!
//! The database held `194 / 194`. The five is the Red Headband on their head: `1002003`'s
//! template carries `incMHP 5`, the record sends that stat on the worn item, and **the client
//! adds every worn item's `incMHP` / `incMMP` to the maximum it draws**, exactly as it adds
//! the percent. The ceiling here counted the percent and not the flat, so a character whose
//! only bonus is a hat was "full" five short of the bar. "Rare" because most beginner gear
//! has no HP on it; the headband is the one Maria hands out.
//!
//! The flat bonus goes on **before** the percent - `(base + worn) x (1 + mhpR/100)` - which
//! is the reference server's order (`Char.getTotalStatAsDouble`: base, then the accumulated
//! equip stats, then the rate applied to the sum) **[R]**. This client's own order is not
//! measured: the one character with a percent (Cobalt, 358 -> 447) wears nothing with HP on
//! it, so their number is the same either way. If a warrior in HP gear ever reads one off, the
//! other order is a one-line change and this paragraph is where to note it.
//!
//! The worn stats are the ones the record SENDS - `Session::dressed`, the template per worn
//! id - not the database rows, so the ceiling cannot disagree with the bar by construction.
//!
//! Callers: `regen_tick`, `on_use_item`, Recovery's tick, the level-up refill, `!heal`,
//! `!resetap`'s clamp, a quest's set-HP, and `party_hp_tick`. Anything that compares HP or MP
//! against a maximum goes through here, or it is comparing against the wrong one.

use super::*;

/// The ceilings the client draws, for one character right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Pools {
    pub(super) max_hp: u32,
    pub(super) max_mp: u32,
}

impl Session {
    /// `chr.max_hp` / `chr.max_mp` plus the worn items' `incMHP` / `incMMP`, raised by a
    /// learned Max HP / Max MP Increase.
    ///
    /// The percent comes from the generated skill table's `mhpR` / `mmpR` at the level the
    /// character actually has; an unlearned skill, a missing table or a missing column all
    /// read as `0` and return the base unchanged - the answer for every character until one
    /// puts a point in the passive. `itemrecovery::boosted_max` is the arithmetic and the
    /// measurement behind it. The worn flat bonus is summed off the same stats the record
    /// sends, so a missing `equips.txt` leaves it at zero exactly as it leaves the tooltip.
    pub(super) fn pools(&self, chr: &net::opcode::Character) -> Pools {
        let worn = self.dressed(chr);
        let flat = |pick: fn(&net::opcode::EquipStatSet) -> u16| -> u32 {
            worn.iter().map(|(_, _, s)| u32::from(pick(&s.stats))).sum()
        };
        let base_hp = chr.max_hp.saturating_add(flat(|s| s.inc_mhp));
        let base_mp = chr.max_mp.saturating_add(flat(|s| s.inc_mmp));
        let percent = |skill_id: u32, pick: fn(&crate::firstjob::CastNumbers) -> Option<u32>| {
            let level = self.store.skill_level(chr.id, skill_id).unwrap_or(0);
            if level == 0 {
                return 0;
            }
            self.config.firstjob.level(skill_id, level).and_then(pick).unwrap_or(0)
        };
        // **Hyper Body, while it is held.** Same shape as the passive, different lifetime: the
        // percent is CTS 94's value on the buff this session is holding, and it goes back to
        // zero the moment the buff expires or is cancelled. A recipient of the party cast holds
        // its own copy, so their ceiling rises on their own session.
        let hyper_body = u32::try_from(self.held_value(net::jobbuffs::CTS_MAX_HP)).unwrap_or(0);
        Pools {
            max_hp: crate::itemrecovery::boosted_max(
                crate::itemrecovery::boosted_max(
                    base_hp,
                    percent(MAX_HP_INCREASE, |l| l.max_hp_percent),
                ),
                hyper_body,
            ),
            max_mp: crate::itemrecovery::boosted_max(
                base_mp,
                percent(MAX_MP_INCREASE, |l| l.max_mp_percent),
            ),
        }
    }
}

/// `1000001` Max HP Increase - `mhpR` 10..25 over fifteen levels. Warrior book.
pub(super) const MAX_HP_INCREASE: u32 = 1_000_001;
/// `2000001` Max MP Increase - `mmpR` 10..25 over fifteen levels. Magician book.
pub(super) const MAX_MP_INCREASE: u32 = 2_000_001;

#[cfg(test)]
mod tests {
    use super::*;

    fn session_with_pools(max_hp: u32, max_mp: u32) -> (Session, Arc<Store>, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Cobalt".to_string(), ..Default::default() };
        let mut made = store.create_character(account, 0, &chr).unwrap();
        made.max_hp = max_hp;
        made.max_mp = max_mp;
        made.hp = max_hp;
        made.mp = max_mp;
        store.save_character_progress(&made).unwrap();
        store.create_migration(account, made.id, 0, 0).unwrap();
        let skills = std::path::Path::new("../../gm-handbook/skills.txt");
        let config = Config {
            firstjob: crate::firstjob::CombatTable::load(skills),
            ..Config::default()
        };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(made.id);
        (s, store, made.id)
    }

    /// **Cobalt's own numbers**: base 358, Max HP Increase 15, the client drew 447.
    #[test]
    fn josiahs_ceiling_is_447_from_a_base_of_358_at_level_15() {
        if !std::path::Path::new("../../gm-handbook/skills.txt").exists() {
            return; // generated, gitignored - python tools/dump_skills.py
        }
        let (s, store, id) = session_with_pools(358, 259);
        let chr = s.claimed_character().unwrap();
        assert_eq!(s.pools(&chr), Pools { max_hp: 358, max_mp: 259 }, "unlearned: the base");

        store.set_skill_level(id, MAX_HP_INCREASE, 15).unwrap();
        assert_eq!(s.pools(&chr), Pools { max_hp: 447, max_mp: 259 }, "the screenshot's number");

        // Level 1 is 10%; and the MP twin follows the same expression.
        store.set_skill_level(id, MAX_HP_INCREASE, 1).unwrap();
        store.set_skill_level(id, MAX_MP_INCREASE, 15).unwrap();
        assert_eq!(s.pools(&chr), Pools { max_hp: 393, max_mp: 323 }, "358+35, 259+64");
    }

    /// **The owner's own numbers**: base 194, a Red Headband (`incMHP 5`), the client drew 199.
    ///
    /// The template is supplied inline rather than read from `equips.txt`, so this runs
    /// without the generated handbook and pins the arithmetic, not the data file. The MP twin
    /// is asserted on the same item to show the two flats are summed independently.
    #[test]
    fn wisps_ceiling_is_199_from_a_base_of_194_and_a_red_headband() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Wisp".to_string(),
            equips: vec![(1, 1_002_003), (11, 1_302_000)],
            ..Default::default()
        };
        let mut made = store.create_character(account, 0, &chr).unwrap();
        made.max_hp = 194;
        made.max_mp = 113;
        made.hp = 194;
        made.mp = 113;
        store.save_character_progress(&made).unwrap();
        store.create_migration(account, made.id, 0, 0).unwrap();
        let mut config = Config::default();
        config.equips.insert(
            1_002_003,
            crate::config::EquipTemplate { inc_mhp: 5, inc_mmp: 3, ..Default::default() },
        );
        // The sword has a template with no HP on it; it must add nothing.
        config.equips.insert(1_302_000, crate::config::EquipTemplate { inc_wat: 17, ..Default::default() });
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(made.id);
        let chr = s.claimed_character().unwrap();
        assert_eq!(s.pools(&chr), Pools { max_hp: 199, max_mp: 116 }, "the screenshot's 199");
        assert_eq!(chr.max_hp, 194, "the record's base is untouched - the client adds the hat");

        // No template for the hat (a missing equips.txt): the record sends zeros, the client
        // draws 194, and so must this.
        let mut cfg = (*s.config).clone();
        cfg.equips.clear();
        s.config = Arc::new(cfg);
        assert_eq!(s.pools(&chr), Pools { max_hp: 194, max_mp: 113 });
    }

    /// Without the generated table the ceiling is the base - a wrong number is worse than the
    /// old behaviour, and the old behaviour is what an absent column must degrade to.
    #[test]
    fn no_table_means_the_base_and_nothing_else() {
        let (mut s, store, id) = session_with_pools(100, 100);
        let mut cfg = (*s.config).clone();
        cfg.firstjob = crate::firstjob::CombatTable::load(std::path::Path::new("no/such/skills.txt"));
        s.config = Arc::new(cfg);
        store.set_skill_level(id, MAX_HP_INCREASE, 15).unwrap();
        let chr = s.claimed_character().unwrap();
        assert_eq!(s.pools(&chr), Pools { max_hp: 100, max_mp: 100 });
    }
}
