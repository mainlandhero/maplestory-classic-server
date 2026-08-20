//! Where the player is: field entry, portals, `!map`'s destination, Log Out, and the
//! channel switch.
//!
//! Split out of `session.rs` rather than rewritten - every method here kept its body and
//! its doc block. The file boundary is the only new thing.

use super::*;

impl Session {

    /// Populate the field the client has just finished entering.
    ///
    /// The client does **not** spawn NPCs from the map WZ - its field loader walks `life`
    /// only to preload art. The only code that builds a populated NPC takes a packet, so
    /// every NPC on every field is ours to send, and ours to re-send after every `SetField`
    /// because the pool is destroyed and rebuilt empty on each field entry.
    ///
    /// `research/npc-spawn.md` has the working, including how the routing was found.
    pub(super) fn on_field_entered(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        // The NPC pool is destroyed and rebuilt on every field entry, so the chatter cursors
        // go with it: an object id from the previous map addresses nothing here, or worse,
        // addresses a different NPC.
        self.reset_chatter(chr.map_id, self.clock_ms);
        let empty: Vec<net::opcode::FieldNpc> = Vec::new();
        let out: Vec<Reply> = self.config.npcs.get(&chr.map_id).unwrap_or(&empty)
            .iter()
            .map(|npc| Reply {
                opcode: net::opcode::NPC_ENTER_FIELD,
                body: net::opcode::npc_enter_field(npc),
                what: format!(
                    "NpcEnterField: template {} at ({}, {}) on foothold {}, object id {} -                      the client cannot spawn this itself, it only preloads the art.",
                    npc.template_id, npc.x, npc.cy, npc.fh, npc.object_id
                ),
            })
            .collect();

        // Mobs, from the same WZ `life` walk that produced the NPCs and for the same
        // reason: the client's field loader only preloads `Mob/%07d.img` art, and the pool
        // is destroyed and rebuilt empty on every field entry, so they must be re-sent
        // after every SetField rather than once.
        //
        // **OFF BY DEFAULT since the run of 2026-08-19, because the body faults the
        // client.** TestCharD entered map 40 (Snail Hunting Ground I, 40 spawns); the two
        // NPCs dispatched cleanly and the client then died on the FIRST 0x03C6 with
        // 0xC0000005 at `0x141c810b0`. That is inside `FUN_141c81040`, and the faulting
        // instruction is `CMP qword ptr [RCX],RDX` after
        // `MOV RAX,[RSI+0x2b8] / LEA RCX,[RAX+0x828] / CMOVE RCX,RDX` - so **`mob+0x2b8`
        // was null** and the client dereferenced without a guard.
        //
        // Everything else on that entry worked, which is what makes the diagnosis narrow:
        // the character record decoded, the equipped list decoded all four items, and both
        // NPCs went through. Mobs are the only thing that changed the outcome.
        //
        // Turn back on with `--mobs` when the body is the variant under test.
        let no_mobs: Vec<net::mob::FieldMob> = Vec::new();
        let mut out = out;
        let mobs = if self.config.send_mobs {
            self.config.mobs.get(&chr.map_id).unwrap_or(&no_mobs)
        } else {
            &no_mobs
        };
        // A spawn point is not a mob. Map 40 has 40 spawn points and a real server keeps
        // about 30 of them filled for a solo player, so sending one per point
        // over-populates the field. The cap is NOT in the WZ - map 40's info node has a
        // mobRate but no capacity of any name - so it is our policy; see
        // config::spawn_capacity for what is measured and what is inferred.
        //
        // And which points are filled matters as much as how many: a mixed map keeps each
        // type's SHARE of the total, so this cannot just take the first N in WZ order.
        // Players ON THE FIELD, not on the channel. Always 1 today: there is no
        // field-occupancy tracking here at all, so the 6+ branch of config::spawn_capacity
        // is written and untaken. Adding occupancy is a change to this line.
        let players_here = 1;
        let alive = crate::config::spawn_capacity(mobs.len(), players_here);
        let alive = match self.config.mob_limit {
            Some(n) => alive.min(n),
            None => alive,
        };
        let chosen = crate::config::share_balanced(mobs, alive);
        // A new field means new object ids. Anything remembered from the last one is stale
        // and, worse, could collide - so it goes.
        self.mob_hp.clear();
        for mob in chosen {
            self.mob_hp.insert(mob.object_id, mob.hp);
            out.push(Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(mob),
                what: format!(
                    "MobEnterField: template {} at ({}, {}) on foothold {}, object id {}, hp {} - {} bytes. The client cannot spawn this itself.",
                    mob.template_id, mob.x, mob.y, mob.fh, mob.object_id, mob.hp,
                    mob.body_len()
                ),
            });

            // **And then hand the mob to the client, which is what makes it move.**
            //
            // Spawning a mob does not animate it. The owner, 2026-08-19: six snails rendered on
            // map 40 and stood completely still. The server does not drive mob movement in
            // this game - it grants CONTROL of a mob to a client, and that client then runs
            // the wander and the idle animation locally and reports each path back as
            // `0x02FF`. Without this packet a mob is a picture.
            //
            // It explains the second symptom too. The combat agent decoded a real attack
            // from the same session: the owner at (473, 395), mob 2000 at (424, 395) - 49 pixels
            // away on the same ground line - and the attack carried **zero targets**. The
            // client would not aim at a mob nobody had given it. One packet, both symptoms.
            //
            // Order matters: `after` its MobEnterField, per research/mob-behaviour.md §3.
            // And the level must not be 0 - that DESPAWNS rather than releases, which is why
            // `mob_release_controller` exists under its own name.
            out.push(Reply {
                opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                body: net::mobmove::mob_change_controller(mob, net::mobmove::CONTROL_NORMAL),
                what: format!(
                    "MobChangeController: object id {} to this client, level {} - {} bytes. The client runs the mob's movement and reports it as 0x02FF.",
                    mob.object_id,
                    net::mobmove::CONTROL_NORMAL,
                    net::mobmove::change_controller_len(mob)
                ),
            });
        }

