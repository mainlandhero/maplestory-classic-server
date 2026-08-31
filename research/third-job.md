# The third job advancement: the people and the place exist, the test does not

Written 2026-08-31. **No Ghidra pass and no client run went into this.** Every number is out
of the client's own WZ via the generated `gm-handbook/*.txt`, plus this repo's source.

Labels are the project's: **[L]** read off this client's WZ dumps, its listing or a capture;
**[D]** derived from two or more [L] facts; **[I]** inferred — a policy nothing on this
machine can confirm.

**Nothing here authenticates.** As everywhere in this project, the channel socket carries no
credentials. A third job advancement would be granted on the say-so of whoever holds the
socket.

> **Companions.** `research/job-advancement.md` owns the packet (`0x007C` mask bit 5).
> `research/job-instructors.md` owns the first advancement. `research/second-job.md` owns the
> second advancement's decision and `research/second-job-fields.md` its four hidden maps.
> Nothing in any of them is retracted here.

---

## 0. Answer up front

| question | answer |
|---|---|
| Does this client have a third job advancement? | **It has half of one.** The ten jobs, their skill books and their four instructors are all here and complete. The *test* is not — no quest, no field, no mobs, no items. §1, §2 |
| Where does it happen? | **Chief's Residence, map 20001001, in El Nath.** All four instructors stand in one room, each placed exactly once in the whole archive. **[L]** §2 |
| Who are they? | **1104 Tylus** (Warrior), **1105 Robeira** (Magician), **1106 Rene** (Bowman), **1107 Arec** (Thief) — identified by their *own* idle lines, not by memory. **[L]** §2.1 |
| Is there a third-job quest chain? | **No.** All 322 quests enumerated; the only advancement quests in the client are the sixteen `20xxx` second-job ones. Nothing in `21xxx`. **[L]** §3.1 |
| Is there a hidden test field like the second job's four? | **No.** Every one-portal map in the archive was enumerated — there are twelve, and the only four with a job NPC in them are the second-job dungeons. **[L]** §3.2 |
| Are there test mobs or test items? | **No** to both, and both are enumerations rather than searches. §3.3, §3.4 |
| **What actually blocks it today?** | **El Nath is unreachable.** Victoria Island is a 223-map portal component; Orbis + El Nath is a separate 87-map component. The only link is a ship, and a ship is not a portal. **[L]** §4 |
| Can the server serve El Nath if it puts someone there? | **Yes — all 92 maps have field images and footholds.** §4.2 |
| What level? | **Nothing in this client says.** There is no quest to carry a `Check.0.lvmin`. 70 is **[I]** and would be ours. §5.1 |
| Does the SP packet change shape? | **No, for all ten.** Read off `uses_extended_sp`, not quoted. §5.2 |
| What would we have to write? | The whole test, and the travel. §6 |
| **What was built** | The advancement itself, and a three-stop ferry. §7 |
| How does a player get there now? | Cab to **Sleepywood**, then **Eurek the Alchemist** - the one NPC this client places on both continents. §7.3 |

---

## 1. The ten third jobs

`String.wz/Skill.img/<book>/bookName`, **[L]**:

| job | bookName | job name | source | grantable skills |
|---:|---|---|---|---:|
| 111 | `Crusader's Guide` | **Crusader** | **[D]** leading word | 7 |
| 121 | `White Knight's Code` | **White Knight** | **[D]** | 7 |
| 131 | `Dragon Knight's Path` | **Dragon Knight** | **[D]** | 7 |
| 211 | `Adv. Fire & Poison` | *not named in this client* | **[I]** | 7 (of 8) |
| 221 | `Adv. Ice & Lightning` | *not named in this client* | **[I]** | 7 |
| 231 | `Adv. Holy Magic` | *not named in this client* | **[I]** | 7 |
| 311 | `Path of the Ranger` | **Ranger** | **[D]** | 7 (of 8) |
| 321 | `Sniper's Scope` | **Sniper** | **[D]** | 7 (of 8) |
| 411 | `The Way of Hermit` | **Hermit** | **[D]** | 7 |
| 421 | `Chief Bandit's Tricks` | **Chief Bandit** | **[D]** | 7 |

