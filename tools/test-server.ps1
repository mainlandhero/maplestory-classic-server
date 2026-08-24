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
    ================== THE TEST PLAN, as of 2026-08-21 night ==================

    TWO copies in this file - this one and the Write-Host block near the bottom that
    actually gets printed. Update both, then RENDER the second one and read it.

    -SetFieldProbe is NOT optional: without it Session::handle returns nothing for EVERY
    packet and the client sits on "Connecting...". Run -Stop before relaunching.

    WHAT THE LAST RUN CLOSED - none of this needs testing again
    ----------------------------------------------------------
    Create on the second login. Consumables and their cap (a 100 HP potion healing 11 into
    a bar 11 short is the cap WORKING). Sera's idle chatter. The mob-damage override. The
    damage model, now at a second stat point - 14 swings at STR 35 came in at 17..31 where
    the model spans 15.5..32.6. And the two big ones:

      THE CHANNEL CHANGE WORKS. Inventory and mesos carried over, and world-ch1.log shows
      the migration claimed BY CHANNEL with a real SetField rather than the minimal one.

      THE HEAP DEATH IS NOT HEAP CORRUPTION, and there are now TWO dumps that agree.
      Both were written by the hook and read with tools/dumpwalk.py - no debugger, and
      the tool self-checks against the dump before it prints anything. In both the heap
      chain is intact and the address the allocator complained about lands inside a
      LIVE, BUSY ~264 KB block: RtlFreeHeap REFUSED a bad free rather than discovering
      damage. The STACK IS THE SAME FRAME FOR FRAME in both - the client's own free()
      called from a PCOM.dll refcounted release, under oleaut32!VariantClear, under
      NAMESPACE.DLL - which is corroboration one dump could not give.
      What the second dump CONFIRMED: the first found a block header reading
      0x0000000100000020 where 0x20 was expected and flagged 'a stray 1' as its single
      inference. It generalises exactly - EVERY damaged header in both processes is that
      same value, five instances, and all five are in the 0x20 size class.
      I briefly wrote the opposite here, from one line of tool output: the failure
      record's Address field is the ENTRY for a type-8 failure and the CALLER'S POINTER
      for a type-9, and decoding a pointer as an entry always prints a plausible header.
      The 'garbled bytes' were a BSTR - length prefix 0x10, then "Pr", i.e. "Property".
      tools/dumpwalk.py now refuses that decode unless the type says it is an entry, and
      tools/poolchain.py enumerates the pool exactly.
      research/heap-corruption-dump.md and research/fixtures/heap-second-dump-*.log.
      Page heap would NOT have helped either way: it guards Windows heap blocks, and
      this is a slot inside a client-allocator arena it cannot see into.

    THE NPC FADE - THERE IS A LEVER AFTER ALL, AND IT IS STEP 4 BELOW. The echo could never
    have answered it: 0x044F and 0x0451 run the SAME decoder body, so comparing them was
    comparing a thing with itself. Inside that shared body is a block gated on a global that
    allocates a 0x90-byte object per NPC, stamps it with a clock value and starts it - and
    0x0452 is the packet that sets the global. Its two branches identify the object: v=0
    rebuilds that same object on every NPC, v!=0 tears it down. !npcfx off, then !npcecho.

    THE POINT OF THIS RUN - four steps, and STEP 1 MUST BE FIRST
    -----------------------------------
     1. TYPE  !map 10001050  AS THE FIRST THING AFTER YOU LOG IN. Ten seconds of work, and
        doing it later destroys it.
        That teleport crashed the client last run, 328 ms into the map load. It is a DIFFERENT
        fault from the one we have three dumps of: an access violation reading [0 + 0x3530] -
        a null object pointer - where the others were the allocator refusing a bad free. The
        damaged pool slot was present in this dump too and was never touched, so whatever
        accumulates is not what nulled this field.
        Two readings, and this is the same experiment that answered the GoodTest question
        yesterday - which came back the OTHER way, so it is worth running rather than assuming:
          (a) map 10001050 (Henesys Park) is fatal to load. It is the only map of its group
              this client has ever been sent to.
          (b) it was the FOURTH map load of a 389-second session, and something accumulates.
        At ~40 s of client life:
          it dies again -> (a), the map. Relaunch and do steps 2-4; the crash is then worth
                           one Ghidra pass and we know exactly where to point it
          it loads      -> (b). Carry straight on with steps 2-4 in the same session
        Then, either way, try  !map 10001000  - Henesys town, the map next door, also never
        loaded. Park dies and town loads -> that one map. Both die -> that part of the world.

     2. ORGANIZE ITEM NOW ORGANISES. You hit it three times and nothing happened, because
        mode 6 answered with the unchanged box and the log line called that "a legal no-op".
        Legal it was; a no-op is not what the button says it does.
        It repacks to slots 1..n, grouped by tab and then by item id. Put three or four things
        in the box, take one out from the middle to leave a hole, then hit Organize.
          the gap closes and the items group by tab -> done
          nothing moves -> the sort ran on the database and the client is not redrawing from
                       mode 15. Mode 19 preserves the scroll instead of recalculating it, and
                       that is the next thing to try
          the order looks arbitrary -> tell me what order you expected. Equips first by id,
                       then Use, then Etc is what this does
          items VANISH -> stop and say so immediately. The sort re-reads the box and refuses
                       to commit if the count changed, so this should be impossible
        HIT IT THREE TIMES AGAIN. It is a pure function of the contents, so clicks two and
        three must change nothing; anything that keeps shuffling is a different bug.

     3. THE CASH SHOP: CLICK IT THREE TIMES AND COUNT THEM IN THE LOG.
        YOUR THREE-CLICK RUN OVERTURNED MY OWN FINDING, so thank you for doing it exactly as
        described. THE BUTTON WAS SENDING ALL ALONG. The peek read [ctx+0x2330] as 0, then 1,
        then 1 - it fires once, sets the exclusive-request latch, and waits for a reply. And
        0x00D5 was in world.log at the same millisecond as your first click, and in the two
        sessions before that.
        I had reported three times that no packet was sent. That was wrong, and the reason is
        worth one sentence: 0x00D5 arrives inside a burst with 0x0420..0x0426 that lands near
        the end of a session, and I filed the whole burst as "shutdown telemetry" without ever
        separating the opcodes in it. One grep of the archives: 0x0420 appears in five runs
        where nobody touched the button, 0x00D5 appears in exactly the two runs with a click.
        Your very first message said "the opcode is most likely not handled". It was.

        The server now answers 0x00D5 with the 0x0070 that clears the latch. THIS IS NOT A
        CASH SHOP - there is no cash shop server and no window will open. What should change
        is that the button stops being a once-per-session button.
          click Cash Shop THREE times, a couple of seconds apart, then quit. Then count:
            powershell -NoProfile -Command "(Select-String -Path 'C:\MapleCW\world.log' -Pattern '<- 0x00D5').Count"
          THREE -> the latch is being cleared and the fix works
          ONE   -> 0x0070 does not clear this latch. Not a disaster: the watch will show
                   [ctx+0x2330] going 0,1,1 again, and the next candidate is the real refusal
                   packet rather than a borrowed one
          You should also see a chat line: "The Cash Shop is not available on this server."
        AND THE PICK-UP MATTERS HERE. That latch is not the Cash Shop's own field - it gates
        the pick-up sweep too - so after clicking Cash Shop, kill something and walk over the
        drop. Picking up fine after three clicks is the second half of the same measurement.

     4. GLANCE, NO SETUP: does a grey "<item> x<n> earned." line appear in the SCREEN MESSAGE
        AREA - the strip above the chat box - when you pick something up? It has gone out on
        every successful pick-up for two runs and nobody has said what it looks like.

    OPTIONAL, AND NOT ON THE SAME RUN AS STEP 1: -HeapFix
    -----------------------------------
    Three bytes at 14019b504 in the mapped image. Nothing in client-patched\ changes on disk.
    The client's free reads the whole 64-bit pool slot header where only the low half is ever
    legal; a stray 1 in the high dword therefore sends a pooled 0x20 slot to HeapFree, and
    Windows kills the process. Reading 32 bits returns it to the correct free list.
    SEVEN damaged slots across four dumps now, every one the identical 0x0000000100000020,
    every one in the 0x20 class, accumulating at about one per 250 s.
    research/heap-third-dump.md, and research/henesys-park-null-deref.md for why last run's
    crash was NOT this one.
      the client stops dying with 0xC0000374  -> the whole chain is confirmed end to end
      it dies anyway                          -> something in that chain is wrong, and the
                                                 dump says which half. That is worth more
    One variable at a time: step 1 is the one that matters, so run it unpatched.

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
      - The classic shop counter. DECODED now, including the price (row+0x38), and
        deliberately not built - three of its fields fail silently or desynchronise the
        stream if they are wrong, and this packet has killed the client twice.
      - Outgoing damage validation. The formula is decoded but the 0x00DF header does not
        carry the action or the skill id, so nothing can be checked against it yet.
      - Page heap is OFF, so !heap -p -a has no allocation stacks to print. That is an IFEO
        setting and it is the owner's to turn on.
      - There is NO EXP-gain sound in this client.
      - Two refusal paths still answer 0x00D2 with 0x0011, which a channel socket cannot
        dispatch. Nothing decoded can.

    COMMANDS: !map, !item, !exp, !heal, !job, !npcecho, !npcfx, !migsweep, !exprate, !mesorate,
    !droprate, !setrates, !rates. !help lists them all.

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
      heapfix=on        ONLY with -HeapFix, off by default. Three bytes at 14019b504 so a
                        damaged pool header goes back to the free list instead of to
                        HeapFree. research/heap-third-dump.md section 5 states the risk.

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
    # OFF by default, and it is an EXPERIMENT rather than a fix.
    #
    # Three bytes at 14019b504 in the mapped image: the client's free reads the 64-bit pool
    # slot header where only the low half is ever legal, so a stray 1 in the high dword sends
    # a pooled 0x20 slot to HeapFree, and Windows kills the process. Reading 32 bits instead
    # returns it to the right free list - the correct outcome, not a suppression.
    #
    # Six damaged slots across three dumps, all the identical value, all in the 0x20 class,
    # accumulating at about one per 250 s. research/heap-third-dump.md.
    #
    # Nothing in client-patched\ changes on disk. The patch verifies the three bytes before
    # writing, reads them back after, and logs both. If the client still dies with
    # 0xC0000374 with this on, research/heap-wild-write.md is wrong somewhere - which is
    # exactly what makes it worth running.
    [switch]$HeapFix,
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
        # THE CASH SHOP IS ANSWERED NOW, AND THE WATCH IS THE FALSIFIER.
        #
        # ANSWERED 2026-08-22 with a peek across three clicks 5.6 s apart: [ctx+0x2330] read
        # 0, then 1, then 1. The button fires ONCE, sets the exclusive-request latch, and
        # waits - and 0x00D5 was in world.log at the same millisecond as click 1, and in the
        # two sessions before it. The "no packet was sent" finding was WRONG for two days.
        #
        # The server now answers 0x00D5 with the 0x0070 that clears ctx+0x2330. The watch
        # stays exactly as it was, because it is what falsifies the fix:
        #   [ctx+0x2330] reads 0 on EVERY click -> the latch is being cleared. Done
        #   0 then 1 then 1 again              -> 0x0070 does not clear THIS latch, and the
        #                                         next candidate is the real refusal packet
        #
        # 140304100:hits=200:dump=143AC2400/968 - the equip decode at world entry. POSITIVE CONTROL: no lines
        #   at all means the hook never armed and the log proves nothing.
        $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,142caee70:peek=2330:hits=60,140304100:hits=200:dump=143AC2400/968'
    }
    # Announce which pair actually got armed. The old line said "mobs" for 141c532ab, which
    # is the mob SPAWN decoder - now that -MobTargets arms a mob TARGETING watch, one word
    # would have covered two different runs. Same precedence as the if/elseif above, and
    # written as three statements because 5.1 has no ternary.
    $pair = "THE CASH SHOP SENDER (142caee70, peeking the exclusive-request latch at +0x2330)"
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
    Write-Host '  4 steps. STEP 1 MUST BE FIRST - the order is the experiment.' -ForegroundColor Yellow
    Write-Host '  Full text: Get-Help on this script.'
    Write-Host ''
    Write-Host '  CONFIRMED, DO NOT RE-TEST.' -ForegroundColor Green
    Write-Host '  CHANNEL CHANGE WORKS - claimed by channel, real SetField, inventory'
    Write-Host '  and mesos carried over. And THE HEAP DEATH IS NOT HEAP CORRUPTION -'
    Write-Host '  THREE dumps of it. RtlFreeHeap REFUSED a bad free every time; the'
    Write-Host '  heap chain is intact. SEVEN damaged slots over four dumps, every one'
    Write-Host '  the identical 0x0000000100000020, every one in the 0x20 class,'
    Write-Host '  against 0 of 472760 slots in the other three. They accumulate at'
    Write-Host '  about ONE PER 250 SECONDS - a rate, which says the writer fires on'
    Write-Host '  something repeated. THERE ARE NOW TWO CRASH FAMILIES: last run''s'
    Write-Host '  was an ACCESS VIOLATION, a null read, and the damaged slot sat'
    Write-Host '  there untouched. Step 1 is about that one.'
    Write-Host '  DEATH AND REVIVE WORK, first time out - the dialog appeared for a'
    Write-Host '  character who logged in ALREADY DEAD, and the revive warped them to'
    Write-Host '  Lith Harbor at 50 HP with no exp penalty at level 10. They then went'
    Write-Host '  on to reach level 11, which answers the half most likely to fail:'
    Write-Host '  they could move and attack afterwards.'
    Write-Host '  BULK SKILL POINTS work. FRESH SPAWNS ARE SPREAD OUT. INVENTORIES'
    Write-Host '  ARE FIXED - a full Equip tab no longer stops Use, Etc or mesos being'
    Write-Host '  picked up. THE BLUE RECOVERY NUMBER DRAWS: "+10 in blue above the'
    Write-Host '  character", so 0x02D1 effect 0x41 is settled. And MAP 10 IS NOT'
    Write-Host '  FATAL - GoodTest logged in there first thing and was fine, which'
    Write-Host '  answered yesterday''s experiment in one login.'
    Write-Host '  BUFFS ARE DONE, BOTH WAYS. Nimble Feet grants and the right-click'
    Write-Host '  cancels it - one 0x013F, one 0x007E, no retry loop. That settled'
    Write-Host '  0x007D, the 124-byte mask, bit 92 = Speed, the i16 value width and'
    Write-Host '  milliseconds, none of which was readable statically, plus 0x013F''s'
    Write-Host '  layout. THREE SNAILS WORKS and deals damage.'
    Write-Host '  STORAGE IS DONE except Organize: the window, 30 slots, mesos both'
    Write-Host '  ways, items in and out, and the 100 meso fee - ten deposits in one'
    Write-Host '  session, all charged.'
    Write-Host '  Also closed: create on second login, consumables and their cap,'
    Write-Host '  Sera''s chatter, the damage model at STR 35, quest EXP in the chat'
    Write-Host '  log, the quest fanfare, and two NEGATIVES worth as much: the blue'
    Write-Host '  number does NOT come from the 0x007C trailer (it was sent, three'
    Write-Host '  times, and drew nothing), and the floating damage number is a STUB'
    Write-Host '  - a red snail doing 10 still shows 1, so no server change reaches'
    Write-Host '  it.'
    Write-Host '  And the NPC fade is not a fade: the appear-effect object was ruled'
    Write-Host '  out by !npcfx off, and it is not see-through, so what is left is a'
    Write-Host '  late first draw, and every server-side cause is now eliminated.'
    Write-Host ''
    Write-Host '  --- the point of this run -------------------------------------------'
    Write-Host '  1. TYPE  !map 10001050  AS THE FIRST THING AFTER LOGIN.' -ForegroundColor Cyan
    Write-Host '     Ten seconds of work, and doing it later destroys it.'
    Write-Host '     That teleport crashed the client last run, 328ms into the map'
    Write-Host '     load - and it is a DIFFERENT fault from the three heap dumps:'
    Write-Host '     an access violation reading [0 + 0x3530], a NULL pointer. The'
    Write-Host '     damaged pool slot was there too and was never touched.'
    Write-Host '       (a) map 10001050 is fatal to load - never tried before'
    Write-Host '       (b) it was the 4th map load of a 389s session'
    Write-Host '     At ~40s of client life:'
    Write-Host '       it dies again -> (a). Relaunch, do steps 2-4, and the crash is'
    Write-Host '                        worth one Ghidra pass at a known address'
    Write-Host '       it loads      -> (b). Carry on with 2-4 in the same session'
    Write-Host '     Then either way try  !map 10001000  - Henesys town, next door,'
    Write-Host '     also never loaded. Park dies + town loads -> that ONE map.'
    Write-Host ''
    Write-Host '  2. ORGANIZE ITEM NOW ORGANISES.' -ForegroundColor Cyan
    Write-Host '     You hit it three times and nothing happened: mode 6 answered'
    Write-Host '     with the unchanged box and called that "a legal no-op". Legal,'
    Write-Host '     but not what the button says it does.'
    Write-Host '     It repacks to 1..n, grouped by tab then item id. Put a few'
    Write-Host '     things in, take one from the middle to leave a hole, Organize.'
    Write-Host '       the gap closes, items group by tab -> done'
    Write-Host '       nothing moves   -> the DB sorted and the client is not'
    Write-Host '                          redrawing from mode 15. Mode 19 next'
    Write-Host '       order looks odd -> say what you expected. This does equips'
    Write-Host '                          by id, then Use, then Etc'
    Write-Host '       items VANISH    -> stop and say so. The sort re-reads and'
    Write-Host '                          refuses to commit if the count changed'
    Write-Host '     HIT IT THREE TIMES AGAIN - clicks 2 and 3 must change nothing.'
    Write-Host ''
    Write-Host '  3. CASH SHOP: CLICK IT THREE TIMES AND COUNT THEM.' -ForegroundColor Cyan
    Write-Host '     YOUR THREE-CLICK RUN OVERTURNED MY OWN FINDING. The button was'
    Write-Host '     SENDING ALL ALONG: the peek read [ctx+0x2330] as 0, then 1, then'
    Write-Host '     1 - it fires once, latches, and waits. 0x00D5 was in world.log at'
    Write-Host '     the same millisecond as click 1, and in the two runs before.'
    Write-Host '     I reported "no packet was sent" three times and it was wrong:'
    Write-Host '     0x00D5 rides in a burst with 0x0420..0x0426 near the end of a'
    Write-Host '     session and I filed the whole burst as telemetry without ever'
    Write-Host '     splitting it. Your first message said "the opcode is most likely'
    Write-Host '     not handled". It was.'
    Write-Host '     The server now answers 0x00D5 with the 0x0070 that clears the'
    Write-Host '     latch. THIS IS NOT A CASH SHOP - no window will open. What should'
    Write-Host '     change is that the button stops being once-per-session.'
    Write-Host '       click it THREE times a couple of seconds apart, then quit and'
    Write-Host '       count the requests with the one-liner under Logs: below.'
    Write-Host '         THREE -> the latch clears and the fix works'
    Write-Host '         ONE   -> 0x0070 does not clear this latch; the watch will'
    Write-Host '                  show 0,1,1 again and names the next candidate'
    Write-Host '       expect a chat line: "The Cash Shop is not available on this'
    Write-Host '       server."'
    Write-Host '     THEN KILL SOMETHING AND WALK OVER THE DROP. That latch gates the'
    Write-Host '     pick-up sweep too, so picking up fine after three clicks is the'
    Write-Host '     second half of the same measurement.'
    Write-Host ''
    Write-Host '  4. GLANCE: when you pick something up, does a grey "<item> x<n>' -ForegroundColor Cyan
    Write-Host '     earned." line appear in the SCREEN MESSAGE AREA above the chat'
    Write-Host '     box? Two runs of it going out and nobody has described it.'
    Write-Host ''
    Write-Host '  OPTIONAL: -HeapFix (off by default, NOT on the same run as step 1)' -ForegroundColor DarkGray
    Write-Host '     Three bytes at 14019b504 in memory only; nothing on disk changes.'
    Write-Host '     A damaged pool header goes back to the free list instead of to'
    Write-Host '     HeapFree. SEVEN damaged slots over four dumps, all the identical'
    Write-Host '     value, all in the 0x20 class, ~1 per 250s.'
    Write-Host '       stops dying with 0xC0000374 -> chain confirmed end to end'
    Write-Host '       dies anyway                 -> the chain is wrong somewhere,'
    Write-Host '                                      which is worth more'
    Write-Host ''
    Write-Host '  NOT THIS RUN - decoded but deliberately NOT built:' -ForegroundColor DarkGray
    Write-Host '     Mina''s shop. The price is now known (row+0x38) but three of its'
    Write-Host '     fields fail SILENTLY or desynchronise the byte stream if wrong,'
    Write-Host '     and it has killed the client twice already.'
    Write-Host ''
    Write-Host '  GLANCES: drops arc from the corpse and are walkable-over; kill-EXP'
    Write-Host '  line is WHITE; mobs on map 40 already standing; pick-ups stay OUT of'
    Write-Host '  the chat log; level-up +16 HP / +12 MP; relog keeps Etc and mesos;'
    Write-Host '  ores stack; !setrates 2 3 5 -> one banner.'
    Write-Host ''
    Write-Host '  COMMANDS: !map !item !exp !heal !job !buff !unbuff !npcecho !npcfx'
    Write-Host '  !migsweep !exprate !mesorate !droprate !setrates !rates. !help lists all.'
} else {
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
Write-Host 'Count the Cash Shop requests - THREE clicks should be THREE lines:' -ForegroundColor DarkGray
Write-Host ('  powershell -NoProfile -Command "(Select-String -Path ''' + $serverLog + ''' -Pattern ''<- 0x00D5'').Count"') -ForegroundColor DarkGray
Write-Host 'And the latch the client read on each click:' -ForegroundColor DarkGray
Write-Host ('  powershell -NoProfile -Command "Select-String -Path ''' + $hookLog + ''' -Pattern ''WATCH #\d+: 0x142caee70'' | ForEach-Object { $_.Line }"') -ForegroundColor DarkGray
Write-Host ''
Write-Host "Then: powershell -ExecutionPolicy Bypass -File `"$PSCommandPath`" -Stop"