        // The character is dressed by the SetField record itself now, not from here. This
        // used to push a 0x0138 UserAvatarModified as a guess at the equipment problem;
        // that opcode is **dead code at byte level** - its apply is guarded by a call to
        // 0x1407f5ce0, which is three bytes of `xor eax,eax; ret`, followed by TEST/JZ. No
        // trigger and no timing would ever have made it work, so sending it was noise in
        // the log. See net::opcode::USER_AVATAR_MODIFIED and research/naked-character.md.
        out
    }


    /// Move a character to a map and tell the client, persisting the move.
    ///
    /// Shared by the portal walk and the `/map` GM command, so both go through one path -
    /// a second copy of this is how the two would drift.
    pub(super) fn go_to_map(&mut self, chr: &mut net::opcode::Character, map: u32, portal: u8, why: String)
        -> Vec<Reply>
    {
        chr.map_id = map;
        chr.portal = portal;
        let stored = self.store.set_character_map(chr.id, map);
        let warn = match stored {
            Ok(()) => String::new(),
            // Not fatal: the client is told where it is either way, and the next login puts
            // it back where it was.
            Err(e) => format!(" - WARNING: not stored ({e}), so this will not survive a relog"),
        };
        // **Dressed, exactly like the migration's SetField.** This sent the bare form until
        // 2026-08-19, which is why the owner's items had their stats on entering the world and
        // lost them the moment they used a portal or `!map`: every record after the first
        // carried EquipStats::default(), all zeros. It also explains the whole "the tooltip
        // reads a different object" investigation - they had reached map 40 with `!map`, so
        // the record they were hovering really did contain zeros. There was never a second
        // object.
        let dressed = self.dressed(chr);
        let (quests, quest_note) = self.quest_book(chr.id);
        vec![Reply {
            opcode: net::opcode::SET_FIELD,
            body: net::opcode::set_field_with_character_dressed_quests(
                chr,
                self.config.world_id,
                self.clock_base(),
                self.config.channel_id,
                &dressed,
                &quests,
            ),
            what: format!(
                "SetField, {why}, for character {} ({}){warn}{quest_note}",
                chr.id, chr.name
            ),
        }]
    }


    /// **Change Channel.** The client pressed the button in the dialog.
    ///
    /// # This exists because enabling one byte made the button reachable
    ///
    /// `net::opcode::CHANNEL_ENABLED` went from `0` to `1` on 2026-08-20, which is what makes
    /// a channel row clickable at all. Before that the click produced nothing on the wire and
    /// this handler would have been dead code. `research/channel-two-greyed.md` establishes
    /// that draw and click read the **same** predicate - all six callers of `FUN_142cb9510`
    /// test its result with a bare `test eax,eax` - so there was never an option to make the
    /// row look enabled without making it send.
    ///
    /// **So this had to be answered in the same change.** An unanswered packet freezes the
    /// client's entire UI, quit prompt included, and shipping a newly-clickable button whose
    /// packet nobody answers would have been building the freeze deliberately.
    ///
    /// # The reply shape is inference, and the honest label is [I]
    ///
    /// No capture of a successful channel change exists. `net::opcode::migrate` is what the
    /// login server answers character-select with, and a channel change is the same
    /// operation - "connect to this address as this character" - so it is sent here too.
    /// `crates/net/src/channel.rs` records that expectation as **[I]**; this does not upgrade
    /// it. A wrong reply is a different failure from a frozen UI, and only one of the two is
    /// certain in advance.
    ///
    /// **The migration is minted here, and it is still not authentication.** A `u32` seed
    /// identifies a pending migration; it does not prove who is on the far end. Single use.
    pub(super) fn on_change_channel(&mut self, payload: &[u8]) -> Vec<Reply> {
        let request = net::channel::ChangeChannelRequest::parse(payload);
        let Some(req) = request else {
            return self.change_channel_refused(
                "the Change Channel body did not parse - see net::channel::ChangeChannelRequest"
                    .to_string(),
            );
        };
        let target = u32::from(req.target_channel);
        let Some(claimed) = self.claimed.clone() else {
            return self
                .change_channel_refused("no character is claimed on this connection".to_string());
        };
        let Some(addr) = self.config.channels.get(target as usize).copied() else {
            return self.change_channel_refused(format!(
                "this world has no address for channel {target} - pass --channels to the world server"
            ));
        };
        if target == self.config.channel_id {
            return self.change_channel_refused(format!(
                "channel {target} is the one you are already on"
            ));
        }
        let seed = match self.store.create_migration(
            claimed.account_id,
            claimed.character_id,
            self.config.world_id,
            target,
        ) {
            Ok(seed) => seed,
            Err(e) => return self.change_channel_refused(format!("could not mint a migration: {e}")),
        };
        vec![Reply {
            opcode: net::opcode::MIGRATE_COMMAND,
            body: net::opcode::migrate(addr, claimed.character_id, seed),
            what: format!(
                "Change Channel: character {} to channel {target} at {addr}, seed {seed:#010x} - single use, NOT authentication. The reply SHAPE is inference; no capture of a channel change exists.",
                claimed.character_id
            ),
        }]
    }


    /// Refuse a channel change **with a packet**, not with silence.
    ///
    /// `migrate_refused` carries a message the client renders, which is the same shape the
    /// login server uses when it cannot migrate. Saying nothing would freeze the UI.
    pub(super) fn change_channel_refused(&self, why: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::opcode::MIGRATE_COMMAND,
            body: net::opcode::migrate_refused("Cannot change channel right now."),
            what: format!("Change Channel REFUSED: {why}"),
        }]
    }


    /// Answer Log Out, and **this is not optional in the way most replies are**.
    ///
    /// `0x01BE`'s builder sets `world->[0x33f4] = 1`, and the `SetField` handler
    /// `FUN_142097f80` tests that byte immediately after reading its 8-byte FILETIME and
    /// returns to its epilogue if it is set. **Only `0x0106` clears it.** So an unanswered
    /// Log Out does not just leave the player stuck on the field - it makes **every
    /// subsequent `SetField` vanish in silence**: no dialog, no fault, nothing in any log.
    /// Same failure class as the `player->[0x2330]` latch in `research/npc-click.md`, far
    /// worse blast radius.
    ///
    /// The message must not be empty; an empty string is a no-op in the client.
    ///
    /// **The teardown is in place rather than a reconnect** - the handler builds no
    /// `sockaddr` and constructs a login stage directly. Whether the client also closes this
    /// socket is not settled, and the next run answers it for free: watch whether the
    /// following packet lands in `login.log` or `world.log`.
    pub(super) fn on_log_out(&mut self) -> Vec<Reply> {
        // The conversation and the field's chatter belong to a session that is ending.
        self.conversation = None;
        self.chatter.clear();
        vec![Reply {
            opcode: net::notice::LOG_OUT_RESULT,
            body: net::notice::log_out_result("Returning to the login screen."),
            what: "LogOutResult - and answering this is what clears world->[0x33f4]. Until it is cleared the client silently drops every SetField."
                .to_string(),
        }]
    }


    /// Answer the client walking into a portal.
    ///
    /// The reply is another `SetField` with `characterData = 1` - the **long** form, the one
    /// byte-identical to the packet that already worked, differing only in the map id at
    /// stat-block offset 84. The short `characterData = 0` form is the shape actually
    /// designed for "same character, new map", and it is probably correct here now that the
    /// client has a live field; but `research/transfer-field-request.md` could not prove its
    /// precondition (`world+0x2358`, the `CUserLocal` slot) is populated - an exhaustive scan
    /// found 222 readers and **no** store. Sending an unproven form to save 200 bytes would
    /// trade a working path for a guess. The long form is idempotent because the object it
    /// re-fetches is a lazy singleton.
    pub(super) fn on_transfer_field(&mut self, payload: &[u8]) -> Vec<Reply> {
        let req = parse_transfer_field(payload);
        let chr = self.claimed_character();

        // Always answer. Even a request we cannot resolve gets a SetField for the map the
        // character is already on, because silence freezes the client's entire UI.
        let Some(mut chr) = chr else {
            return vec![Reply {
                opcode: net::opcode::SET_FIELD,
                body: net::opcode::set_field_minimal(self.clock_base(), self.config.channel_id),
                what: "transfer-field request, but the character could not be loaded -                        answered with the minimal record rather than dropped, because an unanswered packet freezes the client's whole UI."
                    .to_string(),
            }];
        };

        // Where the character ARRIVES. The source portal names its destination portal in
        // the WZ's `tn`, and the stat block wants that portal's index on the target map.
        // Without it every walk lands on the map's spawn point, which is right for a login
        // and wrong for a door - the character pops out somewhere else entirely.
        let mut arrival = 0u8;
        let (target, note) = match &req {
            Some(r) => match r.target_field.or_else(|| {
                self.config.portals.get(&(chr.map_id, r.portal_name.clone())).map(|(to, tn)| {
                    arrival = self
                        .config
                        .portal_index
                        .get(&(*to, tn.clone()))
                        .copied()
                        .unwrap_or(0);
                    *to
                })
            }) {
                Some(t) => (t, format!("portal {:?} -> map {t} portal {arrival}", r.portal_name)),
                None => (
                    chr.map_id,
                    format!(
                        "portal {:?} on map {} is NOT in the portal table, so this re-sends the current map rather than guessing a destination",
                        r.portal_name, chr.map_id
                    ),
                ),
            },
            None => (chr.map_id, "the body was too short to parse - re-sending the current map".to_string()),
        };


        self.go_to_map(&mut chr, target, arrival, note)
    }
}
