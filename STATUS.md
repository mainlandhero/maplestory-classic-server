# Where things stand — 2026-08-20: **a character plays, and keeps what it takes off**

Pick-up notes for the next session. See `ROADMAP.md` for the plan and `docs/` for the
specs.

## START HERE

**There is a real server now.** `crates/login` replaced the Python harness on 2026-08-18,
and **characters persist between launches** - the priority the owner set the day before. The
client also no longer kills itself, so a session runs as long as you want it to.

One command, from an **elevated** shell:

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

It builds, installs the hook into `client-patched/`, starts **both servers** -
`maplecw-login` on 8484 and `maplecw-world` on 8485 - applies the client patches and
launches the client. Close the client by hand when done, then `-Stop`. `-ListOnly` prints
the stored characters and launches nothing.

**`-SetFieldProbe` is not optional.** Its name is a fossil: it now means "the channel
answers at all". Without it `Session::handle` returns nothing for *every* packet, the
migration hello goes unanswered, and the client sits on "Connecting..." looking exactly
like a server that is not running. It cost one of the owner's manual launches on 2026-08-20.

**The character-select screen is finished and server-driven**, all confirmed on screen:
the list, create, a truthful name check, the three-slot limit, **delete**, and persistence
across relaunches. Two things that cost real effort and must not be relearned:

* **Never renumber characters from 1.** A create reply carrying id 1 was byte-identical to a
  known-good one except the two copies of the id, and the client silently refused to
  transition. Ids start at 200.
* **Do not pass `-SessionTokens`.** The measurement it existed for is done (the answer is no),
  and passing them causes a "trouble connecting" dialog from a path we do not suppress.

The old harness still exists and still works - `test-charselect.ps1 -SkipNetCheck` - and is
the right tool for capturing packets or trying a hand-written body. It answers from canned
bodies and persists nothing.

**Reverse engineering the client: read `docs/ghidra.md` first.** The working command line,
the JDK 21 requirement, why the project locks (it matters the moment you spawn a subagent),
and the instrument mistakes that have each produced a clean, confident, wrong answer here.

**There are TEN packet-read primitives, not seven.** That count has been wrong three
times - five, then seven, then eight, then nine - and every correction came from
enumerating rather than searching a neighbourhood. `tools/reads.py` carries the list and
the history. A tail `jmp` into one of them **is** a read; missing one shipped a short
packet that killed the client twice.

Where the answers land:

| file | what is in it |
|---|---|
| `login.log` | every packet both ways on the **login** connection, and what each reply was |
| `world.log` | the same for the **channel**, from the migration hello onward - read this one for anything past character select |
| `client-patched\maplecw-hook.log` | `WATCH` lines, session patches, client faults. Not `hook.log`, not the repo root |
| `client-exit.log` | how the client died: exit code, lifetime, job membership, handle holders. A clean `0` is a hand-close; `0xC0000409` is the fail-fast returning |
| `probe.log` | only when running the old Python harness |

The client patches are still patches. The reachability check that `__fastfail`s the client
after ~37 seconds is now neutralised by the **probe patch** `watch@1415db360:ret`, which is
what `test-server.ps1` arms - which is why the launch line carries no `-SkipNetCheck`. The
argument still works and the old harness still uses it. See "SOLVED - the ~37 second exit"
below, and `docs/launcher.md` for which patches retire.

**A frozen UI is almost always an unanswered packet, not a crash.** The client blocks its
whole interface - every button, including the quit prompt's OK - waiting on a reply. That is
what "Check" did before `0x0081` was answered and what "Choose another world" did before
`0x0082` was. **Read `login.log`** - it names every reply and what it answered, so the last
inbound line with nothing after it is the packet nobody answered. `crates/login` has a test
for this rule, and no path in it returns an error in place of a reply.

## THE FIRST GOAL WAS MET — 2026-08-19, seen on screen

> Kept for the method, not the news. **Read the START HERE section below for current
> state** - a great deal has happened since, and this is a snapshot of the day the
> character first stood up.

A character stands on **map 1, Mushroom Town - West Entrance**, playable. The minimap and
street name are right, HP/MP/EXP are live, the tutorial NPC dialog fired, and there was **no
client fault**. Run preserved as `research/fixtures/character-on-map1-playable-*.log`.

The probe discriminated exactly as it was designed to:

```text
0x0010  140302e30 x3   from 0x1403094d0   the character-LIST path - the armed positive control
0x01A0  140304b20      from 0x142098432   the record decoder ran
0x01A0  140302e30      from 0x140304e76   the gate OPENED and the stat block decoded
        (no CLIENT FAULT line)
```

Everything the pre-flight analysis predicted held, and nothing it deferred bit: the three
all-zero randomiser seeds, the zero pair at head offsets 22/26, and the zero at offset 17 were
all sent as-is and none of them mattered.

## NEXT GOALS - set by the owner, 2026-08-19 onward

Login and world entry are done. Each goal carries what is already established, so nobody
re-derives it, and the **one concrete next step**. Goals A (quests) and F (shops) are
wired as of 2026-08-20, and I (inventory persistence) is confirmed on screen; the rest
stand as written.

### START HERE - what to do next, in order

**Last updated 2026-08-21, night: the channel change works, and the heap corruption is
finally captured in a dump.** Read this section and nothing else to know where the project
is. Everything under it is older and
kept for its working, not its verdicts.

**The test plan is NOT here.** It is in `tools/test-server.ps1`, in **two** places - the
`.NOTES` block and the `Write-Host` dialogue the launcher prints on screen - and both must be
kept current. `CLAUDE.md` has the section on why.

#### CONFIRMED on a real client

| | |
|---|---|
| the world | a dressed character on a map, real `Character.wz` stats |
| NPCs, movement, session | clickable and speaking, portals both ways, `!map`, Log Out |
| **combat, both directions** | *"The mob killings work, I'm taking damage, and the mob is also taking damage."* |
| **drops** | fall at the mob, staggered, **arc out over half a second**, and pick up into the right bag |
| **the EXP line is WHITE** | majority damage. Which also *measured* the `white` byte - it was inferred |
| **the scrolling banner** | `0x00AC` type 4 works, and `!exprate` drives it. Both were inferred; both are now seen |
| **mobs appear instantly** | `appearType -1` on field entry, `-2` on respawn |
| **the quest chain** | Heena -> Sera -> Heena: 1000 completes, 1001 starts, the mirror changes hands |
| **character create** | creating with and without a prior delete both transition |
| **equipping, including a swap** | *"Wearing an item no longer crashes."* And this **kills the heap-corruption repro** - see below |
| **ability points** | *"Assigning AP is fine now, in bulk and in singles."* Both `0x0138` and `0x0139` |
| **the job change** | *"the job sound is fine now"* - `0x007C` bit 5, and the client plays the effect and sound itself |
| **the quest-finish fanfare** | *"the quest completion SFX is now working"* - `0x02D1` effect 15, sound and no picture as predicted |
| **Roger's quest opens** | the authored overlay: their real opening, a **Next**, then an Accept/Decline box |
| **THE CHANNEL MIGRATE OPCODE IS `0x001A`** | measured 2026-08-21. The client tore down, connected to **127.0.0.1:8486** and sent a migration hello - so `u32 ip` network-order and `u16 port` little-endian are confirmed too |

#### CONFIRMED on the 2026-08-21 evening run - the big two

* **THE CHANNEL CHANGE WORKS.** The owner: *"Channel changed successfully seemingly. I checked my
  inventory items and mesos, seems like everything carried over."* `world-ch1.log` says how:
  `MIGRATION HELLO: character id 212` then *"claimed the migration for character 212 of
  account 1"* and a real `SetField` carrying map 40 - not the minimal fallback that killed the
  client last time. Goal closed: the opcode is `0x001A`, the body is `u8 ok, u32 ip` network
  order, `u16 port` little-endian, and identification is by **channel** because a migrate
  carries no character id.
* **THE HEAP CORRUPTION IS IN A DUMP.** For the first time in this project, after six
  undiagnosed deaths. `dumps\maplecw-crash-1096760-c0000374-1.dmp`, **1 010 MB**, written by
  the hook in **878 ms**, exception `0xC0000374` = `STATUS_HEAP_CORRUPTION` at
  `0x7ffca83af509` in ntdll. The client died at **596 s** of life, which extends the old
  193-482 s band rather than fitting it.
  The hook's own heuristic stack scan produced exactly one client frame, `0x14019b58e` - a
  lead and nothing more. **The real stack is in the dump**, and reading it needs a debugger:
  `tools/analyse-dump.ps1` finds the file and prints the WinDbg commands, because only the
  Store build of WinDbg is on this machine and it has no scriptable console.
  **Page heap is still off**, so `!heap -p -a` cannot print allocation and free stacks. `kb`
  and `!heap -s` are what carry the weight until it is enabled.

Also confirmed the same run, all of them first sightings on a screen:

| | |
|---|---|
| **create on the second login** | *"Done, no issues"* |
| **the client refuses to forfeit a completed quest** | so the client-side half of the farming loop was never open; the three server-side holes were |
| **consumables, with the cap** | *"hurt down to 136, it recovered to 146, the stack went from 2 to 1"*. The log says `+11 hp (now 146/146)` - a **100 HP potion capped at the 11 that were missing**, which is the cap working, not a 10-point potion |
| **Sera's idle chatter** | *"seem okay now"* - the byte revert holds |
| **the mob-damage override** | *"the snail hit me for 1 with the number on top of my head, but I actually took 3 damage according to the HP bar"* - exactly the predicted outcome |
| **the damage model, at a second stat point** | 14 swings at STR 35 with the same axe: **17..31**. The model spans **15.5..32.6** across the three attack actions (swing 2.4, mixed 1.8, stab 1.2 at `incWAT` 17), snail PDD is **0** and it is level 1 so neither defence nor level gap applies. Fits, and 14 samples would not be expected to reach either extreme |

#### The NPC fade: `!npcecho` says the creation packet is NOT the difference

The owner: *"with !npcecho, Heena still faded in."* The command sent template 1 (Heena) and
template 2 (Sera) on map 1 as **`0x0451` NpcChangeController** - the other creation packet,
the one mobs get and NPCs never had - 70 px to the side.

**The copies faded too.** That kills both standing theories at once: the creation route is not
what makes mobs pop, and the *timing* theory dies with it, because these arrived minutes after
field entry rather than during it. What is left is that the client fades an NPC because it is
an NPC, in its own rendering path, with no field of either packet to change it - which is what
two static passes concluded and which I was wrong to restate as "the server has no lever"
before there was a measurement. Now there is one.

*One thing to confirm before this is closed:* that it was the **copy** that faded rather than
the original. Two Heenas were on screen and only one sentence came back.

#### The owner's new request, built and unseen: the blue recovery number

*"the idle recovery should pop up with a blue number of the recovery amount above the player's
head. I don't see that here, the HP bar just moves up without a number indication."*

The bar always moved, because `hp`/`mp` carry the new totals. Nothing drew a number because
`0x007C`'s **second optional trailer** was absent - `u8 flag`, then `u32 hpRecovery,
u32 mpRecovery`, which the client hands to
`FUN_140fd31f0(uiGlobal, hpRecovery, mpRecovery, oldHp, oldMp)`. That call takes a **UI**
global, and it is the argument `research/level-up.md` used to name those two fields in the
first place. `user-hit.md` §5.2 said the trailer "should stay `None`" - correct for a *hit*,
which draws its own number client-side, and this is the case the field exists for.

Now sent by **idle regen and by consumables**, carrying the **amounts** rather than the new
totals, and absent when nothing moved so a full bar cannot draw "+0". **[L]** for the shape,
**[D]** for the hp/mp pairing.

#### The floating damage number is a constant, and that is worth one measurement

The client claimed **1 on all 25 hits** in that run - not a distribution, a constant - while
the server computed 3 fifteen times and 4 ten times. `incoming_damage` already applies the
player's own defence, so 3-4 is a defended number.

A computed value would vary. A constant will not. So either the client's formula genuinely
yields 1 for a snail against this character, or **the field is a stub and the client never
computes mob damage at all** - in which case the floating number will read 1 for every mob
forever and no server change can move it, because `user-hit.md` §4.4 establishes it is drawn
at *send* time by `FUN_142771360`.

**One hit from a much stronger mob discriminates.** If the number moves off 1, the client
computes and our formula is the thing that disagrees. If it stays 1, it is a stub.

#### PROVEN ON THE WIRE by the 13:48 run of 2026-08-21, but never reported

That run was read properly afterwards and it contains far more than the one thing that was
said about it at the time. **These all worked**, and `world.log` has the whole exchange:

* **Roger's quest, the entire chain.** The 6-byte yes/no accept parsed (`000000001001`),
  quest 1002 was stored, the apple went into Use slot 1, `0x007C` set HP to **25/130**, and
  the grey item line went out as `0x02D1` effect 8. Then the apple was eaten - `0x010E`
  arrived - and the server answered with +30 HP, an `0x0070` that emptied the slot, the quest
  completed, and +3 EXP.
* **Consumables.** That apple *is* the consumable path, answered correctly end to end.
* **Quest completion and its EXP payout.**

The chain is stronger than a log usually is, because the client had to have **drawn** the
apple for the owner to double-click it: the `0x010E` names item 2010000 in Use slot 1, which
existed only because the accept was granted. So the accept, the grant and the HP change all
reached the screen.

**What is still unknown is what any of it LOOKED like** - the grey chat line, where the quest
EXP appeared, whether the fanfare drew anything. Those are questions about the screen and
they are what step 2 of the plan asks.

#### WIRED, and NOT yet seen on a screen

Everything here compiles, is tested and is connected. **None of it has been seen by the
client.** The heading exists separately because `STATUS.md` has twice called something done
while it was unwired. Five rows came off this table on 2026-08-21 once the 13:48 run was read
properly - Roger's quest, the fanfare, consumables, the quest payout and `!migsweep` - because
leaving them here would have spent a launch re-testing what a log already answered.

| | what to look for |
|---|---|
| **the Etc bag and mesos survive a relog** | they always persisted - nothing ever *sent* them. Field entry now re-sends `0x0070` per item plus `0x007C` for the balance |
| **items stack** | Garnet Ore into one slot, not three. `slotMax 0` means unspecified, not one |
| **idle regeneration** | +10 HP and MP every 10 s after 10 s of no movement, attack or damage |
| **the Tutorial Jr. Sentinel** | 100% Shellpiece, no mesos, no second drop |
| **`!setrates <exp> <meso> <drop>`** | all three on one anchor, one banner. Every rate command now refuses below 1x |
| **drops land on the floor** | a mob killed **on a slope or a step**, not on flat ground - flat looks identical before and after, which is why this went unnoticed. `world.log` names the foothold for any drop that moved |
| **giving up a quest works** | start 1001, press give up in the quest window, then click Sera again. Test on **1001, not 1000** - 1000 is completed by then and its give-up button can never send anything |
| **the channel migrate, end to end** | `0x001A` alone now, and the far end claims **by channel** when the hello names nobody. Only the opcode has been seen working; the arrival has not |
| **the hook writes its own crash dump** | any fault should now leave `dumps\maplecw-crash-<pid>-<code>-1.dmp` and two `CRASH DUMP:` lines in the hook log. Tested in-process, never yet against the client |

**One warning about testing the AP fix, because it would otherwise produce a false pass.**
Every `0x007C` clears the client's one-request latch, and **idle regeneration sends one every
ten seconds**. Field entry clears it too. So *click, wait, click* succeeds whether or not the
AP handler works at all - measured, not feared: in `world-20260821-001440.log` a regen
`0x007C` landed 11 s after an AP request and re-opened the window. The discriminating test is
**two clicks inside ~3 s with no map change**, and `grep 0x007C world.log` between the two
`<- 0x0138` lines is the free cross-check.

#### Solved 2026-08-21, late: a finished quest could be farmed

The owner: *"I was able to complete the Heena quest multiple times, this is not okay."* **Three
independent holes**, any one of which is enough on its own. All three are the same shape - the
store was the authority, answered correctly every time, and the caller asked and then did the
work anyway.

* **The turn-in paid out twice.** `apply_quest_completion_rewards` sat *outside* the match on
  `store::complete_quest`, which correctly returns `None` for a quest that is already
  complete. Quest 1001's `Act.1` is `exp 2`, so it was two experience per click, indefinitely.
* **The accept handed the items over twice.** Same shape: `grant_quest_start_items` and
  `apply_quest_hp` were outside the match on `start_quest`. Quest 1001's `Act.0` is Sera's
  Mirror and 1002's is Roger's apple plus an HP change, so both were farmable a click at a
  time.
* **Give-up deleted a completed row.** `forget_quest`'s `DELETE` had no state predicate, so
  forfeiting a *finished* quest put the character back to never having touched it - after
  which the other two were not even needed. The forfeit handler's own doc block already said
  this must not happen; that sentence was describing a wire flag, and nothing enforced the
  database half. **A comment describing a guarantee is not the guarantee.**

**Why it survived a test suite.** The turn-in test counted **fanfares**, and the fanfare was
the one effect that *was* correctly gated on `recorded`. It passed on every run while the
experience doubled beside it. The three new tests fail without the fixes - checked by
reverting each one rather than assumed.

Fixed by making every effect hang off the transition: `record_quest_complete` returns early
when nothing changed, `record_quest_start` pays `Act.0` only when the row is new and sends
nothing at all for a completed quest (re-sending `quest_accepted` would put it back in the
client's *started* list), and the completed-row guard now lives in `forget_quest` itself.

**Still worth watching on the next run:** the server is now authoritative, but nothing here
explains why the *client* offered the turn-in again. Either its journal is not being updated
in-session, or it simply lets you re-click and the server has always been the only guard. The
new log lines say which - a repeat is now silent where a genuine inconsistency still prints.

#### Solved 2026-08-21, and what each one cost to find

* **Roger's quest could not start, and the reason was an absence.** `q1002s` is **not in the
  client at all** - all 205 `.wz` archives and all 10021 images enumerated, 0 errors, and the
  name occurs exactly twice, both times as a *name*. `Data/Etc/Script/Script.wz` is a 63-byte
  header with **zero entries**. Corroborated by a second, non-overlapping instrument: the
  **Spanish-only** `CommandGuide.img` documents `loadscript`, `scriptrun`, `runlua` and
  `loadquest` - commands whose purpose is to let *the server* re-read script files. So the
  bodies are ours to author: `data/quest-scripts.txt`, an overlay in `questlines.txt`'s own
  format. Five of Roger's six lines are their own words from `String.wz`.
* **Drops fell through the floor.** The stagger in `drops_from_kill` moved each item sideways
  and **nothing ever moved it vertically**. Replaying today's placement over every mob spawn
  point: **29% land on no surface at all**, 8.2% by more than the 10 px the client's pick-up
  box forgives. A third of the 94089 floor segments are **sloped**, so reading `y1` instead of
  interpolating would have been wrong 27577 times - usually by less than an icon's height,
  which is the shape of a bug that reads as bad luck.
* **The jump height is [L], not a fan site.** `Map_000.wz` ships `Physics.img`:
  `jumpSpeed 555.0`, `gravityAcc 2000.0`, so `v^2/2a` = **77 px**. Only the closed-form apex
  is [I] - the client integrates per frame.
* **The quest-finish fanfare is `0x02D1` effect 15.** Pinned through the *only two* readers of
  `Sound/Game.img/QuestClear`; the one that is not the pet-skill notice is the packet-driven
  effect handler. `research/level-up.md` had documented only **one** of that handler's two
  switches on the effect byte, which is why the arm had never been seen.
* **The missing quest EXP line was never missing.** The run's own log has it -
  `03 01 0200000000000000 00 0000...` - **byte-for-byte the same shape** as the `+200` lines
  from kills in the same session. Quest 1001's `Act.1.exp` is literally **2**. Two independent
  passes reached that separately. *Nothing was built for this*, which is the right outcome.
* **"Adding stats doesn't change my damage" - it does, and the floor is what stands still.**
  Character 206's real kit (level 7, DEX 10, the `1312000` axe, `incWAT` **17**, no mastery)
  gives **15..20 at STR 5** and **16..27 at STR 30**. Six times the STR moves the ceiling by
  7.7 points and the floor by **0.6**, because `M = (mastery/10 + 0.1) * 0.8` is **0.08**
  without a mastery skill - the primary stat contributes a twelfth of its weight to the
  minimum and its full weight to the maximum. Pinned as a test in `damage.rs`.
  **And the measurement agrees**: four of the eight real hits in that run (21, 21, 24, 26)
  are **above the STR-5 ceiling of 20**, so the client is scaling with STR. What is not
  scaling is the bottom of the range, which is what a player watching small numbers sees.
  **`incWAT` multiplies the entire expression**, so a better weapon moves damage far more
  than stats do at this level.
* **Mobs hit for 1 because the CLIENT said 1, and the server had no opinion.**
  `on_user_hit` applied `hit.damage` verbatim. Template 2's `PADamage` is **3** and
  `damage::incoming_damage` over that character gives **3 or 4**.
  **The value 3 appears at no offset in any of the twelve `0x00E5` bodies** - checked as a
  `u32` across all 144 offsets - so this is not the server misreading a field that holds the
  real number elsewhere. That check mattered: `UserHit::damage`'s own doc says its offset is
  **undiscriminated**, because every capture carries 1 and 1 also sits at six other offsets.
  The server now computes its own from the client's own `Mob.wz` data and **logs both
  numbers**, so the next run says whether the floating number and the health bar disagree on
  screen.
* **The channel migrate is `0x001A`, and a sweep found it in one run.** `research/change-
  channel-reply.md` had everything about that packet except its opcode, which no scan could
  reach - `FUN_1415d8c00` has zero callers of every kind, zero 4-byte RVA references, and
  `.themida` has `SizeOfRawData = 0`. Ten candidates went out and the **hook log named the
  winner**, because it writes one dispatch line per inbound opcode on handler *return*:
  `0x0019` took 64 us and did nothing, `0x001A` took **354 ms** and the socket closed.
* **And the same run showed why the migration still failed.** A channel migrate carries **no
  character id** - seven bytes, `ok`/`ip`/`port` - so the client's `0x007D` on the new channel
  reported id **32513**, which is `01 7f 00 00` read straight back out of our own body.
  Channel 1 refused it, answered with the **MINIMAL SetField** (whose own doc says it will not
  put a character on a map), and the client faulted 3.2 s later. A channel migration is now
  claimed **by channel** when the hello names nobody, and refuses when more than one is
  pending rather than guessing.
* **Answering `0x00D2` with `0x0011` was worse than not answering.** `0x0011` is a
  login-stage opcode below the channel switch's `0x70` floor, so it could never dispatch -
  and `0x00D2` **latches on send**, like `0x0107`, `0x010E` and both AP requests. The owner: *"the
  transfer did not go through, but I lost all ability to attack once the attempt was made."*
  Both halves are that one fact.
* **Roger's Accept was being dropped by the parser.** The dialogue was right all along - the
  overlay loaded, the Accept box drew - and `parse_script_reply` read a `u32 echo` and a
  string unconditionally. **A yes/no box replies with SIX bytes** (`handle, type, action`) and
  echoes nothing, so the reader errored, `on_script_reply` saw `None`, and the accept vanished.
  No HP drop, no apple, and clicking Roger again replayed the opening. The module already
  documented the *outbound* half of that rule and the inbound parser did not mirror it.
* **"Create a character" was dead after any Log Out, and the delete was a red herring.** The
  client's handshake calls `FUN_140c9e8a0`, which stores plaintext **0** into the flag gating
  the button, on **every** success - and our `create=on` patch latched on a static bool and
  set it once per *launch*. `login.log` had two `0x0010`s; the hook log had one
  `called FUN_140c9e230`. So creation worked on the first login of a launch and nowhere else.
  Now re-armed per login result, and a re-arm prints.
* **`0x009E` is the client's "I could not handle this packet" report** - a gift, like
  `0x025F` for drops. Its body carries the offending **opcode and body verbatim**. It appears
  in no other run in this repo, which is what made it a discriminator rather than noise.
* **The crash-dump instrument could never have worked, and it is now replaced.** WER was
  misconfigured once and was fixed; that was not the reason. Settled at **no cost in client
  runs** by a decoy: a program that does nothing but dereference null, **named
  `MapleStory.exe`** because LocalDumps keys match on the base name. It wrote a **9.4 MB dump
  into `dumps\`**. The real client raised the *same* code, `0xC0000005`, at **13:49:56 - 88
  minutes after WER was switched on**, which the registry key's own last-write time
  (`12:21:01`) proves - and produced nothing. Same machine, same hour, same executable name,
  same exception; every variable held but one. **The client ships its own crash reporting and
  never reaches `WerFault`.** `CrashReportClient.exe` sits beside it and it already uploads
  its own call stack in `0x008F`/`0x0090`.
  So the dump now comes from **the hook's vectored handler, which was already catching that
  exact fault and only logging it** - it is what writes the `CLIENT FAULT` line. First-chance,
  so the dump is taken at the faulting instruction rather than after the client has unwound,
  which for `0xC000_0374` is the difference between a usable heap and none.
  `crates/grap-stub/src/minidump.rs`, tested end to end - the test writes a real `MDMP` and
  reads it back, because a dump writer that has never written a dump is exactly the
  instrument this repo keeps getting caught by.
  **And the struct it needed is `packed(4)`, not naturally aligned.** `minidumpapiset.h`
  wraps every `MINIDUMP_*` in `<pshpack4.h>`, so `MINIDUMP_EXCEPTION_INFORMATION` is 16 bytes
  with the pointer at offset 4. With `repr(C)` it is 24 with the pointer at 8, and
  `MiniDumpWriteDump` returns `ERROR_NOACCESS` **whatever the contents are** - which is what
  eventually pointed at layout rather than data. A test asserting 24/8 sat beside it, passing.
* **The hook log is archived now instead of deleted.** `world.log` was being moved into
  `previous-runs/` while `client-patched\maplecw-hook.log` was deleted at the same moment.
  That is precisely the asymmetry that makes `CLAUDE.md`'s "count the same event in two logs"
  impossible for any run but the current one - and three of this project's answers came from
  exactly that comparison. Both halves now land in the repo's `previous-runs/`.
* **"The server cannot fix the NPC fade" was my over-generalisation, and the owner caught it.** Two agents proved a real negative - no field of `0x044F` controls it - and I restated that as *the server has no lever*, which does not follow. The owner: *"You shouldn't need to patch the client. Are there no way for the server to send the NPC data to the client so that it appears instantly?"* There is a second creation packet, `0x0451`, it sets a different state byte, and it is the one mobs get. Both agents were enumerating the wrong list; so was I when I accepted the conclusion.
* **The dump instrument was configured and switched off at the master switch.** The owner,
  reasonably, believed crash dumps were enabled - `HKLM\...\Windows Error Reporting\
  LocalDumps\MapleStory.exe` points `DumpFolder` at the repo's `dumps\`, `DumpType` 2 (full),
  `DumpCount` 2. All correct. But
  **`HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\Disabled = 1`** - WER is off
  machine-wide, so `WerFault` never runs and never honours LocalDumps. **Five crashes since
  the folder was pointed at the repo on 2026-08-20 23:24 have produced zero dumps**, and the
  newest file anywhere on the machine is from 2026-08-19.
  This is `CLAUDE.md`'s own rule biting: an instrument that *looks* armed and is not. Page
  heap is separately absent from IFEO, which the 418 MB peak working set independently
  confirms - a page-heap run peaks near 990 MB.
* **A byte I swapped killed the client, and the comparison is the cleanest this project has
  produced.** A static pass read `0x044F` byte 20 as the facing bool and byte 21 as the
  animation action, both **[L]** off the listing, so they were swapped and byte 21 given the
  constructor's default of `0`. The next run died on Sera's first idle line. Two `0x044F`
  bodies for them on map 1, **differing in exactly those two bytes and identical everywhere
  else**: `...00 01 0800...` chattered lines 1, 2 and 3 over 23 s; `...01 00 0800...` produced
  `0x009E`, two C++ throws, `0xC0000005`, and **no dispatch line for the `0x0453` at all**.
  The chat packet is innocent - `e9030000ff0000000000` is byte-for-byte one that had worked
  11 s earlier on map 40. **Reverted to the bytes that are measured to work.** The attribution
  may still be right; the conclusion drawn from it was not.
* **`0x009E` is the client's "I could not handle this packet" report** - a gift, like
  `0x025F` for drops. Its body carries the offending **opcode and body verbatim**
  (`...5304 e9030000ff0000000000 5304`). It appears in **no other run in this repo**, which is
  what made it a discriminator rather than noise.
* **Consumables did nothing because nothing answered `0x010E`.** The owner: *"I tried to consume
  Red Potion, but it did not recover 100 HP."* The request was on the wire and logged
  `UNKNOWN`: `f7e1140f 0100 80841e00 01000000` - tick, slot 1, item **2000000**, and both the
  id and the slot check out against state this server had written. There was exactly **one**
  in the run despite the owner using more than one item, which is the request-latch signature
  again.
* **What a potion restores is in `spec`, not `info`** - which is why it had never been dumped.
  Every earlier pass read the `info` child. Red Potion `spec/hp` is 100 and Roger's Apple is
  30, matching its own tooltip. `hp`/`mp` are flat and `hpR`/`mpR` are **percentages of the
  maximum**; both units exist in this client's data and some items carry both.
* **Mode 1 of `0x0070` is `UpdateQuantity`.** Drinking one of two potions cannot use mode 3,
  which removes the **whole slot** - the client would empty it while the server still held
  one. `take_quest_item` sends mode 3 for a partial take too, and is correct there only by
  accident: quest 1001 takes one out of a stack of one.
* **AP: the roadmap named half the opcode.** `0x0139` is the **bulk** request; a plain `+`
  click sends **`0x0138`**, a different opcode with a different body - the stat window's
  handler is a flat chain of twelve name comparisons, `"strup"` against `"strupall"`, calling
  two different builders. Answering only `0x0139` still looks broken to anyone using the
  button. The six mask bits are now **[L] by name** from this client's own string ids
  (`SID_MSCW_STAT_STR` and its five siblings), which upgrades six constants in
  `crates/net/src/stats.rs` that were `[I]` from the reference version. Two traps in that
  table: the button says "HP" but the bit is **max** HP, and the widths differ *inside one
  reply* - u16 for the four stats, u32 for max HP/MP, in a body with no resync point.
* **Job advancement: the packet was found before goal E was written.** It is `0x007C` mask
  bit 5, already decoded and already in `stats.rs`; nobody had joined the two up. **The
  client plays the fanfare itself** from that packet, and `JobChanged` *is* one of
  `BasicEff.img`'s 40 nodes - so unlike the quest-clear case, expect sound **and** picture.
  **Do not also send `0x02D1`:** effect 14 reads two `u16`s where effect 15 reads none, so
  reaching for `user_effect_local(14)` by analogy would have shipped a 1-byte body to a
  5-byte read - the short-packet mistake that has killed this client twice.
* **And the first job advancement has no quest at all.** Enumerated rather than searched: the
  whole key space of `Act` and `Check` across all 322 quests is 17 and 32 shapes, and **there
  is no `Act.<n>.job` key anywhere**. `Act.<n>.item.<n>.job` exists 57 times and is a *reward
  filter* - a search for "job" would have matched it and returned a confident wrong yes. The
  `Test`/`Proof of Qualification` quests are the **second** advancement at level 30
  (`Check.0.job` requires 100/400 already, `Check.0.lvmin` is 30, and quest 20001 says "2nd
  job advancement" in words).
* **Change Channel is not a stage case at all.** The migrate reply is `FUN_1415d8c00`, a
  **socket-level** handler dispatched from the Themida VM - established by a fully enumerated
  funnel, not an absence: the connect has exactly one caller, which has exactly two, one of
  which is the login-stage `0x0011` we already send. The body is measured - `u8 ok`,
  `u32 ip` in **network** order, `u16 port` **little-endian** because the client `htons`es it
  - with the read count cross-checked by two instruments that agree exactly. **The opcode is
  the one thing that cannot be read statically** (zero callers, zero RVA references,
  `.themida` `SizeOfRawData = 0`), so `!migsweep` exists to settle it in one run.
* **The quest forfeit was in an opcode we already parse.** `0x0151` **action 3**, a **5-byte**
  body - and `parse_quest_request` needs a 9-byte head, so it returned `None`, and `None` was
  silence. Two captures show the packet arriving and nothing coming back. `0x01ED` and
  `0x01A5` both went unanswered four times in the same run and both looked like better
  candidates; `0x01ED` was ruled out by finding **its two bodies byte-for-byte in a capture
  from 2026-08-19 that contains no quest traffic at all**.
* **The AP latch has a confound, and finding it saved a run.** Every `0x007C` clears
  `ctx+0x2330`, and idle regeneration sends one every ten seconds - so *click, wait, click*
  would have passed even against a completely broken AP handler. The agent that found it also
  **weakened its own earlier claim**: the latch explains some of the three missing clicks,
  not provably all three, because one of the two captures had its latch cleared by regen
  11 s in.
* **The EXP curve was checked against the client at last, and it agrees** - 99 of 99 levels,
  zero disagreements, and the file now runs to 119 because the client's table has 100..119
  and the guide stopped at 100. It cost **no client run**: the dump was already sitting in a
  fixture. What it did cost was fixing `tools/decode_dump.py`, whose off-by-one label made the
  first comparison say all 98 levels disagreed. See "Instruments that have lied".
* **Equipping over a worn item now swaps** - and the client does the swap **itself**. Mode 2
  of `FUN_142d51930` is an unconditional two-way exchange (`142d52c13` writes the displaced
  item into `oldPos`), so the reply stays **one entry, 14 bytes**. A second entry would not be
  redundant, it would be *wrong*: it would move the item that had just arrived.

#### Solved 2026-08-20, and what each one cost to find

* **Sera recited Heena's tutorial.** Quest 1000 has no `Say.1`, so the completion fell back to
  `Say.0` - the opening of the quest being finished. A completion now chains via
  `Act.1.nextQuest` instead.
* **Sera handed over nothing.** `Act.0.item` was never read. Now it is, and `Act.1.item` too.
* **Etc items and mesos "did not persist".** They persisted perfectly; the client was never
  told. Both reasons were already written down in this repo and neither had been noticed.
* **Garnet Ore would not stack.** `ShopTable::max_per_purchase` already had the right rule and
  **three callers had each hand-rolled `slot_max.max(1)` instead of calling it.**
* **A full Equip tab was blamed for blocking Etc pick-ups** and was innocent - a test proves
  it. The symptom was the missing restore above.

#### What to do next, in order

**The test plan is in `tools/test-server.ps1`** - both the `.NOTES` block and the on-screen
`Write-Host` dialogue, and both were rewritten on 2026-08-21 to 7 steps. Item 2 of the old
list is struck off: it has been answered without a run.

| # | do this | why it is here |
|---|---|---|
| 1 | **Finish the channel change** | The opcode is **`0x001A`, measured**, and the body is confirmed - the client tore down and connected to 8486. What failed was identification: a migrate carries **no character id**, the hello reported `32513` (our own `01 7f 00 00`), channel 1 refused, sent the **MINIMAL SetField**, and the client faulted 3.2 s later. Claim-by-channel is wired and prints a distinct line. **If the fault goes away with it, the minimal SetField is a loaded gun** and needs rethinking rather than being the documented safe fallback |
| 2 | **What Roger's quest LOOKS like** | Every packet is proven on the wire (see above). Three open questions, all about drawing: is the item line **grey, in the chat log**; does quest EXP land in the **chat log** rather than bottom-right; does the fanfare play with **nothing drawn**, which is the predicted result |
| 3 | **NPCs fading in - `!npcecho`** | Two passes said the server had no lever; both enumerated the **fields of `0x044F`** rather than the **packets the pool accepts**. There are two that create an NPC: `0x044F` (`or [obj+0x38],1`) and **`0x0451`** (`mov byte [obj+0x38],2`, then the identical body). **The case that works uses the second**: mobs get `0x03C6` *and* `0x03D2`. One command, one run |
| 4 | **The two `0x00DF` header fields** | The damage formula is decoded and cannot be *used* without the **action** and the **skill id**, neither of which is parsed out of the attack header. `research/damage-formula.md`. Highest-value next step for goal J |
| 5 | **The heap corruption, but only once a `.dmp` exists** | The instrument is finally real. `!analyze -v` and `!heap -p -a` are the two commands that matter. Do not spend a run *hunting* it - take the dump the next death produces on its own |
| 6 | **The classic shop counter** | `0x055D` is the opcode; the body's thirteen-field row structure is not decoded. `research/classic-shop-opcode.md` |
| 7 | **Job advancement, the conversation** | The *packet* is done and `!job` tests it; what is left is the NPC path. `research/job-advancement.md` 8.1 has the seven-step exchange. The instructors are **not in the towns** - 511 on map 10004003, 313 on 10002003, 221 on 10001051, 411 on 10003003, pinned by a test |
| 8 | **`tools/dump_equips.py` hard-codes its columns** | Its docstring claims the set is enumerated and it is not. All 1760 equip images carry **`attackSpeed` and `attack` on 203 weapons each**, neither in `equips.txt` - so no caller can supply a real weapon speed |
| 9 | **The other script quests** | 1002 and the four `Proof of Qualification` closes are authored. The `Test of Qualification` four are the **second** advancement at level 30 |
| 10 | **Death, and mob->player damage tuning** | `damage::incoming_damage` is now authoritative for mob hits. Death itself is still unbuilt |

**Two refusal paths still send a packet the client cannot dispatch.** `change_channel_refused`
answers with `0x0011`, and so does the no-such-channel case - both undispatchable on a channel
socket, so neither clears the `0x00D2` latch. Nothing decoded can. Said out loud rather than
left to be rediscovered.

**Two contradictions in this repo, both found 2026-08-21 and neither adjudicated:**

* **The `white` EXP byte.** `STATUS.md` above says `white = 1` was *confirmed on screen*;
  `research/exp-sharing.md` marks it **[I]**. Only `white = 0` -> yellow is [L] (from the owner's
  screenshot). The server sends 1 and the screen has looked right, so this is a labelling
  fault rather than a behavioural one - but one of the two files is wrong.
* **`research/npc-chatter.md` §8.2 retracted `[npc+0x270]` on a bad scan**, and the retraction
  was the mistake. The decoder does write it - `141e36df4 41 89 84 24 70 02 00 00` is
  `mov [r12+0x270], eax`. That scan's positive control used `rsi` as a base, which needs no
  SIB byte, while `r12` always does, **so the control could not exercise the encoding that was
  actually there**. `opcode.rs`'s `ENABLED` label was right all along. This is the third time a
  carefully-hedged negative has been used to withdraw something correct.
* **The `0x0070` read count.** `research/msexe-setfield.md` says 18 read sites;
  `tools/reads.py`, which is the authority, finds **17**. The mode-2 path is unaffected either
  way, but this project has shipped a short packet twice by trusting the wrong count.

**Dead assets, so nobody hunts for them:** `Sound/Game.img` contains `IncEXP` and `questCount`
and **neither name appears anywhere in the 76 MB executable, in either encoding** - while the
same scan finds `PickUpItem`, `Portal`, `QuestAlert`, `LevelUp` and `JobChanged`. There is no
EXP-gain sound in this client to send.

#### THE TEST PLAN

**It lives in `tools/test-server.ps1` now**, at the top of the file, so the steps and the thing
that launches them cannot drift apart. Run the launcher and read them there.

Two watches still need their own runs, neither combinable with the plan:
`-SetFieldProbe -UserState` for the user state machine, and `-SetFieldProbe -MobTargets`.

#### Change Channel: we answer with a LOGIN-stage opcode on a GAME connection - 2026-08-21

The owner changed channels and nothing happened. The capture says the request arrived, we answered
it, and the client ignored the answer:

```text
<- 0x00D2  19 byte body  01 2d2c010d 64000000 47580000 ...   (target channel 1, 0-based)
-> 0x0011  MIGRATE_COMMAND ... to channel 1 at 127.0.0.1:8486
   ...six seconds of ordinary mob traffic...
   ch0 connection closed