**Seven of ten named, three not — the same split as second job**, and the three that are
missing are the same three Magician jobs. That is a property of this client's `String.wz`,
not a coincidence of how hard anybody looked: the same enumeration produced both.
`secondjob::NameSource` already carries that split in the type.

### 1.1 Three books have an eighth skill that must never be granted

Books **211**, **311** and **321** carry eight skills; the other seven books carry seven. The
extra one in each case is `2111006`, `3111006`, `3211006` — and all three are
`invisible = 1`. **[L]**

They are already in `secondjob::HIDDEN_SKILLS`, which holds all thirteen of the archive's
invisible skills and was written for the second job. **Nothing new is needed here**, which is
worth stating because it is the one part of the third advancement that is already done.

> **The instrument bit me here and the control caught it.** My first pass at the invisible
> column returned **zero** rows. `secondjob.rs` reads column index 24 in Rust; `awk` is
> 1-indexed, so the same column is `$25`. A run that returns nothing looks exactly like "no
> third-job skill is invisible", which is a false and load-bearing negative. It was caught
> only because the same query was made to count *all* invisible skills, where 13 was already
> known. `CLAUDE.md`: prove a search can find a positive control before believing its silence.

---

## 2. The four instructors

| branch | from | instructor | template | placements | map |
|---|---:|---|---:|---:|---|
| Warrior | 110 / 120 / 130 | **Tylus** | 1104 | 1 | 20001001 Chief's Residence |
| Magician | 210 / 220 / 230 | **Robeira** | 1105 | 1 | 20001001 |
| Bowman | 310 / 320 | **Rene** | 1106 | 1 | 20001001 |
| Thief | 410 / 420 | **Arec** | 1107 | 1 | 20001001 |

**[L]** — placements from `Map.wz`'s `life` nodes, names from `String.wz/Npc.img`. All four
are in one 29-foothold room with three portals, one of which is the exit to El Nath town.

### 2.1 They are identified by their own words, not by a remembered roster

This matters, because the roster in my head was wrong twice (§8). Each instructor names their
own branch in their own idle line, **[L]** verbatim:

```
1104 Tylus     idle0  "Do you want to be a more powerful warrior than you ever were?"
1105 Robeira   idle0  "You'll need to see me in order to become the best magician in the world."
1105 idle1            "Can anyone pass my test...?"
1106 Rene      idle0  "The path of the bowman is long and treacherous."
1107 Arec      idle0  "You want to become a powerful thief? Then you've come to the right person."
```

and all four carry a `d0` about the advancement itself:

```
1104  "Only the strongest of the strong deserves to move up the ladder and make the job advancement."
1105  "Only those that seek to further probe into the truth deserve to make the job advancement."
1106  "Only those with the eyes that look through the truth deserve to make the job advancement."
1107  "Only the ones that seek to seek further into darkness deserve to make the job advancement."
```

**So a click on Tylus today already says something apt.** `Session::npc_line` sends `d0`, and
`d0` is that sentence. The third advancement is the first one in this project where the
client's own placeholder is *not* embarrassing — which also means "it looks like it works"
is a failure mode to watch for.

### 2.2 A sweep for the whole subject, so the absence below is a measurement

Every string in `String.wz/Npc.img` mentioning a job advancement or a test — 1322 rows,
266 templates — resolves to exactly two groups:

* the four **second**-job examiners (227 / 319 / 424 / 514) and their four in-dungeon twins
  (800003 / 800004 / 800005 / 800006), all saying *"Only those who pass my test can undergo
  the 2nd job advancement as a &lt;branch&gt;"*;
* the four **third**-job instructors above.

**There is no third-job examiner and no third-job dungeon NPC.** **[L]**

### 2.3 Two NPCs that look like hooks and carry no words

| template | name | map | strings |
|---:|---|---:|---|
| 1123 | **Holy Stone** | 20001044 Holy Ground at the Snowfield | **`name` only** — no `d0`, no idle |
| 1125 | **Piece of Statue** | 20001073 Cave Within the Cave | **`name` only** |

