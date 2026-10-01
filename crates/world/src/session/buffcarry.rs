//! **Buffs survive a channel change**, so the arriving channel can take them down.
//!
//! A player, relayed by the owner 2026-10-01: *"Once buff expires, it does not go away."* The
//! icon comes down only on a `0x007E` from the server (`session/buff.rs` `buff_tick` - the
//! client's own expiry drives the flashing and nothing else, measured 2026-08-22). Each channel
//! is its own process, and a connection starts with an empty buff list, so a buff cast on one
//! channel and carried to another had nobody left to send that `0x007E`: the leaving session
//! is gone and the arriving one never heard of it.
//!
//! **[I], and it is the only explanation found:** every server path that grants a buff records
//! it, and the tick expires whatever is recorded (`a_toggle_survives_the_buff_tick_and_a_timed_
//! buff_does_not` and its siblings). The report carried no log and no word on what the player
//! did before, so a channel change is the inference, not a measurement.
//!
//! So the leaving channel writes what is held to `store::carriedbuffs` and the arriving one
//! takes it back on the migration hello: recorded in its own list, so its tick expires them,
//! and **sent again** with the time left. The re-send covers both things the client might have
//! done across the migration - kept the icons (then it is a refresh of the same bits) or
//! dropped them (then the buff, which the player still has the time for, comes back).

use super::*;

impl Session {
    /// The leaving half: save every held buff in wall-clock terms. Called on the Change
    /// Channel request, before the migrate reply.
    pub(super) fn carry_buffs_out(&self) {
        self.carry_buffs_out_at(unix_ms());
    }

    pub(super) fn carry_buffs_out_at(&self, now_unix_ms: i64) {
        let Some(chr) = self.claimed_character() else { return };
        let rows: Vec<store::carriedbuffs::CarriedBuff> = self
            .buffs
            .iter()
            .map(|b| store::carriedbuffs::CarriedBuff {
                bit: b.bit,
                source: b.skill_id,
                reason: b.reason,
                value: b.value,
                expires_unix_ms: if b.expires_ms == u64::MAX {
                    store::carriedbuffs::NEVER
                } else {
                    now_unix_ms.saturating_add(i64::try_from(b.expires_ms.saturating_sub(self.clock_ms)).unwrap_or(i64::MAX))
                },
            })
            .collect();
        match self.store.save_carried_buffs(chr.id, &rows, now_unix_ms) {
            Ok(()) => crate::server::log(&format!(
                "   buffs: {} ({}) carries {} temporary stat(s) to the next channel: bits {:?}",
                chr.name,
                chr.id,
                rows.len(),
                rows.iter().map(|r| r.bit).collect::<Vec<_>>()
            )),
            Err(e) => crate::server::log(&format!("   buffs: could not save {}'s buffs for the channel change: {e}", chr.name)),
        }
    }

    /// The arriving half: take back what the last channel saved, hold it, and re-send it with
    /// the time left - one `0x007D` per source, as it was cast. Called after the SetField.
    pub(super) fn carry_buffs_in(&mut self) -> Vec<Reply> {
        self.carry_buffs_in_at(unix_ms())
    }

