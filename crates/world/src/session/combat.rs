//! Hitting things, and the mobs that get hit.
//!
//! The client computes its own damage - the server never sends a number, only the
//! consequence. And a mob move report must be answered or the client runs one simulation
//! step and stops.

use super::*;

impl Session {

    /// The client reporting where a mob it controls has moved to. **This must be answered.**
    ///
    /// # A retraction, and the reason it happened is worth more than the fix
    ///
    /// This handler returned nothing for one run, on the strength of
    /// `research/mob-behaviour.md` §6: a scan for `mob+0x2f4` found 20 sites and *"not one of
    /// them is inside any of the eight mob-pool packet handlers"*. Re-running that scan gives
    /// the **identical 20 sites** - it was never wrong. What was wrong is the set it was
    /// intersected against: there are **110** mob-pool handlers, not eight, and §2.2 of that
    /// same document had already said so. One of the 20 is `141c821f8` inside
    /// `FUN_141c82060`, whose only caller is the stub for **`0x03E4`**.
    ///
    /// Enumerate before you filter, failed twice inside one file. On screen it looked like
    /// mobs moving for half a second and then freezing forever.
    ///
    /// # What the answer does
    ///
    /// `0x03E4` **MobCtrlAck** de-obfuscates the client's own move counter from
    /// `mob+0x2f0`/`+0x2f4` - the pair the sender incremented - compares it against the
    /// `move_id` we echo (`141c82212 CMP EAX,ECX / JNS`, so `ack >= current` passes), and
    /// re-runs slot 8 `FUN_141c54200`. That slot no-ops when the animation is already running
    /// (`141c54248 JNE ret`), which is what makes it a pump rather than an initialiser.
    ///
    /// # The broadcast half, WIRED 2026-09-01
    ///
    /// `0x03D9` is the *rebroadcast to every other client on the field* - the owner's point that a
    /// second player must see the same movement. `net::mobmove::mob_move_broadcast` had been
    /// built and tested against two real captured `0x02FF` bodies and had **no production
    /// caller** since the day it was written. It has one now.
    ///
    /// **It is never sent to the mover, and that is by construction rather than by a second
    /// check.** `research/mob-behaviour.md` §12.1: `FUN_141c813b0` overwrites the mob's
    /// position, animation and `mob+0xcd0` from the packet - state the controlling client
    /// owns. `Bus::publish` excludes the sender, and the sender **is** the controller,
    /// because the gate below runs first and refuses everyone else.
    ///
    /// # The gate, and why a refusal is silent
    ///
    /// Exactly one connection controls each mob (`crate::mobshare`). A report from any other
    /// must not move the mob and must not be acknowledged - acknowledging it would pump a
    /// second simulation, which is the divergence the registry exists to remove. `CLAUDE.md`'s
    /// always-answer rule is about a request that latches the UI; `0x02FF` is volunteered,
    /// and this handler has always returned nothing for a body that does not parse.
    pub(super) fn on_mob_move(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(req) = net::mobmove::parse_mob_move(payload) else {
            return Vec::new();
        };
        let Some(map) = self.claimed_character().map(|c| self.field_of(&c)) else { return Vec::new() };
        // Remember where it says the mob is. This is the only source of a live mob position
        // - the client runs the movement and we only acknowledge it - and it is what lets a
        // drop fall where the mob died instead of at the player's feet.
        //
        // `x`/`y` are the path's START, so this is at most one report stale - about half a
        // second, which is a few pixels for a snail. The path's END would be exact and needs
        // the element walk; this is the cheap 95% and it is the difference between a drop at
        // the mob and a drop across the platform.
        //
        // **`note_position_from` is the guard, not this call site.** It asks the registry and
        // refuses a writer that does not control the mob; the answer is used here rather than
        // logged and ignored, which is the mistake `CLAUDE.md` records under "a guard whose
        // answer is ignored is not a guard".
        // **`(req.x, req.y)` is the path's HEAD - where the mob was when the walk BEGAN, not
        // where it is now.** The comment here used to grant that and call the error "a few
        // pixels for a snail"; that half was never measured and is wrong. Over 642 431
        // deduplicated `0x02FF` events the head lags the mob by a median of 41 px and by more
        // than 25 px - half the client's own pick-up box - 62.8% of the time, which is the owner's
        // *"dropping from an awkward location not related to the current mob location"*.
        // `crate::dropsite::reported_position` takes the path's END instead. Measured with its
        // controls in that module's docs and re-derivable with `tools/mobmove_lag.py`.
        if !self.fields.note_position_from(
            map,
            req.object_id,
            crate::dropsite::reported_position(&req),
            crate::dropsite::reported_foothold(&req),
            self.subscriber.get(),
        ) {
            crate::server::log(&format!(
                "   0x02FF for mob {} IGNORED: this connection ({}) does not control it \
                 (controller {:?}). Two controllers would be two independent wanders - the \
                 client rolls the path itself, research/mob-behaviour.md 5.1",
                req.object_id,
                self.subscriber.get(),
                self.fields.controllers().controller_of(map, req.object_id),
            ));
            return Vec::new();
        }
        // **And now the other screens.** One controller means exactly one client is
        // simulating; without this the mob walks on the controller's screen and stands still
        // on everyone else's, which is the same divergence in a quieter form.
        let audience = crate::mobshare::audience_for(net::mobmove::MOB_MOVE, req.object_id);
        if audience.is_map_wide() {
            self.bus().publish(
                self.subscriber,
                map,
                Reply {
                    opcode: net::mobmove::MOB_MOVE,
                    body: net::mobmove::mob_move_broadcast(&req),
                    what: format!(
                        "MobMove: mob {} to ({}, {}), {} path bytes copied verbatim. Superseded \
                         per mob - a mob wanders for as long as it is alive, so an observer \
                         who stops reading would otherwise accumulate these without bound.",
                        req.object_id,
                        req.x,
                        req.y,
                        req.path.len()
                    ),
                },
                audience.supersedes(),
            );
        }
        // King Slime's MP and summon, and whatever a reported summon spawned.
        // session/mobskill.rs, crate::mobskills.
        let (mp, skill, level, summoned) = self.mob_skill_ack(map, &req);
        let mut out = vec![Reply {
            opcode: net::mobmove::MOB_CTRL_ACK,
            // **The one bool that lets a mob attack at all.**
            //
            // `research/mob-attack-skills.md`: this was a hard-coded `false` from the day the
            // packet was written. Body offset 6 is the only server-driven way to put a mob
            // into controller state **4**, and `CMob::Update` skips its entire attack- and
            // skill-selection block - 1594 bytes, containing the only `GetAttack` and
            // `GetSkill` calls in the function - unless the state is exactly that. **[L]**
            //
            // Three independent measurements agreed that it never happened: 131 003 distinct
            // mob-move reports with zero in the attack or skill range, 257 user-hits all
            // `attackIndex -1`, and `0x0313` never once arriving. Not a sample-size problem -
            // a branch never taken.
            //
            // **Unconditional, deliberately.** The client owns every other precondition - the
            // attack count, the per-attack cooldown, range, target - and the state survives
            // only until the next move report, so a grant buys **one action**, not a mode.
            body: net::mobmove::mob_ctrl_ack_with(
                req.object_id,
                req.move_id,
                crate::mobattack::grant_attack(),
                mp,
                skill,
                level,
            ),
            what: format!(
                "MobCtrlAck: mob {} move {} acknowledged (mp {mp}, skill {skill} level {level}). \
                 Without this the client runs one simulation step and stops - measured twice, 30 \
                 grants and 30 reports all with moveId 1, then silence.",
                req.object_id, req.move_id
            ),
        }];
        out.extend(summoned);
        out
    }