Both are placed exactly once. A `name`-only NPC is this client's signature for *"the script
was server-side"* — the same shape as Phil, Mr. Kim and the taxis, all of which turned out to
be real features with their bodies missing. These two are the most likely places a third-job
ritual lived, and **that is a hypothesis, not a finding** — nothing in the archive attaches
either of them to an advancement.

---

## 3. What is not there, enumerated rather than searched

### 3.1 No quest chain

The whole quest id space of this client, bucketed:

```text
  0..9 999          10 000..19 999      20 000..29 999      80 000..89 999    500 000+
  the tutorial      Victoria Island     the SIXTEEN         the party quest   cash / event
                                        second-job quests
```

322 quests, and the `20000..20303` block is the only advancement content in any of them.
**There is no `21xxx`.** **[L]** Positive control: the same query prints all sixteen second-job
ids.

### 3.2 No hidden test field

Every map in the archive with **exactly one portal** — the signature of the second-job
dungeons — was enumerated. There are twelve:

```text
  80001000  Ant Tunnel For Bowman        npcs=1  mobs=30   <- second job
  80001100  Magician's Tree Dungeon      npcs=1  mobs=26   <- second job
  80001200  Thief's Construction Site    npcs=1  mobs=26   <- second job
  80001300  Warrior's Rocky Mountain     npcs=1  mobs=30   <- second job
  80003000  (no String.wz name)          npcs=0  mobs=6
  80003200  (no String.wz name)          npcs=0  mobs=1
  80003300  (no String.wz name)          npcs=0  mobs=9
  80003400  (no String.wz name)          npcs=0  mobs=1
  80003500  (no String.wz name)          npcs=1  mobs=7    <- Zelya, Zombie Mushmom
  900000000 White Map                    npcs=0  mobs=0    <- GM test maps
  900000001 White Map with Mob           npcs=3  mobs=9
  900000002 Black Map with Mob           npcs=3  mobs=9
```

**[L]** The four second-job fields are the only ones of that shape with a job NPC in them.

**The Cave of Trial is not an instance.** 20001071 / 20001072 / 20001074 have **6, 10 and 6
portals** and are walkable from *The Passage*, which is walkable from *Dead Mine IV*. It is an
ordinary high-level hunting area — Jr. Cerebes 43, Firebomb 51, Cerebes **72**, Bain **90** —
which makes it a plausible *place* for a level-70 test but not a prepared one.

### 3.3 No dedicated test mobs

Every mob template above 100 000 in the whole archive:

```text
  7000xx   Mushmom, Jr. Balrog, Zombie Mushmom, Rotten Mushmom, Mano          (bosses)
  8000xx   Ligator, Jr. Necki, Curse Eye, King Slime, Mano                    (event/PQ)
  8000 10..17  the eight SECOND-job test mobs
  999000x  Test(Basic), Test(Fire), Test(Poison), Test(Ice), …                (GM dummies)
```

**[L]** There is no third-job equivalent of the `800010..800017` set.

### 3.4 No test items

No *Black Charm*, no third-job letter, no third-job proof. **[L]** Positive control: the same
grep style finds `4031017 Dark Marble` and `4031018 The Proof of a Hero` immediately.

`4003024 Dark Crystal` and `4003019 Dark Crystal Ore` exist and are **not** relevant — `4003xxx`
is this client's ore range, and Dark Crystal is its top-tier mineral. Naming it here so nobody
finds it later and mistakes it for the token.

---

## 4. The blocker: El Nath cannot be reached

### 4.1 Two components, and a ship between them

A breadth-first walk of `gm-handbook/portals.txt`, portals treated as two-way:

```text
  component containing 10000000 Lith Harbor   223 maps    <- everything the game currently is
  component containing 20000000 Orbis          87 maps    <- Orbis, El Nath, the Cave of Trial
```

**[L]** They do not touch. The link in the real game is the Ellinia ↔ Orbis ship, and a ship
is not a portal — it is a timed boarding a server runs. The terminals are all here:

