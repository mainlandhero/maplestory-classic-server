//! Using a consumable: `0x010E` in, HP/MP and a smaller stack out.
//!
//! The owner, 2026-08-21: *"Using a consumable item such as Roger's Apple does not recover HP. I
//! tried to consume Red Potion, but it did not recover 100 HP. (Or less if it will fill my
//! HP bar up to full)"* — the parenthesis is the cap, and it is implemented.
//!
//! **Every path answers.** There is exactly one `0x010E` in the whole capture even though
//! The owner used more than one item, which is the signature of a client-side request latch —
//! the same one `0x0107` and both ability-point requests have. A refusal that sends nothing
//! would cost not one potion but every later use for the rest of the session.

use super::*;

impl Session {
    /// `0x010E` — the player double-clicked something in the Use tab.
    /// `0x0123`, the client's own opcode for a Return Scroll (`net::useitem::
    /// CLIENT_USE_RETURN_SCROLL`). The body is `0x010E`'s and so is the latch, so this is the
    /// same walk: slot checked against the bag, [`Session::use_return_scroll`] decides, and
    /// every refusal still answers with the unlock. A non-scroll id arriving here is refused
    /// the way an unknown item on `0x010E` is.
    pub(super) fn on_use_return_scroll(&mut self, payload: &[u8]) -> Vec<Reply> {
        crate::server::log("   return scroll: 0x0123 - the client's opcode for the 0203 range");
        // **Its own parser, and this is the whole bug of 2026-09-18..21.** This forwarded
        // into `on_use_item`, whose parser wants fourteen bytes; `0x0123` is ten, so every
        // scroll the client actually sent came back "the 0x010E body did not parse" - which
        // is the line the owner read on screen. See `net::useitem::parse_use_return_scroll`.
        let Some(req) = net::useitem::parse_use_return_scroll(payload) else {
            return self.use_refused(format!("the 0x0123 body did not parse ({} bytes)", payload.len()));
        };
        self.use_parsed_item(req)
    }

    /// `0x0206` - a pet with Auto HP or Auto MP drinking one of its owner's potions
    /// (`net::useitem::CLIENT_PET_USE_ITEM`). **The same walk as a double-click**: the slot is
    /// checked against the bag, the potion's restore is capped at what is missing, the stack
    /// shrinks, and every path answers - the builder sets the same latch `0x010E` does.
    ///
    /// The owner, 2026-09-25: the pet tried, and *"potions are never consumed and clients never
    /// recover"*. This opcode had no handler at all; the deployed server logged 25 of them as
    /// UNKNOWN and answered none.
    ///
    /// Only a potion: Auto HP and Auto MP drink nothing else, so anything that is not one - a
    /// Return Scroll above all, which would move the player - is refused rather than walked.
    pub(super) fn on_pet_use_item(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some((pet, req)) = net::useitem::parse_pet_use_item(payload) else {
            return self.use_refused(format!("the 0x0206 body did not parse ({} bytes)", payload.len()));
        };
        if self.config.consumables.get(req.item_id).is_none_or(|r| r.is_nothing()) {
            return self.use_refused(format!("pet {pet} offered item {}, which is not a potion", req.item_id));
        }
        crate::server::log(&format!("   pet {pet} drinks {} from Use slot {} for its owner", req.item_id, req.slot));
        self.use_parsed_item(req)
    }

    pub(super) fn on_use_item(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(req) = net::useitem::parse_use_item(payload) else {
            return self.use_refused("the 0x010E body did not parse".to_string());
        };
        self.use_parsed_item(req)
    }

    /// The walk both opcodes share, from an already-parsed request. The two differ only in
    /// the bytes on the wire.
    fn use_parsed_item(&mut self, req: net::useitem::UseItem) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else {
            return self.use_refused("no character is claimed on this connection".to_string());
        };
        let Ok(slot) = u16::try_from(req.slot) else {
            return self.use_refused(format!("slot {} is not a bag slot", req.slot));
        };

        // **The bag is the server's, so the slot is checked rather than trusted.** The
        // client names the item it believes is there; if the two disagree the server's copy
        // wins, because it is the one that persisted.
        let inv = store::InventoryType::Use;
        let held = self
            .store
            .bag_items(chr.id, inv)
            .ok()
            .and_then(|rows| rows.into_iter().find(|i| i.slot == slot));
        let Some(row) = held else {
            return self.use_refused(format!("Use slot {slot} is empty"));
        };
        if row.item.item_id != req.item_id {
            return self.use_refused(format!(
                "Use slot {slot} holds {} and the client asked to use {}",
                row.item.item_id, req.item_id
            ));
        }
        // How many are in the slot **before** this use. Read from the row that was just
        // checked rather than re-queried, for the same reason the potion path does it:
        // `remove_item` returns what it took, not what remains.
        let held = row.item.kind.quantity();

        // **A Return Scroll is checked before the potion table, and it is not a potion.**
        // Until 2026-08-29 this fell straight through to the refusal below - the comment
        // there named "a return scroll" as one of the things nothing here knew how to do.
        // [`Session::use_return_scroll`] answers `None` for anything that is not one of the
        // ten, so the potion path is unchanged for every other item.
        if let Some(out) = self.use_return_scroll(&mut chr, req.item_id, inv, slot, held) {
            return out;
        }