    /// Spend the MP an attack skill costs, and tell the client the new total.
    ///
    /// # Why this did not exist until now
    ///
    /// The cost is `mpCon`, keyed by **(skill id, skill level)**, and until 2026-08-28 this
    /// server could not read either off the wire. `research/attack-skill-id.md` found both -
    /// the id is the `u32` at body offset 2 and the level the `u8` at offset 6 - so this is
    /// the first thing that finding paid for.
    ///
    /// # The level comes from the STORE, not from the packet
    ///
    /// The client tells us which level it thinks it cast, and nothing on this socket
    /// authenticates anybody. A crafted body claiming level 1 would buy a level-20 cast at
    /// the level-1 price. The packet's level is only used when the store has no row, which
    /// means the skill was never granted through us.
    ///
    /// # It never refuses
    ///
    /// The swing has already happened on screen. Refusing here cannot un-play the animation,
    /// and would recreate the very desynchronisation this exists to fix - so an overdraw
    /// spends what is there, floors at zero, and says so in the log line.
    ///
    /// **And the HP, for the one skill that has an `hpCon`.** Slash Blast costs 3..8 HP a
    /// swing on top of its MP (`firstjob.rs`: *"Slash Blast is the only one of the 24 with an
    /// `hpCon`"*). `Obligation::deduct_hp` had said so since 2026-08-28 and nothing read it -
    /// the same "built, not wired" shape as the arrows, found by the Warrior audit on
    /// 2026-09-06. The HP is **floored at 1, not 0**: a skill's own cost must never be the
    /// thing that kills its caster, and the client does not let a swing go out at 1 HP for
    /// a 3-HP skill in the first place, so a floor is a repair of a desync and not a rule
    /// the player can lean on.
    fn spend_attack_costs(&mut self, opcode: u16, payload: &[u8]) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        // The full parser rather than a hand-rolled offset read: it checks the length and the
        // trailer, so a body it accepts is one whose head we have actually understood.
        let Ok(parsed) = net::attack::parse(opcode, payload) else { return Vec::new() };
        let Some((skill_id, claimed_level)) = parsed.skill() else {
            return Vec::new(); // an ordinary swing costs nothing
        };
        let level = self
            .store
            .skill_level(chr.id, skill_id)
            .ok()
            .filter(|l| *l > 0)
            .unwrap_or(u32::from(claimed_level));
        let Some(row) = self.config.firstjob.level(skill_id, level) else {
            return Vec::new(); // a skill this table does not describe
        };
        // Element Amplification raises every MP cost while it is held - `buff::amplified_mp`.
        let mp_cost = self.amplified_mp(skill_id, row.mp_con.unwrap_or(0));
        let hp_cost = row.hp_con.unwrap_or(0);
        // `moneyCon`: Shadow Meso throws mesos, 200..500 a cast. **[L]**
        let meso_cost = row.money_con.unwrap_or(0);
        if mp_cost == 0 && hp_cost == 0 && meso_cost == 0 {
            return Vec::new(); // no cost column at all
        }
        let mp_short = mp_cost.saturating_sub(chr.mp);
        chr.mp = chr.mp.saturating_sub(mp_cost);
        let hp_before = chr.hp;
        if hp_cost > 0 {
            chr.hp = chr.hp.saturating_sub(hp_cost).max(1);
        }
        let hp_short = hp_cost.saturating_sub(hp_before.saturating_sub(1));
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not spend the MP for skill {skill_id}: {e}"));
        }
        // The mesos, never refused: the throw has already left the hand on screen. A balance
        // short of the cost pays what it has (the store refuses to go below zero, so this
        // takes the whole balance in that case) and says so.
        let meso_after = if meso_cost > 0 {
            let have = self.store.mesos(chr.id).unwrap_or(0);
            let pay = meso_cost.min(have);
            match self.store.add_mesos(chr.id, -i64::from(pay)) {
                Ok(left) => {
                    crate::server::log(&format!(
                        "   moneyCon: skill {skill_id} threw {pay} of {meso_cost} mesos -> {left}{}",
                        if pay < meso_cost { format!(" (SHORT by {}, not refused)", meso_cost - pay) } else { String::new() }
                    ));
                    Some(left)
                }
                Err(e) => {
                    crate::server::log(&format!("   moneyCon: could not take {pay} mesos for skill {skill_id}: {e}"));
                    None
                }
            }
        } else {
            None
        };
        vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                mp: Some(chr.mp),
                hp: (hp_cost > 0).then_some(chr.hp),
                meso: meso_after.map(u64::from),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: skill {skill_id} level {level} cost {mp_cost} mp -> {}/{}{}{}. The \
                 CLIENT already spent this locally; before 2026-08-28 the server did not, and \
                 the stale total came back the next time any 0x007C carried the MP field - \
                 which is what looked like a Red Potion restoring MP",
                chr.mp,
                chr.max_mp,
                if mp_short > 0 {
                    format!(" (SHORT by {mp_short}, floored at 0 rather than refused)")
                } else {
                    String::new()
                },
                if hp_cost > 0 {
                    format!(
                        " and {hp_cost} hp -> {}/{} (hpCon; Slash Blast is the only first-job skill with one){}",
                        chr.hp,
                        chr.max_hp,
                        if hp_short > 0 {
                            format!(" (SHORT by {hp_short}, floored at 1 - a skill never kills its caster)")
                        } else {
                            String::new()
                        }
                    )
                } else {
                    String::new()
                }
            ),
        }]
    }

    /// **Tell the client what this mob's stats are**, so it can compute a contact hit.
    ///
    /// `research/mob-to-player-damage-packet.md`: there is **no inbound packet carrying a
    /// damage number** for the local player. The client computes contact damage itself and
    /// reports it in the outbound `0x00E5`. What the server owes is the mob's **attack
    /// power** - and until this existed we never sent one, so `[rdi+0xe8]`-style reads found
    /// nothing and the client's own formula floored at `1.0`. 198 captured hits, all `1`.
    ///
    /// **Built from the mob's own WZ row so exactly nothing else changes.** The block forces
    /// *every* stat at once, so sending zeros for the ones we do not mean to touch would make
    /// every mob defenceless and unable to miss - a far bigger change than the intended one.
    /// `Config::mob_templates` carries the real columns for that reason.
    ///
    /// `None` when the template is unknown, which writes the single `0` byte this packet has
    /// always written. A mob we have no data for behaves exactly as it did before.
    pub(super) fn forced_stat_for(&self, template: u32) -> Option<net::mobdamage::MobForcedStat> {
        let t = self.config.mob_templates.get(&template)?;
        Some(net::mobdamage::MobForcedStat::from_template(
            u64::from(t.max_hp),
            t.level,
            t.pa_damage,
            t.ma_damage,
            t.pd_damage,
            t.md_damage,
            t.accuracy,
            t.evasion,
        ))
    }

    /// The player swung at something.
    ///
    /// **The client has already worked out the damage.** Each target block in `0x00DF` carries
    /// the numbers it intends to show, so nothing is sent back to make them appear - what the
    /// server owes is the *consequence*: the health bar, and the death.
    ///
    /// **Drops are rolled here**, on the death branch - see [`Session::drops_from_kill`].
    ///
    /// Still missing: **no EXP for a kill.** `0x007C` bit 16 carries it and experience now
    /// persists (`!exp` proves the whole chain), but the amount per kill is the EXP curve,
    /// which lives in the BSS tail of `.data` and cannot be read statically. `STATUS.md`.
    ///
    /// **A miss is not an error.** An attack with no targets is exactly what the client sends
    /// when it swings at empty air, and on 2026-08-19 it was also - wrongly - reported as
    /// proof that the client would not target our mobs at all. That claim came from combining
    /// two different sessions and is retracted; `research/mob-combat.md` §17.
    /// **How many arrows this swing costs, and which stack they come from.**
    ///
    /// The owner, 2026-09-06: *"regular attacks or skills using bows/crossbows should consume
    /// arrows from the use tab depending on the attack amount. for example double shot should
    /// consume 2 arrows."* Until this, nothing did: `crate::firstjob` had carried
    /// [`crate::firstjob::BulletDuty`] since 2026-08-28 - Arrow Blow `Consume(1)`, Double Shot
    /// `Consume(2)`, Power Knockback none - and `on_attack` never read it. Built, not wired.
    ///
    /// # The rule
    ///
    /// * The character must be holding a **bow or a crossbow**; any other weapon costs no
    ///   arrow whatever the packet says. Found by item class among the worn items, because
    ///   only weapon ids classify (`WeaponClass::from_item_id`).
    /// * A **skill** costs its `bulletConsume` - the column the client's own data carries for
    ///   Arrow Blow (1) and Double Shot (2), read through `firstjob::server_obligation`. A
    ///   skill with **no** bullet column costs nothing: Power Knockback is the bow swung as a
    ///   club, and its row has no `bulletConsume` at any level. **[L]**
    /// * A skill the data does not settle (`bulletCount` with no `bulletConsume`, or a skill
    ///   outside the first-job book) costs **one on the shoot opcode and nothing on melee** -
    ///   the plain-shot rule, and it is logged as [I] every time it fires.
    /// * A **plain attack** costs one arrow on `0x00E0 USER_SHOOT_ATTACK` and nothing on the
    ///   melee opcode. The client sends a bow's normal attack as a shoot. **[L]** for the
    ///   opcode; the "one arrow" is the game's rule and is stated as such.
    /// * Bows take **`2060xxx` arrows**, crossbows **`2061xxx`** - `gm-handbook/items.txt`
    ///   names them *"Arrows for Bows"* / *"Arrows for Crossbows"*. The lowest Use-tab slot
    ///   holding a matching stack is drained first; a stack short of the cost gives what it
    ///   has and the remainder comes out of the next.
    ///
    /// # What the packet does NOT carry
    ///
    /// No field of the 33-field attack header is a bullet slot (`net::attack::AttackHeader`,
    /// every field read and most of them constant across 434 captures), so the server picks
    /// the stack rather than being told. That is a decision and it is written down as one.
    ///
    /// # It never refuses
    ///
    /// The arrow has already flown on the shooter's screen. No matching stack, or not enough,
    /// is a log line and a smaller deduction, never a rejected swing - the client will not
    /// fire without arrows in the first place, so a shortfall here is the two ends
    /// disagreeing about a count, which the `0x0070` sent for what *was* taken then repairs.
    ///
    /// **Throwing stars follow the same rule with a claw.** The owner, 2026-09-06: *"Thief
    /// skills/basic attack should consume stars similar to bowman with arrows."* A claw
    /// (`147xxxx`) draws from the `207xxxx` family - the same two-range test the client's own
    /// bundle decoder makes (`net::bag::BUNDLE_SERIAL_RANGES`), so Subi through Hwabi and
    /// the three event stars all count. A plain throw is one star; Lucky Seven is **two**,
    /// which is where the archer rule was tightened: it fires `bulletCount 2` with no
    /// `bulletConsume` column, and the old arm charged the plain-shot 1 for that shape.
    /// The owner's rule is "depending on the attack amount", so a skill that fires N and names no
    /// consume column is now charged N. That is **[I]** - no capture shows a Lucky Seven -
    /// and it changes nothing for the Archer (Double Shot has the column, Power Knockback
    /// fires nothing) or for wands, which never reach this far.
    ///
    /// **Which stack.** The attack header has no slot field (`net::attack::AttackHeader`,
    /// forty fields, none of them an inventory position), so the lowest matching stack goes
    /// first. A Rogue carrying two kinds of star will see the lower slot drain whichever kind
    /// the client's star icon shows; the `0x0070` keeps the counts honest either way.
    ///
    /// **One instrument caveat, stated because it decides whether any of this runs.** The
    /// parser was decoded from `0x00DF` melee bodies - **no `0x00E0` shoot body has ever been
    /// captured** (67 archived logs carry a melee, none a shot, checked 2026-09-06). All
    /// three attack opcodes share one encoder (`FUN_140f31fe0`, `net::attack` module docs),
    /// which is why the same parser is used; but a shot that failed to parse would take
    /// nothing and say nothing, so that case now logs loudly instead of returning quietly.
    fn spend_attack_arrows(&mut self, opcode: u16, payload: &[u8]) -> Vec<Reply> {
        use crate::damage::WeaponClass;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let parsed = match net::attack::parse(opcode, payload) {
            Ok(p) => p,
            Err(e) => {
                if opcode == net::combat::USER_SHOOT_ATTACK {
                    crate::server::log(&format!(
                        "   projectiles: a 0x00E0 SHOOT body did not parse ({e:?}, {} bytes) - \
                         no arrow or star was taken. The parser comes from melee captures; \
                         if this line appears, the shoot layout differs and needs a capture",
                        payload.len()
                    ));
                }
                return Vec::new();
            }
        };

        // The weapon in hand decides everything else.
        let held = self
            .store
            .equipped_items(chr.id)
            .unwrap_or_default()
            .iter()
            .find_map(|e| WeaponClass::from_item_id(e.item_id));
        let (arrow_range, ammo) = match held {
            Some(WeaponClass::Bow) => (2_060_000..=2_060_999, "arrows"),
            Some(WeaponClass::Crossbow) => (2_061_000..=2_061_999, "crossbow arrows"),
            // The whole 207 family, as the client's own bundle decoder ranges it.
            Some(WeaponClass::Claw) => (2_070_000..=2_079_999, "stars"),
            _ => return Vec::new(),
        };

        let shooting = opcode == net::combat::USER_SHOOT_ATTACK;
        // **Soul Arrow: nothing is taken while it is held.** The client stops counting its
        // own arrows down under CTS 104, so a server that kept taking them would drift by
        // one per shot until the next bag refresh. Plain shots and skills alike.
        if self.holds(net::jobbuffs::CTS_SOUL_ARROW) {
            crate::server::log(&format!("   {ammo}: Soul Arrow is held - none taken"));
            return Vec::new();
        }
        let (cost, why): (u32, String) = match parsed.skill() {
            None => (u32::from(shooting), "a plain shot".to_string()),
            Some((skill_id, claimed)) => match crate::firstjob::server_obligation(skill_id).map(|o| o.bullets) {
                Some(crate::firstjob::BulletDuty::Consume(n)) => {
                    (n, format!("skill {skill_id}: bulletConsume {n} [L]"))
                }
                Some(crate::firstjob::BulletDuty::None) => {
                    (0, format!("skill {skill_id}: no bullet column - the weapon swung, not fired [L]"))
                }
                // Lucky Seven: `bulletCount 2`, no consume column, charged 2 when thrown.
                Some(crate::firstjob::BulletDuty::ProjectilesNoConsumeColumn(fired)) => (
                    if shooting { fired } else { 0 },
                    format!(
                        "skill {skill_id}: fires {fired} and the data has no bulletConsume; \
                         charging one per projectile - the owner's attack-amount rule [I]"
                    ),
                ),
                // Outside the first-job book the row itself decides, in the order the three
                // columns override one another: `noBulletConsume` (the three hidden third-job
                // hits) beats everything; `bulletConsume` is the number the data states (Arrow
                // Rain 8..4, Avenger 4, Mortal Blow 1); `bulletCount` with no consume column is
                // one per projectile on the shoot opcode, the same reading Lucky Seven got from
                // The owner's attack-amount rule (Strafe 3..4, Iron Arrow 1); and no column at all
                // is the plain-shot rule. Second- and third-job audit, 2026-09-07.
                None => {
                    let level = self
                        .store
                        .skill_level(chr.id, skill_id)
                        .ok()
                        .filter(|l| *l > 0)
                        .unwrap_or(u32::from(claimed));
                    match self.config.firstjob.level(skill_id, level) {
                        Some(r) if r.no_bullet_consume => {
                            (0, format!("skill {skill_id}: noBulletConsume [L]"))
                        }
                        Some(r) if r.bullet_consume.is_some() => {
                            let n = r.bullet_consume.unwrap_or(0);
                            (n, format!("skill {skill_id}: bulletConsume {n} [L]"))
                        }
                        Some(r) if r.bullet_count.is_some() => {
                            let fired = r.bullet_count.unwrap_or(0);
                            (
                                if shooting { fired } else { 0 },
                                format!(
                                    "skill {skill_id}: fires {fired} and the data has no \
                                     bulletConsume; one per projectile [I]"
                                ),
                            )
                        }
                        Some(_) => (
                            u32::from(shooting),
                            format!("skill {skill_id}: no bullet column; plain-shot rule [I]"),
                        ),
                        None => (
                            u32::from(shooting),
                            format!("skill {skill_id}: not in the skill table; plain-shot rule [I]"),
                        ),
                    }
                }
            },
        };
        if cost == 0 {
            return Vec::new();
        }

        // Lowest matching stack first, draining across stacks if one is short.
        let Ok(stacks) = self.store.bag_items(chr.id, store::InventoryType::Use) else {
            return Vec::new();
        };
        let mut remaining = cost;
        let mut out = Vec::new();
        let mut taken_from = Vec::new();
        for row in stacks.iter().filter(|r| arrow_range.contains(&r.item.item_id)) {
            if remaining == 0 {
                break;
            }
            let held = u32::from(row.item.kind.quantity());
            if held == 0 {
                continue;
            }
            let take = remaining.min(held);
            let take_u16 = u16::try_from(take).unwrap_or(u16::MAX);
            if let Err(e) = self.store.remove_item(chr.id, store::InventoryType::Use, row.slot, Some(take_u16)) {
                crate::server::log(&format!(
                    "   arrows: could not take {take} from Use slot {} for {why}: {e}",
                    row.slot
                ));
                break;
            }
            let left = u16::try_from(held - take).unwrap_or(0);
            out.extend(self.stack_change_replies(store::InventoryType::Use, row.slot, left));
            taken_from.push(format!("slot {} ({} -> {left})", row.slot, held));
            remaining -= take;
        }
        crate::server::log(&format!(
            "   {ammo}: {} of {cost} taken for {why} - {}{}",
            cost - remaining,
            if taken_from.is_empty() { format!("NO matching {ammo} stack in the Use tab") } else { taken_from.join(", ") },
            if remaining > 0 && !taken_from.is_empty() { format!("; SHORT by {remaining}, swing not refused") } else { String::new() }
        ));
        out
    }

    /// `itemCon` / `itemConNo`: the item a cast throws. **Three Snails** is the case that
    /// exists in this client - `Skill.wz` level 1 throws a Snail Shell (4000001), level 2 a
    /// Blue Snail Shell (4000002), level 3 a Red Snail Shell (4000004), one a cast. **[L]**
    ///
    /// The owner, 2026-09-13: *"Three Snails is a skill that takes 1 Red Snail Shell to cast. If
    /// the user does not have red snail shells in their inventory, the skill should output a
    /// red error text in chat saying you do not have enough Red Snail Shell to cast this
    /// skill. Casting it should decrease the client's Red Snail Shell inventory count by 1."*
    ///
    /// Unlike the MP and the arrows, **this one refuses**: `Err` carries the red line and the
    /// caller returns it alone - no MP spent, no damage applied, no broadcast. The client
    /// let the swing out (its own `itemCon` check does not fire in this build, or the owner would
    /// never have seen it cast), so the server is the only thing that can say no. The line is
    /// the client's own system category (11, `0xFFFFAFAF` - the colour of every "You cannot"
    /// it prints itself), through `net::message::chat_line_system`.
    ///
    /// `Ok` carries the stack updates: the lowest matching stack first, `0x0070` per stack
    /// touched, the same shape `spend_attack_arrows` uses.
    fn spend_attack_item(&mut self, opcode: u16, payload: &[u8]) -> Result<Vec<Reply>, Vec<Reply>> {
        let Some(chr) = self.claimed_character() else { return Ok(Vec::new()) };
        let Ok(parsed) = net::attack::parse(opcode, payload) else { return Ok(Vec::new()) };
        let Some((skill_id, claimed_level)) = parsed.skill() else { return Ok(Vec::new()) };
        let level = self
            .store
            .skill_level(chr.id, skill_id)
            .ok()
            .filter(|l| *l > 0)
            .unwrap_or(u32::from(claimed_level));
        let Some(row) = self.config.firstjob.level(skill_id, level) else { return Ok(Vec::new()) };
        let Some(item_id) = row.item_con.filter(|id| *id != 0) else { return Ok(Vec::new()) };
        let need = row.item_con_no.unwrap_or(1).max(1);
        // The tab is the id's first digit: 2 is Use, 4 is Etc - the shells' tab.
        let inv = match item_id / 1_000_000 {
            2 => store::InventoryType::Use,
            _ => store::InventoryType::Etc,
        };
        let stacks = self.store.bag_items(chr.id, inv).unwrap_or_default();
        let held: u32 = stacks
            .iter()
            .filter(|r| r.item.item_id == item_id)
            .map(|r| u32::from(r.item.kind.quantity()))
            .sum();
        let name = self
            .config
            .item_names
            .get(&item_id)
            .cloned()
            .unwrap_or_else(|| format!("item {item_id}"));
        if held < need {
            let text = format!("You do not have enough {name} to cast this skill.");
            crate::server::log(&format!(
                "   itemCon: skill {skill_id} level {level} needs {need} x {item_id} ({name}); the {inv:?} tab holds {held} - REFUSED, nothing spent, no damage"
            ));
            return Err(vec![Reply {
                opcode: net::message::MESSAGE,
                body: net::message::chat_line_system(&text),
                what: format!("Message chat line (system, category 11): {text:?} - skill {skill_id} refused for want of {need} x {item_id}"),
            }]);
        }
        let mut remaining = need;
        let mut out = Vec::new();
        let mut taken_from = Vec::new();
        for r in stacks.iter().filter(|r| r.item.item_id == item_id) {
            if remaining == 0 {
                break;
            }
            let have = u32::from(r.item.kind.quantity());
            if have == 0 {
                continue;
            }
            let take = remaining.min(have);
            if let Err(e) = self.store.remove_item(chr.id, inv, r.slot, Some(u16::try_from(take).unwrap_or(u16::MAX))) {
                crate::server::log(&format!("   itemCon: could not take {take} x {item_id} from {inv:?} slot {}: {e}", r.slot));
                break;
            }
            let left = u16::try_from(have - take).unwrap_or(0);
            out.extend(self.stack_change_replies(inv, r.slot, left));
            taken_from.push(format!("slot {} ({have} -> {left})", r.slot));
            remaining -= take;
        }
        crate::server::log(&format!(
            "   itemCon: skill {skill_id} level {level} threw {} x {item_id} ({name}) - {}",
            need - remaining,
            taken_from.join(", ")
        ));
        Ok(out)
    }

    pub(super) fn on_attack(&mut self, opcode: u16, payload: &[u8]) -> Vec<Reply> {
        let Ok(attack) = net::combat::parse_attack(payload) else {
            return Vec::new();
        };
        // **The item the cast throws, before anything else - and the one cost that refuses.**
        // Three Snails without a shell: the red line, and nothing else this swing would do.
        let thrown = match self.spend_attack_item(opcode, payload) {
            Ok(replies) => replies,
            Err(refusal) => return refusal,
        };
        // **The MP the skill cost, before anything else this swing does.**
        //
        // The owner, 2026-08-28: *"Using the Red Potion when my MP is depleted incorrectly
        // recovered my MP?"* It did not. The potion's own `spec` is `hp 100, mp 0` and the
        // handler added `+0 mp` - the log line says so. What happened is that the **client**
        // had been spending MP locally on every Power Strike while the **server** never did,
        // so `chr.mp` here was still 181/181. The potion's `0x007C` carries the MP field
        // like every other stat change, and the client believed it.
        //
        // So the bug was never in the potion; it was a stale number finally being spoken
        // aloud. Any `0x007C` would have done it - idle regen would have done it a few
        // seconds later.
        //
        // This is the fix, and it is the first thing the skill id unblocked
        // (`research/attack-skill-id.md`): the id is the `u32` at body offset 2 and the
        // level the `u8` at offset 6, so the cost is a lookup away.
        //
        // **Log only, never refuse.** The client has already played the animation and
        // computed its damage; rejecting the swing here would desynchronise the very thing
        // this is fixing. If the MP does not cover it we spend what there is and say so.
        let mut out = thrown;
        out.extend(self.spend_attack_costs(opcode, payload));
        // **And the arrows or stars it cost.** Same rule as the MP: the shot has already
        // left the weapon on screen, so the server takes what it owes and never refuses.
        out.extend(self.spend_attack_arrows(opcode, payload));
        // **The only coordinate pair this server reads from the client.** The attack body
        // carries the player's own position (fields 13/14), which is how the zero-target
        // captures were paired against mob positions in `research/mob-target-gates.md` §1.
        // Nothing parses `0x00D9`, so until something does, this is where a drop learns
        // where to land - see `Session::last_position`. Recording it here rather than in the
        // drop path means it survives the swing that produced it.
        // The attack packet carries no stance, so the last one reported stands. `None`
        // means "unchanged" here rather than "unknown".
        self.note_own_position(attack.x as i16, attack.y as i16, None);
        self.note_activity();
        // Read once rather than per target: it is a database round trip, and a swing can
        // legitimately kill several mobs at once.
        let killer = self.claimed_character().map(|c| (c.id, self.field_of(&c)));
        let Some((chr_id, map)) = killer else { return out };
        // **The attack half of the owner's 2026-08-29 sentence**, and the second production
        // caller of `Bus::publish` there has ever been. Before the target loop, so a swing
        // that hit nothing still crosses: 43 of the 434 captured bodies have no target at
        // all, and a miss that is invisible to everyone else reads as a frozen character.
        //
        // Additive by construction - it appends to other connections' mailboxes and returns
        // `()`. Nothing about `out` changes, which is what
        // `the_broadcast_does_not_change_what_the_attacker_gets` pins.
        self.publish_user_attack(opcode, payload);
        let me = self.subscriber.get();
        // The templates this swing actually damaged, for the per-swing effects below.
        let mut landed: Vec<u32> = Vec::new();
        for target in &attack.targets {
            if self.fields.mob_hp(map, target.object_id).is_none() {
                continue; // not a mob of ours, or already dead and removed
            }
            let template = self.fields.mob_template(map, target.object_id).unwrap_or(0);

            // **Whoever hits it, drives it - and the old holder is TOLD, which is what was
            // missing the first time.**
            //
            // The flinch and the knockback are local to whoever holds the `0x03D2`; nothing
            // the server sends produces them (`research/mob-hit-reaction.md`). So the
            // attacker has to own what it hits.
            //
            // That shipped once without the release and made mobs teleport - two clients
            // simulating one mob - and was reverted because `CONTROL_RELEASE` was documented
            // as a despawn. **It is not.** The owner said so and the listing agrees: the zero
            // branch releases and a live mob never reaches the erase behind it. See
            // `net::mobmove::CONTROL_RELEASE`.
            //
            // **Release first, then grant.** Granting first leaves both clients past their
            // run gate, both rolling independent wanders, both sending `0x02FF` - which is
            // exactly the teleporting. Order is the fix, not an optimisation.
            if let Some(previous) =
                self.fields.controllers().hand_over_one(map, target.object_id, me)
            {
                if let Some(loser) = previous.and_then(|s| self.bus().subscriber_of(s)) {
                    self.bus().publish_to_subscriber(
                        loser,
                        Reply {
                            opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                            body: net::mobmove::mob_release_controller(target.object_id),
                            what: format!(
                                "MobChangeController RELEASE: object id {} - somebody else is \
                                 hitting it and needs to drive it. This does NOT despawn a \
                                 live mob; it stops this client simulating it, and the 0x03D9 \
                                 relay keeps it moving on their screen",
                                target.object_id
                            ),
                        },
                    );
                }
                if let Some(mob) =
                    self.fields.mobs_on(map).iter().find(|m| m.spawn.object_id == target.object_id)
                {
                    // `as_seen`, not `spawn`, or the mob jumps back to its spawn point.
                    out.push(Reply {
                        opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                        body: net::mobmove::mob_change_controller(
                            &mob.as_seen(),
                            net::mobmove::CONTROL_NORMAL,
                        ),
                        what: format!(
                            "MobChangeController: object id {} to the attacker, so its own \
                             client can play the hit reaction",
                            target.object_id
                        ),
                    });
                }
            }

            let damage = target.total_damage();
            if damage > 0 {
                landed.push(template);
            }
            out.extend(self.deal_to_mob(map, target.object_id, damage, chr_id));
        }

        // ---------------------------------------------------------------------------------
        // What hangs off this swing beyond the mob's HP. Second- and third-job audit,
        // 2026-09-07. Every one of these reads a number the server owns (the character's HP,
        // MP, a held stat's value) and none refuses the swing.
        // ---------------------------------------------------------------------------------
        let skill = net::attack::parse(opcode, payload).ok().and_then(|p| p.skill());
        let mut finisher = false;
        if let Some((skill_id, claimed)) = skill {
            let level = self
                .store
                .skill_level(chr_id, skill_id)
                .ok()
                .filter(|l| *l > 0)
                .unwrap_or(u32::from(claimed));
            // **Heal, arriving as a magic attack.** The client may send it on `0x00E1` when
            // undead are in range and on `0x013C` otherwise - no Heal has been captured on
            // either - so both paths heal. Undead damage is the client's per-target lines,
            // applied above like any other hit.
            if skill_id == crate::advbuffs::HEAL {
                out.extend(self.heal_cast(level));
            }
            // **Drain**: `prop`% chance to absorb `x`% of the damage dealt as HP. **[L]** for
            // the columns (*"2% chance to absorb 5% of damage as HP"*).
            if skill_id == crate::advbuffs::DRAIN {
                let total: u64 = attack.targets.iter().map(|t| t.total_damage()).sum();
                if let Some(row) = self.config.firstjob.level(skill_id, level).copied() {
                    let prop = u64::from(row.prop.unwrap_or(0));
                    let x = row.x.and_then(|x| u64::try_from(x).ok()).unwrap_or(0);
                    if total > 0 && prop > 0 && self.rng.next() % 100 < prop {
                        let heal = u32::try_from(total * x / 100).unwrap_or(u32::MAX);
                        out.extend(self.heal_flat(heal, "Drain"));
                    }
                }
            }
            // Coma and Panic spend the Combo orbs rather than adding one.
            if crate::advbuffs::COMBO_FINISHERS.contains(&skill_id) {
                finisher = true;
                out.extend(self.combo_spend());
            }
        }
        if !landed.is_empty() {
            if !finisher {
                out.extend(self.combo_hit());
            }
            if opcode == net::combat::USER_MAGIC_ATTACK {
                out.extend(self.mp_eater(&landed));
            }
        }
        out
    }

    /// **Apply `damage` to one mob and do everything a hit owes**: the HP bar and the death
    /// to every viewer, and on a death the drops, the experience, the quest credit and the
    /// controller registry. `on_attack` calls it per target; Power Guard's reflection calls it
    /// once from `on_user_hit`. One body so the two cannot drift - the `0x03F0` percentage bug
    /// (`research/mob-hp-bar.md`) is exactly what a second copy of this would grow.
    ///
    /// The **controller handover stays in `on_attack`**: a reflection is not a swing, and a
    /// mob should not change hands because it walked into someone.
    pub(super) fn deal_to_mob(&mut self, map: crate::fields::FieldKey, object_id: u32, damage: u64, chr_id: u32) -> Vec<Reply> {
        let mut out = Vec::new();
        let Some(hp_before) = self.fields.mob_hp(map, object_id) else {
            return out; // not a mob of ours, or already dead and removed
        };
        let template = self.fields.mob_template(map, object_id).unwrap_or(0);
        let hit = net::combat::apply_damage(hp_before, damage);
        // The field owns the death: it removes the mob and books its spawn point to
        // refill from the WZ's own mobTime. Nothing here has to remember a corpse.
        // **Where it is, BEFORE it dies.** `hurt` removes the mob from the field, so
        // asking afterwards returns nothing and every drop fell back to the player's
        // feet - which is exactly what the owner saw twice. Read it first, hand it down.
        // `mob_site`, not `mob_position`: a mob that has never reported a move still has the
        // exact `Map.wz` spawn pixel on `LiveMob`, and falling through to `None` here put its
        // drops at the PLAYER'S FEET, which can be a field away. `mob_position` stays the raw
        // report because two controller-guard tests prove that guard by asserting it returns
        // `None`, and a fallback inside it would make them pass whatever the guard did.
        let died_at = self.fields.mob_site(map, object_id);
        let left = self.fields.hurt(map, object_id, damage, chr_id, &self.config, self.clock_ms);
        if let crate::fields::Hurt::Died(shares) = left {
            // **The drops go to the top damager, not to whoever landed the last hit.**
            // The owner, 2026-09-01: *"If multiple clients hit the mob, the one who dealt the
            // most damage (without counting over-damage) will see the drops."*
            // `LiveMob::credit` already caps at what landed and `shares()` already ranks;
            // `drop_audience` adds no arithmetic to either, and `chr_id` terminates the
            // walk because the killer is on this map by definition, having just swung.
            let ranked = crate::mobshare::drop_audience(&shares, chr_id);
            // The King Slime's per-member shoes ride in the same row. session/firsttime.rs.
            let personal = self.party_quest_personal_drops(map, template);
            out.extend(self.drops_from_kill_for(
                template,
                object_id,
                died_at,
                &ranked,
                map,
                Some(chr_id),
                &personal,
            ));
            let (worth, why) = self.exp_for_kill(template);
            out.extend(self.award_kill_experience(worth, &why, chr_id, &shares));
            out.extend(self.credit_kill_to_quests(template, chr_id));
            // The King Slime's shoes and its twenty Slimes. session/firsttime.rs.
            out.extend(self.party_quest_kill(map, template, died_at));
            // The registry's entry for a mob that no longer exists. `reconcile` on the
            // next field entry would catch it anyway - this is so the count in a log line
            // means what it says between now and then.
            self.fields.controllers().forget(map, object_id);
        }
        // The template's real maxHP, because 0x03F0 carries a PERCENTAGE.
        let max_hp = self
            .config
            .mobs
            .get(&map.map)
            .and_then(|l| l.iter().find(|m| m.object_id == object_id))
            .map(|m| m.hp)
            .unwrap_or(hp_before);
        for (opcode, body) in net::combat::mob_hit_replies(object_id, &hit, max_hp) {
            let reply = Reply {
                opcode,
                body,
                what: format!(
                    "mob {} took {} ({} -> {}){}",
                    object_id,
                    hit.damage_applied,
                    hit.hp_before,
                    hit.hp_after,
                    if hit.died { " - DEAD, leaving the field" } else { "" }
                ),
            };
            // **The damage half of the owner's 2026-09-01 sentence**: *"All clients need to see
            // other clients damages to mobs."* `0x03F0` is the health bar and `0x03D1` is
            // the death, and both say exactly the same thing to every viewer.
            //
            // **Publish the SAME `(opcode, body)` the attacker gets. Do not recompute.**
            // `0x03F0`'s hp field is a **percentage**, 0..100 - `net::combat::hp_percent`,
            // `research/mob-hp-bar.md` - and the absolute went out once and drew a 45-HP
            // snail at 27%. A second call site that looked `max_hp` up its own way is
            // exactly how that unit error comes back. One body, two destinations.
            //
            // An observer whose pool has never held this object id is safe: the second
            // dispatcher looks the id up and returns (`141d32b62 je 0x141d33432`), and
            // `0x03D1` reads its whole body first and then does the same. **[L]**
            let audience = crate::mobshare::audience_for(opcode, object_id);
            if audience.is_map_wide() {
                self.bus().publish(self.subscriber, map, reply.clone(), audience.supersedes());
            }
            out.push(reply);
        }
        out
    }

    /// **MP Eater**: on a magic attack that landed, `prop`% chance per mob hit to absorb `x`%
    /// of that mob's **maximum** MP (*"absorb 30% of the enemy's Max MP"*; the mob's `maxMP`
    /// is column 3 of `mobtemplates.txt`). One proc per swing, then the row's `cooltime` of
    /// 5 s. All three copies of the passive (`2100000`, `2200000`, `2300000`) are one skill
    /// with three ids; whichever the character has is read. **[L]** for the columns, **[I]**
    /// that a mob with `maxMP 0` in its row gives nothing rather than a floor.
    pub(super) fn mp_eater(&mut self, templates_hit: &[u32]) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let Some((skill, level)) = crate::advbuffs::MP_EATER.iter().find_map(|&s| {
            self.store.skill_level(chr.id, s).ok().filter(|l| *l > 0).map(|l| (s, l))
        }) else {
            return Vec::new();
        };
        let now = self.clock_ms;
        if now < self.mp_eater_ready_ms {
            return Vec::new();
        }
        let Some(row) = self.config.firstjob.level(skill, level).copied() else { return Vec::new() };
        let prop = u64::from(row.prop.unwrap_or(0));
        let x = row.x.and_then(|x| u64::try_from(x).ok()).unwrap_or(0);
        let cool = u64::from(row.cooltime_seconds.unwrap_or(5));
        let cap = self.pools(&chr).max_mp;
        for template in templates_hit {
            if prop == 0 || self.rng.next() % 100 >= prop {
                continue;
            }
            let mob_max_mp = self.config.mob_templates.get(template).map(|t| u64::from(t.max_mp)).unwrap_or(0);
            let gain = u32::try_from(mob_max_mp * x / 100).unwrap_or(u32::MAX);
            if gain == 0 || chr.mp >= cap {
                continue;
            }
            let mp = chr.mp.saturating_add(gain).min(cap);
            let gained = mp - chr.mp;
            chr.mp = mp;
            if self.store.save_character_progress(&chr).is_err() {
                return Vec::new();
            }
            self.mp_eater_ready_ms = now.saturating_add(cool * 1000);
            return vec![Reply {
                opcode: net::stats::STAT_CHANGED,
                body: net::stats::StatChange { mp: Some(chr.mp), ..Default::default() }.build(),
                what: format!(
                    "StatChanged: MP Eater absorbed {gained} mp ({x}% of template {template}'s {mob_max_mp}) -> {}/{cap}; next proc in {cool}s",
                    chr.mp
                ),
            }];
        }
        Vec::new()
    }

    /// Restore a flat amount of HP, capped at the drawn ceiling, with the blue number. The
    /// primitive behind `heal_percent`, Drain, and anything else that gives HP outside regen.
    pub(super) fn heal_flat(&mut self, add: u32, why: &str) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if chr.hp == 0 || add == 0 {
            return Vec::new();
        }
        let cap = self.pools(&chr).max_hp;
        let hp = chr.hp.saturating_add(add).min(cap);
        let healed = hp - chr.hp;
        if healed == 0 {
            return Vec::new();
        }
        chr.hp = hp;
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not save the health {why} gave you: {e}"));
        }
        vec![
            Reply {
                opcode: net::stats::STAT_CHANGED,
                body: net::stats::StatChange { hp: Some(chr.hp), ..Default::default() }.build(),
                what: format!("StatChanged: {why} +{healed} hp -> {}/{cap}", chr.hp),
            },
            Reply {
                opcode: net::stats::USER_EFFECT_LOCAL,
                body: net::revive::recovery_number(healed as i32, 0),
                what: format!("UserEffectLocal effect 0x41: the blue +{healed} from {why}"),
            },
        ]
    }


    /// Roll a kill's drops for **one** owner, and hand them back to the caller.
    ///
    /// The one-candidate case of [`Session::drops_from_kill_for`], which is what the attack
    /// path calls and where all the working is.
    ///
    /// **Tests only, and that is not an oversight.** Production went to the ranked form the
    /// day the drops became the top damager's, and three tests that predate it - the drop
    /// roll, the stagger and the mob-position fallback - drive this shared body through the
    /// simplest possible caller. Leaving it compiled into the server would be a second entry
    /// point into drop placement that always gives the loot to the killer.
    #[cfg(test)]
    pub(super) fn drops_from_kill(
        &mut self,
        template: u32,
        object_id: u32,
        died_at: Option<(i16, i16)>,
        killer: u32,
        map: crate::fields::FieldKey,
    ) -> Vec<Reply> {
        // `Some(killer)` says "the one candidate is this connection's own character", which is
        // what makes the packets come back in the return value rather than going over the bus.
        // That is this form's whole contract and it is what every existing caller relies on.
        self.drops_from_kill_for(template, object_id, died_at, &[killer], map, Some(killer), &[])
    }

    /// **Roll what a dead mob leaves on the floor, put it there, and give it to the first
    /// candidate in `ranked` who can actually be handed it.**
    ///
    /// **Two tables, in order: this mob's own, then the global one.** The owner asked for the
    /// global table so an event item can drop from anything without touching code; it is
    /// empty until there is an event. `crate::droptables` owns the rolling and the policy,
    /// and this function owns only the consequences.
    ///
    /// # Where it lands, and why it can decline
    ///
    /// Where the **mob** was, from `died_at`, falling back to the player's last known
    /// position for a mob that never moved and so never reported one. With neither it drops
    /// **nothing** and says so in the log rather than guessing: an item placed where the
    /// player cannot reach looks identical to no drop at all, and would make the next run
    /// unreadable.
    ///
    /// # Nothing can be picked up yet
    ///
    /// The player's pick-up request opcode is `0x032C`, measured 2026-08-20 - `crate::drops`.
    ///
    /// # Who gets to see it
    ///
    /// `ranked` is `crate::mobshare::drop_audience` - the damage ranking highest first, with
    /// the killer appended as the terminator. The walk stops at the first candidate the bus
    /// will deliver to **on this map**, because an item nobody can see is indistinguishable
    /// from no item at all, which is this function's own sentence about a drop placed out of
    /// reach.
    ///
    /// # Why the walk mints first and re-addresses after
    ///
    /// `Bus::publish_to_character` is the **only** presence query the bus exposes, and it
    /// answers by attempting the delivery. A candidate who is not on this map is refused and
    /// **nothing is posted**, so trying them in order is free; the drop is minted once and
    /// [`crate::drops::DropTable::readdress`] moves it down the ranking until one lands. No
    /// client has heard about the drop while that is going on, so the re-address is invisible.
    ///
    /// The killer's own copy goes into the returned `Vec` rather than over the bus, for the
    /// same reason `award_kill_experience` pays itself directly: it is this connection's own
    /// reply and the bus deliberately never delivers to the publisher.
    ///
    /// `mine` is the character **this connection is playing**, and it is a parameter rather
    /// than a `claimed_character()` read so that [`Session::drops_from_kill`] can keep its
    /// old contract exactly: one candidate, always ours, always returned.
    pub(super) fn drops_from_kill_for(
        &mut self,
        template: u32,
        object_id: u32,
        // Where the mob was when it died, read BEFORE it was removed from the field.
        died_at: Option<(i16, i16)>,
        ranked: &[u32],
        map: crate::fields::FieldKey,
        mine: Option<u32>,
        // Drops each only ONE member may see, placed together in one slot after the shared
        // ones - see the end of this function.
        personal: &[(u32, store::Item)],
    ) -> Vec<Reply> {
        // The provisional owner, replaced by the walk below the moment a drop exists. Never
        // empty in practice - `drop_audience` always appends the killer - but a caller that
        // passed an empty list must not silently mint drops owned by character 0.
        let Some(&first_choice) = ranked.first() else { return Vec::new() };
        if template == 0 {
            return Vec::new(); // an object id we never spawned; nothing to look up
        }
        // **Where the mob was, not where the player is.** The owner, after seeing it on screen:
        // *"they should drop from the killed mob's position, not from the player character
        // position"*. `mob_position` is fed by the client's own movement reports; the
        // player's position is the fallback for a mob that never moved.
        let _ = object_id;
        let at = died_at.or(self.last_position);
        let Some((x, y)) = at else {
            return self.notice(
                "A mob died with drops to give, but the server does not know where it was standing, so it dropped nothing rather than putting it out of reach."
                    .to_string(),
            );
        };
        // The server's drop rate scales every chance in the table. Read before the borrow
        // below, because it is a database query and `roll_at` holds `self.rng`.
        let drop_rate = self.rate(store::rates::RateKind::Drop);
        let mut rolled = {
            let rng = &mut self.rng;
            self.config.drops.roll_at(template, drop_rate, &mut || rng.next())
        };
        // **A mob with no authored meso row still drops mesos, scaled by its level.**
        // Only 23 of the 170 mobs with a drop table have one, so without this most of the
        // game drops items and no money at all - Slime has seventeen item rows and no mesos.
        // `has_meso_row` asks whether a row EXISTS rather than whether it hit, so an
        // authored row at any chance - including a `0` opt-out - wins over this default.
        if !self.config.drops.has_meso_row(template) {
            if let Some((min, max)) =
                self.config
                    .mob_templates
                    .get(&template)
                    .and_then(|t| crate::droptables::level_meso_range(t.level))
            {
                let span = u64::from(max - min) + 1;
                let amount = min + (self.rng.next() % span) as u32;
                rolled.push(crate::droptables::Rolled {
                    item_id: crate::droptables::MESOS,
                    quantity: amount,
                });
            }
        }

        // **The Dark Marble: the only item in the game whose drop depends on WHERE it died.**
        //
        // The four second-job test fields spawn dedicated mob templates - 800010..800017 -
        // and the marble is the proof the examiner asks 30 of. Two things have to be true at
        // once and neither is enough on its own:
        //
        // * **The mob must be a test mob.** `data/drops.txt` already carries scraped marble
        //   rows for six of the eight at 6%, which is 30 marbles in roughly 500 kills, and it
        //   is **missing 800014 and 800017 entirely** - so the Thief's Cold Eye and the
        //   Warrior's Lupin drop nothing towards a test they are half of. Doing it here fixes
        //   both without depending on what the next scrape decides to write.
        // * **The map must be that mob's own field.** `800010` and `800015` also spawn on
        //   80003500 and `800011` also spawns on **10006160, Precipice of Darkness** - an
        //   ordinary field an ordinary player grinds. **[L]** Without the map test three of
        //   the eight leak the test's proof into normal play.
        //
        // So the filter runs first and is unconditional: any marble that is not this map's
        // marble is removed however it got into the roll. Then the right one is put in
        // exactly once, so a 6% row that happened to hit cannot stack with the guarantee.
        // `secondjob::MARBLE_DROP_IS_CERTAIN` is the [I] policy and the single place to
        // change it.
        let marble_here = crate::secondjob::marble_for_kill(template, map.map);
        rolled.retain(|r| {
            !crate::secondjob::is_marble(r.item_id) || Some(r.item_id) == marble_here
        });
        if let Some(marble) = marble_here {
            let already = rolled.iter().any(|r| r.item_id == marble);
            if !already && crate::secondjob::MARBLE_DROP_IS_CERTAIN {
                rolled.push(crate::droptables::Rolled { item_id: marble, quantity: 1 });
            }
        }
        let mut out = Vec::new();
        let me = mine;
        // **The party seam for drops.** The owner, 2026-09-05: *"All members of a party should see
        // all drops killed by members of the party ... Once someone leaves the party, they can
        // no longer pick up the party's drops unless they were the killer."* So when the
        // killer is in a party the drop is stamped with that party id and owned by the KILLER
        // (not the top damager), and every current member on this field is shown it. The
        // member roster and presence are read ONCE here, before any drop is minted, and the
        // guard is dropped immediately so nothing holds the parties lock across `with_drops`.
        let party = me.and_then(|k| {
            self.fields
                .parties()
                .party_of(k)
                .filter(|p| p.members.len() > 1)
                .map(|p| (p.id, p.members.clone()))
        });
        let party_here: Vec<u32> = match &party {
            Some((_, members)) => self.bus().characters_on(map, members),
            None => Vec::new(),
        };
        // Read the meso rate ONCE, not once per drop: it is a database query, and it cannot
        // change between two items falling off the same mob.
        let meso_rate = self.rate(store::rates::RateKind::Meso);
        // **Quest items are per-player.** The owner, 2026-09-08, with a screenshot of an ETC tab
        // full of them: an item labelled "Quest Item" must only be offered to somebody who
        // has that quest ACTIVE. Two players killing the same mob can legitimately differ, so
        // this filters the ROLL - which belongs to this kill - and never the table, which is
        // shared by the whole map.
        //
        // The audience is everyone this drop could reach: the party members present when the
        // killer is in one, otherwise the damage ranking the walk below tries in order. One
        // database read per character per KILL, taken here so the item loop takes none.
        // `crate::questitems` owns every rule - the orphan policy and the Dark Marble
        // exemption included; this call site owns only the consequence.
        let quest_audience = crate::questitems::audience_for(
            &self.store,
            if party.is_some() { party_here.as_slice() } else { ranked },
        );
        let before = rolled.len();
        self.config.quest_items.filter_roll(&mut rolled, &quest_audience);
        if rolled.len() != before {
            crate::server::log(&format!(
                "   {} of {before} rolled item(s) from template {template} are quest items that \
                 nobody in {:?} has the quest for, and were not offered",
                before - rolled.len(),
                quest_audience.iter().map(|(id, _)| *id).collect::<Vec<_>>()
            ));
        }
        // **Stagger them.** The owner, with a screenshot of the live server: *"the items that
        // drop should also be slightly staggered from each other"*. Three items landing on
        // exactly the same pixel render as one. Centred on the mob so a single drop is
        // exactly where it died, and spread outward from there.
        // One more slot when there are personal drops: they all share the last one.
        let n = rolled.len() as i16 + i16::from(!personal.is_empty());
        // The corpse, bound before the loop shadows `x` with the staggered landing spot.
        // Both ends of the arc have to exist at once or there is no arc.
        let (mob_x, mob_y) = (x, y);
        for (i, r) in rolled.into_iter().enumerate() {
            let offset = (i as i16 - (n - 1) / 2) * crate::drops::DROP_STAGGER_PX;
            let x = x.saturating_add(offset);
            // **Put it on the floor.** The owner, 2026-08-21: *"item drops still go through the
            // ground and are unable to be picked up."* The stagger above moves the item
            // sideways and nothing ever moved it vertically, so on any ground that is not
            // flat it ends up in mid-air or inside terrain - and the client's pick-up box
            // reaches only 10 px below the player's feet, so such an item is drawn and
            // uncollectable. A third of this client's 94089 floor segments are sloped.
            //
            // **Two different fallbacks, and the difference matters.** With no table loaded
            // at all, keep the staggered position - that is exactly what shipped before this
            // existed, and collapsing every drop onto the corpse would be a visible
            // regression in the case where we simply have no data. With a table loaded, a
            // `None` means the vertical through `x` meets no surface anywhere on the map, and
            // there the corpse is strictly better: a mob was standing on it, so it is
            // certainly a floor.
            let fallback = if self.config.footholds.is_empty() { (x, y) } else { (mob_x, mob_y) };
            let placed = self.config.footholds.landing(map.map, x, y);
            let (x, y) = placed.map(|l| (l.x, l.y)).unwrap_or(fallback);
            let (item, inv_type, meso) = if r.is_mesos() {
                // A placeholder item: `LiveDrop::is_meso` gates every read of it.
                // **The meso rate multiplies the pile on the floor, not the credit on
                // pick-up.** Either would show the right number in the end, but this way the
                // amount in the drop, the amount in the message and the amount added to the
                // balance are all the same number, and a run that disagrees with itself is
                // the kind of evidence this project keeps having to re-gather.
                let amount = meso_rate.apply(u64::from(r.quantity)).min(u64::from(u32::MAX)) as u32;
                (store::Item::bundle(0, 0), store::InventoryType::Etc, amount)
            } else {
                // An id whose leading digit names no tab is not an item this game has. Skip
                // it rather than guess a bag: the same rule `!item` follows, and for the same
                // reason - the client has to render whatever arrives.
                let Some(inv) = store::InventoryType::for_item(r.item_id) else {
                    continue;
                };
                let item = if inv == store::InventoryType::Equip {
                    // **Item variance**: a mob's equip rolls around its template. The owner,
                    // 2026-09-24 - `crate::variance` has the rules.
                    self.mob_drop_variance(store::Item::equip(r.item_id))
                } else {
                    store::Item::bundle(r.item_id, r.quantity.min(u32::from(u16::MAX)) as u16)
                };
                (item, inv, 0u32)
            };
            let now = self.clock_ms;
            // A party drop belongs to the killer (so a member who later leaves loses it while
            // the killer keeps it), a solo drop to the top damager the walk below settles on.
            let (owner_id, party_id) = match &party {
                Some((id, _)) => (me.unwrap_or(first_choice), *id),
                None => (first_choice, 0),
            };
            let (drop_id, first_reply) = self.fields.with_drops(map, |d| {
                d.drop_from_mob(crate::drops::DropFromMob {
                    from_mob: true,
                    map_id: map,
                    owner_id,
                    item,
                    inv_type,
                    meso,
                    x,
                    y,
                    // The corpse, un-staggered: where the arc starts.
                    source_x: mob_x,
                    source_y: mob_y,
                    now_ms: now,
                    party_id,
                })
            });
            // The landing goes in the log line, so "did the placement do anything" is a
            // `world.log` grep instead of a second manual launch. Only when it actually
            // moved the item - an unmoved drop on flat ground is the common case and would
            // bury the interesting ones.
            let note = placed
                .filter(|l| l.moved != 0)
                .map(|l| format!(" [{}]", l.what()))
                .unwrap_or_default();

            // **A party drop is shown to EVERY member on this field, not one winner.** The
            // enter packet is the same bytes for everyone - it names the owner (the killer),
            // not the recipient - so this connection keeps a copy and the rest get it over the
            // bus. Pick-up is gated by membership at take time, so a member seeing it can take
            // it and a bystander who is not in the party cannot.
            if party.is_some() {
                let mut first_reply = first_reply.clone();
                first_reply.what.push_str(&note);
                let mut shown_to = Vec::new();
                for &member in &party_here {
                    // **A party drop is still per-player when it is a quest item.** The enter
                    // packet names the owner, not the recipient, so the bytes are the same for
                    // everyone - but a member with no quest for it must not be shown it, or an
                    // item they can never hand in is on their screen and inside their reach.
                    if !self
                        .config
                        .quest_items
                        .may_receive_in(r.item_id, member, &quest_audience)
                    {
                        continue;
                    }
                    if Some(member) == me {
                        out.push(first_reply.clone());
                        shown_to.push(member);
                    } else if self.bus().publish_to_character(member, map, first_reply.clone()) {
                        shown_to.push(member);
                    }
                }
                crate::server::log(&format!(
                    "   drop {drop_id} on map {map} is a PARTY drop owned by {owner_id}; shown to \
                     {shown_to:?} of party members {party_here:?} on this field"
                ));
                continue;
            }

            // **The walk.** Stop at the first candidate this drop can actually be given to on
            // this map. `ranked[0]` already owns it, so the first iteration never re-addresses.
            let mut winner = None;
            for (rank, candidate) in ranked.iter().enumerate() {
                // **The recipient has to be eligible for THIS item.** `filter_roll` only
                // guaranteed that SOMEBODY in the audience is; the walk is where one candidate
                // is chosen, so it is where a quest item stops at the right person instead of
                // at the top damager. Leaving `winner` as None here is the EXISTING "reached
                // NOBODY" path, whose log line already covers it - not a new class of leak.
                if !self
                    .config
                    .quest_items
                    .may_receive_in(r.item_id, *candidate, &quest_audience)
                {
                    continue;
                }
                let mut reply = if rank == 0 {
                    first_reply.clone()
                } else {
                    match self.fields.with_drops(map, |d| d.readdress(drop_id, *candidate)) {
                        Some(r) => r,
                        None => break, // swept between the mint and now; nothing to give away
                    }
                };
                reply.what.push_str(&note);
                if Some(*candidate) == me {
                    // Our own reply. The bus never delivers to its publisher, so this is the
                    // only way this connection can be the recipient.
                    out.push(reply);
                    winner = Some(*candidate);
                    break;
                }
                if self.bus().publish_to_character(*candidate, map, reply) {
                    winner = Some(*candidate);
                    break;
                }
            }
            match winner {
                Some(w) => crate::server::log(&format!(
                    "   drop {drop_id} on map {map} goes to character {w} - the top damager \
                     still on this field, out of {ranked:?}"
                )),
                None => crate::server::log(&format!(
                    "   drop {drop_id} on map {map} reached NOBODY: none of {ranked:?} is \
                     playing on this field. An item nobody can see is the same as no item, so \
                     this is a finding, not routine"
                )),
            }
        }

        // **Personal drops share ONE slot, the last in the row.** The owner, 2026-09-24, on the
        // King Slime's shoes: they landed *"right on top of the pass"*, and *"the clients
        // should also not have weird drop placement such as empty spaces where they do not
        // see a drop they can pick up because it's instanced for someone else."* A slot per
        // member would leave every client a gap for each pair it cannot see; one slot for all
        // of them means each client sees exactly one drop there - its own - after the shared
        // ones, and nothing sits on top of anything it can see.
        if !personal.is_empty() {
            let offset = ((n - 1) - (n - 1) / 2) * crate::drops::DROP_STAGGER_PX;
            let x = mob_x.saturating_add(offset);
            let fallback = if self.config.footholds.is_empty() { (x, mob_y) } else { (mob_x, mob_y) };
            let (x, y) = self.config.footholds.landing(map.map, x, mob_y).map(|l| (l.x, l.y)).unwrap_or(fallback);
            for (member, item) in personal {
                let Some(inv_type) = store::InventoryType::for_item(item.item_id) else { continue };
                // Each member's pair is rolled on its own - item variance, `crate::variance`.
                let item = self.mob_drop_variance(*item);
                let now = self.clock_ms;
                let (drop_id, enter) = self.fields.with_drops(map, |d| {
                    d.personal_drop_from_mob(crate::drops::DropFromMob {
                        from_mob: true,
                        map_id: map,
                        owner_id: *member,
                        item,
                        inv_type,
                        meso: 0,
                        x,
                        y,
                        source_x: mob_x,
                        source_y: mob_y,
                        now_ms: now,
                        party_id: 0,
                    })
                });
                let shown = if Some(*member) == me {
                    out.push(enter);
                    true
                } else {
                    self.bus().publish_to_character(*member, map, enter)
                };
                crate::server::log(&format!(
                    "   drop {drop_id} ({}) on map {map} at ({x}, {y}) is PERSONAL to {member} - slot {} of {n}, the one every member's own copy shares; {}",
                    item.item_id,
                    n - 1,
                    if shown { "shown to them" } else { "they are not on this field" }
                ));
            }
        }
        out
    }


    /// **An equip a mob drops, with its stats rolled** (`crate::variance`); anything else
    /// unchanged. The roll goes in the log, because a player asking "why is mine worse"
    /// deserves an answer that does not need a second drop.
    pub(super) fn mob_drop_variance(&mut self, item: store::Item) -> store::Item {
        let rolled = {
            let rng = &mut self.rng;
            crate::variance::for_mob_drop(&self.config.equips, item, &mut || rng.next())
        };
        if let (store::ItemKind::Equip(Some(after)), Some(t)) = (rolled.kind, self.config.equips.get(&item.item_id)) {
            crate::server::log(&format!(
                "   variance: {} (req level {}, range {:.1}) rolled {:?} from template {:?}",
                item.item_id,
                t.req_level,
                crate::variance::range(t.req_level, item.item_id),
                after.stats,
                t.fresh_stats().stats
            ));
        }
        rolled
    }

    /// Spawn everything on this map whose timer is due, and tell the client.
    ///
    /// **This is both the first fill of a field and every refill after a kill** - see
    /// `crate::fields`, where a newly-registered field starts with every spawn point due
    /// rather than alive. The owner: *"on first enter, no mobs should exist until the respawn
    /// timer kicks in"* and *"The mobs that I kill also do not respawn."*
    ///
    /// Sends the same pair a field entry does, in the same order: a mob the client has not
    /// been given control of is a picture that never moves.
    ///
    /// # The spawn is PUBLISHED and the grant is not, and that split is the whole fix
    ///
    /// `Fields::due_respawns` **drains** `field.pending`. Every session on the map ticks, so
    /// whichever ticked first took the new mobs and every other player on that field was
    /// never told they exist - a respawn was **unicast**, and there is no second packet later
    /// that would have healed it. So the `0x03C6` goes to the map.
    ///
    /// The `0x03D2` does **not**, and must not: two `MOB_CHANGE_CONTROLLER`s for one mob are
    /// two clients each rolling their own wander (`research/mob-behaviour.md` §5.1 - the
    /// client builds the path out of its own random source), and the two screens then diverge
    /// on the first step and never reconverge. `Controllers::claim_one` is a test-and-set, so
    /// the grant follows the claim rather than the tick.
    ///
    /// A side effect worth having: whichever session ticks first takes each new mob, so
    /// control spreads across the connections on a map over time instead of one client
    /// holding everything and freezing every monster on the field if it stalls.
    pub(super) fn spawn_due_mobs(&mut self, map: crate::fields::FieldKey, now_ms: u64) -> Vec<Reply> {
        let arrived = self.fields.due_respawns(map, &self.config, now_ms);
        let mut out = Vec::new();
        for live in arrived {
            let mut mob = live.as_seen();
            // This one really is arriving while the player watches, so it keeps the spawn
            // effect. The pair of these two lines IS the feature - one value for "was already
            // here", another for "just turned up".
            mob.appear_type = net::mob::APPEAR_SPAWNING;
            // Same as on field entry: without this the client has no attack power for the mob
            // and its own contact-damage formula floors at 1. See `forced_stat_for`.
            mob.forced_stat = self.forced_stat_for(mob.template_id);
            let spawn = Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(&mob),
                what: format!(
                    "MobEnterField: SPAWN of template {} at ({}, {}), object id {}, hp {}.",
                    mob.template_id, mob.x, mob.y, mob.object_id, mob.hp
                ),
            };
            let audience = crate::mobshare::audience_for(spawn.opcode, mob.object_id);
            if audience.is_map_wide() {
                self.bus().publish(self.subscriber, map, spawn.clone(), audience.supersedes());
            }
            out.push(spawn);
            // **A narrow race, and it is benign.** A player entering the map at the instant
            // this publishes can be handed the same `0x03C6` twice - once from `mobs_on`, once
            // from here. `FUN_141d33630` looks the object id up at `141d33711` and, when it is
            // found, takes the branch at `141d33725` that re-initialises the existing mob
            // instead of creating a second one. **[L]** for the fork.
            if self.fields.controllers().claim_one(map, mob.object_id, self.subscriber.get()) {
                out.push(Reply {
                    opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                    body: net::mobmove::mob_change_controller(&mob, net::mobmove::CONTROL_NORMAL),
                    what: format!(
                        "MobChangeController: object id {} to this client, which claimed it. \
                         Exactly one connection controls each mob - crate::mobshare.",
                        mob.object_id
                    ),
                });
            }
        }
        out
    }


    /// Give the character experience, level them up if it pays for one, and tell the client.
    ///
    /// **One path for every source of experience**, so a kill and `!exp` cannot drift: the
    /// levelling rule, the persistence and the `0x007C` all live here. `crate::expcurve`
    /// owns the arithmetic.
    ///
    /// The level-up animation comes free - the `0x007C` handler plays
    /// `Effect/BasicEff.img/LevelUp` itself when the level in the packet is higher than the
    /// one the client is holding, so there is no separate effect packet to send.
    ///
    /// Returns nothing at all for an award of zero, which is the ordinary case for a mob
    /// with no EXP value: a `0x007C` that changes nothing is a packet the client has to
    /// parse for no reason.
    /// What a kill of `template` is worth after the server's EXP rate, and the phrase that
    /// goes in the log beside it.
    ///
    /// **A seam, not a convenience.** The multiplication used to be two lines at the call
    /// site inside the attack handler, where the only way to test it was to build a
    /// 147-byte attack packet. Here a test can ask directly.
    ///
    /// The rate is global and lives in the database, because a channel is a process -
    /// `store::rates`. **`!exp` deliberately does not come through here**: it is a debugging
    /// command that means "give me exactly this much", and one that quietly doubled would be
    /// useless for checking the curve.
    pub(super) fn exp_for_kill(&self, template: u32) -> (u64, String) {
        let base = self.config.mob_exp.get(&template).copied().unwrap_or(0);
        let rate = self.rate(store::rates::RateKind::Exp);
        let worth = rate.apply(u64::from(base));
        // Holy Symbol: `x`% more while held - *"5%"* at level 1, 35% at 30. Self only in this
        // client's data (no rectangle, `processtype 6`). **[L]** for the column.
        let symbol = u64::try_from(self.held_value(net::jobbuffs::CTS_HOLY_SYMBOL)).unwrap_or(0);
        let worth = worth.saturating_add(worth.saturating_mul(symbol) / 100);
        let why = match (rate.is_normal(), symbol > 0) {
            (true, false) => "a kill".to_string(),
            (false, false) => format!("a kill ({base} at {rate}x)"),
            (true, true) => format!("a kill ({base}, Holy Symbol +{symbol}%)"),
            (false, true) => format!("a kill ({base} at {rate}x, Holy Symbol +{symbol}%)"),
        };
        (worth, why)
    }



    /// `amount` after this session's own EXP coupon, and a note for the log.
    ///
    /// The owner, 2026-09-09: *"Once the EXP buff is applied, either the 2x or the 3x EXP buff, are
    /// we sure that the EXP gained is actually properly being modified?"* It was not. The
    /// percentage was parsed out of the client's own data, stored on the item, and **read by
    /// nothing**, so the coupon was consumed and changed no number anywhere.
    ///
    /// # Where it is applied, and where it deliberately is not
    ///
    /// **Only on kill experience, and only to the recipient's own share.** The pool in
    /// [`Session::exp_for_kill`] is split between everyone who did damage, so multiplying it
    /// there would pay the whole map out of one player's coupon. Each session therefore
    /// applies its own, at the two points where a session pays *itself*: its cut of its own
    /// kill, and a share collected off the bus.
    ///
    /// Not on `!exp`, which means "give me exactly this much" and has a test saying so, and
    /// not on quest turn-ins - the existing EXP rate already draws that same line, and
    /// `the_quest_exp_rate_multiplies_a_turn_in_and_the_kill_rate_does_not` pins it.
    ///
    /// An expired coupon is dropped on read rather than swept by the tick: there is exactly
    /// one reader, so there is nowhere for a stale value to be seen from.
    pub(super) fn with_exp_coupon(&mut self, amount: u64) -> (u64, String) {
        let now = self.clock_ms;
        let Some(coupon) = self.exp_coupon else { return (amount, String::new()) };
        if !coupon.active_at(now) {
            self.exp_coupon = None;
            return (amount, String::new());
        }
        let boosted = coupon.applied(amount);
        (boosted, format!(" [{} EXP coupon: {amount} -> {boosted}]", coupon.label()))
    }

    /// Award experience for a kill, splitting it by who actually did the damage.
    ///
    /// The owner, 2026-08-20, with a screenshot: *"if I was the person who dealt majority damage,
    /// I should see a white line of EXP gained. If I was not the person who dealt majority
    /// damage, I would only get a % portion of the EXP that belonged to the mob ... and that
    /// line would be yellow."*
    ///
    /// **Every contributor is paid, including the ones on other connections.** This session
    /// pays itself directly; everyone else is paid across the channel's message bus, as a
    /// *fact* rather than a packet - `crate::broadcast::Event::Experience`.
    ///
    /// It has to be a fact. `0x007C` carries the **recipient's own** new total and level,
    /// which only their session can compute from their own record, so a packet built here
    /// would show them somebody else's EXP. Each session turns the fact into its own packet
    /// when it collects its mail, in `Session::collect_mail`.
    ///
    /// A share addressed to nobody - they logged out or changed channel between landing the
    /// hit and the mob dying - is dropped and logged. That is ordinary, not an error.
    ///
    /// **Parties do not exist**, so the 70/30 split and `You received party EXP` are not
    /// implemented. `research/exp-sharing.md` records the rule so it does not have to be
    /// asked for twice.
    pub(super) fn award_kill_experience(
        &mut self,
        worth: u64,
        why: &str,
        chr_id: u32,
        shares: &[crate::fields::DamageShare],
    ) -> Vec<Reply> {
        // **In a party, the split is by membership, not by damage.** The owner, 2026-09-05: *"my
        // party members are not getting party exp in yellow"*, and the rule in
        // `research/exp-sharing.md`: the killer gets 70%, every other party member on the same
        // field splits the remaining 30% equally, their line yellow. A party member who never
        // touched the mob is still paid; a non-party bystander is not. So the damage `shares`
        // do not enter this path at all - presence does.
        if let Some(party) = self.party_exp_split(worth, chr_id) {
            return party;
        }

        // Nothing credited means nothing landed - a mob that died without being hurt, which
        // only a bug produces. Pay the killer in full rather than nothing.
        let Some(mine) = shares.iter().find(|s| s.character == chr_id) else {
            return self.award_experience(worth, why, true, false);
        };
        // Ours first, and through the same reason-line builder everyone else gets, so the
        // two paths cannot drift apart.
        // **The killer's own cut, through their own coupon.** Not the pool above - that is
        // split between everybody who helped, and one player's coupon must not pay the map.
        let (mine_amount, coupon_note) = self.with_exp_coupon(mine.cut_of(worth));
        let out = self.award_experience(
            mine_amount,
            &format!("{}{coupon_note}", Self::share_reason(why, mine)),
            mine.majority,
            false,
        );

        // **And now everyone else who helped.** This is what the bus was built for.
        for share in shares.iter().filter(|s| s.character != chr_id) {
            let delivered = self.bus().send_to_character(
                share.character,
                crate::broadcast::Event::Experience {
                    amount: share.cut_of(worth),
                    why: Self::share_reason(why, share),
                    white: share.majority,
                },
            );
            if !delivered {
                // Reported rather than swallowed. A share that evaporates in silence is
                // indistinguishable from a share that was never computed, and this project
                // has already shipped one guard whose answer nobody read.
                crate::server::log(&format!(
                    "exp: character {} earned {} from {why}, but nobody on this channel is playing them",
                    share.character,
                    share.cut_of(worth),
                ));
            }
        }
        out
    }

    /// The reason line for one contributor's cut.
    ///
    /// The majority holder is paid in full and told the plain reason; everyone else is told
    /// what fraction of the damage they did, because a smaller number with no denominator
    /// reads as a bug rather than as a share.
    /// The party split, or `None` when the killer is not in a party with anyone else on this
    /// field - in which case the caller falls back to the solo damage-share path.
    ///
    /// The owner, 2026-09-05, and `research/exp-sharing.md`: the killer keeps **70%**, every other
    /// party member standing on the same map splits the other **30%** equally, and their line
    /// is **yellow** - the client shows *"You received EXP"* in yellow, which is this client's
    /// party-EXP presentation; there is no distinct string id for it. A member on another map
    /// or offline is not eligible and their slice is not minted, so the killer keeps the
    /// remainder and a lone party member on the field gets the full worth.
    ///
    /// **AFK is not modelled** - this server has no idle signal - so "on the same field and
    /// online" is the whole of the eligibility test. Said out loud because the owner's rule names
    /// AFK and this is the honest approximation of it.
    pub(super) fn party_exp_split(&mut self, worth: u64, chr_id: u32) -> Option<Vec<Reply>> {
        let map = self.claimed_character().map(|c| self.field_of(&c))?;
        let members = self.fields.parties().party_of(chr_id).map(|p| p.members.clone())?;
        if members.len() < 2 {
            return None; // a party of one is solo for EXP purposes
        }
        let others: Vec<u32> = members.into_iter().filter(|&m| m != chr_id).collect();
        let eligible = self.bus().characters_on(map, &others);
        if eligible.is_empty() {
            // Nobody else is here. The killer keeps the whole worth, white, and there is no
            // solo damage-share to fall back to - being in a party is what suppressed it.
            return Some(self.award_experience(worth, "for the kill (party, alone on the map)", true, false));
        }

        // **A COPY to each member, not a split.** The owner, 2026-09-06: *"you kill a shitty slime
        // that gets you 100 EXP ... 30% split copy for party member means killer (70% - 70
        // EXP), party mem 2-6 (30% each, 30 EXP each), this mob awarded a total of 220 EXP;
        // 50% ... 50 EXP each, total 320."* So the killer keeps 70% and every other member on
        // the field receives the party share of the WHOLE worth, each - the share is the
        // fifth `!setrates` field (`RateKind::Party`, 30% until changed), and the total paid
        // grows with the party. It replaces the 08-2x rule that divided 30% among them; that
        // rule is what the owner described as "split" before they specified "split copy".
        let share = self.rate(store::rates::RateKind::Party);
        let each = share.share_of(worth);
        let killer_share = worth * 70 / 100;

        for member in &eligible {
            let delivered = self.bus().send_to_character(
                *member,
                crate::broadcast::Event::Experience {
                    amount: each,
                    why: "party EXP".to_string(),
                    white: false,
                },
            );
            if !delivered {
                // Raced off the map between the presence check and now. Ordinary; logged, not
                // swallowed - a share that vanishes silently is the shape CLAUDE.md warns of.
                crate::server::log(&format!(
                    "   exp: party member {member} was on map {map} at the split and gone by \
                     delivery; their {each} party EXP was not paid"
                ));
            }
        }
        crate::server::log(&format!(
            "   exp: party kill on map {map} worth {worth} - killer {chr_id} keeps {killer_share} (70%, white), \
             {} member(s) each receive {each} ({} of the whole, yellow, party EXP); {} paid in all",
            eligible.len(),
            share.as_percent(),
            killer_share + each * eligible.len() as u64
        ));
        Some(self.award_experience(killer_share, "for the kill (party)", true, false))
    }

    fn share_reason(why: &str, share: &crate::fields::DamageShare) -> String {
        if share.majority {
            why.to_string()
        } else {
            format!("{why}, {}/{} of the damage", share.dealt, share.total)
        }
    }


    pub(super) fn award_experience(
        &mut self,
        gained: u64,
        why: &str,
        white: bool,
        to_chat: bool,
    ) -> Vec<Reply> {
        if gained == 0 {
            return Vec::new();
        }
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let before_level = chr.level;
        // The job picks the HP/MP line. Every character is a beginner today - job
        // advancement is goal E - so this is +16/+12 in practice, and the other four lines
        // are there so levelling does not have to be revisited when E lands.
        let a = self.config.exp_curve.award(chr.job, chr.level, chr.exp, gained);
        chr.level = a.level;
        chr.exp = a.exp;
        if a.levels > 0 {
            // Gains apply to the maximums, and a level-up refills - which is this game's
            // behaviour and also the only reading under which the numbers cannot end up
            // above their own maximum.
            chr.max_hp += a.max_hp;
            chr.max_mp += a.max_mp;
            // The refill goes to the ceiling the client draws, which is the base just
            // raised PLUS a learned Max HP/MP Increase - `Session::pools`. The stat packet
            // below still carries the base, because the client adds the percent itself.
            let pools = self.pools(&chr);
            chr.hp = pools.max_hp;
            chr.mp = pools.max_mp;
            chr.ap += a.ap;
        }
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not save your experience: {e}"));
        }

        let mut change = net::stats::StatChange::exp(chr.exp);
        if a.levels > 0 {
            change.level = Some(chr.level);
            change.max_hp = Some(chr.max_hp);
            change.max_mp = Some(chr.max_mp);
            change.hp = Some(chr.hp);
            change.mp = Some(chr.mp);
            change.ap = Some(chr.ap);
            // **Everyone else on the map sees and hears it.** The owner, 2026-09-08: *"the level up
            // sound was not broadcast to other players"*. The leveller's own animation is
            // client-side off the 0x007C below and is untouched - this is only for observers,
            // and `Bus::publish` excludes this connection by construction rather than by a
            // filter here, because a filter here is the thing that gets forgotten.
            //
            // Hung off the transition and placed AFTER `save_character_progress`, so a refused
            // save cannot broadcast a level nobody kept - `CLAUDE.md`'s Heena rule. Every
            // source of levelling funnels through here (kills, `!exp`, quest turn-ins, and
            // party shares collected off the bus), so this one hook covers all of them.
            let seen_by = crate::leveleffect::publish_level_up(self.bus(), self.subscriber, chr.id);
            crate::server::log(&format!(
                "   level: {} ({}) reached {} - 0x02AF UserEffectRemote effect 0 sent to \
                 {seen_by} other player(s) on this map",
                chr.name, chr.id, chr.level
            ));
        }
        // **The party window carries a level too.** Beside the level-up effect and for the
        // same reason: this is the one funnel every source of levelling goes through.
        // `session::party::party_window_after_level_up`.
        let party_rows = if a.levels > 0 { self.party_window_after_level_up() } else { Vec::new() };
        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: change.build(),
            what: format!(
                "StatChanged: +{gained} exp from {why} -> {} total{}. Bit 16 carries the NEW                  TOTAL; the client differences its own snapshot to draw the gain.",
                chr.exp,
                if a.levels > 0 {
                    format!(", LEVEL {before_level} -> {} (+{} ap)", chr.level, a.ap)
                } else {
                    String::new()
                }
            ),
        }];
        // **The right-hand message area, not the chat log.** The owner: *"it should actually show
        // on the right hand side of the client. We should not be outputting in the chat log
        // regarding level ups and item pickups."*
        //
        // `0x0089` type 3 carries the number and the client composes the sentence from its
        // own string table - `You received EXP (+211)` in this build. The third field is what
        // chooses the destination: non-zero routes the line to the chat log, zero routes it
        // to the other place. `research/client-messages.md`.
        //
        // The level-up itself gets no message: there is no level-up type in the table, and
        // `0x007C` already plays the animation when the level in it is higher than the one
        // the client holds. That is a "did not find", not a "there is none".
        // **Where the line lands is not the same for a kill and a quest.** The owner, 2026-08-21:
        // *"the quest completion EXP should not show up the same way as mob EXP. Quest EXP
        // and items should show up in the chat log as a gray text."*
        //
        // Same packet, one flag: `in_chat` non-zero posts to `FUN_1415eca30(text, 6)`, the
        // chat log, instead of the on-screen singleton. **[L]** for both destinations.
        out.push(Reply {
            opcode: net::message::MESSAGE,
            body: if to_chat {
                net::message::exp_gained_in_chat(gained, white)
            } else {
                net::message::exp_gained(gained, white)
            },
            what: format!(
                "Message: +{gained} exp, {}, to the {}",
                if white { "WHITE (majority damage)" } else { "yellow (a share)" },
                if to_chat { "CHAT LOG (type 6)" } else { "screen message area" }
            ),
        });
        // After the stat change, so the client has the new level before its party row
        // restates it.
        out.extend(party_rows);
        out
    }


    /// `0x00E5` - the client says the player took damage. **Apply it.**
    ///
    /// The owner, after a run: *"Getting hit by the mob does not subtract my HP."* The client
    /// computes the damage and does **not** apply it - established three ways with different
    /// blind spots in `research/user-hit.md` - so nothing moved their bar because nothing was
    /// moving it. The server is the authority and has to send the new HP back.
    ///
    /// **The new HP, not a delta.** `0x007C` mask bit 10 carries an absolute value; sending
    /// a difference would make the bar wander.
    ///
    /// Not a latch, so an unparseable body costs one hit rather than the session - which is
    /// why this can afford to be strict about the length.
    pub(super) fn on_user_hit(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(hit) = net::userhit::parse_user_hit(payload) else {
            return Vec::new();
        };
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        // Being hit counts as activity: standing in a mob's path is not resting.
        self.note_activity();
        if hit.damage == 0 {
            return Vec::new();
        }

        // **The server decides how much this hurt, and it did not used to.**
        //
        // The owner, 2026-08-21: *"all mobs should not only just deal 1 damage to the player."*
        // Every one of the twelve snail hits in that capture carried `damage = 1` - and this
        // handler applied the client's number verbatim, so the server had no opinion at all.
        //
        // The client's own data says a snail should hurt more than that. Template 2's
        // `PADamage` is **3**, and `damage::incoming_damage` over that character's real
        // numbers (level 7, 18 points of `incPDD`) gives **3 or 4**. `damage.rs` carries
        // that as a pinned test.
        //
        // So the two disagree, and the server's number is the one that moves the HP bar.
        // **Both go in the log line**, because that disagreement is the measurement the next
        // run is for: if the floating number over the player's head says one thing and the
        // bar drops by another, that is worth seeing rather than guessing about.
        //
        // Falls back to the client's number when the template is unknown or carries no
        // attack column - a mob we have no data for should still hurt.
        let claimed = hit.damage;
        let computed = self.incoming_damage_for(hit.mob_template_id, &chr);
        // **When the client sends a real number, take it - that is the whole point.**
        //
        // The owner, 2026-08-28: *"try harder to see how the client can compute its own mob damage
        // and then sending that to the server instead. This is one of the ways that both the
        // server and client can agree."*
        //
        // They are right, and the client can. `FUN_14288ac30`'s contact path computes
        // `mobAttack * x / 100` at `0x14288b337` and then **discards it** four instructions
        // later, in a block gated on `user+0x544a` - which is why 198 of 198 captured hits
        // claimed `1`. `crates/grap-stub/src/hitnumber.rs` clears that byte, and the client
        // then keeps what it worked out.
        //
        // So the rule: **a claim above the floor is the client's own arithmetic and we apply
        // it.** The number it draws and the number the HP bar moves by are then the same
        // number, by construction rather than by our two models happening to agree.
        //
        // `1` is the floor the discarded path produces, so it means "the patch is not on, or
        // the mob's attack power at `mob+0xe8` was zero and the client bailed before
        // computing". Then the server's own model stands in, exactly as it did before - which
        // is what keeps an unpatched client behaving the way it always has.
        let client_computed_it = claimed > 1;
        let applied = if client_computed_it { claimed } else { computed.unwrap_or(claimed) };

        // ---------------------------------------------------------------------------------
        // **The held guards, in the order they take their share.** Second- and third-job
        // audit, 2026-09-07. Each reads the value the client was told for its bit, so the
        // number the server acts on is the number the icon promised.
        // ---------------------------------------------------------------------------------
        // Invincible: `x`% of physical damage ignored outright. **[L]** *"Physical damage -10%"*.
        let invincible = u64::try_from(self.held_value(net::jobbuffs::CTS_INVINCIBLE)).unwrap_or(0);
        let after_invincible =
            applied - u32::try_from(u64::from(applied) * invincible.min(100) / 100).unwrap_or(0);
        // Power Guard: `x`% of what is left is not taken and goes back to the mob, below,
        // once the character's own bars are settled. **[L]** *"return 20% of the physical
        // damage received"*.
        let power_guard = u64::try_from(self.held_value(net::jobbuffs::CTS_POWER_GUARD)).unwrap_or(0);
        let reflected =
            u32::try_from(u64::from(after_invincible) * power_guard.min(100) / 100).unwrap_or(0);
        let after_power_guard = after_invincible - reflected;
        // Meso Guard: `x`% of what is left is paid in mesos at `y`% of the blocked amount -
        // *"Blocks 30% of incoming damage using Meso; Consumes Meso equal to 50% of the
        // blocked damage"*. Short of the price, nothing is blocked: the client's own gate is
        // the same (it drops the toggle at zero mesos), and a partial block would be a number
        // neither end agreed on.
        let mut meso_after: Option<u32> = None;
        let meso_guard = u64::try_from(self.held_value(net::jobbuffs::CTS_MESO_GUARD)).unwrap_or(0);
        let applied = if meso_guard > 0 {
            let blocked = u32::try_from(u64::from(after_power_guard) * meso_guard.min(100) / 100).unwrap_or(0);
            let y = self
                .store
                .skill_level(chr.id, crate::advbuffs::MESO_GUARD)
                .ok()
                .and_then(|l| self.config.firstjob.level(crate::advbuffs::MESO_GUARD, l))
                .and_then(|r| r.y)
                .and_then(|y| u64::try_from(y).ok())
                .unwrap_or(50);
            let price = u32::try_from(u64::from(blocked) * y / 100).unwrap_or(u32::MAX);
            let have = self.store.mesos(chr.id).unwrap_or(0);
            if blocked > 0 && have >= price {
                match self.store.add_mesos(chr.id, -i64::from(price)) {
                    Ok(left) => {
                        meso_after = Some(left);
                        after_power_guard - blocked
                    }
                    Err(_) => after_power_guard,
                }
            } else {
                after_power_guard
            }
        } else {
            after_power_guard
        };

        // **Magic Guard sends part of the damage to MP, and that split is the SERVER's job.**
        //
        // `research/magic-damage.md`: the client computes `mpLoss = guard% * damage / 100`
        // clamped to current MP - the hit handler reads `secStat+0x614`, multiplies, divides
        // by 100 and clamps - but it **never writes HP**. `user-hit.md` enumerated every write
        // to the HP field, closed the `lea`-handoff blind spot, and walked the hit path to
        // depth 8 without finding one. Only `0x007C` moves either bar.
        //
        // So setting CTS bit 97 buys an icon and nothing else. Without this block the buff
        // would look active, cost MP to cast, and change nothing about how much a hit hurt -
        // which is the shape of bug that gets reported as "the skill does nothing".
        //
        // **Clamped to current MP**, like the client's own arithmetic: what MP cannot absorb
        // still comes off HP. A player at 0 MP with Magic Guard up takes the full hit, which
        // is the behaviour the clamp in the client produces too.
        let guard_percent = self.magic_guard_percent();
        let to_mp = if guard_percent > 0 {
            (u64::from(applied) * u64::from(guard_percent) / 100).min(u64::from(chr.mp)) as u32
        } else {
            0
        };
        let to_hp = applied - to_mp;

        let before = chr.hp;
        chr.mp = chr.mp.saturating_sub(to_mp);
        chr.hp = chr.hp.saturating_sub(to_hp);
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not save your health: {e}"));
        }

        // **HP and MP in ONE packet.** `hp_only` was right until Magic Guard existed; sending
        // two packets would let the client draw the HP bar against a stale MP value, and the
        // revive dialog below gates on the HP this packet sets.
        // **Everyone else sees it too.** Before this the hurt player flinched only on their
        // own screen: 0x007C goes to them alone and nothing went to the field.
        self.publish_user_hit(&hit, payload, applied);


        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                hp: Some(chr.hp),
                mp: (to_mp > 0).then_some(chr.mp),
                meso: meso_after.map(u64::from),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: hit by mob {} (template {}, attack index {}) for {applied} - hp {before} -> {}{}. The CLIENT claimed {claimed}{}. Bit 10 carries the NEW HP, not a delta.",
                hit.mob_object_id,
                hit.mob_template_id,
                hit.attack_index,
                chr.hp,
                if to_mp > 0 {
                    format!(", and MAGIC GUARD sent {to_mp} of it to MP ({guard_percent}%) -> {} mp", chr.mp)
                } else {
                    String::new()
                },
                match computed {
                    // **The client did its own arithmetic and we took it.** This is the state
                    // worth reaching: one number, drawn by the client and applied by us. Our
                    // model is still run and still printed, as a check rather than an
                    // authority - if the two drift far apart that is worth seeing.
                    Some(c) if client_computed_it => format!(
                        " and the server APPLIED IT (our own model says {c}); the number drawn \
                         and the number applied are now the same"
                    ),
                    Some(_) if claimed != applied => {
                        " and the server overrode it - a claim of 1 is the floor the discarded \
                         path produces, so either hitnumber=off is not armed or the mob's \
                         attack power at mob+0xe8 is zero"
                    }
                    .to_string(),
                    Some(_) => " and the server agreed".to_string(),
                    None => " and the server had no template to check it against".to_string(),
                }
            ),
        }];
        // **Power Guard's reflection lands on the mob that hit us**, through the same body a
        // swing uses, so the bar and the death reach every viewer the same way. Not a swing:
        // no controller handover, no arrows, no combo orb.
        if reflected > 0 {
            let map = self.field_of(&chr);
            out.extend(self.deal_to_mob(map, hit.mob_object_id, u64::from(reflected), chr.id));
            crate::server::log(&format!(
                "   power guard: {reflected} of the hit went back to mob {} ({power_guard}%)",
                hit.mob_object_id
            ));
        }

        // **Death.** The owner, 2026-08-21: *"My HP hit 0, I see the tombstone on my character,
        // but I do not see the revive confirmation."*
        //
        // The tombstone was already working: `hp = 0` in the `0x007C` above disables the
        // player through the ~65 sites `research/user-hit.md` §6.2 enumerates. What was
        // missing is the dialog, and **the client never opens it by itself** - a packet does,
        // `0x0315`, and `research/revive.md` traced it through a vtable slot with exactly one
        // construction site.
        //
        // **Order matters and it is not cosmetic.** The handler gates on a client-side HP
        // test of the value the server just wrote (`world[0x2358]`, the field `0x007C` bit 10
        // sets), so a `0x0315` that overtakes the `0x007C` is dropped **silently**. It goes
        // after, in the same batch.
        //
        // **Gated on the transition, not on the state.** `before > 0 && chr.hp == 0` fires
        // once; `chr.hp == 0` alone would re-open the dialog on every subsequent hit, and a
        // dead character can still be hit. That is the same rule the quest payouts had to
        // learn: every effect hangs off the transition.
        if before > 0 && chr.hp == 0 {
            out.push(Reply {
                opcode: net::revive::SHOW_REVIVE_DIALOG,
                body: net::revive::show_revive_dialog(),
                what: format!(
                    "ShowReviveDialog: character {} died to mob {} (template {}). 0x0315, and it MUST follow the 0x007C above - the client gates it on its own copy of the HP the server just set, and drops it silently if that is still positive. Clicking REVIVE IN TOWN sends 0x00D1 with targetField 0",
                    chr.id, hit.mob_object_id, hit.mob_template_id
                ),
            });
        }

        // **The damage number the player actually took.**
        //
        // The owner, 2026-08-27: *"When I died to the Drakes, I still visually took 1 damage, but
        // it wiped out my whole HP bar."* They are right, and this is the packet the comment
        // above asked for when it said the disagreement "is the measurement the next run is
        // for".
        //
        // # The client's own number is a stub, and that is now measured rather than suspected
        //
        // 224 `0x00E5` bodies across 22 capture files, decoded with a passing control: mob
        // templates whose `PADamage` runs from **3 to 287** - a 96x spread - every one of them
        // reporting **damage = 1**. There is an explicit `max(damage, 1)` floor at
        // `0x1428AB959` and all sixteen mob-family call sites hand it `nDamage = 0`. The
        // client is not computing a number we could correct; it never had one.
        //
        // # The colour is the SIGN, and `recovery_number` already carries it
        //
        // `FUN_142771360` forks at `0x142771395`: a positive amount picks digit set 2
        // (`NoBlue`), a negative one picks set 3 (`NoViolet`) - and set 3 is what the client's
        // own hit builder reaches with a non-positive argument at `0x1428ACA14`. So the blue
        // recovery number and the damage number are not merely the same renderer by analogy;
        // they are the same call with the sign flipped. `net::revive::recovery_number` already
        // preserves the sign, and its own test says why.
        //
        // Everything upstream of the fork is proven by the owner seeing the blue `+10`.
        //
        // # THIS DOES NOT DELETE THE CLIENT'S `1`, AND THAT IS THE COST
        //
        // The stub number is drawn at *send* time, before this packet exists, so the screen
        // will show **both**. The flag that suppresses it, `user+0x544a`, has exactly one
        // writer and it is in a function with no packet reads - no server route was found,
        // and the search that failed named its own blind spots.
        //
        // # AND IT IS GONE, 2026-08-29 - the prediction above came true
        //
        // That paragraph used to end: *"Two numbers is worse than one correct number and
        // better than one wrong one, so it goes in and the run says whether it reads
        // acceptably. It is one push to remove. The only route to a single correct number is
        // `forcedStatPresent` at offset 10 of the spawn body - sent as 0 today, never tried."*
        //
        // It was tried. `MobForcedStat` now rides every spawn, so the client is told the
        // mob's attack power and computes a real number instead of the `max(damage, 1)` stub.
        // The owner, 2026-08-29: *"Latest client run was able to confirm that the mob damage
        // numbers are now agreeing with each other. No need to show the number twice."*
        //
        // So the client's number is correct on its own and this packet is the redundant one.
        // Removing OURS rather than the client's is the only choice available - the flag that
        // suppresses the client's, `user+0x544a`, has exactly one writer and it is in a
        // function with no packet reads, so no server route to it was ever found.
        //
        // **The server still computes and applies the damage**; `applied` above is what the HP
        // bar moves by, and the log line still carries both numbers. What is gone is only the
        // second thing drawn on screen. If the two ever disagree again, the log says so
        // without a packet, and this is four lines to put back.
        out
    }


    /// What a hit from `template` should actually take off, or `None` if we cannot say.
    ///
    /// `None` means the template is not in `gm-handbook/mobtemplates.txt` or carries no
    /// attack column, and the caller then keeps the client's number. That is the safe
    /// direction: a mob we have no data for should still hurt.
    ///
    /// # The defence is the player's worn `incPDD`, summed
    ///
    /// It is read off the equipped items rather than stored on the character, for the same
    /// reason [`Session::dressed`] resolves stats from the template: `crates/store` keeps
    /// `(slot, itemId)` and a defence column would be a second source of truth for a number
    /// the client already has its own copy of.
    ///
    /// At this scale defence barely matters and that is worth knowing before reading a run:
    /// 18 points of `incPDD` against a denominator of `5 * (level + 40)` is noise, so a
    /// naked character and a dressed one take the same from a snail. The term earns its keep
    /// later, not now.
    fn incoming_damage_for(
        &mut self,
        template: u32,
        chr: &net::opcode::Character,
    ) -> Option<u32> {
        let pa_damage = self.config.mob_attack.get(&template).copied().unwrap_or(0);
        if pa_damage == 0 {
            return None;
        }
        // **Equipment, the STR seed, and any held defence buff.**
        //
        // The owner, 2026-08-28: *"Iron Body did not seem to reduce the damage I take."* It could
        // not have: this summed equipment `inc_pdd` and stopped there, so a buff that set CTS
        // bit 86, drew an icon and cost MP was invisible to the one calculation it exists to
        // change. Same shape as the Magic Guard bug - the bit buys the icon, the arithmetic
        // is the server's.
        //
        // The `floor(STR/4)` seed goes in for the same reason: `Session::weapon_defence`
        // already documents that the stat window shows both, and a Warrior's STR is not a
        // rounding error.
        let wdef = u32::try_from(self.weapon_defence(chr)).unwrap_or(0)
            + self.held_weapon_defence();
        // A roll in [1.1, 1.5), the half-open span `damage::incoming_window` pins both ends
        // of. Taken from the session rng so a run is varied and a test can seed it.
        let span = crate::damage::INCOMING_ROLL_SPAN;
        let roll = crate::damage::INCOMING_ROLL_LO
            + (self.rng.next() % 10_000) as f64 * span / 10_000.0;
        Some(crate::damage::incoming_damage(pa_damage, chr.level, wdef, roll))
    }


    /// Count a kill against every started quest that asked for that mob.
    ///
    /// The owner: *"I accepted Sam's suggestion which requires Snail kills, but the quest is not
    /// progressing even when I kill snails."* Accepting was recorded; nothing counted.
    ///
    /// # The count is a string, and that is not a stylistic choice
    ///
    /// The client stores a quest's progress as **three zero-padded decimal characters per
    /// mob requirement**, concatenated in slot order - four snails is the three bytes
    /// `"004"`. `FUN_14070cb70` takes `substr(slot*3, slot*3+3)` and converts base 10. A
    /// count sent as an integer would render as nothing at all.
    ///
    /// # A kill fans out
    ///
    /// One template can be named by several quests - template 13 by six of them - so this
    /// cannot stop at the first match.
    ///
    /// # Item quests are not counted here, deliberately
    ///
    /// The client counts the bag live whenever it checks, so an item quest needs no running
    /// total and gets no packet. Only mob requirements have a counter, which is why the
    /// progress string's length is measured in mob slots.
    /// A kill credits the killer's own kill-quests, **and every party member on the same
    /// field who needs that mob**.
    ///
    /// The owner, 2026-09-05: *"Monster quests killed in a party should have their quest
    /// progression shared amongst the party members (only if party members need a particular
    /// mob killed, so any party member contributing to one person's kill count quest is
    /// allowed.)"* Each member advances **their own** quest row: the per-character crediting
    /// only touches a quest that that character has in progress and that needs this template,
    /// so a member with no such quest is untouched. The killer's records come back as replies
    /// on this connection; every other member's are delivered to them over the bus.
    pub(super) fn credit_kill_to_quests(&mut self, template: u32, killer: u32) -> Vec<Reply> {
        if self.config.quest_reqs.quests_for_mob(template).is_empty() {
            return Vec::new();
        }
        let out = self.credit_kill_to_quests_for(template, killer);

        // Fan out to party members standing on the same field. Solo, or a party alone on the
        // map, adds nothing here and the killer's own credit above is the whole of it.
        //
        // The roster is read into a local so the parties lock is not held across the mutable
        // per-member crediting below - the same guard-lifetime trap `CLAUDE.md` warns of.
        let roster = self.fields.parties().party_of(killer).map(|p| p.members.clone());
        if let Some(map) = self.claimed_character().map(|c| self.field_of(&c)) {
            if let Some(members) = roster {
                let others: Vec<u32> = members.into_iter().filter(|&m| m != killer).collect();
                for member in self.bus().characters_on(map, &others) {
                    for reply in self.credit_kill_to_quests_for(template, member) {
                        // Delivered to that member's own connection; their client draws the
                        // updated counter. A member who raced off the map is logged by
                        // publish returning false, not crashed.
                        if !self.bus().publish_to_character(member, map, reply) {
                            crate::server::log(&format!(
                                "   quest: party member {member} advanced a kill-quest on map \
                                 {map} but was gone before the record could be delivered"
                            ));
                        }
                    }
                }
            }
        }
        out
    }

    /// Credit one character's own kill-quests for `template`, returning the `0x0A0F` records.
    /// No party logic - the fan-out is [`Session::credit_kill_to_quests`].
    fn credit_kill_to_quests_for(&mut self, template: u32, character_id: u32) -> Vec<Reply> {
        let quests = self.config.quest_reqs.quests_for_mob(template);
        if quests.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::new();
        for &quest_id in quests {
            let Some(reqs) = self.config.quest_reqs.get(quest_id) else { continue };
            // Only a quest the player actually has. `quest_row` returns the in-progress row
            // or nothing, so a completed or unaccepted quest counts nothing.
            let Ok(Some(row)) = self.store.quest_row(character_id, quest_id) else { continue };
            if row.state != store::QuestState::InProgress {
                continue;
            }
            let Some(next) = net::quest::apply_kill(reqs, &row.progress, template) else {
                continue; // no requirement on this template, or the counter is already full
            };
            if self.store.set_quest_progress(character_id, quest_id, &next).is_err() {
                continue;
            }
            out.push(Reply {
                opcode: net::quest::MESSAGE,
                body: net::quest::quest_record(
                    quest_id,
                    &net::quest::QuestProgress::InProgress { progress: next.clone() },
                ),
                what: format!(
                    "QuestRecord: quest {quest_id} progress {next:?} after killing template                      {template}. Three zero-padded characters per mob slot - an integer here                      renders as nothing."
                ),
            });
        }
        out
    }
}