```

**Channel 1 was listening on 8486 and received no connection at all** - `world-ch1.log` is 23
lines of startup banner and nothing else.

**The body is not the problem.** Compared byte for byte against the `0x0011` the *login*
server sends, which demonstrably works, the two are structurally identical - same length,
same fields, differing only in the port (`2521` = 8485 vs `2621` = 8486, both correct) and the
single-use seed.

**The opcode is the problem.** `0x0011` is a **login-stage** opcode. The channel stage
dispatches through `FUN_142cbaa80`, whose labels are **`0x70..0x19f`** plus the outliers
`0x275` and `0x39a` - `research/msexe-gamestage-opcodes.md`. `0x0011` is **below the bottom of
that range**, so a channel connection cannot dispatch it at all. The client is not refusing
the migration; it never sees one.

That also explains the note this reply already carried: *"The reply SHAPE is inference; no
capture."* The shape was inferred from the login migrate, and so, silently, was the opcode.

**Next step:** find the channel stage's own migrate/change-channel reply. It is somewhere in
`0x70..0x19f`, and the two request/response pairs already identified in that switch (`0x162`
answering with `0x175`, `0x275` with `0x17e`) are the model for how to look - a case that
*sends* something is rare enough to enumerate.

#### The AP request is `0x0139`, and the body decodes - 2026-08-21

The owner tried allocating ability points three times over two runs and nothing happened. It was
not identified until they did a **bulk** assign, which put a distinctive payload on the wire:

```text
<- 0x0139  16 byte body  dfbdfe0c 01000000 40000000 1e000000
                         ^tick    ^count   ^0x40    ^30
```

`0x40` is the **STR** bit and `30` is the amount they typed. So the shape is
`u32 tick, u32 count, u32 statMask, u32 amount`, and it sits one slot below `0x013B`, the
skill-up request - which is exactly where a stat-up request belongs.

**Why it took three attempts to find.** The earlier hunts looked at the *unanswered* census
and `0x0139` never rose above one occurrence, because a single-point click and a bulk assign
do not produce the same traffic. What found it was searching the bodies for the **number they
typed** rather than ranking opcodes by count. Enumerating by frequency hid it; searching for
the payload did not.

**Not implemented.** The handler, the stat mask table and the `0x007C` reply that confirms the
new value are all still to write. This is now a small job rather than an open question.

#### Roger's quest needs a SCRIPT engine, which does not exist - 2026-08-21

The owner: *"Roger's Apple quest doesn't start as expected. All I see is 'Hey, nice weather isn't
it', which is just their normal text."*

**Quests in this client come in two kinds and only one is implemented.** [L], from the WZ:

```text
1002  Check  0.npc          3
1002  Check  0.startscript  q1002s
1002  Check  1.endscript    q1002e
1002  Say    1.stop.item.0  Eat the #r#t2010000##k I gave you...
```

Quest 1002 has **no `Say.0` at all**. Its opening is a *script*, `q1002s`, and the client
asks for it with `0x0151` **action 4** - `QUEST_ACTION_OPENING_SCRIPT`, which
`crates/net/src/script.rs` already names. The capture shows exactly that, three times over as
The owner retried: `04 ea030000 03000000`.

With no `Say.0` to play, the conversation falls through to the NPC's own idle line - which is
what was on screen. **Nothing is broken; a whole feature is missing.** The Say-tree quests
(1000, 1001) work because they *have* Say trees.

What a script engine needs, and it is not small: the scripts are named but their bodies are
not in `questlines.txt` at all, so the first question is where `q1002s` lives - a WZ node this
project has not dumped, or something compiled into the client.

#### Character create does not transition, 2026-08-21 - narrowed, not solved

The owner created "Tester", the client said the name was available, and then **stayed on the
creation screen**. Clicking OK again said the name was taken - because it was: the character
exists.

**Established from the captures, all [L]:**

* The character **is created**. Id 206 is in the database with its equips.
* The reply is **not** the problem. `0x0015` for "Tester" is **353 bytes**, and so is the
  `0x0015` for "Engineer" on 2026-08-19 that **did** transition - same length, same head
  structure field for field, differing only in the id, the name, the cosmetic ids and the
  random stat roll (both totalling 25).
* The **flow into it is identical** in both runs: `0x00A8` -> `0x05F4` -> `0x0081` name check
  -> `0x008A` -> `0x0015`.
* So this is a **regression in something other than the create reply**, and the reply is
  ruled out rather than assumed innocent.
* The static route is **blocked**: the `0x0015` handler `FUN_141b2cf30` decompiles to
  `halt_baddata()` - 35 bytes of bad instruction data. `research/msexe-char-create.c`.

**What actually differs between the working run and the failing one** is the account's
character-list state. On 08-19 the client logged in holding **3** characters and had deleted
one earlier in the same session; today it held **1** and had deleted two in a *previous*
session.

**The list-state hypothesis is FALSIFIED.** The owner ran both halves of the experiment on
2026-08-21: creating without deleting first **transitions**, and delete-then-create
**transitions**. Four mechanisms are now eliminated:

* **not the reply** - byte-compatible and the same 353 bytes as a create that worked;
* **not the list state** - both orders work;
* **not the cosmetics** - the face and hair in the reply match the request and the stored row
  exactly, for the failing character and the working ones alike;
* **not a live channel connection** - `world.log` for that run begins six minutes *after* the
  failed create, so no channel socket existed.

So it happened **once**, and nothing reproduces it. That is where it rests. The capture is
`research/fixtures/create-did-not-transition-tester-login.log`, and the static route is closed
- `FUN_141b2cf30`, the `0x0015` handler, is 35 bytes of `halt_baddata()`.

**If it recurs, the thing worth writing down is what came before it** - how long the client
had been up, what screen it came from, and whether a channel session had been entered and left
in the same launch. The packet is not the variable, so the state is.

#### The heap corruption, and what the last run actually showed

The owner, 2026-08-20: *"I tried to execute `!map 45`, which the command should guard me against,
but the client crashed."* **The guard is innocent, and this is what the evidence says.**

* **No chat packet reached the server at all.** Enumerating every inbound opcode in that
  run's `world.log` gives no `0x00E7` of any kind. `!map 45` was never transmitted, so
  `gm_map` never ran.
* **The guard was live.** `world.log.err` is 0 bytes, and a missing field table writes to
  stderr there, so `gm-handbook/fields.txt` loaded. 45 is not in it, and the command would
  have refused.
* **The fault is `0xC0000374`, heap corruption**, raised 325 ms after the last packet inside
  `FUN_14019b4e0` - a heap helper beside the client's allocator and free. That is where it
  was *detected*, not what corrupted it, and heap corruption is detected arbitrarily long
  after the write that caused it.
* **It is not new and not today's code.** Four sightings now, the earliest on 2026-08-19,
  before any of today's packets existed. Full write-up and the three captures:
  **`research/heap-corruption.md`**.
* **The new banner packet has never been sent in any of them** - zero `0x00AC` - so it is
  not that, and neither are the rate multipliers or the level gains.

**The next step is page heap, and the owner has to run it** because it writes a system key. The
exact command is in `research/heap-corruption.md`. It makes the client fault at the
instruction that corrupts rather than at the next free, which is the difference between a
diagnosable crash and this one. **The two full captures have also never been diffed**, and
that costs no client run at all.

One real defect did come out of it: `map_exists` is fail-open on an empty field table, so a
missing generated file silently removes the `!map` guard. There *was* a warning for that, on
**stderr**, where nothing reads it - the banner that reaches `world.log` is stdout. The banner
now reports the field count, and `!map` refuses outright rather than answering as though it
had checked.

#### Things that are NOT open, so nobody re-opens them

* **The pick-up opcode is `0x032C`**, object id at body offset 13. It can never be confirmed
  further: the builder is in `.themida`, whose `SizeOfRawData` is **0**.
* **`pool+0x90` is not a latch.** It is a client-side anti-cheat check that clears itself.
* **`mob+0xa88` is not the mob's animation object** - it is the avatar-look renderer, and
  `0` is correct for it on all 193 mob templates. The animation object is `mob+0x610`.
* **Item quests need no running count.** The client counts the bag live.
* **The shop is not a row problem and not a map problem.** One correctly-formed row killed
  the client exactly as twelve did, and the map loads fine without a shop.
* **There are TWO shop windows and we were sending the wrong one.** `0x055D`/`0x055E` are the
  classic `UIShop.img/Shop`; `0x0560`/`0x055F` are Shop2, whose art is missing. Both are arms
  of the same range chain in `CField::OnPacket`, and that chain is validated by `0x01A0`
  landing on the SetField handler two entries away. `research/classic-shop-opcode.md`.
* **The client cannot settle goal K.** The EXP curve at `0x143AC2400` has no sibling table:
  a `lea` scan of the whole neighbourhood that fills it finds exactly one `.data` target and
  it is the EXP curve itself. HP/MP per level is a server-side rule and the client never
  computes it - it is told new maxima. See goal K.
* **The bag is not the unequip blocker**, `0x02FF` must be answered, `0x0107` must always be
  answered including refusals, and the client computes its own damage.
* **`0x0301` is a MOB picking up a drop**, not the player's request.
* **`0x00AC` is BroadcastMsg and type 4 is the banner**, `0x00AB` is TownPortal, and the
  candidate table's two `BroadcastMsg` entries are both wrong. `research/broadcast-banner.md`
  has the working, including the bit mask that says which types carry no string.

#### Instruments that have lied, and are now fixed

Every one of these produced a clean, confident, wrong answer. They are listed because the
next wrong answer will come from an instrument nobody has checked yet.

| tool | what it did |
|---|---|
| `tools/callers.py` | scanned for `call` only. **27 909** of 120 981 functions would have come back "zero callers" while reachable by a tail `jmp` or a vtable pointer. Now reports three kinds |
| `tools/rtti.py` | mapped addresses in the zero-initialised tail of `.data` into `.pdata` and returned those bytes as data - 120 confident wrong numbers for the EXP curve. Now raises |
| `tools/fieldrefs.py`, `tools/rangescan.py` | **silently drop `rbp`-based operands.** A scan for a field's writers came back without the one writer everybody already knew about. **Not fixed** - work around it and say you did |
| `tools/encodes.py` | misses fields written by a **loop**, so a body length from it alone is short and confident. **Not fixed** |
| the probe's throw log | logged throws only 25 s after arming, so a client that died at 23 s recorded **zero throws** and read as "it did not throw". Now logs the first eight always, and every fault line reports throws *seen* against *logged* |
| `tools/decode_dump.py --exp-curve` | labelled entry `i` as level **`i+1`**, so the first-ever comparison against `data/exp-curve.txt` reported **all 98 levels disagreeing when all 99 agree**. Its own comment asserted the wrong indexing, so reading the code confirmed the bug rather than catching it. It also printed a **permanent** false `monotonic: NO - suspect` - the trailing cap zero compared against the last real level. **Fixed 2026-08-21**, and the fix is checkable without trusting either table: 1 hole + levels 1..120 = 121 entries with exactly 2 zeros, which is the 119 non-zero it reports |

---

> Everything below is **older working, kept for its method rather than its verdicts.**
> Where it disagrees with the section above, the section above is right.

#### THE FIX: every mob was sent a size of **zero percent** - 2026-08-20

Body offset 91 -> `mob+0xd64` is the mob's **size percentage**, and this server sent `0`
there from the day mobs were implemented. `0` is not "unset"; it is *zero percent*. **[L]**

```text
141c571bc  cmp   ecx, 0x64          ; scale == 100?
141c571bf  je    0x141c573c8        ;   yes -> skip the adjustment entirely
141c571cb  sub   eax, r8d           ; width = right - left
141c571d4  sar   eax, 1             ; halfWidth
141c571da  lea   eax, [rcx - 0x64]  ; scale - 100
141c571f6  mulsd ...                ; delta = halfWidth * (scale - 100) / 100
141c571fe  add   [rdi + 8], eax     ; right += delta
141c57201  sub   r8d, eax           ; left  -= delta
```

At `0` the multiplier is `-1.0`, so `delta = -halfWidth`, both edges collapse onto the centre
and `left >= right`. `141d326ae` then **skips** the rect instead of rejecting it, `r14b` stays
`0`, and `141d327c6` drops the mob - with all seventeen documented gates green, which is
precisely why seventeen green gates explained nothing.

**Invisible because the two paths disagree about what `0` means**: the rendering paths test
`<= 0` and read it as "no scale set, draw normally"; the hit-box path tests `!= 100`. The
snails drew, animated and walked while having no hit box at all.

**`mob+0xa88` is not the bug and the label was wrong.** It is the mob's *avatar-look*
renderer - mobs drawn as player characters - and **0 of 193** mob images carry the
`avatarLook` node that would allocate it. Null is correct. The animation object is
`mob+0x610`. `research/mob-a88.md`.

> **CONFIRMED ON SCREEN, and this un-does a retraction.** The owner, after the fix: *"The mob
> killings work, I'm taking damage, and the mob is also taking damage."* One field, both
> directions.
>
> The retraction is worth keeping because it was the more expensive mistake. A static pass
> reported that the mob-to-player path reads the **player's** body rect and takes its
> rectangles from the mob's **attack template**, and that a snail has no `attack` node - so
> this field could not be involved. The two-direction claim was withdrawn on that basis, and
> The owner was told not to expect the snail to hurt them.
>
> What that pass had found was the *attack-node* path, for mobs with attack animations. It
> stated in its own words that a **body**/touch-damage path was **not found**, and marked it
> `[D]` with a named blind spot: a function handed the template as an argument reads
> `[rcx+0x7c]` without touching `+0x3a8`, so the intersection scan cannot prove absence.
> **"Not found" was read as "not there".** That is this file's own rule pointing the other
> way - a silent negative believed because it was tidy - and it cost a correct answer being
> withdrawn hours after it was made. `research/touch-damage.md` remains the best map of the
> attack-node path; only its negative failed.

#### MEASURED: `mob+0xa88` is null on every mob - 2026-08-20, on a real client

`research/fixtures/mob-a88-null-on-every-mob-{world,hook,exit}.log`. The client closed
cleanly (exit 0); nothing crashed.

| | |
|---|---|
| positive control `140304100` | **10 lines** - the hook armed, so the readings mean something |
| distinct mobs the collector walked | **30**, all inside a single swing |
| `mob+0xa88` | **`0` on all 40 readings**, every mob without exception |
| `mob+0x42c` | `0xffffffee` / `0xffffffef` - **not** zero |
| attacks sent | 11, **every one 127 bytes**, which is zero targets |
| first collector hit -> first `0x00DF` | 16 ms, so the pairing holds as before |
| `0x02FF` mob movement reports | 990 - the mobs were alive and moving, not a dead field |

**The predicted root cause is confirmed.** The animation object is null on every mob, so
`FUN_141c57120` bails at `141c57185` and writes an all-zero rectangle, so `141d326ae` skips
every rect, so `141d327c6` rejects every mob - with all seventeen documented gates green.

**One prediction was wrong and it does not change the answer.** The forecast was that a null
`0xa88` would show up as an all-zero `mob+0x42c`; the measured left edge is **-18/-17**, a
perfectly ordinary coordinate. That is consistent rather than contradictory: `mob+0x42c` is
an *input* to `FUN_141c57120`, and the `0xa88` bail at `141c57185` happens **before** the
input is ever consulted. A real value sitting in a field that is never read is exactly what
a bail-out looks like from the outside. Worth writing down, because reading `0x42c` alone -
which the superseded watch pair would have done - would have shown a healthy rectangle and
pointed the next pass in precisely the wrong direction.

**What this is now:** `mob+0xa88` has two writers in the mob code range, the constructor's
null and `FUN_141cd1620`. So the question has moved off the attack path entirely and onto
the **spawn** path: what makes `FUN_141cd1620` run, and whether anything our `0x03C6` sends
can reach it. Nothing about the swing, the attack rectangle, or the gate chain needs more
work.

#### The gate that rejects our mobs is geometric, and it was never in the table

`research/mob-gates-arm-c.md`. All seventeen documented gates pass; the rejection is at
label `141d32a18`, which the earlier write-up dismissed as *"a nested container-growth block,
plus two list-walk exits"*.

```
141d326ae  cmp [rdi],eax / jge   left >= right  -> SKIP this rect (not a rejection)
141d327c6  test r14b,r14b / je   nothing intersected -> REJECT THE MOB
```

**An all-zero rectangle satisfies the per-rect filter and is skipped**, so `r14b` stays 0,
the loop finishes clean, and every gate anyone had enumerated is green. Fifteen slots, the
mob examined, nothing accepted - exactly the symptom.

The chain reaches a single field:

```
FUN_141cd1620   sets mob+0xa88, the animation object (the constructor leaves it NULL)
FUN_141cb4600   141cb4645  mob+0xa88 == 0 -> RETURN, writing no rect
                141cb4647  otherwise -> lea rdx,[r15+0x42c]  <- THE SETTER
FUN_141c57120   141c57185  mob+0xa88 == 0 -> bail -> ALL-ZERO rect
```

**`mob+0xa88` gates the rect in two independent places.** `-MobTargets` now reads that field
and the rect together, because reading the cause beside the effect separates three outcomes
where two halves of the rect separated only two. The three readings are in the launcher's own
comment, beside the probe string.

> **A retraction worth reading.** "Nothing in the mob code range sets `mob+0x42c`" was
> reported, then found wrong. Two scans - one over the mob range, one over the whole image -
> agreed, and both were blind the same way: a `[reg+disp]` write-scan cannot see a store made
> through a `lea`'d pointer, which is precisely what the setter does. `CLAUDE.md` now carries
> the general form: re-running the same tool is not a second opinion.

#### THE ROWS ARE NOT THE VARIABLE - run 2, 2026-08-20

`research/fixtures/shop-one-row-still-faults-{world,hook,exit}.log`. `-ShopRows 1` sent
**one buy row, 38 bytes** (`4 + 2 + 31 + 1`, and the log line confirms *"1 rows (1 buy, 0
sell) - 11 row(s) HELD BACK"*). The client hung, then died: fault at `0x140ce89d6`, 2.9 s
after the packet, exit `0xC0000005`.

Twelve rows killed it and one correctly-formed row killed it. **So the fault is not in row
content, not in row count, and not in the sell rows.** It is the shop path itself.

**And map 1013 is not the variable either - established without spending a launch on it.**
In `research/fixtures/mob-a88-null-on-every-mob-world.log` the client entered the world
**on map 1013**, received Lucy's `NpcEnterField` (template 21), stood there 5.4 s, walked
away and exited cleanly with code 0. The map loads, Lucy spawns, everything is fine. The
only difference in the two crashing runs is that they were **clicked** and we answered with
`0x0560`.

> **Run 2 logged zero C++ throws, and that is the instrument, not the client.** The probe
> logged throws only 25 s after arming. The watches armed at `09:46:10.6`, so the window
> opened at `09:46:35.6` - and the client was already dead at `09:46:33.8`. Read naively
> that says "the client did not throw", which is the opposite of the truth, since run 1
> caught a throw 10 ms after the same packet. **Fixed**: the first eight throws are now
> logged whatever the clock says, and every fault line reports how many throws were *seen*
> against how many were *logged*, so those two can never be confused again.

#### Lucy's counter kills the client - what the logs establish, 2026-08-20

Fixtures: `research/fixtures/shop-opens-then-client-faults-{world,hook,exit}.log`.

```
11:51:21.098  -> 0x0560 OpenShop, 12 rows (6 buy, 6 sell), 379 bytes
11:51:21.108  C++ THROW #7                       <- ten milliseconds later
              (the client sends NOTHING further, at all)
