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
            // **The line goes out as chat either way**, and if it happens to be one of the
            // summoned pet's commands the pet answers beside it. The owner, 2026-09-13, measured:
            // typing `bad` sends an ordinary `0x00E7` and no pet packet, so the pet's response
            // is the server's to add. `session/pet.rs`.
            let mut out = self.say_out_loud(text);
            out.extend(self.pet_command_replies(text));
            return out;
        };
        let (name, arg) = match command.split_once(char::is_whitespace) {
            Some((n, a)) => (n, a.trim()),
            None => (command, ""),
        };
        // **Three commands are for everyone.** The owner, 2026-09-06: *"`!rates` should be kept
        // because it should be a public command that anyone can execute"*, and *"`!help`
        // should also display the commands that the player/GM can execute. A player should
        // only be shown commands that they are allowed to execute."* So these are
        // answered before the gate, and `!help` answers from the list that matches the
        // caller - a player is never told which GM words exist.
        //
        // **`!tool` is the third, and it is public on purpose.** The owner, 2026-09-08: *"Introduce
        // a new public command !tool"*. It must sit above the `is_gm` gate or a player typing it
        // would have it said out loud as ordinary chat, which is what happens to every GM word.
        // It grants nothing by itself - it draws a box - and the three favours behind it are
        // gated per UTC day in the database, so spamming the command costs boxes and not
        // allowance. `crate::dailyperks::COMMAND`.
        let is_gm = self.account_is_gm();
        match name {
            "rates" => return self.gm_rates(),
            // The owner, 2026-09-14: *"`!online` - available to everyone, list all characters that
            // are currently online across all channels."* Names and channels only; where
            // each one IS is `!track`, and that is a GM word.
            "online" => return self.who_is_online(),
            crate::dailyperks::COMMAND => return self.open_daily_perks(),
            // Same shape as !tool and open to everyone: the owner asked for a command that
            // "functions very similar to !tool" with the Administrator's dialogue.
            crate::scrollnpc::COMMAND => return self.open_scroll_picker(),
            // `!giftdrop` alone opens the caller's own box; with arguments it is a GM word
            // that queues a gift, and a non-GM typing that is said out loud like any other.
            // crate::giftdrop.
            crate::giftdrop::COMMAND => return self.giftdrop_command(arg, is_gm, text),
            "help" => {
                return self.gm_ack(if is_gm { GM_COMMANDS.to_string() } else { PLAYER_COMMANDS.to_string() })
            }
            _ => {}
        }
        // **Every other command is gated on the account's GM flag.** The owner, 2026-08-29: *"can
        // you please make GM commands only available to accounts with GM status? All commands
        // should have this gate for now until otherwise specified."* The two exceptions above
        // are the "otherwise specified", dated.
        //
        // **This is authorisation, not authentication, and the difference is the whole
        // caveat.** The game socket carries no credentials: which account this connection is
        // served as comes from a launcher claim, not from anything the client proved. So this
        // stops a *second account on this machine* from using `!item`; it stops nothing that
        // can reach the port. Say so when reporting it.
        //
        // **A refused command is SAID OUT LOUD, not answered with a refusal.** The owner,
        // 2026-08-29: *"For items that are GM account specific, if the ! commands do not
        // work, please make sure that it is sent as a normal chat message."*
        //
        // Which is the right way round, and better than the system notice this used to send.
        // To an account that cannot run commands, `!heal` is not a refused command - it is a
        // person typing text that begins with an exclamation mark, and the game's answer to
        // typed text is to say it. It also means the server never tells a non-GM which `!`
        // words are real, which the old message did by naming the flag that grants them.
        //
        // The "always answer" rule is still satisfied: `say_out_loud` sends `0x0231`, so the
        // client draws a balloon and a chat line exactly as it would for "Hello". Silence
        // here would be the failure - the client draws nothing for its own chat, so a
        // dropped line is invisible, which is what the owner hit with "Hello", "Hello2" and
        // "Hello3" on 2026-08-19.
        if !is_gm {
            return self.say_out_loud(text);
        }

        // **Pruned 2026-09-06 on the owner's instruction.** Gone: the per-kind rate setters
        // (`!setrates` covers all three), `!migsweep`, `!npcfx`, `!buff`, `!unbuff`, and
        // `!buy`, `!locker`, `!kit` - *"quite useless when I can spawn items"*. `!heal` stays
        // as the one instant refill for damage tests.
        match name {
            "map" => self.gm_map(arg),
            "item" => self.gm_item(arg),
            // One gift per account, claimable on any of its characters. crate::giftdrop.
            crate::giftdrop::COMMAND_ALL => self.giftall_command(arg),
            "hair" => self.gm_look("hair", arg),
            "face" => self.gm_look("face", arg),
            "exp" => self.gm_exp(arg),
            "heal" => self.gm_heal(),
            "setrates" => self.gm_set_rates(arg),
            "job" => self.gm_job(arg),
            "npcecho" => self.gm_npc_echo(arg),
            "nx" => self.gm_nx(arg),
            // Added 2026-09-09 so the meso-drop run can reach its own edges. Without it a
            // drop test is limited to whatever balance the character happened to have, and
            // "drop your whole balance" - the boundary the tests pin - is untestable.
            "meso" | "mesos" => self.gm_meso(arg),
            // The shop prices in LP, so this is the one that funds it. See gm_lp.
            "lp" | "leafpoints" => self.gm_lp(arg),
            // Put spent points back in the pool. See gm_reset_ap for why the AP one
            // conserves the total rather than recomputing it from the level.
            "resetap" => self.gm_reset_ap(),
            "resetsp" => self.gm_reset_sp(),
            "learn" => self.gm_learn(arg),
            "craft" => self.gm_craft(arg),
            // Show or set a citizenship - the client's own `/citizenship` shape.
            // session/citizenship.rs.
            "citizenship" => self.gm_citizenship(arg),
            // Re-read `data/npc-dialogue.txt` without restarting. See `gm_npc_reload`.
            "npcreload" => self.gm_npc_reload(arg),
            // Account administration from inside the game. The owner, 2026-09-05. The codes are
            // credentials and go to the GM's screen ONLY - see the two functions.
            "registrationcode" | "regcode" | "invite" => self.gm_registration_code(),
            "recoverycode" => self.gm_recovery_code(arg),
            "track" => self.gm_track(arg),
            "" => self.gm_ack(format!("Not a command. {GM_COMMANDS}")),
            other => self.gm_ack(format!("!{other} is not a command. {GM_COMMANDS}")),
        }
    }


    /// `!craft [profession] [level] [mastery]` - open a Crafting Journal tab without doing
    /// the quest.
    ///
    /// The tab unlocks on nothing but the profession skill's level, so this writes the row
    /// the starter quest would have written. `!craft` with no arguments lists what the
    /// character has; `!craft all 10` opens every tab at the cap; `!craft tailoring 0`
    /// closes one again.
    ///
    /// **The character-level gate is NOT applied here.** `!craft` is a test instrument and
    /// The owner's test characters are level 10 - the cap is what a *craft* obeys
    /// (`crate::crafting::mastery_level_cap`), and a GM setting a level outright is saying
    /// what they mean. The acknowledgement says so rather than leaving it to be discovered.
    pub(super) fn gm_craft(&mut self, arg: &str) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self.gm_ack("!craft REFUSED: no character is claimed on this connection.".to_string());
        };
        let mut fields = arg.split_whitespace();
        let Some(which) = fields.next() else {
            let held = self.store.crafting(chr.id).unwrap_or_default();
            if held.is_empty() {
                return self.gm_ack(format!(
                    "No crafting professions learnt. !craft <profession> [level] [mastery] - profession is 0..5 or one of {}.",
                    crate::crafting::PROFESSION_NAMES.join(", ")
                ));
            }
            let list: Vec<String> = held
                .iter()
                .map(|p| {
                    format!(
                        "{} level {} ({}/{})",
                        crate::crafting::profession_name(p.profession),
                        p.level,
                        p.exp,
                        crate::crafting::mastery_exp_needed(p.level)
                    )
                })
                .collect();
            return self.gm_ack(format!("Crafting: {}.", list.join(", ")));
        };
        let level: u32 = fields.next().and_then(|v| v.parse().ok()).unwrap_or(1);
        let mastery: u32 = fields.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        if level > net::craft::MAX_PROFESSION_LEVEL {
            return self.gm_ack(format!(
                "!craft: {level} is past the maximum of {}. A profession stops there.",
                net::craft::MAX_PROFESSION_LEVEL
            ));
        }
        let professions: Vec<u8> = if which.eq_ignore_ascii_case("all") {
            (0..6).collect()
        } else {
            match crate::crafting::profession_from_word(which) {
                Some(p) => vec![p],
                None => {
                    return self.gm_ack(format!(
                        "!craft: {which:?} is not a profession. Try 0..5, \"all\", or one of {}.",
                        crate::crafting::PROFESSION_NAMES.join(", ")
                    ))
                }
            }
        };
        let mut out = Vec::new();
        let mut done = Vec::new();
        for profession in professions {
            if self.store.set_profession(chr.id, profession, level, mastery).is_err() {
                continue;
            }
            done.push(crate::crafting::profession_name(profession));
            // Level 0 closed the tab: tell the client the skill is gone rather than leaving
            // the old number on a record it will re-read at the next field entry.
            out.push(if level == 0 {
                let id = net::craft::profession_skill(profession).unwrap_or(0);
                Reply {
                    opcode: net::skills::CHANGE_SKILL_RECORD_RESULT,
                    body: net::skills::change_skill_record_result(
                        true,
                        false,
                        &[net::skills::SkillChange::Forget { id }],
                    ),
                    what: format!("ChangeSkillRecordResult: {id} forgotten - !craft closed the tab"),
                }
            } else {
                self.craft_skill_reply_for_gm(profession, level, mastery)
            });
        }
        let cap = crate::crafting::mastery_level_cap(u32::from(chr.level));
        let note = if level > cap {
            format!(
                " Note: crafting will not raise it past level {cap} until your character is level {}.",
                (cap + 1) * 5
            )
        } else {
            String::new()
        };
        out.extend(self.gm_ack(if level == 0 {
            format!("Closed: {}.", done.join(", "))
        } else {
            format!("Set to level {level} (mastery {mastery}): {}.{note}", done.join(", "))
        }));
        out
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
        // The warning goes to the LOG since 2026-09-06 - the owner: *"!job <id> should just say
        // Cobalt is now a <Job Name>"*. It is still worth having where a puzzled damage number
        // gets investigated, which is the log and not the chat.
        if let Some((stat, have)) = mismatch {
            crate::server::log(&format!(
                "!job {job} for {}: has {have} {} and this job wants {} - !job does not move ability points, so its skills will compute almost no damage (a magic attack with {have} INT lands on the damage floor of 1). The formula being right, not a bug.",
                chr.name,
                stat.label(),
                crate::jobs::STAT_MINIMUM
            ));
        }
        crate::server::log(&format!("!job: {} is now job {job} (was {was})", chr.name));
        let mut out = self.gm_ack(match crate::jobs::job_name(job) {
            Some(name) => format!("{} is now a {name}", chr.name),
            None => format!("{} is now job {job}", chr.name),
        });
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
        out.extend(self.teleport(&mut chr, map, format!("GM !map {map}")));
        out
    }


    /// `!hair <id>` / `!face <id>` - change the character's look, and re-enter the map so
    /// the client draws it.
    ///
    /// Built 2026-09-10 as the instrument for the first hybrid-asset client test: the
    /// backported Frieren hair is `42540` and face `22035`, and until this there was no way
    /// to put either id on a character at all. The set coupons will use the same path once
    /// their request packet is captured (`backport/signature-style/README.md`).
    ///
    /// **A `0x007C` with the look bit, not a re-entry and not `0x0138`.** `0x0138
    /// UserAvatarModified` applies nothing in this client - its apply sits behind a guard
    /// that always fails (`docs/opcodes.md`). Until 2026-09-18 this re-sent the `SetField`
    /// for the same map, a visible reload that put the character on the spawn point; the
    /// `StatChanged` handler's FACE/HAIR branch rebuilds the avatar in place
    /// (`beautycoupon::look_stat_changed`), and this command is the cheapest way to watch it
    /// do so: `!hair 42540` on a field, no reload, the hair changes where you stand.
    ///
    /// **The gate is the name table, and it says so.** The client draws an id straight from
    /// `Character/Hair/%08d.img`; an id with no image draws nothing and there is no packet
    /// that reports it. `gm-handbook/items.txt` is generated from the same `String.wz` the
    /// art was installed beside, so "has a name" is the closest cheap proxy for "has art".
    pub(super) fn gm_look(&mut self, kind: &str, arg: &str) -> Vec<Reply> {
        let Ok(id) = arg.parse::<u32>() else {
            return self.gm_ack(format!("!{kind}: {arg:?} is not an id. Try !{kind} {}.", if kind == "hair" { 30000 } else { 20000 }));
        };
        let legal = if kind == "hair" { (30_000..100_000).contains(&id) } else { (20_000..30_000).contains(&id) };
        if !legal {
            return self.gm_ack(format!("!{kind} REFUSED: {id} is outside the {kind} id space."));
        }
        if self.config.item_names.is_empty() {
            return self.gm_ack(format!(
                "!{kind} REFUSED: the name table is empty, so {id} could not be checked and an id                  with no art draws NOTHING with no error. Regenerate gm-handbook/items.txt with                  tools/dump_names.py, or restart the server so it loads."
            ));
        }
        let Some(name) = self.config.item_names.get(&id).cloned() else {
            return self.gm_ack(format!(
                "!{kind} REFUSED: {id} has no name in this client's String.wz, so it almost                  certainly has no art either, and an id with no art draws nothing."
            ));
        };
        let Some(mut chr) = self.claimed_character() else {
            return self.gm_ack(format!("!{kind} REFUSED: no character is claimed on this connection."));
        };
        let (hair, face) = if kind == "hair" { (Some(id), None) } else { (None, Some(id)) };
        match self.store.set_character_look(chr.id, hair, face) {
            Ok(true) => {}
            Ok(false) => return self.gm_ack(format!("!{kind} REFUSED: character {} is not in the store.", chr.id)),
            Err(e) => return self.gm_ack(format!("!{kind} REFUSED: {e}")),
        }
        if kind == "hair" {
            chr.hair = id;
        } else {
            chr.face = id;
        }
        let k = if kind == "hair" { crate::cosmetics::Kind::Hair } else { crate::cosmetics::Kind::Face };
        let mut out = self.gm_ack(format!(
            "{}'s {kind} is now {id} ({name}). Redrawn in place by 0x007C - no reload; if it only shows after a map change, say so.",
            chr.name
        ));
        out.push(super::beautycoupon::look_stat_changed(k, id, false, &format!("GM !{kind} {id}")));
        self.broadcast_look_change(&chr);
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


    /// `!meso [amount]` - set the balance, so a meso-drop run can reach its own edges.
    ///
    /// Added 2026-09-09 for the run that tests `0x0143`. Without it the drop test is capped at
    /// whatever the character happened to be carrying, and two of the cases that matter are
    /// unreachable: *drop more than you hold* needs a known balance to exceed, and *drop your
    /// whole balance* is the boundary `a_meso_drop_larger_than_the_balance_is_refused` pins.
    ///
    /// # It SETS rather than adds, and that is the point
    ///
    /// `!lp` and `!nx` add, because a cash wallet is a running total nobody needs to know
    /// exactly. A drop test needs the balance to be a **known number**, so that "the counter
    /// went down by 10" is a claim that can come back false. `!meso` with no argument reports
    /// without changing anything.
    ///
    /// # The client is told, or the screen and the database disagree
    ///
    /// The meso counter is not re-read on its own. The same `StatChanged` bit the pick-up path
    /// uses carries the new value, so the number on screen is the number in the row - otherwise
    /// the next drop would look wrong for a reason that has nothing to do with dropping.
    pub(super) fn gm_meso(&mut self, arg: &str) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self
                .gm_ack("!meso REFUSED: no character is claimed on this connection.".to_string());
        };
        let held = self.store.mesos(chr.id).unwrap_or(0);
        let Some(token) = arg.split_whitespace().next() else {
            return self.gm_ack(format!(
                "{} holds {held} mesos. `!meso 10000` sets it; `!meso 0` empties it.",
                chr.name
            ));
        };
        let Ok(amount) = token.parse::<u32>() else {
            return self.gm_ack(format!(
                "!meso: {token:?} is not an amount. It takes a whole number, 0 or more."
            ));
        };
        if let Err(e) = self.store.set_mesos(chr.id, amount) {
            return self.gm_ack(format!("!meso FAILED and the balance is unchanged: {e}"));
        }
        let mut out = self.gm_ack(format!(
            "{} now holds {amount} mesos (was {held}).",
            chr.name
        ));
        out.push(Reply {
            opcode: net::combat::STAT_CHANGED,
            body: net::combat::stat_changed(&net::combat::StatChange {
                meso: Some(u64::from(amount)),
                ..Default::default()
            }),
            what: format!(
                "StatChanged: !meso set the balance to {amount}. Sent because the counter is \
                 not re-read on its own - without this the screen and the row disagree, and \
                 the next drop looks wrong for a reason that is not the drop."
            ),
        });
        out
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
        // To the ceiling the client draws, not the base: a Max HP Increase character healed
        // to `max_hp` would stand at 358/447 and call that a bug.
        let pools = self.pools(&chr);
        chr.hp = pools.max_hp;
        chr.mp = pools.max_mp;
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.gm_ack(format!("!heal FAILED: {e}"));
        }
        let mut out = self.gm_ack(format!("{} is restored to {} HP.", chr.name, pools.max_hp));
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { hp: Some(chr.hp), mp: Some(chr.mp), ..Default::default() }
                .build(),
            what: format!("StatChanged: healed to {}/{} hp, {}/{} mp", chr.hp, chr.max_hp, chr.mp, chr.max_mp),
        });
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
        // Current cannot exceed the maximum THE CLIENT DRAWS, or the bar draws past its own
        // end - and that ceiling includes a learned Max HP/MP Increase on top of the base
        // just recomputed, so it is read after the base moved.
        let pools = self.pools(&chr);
        chr.hp = chr.hp.min(pools.max_hp);
        chr.mp = chr.mp.min(pools.max_mp);
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
        // The owner, 2026-09-06: *"resetap and resetsp should just respond with the message
        // 'Ability Point/Skill Point successfully reset for <character name>'"*. The working
        // goes to the log, where it is still readable after the fact; the chat gets one line.
        crate::server::log(&format!(
            "!resetap {}: STR {was_str}->{}, DEX {was_dex}->{}, INT {was_int}->{}, LUK {was_luk}->{}{hpmp_note} - {refund} points back, {} to spend",
            chr.name, chr.strength, chr.dexterity, chr.intelligence, chr.luck, chr.ap
        ));
        let mut out = self.gm_ack(format!("Ability Point successfully reset for {}", chr.name));
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
        // One line in chat (the owner, 2026-09-06); the roster of what was forgotten goes to the log.
        crate::server::log(&format!(
            "!resetsp {}: forgot {} skill(s) [{}], refunded {} skill point(s){}",
            chr.name,
            forgotten.len(),
            named.join(", "),
            refunded.points,
            if failed.is_empty() {
                String::new()
            } else {
                format!(" - BUT THESE FAILED and are still learned: {}", failed.join(", "))
            }
        ));
        let mut out = self.gm_ack(format!("Skill Point successfully reset for {}", chr.name));
        if !forgotten.is_empty() {
            out.push(self.skill_reply(
                // No "A skill has been activated." line - see `session/skills.rs`.
                net::skills::change_skill_record_result(true, false, &forgotten),
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
            // No "A skill has been activated." line per skill - see `session/skills.rs`.
            net::skills::change_skill_record_result(true, false, &changes),
            format!("!learn granted {} skill(s) to job {}", changes.len(), chr.job),
        ));
        if !warning.is_empty() {
            out.extend(self.notice(warning.trim().trim_matches('*').trim().to_string()));
        }
        out
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
        for tier in [
            crate::skillpoints::Tier::First,
            crate::skillpoints::Tier::Second,
            crate::skillpoints::Tier::Third,
        ] {
            let amount = crate::skillpoints::entitlement(tier, level);
            if amount > 0 {
                pools.push(net::stats::SpPool {
                    job_level: net::stats::tier_for_job(match tier {
                        // A *representative* job per tier, because `tier_for_job` is what
                        // the wire wants and it is derived from a job id. 111 is a Crusader;
                        // any third job would do, since all ten end in 1 and map to tier 3.
                        crate::skillpoints::Tier::First => 100,
                        crate::skillpoints::Tier::Second => 110,
                        crate::skillpoints::Tier::Third => 111,
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
    pub(super) fn give_item(
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
        let Some(inv) = self.config.tab_for(item_id) else {
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


    /// `!npcreload [templateId]` - re-read `data/npc-dialogue.txt` **without a restart**.
    ///
    /// The owner, 2026-08-29: *"Restarting the server kicks all of the clients off, but if
    /// possible, I would like to introduce a command to reload all of the NPC server side
    /// chats, so we can amend server side NPC dialogue whenever necessary without disrupting
    /// client connection."*
    ///
    /// # It reaches sessions that were already connected, and that is the whole trick
    ///
    /// `Arc<Config>` is cloned into a `Session` at accept time, so replacing the server's
    /// `Arc` would change nothing for anybody already playing. The table is swapped **inside**
    /// the shared allocation instead - `config::NpcStringTable` - and `npc_line` reads it
    /// fresh for every box it builds, so the next click on that NPC gets the new line.
    ///
    /// # What it does NOT reach
    ///
    /// * **Another channel.** One channel is one `maplecw-world` process with its own
    ///   `Config`; the reply says so rather than implying a world-wide effect.
    /// * **`gm-handbook/npcstrings.txt`.** The generated base is named in the answer but not
    ///   re-read - see `config::NpcStringTable` for why that is a guarantee about the shop
    ///   join rather than laziness.
    /// * **Quest dialogue.** Those lines come from `Config::quests`, not from this table.
    ///
    /// # The optional argument is a read-back, not a filter
    ///
    /// `!npcreload 8` reloads everything and then says what template 8's `d0` now is, so an
    /// edit can be checked from the chat box instead of by walking to the NPC. It cannot
    /// reload one NPC: the file is read whole or not at all.
    pub(super) fn gm_npc_reload(&mut self, arg: &str) -> Vec<Reply> {
        // Reject a bad argument BEFORE reloading. Doing the work and then complaining about
        // the argument would leave the person unsure whether the reload happened.
        let want: Option<u32> = if arg.is_empty() {
            None
        } else {
            match arg.parse::<u32>() {
                Ok(t) => Some(t),
                Err(_) => {
                    return self.gm_ack(format!(
                        "!npcreload: {arg:?} is not an NPC template id, and NOTHING was \
                         reloaded. Use !npcreload on its own, or !npcreload 8 to read Robin's \
                         line back afterwards."
                    ))
                }
            }
        };

        let report = crate::config::reload_npc_dialogue(&self.config);
        // The server console gets every refusal; the chat notice gets the first two. A row
        // the author cannot see refused is a row they will conclude the command ignored.
        for line in &report.refused {
            crate::server::log(&format!("npcreload: refused {line}"));
        }
        crate::server::log(&format!("npcreload: {}", report.summary()));

        let mut text = format!("!npcreload: {}", report.summary());
        if let Some(template) = want {
            // Read back through the same accessor the dialogue path uses, so this echoes what
            // the next click will actually send rather than what the file says.
            let now = self
                .config
                .npc_strings
                .dialogue_line(template)
                .unwrap_or_else(|| "(no d0 - this NPC falls through to the placeholder)".into());
            let now: String = now.chars().take(120).collect();
            text.push_str(&format!(" Template {template} d0 is now: {now:?}"));
        }
        let mut out = self.gm_ack(text);
        // A SECOND notice rather than a longer one. The summary already runs about as long as
        // `GM_COMMANDS`, which is the longest notice this client has been seen to draw, and a
        // list of parse errors has no bound. Both are `0x00BB` and neither is blocked on.
        if let Some(line) = report.refusal_line() {
            out.extend(self.gm_ack(format!("!npcreload: {line}")));
        }
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
    /// **`!online` - everyone playing right now, across every channel.** Public.
    ///
    /// The list is the world hub's directory (`crate::link`) merged with this process's own
    /// announcements, because the hub echoes a channel's arrivals to everyone *else*. With no
    /// hub linked - a single-channel launch - it falls back to this channel's bus, which is
    /// then the whole world, and says so. Names and channels only: a player is not told
    /// where anyone is; that is `!track`, a GM word.
    ///
    /// One notice, comma-separated, because the client's notice draws one line and the list
    /// is short on a server this size. If it ever is not, the count at the front still says
    /// how many there are.
    pub(super) fn who_is_online(&self) -> Vec<Reply> {
        let (names, note): (Vec<String>, &str) = match crate::link::installed() {
            Some(link) => (
                link.everyone().into_iter().map(|(_, e)| format!("{} (ch {})", e.name, e.channel + 1)).collect(),
                "",
            ),
            None => (
                self.bus()
                    .everyone_here()
                    .into_iter()
                    .filter_map(|(id, _)| self.store.character_brief(id).ok().flatten().map(|c| c.name))
                    .map(|n| format!("{n} (ch {})", self.config.channel_id + 1))
                    .collect(),
                " (this channel only - no world hub is linked)",
            ),
        };
        if names.is_empty() {
            return self.gm_ack(format!("Nobody is online{note}."));
        }
        self.gm_ack(format!("Online ({}){note}: {}", names.len(), names.join(", ")))
    }

    /// **`!track <character>` - where one player is: channel and map.** GM only.
    ///
    /// The owner, 2026-09-14: *"`!track <character name>` - available to GMs, track a specific
    /// player's map location and channel information."* The hub's directory carries the map
    /// since today - every field entry re-announces it - so a portal walk shows up within the
    /// time it takes the client to send its `0x00DC`. Without a hub, this channel's bus is
    /// what there is, and the answer says so.
    pub(super) fn gm_track(&self, arg: &str) -> Vec<Reply> {
        let wanted = arg.trim();
        if wanted.is_empty() {
            return self.gm_ack("!track: name a character. Try !track the owner.".to_string());
        }
        let found: Option<(u32, u32, String)> = match crate::link::installed() {
            Some(link) => link.find(wanted).map(|(_, e)| (e.channel, e.map, e.name)),
            None => self.bus().everyone_here().into_iter().find_map(|(id, map)| {
                let c = self.store.character_brief(id).ok().flatten()?;
                // `!track` reports the MAP a player is on, not which copy of it - an
                // instance is not somewhere a GM can be told to walk to.
                c.name.eq_ignore_ascii_case(wanted).then(|| (self.config.channel_id, map.map, c.name))
            }),
        };
        let scope = if crate::link::installed().is_some() { "" } else { " (this channel only - no world hub is linked)" };
        match found {
            Some((channel, map, name)) => self.gm_ack(format!(
                "{name} is on channel {} in map {map}, {}{scope}.",
                channel + 1,
                self.map_name(map)
            )),
            None => self.gm_ack(format!("{wanted} is not online{scope}.")),
        }
    }

    pub(super) fn gm_ack(&self, text: String) -> Vec<Reply> {
        self.notice(text)
    }

    /// `!registrationcode` - mint a single-use registration code and show it to the GM.
    ///
    /// The owner, 2026-09-05: *"There should be a !registrationcode and !recoverycode
    /// <email>/<username> command ingame, which outputs a 1 time use 8 character alphanumeric
    /// code (uppercase) to allow clients to either register an account with us or set a new
    /// password for an existing account."*
    ///
    /// **The code is a credential and it reaches the GM's screen and nothing else.** `notice`
    /// would copy the text into `Reply::what`, and `what` is what `world.log` records - so
    /// these two commands build their notice through [`Session::code_notice`], whose log line
    /// says a code was minted and not which. `store::codes` keeps only a hash; the plaintext
    /// exists in the packet to the GM and then wherever the GM chooses to paste it.
    pub(super) fn gm_registration_code(&self) -> Vec<Reply> {
        match self.store.create_invite_code(store::INVITE_TTL_SECS) {
            Ok(minted) => self.code_notice(
                format!(
                    "Registration code {} - single use, valid {} days. The player enters it on \
                     the launcher's Register screen with a username, email and password.",
                    minted.code,
                    store::INVITE_TTL_SECS / 86_400
                ),
                "registration code minted (not logged)".to_string(),
            ),
            Err(e) => self.gm_ack(format!("could not mint a registration code: {e}")),
        }
    }

    /// `!recoverycode <email|username>` - mint a single-use password-reset code for one account.
    pub(super) fn gm_recovery_code(&self, arg: &str) -> Vec<Reply> {
        let identity = arg.trim();
        if identity.is_empty() {
            return self.gm_ack("!recoverycode needs the account's email or username".to_string());
        }
        match self.store.create_recovery_code(identity, store::RECOVERY_TTL_SECS) {
            Ok(minted) => self.code_notice(
                format!(
                    "Recovery code for {identity}: {} - single use, valid {} hours. They enter \
                     it on the launcher's Forgot password screen with their email or username \
                     and a new password.",
                    minted.code,
                    store::RECOVERY_TTL_SECS / 3_600
                ),
                format!("recovery code minted for {identity:?} (not logged)"),
            ),
            Err(store::StoreError::NoSuchAccount { .. }) => {
                self.gm_ack(format!("no account has the name or email {identity:?}"))
            }
            Err(e) => self.gm_ack(format!("could not mint a recovery code: {e}")),
        }
    }

    /// A chat notice whose LOG LINE is not its text. For the two code commands only.
    fn code_notice(&self, text: String, what: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::notice::CHAT_NOTICE,
            body: net::notice::chat_notice(&text),
            what: format!("ChatNotice: {what}"),
        }]
    }


    /// Say something as the player: a balloon over the head and a line in the chat log.
    ///
    /// **An empty message is dropped rather than sent.** The client's own box will not
    /// submit one, so an empty `0x00E7` means something else is going on, and a balloon
    /// with no text is a worse answer than none.
    ///
    /// The speaker gets the echo back on this connection, and **everyone else on the map gets
    /// the same packet through the bus**. `0x0231` carries the character id, so a remote
    /// client draws the balloon over that character's head and writes the chat-log line.
    ///
    /// This said *"a local echo, not a broadcast ... there is nobody else on the field to send
    /// it to yet"* until 2026-09-05, which was true when written and had been false since the
    /// multiplayer bus arrived. The owner, on the first two-client run with chat: *"each client was
    /// only able to see the message that they sent."* Built before the bus, never revisited -
    /// the "built is not wired" shape in its oldest form.
    pub(super) fn say_out_loud(&mut self, text: &str) -> Vec<Reply> {
        if text.is_empty() {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let echo = Reply {
            opcode: net::userchat::USER_CHAT,
            body: net::userchat::user_chat(chr.id, text),
            what: format!("UserChat: {} ({}) says {:?}", chr.id, chr.name, text),
        };
        // `supersedes: None` - chat is an event, and two lines are two lines.
        if let Some(map) = self.bus().map_of(self.subscriber) {
            self.bus().publish(
                self.subscriber,
                map,
                Reply {
                    what: format!(
                        "UserChat (relayed to the map): {} ({}) says {:?}",
                        chr.id, chr.name, text
                    ),
                    ..echo.clone()
                },
                None,
            );
        }
        vec![echo]
    }
}


/// `!npcreload`, tested through **sessions that were built before it ran**.
///
/// That is the only claim worth making about this feature. `Arc<Config>` is cloned into a
/// `Session` at accept time, so a test that reloads and then builds a session proves nothing
/// at all: it would pass just as happily against a design that swaps the server's `Arc` and
/// leaves every live player on the old text, which is the exact failure this exists to avoid.
/// Every test below therefore constructs its sessions first and never rebuilds them.
#[cfg(test)]
mod npc_reload_tests {
    use super::*;

    /// One `0x00E7`, the way a client sends a typed line.
    fn gm_chat(text: &str) -> Vec<u8> {
        let mut b = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
        b.extend_from_slice(&[0u8; 4]);
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b.push(3);
        b
    }

    /// The `0x00F2` body: u32 npcObjectId, i16 x, i16 y, u32 tail.
    fn npc_click(object_id: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        b.extend_from_slice(&object_id.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&u32::MAX.to_le_bytes());
        b
    }

    /// The `0x00F3` body: u32 handle, u8 messageType, u32 echo, a u16-prefixed string, u8
    /// action.
    fn script_reply(action: i8) -> Vec<u8> {
        let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(0);
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes());
        b.push(action as u8);
        b
    }

    /// **`!online` lists everyone, `!track` says where one of them is - and only a GM may
    /// track.** The owner, 2026-09-14. With no world hub linked (this test, and a one-channel
    /// launch) both answer from this channel's bus and say so; with a hub they answer from its
    /// directory, which `crate::link`'s own tests cover. Names come from the store, not from
    /// anything the client said.
    #[test]
    fn online_lists_everyone_on_the_channel_and_track_is_a_gm_word() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let config = Arc::new(Config::default());
        let fields = Arc::new(crate::fields::Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_gm("maplecw", true).unwrap();
        let other = store.create_account("someone", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for (acct, name, map) in [(account, "Wisp", 10_001_000u32), (other, "Tester2", 1_010u32)] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map, ..Default::default() };
            let id = store.create_character(acct, 0, &chr).unwrap().id;
            store.create_migration(acct, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut gm = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut player = Session::joining(store, config, fields);
        gm.claim_for_character(ids[0]);
        player.claim_for_character(ids[1]);
        gm.on_field_entered();
        player.on_field_entered();

        // Everyone sees the list, and the list is everyone.
        for s in [&mut gm, &mut player] {
            let out = s.handle(&gm_chat("!online"));
            let said = out.iter().filter(|r| r.opcode == net::notice::CHAT_NOTICE).map(notice_text).collect::<Vec<_>>().join(" ");
            assert!(said.starts_with("Online (2)"), "{said}");
            assert!(said.contains("the owner (ch 1)") && said.contains("Tester2 (ch 1)"), "{said}");
            assert!(said.contains("this channel only"), "no hub is linked here, and it says so: {said}");
        }

        // The GM can track, case-insensitively, and is told the map by name.
        let out = gm.handle(&gm_chat("!track tester2"));
        let said = out.iter().filter(|r| r.opcode == net::notice::CHAT_NOTICE).map(notice_text).collect::<Vec<_>>().join(" ");
        assert!(said.starts_with("Tester2 is on channel 1 in map 1010"), "{said}");
        let out = gm.handle(&gm_chat("!track Nobody"));
        let said = out.iter().filter(|r| r.opcode == net::notice::CHAT_NOTICE).map(notice_text).collect::<Vec<_>>().join(" ");
        assert!(said.starts_with("Nobody is not online"), "{said}");

        // A player typing !track is not running a command - it is said out loud, like every
        // other GM word, so the word itself is never confirmed to exist.
        let out = player.handle(&gm_chat("!track the owner"));
        assert!(
            !out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE),
            "a non-GM must get no notice for !track: {out:?}"
        );
        assert!(out.iter().any(|r| r.opcode == net::userchat::USER_CHAT), "it is said as chat instead: {out:?}");
    }

    fn notice_text(r: &Reply) -> String {
        assert_eq!(r.opcode, net::notice::CHAT_NOTICE, "{}", r.what);
        let len = u16::from_le_bytes([r.body[1], r.body[2]]) as usize;
        String::from_utf8(r.body[3..3 + len].to_vec()).unwrap()
    }

    /// **The observable that matters**: the text is in the bytes going to the client, not
    /// merely in a struct the server kept.
    fn body_carries(r: &Reply, text: &str) -> bool {
        r.body.windows(text.len()).any(|w| w == text.as_bytes())
    }

    fn scratch(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("maplecw-gmreload-{tag}-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Two sessions on one channel, both claimed, both standing on map 40 in front of
    /// template 8 - and **both built before any reload runs**.
    fn two_players(
        overlay: &std::path::Path,
        gm_second: bool,
    ) -> (Session, Session, Arc<Config>, std::path::PathBuf) {
        let mut base = std::collections::HashMap::new();
        base.insert(
            8u32,
            crate::config::NpcStrings {
                name: "Robin".to_string(),
                dialogue: vec!["THE SHIPPED LINE".to_string()],
                info: vec!["chatter".to_string()],
                ..Default::default()
            },
        );
        let npcs = vec![net::opcode::FieldNpc {
            object_id: 1000, template_id: 8, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0,
        }];
        let config = Arc::new(Config {
            npcs: [(40u32, npcs)].into_iter().collect(),
            npc_strings: base.into(),
            npc_dialogue_path: overlay.to_path_buf(),
            ..Config::default()
        });

        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_gm("maplecw", true).unwrap();
        let other = store.create_account("guest", "correct horse battery").unwrap();
        if gm_second {
            store.set_gm("guest", true).unwrap();
        }

        let mut made = Vec::new();
        for (n, acct) in [("Reloader", account), ("Bystander", other)] {
            let chr = net::opcode::Character {
                name: n.to_string(), map_id: 40, ..Default::default()
            };
            let id = store.create_character(acct, 0, &chr).unwrap().id;
            store.create_migration(acct, id, 0, 0).unwrap();
            // The `Arc<Config>` is cloned into the session HERE, before any reload. That is
            // the whole point of the test.
            let mut s = Session::new(store.clone(), config.clone());
            assert!(s.claim_for_character(id).contains("claimed the migration"));
            made.push(s);
        }
        let bystander = made.pop().unwrap();
        let gm = made.pop().unwrap();
        (gm, bystander, config, overlay.to_path_buf())
    }

    /// **The feature, stated as a test.**
    ///
    /// The owner: *"I would like to introduce a command to reload all of the NPC server side chats,
    /// so we can amend server side NPC dialogue whenever necessary without disrupting client
    /// connection."*
    ///
    /// Both sessions exist before the file is written and neither is rebuilt. The second one
    /// never runs the command and never even sees it - it is the one that proves the swap
    /// reaches connections that are already open, which is the half a per-session reload would
    /// silently fail.
    #[test]
    fn a_session_built_before_the_reload_sends_the_amended_line() {
        let dir = scratch("live");
        let file = dir.join("npc-dialogue.txt");
        let (mut gm, mut bystander, _config, _) = two_players(&file, false);

        // Before: the generated line, in the bytes.
        let before = bystander.handle(&npc_click(1000));
        assert_eq!(before.len(), 1, "one script box");
        assert!(body_carries(&before[0], "THE SHIPPED LINE"), "{}", before[0].what);

        std::fs::write(&file, "8\td0\tAMENDED WITHOUT A RESTART\n").unwrap();

        let ack = gm.handle(&gm_chat("!npcreload"));
        assert_eq!(ack.len(), 1);
        let ack = notice_text(&ack[0]);
        assert!(ack.starts_with("!npcreload: 1 NPC templates live"), "{ack}");
        assert!(ack.contains("1 replaced a line the NPC already had"), "{ack}");
        assert!(ack.contains("npc-dialogue.txt"), "it names the file it read: {ack}");
        assert!(ack.contains("npcstrings.txt"), "and the base it sat on: {ack}");
        assert!(!ack.contains("REFUSED"), "{ack}");

        // After: the SAME session object, never rebuilt, never reconnected.
        let after = bystander.handle(&npc_click(1000));
        assert_eq!(after.len(), 1);
        assert!(body_carries(&after[0], "AMENDED WITHOUT A RESTART"), "{}", after[0].what);
        assert!(!body_carries(&after[0], "THE SHIPPED LINE"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A refused reload must leave the live sessions on the text they had. The control is the
    /// same click, run before and after, on a session that is not rebuilt in between.
    #[test]
    fn a_broken_file_is_refused_and_a_live_session_keeps_the_line_it_had() {
        let dir = scratch("broken");
        let file = dir.join("npc-dialogue.txt");
        let (mut gm, mut bystander, _config, _) = two_players(&file, false);

        std::fs::write(&file, "8\td0\tGOOD EDIT\n").unwrap();
        assert!(!notice_text(&gm.handle(&gm_chat("!npcreload"))[0]).contains("REFUSED"));
        assert!(body_carries(&bystander.handle(&npc_click(1000))[0], "GOOD EDIT"));

        // Saved with spaces. Every row is unusable.
        std::fs::write(&file, "8 d0 BAD EDIT\n9 d0 ALSO BAD\n").unwrap();
        let ack = notice_text(&gm.handle(&gm_chat("!npcreload"))[0]);
        assert!(ack.contains("REFUSED and NOTHING changed"), "{ack}");
        assert!(ack.contains("TAB, not spaces"), "{ack}");

        let after = bystander.handle(&npc_click(1000));
        assert!(body_carries(&after[0], "GOOD EDIT"), "the old table survived: {}", after[0].what);
        assert!(!body_carries(&after[0], "BAD EDIT"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A missing file is not a failure, and it says how many it applied - zero. This is the
    /// case a naive wiring reports as plain success, because `load_npc_strings` returns an
    /// empty map on a read error.
    #[test]
    fn a_missing_overlay_reports_zero_overrides_rather_than_a_bare_ok() {
        let dir = scratch("absent");
        let file = dir.join("nothing-here.txt");
        let (mut gm, mut bystander, _config, _) = two_players(&file, false);

        let ack = notice_text(&gm.handle(&gm_chat("!npcreload"))[0]);
        assert!(!ack.contains("REFUSED"), "{ack}");
        assert!(ack.contains("0 overlaid from"), "{ack}");
        assert!(body_carries(&bystander.handle(&npc_click(1000))[0], "THE SHIPPED LINE"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **A mid-conversation reload must not wedge the client.**
    ///
    /// `CLAUDE.md`: an unanswered packet freezes the whole UI. The control is the identical
    /// exchange with no reload in the middle - the two answers have to match, or the reload
    /// changed how a conversation ends.
    #[test]
    fn a_reload_between_the_box_and_the_ok_answers_exactly_as_no_reload_does() {
        let dir = scratch("midconvo");
        let file = dir.join("npc-dialogue.txt");

        // Control: click, OK, no reload.
        let (_gm0, mut player0, _c0, _) = two_players(&file, false);
        assert_eq!(player0.handle(&npc_click(1000)).len(), 1);
        let control = player0.handle(&script_reply(net::script::SCRIPT_ACTION_YES));

        // Variant: click, reload, OK.
        let (mut gm, mut player, _config, _) = two_players(&file, false);
        assert_eq!(player.handle(&npc_click(1000)).len(), 1);
        std::fs::write(&file, "8\td0\tCHANGED MID CONVERSATION\n").unwrap();
        assert!(!notice_text(&gm.handle(&gm_chat("!npcreload"))[0]).contains("REFUSED"));
        let variant = player.handle(&script_reply(net::script::SCRIPT_ACTION_YES));

        assert_eq!(
            control.len(),
            variant.len(),
            "the reload changed how an open conversation is answered"
        );
        // And the next click gets the new line, so the conversation state was not left stale.
        let next = player.handle(&npc_click(1000));
        assert!(body_carries(&next[0], "CHANGED MID CONVERSATION"), "{}", next[0].what);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The read-back argument echoes what the next click will send, through the same accessor
    /// the dialogue path uses - so it cannot agree with the file while disagreeing with the
    /// wire.
    #[test]
    fn the_template_argument_reads_the_new_line_back() {
        let dir = scratch("readback");
        let file = dir.join("npc-dialogue.txt");
        let (mut gm, _bystander, _config, _) = two_players(&file, false);
        std::fs::write(&file, "8\td0\tREAD ME BACK\n").unwrap();

        let ack = notice_text(&gm.handle(&gm_chat("!npcreload 8"))[0]);
        assert!(ack.contains("Template 8 d0 is now: \"READ ME BACK\""), "{ack}");

        let ack = notice_text(&gm.handle(&gm_chat("!npcreload 99"))[0]);
        assert!(ack.contains("falls through to the placeholder"), "{ack}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A bad argument must reload NOTHING, and say so. Doing the work and then complaining
    /// about the argument leaves the person unable to tell whether the file was read.
    #[test]
    fn a_bad_argument_reloads_nothing_at_all() {
        let dir = scratch("badarg");
        let file = dir.join("npc-dialogue.txt");
        let (mut gm, mut bystander, _config, _) = two_players(&file, false);
        std::fs::write(&file, "8\td0\tSHOULD NOT BE READ\n").unwrap();

        let ack = notice_text(&gm.handle(&gm_chat("!npcreload potato"))[0]);
        assert!(ack.contains("NOTHING was reloaded"), "{ack}");
        assert!(body_carries(&bystander.handle(&npc_click(1000))[0], "THE SHIPPED LINE"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The GM gate covers this command like every other one. A non-GM's `!npcreload` is said
    /// out loud as ordinary chat and reloads nothing.
    #[test]
    fn a_non_gm_cannot_reload_and_is_answered_with_chat() {
        let dir = scratch("nongm");
        let file = dir.join("npc-dialogue.txt");
        let (_gm, mut bystander, _config, _) = two_players(&file, false);
        std::fs::write(&file, "8\td0\tSHOULD NOT BE READ\n").unwrap();

        let out = bystander.handle(&gm_chat("!npcreload"));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].opcode, net::userchat::USER_CHAT, "{}", out[0].what);
        assert!(body_carries(&bystander.handle(&npc_click(1000))[0], "THE SHIPPED LINE"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A refused row is a SECOND notice, not a longer first one - and it is said, not
    /// counted. A row an author cannot see refused is a row they conclude was applied.
    ///
    /// The length bound matters: `GM_COMMANDS` at 474 characters is the longest notice this
    /// client has been seen to draw, so neither line may run away.
    #[test]
    fn a_refused_row_gets_its_own_bounded_notice_and_the_good_rows_still_apply() {
        let dir = scratch("refusedrow");
        let file = dir.join("npc-dialogue.txt");
        let (mut gm, mut bystander, _config, _) = two_players(&file, false);
        std::fs::write(&file, "8\tname\tNot Robin\n8\tinfo0\tnope\n8\td0\tTHE GOOD ROW\n").unwrap();

        let out = gm.handle(&gm_chat("!npcreload"));
        assert_eq!(out.len(), 2, "the counts, then the refusals");
        let counts = notice_text(&out[0]);
        let refused = notice_text(&out[1]);
        assert!(!counts.contains("REFUSED"), "the summary stays about the counts: {counts}");
        assert!(refused.contains("2 overlay row(s) REFUSED"), "{refused}");
        // The bound used to be `GM_COMMANDS.len()` - "no longer than a notice known to draw".
        // The help text was pruned to a third of its length on 2026-09-06, which would have
        // made this test fail for a reason unrelated to the reload. The number it stood in for
        // is kept instead: the pre-pruning help text, **525 characters**, drew in full in
        // The owner's own screenshot that day, so that is the longest notice measured on a screen.
        const LONGEST_NOTICE_SEEN_ON_SCREEN: usize = 525;
        assert!(counts.len() <= LONGEST_NOTICE_SEEN_ON_SCREEN, "{} chars: {counts}", counts.len());
        assert!(refused.len() <= LONGEST_NOTICE_SEEN_ON_SCREEN, "{} chars: {refused}", refused.len());

        // And the one good row was still applied - a refusal is per row, not per file.
        assert!(body_carries(&bystander.handle(&npc_click(1000))[0], "THE GOOD ROW"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **`!meso` SETS a known balance**, which is what makes a drop test able to fail.
    ///
    /// Added 2026-09-09 for the `0x0143` run. `!lp` and `!nx` add; this sets, because "the
    /// counter went down by 10" is only a claim that can come back false if the number it
    /// started from is known. It also asserts the client is TOLD - a balance changed in the
    /// row and not on screen makes the next drop look wrong for a reason that is not the drop.
    #[test]
    fn meso_sets_a_known_balance_and_tells_the_client() {
        let dir = scratch("meso");
        let file = dir.join("npc-dialogue.txt");
        let (mut gm, _b, _config, _) = two_players(&file, false);
        let id = gm.claimed_character().unwrap().id;

        let out = gm.handle(&gm_chat("!meso 10000"));
        assert_eq!(gm.store.mesos(id).unwrap(), 10_000, "the row must hold the new balance");
        let stat = out
            .iter()
            .find(|r| r.opcode == net::combat::STAT_CHANGED)
            .expect("the client must be told, or the screen and the row disagree");
        assert!(stat.what.contains("10000"), "{}", stat.what);

        // It SETS rather than adds - a second call is not 20000.
        gm.handle(&gm_chat("!meso 25"));
        assert_eq!(gm.store.mesos(id).unwrap(), 25);
        // Zero is a legitimate balance, and it is the one the "drop your whole balance" case
        // leaves behind.
        gm.handle(&gm_chat("!meso 0"));
        assert_eq!(gm.store.mesos(id).unwrap(), 0);

        // Garbage changes nothing and says so, rather than silently setting zero.
        let bad = gm.handle(&gm_chat("!meso lots"));
        assert_eq!(gm.store.mesos(id).unwrap(), 0, "a bad argument must not write");
        assert!(notice_text(&bad[0]).contains("not an amount"), "{}", notice_text(&bad[0]));

        // No argument reports without changing anything.
        gm.handle(&gm_chat("!meso 77"));
        let report = gm.handle(&gm_chat("!meso"));
        assert_eq!(gm.store.mesos(id).unwrap(), 77, "a bare !meso must not write");
        assert!(notice_text(&report[0]).contains("77"), "{}", notice_text(&report[0]));
    }

    /// `!help` and the dispatcher must agree, **in both directions and for both lists**: every
    /// word `!help` names is a command the dispatcher takes, and every command the owner asked to
    /// remove on 2026-09-06 is neither listed nor dispatched.
    #[test]
    fn the_help_text_lists_exactly_the_commands_the_dispatcher_has() {
        let dir = scratch("help");
        let file = dir.join("npc-dialogue.txt");
        let (mut gm, _b, _config, _) = two_players(&file, false);
        let names = |text: &str| -> Vec<String> {
            text.split_whitespace()
                .filter_map(|w| w.strip_prefix('!'))
                .map(|w| w.trim_end_matches(',').to_string())
                .collect()
        };
        for word in names(GM_COMMANDS).into_iter().chain(names(PLAYER_COMMANDS)) {
            // A command with a required argument answers with its own complaint; only the
            // dispatcher's "is not a command" would mean the help text is lying.
            let ack = gm.handle(&gm_chat(&format!("!{word}")));
            // Only the notices: `!heal` also sends a stat change, and that is not text.
            let said = ack
                .iter()
                .filter(|r| r.opcode == net::notice::CHAT_NOTICE)
                .map(notice_text)
                .collect::<Vec<_>>()
                .join(" ");
            assert!(!said.contains("is not a command"), "!{word} is in the help text but not the dispatcher: {said}");
        }
        for gone in ["exprate", "mesorate", "droprate", "migsweep", "npcfx", "buff", "unbuff", "buy", "locker", "kit"] {
            assert!(!GM_COMMANDS.contains(&format!("!{gone}")), "!{gone} was removed on 2026-09-06");
            let said = notice_text(&gm.handle(&gm_chat(&format!("!{gone}")))[0]);
            assert!(said.contains("is not a command"), "!{gone} still dispatches: {said}");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **`!rates` and `!help` are for everyone; `!help` shows a player only what they may
    /// run.** The owner, 2026-09-06. A player's `!help` names `!rates` and nothing a GM has, and a
    /// player's `!item` is still said out loud like any other typed line.
    #[test]
    fn a_player_gets_rates_and_a_help_that_names_no_gm_command() {
        let dir = scratch("public");
        let file = dir.join("npc-dialogue.txt");
        let (mut gm, mut player, _config, _) = two_players(&file, false);

        let rates = notice_text(&player.handle(&gm_chat("!rates"))[0]);
        assert!(rates.contains("EXP 1x"), "a player reads the rates: {rates}");

        let help = notice_text(&player.handle(&gm_chat("!help"))[0]);
        assert_eq!(help, PLAYER_COMMANDS);
        assert!(!help.contains("!item") && !help.contains("!map"), "no GM word leaks: {help}");

        let gm_help = notice_text(&gm.handle(&gm_chat("!help"))[0]);
        assert_eq!(gm_help, GM_COMMANDS);

        // Everything else a player types with a bang is chat, exactly as before.
        let out = player.handle(&gm_chat("!item 2000000"));
        assert!(out.iter().any(|r| r.opcode == net::userchat::USER_CHAT), "said out loud: {out:?}");
        assert!(!out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "and no notice of any kind: {out:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