        let Some(restores) = self.config.consumables.get(req.item_id) else {
            // A real item that simply is not a potion - an upgrade scroll, a summoning sack.
            // Nothing here knows what those do, and pretending otherwise would take the item
            // away for no effect.
            //
            // **Return scrolls are no longer in that list** - they are handled above, since
            // 2026-08-29. This comment used to name them and it is worth saying they left,
            // because "the item is refused" is what a stale branch and a missing feature look
            // like from the same seat.
            return self.use_refused(format!(
                "item {} restores nothing this server knows about",
                req.item_id
            ));
        };
        if restores.is_nothing() {
            return self.use_refused(format!("item {} restores nothing", req.item_id));
        }

        // **Cap at what is actually missing.** The owner asked for this in the same sentence, and
        // it is also the only reading that cannot put a number above the maximum into the HP
        // field - which the client would then draw as a bar past its own end.
        // **Improved HP/MP Recovery, at last.** The owner, 2026-08-30: *"one of the passive for
        // 'Improved MP Recovery' says it will increase MP recovery item recovery amount by
        // 20%, it currently does not do that."*
        //
        // It did not because nothing on this path had ever asked what skills the character
        // had. The bug is measured rather than reasoned: character 213 holds `2000000` at
        // level 15 in `maplecw.db`, and `previous-runs/world-20260830-221224.log` - the same
        // day as the report - shows `used item 2000003 ... +200 mp` eleven times. 2000003 is
        // 200 flat MP; 20% of it is the 40 that never arrived.
        //
        // **The bonus goes in BEFORE the cap**, which is the whole reason this is one call
        // and not two: bonusing a value that has already been clamped to what is missing
        // would let a near-full drink overshoot the maximum, and the client draws a bar past
        // its own end. `restored` folds each skill's `y` into its own pool; `capped` then does
        // what the two lines here used to do.
        //
        // `Store::skill_level` returns `Ok(0)` for a skill the character never raised, so
        // `unwrap_or(0)` is reached only on a real database failure - and there it degrades to
        // the old behaviour rather than refusing the item. This path is latched: a refusal
        // costs not one potion but every later use in the session.
        let learned = crate::itemrecovery::Learned {
            improved_hp_recovery: self
                .store
                .skill_level(chr.id, crate::itemrecovery::IMPROVED_HP_RECOVERY)
                .unwrap_or(0),
            improved_mp_recovery: self
                .store
                .skill_level(chr.id, crate::itemrecovery::IMPROVED_MP_RECOVERY)
                .unwrap_or(0),
        };
        // **Every maximum here is the one the client draws** - base plus a learned Max HP/MP
        // Increase (`Session::pools`) - both for a percent-of-max potion and for the cap.
        // Against the base, a Max HP Increase character drinking at 400/447 was CUT to 358:
        // the `.min` below is a clamp, and a clamp to the wrong ceiling takes health away.
        let pools = self.pools(&chr);
        let gained = crate::itemrecovery::restored(&restores, pools.max_hp, pools.max_mp, learned);
        let (hp_gain, mp_gain) = gained.capped(chr.hp, pools.max_hp, chr.mp, pools.max_mp);
        chr.hp = chr.hp.saturating_add(hp_gain).min(pools.max_hp.max(chr.hp));
        chr.mp = chr.mp.saturating_add(mp_gain).min(pools.max_mp.max(chr.mp));

