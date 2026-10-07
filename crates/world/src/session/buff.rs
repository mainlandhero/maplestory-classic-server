//! Casting a buff, holding it, and letting it go.
//!
//! The owner, 2026-08-21 and again 2026-08-22: *"Nimble Feet still does not give me a buff despite
//! me activating the skill."*
//!
//! # It was arriving the whole time
//!
//! `0x013C` has been in `world.log` since the day skills went in - one on 2026-08-22 at
//! 13:00:49, 51 bytes, `skillId 1002 level 3` - logged as `UNKNOWN` and dropped. This is the
//! third instance this week of a subsystem that was decoded, written up and never connected,
//! which is why `CLAUDE.md` has a section called "Built is not wired".
//!
//! # `0x013C` does not latch, and that is measured rather than assumed
//!
//! Every other request this session answers - the pick-up, the inventory move, the storage
//! window, the AP dialog - sets a one-outstanding flag that exactly one inbound opcode
//! clears, and a refusal that sends the wrong packet kills the feature for the session. This
//! one does not: the 13:00:49 cast went **entirely unanswered** and the client played on for
//! **four more minutes**, walking, fighting and changing maps. So a refusal here can be a
//! chat line, and it is.
//!
//! # The client never removes a temporary stat by itself
//!
//! `0x007D`'s duration reaches the client's own `tExpire`, and it was reasonable to think
//! that meant the client would drop the stat on schedule. It does not: it **flashes the icon
//! and waits**. The owner, 2026-08-22: *"after the expiry, the buff did not go away. (It just kept
//! flashing, but the temporary stats were still there)"* - and at that moment the client sent
//! nothing at all.
//!
//! Right-clicking the icon is how it asks: `0x013F`, retried every ~180 ms until something
//! answers. So **removal is the server's job on both paths**, and `0x007E` is mandatory.
//!
//! # Every effect hangs off "the cast was allowed"
//!
//! MP, the stat change, the grant and the cooldown stamp are all reached through one `Ok`,
//! for the reason `CLAUDE.md` records under the Heena quest: separately-gated effects are how
//! one gets missed. A refused cast costs nothing and stamps nothing.

use super::*;

/// A temporary stat this character is holding, and when it runs out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ActiveBuff {
    pub(super) bit: u32,
    pub(super) skill_id: u32,
    /// `u64::MAX` for a toggle. See [`Session::grant_buff_with_tail`].
    pub(super) expires_ms: u64,
    /// What the client was told the stat is worth. Read back by the server-side halves that
    /// need it - Hyper Body's percent for the HP ceiling, Power Guard's for the reflection,
    /// Combo's orb count - so the number the server acts on is the number the client drew.
    pub(super) value: i16,
    /// What the client was told the source is: the skill id, or an item's negative reason
    /// (`net::buff::item_reason`). Kept because the two id spaces overlap - skill `2001002` is
    /// Magic Guard and item `2001002` is a potion - and a re-send must say the same thing.
    pub(super) reason: u32,
}

/// What one `0x013C` does, decided **before** anything is spent so that a refusal costs
/// nothing and a success pays for exactly what it produced.
enum CastEffect {
    /// A temporary stat from one of the three buff tables.
    Stat(net::buff::BuffLevel),
    /// Beginner Recovery's heal-over-time.
    Recovery(crate::skilltable::SkillLevel),
    /// The Cleric's Heal - HP now, to the caster and the party on this field.
    Heal,
    /// A skill the client's table prices but the server grants nothing for: Teleport, Flash
    /// Jump, the summons, Mystic Door. **The cost is still taken**, or the client - which
    /// already spent it locally - has its MP silently restored by the next `0x007C`, which
    /// is the exact bug the attack path fixed on 2026-08-28.
    CostOnly,
}