| map | name | NPCs |
|---:|---|---|
| 10002090 | Ellinia Station | **322 Joel**, **323 Cherry** |
| 10002091 | Before Takeoff &lt;To Orbis&gt; | **324 Purin** |
| 20000010 | Orbis Ticketing Booth | **1000 Agatha**, **1001 Platform Usher** |
| 20000012/13 | Station / Before Takeoff &lt;To Ellinia&gt; | 1004 Rini, 1006 Erin |

and they say what they are for — Joel: *"You need to purchase the ticket to get on board the
ship that heads to Orbis Station."* **[L]**

> **A correction to my own first pass, stated plainly.** I first walked the portal graph
> **one-way** and reported that the Holy Ground and the Cave of Trial were unreachable even
> from El Nath. That was wrong: a portal pair is listed from both sides and is walked both
> ways, and read that way both are in El Nath's own component. The two-component result below
> survives either reading; the intra-El-Nath one did not. The error was in the instrument, and
> it is recorded rather than quietly fixed because a directed reading of a portal table is a
> mistake somebody will make again.

### 4.1a The obvious ports are unreachable, and that is a second finding

The client ships a full ferry cast - Joel and Cherry at **Ellinia Station**, Purin in the
boarding room, Agatha and the Platform Usher at the **Orbis Ticketing Booth**, Rini and Erin
on the return leg. **[L]** Four of those seven stand in rooms nothing can walk into.

```text
  10002090  Ellinia Station    portals targeting it, in the WHOLE archive:  0
  10002091  Before Takeoff     ditto                                        0
  20000010  Orbis Ticketing Booth                                           2   <- reachable
```

**[L]** And Ellinia's own `in03` - the station door in the retail game - is
`10002000, 38, in03, 0,` : **a portal with no target map.** So the Ellinia end of the ferry was
server-driven in the original and is a dead end here, while the Orbis end is an ordinary room
off the town (`20000000 top00 -> 20000010`).

That asymmetry decided the ferry's shape in §7.2, and it is asserted by a test rather than
remembered: if a later dump ever gives Ellinia Station a portal, `taxi::tests::
every_row_stands_where_the_table_says` fails and invites Joel back in.

> **A correction to my own instrument, again.** §4.1's walk treats portals as two-way, which
> is right for a matched `in00`/`out00` pair and **wrong for a portal whose target is 0**.
> Reversing Ellinia Station's `out00` invented an edge into it and made the station look one
> hop from Ellinia. The two-component headline survives both readings - a directed walk from
> Lith Harbor reaches 220 maps and neither Orbis nor El Nath - but *"Ellinia Station is
> reachable"* was an artefact of the reversal. **Neither reading alone is correct**: a portal
> row is a directed edge, and the two-way assumption is only safe where the matching row
> exists. That is the third time on this feature that an instrument gave a clean, confident
> answer to a question it could not actually see.

### 4.2 But the server could serve it today

All **92** Orbis/El Nath maps have a field image in `Map.wz`, and the job-relevant ones have
real geometry:

```text
  20000000  Orbis                          388 footholds   9 npcs
  20001000  El Nath                        275 footholds   4 npcs
  20001001  Chief's Residence               29 footholds   4 npcs   <- the four instructors
  20001044  Holy Ground at the Snowfield    72 footholds   1 npc    <- the Holy Stone
  20001071  The Cave of Trial I            694 footholds   30 mobs
```

**[L]** So `!map 20001001` reaches Tylus today, and clicking them already draws their `d0`.
Nothing about the *maps* needs work; the travel is a gameplay decision, not a data gap.

---

## 5. What is already done, on our side

### 5.1 The prerequisites would be ours

`Check.0.lvmin` and `Check.0.job.0` gave the second advancement its level 30 and its branch
gate **[L]**. **There is no quest here to read either from.** Level 70 is the classic value
and is **[I]** — a second constant of the same kind as `jobs::STAT_MINIMUM`, and nothing on
this machine will ever catch it being wrong. It should be named and put in one place, exactly
as `secondjob::LEVEL_MINIMUM` is.