11:51:24.656  CLIENT FAULT 0xc0000005 at 0x140ce89f7
```

**The throw is the event and the fault is the wreckage**, and that is now a chain rather
than a guess:

* seven throws in the whole two-minute session, and this one lands 10 ms after our packet;
* from that instant the client sent nothing - no movement, no shop request - so it was
  wedged immediately, 3.5 s before it died;
* `0x140ce89f7` is inside `FUN_140ce89c0`, a **reference-counted release**, and the fault
  stack's first frame `0x14308e3ed` is the return address of the call at `0x14308e3e8` in
  `FUN_14308e3db` - a 24-byte destructor thunk;
* `research/setfield-fault-shape.md` had already characterised that function as **a latent
  bug in the client's own error path**: any early exit that skips the acquisition leaves the
  holder's `+8` as uninitialised stack, and the epilogue's guard then releases it.

A throw unwinds, unwinding runs destructors, and one of them ran over a local the skipped
path never wrote. So **the fault address says "something bailed out", not "the shop"** - it
is the same address a completely different subsystem faulted at in
`setfield-accepted-client-entered-world-hook.log`.

**Four hypotheses checked and dead**, so nobody re-checks them:

| | |
|---|---|
| the body is short | **no.** `tools/reads.py` over the row decoder gives exactly the nine documented fields plus the trailing `u8`: `4 + 2 + 31n + 1` = 379, which is what went out. The two *extra* `u8` reads it finds in the outer handler belong to the `0x055F` arm - `140d22888` is followed immediately by `jmp` to the exit |
| an item id the client cannot resolve | **no.** All six ids are in the client's own `itemdata`. `!item` refuses unknown ids; the shop has no such check, so this was the obvious suspect |
| the disabled flag, or a zero max-per-purchase | **no.** The two fields `research/npc-shop.md` §2.6 names as able to silently break a shop are 0 and 100/200/1 |
| the old Amherst crash | **no.** That one had the NPC never appearing and died in the `SetField` tail. Here the map loaded, Lucy spawned, and they were clickable |

The array-constructor frame in the throw's stack is **stale** - there is no call to
`FUN_142ef44fc` anywhere in the shop handler. A stack scan is not a call stack.

**What is not established: which row, or whether rows are the variable at all.**
`--shop-rows 1` (`-ShopRows 1`) is the lever - one **buy** row, because the buy direction has
a straight-line trace behind it and the sell direction has never been on a wire in either
direction. A counter that opens says the shop path is sound; one that still dies says rows
are not the variable and the next work is static, not another launch.

#### The loop RUNS, with room for fifteen - measured, 2026-08-20, and it moves the question

**No client run was spent on this.** It came out of a fixture that had been sitting in
`research/fixtures/` for a day with the answer already in it.

The suspicion was that `FUN_141d31b20` returns at `141d31c96` before examining a single mob,
when argument 17 is at least argument 4. Filtering
`melee-collector-runs-once-per-swing-hook.log` to the collector's **own** watch lines - the
file carries four watches and the attribution matters - all six entries read:

```
0x141d31b20 ENTERED ... rcx=0x37d74fa0 rdx=0x146530 r8=0x145de0 r9=0xf  called-from=0x141d2545a
```

| | | |
|---|---|---|
| `r9 = 0xf` | **argument 4 = 15** | the loop's capacity, hardcoded at the call site |
| argument 17 | the output cursor, 0 on entry | so `0 >= 15` is **false** |
| `called-from` | `0x141d2545a` | `FUN_141d25360`, and **not** `0x1428c2c32` |

**So the loop ran, with room for fifteen targets, and accepted none.** The early-out is dead
as an explanation. Two corrections come with that, and the second is the important one:

* `141d31c96` is the **loop header**, not a one-shot precondition. Argument 4 is a capacity
  and argument 17 is the output cursor, incremented at `141d327de`/`141d32939` with
  `141d32a62` re-entering. `research/mob-target-gates.md` §4.2 reads it as a precondition.
* **`research/mob-target-gates.md` §6 dismissed gates 3, 5, 11, 14, 15 and 16 as "switched
  off by the arguments at the only known call site" - and that call site was the wrong one.**
  It analysed `0x1428c2c2d` inside `FUN_1428c1fa0`. The client calls through
  `FUN_141d25360`, a different arm with different arguments, which reaches the same collector
  at `141d25455`. Those six gates were ruled out for a caller the client never uses, so the
  clean "every gate passes" result was answering about the wrong path.

`research/mob-collector-callsites.md` has all 86 call sites with their return addresses, so
any future `called-from=` reads straight off a table. It also establishes that **every** entry
is a 5-byte `call rel32` - no tail `jmp`, no vtable, no function pointer anywhere in the image
- so a return address always names a real call site.

The next pass re-runs the gate analysis against **arm C's** arguments. That is static work and
it does not need a launch either.

#### `FUN_141d31b20` IS the melee collector - measured, and it corrects a prediction

`research/fixtures/melee-collector-runs-once-per-swing-{world,hook}.log`, one session:

| collector hit (hook clock) | attack sent (server clock) | gap |
|---|---|---|
| `01:31:40.568` | `05:31:40.570` | 2 ms |
| `01:31:41.668` | `05:31:41.669` | 1 ms |
| four more, same shape | | 1 ms |

**Six swings, six hits, each 1-2 ms before its `0x00DF`, in order.** No unpaired hits, no
unpaired attacks. Every attack is 127 bytes, which is zero targets.

Two corrections fall out of that, and both would mislead the next person:

* The caller is **`0x141d2545a`**, not the `0x1428c2c2d` that `research/mob-target-gates.md`
  predicted. Its outcome table maps "called from elsewhere" to *"per-frame noise, melee is
  elsewhere"* - which is exactly wrong here.
* `while dispatching opcode` in a WATCH line is **sticky** - it is the last inbound opcode,
  not the reason for the call. It read as mob traffic and nearly filed this as noise.

So the gate chain is the right instrument, the collector runs once per swing, and it
collects nothing. The next run watches **the gates**, not the entry.

**Do not change `appear_type`.** `>= 0` writes the literal `1` into `mob+0x504`, which is
exactly what gate 2 rejects on - it would make every mob permanently unhittable and look
like a regression somewhere else. The warning is on the field in `crates/net/src/mob.rs`.

#### Things that are NOT open, so nobody re-opens them

* **The bag is not the unequip blocker.** `presence[7]` lands; 125 slots render. Minimum 30,
  maximum 125, both from the owner.
* **The four lists after the equipped one are not four bags.** Only the first is - the Equip
  tab. The other three take positions 3000+, which nothing here can create. `research/bag-lists.md`.
* **`0x02FF` must be answered**, or mobs freeze after one simulation step.
* **`0x0107` must always be answered, including refusals**, or the whole inventory UI dies.
* **The client computes its own damage.** We never send numbers, only consequences.
* **`0x0301` is a MOB picking up a drop**, not the player's request. It nearly shipped as one.

#### The test plan for the next run

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

**The path is absolute on purpose - write it that way in anything the owner runs.** The launch has
to come from an **elevated** window, and an elevated window opens in `C:\Windows\System32`,
so a relative `tools\test-server.ps1` is not a shorter spelling of the same command. It is
one that works in the agent's shell and fails in theirs.

**`-SetFieldProbe` is not optional and its name is a fossil.** Without it `Session::handle`
returns nothing for *every* packet: the migration hello goes unanswered and the client
freezes on "Connecting...". It cost a launch on 2026-08-20. `crates/world/src/session/mod.rs`,
`Session::handle`.

**Do not pass `-Probe` unless you mean to.** With `-SetFieldProbe` and no explicit `-Probe`
the launcher installs a matched pair *plus* `140304100:hits=200`, the **positive control**:
no lines from it means the hook never armed and nothing else in the log proves anything.
Passing `-Probe` by hand replaces all four slots and silently drops it.

| # | do | what to watch | what it means |
|---|---|---|---|
| 1 | glance at the top of `login.log` | an ELog record | the client replays its on-disk error log at startup and then deletes it. Free, and it is the only place a previous crash survives. Run it **from the repo** - `cd C:\MapleCW` then `python tools/decode_elog.py login.log --stack`; the repo has to be the working directory or the script imports the scratchpad's stale copies |
| 2 | enter the world, `!item 1302000` | the sword appears in the Equip tab | `!item` works, and so does the `0x0070` Add it rides on |
| 3 | `!map 1013`, click **Lucy**, with **`-ShopRows 1`** | the shop counter | **the counter opening at all is the result.** Twelve rows killed the client on 2026-08-20; one buy row says whether the shop path is sound. If it opens, raise `-ShopRows` next run |
| 4 | buy something, then sell it back | mesos, and the bag | both directions, both prices. The **buy** price is authored; the **sell** price is the client's own |
| 5 | accept a quest, then `!map 40` | the quest journal | still listed = the journal persists |
| 6 | type **Hello** in the chat box | a balloon and a log line | the flag byte was `0` and the client's own builder sends `3`. Nothing still = the handler is not running, and the next step is `0x0224`, **not** more chat bytes |
| 7 | `!exp 100` | the EXP bar | it moves **at once** - `0x007C` bit 16 carries the new total. Relog and it is still 100 |
| 8 | walk a few steps, then drag the sword out of the window | the sword on the ground | `0x00D9` is parsed now, so the server knows where you are. A `0x025F` in `world.log` means the client **rejected** our `0x046E` and abandoned the drop - read its second `u32` against `research/item-drop.md` §10.1 |
| 9 | walk over the sword | it goes back in the bag | **even if nothing visible happens, this step succeeded**: `grep UNKNOWN world.log` names the pick-up opcode, which is the whole reason for it |
| 10 | open Change Channel, **single-click** CH.2 | the row's colour | cream then blue = the enable byte is right. **Do the double-click last of all** - it sends `0x00D2` and either changes channel or ends the session |

**`0x025F` was a false alarm, and the retraction is worth reading.** The claim was that the
client "builds it six times from inside `DropEnterField`, so it will arrive the moment a
sword lands". That misread **six call sites as six sends**. All six are the same shape -
`test ptr,ptr / jne carry-on / mov r8d,<code> / call the builder / jmp abandon` - so it is an
**error report**, sent only when the drop handler hits a null and gives up. Nothing waits on
a reply: the builder's `SendPacket` is followed by the stack cookie and `RET`, five of six
callers jump straight to the handler's exit, and no capture in the repo contains one.

So it is a **gift, not a hazard**. A `0x025F` means our `0x046E` was rejected and the drop
was abandoned - which otherwise looks exactly like "nothing on the ground, no fault, nothing
in any log". Its second `u32` is a `__LINE__`-like code naming which of the six null checks
failed; `research/item-drop.md` §10.1 turns it into an address. Expect site 2 (`0x3d1`)
first: it is the only null that comes from a **lookup by id** rather than a packet field.

Two watches, each needing its own run, neither combinable with the above:

* chat: `-Probe "watch@1415db360:ret,141b2a280:rdx=0,14276df20:peek=10d0:hits=6,140304100:hits=200"`
* **mob targeting: `-SetFieldProbe -MobTargets`** - a switch since 2026-08-20, so the four
  slots and the positive control come out right without hand-typing them. Go to map 40 and
  swing at a snail six or eight times.

**Do not spend a run re-asking whether the loop starts.** It does: argument 4 is 15 and the
cursor is 0, measured off an existing fixture - see "The loop RUNS" below. `-MobTargets`
survives as the frame to hang the *gate* watches on once the arm-C analysis names them; its
`141d31b20:args=17` slot is worth keeping only because the cursor is the one number that says
whether **any** mob was accepted mid-loop, and `called-from=` names the arm in one word.

`:args=<n>` dumps integer arguments **5..=n** off the stack (argument *n* at `[rsp + 8*n]`, so
argument 17 is `[rsp+0x88]`). Arguments 1-4 were always in the log as rcx/rdx/r8/r9, and
`stack_trace` filters to things that look like code addresses, so it discards exactly the
small integers this turns on. Slots 5..16 print too, on purpose: "argument 17" is a
decompiler's numbering, and if it disagrees with the ABI by one the right value is still on
the line.

**Nothing authenticates.** The game socket carries no credentials, and none of the above
changes that.

### THE NEW GOALS - set by the owner, 2026-08-19, later in the day

These are goals rather than tasks; each is bigger than a sitting.

#### A. Quest state that actually advances - **kill quests DONE 2026-08-20**

**The paragraph below is stale and kept for its method.** Quests persist, accepting is
recorded on `0x0151` action 1, the journal is sent on field entry, and a kill now counts
toward every started quest that named that mob. The count is **three zero-padded decimal
characters per mob slot** - an integer there renders as nothing, which is exactly what a
broken counter looks like. Item quests need no running count: the client counts the bag live.
`research/quest-progress.md`.

What is *not* done: turning a finished quest in, and what the client expects when the player
returns to the NPC short of the requirement.

The original write-up, from when none of it worked:

**Nothing about quests persists or changes.** Accepting does nothing, the same line comes
back every time, and the journal never fills. Full write-up is in its own section below
("NEW GOAL ... quest state that actually advances"). In short it needs the **presence-gated
quest block** in the character record - the third block of the same shape as `presence[0]`
and `presence[2]`, both of which are worked examples - a **quest-result packet that has
never been found in either direction**, a `quest_state` table, and the `Check`/`Act` rules,
which are a pure function over data already generated.

**Depends on item 1 above:** a quest cannot be *accepted* until the client can answer a
yes/no box.

#### B. NPC idle chatter

The owner, 2026-08-19: NPCs should cycle their idle lines **in order, on a cooldown**. The data
is generated - `gm-handbook/npcstrings.txt`, 441 chatter lines across 266 NPCs, and Robin's
ten match an outside listing exactly, in order.

**What is not settled is whether the server sends it at all.** The client may well do this
itself, the way its minimap reads the WZ directly and the way it picks its own
`0x00F2`-vs-`0x0151` click path out of `Quest.wz`. If it is client-side, the answer is a
**field in the `0x044F` body** - and that body has form: every NPC was invisible for days
because `isEnabled` and `alpha` were zero while the layout was perfect. Several fields there
are still placeholders.

#### C. Mob drops - **DONE 2026-08-20**

Items fall when a mob dies, at the mob's position, staggered, expiring after two minutes,
and the pick-up works. 993 rows over 170 mob templates from the community database, plus a
global table for event items that every mob rolls. `crates/world/src/droptables.rs`,
`data/drops.txt`, `tools/scrape_drops.py`.

**The chances are ours, not data.** The source has vote scores and no drop rates; only the
meso rows carry a real chance. The score is kept as a column so the policy can be retuned
without scraping again.

#### D. Character level up - **DONE 2026-08-20**, and the curve is worth a check

A kill awards the mob's own EXP from `gm-handbook/mobtemplates.txt`, levels carry over, and
the client plays its own level-up animation from the `0x007C`. The curve is
`data/exp-curve.txt` - **and it is no longer a community guide's word. The comparison was
finally made on 2026-08-21 and levels 1..99 agree EXACTLY, 99 of 99**, against the dump in
`research/fixtures/equip-into-empty-hat-slot-kills-client-hook.log`. The file now runs to
**119**, because the client's table carries 100..119 and the guide stopped at 100. Level 120
is the cap; the client's own entry for it is `0`.

**It took fixing the instrument first, and that is the point.** `tools/decode_dump.py
--exp-curve` labelled entry `i` as level `i+1` - its comment said *"Entry 0 is level 1's
requirement"* - so the first comparison reported **all 98 levels disagreeing when none of them
do**. Entry `i` is level `i`'s requirement: `[0] = 0` is the hole that makes the index
1-based, `[1] = 15` is level 1, `[119] = 28171993`, `[120] = 0`. One hole plus levels 1..120
is 121 entries with exactly two zeros, which is the 119 non-zero the dump reports - the counts
settle it without needing either table to be trusted. The same fix cleared a **permanent**
false `monotonic: NO - suspect`, which was the trailing cap zero being compared against the
last real level.

What a level *awards* is policy, tagged `[I]`, in one place: five AP, flat HP/MP, no SP.

The original brief follows.

> *"Once we can kill mobs, the next thing to handle is character level up. Once the EXP
> reaches or exceeds 100%, the player advances to the next level. They receive their level up
> animation client side and gets extra HP/MP, 5 ability points, and 3 skill points. If they
> are under level 10, they only receive 1 skill point per level."*

Blanket-accepted as the rule. What it needs, and which parts are blocked:

| part | blocked on |
|---|---|
| EXP arriving at all | **mob combat** - EXP has no source until a mob can be killed |
| the EXP-to-next-level curve | nothing - it is in the client, and extracting it is independent |
| +5 AP, +3 SP (1 below level 10) | nothing - server-side arithmetic |
| extra HP/MP | nothing to *decide*, but the per-level amounts come from the client's own job data |
| the stat change reaching the client | **mob combat** - that agent owns the stat-change packet, and level/HP/MP/AP/SP ride the same one |
| the level-up animation | nothing - the owner says the client owns the animation, so this is one effect packet |

**One ambiguity, and it is stated rather than guessed.** "If they are under level 10" is read
here as **the level being reached**: 5 -> 6 gives 1 SP, and 9 -> **10** gives 3, because the
new level is not under 10. The other reading (the level being left) would make 9 -> 10 give
1. It is one comparison to flip if that is wrong, and it is called out here so it is a
decision rather than an accident.

**The trap to avoid.** HP/MP per level is exactly the kind of number this project keeps
getting from the reference server and paying for later - the inventory bag was 24 for a
commit because of it. Take it from this client's own WZ, and if it cannot be found there,
say so and label the number **[I]** rather than shipping it as fact.

#### E. First job advancement at level 10 - set by the owner, 2026-08-19

> *"Once player levels up to 10, they should be able to job advance to one of the 4 primary
> job IDs, either Thief/Magician/Bowman/Warrior by talking to the respective job advancement
> instructors. Each job has a pre-requisite, which I assume is widely available on the
> internet. Magician 35 INT, Warrior 35 STR, Thief 35 LUK, Bowman 35 DEX."*

Blocked on **D (level up)** - there is no way to reach level 10 yet - and it shares the
script machinery with **A (quest state)**.

**The job ids are [L], from this client.** `net::opcode::uses_extended_sp` decodes the SP
fork's three literal bit masks in `FUN_140302e30`, and they give exactly the explorer tree:
`100/110/111/112/120/121/122/130/131/132` and the same shape at `200`, `300`, `400`, `500`.
So the four first jobs are **Warrior 100, Magician 200, Bowman 300, Thief 400** (and Pirate
500 exists in the masks whether or not we use it).

**The instructors are [L], from the client's own `String.wz/Npc.img`** via
`gm-handbook/npcstrings.txt`:

| job | instructor | template |
|---|---|---:|
| Bowman | Athena Pierce | **221** |
| Magician | Grendel the Really Old | **313** |
| Thief | Dark Lord | **411** |
| Warrior | Dances with Balrog | **511** |

There are also four NPCs literally named `<Job> Job Instructor` - 227, 319, 424, 514 - and a
second set at 800003/800004. Which set a real advancement uses is **not** established; the
named four are the ones the game is known for.

**The 35-stat prerequisite is [I], and the client will not help.** The owner flagged it as
fan-site sourced themself. A scan of every `Check` node in all 322 quests
(`gm-handbook/questlines.txt`) finds **no `int`, `str`, `dex` or `luk` requirement anywhere**
- and that is a *verified* negative, not a failed search: the same scan enumerates 20 other
Check keys including `lvmin` (406 uses), `job` (104), `skill` (48) and `item` (798). The
requirement is not in `Quest.wz`.

That is the expected answer rather than a surprise: in this game family the first job
advancement is an **NPC script**, and NPC scripts are server-side. So:

> **The client does not enforce this rule, which means the server is the only thing that
> can.** There is nothing to verify it against and nothing that will catch it being wrong.
> Ship the owner's numbers, label them **[I]**, and put them in one named table so they are one
> edit to change - do not scatter `35` through the code.

**What still has to be found:** the packet that actually changes a character's job, and
whether the client needs anything beyond the stat block's `job` field at the next `SetField`.
`research/charstat-layout.md` has the stat block; the job field is at `+0x33` and it is
already sent on every `SetField`, so the cheapest first experiment is whether simply storing
a new job and re-sending the record is enough to make the client show a first-job character.

#### F. NPC shops - **UNBLOCKED 2026-08-20: the opcode is `0x055D`**

**`0x0560` cannot work on this client and no server byte can change that.** The shop window's
constructor loads `UI/UIWindow2.img/Shop2/backgrnd`; that image is **not in this client's
WZ**; the resource call fails, `_com_issue_errorex` throws, and the unwinder faults. The
client never returns from the handler - neither crashing run has a numbered dispatch line for
`0x0560` - and it dies **before reading a single row byte**, which is why one correctly-formed
row killed it exactly as twelve did. Two of the owner's manual launches went on this.

It is **off by default** and a shopkeeper falls through to ordinary dialogue.

**That question is answered.** There are two shop windows on two adjacent opcode pairs, and
we were sending the wrong one:

| | window | art | open | result |
|---|---|---|---|---|
| classic | `UI/UIShop.img/Shop` | **present** | **`0x055D`** | `0x055E` |
| Shop2 | `UI/UIWindow2.img/Shop2` | **absent** | `0x0560` | `0x055F` |

Both are arms of the same range chain in `CField::OnPacket`, and the chain is validated two
entries away by `0x01A0..0x01A3` landing on the SetField handler. Full working, including how
the strings were found and what the instrument was checked against, in
`research/classic-shop-opcode.md`.

**What is left is the body, and it is not small.** The head is decoded and confirmed by two
instruments - `u32`, `u8`, a conditional `u32`, then `u32`, `u32`, a 31-character string, a
`u32` and a `u16` row count. But **each row carries a thirteen-field item structure**
(`FUN_1404ba100`, one caller, bespoke to this shop) with an undecoded conditional tail, and
**nothing yet says which of those fields is the price**.

**Nothing was implemented, deliberately.** `crates/net/src/shop.rs` still builds `0x0560`.
Pointing it at `0x055D` without decoding the body would send a short body to a handler that
reads a longer one - the same mistake as the truncated chat packet, with the same
consequence. `research/npc-shop.md` is the template for finishing it; most of its method
transfers.

Everything else about the shop is built and tested - the row layout, both tabs, the
transaction, the untradeable rule - and is waiting on that one answer.

The original brief:

> *"NPC shops when clicked on by the client should open the appropriate NPC shop with the
> appropriate shop list with appropriate item prices. Users should also be able to sell items
> in their inventory back to the NPC shop for mesos, such as unused equips and monster ETC
> drops. Please do not allow quest items to be sold."*

#### G. Storage - set by the owner, 2026-08-19

> *"Storage is kind of like inventory, except all of the characters of a particular account
> share this inventory. The storage stores mesos and items. Please do not allow untradeable
> items to be stored."*

**That blocker is gone, 2026-08-20.** This paragraph used to say storage was blocked on the
inventory existing at all: no item in a bag, no meso balance, `0x0107` never sent. All three
are now false. The bag persists across a map change (goal I, confirmed on screen), every
`0x0107` is answered including refusals, and meso drops credit `characters.mesos` and report
the new balance with `0x007C` bit 18 - the only way this client is ever told one.

What storage still needs is its own work: an **account-scoped** container beside the
character-scoped one, and **the opcode that opens the storage window - which nobody has
looked for.** There is no storage decompilation in `research/` at all, so treat that as the
first step rather than assuming it resembles the shop window. `crates/world/src/shops.rs`
already has `may_be_stored` waiting for a caller. Shops and storage are both "move an item
between two containers", so whichever gets built first should own that machinery.

##### What the client already gives us, measured

| | where | |
|---|---|---|
| **item price** | `Item.wz` `<item>/info/price` | **[L]** - read directly out of `Item/Etc/Etc_000.wz`; `04000001` is `price: 1`, `04000002` is `price: 2` |
| **the quest-item flag** | `Item.wz` `<item>/info/quest` | **[L]** - `04000000` carries `quest: 1` **and** `price: 0`. The owner's "do not allow quest items to be sold" is enforceable from the client's own data, not a fan site |
| **the untradeable flag** | `info/tradeBlock` | **[L]** - already extracted for equips by `tools/dump_equips.py`, which measured **7 of 1760**. The owner's "do not allow untradeable items to be stored" is likewise enforceable from client data |
| **stack size** | `info/slotMax` | **[L]** - `200` on the ETC items sampled |

That is a good outcome and worth saying explicitly: **both of the owner's restrictions are
properties the client ships**, so neither has to be invented or taken from a fan site. Extend
the `tools/dump_*.py` family to write a `gm-handbook/itemdata.txt` carrying
`id, price, quest, tradeBlock, slotMax` and both rules become table lookups.

##### What the client does NOT have, and this is the scoping fact

**Shop contents are not in the client.** Checked three ways, each with a control:

* `Data/Etc/Script/Script_000.wz` contains **no images at all** - the client ships no NPC
  scripts. Control: the same `wz-dump tree` call on `Etc_000.wz` lists 67.
* Of those 67 images, none is a shop or storage list. The only near-misses are
  `CashShopCategory.img` (cash shop UI categories) and `NpcNoticeBoard.img` (13 bytes).
* `Npc.wz` carries no shop node reachable from the NPC template.

So **which items an NPC sells, and for how much, is server data we author** - which is how
the real service works too, and it is why `price` alone is not a shop.

> **An instrument correction, because it nearly became a finding.** The first pass at this
> searched the `.wz` files for the raw bytes `shop`/`storage` and got zero, which looked like
> an answer. It was not: WZ encodes property names, and the same search returns **zero for
> `price`, `quest`, `info` and `icon`** - all of which had just been read out of that exact
> archive with `wz-dump`. A raw byte grep over a WZ archive is a broken instrument and any
> negative from one is worthless. Use `wz-dump`.

##### What still has to be found

1. **The inventory item itself** - the record block that puts an item in a bag, and `0x0107`.
   The equipped-list block (`presence[2]`) is already decoded and is the closest model.
2. **The shop dialog packet** - what opens the shop UI and carries the list, and the inbound
   buy/sell request. The NPC-click path (`0x00F2`) already reaches the server, so the trigger
   exists; only the reply is missing.
3. **The storage dialog packet**, and the storage NPC templates.
4. **Mesos.** Nothing maintains a meso balance today. The character stat block has a field
   for it (`research/charstat-layout.md`); it is sent as zero and has never been exercised.

##### Storage is per ACCOUNT, and that is a schema decision

`characters` is keyed per character; storage is not. It wants its own table keyed on
`account_id`, alongside a meso column - **not** a column on `characters`. Worth stating
before anyone adds it in the wrong place: the account is already the unit that owns
characters (`crates/store/src/db.rs`), so the foreign key is natural.

#### H. Citizenship - set by the owner, 2026-08-19

Classic World's town-membership system, and **it is the thing the shop data's rank tags
were waiting for.** Pick Henesys or Kerning City, work a community board, climb ten grades
that unlock town shop items, discounts, storage rates and eventually housing.

##### This is the best-corroborated spec in the project, and that is measurable

The owner's source is a fan site drawn from the second closed online test - the same "COT2" the
shop data is labelled with. Normally that would make every number **[I]**. It does not here,
because **the client ships the same data and it agrees**.

All **88** citizenship quests exist in this client's own `Quest.wz`, ids `506000`-`506141`,
and the count matches the site's exactly. Quest `506001` ("First Greeting with Rina")
checks field for field:

| | client `Quest.wz` | the site |
|---|---|---|
| level gate | `Check.0.lvmin = 12` | Lv 12 |
| town | `Check.0.citizenshipTown = 1` | Henesys |
| grade gate | `Check.0.citizenshipGrade = 1` | grade 1 |
| the board | `Check.0.npc = 235` | Community Board (Henesys) |
| the resident | `Check.1.npc = 201` | Rina |
| EXP | `Act.1.exp = 321` | 321 |
| mesos | `Act.1.money = 351` | 351 |
| reward | `Act.1.item.0 = 2010004 x5` | Orange x5 |
| contribution | `Act.1.citizenshipContr.amountFormula` | "100 at grade 1, +50 per grade" |

That last row is the striking one: the client carries the rule as a **literal formula
string**, `100 + ( ( citizenshipGrade - 1 ) x 50 )`, in 36 quests. **[L]**

##### What the client's own data establishes

| | |
|---|---|
| `Check.N.citizenshipTown` | 86 uses, values **1 and 2** only - 45 quests for town 1, 41 for town 2 |
| `Check.N.citizenshipGrade` | 86 uses, values **1..9** as gates |
| `Act.N.citizenshipContr.town` | 86 uses - which town banks the contribution |
| `Act.N.citizenshipContr.amount` | 50 uses, flat: 50, 80, 100, 180, 500, 750, 1000, 1250, 1500, 1750, 2000, 2250, 2500 |
| `Act.N.citizenshipContr.amountFormula` | 36 uses, the formula above |

So **contribution is awarded through `Act.citizenshipContr`**, with either a flat `amount` or
a formula, and it is gated by `Check.citizenshipTown` + `citizenshipGrade`. All **[L]**.

##### The grade table

Levels, contribution thresholds and the three discount columns are **[I]** from the site -
none of them appears in `Quest.wz`. The names are corroborated: they are exactly the rank
tags in `data/shops.txt`.

| grade | name | Lv | contribution | shop disc. | storage disc. | storage fee/item |
|---:|---|---:|---:|---:|---:|---:|
| 1 | Traveler | 12 | from the quest | 5% | 5% | 95 |
| 2 | Visitor | 17 | 1,000 | 7% | 10% | 90 |
| 3 | Helpful Stranger | 22 | 2,000 | 9% | 15% | 85 |
| 4 | Recognized Guest | 27 | 3,000 | 11% | 20% | 80 |
| 5 | **Town Resident** | 32 | 4,000 | 13% | 25% | 75 |
| 6 | Trusted Neighbor | 37 | 5,000 | 15% | 30% | 70 |
| 7 | Distinguished Citizen | 42 | 6,000 | 17% | 35% | 65 |
| 8 | Town Patron | 47 | 7,000 | 19% | 40% | 60 |
| 9 | Guardian of the Village | 52 | 8,000 | 21% | 45% | 55 |
| 10 | Citizen of Honor | 57 | 10,000 | 25% | 50% | 50 |

Grade 5 unlocks VIP dailies from the town leaders - 3,487 EXP against 321 for a regular
daily, and both numbers appear in the client's quest data.

##### THIS UNBLOCKS THE SHOP DATA

`data/shops.txt` records rank tags exactly as the live UI shows them - `Visitor+`,
`Town Resident+`, `Guardian of the Village+` - and its header says the mapping to grade
numbers "is NOT established". **It is now**, and it is just the table above: the tag is the
grade name, `+` means that grade or higher. That turns every gated shop row into an integer
comparison.

##### The rest of the rules, all [I] from the site

* Sign with **Arthur** (Henesys Town Hall) or **Roxy** (Kerning City Civic Center). One town
  at a time; switching banks the old town's progress and restores it on return. First move
  free; reactivation 50,000 mesos at grade 1, other grades unconfirmed **by the site itself**.
* Dailies: `First Greeting` runs once, `Asking After` repeats. Weekly asks for 100 of one
  monster's ETC drop and pays 500 contribution at grade 1, +250 per grade.
* Citizen discounts apply in the **active** town, on tagged items only - buff potions, pet
  food, return scrolls, the town cab.
* Resident's Chair: 10,000 mesos from the furnishings store at Visitor, **trade-blocked**,
  restores 20 HP / 5 MP per 10 seconds seated, one per town. Both chairs are already in
  `data/shops.txt` (Oak, Weston) tagged `Visitor+`.
* Town Earrings: Lv 57, awarded at Citizen of Honor, untradeable.
* Ten NPCs per town greet with different dialogue as the grade rises - which is the same
  machinery as goal B's idle chatter, keyed on grade.

##### Dependencies

Blocked on **D (level up)** for the level gates to mean anything, and it shares the quest
machinery with **A**. But the quest data is already extracted and the gates are already
expressible: `crates/store` needs a per-character `(town, grade, contribution)` and
`crates/world` needs to honour `Check.citizenshipTown`/`citizenshipGrade`, both of which are
small next to what is already built.

#### I. The bag has to persist - set by the owner, 2026-08-19

The owner, after the first successful unequip: *"when I travel to map 40, the item I un-equipped
re-equipped itself. This is not correct, items taken off should persist as is during
transitions from map to map."*

**Today the unequip works on screen and nowhere else.** `0x0107` is answered with a `0x0070`
and the client moves the item, but the server writes nothing down: the equipped list in the
character record is still built from `crates/store`'s `equipment` rows, so the next
`SetField` - a portal, a `!map`, a relog - puts the item straight back on.

That was a deliberate choice with the wrong ceiling on it. The alternative available at the
time was to delete the `equipment` row, which would have left the item **nowhere at all**,
because nothing stores bag contents. "Comes back" beat "is gone". The owner's answer is that
neither is acceptable, and they are right - the move has to survive the transition.

##### Two halves, and the second is the real work

**1. Somewhere to put it.** An `inventory` table keyed by character - `(character_id,
inv_type, slot, item_id, count)` - and `on_inventory_move` writing to it: delete the
`equipment` row, insert the inventory row. Small, and it follows the idioms already in
`crates/store/src/character.rs` and `quest.rs`.

**2. Getting it back onto the wire, which is not decoded.** The character record's
`presence[2]` region ends with four `u16` terminators that this server sends as zero:

```rust
b.extend_from_slice(&0u16.to_le_bytes()); // end of the equipped list
b.extend_from_slice(&0u16.to_le_bytes()); // FUN_14030b6f0
b.extend_from_slice(&[0u8; 6]);           // FUN_14030b9e0, three lists
```

Those four are **the bag inventories**, sent empty. `research/naked-character.md` established
that setting `presence[2]` opens three list readers rather than one, and that omitting the
terminators desynchronises everything after them - but nothing has read what a **non-empty**
list looks like. Until that is decoded, an item can be stored in the database and still not
appear in the bag after a field entry, which from the player's side is indistinguishable from
losing it.

Existing material to start from, not yet worked: `research/msexe-invlist-b6f0.txt`,
`research/msexe-invlist-b9e0.txt`, `research/msexe-invkey-23d0.txt`, and the equipped-item
blob decode in `research/msexe-setfield.md` §"The item blob", which gives the **bundle**
(`FUN_140304450`), **pet** (`FUN_140304550`) and **equip** (`FUN_140304100`) shapes - the
same blobs a bag list must carry.

##### It is the same table three goals are waiting on

**F (NPC shops)** cannot sell into a bag that does not exist, **G (storage)** is a second
container needing the same item representation, and this is the first. Whoever does it should
know they are unblocking all three, and should design the table for all three rather than for
the unequip alone.

##### The one thing already proven

`0x0070` InventoryOperation mode 2 moves an item on screen, confirmed by the owner. So the
*outbound* half of any inventory change is settled - `crates/net/src/inventory.rs`. What is
missing is durability and the record block, not the ability to tell the client.

#### J. The damage formula - set by the owner, 2026-08-20

The owner: *"an integral part of our server"*, with three links. All three are captured in
**`research/meowdb-combat-formulas.md`** - the formulas, the weapon multiplier table, the
accuracy and avoidability model, the defence order, the crit rule and the site's own list of
what it could not resolve.

**Today the client computes its own damage and the server accepts it.** That is measured, not
assumed: `research/mob-combat.md`. So this goal is not "make damage work" - damage works. It
is **making the server the authority**, which matters the moment anything is meant to be
balanced or resisted.

The one thing to know before starting: the formula needs `TotalWATK`, mastery, the weapon's
type and the four stats, and **the server already has all of them** - equipment carries real
`Character.wz` stats (`research/equip-stats.md`) and the stat block is decoded. What it does
not have is a notion of *which attack* was used, which is in the attack packet nobody parses
past its damage numbers.

Do the **[I]** labelling honestly here. The site is a fan site with a good prior on this
client - see the note at the top of that file - not a listing.

#### K. HP and MP per level - **DONE 2026-08-20**, and the client cannot check it

| class | HP / level | MP / level |
|---|---|---|
| Beginner | +16 | +12 |
| Warrior | +28 | +12 |
| Bowman | +22 | +17 |
| Thief | +22 | +17 |
| Magician | +16 | +22 |

Plus a fixed 500 points split at job advancement, and +25% of base from the maxed Improving
Max HP / Max MP skills. Full table and the job split in `research/meowdb-combat-formulas.md`.

**This contradicts what MapleCW ships.** `crates/world/src/expcurve.rs` gives every character
`LevelGains { ap: 5, max_hp: 14, max_mp: 10 }` at every level. A Beginner should be getting
**+16 / +12**.

**Neither number is measured.** Ours was a placeholder; the site's is a fan site.

**And the client will not settle it, which this entry claimed yesterday that it would.** That
claim was an inference and it is now checked: the EXP curve at `0x143AC2400` has **no sibling
table**. A `lea reg,[rip+disp32]` scan of the whole 0x500-byte neighbourhood that fills it
finds exactly one `.data` target, and it is the EXP curve itself. That fits what the client is
for - it draws the EXP bar, so it needs the EXP table; it is *told* new maxima on level up and
never computes them, so it has no reason to carry HP/MP per level at all.

The blind spot, named rather than left implicit: that scan cannot see a table reached through
a register, through `mov reg, imm64`, or from `.themida`. So this is "no evidence of one in
the obvious place", not "provably absent".

Which leaves the fan site's behavioural measurement as the best evidence available, and it
is written down **as that** - a measurement of the live COT2 service, not a listing.

**Implemented.** `LevelGains::for_class` carries all five lines and `ClassLine::of_job` keys
them on the hundreds digit of the job id, which is `[L]` from this client's own SP fork
masks. A beginner now gains **+16 HP / +12 MP** instead of +14 / +10, and since job
advancement is goal E, that is what every character on this server gets. AP stays at five and
is ours, not theirs.

Two things deliberately left out, because neither has anything to hang off yet: the 500-point
**job-advancement bonus** and the **+25% from maxed Improving Max HP/MP**. Both are in
`research/meowdb-combat-formulas.md`. Adding an unwired table for them is the failure mode
`CLAUDE.md` calls "built is not wired".

Pirate (`500`) has no row in the source and falls back to the beginner line. That is a
placeholder and is labelled one - nothing can reach it today.

Five constants and a job split. It is the smallest of the three and the only one that changes
something a player would notice today.

#### L. Attack speed and animation timing - set by the owner, 2026-08-20

Captured in `research/meowdb-combat-formulas.md`, and the honest summary is that **the page
does not carry the table**. It gives the model - timing quantised to **30 ms steps**, four
labels (base animation, animation time, weapon speed, timing range), Weapon Boosters worth two
speed stages at every level, Spell Booster worth one at Lv1-10 and two at Lv11+ and not
available to Clerics or Priests - and then points at per-skill pages for the numbers.

**The server does not check attack intervals at all today**, and on a single-player local
server it does not need to. This goal is here because the owner asked for it and because the
numbers have to come from the client's own WZ rather than that page if they are ever wanted.
Lowest priority of the three, and say so rather than quietly leaving it out.

### RUN OF 2026-08-19: the equipped list DECODED, and the mob body kills the client

The owner entered the world with `TestCharD` on **map 40** and the client exited immediately.
Preserved as `research/fixtures/mob-body-faults-client-{world,hook,exit}.log`.

**This run answered three questions and only one badly.**

```text
14:27:49.788  140302e30 x3  from 0x1403094d0   character select - THE POSITIVE CONTROL
14:27:52.421  140302e30     from 0x140304e76   while dispatching 0x01A0: the record decoded
14:27:52.421  140304100 x4  from 0x140309686 <- 0x140306223 <- 0x140309f70
14:27:52.951  opcode=0x044F  dispatched
14:27:52.952  opcode=0x044F  dispatched
14:27:52.952  CLIENT FAULT 0xC0000005 at 0x141c810b0
```

**The equipped-item block works.** `140304100` is the type-1 equip decode at `vtable+0x358`,
and it fired **four times** - once per stored item - from inside the equipped-list loop at
`0x140306223`. Its `rcx` points at an object whose first qword is `0x14327E1D8`, the type-1
vtable, exactly as `research/naked-character.md` predicted. So `presence[2]`, the block's
position at record offset 223, the `u16` slot loop and the 125-byte item body are all
**right**, and the record decoded past them into the NPC dispatch.

**What is still unknown is whether the character LOOKS dressed**, because the client died
before the owner could see it. Decode confirmed; appearance not.

**The mob body faults the client, and the fault names the field.** `0x141c810b0` is
`+0x70` into `FUN_141c81040`:

```asm
141c81094  MOV   RAX,qword ptr [RSI + 0x2b8]
141c8109b  MOV   EDX,0x848
141c810a0  TEST  RAX,RAX
141c810a3  LEA   RCX,[RAX + 0x828]
141c810aa  CMOVE RCX,RDX              ; RAX == 0  ->  RCX = 0x848
141c810ae  XOR   EDX,EDX
141c810b0  CMP   qword ptr [RCX],RDX  ; <-- faults reading 0x848
```

So **`mob+0x2b8` was null** and the client dereferenced it with no guard. It died on the
**first** `0x03C6`, after both `0x044F` NPCs had dispatched cleanly - so mobs are the only
thing that changed the outcome, and the other three builds are not implicated.

This is the failure `research/mob-spawn.md` ranked **first**: the body is structurally wrong
somewhere, and the WZ-template-driven blocks (`template+0x104` adds 16 bytes, `+0x1a0` adds
4) are the leading suspect, because a wrong length there misreads every field after it -
which is exactly how a pointer field ends up null.

> **Next instrument:** find who *writes* `template+0x104` and `+0x1a0` and read the WZ
> property name beside it, or dump `Mob.wz` template 2 directly and look for the properties
> that would set them. Then find what assigns `mob+0x2b8`, which names the field that did
> not arrive.

**Mobs are OFF by default now** (`send_mobs`, `--mobs` to re-enable), so the next run gets
the other three builds in front of the client without waiting for this. The world server
prints a line saying so on startup.

### Mob capacity: a spawn point is not a mob

The owner, 2026-08-19: map 40 has 40 spawn points but a real server keeps about **30** alive on
it, and the cap rises with player count.

**Checked in the WZ, and it is not there.** Map 40's whole `info` node is `AmbientBGM(v)`,
`MR*`/`VR*`, `bgm`, `cloud`, `fieldLimit`, `fieldLimit2`, `fieldLimit_tw`, `fieldScript`,
`fieldType`, `fly`, `forcedReturn`, `hideMinimap`, `mapDesc`, `mapMark`, **`mobRate`**,
`moveLimit`, `noMapCmd`, `onFirstUserEnter`, `onUserEnter`, `partyStandAlone`,
`personalShop`, `quarterView`, `returnMap`, `standAlone`, `swim`, `town`, `version`. **No
capacity field of any name.** `mobRate` is `1.0` and is a respawn *rate*, not a cap. Its
`life` node has 42 entries: 40 mobs plus Robin and Sam, which also confirms the generated
table.

So the cap is **server policy**, and the owner adopted the fan site's rule blanket on
2026-08-19: **75% of the spawn points below six players on the field, 100% at six or more**,
with nothing in between. `world::config::spawn_capacity(points, players)`.

**[I], and adopted knowingly** - the owner flagged the source as an unofficial fan site themself.
Nothing in this client corroborates it. Two things are still unsettled and are written down
rather than smoothed over:

* **The rounding.** The one datapoint is 40 -> 30, which both flooring and rounding up
  reproduce. The code floors. A small map is where they differ: 6 spawn points give 4 by
  flooring and 5 by rounding up.
* **The threshold's edge.** The site's columns are labelled "Solo" and "6+ players", so
  `CROWD_THRESHOLD` is **six or more**. The owner's wording was "more than 6". If strictly more
  was meant, that constant is the single number to change.

**The crowded branch is written and untaken.** `players` is the count on the *field*, and it
is always `1` today because this server has no field-occupancy tracking at all. It is a
parameter rather than a constant so that adding occupancy is a change at the call site.

The full spawn-point list stays intact in `Config::mobs`; the cap is applied where they are
*sent*, because respawn will need the points that are not currently filled.

**And which points are filled matters as much as how many.** The owner, 2026-08-19: *"on maps
with multiple mobs, there's a concept of shares, the map will try to maintain the balance
ratio between the mobs under the cap."* Taking the first N in WZ order is wrong and was the
first thing written here - the generated table is grouped by spawn index, so on The Field
South of Ellinia (45 spawns: Snail 10, Blue Snail 16, Shroom 7, Red Snail 6, Orange Mushroom
6) the first 33 rows **miss a whole type**. `config::share_balanced` apportions the cap by
**largest remainder**: `floor(count * cap / total)` each, leftovers to the largest
remainders, ties broken by template id. For that map at cap 33 it gives 7 / 12 / 5 / 5 / 4,
every type within one slot of its exact share. A test pins both the split and the fact that
the naive version drops a type.

**[I] again, and only the shape.** That the engine balances by share is the owner's, from the
same fan site as the capacity scalar, and the exact rounding the real engine uses is
unknown. What the code guarantees is that the result is capped, proportional and stable
between runs.

**Not yet built: respawn.** A share-balanced *refill* when a mob dies is the same rule
applied over time, and nothing here kills mobs yet.

### THE RUN - what to do, in this order, and what each outcome means

All four builds are pre-flighted: `cargo test` (210), `channel_smoke.py`,
`channel_smoke.py --set-field-probe` and `login_smoke.py --spawn` all pass, so framing, the
cipher and every field offset are already checked over an independent Python transport. What
a launch adds is the only thing those cannot: **what the client does with the bytes.**

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

**`-SetFieldProbe` is not optional and the name is a lie.** `Session::handle` returns
`Vec::new()` for every packet unless it is on, so without it the channel answers **nothing at
all** - not even the migration hello - and the client sits on "Connecting..." with a frozen
UI. It gates the whole working channel now: the `SetField`, the NPCs, the mobs, the portals,
chat and the quest reply. It stopped being a probe some time ago.

**Do them in this order. The last one can end the session.**

| # | do | what to look at | what it means |
|---|---|---|---|
| 0 | Log in and enter the world with a character that has equipment | does a character appear on the map at all? | **This is the gate.** The equipment change is inside the character record, which has no length prefix and no resync point. If world entry breaks - a fault, or a freeze on "Connecting..." - the record desynchronised, and **nothing below can be observed**. Read the `ELog` (`0x008F`/`0x0090`) and run `tools/pdata_lookup.py` on its RVAs; that names the mis-sized field |
| 1 | **Hover the TROUSERS (slot 6, item 1060002) and read the tooltip** | the Grey T-Shirt should now say **Weapon Def.: +6**, **Remaining Enhancements: 7**, and **no** "Cannot be Traded when equipped". The starter sword should show **17** attack | Being dressed is already confirmed; what is new is what each item *says*. **No stat line at all** - the packet value is not what the tooltip reads. **A wrong number** - the bit order is off, and which stat shows which number names the bit. **Fault or freeze** - the record desynchronised; items are **129** bytes now and the record **759**, so a width error is live again. **Answer this even if nothing changed:** are the `Remaining Enhancements` and `Scissors Usages Available` lines present *at all*? They sit behind the same `ITEMINFO` gate as the stat lines, so "those two are there and the stats are not" and "all three are gone" are completely different diagnoses |
| 2 | ~~Walk to map 30~~ **SKIP - mobs are off.** | - | The mob body faulted the client on 2026-08-19 and `send_mobs` is now `false`. Re-enable with `--mobs` only when the mob body is the variant under test |
| 3 | Click a **quest** NPC (Heena, map 1) **and** a **quest-less** one (Robin, map 40) | does a dialog box appear for both? | They take different paths through the client - Heena's click sends `0x0151`, Robin's `0x00F2` - and only the first was answered before, which is exactly why Robin was silent. **Text on both** - the whole chain works. **Nothing, no fault** - check `world.log` shows the `0x055B` going out, then suspect the message type or the flags |
| 4 | Open Change Channel | is CH.2 **cream** rather than grey? does clicking it turn it blue? | Cream means the enable byte is right. Blue on click is only a highlight move, not a send |
| 5 | **LAST.** Click the Change button | anything | **A freeze here is the measurement, not a crash.** Nothing answers `0x00D2` yet, and an unanswered packet freezes the client's whole UI - including the quit prompt's OK. `world.log`'s last inbound line names the packet, which is what this step is for |

**One variant at a time still holds** - these are four disjoint subsystems with four disjoint
observables (the record, a separate pool, a reply to a click, the login world list), so a
failure in one does not explain a failure in another. The single exception is step 0, which
gates everything.

**Two rules that have each cost a run:**

* **Never send a script (`0x055B`) with or just before a `SetField`.** Field entry runs
  `FUN_142caa4e0`, which resets the script manager.
* **The character record has no length prefix and no resync point.** One wrong width
  desynchronises everything after it, silently.

### 1. NPCs show up on maps - **DONE 2026-08-19**

Visible and clickable. `NpcEnterField` is `0x044F`; the routing was right from the start and
the body had two zeros that mattered - **read 12 is `isEnabled`** and **read 19 is `alpha`**,
so every NPC was created, pooled, disabled and fully transparent. No fault, nothing in any
log. See the doc comment on `net::opcode::npc_enter_field`.

NPCs come from `gm-handbook/npcs.txt`, generated by `tools/dump_portals.py` out of each
field's WZ `life` node: 308 across 150 maps.

*Original notes kept below, because the routing work is still the reference for goal 2.*

### 1a. How the NPC routing was established

**Built, not working yet.** `NpcEnterField` is **`0x044F`**, a fixed **64-byte** body, routed
through **`FUN_141820080`** (`0x1a4..0x5ab`) to the NPC pool dispatcher `FUN_141e75800`
(`0x44F..0x468`). `FUN_141820080` is a dispatcher this project had not found; it fills the
gap `research/msexe-gamestage-dispatch.md` left open. Full layout: `research/npc-spawn.md`.

Settled, not assumed: **the client cannot spawn NPCs itself.** Its field loader walks the WZ
`life` node only to preload `Npc/%07d.img` art; the only code that builds a populated NPC
takes a `CInPacket *`. And the pool is **destroyed and rebuilt empty on every field entry**,
so NPCs must be re-sent after *every* `SetField`, not once.

The trigger was wrong and is fixed: `0x0238` fires **only on the very first field entry** -
measured, three portal walks produced none - while **`0x00DC`** arrives once per `SetField`,
every time, ~420 ms after. It is now `0x00DC`.

> **Next step:** one launch. The probe watches `141e75800`; if it fires while dispatching
> `0x044F` the routing and timing are right and the **body** is wrong, and if it stays silent
> the packet is never dispatched at all.

Also needed regardless: the NPC table is a **stub covering map 1 only**. The real data is
every field's WZ `life` node, and it belongs in a generator beside `tools/dump_portals.py`.

### 2. NPCs have dialogue when clicked - **DONE 2026-08-19, confirmed on screen**

NPCs speak the game's own lines on **both** click paths, quest NPCs open the real quest
dialogue, and pressing **Accept** answers with the quest's `yes` branch. What is *not* done
is quest **state**: nothing persists, so the same conversation is available every time. That
is goal A.

*Original notes below - the identification is still the reference.*

`crates/world` answers `0x0151` with a `0x055B` type-0 Say, spoken by **the NPC template the
client itself named** - by construction a real `Npc.wz` id, and a bad one costs the portrait
rather than faulting (the loader result is null-checked at `142a7b52a` and falls back to
`[ui+0x6f0]`). The builder is `net::script::npc_say`; the request parser is
`net::script::parse_quest_request`, and no parser for `0x0151` existed anywhere before.

**Verified without a launch:** `python tools/channel_smoke.py --set-field-probe` now sends
the real captured 17-byte click and checks the reply is one `0x055B`, speaks as template 1,
has `hasOverride = 0`, has message type 0, and that the body length is exactly
`20 + textLen + 6`.

> **What to watch on the run:** click Heena on map 1. **A dialog box with our text** means
> the whole chain works. **Nothing at all, and no fault** means the message was built and
> torn down, or the type/flags shifted the body - check `world.log` for the `0x055B` going
> out first. **A freeze** would be new: `0x0151` has never blocked before.

**It is text on screen and nothing more.** No quest-result packet has been found, so no
state advances - accepting the same quest twice shows the same message, and the message
says so.

**Two fields change the body length with nothing to resync on**, `hasOverride` and
`flags & 0x04`, so the flag bit is derived from the value rather than set by hand.

**One correction to `research/npc-dialogue.md`**, from re-deriving off the listing rather
than trusting the file: the second style bit is **`0x80`**, not `0x40`. `141f6fbcb` is
`movzx ebx, sil / shr ebx, 6 / and ebx, 2`, and `(0x40 >> 6) & 2 == 0`. The formula two
sections later was already right; the prose gloss was not.

*Original notes below - the identification is still the reference.*

### 2-orig. How the request was identified

The owner clicked Nina, Roger and Heena on 2026-08-19 and the client named its own request, the
way it named the portal. **`0x0151` is the NPC interaction packet**, captured in
`research/fixtures/npcs-visible-quests-clicked-world.log`:

```text
0x0151  01 e8030000 01000000 0c046d01 00000000     13-17 bytes
        ^  ^         ^         ^
        |  objectId  templateId  x,y as two u16
        a leading u8 that varies (seen 01 and 04)