    pub(super) fn carry_buffs_in_at(&mut self, now_unix_ms: i64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let rows = match self.store.take_carried_buffs(chr.id, now_unix_ms) {
            Ok(rows) => rows,
            Err(e) => {
                crate::server::log(&format!("   buffs: could not read {}'s carried buffs: {e}", chr.name));
                return Vec::new();
            }
        };
        // One packet per (source, reason, expiry): a skill's stats went out together and come
        // back together, and the client keys its icon on the reason.
        let mut groups: Vec<((u32, u32, i64), Vec<net::buff::TemporaryStat>)> = Vec::new();
        for row in &rows {
            let left_ms = if row.expires_unix_ms == store::carriedbuffs::NEVER {
                None
            } else {
                Some(u64::try_from(row.expires_unix_ms - now_unix_ms).unwrap_or(0))
            };
            let expires_ms = left_ms.map_or(u64::MAX, |l| self.clock_ms.saturating_add(l));
            self.buffs.retain(|b| b.bit != row.bit);
            self.buffs.push(super::buff::ActiveBuff {
                bit: row.bit,
                skill_id: row.source,
                expires_ms,
                value: row.value,
                reason: row.reason,
            });
            // Recovery's icon is a heal the server delivers; the ticks left come with it, one
            // every five seconds up to the same end.
            if row.bit == net::buff::CTS_REGEN {
                if let Some(left) = left_ms.filter(|l| *l > 0) {
                    let every = super::recovery::RECOVERY_TICK_MS;
                    let ticks = left.div_ceil(every);
                    self.recovering = Some(super::recovery::Recovering {
                        per_tick: u32::try_from(row.value).unwrap_or(0),
                        ticks_left: u32::try_from(ticks).unwrap_or(0),
                        next_ms: self.clock_ms.saturating_add(left - (ticks - 1) * every),
                        level: 0,
                    });
                }
            }
            // The EXP coupon is a rate as well as an icon, and the rate is the server's.
            if row.bit == net::buff::CTS_EXP_BUFF_RATE {
                self.exp_coupon = Some(crate::consumables::ExpCoupon {
                    percent: u32::try_from(row.value).unwrap_or(0),
                    expires_ms,
                    item_id: row.source,
                });
            }
            let stat = net::buff::TemporaryStat {
                bit: row.bit,
                value: row.value,
                reason: row.reason,
                duration_ms: left_ms.map_or(0, |l| u32::try_from(l).unwrap_or(u32::MAX)),
            };
            let key = (row.source, row.reason, row.expires_unix_ms);
            match groups.iter_mut().find(|(k, _)| *k == key) {
                Some((_, stats)) => stats.push(stat),
                None => groups.push((key, vec![stat])),
            }
        }
        groups
            .into_iter()
            .map(|((source, _, _), stats)| Reply {
                opcode: net::buff::TEMPORARY_STAT_SET,
                body: net::buff::temporary_stat_set_with_tail(&stats, net::buff::TAIL_LEN),
                what: format!(
                    "TemporaryStatSet: {source}'s buff carried over from the last channel - {} with {} ms left",
                    stats.iter().map(|s| format!("CTS {} = {}", s.bit, s.value)).collect::<Vec<_>>().join(", "),
                    stats[0].duration_ms
                ),
            })
            .collect()
    }
}

