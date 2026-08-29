<#
.SYNOPSIS
    Run the client against the real login server, with characters that persist.

.DESCRIPTION
    The replacement for test-charselect.ps1. That script drives tools/handshake_probe.py,
    which replays bodies handed to it on a command line; this one starts crates/login,
    which reads and writes a database. The visible difference is the one that matters:
    a character created in one run is still there in the next.

    Everything else is deliberately identical to the run that is known to work - the same
    client patches, the same launch mode, the same exit forensics - so that if a run goes
    wrong, the server is the only thing that changed.

.NOTES
    ============ THE TEST PLAN, as of 2026-08-28: ALL FOUR FIRST JOBS ============

    TWO copies in this file - this one and the Write-Host block near the bottom that
    actually gets printed. Update both, then RENDER the second one and read it.

    -SetFieldProbe is NOT optional: without it Session::handle returns nothing for EVERY
    packet and the client sits on "Connecting...". Run -Stop before relaunching.

    DO NOT PASS -HeapFix. It armed, it held, and it was irrelevant - 10 deaths of that
    family across the archive, 8 at 0x14019b58e and 2 at 0x14019bbf3, and FIFTY-SIX sites
    carry the same ladder. It also costs a measurement every run: the one constraint anyone
    has on when the stray 1 is written comes from finding a damaged slot on the free list,
    which is only an argument while the free is unpatched.

    WHAT THE LAST RUN CLOSED - none of this needs testing again
    ----------------------------------------------------------
    SELLING WORKS, and the Buy Back tab is not coming. Removing the type-10 refresh fixed
    the crash. This client's UIShop.img/Shop has no repurchaseInfo node and exactly TabBuy
    and TabSell, so there is no third tab to draw. Not a bug and not a gap.
    WARRIOR WORKS - skills cast, and the client refuses at 0 MP.
    THE RED POTION NEVER RESTORED MP. The owner: *"Using the Red Potion when my MP is depleted
    incorrectly recovered my MP?"* It did not. Its spec is hp 100, mp 0, and the log line
    reads "+0 mp (now 181/181)". The CLIENT had been spending MP locally on every swing
    while the SERVER never did, so our stale 181/181 rode along on the potion's 0x007C and
    the client believed it. Any 0x007C would have done it - idle regen a few seconds later
    would have. FIXED: attack skills now cost MP server-side, keyed by (skill id, level),
    which is the first thing research/attack-skill-id.md actually bought.
    THE CLASSIC SHOP WINDOW DRAWS. See step 7.