```

The object id matches one we assigned (`1000` = map 1's Heena) and the template id matches
the WZ, so the first three fields are **read off our own data**, not guessed.

**The reply is inbound `0x055B`, the script message.** Routed by `CField::OnPacket`
(`FUN_141820080`) - the same range chain that already delivers our working `0x044F` - to
`FUN_141f6f350`. Head: `u32 handle, u8, u32 speakerNpcTemplateId, u8 hasOverride,
[u32 override], u8 messageType, u16 flags, u8`. Type **0 = Say**: `u32 echo,
[u32 speakerOverride if flags & 4], str text, u8 prev, u8 next, u32`. The speaker field is
**read, not inferred** - it lands in `[ui+0x2cc]` and is fed to `FUN_141e77b70`, the NPC
template loader `npc-spawn.md` already identified. Full working: `research/npc-dialogue.md`.

**Ordering constraint that would have cost a run:** `FUN_142caa4e0` - the routine that emits
`0x0238`/`0x024D` on field entry - resets the script manager. **Never send a script with or
just before a `SetField`.**

> **Next step:** build the 32-byte minimum-viable Say and send it in answer to `0x0151`. The
> values that matter are in the research file: the speaker template must be a real `Npc.wz`
> id (`0` is not one), and `hasOverride` and `flags & 4` each change the body length with no
> resync point. **Text on screen is still not a quest** - no quest-result packet has been
> found, so state will not advance.

### 2f. THE MOB WATCH FIRED, and it says there are two objects

Run of 2026-08-19 with `-Mobs -MobLimit 1`, one mob on map 40 (`template 2, object id 2000,
hp 45, 137 bytes`). Client died on entering the world. Preserved as
`research/fixtures/mob-watch-2b8-null-second-object-{hook,world}.log`.

```text
WATCH #1: 0x141c81040 ENTERED while dispatching opcode 0x03C6
  rcx=0x382a9760 [0x43407950]  [rcx+0x2b8]=u32:0x00000000  called-from=0x1409c687d
  stack: 0x1409c687d 0x142f0492d 0x140c79143 0x142ac10d0
         0x142ac1128 0x142f0492d 0x141c81297 0x142ac10d0 0x141c50da8
CLIENT FAULT #1: 0xC0000005 at 0x141c810b0
```

**`mob+0x2b8` is confirmed null** - the diagnosis holds, and the watch did in one hover what
a direct-call graph could not do at all.

**SOLVED. There is no second object, and the field named here is the wrong one.** See the
correction immediately below; the reading in this section is kept because the *measurement*
is what solved it.

**What this looked like at the time:** `0x141c50da8` is inside
`encodeInit` and **past** the `+0x2b8` assignment at `0x141c50c9c`. So `encodeInit` is running
and has already executed that assignment - for *some* mob - while the mob in `rcx` still holds
the constructor's zero. The reading that fits is **`encodeInit` initialising mob A reaches
`FUN_141c81040` on mob B**, which has not been initialised yet.

**Two more discriminations from the same line:**

* **The vtable low dword is `0x43407950`, and it is not one of the eight** the static pass
  predicted (`0x434077e8`, `0x4341f548`, `0x4341e510`, `0x4341e758`, `0x433765c0`,
  `0x4341ede0`, `0x433767f0`, `0x4341eff0`). Either that set is incomplete or `rcx` is not a
  mob at all. The claim it rested on - that the eight aligned `.rdata` qwords holding
  `FUN_141c81040` *are* the eight mob vtables - now needs re-checking.
* **The second watch never fired.** `141c532ab:peek=24` produced no line, so the body read at
  offset 107 never happened. That does not contradict the above; the fault simply came first.

`called-from=0x1409c687d` is `FUN_1409c50a0 + 0x17dd` (7184 bytes). `0x142f0492d` appears
**twice** in the chain, which has the shape of a trampoline or dispatcher, and `0x141c81297`
is past `FUN_141c81040`'s own `.pdata` end - so it is a different function, plausibly the
same virtual on a different class.

> **The shape worth looking at first**, because it is the only category that both fits "our
> packet cannot make `+0x2b8` null" *and* explains a second object appearing: a **value** in
> our body steering control flow. The 35 unconditional reads are verified identical to ours,
> so the layout is right - but `summonType` is `1` in both reachable templates' WZ while we
> send `appear_type = -2`, and the head carries two `u8`s whose meaning is unread. If one of
> them makes the client treat this as a summon **with a parent**, mob B is the parent it goes
> looking for and does not have.

**Mobs remain off by default.** Nothing about the body has been changed, deliberately: it is
the one part verified three ways, and changing it now would confound the only measurement
that has ever discriminated here.

### 2g. THE MOB CRASH, SOLVED - and it is one byte we send

**`0x143407950` is not a ninth vtable. It is the mob's own second base**, written by the
constructor at `mob+8` alongside `0x1434077e8` at `mob+0` and `0x1434079c0` at `mob+0x10`.
`FUN_141c81040` is slot 1 of that table, so `rcx = mob + 8` - and **`[rcx+0x2b8]` is
`mob+0x2c0`**, not the `mob+0x2b8` the whole first pass analysed. Two independent locks agree
on `this = mob+8`: `FUN_141c76190` and `FUN_141c81040` assert the *same* id `0x431` on
`mob+0x3c8` and `this+0x3c0`, and `encodeInit` reads the template at `[rsi+0x3a8]` where
`FUN_141c81040` reads it at `[rsi+0x3a0]`.

`mob+0x2c0` has exactly two writers: the constructor's zero, and **`141c50eed` in
`encodeInit`** - a *second* interface object, created at `141c50e77`. So `+0x2b8` and
`+0x2c0` are a **pair**, built `0x1c8` bytes apart, and the callback lands **between** them.

**The lever is `move_action`, body offset 35, and we were sending 0.** That byte is
`action*2 + facing` - `encodeInit` splits it twelve instructions later, `AND EDI,1` for
facing and `SAR EAX,1` for the action, into a 16-entry jump table. The chain:

```text
offset 35 = 0  ->  obfuscated into mob+0x3dc/0x3e0
141c50cfd  EDI = ROL([mob+0x3e0],5) XOR [mob+0x3dc]      the plaintext back
141c50da5  CALL iface->vtable[0x118], EDI as arg 7        <-- DOMINATES the tail reads
1409c6858  TEST EBX,0xfffffffe                            <-- only action == 0 falls through
1409c687a  CALL [mob8_vtable+8] = FUN_141c81040 -> mob+0x2c0 is still null -> 0x848 -> dead
```

`TEST EBX,0xfffffffe` is the whole answer: **any action >= 1 skips it.** The call at
`141c50da5` dominates the tail, so the callback itself is unavoidable - only the value is a
lever. `net::mob::MOVE_ACTION_MIN_SAFE` is `2`, `FieldMob::new` defaults to it, and both a
unit test and the smoke test pin it.

> **The pass condition on the next run:** **no `141c81040` line at all**, and `141c532ab`
> firing with cursor **`0x71`**. If the stance looks wrong on screen, `3` is the same action
> facing the other way and `5` is action 2 - all of them skip the fault, so that is a *look*
> question, not a crash question.

**An instrument correction that matters beyond this.** The probe's `stack:` line is a
**heuristic scan of the stack for code-shaped values, not an unwind.** In the capture,
`0x142ac10d0` is a function *start*, which no return address can be, and `0x140c79143` is the
return of an indirect call in an unrelated function - both stale. **Only `called-from=` is
exact.** The "two objects" reading in §2f was built partly on those frames.

**And the first pass's own retraction, which is the useful part:** it blamed `mob+0x2b8`,
proved from a dominator test that no packet byte could make it null, and was right about that
field and wrong about which field was being read. The tell was a "slot 46/47/48" wobble it
noticed and explained away instead of chasing - offsets into *secondary* vtables.

### 2e. RUN OF 2026-08-19 (evening): what it settled, and the best lead yet on unequip

Preserved as `research/fixtures/stats-work-then-map-loses-them-world.log`.

**Working on screen, confirmed by the owner:** idle chatter (Robin cycling their lines), item stats
on the tooltips, the `!map` chat notice for both a bad id and a good one, and **Log Out**
returning to the login screen.

**The tooltip investigation was chasing my own bug.** Items had correct stats on entering the
world and lost them the moment `!map` was used, because `go_to_map` still sent the *bare*
record. Every earlier tooltip observation was made on map 40, reached with `!map`. There was
no second object.

**World select cannot change channels, and that hypothesis is dead.** The owner: *"So I actually
cannot change channel via world select. This client doesn't implement that. When I go to
world select, the moment I choose 'Classic', I am redirected to the classic login screen with
my masked email."* So the run where CH.1 and CH.2 listed is still unexplained, and **all
three** attributions so far - the four trailing bytes, the enable byte, the world-select
route - are retracted. Whatever populates that list, nothing yet proposed does it.

#### The unequip still never reaches the wire, and there is now a strong candidate why

The owner tried repeatedly to unequip the Undershirt. **The capture contains no `0x0107` at all**,
confirming `research/npc-click.md` §4: the client drops it inside `FUN_142cc5b00`, before
building anything, at one of six pre-send gates.

> **Leading hypothesis: the bag has no slots.** An unequip needs somewhere to put the item,
> and until 2026-08-19 the server never told the client how big any inventory was.

**BUILT the same day, and the hypothesis above was right about the symptom and wrong about
every detail of the mechanism.** The full working is `research/inventory-slots.md`; what
this section had wrong is worth keeping, because the two errors are the two this project
keeps making.

* It said the counts sit at **record offsets 219-222**, in the bytes labelled "one `u8` and
  three optional-string flags". **They do not.** They are at offset **223**, in a block that
  did not exist in the layout at all, switched on by `presence[7]`. The old labelling of
  219-222 was never wrong - it just was not the thing being looked for.
* It said there are **five**. There are **six**: the loop's trip count is the literal
  `MOV R15D,0x6` at `0x140305def`.

Both errors came from the same place - reasoning from the reference server's shape instead
of reading this client's loop - and both were the *reference being right about the concept
and wrong about the numbers*, which is exactly the failure `CLAUDE.md` scores it 1 of 8 for.
Ten minutes on the listing gave the right answer, and the listing had been sitting in
`research/msexe-charrecord-full.txt` the whole time.

> **What is still open** is the part that mattered: whether a null inventory array is why
> the unequip never reaches the wire. The arithmetic is measured - a null array makes the
> default slot count `-1`, and `CMP dword ptr [RSP+0x60],0x0 / JL` at `0x140305f09` then
> skips that inventory's whole slot walk - but **whether these arrays start null has not
> been read**, and it lives in a constructor nobody has opened. **[I]** with a mechanism.
>
> **The one-variant test: `-InventorySlots 10`.** Go UNDER the default, not over. The owner's
> inventory window is **six rows of five** with a scrollbar, so at any number of 30 or more a
> fixed viewport and a real slot count look identical on screen. At 10 they do not.
>
> **And the screenshot already weakens the hypothesis**: those thirty cells were drawn in a
> session where the server sent no sizes at all. Either the arrays were never null - in which
> case this explanation is wrong and should be dropped - or the grid is a viewport and the
> screenshot says nothing either way. The run at 10 is what separates them.

#### And a client state dump nobody had seen: `0x0420`-`0x0426`

Six packets arrive together, once, ~7 minutes in. `0x0421` is **1115 bytes** and carries the
character id (204), the name `TestCharD`, and **our four item ids** - `1040003`, `1060002`,
`1072003`, `1302000` - in equipped-slot order. `0x0420` carries a timestamp string and the
**NPC object ids we assigned** (1000, 1001, 1002). `0x0426` is 20 bytes beginning
`ffffffffffffffff`.

This is the client reporting its own world state back, and it is a **free read-back
instrument** of exactly the kind `research/equip-stats.md` wished for: it says what the
client thinks it is wearing, in its own words. Nobody has decoded it. None of the six is
answered and none has caused a freeze.

### 2d. The unequip request never reached the wire - and the gate that ate it is a hazard

The owner tried to unequip the Undershirt on 2026-08-19 and was not sure whether anything was
sent. **Nothing was.** The move request is outbound **`0x0107`** - `FUN_142cc5b00(player,
invType, src, dst, count)`, body `u32 tick, u8 invType, i16 srcSlot, i16 dstSlot, i16 count`,
11 bytes, with **negative slots meaning equipped** (read, not assumed: `test r14d,r14d; js`,
`cmp r14d,-0xb`, `cmp edi,-0xb`). Their action would have been `invType=1, src=-5`. It is not
in the capture and nothing else in that session has the shape - every other captured opcode
is accounted for against `research/msexe-packet-fields.txt`. The client dropped it inside
`FUN_142cc5b00`, before building anything, at one of six pre-send gates.

> **And one of those gates is an "always answer" landmine that has nothing to do with this
> bug.** `player->[0x2330]` is a **one-request-outstanding latch**: set immediately after the
> send at `142cc5f01` by **37** functions across the image, and cleared by only **7**, *all
> of which are inbound packet handlers* - `FUN_142cc52a0` is literally
> `read(packet); [this+0x2330] = 0`. So if the server ever fails to answer one of those 37
> requests, **every later request in that class is silently dropped** - no dialog, no freeze,
> nothing in any log. This is not what happened here (none of the 37 setters appears in the
> capture), but it is the mechanism by which a single unanswered packet turns into "the
> inventory stopped working" three minutes later.

**Next instrument, and it needs no new code:** `WATCH` on `0x142cc5b00` plus its two exits
`0x142cc5c16` (bail) and `0x142cc5ea3` (`mov edx,0x107`). One drag of an item distinguishes
"the UI never asked" from "a gate blocked it" from "it sent and the capture is wrong".

### 2c-ii. Two NPC-click packets, and the server answered the wrong one - **FIXED, unconfirmed**

The owner clicked Robin on map 40 and nothing happened. The capture has **no `0x0151` at all**;
the client sent **`0x00F2`** and nothing answered it.

**Which one goes out is decided inside the client, from `Quest.wz`.** `FUN_1428de280` forks
on `FUN_141e39b50(npc)` - whether the NPC has a non-empty script name, a string the *client*
fills at construction from its own quest singleton - and only a menu line carrying a quest id
in `npc->[0x200]/[0x208]/[0x210]` reaches the `0x0151` builder. **Every other outcome sends
`0x00F2`.** Robin (template 8) has no quests. The server does not select the path and cannot.

`0x00F2` is `u32 npcObjectId, i16 charX, i16 charY, u32 tail`, 12 bytes. Answered now with
the same `0x055B` Say. Full working: `research/npc-click.md`.

**The trap in answering it**, which would have produced an unexplainable failure: `0x0151`
hands over the NPC's **template** id and `0x00F2` hands over the **object** id we chose,
while `0x055B`'s speaker field wants a template. The handler maps back through the table
that assigned it, **scoped to the character's current map**, because `config::load_npcs`
restarts the numbering on every field. Sending the object id straight through would not
fault - the loader result is null-checked at `142a7b52a` - it would just draw a box with no
portrait.

**Two corrections to the first reading of that capture**, both from the client's own builder
rather than from the numbers lining up:

* **Field 1 really is our object id** - `[npc+0x190]`, written from `NpcEnterField`'s
  objectId by the `CNpc` constructor at `141e35f59` and by the body decoder at `141e36b73`.
  The code reads back the field the spawn packet wrote. That distinction matters, because
  the *previous* "read off our own data" claim here was retracted for exactly the reason it
  invites: every map's first NPC is given object id 1000, so a `1000` proves nothing alone.
* **Fields 2 and 3 are the CHARACTER's position, not the NPC's.** The first reading called
  `275` Robin's `cy`; it is the player's `y`, and Robin's `cy` is 275 only because the player
  was standing on the same ground line. All four build sites read the local-user singleton,
  and the same pair appears verbatim in both `0x00D9` packets of the same capture.

### 2a. Channel swapping - two channels now run

The owner, 2026-08-19: *"In the classic world startup, the user is defaulted to channel 1 of the
server. We're not trying to change that behavior, we're trying to allow the client to swap
channels from 1 to 2 and vice versa."*

**UPDATE 2026-08-19: the dialog opens but lists no channels**, and opening it sends
**nothing** - so the list is built entirely from login data, client-side. The server did
advertise two (`world Scania id 0 with 2 channel(s)`, both addresses logged) and the world
list went out, so the count reached the wire.

The likely cause, now fixed but **untested**: every channel entry's four trailing `u8`s were
zero, which told the client each channel was **channel 0 of world 0**. They now carry
`[world_id, index, 0, 0]`. That the client reads exactly four bytes there is **[L]**, read
from its own decoder `FUN_141b2fac0` (the login stage's `case 0xb`); that they mean
world/channel/adult is **[I]** from the packet family's usual shape.

**UPDATE 2026-08-19, second run: the list now shows CH.1 and CH.2** - the
`[world_id, index, 0, 0]` fix worked. **But CH.2 cannot be selected**, and clicking it sends
**nothing at all** (`research/fixtures/channel-list-shows-two-but-unselectable-world.log`,
whose whole inbound set contains no new opcode). So the client is refusing the selection
**client-side, before it would send anything** - this is not an unanswered-packet freeze.

### THE CHANNEL LIST IS NOT ABOUT THE PACKET AT ALL - 2026-08-19, from two captures

**Both of my explanations for this dialog were wrong, and the world list was never the
variable.** Comparing the captured `0x000B` bodies settles it: the world-list body from the
run where **CH.1 and CH.2 were listed** and the body from the run where the dialog was
**empty** are **byte-identical**.

```text
run WITH channels listed  0006005363616e6961000000000208005363616e69612d30...0001000000000000000000
run with an EMPTY dialog  0006005363616e6961000000000208005363616e69612d30...0001000000000000000000
```

Both are `research/fixtures/world-select-0076-login.log` and today's `login.log`. Same world
id, same name, same channel count, same four trailing bytes per entry. So no value in that
packet decides whether the dialog has rows.

**What differs between the two runs is the route through the login screens.**

| capture | `0x0076` world-select requests | Change Channel |
|---|---|---|
| `world-select-0076-login.log` | **2** | **CH.1 and CH.2 listed** |
| today | **0** | **empty** |

The owner reached character select by auto-login today and never opened world select. In the run
that worked they had clicked "Choose another world" and picked Classic.

**That fits the static reading exactly, and rescues most of it.** `research/channel-select.md`
established that the rows are drawn from an `int` array at `singleton+0x2cc8`, filled by
`FUN_142cb8e10`'s sixth argument. Nothing in that chain is contradicted - what was never
checked is **who calls `FUN_142cb8e10`, and when**. If the world-select screen is what runs
it, then a client that skips that screen never populates the array, and the dialog is empty
no matter what the world list said.

> **The test costs nothing and needs no code change.** On the next run, click **"Choose
> another world"**, pick **Classic**, then enter the world and open Change Channel. If the
> rows are back, the answer is "world select populates the array" and the server owes the
> dialog nothing. If they are still missing, the route is not the variable either and the
> next instrument is a watch on `FUN_142cb8e10`.

**What this cost, and the lesson.** Two client runs were spent on a byte that never mattered:
first `[world_id, index, 0, 0]`, which was credited with fixing the list, and then the enable
byte, which was blamed for emptying it. The first "fix" was almost certainly a coincidence -
it landed in the same run as the world-select work - and the second was a coincidence in the
opposite direction. **A change and an observation in the same run are not a measurement**,
and neither of those runs changed one variable.

### SOLVED 2026-08-19: the predicate is the 4th trailing `u8`, and we were sending 0

Full working, every link read off the listing: **`research/channel-select.md`**.

**`FUN_142cb9510`** is a 15-byte leaf that returns `arr[i]` from the per-channel `int` array
at `singleton+0x2cc8`. It has six callers - Draw, the mouse hit test, the Change button and
three keyboard navigators - and **all six test the result with a bare `test eax, eax`**.
There is no enum and no magic value: **any non-zero int enables the row.** **[L]**

The chain from the wire:

```text
4th trailing u8 of the channel entry
  -> chan+0x18         FUN_141b2fac0, the login case 0xb - four sequential u8 reads into
                       +0x0c, +0x10, +0x14, +0x18
  -> vecB[i]           FUN_141b2c7c0 at 141b2ca89
  -> singleton+0x2cc8  FUN_142cb8e10, argument 6, at 142cb8ed9
  -> FUN_142cb9510(i)  must be non-zero
