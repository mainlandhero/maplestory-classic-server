# The four hidden job-test fields, and how job advancement works in this client

Written 2026-08-31. **No Ghidra pass and no client run went into this.** Every number below
is out of the client's own WZ, via the generated `gm-handbook/*.txt`, plus this repo's source.

Labels are the project's: **[L]** read off this client's WZ dumps, its listing or a capture;
**[D]** derived from two or more [L] facts; **[I]** inferred — a policy nothing on this
machine can confirm.

**Nothing here authenticates.** As everywhere in this project, the channel socket carries no
credentials. A second job advancement — and a warp into one of these fields — is granted on
the say-so of whoever holds the socket.

> **Companions, not replacements.** `research/job-advancement.md` owns the *packet* (`0x007C`
> mask bit 5) and the enumeration proving no quest in this client can change a job.
> `research/job-instructors.md` owns the *first* advancement's people and words.
> `research/second-job.md` owns the *decision* — the ten jobs, the SP tier, the thirteen
> invisible skills. Nothing in any of the three is retracted here. **This file owns the four
> maps**, which is the one piece all three said was missing.

---

## 0. Answer up front

| question | answer |
|---|---|
| How does a job advancement work in this client? | **It does not — no quest in this client can change a job.** All 322 quests were enumerated and there is no `Act.<n>.job` key in the whole tree. It is an NPC script, and NPC scripts were server-side. Both advancements are the server's to perform. **[L]**, `research/job-advancement.md` |
| Where are the second-job "quest maps"? | **80001300, 80001100, 80001000, 80001200** — Warrior, Magician, Bowman, Thief. §2 |
| Is that pairing a guess? | **No. Five nodes across two archives agree, and they were read separately.** §2.1 |
| How do you get in? | **Nothing walks in.** Each map has **exactly one portal** and it is the spawn point — against 31 for Perion. A server putting you there is the only way anybody has ever been in one. **[L]** §3 |
| How do you get out? | The **warden**, the one NPC standing inside. `returnMap` *and* `forcedReturn` both name the examiner's map. **[L]** §3.2 |
| What is in them? | Two dedicated mob templates each, from the `8000xx` range, **all eight at level 30 with 74 exp** — the level of the advancement itself. **[L]** §4 |
| So the marbles are ours to author? | **The item ids are the client's; the drop is ours.** `Check.1.item.0` of quest 20002 is `30 x 4031017` **[L]**. Nothing in this client carries a drop rate for anything. §5 |
| Was `research/second-job.md` wrong? | **Half.** *"The hidden field and its `q20002s` script do not exist"* — the **script** genuinely does not exist and that still holds. The **field** was in the client the whole time. §6 |
| Is it 20 marbles or 30? | **30**, on all four branches, and the client says so twice. §5.1 |

---

## 1. How the advancement actually works, end to end

Four quests per branch. All sixteen carry `Check.0.lvmin = 30` and a single `Check.0.job.0`
of `100`/`200`/`300`/`400`. **[L]**

```text
  20000  The Warrior's Next Journey   start 511  end 511   ->  20001    selfStart
  20001  Finding the Instructor       start 511  end 514   ->  20002    Act.0 gives the LETTER
  20002  Test of Qualification        start 514  end 514   ->  20003    wants 30 x MARBLE
                                      Check.0.startscript  q20002s
                                      Check.1.endscript    q20002e
                                      Check.1.failscript    q20002f
  20003  Proof of Qualification       start 514  end 511   ->  --       Act.0 gives the PROOF
```

**[L]**, `gm-handbook/questlines.txt`. Every quest pays `Act.1.exp = 3150`, so the chain is
**12 600 experience**.

| branch | quests | letter | marble (x30) | proof |
|---|---|---|---|---|
| Warrior | 20000..20003 | 4031013 | 4031017 | 4031018 |
| Magician | 20100..20103 | 4031014 | 4031019 | 4031020 |
| Bowman | 20200..20203 | 4031015 | 4031021 | 4031022 |
| Thief | 20300..20303 | 4031016 | 4031023 | 4031024 |

Three NPCs per branch, and **they are three different templates with two names between
them**:

| role | who | templates | what they do |
|---|---|---|---|
| **instructor** | Dances with Balrog / Grendel / Athena Pierce / Dark Lord | 511 / 313 / 221 / 411 | hands out the letter, and **performs the advancement** |
| **examiner** | `<Job> Job Instructor` | 514 / 319 / 227 / 424 | runs the test, **advances nobody** |
| **warden** | `<Job> Job Instructor` — *the same name, a different id* | 800003 / 800004 / 800005 / 800006 | stands inside the hidden field; the only door |

The examiner's own `idle0` is the client saying this out loud: *"Do you want to undergo your
2nd job advancement as a warrior? Then go see Dances with Balrog in Perion."* **[L]**

