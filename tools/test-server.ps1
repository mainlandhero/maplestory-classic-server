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
    ================== THE TEST PLAN, as of 2026-08-21 ==================

    It lives here rather than in STATUS.md so that the steps and the thing that launches
    them cannot drift apart. Update it in the same commit that changes what it tests.

    -SetFieldProbe is NOT optional. Without it Session::handle returns nothing for EVERY
    packet and the client sits on "Connecting...", which looks exactly like a server that
    is not running. It has cost a launch.

    Run -Stop before relaunching. A running server holds the release binaries and the
    rebuild fails with "Access is denied".

    FIRST, BEFORE THE CLIENT: read the startup banner in the server window.
    ----------------------------------------------------------------------
    Three lines say whether anything below is even being tested:

      maplecw-world: footholds: 94089 segments across 426 maps (...)
      maplecw-world: consumables: 44 items restore something
      maplecw-world: quests: 322 loaded, 5 of them carrying an authored script overlay

    "NONE LOADED" or "no script overlay" means steps 1, 6 and 7 fail for a reason that has
    nothing to do with the client. Regenerate with:
      python tools/dump_portals.py     (footholds)
      python tools/dump_itemdata.py    (consumables)

    CRASH DUMPS ARE ARMED FOR THE FIRST TIME - 2026-08-21
    -----------------------------------------------------
    Every heap-corruption death so far has been undiagnosable, because it is raised at the
    NEXT allocator walk rather than where the damage happened. A full dump is the one
    instrument that fixes that, and it has never actually been captured.

    It was configured but not armed: LocalDumps pointed at the repo's dumps\ correctly, and
    HKLM\...\Windows Error Reporting\Disabled was 1, so WerFault never ran. Five crashes
    since 2026-08-20 23:24 produced nothing. That is now 0.

    SO: IF THE CLIENT DIES, LOOK IN C:\MapleCW\dumps FIRST.
      a ~24 MB MapleStory.exe.<pid>.dmp        -> say so IMMEDIATELY. Copy it somewhere safe
                                                  before the next crash: DumpCount is 2
      the folder is still empty                -> the instrument is STILL not armed, and no
                                                  further run should be spent on the heap
                                                  corruption until it is
    This costs nothing to check and it is worth more than any step below.

    THESE ARE SEPARATE CLAIMS. TEST THEM ONE AT A TIME.
    ---------------------------------------------------
    Everything in the two lists below is WIRED AND UNSEEN - written on 2026-08-21 by six
    agents working in parallel, and none of it has been in front of the client. Each numbered
    step is an independent claim with its own failure signature, so a run that does three
    steps and crashes tells you less than a run that does one and reports it.

    Suggested order if you only have one run: 2 (does it still crash), then 1 (potions),
    then 7 (Roger's whole quest), then 3 (mob damage). Those four answer the most open
    questions per minute, and 2 comes first because a crash ends the run for everything
    after it.

    NEW COMMANDS THIS ROUND: !job <id>, !migsweep [first] [last]. !help lists them all.

    CONFIRMED ON 2026-08-21 - do not re-test these, they are done
    -------------------------------------------------------------
    Equipping and swapping (no crash), ability points in singles AND in bulk, the job
    change with its sound. Those three are off this list for good.

    NEW SINCE THAT RUN - the point of this one
    -----------------------------------------
     0. CREATE A CHARACTER ON THE SECOND LOGIN OF A LAUNCH.
        Enter the world, Log Out back to character select, then click "Create a character".
        The owner, 2026-08-21: *"when I log in, I have full character slots, if I delete one, I
        cannot immediately create another to replace it."*
        THE DELETE WAS A RED HERRING. What that session did was log in TWICE, and the
        client's handshake calls FUN_140c9e8a0 - which stores plaintext 0 into the flag
        gating the button - on every success. Our `create=on` patch latched on a static
        bool and set it once per LAUNCH, so the second login left it at 0. login.log shows
        two connections and two 0x0010s; the hook log showed exactly ONE "called
        FUN_140c9e230". Now it re-arms on every login result.
        The symptom is what a cleared flag predicts: the button draws enabled, because that
        is separate state, and FUN_141177a10 never reaches FUN_141b282d0 - so NO packet is
        sent. Not a refusal notice, not an 0x00A8. Silence.
          creation screen opens                  -> fixed
          nothing, and no 0x00A8 in login.log    -> still gated. The hook log should carry
                                                    "re-armed the create-character flag"
                                                    on the second login; if it does not,
                                                    the patch is not running
          "no room for another character"        -> we DID reach the handler and the slot
                                                    arithmetic is wrong. That one is ours

     1. DRINK A RED POTION. Get hurt first, then double-click it in the Use tab.
          HP goes up by 100, or to full if 100 would overshoot   -> works
          the stack drops 2 -> 1 and the slot does NOT empty     -> mode 1 is right
          a chat line saying it could not be used                -> read it, it says why
          nothing at all                                         -> world.log will have a
                                                                    "<- 0x010E" with no reply
        Then drink the last one: the slot should empty. Then try Roger's Apple (30 HP) if
        you have one.
        NOTE: this was never "the potion did nothing" - the request was arriving and going
        unanswered, and there was exactly ONE of them in the whole run because the client
        latches until the server replies. So test it TWICE: the second drink is the one that
        proves the latch is being cleared.

     2. SERA MUST NOT CRASH THE CLIENT. Go to !map 1, stand there, and wait about 30
        seconds without doing anything.
        Last run the client died 9 seconds after !map 1, on Sera's first idle chat line.
        Cause found and reverted: two bytes of their spawn packet had been swapped on the
        strength of a static read. The bytes are back to the pair that is measured to work.
          they say something and the client lives     -> the revert is right
          the client dies again                       -> the revert was NOT the fix. Say how
                                                         long you were on map 1. world.log
                                                         will end with a 0x0453 and a 0x009E
        This costs nothing but standing still, and it is the highest-value 30 seconds in
        the run.

     3. MOB DAMAGE. Let a snail hit you three or four times on !map 40 and say TWO things:
        the number that floats over your head, and how much the HP bar actually dropped.
        They may now DISAGREE, and that is the measurement.
        The server no longer trusts the client's number. Every one of the twelve captured
        hits said 1; the snail's PADamage is 3 in the client's own data, and 3 appears at no
        offset in any of those bodies, so the client really did compute 1. The server now
        works it out itself and world.log prints both as "for N ... The CLIENT claimed M".
          bar drops 3 or 4, number says 1     -> working, and the mismatch is cosmetic
          bar drops 3 or 4, number agrees     -> better than expected; say so
          bar still drops 1                   -> the override did not fire. world.log will
                                                 say "no template to check it against"
          the number is huge or the bar empties
                                              -> stop and say so

     4. QUEST EXP GOES TO THE CHAT LOG NOW, not the bottom-right. Turn in any quest that
        pays EXP - Heena/Sera's 1001 pays 2 - and say WHERE the line appears.
        Kill EXP is unchanged and still belongs bottom-right, so the two are now different
        on purpose. The client composes the words itself from string 0x00C1,
        "You received EXP (+n)".
          in the chat log      -> working
          still bottom-right   -> the in_chat byte is not taking effect
          in BOTH              -> say so; that would be new
        The COLOUR is chat category 6 and is the client's, not ours. If it is not grey,
        that is worth knowing but it is not a bug in the packet.

     5. ITEM PICK-UPS MUST STAY BOTTOM-RIGHT. Kill a snail, walk over the drop, and check
        the chat log stays clean.
        Expected to already be right and needs no server change: the client's only chat-log
        copy of a pick-up is gated on the map's fieldType being 0x56, and NONE of this
        client's 426 maps has that type. A cheap regression check on a claim that was
        measured rather than tested.

     6. A QUEST'S ITEM REWARD GOES TO THE CHAT LOG, IN GREY. Accept or finish any quest that
        hands an item over - Heena's 1000 gives Sera's Mirror, Roger's 1002 gives the apple.
        Expect a grey line reading "<Item> x<n> earned. (<Tab>)" with the item name as a
        link, in the chat log, and NOT bottom-right.
        This is a different OPCODE from the pick-up line - 0x02D1 effect 8, not 0x0089 - and
        it was found by enumerating all 36 of 0x0089's sub-cases and confirming none of them
        can do it. Category 6's colour constant is 0xFFBBBBBB, grey; category 7, which the
        old chat notice used, is 0xFFFFFF00, yellow.
          grey line in the chat log       -> done
          line in another colour          -> route right, colour is a separate question
          nothing at all                  -> either the item id resolved no name, or the
                                             category is a tab this window does not show
          the client freezes or dies      -> read the bytes in world.log FIRST. Only count 0
                                             sends a short body and the builder refuses it

    STILL UNSEEN FROM THE PREVIOUS ROUND - these never got tested
    ------------------------------------------------------------
     7. ROGER'S WHOLE QUEST, which is four separate things in one conversation.
        a. They must open with "You'll die when your HP reaches 0..." and a Next button.
           "Hey, nice weather, isn't it?" is the exact signature of the overlay not loading.
        b. The second box must be ACCEPT / DECLINE, not OK.
        c. On Accept: your HP drops to 25/50 AND a Roger's Apple lands in the USE tab.
           LAST RUN THE DIALOGUE WAS RIGHT AND THE ACCEPT WAS DROPPED. The client answers a
           yes/no box with SIX bytes - handle, type, action - and the parser read a u32 echo
           and a string that are not there, so parse_script_reply returned None and
           on_script_reply turned that into silence. No HP change, no apple, and clicking
           Roger again just replayed the opening.
        d. EAT THE APPLE. The quest completes on the apple being CONSUMED, not on clicking
           Roger again - so expect the QuestClear fanfare with no second conversation.
           It heals 30, which from 25/50 caps at full.
        Say which of a/b/c/d worked; they fail independently.

     8. QUEST FORFEIT. Accept 1000 from Heena, take the mirror to Sera so 1001 STARTS, then
        open the quest window and press give up on Sera's Mirror. Then click Sera again.
        TEST 1001, NOT 1000 - 1000 is completed by then and the client will not even build
        a forfeit packet for a completed quest, so it can only ever look broken.
     9. Turn in ANY quest and LISTEN. A fanfare should play with NOTHING DRAWN - that node
        was cut from this client's WZ, so sound-and-no-picture is the EXPECTED result. If
        something IS drawn, say so; the analysis needs correcting.
     10. Kill a mob ON A SLOPE OR A STEP, not on flat ground - flat looks identical before
        and after, which is why this went unnoticed. Every drop must be walkable-over.
     11. Kill snails until you have Etc items and mesos, then log out and back in. THE ETC
        ITEMS AND THE MESO COUNT MUST STILL BE THERE.
     12. Pick up several Garnet Ores. ONE slot with a count, not three slots.
    13. Stand still 10s, then 20s. +10 HP and +10 MP every 10 seconds, stopping when full.
    14. Kill the Tutorial Jr. Sentinel. Always a Shellpiece, never mesos, never anything
        else.
    15. !setrates 2 3 5 -> one banner naming all three. !rates reads them back.
        !setrates 1 1 1 -> three "rate-up event has ended" lines on one banner.

    REGRESSION GLANCES - seconds each, not exercises
    -----------------------------------------------
    16. Drops arc out of the corpse over about half a second, at the mob, spread apart.
    17. The EXP line bottom-right is WHITE.
    18. Mobs on !map 40 are already standing there - no fade-in.
    19. A level-up gives +16 max HP and +12 max MP.
        (NPCs fading in is NOT closed after all - see the !npcecho step below.)

    FREE, IF YOU ARE ON MAP 40 ANYWAY
    ---------------------------------
    20. Swing at snails until about 40 hits land, then say the LOWEST and the HIGHEST.
        Character 206 now has STR 30 and the 1312000 axe (incWAT 17), so the predicted
        window is 16..27. It was 15..20 at STR 5 - note how little the BOTTOM moved. That
        is the answer to "adding stats does nothing": without a mastery skill the mastery
        term is 0.08, so the primary stat contributes a twelfth of its weight to the
        minimum and its full weight to the maximum. Anything above 27 or below 16
        falsifies the model, and which end it misses says which term is wrong.
        Costs no server change.

    THE NPC FADE - ONE COMMAND, AND IT ANSWERS THREE THINGS
    -------------------------------------------------------
    21. !map 1, WATCH HEENA AND SERA FADE IN, then type !npcecho and watch the copies.
        A second Heena and a second Sera appear about 70 px to the right, created by the
        OTHER packet the NPC pool accepts. Say whether the copies POP or FADE.

        Why this exists: two static passes concluded the server could not fix this, and
        both were answering a narrower question than the one asked - they enumerated the
        FIELDS OF 0x044F and correctly found no alpha, no visibility timer, no appear type.
        Neither asked what OTHER packets the pool accepts. There are two that create an NPC:

          0x044F  NpcEnterField        or  [obj+0x38], 1   then the 20-field body
          0x0451  NpcChangeController  mov byte [obj+0x38], 2   then the IDENTICAL body

        And the case that works uses the second one: every mob is sent 0x03C6 AND 0x03D2,
        and mobs are instant. NPCs have only ever been sent 0x044F.

          the copies POP in solid      -> 0x0451 is the fix; field entry switches to it
          the copies FADE too          -> the creation route is not the difference, AND the
                                          timing theory dies with it, because these arrive
                                          minutes after field entry
          nothing appears              -> the flag or the body is wrong, not the theory.
                                          world.log names every packet sent
          the copies stay SEE-THROUGH  -> it was never a fade-in at all

        Whichever happens, this is the one question of the run that costs nothing to ask.

    LAST, OR NOT AT ALL
    -------------------
    22. CLICK CHANGE CHANNEL. It either works or ends the session, so nothing can follow it.
        The button now sends the migrate sweep itself. It used to answer 0x00D2 with 0x0011,
        a LOGIN-stage opcode below the channel switch's 0x70 floor - undispatchable on a
        channel connection. The owner, 2026-08-21: "the transfer did not go through, but I lost
        all ability to attack once the attempt was made." Both halves are that one fact:
        0x00D2 latches on send, and only an inbound handler clears the latch, so a reply the
        client cannot dispatch strands the character mid-migration.
        The body is fully measured; the OPCODE is the one field that cannot be read
        statically, so ten candidates go out and the hook log names the one that dispatched.
          the channel changes    -> found it; wire that opcode and delete the sweep
          nothing at all         -> outside 0x19..0x22. Try !migsweep 24 33
          the client dies        -> say so; the 64 bytes of padding exist to prevent it
        !migsweep [first] [last] still exists for a different range.

    STILL OPEN - do not spend the run confirming these are broken
    ------------------------------------------------------------
      - Quest ITEM rewards in the chat log. The wording is right and the destination is not
        reachable through any decoded packet; 28 of the 36 0x0089 sub-cases are unread.
      - The classic shop counter. 0x055D is the opcode; the row structure is not decoded.
      - The blue HP/MP recovery number. Packet not found.
      - The other script quests. 1002 and four closes are authored; the rest are not.
      - There is NO EXP-gain sound in this client. IncEXP and questCount are in
        Sound/Game.img and neither name appears anywhere in the executable.
      - Outgoing damage validation. The formula is decoded but the 0x00DF header does not
        carry the action or the skill id, so nothing can be checked against it yet.

    IF THE CLIENT DIES
    ------------------
    CHECK dumps\ FIRST - see the top of this plan. Then: do not lose the logs. previous-runs/ is a rolling buffer; copy anything that settles a
    question into research/fixtures/ under a name that says what it proves. Say roughly how
    long you were in and what you were doing - for the heap corruption that is the variable
    the logs cannot supply. Dumps land in dumps/ if WER LocalDumps is still configured.

    THE FREE MEASUREMENT NOBODY HAS TAKEN
    -------------------------------------
    Every -SetFieldProbe run dumps the client's own EXP curve on the positive control's
    first hit:  python tools/decode_dump.py --exp-curve
    Compare it against data/exp-curve.txt. If they disagree, the client wins.

    THE CLIENT PATCHES ARE STILL PATCHES. Nothing here makes the session valid:

      1415db360:ret     skip the server-reachability check. NOT OPTIONAL: without it the
                        client __fastfails after ~37 seconds, because that check overruns
                        its own stack buffer when every address is unreachable. A client
                        bug, on a path that only ever runs because we are firewalled.
      141b2a280:rdx=0   suppress the "trouble logging in" dialog, which otherwise blocks
                        the per-frame tick that enables the Login button.
      mode=2            leave the client's mode-5 auto-login so the button gets a turn.
      create=on         set the protected flag that gates "Create a character". The real
                        service sets this from virtualised code and we have not found the
                        packet that does it.

    The last two are standing in for protocol we have not implemented. See
    docs/launcher.md for which patches retire and when.

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
    # The account every connection is served as. The game socket carries no credentials,
    # so this is not a login - it decides whose characters appear. Create it first with
    #   .\target\release\maplecw-useradd.exe <name>
    [string]$Account = 'maplecw',
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
        # 141c532ab:peek=24 - rcx is the CInPacket and +0x24 is its read cursor, inside the
        #   mob's encodeInit. Mobs render now, so this is a regression check rather than a
        #   diagnosis: the cursor should be consistent across every mob in a field.
        # 140304100:hits=200:dump=143AC2400/968 - the equip decode at world entry. POSITIVE CONTROL: no lines
        #   at all means the hook never armed and the log proves nothing.
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,141c532ab:peek=24:hits=20,140304100:hits=200:dump=143AC2400/968'
    }
    # Announce which pair actually got armed. The old line said "mobs" for 141c532ab, which
    # is the mob SPAWN decoder - now that -MobTargets arms a mob TARGETING watch, one word
    # would have covered two different runs. Same precedence as the if/elseif above, and
    # written as three statements because 5.1 has no ternary.
    $pair = "mob spawn (141c532ab)"
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
    & cargo build --release -p login -p world -p store -p grap-stub
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
function Save-PreviousLog([string]$Path) {
    if (-not (Test-Path $Path)) { return }
    $dir = Split-Path -Parent $Path
    $prev = Join-Path $dir 'previous-runs'
    if (-not (Test-Path $prev)) { New-Item -ItemType Directory -Path $prev | Out-Null }
    $stamp = (Get-Item $Path).LastWriteTime.ToString('yyyyMMdd-HHmmss')
    $name = [IO.Path]::GetFileNameWithoutExtension($Path)
    $ext = [IO.Path]::GetExtension($Path)
    Move-Item $Path (Join-Path $prev ("{0}-{1}{2}" -f $name, $stamp, $ext)) -Force
}

