//! The chat box: what the player says out loud, and the `!` commands typed into it.
//!
//! Every command acknowledges itself. A command that silently works and a command that
//! silently does nothing look identical on screen, and telling them apart has cost
//! client launches.

use super::*;

impl Session {

    /// GM commands typed into the chat box.
    ///
    /// **This is a debugging tool on a server where nothing authenticates**, so there is no
    /// permission check to write - every connection is already the same account, and adding
    /// one here would be theatre. Say so rather than implying otherwise.
    ///
    /// Chat is fire-and-forget: the client froze on none of the runs where it went
    /// unanswered, so a command that does nothing is safe.
    ///
    /// ## The prefix is `!`, not `/`, and that is not a preference
    ///
    /// **The client never transmits a `/` line.** The owner typed `/map 1` and the session's
    /// entire capture contains no `0x00E7` at all, while a plain "Hello" in the same tab had
    /// produced one. The client parses slash commands itself: `/find`, `/whisper`, `/party`,
    /// `/friend`, `/trade`, `/level` and `/h` are all baked into the executable as strings,
    /// and an unknown one is swallowed before it reaches the wire.
    ///
    /// So a server-side command has to look like ordinary chat. `!` is ordinary chat.
    pub(super) fn on_chat(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(text) = net::opcode::parse_chat(payload) else { return Vec::new() };
        let text = text.trim();

        // **Anything that is not a command is said out loud.** The client draws nothing for
        // its own chat: typing sends `0x00E7` and stops. The owner, 2026-08-19, typed "Hello",
        // "Hello2" and "Hello3" and saw nothing at all, because this function matched them
        // against `!map`, found nothing, and returned an empty reply. The balloon and the
        // chat-log line both come from `0x0231` coming back - see net::userchat.
        let Some(command) = text.strip_prefix('!') else {
            return self.say_out_loud(text);
        };
        let (name, arg) = match command.split_once(char::is_whitespace) {
            Some((n, a)) => (n, a.trim()),
            None => (command, ""),
        };
        // `!meso rate 2` is how the owner wrote it, and it splits into a name of "meso" with
        // "rate 2" left in the argument. Fold the two-word spellings onto the one-word ones
        // rather than answering `!exp rate 2` with a complaint that "rate 2" is not a number,
        // which is a true statement about the wrong question.
        let (name, arg) = match name {
            "exp" | "meso" | "drop" if arg == "rate" || arg.starts_with("rate ") => {
                let one_word = match name {
                    "exp" => "exprate",
                    "meso" => "mesorate",
                    _ => "droprate",
                };
                (one_word, arg["rate".len()..].trim())
            }
            _ => (name, arg),
        };
        match name {
            "map" => self.gm_map(arg),
            "item" => self.gm_item(arg),
            "exp" => self.gm_exp(arg),
            "heal" => self.gm_heal(),
            "exprate" => self.gm_exp_rate(arg),
            "mesorate" => self.gm_meso_rate(arg),
            "droprate" => self.gm_drop_rate(arg),
            "setrates" => self.gm_set_rates(arg),
            // Reads and changes nothing, which is why it is the one command here that would
            // survive a permission check if this server ever grew one.
            "rates" => self.gm_rates(),
            "job" => self.gm_job(arg),
            "migsweep" => self.gm_mig_sweep(arg),
            "npcecho" => self.gm_npc_echo(arg),
            "npcfx" => self.gm_npc_effect(arg),
            "buff" => self.gm_buff(arg),
            "unbuff" => self.gm_unbuff(arg),
            "nx" => self.gm_nx(arg),
            // The shop prices in LP, so this is the one that buys. See gm_lp.
            "lp" | "leafpoints" => self.gm_lp(arg),
            "buy" => self.gm_buy(arg),
            "locker" => self.gm_locker(arg),
            "help" => self.gm_ack(GM_COMMANDS.to_string()),
            "" => self.gm_ack(format!("Not a command. {GM_COMMANDS}")),
            other => self.gm_ack(format!("!{other} is not a command. {GM_COMMANDS}")),
        }
    }