> **The name collision is a trap and it is the client's, not ours.** Eight NPCs share four
> `name` strings. `crate::jobs::first_job_at`, `secondjob::branch_at` and
> `secondjob::branch_examined_by` all refuse the wardens, and `branch_warded_by` refuses the
> examiners. A lookup by name would route a click to the wrong half of the chain and the
> failure would be a warp, not a compile error.

---

## 2. The four maps

| branch | field | name | mobs | warden | ejects to |
|---|---:|---|---|---:|---:|
| Warrior | **80001300** | Warrior's Rocky Mountain | 800016 Fire Boar x15, 800017 Lupin x15 | 800006 | 10004023 |
| Magician | **80001100** | Magician's Tree Dungeon | 800012 Curse Eye x13, 800013 Horny Mushroom x13 | 800004 | 10002070 |
| Bowman | **80001000** | Ant Tunnel For Bowman | 800010 Evil Eye x15, 800011 Zombie Mushroom x15 | 800003 | 10001090 |
| Thief | **80001200** | Thief's Construction Site | 800014 Cold Eye x13, 800015 Blue Mushroom x13 | 800005 | 10003080 |

All **[L]**. Everything in `crates/world/src/secondjob.rs`'s `TestField`, and
`secondjob::tests::the_four_test_fields_are_exactly_what_this_client_ships` asserts the whole
table against the generated dumps rather than trusting this page.

### 2.1 The pairing is corroborated five ways, and they are not the same instrument

`CLAUDE.md`: *two scans agreeing is not corroboration when they share a blind spot.* So each
row below is a different **node** of the archive, and two of them are a different **archive**:

| # | node | archive | what it says about the Warrior's field |
|---|---|---|---|
| 1 | `Map.wz/<map>/info` → `returnMap`, `forcedReturn` | Map.wz | both are **10004023**, the Warrior examiner's map |
| 2 | `Map.wz/<map>/life` type `n` | Map.wz | the one NPC in it is **800006, "Warrior Job Instructor"** |
| 3 | `Map.wz/<map>/life` type `m` | Map.wz | Fire Boar and Lupin — Perion's own mobs, in a Perion-themed map |
| 4 | `String.wz/Map.img` | **String.wz** | it is called **"Warrior's Rocky Mountain"**; the examiner stands on **"West Rocky Mountain IV"** |
| 5 | `Quest.wz` chain | **Quest.wz** | `20002`'s `Check.0.npc` is **514**, whose only placement is 10004023 |

Four for four, in every branch. **[D]** from five [L] facts, and the derivation has no
freedom in it: there is exactly one way to match four dungeons to four branches that satisfies
all five.

### 2.2 What is NOT in them

`80001000`..`80001300` are the only four maps in the whole archive with this shape. The
neighbouring `800xxxxx` ids are the Maple Island party quest (`80000000`..`80000600`) and the
Free Market (`80002000`..`80002011`), and neither has a job NPC in it. **[L]**, and it is an
enumeration of the `80*` name space rather than a search for what was expected — `CLAUDE.md`'s
*enumerate before you filter*.

---

## 3. They cannot be walked into, and that is the whole design

### 3.1 One portal each

```text
  map        portals   what the one portal is
  80001000        1    sp   (spawn point, no target)
  80001100        1    sp
  80001200        1    sp
  80001300        1    sp
  ------------------------------------------------  controls
  10004000       31    Perion, an ordinary town
  10004023        8    the examiner's own map
```

**[L]**, `gm-handbook/portals.txt`. A spawn point has no target map; it is where the server
puts you, not somewhere you can walk. **There is no door in and no door out.**

That is not an oddity to work around — it is the client telling us the entry is server-side,
which is exactly what `Check.0.startscript = q20002s` says in a different language.

### 3.2 The exit is the client's own answer

`Map.wz/<map>/info` carries `returnMap` and `forcedReturn`:

```text
  map        returnMap  forcedReturn   reviveMap
  80001000   10001090     10001090     10001000
  80001100   10002070     10002070     10002000
  80001200   10003080     10003080     10003000
  80001300   10004023     10004023     10004000
  ------------------------------------------------------  control
  10004000   10004000    999999999     10004000
```

**[L]** An ordinary map's `forcedReturn` is `999999999` — *none*. These four name a real map,
and it is their own examiner's. **This is the archive saying out loud that these are places
you get ejected from**, and it is the single strongest piece of evidence in this file, because
nothing about a map's `info` node knows anything about quests or NPC names.

`secondjob::TestField::exit_map_id` is that number, and
`each_test_field_is_ejected_into_its_own_examiner_s_map` asserts it against the dump — with
Perion as the control, so "these four have a forced return" is a measurement rather than a
coincidence of parsing.

