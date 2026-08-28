# The first-job instructors: who they are, what they say, and what they cannot say

**Written 2026-08-27. No Ghidra** (another agent holds the project lock), no client run, no
`cargo`. Instruments: the generated `gm-handbook/*.txt`, the repo's own source, and **three
archived sessions in `previous-runs/`** whose logs answer two questions that were standing
open. Every negative below was run against a positive control, and §9 names the two controls
that caught my own mistakes.

Labels: **[L]** read off this client's WZ dumps, the repo's source, or a capture; **[D]**
derived from two or more [L] facts; **[I]** inferred — a policy nothing on this machine can
confirm.

**Nothing authenticates.** The channel socket carries no credentials. An advancement is
granted on the say-so of whoever holds the socket, and none of what follows changes that.

> **Companion, not a replacement:** `research/job-advancement.md` (2026-08-21) owns the
> *packet* — `0x007C` mask bit 5 — and the enumeration that proves no quest can change a job.
> Nothing in it is retracted here. This file owns the **content and the conversation**, and it
> sharpens four of its claims with measurements that did not exist when it was written.

---

## 0. Answer up front

| question | answer |
|---|---|
| How many first-job branches does this build have? | **Four.** Warrior, Magician, Bowman, Thief. **There is no Pirate instructor, no Pirate map and no Pirate weapon class in this client** — three independent instruments, each with a control. §2 |
| Who are the four? | **511 Dances with Balrog, 313 Grendel the Really Old, 221 Athena Pierce, 411 Dark Lord.** **[L]** §1 |
| Do the job ids in `crates/world/src/jobs.rs` agree with the client? | **Yes, and now for a second independent reason.** Each instructor's own second-advancement chain gates on exactly the job id `jobs.rs` pairs them with — `20000`/511/`job 100`, `20100`/313/`200`, `20200`/221/`300`, `20300`/411/`400`. **[L]** §3 |
| Does this client ship the first-advancement conversation? | **No, and there is nothing for it to ship it *in*.** There is no first-advancement quest, so there is no `Say` tree; the four instructors between them have **21 strings total** and every one is an idle line. §4 |
| Then what does the client already do when you click one? | **It sends `0x00F2`, the no-quest click.** This was a prediction labelled [D] with a named blind spot in `job-advancement.md` §5.2. **It is now measured**: 2026-08-27, map 10003003, a jobless level-10 character clicked the Dark Lord **twice** and got `0x00F2` both times. **[L]** §4.3 |
| Is the 25-stat rule in the client anywhere? | **No.** The whole `Check` key space of all 322 quests is 33 shapes and none of them is `str`/`dex`/`int`/`luk`; the whole quest text carries no `STR`/`DEX`/`INT`/`LUK` token. **It is the owner's design decision, [I], and nothing on this machine will ever catch it being wrong.** §6 |
| Does 25 contradict anything in the tree? | **Yes — `world::jobs::STAT_MINIMUM` is `35`.** That is the owner's own earlier number from 2026-08-19. Their 2026-08-27 sentence supersedes it and it is a **one-line edit**, which is exactly what that constant exists for. §6.1 |
| Which opcodes carry the accept/decline? | `0x00F2` click → `0x055B` box → **`0x00F3`, trailing `i8`: `1` Yes, `0` No, `-1` closed**, on a message-type `0x10` or `3` box. All three already work in this repo. §7 |
| What stops it working today? | **One line.** `say_line`'s `branches` test is `... && convo.quest_id.is_some()`, and an advancement has no quest id, so an instructor conversation can only ever draw a plain `Say`. §7.2 |
| Anything expensive hiding in the routes? | **One.** The Bowman instructor is the only one of the four **not** reachable from their own town — Athena Pierce sits behind **Henesys Park, 10001050**, the map with the open crash question. §8 — which this file also closes. |

---

## 1. The four branches, as a table

Template ids and names from `String.wz/Npc.img` via `gm-handbook/npcstrings.txt`; spawn maps
from `Map.wz`'s `life` nodes via `gm-handbook/npcs.txt`; map names from `String.wz/Map.img`
via `gm-handbook/maps.txt`; routes from `gm-handbook/portals.txt`. All **[L]**.