THE /hitdamagetest ROUTE IS DEAD, but that is one lever, not the answer.
    No 0x0189 came back, so the permission gate refused it; the echo happens BEFORE the
    dispatch, so seeing it in chat meant nothing. 0x00EA is no longer sent. The owner pushed back
    on the conclusion I drew from that and they were right - see step 1.
    YOU STAY DEAD. You could previously regenerate out of death, and because the revive
    dialog fires on the TRANSITION it would never have come back.
    THE CASH SHOP OPENS and the wallet reads both fields. Exit works. !lp works.
    THE JOB CHANGE, THE SKILL POINTS AND THE MAGICIAN BOOK all worked first time: !job 200
    played the effect, the points showed, the + button was live, all six Magician skills
    listed, and Magic Claw accepted seven points.
    MAGIC CLAW DEALING 1 WAS NOT A BUG. The owner was a Rogue - LUK 36, DEX 24, INT 6 - wearing
    a Magician job id. MagicTotal seeds from floor(INT/2), so the whole damage window sat
    between 1 and 2 before the mob magic defence was even applied. The formula predicts
    exactly the 1 they saw. Every branch step below therefore begins with !resetap.

    THE FINDING THIS RUN RESTS ON - THE ATTACK PACKET CARRIES THE SKILL ID
    ---------------------------------------------------------------------
    For nine days world::magic, damage-formula.md 9.1 and mob-combat.md 7 all said the
    server cannot tell which skill was cast, and every damage validator was left unwired
    because of it. It is the u32 at body offset 2, with the level as the u8 at offset 6.

    The evidence for the old claim was an ABSENCE IN CAPTURES THAT COULD NOT HAVE CONTAINED
    THE THING: every archived body was an ordinary swing, where the field is legitimately 0,
    and a zero field explains nothing about itself. 426 swings at 0, one Three Snails at
    1000 level 3 (its maxLevel is 3), seven Magic Claws at 2001003 level 7 (the owner had put in
    7). Two skills, two levels each known from an unrelated source - and THAT is the
    corroboration, not the count. The first pass reported 689/2/14 because a glob over
    previous-runs/ AND research/fixtures/ counts a capture once per name it has: 155
    world logs on disk are 119 distinct files. Hash before counting.

    Nothing in this run tests that directly. It is why the rest of the plan is possible.
    research/attack-skill-id.md.

    TWO NEW COMMANDS DO THE SETUP
    -----------------------------
    !learn        every skill of your current job at its own max level, no skill points
                  needed. !learn 5 caps at 5. !learn 2001003 7 does one skill. Each skill
                  is clamped to ITS OWN ceiling - the Magician book runs to 15 AND 20, so
                  any single constant is wrong for half of it.
    !kit          the weapon and ammunition that job needs, and it WARNS if the character
                  cannot equip what it just handed over. There is no free bow, crossbow or
                  claw in this client - all 230 weapon images were read to establish that -
                  so a !job character can end up holding something it cannot wear, which on
                  screen is indistinguishable from a broken skill.

    THE STEPS, 1-9 plus 5b. Each is a claim that can come back false; report them
    separately.
    ORDER: 8 FIRST - it only means anything in a young session. Then 2 with 3 inside it,
    then 4, 5, 5b, 7, and 9 whenever it suits - it rides along and costs no run of its own.
    Step 1 is a READ, not a test.
    DO STEP 7 FIRST: NPC shops have never been sent to a client, so it is the step most
    likely to end the session, and everything after it is cheaper to redo than to lose.

     1. NOTHING TO TEST HERE - the 1 is settled, and not in my favour. Read once, skip.

        I told the owner the client computes contact damage and merely discards it, and that
        clearing one byte would let it keep the answer. The patch armed, verified its own
        write, logged the new bytes - and the claim was still 1.

        The path is real but it is a FIXED-DAMAGE OVERRIDE:
            14288b2dc  cmp dword [rdi+0xe8], 0
            14288b2e3  jle 0x14288ca17          ; `xor dil,dil` into the EPILOGUE
        rdi is the mob's TEMPLATE record (FUN_141c54dd0, named in user-hit.md 3.5), and
        template+0xe8 is written by the template loader FUN_14047d990 at 0x140481995 from the
        WZ property `fixedBodyAttackDamage` (string at 0x14328afb8).

        ZERO of this client's 193 mob images carry fixedBodyAttackDamage. All 193 carry
        PADamage. So every contact hit bails into the epilogue and the gate is never reached,
        which is exactly what the run showed. The patch is INERT for contact damage; it is off
        by default now and -ClientHitNumberPatch re-arms it, kept only in case a mob
        ATTACK-SKILL hit reaches the gate - and no such hit exists in 198 captures.

        THE ERROR WAS MINE AND IT IS THIS FILE'S OLDEST RULE: I found a real calculation and
        did not check its precondition. One wz-dump over 193 images - the same instrument used
        three times already in that write-up - would have said so before a launch was spent.

        So: the client has no ordinary contact-damage calculation in this build. Contact damage
        is the SERVER's to supply, and which packet carries it to the client is still not
        found. Two numbers stay for now, and ours is the correct one.
        research/damage-number-two-numbers.md.

     2. WARRIOR - FIRST OF THE FOUR BRANCHES, after step 7. Do step 3 in the MIDDLE of it.
        It is the only branch whose weapon is FREE (Sword 1302000: reqLevel 0, no stat, no
        job bit) and none of its skills carries a weapon column. So it separates "does a
        skill attack work at all" from "am I holding the right thing", which 4 and 5 cannot.
          !resetap, put points into STR to 35+, !job 100, !kit.
          *** STOP. DO STEP 3 NOW, BEFORE !learn. *** Then !learn, EQUIP THE SWORD.
        The six: Improved HP Recovery, Max HP Increase, Precise Strikes (passive), Iron Body
        (buff), Power Strike, Slash Blast (attacks).
          a) do Power Strike and Slash Blast do real damage, well above 1?
          b) does Slash Blast hit up to FOUR mobs? mobCount 4 is what tells it from Power
             Strike, whose mobCount is 1
          c) IRON BODY - SAY THE W. DEF NUMBER BEFORE AND AFTER. This is a measurement:
               rises by about a QUARTER of its base -> the percent-to-flat resolution is
                          right. That is the [I] this run promotes
               rises by exactly 25 whatever the base -> the raw percent reached the wire
               does not move -> CTS bit 86 is wrong, and Magic Armor rests on the same bit
             indiePddR is a PERCENT and bit 86 is a FLAT add, so the server has to resolve
             one into the other. Passing 0 yields a working cast that adds nothing.
          d) does MP drop on every cast?

     3. THE PASSIVE QUESTION - one command, and it decides a design.
        Nine of the 24 are passive. The client's own lookup chain reads them from its
        Skill.wz off the level in 0x0081 - FUN_1407b3df0 -> FUN_14079fe90 -> FUN_1401ba9d0 -
        so seven of the nine change nothing this server owns and cannot be tested. TWO can.
        DO IT INSIDE STEP 2, after !kit and BEFORE !learn: !learn hands out Max HP
        Increase at level 15 with everything else, and once it is learned the
        before-number is gone. If !learn has already run, !resetsp forgets the book.
        WRITE DOWN MAX HP. Then !learn 1000001 15 (Max HP Increase, tooltip +25%).
        Look again.
          UP by about a quarter -> the client applies passives itself. The server must NOT
                     also apply them or every one of them doubles
          unchanged -> the client does nothing with them and the server owes nine skills
                     their effect
        Both answers are actionable and they point opposite ways, which is why this is worth
        a step of its own rather than a glance.

     4. BOWMAN - and this one has a stat gate that can stop the run.
          !resetap, points into DEX to 35+, !job 300, !kit, !learn, EQUIP THE BOW.
        War Bow 1452000 needs level 10 and DEX 25; arrows 2060000 are free and go to the USE
        tab, where the client picks them up itself. That tab is [L] now, not [I]: the
        client's GetInventoryType returns itemId/1000000 for everything outside the equip
        family, and !item routes on the identical quotient.
          a) does Arrow Blow fire, and does the arrow count drop by 1?
          b) does Double Shot fire, and drop it by 2?
          c) FOCUS - do ACCURACY and AVOIDABILITY both rise? Say both:
               both -> bits 88 and 89 correct
               only accuracy -> 89 is not Avoidability; sweep 89..91
               neither -> the window may show equipment only; Magic Armor is the control
          d) UNEQUIP THE BOW and try Arrow Blow. It should refuse - the CLIENT enforces the
             weapon column. If it fires anyway, say so; that changes who owns the gate

     5. THIEF - two weapons, and a swap between them. THIS IS THE ONE THAT LOOKS BROKEN
        IF THE SWAP IS FORGOTTEN. Double Stab wants weapon 33 (dagger), Lucky Seven wants 47
        (claw), and no item in this client is both.
          !resetap, points into LUK to 35+, !job 400, !kit, !learn.
          a) EQUIP THE RAZOR 1332000 (dagger). Double Stab - two hits per cast
          b) SWAP TO THE GARNIER 1472000 (claw). Lucky Seven
          c) DARK SIGHT - translucent, and slower?
               translucent AND slower -> bit 99 and the speed penalty both right
               translucent, same speed -> the penalty was dropped
               slower, not translucent -> a flag of 1 is not enough on bit 99
             THEN RIGHT-CLICK THE ICON. Back to normal speed AND visible? Staying slow means
             the off-path cleared one bit of the two, which is a fixed hazard being checked
          d) Disorder will say it grants nothing. Correct and deliberate: it is a debuff on
             the MOB and this server has no packet for that. Check it freezes nothing

     5b. MAGICIAN - the wand, which is the bug the owner hit on 2026-08-28.
        !kit said "the Magician needs nothing". That was my error and it is the same one I
        made about the Warrior an hour earlier, in the other direction: no Magician skill is
        GATED on a weapon, and I read that as no weapon NEEDED. MagicTotal is floor(INT/2)
        plus equipment incMAD, the Wooden Wand's incMAD is 27, and the row had read its
        incWAT of 18 and concluded weapons do not matter on the magic path.
          !resetap, points into INT to 35+, !job 200, !kit, !learn, EQUIP THE WAND.
        Wooden Wand 1372000 needs level 10 and INT 20; there is no free wand or staff, and
        the staff is strictly worse for magic (incMAD 24) and adds a reqJob bit.
          a) does Magic Claw do real damage now? Roughly what number?
          b) Magic Claw is attackCount 2 - two numbers per cast?
          c) Magic Guard: cast, then get hit. Does the damage come off MP instead of HP?
             That split is the SERVER's arithmetic, not the client's - the client never
             writes HP, so this is the one buff whose effect we compute
          d) does the wand go in the hand, or does !kit warn that it cannot?

     6. THE PURCHASE - carried over, still unconfirmed.
        (If leaf points are under 1000, !lp 99000 first. A PREREQUISITE, not a test: !lp has
        been confirmed on several runs and was struck from this plan on 2026-08-28 when the owner
        said "we tested 0 quite a few times".)
        Last run the coupon bought fine and
        landed in the ITEM inventory: 0x19 is the reply to "move a locker item into a bag",
        so it did what it says. The purchase reply is 0x05AE sub-op 0x0C.
        Buy the MYSTERY HAIR COUPON - Main tab, 100 LP, SN 150000000. NOT Brown Puppy (a
        pet, refused) and NOT Red Hat (1802002 is pet EQUIPMENT).
          a) CASH INVENTORY, upper left panel?   b) success message and sound?
          c) leaf points drop by exactly 100?
          all three -> the cash shop is CLOSED as a feature
          balance keeps dropping, or several coupons from one click -> the re-entry is
                     SENDING rather than completing. CLOSE THE CLIENT and say so

     7. BUYING FROM AN NPC SHOP - the only half of the counter never tested.

        Selling is done and the window draws. Buying has never been on a wire.
          a) buy something from Lucy. Do the ITEM and the MESOS both move?
          b) click Buy AGAIN.
               works twice -> buying is done
               the second click does nothing, silently -> the 0x055E never went out and
                          shopUI+0x4b0 is latched. Close and re-click the NPC; a fresh
                          0x055D alone will NOT recover it, because of the modal guard
          c) do the prices match data/shops.txt?

     8. THE !learn CRASH - one observation, and it needs the young-session discriminator.

        `!learn 1000001 15` (Max HP Increase) killed the client, but the fault was at
        18:28:59.889 and the packet went out at 18:28:56.211 - **3.7 seconds earlier**, with
        a `0x0226` reply and dozens of mob moves in between. Not on the packet.

        The packet is byte-correct: header `01 01 00 | 01 00`, entry `41420f00` = 1000001,
        `0f000000` = 15, masterLevel 0, then `00 80 05 bb 46 e6 17 02` = the never-expires
        constant, then the trailing `00`. 26 bytes = 5 + 20 + 1, which is the builder's own
        arithmetic. `needs_master_level` correctly excludes this skill.

        Fault `0x1415f0db0` appears in NONE of the other archived hook logs.

        So it is the pair this file keeps meeting: the command is fatal, or the session had
        been running long enough for something else to fire. Opposite work, identical on
        screen. **Do it at ~40 s of client life, before anything else.**
          dies again, early -> the command is fatal, and repeatable
          does not die      -> the session was long and the command is innocent
        Say WHEN either way. That discriminator cleared GoodTest in one login on 2026-08-22
        and has convicted nothing since, so neither answer is the safe default.

     9. THE KEYBOARD LAYOUT - 20 seconds, and it decides which of two implementations.

        The owner: "Upon logout then subsequent login, this customization is completely gone."
        Correct, and it is not a broken feature: NOTHING in the seven crates has ever
        touched a key mapping, and the client has nowhere local to keep one - its settings
        block is sound, graphics and chat options, no key mapping in it. So it can only come
        from us, and we have never sent it.

        WHAT IS NOT KNOWN is whether the client even TELLS us when a key changes. 162
        distinct archived captures contain no keymap-shaped packet - but nobody has ever
        changed a key DURING a capture, so that is "never captured", not "never sent". This
        step is the whole difference and it costs nothing.

          Open the keyboard settings, DRAG ONE SKILL ONTO AN EMPTY KEY, close the window.
          Do it at a moment you can name, and say roughly when.

        Then the answer is in world.log without another launch:
          a NEW opcode near that moment -> the client reports changes. Its body IS the
                     layout, and the same layout inverted is the restore packet. Both halves
                     become measured. This is the server-side implementation and it is the
                     right one
          nothing new -> the client never volunteers it. The mapping can then only be
                     recovered from the client's own memory by the hook, which is a
                     different design and a worse one

        The client does NOT freeze when a key is changed - the owner has done it several times
        across sessions - so whatever it sends needs no reply, and our current silence is
        already safe. That is one thing this step does not have to establish.
        research/keymap-not-saved.md.

    MORE THAN ONE ACCOUNT - new 2026-08-28, and not a step in this plan
    -------------------------------------------------------------------
    The login server used to resolve --account ONCE at startup, so one process could only
    ever be one player. It resolves PER CONNECTION now, from a claim staked by
    maplecw-launcher after it checks a password (argon2id). Relaunch with -Launcher to use
    it; sign in with an account name OR its email.
      .\target\release\maplecw-useradd.exe <name> --email <addr>
      .\target\release\maplecw-useradd.exe --list
    An ordinary run (no -Launcher) CLEARS any leftover claim first, so --account wins and a
    claim from yesterday cannot quietly serve someone else's characters through a whole plan.
    Still not authentication: the game socket carries no credentials.

    BUILT BUT NOT WIRED - say so rather than let it look like a bug
    --------------------------------------------------------------
    DAMAGE VALIDATION. world::damage::check_hit and world::magic::check_magic_hit are written
    and tested and have NO caller. The skill id unblocked them; nothing else was done. Log-only
    when wired - the client authors the number.
    MOB -> PLAYER CONTACT DAMAGE is the server's to supply, and which packet tells the CLIENT
    the number is NOT FOUND. That is why two numbers are on screen.
    JOB ADVANCEMENT through an NPC. world::jobs decides it and only !job reaches it, so the
    four instructors still just talk.
    SPENT SKILL POINTS are not persisted - the pool is computed from LEVEL, so points come
    back. !learn grants directly and sidesteps it.
    DISORDER is a debuff on the MOB and this server has no packet for that.
    QUEST COMPLETION cannot TAKE an item (Act.n.item with a negative count).
    The Shop2 window (0x0560): its art is not in this client and it can no longer be sent.
    MP COST on attack skills and the CLASSIC SHOP counter were on this list and are now WIRED.

    REGRESSION GLANCES - seconds each
    ---------------------------------
      Drops arc out of the corpse and are walkable-over, especially on a slope or step.
      The kill-EXP line bottom-right is WHITE, and quest EXP is not.
      Mobs on !map 40 are already standing there - no fade-in.
      Item pick-ups stay OUT of the chat log.
      A level-up gives +16 max HP and +12 max MP.
      Etc items and mesos survive a relog; Garnet Ores stack into one slot.
      !setrates 2 3 5 -> one banner naming all three; !rates reads them back.

    STILL OPEN - do not spend the run confirming these are broken
    ------------------------------------------------------------
      - NPC SHOPS: BUILT this session and awaiting step 7. Never on a wire.
      - BUYING FROM THE CASH SHOP WINDOW is BUILT and awaiting step 6. The old note here
        said no packet could report a purchase without a message; that was a known-list
        search over the six INLINE arms and it missed the two that delegate. 0x05AE sub-op
        0x0C is the one. !buy from the field still works as the control.
      - The Shop2 window (0x0560) can no longer be sent at all: its art is not in this
        client, which is what killed the client twice. `--shop` is a no-op that says so.
        The CLASSIC counter (0x055D) replaced it and is step 7.
      - Outgoing damage validation. HALF-UNBLOCKED 2026-08-28: the 0x00DF header DOES carry
        the skill id (u32 at body offset 2) and its level, so a per-skill ceiling is now
        computable. The ACTION field is still unfound, and nothing is wired - attack skills
        still cost no MP and no hit is checked. research/attack-skill-id.md.
      - Page heap is OFF, so !heap -p -a has no allocation stacks to print. That is an IFEO
        setting and it is the owner's to turn on.
      - There is NO EXP-gain sound in this client.
      - Two refusal paths still answer 0x00D2 with 0x0011, which a channel socket cannot
        dispatch. Nothing decoded can.

    COMMANDS: !map, !item, !exp, !heal, !job, !learn, !kit, !buff, !unbuff, !nx, !lp,
    !buy, !locker, !resetap, !resetsp,
    !npcecho, !npcfx,
    !migsweep, !exprate, !mesorate, !droprate, !setrates, !rates. !help lists them all.
      !buff [skillId] [level] [tailBytes]   cast with no skill check, MP or cooldown
      !unbuff [tailBytes]                   send the 0x007E that removes a held stat
      !nx [amount]                          grant NX. Real and displayed, but it buys
                                            NOTHING - every price tag reads LP
      !lp [amount]                          grant LEAF POINTS, the currency the shop
                                            actually charges. This is the one that buys
      !buy <commoditySN>                    buy a cash-shop sale row for real: debits NX and
                                            puts the item in the cash locker. An SN, NOT an
                                            item id - gm-handbook/commodity.txt lists all 159
      !locker [slot]                        list the cash locker, or move one slot into the
                                            Cash tab
      !resetap                              put every spent ability point back in the pool.
                                            Conserves the total - it refunds the difference
                                            from a fresh character rather than recomputing a
                                            per-level number nothing here knows
      !resetsp                              forget every skill. The points come back on their
                                            own: the pool is computed from your LEVEL, so a
                                            forgotten skill IS the refund
      !learn [level]                        NEW. Learn every skill of your current job, each
                                            clamped to ITS OWN maximum - the Magician book
                                            runs to 15 and 20, so one constant is wrong for
                                            half of it. No skill points spent. !learn 5 caps
                                            them; !learn <skillId> <level> does one
      !kit                                  NEW. Hand over the weapon and ammunition this
                                            job needs, and WARN about anything the character
                                            cannot equip. Five of the 24 first-job skills
                                            carry a weapon column: 45/46 bow or crossbow,
                                            33 dagger, 47 claw. The Magician is gated on
                                            NOTHING and still needs a wand: MagicTotal is
                                            floor(INT/2) + equipment incMAD, and the
                                            Wooden Wand's incMAD is 27. No weapon
                                            column and no weapon needed are different
                                            claims - this row had them confused

    THE FREE MEASUREMENT NOBODY HAS TAKEN
    -------------------------------------
    Every -SetFieldProbe run dumps the client's own EXP curve on the positive control's
    first hit:  python tools/decode_dump.py --exp-curve
    Compare it against data/exp-curve.txt. If they disagree, the client wins.

    THE CLIENT PATCHES ARE STILL PATCHES. Nothing here makes the session valid:

      1415db360:ret     skip the server-reachability check. NOT OPTIONAL: without it the
                        client __fastfails after ~37 seconds, because that check overruns
                        its own stack buffer when every address is unreachable.
      141b2a280:rdx=0   suppress the "trouble logging in" dialog, which otherwise blocks
                        the per-frame tick that enables the Login button.
      mode=2            leave the client's mode-5 auto-login so the button gets a turn.
      create=on         set the protected flag that gates "Create a character", re-armed on
                        every login result because the handshake zeroes it.
      hitnumber=off     OFF by default, with -ClientHitNumberPatch, and MEASURED INERT for
                        contact damage on 2026-08-28. It clears user+0x544a, which gates a
                        block that REPLACES the computed damage. But the only path that
                        computes one is entered from
                          14288b2dc  cmp dword [rdi+0xe8], 0
                          14288b2e3  jle <epilogue>
                        and template+0xe8 is the WZ node `fixedBodyAttackDamage`, which
                        ZERO of this client's 193 mobs carry - all 193 carry PADamage
                        instead. So a contact hit bails before the gate is reached, and
                        clearing the flag changes nothing. Confirmed on a client: the patch
                        armed, verified its own write, and the claim was still 1.
                        Kept because it may matter for a mob ATTACK-SKILL hit, which has
                        never been observed in 198 captures.
                        research/damage-number-two-numbers.md.
      heapfix=on        ONLY with -HeapFix, off by default, and TRIED AND FOUND USELESS on
                        2026-08-28. The three bytes at 14019b504 still do exactly what they
                        say; the TARGET was wrong. A second pooled free at 14019bb50 has the
                        identical qword header load, the route to it is fixed at compile
                        time, and a client died there UNPATCHED two days before this patch
                        existed. Fifty-six sites carry that ladder.
                        research/heapfix-did-not-hold.md.

    NOTHING AUTHENTICATES. The game socket carries no credentials at all.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -Stop