> **`reviveMap` is a different column and it is a trap.** For the Bowman's field the two read
> `returnMap 10001090` and `reviveMap 10001000` — *The Road to the Dungeon*, where the examiner
> actually stands, versus Henesys, which is a town with nobody in it. `exit_map_id` is the
> `returnMap`, on all four. Reading the wrong column would send a player somewhere plausible
> and useless, which is the kind of wrong that does not look wrong on screen.

### 3.3 The consequence for the server, stated plainly

A character standing in one of these with no warden click available **is stuck**. Not
inconvenienced — stuck: no portal, and the only two other exits are a return scroll (which
reads `returnmaps.txt`, so it works) and dying. `secondjob::warden_step` therefore has exactly
one arm and **cannot refuse**. A guard there would be a bug.

---

## 4. The eight mobs are purpose-built, and the uniformity is the tell

```text
  template  name             level  maxHP  exp   ordinary twin  its level
  800010    Evil Eye            30    789   74      23             27
  800011    Zombie Mushroom     30    718   74      21             24
  800012    Curse Eye           30    789   74      31             33
  800013    Horny Mushroom      30    718   74      19             22
  800014    Cold Eye            30    789   74      37             40
  800015    Blue Mushroom       30    789   74      16             19
  800016    Fire Boar           30    718   74      30             32
  800017    Lupin               30    718   74      35             37
```

**[L]**, `gm-handbook/mobtemplates.txt`. **Every one of the eight is level 30 with 74 exp**,
while their ordinary twins run from 19 to 40. Eight clones flattened onto one level, and that
level is `Check.0.lvmin`. **[D]** — these were built for this test rather than borrowed for
it, and it means a level-30 character faces the same fight in every branch.

None is a boss; `800011` is flagged undead. Spawn counts are 30 / 26 / 26 / 30 and every
spawn point has `mobTime = 0`, the field's ordinary respawn rate — the same as the examiner's
own outdoor map. **[L]**

### 4.1 Three of the eight leak, and it is the reason the drop is gated on the map

Two of the eight are **not** exclusive to their dungeon:

```text
  800010  Evil Eye         also on 80003500   (an unnamed boss field: Zombie Mushmom,
  800015  Blue Mushroom    also on 80003500    Rotten Mushmom, NPC "Zelya")
  800011  Zombie Mushroom  also on 10006160   *** Precipice of Darkness - an ORDINARY field
                                                  with Duskmanders and Zombie Lupins ***
```

**[L]** So a rule of the form *"template 800011 drops the Bowman's marble"* hands the test's
own proof to any player grinding Precipice of Darkness. `secondjob::marble_for_kill` therefore
takes **both** the mob and the map, and `session/combat.rs` **filters every marble out of the
roll first** and then puts the right one back — so a drop-table row cannot smuggle one out
either. `a_dark_marble_drops_in_the_test_field_and_nowhere_else` names all three leaks
individually rather than testing them as a class.

---

## 5. The marbles

### 5.1 Thirty, and the client says so twice

```text
  20002  Check      1.item.0.id      4031017
  20002  Check      1.item.0.count   30
  20002  QuestInfo  demandSummary    #i4031017:# #t4031017:# #c4031017# / 30#k
```

**[L]**, and `gm-handbook/questreq.txt` carries the same `30` from a second pass over the same
archive. All four branches. **It is 30, not 20.**

### 5.2 One naming discrepancy, left alone

`QuestInfo.1` reads *"enter a hidden area, defeat the monsters there, and collect **Black
Marbles**"*. `String.wz/Item.img/4031017/name` is **"Dark Marble"**. Both are the client's,
they disagree, and the bag is what a player actually reads — so this server's own lines say
Dark Marble and the journal keeps saying Black. Recorded rather than tidied: renaming a
shipped item to hide Nexon's inconsistency would be editing evidence.

### 5.3 The drop rate is **[I]**, and it is ours