```

The mouse gate at `142a31810` takes `je` on zero, which is exactly "the click produces
nothing on the wire" - what the owner measured when they clicked CH.2 and the whole capture
contained no new opcode.

**RETRACTED 2026-08-19 by the run.** The one-byte fix - the channel entry ending
`[world_id, i, 0, 1]` instead of `[world_id, i, 0, 0]` - **emptied the dialog**. The owner: *"On
change channel UI, I'm back to no channels being shown again, it's completely blank."*

The count reached the client either way: that run's `login.log` has `channel 0 advertised`,
`channel 1 advertised`, `world Scania id 0 with 2 channel(s)` and the `0x000B` world list
going out. So the byte emptied a list that populated without it.

| 4th trailing `u8` | what the owner saw |
|---|---|
| `0` | CH.1 and CH.2 both listed. CH.2 grey, and clicking it sends **nothing at all** |
| `1` | **no channels listed** |

`crates/net` is back to `0`. Every individual link in the derivation still reads correctly -
`FUN_142cb9510` is a leaf returning `arr[i]`, its six callers all test with a bare
`test eax, eax`, the mouse gate `je`s on zero. What is falsified is the **end-to-end** claim
that the 4th wire byte arrives in that array as an enable flag: something between the
decoder and the array drops the entry when it is non-zero.

> **The next instrument is upstream, not downstream.** Read what `FUN_141b2c7c0` does with
> `chan+0x18` **before** it becomes `FUN_142cb8e10`'s argument 6, looking for a test that
> **skips the entry**. The failure mode to search for is "the list is built shorter", not
> "the row is drawn grey" - and nothing in `research/channel-select.md` looked for that.

**Which row is grey, confirmed rather than assumed.** The dialog loads four sibling canvases
`channel0..channel3`, all 68x20 - exactly the hit rectangle (`lea eax,[r9+0x44]` /
`lea eax,[rcx+0x14]`). Rendered from `_Canvas_000.wz`: `channel0` cream (normal),
`channel1` grey (current), `channel2` blue (selected), `channel3` grey (disabled). OnCreate
sets `selection := current` and Draw checks selection first, so **CH.1 draws blue**. The
current channel *is* excluded from clicking (`142a31803`), but that is not the cause -
**CH.2 is the grey one, and it is grey for the disabled reason.** The two greys differ by
about six RGB points, which is why they cannot be told apart by eye.

**`userCount` is ruled out, and the old advice here was wrong.** This section used to
propose a non-zero user count as the cheapest experiment. Enumerating *every* memory operand
in `FUN_141b2c7c0` and dropping the `rsp`-based ones leaves `[rax]` x4, `[rax+0x14]` and
`[rax+0x18]` and nothing else, so the `u32` reaches no gate at all.

**And the Change Channel opcode is `0x00D2`**, which this file listed as unknown.
`FUN_142a316e0`, the Change button, re-checks the same predicate and then calls
`FUN_1418287f0` - which `research/msexe-packet-fields.txt:86` already named as the `0x00D2`
builder. What was missing was that this is the Change Channel action. Body: `u8
targetChannel` (**0-based**, matching this repo's numbering), a `u32`, and the shared
14-byte `FUN_140c7b890` preamble. The field **set** is **[L]**; the field **order** is
**[D]** and unsettled - `0x00D1` calls that preamble first while `0x00D2` calls it last, and
there is no capture to decide. **Do not build the answer from that ordering** until a click
produces one.

> **The reply, once a click does send something.** Expect a migrate command: the swap has to
> mint a migration for the **target** channel, which `store::create_migration` already takes
> as a parameter, and hand back that channel's address the way `0x0011` does at login.

**Not** the world-list layout in general: the count reaches the client, the entries parse,
and both rows draw. Only selection fails.

**The startup channel is unchanged.** `world.channel_id` is still `0`, so a login lands
where it always did. What changed is that `tools/test-server.ps1` now runs **two** channel
processes by default (`-Channels`, one process per channel, `$ChannelPort + N`) and the login
server advertises both - `crates/login/src/config.rs` is explicit that you cannot advertise
more channels than you run, because the client connects to the address for the channel it
picked.

> **Watch the indexing.** The client's UI is **1-indexed**; everything in this repo is
> **0-indexed**. The client's "channel 1" is our channel `0` on 8485, and its "channel 2" is
> our channel `1` on 8486.

**What is not built is the swap itself, and this is now a live hazard.** The request is
**`0x00D2`** on the **channel** connection (above), and **nothing answers it**. Until the
enable byte landed, that did not matter, because the click never produced a packet. Now it
may.

> **An unanswered packet freezes the client's whole UI** - every button, including the quit
> prompt's OK. So on the next run, **click Change Channel last**. If the client freezes
> right after, that is the unanswered `0x00D2`, not a crash, and `world.log`'s last inbound
> line names it - which is the measurement this run is for. Do the equipment and dialogue
> checks first, because a freeze ends the session.
>
> The login flow cannot carry the swap: in mode 5 the client never sends a world/channel
> selection at all, going straight from `0x0080` to `0x0078`. Answering it will mean minting
> a migration for the **target** channel, which `create_migration` already takes as a
> parameter, and handing back that channel's address the way `0x0011` does at login.

### 2b. Newly identified packets - the client keeps naming its own requests

Every one of these came from the owner using a feature and the capture showing what went out.
That method has now identified four requests and cost no static analysis at all.

| opcode | what | body, as far as it is read |
|---|---|---|
| `0x00D1` | transfer field (portal) | fully decoded, `research/transfer-field-request.md` |
| `0x0151` | **quest request** | `u8 action, u32 questId, u32 npcTemplateId, [i16 x, i16 y], [u32 selection]`, builder `FUN_141f0e4c0`. **RETRACTED:** this was recorded as "NPC click" with the first `u32` as an objectId "read off our own data". It is a **quest id**. Our own logs disprove the old reading - every map's first NPC is given `object_id = 1000`, yet the client answered 1000/1002/1003/1005 for four NPCs, the same values in both sessions despite opposite visit orders. The listing agrees: that field keys six accessors on a quest table and is range-tested against 40000-40999 and 30051-30079. The **second** `u32` really is the template we sent, in all four captures |
| `0x00E7` | **chat** | `u32`, then a `u16`-length string, then a `u8` - `...05 00 "Hello" 03` |
| `0x0182` | **party create** | 68 bytes carrying the length-prefixed string `"TestCharD's Party"` |

None is answered yet. **None has caused a freeze**, so none is a blocking request.

### 2c. GM commands - `!map <id>`

Typed into any chat tab. Moves the character and persists it, so a relog stays put.

**The prefix is `!`, not `/`, and that is measured.** The owner typed `/map 1` and the session's
entire capture contains **no `0x00E7` at all**, while a plain "Hello" in the same tab had
produced one. The client parses slash commands itself - `/find`, `/whisper`, `/party`,
`/friend`, `/trade`, `/level` and `/h` are baked into the executable as strings - and an
unknown one is swallowed before it reaches the wire. A server-side command therefore has to
look like ordinary chat.

**Map ids are validated** against `gm-handbook/fields.txt`, the 426 maps with a real field
image in `Map.wz` - not against the name table, which disagrees with it in both directions
(12 named-but-absent, 6 present-but-unnamed).

**A refusal now says why, in the chat window.** It used to be silent, because there was no
outbound "tell the player something" packet - that is `0x00BB` (`u8 force, str text`) and it
is found and sent. `force` must be **1**: with `0` the client shows only the first line after
each field entry and drops the rest, which reads exactly like the feature being broken.
Confirmed on screen for both a bad id and a good one.

**No permission check, and there should not be one yet** - nothing on this server
authenticates and every connection is already the same account.

### 3. Mob spawns and mob drops - **priority, set by the owner 2026-08-19**

*"there should be tutorial monsters spawning on East Entrance to Mushroom Town (ID 30), can
we also put mob spawns and in turn mob drops as a priority please?"*

The foundation is in:

* **The data is generated.** `tools/dump_portals.py` now emits `gm-handbook/mobs.txt` from
  each field's WZ `life` node where `type == "m"` - **9928 spawns across 289 maps**. Map 30
  has its six snails (template 1), which is exactly what the owner expects to see.
* **The opcode is identified.** The mob pool is `0x3C6..0x44E` on the singleton at
  `[0x143ABFE00]`, dispatcher `FUN_141D30E80`, and **`case 0x3c6` calls `FUN_141d33630`** -
  mob enter field, the exact analogue of the NPC pool's `0x44F`.
* Mobs are **server-sent** for the same reason NPCs are: the client's field loader walks
  `life` only to preload `Mob/%07d.img`. `research/npc-spawn.md` established this for both.

### BUILT 2026-08-19 - and the premise this section rested on was wrong

`net::mob::mob_enter_field`, wired into `crates/world`. **137 bytes**: an 11-byte head, a
20-byte mask, and a 106-byte tail. `gm-handbook/mobs.txt` feeds it, so map 30 gets its six
snails and map 40 its forty.

**`FUN_14046fba0` is not a movement-path decoder.** This file described it as one, and that
framing is what made the body look unbuildable. It is `MobStat::DecodeTemporary`: a flat
list of optional fields, each gated by **one bit of the 20-byte raw block** that
`FUN_141c76190` reads immediately before calling it. `MOV R15,R8` at `14046fbbf` is the only
write to `R15` in all 10528 bytes, and the first loop bounds the bit index at
`CMP EDI,0xa0` = 160 bits = 20 bytes. **All 331 packet reads sit behind a mask-bit test, so
an all-zero mask costs zero bytes.** **[L]**

That scan was itself checked: a guard-interval pass covered 328 of the 331. The other three
(`140471db3`/`dc1`/`dd4`) sit behind an **OR of two bits** - `BT/JC take; BT/JNC skip` -
which the pass could not see because it only looked at the first conditional jump after each
test. Read by hand at `140471da4`. 331 of 331.

**Two readers the earlier pass had missed:**

* `FUN_141cc9410` was recorded here as "reads NOTHING". It calls `FUN_14085acd0(..., packet)`,
  which reads **57 bytes** unconditionally - gated by the `u8` at `141d33734`/`141d338fa`,
  so we send 0.
* The real body is behind a **virtual call**, `CALL [RAX+0x38]` at `141d33929` =
  `FUN_141c4ff80`: 52 reads, **106 bytes minimum**, and where x, y, foothold and HP live.
  All eight concrete mob classes share slot 7, and each vtable was reached through the
  constructor that installs it rather than by aligning tables - so this is not the
  `/OPT:ICF` trap.

**That function also found an eighth packet-read primitive.** Two instruments disagreed, 50
reads against 52. Sweeping *all* 116 direct call targets - enumerating rather than filtering
against the known list - turned up **`0x1406e8ef0`, a bare `JMP 0x1406e8b80`**, a second u16
thunk. With it both say 52. `docs/ghidra.md`'s table now says eight, and carries the checked
negative that **it appears in none of** `FUN_140304b20` (the character record),
`FUN_140304100` (the equipped item) or `FUN_141f6f350` (the script message) - so those
layouts stand.

> **RESOLVED, and it was the wrong suspect.** This paragraph ranked the WZ-template-driven
> blocks as the most likely first-attempt failure. They are **`patrol`** (`template[0x104]`)
> and **`targetFromSvr`** (`template[0x1a0]`), parsed by `FUN_14047d990` - and **none of the
> 193 mob images in this client carries either**. `Mob.ini` says `LastWzIndex|0`, so 193 is
> all of them, and the same search finds keys present in 1, 13, 37 and 193 images, so a zero
> is a real zero rather than a broken search. Templates 1 and 2 are both clean.
>
> **The body is not implicated at all.** Its 52 reads are `.pdata`-bounded, a dominator test
> picks out exactly **35 unconditional** ones, and those 35 are identical to what `mob.rs`
> emits. And `mob+0x2b8` is **client-side**: `encodeInit` fills it from a `QueryInterface`
> on a freshly allocated object, in a block that **dominates** body offset 107 - so any body
> that parses that far has filled the field. A 137-byte body cannot make it null.
>
> **Two corrections to what this file said about the run.** The fault landed **~23 ms after
> the send, inside the dispatch** - there is no `elapsed_us` line for any `0x03C6`, and that
> line is written after the trampoline returns - not half a second later; the two logs are
> stamped four hours apart in different zones. And it was the **first** mob, not the
> fortieth: `-MobLimit 1` reproduces it, so **volume is not the variable** and the
> blast-radius reasoning that flag was added for was based on a misread timestamp.
>
> So `+0x2b8` was null because either the virtual ran **before** `encodeInit` reached the
> assignment, or `encodeInit` **threw** first. That could not be settled statically: every
> step of the real path goes through a vtable, which a direct-call graph cannot see. Note
> also that **the absence of a C++ THROW line is not evidence** - the hook's threshold is
> 25 s and the fault landed at 11.7 s.
>
> **Next step is a measurement, not a change.** Changing a value or a width would confound
> the one thing verified three ways.

**The NPC lesson's mob equivalent is HP.** Zero is structurally legal and draws a mob at 0%;
the bar is `hp * 100 / maxHp` through an `IDIV` at `141c50502` with **no zero guard**. The
server sends 100 until `Mob.wz` gives the real value, and the smoke test asserts no mob goes
out with `hp = 0`.

Two other fields are deliberately not zero: `calcDamageIndex = 1` (**[I]**, the reference's
initialised value) and offset 74 = `-1` (**[L]** - `CMP R14D,-0x1 / JLE` means a negative
value skips a call that zero would enter with an out-of-range index).

**Object ids start at 2000**, so mobs and NPCs cannot collide even if the two pools share an
id space, and `FieldMob::new` steps any id that is zero or a multiple of 178 past itself.

**What the reference was worth here, measured:** it matched the head **5 for 5** and the
three blocks after it in order, plus about ten structural details inside `encodeInit` - and
got the three special template ids **wrong** (`8910000/8910100/9990033` against the real
`8909488/8909588/9990545`). Shape yes, numbers no, which is exactly what
`CLAUDE.md` says to expect from it.

Still open: meanings for 17 of the 35 `encodeInit` fields, sent as zero on a "no readable
consumer" argument - which is the argument `research/setfield-zero-audit.md` exists to
distrust.

Drops follow spawns: a mob has to exist before it can drop.

### NEW GOAL, set by the owner 2026-08-19: **quest state that actually advances**

The owner split the quest work into three and asked for the first two now, with this recorded as
its own goal:

1. **Say the right line for the right NPC and quest** - **DONE**, see below.
2. **Paging and yes/no** - blocked on `0x00F3`, which is being decoded.
3. **Quest state that actually advances** - *this goal*.

**Nothing about quests persists or changes.** Accepting a quest does nothing, the same line
comes back every time, and the journal never fills. What that needs, in rough order:

* **Where quest state lives on the wire.** The character record has a **presence-gated quest
  block** - `research/charrecord-presence-map.md` has the 40-row table, and the block needs
  the same treatment `presence[0]` (the stat block) and `presence[2]` (the equipped list)
  each got. Those two are the worked examples; this is the third of the same shape, and both
  earlier ones took a listing walk plus one client run.
* **The quest-result packet.** No packet that *accepts* or *completes* a quest has been
  found in either direction. Until one is, nothing the client does can change state, and
  nothing the server sends can tell it that state changed.
* **Storage.** `crates/store` would need a `quest_state` table keyed by character and quest,
  with the same exhaustive-destructure discipline `character.rs` uses so a new field cannot
  silently fail to persist.
* **`Check` and `Act`.** The generated table already carries both: `Check.<state>` has `npc`,
  `lvmin` and `job`, and `Act.<state>` has the rewards and `nextQuest`. Deciding whether a
  character *may* start a quest is a pure function over data we already have - it is the
  wire format that is missing, not the rules.

**What is already in hand, so nobody re-derives it:** all 322 quests with their full `Say`
trees, `Check` requirements and `Act` rewards, generated from the client's own
`Quest.wz/QuestData` by `tools/dump_quests.py`; and the confirmation that the WZ's quest ids
are the protocol's, because NPC template 1 starts exactly quest 1000 and a real client's
`0x0151` carried quest 1000 with template 1.

**Depends on goal 2's `0x00F3` work** for anything interactive: a quest cannot be *accepted*
until the client can answer a yes/no box.

### 4a. Quest dialogue - **the right line, DONE 2026-08-19**

`0x0151` is answered with that quest's own opening line out of `Quest.wz`, chosen by the
request's action byte: the `Say."0"` conversation for a start or opening-script action, and
`Say."1"` for a completion one. It falls back to the NPC's `d0` line and then to a notice, so
a quest the table does not have still produces something rather than silence.

**Only the first line is sent, and that is a decision rather than an omission.** Paging means
setting the `next` flag, which asks the client to send a `0x00F3` when the user presses it -
and `0x00F3`'s body is not decoded, so the server could not answer. An unanswered request
here does not merely do nothing: `research/npc-click.md` found `player->[0x2330]`, a
**one-request-outstanding latch** set by 37 functions and cleared only by inbound handlers,
so leaving one outstanding silently kills every later request in its class with no dialog and
nothing in any log. One line that ends cleanly beats four that wedge the client.

### 4. NPC quests

**Not started, and it depends on goal 2.** One thing already known: the quest record is a
presence-gated block in the character record, so it will need the same gate work
`presence[0]` needed - see `research/charrecord-presence-map.md` for the 40-row table.

### 5. Map portal transitions completely working - **DONE 2026-08-19**

Confirmed on screen by the owner: *"When I walk through the portal, the portal does spawn me in
the right connecting portal. That's fixed."* Both directions work and arrival lands on the
connecting door rather than the map spawn.

*Original notes below.* **Mostly working.** Map 1 -> 10 confirmed on screen. The stub that stranded the character on
map 10 is gone: `tools/dump_portals.py` generates **1135 portals across all 426 field
images** from the client's own `Map.wz`, and the server loads it at startup.

Two things left, both known:

* **The arrival portal is ignored.** The WZ gives `tn`, the target portal's name, and we send
  portal `0` - the spawn - so the character always arrives at the map's spawn point rather
  than at the matching door. `tn` is already in the generated table's fourth column.
* **The short `characterData = 0` form is now known to be usable.** `[world+0x2358]`, its
  precondition, measured `0x00` on the first `SetField` and **non-zero on every later one**.
  We still send the long form, which works; switching is an optimisation, not a fix.

### 6. Equipment and consumables - **the equipment half is DONE, confirmed on screen**

The character is dressed and **every item carries its `Character.wz` stats**, on every
`SetField` rather than only the first - see §2e for the bug that made it look otherwise for
most of a day.

**Consumables are not started**, and the nearest concrete step is the bag: the unequip
request never reaches the wire, and the leading explanation is that the bag has **no slots**
(§2e, START HERE item 2).

*Original notes below.*

The owner keeps reporting the character as naked and the Equipment window as empty, and this is
the goal that fixes it. Full working: `research/naked-character.md`.

**What the server now sends.** `presence[2]` is set alongside `presence[0]`, and the record
carries a real equipped list: `net::opcode::equipped_block` and `net::opcode::equipped_item`.
A character wearing the four starter items sends a **743-byte record** instead of 224.

**Verified without spending a launch**, which is the whole reason route 1 was chosen over
the standalone item packets: `python tools/channel_smoke.py --set-field-probe` decodes the
real server's real bytes over the independent Python transport and now parses the equipped
block the way the client does - `u8 flagA`, then `(u16 slot, 125-byte item)` until a zero
slot, then **five** `u16` terminators. It checks all four items come back, that each is
exactly 125 bytes, that `dateExpire` is not zero, and that the record is 743 bytes.

The parse is deliberately run at **both** possible stat-block lengths (108 for the
extended-SP branch, 109 for the plain one) and required to succeed at exactly one. That is
a discriminator rather than an assumption: a width error shows up as "neither parses"
instead of as a client fault.

> **What to watch on the run, and what each outcome means:**
>
> * **Character dressed, Equipment window populated** - done.
> * **No fault, still naked, Equipment window still empty** - the *layout* is right and a
>   *value* is wrong. First suspect is `dateExpire`; see `ITEM_NEVER_EXPIRES`. This is the
>   NPC lesson exactly: that body was structurally perfect and produced nothing because
>   `isEnabled` and `alpha` were zero.
> * **No fault, still naked, but the Equipment window lists items** - the items decoded and
>   the *avatar* is not being rebuilt. A different and much smaller problem.
> * **Client faults, or freezes at "Connecting..."** - the record desynchronised. The
>   client's readers throw on underrun and the throw is reported in `ELog` (`0x008F`/
>   `0x0090`) with section-relative RVAs; `tools/pdata_lookup.py` turns those into
>   functions, which names the field that was mis-sized.

**`0x0138` is no longer sent.** The server used to push a `UserAvatarModified` on every
field entry as a guess at this problem. It is dead code at byte level (below), so it was
noise in the log and nothing else; the smoke test now asserts it is absent.

**A correction that would have shipped an expired item.** `research/naked-character.md`
gave the "permanent" `dateExpire` sentinel as `0x00_00_C9_2A_69_C0_00_00`, which is
221184000000000 and decodes to **1601-09-14**. The decimal beside it, 150842304000000000,
is right and is 2079-01-01; the hex was not. `crates/net` sends the decimal and a test
asserts it is non-zero.

**What was wrong with the old reading, and both halves were wrong.** This section used to say
the decode was a vtable call at `+0x330` on classes with no RTTI.

* **`+0x330` is not the decode.** For item type 1 it is `FUN_1402fbb30` = `return this+0x242`,
  an accessor. **The decode is at `+0x358` = `FUN_140304100`**, listing in
  `research/msexe-itemslot-equip-decode.txt`.
* **RTTI was never needed.** The pooled factory `FUN_1403095e0` reads a `u8 type` and calls a
  per-type allocator whose fallback runs the constructor, and the constructor stores its
  vtable with `LEA RAX,[0x14327E1D8]`. Positive control: `vtable+0x88` is literally
  `return 1` for type 1 and `return 2` for type 2 - the same 1/2/3 the release function
  switches on.

**A minimal equipped item is 125 bytes**, because three fields are `u32` bitmasks whose bits
each gate one optional read (17 `u16` in `FUN_140303800`, 21 mixed-width in `FUN_140303b40`).
All-zero masks read nothing past the mask.

> **The trap that would wreck the record, and the one the build honours.** `presence[2]`
> gates the equipped list **and both helper lists called right after it** - `FUN_14030b6f0`
> and `FUN_14030b9e0` re-gate through the same byte, and the second reads **three** lists.
> Setting `presence[2]` therefore costs **four extra `u16` terminators**, five in all. Omit
> them and the record desynchronises, and it has no resync point. `equipped_block` sends all
> five and a test counts them.

**Where it goes:** record offset **223**, between the three string flags (220-222) and the
final ungated `u8`. Verified by walking all 18660 bytes of `FUN_140304b20`: with
`presence={0}` it reproduces today's 224-byte record exactly.

**Ruled out, so nobody spends a run on them again:**

* **`0x0138` is dead code at byte level.** Its apply is guarded by a call to
  `0x1407f5ce0`, which is three bytes of `xor eax,eax; ret`, then `TEST/JZ`. No trigger or
  timing would ever have worked. The earlier "the list at `user+0x1200` was empty"
  explanation was **wrong**.
* **`0x0107` only logs** - it formats `"[BP:%02d] %d"` for 32 body parts and applies nothing.
  (Potentially a free read-back instrument.)
* **`0x0114` never reaches the avatar-apply primitive**, by a reachability walk with the
  `0x0138` path as a passing control.
* **No inbound opcode reaches `FUN_140f80140`** (the apply primitive) by direct call - all 23
  callers checked against the 273-case table.
  **Amended 2026-08-20: the entry set was 23 and is 28.** `tools/callers.py` saw `call` only,
  so five tail-jump entries were invisible - and with them a chain nobody walked,
  `FUN_142d012e0` -> `FUN_142797be0` @ `0x142797df5` -> shim `0x1420dd920` -> `FUN_140f80140`.
  Neither new function is in the case table, so the **conclusion survives**; the *evidence as
  stated* no longer covers the entry set, which is a different thing and worth saying.

**One value to watch, per the NPC lesson:** `dateExpire`, the `u64` at `+0x40`, is zero =
1601-01-01. If a run comes back "no fault, still naked", that is the first suspect.

### Undecoded traffic seen alongside all of this

`0x00D9` (every ~510 ms, coordinate-shaped - almost certainly movement), `0x013D`, `0x00B8`,
`0x02EB`, `0x01ED`, `0x0408`, `0x0184`, `0x0194`, `0x01A5`, `0x02DE`, `0x00ED`, `0x02B2`.
None is answered. None has caused a freeze, so none of them is a blocking request.

---

**Nothing authenticates.** The game socket still carries no credentials; the character is
identified by the migration row and nothing else.

---

## History - how the goal above was reached

Everything below is the record of getting a character onto map 1. It is still accurate and
several parts are load-bearing reference - the presence-array table, the stat-block layout
and the pre-flight analysis are all cited by the goals above. It is **not** a to-do list;
the work is in NEXT GOALS.

### Where the client was before world entry worked

Measured 2026-08-19, end to end. Login -> character select -> pick a character ->
"Connecting..." -> the enter-success sound -> it closes the login socket, connects to
`127.0.0.1:8485`, accepts the channel greeting, and sends `0x0070` then `0x007D` (the
migration hello, character id at offset 8).

**We now answer it, and the answer reaches the right handler.** A probe caught the client's
own dispatcher entering `FUN_142097f80` *while dispatching opcode `0x01A0`*. Everything
between the migration and `SetField` is settled:

| | |
|---|---|
| the channel's dispatcher | `FUN_142cbaa80`, 273 cases, `0x70..0x39a` - `research/msexe-gamestage-dispatch.md` |
| the cipher | **asymmetric**: client sends AES, receives a byte subtract, so we **add** `iv[0]` |
| `SetField` | inbound **`0x01A0`**, handler `FUN_142097f80` - confirmed on the wire |
| the routing | reached from the **login** stage's `OnPacket`, so **no stage transition is needed first** |
| both early returns | pass - `[world+0x33f4]` measured `0x00` |
| the 33-byte fixed head | decoded field by field, `research/msexe-stage-setfield.md` |

### H1. The character record - the last wall (historical: it fell on 2026-08-19)

`SetField` must carry `characterData = 1`, and that branch calls **`FUN_140304b20`**, an
18525-byte decoder with **126 packet reads** (117 was the listing grep's undercount - it missed a `JMP` thunk and a `u64` primitive; see `research/charrecord-decode.md`). Sending `characterData = 0` instead **faults
the client** at `FUN_1402fa540+0x1c`: that short form is "same character, new map" and
assumes state a freshly migrated client does not have. Measured, not guessed.

What is already known, in `research/charrecord-decode.md`:

* **Field 1 is a 100-byte presence array** - one byte per flag, gating whole blocks. 43
  gates consult it. An earlier note here said there was *no* mask, because a scan for `BT`
  and `TEST reg,imm` found nothing; the mask is a **byte array**, not a bitfield, and the
  scan was looking for the wrong shape.
* **Field 1 is a fixed 100-byte raw block.** The decompiler renders its size as computed and
  it is a constant.
* **All 11 raw reads are constant-sized**: 100 once, 8 bytes ten times. 180 bytes, fixed.
* **No length prefix anywhere at the top level.** A wrong width desyncs everything after it
  and there is no resynchronisation point - this record works completely or not at all.
* `FUN_140302e30`, the stat decoder **we already build for the character list**, is called
  at `140304e71`. That part is known ground.

Companion passes: `charrecord-loops.md` (loop census and the straight-line spine),
`charrecord-reuse.md` (what `crates/net` already emits), `charrecord-v214-shape.md`
(candidate names from the reference - candidates only, the method scored 1 of 8 on a
held-out control).

**Read the listing, not the decompiler, for field order.** They disagree by five `u32`
reads; the listing is complete and authoritative. `research/charrecord-decode.md` shows the
working.

### 1a. RUN OF 2026-08-19: the record was accepted, and the map is what is missing

The owner reported it plainly: enter-success sound, **the screen faded to black**, and a few
seconds later the client exited. That fade is the stage transition, and it had never
happened before.

What the logs show, in order:

```text
0x01A0 accepted   142097f80 entered, latch 0x00, both early returns passed
+422 ms           the client SENDS 0x0238 and 0x024D, both empty bodies, both built by
                  FUN_142caa4e0 in the world-object subsystem - it entered the game stage