    /// `!job <id>` - set the character's job, and nothing else.
    ///
    /// **This is goal E's cheapest first experiment, and it is a measurement, not a
    /// feature.** `STATUS.md` goal E said the job-change packet was "still to be found"; it
    /// was already decoded and nobody had joined the two up. It is **`0x007C` mask bit 5**
    /// (`net::stats::bits::JOB`), `u16 job, u16 subJob`.
    ///
    /// And the client does the rest itself. The `0x007C` handler ends with **[L]**:
    ///
    /// ```text
    /// 142d55bce  test r12b, 0x20            ; the JOB bit
    /// 142d55beb  test ax, ax / je           ; job read back from +0x33; 0 -> NO fanfare
    /// 142d55bf1  [0x143A46F50] -> L"Effect/BasicEff.img/JobChanged"
    /// 142d55d52  [0x143A48498] -> L"JobChanged"     ; sound, volume 100
    /// ```
    ///
    /// `JobChanged` **is** one of `BasicEff.img`'s 40 nodes - unlike `QuestClear`, which was
    /// cut - so this one should be sound *and* picture. **Do not also send `0x02D1`**: the
    /// `0x007C` already fires the effect and two would stack. That was a real near-miss -
    /// reaching for `user_effect_local(14)` by analogy with the quest fanfare would have sent
    /// a **1-byte body where effect 14 reads two `u16`s**, which is the short-packet mistake
    /// this project has already paid for twice.
    ///
    /// `!job 0` must be **silent**: that is the `test ax,ax` gate above, and it confirms the
    /// gate that was read is the gate that runs. That check is free, and it is why the
    /// argument is not validated against the job table here.
    pub(super) fn gm_job(&mut self, arg: &str) -> Vec<Reply> {
        let Ok(job) = arg.parse::<u16>() else {
            return self.gm_ack(format!(
                "!job: {arg:?} is not a job id. Try !job 100 (Warrior), 200 (Magician), 300 (Bowman), 400 (Thief), or !job 0 to check the no-fanfare gate."
            ));
        };
        let Some(mut chr) = self.claimed_character() else {
            return self
                .gm_ack("!job REFUSED: no character is claimed on this connection.".to_string());
        };
        let was = chr.job;
        chr.job = job;
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.gm_ack(format!("!job FAILED: {e}"));
        }
        let mut out = self.gm_ack(format!(
            "{} is now job {job} (was {was}). Expect the JobChanged effect AND its sound - unless job is 0, which the client's own gate suppresses.",
            chr.name
        ));
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { job: Some((job, 0)), ..Default::default() }.build(),
            what: format!(
                "StatChanged: job {was} -> {job}, mask bit 5. The client plays Effect/BasicEff.img/JobChanged and the JobChanged sound ITSELF from this packet - no 0x02D1 is sent, deliberately"
            ),
        });
        out
    }


    /// `!migsweep [first] [last]` - find the channel stage's migrate opcode, in one run.
    ///
    /// **The one thing about Change Channel that cannot be read statically.** The reply is
    /// not a case of the stage switch at all: it is `FUN_1415d8c00`, a *socket-level*
    /// handler dispatched from the Themida VM, and `.themida` has `SizeOfRawData = 0`. It has
    /// zero callers of every kind and zero 4-byte RVA references, so no scan can name its
    /// opcode. `research/change-channel-reply.md`.
    ///
    /// Everything else about it *is* measured. The body is **seven bytes** - `u8 ok`,
    /// `u32 ip` in **network** order straight into `sin_addr`, and `u16 port`
    /// **little-endian**, because the client `htons`es it itself. The read count was
    /// cross-checked two ways, by `tools/listing.py` and `tools/reads.py --depth 2`, and they
    /// agree exactly.
    ///
    /// So the opcode is swept. `0x0019..0x0022` are the ten slots the login switch has no
    /// case for, and they line up against the reference's socket opcodes at a constant
    /// `+0x0A`. **[I]** - which is precisely why this is an experiment and not a fix.
    ///
    /// # Why one batch is enough, and how the winner is identified
    ///
    /// The hook writes one dispatch line per inbound packet **naming the opcode**, on handler
    /// return. So the hook log shows exactly which of these the client dispatched, and the
    /// migrate one is the last line before the socket closes. That is the same instrument
    /// that settled the equip crash, and it means the owner has to time nothing.
    ///
    /// **The 64 zero bytes of padding are not decoration.** An over-read in the client throws
    /// (`1406e8b51` -> `_CxxThrowException` -> `int3`), so a wrong guess landing on a handler
    /// that wants a longer body would kill the client rather than be ignored. Padding makes a
    /// wrong guess *inert*.
    ///
    /// **The server must not close the socket.** `FUN_142caa360`'s first act is to tear the
    /// connection down client-side. Measured corroboration: after the login `0x0011` the
    /// client closed 8 ms later; after the failed channel `0x0011` it stayed open six seconds.
    pub(super) fn gm_mig_sweep(&mut self, arg: &str) -> Vec<Reply> {
        let mut parts = arg.split_whitespace();
        let parse_one = |t: Option<&str>, fallback: u16| -> u16 {
            t.and_then(|t| {
                let t = t.trim_start_matches("0x").trim_start_matches("0X");
                u16::from_str_radix(t, 16).ok()
            })
            .unwrap_or(fallback)
        };
        let first = parse_one(parts.next(), 0x0019);
        let last = parse_one(parts.next(), 0x0022);
        if last < first || usize::from(last - first) >= 32 {
            return self.gm_ack(format!(
                "!migsweep: {first:#06x}..{last:#06x} is not a sensible range. Try !migsweep (defaults to 19 22) or !migsweep 24 33."
            ));
        }

        // The channel we are NOT on. With the usual two channels that is the other one.
        let target = if self.config.channel_id == 0 { 1 } else { 0 };
        let Some(addr) = self.config.channels.get(target as usize).copied() else {
            return self.gm_ack(format!(
                "!migsweep REFUSED: this world has no address for channel {target} - pass --channels to the world server."
            ));
        };
        let Some(claimed) = self.claimed.clone() else {
            return self.gm_ack("!migsweep REFUSED: no character is claimed.".to_string());
        };
        // Mint a real migration, so a channel that DOES accept the packet can be entered
        // rather than bouncing on arrival. `on_change_channel` was always right about this
        // half - only the opcode and the body were wrong.
        let seed = match self.store.create_migration(
            claimed.account_id,
            claimed.character_id,
            self.config.world_id,
            target,
        ) {
            Ok(seed) => seed,
            Err(e) => return self.gm_ack(format!("!migsweep FAILED to mint a migration: {e}")),
        };

        // `SocketAddrV4`, so there is no IPv6 case to refuse - which is right, because the
        // client's `sin_addr` is a 4-byte IPv4 field and nothing else would fit.
        let mut out = self.gm_ack(format!(
            "!migsweep: sending {first:#06x}..{last:#06x} to channel {target} at {addr}. Read the hook log: the LAST dispatch line before the socket closes names the opcode. The Change Channel button now sends the same sweep on its own."
        ));
        out.extend(self.migrate_candidates(target, addr, seed, first, last));
        out
    }


    /// `!npcfx on|off` - the NPC **appear-effect switch**, `0x0452`.
    ///
    /// # The lever two passes and one measurement all missed
    ///
    /// The owner, after `!npcecho`: *"The copy that was spawned in additionally faded in as well
    /// after !npcecho was executed. The original Heena stayed on the screen."* That looked
    /// like the end of the road - both creation packets fade, so the packet is not the
    /// variable. It is not the end of the road, and the reason the echo could never have
    /// answered it is that **`0x044F` and `0x0451` run the same decoder body**,
    /// `FUN_141e36b20`. Comparing them was comparing a thing with itself.
    ///
    /// Inside that shared body sits a block gated on `DAT_143ad2d30 == 0` which allocates a
    /// `0x90`-byte object, constructs it against the NPC, stamps it with a clock value and
    /// starts it. `0x0452` is the packet that sets that global, and its two branches are what
    /// make the identification more than a guess: `v == 0` runs the **identical** allocate /
    /// construct / stamp sequence on every NPC already in the pool, and `v != 0` calls
    /// `FUN_141e64690` to tear it down. One packet creates and destroys exactly the thing
    /// creation creates. `research/npc-spawn.md` §3.1 named `0x0452` "a global show/hide
    /// toggle" from a quick read; the listing says it is narrower and more useful than that.
    ///
    /// # How to test it, and why it is a command rather than a change to field entry
    ///
    /// `!npcfx off` then `!npcecho`, on a map whose NPCs have already faded in. The switch is
    /// global and sticky, so the echoes are created with the global already set:
    ///
    /// | on screen | what it says |
    /// |---|---|
    /// | the echoes **pop in solid** | that object is the fade, and field entry should send `0x0452` before its `0x044F`s |
    /// | the echoes **still fade** | the object is not the fade. It is a real elimination rather than another absence, because this is the only creation-time branch left in `FUN_141e36b20` |
    /// | **existing NPCs change** when the command runs | the walk does more than tear down an animation - say what changed |
    /// | **NPCs vanish** | `research/npc-spawn.md`'s "show/hide toggle" reading was right after all. `!npcfx on` puts it back, and so does a map change |
    ///
    /// Field entry is deliberately **not** changed: this way one run compares faded NPCs and
    /// popped ones on the same map, and nothing needs undoing if it does nothing.
    pub(super) fn gm_npc_effect(&mut self, arg: &str) -> Vec<Reply> {
        let enabled = match arg.trim().to_ascii_lowercase().as_str() {
            "on" | "1" | "true" => true,
            "off" | "0" | "false" => false,
            "" => {
                return self.gm_ack(
                    "!npcfx wants on or off. `!npcfx off` disables the NPC appear animation, then `!npcecho` shows whether that was the fade.".to_string(),
                )
            }
            other => {
                return self.gm_ack(format!("!npcfx: {other:?} is not on or off."));
            }
        };
        let mut out = self.gm_ack(format!(
            "!npcfx {}: sending 0x0452 with v={} - the appear-effect switch. The wire value is INVERTED (v=0 leaves it on). Now run !npcecho and say whether the copies POP or FADE.",
            if enabled { "on" } else { "off" },
            u32::from(!enabled)
        ));
        out.push(Reply {
            opcode: net::opcode::NPC_APPEAR_EFFECT,
            body: net::opcode::npc_appear_effect(enabled),
            what: format!(
                "NpcAppearEffect: {} - 0x0452 sets DAT_143ad2d30 = {}, and the creation path in FUN_141e36b20 builds the 0x90-byte appear object ONLY while that global is 0. Also walks every NPC already in the pool: v=0 rebuilds the object on each, v!=0 tears it down",
                if enabled { "ENABLED" } else { "DISABLED" },
                u32::from(!enabled)
            ),
        });
        out
    }

    /// `!npcecho [dx]` - spawn a second copy of every NPC on this map, the OTHER way.
    ///
    /// **An experiment, and it costs no relaunch.** The owner: *"You shouldn't need to patch the
    /// client. Are there no way for the server to send the NPC data to the client so that it
    /// appears instantly on map transition?"*
    ///
    /// Two static passes had concluded the server could not, and both were answering a
    /// narrower question than the one asked: they enumerated the **fields of `0x044F`** and
    /// correctly found no alpha, no visibility timer and no appear type. Neither enumerated
    /// the **packets the NPC pool accepts**, and there are two that create an NPC:
    ///
    /// ```text
    /// 0x044F  NpcEnterField        or  [obj+0x38], 1   then the 20-field body
    /// 0x0451  NpcChangeController  mov byte [obj+0x38], 2   then the IDENTICAL body
    /// ```
    ///
    /// **[L]**, `research/npc-spawn.md` §3.1. And the case that works uses the second one:
    /// every mob is sent `0x03C6` **and** `0x03D2`, and mobs are instant. NPCs have only ever
    /// been sent `0x044F`.
    ///
    /// # Why a GM command rather than changing field entry
    ///
    /// This way the comparison happens **in one run, on one map, against NPCs the owner has just
    /// watched fade in** - and it costs no manual launch to undo if it does nothing. Changing
    /// field entry would make the whole map one variant and need a relaunch to compare.
    ///
    /// The echoes get **fresh object ids** (`+ ECHO_ID_BASE`) because `flag != 0` on an id
    /// the pool already holds does not take the allocate path, and what it does instead is
    /// not decoded.
    ///
    /// # What each outcome means
    ///
    /// | on screen | what it says |
    /// |---|---|
    /// | the copies **pop in solid** | `0x0451` is the fix. Field entry switches to it |
    /// | the copies **fade in too** | the creation route is not the difference. It also kills the timing theory, because these arrive long after field entry |
    /// | **nothing appears** | the body or the flag is wrong, not the theory. `world.log` names every packet sent |
    /// | the copies are **see-through and stay so** | it was never a fade-in; something else is drawing them wrong |
    pub(super) fn gm_npc_echo(&mut self, arg: &str) -> Vec<Reply> {
        /// Added to each NPC's object id so the echo is a new key to the pool.
        const ECHO_ID_BASE: u32 = 5000;
        let dx: i16 = arg.parse().unwrap_or(70);
        let Some(chr) = self.claimed_character() else {
            return self.gm_ack("!npcecho REFUSED: no character is claimed.".to_string());
        };
        let empty: Vec<net::opcode::FieldNpc> = Vec::new();
        let npcs: Vec<net::opcode::FieldNpc> =
            self.config.npcs.get(&chr.map_id).unwrap_or(&empty).clone();
        if npcs.is_empty() {
            return self.gm_ack(format!(
                "!npcecho: map {} has no NPCs to copy. Try !map 1 first.",
                chr.map_id
            ));
        }
        let mut out = self.gm_ack(format!(
            "!npcecho: sending {} NPC(s) again as 0x0451 NpcChangeController, {dx} px to the side. Do they POP or FADE? The ones already on this map arrived as 0x044F.",
            npcs.len()
        ));
        for npc in npcs {
            let echo = net::opcode::FieldNpc {
                object_id: npc.object_id + ECHO_ID_BASE,
                x: npc.x.saturating_add(dx),
                ..npc
            };
            out.push(Reply {
                opcode: net::opcode::NPC_CHANGE_CONTROLLER,
                body: net::opcode::npc_change_controller(&echo, 1),
                what: format!(
                    "NpcChangeController ECHO: template {} at ({}, {}) as object {} - the 0x0451 creation route, which sets obj+0x38 to 2 where 0x044F sets bit 0. This is the packet mobs get and NPCs never have",
                    echo.template_id, echo.x, echo.cy, echo.object_id
                ),
            });
        }
        out
    }


    /// `!map <id>` - put the character on a map.
    pub(super) fn gm_map(&mut self, arg: &str) -> Vec<Reply> {
        let Ok(map) = arg.parse::<u32>() else {
            return self.gm_ack(format!("!map: {arg:?} is not a map id. Try !map 40."));
        };
        // Refuse a map the client cannot load. A character sent to an id with no field image
        // is stranded with no way back except another command.
        if !self.config.map_exists(map) {
            return self.gm_ack(format!(
                "!map REFUSED: {map} has no field image in this client, so it would strand you."
            ));
        }
        // **Do not claim to have checked something that was not checked.** `map_exists` is
        // fail-open on an empty table, so with no `gm-handbook/fields.txt` every id above
        // gets through - and a map with no field image kills the client. The owner typed
        // `!map 45` on 2026-08-20 and lost a session to it. Refusing instead would fail
        // closed on a tool problem; saying so out loud costs one line and turns a silent
        // hole into a visible one.
        if self.config.fields.is_empty() {
            return self.gm_ack(format!(
                "!map REFUSED: the field table is empty, so {map} could not be checked and a                  bad id would kill the client. Regenerate gm-handbook/fields.txt with                  tools/dump_portals.py, or restart the server so it loads."
            ));
        }
        let Some(mut chr) = self.claimed_character() else {
            return self.gm_ack("!map REFUSED: no character is claimed on this connection.".to_string());
        };
        let mut out = self.gm_ack(format!(
            "Teleporting {} to map {map}, {}",
            chr.name,
            self.map_name(map)
        ));
        // Portal 0 is the map's spawn point, which is where a GM warp should land.
        out.extend(self.go_to_map(&mut chr, map, 0, format!("GM !map {map}")));
        out
    }


    /// `!exp <amount>` - award experience, and level up if it pays for a level.
    ///
    /// A shortcut to the same machinery a kill uses, so goal D can be exercised without
    /// finding a mob: [`Session::award_experience`] does the levelling, the persistence and
    /// the `0x007C`. It adds rather than sets, because what is being checked is that a
    /// number *moves*.
    ///
    /// Three things this doc used to say are no longer true and are worth naming, because
    /// each was fixed by something measured rather than by a rewrite: levelling was "blocked
    /// on why the client collects zero targets" (the mob size scale, fixed); the bar "will
    /// not move until the next field entry" (`0x007C` moves it in place); and `exp` was "a
    /// hardcoded zero" in the record (it is a real column now).
    pub(super) fn gm_exp(&mut self, arg: &str) -> Vec<Reply> {
        let Ok(amount) = arg.parse::<u64>() else {
            return self.gm_ack(format!("!exp: {arg:?} is not an amount. Try !exp 100."));
        };
        let Some(chr) = self.claimed_character() else {
            return self
                .gm_ack("!exp REFUSED: no character is claimed on this connection.".to_string());
        };
        // One path for every source of experience - see `Session::award_experience`. `!exp`
        // used to add and persist on its own, which meant a GM award could never level
        // anyone while a kill could, and nothing would have said so.
        let before = chr.exp;
        // White: `!exp` is your own experience, not a share of somebody else's kill.
        let mut out = self.award_experience(amount, "!exp", true, false);
        if out.is_empty() {
            return self.gm_ack(format!("!exp {amount}: nothing to award."));
        }
        let now = self.claimed_character().map(|c| c.exp).unwrap_or(before);
        let mut ack = self.gm_ack(format!("{} gains {amount} experience: {before} -> {now}.", chr.name));
        ack.append(&mut out);
        ack
    }


    /// `!heal` - back to full HP and MP.
    ///
    /// **This exists because death does not.** Mobs deal contact damage now, and a character
    /// that reaches zero HP is disabled by the client with no way back: `research/user-hit.md`
    /// established that `hp = 0` will not hang the client, but not what plays the death and
    /// revive sequence. Until that is decoded, this is the way out - and it is better than
    /// silently clamping HP at 1, which would hide the fact that death is missing.
    pub(super) fn gm_heal(&mut self) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else {
            return self.gm_ack("!heal REFUSED: no character is claimed on this connection.".to_string());
        };
        chr.hp = chr.max_hp;
        chr.mp = chr.max_mp;
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.gm_ack(format!("!heal FAILED: {e}"));
        }
        let mut out = self.gm_ack(format!("{} is restored to {} HP.", chr.name, chr.max_hp));
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { hp: Some(chr.hp), mp: Some(chr.mp), ..Default::default() }
                .build(),
            what: format!("StatChanged: healed to {}/{} hp, {}/{} mp", chr.hp, chr.max_hp, chr.mp, chr.max_mp),
        });
        out
    }


    /// `!buff [skillId] [level] [tailBytes]` - send the temporary-stat packet with no skill,
    /// MP or cooldown in the way. Defaults to Nimble Feet at level 3 with the safe tail.
    ///
    /// # This is the single-variant test `research/buffs.md` §7.3 asked for
    ///
    /// The cast path checks four things before it sends anything - the skill is a buff, the
    /// character owns it at that level, the cooldown has run, and there is MP - and any of
    /// those refusing looks on screen exactly like the packet being wrong. `Skill.wz` puts
    /// Nimble Feet's `cooltime` at **180 seconds**, so a second cast to check something is
    /// three minutes away. This command skips all four and sends the same bytes through the
    /// same builder, so a `!buff` that works and a keypress that does not is a statement
    /// about the *gates*, not about the packet.
    ///
    /// It does still record the expiry, so the `0x007E` goes out on time and the buff can be
    /// watched all the way through.
    ///
    /// # The third argument is what makes the unknown measurable
    ///
    /// `net::buff::TAIL_LEN` is **slack around a length nobody has derived** - the 18-byte
    /// version threw an unhandled C++ exception in the client and killed it, while four
    /// static instruments say 18 should have been ample. A rebuild per attempt costs a manual
    /// launch; a chat line costs nothing, so the tail length is typeable.
    ///
    /// **Bisect downwards from a length that worked**, never upwards from one that did not:
    /// too short ends the session outright. Anything at or below the 18 that has already
    /// killed a client is refused here rather than re-learned.
    pub(super) fn gm_buff(&mut self, arg: &str) -> Vec<Reply> {
        if self.claimed_character().is_none() {
            return self.gm_ack("!buff REFUSED: no character is claimed on this connection.".to_string());
        }
        let mut parts = arg.split_whitespace();
        let skill_id = match parts.next() {
            None => net::buff::NIMBLE_FEET,
            Some(t) => match t.parse::<u32>() {
                Ok(v) => v,
                Err(_) => return self.gm_ack(format!("!buff: {t:?} is not a skill id.")),
            },
        };
        let level = match parts.next() {
            None => 3,
            Some(t) => match t.parse::<u32>() {
                Ok(v) => v,
                Err(_) => return self.gm_ack(format!("!buff: {t:?} is not a level.")),
            },
        };
        let tail = match parts.next() {
            None => net::buff::TAIL_LEN,
            Some(t) => match t.parse::<usize>() {
                Ok(v) if v <= net::buff::TAIL_KNOWN_TOO_SHORT => {
                    return self.gm_ack(format!(
                        "!buff: a {v}-byte tail is at or below the {} that already killed a \
                         client on 2026-08-22 - it would end this session and teach nothing. \
                         Bisect DOWNWARDS from a length that worked.",
                        net::buff::TAIL_KNOWN_TOO_SHORT
                    ))
                }
                Ok(v) => v,
                Err(_) => return self.gm_ack(format!("!buff: {t:?} is not a tail length.")),
            },
        };
        let Some(bl) = net::buff::buff_level(skill_id, level) else {
            return self.gm_ack(format!(
                "!buff: skill {skill_id} level {level} grants no temporary stat this server \
                 knows. Today that table is Nimble Feet (1002) at levels 1-3."
            ));
        };

        let now = self.clock_ms;
        let mut out = self.gm_ack(format!(
            "Casting skill {skill_id} level {level}: CTS bit {} = +{} for {} s, {tail}-byte \
             tail ({} bytes total). No skill check, no MP, no cooldown - if this works and the \
             keypress does not, the difference is a gate and not the packet.",
            bl.bit,
            bl.value,
            bl.seconds,
            net::buff::MASK_LEN + 10 + tail
        ));
        out.extend(self.grant_buff_with_tail(skill_id, bl, now, tail));
        out
    }

    /// `!unbuff [tailBytes]` - send `0x007E` for whatever buffs are held.
    ///
    /// # Why this is a command and not a timer
    ///
    /// The natural expiry does not send `0x007E` any more: the client holds its own
    /// `tExpire` and drops the stat on schedule, and a 127-byte `0x007E` killed the client
    /// thirty seconds after a working grant on 2026-08-22. Early removal - dispel, death,
    /// logout - will need the packet, so it stays built and stays testable, but it fires only
    /// when someone asks for it.
    ///
    /// The optional argument is the tail length, for the same downward bisect `!buff`
    /// supports. 127 total - the length that already threw - is refused.
    pub(super) fn gm_unbuff(&mut self, arg: &str) -> Vec<Reply> {
        if self.claimed_character().is_none() {
            return self.gm_ack("!unbuff REFUSED: no character is claimed.".to_string());
        }
        let tail = match arg.split_whitespace().next() {
            None => net::buff::TAIL_LEN,
            Some(t) => match t.parse::<usize>() {
                Ok(v) if 3 + net::buff::MASK_LEN + v <= net::buff::RESET_KNOWN_TOO_SHORT => {
                    return self.gm_ack(format!(
                        "!unbuff: a {v}-byte tail makes {} bytes, at or below the {} that \
                         already threw in the client. Bisect DOWNWARDS from a length that worked.",
                        3 + net::buff::MASK_LEN + v,
                        net::buff::RESET_KNOWN_TOO_SHORT
                    ))
                }
                Ok(v) => v,
                Err(_) => return self.gm_ack(format!("!unbuff: {t:?} is not a tail length.")),
            },
        };
        let mut out = self.clear_buffs(tail);
        if out.is_empty() {
            return self.gm_ack("!unbuff: nothing is buffed right now.".to_string());
        }
        let len = 3 + net::buff::MASK_LEN + tail;
        out.splice(
            0..0,
            self.gm_ack(format!(
                "Clearing held buffs with a {len}-byte 0x007E ({tail}-byte tail). The reads \
                 enumerate to 129, or 133 if the gated u32 fires; 127 threw."
            )),
        );
        out
    }

    /// `!nx [amount]` - grant NX to this account, or report the balance with no argument.
    ///
    /// # Why a new account starts at zero
    ///
    /// A test server that hands out currency by existing makes every later "did the purchase
    /// deduct?" question unanswerable, because the balance moves for reasons nobody is
    /// tracking. `store::cash::DEFAULT_NX` is 0 and this command is the only way in, so every
    /// credit has a cause and a log line.
    ///
    /// The wallet is per **account**, like storage, so this credits every character on it.
    pub(super) fn gm_nx(&mut self, arg: &str) -> Vec<Reply> {
        let Some(claimed) = self.claimed() else {
            return self.gm_ack("!nx REFUSED: no character is claimed on this connection.".to_string());
        };
        let account_id = claimed.account_id;

        let amount = match arg.split_whitespace().next() {
            None => {
                let w = self.store.cash_wallet(account_id).unwrap_or_default();
                return self.gm_ack(format!(
                    "Account {account_id} holds {} NX and {} maple points. \
                     `!nx 10000` grants some. NX is what this client's UI calls LEAF POINTS.",
                    w.nx, w.maple_points
                ));
            }
            Some(t) => match t.parse::<i64>() {
                Ok(v) => v,
                Err(_) => return self.gm_ack(format!("!nx: {t:?} is not an amount.")),
            },
        };

        match self.store.add_nx(account_id, amount) {
            // **Type this BEFORE clicking Cash Shop.** The balance reaches the screen in the
            // `0x05AD` that goes out with `SetCashShop`, and the client's own poll for it is
            // throttled to once every 60 s - so a grant made while the shop is already open
            // is not visible until it next opens. Nothing pushes a wallet update, deliberately:
            // the `0x05AD` arm re-triggers a pending purchase (`net::cashshop`).
            Ok(nx) => self.gm_ack(format!(
                "Account {account_id} now holds {nx} NX - the LEAF POINTS the shop spends. \
                 The wallet is per ACCOUNT, so every character on it sees this. Grant it \
                 BEFORE you click Cash Shop: the balance is carried in by the entry packet."
            )),
            // A refusal is reported rather than clamped: `add_nx` refuses a debit that would
            // go negative instead of flooring at zero, because a silent clamp is how a
            // purchase succeeds for free.
            Err(e) => self.gm_ack(format!("!nx FAILED and the balance is unchanged: {e}")),
        }
    }

    /// `!lp [amount]` - grant **Leaf Points**, the currency the shop actually charges.
    ///
    /// # Why this exists beside `!nx` rather than replacing it
    ///
    /// The owner, 2026-08-25, from inside the shop: *"NX and Leaf Points are separate fields. All of
    /// the items are priced in leaf points, so I was not able to purchase using NX."*
    ///
    /// `0x05AD` carries two `u32`s and the run confirmed **both**, in order: `!nx 10000` put
    /// 10,000 in the field the UI labels **NX** and 0 in the one it labels **Leaf Points**.
    /// Neither is misread and neither is swapped - they are simply two different pots, and
    /// every price tag in the shop reads `LP`.
    ///
    /// The decisive part is a **negative**: with `LP = 0` the client refused the purchase
    /// **itself** and sent **no `0x03E1` at all** - zero of them across a 103-second visit,
    /// grepped for that specific opcode rather than eyeballed. Had it been checking the NX
    /// field it would have sent, because that field held 10,000. So the affordability gate is
    /// client-side and it reads this balance.
    ///
    /// `!nx` is deliberately untouched. It fills the other field, which is real and displayed,
    /// and the owner asked for a second command rather than a changed one.
    pub(super) fn gm_lp(&mut self, arg: &str) -> Vec<Reply> {
        let Some(claimed) = self.claimed() else {
            return self
                .gm_ack("!lp REFUSED: no character is claimed on this connection.".to_string());
        };
        let account_id = claimed.account_id;

        let amount = match arg.split_whitespace().next() {
            None => {
                let w = self.store.cash_wallet(account_id).unwrap_or_default();
                return self.gm_ack(format!(
                    "Account {account_id} holds {} Leaf Points and {} NX. The shop prices \
                     everything in LP, so LP is the one that buys. `!lp 10000` grants some.",
                    w.maple_points, w.nx
                ));
            }
            Some(t) => match t.parse::<i64>() {
                Ok(v) => v,
                Err(_) => return self.gm_ack(format!("!lp: {t:?} is not an amount.")),
            },
        };

        match self.store.add_maple_points(account_id, amount) {
            Ok(lp) => self.gm_ack(format!(
                "Account {account_id} now holds {lp} Leaf Points - the LP every price tag in \
                 the shop is quoted in. Per ACCOUNT, like storage. Grant it BEFORE you click \
                 Cash Shop: the balance is carried in by the entry packet, and the client only \
                 re-asks once a minute."
            )),
            Err(e) => self.gm_ack(format!("!lp FAILED and the balance is unchanged: {e}")),
        }
    }

    /// `!buy <sn>` - **perform a real cash-shop purchase from the field.**
    ///
    /// # Why this exists at all
    ///
    /// The owner, 2026-08-24: *"we need a way to add Leaf Points (NX) in our server so we can
    /// attempt to make purchases in the Cash Shop so we can finish that entire transaction
    /// flow."* `!nx` is the first half. This is the second, and it is here rather than on the
    /// `0x03E1` path for one reason: **inside the shop there is no way to report a success.**
    /// Every `0x05AE` arm that clears the client's in-flight latch also puts a message on
    /// screen, and the wallet packet re-triggers the purchase. `session/cashshop.rs` has the
    /// addresses. So the shop refuses, and the sale happens here, where the answer is a chat
    /// line that has been on a wire hundreds of times.
    ///
    /// # The argument is an SN, not an item id, and that is not pedantry
    ///
    /// `Commodity.img` sells the same item at several counts and prices - `130200000` is one
    /// Megaphone for 100 NX and `130200001` is eleven for 1000, both item `5070000`. An item
    /// id cannot name a sale. `gm-handbook/commodity.txt` lists all 159.
    ///
    /// **Every effect hangs off the transition.** `store::buy_cash_item` checks the balance,
    /// debits it and places the item in one transaction; nothing here reports success unless
    /// that returns `Ok`, and nothing here reports a price that was not actually taken.
    pub(super) fn gm_buy(&mut self, arg: &str) -> Vec<Reply> {
        let Some(claimed) = self.claimed() else {
            return self
                .gm_ack("!buy REFUSED: no character is claimed on this connection.".to_string());
        };
        let account_id = claimed.account_id;

        let Some(Ok(sn)) = arg.split_whitespace().next().map(str::parse::<u32>) else {
            return self.gm_ack(format!(
                "!buy: {arg:?} is not a commodity serial. It is an SN, NOT an item id - the \
                 same item is sold at several prices. gm-handbook/commodity.txt has all {}. \
                 Try !buy 130200000 (1 Megaphone, 100 NX).",
                self.config.commodity.len()
            ));
        };

        let Some(row) = self.config.commodity.get(sn).cloned() else {
            return self.gm_ack(format!(
                "!buy REFUSED: SN {sn} is not a sale row. {} rows are loaded{}",
                self.config.commodity.len(),
                if self.config.commodity.is_empty() {
                    " - regenerate with: python tools/dump_commodity.py"
                } else {
                    ""
                }
            ));
        };
        if !row.on_sale {
            return self.gm_ack(format!(
                "!buy REFUSED: SN {sn} ({}) has onSale 0 in the client's own data.",
                row.name
            ));
        }
        let Some(inv) = store::InventoryType::for_item(row.item_id) else {
            return self.gm_ack(format!(
                "!buy REFUSED: item {} is in no inventory tab, so nothing could hold it.",
                row.item_id
            ));
        };

        let item = if inv == store::InventoryType::Equip {
            store::Item::equip(row.item_id)
        } else {
            store::Item::bundle(row.item_id, row.count)
        };

        match self.store.buy_cash_item(account_id, &item, row.price) {
            Ok(placed) => {
                let nx = self.store.cash_wallet(account_id).unwrap_or_default().nx;
                self.gm_ack(format!(
                    "Bought SN {sn}: {}x {} ({}) for {} NX. Locker slot {}, {nx} NX left. \
                     Period {} day(s). `!locker` lists it, `!locker {}` moves it into the \
                     {inv:?} tab.",
                    row.count,
                    row.name,
                    row.item_id,
                    row.price,
                    placed.slot,
                    row.period_days,
                    placed.slot
                ))
            }
            // The refusal is reported rather than swallowed. `add_nx` and `buy_cash_item`
            // both refuse instead of clamping, and a refusal nobody is told about is the
            // exact shape of the repeated-quest bug.
            Err(e) => self.gm_ack(format!(
                "!buy FAILED and NOTHING changed - no NX taken, no item placed: {e}"
            )),
        }
    }

    /// `!locker` - list the cash locker. `!locker <slot>` - move that slot into the bag.
    ///
    /// # The move is two transactions, so it needs an undo
    ///
    /// The locker and the bag are separate stores. Taking an item out of one and failing to
    /// put it in the other would simply destroy it, so the failure path puts it back with
    /// `store::put_cash_item`, which costs nothing. If **that** fails too the item really is
    /// gone, and the message says so in those words rather than reporting a tidy error - a
    /// player who loses an item needs to know immediately, not on the next login.
    pub(super) fn gm_locker(&mut self, arg: &str) -> Vec<Reply> {
        let Some(claimed) = self.claimed() else {
            return self
                .gm_ack("!locker REFUSED: no character is claimed on this connection.".to_string());
        };
        let account_id = claimed.account_id;
        let Some(chr) = self.claimed_character() else {
            return self.gm_ack("!locker REFUSED: no character is claimed.".to_string());
        };

        let held = self.store.cash_locker(account_id).unwrap_or_default();
        let Some(token) = arg.split_whitespace().next() else {
            if held.is_empty() {
                return self.gm_ack(
                    "The cash locker is empty. `!nx 10000` then `!buy 130200000` puts \
                     something in it."
                        .to_string(),
                );
            }
            let list = held
                .iter()
                .map(|l| {
                    format!(
                        "{}: {}x {} ({})",
                        l.slot,
                        l.item.kind.quantity(),
                        self.item_name(l.item.item_id),
                        l.item.item_id
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            let nx = self.store.cash_wallet(account_id).unwrap_or_default().nx;
            return self.gm_ack(format!("Cash locker ({nx} NX): {list}. `!locker <slot>` takes one."));
        };
        let Ok(slot) = token.parse::<u16>() else {
            return self.gm_ack(format!("!locker: {token:?} is not a slot number."));
        };

        let item = match self.store.take_cash_item(account_id, slot) {
            Ok(i) => i,
            Err(e) => return self.gm_ack(format!("!locker REFUSED and nothing moved: {e}")),
        };
        let Some(inv) = store::InventoryType::for_item(item.item_id) else {
            // Cannot happen for anything `!buy` placed, but the undo runs anyway rather than
            // leaving the item in a variable that is about to go out of scope.
            let _ = self.store.put_cash_item(account_id, &item);
            return self.gm_ack(format!(
                "!locker REFUSED: item {} is in no inventory tab. Put back in the locker.",
                item.item_id
            ));
        };
        let max_stack = self.config.shops.max_stack(item.item_id);

        let placed = match self.store.add_item(chr.id, inv, &item, max_stack) {
            Ok(rows) => rows,
            Err(e) => {
                return match self.store.put_cash_item(account_id, &item) {
                    Ok(back) => self.gm_ack(format!(
                        "!locker REFUSED: the {inv:?} tab would not take it ({e}). It is back \
                         in the locker, slot {}.",
                        back.slot
                    )),
                    Err(worse) => self.gm_ack(format!(
                        "!locker: the bag refused it ({e}) AND THE LOCKER WOULD NOT TAKE IT \
                         BACK ({worse}). ITEM {} IS LOST - say so.",
                        item.item_id
                    )),
                };
            }
        };

        let name = self.item_name(item.item_id);
        let mut out = self.gm_ack(format!(
            "Took locker slot {slot}: {}x {name} ({}) -> {inv:?} tab, slot {}",
            item.kind.quantity(),
            item.item_id,
            placed.iter().map(|r| r.slot.to_string()).collect::<Vec<_>>().join(", ")
        ));
        out.extend(self.inventory_added_replies(inv, &placed, "GM !locker"));
        out
    }

    /// `!item <itemId> [count]` - put an item in the bag, in the tab its id belongs to.
    ///
    /// **The owner asked for this as a safety net**, in these words: *"since most likely my item
    /// will disappear, I need to request a new GM command called `!item <itemID>` which will
    /// add that item into my inventory in the proper tab."* Dropping is not built, so an item
    /// dragged out of the window is currently refused rather than lost - but the moment
    /// dropping *is* built, this is what puts a mistake right.
    ///
    /// The tab comes from the id's leading digit (`store::InventoryType::for_item`), which is
    /// this game's own convention: 1 equip, 2 use, 3 setup, 4 etc, 5 cash.
    ///
    /// **An unknown id is refused rather than sent.** The client has to render whatever
    /// arrives, and an item body for an id with no `Item.wz` entry is exactly the kind of
    /// thing that has faulted it before.
    pub(super) fn gm_item(&mut self, arg: &str) -> Vec<Reply> {
        let mut parts = arg.split_whitespace();
        let Some(Ok(item_id)) = parts.next().map(str::parse::<u32>) else {
            return self.gm_ack(format!("!item: {arg:?} is not an item id. Try !item 1302000."));
        };
        let count: u16 = parts.next().and_then(|c| c.parse().ok()).unwrap_or(1).max(1);

        let Some(inv) = store::InventoryType::for_item(item_id) else {
            return self.gm_ack(format!(
                "!item REFUSED: {item_id} is not in any inventory tab - ids start 1..5."
            ));
        };
        if !self.config.item_names.contains_key(&item_id)
            && !self.config.shops.item_data.contains_key(&item_id)
        {
            return self.gm_ack(format!(
                "!item REFUSED: {item_id} is not in this client's Item.wz, so it has nothing to draw."
            ));
        }
        let Some(chr) = self.claimed_character() else {
            return self.gm_ack("!item REFUSED: no character is claimed on this connection.".to_string());
        };

        let is_equip = inv == store::InventoryType::Equip;
        let item = if is_equip {
            store::Item::equip(item_id)
        } else {
            store::Item::bundle(item_id, count)
        };
        let max_stack = self.config.shops.max_stack(item_id);

        let placed = match self.store.add_item(chr.id, inv, &item, max_stack) {
            Ok(rows) => rows,
            Err(e) => return self.gm_ack(format!("!item REFUSED: {e}")),
        };

        let name = self.item_name(item_id);
        let mut out = self.gm_ack(format!(
            "Giving {} {count}x {name} ({item_id}) -> {inv:?} tab, slot {}",
            chr.name,
            placed.iter().map(|r| r.slot.to_string()).collect::<Vec<_>>().join(", ")
        ));
        out.extend(self.inventory_added_replies(inv, &placed, "GM !item"));
        out
    }


    /// A map's name, for a line a person reads. The id alone if there is no table.
    pub(super) fn map_name(&self, map: u32) -> String {
        self.config.map_names.get(&map).cloned().unwrap_or_else(|| "unnamed".to_string())
    }


    /// An item's name, for a line a person reads.
    pub(super) fn item_name(&self, item_id: u32) -> String {
        self.config.item_names.get(&item_id).cloned().unwrap_or_else(|| "unnamed".to_string())
    }


    /// Acknowledge a GM command on screen.
    ///
    /// **The owner asked for every GM command to say what it is about to do**, rather than the
    /// only feedback being a refusal. A command that silently works and a command that
    /// silently does nothing look identical on screen, and telling them apart has cost
    /// launches.
    ///
    /// It goes out as [`net::notice::CHAT_NOTICE`], the same `0x00BB` a refusal already
    /// used. **What colour that renders is not established.** It reaches the chat window
    /// through printer type 7, which is also how the client's own `[Welcome] Welcome to
    /// MapleStory!!` line arrives - and that line is yellow on screen - so yellow is the
    /// expectation. It is an expectation, not a measurement, and the run will settle it.
    /// `net::notice` records that colour is not controllable through this packet.
    pub(super) fn gm_ack(&self, text: String) -> Vec<Reply> {
        self.notice(text)
    }


    /// Say something as the player: a balloon over the head and a line in the chat log.
    ///
    /// **An empty message is dropped rather than sent.** The client's own box will not
    /// submit one, so an empty `0x00E7` means something else is going on, and a balloon
    /// with no text is a worse answer than none.
    ///
    /// This is a **local echo, not a broadcast**: it goes back to the one connection that
    /// spoke. There is nobody else on the field to send it to yet - the server has no
    /// concept of a second player in a field - and saying so here is cheaper than
    /// rediscovering it when there is.
    pub(super) fn say_out_loud(&mut self, text: &str) -> Vec<Reply> {
        if text.is_empty() {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        vec![Reply {
            opcode: net::userchat::USER_CHAT,
            body: net::userchat::user_chat(chr.id, text),
            what: format!("UserChat: {} ({}) says {:?}", chr.id, chr.name, text),
        }]
    }
}
