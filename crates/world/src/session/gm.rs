//! The chat box: what the player says out loud, and the `!` commands typed into it.
//!
//! Every command acknowledges itself. A command that silently works and a command that
//! silently does nothing look identical on screen, and telling them apart has cost
//! client launches.

use super::*;

/// What `!resetap` puts STR, DEX, INT and LUK back to.
///
/// The owner, 2026-08-29: *"When !resetap runs, the character should only have 4, 4, 4, 4 in STR,
/// DEX, INT and LUK, that represents the lowest amount of AP available for characters to
/// increase from."*
///
/// **Not `net::opcode::Character::default()`, which is 12/5/4/4.** Those are the stats a
/// character is *created* with, and they are the right answer for creation - but a reset that
/// stops at 12 STR leaves eight points stranded in a stat the player may not want, which is
/// exactly the situation `!resetap` exists to get out of. The floor is the lowest value the
/// game lets a stat reach, so every point above it is refundable by definition.
///
/// The two numbers are deliberately separate: changing creation stats and changing the reset
/// floor are different decisions, and sharing a constant would couple them.
pub(super) const AP_RESET_FLOOR: u16 = 4;

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
    /// Does the account this connection is served as hold GM status?
    ///
    /// **`false` is the answer to every uncertainty** - no claimed migration, a database that
    /// will not answer, an account that has been deleted. A permission check that fails open
    /// is not a permission check, and the cost of failing closed here is one chat line telling
    /// a real GM to run `--gm`.
    pub(super) fn account_is_gm(&self) -> bool {
        let Some(claimed) = self.claimed() else { return false };
        match self.store.is_gm(claimed.account_id) {
            Ok(yes) => yes,
            Err(e) => {
                crate::server::log(&format!(
                    "gm: could not read GM status for account {}: {e} - refusing the command",
                    claimed.account_id
                ));
                false
            }
        }
    }

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
        // **Every command is gated on the account's GM flag.** The owner, 2026-08-29: *"can you
        // please make GM commands only available to accounts with GM status? All commands
        // should have this gate for now until otherwise specified."* All of them, including
        // `!help` and the read-only `!rates` - "until otherwise specified" is the instruction,
        // and a gate with quiet exceptions is the shape that gets found by accident later.
        //
        // **This is authorisation, not authentication, and the difference is the whole
        // caveat.** The game socket carries no credentials: which account this connection is
        // served as comes from a launcher claim, not from anything the client proved. So this
        // stops a *second account on this machine* from using `!item`; it stops nothing that
        // can reach the port. Say so when reporting it.
        //
        // The refusal is a chat line rather than silence, for the reason every refusal here
        // is: "nothing happened" with no explanation is indistinguishable from a broken
        // command, and that has cost this project two rounds of investigation already.
        if !self.account_is_gm() {
            return self.gm_ack(format!(
                "!{name} is a GM command and this account does not have GM status. Grant it \
                 with: maplecw-useradd --gm <account>"
            ));
        }

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
            // Put spent points back in the pool. See gm_reset_ap for why the AP one
            // conserves the total rather than recomputing it from the level.
            "resetap" => self.gm_reset_ap(),
            "resetsp" => self.gm_reset_sp(),
            "learn" => self.gm_learn(arg),
            "kit" => self.gm_kit(arg),
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
        // **Warn when the job does not match the stats.** The owner, 2026-08-28, after putting
        // seven points into Magic Claw: *"the skill only deals 1 damage, which is definitely
        // not correct."* It was correct. They were a Rogue - LUK 36, DEX 24, INT 6 - and `!job
        // 200` changes the job number without moving a single ability point. `MagicTotal`
        // seeds from `floor(INT/2)`, so their whole damage window sat between 1 and 2 before
        // the mob's magic defence was even applied.
        //
        // The formula was right and the character was wrong, and there was nothing on screen
        // to say so. `!job` is a debug command and deliberately skips `jobs::advancement_for`,
        // which would have refused - so it warns instead of refusing, because refusing would
        // break the command's whole purpose.
        let mismatch = crate::jobs::FIRST_JOBS
            .iter()
            .find(|j| j.job == job)
            .map(|j| (j.stat, j.stat.of(&chr)))
            .filter(|(_, have)| *have < crate::jobs::STAT_MINIMUM);
        let warning = match mismatch {
            Some((stat, have)) => format!(
                " *** WARNING: you have {have} {} and this job wants {}. !job does NOT move                  ability points, so its skills will compute almost no damage - a magic attack                  with {have} INT lands on the damage floor of 1 whatever level the skill is.                  That is the formula being right, not a bug. ***",
                stat.label(),
                crate::jobs::STAT_MINIMUM
            ),
            None => String::new(),
        };
        let mut out = self.gm_ack(format!(
            "{} is now job {job} (was {was}). Expect the JobChanged effect AND its sound - unless job is 0, which the client's own gate suppresses.{warning}",
            chr.name
        ));
        // **The skill points, in the same packet as the job.**
        //
        // The owner, 2026-08-27: *"Once I became a Magician (or any job at level 10), I should
        // immediately get 1 skill point for 1st job. Currently I get none."*
        //
        // They got none because this packet used to carry **bit 5 alone**. `FUN_1407E4D70` is
        // what prints the number beside `sp` and what greys the `+` button (`test r14,r14 /
        // jle` at `0x142584E4F` disables `BtSpUp`), and its extended path looks the pool up by
        // TIER. No pool, no number, dead button - with nothing on screen to say why.
        //
        // **One packet, not two.** `crates/net/src/combat.rs` used to claim a combined packet
        // decodes SP with the OLD job, labelled [L]; it is wrong and now says so. The job arm
        // stores `charstat+0x33` at `0x1402CBC04` and the SP fork loads it 405 bytes later,
        // straight-line, so the new job is what picks the encoding.
        //
        // **The amount is computed from the LEVEL, and nothing is persisted yet.** That is a
        // real limitation, not an oversight: `world::skillpoints::entitlement` is a total owed
        // rather than an increment, so re-sending it is idempotent and advancing late pays the
        // same as advancing early. What is missing is a record of what has been SPENT - so
        // until `0x013B` persists, points come back on the next advancement. Said out loud
        // because a player who spends and then sees them return will otherwise report a bug.
        out.push(self.job_change_reply(was, job));
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

    /// `!resetap` - put every spent ability point back in the pool.
    ///
    /// The owner, 2026-08-28, after `!job 200` left a Rogue's stats on a Magician: *"Please
    /// implement two more GM commands to reset the ability points and the skill points."*
    ///
    /// # It conserves the total rather than recomputing it
    ///
    /// The obvious implementation is "work out how many points a level-N character should
    /// have and set that". It is also the wrong one: **nothing here knows the per-level AP
    /// award**, and inventing a number would either create points or destroy them, silently,
    /// in a command whose whole job is to be safe to run.
    ///
    /// So this does arithmetic that cannot be wrong in either direction. Every stat goes back
    /// to [`AP_RESET_FLOOR`] and the difference lands in the pool:
    ///
    /// ```text
    /// refund = (str + dex + int + luk) - (4 + 4 + 4 + 4)
    ///        + ap_spent_hp + ap_spent_mp
    /// ap     = ap + refund
    /// ```
    ///
    /// Total points in and total points out are equal by construction. Run it twice and the
    /// second run refunds zero: the stats are already at the floor and the HP/MP counters were
    /// cleared by the first.
    ///
    /// **A stat below the floor is left alone rather than "corrected" upward.** That would be
    /// creating points out of a character this server had already got wrong, and a reset that
    /// can hand out free stats is worse than one that occasionally refunds nothing.
    ///
    /// # HP and MP need a ledger; the four stats do not
    ///
    /// A stat's value *is* the record of what was spent on it. `max_hp` is not: it also grows
    /// on level-up, and the stored number does not say which part came from where, so
    /// `max_hp / MAX_HP_PER_AP` would refund the character's entire level history as ability
    /// points. The points are counted when they are spent instead - `store::abilityspend`,
    /// incremented by `session::ability`.
    ///
    /// **Points spent on HP before that column existed are not refundable.** They really are
    /// indistinguishable from level-up HP, which is the whole reason the column had to exist.
    pub(super) fn gm_reset_ap(&mut self) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else {
            return self
                .gm_ack("!resetap REFUSED: no character is claimed on this connection.".to_string());
        };
        let floor = AP_RESET_FLOOR;
        let (was_str, was_dex, was_int, was_luk) =
            (chr.strength, chr.dexterity, chr.intelligence, chr.luck);
        let (was_max_hp, was_max_mp) = (chr.max_hp, chr.max_mp);

        // `saturating_sub` on each stat separately, so one stat already under the floor
        // cannot eat another stat's refund.
        let stat_refund = was_str.saturating_sub(floor)
            + was_dex.saturating_sub(floor)
            + was_int.saturating_sub(floor)
            + was_luk.saturating_sub(floor);

        // **Read and cleared in one transaction**, so a reset cannot count the points, fail to
        // clear them, and hand the same ones out again on the next run.
        let hpmp = match self.store.take_ap_spend(chr.id) {
            Ok(spend) => spend,
            Err(e) => return self.gm_ack(format!("!resetap FAILED and nothing changed: {e}")),
        };
        let refund = stat_refund.saturating_add(u16::try_from(hpmp.total()).unwrap_or(u16::MAX));

        chr.strength = if was_str > floor { floor } else { was_str };
        chr.dexterity = if was_dex > floor { floor } else { was_dex };
        chr.intelligence = if was_int > floor { floor } else { was_int };
        chr.luck = if was_luk > floor { floor } else { was_luk };

        // The HP and MP those points bought, removed with the SAME constant that granted it,
        // so the two cannot drift. Never below 1 max HP: a character with a zero-length HP bar
        // is dead in a way nothing here knows how to undo.
        chr.max_hp = chr
            .max_hp
            .saturating_sub(hpmp.hp.saturating_mul(net::abilityup::policy::MAX_HP_PER_AP))
            .max(1);
        chr.max_mp = chr
            .max_mp
            .saturating_sub(hpmp.mp.saturating_mul(net::abilityup::policy::MAX_MP_PER_AP));
        // Current cannot exceed maximum, or the bar draws past its own end.
        chr.hp = chr.hp.min(chr.max_hp);
        chr.mp = chr.mp.min(chr.max_mp);
        chr.ap = chr.ap.saturating_add(refund);

        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.gm_ack(format!("!resetap FAILED and nothing changed: {e}"));
        }

        let hpmp_note = if hpmp.total() > 0 {
            format!(
                ", plus {} point(s) out of max HP ({was_max_hp}->{}) and {} out of max MP \
                 ({was_max_mp}->{})",
                hpmp.hp, chr.max_hp, hpmp.mp, chr.max_mp
            )
        } else {
            String::new()
        };
        let mut out = self.gm_ack(format!(
            "Ability points reset. STR {was_str}->{}, DEX {was_dex}->{}, INT {was_int}->{}, \
             LUK {was_luk}->{}{hpmp_note} - {refund} points back, {} to spend. The totals \
             match by construction: nothing was created or destroyed.",
            chr.strength, chr.dexterity, chr.intelligence, chr.luck, chr.ap
        ));
        // **One packet with all five fields.** The stat window reads them together, and five
        // packets would let it redraw against a half-applied state.
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                strength: Some(chr.strength),
                dexterity: Some(chr.dexterity),
                intelligence: Some(chr.intelligence),
                luck: Some(chr.luck),
                ap: Some(chr.ap),
                // Only when they moved. Restating an unchanged max HP is what `regen_tick`
                // was corrected for, and it costs a redraw for nothing.
                max_hp: (chr.max_hp != was_max_hp).then_some(chr.max_hp),
                max_mp: (chr.max_mp != was_max_mp).then_some(chr.max_mp),
                hp: (chr.max_hp != was_max_hp).then_some(chr.hp),
                mp: (chr.max_mp != was_max_mp).then_some(chr.mp),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: ability reset - STR/DEX/INT/LUK back to the floor of \
                 {floor}/{floor}/{floor}/{floor}, {} point(s) out of max HP and {} out of max \
                 MP, {refund} refunded into AP, now {}",
                hpmp.hp, hpmp.mp, chr.ap
            ),
        });
        out
    }

    /// `!resetsp` - unlearn every skill, so the points can go somewhere else.
    ///
    /// # Why this gives the points back without touching a counter
    ///
    /// It does not refund anything, and that is deliberate. `world::skillpoints` computes an
    /// **entitlement** - a total owed at the character's level - rather than tracking a
    /// balance, so the pool the client is shown is already "everything you have ever earned".
    /// Erasing the skills is therefore the whole reset: the points were never subtracted from
    /// a stored number, so there is nothing to add back.
    ///
    /// That also means this is honest about the limitation rather than papering over it: with
    /// spending unpersisted, a skill's level **is** the only record that a point was spent.
    ///
    /// **The client is told, or it keeps drawing the old levels.** `SkillChange::Forget`
    /// encodes as a negative level, which `net::skills` records as the only thing that reaches
    /// the client's erase arm - a `Learn` at level 0 would not.
    pub(super) fn gm_reset_sp(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self
                .gm_ack("!resetsp REFUSED: no character is claimed on this connection.".to_string());
        };
        let learned = self.store.skills(chr.id).unwrap_or_default();
        if learned.is_empty() {
            return self.gm_ack("!resetsp: no skills to forget.".to_string());
        }

        // **Forget and refund in ONE transaction, or this command's own chat line is a lie.**
        //
        // It used to loop `set_skill_level(.., 0)`, which was right while nothing tracked a
        // balance: the pool was recomputed from LEVEL, so forgetting a skill *was* the refund.
        // With the ledger that loop would erase the levels and leave the pool charged, and the
        // points would NOT come back - while the line below went on promising they would.
        //
        // **Reported, not swallowed**, and all-or-nothing: a reset that silently half-ran
        // would leave the client and the database disagreeing about what is learned, and the
        // player would find out by clicking a skill that no longer exists.
        let refunded = match self.store.forget_all_skills_and_refund(chr.id) {
            Ok(r) => r,
            Err(e) => return self.gm_ack(format!("!resetsp FAILED and changed nothing: {e}")),
        };
        let forgotten: Vec<net::skills::SkillChange> =
            learned.iter().map(|s| net::skills::SkillChange::Forget { id: s.id }).collect();
        let failed: Vec<String> = Vec::new();

        let named: Vec<String> = learned
            .iter()
            .map(|s| format!("{} lv{}", self.skill_name(s.id), s.level))
            .collect();
        let mut out = self.gm_ack(format!(
            "Forgot {} skill(s): {}. Refunded {} skill point(s) - the forget and the refund are \
             ONE transaction, so a forgotten skill IS the whole refund.{}",
            forgotten.len(),
            named.join(", "),
            refunded.points,
            if failed.is_empty() {
                String::new()
            } else {
                format!(" *** BUT THESE FAILED and are still learned: {} ***", failed.join(", "))
            }
        ));
        if !forgotten.is_empty() {
            out.push(self.skill_reply(
                net::skills::change_skill_record_result(true, true, &forgotten),
                format!("!resetsp forgot {} skill(s)", forgotten.len()),
            ));
        }
        // **The refunded pool has to reach the screen.** The client never increments a pool
        // itself, so without this the points are back in the database and the window still
        // says zero - which reads as the refund not happening.
        out.extend(self.skill_point_reply(&chr));
        out
    }

    /// `!learn [level]` or `!learn <skillId> <level>` - put every skill of this job's book on
    /// the bar, without spending a single skill point.
    ///
    /// # Why this exists
    ///
    /// The owner, 2026-08-28: *"I need all 1st job skills of all branches to have their damage
    /// calculation ready and their skills available to test next session."* That is **24
    /// skills across four branches**, and reaching them through `0x013B` means levelling to
    /// earn the points, four times over, inside one run that costs a manual launch.
    ///
    /// `!resetsp`'s own report already says why the points are not the obstacle worth
    /// engineering around: *"this server computes the pool from your LEVEL rather than
    /// tracking a balance"*. So a bulk grant is not a cheat against a balance that exists -
    /// there is no balance yet. It is the same `0x0081` the `+` button produces, sent for
    /// several skills at once.
    ///
    /// # It refuses exactly what `0x013B` refuses, by calling the same predicate
    ///
    /// [`crate::skilltable::SkillTable::book`] filters on `may_learn`, the function
    /// `on_skill_up` uses. Two copies of one rule is how one of them ends up wrong - the
    /// Heena quest paid for that lesson with a farming loop - so there is deliberately not a
    /// second list of "what a Bowman may have" anywhere in this file.
    ///
    /// # The clamp is the skill's own ceiling, not one constant
    ///
    /// `BEGINNER_SKILL_MAX_LEVEL` is 3, right for Three Snails and wrong for all 24 of these:
    /// the first-job books run to 15 and 20. Each skill is clamped to its **own** `maxLevel`,
    /// so `!learn 20` gives Magic Guard 15 and Magic Claw 20 in the same breath.
    pub(super) fn gm_learn(&mut self, arg: &str) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self
                .gm_ack("!learn REFUSED: no character is claimed on this connection.".to_string());
        };
        // **A missing table is a refusal with a fix in it, not a silent no-op.** The file is
        // gitignored and regenerable, so the one thing this must never do is look like the
        // job has no skills.
        if self.config.skills.is_empty() {
            return self.gm_ack(
                "!learn REFUSED: no skill table is loaded, so this server does not know what \
                 any job may learn. Regenerate it from the repo root with: python \
                 tools/dump_skills.py - then restart the world server."
                    .to_string(),
            );
        }

        let mut fields = arg.split_whitespace();
        let (targets, asked): (Vec<u32>, Option<u32>) = match (fields.next(), fields.next()) {
            // `!learn` - the whole book, each at its own maximum.
            (None, _) => (self.config.skills.book(chr.job).iter().map(|s| s.id).collect(), None),
            // `!learn <level>` - the whole book, capped.
            (Some(one), None) => match one.parse::<u32>() {
                Ok(level) => {
                    (self.config.skills.book(chr.job).iter().map(|s| s.id).collect(), Some(level))
                }
                Err(_) => {
                    return self.gm_ack(format!(
                        "!learn: {one:?} is not a level. Try !learn (whole book at maximum), \
                         !learn 5, or !learn 2001003 7."
                    ))
                }
            },
            // `!learn <skillId> <level>` - one skill.
            (Some(id), Some(level)) => match (id.parse::<u32>(), level.parse::<u32>()) {
                (Ok(id), Ok(level)) => (vec![id], Some(level)),
                _ => {
                    return self.gm_ack(format!(
                        "!learn: {id:?} {level:?} is not a skill id and a level."
                    ))
                }
            },
        };

        if targets.is_empty() {
            return self.gm_ack(format!(
                "!learn: job {} has no skill book in this client's Skill.wz. The four first \
                 jobs are 100, 200, 300 and 400 - try !job 300 first.",
                chr.job
            ));
        }

        let mut changes = Vec::new();
        let mut granted = Vec::new();
        let mut refused = Vec::new();
        let mut failed = Vec::new();
        for id in targets {
            if !self.config.skills.may_learn(chr.job, id) {
                refused.push(match self.config.skills.get(id) {
                    Some(s) => format!("{} ({}) is in job book {}", id, s.name, s.job),
                    None => format!("{id} is not in this client's Skill.wz"),
                });
                continue;
            }
            // **Clamped to this skill's ceiling, and `0` is not a level.** A `!learn 0` is a
            // forget, which `!resetsp` already does properly with `SkillChange::Forget`;
            // sending `Learn` at level 0 would tell the client to draw a skill it cannot.
            let Some(ceiling) = self.config.skills.max_level(id) else {
                refused.push(format!("{id} has no level rows in the table"));
                continue;
            };
            let level = asked.unwrap_or(ceiling).min(ceiling);
            if level == 0 {
                refused.push(format!("{id}: level 0 is a forget - use !resetsp"));
                continue;
            }
            match self.store.set_skill_level(chr.id, id, level) {
                Ok(()) => {
                    changes.push(net::skills::SkillChange::Learn(net::skills::Skill::at_level(
                        id, level,
                    )));
                    granted.push(format!("{} lv{level}", self.skill_name(id)));
                }
                // Reported, never swallowed - same reason as `!resetsp`. A half-run grant
                // leaves the client and the database disagreeing about what exists.
                Err(e) => failed.push(format!("{id} ({e})")),
            }
        }

        if changes.is_empty() {
            return self.gm_ack(format!(
                "!learn granted nothing. {}{}",
                if refused.is_empty() {
                    String::new()
                } else {
                    format!("Refused: {}. ", refused.join("; "))
                },
                if failed.is_empty() {
                    String::new()
                } else {
                    format!("Failed: {}.", failed.join("; "))
                }
            ));
        }

        // **Say when the stats will make every one of these land on the damage floor.** This
        // is the same warning `!job` and `on_skill_up` carry, and it is repeated here because
        // this is the command that will be typed immediately before the owner swings. They spent a
        // session concluding Magic Claw was broken when they were a Rogue with 6 INT.
        let warning = crate::jobs::FIRST_JOBS
            .iter()
            .find(|j| j.job == chr.job)
            .map(|j| (j.stat, j.stat.of(&chr)))
            .filter(|(_, have)| *have < crate::jobs::STAT_MINIMUM)
            .map(|(stat, have)| {
                format!(
                    " *** WARNING: you have {have} {}. Every attack skill here scales on it, \
                     so they will all land on the damage floor of 1 whatever level they are. \
                     That is the formula being right. Raise {} first. ***",
                    stat.label(),
                    stat.label()
                )
            })
            .unwrap_or_default();

        let mut out = self.gm_ack(format!(
            "Learned {} skill(s) for job {}: {}.{}{}{}",
            changes.len(),
            chr.job,
            granted.join(", "),
            if refused.is_empty() {
                String::new()
            } else {
                format!(" Refused: {}.", refused.join("; "))
            },
            if failed.is_empty() {
                String::new()
            } else {
                format!(" *** FAILED: {} ***", failed.join("; "))
            },
            warning
        ));
        out.push(self.skill_reply(
            net::skills::change_skill_record_result(true, true, &changes),
            format!("!learn granted {} skill(s) to job {}", changes.len(), chr.job),
        ));
        if !warning.is_empty() {
            out.extend(self.notice(warning.trim().trim_matches('*').trim().to_string()));
        }
        out
    }

    /// `!kit` - hand over everything this job needs to cast its own skills.
    ///
    /// # Why a command and not a note in the test plan
    ///
    /// Five of the 24 first-job skills carry a **weapon gate** in this client's `Skill.wz`,
    /// measured over all four `weapon` columns: Arrow Blow, Double Shot and Power Knockback
    /// want 45 or 46 (bow or crossbow), Double Stab wants 33 (dagger), Lucky Seven wants 47
    /// (claw). Without the right item in hand the client refuses the cast itself, and on
    /// screen that is indistinguishable from a server that never implemented the skill.
    ///
    /// A run that discovers this costs the owner a manual launch. Typing four `!item` lines off a
    /// plan costs a transcription error.
    ///
    /// # It warns about what cannot be equipped, which is the half that matters
    ///
    /// **There is no zero-requirement bow, crossbow or claw in this client, and no free
    /// throwing star** - `crate::loadout` read all 230 weapon images from the WZ to establish
    /// that, because `gm-handbook/equips.txt` is missing the requirement columns entirely.
    /// Anything advanced through `jobs::advancement_for` clears its own kit, since
    /// `LEVEL_MINIMUM` is 10 and `STAT_MINIMUM` is 35 against the bow's 25.
    ///
    /// **`!job` bypasses that check**, and `!job` is how these branches will be reached. So a
    /// character can end up holding a bow it cannot equip, which reads on screen as the skill
    /// being broken. `Loadout::unequippable` names the failing clause instead.
    pub(super) fn gm_kit(&mut self, _arg: &str) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self
                .gm_ack("!kit REFUSED: no character is claimed on this connection.".to_string());
        };
        let Some(kit) = crate::loadout::loadout_for(chr.job) else {
            return self.gm_ack(format!(
                "!kit: job {} has no loadout. The four first jobs are 100 (Warrior), 200 \
                 (Magician), 300 (Bowman) and 400 (Thief) - try !job 300 first.",
                chr.job
            ));
        };
        // **An empty kit is a measurement, not a failure.** No Magician skill carries a
        // `weapon` column at all - confirmed over all four columns on all six skills - so the
        // honest answer is "nothing", said out loud.
        if kit.needs_nothing() {
            return self.gm_ack(format!(
                "!kit: {} needs NOTHING. None of its six skills carries a weapon column in \
                 this client's Skill.wz, so every one of them casts bare-handed. That is \
                 measured, not an empty table.",
                kit.job_name
            ));
        }

        let mut out = Vec::new();
        let mut lines = Vec::new();
        for piece in kit.pieces {
            match self.give_item(piece.item_id, piece.quantity, "GM !kit") {
                Ok((line, replies)) => {
                    lines.push(format!("{line} - {}", piece.why));
                    out.extend(replies);
                }
                // Reported, never swallowed. A half-delivered kit that says nothing is how a
                // missing arrow becomes "Double Shot is broken".
                Err(why) => lines.push(format!("{} ({}) {why}", piece.name, piece.item_id)),
            }
        }

        // The equip check runs on the character as it is now, after the grants.
        let now = self.claimed_character().unwrap_or(chr);
        let blocked = kit.unequippable(&now);
        let warning = if blocked.is_empty() {
            String::new()
        } else {
            let each: Vec<String> = blocked
                .iter()
                .map(|(p, unmet)| format!("{} needs {}", p.name, unmet.join(" and ")))
                .collect();
            format!(
                " *** WARNING: you cannot EQUIP {}. !job does not move ability points, so the \
                 skills these gate will refuse to cast and it will look like the server. Use \
                 !resetap and raise the stat. ***",
                each.join("; ")
            )
        };

        let mut replies = self.gm_ack(format!(
            "!kit for {} (job {}): {}.{}{}",
            kit.job_name,
            kit.job,
            lines.join(" | "),
            kit.caveat.map(|c| format!(" NOTE: {c}")).unwrap_or_default(),
            warning
        ));
        replies.extend(out);
        if !warning.is_empty() {
            replies.extend(self.notice(warning.trim().trim_matches('*').trim().to_string()));
        }
        replies
    }

    /// **The one packet that changes a job**, shared by `!job` and the NPC instructors.
    ///
    /// Extracted on 2026-08-28 when the instructors were wired. A second copy of this would
    /// have been the third time in this project that one rule lived in two places - the Heena
    /// quest and `may_learn` are the other two - and the failure mode is always that one copy
    /// gets a fix and the other does not.
    ///
    /// It is **one** `0x007C`, mask bit 5 plus the SP bit. The client picks the SP encoding
    /// from the **new** job, because the job arm stores `charstat+0x33` before the SP fork
    /// reads it, 405 bytes later and straight-line. Do **not** also send `0x02D1`: this packet
    /// fires the `JobChanged` effect and its sound by itself, and two would stack.
    ///
    /// The pool key is a **TIER**, not a job id - `FUN_1402CB030` returns 0 for any key above
    /// 10, so a job id reads an empty pool and greys the `+` button with nothing on screen to
    /// say why.
    pub(super) fn job_change_reply(&self, was: u16, job: u16) -> Reply {
        let level = self.claimed_character().map(|c| c.level).unwrap_or(0);
        let mut pools = Vec::new();
        for tier in [crate::skillpoints::Tier::First, crate::skillpoints::Tier::Second] {
            let amount = crate::skillpoints::entitlement(tier, level);
            if amount > 0 {
                pools.push(net::stats::SpPool {
                    job_level: net::stats::tier_for_job(match tier {
                        crate::skillpoints::Tier::First => 100,
                        crate::skillpoints::Tier::Second => 110,
                    }),
                    amount,
                });
            }
        }
        let owed: Vec<String> =
            pools.iter().map(|p| format!("tier {} = {}", p.job_level, p.amount)).collect();

        // **The encoding forks on the job, so let the type check rather than assume.** Every
        // first job takes the extended branch, but a job outside the explorer tree takes a
        // plain `u16` - and sending the wrong shape would desynchronise the rest of the
        // packet, not merely lose the points.
        let table = net::stats::Sp::Extended(pools);
        let sp = table.matches_job(job).then_some(table);
        let sp_note = match &sp {
            Some(_) => format!("AND the skill points [{}]", owed.join(", ")),
            None => format!(
                "and NO SP: job {job} takes the PLAIN u16 encoding and this server only \
                 computes the extended table. The wrong shape would desynchronise the packet"
            ),
        };
        Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { job: Some((job, 0)), sp, ..Default::default() }
                .build(),
            what: format!(
                "StatChanged: job {was} -> {job}, {sp_note}. One packet, mask bit 5 plus the \
                 SP bit. The client plays JobChanged itself from this packet - no 0x02D1 is \
                 sent, deliberately"
            ),
        }
    }

    /// A skill's name, for a line a person reads. The id alone if the table is not loaded.
    pub(super) fn skill_name(&self, skill_id: u32) -> String {
        self.config
            .skills
            .get(skill_id)
            .map(|s| s.name.clone())
            .unwrap_or_else(|| format!("skill {skill_id}"))
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
                // **Leaf Points, not NX.** This line said "for 100 NX ... 10000 NX left" on
                // 2026-08-26 while the database correctly went 99,000 -> 98,900 LP. The
                // purchase was right and only the sentence was wrong, which is the worst
                // direction: a reader would have concluded the debit had not happened.
                let lp = self.store.cash_wallet(account_id).unwrap_or_default().maple_points;
                self.gm_ack(format!(
                    "Bought SN {sn}: {}x {} ({}) for {} LP. Locker slot {}, {lp} LP left. \
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
            let lp = self.store.cash_wallet(account_id).unwrap_or_default().maple_points;
            return self.gm_ack(format!("Cash locker ({lp} LP): {list}. `!locker <slot>` takes one."));
        };
        let Ok(slot) = token.parse::<u16>() else {
            return self.gm_ack(format!("!locker: {token:?} is not a slot number."));
        };

        let item = match self.store.take_cash_item(account_id, slot) {
            Ok(i) => i,
            Err(e) => return self.gm_ack(format!("!locker REFUSED and nothing moved: {e}")),
        };
        // **The guard that would have saved a client launch.** A pet id sent as a bundle
        // kills this client - see net::inventory::is_pet. Put it back rather than hand it
        // over; the locker is server-side and the client never sees what is in it.
        if net::inventory::is_pet(item.item_id) {
            let back = self.store.put_cash_item(account_id, &item);
            return self.gm_ack(format!(
                "!locker REFUSED: {} is a PET, and this server cannot build a pet item body                  yet - sending one as a bundle kills the client (it did, on 2026-08-26).                  It is back in the locker{}.",
                item.item_id,
                match back {
                    Ok(l) => format!(", slot {}", l.slot),
                    Err(e) => format!(" - EXCEPT IT WOULD NOT GO BACK: {e}. Say so"),
                }
            ));
        }
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
        match self.give_item(item_id, count, "GM !item") {
            Ok((line, replies)) => {
                let mut out = self.gm_ack(line);
                out.extend(replies);
                out
            }
            Err(why) => self.gm_ack(why),
        }
    }

    /// **Hand one item over: every refusal, the row, and the packets.** Shared by `!item` and
    /// `!kit`.
    ///
    /// `session/inventory.rs` records that this path *already existed twice and drifted*.
    /// A third copy is how one of them ends up missing the pet guard - the guard that exists
    /// because sending a pet as a bundle throws `0xE06D7363` and kills the client. So `!kit`
    /// adds no new wire path at all; it calls this in a loop.
    ///
    /// `Ok` carries the line to print and the packets to send. `Err` carries the refusal,
    /// already worded for a person.
    fn give_item(
        &mut self,
        item_id: u32,
        count: u16,
        why: &str,
    ) -> Result<(String, Vec<Reply>), String> {
        let count = count.max(1);
        if net::inventory::is_pet(item_id) {
            return Err(format!(
                "REFUSED: {item_id} is a PET. This server cannot build a pet item body yet, \
                 and sending one as a bundle kills the client - measured 2026-08-26, \
                 net::inventory::is_pet has the mechanism."
            ));
        }
        let Some(inv) = store::InventoryType::for_item(item_id) else {
            return Err(format!(
                "REFUSED: {item_id} is not in any inventory tab - ids start 1..5."
            ));
        };
        if !self.config.item_names.contains_key(&item_id)
            && !self.config.shops.item_data.contains_key(&item_id)
        {
            return Err(format!(
                "REFUSED: {item_id} is not in this client's Item.wz, so it has nothing to draw."
            ));
        }
        let Some(chr) = self.claimed_character() else {
            return Err("REFUSED: no character is claimed on this connection.".to_string());
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
            Err(e) => return Err(format!("REFUSED: {e}")),
        };

        let name = self.item_name(item_id);
        let line = format!(
            "Giving {} {count}x {name} ({item_id}) -> {inv:?} tab, slot {}",
            chr.name,
            placed.iter().map(|r| r.slot.to_string()).collect::<Vec<_>>().join(", ")
        );
        let replies = self.inventory_added_replies(inv, &placed, why);
        Ok((line, replies))
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