+3.4 s            CLIENT FAULT 0xC0000005 at 0x140ce89d6
```

**The fault moved**, which was the outcome to hope for. It is no longer `FUN_1402fa540`
(the `characterData = 0` short form). `FUN_140ce89c0` is a **reference-counted release**:

```c
obj = holder->[8];
if (obj != 0) { ... atomic_dec(obj->[0x28]); ... }   // faults reading [RBX+0x28]
```

The null check passes and the dereference still faults, so the holder contains a **non-null
but invalid** pointer - the signature of an object that was never properly constructed and
is then released during teardown.

Which is exactly what an all-zero record predicts. **The client accepted the packet,
transitioned, tried to load map `0`, failed, and faulted cleaning up.** `0` is not a map.

**So the next step is no longer "build the record" - it is "put the map id in it".**

### 1a-i. SETTLED 2026-08-19: the byte is `presence[0]`

The mapping the census could not prove is now read out of the client's own startup code,
with no client run. Full working in **`research/charrecord-presence-map.md`**.

* `FUN_1402fa9a0` is not an accessor. It is a **100-byte bytewise AND**:
  `out[i] = presence[i] & key[i]`. The gate runs its block if **any** byte of `out` is set,
  so every gate carries its own 100-byte key mask.
* Those masks sit in the uninitialised tail of `.data` and are built by **40 CRT dynamic
  initialisers** (pointer array at `.rdata 0x143264a00`), each of which zeroes 100 bytes and
  then sets **exactly one** to 1. So a gate fires **iff its one presence byte is non-zero** -
  40 gates, 40 distinct bytes.
* The gate guarding `FUN_140302e30`, the stat decoder, is entry 7. **Its byte is
  `presence[0]`** - which is byte **45** of the `SetField` body (33-byte head + three `u32`s).

**The census's own labelled guess - "key #k reads array byte k" - was wrong.** The mapping
is a permutation: entry 7 -> byte 0, entry 8 -> byte 62, entry 1 -> byte 44. Both research
documents that recorded the guess now say so. Building on it would have put every flag in
the wrong byte, and a client that skips every block looks exactly like one sent nothing.

### 1a-ii. SETTLED 2026-08-19: the map id is at stat-block offset 84

Working in **`research/charstat-layout.md`**; the flag-#7 region's own layout is in
**`research/charrecord-flag7.md`**. Three independent lines:

* The `u32` at `0x14030325e` is mangled into a 12-byte heap object hung off `record+0xfb`
  with the rolling-checksum seed `0x9a65`. `FUN_1402fa540` is the byte-for-byte inverse.
* `SetField` calls `FUN_1402fa540(user + 0xf3)` and hands the result to a lookup keyed by
  `PTR_s_mapName_143a49020`, which dereferences in `.rdata` to the ASCII string `mapName`.
  The neighbouring literal is `MAP` spliced with TAB, CR and LF.
* It sits immediately before `portal`, which is where `CharacterStat` puts a map.

**We had it in the wrong place.** `chr.map_id` went to record offset **120**, in the
character-list trailer, which is not on the `SetField` path at all - and offset 84 was a
literal zero. The test that was supposed to catch this scanned the record for any
`u32 == START_MAP_ID` and passed on any of them, so it could not fail. It now asserts the
offset, and that changing the map changes those four bytes and nothing else.

`param_3 == 0` on both paths, measured from `[RBP+0x3118]` being arg4's home slot, so the
`SetField` record and the character list share the identical 108-byte stat block. It is one
shared function now.

### 1b. READY TO RUN - and this is the run that can put a character on screen

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

Verified first without spending a launch: `python tools/channel_smoke.py --set-field-probe`
decodes the real server's real bytes with the independent Python transport and reads
`presence[0] = 1`, the character id, and the map at stat-block offset 84. 27 checks pass.

The probe's two free slots are aimed at `140304b20` and `140302e30`, which discriminate:

| hook log | means |
|---|---|
| no WATCH lines at all | the hook never armed - conclude nothing, re-run |
| `140302e30` at character select | the **positive control**: the watch is armed and works |
| `140304b20` but not `140302e30` after the migration | the gate SKIPPED the stat block, so `presence[0]` is the wrong byte |
| both, after the migration | the gate opened and the stats decoded |

**Unlike every previous run, "nothing visible" is now a failure rather than the expected
result.**

**How to read the fault if it comes back.** `research/setfield-fault-shape.md`, written
before this run. The fault at `0x140ce89d6` is a **scope-exit destructor on a stack local**,
not the teardown of a long-lived object: both real callers of `FUN_140ce89c0` end
`LEA RCX,[RSP+N]` / `CALL` / epilogue / `RET`, and the holder's `+8` was non-null garbage
because the local was never written. **Amended 2026-08-20: there are 10 entries, not 5** -
the five extra are adjustor thunks, i.e. virtual entries into the same destructor, found once
`tools/callers.py` learned to see tail jumps and data pointers. That makes it a latent bug in the client's own error
path - **any** early exit from either function faults at the **same address**.

> So a repeat of `0x140ce89d6` would mean "that function bailed out again", **not** "the map
> id is still wrong". The address cannot say which failure it was. The signals that
> discriminate are the two watches and what appears on screen.

A fault at a **different** address is still progress, exactly as before.

### 1b-i. Pre-flight static analysis, 2026-08-19 - what was ruled out before the launch

| question | answer | where |
|---|---|---|
| Does map 1's **field data** exist, or only its name? | **It exists.** `Map/Map/Map0/000000001.img`, 21,532 bytes, parses clean, byte-identical in both data trees. Portal **0** is a type-0 `sp` spawn at (-189, 437) with footholds 48px below. No scripts, mobs or reactors, so the server owes the field nothing extra. Surveyed all 426 field images, not a shortlist. | `research/map1-exists.md` |
| Does the map-name lookup have a bad failure mode? | `FUN_1403999e0` looks the key up first and only builds `Map/Map/Map%d/%09d.img` on a miss. **Map 1 hits**, so the fallback - which contains a **non-returning `E_POINTER`** call - is not reached. A map with no name entry would be a different story. | same, addendum |
| Is the fault a dead end or a red herring? | **A red herring for diagnosis.** It is a scope guard over an uninitialised stack local, so *every* failure in those two functions faults at the same address. | `research/setfield-fault-shape.md` |
| Does the client **block** on `0x0238`/`0x024D`? | **No block is demonstrable.** Fire-and-forget send, two-thirds of the function runs afterwards, and the client's known blocking idiom (an entry guard on a latch) is absent. Both bodies are empty, confirmed three ways. No inbound handler resembles a reply. **Recommendation: send nothing and watch.** | `research/outbound-0238-024d.md` |
| Is the `portal` byte an index or a spawn id? | **Unsettled**, and `xref.py --field` does not answer it. **Moot for map 1**, where portals 0-3 are all `sp`. | `research/charstat-layout.md` addendum |

The falsifier for the "no block" call, worth knowing before the run: **the client alive but
every button dead, with `0x0238` or `0x024D` as the last inbound line in `world.log`.**

**The zero audit** (`research/setfield-zero-audit.md`) checked every field we send as zero
around the record. Its framing is the right one: **every byte outside the record is identical
to the packet the client already accepted**, so none of these zeros aborts the handler - what
changed is that a field now really loads, which promotes the zeros the *field object*
consumes. It ranked six; the top two were then read rather than inferred:

* **`FUN_14187e880(field, 0, 0)`** (head offsets 22/26) is a **28-byte plain setter** - two
  stores into `field+0xa8`, no division, no allocation, no indexing. The zero-divisor story it
  was ranked #1 for is not at the call site. **Not a reason to invent a width/height pair.**
* **`FUN_142d16ef0`** (head offset 17) is a `std::map` in-order walk gated on the head node's
  `_Isnil` byte, so it is a **no-op on an empty tree**. Sending `0` instead of the reference's
  `1` changes nothing on a first field entry.
* The **three `u32`s before the record are randomiser seeds**, passed intact as a triple and
  nowhere else - which also retires the `(fieldId, portal, ...)` candidate in
  `charrecord-flag7.md` §6. All-zero is the absorbing state of the LFSR family, so it is worth
  fixing - but the audit's own finding is that **zero cannot fault**, so it cannot stop a map
  loading. **Deferred to after this run** rather than folded into it.

**Decision: change nothing before the launch.** The variant under test is the record -
`presence[0] = 1` and a real map id. Everything else is byte-identical to the packet the
client accepted on 2026-08-19.

`research/msexe-setfield-aftermath.c` has the fault site and `FUN_142caa4e0`, the builder
of the two packets the client sent on entering.

### 1c. The all-flags-clear minimum record is 112 bytes

**Historical now** - this is the packet that was accepted on 2026-08-19 and faded to black,
not the one the server sends today. Kept because the skip-chain it documents is what makes
the zeros in the current record safe. Settled by the loop census,
`research/charrecord-loops.md`. With every presence flag clear,
every count zero and the boolean at `0x140304cf2` zero, the client reads **7 fields, 112
bytes**:

```text
raw[100]   the presence array - all zero
u8
u32
u8
u32
u8
u8
```

Verified by walking the gate skip-target chain end to end. Supporting facts:

* **61 straight-line reads**, and **31 of the 37 read-bearing loops collapse cleanly** on a
  zero count. Every counted loop is MSVC-rotated - guard before the head - so a zero count
  never runs the body. That was checked specifically, because a bottom-tested loop would
  read fields anyway and desync everything after it. **There are none.**
* **Six loops are not count-skippable.** Four have fixed trip counts (two of them adjacent,
  forcing 15 x u32 = 60 bytes); all four sit behind presence flags instead. Two are
  **sentinel-terminated**, `u16 key; while (key != 0) {...}` - sending `0` skips them but
  costs a trailing `u16`, and reading them as counts would desync.
* **All 43 gates confirmed** to take the 100-byte field as their input. The static keys live
  at `0x143abeb10`, stride `0x70`, and all 38 land on exact multiples - which is what makes
  the index assignment trustworthy. Flag **#17** is the biggest lever: 21 reads and 7 loops.
  One gate is an `OR` - the block at `0x140305104` needs flag #5 **or** #12.

**Not proven, and it is the thing to settle before building**: the mapping from key index to
*byte offset* inside the array. `FUN_1402fa9a0` was not in the listing the census used. A
1:1 index-to-byte mapping is plausible and is inference.

**And a caveat that bounds the whole estimate**: 26 non-primitive calls also receive the
packet pointer and consume bytes that are not among the 126 reads. Zeros still collapse
them, but any non-zero count costs more wire than the census implies.

### 2. Then send it and run

`crates/world --set-field-probe` already builds and sends the head with `characterData = 0`.
Swap in the real record, check it with `python tools/channel_smoke.py --set-field-probe`
first - that validates framing, the cipher and every field offset without spending a client
launch - then run
`powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe`.

### 3. Then the ordinary game-stage work

Whatever the client asks for once the map loads. `research/msexe-gamestage-outbound.txt` is
the set of 175 client -> server opcodes the game subsystem builds, which is what to expect.

### Standing, and unchanged by any of this

* **Nothing authenticates.** The channel claims a migration by character id and the row is
  single-use; that is not a proof of identity. Say so when reporting.
* **`0x00BC` is still undecoded** and deliberately unnamed, so it logs in full.
* **`conn+0x48` meant exactly what the code said** - that question is closed. All three of
  its readings hold; a channel is simply **asymmetric**, and the retraction that once stood
  here had generalised a measurement of one direction to both. See
  [[maplecw-connection-type]] and `docs/transport.md`.


### BUILT 2026-08-19: the channel greeting, and a cipher that turned out to be wrong

* **`net::handshake::channel_greeting`** - the login greeting minus its two gated blocks,
  **26 bytes shorter**: `A..F` is 14 (it is easy to forget `B`'s 2-byte length prefix, and a
  test pins it) and the version block is 12. It starts at `G`, which is the field whose
  zero-read raised "The client is outdated". **Confirmed on screen**: the client accepted it
  and went on to send packets.

* **`net::ByteShiftCipher`** - `out[i] = in[i] - iv[0]`, and it **is** the channel's
  server -> client cipher. `crates/world` sends with it in `Shift::Add`, because the client
  subtracts on receive; it decrypts the client with `MapleCipher`, because the client sends
  AES. This entry said the opposite for most of a day - see the three-pass history in
  `docs/transport.md`, which is the more useful thing to read than this bullet.

  The methodological point outlasts the code: the polarity was going to be settled by
  logging a body under both readings and seeing which named a plausible opcode. That
  discriminator was sound. It was aimed at the wrong question, because the premise it rested
  on - that `conn+0x48` selects the cipher - was never checked against the wire.

### RETRACTED 2026-08-19: the login handshake was never failing

An `ELog` record showing `INVALID_CLIENT_VERSION` at the `G`/`H` gate was reported here as
the login connection failing on every run. **It was the previous run's migrated connection.**
The upload is a file replay - `FUN_1415ddd10` reads a log file and deletes it - so a record
that arrives at startup describes an *earlier* failure. `docs/handshake.md` carries the full
retraction and `tools/decode_elog.py` now carries the drain/experiment/re-read protocol.

The `conn+0x48` explanation is untouched and is now the *only* explanation needed: a channel
connection skips the greeting's gated blocks, so it reads `G` from our `A` field and raises
`0x348`. Login passes. Channel fails. One cause.

Two corrections fall out. A site-`840` record means the handshake **died** - `FUN_140cc2350`
reaches `_CxxThrowException` then `INT3`, and `FUN_1415d10e0` has zero catch funclets. And
**"First Connect" is dead code**: all three call sites pass `param_3 = 0`, which retires the
`high == 100` gate.

### 2026-08-19: the client has been reporting a failed login, and we called it a greeting

`0x00C0` was in the log table as `CLIENT_HELLO`. It is a **login failure report**.
`FUN_141b2a660` builds it *only* when `FUN_141d60eb0(user, pass, 0xc9, 0)` returns
non-zero, and the body is the launch mode followed by the error code. Every capture we have
carries **`0x4E20` = 20000, a Nexon Passport error**, twice per run.

That matters beyond the name. `docs/session.md` says the login form is vestigial and the
server supplies the identity - which is still true of what reaches *us* - but the client is
evidently making a local auth call and getting a failure, and it says so on the wire. Nobody
had read it. Whether that failure is connected to the handshake's `INVALID_CLIENT_VERSION`
is **not established**; they are two separate reports from the same run.

Five of eight inferred opcode names were wrong. Evidence per opcode in
`research/msexe-client-opcodes.md`; the corrected table is `crates/net/src/names.rs`.


### 2026-08-19: the migration works, and then the client says it is outdated

**Confirmed on screen.** The client selected a character, `FUN_141b36f60` was entered while
dispatching `0x0011` and ran its **whole** body (2755 us, against 355 us for the login
result - so not an early bail), closed the login socket, and **opened a second connection**.
The `0x0011` decode is right.

It then showed **"The client is outdated"** and exited cleanly (code 0, 38.7s).

**That dialog is the handshake's, not the migration's.** `0x22000007` is raised from four
sites, all inside the greeting check - this is the same dialog the whole of
`docs/handshake.md` was written about. Our server sent connection #2 the *identical* 48
bytes that connection #1 accepted, so the failure is state-dependent, not layout-dependent.
`FUN_1415d10e0`'s `param_3` selects "First Connect" from "Second Connect", and connection #2
is the second - a path this project has never exercised.

**Two things were wrong by construction and are now fixed:**

1. The client reconnected to the **login server**, because that is what the migration packet
   advertised. A login server answering a game connection is wrong whatever the dialog says.
   `crates/world` is now a separate per-channel process (the owner's instruction), and
   `World::channels` is one address per channel rather than a count.
2. The login server answers a new connection with an unprompted **`0x0032` startup gate**.
   That releases the *login* connection's startup loop; a channel has no startup loop.
   `crates/world` greets and then waits.

**The next run names the failing gate.** `FUN_140cc2350`, `FUN_1415e0e30` and
`FUN_1415e0fb0` - every raiser - funnel into **`FUN_141804870`**, so one watch identifies
the site. `rdx` is the site id (`0x348` G/H, `0x2df` L, `0x33b` second connect, `0x327` first
connect) and `r8` is the error code. `tools/test-server.ps1` arms it by default.

**`-Session mode=2` is off by default now.** It was meant to stop mode 5's auto-login but is
applied on `0x0000` dispatch - our *reply* to the login request - so it always landed after
the auto-login it was meant to prevent. It never did its job, and it writes `session+0x68`,
which `FUN_1415d10e0` reads on every connection including the channel one. That also closes
goal 4 below by deleting it rather than fixing it.


In the order that unblocks the most. Everything above the line is done and confirmed on
screen; nothing here is speculative work.

1. **Enter the game world.** `0x0078` is the select-character request and carries the
   character id - captured twice, with 203 and 204. It is unanswered, which is why the client
   sits on "Connecting...". This is Stage 4 and the biggest single step left.

   **The reply is `0x0011`, and that is no longer a candidate - it is identified, statically,
   with no client run** (2026-08-19). `case 0x11` is `FUN_141b36f60`, and it is the *only*
   login-stage handler that builds a `sockaddr_in`: `htons` appears exactly once across all
   fourteen decompiled case handlers, and it is in this one, which then calls
   `FUN_1429f14c0(PTR_u_GameIn_143a47c08, 100)` - the string **`GameIn`**.

   **Decoded so far** (full write-up in `docs/opcodes.md`): `u8 result` (0 proceeds, through
   the same `FUN_141b267c0` gate as `0x0000`), `str message`, `u8`, then `u32 ip` (four
   octets in order, straight into `sin_addr`), `u16 port` (the client `htons`es it),
   `u32 characterId` (looked up in the map at `DAT_143ac9890`; a character the login result
   did not send fails the lookup and skips the whole action block), three `u32` (the second
   one non-zero makes the client load `Etc/SpecialServerInfo.img` - send zero), a flags byte,
   `u32`, `u8`, four discarded fields, `u8[8]`, then `u32 key` and `u32 length`.

   **The tail is decoded too, and the packet is built.** The last `length` bytes are
   transformed in place and read back over; a second nested pass follows but its key is
   `FUN_140738db0(0x80000000, 0x7fffffff)` - a **random number** - and it happens after every
   read, so it is a scramble-back of already-consumed bytes, not a decode. Only the first
   pass matters, its key and length are both fields of the packet, and it inverts. Sending
   `length = 4` covers the client's one 4-byte read with a single aligned word and no partial
   tail.

   **`crates/login` now answers `0x0078`.** `Config::advertise` is new and is deliberately
   not `bind`: the octets go straight into the client's `sockaddr_in`, so the address must be
   reachable from the *client* machine - the first place the homelab move actually bites.
   Refusals use `0x0A`; see `docs/login-server.md` for the three separate ways a
   wrongly-chosen refusal code migrates the client anyway, opens a browser, or returns in
   silence.

   **What is left is one client run, and it has a falsifiable prediction.** The migration
   hands the client a `u32` seed. The only thing that reads it back is `FUN_1415d10e0`, the
   builder for outbound **`0x007D`** - so the client should reconnect and send `0x007D` with
   our seed inside. The server numbers connections and `describe()` names that packet, so the
   log alone answers it. The seed is `0xC0DE0000 ^ id`, **a placeholder, not a token**.
2. **Real sessions for multiple accounts** (the owner, 2026-08-18; testing-grade is fine). The
   token-in-`0x0073` route is measured dead, so this is one login server per account per
   port, or the `grap-stub` identity patch. `Session` should take its account from a resolver
   rather than from `Config` either way, so the swap is one function.
3. **`create=on`, the last honest patch.** Static analysis is exhausted (see below). The
   remaining route is the **in-process opcode walk** with `FUN_140c9e230` as the oracle: one
   launch covers the whole inbound opcode space. **Blocked on a small change** - `-Probe`
   takes either a walk range *or* watch targets, and this needs a walk plus the two mandatory
   patches (`1415db360:ret`, `141b2a280:rdx=0`) or the client dies at ~37s and the dialog
   blocks the screen. If the walk comes back empty, `create=on` is a permanent workaround for
   a client-side entitlement rather than a placeholder for protocol - which is worth knowing,
   because it turns a TODO into a fact.
4. **The auto-login ordering**, which the owner spotted. `mode=2` is applied when opcode `0x0000`
   is dispatched - and `0x0000` is our *reply to* the login request, so it lands after the
   auto-login it is meant to prevent. Patch the mode when the session object first becomes
   readable (the hook log shows that at +4s, long before login) instead of on a dispatched
   opcode. Hook work, not protocol.
5. **Stage 3.5 off-box**: the firewall carve-out, `crates/auth` with a configurable bind and
   TLS, `crates/launcher`. See `docs/deployment.md` and `docs/launcher.md`.
6. Smaller: the slot count should come from the account rather than the constant `3`.

## DONE - a real server, and characters that persist

**The owner set this on 2026-08-17; built 2026-08-18.** `crates/login` answers the client from a
database instead of from canned bodies, and a character created in one run is in the
character list of the next. Full notes in **`docs/login-server.md`**.

### What it is

| module | holds |
|---|---|
| `handshake` | the greeting - 48 bytes, unencrypted, server speaks first |
| `session` | the whole protocol as a **pure state machine**: bodies in, bodies out |
| `server` | the socket loop, framing, the log. Nothing protocol-shaped |

`session` having no socket and no clock is the point: every exchange measured against the
real client is a unit test, so a change can be checked without spending a client launch.

Storage is `crates/store/src/character.rs`: a `characters` table whose columns match the
protocol's `Character` field for field, plus an `equipment` table keyed by character and
slot, both cascading from `accounts`. The row maps to `net::opcode::Character` by
**exhaustive destructure in both directions**, so adding a protocol field breaks the build
until a column exists - a separate storage struct would have meant mapping nineteen fields
by hand, and a missed field is a stat that silently does not persist.

Character ids are database rowids, so they are distinct per character. That is a protocol
requirement: a canned reply that sent id 200 twice made the client drop the second character.

### The rule the whole thing is built around: always answer

**An unanswered request freezes the client's entire UI** - every button, including the OK on
the quit prompt. No path in `session` returns an error instead of a reply; a database
failure becomes a refusal the client can render, and the reason travels in the log label.
Pinned by `every_request_the_client_blocks_on_gets_an_answer`.

### The name check is the first genuinely honest answer

The harness replied "available" to every name, including ones it had already handed out.
The server checks the database, distinguishes available / already used / not allowed, and
names are unique across the **whole service** case-insensitively - the client's request
carries no account to scope by.

### How it was verified without a client launch

`tools/login_smoke.py` is a **stand-in client** built on `tools/transport.py`, an
implementation written independently of the Rust one from the client's own receive path. So
agreement between them is evidence, not one module agreeing with itself.

Verified 2026-08-18 against a fresh database: greeting accepted, gate delivered and repeated
to a quiet client, login answered with four packets in order, creation permitted, a free
name available and the same name taken immediately after creating it, create accepted, the
new character in the next login result - then the server **stopped and restarted**, and a
new connection still listed it. `maplecw-login --list` and an independent `sqlite3` read
agree.

**What that does not prove is the client's reaction.** Only a launch shows whether the
character is drawn, and drawn correctly. Every packet-level fact in this repo that turned out
to be wrong was wrong about a body the client read differently, not about a byte count.

### CONFIRMED ON SCREEN 2026-08-18 - creation, persistence, and the slot limit

The owner created three characters against the real server and reported all three working. This
closes the priority end to end.

| what | evidence |
|---|---|
| the create transitions back to character select | on screen |
| **three characters persist** | ids 201, 202, 203 in `maplecw.db`; a *fresh server* and a *fresh connection* return all three in the login result |
| **the three-character limit works** | "Create a character" is **disabled** at three, on screen, with no patch for it |
| entering the world hangs on "Connecting..." | expected - there is no channel server |

**The transition bug was the character id.** The first run created `TestChar` and the client
stayed on the creation screen. The reply was byte-identical to the one that worked on
2026-08-17 **except the two copies of the id** - `1` against `200`. Ids now start at 200
(`FIRST_CHARACTER_ID`, seeded through `sqlite_sequence`) and the client accepts them.

**Do not renumber characters from 1 again.** That is the whole finding.

**The slot limit needed no new code.** `login_result` already sends a truthful list and
`slotCount`, and `FUN_141b282d0` computes the free slot from them. So the client-side half
of the owner's "enforce it properly" goal was already satisfied by telling the truth; the
server-side half was already tested. What remains is `create=on`, which is a **different**
gate - it enables the button *at all* - and removing it is finding a packet.

### CAPTURED FOR FREE: the select-character flow

Entering the world sent three packets nothing answers yet. Bodies are in `login.log`
because it records them now. First read, from one capture:

```text
0x0078  73B  u32 0 | str "." (the placeholder PIC) | u32 203  <- the character id
              | u8 0 | str "AA-BB-CC-DD-EE-FF, 00-00-..." | str "AABBCCDDEEFF_DEADBEEF"
0x0079  67B  u32 203 | str "TestCharC" | u32s | SYSTEMTIME 2026-08-18 13:36:32
0x00BC  12B  09040000 09040000 09040000   - 1033 three times
```

**`0x0078` carries the character id** (`cb000000` = 203 = `TestCharC`), which makes it the
select-character request and its reply the migration packet - our `0x0011` candidate, and
Stage 4. The MAC list and machine id are there again: record, never gate on.

`0x0079` is a client report carrying a timestamp; the client did not block on it.

### MEASURED: a launch-argument token does NOT reach the server

**Settled 2026-08-18**, and it closes a question open since the launcher was designed. The owner
launched with `-SessionTokens "tokA tokB tokC tokD tokE tokF"` and `0x0073` came back
**byte-identical to the run without them**:

```text
26B  05000000 0000 aabbccddeeff deadbeef 00000000 764d0000 0000
     mode=5   ""   MAC          machine id
