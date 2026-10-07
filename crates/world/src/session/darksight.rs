//! **Dark Sight, as everybody else sees it** - and an attack ends it.
//!
//! The owner, 2026-10-05: *"Currently other players are not able to see when the thief players
//! enter dark sight on other players screens, also thief players should automatically exit dark
//! sight when they are attacking in dark sight."*
//!
//! * **Seen by the map.** Whenever this character starts or stops holding bit 99
//!   (`net::jobbuffs::CTS_DARK_SIGHT`) - cast, recast, right-click, expiry, an attack, anything
//!   that edits the buff list - the rest of the map is sent `0x02B0` / `0x02B1`
//!   (`net::remotebuff`), and the cached `0x0224` is rebuilt with bit 99 so somebody who walks
//!   in later sees it too. One sync point ([`Session::sync_remote_dark_sight`], after every
//!   packet and at every tick) rather than a call at each place the list changes: a place
//!   nobody remembered is exactly how a thief stays half-hidden on one screen.
//! * **An attack ends it**, the whole skill - the invisibility and its Speed penalty together,
//!   as a right-click does (`buff::on_skill_cancel`'s note on why both halves go).

use super::{Reply, Session};

impl Session {
    /// Tell the map when this character's Dark Sight has come on or gone off since last time.
    /// Nothing - not even a store read - when it has not changed.
    pub(super) fn sync_remote_dark_sight(&mut self) {
        let on = self.holds(net::jobbuffs::CTS_DARK_SIGHT);
        if on == self.dark_sight_shown {
            return;
        }
        let Some(chr) = self.claimed_character() else { return };
        self.dark_sight_shown = on;
        let bits = [net::jobbuffs::CTS_DARK_SIGHT];
        let reply = if on {
            Reply {
                opcode: net::remotebuff::REMOTE_TEMPORARY_STAT_SET,
                body: net::remotebuff::remote_stat_set(chr.id, &bits),
                what: format!("RemoteTemporaryStatSet: {} ({}) went into Dark Sight - bit 99 on every other screen", chr.name, chr.id),
            }
        } else {
            Reply {
                opcode: net::remotebuff::REMOTE_TEMPORARY_STAT_RESET,
                body: net::remotebuff::remote_stat_reset(chr.id, &bits),
                what: format!("RemoteTemporaryStatReset: {} ({}) came out of Dark Sight", chr.name, chr.id),
            }
        };
        crate::server::log(&format!(
            "   dark sight: {} ({}) {} - told the map",
            chr.name,
            chr.id,
            if on { "hidden" } else { "visible again" }
        ));
        let map = self.field_of(&chr);
        self.bus().publish(self.subscriber, map, reply, None);
        // And whoever arrives later: their 0x0224 carries the mask.
        let spawn = self.presence(&chr).spawn;
        self.fields.bus().refresh_spawn(self.subscriber, spawn);
    }

    /// The bits this character's `0x0224` must carry in its remote mask.
    pub(super) fn remote_spawn_bits(&self) -> Vec<u32> {
        if self.holds(net::jobbuffs::CTS_DARK_SIGHT) {
            vec![net::jobbuffs::CTS_DARK_SIGHT]
        } else {
            Vec::new()
        }
    }