The *branch* gate is different: it is **[D]** and forced. A Crusader can only come from a
Fighter because `111`'s book sits under `110`'s in `Skill.wz`, and the ten third jobs map onto
the ten second jobs one-to-one.

### 5.2 The packet needs no new work

* **SP pool key is 3.** `net::stats::tier_for_job` is `2 + (job % 10)` for a non-round job, and
  every third job ends in `1`. `secondjob`'s own test already pins `tier_for_job(111) == 3` as
  a *control*. **[D]**
* **The SP encoding does not fork.** Read off `net::opcode::uses_extended_sp` rather than
  quoted: branch `1|2` accepts `0 | 10..=12 | 20..=22 | 30..=32` and branch `3..=5` accepts
  `0 | 10..=12 | 20..=22`. Every `x10 → x11`, `x20 → x21`, `x30 → x31` pair is on the same
  side. **[D]**
* **`skillpoints::Tier` has only `First` and `Second`.** A `Third` arm and a `THIRD_JOB_LEVEL`
  are the one structural change needed, and `Grant::sp_total_owed` is a *total*, so it stays
  idempotent.
* **The thirteen invisible skills are already handled** — §1.1.

---

## 6. What building it would actually mean

The second advancement was **decoded**: the client shipped the chain, the items, the counts,
the maps and the mobs, and the work was joining them up. The third advancement would be
**authored**: the client ships the people, the place and the rewards, and *nothing* of the
test. That is a different kind of task and it should be called by its right name.

Forced by the data, no decision needed:

* four instructors → ten jobs, one-to-one from the second job;
* level 70 **[I]**, one named constant;
* `0x007C` with tier 3, no encoding fork;
* seven grantable skills per book, three invisible ones already filtered.

Genuinely open, and the owner's to decide:

* **How a player reaches El Nath.** Nothing walks there. Options range from running the real
  Ellinia↔Orbis ferry (two terminals, five NPCs, a schedule) to adding El Nath to the existing
  taxi table, to having the second-job instructor send you.
* **What the test is.** The client offers no marbles, no field and no mobs, but it does offer
  two silent NPCs (§2.3) and a level-72–90 hunting area named *The Cave of Trial*.

---

## 7. What was built, 2026-08-31

The owner chose between the options this file leaves open: **no test - level 70 and click**, and
**a ferry reusing the taxi machinery**. Both are built and wired.

### 7.1 The advancement

`crates/world/src/thirdjob.rs`, wired into `session/npc.rs`'s click chain after the
second-job branches. Clicking Tylus, Robeira, Rene or Arec:

| character | what happens |
|---|---|
| second job of that instructor's branch, level >= 70 | **advances**, `0x007C` with the job and the tier-3 SP pool, then a sentence |
| same but below 70 | a sentence naming 70. Nothing else |
| a second job of another branch | a sentence naming the branch, **not** the level - "come back at 70" would be advice that never becomes true |
| a beginner or a first job | a sentence saying take the second step first |
| already third job | a sentence. One-way, like the other two advancements |

**No box in between.** There is exactly one destination per second job, so a menu would be a
list of length one. That is a property of the client's data, not a shortcut - `111` is the
book under `110`, ten times over.

**`skillpoints::Tier` gained a `Third`**, and with it a change worth naming: **the second tier
now stops accruing at 70**, exactly as the first stops at 30, because a tier that has a
successor should hand over to it. Nobody loses a point they already had - `top_up` saturates
and never claws back - but a level-71 second-job character who does not advance stops earning
second-job points, which is the same incentive the first tier has always had.

### 7.2 The Ossyria line

Three stops, in `crate::taxi` rather than a second module of its own - the menu, the fare, the
map check and the warp already existed and a copy of them is how two features drift.

```text
  10005000  Sleepywood              Eurek the Alchemist   605
  20000010  Orbis Ticketing Booth   Platform Usher       1001
  20001000  El Nath                 Eurek the Alchemist   605
```