```

No token text anywhere in the log. `-NXLDEBUG` does route arguments 3 onward into the client
config at `+0x90`, but **nothing carries them onto the wire**.

**This kills the design where the launcher's single-use token rides in `0x0073`.** See
"Next goals" below for what replaces it.

**The tokens also broke that run:** a "trouble connecting" dialog appeared immediately after
the splash, and `FUN_141b2a280` - the function we suppress - was **never entered** (its watch
was armed and logs every call; the hook log has no `WATCH` lines). So it came from a
different path. **Do not pass `-SessionTokens` in ordinary runs.**

### DONE: delete a character

**`0x008B`, confirmed on screen 2026-08-18, first try.** The owner deleted `TestCharB`; the request
was `0x008B` with body `ca000000` (202), answered `0x0016 deleted "TestCharB" (id 202)`.

**It was read, not captured.** `FUN_141b28750` is the Delete button handler and is not
virtualised; `research/msexe-send-opcodes.txt` already listed the opcode against it. Checking
that table before guessing saved a launch. The counterexample is `0x008A`, which *is*
virtualised - **absence from that table means virtualised, not non-existent.**

Reply is `u32 characterId, u8 result`, correcting an earlier note that called it a single
`u32`. Two traps, both now pinned by tests:

* **A refusal must use `6`.** The switch names `6, 9, 10, 0x10, 0x12, 0x14` and a *default* -
  and the default is the branch that removes the character. Any other non-zero code deletes
  it anyway.
* **An unanswered delete disables the button for the session.** The builder sets
  `stage+0xd4` before sending and returns early while it is set; only the result clears it.

Ids are **not reused**: the freed 202 was not handed to the next character, which got 204.

### SETTLED: `create=on` cannot be resolved by reading the image

The owner asked for the packet that sets the create-character flag. **Static analysis is finished
and the answer is that no readable code sets it.** Every route checked, each with a control
proving the instrument finds things:

| scan | result | control |
|---|---|---|
| direct `call` to the setter `FUN_140c9e230` | **0** | the getter: 2 found |
| its address as a qword (vtable / fn table) | **0** | `FUN_141b25f30`, a known vtable entry: found in `.rdata` |
| the one cluster function with a live caller | initialisation | decompiled: a run-once latch that seeds protected values and never calls the setter |

The flag is a six-byte self-checksumming blob that reallocates every 0x6f accesses, seeded
during the connection handshake. `FUN_14003fb80` is a second orphan of the same shape. The
unpacked Nexon DLLs hold no creation-shaped strings.

**The evidence supports the flag being flipped from Themida-virtualised code.** The remaining
route is runtime - see "Next goals".

**One scan is not trustworthy and must not be quoted:** references to the blob pointer
`DAT_143ac8170` come back zero even with REX.R forms added, which cannot be right when three
functions dereference it. The *counter* scan (`DAT_143ac8168`) works and is what found the
cluster.

### SETTLED: the launch keyword is not a lever - stay on `-NXLDEBUG`

**The owner's hunch, checked statically 2026-08-19, entirely by reading the image - no client
run.** Full write-up in `docs/launch-protocol.md`; new decompilation in
`research/msexe-launchmode.c`, `msexe-launchconfig.c`, `msexe-modeclass.c`,
`msexe-loginbutton-modes.c`.

**`-NXLDEBUG` is not a debug mode.** The chain is short and now fully verified:

```text
argv[0] keyword -> cfg+0x38 -> FUN_142c95c90(cfg) -> session+0x68 -> FUN_142c4a810(session)
```

`FUN_142c95c90` is `return *(u32*)(cfg+0x38)` and has exactly **two** callers; the session
constructor `FUN_142c43db0` writes `session+0x68` from it **once**, and nothing else in
readable code writes that field. And `-NXL`, `-NXLDEBUG` and `-NXLPTS` all write the same
value: **5**. So `-NXLDEBUG` selects the identical mode the real Nexon Launcher passes; the
`DEBUG` suffix only changes which argv slots map to IP and port. It sets no debug flag and
relaxes no check.

| keyword | mode | reachable? |
|---|---|---|
| `GAMELAUNCHING`, `IPPORT` | 2 | only with a whitelisted IP (below) |
| `WEBSTART` | 3 | needs token 1 non-empty |
| `STEAMSTART` | 4 | (the doc previously mis-attributed mode 4 to "no keyword") |
| `-NXL`, `-NXLDEBUG`, `-NXLPTS` | 5 | what we use |
| anything else | *unset* | opens the Nexon micro-site and returns |

**Why `IPPORT` "crashed":** it `strcmp`s the IP against six hard-coded literals
(`10.9.2.131/132/133`, `44.234.166.161`, `44.234.167.163`, `44.234.163.43`) and on a miss
opens the micro-site, reports error `0x18a`, and returns **without setting a mode**. It was
never a crash in the parser - mode 2 was simply never reached.

**Mode 2 is not the mode we want anyway.** The login screen's buttons fork on `mode == 5`
(`FUN_14112a570`): `login` calls `FUN_141b3ff10` (the login request) in mode 5, and
`FUN_141b3f050(stage, 4, 600)` otherwise - a 600 ms fade to **screen 4, CharSelect**.
(This line said "world select" until 2026-08-19; see the retraction below.)
So mode 2 restores the classic WorldSelect -> ChannelSelect flow; it does **not** make the
client authenticate, so it does nothing for multi-account. It also silently kills the
login-screen Quit button, which reads as a freeze.

**`-NXLPTS` buys nothing:** its `cfg+0xc8` has no reader, and its `DAT_143a88df8 = 0` is
read in exactly one place, `FUN_141b5bb50`, the login-screen draw.

**The client's own classifier** is `FUN_1401e7bf0(mode) { return mode==3||mode==4||mode==5; }`
- "a launcher started me" vs "I was started directly". Worth grepping for alongside `== 5`.

**Static confirmation of the token measurement.** `FUN_142c95f20(cfg,out,i)` reads
`cfg+0x90+i*8` - the six session tokens - and `Xrefs` returns **no callers**, in a run where
sibling accessors did return callers, so the instrument was working. Two independent
instruments (this and the wire capture) now agree that a launcher token cannot ride in
`+0x90`.

**Only remaining launch-line option worth anything:** `-NXL <anything> <region> <ip> <port>`,
purely to set the region string at `cfg+0xc0`, which `-NXLDEBUG` leaves at the config
default. Worth one run only if something is ever traced to the region.

### Still open

* The slot count is the constant `3` rather than a property of the account.
* Entering the world. **`0x0078` is answered** - with `0x0011`, and the client migrates. What
  is left is the character record `SetField` must carry; see NEXT GOALS. (This entry used to
  say `0x0078` was unanswered and that this was why the client sat on "Connecting...". Both
  halves stopped being true on 2026-08-19.)

## NOT AUTHENTICATED - say so when reporting

**Nothing on the game socket proves who the player is.** The client's login request carries
no credentials; the login form is vestigial and the server supplies both the login result
*and* the account name. So `--account` decides whose characters every connection sees, and
two different people connecting are the same account. The server prints this at startup:

```text
serving every connection as account "maplecw" (id 1), 1 character(s) stored
NOT AUTHENTICATED: the game socket carries no credentials, so anyone who
  connects is served as that account. See docs/launcher.md.
```

Closing it was to be Stage 3.5: the launcher authenticates against `crates/auth`, gets a
single-use token, and the token reaches the login server so it can call `/consume`. **The
route that design assumed is now measured and dead** - the client does not put launch
arguments on the wire, so the token cannot ride in `0x0073`. What is left:

* **one login server per account, each on its own port**, the launcher choosing the port.
  Crude, needs no protocol, works today, and the owner said testing-grade is acceptable for now;
* or write the identity string at `DAT_143ac1898+0x1b8` from `grap-stub`, which is already
  in-process. `0x0073` sends it as its second field and it is empty because nothing computes
  it. That is a **client patch standing in for a real session**, honest only if labelled.

Until `/consume` gates the login result, do not describe a session as authenticated.

### Build it for two machines from the start - see `docs/deployment.md`

The owner will host this on a homelab box, so **the client and the server are not the same
machine**. Designing for that now is cheap; retrofitting it is not. The three things that
actually change:

1. **The firewall rule breaks the moment the server moves off-box.** It blocks all outbound
   from the client, and has been harmless only because *Windows Firewall does not filter
   loopback* - the script's own docstring says so. Off-box, our own traffic is caught by the
   rule that blocks Nexon. It has to become a block whose remote address is the complement of
   the server. The twenty Nexon addresses must stay blocked, so `-SkipNetCheck` is still
   required either way.
2. **Bind address is not advertise address.** `crates/login` binds `127.0.0.1:8484` by
   default and takes `--bind`; there is no `advertise` yet, because **nothing we send carries
   an address**. Whatever packet eventually does must carry one the *client* can reach.
   `0x0011` is the candidate and it is Stage 4.
3. **Machine identity must not be an authorisation input.** `0x0073` and `0x0078` carry a MAC
   list and a machine id; record them, never gate on them, or a second machine cannot play.

`crates/auth` already has the right shape - `POST /login` for the launcher, `POST /consume`
for the login server, tokens stored only as hashes. It binds loopback only today and will
need a configurable bind plus TLS.

### And a launcher - see `docs/launcher.md`

The owner asked for a minimal launcher that applies the client patches and takes a username and
password, since the real client uses a validated session. The design is written, but the
route it assumed is **dead**: the session array at config `+0x90` - which `-NXLDEBUG` fills
from launch arguments 3 onward - is not transmitted in `0x0073` (measured on the wire) and
has no reader in readable code (`FUN_142c95f20`, `Xrefs` with working controls). So the
launcher cannot hand the server a token through a launch argument. See
`docs/launcher.md` for the two routes that remain.

## SOLVED - the ~37 second exit (kept for the method, not the answer)

**How it dies is now measured: exit code `0xC0000409`, `STATUS_STACK_BUFFER_OVERRUN`.** On
x64 that is `__fastfail` - `int 0x29`. The client ends *itself*, deliberately.

That one fact explains every negative collected before it, and they were all real:

| what was seen | why `__fastfail` produces it |
|---|---|
| no vectored handler ever saw a fault | `int 0x29` traps straight to the kernel and is never dispatched to user-mode handlers |
| `RtlExitUserProcess` never entered | a fail-fast does not go through the ordinary exit path |
| `NtTerminateProcess` never entered | same |
| no thread drain before the process vanished | every thread is torn down at once by the kernel |
| nothing external held a terminate handle | there is nothing external to find |

**RETRACTED: "a crash or `__fastfail` is ruled out".** That rested on the Windows
Application log holding no error for these exits. The log does work - it holds a real
MapleStory `0xc0000005` - but a fail-fast is not required to produce a WER Application
Error entry, and here it produced none. The log-based negative only ever covered
WER-reported crashes, and it was stretched past what it could carry.

**It is not an external kill, and that is now measured rather than assumed.** The elevated
handle scan through the client's whole life found only `lsass`, three `svchost`s,
`RadeonSoftware` and `audiodg` holding handles to it - no Nexon process, no protection
process, nothing that appeared before the exit. The owner's anticheat-service hypothesis is not
supported.

**The instruments did speak, which is what makes the silence readable.** Both
`ntdll!RtlExitUserThread` and `ntdll!NtTerminateThread` armed with verified `int3`s and
each fired six times for ordinary thread exits, the last 5.8s before death, with the
200-hit cap nowhere near reached. Neither fired at the exit, exactly as a fail-fast
predicts.

### SETTLED: the deadline is anchored to process start, not to anything on the wire

| run | launched | exited | from launch | from login result |
|---|---|---|---|---|
| 4 - created a character, attempted to enter the world | 20:50:02.083 | 20:50:39.040 | **36.96s** | 27.0s |
| 5 - nothing clicked at all | 20:59:42.906 | 21:00:19.793 | **36.89s** | 23.8s |

**0.07 seconds apart from launch; 3.2 seconds apart from the login result.** The client
`__fastfail`s on a fixed wall-clock deadline of about 36.9 seconds from process start. Not
a CPU quota either - the two runs burned 12.13s and 11.64s of CPU.

Run 5 settles more than it was asked to. The owner touched nothing, and **the client logged
itself in anyway** at +7.4s, sending `0x00C0`, `0x0073` and `0x0080` on its own. So the
login-result interval in both runs is the client's own timing rather than a human's, and it
still moved by 3.2s while the launch interval did not move at all.

**Therefore the exit is not a protocol timeout. The server owes the client nothing**, and
everything measured on the wire is beside the point for this bug.

### Correction: `-Session mode=2` does not prevent the auto-login

The mode byte is patched when opcode `0x0000` is dispatched, and in run 5 that landed at
20:59:56.506 - about 100ms *after* the client had already sent its login sequence at
20:59:56.4. A patch cannot prevent an auto-login it arrives after. What carries the client
to character select is the login result, not this patch. Keep the patch (the transition
behaviour downstream depends on it), but the claim in its log line that "the tick should no
longer auto-login" is wrong.

### SETTLED: our own patching is not the cause

| run | patches in the image | lifetime |
|---|---|---|
| 4 | full, clicked through creation | 36.96s |
| 5 | full, nothing clicked | 36.89s |
| 6 | **none** | **36.70s** |

Run 6 used `-NoPatch`: no dispatcher detour, no `int3`, no session patch. The control is
verifiable rather than assumed - the only line the hook wrote that run is
`install_once: our code IS running. env=false marker=false -> standing down`, and all three
marker files were absent. The client's `.text` was exactly as installed, and it still
`__fastfail`ed on the same deadline.

So the `int3` watches are free to use for this. They change nothing.

### The fail-fast is an ordinary CRT fatal error, not an anticheat kill primitive

`tools/ghidra_scripts/FindFastFail.java` walked the disassembly - 11,684,028 instructions -
and found **8 real `int 0x29` sites**, all in `.text`. The raw byte scan had reported 63,
because x86 is variable-length and a byte scan is not instruction-aligned; that number was
eight times too high and should not be quoted again.

Every one sits behind the standard MSVC preamble
`MOV ECX,0x17; CALL [IsProcessorFeaturePresent]; TEST EAX,EAX; JZ skip`, and the reason
code loaded into ECX names the path:

| site | reason | what it is |
|---|---|---|
| `FUN_142f048cc`, 44 callers | `7` FATAL_APP_EXIT | `abort()` |
| `_invoke_watson` @ `142f04834`, 52 callers | `5` INVALID_ARG | the CRT invalid-parameter handler |
| `FUN_142ef3e44` | `2` STACK_COOKIE | `__report_gsfailure` |
| `FUN_142ef4c1c`, 5 callers | from `EBX` | a generic `__fastfail(code)` wrapper |
| `FUN_142ef3f2c` | from the stack | another wrapper |
| `__except_validate_context_record` | `0xd` | SEH context validation |
| `__except_validate_jump_buffer`, 2 sites | `0xd` | `longjmp` validation |

**This reframes the bug.** A deliberate "protection decided to kill you" would not go
through `abort` or the invalid-parameter handler. An uncaught C++ exception reaches
`abort` through `terminate`, and that is a timeout in the client's own code failing in a
way nobody caught - a very different thing to chase than an anticheat.

**Caveat, stated because it changes what a negative would mean:** this scan covers code
Ghidra disassembled, which is `.text`. `.boot` is Themida's own 12.8 MB and holds 140 raw
`CD 29` byte matches that the instruction walk did not see, so a fail-fast inside the
packer's runtime would not appear in the table above. If none of the four watches below
fire, that is where to look next.

### FOUND: a stack cookie failure in a Themida-virtualised function

The `-FastFail` run named it in one launch. Of the four watched CRT entry points, exactly
one fired:

```text
21:15:39.557 WATCH #1: 0x142ef3e44 ENTERED on tid 140940 ... called-from=0x142e9fe03
21:15:39.783 EXIT code 0xC0000409 after 36.6s
```

`0x142ef3e44` is `__report_gsfailure`, reason code `2`,
`FAST_FAIL_STACK_COOKIE_CHECK_FAILURE`. **A stack buffer overrun**, 226ms before the
process died - not `abort`, not the invalid-parameter handler. And it fired on **tid
140940, the main thread**, which was the first thread in the process.

`.pdata` puts the caller inside **`0x142e9ebd0 .. 0x142e9fe0c`** (4668 bytes).
`tools/pdata_lookup.py` is new and does this lookup: Ghidra had no function containing that
address and `getFunctionContaining` returned null, so `DecompileFunc` created one at the
epilogue, which decompiles to nothing. The PE exception table is authoritative - the linker
wrote it - and it has 120,981 entries covering every function with unwind data.

The function's frame is readable even though its body is not:

```text
142e9ebd0  MOV [RSP+0x20],R9B      four arguments homed: ptr, ptr, int, bool
142e9ebdf  MOV [RSP+0x8],RCX
142e9ebe4  PUSH RDI
142e9ebe5  SUB RSP,0x410
142e9ebec  MOV RAX,[0x143a8b908]   __security_cookie
142e9ebf3  XOR RAX,RSP
142e9ebf6  MOV [RSP+0x400],RAX     planted
142e9ebfe  JMP 0x144f94a9c         <- tail jump into .themida
...
142e9fddf  LEA RCX,[RSP+0x200]     a 0x200-byte local
142e9fde7  CALL 0x142e9e350        a thunk: XOR EDX,EDX; JMP 0x142e9e9e0 - a destructor
142e9fdfa  MOV RCX,[RSP+0x400]     the cookie, immediately above that local
           XOR RCX,RSP
142e9fdfe  CALL 0x142ef44b0        __security_check_cookie
142e9fe03  ADD RSP,0x410           <- the called-from the watch recorded
```

**Its body is virtualised.** The prologue plants the cookie in plain code and tail-jumps
into `.themida`; everything after that address disassembles as noise, which is exactly the
`halt_baddata()` blind spot, and `Xrefs` finds no references to it because its callers are
virtualised too. So the overflow happens inside the VM, in a body we cannot read, into the
`0x200`-byte local that sits directly beneath the cookie.

That Nexon chose to virtualise this particular function says it is security-relevant, and a
4668-byte function taking `(ptr, ptr, int, bool)` that builds something into a 512-byte
stack buffer on a timer has the shape of a periodic report builder.

### SOLVED: it is a server-reachability check overrunning its own buffer

The second `-FastFail` run answered it. `FUN_142e9ebd0` was entered **exactly once**, and
`__report_gsfailure` followed 150ms later:

```text
21:22:34.185 WATCH #1: 0x142e9ebd0 ENTERED ... rcx=0x14cb18 [0x0000067c]
                       rdx=0x14cb20 [0x0009000a] r8=0x14 r9=0x1 called-from=0x1415db7ac
21:22:34.335 WATCH #1: 0x142ef3e44 ENTERED ... called-from=0x142e9fe03
21:22:34.601 EXIT code 0xC0000409 after 36.7s
```

Once, not repeatedly - so it is on a timer, not a loop that eventually goes wrong.

`.pdata` puts the caller in `0x1415db360 .. 0x1415db7d6`, next door to the packet
dispatcher at `0x1415d60e0`, and **that function is not virtualised**. It builds a table of
twenty 4-`u16` groups on its stack and passes it as the second argument with `0x14` - 20 -
as the third. `rdx` dereferenced to `0x0009000a` in the log, which is `10, 9`: the first two
octets of the first entry. They are IP addresses:

```text
 1. 10.9.2.131        8. 44.234.161.18     15. 44.234.176.71
 2. 10.9.2.132        9. 44.234.171.239    16. 44.234.175.183
 3. 10.9.2.133       10. 44.234.78.153     17. 44.234.167.70
 4. 44.234.166.161   11. 44.234.182.63     18. 44.234.181.229
 5. 44.234.167.163   12. 44.234.171.56     19. 166.117.115.214
 6. 44.234.163.43    13. 44.234.162.137    20. 166.117.144.41
 7. 44.234.175.85    14. 44.234.159.5
```

Three Nexon-internal addresses, fifteen on AWS `us-west-2`, two more elsewhere.

**So the mechanism, end to end:** about 36 seconds after launch the client runs a
server-reachability check over twenty hardcoded addresses. The routine that does the work is
virtualised and writes its result into a `0x200`-byte stack buffer that sits directly under
its `/GS` cookie. **The patched client is firewalled, so all twenty fail**, and the
all-unreachable path overruns that buffer. `__security_check_cookie` catches it in the
epilogue, `__report_gsfailure` raises `int 0x29`, and the process dies with `0xC0000409`.

It is a latent bug in the client, on a path that never runs in production because the
servers are always reachable, and runs on every one of ours because they never are.

### The fix, and why it does not touch the firewall

Neither the buffer nor the routine can be fixed - the body is inside the VM. So the call is
skipped. `-Probe` grew a `:ret` option: log the entry and return immediately, leaving the
`int3` planted. Pointed at `FUN_1415db360`, the check never runs.

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-charselect.ps1" -SkipNetCheck
```

**CONFIRMED WORKING.** The client lived **92.7 seconds** and exited with code
`0x00000000` when closed by hand - no `__fastfail`. And the check turns out to be
**periodic, not a one-shot**: `FUN_1415db360` was entered at +36.2s, +66.2s and again
moments later, each returning harmlessly. `rdx` on those entries reads `30000` and `60000` -
millisecond timer values - and `called-from` is inside `.themida`, so the scheduler is
virtualised too. Every 30-second cycle would have killed the client.

**This is a client patch, and it does not make anything reachable - it stops the client
asking.** Report it as a patch. The `/GS` site stays armed alongside it, so if the client
dies anyway the log says whether it was still a cookie failure - a second overflow - or
something else.

**`141b2a280:rdx=0` must be in this mode's watch list**, and was missing from its first
version: without it the "trouble logging in" dialog blocks the tick that enables the Login
button, which cost a run. All four slots are used now, and the reason for each is in the
script.

Firewall untouched. Turning it off would presumably also stop the crash, by letting the
check succeed, but that means letting the patched client reach Nexon - which is what the
rule exists to prevent, and is the owner's call rather than ours.

**Do not** re-test the keepalive, re-watch `RtlExitUserProcess`, go looking for an external
killer, or blame our own patches. All four are settled, each by a measurement. And do not
re-quote "63 `int 0x29` sites" - that was a byte scan, and the real number is 8.

## Character creation: what works, and what is still fake

The protocol is finished and every opcode measured. Four problems found in the first
unlimited-length session, all fixed, all worth knowing about:

| seen on screen | cause | fix |
|---|---|---|
| the created character is naked | the `0x0015` reply was a canned body built from `Character::default()` | `--build`, which runs `packet-hex create-result-from` over the request itself |
| a second character never appears | that canned body carried **id 200 every time**, so the client was told it had re-created the character it already had | the builder takes an id and increments it |
| wrong hair, and no equipment | **the avatar look wrote `face` and `hair` one field too early** - see below | field order corrected, and pinned by a test |
| "Choose another world" freezes the UI, and re-entering a world hangs | `0x0082` was never answered, and the world sequence was one-shot | `0x0082` answered with the world list; the whole `0x0080` sequence is now standing |

### The avatar look field order - the subtle one

`FUN_1402ee8d0` reads `u8 gender`, `u8 skin`, three `u32`s, a discarded byte, one more
`u32`, then the equipment pairs. We were writing `face` and `hair` into the first two
`u32`s. The destinations say what those fields really are: the third `u32` goes to
`+0x1bd`, far from the look block, and the last goes to `+0x39`, which is **index 0 of the
equipment array** the pair loop fills at `+0x39 + slot*4`. That loop rejects anything
outside slots 1..31, so index 0 can only be written by the standalone field - and it is the
hair. The Swordie source names the same run `0, face, job, pad, hair`, and the client's
reader agrees with it. Correct order:

```text
u8 gender | u8 skin | u32 0 | u32 face | u32 job | u8 pad | u32 hair | pairs, 0xFF | pairs, 0xFF
```

**Why the tests did not catch it:** the record round-trip test *skips* the look block by
size rather than reading it, so it passed with the fields transposed.
`the_avatar_look_puts_face_and_hair_where_the_client_reads_them` now asserts the exact byte
run, and was checked by putting the bug back - it fails - and taking it out again.

### Now persisted

That was all measured against the harness, which generated the list from `-Characters` at
launch and persisted nothing. `crates/login` stores it. The four fixes above are still the
reason the transaction works, and the builders they corrected are what the server uses -
in particular the avatar look field order, which is pinned by a positional test.

## THE GOAL (set 2026-08-17) - reached on the wire 2026-08-18

**The server processes an entire character creation transaction.** The owner set this after the
masked email landed and the "connection dies" problem turned out not to exist. `crates/login`
does all four steps below against real storage; what has not happened yet is a client launch
to watch it.

That means, end to end and against a real server-side implementation:

1. the client reaches CharSelect with a **character list we sent**;
2. it asks the server to **check a name**, and the server answers;
3. it sends the **create request**, and the server creates the character and answers;
4. the client returns to CharSelect **with the new character in the list**.

### Where things stand

| | |
|---|---|
| Login screen | done, `0x0032` |
| Login button lit and clickable | done, `0x000B` world entry sets `stage+0x108` |
| Masked email on the login screen | **done**, `0x0000` - no client patch needed |
| Transition to character select | done |
| Connection stays up | **not a problem** - the "reset" was our own log format, see the retraction below |
| Character record, 327 bytes | **MEASURED** - drawn on screen with the exact stats sent |
| Character list | **MEASURED**, `login_result` |
| "Create a character" button | **works**, but only with the `create=on` client patch |
| Name check `0x0081`/`0x0014` | **MEASURED** both ways |
| Create request `0x008A` | **MEASURED** - virtualised builder, so a capture was the only way |
| Create result `0x0015` | **MEASURED** - the client returns to CharSelect with the new character |
| Client exits ~37s after launch | **FIXED** - a firewalled reachability check overran its buffer. `-SkipNetCheck` skips it; the runs since use the probe patch `watch@1415db360:ret` instead, which is what `test-server.ps1` arms and why the launch line carries no `-SkipNetCheck` |
| Server-side creation | **done** - `crates/login` reads the request, stores the character, and replies from the stored row |
| Characters persist between launches | **CONFIRMED on screen** - the same three characters have come back on every launch since |
| Name check answered truthfully | **done** - available / already used / not allowed, from the database |
| Valid session | still faked by client patches, and the game socket carries no credentials at all |

### The whole transaction, as measured - and now as served

| # | client sends | we answer | builder |
|---|---|---|---|
| 1 | `0x0080` world info request | `0x0000` account, `0x000B` world, `0x000B` end, `0x0010` login result | `account_info`, `world_list_entry`, `world_list_end`, `login_result` |
| 2 | `0x00A8` open creation (placeholder PIC, `01 00 2e`) | `0x05F4` `00 00` | `enter_creation_permitted` |
| 3 | `0x0081` check name | `0x0014` name + result | `check_name_result`, or `--answer 0081=0014:<req>00` |
| 4 | `0x008A` create, 101 bytes | `0x0015` result + record | `create_character_result` |

`0x00A8` and `0x0081` arrive on **every** click, so they need standing answers, not
one-shots. `crates/login` is stateless about all of them except the login request, which it
remembers only to decide whether to repeat the startup gate to a quiet client.

### Read these first
* **`docs/login-server.md`** - the login server: what it answers, how it stores, what is
  not authenticated, and how it was checked without a client launch.
* **`docs/deployment.md`** - running the server on another machine, and what breaks first.
* **`docs/launcher.md`** - the launcher, the patch inventory, and how each patch retires.
* **`docs/character.md`** - the whole transaction, the complete record layout, the NewChar
  screen, the create-request body, and what is still not established.
* `docs/opcodes.md`, `docs/session.md`, `docs/handshake.md`, `docs/transport.md`.
* `crates/net/src/opcode.rs` - every builder, every constant, each documented with how it
  was established. The tests there are the specification.
* `research/msexe-charstats.c`, `msexe-charrecord.c`, `msexe-avatarlook.c`,
  `msexe-newchar-ui.c`, `msexe-char-create.c`, `msexe-createflag.c`,
  `msexe-createbutton-gates.c` - the decompilation this rests on.
* `research/fixtures/` - the logs behind each claim, named for what they show.

### The tools, and what each is for

| tool | use |
|---|---|
| `tools/test-server.ps1` | **the run to use.** Builds, installs the hook, starts `maplecw-login`, applies the client patches, launches the client. `-Stop` tears down, `-ListOnly` prints stored characters |
| `maplecw-login` | the login server. `--list` prints what is stored without listening; `--bind`, `--db`, `--account`, `--world` |
| `tools/login_smoke.py` | a **stand-in client** over `transport.py`: proves the transport and every reply without a client launch. `--check-quiet`, `--list-only` |
| `tools/test-charselect.ps1` | the old harness. Canned bodies, no persistence - still the right tool for capturing packets or trying a hand-written body |
| `tools/test-one.ps1` | the general harness underneath it |
| `packet-hex` | prints a reply body from the Rust builders, so hex is never typed by hand |
| `tools/handshake_probe.py` | the old stand-in server. `--answer` standing, `<req>` splices the request's payload, **`--build IN=ELEMENT` computes a reply from the request by running `packet-hex`**, `--keepalive`. **Superseded by `crates/login`** for serving; kept for capture |
| `tools/transport.py` | the cipher, the framing, and the client-stream decoder |
| `-Probe watch@A,B,C` | up to four `int3` watches, each logging the calling thread id; `<module>!<export>` for relocated modules; options `:rdx=` forces an argument, `:peek=` logs `[rcx+off]`, `:hits=` sets the per-target log cap (default 32) |
| `tools/exit-forensics.ps1` | how the client died, from outside: exit code, thread table, job membership, handle holders. Started automatically by `test-one.ps1`; verified against a killed and an orderly control |
| `tools/handle-holders.ps1` | which processes hold a handle to a given pid and may terminate it. Read-only; **needs elevation** or the list is silently short |
| `tools/pdata_lookup.py` | exact function bounds from the PE exception table - **use when Ghidra has no function** for an address, rather than letting `DecompileFunc` create one at the wrong place |
| `tools/ghidra_scripts/FindFastFail.java` | the real `int 0x29` sites, by walking the disassembly rather than scanning bytes |
| `-Session mode=2,create=on` | the client patches, comma separated |
| `tools/ghidra_scripts/DecompileFunc.java` | decompile by address, creating the function if Ghidra has none |
| `tools/ghidra_scripts/Xrefs.java` | callers, and data references |
| `tools/ghidra_scripts/DumpAsm.java` | raw listing for a VA range - **use when the decompiler says "bad instruction data"** |
| `tools/ghidra_scripts/DumpData.java` | bytes as ASCII and UTF-16 - turns `&DAT_1433881d0` into `"new"` |
| Windows Application event log | records real client crashes; verified working |

### The Swordie comparison - how to use it

`C:\Users\user\Desktop\ModernMapleSource` holds a Swordie-family server (`v214 src`) for a
modern MapleStory. This client is **MapleStory Classic World**: a modern engine running
classic content, so the source matches its *structures* but not its *numbers*.

**What it earned:** `CharacterStat.encode` agreed with `FUN_140302e30` field for field
across the record head, which turned offsets into named stats; `selectWorldResult`'s
three-list shape matched `FUN_14108d290` + `FUN_14108bdf0`; `checkDuplicatedIDResult` and
`createNewCharacterResult` matched `0x0014` and `0x0015` body for body; and
`src/main/resources/ins.txt` names the inbound opcodes, ours being its names shifted by ten
in the character range.

**How to use its numbers - this is the nuance, and it cost a pass in both directions.**
The offset is *not* constant across the whole range, so a number cannot be trusted. But
`0x008A` for the create request was predicted exactly by its `CREATE_NEW_CHARACTER(141)` at
an offset of -3, and that prediction was **discarded** because the opcode was absent from
`research/msexe-send-opcodes.txt` - a scan of `FUN_1406ed520` call sites, and therefore
blind to a builder inside the VM. So: treat its numbers as **hypotheses worth testing**,
never as facts, and remember that **absence from the send-opcode table is evidence of
virtualisation, not of non-existence**.

### Standing warnings

* **`-Session mode=2,create=on` and `-Probe watch@141b2a280:rdx=0` are client patches.**
  They make the normal flow reachable; they do not make the session valid. Say so when
  reporting.
* **Rebuilding `grap-stub` does not update the client** unless `setup-client.ps1` runs.
  `test-charselect.ps1` and `test-server.ps1` do this themselves; anything else must.
* **`cargo test` does not refresh `target/release/maplecw-login.exe`.** The same trap one
  layer up: a test run leaves the release binary stale, so a manually started server can be
  a build old enough to predate the feature under test. That happened on 2026-08-18 and the
  smoke test caught it. `cargo build --release` before starting anything by hand.
* **`powershell -File` flattens array arguments** into separate words, so a `[string[]]`
  parameter silently takes only its first element and the rest bind positionally. Pass
  delimited strings.
* **Check the console line `standing answers (N)`** before reading anything into a run. A
  missing answer leaves the client on "Connecting..." and looks like a client problem.
* **Get the date from `git log`, not from a guess.** Several notes in this repo were
  written with dates two days ahead of the commits they describe, which made a single
  afternoon read as three days of separate work. Corrected 2026-08-17.
* **An instrument that has never been seen working proves nothing by staying silent** -
  and a *discriminator* has to be shown to discriminate, not just to run. The thread-drain
  test for the exit ran perfectly and reported confidently, and was still wrong: both
  controls looked identical under it.

## Working right now

```bash
cargo test --release          # 107 tests green
cargo build --release
```

**The client runs and connects to our server:**

```bash
# 1. once: create the account the server serves
./target/release/maplecw-useradd.exe maplecw
# 2. start the login server
./target/release/maplecw-login.exe
# 3. launch the patched client (from client-patched/)
MapleStory.exe -NXLDEBUG 127.0.0.1 8484
```

`tools/test-server.ps1` does all three. The client connects to `127.0.0.1:8484`, and
GameGuard never loads.

## Done