    /// **An attack in Dark Sight ends it**: every stat the skill granted, with the caster's own
    /// `0x007E`. The map hears it through [`Session::sync_remote_dark_sight`] after the packet.
    /// Nothing when Dark Sight is not held.
    pub(super) fn leave_dark_sight_on_attack(&mut self) -> Vec<Reply> {
        let Some(skill) = self.buffs.iter().find(|b| b.bit == net::jobbuffs::CTS_DARK_SIGHT).map(|b| b.skill_id) else {
            return Vec::new();
        };
        let bits: Vec<u32> = self.buffs.iter().filter(|b| b.skill_id == skill).map(|b| b.bit).collect();
        self.buffs.retain(|b| b.skill_id != skill);
        self.reset_reply(&bits, net::buff::TAIL_LEN, format!("ended by an attack (Dark Sight, skill {skill})"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    const MAP: u32 = 100_000_000;

    /// Two thieves' worth of map: the hider, a watcher on the same map, and one on another map
    /// who walks in later.
    fn three() -> (Vec<(Session, u32)>, Arc<Store>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = Config::default();
        cfg.fields.insert(MAP);
        cfg.fields.insert(MAP + 1);
        let (cfg, fields) = (Arc::new(cfg), Arc::new(Fields::new()));
        let mut out = Vec::new();
        for (i, map) in [MAP, MAP, MAP + 1].into_iter().enumerate() {
            let chr = net::opcode::Character { name: format!("Wisp{i}"), map_id: map, level: 20, job: 400, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, map).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), cfg.clone(), fields.clone());
            s.claim_for_character(id);
            let _ = s.on_field_entered();
            out.push((s, id));
        }
        (out, store)
    }

    fn seen(s: &mut Session, opcode: u16) -> Vec<Vec<u8>> {
        s.tick(1_000).into_iter().filter(|r| r.opcode == opcode).map(|r| r.body).collect()
    }

    /// **The map sees Dark Sight come and go, and an attack ends it.** The owner, 2026-10-05.
    /// Every effect: the watcher gets `0x02B0` with bit 99 and nothing for a second tick; the
    /// player who walks in later is handed a `0x0224` whose remote mask carries bit 99; an attack
    /// sends the hider's own `0x007E` for BOTH of Dark Sight's stats (99 and the Speed penalty,
    /// 92) and the watcher `0x02B1`; the other map hears none of it.
    #[test]
    fn dark_sight_is_seen_by_the_map_and_an_attack_ends_it() {
        let (mut ss, _store) = three();
        let hider = ss[0].1;
        let bits = [net::jobbuffs::CTS_DARK_SIGHT];
        let level = net::jobbuffs::buff_level(net::jobbuffs::DARK_SIGHT, 1, 0).expect("Dark Sight 1");
        let now = ss[0].0.clock_ms;
        let _ = ss[0].0.grant_buff(net::jobbuffs::DARK_SIGHT, level, now);
        let _ = ss[0].0.tick(1_000); // the sync point
        assert_eq!(seen(&mut ss[1].0, net::remotebuff::REMOTE_TEMPORARY_STAT_SET), vec![net::remotebuff::remote_stat_set(hider, &bits)]);
        assert!(seen(&mut ss[1].0, net::remotebuff::REMOTE_TEMPORARY_STAT_SET).is_empty(), "said once");
        assert!(seen(&mut ss[2].0, net::remotebuff::REMOTE_TEMPORARY_STAT_SET).is_empty(), "another map");

        // A later arrival: the hider's 0x0224 carries bit 99 in the remote mask at 55 + name.
        let mut late = ss[2].0.claimed_character().unwrap();
        let _ = ss[2].0.go_to_map(&mut late, MAP, 0, "walks in while the thief is hidden".to_string());
        let arrived = ss[2].0.on_field_entered();
        let spawn = arrived
            .iter()
            .find(|r| r.opcode == net::userpool::USER_ENTER_FIELD && r.body[4..8] == hider.to_le_bytes())
            .expect("the hider's 0x0224");
        let at = 55 + "Wisp0".len();
        assert_eq!(&spawn.body[at..at + net::buff::MASK_LEN], &net::buff::stat_mask(&bits)[..], "bit 99 in the spawn");

        // An attack: out of Dark Sight, both stats, and the map is told.
        let mut attack = net::combat::USER_MELEE_ATTACK.to_le_bytes().to_vec();
        attack.extend_from_slice(&[0u8; 40]);
        let out = ss[0].0.handle(&attack);
        let reset = out.iter().find(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET).expect("the hider's own 0x007E");
        assert_eq!(&reset.body[3..3 + net::buff::MASK_LEN], &net::buff::stat_mask(&[net::jobbuffs::CTS_DARK_SIGHT, net::buff::CTS_SPEED])[..], "both halves");
        assert!(!ss[0].0.holds(net::jobbuffs::CTS_DARK_SIGHT) && !ss[0].0.holds(net::buff::CTS_SPEED));
        assert_eq!(seen(&mut ss[1].0, net::remotebuff::REMOTE_TEMPORARY_STAT_RESET), vec![net::remotebuff::remote_stat_reset(hider, &bits)]);
        // A second attack has nothing to end.
        let out = ss[0].0.handle(&attack);
        assert!(!out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET));
        assert!(seen(&mut ss[1].0, net::remotebuff::REMOTE_TEMPORARY_STAT_RESET).is_empty());
    }
}
