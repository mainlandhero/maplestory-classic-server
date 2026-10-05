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
    ====== THE TEST PLAN, 2026-09-01: T0 DECIDES WHAT THE REST OF THIS RUN IS ======

    TWO copies in this file - this one and the Write-Host block near the bottom that
    actually gets printed. Update both, then RENDER the second one and read it.

    ###################################################################################
    ## THIS RUN IS ONE THING: !scroll. 2026-09-09.
    ###################################################################################

    Do NOT re-test Set Up chairs, map chairs, meso dropping, Shanks, !tool or the
    12-hour login. All six are confirmed on a screen and re-testing them spends the
    launch on answers we already have.

      MESOS      - ANSWERED 2026-09-09. The owner: "meso dropping is fine now, inventory is
                   fine as well after meso dropping." M0-M5 are struck.
      MAP CHAIRS - ANSWERED 2026-09-09. The owner: "I tested the map chairs with two clients,
                   that's all working now." 0x0252 is on the wire and works. C1-C4 struck.

    !scroll was rebuilt from the ground up after the last run and has FIVE open questions,
    every one of them a client question the server cannot answer for itself. It is step TS
    below. Do TS(a) before you !item anything.

    IF THE CLIENT DIES, SAY WHICH STEP YOU WERE ON. That one sentence is the whole
    difference between "a stack over slotMax is a client-killer" and "the session ended".


    AFTERWARDS: world-ch0.log is the evidence. Say what you saw per step; I will read it.

    HOW A RUN STARTS, since 2026-09-05: the servers come up and THE LAUNCHER OPENS. Sign
    in there, press Start Game. Nothing is served to a client that did not come through a
    sign-in - the login server answers it "not a registered ID". The old default, which
    opened MapleStory.exe directly and served it as maplecw with nobody signed in, is
    -DirectClient -FallbackAccount maplecw, and it is for arming hook watches only.

    ORDER: THE SINGLE-CLIENT HALF. The multiplayer half is BLOCKED, not untested.
    -----------------------------------------------------------------------------
    T0 is ANSWERED: this machine runs two clients. It took six launches and the
    answer is yes - grap-stub suppresses FindWindowA("MapleStoryClass") and redirects
    kernel32's CreateMutex forwarder slot so the second client survives
    Global\WvsClientMtx.

    T1/T2/T2b are BLOCKED on a client crash that is being debugged from crash dumps,
    not from launches. 0x0224 UserEnterField used to kill both clients inside its
    handler; that is fixed and confirmed on the wire. What remains is a fault at
    0x140f9295e on the update tick 158 ms later. DO NOT spend a launch putting two
    clients on one map until this file says the fault is fixed - the answer is already
    known (they die) and the next reading has to come from a change, not a repeat.

    So this run is T11, T10 and the rest, all single-client, in that order.

    -SetFieldProbe is no longer needed and is ignored - the channel answers by default
    since 2026-09-14. Old launch lines that carry it still work. The off case is
    -SilentChannel, and you have to ask for it. Run -Stop before relaunching.

    DO NOT PASS -HeapFix. It armed, it held, and it was irrelevant - 10 deaths of that
    family across the archive and FIFTY-SIX sites carry the same ladder.

    WHAT PREVIOUS RUNS CLOSED - none of this needs testing again
    -----------------------------------------------------------
    THE LAUNCHER WORKS: UAC prompt present, wrong password refused, client starts with no
    second prompt. A SECOND ACCOUNT WORKS and signs in BY EMAIL. THE MASKED EMAIL FOLLOWS
    THE ACCOUNT. ONE DAMAGE NUMBER. MP COST ACCURATE, selling works, no Buy Back tab,
    warrior skills cast, the classic shop draws.

    AND **THE TYPE-6 MENU RENDERS FROM THE SERVER** - the old T3 and T4, now answered and
    struck off. `research/fixtures/type6-menu-renders-and-taxi-rides-world.log` has Lyn and
    the Regular Cab each sending one box, each answered by a 10-byte 0x00F3 ending
    `06 01` with a real selection - line 2 then line 0 - and each followed by the fare and
    the SetField. Two NPCs, two different lines picked, one session. Nothing about menus
    needs re-testing, and T10 below rests on it rather than gambling on it.

    WHAT CHANGED IN THE SERVER SINCE THE LAST RUN
    ---------------------------------------------
    THE SESSION BUG IS FIXED AND AN ACCOUNT CAN ONLY BE LOGGED IN ONCE (2026-09-08).
    Start Game twice from one sign-in used to say the session was invalid on the second
    press - the launcher reuses its token and the server spent it on first use. The token
    is now honoured until the claim expires. The second client is instead told "That ID is
    already logged in", which is the client's own baked notice on login result 7. NOBODY
    HAS SEEN THAT DIALOG YET: the wording is read out of the WZ and the code out of
    FUN_141b267c0, both static. Step 0A below is what makes it measured.
    The "logged in" state is a LEASE on a live connection, not a flag: a crashed client
    frees its account at once, and at worst 60 s later. If a crash ever locks you out for
    longer, STOP and report it - that is worse than the bug this fixed.

    TWO PLAYERS CAN NOW SIGN IN IN ANY ORDER. The old instruction here said to wait until
    the first client reached the world, because the login server served whichever account
    claimed LAST. That was a real bug - one player was served the other's character list -
    and it is fixed: one claim per launch, and the server matches a connection to its
    launch by asking the OS which process owns the socket. Proved over real sockets by
    tools/claims_smoke.py, two accounts on 127.0.0.1, each served its own characters.
    SO: SIGN IN IN WHATEVER ORDER YOU LIKE. If both clients still show the same account,
    that is a NEW finding and worth saying.

    TWO CLIENTS RUN. Answered 2026-09-03 - this block said "STILL UNKNOWN" four lines
    under its own "T0 IS ANSWERED", which is the two-copies drift CLAUDE.md is about,
    inside ONE copy. Launch them one at a time: maplecw-hook.identity is a single shared
    file and overlapping launches give both clients the same credential.

    ============ WHAT THIS RUN IS FOR ============

    T14 (2026-09-05, revised the same evening). CHAT AND PARTY INVITE. The evening run: the
    invite went out and BOTH CLIENTS DIED. Two things were wrong, both now fixed and both
    unit-tested against the client's own reader. (1) 0x13 "joined" went out as the joiner's
    name alone; the client reads a six-seat PARTYBLOCK after the name, ran off the end, threw,
    reported the packet back in 0x009E and closed its socket - both clients, the same stacks.
    (2) The first 0x0183 ever decoded arrived 1 ms after the 0x03 with answer 0: that is the
    client's own "dialog opening" acknowledgement, sent by its 0x03 handler before any click,
    and the server took it for an accept. The answer byte is the 0x1B outcome numbering, read
    off the client's code: 0 received, 1 blocking, 2 busy, 3 already invited (all sent by the
    handler, no dialog), 4 the Decline button, 5 the Accept button (4/5 [D], the rest [L]).
    Fixture: research/fixtures/party-join-0x13-rejected-by-client-0x009E-*.
    This run: CHAT - a line typed on one client draws the balloon and the log line on the
    OTHER. PARTY - leader Create, Invite by name; leader reads "You have invited"; the TARGET
    gets a dialog that STAYS until clicked (say what it shows: name? level? job?), and nobody
    joins before a click - a party window already listing the target means answer 0 is still
    being acted on. ACCEPT - both read "has joined the party" and both party windows list
    both members; the block is what draws them, so a client dying HERE means the block is
    wrong: stop and keep the logs. DECLINE on a fresh invite - leader reads "has denied the
    party request", nobody joins. TIMEOUT - invite, leave the dialog alone about a minute,
    invite the same character again: it must go through; world-ch0.log prints "LAPSED".

    T14b (2026-09-05, from the owner's screenshot run - the party draws both members correctly).
    Six party mechanics, all server-side and unit-tested, none yet on a screen. LEAVE: the
    member can now actually leave (was "unknown error", stuck). PICK-UP RIGHTS: the leader's
    button no longer errors (stored server-side; the value gates nothing beyond membership,
    and this client has no standalone rights-changed packet). PARTY EXP: on the same map, one
    kills a mob - the killer sees a white line, every other member on the field a yellow line
    (a COPY of the party share each - 30% by default, the 5th !setrates field, since
    2026-09-06; no AFK signal exists, so on-field-and-online is the test).
    QUEST: a kill credits every party member on the field who has that kill-quest in progress.
    DROPS: a party mob's drops show to all members on the field and any of them may take them;
    a member who then LEAVES loses access while the killer keeps it. GROUND DROP: an item a
    player drops from their bag is broadcast to the whole map and anyone may pick it up
    (untradeable excepted). EXPIRY (2026-09-18): a drop that reaches its 120 s lifetime now
    fades on EVERY screen that was shown it - the party members and, for a ground drop, every
    bystander - not only the owner's. The owner saw the others keep drawing an expired party drop.
    Kill one mob in a party and drop one item from the bag, then leave both alone for two
    minutes: both vanish from BOTH screens at the same moment -> fixed; still drawn on the
    other screen -> paste that client's world-ch0.log lines for DropLeaveField at that time.
    PARTY HP (built 2026-09-06): the packet is 0x02B2, found by
    walking back from the HUD gauge to the field it reads, to that field's one writer, to the
    handler, to a compacted third switch in the remote-user router; every link [L]. The other
    member's bar in the top-right HUD should fill within a second of the party forming and
    follow their HP under damage and potions; say whether the small bar over their head moves
    too (it is fed from the same packet, a second field). A blank bar is the finding. One
    thing is NOT built because its opcode has never been captured, and guessing a packet body
    has killed this client three times: DROPPING MESOS. Try it and read the inbound opcode off
    world-ch0.log (or report that none appears) - that one measurement is all it needs.

    T15 (2026-09-06). ARCHER AUDIT. Arrows were never taken - the rule sat in firstjob.rs since
    08-28 and on_attack never read it. Now: with a bow or crossbow worn, a normal shot takes 1
    arrow from the lowest matching Use-tab stack (2060xxx for a bow, 2061xxx for a crossbow),
    Arrow Blow 1, Double Shot 2 (their bulletConsume column, [L]), Power Knockback 0 (no bullet
    column; the bow is swung). The stack size comes back as a 0x0070, mode 1, or mode 3 when it
    empties. A shortfall is logged and the swing is NEVER refused. Read the count off the Use
    tab. Power Knockback's push is produced by the CLIENT from the skill's own `range` (130 px
    at level 1, 150 at 15 - the tooltip's "knockback N enemies by 130") and, like every hit
    reaction, only on the screen of the client that controls the mob; the server hands control
    to whoever hits, so on a mob the other client was driving the first hit transfers it and
    the second pushes. Say how far it flies next to a normal hit and whether the other client's
    screen shows the same landing spot. MP for all three is spent server-side already.

    T16 (2026-09-06). THIEF AND WARRIOR AUDIT. STARS: with a claw worn and stars in the Use tab,
    a normal throw takes 1 star, LUCKY SEVEN takes 2 (one per projectile - the data has no
    bulletConsume for it, so this is the owner's attack-amount rule, [I]), Double Stab with a dagger
    takes 0. Every 207xxxx star counts; the lowest matching stack drains first (the attack
    packet carries no slot). If a throw takes NOTHING and world-ch0.log says "a 0x00E0 SHOOT body
    did not parse", that is the finding: no shoot body has ever been captured and the parser
    comes from melee. RECHARGE: at any Grocer (Lucy, Mina, Luna...) select a partial star stack
    in your inventory and press Recharge. Expected: the stack fills to slotMax (Subi 500) and
    the meso count drops by ceil(missing x unitPrice), Subi 0.3 per star; write down the number
    the window showed next to "Recharge:" and the number the mesos moved by - both are
    ceil(missing x unitPrice) by the listing (2026-10-03); a LOWER label is the client's own
    discount (FUN_141fb9f30, untraced). EVERY STAR at a general store (2026-10-02): a Grocer, a
    town General Store or the Mobile Store also lists each star it does not sell as a price-0
    recharge-only row, so a dropped Wolbi recharges too (0.4 per star). The Buy tab still shows
    ONLY the shelf - a Wolbi/Ilbi/... at 0 mesos there means price 0 does NOT hide a row. The
    button itself is plan step 39f. SLASH
    BLAST now costs HP (3 at level 1) as well as MP; Power Strike does not. MAX HP INCREASE:
    The owner's 358/447 screenshot settled experiment A - the CLIENT adds the percent on top of the
    server's max, and the server was calling 358 full. Every ceiling the server enforces is now
    base plus the percent (session::pools): idle regen must climb past 358 to 447, a potion
    drunk above 358 must NOT cut HP back to 358, a level-up refills to 447, and the party bar
    shows 447. Max MP Increase is the same code and is INFERRED - a Magician's MP number under
    the same three checks is the measurement. Also: no "A skill has been activated." line on
    a skill-up (0x0081 byte 1 is off, as in the real game). PARTY BUFFS: Haste
    (Assassin/Bandit) and Rage (Fighter) reach every party member ON THE SAME MAP; a member on
    another map and a non-member beside you get nothing. A character must actually have the
    skill (!job 410 then !learn on an Assassin, or !job 110 for Rage). What to watch: the buff icon
    on BOTH screens, the recipient's walk AND jump for Haste (jump is bit 93, [D] - a faster
    walk with the same jump means the pair is off), Rage's number on the stat window's attack
    line (bit 84, [D] - icon without the number means 83 is next). Iron Will is SELF-ONLY in
    this client's data (no lt/rb rectangle) - report it as expected, not as a bug.

    T17 (2026-09-07). DONE, AND IT FOUND A CLOCK. The sentry run caught the write FOUR times
    in one idle session and the intervals are the finding: +180.002 s, +180.115 s, +180.020 s.
    With the previous evening's pair (+180.038 s) that is SIX catches, FOUR intervals, every
    one 180.0 s to within 0.12 s. research/the-180-second-clock-2026-09-07.md.

    THE WRITER IS ON A TIMER, NOT ON TRAFFIC. No packet, no exception (2 C++ throws all
    session, both in the first 1.4 s), no socket event, every thread parked. And it RETRACTS
    the census lead this plan carried: the 0x013D census is on a 30 s grid, 180 is a multiple
    of it, so a census lands beside every catch and five out of six censuses produce nothing.
    Two grids sharing a wall clock, not cause.

    THE MITIGATION HOLDS - 26 minutes, 7 catches, 7 repairs, 0 refusals, no death, closed by
    hand; the same idle session the night before died at 23 minutes.

    TO PLAY, add -SentryQuiet. The owner: "it lags/freezes the client every time it runs." Measured
    off that run's own heartbeats rather than guessed: an ordinary walk costs 0.63-0.71 ms, a
    finding without a dump costs 49 ms (the 68-thread stack scan), and a finding WITH a dump
    costs 703-895 ms with the client frozen throughout. -SentryQuiet drops the dumps and the
    thread scan and walks every 2 s except within 5 s of a predicted firing, where it returns
    to 100 ms. The period is LEARNED from the first two catches - nothing is hard-coded - and a
    catch outside the predicted window drops it and goes back to fine. The repair stays on.
    Use -PoolSentry alone when the run is a MEASUREMENT rather than play.

    T19 (NEW 2026-09-07). SECOND AND THIRD JOB SKILLS - 149 audited, most built, NONE on a
    screen yet. research/second-third-job-audit-2026-09-07.md has a row per skill. Use !job
    and !learn; test ONE at a time and say which. Each line is a claim that can come back false:
      (a) SWORD BOOSTER: icon appears, swings speed up, AND BOTH HP and MP drop 30. Only MP
          moving = the hpCon path is broken. No icon = bit 96 wrong (it was a name-table
          positive control, so unlikely).
      (b) HYPER BODY: max HP on the stat window +10%, and regen keeps going PAST the old max.
          Icon but regen stops at the old max = the server ceiling is not reading the buff.
      (c) POWER GUARD: get hit. You lose 80% of the number, the mob's bar drops by 20% of it.
      (d) COMBO ATTACK: cast it and COUNT THE ORBS. Zero orbs = the value-1 convention is right;
          one orb = the value is the orb count and every number is one high. Each hit adds one
          up to 3; Coma clears them.
      (e) SOUL ARROW: the arrow count stops moving. STRAFE takes 3 per cast, ARROW RAIN 8.
      (f) HEAL at low HP: +40% of max and a blue number. Bless up: 41%.
      (g) MAGIC GUARD stays on. It used to expire on the NEXT LOOP PASS - a toggle's expiry
          was recorded as now+0 - so this is a regression check on a first-job skill.
      (h) TELEPORT / FLASH JUMP: the MP stays spent. Before, a potion or regen "refunded" it.
      (i) ELEMENT AMPLIFICATION on: Fire Arrow costs 16, not 14.
      (j) MESO GUARD: a hit costs mesos and less HP; with 0 mesos it costs full HP.
    NOT BUILT - if asked, say so rather than test it: summons/Puppet, Mystic Door, every mob
    status (slow, seal, stun, freeze, bleed/DoT - no mob-stat packet is decoded), Pickpocket,
    Meso Explosion, Meso Saver, Chakra, Critical/Nimble Recovery, Final Attack's HP absorb,
    Steal's theft. A cast of those still costs MP and does nothing else, on purpose.

    T18 (2026-09-07). NAME THE WRITER. RUN 1 DONE (19:40-20:50, 70 min): the store was NOT
    caught, and the run still moved three things. (1) THE WRITER RE-HITS SLOTS IT HAS HIT: three
    of twelve repaired headers read damaged again in the death dump, one of them 0x..02..20 -
    hit TWICE more. The sentry's "report once per address" rule suppressed every re-hit (21
    firings, 12 caught, 9 silent) and the re-hit on catch #12's slot went unrepaired; its free
    killed the client at 20:50:55. FIXED: a repaired slot is reported and repaired again.
    (2) THE ALLOCATION BEFORE EVERY CATCH IS NAMED BY CADENCE: 140ca61d0(out, 5) - 28 bytes,
    the 0x20 class - from 0x14491cafd, every 180 s, ~100 ms before each catch, 14 of 14. Not the
    n=6 caller (that one is on 240 s). 0x14491cafd is in the Themida region, which is why no
    listing reads it. (3) THE WATCH COVERED 10%: only 73 of ~700 pool pages lie entirely inside
    chunks, so every store landed on an unprotected page (208 body writes, 0 header writes;
    control PASS). FIXED: the pages of every header already caught are PINNED on every window.
    Given (1), the next window over a re-hit slot should catch the store.

    T21 (NEW 2026-09-08). FIVE LIVE FIXES, none of them ever on a screen. Test ONE at a time
    and say which. Each line is a claim that can come back false.
      (a) MESOS - THE IMPORTANT ONE. Drop 10 mesos, then IMMEDIATELY try to move an item in
          the bag.
            refuses in words AND the bag still works -> fixed.
            nothing happens and the bag is DEAD    -> the latch is still set and the 0x007C
                                                      is not the unlock. This is the bug: the
                                                      client latches when it SENDS, so silence
                                                      kills the inventory, the AP buttons and
                                                      the cash shop for the whole session.
          Then try the AP buttons and the cash shop in the same session - all three share the
          latch, so if the bag is dead they are too.
      (b) MOB DROPS - WALKING IS FIXED, JUMPING IS NOT. The owner, 2026-09-08 on a live client:
          "the drops so far are good ... particularly when a mob is jumping, the loot drops
          BELOW the current platform." So do not re-test a walking snail; that half is done.
          Kill a mob MID-JUMP and watch where the loot lands.
            it lands on the corpse, on the platform -> the jump half is fixed too.
            it lands BELOW the platform             -> still wrong, and the interesting
                                                       question is whether the y came out
                                                       mid-air or the foothold snap searched
                                                       down past the platform.
          Say WHICH platform it landed on relative to the mob - one below, or the ground.
      (c) QUEST ITEMS - CONFIRMED ON A SCREEN 2026-09-08. The owner played a live session with no
          Omok quest and no quest item dropped. Nothing to re-test unless it regresses; the
          remaining half nobody has watched is the POSITIVE case - take the Omok quest and
          confirm the piece starts dropping again. Dark Marbles are EXEMPT by design.
      (d) LEVEL UP - needs TWO clients on ONE map. Level one and watch the OTHER screen for the
          animation and sound. This cannot be confirmed from the server side: the client drops
          the packet in silence if that observer does not already hold the leveller's spawn.
      (e) !tool - CONFIRMED WORKING ON A SCREEN 2026-09-08 ("!tool seems to be working
          correctly"). The steps below are kept only for a regression; do not spend a run on
          them. The one thing still unwatched is the daily REFUSAL - run it twice in one day
          and confirm it refuses in words rather than going silent.
          Original steps: type it in chat on a NON-GM account. It is a public command.
            a box with the Maple Administrator's PORTRAIT and three numbered lines
                                                   -> the whole chain works. Pick Level up:
                                                      expect a level, an EXP line, +5 AP.
            three lines but NO PORTRAIT            -> the speaker template is not resolving.
                                                      This is the ONE thing the tests cannot
                                                      see, so look at the portrait on purpose.
            "is not a command"                     -> the dispatcher never reached it.
            nothing at all                         -> worse than a refusal; check world-ch0.log for
                                                      the 0x00E7 and whether anything went back.
          Then run !tool again the SAME session and pick the same option: it must REFUSE IN
          WORDS, never go silent, and the log must say NOTHING PAID. Reset is UTC midnight.
          Leaf Points are per ACCOUNT (a second character is refused); Level up and Henesys
          are per CHARACTER (a second character still gets its own).
      (f) THE MAPLE ADMINISTRATOR HERSELF - Henesys, far left near the portal. Click them.
          They must give their QUEST or their greeting and NEVER the favours menu. That is the
          point of (e) being a command: the client's click fork is keyed on their TEMPLATE, so
          a summoned copy of their would send bytes identical to clicking them.

      (g) CHAIRS - CONFIRMED ON TWO SCREENS 2026-09-09. Set Up chairs sit, stand, scale the
          idle tick, show the right model, and Tester2 sees Cobalt in them. Do NOT spend a
          run re-testing that half. Two things are still worth a look:
            - the Blue Seal Cushion (3010008) must add 10 MP and NO HP. If it adds 30 HP the
              chair table was defaulted somewhere. Never checked on a screen.
            - MAP CHAIRS (a Henesys bench): A FIX IS IN, BUILT TODAY, NEVER SEEN. Sit on a
              Henesys bench. The player should seat and STAY seated; press a movement key
              to get up again.
                seats           -> 0x0252 was the missing packet. Say so and we are done.
                nothing at all  -> check world-ch0.log. If 0x0252 went out, the suspect is the
                                   handler's own gate at 1428341d3, which returns before
                                   reading anything. It is NOT a body-length fault.
                you get up by yourself, or a 0x00DA ffff appears in the log
                                -> the client re-validated and REFUSED: it wants
                                   seatX-10 <= myX < seatX+10 and seatY-30 <= myY <
                                   seatY+30. That is a position problem, not a packet one,
                                   and it is a DIFFERENT outcome from "nothing happened".
              Why it never worked before: 0x02AD lives in a dispatcher that looks the target
              up in the remote hash only, and the local player is not in that hash - so it
              could never address the sitter, whatever body it carried. 0x0252 lives in the
              dispatcher that checks the local-user slot first.
          Chair recovery: sit, stand still 15 s, HP should climb by 40 per tick (10 flat plus
          the Red Chair's 30).

      (h) THE CHAIR RELAY IS LIVE AND IT KILLED A CLIENT ONCE. 0x02AD is sent on every sit,
          map-chair attempt and stand. On 2026-09-09 an earlier 12-byte version of it faulted
          Tester2's client 5 ms after it went out; the body is 13 bytes and a test pins that.
          If a second client EVER exits while somebody sits, that packet is the first suspect:
            world-ch0.log        -> 0x02AD ... 13 bytes
            hook log         CLIENT FAULT #1 code=0xc0000005
          Say so immediately and do not close the surviving client - tools\chairprobe.py
          --pid <n> reads its state, and with two clients up you must pass --pid because they
          share one hook log.

      (i) THE ANTI-CHEAT GATE, and it is TIME-CRITICAL. Run this at about 60 s of client
          life and again at about 150 s:
              cd "C:\MapleCW"; python tools\gatescan.py
          [0x143AC7F3C] flips from 0 to 2 somewhere between 38 s and 194 s of every session
          (38 dumps, clean split). Two readings inside that window bracket it. A flip at
          180 s is the client's own clock; a flip at 40 s is something else, and something
          else is where our stub could be implicated.

      (j) THE TRADE INVITE POPUP - BUILT 2026-09-09, NEVER SEEN ON A SCREEN. Two clients.
          Tester2 sends Cobalt a trade request. Cobalt must get a "Trade request from
          Tester2" popup; on 2026-09-09 nothing appeared at all, which is what this fixes.
            popup appears  -> the `type` field was the bug and it is fixed.
            no popup       -> check world-ch0.log for 0x0575 going out. If it went out, the
                              cause is field 3, the only guessed field in the packet: a HIT
                              on the client's local lookup makes it auto-decline SILENTLY,
                              which looks identical to today's symptom.
          ACCEPT DOES NOTHING, AND THAT IS EXPECTED - not a regression. The trade WINDOW is
          0x0575 mode 4, whose per-member body is dispatched through a virtual call on the
          open dialog and is undecoded. Accept and Decline are logged and answered with
          nothing rather than with a guess; a guessed body killed a client THIS SAME DAY.

      (k) SHANKS - THE PAID SAIL IS CONFIRMED (2026-09-09); THE FREE ONE WAS REBUILT
          2026-09-16. The owner: "the dialogue that they'll waive it because you have finished
          Mai's Final Training does not show. However, Shanks does correctly waive the fee
          and TP the players." The line was sent in the same batch as the SetField and
          field entry tore it down. Now Yes puts the waiver on ITS OWN box ("Hold on -
          you're the one who finished Mai's training...") and moves nobody; the boat sails
          when that box is dismissed (OK or Esc, either). And the waiver is for BEGINNERS
          ONLY: a Swordsman who finished Mai pays the 1000 like anyone else, no box.
            1. a Beginner who finished Mai's Final Training says Yes: the waiver box, and
               you are STILL in Southperry -> as designed. Dismiss it: Lith Harbor, mesos
               unchanged, no grey fare line -> fixed
            2. the same with a first-job character: no waiver box, straight to Lith Harbor,
               the grey "lost mesos (-1000)" line -> as designed
          And the way BACK, 2026-09-16: "Lyn in Lith Harbor ... should have an additional
          destination to allow travelers to go back to Maple Island Southperry for 20,000
          mesos. This should only exist at Lyn and not at other Taxis." Lyn's menu has a
          SIXTH line, "Southperry (Maple Island) - 20000 mesos", under the same header that
          still quotes the 500-meso tour; the five towns are lines 1-5 exactly as before.
            3. click Lyn: six lines, the last names Southperry and 20000 -> as designed.
               Pick it with 20,000+: Southperry, grey "lost mesos (-20000)" -> fixed.
               With less: their refusal quotes 20000 (not 500), nothing taken.
            4. the Lith Harbor VIP Cab and the Henesys Regular Cab: NO Southperry line.

      (l) THE FARE LINE MUST BE GREY, NOT RED. Any fare - Shanks or a taxi - prints a grey
          chat line "You have lost mesos (-1000)". A RED "You have received Meso Penalty"
          means the old path is still live. This is the same check for (k) and for any taxi.

      (m) DROPPING MESOS - NEW 2026-09-09, NEVER SEEN. The owner: "I still cannot drop mesos."
          Until today the server DECODED the request and then refused it; the 09-08 work
          only stopped the refusal freezing the inventory.
          Drop 10 mesos while standing still. Then:
            a bag of coins on the floor, meso counter down by 10, and you can pick it
            back up                          -> done.
            nothing on the floor, and a chat line saying why
                                             -> a refusal fired. The line says which:
                                                not enough / not a positive amount /
                                                walk a step first. That is a REFUSAL,
                                                not a freeze, and it is working as built.
            nothing at all and no chat line  -> that is the freeze coming back. Check
                                                world-ch0.log for 0x0143 and say so.
          THEN TRY THE INVENTORY IMMEDIATELY. The whole reason this opcode matters is that
          an unanswered one latches +0x2330 and kills the bag, the AP buttons and the cash
          shop for the rest of the session. Move an item after dropping: if the bag is dead,
          the reply is not clearing the latch.
          Drop your WHOLE balance too - that is allowed, and it is the boundary the tests
          pin. Anyone on the map can pick the coins up, not just you.

    T20 (2026-09-08, REWRITTEN AFTER THE 12:01 RUN). THE OVERNIGHT RUN - SURVIVE, do not
    measure. The owner: "our goal is to leave the client running overnight without it exiting."

      -PoolSentry -SentryQuiet -SentryRepair -GuardPage -PinPatches

    -GuardBucket now DEFAULTS to 0x20+0x40. Do not type it.

    AND AS OF THIS AFTERNOON THE LAUNCHER SHIPS IT ANYWAY: DEFAULT_SESSION is
    mode=2,create=on,guardpage=0x20+0x40, so a plain launcher launch is already quarantining
    both classes. -GuardPage still matters here because this script writes a session PIN that
    replaces that default. If the guard page ever needs to come off a machine, the switch is
    maplecw-hook.guardpage.off beside MapleStory.exe - and it wins over a pin on that one
    token, so DELETE IT before a measurement run or the marker read-back below will refuse
    the launch.

    AND NOTHING ELSE. NO -Probe WATCHES ON THIS RUN.
    2026-09-08 14:47: this command plus five watch@ targets was run, and the client closed
    the instant it entered the field - the first death in this whole investigation with NO
    exception and NO crash dump. The guard page armed clean, control PASSed, and BOTH classes'
    first-free controls fired, so the guard page is not the suspect; the log's LAST line is
    the watch firing. Two things were changed at once and that is why it is still a suspicion
    rather than a fact. Run the guard page ALONE. Add watches only after it has survived.

    IF YOU DO ADD WATCHES LATER, THE GRAMMAR IS ONE `watch@`:
      -Probe "watch@140c93530,140c936a0,140c93810,140c93b70,140c93930"   RIGHT
      -Probe "watch@140c93530,watch@140c936a0,..."                        WRONG - four of the
                                                                          five are refused
    The launcher now refuses the wrong form outright, so this cannot cost another run.

    SETTLED AT 12:01 TODAY, DO NOT RE-TEST. The guard page armed on a client for the first
    time ("GUARD PAGE ARMED: size class 0x20 ... control PASS"), and the client then ran
    12:01:18 -> 13:58:14, 1h57m, the longest session this project has had. An inline hook on
    the pool allocator, called from thirty threads thousands of times a second, does not
    destabilise the client. That question is closed and no run needs to re-open it.

    TWO THINGS IT DID NOT DO, and both are what this build changes.

    (1) IT RAN OUT OF RESERVE AT SIX MINUTES. From the heartbeats: 627172 slots served in the
    FIRST MINUTE (a startup burst, ~10000/s), then a steady 1560/s. The 1048576-slot cursor
    was spent at 06:00 - four minutes before anything could age out of the 10-minute
    retirement queue - and 419588 allocations fell back to the client's own pool before
    recycling began at 10:00. Steady state alone (1560 x 600 = 936000) would have FITTED; the
    burst is what broke it. FIXED by growing the cursor, not by shortening the window:
    8388608 slots, 32 GB of address space (which is nearly free), and the 40-byte-a-slot
    metadata array - 320 MB if committed up front, and the real reason the reserve could not
    grow - is now committed lazily as the cursor advances. REUSE_AFTER_MS stays at 600 s: it
    comes from the writer's 180 s clock (three firings), not from the reserve.

    (1b) 2026-09-16 16:46, THE SAME THING AGAIN AT 22x THE CHURN, AND THE WINDOW IS NOW THE
    LEVER. On D:\MapleCW the pair 0x20+0x40 spent the whole 8388608-slot reserve at ~254 s:
    0x20 ran at 34173/s (the 12:01 run it was sized from ran at 1560/s), 0x40 at 3396/s, and
    the two classes share ONE cursor - their served counters both stopped between the 240 s
    and 300 s heartbeats and sum to exactly 8388608. At 600 s nothing could age out before the
    reserve was gone, so from 4.5 min every allocation went back to the client's own pool,
    the sentry caught the known 0x0000000100000020 header at 6 min in a slot the guard no
    longer served, and the client died at 9 min of 0xC0000374. You: "Okay, let's recycle
    sooner." REUSE_AFTER_MS is 200 s (one full 180 s firing plus 20 s) and the reserve is
    16777216 slots (64 GB of address space, 64 MB ring at arm). The pair's 200 s window is
    7.5 M slots, 2.2x headroom, and the arming line now prints the pair's need from the 16:46
    numbers rather than one class's from the 12:01 ones. WHAT TO READ: the heartbeat's
    "recycled" counter must be NON-ZERO from the 260 s heartbeat on, and "FELL BACK" must
    stay absent. Recycled > 0 with FELL BACK absent past 10 min -> the window holds at this
    churn. FELL BACK present -> the churn is higher still; paste the heartbeat lines.
    A heap death with FELL BACK absent -> a write through a pointer older than 200 s, which
    is the margin this gave up, and the answer is the window back up with a bigger reserve.

    (2) IT DIED ON A CLASS WE WERE NOT QUARANTINING. The fatal object is a 0x40 slot whose
    vtable pointer was incremented by 2 (it reads 0x143406c02; 0x143406c00 is the genuine
    vtable). The writer holds a stale ADDRESS, not a class - whichever bucket's chunk is later
    carved over that address is the victim - so one class is whack-a-mole. The guard now
    quarantines a SET, and the default is 0x20+0x40, which contains all three deaths on
    record: 0x40 map node +2 (runs 2 and 5), 0x20 tree node set to -1 (the overnight run),
    0x40 vtable +2 (12:01). One shared reserve serves both, because the header stamp 0x100 is
    above every rung of all three of the client's free ladders and those ladders read the
    HEADER and nothing else - so one stamp covers every class at once.

    RUN FIVE MINUTES FIRST, then leave it overnight. TWO classes at once has never run.

    WHAT TO READ, in client-patched\maplecw-hook.log:
      "GUARD PAGE ARMED ... 0x20+0x40 ... control PASS"
                    -> armed on both. The line prints the sizing model, the measurement it
                       came from, and its headroom. "NOT enough headroom" inside it means a
                       fall-back is expected - say so rather than reporting a clean arm.
      "control FAIL" / prologue mismatch
                    -> it stood down and REVERTED; the client is unpatched by it. Report the
                       line, do not leave it overnight.
      "the FIRST free of class 0x40 came back through our HeapFree shim"
                    -> THE NEW CONTROL, expected within seconds. 0x40 has never been
                       quarantined, and 53 of the client's 56 free sites are inlined and
                       untraced, so "0x40 frees reach us" is a GUESS until this line appears.
      "***** NEVER FREED ... does NOT reach our shim *****"
                    -> that control did not come and the class is leaking a page per
                       allocation. Stop the run and report it.
      "***** N FELL BACK - NO LONGER COVERED *****"
                    -> should now be ZERO for the whole night. Any number, and say WHICH
                       CLASS: the heartbeat prints the classes apart on purpose, because a
                       total hides which one is exhausting the shared cursor.
      "pool allocations seen by class: 0x10 N, 0x20 N, 0x40 N, 0x80 N"
                    -> THE MEASUREMENT THIS RUN MAKES EVEN IF IT DIES. Only 0x20 has ever been
                       measured (1560/s after a 627172 burst); these four numbers decide
                       whether all four classes can be quarantined at once (-GuardBucket all).
                       Paste them whatever happens.
      "GUARD PAGE - STALE WRITE at X ... RIP R ... allocated from A freed from F"
                    -> THE ANSWER: the writer, named AND neutralised. It now also says how
                       many ms ago the slot was allocated and freed, which tests the 180 s
                       clock directly. Several of these with the client still up is the run
                       working, not failing.
      "SENTRY REPAIR" lines - the header half doing its job.

    COST: ~33 MB at arm (32 MB retirement ring + the first metadata block), plus ~100 MB of
    committed pages per class - MEASURED, not predicted: the 12:01 run held ~26000 live 0x20
    slots all afternoon, against the ~230 MB predicted beforehand. Retired pages are
    decommitted and cost only address space. Memory is not the constraint; the cursor was.

    NO -SentryWriteWatch. The write watch only OBSERVES: read-only pages and single-stepped
    writes, ~160 windows in eight hours, and it cannot prevent anything. -SentryQuiet is the
    long-run cadence (no dumps, no 68-thread stack scan, 2 s walk except near a firing) and it
    KEEPS the repair.

    A CLEAN POOL IS NOT SUCCESS. The 01:33 run died with 0 damaged headers in 174528 slots:
    the writer damages LIVE objects, and the sentry only ever checks free headers. Every
    "pool clean, N repaired" line is true and says nothing about whether the client will live.

    IF THE CLIENT DIES INSIDE FIVE MINUTES: relaunch with -GuardBucket 0x20, the configuration
    that already survived 1h57m, and say which of the two it was. Change ONE thing.

    UNKNOWN, said plainly: the longest run is 1h57m, so eight hours is still an extrapolation.
    Multi-class serving, the lazy metadata commit, the reserve ladder and the first-free
    control have never run on a client.

    RUN 5 (2026-09-08 build). NAME THE WRITER ON BOTH SURFACES IN ONE LAUNCH. The write watch
    (0x20 headers, windows) and the GUARD PAGE (0x40, quarantine) are complementary and run
    together. Run 5 died at 3.5 min on a 0x40 map node used +2 (heap-wild-write dump 2) - a
    surface the window watch structurally cannot reach. The guard page covers it: it serves the
    0x40 class one-slot-per-page and decommits on free, so a stale pointer into a freed 0x40
    slot faults at the writer on any clock. Recipe now:

      -ServersOnly -PoolSentry -SentryRepair -SentryWriteWatch -GuardPage
      -PinPatches -Probe "watch@1415db360:ret,141b2a280:rdx=0,140ca61d0:hits=400"

    In the hook log: the sentry heartbeat gains a "guard page: N served, M freed, K live, C
    STALE-ACCESS CATCH(es)" line; on a catch, "GUARD PAGE - STALE WRITE at X ... RIP R ...
    allocated from A freed from F" - R is the 0x40 writer. "GUARD PAGE ARMED ... control PASS"
    confirms it armed; "control FAIL" or a prologue-mismatch line means it stood down and the
    client is unpatched by it. The guard writes to the client (an allocator inline hook + a
    HeapFree pointer swap). -GuardBucket picks the classes here (default 0x20+0x40).
    2026-09-08: THE LAUNCHER NOW SHIPS IT. maplecw-launcher's DEFAULT_SESSION carries
    guardpage=0x20+0x40, so an ordinary Start Game arms it with no flag at all; -GuardPage
    only matters on this script's pinned/direct path. To turn it off on any machine without a
    rebuild: create maplecw-hook.guardpage.off beside MapleStory.exe, or set
    guardpage = "off" in maplecw-launcher.toml. Absent means ON.

    RUN 4 (22:45-23:35, closed by hand): 11 windows, one every cycle; pinned pages grew to 8;
    9 catches in 15 firings (six stores landed where the sentry cannot see, as the run-2 death
    predicted). And the decisive pair: catches #3 and #4 were the SAME slot, its page PINNED,
    and #4's store landed INSIDE window #3 on that protected page - uncaught. The first write
    to a page opened it until the next 5 ms sweep, and a body write on the same page held the
    door. FIXED: the page is opened for ONE instruction (trap flag, then re-protect on the
    step - probe.rs's own int3 trick), so EVERY write faults. Tested with two writes to one
    page. AGAIN, same recipe.
    RUN 3 (21:57-22:39, closed by hand): 12 firings, 12 catches, none silent - the re-hit fix
    works, and a THIRD of firings land on an address already hit (0x696fde0 three times). Pool
    clean at close. But ONE window opened in 42 minutes: the re-arm guard compared the firing
    index alone, and every catch re-anchors the clock to index 1 again. FIXED (window_due,
    tested). AGAIN, same recipe; the first re-hit has come at catch #3 both times, so expect a
    pinned-page window with a re-hit under it about ten minutes in. The probe cap is 400:

      -ServersOnly -PoolSentry -SentryRepair -SentryWriteWatch -PinPatches
      -Probe "watch@1415db360:ret,141b2a280:rdx=0,140ca61d0:hits=400"

    (A) -SentryRepair writes a confirmed damaged header back to the slot size, so the pool's
    free recognises the slot as the 0x20 slot it still is and puts it on its own list instead
    of handing it to HeapFree. That removes the 0xC0000374 death. OFF by default because it is
    the only client byte this hook writes. Three things it cannot do, all in repair_header's
    doc block: it is a RACE (a write and a free inside one 100 ms walk still dies), it only
    sees damage shaped like a pool header (the 0xC0000005 death of 2026-09-06 was a -1 in a
    map-node pointer), and if that dword is a refcount reached through a stale pointer then
    zeroing it changes what the writer sees next. Expect FOUR repairs; the heartbeat counts
    them. Say the flag was on in any result that depends on the client having stayed alive.

    (B) RUN 1 SETTLED THIS BY CADENCE: the rdx=5 caller 0x14491cafd (28 bytes, the 0x20
    class) fires every 180 s, ~100 ms before each catch, 14 of 14; rdx=6 is on 240 s and is
    not it. The paragraph below is the static reading it replaced, kept for its working.
    The 180 s clock is a FAMILY OF FIFTEEN near-identical tickers at 0x140c93530..95095,
    seven of them called straight from the frame tick 0x142ce0130. Each seeds a timestamp on
    its first call and RE-ARMS IT ON THE FIRING BRANCH, which is what makes the period exact
    rather than drifting - verified in the listing, not taken on trust. Six of them call
    FUN_140ca61d0(out, n), which allocates n*4+8 bytes from the pool and frees it again; two
    pass n=6, i.e. EXACTLY 32 bytes, the only size class that has ever been damaged. Watching
    140ca61d0 costs one probe slot and says whether that family runs on the catch clock.

    -PinPatches is what makes (B) possible at all. The LAUNCHER writes the probe and session
    markers with its own defaults on every launch, and it is the only path that reaches the
    world, so its patch set was in practice the only one the client could ever run. A pin
    overrides it for ONE launch; the launcher deletes it on read and prints OVERRIDES.

    (C) -SentryWriteWatch IS THE POINT OF THIS RUN. Every instrument before it could say
    WHEN. This makes the store fault at the instruction that makes it. For ~1.2 s around each
    PREDICTED firing, bucket 1's pages go PAGE_READONLY. Reads are untouched, so the walk and
    the client's own strings carry on; a WRITE faults, the handler records RIP and the exact
    address, makes the page writable and re-executes, and the client keeps running. It writes
    NOTHING to the client - VirtualProtect is a permission change, not an edit. Nothing can arm
    until the FIRST catch gives it a phase, so the opening minutes are an ordinary sentry run.

    This is NOT the guard-page build of heap-corruption-2026-09-06.md 3.2, and the reason is a
    measurement: TWO of the four catches on 2026-09-07 were LIVE slots, and a decommit-on-free
    scheme is blind to those by construction. It also replaces no allocator and hooks nothing.

    In client-patched\maplecw-hook.log, in order of value:
      "THE WRITER: a store to slot header X at +4 ... from RIP R"  -> THE ANSWER. R is the
                 instruction. Whose module R is in also answers "is any of this OURS".
      "POOL WRITE WATCH saw a write into a watched page"  -> the liveness control: pages are
                 really protected and faults really reach us. Expect several.
      "window #N open ... control PASS"  -> armed, and its own EXCEPTION_RECORD self-test
                 passed. "control FAIL" means it stood itself down rather than report silence.
      windows opened, ZERO write faults  -> report exactly that. It is a property of the
                 instrument, NOT evidence that the client did not write.
      no window at all  -> no catch happened, so there was never a phase to predict from.
    Cost: a stutter under a second every three minutes, on top of the repair's own.

    Four of the seven frame-tick tickers jump into .themida, rawsize 0, so if (B) comes back
    empty that is where they went and static analysis stops there. (C) does not care.

    (D) -FreeGuard is the OTHER half, and it is NOT for this run. The field crash of
    2026-09-07 died with the repair on and the live pool provably clean (0 damaged of 193 632):
    the same disease surfaced at a DIFFERENT free - PCOM's WZ property teardown on a map
    change handing a POOL CHUNK to the NT heap. -FreeGuard refuses that one free. It replaces
    ONE cached function pointer, PCOM+0xdbb80, which all seven of PCOM's free sites call
    through; the IAT is NOT the call site (PCOM does `call rbx`), and an IAT hook would have
    installed cleanly and intercepted nothing. -FreeGuardObserve logs without refusing.
    Both need -PinPatches to survive a launcher run. Watch for "FREE GUARD ARMED" and then a
    liveness line every 120 s: ZERO passes means the shim is not on the free path, so no
    refusal count from that run means anything.
    DO NOT combine it with (C). It is one more patch to the client, and (C) is the run that
    measures whether our patches matter. research/naming-the-writer-2026-09-07.md.

    WHETHER ANY OF THIS IS OURS is still open, and (C) is the first instrument that can
    answer it - the faulting RIP names a module. 75 of 75 archived client runs carried our
    hook, so the archive cannot. The patch-set control needs the LAUNCHER to write a smaller
    set - a change to crates/launcher/src/client.rs, not a command line. DO NOT retry
    -DirectClient for it: three launches on 2026-09-06, none reached the world, and one
    produced a retraction. research/is-the-corruption-ours-2026-09-06.md 5.

    T13 (NEW 2026-09-05). LOGIN IS ENFORCED. The launcher path is the ordinary run: sign in,
    Start Game, and the world as before - that half is regression. (Every Start Game also puts
    one "served as NOBODY - REFUSED" line in login.log about half a second before the served
    connection: that is the launcher probing the port before it starts the client - see
    crates/launcher/src/servers.rs - not a client being refused. Seen in the run of 2026-09-05
    18:17.) The new observation is the REFUSAL on a real client: with nobody signed in yet, start the PATCHED client with its own
    folder as the working directory - Start-Process -FilePath <repo>\client-patched\
    MapleStory.exe -WorkingDirectory <repo>\client-patched -ArgumentList '-NXLDEBUG',
    '127.0.0.1','8484'. The working directory is not optional: the hook writes its log and
    reads its markers relative to it, and the client inspects that folder. And it must be the
    client-patched copy, not the one under C:\Nexon, whose real GameGuard answers a hand
    launch with "Please delete the hacking program from the MapleStory launch folder" (seen
    2026-09-05 on the first attempt at this step). Expected: the client's own "not a
    registered ID" notice, and a client that stays usable afterwards. A FROZEN client means
    the refusal packet was not accepted by the login-result handler and the "always answer"
    rule is broken on this path - that is the finding, and it is the reason this is a step
    rather than a test in the suite (the suite proves the packet goes out, not that the client
    likes it). login.log shows "served as NOBODY - REFUSED" for that connection.

    T12 (NEW 2026-09-05). REGISTRATION AND RECOVERY, and the launcher half needs no client.
    As the GM type !registrationcode: a chat notice shows an 8-character code, XXXX-XXXX,
    and world-ch0.log must NOT contain it (grep it - the log line says "not logged"). In the
    launcher, Register tab: any username, an email, a password WITHOUT a digit -> refused on
    the spot and the code is still live; with a digit -> "account created", the Sign in tab
    comes back with the name filled in, Login works. Then !recoverycode <that email> in game;
    Forgot password tab with the email, the code and a new password -> the new one signs in
    and the old one is refused. The wrong identity with the right code -> refused AND the code
    still works afterwards. Every one of those sentences is a test in the suite; this run is
    whether the SCREENS say them.

    T11 IS THE MEASUREMENT, and T10 is right behind it. Both are walks, not clicks.

    T11 crosses to ANOTHER CONTINENT - 87 maps, a separate portal component from
    Victoria Island, reachable by the ship to Orbis (the ferry was removed 2026-09-29).

    **ORBIS ITSELF ALREADY LOADED, and this block claimed the opposite for a week.** Map
    20000000 was served twice on 2026-08-28, the client answered 0x00DC 526 ms later and
    four NPCs drew. So "no character has ever stood on them" was false when written, and
    the risky-sounding part of T11 - does the client survive an Orbis map - is already
    answered YES. What is untested is EL NATH.

    That correction matters more than the map: the sentence was repeated into the printed
    plan and into three conversations, and it made T11 sound like an expedition when the
    expedition had already happened. The archive had it the whole time.

    The second job advancement now exists end to end, and FOUR MAPS THAT NOBODY HAS EVER
    STOOD IN are part of it. 80001300 / 80001100 / 80001000 / 80001200 have exactly one
    portal each - the spawn point - so there is no way in or out on foot and the server is
    the only thing that has ever put anybody there. If a SetField into one of them goes
    wrong, the symptom is a character stuck in a map with no door.

    Everything else below is either cheap (T0, T6), already built and waiting for its first
    look (T7, T8, T9), or unrelated and worth doing while you are in there (T1, T2, T5).

     TV. PHIL'S JOB GUIDE IS A REAL MENU NOW. The owner, 2026-09-15: "Phil's dialogue to allow
         Beginners to choose a location to job advance to does not work. The selection is
         fundamentally broken and cannot be selected by the cursor." It was a chain of
         yes/no boxes drawn to look like a list - Yes and No were the only controls. Phil
         (Lith Harbor, template 101) now sends ONE type-6 menu, the same box the taxis and
         the second-job instructors use: heading, then a highlighted line per first job -
         "Swordsman - Dances with Balrog in Warriors' Sanctuary (needs 35 STR)" and the
         other three. Clicking a line rides to THAT instructor's map (arrival line on the
         right, then the SetField); Close sends nothing. Beginner, level 10+ only; the
         refusals for a non-beginner or a level-9 are unchanged (a plain OK box).
           1. a level-10 beginner clicks Phil: a list whose lines HIGHLIGHT under the cursor
              and can be clicked -> fixed. If it is still a Yes/No box -> the old chain,
              report it.
           2. click "Magician": a yellow arrival line, then Grendel's map (Magic Library),
              job still Beginner -> fixed. Talk to Grendel: the TR yes/no box.
           3. Escape/Close on the menu: nothing, you stay in Lith Harbor.

     TR. THE FIRST JOB ADVANCEMENT ASKS FIRST. The owner, 2026-09-15: "the moment you click on
         the first job instructors, you simply become that job. There should be a yes or
         no dialogue (including the requirement)". An eligible click now opens the
         instructor's yes/no box - "You meet the requirements to become a Swordsman:
         Level 10 or above, and STR 35 or more (you are Level N with STR M). A job
         advancement cannot be undone once made. Do you want to become a Swordsman now?" -
         and the job changes ONLY on Yes (the eligibility check runs again first). No says
         "Take your time"; closing the box does nothing. Refusals (level, stat, already
         advanced) are unchanged. The box is the same npc_ask Shanks and Phil use.
           1. a level-10 beginner with the stat clicks an instructor: the yes/no box with
              the sentence above -> as designed. Press No: a "take your time" line, job
              unchanged. Click again, press Yes: "Congratulations", the job changes and
              the skill window has points -> fixed
           2. click again after advancing: the "already taken that step" refusal, no box
           3. (2026-09-16) the Yes also hands over the Beginner's set, free: Long Sword
              (Swordsman), Wooden Wand (Magician), War Bow (Archer), Triangular Zamadar
              AND Garnier (Rogue). The congratulations names it, a grey "gained" chat
              line draws, and it is in the Equip tab -> fixed. A full Equip tab: the job
              still changes and a yellow line says the item could not be placed.

     TX. CONSOLIDATE ITEM WORKS (2026-09-18). The owner clicked it on the Etc tab and nothing
         happened - 0x0105 (u32 tick, u8 tab) fell to the generic latch unlock. The owner: "all
         items that can be stacked without violating their max stack size should be" and
         "move all items to take the first available slots in the inventory". Now: every
         later stack of an item is poured into the earlier ones up to slotMax, then every
         stack slides up so the tab has no gaps, order kept. Answered with plain 0x0070s
         (mode 1 counts, mode 3 emptied slots, then mode 2 bag-to-bag slides) - no guessed
         GatherItemResult opcode. Equips, pets and slotMax-1 items never merge.
           1. Use tab as on the screenshot (red 54, orange 21, a gap, blue 100): click
              Consolidate Item: the blue potion lands right after the orange, no gap,
              no "New" mark, the tab still usable (drag something) -> fixed
           2. two part-stacks of one item (split a stack first): one full stack and the
              remainder, the remainder AFTER the full one; counts add up -> fixed
           3. a tab with nothing to do: nothing changes, the tab stays usable -> fixed
              a frozen tab after any of these -> the nCount-0 reply did not clear the
              latch; paste the "consolidate" lines from world-ch0.log
              the client dies on the click -> paste client-exit.log; the mode-2 slide
              is the suspect (it is the shape a drag uses, but never in a burst)
         SORT ITEMS (same day, same shape: 0x0106). The owner: "Sort Items should sort by
         quantity, then name." It does the consolidate first, then puts the tab in order:
         BIGGEST stack first, then name A to Z (ties: item id, then old slot). The reorder
         goes out as mode-2 moves onto OCCUPIED slots - the packet a drag-swap is answered
         with, so the client should draw each as a swap. "Biggest first" is the reading
         taken of "by quantity"; one comparator flips it.
           4. click Sort on the Use tab of the screenshot: 350 arrows, 325 arrows, blue
              100, red 54, orange 21, orange 7, scroll 6, apple 3 (names break ties) ->
              fixed. Smallest first / wrong order -> say what you see, the comparator
              is one line. Two items in one slot or a blank slot -> the client did NOT
              swap on an occupied mode-2; paste the "sort" lines from world-ch0.log
         A DRAG ONTO THE SAME ITEM FILLS IT FIRST (same day). The owner: "it should try to fill
         the stack first (any remaining after the full stack will remain at the original
         position), if the resulting stack is already full, then it will carry out the
         swap slots procedure." The rows already did that; the REPLY was one mode-2, which
         the client draws as a plain exchange - so the screen swapped while the rows had
         merged, and the next drag on either slot moved the wrong stack. A merge is now
         two mode-1 counts (or a count + a mode-3 when the source is poured out); a full
         destination is still the one mode-2 swap; N of a stack into an EMPTY slot is a
         count + a mode-0 ADD. And the stack limit for a drag is now the same one pick-ups
         use, so two 100-stacks of a slotMax-less Etc item merge instead of swapping.
           5. drag 40 red potions onto 70: 100 and 10, in those slots -> fixed
              drag the 10 onto the 100: they SWAP (destination full) -> fixed
              split 4 off a stack into an empty slot (the count dialog): 4 there, the
              rest stay, no New mark needed either way -> fixed
              two stacks shown where one should be, or a stack that "comes back" on the
              next drag -> the client did not draw a count; paste the MERGE/SPLIT lines

     TW. A PARTY LEADER WHO LEAVES THE GAME HANDS THE PARTY TO ITS HIGHEST-LEVEL MEMBER
         (2026-09-18). The owner: "the party leader needs to be handed over to the next
         highest level player automatically", then: "Only the party leader should be
         handed off. The disconnected client should remain in the party. [...] If the
         leader position cannot be handed off to an online player, then the entire party
         should be disbanded." and, told a party could outlive everyone's connection:
         "If everyone is offline, the party shouldn't exist?"
         So: a log out, a crash or a dropped socket keeps the character's SEAT while
         somebody else in the party is online; if they led, the crown goes to the
         highest-level member who is ONLINE (ties: earliest joined). The LAST member
         online to leave - leader or not - disbands the party. A CHANNEL CHANGE changes
         nothing. A member who logs back in gets the party window rebuilt at the login
         field entry (0x0D - the first time this packet lands at login; watch it).
         Through the hub, so every channel agrees. Three clients to see the level rule:
           1. The owner (leader) + Tester2 (lower level) + Tester3 (higher level) in a party;
              The owner logs out: Tester3 becomes leader (crown moves in the two windows),
              The owner STAYS in both lists -> fixed. Tester2 leader -> join order won; paste
              the "party: character N, leader of party" line (it names the pick)
           2. The owner closes the client instead (crash path): the same
           3. The owner logs back in: their party window is there, Tester3 leads, all three
              listed -> fixed. No window -> the 0x0D at login did not draw; paste it.
              Client dies at login -> the 0x0D in the entry batch; paste client-exit.log
           4. Tester3 changes channel: still leader after, all three still listed
           5. Tester3 (the leader) and the owner log out, then Tester2 - a plain member, the
              last one online - logs out: the party is gone; anyone logs back in: no
              window -> fixed. A window -> a party outlived everyone; paste the
              "party: character N of party" line from world-ch0.log

     TQ. ACROSS CHANNELS: THE WORLD HUB ON 8483. The owner, 2026-09-14: party chat "should be
         broadcasted to all party members across channels", "do not use the database as
         a shared bus", "we can have a chat server hosted on 8483". Built: maplecw-chat,
         a third process the launcher starts first (log: chat-hub.log). Every channel dials
         it once (--link 127.0.0.1:8483; world-chN.log says "connected to the hub") and:
           - every PARTY REQUEST goes to the hub, which echoes it to every channel in one
             order; each channel applies it to its own copy of the party registry, and
             the channel hosting the clicker answers the client from the echo (a tick
             later, ~1 ms over loopback). A channel started late gets the whole registry
             on connect. So a party now EXISTS across channels: invite, join, leave,
             expel, leader, pick-up rights all cross.
           - a packet for a character on another channel (party chat, an invite dialog)
             goes to the hub, which forwards it to the channel that hosts them (the hub
             keeps who-is-where from each channel's field entries).
           - no hub (it is down, or --link none): the channel says so once and runs
             alone, exactly as before - nothing waits on the link.
         Two clients, ONE party, put them on DIFFERENT CHANNELS (Change Channel on one):
           1. party chat from either: the other, on the other channel, sees the line
              -> fixed. Nothing -> chat-hub.log shows whether the hub forwarded a 0x01B1
              for that character ("not in the directory" means the field entry never
              announced them; world-chN.log has "link:" lines)
           2. the party window on both still lists both after the channel change ->
              the registry crossed; if the member vanished from the leader's window,
              say which window and which channel changed
           3. invite a third character who is on the other channel: the dialog opens
              there -> the invite crossed
           4. leader clicks Pick-up rights: BOTH clients, both channels, see the
              "changed to ..." line -> the echo reached both
           5. change channel again and look at the party window: still intact
         WHISPERS: built from the owner's capture (00:29:35, "Hello Whisper" to Tester2:
         0x017B = u8 kind 6, u32 tick, str target, str text). The target gets 0x01B3
         mode 0x12 (the sender's name, id, channel, the text, then the same chat-info
         block a party line carries; every read is unconditional) wherever they are -
         other channel included, through the hub; the sender gets mode 0x0A, whose
         found byte draws "Tester2<< text" (the echo) or "Could not find Tester2."
         /find is answered with a plain line ("X is on channel 2") - mode 0x09's second
         field has not been read. Nothing of this has been on a screen.
           6. whisper the other client (/w Tester2 hi), on the SAME channel first:
              they see "the owner>> hi" (or the client's own whisper line), you see
              "Tester2<< hi" -> fixed; then with them on the OTHER channel -> crosses
              you see "Could not find Tester2." -> the directory did not have them;
                          chat-hub.log lists who is online where
              the client DIES on receipt -> the chat-info tail; say which client
           7. /find Tester2 -> "Tester2 is on channel N." as a yellow line
         MAPLE CHAT: run 7 (01:52) DONE for the open and the dialog - the owner's window
         opened, Tester2 got the invite, and the Accept was captured (0x01FD mode 7 +
         the messenger id). Built since: the accept seats Tester2 and answers with mode
         0 (the window opens) and mode 4 - the room's SIX seats, each with the member's
         avatar look (the same bytes the character select and 0x0224 draw); the owner gets a
         one-record mode 4 with the newcomer. The opener now gets the six seats right
         after their window opens too, which is what was missing when the owner "did not see
         even their own avatar". A typed line and a closed window are still uncaptured and
         are logged with their bytes. Rooms live in the channel process (both clients on
         ONE channel for this run; a cross-channel accept is answered "not here").
           8. invite Tester2 (same channel), Tester2 clicks Accept:
              The owner's avatar is in their window from the start; Tester2's window opens
                          with BOTH avatars; the owner's window gains Tester2 -> fixed
              windows open, seats empty -> mode 4 was read but the look was not drawn;
                          say whose window and how many silhouettes
              Tester2's window does not open -> world-chN.log says "result 1" (the room
                          was not on that channel) or shows the mode 0 line; say which
              a client DIES on the accept -> the look inside the seat record; say which
           9. type a line in the Maple Chat and close the window: both land in
              world-chN.log as "mode N ... THIS IS THE CAPTURE" - the next step.
         BUDDY CHAT: not built - no capture yet. Open the buddy list once and add someone;
         the bodies land in world-chN.log as UNKNOWN, which is the capture it needs.

     TP. PARTY CHAT, AND THE PICK-UP RIGHTS BUTTON. (Party mesos: DONE, "Party loot
         works".) the owner, 2026-09-14: "Hello" in party chat reached nobody, and Pick-up
         rights did nothing.
         PARTY CHAT: the line arrives as 0x0179 (kind 1, the client's recipient list,
         the text) and went unanswered. It now goes to every OTHER member on this
         channel, on any map, as 0x01B1 - the client's own group-message packet, whose
         handler reads u8 kind, u32 account, u32 char, str name, str text, then the same
         chat-info block the reference sends (name, text, ids, world, zeros). The sender
         is not sent a copy: its client draws its own line.
           ACROSS CHANNELS: see TQ - built the same night, through the hub.
         PICK-UP RIGHTS: the button's request carries no value (its builder hardcodes the
         payload - both of the owner's clicks were byte-identical), so it is a TOGGLE, and
         0x2D is the client's own rights-changed packet: str name, u8 isPublic, u8 rights
         (1 = Party Leader, 0 = All) - the arm stores both, says "The party's item
         pick-up rights changed to %s" when it differs, and relabels the window. Under
         Party Leader, only the leader (and a drop's killer) may pick up party drops.
         Two clients in a party, same channel, different maps is fine for chat:
           1. type a line in party chat on one:
              the other sees "[name]: line" in its party chat colour -> fixed
              nothing arrives -> world-ch0.log: "-> N of M member(s)" says whether
                          the bus delivered; 0 of 1 means the member was between fields
              the client DIES -> the chat-info tail; say so, the hook log names the packet
           2. leader clicks Pick-up rights:
              both see "The party's item pick-up rights changed to Party Leader" and the
                          window's label follows; click again -> "...to All" -> fixed
              the line says the opposite of the label -> the byte's meaning is
                          reversed; say which word appeared first
              nothing -> world-ch0.log has the 0x2D lines; say so
           3. under Party Leader, the member tries to pick up a party drop: refused
              (it stays on the floor); the leader takes it.

     TO. PARTY MESOS: 70% TO THE PICKER, A YELLOW COPY OF THE SHARE TO EVERY MEMBER.
         The owner, 2026-09-14: the EXP split, for mesos. Built: when a party member picks up
         mesos A MOB dropped, the picker keeps 70% (white "You have gained mesos"), and
         every other member on the map is credited a copy of the party share (30%, the
         same !setrates field EXP uses) and sees the client's OWN yellow line for it:
         "Spotting Small Change (+n)" - string 0xE5, drawn by the smallChange field of
         the pick-up message with the same colour selector as party EXP. Mesos a player
         dropped are 100% to whoever picks them up; no party, or nobody else on the map,
         is 100% too. Nobody has sent a non-zero smallChange to this client before.
         Two characters in a party on one map (two clients, or one plus a second
         character on the other channel is NOT enough - same map, same channel). Kill a
         mob, let one pick up its mesos:
           picker: white "You have gained mesos (+70% of it)"; other: a YELLOW "Spotting
                          Small Change (+30%)" and their meso counter rises -> fixed
           other gets the line but the counter does not move -> the 0x007C is missing
                          or late; world-ch0.log has both packets in order
           other gets a WHITE "gained mesos" line -> the share was over 65,535 (the
                          line's u16) and fell back on purpose; say the amount
           other gets nothing -> world-ch0.log says whether the share was mailed
                          ("party member N ... gone by delivery" is the miss)
           the picker's own line ALSO shows a small-change line -> report; it should not
         Then drop mesos from the inventory and pick them up: 100%, no member line.

     TN. THE SUMMONING SACK: THE BALROG MOVES, AND IT ARRIVES WITH THE CIRCLE.
         The owner, 2026-09-12 04:21: the sack's Balrog *"does not have AI and does not have
         movement and does not use skills"*, and *"it is also missing the summon effect
         that is played for all players."* Two causes, two changes:
           1. The sack spawned the mob (0x03C6) and granted nobody control (0x03D2). The
              server never drives a mob; the client that holds 0x03D2 runs its wander,
              aggro and skills. Field entry and the respawn tick grant it; the sack did
              not. It does now, to the summoner, right after the spawn.
           2. The spawn now carries the template's summonType as its appear type: the
              client plays Effect/Summon.img/<n> for everyone on the map (Balrog: 0, the
              2.5 s circle) and holds the mob SUSPENDED - untargetable - until a 0x03E8
              MobSuspendReset, which the server now sends to the map when the animation
              ends (read off the client's own 0x3E8 handler, 141c82390; the old "summonType
              makes mobs permanently unhittable" warning was this packet's absence).
         Use a Balrog sack (2100006) on an empty platform. Watch for:
           the summoning circle plays, THEN the Balrog walks and attacks, and you can hit
                          it after ~2.5 s -> both fixed
           no circle, but it moves -> the appear type is being ignored; say so, and
                          whether it was hittable at once
           circle plays, it never moves and cannot be hit -> the 0x03E8 did not clear
                          the state; world-ch0.log shows whether it went out (2500 ms after
                          the spawn line) - if it did, the next run watches 141c82390
           it moves but stays unhittable -> the reset reached it late or not at all; same
           the client DIES at the spawn -> the appear-option word; say so, the hook log
                          names the packet
         (Jr. Balrog 800020 and 700004 are summonType 0 too; the sack's other mobs are 1,
          a 0.4 s pop. Both are Summon.img entries in this client.)

     TM. THE OUTFIT IS IN THE DECO TAB, AND THE CHAT SAYS "UBEL"..
         The owner, 2026-09-11 04:12: the Ubel set opened, the chat drew the U-umlaut as a box,
         and the Deco tab was empty. The four equips had gone to the EQUIP tab by their
         leading digit; the client keeps a cash equip (WZ info/cash = 1 - every backported
         Signature Style equip) in tab 6, Deco, and every request it builds about one
         names that tab. equips.txt now carries a `cash` column (596 of 1795 equips), the
         server places by it, restores the Deco tab on every field entry, and moves any
         cash equip found in the Equip tab into Deco first - your four included, on the
         next field entry, with a line in world-ch0.log each. Chat notices are folded to
         ASCII before they go out (Ubel, not a box).
         Log in and change map once (the relocation runs on field entry):
           the Deco tab shows Ubel's Clothes, Shoes, Gloves and Weapon -> fixed
           Deco still empty, Equip tab still has them -> world-ch0.log has no "moved from
                          Equip slot" line: the config did not flag them; say so
           Deco empty AND Equip tab lost them -> the Deco Add is not drawn; the rows
                          are in the database (world-ch0.log lists the moves)
           double-click one in the Deco tab -> equips? (not built for Deco - report)
         Then open the Cash Shop, double-click Ubel's Clothes in the Item Inventory's
         Deco tab: it should go to the Cash Inventory like a Cash-tab item does (0x0B
         with tab 6). Say what happens.

     TL. BAG -> LOCKER, WITH EVERY CASH ITEM CARRYING A SERIAL..
         Run 6 (04:03) measured the prediction: "nothing moves back" and NO 0x03E1 in
         world-ch0.log after the entry reload - the client sent nothing, because every item
         in the Cash tab had come from the bag restore with +0x38 = 0, and the double-
         click builder (1410cff01) builds no request for those. So now EVERY body that
         lands in the Cash tab - the field-entry restore, the shop-entry restore, every
         Add - carries hasCashSN = 1 and a serial: 0x40000000|character in the high
         dword, the slot in the low. The server never decodes it (0x0B names tab+slot);
         the client only hands it back and the 0x1B record echoes it, then the 0x04
         reload re-keys the row. Other tabs are unchanged. A Cash-tab BUNDLE body is
         8 bytes longer than before on this run - the one variable.
         Enter the Cash Shop. The Item Inventory should show the Cash tab as before
         (if it is EMPTY, the longer body is the reason - say so at once). Then:
           1. double-click a Mystery Hair Coupon (a bag-restored item, never moved):
              it goes to the Cash Inventory and leaves the Item Inventory -> fixed
              nothing happens at all -> the builder still saw +0x38 = 0; the restore
                          body is not reaching the item; world-ch0.log has the body
              in the locker AND still in the Item Inventory -> the lookup missed; say
                          whether anything ELSE in the Cash tab vanished (slot 0)
              "unknown error" -> refused; world-ch0.log says why
              client DIES -> say so; the longer Cash-tab body is the suspect
           2. drag it out again -> should move (0x0A on the re-keyed row)
           3. exit the shop, open the inventory Cash tab: the coupons are where they
              should be; re-enter, both panels agree with the database

     TL2 (DONE 23:27). THE CASH SHOP'S TWO EMPTY PANELS - MEASURING RUN 2. (Run 1 is below, DONE.)
         Run 1 (23:20) measured: the 0x04 handler ran once, the locker-map INSERT ran
         SIX times (one per row), the panel REPAINT ran seven times. So the client HAS the
         six records and repainted - the rows exist and are invisible. The widget-ctor
         watch was blind: that constructor is a generic UI slot and hit its 32-call limit
         during login, before the shop. Read since: the repaint builds one 35x35 widget per
         row at (0,0) and the LAYOUT places the visible ones on a 6x2 grid, 38 px apart,
         offset from a resource anchor "list_lt" (present in CashShopUI.img at 191,462),
         through one call per widget whose 2nd and 3rd arguments are x and y. Run 2 reads
         those arguments. Same launch shape as run 1, -PinPatches included:

           -PinPatches -Probe "watch@1415db360:ret,141b2a280:rdx=0,1410b6060,1417113f0:hits=200,14170fd10:hits=400,1410b6970"

         Enter the Cash Shop once, exit, close. In client-patched\maplecw-hook.log:
           1410b6060  grid placement        expect >= 1 per repaint
           1417113f0  place-and-show, PER WIDGET: rdx = x, r8 = y
                        x in {191, 229, 267, 305, 343, 381} and y in {462, 500}
                                     -> placed on the grid; the ICON DRAW is the fault
                        x or y = 0, or wild -> the anchor lookup failed; placement is the fault
                        no hits at all  -> the visible list was empty; the layout filtered
           14170fd10  widget ctor           count the hits AFTER the 0x05AE dispatch line
           1410b6970  post-placement refresh
           client DIES -> say so; the watch set is the one variable

     TL1 (DONE 23:20). THE CASH SHOP'S TWO EMPTY PANELS - A MEASURING RUN, and it needs ONE flag.
         The owner, 2026-09-10 night: the Cash Inventory shows nothing after six purchases,
         and the shop's Item Inventory shows nothing although the character holds three
         Mystery Hair Coupons in its Cash tab. The purchases are in the database (LP was
         debited, cash_locker has the rows). The server now lists the locker with 0x04
         (Res_LoadLocker_Done) at entry AND after every buy, and that drew nothing either.

         I have read the whole client pipeline - 0x04's handler, the locker-map insert
         (rejects only serial -1), the repaint (one 35x35 row widget per map entry, no
         filter), the layout - and it says the rows should draw. So this run MEASURES it.
         Add this to the launch line, exactly, and nothing else new:

           -PinPatches -Probe "watch@1415db360:ret,141b2a280:rdx=0,140d7e1f0,140d75850,1410b5540,14170fd10"

         **-PinPatches is not optional, and leaving it out cost the 23:18 launch.** The
         LAUNCHER overwrites maplecw-hook.probe with its own defaults on every launch, so a
         -Probe given here reaches the client only as a pin the launcher consumes once. The
         hook log of that run armed the four defaults and none of the four wanted watches.
         Proof the pin was taken: the launcher's log pane prints OVERRIDES before the launch,
         and client-patched\maplecw-hook.log has "probe: watching 0x140d7e1f0". If neither
         is there, the run was NOT instrumented - say so rather than reading it as zero hits.

         The first two are the MANDATORY client patches from the default -Probe (the
         reachability skip and the login-dialog suppress); giving -Probe replaces the
         default, so they have to be repeated. The default's two observers (141b36f60,
         142ef3e44) are dropped for this run: six slots, and four are needed here. The
         layout (1410b5a40) is not watched because the repaint calls it unconditionally.

         Then enter the Cash Shop once and exit. client-patched\maplecw-hook.log gets a
         WATCH line per call, with arguments. The four, in pipeline order:
           140d7e1f0  the 0x04 handler     expect 1 hit at entry
           140d75850  locker-map insert    expect one hit PER ROW (6 today)
           1410b5540  panel repaint        expect >= 1
           14170fd10  row-widget ctor      expect one hit PER ROW
           all four as expected -> the rows exist and are invisible; the draw is next
           insert hits, no repaint -> [stage+0xc8] is null at that moment; a late repaint
           no insert hits at all  -> the 0x04 body is not reaching the handler as read
           the client DIES -> say so; five watches once coincided with a close (T17) and
                              this is the one variable this run changes
         Nothing else is on this run: no purchase, no coupon use.

     TK. THE STATION: THE DOOR, THEN THE CLOCK. Both 2026-09-10.

         a) THE DOOR - CONFIRMED 2026-09-10 evening: "I can indeed press up at the correct
            location and be teleported to Ellinia station." STRUCK. (It has no picture on
            purpose: pt 8 has no game graphic in MapHelper.img; the arch is the door.)
            TF is testable now - the Free Market doors send the same packet.
            Kept for the record:
            In Ellinia, stand where you stood for the screenshot - top right,
            (819, -3072), the `in03` spot - and press UP.
            Yesterday's fix put the destination in the table and the door stayed dead,
            because a SCRIPT portal (pt 7/8) does not send the ordinary `0x00D1` at all.
            Your two presses this morning at 11:27:53 sent **`0x014A`** - `u8, "in03",
            x, y` - which the server logged as UNKNOWN and never answered. Twelve captures
            of it across three runs, every one "in03". It is handled now, and it goes
            through the same named-portal lookup as a walk, so THE FREE MARKET DOORS ARE
            THE SAME PACKET (all four are pt 7) - TF below was untestable until this.
              you arrive in Ellinia Station -> done, and TF is worth doing
              nothing -> grep world-ch0.log for 0x014A. If it is there and no SetField
                         follows, the lookup missed: say what the log line says
              you arrive somewhere else -> say where; the reverse-link derivation is wrong

         b) THE CLOCK, once you are in the station. It sat at 00:00 because nothing ever
            sent it a time. It is `0x01BC` type 1 - hour, minute, second - and it carries
            **UTC**, per your correction this morning: the same time every world-ch0.log line
            is stamped with, so the wall and the log agree.
              it shows UTC (your taskbar plus four hours), AM/PM right, and TICKS -> done
              it shows your taskbar time -> local got through; serverclock.rs is wrong
              still 00:00 -> grep world-ch0.log for 0x01BC on that entry. Sent = the client
                         ignored it; not sent = clocks.txt does not list the map
              the client DIES on entry -> the widget was missing when the packet arrived,
                         which the decode says throws. Every map in clocks.txt is one
                         whose image declares the node, so this is the reading I do
                         not expect. Say which map.

     TG. JOSIAH'S TOP AT CHARACTER SELECT - one discriminator, no server change.
         Yesterday said "Cobalt is female". Wrong: the database says male and the client's
         own MakeCharInfo lists their face and hair only under male. So the gender gate was
         never what kept the top off the select screen. The ONE difference the data shows:
         the Blue Sergeant needs STR 30 / DEX 10 and Cobalt has STR 27 / DEX 5 after the AP
         reset into INT; every other worn item requires nothing. Hypothesis, not finding.
         **Superseded by your next request, and the same launch answers both.** You asked
         for the select sheet to show equipment totals like the in-game window (STR 1026,
         not 27). The login server now sums worn items the way the field client does -
         stored block first, template second - into the sheet it sends. So just log out to
         the select screen and look at TWO things:
           the sheet reads STR 1026 / DEX 1006 / INT 1073 / LUK 1003, HP 517 -> the
                         totals match the in-game window. If any number differs, say which
           AND the top now draws with no AP spent -> the select renderer checks item
                         requirements against the sheet it is handed (base 27 < 30 failed,
                         1026 passes). Then the server needs a REQUIREMENT gate on equip
           sheet right, top still bare -> requirements are not it; say so and I go to
                         the client with the two screens' difference narrowed to the look

     TL. THE BLANK CHARACTER-SELECT SCREEN - CONFIRMED FIXED 2026-09-12 09:23 ("it seems
         consistently fixed now"). STRUCK. Kept for the record because THREE theories died
         on the way and the fix is a hook step a tidy-minded agent could delete:
         research/charselect-avatar-fade-race.md. The client builds the select UI ONCE and
         fills its three slots from whatever character list exists at that instant; on a
         fast start it builds it 30 ms after the login request, inside the 0x0032 dispatch,
         from an EMPTY list, and the mode-2 login handler never refills. The hook now calls
         the client's own refill FUN_141177e40 after every 0x0010 on which the select UI
         already exists (grap_stub::session::refresh_select_after_dispatch). Four for four
         rescued on the confirming launches; a wiring test keeps it in hook.rs. Kill switch,
         one launch: -PinPatches -Session 'mode=2,create=on,guardpage=0x20+0x40,selectfill=off'.
         Never watch 141177e40 in -Probe: the int3 is the byte the guard reads (it tolerates
         it now, and the SELECTFILL: log line already says when the call happened).
           blank avatars ever again -> grep maplecw-hook.log for SELECTFILL first: absent =
                         the step was removed or switched off; "refusing" = a guard tripped
                         (paste it); "called" and still blank = new problem, keep the log

     TU. EVERY INBOUND OPCODE HAS A DISPOSITION - 2026-09-12, a LOG check, no screen step.
         Everything that still logged "UNKNOWN ... is not answered yet" is a one-way client
         report (0x013D 30 s census - MUST NOT be answered - 0x01ED log channel, 0x01A5,
         the 0x0420..0x0426 leaving burst, 0x0425 resource census, and three undecoded:
         0x01C1 0x01B9 0x0226). Named, routed through session/reports.rs, answered with
         nothing on purpose, and both servers now say so. Seventeen HANDLED opcodes had no
         name either (0x00E5, 0x0199, ...) - fixed and pinned by a test.
         research/opcode-dispositions-2026-09-12.md. After any launch:
           grep -c UNKNOWN login.log world-ch0.log world-ch1.log  -> 0 is the expectation.
                         Any hit is a packet this project has never seen: paste the line
           grep "is a client report" world-ch0.log -> the correct silences; 0x0422 lines now
                         say "leaving the field: reason N" - a reason other than 2 or 4
                         is new information, say which
           anything "is not answered yet" that is NOT a report -> a real gap; paste it

     TR. THE AP AND SP RESET SCROLLS - CONFIRMED 2026-09-10: "both AP and SP scrolls now
         work." STRUCK. Kept for the record: they were on the wrong opcode. Your two presses at
         02:59:52 and 02:59:55 were `0x0116`, not the coupons' `0x0114`, and nothing answered
         them: that is why nothing happened AND the item stayed. Handled now, through the
         same full refund `!resetap` / `!resetsp` do.
           the stat window refunds to base and shows the AP, the scroll is gone -> done
           the stats reset but the scroll stays -> the consume failed; world-ch0.log names it
           nothing, and the inventory is FROZEN -> the latch is not cleared; relog and tell me
           a message but no change -> the reset itself refused; the chat line says why

     TB. THE BEAUTY COUPON DIALOG'S WHITE NAME - CONFIRMED FIXED 2026-09-12 ("The dialogue is
         readable now"). STRUCK. Kept for the record: Frieren's was white too, so it is the client:
         its own string 0x0464 is "Would you like to use #fc0xffffffff#%s?" - opaque WHITE,
         meant for the modern dark panel. The hook now patches six bytes of that encrypted
         string at load so the colour is black. Kill switch: -NoBeautyTextPatch.
         Double-click any hair or face coupon.
           the name is black and readable -> done
           still white, and maplecw-hook.log has "BEAUTYTEXT: patched" -> the client
                         decrypted the table before the hook ran; I move the six bytes into
                         the exe file instead
           still white, and the log says "refusing" or "not readable" -> paste that line

     TO. THE ÜBEL OUTFIT - three findings from your run, two fixed, one not.
         a) The clothes (overall, shoes, gloves) from the DECO tab: the client SENT the
            equips (invType 6, slot -> -105/-107/-108) and the SERVER refused them - the
            equip path only knew the Equip tab. Fixed: Deco moves go on to worn slots
            101+ and come off into the Deco tab. Drag each on again.
              it sits in the Deco equip window -> the move works
              BUT the character does not draw it and it is gone after a relog -> EXPECTED
                         for now: the record cannot carry worn slots above 31 (measured,
                         the client discards them) and the look's cover map is unbuilt.
                         That is the next piece of work, not a failure of this one
              refused again -> world-ch0.log names the reason; paste it
         b) The weapon: CLIENT-side, and it is the data - FIXED IN THE DATA 2026-09-12,
            INSTALLED the same night (client closed; all 24 archives; originals kept .bak). The classic covers spell the weapon TYPES they dress as
            children: 01702001 has a real 30 and 31/32/33 as LINKS to it. Every backported
            cover has only 30 and 49 (49 = gun, a type this client lacks); your suitcase is
            type 32, so the client found nothing and refused. tools/backport_install.py now
            gives each of the six covers a link to 30 for every classic type (31,32,33,
            37..47); built, verified and installed - the cover on disk lists all fifteen.
            research/weapon-cover-types-2026-09-12.md. Launch, and in game drag the weapon
            onto the suitcase:
              it goes on (sits in the Deco window) -> the per-type rule is confirmed
              it does not DRAW -> expected: covers cannot draw until the record carries
                         worn slots above 31 (the same open item as a) above)
              still refused, silently -> the check is not the type child; say so and I
                         read the client's equip check rather than the data
              refused WITH a message -> paste the message
         d) NEW 2026-09-12 - THE RECEIPT. Opening the Collection or any Outfit Set Coupon
            now also opens an NPC box from "MapleStory Administrator" (NPC 9010000,
            renamed in String.wz from "Maple Administrator"): "You have received the
            following items:" then one line per item, icon + name (#i/#t tags). OK closes
            it. Open a set coupon and look:
              the box appears, every item on its own line with its icon -> done
              box appears, names missing or "null" -> #t does not resolve for that id; say
                         which lines (the equips' names live under Eqp.img/ClassicWorld)
              box appears, no icons -> #i does not resolve; say so, and whether the
                         inventory shows the same icons
              lines run together -> the client wants a different line break; say so
              no box at all, items still arrive -> the unsolicited Say is refused; paste
                         world-ch0.log's "ScriptMessage ... receipt" line and what came after
              the name reads "Maple Administrator" -> the String.wz rename did not land;
                         say so
         f) THE BOX ITSELF - "Double clicking the Signature Style Collection box does not
            grant all 8 character costume coupons" (2026-09-12). world-ch0.log shows NO packet
            for the double-click: the client did not treat it as a use. It opens a Cash item
            on double-click by its id FAMILY; Nexon's box is 5222221 (family 522, which this
            client has no items of) while the set coupons are 5681xxx (family 568, its own
            5-slot coupons) and their double-click sends 0x0114 every time - measured, this
            session. [The family reading is I; the pair of controls is L.] So the box now
            wears 5681599: its node, string and Cash Shop row are installed under it, the
            server's COLLECTION is 5681599, and a box already in a bag under the old id is
            rewritten on the next server start (store::inventory::rename_item_ids, every
            open, so the live database gets it too). No archived run ever recorded the box
            opening - this was never exercised on screen before today. Double-click it:
              the eight coupons arrive (Cash tab), the Administrator's receipt lists
                         them, the box is gone -> done
              still nothing, and world-ch0.log has no 0x0114 -> the family reading is wrong;
                         say so, and I read the client's double-click dispatch instead
              0x0114 arrives but the server refuses -> world-ch0.log names why; paste it
              the box in your bag shows no name / a blank icon -> the rename on open did
                         not run; grep world-ch0.log for "rename" and paste
         g) CASH EQUIPS SURVIVE A RELOG - CONFIRMED, and the weapon cover too (the owner,
            2026-09-12: "Ubel's weapon is fine in game and on character select"). The cover
            rides in the look's weapon-sticker u32 (look+0x2d) and the real weapon keeps
            slot 11. Nothing to test here any more.
         h) HIMMEL'S CAPE - DONE (the owner, 2026-09-12: "Himmel's cape now looks fine"): the
            effect draws behind the body at z -2. Nothing to test here.
         i) THE HAIR-HATS - RENUMBERED, 2026-09-18. The gate was found by reading, not by
            the watch run: the client's gender-from-id rule (FUN_140253130, the FOURTH digit)
            reads 6 as FEMALE, so 1006910..1006912 were female-only caps, the body-part
            resolver returned nothing for a male character, and the double-click handler
            never called the equip path at all (which is also why FUN_1417dd7e0 was the wrong
            function - it is the drag-from-a-worn-slot path; the double-click is
            FUN_1417da2a0 behind FUN_142d44b20). No WZ key overrides the digit in this
            build, so per the owner ("fix it in the WZ data instead of patching the client") the
            three wear 1007910..1007912 now - digit 7, unisex, free. Hats already in a bag
            are renumbered by the store on start. NEEDS the rebuilt client package (or
            backport_install.py --install locally, done 2026-09-18 01:53).
              double-click Aura / Linie / Lugner Hair (Hat) in the Deco tab on a MALE
              character: it goes on, hair hidden under it                    -> DONE
              goes on but the character's own hair shows through the hat -> vslot's
                         modern H-tokens are ignored; say so, that is the next variant
              still nothing, and no message box -> paste the bag line for the hat from
                         world-ch0.log: if it still says 1006xxx the package is old
              Linie only refuses a SECOND Linie hat: as designed (onlyEquip 1 in the WZ)
         j) NEW 2026-09-12 - THE FACE COUPONS OPEN THE DIALOG. "The face coupons from the
            backported collaboration items still does not work." No packet in any run: the
            Beauty Coupon dialog (the 0x0165 CONFIRM) opens only for ids in seven ranges read
            off its opener - 2540000..2549999 (hair coupons, which work), 2890000..2890999
            (the modern client's own Face Coupon family), plus skins and android faces - and
            2897xxx is in none of them [L]. So the eight face coupons now wear 2890907..914
            (node, canvas, string, server table, and a rename of any already in a bag on the
            next server start). Double-click a face coupon in the Use tab:
              the Beauty Coupon dialog opens with the face previewed, CONFIRM changes your
                         face, coupon gone -> done
              the dialog opens but previews nothing / a hairstyle -> the dialog keys hair
                         vs face on something else; say what it shows
              still nothing -> the range reading is wrong; say so
              the coupon in your bag has no name/icon -> the rename on start did not run
         k) NEW 2026-09-13 - NINA NO LONGER REPEATS HERSELF ON ACCEPT. "They say the same two
            dialogues before and after I click 'Accept'" (What Sen wants to eat, 1003). The
            client shows a quest's Say.0 itself before the button; the server's answer to the
            Accept (0x0151 action 1) is the Say.0.yes branch - and 1003 has none, so the code
            fell back to Say.0 and replayed it (world-ch0.log 04:06:17). An accept with no yes
            branch now sends the quest record and NO box; quests that have a yes branch
            (Heena's 1000) are unchanged. Take 1003 from Nina again (give it up first if held):
              their two lines, Accept, the window closes, journal has the quest -> done
              their two lines, Accept, the SAME two lines again -> the fix is not in the
                         running server; say so
              their two lines, Accept, their greeting ("Hello...") -> the d0 fallback is still
                         reachable; say so
              Heena's quest no longer says "hill to the east" after Accept -> the yes branch
                         broke; say so
         l) NEW 2026-09-13 - THREE SNAILS THROWS A SHELL. "Three Snails is a skill that takes
            1 Red Snail Shell to cast ... the skill should output a red error text in chat
            saying you do not have enough Red Snail Shell ... Casting it should decrease the
            client's Red Snail Shell inventory count by 1." Skill.wz: level 1 throws a Snail
            Shell (4000001), level 2 a Blue Snail Shell (4000002), level 3 a Red Snail Shell
            (4000004), one a cast. The server now takes the shell from the Etc tab on every
            cast (0x0070 count change) and, with none, answers the swing with ONE system
            chat line (category 11, the client's own "You cannot" colour) and nothing else:
            no MP, no damage, no broadcast. Cast it at a snail with a few shells, then with
            none:
              the Etc count drops by one per cast; with none, the red/pink line appears and
                         the mob takes no damage -> done
              the count drops on the server (world-ch0.log "itemCon: ... threw") but not in
                         the bag window -> the 0x0070 shape for Etc; say so
              the line appears in a colour that is not the client's usual error colour ->
                         say which; category 11 is the guess, 1..5 are the client's own
              the swing still hurts the mob with no shell -> the client applies damage
                         locally; say so (then the refusal has to also revert HP)
              nothing at all with no shell -> the line was refused; paste world-ch0.log's
                         "itemCon" line
         m) RAIN'S QUIZ - the MENU works (2026-09-13: quiz 2 answered, "Yup..." drawn), and
            then "Rain's quiz dialogue repeats after I choose the correct answer. That is not
            okay." world-ch0.log 04:59:24: the record, the exp and the fanfare went out BEFORE
            the question, then the menu, then the answer and the closing line - and the
            server sent nothing after. So what came back was the client's own doing: it acts
            on a completion at once and offers the NEXT quiz (1015's opening), which the menu
            covered and which reappeared after the closing line. Say.1.ask = 1 is the data
            saying the turn-in depends on the answer, so now a quiz's completion WAITS for the
            right choice: the click sends the question alone; the right pick sends the record,
            the exp, the fanfare and "That's right!"; a wrong pick or a closed box leaves the
            quest in progress. Take Rain's Maple Quiz 3 (talk to them):
              question; right answer -> quest-clear sound, "That's right!", OK closes, and
                         Rain's NEXT offer opens ONCE (that is the chain, and it is theirs to
                         open) -> done
              the offer opens BEFORE the question again -> the completion is still early;
                         paste the world-ch0.log lines between 0x0151 and the menu
              wrong answer, close the box: the quest is still in the journal as started,
                         and clicking Rain asks again -> done for that half
              "That's right!" but no quest-clear sound / exp -> the deferred record did not
                         fire; paste world-ch0.log's "quiz:" line
         o) NEW 2026-09-13 - ONE REWARD FROM THE POOL. "When I finished 'Please bring this
            letter to Lucas', Maria gave me one of every single Headband item when it's
            suppose to be choose 1 randomly from the pool." Quest 1008's Act.1.item.1..7 are
            seven headbands each marked prop 1; the letter back (item.0, count -1) has no
            prop. The turn-in handed over every row. Now prop 0 rows are unconditional, the
            prop > 0 rows of a state form a pool and ONE is drawn with weight prop, and a
            gender-marked row goes only to that gender. 39 quests carry the mark. Any of them
            on a fresh character - Lucas's Reply is the nearest: take Maria's letter to Lucas,
            bring their reply back:
              ONE headband in the Equip tab, the letter gone -> done
              still all seven -> the running server predates the fix; say so
              no headband at all -> the draw picked nothing; paste world-ch0.log's "reward rows"
                         line
         p) NEW 2026-09-13 - THE ELEVEN PETS, PERMANENT, IN THE SHOP. "Brown Puppy, Panda and
            Dino Boy all have 3 day duration ... change all of them to permanent duration.
            Also please add all of the other pets into the Cash Shop ... They should never
            need to be revived." The duration the shop showed was each pet's own
            Item/Pet/<id>.img info/life (3, 7 or 90 days) - the three rows already said
            Period 0. Every pet now carries permanent 1, the shape of the modern
            client's one permanent pet [L on the modern data; that the classic client honours
            `permanent` is I]. The eight pets with no row have one under the Pets tab (SN
            160000003..10, 100 LP). And the PURCHASE: a pet was REFUSED until today because
            no type-3 item body existed (a bundle sent for one killed the client on 08-26);
            the body is built now off the client's own pet decoder - name, level 1, fullness
            100 - so buying works. (life 0 was WRONG and is reverted: it is row 2 of the
            deadness test in step v, and it hid a bad dateDead for two runs.) Summoning a pet to
            follow you is NOT built yet; this run is about the shop and the bag.
            Open the Cash Shop, Pets tab:
              eleven pets, each "permanent" (or no duration line) -> the data half is done
              still "3 days" on the three -> the classic client reads a different node; say
                         what the tooltip says exactly
              buy one: it lands in the Cash Inventory panel with its icon, moves to the Cash
                         tab, and double-clicking it does whatever it does (probably nothing
                         yet) WITHOUT the client dying -> the body is right; say what the
                         double-click did
              client dies at purchase or at the move -> paste client-exit.log and the last
                         0x03E1 / 0x0070 line in world-ch0.log; the pet body is I on meanings
              only three pets listed -> the Pets tab does not list by SN prefix; say so
         q) NEW 2026-09-13 - THE REPEAT-DIALOGUE AUDIT, every quest. "Please audit all of the
            questline and make sure repeat dialogue is no longer a concern." The rule the three
            fixes converge on: the client shows a quest's OPENING (Say.0) itself, so the server
            must never answer that quest's Accept or turn-in with its own Say.0, and one request
            never opens two boxes. A test now walks all 316 quests with dialogue through Accept
            and turn-in and asserts exactly that: Accept -> 157 speak their yes branch, 159 send
            the record alone; turn-in -> 287 speak Say.1, 11 ask a quiz (menu, completion on the
            answer), 7 chain to the next quest's opening (accepting it in the same breath, so the
            client does not offer it again - 1000 -> 1001, seen 2026-08-20), 11 send the record
            alone. It found one more: a finished quest with nothing to say (1002, Roger) was
            answered with the NPC's greeting - silent now. The one path the server still speaks
            Say.0 on is action 4, the opening SCRIPT (27 captures, all quest 1002, all fine): a
            scripted quest has no local text. Nothing to test as a step; take any quest and it
            is covered. If a line still repeats anywhere, name the quest and WHICH line (the
            opening, the yes branch, the completion) - the audit test is where it gets pinned.
         r) NEW 2026-09-13 - THE CEILING COUNTS WORN HP. "Currently the character the owner has
            199 max, but passive recovery only recovers up to 194 and stops." The database
            said 194; the five is the Red Headband (incMHP 5), which the client adds to the
            bar itself. The server's ceiling counted Max HP Increase's percent and not a worn
            item's flat, so it called 194 "full". Every ceiling (regen, potions, level-up
            refill, !heal, party bar) now adds the worn items' incMHP / incMMP, flat before
            percent [R on the order; nobody here has both]. Stand still on the owner with the
            headband on, HP below 199:
              ticks up to 199 and stops                     -> fixed
              stops at 194 again                            -> the client adds something else
                         too; say the hat's tooltip and whether a scrolled item is worn
              overshoots or the bar jumps to a new maximum   -> the flat was folded into the
                         record; paste the 0x007C line
         s) NEW 2026-09-13 - A KILL REFILLS THE MAP, NOT THE POINT. Audit first: the solo cap
            IS applied - the last two runs sent 49 of 66 on A Split Road and 31 of 42 on map
            50, exactly 75%, never more over a session, and the fan site's own figure for
            Split Road is 49.5. What made it feel pinned at the cap: a dead mob came back
            after 7 s on the SAME point, so the same 49 points held forever and 17 were never
            used. The owner: "once the mob is dead, a completely random spawn point should be
            chosen that's not necessarily the dead mob's spawn point." Now a kill books a
            refill of the map: when due, one free ORDINARY point is drawn uniformly from all
            of them and that point's mob stands up - possibly a different type. Timed points
            (WZ mobTime > 0) still return at their own place on their own clock; mobTime -1
            never. The cap is unchanged: one death, one refill. Kill a few on Split Road:
              a mob stands up somewhere else on the map, sometimes a different type,
                         and the count stays 49                        -> as designed
              it always comes back on the spot it died                 -> the draw is not
                         reaching the field; paste the SPAWN lines around one kill
              the map thins out over time (fewer than 49 standing)      -> a refill is being
                         dropped; paste world-ch0.log
              two mobs on one point, or more than 49                    -> paste world-ch0.log
         t) DONE 2026-09-13 - RAIN'S QUIZ: THE CLIENT CONDUCTS IT. The owner's five screenshots
            plus world-ch0.log settled it without the watch run: for 41 s the client drew the whole
            quiz - offer, question, "that's correct" - with ZERO inbound quest/script packets,
            then sent the turn-in, which the server answered by asking the question AGAIN. The
            client grades the quiz from its own Quest.wz and sends the turn-in only on a right
            answer, so the server now records the completion and says nothing. NOT YET SEEN
            on screen: a turn-in answered by the record alone (the silent accept is the
            precedent). Take Rain's next question, answer right, press OK:
              nothing else appears, exp lands at once, journal shows it done -> fixed
              the same question again                        -> paste the turn-in lines
              the UI freezes after the OK                    -> the record alone was not an
                         answer; the fix becomes a closing Say instead of silence
         u) NEW 2026-09-13 - SUMMONING A PET. "I tried summoning the Husky pet, but the pet
            does not come out." The double-click was 0x0147 (u32 tick, u16 Cash-tab slot),
            unanswered. Now it is answered with 0x0277 PetActivated, read off the client's
            own pet decoder (CPet::Init - itemId, name, serial, x, y, moveAction, foothold,
            then six tail fields whose MEANINGS are the reference's), the Cash-tab item is
            re-sent as active with a pairing serial, and the request is closed. A second
            double-click puts it away; a map change brings it back. Movement is the
            client's; nothing about it is answered yet. Double-click the Husky:
              it appears beside you and follows/idles        -> the shape is right; say
                         whether the item in the Cash tab draws as summoned, and paste any
                         UNKNOWN inbound opcodes that start arriving (its moves)
              it appears, the item does not look summoned   -> the pairing serial is not
                         what the item holds; say so
              the client dies at the click                   -> paste client-exit.log and
                         the 0x0277 line; one of the six tail fields is wrong
              nothing at all                                 -> paste the lines after 0x0147
              double-click again: it goes away               -> the toggle works
            AND THE VACUUM: see step 8, LOOT - the pet's request is 0x0205 (settled
            2026-09-15); the box is the client's, keyed on wonderGrade 6 via the BOUGHT
            Expanded Auto Move since 2026-09-16 evening.
              the pet never moves toward drops                              -> the keys are
                         not what this pet code reads; say so
              the pet takes your OWN dropped item                           -> the byte is
                         ignored; paste the pet pick-up line
         v) 2026-09-14 - THE HUSKY RENDERS (giantRate is the size in percent; 0 for
            eleven runs, 100 now). CLOSED. research/pet-draw-chain-2026-09-14.md.
            THREE TWO-CLIENT FIXES FROM YOUR NEXT REPORT, one launch for all three -
            two clients, Tester2 as the second:
            (a) THE SECOND CLIENT NOW SEES THE PET. The summon and every walk were
            already published to the map; what nobody covered was the OTHER order -
            pet already out, Tester2 walks in - and Tester2 got the owner's spawn alone.
            A Presence now carries "companions": the pet's 0x0277, posted right after
            the owner's 0x0224 to whoever arrives. With the Husky out, have Tester2
            walk into your map:
              Tester2 sees the Husky at your feet         -> done
              Tester2 sees you and no pet                 -> paste Tester2's world log
                         lines after their 0x00DC: the 0x0277 should follow the 0x0224
              Tester2 sees the pet but at (0,0)/wrong     -> the companion body's
                         position; say where it stands
            (b) MOBS NO LONGER SNAP ON JOIN. The joiner got each mob at its CURRENT
            x,y but with the SPAWN POINT's foothold, so their client placed it and then
            dropped it onto the wrong floor. The end of the controller's last path
            names the floor under it (fifth u16 of a 21-byte element), and that now
            travels with the position. Join a map where the other client has been
            fighting for a while:
              mobs stand where they are, no jump           -> done
              still snap                                    -> say whether they snap
                         sideways (position) or up/down (floor) - different fixes
            (c) THE JOINER APPEARS AT THE PORTAL, LANDING, NOT AT THE ORIGIN. The
            0x0224 the field is told used to stand at (0,0) until the newcomer's first
            step - the "snap". portals.txt now carries each portal's x,y (regenerated
            with tools/dump_portals.py - the server banner warns if the columns are
            missing) and the announcement stands there in action 4, the jump pose, on
            the foothold under it. Watch the other client walk in:
              appears at the doorway in a landing pose, then walks -> done
              appears at the doorway STANDING (not the pose)      -> the pose byte is
                         wrong; it is 8 (action 4 << 1) and I will re-derive it
              still appears at the origin                          -> paste the
                         server banner's portals line and the 0x0224 body
            STILL OPEN, same launch: THE PORTAL CRASH (was the old 0%-scaled pet: repeat
            pet out, Tester2 in 10001010, walk west00 - no crash closes it; a crash
            means paste client-exit.log tail + dump name) and PICK-UP (kill a mob near
            the Husky; takes the mob drop with no click -> works; walks to it and
            nothing happens -> paste the inbound lines after the walk-over).
         e) NEW 2026-09-12 - FRIEREN ASKS WHICH VERSION. Nexon ships Frieren's set as
            normal / Ringlets / Sleep (nexon.com/maplestory/news/sale/44291), so opening the
            Frieren Outfit Set Coupon (the Cash Shop's / the Collection's, 5681543) now
            opens a three-row MENU from the Administrator, each row with that version's
            hair icon. The coupon is spent by the CHOICE, not by opening; End Chat keeps it.
            Where the page says "your choice of" (Clothes / Winter Clothes) you get BOTH.
              normal   -> Hair, Face, Clothes + Winter Clothes, Shoes, Earrings, Staff (7)
              Ringlets -> Hair (Ringlets), Face, the same five (7)
              Sleep    -> Hair (Sleep), Face, Sleep Clothes, Earrings (4)
            Then the receipt (d) lists exactly those. Open the Frieren coupon:
              menu with three rows and hair icons; pick one; those items + receipt -> done
              menu opens but End Chat spends the coupon -> tell me (it must not)
              no menu, the items arrive as before -> the coupon id differs; paste the
                         world-ch0.log line for 0x0114
              rows show no icons / wrong names -> say which
         c) HAIR / FACE COUPON: REDRAWN IN PLACE BY 0x007C (2026-09-18). The owner: "the player
            needs to enter a different map to see the hair or face updated" - so the dialog
            only previews; the "client self-applies" reading of 2026-09-12 came back false,
            as this step's last line said it could. The Confirm is now answered with ONE
            StatChanged carrying the HAIR or FACE bit and the id (and the unlock byte); the
            client's 0x007C handler runs the same two avatar calls the equip handler does
            (FUN_142ce51b0 x2 then FUN_142ce5e60, at 142d560d5 / 142d56122 [L]), which is
            the redraw every equip on screen goes through. !hair 42540 is the cheap form.
              hair/face changes THE MOMENT you press Confirm, no reload, coupon gone
                         -> CONFIRMED 2026-09-18 14:51 (!hair 42540, two clients side by side)
              the hair changes but the character flickers / a CLIENT FAULT -> paste
                         client-exit.log; the pair ran on an avatar mid-animation
            ANOTHER PLAYER'S CHARACTER INFO (2026-09-18, the owner: "When double clicking another
            player, a similar Character Info window should show ... I just tried double
            clicking on Tester2"). The click sent 0x01FC (u32 tick, u32 id, str "", u8
            petInfo) and nothing answered - and BOTH its builders set the shared request
            latch, so an unanswered click also froze ~35 other request senders until the
            next map change. The reply is 0x00A2 (found from the double-click handler's own
            window singleton back to the dispatcher case that clears the latch; the opcode
            table had it wrong in both columns): result, id, name, level, job, fame 0,
            guild "", the pet out (item id, name, level, closeness, fullness, the pet item)
            or zeros, no ITEM/CITIZENSHIP rows, and petInfo echoed. Every request is
            answered; an unknown character gets the 4-byte refusal.
            research/character-info-2026-09-18.md. NEVER ON A SCREEN. Two clients:
              double-click Tester2 as the owner: the Character Info window opens with Tester2's
                         name, level, job, FAME 0, GUILD -, and (if a pet is out) the pet
                         panel's TYPE/LEVEL/CLOSENESS/FULLNESS matching Tester2's own
                         -> DONE; say whether Tester2's avatar is drawn in it
              the window opens but the avatar box is blank -> the pool lookup by id missed;
                         say so, that is a separate variant
              nothing opens, and the log has the 0x00A2 line -> one of the two pre-open
                         gates (research 2.1) refused; paste the 0x01FC and 0x00A2 lines
              nothing opens and NO 0x00A2 line -> the arm is not reached; paste the 0x01FC
                         line and whatever follows it
              the window opens but a later inventory move is refused -> the reply did not
                         clear the latch; paste the 0x00A2 line
            FAME, AND THE ITEM LIST (2026-09-18 evening, the owner: "attempted to fame them";
            "once per day (reset at midnight UTC) ... not the same character twice in a
            week, resets on Monday midnight UTC"; "the full Item List of everything they
            are wearing"). The click sends 0x0144 (u32 target, u8 up) and got nothing. The
            reply is 0x0087 - the one handler that references every fame message string -
            mode 0 (name, up/down, new fame) to the giver, mode 5 (giver's name, up/down)
            plus a fame StatChanged to the target; mode 3 = "not anymore for today", mode 4
            = "not that character this month" (the client's word; the rule is a WEEK). The
            fame is a real column now (it was a literal 0). The ITEM tab lists the HAIR, the
            FACE, then every worn equip and cash cover, as whole equip slots. A hair or face
            is an equip to the client's icon lookup (Character/Hair/000300xx.img), but the
            classic data had no info/icon on those images - so backport_install.py now
            RENDERS one per hair and face (1312 + 540, from the part's own default frame;
            build/look-icons/sheet.png shows eight) into the hybrid Hair and Face archives.
            THAT INSTALL MUST BE ON THE CLIENT before the window is opened, or the two look
            entries ask the widget for an icon that is not there: run
            "python C:\MapleCW\tools\backport_install.py --install"
            with the client closed (the other session does this with the rebuild), or launch
            with -NoLookItems for equips only.
            THE APOSTROPHES - CONFIRMED 2026-09-18 evening ("The fame message works fine
            now"): the hook rewrites the four templates in place (fametext.rs); off switch
            -NoFameTextPatch. Struck.
              the arrow again (either way, anyone): "can't ... anymore for today" -> DONE
              Tester2 fames the owner back: allowed (per giver) -> DONE
              THE PET'S CELL - CONFIRMED 2026-09-18 evening ("The top hat now shows"): the
              cell is the pet's equip from its worn row, scrolled stats included. Struck.
              Item List panel: Tester2's HAIR, FACE, then hat, coat, weapon, cash cover,
                         each with an icon -> DONE. Hover the hair: a tooltip with its
                         name -> the String.wz lookup works for a look id too
                         MEASURED 2026-09-18 evening: the list draws, fame 1 draws, the
                         tooltips work - and Fern's Staff and Fern Hair were BLANK cells.
                         Two causes, both installed since: the six modern weapon covers
                         ship iconRaw and no info/icon (the list reads icon, the tooltip
                         iconRaw - which is why the tooltip drew), so the installer now
                         copies iconRaw to icon; and the cell draws nothing for an icon
                         over its box (Fern Hair 46x56 blank, Fern Face 27x17 fine), so
                         every hair/face icon and cover icon is fitted into 32x32.
                         Fern's Staff and Fern Hair cells drawn -> DONE; still blank ->
                         say which, and whether a classic hair (38x22 -> fitted) draws
                         SECOND MEASUREMENT (19:30 run): the staff drew; Fern Hair's cell
                         STILL blank; Fern Face drew in the list but its tooltip showed a
                         garbled image block. All 778 classic equip icon canvases are
                         format 1 (BGRA4444) and ours were format 2 (8888) - a garbled
                         block is 8888 read two bytes a pixel - so every synthesised icon
                         is BGRA4444 now (installed 15:36). Size was never it: classic
                         icons run to 34x34.
                         THIRD MEASUREMENT (the owner, three screenshots): classic hair and
                         face draw in the list; a classic face's tooltip image sits at the
                         BOTTOM-LEFT of the preview frame, half outside; Fern Face's
                         tooltip shows only a grey smear; Fern Hair blank everywhere. Two
                         causes, both read off the client, both fixed, neither a guess:
                         (1) CItemInfo's node getter (FUN_14039e630) takes the equip path
                             only for ids 1xxxxxx or id/10000 <= 3 - classic faces 2xxxx,
                             hairs 3xxxx. A backported hair is 4xxxx, so every icon and
                             tooltip lookup went to the Item trees and found nothing; the
                             avatar and the name take other paths. The owner: "can we just make
                             changes to the display logic instead?" - so the hook now
                             patches that one byte, 3 -> 6 (the path builder itself maps
                             id/10000 of 3, 4, 6 to Character/Hair). grap-stub lookgate.rs,
                             "LOOKGATE: patched" in maplecw-hook.log; lookgate=off undoes it.
                         (2) the tooltip anchors the image on the canvas ORIGIN and expects
                             a cap-shaped icon (~30 tall, origin at its bottom row); a
                             16-tall face with origin 16 landed 16 px too low. Every
                             synthesised icon is a full 32x32 canvas now, art centred,
                             origin (-2, 32) - a cap's shape. Installed, read back, --check.
                         FOURTH LOOK (two screenshots): Fern Hair's cell draws (the gate
                         patch held); the tooltip image is STILL low-left, enlarged, for a
                         classic face and Fern Face alike, and the 32x32 padding moved
                         nothing. The tooltip's draw is a call into the graphics engine
                         with the canvas the engine built from the PIXEL node - for a real
                         icon that is the originless node in the _Canvas archive (the stub
                         in the property image holds the origin and an _outlink); ours
                         were inline, so the engine's canvas carried origin (-2, 32) and
                         the picture landed 32 px low. Every synthesised icon is now the
                         real shape: a 1x1 stub with origin + _outlink in the property
                         image, pixels without origin in _Canvas (a colour variant gets a
                         new _Canvas image of its own). The weapon covers' icon is a stub
                         onto their iconRaw pixels. Installed 16:1x, read back, --check.
                         Fern Hair's cell drawn, its tooltip image inside the frame, the
                         face tooltips inside the frame -> DONE
                         still low-left -> the engine reads the STUB's origin after all;
                         say so and the next variant is origin (0, 0) on the stub
                         Fern Hair still blank + no "LOOKGATE: patched" line -> the hook
                         did not arm; paste the LOOKGATE lines from maplecw-hook.log
                         images inside the frame but a grey smear beside them -> say so;
                         that is the tooltip's own shadow of the icon, or it is not
                         hair/face cells BLANK, equips fine -> the icons are not installed,
                         or the widget wants another node; say which
                         the client DIES on opening -> a look id in an equip slot is fatal;
                         relaunch with -NoLookItems and paste client-exit.log
                         blank cells everywhere or a death with -NoLookItems -> the widget
                         rejected an equip body; paste the 0x00A2 line and client-exit.log
              nothing at all on the click -> the 0x0144 arm not reached; paste the line
            THE OTHER CLIENTS: IN PLACE BY 0x02AE - CONFIRMED 2026-09-18 evening ("In place
            look update is fine now"): no blink, the pet stays. The third attempt, and the
            one that was read rather than guessed (research/remote-redress-2026-09-18.md);
            -LookReenter is the leave + enter fallback, not needed. Struck.

     TH. THE FACE COUPON opens no dialog and has no tooltip preview. The face's images are
         installed and structurally identical to a classic face (checked node by node); the
         one difference left is the ID: the classic client's faces are 20000..21825 and the
         backport's are 22035..22042, while hair 42600 (also outside the classic 30000..31807)
         works. So the client's FACE path checks the id and the HAIR path does not. [I]
         Test that costs one chat line: `!face 22039`. If the face draws in the field, the
         id works once applied and only the coupon UI refuses it (then a server-side apply
         on double-click is the fix); if it does not draw either, the backport must renumber
         the faces into the classic 20xxx/21xxx space.

     TF. THE FREE MARKET DOOR, which is the one that can strand somebody if it is wrong.
         From **Henesys Market** (10001040) walk into the `market00` portal, then walk back
         out of the Free Market Entrance's `out00`.
           you land back in Henesys Market -> the memory works
           you land somewhere else -> say WHERE. Henesys is also the FALLBACK, so do the
                         second half below before believing the first
           nothing happens on either -> the portal is still dead; grep world-ch0.log for
                         "free market"
         **Then the half that tells the two apart**: do it again from **El Nath Market**
         (20001010, `!map 20001010`). El Nath is NOT the fallback, so landing there proves
         the town is remembered rather than hard-coded.
           back to El Nath -> done, and the fallback is not being used
           back to HENESYS -> the memory is not being read; it is falling through
         And the one that costs a rescue if it is broken: enter the Free Market, **log out
         and back in**, then leave. The memory is in the database, so it should still work.
           back to your town -> it survives a relog
           back to Henesys from El Nath -> the row is not persisting

     TC. THE COUPON PURCHASE - and the capture it used to ask for is DONE.
         (The old TC asked for one 0x0114 capture. You made it on 2026-09-09 and the
         cash-item opcode has been handled since. Struck.)
         The owner, 2026-09-10: "I just tried purchasing 5x Etc Tab 5-Slot Coupon, but I was
         met with I did not have enough leaf points. I absolutely do." You did: 105,500.
         The log has the real reason - the locker table on your file was missing the
         `failed_slots` column the shared item reader had gained, every locker read failed,
         and the handler reported ANY store error as "not enough cash". Both fixed: the
         column is added on open, and a server error now says "unknown error" instead of
         blaming your wallet. Storage had the same missing column and is fixed alongside.
         The coupon goes into the CASH INVENTORY (the shop's own panel), never storage;
         the storage line below is a separate check of the second repaired table.
         a) Buy ONE Etc Tab 5-Slot Coupon (100 LP).
           it lands in Cash Inventory and LP drops by 100 -> fixed
           "unknown error" -> a different server error; the world-ch0.log line names it
           "not enough leaf points" again -> the wallet read is wrong; say your LP
         b) MOVE it from Cash Inventory into your bag's Cash tab (the move button or a
            double-click in the panel). NEW 2026-09-10: this was refused by the server
            until now - no archived run ever had one - so it is unseen on a screen.
           it appears in the Cash tab at the slot you picked -> 0x0A/0x19 work
           "unknown error" and it stays put -> world-ch0.log names which check refused it
           it vanishes from the panel but is NOT in the Cash tab -> the 0x19 body is
                         wrong; say which tab you were looking at
         c) LEAVE the shop and come back with something still in Cash Inventory.
           it is still listed -> the entry listing works (new today; unmeasured)
           the panel is empty -> the client ignores 0x0C at entry; the row is in the DB
         d) Double-click the coupon in the Cash tab -> the Etc tab widens by 5 on screen.
         e) Separately: open the STORAGE keeper. It opens with its slots -> the second
            table is fixed as well.

     TD. THE FOUR THINGS FIXED AFTER THE LAST RUN. All four came out of that run, and
         none has been seen working. Quick, and they need no setup beyond a bag.

         a) REAL SCROLLING, the client's own window - drag a scroll onto an equip.
            `0x0125` was decoded in full and NEVER HANDLED; your four attempts got the
            latch unlock and nothing else. CONFIRMED WORKING 2026-09-18 (the shirt took
            +70 HP). WHAT WAS WRONG THAT DAY: "When I change maps, those scrolled stats
            disappear ... in the inventory ... they come back" - the SetField record's
            dresser asked the item TEMPLATE for every worn item (a doc block from before
            the store kept per-item stats); the bag restore never did. It reads the worn
            row's own stats first now (session/inventory.rs dressed).
              scroll a WORN shirt, change maps: the tooltip keeps the +HP and the spent
                         enhancement -> DONE; template numbers come back -> paste the
                         SetField line and the "scroll" line before it
            **`!item 2040400 3`** - a topwear DEF scroll, 100% success and cursed 0, so it
            cannot destroy anything. Use THAT one first.
              the stats change and a sound plays -> the whole path works
              "cannot be used here" -> a refusal, and world-ch0.log names which one
              nothing at all -> grep world-ch0.log for 0x0236. If it went out, the client
                            did not draw it; if it did not, the handler refused early
            THEN, deliberately: **`!item 2040403 3`** is the same scroll at 10% success and
            **cursed 50** - it DESTROYS the shirt on half its failures. That arm has never
            run, and a destroy has to remove the item from the screen as well as say so.
            Wear something you do not want before trying it.

         b) PARTIAL DROP - drag 2 out of a stack of 5 onto the ground.
              2 on the floor and 3 still in the slot -> mode 1 works
              the whole slot empties on screen -> the mode is wrong, and the store will
                            disagree with the screen. Say so, do not close the client

         c) THE MESO CAP. The client's own box stops you at 10000; the server now
            refuses above it too. Nothing to see unless it misfires:
              dropping exactly 10000 works -> the boundary is right
              10000 refused -> off by one, and the chat line says the cap

         d) THE LADDER. Climb a ladder, drop mesos AND an item from part way up.
              they fall from you down to the floor -> fixed
              they appear on the floor with no fall -> the arc is still zero-length
              they hang in the air at your feet -> the snap was lost, and they will not
                            be pickable. That is the worse failure of the two

     TS. !scroll - REBUILT 2026-09-09 AND NOT SEEN ON A SCREEN SINCE.
         Run as `maplecw`. Five questions, each with its readings written down.

         a) THE EMPTY-HANDED SCREEN - **DO THIS FIRST, BEFORE YOU `!item` ANYTHING**, because
            granting yourself a scroll destroys the state it tests.
            Just `!scroll`, carrying neither.
              two rows, each an ICON + the item's NAME + what it does, then the drop rate
                            -> `#i` and `#t` both resolve in a say box. `#t` is the client's
                               OWN String.wz lookup, so the names being right proves the link
                               resolved rather than us having typed them
              the raw text `#t4031065#` on screen -> `#t` is not honoured here. Fall back to
                            our own names, which `Scroll::name` already carries
              no icons, names fine -> `#i` needs a menu context. Keep the names, drop the icons

         b) THE STACK. `!item 4031065 5`, then `!item 4031066 5`.
            **THE SERVER HALF IS ALREADY PROVEN AND IS NOT WHAT THIS TESTS.** A test drives
            the exact path `!item` uses and gets five in one slot, a second five merging into
            it, and 105 spilling correctly to 100 + 5. So the only thing left in question is
            what the CLIENT does with a quantity above its own `info/slotMax`, which is 1 for
            both of these.
              one slot, count drawn -> the client honours a server quantity over its own
                            slotMax. The question is closed. Then try to DRAG THE STACK APART:
                            a split goes through the client's own UI rule and is undecoded, so
                            that is a second, separate answer
              five slots -> the client clamps to slotMax on receipt. Since the server is proven
                            to send one stack, this is definitively the client - the same wall
                            Event Trophy (slotMax 0) hit. Say so and we swap both ids for two
                            of the 161 Etc items that already carry slotMax 200
              the client DIES -> a quantity above slotMax is not merely ignored, it faults.
                            Grab client-exit.log and maplecw-hook.log; that is the outcome
                            that decides it fastest and it is worth knowing

         c) THE MENU ROW. Every row is now `#i<itemId>#` icon + name on ONE line.
              icon draws, rows do not overlap -> the clip is fixed AND an icon inside a #L
                            region works. That second half is unattested: ZERO of this
                            client's own 33 #L menus put an icon in one
              rows overlap again -> the icon is what breaks the row height, not the text
                            after #l. Drop the icon, keep the one-line rows
              no icon, just a gap -> #i is not honoured inside a #L. Same fix

         d) THE SOUND. 0x0236 now goes to the whole map on every success and failure.
              a sound and a flash on YOUR screen -> the packet lands and the effect is real
              nothing at all -> the body or the dispatcher is wrong. world-ch0.log will show
                            0x0236 going out either way, so an empty screen is a CLIENT
                            result, not a server one
              second client sees it too -> the map broadcast works. Worth one look if T1 is
                            already running two clients

         e) THE TREASURE SCROLL - NEVER RUN, NOT ONCE. `!item 2043200 2` (a One-Handed Blunt
            Weapon scroll) while wearing a One-Handed Blunt Weapon, then `!scroll` ->
            Treasure Scroll -> the weapon -> the list of your own scrolls.
              the list shows ONLY scrolls that fit that weapon -> the category rule holds on
                            live data as well as on the 24 names it was checked against
              the list is empty -> either the scroll is in the Use tab and not being found,
                            or the category arithmetic is wrong. `!item 2040000 1` (a HAT
                            scroll) must NOT appear in the list; if it does, the filter is
                            inverted
              it applies and the tooltip gains the scroll's own stats -> done

         And the standing one: an `!` command answered by a CHAT BALLOON is the GM gate,
         not a broken command.

     T1. TWO CLIENTS, SAME MAP. Do this first; T2 waits on it.

          FOUR AGENTS WENT OVER THIS ON 2026-08-31 AND THE PREDICTION HAS CHANGED.
          The old (b) read "a client dies when the second arrives -> the 0x0224 body is
          wrong". That WAS wrong, it has been found and fixed, and the fix moved every
          field after byte 179:

            the remote temporary-stat block is 131 bytes, not 124. The decoder reads
            u8, u8, u32, u8 after the mask, unconditionally. The body was 508 bytes and
            the client wanted 515, so it would have consumed everything and then thrown
            with one byte left. The mask length was never wrong - a mask is not a block,
            which is the same mistake net/buff.rs already records one decoder over, where
            it cost a client death ("Nimble Feet crashed the client").

          So this run is no longer expected to die. What it settles:
            a) the other character appears, dressed, and walks when they walk
                       -> the whole user pool works. THE result of this run
            b) a client STILL dies when the second arrives -> the body is wrong somewhere
                       ELSE. Say WHICH client died - the arriving one or the one already
                       there - because that names the direction. world-ch0.log has the length
                       we sent; the hook log will have NO dispatch line for 0x0224, since
                       that line is written on return
            c) nothing appears and nothing dies -> the packet was DROPPED, not misread,
                       and no more body work will help. Six gates in front of the insert
                       can do that; the discriminator is a watch on 0x1429ba60b, the
                       allocation past all six. research/user-enter-verification.md
            d) they appear standing at the map ORIGIN and stay there until they move
                       -> expected and self-healing. The server has no position for a
                       player who has not moved yet
            e) they appear at the origin and STAY there while walking -> the 0x0293
                       rebroadcast is not arriving; that is a different packet

     T2. KILL ONE MOB TOGETHER, both of you hitting it.

          THIS USED TO SAY "EXPECT IT TO LOOK BROKEN". It should now look right, and
          every line below is a claim that has only ever been proved by the test suite -
          two clients have never been connected to this server at once.

          a) BEFORE HITTING ANYTHING, just stand and watch the mobs on both screens.
             They should be in THE SAME PLACES and walking THE SAME WAY. One client is
             granted control of each mob and its moves are rebroadcast to the other.
               the two screens agree           -> 0x03D9 is arriving
               they disagree / mobs drift      -> the rebroadcast is not landing
               ONE screen's mobs are frozen    -> that client was granted nothing
          b) NOW BOTH HIT ONE MOB.
               the HP bar moves on BOTH screens for EITHER player's hit  -> 0x03F0
               it dies on both screens                                  -> 0x03D1
               only your own hits move the bar -> the publish is not reaching the map
          c) THE DROP IS DELIBERATELY NOT SHARED. Whoever dealt the most damage - not
             the killer - is the only one who sees it. Over-damage does not count, so a
             500-damage finisher on a snail with 3 HP left is credited 3.
               only the top damager sees the item  -> correct, and intended
               BOTH see it                         -> the drop went map-wide, a real bug
               NEITHER sees it                      -> the ranking picked a departed client
             (Parties would share drops. THERE IS NO PARTY SYSTEM YET - the window's
              buttons are answered with a refusal so the UI cannot freeze, nothing more.)
          d) THE EXP LINE. White for the majority contributor, yellow and smaller for
             the other.
               only the killer is paid  -> the fact never crossed the bus
               both lines white         -> the majority flag is wrong
               the helper is paid IN FULL -> the split is not being applied

     T2b. NOW ONE OF YOU LEAVES THE MAP - a portal, or just close the client.
          THE OTHER SCREEN IS THE MEASUREMENT. Watch the mobs on the player who STAYS.

          Control of every mob the leaver was driving is handed to whoever is left, and
          that client is told without having to move. Before 2026-09-01 nothing was sent
          at all, and the monsters stood still until somebody walked through a portal
          and back.
            they carry on walking, no pause, no jump   -> the handover works
            they all FREEZE and stay frozen            -> nothing was handed over
            they freeze until you walk a portal        -> the old behaviour is back
            they JUMP to their spawn points            -> the grant sent the spawn
                       position instead of where the mob is standing
          Closing the client is the better half of this test: it is the exit that goes
          through no log out, and the one the leaving player cannot see.

          world-ch0.log discriminates all of this without a second launch - grep it for
          "mob control:", which names the count and the recipient on every handover.
     T11. THE THIRD JOB ADVANCEMENT. Set yourself up first:
              !job 110   !exp 31545355   !map 20001000
          That is a level-70 Fighter in El Nath. 31 545 355 is the exp curve summed 1 to
          70 and one !exp crosses every level in it. If the level comes out wrong, say what
          it actually was - the curve is ours and that would be a finding of its own.
          THERE IS NO FERRY (the owner, 2026-09-29: "El Nath should only be accessible by foot or
          teleport scroll"). A player gets here by the ship to Orbis and the Orbis Tower.

          a) You arrive in a snowy town -> El Nath loads. Say so
               black screen, or the client dies -> THE finding of this run. world-ch0.log's
                          SetField line names the map; say whether the screen drew
                          anything first
          b) CLICK EUREK THE ALCHEMIST (they stand here and in Sleepywood): their own line
             about wandering the world, NO menu -> DONE
               a menu of stops -> old build; the ferry is back
          c) WALK RIGHT and take the door into Chief's Residence. Four NPCs are inside:
             Tylus, Robeira, Rene and Arec.
               all four visible -> the NPC list crossed the continent too
          d) CLICK TYLUS. They serve Fighters, Pages and Spearmen.
               "You are a Crusader now" and the job changes -> DONE
               open the skill window: there should now be a THIRD page with points on it
                          no third page -> the SP pool key is wrong. The job still
                          changed, so say both halves
               "Come back when you have reached Level 70" -> the !exp did not land
               nothing happens -> the click never reached third_advancement_for
          e) CLICK ROBEIRA, RENE OR AREC as the same character. They must all REFUSE
             and name the branch rather than the level. A Crusader they will refuse too,
             because the advancement is one-way.
          f) Orbis Ticketing Booth's Platform Usher: ONE line, the platform to Victoria
             Island, no ferry line -> DONE

          FOR THE RECORD: there is NO third-job test in this client - no quest, no hidden
          field, no marbles. All 322 quests were enumerated. Level 70 and the right second
          job is the whole gate, which is what you chose. If a level-69 character advances,
          that is a bug.

     T10. THE SECOND JOB ADVANCEMENT. ALSO NEW. Set yourself up first:
              !job 100   !item 1302000   !exp 548637   !map 10004023
          548 637 is the exp curve summed from level 1 to 30 and one !exp crosses as many
          levels as it is worth, so that is a single command. THERE IS NO !level - it was
          in an earlier draft of this plan and it does not exist.
          **Then put your ability points into STR in the stat window.** !job does not move
          them, so a fresh level-30 Swordsman still has beginner stats, and the mobs in
          there hit for ~204 with 718-789 HP.
          That puts a level-30 Swordsman on West Rocky Mountain IV, where the Warrior Job
          Instructor (the EXAMINER, template 514) stands. Then, in order:

          a) CLICK THE EXAMINER. You should get a line and then find yourself somewhere
             else - Warrior's Rocky Mountain, map 80001300, full of Fire Boars and Lupins.
               you arrive -> the warp works and the map loads. THIS IS THE STEP THAT
                          MATTERS; no character has ever been on one of these four maps
               the screen goes black / the client dies -> say WHICH, and world-ch0.log's
                          SetField line names the map. That map has 132 footholds and 30
                          mobs, so a load failure is a real finding
               nothing happens -> the click never reached job_test_for. Say so; the
                          examiner also carries quests and the client may have sent
                          0x0151 instead of 0x00F2, which is a routing question

          b) KILL ANYTHING IN THERE. Every mob drops one Dark Marble, guaranteed.
               a marble per kill -> the drop rule works
               no marble -> the map gate. world-ch0.log's drop line names the map it used
               marbles from OTHER mobs elsewhere later -> the leak the gate exists to
                          stop; say where you were

          c) CLICK THE NPC INSIDE (it is called Warrior Job Instructor too - a different
             NPC from the one outside, template 800006). It is the ONLY way out.
               you land back on West Rocky Mountain IV -> the door works
               nothing happens -> YOU ARE STUCK. Use a return scroll or !map to get out
                          and say so loudly; that is the worst failure in this run

          d) COLLECT 30 MARBLES and click the examiner outside again. Faster: !item
             4031017 30. They should take all 30 and hand back The Proof of a Hero.
               30 leave the bag and the proof arrives -> the exchange works
               they warp you back in instead -> they counted fewer than 30. Say how many
                          the Etc tab showed

          e) !map 10004003 and CLICK DANCES WITH BALROG holding the proof.
               a box listing Fighter / Page / Spearman -> pick one
               "You carry no proof" -> the proof did not survive the trip. Check the bag
             Then, after picking:
               the job name changes and the skill window has a SECOND page -> DONE
               job changes but the skill page is empty -> the SP pool key is wrong
               nothing happens -> the menu answer never routed

          FOR THE RECORD: without the proof, Dances with Balrog must REFUSE. If a level-30
          Swordsman can advance without ever entering the map, the whole chain is optional
          and that is a bug worth reporting even though it looks like a feature working.
     T0. CAN THIS MACHINE RUN TWO CLIENTS AT ONCE? T1 and T2 are impossible until it
         can, so do it first.

         **LAUNCH THEM ONE AT A TIME.** Wait for the first client to reach the CHARACTER
         LIST before pressing Login in the second launcher. This is not politeness:
         `client-patched\maplecw-hook.identity` is ONE shared file. Both launchers write
         it, and the hook reads and deletes it several seconds into the client's life
         (`crates/grap-stub/src/hook.rs` arms it well after start-up). Overlapping
         launches mean the second launcher's token is on disk when the FIRST client's
         hook goes looking - so client 1 carries client 2's credential, both connections
         resolve to the same claim, and **both clients show the same account.** That is
         indistinguishable on screen from the login-claim bug that was fixed on
         2026-08-29, and it would be reported as a regression that is not there.

         **TWO ACCOUNTS ARE NOT REQUIRED, and are still what to use.** The claim table is
         keyed per LAUNCH - one row per sign-in, keyed by the SHA-256 of that sign-in's
         session token, and `Store::authenticate` mints a fresh token every time - so two
         sign-ins on `maplecw` make two rows and neither evicts the other. What separates
         them is the owning PID (rule 2), which is per-process and knows nothing about
         accounts. But with one account both clients legitimately show the SAME character
         list, which deletes the only check that can catch the claim bug coming back; and
         nothing in this server stops the same character being claimed twice. So: maplecw
         and tester.

         **THE OLD VERSION OF THIS STEP WAS WRONG AND COST WISP A TRY.** It said to
         double-click client-patched\MapleStory.exe with no server running. The client
         does not start that way: `crates/launcher/src/prepare.rs` launches it as
         `-NXLDEBUG <ip> <port>`, and with no arguments at all it exits immediately.
         So there is no no-server version of this test - the only argument the client
         takes is the address of a server.

         Do this instead. Start the servers ONCE, then run the launcher TWICE:
             powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -ServersOnly
             (or just double-click start-servers.cmd, which passes exactly that plus
              -PoolSentry. -SetFieldProbe used to be mandatory here and is now the
              default; this line carried -ServersOnly alone until 2026-09-09 and cost
              The owner a launch, which is the reason the default moved.)
             C:\MapleCW\target\release\maplecw-launcher.exe      <- sign in as maplecw
             C:\MapleCW\target\release\maplecw-launcher.exe      <- sign in as tester
          THE STUB IS NOW INSTRUMENTED FOR THIS. The launcher writes
          maplecw-hook.multiclient, and grap64.dll - which MapleStory.exe imports
          STATICALLY, so our DllMain runs before the client's own startup - hooks
          FindWindowW/A and CreateMutexW/A. Every call is logged to
          client-patched\maplecw-hook.log. The FIRST client only LOGS: it works out
          its own ordinal from its own mutex, so a single-client run behaves exactly
          as it did before. A SECOND client gets the guard suppressed - FindWindow
          returns NULL and CreateMutex's ERROR_ALREADY_EXISTS is cleared.

          So read the hook log afterwards. It answers this even when the screen cannot:
          two clients reach the character list -> T1 and T2 are possible. Say so, and
                     say WHICH instance: line fired - that names the guard
          two clients, and NO instance: line logged a call at all -> there was no
                     guard; the launcher was the only thing stopping it
          no second client, but the log HAS instance: lines from a SECOND pid ->
                     the guard is one of these and suppressing it was not enough.
                     The log says what it asked for, which is what the next attempt
                     needs
          no second client and NO second pid in the log -> our DllMain never ran
                     there, nothing in-process can help, and the answer really is a
                     second machine
          a line saying a prologue "will not steal" -> that API was NOT hooked and
                     nothing was written. Its first 16 bytes are in the log; paste
                     them and the next build can hook it
          the second LAUNCHER fails or hangs -> not the client's fault. Two launchers
                     patch the same files in client-patched\; say which one failed and
                     whether the first client was still running
          both clients show the SAME account -> the per-launch claim broke. That was
                     fixed and proved over sockets (tools/claims_smoke.py, two accounts
                     on 127.0.0.1), so it would be a regression worth stopping for

         Nobody has ever tried this. 211 connections across 188 archived logs contain
         ZERO overlapping pairs, and no archived instrument could have seen a second
         process that dies before it connects.

     T9. THE COLLECTION TOOLTIP, and it carries its own control. Walk one portal with
         "Vicious in Need of an Apprentice" accepted, then OPEN THE INVENTORY.
         Only the **Etc** tab is restored with the quiet mode 5; Use, Set Up and Cash
         still use the ordinary ADD, so they are the control in the same run.
          no tooltip, and the Etc tab still has everything
                     -> mode 5 works. Roll it out to the other three tabs
          no tooltip, but the Etc tab is EMPTY while the others are full
                     -> mode 5 stores nothing the UI can see. THIS IS THE FAILURE THAT
                        MATTERS. Say so and it is reverted; the fallback is carrying the
                        Etc bag in the character record instead, which is silent by
                        construction rather than by experiment
          the tooltip still pops
                     -> the chain is wrong somewhere. world-ch0.log will show the mode-5
                        bodies went out, so the next question is whether the store call
                        alone still trips the quest check
          ALL FOUR tabs empty
                     -> not mode 5; something else broke. Revert and say so
         Nobody has ever sent a mode 5 on this wire, which is why the other three tabs
         were deliberately left alone.

     T8. THE POOL SENTRY, if you pass -PoolSentry. It prevents nothing; it WATCHES.
         Play normally for 8-10 minutes - the expected first catch is ~300 s of in-field
         time, and the archive has ZERO heap deaths under 192 s. Then read the hook log:

           FINDING #1 at 250-400 s      the catch. The slot's contents are <=106 ms old,
                      which is the whole point: a freshly-written body supports the
                      refcount/flag reading, a long-lived body or a live predecessor
                      supports the +0x24 overrun, and the damaged slot being ON THE FREE
                      LIST kills "the occupant underruns its own buffer" live
           FINDING [EARLY - SUSPECT] under 120 s
                      suspect the instrument first. The rate predicts ~0.34 damaged slots
                      by 100 s. Check the confirm re-reads and the carve identity in the
                      same block before it goes anywhere
           heartbeats, 0 findings past ~600 s in-field
                      a RESULT, not a failure. It contradicts the ~298 s/slot rate, and it
                      is the first UNBIASED sample this project has had - every rate point
                      so far is a death, and a death needs a damaged slot to have been freed
           NOVEL in a finding
                      the biggest result available. 14-of-14-identical is the whole basis
                      for "the writer is selective"; one novel value breaks it
           any `carve FAIL` in a heartbeat
                      the walk is short and every count that run is a lower bound
           no POOL SENTRY line at all
                      the marker did not arrive or the hook did not install - look for
                      `install: hook active` above it

         **Leave -HeapFix off.** It voids the free-list argument the sentry exists to
         exploit, and it patches one of three entry points anyway.

     T7. THE CLIENT CARRIES A CREDENTIAL. Launch through maplecw-launcher, sign in, and
         get as far as the CHARACTER LIST - `0x0073` goes out before it, so nothing
         in-game is needed. Read THREE lines, in this order, because they discriminate:

           launcher pane            client credential written (maplecw-hook.identity, 26 characters)
           maplecw-hook.log         IDENTITY wrote 26 bytes into 0x...+0x1b8 on getter entry #1
           login.log                0x0073 IDENTITY: mode=5 identity length=26 ... then ACCEPTED

         all three say 26 and login.log says ACCEPTED
              -> the client carried a session, through its own cipher, no forged packet.
                 The first non-zero identity in 73 captures. LOGIN socket only; the game
                 socket still carries nothing
         26 / 26 / 26 but REFUSED
              -> the client half works and the MATCH failed. Nothing transforms the bytes
                 in flight, so look at minting and storage, not at the wire
         hook says 26, login.log says length=0
              -> dropped between session+0x1b8 and the encoder. Watch the ctor store at
                 0x142c440ce for a re-zero
         hook says armed but no "IDENTITY wrote"
              -> the getter never ran, so 0x0073 came from a path nobody has found. That
                 contradicts 73 archived observations; re-check the archive, not the code
         no `identity:` lines at all in the hook log
              -> the marker never reached the hook. NOT a protocol result. The hook deletes
                 `client-patched\maplecw-hook.identity` about 1.5 s in, on purpose, so look
                 for it between Start Game and the client window appearing
         `does not begin with the prologue`
              -> the client binary is not the one this was built against. Nothing was
                 patched and the run is an ordinary run

     T5. RETURN SCROLLS. **RUN THIS AS `maplecw`, NOT AS `tester`.**
         `tester` has is_gm = 0, so `!item` comes back as a CHAT BALLOON and no scroll
         is granted - which reads exactly like "the client never sent 0x010E, the whole
         path is dead code" and would be a false negative on the one question T5 exists
         to answer. **A `!` command answered by a chat balloon is the GM gate, not a
         broken command.**
         ANSWERED 2026-09-18: the client NEVER sends 0x010E for a 0203 item. Both of its
         item-use dispatchers route 2030000..2039999 to their own builder, which sends
         0x0123 (same body: tick, slot, item) and sets the request latch. The live server's
         logs had fifty 0x010E and not one scroll; the handler had sat on the wrong opcode
         since 2026-08-29. 0x0123 is wired now (the owner: "get these return scrolls working";
         2030001..7 Victoria only, 2030008..9 Ossyria only - the continent rule the
         handler already had). NEVER ON A SCREEN.
         !item 2030004 2, then !item 2030009 1. Use each from the bag on Victoria Island.
          Henesys: you land in Henesys, the stack reads 1 -> fixed
          El Nath: a chat notice naming Ossyria and Victoria Island, the scroll STILL in
                   the bag, and the NEXT item use still works (the latch cleared) -> fixed
            the scroll vanishes on the refusal -> the transition guard is broken
            nothing at all, and world-ch0.log has no 0x0123 -> the client's own field rule
                       refused it before sending ("cannot use that in this map" on screen)
            nothing at all, and the log HAS the 0x0123 -> paste that line and the reply
            the next potion does nothing after a refusal -> the unlock did not clear
                       0x0123's latch; paste the 0x007C that answered it
          !map 20001000 (El Nath), !item 2030008 1: Orbis works; !item 2030004 1: Henesys
                   refused there -> the Ossyria half
     TS. THE HAIR SALONS (2026-09-18). The owner, third correction: the OWNERS take both style
         coupons and the ASSISTANTS do colour. Henesys Hair Salon (10001044): Natalie
         (owner), Brittany (assistant); Kerning City Hair Salon (10003005): Don Giovanni,
         Andre. The plastic surgeons (Denma, Dr. Feeble) are out. Coupons: 5150100
         Signature Hair (you pick from the salon's REG list), 5150000 Mystery Hair (a
         random VIP style); 5151100 Signature Color (you pick one of 8), 5151000 Mystery
         Hair Color (random of the 8, equal odds). Each salon its own style lists (COT
         rotation), males from the male pool, females from the female; colours are the
         same everywhere. A style change KEEPS your colour; a colour change KEEPS your
         style. An NPC lists the coupons you hold as a menu (one line each, with the item
         icon); with neither, a line links both and names the Cash Shop. Signature opens
         the client's own "pick a look" box (message type 0x0a, decoded from the client,
         net::script::npc_avatar) - NEVER ON A SCREEN; a Mystery line spends the coupon
         on the roll the moment you click it (the line says so).
           1. click Natalie with NO style coupon: a line with BOTH coupons' icons + names
              and "Cash Shop" -> fixed; a bare "no dialogue" line -> paste it
           2. !item 5150100 1, click Natalie: a one-line menu with the coupon's icon. Pick
              it: a window with your character wearing each of 6 styles (male: Metro, Line
              Scratch, Mane, Shaggy Wax, Cabana Boy, Dragon Layered; female: Monica, Miru,
              Angelica, Lori, Rose, Swooshy Ponytail) -> the 0x0a box draws. Pick one:
              your hair changes IN PLACE, in your current colour, the coupon is gone, a
              chat line names the style -> fixed
                the menu shows but the line has no icon -> paste the MENU line
                the window opens but shows a different set/gender -> paste the AVATAR line
                nothing opens, log has "ScriptMessage AVATAR" -> the 0x0a body is wrong;
                          paste the line and, if the client died, client-exit.log
                the style lands in colour 0 (black) though you were not -> hair_ids
                          empty: gm-handbook/beauty.txt missing on the server box
           3. !item 5150000 1 AND hold a 5150100: the menu has TWO lines. Pick the Mystery
              line: one of the 7 (male) / 6 (female) Henesys VIP styles at once, your
              colour, the Mystery coupon gone, the Signature one still there -> fixed
           4. Brittany with no colour coupon: a line linking 5151100 and 5151000 + "Cash
              Shop" (a style coupon in the bag does not count) -> fixed. !item 5151100 1,
              click them, pick the line: a window with YOUR style in 8 colours (Black, Red,
              Orange, Blonde, Green, Blue, Purple, Brown); pick one -> your colour changes,
              the style stays, the line names the colour -> fixed. !item 5151000 1, the
              Mystery line -> a random one of the 8 at once
           5. Kerning City Hair Salon: Don Giovanni's window shows the KERNING REG list
              (male 7: Antagonist .. Shaggy Dragon; female 6: Cutie Hair .. Chantelle) and
              their Mystery line rolls a Kerning VIP style; Andre does colours the same as
              Brittany -> fixed. Henesys's list there -> paste the line
           6. the other client sees the new hair without a map change -> the look
              broadcast; a stale look there -> paste "look change for" lines

     TP. THE PLASTIC SURGERIES (2026-09-18). Same shape as TS. Henesys Plastic Surgery
         (10001043): Denma the Owner = faces, Dr. Feeble = skins; Orbis Plastic Surgery
         (20000031): Franz the Owner = faces, Riza the Assistant = skins. KERNING HAS NO
         PLASTIC SURGERY IN THIS CLIENT (maps.txt has exactly two). Coupons: 5152200
         Signature Face (pick from the REG list, same list in both towns), 5152000 Mystery
         Face (random VIP face); 5153000 Signature Skin Color (pick one of the SEVEN skins
         this client has art for: Light, Tan, Brown, Pale, Ashen, White, Pink). THERE IS NO
         MYSTERY SKIN COUPON ITEM in this client, so the assistant's menu has one line. A
         face change keeps your EYE COLOUR (the hundreds digit).
           1. Denma with no face coupon: both icons + "Cash Shop" -> fixed
           2. !item 5152200 1, click Denma, pick the line: a window with your character in
              each of 7 (male: Dramatic Face .. Look of Wonder) / 6 (female: Babyface Pout,
              Pucker Up Face, Look of Death, Wisdom Glance, Hypnotized Look, Curious Look)
              REG faces -> the box classifies by its first id, 2xxxx = face. Pick one: the
              face changes in place, eye colour kept, coupon gone, line names it -> fixed
                the face lands at eye colour 0 though yours was not -> face_ids empty:
                          gm-handbook/beauty.txt missing on the server box
           3. !item 5152000 1, the Mystery line -> a random VIP face at once
           4. Dr. Feeble with no 5153000: its icon + Cash Shop (the line says "buy it");
              !item 5153000 1, pick the line: a window with your character in 7 skin tones
              -> THE FIRST TEST OF A SKIN BOX: the client reads ids under 24000 as skins
              (FUN_142a91f30), and 0..6 go on the wire as they are. Pick one: skin changes
              in place, one 0x007C with the SKIN bit, coupon gone -> fixed
                the window opens with 7 identical or blank figures -> the ids need +12000;
                          paste the AVATAR line
                nothing opens / the client dies -> paste the line and client-exit.log
           5. Orbis: Franz and Riza do the same -> fixed
           6. the other client sees the new face/skin without a map change -> fixed
           7. EYE COLOUR (2026-10-02). Dr. Feeble (and Riza in Orbis) take the Signature Eye
              Color Coupon 5152100 on a THIRD menu line beside the skin coupon. The owner's
              first try: the box OPENED but Next showed one look. Fixed in 2ab2e4d: the box's
              first u32 is the COUPON id, which the client types the box from (0 made it
              treat colour candidates as styles and rewrite them to your current colour).
              Wear Fern Face 22036, !item 5152100 1, pick the line: Next cycles NINE eye
              colours; pick one -> only the eyes change, coupon gone -> fixed
                Next still shows ONE look -> the coupon type is not what the client reads;
                          next, watch FUN_142a91f30's param_2
           8. HAIR COLOUR, same fix: Brittany with 5151100 on Fern Hair 42570 - Next cycles
              all EIGHT colours and the label changes (Black, Red, ...) -> fixed
                one look only -> as 7
              The collaboration hairs and faces now have REAL colours (db45025: the recoloured
              archives in client-patched, delivered by the launcher once the server is
              repackaged), so the eight hairs and nine eyes must look VISIBLY different, not
              eight copies of one colour under different labels -> fixed
                labels change, pictures do not -> this client has not got the client patch yet
           9. COUPON DEFAULTS (ab39b6a): a collaboration coupon gives the style in its DEFAULT
              colour - Fern Hair coupon -> Violet 42576, Ubel Hair -> Green 42604, Ubel Face ->
              Violet 22639. Use one: the colour on screen is that default -> fixed

     T6. !npcreload. Add a line to data/npc-dialogue.txt while the server is RUNNING, then
         run the command and click that NPC.
          a) does it say the new line without a restart?
          b) the reply names counts - "N replaced, N added". A reload that read nothing
             must say zero, not "ok"

    ============ CARRIED OVER - none of this has changed ============

    THE STEPS. Each is a claim that can come back false; report them separately.
    ORDER: whatever suits. 6 costs no launch of its own and 7 is a question, not a test.

     1. RECOVERY (skill 1001). !learn 1001 3, then cast it.
        NO BUFF ICON WILL APPEAR and that is expected - Recovery has no stat bit anybody
        has identified. The heal is real, the tray is empty, and the cast says so in chat.
          a) six blue +12 numbers, one every five seconds, over thirty seconds?
             the numbers but wrong spacing -> the 5s interval is derived from the tooltip
                        arithmetic, not read from a column. Say the spacing you saw
             one number then nothing -> the tick is not being driven
          b) does it stop on its own after six, or keep going?

     2. !resetap. Put points into STR, HP and MP first, then run it.
          a) STR/DEX/INT/LUK all read 4?
          b) did max HP and max MP come back DOWN, and the points return to the pool?
             the four stats reset but max HP did not -> the HP ledger is not recording
          c) run it a second time. It must refund NOTHING.
             more points the second time -> the counters are not being cleared, which is
                        free stats out of a command whose job is to be safe to repeat

     3. OVERALLS. Wear trousers, then equip a robe (1050000 is a Beige Plain Robe).
          a) do the trousers come off into the bag?
          b) now put the trousers back on. The robe should come off - it is symmetric
          c) the control: equip a plain TOP (1040000) over trousers. The trousers must
             STAY ON. A shirt taking your trousers off is a worse bug than the one fixed

     4. THE GM GATE, as `tester`. Every ! command is gated on the account now.
        A refused command is SAID OUT LOUD rather than answered with a refusal - the owner,
        2026-08-29: "if the ! commands do not work, please make sure that it is sent as a
        normal chat message." To an account that cannot run commands, `!heal` is a person
        typing text, so the game says it.
          a) !heal as tester -> appears as a CHAT BALLOON reading "!heal", and nothing
             happens to your HP
          b) it must NOT print a system notice about GM status - that told a non-GM which
             ! words are real, which is why it went
          c) ordinary chat still works? Type "Hello". A gate one line higher would have
             silenced everybody, and the client draws nothing for its own chat, so a
             dropped line is invisible
          d) then as maplecw: !heal works

     5. THE tester ACCOUNT SEES ITS OWN CHARACTERS - it has none.
          empty character list -> the login claim is being read
          Cobalt is there -> the claim is not, and you are still maplecw. login.log names
                     the account on every connection; that line says which
        Making a character here also exercises the create path on a fresh account, which
        nothing has done since the name check went in.

     6. KEY BINDINGS - the CONTROLLER tab, 2026-09-14. You said: "customization to
        keybindings in the controller settings is not getting saved properly, and when
        clients switch maps their controller settings are completely screwed up." Measured
        in world-ch0.log 00:56-00:58: the byte after the subtype in a CONFIRM names the
        TABLE - 0 for the keyboard, 3 for the controller - and the server threw it away, so
        every controller button you bound was stored as a keyboard scan code; and the
        0x05F1 at each map change sent table 3 as "keep", which the client reads as "keep
        the RESET to keyboard preset 0" (0x1419ffc73 resets all four tables to the keyboard
        const before any gate). Your controller was handed a keyboard layout at every
        SetField. The 53-entry CONFIRM the client then sent is the controller factory table
        XOR keyboard preset 0, including "unbind Q, W, E" - on a controller. Now: the row
        carries its table, 0x05F1 is 1785 bytes with ALL FOUR tables READ (the controller's
        = its own const at 0x143274b20, 22 buttons, + your bindings), and Wisp#215's 22
        stray rows - exactly the controller factory - were scrubbed by a migration verified
        on a copy of the live database. THE ACCEPTED SHAPE: the 1340-byte packet went out
        twice today and the client took it; 1785 is the same shape with one more READ table.
        Log in as the owner, open KEY BINDINGS -> Controller:
          the tab shows the controller DEFAULTS (not keyboard actions on buttons) -> the
                       scrub and the init copy are right
          bind one skill to one button, CONFIRM; walk through a portal; open the tab again:
            the skill is still on that button, everything else default -> DONE
            the skill is gone, defaults shown -> the delta did not store as table 3; paste
                       the "keymap:" log line - it now says WHICH table
            buttons show keyboard actions (menus, Ctrl attack) -> the READ table for the
                       controller is not being taken; say so, STOP
          then the Keyboard tab: Q/W/E/I menus, Ctrl attack, Space jump, nothing on the
                       1..0 row -> the keyboard table is clean of controller buttons
          client exits at field entry -> paste world-ch0.log's 0x009E line; the position
                       byte says which of the four reads failed
        And as Cobalt, once: Power Strike on Ctrl, Slash Blast on Shift after a relog ->
        the keyboard restore, still unconfirmed since 2026-09-12, is confirmed too.

     7. SKILL POINTS SURVIVE A MAP CHANGE, 2026-09-14. seedling: job-advanced to Bowman at
        level 12, got the 7 SP in the advance, then they vanished. Measured: the advance
        0x007C was correct (tier 1 = 7), but the stat block in every SetField carried an
        EMPTY SP table, which the client reads as "zero every pool", so the next portal wiped
        them. Now the pool packet rides after every SetField, like the keymap. As purr (id
        218, already a level-12 Bowman with 0 spent), open the skill window - Bowman tab:
          7 SP shown -> walk through a portal, reopen the window:
            still 7 -> DONE; the fix holds across a field change
            back to 0 -> the after-SetField 0x007C did not apply; paste the world-ch0.log
                         lines around the SetField (the "skill points now [tier 1 = 7]" one
                         should be right after the SET_FIELD)
          0 SP on the FIRST open, before any portal -> the login SetField's 0x007C is
                         missing or ordered before the SetField; say so
        Then spend a point and change channel: the spent count must hold (spend IS persisted;
        the pool is entitlement - spent). A beginner (ouggh, id 215) must still show its own
        computed SP and NOT a phantom first-job pool.

     8. THE PET, SIX THINGS - 2026-09-15, from your two-client run (the walk is CONFIRMED:
        Tester2 sees the Husky, no crash). Every item below left a packet in world-ch0.log and
        each is answered now; none has been on a screen. As the owner with the Husky out, Tester2
        watching:
          LOOT, AND THE VACUUM - REWRITTEN 2026-09-17. The in-range vacuum (Petite Luna) is
          FREE on every pet: the pet ITEM carries wonderGrade 6 always, which is the wide
          pickup box (0x0198, fed after every SetField) AND the "Petite Luna" designation the
          tooltip shows. Auto Move (5190002) and Expanded Auto Move (5190003) are PAID skills
          for the pet's WALKING toward drops, not the box. Default declared skills are Meso
          Magnet + Item Pouch only (installer 4c no longer writes sweepForDrop/longRange) -
          NEEDS a client rebuild: python toolsackport_install.py --install (client closed).
            a. a FRESH Husky, no skills bought: tooltip shows only Meso Magnet + Item Pouch
               and "Petite Luna". Kill a snail ~200 px away: the drop flies to the pet, no
               walk, lands in the bag -> DONE; say the farthest distance that still works
               (the box is 600 x 590). Tooltip lists Expanded Auto Move / Auto Move as
               "unregistered" -> the WZ still declares them: rebuild the client (backport
               --install). "Ignore Item (Learned)" -> an old-numbering row the store did not
               remap; it cannot happen after the first start of this build (pets.rs remaps
               every row once), so paste the pet's "skills" from the log if it does.
               No suck-up at range -> paste the PetPickupRange line and the item bytes 61..63.
            b. buy Auto Move (5190002), then Expanded Auto Move (5190003) - a chain, Expanded
               is refused until Auto Move is learned. Each then lists as (Learned). What they
               change on screen is the pet WALKING to drops; say what you see move.
            c. A SKILL ITEM RIGHT AFTER LOGIN, before the pet is summoned or fed (2026-09-18,
               the live server, Cobalt 02:28): "That skill needs a pet to learn it. Nothing
               was used up." with the Husky in Cash slot 1. The field-entry restore sent the
               pet with the generic BAG serial (mark, character, tab, slot) and only a summon
               or feed re-sent it with the PET serial the skill lookup understood; Moth's
               worked because they had summoned first. The restore sends the pet serial now,
               and the lookup reads the bag serial too. Log in, open the Cash tab, double-click
               a skill item on the pet WITHOUT summoning: (Learned) in the tooltip, item gone
               -> fixed. The old message again -> paste the "pet skill:" line (it prints the
               serial)
            d. NO "CLOSENESS HAS INCREASED (+N)" ON A MAP CHANGE - SECOND FIX (2026-09-21).
               The slot fix of 2026-09-18 was real (two lines became one) but it was not the
               cause. The owner: "if the pet has some sort of closeness, a message of +1 closeness
               still erroneously show up ... despite not actually adding any closeness."
               THE NUMBER IS THE CLOSENESS, not a constant: Lucy's is 1, read out of the
               0x0070 body in the 21:20 log. The client's FUN_141ec4f60 prints string 0x1AC
               with the DIFFERENCE between the pet's cached closeness and the one it re-reads
               from the Cash item, on every local-user refresh - so any write that raises it
               prints a line and there is no quiet path. A field entry clears the client's
               bag, so the pet was being built with no item to read: cached 0, then our
               post-summon write took it to 1.
               The entry now sends the pet's item BEFORE the summon as well as after (the
               second write is the vacuum re-read and must stay).
               Feed the pet a few times so its closeness is > 1, then change maps:
                 no closeness line at all -> DONE
                 one line reading exactly the pet's closeness (+5, say) -> the pet does NOT
                            read its item at construction; the next lever is to drop the
                            post-summon write from the ENTRY batch only (the first-move
                            re-summon is the one that fixed the vacuum). SAY THE NUMBER
                 two lines -> the first-move re-summon prints one too; same lever
                 the pet spawns sad/droopy or stops vacuuming -> the pre-write is not enough
                            on its own; say which, it is the opposite regression
          THE COLLABORATION PETS AND EVERY PET EQUIP (2026-09-17, INSTALLED here - the
          client package must be rebuilt from client-patched\Data). The owner: "backport these
          pets ... as well as these pet equipment. All pets from these collaboration should
          be 1000 LP. Pet equipment should remain 100 LP each ... make sure all pet
          equipment is available." Cash Shop, Pets tab: Lil Frieren / Fern / Stark / Ubel
          at 1000 LP (SN 160000011..14). Pet Equip sub-tab: all TEN classic hats and the
          four weapons at 100 LP (was three hats). NEVER ON A SCREEN. Two things are [I]:
          the classic client drawing a modern pet (its animation set differs) and the
          weapon's inlined pixels.
            1. Pets tab: the four at 1000, and EVERY pet's icon - all fifteen - drawn WITH
               the purple "P" badge at the bottom-right (Nexon's own CashItem_label/9,
               baked into the icon pixels, 2026-09-18: "every pet is now a Petite Luna
               pet"); the same badge on the Cash-tab icon once bought, and NOT on the pet
               walking on the field (the badge was on a canvas two modern pets reuse as a
               frame; only icon/iconRaw carry it now) -> fixed. THIS NEEDS
               "python tools\backport_install.py --install" WITH THE CLIENT CLOSED - the
               2026-09-18 evening install was refused because the client was open, and
               the archives on disk are the earlier build (no roll/angry, badge on the
               field pet). Installed 2026-09-18 02:10 with the client closed, read back:
               all four declare pickupItem alone, c22/c23 present. And NO
               "3 / 7 / 90 day(s)" line on any pet's shop tooltip (info/life is 0 on all
               fifteen; the classic tooltip has no "Unlimited" wording, it just omits the
               line) -> fixed. A days line -> the old archive. A summoned classic pet
               that is INVISIBLE -> the 2026-09-14 "life 0" claim was right after all;
               paste the item line and client-exit.log (f0c3010 says giantRate was it). No
               badge -> the format-2 canvas did not draw; paste "wz-dump cat
               ...Item\Pet\_Canvas\_Canvas_000.wz 5002828.img" info lines.
               Pet Equip: 14 rows at 100 -> fixed
            1b. THE COLLAB PETS' SKILL LINE (the owner, 2026-09-18, Lil Fern's shop tooltip read
               "Skill: Meso Magnet, Item Pouch, Auto Move, Auto Buff"): "have them match
               existing pets ... only Meso Magnet and Item Pouch at default purchase time."
               Nexon's image declares sweepForDrop AND autoBuff; the installer now strips
               both (the classic eleven never had them). Hover Lil Fern in the Pets tab:
               "Skill: Meso Magnet, Item Pouch" and nothing more -> fixed. Still four ->
               the old archive (the same install that carries roll/angry). "Auto Buff"
               alone -> the strip missed autoBuff; paste "wz-dump cat
               ...Item\Pet\Pet_000.wz 5002829.img" info lines.
            0a. NO "NEW" MARK ON THE PET - the pet item re-sent on field entry is mode 5
               (2026-09-18). The owner's evening run: "Pets now work on initial summon"; no
               word on the mark itself, so: no highlighted Cash cell on login -> struck;
               still highlighted -> paste the Cash-tab 0x0070 lines after the SetField.
            0. THE VACUUM AT LOGIN / MAP CHANGE - CONFIRMED 2026-09-18 evening ("Pets now
               work on initial summon"): the re-summon on the first move after a field
               entry. Struck.
            2. buy Lil Frieren, summon: it draws, walks, name tag, no crash -> DONE. Its
               tooltip ends "Commands: sit, slap, iloveyou, sleep, talk, roll, angry". Type
               "roll", then "angry": it rolls / looks angry (the two animations Nexon's
               table never played); "sleep" -> it sleeps, "talk" -> the chat animation
               (Nexon's acts were sat/asleep and are corrected) -> commands landed. A
               refused command shows the puzzled look. Invisible pet with a
               name tag -> paste bytes 61..63 of its item line; client dies on summon ->
               the modern animation set, paste client-exit.log
            3. buy Lil Frieren's Staff (Deco tab), equip it on Lil Frieren: it draws on the
               pet -> the inline worked; on a Husky the client itself should refuse.
               Draws nothing on the pet -> paste wz-dump cat ...PetEquip_000.wz 01803148.img
          SIGNATURE STYLE PRICES (2026-09-17). The owner: "200 LP individually, or 800 LP for
          the signature set of all of them" (was 2000 / 8000). The price is in the CLIENT's
          Commodity.img and the server debits from gm-handbook/commodity.txt; BOTH come
          from `python tools/backport_install.py --install` (client closed), which also
          regenerates the handbook. Until that is run, the shop still shows and charges
          the old prices - consistently, so nothing breaks, it is just not done yet.
            1. after --install: Cash Shop, Special tab: the box 800, each set coupon 200
               -> fixed. Buy one: the wallet drops by 200 (the balance line) -> fixed
            2. still 2000 / 8000 -> --install was not run, or the client is the old
               package; the tag and the charge will still agree with each other
          LEAF POINT EXCHANGE COUPONS (2026-09-17). The owner: "when you use one of these items,
          it gives the player who used them the appropriate amount of Leaf Points in their
          account." 2430004..2430008 = 1,000 / 5,000 / 10,000 / 50,000 / 100,000. They are
          scripted consumables in the USE tab; the client's double-click dispatcher sends
          them on 0x0114 (read off the listing, NEVER YET ON A WIRE), the same 10-byte body
          the Cash-tab coupons use. The points go on the ACCOUNT's cash wallet.
            1. !item 2430004, double-click it in the Use tab: yellow "You received 1,000
               Leaf Points. You now have N Leaf Points.", the coupon gone -> fixed. Open the
               Cash Shop: the Leaf Point balance is N -> fixed
            2. nothing happens and the coupon stays -> paste the "<- 0x...." line the click
               produced from world-chN.log: that is the opcode it really sends. If NO line
               at all, the click never left the client (say so - no server fix reaches it)
          NPC SHOP DUPLICATES (2026-09-16). The owner: "duplicate items in the NPC shop, one
          being regular price, another being 10 times cheaper ... across multiple if not
          all NPC shops." The cheap twin was our Sell row (the WZ sell price); the
          classic window files EVERY row into the Buy list. The twins are gone; the Sell
          panel is the client's own (your bag at the WZ price) and was never reading them.
            1. open any shop (Lucy, the department store): each item ONCE, at its real
               price -> fixed. Two of anything -> the old build.
            2. the right-hand Sell panel still lists your bag with prices; sell a Red
               Potion: mesos up, potion gone -> as before. Sell panel MISSING or the
               sale refused -> paste the 0x055D line (row count) and the 0x00F5/0x055E
               pair - that would mean the sell byte was load-bearing after all.
            3. a quest item (Sera's Mirror) in the Sell panel: refused, stays -> as designed
          TWO HUSKIES ARE TWO PETS (2026-09-16). The owner: "Two Husky should not share the
          same name. The pets should in the background have different ids." Every pet
          item now carries a pet id (inventory.pet_id -> the pets table; the old
          character_pets rows are moved across on the first start and stamped onto the
          first matching pet, so a named Husky keeps its name). The pet's serial is the
          pet id, so a name tag or a skill item names ONE Husky. Needs two of the same
          pet in the Cash tab (buy a second Husky):
            summon the second, rename it Dummy with a Name Tag: the first is still
              Husky in its tooltip, the second is Dummy; relog: Dummy is out, Husky is
              not -> fixed
            both show the same name, or the wrong one comes back -> paste the two
              "re-sent as pet 5000006 #N" lines from world-chN.log (the #N must differ)
          A FULL TAB (2026-09-16): our yellow "Your bag would not take it" chat line is
          GONE. A refused pick-up - yours or the pet's - now sends the client's own
          0x0089 sub-mode -1, and the CLIENT draws "You can't get anymore items." in the
          message area where the EXP line draws, at most once every 2 seconds (its own
          latch, FUN_142dada50), so the pet's retries cannot spam anything. The 0x0070
          unlock still goes first, so later pick-ups keep working.
            pet over a drop for a full tab: "You can't get anymore items." in the
              message area, NOTHING in the chat log, drop stays -> fixed
            click it yourself: the same line, same place                -> fixed
            the line in the CHAT LOG, or a yellow one anywhere -> the old build
            no line at all, drop stays -> paste the 0x0089 from world-chN.log (3 bytes,
              00 00 FF); if it is there, the latch flush is not reached - say so
          SKILLS: use Auto HP on the Husky. Your 23:34 try KILLED THE CLIENT: the put-away
          half of the re-summon was 11 bytes and the owner's handler reads a 12th, a reason
          byte the remote handler never did (the client named the packet in a 0x009E). It is
          there now; the skill itself was stored before the crash. Expect ONE summon
          animation (put away and re-summoned so the pet re-reads its item), then:
            client exits at the summon animation -> the reason byte is not enough; paste
                       the new 0x009E line, its position says which byte
            tooltip says Auto HP (Learned), the item is gone  -> stored and applied
            walk into a mob until HP drops: does the pet feed you a potion? YES -> the mask
                       is read live or on init, DONE. NO with (Learned) shown -> the client
                       wants more than the mask; say so, that is the next measurement
          NAME TAG: use one on the Husky, type Dummy.
            name tag over the pet says Dummy on BOTH screens, tag gone  -> DONE (0x027B)
            only the owner's screen changes -> the map copy of 0x027B is dropped; say so
          HAT: put the Blue Top Hat on the pet with Tester2 already in the map.
            Tester2 sees the hat at once             -> DONE
            Tester2 sees TWO copies of the owner's character, or a frozen one  -> the second 0x0224 duplicates the user;
                       STOP, say so - the beauty coupon uses the same path and never tested it
            Tester2 sees nothing until a map change  -> the redraw ignores the look; say so
          FEEDING (2026-09-15, the owner: +30 fullness, +1 closeness, -1 fullness every five
          minutes). Buy Pet Food from Lucy (35 mesos), summon the Husky, open Show Pet Info:
            use one food: Fullness +30 (capped 100), Closeness +1, Level 2 at closeness 1 ->
                       DONE. The request is 0x0112, read off the client, never captured before
            the food is not used up / nothing changes -> paste the "pet food:" log line, or
                       the "<- ... 0x0112" line if there is none: the opcode is wrong
            the Husky EATS (0x027E type 2 with the food's id, read off the client's own
                       handler) on YOUR screen and on Tester2's -> DONE
            no eating animation on either screen -> the pet image has no food entry for
                       its level, or the type byte is wrong; say which screen
            NO "Yum, yum! Pet Food xN left!" balloon over the pet (2026-09-18, the owner: that
                       is the AUTO-FEED message, and it printed the count one short). The
                       0x027E carries food id 0 now: the client's balloon is gated on a
                       pet-food id, the animation is not. Eats, no balloon -> DONE
            the balloon still shows -> paste the 0x027E line; the animation is gone too
                       -> the food table needs the id after all; say so
            closeness 0 -> 1 is level 1 -> 2: a LEVEL-UP flash on both screens (UserEffect
                       9, subtype 0 = Effect/PetEff.img/Basic/LevelUp) -> DONE
            the flash on your screen only -> the remote 0x02AF is dropped; say so
            feed it twice more at 100: the second overfeed costs a closeness (the wiki rule)
          HUNGER: leave the Husky out. Every five minutes Fullness drops one (reopen the
          panel). At 0 it goes home, one closeness gone, "is starving" in chat. To see the
          end without an hour: this cannot be sped up from the client; say if you want a
          GM lever.
          TRICKS: "sit" that succeeds -> Closeness +3 (the table's own inc), and the level
          climbs the wiki table. A failed trick earns nothing.
          RE-LOGIN: log out with the Husky out, log back in.
            it is standing beside you on arrival, Cash tab shows it summoned -> DONE
            it is in the bag -> paste the "pet: character" log line at claim time
          SHOW PET INFO: worked on your second look, same build - the window's own pet list
          is built when it opens. If it is ever grey again, say whether Character Info was
          opened BEFORE the summon; that is the only measurement left on it.

     9. THE LAUNCHER UPDATES ITSELF - 2026-09-16. You: "the launcher should have the ability
        to patch itself." The server package now ships bin\maplecw-launcher.exe and
        maplecw-auth publishes it (--launcher); at Start Game, before the client is touched,
        the launcher hashes its own exe against the server's and swaps itself if they differ.
        ONE LAST MANUAL INSTALL: the launcher on D:\MapleCW (built 00:29) predates this and
        cannot update itself - install out\MapleCW-setup.zip over it once more. Then, with a
        server deployed from the matching MapleCW-server.zip, press Start Game:
          "launcher version <16 hex> confirmed with the server"          -> DONE (up to date)
          "updating it (8.x MB)", the window closes, a NEW window opens at sign-in with
                       "launcher updated: the previous executable ... was removed" in its log,
                       and no maplecw-launcher.exe.old left in D:\MapleCW      -> DONE
          "publishes no launcher ... Going on with the one you have"     -> the deployed
                       server has no bin\maplecw-launcher.exe: it is an older package
          the window closes and NOTHING opens -> the new exe failed to start; look for
                       maplecw-launcher.exe.old beside it and say what the folder holds
          it updates on EVERY Start Game -> the server's copy and the installed one differ
                       by build; say so, that means two builds of the same source in play
        AND SIGN OUT (same day): press Login, then Sign out.
          Start Game greys the instant Sign out is pressed, and the status line then says
                       "signed out - 1 claim(s) revoked on the server"        -> DONE; the
                       server's auth.log has a "sign-out: 1 login claim(s) revoked" line
          "the server had no live claim" -> a later Login of the same account had already
                       replaced it; not a bug
          "could NOT be revoked ... predates sign-out" -> the deployed server is older than
                       this launcher; deploy the matching MapleCW-server.zip

    10. BUFF ICONS FROM ITEMS - 2026-09-16. You: "the EXP coupon effects are not applying
        the appropriate buff icon on the top right" and "make sure that Magic Potions and
        other similar potions are applying the buff icons as well."
        Two bugs, one packet. A potion's 0x007D named its item id POSITIVE, which the client
        reads as a skill id (there is no skill 2002001, so it drew nothing). The coupon sent
        no 0x007D at all. Now every item stat names -itemId, and the coupon rides CTS bit
        163 ExpBuffRate (the client's own name for it) worth 200 or 300 for its duration.
        THE SIGN CONVENTION IS [D] - it is the modern reference's and a decade of clients',
        but no instruction in THIS build has been read testing it. This run is the reading.
          a. drink a Magic Potion (2002001): an icon top-right with the POTION's picture,
             counting down from 10:00                                          -> DONE
             icon appears but with no picture / a blank tooltip -> the sign is read but the
                       item lookup is not; say what the tooltip shows
             no icon, M.Att still +10 in the stat window -> the client ignores a negative
                       reason; the stat itself is unaffected. Say so - that is the
                       whole finding, and the fallback is a hidden-icon convention
          b. use a 3x EXP Coupon (2450001): a SECOND icon, 15:00, and the yellow
             "3x experience for 15 minutes" line                               -> DONE
             the potion's icon shows and the coupon's does not -> bit 163 is not what this
                       client draws for ExpBuffRate; the multiplier still works (kill
                       something: triple EXP) - say both halves separately
          c. wait either out, or !buff-cancel by right-clicking the icon: it goes away and
             the stat window drops the number. A coupon that outlives its icon or an icon
             that outlives its multiplier is a bug either way - say which.

    38. SP ON LEVEL-UP (2026-10-02). A first/second-job character one kill from a level.
          a. level up with the skill window open: the SP count rises AT ONCE and a + button
             works without a map change. Still needs a map change -> the second 0x007C did not
             apply; world-ch0.log shows "skill points now [...]" right after the LEVEL line

    50. JUMP QUESTS (2026-10-04). Forest of Patience, Deep Forest of Patience, Construction
        Site B1-B3. None of it has been on a screen yet. Every step logs a "jump quest:" line.
          a. Ellinia, Shane (top of the tree), on a character that never took Sabitrama's errand
             (quest 10509): his own "I can't let some stranger like you enter" -> right.
             With 10509 taken: a menu, Pink Anthurium (Steps 1-2) / Double-Rooted Red Ginseng
             (Steps 3-5), no mesos taken -> lands on step 1 / step 3
          b. Louis at the bottom of any step: No stays; Yes -> Ellinia, standing beside Shane
          c. the pile of flowers at the top of step 2: a box lists a Use prize and a Scroll (and
             the Pink Anthurium while 10509 is in progress). OK -> grey chat lines, the items in
             the bag, back in Ellinia beside Shane. No box, or nothing given -> paste the
             "jump quest:" lines from world-ch0.log. Click it from a few steps away (more than
             250 px): "Go a little closer", nothing given -> right
          d. Sleepywood, the Mysterious Statue: a menu of three flowers -> Deep Forest step 1 / 3
             / 5. The Crumbling Statue at the bottom: Yes -> Sleepywood beside the statue. With
             John's quest 10006 in progress the pink pile (top of step 2) gives 10 Pink Violas
          e. Subway Ticketing Booth: Jake sells B1/B2/B3 tickets (Lv 20/30/40, 500/1200/2000
             mesos); the Ticket Gate lists only the tickets held, takes one -> that floor's
             Area 1. An Exit: Yes -> back at the booth beside the gate
          f. B1 Area 1 has press-up portals that lead elsewhere on the same map. Using one moves
             you -> fine. The client hangs on one -> paste the last world-ch0.log lines: nobody
             knows yet whether the client sends anything for a same-map portal
          g. the B1 depot's chest: a box lists a Use prize and a Scroll, plus Shumi's Coin while
             quest 10312 is in progress; OK -> all of it in the bag, back at the booth
          h. the drops page: "Jump Quest Reward" (badge JQ) lists the use and scroll slots;
             after c its "finished" count reads 1 within 30 minutes
          i. PITY TIMER. Only an entry ON the course's quest (still short of its item) starts the
             hour; a reward-only run never does. !skipjq before the hour -> "N more minutes to
             go", nothing else happens. After 60 minutes on the course a yellow line "You have spent over an
             hour on this jump quest" -> right, and again every 5 minutes. !skipjq then -> the
             quest item only (no Use, no Scroll), back in town. Log out at 30 minutes and back
             in: !skipjq still says about 30 to go -> the hour was kept. Climb from one step to
             the next: !skipjq's minutes keep counting down -> one timer for the whole course.
             Disconnect on the course (close the client), log back in: the minutes carry on.
             Leave by the warden (or a return scroll), come back in: the hour starts again from 60

    49. PLAYER STORE (2026-10-04). Two clients on one map; the owner holds a Store Permit
        (5140001) in the Cash tab. Nothing of it has been on a screen yet.
          a. owner: use the permit, type a title -> the store window opens, owner in the first
             seat, and a sign with the title goes up over their head ("cannot enter").
             nothing opens -> paste the 0x017F and 0x0577 lines from world-ch0.log
          b. owner: put an item on the shelf (bundles and a price) -> it leaves the bag and a row
             reads "N for P mesos". Row missing or bag unchanged -> paste the 0x0180 / 0x0579 lines
          c. owner: Open Store -> the sign turns to "can enter"
          d. visitor: double-click the sign -> their window opens; the owner's chat says
             they entered. No window -> paste the visitor's 0x017F line
          e. visitor: buy one bundle -> it lands in their bag, their mesos drop by the price, the
             owner's rise by the price less 3%, and both windows show one bundle fewer
          f. a chat line from each side shows in both windows
          g. owner: Close Store -> the visitor's window closes with "The shop has been closed.",
             the unsold rest is back in the owner's bag, the sign is gone
          (first run 2026-10-04: a-f WORKED; the owner's whole UI then froze after Open Store - fixed)
          g2. after Open Store the owner can still click everything (Close Store, the bag) -> fixed.
              Still frozen -> paste the 0x0579 line right after the owner's 4-byte 0x0180
          g3. buy one line out while another remains -> that row reads SOLD OUT with its price;
              buy the last -> the store closes itself for everybody ("The shop has been closed.")
          g4. a second store (or elf) right beside an open store or elf -> "You can't open a store
              here"; a step or two further away (120 px) -> it opens
          h. HIRED MERCHANT, in the Free Market: double-click the Hired Merchant (5030000) in the
             Cash tab -> a title prompt, then the window; the elf appears where the owner stands.
             Nothing again -> paste the 0x0181 line and what followed (0x00A1)
          i. list an item, Open Store, close the window, log out -> the elf STAYS (check from the
             second client). The visitor double-clicks it, buys -> the owner's mesos rose at next login
             no elf on the second client -> paste the 0x0622 line it was sent
          j. owner double-clicks their own elf -> maintenance window; Close Store -> elf gone, rest home
          k. 24 hours after setup it closes by itself; the rest of the shelf comes home
        Any store button going dead = a request left unanswered: paste the last 0x017F / 0x0180
        and what the server sent after it.

    48. LIL UBEL'S NAME TAG (2026-10-04). Summon Lil Ubel (the accented U). The tag under it
        reads "Lil Ubel" with a plain U -> fixed (the U-umlaut was sent as byte 0xDC, which the
        tag's font draws as a box; the name is now folded to ASCII on the wire, as chat is).
          still a box -> the tag is not drawn from 0x0277's name; paste the 0x0277 line

    47. PET EQUIPS (2026-10-04). Regenerate first: python "C:\MapleCW\tools\dump_petequips.py"
          a. a Pet Equip scroll (e.g. !item 2048000 1) on the hat your pet WEARS: the hat's
             tooltip shows the new Speed and one fewer enhancement AT ONCE -> fixed
               only after a map change -> paste the "re-sending" line from world-ch0.log (it
               names "type 6 position -114"; type 1 there is the old bug)
          b. the same scroll on a pet hat sitting in the Deco tab: THAT hat changes, at once
          c. Tester2 opens Show Pet Info on a Lil Frieren wearing a Blue Top Hat: the hat cell
             is EMPTY -> fixed. The same hat on a classic pet (a Husky): it shows
               hat still shown on Lil Frieren -> paste the "pet equip ... does not fit" line,
               or say there is none (the table is missing: the server prints so at start)

    46. ANOTHER PLAYER'S CITIZENSHIP (2026-10-04). Two clients on one map; the owner a citizen
        (Henesys, as in the screenshot), Tester2 never signed.
          a. Tester2 double-clicks the owner: CITIZENSHIP is lit; open it: TOWN Henesys, the same
             GRADE and CONTRIBUTION the owner sees on their own window -> fixed
               still greyed -> paste the "character info:" line from world-ch0.log (it lists
               what was sent as "citizenship ...")
               lit but the panel is blank or names the wrong town -> the panel does not read
               the records the way the owner's own window does; paste a screenshot of both
          b. the owner double-clicks Tester2: CITIZENSHIP stays greyed (never signed), as on
             Tester2's own window

    45. OMOK AND MATCH CARDS ROOMS (2026-10-04). Two clients on one map; the owner and Tester2 each !item 4080000 1
        (an Omok set). Never on a screen before - every line is a first.
          a. the owner double-clicks the set, title "hello", no password: the Omok window opens
             with the owner seated, and a balloon "hello 1/2" floats over their head - on BOTH
             screens - with a "[Miniroom]" line in chat -> fixed. No window -> world-ch0.log
             "omok:" lines; client dies -> client-exit.log
          b. Tester2 clicks the balloon: Tester2's window opens with both seated, the owner's
             shows Tester2 sit down, the balloon says 2/2 -> fixed
          c. Tester2 presses Ready (both see it), the owner presses Start: the owner moves first.
             Alternate stones; an occupied square says "You can't put it there."; five in a row
             -> "You win." / "You lost." on the right screens
          d. a second game: the loser of the first moves first. Try take-back (both stones
             come off when the other has moved since), tie, give up, and expel
          e. the owner closes the window: Tester2 sees "The room is closed." and the balloon
             goes on both screens -> fixed
          f. a private room (password set): a wrong password says "The password is
             incorrect."; the right one opens it. A player who walks onto the map AFTER a room
             opened sees its balloon too
          g. THE RECORD (W / L / D and PTS in the side panel), per game: both start at 0/0/0,
             2000. After a game the winner shows W 1, PTS 2010 and the loser L 1, PTS 1990, on
             BOTH screens, and the next room shows the same -> fixed. The game ENDS cleanly (the
             result carries both records; a short one would have frozen it) -> say if not
          i. THE TIME-OUT (second run: both clients report it, and the turn bounced back): let
             a clock run out. Omok: a stone appears for the late player and it is the other's
             turn on BOTH screens, and their next stone is accepted -> fixed. Match Cards: the
             turn passes, a half-turned pair goes face down
          h. MATCH CARDS: !item 4080100 1 (a card set), same steps a-e. Every card shows for a
             moment at the start, then turns down. Your first card turns at once; the second
             shows on both screens; a miss turns both down by itself and passes the turn, a
             pair stays up and you go again. All pairs found -> the result. The Match Cards
             record is separate from the Omok one

    44. MIX DYE AND COLORBLEND (2026-10-03). The first look id above 9 999 999 this client is
        ever sent. !item 5151200 1 and !item 5152300 1 (neither is sold in the Cash Shop).
          a. Brittany (Henesys salon), menu line "mix two hair colours": the client's own
             UtilDlgEx_MixHair window opens on your character -> the box is right
               a plain dialog, or nothing -> paste the "ScriptMessage MIX" line
             The swatches are LIVE now (2026-10-03: the first run opened with every colour
             dead - the box sent a starting ratio of 0). Click a Base and a Mix colour: the
             preview recolours and a ratio slider shows 50/50 -> fixed
               still dead -> the setup gates on more than the ratio; say whether a slider
               appears at all
             Press CANCEL: the window closes with NO "same color is already equipped" box
             (the first run drew it on every Cancel - same cause) -> fixed
          b. pick two colours and a ratio, OK: hair shows BOTH colours, the coupon is gone,
             world-ch0.log "HAIR bit -> 4xxxxxxx" (8 digits) -> fixed
               head blank, or the client dies on the HAIR bit -> this client cannot draw a
               mixed id: paste client-exit.log and that line
               "mix answer ... refused" -> paste it (it names the reason)
          c. change map and relog: the mix is still drawn (SetField and character select carry
             the same id) -> fixed
          d. Dr. Feeble (Henesys surgery), "blend two eye colours": UtilDlgEx_MixLens, then OK:
             both eye colours drawn -> fixed
          e. open the box and pick the look you already wear: the CLIENT warns and nothing is
             spent
          f. (2026-10-03, hook patch MIXTOOLTIP) rest the cursor on a swatch: its name ("Violet")
             fades in ONCE and stays -> fixed
               still flickers -> paste the MIXTOOLTIP lines from client-patched\maplecw-hook.log
               (no line at all = the patch did not install; "refusing" names why)
               the name shows, then vanishes until the mouse moves -> the hide is a per-frame
               leave, not the rebuild; paste the MIXTOOLTIP lines too
             clicking a swatch still picks it (the label must not eat the click)

    43. TRADE, THIRD PASS (2026-10-03). Two clients on one map. What you put in LEAVES your bag
        or wallet and comes back if the trade is cancelled; the Trade button now completes it.
        Already seen on screen: your own offer draws on YOUR side of BOTH windows, the red
        "You have sent a trade request" line, and puts after the first one.
          a. Tester2 invites the owner: Tester2's chat shows a red "You have sent a trade request
             to '<name>'." -> fixed. Now have the owner open an NPC shop (or storage, or talk
             to an NPC) and invite again: no popup for the owner, and Tester2 sees "'<name>' is
             doing something else right now." -> fixed. Paste the "mode 6" line if not.
          b. drag a stack into the trade window: it LEAVES your bag and shows on your side of
             both windows. Drag a SECOND item: it goes in too -> fixed (before, the first put
             was the last - the client waited for an inventory update that never came)
          c. put in 3000 mesos: your wallet drops by 3000 at once -> fixed. Then put in 1000:
             the window shows 1000 and your wallet is 3000 + 2000 back -> the client sends a
             TOTAL (what the server assumes). If it meant "1000 more", say so.
          d. type in the trade chat: the line shows in BOTH windows, yours and theirs in two
             colours -> fixed. Nothing in one or both -> paste the "mode 8" lines
          e. close the window with things on both sides: the other closes with "Trade cancelled
             by the other character", and BOTH players get their items and mesos back -> fixed
          f. press Trade on ONE side: the OTHER window shows that side as ready -> fixed.
               no sign on the other screen -> paste the "0x10/2" line
          g. press Trade on the other side too: both windows close with "Trade successful.",
             the items cross, and the mesos arrive LESS 5% (2000 arrives as 1900) -> fixed.
               the receiver's message says "Received 1900 mesos after fees" or just "Trade
               successful" - say which; either is fine
               "There was a problem trading the item." -> paste the 0x10/2 and 0x10/5 lines
          h. MEASURED 2026-10-03: this client never un-presses Trade, so a change after a press
             left the presser stuck. Now a press FIXES THE TABLE: Tester2 presses, then the
             owner tries to put something in -> refused in red ("Tester2 has pressed Trade -
             press Trade to accept, or close the window to cancel"), nothing leaves the owner's
             bag, and the owner can still drag again afterwards -> fixed. The owner presses:
             done, with the table as Tester2 accepted it.
          i. (from the decompile, 2026-10-03) the client's own messages: decline an invite ->
             the inviter sees "'<name>' has denied the invitation."; accept an invite after the
             inviter has left -> "The room is already closed."; drag an untradeable item (the
             beginner's weapon) -> a "This item temporarily can't be traded." dialog and the
             window still works -> fixed
          j. THE REQUEST COOLDOWN (2026-10-03). Tester2 invites the owner and the owner does NOT
             answer; Tester2 invites again -> Tester2 sees "Please invite later." and the owner
             gets no second popup -> fixed. The owner declines; Tester2 invites again within a
             minute -> "Please invite later." again; after a minute it goes through -> fixed

    42. DISORDER'S DEBUFF (2026-10-02) - the first mob status ever sent (0x03E6). A thief with
        Disorder: hit a mob with it and look ABOVE the mob.
          a. a debuff icon appears over the mob for ~10-30 s (by level) -> fixed
               no icon, nothing else odd -> the status index (12 attack / 13 defence) is not
                          what this client draws; paste the "MobStatSet 0x03E6" line
               the client dies / disconnects on the hit -> the packet layout is wrong: paste
                          client-exit.log and the 0x03E6 line - this is the measurement
          b. let the debuffed mob touch you: a little less damage than before (attack -5..-25)
          c. the OTHER client on the map sees the same icon

    41. STARS BY THE SET (2026-10-02). At a Grocer, buy Subi Throwing Stars once.
          a. the window asks yes/no (no quantity box) -> fixed
               a quantity box still appears -> the cap of 1 is not what hides it; buy 3 anyway:
                          you must still get ONE set for ONE price
          b. the Use tab gains 500 Subi and the mesos drop by 500 (one set) -> fixed
               1 star, or 3 sets charged -> paste the "bought a SET" line from world-ch0.log

    40. THE DAMAGE GUARD (2026-10-02). A hit more than 25% over what the character could deal is
        CAPPED to that, and the attacker gets a line in damage-suspects.log (beside world-ch0.log).
          a. play normally on EVERY class you have - warrior, magician (Magic Claw), archer,
             thief with Ilbi or better, Power Strike / Slash Blast, crits included. Then:
               damage-suspects.log absent or empty -> the ceiling holds for honest play -> fixed
               a line names one of YOUR characters -> the model is low for that case: paste the
                          line (it lists weapon, skill, attack, stats). Until fixed, restart
                          with -DamageGuardLogOnly
          b. EVERY thief and archer skill is priced now - Lucky Seven, Avenger, Shadow Meso
             (mesos x 8), Three Snails (fixed 15/25/35), Arrow Bomb, Power Knockback. Use each
             once: a suspects line for one of them is that skill's pricing being low - paste it
          c. the cap itself cannot be triggered by an honest client; it is pinned by
             the_damage_guard_caps_an_impossible_hit_and_passes_an_honest_one

    39. EMPTY STAR STACKS (2026-10-02). A thief with a claw and a SMALL star stack (!item 2070000 3).
          a. throw until it runs out: the stack STAYS in the Use tab showing 0 -> fixed
               the slot goes blank / the client dies on the 0 -> this client cannot draw a
                          0-count stack; paste the "down to 0" line and client-exit.log
          b. throw again with only the empty stack: no star leaves, no damage line oddity
          c. at a Grocer, Recharge the empty stack: the label reads "Recharge: 150", the stack
             fills to 500 and the mesos drop by 150 -> fixed
          d. drop the empty stack, pick it back up: it returns at 0, no "x1" message -> fixed
          e. relog: still 0 (not 1 - the store used to read a 0 back as 1) -> fixed
          f. (2026-10-03) THE RECHARGE BUTTON. It was never drawn for a stack of 1 or more:
             every star row told the window its full stack was 1, and the window prices a
             recharge as (full - held) x unitPrice and shows the button only above 0
             (research/shop-recharge-button.md). Hold 37 Subi AND 2 Wolbi, go to Luna or any
             Grocer, open the Sell tab:
               a Recharge button on BOTH, labels "Recharge: 139" (463 x 0.3) and
               "Recharge: 200" (498 x 0.4); press each: full, mesos down by the label -> fixed
               a button on Wolbi only        -> Subi's recharge row is not the one found
                                                first; paste the ClassicOpenShop line
               a button on neither           -> something else gates the button; paste the
                                                ClassicOpenShop line ("N of them recharge-
                                                only" must be N > 0, or the server is old)
               a button, pressed, no refill  -> paste the "recharge:" line from world-ch0.log
             Then BUY one Subi set there: still a yes/no and +500 Subi for 500 mesos.
             BUY INDEX: only Max (Kerning City Civic Center) sells anything AFTER a star. If
             you hold the grade, buy Unagi there: Unagi arrives -> fixed; "row N is item X and
             the client asked for Y" in world-ch0.log -> the client DOES count the recharge
             rows; paste it (no wrong item can be sold - the server refuses it)
          g. (2026-10-03) every star stack is its own: hold a partial Wolbi AND an empty one,
             pick up dropped Wolbi and buy a set - each lands in a NEW slot at its own count,
             and neither old stack changes. Recharge the HIGHER-slot partial stack: that one
             fills, the lower one does not -> fixed
          h. (2026-10-03) a star stack is ONE item. With two partial Subi stacks (37 and 120):
               drag one onto the other: they SWAP, both counts unchanged -> fixed
               (a merge into 157 = the old server)
               press Consolidate / Sort: they slide up, still two stacks -> fixed
               drop the 120: the slot empties (not 119), pick it up: 120 in a new slot -> fixed
               store one at Mr. Kim, take it back: same count, own slot -> fixed

    38. FAST SELLING (2026-10-02). At any NPC shop, Sell tab.
          a. double-click-sell one item as FAST as you can, twice on the same row: it sells
             ONCE, mesos rise once, NO "not enough mesos" message, and the row disappears from
             the Sell list -> fixed (world-ch0.log: "a STALE sell ... the slot's real state
             follows")
               the row stays listed after the stale click -> a REMOVE does not redraw the list
                          when it repeats one already sent; paste the STALE line
          b. sell PART of a potion stack (e.g. 1 of 3): the slot shows 2 left, not empty ->
             fixed. Before this, the slot vanished on screen while the server kept the rest

    37. OPTIONS KEPT ON THE SERVER (2026-10-02). Two characters on one account.
          a. log in; System Options: change the HP warning % (the pet's auto-potion threshold),
             the effect volume. world-ch0.log shows "options: account N group ..." for each
          b. Change Channel, then log out and back in: both kept. Then the OTHER character:
             the same VOLUME (per account) but NOT the first one's HP/MP warning % - since
             2026-10-02 the pet's auto-potion setup (acpHP/flHP/flMP and their wrnHP3/wrnMP3/
             petHP2 twins) is per CHARACTER; the log line marks those keys "(character)". A
             character that never set one gets the account's pre-split value. Set a different
             % on the second, go back to the first: its own % is still there. The potions the
             pet drinks and the key layout were already per character (keymap); check the
             second character does NOT inherit the first's pet potions or key bindings
          c. THE RISK: a login now carries record block #32 for the first time. Login fails
             or the client dies right after SetField -> restart the server with
             -NoSystemOptions and log in again. Works then -> #32's placement is wrong; the
             HP warning should still be kept (it is block #28)
          d. HP warning kept but sound not -> #32 decoded but the sound reader looks elsewhere

    36. FIRST-JOB SP PAST 30 (2026-10-02). A first/second-job character above level 30.
          a. the skill window's 1st-job tab shows more SP than before: level 45+ has 106 in total
             (Thief 112 at 47+), minus what is spent. Unchanged -> the skill table did not load
             (the startup banner says so)
          b. SP Reset Scroll: every first-job skill back to 0 and the full grown total back

    35. DEATH WITH A PET + TWO CLIENTS (2026-10-02). Two characters on one map; the dying one has a
        pet with Auto HP on.
          a. die: the revive dialog opens and the pet drinks NOTHING; HP stays 0, the stack is unchanged.
             A potion arriving -> the refusal did not fire; world-ch0.log says "refused, character N
             is DEAD"
          b. ON THE OTHER CLIENT: does the dead character lie down as a ghost / get a tombstone?
             YES -> the forwarded stance is enough. NO -> an observer needs a packet nobody has found
             yet (research/same-map-capability-sweep.md row 14); say what you DID see
          c. REVIVE IN TOWN: town, 50 HP. The character that crashed every login (saved on map 0)
             now logs in to Henesys

    34. DROP PAGE: MAPS (2026-10-01). After a redeploy.
          a. no Ludibrium monster (Ratz, Chronos, Thanatos) is listed; Crimson Balrog is, saying it
             invades the ship -> DONE
          b. open Slime: "Found on" lists its maps, most spawn points first (The Tree That Grew II
             x30 at the top) -> DONE. Search a map name and its monsters come up -> DONE

    33. GLOBAL SCROLLS + CHAOS + TOOLTIPS (2026-10-01). Redeploy: data\drops.txt, the binaries and
        gm-handbook\itemdesc.txt (python tools/dump_names.py) all changed.
          a. !scroll with no Scroll of Secrets / Treasure Scroll: the box says they no longer drop
             and lists Innocence (1 in 1000), Chaos (1 in 500), Pure Clean Slate (1 in 500) and
             Lucky Day (1 in 1000) with icons -> DONE
          b. a Chaos Scroll on an item: whatever happens, a success changes a stat by 1-5 points,
             never 0 -> DONE. A success with no change = old build
          c. drop page: hover a scroll or an equip name - a dark tooltip with the game's text, an
             equip's REQ LEV and stats, a backported scroll's "On this server" rule -> DONE

    32. DROP PAGE ON 8481 (2026-10-01). On the server box (or forward 8481), after some kills.
          a. open http://<server>:8481/ - the drop rate box already shows the server's rate, marked
             (server) -> DONE. 1x when the server runs 5x = it is not reading live.json
          b. kill 10 Snails, wait up to 35 minutes (5 to write, 30 for the page's cache), reload: the
             bar's kills and Snail's "kills - players" went up, and Snail Shell's Seen count too
             -> DONE. Nothing after an hour = paste the "killstats:" lines from world-ch0.log
          c. search "shoes greater scroll": Shoes Jump Scroll: Greater is listed -> DONE

    31. V83 DROPS (2026-10-01). Live is fine; note the server's drop rate first.
          a. Snails and Blue Snails: an equip about 1 kill in 100-200 at 1x (1 in 20-40 at 5x),
             and no scroll -> DONE. An equip every few kills = the old drop file is still deployed
          b. Mano: shells nearly every time; at 5x a Wand Magic Attack scroll about 1 kill in 67
             -> DONE (the drop-table page lists each mob's numbers at any rate)
          c. Scroll of Secrets / Treasure Scroll: about 1 kill in 200 EACH whatever the rate ->
             DONE. Far more often at 5x = the global table is still being scaled

    30. BEGINNER RECOVERY (2026-10-01). A Beginner with Recovery, hurt.
          a. cast it: a Recovery icon in the tray counting down 30 s, no chat line -> DONE
               no icon = bit 131 is not Recovery's; say so, and whether anything else changed
          b. blue +4 (level 1) every 5 s, six times; the icon goes with the last one -> DONE
          c. cast again, right-click the icon: it goes and the +numbers stop -> DONE

    29. BUFFS ACROSS A CHANNEL CHANGE + RESPAWN WAVES (2026-10-01). Live is fine for this.
          a. cast a timed buff (Magic Armor, or a potion), change channel while it runs: the icon
             is there on the new channel with the time left -> DONE. When it reaches 0 the icon
             AND the stat go -> DONE
               icon stuck at 0 = the carry did not land; paste the "buffs:" lines from BOTH
               channel logs (world-ch0.log and world-ch1.log)
          b. Magic Guard (no timer) is still on after the change -> DONE
          c. on a busy field, kill three mobs a few seconds apart: all three come back TOGETHER,
             at most 8 s after the first kill -> DONE
               each back 8 s after its own kill = old build

    31. QUEST LINES ONCE (2026-10-02). Turn in any quest at its NPC (Heena's mirror, a board
        donation, anything):
          a. the completion lines appear ONCE - the client's own box - and after you press
             Next/Yes at most ONE more box (the thank-you) or none -> DONE
               the same lines a second time = old build
          b. a weekly donation: "are you saying you'd like to donate?" asked ONCE; Yes ->
             "Thank you so much!" once, and the client stays up -> DONE

    30. BACKPORTED SCROLLS + LUCKY DAY (2026-10-01). The client data changed: launch the
        client from client-patched (or let the launcher patch) BEFORE this step.
          !item 2049100 5   (Chaos 60%)     !item 2049003 5   (Pure Clean Slate 20%)
          !item 2049190 5   (Innocence 70%) !item 2530000 5   (Lucky Day Scroll)
          a. each shows its modern icon and name in the Use tab; Lucky Day's tooltip reads
             "Increases the success chance of your next scroll by 100%" -> DONE
               a blank icon / no name = the client did not take the backported data; say so
          b. DRAG Chaos onto a worn item: the client's own "Do you want to use..." box, then the
             scroll animation, success or failure at about 60%; a slot is used either way -> DONE
               nothing at all after Yes = the client refused it before sending; grep the log
               for "0x0125" - absent means the drag never reached the server
          c. DRAG Innocence onto that item: back to its original stats at about 70% -> DONE
          d. DRAG Lucky Day onto an item: the client's confirm, then the success animation, one
             Lucky Day gone. Look at the item's tooltip - does it show anything new? SAY WHICH
             (that is a measurement: attribute bit 9 is [I]). Then drag a 10% scroll (or Chaos)
             onto it: it SUCCEEDS. Do it three times -> DONE
               the drag does nothing = grep for "0x0126"; absent = the client did not send it
               a second Lucky Day on a marked item is refused (client message or ours) -> DONE

    29. OVERALL + SHOP TABS (2026-10-01).
          a. wear a top AND a bottom, equip an overall: the top AND the bottom both land in the
             Equip tab at once, no map change needed; the bottom slot is empty -> DONE
               the bottom still drawn until a map change = old build
          b. the other way: overall on, equip a bottom -> the overall drops into the bag at once -> DONE
          c. at an NPC shop: BUY something from the Use tab (the window jumps to Use - fine),
             then click the Etc tab and SELL an Etc item: the window STAYS on Etc -> DONE, and
             the sold item disappears from the list -> DONE
               jumps back to Use = old build
               stays on Etc but the sold item is STILL LISTED = the sell list only redraws on
               result type 0; say so - that is the whole finding, and it decides the next fix

    28. BOARD + !SCROLL (2026-10-01). A Henesys citizen; the live server is fine for this.
          a. ONE weekly: the quest log's In Progress shows at most ONE "Donating to Henesys",
             and a character who had several in progress keeps only one after logging in
             (the highest one their level can turn in) -> DONE
               still several = old build; paste the "board 510002" log lines
          b. the posted donation is one the character can TURN IN: a level-17..21 character at
             grade 3 gets a level-17 donation, not the level-22 one -> DONE
          c. turn in the day's resident, then "!citizenship 1 grade 5": NO new daily (no leader,
             no "Asking After") until tomorrow; a grade-up mid-week posts no new weekly -> DONE
               a new one appears = the pick is not being kept; paste "board 510001"
          d. !scroll -> Treasure Scroll -> an item: under the real scrolls, "Scroll of Secrets
             as a Chaos Scroll" and "... Clean Slate Scroll" (when you hold a Scroll of Secrets).
             Chaos through it always works and takes one of each scroll -> DONE
          e. after ANY !scroll result, with scrolls left: "Shall we keep scrolling?" Yes = the
             !scroll menu again, No = closes -> DONE. The last scroll: just the result -> DONE

    27. WEATHER ITEMS (2026-09-30). Two clients on one map; Sprinkled Chocolate (or any 512xxxx).
          a. use it with a message: chocolate falls and the message shows on BOTH screens, one
             spent -> DONE. After ~30 s it fades out -> DONE
               it stops dead instead of fading, or lasts ~10 s -> say which; 0x01B7 carries 30
          b. use a second one while the first is running: refused with a notice, item kept -> DONE
          c. a third client walking onto the map mid-effect sees the rest of it -> DONE
          d. GM'S BLESSINGS (Use tab, !item 2023000 / 2023001), two clients on one map: the GM
             weather with "<name> ..." on both screens; BOTH get a buff icon counting down from
             60:00 - Wind: faster and higher jumps; Precision: accuracy +20 -> DONE
               icon but no faster run / higher jump -> Wind's bits 92/93 are the [D]; say which
               only the user gets it -> paste the "blessing:" log line

    26. MEGAPHONES (2026-09-30). Two clients on DIFFERENT channels, both level 10+.
          a. Super Megaphone with the whisper box ticked: "Name : text" on BOTH clients, pink,
             with the whisper icon; one Super Megaphone gone from the Cash tab -> DONE
          b. again with the box unticked: no whisper icon -> DONE
          c. Megaphone: only the client on YOUR channel sees it, not pink, whisper icon as
             ticked -> DONE
               the Megaphone line comes out pink, or with an item box -> say which; it is sent
                          as 0x00AC type 8 (kind 0xf), and that choice is the [D] in this
               nothing on the other channel for (a) -> paste the "megaphone:" log line

    25. SHIP TO ORBIS (2026-09-26). Ellinia Station (10002090) - Ellinia's station door.
        Departures are every :x0 of the station's wall clock (UTC); boarding is from :x5 to
        :x9, and the last minute before a departure is closed on purpose.
          a. Joel: "Hi there! I'm Joel..." with Next, then a MENU with both tickets (not a shop
             window). Buy one of each -> Basic costs 5,000 and Regular 20,000, each with a grey
             meso line and a grey item line, and they send you to Cherry -> DONE
               a shop window, or the menu with no introduction -> old build
          b. Cherry OUTSIDE :x5-:x9, pick Basic: "We will begin boarding 5 minutes before the
             takeoff..." (or in the last minute "This ship is getting ready for takeoff...") with
             the next ship's time, and the ticket stays in the bag -> DONE
          c. Cherry INSIDE the window, pick Basic: "This will not be a short flight... Do you
             still wish to board the ship?" No -> "You must have some business..." and nothing
             moves. Yes -> the ticket goes (grey line), you land in Before Takeoff with a
             "Time Left" countdown to the departure -> DONE
               no countdown -> paste the "FieldClock type 2" line, or its absence
          d. at the departure you are moved to To Orbis with ~5:00 on the clock; walk into the
             cabin and back - the clock carries on, it does not restart -> DONE
          e. at 0:00 you land in the Orbis Ticketing Booth, "The ship has arrived" -> DONE
          f. TWO CLIENTS, both Basic for the same departure: they see each other in the waiting
             room, on the deck and in the cabin -> DONE. A third boarding the NEXT departure
             sees neither -> DONE
          g. Regular: Cherry any time -> Before Takeoff ALONE with 0:10 on the clock, then To
             Orbis alone with 1:00, Orbis at 0:00 -> DONE (was straight on deck before
             2026-09-29). The same from Rini on the way back -> DONE. A second client on its
             own Regular ride sees nobody, in the waiting room or on deck -> DONE
               straight onto the deck = old build
          h. Purin in the waiting room: "Are you sure you want to get off the ship?" No -> "You'll
             get to your destination in a short while..." and you stay. Yes -> back to Ellinia
             Station before the ship leaves, ticket not returned -> DONE
          i. disconnect (or change channel) in the waiting room, on deck or in the cabin, then log
             in: you are in Ellinia Station -> DONE
          j. THE SHIP AT THE STATION (0x01BF / 0x01C0, new). Enter the station between :x5 and
             :x0: a whistle, and the ship slides in from the right over ~15 s -> DONE. Enter at
             any other time: a whistle, and it slides out to the right -> DONE. Stand there
             across :x5:00 - it comes in; across :x0:00 - it leaves -> DONE
               no ship and no whistle -> paste the "ContiState 0x01C0" / "ContiMove 0x01BF"
                          lines; present = the packet went and the client ignored it
               the ship jumps into place instead of sliding -> say so; the arrive/leave
                          reading of the two routines is the [D] in this
               a crash on entering the station -> say so FIRST, before anything else
          k. THE CRIMSON BALROG INVASION (half the Basic crossings, never a Regular one). A minute
             into the ride, standing on the deck: the Balrog's ship comes alongside and two
             Crimson Balrogs appear at it, in the air -> DONE. A second player on the deck sees
             the same; one in the cabin sees nothing until they come up -> DONE
               no invasion in several rides -> grep the log for "will be INVADED"; present and
                          nothing on screen = paste the ContiMove / MobEnterField lines
          l. THE WAY BACK (2026-09-27). Orbis Ticketing Booth: Agatha "Hello, I'm the information
             guide..." + Next, then Ticket to Ellinia (Basic) / (Regular) -> DONE. The Platform
             Usher: "Platform to Board a Ship to Victoria Island" -> Isa's question -> Yes puts
             you in the Station Tunnel; walk right to Rini's platform -> DONE. They have NO second
             line - no ferry since 2026-09-29 -> DONE. Rini, Erin and the ride are the same as (b)-(k); the
             ship lands in Ellinia Station -> DONE. A disconnect aboard logs in on Rini's
             platform -> DONE
               a ferry line on the Platform Usher -> old build

    26. EMOTES (2026-09-29). Two clients on the same map.
          a. press an emote on A (F1-F7, or Queasy): B sees A's face change too -> DONE
               nothing on B = grep B's channel log for "UserEmotion 0x02A6" (absent = not
               relayed, present = the client did not draw it - say which)
          b. B on a different map sees nothing, and A's own face works as before -> DONE
          c. SHADOW STYLE (09-30): A double-clicks it ONCE (a second double-click switches it
             OFF again - that is what the first try did) and WALKS: afterimages
             behind A on A's screen AND on B's -> DONE. B changes map and comes back: still on
             -> DONE. A double-clicks again: gone on both -> DONE
               on A only = grep B's log for "UserEffectItem 0x02A8"

    25. CITIZENSHIP (2026-09-28). A GM character of level 12+, no citizenship yet.
          a. Henesys Town Hall, talk to Arthur: the "Oath of Citizenship" CONTRACT window (not a
             chat box), Arthur's name on it -> press Sign: the STAMP plays, ~2 s later the
             window closes, "You are now a citizen of Henesys" + the CitizenshipGet effect -> DONE
               buttons grey and the window stays forever = old build (09-29 fix: the server
               sends the force-close that starts the stamp) - paste the "force-close" log line
               a plain line instead = old build or under level 12; blank name = say so;
               a CRASH on opening = the window's layout is wrong - say so FIRST, nothing else
          b. the Community Board in front of the hall: exactly ONE resident's "First Greeting"
             and ONE "Donating to Henesys" are available, nothing else -> DONE
               all of them available = the board record is not read (paste the "quest 510001"
               log line); none available = the record's format is wrong (paste it too)
          c. "!citizenship 1 contr 950", then do the resident quest: "You have gained ...
             Contribution (+1000)" at 10x (+100 x the Quest rate), the grade-up effect, a notice to see Arthur -> DONE
               no Contribution line = paste the "Message 35" log line
          d. talk to Arthur: the "Citizenship Grade Update" certificate, rank Visitor -> DONE.
             Talk again: a menu (standing / renounce) -> DONE
          e. Flint (scroll shop, same hall): the Town Resident rows drawn LOCKED; buying one is
             refused. "!citizenship 1 grade 5", reopen: they unlock and sell -> DONE
               drawn open and refused = the row fields are not read; drawn locked at grade 5
               = paste the "ClassicOpenShop" line
          h. CITIZEN OF HONOR (09-29), two clients on different channels if you can:
             "!citizenship 1 grade 9", "!citizenship 1 contr 9950", then do today's resident
             quest -> grade 10: the Henesys Earrings appear in Equip, and BOTH clients get a blue
             "[Notice]Let us all congratulate <name> for becoming a Citizen of Honor in Henesys!"
             -> DONE. Doing it again gives no second earring and no second notice -> DONE
               no line on the other channel = paste its log's "BroadcastMsg type 0" line (absent
               = the hub did not deliver); a red/yellow line = the type's colour is not blue
             Raymond / Max: EVERY row locked for a non-citizen, Fried Chicken etc. included
             (Traveler+, 09-29); a Henesys Traveler buys them at Raymond, not at Max -> DONE
          f. Kerning City Civic Center, Roxy: "Transfer of Citizenship" -> OK: you are Kerning's,
             Henesys is frozen ("!citizenship" shows st1=2) -> DONE
          g. QUEST RATE ("!rates" shows the Quest field, 10x live): in (c) the Contribution line
             reads 10x the quest window's number, and "mesos (+n)" appears at 10x its meso
             reward. ANY other quest turned in now pays its mesos too -> DONE
               no mesos line = paste the "paid ... mesos" log line; 1x numbers = check !rates

    24. CASH SHOP BEAUTY PREVIEW (2026-09-26). Cash Shop > Beauty > Hairstyles.
          a. click Mystery Hair Coupon: the panel below fills with hairstyles for YOUR gender,
             and the colour swatches recolour them -> DONE
               still empty -> grep the channel log for "CashShopBeautyPreview 0x05B9"
               absent = old build; present but empty panel = paste the log line
          b. the same for Signature Hair Coupon, and for the two face coupons under Faces -> DONE
          c. the client does not crash or eject you on entering the shop -> DONE (a wrong
             layout would show here first)

    23. SPAWN POINTS (2026-09-26).
          a. walk to the far side of a big map (Kerning City), log off, log in: you appear at the
             spawn point NEAREST where you stood, not at the map's first one -> DONE
          b. same across a channel change, and in and out of the Cash Shop -> DONE
               back at the usual spot -> paste the "spawn point:" log lines
          c. Nella -> Kerning City (or a Return Scroll) several times: you land at DIFFERENT
             spawn points around town -> DONE

    22. 2ND JOB TEST OF QUALIFICATION (2026-09-26). A level-30 Magician who has handed Grendel's
        letter to the Magician Job Instructor.
          a. pick "Test of Qualification": three lines, the LAST with Accept/Decline. Accept ->
             in the Magician's Tree Dungeon, and the quest book shows it in progress with 30 Dark
             Marbles -> DONE
               last line has only OK -> old build; Accept but no warp -> paste "second-job test:"
          b. talk to the instructor inside: back out BESIDE the Magician Job Instructor, not at
             the bottom of the map -> DONE
          c. "Talk to Magician Job Instructor" with <30 marbles: asks to go back in; Yes -> in,
             No -> stays -> DONE
          d. with 30: pick the quest, hand them in, get The Proof of a Hero; Grendel advances -> DONE
          e. as a 2nd job: "nothing more to teach"; before the quest: "not ready yet ... Grendel" -> DONE

    21. LAUNCHER UPDATE DIALOGS + COPY LOGS (2026-09-25).
          a. the post-update notice, without a real update - from the launcher's folder:
               maplecw-launcher.exe --updated-from no-such-file.old
             it opens with "Launcher updated ... please sign in again", nothing behind it
             clickable until OK -> DONE
          b. a real update (package once after changing the launcher): the OLD window shows
             "Launcher updated ... will close and reopen ... sign in again"; OK closes it and
             the new one opens with the notice from (a) -> DONE
               closes with no dialog -> the player still has the old launcher build
          c. open Log, press Copy logs, paste into Notepad: every line, with [info]/[WARN]
             tags, under a header line -> DONE

    20. PET FEED LINE, SAME ON EVERY SCREEN (2026-09-25). Two players on one map; one feeds
        their pet by hand, several times.
          a. both screens: the pet eats and shows the SAME bubble text each time -> DONE
               different text -> paste the "pet line:" lines from the feeder's channel log
          b. the watcher sees the EATING animation too, not only the bubble -> DONE
               bubble but no eating -> the relay's second byte matters; say so, it is one line
          c. nothing doubled: one bubble per feed on the watcher's screen -> DONE

    19. PET AUTO HP / AUTO MP (2026-09-25). The pet's drink (0x0206) was never handled.
          a. a pet with Auto HP, potions set in its slot, take damage below the threshold: the
             pet drinks, HP rises, the stack drops by one -> DONE
               nothing happens -> grep the channel log for "pet 0 drinks"; absent = old build
          b. same for Auto MP -> DONE
          c. afterwards, inventory / AP / Cash Shop still respond (the latch is cleared) -> DONE

    18. ITEM VARIANCE (2026-09-24). Every equip a mob drops rolls around its template.
          a. kill mobs until an equip with a level requirement drops; pick it up and hover it.
             A stat differing from the item's base -> DONE (the log's "variance:" line
             names the roll and the template side by side)
               the tooltip shows the plain template on every drop -> paste the variance lines
          b. a stat rolled BELOW the base: the tooltip still draws it, no crash -> DONE
             (Chaos already does this; this confirms it for a fresh drop)
          c. log out and back in: the same numbers -> DONE
          d. the King Slime: each member's Squishy Shoes differ from each other -> DONE
          e. wear a dropped equip, note its stats, Chaos it, then Innocence it: it returns to
             the numbers it DROPPED with, not the plain item's -> DONE
               the plain item's numbers -> paste the "variance:" line for that drop

    17. FIRST TIME TOGETHER, LAST STAGE DROPS (2026-09-24). Two members, kill the King Slime.
          a. each screen: the Pass and mesos, then ONE pair of Squishy Shoes at the end of the
             row, evenly spaced - nothing stacked, no gap -> DONE
               shoes on top of the Pass -> the server is the old build
               a gap in the row -> paste the "is PERSONAL to" lines from the channel log
          b. Jr. Necki / Curse Eye drop a Pass and mesos, never a Coupon -> DONE
          c. on the Exit map, holding Passes/Coupons, talk to Nella: they go, one grey "lost"
             line each, before they ask; other Etc items stay -> DONE

    16. MAPLE CHAT ACROSS CHANNELS, AND BUDDY CHAT (2026-09-24). The deployed server's logs: a
        room opened on channel 1 could not be joined from channel 0 ("result 1 - not here", the
        "busy" message), and buddy chat went nowhere. Rooms now live in the hub, like parties.
        NEEDS THE HUB (maplecw-chat) REBUILT TOO - an old hub drops the new frames silently.
          a. The owner on channel 1, Tester2 on channel 2. The owner opens Maple Chat and invites Tester2;
             Tester2 presses Accept: both windows show both avatars -> DONE
               "busy" / nothing opens -> grep Tester2's channel log for "maple chat: hub echo";
                          absent = the hub is the old build
          b. type a line on each side: both see both lines -> DONE
          c. Tester2 closes the window: the owner's window drops Tester2's avatar -> DONE
          d. BUDDY CHAT: with the two as buddies, switch the chat box to Buddy and type: the
             other side sees it as a buddy line, on the same channel AND across channels -> DONE
               nothing arrives -> paste the "buddy chat:" line from the sender's log

    15. THE CASH SHOP, 483 NEW WARES (2026-09-23). Every named cash item the classic shop never
        listed, except the collaboration sets (they come from the coupons). All 100 LP; no
        cash equipment has a duration any more, including the classic shop's own clothing.
        Needs the NEW client data: tools/backport_install.py --install was run, so this
        machine has it; restart the world servers so they load the new table.
          a. Fashion / Gloves and Fashion / Effects, both EMPTY before, now list items -> DONE
          b. any hat or top: the tag says 100 and there is NO "90 days" line -> DONE
               a days line still shows -> the client is reading an old Etc_000.wz
          c. BUY one new ware (e.g. Red Boxing Gloves): it arrives, 100 LP leaves, and the
             server's log line names SN 140700000 -> DONE. "NOT on sale" / "no commodity
             serial" -> the world server was not restarted
          d. Frieren's Clothes / Himmel's Clothes are NOT in any tab - they still come only
             from the Signature Style coupons -> DONE. Listed -> the exclusion failed; say so
          e. DELETE (2026-09-24): select an item in Cash Inventory, press the trash button,
             confirm. It disappears with "The cash item has been deleted." -> then BUY
             something right after: it works -> DONE, the latch is released
               the delete works but the next buy does nothing -> the wallet did not clear
                          the latch; paste the two 0x05AE / 0x05AD lines
               "Due to an unknown error" again -> the old build; restart the world servers

    14. GROUND DROPS BELONG TO EVERYONE; UNTRADEABLE ONES VANISH (2026-09-23). The owner:
        "nobody except themselves were able to pick up what was dropped on the ground." The
        server had allowed it all along; the 0x046E told every other client the item was the
        dropper's (ownType 0). A player's tradeable drop and their mesos now go out as ownType
        2, "anyone". An untradeable item a player drops is drawn landing, then FADES for
        everyone after 1.5 s, and nobody gets it. NO CAPTURE OF THE FAILURE EXISTS - this step
        is the one that proves the client half. Two clients, same map:
          a. The owner drops a TRADEABLE item (any sword). Tester2 walks onto it and presses the
             pick-up key: it goes into TESTER2's bag -> DONE
               nothing happens -> grep world-ch0.log for "<- [Tester2" and 0x032C:
                 absent  -> Tester2's client still will not ask; the gate is client-side and
                            the next suspect is ownerId (drop+0x68), not ownType
                 present -> the server refused; paste the "pick-up:" line under it
          b. the same with MESOS: Tester2 gets all of them (a player's mesos are not split)
          c. drop one and wait: it stays for the whole drop lifetime, then fades on BOTH
             screens -> DONE
          d. The owner drops an UNTRADEABLE item (the Beginner's weapon, or anything that refuses
             storage). It goes out as enter type 3, the client's OWN disappearing animation,
             to both screens. Watch for TWO things and say which you saw:
               it plays a distinct fade-out on BOTH screens, with no pick-up prompt -> DONE
               it lands like a normal drop, sits ~1.5 s, then fades -> type 3 is not the
                          animation; the 1.5 s fade is the server's cleanup
               it fades, then REAPPEARS or blinks at ~1.5 s -> the client already destroyed
                          it and the server's cleanup is redundant; say so and it goes
               The owner's screen only -> the enter is not reaching the field
          e. LATE ARRIVAL: the owner drops a tradeable item, THEN Tester2 portals in. Tester2 sees
             it and can pick it up -> DONE. Invisible to Tester2 -> the entry re-send
          f. a QUEST item a mob drops is unaffected: the killer can still take it, and it
             does not vanish. If it does, the disposal rule has leaked onto mob drops

    13. THE TRADE WINDOW - 0x0575 mode 4 (2026-09-22). The owner: "Tester2 just sent the owner a
        trade request, but after the owner accepts it, the Trade window did not open." The invite
        has worked since 2026-09-09; what was missing is the packet that OPENS the window,
        whose payload runs through a virtual call on whichever miniroom class the room type
        picks. For a trade that call is FUN_141C423D0 and its only packet read is
        FUN_1402ee8d0 - the avatar decoder 0x0224 already uses. research/trade-2026-09-09.md.
        DONE 2026-10-03: both windows opened and both players put things in them. Putting
        things in is step 43 now; (b)..(e) below are kept only for a regression.
          a. Tester2 invites the owner, the owner clicks Accept:
               BOTH trade windows open, each showing the OTHER player's avatar and name in
                          the far seat and itself in the near one -> DONE
               only one side opens -> say WHICH; the other's copy goes over the field bus and
                          the log says "has no live session on map"
          b. IF THE CLIENT DIES ON ACCEPT: paste client-exit.log and the CLIENT FAULT line
             from maplecw-hook.log. That is the one guess in this packet - after the member
             list the client makes a last virtual call, resolved to a method that reads
             NOTHING, and the resolution is [D] because the slot arithmetic lands in a region
             shared with a second vtable. A fault means the body is short by whatever it does
             read, and the fix is that function, not the rest of the body.
          c. the avatars: the near seat is you, the far seat is them, dressed as they are.
               a blank or naked seat -> the look block; say which seat and paste the mode 4
                          line's byte count (it is 438 for two bare characters)
               the NAMES swapped or both the same -> mySlot is inverted; the host is slot 0
          d. TWO windows on one screen -> the creator should not have been sent a mode 4 at
             all; mode 0xB (somebody entering a window that is already open) is undecoded and
             that would be the packet to read next. Say so, it is a real outcome
          e. whatever you do next in the window (drag an item, press Confirm) sends modes
             0x0C / 0x10 and this server answers neither. Expect nothing to happen; if
             something DOES, paste it, because that is a client-side behaviour nobody has
             recorded

    12. FRIENDS - THE BUDDY LIST DRAWS, AND THE LOOP IS GONE (2026-09-22, third pass).
        Confirmed on the owner's screen: the popup appears, Yes/No work, and the Buddy tab lists
        "Default Group (1/1)" with the owner in a NAME / JOB / LV row - so the 329-byte record is
        right where it is measured. Two things fixed since that screenshot and one wanted:
          * THE LOOP. Every list reply ends in a window refresh, and a refreshed window hands
            its group names back as 0x0193 sub-op 0x14 - which this server answered with a
            list. 32 566 round trips, a 42 MB log, "lagging a lot" and a frozen buddy list.
            Sub-op 0x14 is a REPORT and is answered with nothing now.
          * A TIMEOUT, 60 s, measured from when the BALLOON went up rather than from the ask -
            otherwise every request made while the target was offline expires before its
            balloon is drawn. Both sides get 0x2A "The request to add a Friend has been
            canceled."
          * STILL OPEN and all one packet (0x00A7 sub-op 0x2D, decompiled, not built): the
            location check, the logout that leaves a friend showing online, the blank JOB and
            LV columns, and "Tester2 is now your friend" as the client's own line.
          a. Tester2 adds the owner, the owner presses Yes. THEN LEAVE BOTH CLIENTS SITTING FOR A MINUTE:
               no lag, and opening either buddy list is instant -> THE LOOP IS DEAD, done
               still laggy -> say WHICH action starts it, then grep world-ch0.log for 0x0193
                          and count the sub-ops. A different one is looping and that number
                          names it
          b. the rows: the owner in Tester2's list and Tester2 in the owner's, under Default Group
               both -> DONE
               name blank or garbled -> offset 4 is measured, so that would be the string
                          write, not the layout - say exactly what it shows
          c. JOB and LV (2026-09-23): the row builder FUN_1411be0a0 was read, not guessed -
             LV is rec+0x139, JOB is rec+0x13D into the client's own job-name lookup. Open the
             list with Tester2 OFFLINE as well as online:
               "Beginner" / "Magician" and the right level, both times -> DONE
               the level right, JOB blank or wrong -> the job id is not what the lookup keys
                          on; say what it shows
               both still blank -> the server is the old build; rebuild
          c2. LOCATION: select Tester2, the status line reads "Tester2 - Kerning City" (or
             whatever map), NOT a chat line. On another channel it reads the channel.
               a chat line "'Tester2' is currently at ..." -> the old build (mode 0x09)
               "Channel 1" when they are on channel 2 -> the channel is off by one; say so
          d. TIMEOUT: Tester2 adds the owner, the owner IGNORES the balloon. After one minute both read
             "The request to add a Friend has been canceled." and the row is gone both sides
               -> DONE
               the balloon is still on screen after the sentence -> EXPECTED, [I]: whether
                          0x2A also dismisses a live balloon is not decoded. Press its Yes
                          anyway and say what happens - nothing should, and the log says
                          "nothing is waiting"
               no sentence at all -> the tick is not reaching it; paste the "friends:" lines
          e. TIMEOUT DOES NOT FIRE ON AN ANSWER: accept within the minute, then wait two more.
             Nothing is cancelled and you stay friends -> DONE
          f. PRESENCE (0x2D, new). With both logged in and friends:
               log out as the owner -> Tester2's row for the owner GREYS OUT, with no chat line
               log back in as the owner -> Tester2 reads "[Friend] the owner has logged in." and the
                          row un-greys -> DONE, all three presence reports
               the row never changes -> paste the "friends: ... went ONLINE/offline" line;
                          if it is there, the packet went and the client ignored it, which
                          points at the status byte (0/1 is [I])
               the line is said but the row stays grey -> they are two different fields and
                          the list half is what to look at
               a line on every PORTAL rather than once per login -> announced_presence is
                          not holding; say so
          g. "Tester2 is now your friend." should now be in the SAME COLOUR as "Tester2 has
             declined the friend request." - both go through the client's own printer at kind
             0xb. Different colours -> say which is which
          h. the 51st buddy is refused with "Your buddy list is full." The header still reads
             [n/0] - KNOWN, the client's max lives in a field nothing has been found to set

    11c. CRAFTING - the Crafting Journal, and the six quests that open its tabs
    11c. CRAFTING - the Crafting Journal, and the six quests that open its tabs
        (2026-09-21). The owner: "We need to implement crafting in our server. After these quest
        completions, they should unlock the appropriate crafting menu within the client."
        The window, the recipe list and the animation are the CLIENT's; this server owns the
        bag, the mesos and the mastery. One craft is TWO packets (0x02F6 mode 0 "may I", then
        mode 3 "it finished"), and NOTHING is taken until the second.
        research/crafting-2026-09-21.md; 348 recipes from tools/dump_craftrecipe.py.
          a. !craft smithing 1 - the tab opens with no quest. Then walk to the Anvil in
             Perion (10004000, x 718) and open the Crafting Journal.
               the SMITHING tab is live and lists level-1 recipes; the other five are greyed
                          with "Smithing Level 1 or higher is required to craft."  -> DONE
               every tab still greyed -> the skill did not reach the client: paste the
                          "ChangeSkillRecordResult" line from world-ch0.log
               (the Journal is opened by a CLIENT KEYBIND - the owner, 2026-09-21 - so it opens
                          anywhere; the greyed tab is the only gate)
               still greyed after !craft -> the skill did not reach the client; paste the
                          "ChangeSkillRecordResult" line from world-ch0.log
          b. !item 4010000 20, then craft a Bronze Plate (5 ore, 100 meso, 3 mastery):
               the bar animates, the ore drops by 5, 100 mesos go, the plate lands in Etc,
                          and the chat says "Smithing's mastery increased. (+3)" -> DONE
               the bar fills and nothing happens -> the mode 7 never came; paste the
                          "CraftResult" lines
               a red sentence instead ("not enough materials/mesos/space", "only available
                          near crafting tools") -> quote it exactly. Each one is a different
                          check and they are listed in the research §4
          c. the MASTERY BAR's percentage. The curve is SETTLED (the owner, 2026-09-21: "settle
             for the EXP curve in the client, since that's the source of truth for the client
             display") - this is a sanity reading, not a choice.
             !craft smithing 2 0, then craft ONE plate:
               1.80% (3 of 166) -> DONE, the server and the client agree
               anything else -> the server's mastery_exp_needed and the client's
                          FUN_1401d2320 have drifted; say the number
          d. Craft All with 20 ore: four crafts run back to back off ONE button press (the
             client loops by itself, one 0x02F6 pair each) -> DONE. It stops after one ->
                          the mode 7 result was not 0; paste the CraftResult line
          e. !craft (no arguments) lists what is open and the mastery as n/needed;
             !craft all 10 opens all six; !craft 2 0 closes Tailoring again -> DONE
          f. the real thing: do quest 80008 (Silas Irons, Perion, level 10+) and watch for
             "You have learnt Smithing." with the tab opening on the same screen -> DONE
          g. ALREADY-FINISHED QUESTS (2026-09-21). The owner had finished Vicious's quest before
             the server read Act.1.skill at all, and the Woodcrafting tab still said "Vicious
             in Henesys is looking for an apprentice". The claim now backfills every finished
             crafting quest, so: just LOG IN on that character and open the Journal.
               Woodcrafting is live on the first screen, no relog, no re-quest -> DONE
               still greyed -> the quest row is not Complete in the database; paste the
                          "crafting: character ... finished quest" line, or its absence, from
                          world-ch0.log
               NOTE: the backfill learns the profession at LEVEL 1 with an EMPTY bar. The
                          mastery those old turn-ins would have paid is gone for good and is
                          not replayed (a login that added mastery would be a farm).

    11b. GIFT DROPS - !giftdrop and !giftall (2026-09-18/19). The owner wanted the modern Gift
        Drop window for compensation; this client has no such window (no UI image, none of
        its strings), its mailbox window has no way in, and the Cash Shop locker was refused
        ("the Cash Shop should not handle items that are not Cash Items"). So: "via our usual
        MapleStory Administrator ... via !giftdrop, and our usual show NPC chat dialogue",
        "all gifts expire in 7 days if unclaimed", "!giftall which gives all accounts (not
        character) an item ... claim it on any character", "a cancel option". Rows in the
        store (store::gifts); the box is the Administrator's type-6 menu, like !tool.
          GM: !giftdrop Tester2 2000000 10 Sorry about the crash
            Tester2 ON THIS CHANNEL: their box opens at once - "GIFT DROP", the message,
                       "Reward: [icon] Red Potion x10 (from the owner)", "Expires in 7 days.",
                       Claim / Refuse / Cancel -> DONE. Claim: 10 potions in the Use tab, a
                       "Claimed:" box; the chat had "You have a gift waiting" first
            Tester2 OFFLINE or on the other channel: nothing until they log in; then on
                       their FIRST MOVE after entry the notice + the box -> DONE. A box that
                       shows with the SetField -> say so (it must wait for the move)
            Claim with the Use tab full -> the quests' own "make 1 space in your Use tab"
                       box, the gift stays; !giftdrop again after freeing a slot claims it
            Refuse -> "You refused ..." and it is gone for good; Cancel -> "Kept for
                       later", and !giftdrop shows it again
            queue two: the box says "1 more waiting"; after Claim the second box opens
                       by itself -> DONE
            a non-GM typing !giftdrop with arguments: said out loud as chat, nothing queued
          GM: !giftall 2000000 5 Thanks for testing
            every player on this channel gets the box at once, marked "For your account:
                       claim it on whichever character you like." -> DONE
            Cancel on Tester2, log in Tester3 (same account): offered on the first move;
                       Claim there; back on Tester2, !giftdrop -> "nothing to claim" -> DONE
            the GM's own screen gets one too (the GM's account is an account)
          the icon does not draw in the box / the name is blank -> the #i / #t token for
                       that id; say which item. A box that never appears -> paste the
                       "giftdrop:" line and whatever 0x055B follows it in world-chN.log
    11a. THE QUEST HELPER'S ITEM COUNTS AFTER A MAP CHANGE (2026-09-18). The owner: "when
        players enter a new map, the progress in quest helper completely zeroes out. But
        when you pick up an item in that map from killing mobs, it will return back to
        normal" - 0/5 Bronze Ore, then 49/5 after one pickup. The helper caches its item
        counts and refreshes them from a hook the loud 0x0070 modes call; the bag is
        restored with mode 5 on purpose (no collection popup on every map change,
        2026-08-30) and mode 5 skips that hook too. The 0x0089 quest-record message's
        handler calls the same refresh and never the popup, so after the bag is back the
        server re-sends every in-progress quest's own record, unchanged. [L] both chains,
        [I] that the refresh re-reads the bag. With a collection quest in progress and its
        items in the bag, change maps:
          the helper shows the right count at once (49/5 Bronze Ore), no popup -> DONE
          still 0/N until a pickup -> the refresh uses a cached count; paste the
                     QuestRecord lines after the restore, and the next variant is one
                     mode-0 add for a quest item
          the collection popup ("n / N item") appears on the map change -> say so; that
                     is what mode 5 exists to prevent and this must not undo it
    11. A QUEST INTO A FULL BAG - 2026-09-17. Mint: "quest continues to complete despite this
        happening" under "Quest 1008 could not give you item 1002005: inventory 1 is full".
        The completion was written, the EXP paid, the letter taken, and the hat never came.
        Now the room is counted BEFORE anything moves (crate::questroom - stacks are topped
        up the way the store places them, a take frees the slot it empties), and a shortfall
        is the NPC's own box: "Your bag is full ... Please make N space(s) in your <Tab> tab.
        Then come and talk to me again." Nothing written, nothing paid. Same rule on accept
        (Sera's mirror into a full Etc tab). As any character with a full Equip tab, holding
        Lucas's letter (quest 1008 in progress): talk to Lucas.
          Lucas's box says "Please make 1 space in your Equip tab", the quest is still in
                       the journal, the letter still in Etc, EXP unchanged      -> DONE
          then free one Equip slot and click them again: hat in, letter gone, EXP up,
                       their own closing line                                      -> DONE
          the box shows AND their closing line follows it -> the refusal path was not seen
                       by the 0x0151 handler; paste both ScriptMessage lines
          a yellow "could not give you item" line -> the old path; the server is stale

    12. THE CRASH - a question, not a test.
        A 1.36 GB dump was written at 00:08 from a fault at 0x14090a6f0, an address that
        appears NOWHERE else in the archive. It is an std::map node walk hitting a bad
        pointer, with 38 C++ throws before it. You said you were "just in the map with
        monsters, not doing anything particular" - so there is no action to blame, and what
        is left is something that accumulates while a session runs.
        WHAT WOULD HELP: roughly how long had the client been up? A duration turns an
        unreproducible crash into a number that can be compared between runs.

    CARRIED OVER, STILL UNCONFIRMED - lower value than the above
    ------------------------------------------------------------
    IRON BODY reducing damage. It never did, because our model summed equipment defence and
    nothing else; that is fixed and unretested. SAY THE W.DEF NUMBER BEFORE AND AFTER:
    a rise of about a quarter means the percent-to-flat conversion is right, a rise of
    exactly 25 whatever the base means the raw percent reached the wire.
    !learn 1000001 15 killed the client once, 3.7 s after a byte-correct packet. Do it at
    ~40 s of client life: dies again early -> the command is fatal; does not -> the session
    was long and the command is innocent.
    BOWMAN, THIEF and MAGICIAN branches - none tested since the weapons were sorted out.
    BUYING from an NPC shop, and the cash-shop purchase.

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
      A pet walks on BOTH screens - CONFIRMED 2026-09-15 (the 0x0202 head is 5 bytes, not
        9; the path goes out whole). --no-broadcast-pets is the owner-local fallback.

    STILL OPEN - do not spend the run confirming these are broken
    ------------------------------------------------------------
      - NPC SHOPS: BUILT this session and awaiting step 7. Never on a wire.
      - BUYING FROM THE CASH SHOP WINDOW is BUILT and awaiting step 6. The old note here
        said no packet could report a purchase without a message; that was a known-list
        search over the six INLINE arms and it missed the two that delegate. 0x05AE sub-op
        0x0C is the one. (The field-side !buy control was removed on 2026-09-06.)
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

    COMMANDS (GM): !map, !item, !exp, !heal, !job, !learn, !nx, !lp, !resetap, !resetsp,
    !npcecho, !setrates, !npcreload, !registrationcode, !recoverycode, !track <name> (channel
    and map of one player, from the world hub's directory). EVERYONE: !rates, !online (who is
    on, across channels), !tool (three favours a day - Leaf Points, Level up (the AP/SP reset left 2026-10-02: Cash Shop),
    and RETURN TO HENESYS for a player stuck in a map; refused free if already there), !scroll,
    !help - a player's !help shows only those. PRUNED 2026-09-06 on the owner's instruction:
    !kit, !buff, !unbuff, !npcfx, !migsweep, !buy, !locker and the per-kind rate setters
    (!exprate !mesorate !droprate) are GONE; !setrates <exp> <meso> <drop> <quest> <party%> is
    the one rate command (FIVE fields since 2026-09-06: the 4th multiplies quest-completion
    EXP, the 5th is the percent of a kill each OTHER party member on the map receives as a
    COPY - killer keeps 70%; 30 is the old behaviour, 50 makes a 100-EXP mob pay 70 + 50 per
    member), and it writes only the kinds that changed. !rates lists all five and is public.
    !RATES AND THE CLIENT: on 2026-09-06 the client died 8 ms after a !rates reply, 51 min into
    a session, inside a std::map walk on a garbage node - the FIRST chat notice in 30 min. The
    same-shaped text survived on 08-21. So: type !rates at ~40 s of client life. Survives ->
    the session was the cause (the long-session corruption family); dies -> !rates is fatal
    and that is a finding nobody has yet. !job, !resetap and !resetsp answer in
    ONE line now ("Cobalt is now a Swordsman", "Skill Point successfully reset for Cobalt");
    the working is in world-ch0.log.
      !nx [amount]                          grant NX. Real and displayed, but it buys
                                            NOTHING - every price tag reads LP
      !lp [amount]                          grant LEAF POINTS, the currency the shop
                                            actually charges. This is the one that buys
      !resetap                              put every spent ability point back in the pool.
                                            Conserves the total - it refunds the difference
                                            from a fresh character rather than recomputing a
                                            per-level number nothing here knows
      !resetsp                              forget every skill. The points come back on their
                                            own: the pool is computed from your LEVEL, so a
                                            forgotten skill IS the refund
      !learn [level]                        Learn every skill of your current job, each
                                            clamped to ITS OWN maximum - the Magician book
                                            runs to 15 and 20, so one constant is wrong for
                                            half of it. No skill points spent. !learn 5 caps
                                            them; !learn <skillId> <level> does one. The
                                            weapon is yours to !item: five of the 24 first-job
                                            skills carry a weapon column (45/46 bow or
                                            crossbow, 33 dagger, 47 claw), and a Magician
                                            still needs a wand for its incMAD

    THE FREE MEASUREMENT NOBODY HAS TAKEN
    -------------------------------------
    Every run dumps the client's own EXP curve on the positive control's
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
    # The SIGN-IN service's port - crates\auth, over TLS. A different service from the game
    # ports, and this harness used to hardcode 8080 while every other script took a flag, so
    # a dev run and an installed one could not be made to match.
    #
    # 8480 is the default on all three sides now (crates\auth DEFAULT_PORT, the launcher's
    # DEFAULT_AUTH_PORT, and start-server.ps1), because 8080 is contended on Windows and a
    # collision fails as "received corrupt message of type InvalidContentType" rather than
    # anything about a busy port. A test in the launcher pins the two Rust constants together.
    [int]$AuthPort = 8480,
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
    # Channel N listens on $ChannelPort + N and logs to world-ch0.log (channel 0) or
    # world-ch<N>.log (the rest). Channel 0 keeps the plain name because every doc and
    # instruction in this repo points at world-ch0.log.
    [int]$Channels = 2,
    # The account -ListOnly prints. Nothing else reads it any more.
    #
    # Until 2026-09-05 this was ALSO the fallback the login server served every connection it
    # could not tie to a launcher sign-in - which is how the default run opened a client that
    # was simply "logged in" as maplecw with nobody having signed in anywhere. The owner: "I want to
    # remove this functionality and enforce login." The fallback is -FallbackAccount now, off
    # by default, and the default run goes through the launcher.
    [string]$Account = 'maplecw',
    # Serve a connection the login server CANNOT attribute to a launcher sign-in as this
    # account instead of refusing it. Off = LOGIN ENFORCED. Only -DirectClient needs it.
    [string]$FallbackAccount,
    # Give a -DirectClient run a client token, WITHOUT a launcher sign-in.
    #
    # Only maplecw-launcher can mint a token the login server will recognise, so anything
    # passed here is a WRONG token by construction - and the login server's own rule is that a
    # wrong or spent one DOWNGRADES the connection to -FallbackAccount rather than refusing it
    # (its startup banner says so). So the account served is unchanged; the only thing that
    # changes is that the CLIENT's identity field is non-empty.
    #
    # That is the point. Enumerated over 15 archived login logs on 2026-09-06: every run whose
    # client carried a token sent 0x0078 CLIENT_SELECT_CHARACTER_REQUEST and got a migrate
    # (9 of 9); every run without one sent 0x00C0 CLIENT_AUTH_FAILURE_REPORT instead and NEVER
    # sent 0x0078, hanging on "Connecting to server" after a character is picked (3 of 3,
    # 2026-09-05 21:10 and both 2026-09-06 23:2x attempts). The two remaining logs never
    # reached a character list at all and say nothing either way. Whether that is the token or
    # something else about the direct path is confounded - every token run is a launcher run -
    # and THIS FLAG IS THE DISCRIMINATOR: a token the server rejects still fills the client's
    # field. If the client then selects a character, the field is what it checks. If it still
    # hangs, the token is not the cause and the direct path differs some other way.
    #
    # 26 characters of uppercase base32 is the shape the real thing has. identity.rs refuses
    # anything with a byte outside 0x21..0x7e, or longer than 1024.
    [string]$ClientToken,
    # When a migration is bound to the sign-in that minted it: auto (default - bound when the
    # login connection came from a process on this machine, which the channel re-checks
    # through the OS; address-bound otherwise), always, or never. The escape hatch if a
    # character stops entering the world and world-ch0.log says "REFUSED the migration":
    #   -BindMigrations never
    [string]$BindMigrations = 'auto',
    # The launcher drives every ordinary run now: it signs in, installs the hook and starts
    # the client. This switch is kept so an old command line still works; it changes nothing.
    [switch]$Launcher,
    # THE OLD DEFAULT: start MapleStory.exe directly, no launcher, no sign-in. Kept for
    # arming a hook watch (-Probe, -Session) because the launcher writes those markers with
    # its own defaults. Login is enforced, so a direct client is REFUSED at the login screen
    # unless -FallbackAccount names who to serve it as - and then so is anything else that
    # reaches the port. The script refuses to start a direct client without it, because a
    # run whose client cannot get past the login screen answers nothing.
    [switch]$DirectClient,
    # Start the servers and stop there - launch neither the client nor the launcher.
    #
    # This is what start-servers.cmd uses. The owner starts the servers by double-clicking that,
    # and then starts the CLIENT by double-clicking maplecw-launcher.exe, which carries its
    # own requireAdministrator manifest. Without this switch the only two outcomes were "also
    # launch the client" and "also launch the launcher", and neither is that flow.
    #
    # Like -Launcher, this does NOT clear a live login claim: the launcher is about to stake
    # one, and clearing it here would only matter if it managed to race the sign-in.
    [switch]$ServersOnly,
    # What the login screen displays when the account has NO email - a fallback now, not the
    # answer. The masked address is derived from the account being served
    # (`Account::masked_email`), so it follows the launcher's claim instead of being fixed at
    # startup. This value is what an account created before the email column existed gets,
    # because an empty field draws as a blank line where a person expects to see themselves.
    #   maplecw-useradd.exe --email <name> <address>   (in target/release)
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
    # The migration-handler (141b36f60) and GS-reporter (142ef3e44) watches are both dropped
    # for the blank-select question - four slots are needed.
    #
    # **THIS DEFAULT DOES NOT REACH THE CLIENT ON ITS OWN.** The client is launched by
    # maplecw-launcher, which writes ITS compiled default (crates/launcher/src/client.rs
    # DEFAULT_PROBE) over maplecw-hook.probe on every launch; this string only gets through
    # as a -PinPatches pin. Keep the two IDENTICAL, and check the hook log armed them.
    #
    # 142ef3e44 is __report_gsfailure, kept because a silent 37s death is the failure mode
    # this project spends the most runs on. The last two are plan step TL,
    # research/charselect-avatar-fade-race.md sec 8:
    #   141177790:hits=6   the select UI's build. Its time against login.log's "-> 0x0010"
    #                      says whether the screen was built BEFORE the list (the blank
    #                      ordering, which the hook's SELECTFILL step now rescues) or after
    #   141177e80:hits=16  the per-slot fill - three lines per fill
    # **Never watch 141177e40 here.** The hook's selectfill step CALLS it and reads its
    # prologue first; a watch plants 0xCC over byte 0, and on 2026-09-12 that made the guard
    # refuse the fix on both blank logins. The SELECTFILL: log line already says when the
    # call happens. Watch lines are written on ENTRY, so a missing line means never entered.
    [string]$Probe = 'watch@1415db360:ret,141b2a280:rdx=0,142ef3e44:hits=8,141177790:hits=6,141177e80:hits=16',
    [string]$SessionTokens = '',
    # **-PetGates**: arm the four watches that name which gate hides a summoned pet, instead
    # of the default watches. A switch rather than a pasted -Probe string because the run of
    # 2026-09-13 was launched without the string and came back with the DEFAULT watches - four
    # clean zeros that looked like an answer and were an unarmed instrument, which is the
    # failure mode `CLAUDE.md` calls the most expensive on this project. One word cannot be
    # half-pasted. research/pet-not-drawn-2026-09-13.md.
    # **-PetSync** (was -PetFlags, and the alias still works): read the pet show/hide sync
    # end to end in ONE capture - the ladder runs, gate 11's two flags, and whether the sync
    # ever changes anything. It replaces the setter watch, which could only ever say that
    # nobody CALLED a setter and not what the fields hold. See the block that sets $Probe.
    [Alias('PetFlags')]
    [switch]$PetSync,
    # **-PetFrames**: walked the chain that puts frames into the pet's layer. ANSWERED
    # 2026-09-14: every step ran and the layer is NOT empty, which eliminated the chain.
    # Kept because it is the control that says the draw path still works.
    [switch]$PetFrames,
    # **-PetEnable**: ANSWERED 2026-09-14 and the answer was NO - user+0x100+0x5ac read 0
    # on all 6000 samples, 141ec7970 never fired, and the pet's layer was set OPAQUE once
    # and never touched again. Both that hypothesis and the transparency one are dead.
    [switch]$PetEnable,
    # **-PetLayer**: ANSWERED 2026-09-14 - pet[0x26] is the pet's NAME ("Husk..."), not a
    # parent, and it is steady. Attachment is fine. Kept as the control that says so.
    [switch]$PetLayer,
    # **-PetAlpha**: ANSWERED 2026-09-14 - 0xff at the write and on all 357 samples after.
    [switch]$PetAlpha,
    # **-PetParent**: ANSWERED 2026-09-14 [L]. FUN_142934760(user, 1) fires ONCE, during
    # 0x01A0 SetField, from 0x142887193, with the value read out of DAT_143ac87a0+0x70 -
    # the client's own options object. user+0x3fd0 then reads 1 on 5961 of 6000 samples
    # and user+0x3fd8 is a live object. The sprite really is being re-parented.
    [switch]$PetParent,
    # **-PetParentOff**: ANSWERED 2026-09-14 - the rewrite took (user+0x3fd0 = 0 on all
    # 3555 samples) and the Husky stayed invisible. The re-parent theory is dead.
    [switch]$PetParentOff,
    # **-PetLoad**: does the pet ever get FRAMES for its action? FUN_141ec87b0 lazily loads
    # them through FUN_140cd8da0 and falls into FUN_141ec86c0 when there are none. See the
    # $Probe block.
    [switch]$PetLoad,
    [switch]$PetGates,
    # ON BY DEFAULT SINCE 2026-09-14, and accepted only so that every launch line already
    # written down keeps working. It used to be the switch that made the channel answer at
    # all, and forgetting it left the client on "Connecting..." looking like a dead server -
    # which cost a manual launch on 2026-08-20 and was nearly repeated today.
    #
    # Nobody ever wanted it off: all 61 callers in the tree passed it, the shipped
    # installer/start-server.ps1 included. A default nobody chooses is not a safety measure.
    # -SilentChannel is the off case now, and it has to be asked for by name.
    #
    # The run still dumps the EXP curve, and it costs nothing.
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
    # Do NOT send the character list a second time when the client's 0x007A report shows
    # it arrived before the client was ready - the kill switch for the blank-select-screen
    # experiment (login::session::LIST_RESEND_THRESHOLD_MS, plan step TL).
    [switch]$NoListResend,
    [switch]$SetFieldProbe,
    # The channel answers NOTHING: every packet, the migration hello included, gets an empty
    # reply, so the client hangs on "Connecting..." on purpose. This was the DEFAULT until
    # 2026-09-14. Its one honest use is eliminating the channel as a variable.
    [switch]$SilentChannel,
    # Log every mob move (0x02FF), ack (0x03E4) and inbound 0x0070 report as its own line -
    # the 97% of a busy channel log that is COUNTED once a minute by default since
    # 2026-09-14. Pass it for a run where the mob paths are the subject.
    [switch]$LogChatter,
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
    # Arm the pool sentry: a read-only 100 ms watch on the client's own allocator that
    # snapshots a slot the moment its header goes bad, ~106 ms after the write instead of
    # tens of thousands of allocations later. Off by default because it is an instrument,
    # not a fix - it prevents nothing. See crates/grap-stub/src/poolsentry.rs.
    [switch]$PoolSentry,
    # How many dumps a sentry run may write. The 2026-09-06 run caught the write TWICE and
    # could only dump the FIRST - the cap was 1, and the second catch, the one whose slot was
    # freed 720 ms later into the 0xC0000374, left nothing but its log block. A dump is ~1.3 GB
    # and the client freezes for about a second while each one is written, so this is not free;
    # 4 is enough for a fifteen-minute run at the observed rate of one catch per ~3 minutes.
    [int]$SentryDumps = 4,
    # THE MITIGATION, and the only thing in the hook that writes to the client's own memory.
    #
    # On a confirmed damaged header the sentry puts the high dword back to zero, which is the
    # value the pool's carve wrote and the only legal one. That turns the fatal free - the
    # 0xC0000374 this family dies of, where a non-zero high dword sends a pool slot to
    # HeapFree - back into an ordinary free onto the pool's own list.
    #
    # It does NOT stop the writer, and three things it cannot do are in repair_header's doc
    # block: it is a race against a free inside one walk interval, it only sees damage shaped
    # like a pool header (the 0xC0000005 death of 2026-09-06 was a -1 in a map node pointer),
    # and if that dword is a refcount reached through a stale pointer then zeroing it changes
    # what the writer sees next time.
    #
    # Off by default because it writes. Say so in any result that depends on it.
    [switch]$SentryRepair,
    # NAME the writer instead of timing it. For ~1.2 s around each PREDICTED firing of the
    # 180 s clock, bucket 1's pages go PAGE_READONLY: reads are untouched, so the sentry walk
    # and the client's own string reads carry on, and a WRITE faults at the instruction that
    # made it. The handler records RIP and the exact address, makes the page writable and
    # re-executes, so the client keeps going. Nothing is written to the client - VirtualProtect
    # is a permission change, not an edit. It cannot arm until the first catch gives it a
    # phase, so it costs nothing for the first few minutes.
    # crates/grap-stub/src/writewatch.rs. Implies -PoolSentry.
    [switch]$SentryWriteWatch,
    # PLAY MODE: keep the repair, drop everything that makes the client hitch.
    #
    # The owner, 2026-09-07: "it lags/freezes the client every time it runs, which is undesirable."
    # Measured off that session's own heartbeats, the three costs are very different:
    #   ordinary walk            0.63 - 0.71 ms   every 100 ms. Negligible in CPU.
    #   a finding, no dump       49 ms            the 68-thread stack scan
    #   a finding WITH a dump    703 - 895 ms     and the client is frozen for all of it
    # So this sets dumps=0 (nine sentry dumps already exist; a tenth proves nothing),
    # stacks=off, and coarse=2000 - which walks every 2 s except within five seconds of a
    # predicted firing, where it goes back to 100 ms. The period is LEARNED from the findings,
    # so the first two catches are at the fine interval and nothing is assumed.
    #
    # It keeps the repair. What it gives up: no dump if something novel turns up, no thread
    # snapshot, and a first catch no earlier than it would have been anyway. Use -PoolSentry
    # alone when the run is a MEASUREMENT rather than play.
    [switch]$SentryQuiet,
    # Make the LAUNCHER use this -Probe / -Session for its next launch instead of its own
    # defaults, by writing maplecw-hook.probe.pin / .session.pin beside the client.
    #
    # Why this exists: the launcher is the only path that reaches the world - a direct client
    # has not got past character select since 2026-09-05 and three launches went into finding
    # that out - and it overwrites the probe and session markers on every launch. So the
    # launcher's defaults were in practice the only patch set the client could ever run, which
    # blocked two separate measurements: the heap patch control, and a probe slot for
    # 140ca61d0 (the 32-byte array allocator the 180-second ticker family calls).
    #
    # The launcher DELETES a pin before using it, so it is one launch only and the substitution
    # is printed in the launcher's log pane. Use with -ServersOnly: this writes the pin, you
    # start the launcher, the launcher consumes it.
    [switch]$PinPatches,
    [switch]$HeapFix,
    # Refuse the OTHER lethal free: a pool chunk handed to the NT heap by PCOM's WZ property
    # teardown on a map change. That is how process 288744 died on 2026-09-07 with the pool
    # repair on and the live pool provably clean - a different surface from the pooled free
    # the repair covers. It replaces ONE cached function pointer in PCOM's .data (the IAT is
    # not the call site; PCOM does `call rbx` out of PCOM+0xdbb80) and passes every free
    # through untouched unless the pointer's first three qwords read as a pool chunk.
    # OFF by default. Do NOT combine with -SentryWriteWatch on a measurement run: it is one
    # more patch to the client, and "is any of this ours" is the thing that run is measuring.
    # crates/grap-stub/src/freeguard.rs.
    [switch]$FreeGuard,
    # Same guard, but it logs and frees anyway - so the false-positive rate can be measured
    # without changing what the client does. Expect the client to still die on the map change.
    [switch]$FreeGuardObserve,
    # QUARANTINE one pool size class: each allocation of that class gets its own page, and its
    # free DECOMMITS the page and never reuses the address. A stale pointer into freed memory -
    # the writer's habit - then faults at the instruction that uses it, on ANY clock, and the
    # handler logs RIP + who allocated + who freed and recommits so the client runs on. This is
    # the surface runs 2 and 5 died on (a 0x40 map node used +2) that the write watch cannot
    # reach. Needs -PinPatches. Default class 0x40. crates/grap-stub/src/guardpage.rs.
    [switch]$GuardPage,
    # Which size class the guard quarantines: 0x10, 0x20, 0x40 or 0x80. Default 0x40 (bucket 2),
    # the lowest-traffic class and the one with no window coverage. Pair with -SentryWriteWatch
    # on 0x20 and the two surfaces are both covered in one launch.
    # DEFAULT 0x20, not 0x40. The 2026-09-08 death was a 0x20 slot - a red-black tree node
    # whose child pointer had its high dword smashed to -1 - and the pool's HEADERS were
    # perfectly clean at the time. 0x40 was the default until then, chosen from runs 2 and 5,
    # and it would have quarantined the wrong class.
    # Which classes the guard quarantines. One (0x20), several joined with + (0x20+0x40),
    # or `all`. NOT commas - the session marker is comma-separated and a comma would arm half
    # of what was asked for, silently.
    #
    # DEFAULT 0x20+0x40, TWO classes. The writer holds a stale ADDRESS, not a class: the three
    # deaths on record are a 0x40 map node +2, a 0x20 tree node set to -1, and a 0x40 vtable
    # pointer +2 - the last one on 2026-09-08 while 0x20 ALONE was quarantined. Quarantining
    # one class is whack-a-mole. 0x10 and 0x80 are left out because their churn has never been
    # measured; the heartbeat's "pool allocations seen by class" counters are what would
    # justify adding them, and -GuardBucket all is the flag if they do.
    [string]$GuardBucket = '0x20+0x40',
    # Clear user+0x544a. OFF by default and MEASURED INERT on 2026-08-28: the contact path
    # bails before the gate is ever reached, because it is gated on a WZ node no mob has.
    # Kept because it may matter for a mob ATTACK-SKILL hit, which has never been observed.
    [switch]$ClientHitNumberPatch,
    # Leave the Beauty Coupon dialog's item name WHITE (the client's own string 0x0464 colours
    # it 0xffffffff, invisible on this client's white panel). The hook patches six bytes of
    # that encrypted string to black by default; this is the off switch. beautytext.rs.
    [switch]$NoBeautyTextPatch,
    # Leave the fame messages as the client wrote them ("'%s''s level of fame"). The hook
    # rewrites the four templates in place by default; this is the off switch. fametext.rs.
    [switch]$NoFameTextPatch,
    # Redress another player's copy of a changed character with a leave + enter (+ pet) -
    # the sequence that blinks and respawns the pet - instead of the default in-place 0x02AE.
    # The fallback if 0x02AE is refuted on screen. --look-reenter.
    [switch]$LookReenter,
    # Character Info ITEM tab: equips only, no hair/face entries. For a client without the
    # rendered hair/face icons (backport_install.py --install after 2026-09-18 evening).
    [switch]$NoLookItems,
    # Monsters are ON by default since 2026-08-19. -NoMobs turns them off.
    #
    # -Mobs used to be the opt-in, and it cost a launch: the owner stood on map 40, which has
    # six snails, and saw none - the server had them loaded and sent none, because the
    # switch was not passed. It said so only in world-ch0.log.err. Kept as a no-op so an old
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
    # The moveAction byte a summoned pet gets in 0x0277 (0 = the normal value). 30 is the
    # one-byte experiment that sends the client down the only pet arm that sets the pet
    # layer's z. See Config::pet_move_action and plan step v).
    [int]$PetMoveAction = -1,
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
    # **Turn the single-instance work OFF for one run, as a control.**
    #
    # grap-stub patches two user32 functions and redirects kernel32's mutex forwarder so a
    # SECOND client can start. This switch stops all of it, so the client runs exactly as it
    # did before that module existed. Use it whenever something looks wrong and you need to
    # know whether those hooks are why: one launch with, one without, change nothing else.
    #
    # A second client will not start with this on. That is the switch working, not a finding.
    [switch]$NoInstanceHooks,
    [string]$ClientDir
)

$ErrorActionPreference = 'Stop'

# **The channel answers unless -SilentChannel says otherwise.** Until 2026-09-14 this was
# $SetFieldProbe's job and it had to be typed on every launch line; leaving it off produced a
# client stuck on "Connecting..." and one wasted launch. -SetFieldProbe is still accepted -
# it is in STATUS.md, in the fixture notes and in the owner's paste buffer - and now means nothing
# on its own. Everything downstream still reads $SetFieldProbe, so it is simply forced on.
if (-not $SilentChannel) { $SetFieldProbe = $true }

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
# **For the channel-swap run the probe is NOT the instrument - world-ch0.log is.** The swap
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
    if ($PetLoad) {
        # **DOES THE PET EVER GET FRAMES FOR ITS ACTION?**
        #
        # Seven things measured or forced and the sprite is still not on screen, so the
        # remaining question is whether there is anything IN it. Re-reading FUN_141ecaa40
        # after -PetParentOff: a "reset / set from VARIANT / insert into layer" triple whose
        # second arm hands it the USER's object is the shape of an IWzVector2D - the pet's
        # position, relative to the field or to the user - not a canvas. The frames go
        # through the one 8 KB function only skimmed so far, FUN_141ec87b0
        # (research/msexe-pet-postinit.c), and its core is a loop:
        #
        #   action = FUN_141ebe350(pet, &facing)         ; the moveAction decode -> 0 for us
        #   FUN_141ec3380(pet, &action)                  ; may adjust it
        #   arr = [pet+0x360] (or +0x368)                ; per-action frame lists, 24 B each
        #   if (action < 0 || arr == 0 || action >= count(arr)) -> FALLBACK, action = 2
        #   list = arr + action*24
        #   if (list->count != 0) break                  ; frames already cached
        #   FUN_140cd8da0(DAT_143ac0188, template, action, ..., list)   ; LAZY LOAD from WZ
        #   if (list->count != 0) break                  ; loaded
        #   FALLBACK: FUN_141ebdf10(pet) if pet+0x308 >= 0; FUN_141ec86c0(pet); action = 2
        #
        # Every earlier measurement is consistent with that loader returning ZERO frames:
        # the layer is visible, positioned, opaque, drawn (its tag child shows) and empty.
        #
        # 140cd8da0:hits=200    the loader. r8 is the ACTION INDEX it was asked for, rdx the
        #                       template. If it fires and the pet stays blank, it loaded
        #                       nothing or the wrong node
        # 141ec86c0:hits=200    THE FALLBACK. Fires only when the action index is invalid or
        #                       the loader left the list empty. rcx is the pet
        # 141ebdf10:hits=200    the other fallback (pet+0x308 >= 0 arm)
        # 140304100:hits=200    positive control
        #
        # READ IT LIKE THIS:
        #   140cd8da0 fires, then 141ec86c0 fires   -> the loader produced NO FRAMES for
        #                      that action. The bug is between the template and the WZ node
        #                      names, and r8 says which action index. That is the answer
        #   141ec86c0 fires, 140cd8da0 silent       -> the action INDEX was rejected before
        #                      loading; the moveAction decode (0) is out of range for this
        #                      pet's table, so moveAction is back in play after all
        #   140cd8da0 fires, 141ec86c0 silent       -> frames loaded and the pet still does
        #                      not draw. Then the sprite has content and the fault is in
        #                      how it is presented; that would need the layer's rect
        #   neither fires                           -> FUN_141ec87b0 returned at its top
        #                      (user+0x100+0x5ac, which reads 0, so this should not happen)
        #   no 140304100 lines                      -> the hook never armed; not evidence
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,140cd8da0:hits=200,141ec86c0:hits=200,141ebdf10:hits=200,140304100:hits=200:dump=143AC2400/968'
    } elseif ($PetParentOff) {
        # **CAUSATION TEST. THIS RUN PATCHES THE CLIENT** - `:rdx=0` rewrites the second
        # argument of FUN_142934760 on entry, so user+0x3fd0 is stored as 0 instead of the
        # option's 1, and FUN_141eca710 takes its NORMAL arm for the pet's sprite. Describe
        # the result as a patched client, not as the client.
        #
        # What -PetParent established [L]: the setter runs once, at SetField, with the value
        # of DAT_143ac87a0+0x70 - a client OPTION, the same options object whose +0x58 gave
        # the (fine) alpha. So this is not a byte we sent. What that option MEANS is not yet
        # known; its writer is a generic loader that a displacement grep cannot see. This
        # run asks the only question that matters first: is that field the reason the
        # sprite is missing?
        #
        # 142934760:rdx=0:hits=50   the patch, and the log records the ORIGINAL rdx first
        # 140f8abc0:peek=3ed0       proof the field stayed 0 for the whole run
        # 140304100:hits=200        positive control
        #
        # READ IT LIKE THIS - the screen is the instrument this time:
        #   THE HUSKY DRAWS                -> user+0x3fd0 = 1 is the cause. The next work is
        #                      naming the option (and its registry/ini key), and deciding
        #                      whether the fix is the option, a launcher-side setting, or a
        #                      session patch like the ones the launcher already carries
        #   still invisible, peek 3ed0 = 0 -> the field was not it; the arm is a symptom of
        #                      something upstream and the re-parent theory is dead
        #   still invisible, peek 3ed0 = 1 -> the rewrite did not take; say so, nothing was
        #                      measured
        #   no 140304100 lines             -> the hook never armed; not evidence
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,142934760:rdx=0:hits=50,140f8abc0:peek=3ed0:hits=6000,140304100:hits=200:dump=143AC2400/968'
    } elseif ($PetParent) {
        # **THE SPRITE IS BEING RE-PARENTED ONTO SOMETHING THE USER OWNS.**
        #
        # Six things are measured right: alive, sync VISIBLE, frames inserted, enable flag
        # never zeroed, layer opaque, alpha 0xff - and the layer is provably DRAWN, because
        # the name tag is its child. Only the sprite is missing. FUN_141eca710, the function
        # that places the sprite, has two arms (research/msexe-pet-frames.c):
        #
        #   if (user == 0 || *(int *)(user + 0x3fd0) == 0)
        #       NORMAL: variant = the global DAT_143ad4a38, z computed from the position
        #   else if (*(void **)(user + 0x3fd8) != 0)
        #       variant = VT_UNKNOWN(user+0x3fd8 object), z = 1        <- rdx=1
        #   FUN_141ecaa40(pet, z, &variant)  -> sprite->vtbl[0x40](&variant)
        #
        # The -PetFrames capture has 141ecaa40 called from 0x141eca9fc with rdx=1. THAT IS
        # THE SECOND ARM. So user+0x3fd0 is non-zero on the owner's character and the pet's
        # sprite is being attached to whatever lives at user+0x3fd8 - a vehicle, a chair,
        # a morph, something that is drawn INSTEAD of the field for a rider. The owner is riding
        # nothing, so if the client thinks they are, we told it so, in the SetField record or
        # a stat change. This is [D] from one register value; the run below makes it [L].
        #
        # 142934760:hits=50       THE ONLY SETTER of user+0x3fd0 (a virtual, slot 66 of a
        #                         200-slot vtable). rdx is the value, called-from is who
        # 140f8abc0:peek=3ed0     user+0x3fd0 READ - rcx there is user+0x100, so +0x3ed0
        #                         lands on it. ~6000 samples
        # 140f80830:peek=3ed8     user+0x3fd8, the object, same trick from gate 5's rcx.
        #                         Only the calls with called-from=0x141ecde7e are the
        #                         ladder's and carry user+0x100 in rcx; ignore the rest
        # 140304100:hits=200      positive control
        #
        # READ IT LIKE THIS:
        #   142934760 fires, rdx non-zero, before the summon
        #                                   -> paste called-from and the opcode being
        #                      dispatched: that names the PACKET that told the client the owner
        #                      is riding something. Then it is a server fix
        #   peek 3ed0 non-zero, 142934760 silent
        #                                   -> set by a path that is not the virtual; the
        #                      field is still the bug, the writer is not yet named
        #   peek 3ed0 = 0 everywhere        -> the arm was taken for another reason and my
        #                      read of FUN_141eca710 is wrong; paste the 3ed8 peek anyway
        #   no 140304100 lines              -> the hook never armed; not evidence
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,142934760:hits=50,140f8abc0:peek=3ed0:hits=6000,140f80830:peek=3ed8:hits=6000,140304100:hits=200:dump=143AC2400/968'
    } elseif ($PetAlpha) {
        # **THE PET HAS ITS OWN ALPHA, AND NOBODY HAS EVER READ IT.**
        #
        # Two things came out of the Ghidra pass on FUN_141b054f0, and the first reframes
        # the whole problem:
        #
        # 1. `0x3eb` - the constant Init passes it - selects the string
        #    `UI/NameTag.img/pet/%d`. That call BUILDS THE PET'S NAME TAG, and Init hands it
        #    pet+0x3c8, the pet's own layer, as the layer to use. The tag is on the owner's
        #    screen. So the pet's layer is attached, visible and being DRAWN - proved by its
        #    own child - and the thing that is missing is only the sprite inside it.
        #
        # 2. The tail of Init, at 141ebc1b2:
        #        eax = 0x51eb851f; imul ecx; sar edx,5      ; edx = iVar24 * 255 / 100
        #        mov dword [rdi + 0x3c0], edx               ; <- the pet's ALPHA
        #    where iVar24 is *(int *)(DAT_143ac87a0 + 0x58) - a global config PERCENTAGE.
        #    (+0x5c is the other arm, and it is unreachable: it needs user->vtbl[0x50] to
        #    return 0 and that function is `mov eax,1; ret`.)
        #
        # A zero percentage there gives alpha 0: an invisible sprite in a layer that still
        # draws its name-tag child. That is exactly the screen, and it is a DIFFERENT
        # mechanism from FUN_140eeba60, the layer colour, which -PetEnable already cleared.
        # pet+0x3c0 is written once in Init and read by nothing in any pet function dumped
        # so far, so whatever consumes it is in the render.
        #
        # 141ebc1b2:hits=50     THE ALPHA, at the instant it is written. rdx IS the value,
        #                       0..255. If this never fires, DAT_143ac87a0 was null and the
        #                       block was skipped, which is its own answer
        # 141ec7880:peek=3c0    the same field over the pet's whole life, ~700 samples, so a
        #                       value that is right at Init and clobbered later cannot hide
        # 141ecf340:hits=200    the last call in Init and the only one never examined
        # 140304100:hits=200    positive control
        #
        # READ IT LIKE THIS:
        #   141ebc1b2 rdx=0, or the peek reads 0
        #                                 -> THE PET IS DRAWN AT ALPHA ZERO. That is the bug,
        #                      and the next question is what the config at DAT_143ac87a0+0x58
        #                      is meant to hold and who fills it
        #   rdx small but non-zero        -> it is drawn faint, not absent; say whether you
        #                      can see anything at all where the tag is
        #   rdx=0xff / peek=0xff          -> alpha is fine and this is eliminated too
        #   141ebc1b2 never fires         -> DAT_143ac87a0 is null, the alpha keeps whatever
        #                      the constructor left, and the peek is then the only evidence
        #   no 140304100 lines            -> the hook never armed; not evidence
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,141ebc1b2:hits=50,141ec7880:peek=3c0:hits=6000,141ecf340:hits=200,140304100:hits=200:dump=143AC2400/968'
    } elseif ($PetLayer) {
        # **STOP ASKING ABOUT THE PET; ASK WHAT ITS LAYER IS ATTACHED TO.**
        #
        # Five things are now measured working, each on its own capture: the item is alive
        # (dateDead), the sync has it VISIBLE (2964 gate-11 reads of 0, no transition), the
        # frames go in (141ecaa40 x11 with pet+0x3d8 = 0), the enable flag is never zeroed
        # (user+0x100+0x5ac = 0 on 6000 reads, 141ec7970 never fired) and the layer is
        # OPAQUE (140eeba60 touched the pet's layer once, with 0xffffffff; the 1281 calls
        # carrying 0xffffff - alpha 0 - were all on OTHER objects, checked by layer pointer).
        # The sprite and the layer both come from the client's own Gr2D root DAT_143add050,
        # the same singleton everything else that draws uses.
        #
        # So the pet is built correctly and is not on screen, and widening the same
        # instrument again would be the mistake CLAUDE.md names. The one structural link
        # nobody has looked at is the REGISTRATION:
        #
        #   141ebbe68  call FUN_141b054f0(pet, pet[0x26], &layer, ..., 0x3eb, ...)
        #
        # pet[0x26] is pet+0x130, and if it is null the layer is attached to nothing while
        # every other measurement still comes back perfect.
        #
        # 141b054f0:hits=6000    the registration. rdx IS pet[0x26]; the pet's own call has
        #                        called-from=0x141ebbe6d. It is a shared 16 KB function, so
        #                        the cap is large and called-from is what identifies ours
        # 141ec7880:peek=130     pet[0x26] again, 1166 times over the life of the pet, so a
        #                        field that is set at Init and CLEARED later cannot hide
        # 141ec87b0:hits=6000    the post-Init layer work (vtbl[0x268], [0x300]...). Called
        #                        at 141ebbe89, right after SetStance
        # 140304100:hits=200     positive control
        #
        # READ IT LIKE THIS:
        #   141b054f0 with rdx=0            -> the layer is registered into NOTHING. That is
        #                      the bug, and it is the first thing all session that would be
        #   141b054f0 never fires from 0x141ebbe6d
        #                                   -> Init took the other arm and never registered
        #                      the layer at all
        #   rdx non-zero, and 141ec7880's peek goes 0 later
        #                                   -> it is attached at birth and detached after;
        #                      paste the first timestamp where the peek changes
        #   rdx non-zero, peek stays non-zero, 141ec87b0 fires
        #                                   -> attachment is fine too, and I am out of
        #                      structural leads. Say so plainly; the next move is a Ghidra
        #                      pass on FUN_141b054f0 itself rather than another launch
        #   no 140304100 lines              -> the hook never armed; not evidence
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,141b054f0:hits=6000,141ec7880:peek=130:hits=6000,141ec87b0:hits=6000,140304100:hits=200:dump=143AC2400/968'
    } elseif ($PetEnable) {
        # **THE SECOND WRITER TO THE PET'S ENABLE FLAG.** -PetFrames eliminated the frame
        # chain: FUN_141ecaa40 ran 11 times with pet+0x3d8 = 0 every time, so the sprite was
        # reset, given its frames, and inserted into a layer that -PetSync had already
        # measured as visible. Everything the client needs to draw a pet was done, twice
        # over, and nothing drew.
        #
        # Reading CPet::SetStance for that run turned up a SECOND writer. Every periodic
        # call - force 0, stance unchanged, and there were 1163 of them - falls through to:
        #
        #   141ec794a  rcx = user+0x100
        #   141ec7958  call 140f8abc0          ; returns ([rcx+0x5ac] != 0)
        #   141ec795f  je epilogue             ; zero -> do nothing
        #   141ec7970  call [pet->vtbl + 0x18] ; pet->vtbl[0x18](pet, 0, 0)
        #
        # **`vtbl[0x18]` is the same slot the visibility sync sets**: FUN_141ecde00's show
        # path calls `pet->vtbl[0x18](pet, verdict, 0)`. So two things write that flag, and
        # the periodic one always writes ZERO. The sync would never correct it, because what
        # the sync reads back is the LAYER's own flag through vtbl[0x2b0] - a different
        # object - which is exactly why it measured "already visible" 2964 times.
        #
        # **This is a hypothesis, not a measurement.** It only fires when user+0x100+0x5ac is
        # non-zero, and nothing has ever read that field. If it is zero the whole idea is
        # dead, and that is what the first watch is for.
        #
        # 140f8abc0:peek=5ac    THE GATE, read rather than inferred. rcx is user+0x100, so
        #                       the peek prints the exact dword the function tests
        # 141ec7970:hits=6000   the disable call itself. Fires only past that gate
        # 140eeba60:hits=6000   the layer colour setter, rdx is the value. SetStance can pass
        #                       0xffffff - alpha 0, fully transparent - to a pet that is not
        #                       the user's current one. That arm needs stance != 0 so it
        #                       should be silent here, and this watch is what makes "should"
        #                       into "is". It is also the NPC bug's shape, which this project
        #                       has already been caught by once
        # 140304100:hits=200    positive control
        #
        # READ IT LIKE THIS:
        #   140f8abc0 peek 0 everywhere      -> the gate is shut, 141ec7970 cannot fire, and
        #                      this whole idea is dead. Say so; it costs nothing to be wrong
        #                      here and it removes the last lead I have
        #   peek NON-ZERO and 141ec7970 fires repeatedly
        #                                    -> the pet is being disabled ~30 times a second
        #                      by a writer the sync never sees. That is the bug, and the next
        #                      question is what sets user+0x100+0x5ac
        #   peek non-zero but 141ec7970 silent
        #                                    -> the gate is open and the call still does not
        #                      happen; my read of the fall-through is wrong
        #   140eeba60 with rdx=0xffffff      -> something IS making the layer transparent.
        #                      Paste called-from; that is a different bug from the above
        #   no 140304100 lines               -> the hook never armed; not evidence
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,140f8abc0:peek=5ac:hits=6000,141ec7970:hits=6000,140eeba60:hits=6000,140304100:hits=200:dump=143AC2400/968'
    } elseif ($PetFrames) {
        # **WHY THE VISIBLE LAYER HAS NOTHING IN IT.** -PetSync proved the pet is marked
        # visible on all 2964 evaluations and never transitions, so the question moved from
        # "is it shown" to "is there anything to show". A Ghidra pass over CPet::Init
        # (FUN_141eb9760) gives the chain, and it is short - research/msexe-pet-init.c,
        # msexe-pet-action.c, msexe-pet-frames.c:
        #
        #   pet+0x3b8   the sprite, created from the global factory DAT_143add050 vtbl+0x1d8
        #   pet+0x3c8   the layer,  created by vtbl+0x168 and registered by FUN_141b054f0
        #               with 0x3eb; this is the object -PetSync measured as visible
        #   Init then calls FUN_141ec7880(pet, FUN_141ec7e90(pet), 1)   = CPet::SetStance
        #   FUN_141ec7e90 returns a BOOL, not a stance: 1 if FUN_14276e860(user) is non-zero
        #               or the moveAction stance decodes to 8, else 0
        #   SetStance(0) -> FUN_141ec2690, the LAND arm, and returns before the flying path
        #   FUN_141ec2690 -> FUN_141ecaa40, which is where frames actually go in:
        #                    sprite->vtbl[0x20](), sprite->vtbl[0x40](&variant),
        #                    layer->vtbl[0x238](sprite)
        #   FUN_141ecaa40 BAILS OUT AT THE TOP if pet+0x3d8 != 0, clearing the variant and
        #                 inserting nothing at all. That is a real gate and nothing has
        #                 measured it.
        #
        # 141ec7880:hits=2000   CPet::SetStance. rdx is the stance, r8 the force flag. Init
        #                       passes force 1, so a hit at the summon is expected
        # 141ec2690:hits=2000   the land arm. Its only other caller is 141ec60f0
        # 141ecaa40:peek=3d8    THE FRAME INSERT, and the peek reads the exact field its
        #                       early bail tests. rcx is the pet
        # 140304100:hits=200    the equip decode at world entry. POSITIVE CONTROL
        #
        # READ IT LIKE THIS - the deepest one that fires names where the chain stops:
        #   no 141ec7880 at the summon       -> Init never set a stance; the chain never
        #                      starts, and the question is why Init returned early
        #   141ec7880 with rdx=1             -> the client put the pet in the FLYING arm on
        #                      dry land. moveAction is then worth changing; it is 0 today
        #   141ec7880 rdx=0, no 141ec2690    -> SetStance returned before the land arm; the
        #                      early-exit at its top fired even though force was 1
        #   141ec2690 fires, no 141ecaa40    -> pet+0x3d8 or pet+0x3b8 sent it down another
        #                      arm; paste the counts and I will read the other two
        #   141ecaa40 with peek NON-ZERO     -> the bail. pet+0x3d8 is the whole bug, and
        #                      the next question is who sets it
        #   141ecaa40 with peek 0            -> frames WERE inserted into a visible layer and
        #                      it still draws nothing. That would eliminate this entire
        #                      chain, and the hunt moves to the sprite's own contents
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,141ec7880:hits=2000,141ec2690:hits=2000,141ecaa40:peek=3d8:hits=2000,140304100:hits=200:dump=143AC2400/968'
    } elseif ($PetSync) {
        # **THE WHOLE PET SHOW/HIDE SYNC, IN ONE CAPTURE.** Four watches, and every claim
        # below is checkable inside this single run - deliberately, because CLAUDE.md's rule
        # is that a conclusion must not be assembled from two sessions.
        #
        # FUN_141ecde00 is a SYNC, not a verdict, and its registers are unambiguous:
        #     141ecde15  xor r14d, r14d          r14 = 0
        #     141ecde1f  mov edi, r14d           edi = DESIRED = hidden
        #     141ecde27  lea ebp, [r14+1]        ebp = 1
        #     ... every gate that fails jumps to 141ecdf15 with edi still 0 ...
        #     141ecdf0b  call 142cc1e40
        #     141ecdf12  cmove edi, ebp          gate 11 returns 0 -> DESIRED = 1 = VISIBLE
        #     141ecdf15  ebp = the renderable's CURRENT state (or 0 if there is none)
        #     141ecdf56  cmp ebp, edi / je       equal -> return, touch nothing
        #     141ecdf8f  call 14159b0a0(pet+0x40, edi)   <- ONLY on a change; rdx IS the verdict
        #
        # 141ecde00:hits=6000   the ladder ran for the pet at all. rcx is the pet. It fires
        #                       about thirty times a second while one is out, so a summon in
        #                       the first seconds gives thousands of samples.
        # 142cc1e49:peek=24ac   gate 11, READ RATHER THAN INFERRED. 142cc1e40 is
        #                       `[rcx+0x24b0] != 0 || [rcx+0x24ac] != 0`, and 142cc1e49 is the
        #                       second compare - reaching it PROVES +0x24b0 is 0, and peek
        #                       prints +0x24ac. No call has happened yet at that address, so
        #                       called-from still reads the ladder's 0x141ecdf10.
        #                       The old -PetFlags watched the two SETTERS instead, and "no
        #                       setter fired" is not "the field is zero" - the same blind spot
        #                       that hid mob+0x42c behind a lea'd pointer.
        # 14159b0a0:hits=6000   the sync CHANGED something, and rdx says to what.
        # 140304100:hits=200    the equip decode at world entry. POSITIVE CONTROL: no lines at
        #                       all means the hook never armed and nothing here proves anything.
        #
        # READ IT LIKE THIS:
        #   141ecde00 absent                  -> the ladder never ran for the pet; the sync is
        #                      not the mechanism and everything built on it is void
        #   142cc1e49 absent while 141ecde00 fires
        #                                     -> +0x24b0 is NON-ZERO, or a gate before 11
        #                      closed. Re-run -PetGates to say which; gate 11 is not settled
        #   142cc1e49 fires, peek non-zero    -> +0x24ac is the blocker. A field to chase, and
        #                      the first thing to ask is what sets it, since we send no 0x02E2
        #   142cc1e49 fires, peek 0, and 14159b0a0 fires with rdx=1
        #                                     -> the sync turned the pet VISIBLE and it still
        #                      is not on screen: the bug is in DRAWING, not visibility
        #   142cc1e49 fires, peek 0, and 14159b0a0 never fires from 0x141ecdf8f
        #                                     -> desired 1 equals current 1: the client ALREADY
        #                      thinks the pet is visible. Same conclusion as above and stronger -
        #                      the ladder is innocent, closed, and the hunt moves to the
        #                      renderable at pet+0x3c8
        #   14159b0a0 fires with rdx=0        -> something re-hides it; called-from names who
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,141ecde00:hits=6000,142cc1e49:peek=24ac:hits=6000,14159b0a0:hits=6000,140304100:hits=200:dump=143AC2400/968'
    } elseif ($PetGates) {
        # The client's pet show/hide (FUN_141ecde00) is a ladder of gates and every failure
        # jumps to the same label, so the DEEPEST of these that is entered names how far it
        # got. Only calls whose called-from is the ladder count - all four are shared:
        #   142826340 from 0x141ecde63   gates 1..3 passed
        #   140f80830 from 0x141ecde7e   gate 4 passed
        #   1409bd2f0 from 0x141ecdec6   gate 5 passed
        #   142cc1e40 from 0x141ecdf10   gates 8..10 passed -> gate 11 is the blocker
        # Big caps: in run 2 two of them spent 40 hits on other callers before the summon.
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,142826340:hits=6000,140f80830:hits=6000,1409bd2f0:hits=6000,142cc1e40:hits=6000'
    } elseif ($UserState) {
        # 140f810e0 - the only setter of the user state field. EXPECT several lines; read
        #   rdx on each. A value whose (v & ~1) == 0x12 is the state that disables attacking,
        #   the drop-pool clear and the pick-up pre-check all at once.
        # 140304100:hits=200 - the equip decode at world entry. POSITIVE CONTROL.
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,140f810e0:hits=60,140304100:hits=200:dump=143AC2400/968'
    } elseif ($MobTargets) {
        # 141d31b20:args=17 - the melee target collector. EXPECT ONE ENTRY PER SWING; the
        #   pairing against 0x00DF in world-ch0.log is already established, so a missing entry
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
        # cost nothing to read and pair directly with world-ch0.log - CLAUDE.md, "count the same
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
        #   and in world-ch0.log without spending a slot.
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

# **The instance hooks are ON unless this run says otherwise.**
#
# `-NoInstanceHooks` writes the veto `grap-stub` checks at `DllMain`: nothing hooked in user32,
# kernel32's mutex forwarder left alone, the client exactly as it was before that module. A
# second client will then be stopped by its own guard - that is the switch working, not a
# finding. Use it to answer "did OUR hooks do this?" in one launch.
#
# **HERE, and not next to the other marker writes**, which is where it was first put and where
# it did nothing. Two separate reasons, and each alone is fatal:
#
#   * the first attempt landed inside the `if ($Stop)` branch, so the flag only had an effect
#     on the run that stops the servers;
#   * the other marker writes sit below the `-ServersOnly` wait loop, which never returns -
#     and `-ServersOnly` plus the launcher by hand is how every launch now happens.
#
# The owner ran the control, reported the result, and the hook log had all four hooks in it. A
# control that silently does not run returns the SAME answer as one that ran and found
# nothing, which is the more dangerous of the two. This block is above every early exit.
#
# Deleted on every other run rather than merely not written: a control that can be left
# switched on by accident silently un-tests whatever comes after it.
$vetoPath = Join-Path $ClientDir 'maplecw-hook.nomulticlient'
if ($NoInstanceHooks) {
    New-Item -ItemType File -Path $vetoPath -Force | Out-Null
    Write-Host ''
    Write-Host 'INSTANCE HOOKS ARE OFF THIS RUN (-NoInstanceHooks).' -ForegroundColor Yellow
    Write-Host ("  wrote {0}" -f $vetoPath)
    Write-Host '  A SECOND CLIENT WILL NOT START - expected, not the finding.'
    Write-Host '  CHECK IT ACTUALLY TOOK. The hook log must say:'
    Write-Host '    instance: maplecw-hook.nomulticlient is present - NOTHING is hooked'
    Write-Host '  If you see ARMING or "hooked user32" instead, the control did NOT run'
    Write-Host '  and whatever you observe this launch means nothing.'
    Write-Host ''
} else {
    Remove-Item $vetoPath -ErrorAction SilentlyContinue
}
if (-not $Database) { $Database = Join-Path $root 'maplecw.db' }
$exe = Join-Path $ClientDir 'MapleStory.exe'
$loginExe = Join-Path $root 'target\release\maplecw-login.exe'
$worldExe = Join-Path $root 'target\release\maplecw-world.exe'
# The world hub, 2026-09-14: cross-channel parties and party chat go through it (port 8483,
# The owner's number). Started before the channels so they connect on their first dial.
$chatExe = Join-Path $root 'target\release\maplecw-chat.exe'
$chatLog = Join-Path $root 'chat-hub.log'
$ChatPort = 8483
$userAdd = Join-Path $root 'target\release\maplecw-useradd.exe'
$serverLog = Join-Path $root 'login.log'
$worldLog = Join-Path $root 'world-ch0.log'

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
        if (Get-Process maplecw-chat -ErrorAction SilentlyContinue) {
            taskkill /F /IM maplecw-chat.exe | Out-Null
        }
        # The sign-in service, added 2026-08-29. Without this it survives -Stop and holds
        # target\release\maplecw-auth.exe open, so the NEXT build fails with "Access is
        # denied" against a path that says nothing about servers.
        if (Get-Process maplecw-auth -ErrorAction SilentlyContinue) {
            taskkill /F /IM maplecw-auth.exe | Out-Null
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
    # The client credential, if the hook did not get far enough to delete it itself (it
    # deletes it ~1.5s in, on purpose). A STALE one is REFUSED by the login server rather
    # than ignored, so leaving it costs the NEXT run its account - and that failure looks
    # like nothing at all on a one-player machine.
        Remove-Item (Join-Path $ClientDir 'maplecw-hook.identity') -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.sentry') -ErrorAction SilentlyContinue
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
    # **`launcher` is built on EVERY run, including -ServersOnly.** It used to be gated on
    # -Launcher, on the reasoning that eframe is minutes nobody asked for on a run that is
    # not about accounts. That reasoning was right once and produced the exact failure the
    # comment above it warns against, one crate over:
    #
    #   2026-09-01. The instance-guard hooks were written, tested, committed and INSTALLED -
    #   grap64.dll on disk contained them. `crates/launcher/src/client.rs` was edited at 02:17
    #   to write `maplecw-hook.multiclient`, which is what ARMS them. The launcher binary was
    #   from 01:53. The owner ran -ServersOnly and started the launcher by hand, which is how every
    #   launch now happens, so `-p launcher` was never in the build line. The marker was never
    #   written, `instance::enabled()` returned false, `arm()` never ran, and the hook log had
    #   ZERO `instance:` lines. T0 came back "cannot run two clients" from a run in which the
    #   instrument measuring it was switched off, and nothing said so.
    #
    # The first-build cost is real and is paid once. A stale binary costs a launch every time,
    # and this one cost a launch that could not have answered its own question.
    & cargo build --release -p login -p world -p store -p auth -p grap-stub -p launcher
    if ($LASTEXITCODE -ne 0) {
        # **"Access is denied" on a file you own means something has it open**, and cargo does
        # not say what. It is almost always the launcher or a client still running from the
        # previous attempt, and it has stopped a run three times now - once silently, before
        # `-p launcher` was in this line at all.
        #
        # It matters more than an ordinary build failure: the launcher EMBEDS grap64.dll with
        # `include_bytes!` (crates/launcher/build.rs), so a launcher that fails to relink
        # installs the PREVIOUS stub. Carrying on from here would test the old hook and report
        # the result as though it were the new one.
        $holding = Get-Process -Name maplecw-launcher, MapleStory -ErrorAction SilentlyContinue
        if ($holding) {
            Write-Host ''
            Write-Host 'THE BUILD COULD NOT REPLACE A BINARY THAT IS STILL RUNNING.' -ForegroundColor Red
            foreach ($h in $holding) {
                Write-Host ("  {0}  pid {1}  started {2}" -f $h.Name, $h.Id, $h.StartTime) -ForegroundColor Red
            }
            Write-Host '  Close them and run this again. The launcher window counts even'
            Write-Host '  with no client open - it holds its own .exe.'
            Write-Host '  The launcher EMBEDS grap64.dll, so a launcher that did not relink'
            Write-Host '  would install the PREVIOUS hook and the run would measure that.'
            Write-Host ''
            throw 'build failed - close the processes listed above and re-run'
        }
        throw 'build failed'
    }
}
finally { Pop-Location }

# **A launcher built BEFORE grap-stub carries no stub at all, and still says "Finished".**
#
# `crates/launcher/build.rs` embeds `grap64.dll` with `include_bytes!`. On a COLD build - the
# first after `cargo clean` - cargo is free to finish `-p launcher` before `-p grap-stub` has
# produced the dll, and build.rs handles that case by printing a `cargo:warning` and compiling
# WITHOUT the stub. The build still ends "Finished". The launcher then starts the client,
# installs nothing, and the run measures an UNHOOKED client while looking entirely normal: no
# instance guard, no WATCH lines, no crash dump, no session patches.
#
# That is the same failure as the stale 01:53 launcher noted further down, arriving by a
# different route, and it is worse in one way - a stale binary at least contains A stub.
#
# Measured 2026-09-04, on the first build after the `cargo clean` the owner asked for:
# 4,653,056 bytes cold against 5,038,080 once relinked. The 385 KB difference is grap64.dll,
# which is 386,048 bytes.
#
# Checked by CONTENT rather than by SIZE, because a size threshold is a magic number that goes
# stale the first time either binary changes - and it would go stale by passing.
# 'MapleCW-client-instance' is the stub's own mutex name (crates/grap-stub/src/instance.rs)
# and appears in the launcher only if the stub is inside it.
$launcherBin = Join-Path $root 'target\release\maplecw-launcher.exe'
$stubMark = 'MapleCW-client-instance'
$launcherHasStub = $false
if (Test-Path $launcherBin) {
    $launcherHasStub = [System.Text.Encoding]::ASCII.GetString(
        [System.IO.File]::ReadAllBytes($launcherBin)).Contains($stubMark)
}
if (-not $launcherHasStub) {
    # Repaired rather than reported, because a guard that hands back a paste-able command is a
    # guard whose answer can be ignored, and this script has already built everything it needs
    # to fix it. build.rs is touched because cargo otherwise considers the launcher fresh and
    # will not re-run the script that does the embedding.
    Write-Host ''
    Write-Host 'THE LAUNCHER EMBEDS NO STUB - relinking it now.' -ForegroundColor Yellow
    Write-Host '  It compiled before grap-stub produced grap64.dll, which build.rs reports'
    Write-Host '  only as a cargo:warning in a build that ends "Finished".'
    (Get-Item (Join-Path $root 'crates\launcher\build.rs')).LastWriteTime = Get-Date
    Push-Location $root
    try { & cargo build --release -p launcher }
    finally { Pop-Location }
    $launcherHasStub = (Test-Path $launcherBin) -and [System.Text.Encoding]::ASCII.GetString(
        [System.IO.File]::ReadAllBytes($launcherBin)).Contains($stubMark)
    if (-not $launcherHasStub) {
        Write-Host ''
        Write-Host 'THE LAUNCHER STILL EMBEDS NO STUB. STOP.' -ForegroundColor Red
        Write-Host '  A run from here would install no hook and measure an unhooked client.'
        Write-Host '  Check that grap-stub actually built:'
        Write-Host ("    Get-Item `"{0}\target\release\grap64.dll`"" -f $root) -ForegroundColor Yellow
        Write-Host ''
        throw 'the launcher embeds no stub - it would install no hook'
    }
    Write-Host '  relinked, and the stub is in it.' -ForegroundColor Green
}

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
# **Three separate conclusions have died with an overwritten world-ch0.log.** The most expensive:
# an attack capture showing 127-byte zero-target bodies was read out of a world-ch0.log that also
# carried 90 mob-control packets, the pair was reported as "the client will not target our
# mobs", and by the time anyone tried to re-check it the file had been replaced by the next
# launch. No fixture had been taken. The observation was real and is now unverifiable, which
# is the worst of both.
#
# A client launch costs the owner a manual launch. Throwing away its output to save a few hundred
# kilobytes is the wrong trade in every direction.
# `-Into` exists so the HOOK log can be archived next to the server logs rather than into a
# second buffer under client-patched\. `CLAUDE.md`'s "count the same event in two logs"
# needs both halves of one run in one place: world-ch0.log says what the server SENT, the hook
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
# Only the direct-client path does this, because only there is the claim a trap: the
# launcher path is about to stake a fresh one, and -ServersOnly hands over to the launcher.
if ($DirectClient) {
    $userAddExe = Join-Path $root 'target\release\maplecw-useradd.exe'
    if (Test-Path $userAddExe) {
        & $userAddExe --db "$Database" --clear-claims | ForEach-Object { Write-Host "  $_" }
    }
}

# HOW THE SERVERS ARE ATTACHED, and it decides whether closing this window stops them.
#
# `-WindowStyle Hidden` gives each child its OWN hidden console. That is right for a run this
# script drives to completion - the client is the thing on screen - but it also means the
# children are attached to no console anybody can close, which is exactly why `-Stop` had to
# exist as a separate step.
#
# `-NoNewWindow` shares THIS console. Closing the window then sends CTRL_CLOSE_EVENT to every
# process attached to it, children included, and Windows gives them five seconds to go. That
# is what makes "one shell, close it to stop the server" work, so -ServersOnly uses it.
#
# MEASURED 2026-08-28, both ways, because the whole feature rests on it. A harness spawned a
# long-lived child and its console window was closed with a real WM_CLOSE:
#
#   -NoNewWindow        child gone after the close        <- what -ServersOnly does
#   -WindowStyle Hidden child STILL RUNNING after it      <- the control, and the old behaviour
#
# The control is the half that matters: it is why a separate stop script had to exist, and it
# shows the difference is this switch rather than something incidental about the close.
$spawn = if ($ServersOnly) { @{ NoNewWindow = $true } } else { @{ WindowStyle = 'Hidden' } }

# One address per channel. The client connects to this when it enters the world, so it must
# be reachable from the *client* machine - loopback here, and --advertise decides elsewhere.
# The -join that builds $channelList MUST be fully parenthesised: PowerShell's -join binds
# looser than the commas of an array literal, so an unparenthesised one swallows the rest of
# the argument list into one string and the server sees a single argument "--db,...".
$loginArgs = @(
    '--db', "`"$Database`"", '--bind', "127.0.0.1:$Port",
    '--channels', $channelList,
    '--display-name', "`"$DisplayName`"", '--world', $World,
    '--bind-migrations', $BindMigrations
)
# LOGIN ENFORCED unless -FallbackAccount says otherwise. No --account: it selected the
# fallback until 2026-09-05, and a default run should never be able to reach a character
# list without somebody having signed in through the launcher.
if ($FallbackAccount) { $loginArgs += @('--fallback-account', $FallbackAccount) }
if ($NoListResend) { $loginArgs += '--no-list-resend' }
$server = Start-Process -FilePath $loginExe -WorkingDirectory $root -PassThru @spawn `
    -ArgumentList $loginArgs `
    -RedirectStandardOutput $serverLog -RedirectStandardError "$serverLog.err"

# THE SIGN-IN SERVICE. Started with the other two, and that is not optional any more.
#
# The launcher no longer opens maplecw.db - the owner, 2026-08-29: "on a client machine you won't
# have access to the project or the database." Sign-in is an HTTP POST to this service, so a
# dev run without it cannot log in either. Starting it here means the dev box exercises
# exactly the path an installed machine does, rather than testing one and shipping the other.
#
# Bound to loopback here. An installed server box passes --bind 0.0.0.0; see start-server.ps1.
$authLog = Join-Path $root 'auth.log'
Save-PreviousLog $authLog
Remove-Item $authLog -Force -ErrorAction SilentlyContinue
# **THE LOCAL CLIENT IS PUBLISHED TOO, and its gate byte is set BEFORE the scan.**
#
# Since 2026-09-14 the launcher refuses to start the game until it has confirmed its client's
# version against maplecw-auth, so a dev run whose auth service publishes nothing cannot press
# Start Game at all. `--client-dir` here is $ClientDir, which is client-patched\ - the very
# folder the launcher is about to check, so locally the two are the same directory and the
# check is always a pass.
#
# **Order is the whole trick.** maplecw-auth hashes the folder ONCE at startup. The launcher
# patches one byte of MapleStory.exe (the Nexon gate) at Start Game, which is after that. If
# the byte were still unset when auth scanned, the manifest would carry the pre-patch hash, the
# launcher would see its own patched exe as out of date, download the file - and get the
# PATCHED bytes off disk, which do not match the hash the manifest promised. The launch would
# be refused with "the patch did not survive the download", and the cause would be four steps
# away from the symptom. Setting the byte here, before the scan, removes the window entirely.
# crates/launcher/src/client.rs owns the same constants; tools/package-server.ps1 does the same
# thing to the staged copy for the same reason.
$clientExeForPatch = Join-Path $ClientDir 'MapleStory.exe'
if (Test-Path $clientExeForPatch) {
    $fs = [System.IO.File]::Open($clientExeForPatch, 'Open', 'ReadWrite')
    try {
        $gate = New-Object byte[] 4
        $fs.Position = 0xd90388
        [void]$fs.Read($gate, 0, 4)
        $gateHex = ($gate | ForEach-Object { $_.ToString('x2') }) -join ' '
        if ($gateHex -eq '85 c0 75 05') {
            $fs.Position = 0xd9038a
            $fs.WriteByte(0xEB)
            Write-Host 'gate-patched client-patched\MapleStory.exe (75 -> EB) before the scan, so' -ForegroundColor Green
            Write-Host '  the published manifest matches what the launcher will have on disk'
        } elseif ($gateHex -ne '85 c0 eb 05') {
            Write-Host "NOTE: the Nexon gate bytes are '$gateHex', not the build these offsets" -ForegroundColor Yellow
            Write-Host '  were measured on. Left alone; the launcher will say the same thing.' -ForegroundColor Yellow
        }
    }
    finally { $fs.Close() }
}

$authExe = Join-Path $root 'target\release\maplecw-auth.exe'
if (Test-Path $authExe) {
    $authArgsLocal = @('--db', "`"$Database`"", '--bind', '127.0.0.1', '--port', "$AuthPort")
    # Publish the client, so the launcher's version check can pass. Without it every Start Game
    # is refused with "this server publishes no client".
    if (Test-Path (Join-Path $ClientDir 'MapleStory.exe')) {
        $authArgsLocal += @('--client-dir', "`"$ClientDir`"")
    } else {
        Write-Host "NO CLIENT AT $ClientDir - the launcher will REFUSE to start the game," -ForegroundColor Red
        Write-Host '  because it cannot confirm which version to run. -DirectClient still works.' -ForegroundColor Red
    }
    $authSrv = Start-Process -FilePath $authExe -WorkingDirectory $root -PassThru @spawn `
        -ArgumentList $authArgsLocal `
        -RedirectStandardOutput $authLog -RedirectStandardError "$authLog.err"
    # TLS: the service writes auth-cert-fingerprint.txt at the repo root (beside its db), and a
    # dev-layout launcher reads it from there - nothing to copy on this machine.
    Write-Host "sign-in service on 127.0.0.1:$AuthPort (pid $($authSrv.Id), TLS; the launcher pins auth-cert-fingerprint.txt from the repo root), log $authLog"
} else {
    Write-Host "NO SIGN-IN SERVICE at $authExe - the launcher cannot log in." -ForegroundColor Red
    Write-Host "  cargo build --release -p auth" -ForegroundColor Red
}

$worldSrv = $null
# Every channel, not just channel 0. -ServersOnly watches all of them: a channel that dies
# leaves the login screen working and the world unreachable, which is the confusing half.
# The hub first. `cargo build -p world` builds it beside maplecw-world (same crate, second
# binary), so it cannot be stale relative to the channels that dial it.
Save-PreviousLog $chatLog
Remove-Item $chatLog -Force -ErrorAction SilentlyContinue
$chatSrv = Start-Process -FilePath $chatExe -WorkingDirectory $root -PassThru @spawn `
    -ArgumentList @('--bind', "127.0.0.1:$ChatPort") `
    -RedirectStandardOutput $chatLog -RedirectStandardError "$chatLog.err"
Write-Host "world hub (maplecw-chat) on 127.0.0.1:$ChatPort (pid $($chatSrv.Id)), log $chatLog"

$worldAll = @()
foreach ($ch in 0..($Channels - 1)) {
    $chPort = $ChannelPort + $ch
    # **Every channel is named the same way**, channel 0 included. It used to be the
    # odd one out - `world.log` while channel 1 was `world-ch1.log` - which made a
    # two-channel server look like one channel plus a mystery file. The owner, 2026-09-14.
    $chLog = Join-Path $root "world-ch$ch.log"
    Save-PreviousLog $chLog
    Remove-Item $chLog -Force -ErrorAction SilentlyContinue
    # The rolls too (world-chN.log.1 .. .5 - the server rolls its own file at 50 MB now), and
    # the stdout stub, so a run starts from an empty slate and nothing is confused for new.
    foreach ($roll in (Get-ChildItem -Path $root -Filter "world-ch$ch.log.*" -ErrorAction SilentlyContinue)) {
        if ($roll.Name -match '\.log\.\d+$') { Save-PreviousLog $roll.FullName }
        Remove-Item $roll.FullName -Force -ErrorAction SilentlyContinue
    }
    $chArgs = @('--db', "`"$Database`"", '--bind', "127.0.0.1:$chPort", '--channel', "$ch")
    # Every channel is told where every channel listens, because Change Channel (0x00D2)
    # arrives on the CHANNEL connection and has to be answered with the target's address.
    # Same list the login server advertises, built from the same two numbers, so the two
    # cannot drift into advertising a channel nobody can enter.
    $chArgs += @('--channels', $channelList)
    $chArgs += @('--link', "127.0.0.1:$ChatPort")
    # The channel answers by default now; only the deliberate silence needs a flag.
    if ($SilentChannel) { $chArgs += '--silent-channel' }
    if ($LookReenter) { $chArgs += '--look-reenter' }
    if ($NoLookItems) { $chArgs += '--no-look-items' }
    # The channel writes and ROLLS its own log now (50 MB, five kept). Stdout gets nothing
    # after the file opens; it is redirected to a .out stub so nothing is lost if it does.
    $chArgs += @('--log-file', "`"$chLog`"")
    if ($LogChatter) { $chArgs += '--log-chatter' }
    if ($NoMobs) { $chArgs += '--no-mobs' }
    if ($MobLimit -gt 0) { $chArgs += @('--mob-limit', "$MobLimit") }
    if ($ShopRows -gt 0) { $chArgs += @('--shop-rows', "$ShopRows") }
    if ($InventorySlots -gt 0) { $chArgs += @('--inventory-slots', "$InventorySlots") }
    if ($PetMoveAction -ge 0) { $chArgs += @('--pet-move-action', "$PetMoveAction") }
    $p = Start-Process -FilePath $worldExe -WorkingDirectory $root -PassThru @spawn `
        -ArgumentList $chArgs `
        -RedirectStandardOutput "$chLog.out" -RedirectStandardError "$chLog.err"
    if ($ch -eq 0) { $worldSrv = $p }
    $worldAll += $p
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

# ---------------------------------------------------------------- the test plan
#
# **This is one of the TWO copies CLAUDE.md requires, and it is the one the owner actually
# reads.** The other is the .NOTES block at the top of this file. Update both together,
# then RENDER this one and read it - a parse check does not catch a plan that is simply
# out of date, and it does not catch a quoting bug that mangles the text.
#
# It is a FUNCTION because it has two callers, and for a while it had none that the owner
# ever saw: the body used to sit after the client launch, so -ServersOnly - which is
# what start-servers.cmd uses, and therefore how every launch now happens - returned
# before reaching it. The plan silently stopped printing and nobody noticed until the owner
# asked where it had gone. One copy, two callers; do not inline it back.
function Show-TestPlan {
    Write-Host ''
    Write-Host 'On screen:'
    if ($SetFieldProbe) {
        Write-Host '  TWO CLIENTS PLAY TOGETHER NOW. T0, T1 and T2 are ANSWERED.' -ForegroundColor Green
        Write-Host '    They see each other move, attack, and take damage. 0x0224'
        Write-Host '    killed both clients three times on the way - the stat tail'
        Write-Host '    (7 -> 23), then a SEAT INDEX at body 416 where 0 is a valid'
        Write-Host '    seat, then position/foothold/facing. All fixed.'
        Write-Host ''
        Write-Host ''
        Write-Host '  #############################################################' -ForegroundColor Cyan
        Write-Host '  ##          THIS RUN IS ONE THING:  !scroll                 ##' -ForegroundColor Cyan
        Write-Host '  #############################################################' -ForegroundColor Cyan
        Write-Host '  MESOS - ANSWERED. "meso dropping is fine now, inventory is' -ForegroundColor Green
        Write-Host '          fine as well after meso dropping." M0-M7 struck.' -ForegroundColor Green
        Write-Host '  MAP CHAIRS - ANSWERED. "I tested the map chairs with two' -ForegroundColor Green
        Write-Host '          clients, that is all working now." C1-C4 struck.' -ForegroundColor Green
        Write-Host '  Also do NOT re-test Set Up chairs, Shanks, !tool, 12h login.' -ForegroundColor Green
        Write-Host ''
        Write-Host '  !scroll was REBUILT after the last run. Five open questions,'
        Write-Host '  every one of them a CLIENT question the server cannot answer'
        Write-Host '  for itself. It is step TS below. Do TS(a) FIRST, before you'
        Write-Host '  !item anything - granting a scroll destroys what (a) tests.'
        Write-Host '  IF THE CLIENT DIES, SAY WHICH STEP YOU WERE ON.' -ForegroundColor Red

        Write-Host '  TV. PHIL''S JOB GUIDE IS A REAL MENU NOW (type 6, like the taxis).' -ForegroundColor Magenta
        Write-Host '      Level-10 beginner clicks Phil (Lith Harbor): one list whose lines'
        Write-Host '      highlight under the cursor and can be CLICKED -> fixed.'
        Write-Host '      Still a Yes/No box -> the old chain, report it.'
        Write-Host '        pick Magician -> arrival line, Grendel''s map, still a Beginner'
        Write-Host '        Close -> nothing, you stay in Lith Harbor'
        Write-Host ''
        Write-Host '  TR. THE FIRST JOB ADVANCEMENT ASKS FIRST.' -ForegroundColor Magenta
        Write-Host '      An eligible click opens a yes/no box naming the requirements and'
        Write-Host '      the one-way warning; the job changes ONLY on Yes.'
        Write-Host '        No -> "take your time", job unchanged; Yes -> advanced -> fixed'
        Write-Host '        a second click after advancing -> the refusal, no box'
        Write-Host '        Yes also hands over the Beginner''s weapon (Grendel: Wooden Wand;' -ForegroundColor Magenta
        Write-Host '        the Rogue gets Zamadar AND Garnier): named in the box, grey'
        Write-Host '        "gained" line, in the Equip tab -> fixed'
        Write-Host ''
        Write-Host '  TX. CONSOLIDATE ITEM WORKS (2026-09-18): stacks merge up to slotMax, then' -ForegroundColor Magenta
        Write-Host '      everything slides up with no gaps, order kept. Plain 0x0070s, no new opcode.'
        Write-Host '        1. Use tab as on your screenshot: click it -> the blue potion sits right'
        Write-Host '           after the orange, no gap, no New mark, tab still usable -> fixed'
        Write-Host '        2. split a stack, click: one full stack + the remainder AFTER it -> fixed'
        Write-Host '        3. nothing to do: nothing changes, tab still usable -> fixed'
        Write-Host '        4. SORT ITEMS: consolidate, then BIGGEST stack first, then name A-Z.' -ForegroundColor Magenta
        Write-Host '           Your Use tab: 350 arrows, 325, blue 100, red 54, orange 21, orange 7,'
        Write-Host '           scroll 6, apple 3 -> fixed. Wrong direction -> say so (one comparator).'
        Write-Host '           Two items in one slot or a blank -> the swap did not draw; paste "sort"' -ForegroundColor Yellow
        Write-Host '        5. DRAG ONTO THE SAME ITEM: 40 red onto 70 -> 100 and 10 (fills first);' -ForegroundColor Magenta
        Write-Host '           the 10 onto the 100 -> they swap (full); split 4 off into an empty'
        Write-Host '           slot -> 4 there, rest stay -> fixed. Was: the screen swapped while'
        Write-Host '           the rows merged. A stack that "comes back" -> paste MERGE/SPLIT lines' -ForegroundColor Yellow
        Write-Host '        frozen tab -> paste the "consolidate"/"sort" lines; client dies -> client-exit.log' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  TW. A LEADER WHO LEAVES THE GAME HANDS THE PARTY TO ITS HIGHEST-LEVEL MEMBER.' -ForegroundColor Magenta
        Write-Host '      Log out or close the client as leader: the highest-level ONLINE member'
        Write-Host '      leads and you STAY in the list; log back in: your window is rebuilt at'
        Write-Host '      login (0x0D, first time at login - watch it) -> fixed. The LAST member'
        Write-Host '      online to log out, leader or not, ends the party: nobody who logs back'
        Write-Host '      in has a window. A channel change: nothing.'
        Write-Host '        join order won / no window on relogin / a window after everyone' -ForegroundColor Yellow
        Write-Host '        was offline -> paste the "party: character N" lines' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  TQ. ACROSS CHANNELS: THE WORLD HUB (maplecw-chat, 8483, chat-hub.log).' -ForegroundColor Magenta
        Write-Host '      Every channel dials it. Party requests are echoed to every'
        Write-Host '      channel in one order (a party now exists across channels);'
        Write-Host '      a packet for someone on another channel is forwarded to them.'
        Write-Host '      Two clients, one party, on DIFFERENT channels:'
        Write-Host '        1. party chat either way crosses -> fixed; nothing -> chat-hub.log'
        Write-Host '        2. the party window on both still lists both -> registry ok'
        Write-Host '        3. invite someone on the other channel: dialog opens there'
        Write-Host '        4. Pick-up rights: both see "changed to ..." on both channels'
        Write-Host '        5. whisper (/w Tester2 hi), same channel then other channel:'
        Write-Host '           they see the line, you see "Tester2<< hi" -> fixed'
        Write-Host '           "Could not find Tester2." -> not in the directory (chat-hub.log)'
        Write-Host '           a client DIES on receipt -> say which' -ForegroundColor Red
        Write-Host '        6. /find Tester2 -> "Tester2 is on channel N." (a plain line)'
        Write-Host '        7. Maple Chat (same channel): invite Tester2, Accept on Tester2:'
        Write-Host '           the owner sees their own avatar at once; Tester2''s window opens with'
        Write-Host '           both; the owner''s gains Tester2 -> fixed'
        Write-Host '           windows open, seats empty -> the look is not drawn; say whose'
        Write-Host '           a client DIES on the accept -> say which' -ForegroundColor Red
        Write-Host '           then type a line and close the window: both are CAPTURES.' -ForegroundColor Yellow
        Write-Host '      Buddy list: NOT built - open it and add someone once; the bytes' -ForegroundColor Yellow
        Write-Host '      land in world-chN.log as UNKNOWN, which is the capture it needs.' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  TP. PARTY CHAT, AND THE PICK-UP RIGHTS BUTTON. (Party mesos: DONE.)' -ForegroundColor Magenta
        Write-Host '      Party chat (0x0179) went unanswered; it now reaches every other'
        Write-Host '      member on this channel, any map, as the client''s 0x01B1 - and'
        Write-Host '      across channels through the hub (TQ).'
        Write-Host '      Pick-up rights: the button sends no value - it is a TOGGLE - and'
        Write-Host '      0x2D is the client''s rights-changed packet (1 Leader, 0 All).'
        Write-Host '        1. type in party chat: the other client sees it -> fixed;'
        Write-Host '           nothing -> log line "-> N of M member(s)"; DIES -> say so' -ForegroundColor Red
        Write-Host '        2. leader clicks Pick-up rights: both see "changed to Party'
        Write-Host '           Leader", label follows; again -> "to All" -> fixed'
        Write-Host '           line and label disagree -> the byte is reversed; say which'
        Write-Host '        3. under Party Leader the member cannot take a party drop'
        Write-Host ''
        Write-Host '  TO. PARTY MESOS: 70% TO THE PICKER, A YELLOW COPY TO EVERY MEMBER.' -ForegroundColor Magenta
        Write-Host '      A party member picks up a MOB''s mesos: 70% to them (white), and'
        Write-Host '      every other member on the map gets a copy of the 30% share with'
        Write-Host '      the client''s own yellow line, "Spotting Small Change (+n)".'
        Write-Host '      Player-dropped mesos: 100% to the picker, no share. Two clients'
        Write-Host '      in a party, same map, same channel. Kill, pick up the mesos:'
        Write-Host '        picker white +70%, other YELLOW "Small Change" +30% and the'
        Write-Host '          counter rises -> fixed'
        Write-Host '        other has the line, counter still -> the 0x007C is missing'
        Write-Host '        other has a WHITE line -> share over 65,535 fell back; say it'
        Write-Host '        other gets nothing -> world-ch0.log: was the share mailed?'
        Write-Host '      Then drop mesos yourself and pick them up: 100%, no member line.'
        Write-Host ''
        Write-Host '  TN. THE SUMMONING SACK: THE BALROG MOVES, AND ARRIVES WITH THE CIRCLE.' -ForegroundColor Magenta
        Write-Host '      04:21: the sack''s Balrog had no AI and no summon effect. The sack'
        Write-Host '      spawned it and granted nobody control (0x03D2) - it does now. And'
        Write-Host '      the spawn carries the WZ summonType: the client plays the'
        Write-Host '      Summon.img circle for everyone and holds the mob untargetable'
        Write-Host '      until a 0x03E8, which the server sends when the animation ends.'
        Write-Host '      Use a Balrog sack on an empty platform:'
        Write-Host '        circle, THEN it walks/attacks, hittable after ~2.5 s -> fixed'
        Write-Host '        no circle but it moves -> appear type ignored; say so'
        Write-Host '        circle, never moves, cannot be hit -> the 0x03E8 did not'
        Write-Host '          clear it; world-ch0.log says if it went out 2500 ms later'
        Write-Host '        moves but unhittable -> same; say so'
        Write-Host '        client DIES at the spawn -> the appear-option word' -ForegroundColor Red
        Write-Host ''
        Write-Host '  TM. THE OUTFIT IS IN THE DECO TAB; THE CHAT SAYS "UBEL"..' -ForegroundColor Magenta
        Write-Host '      04:12: the Ubel set went to the EQUIP tab by leading digit; the'
        Write-Host '      client keeps cash equips (info/cash = 1) in tab 6, Deco. The'
        Write-Host '      server now places by the WZ flag, restores Deco on field entry,'
        Write-Host '      and moves strays out of Equip first. Chat is folded to ASCII.'
        Write-Host '      Log in, change map once:'
        Write-Host '        Deco shows Clothes, Shoes, Gloves, Weapon -> fixed'
        Write-Host '        Deco empty, Equip still has them -> not flagged; say so'
        Write-Host '        Deco empty AND Equip lost them -> Deco Add not drawn' -ForegroundColor Red
        Write-Host '      Then in the shop, double-click Ubel''s Clothes (Deco tab):'
        Write-Host '        goes to the Cash Inventory -> 0x0B with tab 6 works'
        Write-Host ''
        Write-Host '  TL. BAG -> LOCKER, EVERY CASH ITEM NOW CARRIES A SERIAL..' -ForegroundColor Magenta
        Write-Host '      Run 6 (04:03): "nothing moves back" and NO 0x03E1 in world-ch0.log -'
        Write-Host '      the client sent nothing: bag-restored items had +0x38 = 0 and'
        Write-Host '      the double-click builder skips those. Now every Cash-tab body'
        Write-Host '      (field entry, shop entry, every Add) carries a serial. A Cash'
        Write-Host '      BUNDLE body is 8 bytes longer - the one variable.' -ForegroundColor Yellow
        Write-Host '      Enter the shop. Item Inventory EMPTY -> the longer body; say so.' -ForegroundColor Red
        Write-Host '        1. double-click a Mystery Hair Coupon (never moved before):'
        Write-Host '           in the locker, gone from Item Inventory -> fixed'
        Write-Host '           nothing at all -> the builder still saw 0; world-ch0.log has the body'
        Write-Host '           in the locker AND still in Item Inventory -> lookup missed;'
        Write-Host '             say if anything ELSE in the Cash tab vanished (slot 0)'
        Write-Host '           "unknown error" -> refused; world-ch0.log says why'
        Write-Host '           client DIES -> say so; the longer body is the suspect' -ForegroundColor Red
        Write-Host '        2. drag it out again -> should move'
        Write-Host '        3. exit, check the inventory Cash tab; re-enter, both panels'
        Write-Host '           agree with what you moved'
        Write-Host '      (Run 2 and run 1 lines, for the record:)' -ForegroundColor DarkGray
        Write-Host '        -PinPatches -Probe "watch@1415db360:ret,141b2a280:rdx=0,1410b6060,1417113f0:hits=200,14170fd10:hits=400,1410b6970"' -ForegroundColor DarkGray
        Write-Host '        -PinPatches -Probe "watch@1415db360:ret,141b2a280:rdx=0,140d7e1f0,140d75850,1410b5540,14170fd10"' -ForegroundColor DarkGray
        Write-Host '      -PinPatches IS NOT OPTIONAL: the launcher overwrites the probe' -ForegroundColor Red
        Write-Host '      marker on every launch; a pin is the only way in. The 23:18' -ForegroundColor Red
        Write-Host '      run without it armed the defaults and measured nothing.' -ForegroundColor Red
        Write-Host '      PROOF it took: the launcher pane prints OVERRIDES, and the'
        Write-Host '      hook log has "probe: watching 0x140d7e1f0". Neither = not'
        Write-Host '      instrumented; say so. (First two = mandatory patches; 6 slots.)'
        Write-Host '      Enter the Cash Shop once, exit. Then read the hook log:'
        Write-Host '        140d7e1f0 0x04 handler   -> 1 hit'
        Write-Host '        140d75850 map insert     -> one per row (6)'
        Write-Host '        1410b5540 repaint        -> >= 1'
        Write-Host '        14170fd10 row widget     -> one per row (6)'
        Write-Host '        all as expected -> rows exist, invisible; the draw is next'
        Write-Host '        insert but no repaint -> the panel pointer was null then'
        Write-Host '        no insert at all -> the 0x04 body is not what the handler reads'
        Write-Host '        client DIES -> say so; this is the ONE variable' -ForegroundColor Red
        Write-Host '      NOTHING ELSE on this run. Also: the Cash tab glow on every'
        Write-Host '      shop exit is fixed - all tabs restore quietly now (mode 5).'
        Write-Host '      A tab that comes up EMPTY names that change; say which.' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  TK. THE STATION: THE DOOR, THEN THE CLOCK.' -ForegroundColor Magenta
        Write-Host '      a) THE DOOR - CONFIRMED: "I can indeed press up at the' -ForegroundColor Green
        Write-Host '         correct location and be teleported." STRUCK. It has no' -ForegroundColor Green
        Write-Host '         picture on purpose (pt 8 has no game graphic); the arch' -ForegroundColor Green
        Write-Host '         is the door. TF is TESTABLE NOW - same packet.' -ForegroundColor Green
        Write-Host '      b) THE CLOCK, once in the station. It sat at 00:00 because'
        Write-Host '         nothing sent a time. 0x01BC type 1 (h, m, s) now carries'
        Write-Host '         UTC per your correction - the same time world-ch0.log stamps.'
        Write-Host '           UTC (taskbar + 4h), AM/PM right, and it TICKS -> done'
        Write-Host '           your taskbar time -> local got through; serverclock.rs'
        Write-Host '           still 00:00 -> grep world-ch0.log for 0x01BC on the entry.'
        Write-Host '                       sent = the client ignored it; not sent ='
        Write-Host '                       the map is not in clocks.txt'
        Write-Host '           client DIES on entry -> the widget was missing when the' -ForegroundColor Yellow
        Write-Host '                       packet came, which the decode says throws.'
        Write-Host '                       Not expected on any map in clocks.txt.'
        Write-Host ''
        Write-Host '  TG. JOSIAH TOP AT CHARACTER SELECT - one discriminator.' -ForegroundColor Magenta
        Write-Host '      "Cobalt is female" was WRONG: the database and the client'
        Write-Host '      own MakeCharInfo both say male. The one difference left:'
        Write-Host '      Blue Sergeant needs STR 30 / DEX 10; Cobalt has 27 / 5 after'
        Write-Host '      the AP reset. Nothing else they wear requires anything.'
        Write-Host '      SUPERSEDED by your next ask: the select SHEET now shows' -ForegroundColor Yellow
        Write-Host '      equipment totals like the in-game window. Log out to the' -ForegroundColor Yellow
        Write-Host '      select screen and look at TWO things:' -ForegroundColor Yellow
        Write-Host '        sheet reads STR 1026 / DEX 1006 / INT 1073 / LUK 1003,'
        Write-Host '          HP 517 -> totals match. Any number off -> say which'
        Write-Host '        AND the top now draws, no AP spent -> select checks item'
        Write-Host '          requirements against the sheet; server needs a REQ gate'
        Write-Host '        sheet right, top still bare -> not requirements; say so'
        Write-Host ''
        Write-Host '  TL. BLANK CHARACTER-SELECT - CONFIRMED FIXED 09:23. STRUCK.' -ForegroundColor Green
        Write-Host '      The client builds the select UI once, from whatever list'
        Write-Host '      exists at that instant; a fast start builds it EMPTY 30 ms'
        Write-Host '      after login and mode 2 never refills. The hook now calls the'
        Write-Host '      client own refill (FUN_141177e40) after every 0x0010 where'
        Write-Host '      the UI already exists. Four for four rescued. A wiring test'
        Write-Host '      keeps it in hook.rs. Never watch 141177e40 in -Probe.'
        Write-Host '        blank ever again -> grep maplecw-hook.log SELECTFILL first:'
        Write-Host '          absent = step removed/off; "refusing" = paste it'
        Write-Host ''
        Write-Host '  TU. EVERY INBOUND OPCODE HAS A DISPOSITION - a LOG check.' -ForegroundColor Cyan
        Write-Host '      All remaining UNKNOWN/unanswered opcodes were one-way client'
        Write-Host '      reports (0x013D census - must NOT be answered - 0x01ED log,'
        Write-Host '      0x0420..0x0426 leaving burst, 3 undecoded). Named, routed'
        Write-Host '      through session/reports.rs, and the log now says "is a client'
        Write-Host '      report; nothing is expected back". 17 handled opcodes were'
        Write-Host '      unnamed too (0x00E5, 0x0199...) - fixed, test-pinned.'
        Write-Host '        grep -c UNKNOWN login.log world-ch0.log world-ch1.log -> 0'
        Write-Host '          any hit = a packet never seen before; paste the line' -ForegroundColor Yellow
        Write-Host '        "leaving the field: reason N" with N not 2 or 4 -> new; say N'
        Write-Host '        "is not answered yet" on a non-report -> a real gap; paste'
        Write-Host ''
        Write-Host '  TR. AP / SP RESET SCROLLS - CONFIRMED: "both now work." STRUCK.' -ForegroundColor Green
        Write-Host '      Your two presses were 0x0116, not the coupons 0x0114, and'
        Write-Host '      nothing answered them. Handled now: full refund, as !resetap.'
        Write-Host '        stats back to base, AP shown, scroll gone -> done'
        Write-Host '        stats reset, scroll stays -> consume failed; world-ch0.log'
        Write-Host '        nothing and inventory FROZEN -> latch not cleared; tell me' -ForegroundColor Yellow
        Write-Host '        a message, no change -> the reset refused; chat says why'
        Write-Host ''
        Write-Host '  TB. BEAUTY COUPON DIALOG NAME - CONFIRMED readable. STRUCK.' -ForegroundColor Green
        Write-Host '      Frieren was white too, so it is the client: its own string'
        Write-Host '      0x0464 colours the name 0xffffffff. The hook now patches six'
        Write-Host '      bytes of it to black at load. Off switch: -NoBeautyTextPatch.'
        Write-Host '      Double-click any hair or face coupon.'
        Write-Host '        name black -> done'
        Write-Host '        still white + hook.log "BEAUTYTEXT: patched" -> table was'
        Write-Host '                      decrypted before the hook; exe-file patch next'
        Write-Host '        still white + "refusing"/"not readable" -> paste the line'
        Write-Host ''
        Write-Host '  TO. THE UBEL OUTFIT - two fixed, one is data.' -ForegroundColor Magenta
        Write-Host '      a) Clothes from the DECO tab: the SERVER had refused them.'
        Write-Host '         Fixed. Drag each on again.'
        Write-Host '           sits in the Deco equip window -> the move works'
        Write-Host '           not drawn / gone after relog -> EXPECTED for now: the'
        Write-Host '                      record cannot carry worn slots over 31 yet'
        Write-Host '           refused again -> world-ch0.log names why; paste it'
        Write-Host '      b) Weapon: CLIENT data - FIXED AND INSTALLED. Covers list'
        Write-Host '         the weapon TYPES they dress as children (classic: 30 + links'
        Write-Host '         31/32/33); ours had 30 and 49 only; suitcase is 32. Each'
        Write-Host '         cover now links every classic type; on disk. Drag it on:' -ForegroundColor Yellow
        Write-Host '           goes on (Deco window) -> per-type rule confirmed'
        Write-Host '           does not DRAW -> expected until worn slots >31 carry'
        Write-Host '           still refused silently -> not the type child; say so'
        Write-Host '           refused with a message -> paste it'
        Write-Host '      d) NEW - THE RECEIPT: opening the Collection or a Set Coupon' -ForegroundColor Yellow
        Write-Host '         also opens an NPC box from "MapleStory Administrator":'
        Write-Host '         "You have received the following items:" then one line per'
        Write-Host '         item, icon + name. OK closes it. Open a set coupon:'
        Write-Host '           box, every item on its line with its icon -> done'
        Write-Host '           names missing/null -> #t unresolved; say which lines'
        Write-Host '           no icons -> #i unresolved; say so'
        Write-Host '           lines run together -> wrong line break; say so'
        Write-Host '           no box, items arrive -> unsolicited Say refused; paste'
        Write-Host '             the world-ch0.log "ScriptMessage ... receipt" line'
        Write-Host '           name reads "Maple Administrator" -> rename not landed'
        Write-Host '      f) THE BOX: double-click sent NO packet - the client opens a' -ForegroundColor Yellow
        Write-Host '         Cash item by id FAMILY, and 522 is not one it opens; the'
        Write-Host '         568 coupons are. The box now wears 5681599 (node, string,'
        Write-Host '         shop row, server); an old one in a bag is renamed on start.'
        Write-Host '           eight coupons + receipt, box gone -> done'
        Write-Host '           still nothing, no 0x0114 in world-ch0.log -> family reading' -ForegroundColor Yellow
        Write-Host '             wrong; say so' -ForegroundColor Yellow
        Write-Host '           0x0114 arrives, server refuses -> paste world-ch0.log line'
        Write-Host '           box in bag has no name/icon -> rename did not run; say so'
        Write-Host '      g) CASH EQUIPS + THE WEAPON COVER SURVIVE A RELOG - CONFIRMED on select' -ForegroundColor DarkGray
        Write-Host '         and in the field (2026-09-12). Nothing to test here.' -ForegroundColor DarkGray
        Write-Host '      h) HIMMEL''S CAPE - DONE (behind the body at z -2). Nothing to test.' -ForegroundColor DarkGray
        Write-Host '      i) THE HAIR-HATS - RENUMBERED (2026-09-18). The client reads an equip' -ForegroundColor Yellow
        Write-Host '         id''s 4th digit as gender and 6 = FEMALE, so 1006910..12 refused every'
        Write-Host '         male character before any packet. They are 1007910..12 now (digit 7,'
        Write-Host '         unisex); bags are renumbered on start. Needs the rebuilt client.'
        Write-Host '           male char double-clicks a hair-hat: it goes on -> DONE' -ForegroundColor Green
        Write-Host '           on, but own hair shows through -> vslot tokens ignored; say so' -ForegroundColor Yellow
        Write-Host '           still nothing -> paste the hat''s bag line; 1006xxx = old package' -ForegroundColor Yellow
        Write-Host '      j) NEW - FACE COUPONS: no packet ever; the Beauty dialog opens only for' -ForegroundColor Yellow
        Write-Host '         2540xxxx (hair) and 2890xxx (face) + skins/androids, never 2897xxx.'
        Write-Host '         They wear 2890907..914 now (node, string, server, bag rename).'
        Write-Host '         Double-click one in the Use tab:'
        Write-Host '           dialog opens, face previewed, CONFIRM changes it -> done'
        Write-Host '           dialog opens, wrong/no preview -> say what it shows'
        Write-Host '           still nothing -> range reading wrong; say so' -ForegroundColor Yellow
        Write-Host '           coupon has no name/icon -> rename on start did not run'
        Write-Host '      k) NEW - NINA NO LONGER REPEATS HERSELF ON ACCEPT: 1003 has no Say.0.yes,' -ForegroundColor Yellow
        Write-Host '         so the server replayed Say.0 after the button. An accept with no yes'
        Write-Host '         branch now sends the record and NO box. Take 1003 from Nina again:'
        Write-Host '           two lines, Accept, window closes, quest in journal -> done'
        Write-Host '           the same two lines again -> fix not in the running server; say so'
        Write-Host '           their greeting after Accept -> d0 fallback still reachable; say so'
        Write-Host '           Heena no longer says "hill to the east" -> yes branch broke' -ForegroundColor Yellow
        Write-Host '      l) NEW - THREE SNAILS THROWS A SHELL: level 3 takes a Red Snail Shell' -ForegroundColor Yellow
        Write-Host '         (4000004) per cast from the Etc tab (0x0070 count change); with'
        Write-Host '         none, the swing gets ONE system chat line and nothing else. Cast it'
        Write-Host '         with a few shells, then with none:'
        Write-Host '           count drops per cast; no shell -> red line, mob unhurt -> done'
        Write-Host '           server log says threw, bag window unchanged -> 0x0070 shape; say so'
        Write-Host '           line in an odd colour -> say which; category 11 is the guess'
        Write-Host '           mob still hurt with no shell -> client-local damage; say so'
        Write-Host '           nothing at all -> paste world-ch0.log "itemCon" line' -ForegroundColor Yellow
        Write-Host '      m) RAIN''S QUIZ - menu works; the "repeat" after the right answer was the' -ForegroundColor Yellow
        Write-Host '         client offering the NEXT quiz: our completion went out before the'
        Write-Host '         question. Now the turn-in waits for the right choice (record, exp,'
        Write-Host '         fanfare ride with "That''s right!"). Take Quiz 3:'
        Write-Host '           question; right -> clear sound, line, next offer opens ONCE -> done'
        Write-Host '           offer opens BEFORE the question -> paste 0x0151..menu lines' -ForegroundColor Yellow
        Write-Host '           wrong + close -> quest still started, click asks again -> done'
        Write-Host '           right, no clear sound/exp -> paste the "quiz:" line'
        Write-Host '      o) NEW - ONE REWARD FROM THE POOL: Lucas''s Reply gave all seven' -ForegroundColor Yellow
        Write-Host '         headbands; the WZ marks them prop 1 = draw one. prop 0 rows are'
        Write-Host '         unconditional, prop > 0 rows are a weighted pool of one, gender'
        Write-Host '         rows go to that gender (39 quests). Do Lucas''s Reply again:'
        Write-Host '           ONE headband, letter gone -> done'
        Write-Host '           still seven -> old server; say so'
        Write-Host '           none -> paste world-ch0.log "reward rows" line' -ForegroundColor Yellow
        Write-Host '      p) NEW - ELEVEN PETS, PERMANENT, IN THE SHOP: the "3 days" was each' -ForegroundColor Yellow
        Write-Host '         pet''s own info/life; all eleven now permanent 1, the eight'
        Write-Host '         missing ones have Pets-tab rows, and a pet is BOUGHT as a type-3'
        Write-Host '         body instead of refused. (life 0 was wrong - see step v.)'
        Write-Host '         Cash Shop, Pets tab:'
        Write-Host '           eleven pets, no duration / permanent -> data half done'
        Write-Host '           still "3 days" -> say the tooltip text exactly'
        Write-Host '           buy one: locker icon, moves to Cash tab, no death -> body right'
        Write-Host '           client dies at buy/move -> paste client-exit.log + last 0x03E1' -ForegroundColor Yellow
        Write-Host '           only three listed -> tab does not list by SN prefix; say so'
        Write-Host '      q) THE REPEAT-DIALOGUE AUDIT: a test walks all 316 quests with dialogue' -ForegroundColor DarkGray
        Write-Host '         through Accept and turn-in - the server never answers with the quest''s' -ForegroundColor DarkGray
        Write-Host '         own opening and never opens two boxes. One more found and fixed (1002''s' -ForegroundColor DarkGray
        Write-Host '         turn-in said the NPC''s greeting). If a line still repeats, name the' -ForegroundColor DarkGray
        Write-Host '         quest and WHICH line.' -ForegroundColor DarkGray
        Write-Host '      r) NEW - THE CEILING COUNTS WORN HP: 194/199 was the Red Headband''s' -ForegroundColor Yellow
        Write-Host '         incMHP 5, which the client adds and the server''s ceiling did not.'
        Write-Host '         Stand still on the owner below 199:'
        Write-Host '           ticks to 199 and stops -> fixed'
        Write-Host '           stops at 194 -> something else adds too; say the tooltip'
        Write-Host '           bar jumps to a new max -> folded into the record; paste 0x007C' -ForegroundColor Yellow
        Write-Host '      s) NEW - A KILL REFILLS THE MAP, NOT THE POINT: the solo cap was' -ForegroundColor Yellow
        Write-Host '         already 75% (49 of 66 on Split Road, measured); what felt pinned'
        Write-Host '         was the dead mob returning on its own point. A refill now draws a'
        Write-Host '         random free point across the whole map. Kill a few on Split Road:'
        Write-Host '           stands up elsewhere, maybe another type, count stays 49 -> designed'
        Write-Host '           always back on the same spot -> paste SPAWN lines around a kill' -ForegroundColor Yellow
        Write-Host '           map thins out, or more than 49 -> paste world-ch0.log' -ForegroundColor Yellow
        Write-Host '      t) RAIN''S QUIZ: THE CLIENT CONDUCTS IT. The screenshots +' -ForegroundColor Yellow
        Write-Host '         world-ch0.log showed the client draws the whole quiz with no packets,' -ForegroundColor DarkGray
        Write-Host '         then sends the turn-in - which the server used to re-ask. Fixed: a' -ForegroundColor DarkGray
        Write-Host '         quiz turn-in records the completion and says nothing - NOT YET' -ForegroundColor Yellow
        Write-Host '         seen on screen. Rain''s next question, answer right, press OK:'
        Write-Host '           nothing else appears, exp lands at once -> fixed'
        Write-Host '           same question again -> paste the turn-in lines' -ForegroundColor Yellow
        Write-Host '           UI freezes after OK -> record alone is not an answer; say so' -ForegroundColor Yellow
        Write-Host '      u) NEW - SUMMONING A PET: the double-click (0x0147) is answered with' -ForegroundColor Yellow
        Write-Host '         0x0277, read off the client''s own pet decoder. Double-click the Husky:'
        Write-Host '           appears beside you -> right; say if the item draws as summoned,'
        Write-Host '                         and paste any new UNKNOWN inbound opcodes (its moves)'
        Write-Host '           appears, item not summoned -> the pairing serial; say so'
        Write-Host '           client dies at the click -> paste client-exit.log + the 0x0277 line' -ForegroundColor Yellow
        Write-Host '           nothing -> paste the lines after 0x0147;  click again -> it goes away'
        Write-Host '         VACUUM: see step 8 LOOT (0x0205 settled; bought Expanded Auto Move = wonderGrade 6)' -ForegroundColor DarkGray
        Write-Host '      v) THE HUSKY RENDERS. CLOSED. THREE TWO-CLIENT FIXES, one launch,' -ForegroundColor Green
        Write-Host '         Tester2 as the second client:'
        Write-Host '         (a) THE SECOND CLIENT SEES THE PET: a Presence now carries the' -ForegroundColor Yellow
        Write-Host '         pet''s 0x0277 as a companion, posted right after the owner''s'
        Write-Host '         0x0224 to whoever arrives. Husky out, Tester2 walks into your map:'
        Write-Host '           Tester2 sees the Husky at your feet -> done'
        Write-Host '           sees you, no pet -> paste Tester2''s log after their 0x00DC' -ForegroundColor Yellow
        Write-Host '           pet at (0,0)/wrong place -> say where it stands'
        Write-Host '         (b) MOBS NO LONGER SNAP ON JOIN: the floor under the end of the' -ForegroundColor Yellow
        Write-Host '         controller''s last path now travels with the position. Join a map'
        Write-Host '         the other client has been fighting on:'
        Write-Host '           mobs stand where they are -> done'
        Write-Host '           still snap -> sideways (position) or up/down (floor)?' -ForegroundColor Yellow
        Write-Host '         (c) THE JOINER APPEARS AT THE PORTAL, LANDING POSE, not at the' -ForegroundColor Yellow
        Write-Host '         origin: portals.txt has x,y now (regenerated), the 0x0224 stands'
        Write-Host '         there in action 4 (jump) on the foothold under it. Watch them in:'
        Write-Host '           doorway, landing pose, then walks -> done'
        Write-Host '           doorway but STANDING -> the pose byte (8) is wrong; say so'
        Write-Host '           still the origin -> paste the banner portals line + 0x0224 body' -ForegroundColor Yellow
        Write-Host '         STILL OPEN: the portal crash (repeat: pet out, Tester2 in 10001010,'
        Write-Host '         walk west00; no crash closes it) and PICK-UP (kill a mob near the'
        Write-Host '         Husky; no-click pickup -> works; walks to it, nothing -> paste log).'
        Write-Host '      e) NEW - FRIEREN ASKS WHICH VERSION: opening the Frieren' -ForegroundColor Yellow
        Write-Host '         coupon opens a 3-row menu (normal / Ringlets / Sleep, hair'
        Write-Host '         icons). The CHOICE spends the coupon; End Chat keeps it.'
        Write-Host '         "your choice of" Clothes/Winter Clothes -> you get BOTH.'
        Write-Host '           normal 7 items, Ringlets 7, Sleep 4, then the receipt'
        Write-Host '           menu, pick, those items + receipt -> done'
        Write-Host '           End Chat spends the coupon -> tell me; it must not' -ForegroundColor Yellow
        Write-Host '           no menu, items arrive as before -> paste the 0x0114 line'
        Write-Host '      c) HAIR/FACE COUPON REDRAWN IN PLACE (2026-09-18): "only after a map' -ForegroundColor Yellow
        Write-Host '         change" = the dialog only previews. Confirm now gets ONE 0x007C with' -ForegroundColor Yellow
        Write-Host '         the HAIR/FACE bit + id; the client runs the equip redraw pair on it.' -ForegroundColor Yellow
        Write-Host '         !hair 42540 is the cheap form (no reload now either).'
        Write-Host '           own screen: changes the MOMENT you Confirm -> CONFIRMED 14:51' -ForegroundColor Green
        Write-Host '           flicker / CLIENT FAULT -> paste client-exit.log' -ForegroundColor Yellow
        Write-Host '         ANOTHER PLAYER''S CHARACTER INFO (2026-09-18): double-click Tester2 as the owner.' -ForegroundColor Magenta
        Write-Host '         0x01FC was unanswered (and it LATCHES); now one 0x00A2 with name, level,' -ForegroundColor Magenta
        Write-Host '         job, fame 0, guild -, and the pet out with its numbers.' -ForegroundColor Magenta
        Write-Host '           window opens with Tester2''s numbers (+ pet panel) -> DONE; avatar drawn?' -ForegroundColor Green
        Write-Host '           nothing opens, log HAS 0x00A2 -> a pre-open gate; paste both lines' -ForegroundColor Yellow
        Write-Host '           nothing opens, NO 0x00A2 -> arm not reached; paste the 0x01FC line' -ForegroundColor Yellow
        Write-Host '         FAME (evening): the up/down arrows answer now (0x0144 -> 0x0087). Once a day' -ForegroundColor Magenta
        Write-Host '         per giver (00:00 UTC), same target once a week (Mon 00:00 UTC); the'
        Write-Host '         client says "month" for the week rule. ITEM LIST: hair, face, then every'
        Write-Host '         worn equip and cash cover. Hair/face icons are RENDERED into the hybrid'
        Write-Host '         WZ by backport_install.py --install (client closed) - needed first, or'
        Write-Host '         launch with -NoLookItems for equips only.'
        Write-Host '         APOSTROPHES: CONFIRMED (fametext in the hook; -NoFameTextPatch off).' -ForegroundColor DarkGray
        Write-Host '           up on Tester2: "raised", FAME 1 in the window, Tester2 sees it; again ->' -ForegroundColor Green
        Write-Host '           "not anymore for today" -> DONE' -ForegroundColor Green
        Write-Host '           Item List: hair, face, hat/coat/weapon/cover, all with icons -> DONE' -ForegroundColor Green
        Write-Host '           3rd look: Fern Hair blank everywhere = the client''s id gate (<= 3xxxx);' -ForegroundColor Yellow
        Write-Host '           the hook patches it now (LOOKGATE in maplecw-hook.log). Tooltip images' -ForegroundColor Yellow
        Write-Host '           sat too low = origin; every icon is a 32x32 cap-shaped canvas now.' -ForegroundColor Yellow
        Write-Host '           4th look: cell draws; tooltip still low-left -> ours were INLINE canvases,' -ForegroundColor Yellow
        Write-Host '           real icons are stub+_outlink with originless pixels: now the same shape.' -ForegroundColor Yellow
        Write-Host '           tooltip image inside the frame -> DONE; still low-left -> say so' -ForegroundColor Green
        Write-Host '           (next variant: origin 0,0 on the stub); cell blank -> LOOKGATE lines' -ForegroundColor Yellow
        Write-Host '           the cell under the PET: its Top Hat -> CONFIRMED' -ForegroundColor DarkGray
        Write-Host '           hair/face blank, equips fine -> icons not installed; a DEATH on open ->' -ForegroundColor Yellow
        Write-Host '           relaunch -NoLookItems, paste client-exit.log; blank everywhere -> 0x00A2 line' -ForegroundColor Yellow
        Write-Host '         OTHER CLIENTS IN PLACE BY 0x02AE: CONFIRMED - no blink, pet stays.' -ForegroundColor DarkGray
        Write-Host '  TH. FACE COUPON: no dialog, no preview. Images are fine; the'
        Write-Host '      ID is the one difference (22039; classic faces end at 21825).'
        Write-Host '      One chat line settles it: !face 22039' -ForegroundColor Yellow
        Write-Host '        face draws in the field -> only the coupon UI refuses the id'
        Write-Host '        does not draw -> the backport must renumber the faces'
        Write-Host ''
        Write-Host '  TF. THE FREE MARKET DOOR - can strand you if it is wrong.' -ForegroundColor Magenta
        Write-Host '      From HENESYS MARKET (10001040) walk into market00, then'
        Write-Host '      walk back out of the Free Market via out00.'
        Write-Host '        back in Henesys Market -> the memory works'
        Write-Host '        somewhere else -> say WHERE'
        Write-Host '        nothing at all -> grep world-ch0.log for "free market"'
        Write-Host '      THEN from EL NATH MARKET (!map 20001010) - El Nath is NOT' -ForegroundColor Yellow
        Write-Host '      the fallback, so landing there proves the town is'
        Write-Host '      REMEMBERED and not hard-coded. Henesys alone cannot.'
        Write-Host '        back to El Nath -> done'
        Write-Host '        back to HENESYS -> it is falling through to the fallback'
        Write-Host '      AND: enter, LOG OUT and back in, then leave. The memory is'
        Write-Host '      in the database and should survive.'
        Write-Host ''
        Write-Host '  TC. THE COUPON PURCHASE. (The old capture is DONE - struck.)' -ForegroundColor Magenta
        Write-Host '      "Not enough leaf points" with 105,500 LP: the log shows the'
        Write-Host '      real cause - the locker table on your file lacked the new'
        Write-Host '      failed_slots column, every locker read failed, and ANY store'
        Write-Host '      error was reported as "not enough cash". Both fixed; storage'
        Write-Host '      had the same missing column and is fixed too.'
        Write-Host '      The coupon goes to the CASH INVENTORY panel, never storage.'
        Write-Host '      a) Buy ONE Etc Tab 5-Slot Coupon (100 LP).' -ForegroundColor Yellow
        Write-Host '          in Cash Inventory, LP down 100 -> fixed'
        Write-Host '          "unknown error" -> other server error; world-ch0.log names it'
        Write-Host '          "not enough leaf points" again -> wallet read wrong'
        Write-Host '      b) MOVE it from Cash Inventory to your Cash tab. NEW: the' -ForegroundColor Yellow
        Write-Host '         server refused this until today; never seen on a screen.' -ForegroundColor Yellow
        Write-Host '          appears in the Cash tab at your slot -> 0x0A/0x19 work'
        Write-Host '          "unknown error", stays put -> world-ch0.log names the check'
        Write-Host '          gone from panel, NOT in Cash tab -> 0x19 body wrong; say tab'
        Write-Host '      c) LEAVE the shop and return with an item still in the panel.'
        Write-Host '          still listed -> entry listing works (new, unmeasured)'
        Write-Host '          panel empty -> client ignores 0x0C at entry; row is in DB'
        Write-Host '      d) Double-click the coupon in the Cash tab -> Etc tab +5.'
        Write-Host '      e) Separately: open STORAGE; opens with slots -> 2nd table OK.'
        Write-Host ''
        Write-Host '  TD. THE FOUR THINGS FIXED AFTER THE LAST RUN.' -ForegroundColor Magenta
        Write-Host '      All four came out of that run. None has been seen working.'
        Write-Host '      a) REAL SCROLLING - drag a scroll onto an equip in the'
        Write-Host '         inventory. WORKS (2026-09-18). A WORN item''s scroll was lost on a'
        Write-Host '         map change (the record dressed from the template); it reads the' -ForegroundColor Yellow
        Write-Host '         worn row now. Scroll a worn shirt, change maps: +HP stays -> DONE' -ForegroundColor Green
        Write-Host '         !item 2040400 3 - topwear DEF, 100% and cursed 0,'
        Write-Host '         so it CANNOT destroy anything. That one first.' -ForegroundColor Yellow
        Write-Host '           stats change + a sound -> the whole path works'
        Write-Host '           "cannot be used here" -> a refusal; world-ch0.log names it'
        Write-Host '           nothing -> grep world-ch0.log for 0x0236. Went out = the'
        Write-Host '                      client did not draw it. Did not = refused'
        Write-Host '         THEN on purpose: !item 2040403 3 is 10% with CURSED 50.'
        Write-Host '         It DESTROYS the shirt on half its failures - that arm'
        Write-Host '         has never run. Wear something you do not want.' -ForegroundColor Yellow
        Write-Host '      b) PARTIAL DROP - drag 2 of a stack of 5 to the ground.'
        Write-Host '           2 on the floor, 3 still in the slot -> mode 1 works'
        Write-Host '           the slot EMPTIES -> wrong mode, and the store now'
        Write-Host '                      disagrees with the screen. Say so'
        Write-Host '      c) THE MESO CAP. Server now refuses over 10000 too.'
        Write-Host '           exactly 10000 works -> the boundary is right'
        Write-Host '           10000 refused -> off by one'
        Write-Host '      d) THE LADDER. Climb one, drop mesos AND an item.'
        Write-Host '           they FALL from you to the floor -> fixed'
        Write-Host '           they appear on the floor, no fall -> arc still zero'
        Write-Host '           they HANG at your feet -> the snap was lost and they'
        Write-Host '                      cannot be picked up. The worse failure' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  TS. !scroll - REBUILT TODAY, NOT SEEN ON A SCREEN SINCE.' -ForegroundColor Magenta
        Write-Host '      Run as maplecw. Five questions:'
        Write-Host '      a) EMPTY-HANDED. DO THIS BEFORE YOU !item ANYTHING -' -ForegroundColor Yellow
        Write-Host '         granting a scroll destroys the state it tests.'
        Write-Host '         Just !scroll, carrying neither.'
        Write-Host '           two rows, icon + NAME + what it does + the rate'
        Write-Host '                       -> #i and #t both resolve in a say box.'
        Write-Host '                          #t is the client OWN String.wz name,'
        Write-Host '                          so right names prove it resolved'
        Write-Host '           raw #t4031065# on screen -> #t not honoured here.'
        Write-Host '                          Fall back to our own names'
        Write-Host '           no icons, names fine -> #i needs a menu context'
        Write-Host '      b) THE STACK. !item 4031065 5, then !item 4031066 5.'
        Write-Host '         THE SERVER HALF IS PROVEN - a test gets five in one' -ForegroundColor Yellow
        Write-Host '         slot, a merge, and 105 spilling to 100+5. This tests'
        Write-Host '         only what the CLIENT does above its own slotMax (=1).'
        Write-Host '           one slot   -> the client honours it. Question closed.'
        Write-Host '                         Then try to SPLIT the stack by dragging'
        Write-Host '           five slots -> the CLIENT clamps. Definitive, since the'
        Write-Host '                         server is proven to send one stack. Swap'
        Write-Host '                         to ids with slotMax 200 (161 of them)'
        Write-Host '           client DIES -> above slotMax faults rather than being'
        Write-Host '                         ignored. Grab client-exit.log'
        Write-Host '      c) THE MENU ROW. Each row is now icon + name on ONE line.'
        Write-Host '           draws clean -> the clip is fixed AND an icon works'
        Write-Host '                          inside a #L. ZERO of this client own 33'
        Write-Host '                          menus do that, so it is unattested'
        Write-Host '           overlaps    -> the ICON breaks the row height. Drop it'
        Write-Host '           gap, no art -> #i is not honoured in a #L. Same fix'
        Write-Host '      d) THE SOUND. 0x0236 goes to the whole map now.'
        Write-Host '           sound + flash -> the effect packet is real'
        Write-Host '           nothing       -> world-ch0.log shows 0x0236 leaving either'
        Write-Host '                            way, so silence is a CLIENT result'
        Write-Host '      e) TREASURE SCROLL - NEVER RUN, NOT ONCE.' -ForegroundColor Yellow
        Write-Host '           !item 2043200 2 while wearing a One-Handed Blunt'
        Write-Host '           Weapon, then !scroll -> Treasure -> the weapon.'
        Write-Host '           list shows ONLY fitting scrolls -> category rule holds'
        Write-Host '           list empty -> wrong tab or inverted filter. Control:'
        Write-Host '                         !item 2040000 1 is a HAT scroll and must'
        Write-Host '                         NOT appear. If it does, the filter is'
        Write-Host '                         backwards'
        Write-Host '      A ! command answered by a CHAT BALLOON is the GM GATE.' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  AFTERWARDS: world-ch0.log is the evidence. Say what you saw per step.' -ForegroundColor Green
        Write-Host ''
        Write-Host '  ---- everything below is reference, not this run ----' -ForegroundColor DarkGray
        Write-Host '  WHAT IS WORTH A RUN NOW, in order:' -ForegroundColor Yellow
        Write-Host '    0A. THE SESSION BUG AND ONE LOGIN PER ACCOUNT (new 2026-09-08).' -ForegroundColor Green
        Write-Host '        Two changes, and they only make sense together. Do this FIRST -'
        Write-Host '        it is two minutes and it gates the launcher for everything below.'
        Write-Host '        (i) START GAME TWICE from ONE sign-in. Sign in, Start Game, let'
        Write-Host '            the client reach CHARACTER SELECT. Leave it there. Press'
        Write-Host '            Start Game again in the launcher.'
        Write-Host '              "That ID is already logged in. Please try again later"'
        Write-Host '                 -> BOTH changes work. The token was honoured (it used'
        Write-Host '                 to say the session was invalid) and the account was'
        Write-Host '                 then held by the first client. The whole feature.'
        Write-Host '              session/ID invalid, or NOT REGISTERED -> the token was'
        Write-Host '                 refused, so change 1 did not take. Read login.log for'
        Write-Host '                 the 0x0073 IDENTITY line.'
        Write-Host '              second client reaches character select -> the lease was'
        Write-Host '                 not held. login.log says why on a PRESENCE line.'
        Write-Host '              FIRST client FREEZES -> STOP. The refusal packet was not'
        Write-Host '                 accepted. Keep both logs; that is the finding.'
        Write-Host '            NOBODY HAS SENT THIS CLIENT LOGIN RESULT 7 BEFORE. The'
        Write-Host '            wording comes from its own WZ and the code from'
        Write-Host '            FUN_141b267c0 - both static. This step is what makes it'
        Write-Host '            measured, so SAY WHAT THE DIALOG ACTUALLY SAID.'
        Write-Host '        (ii) CLOSE THE SECOND CLIENT and press Start Game again.'
        Write-Host '              still refused -> the lease did not release. It should'
        Write-Host '                 expire on its own within 60 s; if not, that is the'
        Write-Host '                 finding.'
        Write-Host '        (iii) THE LOCKOUT, the one a player would hit. Take the FIRST' -ForegroundColor Yellow
        Write-Host '             client INTO THE WORLD, then KILL IT from Task Manager -' -ForegroundColor Yellow
        Write-Host '             not a clean quit. Press Start Game.' -ForegroundColor Yellow
        Write-Host '              it starts and plays -> correct. A crash frees the account'
        Write-Host '                 at once: the OS closes the socket and world-ch0.log logs'
        Write-Host '                 "ended: ... forcibly closed ... (os error 10054)".'
        Write-Host '              "already logged in" -> A LOCKOUT. Say how long it lasts.'
        Write-Host '                 More than ~60 s means the release AND the expiry both'
        Write-Host '                 missed, and that is WORSE than the bug this fixed.'
        Write-Host '        (iv) LOG OUT / CHOOSE ANOTHER WORLD from one client, twice.'
        Write-Host '             Neither must ever say "already logged in" - the lease is'
        Write-Host '             held per client PROCESS, so your own reconnect re-takes it.'
        Write-Host '        Not worth a step: two DIFFERENT accounts, two clients. Unchanged,'
        Write-Host '        and claims_smoke.py measures it over real sockets every run.'
        Write-Host ''
        Write-Host '    0. LOGIN IS ENFORCED (new 2026-09-05). The launcher path is the run'
        Write-Host '       now: sign in there, Start Game, the world as before. To SEE the'
        Write-Host '       refusal, with NOBODY signed in yet, start the PATCHED client with'
        Write-Host '       its OWN folder as the working directory (the hook logs and reads'
        Write-Host '       its markers relative to it, and the client checks that folder):'
        Write-Host ("         Start-Process -FilePath `"{0}\client-patched\MapleStory.exe`" -WorkingDirectory `"{0}\client-patched`" -ArgumentList '-NXLDEBUG','127.0.0.1','8484'" -f $root)
        Write-Host '       NOT the copy under C:\Nexon - that one has real GameGuard. It must'
        Write-Host '       show "not a registered ID" and stay USABLE (a frozen client would'
        Write-Host '       mean the refusal packet was not accepted - report that). login.log'
        Write-Host '       says "served as NOBODY - REFUSED" for that connection.'
        Write-Host '       (Every Start Game ALSO draws one REFUSED line half a second before'
        Write-Host '       the served connection - that is the launcher probing the port, not'
        Write-Host '       a client. The T13 line is the one with login traffic after it.)'
        Write-Host '       AND THE CHANNEL NOW HOLDS THE CLAIM TO THE SIGN-IN: on this box the'
        Write-Host '       migration is bound to your launcher sign-in and the channel checks'
        Write-Host '       it through the OS. If a character does NOT enter the world, read'
        Write-Host '       world-ch0.log for "REFUSED the migration" - that is the on-box'
        Write-Host '       attestation failing, it is a finding, and -BindMigrations never'
        Write-Host '       gets you playing while it is looked at.'
        Write-Host '    0b. CHAT AND PARTY INVITE. The 2026-09-05 evening run: the invite went'
        Write-Host '       out and BOTH CLIENTS DIED - the "joined" packet was sent without its'
        Write-Host '       party block, each client threw, reported it back (0x009E) and hung'
        Write-Host '       up. Fixed: 0x13 now carries the six-seat block. And the first 0x0183'
        Write-Host '       ever decoded was the client saying "dialog opening", not a click -'
        Write-Host '       the server took it for an accept. The button values are now read'
        Write-Host '       off the client: 5 accept, 4 decline. So, this run:'
        Write-Host '       CHAT: type on one client, the OTHER shows the balloon and the log line.'
        Write-Host '       PARTY: leader Create, Invite by name. Leader: "You have invited".'
        Write-Host '       TARGET: an invite dialog that STAYS until you click. Say what it shows'
        Write-Host '       (name? level? job?). Nobody joins before a click - if the party'
        Write-Host '       window already lists the target, answer 0 is still being acted on.'
        Write-Host '       ACCEPT: both read "has joined the party" and BOTH party windows list'
        Write-Host '       both members - the block is what draws them. A client dying here'
        Write-Host '       means the block is wrong: STOP and keep the logs.'
        Write-Host '       DECLINE (fresh invite): leader reads "has denied the party request",'
        Write-Host '       nobody joins.'
        Write-Host '       TIMEOUT: invite, leave the dialog ALONE about a minute, then invite'
        Write-Host '       the SAME character again. It must go through. world-ch0.log: "LAPSED".'
        Write-Host '    0c. PARTY MECHANICS (new 2026-09-05, from the owner''s screenshot run).'
        Write-Host '       LEAVE: the member clicks Leave - they must actually leave, and the'
        Write-Host '       leader''s window drops them. Before: "unknown error", stuck in party.'
        Write-Host '       PICK-UP RIGHTS: the leader clicks it - no more "unknown error".'
        Write-Host '       PARTY EXP: both stand on the same map, ONE kills a mob. The other'
        Write-Host '       should see a YELLOW EXP gain (a 30% COPY each - the 5th !setrates'
        Write-Host '       field, not a split); the killer white with 70%.'
        Write-Host '       QUEST: both have a kill quest (e.g. Sam''s snails). One kills - the'
        Write-Host '       OTHER''s counter should tick up too.'
        Write-Host '       PARTY DROPS: a mob killed by one shows its drops to BOTH, and either'
        Write-Host '       may pick them up. Then one LEAVES and the other kills: the leaver'
        Write-Host '       must NOT be able to take those drops (the killer still can).'
        Write-Host '       GROUND DROP: drop an item from your bag - anyone on the map should'
        Write-Host '       see it and be able to pick it up.'
        Write-Host '       EXPIRY (2026-09-18): a party drop and a bag drop left alone for two' -ForegroundColor Magenta
        Write-Host '       minutes vanish from BOTH screens at the same moment -> fixed. Still' -ForegroundColor Magenta
        Write-Host '       drawn on the other screen -> paste its DropLeaveField log lines.' -ForegroundColor Yellow
        Write-Host '       PARTY HP (built 2026-09-06, walked back from the gauge): the other'
        Write-Host '       member''s bar in the top-right HUD should FILL within a second of the'
        Write-Host '       party forming, and follow their HP when a mob hits them or they'
        Write-Host '       drink a potion. Also say whether the small bar OVER THEIR HEAD moves.'
        Write-Host '       Blank bar = the packet did not draw; that is the finding.'
        Write-Host '    0e. ARCHER AUDIT (2026-09-06). Wear a bow, arrows in the Use tab.'
        Write-Host '       Normal shot: the stack drops by 1. DOUBLE SHOT: by 2. ARROW BLOW: 1.'
        Write-Host '       POWER KNOCKBACK: by 0 (the bow is swung, not fired). Read the count'
        Write-Host '       off the Use tab, not the shot animation. A crossbow uses 2061xxx.'
        Write-Host '       POWER KNOCKBACK''s push is the CLIENT''s (range 130 px at level 1) and'
        Write-Host '       only plays when YOU control the mob: on a mob the other client was'
        Write-Host '       driving, the FIRST hit hands it over and the SECOND pushes. Say how'
        Write-Host '       far it flies next to a normal hit, and whether the other screen agrees.'
        Write-Host '    0f. THIEF AUDIT (2026-09-06). Wear a claw, stars in the Use tab.'
        Write-Host '       Normal throw: the stack drops by 1. LUCKY SEVEN: by 2 (one per'
        Write-Host '       star thrown - the data has no bulletConsume, so [I]). Double Stab'
        Write-Host '       with a dagger: 0. If NOTHING drops and world-ch0.log says "SHOOT body'
        Write-Host '       did not parse", that is the finding: no 0x00E0 was ever captured.'
        Write-Host '       RECHARGE: see step 39f. Buy tab: Wolbi/Ilbi/... at 0 mesos there'
        Write-Host '       means price 0 does NOT hide a row (the server refuses those buys).'
        Write-Host '    0g. WARRIOR AUDIT. SLASH BLAST costs HP (3 at level 1) AND MP now;'
        Write-Host '       Power Strike only MP. Watch the HP bar tick down per swing.'
        Write-Host '       MAX HP INCREASE (found from your 358/447 screenshot: the CLIENT adds'
        Write-Host '       the 25%, the server called 358 full). Now: idle regen must climb'
        Write-Host '       PAST 358 to 447; a potion drunk above 358 must NOT drop you to 358;'
        Write-Host '       a level-up refills to 447. Same for Max MP Increase on a Magician,'
        Write-Host '       which is inferred, not seen - say what the MP number does.'
        Write-Host '       NO "A skill has been activated." line on a skill-up any more.'
        Write-Host '    0h. PARTY BUFFS. Haste (4101001/4201001) and Rage (1101004) reach'
        Write-Host '       every party member ON THE SAME MAP: icon on BOTH screens. A member'
        Write-Host '       on another map, or a stranger beside you, gets nothing. Haste: the'
        Write-Host '       recipient walks AND jumps higher (jump = bit 93, [D]; faster walk'
        Write-Host '       with the same jump = pair off by one). Rage: the number on the'
        Write-Host '       stat window''s attack line (bit 84, [D]; icon and no number = try'
        Write-Host '       83). The caster must HAVE the skill: !job 410 then !learn (Haste).'
        Write-Host '       IRON WILL is SELF-ONLY in this client''s data (no rectangle):'
        Write-Host '       expected, not a bug.'
        Write-Host '    0i. THE REPAIR HOLDS. 26 min, 7 catches, 7 repairs, 0 refusals, NO death'
        Write-Host '       - the night before, the same idle session died at 23 min. It does NOT'
        Write-Host '       stop the writer; it turns the fatal free into a correct one.'
        Write-Host '       TO PLAY, add -SentryQuiet. The freezes were measured, not guessed:'
        Write-Host '         ordinary walk        0.63-0.71 ms   negligible'
        Write-Host '         finding, no dump     49 ms          the 68-thread stack scan'
        Write-Host '         finding WITH a dump  703-895 ms     client frozen throughout'
        Write-Host '       -SentryQuiet drops dumps and the thread scan, and walks every 2s'
        Write-Host '       except within 5s of a predicted firing. The 180s period is LEARNED'
        Write-Host '       from the first two catches, so nothing is assumed, and a catch'
        Write-Host '       outside the window resets it. Keep the repair either way.'
        Write-Host '    0h. FIVE LIVE FIXES - none has ever been on a screen. ONE at a time.' -ForegroundColor Green
        Write-Host '       (a) MESOS, THE IMPORTANT ONE. Drop 10 mesos, then immediately try'
        Write-Host '           to move an item in the bag.'
        Write-Host '             refuses in words AND the bag works -> fixed'
        Write-Host '             nothing happens and the bag is DEAD -> latch still set'
        Write-Host '           The client latches when it SENDS, so an unanswered drop killed'
        Write-Host '           the bag, the AP buttons and the cash shop for the whole'
        Write-Host '           session. Try all three if the bag is dead.'
        Write-Host '       (b) MOB DROPS. WALKING IS FIXED. JUMPING IS NOT.' -ForegroundColor Yellow
        Write-Host '           the owner on a live client: "the drops so far are good ... when a'
        Write-Host '           mob is JUMPING, the loot drops BELOW the current platform."'
        Write-Host '           Do NOT re-test a walking snail. Kill a mob MID-JUMP.'
        Write-Host '             lands on the corpse, on the platform -> jump half fixed'
        Write-Host '             lands BELOW the platform             -> still wrong'
        Write-Host '           Say WHICH platform it landed on: one below, or the ground.'
        Write-Host '       (c) QUEST ITEMS - CONFIRMED ON A SCREEN 2026-09-08. No quest item' -ForegroundColor Green
        Write-Host '           dropped in a live session with no Omok quest. Do not re-test.'
        Write-Host '           Still unwatched: the POSITIVE case - take the quest and confirm'
        Write-Host '           the piece starts dropping again.'
        Write-Host '       (d) LEVEL UP needs TWO clients on ONE map. Level one, watch the'
        Write-Host '           OTHER screen. The server cannot confirm this one.'
        Write-Host '       (e) !tool - CONFIRMED WORKING ON A SCREEN 2026-09-08. Do not spend' -ForegroundColor Green
        Write-Host '           a run on it. Still unwatched: the daily REFUSAL - run it twice'
        Write-Host '           in one day and confirm it refuses IN WORDS, never silently.'
        Write-Host '           Original steps: type it in chat on a NON-GM account (public).'
        Write-Host '             box with their PORTRAIT + 3 lines -> works. Pick Level up:'
        Write-Host '                                                level, EXP line, +5 AP'
        Write-Host '             3 lines but NO PORTRAIT -> speaker template not resolving.'
        Write-Host '                                        The ONE thing tests cannot see -'
        Write-Host '                                        look at the portrait on purpose.'
        Write-Host '             "is not a command"      -> dispatcher never reached it'
        Write-Host '             nothing at all          -> check world-ch0.log for the 0x00E7'
        Write-Host '           Run !tool again the same session: it must REFUSE IN WORDS and'
        Write-Host '           log NOTHING PAID. Reset is UTC midnight. Leaf Points are per'
        Write-Host '           ACCOUNT; Level up and Henesys are per CHARACTER. NO RESET.'
        Write-Host '       (f) THE ADMINISTRATOR HERSELF, Henesys. Click them: they must give'
        Write-Host '           their QUEST or their greeting, NEVER the favours menu. That is why'
        Write-Host '           (e) is a command - the click fork is keyed on their TEMPLATE, so'
        Write-Host '           a summoned copy would send bytes identical to clicking them.'
        Write-Host '       (g) CHAIRS - Set Up chairs CONFIRMED on two screens. Do not' -ForegroundColor Green
        Write-Host '           re-test sit/stand/model/relay. Two gaps remain:'
        Write-Host '             - Blue Seal Cushion must add 10 MP and NO HP. If it adds'
        Write-Host '               30 HP the chair table was defaulted. Never seen.'
        Write-Host '             - MAP CHAIRS: A FIX IS IN. BUILT TODAY, NEVER SEEN.' -ForegroundColor Yellow
        Write-Host '               Sit on a Henesys bench. You should seat AND STAY'
        Write-Host '               seated; a movement key gets you up.'
        Write-Host '                 seats        -> 0x0252 was the missing packet, done'
        Write-Host '                 nothing      -> grep world-ch0.log for 0x0252. If it WENT'
        Write-Host '                                 OUT the suspect is the handler gate at'
        Write-Host '                                 1428341d3, NOT a body length.'
        Write-Host '                 you stand up by yourself, or 0x00DA ffff in the log'
        Write-Host '                              -> the client REFUSED on position: it'
        Write-Host '                                 wants seatX-10 <= myX < seatX+10 and'
        Write-Host '                                 seatY-30 <= myY < seatY+30. That is a'
        Write-Host '                                 DIFFERENT outcome from "nothing".'
        Write-Host '               Why it never worked: 0x02AD is dispatched by a hash-only'
        Write-Host '               lookup and the local player is not in that hash, so it'
        Write-Host '               could never address the sitter whatever body it carried.'
        Write-Host '           Recovery check: sit, stand still 15s, HP +40 per tick.'
        Write-Host '       (h) THE RELAY IS LIVE AND IT KILLED A CLIENT ONCE.' -ForegroundColor Red
        Write-Host '           0x02AD goes out on every sit, bench attempt and stand. An'
        Write-Host '           earlier 12-byte version faulted Tester2 5 ms after sending;'
        Write-Host '           it is 13 bytes now and a test pins that.'
        Write-Host '           IF A SECOND CLIENT EVER EXITS WHILE SOMEONE SITS, say so and' -ForegroundColor Red
        Write-Host '           do NOT close the survivor. Check world-ch0.log for 0x02AD and the'
        Write-Host '           hook log for CLIENT FAULT 0xc0000005.'
        Write-Host '           Two clients share one hook log, so chairprobe needs --pid:'
        Write-Host '             cd "C:\MapleCW"; python tools\chairprobe.py --pid <n>'
        Write-Host '       (i) ANTI-CHEAT GATE - TIME-CRITICAL, at ~60s AND ~150s of life:' -ForegroundColor Yellow
        Write-Host '             cd "C:\MapleCW"; python tools\gatescan.py'
        Write-Host '           The gate flips 0 -> 2 between 38s and 194s in every session'
        Write-Host '           (38 dumps, clean split). Two readings bracket it.'
        Write-Host '       (j) TRADE INVITE POPUP - BUILT TODAY, NEVER SEEN. Two clients.' -ForegroundColor Yellow
        Write-Host '           Tester2 sends Cobalt a trade request. Cobalt must get a'
        Write-Host '           "Trade request from Tester2" popup. Yesterday: nothing at all.'
        Write-Host '             popup     -> the type field was the bug, and it is fixed'
        Write-Host '             no popup  -> grep world-ch0.log for 0x0575. If it WENT OUT, the'
        Write-Host '                          cause is field 3, the one guessed field: a hit'
        Write-Host '                          on the local lookup auto-declines SILENTLY.'
        Write-Host '           ACCEPT DOES NOTHING AND THAT IS EXPECTED, not a regression:' -ForegroundColor Yellow
        Write-Host '           the trade WINDOW is mode 4, whose body is undecoded. It is'
        Write-Host '           answered with nothing rather than a guess - a guessed body'
        Write-Host '           killed a client yesterday.'
        Write-Host '       (k) SHANKS'' FREE TRIP WAS REBUILT 2026-09-16 (the paid sail is fine).' -ForegroundColor Magenta
        Write-Host '           The waiver line was torn down by the SetField it shipped with.'
        Write-Host '           Beginner who finished Mai, Yes -> the waiver box, still in'
        Write-Host '           Southperry; dismiss it -> Lith Harbor, mesos unchanged -> fixed'
        Write-Host '           A first-job character who finished Mai -> NO box, pays 1000.'
        Write-Host '           LYN (Lith Harbor) has a SIXTH line since 2026-09-16: "Southperry' -ForegroundColor Magenta
        Write-Host '           (Maple Island) - 20000 mesos". Pick it with 20,000+ -> Southperry,'
        Write-Host '           grey "lost mesos (-20000)" -> fixed. With less -> refusal quotes'
        Write-Host '           20000. The VIP Cab and the Henesys cab have NO such line.'
        Write-Host '       (l) THE FARE LINE MUST BE GREY. Any fare - Shanks or a taxi -' -ForegroundColor Yellow
        Write-Host '           prints a grey chat line "You have lost mesos (-1000)".'
        Write-Host '           A RED "You have received Meso Penalty" means the old path is'
        Write-Host '           still live. Same check for (k) and for any taxi.'
        Write-Host '       (m) DROPPING MESOS - NEW TODAY, NEVER SEEN. Until today the' -ForegroundColor Yellow
        Write-Host '           server decoded the request and REFUSED it. Drop 10 mesos'
        Write-Host '           while standing still:'
        Write-Host '             coins on the floor, counter down 10, you can pick them'
        Write-Host '             back up            -> done'
        Write-Host '             nothing + a chat line saying why'
        Write-Host '                                -> a refusal fired, and the line says'
        Write-Host '                                   which. That is working as built.'
        Write-Host '             nothing, no line   -> the FREEZE is back. Grep world-ch0.log'
        Write-Host '                                   for 0x0143 and say so.'
        Write-Host '           THEN MOVE AN ITEM IN YOUR BAG straight after.' -ForegroundColor Yellow
        Write-Host '           An unanswered 0x0143 latches +0x2330 and kills the bag, the'
        Write-Host '           AP buttons and the cash shop for the rest of the session.'
        Write-Host '           A dead bag means the reply is not clearing the latch.'
        Write-Host '           Dropping your WHOLE balance is allowed. Anyone on the map'
        Write-Host '           can pick the coins up, not just you.'
        Write-Host '    0i. THE OVERNIGHT RUN - the goal is to SURVIVE, not to measure.' -ForegroundColor Green
        Write-Host '       -PoolSentry -SentryQuiet -SentryRepair -PinPatches -GuardPage'
        Write-Host '       -GuardBucket now DEFAULTS to 0x20+0x40 - TWO classes. Type nothing.' -ForegroundColor Yellow
        Write-Host '       NEW TODAY: the LAUNCHER now ships the guard page - DEFAULT_SESSION is' -ForegroundColor Green
        Write-Host '       mode=2,create=on,guardpage=0x20+0x40, so an ordinary Start Game arms' -ForegroundColor Green
        Write-Host '       it with no flag. -GuardPage still matters HERE because this script' -ForegroundColor Green
        Write-Host '       writes a session PIN that replaces that default.'
        Write-Host '       KILL SWITCH, no rebuild: create maplecw-hook.guardpage.off beside' -ForegroundColor Yellow
        Write-Host '       MapleStory.exe (or guardpage = "off" in maplecw-launcher.toml).' -ForegroundColor Yellow
        Write-Host '       Absent means ON. DELETE IT before a measurement run: it wins over a' -ForegroundColor Yellow
        Write-Host '       pin on that token, and the marker read-back below will then refuse.' -ForegroundColor Yellow
        Write-Host '       AND NOTHING ELSE. NO -Probe WATCHES ON THIS RUN.' -ForegroundColor Red
        Write-Host '       14:47 today: this command PLUS five watch@ targets, and the client' -ForegroundColor Red
        Write-Host '       closed the instant it entered the field - the first death here with' -ForegroundColor Red
        Write-Host '       NO exception and NO dump. The guard page armed clean and both'
        Write-Host '       classes first-free controls fired, so it is not the suspect; the'
        Write-Host '       log ends ON the watch firing. But TWO things were changed at once,'
        Write-Host '       so that is a suspicion, not a fact. Guard page ALONE this time.'
        Write-Host '       If you add watches later the grammar is ONE watch@:'
        Write-Host '         -Probe "watch@140c93530,140c936a0,140c93810"     RIGHT'
        Write-Host '         -Probe "watch@140c93530,watch@140c936a0"         WRONG'
        Write-Host '       The launcher now refuses the wrong form outright.'
        Write-Host '       SETTLED 12:01 TODAY, do NOT re-test: the guard page armed on a client' -ForegroundColor Green
        Write-Host '       for the first time, control PASSed, and the client ran 1h57m - the' -ForegroundColor Green
        Write-Host '       longest session this project has had. The allocator inline hook does' -ForegroundColor Green
        Write-Host '       not destabilise the client. That question is closed.' -ForegroundColor Green
        Write-Host '       IT STILL DIED, on a class we were NOT quarantining: a 0x40 slot whose'
        Write-Host '       vtable pointer had been incremented by 2. The writer holds a stale'
        Write-Host '       ADDRESS, not a class, so one class is whack-a-mole - hence two.'
        Write-Host '       AND IT RAN OUT OF RESERVE AT SIX MINUTES: 0x20 burst to 627172'
        Write-Host '       allocations in its FIRST minute then ran at 1560/s, so the 1048576'
        Write-Host '       cursor was spent before anything could age out at ten minutes, and'
        Write-Host '       419588 allocations fell back to the client pool. FIXED: 8388608 slots'
        Write-Host '       (32 GB of address space, which is nearly free) and the 40-byte-a-slot'
        Write-Host '       metadata is committed lazily instead of 320 MB up front.'
        Write-Host '       WHAT TO WATCH, in client-patched\maplecw-hook.log:'
        Write-Host '         "GUARD PAGE ARMED ... 0x20+0x40 ... control PASS"'
        Write-Host '                  -> armed on BOTH classes. The line states the sizing model'
        Write-Host '                     and its headroom; "NOT enough headroom" inside it means'
        Write-Host '                     expect a fall-back, and say so.'
        Write-Host '         "the FIRST free of class 0x40 came back through our HeapFree shim"'
        Write-Host '                  -> THE NEW CONTROL, expected within seconds. 0x40 has' -ForegroundColor Yellow
        Write-Host '                     never been quarantined, and 53 of the client''s 56 free' -ForegroundColor Yellow
        Write-Host '                     sites are inlined and untraced - until this line' -ForegroundColor Yellow
        Write-Host '                     appears, "0x40 frees reach us" is a GUESS.' -ForegroundColor Yellow
        Write-Host '         "NEVER FREED" -> that control did NOT come. The class is leaking a'
        Write-Host '                     page per allocation. Stop the run and report it.'
        Write-Host '         "N FELL BACK" -> should now be ZERO all night. Any number at all,'
        Write-Host '                     say WHICH CLASS - the heartbeat prints them apart now.'
        Write-Host '         "pool allocations seen by class: 0x10 N, 0x20 N, 0x40 N, 0x80 N"'
        Write-Host '                  -> THE MEASUREMENT THIS RUN MAKES EVEN IF IT DIES. Only' -ForegroundColor Yellow
        Write-Host '                     0x20 has ever been measured (1560/s); these four' -ForegroundColor Yellow
        Write-Host '                     numbers decide whether all four classes can be' -ForegroundColor Yellow
        Write-Host '                     quarantined at once. Paste them whatever happens.' -ForegroundColor Yellow
        Write-Host '         "GUARD PAGE - STALE WRITE ... RIP R" -> THE ANSWER: the writer,'
        Write-Host '                     named AND neutralised. It now also says how long ago the'
        Write-Host '                     slot was allocated and freed, which tests the 180s clock'
        Write-Host '                     directly. Several of these with the client still up is'
        Write-Host '                     the run WORKING, not failing.'
        Write-Host '       RUN FIVE MINUTES FIRST, then leave it overnight. TWO classes at once'
        Write-Host '       has never run: if the client dies inside five minutes, relaunch with'
        Write-Host '       -GuardBucket 0x20 - the configuration that already survived 1h57m -'
        Write-Host '       and say which of the two it was.'
        Write-Host '       Costs ~33 MB at arm and ~100 MB of live pages per class; the metadata'
        Write-Host '       grows with the cursor. Retired pages are decommitted and cost nothing.'
        Write-Host '       A CLEAN POOL IS NOT SUCCESS. That death had 0 damaged headers in' -ForegroundColor Yellow
        Write-Host '       174528 slots: the writer damages LIVE objects, and the sentry only' -ForegroundColor Yellow
        Write-Host '       ever checks free headers.' -ForegroundColor Yellow
        Write-Host '       NO -SentryWriteWatch: it only OBSERVES, and overnight it is 160'
        Write-Host '       windows of read-only pages and single-stepped writes for no'
        Write-Host '       protection at all. -SentryQuiet keeps the repair and drops the dumps,'
        Write-Host '       the stack scan and the 100ms walk.'
        Write-Host '    0j. WHAT THE GUARD PAGE NOW COVERS, and what it does not.' -ForegroundColor Yellow
        Write-Host '       It quarantines a SET of pool size classes, not one: every allocation'
        Write-Host '       of a watched class gets its own page and its free decommits that page'
        Write-Host '       and holds the address back for 200s (600s until 2026-09-16). All three deaths on record -'
        Write-Host '       0x40 map node +2, 0x20 tree node set to -1, 0x40 vtable +2 - are'
        Write-Host '       inside 0x20+0x40. 0x10 and 0x80 are NOT watched by default and their'
        Write-Host '       churn is unmeasured; the "seen by class" counters are what would'
        Write-Host '       justify adding them, and -GuardBucket all is the flag if they do.'
        Write-Host '       History below.'
        Write-Host '    0j. NAME THE WRITER. RUN 1 (70 min) MISSED THE STORE and found three' -ForegroundColor Yellow
        Write-Host '       things: the writer RE-HITS slots it hit before (3 of 12 repaired'
        Write-Host '       slots damaged again, one twice); the sentry ignored re-hits and'
        Write-Host '       that silence killed the client (FIXED - re-hits are repaired again);'
        Write-Host '       the 28-byte alloc from 0x14491cafd fires 180s apart ~100ms before'
        Write-Host '       every catch (14/14); and the watch covered only 10% of the pool'
        Write-Host '       (FIXED - pages of caught headers are pinned every window).'
        Write-Host '       RUN 3: 12/12 catches, re-hits caught, one window (guard bug, FIXED).'
        Write-Host '       RUN 4: 11 windows, 8 pinned pages, and the store landed INSIDE a'
        Write-Host '       window on a PINNED page - uncaught: the first write to a page opened'
        Write-Host '       it until the next sweep. FIXED: a page is open for ONE instruction'
        Write-Host '       (trap flag + re-protect on the step), so every write faults.'
        Write-Host '       AGAIN, same recipe, 20+ min. 6 of 15 firings landed off-pool: the'
        Write-Host '       catch will come from one that lands on a pinned header.'
        Write-Host '         "THE WRITER: ... from RIP R"  -> THE ANSWER. R is the instruction,'
        Write-Host '                    and the module it sits in also settles "is this OURS"'
        Write-Host '         "saw a write into a watched page" -> liveness. Expect several'
        Write-Host '         "window #N open ... control PASS" -> armed and self-tested'
        Write-Host '         windows but ZERO write faults -> say exactly that. It is the'
        Write-Host '                    instrument, NOT evidence the client did not write'
        Write-Host '         no window at all -> no catch, so there was no phase to predict'
        Write-Host '       The probe rides along; its rdx=5 hit from 0x14491cafd should keep'
        Write-Host '         -PinPatches -Probe "watch@1415db360:ret,141b2a280:rdx=0,140ca61d0:hits=400"'
        Write-Host '         landing ~100ms before each catch. rdx=6 is the 240s clock, not it.'
        Write-Host '       research/the-180-second-clock-2026-09-07.md'
        Write-Host '    0l. 2ND/3RD JOB SKILLS: 149 audited, most built, NONE seen yet.' -ForegroundColor Yellow
        Write-Host '       research/second-third-job-audit-2026-09-07.md. !job, !learn, ONE'
        Write-Host '       at a time. (a) SWORD BOOSTER: icon, faster swings, HP AND MP -30.'
        Write-Host '       (b) HYPER BODY: max HP +10% and regen goes PAST the old max.'
        Write-Host '       (c) POWER GUARD: you take 80%, the mob bar drops 20% of the hit.'
        Write-Host '       (d) COMBO: cast and COUNT ORBS - zero = value-1 convention right,'
        Write-Host '           one = every number is one high. Hits add up to 3; Coma clears.'
        Write-Host '       (e) SOUL ARROW: arrows stop moving. STRAFE takes 3, ARROW RAIN 8.'
        Write-Host '       (f) HEAL: +40% of max, blue number; with Bless 41%.'
        Write-Host '       (g) MAGIC GUARD STAYS ON - it used to expire on the next loop pass.'
        Write-Host '       (h) TELEPORT MP stays spent. (i) Element Amp: Fire Arrow costs 16.'
        Write-Host '       (j) MESO GUARD: a hit costs mesos and less HP; broke = full HP.'
        Write-Host '       NOT BUILT (say so, do not test): summons, Puppet, Mystic Door, ALL'
        Write-Host '       mob statuses (slow/seal/stun/freeze/DoT), Pickpocket, Meso'
        Write-Host '       Explosion, Meso Saver, Chakra, Crit/Nimble Recovery, FA HP absorb.'
        Write-Host '    0k. THE OTHER FREE: -FreeGuard. NOT on the same run as 0j.'
        Write-Host '       The field crash died with the repair ON and the pool CLEAN: the'
        Write-Host '       same disease came out at a different free - PCOM handing a POOL'
        Write-Host '       CHUNK to the NT heap while tearing down WZ properties on a map'
        Write-Host '       change. -FreeGuard refuses that one free (-FreeGuardObserve logs'
        Write-Host '       and frees anyway). Needs -PinPatches. Watch for "FREE GUARD ARMED"'
        Write-Host '       then a liveness line every 120s: ZERO passes = the shim is not on'
        Write-Host '       the free path, so no refusal count from that run means anything.'
        Write-Host '       Keep it OFF for 0j - that run measures whether our patches matter.'
        Write-Host '    0d. STILL NEEDS A CAPTURE - do this and report the inbound opcode:'
        Write-Host '       DROP MESOS: try to drop mesos. It does nothing today because the'
        Write-Host '       client''s meso-drop request has never been captured. Note what'
        Write-Host '       "<- 0x...." appears in world-ch0.log when you try (or that none does).'
        Write-Host '    1. THE MOB FLINCH. A non-controller hits a mob: from the'
        Write-Host '       SECOND hit it should flinch and slide. First hit never'
        Write-Host '       will - the grant ships with that swing.'
        Write-Host '    2. EXP SHARING has NEVER executed. 329 kill payouts in the'
        Write-Host '       archive, ZERO carrying a damage fraction. Two clients have'
        Write-Host '       never killed the SAME mob. Do that.'
        Write-Host '    3. T11/T10, single-client, still untested.'
        Write-Host '    4. REGISTRATION AND RECOVERY (new 2026-09-05, no client needed for'
        Write-Host '       the launcher half). As the GM type !registrationcode - a chat'
        Write-Host '       notice shows an 8-character code, XXXX-XXXX, and world-ch0.log must'
        Write-Host '       NOT contain it. In the launcher: Register tab, any username, an'
        Write-Host '       email, a password WITHOUT a digit -> refused on the spot, the'
        Write-Host '       code still live; with a digit -> "account created", back on the'
        Write-Host '       Sign in tab, name filled in, Login works. Then !recoverycode'
        Write-Host '       <that email> in game; Forgot password tab with the email, the'
        Write-Host '       code and a new password -> sign in with the new one, old refused.'
        Write-Host '       Wrong identity + right code -> refused AND the code still works.'
        Write-Host '  Full text: Get-Help on this script.'
        Write-Host ''
        Write-Host '  THIS WINDOW IS THE SERVER. Close it to stop.' -ForegroundColor Green
        Write-Host '  The CLIENT comes from maplecw-launcher.exe (asks for administrator).'
        Write-Host '  ACCOUNTS: maplecw (GM), tester / tester@example.test (NOT a GM).'
        Write-Host '  A forgotten password is a reset and never asks for the old one:'
        Write-Host '    & ".\target\release\maplecw-useradd.exe" --db ".\maplecw.db" --passwd maplecw'
        Write-Host ''
        Write-Host '  CONFIRMED ALREADY, DO NOT RE-TEST.' -ForegroundColor Green
        Write-Host '  Launcher, UAC, wrong-password refusal, second account by email,'
        Write-Host '  masked email, one damage number, MP costs, selling, warrior casts.'
        Write-Host ''
        Write-Host '  SIGN IN IN ANY ORDER NOW - this changed.' -ForegroundColor Cyan
        Write-Host '  The login server used to serve whichever account claimed LAST, so'
        Write-Host '  one player got the other characters. That was real and it is fixed:'
        Write-Host '  one claim per launch, matched by asking the OS which process owns'
        Write-Host '  the socket. Two accounts on 127.0.0.1 now each get their own.'
        Write-Host '  If both clients STILL show the same account, that is a new finding.'
        Write-Host '  AND TWO CLIENTS RUN NOW - that used to be UNKNOWN here.'
        Write-Host '  Launch them ONE AT A TIME: wait for the first to reach the'
        Write-Host '  character list before pressing Login in the second, because'
        Write-Host '  maplecw-hook.identity is one shared file.'
        Write-Host ''
        Write-Host '  T2b: the departure handover. MEASURED on the wire already -' -ForegroundColor Magenta
        Write-Host '  30 mobs handed over, then 1170 reports from the heir over 41 s.'
        Write-Host '  What is unmeasured is the SCREEN: do they keep walking, or'
        Write-Host '  freeze, or jump to their spawn points.'
        Write-Host ''
        Write-Host '  T11 AND T10 are the single-client half, and still untested.' -ForegroundColor Magenta
        Write-Host '  T11 CROSSES TO ANOTHER CONTINENT - 87 maps, reached by the ship.'
        Write-Host '  ORBIS ITSELF ALREADY LOADED (2026-08-28, 0x00DC accepted, NPCs'
        Write-Host '  drew). This plan claimed it never had, for a week. What is'
        Write-Host '  untested is EL NATH - not "can the client survive'
        Write-Host '  a map over there", which is answered yes.'
        Write-Host '  If a client dies loading one, THAT is the finding - worth more'
        Write-Host '  than the advancement it was on the way to.'
        Write-Host ''
        Write-Host '  T10 IS A WALK TOO, through four more maps nobody has loaded.' -ForegroundColor Magenta
        Write-Host '  The second job advancement now exists end to end, and it goes'
        Write-Host '  through FOUR MAPS NOBODY HAS EVER STOOD IN. Each has exactly one'
        Write-Host '  portal - the spawn point - so there is no way in or out on foot.'
        Write-Host '  If a warp into one goes wrong, the symptom is being STUCK.'
        Write-Host '  (The old T3/T4 are answered and gone: the type-6 menu renders from'
        Write-Host '   the server. Lyn and the Cab both drew one and both were clicked.)'
        Write-Host ''
        Write-Host '  T1. TWO CLIENTS, SAME MAP. First, and T2 waits on it.' -ForegroundColor White
        Write-Host '      The 0x0224 body WAS 7 bytes short and is fixed - the stat'
        Write-Host '      block is 131, not 124. So this is no longer expected to die.'
        Write-Host '        a) the other appears, dressed, and walks -> THE result'
        Write-Host '        b) a client STILL dies -> wrong somewhere else. Say WHICH'
        Write-Host '           died, arriving or already there. The hook log will have'
        Write-Host '           NO dispatch line for 0x0224 (written on return)'
        Write-Host '        c) nothing appears and nothing dies -> DROPPED, not misread.'
        Write-Host '           No more body work helps; next run watches 0x1429ba60b'
        Write-Host '        d) they stand at the map ORIGIN until they move -> expected'
        Write-Host '        e) origin AND they stay there while walking -> 0x0293'
        Write-Host '  T2. KILL ONE MOB TOGETHER, both hitting it.' -ForegroundColor White
        Write-Host '      This used to say EXPECT IT TO LOOK BROKEN. It should now look'
        Write-Host '      RIGHT - and only the test suite says so. Two clients have never'
        Write-Host '      been connected to this server at once.'
        Write-Host '       a) FIRST JUST WATCH, hit nothing. Both screens should show the'
        Write-Host '          mobs in the SAME PLACES walking the SAME WAY.'
        Write-Host '            they agree            -> 0x03D9 is arriving'
        Write-Host '            they drift apart      -> the rebroadcast is not landing'
        Write-Host '            ONE screen is frozen  -> that client was granted nothing'
        Write-Host '       b) NOW BOTH HIT ONE MOB.'
        Write-Host '            bar moves on BOTH screens for EITHER hit -> 0x03F0'
        Write-Host '            it dies on both                          -> 0x03D1'
        Write-Host '            only your own hits move it -> the publish never left'
        Write-Host '       c) THE DROP IS NOT SHARED, on purpose. Only the TOP DAMAGER'
        Write-Host '          sees it - not the killer. Over-damage does not count.'
        Write-Host '            only the top damager sees it -> correct'
        Write-Host '            BOTH see it    -> it went map-wide. A real bug'
        Write-Host '            NEITHER sees it -> ranked to a client that had left'
        Write-Host '          (Parties would share drops. THERE IS NO PARTY SYSTEM YET -'
        Write-Host '           the buttons are refused so the UI cannot freeze.)'
        Write-Host '       d) THE EXP LINE: white for most damage, yellow for less'
        Write-Host '            only the killer paid -> the fact never crossed the bus'
        Write-Host '            both white -> the majority flag is wrong'
        Write-Host '            helper paid in FULL -> the split is not applied'
        Write-Host '  T2b. NOW ONE OF YOU LEAVES - a portal, or CLOSE THE CLIENT.' -ForegroundColor White
        Write-Host '       WATCH THE SCREEN OF THE ONE WHO STAYS. That is the whole test.'
        Write-Host '       The leaver''s mobs are handed to whoever is left, and that'
        Write-Host '       client is told without moving. Before 2026-09-01 nothing was'
        Write-Host '       sent and they stood still until somebody walked a portal.'
        Write-Host '            they carry on walking      -> the handover works'
        Write-Host '            they FREEZE and stay frozen -> nothing was handed over'
        Write-Host '            frozen until you walk a portal -> the old behaviour'
        Write-Host '            they JUMP to spawn points  -> the grant sent the spawn'
        Write-Host '                       position, not where the mob was standing'
        Write-Host '       CLOSING THE CLIENT is the better half: no log out runs, and'
        Write-Host '       it is the exit the leaving player cannot see.'
        Write-Host '       grep world-ch0.log for "mob control:" - it names the count and'
        Write-Host '       the recipient, so this needs no second launch to read.'
        Write-Host '  T11. THIRD JOB. THE ONE. Set up with:' -ForegroundColor Yellow
        Write-Host '         !job 110   !exp 31545355   !map 20001000'
        Write-Host '       (a level-70 Fighter in El Nath. If the level comes out'
        Write-Host '        wrong, say what it was - the exp curve is ours.)'
        Write-Host '       NO FERRY any more: El Nath is on foot from Orbis, or a scroll.'
        Write-Host '       a) a snowy town -> El Nath loads; black screen / dies -> THE finding'
        Write-Host '       b) CLICK EUREK: their own wandering line, NO menu -> DONE'
        Write-Host '            a menu of stops -> old build'
        Write-Host '       c) WALK RIGHT into Chief Residence. Four NPCs inside.'
        Write-Host '       d) CLICK TYLUS (they serve Fighter/Page/Spearman).'
        Write-Host '            "You are a Crusader now" -> DONE. Then open the skill'
        Write-Host '                       window: a THIRD page with points on it'
        Write-Host '            no third page -> the SP pool key is wrong. Say both'
        Write-Host '                       halves - the job still changed'
        Write-Host '            "come back at Level 70" -> the !exp did not land'
        Write-Host '       e) CLICK ROBEIRA / RENE / AREC. All must REFUSE, naming the'
        Write-Host '          BRANCH rather than the level.'
        Write-Host '       f) Orbis booth, Platform Usher: only the platform line -> DONE'
        Write-Host '       NO third-job test exists in this client - no quest, no field,'
        Write-Host '       no marbles. Level 70 + the right 2nd job IS the gate.' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  T10. THE SECOND JOB ADVANCEMENT. ALSO NEW. Set up with:' -ForegroundColor Yellow
        Write-Host '         !job 100   !item 1302000   !exp 548637   !map 10004023'
        Write-Host '       (548637 is the curve summed 1->30; one !exp crosses all of'
        Write-Host '        them. THERE IS NO !level.) Then PUT YOUR AP INTO STR - !job'
        Write-Host '        does not move it, and those mobs hit for ~204.'
        Write-Host '       a) CLICK THE EXAMINER there (Warrior Job Instructor).'
        Write-Host '            you end up somewhere else -> the warp works. NOBODY HAS'
        Write-Host '                       EVER BEEN ON THAT MAP. This is the step'
        Write-Host '            black screen / client dies -> say which; world-ch0.log names'
        Write-Host '                       the map in its SetField line'
        Write-Host '            nothing happens -> the click never routed'
        Write-Host '       b) KILL ANYTHING IN THERE. Every mob drops one Dark Marble.'
        Write-Host '            no marble -> the map gate; world-ch0.log names the map used'
        Write-Host '       c) CLICK THE NPC INSIDE. It is the ONLY way out.'
        Write-Host '            nothing happens -> YOU ARE STUCK. Return scroll or !map,'
        Write-Host '                       and say so - worst failure in this run'
        Write-Host '       d) !item 4031017 30, then click the examiner OUTSIDE again.'
        Write-Host '            30 leave the bag, The Proof of a Hero arrives -> good'
        Write-Host '            they warp you back in -> they counted fewer than 30'
        Write-Host '       e) !map 10004003, CLICK DANCES WITH BALROG holding the proof.'
        Write-Host '            a box listing Fighter / Page / Spearman -> pick one'
        Write-Host '            job changes AND the skill window has a 2nd page -> DONE'
        Write-Host '            job changes, page empty -> the SP pool key is wrong'
        Write-Host '       WITHOUT the proof they must REFUSE. If they advance you anyway,'
        Write-Host '       the whole chain is optional - report it even though it looks'
        Write-Host '       like the feature working.' -ForegroundColor Yellow
        Write-Host '  T0. ANSWERED - two clients run. Nothing to do.' -ForegroundColor Green
        Write-Host '      grap-stub suppresses FindWindowA("MapleStoryClass") and'
        Write-Host '      redirects kernel32 CreateMutex through its forwarder SLOT'
        Write-Host '      (patching kernelbase code hung the client - three launches).'
        Write-Host '      The second client survives Global\WvsClientMtx.'
        Write-Host '      Sign out in the launcher to swap accounts - no restart.'
    Write-Host '  T9. COLLECTION TOOLTIP - carries its own control.' -ForegroundColor Cyan
    Write-Host '      Walk one portal with the Vicious quest accepted, then OPEN'
    Write-Host '      THE INVENTORY. Only the Etc tab uses the new quiet mode 5;'
    Write-Host '      Use / Set Up / Cash still use the old ADD and are the control.'
    Write-Host '        no tooltip + Etc still full -> it works, roll it out'
    Write-Host '        no tooltip + Etc EMPTY, others full -> THE FAILURE THAT'
    Write-Host '                   MATTERS. Say so; it gets reverted'
    Write-Host '        tooltip still pops -> the chain is wrong; world-ch0.log shows'
    Write-Host '                   the mode-5 bodies went out'
    Write-Host '        ALL FOUR tabs empty -> not mode 5, something else broke'
    Write-Host '      No server has ever sent a mode 5. That is why only one tab.' -ForegroundColor Yellow
    Write-Host '  T8. POOL SENTRY (only with -PoolSentry). It WATCHES, it does' -ForegroundColor Cyan
    Write-Host '      not fix. Play 8-10 min; first catch expected ~300s in-field.'
    Write-Host '        FINDING #1 at 250-400s -> the catch. Contents <=106ms old:'
    Write-Host '                   fresh body = refcount/flag; long-lived body or a'
    Write-Host '                   live predecessor = the +0x24 overrun; damaged slot'
    Write-Host '                   ON THE FREE LIST kills the underrun reading, live'
    Write-Host '        FINDING [EARLY - SUSPECT] under 120s -> suspect the tool.'
    Write-Host '                   Zero heap deaths under 192s in the whole archive'
    Write-Host '        0 findings past ~600s in-field -> a RESULT. First unbiased'
    Write-Host '                   sample ever; every rate point so far is a death'
    Write-Host '        NOVEL -> the biggest result available. Breaks 14-of-14'
    Write-Host '        no POOL SENTRY line -> marker or hook, not the allocator'
    Write-Host '      Leave -HeapFix OFF - it voids the free-list argument.' -ForegroundColor Yellow
    Write-Host '  T7. ANSWERED - the client carried a real credential.' -ForegroundColor Green
    Write-Host '      login.log, twice in one run: 0x0073 IDENTITY: ACCEPTED and'
    Write-Host '      SPENT, length 26. Every capture before 2026-08-29 was 0.'
    Write-Host '      The LOGIN socket is identified by a credential now, not by'
    Write-Host '      inference. Nothing in game authenticates; that is unchanged.'
    Write-Host '  T5. RETURN SCROLLS. RUN AS maplecw, NOT AS tester.' -ForegroundColor White
    Write-Host '      tester has is_gm = 0, so !item comes back as a CHAT BALLOON'
    Write-Host '      and grants nothing - which reads exactly like "the client'
    Write-Host '      never sent 0x010E, the path is dead code". That would be a'
    Write-Host '      FALSE NEGATIVE on the only question T5 exists to answer.'
    Write-Host '      A ! command answered by a chat balloon is the GM GATE.' -ForegroundColor Yellow
    Write-Host '      ANSWERED 2026-09-18: the client sends 0x0123 for a scroll, never 0x010E;' -ForegroundColor Magenta
    Write-Host '      the handler sat on 0x010E for three weeks. Wired now. NEVER ON A SCREEN.' -ForegroundColor Magenta
    Write-Host '      !item 2030004 2 then !item 2030009 1, on Victoria Island:'
        Write-Host '        Henesys: you land there, stack reads 1 -> fixed' -ForegroundColor Green
        Write-Host '        El Nath: notice naming Ossyria/Victoria, scroll KEPT, next item use' -ForegroundColor Green
        Write-Host '          still works -> fixed' -ForegroundColor Green
        Write-Host '          scroll vanishes on a refusal -> the guard is broken' -ForegroundColor Yellow
        Write-Host '          nothing at all + no 0x0123 in the log -> the client''s own map rule;' -ForegroundColor Yellow
        Write-Host '          nothing + the 0x0123 IS there -> paste it and the reply' -ForegroundColor Yellow
        Write-Host '        !map 20001000, !item 2030008 1: Orbis works; 2030004 refused there' -ForegroundColor Green
        Write-Host '  TS. THE HAIR SALONS. OWNERS do styles (Natalie 10001044, Don Giovanni' -ForegroundColor Magenta
        Write-Host '      10003005), ASSISTANTS do colour (Brittany, Andre). NEVER ON A SCREEN.' -ForegroundColor Magenta
        Write-Host '      1. Natalie with no 5150100/5150000: both icons + "Cash Shop" line -> fixed'
        Write-Host '      2. !item 5150100 1: a 1-line menu with the icon; pick -> a pick-a-look window,'
        Write-Host '         YOUR gender''s 6 REG styles; pick -> hair changes in place, colour kept,' -ForegroundColor Green
        Write-Host '         coupon gone, line names it -> fixed' -ForegroundColor Green
        Write-Host '         wrong set/gender -> paste the AVATAR line; nothing opens -> the 0x0a' -ForegroundColor Yellow
        Write-Host '         body; black though you were not -> beauty.txt missing on the box' -ForegroundColor Yellow
        Write-Host '      3. hold both: 2 menu lines; the Mystery line rolls a VIP style AT ONCE,'
        Write-Host '         colour kept, only the Mystery coupon gone'
        Write-Host '      4. Brittany: no colour coupon -> 5151100 + 5151000 icons + Cash Shop; with'
        Write-Host '         5151100 -> your style in 8 colours, pick one, style kept; 5151000 -> random'
        Write-Host '      5. Kerning: Don Giovanni shows the KERNING lists; Andre = colours, same'
        Write-Host '      6. the other client sees it without a map change -> fixed'
        Write-Host '  TP. THE PLASTIC SURGERIES. Henesys 10001043: Denma = faces, Dr. Feeble = skins;' -ForegroundColor Magenta
        Write-Host '      Orbis 20000031: Franz = faces, Riza = skins. (Kerning has none in this client.)' -ForegroundColor Magenta
        Write-Host '      1. Denma, no coupon: 5152200 + 5152000 icons + Cash Shop -> fixed'
        Write-Host '      2. !item 5152200 1, pick the line: your gender''s REG faces (7 M / 6 F); pick ->'
        Write-Host '         face changes in place, EYE COLOUR kept, coupon gone, line names it -> fixed' -ForegroundColor Green
        Write-Host '      3. !item 5152000 1, the Mystery line -> a random VIP face at once'
        Write-Host '      4. Dr. Feeble, !item 5153000 1 (no mystery skin item exists): 7 skins in the'
        Write-Host '         box - FIRST SKIN BOX EVER; blank/identical figures -> ids need +12000,' -ForegroundColor Yellow
        Write-Host '         paste the AVATAR line. Pick -> skin changes in place, SKIN bit -> fixed' -ForegroundColor Green
        Write-Host '      5. Orbis: Franz/Riza the same; 6. the other client sees it -> fixed'
        Write-Host '      7. COLOUR BOXES (fix 2ab2e4d - your try showed ONE look on Next):' -ForegroundColor White
        Write-Host '         Dr. Feeble, 5152100, Fern Face 22036: Next cycles NINE eye colours' -ForegroundColor Cyan
        Write-Host '         Brittany, 5151100, Fern Hair 42570: Next cycles EIGHT colours, label' -ForegroundColor Cyan
        Write-Host '         changes (Black, Red, ...). Pick -> only the colour changes -> fixed' -ForegroundColor Green
        Write-Host '         still ONE look -> coupon type is not what the client reads; watch' -ForegroundColor Yellow
        Write-Host '         FUN_142a91f30 param_2 next' -ForegroundColor Yellow
        Write-Host '         The colours must look DIFFERENT (real recolours, db45025, via the client' -ForegroundColor Cyan
        Write-Host '         patch). Labels change but pictures do not -> no client patch yet' -ForegroundColor Yellow
        Write-Host '      8. COUPON DEFAULTS: Fern Hair coupon -> Violet 42576, Ubel Hair -> Green' -ForegroundColor White
        Write-Host '         42604, Ubel Face -> Violet 22639 - that colour on screen -> fixed' -ForegroundColor Cyan
        Write-Host '  T6. !npcreload. Add a line to data/npc-dialogue.txt with the' -ForegroundColor White
        Write-Host '      server RUNNING, run the command, click that NPC.'
        Write-Host '        a) new line with no restart?'
        Write-Host '        b) the reply names counts. A reload that read nothing must'
        Write-Host '           say zero, not ok'
        Write-Host ''
        Write-Host '  ---- carried over, unchanged ----' -ForegroundColor DarkGray
        Write-Host '  1. RECOVERY (1001). !learn 1001 3, then cast it.' -ForegroundColor White
        Write-Host '     NO BUFF ICON WILL APPEAR and that is expected - Recovery has no'
        Write-Host '     stat bit anybody has identified. The heal is real, the tray is'
        Write-Host '     empty, and the cast says so in chat.'
        Write-Host '       a) six blue +12 numbers, one every 5s, over 30s?'
        Write-Host '            numbers but wrong spacing -> the 5s interval is DERIVED from'
        Write-Host '                       the tooltip arithmetic, not read from a column.'
        Write-Host '                       Say the spacing you saw'
        Write-Host '            one then nothing -> the tick is not being driven'
        Write-Host '       b) does it stop on its own after six?'
        Write-Host ''
        Write-Host '  2. !resetap. Put points into STR, HP and MP first.' -ForegroundColor White
        Write-Host '       a) STR/DEX/INT/LUK all read 4?'
        Write-Host '       b) did max HP and max MP come DOWN, and the points come back?'
        Write-Host '            stats reset but max HP did not -> the ledger is not recording'
        Write-Host '       c) run it AGAIN. It must refund NOTHING.'
        Write-Host '            more points -> the counters are not cleared, which is free'
        Write-Host '                       stats out of a command meant to be safe to repeat'
        Write-Host ''
        Write-Host '  3. OVERALLS. Wear trousers, then equip a robe (1050000).' -ForegroundColor White
        Write-Host '       a) do the trousers come off into the bag?'
        Write-Host '       b) put them back on - the robe should come off. It is symmetric'
        Write-Host '       c) THE CONTROL: equip a plain TOP (1040000) over trousers. They'
        Write-Host '          must STAY ON. A shirt removing trousers is a worse bug than'
        Write-Host '          the one being fixed'
        Write-Host ''
        Write-Host '  4. THE GM GATE, as tester. Every ! command is gated now.' -ForegroundColor White
        Write-Host '     A refused command is SAID OUT LOUD rather than answered with a'
        Write-Host '     refusal - to an account that cannot run commands, !heal is a'
        Write-Host '     person typing text, so the game says it.'
        Write-Host '       a) !heal as tester -> a CHAT BALLOON reading "!heal", and'
        Write-Host '          nothing happens to your HP'
        Write-Host '       b) it must NOT print a system notice about GM status - that told'
        Write-Host '          a non-GM which ! words are real, which is why it went'
        Write-Host '       c) ordinary chat still works? Type "Hello". A gate one line'
        Write-Host '          higher would have silenced everybody, and the client draws'
        Write-Host '          nothing for its own chat, so a dropped line is invisible'
        Write-Host '       d) then as maplecw: !heal works'
        Write-Host ''
        Write-Host '  5. THE tester ACCOUNT SEES ITS OWN CHARACTERS - it has none.' -ForegroundColor White
        Write-Host '       empty list -> the login claim is being read'
        Write-Host '       Cobalt -> it is not, and you are still maplecw. login.log names'
        Write-Host '                 the account on every connection'
        Write-Host '     Making a character here also exercises the create path on a fresh'
        Write-Host '     account, which nothing has done since the name check went in.'
        Write-Host ''
        Write-Host '  6. KEY BINDINGS - the CONTROLLER tab. Your report: not saved, and' -ForegroundColor White
        Write-Host '     "completely screwed up" after a map change. Measured: the byte after'
        Write-Host '     the subtype in a CONFIRM is the TABLE (0 keyboard, 3 controller) and'
        Write-Host '     the server dropped it - your buttons were stored as scan codes - and'
        Write-Host '     0x05F1 sent table 3 as "keep", which means keep the RESET to keyboard'
        Write-Host '     preset 0. Every SetField handed the controller a keyboard layout.'
        Write-Host '     Now: rows carry the table, 0x05F1 is 1785 bytes, all four tables'
        Write-Host '     READ, the controller one from its own const (22 buttons) + yours;'
        Write-Host '     Wisp#215''s 22 stray rows scrubbed by a migration (verified on a copy).'
        Write-Host '     As the owner, KEY BINDINGS -> Controller:'
        Write-Host '       tab shows controller DEFAULTS, not menus/attack on buttons -> ok'
        Write-Host '       bind a skill to a button, CONFIRM, take a portal, open again:'
        Write-Host '         skill still there, rest default -> DONE' -ForegroundColor Green
        Write-Host '         skill gone -> paste the "keymap:" log line; it names the table' -ForegroundColor Yellow
        Write-Host '         buttons show keyboard actions -> READ not taken; STOP, say so' -ForegroundColor Yellow
        Write-Host '       Keyboard tab: Q/W/E/I menus, Ctrl attack, 1..0 row empty -> clean'
        Write-Host '       client exits at field entry -> paste the 0x009E line' -ForegroundColor Yellow
        Write-Host '     As Cobalt once: Ctrl Power Strike, Shift Slash Blast after a relog'
        Write-Host '     -> the keyboard restore (unconfirmed since 09-12) is confirmed too.'
        Write-Host ''
        Write-Host '  7. SKILL POINTS SURVIVE A MAP CHANGE. seedling: advanced to Bowman at' -ForegroundColor White
        Write-Host '     level 12, got 7 SP, then they vanished. The advance was right; the'
        Write-Host '     stat block in every SetField carried an EMPTY SP table, so the next'
        Write-Host '     portal zeroed the pool. Now the pool packet rides after every'
        Write-Host '     SetField. As purr (id 218, level-12 Bowman, 0 spent), skill window:'
        Write-Host '       Bowman tab shows 7 -> walk a portal, reopen:'
        Write-Host '         still 7 -> DONE' -ForegroundColor Green
        Write-Host '         back to 0 -> after-SetField 0x007C did not apply; paste the log' -ForegroundColor Yellow
        Write-Host '       0 on the FIRST open (no portal yet) -> the login 0x007C is missing' -ForegroundColor Yellow
        Write-Host '     Spend one, change channel: the spend must hold. A beginner (ouggh,'
        Write-Host '     215) must show its OWN computed SP, not a phantom first-job pool.'
        Write-Host ''
        Write-Host '  8. THE PET, SIX THINGS - from your two-client run. The walk is' -ForegroundColor White
        Write-Host '     CONFIRMED. Each item below left a packet and is answered now; none'
        Write-Host '     has been on a screen. As the owner, Husky out, Tester2 watching:'
        Write-Host '       LOOT + VACUUM, REWRITTEN (2026-09-17). The in-range vacuum is FREE:'
        Write-Host '       every pet item is wonderGrade 6 = the wide 0x0198 box AND "Petite Luna".'
        Write-Host '       Auto Move / Expanded Auto Move are PAID skills for WALKING to drops.'
        Write-Host '       Default tooltip = Meso Magnet + Item Pouch (NEEDS backport --install).'
        Write-Host '         a. fresh Husky: tooltip 2 skills + Petite Luna; a drop ~200px away' -ForegroundColor Green
        Write-Host '            FLIES to it, no walk -> DONE; say how far (box is 600x590)' -ForegroundColor Green
        Write-Host '            still lists Expanded/Auto Move "unregistered" -> rebuild the client' -ForegroundColor Yellow
        Write-Host '            "Ignore Item (Learned)" -> an old row the remap missed; paste skills' -ForegroundColor Yellow
        Write-Host '            no suck-up -> paste PetPickupRange + item bytes 61..63' -ForegroundColor Yellow
        Write-Host '         b. buy Auto Move (5190002) then Expanded (5190003) - a chain; each' -ForegroundColor Green
        Write-Host '            lists (Learned) and changes the pet WALKING to drops; say what moves' -ForegroundColor Green
        Write-Host '         c. a skill item RIGHT AFTER LOGIN, pet not summoned or fed: (Learned),' -ForegroundColor Magenta
        Write-Host '            item gone -> fixed (live server: "needs a pet" - the restore sent the'
        Write-Host '            bag serial, the lookup knew only the pet serial). Same message again'
        Write-Host '            -> paste the "pet skill:" line, it prints the serial' -ForegroundColor Yellow
        Write-Host '         d. NO "Closeness has increased (+N)" ON A MAP CHANGE - 2nd fix (09-21).' -ForegroundColor Magenta
        Write-Host '            THE NUMBER IS THE CLOSENESS (Lucy''s is 1, read off the wire). The'
        Write-Host '            client prints the DIFFERENCE between the pet''s cached closeness and'
        Write-Host '            the item''s, on every refresh - no quiet path. A field entry clears'
        Write-Host '            the bag, so the pet was built with no item: 0 -> 1. The item now'
        Write-Host '            goes out BEFORE the summon as well as after.'
        Write-Host '            FEED the pet first so closeness > 1, then change maps:' -ForegroundColor Yellow
        Write-Host '              no line -> DONE' -ForegroundColor Green
        Write-Host '              a line reading the pet''s closeness -> SAY THE NUMBER; next lever' -ForegroundColor Yellow
        Write-Host '              is dropping the post-summon write from the ENTRY batch only' -ForegroundColor Yellow
        Write-Host '              pet sad/droopy or no vacuum -> the opposite regression, say so' -ForegroundColor Yellow
        Write-Host '       NO "NEW" MARK ON THE PET: mode 5 on the re-send. No highlighted Cash cell' -ForegroundColor Yellow
        Write-Host '         on login -> struck; still highlighted -> paste the Cash-tab 0x0070 lines.' -ForegroundColor Yellow
        Write-Host '       PET VACUUM AT LOGIN/MAP CHANGE: CONFIRMED ("Pets now work on initial summon").' -ForegroundColor DarkGray
        Write-Host '       COLLAB PETS + EVERY PET EQUIP (2026-09-17, installed; rebuild the client'
        Write-Host '         package). Pets tab: Lil Frieren/Fern/Stark/Ubel at 1000; Pet Equip:' -ForegroundColor Yellow
        Write-Host '         all 10 hats + 4 weapons at 100; ALL 15 pets'' icons carry the purple P' -ForegroundColor Yellow
        Write-Host '         badge (shop AND bag, NOT on the walking pet) and NO "N day(s)" line' -ForegroundColor Yellow
        Write-Host '         -> fixed. Installed 2026-09-18 02:10 with the client closed (983e79f).'
        Write-Host '         A classic pet summoned INVISIBLE -> life 0 was it after all; paste.' -ForegroundColor Yellow
        Write-Host '         Hover Lil Fern in the Pets tab: "Skill: Meso Magnet, Item Pouch" and' -ForegroundColor Yellow
        Write-Host '         NOTHING more -> fixed (2026-09-18; Nexon declared Auto Move + Auto' -ForegroundColor Yellow
        Write-Host '         Buff, both stripped). Four skills -> old archive; "Auto Buff" -> paste.' -ForegroundColor Yellow
        Write-Host '         Buy + summon Lil Frieren: it' -ForegroundColor Yellow
        Write-Host '         draws, tooltip lists 7 commands, answers "roll"/"angry"/"sleep"/"talk"' -ForegroundColor Yellow
        Write-Host '         with the right animation -> DONE; dies/invisible -> paste the item line.' -ForegroundColor Yellow
        Write-Host '         Equip its Staff (Deco tab) on it: draws on the pet -> the inline worked.' -ForegroundColor Yellow
        Write-Host '       SIGNATURE STYLE PRICES (2026-09-17): 200 LP a set coupon, 800 the box'
        Write-Host '         (was 2000/8000). Needs backport_install.py --install (client closed):' -ForegroundColor Yellow
        Write-Host '         it writes the client WZ AND regenerates commodity.txt. Special tab' -ForegroundColor Yellow
        Write-Host '         shows 800/200 and a buy debits 200 -> fixed; still 2000 -> not installed' -ForegroundColor Yellow
        Write-Host '       LEAF POINT COUPONS (2026-09-17): !item 2430004, double-click it in the'
        Write-Host '         Use tab: "You received 1,000 Leaf Points", coupon gone, Cash Shop' -ForegroundColor Yellow
        Write-Host '         balance up -> fixed. Nothing happens -> paste the "<- 0x" line the' -ForegroundColor Yellow
        Write-Host '         click sent (the opcode is read off the listing, never on a wire).' -ForegroundColor Yellow
        Write-Host '       NPC SHOP DUPLICATES (2026-09-16): the 10x-cheaper twin was our Sell row;'
        Write-Host '         gone. Any shop: each item ONCE at its real price -> fixed. Sell a Red' -ForegroundColor Yellow
        Write-Host '         Potion from the right panel: still works. Panel MISSING or sale refused' -ForegroundColor Yellow
        Write-Host '         -> paste the 0x055D and 0x00F5/0x055E lines (the sell byte mattered).' -ForegroundColor Yellow
        Write-Host '       TWO HUSKIES ARE TWO PETS (2026-09-16): every pet item carries its own'
        Write-Host '         id; a Name Tag names ONE of them. Buy a second Husky, summon it, name'
        Write-Host '         it Dummy: the first stays Husky; relog: Dummy is the one out -> fixed' -ForegroundColor Yellow
        Write-Host '         same name on both / wrong one back -> paste the "re-sent as pet #N" lines' -ForegroundColor Yellow
        Write-Host '       FULL TAB (2026-09-16): our yellow chat line is GONE. A refused pick-up'
        Write-Host '         (yours or the pet''s) is the CLIENT''s "You can''t get anymore items."'
        Write-Host '         in the message area where EXP draws, at most once per 2 s.'
        Write-Host '         pet over it / click it: that line, chat log EMPTY, drop stays -> fixed' -ForegroundColor Yellow
        Write-Host '         yellow or in the chat log -> old build; no line -> paste the 0x0089' -ForegroundColor Yellow
        Write-Host '       SKILLS: Auto HP. Your 23:34 try KILLED THE CLIENT: the put-away half'
        Write-Host '         of the re-summon lacked the reason byte the OWNER reads. Fixed.'
        Write-Host '         Expect ONE summon animation. Client exits there -> paste 0x009E' -ForegroundColor Yellow
        Write-Host '         tooltip (Learned) + item gone -> stored. Take damage: potion fed?'
        Write-Host '         YES -> DONE. NO with (Learned) -> client wants more; say so' -ForegroundColor Yellow
        Write-Host '       NAME TAG: rename to Dummy:'
        Write-Host '         Dummy over the pet on BOTH screens -> DONE' -ForegroundColor Green
        Write-Host '         owner only -> the map 0x027B is dropped; say so' -ForegroundColor Yellow
        Write-Host '       HAT: Blue Top Hat on the pet with Tester2 present:'
        Write-Host '         Tester2 sees it at once -> DONE' -ForegroundColor Green
        Write-Host '         Tester2 sees TWO copies of the owner''s character / a frozen one -> STOP, say so' -ForegroundColor Red
        Write-Host '         nothing until a map change -> redraw ignores the look' -ForegroundColor Yellow
        Write-Host '       FEEDING: Pet Food (Lucy, 35 mesos) on the Husky, Show Pet Info open:'
        Write-Host '         Fullness +30, Closeness +1, Level 2 -> DONE (0x0112, never captured)' -ForegroundColor Green
        Write-Host '         nothing changes -> paste the "pet food:" line, or the 0x0112 line' -ForegroundColor Yellow
        Write-Host '         the Husky EATS on BOTH screens (0x027E) -> DONE; one screen only -> say which' -ForegroundColor Green
        Write-Host '         and NO "Yum, yum! ... left!" balloon (food id 0; that is the auto-feed' -ForegroundColor Yellow
        Write-Host '         message, and it counted one short) -> DONE; balloon -> paste the 0x027E line' -ForegroundColor Yellow
        Write-Host '         level 1 -> 2 on the first feed: a LEVEL-UP flash on BOTH screens' -ForegroundColor Green
        Write-Host '         3rd feed at 100 costs a closeness.'
        Write-Host '       HUNGER: -1 Fullness per five minutes out; at 0 it goes home.'
        Write-Host '       TRICKS: a "sit" that lands -> Closeness +3, the level climbs.'
        Write-Host '       RE-LOGIN: log out with the Husky out, log back in:'
        Write-Host '         standing beside you on arrival -> DONE' -ForegroundColor Green
        Write-Host '         in the bag -> paste the "pet: character" claim-time line' -ForegroundColor Yellow
        Write-Host '       SHOW PET INFO: worked on the second look. Grey again? say whether the' -ForegroundColor DarkGray
        Write-Host '         window was opened BEFORE the summon - the only measurement left' -ForegroundColor DarkGray
        Write-Host ''
        Write-Host '  9. THE LAUNCHER UPDATES ITSELF. Server package ships bin\maplecw-launcher' -ForegroundColor White
        Write-Host '     .exe, auth publishes it, the launcher swaps itself at Start Game. ONE'
        Write-Host '     LAST MANUAL INSTALL of MapleCW-setup.zip over D:\MapleCW (the 00:29 one'
        Write-Host '     cannot self-update). Then Start Game:'
        Write-Host '       "launcher version ... confirmed" -> DONE' -ForegroundColor Green
        Write-Host '       "updating it", window closes, NEW window opens, "previous ... removed"' -ForegroundColor Green
        Write-Host '         in its log, no .old left -> DONE' -ForegroundColor Green
        Write-Host '       "publishes no launcher" -> the server is an older package' -ForegroundColor Yellow
        Write-Host '       closes and nothing opens -> say what the folder holds' -ForegroundColor Yellow
        Write-Host '       updates EVERY time -> two builds of one source in play; say so' -ForegroundColor Yellow
        Write-Host '     SIGN OUT: Login, then Sign out. Start Game greys AT ONCE and the status'
        Write-Host '       says "1 claim(s) revoked on the server" -> DONE' -ForegroundColor Green
        Write-Host '       "predates sign-out" -> deploy the matching server zip' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  10. BUFF ICONS FROM ITEMS (2026-09-16). Potions named their item id' -ForegroundColor White
        Write-Host '     POSITIVE (= a skill id, nothing to draw); the EXP coupon sent no'
        Write-Host '     stat at all. Now -itemId, and the coupon rides CTS 163 ExpBuffRate.'
        Write-Host '     The sign convention is [D]: this run is the reading.' -ForegroundColor Yellow
        Write-Host '     a. Magic Potion 2002001: icon top-right, potion picture, 10:00 -> DONE' -ForegroundColor Green
        Write-Host '        no icon, M.Att still +10 -> negative reason ignored; SAY SO' -ForegroundColor Yellow
        Write-Host '     b. 3x EXP Coupon 2450001: a second icon, 15:00, + yellow line -> DONE' -ForegroundColor Green
        Write-Host '        potion icon yes, coupon icon no -> 163 is not it; kill something,' -ForegroundColor Yellow
        Write-Host '        triple EXP still? report both halves separately' -ForegroundColor Yellow
        Write-Host '     c. wait one out / right-click it: icon AND stat go together' -ForegroundColor Green
        Write-Host ''
        Write-Host '  38. SP ON LEVEL-UP (2026-10-02): skill window open, level up - SP rises at once,' -ForegroundColor White
        Write-Host '       + works with no map change / channel change' -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  50. JUMP QUESTS (2026-10-04, NEVER ON A SCREEN): every step logs a "jump quest:" line' -ForegroundColor White
        Write-Host '       a. Shane (Ellinia, top): stranger to quest 10509 -> his refusal; with 10509 -> free menu, step 1 or step 3' -ForegroundColor Cyan
        Write-Host '       b. Louis: Yes -> Ellinia beside Shane   c. pile of flowers (top of step 2): box lists a Use + a Scroll' -ForegroundColor Cyan
        Write-Host '          (+ Pink Anthurium on 10509); OK -> items + chat lines, back beside Shane; from >250 px: "Go a little closer"' -ForegroundColor Cyan
        Write-Host '       d. Sleepywood statue: 3 flowers -> Deep Forest step 1/3/5; Crumbling Statue -> back; 10006 -> 10 Pink Violas' -ForegroundColor Cyan
        Write-Host '       e. Booth: Jake sells B1/B2/B3 (Lv 20/30/40, 500/1200/2000); the gate takes one -> Area 1; Exit -> booth' -ForegroundColor Cyan
        Write-Host '       f. B1 press-up portals within the map move you -> fine; a hang -> paste the last world-ch0.log lines' -ForegroundColor Yellow
        Write-Host '       g. B1 depot chest: a Use + a Scroll (+ Shumi''s Coin on 10312); back at the booth   h. drops page: Jump Quest Reward (JQ)' -ForegroundColor Cyan
        Write-Host '       i. PITY TIMER (quest entries only): 60 min on the course -> yellow reminder, every 5 min after;' -ForegroundColor Cyan
        Write-Host '          !skipjq before -> "N more minutes"; after -> quest item only, back in town; a relog keeps the time' -ForegroundColor Cyan
        Write-Host '          next step of the same course keeps counting; leaving by the warden or a scroll stops it (back in -> 60 again)' -ForegroundColor Cyan
        Write-Host '       no box / nothing given / no warp -> paste the "jump quest:" lines' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  49. PLAYER STORE (2026-10-04): two clients, owner has a Store Permit (5140001) in the Cash tab' -ForegroundColor White
        Write-Host '       a. use the permit: window opens + sign over the head   b. list an item: it leaves the bag, a row appears' -ForegroundColor Cyan
        Write-Host '       c. Open Store: sign says can enter   d. visitor double-clicks the sign: their window opens' -ForegroundColor Cyan
        Write-Host '       e. visitor buys: item + mesos move, owner gets the price less 3%   f. chat shows both sides' -ForegroundColor Cyan
        Write-Host '       g. Close Store: visitor told "The shop has been closed.", unsold items back, sign gone' -ForegroundColor Cyan
        Write-Host '       (first run: a-f worked; owner UI froze after Open Store - FIXED: owner can still click everything)' -ForegroundColor Green
        Write-Host '       g3. a line bought out reads SOLD OUT with its price; the last one sold closes the store itself' -ForegroundColor Cyan
        Write-Host '       g4. a store or elf right beside another -> "You can''t open a store here"; 120 px away -> it opens' -ForegroundColor Cyan
        Write-Host '       h. Free Market: double-click the Hired Merchant in the Cash tab -> title prompt, window, the elf appears' -ForegroundColor Cyan
        Write-Host '       i. list, Open Store, close the window, log out -> the elf STAYS; a visitor can buy from it' -ForegroundColor Cyan
        Write-Host '       j. owner double-clicks own elf -> maintenance; Close Store -> elf gone, items home   k. gone after 24 h' -ForegroundColor Cyan
        Write-Host '       anything fails or a button goes dead -> paste the 0x017F / 0x0180 / 0x0181 lines and what followed' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  48. LIL UBEL NAME TAG (2026-10-04): summon Lil Ubel - the tag reads "Lil Ubel" with a plain U -> fixed' -ForegroundColor White
        Write-Host '       still a box -> paste the 0x0277 line from world-ch0.log' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  47. PET EQUIPS (2026-10-04): run tools\dump_petequips.py first.' -ForegroundColor White
        Write-Host '       Pet Equip scroll (!item 2048000 1) on the WORN pet hat: tooltip updates AT ONCE -> fixed' -ForegroundColor Green
        Write-Host '       only after a map change -> paste the "re-sending" line (type 6 -114 expected)' -ForegroundColor Yellow
        Write-Host '       same on a pet hat in the Deco tab: that hat changes at once' -ForegroundColor Cyan
        Write-Host '       Tester2: Show Pet Info on Lil Frieren in a Blue Top Hat -> hat cell EMPTY; on a Husky it shows' -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  46. OTHER PLAYER CITIZENSHIP (2026-10-04): Tester2 double-clicks the owner (a citizen).' -ForegroundColor White
        Write-Host '       CITIZENSHIP lit; town, grade, contribution as on the owner own window -> fixed' -ForegroundColor Green
        Write-Host '       greyed -> paste the "character info:" line; wrong town / blank -> screenshot both' -ForegroundColor Yellow
        Write-Host '       Tester2 (never signed): the owner sees it greyed, as Tester2 does' -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  45. OMOK + MATCH CARDS (2026-10-04, NEVER ON A SCREEN): both !item 4080000 1, same map.' -ForegroundColor White
        Write-Host '       a. owner opens "hello": window opens + balloon over the head on BOTH screens' -ForegroundColor Cyan
        Write-Host '          + a [Miniroom] chat line -> fixed. No window -> "omok:" lines in world-ch0.log' -ForegroundColor Green
        Write-Host '       b. Tester2 clicks the balloon: both seated in both windows, balloon 2/2' -ForegroundColor Cyan
        Write-Host '       c. Ready, Start: owner moves first; 5 in a row -> You win / You lost' -ForegroundColor Cyan
        Write-Host '       d. 2nd game: loser first. Take-back, tie, give up, expel' -ForegroundColor Cyan
        Write-Host '       e. owner closes: Tester2 gets "The room is closed.", balloon gone' -ForegroundColor Cyan
        Write-Host '       f. password room: wrong pw -> "password is incorrect"; late arrival sees balloon' -ForegroundColor Cyan
        Write-Host '       g. RECORD (W/L/D + PTS), per game: winner W 1 PTS 2010, loser L 1 PTS 1990,' -ForegroundColor Cyan
        Write-Host '          both screens, kept into the next room -> fixed. Game must END cleanly' -ForegroundColor Green
        Write-Host '       i. TIME-OUT: let the clock run out - Omok: a stone is placed for you, the other' -ForegroundColor Cyan
        Write-Host '          player moves next on both screens and their stone is accepted -> fixed' -ForegroundColor Green
        Write-Host '       h. MATCH CARDS (!item 4080100 1): a-e again; miss = both turn down + turn' -ForegroundColor Cyan
        Write-Host '          passes, pair = stays up + go again; its record is separate' -ForegroundColor Cyan
        Write-Host '       client dies at any step -> paste client-exit.log and say which step' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  44. MIX DYE / COLORBLEND (2026-10-03): !item 5151200 1, !item 5152300 1.' -ForegroundColor White
        Write-Host '       Brittany, "mix two hair colours": the colours are clickable now (first run: all dead,' -ForegroundColor Cyan
        Write-Host '       starting ratio 0); the preview recolours and a 50/50 slider shows. CANCEL: no' -ForegroundColor Cyan
        Write-Host '       "same color is already equipped" box (first run: every Cancel). Then pick, OK:' -ForegroundColor Cyan
        Write-Host '       BOTH colours drawn, coupon gone, still drawn after map change + relog -> fixed' -ForegroundColor Green
        Write-Host '       blank head / client dies on the HAIR bit -> paste client-exit.log' -ForegroundColor Yellow
        Write-Host '       Dr. Feeble, "blend two eye colours": the same for the eyes' -ForegroundColor Cyan
        Write-Host '       HOVER (hook patch): rest on a swatch - the name fades in ONCE and stays -> fixed;' -ForegroundColor Cyan
        Write-Host '       still flickers, or shows then vanishes -> paste MIXTOOLTIP lines from the hook log' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  43. TRADE, 3RD PASS (2026-10-03): the Trade button completes it now.' -ForegroundColor White
        Write-Host '       Puts LEAVE the bag/wallet; a cancel gives them back.' -ForegroundColor Cyan
        Write-Host '       a. invite: red "You have sent a trade request to ..." -> fixed. Target in an' -ForegroundColor Green
        Write-Host '          NPC shop/storage/dialogue: no popup, "... is doing something else" -> fixed' -ForegroundColor Green
        Write-Host '       b. drag a stack in: it leaves the bag; drag a SECOND item: it goes in -> fixed' -ForegroundColor Green
        Write-Host '       c. 3000 mesos: wallet drops at once -> fixed. Then 1000: window shows 1000' -ForegroundColor Green
        Write-Host '          (a total, as assumed); if it meant 1000 more, say so' -ForegroundColor Cyan
        Write-Host '       d. trade chat: your line in BOTH windows, two colours -> fixed' -ForegroundColor Green
        Write-Host '          missing -> paste the mode 8 lines' -ForegroundColor Yellow
        Write-Host '       e. close with things on both sides: other window closes "cancelled by the' -ForegroundColor Green
        Write-Host '          other character", BOTH get items + mesos back -> fixed' -ForegroundColor Green
        Write-Host '       f. Trade on ONE side: the other window shows them ready -> fixed' -ForegroundColor Green
        Write-Host '       g. Trade on both: "Trade successful.", items cross, mesos arrive LESS 5%' -ForegroundColor Green
        Write-Host '          (2000 -> 1900) -> fixed. "problem trading the item" -> paste 0x10/2 + 0x10/5' -ForegroundColor Yellow
        Write-Host '       h. A presses Trade, B tries to put more in: refused in red, nothing leaves' -ForegroundColor Green
        Write-Host '          B''s bag, B can still drag after -> fixed. B presses: done as A accepted it' -ForegroundColor Green
        Write-Host '       i. decline -> inviter sees "has denied the invitation"; accept after the' -ForegroundColor Green
        Write-Host '          inviter left -> "room is already closed"; untradeable item -> a dialog' -ForegroundColor Green
        Write-Host '       j. COOLDOWN: invite twice while unanswered, or again within 1 min of a' -ForegroundColor Green
        Write-Host '          decline -> "Please invite later.", no popup -> fixed' -ForegroundColor Green
        Write-Host ''
        Write-Host '  42. DISORDER DEBUFF (2026-10-02, FIRST 0x03E6 EVER): hit a mob with Disorder,' -ForegroundColor White
        Write-Host '       look ABOVE it: debuff icon for 10-30 s -> fixed' -ForegroundColor Cyan
        Write-Host '       no icon -> wrong status index; client dies -> wrong layout: paste' -ForegroundColor Yellow
        Write-Host '       client-exit.log + the "MobStatSet 0x03E6" line from world-ch0.log' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  41. STARS BY THE SET (2026-10-02): buy Subi once at a Grocer: yes/no (no qty box),' -ForegroundColor White
        Write-Host '       +500 Subi for 500 mesos -> fixed. 1 star or 3 sets charged -> paste the line' -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  40. DAMAGE GUARD (2026-10-02): play every class, crits and skills included.' -ForegroundColor White
        Write-Host '       Then damage-suspects.log (beside world-ch0.log) empty -> ceiling holds -> fixed' -ForegroundColor Cyan
        Write-Host '       a line names YOUR character -> paste it; restart -DamageGuardLogOnly' -ForegroundColor Yellow
        Write-Host '       use Lucky Seven, Avenger, Shadow Meso, Arrow Bomb, Power Knockback once each' -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  39. EMPTY STARS (2026-10-02): !item 2070000 3, throw until out: the stack STAYS' -ForegroundColor White
        Write-Host '       at 0 -> fixed. Blank slot or client death on the 0 -> paste client-exit.log' -ForegroundColor Yellow
        Write-Host '       Recharge it at a Grocer ("Recharge: 150", 500); drop + pick up (back at 0, no' -ForegroundColor Cyan
        Write-Host '       "x1"); relog (still 0, not 1) -> fixed' -ForegroundColor Cyan
        Write-Host '   39f. RECHARGE BUTTON (2026-10-03, never drawn before): hold 37 Subi + 2 Wolbi,' -ForegroundColor White
        Write-Host '       Grocer, Sell tab: a Recharge button on BOTH, "Recharge: 139" and' -ForegroundColor White
        Write-Host '       "Recharge: 200"; press each: full, mesos down by the label -> fixed' -ForegroundColor Cyan
        Write-Host '       Wolbi only -> Subi''s recharge row not found first; neither -> paste the' -ForegroundColor Yellow
        Write-Host '       ClassicOpenShop line; pressed, no refill -> paste the "recharge:" line' -ForegroundColor Yellow
        Write-Host '       Then buy Subi: yes/no, +500 for 500. At Max (Kerning Civic Center), if' -ForegroundColor Cyan
        Write-Host '       you hold the grade, buy Unagi: arrives -> fixed; "client asked for" -> paste' -ForegroundColor Yellow
        Write-Host '   39g. ONE STACK PER SLOT (2026-10-03): with a partial AND an empty Wolbi, pick up' -ForegroundColor White
        Write-Host '       Wolbi and buy a set: each in a NEW slot, old stacks unchanged. Recharge the' -ForegroundColor Cyan
        Write-Host '       higher-slot partial: that one fills, not the lower -> fixed' -ForegroundColor Cyan
        Write-Host '   39h. STAR = ONE ITEM (2026-10-03): two Subi stacks (37, 120). Drag one onto the' -ForegroundColor White
        Write-Host '       other: SWAP, counts kept; Consolidate/Sort: still two; drop the 120: slot' -ForegroundColor Cyan
        Write-Host '       empties (not 119), pick up: 120 in a new slot; store + withdraw: same -> fixed' -ForegroundColor Cyan
        Write-Host '       a merge into 157, or 119 left behind -> the old server; paste the line' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  38. FAST SELLING (2026-10-02): sell the same row twice, fast: sells ONCE, no' -ForegroundColor White
        Write-Host '       "not enough mesos", the row leaves the Sell list -> fixed' -ForegroundColor Cyan
        Write-Host '       row still listed -> paste the "a STALE sell" line from world-ch0.log' -ForegroundColor Yellow
        Write-Host '       sell 1 of 3 potions: the slot shows 2 left, not empty -> fixed' -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  37. OPTIONS ON THE SERVER (2026-10-02): set HP warning % + effect volume; relog,' -ForegroundColor White
        Write-Host '       change channel: both kept. OTHER character on the account: same volume,' -ForegroundColor Cyan
        Write-Host '       but its OWN HP/MP warning %, pet potions and key layout (per character)' -ForegroundColor Cyan
        Write-Host '       - set a different % there, back to the first: the first''s % is intact' -ForegroundColor Cyan
        Write-Host '       login fails after SetField -> restart with -NoSystemOptions; works then =' -ForegroundColor Yellow
        Write-Host '       block #32 placement wrong (HP warning should still be kept)' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  36. FIRST-JOB SP PAST 30 (2026-10-02): 1st-job tab keeps 3/level past 30 to 106 (Thief 112)' -ForegroundColor White
        Write-Host '       SP Reset Scroll gives the whole grown total back' -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  35. DEATH WITH A PET + TWO CLIENTS (2026-10-02) - Auto HP pet on, die near the other client' -ForegroundColor White
        Write-Host '       a. dialog opens, pet drinks NOTHING, HP stays 0 (a potion = refusal did not fire)' -ForegroundColor Cyan
        Write-Host '       b. OTHER client: ghost / tombstone on the dead one? YES = stance is enough;' -ForegroundColor Cyan
        Write-Host '          NO = an observer needs a packet not yet found - say what you saw' -ForegroundColor Yellow
        Write-Host '       c. REVIVE IN TOWN -> town, 50 HP; the character saved on map 0 logs in to Henesys' -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  34. DROP PAGE MAPS (2026-10-01): no Ludibrium monsters; Slime "Found on" lists its maps, most spawns first -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  33. GLOBAL SCROLLS + CHAOS + TOOLTIPS (2026-10-01) - redeploy incl. gm-handbook\itemdesc.txt' -ForegroundColor White
        Write-Host '       !scroll with no Secrets/Treasure: lists the 4 scrolls monsters drop now -> DONE' -ForegroundColor Green
        Write-Host '       a successful Chaos always moves a stat 1-5 points, never 0 -> DONE' -ForegroundColor Green
        Write-Host '       drop page: hover an item name, the game-style tooltip appears -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  32. DROP PAGE (2026-10-01): http://<server>:8481/ - forward 8481 for players' -ForegroundColor White
        Write-Host '       the rate box starts at the server rate, marked (server) -> DONE' -ForegroundColor Green
        Write-Host '       10 Snail kills show up within ~35 min (5 to write + 30 cache) -> DONE' -ForegroundColor Green
        Write-Host '          nothing after an hour -> paste the "killstats:" lines from world-ch0.log' -ForegroundColor Yellow
        Write-Host '       "shoes greater scroll" finds Shoes Jump Scroll: Greater -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  31. V83 DROPS (2026-10-01), live is fine:' -ForegroundColor White
        Write-Host '       Snails: an equip ~1 kill in 100-200 at 1x (1 in 20-40 at 5x), no scroll -> DONE' -ForegroundColor Green
        Write-Host '          an equip every few kills -> the old data\drops.txt is still deployed' -ForegroundColor Yellow
        Write-Host '       Mano at 5x: Wand Magic Attack scroll about 1 kill in 67 -> DONE' -ForegroundColor Green
        Write-Host '       Scroll of Secrets / Treasure Scroll: ~1 in 200 kills each at ANY rate -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  30. BEGINNER RECOVERY (2026-10-01), a hurt Beginner:' -ForegroundColor White
        Write-Host '       cast: Recovery icon counting down 30 s, no chat line -> DONE' -ForegroundColor Green
        Write-Host '          no icon -> bit 131 is not its bit; say so' -ForegroundColor Yellow
        Write-Host '       blue +4 every 5 s, six times; icon goes with the last -> DONE' -ForegroundColor Green
        Write-Host '       right-click the icon: it goes and the heals stop -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  29. BUFFS ACROSS CHANNELS + RESPAWN WAVES (2026-10-01), live is fine:' -ForegroundColor White
        Write-Host '       timed buff on, change channel: icon there with the time left;' -ForegroundColor Green
        Write-Host '       at 0 the icon AND the stat go -> DONE. Magic Guard survives too -> DONE' -ForegroundColor Green
        Write-Host '          icon stuck at 0 -> paste the "buffs:" lines from world-ch0.log AND world-ch1.log' -ForegroundColor Yellow
        Write-Host '       kill 3 mobs a few seconds apart: all 3 back TOGETHER within 8 s -> DONE' -ForegroundColor Green
        Write-Host '          each back 8 s after its own kill -> old build' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  31. QUEST LINES ONCE (2026-10-02): turn in any quest at its NPC ->' -ForegroundColor White
        Write-Host '       completion lines ONCE, then at most one thank-you box -> DONE' -ForegroundColor Green
        Write-Host '       a weekly donation asks "...donate?" ONCE; Yes -> thanks, client stays up -> DONE' -ForegroundColor Green
        Write-Host '          the same lines twice = old build' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  30. BACKPORTED SCROLLS + LUCKY DAY (2026-10-01) - client data changed, launch from client-patched:' -ForegroundColor White
        Write-Host '       !item 2049100 / 2049003 / 2049190 / 2530000 (Chaos, Clean Slate 20%, Innocence, Lucky Day)' -ForegroundColor Green
        Write-Host '       modern icons + names; Lucky Day says "...next scroll by 100%" -> DONE' -ForegroundColor Green
        Write-Host '       DRAG Chaos onto worn gear: confirm box, animation, ~60% -> DONE' -ForegroundColor Green
        Write-Host '          nothing after Yes -> grep the log for 0x0125' -ForegroundColor Yellow
        Write-Host '       DRAG Lucky Day onto gear: confirm, success animation; tooltip shows anything? SAY WHICH' -ForegroundColor Green
        Write-Host '       then a 10% scroll on it SUCCEEDS, three times running -> DONE' -ForegroundColor Green
        Write-Host '          the drag does nothing -> grep the log for 0x0126' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  29. OVERALL + SHOP TABS (2026-10-01):' -ForegroundColor White
        Write-Host '       top + bottom worn, equip an overall: BOTH drop into Equip at once, no map change -> DONE' -ForegroundColor Green
        Write-Host '       overall worn, equip a bottom: the overall drops into the bag at once -> DONE' -ForegroundColor Green
        Write-Host '       shop: buy a Use item, click Etc, sell an Etc item: stays on Etc -> DONE' -ForegroundColor Green
        Write-Host '       and the sold item leaves the list -> DONE' -ForegroundColor Green
        Write-Host '          stays on Etc but the sold item is STILL LISTED -> say so; that decides the next fix' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  28. BOARD + !SCROLL (2026-10-01), a Henesys citizen; live is fine:' -ForegroundColor White
        Write-Host '       In Progress shows at most ONE Donating to Henesys; extras gone after login -> DONE' -ForegroundColor Green
        Write-Host '       the donation is one your LEVEL can turn in (Lv 17-21 at grade 3: the Lv 17 one) -> DONE' -ForegroundColor Green
        Write-Host '       daily turned in, then !citizenship 1 grade 5: NO new daily until tomorrow -> DONE' -ForegroundColor Green
        Write-Host '          a new one appears -> paste the "board 510001" / "board 510002" lines' -ForegroundColor Yellow
        Write-Host '       !scroll -> Treasure -> item: Scroll of Secrets as Chaos / Clean Slate listed;' -ForegroundColor Green
        Write-Host '       Chaos through it always works and takes one of each -> DONE' -ForegroundColor Green
        Write-Host '       any !scroll result with scrolls left: "keep scrolling?" Yes = menu again -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  27. WEATHER ITEMS (2026-09-30): Sprinkled Chocolate etc. with a message, two clients on one map' -ForegroundColor White
        Write-Host '       both see the effect and message, one spent, fades after ~30 s -> DONE' -ForegroundColor Green
        Write-Host '       a second while one runs: refused, kept; a late arrival sees the rest -> DONE' -ForegroundColor Green
        Write-Host '       GM Blessings 2023000/2023001: GM weather + name on both screens, BOTH get a 60:00 icon;' -ForegroundColor Green
        Write-Host '       Wind = faster + higher jump, Precision = accuracy -> DONE' -ForegroundColor Green
        Write-Host '          icon but no speed/jump = say so (bits 92/93 are the [D])' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  26. MEGAPHONES (2026-09-30): two clients on different channels, level 10+.' -ForegroundColor White
        Write-Host '       Super Megaphone -> both see it, pink, whisper icon as ticked, one spent -> DONE' -ForegroundColor Green
        Write-Host '       Megaphone -> only your channel, not pink, whisper icon as ticked -> DONE' -ForegroundColor Green
        Write-Host '          pink or an item box on the Megaphone = say which (type 8 is the [D])' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  25. SHIP TO ORBIS (2026-09-26): Ellinia Station. Ships leave every :x0 (UTC wall clock),' -ForegroundColor White
        Write-Host '       boarding :x5 to :x9. Joel: "Hi there!" + Next, then a MENU, Basic 5,000 /' -ForegroundColor Green
        Write-Host '       Regular 20,000, grey lines -> DONE' -ForegroundColor Green
        Write-Host '       Cherry + Basic outside the window: refused, names the next ship, ticket kept -> DONE' -ForegroundColor Green
        Write-Host '       inside: "Do you still wish to board?" No = stays; Yes = Before Takeoff, countdown;' -ForegroundColor Green
        Write-Host '       departure -> To Orbis ~5:00;' -ForegroundColor Green
        Write-Host '       cabin and back keeps the clock; 0:00 -> Orbis Ticketing Booth -> DONE' -ForegroundColor Green
        Write-Host '       two clients same departure see each other; next departure sees neither -> DONE' -ForegroundColor Green
        Write-Host '       Regular (NEW 09-29): waiting room ALONE 0:10, then deck alone 1:00 -> Orbis,' -ForegroundColor Green
        Write-Host '       both directions -> DONE; straight on deck = old build' -ForegroundColor Green
        Write-Host '       Purin (waiting room): No = stays; Yes = Ellinia Station, ticket not returned -> DONE' -ForegroundColor Green
        Write-Host '       disconnect or change channel anywhere aboard, log in -> Ellinia Station -> DONE' -ForegroundColor Green
        Write-Host '       THE SHIP (new): enter the station :x5-:x0 -> whistle, ship slides IN from the right;' -ForegroundColor Green
        Write-Host '       other times -> slides OUT; standing there it comes in at :x5, leaves at :x0 -> DONE' -ForegroundColor Green
        Write-Host '          no ship = paste the ContiState/ContiMove lines; jumps not slides = say so;' -ForegroundColor Yellow
        Write-Host '          a crash entering the station = say so FIRST' -ForegroundColor Yellow
        Write-Host '       BALROGS: half the Basic rides, a minute in, on deck: their ship + two Crimson Balrogs -> DONE' -ForegroundColor Green
        Write-Host '       THE WAY BACK: Agatha sells Tickets to Ellinia; Platform Usher -> tunnel -> Rini;' -ForegroundColor Green
        Write-Host '       same ride, lands in Ellinia Station; the Usher has NO ferry line (09-29) -> DONE' -ForegroundColor Green
        Write-Host '          no countdown -> paste the "FieldClock type 2" line; a shop window = old build' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  26. EMOTES (2026-09-29), two clients on one map: A presses Queasy / F1-F7 ->' -ForegroundColor White
        Write-Host '       B sees A make the face -> DONE' -ForegroundColor Green
        Write-Host '          nothing on B -> grep its log for "UserEmotion 0x02A6"' -ForegroundColor Yellow
        Write-Host '       SHADOW STYLE: double-click ONCE (twice = off again), WALK -> afterimages on A and B;' -ForegroundColor Green
        Write-Host '       B leaves and returns -> still on; double-click again -> off on both -> DONE' -ForegroundColor Green
        Write-Host '          on A only -> grep B''s log for "UserEffectItem 0x02A8"' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  25. CITIZENSHIP (2026-09-28), a GM character Lv 12+: Arthur (Henesys Town Hall) ->' -ForegroundColor White
        Write-Host '       the Oath CONTRACT window; Sign -> STAMP, closes ~2 s later, "citizen of Henesys" -> DONE' -ForegroundColor Green
        Write-Host '          window stuck with grey buttons = old build (fixed 09-29)' -ForegroundColor Yellow
        Write-Host '          a plain line = old build / under Lv 12; a CRASH on opening = say so FIRST' -ForegroundColor Yellow
        Write-Host '       Community Board: ONE First Greeting + ONE donation available, nothing else -> DONE' -ForegroundColor Green
        Write-Host '          all or none available -> paste the "quest 510001" log line' -ForegroundColor Yellow
        Write-Host '       "!citizenship 1 contr 950", do the resident: +1000 Contribution at 10x, grade-up effect;' -ForegroundColor Green
        Write-Host '       Arthur then shows the Grade Update certificate (Visitor), next time a menu -> DONE' -ForegroundColor Green
        Write-Host '       Flint: Town Resident rows LOCKED; "!citizenship 1 grade 5", reopen -> they sell -> DONE' -ForegroundColor Green
        Write-Host '       Raymond/Max: EVERY row locked for a non-citizen (Fried Chicken is Traveler+) -> DONE' -ForegroundColor Green
        Write-Host '       HONOR (09-29): "!citizenship 1 grade 9", "... contr 9950", do the resident quest ->' -ForegroundColor Green
        Write-Host '       Henesys Earrings in Equip + blue [Notice] congratulation on EVERY channel -> DONE' -ForegroundColor Green
        Write-Host '          not on the other channel -> paste its "BroadcastMsg type 0" line' -ForegroundColor Yellow
        Write-Host '       Roxy (Kerning Civic Center): Transfer window -> OK; "!citizenship" shows st1=2 -> DONE' -ForegroundColor Green
        Write-Host '       QUEST RATE: Contribution and mesos at the !rates Quest multiplier; EVERY quest now' -ForegroundColor Green
        Write-Host '       pays its mesos (+n) on turn-in -> DONE' -ForegroundColor Green
        Write-Host '          no mesos line -> paste the "paid ... mesos" log line; 1x -> check !rates' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  24. CASH SHOP BEAUTY PREVIEW (2026-09-26): Beauty > Hairstyles > a hair coupon ->' -ForegroundColor White
        Write-Host '       the panel shows hairstyles; face coupons the same -> DONE' -ForegroundColor Green
        Write-Host '          empty -> grep the log for "CashShopBeautyPreview"; crash on entry -> say so first' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  23. SPAWN POINTS (2026-09-26): log off far across Kerning, log in -> nearest spawn;' -ForegroundColor White
        Write-Host '       channel change / Cash Shop the same; Nella or a Return Scroll -> varied spots -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  22. 2ND JOB TEST (2026-09-26): accept Test of Qualification -> warped in, quest shows' -ForegroundColor White
        Write-Host '       30 Dark Marbles; leave -> beside the instructor; talk again -> go back in? -> DONE' -ForegroundColor Green
        Write-Host '          last line only OK = old build; no warp = paste "second-job test:" lines' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  21. LAUNCHER (2026-09-25): "maplecw-launcher.exe --updated-from no-such-file.old" shows the' -ForegroundColor White
        Write-Host '       sign-in-again notice; a real update explains before reopening -> DONE' -ForegroundColor Green
        Write-Host '       Log > Copy logs, paste into Notepad: every line, [info]/[WARN] tags -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  20. PET FEED LINE (2026-09-25): two players, one feeds: SAME bubble on both -> DONE' -ForegroundColor Green
        Write-Host '          watcher: eating animation too, one bubble; else paste "pet line:" log lines' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  19. PET AUTO HP / MP (2026-09-25): the pet drinks, HP/MP rise, stack -1 -> DONE' -ForegroundColor Green
        Write-Host '          nothing -> grep the log for "pet 0 drinks"; then check inventory still works' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  18. ITEM VARIANCE (2026-09-24): mob-dropped equips roll their stats.' -ForegroundColor White
        Write-Host '       a. hover a dropped equip: a stat differs from its base -> DONE' -ForegroundColor Green
        Write-Host '          plain template every time -> paste the "variance:" log lines' -ForegroundColor Yellow
        Write-Host '       b. a stat BELOW base draws fine, no crash; c. same after relog -> DONE' -ForegroundColor Green
        Write-Host '       d. King Slime: each member''s Squishy Shoes differ -> DONE' -ForegroundColor Green
        Write-Host '       e. drop -> Chaos -> Innocence: back to the DROPPED stats, not plain -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  17. FIRST TIME TOGETHER LAST STAGE (2026-09-24): two members, kill the King.' -ForegroundColor White
        Write-Host '       a. each screen: Pass, mesos, then ONE Shoes at the row end, even spacing -> DONE' -ForegroundColor Green
        Write-Host '          shoes on the Pass = old build; a gap = paste the "is PERSONAL to" lines' -ForegroundColor Yellow
        Write-Host '       b. Jr. Necki / Curse Eye: a Pass and mesos, never a Coupon -> DONE' -ForegroundColor Green
        Write-Host '       c. Exit map: Nella takes every Pass and Coupon (grey lines), nothing else -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  16. MAPLE CHAT ACROSS CHANNELS + BUDDY CHAT (2026-09-24). Rooms live in the' -ForegroundColor White
        Write-Host '     hub now, like parties. REBUILD THE HUB TOO - an old one drops the frames.'
        Write-Host '       a. open on ch1, invite someone on ch2, they Accept: both avatars -> DONE' -ForegroundColor Green
        Write-Host '          "busy" again -> no "maple chat: hub echo" line = the hub is old' -ForegroundColor Yellow
        Write-Host '       b. lines typed on both sides show on both -> DONE' -ForegroundColor Green
        Write-Host '       c. one closes: the other window drops their avatar -> DONE' -ForegroundColor Green
        Write-Host '       d. BUDDY chat reaches your buddies, same channel and across -> DONE' -ForegroundColor Green
        Write-Host ''
        Write-Host '  15. CASH SHOP (2026-09-23): 483 NEW WARES, all 100 LP, no duration on any' -ForegroundColor White
        Write-Host '     cash equipment. Restart the world servers first (they load the table).'
        Write-Host '       a. Gloves and Effects tabs, empty before, now have items -> DONE' -ForegroundColor Green
        Write-Host '       b. a hat or top: 100 LP and NO "90 days" line -> DONE' -ForegroundColor Green
        Write-Host '       c. buy Red Boxing Gloves: arrives, 100 LP taken -> DONE' -ForegroundColor Green
        Write-Host '          "NOT on sale" -> the world server still has the old table' -ForegroundColor Yellow
        Write-Host '       d. Frieren/Himmel clothes are NOT listed (coupons only) -> DONE' -ForegroundColor Green
        Write-Host '       e. DELETE an item in Cash Inventory: it vanishes, "The cash item has been' -ForegroundColor Green
        Write-Host '          deleted." THEN buy something - it still works -> DONE' -ForegroundColor Green
        Write-Host '          next buy does nothing -> the latch stayed set; paste the 0x05AE line' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  14. GROUND DROPS (2026-09-23): a player''s drop is EVERYONE''s now, and an' -ForegroundColor White
        Write-Host '     untradeable one VANISHES. The server always allowed it; the 0x046E told the'
        Write-Host '     other clients it was the dropper''s (ownType 0). Now ownType 2. No capture of'
        Write-Host '     the failure exists, so this step is what proves it. Two clients, one map:'
        Write-Host '       a. The owner drops a sword; Tester2 picks it up -> in TESTER2''s bag -> DONE' -ForegroundColor Green
        Write-Host '          nothing -> grep world-ch0.log for Tester2''s 0x032C: absent = the client' -ForegroundColor Yellow
        Write-Host '          still will not ask (suspect ownerId next); present = paste "pick-up:"' -ForegroundColor Yellow
        Write-Host '       b. mesos the same way: Tester2 gets all of them' -ForegroundColor Green
        Write-Host '       c. leave one: it lasts the full lifetime, then fades on BOTH screens' -ForegroundColor Green
        Write-Host '       d. The owner drops an UNTRADEABLE item: enter type 3, the client''s OWN' -ForegroundColor Green
        Write-Host '          disappearing animation, on BOTH screens, no pick-up prompt -> DONE' -ForegroundColor Green
        Write-Host '          normal landing, then a fade at ~1.5 s -> type 3 is not the animation' -ForegroundColor Yellow
        Write-Host '          fades then blinks/reappears at ~1.5 s -> the cleanup is redundant' -ForegroundColor Yellow
        Write-Host '       e. LATE ARRIVAL: the owner drops a sword, THEN Tester2 portals in: sees it and' -ForegroundColor Green
        Write-Host '          can take it -> DONE. Invisible -> the entry re-send is still wrong' -ForegroundColor Yellow
        Write-Host '       f. a mob''s QUEST item still goes to the killer and does NOT vanish' -ForegroundColor Green
        Write-Host ''
        Write-Host '  13. THE TRADE WINDOW (2026-09-22): the invite worked; the packet that OPENS' -ForegroundColor White
        Write-Host '     the window (0x0575 mode 4) did not exist. Its per-member payload is the'
        Write-Host '     SAME avatar block 0x0224 carries. DONE 2026-10-03: both windows opened.'
        Write-Host '     Putting things in is step 43 now.'
        Write-Host '       a. Tester2 invites, the owner accepts: BOTH windows open, each showing the' -ForegroundColor Green
        Write-Host '          other player in the far seat -> DONE. One side only -> say which.' -ForegroundColor Green
        Write-Host '       b. IF THE CLIENT DIES ON ACCEPT: paste client-exit.log + the CLIENT' -ForegroundColor Yellow
        Write-Host '          FAULT line. The last virtual call is the one [D] in this packet.' -ForegroundColor Yellow
        Write-Host '       c. blank/naked seat -> the look block; names swapped -> mySlot inverted' -ForegroundColor Yellow
        Write-Host '       d. TWO windows on one screen -> the creator should get mode 0xB instead' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  12. FRIENDS - THE BUDDY LIST DRAWS, THE LOOP IS GONE (2026-09-22, 3rd pass)' -ForegroundColor White
        Write-Host '     Confirmed on screen: popup, Yes/No, and the Buddy tab listing the owner under'
        Write-Host '     "Default Group (1/1)". Fixed since: answering the group report (sub-op 0x14)'
        Write-Host '     was an INFINITE LOOP - 32566 round trips, a 42 MB log, the lag and the frozen'
        Write-Host '     window. It is a report now and gets no reply. Added: a 60 s request TIMEOUT.'
        Write-Host '       a. add + accept, then SIT FOR A MINUTE: no lag, buddy list opens instantly' -ForegroundColor Green
        Write-Host '          still laggy -> say WHICH action starts it, then count 0x0193 sub-ops' -ForegroundColor Yellow
        Write-Host '       b. The owner in Tester2''s list and Tester2 in the owner''s, under Default Group' -ForegroundColor Green
        Write-Host '       c. JOB and LV now filled, online AND offline (the row builder was read:' -ForegroundColor Green
        Write-Host '          LV = rec+0x139, JOB = rec+0x13D into the job-name lookup) -> DONE' -ForegroundColor Green
        Write-Host '          still blank -> old build. Level right, job wrong -> say what it shows' -ForegroundColor Yellow
        Write-Host '       c2. LOCATION: the status line says "Tester2 - Kerning City", not chat.' -ForegroundColor Green
        Write-Host '          Other channel -> the channel. "Channel 1" for ch 2 -> off by one' -ForegroundColor Yellow
        Write-Host '       d. TIMEOUT: ignore the balloon for a minute -> both read "The request to' -ForegroundColor Green
        Write-Host '          add a Friend has been canceled." and the row is gone -> DONE' -ForegroundColor Green
        Write-Host '          balloon still on screen after it -> EXPECTED. Press Yes anyway and say' -ForegroundColor Yellow
        Write-Host '          what happens (nothing should).' -ForegroundColor Yellow
        Write-Host '       e. accept inside the minute, wait two more: nothing is cancelled' -ForegroundColor Green
        Write-Host '       f. PRESENCE (0x2D, NEW): log out as the owner -> Tester2''s row GREYS OUT, no' -ForegroundColor Green
        Write-Host '          chat line. Log back in -> "[Friend] the owner has logged in." and it' -ForegroundColor Green
        Write-Host '          un-greys -> DONE, three reports at once' -ForegroundColor Green
        Write-Host '          nothing changes -> paste the "friends: ... went ONLINE/offline" line' -ForegroundColor Yellow
        Write-Host '          a line on every PORTAL instead of once per login -> say so' -ForegroundColor Yellow
        Write-Host '       g. "Tester2 is now your friend." should now match the COLOUR of "Tester2' -ForegroundColor Green
        Write-Host '          has declined the friend request." Different -> say which is which' -ForegroundColor Yellow
        Write-Host '       h. buddy limit is 50; the 51st is refused. The header still reads [n/0]' -ForegroundColor DarkGray
        Write-Host '          - KNOWN, nothing found yet that sets the client''s own maximum' -ForegroundColor DarkGray
        Write-Host ''
        Write-Host '  NOT A BUG: there is no Maple Chat TYPING indicator and no server can add one.' -ForegroundColor DarkGray
        Write-Host '     Measured three ways 2026-09-22: every 0x01FD the client can build writes mode' -ForegroundColor DarkGray
        Write-Host '     0, 1, 3, 5, 7 or 8 (byte scan, control 38/38 - tools/builder_scan.py), the' -ForegroundColor DarkGray
        Write-Host '     MapleChat.img window has no typing canvas, and none of the 6165 strings says' -ForegroundColor DarkGray
        Write-Host '     "typing". Lines themselves work - that is mode 3.' -ForegroundColor DarkGray
        Write-Host ''
        Write-Host '  11c. CRAFTING (2026-09-21): the Journal is the CLIENT''s window; the server owns' -ForegroundColor White
        Write-Host '     the bag, the mesos and the mastery. One craft = two packets, nothing taken'
        Write-Host '     until the second. 348 recipes; the six quests grant the six professions.'
        Write-Host '       a. !craft smithing 1, stand at the Anvil in Perion, open the Journal:' -ForegroundColor Green
        Write-Host '          Smithing live, the other five greyed -> DONE' -ForegroundColor Green
        Write-Host '          (the Journal is a client KEYBIND, so it opens anywhere; the greyed' -ForegroundColor Yellow
        Write-Host '          tab is the only gate. Still greyed -> paste the skill-record line.)' -ForegroundColor Yellow
        Write-Host '       b. !item 4010000 20, craft a Bronze Plate: -5 ore, -100 meso, +1 plate,' -ForegroundColor Green
        Write-Host '          chat "Smithing''s mastery increased. (+3)" -> DONE' -ForegroundColor Green
        Write-Host '          a red refusal instead -> QUOTE IT; each one is a different check' -ForegroundColor Yellow
        Write-Host '       c. !craft smithing 2 0, craft ONE: the bar reads 1.80% (3 of 166).' -ForegroundColor Green
        Write-Host '          The curve is settled - this only checks the two have not drifted.' -ForegroundColor Green
        Write-Host '       d. Craft All with 20 ore: four crafts off one press -> DONE' -ForegroundColor Green
        Write-Host '       e. !craft lists them, !craft all 10 opens all six, !craft 2 0 closes one' -ForegroundColor Green
        Write-Host '       f. quest 80008 at Silas Irons: "You have learnt Smithing." -> DONE' -ForegroundColor Green
        Write-Host '       g. a quest finished BEFORE today: just log in - the claim backfills it,' -ForegroundColor Green
        Write-Host '          tab live on the first screen (level 1, empty bar; mastery not replayed)' -ForegroundColor Green
        Write-Host ''
        Write-Host '  11b. GIFT DROPS (2026-09-19): !giftdrop <player> <item> [n] [msg] and !giftall' -ForegroundColor White
        Write-Host '     <item> [n] [msg] queue gifts; the Administrator''s box offers them (Claim /'
        Write-Host '     Refuse / Cancel), 7-day expiry, !giftall = one per ACCOUNT, any character.'
        Write-Host '       online target: box at once; offline: notice + box on the FIRST MOVE after' -ForegroundColor Green
        Write-Host '       login; Claim -> item + "Claimed:"; full tab -> "make 1 space", kept' -ForegroundColor Green
        Write-Host '       Cancel keeps it; claim an account gift on the alt, the main sees none' -ForegroundColor Green
        Write-Host '       box with the SetField / no box / blank icon -> say which; paste "giftdrop:"' -ForegroundColor Yellow
        Write-Host '  11a. QUEST HELPER COUNTS AFTER A MAP CHANGE (2026-09-18): the mode-5 restore' -ForegroundColor White
        Write-Host '     skips the client''s recount hook, so every in-progress quest''s record is' 
        Write-Host '     re-sent after the bag. Collection quest + items in bag, change maps:'
        Write-Host '       right count at once (49/5), no popup -> DONE' -ForegroundColor Green
        Write-Host '       still 0/N until a pickup -> paste the QuestRecord lines after the restore' -ForegroundColor Yellow
        Write-Host '       the "n / N" collection popup on the map change -> say so (must not)' -ForegroundColor Red
        Write-Host '  11. A QUEST INTO A FULL BAG (2026-09-17). Mint: quest 1008 completed and the' -ForegroundColor White
        Write-Host '     hat never came. Now the room is counted BEFORE anything moves. Full Equip'
        Write-Host '     tab, Lucas'' letter in Etc, quest 1008 in progress: talk to Lucas.'
        Write-Host '       their box: "Please make 1 space in your Equip tab"; journal, letter,' -ForegroundColor Green
        Write-Host '       EXP all unchanged -> DONE. Free a slot, click again: hat, EXP -> DONE' -ForegroundColor Green
        Write-Host '       box AND their closing line -> paste both ScriptMessage lines' -ForegroundColor Yellow
        Write-Host '       yellow "could not give you item" -> the server is stale' -ForegroundColor Yellow
        Write-Host ''
        Write-Host '  12. THE CRASH - a question, not a test.' -ForegroundColor White
        Write-Host '     A 1.36 GB dump at 00:08, fault 0x14090a6f0 - an address that'
        Write-Host '     appears NOWHERE else in the archive. It is an std::map node walk'
        Write-Host '     hitting a bad pointer, 38 C++ throws before it. You were "just in'
        Write-Host '     the map with monsters", so there is no action to blame and what is'
        Write-Host '     left is something that accumulates.'
        Write-Host '     WHAT WOULD HELP: roughly how long had the client been up?'
        Write-Host ''
        Write-Host '  CARRIED OVER, lower value than the above:' -ForegroundColor DarkGray
        Write-Host '     IRON BODY reducing damage - fixed and unretested. SAY THE W.DEF'
        Write-Host '     NUMBER BEFORE AND AFTER: about a quarter means the percent-to-flat'
        Write-Host '     conversion is right; exactly 25 means the raw percent reached the wire.'
        Write-Host '     !learn 1000001 15 killed the client once, 3.7s after a byte-correct'
        Write-Host '     packet. At ~40s of life: dies early -> fatal; does not -> the'
        Write-Host '     session was long and the command is innocent.'
        Write-Host '     Bowman, Thief and Magician branches. Shop buying. The cash purchase.'
        Write-Host ''
        Write-Host '  COMMANDS (GM): !map !item !exp !heal !job !learn !npcecho !setrates'
        Write-Host '  !nx !lp !resetap !resetsp !npcreload !registrationcode !recoverycode'
        Write-Host '  !track <name> (NEW: channel + map of one player). EVERYONE: !rates,'
        Write-Host '  !online (NEW: who is on, across channels), !tool (NEW 4th favour:'
        Write-Host '  Return to Henesys, once a day, free refusal if already there), !scroll,'
        Write-Host '  !help - a player''s !help shows only those.'
        Write-Host '  Pruned 2026-09-06: !kit !buff !unbuff !npcfx !migsweep !buy !locker and'
        Write-Host '  the per-kind rate setters are GONE. !setrates now takes FIVE fields:'
        Write-Host '  <exp> <meso> <drop> <quest> <party%> - quest multiplies turn-in EXP; party%'
        Write-Host '  is the COPY each other member on the map gets (killer keeps 70%).'
        Write-Host '  !RATES: type it at ~40 s of client life. It killed the client once, 51 min'
        Write-Host '  in, on a corrupted map walk - the same text survived on 08-21. Survives at'
        Write-Host '  40 s = the session was the cause; dies = !rates is fatal, a new finding.'
        Write-Host '  gm-handbook/equips.txt NOW HAS NAMES - and reqLevel, reqSTR, reqDEX,'
        Write-Host '  reqINT, reqLUK and reqJob. 1759 rows, name is the LAST column. That is'
        Write-Host '  the file to read when picking something to !item in.'
        Write-Host '  !learn does this run''s skill setup for you; !item the weapon. !lp'
        Write-Host '  grants LEAF POINTS, the currency the cash shop charges; !nx buys'
        Write-Host '  nothing. !help lists them all.'
    } else {
        # This branch used to be reachable by FORGETTING -SetFieldProbe, and that is how it
        # cost a launch on 2026-08-20: the login server is fine, so it reads as a server bug
        # rather than a missing flag. Since 2026-09-14 it takes -SilentChannel, asked for by
        # name, so nobody arrives here by accident. The steps below are a real run; they are
        # just not THIS run, and today's plan lives entirely in the other branch.
        Write-Host '  -SilentChannel: THE WORLD IS OFF, AND YOU ASKED FOR THAT.' -ForegroundColor Red
        Write-Host '  Login, character list, create and delete all work. But the CHANNEL' -ForegroundColor Red
        Write-Host '  answers NOTHING - Session::handle returns empty for every packet -' -ForegroundColor Red
        Write-Host '  so picking a character will hang on "Connecting...". That is the' -ForegroundColor Red
        Write-Host '  flag doing its job. The run plan is NOT printed on this branch.' -ForegroundColor Red
        Write-Host '  Drop -SilentChannel to get into the world. THIS LINE:' -ForegroundColor Red
        Write-Host ''
        Write-Host ("    powershell -ExecutionPolicy Bypass -File `"{0}\tools\test-server.ps1`" -ServersOnly" -f $root) -ForegroundColor Cyan
        Write-Host ''
        Write-Host '  THE RUN PLAN IS ON THAT BRANCH TOO, so on this one you are reading' -ForegroundColor Red
        Write-Host '  neither the world nor the plan. Written out in full because an' -ForegroundColor Red
        Write-Host '  elevated window opens in system32, where a relative path is not a' -ForegroundColor Red
        Write-Host '  shorter way of saying the same thing - it is a command that fails.' -ForegroundColor Red
        Write-Host ''
        Write-Host '  1. click Login. Any character created in an EARLIER run should be there.'
        Write-Host '  2. create one. Check the name first - a name already used is now refused'
        Write-Host '     by the server rather than always accepted.'
        Write-Host '  3. close the client, run this script again, and click Login. The character'
        Write-Host '     should still be listed. That is the whole point of this run.'
    }
}

# **The sentry marker, written BEFORE the -ServersOnly return.** It sat after it, which meant
# start-servers.cmd - the way every launch actually happens - never reached it and -PoolSentry
# silently did nothing. That is the second time something in this file was placed past that
# return; the test plan was the first.
#
# The `else` is not optional: a stale marker arms a 100 ms allocator walk on an unrelated run,
# a confound invisible in the logs of whatever that run was measuring. The hook also deletes
# the marker once it has read it, so this is belt and braces.
if ($SentryWriteWatch -and -not $PoolSentry) {
    # The write watch lives inside the sentry - it needs the sentry's chunk list to know which
    # pages to protect, and its phase to know when. A run that passed only this flag would arm
    # nothing at all and look identical to one that armed and saw nothing, which is the exact
    # failure mode CLAUDE.md keeps cataloguing.
    Write-Host '-SentryWriteWatch implies -PoolSentry; arming both.' -ForegroundColor Cyan
    $PoolSentry = $true
}
if ($PoolSentry) {
    if ($SentryDumps -lt 0) { $SentryDumps = 0 }
    $sentryCfg = "dumps=${SentryDumps}"
    if ($SentryQuiet) { $sentryCfg = 'dumps=0,stacks=off,coarse=2000' }
    if ($SentryRepair) { $sentryCfg = "${sentryCfg},repair=on" }
    if ($SentryWriteWatch) { $sentryCfg = "${sentryCfg},write=on" }
    Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.sentry') -Value $sentryCfg -Encoding ascii
    Write-Host "POOL SENTRY ARMED ($sentryCfg) - a heartbeat every 60s in the hook log, findings when they happen" -ForegroundColor Cyan
    if ($SentryQuiet) {
        Write-Host '  QUIET MODE: no dumps, no thread scan, and the walk drops to every 2s' -ForegroundColor Cyan
        Write-Host '  except within 5s of a predicted firing. The 703-895ms freezes were the' -ForegroundColor Cyan
        Write-Host '  dumps; the 49ms hitches were the 68-thread stack scan. Both are off.' -ForegroundColor Cyan
        Write-Host '  The period is LEARNED, so the first two catches are still at 100ms and' -ForegroundColor Cyan
        Write-Host '  a catch outside the predicted window resets it. This is a PLAY setting -' -ForegroundColor Cyan
        Write-Host '  use -PoolSentry on its own when the run is a measurement.' -ForegroundColor Cyan
    }
    if ($SentryWriteWatch) {
        Write-Host '  WRITE WATCH IS ON. Around each PREDICTED firing, bucket 1 goes read-only' -ForegroundColor Yellow
        Write-Host '  for ~1.2s so the damaging STORE faults at its own instruction. Nothing is' -ForegroundColor Yellow
        Write-Host '  written to the client. It cannot arm until the FIRST catch gives it a phase.' -ForegroundColor Yellow
        Write-Host '  Look for: "control PASS" (armed), "saw a write into a watched page"' -ForegroundColor Yellow
        Write-Host '  (liveness), and "THE WRITER: ... from RIP" (the answer).' -ForegroundColor Yellow
    }
    if ($SentryRepair) {
        Write-Host '  REPAIR IS ON. The sentry will WRITE to the client: a confirmed damaged' -ForegroundColor Yellow
        Write-Host '  header goes back to the slot size, so the next free of that slot is an' -ForegroundColor Yellow
        Write-Host '  ordinary free instead of 0xC0000374. It does NOT stop the writer, and it' -ForegroundColor Yellow
        Write-Host '  is a race - a write and a free inside one 100 ms walk still dies. Count' -ForegroundColor Yellow
        Write-Host '  "header(s) repaired" in the heartbeats and say this flag was on in any' -ForegroundColor Yellow
        Write-Host '  result that depends on the client having stayed alive.' -ForegroundColor Yellow
    }
} else {
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.sentry') -ErrorAction SilentlyContinue
}

# **The session tokens are appended HERE, above the pin.** They used to be appended beside the
# `maplecw-hook.session` write, which sits BELOW the `-ServersOnly` wait loop - a loop that
# never returns. So `-HeapFix -PinPatches -ServersOnly`, which is how every instrumented launch
# is actually run, pinned a session string with no `heapfix=on` in it and the flag did nothing.
# Silently: the launcher would print its OVERRIDES line for a pin that was missing the very
# token the run was about. That is the THIRD time something in this file was placed past that
# return (the test plan was the first, the sentry marker the second), and it is the same fix.
if ($HeapFix) { $Session = "$Session,heapfix=on" }
if ($ClientHitNumberPatch) { $Session = "$Session,hitnumber=off" }
if ($NoBeautyTextPatch) { $Session = "$Session,beautytext=off" }
if ($NoFameTextPatch) { $Session = "$Session,fametext=off" }
if ($FreeGuard) { $Session = "$Session,freeguard=on" }
elseif ($FreeGuardObserve) { $Session = "$Session,freeguard=observe" }
if ($GuardPage) {
    # Refuse a class list the hook would reject, HERE, rather than three minutes into a launch.
    # The hook logs "guardpage=... rejected" and stands down; that is a wasted manual launch.
    foreach ($term in ($GuardBucket -split '[+|]')) {
        if ($GuardBucket.Trim().ToLower() -eq 'all') { break }
        if ($term.Trim() -notmatch '^(0x)?(10|20|40|80|16|32|64|128)$') {
            throw "-GuardBucket '$GuardBucket': '$term' is not one of 0x10/0x20/0x40/0x80 (or 'all', or a +-joined list). Commas are NOT allowed - the session marker is comma-separated."
        }
    }
    $Session = "$Session,guardpage=$GuardBucket"
}

if ($GuardPage) {
    if (-not $PinPatches) {
        Write-Host 'GUARD PAGE needs -PinPatches to reach a launcher run: the launcher writes' -ForegroundColor Yellow
        Write-Host '  maplecw-hook.session with its OWN defaults and would overwrite this.' -ForegroundColor Yellow
    }
    Write-Host "GUARD PAGE: quarantining size class(es) $GuardBucket." -ForegroundColor Cyan
    Write-Host '  Each allocation of those classes gets its OWN page; its free DECOMMITS the' -ForegroundColor Cyan
    Write-Host '  page and holds the address back 200s - one full firing of the 180s clock' -ForegroundColor Cyan
    Write-Host '  plus slack; 600s until 2026-09-16, when the 16:46 run spent the reserve at' -ForegroundColor Cyan
    Write-Host '  254s: 0x20 ran at 34173/s, 22x the run it was sized from, and the two' -ForegroundColor Cyan
    Write-Host '  classes share ONE cursor. The reserve is 16777216 slots (64 GB of address' -ForegroundColor Cyan
    Write-Host '  space), metadata committed lazily: ~65 MB at arm, ~210 MB of live pages' -ForegroundColor Cyan
    Write-Host '  for 0x20 at that churn. READ "recycled" (must be > 0 from ~260s) and' -ForegroundColor Cyan
    Write-Host '  "FELL BACK" (must stay absent) in every heartbeat before anything else.' -ForegroundColor Cyan
    Write-Host '  In the hook log: "GUARD PAGE ARMED ... control PASS" (it states the sizing' -ForegroundColor Cyan
    Write-Host '  model and its headroom), a per-class heartbeat line, "the FIRST free of class' -ForegroundColor Cyan
    Write-Host '  0x40 came back through our HeapFree shim" (the control that a NEWLY watched' -ForegroundColor Cyan
    Write-Host '  class is actually intercepted), and on a hit "GUARD PAGE - STALE WRITE at X' -ForegroundColor Cyan
    Write-Host '  ... RIP R ... allocated from A freed from F". R is THE ANSWER.' -ForegroundColor Cyan
    Write-Host '  It writes to the client (an inline hook on the allocator + a HeapFree swap);' -ForegroundColor Cyan
    Write-Host '  it stands down and REVERTS if the prologue does not match or the self-test' -ForegroundColor Cyan
    Write-Host '  fails, and says which.' -ForegroundColor Cyan
}

if ($FreeGuard -or $FreeGuardObserve) {
    if (-not $PinPatches) {
        Write-Host 'FREE GUARD needs -PinPatches to reach a launcher run: the launcher writes' -ForegroundColor Yellow
        Write-Host '  maplecw-hook.session with its OWN defaults and would overwrite this.' -ForegroundColor Yellow
    }
    Write-Host "FREE GUARD: $(if ($FreeGuard) { 'REFUSE' } else { 'OBSERVE' }) mode." -ForegroundColor Cyan
    Write-Host '  It replaces PCOM+0xdbb80, the cached free ALL SEVEN of PCOM''s free sites' -ForegroundColor Cyan
    Write-Host '  call through. A pointer whose first three qwords read as a pool chunk is' -ForegroundColor Cyan
    Write-Host "  $(if ($FreeGuard) { 'not freed' } else { 'logged and freed anyway' }); everything else passes untouched." -ForegroundColor Cyan
    Write-Host '  In the hook log: "FREE GUARD ARMED", then a liveness line every 120s with a' -ForegroundColor Cyan
    Write-Host '  pass-through count. ZERO passes means the shim is NOT on the free path, and' -ForegroundColor Cyan
    Write-Host '  no refusal count from that run means anything. Say which you saw.' -ForegroundColor Cyan
    Write-Host '  It does NOT stop the writer and it is NOT the pool repair.' -ForegroundColor Cyan
}

# The launcher pins. Same `else` and the same reason as the sentry marker above: a pin left
# behind by one run would silently change which bytes of the client the NEXT run patches, and
# that is the highest-consequence stale marker this project could leave lying about. The
# launcher deletes a pin as it reads it; this clears one that was never consumed.
if ($PinPatches) {
    Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.probe.pin') -Value $Probe -Encoding ascii -NoNewline
    Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.session.pin') -Value $Session -Encoding ascii -NoNewline
    Write-Host 'LAUNCHER PATCHES PINNED for the NEXT launch only:' -ForegroundColor Cyan
    Write-Host "  probe:   $Probe" -ForegroundColor Cyan
    Write-Host "  session: $Session" -ForegroundColor Cyan
    Write-Host '  The launcher deletes each pin as it reads it and prints OVERRIDES in its log' -ForegroundColor Cyan
    Write-Host '  pane. If you do not see that line, the pin was not picked up - say so rather' -ForegroundColor Cyan
    Write-Host '  than assuming the run was instrumented.' -ForegroundColor Cyan
} else {
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.probe.pin') -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.session.pin') -ErrorAction SilentlyContinue
}

if ($ServersOnly) {
    # Stop here. The launcher writes the hook markers and starts the client, so neither the
    # marker block below nor the launch after it should run - one writer, and it is whichever
    # of the two is driving. See the note in the -Launcher block just below.
    Write-Host ''
    Write-Host 'SERVERS ARE UP. The client has NOT been started.' -ForegroundColor Green
    Write-Host ''
    Write-Host '  Now double-click the launcher:' -ForegroundColor Cyan
    Write-Host ("    {0}\target\release\maplecw-launcher.exe" -f $root)
    Write-Host '  It asks for administrator, which is what lets the client start without a'
    Write-Host '  second prompt. Sign in with an account name OR its email, then Start Game.'
    Write-Host ''
    Write-Host ("  accounts:  & `"{0}\target\release\maplecw-useradd.exe`" --db `"{1}`" --list" -f $root, $Database)
    Write-Host ("  add one:   & `"{0}\target\release\maplecw-useradd.exe`" --db `"{1}`" <name> --email <addr>" -f $root, $Database)
    Write-Host ''
    Write-Host 'THIS WINDOW IS THE SERVER. Close it, or press Ctrl+C, to stop.' -ForegroundColor Green
    Write-Host '  There is no separate stop script on this path and that is the point:' -ForegroundColor Green
    Write-Host '  the servers share this console, so closing it takes them with it.' -ForegroundColor Green
    Write-Host ''

    # **Exit forensics, on the path the owner actually launches from - and this is the SECOND
    # thing found sitting past this return.** `exit-forensics.ps1` is started in exactly one
    # place, right after the client launch below, which -ServersOnly never reaches. So on
    # this path `client-exit.log` was never written, and the test plan has been telling the owner
    # to read it first for weeks. On 2026-09-09 two clients exited after 13.5 minutes and the
    # file on disk was from 09-06: no exit code, no thread census, nothing.
    #
    # That is CLAUDE.md's WER lesson exactly - a step that could only ever come back empty
    # looks identical to a step that ran and found nothing.
    #
    # The watcher attaches to clients it FINDS rather than to one it was handed, so it does
    # not care that the launcher starts them, and it writes one log per pid because two
    # clients sharing one file is how the second one's answer overwrites the first's.
    $exitWatch = Start-Process -FilePath 'powershell' -WindowStyle Hidden -PassThru -ArgumentList @(
        '-ExecutionPolicy', 'Bypass', '-File', "`"$(Join-Path $here 'client-exit-watch.ps1')`"",
        '-Root', "`"$root`"", '-ParentPid', $PID
    )
    if ($exitWatch) {
        Write-Host ("exit forensics: watching for clients (pid {0}) -> client-exit-<clientpid>.log" -f $exitWatch.Id) -ForegroundColor Green
    } else {
        Write-Host 'exit forensics: COULD NOT START - a client exit will not be measurable' -ForegroundColor Red
    }
    Write-Host ''

    # **The plan, on the path the owner actually launches from.** It used to sit after the
    # client launch, which -ServersOnly never reaches, so start-servers.cmd printed
    # everything EXCEPT the one thing this file exists to put in front of them. They
    # noticed; the script did not.
    Show-TestPlan
    Write-Host ''
    Write-Host '  Scroll up for the plan above, or read it any time with:' -ForegroundColor DarkGray
    Write-Host '    Get-Help "C:\MapleCW\tools\test-server.ps1" -Full' -ForegroundColor DarkGray
    Write-Host ''

    # THE WAIT. Two things stop it, and they arrive by different routes:
    #
    #   Ctrl+C          PowerShell raises a terminating error out of Start-Sleep, so the
    #                   `finally` below runs and stops whatever is left.
    #   closing the X   every process attached to this console gets CTRL_CLOSE_EVENT,
    #                   children included. They exit on their own; the `finally` may not get
    #                   to run at all, and does not need to.
    #
    # "Gracefully" is worth being precise about, because nothing here runs a shutdown routine.
    # It means nothing is lost: `login::server::log` flushes stdout on EVERY line, so the logs
    # are complete to the last thing that happened, and SQLite is in WAL mode, which is
    # crash-safe by construction. A terminated server loses no state and no evidence.
    #
    # The gap, stated rather than hidden: killing THIS process from Task Manager sends no
    # console event and runs no `finally`, so the children survive it. That is the one case
    # `-Stop` is still for.
    $watched = @($server)
    foreach ($w in $worldAll) { $watched += $w }
    try {
        while ($true) {
            Start-Sleep -Seconds 1
            $dead = @($watched | Where-Object { $_.HasExited })
            if ($dead.Count -gt 0) {
                Write-Host ''
                Write-Host 'A SERVER EXITED ON ITS OWN - that is not you closing the window.' -ForegroundColor Red
                foreach ($d in $dead) {
                    Write-Host ("  {0} (pid {1}) exit code {2}" -f $d.ProcessName, $d.Id, $d.ExitCode) -ForegroundColor Red
                }
                Write-Host '  The usual cause is an account that does not exist. Read:' -ForegroundColor Red
                Write-Host ("    {0}" -f $serverLog) -ForegroundColor Red
                Write-Host ("    {0}.err" -f $serverLog) -ForegroundColor Red
                break
            }
        }
    }
    finally {
        Write-Host ''
        Write-Host 'stopping the servers...' -ForegroundColor Cyan
        Stop-All
        Write-Host 'stopped.' -ForegroundColor Green
    }
    return
}

# THE DEFAULT PATH since 2026-09-05: the launcher drives. The old default - MapleStory.exe
# started directly and served as --account with nobody signed in - is -DirectClient, below.
if (-not $DirectClient) {
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
    # **Existing is not current, and the difference cost a run.** The build above should have
    # made this impossible, but a build can FAIL TO REPLACE a running or elevated-owned exe
    # with "Access is denied (os error 5)" - which is how the 01:53 launcher survived an 02:17
    # source edit. So this is measured rather than assumed, from the file that is actually
    # about to be started.
    $launcherBuilt = (Get-Item $launcherExe).LastWriteTime
    $newestSrc = Get-ChildItem (Join-Path $root 'crates\launcher\src') -Recurse -File |
        Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if ($newestSrc -and $newestSrc.LastWriteTime -gt $launcherBuilt) {
        Write-Host ''
        Write-Host 'THE LAUNCHER BINARY IS OLDER THAN ITS SOURCE. STOP.' -ForegroundColor Red
        Write-Host ("  built  {0}" -f $launcherBuilt) -ForegroundColor Red
        Write-Host ("  source {0}  ({1})" -f $newestSrc.LastWriteTime, $newestSrc.Name) -ForegroundColor Red
        Write-Host '  The build above did not replace it - almost always because a client or'
        Write-Host '  launcher still holds it, or an elevated run owns the file. Close both,'
        Write-Host '  delete it by hand, and re-run. A stale launcher writes stale markers,'
        Write-Host '  and a hook feature it does not arm looks EXACTLY like a feature that'
        Write-Host '  does not work.'
        Write-Host ''
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
# ARCHIVED, not deleted - and it used to be deleted, while world-ch0.log beside it was kept.
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
# -DirectClient from here on. Login is enforced, so this client is refused at the login
# screen unless the login server was told whom to serve it as - and a run that cannot get
# past the login screen measures nothing, so it is refused HERE, with the fix.
if (-not $FallbackAccount) {
    Write-Host ''
    Write-Host '-DirectClient starts the client with no sign-in, and LOGIN IS ENFORCED.' -ForegroundColor Red
    Write-Host '  The login server would answer it with "not a registered ID". To serve a'
    Write-Host '  direct client as an account anyway (dev only - anything reaching the'
    Write-Host '  port is then served as it too), pass:'
    Write-Host '    -DirectClient -FallbackAccount maplecw' -ForegroundColor Yellow
    Write-Host '  Or drop -DirectClient and sign in through the launcher, which is the'
    Write-Host '  ordinary run now.'
    Write-Host ''
    Stop-All
    throw '-DirectClient needs -FallbackAccount <name>'
}
New-Item -ItemType Directory -Path $dumpDir -Force | Out-Null
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.dumpdir') -Value $dumpDir -Encoding ascii
# **VALIDATE THE PROBE SPEC BEFORE IT COSTS A RUN.**
#
# 2026-09-08 14:47: a spec of "watch@A,watch@B,watch@C,watch@D,watch@E" armed ONE target. The
# grammar is a SINGLE `watch@` prefix followed by comma-separated targets - `arm_watch` does
# `strip_prefix("watch@")` once and splits the remainder on commas - so every term after the
# first carried a literal "watch@" and was refused one line at a time in the hook log, which
# nobody reads until after the client has died. Four of five watches were silently absent.
if ($Probe -and $Probe.Trim()) {
    $spec = $Probe.Trim()
    if ($spec.StartsWith('watch@')) {
        $rest = $spec.Substring(6)
        if ($rest -like '*watch@*') {
            Write-Host ''
            Write-Host 'THE PROBE SPEC REPEATS "watch@" AND MOST OF IT WOULD BE IGNORED.' -ForegroundColor Red
            Write-Host '  The grammar is ONE watch@ then comma-separated targets:' -ForegroundColor Red
            Write-Host '    -Probe "watch@140c93530,140c936a0,140c93810"' -ForegroundColor Yellow
            Write-Host '  not  -Probe "watch@140c93530,watch@140c936a0,..."' -ForegroundColor Yellow
            throw 'probe spec repeats watch@; only the first target would arm'
        }
        foreach ($term in ($rest -split ',')) {
            if (-not $term.Trim()) { continue }
            $target = ($term -split ':')[0].Trim()
            if ($target -notmatch '^[0-9a-fA-F]+$' -and $target -notmatch '^[A-Za-z0-9_.-]+!.+$') {
                throw "-Probe target '$target' is neither a hex VA nor <module>!<export>. The hook would refuse it and the run would be short that watch."
            }
        }
    }
}

Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.probe') -Value $Probe -Encoding ascii
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.session') -Value $Session -Encoding ascii

# **READ THE MARKER BACK AND PROVE THE FLAGS ARE IN IT.**
#
# 2026-09-08: an overnight run was spent with no guard page because `-GuardPage` never reached
# this file. The marker read `mode=2,create=on`, the hook found no `guardpage=` token, and
# every other line in the log looked healthy. Writing a file is not evidence that it says what
# you meant; this reads it back and refuses the launch if a flag that was asked for is missing.
# The same rule as everywhere else here - verify the instrument before believing it.
#
# **And this guard was itself driven both ways before it shipped**, because a guard that has
# never refused anything is a guard nobody has tested. Five cases, all passing: it refuses the
# exact 2026-09-08 marker (`mode=2,create=on` with -GuardPage asked for); it accepts a correct
# one; it refuses `guardpage=0x40` when 0x20 was asked for, which is the subtler version of the
# same loss and the one that would look right at a glance; it leaves a plain run alone; and it
# catches a missing `heapfix=on` too. Re-drive it by copying this block into a scratch .ps1 with
# $ClientDir, $Session and the switches defined - the same way the Write-Host plan is rendered.
$markerPath = Join-Path $ClientDir 'maplecw-hook.session'
$markerBack = (Get-Content -Path $markerPath -Raw -ErrorAction SilentlyContinue)
if ($null -eq $markerBack) { throw "the session marker was not written to $markerPath" }
$markerBack = $markerBack.Trim()
$wanted = @()
if ($GuardPage) { $wanted += "guardpage=$GuardBucket" }
if ($HeapFix) { $wanted += 'heapfix=on' }
if ($FreeGuard) { $wanted += 'freeguard=on' }
elseif ($FreeGuardObserve) { $wanted += 'freeguard=observe' }
$missing = @($wanted | Where-Object { $markerBack -notlike "*$_*" })
Write-Host ''
Write-Host "SESSION MARKER: $markerBack" -ForegroundColor Cyan
if ($missing.Count -gt 0) {
    Write-Host ''
    Write-Host 'THE SESSION MARKER IS MISSING A FLAG YOU ASKED FOR.' -ForegroundColor Red
    foreach ($m in $missing) { Write-Host "  missing: $m" -ForegroundColor Red }
    Write-Host '  The client would run WITHOUT it and the log would look healthy - which is'
    Write-Host '  exactly how the 2026-09-08 overnight run was spent. Refusing to launch.'
    throw 'session marker does not carry the requested flags'
}
if ($GuardPage) {
    Write-Host "  guardpage=$GuardBucket is in the marker. In the hook log expect either" -ForegroundColor Cyan
    Write-Host '  "GUARD PAGE ARMED ... control PASS" or a line saying why it stood down.' -ForegroundColor Cyan
}
# **Only maplecw-launcher can mint a client credential, because only it signs in.** This
# path is a direct run, so clear any leftover: presenting a stale token is refused, and a
# refusal downgrades the connection to the --account fallback silently.
#
# -ClientToken overrides that, and the parameter's own comment says why: without SOME token in
# the client's identity field, the client reports 0x00C0 and never sends a character selection,
# so a direct run cannot reach the world at all. The token written here is wrong by
# construction and the login server downgrades it to -FallbackAccount, which is the account
# this run was going to be served as anyway.
if ($ClientToken) {
    Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.identity') -Value $ClientToken -Encoding ascii -NoNewline
    Write-Host "client token: $($ClientToken.Length) chars written to maplecw-hook.identity - the login server will REJECT it and serve this connection as `"$FallbackAccount`". The point is the CLIENT's field, not the server's answer." -ForegroundColor Yellow
} else {
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.identity') -ErrorAction SilentlyContinue
}
# **And the multiclient marker, for the same reason and a sharper one.** The launcher writes
# it on every launch and NOTHING deletes it - the copy in client-patched\ dated from a launcher
# run twelve days earlier - so a -DirectClient run silently inherited the FindWindow and
# CreateMutex hooks from whenever the launcher last ran. That is exactly the stale-marker
# confound this script already guards against for the sentry, and it matters now: -DirectClient
# is the minimal-patch path, and research/is-the-corruption-ours-2026-09-06.md §5 uses it as the
# control that decides whether any of our patches touch the heap corruption. A control that
# quietly carries one of the things it is meant to exclude is not a control. The launcher
# rewrites this marker on its own next run, so removing it here costs that path nothing.
Remove-Item (Join-Path $ClientDir 'maplecw-hook.multiclient') -ErrorAction SilentlyContinue
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
    Write-Host '  -> THIS CANNOT WORK AND THE STEP IS KEPT ONLY AS A LABEL.' -ForegroundColor DarkYellow
    Write-Host '     research/client-session-args.md section 2: the reader for cfg+0x90'
    Write-Host '     has ZERO callers, so nothing the command line puts there reaches'
    Write-Host '     the wire. Of every slot the arguments can set, only the mode u32'
    Write-Host '     is ever encoded. The credential goes in via the hook instead -'
    Write-Host '     see the IDENTITY steps in the plan.'
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

Show-TestPlan
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