Save-PreviousLog $serverLog
Remove-Item $serverLog -Force -ErrorAction SilentlyContinue
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

# The hook is switched on by marker files, because ShellExecute does not carry $env: into
# the child - a launcher will replace all of this with one config file (docs/launcher.md).
$hookLog = Join-Path $ClientDir 'maplecw-hook.log'
New-Item -ItemType File -Path (Join-Path $ClientDir 'maplecw-hook.enable') -Force | Out-Null
Remove-Item $hookLog -Force -ErrorAction SilentlyContinue
$env:MAPLECW_HOOK_LOG = $hookLog
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.probe') -Value $Probe -Encoding ascii
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
    Write-Host "CRASH DUMPS ARE ARMED FOR THE FIRST TIME." -ForegroundColor Cyan
    Write-Host "  Every heap-corruption death so far has been undiagnosable: it is raised"
    Write-Host "  at the NEXT allocator walk, not where the damage happened. A full dump is"
    Write-Host "  the one instrument that fixes that, and none has ever been captured."
    Write-Host "  LocalDumps was configured correctly the whole time and WER itself was"
    Write-Host "  switched off, so five crashes since 2026-08-20 produced nothing."
    Write-Host ""
    Write-Host "  IF THE CLIENT DIES, LOOK HERE FIRST:" -ForegroundColor Yellow
    Write-Host "    $root\dumps"
    Write-Host "    a ~24 MB MapleStory.exe.<pid>.dmp -> SAY SO. Copy it out; only 2 are kept"
    Write-Host "    still empty                       -> the instrument is STILL not armed,"
    Write-Host "                                         and no further run should be spent"
    Write-Host "                                         on the heap corruption until it is"
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
    Write-Host '  22 separate claims. TEST THEM ONE AT A TIME and say which you did.' -ForegroundColor Yellow
    Write-Host '  A run that does three steps and crashes tells us less than one that'
    Write-Host '  does a single step and reports it. Full text: Get-Help on this script.'
    Write-Host ''
    Write-Host '  If you only have one run: 0, then 2, then 1, then 7.' -ForegroundColor Cyan
    Write-Host '  2 goes first because a crash ends the run for everything after it.'
    Write-Host ''
    Write-Host '  DONE - do not re-test: equipping and swapping, ability points in'
    Write-Host '  singles and in bulk, the job change with its sound.'
    Write-Host ''
    Write-Host '  --- the point of this run -------------------------------------------'
    Write-Host '  0. LOG IN TWICE, THEN CLICK CREATE A CHARACTER.' -ForegroundColor Cyan
    Write-Host '     Enter the world, Log Out back to character select, then click Create.'
    Write-Host '     It used to do NOTHING on the second login - no packet at all - and the'
    Write-Host '     delete was a red herring: the handshake zeroes the create flag on every'
    Write-Host '     success and our patch only set it once per launch.'
    Write-Host '       creation screen opens          -> fixed'
    Write-Host '       nothing, and no 0x00A8 in login.log'
    Write-Host '                                      -> still gated. Check the hook log for'
    Write-Host '                                         "re-armed the create-character flag"'
    Write-Host ''
    Write-Host '  1. DRINK A RED POTION, twice. Get hurt first.' -ForegroundColor Cyan
    Write-Host '     +100 HP or to full; stack 2->1 WITHOUT the slot emptying.'
    Write-Host '     The second drink is the real test: one unanswered request used to'
    Write-Host '     latch the client and block every later use.'
    Write-Host ''
    Write-Host '  2. !map 1, THEN STAND STILL FOR 30 SECONDS.' -ForegroundColor Cyan
    Write-Host '     Last run the client died 9s in, on Sera''s first idle line, from two'
    Write-Host '     bytes of their spawn packet I had swapped. Reverted.'
    Write-Host '       they speak and it lives -> the revert is right'
    Write-Host '       it dies again           -> say how long you were on map 1'
    Write-Host ''
    Write-Host '  3. LET A SNAIL HIT YOU 3-4 TIMES. Report TWO numbers:'
    Write-Host '     the number over your head, and how much the bar dropped.'
    Write-Host '     They may DISAGREE now - that is the measurement. The server no'
    Write-Host '     longer trusts the client, which was claiming 1 for a PADamage-3 mob.'
    Write-Host ''
    Write-Host '  4. QUEST EXP NOW GOES TO THE CHAT LOG, not bottom-right. Kill EXP is'
    Write-Host '     unchanged and still belongs bottom-right. Category 6 is grey.'
    Write-Host ''
    Write-Host '  5. ITEM PICK-UPS MUST STAY OUT OF THE CHAT LOG. Expected already right.'
    Write-Host ''
    Write-Host '  6. A QUEST ITEM REWARD -> grey chat line, "<Item> x<n> earned. (<Tab>)".'
    Write-Host '     Different opcode from the pick-up line. Item name should be a link.'
    Write-Host ''
    Write-Host '  7. ROGER, on Maple Island - four things that fail independently:' -ForegroundColor Cyan
    Write-Host '     a. opens with "You will die when your HP reaches 0..." + Next'
    Write-Host '        ("Hey, nice weather" = the overlay did not load)'
    Write-Host '     b. second box is ACCEPT/DECLINE, not OK'
    Write-Host '     c. on Accept: HP drops to 25/50 AND an apple lands in the USE tab'
    Write-Host '        (last run the dialogue was RIGHT and the Accept was dropped: a'
    Write-Host '         yes/no reply is 6 bytes and the parser read an echo that is not'
    Write-Host '         there, so nothing happened at all)'
    Write-Host '     d. EAT THE APPLE -> the quest completes. No second conversation.'
    Write-Host ''
    Write-Host '  8. QUEST FORFEIT. Start 1001 (Heena -> Sera), give it up in the quest'
    Write-Host '     window, then click Sera again. TEST 1001, NOT 1000 - the client will'
    Write-Host '     not even build a forfeit packet for a completed quest.'
    Write-Host ''
    Write-Host '  9. TURN IN ANY QUEST AND LISTEN. A fanfare with NOTHING DRAWN is the'
    Write-Host '     EXPECTED result - that art was cut from this client.'
    Write-Host ''
    Write-Host '  10. KILL A MOB ON A SLOPE OR STEP, not flat ground. Flat looks the same'
    Write-Host '      before and after, which is why this went unnoticed. Drops must be'
    Write-Host '      walkable-over.'
    Write-Host ''
    Write-Host '  11-15. Relog keeps Etc items and mesos; Garnet Ores stack; idle regen'
    Write-Host '      +10/+10 per 10s; Jr. Sentinel drops only a Shellpiece; !setrates.'
    Write-Host ''
    Write-Host '  16-19. Glances: drops arc from the corpse; kill EXP line is WHITE; mobs'
    Write-Host '      on !map 40 already standing; level-up +16 HP / +12 MP.'
    Write-Host ''
    Write-Host '  20. FREE, if you are on map 40: ~40 snail hits, report the LOWEST and'
    Write-Host '      HIGHEST. Predicted 16..27 at STR 30 (it was 15..20 at STR 5 - note'
    Write-Host '      how little the BOTTOM moves; that is the mastery term).'
    Write-Host ''
    Write-Host '  21. !map 1, WATCH HEENA AND SERA FADE IN, then type !npcecho.' -ForegroundColor Cyan
    Write-Host '      Copies appear ~70px right, created by the OTHER packet the NPC pool'
    Write-Host '      accepts - the one mobs get and NPCs never have. DO THEY POP OR FADE?'
    Write-Host '        pop solid -> that packet is the fix; field entry switches to it'
    Write-Host '        fade too  -> the route is not the difference, and the timing'
    Write-Host '                     theory dies with it: these arrive long after entry'
    Write-Host '        nothing   -> flag or body wrong, not the theory'
    Write-Host ''
    Write-Host '  22. LAST, OR NOT AT ALL: CLICK CHANGE CHANNEL. It either works or ends' -ForegroundColor Red
    Write-Host '      the session, so nothing can follow it.' -ForegroundColor Red
    Write-Host '      The button now sends the migrate SWEEP itself - it used to answer' -ForegroundColor Red
    Write-Host '      with 0x0011, a LOGIN-stage opcode a channel connection cannot even' -ForegroundColor Red
    Write-Host '      dispatch. That is why the last attempt did nothing AND left you' -ForegroundColor Red
    Write-Host '      unable to attack: 0x00D2 latches on send and only a reply clears it.' -ForegroundColor Red
    Write-Host '      The winning opcode is the last one in the hook log before the close.' -ForegroundColor Red
    Write-Host '      !migsweep [first] [last] is still there for a different range.' -ForegroundColor Red
    Write-Host ''
    Write-Host '  NEW COMMANDS: !job <id>, !npcecho [dx], !migsweep [first] [last].'
    Write-Host '  !help lists them all.'
} else {
    Write-Host '  1. click Login. Any character created in an EARLIER run should be there.'
    Write-Host '  2. create one. Check the name first - a name already used is now refused'
    Write-Host '     by the server rather than always accepted.'
    Write-Host '  3. close the client, run this script again, and click Login. The character'
    Write-Host '     should still be listed. That is the whole point of this run.'
}
Write-Host ''
Write-Host 'Logs:'
Write-Host "  $serverLog                 every packet both ways, and what each reply was"
Write-Host "  $hookLog   client patches and faults"
Write-Host "  $exitLog          how the client died; 0 is a hand-close"
Write-Host ''
Write-Host "Then: powershell -ExecutionPolicy Bypass -File `"$PSCommandPath`" -Stop"