impl Session {
    /// `0x013C` - the player pressed a skill.
    ///
    /// # Every cast pays, whether or not the server grants a stat (2026-09-07)
    ///
    /// Until the second- and third-job audit a skill with no buff table behind it - Teleport,
    /// Flash Jump, every Booster's HP half, Spell Booster's Magic Rock - was answered with the
    /// "does not grant" notice and **nothing was spent**. The client had already spent it
    /// locally, so the next `0x007C` to carry the MP field handed it back: the same stale-total
    /// bug the attack path fixed on 2026-08-28, on a second path. Now the client's own table
    /// prices every cast (`mpCon`, `hpCon`, `itemCon`), and the price is taken through one
    /// transition whatever comes after it.
    ///
    /// **Items are taken only when something was produced.** A Summoning Rock for a summon
    /// this server cannot show would be an item gone for nothing; the MP still goes, because
    /// the client's copy already did.
    pub(super) fn on_skill_use(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let Some(req) = net::buff::parse_skill_use(body) else {
            return self.notice(format!("Unreadable skill-use body: {body:02x?}"));
        };
        let (skill_id, asked) = (req.skill_id, req.level);

        // Recovery is HP over time rather than a stat, so it comes from the generated table
        // instead of the buff tables; everything after this (has-the-skill, cooldown, the
        // costs) is shared with every other skill so the two cannot drift.
        // `crate::session::recovery`.
        let recovery_row = (skill_id == recovery::RECOVERY_SKILL_ID)
            .then(|| self.recovery_level(asked).cloned())
            .flatten();
        let row = self.config.firstjob.level(skill_id, asked).copied();

        let effect = match (self.buff_level_for(skill_id, asked, &chr), recovery_row, row) {
            (Some(level), _, _) => CastEffect::Stat(level),
            (None, Some(r), _) => CastEffect::Recovery(r),
            (None, None, Some(_)) if skill_id == crate::advbuffs::HEAL => CastEffect::Heal,
            (None, None, Some(_)) => CastEffect::CostOnly,
            (None, None, None) => {
                // Not in the client's own Skill.wz at all. Still answered - a `0x013C` does not
                // latch (module docs), so a chat line is a complete answer.
                return self.notice(format!(
                    "This server does not know skill {skill_id}: it is not in this client's \
                     Skill.wz. Three Snails is an attack, and Disorder is a debuff on the MOB \
                     rather than a stat on you."
                ));
            }
        };

        // The price, from whichever table described the skill; the `hpCon` always from the
        // generated row, because the buff tables never carried one and ten Boosters have it.
        let (mp_cost, hp_cost, cooldown_seconds) = match (&effect, row) {
            (CastEffect::Stat(l), r) => (
                u32::from(l.mp_cost),
                r.and_then(|r| r.hp_con).unwrap_or(0),
                l.cooldown_seconds,
            ),
            (CastEffect::Recovery(r), _) => {
                (r.mp_con.unwrap_or(0), 0, r.cooltime_seconds.unwrap_or(0))
            }
            (_, Some(r)) => {
                (r.mp_con.unwrap_or(0), r.hp_con.unwrap_or(0), r.cooltime_seconds.unwrap_or(0))
            }
            (_, None) => (0, 0, 0),
        };
        let mp_cost = self.amplified_mp(skill_id, mp_cost);

        // **The client's claim about its own level is checked, not trusted.** It sends the
        // level it thinks it has, and nothing on this socket authenticates anybody.
        let has = self
            .store
            .skills(chr.id)
            .unwrap_or_default()
            .into_iter()
            .find(|s| s.id == skill_id)
            .map(|s| s.level)
            .unwrap_or(0);
        if has < asked {
            return self.notice(format!(
                "You asked to cast skill {skill_id} at level {asked} and you have it at {has}."
            ));
        }

        let now = self.clock_ms;
        if let Some(ready) = self.skill_ready_ms.get(&skill_id).copied() {
            if now < ready {
                let left = (ready - now).div_ceil(1000);
                // Loud on purpose. A silent refusal here would be indistinguishable from the
                // bug being fixed - which is exactly what a run is trying to tell apart.
                return self.notice(format!(
                    "Skill {skill_id} is on cooldown for another {left}s (cooltime is \
                     {cooldown_seconds}s in Skill.wz)."
                ));
            }
        }
        if chr.mp < mp_cost {
            return self.notice(format!(
                "Not enough MP: skill {skill_id} level {asked} costs {mp_cost} and you have {}.",
                chr.mp
            ));
        }
        // An item the cast consumes, and only for a cast that produces something.
        let item = match effect {
            CastEffect::CostOnly => None,
            _ => row.and_then(|r| Some((r.item_con?, r.item_con_no.unwrap_or(1).max(1)))),
        };
        if let Some((item_id, n)) = item {
            if !self.has_items(chr.id, store::InventoryType::Etc, item_id, n) {
                return self.notice(format!(
                    "Skill {skill_id} consumes {n} x item {item_id} and you do not have them. \
                     Nothing was cast."
                ));
            }
        }

        // ---------------------------------------------------------------- the transition
        chr.mp = chr.mp.saturating_sub(mp_cost);
        if hp_cost > 0 {
            // Floored at 1, as the attack path floors Slash Blast: a skill's own price never
            // kills its caster, and the client does not let the cast out at 1 HP anyway.
            chr.hp = chr.hp.saturating_sub(hp_cost).max(1);
        }
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not spend the MP, so nothing was cast: {e}"));
        }
        self.skill_ready_ms
            .insert(skill_id, now.saturating_add(u64::from(cooldown_seconds) * 1000));

        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                mp: Some(chr.mp),
                hp: (hp_cost > 0).then_some(chr.hp),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: skill {skill_id} level {asked} cost {mp_cost} mp -> {}/{}{}",
                chr.mp,
                chr.max_mp,
                if hp_cost > 0 {
                    format!(" and {hp_cost} hp -> {}/{} (hpCon)", chr.hp, chr.max_hp)
                } else {
                    String::new()
                }
            ),
        }];
        if let Some((item_id, n)) = item {
            out.extend(self.take_items(chr.id, store::InventoryType::Etc, item_id, n));
        }
        match effect {
            CastEffect::Stat(level) => {
                out.extend(self.grant_buff(skill_id, level, now));
                // A party buff reaches the rest of the party on this field. Hung off the
                // same transition as the caster's own grant: a refused cast shares nothing.
                self.share_party_buff(skill_id, asked);
                if skill_id == crate::advbuffs::DRAGON_BLOOD {
                    let every = row.and_then(|r| r.y).and_then(|y| u64::try_from(y).ok()).unwrap_or(3);
                    self.dragon_blood_next_ms = now.saturating_add(every * 1000);
                }
            }
            CastEffect::Recovery(r) => out.extend(self.start_recovery(&r, asked)),
            CastEffect::Heal => out.extend(self.heal_cast(asked)),
            CastEffect::CostOnly => crate::server::log(&format!(
                "   cast: skill {skill_id} level {asked} - {mp_cost} mp{} taken, no stat \
                 granted. Its effect is the client's own (Teleport, Flash Jump) or is not \
                 built (summons, Mystic Door); research/second-third-job-audit-2026-09-07.md",
                if hp_cost > 0 { format!(" and {hp_cost} hp") } else { String::new() }
            )),
        }
        out
    }

    /// **Send a party buff to every other party member standing on this field.**
    ///
    /// The owner, 2026-09-06: *"party buffs should apply to everyone in the party who is in the
    /// same map."* Which skills are party buffs is the generated table's business -
    /// `SkillCombat::party_rect`, the `lt`/`rb` rectangle that Rage and Haste carry and the
    /// self buffs do not. What crosses is the fact (`Event::PartyBuff`), not the packet: each
    /// recipient builds its own `0x007D` and owns its own expiry, so the `0x007E` comes from
    /// the session that can actually send it to that client.
    ///
    /// "Same map" is the owner's rule and it is what is checked. The rectangle's size is not: the
    /// client draws the cast's area from it, but a member across the map still gets the buff
    /// here. That is a decision, written down so it can be reversed rather than discovered.
    fn share_party_buff(&mut self, skill_id: u32, level: u32) {
        let Some(chr) = self.claimed_character() else { return };
        if !self.config.firstjob.get(skill_id).is_some_and(|s| s.party_rect) {
            return;
        }
        // Bound and dropped before the bus is touched: the parties guard must not be held
        // across another lock.
        let members: Vec<u32> = self
            .fields
            .parties()
            .party_of(chr.id)
            .map(|p| p.members.clone())
            .unwrap_or_default();
        if members.is_empty() {
            crate::server::log(&format!(
                "   party buff: skill {skill_id} is a party buff, but {} ({}) is in no party - self only",
                chr.name, chr.id
            ));
            return;
        }
        let Some(map) = self.bus().map_of(self.subscriber) else { return };
        let here = self.bus().characters_on(map, &members);
        let mut sent = Vec::new();
        for member in here.into_iter().filter(|m| *m != chr.id) {
            let delivered = self.bus().send_to_character(
                member,
                crate::broadcast::Event::PartyBuff { skill_id, level, caster: chr.id },
            );
            if delivered {
                sent.push(member.to_string());
            } else {
                crate::server::log(&format!(
                    "   party buff: character {member} is on map {map} per the party roster but nobody on this channel is playing them"
                ));
            }
        }
        crate::server::log(&format!(
            "   party buff: skill {skill_id} level {level} from {} ({}) shared with {} member(s) on map {map}{} - {} in the party, {} elsewhere or offline",
            chr.name,
            chr.id,
            sent.len(),
            if sent.is_empty() { String::new() } else { format!(" [{}]", sent.join(", ")) },
            members.len(),
            members.len().saturating_sub(sent.len() + 1),
        ));
    }

    /// **Receive a party buff another member cast** - `Event::PartyBuff` arriving over the
    /// bus. No skill check, no MP, no cooldown: those were the caster's. The level is
    /// resolved through the same three tables the keypress uses, so what the recipient
    /// gets is exactly what the caster got, computed against the recipient's own record
    /// where a value depends on the wearer.
    pub(super) fn receive_party_buff(&mut self, skill_id: u32, level: u32, caster: u32) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(bl) = self.buff_level_for(skill_id, level, &chr) else {
            // The caster's table granted it, so this cannot happen unless the two sessions
            // load different tables. Said out loud rather than dropped.
            crate::server::log(&format!(
                "   party buff: skill {skill_id} level {level} from {caster} reached {} ({}) but no table here grants it",
                chr.name, chr.id
            ));
            return Vec::new();
        };
        let now = self.clock_ms;
        crate::server::log(&format!(
            "   party buff: {} ({}) receives skill {skill_id} level {level} from character {caster} - CTS bit {} = {}{} for {} s",
            chr.name,
            chr.id,
            bl.bit,
            bl.value,
            bl.second.map(|s| format!(" and bit {} = {}", s.bit, s.value)).unwrap_or_default(),
            bl.seconds
        ));
        self.grant_buff(skill_id, bl, now)
    }

    /// **Every buff this server can grant, from either table.**
    ///
    /// `net::buff` holds the three that were decoded first - Nimble Feet, Magic Guard, Magic
    /// Armor. `net::jobbuffs` holds the other three first-job buffs, whose CTS bits were found
    /// on 2026-08-28 by **re-deriving the index-to-name table**: the earlier pass produced 323
    /// names, this one produces 408, and the 85 it had been missing include every bit needed
    /// here - 86 `PDD`, 88 `ACC`, 89 `EVA`, 99 `DarkSight`. `CLAUDE.md`'s oldest rule, again:
    /// the old absence was a property of the search, not of the client.
    ///
    /// **Order matters only in that the two tables must not overlap**, and they do not: the
    /// skill ids are disjoint and `jobbuffs::buff_level` returns `None` for everything outside
    /// its four.
    ///
    /// **A third source since 2026-09-06: the generated skill table itself**, for the buffs
    /// whose grants are flat `indie*` columns - Haste, Rage, Iron Will and whatever else in
    /// `Skill.wz` is written that way. Consulted last, so the two hand-checked tables keep
    /// their say over every skill they name; see [`Self::table_buff_level`] for what it will
    /// and will not build.
    ///
    /// Disorder returns `None` from all three, on purpose. It is a debuff on the **mob**, and
    /// this server has no packet for that - `research/first-job-buffs.md` §5. A `0x013C` for
    /// it is still answered, with the chat line above; the alternative is the frozen UI.
    pub(super) fn buff_level_for(
        &self,
        skill_id: u32,
        level: u32,
        chr: &net::opcode::Character,
    ) -> Option<net::buff::BuffLevel> {
        net::buff::buff_level(skill_id, level)
            .or_else(|| net::jobbuffs::buff_level(skill_id, level, self.weapon_defence(chr)))
            .or_else(|| self.table_buff_level(skill_id, level))
    }

    /// A buff level read out of `gm-handbook/skills.txt` - the `indie*` columns, and since
    /// 2026-09-07 the flag buffs `crate::advbuffs` names.
    ///
    /// Built when the row grants one or two stats this server knows a CTS bit for, and has
    /// either a `time` or is a toggle; `None` otherwise, which lands the keypress on the
    /// cost-only path rather than on a guess. The bit for each column:
    ///
    /// | column | bit | standing |
    /// |---|---|---|
    /// | `indieSpeed` | 92 | [L] - Nimble Feet, on a client |
    /// | `indieJump` | 93 | [D] - `net::jobbuffs::CTS_JUMP` |
    /// | `indiePad` | 84 | [D] - `net::jobbuffs::CTS_WEAPON_ATTACK` |
    /// | `indieMad` | 85 | [D] - `net::jobbuffs::CTS_MAGIC_ATTACK` |
    /// | `indiePdd` | 86 | [D]/[L] - Iron Body's bit, drawn on a client |
    /// | `indieMdd` | 87 | [D] - Magic Armor's second bit |
    /// | `indieAcc` | 88 | [L] - Focus's bit, `net::jobbuffs::CTS_ACCURACY` |
    /// | `indieEva` | 89 | [D] - `net::jobbuffs::CTS_AVOIDABILITY` |
    /// | `indieMhpR` | 94 | [D] - `net::jobbuffs::CTS_MAX_HP`, a percent |
    ///
    /// and for a skill in `advbuffs::flag_buff` - Boosters, Soul Arrow, Power Guard, the
    /// Charges and the rest - that table's bit **goes first**, so `BuffLevel::bit` is the flag
    /// and any `indie*` column the same row carries (Dragon Blood's `indiePad`) is the second.
    ///
    /// **More than two grants is refused, not truncated.** `BuffLevel` carries two, and
    /// granting the first two of three would be the half-a-buff failure `BuffLevel::second`'s
    /// doc describes. Bless is the closest call in the census - `indieAcc`, `indieEva` and an
    /// `x` that is a heal bonus, not a stat - and it fits.
    ///
    /// A percent column (`indiePddR`, Iron Body) is not a grant and is not read here; the
    /// `jobbuffs` table resolves that one against the wearer's own defence and runs first.
    fn table_buff_level(&self, skill_id: u32, level: u32) -> Option<net::buff::BuffLevel> {
        let skill = self.config.firstjob.get(skill_id)?;
        let row = skill.level(level)?;
        let flag = crate::advbuffs::flag_buff(skill_id);
        let mut grants: Vec<net::buff::StatGrant> = Vec::new();
        if let Some(fb) = flag {
            // A flag whose value column is missing is a refusal, not a zero: the client would
            // read `0` as "off" for the very bit that was just set.
            let value = fb.value.resolve(row, level)?;
            grants.push(net::buff::StatGrant { bit: fb.bit, value });
        }
        grants.extend(
            [
                (row.indie_speed, net::buff::CTS_SPEED),
                (row.indie_jump, net::jobbuffs::CTS_JUMP),
                (row.indie_pad, net::jobbuffs::CTS_WEAPON_ATTACK),
                (row.indie_mad, net::jobbuffs::CTS_MAGIC_ATTACK),
                (row.indie_pdd, net::buff::CTS_WEAPON_DEFENCE),
                (row.indie_mdd, net::buff::CTS_MAGIC_DEFENCE),
                (row.indie_acc, net::jobbuffs::CTS_ACCURACY),
                (row.indie_eva, net::jobbuffs::CTS_AVOIDABILITY),
                (row.indie_mhp_r.map(|v| i32::try_from(v).unwrap_or(0)), net::jobbuffs::CTS_MAX_HP),
            ]
            .into_iter()
            .filter_map(|(value, bit)| {
                let value = value.filter(|v| *v != 0)?;
                Some(net::buff::StatGrant { bit, value: i16::try_from(value).unwrap_or(i16::MAX) })
            }),
        );
        let (first, rest) = grants.split_first()?;
        if rest.len() > 1 {
            crate::server::log(&format!(
                "   buff: skill {skill_id} level {level} grants {} stats and BuffLevel carries two - refused rather than halved",
                grants.len()
            ));
            return None;
        }
        // A `time` is a duration. No `time` on a toggle row is a toggle. No `time` on anything
        // else is a passive, and a passive is not a stat this can build.
        let duration = match row.time_seconds.filter(|s| *s > 0) {
            Some(seconds) => net::buff::BuffDuration::Seconds(seconds),
            None if flag.is_some_and(|f| f.toggle) || skill.processtype == Some(113) => {
                net::buff::BuffDuration::Toggle
            }
            None => return None,
        };
        Some(net::buff::BuffLevel {
            mp_cost: u16::try_from(row.mp_con.unwrap_or(0)).unwrap_or(u16::MAX),
            seconds: duration.seconds(),
            cooldown_seconds: row.cooltime_seconds.unwrap_or(0),
            bit: first.bit,
            value: first.value,
            second: rest.first().copied(),
            duration,
        })
    }

    /// The character's Weapon Defence, which **Iron Body alone** reads.
    ///
    /// Its `indiePddR` is a **percent** while CTS bit 86 is a flat add, so the server has to
    /// resolve the one into the other - `research/first-job-buffs.md` §3.2. The resolution is
    /// **[I]** and one launch decides it: a W.Def that rises by 25% of its base means this is
    /// right, and a rise of exactly 25 regardless of the base means the raw percent reached
    /// the wire.
    ///
    /// **`0` is a safe answer, not a failure.** `jobbuffs` yields a working cast that adds
    /// nothing rather than a refusal, which is the right way round for a debug server.
    ///
    /// # This is not the same sum `incoming_damage_for` uses, and that is deliberate
    ///
    /// That function sums equipment `inc_pdd` and **omits the `floor(STR/4)` seed**. The stat
    /// window shows both, and Iron Body's percentage applies to what the window shows, so this
    /// adds the seed. The two are now knowingly different rather than accidentally different;
    /// whether `incoming_damage_for` should also include the seed is a behaviour change on
    /// working combat code and is left alone until someone measures it.
    pub(super) fn weapon_defence(&self, chr: &net::opcode::Character) -> i16 {
        let equipment: u32 =
            self.dressed(chr).iter().map(|(_, _, s)| u32::from(s.stats.inc_pdd)).sum();
        let total = crate::damage::wdef_from_strength(u32::from(chr.strength)) + equipment;
        i16::try_from(total).unwrap_or(i16::MAX)
    }

    /// Put the stat on, replacing whatever held that bit before.
    ///
    /// Shared by `0x013C` and the `!buff` GM command, so the command tests the packet the
    /// skill sends rather than a second builder that could drift from it.
    pub(super) fn grant_buff(
        &mut self,
        skill_id: u32,
        level: net::buff::BuffLevel,
        now_ms: u64,
    ) -> Vec<Reply> {
        self.grant_buff_with_tail(skill_id, level, now_ms, net::buff::TAIL_LEN)
    }

    /// [`Self::grant_buff`] with the tail length chosen by the caller - `!buff`'s third
    /// argument. See `net::buff::TAIL_LEN` for why that number is slack and not a length.
    /// **How much of an incoming hit Magic Guard sends to MP**, as a percent, or `0`.
    ///
    /// Read from the buff this session is holding rather than from the skill table, because
    /// the level the player actually cast is what matters and that is what `buffs` records.
    ///
    /// The value is the WZ's `x` - 30 at level 1 rising to 80 - and it is a **percent**, which
    /// was measured together with the bit: the client's hit handler multiplies by the damage
    /// and divides by 100. Same measurement, so the unit cannot be wrong independently of the
    /// bit being wrong.
    pub(super) fn magic_guard_percent(&self) -> u32 {
        self.buffs
            .iter()
            .find(|b| b.skill_id == net::buff::MAGIC_GUARD)
            .and_then(|b| {
                let level = self.store.skill_level(self.claimed_character()?.id, b.skill_id).ok()?;
                net::buff::buff_level(net::buff::MAGIC_GUARD, level)
            })
            .map(|l| u32::try_from(l.value).unwrap_or(0))
            .unwrap_or(0)
    }

    /// **The weapon defence a held buff is adding right now**, or `0`.
    ///
    /// The owner, 2026-08-28: *"Iron Body did not seem to reduce the damage I take."* It could not
    /// have. `incoming_damage_for` summed equipment `inc_pdd` and nothing else, so the buff
    /// set CTS bit 86 on the client, drew its icon, cost MP - and the server's own damage
    /// model never heard about it.
    ///
    /// That is the same shape as the Magic Guard bug this file already documents: setting the
    /// bit buys an icon, and the arithmetic that makes the buff *mean* something is the
    /// server's. `magic_guard_percent` is the sibling.
    ///
    /// Read from the buff table rather than the skill table, because the level the player
    /// actually cast is what matters, and that is what `buffs` records.
    pub(super) fn held_weapon_defence(&self) -> u32 {
        let Some(chr) = self.claimed_character() else { return 0 };
        self.buffs
            .iter()
            .filter(|b| b.bit == net::buff::CTS_WEAPON_DEFENCE)
            .filter_map(|b| {
                let level = self.store.skill_level(chr.id, b.skill_id).ok()?;
                // The same third argument the grant used, so the resolved flat value matches
                // what the client was told rather than being recomputed from a stale base.
                let base = self.weapon_defence(&chr);
                net::buff::buff_level(b.skill_id, level)
                    .or_else(|| net::jobbuffs::buff_level(b.skill_id, level, base))
            })
            .map(|l| u32::try_from(l.value).unwrap_or(0))
            .sum()
    }

    pub(super) fn grant_buff_with_tail(
        &mut self,
        skill_id: u32,
        level: net::buff::BuffLevel,
        now_ms: u64,
        tail: usize,
    ) -> Vec<Reply> {
        // **`all_granted_by`, not `granted_by` - a skill may grant more than one stat.**
        //
        // Magic Armor sets weapon defence AND magic defence. `granted_by` returns only the
        // first, so this call site would have set one and silently dropped the other - and
        // `research/magic-damage.md` says exactly what that would have cost: on screen it
        // reads as "only W. Def moved", which is the signature of *the bit pair being off by
        // one*. It would have produced a confident wrong retraction from a real observation,
        // on the very run meant to promote those two bits from [D] to [L].
        let stats = level.all_granted_by(skill_id);
        // One holder per bit. Two entries for the same stat would leave the second expiry
        // clearing a buff the first had already replaced - and the client tracks one value
        // per bit, so our table has to as well.
        // **A toggle has no expiry.** Found 2026-09-07 while adding five more of them: a
        // `BuffDuration::Toggle` has `seconds 0`, so this used to record `expires_ms = now`,
        // and `buff_tick` - which clears anything at or past its expiry - took Magic Guard
        // off again on the very next pass of the session loop, `0x007E` and all. The one
        // Magic Guard test grants and hits at the same instant, so it never saw a tick. A
        // toggle is held until it is cast again or right-clicked, and `u64::MAX` is how
        // `buff_tick` is told so.
        let expires_ms = if level.duration.is_toggle() {
            u64::MAX
        } else {
            now_ms.saturating_add(u64::from(stats[0].duration_ms))
        };
        for stat in &stats {
            self.buffs.retain(|b| b.bit != stat.bit);
            self.buffs.push(ActiveBuff { bit: stat.bit, skill_id, expires_ms, value: stat.value, reason: stat.reason });
        }
        let stat = stats[0];
        let body = net::buff::temporary_stat_set_with_tail(&stats, tail);
        vec![Reply {
            opcode: net::buff::TEMPORARY_STAT_SET,
            body,
            what: format!(
                "TemporaryStatSet: skill {skill_id} grants CTS bit {} = {} for {} ms ({} s from Skill.wz), \
                 {tail}-byte tail. The 18-byte tail threw an unhandled C++ exception at 0x142d5690c on \
                 2026-08-22 - the u32 reader's own `cmp edi,4 / jb` underflow path - so this length is \
                 SLACK around an unknown, not a computed size. net::buff::TAIL_LEN has the working",
                stat.bit, stat.value, stat.duration_ms, level.seconds
            ),
        }]
    }

    /// Expire buffs whose time is up, and **tell the client**, because it will not do it.
    ///
    /// # The retraction, and the observation that forced it
    ///
    /// This function sent `0x007E` on expiry, then stopped, and now sends it again. The
    /// middle step was wrong and one run said so.
    ///
    /// The reasoning for stopping was that the client holds its own `tExpire` - true, and
    /// `research/buffs.md` §5.3 reads the decoder storing it - so it would drop the stat by
    /// itself and the packet was pure risk on the one path every buff takes. The owner,
    /// 2026-08-22: *"after the expiry, the buff did not go away. (It just kept flashing, but
    /// the temporary stats were still there)"*
    ///
    /// **`tExpire` drives the flashing and nothing else.** The client sent nothing at the
    /// thirty-second mark - not a request, not a report - and kept the stat. Removal is the
    /// server's job, and `0x007E` is mandatory.
    ///
    /// That is what the plan's outcome table called "the client does not self-expire after
    /// all - harmless, and very informative", which is the only reason the wrong version was
    /// worth shipping for one run: it was written down as a claim that could come back false,
    /// and it did.
    pub(super) fn buff_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        let done: Vec<ActiveBuff> =
            self.buffs.iter().copied().filter(|b| now_ms >= b.expires_ms).collect();
        if done.is_empty() {
            return Vec::new();
        }
        self.buffs.retain(|b| now_ms < b.expires_ms);
        let bits: Vec<u32> = done.iter().map(|b| b.bit).collect();
        let skills: Vec<u32> = done.iter().map(|b| b.skill_id).collect();
        self.reset_reply(&bits, net::buff::TAIL_LEN, format!("expired (from skill(s) {skills:?})"))
    }

    /// `0x013F` - the player right-clicked a buff icon.
    ///
    /// # It retries until something answers
    ///
    /// Fourteen of these arrived in three seconds, one every ~180 ms, all identical and all
    /// dropped. That cadence is a retry loop rather than fourteen clicks, and it is the
    /// clearest statement the client has made that it is waiting on us.
    ///
    /// The body names both the skill and the CTS bits - `u32 skillId`, five bytes, then the
    /// same 124-byte mask `0x007D` uses. **The mask is preferred over the skill id** because
    /// it is what the client is actually pointing at; the skill id is used only to explain
    /// the refusal when nothing matches, and a mismatch between the two is worth a log line
    /// rather than a guess.
    pub(super) fn on_skill_cancel(&mut self, body: &[u8]) -> Vec<Reply> {
        if self.claimed_character().is_none() {
            return Vec::new();
        }
        let Some(req) = net::buff::parse_skill_cancel(body) else {
            return self.notice(format!(
                "Unreadable buff-cancel body, {} bytes - expected {}.",
                body.len(),
                net::buff::CLIENT_SKILL_CANCEL_LEN
            ));
        };
        // Only bits we believe are held. Answering for a bit we never granted would tell the
        // client to clear something it may hold from elsewhere.
        //
        // **But cancel the whole SKILL, not just the bit that was named.** A buff is a skill's
        // set of stats: Dark Sight grants invisibility (99) *and* a Speed penalty (92), and
        // Focus grants Accuracy (88) *and* Avoidability (89). If a right-click names one bit
        // and this cleared only that one, the player keeps the other half forever - and for
        // Dark Sight the half left behind is the **drawback**, so on screen it is a cancelled
        // buff and a character who is permanently slow for no visible reason. Nobody reports
        // that as a buff bug.
        //
        // `research/first-job-buffs.md` §7 item 3 names this hazard; it is the same shape as
        // `grant_buff_with_tail`'s `all_granted_by` note, which is about the other direction
        // of the identical mistake.
        let skills: Vec<u32> = self
            .buffs
            .iter()
            .filter(|h| req.bits.contains(&h.bit))
            .map(|h| h.skill_id)
            .collect();
        let held: Vec<u32> = self
            .buffs
            .iter()
            .filter(|h| req.bits.contains(&h.bit) || skills.contains(&h.skill_id))
            .map(|h| h.bit)
            .collect();
        if held.is_empty() {
            // **Still worth a line, because this is the retry case.** Fourteen unanswered
            // requests is what the run before this looked like, and silence here would be
            // indistinguishable from the handler not existing.
            return self.notice(format!(
                "Nothing to cancel for skill {}: the server is not holding CTS bit(s) {:?}.",
                req.skill_id, req.bits
            ));
        }
        self.buffs.retain(|b| !held.contains(&b.bit));
        if held.contains(&net::buff::CTS_REGEN) {
            self.stop_recovery();
        }
        self.reset_reply(
            &held,
            net::buff::TAIL_LEN,
            format!("cancelled by right-click on skill {}", req.skill_id),
        )
    }

    /// The value of a held temporary stat, or `0` when it is not held.
    pub(super) fn held_value(&self, bit: u32) -> i16 {
        self.buffs.iter().find(|b| b.bit == bit).map(|b| b.value).unwrap_or(0)
    }

    pub(super) fn holds(&self, bit: u32) -> bool {
        self.buffs.iter().any(|b| b.bit == bit)
    }

    /// The level of the skill that put `bit` on, from the character's own record.
    fn held_skill_level(&self, bit: u32) -> Option<(u32, u32)> {
        let held = self.buffs.iter().find(|b| b.bit == bit)?;
        let chr = self.claimed_character()?;
        let level = self.store.skill_level(chr.id, held.skill_id).ok().filter(|l| *l > 0)?;
        Some((held.skill_id, level))
    }

    /// **Element Amplification's MP side.** Its tooltip: *"MP cost increased to 120%"* - and
    /// the column is `x = 20..50`, the **increase**, not the multiplier. The first version of
    /// this read `x` as the multiplier, clamped it up to 100 and scaled nothing; the unit test
    /// caught it (`CLAUDE.md`: the unit, not the arithmetic). The client raises what it spends
    /// locally by the same amount while the toggle is on; if the server did not, the next
    /// `0x007C` would hand the difference back. The toggle's own cast is not amplified. **[I]**
    /// that it applies to every skill rather than only the attack spells; the tooltip says "MP
    /// cost" without qualification.
    pub(super) fn amplified_mp(&self, skill_id: u32, mp: u32) -> u32 {
        if crate::advbuffs::ELEMENT_AMPLIFICATION.contains(&skill_id) {
            return mp;
        }
        let Some((amp, level)) = self.held_skill_level(net::jobbuffs::CTS_ELEMENT_AMP) else {
            return mp;
        };
        let Some(x) = self.config.firstjob.level(amp, level).and_then(|r| r.x) else { return mp };
        let increase = u64::try_from(x).unwrap_or(0);
        u32::try_from(u64::from(mp) * (100 + increase) / 100).unwrap_or(u32::MAX)
    }

    /// Does the character hold at least `n` of `item_id` in `inv`, across every stack?
    pub(super) fn has_items(&self, chr_id: u32, inv: store::InventoryType, item_id: u32, n: u32) -> bool {
        self.store
            .bag_items(chr_id, inv)
            .unwrap_or_default()
            .iter()
            .filter(|r| r.item.item_id == item_id)
            .map(|r| u32::from(r.item.kind.quantity()))
            .sum::<u32>()
            >= n
    }

    /// Take `n` of `item_id` from `inv`, lowest slot first, telling the client per stack -
    /// the same drain `spend_attack_arrows` does for arrows. Check [`Self::has_items`] first;
    /// a shortfall here takes what there is and logs it rather than refusing halfway.
    pub(super) fn take_items(&mut self, chr_id: u32, inv: store::InventoryType, item_id: u32, n: u32) -> Vec<Reply> {
        let Ok(stacks) = self.store.bag_items(chr_id, inv) else { return Vec::new() };
        let mut remaining = n;
        let mut out = Vec::new();
        for row in stacks.iter().filter(|r| r.item.item_id == item_id) {
            if remaining == 0 {
                break;
            }
            let held = u32::from(row.item.kind.quantity());
            if held == 0 {
                continue;
            }
            let take = remaining.min(held);
            if let Err(e) = self.store.remove_item(chr_id, inv, row.slot, Some(u16::try_from(take).unwrap_or(u16::MAX))) {
                crate::server::log(&format!("   itemCon: could not take {take} x {item_id} from slot {}: {e}", row.slot));
                break;
            }
            out.extend(self.stack_change_replies(inv, row.slot, u16::try_from(held - take).unwrap_or(0)));
            remaining -= take;
        }
        crate::server::log(&format!(
            "   itemCon: {} of {n} x {item_id} taken{}",
            n - remaining,
            if remaining > 0 { format!(" - SHORT by {remaining}") } else { String::new() }
        ));
        out
    }

    /// **Heal** (`2301001`): HP now, to the caster and to the party on this field.
    ///
    /// The amount is `x`% of the ceiling the client draws - *"Recovery rate 40%"* at level 1,
    /// 100% at 30 - plus Bless's `x` while Bless is held (*"HP recovery from the Heal skill is
    /// increased by 1%"*). **[I]** for reading the rate as a percent of max HP: the client's
    /// own Heal formula is not decoded here, and this is the plainest reading of the tooltip's
    /// number. Undead damage is not done - that half needs the target list the attack packet
    /// carries, and no Heal has been captured on either opcode.
    pub(super) fn heal_cast(&mut self, level: u32) -> Vec<Reply> {
        let Some(row) = self.config.firstjob.level(crate::advbuffs::HEAL, level).copied() else {
            return Vec::new();
        };
        let Some(x) = row.x.and_then(|x| u32::try_from(x).ok()) else { return Vec::new() };
        let percent = x.saturating_add(self.bless_heal_bonus());
        let out = self.heal_percent(percent, "Heal");
        let Some(chr) = self.claimed_character() else { return out };
        let members: Vec<u32> = self
            .fields
            .parties()
            .party_of(chr.id)
            .map(|p| p.members.clone())
            .unwrap_or_default();
        if members.is_empty() {
            return out;
        }
        let Some(map) = self.bus().map_of(self.subscriber) else { return out };
        let here = self.bus().characters_on(map, &members);
        for member in here.into_iter().filter(|m| *m != chr.id) {
            self.bus().send_to_character(
                member,
                crate::broadcast::Event::PartyHeal { percent, caster: chr.id },
            );
        }
        out
    }

    /// Bless's heal bonus while Bless is held: its row's `x`, 1..10 percent. `0` otherwise.
    fn bless_heal_bonus(&self) -> u32 {
        self.buffs
            .iter()
            .find(|b| b.skill_id == crate::advbuffs::BLESS)
            .and_then(|_| {
                let chr = self.claimed_character()?;
                let level = self.store.skill_level(chr.id, crate::advbuffs::BLESS).ok()?;
                self.config.firstjob.level(crate::advbuffs::BLESS, level)?.x
            })
            .and_then(|x| u32::try_from(x).ok())
            .unwrap_or(0)
    }

    /// Restore `percent` of the drawn ceiling, through `combat::heal_flat`. Nothing for the
    /// dead - a heal is not a revive - and nothing when already full.
    pub(super) fn heal_percent(&mut self, percent: u32, why: &str) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let cap = self.pools(&chr).max_hp;
        let add = u32::try_from(u64::from(cap) * u64::from(percent) / 100).unwrap_or(u32::MAX);
        self.heal_flat(add, &format!("{why} ({percent}% of {cap})"))
    }

    /// **Dragon Blood's drain**: `x` HP every `y` seconds while the toggle is held, floored at
    /// 1 - the toggle's own price never kills its holder. `[L]` for the numbers (*"HP -40 every
    /// 3 sec"*); the floor is the same rule as Slash Blast's `hpCon`.
    pub(super) fn dragon_blood_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        if !self.holds(net::jobbuffs::CTS_DRAGON_BLOOD) || now_ms < self.dragon_blood_next_ms {
            return Vec::new();
        }
        let Some((skill, level)) = self.held_skill_level(net::jobbuffs::CTS_DRAGON_BLOOD) else {
            return Vec::new();
        };
        let Some(row) = self.config.firstjob.level(skill, level).copied() else { return Vec::new() };
        let every = row.y.and_then(|y| u64::try_from(y).ok()).filter(|y| *y > 0).unwrap_or(3);
        self.dragon_blood_next_ms = now_ms.saturating_add(every * 1000);
        let Some(drain) = row.x.and_then(|x| u32::try_from(x).ok()) else { return Vec::new() };
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if chr.hp <= 1 {
            return Vec::new();
        }
        chr.hp = chr.hp.saturating_sub(drain).max(1);
        if self.store.save_character_progress(&chr).is_err() {
            return Vec::new();
        }
        vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { hp: Some(chr.hp), ..Default::default() }.build(),
            what: format!("StatChanged: Dragon Blood drains {drain} hp every {every}s -> {}/{}", chr.hp, chr.max_hp),
        }]
    }

    /// **Combo Attack gains an orb** on a swing that hit something, up to the row's `y` orbs.
    ///
    /// The value on the wire is orbs **plus one** - it starts at 1 on the cast and the client
    /// draws `value - 1` orbs - which is the convention every reference server uses for this
    /// bit and is **[I]** here until a screen shows the first orb. `y` is *"Max combo count
    /// 3"* at level 1, 5 at 30.
    pub(super) fn combo_hit(&mut self) -> Vec<Reply> {
        let Some(held) = self.buffs.iter().find(|b| b.bit == net::jobbuffs::CTS_COMBO).copied() else {
            return Vec::new();
        };
        let Some((skill, level)) = self.held_skill_level(net::jobbuffs::CTS_COMBO) else {
            return Vec::new();
        };
        let max_orbs = self.config.firstjob.level(skill, level).and_then(|r| r.y).unwrap_or(0);
        let ceiling = i16::try_from(max_orbs + 1).unwrap_or(i16::MAX);
        if held.value >= ceiling {
            return Vec::new();
        }
        self.set_held_value(net::jobbuffs::CTS_COMBO, held.value + 1)
    }

    /// Coma and Panic spend every orb: back to one.
    pub(super) fn combo_spend(&mut self) -> Vec<Reply> {
        if self.held_value(net::jobbuffs::CTS_COMBO) <= 1 {
            return Vec::new();
        }
        self.set_held_value(net::jobbuffs::CTS_COMBO, 1)
    }

    /// Change a held stat's value and tell the client with a fresh `0x007D` carrying the time
    /// it has left (`0` for a toggle, which is what the original cast sent).
    fn set_held_value(&mut self, bit: u32, value: i16) -> Vec<Reply> {
        let now = self.clock_ms;
        let Some(held) = self.buffs.iter_mut().find(|b| b.bit == bit) else { return Vec::new() };
        held.value = value;
        let stat = net::buff::TemporaryStat {
            bit,
            value,
            reason: held.reason,
            duration_ms: if held.expires_ms == u64::MAX {
                0
            } else {
                u32::try_from(held.expires_ms.saturating_sub(now)).unwrap_or(u32::MAX)
            },
        };
        vec![Reply {
            opcode: net::buff::TEMPORARY_STAT_SET,
            body: net::buff::temporary_stat_set_with_tail(&[stat], net::buff::TAIL_LEN),
            what: format!("TemporaryStatSet: CTS bit {bit} now {value} (skill {})", held.skill_id),
        }]
    }

    /// One `0x007E`, with the length and the reason written into the log line.
    pub(super) fn reset_reply(&self, bits: &[u32], tail: usize, why: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::buff::TEMPORARY_STAT_RESET,
            body: net::buff::temporary_stat_reset_with_tail(bits, tail),
            what: format!(
                "TemporaryStatReset: CTS bit(s) {bits:?} {why}. {}-byte body ({tail}-byte tail); \
                 the 127-byte version threw at 0x142d57322 on 2026-08-22 and the enumerated \
                 reads want 129, or 133 if the gated u32 fires",
                3 + net::buff::MASK_LEN + tail
            ),
        }]
    }

}