#>
[CmdletBinding()]
param(
    [switch]$Stop,
    # Show what is stored without launching anything. Cheap, and it answers "did the last
    # run actually save the character?" without spending a client launch on it.
    [switch]$ListOnly,
    [int]$Port = 8484,
    # The channel server's port. A channel is a separate process: the login server hands
    # the client this address and the client connects to it, so nothing enters the world
    # unless maplecw-world is listening here.
    [int]$ChannelPort = 8485,

    # How many channels to run. The owner, 2026-08-19: "from now on can we make sure we always
    # have 2 channels running so that users can swap between the two channels".
    #
    # This does NOT change where a login lands. The owner, same day: "In the classic world
    # startup, the user is defaulted to channel 1 of the server. We're not trying to change
    # that behavior, we're trying to allow the client to swap channels from 1 to 2 and vice
    # versa." So `world.channel_id` stays 0 and the login result still puts the player
    # there; the second channel exists to be swapped TO.
    #
    # **The UI is 1-indexed and everything here is 0-indexed.** The client's "channel 1" is
    # our channel 0 on $ChannelPort, and its "channel 2" is our channel 1 on $ChannelPort+1.
    #
    # One process per channel, not one process with two listeners - a channel IS a process
    # here, and crates/login/src/config.rs is explicit that you cannot advertise more
    # channels than you run: the client connects to the address for the channel it picked,
    # so an advertised channel with nothing behind it is one nobody can enter.
    #
    # Channel N listens on $ChannelPort + N and logs to world.log (channel 0) or
    # world-ch<N>.log (the rest). Channel 0 keeps the plain name because every doc and
    # instruction in this repo points at world.log.
    [int]$Channels = 2,
    # The FALLBACK account - who a connection is served as when no launcher claim is live.
    #
    # It used to be the only answer: the login server resolved it once at startup, so one
    # process could only ever be one player. It is now resolved PER CONNECTION, and a claim
    # staked by maplecw-launcher wins. Create the account first with
    #   .\target\release\maplecw-useradd.exe <name> --email <address>
    # The game socket still carries no credentials, so neither this nor a claim is a login:
    # they decide whose characters appear, and anything that reaches the port gets them.
    [string]$Account = 'maplecw',
    # Start maplecw-launcher instead of the client, and let IT start the client.
    #
    # This is the multi-account path: sign in as whoever you want, press Start Game, and the
    # launcher installs the hook and launches. The servers are started either way. Use it
    # when the question is about accounts; use the ordinary path when the question is about
    # the game, because the launcher writes the same marker files by a different route and
    # that is one more thing standing between a run and its evidence.
    [switch]$Launcher,
    # What the login screen displays. Server-supplied; the client cannot compute it.
    [string]$DisplayName = 'wisp****@example.com',
    [string]$Database,
    [string]$World = 'Scania',
    # REVERTED 2026-08-19. Dropping mode=2 was tried and the client crashed with an
    # access violation at 0x141177f4a right after the second 0x000B, before the login
    # screen. Server bytes were identical to the known-good run up to the login result, so
    # the regression is client-side: either this or the 141804870 watch, and both changed
    # at once. Back to the configuration that reached character select and migrated.
    #
    # mode=2 still does not do what it was added for - it is applied on 0x0000 dispatch,
    # our reply to the login request, so it lands after the auto-login it should prevent -
    # but it also routes 0x000B to the classic handler, and THAT is load-bearing. Change
    # one of these at a time, not both.
    [string]$Session = 'mode=2,create=on',
    # Six watch slots since 2026-08-20 (four before). Two are permanently spoken for -
    # 1415db360:ret and 141b2a280:rdx=0 keep the client alive - and one should always be
    # the positive control, which left exactly one for the actual question.
    #
    #   1415db360:ret     skip the reachability check - without it the client __fastfails
    #   141b2a280:rdx=0   suppress the login dialog that blocks the Login button's tick
    #   142ef3e44:hits=8  the /GS site, so a second stack overflow would still be visible
    #   141b36a10:peek=1c0  the create-result handler, reading stage+0x1c0
    #
    # The last slot is the open question. FUN_141b36a10 handles 0x0015 and compares the
    # world id we send against stage+0x1c0; if they differ it returns having registered
    # nothing and shown nothing, which is indistinguishable from the packet never arriving.
    # On 2026-08-18 the client dispatched a successful 0x0015 and stayed on the creation
    # screen, so this reads the value it compared against. It replaces the watch on
    # 142e9ebd0 (the virtualised routine), which should never be entered now that the
    # reachability check is skipped, and was not entered on the last run.
    # 1415db360:ret and 141b2a280:rdx=0 are mandatory - without them the client dies at
    # ~37s and the "trouble connecting" dialog blocks the screen.
    #
    # The 141804870 watch is GONE. It was armed to name which handshake gate raises "the
    # client is outdated", and it never fired - but the client's own uploaded error log
    # (0x0090) carries a full call stack that names the site for free, so the watch bought
    # nothing and was one of two suspects for the crash. `python tools/decode_elog.py`.
    #
    # 141b36f60 is the migration handler; 142ef3e44 is __report_gsfailure, kept because a
    # silent 37s death is the failure mode this project spends the most runs on.
    [string]$Probe = 'watch@1415db360:ret,141b2a280:rdx=0,141b36f60,142ef3e44:hits=8',
    [string]$SessionTokens = '',
    # Answer the migration hello with the fixed head of a SetField, and swap the probe for
    # the two watches that make the answer readable. See research/msexe-stage-setfield.md.
    #
    # This CANNOT put a character in a map - characterData is 0 and the branch that carries
    # a character needs an 18525-byte record decoder nobody has read. It answers exactly one
    # question: does 0x01A0 reach FUN_142097f80? Both of that handler's early returns are
    # silent, so without the watches the run cannot tell an ignored packet from one that
    # never arrived, which is the whole reason for spending the launch.
    # EVERY -SetFieldProbe run also dumps the EXP curve, and it costs nothing.
    #
    # 143AC2400 is 121 u64s, the experience needed for levels 1..120. It lives in the
    # ZERO-INITIALISED TAIL of .data - vsize 0xa2aa8, rsize 0x67400 - so it has no bytes on
    # disk at all and no static read can ever produce it. Reading it statically returned 120
    # confident wrong numbers that were actually exception-handling records, which is why
    # tools/rtti.py now raises for addresses in that region instead of answering.
    #
    # A running client has the real table. The dump rides the 140304100 positive control,
    # which fires at world entry - late enough that the table is populated - and fires ONCE,
    # so it is one log line and no extra watch slot. Decode it with tools/decode_dump.py.
    [switch]$SetFieldProbe,
    # OFF by default, and it is an EXPERIMENT rather than a fix.
    #
    # Three bytes at 14019b504 in the mapped image: the client's free reads the 64-bit pool
    # slot header where only the low half is ever legal, so a stray 1 in the high dword sends
    # a pooled 0x20 slot to HeapFree, and Windows kills the process. Reading 32 bits instead
    # returns it to the right free list - the correct outcome, not a suppression.
    #
    # Six damaged slots across three dumps, all the identical value, all in the 0x20 class,
    # accumulating with session AGE - the old 'one per 250 s' is falsified, 1046 s gave 2
    # rather than 4. research/heap-corruption-2026-08-27.md.
    #
    # Nothing in client-patched\ changes on disk. The patch verifies the three bytes before
    # writing, reads them back after, and logs both. If the client still dies with
    # 0xC0000374 with this on, research/heap-wild-write.md is wrong somewhere - which is
    # exactly what makes it worth running.
    [switch]$HeapFix,
    # Clear user+0x544a. OFF by default and MEASURED INERT on 2026-08-28: the contact path
    # bails before the gate is ever reached, because it is gated on a WZ node no mob has.
    # Kept because it may matter for a mob ATTACK-SKILL hit, which has never been observed.
    [switch]$ClientHitNumberPatch,
    # Monsters are ON by default since 2026-08-19. -NoMobs turns them off.
    #
    # -Mobs used to be the opt-in, and it cost a launch: the owner stood on map 40, which has
    # six snails, and saw none - the server had them loaded and sent none, because the
    # switch was not passed. It said so only in world.log.err. Kept as a no-op so an old
    # command line still runs.
    #
    # -MobLimit caps how many go out per field. Still useful when a mob run does fault:
    # -MobLimit 1 tells "the body is wrong" apart from "thirty objects at once".
    [switch]$Mobs,
    [switch]$NoMobs,
    [int]$MobLimit = 0,
    # Give every inventory this many slots instead of the character's own count.
    #
    # A test lever, and the numbers in it were wrong until 2026-08-20. The default is 30,
    # not 24 - the owner: "the default inventory slots is actually what appears on the screen
    # without the scroll bar, there are 6 rows of 5 slots". 30 is also the MINIMUM the
    # server will send; below it the value has no use and is clamped.
    #
    # The lever exists because a run at the default cannot tell "the server sized the bag"
    # from "the server changed nothing" - the client would plausibly have arrived at the
    # same number on its own. A run at 125, the maximum, can: the bag either shows a
    # scrollbar or it does not. That is the run that settled it.
    [int]$InventorySlots = 0,
    # Cap how many rows a shop counter sends. 0 means no cap.
    #
    # On 2026-08-20 Lucy's counter went out with twelve rows and the client threw a C++
    # exception TEN MILLISECONDS later, stopped sending anything at all, and faulted three
    # and a half seconds after that. A crash like that can come from one row being wrong or
    # from twelve rows arriving at once, and on screen those are the same picture.
    #
    # -ShopRows 1 sends a single BUY row. Both outcomes are worth a launch: the counter
    # opening means the shop path is sound and the fault is in row content or row count;
    # still crashing means the shop path itself is wrong and rows are not the variable.
    #
    # Same lever as -MobLimit, and it exists for the same reason.
    [int]$ShopRows = 0,
    # Point the two free watch slots at the melee target collector, FUN_141d31b20.
    #
    # The collector is PROVEN to run once per swing - six swings, six entries, each 1-2 ms
    # before its outbound 0x00DF, in research/fixtures/melee-collector-runs-once-per-swing-*.
    # It runs, every per-mob gate our packets can touch passes (research/mob-target-gates.md
    # section 4), and it still returns zero targets. The one path nobody has measured is the
    # early-out at 141d31c96, which returns BEFORE EXAMINING A SINGLE MOB when argument 17
    # is at least argument 4.
    #
    # Argument 4 arrives in R9 and was always in the log. Argument 17 is on the stack, and
    # until 2026-08-20 nothing could read it: the register dump stops at four, and
    # stack_trace filters to values that look like code addresses, so it discards exactly
    # the small integers this question is about. :args=17 dumps slots 5..17.
    #
    # A switch rather than a hand-typed -Probe string on purpose. Passing -Probe by hand
    # replaces ALL FOUR slots, which silently drops 140304100 - the positive control that
    # is the only thing distinguishing "the collector did not run" from "the hook never
    # armed". That mistake costs a whole launch, and launches are the scarcest thing here.
    # Watch the user state machine that stopped the owner attacking on 2026-08-20.
    #
    # FUN_140f810b0(x) is four instructions: (x->[0x5e4] & ~1) == 0x12. The melee builder
    # FUN_1428c1fa0 calls it at 1428c2053 and jne 1428c5a03 - the epilogue, 3200 listing
    # lines BEFORE its COutPacket(0x00DF). Four of the six attack builders check it. The
    # same predicate skips the drop-pool clear and refuses the pick-up pre-check.
    #
    # One field stuck at 18/19 produces every measured fact of that run: zero attacks built,
    # exactly one 0x032C, walking unaffected, and 44 more movement reports over the next six
    # and a half minutes. What PUT the user in that state is [I], not [L].
    #
    # FUN_140f810e0 is the field's only setter on this class. Watching it prints the whole
    # state machine in one ordinary session - rdx carries the value.
    [switch]$UserState,
    [switch]$MobTargets,
    [string]$ClientDir
)