**Eurek is the line.** They are the only NPC this client places on **both continents** - §4.1a
rules out the ferrymen who cannot be reached - their one and only `d0` is *"I'm Eurek the
Alchemist, and I wander all over the world of MapleStory"*, and they carry **zero** quest rows,
so giving them a menu swallows nothing. **[L]** on all three counts. That they are an alchemist
rather than a ticket clerk is the **[I]** in the design, and the alternative was an NPC nobody
can walk to on one side and no NPC at all on the other.

`Taxi` gained a `network` and a per-row `fare`, and `destinations` filters by network.
**Without that filter the three new rows would have put Orbis and El Nath on every Victoria
cab's menu at 500 mesos** - because `destinations` is derived from the table rather than typed
out, which is the design that stops a hand-written route matrix from losing a row. The
filter is asserted in both directions: no cab names an Ossyria stop, and the ferry sells no
hop between two Victoria towns.

**Sleepywood is the interchange**, and it is the one map with a row in each network: cab there
from any town for 500, cross for 1000. `taxi_for` now takes the map as well as the template,
which is what lets one NPC be a port twice and an ordinary NPC nowhere else.

### 7.3 The route, end to end

```text
  any Victoria town  --cab, 500--> Sleepywood
  Sleepywood         --Eurek, 1000--> El Nath
  El Nath            --portal in01--> Chief's Residence   <- Tylus, Robeira, Rene, Arec
  and back:          El Nath --Eurek, 1000--> Sleepywood
```

Four clicks each way, and no seventeen-floor tower unless the player wants it. The Orbis Tower
is still there and still walkable - it is the client's own route and 18 hops.

### 7.4 What is still missing

* **Nothing has been on a screen.** No character has stood in El Nath, and none of the 87
  Ossyria maps has ever been loaded by this server.
* **Skill points are still not persisted when spent.** A third pool triples the surface of
  that. A player who spends and sees them return will file it as a bug.
* **`skilltable::book()` will offer the three invisible third-job skills** to a `!learn`.
  `secondjob::HIDDEN_SKILLS` holds them and `ThirdJob::grantable_skills` filters, but `!learn`
  does not go through it.
* **No third-job skill has a cast handler**, exactly as no second-job one does.
* **El Nath's own content is untouched** - the Holy Stone and the Piece of Statue still have a
  name and no words, and nothing warps into the Cave of Trial.

---

## 8. Two traps for whoever picks this up

**"Chief Stan" is not the El Nath chief in this client.** They are **NPC 202, in Henesys**, a
father in a gold-watch quest: *"My son...! That ungrateful kid."* **[L]** The map called
*Chief's Residence* contains Tylus, Robeira, Rene and Arec and no chief at all. Anybody
working from memory of the retail game will go looking for the wrong NPC on the right map.

**The Bowman instructor is Rene here.** I expected Helena, and the only reason the table above
is right is that §2.1 read the NPC's own line instead. There is no Helena in this client; the
nearest name is **Hella**, mentioned in Jade's dialogue as a friend who disappeared, with no
NPC behind it.

The general form is the one `CLAUDE.md` already makes about the v214 reference tree: **this
build is a later MapleStory wearing a classic skin, and a remembered roster scores badly
against it.** Read the strings.

---

## 9. How to re-derive every number here

With **the repo as the working directory**:

```
cd C:\MapleCW
python tools\dump_portals.py       # npcs.txt, mobs.txt, portals.txt, fields.txt
python tools\dump_names.py         # maps.txt, items.txt
python tools\dump_mobs.py          # mobtemplates.txt
python tools\dump_quests.py        # questlines.txt
python tools\dump_npcstrings.py    # npcstrings.txt
python tools\dump_skills.py        # skills.txt   (invisible is column index 24, awk $25)
target\release\wz-dump.exe cat client-patched\Data\String\String_000.wz Skill.img
```

The component walk in §4.1 is a breadth-first search over `portals.txt` with `(src, target)`
edges added **in both directions**; adding them one way gives a different and wrong answer for
maps inside El Nath, which is the error recorded there.