        // The item is consumed whether or not it healed anything: drinking a potion at full
        // health still drinks the potion, and refusing here would let a player at full HP
        // hold an infinite stack. That is a rule rather than an observation, and it is the
        // same one every version of this game has.
        // What is left AFTER this one is drunk.
        let left = held.saturating_sub(1);
        if let Err(e) = self.store.remove_item(chr.id, inv, slot, Some(1)) {
            return self.use_refused(format!("could not consume the item: {e}"));
        }
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.use_refused(format!("could not save your health: {e}"));
        }

        // **No recovery trailer here, and that is deliberate.** The owner, 2026-08-21: *"Potion
        // recovery should not trigger the recovery number, that's only for idle regeneration
        // standing or sitting in a chair in the Set-up tab or sitting on a chair in a map."*
        //
        // `0x007C`'s second optional trailer draws a number over the player's head, and I had
        // reasoned from the packet outwards - a potion recovers, the field is called
        // recovery, so send it. That is the wrong direction. The field is not "something was
        // restored", it is **the regeneration indicator**, and which events are allowed to
        // raise it is a property of the game rather than of the encoding. A potion moves the
        // bar and says nothing, exactly as it did before.
        //
        // The trailer lives in `regen.rs` and belongs to idle recovery alone. Chairs are the
        // other case the owner named and this server has no chairs yet; when it does, that path
        // sends it too and this one still does not.
        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                hp: Some(chr.hp),
                mp: Some(chr.mp),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: used item {} from Use slot {slot} - +{hp_gain} hp (now {}/{}), +{mp_gain} mp (now {}/{}){}. No recovery trailer: that number is for idle regeneration and chairs, not for items. Byte 0 also clears the client's request latch",
                req.item_id, chr.hp, chr.max_hp, chr.mp, chr.max_mp,
                // **The only place a run can show the bonus fired.** Without it a capture
                // cannot tell "the skill applied 0%" from "the wiring is absent", which is
                // exactly the confusion that left this open for a week.
                if gained.any_bonus_applied() {
                    format!(
                        " [Improved Recovery: hp {} -> {} (+{}%), mp {} -> {} (+{}%), applied BEFORE the missing-health cap]",
                        gained.hp_base,
                        gained.hp,
                        gained.hp_bonus_percent,
                        gained.mp_base,
                        gained.mp,
                        gained.mp_bonus_percent
                    )
                } else {
                    String::new()
                }
            ),
        }];
        out.extend(self.stack_change_replies(inv, slot, left));
        out.extend(self.buff_from_item(req.item_id, &restores, chr.id, self.field_of(&chr)));
        // **Eating the thing can be the turn-in.** The owner, 2026-08-21: *"Once the user
        // consumes the apple, the quest would be completed."*
        out.extend(self.quests_completed_by_consuming(req.item_id));
        out
    }

    /// The **timed stats** a consumable grants, if it grants any.
    ///
    /// The owner, 2026-09-09: *"Drinking the Dexterity Potion or the Magic Potion also does not
    /// give me the proper buff and subtract the item by 1."* Both do now, and the reason
    /// neither did is that this table only ever carried HP and MP - so the server's refusal,
    /// *"item 2002003 restores nothing this server knows about"*, was true and useless. It
    /// restores nothing. It buffs `eva 5` for ten minutes.
    ///
    /// # The GM's Blessings reach the whole map
    ///
    /// The owner, same day: *"Using either item should trigger an effect for all players on a map
    /// saying who is the person that gave the blessing, and give everyone on the map the
    /// appropriate buff."*
    ///
    /// `0x007D` is *"your* temporary stats changed", so the same bytes handed to every client
    /// on the map buff every one of them - there is no separate "remote" form to build. The
    /// announcement is a chat notice carrying the giver's name, because the client has no
    /// string of its own for a thing this game does not have.
    ///
    /// **`Bus::publish` skips the publisher**, so the giver's own copy is in the returned
    /// replies and the map's copy goes through the bus - the same two-send shape the scroll
    /// effect uses, for the same reason.
    fn buff_from_item(
        &mut self,
        item_id: u32,
        restores: &crate::consumables::Restores,
        _character_id: u32,
        map: crate::fields::FieldKey,
    ) -> Vec<Reply> {
        // **Every stat an item grants names the item as a NEGATIVE reason.** The owner,
        // 2026-09-16: *"Magic Potions and other similar potions are not applying the buff
        // icons."* They were sent with `reason: item_id` - positive, which the client reads
        // as a skill id, and there is no skill 2002001 to draw. `net::buff::item_reason`.
        let mut out = Vec::new();
        let mut stats: Vec<net::buff::TemporaryStat> = restores
            .buffs()
            .iter()
            .map(|&(bit, value)| net::buff::TemporaryStat {
                bit,
                value: i16::try_from(value).unwrap_or(i16::MAX),
                reason: net::buff::item_reason(item_id),
                duration_ms: restores.duration_ms,
            })
            .collect();

        // **The EXP coupon, which is a rate rather than a stat.** The owner: *"are we sure that
        // the EXP gained is actually properly being modified?"* It was not - this field was
        // read by nothing, so the coupon was consumed and did nothing at all.
        //
        // The multiplier is the server's (`with_exp_coupon`); the client is told on CTS 163
        // `ExpBuffRate` so the icon and its countdown appear top-right. The owner, 2026-09-16:
        // *"the EXP coupon effects are not applying the appropriate buff icon ... make sure
        // 2x and 3x coupons have the proper buff durations applied"*. Until then the coupon
        // sent no `0x007D` at all - it had no bit - so there was nothing to draw.
        if restores.exp_percent > 0 && restores.duration_ms > 0 {
            let coupon = crate::consumables::ExpCoupon {
                percent: restores.exp_percent,
                expires_ms: self.clock_ms.saturating_add(u64::from(restores.duration_ms)),
                item_id,
            };
            // A second coupon REPLACES the first rather than stacking or being refused.
            // Stacking would make two 2x coupons a 4x, which no version of this game does,
            // and refusing would eat the item - the worst of the three.
            self.exp_coupon = Some(coupon);
            stats.push(net::buff::TemporaryStat {
                bit: net::buff::CTS_EXP_BUFF_RATE,
                value: i16::try_from(restores.exp_percent).unwrap_or(i16::MAX),
                reason: net::buff::item_reason(item_id),
                duration_ms: restores.duration_ms,
            });
            let line = format!(
                "{} experience for {} minutes.",
                coupon.label(),
                restores.duration_ms / 60_000
            );
            out.push(Reply {
                opcode: net::notice::CHAT_NOTICE,
                body: net::notice::chat_notice(&line),
                what: format!(
                    "ChatNotice: item {item_id} starts a {} EXP coupon, {} ms. Applied to KILL \
                     experience only - not to !exp and not to quest turn-ins, which is where \
                     the existing rate draws the same line.",
                    coupon.label(),
                    restores.duration_ms
                ),
            });
        }

        // A restore-only potion takes this path too and must leave with nothing: an empty
        // `0x007D` would set a mask with no bits and is not worth sending.
        if stats.is_empty() || restores.duration_ms == 0 {
            if !restores.unsupported().is_empty() {
                crate::server::log(&format!(
                    "   item {item_id}: {} cannot be sent - no measured CTS bit for it in this \
                     repo, and guessing one sends a number to an unknown stat. The rest of the \
                     item still applied.",
                    restores.unsupported().join(", ")
                ));
            }
            return out;
        }
        // Remember them the same way a skill's are, so the tick expires them and a re-drink
        // replaces rather than stacks. The coupon's bit 163 is in here too: the `0x007E` the
        // tick sends at `expires_ms` is what takes the icon down, and it is the same instant
        // `with_exp_coupon` stops multiplying.
        let expires_ms = self.clock_ms.saturating_add(u64::from(restores.duration_ms));
        for stat in &stats {
            self.buffs.retain(|b| b.bit != stat.bit);
            self.buffs.push(super::buff::ActiveBuff {
                bit: stat.bit,
                skill_id: item_id,
                expires_ms,
                value: stat.value,
                reason: stat.reason,
            });
        }
        let described: Vec<String> =
            stats.iter().map(|s| format!("CTS {} = {}", s.bit, s.value)).collect();
        let body = net::buff::temporary_stat_set_with_tail(&stats, net::buff::TAIL_LEN);
        let mine = Reply {
            opcode: net::buff::TEMPORARY_STAT_SET,
            body: body.clone(),
            what: format!(
                "TemporaryStatSet: item {item_id} grants {} for {} ms. Nothing authenticates.",
                described.join(", "),
                restores.duration_ms
            ),
        };
        if !restores.unsupported().is_empty() {
            crate::server::log(&format!(
                "   item {item_id}: granted {} but {} could not be sent - no measured CTS bit.",
                described.join(", "),
                restores.unsupported().join(", ")
            ));
        }
        out.push(mine);
        if crate::consumables::blesses_the_whole_map(item_id) {
            // Everyone else on the map applies it through their own session - recorded, iconed
            // and expired there - and the whole map gets the GM weather with the giver's name.
            // session/weather.rs.
            out.extend(self.bless_the_map(item_id, map));
        }
        out
    }


    /// A Return Scroll: teleport, or refuse and **keep the scroll**.
    ///
    /// `None` means "this is not one of the ten", and the caller carries on to the potion
    /// table. Every other answer is a complete set of replies.
    ///
    /// # Every effect hangs off the transition
    ///
    /// [`crate::returnscroll::resolve`] decides and performs nothing; consuming, warping and
    /// the stack reply all live inside the single `Teleport` arm below. That is
    /// `CLAUDE.md`'s Heena-quest rule applied before it can be broken rather than after:
    /// there is no arrangement of this function where the scroll is spent and the character
    /// stays put, because the removal and the warp are the same three lines.
    ///
    /// **A refusal consumes nothing.** Wrong continent, no return row, a destination this
    /// client has no field image for, an empty field table - all of them leave the stack
    /// exactly as it was and say why on screen. A scroll eaten by a refusal is the bug a
    /// player notices first.
    ///
    /// # The guard is the one `gm_map` has, and it is not weaker
    ///
    /// `session::gm_map` refuses a map with no field image because a bad id strands or kills
    /// the client - the owner lost a session to `!map 45` on 2026-08-20 - and it refuses *louder*
    /// when the field table is empty, because `Config::map_exists` is fail-open and would
    /// otherwise let every id through while looking checked. Both are here. The existence
    /// check is an argument of `resolve` rather than a step in it, so it cannot be skipped
    /// by a future caller.
    ///
    /// # The latch
    ///
    /// The success path sends an empty `StatChanged` before anything else, exactly as
    /// [`Session::use_refused`] does. Its `bExclRequestSent` clears the client's
    /// one-request-outstanding latch at `+0x2330`; the `InventoryOperation` that follows
    /// carries the same byte. Both are already-shipped packet shapes - this path invents
    /// none - and the reason for the belt as well as the braces is that a `0x010E` that
    /// leaves the latch set costs not one scroll but **every later item use in the
    /// session**.
    fn use_return_scroll(
        &mut self,
        chr: &mut net::opcode::Character,
        item_id: u32,
        inv: store::InventoryType,
        slot: u16,
        held: u16,
    ) -> Option<Vec<Reply>> {
        use crate::returnscroll::{self, Outcome};

        if !returnscroll::is_return_scroll(item_id) {
            return None;
        }
        // Said out loud rather than silently fail-open. `map_exists` answers `true` for
        // everything when the table is empty, so without this a missing
        // `gm-handbook/fields.txt` would turn the strand guard off while the log still
        // read as though it had run.
        if self.config.fields.is_empty() {
            return Some(self.use_refused(format!(
                "the field table is empty, so this server cannot check that the scroll's \
                 destination exists, and a bad map id kills the client. Item {item_id} was \
                 NOT consumed. Regenerate gm-handbook/fields.txt with tools/dump_portals.py, \
                 or restart the server so it loads"
            )));
        }

        // Cloned so the closure below borrows the table rather than `self`, which the
        // teleport needs mutably a few lines later.
        let cfg = self.config.clone();
        let from = chr.map_id;
        let anchor = cfg.revive_field(from);
        let (to, why) = match returnscroll::resolve(item_id, from, anchor, |m| cfg.map_exists(m)) {
            Outcome::Teleport { to, why } => (to, why),
            Outcome::Refused { why } => return Some(self.use_refused(why)),
            // Unreachable: `is_return_scroll` above already said it is one. Falling through
            // to the potion table is the only harmless reading of it.
            Outcome::NotAScroll => return None,
        };

        let left = held.saturating_sub(1);
        if let Err(e) = self.store.remove_item(chr.id, inv, slot, Some(1)) {
            return Some(self.use_refused(format!("could not consume the scroll: {e}")));
        }
        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange::default().build(),
            what: format!(
                "StatChanged: EMPTY - item {item_id} is a Return Scroll and moves no stat. Sent \
                 because byte 0 clears the client's 0x010E request latch"
            ),
        }];
        out.extend(self.stack_change_replies(inv, slot, left));
        // A teleport: a random spawn point of the town (the owner, 2026-09-26), like `!map`.
        out.extend(self.teleport(
            chr,
            to,
            format!("{why}, item {item_id} from Use slot {slot}"),
        ));
        Some(out)
    }


    /// Finish any started quest whose completion is *consuming* this item.
    ///
    /// `Check.<state>.consumeitem` is an authored key, and it states a rule the WZ only
    /// implies: quest 1002's `Check.1.item.0` carries an **id and no count** while 214 other
    /// quests carry one, and `research/quest-scripts.md` marked "no count means must-not-hold"
    /// **[I]** and left it open. The owner's sentence settles it.
    ///
    /// **Only a quest that is actually started counts.** Eating an apple you were never asked
    /// for finishes nothing, and eating one twice cannot finish it twice - `complete_quest`
    /// returns `Ok(None)` for a quest with no in-progress row and
    /// [`Session::record_quest_complete`] already declines to play a fanfare for that.
    fn quests_completed_by_consuming(&mut self, item_id: u32) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Ok(rows) = self.store.quest_rows(chr.id) else { return Vec::new() };
        let finished: Vec<u32> = rows
            .iter()
            // **In progress only.** `quest_rows` returns completed rows too, so without this
            // a second apple would re-complete a quest that is already finished - and play
            // its fanfare again.
            .filter(|r| r.state == store::QuestState::InProgress)
            .map(|r| r.quest_id)
            .filter(|id| {
                self.config
                    .quests
                    .get(id)
                    .and_then(|q| q.complete_on_consume)
                    .is_some_and(|want| want == item_id)
            })
            .collect();
        let mut out = Vec::new();
        for quest_id in finished {
            let next = self
                .config
                .quests
                .get(&quest_id)
                .and_then(|q| q.next_quest)
                .unwrap_or(quest_id);
            out.extend(self.record_quest_complete(quest_id, next));
        }
        out
    }

    /// Tell the client what the Use tab looks like now: one fewer, or the slot emptied.
    ///
    /// `pub(super)` because the arrow consumption in `session::combat` sends the same two
    /// shapes for the same reason, and a second copy is a second place to send mode 3 for a
    /// partial take.
    pub(super) fn stack_change_replies(
        &self,
        inv: store::InventoryType,
        slot: u16,
        left: u16,
    ) -> Vec<Reply> {
        if left == 0 {
            return vec![Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_removed(inv.as_u8() as i8, slot as i16),
                what: format!("InventoryOperation REMOVE: {inv:?} slot {slot} is empty now"),
            }];
        }
        vec![Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_quantity(inv.as_u8() as i8, slot as i16, left),
            what: format!("InventoryOperation QUANTITY: {inv:?} slot {slot} down to {left}"),
        }]
    }

    /// A refusal that still answers, and says why in the log.
    ///
    /// The notice goes to the chat window so the refusal is visible on screen rather than
    /// only in `world.log` - a use that silently does nothing is indistinguishable from a
    /// handler that never ran, and telling those apart has cost launches here.
    fn use_refused(&self, why: String) -> Vec<Reply> {
        let mut out = self.notice(format!("That item could not be used: {why}."));
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange::default().build(),
            what: format!("StatChanged: EMPTY - 0x010E refused ({why}). Sent anyway because byte 0 clears the client's request latch, and a silent refusal is what kills the next use"),
        });
        out
    }
}