fn unix_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    /// One character, claimed on a fresh connection: `(store, session, character id)`.
    fn claimed_session() -> (Arc<Store>, Session, (i64, u32)) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Cobalt".into(), map_id: 101_000_000, level: 30, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        let s = session_for(&store, (account, id));
        (store, s, (account, id))
    }

    /// A new connection - another channel's - claiming `id`.
    fn session_for(store: &Arc<Store>, (account, id): (i64, u32)) -> Session {
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), Arc::new(crate::config::Config::default()), Arc::new(Fields::new()));
        s.claim_for_character(id);
        s
    }

    /// **Cast on one channel, expired by the next.** Magic Guard (a toggle) and a 30 s Speed
    /// buff are held when the player changes channel 10 s in. The arriving session re-sends
    /// both - the toggle with no duration, Speed with 20 s left - and its own tick takes Speed
    /// down at the 20 s mark, which is the `0x007E` nobody used to send.
    #[test]
    fn a_buff_carried_across_a_channel_change_is_expired_by_the_new_channel() {
        let (store, mut old, id) = claimed_session();
        old.clock_ms = 10_000;
        old.buffs.push(super::super::buff::ActiveBuff { bit: 92, skill_id: 1002, expires_ms: 30_000, value: 10, reason: 1002 });
        old.buffs.push(super::super::buff::ActiveBuff { bit: 4, skill_id: 2_001_002, expires_ms: u64::MAX, value: 80, reason: 2_001_002 });
        old.carry_buffs_out_at(1_000_000);
        drop(old);

        let mut new = session_for(&store, id);
        new.clock_ms = 500;
        let out = new.carry_buffs_in_at(1_000_000);
        let sets: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::buff::TEMPORARY_STAT_SET).collect();
        assert_eq!(sets.len(), 2, "one 0x007D per source: {out:?}");
        let speed = net::buff::temporary_stat_set_with_tail(
            &[net::buff::TemporaryStat { bit: 92, value: 10, reason: 1002, duration_ms: 20_000 }],
            net::buff::TAIL_LEN,
        );
        let guard = net::buff::temporary_stat_set_with_tail(
            &[net::buff::TemporaryStat { bit: 4, value: 80, reason: 2_001_002, duration_ms: 0 }],
            net::buff::TAIL_LEN,
        );
        assert!(sets.iter().any(|r| r.body == speed), "Speed, 20 s left");
        assert!(sets.iter().any(|r| r.body == guard), "the toggle, no duration");

        assert!(new.buff_tick(20_499).is_empty(), "not early");
        let gone = new.buff_tick(20_500);
        assert_eq!(gone.len(), 1);
        assert_eq!(gone[0].opcode, net::buff::TEMPORARY_STAT_RESET, "the new channel takes it down");
        assert!(new.holds(4) && !new.holds(92), "the toggle stays");
        assert!(new.carry_buffs_in_at(1_000_000).is_empty(), "taken once");
    }

    /// **Wired, through the real packets**: a `0x00D2` on the leaving channel saves the buff,
    /// and the arriving channel's migration hello sends it back after the SetField.
    #[test]
    fn the_change_channel_request_and_the_hello_carry_the_buff() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Pebble".into(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let config = crate::config::Config {
            channel_id: 0,
            channels: vec!["127.0.0.1:8485".parse().unwrap(), "127.0.0.1:8486".parse().unwrap()],
            ..crate::config::Config::default()
        };
        let mut old = Session::joining(store.clone(), Arc::new(config), Arc::new(Fields::new()));
        old.claim_for_character(id);
        old.buffs.push(super::super::buff::ActiveBuff { bit: 92, skill_id: 1002, expires_ms: 60_000, value: 10, reason: 1002 });
        // The `0x00D2` body, preamble first: the client's literal 100, ten bytes, the target.
        let mut cc = net::channel::CLIENT_CHANGE_CHANNEL.to_le_bytes().to_vec();
        cc.extend_from_slice(&100u32.to_le_bytes());
        cc.extend_from_slice(&[0u8; 10]);
        cc.push(1);
        cc.extend_from_slice(&0u32.to_le_bytes());
        let _ = old.handle(&cc);
        drop(old);

        let mut new = Session::joining(store.clone(), Arc::new(crate::config::Config { channel_id: 1, ..Default::default() }), Arc::new(Fields::new()));
        new.claim_for_character(id);
        let out = new.handle(&crate::session::CLIENT_MIGRATION_HELLO.to_le_bytes());
        let set_field = out.iter().position(|r| r.opcode == net::opcode::SET_FIELD).expect("the SetField");
        let buff = out.iter().position(|r| r.opcode == net::buff::TEMPORARY_STAT_SET).expect("the carried buff");
        assert!(buff > set_field, "after the SetField, which builds the stage it belongs to");
        assert!(new.holds(92), "and held, so this channel's tick takes it down");
    }

    /// Recovery's heal comes across with its icon: 12 s left of a 4-per-tick cast is three more
    /// heals, at 2, 7 and 12 s, ending with the icon.
    #[test]
    fn recoverys_heal_is_carried_with_its_icon() {
        let (store, mut old, id) = claimed_session();
        old.clock_ms = 18_000;
        old.buffs.push(super::super::buff::ActiveBuff { bit: net::buff::CTS_REGEN, skill_id: 1001, expires_ms: 30_000, value: 4, reason: 1001 });
        old.carry_buffs_out_at(1_000_000);
        drop(old);
        let mut new = session_for(&store, id);
        let _ = new.carry_buffs_in_at(1_000_000);
        let r = new.recovering.expect("the heal came across");
        assert_eq!((r.per_tick, r.ticks_left, r.next_ms), (4, 3, 2_000));
    }

    /// The EXP coupon's rate comes across with its icon, or the second channel would show a
    /// coupon and pay plain experience.
    #[test]
    fn the_exp_coupon_rate_is_carried_with_its_icon() {
        let (store, mut old, id) = claimed_session();
        old.buffs.push(super::super::buff::ActiveBuff {
            bit: net::buff::CTS_EXP_BUFF_RATE,
            skill_id: 5_211_000,
            expires_ms: 600_000,
            value: 200,
            reason: net::buff::item_reason(5_211_000),
        });
        old.carry_buffs_out_at(1_000_000);
        drop(old);
        let mut new = session_for(&store, id);
        assert_eq!(new.carry_buffs_in_at(1_000_000).len(), 1);
        let coupon = new.exp_coupon.expect("the rate came across");
        assert_eq!((coupon.percent, coupon.item_id, coupon.expires_ms), (200, 5_211_000, 600_000));
    }
}