| | |
|---|---|
| `crates/wz` | WZ parser. **9,994/9,994 images** across 102 archives parse. `wz-dump` CLI. |
| `crates/net` | **The client's real wire cipher**, verified against captures, plus the recovered inbound opcodes and their bodies, and `packet-hex` to put a body on a command line without typing it. 28 tests. |
| `crates/store` | SQLite accounts/sessions **and characters**. argon2id, per-password salt, hashed single-use tokens; `characters` + `equipment` tables cascading from `accounts`. 33 tests. |
| `crates/auth` | Local HTTP auth server (loopback only) + `maplecw-useradd`. Verified end to end. |
| `crates/login` | **The login server.** Greeting, the protocol as a pure state machine, and the socket loop. Characters persist. 21 tests, plus an end-to-end check against a stand-in client. |
| `crates/grap-stub` | No-op `grap64.dll`; GameGuard never starts. Plus the **in-process dispatcher hook**, the opcode walk / watch probe, a session monitor and patcher, and a socket watch over `connect`/`closesocket`/`shutdown`. |
| Client copy | `client-patched/` — original install untouched, firewalled outbound. |
| Tooling | `handshake_probe.py` decodes the client's live stream; `dump_runtime.py` reads its memory. |
| Canvas render | `wz-dump canvas` + `tools/wz_png.py` turn WZ canvases into PNGs. **This is how the client's baked UI text gets read** - much of its on-screen wording is pixels, invisible to any string search. Formats 1, 2 and 513. |

## Key facts (do not re-derive)

- **WZ data version 779**, hash `0x0000E73A`, **zero** string key.
- **Network protocol version is 100** — unrelated to 779. Don't conflate them again.
- Launch: **`-NXLDEBUG <ip> <port>`** is the only mode that runs *and* connects, and it is
  **not** a debug mode - it sets the same launch mode (5) as `-NXL`, which is what the real
  Nexon Launcher passes. `IPPORT` does not crash in the parser: it whitelists the IP against
  six Nexon literals and bails without setting a mode. `WEBSTART` needs token 1 non-empty.
  Settled statically 2026-08-19; see `docs/launch-protocol.md`.
- Handshake framing: **`u16` little-endian body length, then the body** (length excludes
  itself; the client rewinds over the prefix). Confirmed working.
- `MapleStory.exe` is **Themida**-protected with a rebuilt IAT — do not patch it on disk.
  Find code via **string xrefs**, never import xrefs. That technique has worked four
  times now.
- The client cannot be killed with `Stop-Process`; use `taskkill /F`.

## Transport: SOLVED IN BOTH DIRECTIONS

**The handshake is solved.** `FUN_1415d10e0` line 606 rejects the connection unless fields
`G == 1` **and** `H == 1`, raising the *same* `0x22000007` "client is outdated" error as a
version mismatch, unconditionally — which is why every early version sweep looked
identical. Full table in `docs/handshake.md`.

**The packet transport is solved**, and the client both accepts our frames and has its own
stream fully decoded. See `docs/transport.md`.

```
len     = a ^ b                     # two u16 LE; no byte-swap, unlike classic MapleStory
a       = ((iv >> 16) & 0xFFFF) ^ K # K = 0xFFFE for packets we send, 0x00DF for the client's
payload = AES-256-OFB(key, iv repeated 4x)   # chunks 0x5B0 then 0x5B4
iv       -> stock shuffle table at 0x143A86890, rolled once per packet
```

Our chain seeds from `K`, the **second** u32 of the greeting (`conn+0xec`); the client
transmits on `J`, the first (`conn+0xe8`). Lengths `>= 0xFF00` use an 8-byte header.

### The AES key is a decoy on disk — do not "fix" it

The table at `0x143A86810` holds the **stock** MapleStory key in the file, and the client
overwrites the low byte of all 32 dwords at startup. Only that table — the shuffle table
beside it is untouched, which is exactly why framing, the header constant and the IV chain
were provably correct while everything AES-shaped failed in *both* directions at once.

```
0f 00 00 00  1b 00 00 00  c5 00 00 00  46 00 00 00
f3 00 00 00  be 00 00 00  ff 00 00 00  75 00 00 00
```

Read with `tools/dump_runtime.py` (read-only, needs an elevated shell), stable across
sessions, so it is a build constant. `the_disk_key_is_a_decoy_and_does_not_decrypt` guards
against reverting it.

With it, every captured packet matches its decompiled builder field for field — packet 1 is
`70 00 02 64 00 00 00`, exactly `FUN_1415d5b40`'s `u8 2, u32 100`.

### What the client sends, and what it waits for

Login connection startup, read from the handshake tail (`conn+0x48 != 0` selects it):
**`0x70` version, `0x71` environment, `0x8F`/`0x90`/`0x91` log uploads, optional `0xA1`** —
then the handler *returns*. The hang is in the main loop, waiting on the socket. The eleven
6-byte packets are `0x00A6` carrying an incrementing id.

`0x8F`-`0x91` read a file up to 8 KB, upload it and delete it; they need no reply, and they
are why opening bursts varied 294 to 3393 bytes between runs.

### All earlier sweep results are void

Every sweep predates the key fix, so the client never saw an opcode we intended, and the
scattered exits at `0x0023`, `~0x01DC`, `~0x01F1`, `~0x03C5` were random garbage opcodes
hitting a disconnect handler — none reproduced, and `0x0023` sent alone did nothing.

Note the trap that hid this: **acceptance only proves the header**. A bad payload decrypts
to a random opcode and is silently ignored, not rejected.

### The startup gate is solved - the client reaches its login screen

**Inbound opcode `0x0032`, body `0x00`.** Seven bytes on the wire, and the client goes from
a blank non-responding window to the login screen. Verified with a single packet and no
probe: `flag=0->1 state=0->2`.

It was never a login handshake. The client hashes `Data.wz` into `conn+0x14c`, sends
`0x00A1` carrying that `u32`, and blocks in `recv` **on its UI thread** inside
`FUN_1415e7090`, looping recv -> decrypt -> dispatch until a handler sets the byte at
`conn+0x150`. Only `FUN_1415e5c20` does that, and it is a `Data.wz` patch handler whose
first field is a **zigzag varint** length (`FUN_1406efcc0`):

| length | client does |
|---|---|
| `0` | nothing to patch - sets the flag and carries on |
| `> 0` | expects that many bytes in 64 KB chunks, then writes `Data.wz` |
| `< 0` | deletes `Data.wz` and carries on |

This client ships no `Data.wz` at all (a `Data/` directory instead), so it sends hash `0`
and a varint `0` is the right answer. See `crates/net/src/opcode.rs`.

That also explains the old "26 packet ceiling": every unhandled packet allocates a `0x5b4`
buffer inside that loop and the loop never exits to free them. A leak, not a limit.

### The login exchange

**Superseded in part:** "the client logs in by itself" is true only in **mode 5**
(`-NXLDEBUG`), where a per-frame tick calls the same function the Login button calls. With
`-Session mode=2` the client waits for the button, which is the real flow. See "THE GOAL"
at the top.

After the gate the client sends:

```
0x00C0  05 00 00 00 20 4e 00 00
0x0073  26B  05 00 00 00 00 00 aa bb cc dd ee ff de ad be ef...   <- 20 bytes, constant
0x0080  (empty body)                                              <- the login request
0x007A  01 01 4x 00 00 00 ...
```

then waits **4-7 seconds** and abandons the connection. `0x0073` and `0x0080` are both
built by `FUN_141b21ea0`, the function that loads `UI/Login.img`.

**The reply is inbound `0x0010`**, and this is the structural find of the session: the
login stage's `OnPacket` is `FUN_141b25f30`, and it is an **ordinary readable switch on the
opcode**. The Themida-virtualised dispatcher hands a stage its opcode; the stage dispatches
in plain code. So the whole login-stage opcode map is readable:

```
0x00, 0x0b-0x18, 0x23, 0x25-0x27, 0x29, 0x2b, 0x34-0x39, 0x45-0x48, 0x4a, 0x50, 0x5f, 0x5f4
case 0x10 -> FUN_141b307b0    the login result
```

Watch mode confirmed at runtime that `FUN_141b307b0` **is entered while dispatching
`0x0010`**, so the opcode and the stage are both right.

Body of `0x0010`, from `FUN_141b307b0` and `FUN_1406e9050` (strings are `u16` length then
bytes):

```
u8  result
str message
if result == 0:    u8, 8 bytes, u32, u32, 4B, 4B, 4B, u32, u8,
                   then FUN_14108d290 and FUN_14108bdf0 read further
if result == 0x83: two more u32
```

**Result `0` is success.** `FUN_141b267c0(this, result, 0, ...)` raises the error dialog,
and the proceed branch is `cVar6 != 0 && result == 0`. `0x65`/`0x67` are *not* success -
they take a different branch that re-sends `0x0080`. Misreading them as success cost three
runs of the same dialog.

Corroborated independently: non-zero results are **error message IDs**, resolved through
`FUN_141803cd0` in `docs/client-messages.md`. `0x65` is 101, *"You have been disconnected
from the login server"* - exactly the dialog that replying `0x65` produced.

### DONE - the login result is accepted, the client reaches character select

`0x0010` with `body = 00 00 00` + 256 zero bytes ran the success path to completion and the
client's UI **advanced to character select**. Compare `0x65`, which dropped the connection
in 0.0 s with no follow-up. Fixture:
`research/fixtures/reply-0010-result0-advanced-to-charselect.log`.

Full field list in `docs/opcodes.md`. Two fields matter beyond filler: the `u32` world id
and `u32` channel id, which the client looks up in a world list it does not yet have.

### The whole login stage has two variants, and we are in mode 5

**Check this before decoding any login-stage handler.** Several open with

```c
if (FUN_142c4a810(DAT_143ac1898) == 5) { <other handler>(...); return; }
```

`session+0x68` is **5** in our client - transmitted as the first `u32` of `0x0073`, captured
as `05 00 00 00`. Mode 5 is what **`-NXL`, `-NXLDEBUG` and `-NXLPTS` all** set, so it is the
production Nexon-Launcher mode rather than a debug one, and it is how we launch. So the
mode-5 branch is always the live one and the handler the switch names first is dead code
for us. `0x000B`, the login flow, and the Login button all fork this way. Decoding the
wrong side costs a full analysis pass. Table in `docs/opcodes.md`.

### The Login button is enabled by one byte, and `0x000B` sets it

The owner: the button starts **disabled** in an invalid session. `FUN_14112a720`, the
`ClassicIntro` tick, enables the control named `"login"` only when
`FUN_141b2a160(stage)` - that is, `*(u8 *)(stage + 0x108)` - is non-zero. The screen
builder `FUN_141129930` creates it disabled.

`stage+0x108` is written by the **world-list handler**, one line above the list append:

```c
*(undefined1 *)(param_1 + 0x108) = 1;
piVar10 = (int *)FUN_141b44520(param_1 + 0x100, 0xffffffff);
```

So **inbound `0x000B` enables the button**, populates the list the login result searches,
and is the one thing missing since the client first reached the login screen. Only a real
world entry does it - the terminator branch returns before both writes.

The account field is a different object: `FUN_142cb83a0` is `DAT_143aa84a0 + 0x22f8`,
rendered into `textAccount` when non-empty. `DAT_143aa84a0` also holds world id `+0x2258`
and channel id `+0x2260`; it is **not** the `DAT_143ac1898` that carries the `0x0073`
identity.

### MILESTONE - login screen -> Login button -> character select

**Reached 2026-08-17.** The owner clicked a lit Login button, the client played its animated
transition into character select, and "Create a character" was the blocker - the goal set
at the start of the day.

The recipe, all four parts needed together:

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4 -HookLog on
  -Session mode=2 -Probe watch@141b2a280:rdx=0 -ReplyTo 0x0080
  -ReplySeq "000b:0006005363616e6961000000000108005363616e69612d30000000000000000000000000000000"
```

| Part | Why it is needed |
|---|---|
| `0x0032` gate | releases the startup loop; login screen appears |
| `0x000B` world **entry** | sets `stage+0x108`, which is what enables the Login button |
| `-Session mode=2` | leaves mode 5 so the per-frame tick stops auto-logging-in and the button gets a turn |
| `-Probe watch@141b2a280:rdx=0` | suppresses the "trouble logging in" dialog, which otherwise **blocks the tick** and stops the button ever being enabled |

**No terminator.** With the mode patched, a second `0x000B` is handled by the classic
`FUN_141b2fac0`, whose terminator transitions to WorldSelect - a screen this service does
not use.

**Two of those four are client-side patches.** They make the client's normal flow
reachable; they do **not** make the session valid. Describe results accordingly.

**Everything after the login screen was offline.** The client closed the connection at
8.4s - immediately after sending `0x007A`, its loading-complete report - so the Login click,
the transition and the "Create a character" clicks all happened with no server attached,
and **sent nothing**. Those transitions are purely client-side.

**Adding the login result back did not keep the connection alive.** It *was* dispatched and
handled (`2 opcode=0x0010 ... ret=1`), and the client closed 0.42s later, exactly as it does
without one. So the close is not a rejection of our reply.

### RETRACTED: "the client does not migrate" was never established

This section used to read "SETTLED: the client does not migrate", on the strength of a
`connect` hook that logged **nothing**. That was the same silent-negative mistake the repo
warns about everywhere else, and it took two runs to notice.

**The hook has never logged a `CONNECT` line at all — including for the connection to
`127.0.0.1:8484`, which certainly happened.** So "no connect was logged" says nothing
about the client's behaviour until the hook is shown to work. It may be that the client
reaches its socket through a path `ws2_32!connect`/`WSAConnect` do not cover; it may be
that the hook is simply broken. Either way the migration question is **open**, and so is
everything that was inferred from it.

`netwatch` now runs a **self-test** at install: it makes its own loopback `connect` and
`closesocket` and reports whether its handler caught them.

```
netwatch: SELF-TEST ok - 2 of our own calls were caught, so a later absence of lines is a
real negative
```

Read that line before reading anything else from this hook. `SELF-TEST FAILED` means every
negative it reports is worthless.

**What is still true:** `tools/watch-sockets.ps1` saw only one socket, and no second
endpoint was ever observed. That is weak evidence for one connection, not proof.

**What was inferred from the retracted claim, and is now unsupported:**

* that no channel server is needed;
* that the unread fields in the login result cannot be a server address.
* **The close is *not* explained.** It was recorded here as an ~8s idle timeout; that was
  inferred from timing alone and the timing has a second explanation - see "CORRECTION"
  above. What is settled is only that no reconnect follows it.

### CLOSED - "the client has no character list"

This section used to say the missing character-list packet was the next thing to find. It
was found and it is done. The list is inside `0x0010` (`FUN_14108d290` then
`FUN_14108bdf0`), it has been sent, and the client draws it. `FUN_141b28570`, nominated
here on the strength of where it is called from and never actually read, turned out to be a
290-byte state check with nothing to do with character lists.

The inert screen had a second cause that outlived the list: `FUN_141b282d0` gates the
"Create a character" button on three `0x0010` tail fields **and** on an obfuscated flag the
handshake sets to zero. See `docs/character.md`.

### SOLVED - `FUN_141b2a280` raises the prompt

`called-from=0x141b2a61e` at a watch on the notice display named it.
`FUN_141b2a280(stage, code, flag)` shows `loginTroubleAskSupport` for **codes -1, 6, 8, 9
and 12** (`0x2681` bit-tested at `code + 1`); `code == 0` is success. Full code -> notice
table in `docs/session.md`, and the whole baked dialog table in `docs/client-notices.md`.

It is a near-duplicate of `FUN_141b267c0` - same mapping, different function.

**Why it hid for three sessions, and the lesson:** it never takes the string's address. It
**copies the literal inline** with RIP-relative `mov`. `tools/xref.py` matches `lea`, so it
reported three references, all of which were then proven never entered - and the real raiser
was invisible to every scan built on it. A "0 references" result means *nothing takes its
address*, not *nothing uses it*. That warning is now at the top of `xref.py`.

Independently corroborated: an exhaustive render of all 170 `/Notice/` canvases found no
duplicate node and no numeric twin, so the dialog on screen is definitely this one.

### The code is 12, and the caller is virtualised

```
WATCH #1: 0x141b2a280 ENTERED  rdx=0xc (as i32 12)  r8=0x1  called-from=0x144c05eb2
```

* **`code = 12`** - in the trouble set, and a *generic* failure: no specific notice maps to
  it, unlike 4 (`incorrectPassword`) or 5 (`notRegisteredID`). The client is not reporting a
  named reason, it is reporting "login did not succeed".
* **`r8 = 1`** - the flag argument, which sets `stage+0xf0 = 1`.
* **`called-from = 0x144c05eb2` is inside `.themida`.** The immediate caller is virtualised
  and cannot be decompiled.

No packet carried this. Only `0x0032` was ever dispatched, so the client generated code 12
on its own.

**The stack walk is a dead end, and that is settled.** A 0x400-byte, 16-slot scan of the
stack at the call found exactly one image address: the VM return address itself.

```
stack: 0x144c05eb2(vm)
```

No `.text` frames at all. Themida runs the VM on **its own stack**, so the caller chain is
not there to find and widening the scan only reads more VM stack. **Who decided code 12
cannot be answered by walking back from the call.**

Two ways forward, and they answer different questions.

**1. What does the client do if it believes login succeeded?** `FUN_141b2a280` returns
success for code `0`, so rewriting the code at its entry answers that in one run.

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4
  -HookLog on -Probe watch@141b2a280:rdx=0 -Session watch
```

This is a **client-side patch** - it makes the client stop concluding it failed; it does not
make the session valid. Say so when reporting results. What it buys is the rest of the
flow: whether the login screen becomes usable, and where the client gets stuck next.

**2. Why the client concludes failure.** The decision is virtualised, but what it *consults*
need not be. The `CNM*` session interface lives in `nexon_api_x64.dll` / `nmcogame64.dll`,
both **unpacked** - readable statically and hookable at their exports. That is the honest
route to a genuinely valid session, and it needs no client runs to start.

### How it was found

A watch on the notice display `FUN_141b4ac80(name, ...)` caught it:

```
WATCH #1: 0x141b4ac80 ENTERED  rcx=0x14d148 [0x057f5ed8] "loginTroubleAskSupport"
          rdx=0x14d100  r8=0x0  r9=0xe
```

Settled by that line: the dialog **is** `loginTroubleAskSupport` (not some similar node),
it is raised through `FUN_141b4ac80`, and the name arrives **intact** - so it is not built
at runtime.

Which leaves a genuine puzzle. Something loaded that name, but:

* the `.rdata` literal at `0x1433d5d98` has exactly **three** code references, and all three
  are proven never entered;
* there is **no pointer-table reference** either - a scan for the qword `0x1433d5d98`
  anywhere in the file finds nothing;
* `rcx` pointed at a **heap** copy (`0x057f5ed8`), not the literal.

The leading explanation is that the caller is **virtualised**: a `lea` inside Themida VM
bytecode is invisible to every static scan we have. If so, static analysis is finished here
and everything further must be measured.

**Next:** watch mode now logs `called-from`, read from `[rsp]` at the breakpoint - the
breakpoint sits on the function's first byte, so the `call` has just pushed the return
address. Re-run the same command; the caller names itself.

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4
  -HookLog on -Probe watch@141b4ac80 -Session watch
```

A `called-from` inside `.themida` (roughly `0x144C0000`+) confirms the virtualised-caller
theory. Anything in `.text` names a real function to decompile.

#### Older note, now superseded

**One question, and it has a designed experiment:** what result code reaches
`FUN_141b267c0`, and when? The dialog is raised for result -1, 6, 8 or 9, but the owner sees it
*before* any login exchange, and `0x0032` (handled by `FUN_1415e5c20`) never touches that
path. Two of its callers - `FUN_141b2b120`, a 31-byte wrapper that passes the code straight
through, and `FUN_141b2ae80` - have no callers and are in no vtable, so they are reached
only through the virtualised dispatcher and **cannot be traced statically**.

So observe it. **`-Probe watch@<VA>` now does this**: it reports *every* entry with the
dispatching opcode and the first four integer arguments (`rcx`, `rdx`, `r8`, `r9`), rather
than announcing one hit and disarming. `rdx` is the result code, and the switch above turns
that number into the dialog on screen.

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-one.ps1" -Reply ping
  -Opcode 0x0010 -PingFirst 0x0032 -PingBody 00 -ReplyTo 0x0080 -QuietBefore 4
  -HookLog on -Probe watch@141b267c0
```

Send only the gate, so nothing we send can be the cause: if the dialog still appears, the
failing code came from the client itself. Read `rdx as i32` out of the `***** WATCH #n`
lines in the hook log.

One known limit: it stops logging after 32 hits so a per-frame caller cannot fill the disk.
(The old "can only arm once the hook has seen a dispatch" limit is **gone** - watch now
arms from `install()`, which also removes the race that twice brought the dialog back.)

**The auto-advance is not a bug.** In mode 5 the `ClassicIntro` tick calls
`FUN_141b3ff10` - *the same function the Login button calls* - as soon as `0x000B` sets
`stage+0x108`. Mode 5 is the **Nexon-Launcher** mode, not a debug one (`-NXL` sets it too):
a launcher-started client already holds a session, so it logs in without the button. That is
why the flow does not match a normal server.

**To get the click-the-button flow**, write anything but `5` to `[0x143ac1898] + 0x68`
(`session+0x68`) from `grap-stub` once the world list has landed. Then the tick's
auto-login goes false, the button still enables (that happens as a side effect of the
`+0x108` check, independent of mode), and clicking Login takes
`FUN_141b3f050(stage, 4, 600)`.

> **RETRACTED 2026-08-19 (second pass).** A "correction" here previously said screen 4 was
> **world select** and that the original "straight to CharSelect" reading was wrong. **The
> correction was the error.** `docs/session.md` carries the stage table read out of
> `FUN_141127730`, which registers each id against a screen *name* - data, not inference:
> **1 Title, 2 WorldSelect, 3 ClassicIntro, 4 CharSelect, 5 NewChar.** So screen 4 is
> CharSelect and the original claim was right.
>
> What actually lands on world select is the **world-list terminator**: its branch in
> `FUN_141b2fac0` calls `FUN_141b3f050(param_1, 2, 400)` - screen **2**, WorldSelect. That is
> why sending the terminator put the client there, and it is the lever for
> "Choose another world". Switching modes sends `0x000B` to the
classic handler `FUN_141b2fac0` instead of `FUN_141b31ff0`, which is safe: their read
sequences were compared field by field and are identical. **Be honest about what this is** -
it makes the client follow the normal flow, it does not make the session valid.

### DONE - the world list

The login result makes the client search for its world in the list at `stage+0x100`
(`FUN_141b2c7c0`), and only inbound **`0x000B`** appends to it. Format decoded from
`FUN_141b31ff0` (the mode-5 handler) and built by
`crates::net::opcode::{world_list_entry, world_list_end}`, with a test that re-reads the
bytes the way the client does.

Sent on every run since, and it works. **Use `tools/test-charselect.ps1`** rather than the
hand-written `-ReplySeq` that used to be here: it generates every body from `packet-hex`,
so the command cannot drift from the builders. That drift is not hypothetical - the
world-list hex on a command line was once two characters too long, and the only reason it
was caught is that a test happened to compare against the builder.

**Send the world entry only, never the terminator plus the mode patch.** With the mode
patched, a second `0x000B` reaches the classic `FUN_141b2fac0`, whose terminator branch
transitions to WorldSelect - a screen this service does not use. That cost a run.

### The session identity - see `docs/session.md`

Short version, because two long-standing assumptions turned out to be wrong:

- **"Having trouble logging in" is `/Notice/text/loginTroubleAskSupport`** - a baked bitmap
  in `Login.img`, which is why no string search ever found it. **Solved:** raised by
  `FUN_141b2a280(stage, code, flag)` for codes -1, 6, 8, 9 and 12; measured live as **code
  12**, a generic failure, from a **virtualised** caller. Three other candidates were ruled
  out by measurement first. Full table and the tooling lesson in `docs/session.md`.
  `Login.img` also has **two** login screens (`Title_new`, and `ClassicIntro` = ours, the
  one carrying `find_id`/`find_pw`); `FUN_141129930` builds `ClassicIntro`.
- **The empty identity did not block the login.** The client still sent `0x0073` and
  `0x0080` and accepted a `result = 0` reply. It is a real gap but not the current blocker.
- **The account name is server-supplied** (`0x0000` / `0x0012`), so the session may be too.
  That demotes the launcher-handoff theory this section was built around - see "THE GOAL".

The identity is one `char *` at **`DAT_143ac1898 + 0x1b8`**, read by `FUN_142c50400` and
sent as the second field of `0x0073`, where we captured a **zero-length string**. Nothing
computes it. The six `+0x90` launcher tokens are ruled out. Next: find its writer in
`nexon_api_x64.dll` / `nmcogame64.dll` (both unpacked), or write the field directly from
`grap-stub`, which is already in-process.

The old "constant 20-byte tail of `0x0073`" question is closed: it is a 16-byte GUID plus a
4-byte counter, not session data.

### The opcode walk, and how to aim it

`crates/grap-stub/src/probe.rs` walks the inbound opcode space **inside** the client:
snapshot one captured packet, rewrite its opcode, re-dispatch, watch an oracle. The whole
enum in one launch instead of ~2 opcodes per launch over the wire. Faults are caught by a
vectored handler and the loop resumes; `ExitProcess`, `TerminateProcess`,
`RtlExitUserProcess` and `NtTerminateProcess` are detoured so a handler cannot end the run;
progress is appended to a resume file so a fatal opcode costs one launch, not the search.

`-Probe <from>-<to>[@targetVA][#N]`, or `-Probe watch@<VA>` to observe whether a function
runs at all.

**Aim it with `tools/handler_root.py`, never by hand.** A walk target must be a dispatcher
entry: no direct callers **and in no vtable**. `FUN_141b25f30` has no callers but *is* a
vtable entry, and aiming at it burned a full 3968-opcode run that missed cleanly.

**And time it.** `#N` starts the walk on the Nth dispatched packet. The walk runs inside
whatever loop the client is in, so walking for a login-stage handler before the login
screen exists cannot work no matter what address is used.

### Traps that cost time - do not re-learn these

**Protocol**

* The **on-disk AES key is a decoy**; read the real one from a running client.
* **Accepting a packet only proves the header.** A bad payload decrypts to a random opcode
  and is silently ignored, not rejected.
* Login result **`0` is success**; `0x65`/`0x67` are a different branch entirely.
* **Check the `session+0x68 == 5` fork before decoding any login-stage handler.** The
  handler the switch names is often a shim that hands off to the mode-5 one, and we are
  always mode 5.
* **The "trouble logging in" prompt is a bitmap, but it *is* a state readout.** It appears
  before any packet exchange, so no reply can clear it and it does not measure the wire.
  For wire questions use the identity string in `0x0073` and how long the connection
  survives a reply; for the prompt, look at which login screen was built.
* The client's opening burst varies **294 to 3393 bytes** because `0x8F`-`0x91` upload and
  delete log files.

**The walk**

* **Snapshot the packet before the dispatch, never after.** The dispatcher consumes the
  opcode and moves the cursor 4 -> 6, so an after-snapshot replays "opcode 0" every time:
  4096 dispatches, no faults, reported as an empty range.
* **One oracle per walk.** With a target armed, `conn+0x150` is *expected* to be set
  already, so consulting it as well reports a hit on the first opcode tested.
* **Aim at dispatcher entries** - no callers *and* no vtable. Use `handler_root.py`.
* **Time the walk** into the phase where the handler exists (`#N`).
* **Append resume records.** `fs::write` truncates first, so dying mid-write leaves an
  empty file and the next launch restarts from zero and dies in the same place.
* The probe detours `ExitProcess`, so the client survives and, being elevated, **cannot be
  killed from a normal shell** - use `taskkill /F /IM MapleStory.exe /T` from an elevated
  one, or the leftover holds port 8484 and the DLL file.

**The harness**

* **Answer on packet arrival, not on a timer.** The client sends `0x0080` at +4.4s, +6.1s
  or +7.2s and gives up seconds later; a fixed delay once fired 0.12s *before* the request
  it was meant to answer. Use `-ReplyTo`.
* `-QuietBefore` also gates timed replies, and the client is rarely quiet for that long.
* PowerShell variable names are **case-insensitive**: a `$probe` local silently ate the
  `-Probe` parameter.
* A parameter that never arrives looks exactly like one that arrives and does nothing -
  `test-one.ps1` echoes the real command line for that reason.
* Scripts are invoked as `powershell -ExecutionPolicy Bypass -File "<abs path>"` - and the
  path really is absolute, never relative. The window is elevated, so it opens in
  `C:\Windows\System32` rather than in the repo.
* **Rebuilding `grap-stub` does not update the client.** `cargo build` writes
  `target/release/grap64.dll`, but the client loads `client-patched/grap64.dll`, and only
  `tools/setup-client.ps1` copies one to the other. Skip it and the run silently uses the
  old hook - the most expensive kind of failure here, because it looks like the new code
  did nothing. Compare hashes if in doubt.

### Testing loop that works

The client's dialog is the oracle; it is not visible to the agent. Run **one variant at
a time** and have the owner report what they see. That loop found the framing, disproved the
version-check theory, and confirmed the `L` gate.

`tools\test-one.ps1` runs one variant: it starts the probe, launches the client at
BelowNormal priority pinned off core 0 (the client otherwise saturates the host while a
test sits waiting for a dialog to be read), and tears both down with `-Stop`.

The probe now **holds the connection open** (`--hold`, default 300s). Closing it early
makes the client's `recv` return 0, which sends it down its own disconnect path
(`FUN_1415d10e0` recurses with `param_2 = 0` → `0x22000001`) and looks exactly like a
rejected handshake.

## Housekeeping

**Run scripts with Windows PowerShell, from an elevated shell:**

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
```

`pwsh` is **not installed** on this machine - PowerShell 7 was never set up and the shell is
5.1 - so anything written as `pwsh tools/...` errors with "not recognized". This file said
exactly that in two places, and `firewall.ps1` and `setup-client.ps1` in three more, until
2026-08-19.

Elevation matters separately: `exit-forensics.ps1` runs from `test-server.ps1` and cannot
see SYSTEM-owned handles without it. It reports how many it could not reach, so a short
list is never misread as an empty one.

When editing these scripts, remember what 5.1 does not have: `&&`, `||`, ternary,
null-coalescing.

- Firewall rule `MapleCW - block patched client outbound` is **active**. Remove with
  `powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\firewall.ps1" -Remove`
  (needs elevation, which is also why the path is absolute - an elevated window opens in
  `C:\Windows\System32`).
- `client-patched/` has the GameGuard stub installed;
  `powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\setup-client.ps1" -Restore`
  puts the real DLL back.
- Ghidra projects in `research/ghidra/` (~1.2 GB, gitignored). `msexe`, `grap64`,
  `mssecure`, `nexoncm` are all analysed — reuse them rather than re-importing.
- **Ghidra: `docs/ghidra.md` is the full workflow** - read that, not this bullet. The two
  things that bite first are the JDK, below, and that **the project locks**: never let a
  subagent run it while you are.
- **Ghidra needs JDK 21, not 25.** Under JDK 25 the bundled Felix 7.0.5 aborts with
  `Bundle org.apache.felix.framework [0] The data file must be inside the data dir`.
  Prefix headless runs with:

  ```powershell
  $env:JAVA_HOME="C:\Program Files\Eclipse Adoptium\jdk-21.0.6.7-hotspot"
  $env:PATH="$env:JAVA_HOME\bin;$env:PATH"
  ```

  If it was already run under 25, also delete
  `%APPDATA%\ghidra\ghidra_12.1.2_PUBLIC\osgi\felixcache` (a regenerable script cache).