| branch | instructor | template | stands on | map name | reached from | job id | class name | does the client ship the script? |
|---|---|---:|---:|---|---:|---:|---|---|
| Warrior | Dances with Balrog | **511** | **10004003** | Warriors' Sanctuary | 10004000 Perion, portal `in02` | **100** | Swordsman | **No** — 6 idle strings, no dialogue |
| Magician | Grendel the Really Old | **313** | **10002003** | Magic Library | 10002000 Ellinia, portal `jobin00` | **200** | Magician | **No** — 4 idle strings |
| Bowman | Athena Pierce | **221** | **10001051** | Bowman Instructional School | **10001050 Henesys Park**, portal `in02` | **300** | Archer | **No** — 4 idle strings |
| Thief | Dark Lord | **411** | **10003003** | Thieves' Hideout | 10003000 Kerning City, portal `in03` | **400** | Rogue | **No** — 4 idle strings |
| **Pirate** | — | — | — | — | — | (500 exists in the client's SP masks) | — | **No instructor exists in this build.** §2 |

**None of the four stands in their town**, which reproduces `job-advancement.md` §6 from the
same dumps. `!map 10004000` finds nobody; the instructor is one map inside, through a door.

**Two naming sets, both the client's own, and they are not interchangeable.** The *branch* is
Warrior / Magician / Bowman / Thief — that is what the NPCs' own lines say (*"become a
warrior"*) and what the second-advancement quests are titled (*"The Bowman's Next Journey"*).
The *class you become* is Swordsman / Magician / Archer / Rogue — that is quest `10001`'s
`Say.1.yes.0`, quoted in §3.2. `jobs.rs` already carries both, in `npc_name` and `job_name`.

### 1.1 The two NPC sets that are **not** first-job instructors

`gm-handbook/npcstrings.txt` holds four NPCs literally named `<Job> Job Instructor`, and then
a **second** set of four with the same names. Both sets are second-advancement, **and this
build says so in its own words** — which settles `STATUS.md` goal E's *"which set a real
advancement uses is **not** established"*:

```
514 / 319 / 227 / 424   d0    "What's going on?"
                        idle0 "Do you want to undergo your 2nd job advancement as a warrior?
                               Then go see Dances with Balrog in Perion."
800006 / 800004 / 800003 / 800005
                        idle0 identical text, and no d0 at all
```

**[L]**, verbatim. The second-advancement NPCs *name the first-job instructors* and send you
to them, which is independent corroboration of the §1 table from a string set that has nothing
to do with `Map.wz` or `Quest.wz`.

| set | templates | stands on | referenced by a quest? |
|---|---|---|---|
| examiners | 514, 319, 227, 424 | 10004023, 10002070, 10001090, 10003080 | **yes** — `Check.0.npc` of the four *Test of Qualification* quests, `lvmin 30`, `job.0` already 100..400 |
| dungeon signposts | 800006, 800004, 800003, 800005 | 80001300, 80001100, 80001000, 80001200 (the four job dungeons) | **no — zero rows, all four.** Control: the same scan finds 10 rows for 511 |

`jobs::first_job_at` already refuses all eight and a test pins the refusal. Nothing to change.

---

## 2. There is no Pirate branch in this build — three instruments, three controls

The owner's brief asks for Pirate *"if this build has one"*. It does not, and the reason to spend a
paragraph on it is that the client's **job-id space does** contain 500, so a quick look at the
wrong instrument says yes.

**The thing that says yes, and what it actually claims.** `net::opcode::uses_extended_sp`
decodes three literal bit masks out of `FUN_140302e30` and they select `100/110/111/112/…`
*and the same shape at 500* — `crates/net/src/opcode.rs`, and a test asserts `500` and `522`
take the extended branch. **[L]** But that is the **stat block's SP decoder** knowing a job
*number*. It is not content. A job number with no NPC, no map and no weapon is not a branch.

**Three instruments that say no, each with a positive control:**

| instrument | source | pirate result | control |
|---|---|---|---|
| `gm-handbook/npcstrings.txt` — 266 templates, 1322 rows, the whole `String.wz/Npc.img` | String.wz | `pirate` **0**, `kyrin` **0**, `nautilus` **0**, `brawler\|gunslinger\|buccaneer\|corsair` **0**; highest template is `9010000` Maple Administrator, no `1090000` | `warrior` 19, `magician` 19, `bowman` 19, `thief` 17 |
| `gm-handbook/maps.txt` (432 names) and `gm-handbook/fields.txt` (**426** field images) | String.wz **and** Map.wz — two different archives | no `nautilus`/`pirate`/`ship` name; **zero** ids in the `12xxxxxxx` range. Map prefixes are `1..61`, `100x`, `101x`, `102x`, `200x`, `800x`, `88x`, `89x`, `900x` and nothing else | the four instructor maps are present in both, `grep -cx` |
| `gm-handbook/equips.txt` | Character.wz | weapon classes present are `1302 1312 1322 1332 1372 1382 1402 1412 1422 1432 1442 1452 1462 1472`. **`1482` (knuckle) and `1492` (gun) are absent**, and `^14[89][0-9]{4},` matches **0** rows | the enumeration itself — it prints the whole class list rather than searching for one |
| `gm-handbook/questlines.txt` | Quest.wz | the full distinct set of `Check.<n>.job.<n>` values is `0, 100..134, 200..234, 300..334, 400..436` — **nothing in the 5xx range at all** | the same enumeration prints 68 distinct values including ones used once |

**[L]** throughout. Four instruments, four archives, one answer.

> **Say it as the negative it is:** *this client ships no Pirate instructor, no Pirate map and
> no Pirate weapon*. It is **not** "we did not find one". A fifth row in `FIRST_JOBS` would be
> a job with nobody to grant it, standing nowhere.

---

## 3. The job ids — `jobs.rs` agrees with the client, and now for a second reason

`crates/world/src/jobs.rs` was read first, as the brief asks. Its `FIRST_JOBS` is:

```
511 Dances with Balrog   -> 100 Swordsman  STR   10004003 Warriors' Sanctuary        town 10004000
313 Grendel the Really Old-> 200 Magician   INT   10002003 Magic Library              town 10002000
221 Athena Pierce        -> 300 Archer     DEX   10001051 Bowman Instructional School town 10001000
411 Dark Lord            -> 400 Rogue      LUK   10003003 Thieves' Hideout            town 10003000
```

**Every field of it reproduces from this client's dumps, and I checked each rather than
trusting the row** — `CLAUDE.md`'s *"a table row written from a quick read is a claim"*.
Templates, names, maps and map names: §1, all **[L]**, all matching.

### 3.1 The corroboration `job-advancement.md` did not have

The job **pairing** in that file rests on the SP masks (which give the *tree*, not the
instructor) plus quest `10001`'s prose. There is a third, harder source, and it is a
per-instructor gate rather than a global one — from `gm-handbook/questlines.txt`, **[L]**:

```
20000  "The Warrior's Next Journey"    Check 0.npc 511   Check 0.job.0 100   Check 0.lvmin 30
20100  "The Magician's Next Journey"   Check 0.npc 313   Check 0.job.0 200   Check 0.lvmin 30
20200  "The Bowman's Next Journey"     Check 0.npc 221   Check 0.job.0 300   Check 0.lvmin 30
20300  "The Thief's Next Journey"      Check 0.npc 411   Check 0.job.0 400   Check 0.lvmin 30
```

Each instructor's *own* second-advancement chain refuses anyone who is not already the job
`jobs.rs` says that instructor grants. That is the client asserting the pairing NPC by NPC,
and it agrees with `jobs.rs` four times out of four. **On the ids, the maps, the names and the
towns there is no disagreement at all**, so there is no "the client wins" finding to report
here.

**One field of that table does disagree with the client, and it is not an id:** `idle_line`
holds the `d0` for all four, and for 511 and 221 the shipped `idle0` is a *different and
better* sentence. §4.1.

### 3.2 The class names, verbatim, since the dialogue in §5 uses them

`10001` `Say.1.yes.0` and `1.yes.1`, **[L]**:

> *"Now that you've reached level 10, you can choose to advance to one of the following
> classes: #rSwordsman#k, #rMagician#k, #rArcher#k, or #rRogue#k."*
>
> *"To become a Swordsman, you need to go to #m10004000#; to become a Magician, you need to
> head to #m10002000#; to become an Archer, you need to go to #m10001000#; and to become a
> Rogue, you need to make your way to #m10003000#."*

**The client's own typo reproduces**, so nobody "fixes" it in our direction: `10001`'s
`QuestInfo.2` says *"a Thief in #m10004000#"* — Perion — where the dialogue above says Kerning
City. The journal line is wrong and the dialogue is right. **[L]** both, and this is the
second independent confirmation of the same typo.

---

## 4. The conversation: this client has no words for it

### 4.1 What the four instructors actually own, complete

`gm-handbook/npcstrings.txt`, every row for the four, nothing elided. **[L]**

```
511  name   Dances with Balrog
511  d0     Those who want to become a warrior, come see me...
511  d1     Those who want to become a warrior, come see me...
511  idle0  Do you want to become a warrior?
511  idle1  Those who want to become a warrior, come see me...
511  info0  Do you want to become a warrior?
511  info1  Those who want to become a warrior, come see me...

313  name   Grendel the Really Old
313  d0     All who desire to become a magician, talk to me...
313  d1     All who desire to become a magician, talk to me...
313  idle0  All who desire to become a magician, talk to me...
313  info0  All who desire to become a magician, talk to me...

221  name   Athena Pierce
221  d0     Those who want to become a bowman... Talk to me...
221  d1     Those who want to become a bowman... Talk to me...
221  idle0  Do you want to become a bowman?
221  info0  Do you want to become a bowman?

411  name   Dark Lord
411  d0     Those that want to be a thief, come...
411  d1     Those that want to be a thief, come...
411  idle0  Those that want to be a thief, come...
411  info0  Those that want to be a thief, come...
```

**Twenty-two rows, four of them names, and every one of the remaining eighteen is an idle
line.** Two of them
— 511's and 221's `idle0` — are *questions* (*"Do you want to become a warrior?"*), which is
the closest this client comes to an advancement offer and is still only one sentence.

> **This corrects one row of `job-advancement.md` §6.1 and of `jobs.rs`.** That file lists
> 511's `d0` as its idle line and 221's as *"Those who want to become a bowman... Talk to
> me..."*. The `d0`/`idle0` split above is the shipped one: for **511 and 221 the `idle0` is a
> different sentence from the `d0`**, and it is the better opener because it asks a question.
> `jobs::FirstJob::idle_line` currently holds the `d0` for all four. Small, but it is the one
> [L] sentence the design leans on.

### 4.2 Which case each instructor is in — and it is neither of the two

`research/quest-scripts.md` and `data/quest-scripts.txt` record a third state beyond
"shipped" and "absent": **twelve quests whose `Check.<n>.startscript` / `endscript` names a
script the client does not contain.** The instructors are not in that state either.

Every script name in the whole tree, enumerated rather than searched — **[L]**,
`gm-handbook/questlines.txt`:

```
1002   q1002s   q1002e
20002  q20002s  q20002e  q20002f        20102  q20102s  q20102e  q20102f
20202  q20202s  q20202e  q20202f        20302  q20302s  q20302e  q20302f
20003  q20003e  20103 q20103e  20203 q20203e  20303 q20303e
500000 q500000s q500000e q500000_midNPC1   500001 q500001s q500001e
500002 q500002s q500002e   500005 q500005s q500005e   500006 q500006s
500008 q500008s q500008e   500009 q500009s q500009e
```

**Not one of them belongs to a first advancement.** The `200x2`/`200x3` families are the
*second* advancement at level 30 (`job.0` already 100..400, `lvmin 30`) — the correction
`job-advancement.md` §1 made, reproduced here from the same file.

So the three states, and the fourth one that actually applies:

| state | example | what a server has to do |
|---|---|---|
| the client ships the whole tree | quest `1000` Heena | drive it from `questlines.txt` |
| the client names a script and ships no body | quest `1002` `q1002s` | author the body into `data/quest-scripts.txt`, keyed by quest id |
| the client ships a `Say."0"` and no `Say."1"` | `20003` and siblings | author one row |
| **the first advancement** | **511 / 313 / 221 / 411** | **there is no quest, so there is no node to fill.** Every word is ours, and it cannot live in `data/quest-scripts.txt` — §5.2 |

### 4.3 What the client does today when you click one — **measured, on 2026-08-27**

`job-advancement.md` §5.2 predicted `0x00F2` and honestly named two client-side branches that
could send **nothing at all** (`FUN_1401d3220(tid)`, and `template->[0x178]` at `141e3c72b`),
neither disassembled. That prediction is now settled by a capture that already existed.

`previous-runs/world-20260827-210956.log`, one session, character `Cobalt` (id 213), **job 0**
at the time, level 10, LUK 36:

```text
01:07:51.738  -> 0x00BB   Teleporting Cobalt to map 10003003, Thieves' Hideout
01:07:51.738  -> 0x01A0   SetField, GM !map 10003003
01:07:52.177  <- 0x00DC   CLIENT_FIELD_ENTERED
01:07:52.178  -> 0x044F   NpcEnterField: template 411 at (128,128) fh 39, object id 1000
01:07:55.996  <- 0x00F2   CLIENT_NPC_CLICK  e8030000 5500b700 ffffffff   (object id 1000)
01:07:56.005  -> 0x055B   ScriptMessage Say from NPC template 411, line 1 of 1 on path ""
01:07:57.427  <- 0x00F3   CLIENT_SCRIPT_REPLY, 50 bytes
01:07:58.317  <- 0x00F2   CLIENT_NPC_CLICK    <- clicked again, same packet
01:07:58.325  -> 0x055B   ... same box
01:07:59.748  <- 0x00F3   ...
```

**[L]**, all of it. Three things fall out, and the second is the one that was open:

* **The instructor is clickable and the click reaches the server.** Not silence, not `0x0151`.
* **It is `0x00F2`, twice.** Both of §5.2's silent branches were passed. The prediction is
  confirmed and the blind spot is closed *for this NPC on this map* — which is the whole
  population that matters.
* **The box the client drew was the `d0`**: the `0x055B` body carries `0x26` = 38 bytes,
  `"Those that want to be a thief, come..."`. So `Session::npc_line`'s `d0` fallback is what a
  player sees when they click an instructor **right now**, and it is indistinguishable from a
  click that did nothing.

**And `0x0151` cannot displace it, for any level.** The client picks `0x0151` only when the
requirement checker leaves a quest in the NPC's menu. Enumerating every quest whose
`Check.0.npc` is one of the four and reading its other `Check.0` rows — **[L]**:

| lowest gate on a startable quest | quest | why a jobless advancement candidate never sees it |
|---|---|---|
| `lvmin 13` | `80029` at 511 | also needs `quest 80028 state 2` |
| `lvmin 15` | `500002` at 313 | needs `quest 500001 state 2`, **and** a date window `start 202604211500 / end 202604232359` that closed in April |
| `lvmin 30` | `20000`, `20001`, `20100`, `20101`, `20200`, `20201`, `20300`, `20301` | `job.0` is already 100..400 |
| `lvmin 39/49/50/55` | `106xx`, `1212x`–`1213x` | far above the gate |

The `506017`/`506018`/`506117`/`506118` "First Greeting" quests name 221 and 411 on
**`Check.1.npc`** — the *completion* side — and their `Check.0.npc` is 235 / 431 with
`citizenshipGrade 5`. They do not put a menu line on an instructor.

> **So `0x00F2` is not merely what happened once; nothing in the data can produce anything
> else for a jobless character.** [D], from an [L] enumeration and an [L] capture that agree.

---

## 5. The dialogue, proposed

### 5.1 The lines

Written in `data/quest-scripts.txt`'s style and with its provenance discipline: **a line
marked [L] is the client's and is evidence; a line marked OURS is a placeholder the owner may
replace, and each is one row.** No attempt is made to reconstruct Nexon's wording — it is not
recoverable from anything on this machine, and §4.1 is the whole vocabulary that survives.

```text
# ---------------------------------------------------------------------------------------
# FIRST JOB ADVANCEMENT - authored dialogue. There is NO QUEST behind this; see 5.2.
# opener  = the box sent in answer to 0x00F2
# offer   = the last box, sent as a yes/no (message type 0x10), draws Accept / Decline
# yes     = after Accept, sent WITH the 0x007C that changes the job
# no      = after Decline
# refuse.* = the complete answer when the gate says no. A refusal is still an answer.
# ---------------------------------------------------------------------------------------

511  opener   Do you want to become a warrior?                        [L] Npc.img/511/idle0
511  offer    Then take up the sword, and do not set it down. Shall I make you a Swordsman?   OURS
511  yes      Then it is done. You are a Swordsman. Strength is a habit, not a gift -- go and practise it.   OURS
511  no       Come back when your mind is made up.                    OURS

313  opener   All who desire to become a magician, talk to me...      [L] Npc.img/313/d0
313  offer    Power without study is noise. Are you willing to learn? Shall I make you a Magician?   OURS
313  yes      Then it is done. You are a Magician. What you do not understand will kill you -- so understand it.   OURS
313  no       Come back when your mind is made up.                    OURS

221  opener   Do you want to become a bowman?                         [L] Npc.img/221/idle0
221  offer    The bow rewards patience and nothing else. Shall I make you an Archer?   OURS
221  yes      Then it is done. You are an Archer. Aim once, well, rather than often.   OURS
221  no       Come back when your mind is made up.                    OURS

411  opener   Those that want to be a thief, come...                  [L] Npc.img/411/d0
411  offer    Down here, quick hands outlive strong ones. Shall I make you a Rogue?   OURS
411  yes      Then it is done. You are a Rogue. Be somewhere else before anyone asks who you were.   OURS
411  no       Come back when your mind is made up.                    OURS

# The three refusals are shared, because the reason is shared and four voices for one
# rule would be inventing personality at scale. `world::jobs::refusal` already produces
# all three and the wording below is its wording.
*    refuse.advanced   You have already chosen your path. It cannot be undone.     [L]-flavoured: 10001 QuestInfo.2 says "a job advancement cannot be undone once made"
*    refuse.level      Come back when you have reached Level {needed}. You are only Level {level}.   OURS
*    refuse.stat       You are not ready. You need {needed} {STAT} and you have {have}.              OURS
```

Two deliberate choices:

* **The opener is the instructor's own sentence in all four cases**, because it is the only
  [L] text there is, and for 511 and 221 it is already a question — the box after it reads as
  an answer rather than a non-sequitur.
* **`refuse.advanced` is a refusal, not a re-offer.** The client's own words:
  *"a job advancement cannot be undone once made"*. `jobs::Advancement::AlreadyAdvanced`
  already encodes this.

**Avoid `#p`/`#m` substitution codes in authored lines.** Whether this client expands them
inside a `0x055B` box **is not established** — `session/npc.rs` says so in as many words about
`#p8#`, and no capture or screen report settles it. The lines above use none, so they cannot
fail that way. (Sending one *deliberately* one day is a free measurement; it is not one to
take on the run that first tests advancement.)

### 5.2 Why this cannot go in `data/quest-scripts.txt`

That file's format is `questId <TAB> node <TAB> dotted.path <TAB> value`, and
`world::config::load_quests` parses exactly that. **A first advancement has no quest id**
(§4.2), so putting these rows there means inventing one — and the repo already refuses that,
for a reason with teeth: `Session::accept_quest` → `record_quest_start` → `store::start_quest`
writes a `quest_state` row, so a synthetic id would put a nonexistent quest in the player's
journal and in the database. `job-advancement.md` §8.2 says this and it is still right.

The text belongs beside the table that owns the decision — `crates/world/src/jobs.rs`'s
`FirstJob`, which already carries `idle_line` — or in a small new `data/` file keyed by NPC
template. Either way it is **authored data, committed on purpose**, like `data/shops.txt`, and
it must say so in a header.

---

## 6. The 25 — [I], and it is a design decision, not a measurement

The owner, 2026-08-27:

> *"The instructor of each job should offer the player to advance to each respective job when
> they have at least 25 ability points in that job branch's main stat. 25 points of STR for
> warrior, 25 points of INT for magician, and so on."*

**Nothing in this client expresses that rule, or any rule like it.** Two enumerations, both
re-run from scratch here rather than cited:

* **The whole `Check` key space of all 322 quests is 33 shapes** — `npc` 640, `lvmin` 406,
  `item.*` 798, `quest.*` 486, `job.<n>` 104, `mob.*` 232, `citizenship*` 172, `skill.*` 48,
  the script names, the date fields, and four shapes with a **single** use each (`pop`,
  `interval`, `npcSpeech.<n>.script`, `npcSpeech.<n>.order`). **There is no `str`, `dex`,
  `int` or `luk` key.** A scan that resolves a key with one use is not a scan that missed one
  with dozens: **the negative is verified, not merely empty.** **[L]**
* **The quest *text* carries no stat token either.** `STR` / `DEX` / `INT` / `LUK` as whole
  words, case sensitive: **0, 0, 0, 0**. The words `dexterity` **0**; `strength` 18 and `luck`
  45, every one of them prose (*"I can regain my full strength"*, *"good luck"*), never a
  requirement. Controls on the same file: `level` 246, `job advancement` **20**,
  `Ability Points` 2. **[L]**
* `gm-handbook/questreq.txt` cannot express it at all — it is a filtered dump carrying only
  `item` (329) and `mob` (87) rows. Naming it as a place the rule might hide would have been a
  search that could only come back empty.

> **So: the 25 is [I]. It is the owner's design decision. The client does not enforce it, cannot
> contradict it, and nothing will ever catch it being wrong.** That sentence is the most
> valuable line in this file and it is deliberately not dressed up: the server is the only
> thing in the world that will ever know this rule exists.

### 6.1 It contradicts the constant that is in the tree, and that is the finding

`crates/world/src/jobs.rs`:

```rust
pub const STAT_MINIMUM: u16 = 35;
```

sourced from the owner, **2026-08-19**: *"Magician 35 INT, Warrior 35 STR, Thief 35 LUK, Bowman 35
DEX"*, which they flagged themself as fan-site sourced. Their 2026-08-27 sentence says **25**.

**Both numbers are [I] and the newer one is the user's own instruction, so 25 wins.** The
constant exists precisely so this is one edit — `jobs.rs` has a test,
`the_prerequisite_is_one_constant`, that asserts `STAT_MINIMUM == 35`, so the edit is two
lines and the test is what makes "one edit to change" true rather than aspirational. Not made
here: `crates/` is off limits to this pass.

`STATUS.md` goal E still quotes the 35 sentence as the brief. It will need the same edit.

### 6.2 The ambiguity in the sentence, which is worth one question

*"25 ability points in that job branch's main stat"* and *"25 points of STR"* have two
readings that differ by the character's rolled base:

| reading | what the gate is | for a character rolled at STR 12 |
|---|---|---|
| **(a) the stat value** | `chr.strength >= 25` | needs 13 AP → **level 4** |
| (b) AP *invested* | `chr.strength - base >= 25` | needs 25 AP → **level 6** |

**(a) is the only one this server can enforce**, and that is a fact about the schema rather
than a preference: `crates/store/src/db.rs`'s `characters` table stores `strength, dexterity,
intelligence, luck` and **no base-roll column**, so "points invested" is not computable from
stored state without a migration. **[L]** `world::jobs::Stat::of` already implements (a).

Either way **the level gate binds first**: `LEVEL_MINIMUM = 10` is corroborated by the client
itself (`10001` `Check.1.lvmin = 10` and its text), `expcurve::LevelGains` awards 5 AP a level,
and the client's create roll totals 25 across four stats (`STATUS.md`, two captures, and
`Character::default` is 12/5/4/4). Taking the roll's best and worst case for one stat, **25 is
reachable at level 4 to 6 and 35 at level 6 to 8** — both below the level gate either way.
**So a 25 gate does not make anything newly testable that 35 did not; it only makes the
refusal rarer.** [D]

### 6.3 And the gate is testable *today*, which it was not when §9.1 was written

`job-advancement.md` §9.1 said a level-10 beginner could not reach the stat on this server,
because AP allocation existed and was not dispatched — and told the reader to **re-check the
dispatcher rather than trust the paragraph**. Re-checked, and it has changed:

* `crates/world/src/session/mod.rs` dispatches **both** `net::abilityup::CLIENT_ABILITY_UP`
  (`0x0138`, a single `+` click) and `CLIENT_ABILITY_MASS_UP` (`0x0139`). **[L]**
* And it has been exercised on a real client. Same 2026-08-27 session, 29 seconds before the
  Dark Lord click — **[L]**, `previous-runs/world-20260827-210956.log`:

```text
01:07:26.297  <- 0x0139   -> StatChanged: bulk spent 30 into LUK -> 36 - 15 ap left
01:07:27.941  <- 0x0139   -> StatChanged: bulk spent 15 into DEX -> 24 - 0 ap left
```

A character reached **36 LUK** by spending AP in the client's own stat window. That clears 35
and 25 alike. **The prerequisite half of this feature is no longer blocked on anything.**

---

## 7. How the conversation reaches the server

### 7.1 The opcodes, all of them already working in this repo

Citing the repo's own working rather than re-deriving: `research/npc-click.md` (the fork that
chooses the click packet), `research/npc-dialogue.md` (the `0x055B` head), and
`research/script-reply.md` (the answer byte, cross-checked against three real bodies).

| direction | opcode | constant | role here |
|---|---:|---|---|
| client → | `0x00F2` | `net::script::CLIENT_NPC_CLICK` | the click. **This is the one an instructor produces** — measured, §4.3 |
| client → | `0x0151` | `CLIENT_QUEST_REQUEST` | the *quest* request. Never fires for an instructor, §4.3 |
| → client | `0x055B` | `SCRIPT_MESSAGE` | every box. `npc_say(template, text, prev, next)` / `npc_ask(template, text, quest_flavoured)` |
| client → | `0x00F3` | `CLIENT_SCRIPT_REPLY` | **the accept/decline.** Body `u32 handle, u8 messageType, u32 echo, str text, i8 action` |
| → client | `0x007C` | `net::stats::STAT_CHANGED` | the job change itself, mask bit 5. `job-advancement.md` §4 owns it |

**The answer byte is the trailing `i8`**: `SCRIPT_ACTION_YES = 1`, `SCRIPT_ACTION_NO = 0`,
`SCRIPT_ACTION_CLOSED = -1`. **[L]** It is only unambiguous on a yes/no box — on a plain
`Say`, the client rewrites `BtOK` to the Next result at `142a59fb5` so OK and Next both come
back as `1`, which is why `Conversation::sent_with_next` exists.

**The box that draws Accept / Decline is message type `0x10`** (`SCRIPT_TYPE_QUEST_YES_NO`,
`BtQYes`/`BtQNo`). Type **`3`** (`SCRIPT_TYPE_YES_NO`, plain `BtYes`/`BtNo`) is decoded and
built by the same function.

> **Use type `0x10`, and know why.** 27 yes/no boxes have gone to this client across the
> archived runs and **every one of them was type `0x10`** — `npc_ask`'s only caller passes
> `quest_flavoured = true`. **Type 3 has never been on the wire here.** It is [L] from the
> listing and **[D]** that it draws what the listing says. A job advancement is not a quest, so
> type 3 is the semantically tidier box — but choosing it means putting an untested message
> type in the same run as an untested feature, which is the two-variables mistake. Ship `0x10`
> first; try type 3 later, on its own, as a one-line change.

### 7.2 The single line that blocks it

`crates/world/src/session/npc.rs`, `say_line`:

```rust
let branches =
    last && !on_branch && convo.quest_id.is_some() && self.has_branch(&convo, "yes");
```

`Conversation` distinguishes "quest" from "plain talk" by `quest_id: Option<u32>`, and
`say_lines` returns exactly one line — the NPC's `d0` — whenever it is `None`. **So an
instructor conversation as the code stands can only ever be a single plain `Say` box, which is
precisely what the 2026-08-27 capture shows happening.** `job-advancement.md` §5.1 identified
this and it is still true; §8.2 of that file has the shape of the fix (a third conversation
kind carrying the offered job, tested in `say_line` beside `quest_id` and read in
`on_script_reply`'s `awaiting_yes_no` branch). Both files agree, and neither has wired it.

### 7.3 Two rules this path has already paid for

* **Every refusal is a box, never silence.** `jobs::refusal` returns a sentence for all three
  refusal arms and `None` only when the character may advance. An unanswered packet freezes the
  client's whole UI.
* **Every effect hangs off the transition.** The job change, the save and the congratulation
  line must all be inside the arm that actually advanced the character —
  `store::complete_quest`'s payout bug was exactly this, and the test that missed it counted
  the one effect that was gated correctly.

---

## 8. The routes, and the one that is not like the others

From `gm-handbook/portals.txt`, **[L]**. Each instructor map has exactly **one** way in:

```text
10004000 Perion        portal[23] in02     -> 10004003 Warriors' Sanctuary
10002000 Ellinia       portal[37] jobin00  -> 10002003 Magic Library
10003000 Kerning City  portal[26] in03     -> 10003003 Thieves' Hideout
10001050 Henesys Park  portal[15] in02     -> 10001051 Bowman Instructional School
                                               ^^^ NOT Henesys. One map further out.
```

Three of the four hang directly off their town. **Athena Pierce does not**: Henesys
(`10001000`) → `in01` → **Henesys Park (`10001050`)** → `in02` → the school. `!map 10001051`
skips the walk, but any test that *walks* the Bowman route crosses Henesys Park.

### 8.1 Henesys Park: the question `research/henesys-park-null-deref.md` §3 left open has already been answered, in an archived run

That file says *"Map 10001050 has been tried exactly once, and that once it crashed"*, and
sets the discriminator: `!map 10001050` **as the first thing after login**, at ~40 s of client
life. Dies → the map is fatal. Survives → it was the 389-second session.

**That run happened on the evening of 2026-08-22 and nobody looked.** Both halves come from
one session, and the two logs are aligned by their shared archive stamp and by a constant
4-hour offset between the world clock and the hook clock, which holds in both sessions checked:

```text
maplecw-hook-20260822-205640.log   20:55:40.934  install_once: our code IS running
world-20260822-205640.log          00:55:57.366  -> 0x01A0 SetField, GM !map 10001050, GoodTest
                                   00:55:57.820  <- 0x00DC CLIENT_FIELD_ENTERED
                                   00:55:57.821  -> 0x044F  four NPCs: 218, 219, 220, 228
                                   ... 43 more seconds of ordinary traffic ...
maplecw-hook-20260822-205640.log   20:56:40.125  SOCKET closed and cleared by the client
                                   CLIENT FAULT lines in the whole session: 0
```

**[L]** So the park was loaded **17 seconds into client life**, the client acknowledged field
entry, drew the four NPCs and ran on for another 43 seconds to a clean, client-initiated
close. **That is reading (b): the map is not fatal to load, and the 2026-08-22 09:04 death at
389 s was the session.** The discriminating experiment was performed and its result never
reached `STATUS.md`, which still lists the question as open.

**What it does not prove**, stated plainly: the run lived 60 seconds in total, so this says
nothing about a *long* session on that map. It refutes "the map cannot be loaded", which is
what (a) claimed.

### 8.2 And the death in the 2026-08-27 run is the other family, at its expected age

Same session as §4.3 and §6.3, 100 seconds after the job change:

```text
world-20260827-210956.log   01:09:54.508  -> 0x01A0 SetField, GM !map 10001050, Cobalt
maplecw-hook-20260827-210956.log
                            21:09:54.948  WATCH #29 ... while dispatching opcode 0x01A0
                            21:09:54.954  CLIENT FAULT #1: code=0xc0000374 at ntdll.dll
                                          ... at +401562 ms
                            21:09:56.389  CRASH DUMP wrote ...-c0000374-1.dmp, 1.35 GB
```

**[L]** `0xC0000374` is **STATUS_HEAP_CORRUPTION** — `STATUS.md` item 4's family, the wild
write whose damaged-slot count *tracks session length at about one per 250 s*. This died at
**401 s**. It is **not** `0xC0000005`, which is the park null-dereference family.

> **So do not write "10001050 crashed the client again".** Three loads of that map are on
> record across three sessions: one died of `0xC0000005` at 389 s, one **survived** at 17 s,
> one died of `0xC0000374` at 401 s. The two deaths are different exception codes and both sit
> at the age the accumulating family predicts. `CLAUDE.md`'s corollary — *separate "this thing"
> from "this session" first* — has now paid a third time, and it went the session's way.

**Practical consequence for testing the Bowman branch:** it is not more dangerous than the
others, but it is **later in a session** if the route is walked. If a single launch has to
cover the Bowman path, `!map 10001051` early is worth more than a walk through the park at
minute six.

---

## 9. Instruments, and the two controls that caught me

* **Everything was run with the repo as the working directory.** No script was executed out of
  the session scratchpad, so the stale `reads.py`/`listing.py`/`callers.py` copies could not
  shadow `tools/`.
* **No Ghidra, no `cargo`, no `git`, and no client run.** Other agents are working; every
  number here comes from a generated dump, a source file or an archived log.
* **`gm-handbook/` was read only.** Nothing regenerated, nothing edited.
* **The stat-word search was wrong the first time and the control is what said so.** A
  case-insensitive `grep -c " STR"` reported **141** hits and `" INT"` **150** — noise from
  `strange`/`strength` and `into`/`interesting`. Re-run with `-w` and case sensitivity: **0,
  0, 0, 0**. A negative built on the first run would have been right by accident; a *positive*
  built on it would have invented a stat requirement out of the word "into".
* **The map-load count was wrong the first time, for the oldest reason in this file: I counted
  lines, not events.** `grep -c` over the archived logs reported **6** loads of `10001050`,
  because the launcher writes a `0x00BB` notice *and* a `0x01A0` line per teleport. Counting
  only the `SET_FIELD` line gives **3**, in three sessions — which is the number §8 rests on.
  Had the 6 stood, "loaded six times, died twice" would have read as a flaky map instead of
  three clean data points.
* **The Pirate negative was given four instruments across four archives** (`String.wz/Npc.img`,
  `String.wz/Map.img`, `Map.wz` field images, `Character.wz` equips, `Quest.wz`), each printing
  a full enumeration rather than answering a search, and each with a positive control. One of
  those controls was itself void and is reported as such: `grep -i knuckle equips.txt` returns
  0, but so does `grep -i bow` — **that file has no name column**, so the name search there was
  incapable of a positive and only the id-prefix enumeration counts.
* **`gm-handbook/fields.txt` is one bare id per line**, which produced a confident wrong answer
  for a previous pass. Membership was tested with `grep -cx`, and `grep -cx 1` as the control.
* **Two logs, not one**, for both §8 findings: the world log says what the server sent, the
  hook log says whether the client faulted. Neither claim is visible in one file, and the
  4-hour clock offset between them is stated rather than silently corrected.

---

## 10. What I could not establish

* **Whether the client expands `#p<id>#` / `#m<id>#` inside a `0x055B` box.** `session/npc.rs`
  raises the question about `#p8#` and no capture or screen report answers it. §5.1's lines
  avoid the codes entirely so nothing here depends on it.
* **Whether message type `3` draws `BtYes`/`BtNo` on *this* client.** [L] from the listing,
  never sent — 27 for 27 of the yes/no boxes on record are type `0x10`. §7.1.
* **Whether the `JobChanged` animation actually draws.** The `!job` command has been sent twice
  on real clients (`!job 100`, 2026-08-21; `!job 400`, 2026-08-27) and the session survived
  both — so the packet is not fatal, **[L]** — but the fanfare is a screen event and **no
  report of it exists in any file I can read**. Only the owner can close this, and it is one line
  of a test plan, not a launch of its own.
* **Whether a first-job character survives a relog.** The 2026-08-27 session died 100 s after
  `!job 400` and did not log in again, so the character-select list and the next `SetField`
  were never seen carrying job 400. `jobs::sp_encoding_changes(0, 400)` is `false` and a test
  pins it, so nothing *should* shift — but "should" is the word, and this is the cheapest
  remaining unknown.
* **Where the authored dialogue should live on disk.** §5.2 rules out
  `data/quest-scripts.txt` and names two options; choosing between them is the coordinator's,
  and it is a structural decision rather than a finding.
* **Whether any of this is wired.** It is not. `pub mod jobs;` is now in
  `crates/world/src/lib.rs`, so the module compiles with the workspace — but a grep for
  `jobs::` across `crates/` returns **nothing outside `jobs.rs` itself**. The decision table,
  its 12 tests and its refusal sentences are still `CLAUDE.md`'s *"built is not wired"*, and on
  screen that is indistinguishable from not existing.