/// Return Scrolls, end to end through `0x010E`.
///
/// `crate::returnscroll` proves the *decision*; these prove the **effects**, and there are
/// four of them on a success and four matching absences on a refusal. `CLAUDE.md`: a test
/// that checks one of several effects gives false confidence about the rest - the Heena
/// turn-in counted fanfares while the experience doubled beside it - so every test below
/// asserts on all four, or names the ones it is not covering.
///
/// | effect | on a success | on a refusal |
/// |---|---|---|
/// | the stack in the Use slot | one fewer | **unchanged** |
/// | `InventoryOperation` | mode 1, or mode 3 for the last one | **absent** |
/// | the character's stored map | the destination | **unchanged** |
/// | `SetField` | sent | **absent** |
///
/// Plus the one that is the same in both directions and is the most expensive to get wrong:
/// **something with `bExclRequestSent` always goes out**, or the client's `0x010E` latch
/// stays set and every later item use in the session is dropped.
#[cfg(test)]
mod scroll_tests {
    use super::*;

    /// A character standing on `on_map` with `count` of `item_id` in Use slot 1, and a
    /// config that knows the maps these tests warp between.
    ///
    /// The two tables are the real ones' shape, not the real ones' contents: `fields` is the
    /// `gm-handbook/fields.txt` set and `revive_maps` the `reviveMap` column of
    /// `returnmaps.txt`. Their real values for these ids are asserted by
    /// `returnscroll`'s own cross-check against the generated file, so hard-coding a handful
    /// here cannot make a test agree with a wrong table.
    fn session_with_scroll(item_id: u32, count: u16, on_map: u32) -> (Session, i64, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wanderer".to_string(), ..Default::default() };
        let made = store.create_character(account_id, 0, &chr).unwrap();
        store.set_character_map(made.id, on_map).unwrap();
        store.create_migration(account_id, made.id, 0, 0).unwrap();
        store
            .set_inventory_slot(
                made.id,
                store::InventoryType::Use,
                1,
                &store::Item::bundle(item_id, count),
            )
            .unwrap();
        let config = Config {
            // Map 40's anchor is Southperry; a Victoria field's is its own town. Both are
            // rows of the real generated table.
            revive_maps: [
                (40, 60),
                (60, 60),
                (10_001_090, 10_001_000),
                (10_001_000, 10_001_000),
                (20_001_000, 20_001_000),
            ]
            .into_iter()
            .collect(),
            fields: [40, 60, 10_000_000, 10_001_000, 10_001_090, 10_002_000, 20_000_000, 20_001_000]
                .into_iter()
                .collect(),
            consumables: crate::consumables::Consumables::parse("2000000, 100, 0, 0, 0\n"),
            ..Config::default()
        };
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(made.id);
        (s, account_id, made.id)
    }