$ErrorActionPreference = 'Stop'

# -SetFieldProbe swaps two of the four watch slots. The other two are not negotiable:
# 1415db360:ret and 141b2a280:rdx=0 keep the client alive and unblocked, and dropping
# either kills it at ~37s. What goes is 141b36f60, the migration handler - the channel's
# own log already proves the migration, because the client connects and sends 0x007D - and
# 142ef3e44, __report_gsfailure, whose failure mode is still visible in client-exit.log as
# an exit at ~37s, just less precisely.
#
# The two free slots moved on 2026-08-19. 142097f80 and 142cfb500 have both already done
# their job - the run of that day caught SetField entering its handler while dispatching
# 0x01A0, with the latch measured at 0x00 - so re-arming them would re-answer a settled
# question. The open question is now one step further in:
#
#   140304b20  the character-record decoder. Fires if SetField reached the record branch.
#   140302e30  the character-STAT decoder, the block presence[0] is supposed to switch on.
#
# Those two discriminate cleanly, which is the point:
#   neither fires        -> SetField never reached the record path
#   only 140304b20       -> the record decoded but the gate SKIPPED the stat block, i.e.
#                           presence[0] is the wrong byte
#   both                 -> the gate fired and the stats decoded; anything still wrong is
#                           downstream of the map id, not the presence array
#
# 140302e30 also has a built-in positive control. It has exactly two callers in the whole
# image (python tools/callers.py 0x140302e30): this record path, and the character-LIST
# path. So it should fire once at character select, on the login connection, well before
# the migration. If it fires there and NOT after SetField, the probe is demonstrably armed
# and working and the gate genuinely did not open - a silent negative that means something,
# which is the thing this project keeps having to prove the hard way.
#
# 140304b20 retired 2026-08-19: it fired, the gate opened, a character stands on map 1.
# Its slot now carries 142cfb500:peek=2358 - a FREE measurement on the portal run.
# 142cfb500 is called once per SetField from inside 142097f80, with RCX = the world object,
# so peeking +0x2358 reads the CUserLocal slot. That slot is the precondition for the SHORT
# characterData=0 SetField, the form actually designed for "same character, new map":
# research/transfer-field-request.md proved the precondition's identity but could NOT prove
# the slot is ever populated (222 readers, no store found). The peek settles it, and carries
# its own control - the first SetField should read 0, and the second says whether the short
# form is safe to use next time.
#
# Retired 2026-08-19 (round two), both answered on the run that made NPCs appear:
#   141e75800  FIRED on 0x044F from inside CField::OnPacket - the NPC routing was right all
#              along, and the body's `enabled` and `alpha` zeros were what made them invisible.
#   1420dd920  NEVER fired - 0x0138's apply loop does not run, so it is a dead end for
#              dressing the local character.
#
# **For the channel-swap run the probe is NOT the instrument - world.log is.** The swap
# request's opcode is unknown, so there is no handler to watch; the client will name its own
# request in the log the way it named 0x00D1 and 0x0151. These two slots are free
# confirmations rather than a measurement, and neither can affect the client:
#   142cfb500:peek=2358  the world object's CUserLocal slot, once per SetField - including
#                        any SetField a channel swap produces.
#   140302e30:hits=200   the character-stat decoder; it fires at character select and again
#                        per world entry, so it confirms the record path on whichever
#                        channel the client ends up on.
#
# Retired earlier, and kept for the record:
#   142cfb500:peek=2358 -> [world+0x2358] read 0x00 on the FIRST SetField and NON-ZERO on
#     every later one. So the CUserLocal slot IS populated once a field has loaded, and the
#     short characterData=0 SetField's precondition holds for map CHANGES. Measured, not
#     inferred - research/transfer-field-request.md could not settle it statically.
#   142797be0 -> it FIRED. The local character is in the pool 0x0138 searches, and the
#     handler reached its apply. Yet the character is still naked, so the failure is INSIDE
#     the apply, which walks a list at user+0x1200 and only touches the look inside the loop.
#
# The two slots now answer the two things still broken:
#   141e75800  the NPC pool's dispatcher. Two callers only - FUN_141820080 (CField::OnPacket)
#              and the channel dispatcher. If it fires while dispatching 0x044F the routing
#              and timing are right and the BODY is wrong; if it stays silent the packet is
#              never dispatched at all and the trigger or the stage is wrong.
#   1420dd920  exactly ONE call site in the image, inside FUN_142797be0's loop. It fires only
#              if that loop body runs, so it separates "the look was applied and did not
#              render" from "the loop was empty and the look was never applied".
#
# (was) 142797be0 is THE question this run answers, and it is a clean binary. It has exactly ONE
# call site in the whole image (python tools/callers.py 0x142797be0): inside FUN_142d012e0,
# the 0x0138 UserAvatarModified handler, and *past* its `if (pool_lookup != 0)` gate. So:
#   it fires   -> the local character IS in that pool and the compact avatar look was applied
#   it is silent -> the local user is not in that pool, and 0x0138 is a dead end for dressing
#                   the local character; the fallback is the ELog route (see STATUS.md)
# Either way the client is unharmed: on a miss the handler returns having done nothing.
#
# 140302e30 retired - the record path is proven on screen.
#
# (was) :hits=200 on 140302e30 because it fires at character select as well - once per
# character in the list - and the default cap is 32 per slot. The probe does announce
# saturation ("watch hit limit reached"), so this would not have been silent, but the
# decisive line is the one AFTER the migration and losing it costs a whole launch.
#
# An explicit -Probe still wins, so a run can be aimed somewhere else without editing this.
# Two free slots, and two different questions competing for them - so the flag that decides
# which run this is also decides the instrument. Getting the wrong pair costs a whole launch,
# and the two questions cannot be answered in one run anyway: mobs are off unless -Mobs.
# **The instrument follows the VARIABLE, not the feature list.**
#
# This used to key off the mob flag, and on 2026-08-19 that quietly wasted a run: mobs became
# the default, so the mob watches were armed, so the bag watch was NOT - and the run came
# back with "0 hits on 140305e48", which reads exactly like "the client never read the
# inventory size" when it actually means "nobody was watching". An unarmed instrument that
# reports a clean zero is the single most expensive failure mode on this project.
#
# So: passing -InventorySlots means the bag is the variable, and the bag gets the watches.
if ($SetFieldProbe -and -not $PSBoundParameters.ContainsKey('Probe')) {
    if ($UserState) {
        # 140f810e0 - the only setter of the user state field. EXPECT several lines; read
        #   rdx on each. A value whose (v & ~1) == 0x12 is the state that disables attacking,
        #   the drop-pool clear and the pick-up pre-check all at once.
        # 140304100:hits=200 - the equip decode at world entry. POSITIVE CONTROL.
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,140f810e0:hits=60,140304100:hits=200:dump=143AC2400/968'
    } elseif ($MobTargets) {
        # 141d31b20:args=17 - the melee target collector. EXPECT ONE ENTRY PER SWING; the
        #   pairing against 0x00DF in world.log is already established, so a missing entry
        #   here means the watch, not the client.
        #
        #   WHAT THIS NO LONGER ASKS: whether the loop starts. Settled 2026-08-20 off an
        #   existing fixture, with no launch - r9 (argument 4, the capacity) is 0xf on all
        #   six entries and the cursor is 0, so the loop runs with room for fifteen and
        #   accepts nothing. called-from= was 0x141d2545a on all six, which is arm C
        #   (FUN_141d25360) and NOT the 0x1428c2c32 the gate analysis was written against.
        #
        #   What it still buys: a17 is the output cursor, and it is the only number that
        #   says whether ANY mob was accepted part-way through the loop. called-from= names
        #   the arm in one word; look it up in research/mob-collector-callsites.md, which
        #   tables all 86 call sites by return address.
        #
        #   THE GATE IS NAMED, and it was never in the gate table. 141d327c6 rejects a mob
        #   when no rectangle of it intersected the attack rectangle - and it fails
        #   SILENTLY, because an all-zero rect satisfies the per-rect filter at 141d326ae
        #   and is SKIPPED rather than rejected. Fifteen slots, mob examined, every
        #   documented gate green, nothing accepted.
        #
        #   The chain reaches ONE field: mob+0xa88, the animation object. The mob
        #   constructor leaves it NULL and it gates the rect in two independent places
        #   (141cb4645 and 141c57185), so these two watches read the CAUSE beside the
        #   EFFECT rather than reading the same rect twice:
        #     141d32675:peek=0xa88   the animation object
        #     141d3267c:peek=0x42c   the cached body rect, left and top
        #   0xa88 null                 -> the chain explains it end to end, and the next
        #                                 question is what makes FUN_141cd1620 run - a
        #                                 SPAWN-path question, not an attack-path one
        #   0xa88 set, 0x42c zero      -> the object exists but the rect was never
        #                                 computed; FUN_141c68d80 never ran for our mobs
        #   both sane                  -> mob geometry is fine, and the remaining lead is
        #                                 the ATTACK rect, which arm C never validates
        #                                 (arm A checks left<right / top<bottom; arm C does
        #                                 not)
        # 140304100:hits=200:dump=143AC2400/968 - the equip decode at world entry. POSITIVE CONTROL: no lines
        #   at all means the hook never armed and the log proves nothing. It KEEPS its slot:
        #   WATCH_SLOTS went from four to six on 2026-08-20 rather than trade the control
        #   away to fit a measurement, which is a trade this project has lost before.
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,141d32675:peek=0xa88:hits=40,141d3267c:peek=0x42c:hits=40,140304100:hits=200:dump=143AC2400/968'
    } elseif ($InventorySlots -gt 0) {
        # 140305e48:peek=24 - the u16 that sizes ONE inventory, inside the record decoder's
        #   fixed six-turn loop. RCX is the CInPacket and +0x24 is its read cursor. EXPECT
        #   SIX HITS, EACH EXACTLY 2 APART. Origin-independent: it does not matter what the
        #   cursor counts from, only that the client took twelve contiguous bytes where we
        #   put twelve. Fewer than six, or an uneven step, means presence[7] is wrong.
        # 140304100:hits=200:dump=143AC2400/968 - the equip decode at world entry. POSITIVE CONTROL: no lines
        #   at all means the hook never armed and the log proves nothing.
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,140305e48:peek=24:hits=20,140304100:hits=200:dump=143AC2400/968'
    } else {
        # THE CASH SHOP IS THE RUN, so the free slots point at the two addresses that make
        # a blank screen readable. All six slots are in use.
        #
        # The watch line already prints "while dispatching opcode 0x%04X" and rdx, so these
        # cost nothing to read and pair directly with world.log - CLAUDE.md, "count the same
        # event in two logs".
        #
        # 140d78070:peek=0x74:hits=20 - THE INSTRUMENT FOR THIS RUN. It is the buy entry
        #   point, and its FIRST real instruction is `cmp byte [rcx+0x74], 0` at 140d78096,
        #   with rcx the real `this` - so the peek reads the shop's in-flight latch AT THE
        #   MOMENT THE PLAYER CLICKS BUY. Verified by disassembling it, not assumed.
        #     click 1 reads 0                 -> the shop was free, as expected
        #     click 2 and 3 also read 0       -> our 0x19 released it. The purchase is closed
        #     click 2 reads 1                 -> 0x19 did NOT clear it. That is exactly the
        #                                        failure bRelease exists to prevent, and it
        #                                        localises to one byte
        #   It replaced 14209ad60, whose question (does 0x01A3 reach its handler) is now
        #   answered twice over, and whose remaining use - re-entry - is visible on screen
        #   and in world.log without spending a slot.
        # 140d734e0:hits=40 - the cash shop stage's OWN OnPacket. A line means a stage object
        #   exists and is receiving, and rdx names which of 0x5AD/0x5AE/0x5B9/0x5BA arrived.
        #     rdx=0x5ad -> the wallet was accepted. Read the balance off the screen
        #     rdx=0x5ae -> OUR REFUSAL WAS ACCEPTED. No 0x05AE has ever been on this wire,
        #                  so this line is the first evidence the 0x1A sub-op is right
        #     rdx=0x453 -> NPC chatter, sent to a player standing in the shop. 38 of them
        #                  last run. The stage ignores it; it is noise, and it is on the
        #                  list to stop sending
        #     silent, but 14209ad60 fired -> the handler ran and built no stage: the body
        # 140d785f0:hits=20 - THE BUY BUILDER, and it now counts something new. The wallet
        #   reply re-enters it on its COMPLETION path - that is what fetches string 590, "You
        #   have successfully made the purchase." So per purchase:
        #     TWO hits  -> the click, then the completion. The success message came from us
        #                  sending 0x0C and letting 0x05AD do the rest. This is the design
        #     ONE hit   -> the re-entry did not happen; expect no success message
        #     THREE OR MORE, and the balance dropping -> the re-entry SENDS rather than
        #                  completes. That is the purchase loop. Close the client
        # 140304100:hits=200:dump=143AC2400/968 - the equip decode at world entry. POSITIVE
        #   CONTROL: no lines at all means the hook never armed and the log proves nothing.
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,140d78070:peek=0x74:hits=20,140d734e0:hits=60,140d785f0:hits=20,140304100:hits=200:dump=143AC2400/968'
    }
    # Announce which pair actually got armed. The old line said "mobs" for 141c532ab, which
    # is the mob SPAWN decoder - now that -MobTargets arms a mob TARGETING watch, one word
    # would have covered two different runs. Same precedence as the if/elseif above, and
    # written as three statements because 5.1 has no ternary.
    $pair = "THE PURCHASE (140d78070 peeking the in-flight latch AT THE CLICK, 140d785f0 = the buy builder, 140d734e0 = the stage OnPacket)"
    if ($InventorySlots -gt 0) { $pair = "THE BAG (140305e48)" }
    if ($MobTargets) { $pair = "MOB TARGETING (mob+0xa88 and mob+0x42c in the collector loop)" }
    if ($UserState) { $pair = "THE USER STATE FIELD (140f810e0, rdx is the value)" }
    Write-Host ("probe pair: " + $pair) -ForegroundColor Cyan
}