Nothing in this client carries a drop rate for anything — `data/drops.txt` says so in its own
header (*"THE CHANCES ARE OURS"*). What that file had for these eight was a **6%** row scraped
from a fan site, on **six of the eight**: `800014` (the Thief's Cold Eye) and `800017` (the
Warrior's Lupin) had **no marble row at all**, so half of two branches' fields dropped nothing
toward a test they are half of.

`secondjob::MARBLE_DROP_IS_CERTAIN` is `true`. Two reasons, both stated as policy rather than
discovered:

* 6% is 30 marbles in roughly 500 kills. The fields hold 26–30 mobs, so at 100% the test is a
  full clear and a bit, which is a test rather than a grind.
* A feature whose first observation costs hours is a feature this project cannot check, and
  `CLAUDE.md` calls that the one thing a feature here may not be.

One constant, one place to change it if the owner wants it rarer.

---

## 6. What `research/second-job.md` got right, and the half it got wrong

Its §9.5 reads:

> *"The `Test of Qualification` hidden field and its `q20002s` script do not exist, so the
> client's own route to the advancement is not walkable."*

**The script half is right and still holds.** `research/quest-scripts.md` enumerated all 205
archives under `client-patched/` and decoded all 10 021 images; `q20002s` occurs exactly twice
in the whole tree, as the two *names* inside `20002.img`. `Data/Etc/Script/Script.wz` is a
63-byte header with zero entries. There is no body for it anywhere. **[L]**

**The field half is wrong.** The four maps are in `Map.wz`, they have field images, footholds,
mobs, an NPC and an `info` node, and they have been generated into `gm-handbook/` this whole
time. The sentence conflated *"the script that puts you there does not exist"* with *"the
place does not exist"*, and the second does not follow from the first.

It is worth naming the shape rather than just correcting the fact, because it is one this
project has paid for before: **an absence established for one thing was carried over to a
neighbouring thing without being re-asked.** The enumeration behind the script half was
careful, exhaustive and correctly reported — and it was an enumeration of `Script.wz` and the
image tree, which cannot say anything about `Map.wz` at all. `CLAUDE.md`'s *"'not found' is
not 'not there'"* is usually about a search that was too narrow; this is the version where the
search was exactly right and its *conclusion* was widened one noun too far.

The cost was small and the check was free: one `grep` of `gm-handbook/maps.txt` for `80*`.

---

## 7. What this server now does with all of it

Wired, not merely built — `CLAUDE.md`'s *"built is not wired"*, which `secondjob.rs` sat on
the wrong side of from 2026-08-28 until now.

| trigger | handler | effect |
|---|---|---|
| click instructor, beginner | `jobs::advancement_for` | first advancement, unchanged |
| click instructor, first job at 30, **holding the proof** | `second_advancement_for` | type-6 menu of 2–3 second jobs |
| menu answer | `grant_second_job` | `0x007C` job + SP, proof consumed, decision **re-run** |
| click instructor, first job at 30, no proof | same | refusal naming the examiner and the item |
| click examiner, eligible, < 30 marbles | `job_test_for` | notice, then `SetField` into the hidden field |
| click examiner, eligible, ≥ 30 marbles | same | 30 taken, proof given, **no warp** |
| click warden | `job_test_exit_for` | notice, then `SetField` to `exit_map_id` |
| quest 20002/20102/20202/20302 **starts** | `enter_test_field_on_quest_start` | the missing `startscript` |
| kill a test mob in its own field | `drops_from_kill` | one marble, deduped against the table |

Two ordering rules, both learned elsewhere and both applied here: **the notice goes before the
`SetField`**, because a script box sent with or just before one is torn down silently by field
entry; and `on_quest_request` **skips its closing `say_line` when the map changed**, compared
before and after rather than flagged, so it stays true for any future quest that moves
somebody.

### 7.1 Still open

* **Skill points are not persisted when spent.** `skillpoints` computes an entitlement from
  the level and nothing records what has been spent, so a second pool doubles the surface of
  that. A player who spends and sees them return will file it as a bug — say so when
  reporting.
* **`skilltable::book()` will offer the thirteen invisible skills** to a second-job character.
  `secondjob::HIDDEN_SKILLS` holds them and `grantable_skills()` filters, but `!learn` does
  not go through it. `research/second-job.md` §7 has the durable fix.
* **No second-job skill has a cast handler.** `firstjob.rs` classifies the 24 first-job skills
  and `session/skills.rs` acts on them; nothing equivalent exists for the 66.
* **`REQUIRE_QUEST_CHAIN` is still `false`.** The gate that is enforced is the *proof item*,
  which is the client's own `Check.1.item.0` on quest 20003 and is a fact about the bag rather
  than about quest rows. Deliberate, and §7's table says which.

---

## 8. How to re-derive every number here

With **the repo as the working directory** — `CLAUDE.md`: the scratchpad shadows the real
tools.

```
cd C:\MapleCW
python tools\dump_portals.py       # npcs.txt, mobs.txt, portals.txt, fields.txt
python tools\dump_names.py         # maps.txt, items.txt
python tools\dump_mobs.py          # mobtemplates.txt
python tools\dump_quests.py        # questlines.txt, questreq.txt, quests.json
python tools\dump_returnmaps.py    # returnmaps.txt
cargo test -p world --lib secondjob
```

The four file-backed tests degrade to a no-op on a clean checkout where `gm-handbook/` has not
been generated, which is exactly the shape of check `CLAUDE.md` warns can be silently vacuous.
Each therefore asserts a **positive control first** — the file must contain the whole archive,
and Perion must have many portals and no forced return — before any of its negatives are
believed. That degradation was itself checked: three facts were deliberately corrupted (a
wrong warden, a wrong exit map, a wrong mob) and four tests failed. A green run on this
machine means the files were read, not that the check was skipped.