    fn stored_map(s: &Session, account: i64, id: u32) -> u32 {
        s.store
            .characters_for(account, 0)
            .unwrap()
            .into_iter()
            .find(|c| c.id == id)
            .expect("the character is still there")
            .map_id
    }

    fn held(s: &Session, id: u32) -> Option<u16> {
        s.store
            .inventory_slot(id, store::InventoryType::Use, 1)
            .unwrap()
            .map(|i| i.kind.quantity())
    }

    fn has(rs: &[Reply], opcode: u16) -> bool {
        rs.iter().any(|r| r.opcode == opcode)
    }

    /// **The success, all four effects.** The owner: *"Return Scroll to Nearest Town should adhere
    /// to the map's return map and teleport the player there."* Map 40's `reviveMap` is 60,
    /// Southperry.
    #[test]
    fn the_nearest_town_scroll_warps_to_the_maps_own_anchor_and_costs_one() {
        let (mut s, acct, id) = session_with_scroll(2_030_000, 2, 40);
        assert_eq!(stored_map(&s, acct, id), 40);

        let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_030_000, 1));

        // 1. the character moved, in the DATABASE and not only in a reply
        assert_eq!(stored_map(&s, acct, id), 60, "map 40 anchors to Southperry");
        // 2. and the client was told, with a SetField naming it
        let sf = out
            .iter()
            .find(|r| r.opcode == net::opcode::SET_FIELD)
            .expect("a teleport always sends a SetField");
        assert!(sf.what.contains("anchors to 60"), "{}", sf.what);
        // 3. the scroll was paid for
        assert_eq!(held(&s, id), Some(1), "one of the two is gone");
        let op = out
            .iter()
            .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
            .expect("the Use tab is told the stack shrank");
        assert_eq!(
            op.body,
            net::inventory::inventory_quantity(store::InventoryType::Use.as_u8() as i8, 1, 1)
        );
        // 4. and the request latch is cleared, or nothing else works this session
        let stat = out
            .iter()
            .find(|r| r.opcode == net::stats::STAT_CHANGED)
            .expect("every 0x010E is answered");
        assert_eq!(stat.body[0], 1, "bExclRequestSent");
        assert_eq!(op.body[0], 1, "and the inventory reply carries it too");
    }

    /// **The owner's own example, and the four absences.** *"Return Scroll to El Nath should not
    /// be allowed on Victoria Island."*
    #[test]
    fn an_el_nath_scroll_on_victoria_island_is_refused_and_not_consumed() {
        let (mut s, acct, id) = session_with_scroll(2_030_009, 1, 10_001_090);

        let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_030_009, 1));

        assert_eq!(stored_map(&s, acct, id), 10_001_090, "the character did not move");
        assert!(!has(&out, net::opcode::SET_FIELD), "and no SetField was sent");
        assert_eq!(held(&s, id), Some(1), "the scroll is still in the bag");
        assert!(
            !has(&out, net::inventory::INVENTORY_OPERATION),
            "and nothing told the client otherwise"
        );

        // It still answers, and it says why on screen rather than only in world.log.
        let stat = out
            .iter()
            .find(|r| r.opcode == net::stats::STAT_CHANGED)
            .expect("a refusal answers or the next use never leaves the client");
        assert_eq!(stat.body[0], 1, "bExclRequestSent");
        let notice = out
            .iter()
            .find(|r| r.opcode == net::notice::CHAT_NOTICE)
            .expect("the refusal is visible on screen");
        let text = String::from_utf8_lossy(&notice.body).to_string();
        assert!(text.contains("Ossyria"), "{text}");
        assert!(text.contains("Victoria Island"), "{text}");
    }

    /// **The wire.** Every test above calls the handler; the client never sends `0x010E` for
    /// a scroll - it sends `0x0123` (`net::useitem::CLIENT_USE_RETURN_SCROLL`), and for three
    /// weeks nothing answered it. Through `handle`, with the opcode the client uses: Henesys
    /// from Victoria Island warps and consumes; El Nath from Victoria Island is refused, kept,
    /// answered with the unlock and a visible notice; Orbis from Ossyria warps.
    #[test]
    fn the_scroll_opcode_the_client_sends_reaches_the_handler() {
        let packet = |slot: i16, item: u32| {
            let mut p = net::useitem::CLIENT_USE_RETURN_SCROLL.to_le_bytes().to_vec();
            p.extend_from_slice(&net::useitem::use_item(0x1234, slot, item, 1));
            p
        };
        let (mut s, acct, id) = session_with_scroll(2_030_004, 2, 10_001_090);
        let out = s.handle(&packet(1, 2_030_004));
        assert_eq!(stored_map(&s, acct, id), 10_001_000, "Henesys, through 0x0123");
        assert!(has(&out, net::opcode::SET_FIELD));
        assert_eq!(held(&s, id), Some(1), "one of two used");

        let (mut s, acct, id) = session_with_scroll(2_030_009, 1, 10_001_090);
        let out = s.handle(&packet(1, 2_030_009));
        assert_eq!(stored_map(&s, acct, id), 10_001_090, "El Nath from Victoria: refused");
        assert!(!has(&out, net::opcode::SET_FIELD));
        assert_eq!(held(&s, id), Some(1), "kept");
        let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("0x0123 latches; the refusal unlocks");
        assert_eq!(stat.body[0], 1);
        assert!(has(&out, net::notice::CHAT_NOTICE), "and says why on screen");

        let (mut s, acct, id) = session_with_scroll(2_030_008, 1, 20_001_000);
        let out = s.handle(&packet(1, 2_030_008));
        assert_eq!(stored_map(&s, acct, id), 20_000_000, "Orbis from El Nath, both Ossyria");
        assert!(has(&out, net::opcode::SET_FIELD));
        assert_eq!(held(&s, id), None);
    }

    /// **The owner's Ellinia scroll, byte for byte off the wire.** 2026-09-21: *"I just tried to
    /// use the Return Scroll to Ellinia, but the item showed a message that says 0x010E body
    /// did not parse."*
    ///
    /// This drives `on_use_return_scroll` with the **ten-byte** body the client really sent -
    /// `world-ch0.log`, `cf80f90c 1600 b2f91e00` - rather than rebuilding it with
    /// `use_item`, which is a fourteen-byte shape this opcode never carries and is the reason
    /// every other test on this page passed while the feature was broken on screen.
    #[test]
    fn the_captured_ellinia_scroll_packet_teleports_and_is_consumed() {
        // slot 22 on the wire; the fixture puts the scroll in slot 1, so only the id and the
        // body's LENGTH are what this test is really about.
        let (mut s, acct, id) = session_with_scroll(2_030_002, 1, 10_001_090);
        let body = net::useitem::use_return_scroll(0x0cf9_80cf, 1, 2_030_002);
        assert_eq!(body.len(), 10, "the client's shape, not 0x010E's fourteen");

        let out = s.on_use_return_scroll(&body);

        assert_eq!(stored_map(&s, acct, id), 10_002_000, "Ellinia");
        assert!(has(&out, net::opcode::SET_FIELD), "it actually teleports: {:?}", out.iter().map(|r| r.what.clone()).collect::<Vec<_>>());
        assert_eq!(held(&s, id), None, "and the scroll is spent");
        assert!(
            !out.iter().any(|r| String::from_utf8_lossy(&r.body).contains("did not parse")),
            "no parse complaint reaches the screen"
        );
    }

    /// The same packet one byte short is still **answered** - this path is latched, so a
    /// silent refusal would cost every later inventory action in the session.
    #[test]
    fn a_truncated_return_scroll_packet_is_still_answered() {
        let (mut s, _acct, id) = session_with_scroll(2_030_002, 1, 10_001_090);
        let body = net::useitem::use_return_scroll(0, 1, 2_030_002);
        for n in 0..body.len() {
            let out = s.on_use_return_scroll(&body[..n]);
            assert!(!out.is_empty(), "len {n} must still answer");
        }
        assert_eq!(held(&s, id), Some(1), "and nothing was taken");
    }

    /// The other half of the same rule: on its own continent the same shape of scroll works.
    /// Without this the test above would pass on a handler that refused everything.
    #[test]
    fn a_henesys_scroll_works_while_standing_on_victoria_island() {
        let (mut s, acct, id) = session_with_scroll(2_030_004, 1, 10_001_090);
        let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_030_004, 1));

        assert_eq!(stored_map(&s, acct, id), 10_001_000, "Henesys");
        assert!(has(&out, net::opcode::SET_FIELD));
        assert_eq!(held(&s, id), None, "the last one empties the slot");
        let op = out
            .iter()
            .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
            .expect("the slot change is reported");
        assert_eq!(
            op.body,
            net::inventory::inventory_removed(store::InventoryType::Use.as_u8() as i8, 1),
            "mode 3 REMOVE - mode 1 would leave a phantom stack of 0 on screen"
        );
    }

    /// The `gm_map` guard, on this path, in the state that made it necessary: `map_exists` is
    /// **fail-open** on an empty table, so a missing `gm-handbook/fields.txt` would otherwise
    /// let every destination through while the code read as though it had checked.
    #[test]
    fn an_empty_field_table_refuses_loudly_and_keeps_the_scroll() {
        let (mut s, acct, id) = session_with_scroll(2_030_000, 1, 40);
        let mut cfg = (*s.config).clone();
        cfg.fields.clear();
        s.config = Arc::new(cfg);

        let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_030_000, 1));

        assert_eq!(stored_map(&s, acct, id), 40, "nobody moved");
        assert!(!has(&out, net::opcode::SET_FIELD));
        assert_eq!(held(&s, id), Some(1), "and the scroll is still there");
        let notice = out
            .iter()
            .find(|r| r.opcode == net::notice::CHAT_NOTICE)
            .expect("the refusal is visible on screen");
        let text = String::from_utf8_lossy(&notice.body).to_string();
        assert!(text.contains("field table is empty"), "{text}");
        assert!(text.contains("NOT consumed"), "{text}");
    }

    /// A map with no row in the return table: refuse, and do not invent a town.
    #[test]
    fn a_map_with_no_return_row_keeps_the_scroll() {
        let (mut s, acct, id) = session_with_scroll(2_030_000, 1, 40);
        let mut cfg = (*s.config).clone();
        cfg.revive_maps.clear();
        s.config = Arc::new(cfg);

        let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_030_000, 1));

        assert_eq!(stored_map(&s, acct, id), 40);
        assert!(!has(&out, net::opcode::SET_FIELD));
        assert_eq!(held(&s, id), Some(1));
        assert!(has(&out, net::stats::STAT_CHANGED), "still answered");
    }

    /// **A potion above the base maximum must never LOWER health.**
    ///
    /// The owner's Cobalt stands at 447 with a base of 358 because the client applies Max HP
    /// Increase itself. Before `Session::pools`, a Red Potion drunk at 400 hit
    /// `.min(chr.max_hp)` and CUT them to 358 - a clamp to the wrong ceiling takes health
    /// away. Now: base 358, skill 15, at 400 the potion lands +47 to the 447 ceiling and not
    /// one point past it, and the base in the record stays 358.
    #[test]
    fn a_potion_heals_up_to_the_boosted_ceiling_and_never_cuts_health_to_the_base() {
        if !std::path::Path::new("../../gm-handbook/skills.txt").exists() {
            return; // generated, gitignored
        }
        let (mut s, acct, id) = session_with_scroll(2_000_000, 2, 40);
        let mut cfg = (*s.config).clone();
        cfg.firstjob = crate::firstjob::CombatTable::load(std::path::Path::new("../../gm-handbook/skills.txt"));
        s.config = Arc::new(cfg);
        let mut chr = s.claimed_character().unwrap();
        chr.max_hp = 358;
        chr.hp = 400;
        s.store.save_character_progress(&chr).unwrap();
        s.store.set_skill_level(id, super::super::pools::MAX_HP_INCREASE, 15).unwrap();

        s.on_use_item(&net::useitem::use_item(0, 1, 2_000_000, 1));
        let after = s.store.characters_for(acct, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(after.hp, 447, "400 + 100 capped at 358 + 89, not cut to 358");
        assert_eq!(after.max_hp, 358, "the base is the client's input and stays put");

        // And a second one at the ceiling changes nothing but the stack.
        s.on_use_item(&net::useitem::use_item(0, 1, 2_000_000, 1));
        let after = s.store.characters_for(acct, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(after.hp, 447);
    }

    /// **The regression this branch could most easily cause.** The scroll check runs before
    /// the potion table, so a potion must still heal, still shrink its stack, and still send
    /// no SetField.
    #[test]
    fn a_red_potion_is_untouched_by_the_scroll_branch() {
        let (mut s, acct, id) = session_with_scroll(2_000_000, 2, 40);
        let mut chr = s.claimed_character().unwrap();
        chr.max_hp = 500;
        chr.hp = 1;
        s.store.save_character_progress(&chr).unwrap();

        let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_000_000, 1));

        let after = s
            .store
            .characters_for(acct, 0)
            .unwrap()
            .into_iter()
            .find(|c| c.id == id)
            .unwrap();
        assert_eq!(after.hp, 101, "1 + 100 - the potion path still runs");
        assert_eq!(after.map_id, 40, "and a potion is not a teleport");
        assert!(!has(&out, net::opcode::SET_FIELD));
        assert_eq!(held(&s, id), Some(1));
    }

    /// An item that is neither a scroll nor in the potion table is still refused rather than
    /// eaten - the branch must not have turned "unknown" into "scroll".
    #[test]
    fn an_unknown_item_is_still_refused_and_kept() {
        let (mut s, _acct, id) = session_with_scroll(2_040_000, 1, 40);
        let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_040_000, 1));
        assert_eq!(held(&s, id), Some(1), "a magic scroll is not drunk and not a warp");
        assert!(!has(&out, net::opcode::SET_FIELD));
        assert!(has(&out, net::stats::STAT_CHANGED), "still answered");
    }
}