$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$root = Split-Path -Parent $here
if (-not $ClientDir) { $ClientDir = Join-Path $root 'client-patched' }
if (-not $Database) { $Database = Join-Path $root 'maplecw.db' }
$exe = Join-Path $ClientDir 'MapleStory.exe'
$loginExe = Join-Path $root 'target\release\maplecw-login.exe'
$worldExe = Join-Path $root 'target\release\maplecw-world.exe'
$userAdd = Join-Path $root 'target\release\maplecw-useradd.exe'
$serverLog = Join-Path $root 'login.log'
$worldLog = Join-Path $root 'world.log'

function Stop-All {
    # Never pipe a native command's stderr under PowerShell 5.1: it wraps each line in an
    # ErrorRecord, which $ErrorActionPreference='Stop' then treats as fatal, so a taskkill
    # reporting "process not found" - the normal case - would abort the script.
    $prev = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        # The client ignores Stop-Process (Themida); taskkill is the one that works.
        if (Get-Process MapleStory -ErrorAction SilentlyContinue) {
            taskkill /F /IM MapleStory.exe /T | Out-Null
        }
        if (Get-Process maplecw-login -ErrorAction SilentlyContinue) {
            taskkill /F /IM maplecw-login.exe | Out-Null
        }
        if (Get-Process maplecw-world -ErrorAction SilentlyContinue) {
            taskkill /F /IM maplecw-world.exe | Out-Null
        }
    } finally {
        $ErrorActionPreference = $prev
    }
}

if ($Stop) {
    Stop-All
    # Leave no marker behind: a stale probe marker would silently turn the next ordinary
    # run into an instrumented one.
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.probe') -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.enable') -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.session') -ErrorAction SilentlyContinue
    Write-Host 'stopped client and login server'
    exit 0
}

