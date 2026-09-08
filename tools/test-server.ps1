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

    -SetFieldProbe is NOT optional: without it Session::handle returns nothing for EVERY
    packet and the client sits on "Connecting...". Run -Stop before relaunching.

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
    invite the same character again: it must go through; world.log prints "LAPSED".

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
    (untradeable excepted). PARTY HP (built 2026-09-06): the packet is 0x02B2, found by
    walking back from the HUD gauge to the field it reads, to that field's one writer, to the
    handler, to a compacted third switch in the remote-user router; every link [L]. The other
    member's bar in the top-right HUD should fill within a second of the party forming and
    follow their HP under damage and potions; say whether the small bar over their head moves
    too (it is fed from the same packet, a second field). A blank bar is the finding. One
    thing is NOT built because its opcode has never been captured, and guessing a packet body
    has killed this client three times: DROPPING MESOS. Try it and read the inbound opcode off
    world.log (or report that none appears) - that one measurement is all it needs.

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
    packet carries no slot). If a throw takes NOTHING and world.log says "a 0x00E0 SHOOT body
    did not parse", that is the finding: no shoot body has ever been captured and the parser
    comes from melee. RECHARGE: at any Grocer (Lucy, Mina, Luna...) select a partial star stack
    in your inventory and press Recharge. Expected: the stack fills to slotMax (Subi 500) and
    the meso count drops by ceil(missing x unitPrice), Subi 0.3 per star; write down the number
    the window showed next to "Recharge:" and the number the mesos moved by - if they differ
    the rounding direction is the client's and ours is wrong by that much. A star Lucy does not
    list (Wolbi and up) is refused with the not-enough-mesos message; that is deliberate. SLASH
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
      (b) MOB DROPS - kill a mob WHILE IT IS WALKING (a snail mid-stride, not a standing one).
            the drop lands on the corpse            -> fixed.
            it lands ~40 px BEHIND the walk         -> still the path head.
          The arc matters as much as the landing: an item flying out of empty space behind the
          mob is the same bug on a different field. Also kill a mob that has NEVER moved: it
          must drop at the mob, not at your feet.
      (c) QUEST ITEMS - kill Slimes/Octopuses without the Omok quest. NO Omok Piece may drop.
          Then take the quest and kill again: it must drop. Dark Marbles are EXEMPT by design
          and must keep dropping for a second-job run.
      (d) LEVEL UP - needs TWO clients on ONE map. Level one and watch the OTHER screen for the
          animation and sound. This cannot be confirmed from the server side: the client drops
          the packet in silence if that observer does not already hold the leveller's spawn.
      (e) !tool - type it in chat on a NON-GM account. It is a public command.
            a box with the Maple Administrator's PORTRAIT and three numbered lines
                                                   -> the whole chain works. Pick Level up:
                                                      expect a level, an EXP line, +5 AP.
            three lines but NO PORTRAIT            -> the speaker template is not resolving.
                                                      This is the ONE thing the tests cannot
                                                      see, so look at the portrait on purpose.
            "is not a command"                     -> the dispatcher never reached it.
            nothing at all                         -> worse than a refusal; check world.log for
                                                      the 0x00E7 and whether anything went back.
          Then run !tool again the SAME session and pick the same option: it must REFUSE IN
          WORDS, never go silent, and the log must say NOTHING PAID. Reset is UTC midnight.
          Leaf Points are per ACCOUNT (a second character is refused); Level up and the AP/SP
          reset are per CHARACTER (a second character still gets its own).
      (f) THE MAPLE ADMINISTRATOR HERSELF - Henesys, far left near the portal. Click them.
          They must give their QUEST or their greeting and NEVER the favours menu. That is the
          point of (e) being a command: the client's click fork is keyed on their TEMPLATE, so
          a summoned copy of their would send bytes identical to clicking them.

    T20 (NEW 2026-09-08). THE OVERNIGHT RUN - SURVIVE, do not measure. The owner: "our goal is to
    leave the client running overnight without it exiting." That is a different run from every
    one below it, and it wants a different command:

      -SetFieldProbe -PoolSentry -SentryQuiet -SentryRepair -GuardPage -PinPatches

    NO -SentryWriteWatch. The write watch only OBSERVES: it makes pool pages read-only around
    each predicted firing and single-steps every write through them, up to 20 000 faults per
    window, ~160 windows in eight hours. It cannot prevent anything, and overnight it is pure
    risk and CPU. -SentryQuiet is the long-run cadence (no dumps, no 68-thread stack scan,
    2 s walk except near a firing) and it KEEPS the repair.

    The two surfaces are covered by different mechanisms, and both are prevention, not
    observation:
      0x20 pool headers -> the sentry finds the damaged header and REPAIRS it before the free
                           that would be fatal. Proven: runs 3 and 4 ended with a clean pool.
      0x40 map nodes    -> the guard page. The writer's damage only becomes fatal when the
                           pool hands its stale address to a LIVE object (run 2: an empty
                           map's head node, +2 through a recycled address). The quarantine
                           never hands a 0x40 address back within 10 minutes, so the increment
                           lands on a decommitted page nobody owns, is logged, and the page is
                           recommitted. The live object is never touched.

    WHAT TO READ IN THE MORNING, in client-patched\maplecw-hook.log:
      "GUARD PAGE ARMED ... control PASS"  - it armed. "control FAIL" or a prologue-mismatch
                    line means it stood down and the client ran unpatched by it.
      the "guard page:" heartbeat, every 60 s. "N recycled" climbing after the first ten
                    minutes is the intended steady state.
      "***** N FELL BACK - the class is NO LONGER COVERED *****" - THE ONE FAILURE THAT LOOKS
                    LIKE A HEALTHY RUN. It means allocation outran the 10-minute retirement
                    queue and the 0x40 class went back to the client's own pool. Any number
                    above zero and the rest of the night is uncovered. Report the number.
      "GUARD PAGE - STALE WRITE at X ... RIP R" - the writer, named, AND neutralised. Several
                    of these with the client still up is the run succeeding, not failing.
      "SENTRY REPAIR" lines - the 0x20 half doing its job.
    If the client is still up in the morning, say for how long and paste those counters. If it
    is not, client-exit.log and the last heartbeat say which surface gave way.

    UNKNOWN, said plainly: no run has passed 70 minutes, so eight hours is a long extrapolation
    from a short measurement, and nothing rules out a cause that only shows up at hour three.
    The retirement queue, the thread parking and the whole guard page have never run on a
    client - this is their first launch as well as the first survival attempt.

    RUN 5 (2026-09-08 build). NAME THE WRITER ON BOTH SURFACES IN ONE LAUNCH. The write watch
    (0x20 headers, windows) and the GUARD PAGE (0x40, quarantine) are complementary and run
    together. Run 5 died at 3.5 min on a 0x40 map node used +2 (heap-wild-write dump 2) - a
    surface the window watch structurally cannot reach. The guard page covers it: it serves the
    0x40 class one-slot-per-page and decommits on free, so a stale pointer into a freed 0x40
    slot faults at the writer on any clock. Recipe now:

      -SetFieldProbe -ServersOnly -PoolSentry -SentryRepair -SentryWriteWatch -GuardPage
      -PinPatches -Probe "watch@1415db360:ret,141b2a280:rdx=0,140ca61d0:hits=400"

    In the hook log: the sentry heartbeat gains a "guard page: N served, M freed, K live, C
    STALE-ACCESS CATCH(es)" line; on a catch, "GUARD PAGE - STALE WRITE at X ... RIP R ...
    allocated from A freed from F" - R is the 0x40 writer. "GUARD PAGE ARMED ... control PASS"
    confirms it armed; "control FAIL" or a prologue-mismatch line means it stood down and the
    client is unpatched by it. The guard writes to the client (an allocator inline hook + a
    HeapFree pointer swap), off unless -GuardPage; -GuardBucket picks the class (default 0x40).

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

      -SetFieldProbe -ServersOnly -PoolSentry -SentryRepair -SentryWriteWatch -PinPatches
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
    and world.log must NOT contain it (grep it - the log line says "not logged"). In the
    launcher, Register tab: any username, an email, a password WITHOUT a digit -> refused on
    the spot and the code is still live; with a digit -> "account created", the Sign in tab
    comes back with the name filled in, Login works. Then !recoverycode <that email> in game;
    Forgot password tab with the email, the code and a new password -> the new one signs in
    and the old one is refused. The wrong identity with the right code -> refused AND the code
    still works afterwards. Every one of those sentences is a test in the suite; this run is
    whether the SCREENS say them.

    T11 IS THE MEASUREMENT, and T10 is right behind it. Both are walks, not clicks.

    T11 crosses to ANOTHER CONTINENT - 87 maps, a separate portal component from
    Victoria Island, reachable only by the ferry.

    **ORBIS ITSELF ALREADY LOADED, and this block claimed the opposite for a week.** Map
    20000000 was served twice on 2026-08-28, the client answered 0x00DC 526 ms later and
    four NPCs drew. So "no character has ever stood on them" was false when written, and
    the risky-sounding part of T11 - does the client survive an Orbis map - is already
    answered YES. What is untested is EL NATH and the ferry that reaches it.

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
                       there - because that names the direction. world.log has the length
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

          world.log discriminates all of this without a second launch - grep it for
          "mob control:", which names the count and the recipient on every handover.
     T11. THE THIRD JOB ADVANCEMENT, AND THE FERRY. Set yourself up first:
              !job 110   !exp 31545355   !map 10005000
          That is a level-70 Fighter in Sleepywood. 31 545 355 is the exp curve summed 1 to
          70 and one !exp crosses every level in it. If the level comes out wrong, say what
          it actually was - the curve is ours and that would be a finding of its own.

          a) CLICK EUREK THE ALCHEMIST in Sleepywood (they are at the far right, x=1415).
             A menu should offer TWO stops: Orbis Ticketing Booth and El Nath, at
             #b1000 mesos#k - twice a cab fare, on purpose.
               a menu with two lines -> the ferry works
               their ordinary line about wandering the world -> the click never routed
               a menu with SIX towns -> the network filter is broken and they are being
                          treated as a cab. Say so; it means every cab is now also
                          offering another continent
          b) PICK EL NATH.
               *** THIS IS THE MOMENT. No character has ever been on an Ossyria map. ***
               you arrive in a snowy town -> 87 maps just became reachable. Say so
               black screen, or the client dies -> THE finding of this run. world.log's
                          SetField line names the map; say whether the screen drew
                          anything first
               1000 mesos gone but no warp -> the fare moved and the field did not.
                          world.log will say which
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
          f) CLICK EUREK AGAIN, in El Nath this time (they are there as well - they are the
             only NPC in this client standing on both continents).
               a menu offering Sleepywood and Orbis -> the way home works, and nobody
                          can be stranded on the wrong continent
               their ordinary line -> they are not a port there, and El Nath becomes a trap
                          whose only exit is seventeen floors of the Orbis Tower

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
               the screen goes black / the client dies -> say WHICH, and world.log's
                          SetField line names the map. That map has 132 footholds and 30
                          mobs, so a load failure is a real finding
               nothing happens -> the click never reached job_test_for. Say so; the
                          examiner also carries quests and the client may have sent
                          0x0151 instead of 0x00F2, which is a routing question

          b) KILL ANYTHING IN THERE. Every mob drops one Dark Marble, guaranteed.
               a marble per kill -> the drop rule works
               no marble -> the map gate. world.log's drop line names the map it used
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
                     -> the chain is wrong somewhere. world.log will show the mode-5
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
         !item 2030000 3, then !item 2030009 1. Use each from the bag.
          Nearest Town should warp you to the map's own return town and take ONE scroll.
          The El Nath scroll must REFUSE with a chat notice and LEAVE THE SCROLL IN THE BAG.
            the scroll vanishes on the refusal -> the transition guard is broken
            nothing happens at all on either -> the client never sent 0x010E for a 0203
                       item, and the whole path is dead code. Grep world.log for 0x010E
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

     6. THE KEYBOARD LAYOUT - free, rides along, do it whenever.
        Open keyboard settings, DRAG ONE SKILL ONTO AN EMPTY KEY, close the window. Say
        roughly when. 162 archived captures contain no keymap packet, but nobody has ever
        changed a key DURING one, so that is "never captured" and not "never sent".
          a new opcode near that moment -> the client reports changes and both halves
                     become measured
          nothing new -> the client never volunteers it, and the only route left is the
                     hook reading the client's own memory

     7. THE CRASH - a question, not a test.
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
    !npcecho, !setrates, !npcreload, !registrationcode, !recoverycode. EVERYONE: !rates and
    !help - a player's !help shows only those two. PRUNED 2026-09-06 on the owner's instruction:
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
    the working is in world.log.
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
    # Channel N listens on $ChannelPort + N and logs to world.log (channel 0) or
    # world-ch<N>.log (the rest). Channel 0 keeps the plain name because every doc and
    # instruction in this repo points at world.log.
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
    # character stops entering the world and world.log says "REFUSED the migration":
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
    [string]$GuardBucket = '0x40',
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
$authExe = Join-Path $root 'target\release\maplecw-auth.exe'
if (Test-Path $authExe) {
    $authSrv = Start-Process -FilePath $authExe -WorkingDirectory $root -PassThru @spawn `
        -ArgumentList @('--db', "`"$Database`"", '--bind', '127.0.0.1', '--port', "$AuthPort") `
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
$worldAll = @()
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
    $p = Start-Process -FilePath $worldExe -WorkingDirectory $root -PassThru @spawn `
        -ArgumentList $chArgs `
        -RedirectStandardOutput $chLog -RedirectStandardError "$chLog.err"
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
        Write-Host '  WHAT IS WORTH A RUN NOW, in order:' -ForegroundColor Yellow
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
        Write-Host '       world.log for "REFUSED the migration" - that is the on-box'
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
        Write-Host '       the SAME character again. It must go through. world.log: "LAPSED".'
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
        Write-Host '       with a dagger: 0. If NOTHING drops and world.log says "SHOOT body'
        Write-Host '       did not parse", that is the finding: no 0x00E0 was ever captured.'
        Write-Host '       RECHARGE: at Lucy/Mina/Luna, click a partial star stack, press'
        Write-Host '       Recharge. Stack fills to 500 (Subi); mesos drop by missing x 0.3,'
        Write-Host '       rounded UP. WRITE DOWN the "Recharge:" number the window showed'
        Write-Host '       and what the mesos moved by - a difference is the rounding rule.'
        Write-Host '       Wolbi and up at Lucy is refused (they list only Subi): deliberate.'
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
        Write-Host '       (b) MOB DROPS. Kill a mob WHILE IT IS WALKING, not standing.'
        Write-Host '             lands on the corpse      -> fixed'
        Write-Host '             lands ~40px behind it    -> still the path head'
        Write-Host '           Watch the ARC too: an item flying out of empty space behind'
        Write-Host '           the mob is the same bug on a different field. Then kill a mob'
        Write-Host '           that never moved - it must drop at the mob, not at your feet.'
        Write-Host '       (c) QUEST ITEMS. Kill Slimes with no Omok quest: NO Omok Piece may'
        Write-Host '           drop. Take the quest, kill again: it must. Dark Marbles are'
        Write-Host '           EXEMPT by design and must keep dropping.'
        Write-Host '       (d) LEVEL UP needs TWO clients on ONE map. Level one, watch the'
        Write-Host '           OTHER screen. The server cannot confirm this one.'
        Write-Host '       (e) !tool - type it in chat on a NON-GM account (it is public).'
        Write-Host '             box with their PORTRAIT + 3 lines -> works. Pick Level up:'
        Write-Host '                                                level, EXP line, +5 AP'
        Write-Host '             3 lines but NO PORTRAIT -> speaker template not resolving.'
        Write-Host '                                        The ONE thing tests cannot see -'
        Write-Host '                                        look at the portrait on purpose.'
        Write-Host '             "is not a command"      -> dispatcher never reached it'
        Write-Host '             nothing at all          -> check world.log for the 0x00E7'
        Write-Host '           Run !tool again the same session: it must REFUSE IN WORDS and'
        Write-Host '           log NOTHING PAID. Reset is UTC midnight. Leaf Points are per'
        Write-Host '           ACCOUNT; Level up and the AP/SP reset are per CHARACTER.'
        Write-Host '       (f) THE ADMINISTRATOR HERSELF, Henesys. Click them: they must give'
        Write-Host '           their QUEST or their greeting, NEVER the favours menu. That is why'
        Write-Host '           (e) is a command - the click fork is keyed on their TEMPLATE, so'
        Write-Host '           a summoned copy would send bytes identical to clicking them.'
        Write-Host '    0i. THE OVERNIGHT RUN - the goal is to SURVIVE, not to measure.' -ForegroundColor Green
        Write-Host '       -PoolSentry -SentryQuiet -SentryRepair -GuardPage -PinPatches'
        Write-Host '       and NO -SentryWriteWatch: the watch only observes, and overnight it'
        Write-Host '       is 160 windows of read-only pages and single-stepped writes for no'
        Write-Host '       protection at all. -SentryQuiet keeps the repair and drops the'
        Write-Host '       dumps, the stack scan and the 100ms walk.'
        Write-Host '       Both surfaces are PREVENTED, by different means:'
        Write-Host '         0x20 headers -> the sentry repairs the header before the free'
        Write-Host '                         that would be fatal (runs 3 and 4: clean pool).'
        Write-Host '         0x40 nodes   -> the guard page never hands a freed address back'
        Write-Host '                         within 10 min, so the writer increments a dead'
        Write-Host '                         page instead of a live map node (the run-2 death).'
        Write-Host '       In the morning, in client-patched\maplecw-hook.log:'
        Write-Host '         "GUARD PAGE ARMED ... control PASS"  -> it armed'
        Write-Host '         the "guard page:" heartbeat, "N recycled" climbing = steady state'
        Write-Host '         "N FELL BACK - NO LONGER COVERED"    -> THE FAILURE THAT LOOKS' -ForegroundColor Yellow
        Write-Host '                  HEALTHY. Allocation outran the queue; report the number.' -ForegroundColor Yellow
        Write-Host '         "GUARD PAGE - STALE WRITE ... RIP R" -> the writer, named AND'
        Write-Host '                  neutralised. Several of these with the client still up'
        Write-Host '                  is the run WORKING.'
        Write-Host '       UNKNOWN: no run has passed 70 min, so 8 hours is a long guess from a'
        Write-Host '       short measurement - and the guard page has never run on a client.'
        Write-Host '    0j. NAME THE WRITER - two surfaces, one launch (2026-09-08).' -ForegroundColor Yellow
        Write-Host '       -SentryWriteWatch (0x20 headers) AND -GuardPage (0x40 quarantine)'
        Write-Host '       together. The guard serves the 0x40 class one-slot-per-page and'
        Write-Host '       decommits on free, so a stale pointer into a freed 0x40 slot - the'
        Write-Host '       run-2/run-5 death the window watch cannot reach - faults at the'
        Write-Host '       writer. Watch for "GUARD PAGE ARMED ... control PASS", the heartbeat'
        Write-Host '       "guard page:" line, and "GUARD PAGE - STALE ... RIP R" (the answer).'
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
        Write-Host '       "<- 0x...." appears in world.log when you try (or that none does).'
        Write-Host '    1. THE MOB FLINCH. A non-controller hits a mob: from the'
        Write-Host '       SECOND hit it should flinch and slide. First hit never'
        Write-Host '       will - the grant ships with that swing.'
        Write-Host '    2. EXP SHARING has NEVER executed. 329 kill payouts in the'
        Write-Host '       archive, ZERO carrying a damage fraction. Two clients have'
        Write-Host '       never killed the SAME mob. Do that.'
        Write-Host '    3. T11/T10, single-client, still untested.'
        Write-Host '    4. REGISTRATION AND RECOVERY (new 2026-09-05, no client needed for'
        Write-Host '       the launcher half). As the GM type !registrationcode - a chat'
        Write-Host '       notice shows an 8-character code, XXXX-XXXX, and world.log must'
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
        Write-Host '  T11 CROSSES TO ANOTHER CONTINENT - 87 maps behind a ferry.'
        Write-Host '  ORBIS ITSELF ALREADY LOADED (2026-08-28, 0x00DC accepted, NPCs'
        Write-Host '  drew). This plan claimed it never had, for a week. What is'
        Write-Host '  untested is EL NATH and the ferry - not "can the client survive'
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
        Write-Host '       grep world.log for "mob control:" - it names the count and'
        Write-Host '       the recipient, so this needs no second launch to read.'
        Write-Host '  T11. THIRD JOB + THE FERRY. THE ONE. Set up with:' -ForegroundColor Yellow
        Write-Host '         !job 110   !exp 31545355   !map 10005000'
        Write-Host '       (a level-70 Fighter in Sleepywood. If the level comes out'
        Write-Host '        wrong, say what it was - the exp curve is ours.)'
        Write-Host '       a) CLICK EUREK THE ALCHEMIST (far right, x=1415).'
        Write-Host '            a menu, two stops, 1000 mesos -> the ferry works'
        Write-Host '            their wandering line -> the click never routed'
        Write-Host '            SIX towns -> the network filter is broken and every cab'
        Write-Host '                       now offers another continent too'
        Write-Host '       b) PICK EL NATH. *** THIS IS THE MOMENT. ***'
        Write-Host '            a snowy town -> 87 maps just became reachable'
        Write-Host '            black screen / client dies -> THE finding of this run'
        Write-Host '            mesos gone, no warp -> world.log says which half ran'
        Write-Host '       c) WALK RIGHT into Chief Residence. Four NPCs inside.'
        Write-Host '       d) CLICK TYLUS (they serve Fighter/Page/Spearman).'
        Write-Host '            "You are a Crusader now" -> DONE. Then open the skill'
        Write-Host '                       window: a THIRD page with points on it'
        Write-Host '            no third page -> the SP pool key is wrong. Say both'
        Write-Host '                       halves - the job still changed'
        Write-Host '            "come back at Level 70" -> the !exp did not land'
        Write-Host '       e) CLICK ROBEIRA / RENE / AREC. All must REFUSE, naming the'
        Write-Host '          BRANCH rather than the level.'
        Write-Host '       f) CLICK EUREK AGAIN, in El Nath. They stand on both'
        Write-Host '          continents - the only NPC here that does.'
        Write-Host '            a menu home -> nobody can be stranded'
        Write-Host '            their ordinary line -> El Nath is a trap, exit is 17'
        Write-Host '                       floors of the Orbis Tower'
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
        Write-Host '            black screen / client dies -> say which; world.log names'
        Write-Host '                       the map in its SetField line'
        Write-Host '            nothing happens -> the click never routed'
        Write-Host '       b) KILL ANYTHING IN THERE. Every mob drops one Dark Marble.'
        Write-Host '            no marble -> the map gate; world.log names the map used'
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
    Write-Host '        tooltip still pops -> the chain is wrong; world.log shows'
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
    Write-Host '      !item 2030000 3 then !item 2030009 1.'
        Write-Host '        Nearest Town warps to the map return town, taking ONE.'
        Write-Host '        El Nath must REFUSE and LEAVE THE SCROLL IN THE BAG.'
        Write-Host '          scroll vanishes on a refusal -> the guard is broken'
        Write-Host '          nothing at all -> the client never sent 0x010E. Grep it'
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
        Write-Host '  6. THE KEYBOARD LAYOUT - free, rides along, do it whenever.' -ForegroundColor White
        Write-Host '     Open keyboard settings, DRAG ONE SKILL ONTO AN EMPTY KEY, close'
        Write-Host '     the window. Say roughly when. 162 archived captures have no keymap'
        Write-Host '     packet - but nobody has ever changed a key DURING one, so that is'
        Write-Host '     "never captured", not "never sent".'
        Write-Host ''
        Write-Host '  7. THE CRASH - a question, not a test.' -ForegroundColor White
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
        Write-Host '  !nx !lp !resetap !resetsp !npcreload !registrationcode !recoverycode.'
        Write-Host '  EVERYONE: !rates and !help - a player''s !help shows only those two.'
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
if ($FreeGuard) { $Session = "$Session,freeguard=on" }
elseif ($FreeGuardObserve) { $Session = "$Session,freeguard=observe" }
if ($GuardPage) { $Session = "$Session,guardpage=$GuardBucket" }

if ($GuardPage) {
    if (-not $PinPatches) {
        Write-Host 'GUARD PAGE needs -PinPatches to reach a launcher run: the launcher writes' -ForegroundColor Yellow
        Write-Host '  maplecw-hook.session with its OWN defaults and would overwrite this.' -ForegroundColor Yellow
    }
    Write-Host "GUARD PAGE: quarantining size class $GuardBucket." -ForegroundColor Cyan
    Write-Host '  Each allocation of that class gets its OWN page; its free DECOMMITS the page' -ForegroundColor Cyan
    Write-Host '  and never reuses it. A stale write/read into a freed slot FAULTS at the' -ForegroundColor Cyan
    Write-Host '  instruction that makes it - on any clock, not just the 180s window.' -ForegroundColor Cyan
    Write-Host '  In the hook log: "GUARD PAGE ARMED ... control PASS", a heartbeat line' -ForegroundColor Cyan
    Write-Host '  ("N served, M freed, K live, ... CATCH(es)"), and on a hit:' -ForegroundColor Cyan
    Write-Host '  "GUARD PAGE - STALE WRITE/READ at X ... RIP R ... allocated from A freed from F".' -ForegroundColor Cyan
    Write-Host '  R is THE ANSWER for the 0x40 surface. Pair with -SentryWriteWatch (0x20).' -ForegroundColor Cyan
    Write-Host '  It writes to the client (an inline hook on the allocator + a HeapFree swap);' -ForegroundColor Cyan
    Write-Host '  it stands down if the allocator prologue does not match or its self-test fails.' -ForegroundColor Cyan
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
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.probe') -Value $Probe -Encoding ascii
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.session') -Value $Session -Encoding ascii
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