# The account has to exist. Creating one here would mean inventing a password, and this
# repo does not do that - it prompts, and it hashes.
if (-not (Test-Path $Database)) {
    Write-Host ''
    Write-Host "No database at $Database. Create the account first:"
    Write-Host "  $userAdd --db `"$Database`" $Account"
    throw 'no database'
}

# -ListOnly answers a question about the DATABASE, so it runs before anything is killed or
# rebuilt. It used to sit after the build, which made it neither cheap nor side-effect-free:
# it would stop a running server and fail on a build it did not need.
if ($ListOnly) {
    if (-not (Test-Path $loginExe)) {
        throw "$loginExe is missing - run without -ListOnly once to build it"
    }
    & $loginExe --list --db $Database --account $Account --world $World
    exit $LASTEXITCODE
}

# Kill the previous run BEFORE building, not after.
#
# A server from the last launch holds target/release/maplecw-world.exe open, and on Windows
# cargo cannot replace a running executable: the build fails outright with "Access is denied"
# and the whole run stops before it starts. Stop-All used to run further down, after the
# build, which meant a second launch in a row could not work at all. Found 2026-08-19 when
# the servers from the owner mob run blocked a rebuild.
Stop-All
Start-Sleep -Milliseconds 300

Push-Location $root
try {
    # -p world is NOT optional, and it was missing until 2026-08-19. The script starts
    # target/release/maplecw-world.exe but never built it, so every change to the channel
    # server reached a run only if someone had happened to build it by hand - the same
    # silent-stale-binary failure the grap-stub note above warns about, one crate over.
    # `auth` is here for maplecw-useradd, which this script now calls to clear a stale login
    # claim. `launcher` only when it is going to be run - it pulls eframe, and a first build
    # of that is minutes nobody asked for on a run that is not about accounts.
    if ($Launcher) {
        & cargo build --release -p login -p world -p store -p auth -p grap-stub -p launcher
    } else {
        & cargo build --release -p login -p world -p store -p auth -p grap-stub
    }
    if ($LASTEXITCODE -ne 0) { throw 'build failed' }
}
finally { Pop-Location }

# Rebuilding grap-stub does not update the client: cargo writes target/release/grap64.dll
# and the client loads client-patched/grap64.dll. Skipping this is the most expensive kind
# of failure here, because the run looks normal and the new hook code simply is not there.
& powershell -ExecutionPolicy Bypass -File (Join-Path $here 'setup-client.ps1')
if ($LASTEXITCODE -ne 0) { throw 'setup-client.ps1 failed - the client would run the old hook' }

Stop-All
Start-Sleep -Milliseconds 300

# Built on its own line rather than inline in the -ArgumentList, and that is not style:
# see the note at the '--channels' argument below. A syntax check does NOT catch the inline
# form - the script parses fine and the server gets one mangled argument - so this was found
# only when the owner ran it. Verify a change to this block by STARTING the server, not by parsing.
$channelList = (0..($Channels - 1) | ForEach-Object { "127.0.0.1:$($ChannelPort + $_)" }) -join ','

# Keep the previous run's log instead of deleting it.
#
# **Three separate conclusions have died with an overwritten world.log.** The most expensive:
# an attack capture showing 127-byte zero-target bodies was read out of a world.log that also
# carried 90 mob-control packets, the pair was reported as "the client will not target our
# mobs", and by the time anyone tried to re-check it the file had been replaced by the next
# launch. No fixture had been taken. The observation was real and is now unverifiable, which
# is the worst of both.
#
# A client launch costs the owner a manual launch. Throwing away its output to save a few hundred
# kilobytes is the wrong trade in every direction.
# `-Into` exists so the HOOK log can be archived next to the server logs rather than into a
# second buffer under client-patched\. `CLAUDE.md`'s "count the same event in two logs"
# needs both halves of one run in one place: world.log says what the server SENT, the hook
# log says what the client DID with it, and three of this project's answers came from the
# two disagreeing. One of them being archived and the other deleted made that impossible
# for every run but the current one.
function Save-PreviousLog([string]$Path, [string]$Into) {
    if (-not (Test-Path $Path)) { return }
    $dir = if ($Into) { $Into } else { Split-Path -Parent $Path }
    $prev = Join-Path $dir 'previous-runs'
    if (-not (Test-Path $prev)) { New-Item -ItemType Directory -Path $prev | Out-Null }
    $stamp = (Get-Item $Path).LastWriteTime.ToString('yyyyMMdd-HHmmss')
    $name = [IO.Path]::GetFileNameWithoutExtension($Path)
    $ext = [IO.Path]::GetExtension($Path)
    Move-Item $Path (Join-Path $prev ("{0}-{1}{2}" -f $name, $stamp, $ext)) -Force
}

Save-PreviousLog $serverLog
Remove-Item $serverLog -Force -ErrorAction SilentlyContinue

# On the ordinary path, EXPLICIT CONFIGURATION WINS.
#
# A launcher claim says "serve the next connection as this account" and lives for hours. That
# is right when the launcher is driving. It is a trap when it is not: this run passes
# --account, and a claim left over from a launcher run yesterday would quietly serve that
# account instead - which on screen is someone else's characters, with a plan full of steps
# that then all read as broken. The login server logs which one it used every time, but the
# banner the owner reads at launch is not the server log.
#
# -Launcher deliberately does NOT do this: there, the claim is the whole point.
if (-not $Launcher) {
    $userAddExe = Join-Path $root 'target\release\maplecw-useradd.exe'
    if (Test-Path $userAddExe) {
        & $userAddExe --db "$Database" --clear-claims | ForEach-Object { Write-Host "  $_" }
    }
}

$server = Start-Process -FilePath $loginExe -WorkingDirectory $root -PassThru `
    -WindowStyle Hidden `
    -ArgumentList @(
        '--db', "`"$Database`"", '--bind', "127.0.0.1:$Port",
        # One address per channel. The client connects to this when it enters the world,
        # so it must be reachable from the *client* machine - loopback here, a LAN address
        # once the server moves to the homelab.
        # The -join MUST be fully parenthesised. PowerShell's -join binds looser than the
        # commas of an array literal, so `(...) -join ',', '--account', $Account` makes the
        # rest of the argument list part of the join's right operand and collapses the whole
        # thing into one string - the server then sees a single argument "--db,...".
        '--channels', $channelList,
        '--account', $Account, '--display-name', "`"$DisplayName`"", '--world', $World
    ) `
    -RedirectStandardOutput $serverLog -RedirectStandardError "$serverLog.err"

$worldSrv = $null
foreach ($ch in 0..($Channels - 1)) {
    $chPort = $ChannelPort + $ch
    $chLog = if ($ch -eq 0) { $worldLog } else { Join-Path $root "world-ch$ch.log" }
    Save-PreviousLog $chLog
    Remove-Item $chLog -Force -ErrorAction SilentlyContinue
    $chArgs = @('--db', "`"$Database`"", '--bind', "127.0.0.1:$chPort", '--channel', "$ch")
    # Every channel is told where every channel listens, because Change Channel (0x00D2)
    # arrives on the CHANNEL connection and has to be answered with the target's address.
    # Same list the login server advertises, built from the same two numbers, so the two
    # cannot drift into advertising a channel nobody can enter.
    $chArgs += @('--channels', $channelList)
    if ($SetFieldProbe) { $chArgs += '--set-field-probe' }
    if ($NoMobs) { $chArgs += '--no-mobs' }
    if ($MobLimit -gt 0) { $chArgs += @('--mob-limit', "$MobLimit") }
    if ($ShopRows -gt 0) { $chArgs += @('--shop-rows', "$ShopRows") }
    if ($InventorySlots -gt 0) { $chArgs += @('--inventory-slots', "$InventorySlots") }
    $p = Start-Process -FilePath $worldExe -WorkingDirectory $root -PassThru `
        -WindowStyle Hidden `
        -ArgumentList $chArgs `
        -RedirectStandardOutput $chLog -RedirectStandardError "$chLog.err"
    if ($ch -eq 0) { $worldSrv = $p }
    Write-Host "channel $ch on 127.0.0.1:$chPort (pid $($p.Id)), log $chLog"
}

# Never launch the client against a dead server. A server that exited - a missing account
# is the usual reason - leaves the client on "Connecting..." forever, which looks like a
# client problem and is not.
Start-Sleep -Milliseconds 700
if ($server.HasExited) {
    Write-Host ''
    Write-Host '--- the login server exited immediately ---'
    if (Test-Path "$serverLog.err") { Get-Content "$serverLog.err" | Write-Host }
    if (Test-Path $serverLog) { Get-Content $serverLog | Write-Host }
    throw 'the login server did not start'
}
Write-Host "login server pid $($server.Id) -> $serverLog"
Get-Content $serverLog -ErrorAction SilentlyContinue | ForEach-Object { Write-Host "  $_" }

if ($Launcher) {
    # Hand over to maplecw-launcher and stop here, BEFORE the marker block below.
    #
    # The launcher writes the same four hook markers and launches the client itself. Placed
    # after that block - where this used to sit - both would write them and the launcher
    # would silently win, so a custom -Probe would be replaced by the launcher's built-in
    # default and the run would come back missing the watches it was launched for. That is
    # the "a stale instrument answers" failure this repo keeps paying for, so there is one
    # writer and it is whichever of the two is driving.
    #
    # The cost, stated rather than hidden: on this path -Probe, -Session, -HeapFix and
    # -ClientHitNumberPatch DO NOTHING. The launcher's defaults are byte-identical to this
    # script's defaults today (client::DEFAULT_PROBE / DEFAULT_SESSION), so an ordinary run
    # is unaffected - but if you are here to arm a watch, use the ordinary path.
    $launcherExe = Join-Path $root 'target\release\maplecw-launcher.exe'
    if (-not (Test-Path $launcherExe)) {
        throw "no launcher at $launcherExe - build it with: cargo build --release -p launcher"
    }
    Write-Host ''
    Write-Host 'THE LAUNCHER IS DRIVING THIS RUN.' -ForegroundColor Cyan
    Write-Host '  Sign in with an account name OR its email, check the server IP, press'
    Write-Host '  Login, then Start Game. The login server picks up whoever you signed in'
    Write-Host '  as - it is resolved per connection now, so no restart is needed to swap.'
    Write-Host ''
    # The leading `&` is REQUIRED and is not decoration. PowerShell parses a line that starts
    # with a quoted string as a string expression, not as a command, so pasting one of these
    # without it fails with "Unexpected token 'db'" and "The '--' operator works only on
    # variables". Every pasteable line in this repo that starts with a quoted path needs it.
    Write-Host ("  accounts:  & `"{0}\target\release\maplecw-useradd.exe`" --db `"{1}`" --list" -f $root, $Database)
    Write-Host ("  add one:   & `"{0}\target\release\maplecw-useradd.exe`" --db `"{1}`" <name> --email <addr>" -f $root, $Database)
    Write-Host ''
    Start-Process -FilePath $launcherExe -WorkingDirectory $root | Out-Null
    Write-Host 'launcher started. Stop the servers when done:' -ForegroundColor Green
    Write-Host ("  powershell -ExecutionPolicy Bypass -File `"{0}\tools\test-server.ps1`" -Stop" -f $root)
    return
}

# The hook is switched on by marker files, because ShellExecute does not carry $env: into
# the child - a launcher will replace all of this with one config file (docs/launcher.md).
$hookLog = Join-Path $ClientDir 'maplecw-hook.log'
New-Item -ItemType File -Path (Join-Path $ClientDir 'maplecw-hook.enable') -Force | Out-Null
# ARCHIVED, not deleted - and it used to be deleted, while world.log beside it was kept.
# That asymmetry is exactly the trap CLAUDE.md describes: the hook log is the only record
# of what the CLIENT did with a packet, it is where the dispatch lines and the CLIENT FAULT
# line live, and every previous run's copy was thrown away at the next launch.
Save-PreviousLog $hookLog $root
Remove-Item $hookLog -Force -ErrorAction SilentlyContinue
$env:MAPLECW_HOOK_LOG = $hookLog
# Where the hook writes a crash dump. A MARKER FILE, not $env: - ShellExecute does not
# carry the environment into the client, which is why MAPLECW_HOOK_LOG above never
# actually arrives and the hook log lands beside the client by fallback.
#
# The hook writes its own dump now because WER will not. That is measured, not assumed:
# on 2026-08-21 a decoy named MapleStory.exe that does nothing but dereference null
# produced a 9.4 MB dump here, while the real client's own 0xC0000005 at 13:49:56 -
# 88 minutes AFTER WER was switched on - produced nothing at all.
$dumpDir = Join-Path $root 'dumps'
New-Item -ItemType Directory -Path $dumpDir -Force | Out-Null
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.dumpdir') -Value $dumpDir -Encoding ascii
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.probe') -Value $Probe -Encoding ascii
if ($HeapFix) { $Session = "$Session,heapfix=on" }
if ($ClientHitNumberPatch) { $Session = "$Session,hitnumber=off" }
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.session') -Value $Session -Encoding ascii
Write-Host "client patches: $Probe"
Write-Host "session patches: $Session"
if ($SetFieldProbe) {
    Write-Host ""
    Write-Host "READ THE BANNER ABOVE BEFORE LAUNCHING." -ForegroundColor Cyan
    Write-Host "  Three lines say whether this run can test anything:"
    Write-Host "    footholds: 94089 segments across 426 maps"
    Write-Host "    consumables: 44 items restore something"
    Write-Host "    quests: 322 loaded, 5 of them carrying an authored script overlay"
    Write-Host "  NONE LOADED / no overlay -> regenerate and relaunch:"
    Write-Host "    python tools/dump_portals.py     (footholds)"
    Write-Host "    python tools/dump_itemdata.py    (consumables)"
    Write-Host ""
    Write-Host "THE HOOK WRITES ITS OWN CRASH DUMP, AND IT HAS NOW DONE SO." -ForegroundColor Cyan
    Write-Host "  1010 MB on the 20:10 run, the first this project has ever had."
    Write-Host "  Windows Error Reporting was never going to work for this client, and"
    Write-Host "  that is measured rather than assumed: a decoy named MapleStory.exe that"
    Write-Host "  only dereferences null wrote a 9.4 MB dump into dumps\ - while the real"
    Write-Host "  client's own 0xC0000005, 88 minutes AFTER WER was switched on, wrote"
    Write-Host "  nothing. Same machine, same hour, same exception. The client ships its"
    Write-Host "  own crash reporting and never reaches WerFault."
    Write-Host "  So the dump comes from the vectored handler that was already catching"
    Write-Host "  the fault and only logging it. No WER involved."
    Write-Host ""
    Write-Host "  IF THE CLIENT DIES, LOOK HERE:" -ForegroundColor Yellow
    Write-Host "    $dumpDir\maplecw-crash-<pid>-<code>-1.dmp"
    Write-Host "    and two lines in the hook log: 'CRASH DUMP: writing' then 'wrote'"
    Write-Host "      both lines      -> a dump. Move it out; each is about a gigabyte"
    Write-Host "      only 'writing'  -> the dump attempt died partway. Still evidence,"
    Write-Host "                         and NOT the same as never having tried"
    Write-Host "      neither         -> the fault is not one we match, or it killed the"
    Write-Host "                         process before the handler ran"
    Write-Host ""
    Write-Host "In client-patched\maplecw-hook.log:" -ForegroundColor Cyan
    if ($InventorySlots -gt 0) {
        Write-Host "  140305e48   the inventory-size read. Six lines, cursor stepping by 2."
    } else {
        Write-Host "  141c532ab   inside the mob's encodeInit. Regression check only."
    }
    Write-Host "  140304100   the equip decode at world entry. POSITIVE CONTROL - no lines"
    Write-Host "              at all means the hook never armed and the log proves nothing."
    Write-Host "              It arms ~4.5s after connect."
}
# ShellExecute is required: the client has an elevation manifest, and CreateProcess fails
# with "requires elevation".
$launchArgs = @('-NXLDEBUG', '127.0.0.1', "$Port")
if ($SessionTokens) {
    $launchArgs += ($SessionTokens -split '\s+' | Where-Object { $_ })
    Write-Host "session tokens (config +0x90): $SessionTokens"
    Write-Host '  -> read login.log for the 0x0073 body and look for them'
}
$p = Start-Process -FilePath $exe -WorkingDirectory $ClientDir `
    -ArgumentList $launchArgs -PassThru

$exitLog = Join-Path $root 'client-exit.log'
Remove-Item $exitLog -ErrorAction SilentlyContinue
Start-Process -FilePath 'powershell' -WindowStyle Hidden -ArgumentList @(
    '-ExecutionPolicy', 'Bypass', '-File', "`"$(Join-Path $here 'exit-forensics.ps1')`"",
    '-ClientPid', $p.Id, '-Log', "`"$exitLog`""
) | Out-Null

# Keep the host usable while a dialog is being read; the client otherwise saturates it.
try {
    $p.PriorityClass = 'BelowNormal'
    $cores = [Environment]::ProcessorCount
    if ($cores -gt 2) {
        $p.ProcessorAffinity = [IntPtr](([long][Math]::Pow(2, $cores) - 1) -band -bnot 1)
    }
} catch {
    Write-Host "could not lower client priority: $_"
}

# Read the command line back off the running process rather than trusting what we meant to
# pass. A parameter that never arrived looks exactly like one that arrived and did nothing.
$actual = (Get-CimInstance Win32_Process -Filter "ProcessId = $($p.Id)" -ErrorAction SilentlyContinue).CommandLine
if ($actual) { Write-Host "launched: $actual" } else { Write-Host 'launched: (command line unreadable)' }

Write-Host ''
Write-Host 'On screen:'
if ($SetFieldProbe) {
    Write-Host '  STEPS 2-9, plus 5b. 1 is a read.' -ForegroundColor Yellow
    Write-Host '  ORDER: 8 FIRST (it only means anything in a young session),' -ForegroundColor Yellow
    Write-Host '  then 2 with 3 inside it, then 4, 5, 5b, 7, 9. Step 1 is a READ.' -ForegroundColor Yellow
    Write-Host '  Full text: Get-Help on this script.'
    Write-Host ''
    Write-Host '  CONFIRMED LAST RUN, DO NOT RE-TEST.' -ForegroundColor Green
    Write-Host '  SELLING WORKS and the counter DRAWS - 0x055D, Lucy, 12 rows,'
    Write-Host '  1905 bytes = 21 + 12x157 exactly.'
    Write-Host '  No Buy Back tab in this client. Closed, not coming back.'
    Write-Host '  WARRIOR WORKS. Skills cast, and the client refuses at 0 MP.'
    Write-Host '  AND THE RED POTION NEVER RESTORED MP. Its spec is hp 100, mp 0'
    Write-Host '  and the log says "+0 mp". The CLIENT had been spending MP locally'
    Write-Host '  while the SERVER never did, so our stale 181/181 rode along on the'
    Write-Host '  potion stat change and the client believed it. FIXED: attack skills'
    Write-Host '  now cost MP server-side - the first thing the skill id bought.'
    Write-Host '  (The /hitdamagetest route is dead: no 0x0189 came back, so the'
    Write-Host '  permission gate refused it. But that was one lever failing, not the'
    Write-Host '  answer - see step 1, which you were right to push back on.)'
    Write-Host '  YOU STAY DEAD - no more regenerating out of death. The wallet, the'
    Write-Host '  cash shop window, Exit, and the leaf-point balance. !lp. THE JOB'
    Write-Host '  CHANGE, THE SKILL POINTS AND THE MAGICIAN BOOK - !job 200 gave the'
    Write-Host '  effect, the points showed, the + button was live, all six skills'
    Write-Host '  listed, and Magic Claw took seven points.'
    Write-Host '  AND MAGIC CLAW DEALING 1 WAS NOT A BUG. You were a Rogue with 6'
    Write-Host '  INT wearing a Magician job id. The formula predicts exactly 1.'
    Write-Host '  That is why steps 2, 4 and 5 all start by fixing the stat.'
    Write-Host ''
    Write-Host '  TWO NEW COMMANDS DO THE SETUP FOR YOU:' -ForegroundColor Cyan
    Write-Host '     !learn        every skill of your job at its own max level.'
    Write-Host '                   No skill points needed. !learn 5 caps them at 5.'
    Write-Host '     !kit          the weapon and ammunition that job needs, and it'
    Write-Host '                   WARNS if you cannot equip what it just gave you.'
    Write-Host ''
    Write-Host '  1. NOTHING TO TEST. The 1 is settled and it is not fixable' -ForegroundColor White
    Write-Host '     from the server. Read this once, then skip to 2.'
    Write-Host '     I told you the client computes contact damage and throws it'
    Write-Host '     away. WRONG - the patch armed, verified its own write, and the'
    Write-Host '     claim was still 1. The path I found is a FIXED-DAMAGE OVERRIDE:'
    Write-Host '       14288b2dc  cmp dword [rdi+0xe8], 0'
    Write-Host '       14288b2e3  jle <epilogue>'
    Write-Host '     and template+0xe8 is the WZ node fixedBodyAttackDamage, which'
    Write-Host '     ZERO of this client 193 mobs carry - all 193 carry PADamage.'
    Write-Host '     So every contact hit bails before the gate is even reached.'
    Write-Host '     I found a real calculation and did not check its precondition.'
    Write-Host '     One wz-dump would have said so before costing you a launch.'
    Write-Host '     The client has no contact-damage calculation in this build, so'
    Write-Host '     it is the SERVER that must supply the number - and which packet'
    Write-Host '     carries it is not found. Two numbers stay for now; ours is right.'
    Write-Host '     The patch is off by default now. -ClientHitNumberPatch re-arms it.'
    Write-Host ''
    Write-Host '  2. WARRIOR - THE WHOLE BRANCH, AND THE CHEAPEST ONE.' -ForegroundColor White
    Write-Host '     FIRST OF THE FOUR BRANCHES (step 7 comes before all of them).'
    Write-Host '     It is the only branch whose weapon has NO'
    Write-Host '     requirement at all, and none of its skills is gated on a weapon,'
    Write-Host '     so it separates "does a skill attack work" from "am I holding'
    Write-Host '     the right thing" - which is exactly what steps 4 and 5 cannot.'
    Write-Host '       !resetap        then put points into STR until it reads 35+'
    Write-Host '       !job 100'
    Write-Host '       !kit            gives a Sword, 1302000, free to anyone'
    Write-Host '       --- STOP HERE AND DO STEP 3 NOW, BEFORE !learn ---' -ForegroundColor Magenta
    Write-Host '       !learn          all six Warrior skills at max'
    Write-Host '     EQUIP THE SWORD. Then cast Power Strike and Slash Blast.'
    Write-Host '     The six are: Improved HP Recovery, Max HP Increase, Precise'
    Write-Host '     Strikes (all passive), Iron Body (buff), Power Strike and'
    Write-Host '     Slash Blast (attacks).'
    Write-Host '       a) do Power Strike and Slash Blast do REAL damage, well above 1?'
    Write-Host '       b) does Slash Blast hit up to FOUR mobs at once? That is its'
    Write-Host '          mobCount and it is the one thing that tells it from Power Strike'
    Write-Host '       c) IRON BODY - NEWLY FIXED, and this is the one measurement I'
    Write-Host '          need. It never reduced damage because our own model summed'
    Write-Host '          equipment defence and NOTHING ELSE - the buff set the bit,'
    Write-Host '          drew the icon, cost MP, and the arithmetic never saw it.'
    Write-Host '          Third time a buff has done that here. Now it feeds in.'
    Write-Host '          Its WZ value at level 20 is 25, and whether that means +25%'
    Write-Host '          or +25 FLAT is inferred, not read. Those are hard to tell'
    Write-Host '          apart on a low-defence character - which is why I need the'
    Write-Host '          NUMBER: does W. Def in the stat window go UP, and by how'
    Write-Host '          much? SAY THE NUMBER BEFORE AND AFTER. This is a measurement,'
    Write-Host '          not a yes/no:'
    Write-Host '            it rises by about a QUARTER of what it was -> percent, and'
    Write-Host '                     our conversion is right'
    Write-Host '            it rises by exactly 25 whatever it started at -> the value is'
    Write-Host '                     FLAT and we are dividing when we should not. One line'
    Write-Host '            SAY BOTH NUMBERS either way - that is what settles it'
    Write-Host '            it does not move at all -> CTS bit 86 is wrong, and Magic'
    Write-Host '                     Armor rests on the same bit'
    Write-Host '       d) MP should drop on every cast. Does it?'
    Write-Host ''
    Write-Host '  3. THE PASSIVE QUESTION - ONE COMMAND, AND IT DECIDES A DESIGN.' -ForegroundColor White
    Write-Host '     Nine of the 24 first-job skills are passive. The client appears'
    Write-Host '     to apply them itself; if it does not, the server has to fold'
    Write-Host '     every one of them in by hand. Seven of the nine change nothing'
    Write-Host '     this server owns, so they cannot be tested. TWO can.'
    Write-Host '     DO THIS IN THE MIDDLE OF STEP 2 - after !kit, BEFORE !learn.'
    Write-Host '     !learn hands out Max HP Increase at level 15 along with everything'
    Write-Host '     else, and once it is learned the before-number is gone. If you'
    Write-Host '     have already run !learn, use !resetsp to forget the book first.'
    Write-Host '     WRITE DOWN YOUR MAX HP. Then:'
    Write-Host '       !learn 1000001 15      Max HP Increase, tooltip says +25%'
    Write-Host '     Look at Max HP again.'
    Write-Host '       it went UP by about a quarter -> the client applies passives'
    Write-Host '                     itself. The server must NOT also apply them or'
    Write-Host '                     everything doubles'
    Write-Host '       it did not move          -> the client does nothing with them and'
    Write-Host '                     the server owes nine skills their effect'
    Write-Host '     Either answer is worth the run. They point opposite ways.'
    Write-Host ''
    Write-Host '  4. BOWMAN - AND THIS ONE HAS A STAT GATE THAT CAN STOP YOU.' -ForegroundColor White
    Write-Host '       !resetap        then put points into DEX until it reads 35+'
    Write-Host '       !job 300'
    Write-Host '       !kit            War Bow 1452000, and 1000 Arrows 2060000'
    Write-Host '       !learn'
    Write-Host '     THE BOW NEEDS LEVEL 10 AND DEX 25. There is no free bow in this'
    Write-Host '     client - all 230 weapons were read to check. If !kit prints a'
    Write-Host '     WARNING, you cannot equip it yet; fix the stat first.'
    Write-Host '     EQUIP THE BOW. The arrows go in the USE tab and the client picks'
    Write-Host '     them up itself.'
    Write-Host '       a) does Arrow Blow fire, and does the arrow count drop by 1?'
    Write-Host '       b) does Double Shot fire, and does it drop by 2?'
    Write-Host '       c) FOCUS: do ACCURACY and AVOIDABILITY both rise? Say both.'
    Write-Host '            both rise      -> bits 88 and 89 are right'
    Write-Host '            only accuracy  -> 89 is not Avoidability. We sweep 89..91'
    Write-Host '            neither        -> the window may be showing equipment only'
    Write-Host '       d) UNEQUIP THE BOW and try Arrow Blow. It SHOULD refuse - the'
    Write-Host '          client enforces that itself. If it fires anyway, say so'
    Write-Host ''
    Write-Host '  5. THIEF - TWO WEAPONS, AND YOU MUST SWAP BETWEEN THEM.' -ForegroundColor White
    Write-Host '     THIS IS THE ONE THAT WILL LOOK BROKEN IF YOU FORGET.'
    Write-Host '     Double Stab needs a DAGGER. Lucky Seven needs a CLAW. No item'
    Write-Host '     in this client is both, so one of them always refuses.'
    Write-Host '       !resetap        then put points into LUK until it reads 35+'
    Write-Host '       !job 400'
    Write-Host '       !kit            Razor 1332000, Garnier 1472000, 500 stars 2070000'
    Write-Host '       !learn'
    Write-Host '       a) EQUIP THE RAZOR (dagger). Cast Double Stab. Two hits per cast'
    Write-Host '       b) SWAP TO THE GARNIER (claw). Cast Lucky Seven'
    Write-Host '       c) DARK SIGHT: do you go TRANSLUCENT, and are you SLOWER?'
    Write-Host '            translucent AND slower -> bit 99 and the speed penalty'
    Write-Host '            translucent, same speed -> the penalty was dropped'
    Write-Host '            slower, not translucent -> a flag of 1 is not enough'
    Write-Host '          THEN RIGHT-CLICK THE ICON to cancel it. Are you back to'
    Write-Host '          normal speed AND visible? If you stay slow, the off-path is'
    Write-Host '          clearing one bit of two'
    Write-Host '       d) Disorder will say it grants nothing. That is correct and'
    Write-Host '          deliberate - it is a debuff on the MOB and we have no packet'
    Write-Host '          for that. Just check it does not freeze anything'
    Write-Host ''
    Write-Host '  5b. MAGICIAN - THE WAND, WHICH IS THE BUG YOU HIT.' -ForegroundColor White
    Write-Host '     !kit told you the Magician needs nothing. Wrong, and it was my'
    Write-Host '     error: no Magician skill is GATED on a weapon, and I read that as'
    Write-Host '     no weapon needed. MagicTotal is floor(INT/2) + equipment incMAD,'
    Write-Host '     and the Wooden Wand incMAD is 27. Without it the whole formula'
    Write-Host '     runs on floor(INT/2) alone - which is how Magic Claw drew a 1.'
    Write-Host '       !resetap        then put points into INT until it reads 35+'
    Write-Host '       !job 200'
    Write-Host '       !kit            Wooden Wand 1372000. Needs level 10 and INT 20'
    Write-Host '       !learn'
    Write-Host '     EQUIP THE WAND.'
    Write-Host '       a) does Magic Claw now do REAL damage? Say roughly what.'
    Write-Host '          It should be far above the 1 you saw'
    Write-Host '       b) Magic Claw is TWO hits per cast. Do you see two numbers'
    Write-Host '          per swing?'
    Write-Host '       c) MAGIC GUARD: cast it, then get hit. Does the damage come'
    Write-Host '          off MP instead of HP? That split is the SERVER arithmetic'
    Write-Host '       d) does the wand actually go in the hand, or does !kit warn?'
    Write-Host ''
    Write-Host '  6. THE PURCHASE - CARRIED OVER, STILL UNCONFIRMED.' -ForegroundColor White
    Write-Host '     (If leaf points are under 1000, !lp 99000 first. That is a'
    Write-Host '     prerequisite, not a test - it has been confirmed several times.)'
    Write-Host '     Last run the coupon bought fine and landed in the ITEM inventory.'
    Write-Host '     0x19 is the reply to "move a locker item into a bag", so it did'
    Write-Host '     what it says. The purchase reply is 0x05AE sub-op 0x0C.'
    Write-Host '     Buy the MYSTERY HAIR COUPON - Main tab, 100 LP, SN 150000000.'
    Write-Host '     NOT Brown Puppy (a pet, refused) and NOT Red Hat (pet EQUIPMENT).'
    Write-Host '       a) does it appear in the CASH INVENTORY - upper left panel?'
    Write-Host '       b) success message and sound?'
    Write-Host '       c) do the Leaf Points drop by exactly 100?'
    Write-Host '       all three -> the cash shop is CLOSED as a feature'
    Write-Host '       BALANCE KEEPS DROPPING, or several coupons from one click ->'
    Write-Host '                   CLOSE THE CLIENT and say so. One line to disarm'
    Write-Host ''
    Write-Host '  7. BUYING FROM AN NPC SHOP. The only half never tested.' -ForegroundColor White
    Write-Host '     Selling is done and the counter draws. Buying never has been.'
    Write-Host '       a) buy something from Lucy. Do the ITEM and the MESOS both move?'
    Write-Host '       b) then click Buy AGAIN.'
    Write-Host '            it works twice     -> buying is done'
    Write-Host '            the second click does nothing, silently -> the result never'
    Write-Host '                          went out and the window is latched. Close and'
    Write-Host '                          re-click the NPC; a fresh 0x055D will NOT fix it'
    Write-Host '       c) do the prices match data/shops.txt?'
    Write-Host ''
    Write-Host '  8. THE !learn CRASH - ONE OBSERVATION, DO IT AT ~40 SECONDS.' -ForegroundColor White
    Write-Host '     !learn 1000001 15 died last run, but 3.7 SECONDS after the'
    Write-Host '     packet went out - not on it. The packet itself is byte-correct:'
    Write-Host '     skillId 1000001, level 15, masterLevel 0, and the never-expires'
    Write-Host '     constant exactly right. Fault 0x1415f0db0 appears in none of the'
    Write-Host '     other archived hook logs.'
    Write-Host '     So it is the usual pair: the COMMAND is fatal, or the session had'
    Write-Host '     been running long enough for something else to fire. Those need'
    Write-Host '     opposite work and look identical on screen.'
    Write-Host '     Log in and type it IMMEDIATELY, before anything else.'
    Write-Host '       it dies again, early -> the command is fatal. Repeatable'
    Write-Host '       it does NOT die      -> the session was long; it is innocent'
    Write-Host '     Say WHEN either way. This same discriminator cleared GoodTest in'
    Write-Host '     one login and convicted nothing since, so neither is the default.'
    Write-Host ''
    Write-Host '  9. THE KEYBOARD LAYOUT - 20 SECONDS, AND IT PICKS AN ARCHITECTURE.' -ForegroundColor White
    Write-Host '     You are right that it is not saved. It is not a broken feature:'
    Write-Host '     nothing in the seven crates has EVER touched a key mapping, and'
    Write-Host '     the client has nowhere local to keep one - I read its settings'
    Write-Host '     block out of the exe and it is sound, graphics and chat options'
    Write-Host '     with no key mapping in it. So it can only come from us.'
    Write-Host '     What is NOT known is whether the client TELLS us when a key'
    Write-Host '     changes. 162 archived captures have no keymap-shaped packet - but'
    Write-Host '     nobody has ever changed a key DURING a capture, so that is'
    Write-Host '     "never captured", not "never sent". This step is the difference.'
    Write-Host '       OPEN KEYBOARD SETTINGS, DRAG ONE SKILL ONTO AN EMPTY KEY,' -ForegroundColor Magenta
    Write-Host '       CLOSE THE WINDOW. Say roughly when you did it.' -ForegroundColor Magenta
    Write-Host '     I read the answer out of world.log - no second launch:'
    Write-Host '       a NEW opcode near that moment -> the client reports changes, its'
    Write-Host '                body IS the layout, and both halves become measured.'
    Write-Host '                That is the server-side implementation and the right one'
    Write-Host '       nothing new -> the client never volunteers it, and the only way'
    Write-Host '                left is the hook reading the client''s own memory'
    Write-Host '     The client does NOT freeze when you change a key - you have done'
    Write-Host '     it several times - so whatever it sends needs no reply and our'
    Write-Host '     silence today is already safe. One less thing to establish.'
    Write-Host ''
    Write-Host '  NOT THIS RUN - built but deliberately not wired:' -ForegroundColor DarkGray
    Write-Host '     Damage VALIDATION on attack skills. MP cost is wired now; the'
    Write-Host '     hit check is still not - the client authors the number and we'
    Write-Host '     do not argue with it.'
    Write-Host '     Spent skill points still come back; !learn grants directly.'
    Write-Host '     The Shop2 window (0x0560). Its art is not in this client and it'
    Write-Host '     can no longer be sent at all. --shop is now a no-op that says so.'
    Write-Host ''
    Write-Host '  GLANCES: drops arc from the corpse and are walkable-over; kill-EXP'
    Write-Host '  line is WHITE; mobs on map 40 already standing; pick-ups stay OUT of'
    Write-Host '  the chat log; level-up +16 HP / +12 MP; relog keeps Etc and mesos;'
    Write-Host '  ores stack; !setrates 2 3 5 -> one banner. NPC chatter no longer'
    Write-Host '  follows you into the cash shop.'
    Write-Host ''
    Write-Host '  MORE THAN ONE ACCOUNT WORKS NOW - relaunch with -Launcher.' -ForegroundColor Cyan
    Write-Host '  The login server used to resolve --account ONCE at startup, so one'
    Write-Host '  process could only ever be one player. It resolves per CONNECTION now:'
    Write-Host '  maplecw-launcher checks a password, marks who is playing, and you can'
    Write-Host '  swap accounts without restarting anything. Sign in with the account'
    Write-Host '  name OR its email.'
    Write-Host '    add one:  .\target\release\maplecw-useradd.exe <name> --email <addr>'
    Write-Host '    list:     .\target\release\maplecw-useradd.exe --list'
    Write-Host '  THIS run cleared any leftover claim, so it is served as --account.'
    Write-Host '  Still not authentication: the game socket carries no credentials, so'
    Write-Host '  this picks an account, it does not verify the client.'
    Write-Host ''
    Write-Host '  COMMANDS: !map !item !exp !heal !job !learn !kit !buff !unbuff'
    Write-Host '  !npcecho !npcfx !migsweep !exprate !mesorate !droprate !setrates'
    Write-Host '  !rates !nx !lp !buy !locker !resetap !resetsp.'
    Write-Host '  gm-handbook/equips.txt NOW HAS NAMES - and reqLevel, reqSTR, reqDEX,'
    Write-Host '  reqINT, reqLUK and reqJob. 1759 rows, name is the LAST column. That is'
    Write-Host '  the file to read when picking something to !item in.'
    Write-Host '  !learn and !kit are NEW and do this run setup for you. !lp grants'
    Write-Host '  LEAF POINTS and is the one that BUYS; !nx buys nothing. !help'
    Write-Host '  lists them all.'
} else {
    # THIS BRANCH IS A TRAP UNLESS IT SAYS SO. Without -SetFieldProbe the LOGIN server is
    # fine - character list, create, delete all work - but the CHANNEL answers nothing at
    # all, so picking a character hangs on "Connecting...". That looked like a server bug
    # for a whole launch on 2026-08-20. The steps below are a real run; they are just not
    # THIS run, and today's plan lives entirely in the other branch.
    Write-Host '  NO -SetFieldProbe, SO THE WORLD IS OFF.' -ForegroundColor Red
    Write-Host '  Login, character list, create and delete all work. But the CHANNEL' -ForegroundColor Red
    Write-Host '  answers NOTHING - Session::handle returns empty for every packet -' -ForegroundColor Red
    Write-Host '  so picking a character will hang on "Connecting...". That is this' -ForegroundColor Red
    Write-Host '  flag, not a bug. The cash shop plan is NOT printed on this branch.' -ForegroundColor Red
    Write-Host '  Relaunch with -SetFieldProbe to get into the world.' -ForegroundColor Red
    Write-Host ''
    Write-Host '  1. click Login. Any character created in an EARLIER run should be there.'
    Write-Host '  2. create one. Check the name first - a name already used is now refused'
    Write-Host '     by the server rather than always accepted.'
    Write-Host '  3. close the client, run this script again, and click Login. The character'
    Write-Host '     should still be listed. That is the whole point of this run.'
}
Write-Host ''
# WHAT THIS PROJECT COSTS THE MACHINE, said at the moment the owner can act on it.
#
# The owner, 2026-08-22: "my computer has been getting pretty slow with all of these tests."
# The crash dumps are FULL MEMORY - 1.0 to 1.4 GB each - and every crash keeps one. Nothing
# deletes them, deliberately: CLAUDE.md's rule is that a run's output is the most expensive
# data this project produces. So this reports the number instead of acting on it.
$dumpBytes = 0
$dumpCount = 0
try {
    $dmp = Get-ChildItem -Path $dumpDir -Filter *.dmp -ErrorAction Stop
    $dumpCount = @($dmp).Count
    $dumpBytes = ($dmp | Measure-Object -Property Length -Sum).Sum
} catch { }
if ($dumpCount -gt 0) {
    $free = (Get-PSDrive -Name ($root.Substring(0, 1)) -ErrorAction SilentlyContinue).Free
    Write-Host ("Disk: {0} crash dump(s) in dumps\ using {1:N1} GB{2}" -f `
        $dumpCount, ($dumpBytes / 1GB), $(if ($free) { ", {0:N0} GB free" -f ($free / 1GB) } else { '' })) -ForegroundColor DarkGray
    if ($dumpCount -ge 3) {
        Write-Host '      Each new crash adds another ~1.3 GB. research/heap-wild-write.md says' -ForegroundColor DarkGray
        Write-Host '      the first two are exhausted; deleting one is your call, not mine.' -ForegroundColor DarkGray
    }
}
Write-Host 'If the machine is slow and Task Manager blames System, name the driver:' -ForegroundColor DarkGray
Write-Host '  powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\system-cpu.ps1"' -ForegroundColor DarkGray
Write-Host ''
Write-Host 'Logs:'
Write-Host "  $serverLog                 every packet both ways, and what each reply was"
Write-Host "  $hookLog   client patches and faults"
Write-Host "  $exitLog          how the client died; 0 is a hand-close"
Write-Host ''
Write-Host 'Cash Shop regression - N clicks should be N lines:' -ForegroundColor DarkGray
Write-Host ('  powershell -NoProfile -Command "(Select-String -Path ''' + $serverLog + ''' -Pattern ''<- 0x00D5'').Count"') -ForegroundColor DarkGray
Write-Host 'And the latch the client read on each click:' -ForegroundColor DarkGray
Write-Host ('  powershell -NoProfile -Command "Select-String -Path ''' + $hookLog + ''' -Pattern ''WATCH #\d+: 0x142caee70'' | ForEach-Object { $_.Line }"') -ForegroundColor DarkGray
Write-Host ''
Write-Host "Then: powershell -ExecutionPolicy Bypass -File `"$PSCommandPath`" -Stop"
